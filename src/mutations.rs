use std::time::Duration;

use bevy::prelude::*;

use crate::states::GameState;

pub const INITIAL_ENEMY_MUTATION_DURATION: f32 = 6.0;
pub const INITIAL_ENEMY_SPAWN_TIMER: f32 = 5.0;
pub const INITIAL_INCREASE_MUTATION_COUNT_TIMER: f32 = 25.0;

pub(super) fn plugin(app: &mut App) {
    info!("Loading mutations plugin");
    app.init_resource::<MutationTimers>()
        .add_systems(OnEnter(GameState::InGame), insert_mutations_resource)
        .add_systems(
            Update,
            watch_mutation_timers.run_if(in_state(GameState::InGame)),
        );
}

#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct MutationTimers {
    pub change_world_mutation: Timer,
    pub change_enemy_mutation: Timer,
    pub increase_mutation_count: Timer,
    pub spawn_enemy: Timer,
    pub mutation_count: u8,
}

impl Default for MutationTimers {
    fn default() -> Self {
        Self {
            change_world_mutation: Timer::from_seconds(10.0, TimerMode::Repeating),
            change_enemy_mutation: Timer::from_seconds(
                INITIAL_ENEMY_MUTATION_DURATION,
                TimerMode::Repeating,
            ),
            spawn_enemy: Timer::from_seconds(INITIAL_ENEMY_SPAWN_TIMER, TimerMode::Repeating),
            increase_mutation_count: Timer::from_seconds(
                INITIAL_INCREASE_MUTATION_COUNT_TIMER,
                TimerMode::Repeating,
            ),
            mutation_count: 0,
        }
    }
}

impl MutationTimers {
    fn tick(&mut self, amount: f32) {
        let duration = Duration::from_secs_f32(amount);

        self.change_enemy_mutation.tick(duration);
        self.increase_mutation_count.tick(duration);
        self.change_world_mutation.tick(duration);
        self.spawn_enemy.tick(duration);
    }
}

/// A set of potential mutations that an enemy or environment can have
#[derive(Clone, Copy, Debug, Reflect, Hash, Eq, PartialEq)]
pub enum EnemyMutations {
    EnemySpeed,
    AttractedToTarget,
    FasterMutationTimer,
    RotatingContainer,
    Split,
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

fn watch_mutation_timers(
    time: Res<Time>,
    mut timers: ResMut<MutationTimers>,
    mut mutations: ResMut<CurrentMutations>,
) {
    timers.tick(time.delta_secs());

    if timers.change_enemy_mutation.just_finished() {
        mutations.mutations = fastrand::choose_multiple(
            [
                EnemyMutations::EnemySpeed,
                EnemyMutations::AttractedToTarget,
                EnemyMutations::FasterMutationTimer,
                EnemyMutations::RotatingContainer,
                EnemyMutations::Split,
                EnemyMutations::Split,
                EnemyMutations::Split,
                EnemyMutations::Split,
                EnemyMutations::Split,
                EnemyMutations::Split,
                EnemyMutations::Split,
                EnemyMutations::Split,
            ],
            timers.mutation_count as usize,
        );
    }

    if timers.increase_mutation_count.just_finished() {
        timers.mutation_count = timers.mutation_count.saturating_add(1);
    }
}

// /// Mutation: the container rotates
// #[derive(Debug, Clone, Component, Default, Reflect)]
// #[reflect(Component)]
// pub struct RotatingContainer(pub u8);

// fn rotating_container(
//     time: Res<Time>,
//     mut containers: Query<(&mut Transform, &RotatingContainer)>,
// ) {
//     for (mut container, rotation) in &mut containers {
//         if rotation.0 == 0 {
//             continue;
//         }
//         container.rotate_axis(Dir3::Z, 0.1 * time.delta_secs());
//     }
// }
