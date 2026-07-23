use avian2d::{
    collision::collider::Collider,
    dynamics::rigid_body::{
        CoefficientCombine, LinearVelocity, Restitution, RigidBody,
        forces::{Forces, WriteRigidBodyForces},
    },
    physics_transform::Position,
};
use bevy::prelude::*;

use crate::states::GameState;

pub(super) fn plugin(app: &mut App) {
    info!("Loading enemy plugin");
    app.add_systems(OnEnter(GameState::InGame), enemy.spawn())
        .add_systems(Update, attract_enemy_to_center);
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

fn attract_enemy_to_center(mut enemies: Query<(Forces, &Position), With<Enemy>>) {
    for (mut enemy, pos) in enemies.iter_mut() {
        let force_dir = -pos.0;
        let force = force_dir.normalize_or_zero() * 400.;
        enemy.apply_linear_impulse(force);
    }
}
