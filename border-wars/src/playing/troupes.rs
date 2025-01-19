use crate::data::*;
use bevy::prelude::*;

pub fn creat_troup(
    identity: Option<Res<PacketIdentification>>,
    keys: Res<Input<KeyCode>>,
    entity: Option<ResMut<SelectedCase>>,
    mut etat: ResMut<Etat>,
    mut event: EventWriter<TrainEvent>,
    mut data_print: ResMut<DataPrint>,
    blocks: Query<(&BlockType, &BlockApartenance, &BlockPosition)>,
    tour: Option<ResMut<Tour>>,
    mut moula: ResMut<Moula>,
    troups: Query<&TroupPosition>,
    race: Option<Res<Race>>,
) {
    if *etat == Etat::Train {
        if let Some(mut tour) = tour {
            for keycode in keys.get_just_pressed() {
                let niveau: Option<NiveauTroup> = match keycode {
                    KeyCode::Key1 => Some(NiveauTroup::Niveau1),
                    KeyCode::Key2 => Some(NiveauTroup::Niveau2),
                    KeyCode::Key3 => Some(NiveauTroup::Niveau3),

                    KeyCode::E => continue,
                    _ => None,
                };
                if let Some(niveau) = niveau {
                    if let Some(_) = &entity {
                        if let Some(identity) = identity.as_ref() {
                            if tour.current_player == **identity {
                                let troup = Troup {
                                    race: **race.as_ref().unwrap(),
                                    niveau: niveau,
                                };
                                if can_train(troup, *moula) {
                                    moula.food -= troup.get_price();
                                    let mut my_grass_block: Vec<BlockPosition> = blocks
                                        .iter()
                                        .filter_map(|(&type_b, &appa, &pos)| {
                                            if identity.get_block_apartenance() == appa
                                                && type_b == BlockType::Grass
                                            {
                                                Some(pos)
                                            } else {
                                                None
                                            }
                                        })
                                        .collect();

                                    for index in 0..my_grass_block.len() {
                                        for pos_t in troups.iter() {
                                            if pos_t.to_troup_position() == my_grass_block[index] {
                                                my_grass_block[index] =
                                                    BlockPosition { x: 100, y: 100 };
                                            }
                                        }
                                    }
                                    if my_grass_block != vec![] && il_peu(&my_grass_block) {
                                        event.send(TrainEvent { niv: niveau });
                                        tour.change_tour();
                                        *data_print = DataPrint::None;
                                        *etat = Etat::Idle;
                                    } else {
                                        *data_print = DataPrint::Text {
                                            text: "There aren't place to train troup".to_string(),
                                        }
                                    }
                                    *etat = Etat::Idle;
                                } else {
                                    *data_print = DataPrint::Text {
                                        text: "You haven't all materials needed".to_string(),
                                    }
                                }
                                *etat = Etat::Idle;
                            }
                        }
                    }
                }
            }
        }
    }
}
