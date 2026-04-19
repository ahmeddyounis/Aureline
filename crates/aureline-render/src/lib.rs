//! GPU-accelerated rendering primitives for the desktop shell.
//!
//! Owns frame composition, scene graph, and the rendering pipeline used by the
//! shell. Higher layers should treat this crate as the only path that touches
//! the GPU.

#![doc(html_root_url = "https://docs.rs/aureline-render/0.0.0")]
