//! Desktop shell: canonical zones, layout, and live frame wiring.
//!
//! This crate is the production shell container. It defines the canonical
//! shell-zone ids, default metrics, and a small live desktop frame that renders
//! placeholder occupants in each declared zone.

#![doc(html_root_url = "https://docs.rs/aureline-shell/0.0.0")]

pub mod app_frame;
/// Command review-sheet projections for diagnostics and invocation previews.
pub mod commands;
/// Help and inspection projections used by shell surfaces.
pub mod help;
pub mod layout;
/// Command palette query-session state and grouped result projections.
pub mod palette;
