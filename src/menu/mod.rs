use bevy::prelude::*;

use crate::GameState;

mod pause;

pub struct MenuPlugin;

#[derive(Debug, Component)]
struct SplashMarker;
#[derive(Debug, Component)]
struct MainMenuMarker;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(pause::PausePlugin)
            .add_systems(OnEnter(GameState::Splash), Self::splash_screen)
            .add_systems(Update, Self::countdown.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<SplashMarker>)
            .add_systems(OnEnter(GameState::Menu), Self::main_menu)
            .add_systems(Update, Self::menu_action.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), despawn_screen::<MainMenuMarker>);
    }
}

#[derive(Component, Debug)]
pub enum MenuButtonAction {
    Play,
    Exit,
}

const TITLE_FONT_PATH: &str = "fonts/title_font.ttf";
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

impl MenuPlugin {
    fn countdown(
        mut game_state: ResMut<NextState<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::Menu);
        }
    }

    fn splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            SplashMarker,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
            children![(
                Text::new("Stop Yourself!"),
                TextFont {
                    font: asset_server.load(TITLE_FONT_PATH),
                    font_size: 100.,
                    ..Default::default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
            )],
        ));

        commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    }

    fn main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
        let text_font = asset_server.load(TITLE_FONT_PATH);
        let button_node = Node {
            width: Val::Px(300.),
            height: Val::Px(70.),
            margin: UiRect::all(Val::Px(20.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(3.)),
            ..Default::default()
        };

        let button_text_font = TextFont {
            font_size: 33.,
            ..Default::default()
        };

        commands
            .spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                MainMenuMarker,
                BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    children![
                        (
                            Text::new("Stop Yourself!"),
                            TextFont {
                                font_size: 100.,
                                font: text_font,
                                ..Default::default()
                            },
                            TextColor(TEXT_COLOR),
                            Node {
                                margin: UiRect::all(Val::Px(50.)),
                                ..Default::default()
                            },
                        ),
                        (
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::Play,
                            BorderColor(Color::BLACK),
                            children![(
                                Text::new("Start!"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            )],
                        ),
                        (
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            BorderColor(Color::BLACK),
                            MenuButtonAction::Exit,
                            children![(
                                Text::new("Exit"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            )],
                        )
                    ],
                ));
            });
    }

    fn menu_action(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        action: Query<
            (&Interaction, &MenuButtonAction, &mut BackgroundColor),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_events: EventWriter<AppExit>,
        mut app_state: ResMut<NextState<GameState>>,
    ) {
        for (interaction, menu_action, mut background_color) in action {
            if *interaction == Interaction::Pressed {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("sounds/button_select.wav")),
                    PlaybackSettings {
                        volume: bevy::audio::Volume::Linear(0.5),
                        ..Default::default()
                    },
                ));
                match menu_action {
                    MenuButtonAction::Exit => {
                        app_exit_events.write(AppExit::Success);
                    }
                    MenuButtonAction::Play => {
                        app_state.set(GameState::Game);
                    }
                }
            } else if *interaction == Interaction::Hovered {
                commands.spawn((AudioPlayer::new(
                    asset_server.load("sounds/button_hover.wav"),
                ),));
            }

            *background_color = match interaction {
                Interaction::None => NORMAL_BUTTON.into(),
                Interaction::Pressed => PRESSED_BUTTON.into(),
                Interaction::Hovered => HOVERED_BUTTON.into(),
            }
        }
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}
