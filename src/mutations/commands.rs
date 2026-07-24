use std::time::Duration;

use bevy::prelude::*;

use crate::{
    enemy::{DEFAULT_ENEMY_SPEED, Enemy, EnemySpeed, SplitEnemy, split_enemy},
    mutations::{
        EnemyAttractedToTarget, INITIAL_ENEMY_MUTATION_DURATION, MutationTimers, RotatingContainer,
    },
};

/// Enemy Speed Mutation

pub const ENEMY_SPEED_DELTA: f32 = 100.0;

pub struct ApplyEnemySpeedMutation;

impl Command for ApplyEnemySpeedMutation {
    type Out = ();

    fn apply(self, world: &mut World) -> Self::Out {
        let Some(mut speed) = world.get_resource_mut::<EnemySpeed>() else {
            warn!("Unable to find EnemySpeed resource in ApplyEnemySpeedMutation");
            return;
        };

        speed.0 += ENEMY_SPEED_DELTA;
    }
}

pub struct RemoveEnemySpeedMutation;

impl Command for RemoveEnemySpeedMutation {
    type Out = ();

    fn apply(self, world: &mut World) -> Self::Out {
        let Some(mut speed) = world.get_resource_mut::<EnemySpeed>() else {
            warn!("Unable to find EnemySpeed resource in RemoveEnemySpeedMutation");
            return;
        };

        speed.0 = (speed.0 - ENEMY_SPEED_DELTA).max(DEFAULT_ENEMY_SPEED);
    }
}

// Attracted to Target

pub const ADDITIONAL_ATTRACTION_FORCE: f32 = 750.0;

pub struct ApplyAttractedToTargetMutation;

impl Command for ApplyAttractedToTargetMutation {
    type Out = ();

    fn apply(self, world: &mut World) -> Self::Out {
        let mut enemies_query =
            world.query::<(Entity, &Enemy, Option<&mut EnemyAttractedToTarget>)>();
        let entities_to_insert = enemies_query
            .iter_mut(world)
            .filter_map(|(entity, _, maybe_attraction)| {
                let Some(mut attraction) = maybe_attraction else {
                    return Some(entity);
                };

                attraction.0 += ADDITIONAL_ATTRACTION_FORCE;

                None
            })
            .collect::<Vec<_>>();

        for entity in &entities_to_insert {
            world
                .commands()
                .entity(*entity)
                .insert(EnemyAttractedToTarget(ADDITIONAL_ATTRACTION_FORCE));
        }
    }
}

pub struct RemoveAttractedToTargetMutation;

impl Command for RemoveAttractedToTargetMutation {
    type Out = ();

    fn apply(self, world: &mut World) -> Self::Out {
        let mut enemies_query = world.query::<(Entity, &Enemy, &mut EnemyAttractedToTarget)>();

        let entities_to_remove = enemies_query
            .iter_mut(world)
            .filter_map(|(entity, _, mut attraction)| {
                attraction.0 -= ADDITIONAL_ATTRACTION_FORCE;

                if attraction.0 <= 0.0 {
                    return Some(entity);
                }

                None
            })
            .collect::<Vec<_>>();

        for entity in &entities_to_remove {
            world
                .commands()
                .entity(*entity)
                .remove::<EnemyAttractedToTarget>();
        }
    }
}

// Faster Mutation Timer

pub const ADDITIONAL_MUTATION_TIMER: f32 = 1.0;

pub struct ApplyFasterMutationTimerMutation;

impl Command for ApplyFasterMutationTimerMutation {
    type Out = ();

    fn apply(self, world: &mut World) -> Self::Out {
        let Some(mut timers) = world.get_resource_mut::<MutationTimers>() else {
            warn!("Unable to find MutationTimers, aborting ApplyFasterMutationTimerMutation");
            return;
        };

        let current_duration = (timers.enemy_mutation.duration()
            - Duration::from_secs_f32(ADDITIONAL_MUTATION_TIMER))
        .max(Duration::from_secs_f32(3.0));

        timers.enemy_mutation.set_duration(current_duration);
    }
}

pub struct RemoveFasterMutationTimerMutation;

impl Command for RemoveFasterMutationTimerMutation {
    type Out = ();

    fn apply(self, world: &mut World) -> Self::Out {
        let Some(mut timers) = world.get_resource_mut::<MutationTimers>() else {
            warn!("Unable to find MutationTimers, aborting RemoveFasterMutationTimerMutation");
            return;
        };

        let current_duration = (timers.enemy_mutation.duration()
            + Duration::from_secs_f32(ADDITIONAL_MUTATION_TIMER))
        .min(Duration::from_secs_f32(INITIAL_ENEMY_MUTATION_DURATION));

        timers.enemy_mutation.set_duration(current_duration);
    }
}

// Rotating container

pub struct ApplyRotatingContainerMutation;

impl Command for ApplyRotatingContainerMutation {
    type Out = ();

    fn apply(self, world: &mut World) -> Self::Out {
        let mut containers_query = world.query::<&mut RotatingContainer>();
        for mut container in containers_query.iter_mut(world) {
            container.0 = container.0.saturating_add(1);
        }
    }
}

pub struct RemoveRotatingContainerMutation;

impl Command for RemoveRotatingContainerMutation {
    type Out = ();

    fn apply(self, world: &mut World) -> Self::Out {
        let mut containers_query = world.query::<&mut RotatingContainer>();
        for mut container in containers_query.iter_mut(world) {
            container.0 = container.0.saturating_sub(1);
        }
    }
}

// Split

pub struct ApplySplitMutation;

impl Command for ApplySplitMutation {
    type Out = ();

    fn apply(self, world: &mut World) -> Self::Out {
        let mut enemies_query = world.query::<(Entity, &Transform, &Enemy)>();
        let Some((entity, tx, _)) = enemies_query.iter(world).next() else {
            warn!("No enemy found for ApplySplitMutation, aborting");
            return;
        };

        let tx = tx.translation.clone();
        world.commands().spawn_scene(split_enemy(entity, tx));
    }
}

pub struct RemoveSplitMutation;

impl Command for RemoveSplitMutation {
    type Out = ();

    fn apply(self, world: &mut World) -> Self::Out {
        let mut query = world.query::<(Entity, &SplitEnemy)>();
        let entities_to_despawn = query
            .iter(world)
            .filter_map(|(entity, split)| {
                if split.time_remaining <= 0.0 {
                    return Some(entity);
                }

                None
            })
            .collect::<Vec<_>>();

        for entity in &entities_to_despawn {
            world.commands().entity(*entity).despawn();
        }
    }
}
