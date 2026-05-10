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
/// Title/context bar, status surfaces, and canonical identity tuple wiring.
pub mod chrome;
/// Command review-sheet projections for diagnostics and invocation previews.
pub mod commands;
/// Deep-link entry validator (origin/target/command-class admission contract).
pub mod deeplink;
/// Docs/help browser skeleton with source/version/freshness rows.
pub mod docs_browser;
/// Embedded surface boundary chrome and browser-handoff wiring.
pub mod embedded;
/// Virtualized file-tree model with stable node ids and explorer actions.
pub mod explorer;
/// Help and inspection projections used by shell surfaces.
pub mod help;
pub mod layout;
/// Command palette query-session state and grouped result projections.
pub mod palette;
/// Path-truth chip, alias inspector, and pre-write save-target review projections.
pub mod path_truth;
/// Quick-open query session: recent targets, commands, and lexical results.
pub mod quick_open;
/// Safe-mode entry, recovery-ladder rung stubs, and crash-loop containment offers.
pub mod recovery;
/// Restore-prompt projection for resume / missing-target recovery.
pub mod restore;
/// Save-review sheet projections for conflicted save attempts.
pub mod save_review;
/// Workspace search shell: lexical filename/path surface with scope and partiality truth.
pub mod search_shell;
/// Start Center quick-action surface and entry projections.
pub mod start_center;
/// Shared placeholder/state-card vocabulary used across shell surfaces.
pub mod state_cards;
/// Platform windowing adapters used by the native desktop shell.
pub mod windowing;
/// Workspace switcher projections for recent-work entries and workspace transitions.
pub mod workspace_switcher;
