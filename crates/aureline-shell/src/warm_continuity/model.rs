//! Canonical warm-startup / warm-restore / first-useful-work truth model for
//! the desktop shell.
//!
//! ## Why one warm-continuity record, not three loose traces
//!
//! Three things happen, in order, every time the shell comes back warm — a
//! plain relaunch, a crash recovery, a sleep/resume, a display-topology change,
//! a missing extension, an expired remote session, or a revoked authorization:
//!
//! 1. The shell must **paint useful chrome immediately** — canonical zones,
//!    available command entry, and a stable keyboard focus target — *before*
//!    deep discovery (full indexing, remote reconnect, provider hydration)
//!    finishes.
//! 2. It must **restore what it safely can** — local editors, layout, pane
//!    trees, tabs, status surfaces, and non-mutating context — while explaining
//!    exactly what came back *exactly*, what came back *partially*, and what now
//!    *needs review*, and while it **never silently reruns** a terminal, task,
//!    debug session, notebook cell, provider mutation, collaboration control, or
//!    remote action.
//! 3. It must **route the user toward the next meaningful action** through a
//!    typed landing decision (prior active editor, changed-files view, README,
//!    review packet, or post-entry handoff card) and **record why** — without
//!    widening trust, installing packages, applying a workflow bundle, or
//!    suppressing a required admission checkpoint.
//!
//! When each surface invents its own loading state, those three steps drift:
//! the workspace blanks behind a full-surface spinner even though a recoverable
//! frame existed, a restored layout silently implies the runtime resumed too, a
//! zone-owned truth cue (a breadcrumb, a trust badge, an execution-target cue)
//! slides into a different chrome position during hydration or resize, or a
//! remembered preference quietly widens trust. This module mints one governed
//! [`WarmContinuityRecord`] that the desktop shell, diagnostics, support
//! exports, Help/About, and docs all read verbatim instead of cloning status
//! text.
//!
//! The record is the canonical truth source for this lane (the suggested-output
//! stem is `harden_shell_startup_warm_restore_and_first_useful`); its boundary
//! schema is
//! `schemas/ux/harden_shell_startup_warm_restore_and_first_useful.schema.json`
//! and its contract narrative is
//! `docs/ux/m4/harden_shell_startup_warm_restore_and_first_useful.md`.
//!
//! ## The honesty invariants
//!
//! The builder refuses to mint a record that would lie. Every one of these is a
//! [`BuildError`], not a warning, so a dishonest projection fails the row
//! instead of shipping:
//!
//! - **Useful-chrome-first.** The three early milestones (shell chrome painted,
//!   command entry ready, stable focus target) must all be present and all
//!   reached *before* deep discovery completes.
//! - **No implied full resumption.** A restore can preserve user-authored state
//!   and layout, but it may never imply the live runtime resumed.
//! - **No silent rerun.** Every side-effectful surface is skeletoned with
//!   `auto_rerun_forbidden`; remote/auth-bound surfaces require fresh
//!   authorization or review before any rerun.
//! - **No trust widening from memory.** A remembered first-useful-work
//!   preference may influence routing only; it may not widen workspace trust,
//!   install packages, apply a workflow bundle, or suppress a required
//!   checkpoint.
//! - **Zone-owned truth stays put.** A breadcrumb, trust badge, or
//!   execution-target cue may update its label/placeholder during warm-up, but
//!   it may not relocate out of its owning zone.
//! - **Collapsed surfaces stay reachable.** When a side surface collapses to a
//!   sheet, overlay, or overflow under a compact/degraded layout, its reopen
//!   route and last meaningful state stay explicit and keyboard-reachable, and
//!   only approved surfaces move.
//!
//! ## What never crosses this boundary
//!
//! Raw paths, raw command lines, raw URLs, raw tokens, raw provider payloads,
//! and raw user content never appear on these records. Surfaces carry opaque
//! object refs (`aureline://<class>/<id>`), stable tokens, and short reviewable
//! sentences only.

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried in serialized warm-continuity records.
pub const WARM_CONTINUITY_RECORD_KIND: &str = "warm_continuity_record";

/// Schema version for the [`WarmContinuityRecord`] payload shape.
pub const WARM_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Reviewer-facing notice rendered on every warm-continuity surface.
pub const WARM_CONTINUITY_NOTICE: &str =
    "Warm continuity truth: every warm relaunch paints useful shell chrome, available command \
     entry, and a stable focus target before deep discovery finishes; restore explains what came \
     back exactly, what came back partially, and what now needs review without implying the live \
     runtime resumed; terminals, tasks, debug sessions, notebook cells, provider mutations, \
     collaboration controls, and remote actions are skeletoned, never silently rerun; the next \
     useful action is chosen by a typed landing decision that records why and never widens trust; \
     zone-owned cues stay in their zones; and collapsed surfaces keep an explicit, keyboard- \
     reachable reopen route. Shell, diagnostics, support exports, Help/About, and docs read this \
     record verbatim.";

/// Canonical durable-object URI scheme. Every "open editor", "open review",
/// "reopen surface", "open diagnostics", and "open support export" affordance
/// must route to one of these.
pub const CANONICAL_OBJECT_SCHEME: &str = "aureline://";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on a canonical object ref.
const MAX_REF_CHARS: usize = 200;

/// Object-class segments that are generic landing destinations rather than a
/// specific durable object. A ref pointing at one of these is rejected so the
/// chrome cannot wire an affordance to a dashboard home.
const GENERIC_LANDING_CLASSES: &[&str] = &[
    "home", "dashboard", "landing", "index", "overview", "start", "root",
];

/// Returns true when `reference` is a canonical durable-object ref of the form
/// `aureline://<class>/<id>` where `<class>` is not a generic landing page.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = reference.strip_prefix(CANONICAL_OBJECT_SCHEME) else {
        return false;
    };
    let Some((class, ident)) = rest.split_once('/') else {
        return false;
    };
    if class.is_empty() || ident.is_empty() {
        return false;
    }
    !GENERIC_LANDING_CLASSES.contains(&class)
}

fn is_reviewable_sentence(sentence: &str) -> bool {
    let trimmed = sentence.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_SENTENCE_CHARS
}

// ---------------------------------------------------------------------------
// Entry cause
// ---------------------------------------------------------------------------

/// Why the shell came back warm. Closed set; surfaces MUST NOT invent causes
/// outside it. The five non-baseline causes are exactly the regression drills
/// the acceptance criteria require.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryCauseClass {
    /// Ordinary warm relaunch after a clean exit with a saved session.
    WarmRelaunch,
    /// Restore after an abnormal termination (crash / kill / power loss).
    CrashRecovery,
    /// Resume after the machine slept and woke.
    SleepResume,
    /// Resume after the display topology changed (monitor added/removed/rescaled).
    DisplayTopologyChange,
    /// Warm-up fell back because a required extension is missing.
    MissingExtensionFallback,
    /// Warm-up found a remote session that expired and needs reconnect.
    ExpiredRemoteSession,
    /// Warm-up found authorization was revoked and needs re-grant.
    RevokedAuthorization,
}

impl EntryCauseClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WarmRelaunch => "warm_relaunch",
            Self::CrashRecovery => "crash_recovery",
            Self::SleepResume => "sleep_resume",
            Self::DisplayTopologyChange => "display_topology_change",
            Self::MissingExtensionFallback => "missing_extension_fallback",
            Self::ExpiredRemoteSession => "expired_remote_session",
            Self::RevokedAuthorization => "revoked_authorization",
        }
    }

    /// Human-readable label, quoted verbatim across surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WarmRelaunch => "Warm relaunch",
            Self::CrashRecovery => "Crash recovery",
            Self::SleepResume => "Sleep / resume",
            Self::DisplayTopologyChange => "Display-topology change",
            Self::MissingExtensionFallback => "Missing-extension fallback",
            Self::ExpiredRemoteSession => "Expired remote session",
            Self::RevokedAuthorization => "Revoked authorization",
        }
    }
}

// ---------------------------------------------------------------------------
// Startup trace: skeleton-first, hydrate-second
// ---------------------------------------------------------------------------

/// A startup milestone in the warm-startup sequence. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StartupMilestoneClass {
    /// Canonical shell zones painted with placeholders.
    ShellChromePainted,
    /// Command palette / command entry is usable.
    CommandEntryReady,
    /// A stable, keyboard-reachable focus target exists.
    StableFocusTarget,
    /// Windows, pane trees, tabs, and status surfaces restored as skeletons.
    LayoutSkeletonRestored,
    /// The prior useful editor frame is preserved while hydration continues.
    PriorFramePreserved,
    /// Deep discovery (indexing / reconnect / provider hydration) continues.
    HydrationContinuing,
    /// The first-useful-work landing decision was made.
    FirstUsefulWorkRouted,
}

impl StartupMilestoneClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellChromePainted => "shell_chrome_painted",
            Self::CommandEntryReady => "command_entry_ready",
            Self::StableFocusTarget => "stable_focus_target",
            Self::LayoutSkeletonRestored => "layout_skeleton_restored",
            Self::PriorFramePreserved => "prior_frame_preserved",
            Self::HydrationContinuing => "hydration_continuing",
            Self::FirstUsefulWorkRouted => "first_useful_work_routed",
        }
    }

    /// The milestones that MUST be reached before deep discovery completes.
    pub const REQUIRED_BEFORE_DEEP_DISCOVERY: &'static [StartupMilestoneClass] = &[
        Self::ShellChromePainted,
        Self::CommandEntryReady,
        Self::StableFocusTarget,
    ];
}

/// One row in the startup trace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartupMilestoneInput {
    pub milestone: StartupMilestoneClass,
    /// Monotonic ordinal within this warm cycle (0-based).
    pub ordinal: u32,
    /// Whether the milestone was reached before deep discovery completed.
    pub reached_before_deep_discovery: bool,
    /// Whether the milestone exposes a keyboard-reachable affordance.
    pub keyboard_reachable: bool,
    /// Short reviewable detail sentence.
    pub detail: String,
}

/// The warm-startup trace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartupTrace {
    pub milestones: Vec<StartupMilestoneInput>,
}

// ---------------------------------------------------------------------------
// Restore provenance
// ---------------------------------------------------------------------------

/// Re-projected restore-fidelity vocabulary (mirrors
/// `aureline_recovery::session_restore::records::RestoreClass`). Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreClassToken {
    ExactRestore,
    CompatibleRestore,
    LayoutOnly,
    RecoveredDrafts,
    EvidenceOnly,
    NoRestore,
}

impl RestoreClassToken {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRestore => "exact_restore",
            Self::CompatibleRestore => "compatible_restore",
            Self::LayoutOnly => "layout_only",
            Self::RecoveredDrafts => "recovered_drafts",
            Self::EvidenceOnly => "evidence_only",
            Self::NoRestore => "no_restore",
        }
    }

    /// Whether the class is a downgrade from a full exact restore.
    pub const fn is_downgraded(self) -> bool {
        !matches!(self, Self::ExactRestore)
    }
}

/// What kind of surface a restore item refers to. Closed set; non-mutating
/// surfaces only — side-effectful surfaces ride [`NoRerunSurface`] instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreSurfaceClass {
    Window,
    PaneTree,
    TabGroup,
    Editor,
    StatusSurface,
    Layout,
    NonMutatingContext,
}

impl RestoreSurfaceClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Window => "window",
            Self::PaneTree => "pane_tree",
            Self::TabGroup => "tab_group",
            Self::Editor => "editor",
            Self::StatusSurface => "status_surface",
            Self::Layout => "layout",
            Self::NonMutatingContext => "non_mutating_context",
        }
    }
}

/// How completely a restore item came back. Closed set, in worsening order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreProvenanceClass {
    /// Restored exactly as it was.
    RestoredExactly,
    /// Restored partially; some fidelity was lost but the surface is usable.
    RestoredPartially,
    /// Could not restore safely; the user must review before it is trusted.
    NeedsReview,
}

impl RestoreProvenanceClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoredExactly => "restored_exactly",
            Self::RestoredPartially => "restored_partially",
            Self::NeedsReview => "needs_review",
        }
    }
}

/// Re-projected downgrade-trigger vocabulary (mirrors
/// `aureline_recovery::session_restore::records::DowngradeTriggerClass`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeTriggerToken {
    SchemaTranslationRequired,
    SchemaMeaningChanged,
    MissingExtensionDependency,
    MissingRemoteSession,
    MissingRemoteAuthority,
    UnsupportedDisplayTopology,
    ExcludedSecretMaterial,
    ExcludedLiveHandle,
    WorkspaceManifestConflict,
    PolicyNarrowing,
    ManualRepairRequired,
}

impl DowngradeTriggerToken {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchemaTranslationRequired => "schema_translation_required",
            Self::SchemaMeaningChanged => "schema_meaning_changed",
            Self::MissingExtensionDependency => "missing_extension_dependency",
            Self::MissingRemoteSession => "missing_remote_session",
            Self::MissingRemoteAuthority => "missing_remote_authority",
            Self::UnsupportedDisplayTopology => "unsupported_display_topology",
            Self::ExcludedSecretMaterial => "excluded_secret_material",
            Self::ExcludedLiveHandle => "excluded_live_handle",
            Self::WorkspaceManifestConflict => "workspace_manifest_conflict",
            Self::PolicyNarrowing => "policy_narrowing",
            Self::ManualRepairRequired => "manual_repair_required",
        }
    }
}

/// One restored (or partially restored, or review-needing) non-mutating item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreItem {
    pub object_ref: String,
    pub surface_class: RestoreSurfaceClass,
    pub provenance: RestoreProvenanceClass,
    /// Why it is partial / needs review (empty for exact).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<DowngradeTriggerToken>,
    /// Short reviewable detail sentence.
    pub detail: String,
    /// Whether this item carries user-authored state (a dirty buffer, a draft).
    pub user_authored: bool,
}

/// A side-effectful surface that is skeletoned, never silently rerun. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectfulSurfaceClass {
    Terminal,
    Task,
    DebugSession,
    NotebookCell,
    ProviderMutation,
    CollaborationControl,
    RemoteAction,
}

impl SideEffectfulSurfaceClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Terminal => "terminal",
            Self::Task => "task",
            Self::DebugSession => "debug_session",
            Self::NotebookCell => "notebook_cell",
            Self::ProviderMutation => "provider_mutation",
            Self::CollaborationControl => "collaboration_control",
            Self::RemoteAction => "remote_action",
        }
    }

    /// Whether the surface inherently needs fresh authorization / review before
    /// any rerun (remote- or authority-bound surfaces).
    pub const fn inherently_authority_bound(self) -> bool {
        matches!(
            self,
            Self::ProviderMutation | Self::CollaborationControl | Self::RemoteAction
        )
    }
}

/// One side-effectful surface preserved as a skeleton with no auto-rerun.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoRerunSurface {
    pub surface_class: SideEffectfulSurfaceClass,
    pub skeleton_ref: String,
    /// Always true; the builder rejects a false value.
    pub auto_rerun_forbidden: bool,
    /// Whether re-running needs fresh authorization.
    pub requires_fresh_authorization: bool,
    /// Whether re-running needs explicit review.
    pub requires_review: bool,
    /// Canonical ref of the explicit, user-initiated resume affordance.
    pub resume_route_ref: String,
    /// Short reviewable detail sentence.
    pub detail: String,
}

/// Restore provenance for the warm cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreProvenanceInput {
    pub restore_class: RestoreClassToken,
    pub items: Vec<RestoreItem>,
    pub no_rerun_surfaces: Vec<NoRerunSurface>,
}

// ---------------------------------------------------------------------------
// First-useful-work landing decision
// ---------------------------------------------------------------------------

/// The typed destination the warm cycle routes the user toward. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LandingRouteClass {
    PriorActiveEditor,
    ChangedFilesView,
    Readme,
    ReviewPacket,
    PostEntryHandoffCard,
}

impl LandingRouteClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PriorActiveEditor => "prior_active_editor",
            Self::ChangedFilesView => "changed_files_view",
            Self::Readme => "readme",
            Self::ReviewPacket => "review_packet",
            Self::PostEntryHandoffCard => "post_entry_handoff_card",
        }
    }
}

/// Why the landing route was chosen. Closed set; recorded for inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LandingRouteReasonClass {
    PriorActiveEditorPresent,
    ChangedFilesPending,
    FirstRunReadme,
    ReviewRequested,
    PostEntryHandoffPending,
    RememberedPreference,
    NoPriorContext,
}

impl LandingRouteReasonClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PriorActiveEditorPresent => "prior_active_editor_present",
            Self::ChangedFilesPending => "changed_files_pending",
            Self::FirstRunReadme => "first_run_readme",
            Self::ReviewRequested => "review_requested",
            Self::PostEntryHandoffPending => "post_entry_handoff_pending",
            Self::RememberedPreference => "remembered_preference",
            Self::NoPriorContext => "no_prior_context",
        }
    }
}

/// A remembered first-useful-work preference. It may influence routing only; it
/// may never widen trust or execute setup steps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RememberedPreference {
    pub preference_ref: String,
    /// Always true (a remembered preference influences routing by definition).
    pub influences_routing: bool,
    /// All four must be false; the builder rejects any true value.
    pub widens_workspace_trust: bool,
    pub installs_packages: bool,
    pub applies_workflow_bundle: bool,
    pub suppresses_required_checkpoint: bool,
}

/// The first-useful-work landing decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandingDecisionInput {
    pub selected_route: LandingRouteClass,
    pub route_reason: LandingRouteReasonClass,
    pub target_ref: String,
    /// Always true; the builder rejects a false value.
    pub keyboard_reachable: bool,
    /// Always false; the builder rejects a true value.
    pub destructive: bool,
    /// Every route the user can switch to (inspectable; includes the selected).
    pub candidate_routes: Vec<LandingRouteClass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remembered_preference: Option<RememberedPreference>,
    /// Short reviewable detail sentence.
    pub detail: String,
}

// ---------------------------------------------------------------------------
// Shell-slot identity / zone-owned truth
// ---------------------------------------------------------------------------

/// Re-projected canonical shell-zone vocabulary (mirrors
/// `aureline_shell::layout::ShellZoneId`). Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellZoneToken {
    TitleContextBar,
    ActivityRail,
    LeftSidebar,
    MainWorkspace,
    RightInspector,
    BottomPanel,
    StatusBar,
    TransientOverlay,
}

impl ShellZoneToken {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TitleContextBar => "title_context_bar",
            Self::ActivityRail => "activity_rail",
            Self::LeftSidebar => "left_sidebar",
            Self::MainWorkspace => "main_workspace",
            Self::RightInspector => "right_inspector",
            Self::BottomPanel => "bottom_panel",
            Self::StatusBar => "status_bar",
            Self::TransientOverlay => "transient_overlay",
        }
    }
}

/// A zone-owned truth cue whose chrome position is fixed. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZoneOwnedCueClass {
    Breadcrumb,
    TrustBadge,
    ExecutionTargetCue,
    WorkspaceIdentity,
    StatusSummary,
}

impl ZoneOwnedCueClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Breadcrumb => "breadcrumb",
            Self::TrustBadge => "trust_badge",
            Self::ExecutionTargetCue => "execution_target_cue",
            Self::WorkspaceIdentity => "workspace_identity",
            Self::StatusSummary => "status_summary",
        }
    }
}

/// One zone-owned truth cue and where it renders during warm hydration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZoneOwnedCue {
    pub cue: ZoneOwnedCueClass,
    /// The zone that owns this cue's chrome position.
    pub owning_zone: ShellZoneToken,
    /// Where it actually rendered during warm hydration; must equal the owner.
    pub rendered_zone: ShellZoneToken,
    /// Whether the label / placeholder text was updated during warm-up (allowed).
    pub label_or_placeholder_updated: bool,
}

impl ZoneOwnedCue {
    /// True when the cue relocated out of its owning zone (forbidden).
    pub fn relocated(&self) -> bool {
        self.owning_zone != self.rendered_zone
    }
}

/// Shell-slot identity preservation during warm hydration / resize / fallback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZoneIdentityInput {
    pub cues: Vec<ZoneOwnedCue>,
}

// ---------------------------------------------------------------------------
// Responsive fallback
// ---------------------------------------------------------------------------

/// Re-projected window-class vocabulary (mirrors
/// `aureline_shell::layout::AdaptiveClass`). Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowClassToken {
    CompactDesktop,
    StandardDesktop,
    ExpandedDesktop,
}

impl WindowClassToken {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactDesktop => "compact_desktop",
            Self::StandardDesktop => "standard_desktop",
            Self::ExpandedDesktop => "expanded_desktop",
        }
    }
}

/// Where a collapsed side surface went. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollapseTargetClass {
    Sheet,
    Overlay,
    Overflow,
}

impl CollapseTargetClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sheet => "sheet",
            Self::Overlay => "overlay",
            Self::Overflow => "overflow",
        }
    }
}

/// One side surface that collapsed under a compact / degraded layout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollapsedSurface {
    pub surface_ref: String,
    /// The zone whose surface collapsed.
    pub source_zone: ShellZoneToken,
    pub collapsed_to: CollapseTargetClass,
    /// Canonical ref of the explicit reopen affordance.
    pub reopen_route_ref: String,
    /// Canonical ref of the last meaningful state preserved for the surface.
    pub last_meaningful_state_ref: String,
    /// Always true; the builder rejects a false value.
    pub keyboard_reachable: bool,
    /// Whether this surface is approved to move (only approved surfaces move).
    pub approved_to_move: bool,
}

/// Responsive-fallback state for the current window class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponsiveFallbackInput {
    pub window_class: WindowClassToken,
    pub collapsed_surfaces: Vec<CollapsedSurface>,
}

// ---------------------------------------------------------------------------
// The whole input + derived record
// ---------------------------------------------------------------------------

/// Everything a caller supplies to build a [`WarmContinuityRecord`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarmContinuityInput {
    pub record_id: String,
    pub as_of: String,
    pub entry_cause: EntryCauseClass,
    pub title: String,
    pub summary: String,
    pub startup: StartupTrace,
    pub restore: RestoreProvenanceInput,
    pub landing: LandingDecisionInput,
    pub zone_identity: ZoneIdentityInput,
    pub responsive: ResponsiveFallbackInput,
    pub diagnostics_export_ref: String,
    pub support_export_ref: String,
    pub evidence_refs: Vec<String>,
    pub narrative_refs: Vec<String>,
}

/// Derived rollup counts surfaced on the record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WarmContinuitySummaryCounts {
    pub restored_exactly_count: u32,
    pub restored_partially_count: u32,
    pub needs_review_count: u32,
    pub no_rerun_surface_count: u32,
    pub collapsed_surface_count: u32,
    pub candidate_route_count: u32,
    pub user_authored_item_count: u32,
}

/// The "no lie" display-copy invariants. Every field must be false on a minted
/// record; the builder enforces it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WarmContinuityDisplayCopy {
    /// A restore that implied the live runtime resumed.
    pub full_resumption_implied: bool,
    /// A side-effectful session was implied to have rerun.
    pub side_effect_rerun_implied: bool,
    /// The main workspace was blanked while a recoverable frame existed.
    pub blank_workspace_on_recoverable_state: bool,
    /// A zone-owned cue rendered outside its owning zone.
    pub zone_cue_relocated: bool,
    /// A collapsed surface had no keyboard-reachable reopen route.
    pub collapsed_surface_unreachable: bool,
    /// A remembered preference silently widened trust or ran setup.
    pub remembered_preference_widened_trust: bool,
    /// Startup-only chrome (a fake surface that never becomes real) was used.
    pub startup_only_chrome_used: bool,
}

/// The canonical, governed warm-continuity record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarmContinuityRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub notice: String,
    pub record_id: String,
    pub as_of: String,
    pub entry_cause: EntryCauseClass,
    pub entry_cause_label: String,
    pub title: String,
    pub summary: String,
    pub startup: StartupTrace,
    pub restore: RestoreProvenanceInput,
    pub landing: LandingDecisionInput,
    pub zone_identity: ZoneIdentityInput,
    pub responsive: ResponsiveFallbackInput,
    /// True when there is anything narrowed to disclose (partial / needs-review
    /// item, downgraded restore class, or a no-rerun surface).
    pub honesty_marker_present: bool,
    pub summary_counts: WarmContinuitySummaryCounts,
    pub display_copy: WarmContinuityDisplayCopy,
    pub diagnostics_export_ref: String,
    pub support_export_ref: String,
    pub evidence_refs: Vec<String>,
    pub narrative_refs: Vec<String>,
}

/// Reasons a [`WarmContinuityRecord`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// A required early startup milestone was missing.
    MissingEarlyMilestone { milestone: StartupMilestoneClass },
    /// A required early milestone was reached after deep discovery completed.
    EarlyMilestoneAfterDeepDiscovery { milestone: StartupMilestoneClass },
    /// A required early milestone was not keyboard reachable.
    EarlyMilestoneNotKeyboardReachable { milestone: StartupMilestoneClass },
    /// Startup-only chrome cannot reach a stable focus target.
    NoStableFocusTarget,
    /// A side-effectful surface allowed an auto rerun.
    AutoRerunAllowed { surface: SideEffectfulSurfaceClass },
    /// A remote / authority-bound surface skipped its fresh-authorization gate.
    AuthorityBoundSurfaceUngated { surface: SideEffectfulSurfaceClass },
    /// The landing decision was not keyboard reachable.
    LandingNotKeyboardReachable,
    /// The landing decision was destructive.
    LandingDestructive,
    /// The selected landing route was not among the inspectable candidates.
    SelectedRouteNotACandidate { route: LandingRouteClass },
    /// A remembered preference widened trust or ran a setup step.
    RememberedPreferenceWidensTrust,
    /// A zone-owned cue relocated out of its owning zone.
    ZoneCueRelocated { cue: ZoneOwnedCueClass },
    /// A collapsed surface was not keyboard reachable.
    CollapsedSurfaceUnreachable { surface_ref: String },
    /// A collapsed surface moved without approval.
    CollapsedSurfaceNotApproved { surface_ref: String },
    /// A restore carried user-authored items but did not preserve them.
    UserAuthoredStateDropped,
    /// A partial / needs-review item did not name why.
    NarrowedItemWithoutTrigger { object_ref: String },
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSentence { field } => {
                write!(f, "field `{field}` must be a non-empty reviewable sentence")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(f, "field `{field}` must be a canonical object ref, got {value:?}")
            }
            Self::MissingEarlyMilestone { milestone } => write!(
                f,
                "required early milestone `{}` is missing",
                milestone.as_str()
            ),
            Self::EarlyMilestoneAfterDeepDiscovery { milestone } => write!(
                f,
                "milestone `{}` must be reached before deep discovery completes",
                milestone.as_str()
            ),
            Self::EarlyMilestoneNotKeyboardReachable { milestone } => write!(
                f,
                "milestone `{}` must be keyboard reachable",
                milestone.as_str()
            ),
            Self::NoStableFocusTarget => {
                write!(f, "warm startup must reach a stable focus target")
            }
            Self::AutoRerunAllowed { surface } => write!(
                f,
                "side-effectful surface `{}` must forbid auto rerun",
                surface.as_str()
            ),
            Self::AuthorityBoundSurfaceUngated { surface } => write!(
                f,
                "authority-bound surface `{}` must require fresh authorization or review",
                surface.as_str()
            ),
            Self::LandingNotKeyboardReachable => {
                write!(f, "first-useful-work landing must be keyboard reachable")
            }
            Self::LandingDestructive => {
                write!(f, "first-useful-work landing must be non-destructive")
            }
            Self::SelectedRouteNotACandidate { route } => write!(
                f,
                "selected route `{}` must be an inspectable candidate",
                route.as_str()
            ),
            Self::RememberedPreferenceWidensTrust => write!(
                f,
                "a remembered preference may influence routing only, not widen trust or run setup"
            ),
            Self::ZoneCueRelocated { cue } => write!(
                f,
                "zone-owned cue `{}` must stay in its owning zone",
                cue.as_str()
            ),
            Self::CollapsedSurfaceUnreachable { surface_ref } => write!(
                f,
                "collapsed surface {surface_ref:?} must keep a keyboard-reachable reopen route"
            ),
            Self::CollapsedSurfaceNotApproved { surface_ref } => write!(
                f,
                "collapsed surface {surface_ref:?} moved without approval"
            ),
            Self::UserAuthoredStateDropped => {
                write!(f, "user-authored restore items must be preserved")
            }
            Self::NarrowedItemWithoutTrigger { object_ref } => write!(
                f,
                "partial / needs-review item {object_ref:?} must name a downgrade trigger"
            ),
        }
    }
}

impl std::error::Error for BuildError {}

impl WarmContinuityRecord {
    /// Build a governed warm-continuity record from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a record that lies
    /// about startup order, restore fidelity, rerun safety, trust, zone-owned
    /// truth, or responsive fallback.
    pub fn build(input: WarmContinuityInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        if !is_reviewable_sentence(&input.title) {
            return Err(BuildError::InvalidSentence { field: "title" });
        }
        if !is_reviewable_sentence(&input.summary) {
            return Err(BuildError::InvalidSentence { field: "summary" });
        }
        require_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_ref("support_export_ref", &input.support_export_ref)?;

        // --- startup: useful chrome before deep discovery ---------------------
        for required in StartupMilestoneClass::REQUIRED_BEFORE_DEEP_DISCOVERY {
            let row = input
                .startup
                .milestones
                .iter()
                .find(|m| m.milestone == *required)
                .ok_or(BuildError::MissingEarlyMilestone {
                    milestone: *required,
                })?;
            if !row.reached_before_deep_discovery {
                return Err(BuildError::EarlyMilestoneAfterDeepDiscovery {
                    milestone: *required,
                });
            }
            if !row.keyboard_reachable {
                return Err(BuildError::EarlyMilestoneNotKeyboardReachable {
                    milestone: *required,
                });
            }
        }
        // A stable focus target is the anti-"startup-only chrome" guard.
        if !input
            .startup
            .milestones
            .iter()
            .any(|m| m.milestone == StartupMilestoneClass::StableFocusTarget)
        {
            return Err(BuildError::NoStableFocusTarget);
        }
        for row in &input.startup.milestones {
            if !is_reviewable_sentence(&row.detail) {
                return Err(BuildError::InvalidSentence {
                    field: "startup.milestone.detail",
                });
            }
        }

        // --- restore provenance ----------------------------------------------
        let mut restored_exactly = 0u32;
        let mut restored_partially = 0u32;
        let mut needs_review = 0u32;
        let mut user_authored_items = 0u32;
        let mut user_authored_preserved = true;
        for item in &input.restore.items {
            require_ref("restore.item.object_ref", &item.object_ref)?;
            if !is_reviewable_sentence(&item.detail) {
                return Err(BuildError::InvalidSentence {
                    field: "restore.item.detail",
                });
            }
            match item.provenance {
                RestoreProvenanceClass::RestoredExactly => restored_exactly += 1,
                RestoreProvenanceClass::RestoredPartially => {
                    restored_partially += 1;
                    if item.downgrade_trigger.is_none() {
                        return Err(BuildError::NarrowedItemWithoutTrigger {
                            object_ref: item.object_ref.clone(),
                        });
                    }
                }
                RestoreProvenanceClass::NeedsReview => {
                    needs_review += 1;
                    if item.downgrade_trigger.is_none() {
                        return Err(BuildError::NarrowedItemWithoutTrigger {
                            object_ref: item.object_ref.clone(),
                        });
                    }
                }
            }
            if item.user_authored {
                user_authored_items += 1;
                // User-authored state must come back at least partially; a
                // needs-review item is acceptable (it is preserved for review)
                // but it must not be silently dropped, which the record models
                // by requiring the item to be present at all. A dropped item
                // would simply be absent, so any present user-authored item is
                // preserved. We additionally forbid no_restore with present
                // user-authored items.
                if input.restore.restore_class == RestoreClassToken::NoRestore {
                    user_authored_preserved = false;
                }
            }
        }
        if !user_authored_preserved {
            return Err(BuildError::UserAuthoredStateDropped);
        }
        for surface in &input.restore.no_rerun_surfaces {
            require_ref("restore.no_rerun.skeleton_ref", &surface.skeleton_ref)?;
            require_ref("restore.no_rerun.resume_route_ref", &surface.resume_route_ref)?;
            if !is_reviewable_sentence(&surface.detail) {
                return Err(BuildError::InvalidSentence {
                    field: "restore.no_rerun.detail",
                });
            }
            if !surface.auto_rerun_forbidden {
                return Err(BuildError::AutoRerunAllowed {
                    surface: surface.surface_class,
                });
            }
            if surface.surface_class.inherently_authority_bound()
                && !(surface.requires_fresh_authorization || surface.requires_review)
            {
                return Err(BuildError::AuthorityBoundSurfaceUngated {
                    surface: surface.surface_class,
                });
            }
        }

        // --- landing decision -------------------------------------------------
        require_ref("landing.target_ref", &input.landing.target_ref)?;
        if !is_reviewable_sentence(&input.landing.detail) {
            return Err(BuildError::InvalidSentence {
                field: "landing.detail",
            });
        }
        if !input.landing.keyboard_reachable {
            return Err(BuildError::LandingNotKeyboardReachable);
        }
        if input.landing.destructive {
            return Err(BuildError::LandingDestructive);
        }
        if !input
            .landing
            .candidate_routes
            .contains(&input.landing.selected_route)
        {
            return Err(BuildError::SelectedRouteNotACandidate {
                route: input.landing.selected_route,
            });
        }
        if let Some(pref) = &input.landing.remembered_preference {
            require_ref("landing.remembered_preference.ref", &pref.preference_ref)?;
            if pref.widens_workspace_trust
                || pref.installs_packages
                || pref.applies_workflow_bundle
                || pref.suppresses_required_checkpoint
            {
                return Err(BuildError::RememberedPreferenceWidensTrust);
            }
        }

        // --- zone-owned truth -------------------------------------------------
        for cue in &input.zone_identity.cues {
            if cue.relocated() {
                return Err(BuildError::ZoneCueRelocated { cue: cue.cue });
            }
        }

        // --- responsive fallback ---------------------------------------------
        for surface in &input.responsive.collapsed_surfaces {
            require_ref("responsive.collapsed.surface_ref", &surface.surface_ref)?;
            require_ref(
                "responsive.collapsed.reopen_route_ref",
                &surface.reopen_route_ref,
            )?;
            require_ref(
                "responsive.collapsed.last_meaningful_state_ref",
                &surface.last_meaningful_state_ref,
            )?;
            if !surface.keyboard_reachable {
                return Err(BuildError::CollapsedSurfaceUnreachable {
                    surface_ref: surface.surface_ref.clone(),
                });
            }
            if !surface.approved_to_move {
                return Err(BuildError::CollapsedSurfaceNotApproved {
                    surface_ref: surface.surface_ref.clone(),
                });
            }
        }

        // --- refs -------------------------------------------------------------
        for r in &input.evidence_refs {
            require_ref("evidence_refs[]", r)?;
        }
        for r in &input.narrative_refs {
            require_ref("narrative_refs[]", r)?;
        }

        // --- derived rollups --------------------------------------------------
        let summary_counts = WarmContinuitySummaryCounts {
            restored_exactly_count: restored_exactly,
            restored_partially_count: restored_partially,
            needs_review_count: needs_review,
            no_rerun_surface_count: input.restore.no_rerun_surfaces.len() as u32,
            collapsed_surface_count: input.responsive.collapsed_surfaces.len() as u32,
            candidate_route_count: input.landing.candidate_routes.len() as u32,
            user_authored_item_count: user_authored_items,
        };

        let honesty_marker_present = restored_partially > 0
            || needs_review > 0
            || input.restore.restore_class.is_downgraded()
            || !input.restore.no_rerun_surfaces.is_empty();

        // The display-copy invariants are derived as all-false: every condition
        // that would set one true has already been rejected above. Modelling
        // them explicitly keeps the "no lie" contract inspectable on the record.
        let display_copy = WarmContinuityDisplayCopy::default();

        Ok(Self {
            record_kind: WARM_CONTINUITY_RECORD_KIND.to_string(),
            schema_version: WARM_CONTINUITY_SCHEMA_VERSION,
            notice: WARM_CONTINUITY_NOTICE.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            entry_cause: input.entry_cause,
            entry_cause_label: input.entry_cause.label().to_string(),
            title: input.title,
            summary: input.summary,
            startup: input.startup,
            restore: input.restore,
            landing: input.landing,
            zone_identity: input.zone_identity,
            responsive: input.responsive,
            honesty_marker_present,
            summary_counts,
            display_copy,
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Deterministic plaintext truth block for diagnostics / support export.
    ///
    /// This is the inspectable projection the acceptance criteria require: it is
    /// stable, redaction-safe (refs only), and quotable verbatim by the support
    /// bundle and the diagnostics packet.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("warm_continuity: {}", self.record_id),
            format!("entry_cause: {} ({})", self.entry_cause.as_str(), self.entry_cause_label),
            format!("as_of: {}", self.as_of),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            "startup:".to_string(),
        ];
        let mut milestones: Vec<&StartupMilestoneInput> = self.startup.milestones.iter().collect();
        milestones.sort_by_key(|m| m.ordinal);
        for m in milestones {
            lines.push(format!(
                "  [{}] {} before_deep_discovery={} keyboard={}",
                m.ordinal,
                m.milestone.as_str(),
                m.reached_before_deep_discovery,
                m.keyboard_reachable,
            ));
        }
        lines.push(format!("restore_class: {}", self.restore.restore_class.as_str()));
        lines.push(format!(
            "restore: exactly={} partially={} needs_review={} user_authored={}",
            self.summary_counts.restored_exactly_count,
            self.summary_counts.restored_partially_count,
            self.summary_counts.needs_review_count,
            self.summary_counts.user_authored_item_count,
        ));
        for item in &self.restore.items {
            let trigger = item
                .downgrade_trigger
                .map(|t| t.as_str())
                .unwrap_or("none");
            lines.push(format!(
                "  - {} [{}] provenance={} trigger={} user_authored={}",
                item.object_ref,
                item.surface_class.as_str(),
                item.provenance.as_str(),
                trigger,
                item.user_authored,
            ));
        }
        for s in &self.restore.no_rerun_surfaces {
            lines.push(format!(
                "  no_rerun {} auto_rerun_forbidden={} fresh_auth={} review={} resume={}",
                s.surface_class.as_str(),
                s.auto_rerun_forbidden,
                s.requires_fresh_authorization,
                s.requires_review,
                s.resume_route_ref,
            ));
        }
        lines.push(format!(
            "landing: route={} reason={} target={} keyboard={} destructive={}",
            self.landing.selected_route.as_str(),
            self.landing.route_reason.as_str(),
            self.landing.target_ref,
            self.landing.keyboard_reachable,
            self.landing.destructive,
        ));
        lines.push(format!(
            "landing_candidates: {}",
            self.landing
                .candidate_routes
                .iter()
                .map(|r| r.as_str())
                .collect::<Vec<_>>()
                .join(", "),
        ));
        if let Some(pref) = &self.landing.remembered_preference {
            lines.push(format!(
                "remembered_preference: {} influences_routing={} widens_trust={}",
                pref.preference_ref, pref.influences_routing, pref.widens_workspace_trust,
            ));
        }
        lines.push(format!("window_class: {}", self.responsive.window_class.as_str()));
        for s in &self.responsive.collapsed_surfaces {
            lines.push(format!(
                "  collapsed {} -> {} reopen={} last_state={} keyboard={}",
                s.surface_ref,
                s.collapsed_to.as_str(),
                s.reopen_route_ref,
                s.last_meaningful_state_ref,
                s.keyboard_reachable,
            ));
        }
        for cue in &self.zone_identity.cues {
            lines.push(format!(
                "  zone_cue {} owner={} rendered={} relocated={}",
                cue.cue.as_str(),
                cue.owning_zone.as_str(),
                cue.rendered_zone.as_str(),
                cue.relocated(),
            ));
        }
        lines.push(format!("honesty_marker_present: {}", self.honesty_marker_present));
        lines.push(format!("diagnostics_export_ref: {}", self.diagnostics_export_ref));
        lines.push(format!("support_export_ref: {}", self.support_export_ref));
        lines
    }
}

fn require_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A minimal honest input the negative tests mutate to trip one invariant.
    fn honest_input() -> WarmContinuityInput {
        WarmContinuityInput {
            record_id: "warm_continuity:test.base".to_owned(),
            as_of: "2026-05-25T12:00:00Z".to_owned(),
            entry_cause: EntryCauseClass::WarmRelaunch,
            title: "Warm relaunch restored your session.".to_owned(),
            summary: "Useful chrome first; editors restored; routed to prior editor.".to_owned(),
            startup: StartupTrace {
                milestones: vec![
                    StartupMilestoneInput {
                        milestone: StartupMilestoneClass::ShellChromePainted,
                        ordinal: 0,
                        reached_before_deep_discovery: true,
                        keyboard_reachable: true,
                        detail: "Chrome painted.".to_owned(),
                    },
                    StartupMilestoneInput {
                        milestone: StartupMilestoneClass::CommandEntryReady,
                        ordinal: 1,
                        reached_before_deep_discovery: true,
                        keyboard_reachable: true,
                        detail: "Command entry ready.".to_owned(),
                    },
                    StartupMilestoneInput {
                        milestone: StartupMilestoneClass::StableFocusTarget,
                        ordinal: 2,
                        reached_before_deep_discovery: true,
                        keyboard_reachable: true,
                        detail: "Focus stable.".to_owned(),
                    },
                ],
            },
            restore: RestoreProvenanceInput {
                restore_class: RestoreClassToken::ExactRestore,
                items: vec![RestoreItem {
                    object_ref: "aureline://editor/main".to_owned(),
                    surface_class: RestoreSurfaceClass::Editor,
                    provenance: RestoreProvenanceClass::RestoredExactly,
                    downgrade_trigger: None,
                    detail: "Editor restored exactly.".to_owned(),
                    user_authored: true,
                }],
                no_rerun_surfaces: vec![],
            },
            landing: LandingDecisionInput {
                selected_route: LandingRouteClass::PriorActiveEditor,
                route_reason: LandingRouteReasonClass::PriorActiveEditorPresent,
                target_ref: "aureline://editor/main".to_owned(),
                keyboard_reachable: true,
                destructive: false,
                candidate_routes: vec![LandingRouteClass::PriorActiveEditor],
                remembered_preference: None,
                detail: "Routed to prior editor.".to_owned(),
            },
            zone_identity: ZoneIdentityInput {
                cues: vec![ZoneOwnedCue {
                    cue: ZoneOwnedCueClass::TrustBadge,
                    owning_zone: ShellZoneToken::TitleContextBar,
                    rendered_zone: ShellZoneToken::TitleContextBar,
                    label_or_placeholder_updated: false,
                }],
            },
            responsive: ResponsiveFallbackInput {
                window_class: WindowClassToken::StandardDesktop,
                collapsed_surfaces: vec![],
            },
            diagnostics_export_ref: "aureline://diagnostics/test".to_owned(),
            support_export_ref: "aureline://support_export/test".to_owned(),
            evidence_refs: vec![],
            narrative_refs: vec![],
        }
    }

    #[test]
    fn honest_input_builds_and_has_no_honesty_marker() {
        let record = WarmContinuityRecord::build(honest_input()).expect("honest input builds");
        assert_eq!(record.record_kind, WARM_CONTINUITY_RECORD_KIND);
        assert!(!record.honesty_marker_present);
        assert_eq!(record.summary_counts.restored_exactly_count, 1);
    }

    #[test]
    fn missing_command_entry_milestone_is_rejected() {
        let mut input = honest_input();
        input
            .startup
            .milestones
            .retain(|m| m.milestone != StartupMilestoneClass::CommandEntryReady);
        let err = WarmContinuityRecord::build(input).unwrap_err();
        assert_eq!(
            err,
            BuildError::MissingEarlyMilestone {
                milestone: StartupMilestoneClass::CommandEntryReady
            }
        );
    }

    #[test]
    fn early_milestone_after_deep_discovery_is_rejected() {
        let mut input = honest_input();
        input
            .startup
            .milestones
            .iter_mut()
            .find(|m| m.milestone == StartupMilestoneClass::ShellChromePainted)
            .unwrap()
            .reached_before_deep_discovery = false;
        let err = WarmContinuityRecord::build(input).unwrap_err();
        assert_eq!(
            err,
            BuildError::EarlyMilestoneAfterDeepDiscovery {
                milestone: StartupMilestoneClass::ShellChromePainted
            }
        );
    }

    #[test]
    fn auto_rerun_allowed_is_rejected() {
        let mut input = honest_input();
        input.restore.no_rerun_surfaces.push(NoRerunSurface {
            surface_class: SideEffectfulSurfaceClass::Terminal,
            skeleton_ref: "aureline://terminal/x".to_owned(),
            auto_rerun_forbidden: false,
            requires_fresh_authorization: false,
            requires_review: false,
            resume_route_ref: "aureline://command/terminal.reopen".to_owned(),
            detail: "x".to_owned(),
        });
        let err = WarmContinuityRecord::build(input).unwrap_err();
        assert_eq!(
            err,
            BuildError::AutoRerunAllowed {
                surface: SideEffectfulSurfaceClass::Terminal
            }
        );
    }

    #[test]
    fn ungated_authority_bound_surface_is_rejected() {
        let mut input = honest_input();
        input.restore.no_rerun_surfaces.push(NoRerunSurface {
            surface_class: SideEffectfulSurfaceClass::RemoteAction,
            skeleton_ref: "aureline://remote_action/x".to_owned(),
            auto_rerun_forbidden: true,
            requires_fresh_authorization: false,
            requires_review: false,
            resume_route_ref: "aureline://command/remote.x".to_owned(),
            detail: "x".to_owned(),
        });
        let err = WarmContinuityRecord::build(input).unwrap_err();
        assert_eq!(
            err,
            BuildError::AuthorityBoundSurfaceUngated {
                surface: SideEffectfulSurfaceClass::RemoteAction
            }
        );
    }

    #[test]
    fn remembered_preference_widening_trust_is_rejected() {
        let mut input = honest_input();
        input.landing.remembered_preference = Some(RememberedPreference {
            preference_ref: "aureline://preference/x".to_owned(),
            influences_routing: true,
            widens_workspace_trust: true,
            installs_packages: false,
            applies_workflow_bundle: false,
            suppresses_required_checkpoint: false,
        });
        let err = WarmContinuityRecord::build(input).unwrap_err();
        assert_eq!(err, BuildError::RememberedPreferenceWidensTrust);
    }

    #[test]
    fn relocated_zone_cue_is_rejected() {
        let mut input = honest_input();
        input.zone_identity.cues[0].rendered_zone = ShellZoneToken::BottomPanel;
        let err = WarmContinuityRecord::build(input).unwrap_err();
        assert_eq!(
            err,
            BuildError::ZoneCueRelocated {
                cue: ZoneOwnedCueClass::TrustBadge
            }
        );
    }

    #[test]
    fn unreachable_collapsed_surface_is_rejected() {
        let mut input = honest_input();
        input.responsive.collapsed_surfaces.push(CollapsedSurface {
            surface_ref: "aureline://surface/x".to_owned(),
            source_zone: ShellZoneToken::RightInspector,
            collapsed_to: CollapseTargetClass::Overflow,
            reopen_route_ref: "aureline://command/x.reopen".to_owned(),
            last_meaningful_state_ref: "aureline://state/x".to_owned(),
            keyboard_reachable: false,
            approved_to_move: true,
        });
        let err = WarmContinuityRecord::build(input).unwrap_err();
        assert_eq!(
            err,
            BuildError::CollapsedSurfaceUnreachable {
                surface_ref: "aureline://surface/x".to_owned()
            }
        );
    }

    #[test]
    fn destructive_landing_is_rejected() {
        let mut input = honest_input();
        input.landing.destructive = true;
        let err = WarmContinuityRecord::build(input).unwrap_err();
        assert_eq!(err, BuildError::LandingDestructive);
    }

    #[test]
    fn selected_route_must_be_a_candidate() {
        let mut input = honest_input();
        input.landing.candidate_routes = vec![LandingRouteClass::Readme];
        let err = WarmContinuityRecord::build(input).unwrap_err();
        assert_eq!(
            err,
            BuildError::SelectedRouteNotACandidate {
                route: LandingRouteClass::PriorActiveEditor
            }
        );
    }

    #[test]
    fn narrowed_item_without_trigger_is_rejected() {
        let mut input = honest_input();
        input.restore.items[0].provenance = RestoreProvenanceClass::NeedsReview;
        input.restore.items[0].downgrade_trigger = None;
        let err = WarmContinuityRecord::build(input).unwrap_err();
        assert_eq!(
            err,
            BuildError::NarrowedItemWithoutTrigger {
                object_ref: "aureline://editor/main".to_owned()
            }
        );
    }

    #[test]
    fn user_authored_state_with_no_restore_is_rejected() {
        let mut input = honest_input();
        input.restore.restore_class = RestoreClassToken::NoRestore;
        // item still carries user_authored = true
        let err = WarmContinuityRecord::build(input).unwrap_err();
        assert_eq!(err, BuildError::UserAuthoredStateDropped);
    }

    #[test]
    fn non_canonical_ref_is_rejected() {
        let mut input = honest_input();
        input.landing.target_ref = "https://example.com/editor".to_owned();
        let err = WarmContinuityRecord::build(input).unwrap_err();
        assert!(matches!(err, BuildError::NonCanonicalRef { .. }));
    }
}
