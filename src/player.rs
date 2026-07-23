use avian2d::{collision::collider::Collider, dynamics::rigid_body::RigidBody};
use bevy::prelude::*;

use crate::{cursor::MouseData, states::GameState};

pub(super) fn plugin(app: &mut App) {
    info!("Loading player plugin");
    app.init_resource::<PlayerTimers>()
        .add_systems(
            OnEnter(GameState::InGame),
            (player.spawn(), reset_player_timers),
        )
        .add_systems(Update, position_player.run_if(in_state(GameState::InGame)));
}

fn reset_player_timers(mut timers: ResMut<PlayerTimers>) {
    timers.area_reduce.reset();
    timers.enemy_split.reset();
    timers.player_attack.reset();
}

#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct PlayerTimers {
    pub enemy_split: Timer,
    pub area_reduce: Timer,
    pub player_attack: Timer,
}

impl Default for PlayerTimers {
    fn default() -> Self {
        Self {
            enemy_split: Timer::from_seconds(5.0, TimerMode::Repeating),
            area_reduce: Timer::from_seconds(10.0, TimerMode::Repeating),
            player_attack: Timer::from_seconds(8.0, TimerMode::Repeating),
        }
    }
}

#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn player() -> impl Scene {
    let material = ColorMaterial::from_color(Srgba::new(1.5, 0.1, 0.1, 1.0));
    let rigid_body = RigidBody::Dynamic;
    let collider = Collider::polyline(
        vec![
            Vec2::new(-20.0, 45.0),
            Vec2::new(0.0, 48.0),
            Vec2::new(20.0, 45.0),
        ],
        None,
    );

    bsn! {
        Player
        DespawnOnExit<GameState>(GameState::InGame)
        Mesh2d(asset_value(Ring::new(
            CircularSector::new(50.0, 0.4),
            CircularSector::new(46.0, 0.4),
        )))
        MeshMaterial2d<ColorMaterial>(asset_value(material))
        Children [
            (
                template_value(rigid_body)
                template_value(collider)
            )
        ]
    }
}

fn position_player(mouse: Res<MouseData>, mut player: Single<&mut Transform, With<Player>>) {
    let pos = mouse.relative_mouse_pos().normalize_or_zero();
    let rot = Quat::from_rotation_z((-pos.y).atan2(pos.x) - std::f32::consts::FRAC_PI_2);
    player.rotation = rot;
}
