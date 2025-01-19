use crate::data::*;
use bevy::prelude::*;

pub fn used(
    keys: Res<Input<KeyCode>>,
    select: Option<Res<SelectedCase>>,
    identity: Option<Res<PacketIdentification>>,
    mut data_print: ResMut<DataPrint>,
    mut moula: ResMut<Moula>,
    mut event: EventWriter<RecoltEvent>,
    block: Query<(&BlockType, &BlockApartenance, &BlockPosition)>,
    mut etat: ResMut<Etat>,
    mut tour: Option<ResMut<Tour>>,
) {
    if let Some(identity) = identity {
        for touche in keys.get_just_pressed() {
            if *touche == KeyCode::R {
                if let Some(ref select) = select {
                    if tour.as_ref().unwrap().current_player == *identity {
                        for (type_block, apartenance, pos) in block.iter() {
                            if *pos == select.blockpos {
                                if *apartenance == identity.get_block_apartenance() {
                                    match type_block {
                                        // BlockType::AvantPost => todo!(),
                                        BlockType::Caserne => {
                                            *etat = Etat::Train;
                                            *data_print = DataPrint::Train
                                        }
                                        // BlockType::Castle => todo!(),
                                        // BlockType::Farm => todo!(),
                                        // BlockType::Grass => todo!(),
                                        BlockType::GrassForest => {
                                            moula.wood += 500;
                                            event.send(RecoltEvent {
                                                pos: select.blockpos,
                                            });
                                            tour.as_mut().unwrap().change_tour();
                                        }
                                        BlockType::GrassHill => {
                                            moula.stone += 500;
                                            event.send(RecoltEvent {
                                                pos: select.blockpos,
                                            });
                                            tour.as_mut().unwrap().change_tour();
                                        }
                                        // BlockType::Mine => todo!(),
                                        // BlockType::Tower => todo!(),
                                        // BlockType::Upgradeur => todo!(),
                                        // BlockType::Wall => todo!(),
                                        // BlockType::Sheep => todo!(),
                                        _ => {
                                            *data_print = DataPrint::Text {
                                                text: "this block can't be used".to_string(),
                                            }
                                        }
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
    }
}


