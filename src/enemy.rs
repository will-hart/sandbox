use avian2d::{
    collision::collider::Collider,
    dynamics::rigid_body::{
        AngularDamping, CoefficientCombine, LinearVelocity, Restitution, RigidBody,
    },
};
use bevy::prelude::*;

use crate::states::GameState;

pub const DEFAULT_ENEMY_SPEED: f32 = 350.0;

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
        Self(DEFAULT_ENEMY_SPEED)
    }
}

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Debug, Clone, Copy, Component, Reflect)]
#[reflect(Component)]
pub struct SplitEnemy {
    pub time_remaining: f32,
    pub parent: Entity,
}

impl Default for SplitEnemy {
    fn default() -> Self {
        Self {
            time_remaining: 5.0,
            parent: Entity::PLACEHOLDER,
        }
    }
}

fn enemy() -> impl Scene {
    let color = Srgba::new(1.7, 1.0, 1.8, 1.0);
    let enemy_radius = 15.0;

    bsn! {
        Enemy
        DespawnOnExit<GameState>(GameState::InGame)
        Mesh2d(asset_value(Rectangle::new(enemy_radius, enemy_radius)))
        MeshMaterial2d<ColorMaterial>(asset_value(ColorMaterial::from_color(color)))
        template_value(RigidBody::Dynamic)
        Collider::round_rectangle(enemy_radius, enemy_radius, 3.0)
        AngularDamping(1.9)
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

pub fn split_enemy(parent: Entity, translation: Vec3) -> impl Scene {
    let color = Srgba::new(1.5, 1.2, 1.8, 1.0);
    let enemy_radius = 10.0;

    bsn! {
        DespawnOnExit<GameState>(GameState::InGame)
        SplitEnemy {
            parent
        }
        Mesh2d(asset_value(Rectangle::new(enemy_radius, enemy_radius)))
        MeshMaterial2d<ColorMaterial>(asset_value(ColorMaterial::from_color(color)))
        template_value(RigidBody::Dynamic)
        Collider::round_rectangle(enemy_radius, enemy_radius, 3.0)
        AngularDamping(1.9)
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombine::Max
        }
        LinearVelocity(Vec2::new(150.0, -100.0))
        Transform {
            translation: {translation + Vec3::new(30.0, 30.0, 0.0)}
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
