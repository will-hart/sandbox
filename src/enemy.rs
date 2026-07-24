use avian2d::{
    collision::collider::Collider,
    dynamics::rigid_body::{CoefficientCombine, LinearVelocity, Restitution, RigidBody},
};
use bevy::prelude::*;

use crate::states::GameState;

pub(super) fn plugin(app: &mut App) {
    info!("Loading enemy plugin");
    app.init_resource::<EnemySpeed>()
        .add_systems(OnEnter(GameState::InGame), enemy.spawn())
        .add_systems(
            Update,
            normalise_enemy_velocity_by_speed.run_if(in_state(GameState::InGame)),
        );
}

#[derive(Debug, Clone, Copy, Resource, Reflect)]
#[reflect(Resource)]
pub struct EnemySpeed(pub f32);

impl Default for EnemySpeed {
    fn default() -> Self {
        Self(200.0)
    }
}

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Enemy;
fn enemy() -> impl Scene {
    let color = Srgba::new(1.7, 1.0, 1.8, 1.0);
    let enemy_radius = 15.0;

    bsn! {
        Enemy
        DespawnOnExit<GameState>(GameState::InGame)
        Mesh2d(asset_value(Circle::new(enemy_radius)))
        MeshMaterial2d<ColorMaterial>(asset_value(ColorMaterial::from_color(color)))
        template_value(RigidBody::Dynamic)
        Collider::circle(enemy_radius)
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombine::Max
        }
        LinearVelocity(Vec2::new(150.0, -100.0))
        Transform {
            translation: Vec2 {
                x: 0.0,
                y: 80.0,
            }
        }
    }
}

fn normalise_enemy_velocity_by_speed(
    speed: Res<EnemySpeed>,
    mut enemies: Query<&mut LinearVelocity, With<Enemy>>,
) {
    for mut linvel in &mut enemies {
        linvel.0 = linvel.0.normalize_or_zero() * speed.0;
    }
}
