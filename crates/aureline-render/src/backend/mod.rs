//! Renderer backend implementations.
//!
//! Backends own GPU resource initialization, surface configuration, and frame
//! submission. Higher layers provide draw-queue state and rasterized content.

mod wgpu_blit;

pub use wgpu_blit::WgpuBlitRenderer;
