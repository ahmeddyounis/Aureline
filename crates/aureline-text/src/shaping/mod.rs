//! Production text shaping and font fallback.
//!
//! This module owns the canonical shaping pipeline contract used by the
//! desktop shell and editor surfaces. It is responsible for:
//!
//! - Grapheme-aware segmentation and cursor-safe cluster boundaries (UAX #29).
//! - Bidirectional itemization into visual runs (UAX #9).
//! - Deterministic font fallback selection per cluster (ADR 0002).
//! - A stable metrics vocabulary that callers can surface for diagnostics.
//!
//! The renderer consumes the output of this module to populate the glyph atlas
//! and to position glyphs for painting. Higher layers must not re-segment or
//! invent their own fallback decisions.

pub mod fonts;
pub mod shaper;
pub mod types;

pub use fonts::{FontFallbackConfig, FontSystem, GenericFamily};
pub use shaper::{ShapedGlyph, ShapedLine, ShaperMetrics, TextShaper};
pub use types::{FallbackStage, FeatureSet, ShaperPolicy, TextDirection};
