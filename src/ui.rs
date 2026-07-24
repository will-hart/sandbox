use bevy::{
    color::palettes::css::RED,
    feathers::{controls::FeathersButton, palette::WHITE, theme::ThemedText},
    prelude::*,
    ui_widgets::Activate,
};

use crate::{enemy::SplitEnemy, mutations::MutationTimers, states::GameState};

pub(super) fn plugin(app: &mut App) {
    info!("Loading UI plugin");

    app.add_systems(OnEnter(GameState::MainMenu), menu.spawn())
        .add_systems(OnEnter(GameState::InGame), game_ui.spawn())
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
            )
        ]
    }
}

// spawn new items every frame, who cares, we rollin now
fn update_countdown_list(
    mut commands: Commands,
    timers: Res<MutationTimers>,
    parent: Single<Entity, With<CountdownTimerList>>,
    existing_items: Query<Entity, With<CountdownTimerItem>>,
    splits: Query<&SplitEnemy>,
) {
    for item in &existing_items {
        commands.entity(item).despawn();
    }

    // now prepare a list of countdown items
    let mut counters = vec![
        (
            format!("Mutate: {}", timers.enemy_mutation.remaining_secs().round()),
            timers.enemy_mutation.remaining_secs(),
        ),
        (
            format!("Defend: {}", timers.player_attack.remaining_secs().round()),
            timers.player_attack.remaining_secs(),
        ),
    ];
    counters.append(
        &mut splits
            .iter()
            .map(|split| {
                (
                    format!("Fork {}", split.time_remaining.round()),
                    split.time_remaining.max(0.0),
                )
            })
            .collect::<Vec<_>>(),
    );
    counters.sort_by(|a, b| b.1.total_cmp(&a.1));

    commands
        .entity(*parent)
        .queue_spawn_related_scenes::<Children>(bsn_list![{
            counters
                .into_iter()
                .map(|(value, _)| {
                    bsn! {
                        CountdownTimerItem
                        Text(value)
                        ThemedText
                    }
                })
                .collect::<Vec<_>>()
        }]);
}
