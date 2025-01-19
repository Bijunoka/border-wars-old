use bevy::prelude::*;
pub mod case_adjacente;
pub mod conquerir;
pub mod select_system;
use self::case_adjacente::calcule_cases_adjacentes;
use self::conquerir::AttaquePlugin;
use self::used::used;
use self::{build::*, select_system::selection};
mod build;
mod deplacement;

mod troupes;
mod used;
use crate::data::*;

use self::troupes::*;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BuildPlugin)
            .add_system(selection)
            .add_plugin(AttaquePlugin)
            .add_system(calcule_cases_adjacentes)
            .add_system(used)
            .add_system(destroy)
            .add_system(creat_troup)
            .add_system(get_ressource)
            .add_startup_system(musique);
    }
}

fn musique(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("zik.ogg");
    audio.play_with_settings(music, PlaybackSettings::LOOP.with_volume(0.75));
}

fn get_ressource(
    mut current_tour: Local<Option<Tour>>,
    tour: Option<Res<Tour>>,
    mut moula: ResMut<Moula>,
    blocks: Query<(&BlockType, &BlockApartenance)>,
    identity: Option<Res<PacketIdentification>>,
) {
    if let Some(identity) = identity {
        let go = if tour.is_some() {
            if current_tour.is_some() {
                if **tour.as_ref().unwrap() != current_tour.unwrap() {
                    *current_tour = Some(*tour.unwrap());
                    true
                } else {
                    false
                }
            } else {
                *current_tour = Some(*tour.unwrap());
                true
            }
        } else {
            false
        };
        if go {
            for (type_b, apa) in blocks.iter() {
                let id = *identity;
                if id.get_block_apartenance() == *apa {
                    match type_b {
                        BlockType::Farm => moula.wood += 100,
                        BlockType::Mine => moula.stone += 100,
                        BlockType::Sheep => moula.food += 25,
                        _ => (),
                    }
                }
            }
        }
    }
}
