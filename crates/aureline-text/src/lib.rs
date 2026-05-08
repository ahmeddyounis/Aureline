//! Foundational text primitives shared by the buffer core and the renderer.
//!
//! Hosts encoding handling, grapheme/word/line segmentation, and the inputs to
//! shaping. This crate sits at the bottom of the text stack and must not pull
//! in higher-level concerns.
//!
//! The [`prototype`] module hosts the pure-Rust stub shaper, fallback chain,
//! and glyph-cache layout that validate the ADR 0002 contract ahead of the
//! production text stack. Its API is the contract future shaping and
//! rasterisation work must honour; the implementation is a stub.
//!
//! The [`shaping`] module owns the production shaping pipeline contract used by
//! the desktop shell and editor surfaces.

#![doc(html_root_url = "https://docs.rs/aureline-text/0.0.0")]

pub mod prototype;
pub mod shaping;
