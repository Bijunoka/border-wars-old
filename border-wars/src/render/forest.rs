use crate::data::*;
use bevy::prelude::*;

pub fn forest(
    identity: Option<Res<PacketIdentification>>,
    mut troupq: Query<(&TroupPosition, &mut Visibility, &TroupApartenance)>,
    blocq: Query<(&BlockPosition, &BlockType)>,
) {
    if let Some(identity) = identity {
        for (troup_pos, mut sprite, troup_apa) in troupq.iter_mut() {
            for (pos, type_b) in blocq.iter() {
                if troup_pos.to_troup_position() == *pos {
                    if *type_b == BlockType::GrassForest {
                        if *troup_apa == identity.get_enemie().get_troup_apartenance() {
                            *sprite = Visibility::Hidden;
                        }
                    } else {
                        *sprite = Visibility::Visible;
                    }
                }
            }
        }
    }
}
