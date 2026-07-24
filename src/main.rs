use avian2d::{PhysicsPlugins, dynamics::integrator::Gravity};
use bevy::{
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    feathers::{FeathersPlugins, dark_theme::create_dark_theme, palette::BLACK, theme::UiTheme},
    post_process::bloom::Bloom,
    prelude::*,
};

#[cfg(debug_assertions)]
use avian2d::debug_render::PhysicsDebugPlugin;

mod container;
mod cursor;
mod enemy;
mod mutations;
mod player;
mod states;
mod ui;

fn main() {
    let mut app = App::new();
    app.insert_resource(UiTheme(create_dark_theme()))
        .insert_resource(ClearColor(BLACK.into()))
        .add_plugins((DefaultPlugins, FeathersPlugins))
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::ZERO))
        .add_plugins((
            container::plugin,
            cursor::plugin,
            enemy::plugin,
            mutations::plugin,
            states::plugin,
            ui::plugin,
            player::plugin,
        ))
        .add_systems(Startup, spawn_camera);

    #[cfg(debug_assertions)]
    app.add_plugins(PhysicsDebugPlugin);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let mut bloom = Bloom::default();
    bloom.intensity = 0.4;

    // spawn main camera
    commands.spawn((
        Camera2d,
        IsDefaultUiCamera,
        Tonemapping::AcesFitted,
        bloom,
        DebandDither::Enabled,
    ));
}
