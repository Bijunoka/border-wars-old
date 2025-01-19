use crate::{tcp, Packet};
use bevy::prelude::*;
use std::{collections::HashMap, io, net::ToSocketAddrs, sync::Arc};

#[cfg(feature = "sync")]
use ::{
    serde::{de::DeserializeOwned, Serialize},
    std::{any::type_name, marker::PhantomData},
};

/// A trait for a handler function.
pub trait PacketHandlerFn: Fn(Vec<u8>, &mut World) + Send + Sync + 'static {}

impl<T: Fn(Vec<u8>, &mut World) + Send + Sync + 'static> PacketHandlerFn for T {}

/// A function that handle a received [Packet]s.
type PacketHandler = Box<dyn PacketHandlerFn>;

/// A Bevy resource that store the packets handlers.
#[derive(Resource)]
struct HandlerManager(Arc<HashMap<u64, PacketHandler>>);

/// A [Connection] to a remote server.
#[derive(Resource)]
pub struct ServerConnection(tcp::Connection);

impl ServerConnection {
    /// Connects to a remote server.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        Ok(Self(tcp::Connection::connect(addr)?))
    }

    /// Sends a packet through this connection.
    pub fn send<P: Packet>(&self, packet: &P) {
        let mut data = bincode::serialize(packet).expect("Failed to serialize packet");
        data.extend(P::packet_id().to_be_bytes());
        self.0.send(data);
    }
}

#[cfg(feature = "sync")]
/// An event that comes from the server.
#[derive(Deref)]
pub struct FromServer<E: Event + Packet> {
    /// The event.
    pub event: E,
}

#[cfg(feature = "sync")]
/// Mark an [Entity] as synced by the server.
#[derive(Component)]
pub struct ServerEntity(Entity);

/// A plugin that manage the network [Connection]s.
pub struct ClientPlugin;

impl ClientPlugin {
    #[cfg(feature = "client")]
    /// Handles a received [Packet] on the server.
    pub fn handle_packets(world: &mut World) {
        // Get all received packets
        let mut packets = Vec::new();
        if let Some(connection) = world.get_resource::<ServerConnection>() {
            while let Some(mut packet) = connection.0.recv() {
                if packet.len() < 8 {
                    println!("Invalid packet received: {:?}", packet);
                } else {

                    let id_buffer = packet.split_off(packet.len() - 8);
                    let packet_id = u64::from_be_bytes(id_buffer.try_into().unwrap());
                    packets.push((packet_id, packet));
                }
            }
        } else {
            return;
        }

        // Get the packet handlers
        let handlers = Arc::clone(&world.resource_mut::<HandlerManager>().0);

        // Handle all received packets
        for (packet_id, packet) in packets {
            if let Some(handler) = handlers.get(&packet_id) {
                handler(packet, world);
            }
        }
    }

    #[cfg(feature = "client")]
    /// Remove [Connection] if it's disconnected.
    pub fn remove_disconnected(mut commands: Commands, connection: Option<Res<ServerConnection>>) {
        if let Some(connection) = connection {
            if connection.0.closed() {
                commands.remove_resource::<ServerConnection>();
            }
        }
    }

    #[cfg(feature = "sync")]
    /// Removes [ServerEntity] when disconnected.
    fn remove_synced(mut commands: Commands, entities: Query<Entity, With<ServerEntity>>) {
        for entity in entities.iter() {
            commands.entity(entity).despawn();
        }
    }
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HandlerManager(Arc::new(HashMap::new())));
        app.add_system(Self::handle_packets);
        app.add_system(Self::remove_disconnected);

        #[cfg(feature = "sync")]
        {
            app.add_packet_handler(|entity: Entity, world| {
                if let Some((local_entity, _)) = world
                    .query::<(Entity, &ServerEntity)>()
                    .iter(world)
                    .find(|(_, server_entity)| server_entity.0 == entity)
                {
                    world.despawn(local_entity);
                } else {
                    world.spawn(ServerEntity(entity));
                }
            });
            app.add_system(Self::remove_synced.run_if(resource_removed::<ServerConnection>()));
        }
    }
}

/// An extension to add packet handlers.
pub trait NetworkExt {
    /// Add a new packet handler.
    fn add_packet_handler<P: Packet, H: Fn(P, &mut World) + Send + Sync + 'static>(
        &mut self,
        handler: H,
    ) -> &mut Self;

    #[cfg(all(feature = "sync"))]
    /// Register syncronization for an [Event] that can be sent by the server.
    fn sync_server_event<E: Event + Packet>(&mut self) -> &mut Self;

    #[cfg(all(feature = "sync"))]
    /// Register syncronization for an [Event] that comes from the client.
    fn sync_client_event<E: Event + Packet>(&mut self) -> &mut Self;

    #[cfg(all(feature = "sync"))]
    /// Register a [Component] to be synced.
    fn sync_component<C: Component + DeserializeOwned + Serialize + Clone>(&mut self) -> &mut Self;

    #[cfg(all(feature = "sync"))]
    /// Register a [Resource] to be synced.
    fn sync_resource<R: Resource + DeserializeOwned + Serialize>(&mut self) -> &mut Self;
}

impl NetworkExt for App {
    fn add_packet_handler<P: Packet, H: Fn(P, &mut World) + Send + Sync + 'static>(
        &mut self,
        handler: H,
    ) -> &mut Self {
        Arc::get_mut(&mut self.world.resource_mut::<HandlerManager>().0)
            .unwrap()
            .insert(
                P::packet_id(),
                Box::new(
                    move |data: Vec<u8>, world| match bincode::deserialize::<P>(&data) {
                        Ok(packet) => handler(packet, world),
                        Err(_) => println!("Failed to deserialize packet: {}", type_name::<P>()),
                    },
                ),
            );
        self
    }

    #[cfg(feature = "sync")]
    fn sync_server_event<E: Event + Packet>(&mut self) -> &mut Self {
        self.add_event::<FromServer<E>>()
            .add_packet_handler(|event: E, world| world.send_event(FromServer { event }))
    }

    #[cfg(feature = "sync")]
    fn sync_client_event<E: Event + Packet>(&mut self) -> &mut Self {
        self.add_event::<E>().add_system(
            |mut events: EventReader<E>, connection: Option<Res<ServerConnection>>| {
                if let Some(connection) = connection {
                    for event in events.iter() {
                        connection.send(event);
                    }
                }
            },
        )
    }

    #[cfg(feature = "sync")]
    fn sync_component<C: Component + DeserializeOwned + Serialize>(&mut self) -> &mut Self {
        self.add_packet_handler(|data: (Entity, C), world| {
            let (entity, component) = data;
            if let Some((local_entity, _)) = world
                .query::<(Entity, &ServerEntity)>()
                .iter(world)
                .find(|(_, server_entity)| server_entity.0 == entity)
            {
                let mut local_entity = world.entity_mut(local_entity);
                match local_entity.get_mut::<C>() {
                    Some(mut local_component) => {
                        *local_component = component;
                    }
                    None => {
                        local_entity.insert(component);
                    }
                }
            } else {
                println!("Received component for unknown entity: {:?}", entity);
            }
        })
        .add_packet_handler(|data: (Entity, PhantomData<C>), world| {
            let (entity, _) = data;
            if let Some((local_entity, _)) = world
                .query::<(Entity, &ServerEntity)>()
                .iter(world)
                .find(|(_, server_entity)| server_entity.0 == entity)
            {
                world.entity_mut(local_entity).remove::<C>();
            }
        })
    }

    #[cfg(feature = "sync")]
    fn sync_resource<R: Resource + DeserializeOwned + Serialize>(&mut self) -> &mut Self {
        self.add_packet_handler(|resource: R, world| world.insert_resource(resource))
            .add_packet_handler(|_: PhantomData<R>, world| {
                world.remove_resource::<R>();
            })
    }
}
