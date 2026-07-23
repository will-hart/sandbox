use bevy::{dev_tools::states::log_transitions, prelude::*};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}

pub(super) fn plugin(app: &mut App) {
    info!("Loading state plugin");
    app.init_state::<GameState>()
        .add_systems(Update, log_transitions::<GameState>);
}
