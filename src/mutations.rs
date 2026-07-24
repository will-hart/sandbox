use std::time::Duration;

use avian2d::{
    dynamics::rigid_body::forces::{Forces, WriteRigidBodyForces},
    physics_transform::Position,
};
use bevy::prelude::*;

use crate::states::GameState;

mod commands;

pub const INITIAL_ENEMY_MUTATION_DURATION: f32 = 6.0;

pub(super) fn plugin(app: &mut App) {
    info!("Loading mutations plugin");
    app.init_resource::<MutationTimers>()
        .add_systems(
            OnEnter(GameState::InGame),
            (insert_mutations_resource, reset_mutation_timers),
        )
        .add_systems(
            Update,
            (
                watch_mutation_timers,
                attract_enemy_to_center,
                rotating_container,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_observer(handle_adding_or_removing_mutations);
}

#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct MutationTimers {
    pub enemy_mutation: Timer,
    pub player_attack: Timer,
}

impl Default for MutationTimers {
    fn default() -> Self {
        Self {
            enemy_mutation: Timer::from_seconds(
                INITIAL_ENEMY_MUTATION_DURATION,
                TimerMode::Repeating,
            ),
            player_attack: Timer::from_seconds(8.0, TimerMode::Repeating),
        }
    }
}

impl MutationTimers {
    pub fn tick(&mut self, amount: f32) {
        let duration = Duration::from_secs_f32(amount);
        self.enemy_mutation.tick(duration);
        self.player_attack.tick(duration);
    }
}

/// A set of potential mutations that an enemy or environment can have
#[derive(Clone, Copy, Debug, Reflect, Hash, Eq, PartialEq)]
pub enum EnemyMutations {
    EnemySpeed,
    AttractedToTarget,
    FasterMutationTimer,
    RotatingContainer,
}

#[derive(Clone, Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct CurrentMutations {
    /// vec as mutations can be applied more than once
    pub mutations: Vec<EnemyMutations>,
}

#[derive(Clone, Copy, Debug, Event)]
pub enum MutationEffect {
    Apply(EnemyMutations),
    Remove(EnemyMutations),
}

fn insert_mutations_resource(mut commands: Commands) {
    commands.insert_resource(CurrentMutations::default());
}

fn reset_mutation_timers(mut timers: ResMut<MutationTimers>) {
    timers.enemy_mutation.reset();
    timers.player_attack.reset();
}

fn watch_mutation_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<MutationTimers>,
    mut mutations: ResMut<CurrentMutations>,
) {
    timers.tick(time.delta_secs());

    if timers.enemy_mutation.just_finished() {
        // pick a random mutation to add
        if let Some(item) = fastrand::choice([
            EnemyMutations::EnemySpeed,
            EnemyMutations::AttractedToTarget,
            EnemyMutations::FasterMutationTimer,
            EnemyMutations::RotatingContainer,
        ]) {
            info!("Add enemy mutation - {item:?}");
            commands.trigger(MutationEffect::Apply(item));
            mutations.mutations.push(item);
        }
    }

    if timers.player_attack.just_finished() {
        if !mutations.mutations.is_empty()
            && let Some(idx) =
                fastrand::choice(mutations.mutations.iter().enumerate().map(|(idx, _)| idx))
        {
            let mutation = mutations.mutations.remove(idx);
            info!("Remove enemy mutation - {mutation:?}");
            commands.trigger(MutationEffect::Remove(mutation));
        };
    }
}

fn handle_adding_or_removing_mutations(trigger: On<MutationEffect>, mut commands: Commands) {
    let event = trigger.event();
    match event {
        MutationEffect::Apply(mutation) => match mutation {
            EnemyMutations::EnemySpeed => commands.queue(commands::ApplyEnemySpeedMutation),
            EnemyMutations::FasterMutationTimer => {
                commands.queue(commands::ApplyAttractedToTargetMutation)
            }
            EnemyMutations::AttractedToTarget => {
                commands.queue(commands::ApplyFasterMutationTimerMutation)
            }
            EnemyMutations::RotatingContainer => {
                commands.queue(commands::ApplyRotatingContainerMutation)
            }
        },
        MutationEffect::Remove(mutation) => match mutation {
            EnemyMutations::EnemySpeed => commands.queue(commands::RemoveEnemySpeedMutation),
            EnemyMutations::FasterMutationTimer => {
                commands.queue(commands::RemoveAttractedToTargetMutation)
            }
            EnemyMutations::AttractedToTarget => {
                commands.queue(commands::RemoveFasterMutationTimerMutation)
            }
            EnemyMutations::RotatingContainer => {
                commands.queue(commands::RemoveRotatingContainerMutation)
            }
        },
    }
}

/// Mutation: the enemy is attracted to the center
#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub struct EnemyAttractedToTarget(pub f32);

fn attract_enemy_to_center(mut enemies: Query<(Forces, &Position, &EnemyAttractedToTarget)>) {
    for (mut enemy, pos, attraction) in enemies.iter_mut() {
        let force_dir = -pos.0;
        let force = force_dir.normalize_or_zero() * attraction.0;
        enemy.apply_linear_impulse(force);
    }
}

/// Mutation: the container rotates
#[derive(Debug, Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct RotatingContainer(pub u8);

fn rotating_container(
    time: Res<Time>,
    mut containers: Query<(&mut Transform, &RotatingContainer)>,
) {
    for (mut container, rotation) in &mut containers {
        if rotation.0 == 0 {
            continue;
        }
        container.rotate_axis(Dir3::Z, 0.1 * time.delta_secs());
    }
}
