use avian2d::{
    collision::{collider::Collider, collision_events::CollisionEventsEnabled},
    dynamics::rigid_body::{
        AngularDamping, CoefficientCombine, LinearVelocity, Restitution, RigidBody,
    },
};
use bevy::prelude::*;

use crate::{
    enemy::mutated::{EnemyMutation, EnemyType},
    states::GameState,
};

pub const DEFAULT_ENEMY_SPEED: f32 = 350.0;
pub const DEFAULT_SHOOT_TIME: f32 = 5.0;

pub(super) fn plugin(app: &mut App) {
    info!("Loading enemy plugin");
    app.add_plugins(mutated::plugin);

    app.add_systems(OnEnter(GameState::InGame), enemy.spawn()) // spawn first enemy
        .add_systems(
            Update,
            normalise_enemy_velocity_by_speed.run_if(in_state(GameState::InGame)),
        );
}

#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
#[reflect(Component)]
pub struct EnemySpeed(pub f32);

#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
#[reflect(Component)]
pub struct EnemySplits(u8);

#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
#[reflect(Component)]
pub struct EnemyShoots {
    period: f32,
    time_left: f32,
}

impl EnemyShoots {
    fn new(period: f32) -> Self {
        Self {
            period,
            time_left: period,
        }
    }
}

fn enemy() -> impl Scene {
    bsn! {
        Enemy
    }
}

#[derive(Debug, Default)]
pub struct EnemyProps {
    mutations: Vec<EnemyMutation>,
    enemy_type: EnemyType,
}

#[derive(Debug, Clone, Copy, Default, SceneComponent, Reflect)]
#[scene(EnemyProps)]
pub struct Enemy;

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub struct HoverEnemy {
    radius: f32,
    theta: f32,
}

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub struct BouncerEnemy;

fn bouncer() -> impl Scene {
    bsn! {
        BouncerEnemy
        LinearVelocity(Vec2::new(150.0, -100.0))
        Transform {
            translation: Vec2 {
                x: 0.0,
                y: 80.0,
            }
        }
    }
}

fn hover_attacker() -> impl Scene {
    let theta = fastrand::f32() * std::f32::consts::PI * 2.0;
    let radius = 250.0;
    let translation = (Vec2::from_angle(theta) * radius).extend(0.0);

    bsn! {
        HoverEnemy {
            radius,
            theta
        }
        Transform {
            translation
        }
    }
}

impl Enemy {
    pub fn scene(props: EnemyProps) -> impl Scene {
        let color = Srgba::new(1.7, 1.0, 1.8, 1.0);
        let enemy_radius = 15.0;

        let (speed, splits, shoots) = props.mutations.iter().fold(
            (DEFAULT_ENEMY_SPEED, 0, 0.),
            |(speed, splits, shoots), mutation| match mutation {
                EnemyMutation::Fast => (speed + DEFAULT_ENEMY_SPEED, splits, shoots),
                EnemyMutation::Splits => (speed, splits + 1, shoots),
                EnemyMutation::Shoots => (speed, splits, shoots + 1.),
            },
        );

        let enemy_type: Box<dyn Scene> = match props.enemy_type {
            EnemyType::HoverAttacker => Box::new(hover_attacker()),
            EnemyType::Bouncer => Box::new(bouncer()),
        };

        let splits: Box<dyn Scene> = if splits > 0 {
            Box::new(bsn! { EnemySplits(splits) })
        } else {
            Box::new(bsn! {})
        };

        let shoots: Box<dyn Scene> = if shoots > 0. {
            Box::new(bsn! { EnemyShoots::new(DEFAULT_SHOOT_TIME / shoots) })
        } else {
            Box::new(bsn! {})
        };

        bsn! {
            Enemy
            EnemySpeed(speed)
            enemy_type
            splits
            shoots
            DespawnOnExit<GameState>(GameState::InGame)
            Mesh2d(asset_value(Rectangle::new(enemy_radius, enemy_radius)))
            MeshMaterial2d<ColorMaterial>(asset_value(ColorMaterial::from_color(color)))
            template_value(RigidBody::Dynamic)
            Collider::round_rectangle(enemy_radius, enemy_radius, 3.0)
            AngularDamping(1.9)
            CollisionEventsEnabled
            Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombine::Max
            }
        }
    }
}

fn normalise_enemy_velocity_by_speed(mut enemies: Query<(&mut LinearVelocity, &EnemySpeed)>) {
    for (mut linvel, speed) in &mut enemies {
        linvel.0 = linvel.0.normalize_or_zero() * speed.0;
    }
}

mod mutated {
    use bevy::prelude::*;

    use crate::states::GameState;

    pub(super) fn plugin(app: &mut App) {
        info!("Loading enemy::mutated plugin");

        app.add_systems(OnEnter(GameState::InGame), insert_new_mutation_state);
    }

    fn insert_new_mutation_state(mut commands: Commands) {
        commands.insert_resource(CurrentMutation::default());
    }

    #[derive(Debug, Default, Clone, Resource, Reflect)]
    #[reflect(Resource)]
    pub struct CurrentMutation {
        mutations: Vec<EnemyMutation>,
        enemy_type: EnemyType,
    }

    impl CurrentMutation {}

    #[derive(Debug, Clone, Default, Component, Reflect)]
    #[reflect(Component)]
    pub enum EnemyType {
        HoverAttacker,
        #[default]
        Bouncer,
    }

    #[derive(Debug, Clone, Copy, Reflect)]
    pub enum EnemyMutation {
        Fast,
        Splits,
        Shoots,
    }
}
