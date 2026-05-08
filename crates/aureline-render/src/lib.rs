//! GPU-accelerated rendering primitives for the desktop shell.
//!
//! Owns frame composition, scene graph, and the rendering pipeline used by the
//! shell. Higher layers should treat this crate as the only path that touches
//! the GPU.

#![doc(html_root_url = "https://docs.rs/aureline-render/0.0.0")]

/// Renderer backend implementations.
pub mod backend;
/// Draw-queue vocabulary shared between the shell and renderer.
pub mod draw_queue;
/// Frame scheduling and trace-facing timing marks.
pub mod frame_scheduler;
/// Trace hook vocabulary used by the renderer hot path.
pub mod hooks;

pub use backend::WgpuBlitRenderer;
pub use draw_queue::{CompositedFrame, CompositionLayerId, DamageClassId, DamageEvent, DrawQueue};
pub use frame_scheduler::{FrameScheduler, FrameSchedulerDecision, FrameSchedulerStats};
pub use hooks::{Clock, Hook, Tick, TimingMark, TimingRecorder, WallClock};
