use bevy::{
    color::palettes::css::RED,
    feathers::{controls::FeathersButton, palette::WHITE},
    prelude::*,
    ui_widgets::Activate,
};

use crate::states::GameState;

pub(super) fn plugin(app: &mut App) {
    info!("Loading UI plugin");

    app.add_systems(OnEnter(GameState::MainMenu), menu.spawn())
        .add_systems(OnEnter(GameState::InGame), game_ui.spawn());
}

fn menu() -> impl Scene {
    info!("Spawning game menu");
    bsn! {
        DespawnOnExit<GameState>(GameState::MainMenu)
        Node {
            width: percent(100),
            height: percent(100),
            position_type: PositionType::Absolute,
            top: px(0),
            left: px(0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center
        }
        Children [
            (
                @FeathersButton {
                    @caption: bsn!{ Text("Start Game") TextColor(WHITE) }
                }
                Node {
                    width: px(300),
                    height: px(150)
                }
                BackgroundColor(RED)
                on(|_: On<Activate>, mut next_state: ResMut<NextState<GameState>>| {
                    info!("Main menu - start game clicked");
                    next_state.set(GameState::InGame);
                })
            )
        ]
    }
}

fn game_ui() -> impl Scene {
    bsn! {
        DespawnOnExit<GameState>(GameState::InGame)
        Node {
            width: percent(100),
            height: px(30),
            position_type: PositionType::Absolute,
            top: px(0),
            left: px(0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center
        }
        Children [
            Text("Menu")
        ]
    }
}
