use avian2d::{
    collision::collider::Collider,
    dynamics::rigid_body::{CoefficientCombine, LinearVelocity, Restitution, RigidBody},
};
use bevy::prelude::*;

use crate::states::GameState;

pub(super) fn plugin(app: &mut App) {
    info!("Loading enemy plugin");
    app.add_systems(OnEnter(GameState::InGame), enemy.spawn());
}

fn enemy() -> impl Scene {
    let color = Srgba::new(1.7, 1.0, 1.8, 1.0);
    let enemy_radius = 15.0;

    bsn! {
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
