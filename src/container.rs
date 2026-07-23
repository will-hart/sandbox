use avian2d::{collision::collider::Collider, dynamics::rigid_body::RigidBody};
use bevy::prelude::*;

use crate::states::GameState;

pub(super) fn plugin(app: &mut App) {
    info!("Loading container plugin");

    app.add_systems(OnEnter(GameState::InGame), container.spawn());
}

fn container() -> impl Scene {
    let (container_w, container_h) = (500.0, 500.0);
    let thickness = 3.0;
    let color = Srgba::new(1.1, 1.3, 2.0, 1.0);

    bsn! {
        DespawnOnExit<GameState>(GameState::InGame)
        Mesh2d(asset_value(Rectangle::new(container_w, container_h).to_ring(thickness)))
        MeshMaterial2d<ColorMaterial>(asset_value(ColorMaterial::from_color(color)))
        template_value(RigidBody::Static)
        template_value(Collider::rectangle(container_w + thickness / 2.0, container_h + thickness / 2.0))
        Children [
            (
                Mesh2d(asset_value(Circle::new(30.0)))
                MeshMaterial2d<ColorMaterial>(asset_value(ColorMaterial::from_color(color)))
            )
        ]
    }
}
