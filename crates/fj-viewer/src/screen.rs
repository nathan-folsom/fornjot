//! Types that describe aspects of the screen

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

/// Needs to be implemented by types that can serve as a screen to render to
pub trait Screen {
    /// The window
    type Window: HasRawDisplayHandle + HasRawWindowHandle;

    /// Access the size of the screen
    fn size(&self) -> ScreenSize;

    /// Access the window
    fn window(&self) -> &Self::Window;
}

/// Cursor position in normalized coordinates (-1 to +1)
///
/// The center of the screen is at (0, 0). The aspect ratio is taken into
/// account.
#[derive(Clone, Copy, Debug)]
pub struct NormalizedScreenPosition {
    /// The x coordinate of the position [-1, 1]
    pub x: f64,

    /// The y coordinate of the position [-1, 1]
    pub y: f64,
}

/// The size of the screen
#[derive(Clone, Copy, Debug)]
pub struct ScreenSize {
    /// The width of the screen
    pub width: u32,

    /// The height of the screen
    pub height: u32,
}

impl ScreenSize {
    /// Convert size to `f64`
    pub fn as_f64(&self) -> [f64; 2] {
        [self.width, self.height].map(Into::into)
    }
}
