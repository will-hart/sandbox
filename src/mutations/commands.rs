use bevy::prelude::*;

use crate::enemy::{DEFAULT_ENEMY_SPEED, EnemySpeed};

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

// Faster Mutation Timer
