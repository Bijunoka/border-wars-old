use crate::data::*;
use bevy::prelude::*;

pub struct BuildPlugin;

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(build)
            .add_system(gestion_build.run_if(resource_exists::<Tour>()));
    }
}

fn build(
    keys: Res<Input<KeyCode>>,
    identity: Option<Res<PacketIdentification>>,
    selected_case: Option<Res<SelectedCase>>,
    query: Query<(&BlockPosition, &BlockApartenance, &BlockType)>,
    mut etat: ResMut<Etat>,
    mut data_print: ResMut<DataPrint>,
) {
    if let Some(identity) = identity {
        if keys.just_pressed(KeyCode::A) {
            if let Some(ref selected_case) = selected_case {
                for (pos, apartenance, type_b) in query.iter() {
                    if *pos == selected_case.blockpos {
                        if identity.get_block_apartenance() == *apartenance {
                            if *type_b == BlockType::Grass {
                                *etat = Etat::Building;
                            } else {
                                *data_print = DataPrint::Text {
                                    text: "There is already somethings here !".to_string(),
                                }
                            }
                        } else {
                            *data_print = DataPrint::Text {
                                text: "This case is not your ".to_string(),
                            }
                        }
                    }
                }
            } else {
                *data_print = DataPrint::Text {
                    text: "You must select a case".to_string(),
                }
            }
        }
    }
}

fn gestion_build(
    identity: Option<Res<PacketIdentification>>,
    keys: Res<Input<KeyCode>>,
    entity: Option<ResMut<SelectedCase>>,
    mut etat: ResMut<Etat>,
    mut event: EventWriter<EventBuild>,
    mut data_print: ResMut<DataPrint>,
    mut moula: ResMut<Moula>,
    mut tour: ResMut<Tour>,
) {
    if *etat == Etat::Building {
        if let Some(identity) = identity.as_ref() {
            if tour.current_player == **identity {
                *data_print = DataPrint::AllBlock;
                for keycode in keys.get_just_pressed() {
                    println!("{:?}",keycode);
                    let touche: Option<BlockType> = match keycode {
                        KeyCode::W => Some(BlockType::Caserne),
                        KeyCode::X => Some(BlockType::AvantPost),
                        KeyCode::C => Some(BlockType::Mine),
                        KeyCode::V => Some(BlockType::Farm),
                        KeyCode::B => Some(BlockType::Tower),
                        KeyCode::Key8 => Some(BlockType::Upgradeur),
                        KeyCode::Key7 => Some(BlockType::Wall),
                        KeyCode::N => Some(BlockType::Sheep),
                        KeyCode::A => continue,
                        _ => None,
                    };
                    if let Some(block_type) = touche {
                        if let Some(entity) = &entity {
                            if can_build(block_type, *moula) {
                                moula.stone -= block_type.get_price()[0];
                                moula.wood -= block_type.get_price()[1];
                                event.send(EventBuild {
                                    pos: entity.blockpos,
                                    type_b: block_type,
                                    who_are_u: **identity,
                                });
                                tour.change_tour();
                                *data_print = DataPrint::None;
                            } else {
                                *data_print = DataPrint::Text {
                                    text: "You haven't all materials needed".to_string(),
                                }
                            }
                            *etat = Etat::Idle;
                        }
                    } else {
                        *data_print = DataPrint::None;
                        *etat = Etat::Idle;
                    }
                }
            } else {
                *data_print = DataPrint::Text {
                    text: "It's not your turn ! Wait for your oppenent".to_string(),
                };
                *etat = Etat::Idle;
            }
        }
    }
}

pub fn destroy(
    keys: Res<Input<KeyCode>>,
    identity: Option<Res<PacketIdentification>>,
    selected_case: Option<Res<SelectedCase>>,
    query: Query<(&BlockPosition, &BlockApartenance, &BlockType)>,
    mut data_print: ResMut<DataPrint>,
    mut event: EventWriter<DestroyEvent>,
    tour: Option<ResMut<Tour>>,
) {
    if let Some(identity) = identity {
        if let Some(tour) = tour {
            if keys.just_pressed(KeyCode::D) {
                if let Some(ref selected_case) = selected_case {
                    if tour.current_player == *identity {
                        for (pos, apartenance, type_b) in query.iter() {
                            if *pos == selected_case.blockpos {
                                if identity.get_block_apartenance() == *apartenance {
                                    if type_b.is_breakable() {
                                        event.send(DestroyEvent {
                                            pos: selected_case.blockpos,
                                        })
                                    } else {
                                        *data_print = DataPrint::Text {
                                            text: "You can't destroy this case".to_string(),
                                        }
                                    }
                                } else {
                                    *data_print = DataPrint::Text {
                                        text:
                                            "You can't destroy cases if they aren't belong to you"
                                                .to_string(),
                                    }
                                }
                            } else {
                                *data_print = DataPrint::Text {
                                    text: "Any case are selected".to_string(),
                                }
                            }
                        }
                    } else {
                        *data_print = DataPrint::Text {
                            text: "It's not your turn ! Wait for your oppenent".to_string(),
                        };
                    }
                }
            }
        }
    }
}
