use bevy::{
    color::palettes::css::RED,
    feathers::{
        controls::{ButtonVariant, FeathersButton},
        palette::WHITE,
        theme::ThemedText,
    },
    prelude::*,
    ui_widgets::Activate,
};

use crate::{enemy::SplitEnemy, mutations::MutationTimers, states::GameState};

pub(super) fn plugin(app: &mut App) {
    info!("Loading UI plugin");

    app.add_systems(OnEnter(GameState::MainMenu), menu.spawn())
        .add_systems(OnEnter(GameState::InGame), game_ui.spawn())
        .add_systems(OnEnter(GameState::GameOver), game_over.spawn())
        .add_systems(
            Update,
            update_countdown_list.run_if(in_state(GameState::InGame)),
        );
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
                    @caption: bsn!{ Text("Start Game") TextColor(WHITE) },
                    @variant: ButtonVariant::Primary
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

fn game_over() -> impl Scene {
    info!("Spawning game over menu");
    bsn! {
        DespawnOnExit<GameState>(GameState::GameOver)
        Node {
            width: percent(100),
            height: percent(100),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            top: px(0),
            left: px(0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: px(15)
        }
        Children [
            (
                @FeathersButton {
                    @caption: bsn!{ Text("Try Again") TextColor(WHITE) },
                    @variant: ButtonVariant::Primary
                }
                Node {
                    width: px(300),
                    height: px(150)
                }
                BackgroundColor(RED)
                on(|_: On<Activate>, mut next_state: ResMut<NextState<GameState>>| {
                    info!("Game Over - start game clicked");
                    next_state.set(GameState::InGame);
                })
            ),
            (
                @FeathersButton {
                    @caption: bsn!{ Text("Main Menu") TextColor(WHITE) }
                }
                Node {
                    width: px(300),
                    height: px(150)
                }
                BackgroundColor(RED)
                on(|_: On<Activate>, mut next_state: ResMut<NextState<GameState>>| {
                    info!("Game Over - main menu clicked");
                    next_state.set(GameState::MainMenu);
                })
            )
        ]
    }
}

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
struct CountdownTimerList;

#[derive(Debug, Clone, Default, Component, Reflect)]
#[reflect(Component)]
struct CountdownTimerItem;

fn game_ui() -> impl Scene {
    bsn! {
        DespawnOnExit<GameState>(GameState::InGame)
        Node {
            width: px(300),
            height: percent(100),
            padding: px(10),
            position_type: PositionType::Absolute,
            top: px(0),
            left: px(0),
            flex_direction: FlexDirection::Column,
        }
        CountdownTimerList
        Children [
            (
                Text("Countdowns")
                ThemedText
            ),
            countdown_text(""),
            countdown_text(""),
            countdown_text(""),
            countdown_text(""),
            countdown_text(""),
            countdown_text(""),
            countdown_text(""),
            countdown_text(""),
            countdown_text(""),
            countdown_text(""),
        ]
    }
}

fn countdown_text(value: &str) -> impl Scene {
    bsn! {
        CountdownTimerItem
        Text(value)
        ThemedText
    }
}

// spawn new items every frame, who cares, we rollin now
fn update_countdown_list(
    timers: Res<MutationTimers>,
    mut texts: Query<&mut Text, With<CountdownTimerItem>>,
    splits: Query<&SplitEnemy>,
) {
    // now prepare a list of countdown items
    let mut counters = vec![
        (
            format!(
                "{}s - Mutate",
                timers.enemy_mutation.remaining_secs().round()
            ),
            timers.enemy_mutation.remaining_secs(),
        ),
        (
            format!(
                "{}s - Defend",
                timers.player_attack.remaining_secs().round()
            ),
            timers.player_attack.remaining_secs(),
        ),
    ];
    counters.append(
        &mut splits
            .iter()
            .map(|split| {
                (
                    format!("{}s - Fork", split.time_remaining.round()),
                    split.time_remaining.max(0.0),
                )
            })
            .collect::<Vec<_>>(),
    );
    counters.sort_by(|a, b| a.1.total_cmp(&b.1));

    for (idx, mut text) in texts.iter_mut().enumerate() {
        if let Some((val, _)) = counters.get(idx) {
            text.0 = val.clone();
        } else {
            text.0 = String::new();
        }
    }
}
