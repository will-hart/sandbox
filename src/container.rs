use avian2d::{collision::collider::Collider, dynamics::rigid_body::RigidBody};
use bevy::prelude::*;

use crate::{mutations::RotatingContainer, states::GameState};

pub(super) fn plugin(app: &mut App) {
    info!("Loading container plugin");

    app.add_systems(OnEnter(GameState::InGame), container.spawn());
}

#[derive(Debug, Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Container;

fn container() -> impl Scene {
    let (container_w, container_h) = (500.0, 500.0);
    let thickness = 3.0;
    let color = Srgba::new(1.1, 1.3, 2.0, 1.0);

    bsn! {
        Container
        RotatingContainer
        DespawnOnExit<GameState>(GameState::InGame)
        template_value(RigidBody::Static)
        Mesh2d(asset_value(Rectangle::new(container_w, container_h).to_ring(thickness)))
        MeshMaterial2d<ColorMaterial>(asset_value(ColorMaterial::from_color(color)))
        Collider::polyline(
            vec![
                Vec2::new(-container_w / 2.0, -container_h / 2.0),
                Vec2::new(container_w / 2.0, -container_h / 2.0),
                Vec2::new(container_w / 2.0, container_h / 2.0),
                Vec2::new(-container_w / 2.0, container_h / 2.0),
                Vec2::new(-container_w / 2.0, -container_h / 2.0),
            ],
            None
        )
    }
}
