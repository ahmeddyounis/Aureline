//! Winit window creation helper.
//!
//! This module owns native window creation so backend selection (software
//! surface vs GPU surface) can happen above this layer without duplicating
//! window-builder details.

use std::sync::Arc;

use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

#[derive(Debug)]
pub(crate) struct WinitWindow {
    window: Arc<Window>,
}

impl WinitWindow {
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
        Ok(Self { window })
    }

    pub(crate) fn into_arc(self) -> Arc<Window> {
        self.window
    }
}
