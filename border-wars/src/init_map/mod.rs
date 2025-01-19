use crate::data;
use bevnet::server::{NetworkExt, Synced};
use bevy::prelude::*;
use data::*;

pub fn init_map(mut commands: Commands) {
    println!("World succesfully created");
    let max_y = 9;
    let max_x = 10;
    for y in 0..max_y {
        for x in 0..max_x {
            let mut curent_type = BlockType::Grass;
            let mut curent_apartenance = BlockApartenance::Neutre;
            if x == 1 && y == 4 {
                curent_type = BlockType::Castle;
                curent_apartenance = BlockApartenance::Joueur1;
            } else if x == max_y && y == 4 {
                curent_type = BlockType::Castle;
                curent_apartenance = BlockApartenance::Joueur2;
            } else {
                match (x, y) {
                    (0, 5) | (1, 5) | (2, 4) | (1, 3) | (0, 3) => {
                        curent_type = BlockType::Grass;
                        curent_apartenance = BlockApartenance::Joueur1
                    }

                    (9, 5) | (8, 5) | (8, 4) | (9, 3) | (8, 3) => {
                        curent_type = BlockType::Grass;
                        curent_apartenance = BlockApartenance::Joueur2
                    }

                    _ => {
                        if rand::random() {
                            if rand::random() {
                                curent_type = BlockType::GrassForest
                            } else {
                                curent_type = BlockType::GrassHill
                            }
                        }
                    }
                }
            }
            commands.spawn((
                curent_type,
                curent_apartenance,
                BlockPosition { x, y },
                Synced,
            ));
        }
    }
}

pub struct PluginMap;

impl Plugin for PluginMap {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_map)
            .sync_component::<BlockApartenance>()
            .sync_component::<BlockPosition>()
            .sync_component::<BlockType>();
    }
}
