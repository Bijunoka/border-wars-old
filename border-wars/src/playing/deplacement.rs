use crate::data::*;

use bevy::prelude::*;

pub fn deplacer(
    query: Query<(&Troup, &TroupPosition, &TroupApartenance)>,
    pos: Query<(&BlockPosition, &mut Sprite, &BlockApartenance, &BlockType)>,
    select: Option<Res<SelectedCase>>,
    keys: Res<Input<KeyCode>>,
    identity: Option<Res<PacketIdentification>>,
    mut etat: ResMut<Etat>,
    mut data_print: ResMut<DataPrint>,
) {
    if let Some(identity) = identity {
        if !matches!(*etat, Etat::Move { .. }) {
            if keys.just_pressed(KeyCode::Z) {
                if let Some(ref select) = select {
                    for (troup, &pos_t, appa) in query.iter() {
                        if pos_t.to_troup_position() == select.blockpos {
                            if identity.get_troup_apartenance() == *appa {
                                let positions: Vec<BlockPosition> = pos
                                    .iter()
                                    .filter_map(|(&position, _, appa, typeb)| {
                                        let mut res = None;
                                        if *appa == identity.get_block_apartenance() {
                                            if typeb.is_empty() {
                                                for (_, troup_pos, _) in query.iter() {
                                                    if troup_pos.to_troup_position() != position {
                                                        res = Some(position);
                                                    } else {
                                                        res = None;
                                                        break;
                                                    }
                                                }
                                            } else {
                                                res = None;
                                            }
                                        } else {
                                            res = None;
                                        }
                                        res
                                    })
                                    .collect();

                                let adjacentes = cases_adjacentes_rayon(
                                    &positions,
                                    select.blockpos,
                                    troup.get_vitesse(),
                                );

                                *etat = Etat::Move {
                                    adjacentes,
                                    pos_d: select.blockpos,
                                };
                                *data_print = DataPrint::Text {
                                    text: "Where your soldiers will go ?".to_string(),
                                };
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn test(
    mut query: Query<(&BlockPosition, &mut Sprite)>,
    select: Option<Res<SelectedCase>>,
    identity: Option<Res<PacketIdentification>>,
    mut etat: ResMut<Etat>,
    mut event: EventWriter<DeplacementEvent>,
    mut data_print: ResMut<DataPrint>,
    tour: Option<ResMut<Tour>>,
) {
    if let Some(mut tour) = tour {
        let mut fini: Option<[BlockPosition; 2]> = None;
        if let Some(ref select) = select {
            if let Some(identity) = identity {
                if tour.current_player == *identity {
                    match &*etat {
                        Etat::Move { adjacentes, pos_d } => {
                            for (pos, mut sprite) in query.iter_mut() {
                                if adjacentes.contains(pos) {
                                    sprite.color = Color::rgb(1.0, 0.75, 0.0);
                                    if select.is_changed() {
                                        if adjacentes.contains(&select.blockpos) {
                                            fini = Some([select.blockpos, *pos_d]);
                                        } else {
                                            fini = Some([
                                                select.blockpos,
                                                BlockPosition { x: 100, y: 100 },
                                            ]);
                                            *data_print = DataPrint::Text {
                                                text: " You can't go here ".to_string(),
                                            }
                                        }
                                    }
                                } else {
                                    sprite.color = Color::rgb(1.0, 1.0, 1.0)
                                }
                            }
                            if fini.is_some() {
                                if let Some([start, end]) = fini {
                                    if end != (BlockPosition { x: 100, y: 100 }) {
                                        event.send(DeplacementEvent {
                                            pos_start: end,
                                            pos_end: start,
                                        });
                                                                            tour.change_tour();

                                        *data_print = DataPrint::None;
                                    }
                                    *etat = Etat::Idle;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
