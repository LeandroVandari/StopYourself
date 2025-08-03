use bevy::{diagnostic::FrameCount, input::common_conditions::input_just_pressed, prelude::*};

use crate::{GameState, menu::MenuButtonAction, player::record_position::RecordedPositions};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::handle_pause.run_if(input_just_pressed(KeyCode::Escape)),
        )
        .add_systems(OnEnter(GameState::Paused), Self::spawn_pause_menu)
        .add_systems(
            Update,
            Self::pause_menu_action.run_if(in_state(GameState::Paused)),
        )
        .add_systems(
            OnExit(GameState::Paused),
            super::despawn_screen::<PauseMenuMarker>,
        )
        .init_resource::<FramePaused>();
    }
}

const NORMAL_BUTTON: Color = Color::srgba(0.25, 0.25, 0.25, 0.08);
const HOVERED_BUTTON: Color = Color::srgba(0.25, 0.25, 0.25, 0.15);

#[derive(Resource, Default)]
struct FramePaused(u32);

#[derive(Component)]
struct PauseMenuMarker;

impl PausePlugin {
    fn handle_pause(
        mut set_state: ResMut<NextState<GameState>>,
        get_state: Res<State<GameState>>,
        mut time: ResMut<Time<Virtual>>,
        mut positions: ResMut<RecordedPositions>,
        frame: Res<FrameCount>,
        mut frame_paused: ResMut<FramePaused>,
    ) {
        match get_state.get() {
            GameState::Game => {
                set_state.set(GameState::Paused);
                frame_paused.0 = frame.0;
                time.pause();
            }
            GameState::Paused => {
                Self::unpause(
                    &mut set_state,
                    &mut time,
                    &mut positions,
                    &frame,
                    &frame_paused,
                );
            }
            _ => (),
        }
    }

    fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
        let title_font = asset_server.load(super::TITLE_FONT_PATH);

        let button_node = Node {
            width: Val::Px(400.),
            height: Val::Px(70.),
            margin: UiRect::vertical(Val::Px(20.)),
            justify_content: JustifyContent::End,
            align_items: AlignItems::Center,
            ..Default::default()
        };

        let button_text_font = TextFont {
            font_size: 33.,
            font: asset_server.load("fonts/capitolcity.ttf"),
            ..Default::default()
        };

        commands.spawn((
            PauseMenuMarker,
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                ..Default::default()
            },
            children![
                (
                    Text::new("Stop Yourself!"),
                    TextFont {
                        font_size: 80.,
                        font: title_font,
                        ..Default::default()
                    },
                    TextColor(super::TEXT_COLOR),
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
                        Text::new("Continue"),
                        button_text_font.clone(),
                        TextColor(super::TEXT_COLOR),
                        Node {
                            margin: UiRect::right(Val::Px(20.)),
                            ..Default::default()
                        }
                    )],
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    BorderColor(Color::BLACK),
                    MenuButtonAction::Exit,
                    children![(
                        Node {
                            margin: UiRect::right(Val::Px(20.)),
                            ..Default::default()
                        },
                        Text::new("Exit"),
                        button_text_font.clone(),
                        TextColor(super::TEXT_COLOR),
                    )],
                )
            ],
        ));
    }

    fn pause_menu_action(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        action: Query<
            (&Interaction, &MenuButtonAction, &mut BackgroundColor),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_events: EventWriter<AppExit>,
        mut app_state: ResMut<NextState<GameState>>,
        mut time: ResMut<Time<Virtual>>,
        mut positions: ResMut<RecordedPositions>,
        frame: Res<FrameCount>,
        frame_paused: ResMut<FramePaused>,
    ) {
        for (interaction, menu_action, mut background_color) in action {
            if *interaction == Interaction::Pressed {
                commands.spawn(AudioPlayer::new(
                    asset_server.load("sounds/button_select.wav"),
                ));
                match menu_action {
                    MenuButtonAction::Exit => {
                        app_exit_events.write(AppExit::Success);
                    }
                    MenuButtonAction::Play => {
                        Self::unpause(
                            &mut app_state,
                            &mut time,
                            &mut positions,
                            &frame,
                            &frame_paused,
                        );
                    }
                }
            } else if *interaction == Interaction::Hovered {
                commands.spawn((AudioPlayer::new(
                    asset_server.load("sounds/button_hover.wav"),
                ),));
            }

            *background_color = match interaction {
                Interaction::None => NORMAL_BUTTON.into(),
                Interaction::Pressed => super::PRESSED_BUTTON.into(),
                Interaction::Hovered => HOVERED_BUTTON.into(),
            }
        }
    }

    fn unpause(
        set_state: &mut NextState<GameState>,
        time: &mut Time<Virtual>,
        positions: &mut RecordedPositions,
        frame: &FrameCount,
        frame_paused: &FramePaused,
    ) {
        let frames_since_pause = frame.0 - frame_paused.0;
        positions.frame_start += frames_since_pause;
        set_state.set(GameState::Game);
        time.unpause();
    }
}
