use bevy::prelude::*;

use crate::data::*;

pub fn selection(
    select_case: Option<ResMut<SelectedCase>>,
    // need to get window dimensions
    windows: Query<&Window>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<(&Transform, &BlockPosition, Entity)>,
    mut commands: Commands,
    identity: Option<Res<PacketIdentification>>,
) {
    if let Some(_identity) = identity {
        let (camera, camera_transform) = camera_q.single();

        if mouse_button_input.just_pressed(MouseButton::Left) {
            for window in windows.iter() {
                // check if the cursor is inside the window and get its position
                // then, ask bevy to convert into world coordinates, and truncate to discard Z
                if let Some(world_position) = window
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    if world_position.y > -0.97 {
                        let mut plus_petit: Option<(f32, &Transform, &BlockPosition, Entity)> =
                            None;
                        for (transform, blockpos, entity) in query.iter_mut() {
                            let block_pos_y = transform.translation.y - 0.45;

                            let delta_x = transform.translation.x - world_position.x;
                            let delta_y = block_pos_y - world_position.y;

                            let distance = delta_x * delta_x + delta_y * delta_y;

                            if plus_petit.is_none() || distance < plus_petit.unwrap().0 {
                                plus_petit = Some((distance, transform, blockpos, entity));
                            }
                        }
                        if let Some((_, transform, blockpos, _)) = plus_petit {
                            if let Some(position) = select_case.as_ref().map(|c| c.pos) {
                                if position == transform.translation {
                                    commands.remove_resource::<SelectedCase>();
                                } else {
                                    commands.insert_resource(SelectedCase {
                                        pos: plus_petit.unwrap().1.translation,
                                        entity: plus_petit.unwrap().3,
                                        blockpos: *blockpos,
                                    });
                                }
                            } else {
                                commands.insert_resource(SelectedCase {
                                    pos: plus_petit.unwrap().1.translation,
                                    entity: plus_petit.unwrap().3,
                                    blockpos: *blockpos,
                                })
                            }
                        }
                    }
                }
            }
        }
    }
}
