use fj_interop::mesh::Mesh;
use fj_math::Point;

use super::{
    event::KeyState, movement::Movement, rotation::Rotation, zoom::Zoom, Event,
    Key,
};
use crate::{
    camera::Camera,
    screen::{Position, Size},
};

/// Input handling abstraction
///
/// Takes user input and applies them to application state.
pub struct Handler {
    cursor: Option<Position>,

    movement: Movement,
    rotation: Rotation,
    zoom: Zoom,
}

impl Handler {
    /// Returns the state of the cursor position.
    pub fn cursor(&self) -> Option<Position> {
        self.cursor
    }

    /// Handle an input event
    pub fn handle_event(
        &mut self,
        event: Event,
        screen_size: Size,
        mesh: &Mesh<Point<3>>,
        camera: &mut Camera,
        actions: &mut Actions,
    ) {
        match event {
            Event::CursorMoved(position) => {
                if let Some(previous) = self.cursor {
                    let diff_x = position.x - previous.x;
                    let diff_y = position.y - previous.y;

                    self.movement.apply(self.cursor, camera, screen_size);
                    self.rotation.apply(diff_x, diff_y, camera);
                }

                self.cursor = Some(position);
            }
            Event::Key(Key::Escape, KeyState::Pressed) => actions.exit = true,

            Event::Key(Key::Key1, KeyState::Pressed) => {
                actions.toggle_model = true
            }
            Event::Key(Key::Key2, KeyState::Pressed) => {
                actions.toggle_mesh = true
            }
            Event::Key(Key::Key3, KeyState::Pressed) => {
                actions.toggle_debug = true
            }

            Event::Key(Key::MouseLeft, KeyState::Pressed) => {
                let focus_point =
                    camera.focus_point(screen_size, self.cursor(), mesh);

                self.rotation.start(focus_point);
            }
            Event::Key(Key::MouseLeft, KeyState::Released) => {
                self.rotation.stop();
            }
            Event::Key(Key::MouseRight, KeyState::Pressed) => {
                let focus_point =
                    camera.focus_point(screen_size, self.cursor(), mesh);

                self.movement.start(focus_point, self.cursor);
            }
            Event::Key(Key::MouseRight, KeyState::Released) => {
                self.movement.stop();
            }

            Event::Scroll(delta) => {
                self.zoom.push(delta);
            }

            _ => {}
        }
    }

    /// Update application state from user input.
    pub fn update(
        &mut self,
        delta_t: f64,
        camera: &mut Camera,
        screen_size: Size,
        mesh: &Mesh<Point<3>>,
    ) {
        let focus_point = camera.focus_point(screen_size, self.cursor(), mesh);
        self.zoom.apply_to_camera(delta_t, focus_point, camera);
    }
}

impl Default for Handler {
    fn default() -> Self {
        Self {
            cursor: None,

            movement: Movement::new(),
            rotation: Rotation::new(),
            zoom: Zoom::new(),
        }
    }
}

/// Intermediate input state container
///
/// Used as a per frame state container for sending application state to `winit`.
#[derive(Default)]
pub struct Actions {
    /// Application exit state.
    pub exit: bool,

    /// Toggle for the shaded display of the model.
    pub toggle_model: bool,
    /// Toggle for the model's wireframe.
    pub toggle_mesh: bool,
    /// Toggle for debug information.
    pub toggle_debug: bool,
}

impl Actions {
    /// Returns a new `Actions`.
    pub fn new() -> Self {
        Self::default()
    }
}
