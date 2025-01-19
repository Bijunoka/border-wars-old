use bevnet::client::NetworkExt;
use bevnet::client::{ClientPlugin, ServerConnection};
use bevy::prelude::*;
use border_warsv2::data::*;

use border_warsv2::playing::*;
use border_warsv2::render::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StartPlugin)
        .add_plugin(ConexionPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(PlayingPlugin)
        .run();
}

struct ConexionPlugin;
impl Plugin for ConexionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ClientPlugin)
            .insert_resource(ServerConnection::connect("139.59.211.249:8080").unwrap())
            .add_packet_handler(|identity: PacketIdentification, world| {
                world.insert_resource(identity)
            })
            .add_packet_handler(|race: Race, world| {
                world.insert_resource(race);
                println!("{:?}", race);
            })
            .add_plugin(SyncroPlugin)
            .sync_resource::<Tour>();
    }
}

struct StartPlugin;
impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Etat::Idle)
            .insert_resource(CaseAdjacente { entities: vec![] })
            .insert_resource(ClearColor(Color::rgb(0.0, 0.75, 1.)))
            .insert_resource(DataPrint::None)
            .insert_resource(Moula {
                stone: 500,
                wood: 500,
                food: 0,
            });
    }
}

struct SyncroPlugin;
impl Plugin for SyncroPlugin {
    fn build(&self, app: &mut App) {
        app.sync_component::<BlockApartenance>()
            .sync_component::<BlockPosition>()
            .sync_component::<BlockType>()
            .sync_component::<TroupApartenance>()
            .sync_component::<TroupPosition>()
            .sync_component::<Troup>()
            .sync_client_event::<EventBuild>()
            .sync_client_event::<CombatEvent>()
            .sync_client_event::<ConqueteEvent>()
            .sync_client_event::<RecoltEvent>()
            .sync_client_event::<DestroyEvent>()
            .sync_client_event::<TrainEvent>()
            .sync_client_event::<DeplacementEvent>();
    }
}
