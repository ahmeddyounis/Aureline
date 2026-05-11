//! Desktop shell: canonical zones, layout, and live frame wiring.
//!
//! This crate is the production shell container. It defines the canonical
//! shell-zone ids, default metrics, and a small live desktop frame that renders
//! placeholder occupants in each declared zone.

#![doc(html_root_url = "https://docs.rs/aureline-shell/0.0.0")]

/// Accessibility-tree bridge groundwork for shell surfaces.
pub(crate) mod a11y;
/// Durable activity-center / job-row seed: rows projected from the typed
/// notification envelope plus a per-lifecycle observation, with file-backed
/// persistence so completed and failed rows survive a process restart.
pub mod activity_center;
/// AI composer / context-inspector projection over the bounded
/// [`aureline_ai`] composer seed. Read-only for mutation; no model dispatch.
pub mod ai_context_inspector;
pub mod app_frame;
/// Shared badge projections (target/origin chips, boundary cues) consumed by
/// terminal, task, debug-prep, and provider/auth entry points.
pub mod badges;
/// Native desktop shell bootstrap (window creation, event loop, input dispatch).
pub mod bootstrap;
/// Path-ancestry breadcrumbs for editor chrome.
pub mod breadcrumbs;
/// Title/context bar, status surfaces, and canonical identity tuple wiring.
pub mod chrome;
/// Command review-sheet projections for diagnostics and invocation previews.
pub mod commands;
/// Debug-prep seed surface: thin projection over the shared execution-context
/// object reused by the terminal pane and task seed.
pub mod debug_seed;
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
/// Help / About / provenance / service-health seed surface with client-scope
/// badges projected from the shared build-info, runtime, and docs/help truth.
pub mod help_about;
pub mod layout;
/// Notification routing: toast, banner, status, and durable-activity row
/// projections derived from the typed notification envelope contract.
pub mod notifications;
/// Command palette query-session state and grouped result projections.
pub mod palette;
/// Path-truth chip, alias inspector, and pre-write save-target review projections.
pub mod path_truth;
/// Quick-open query session: recent targets, commands, and lexical results.
pub mod quick_open;
/// Safe-mode entry, recovery-ladder rung stubs, and crash-loop containment offers.
pub mod recovery;
/// Release-center / provenance seed surface: thin projection over the
/// shared build-info and support-bundle truth that links the running
/// build's exact-build identity to the live support/export preview.
pub mod release_center;
/// Restore-prompt projection for resume / missing-target recovery.
pub mod restore;
/// Preview/apply/revert lifecycle wedge for one destructive core path
/// (multi-target bulk replace). Bounded prototype: mints named undo
/// groups and content-addressed checkpoints, refuses to widen scope after
/// preview, and reuses the shared mutation-journal / local-history
/// vocabulary without forking.
pub mod review_preview;
/// Runtime projections shared by terminal, task, and debug-prep seed surfaces.
pub mod runtime;
/// Save-review sheet projections for conflicted save attempts.
pub mod save_review;
/// Scope-truth chip projections shared across open and search foundations.
pub mod scope_truth;
/// Workspace search shell: lexical filename/path surface with scope and partiality truth.
pub mod search_shell;
/// Start Center quick-action surface and entry projections.
pub mod start_center;
/// Shared placeholder/state-card vocabulary used across shell surfaces.
pub mod state_cards;
/// Status-bar state items for target, profile, trust, encoding, and background work.
pub mod status_bar;
/// Support-bundle seed surface: live consumer of the support-bundle manifest,
/// redaction defaults, local preview, and exact-build capture provided by
/// `aureline-support`. Drives the protected walk and the failure drill.
pub mod support_seed;
/// Task seed surface: thin projection over the shared execution-context
/// object reused by the terminal pane and debug-prep seed.
pub mod tasks_seed;
/// Bottom-panel terminal pane: snapshots and tab projections from the canonical PTY host.
pub mod terminal_pane;
/// Platform windowing adapters used by the native desktop shell.
pub mod windowing;
/// Workspace switcher projections for recent-work entries and workspace transitions.
pub mod workspace_switcher;
