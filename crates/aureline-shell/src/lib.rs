//! Desktop shell: canonical zones, layout, and live frame wiring.
//!
//! This crate is the production shell container. It defines the canonical
//! shell-zone ids, default metrics, and a small live desktop frame that renders
//! placeholder occupants in each declared zone.

#![doc(html_root_url = "https://docs.rs/aureline-shell/0.0.0")]

/// Accessibility-tree bridge groundwork for shell surfaces.
pub(crate) mod a11y;
pub mod app_frame;
/// Native desktop shell bootstrap (window creation, event loop, input dispatch).
pub mod bootstrap;
/// Command review-sheet projections for diagnostics and invocation previews.
pub mod commands;
/// Embedded surface boundary chrome and browser-handoff wiring.
pub mod embedded;
/// Help and inspection projections used by shell surfaces.
pub mod help;
pub mod layout;
/// Command palette query-session state and grouped result projections.
pub mod palette;
/// Start Center quick-action surface and entry projections.
pub mod start_center;
/// Platform windowing adapters used by the native desktop shell.
pub mod windowing;
/// Workspace switcher projections for recent-work entries and workspace transitions.
pub mod workspace_switcher;
