use crate::data::*;
use bevy::{math::vec3, prelude::*};

pub fn recadre(
    select_case: Option<Res<SelectedCase>>,
    mut query: Query<&mut Transform, With<SelectBar>>,
    time: Res<Time>,
) {
    if let Some(select_case) = select_case {
        for mut bar in query.iter_mut() {
            if bar.translation.x == 100. {
                bar.translation.x = select_case.pos.x;
                bar.translation.y = select_case.pos.y;
            } else {
                let delta_x = select_case.pos.x - bar.translation.x;
                let delta_y = select_case.pos.y - bar.translation.y;
                bar.translation.x += delta_x * time.delta_seconds() * 6.0;
                bar.translation.y += delta_y * time.delta_seconds() * 6.0;
            }
            let delta = (1.0 / 187.0) - bar.scale.x;
            bar.scale.x += delta * time.delta_seconds() * 10.0;
            bar.scale.y += delta * time.delta_seconds() * 10.0;
        }
    } else {
        for mut bar in query.iter_mut() {
            let delta = -bar.scale.x;
            bar.scale.x += delta * time.delta_seconds() * 10.0;
            bar.scale.y += delta * time.delta_seconds() * 10.0;
            if delta > -0.00001 {
                bar.translation.y = 100.;
                bar.translation.x = 100.;
            }
        }
    }
}
fn init_select_bar(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((SpriteBundle {
            texture: SelectBar {}.get_draw(&asset_server),
            transform: Transform::from_scale(Vec3::splat(1.0 / 187.0))
                .with_translation(vec3(100., 100., 50.)),
            ..default()
        },))
        .insert(SelectBar {});
}
pub struct PluginSelectRender;

impl Plugin for PluginSelectRender {
    fn build(&self, app: &mut App) {
        app.add_system(recadre).add_startup_system(init_select_bar);
    }
}
