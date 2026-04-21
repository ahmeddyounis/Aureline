//! Desktop shell spike: end-to-end seams for the window, input, and render paths.
//!
//! This crate is an integration spike. It proves the direction chosen in ADR
//! 0002 (`docs/adr/0002-renderer-text-stack-and-shaping-fallback.md`) is
//! viable, measurable, and boundary-clean. Nothing here carries a stability
//! contract.
//!
//! # Module seams
//!
//! - [`hooks`] — the canonical hook-name vocabulary lifted verbatim from
//!   ADR 0002 §Protected-hot-path hook list. Any measurement or span that
//!   wants to talk about a hot path MUST use a name from this vocabulary;
//!   inventing synonyms is an ADR violation.
//! - [`zones`] — the composited shell frame layout. Title bar, sidebar
//!   placeholder, editor viewport placeholder, and status bar placeholder
//!   are declared here with a deterministic z-order and a single
//!   invalidation entry point.
//! - [`input_path`] — the input seam. Accepts keyboard and mouse events,
//!   routes them into deterministic input actions, and exposes the
//!   entry points the benchmark lab can wrap for latency traces.
//! - [`render_path`] — the render seam. Owns the text-and-decoration layer
//!   vs. overlay-layer boundary, the dirty-rect compositor entry, the
//!   placeholder surface ownership, and the frame submit hook.
//! - [`frame_timing`] — named frame-timing marks emitted at the hot-path
//!   entry points. The recorder is injectable so the binary runs
//!   deterministically regardless of wall clock.
//! - [`fixture_scene`] — a single repeatable scene used by the binary and
//!   by tests to drive input and render paths through a known sequence,
//!   producing the trace samples under `artifacts/render/`.
//! - [`capabilities`] — machine-readable capability manifest emitted at
//!   startup. Later benchmark and conformance tasks consume it by name.
//! - [`trace`] — trace-record format and JSON emission shared between the
//!   binary and the fixture-scene tests.
//! - [`text_layer`] — side-channel integration of the prototype text
//!   stack. The default fixture scene does not route through here so
//!   the committed `artifacts/render/spike_trace_samples/` stay
//!   byte-stable; the binary's `--text-stack-smoke` mode runs the
//!   committed shaping corpus through it.
//!
//! # Intentional non-goals
//!
//! - No editor features, no panel system, no themeable chrome.
//! - No networked subsystems, no RPC, no settings.
//! - No claim of hot-path parity. Budgets are the benchmark lab's job.

#![doc(html_root_url = "https://docs.rs/aureline-shell-spike/0.0.0")]

pub mod capabilities;
pub mod fixture_scene;
pub mod frame_timing;
pub mod hooks;
pub mod input_path;
pub mod render_path;
pub mod text_layer;
pub mod trace;
pub mod zones;

/// Spike build identity. Emitted into the capabilities manifest so later
/// clean-room or provenance tasks can diff two nearby spike builds.
///
/// Kept intentionally small: the full build identity lives in the repo's
/// reproducible-build baseline (`docs/build/reproducible_build_baseline.md`).
/// This struct is the minimum the spike needs to self-report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpikeBuildIdentity {
    pub crate_name: &'static str,
    pub crate_version: &'static str,
    pub rustc_target_triple: &'static str,
}

impl SpikeBuildIdentity {
    pub const fn current() -> Self {
        Self {
            crate_name: env!("CARGO_PKG_NAME"),
            crate_version: env!("CARGO_PKG_VERSION"),
            rustc_target_triple: TARGET_TRIPLE,
        }
    }
}

#[cfg(target_os = "macos")]
const TARGET_TRIPLE: &str = "apple-darwin";
#[cfg(target_os = "linux")]
const TARGET_TRIPLE: &str = "unknown-linux-gnu";
#[cfg(target_os = "windows")]
const TARGET_TRIPLE: &str = "pc-windows-msvc";
#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
const TARGET_TRIPLE: &str = "unknown";
