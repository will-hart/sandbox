use bevy::{prelude::*, window::WindowResized};

pub(super) fn plugin(app: &mut App) {
    info!("Loading cursor plugin");

    app.init_resource::<MouseData>()
        .add_systems(Update, (handle_window_resize, update_mouse_pos));
}

#[derive(Debug, Clone, Copy, Resource, Reflect)]
#[reflect(Resource)]
pub struct MouseData {
    pub mouse_pos: Option<Vec2>,
    pub window_size: Vec2,
}

impl MouseData {
    /// returns relative mouse position compared to the center of the screen
    pub fn relative_mouse_pos(&self) -> Vec2 {
        let Some(pos) = self.mouse_pos else {
            return Vec2::ZERO;
        };

        self.relative_pos(pos)
    }

    /// Gets the position of the given position relative to the window center
    pub fn relative_pos(&self, v: Vec2) -> Vec2 {
        v - self.half_window()
    }

    #[inline(always)]
    pub fn half_window(&self) -> Vec2 {
        self.window_size / 2.0
    }
}

impl FromWorld for MouseData {
    fn from_world(world: &mut World) -> Self {
        let mut query = world.query::<&Window>();
        let window = query.single(world).expect("should find a window");

        let size = Vec2::new(window.width(), window.height());
        info!("Initial window size: {} x {}", size.x, size.y);

        Self {
            window_size: size,
            mouse_pos: None,
        }
    }
}

pub fn update_mouse_pos(
    mut cursor_evr: MessageReader<CursorMoved>,
    mut mouse_pos: ResMut<MouseData>,
) {
    for ev in cursor_evr.read() {
        mouse_pos.mouse_pos = Some(ev.position);
        // debug!("{:?}", mouse_pos.relative_mouse_pos().normalize());
    }
}

fn handle_window_resize(
    mut messages: MessageReader<WindowResized>,
    mut mouse_data: ResMut<MouseData>,
) {
    for message in messages.read() {
        mouse_data.window_size = Vec2::new(message.width, message.height);
        info!("Window resized: {:?}", mouse_data.window_size);
    }
}
