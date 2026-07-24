use avian2d::{collision::collider::Collider, dynamics::rigid_body::RigidBody};
use bevy::prelude::*;

use crate::{cursor::MouseData, states::GameState};

pub(super) fn plugin(app: &mut App) {
    info!("Loading player plugin");
    app.add_systems(OnEnter(GameState::InGame), player.spawn())
        .add_systems(Update, position_player.run_if(in_state(GameState::InGame)));
}

#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Debug, Clone, Copy, Component, Reflect)]
#[reflect(Component)]
pub struct PlayerHealthCountdown(pub u8);

impl Default for PlayerHealthCountdown {
    fn default() -> Self {
        Self(6)
    }
}

fn player() -> impl Scene {
    let material = ColorMaterial::from_color(Srgba::new(1.9, 1.1, 1.1, 1.0));

    bsn! {
        Player
        PlayerHealthCountdown
        DespawnOnExit<GameState>(GameState::InGame)
        Mesh2d(asset_value(Ring::new(
            CircularSector::new(50.0, 0.4),
            CircularSector::new(46.0, 0.4),
        )))
        MeshMaterial2d<ColorMaterial>(asset_value(material))
        template_value(RigidBody::Dynamic)
        Collider::polyline(
            vec![
                Vec2::new(-20.0, 45.0),
                Vec2::new(0.0, 48.0),
                Vec2::new(20.0, 45.0),
            ],
            None,
        )
    }
}

fn position_player(mouse: Res<MouseData>, mut player: Single<&mut Transform, With<Player>>) {
    let pos = mouse.relative_mouse_pos().normalize_or_zero();
    let rot = Quat::from_rotation_z((-pos.y).atan2(pos.x) - std::f32::consts::FRAC_PI_2);
    player.rotation = rot;
}
