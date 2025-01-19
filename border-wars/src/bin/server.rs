use bevnet::server::{Listener, NetworkExt, ServerPlugin};
use bevy::prelude::*;
use border_warsv2::change_map::jedetestetimeoduplusprofonddemonetrejusquaufinfonddemesentrailles;
use border_warsv2::data::*;
use border_warsv2::init_map::*;
use border_warsv2::server_systems::PluginGestionServeur;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(ServerPlugin)
        .insert_resource(Listener::bind("0.0.0.0:8000").unwrap())
        .insert_resource(Tour {
            current_player: PacketIdentification::Joueur1,
        })
        .sync_resource::<Tour>()
        .add_plugin(PluginMap)
        .sync_client_event::<EventBuild>()
        .sync_client_event::<TrainEvent>()
        .sync_client_event::<ConqueteEvent>()
        .sync_client_event::<RecoltEvent>()
        .add_plugin(PluginGestionServeur)
        .sync_client_event::<DestroyEvent>()
        .add_system(jedetestetimeoduplusprofonddemonetrejusquaufinfonddemesentrailles)
        .sync_component::<Troup>()
        .sync_component::<TroupPosition>()
        .sync_component::<TroupApartenance>()
        .sync_client_event::<DeplacementEvent>()
        .add_event::<ReponceCombatEvent>()
        .sync_client_event::<CombatEvent>()
        .run();
}
