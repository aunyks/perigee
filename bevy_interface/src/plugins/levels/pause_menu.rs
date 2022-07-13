use crate::states::GameLevel;
use bevy::prelude::*;

#[derive(Component)]
struct PauseMenuObject;

#[derive(Component)]
struct ResumeButton;

#[derive(Component)]
struct SettingsButton;

#[derive(Component)]
struct QuitButton;

/// This plugin manages gameplay for the pause menu level
pub struct PauseMenuLevel;

impl Plugin for PauseMenuLevel {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameLevel::PauseMenu).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(GameLevel::PauseMenu)
                    .with_system(change_button_style_on_interaction)
                    .with_system(enter_game_on_resume_game_clicked)
                    .with_system(quit_game_on_quit_game_clicked),
            )
            .add_system_set(
                SystemSet::on_exit(GameLevel::PauseMenu).with_system(teardown_pause_level),
            );
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(PauseMenuObject)
        .insert_bundle(UiCameraBundle::default());

    commands
        .spawn()
        .insert(PauseMenuObject)
        .insert_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::rgba(0f32, 0f32, 0f32, 0.75).into(),
            ..Default::default()
        })
        .with_children(|window_root| {
            window_root
                .spawn()
                .insert(PauseMenuObject)
                .insert_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(66.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        // horizontally center child text
                        flex_direction: FlexDirection::ColumnReverse,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|center_column| {
                    // Add title text
                    center_column
                        .spawn()
                        .insert(PauseMenuObject)
                        .insert(ResumeButton)
                        .insert_bundle(TextBundle {
                            text: Text::with_section(
                                "Paused",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            style: Style {
                                margin: Rect {
                                    top: Val::Px(20.0),
                                    bottom: Val::Px(20.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        });

                    // Add resume game button
                    center_column
                        .spawn()
                        .insert(PauseMenuObject)
                        .insert(ResumeButton)
                        .insert_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                                margin: Rect {
                                    top: Val::Px(20.0),
                                    bottom: Val::Px(20.0),
                                    ..Default::default()
                                },
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .with_children(|resume_game_button| {
                            resume_game_button
                                .spawn()
                                .insert(PauseMenuObject)
                                .insert_bundle(TextBundle {
                                    text: Text::with_section(
                                        "Resume",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 40.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                });
                        });

                    // Add settings button
                    center_column
                        .spawn()
                        .insert(PauseMenuObject)
                        .insert(SettingsButton)
                        .insert_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                                margin: Rect {
                                    top: Val::Px(20.0),
                                    bottom: Val::Px(20.0),
                                    ..Default::default()
                                },
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .with_children(|settings_button| {
                            settings_button
                                .spawn()
                                .insert(PauseMenuObject)
                                .insert_bundle(TextBundle {
                                    text: Text::with_section(
                                        "Settings",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 40.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                });
                        });

                    // Add quit button
                    center_column
                        .spawn()
                        .insert(PauseMenuObject)
                        .insert(QuitButton)
                        .insert_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                                margin: Rect {
                                    top: Val::Px(20.0),
                                    bottom: Val::Px(20.0),
                                    ..Default::default()
                                },
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .with_children(|settings_button| {
                            settings_button
                                .spawn()
                                .insert(PauseMenuObject)
                                .insert_bundle(TextBundle {
                                    text: Text::with_section(
                                        "Quit Game",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 40.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                });
                        });
                });
        });
}

fn change_button_style_on_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.35, 0.75, 0.35).into();
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

fn enter_game_on_resume_game_clicked(
    interaction_query: Query<&Interaction, With<ResumeButton>>,
    mut game_level: ResMut<State<GameLevel>>,
) {
    let interaction = interaction_query
        .get_single()
        .expect("Could not find a ResumeButton while setting it up to change GameLevel on click!");
    if *interaction == Interaction::Clicked {
        game_level
            .pop()
            .expect("Error occurred while popping GameLevel from the pause menu!");
    }
}

fn quit_game_on_quit_game_clicked(interaction_query: Query<&Interaction, With<QuitButton>>) {
    let interaction = interaction_query
        .get_single()
        .expect("Could not find a QuitButton while setting it up to quit the game on click!");
    if *interaction == Interaction::Clicked {
        std::process::exit(0);
    }
}

fn teardown_pause_level(mut commands: Commands, query: Query<Entity, With<PauseMenuObject>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
