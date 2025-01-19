use crate::{tcp, Packet};
use bevy::prelude::*;
use std::{collections::HashMap, io, net::ToSocketAddrs, sync::Arc};

#[cfg(feature = "sync")]
use ::{
    serde::{de::DeserializeOwned, Serialize},
    std::{any::type_name, marker::PhantomData, ops::Deref},
};

/// A trait for a handler function.
pub trait PacketHandlerFn:
    Fn(Entity, ClientConnection, Vec<u8>, &mut World) + Send + Sync + 'static
{
}

impl<T: Fn(Entity, ClientConnection, Vec<u8>, &mut World) + Send + Sync + 'static> PacketHandlerFn
    for T
{
}

/// A function that handle a received [Packet]s.
type PacketHandler = Box<dyn PacketHandlerFn>;

/// A Bevy resource that store the packets handlers.
#[derive(Resource)]
struct HandlerManager(Arc<HashMap<u64, PacketHandler>>);

/// A Bevy resource that listens for incoming [Connection]s.
#[derive(Resource)]
pub struct Listener(tcp::Listener);

impl Listener {
    /// Creates a new [Listener] on the given address.
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        Ok(Self(tcp::Listener::bind(addr)?))
    }
}

/// A [Connection] to a remote client.
#[derive(Component)]
pub struct ClientConnection(Arc<tcp::Connection>);

impl ClientConnection {
    /// Sends a packet through this connection.
    pub fn send<P: Packet>(&self, packet: &P) {
        let mut data = bincode::serialize(packet).expect("Failed to serialize packet");
        data.extend(P::packet_id().to_be_bytes());
        self.0.send(data);
    }
}

#[cfg(feature = "sync")]
/// An event that comes from a client.
pub struct FromClient<E: Event + Packet> {
    /// The entity of the [Connection] that sent the event.
    pub entity: Entity,

    /// The [Connection] that sent the event.
    pub connection: ClientConnection,

    /// The event.
    pub event: E,
}

#[cfg(feature = "sync")]
impl<E: Event + Packet> Deref for FromClient<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

#[cfg(feature = "sync")]
/// Mark an [Entity] to be synced.
#[derive(Component)]
pub struct Synced;

/// A plugin that manage the network [Connection]s.
pub struct ServerPlugin;

impl ServerPlugin {
    /// Accept new [Connection]s.
    fn accept_connections(mut commands: Commands, listener: Option<Res<Listener>>) {
        if let Some(listener) = listener {
            if let Some(connection) = listener.0.accept() {
                commands.spawn(ClientConnection(Arc::new(connection)));
            }
        }
    }

    /// Handles a received [Packet]s.
    fn handle_packets(world: &mut World) {
        // Get all received packets
        let mut packets = Vec::new();
        for (entity, connection) in world.query::<(Entity, &ClientConnection)>().iter(world) {
            while let Some(mut packet) = connection.0.recv() {
                if packet.len() < 8 {
                    println!("Invalid packet received: {:?}", packet);
                } else {
                    let id_buffer = packet.split_off(packet.len() - 8);
                    let packet_id = u64::from_be_bytes(id_buffer.try_into().unwrap());
                    packets.push((
                        entity,
                        ClientConnection(Arc::clone(&connection.0)),
                        packet_id,
                        packet,
                    ));
                }
            }
        }

        // Get the packet handlers
        let handlers = Arc::clone(&world.resource_mut::<HandlerManager>().0);

        // Handle all received packets
        for (entity, connection, packet_id, packet) in packets {
            if let Some(handler) = handlers.get(&packet_id) {
                handler(entity, connection, packet, world);
            }
        }
    }

    /// Remove disconnected [Connection]s.
    fn remove_disconnected(
        mut commands: Commands,
        connections: Query<(Entity, &ClientConnection)>,
    ) {
        for (entity, connection) in connections.iter() {
            if connection.0.closed() {
                commands.entity(entity).remove::<ClientConnection>();
            }
        }
    }

    #[cfg(feature = "sync")]
    /// Send to clients the [Synced] entity that has been added to the server.
    fn send_added(
        added_entities: Query<Entity, Added<Synced>>,
        connections: Query<&ClientConnection>,
    ) {
        for entity in added_entities.iter() {
            for connection in connections.iter() {
                connection.send(&entity);
            }
        }
    }

    #[cfg(feature = "sync")]
    /// Send [Synced] entities to new clients.
    fn send_synced(
        synced_entities: Query<Entity, With<Synced>>,
        new_connections: Query<&ClientConnection, Added<ClientConnection>>,
    ) {
        for entity in synced_entities.iter() {
            for connection in new_connections.iter() {
                connection.send(&entity);
            }
        }
    }

    #[cfg(feature = "sync")]
    /// Send to clients the [Synced] entity that has been removed.
    fn send_removed(
        mut removed_entities: RemovedComponents<Synced>,
        connections: Query<&ClientConnection>,
    ) {
        for entity in removed_entities.iter() {
            for connection in connections.iter() {
                connection.send(&entity);
            }
        }
    }
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HandlerManager(Arc::new(HashMap::new())));
        app.add_system(Self::handle_packets);
        app.add_system(Self::remove_disconnected);
        app.add_system(Self::accept_connections);

        #[cfg(feature = "sync")]
        {
            app.add_system(Self::send_added);
            app.add_system(Self::send_synced);
            app.add_system(Self::send_removed);
        }
    }
}

/// An extension to add packet handlers.
pub trait NetworkExt {
    /// Add a new packet handler.
    fn add_packet_handler<
        P: Packet,
        H: Fn(Entity, ClientConnection, P, &mut World) + Send + Sync + 'static,
    >(
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
    fn add_packet_handler<
        P: Packet,
        H: Fn(Entity, ClientConnection, P, &mut World) + Send + Sync + 'static,
    >(
        &mut self,
        handler: H,
    ) -> &mut Self {
        Arc::get_mut(&mut self.world.resource_mut::<HandlerManager>().0)
            .unwrap()
            .insert(
                P::packet_id(),
                Box::new(
                    move |entity, connection, data: Vec<u8>, world| match bincode::deserialize::<P>(
                        &data,
                    ) {
                        Ok(packet) => handler(entity, connection, packet, world),
                        Err(_) => println!("Failed to deserialize packet: {}", type_name::<P>()),
                    },
                ),
            );
        self
    }

    #[cfg(feature = "sync")]
    fn sync_server_event<E: Event + Packet>(&mut self) -> &mut Self {
        self.add_event::<E>().add_system(
            |mut events: EventReader<E>, connections: Query<&ClientConnection>| {
                for event in events.iter() {
                    for connection in connections.iter() {
                        connection.send(event);
                    }
                }
            },
        )
    }

    #[cfg(feature = "sync")]
    fn sync_client_event<E: Event + Packet>(&mut self) -> &mut Self {
        self.add_event::<FromClient<E>>().add_packet_handler(
            |entity, connection, event: E, world| {
                world.send_event(FromClient {
                    entity,
                    connection,
                    event,
                })
            },
        )
    }

    #[cfg(feature = "sync")]
    fn sync_component<C: Component + DeserializeOwned + Serialize + Clone>(&mut self) -> &mut Self {
        let update_components =
            |changed_components: Query<(Entity, &C), (Changed<C>, With<Synced>)>,
             connections: Query<&ClientConnection>| {
                for (entity, component) in changed_components.iter() {
                    for connection in connections.iter() {
                        connection.send(&(entity, component.clone()));
                    }
                }
            };
        let send_components =
            |components: Query<(Entity, &C), With<Synced>>,
             new_connections: Query<&ClientConnection, Added<ClientConnection>>| {
                for (entity, component) in components.iter() {
                    for connection in new_connections.iter() {
                        connection.send(&(entity, component.clone()));
                    }
                }
            };
        let remove_components =
            |mut removed_components: RemovedComponents<C>,
             synced_entities: Query<Entity, With<Synced>>,
             connections: Query<&ClientConnection>| {
                for entity in removed_components.iter() {
                    if synced_entities.contains(entity) {
                        for connection in connections.iter() {
                            connection.send(&(entity, PhantomData::<C>));
                        }
                    }
                }
            };
        self.add_system(update_components.after(ServerPlugin::send_added))
            .add_system(send_components.after(ServerPlugin::send_synced))
            .add_system(remove_components.after(update_components))
    }

    #[cfg(feature = "sync")]
    fn sync_resource<R: Resource + DeserializeOwned + Serialize>(&mut self) -> &mut Self {
        let update = |resource: Res<R>, connections: Query<&ClientConnection>| {
            for connection in connections.iter() {
                connection.send(&*resource);
            }
        };
        let send =
            |resource: Res<R>,
             new_connections: Query<&ClientConnection, Added<ClientConnection>>| {
                for connection in new_connections.iter() {
                    connection.send(&*resource);
                }
            };
        let remove = |connections: Query<&ClientConnection>| {
            for connection in connections.iter() {
                connection.send(&PhantomData::<R>);
            }
        };
        self.add_system(update.run_if(resource_exists_and_changed::<R>()))
            .add_system(send.run_if(resource_exists::<R>()))
            .add_system(remove.run_if(resource_removed::<R>()))
    }
}
