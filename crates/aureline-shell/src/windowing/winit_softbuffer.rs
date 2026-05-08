//! Winit + softbuffer windowing adapter.
//!
//! This module keeps the native window creation and software-surface wiring in
//! one place so the shell bootstrap can focus on lifecycle, input routing, and
//! state transitions.

use std::sync::Arc;

use softbuffer::{Context, Surface};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub(crate) type SoftbufferSurface = Surface<Arc<Window>, Arc<Window>>;

#[derive(Debug)]
pub(crate) struct WinitSoftbufferWindow {
    window: Arc<Window>,
    surface: SoftbufferSurface,
}

impl WinitSoftbufferWindow {
    pub(crate) fn new(
        event_loop: &EventLoop<()>,
        title: String,
        logical_size: LogicalSize<f64>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let window = Arc::new(
            WindowBuilder::new()
                .with_title(title)
                .with_inner_size(logical_size)
                .build(event_loop)?,
        );

        let context = Context::new(window.clone())?;
        let surface = Surface::new(&context, window.clone())?;
        Ok(Self { window, surface })
    }

    pub(crate) fn into_parts(self) -> (Arc<Window>, SoftbufferSurface) {
        (self.window, self.surface)
    }
}
