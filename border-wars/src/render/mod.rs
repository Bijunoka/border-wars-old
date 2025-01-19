use crate::data::*;
use bevy::{prelude::*, window::WindowResized};

mod forest;
mod interface;
mod select_system_renderer;

use self::forest::*;
use self::interface::InterfacePlugin;
use self::select_system_renderer::PluginSelectRender;

/// Configures every spawned tile for rendering.
fn tile_setup(
    mut commands: Commands,
    query: Query<(Entity, &BlockType, &BlockPosition, &BlockApartenance), Without<Sprite>>,
    asset_server: Res<AssetServer>,
    identity: Option<Res<PacketIdentification>>,
) {
    if let Some(identity) = identity {
        for (entity, type_block, pos, _) in query.iter() {
            if pos.x == 0 && pos.y % 2 == 0 {
                continue;
            }

            commands.entity(entity).insert(SpriteBundle {
                texture: type_block.get_draw(&asset_server),
                transform: pos
                    .to_transform(*identity)
                    .with_scale(Vec3::splat(1.0 / 185.0)),

                ..default()
            });
        }
    }
}

fn troup_setup(
    mut commands: Commands,
    query: Query<(Entity, &Troup, &TroupPosition, &TroupApartenance), Without<Sprite>>,
    asset_server: Res<AssetServer>,
    identity: Option<Res<PacketIdentification>>,
) {
    if let Some(identity) = identity {
        for (entity, type_block, pos, _) in query.iter() {
            commands.entity(entity).insert(SpriteBundle {
                texture: type_block.get_image(&asset_server),
                transform: pos
                    .to_transform(*identity)
                    .with_scale(Vec3::splat(1.0 / 1300.0)),

                ..default()
            });
        }
    }
}

fn tile_change(
    mut query: Query<
        (
            Entity,
            &mut BlockType,
            &mut BlockApartenance,
            &mut Handle<Image>,
        ),
        Changed<BlockType>,
    >,
    asset_server: Res<AssetServer>,
    identity: Option<Res<PacketIdentification>>,
) {
    if let Some(_identity) = identity {
        for (_entity, type_block, _apartenance, mut image) in query.iter_mut() {
            *image = type_block.get_draw(&asset_server);
        }
    }
}

fn camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(5., 0.5, 100.),
        ..Default::default()
    });
}

fn resizer(
    resize_event: Res<Events<WindowResized>>,
    mut query: Query<&mut OrthographicProjection>,
) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        for mut projection in query.iter_mut() {
            projection.scale = 1.15 / e.height * 5.;
        }
    }
}

fn pos_troup_change(
    mut query: Query<(&mut Transform, &TroupPosition), Changed<TroupPosition>>,
    identity: Option<Res<PacketIdentification>>,
) {
    if let Some(identity) = identity {
        for (mut transform, pos) in query.iter_mut() {
            *transform = pos
                .to_transform(*identity)
                .with_scale(Vec3::splat(1.0 / 1300.0))
        }
    }
}

pub fn color_system(
    mut query: Query<(&mut Sprite, &BlockApartenance)>,
    identity: Option<Res<PacketIdentification>>,
) {
    if let Some(identity) = identity {
        let colors = match *identity {
            PacketIdentification::Joueur1 => (Color::rgb(0.8, 0.8, 1.0), Color::rgb(1.0, 0.7, 0.5)),
            PacketIdentification::Joueur2 => (Color::rgb(1.0, 0.7, 0.5), Color::rgb(0.8, 0.8, 1.0)),
            PacketIdentification::Spectateur => {
                (Color::rgb(0.8, 0.8, 1.0), Color::rgb(1.0, 0.7, 0.5))
            }
        };
        for (mut sprite, appartenance) in query.iter_mut() {
            sprite.color = match appartenance {
                BlockApartenance::Joueur1 => colors.0,
                BlockApartenance::Joueur2 => colors.1,
                BlockApartenance::Neutre => Color::rgb(1.0, 1.0, 1.0),
            };
        }
    }
}

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_system)
            .add_system(resizer)
            .add_system(tile_setup)
            .add_system(troup_setup)
            .add_system(color_system)
            .add_system(tile_change)
            .add_plugin(PluginSelectRender)
            .add_plugin(InterfacePlugin)
            .add_system(pos_troup_change)
            .add_system(forest);
    }
}
