//! Canonical stable truth model for **battery, thermal, suspend-resume, and
//! user-visible runtime-efficiency adaptation** on a claimed-stable desktop row.
//!
//! ## Why one governed record per efficiency posture
//!
//! Power, thermal, low-disk, and suspend/resume pressure all converge on the
//! same risk: the product quietly trades the user's active work for headroom and
//! never says so. A competitor reduces typing responsiveness "to save battery",
//! keeps a hidden pane animating off-screen, lets indexing or uploads starve the
//! foreground, or shows stale data after a resume with no label — and the user
//! reads it as generic slowness or a broken app. The truth lives only in a
//! transient toast, if anywhere.
//!
//! This module mints one governed [`RuntimeEfficiencyRecord`] per claimed-stable
//! efficiency posture. The record binds, for a single efficiency-state identity:
//!
//! - **The materialized runtime-efficiency state** — `Nominal`,
//!   `EfficiencyAware`, `ThermalConstrained`, `ProtectCore`, or `Recovery`, each
//!   bound to named shed-work classes, protected foreground paths, resume
//!   conditions, and export-safe diagnostics.
//! - **Background shed before foreground** — speculative indexing, extension
//!   warmup, AI background jobs, uploads, and provider-overlay refresh pause or
//!   throttle before typing, save, navigation, or quick-open ever regress.
//! - **Protected foreground latency bands** — editing, save, direct navigation,
//!   quick-open, and the command palette stay within published latency bands at
//!   every posture.
//! - **Hidden-pane quiescence** — a hidden, occluded, or off-screen pane commits
//!   no paint and runs no decorative animation or speculative poll while this row
//!   claims efficiency protection.
//! - **A surfaced queue-governor reason** — battery saver, thermal clamp,
//!   low-disk, suspend, and resume transitions name the governor reason, the
//!   paused lanes, and the resume owner in the shell, status, and diagnostics so
//!   they never masquerade as generic slowness or stale data.
//! - **Preserved durable state** — adaptation may pause optional work but never
//!   skips save durability, loses a dirty buffer, or hides a user-owned artifact.
//! - **Per-OS conformance** — macOS, Windows, and Linux each carry current proof.
//! - **A public claim ceiling** and **automatic narrowing** below Stable with a
//!   named reason.
//! - **Recovery, route, and accessibility parity** — the same posture reachable
//!   from the activity center, command palette, status bar, and a menu command,
//!   keyboard-first, in normal / high-contrast / zoomed layouts.
//! - **No-account / no-managed-services availability**.
//!
//! The power/thermal policy, the workload-budget decisions, the render-visibility
//! audit, and the suspend-resume continuity vocabulary are **not** reinvented
//! here. Each record is a genuine projection of the live efficiency runtime in
//! [`crate::efficiency`] and the suspend-resume / power-posture page in
//! [`crate::runtime_adaptation`], so a posture record can never drift from what
//! ships.
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `stabilize-battery-thermal-suspend-resume-and-user-visible`) are:
//!
//! - [`model`](self) — the governed record, its closed vocabularies, the builder,
//!   and the honesty invariants. The boundary schema is
//!   `schemas/ux/stabilize-battery-thermal-suspend-resume-and-user-visible.schema.json`.
//! - [`corpus`](super::corpus) — the deterministic claimed-stable matrix,
//!   projected through the live efficiency runtime, and pinned on disk under
//!   `fixtures/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible/`.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::efficiency::{EfficiencyDurabilityInvariants, EfficiencyState, HiddenPaneRenderAudit};
use crate::notification_attention_stable::model::{
    is_canonical_object_ref, AccessibilityDisclosure, AttentionRouteSurface, EntryRouteRecord,
    LayoutMode, LifecycleMarker, RecoveryActionRole, RecoveryRouteRecord, StableClaimClass,
};

/// Stable record-kind tag carried in serialized efficiency records.
pub const RUNTIME_EFFICIENCY_RECORD_KIND: &str = "runtime_efficiency_adaptation_record";

/// Schema version for the [`RuntimeEfficiencyRecord`] payload shape.
pub const RUNTIME_EFFICIENCY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const RUNTIME_EFFICIENCY_SHARED_CONTRACT_REF: &str = "shell:runtime_efficiency_adaptation_stable:v1";

/// Reviewer-facing notice rendered on every efficiency surface.
pub const RUNTIME_EFFICIENCY_NOTICE: &str =
    "Runtime-efficiency truth: each desktop posture materializes one of the runtime-efficiency \
     states (Nominal, EfficiencyAware, ThermalConstrained, ProtectCore, Recovery), and every state \
     binds named shed-work classes, protected foreground paths, resume conditions, and export-safe \
     diagnostics; speculative indexing, extension warmup, AI background jobs, uploads, and \
     provider-overlay refresh pause or throttle before typing, save, navigation, or quick-open ever \
     regress, and protected foreground paths stay within published latency bands at every posture; a \
     hidden, occluded, or off-screen pane commits no paint and runs no decorative animation or \
     speculative poll while this row claims efficiency protection; battery saver, thermal clamp, \
     low-disk, suspend, and resume transitions name the queue-governor reason, the paused lanes, and \
     the resume owner across shell, status, and diagnostics so they never masquerade as generic \
     slowness or stale data; adaptation may pause optional work but never skips save durability, \
     loses a dirty buffer, or hides a user-owned artifact; a posture that cannot prove a pillar, or \
     that sits on a binding surface whose own marker is below Stable, is narrowed below Stable with a \
     named reason rather than inheriting an adjacent green row; the same posture opens from the \
     activity center, command palette, status bar, and a menu command, keyboard-first, in normal, \
     high-contrast, and zoomed layouts; and every posture stays available without an account or \
     managed services.";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on a present (non-canonical) ref.
const MAX_REF_CHARS: usize = 200;

/// All five runtime-efficiency states the claimed-stable matrix must materialize.
pub const ALL_EFFICIENCY_STATES: [EfficiencyState; 5] = [
    EfficiencyState::Nominal,
    EfficiencyState::EfficiencyAware,
    EfficiencyState::ThermalConstrained,
    EfficiencyState::ProtectCore,
    EfficiencyState::Recovery,
];

fn is_reviewable_sentence(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_SENTENCE_CHARS
}

fn is_present_ref(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_REF_CHARS
}

fn require_canonical_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_string(),
        })
    }
}

fn require_present_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_present_ref(value) {
        Ok(())
    } else {
        Err(BuildError::MissingUpstreamRef { field })
    }
}

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// The named queue-governor reason that drove a posture. This is the
/// user-facing "why", surfaced so a power/thermal/disk/suspend transition is
/// never read as generic slowness or stale data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernorReasonClass {
    /// No governor pressure; ordinary published budgets apply.
    NoneNominal,
    /// Operating-system battery saver or user low-power mode is active.
    BatterySaver,
    /// Battery is low; optional work is reduced.
    LowBattery,
    /// Battery is critical; only core interaction is funded.
    CriticalBattery,
    /// Host or OS thermal clamp is active.
    ThermalClamp,
    /// Local disk headroom is low; write-heavy background work is paused.
    LowDisk,
    /// A suspend/resume cycle interrupted the foreground.
    SuspendResume,
    /// Pressure cleared and deferred work is resuming in stages.
    PressureCleared,
}

impl GovernorReasonClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneNominal => "none_nominal",
            Self::BatterySaver => "battery_saver",
            Self::LowBattery => "low_battery",
            Self::CriticalBattery => "critical_battery",
            Self::ThermalClamp => "thermal_clamp",
            Self::LowDisk => "low_disk",
            Self::SuspendResume => "suspend_resume",
            Self::PressureCleared => "pressure_cleared",
        }
    }

    /// Human-readable label rendered next to the active state.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoneNominal => "no power or thermal pressure",
            Self::BatterySaver => "battery saver",
            Self::LowBattery => "low battery",
            Self::CriticalBattery => "critical battery",
            Self::ThermalClamp => "thermal clamp",
            Self::LowDisk => "low disk",
            Self::SuspendResume => "suspend/resume",
            Self::PressureCleared => "pressure cleared",
        }
    }

    /// True when the governor reason represents active degradation the shell must
    /// surface (everything except the nominal no-pressure reason).
    pub const fn requires_surfacing(self) -> bool {
        !matches!(self, Self::NoneNominal)
    }
}

/// Protected foreground path that must stay within a published latency band at
/// every posture. Editing, save, direct navigation, quick-open, and the command
/// palette are the paths efficiency adaptation may never trade for headroom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedForegroundPath {
    /// Typing and editing the active buffer (insert, delete, undo).
    EditTyping,
    /// Saving the active document to durable storage.
    SaveDocument,
    /// Direct navigation to a known target (go-to-file, go-to-symbol).
    DirectNavigation,
    /// Quick-open / fuzzy file finder.
    QuickOpen,
    /// Opening and dispatching from the command palette.
    CommandPalette,
}

impl ProtectedForegroundPath {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditTyping => "edit_typing",
            Self::SaveDocument => "save_document",
            Self::DirectNavigation => "direct_navigation",
            Self::QuickOpen => "quick_open",
            Self::CommandPalette => "command_palette",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::EditTyping => "Editing the active buffer",
            Self::SaveDocument => "Saving the document",
            Self::DirectNavigation => "Direct navigation",
            Self::QuickOpen => "Quick open",
            Self::CommandPalette => "Command palette",
        }
    }

    /// Every protected path a Stable posture must keep within its latency band.
    pub const REQUIRED: [Self; 5] = [
        Self::EditTyping,
        Self::SaveDocument,
        Self::DirectNavigation,
        Self::QuickOpen,
        Self::CommandPalette,
    ];
}

/// Who owns bringing paused or shed background work back online once pressure
/// clears. The resume owner is surfaced so a paused lane never looks stuck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResumeOwner {
    /// Work resumes automatically once the runtime observes pressure clear.
    AutomaticOnPressureClear,
    /// The user must plug in / leave low-power mode before resume.
    UserRestorePower,
    /// The user explicitly resumes the paused work.
    UserResume,
    /// Admin or local policy controls when the lane resumes.
    AdminPolicy,
    /// A remote reconnect must complete before the lane resumes.
    RemoteReconnect,
}

impl ResumeOwner {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutomaticOnPressureClear => "automatic_on_pressure_clear",
            Self::UserRestorePower => "user_restore_power",
            Self::UserResume => "user_resume",
            Self::AdminPolicy => "admin_policy",
            Self::RemoteReconnect => "remote_reconnect",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AutomaticOnPressureClear => "resumes automatically when pressure clears",
            Self::UserRestorePower => "resumes after you restore power",
            Self::UserResume => "resumes when you choose",
            Self::AdminPolicy => "resumes under admin policy",
            Self::RemoteReconnect => "resumes after reconnect",
        }
    }
}

/// Per-OS desktop profile a conformance row covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformProfileClass {
    /// macOS (universal).
    #[serde(rename = "macos")]
    MacOs,
    /// Windows (x86_64).
    Windows,
    /// Linux (GNOME/Wayland, x86_64).
    Linux,
}

impl PlatformProfileClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MacOs => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
        }
    }

    /// Every per-OS profile a Stable conformance posture must cover.
    pub const REQUIRED: [Self; 3] = [Self::MacOs, Self::Windows, Self::Linux];
}

/// Surface that ingests the shared efficiency record. The same record drives the
/// shell status strip, the in-product diagnostics review, the CLI inspector,
/// Help/About, and the support export rather than each cloning prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EfficiencyTruthSurface {
    /// The shell status strip / status overflow efficiency pill.
    ShellStatusStrip,
    /// The in-product efficiency / diagnostics review surface.
    DiagnosticsReview,
    /// The CLI / headless inspector.
    CliInspect,
    /// The Help/About efficiency posture.
    HelpAbout,
    /// The diagnostics support export.
    SupportExport,
}

impl EfficiencyTruthSurface {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellStatusStrip => "shell_status_strip",
            Self::DiagnosticsReview => "diagnostics_review",
            Self::CliInspect => "cli_inspect",
            Self::HelpAbout => "help_about",
            Self::SupportExport => "support_export",
        }
    }

    /// The five surfaces that must all bind the shared record.
    pub const REQUIRED: [Self; 5] = [
        Self::ShellStatusStrip,
        Self::DiagnosticsReview,
        Self::CliInspect,
        Self::HelpAbout,
        Self::SupportExport,
    ];
}

/// Closed recovery-action vocabulary exposed on an efficiency posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EfficiencyRecoveryAction {
    /// Open the efficiency status detail for the active posture.
    OpenEfficiencyStatus,
    /// Review the paused / shed background work and its resume owners.
    ReviewPausedWork,
    /// Resume paused work now (only when the resume owner allows it).
    ResumePausedWork,
    /// Override the adaptation for this session (only when allowed).
    OverrideForSession,
    /// Open the runtime diagnostics center.
    OpenDiagnostics,
    /// Export a redacted efficiency-support packet.
    ExportEfficiencySupport,
}

impl EfficiencyRecoveryAction {
    /// Stable action id quoted across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenEfficiencyStatus => "open_efficiency_status",
            Self::ReviewPausedWork => "review_paused_work",
            Self::ResumePausedWork => "resume_paused_work",
            Self::OverrideForSession => "override_for_session",
            Self::OpenDiagnostics => "open_diagnostics",
            Self::ExportEfficiencySupport => "export_efficiency_support",
        }
    }

    /// Reviewer-facing label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::OpenEfficiencyStatus => "Open efficiency status",
            Self::ReviewPausedWork => "Review paused work",
            Self::ResumePausedWork => "Resume paused work",
            Self::OverrideForSession => "Override for this session",
            Self::OpenDiagnostics => "Open diagnostics",
            Self::ExportEfficiencySupport => "Export efficiency support",
        }
    }

    /// Placement / confirmation role for this action.
    pub const fn role(self) -> RecoveryActionRole {
        match self {
            Self::OpenEfficiencyStatus | Self::ReviewPausedWork => RecoveryActionRole::Primary,
            Self::ResumePausedWork | Self::OverrideForSession => RecoveryActionRole::Recovery,
            Self::OpenDiagnostics | Self::ExportEfficiencySupport => RecoveryActionRole::Secondary,
        }
    }

    /// Builds a route record for this action.
    pub fn route(self) -> RecoveryRouteRecord {
        RecoveryRouteRecord {
            action_id: self.as_str().to_string(),
            action_label: self.surface_label().to_string(),
            action_role: self.role(),
            keyboard_reachable: true,
        }
    }

    /// The recovery actions every posture must expose regardless of state.
    pub const REQUIRED: [Self; 3] = [
        Self::OpenEfficiencyStatus,
        Self::ReviewPausedWork,
        Self::ExportEfficiencySupport,
    ];
}

/// Returns the recovery routes a posture must expose, in rendered order, given
/// whether paused work can be resumed and whether a session override is offered.
pub fn required_recovery_routes(resumable: bool, overridable: bool) -> Vec<RecoveryRouteRecord> {
    let mut actions = vec![
        EfficiencyRecoveryAction::OpenEfficiencyStatus,
        EfficiencyRecoveryAction::ReviewPausedWork,
    ];
    if resumable {
        actions.push(EfficiencyRecoveryAction::ResumePausedWork);
    }
    if overridable {
        actions.push(EfficiencyRecoveryAction::OverrideForSession);
    }
    actions.push(EfficiencyRecoveryAction::OpenDiagnostics);
    actions.push(EfficiencyRecoveryAction::ExportEfficiencySupport);
    actions.into_iter().map(EfficiencyRecoveryAction::route).collect()
}

/// Closed reason a posture is narrowed below Stable. Required whenever the claim
/// class is below the cutline; forbidden when it is Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EfficiencyNarrowingReason {
    /// The efficiency state is not materialized with named shed-work, protected
    /// paths, resume conditions, and diagnostics.
    EfficiencyStateNotMaterialized,
    /// Background work is not shed before foreground work regresses.
    BackgroundNotShedBeforeForeground,
    /// A protected foreground path exceeds its published latency band.
    ForegroundExceedsLatencyBand,
    /// A hidden pane is still painting, animating, or polling off-screen.
    HiddenPanesNotQuiescent,
    /// The queue-governor reason / paused lanes / resume owner are not surfaced.
    GovernorReasonNotSurfaced,
    /// Adaptation does not preserve save durability or local durable state.
    DurableStateNotPreserved,
    /// Per-OS conformance is incomplete.
    PlatformConformanceIncomplete,
    /// The binding surface's own lifecycle marker is below Stable, so it must not
    /// inherit Stable by adjacency.
    SurfaceNotYetStable,
}

impl EfficiencyNarrowingReason {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EfficiencyStateNotMaterialized => "efficiency_state_not_materialized",
            Self::BackgroundNotShedBeforeForeground => "background_not_shed_before_foreground",
            Self::ForegroundExceedsLatencyBand => "foreground_exceeds_latency_band",
            Self::HiddenPanesNotQuiescent => "hidden_panes_not_quiescent",
            Self::GovernorReasonNotSurfaced => "governor_reason_not_surfaced",
            Self::DurableStateNotPreserved => "durable_state_not_preserved",
            Self::PlatformConformanceIncomplete => "platform_conformance_incomplete",
            Self::SurfaceNotYetStable => "surface_not_yet_stable",
        }
    }
}

// ---------------------------------------------------------------------------
// Per-pillar evidence blocks
// ---------------------------------------------------------------------------

/// One protected foreground path's latency evidence at this posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtectedPathRow {
    /// The protected foreground path.
    pub path: ProtectedForegroundPath,
    /// Published latency-band ceiling in milliseconds (p99).
    pub published_band_ms: u32,
    /// Observed p99 latency in milliseconds at this posture.
    pub observed_p99_ms: u32,
    /// Whether the observed latency stays within the published band.
    pub within_band: bool,
    /// Whether the path is preserved (never deprioritized) under this posture.
    pub preserved_under_posture: bool,
}

impl ProtectedPathRow {
    /// True when the observed latency is within band and the path is preserved.
    pub fn holds(&self) -> bool {
        self.within_band
            && self.preserved_under_posture
            && self.observed_p99_ms <= self.published_band_ms
    }
}

/// One named shed-work class paused or throttled at this posture. Projected from
/// a live [`crate::efficiency::WorkloadBudgetDecision`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShedWorkRow {
    /// Workload-family token (e.g. `speculative_prefetch`, `ai_warmup`).
    pub workload_id: String,
    /// Shared resource-governor work-class token.
    pub work_class: String,
    /// Shared queue-lane token.
    pub queue_lane: String,
    /// Budget action token (admit / throttle / defer / pause / deny / staged_resume).
    pub action: String,
    /// Whether the action changes user-visible cadence, freshness, or admission.
    pub changes_behavior: bool,
    /// Whether the work is shed before any foreground regression.
    pub shed_before_foreground: bool,
    /// Reviewable user-impact sentence projected from the capability row.
    pub user_impact_label: String,
    /// Who owns resuming this lane.
    pub resume_owner: ResumeOwner,
    /// Reviewable resume-condition sentence.
    pub resume_condition: String,
}

/// The surfaced queue-governor disclosure: the reason, the paused lanes, and the
/// resume owner, surfaced so a transition never reads as generic slowness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueGovernorDisclosure {
    /// The named governor reason that drove the posture.
    pub reason: GovernorReasonClass,
    /// Reviewable reason label.
    pub reason_label: String,
    /// Paused / throttled queue-lane tokens, in canonical order.
    pub paused_lane_tokens: Vec<String>,
    /// Who owns resuming the paused lanes overall.
    pub resume_owner: ResumeOwner,
    /// Reviewable resume-owner label.
    pub resume_owner_label: String,
    /// Whether the reason is surfaced in the shell status strip.
    pub surfaced_in_status_strip: bool,
    /// Whether the reason is surfaced in diagnostics.
    pub surfaced_in_diagnostics: bool,
    /// Whether the degradation is named rather than presented as generic slowness.
    pub not_generic_slowness: bool,
    /// Whether stale data is labeled rather than silently shown as fresh.
    pub not_stale_masquerade: bool,
}

impl QueueGovernorDisclosure {
    /// True when the disclosure is surfaced and never masquerades. A nominal
    /// (no-pressure) posture is trivially honest because there is nothing to
    /// surface.
    pub fn is_surfaced(&self) -> bool {
        if !self.reason.requires_surfacing() {
            return self.not_generic_slowness && self.not_stale_masquerade;
        }
        self.surfaced_in_status_strip
            && self.surfaced_in_diagnostics
            && self.not_generic_slowness
            && self.not_stale_masquerade
    }
}

/// Suspend-resume continuity block, projected from
/// [`crate::runtime_adaptation`]. Present only on postures driven by a
/// suspend/resume transition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuspendResumeContinuity {
    /// Upstream suspend-resume case id this block projects from.
    pub case_id_ref: String,
    /// Lifecycle-event token (e.g. `wake_from_sleep`, `resume_from_hibernate`).
    pub event_token: String,
    /// Continuity-summary tokens (reconnect / stale / resumed_work).
    pub continuity_summary_tokens: Vec<String>,
    /// Whether local non-credentialed work continues across the cycle.
    pub local_work_continues: bool,
    /// Whether privileged or mutating work was paused at the boundary.
    pub privileged_or_mutating_work_paused: bool,
    /// Whether no silent rerun or authority reuse is allowed.
    pub no_silent_rerun_or_authority_reuse: bool,
    /// Whether the user-visible resume summary is required.
    pub user_visible_resume_summary_required: bool,
    /// Recovery action tokens surfaced for the cycle.
    pub recovery_action_tokens: Vec<String>,
}

/// Per-OS conformance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformConformanceRow {
    /// The per-OS profile.
    pub profile: PlatformProfileClass,
    /// Stable profile id (e.g. `macos_15_plus_universal`).
    pub profile_id: String,
    /// Whether the profile is covered with current proof.
    pub covered: bool,
    /// Source proof ref.
    pub proof_ref: String,
    /// Named downgrade behaviors exercised on this profile.
    pub named_downgrade_behaviors: Vec<String>,
}

/// Input form of one binding surface projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EfficiencySurfaceProjectionInput {
    /// The binding surface.
    pub surface: EfficiencyTruthSurface,
    /// The surface's own lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Whether the surface reads the shared record rather than cloning prose.
    pub reads_shared_record: bool,
}

/// Output form of one binding surface projection, with a derived summary line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencySurfaceProjection {
    /// The binding surface.
    pub surface: EfficiencyTruthSurface,
    /// The surface's own lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Whether the surface reads the shared record rather than cloning prose.
    pub reads_shared_record: bool,
    /// Derived, deterministic summary line the surface renders.
    pub summary_line: String,
}

/// The proven pillars of one efficiency posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyPillars {
    /// Whether the efficiency state is materialized with named shed-work,
    /// protected paths, resume conditions, and diagnostics.
    pub efficiency_state_materialized: bool,
    /// Whether background work is shed before foreground work regresses.
    pub background_shed_before_foreground: bool,
    /// Whether protected foreground paths stay within published latency bands.
    pub foreground_within_latency_bands: bool,
    /// Whether hidden panes are quiescent (no off-screen paint / animation / poll).
    pub hidden_panes_quiescent: bool,
    /// Whether the queue-governor reason / paused lanes / resume owner are surfaced.
    pub governor_reason_surfaced: bool,
    /// Whether adaptation preserves save durability and local durable state.
    pub durable_state_preserved: bool,
    /// Whether per-OS conformance is complete.
    pub platform_conformance_complete: bool,
}

/// The public claim ceiling: what a posture is allowed to assert. Each field
/// must be provable from the posture's real evidence; the builder enforces it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EfficiencyClaimCeiling {
    /// Whether the posture may claim the efficiency state is materialized.
    pub asserts_efficiency_state_materialized: bool,
    /// Whether the posture may claim background work is shed before foreground.
    pub asserts_background_shed_before_foreground: bool,
    /// Whether the posture may claim foreground latency bands hold.
    pub asserts_foreground_within_latency_bands: bool,
    /// Whether the posture may claim hidden panes are quiescent.
    pub asserts_hidden_panes_quiescent: bool,
    /// Whether the posture may claim the governor reason is surfaced.
    pub asserts_governor_reason_surfaced: bool,
    /// Whether the posture may claim durable state is preserved.
    pub asserts_durable_state_preserved: bool,
    /// Whether the posture may claim per-OS conformance is complete.
    pub asserts_platform_conformance_complete: bool,
}

/// The derived stable-claim verdict for a posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyQualification {
    /// The derived claim class (Stable when fully qualified, else narrowed).
    pub claim_class: StableClaimClass,
    /// Whether the posture qualifies at or above the launch cutline.
    pub qualifies_stable: bool,
    /// The reasons the posture is narrowed below Stable, in canonical order.
    pub narrowing_reasons: Vec<EfficiencyNarrowingReason>,
}

/// Upstream ids the record is a genuine projection of, kept for support
/// traceability. These are upstream source refs, not canonical durable objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyUpstream {
    /// Efficiency-runtime snapshot id this record projects from.
    pub efficiency_snapshot_ref: String,
    /// Runtime-adaptation page contract ref this record projects from.
    pub runtime_adaptation_page_ref: String,
    /// Contributing workload / decision ids, in canonical order.
    pub contributing_decision_refs: Vec<String>,
}

/// Validated input used to mint a [`RuntimeEfficiencyRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeEfficiencyInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The runtime-efficiency state this posture materializes.
    pub efficiency_state: EfficiencyState,
    /// The named queue-governor reason that drove the posture.
    pub governor: QueueGovernorDisclosure,
    /// The named shed-work rows, in canonical order.
    pub shed_work: Vec<ShedWorkRow>,
    /// The protected foreground-path latency rows, in canonical order.
    pub protected_paths: Vec<ProtectedPathRow>,
    /// The hidden-pane render/poll audit projected from the efficiency runtime.
    pub hidden_pane_audit: HiddenPaneRenderAudit,
    /// The durability invariants preserved by the adaptation.
    pub durability: EfficiencyDurabilityInvariants,
    /// Optional suspend-resume continuity block.
    pub suspend_resume: Option<SuspendResumeContinuity>,
    /// The per-OS conformance rows.
    pub platform_conformance: Vec<PlatformConformanceRow>,
    /// The binding surface projections.
    pub surface_projections: Vec<EfficiencySurfaceProjectionInput>,
    /// Public claim ceiling.
    pub claim_ceiling: EfficiencyClaimCeiling,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the same posture.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the posture stays available without an account.
    pub available_without_account: bool,
    /// Whether the posture stays available without managed services.
    pub available_without_managed_services: bool,
    /// Upstream ids the record projects from.
    pub upstream: EfficiencyUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// The canonical, governed runtime-efficiency record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeEfficiencyRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The runtime-efficiency state this posture materializes.
    pub efficiency_state: EfficiencyState,
    /// The lowest binding-surface lifecycle marker.
    pub surface_lifecycle_marker: LifecycleMarker,
    /// The named queue-governor disclosure.
    pub governor: QueueGovernorDisclosure,
    /// The named shed-work rows, in canonical order.
    pub shed_work: Vec<ShedWorkRow>,
    /// The protected foreground-path latency rows, in canonical order.
    pub protected_paths: Vec<ProtectedPathRow>,
    /// The hidden-pane render/poll audit.
    pub hidden_pane_audit: HiddenPaneRenderAudit,
    /// The durability invariants preserved by the adaptation.
    pub durability: EfficiencyDurabilityInvariants,
    /// Optional suspend-resume continuity block.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend_resume: Option<SuspendResumeContinuity>,
    /// The per-OS conformance rows, in canonical order.
    pub platform_conformance: Vec<PlatformConformanceRow>,
    /// The binding surface projections, in canonical order.
    pub surface_projections: Vec<EfficiencySurfaceProjection>,
    /// The proven pillars.
    pub pillars: EfficiencyPillars,
    /// Public claim ceiling.
    pub claim_ceiling: EfficiencyClaimCeiling,
    /// The derived stable-claim verdict.
    pub stable_qualification: EfficiencyQualification,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the same posture.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the posture stays available without an account.
    pub available_without_account: bool,
    /// Whether the posture stays available without managed services.
    pub available_without_managed_services: bool,
    /// True when there is anything narrowed or below-stable to disclose.
    pub honesty_marker_present: bool,
    /// Upstream ids the record projects from.
    pub upstream: EfficiencyUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// Reasons a [`RuntimeEfficiencyRecord`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence {
        /// The offending field.
        field: &'static str,
    },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef {
        /// The offending field.
        field: &'static str,
        /// The value that failed.
        value: String,
    },
    /// A required upstream ref was missing.
    MissingUpstreamRef {
        /// The offending field.
        field: &'static str,
    },
    /// A required protected foreground path was missing.
    MissingProtectedPath {
        /// The missing path.
        path: ProtectedForegroundPath,
    },
    /// A required per-OS profile was missing.
    MissingPlatformProfile {
        /// The missing profile.
        profile: PlatformProfileClass,
    },
    /// A per-OS profile lacked current proof.
    PlatformProofMissing {
        /// The under-proven profile.
        profile: PlatformProfileClass,
    },
    /// The posture over-claims efficiency-state materialization.
    OverclaimsEfficiencyState,
    /// The posture over-claims background-before-foreground shedding.
    OverclaimsBackgroundShed,
    /// The posture over-claims foreground latency bands.
    OverclaimsForegroundLatency,
    /// The posture over-claims hidden-pane quiescence.
    OverclaimsHiddenPaneQuiescence,
    /// The posture over-claims governor-reason surfacing.
    OverclaimsGovernorReason,
    /// The posture over-claims durable-state preservation.
    OverclaimsDurableState,
    /// The posture over-claims per-OS conformance.
    OverclaimsPlatformConformance,
    /// A required recovery route was missing.
    MissingRecoveryRoute {
        /// The missing action.
        action: EfficiencyRecoveryAction,
    },
    /// A recovery route was not keyboard reachable.
    RecoveryRouteNotKeyboardReachable {
        /// The offending action id.
        action_id: String,
    },
    /// A binding surface was projected more than once.
    DuplicateSurfaceProjection {
        /// The duplicated surface.
        surface: EfficiencyTruthSurface,
    },
    /// A binding surface clones prose instead of reading the shared record.
    SurfaceClonesProse {
        /// The offending surface.
        surface: EfficiencyTruthSurface,
    },
    /// A required binding surface was missing.
    SurfaceProjectionMissing {
        /// The missing surface.
        surface: EfficiencyTruthSurface,
    },
    /// An entry-route surface was projected more than once.
    DuplicateRouteSurface {
        /// The duplicated surface.
        surface: AttentionRouteSurface,
    },
    /// An entry route was not keyboard reachable.
    RouteNotKeyboardReachable {
        /// The offending surface.
        surface: AttentionRouteSurface,
    },
    /// An entry route activates a different item.
    RouteTargetsDifferentItem {
        /// The offending surface.
        surface: AttentionRouteSurface,
    },
    /// A required entry-route surface was missing.
    RouteSurfaceMissing {
        /// The missing surface.
        surface: AttentionRouteSurface,
    },
    /// Accessibility action labels drift from the recovery routes.
    AccessibilityActionLabelsMismatch,
    /// A required layout-mode disclosure was missing.
    AccessibilityLayoutModeMissing {
        /// The missing mode.
        mode: LayoutMode,
    },
    /// A layout-mode disclosure was unreachable.
    AccessibilityLayoutModeUnreachable {
        /// The unreachable mode.
        mode: LayoutMode,
    },
    /// The posture would be hidden without an account.
    HiddenWithoutAccount,
    /// The posture would be hidden without managed services.
    HiddenWithoutManagedServices,
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSentence { field } => write!(f, "field {field} is not a reviewable sentence"),
            Self::NonCanonicalRef { field, value } => {
                write!(f, "field {field} value {value:?} is not a canonical object ref")
            }
            Self::MissingUpstreamRef { field } => write!(f, "missing upstream ref for {field}"),
            Self::MissingProtectedPath { path } => {
                write!(f, "missing protected foreground path {}", path.as_str())
            }
            Self::MissingPlatformProfile { profile } => {
                write!(f, "missing per-OS profile {}", profile.as_str())
            }
            Self::PlatformProofMissing { profile } => {
                write!(f, "per-OS profile {} lacks current proof", profile.as_str())
            }
            Self::OverclaimsEfficiencyState => write!(f, "claims efficiency state not materialized"),
            Self::OverclaimsBackgroundShed => write!(f, "claims background shed not proven"),
            Self::OverclaimsForegroundLatency => write!(f, "claims foreground latency band not held"),
            Self::OverclaimsHiddenPaneQuiescence => write!(f, "claims hidden-pane quiescence not held"),
            Self::OverclaimsGovernorReason => write!(f, "claims governor reason not surfaced"),
            Self::OverclaimsDurableState => write!(f, "claims durable state not preserved"),
            Self::OverclaimsPlatformConformance => write!(f, "claims per-OS conformance not complete"),
            Self::MissingRecoveryRoute { action } => {
                write!(f, "missing recovery route {}", action.as_str())
            }
            Self::RecoveryRouteNotKeyboardReachable { action_id } => {
                write!(f, "recovery route {action_id} not keyboard reachable")
            }
            Self::DuplicateSurfaceProjection { surface } => {
                write!(f, "duplicate surface projection {}", surface.as_str())
            }
            Self::SurfaceClonesProse { surface } => {
                write!(f, "surface {} clones prose", surface.as_str())
            }
            Self::SurfaceProjectionMissing { surface } => {
                write!(f, "missing binding surface {}", surface.as_str())
            }
            Self::DuplicateRouteSurface { surface } => {
                write!(f, "duplicate route surface {}", surface.as_str())
            }
            Self::RouteNotKeyboardReachable { surface } => {
                write!(f, "route {} not keyboard reachable", surface.as_str())
            }
            Self::RouteTargetsDifferentItem { surface } => {
                write!(f, "route {} activates a different item", surface.as_str())
            }
            Self::RouteSurfaceMissing { surface } => {
                write!(f, "missing route surface {}", surface.as_str())
            }
            Self::AccessibilityActionLabelsMismatch => {
                write!(f, "accessibility action labels drift from recovery routes")
            }
            Self::AccessibilityLayoutModeMissing { mode } => {
                write!(f, "missing layout mode {}", mode.as_str())
            }
            Self::AccessibilityLayoutModeUnreachable { mode } => {
                write!(f, "layout mode {} unreachable", mode.as_str())
            }
            Self::HiddenWithoutAccount => write!(f, "posture hidden without an account"),
            Self::HiddenWithoutManagedServices => {
                write!(f, "posture hidden without managed services")
            }
        }
    }
}

impl std::error::Error for BuildError {}

impl RuntimeEfficiencyRecord {
    /// Builds a governed runtime-efficiency record from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a record that lies
    /// about efficiency-state materialization, background shedding, foreground
    /// latency, hidden-pane quiescence, the surfaced governor reason, durable
    /// state, per-OS coverage, recovery routes, binding surfaces, route
    /// reachability, or accessibility. The stable claim class is *derived* from
    /// the evidence, so a posture can never publish a claim wider than its proof.
    pub fn build(input: RuntimeEfficiencyInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        if !is_reviewable_sentence(&input.title) {
            return Err(BuildError::InvalidSentence { field: "title" });
        }
        if !is_reviewable_sentence(&input.summary) {
            return Err(BuildError::InvalidSentence { field: "summary" });
        }
        if !is_reviewable_sentence(&input.posture_label) {
            return Err(BuildError::InvalidSentence {
                field: "posture_label",
            });
        }
        require_canonical_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_canonical_ref("support_export_ref", &input.support_export_ref)?;
        for evidence in &input.evidence_refs {
            require_canonical_ref("evidence_refs", evidence)?;
        }
        for narrative in &input.narrative_refs {
            require_canonical_ref("narrative_refs", narrative)?;
        }
        require_present_ref(
            "upstream.efficiency_snapshot_ref",
            &input.upstream.efficiency_snapshot_ref,
        )?;
        require_present_ref(
            "upstream.runtime_adaptation_page_ref",
            &input.upstream.runtime_adaptation_page_ref,
        )?;

        // --- protected paths: every required path present --------------------
        let present_paths: BTreeSet<ProtectedForegroundPath> =
            input.protected_paths.iter().map(|row| row.path).collect();
        for required in ProtectedForegroundPath::REQUIRED {
            if !present_paths.contains(&required) {
                return Err(BuildError::MissingProtectedPath { path: required });
            }
        }

        // --- per-OS conformance: every profile present with current proof ----
        for required in PlatformProfileClass::REQUIRED {
            let row = input
                .platform_conformance
                .iter()
                .find(|row| row.profile == required)
                .ok_or(BuildError::MissingPlatformProfile { profile: required })?;
            if !row.covered || row.proof_ref.trim().is_empty() {
                return Err(BuildError::PlatformProofMissing { profile: required });
            }
        }
        let platform_conformance_complete = PlatformProfileClass::REQUIRED.iter().all(|profile| {
            input.platform_conformance.iter().any(|row| {
                row.profile == *profile && row.covered && !row.proof_ref.trim().is_empty()
            })
        });

        // --- derive the pillars from the evidence ----------------------------
        let efficiency_state_materialized = ProtectedForegroundPath::REQUIRED
            .iter()
            .all(|path| present_paths.contains(path))
            && (input.efficiency_state == EfficiencyState::Nominal || !input.shed_work.is_empty());
        let background_shed_before_foreground = input
            .shed_work
            .iter()
            .filter(|row| row.changes_behavior)
            .all(|row| row.shed_before_foreground);
        let foreground_within_latency_bands =
            input.protected_paths.iter().all(ProtectedPathRow::holds);
        let hidden_panes_quiescent = input.hidden_pane_audit.passes_hidden_pane_policy;
        let governor_reason_surfaced = input.governor.is_surfaced();
        let durable_state_preserved = input.durability.save_durability_preserved
            && input.durability.dirty_buffers_preserved
            && input.durability.user_owned_artifacts_preserved;

        // --- claim ceiling: never claim what the product cannot prove --------
        if input.claim_ceiling.asserts_efficiency_state_materialized
            && !efficiency_state_materialized
        {
            return Err(BuildError::OverclaimsEfficiencyState);
        }
        if input.claim_ceiling.asserts_background_shed_before_foreground
            && !background_shed_before_foreground
        {
            return Err(BuildError::OverclaimsBackgroundShed);
        }
        if input.claim_ceiling.asserts_foreground_within_latency_bands
            && !foreground_within_latency_bands
        {
            return Err(BuildError::OverclaimsForegroundLatency);
        }
        if input.claim_ceiling.asserts_hidden_panes_quiescent && !hidden_panes_quiescent {
            return Err(BuildError::OverclaimsHiddenPaneQuiescence);
        }
        if input.claim_ceiling.asserts_governor_reason_surfaced && !governor_reason_surfaced {
            return Err(BuildError::OverclaimsGovernorReason);
        }
        if input.claim_ceiling.asserts_durable_state_preserved && !durable_state_preserved {
            return Err(BuildError::OverclaimsDurableState);
        }
        if input.claim_ceiling.asserts_platform_conformance_complete
            && !platform_conformance_complete
        {
            return Err(BuildError::OverclaimsPlatformConformance);
        }

        // --- recovery routes -------------------------------------------------
        let route_ids: Vec<&str> = input
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in EfficiencyRecoveryAction::REQUIRED {
            if !route_ids.iter().any(|id| *id == required.as_str()) {
                return Err(BuildError::MissingRecoveryRoute { action: required });
            }
        }
        for route in &input.recovery_routes {
            if !route.keyboard_reachable {
                return Err(BuildError::RecoveryRouteNotKeyboardReachable {
                    action_id: route.action_id.clone(),
                });
            }
        }

        // --- surface projections ---------------------------------------------
        let mut seen_surfaces: BTreeSet<EfficiencyTruthSurface> = BTreeSet::new();
        for projection in &input.surface_projections {
            if !seen_surfaces.insert(projection.surface) {
                return Err(BuildError::DuplicateSurfaceProjection {
                    surface: projection.surface,
                });
            }
            if !projection.reads_shared_record {
                return Err(BuildError::SurfaceClonesProse {
                    surface: projection.surface,
                });
            }
        }
        for required in EfficiencyTruthSurface::REQUIRED {
            if !seen_surfaces.contains(&required) {
                return Err(BuildError::SurfaceProjectionMissing { surface: required });
            }
        }
        let mut surface_projections: Vec<EfficiencySurfaceProjection> = Vec::new();
        for required in EfficiencyTruthSurface::REQUIRED {
            let projection = input
                .surface_projections
                .iter()
                .find(|p| p.surface == required)
                .expect("surface presence checked above");
            surface_projections.push(EfficiencySurfaceProjection {
                surface: required,
                surface_marker: projection.surface_marker,
                reads_shared_record: projection.reads_shared_record,
                summary_line: surface_summary_line(required, &input),
            });
        }
        let surface_lifecycle_marker = surface_projections
            .iter()
            .map(|projection| projection.surface_marker)
            .min()
            .unwrap_or(LifecycleMarker::Stable);

        // --- entry routes ----------------------------------------------------
        let mut seen_route_surfaces: Vec<AttentionRouteSurface> = Vec::new();
        for route in &input.routes {
            if seen_route_surfaces.contains(&route.surface) {
                return Err(BuildError::DuplicateRouteSurface {
                    surface: route.surface,
                });
            }
            seen_route_surfaces.push(route.surface);
            require_canonical_ref("routes.route_ref", &route.route_ref)?;
            if !route.keyboard_reachable {
                return Err(BuildError::RouteNotKeyboardReachable {
                    surface: route.surface,
                });
            }
            if !route.activates_same_item {
                return Err(BuildError::RouteTargetsDifferentItem {
                    surface: route.surface,
                });
            }
        }
        for required in AttentionRouteSurface::REQUIRED {
            if !seen_route_surfaces.contains(&required) {
                return Err(BuildError::RouteSurfaceMissing { surface: required });
            }
        }

        // --- accessibility ---------------------------------------------------
        if input.accessibility.action_labels.len() != input.recovery_routes.len() {
            return Err(BuildError::AccessibilityActionLabelsMismatch);
        }
        for (label, route) in input
            .accessibility
            .action_labels
            .iter()
            .zip(input.recovery_routes.iter())
        {
            if label != &route.action_label {
                return Err(BuildError::AccessibilityActionLabelsMismatch);
            }
        }
        for required in LayoutMode::REQUIRED {
            let Some(disclosure) = input
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
            else {
                return Err(BuildError::AccessibilityLayoutModeMissing { mode: required });
            };
            if !disclosure.row_narration_available || !disclosure.recovery_affordances_reachable {
                return Err(BuildError::AccessibilityLayoutModeUnreachable { mode: required });
            }
        }

        // --- availability ----------------------------------------------------
        if !input.available_without_account {
            return Err(BuildError::HiddenWithoutAccount);
        }
        if !input.available_without_managed_services {
            return Err(BuildError::HiddenWithoutManagedServices);
        }

        // --- pillars ---------------------------------------------------------
        let pillars = EfficiencyPillars {
            efficiency_state_materialized,
            background_shed_before_foreground,
            foreground_within_latency_bands,
            hidden_panes_quiescent,
            governor_reason_surfaced,
            durable_state_preserved,
            platform_conformance_complete,
        };

        // --- normalise per-OS conformance + upstream refs --------------------
        let mut platform_conformance = input.platform_conformance;
        platform_conformance.sort_by_key(|row| row.profile);
        let mut contributing_decision_refs = input.upstream.contributing_decision_refs.clone();
        contributing_decision_refs.sort();
        contributing_decision_refs.dedup();

        // --- derive the stable-claim verdict ---------------------------------
        let mut narrowing_reasons = Vec::new();
        if !efficiency_state_materialized {
            narrowing_reasons.push(EfficiencyNarrowingReason::EfficiencyStateNotMaterialized);
        }
        if !background_shed_before_foreground {
            narrowing_reasons.push(EfficiencyNarrowingReason::BackgroundNotShedBeforeForeground);
        }
        if !foreground_within_latency_bands {
            narrowing_reasons.push(EfficiencyNarrowingReason::ForegroundExceedsLatencyBand);
        }
        if !hidden_panes_quiescent {
            narrowing_reasons.push(EfficiencyNarrowingReason::HiddenPanesNotQuiescent);
        }
        if !governor_reason_surfaced {
            narrowing_reasons.push(EfficiencyNarrowingReason::GovernorReasonNotSurfaced);
        }
        if !durable_state_preserved {
            narrowing_reasons.push(EfficiencyNarrowingReason::DurableStateNotPreserved);
        }
        if !platform_conformance_complete {
            narrowing_reasons.push(EfficiencyNarrowingReason::PlatformConformanceIncomplete);
        }
        if surface_lifecycle_marker.is_below_stable() {
            narrowing_reasons.push(EfficiencyNarrowingReason::SurfaceNotYetStable);
        }
        let qualifies_stable = narrowing_reasons.is_empty();
        let claim_class = if qualifies_stable {
            StableClaimClass::Stable
        } else if narrowing_reasons.len() == 1
            && narrowing_reasons[0] == EfficiencyNarrowingReason::SurfaceNotYetStable
        {
            match surface_lifecycle_marker {
                LifecycleMarker::Preview => StableClaimClass::Preview,
                _ => StableClaimClass::Beta,
            }
        } else {
            StableClaimClass::Beta
        };
        let stable_qualification = EfficiencyQualification {
            claim_class,
            qualifies_stable,
            narrowing_reasons,
        };
        let honesty_marker_present = !qualifies_stable
            || surface_lifecycle_marker.is_below_stable()
            || input.governor.reason.requires_surfacing()
            || input.suspend_resume.is_some();

        Ok(Self {
            record_kind: RUNTIME_EFFICIENCY_RECORD_KIND.to_string(),
            schema_version: RUNTIME_EFFICIENCY_SCHEMA_VERSION,
            notice: RUNTIME_EFFICIENCY_NOTICE.to_string(),
            shared_contract_ref: RUNTIME_EFFICIENCY_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            posture_id: input.posture_id,
            posture_label: input.posture_label,
            title: input.title,
            summary: input.summary,
            efficiency_state: input.efficiency_state,
            surface_lifecycle_marker,
            governor: input.governor,
            shed_work: input.shed_work,
            protected_paths: input.protected_paths,
            hidden_pane_audit: input.hidden_pane_audit,
            durability: input.durability,
            suspend_resume: input.suspend_resume,
            platform_conformance,
            surface_projections,
            pillars,
            claim_ceiling: input.claim_ceiling,
            stable_qualification,
            recovery_routes: input.recovery_routes,
            routes: input.routes,
            accessibility: input.accessibility,
            available_without_account: input.available_without_account,
            available_without_managed_services: input.available_without_managed_services,
            honesty_marker_present,
            upstream: EfficiencyUpstream {
                efficiency_snapshot_ref: input.upstream.efficiency_snapshot_ref,
                runtime_adaptation_page_ref: input.upstream.runtime_adaptation_page_ref,
                contributing_decision_refs,
            },
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("runtime_efficiency_adaptation: {}", self.record_id),
            format!("as_of: {}", self.as_of),
            format!("posture: {} ({})", self.posture_id, self.posture_label),
            format!("efficiency_state: {}", self.efficiency_state.as_str()),
            format!(
                "surface_lifecycle_marker: {}",
                self.surface_lifecycle_marker.as_str()
            ),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            format!(
                "stable_qualification: class={} qualifies_stable={} narrowing=[{}]",
                self.stable_qualification.claim_class.as_str(),
                self.stable_qualification.qualifies_stable,
                self.stable_qualification
                    .narrowing_reasons
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            format!(
                "pillars: state_materialized={} background_shed={} foreground_latency={} hidden_panes_quiescent={} governor_surfaced={} durable_state={} platform_conformance={}",
                self.pillars.efficiency_state_materialized,
                self.pillars.background_shed_before_foreground,
                self.pillars.foreground_within_latency_bands,
                self.pillars.hidden_panes_quiescent,
                self.pillars.governor_reason_surfaced,
                self.pillars.durable_state_preserved,
                self.pillars.platform_conformance_complete
            ),
            format!(
                "governor: reason={} resume_owner={} paused_lanes=[{}] status_strip={} diagnostics={} not_generic_slowness={} not_stale_masquerade={}",
                self.governor.reason.as_str(),
                self.governor.resume_owner.as_str(),
                self.governor.paused_lane_tokens.join(", "),
                self.governor.surfaced_in_status_strip,
                self.governor.surfaced_in_diagnostics,
                self.governor.not_generic_slowness,
                self.governor.not_stale_masquerade
            ),
        ];
        lines.push("shed_work:".to_string());
        for row in &self.shed_work {
            lines.push(format!(
                "  - {} class={} lane={} action={} shed_before_foreground={} resume_owner={} :: {}",
                row.workload_id,
                row.work_class,
                row.queue_lane,
                row.action,
                row.shed_before_foreground,
                row.resume_owner.as_str(),
                row.user_impact_label
            ));
        }
        lines.push("protected_paths:".to_string());
        for row in &self.protected_paths {
            lines.push(format!(
                "  - {} band_ms={} observed_p99_ms={} within_band={} preserved={}",
                row.path.as_str(),
                row.published_band_ms,
                row.observed_p99_ms,
                row.within_band,
                row.preserved_under_posture
            ));
        }
        lines.push(format!(
            "hidden_pane_audit: audited={} hidden={} hidden_pane_work={} violations={} passes={}",
            self.hidden_pane_audit.audited_surface_count,
            self.hidden_pane_audit.hidden_surface_count,
            self.hidden_pane_audit.hidden_pane_work,
            self.hidden_pane_audit.hidden_pane_render_violation_count,
            self.hidden_pane_audit.passes_hidden_pane_policy
        ));
        lines.push(format!(
            "durability: save={} dirty_buffers={} user_artifacts={}",
            self.durability.save_durability_preserved,
            self.durability.dirty_buffers_preserved,
            self.durability.user_owned_artifacts_preserved
        ));
        if let Some(continuity) = &self.suspend_resume {
            lines.push(format!(
                "suspend_resume: case_id={} event={} continuity=[{}] local_continues={} privileged_paused={} no_silent_rerun={} resume_summary_required={}",
                continuity.case_id_ref,
                continuity.event_token,
                continuity.continuity_summary_tokens.join(", "),
                continuity.local_work_continues,
                continuity.privileged_or_mutating_work_paused,
                continuity.no_silent_rerun_or_authority_reuse,
                continuity.user_visible_resume_summary_required
            ));
        }
        lines.push("platform_conformance:".to_string());
        for row in &self.platform_conformance {
            lines.push(format!(
                "  - {} profile_id={} covered={} downgrades=[{}] :: {}",
                row.profile.as_str(),
                row.profile_id,
                row.covered,
                row.named_downgrade_behaviors.join(", "),
                row.proof_ref
            ));
        }
        lines.push("surface_projections:".to_string());
        for projection in &self.surface_projections {
            lines.push(format!(
                "  - {} marker={} reads_shared_record={} :: {}",
                projection.surface.as_str(),
                projection.surface_marker.as_str(),
                projection.reads_shared_record,
                projection.summary_line
            ));
        }
        lines.push(format!(
            "availability: without_account={} without_managed_services={}",
            self.available_without_account, self.available_without_managed_services
        ));
        lines.push(format!(
            "honesty_marker_present: {}",
            self.honesty_marker_present
        ));
        lines.push(format!(
            "diagnostics_export_ref: {}",
            self.diagnostics_export_ref
        ));
        lines.push(format!("support_export_ref: {}", self.support_export_ref));
        lines
    }
}

fn surface_summary_line(
    surface: EfficiencyTruthSurface,
    input: &RuntimeEfficiencyInput,
) -> String {
    let prefix = match surface {
        EfficiencyTruthSurface::ShellStatusStrip => "Status strip",
        EfficiencyTruthSurface::DiagnosticsReview => "Diagnostics review",
        EfficiencyTruthSurface::CliInspect => "CLI inspect",
        EfficiencyTruthSurface::HelpAbout => "Help/About",
        EfficiencyTruthSurface::SupportExport => "Support export",
    };
    let shed = input.shed_work.iter().filter(|row| row.changes_behavior).count();
    format!(
        "{prefix}: {} under {} — {} background lane(s) shed, foreground protected, {}.",
        input.efficiency_state.as_str(),
        input.governor.reason.label(),
        shed,
        input.governor.resume_owner.label(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn governor_disclosure_nominal_is_trivially_surfaced() {
        let governor = QueueGovernorDisclosure {
            reason: GovernorReasonClass::NoneNominal,
            reason_label: GovernorReasonClass::NoneNominal.label().to_string(),
            paused_lane_tokens: vec![],
            resume_owner: ResumeOwner::AutomaticOnPressureClear,
            resume_owner_label: ResumeOwner::AutomaticOnPressureClear.label().to_string(),
            surfaced_in_status_strip: false,
            surfaced_in_diagnostics: false,
            not_generic_slowness: true,
            not_stale_masquerade: true,
        };
        assert!(governor.is_surfaced());
    }

    #[test]
    fn governor_disclosure_pressure_requires_surfacing() {
        let mut governor = QueueGovernorDisclosure {
            reason: GovernorReasonClass::ThermalClamp,
            reason_label: GovernorReasonClass::ThermalClamp.label().to_string(),
            paused_lane_tokens: vec!["maintenance".to_string()],
            resume_owner: ResumeOwner::AutomaticOnPressureClear,
            resume_owner_label: ResumeOwner::AutomaticOnPressureClear.label().to_string(),
            surfaced_in_status_strip: true,
            surfaced_in_diagnostics: true,
            not_generic_slowness: true,
            not_stale_masquerade: true,
        };
        assert!(governor.is_surfaced());
        governor.surfaced_in_status_strip = false;
        assert!(!governor.is_surfaced());
    }

    #[test]
    fn protected_path_holds_within_band() {
        let row = ProtectedPathRow {
            path: ProtectedForegroundPath::EditTyping,
            published_band_ms: 50,
            observed_p99_ms: 18,
            within_band: true,
            preserved_under_posture: true,
        };
        assert!(row.holds());
        let over = ProtectedPathRow {
            within_band: false,
            observed_p99_ms: 80,
            ..row.clone()
        };
        assert!(!over.holds());
    }

    #[test]
    fn required_recovery_routes_expand_with_needs() {
        let base = required_recovery_routes(false, false);
        let ids: Vec<&str> = base.iter().map(|r| r.action_id.as_str()).collect();
        for required in EfficiencyRecoveryAction::REQUIRED {
            assert!(ids.contains(&required.as_str()));
        }
        assert!(!ids.contains(&"resume_paused_work"));
        let full = required_recovery_routes(true, true);
        let ids: Vec<String> = full.iter().map(|r| r.action_id.clone()).collect();
        assert!(ids.iter().any(|id| id == "resume_paused_work"));
        assert!(ids.iter().any(|id| id == "override_for_session"));
    }

    #[test]
    fn all_efficiency_states_listed() {
        assert_eq!(ALL_EFFICIENCY_STATES.len(), 5);
        assert!(ALL_EFFICIENCY_STATES.contains(&EfficiencyState::ProtectCore));
    }
}
