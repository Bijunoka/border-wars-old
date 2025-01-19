use crate::data::*;
use bevy::prelude::*;

use super::deplacement::*;

pub fn attaque(
    keys: Res<Input<KeyCode>>,
    query: Query<(&mut BlockApartenance, &BlockPosition, Entity)>,
    adjacentes: Res<CaseAdjacente>,
    select: Option<Res<SelectedCase>>,
    identity: Option<Res<PacketIdentification>>,
    mut event: EventWriter<ConqueteEvent>,
    troups: Query<(&TroupApartenance, &TroupPosition, &Troup)>,
    mut data_print: ResMut<DataPrint>,
    tour: Option<ResMut<Tour>>,
    mut combat: EventWriter<CombatEvent>,
) {
    if let Some(mut tour) = tour {
        if keys.just_pressed(KeyCode::E) {
            let mut case_a_moi = false;
            let mut peu_conquerir = 1;
            if let Some(identity) = identity {
                if let Some(ref select) = select {
                    if tour.current_player == *identity {
                        if let Ok((appartenance, _, _)) = query.get(select.entity) {
                            if identity.get_block_apartenance() == *appartenance {
                                peu_conquerir = 0
                            } else if identity.get_enemie().get_block_apartenance() == *appartenance
                            {
                                peu_conquerir = 2
                            }
                        }
                        for entity in &adjacentes.entities {
                            if let Ok((appartenance, _, _)) = query.get(*entity) {
                                if identity.get_block_apartenance() == *appartenance {
                                    case_a_moi = true;
                                    break;
                                }
                            }
                        }
                        if case_a_moi {
                            if peu_conquerir == 1 {
                                for (_, pos, _entity) in query.iter() {
                                    if *pos == select.blockpos {
                                        event.send(ConqueteEvent(*pos));
                                    }
                                }
                            } else if peu_conquerir == 2 {
                                let mut list: Vec<BlockPosition> = vec![];
                                for (_, block_pos, _) in query.iter() {
                                    list.push(*block_pos)
                                }

                                let mut attaque = false;
                                for (trou_apa, pos_t, troup) in troups.iter() {
                                    if *trou_apa == identity.get_troup_apartenance() {
                                        for pos_b in cases_adjacentes_rayon(
                                            &list,
                                            pos_t.to_troup_position(),
                                            troup.get_range(),
                                        ) {
                                            if pos_b == select.blockpos {
                                                attaque = true;
                                            }
                                        }
                                    }
                                }
                                if attaque {
                                    combat.send(CombatEvent {
                                        pos: select.blockpos,
                                    });
                                    tour.change_tour();
                                } else {
                                    *data_print = DataPrint::Text {
                                        text: "You don't have troups next to the case !"
                                            .to_string(),
                                    };
                                }
                            } else {
                                *data_print = DataPrint::Text {
                                    text: "This case is already belong to you".to_string(),
                                };
                            }
                        } else {
                            *data_print = DataPrint::Text {
                                text: "You are too far from your territory".to_string(),
                            }
                        }
                    } else {
                        *data_print = DataPrint::Text {
                            text: "It's not your turn ! Wait for your oppenent".to_string(),
                        };
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

pub struct AttaquePlugin;

impl Plugin for AttaquePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(attaque)
            .add_system(deplacer)
            .add_system(test);
    }
}
