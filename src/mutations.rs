use avian2d::{
    dynamics::rigid_body::forces::{Forces, WriteRigidBodyForces},
    physics_transform::Position,
};
use bevy::prelude::*;

use crate::states::GameState;

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
            ),
        );
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
            enemy_mutation: Timer::from_seconds(5.0, TimerMode::Repeating),
            player_attack: Timer::from_seconds(8.0, TimerMode::Repeating),
        }
    }
}

/// A set of potential mutations that an enemy or environment can have
#[derive(Clone, Copy, Debug, Reflect, Hash, Eq, PartialEq)]
pub enum EnemyMutations {
    AttractedToTarget,
    RotatingContainer,
}

#[derive(Clone, Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct CurrentMutations {
    /// vec as mutations can be applied more than once
    pub mutations: Vec<EnemyMutations>,
}

fn insert_mutations_resource(mut commands: Commands) {
    commands.insert_resource(CurrentMutations::default());
}

fn reset_mutation_timers(mut timers: ResMut<MutationTimers>) {
    timers.enemy_mutation.reset();
    timers.player_attack.reset();
}

fn watch_mutation_timers(timers: Res<MutationTimers>) {
    if timers.enemy_mutation.just_finished() {
        info!("Add enemy mutation");
    }

    if timers.player_attack.just_finished() {
        info!("Remove enemy mutation");
    }
}

/// Mutation: the enemy is attracted to the center
#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub struct EnemyAttractedToTarget;

fn attract_enemy_to_center(mut enemies: Query<(Forces, &Position), With<EnemyAttractedToTarget>>) {
    for (mut enemy, pos) in enemies.iter_mut() {
        let force_dir = -pos.0;
        let force = force_dir.normalize_or_zero() * 750.;
        enemy.apply_linear_impulse(force);
    }
}

/// Mutation: the container rotates
#[derive(Debug, Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct RotatingContainer;

fn rotating_container(
    time: Res<Time>,
    mut containers: Query<&mut Transform, With<RotatingContainer>>,
) {
    for mut container in &mut containers {
        container.rotate_axis(Dir3::Z, 0.1 * time.delta_secs());
    }
}
