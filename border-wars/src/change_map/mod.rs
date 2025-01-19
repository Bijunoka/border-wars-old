use crate::data::*;
use bevnet::server::FromClient;
use bevy::prelude::*;

pub fn jedetestetimeoduplusprofonddemonetrejusquaufinfonddemesentrailles(
    mut reader: EventReader<FromClient<ConqueteEvent>>,
    mut positions: Query<(&BlockPosition, &mut BlockApartenance)>,
    paketid: Query<&PacketIdentification>,
    mut tour: ResMut<Tour>,
) {
    for event in reader.iter() {
        if let Ok(idenity) = paketid.get(event.entity) {
            if tour.current_player == *idenity {
                let mut changement = false;
                let mut option = None;
                for (pos, appa) in positions.iter_mut() {
                    let x = pos.x as i8 - event.0.x as i8;
                    let y = pos.y as i8 - event.0.y as i8;
                    let appartenance = if event.0.y % 2 == 0 {
                        match (x, y) {
                            (1, 1) | (1, -1) => None,
                            (-1..=1, -1..=1) => Some(appa.clone()),
                            _ => None,
                        }
                    } else {
                        match (x, y) {
                            (-1, 1) | (-1, -1) => None,
                            (-1..=1, -1..=1) => Some(appa.clone()),
                            _ => None,
                        }
                    };
                    if let Some(appartenance) = appartenance {
                        if idenity.get_block_apartenance() == appartenance {
                            changement = true;
                        }
                    }
                    if *pos == event.0 {
                        option = Some(appa);
                    }
                    if let Some(ref mut appar) = option {
                        if changement {
                            **appar = idenity.get_block_apartenance()
                        }
                    }
                }
                tour.change_tour()
            }
        }
    }
    // for select in reader.iter() {
    //     let case_adjacente: Vec<&BlockApartenance> = *positions
    //         .iter()
    //         .filter_map(|(position, appartenance)| {
    //             let x = position.x as i8 - select.0.x as i8;
    //             let y = position.y as i8 - select.0.y as i8;
    //             if select.0.y % 2 == 0 {
    //                 match (x, y) {
    //                     (1, 1) | (1, -1) => None,
    //                     (-1..=1, -1..=1) => Some(appartenance),
    //                     _ => None,
    //                 }
    //             } else {
    //                 match (x, y) {
    //                     (-1, 1) | (-1, -1) => None,
    //                     (-1..=1, -1..=1) => Some(appartenance),
    //                     _ => None,
    //                 }
    //             }
    //         })
    //         .filter(|&appartenance| match paketid.get(select.entity) {
    //             Ok(ok) => true,
    //             Err(paok) => false,
    //         })
    //         .collect();
    //     if !case_adjacente.is_empty() {
    //         for (position, mut appartenancess) in positions.iter_mut() {
    //             if *position == select.0 {
    //                 appartenancess = case_adjacente[0];
    //             }
    //         }
    //     }
    // }
}
