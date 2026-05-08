//! Winit + softbuffer windowing adapter.
//!
//! This module keeps the native window creation and software-surface wiring in
//! one place so the shell bootstrap can focus on lifecycle, input routing, and
//! state transitions.

use std::sync::Arc;

use softbuffer::{Context, Surface};
use winit::window::Window;

pub(crate) type SoftbufferSurface = Surface<Arc<Window>, Arc<Window>>;

pub(crate) fn create_softbuffer_surface(
    window: Arc<Window>,
) -> Result<SoftbufferSurface, Box<dyn std::error::Error>> {
    let context = Context::new(window.clone())?;
    let surface = Surface::new(&context, window.clone())?;
    Ok(surface)
}
