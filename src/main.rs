use bevy::{
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    feathers::{FeathersPlugins, dark_theme::create_dark_theme, palette::BLACK, theme::UiTheme},
    post_process::bloom::Bloom,
    prelude::*,
};

mod cursor;
mod player;
mod states;
mod ui;

fn main() {
    App::new()
        .insert_resource(UiTheme(create_dark_theme()))
        .insert_resource(ClearColor(BLACK.into()))
        .add_plugins((DefaultPlugins, FeathersPlugins))
        .add_plugins((cursor::plugin, states::plugin, ui::plugin, player::plugin))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut bloom = Bloom::default();
    bloom.intensity = 0.3;

    // spawn main camera
    commands.spawn((
        Camera2d,
        IsDefaultUiCamera,
        Tonemapping::AcesFitted,
        bloom,
        DebandDither::Enabled,
    ));
}
