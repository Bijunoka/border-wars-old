use crate::data::*;

use bevy::prelude::*;

pub fn calcule_cases_adjacentes(
    positions: Query<(&BlockPosition, Entity)>,
    select: Option<Res<SelectedCase>>,
    mut case_adjacente: ResMut<CaseAdjacente>,
) {
    if let Some(select) = select {
        case_adjacente.entities = positions
            .iter()
            .filter_map(|(position, entity)| {
                let x = position.x as i8 - select.blockpos.x as i8;
                let y = position.y as i8 - select.blockpos.y as i8;
                if select.blockpos.y % 2 == 0 {
                    match (x, y) {
                        (1, 1) | (1, -1) => None,
                        (-1..=1, -1..=1) => Some(entity),
                        _ => None,
                    }
                } else {
                    match (x, y) {
                        (-1, 1) | (-1, -1) => None,
                        (-1..=1, -1..=1) => Some(entity),
                        _ => None,
                    }
                }
            })
            .collect();
    } else {
        case_adjacente.entities.clear();
    }
}
