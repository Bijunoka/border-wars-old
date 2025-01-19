use crate::data::*;
use bevy::{
    math::vec3,
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
};

pub struct InterfacePlugin;
impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_iterface)
            .add_system(gestion_texte_info)
            .add_system(print_info_interface)
            .add_system(print_info_interface_prix)
            .add_system(affichage_moula)
            .add_system(affichage_race)
            .add_system(print_troup);
    }
}

fn init_iterface(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    race: Option<Res<Race>>,
    mut pass: Local<bool>,
) {
    if race.is_some() {
        if *pass == false {
            // block pour la beauté
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.25),
                    custom_size: Some(Vec2::new(10.15, 2.5)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(5., -2.2, 70.)),
                ..default()
            });

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.35, 0.35, 0.35),
                    custom_size: Some(Vec2::new(6.44, 2.5)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(3.325, -2.3, 71.)),
                ..default()
            });

            // Zone de texte pour les info des block
            commands.spawn((
                TextInfo,
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new("init", get_font(&asset_server, 20.))],
                        alignment: TextAlignment::Left,
                        linebreak_behaviour: BreakLineOn::WordBoundary,
                    },
                    text_2d_bounds: Text2dBounds {
                        // Wrap text in the rectangle
                        size: Vec2::new(300.22, 200.5),
                    },
                    // ensure the text is drawn on top of the box
                    transform: Transform::from_scale(Vec3::splat(1.0 / 150.0))
                        .with_translation(Vec3::new(8.29, -1.6, 72.)),
                    ..default()
                },
            ));

            // zonne qui affiche les probleme
            commands.spawn((
                TextInterface {},
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new("", get_font(&asset_server, 20.))],
                        alignment: TextAlignment::Left,
                        linebreak_behaviour: BreakLineOn::WordBoundary,
                    },
                    text_2d_bounds: Text2dBounds {
                        // Wrap text in the rectangle
                        size: Vec2::new(300.22, 200.5),
                    },
                    // ensure the text is drawn on top of the box
                    transform: Transform::from_scale(Vec3::splat(1.0 / 150.0))
                        .with_translation(Vec3::new(3.325, -1.6, 72.)),
                    ..default()
                },
            ));

            for x in 1..9 {
                commands.spawn((
                    BlockInfoBuild,
                    SpriteBundle {
                        texture: get_type_with_number(x).unwrap().get_draw(&asset_server),
                        transform: Transform::from_scale(Vec3::splat(1.0 / 300.0))
                            .with_translation(vec3(0.8 * x as f32 - 0.3, -1.1, 73.)),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                ));
                commands.spawn((
                    get_type_with_number(x).unwrap(),
                    Text2dBundle {
                        text: Text {
                            sections: vec![TextSection::new("", get_font(&asset_server, 10.))],
                            alignment: TextAlignment::Center,
                            linebreak_behaviour: BreakLineOn::WordBoundary,
                        },
                        text_2d_bounds: Text2dBounds {
                            // Wrap text in the rectangle
                            size: Vec2::new(256., 500.5),
                        },
                        // ensure the text is drawn on top of the box
                        transform: Transform::from_scale(Vec3::splat(1.0 / 150.0))
                            .with_translation(Vec3::new(0.8 * x as f32 - 0.3, -1.95, 75.)),
                        ..default()
                    },
                ));
            }

            for x in 1..4 {
                commands.spawn((
                    TroupInfoTrain,
                    SpriteBundle {
                        texture: Troup {
                            race: **race.as_ref().unwrap(),
                            niveau: get_troup_niveau(x),
                        }
                        .get_image(&asset_server),
                        transform: Transform::from_scale(Vec3::splat(1.0 / 900.0))
                            .with_translation(vec3(2.2 * x as f32 - 1.7, -1.8, 73.)),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                ));
                commands.spawn((
                    Troup {
                        race: **race.as_ref().unwrap(),
                        niveau: get_troup_niveau(x),
                    },
                    Text2dBundle {
                        text: Text {
                            sections: vec![TextSection::new("aaaaaaaaaaaaaaa\naaaaaaaaaaaaaaa\naaaaaaaaaaaaaaa\naaaaaaaaaaaaaaa\naaaaaaaaaaaaaaa\n", get_font(&asset_server, 15.))],
                            alignment: TextAlignment::Center,
                            linebreak_behaviour: BreakLineOn::WordBoundary,
                        },
                        text_2d_bounds: Text2dBounds {
                            // Wrap text in the rectangle
                            size: Vec2::new(256., 500.5),
                        },
                        // ensure the text is drawn on top of the box
                        transform: Transform::from_scale(Vec3::splat(1.0 / 150.0))
                            .with_translation(Vec3::new(2.2 * x as f32 - 0.8, -1.7, 75.)),
                        ..default()
                    },
                ));
            }

            // on affiche la moula

            commands.spawn((
                TextRessources {},
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new("", get_font(&asset_server, 20.))],
                        alignment: TextAlignment::Left,
                        linebreak_behaviour: BreakLineOn::WordBoundary,
                    },
                    text_2d_bounds: Text2dBounds {
                        // Wrap text in the rectangle
                        size: Vec2::new(500.22, 200.5),
                    },
                    // ensure the text is drawn on top of the box
                    transform: Transform::from_scale(Vec3::splat(1.0 / 100.0))
                        .with_translation(Vec3::new(1.7, 3.25, 72.)),
                    ..default()
                },
            ));

            // Race
            commands.spawn((
                TextRace {},
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new("", get_font(&asset_server, 20.))],
                        alignment: TextAlignment::Left,
                        linebreak_behaviour: BreakLineOn::WordBoundary,
                    },
                    text_2d_bounds: Text2dBounds {
                        // Wrap text in the rectangle
                        size: Vec2::new(500.22, 200.5),
                    },
                    // ensure the text is drawn on top of the box
                    transform: Transform::from_scale(Vec3::splat(1.0 / 100.0))
                        .with_translation(Vec3::new(8.7, 3.25, 72.)),
                    ..default()
                },
            ));
            *pass = true;
        }
    }
}

fn gestion_texte_info(
    selected_case: Option<Res<SelectedCase>>,
    mut query: Query<&mut Text, With<TextInfo>>,
    asset_server: Res<AssetServer>,
    identity: Option<Res<PacketIdentification>>,
    block: Query<(&BlockType, &BlockApartenance, &BlockPosition)>,
) {
    if let Some(identity) = identity {
        let font = get_font(&asset_server, 20.);
        for mut text in query.iter_mut() {
            if let Some(ref case) = selected_case {
                for (type_block, apartenance, pos) in block.iter() {
                    if *pos == case.blockpos {
                        let a = format!(
                            "This case is {}, {}",
                            type_block.get_text(),
                            apartenance.get_text(&identity)
                        );
                        print_text(&mut text, &a, font.clone())
                    }
                }
            } else {
                print_text(&mut text, "Any blocks are selected", font.clone())
            }
        }
    }
}

pub fn print_info_interface(
    mut query: Query<&mut Text, With<TextInterface>>,
    asset_server: Res<AssetServer>,
    data_print: Res<DataPrint>,
    mut blocs: Query<&mut Visibility, With<BlockInfoBuild>>,
) {
    let font = get_font(&asset_server, 20.);
    match data_print.as_ref() {
        DataPrint::None => {
            for mut block in blocs.iter_mut() {
                *block = Visibility::Hidden;
            }
        }

        DataPrint::AllBlock => {
            for mut block in blocs.iter_mut() {
                *block = Visibility::Visible;
            }
            for mut bloc in query.iter_mut() {
                print_text(&mut bloc, "", font.clone())
            }
        }

        DataPrint::Text { text } => {
            for mut block in blocs.iter_mut() {
                *block = Visibility::Hidden;
            }
            for mut bloc in query.iter_mut() {
                print_text(&mut bloc, text, font.clone())
            }
        }
        DataPrint::Train => {
            for mut bloc in query.iter_mut() {
                print_text(&mut bloc, "", font.clone())
            }
        }
    }
}

pub fn print_info_interface_prix(
    asset_server: Res<AssetServer>,
    data_print: Res<DataPrint>,
    mut blocs: Query<&mut Visibility, With<BlockInfoBuild>>,
    mut zones_texte: Query<(&mut Text, &BlockType)>,
) {
    let font = get_font(&asset_server, 18.);
    match data_print.as_ref() {
        DataPrint::None => {
            for (mut zone, _type_b) in zones_texte.iter_mut() {
                print_text(&mut zone, "", font.clone())
            }
        }
        DataPrint::Text { text: _ } => {
            for (mut zone, _) in zones_texte.iter_mut() {
                print_text(&mut zone, "", font.clone())
            }
        }
        DataPrint::AllBlock => {
            for mut block in blocs.iter_mut() {
                *block = Visibility::Visible;
            }
            for (mut zone, type_b) in zones_texte.iter_mut() {
                print_text(&mut zone, &type_b.get_text_with_typeb(), font.clone())
            }
        }
        DataPrint::Train => (),
    }
}

pub fn print_troup(
    asset_server: Res<AssetServer>,
    data_print: Res<DataPrint>,
    mut blocs: Query<&mut Visibility, With<TroupInfoTrain>>,
    mut zones_texte: Query<(&mut Text, &Troup)>,
) {
    let font = get_font(&asset_server, 18.);
    match data_print.as_ref() {
        DataPrint::Train => {
            for mut block in blocs.iter_mut() {
                *block = Visibility::Visible;
            }
            for (mut zone, troup) in zones_texte.iter_mut() {
                print_text(&mut zone, &troup.get_text(), font.clone())
            }
        }
        _ => {
            for mut block in blocs.iter_mut() {
                *block = Visibility::Hidden;
            }
            for (mut zone, _troup) in zones_texte.iter_mut() {
                print_text(&mut zone, "", font.clone())
            }
        }
    }
}

fn affichage_moula(
    mut zone_texte: Query<&mut Text, With<TextRessources>>,
    asset_server: Res<AssetServer>,
    moula: Res<Moula>,
) {
    let font = get_font(&asset_server, 20.);
    let a = format!(
        "{} wood    {} stone    {} food",
        moula.wood, moula.stone, moula.food
    );
    if let Ok(mut text) = zone_texte.get_single_mut() {
        print_text(&mut text, &a, font.clone())
    }
}

fn affichage_race(
    mut zone_texte: Query<&mut Text, With<TextRace>>,
    asset_server: Res<AssetServer>,
    race: Option<Res<Race>>,
) {
    if race.is_some() {
        let font = get_font(&asset_server, 20.);
        let caca = race.unwrap();
        let a = format!("Your Race is {:?}", *caca);
        if let Ok(mut text) = zone_texte.get_single_mut() {
            print_text(&mut text, &a, font.clone())
        }
    }
}
