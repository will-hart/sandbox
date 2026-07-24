use avian2d::{
    collision::{
        collider::Collider,
        collision_events::{CollisionEventsEnabled, CollisionStart},
    },
    dynamics::rigid_body::{LockedAxes, RigidBody},
};
use bevy::{color::palettes::css::BLACK, prelude::*};

use crate::{
    cursor::MouseData,
    enemy::{Enemy, SplitEnemy},
    states::GameState,
};

pub(super) fn plugin(app: &mut App) {
    info!("Loading player plugin");
    app.add_systems(OnEnter(GameState::InGame), player.spawn())
        .add_systems(
            Update,
            (position_player, update_player_health_count_text).run_if(in_state(GameState::InGame)),
        );
}

#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Debug, Clone, Copy, Component, Reflect)]
#[reflect(Component)]
pub struct PlayerHealthCountdown(pub u8);

#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerHealthCountText;

impl Default for PlayerHealthCountdown {
    fn default() -> Self {
        Self(6)
    }
}

fn player() -> impl Scene {
    let material = ColorMaterial::from_color(Srgba::new(1.9, 1.1, 1.1, 1.0));

    bsn! {
        Player
        DespawnOnExit<GameState>(GameState::InGame)
        Mesh2d(asset_value(Ring::new(
            CircularSector::new(50.0, 0.4),
            CircularSector::new(46.0, 0.4),
        )))
        MeshMaterial2d<ColorMaterial>(asset_value(material.clone()))
        template_value(RigidBody::Dynamic)
        Collider::polyline(
            vec![
                Vec2::new(-20.0, 45.0),
                Vec2::new(0.0, 48.0),
                Vec2::new(20.0, 45.0),
            ],
            None,
        )
        template_value(LockedAxes::ALL_LOCKED)
        Children [
            (
                PlayerHealthCountdown
                CollisionEventsEnabled
                Mesh2d(asset_value(Circle::new(25.0)))
                MeshMaterial2d<ColorMaterial>(asset_value(material))
                Collider::circle(25.0)
                template_value(RigidBody::Static)
                on(collide_enemies_and_player)
                Children [
                    (
                        Transform {
                            translation: Vec3 {
                                z: 0.1
                            }
                        }
                        PlayerHealthCountText
                        Text2d("")
                        TextColor(BLACK)
                    )
                ]
            )
        ]
    }
}

fn position_player(mouse: Res<MouseData>, mut player: Single<&mut Transform, With<Player>>) {
    let pos = mouse.relative_mouse_pos().normalize_or_zero();
    let rot = Quat::from_rotation_z((-pos.y).atan2(pos.x) - std::f32::consts::FRAC_PI_2);
    player.rotation = rot;
}

fn collide_enemies_and_player(
    trigger: On<CollisionStart>,
    mut player_health: Single<&mut PlayerHealthCountdown>,
    enemies: Query<Entity, Or<(With<Enemy>, With<SplitEnemy>)>>,
) {
    if enemies.contains(trigger.collider2) {
        player_health.0 = player_health.0.saturating_sub(1);
        if player_health.0 == 0 {
            warn!("Player dead!");
        }
    }
}

fn update_player_health_count_text(
    player_health: Single<&mut PlayerHealthCountdown>,
    mut text: Single<&mut Text2d, With<PlayerHealthCountText>>,
) {
    text.0 = format!("{}", player_health.0);
}
