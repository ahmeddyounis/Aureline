//! Foundational text primitives shared by the buffer core and the renderer.
//!
//! Hosts encoding handling, grapheme/word/line segmentation, and the inputs to
//! shaping. This crate sits at the bottom of the text stack and must not pull
//! in higher-level concerns.

#![doc(html_root_url = "https://docs.rs/aureline-text/0.0.0")]
