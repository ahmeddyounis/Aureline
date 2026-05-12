//! Desktop shell: canonical zones, layout, and live frame wiring.
//!
//! This crate is the production shell container. It defines the canonical
//! shell-zone ids, default metrics, and a small live desktop frame that renders
//! placeholder occupants in each declared zone.

#![doc(html_root_url = "https://docs.rs/aureline-shell/0.0.0")]
#![allow(
    clippy::collapsible_if,
    clippy::comparison_chain,
    clippy::derivable_impls,
    clippy::if_same_then_else,
    clippy::large_enum_variant,
    clippy::match_like_matches_macro,
    clippy::missing_const_for_thread_local,
    clippy::needless_borrow,
    clippy::needless_lifetimes,
    clippy::needless_range_loop,
    clippy::new_without_default,
    clippy::permissions_set_readonly_false,
    clippy::question_mark,
    clippy::redundant_closure,
    clippy::redundant_guards,
    clippy::too_many_arguments,
    clippy::unnecessary_map_or,
    clippy::unnecessary_to_owned,
    clippy::useless_format,
    clippy::wildcard_in_or_patterns,
    clippy::wrong_self_convention
)]

/// Accessibility-tree bridge groundwork for shell surfaces.
pub(crate) mod a11y;
/// Durable activity-center / job-row seed: rows projected from the typed
/// notification envelope plus a per-lifecycle observation, with file-backed
/// persistence so completed and failed rows survive a process restart.
pub mod activity_center;
/// AI composer / context-inspector projection over the bounded
/// [`aureline_ai`] composer seed. Read-only for mutation; no model dispatch.
pub mod ai_context_inspector;
/// Bounded AI evidence-packet seed and route/spend truth strip wedge for
/// the launch AI wedge. Reuses the upstream [`aureline_ai`] composer
/// draft verbatim and never dispatches a model. The wedge mints one
/// inspectable [`ai_truth_strip::AiEvidencePacketSeedRecord`] plus a
/// typed [`ai_truth_strip::RouteSpendTruthStripRow`] list so the chrome
/// can render a visible provider / route / path / spend strip alongside
/// the AI context inspector without overstating what the seed can do.
pub mod ai_truth_strip;
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
/// Background repository clone execution and typed Git failure mapping.
pub mod clone;
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
/// Target graph state card: graph-readiness truth on one bounded prototype
/// wedge. Reuses the workspace-graph and readiness vocabularies behind
/// explicit Labs inspection until promoted.
pub mod graph_state_card;
/// Help and inspection projections used by shell surfaces.
pub mod help;
/// Help / About / provenance / service-health seed surface with client-scope
/// badges projected from the shared build-info, runtime, and docs/help truth.
pub mod help_about;
/// Bounded host-boundary cue and target-identity-handoff wedge on the
/// bottom-panel terminal pane. Records each session lifecycle event as a
/// typed handoff step that preserves source / current target identity and
/// keeps the boundary cue visible through degraded, reconnecting, and
/// policy-blocked states. Reuses the
/// [`crate::badges::target_origin`] vocabulary instead of forking it.
pub mod host_boundary_cues;
/// Import profile classifier and first-pass review records for external IDE
/// configuration roots.
pub mod import;
/// Bounded install-review fact-grid wedge on one certified install-like
/// action (extension-bearing). Projects the upstream
/// [`aureline_extensions::manifest_baseline`] manifest, effective-
/// permission, and install-decision records into a structured fact
/// grid that lists publisher identity, origin/source, lifecycle and
/// compatibility, declared and effective permissions, the typed
/// install decision, the activation budget, and the rollback posture
/// before commit. Refuses to admit a widening attempt and refuses to
/// admit an extension whose rollback posture is "not yet admitted".
pub mod install_review_fact_grid;
pub mod layout;
/// Bounded managed-workspace lifecycle-labels wedge on one certified
/// prototype path. Mints typed lifecycle steps (authenticating /
/// connecting / warming / ready / reconnecting / read-only degraded /
/// suspended / reprovisioning / snapshot-only view / closed) that
/// distinguish live environment, snapshot, suspended workspace, and
/// fresh reprovisioned copy. Reuses the shared
/// [`crate::state_cards::DegradedStateToken`] vocabulary and mirrors
/// the upstream locality/tenancy/key-mode tokens on the per-step
/// authority lineage rather than forking them.
pub mod managed_workspace_labels;
/// Bounded notebook-trust-badge and representation-state wedge on one
/// certified notebook-like preview row. Renders workspace, notebook,
/// kernel, output, and widget trust as visibly distinct axes alongside
/// the per-row representation state and escape hatches. The wedge never
/// autoexecutes active content on notebook open.
pub mod notebook_trust_badges;
/// Notification routing: toast, banner, status, and durable-activity row
/// projections derived from the typed notification envelope contract.
pub mod notifications;
/// Command palette query-session state and grouped result projections.
pub mod palette;
/// Path-truth chip, alias inspector, and pre-write save-target review projections.
pub mod path_truth;
/// Bounded typed-permission-prompt wedge on one certified
/// ecosystem-bearing install-review path. Projects the upstream
/// [`install_review_fact_grid`] truth into one inspectable
/// `TypedPermissionPromptRecord` carrying actor, authority owner,
/// scope, consequence, deny path, persistence semantics, and a typed
/// invariant set the chrome quotes verbatim — so the prompt cannot
/// collapse to a generic "Allow?" button.
pub mod permission_prompts;
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
/// Representation-labeled safe-preview and copy/export card: shell projection
/// over the bounded [`aureline_preview`] safe-preview wedge. Projects the
/// canonical preview record into a card snapshot the chrome quotes
/// verbatim — including the prototype label chip, the currently visible
/// representation, the paired copy/export options, and the explicit
/// representation-honesty invariants.
pub mod safe_preview_card;
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
/// Labs-only inspector projection over bounded wedge plaintext panels.
pub mod wedge_inspector;
/// Platform windowing adapters used by the native desktop shell.
pub mod windowing;
/// Workspace switcher projections for recent-work entries and workspace transitions.
pub mod workspace_switcher;
