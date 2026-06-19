//! Progress snapshots and learning digests: durable, user-owned, privacy-bounded
//! state for the learnability flows that run across Aureline's M5 feature
//! families.
//!
//! Where [`crate::learning_mode_profiles`] owns the *dial* (how much guidance a
//! person sees) and [`crate::guided_exercise_rails`] / [`crate::tour_and_glossary_packages`]
//! own the *content*, this module owns the *memory*: a
//! [`LearningProgressSnapshot`] records how far a person got through one tour,
//! exercise rail, or glossary walkthrough — its completed and dismissed steps, a
//! resume point, an explicit device/local sync policy, export refs, and the
//! privacy disclosure that keeps it user-owned and local-first — and a
//! [`LearningDigest`] is the durable surface that exposes resume, dismiss,
//! snooze, reset, and export actions over those snapshots so progress never
//! lives only inside an ephemeral banner or toast.
//!
//! ## What a snapshot freezes
//!
//! - **Completed and dismissed steps.** Each [`StepProgressRecord`] carries a
//!   [`StepProgressState`]; a dismissed step is always reversible, so dismissing
//!   a hint never strands the rest of a flow.
//! - **A resume point.** [`ResumePoint`] names the step a person resumes at and
//!   MUST be resumable after a restart, so pausing never loses progress.
//! - **An explicit disclosure state.** [`SnapshotDisclosureState`] discloses
//!   whether a snapshot is local-only, eligible for device sync, exported, or
//!   reset — and that disclosure survives support/export review.
//! - **A device/local sync policy.** [`DeviceSyncPolicy`] defaults to local-only;
//!   cross-device sync is an allowed, *disclosed* choice and never a default.
//! - **Export refs.** [`ExportRef`]s record user-initiated exports that redact
//!   raw payloads, so a person can carry progress out without leaking workspace
//!   bodies.
//!
//! ## What a digest freezes
//!
//! - **Durable lifecycle actions.** Every [`LearningDigest`] exposes the full
//!   action set — resume, dismiss, snooze, reset, export — as command-backed,
//!   keyboard-reachable, reversible, inspectable, non-mutating [`DigestAction`]s.
//! - **Durable recovery, not a vanishing banner.** A digest asserts it replaces
//!   ephemeral banners and keeps a durable recovery path, so feature-family
//!   onboarding never depends on a toast that disappears without privacy truth.
//! - **Inspectable exposure.** A digest's state is visible in settings,
//!   Help/About, diagnostics, and support export, never hidden in a transient
//!   overlay.
//!
//! ## Invariants enforced
//!
//! - **Progress is user-owned and local-first.** A snapshot whose ownership is
//!   not user-owned local-first, that is repo-visible, that is shared with
//!   collaborators, or that grants any extension/background service
//!   telemetry-grade read access narrows below Stable and fails validation. The
//!   mere existence of a tour or exercise never widens repo or collaborator
//!   read access.
//! - **Pause never loses progress.** A snapshot that does not survive restart, or
//!   a resume point that is not resumable after restart, narrows below Stable.
//! - **Experts are never trapped.** No snapshot may force blocking onboarding.
//! - **Educational AI keeps "do" behind the fence.** A flow that uses
//!   educational AI must route any prepared "do" through the same
//!   preview/approval model as ordinary work.
//! - **Sync is honest.** A sync-eligible snapshot must disclose its sync; an
//!   undisclosed sync-eligible snapshot is a masquerade that narrows to Preview.
//!   A disclosed device-sync-eligible snapshot is an honest, user-chosen
//!   deviation that narrows to Beta.
//! - **No progress is stranded.** Every snapshot is covered by at least one
//!   durable digest, so no feature-family onboarding state lives only in an
//!   ephemeral banner.
//!
//! ## Canonical truth source
//!
//! [`seeded_m5_learning_progress_snapshots`] produces the canonical manifest.
//! Settings, Help/About, diagnostics, support export, and docs/migration surfaces
//! ingest it rather than rephrasing progress or privacy state by hand.
//!
//! - Schema: [`M5_LEARNING_PROGRESS_SCHEMA_REF`]
//! - Fixture: [`M5_LEARNING_PROGRESS_FIXTURE_REF`]
//! - Artifact: [`M5_LEARNING_PROGRESS_ARTIFACT_REF`]
//! - Doc: [`M5_LEARNING_PROGRESS_DOC_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::freeze_m5_learnability_lane::DataOwnershipClass;
use crate::m5_feature_family_learning_rails::{
    M5LearningSurfaceFamily, M5_FEATURE_FAMILY_LEARNING_SCHEMA_REF,
};
use crate::qualify_learning_mode_guided_tours_and_teaching_sessions::{
    QualificationVerdict, GUIDED_LEARNING_CONTRACTS_SCHEMA_REF,
};

#[cfg(test)]
mod tests;

// ── Schema-version and record-kind constants ─────────────────────────────────

/// Integer schema version for the learning-progress records. Bumped only on
/// breaking payload changes; additive-optional fields do not bump it.
pub const M5_LEARNING_PROGRESS_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`LearningProgressSnapshot`].
pub const LEARNING_PROGRESS_SNAPSHOT_RECORD_KIND: &str = "learning_progress_snapshot";

/// Record kind for [`LearningDigest`].
pub const LEARNING_DIGEST_RECORD_KIND: &str = "learning_digest";

/// Record kind for [`M5LearningProgressManifest`].
pub const M5_LEARNING_PROGRESS_MANIFEST_RECORD_KIND: &str = "m5_learning_progress_manifest";

// ── Canonical path constants ──────────────────────────────────────────────────

/// Repository-relative path to the learning-progress schema.
pub const M5_LEARNING_PROGRESS_SCHEMA_REF: &str =
    "schemas/help/m5-learning-progress-snapshots.schema.json";

/// Repository-relative path to the canonical manifest fixture.
pub const M5_LEARNING_PROGRESS_FIXTURE_REF: &str =
    "fixtures/help/m5/learning-progress/m5_learning_progress_snapshots.json";

/// Repository-relative path to the proof artifact.
pub const M5_LEARNING_PROGRESS_ARTIFACT_REF: &str =
    "artifacts/ux/m5/learning-progress-proof/add-progress-snapshots-and-learning-digests.md";

/// Repository-relative path to the public doc.
pub const M5_LEARNING_PROGRESS_DOC_REF: &str = "docs/m5/learning-digest-and-progress.md";

// ── Flow kind ──────────────────────────────────────────────────────────────

/// The kind of learnability flow a snapshot tracks progress through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningFlowKind {
    /// A guided tour package.
    Tour,
    /// A hands-on guided exercise rail.
    ExerciseRail,
    /// A glossary-pack walkthrough.
    GlossaryWalkthrough,
    /// A first-run onboarding checklist.
    FirstRunChecklist,
    /// A contextual-help card sequence.
    ContextualHelp,
}

impl LearningFlowKind {
    /// Stable string token for records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tour => "tour",
            Self::ExerciseRail => "exercise_rail",
            Self::GlossaryWalkthrough => "glossary_walkthrough",
            Self::FirstRunChecklist => "first_run_checklist",
            Self::ContextualHelp => "contextual_help",
        }
    }
}

// ── Step progress ────────────────────────────────────────────────────────────

/// The progress state of one step within a tracked flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepProgressState {
    /// The step has not been started.
    NotStarted,
    /// The step is in progress.
    InProgress,
    /// The step was completed.
    Completed,
    /// The step was dismissed by the user (always reversible).
    Dismissed,
    /// The step was skipped without dismissing the flow.
    Skipped,
}

impl StepProgressState {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Dismissed => "dismissed",
            Self::Skipped => "skipped",
        }
    }

    /// Returns true when the state represents progress that a reset must clear.
    pub const fn is_active_progress(self) -> bool {
        matches!(self, Self::InProgress | Self::Completed)
    }
}

/// One step's durable progress within a tracked flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepProgressRecord {
    /// Opaque stable ref to the step (matches the flow's step target ref).
    pub step_ref: String,
    /// Short, privacy-safe label for the step.
    pub step_label: String,
    /// Current progress state.
    pub state: StepProgressState,
    /// Whether a dismissal of this step can be undone. MUST be true when the
    /// state is [`StepProgressState::Dismissed`].
    pub dismissal_reversible: bool,
}

impl StepProgressRecord {
    /// Returns true when the step satisfies Stable requirements.
    pub fn qualifies_stable(&self) -> bool {
        self.state != StepProgressState::Dismissed || self.dismissal_reversible
    }
}

// ── Resume point ──────────────────────────────────────────────────────────

/// The durable point a paused flow resumes from.
///
/// A resume point MUST be resumable after a restart; that is what lets a person
/// pause a learning flow and pick it up later without losing progress.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumePoint {
    /// Opaque ref to the step the flow resumes at.
    pub step_ref: String,
    /// Opaque stable target ref the resume action navigates to.
    pub target_ref: String,
    /// Short, privacy-safe label describing where the resume lands.
    pub label: String,
    /// Whether the resume point survives a restart. MUST be true.
    pub resumable_after_restart: bool,
}

impl ResumePoint {
    /// Returns true when the resume point satisfies Stable requirements.
    pub fn qualifies_stable(&self) -> bool {
        self.resumable_after_restart
    }
}

// ── Disclosure state ──────────────────────────────────────────────────────

/// The disclosed lifecycle state of a snapshot's data.
///
/// This is the state a person (or a support reviewer) sees when they ask "where
/// does my progress live?" — and it survives support/export review unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotDisclosureState {
    /// Progress lives only on this device and never leaves.
    LocalOnly,
    /// Progress is eligible for cross-device sync (disclosed).
    SyncEligible,
    /// Progress has been exported to a portable bundle.
    Exported,
    /// Progress has been reset/cleared.
    Reset,
}

impl SnapshotDisclosureState {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::SyncEligible => "sync_eligible",
            Self::Exported => "exported",
            Self::Reset => "reset",
        }
    }
}

// ── Device/local sync policy ─────────────────────────────────────────────────

/// How a snapshot's progress moves (or does not move) across devices.
///
/// Local-only is the live-authoritative default. Cross-device sync is an allowed
/// user choice but it must be disclosed and it narrows the snapshot below Stable,
/// because synced state can lag behind another device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceSyncPolicy {
    /// Progress never leaves the device; live-authoritative default.
    LocalOnlyDefault,
    /// Progress is eligible for cross-device sync through the user's portable
    /// profile; may lag across devices.
    DeviceSyncEligibleDisclosed,
    /// Policy disables sync; the snapshot is pinned local-only.
    SyncBlockedByPolicy,
}

impl DeviceSyncPolicy {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyDefault => "local_only_default",
            Self::DeviceSyncEligibleDisclosed => "device_sync_eligible_disclosed",
            Self::SyncBlockedByPolicy => "sync_blocked_by_policy",
        }
    }

    /// Returns true when progress is eligible to leave the device via sync.
    pub const fn is_sync_eligible(self) -> bool {
        matches!(self, Self::DeviceSyncEligibleDisclosed)
    }
}

// ── Privacy disclosure ────────────────────────────────────────────────────

/// The privacy posture for a progress snapshot.
///
/// Progress is user-owned and local-first by default. No repo, collaborator, or
/// extension acquires telemetry-grade read access merely because a tour or
/// exercise exists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacyDisclosure {
    /// Whether progress is user-owned and local-first by default. MUST be true.
    pub user_owned_local_by_default: bool,
    /// Whether sharing requires an explicit user promotion. MUST be true.
    pub explicit_promotion_required_for_sharing: bool,
    /// Whether the repository can observe this progress. MUST be false.
    pub repo_visible: bool,
    /// Whether the progress is shared with collaborators. MUST be false.
    pub shared_with_collaborators: bool,
    /// Whether any extension or background service gains telemetry-grade read
    /// access to this progress. MUST be false.
    pub extension_telemetry_read_access: bool,
}

impl PrivacyDisclosure {
    /// Returns true when the posture satisfies Stable privacy requirements.
    pub fn qualifies_stable(&self) -> bool {
        self.user_owned_local_by_default
            && self.explicit_promotion_required_for_sharing
            && !self.repo_visible
            && !self.shared_with_collaborators
            && !self.extension_telemetry_read_access
    }
}

// ── Export refs ───────────────────────────────────────────────────────────

/// The kind of target a progress export is written to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportTargetKind {
    /// The user's portable profile bundle.
    PortableProfile,
    /// A support-export bundle.
    SupportBundle,
    /// A local file download.
    LocalDownload,
}

impl ExportTargetKind {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PortableProfile => "portable_profile",
            Self::SupportBundle => "support_bundle",
            Self::LocalDownload => "local_download",
        }
    }
}

/// A user-initiated export of a progress snapshot.
///
/// An export carries the snapshot's progress out of the device. It MUST redact
/// raw payloads (no credential bodies or provider payloads) and MUST be
/// user-initiated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportRef {
    /// Opaque stable id for this export.
    pub export_id: String,
    /// Where the export was written.
    pub target_kind: ExportTargetKind,
    /// Whether the export includes step-level progress.
    pub includes_step_progress: bool,
    /// Whether the export redacts raw payloads. MUST be true.
    pub redacts_raw_payloads: bool,
    /// Whether the export was user-initiated. MUST be true — no silent exports.
    pub user_initiated: bool,
}

impl ExportRef {
    /// Returns true when the export satisfies Stable requirements.
    pub fn qualifies_stable(&self) -> bool {
        self.redacts_raw_payloads && self.user_initiated
    }
}

// ── Digest actions ────────────────────────────────────────────────────────

/// The kind of lifecycle action a [`DigestAction`] performs over a snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DigestActionKind {
    /// Resume a paused flow from its resume point.
    Resume,
    /// Dismiss a flow (reversibly).
    Dismiss,
    /// Snooze a flow for a bounded period.
    Snooze,
    /// Reset a flow's progress (offers a restore).
    Reset,
    /// Export the progress snapshot.
    Export,
}

impl DigestActionKind {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resume => "resume",
            Self::Dismiss => "dismiss",
            Self::Snooze => "snooze",
            Self::Reset => "reset",
            Self::Export => "export",
        }
    }
}

/// The action kinds every digest must expose so progress can always be resumed,
/// dismissed, snoozed, reset, or exported from a durable surface.
pub const REQUIRED_DIGEST_ACTION_KINDS: [DigestActionKind; 5] = [
    DigestActionKind::Resume,
    DigestActionKind::Dismiss,
    DigestActionKind::Snooze,
    DigestActionKind::Reset,
    DigestActionKind::Export,
];

/// One command-backed action exposed by a learning digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigestAction {
    /// Opaque stable id for the action.
    pub action_id: String,
    /// What the action does.
    pub action_kind: DigestActionKind,
    /// Opaque ref to the command that backs this action.
    pub command_id_ref: String,
    /// Human-readable description of the action's effect.
    pub effect: String,
    /// Opaque ref to the keyboard shortcut; MUST be present (keyboard reachable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyboard_shortcut_ref: Option<String>,
    /// Whether the action is reversible. MUST be true — even reset offers a
    /// restore, so no action is a one-way door.
    pub reversible: bool,
    /// Whether the action surfaces in the action log / inspector. MUST be true.
    pub inspectable: bool,
    /// Whether the action may write progress state silently. MUST be false.
    pub silent_write_allowed: bool,
    /// Whether the action mutates workspace state. MUST be false — actions only
    /// touch local progress state.
    pub mutates_workspace: bool,
}

impl DigestAction {
    /// Returns true when the action satisfies every Stable requirement.
    pub fn qualifies_stable(&self) -> bool {
        self.keyboard_shortcut_ref.is_some()
            && self.reversible
            && self.inspectable
            && !self.silent_write_allowed
            && !self.mutates_workspace
    }
}

// ── Surface exposure ──────────────────────────────────────────────────────

/// Where a digest's state, actions, and recovery path are visible.
///
/// Progress state must be inspectable wherever a user or a support flow would
/// look for it — never hidden inside a transient overlay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceExposure {
    /// Visible in settings. MUST be true.
    pub in_settings: bool,
    /// Visible in Help/About. MUST be true.
    pub in_help_about: bool,
    /// Visible in diagnostics. MUST be true.
    pub in_diagnostics: bool,
    /// Visible in support export. MUST be true.
    pub in_support_export: bool,
    /// Whether the state is reachable only through a transient overlay. MUST be
    /// false.
    pub hidden_in_transient_overlay_only: bool,
}

impl SurfaceExposure {
    /// Returns true when exposure satisfies Stable requirements.
    pub fn qualifies_stable(&self) -> bool {
        self.in_settings
            && self.in_help_about
            && self.in_diagnostics
            && self.in_support_export
            && !self.hidden_in_transient_overlay_only
    }
}

// ── Snapshot ──────────────────────────────────────────────────────────────

/// One durable, user-owned progress snapshot over a single learnability flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningProgressSnapshot {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this snapshot.
    pub snapshot_id: String,
    /// Human-readable label shown in the digest and support export.
    pub display_label: String,
    /// The M5 surface family this flow belongs to.
    pub family: M5LearningSurfaceFamily,
    /// The kind of flow this snapshot tracks.
    pub flow_kind: LearningFlowKind,
    /// Opaque ref to the flow (tour/exercise/glossary) being tracked.
    pub flow_ref: String,
    /// Disclosed lifecycle state of this snapshot's data.
    pub disclosure_state: SnapshotDisclosureState,
    /// Device/local sync policy.
    pub sync_policy: DeviceSyncPolicy,
    /// Whether device sync is disclosed to the user. A sync-eligible snapshot
    /// MUST set this true.
    pub sync_disclosed: bool,
    /// Data-ownership class. MUST be user-owned local-first.
    pub data_ownership: DataOwnershipClass,
    /// Privacy disclosure.
    pub privacy: PrivacyDisclosure,
    /// Per-step progress.
    pub steps: Vec<StepProgressRecord>,
    /// Resume point, if the flow is resumable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_point: Option<ResumePoint>,
    /// User-initiated export refs.
    #[serde(default)]
    pub export_refs: Vec<ExportRef>,
    /// Whether the snapshot survives a restart. MUST be true.
    pub survives_restart: bool,
    /// Whether the snapshot is safe to include in support exports.
    pub safe_for_support_export: bool,
    /// Whether the tracked flow uses educational AI.
    pub flow_uses_educational_ai: bool,
    /// Whether educational AI routes any prepared "do" through the standard
    /// preview/approval model. MUST be true when the flow uses educational AI.
    pub educational_ai_uses_standard_preview_approval: bool,
    /// Whether the flow may force blocking onboarding. MUST be false.
    pub blocking_onboarding_allowed: bool,
    /// Whether the flow is allowed to change an authority boundary. MUST be
    /// false.
    pub authority_boundary_change_allowed: bool,
    /// Whether the command graph stays unchanged. MUST be true.
    pub command_graph_unchanged: bool,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
}

impl LearningProgressSnapshot {
    /// The number of completed steps.
    pub fn completed_step_count(&self) -> usize {
        self.steps
            .iter()
            .filter(|s| s.state == StepProgressState::Completed)
            .count()
    }

    /// The number of dismissed steps.
    pub fn dismissed_step_count(&self) -> usize {
        self.steps
            .iter()
            .filter(|s| s.state == StepProgressState::Dismissed)
            .count()
    }

    /// Returns true when any step records active (in-progress or completed)
    /// progress.
    pub fn has_active_progress(&self) -> bool {
        self.steps.iter().any(|s| s.state.is_active_progress())
    }

    /// Recomputes this snapshot's verdict and narrowing reasons, writing them
    /// back.
    pub fn sync_verdict(&mut self) {
        let (verdict, reasons) = derive_snapshot_verdict(self);
        self.verdict = verdict;
        self.narrowing_reasons = reasons;
    }
}

// ── Digest ────────────────────────────────────────────────────────────────

/// One durable learning digest: the surface that exposes resume, dismiss,
/// snooze, reset, and export actions over a set of progress snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningDigest {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this digest.
    pub digest_id: String,
    /// Human-readable label shown in settings, Help/About, and support export.
    pub display_label: String,
    /// Opaque refs to the snapshots this digest covers.
    pub covered_snapshot_refs: Vec<String>,
    /// Lifecycle actions exposed by the digest.
    pub actions: Vec<DigestAction>,
    /// Whether the digest replaces ephemeral banners with a durable surface.
    /// MUST be true.
    pub replaces_ephemeral_banners: bool,
    /// Whether a durable recovery path exists after a dismissal. MUST be true.
    pub durable_recovery_available: bool,
    /// Where the digest state is exposed.
    pub exposure: SurfaceExposure,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
}

impl LearningDigest {
    /// The set of action kinds this digest exposes.
    pub fn action_kinds(&self) -> BTreeSet<DigestActionKind> {
        self.actions.iter().map(|a| a.action_kind).collect()
    }
}

// ── Verdict derivation ────────────────────────────────────────────────────

/// Derives a snapshot's verdict and narrowing reasons from its evidence.
///
/// Hard safety violations (authority change, command-graph drift, non-user
/// ownership, a privacy leak to the repo/collaborators/an extension, blocking
/// onboarding, an unfenced educational-AI "do", state that does not survive
/// restart, a non-resumable resume point, an irreversible dismissal, an
/// inconsistent disclosure state, a raw-payload-leaking or non-user-initiated
/// export, or an undisclosed sync masquerade) narrow to
/// [`QualificationVerdict::NarrowedPreview`]. A disclosed device-sync-eligible
/// snapshot is an honest, user-chosen deviation and narrows to
/// [`QualificationVerdict::NarrowedBeta`]. With no findings the snapshot is
/// [`QualificationVerdict::QualifiedStable`].
pub fn derive_snapshot_verdict(
    snapshot: &LearningProgressSnapshot,
) -> (QualificationVerdict, Vec<String>) {
    use QualificationVerdict::*;

    let mut verdict = QualifiedStable;
    let mut reasons: Vec<String> = Vec::new();

    // ── Hard safety violations ──
    if snapshot.authority_boundary_change_allowed {
        reasons.push("authority_boundary_change_allowed".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if !snapshot.command_graph_unchanged {
        reasons.push("command_graph_changed".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if !snapshot.data_ownership.qualifies_stable() {
        reasons.push("progress_state_not_user_owned_local_first".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if snapshot.blocking_onboarding_allowed {
        reasons.push("blocking_onboarding_allowed_traps_experts".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if !snapshot.privacy.qualifies_stable() {
        if snapshot.privacy.repo_visible {
            reasons.push("progress_repo_visible".to_string());
        }
        if snapshot.privacy.shared_with_collaborators {
            reasons.push("progress_shared_with_collaborators".to_string());
        }
        if snapshot.privacy.extension_telemetry_read_access {
            reasons.push("extension_telemetry_grade_read_access".to_string());
        }
        if !snapshot.privacy.user_owned_local_by_default {
            reasons.push("progress_not_user_owned_local_by_default".to_string());
        }
        if !snapshot.privacy.explicit_promotion_required_for_sharing {
            reasons.push("sharing_without_explicit_promotion".to_string());
        }
        verdict = verdict.meet(NarrowedPreview);
    }
    if !snapshot.survives_restart {
        reasons.push("progress_does_not_survive_restart".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if snapshot.flow_uses_educational_ai && !snapshot.educational_ai_uses_standard_preview_approval
    {
        reasons.push("educational_ai_do_outside_standard_preview_approval".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    for step in &snapshot.steps {
        if !step.qualifies_stable() {
            reasons.push(format!("dismissed_step_{}_not_reversible", step.step_ref));
            verdict = verdict.meet(NarrowedPreview);
        }
    }
    if let Some(resume) = &snapshot.resume_point {
        if !resume.qualifies_stable() {
            reasons.push("resume_point_not_resumable_after_restart".to_string());
            verdict = verdict.meet(NarrowedPreview);
        }
    }
    if snapshot.sync_policy.is_sync_eligible() && !snapshot.sync_disclosed {
        reasons.push("sync_eligible_but_not_disclosed_masquerade".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    // Disclosure-state consistency.
    match snapshot.disclosure_state {
        SnapshotDisclosureState::SyncEligible => {
            if !snapshot.sync_policy.is_sync_eligible() || !snapshot.sync_disclosed {
                reasons.push("sync_eligible_state_without_disclosed_sync_policy".to_string());
                verdict = verdict.meet(NarrowedPreview);
            }
        }
        SnapshotDisclosureState::Exported => {
            if snapshot.export_refs.is_empty() {
                reasons.push("exported_state_without_export_ref".to_string());
                verdict = verdict.meet(NarrowedPreview);
            }
        }
        SnapshotDisclosureState::Reset => {
            if snapshot.resume_point.is_some() || snapshot.has_active_progress() {
                reasons.push("reset_state_retains_progress".to_string());
                verdict = verdict.meet(NarrowedPreview);
            }
        }
        SnapshotDisclosureState::LocalOnly => {}
    }
    for export in &snapshot.export_refs {
        if !export.qualifies_stable() {
            reasons.push(format!(
                "export_{}_leaks_raw_payload_or_not_user_initiated",
                export.export_id
            ));
            verdict = verdict.meet(NarrowedPreview);
        }
    }

    // ── Disclosed, honest narrowing ──
    if snapshot.sync_policy.is_sync_eligible() && snapshot.sync_disclosed {
        reasons.push("device_sync_eligible_state_may_lag_disclosed".to_string());
        verdict = verdict.meet(NarrowedBeta);
    }

    reasons.sort();
    reasons.dedup();
    (verdict, reasons)
}

/// Derives a digest's verdict and narrowing reasons from its own evidence and
/// the snapshots it covers.
///
/// A digest narrows to [`QualificationVerdict::NarrowedPreview`] when an action
/// is not command-backed/keyboard-reachable/reversible/inspectable/non-mutating,
/// a required action is missing, it relies on ephemeral banners, it lacks a
/// durable recovery path, its state is hidden from inspectable surfaces, or it
/// covers an unknown or empty snapshot set. It also folds in the narrowest
/// verdict of the snapshots it covers, so a digest can never present a covered
/// snapshot as healthier than the snapshot itself.
pub fn derive_digest_verdict(
    digest: &LearningDigest,
    covered: &[&LearningProgressSnapshot],
) -> (QualificationVerdict, Vec<String>) {
    use QualificationVerdict::*;

    let mut verdict = QualifiedStable;
    let mut reasons: Vec<String> = Vec::new();

    for action in &digest.actions {
        if !action.qualifies_stable() {
            reasons.push(format!(
                "action_{}_not_inspectable_keyboard_reversible_non_mutating",
                action.action_kind.as_str()
            ));
            verdict = verdict.meet(NarrowedPreview);
        }
    }
    let kinds = digest.action_kinds();
    for required in REQUIRED_DIGEST_ACTION_KINDS {
        if !kinds.contains(&required) {
            reasons.push(format!("missing_{}_action", required.as_str()));
            verdict = verdict.meet(NarrowedPreview);
        }
    }
    if !digest.replaces_ephemeral_banners {
        reasons.push("digest_relies_on_ephemeral_banners".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if !digest.durable_recovery_available {
        reasons.push("digest_lacks_durable_recovery".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if !digest.exposure.qualifies_stable() {
        reasons.push("digest_state_hidden_from_inspectable_surfaces".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }
    if digest.covered_snapshot_refs.is_empty() {
        reasons.push("digest_covers_no_snapshot".to_string());
        verdict = verdict.meet(NarrowedPreview);
    }

    // Fold in the covered snapshots' verdicts so the digest never over-states
    // the health of the progress it surfaces.
    for snapshot in covered {
        if snapshot.verdict != QualifiedStable {
            reasons.push(format!(
                "covered_snapshot_{}_is_{}",
                snapshot.snapshot_id,
                snapshot.verdict.as_str()
            ));
            verdict = verdict.meet(snapshot.verdict);
        }
    }

    reasons.sort();
    reasons.dedup();
    (verdict, reasons)
}

// ── Manifest ──────────────────────────────────────────────────────────────

/// The canonical manifest binding every progress snapshot and learning digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningProgressManifest {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this manifest.
    pub manifest_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Schema, docs, and contract refs this manifest consumes.
    pub contract_refs: BTreeMap<String, String>,
    /// Progress snapshots.
    pub snapshots: Vec<LearningProgressSnapshot>,
    /// Durable learning digests over the snapshots.
    pub digests: Vec<LearningDigest>,
    /// Overall derived verdict — the strictest verdict across snapshots and
    /// digests.
    pub overall_verdict: QualificationVerdict,
    /// Named narrowing reasons aggregated across records (empty when
    /// overall_verdict is QualifiedStable).
    #[serde(default)]
    pub overall_narrowing_reasons: Vec<String>,
}

impl M5LearningProgressManifest {
    /// Recomputes every snapshot and digest verdict and the overall verdict from
    /// current evidence, writing them back.
    ///
    /// Snapshots are synced first so digests fold in up-to-date covered-snapshot
    /// verdicts.
    pub fn sync_verdicts(&mut self) {
        for snapshot in &mut self.snapshots {
            snapshot.sync_verdict();
        }

        // Snapshot the synced records so digests can borrow them immutably while
        // we mutate the digest vector.
        let snapshots = self.snapshots.clone();
        let lookup = |id: &str| snapshots.iter().find(|s| s.snapshot_id == id);

        let mut overall = QualificationVerdict::QualifiedStable;
        let mut reasons: Vec<String> = Vec::new();

        for snapshot in &self.snapshots {
            overall = overall.meet(snapshot.verdict);
            reasons.extend(snapshot.narrowing_reasons.iter().cloned());
        }

        for digest in &mut self.digests {
            let covered: Vec<&LearningProgressSnapshot> = digest
                .covered_snapshot_refs
                .iter()
                .filter_map(|r| lookup(r))
                .collect();
            let (verdict, dreasons) = derive_digest_verdict(digest, &covered);
            digest.verdict = verdict;
            digest.narrowing_reasons = dreasons;
            overall = overall.meet(digest.verdict);
            reasons.extend(digest.narrowing_reasons.iter().cloned());
        }

        reasons.sort();
        reasons.dedup();
        self.overall_verdict = overall;
        self.overall_narrowing_reasons = reasons;
    }

    /// Returns the snapshot with `snapshot_id`, if present.
    pub fn snapshot(&self, snapshot_id: &str) -> Option<&LearningProgressSnapshot> {
        self.snapshots.iter().find(|s| s.snapshot_id == snapshot_id)
    }

    /// Returns the digest with `digest_id`, if present.
    pub fn digest(&self, digest_id: &str) -> Option<&LearningDigest> {
        self.digests.iter().find(|d| d.digest_id == digest_id)
    }

    /// The set of every snapshot id the manifest defines.
    pub fn known_snapshot_ids(&self) -> BTreeSet<String> {
        self.snapshots
            .iter()
            .map(|s| s.snapshot_id.clone())
            .collect()
    }
}

/// Reopens a progress manifest from its exported JSON form.
///
/// This is the round-trip used to prove progress survives export and reopen
/// without losing step, resume-point, disclosure, or digest identity: the
/// reopened manifest is structurally equal to the original.
///
/// # Errors
///
/// Returns the underlying [`serde_json::Error`] when `json` is not a valid
/// serialized manifest.
pub fn reopen_progress_manifest_from_json(
    json: &str,
) -> Result<M5LearningProgressManifest, serde_json::Error> {
    serde_json::from_str(json)
}

// ── Validation ──────────────────────────────────────────────────────────────

/// A typed validation error from [`validate_m5_learning_progress_snapshots`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearningProgressValidationError {
    /// Opaque id of the snapshot, digest, or manifest that failed.
    pub subject_id: String,
    /// Human-readable description of the failure.
    pub message: String,
}

impl std::fmt::Display for LearningProgressValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.subject_id, self.message)
    }
}

/// Validates a manifest against the learning-progress invariants.
///
/// # Errors
///
/// Returns a non-empty `Vec` when any record's stored verdict diverges from the
/// verdict derived from its evidence; when a snapshot would change an authority
/// boundary, drift the command graph, store progress outside user-owned
/// local-first ownership, make progress repo-visible, share it with
/// collaborators, grant an extension telemetry-grade read access, force blocking
/// onboarding, fail to survive restart, prepare an educational-AI "do" outside
/// the standard preview/approval model, keep an irreversible dismissal, carry a
/// non-resumable resume point, sync without disclosure, disagree with its own
/// disclosure state, or carry a raw-payload-leaking or non-user-initiated export;
/// when a digest omits a required action, exposes a non-inspectable action,
/// relies on ephemeral banners, lacks durable recovery, hides its state, covers
/// an unknown or empty snapshot set; when a snapshot is not covered by any
/// digest; when two snapshots or two digests share an id; or when the manifest's
/// overall verdict does not fold its members.
pub fn validate_m5_learning_progress_snapshots(
    manifest: &M5LearningProgressManifest,
) -> Result<(), Vec<LearningProgressValidationError>> {
    let mut errors: Vec<LearningProgressValidationError> = Vec::new();

    // ── Snapshots ──
    let mut seen_snapshot_ids: BTreeSet<&str> = BTreeSet::new();
    for snapshot in &manifest.snapshots {
        let subject = snapshot.snapshot_id.clone();
        let err = |message: String| LearningProgressValidationError {
            subject_id: subject.clone(),
            message,
        };

        if !seen_snapshot_ids.insert(snapshot.snapshot_id.as_str()) {
            errors.push(err(format!(
                "duplicate snapshot id {}",
                snapshot.snapshot_id
            )));
        }

        let (derived, derived_reasons) = derive_snapshot_verdict(snapshot);
        if derived != snapshot.verdict {
            errors.push(err(format!(
                "stored verdict {} disagrees with derived verdict {}",
                snapshot.verdict.as_str(),
                derived.as_str()
            )));
        }
        if derived_reasons != snapshot.narrowing_reasons {
            errors.push(err(
                "stored narrowing reasons disagree with derived reasons".to_string(),
            ));
        }

        if snapshot.authority_boundary_change_allowed {
            errors.push(err("snapshot changes an authority boundary".to_string()));
        }
        if !snapshot.command_graph_unchanged {
            errors.push(err("snapshot changes the command graph".to_string()));
        }
        if !snapshot.data_ownership.qualifies_stable() {
            errors.push(err("progress is not user-owned local-first".to_string()));
        }
        if snapshot.privacy.repo_visible {
            errors.push(err("progress is repo-visible".to_string()));
        }
        if snapshot.privacy.shared_with_collaborators {
            errors.push(err("progress is shared with collaborators".to_string()));
        }
        if snapshot.privacy.extension_telemetry_read_access {
            errors.push(err(
                "an extension gains telemetry-grade read access to progress".to_string(),
            ));
        }
        if !snapshot.privacy.user_owned_local_by_default {
            errors.push(err(
                "progress is not user-owned local by default".to_string()
            ));
        }
        if !snapshot.privacy.explicit_promotion_required_for_sharing {
            errors.push(err(
                "progress can be shared without explicit promotion".to_string()
            ));
        }
        if snapshot.blocking_onboarding_allowed {
            errors.push(err("snapshot forces blocking onboarding".to_string()));
        }
        if !snapshot.survives_restart {
            errors.push(err("progress does not survive restart".to_string()));
        }
        if snapshot.flow_uses_educational_ai
            && !snapshot.educational_ai_uses_standard_preview_approval
        {
            errors.push(err(
                "educational AI prepares a do outside the standard preview/approval model"
                    .to_string(),
            ));
        }
        if snapshot.steps.is_empty() {
            errors.push(err("snapshot has no steps".to_string()));
        }
        for step in &snapshot.steps {
            if step.state == StepProgressState::Dismissed && !step.dismissal_reversible {
                errors.push(err(format!(
                    "dismissed step {} is not reversible",
                    step.step_ref
                )));
            }
        }
        if let Some(resume) = &snapshot.resume_point {
            if !resume.resumable_after_restart {
                errors.push(err(
                    "resume point is not resumable after restart".to_string()
                ));
            }
        }
        if snapshot.sync_policy.is_sync_eligible() && !snapshot.sync_disclosed {
            errors.push(err(
                "sync-eligible snapshot does not disclose sync (masquerade)".to_string(),
            ));
        }
        match snapshot.disclosure_state {
            SnapshotDisclosureState::SyncEligible => {
                if !snapshot.sync_policy.is_sync_eligible() || !snapshot.sync_disclosed {
                    errors.push(err(
                        "sync_eligible disclosure state without a disclosed sync-eligible policy"
                            .to_string(),
                    ));
                }
            }
            SnapshotDisclosureState::Exported => {
                if snapshot.export_refs.is_empty() {
                    errors.push(err(
                        "exported disclosure state without an export ref".to_string()
                    ));
                }
            }
            SnapshotDisclosureState::Reset => {
                if snapshot.resume_point.is_some() || snapshot.has_active_progress() {
                    errors.push(err("reset disclosure state retains progress".to_string()));
                }
            }
            SnapshotDisclosureState::LocalOnly => {}
        }
        for export in &snapshot.export_refs {
            if !export.redacts_raw_payloads {
                errors.push(err(format!(
                    "export {} does not redact raw payloads",
                    export.export_id
                )));
            }
            if !export.user_initiated {
                errors.push(err(format!(
                    "export {} is not user-initiated (silent export)",
                    export.export_id
                )));
            }
        }
    }

    let known_snapshot_ids = manifest.known_snapshot_ids();

    // ── Digests ──
    let mut seen_digest_ids: BTreeSet<&str> = BTreeSet::new();
    let mut covered_snapshot_ids: BTreeSet<String> = BTreeSet::new();
    for digest in &manifest.digests {
        let subject = digest.digest_id.clone();
        let err = |message: String| LearningProgressValidationError {
            subject_id: subject.clone(),
            message,
        };

        if !seen_digest_ids.insert(digest.digest_id.as_str()) {
            errors.push(err(format!("duplicate digest id {}", digest.digest_id)));
        }

        let covered: Vec<&LearningProgressSnapshot> = digest
            .covered_snapshot_refs
            .iter()
            .filter_map(|r| manifest.snapshot(r))
            .collect();
        let (derived, derived_reasons) = derive_digest_verdict(digest, &covered);
        if derived != digest.verdict {
            errors.push(err(format!(
                "stored verdict {} disagrees with derived verdict {}",
                digest.verdict.as_str(),
                derived.as_str()
            )));
        }
        if derived_reasons != digest.narrowing_reasons {
            errors.push(err(
                "stored narrowing reasons disagree with derived reasons".to_string(),
            ));
        }

        if digest.covered_snapshot_refs.is_empty() {
            errors.push(err("digest covers no snapshot".to_string()));
        }
        for snapshot_ref in &digest.covered_snapshot_refs {
            if !known_snapshot_ids.contains(snapshot_ref) {
                errors.push(err(format!(
                    "digest covers unknown snapshot {snapshot_ref}"
                )));
            } else {
                covered_snapshot_ids.insert(snapshot_ref.clone());
            }
        }

        for action in &digest.actions {
            if action.mutates_workspace {
                errors.push(err(format!(
                    "action {} mutates workspace state",
                    action.action_kind.as_str()
                )));
            }
            if action.silent_write_allowed {
                errors.push(err(format!(
                    "action {} permits a silent write",
                    action.action_kind.as_str()
                )));
            }
            if action.keyboard_shortcut_ref.is_none() {
                errors.push(err(format!(
                    "action {} is not keyboard reachable",
                    action.action_kind.as_str()
                )));
            }
            if !action.reversible {
                errors.push(err(format!(
                    "action {} is not reversible",
                    action.action_kind.as_str()
                )));
            }
            if !action.inspectable {
                errors.push(err(format!(
                    "action {} is not inspectable",
                    action.action_kind.as_str()
                )));
            }
        }
        let kinds = digest.action_kinds();
        for required in REQUIRED_DIGEST_ACTION_KINDS {
            if !kinds.contains(&required) {
                errors.push(err(format!(
                    "digest is missing the {} action",
                    required.as_str()
                )));
            }
        }
        if !digest.replaces_ephemeral_banners {
            errors.push(err(
                "digest relies on ephemeral banners instead of a durable surface".to_string(),
            ));
        }
        if !digest.durable_recovery_available {
            errors.push(err("digest lacks a durable recovery path".to_string()));
        }
        if !digest.exposure.qualifies_stable() {
            errors.push(err(
                "digest state is hidden from settings/help/diagnostics/support".to_string(),
            ));
        }
    }

    // Every snapshot must be covered by a durable digest — no progress may live
    // only inside an ephemeral banner.
    for snapshot in &manifest.snapshots {
        if !covered_snapshot_ids.contains(&snapshot.snapshot_id) {
            errors.push(LearningProgressValidationError {
                subject_id: snapshot.snapshot_id.clone(),
                message: "snapshot is not covered by any durable digest".to_string(),
            });
        }
    }

    // ── Manifest-level: overall verdict must fold the members ──
    let mut expected_overall = QualificationVerdict::QualifiedStable;
    for snapshot in &manifest.snapshots {
        expected_overall = expected_overall.meet(snapshot.verdict);
    }
    for digest in &manifest.digests {
        expected_overall = expected_overall.meet(digest.verdict);
    }
    if expected_overall != manifest.overall_verdict {
        errors.push(LearningProgressValidationError {
            subject_id: manifest.manifest_id.clone(),
            message: format!(
                "manifest overall verdict {} disagrees with folded member verdict {}",
                manifest.overall_verdict.as_str(),
                expected_overall.as_str()
            ),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// ── Seed builders ─────────────────────────────────────────────────────────

/// Builds the standard, fully-inspectable digest action set.
fn standard_actions(token: &str) -> Vec<DigestAction> {
    let spec = [
        (
            DigestActionKind::Resume,
            "cmd:learning.progress.resume",
            "resume the flow at its resume point",
            "kb:learning.progress.resume",
        ),
        (
            DigestActionKind::Dismiss,
            "cmd:learning.progress.dismiss",
            "dismiss the flow (reversibly)",
            "kb:learning.progress.dismiss",
        ),
        (
            DigestActionKind::Snooze,
            "cmd:learning.progress.snooze",
            "snooze the flow for a bounded period",
            "kb:learning.progress.snooze",
        ),
        (
            DigestActionKind::Reset,
            "cmd:learning.progress.reset",
            "reset progress (offers a restore)",
            "kb:learning.progress.reset",
        ),
        (
            DigestActionKind::Export,
            "cmd:learning.progress.export",
            "export the progress snapshot",
            "kb:learning.progress.export",
        ),
    ];
    spec.iter()
        .map(|(kind, command, effect, shortcut)| DigestAction {
            action_id: format!("learning:m5:digest:{token}:action:{}", kind.as_str()),
            action_kind: *kind,
            command_id_ref: (*command).to_string(),
            effect: (*effect).to_string(),
            keyboard_shortcut_ref: Some((*shortcut).to_string()),
            reversible: true,
            inspectable: true,
            silent_write_allowed: false,
            mutates_workspace: false,
        })
        .collect()
}

/// A private, user-owned, repo-invisible privacy disclosure — the default.
fn private_local_disclosure() -> PrivacyDisclosure {
    PrivacyDisclosure {
        user_owned_local_by_default: true,
        explicit_promotion_required_for_sharing: true,
        repo_visible: false,
        shared_with_collaborators: false,
        extension_telemetry_read_access: false,
    }
}

/// Specification for one seeded snapshot, expanded by [`build_snapshot`].
struct SnapshotSpec {
    token: &'static str,
    display_label: &'static str,
    family: M5LearningSurfaceFamily,
    flow_kind: LearningFlowKind,
    disclosure_state: SnapshotDisclosureState,
    sync_policy: DeviceSyncPolicy,
    flow_uses_educational_ai: bool,
    steps: Vec<(&'static str, &'static str, StepProgressState)>,
    resume_at: Option<&'static str>,
    exported: bool,
}

/// Expands a [`SnapshotSpec`] into a full [`LearningProgressSnapshot`].
fn build_snapshot(spec: SnapshotSpec) -> LearningProgressSnapshot {
    let sync_eligible = spec.sync_policy.is_sync_eligible();
    let steps: Vec<StepProgressRecord> = spec
        .steps
        .iter()
        .map(|(step_token, label, state)| StepProgressRecord {
            step_ref: format!("step:{}:{step_token}", spec.token),
            step_label: (*label).to_string(),
            state: *state,
            dismissal_reversible: true,
        })
        .collect();

    let resume_point = spec.resume_at.map(|step_token| ResumePoint {
        step_ref: format!("step:{}:{step_token}", spec.token),
        target_ref: format!("target:{}:{step_token}", spec.token),
        label: format!("Resume {} at {step_token}", spec.display_label),
        resumable_after_restart: true,
    });

    let export_refs = if spec.exported {
        vec![ExportRef {
            export_id: format!("learning:m5:export:{}", spec.token),
            target_kind: ExportTargetKind::PortableProfile,
            includes_step_progress: true,
            redacts_raw_payloads: true,
            user_initiated: true,
        }]
    } else {
        Vec::new()
    };

    let mut snapshot = LearningProgressSnapshot {
        record_kind: LEARNING_PROGRESS_SNAPSHOT_RECORD_KIND.to_string(),
        schema_version: M5_LEARNING_PROGRESS_SCHEMA_VERSION,
        snapshot_id: format!("learning:m5:progress:{}", spec.token),
        display_label: spec.display_label.to_string(),
        family: spec.family,
        flow_kind: spec.flow_kind,
        flow_ref: format!("flow:{}:{}", spec.flow_kind.as_str(), spec.token),
        disclosure_state: spec.disclosure_state,
        sync_policy: spec.sync_policy,
        sync_disclosed: sync_eligible,
        data_ownership: DataOwnershipClass::UserOwnedLocalFirst,
        privacy: private_local_disclosure(),
        steps,
        resume_point,
        export_refs,
        survives_restart: true,
        safe_for_support_export: true,
        flow_uses_educational_ai: spec.flow_uses_educational_ai,
        educational_ai_uses_standard_preview_approval: true,
        blocking_onboarding_allowed: false,
        authority_boundary_change_allowed: false,
        command_graph_unchanged: true,
        verdict: QualificationVerdict::QualifiedStable,
        narrowing_reasons: Vec::new(),
    };
    snapshot.sync_verdict();
    snapshot
}

/// Builds a digest covering the given snapshot ids.
fn build_digest(
    token: &str,
    display_label: &str,
    covered_snapshot_refs: Vec<String>,
) -> LearningDigest {
    LearningDigest {
        record_kind: LEARNING_DIGEST_RECORD_KIND.to_string(),
        schema_version: M5_LEARNING_PROGRESS_SCHEMA_VERSION,
        digest_id: format!("learning:m5:digest:{token}"),
        display_label: display_label.to_string(),
        covered_snapshot_refs,
        actions: standard_actions(token),
        replaces_ephemeral_banners: true,
        durable_recovery_available: true,
        exposure: SurfaceExposure {
            in_settings: true,
            in_help_about: true,
            in_diagnostics: true,
            in_support_export: true,
            hidden_in_transient_overlay_only: false,
        },
        verdict: QualificationVerdict::QualifiedStable,
        narrowing_reasons: Vec::new(),
    }
}

/// Produces the canonical seeded learning-progress manifest.
///
/// Four snapshots span four feature families: a notebook tour (in progress, with
/// a reset hint dismissed), a request-workspace exercise rail (resumable, partly
/// complete), a docs/browser glossary walkthrough that has been exported, and a
/// database-workspace tour that is device-sync-eligible (disclosed). Two digests
/// cover them — one for the local/exported snapshots (Stable) and one for the
/// sync-eligible snapshot ([`QualificationVerdict::NarrowedBeta`]) — so the
/// overall manifest verdict is `narrowed_beta`.
pub fn seeded_m5_learning_progress_snapshots() -> M5LearningProgressManifest {
    let snapshots = vec![
        build_snapshot(SnapshotSpec {
            token: "notebook_intro_tour",
            display_label: "Notebook intro tour",
            family: M5LearningSurfaceFamily::Notebook,
            flow_kind: LearningFlowKind::Tour,
            disclosure_state: SnapshotDisclosureState::LocalOnly,
            sync_policy: DeviceSyncPolicy::LocalOnlyDefault,
            flow_uses_educational_ai: true,
            steps: vec![
                (
                    "open_notebook",
                    "Open a notebook",
                    StepProgressState::Completed,
                ),
                ("run_cell", "Run a cell", StepProgressState::InProgress),
                (
                    "explain_output",
                    "Ask the AI to explain output",
                    StepProgressState::Dismissed,
                ),
                (
                    "save_session",
                    "Save the session",
                    StepProgressState::NotStarted,
                ),
            ],
            resume_at: Some("run_cell"),
            exported: false,
        }),
        build_snapshot(SnapshotSpec {
            token: "request_workspace_first_call",
            display_label: "Request workspace: first call",
            family: M5LearningSurfaceFamily::RequestWorkspace,
            flow_kind: LearningFlowKind::ExerciseRail,
            disclosure_state: SnapshotDisclosureState::LocalOnly,
            sync_policy: DeviceSyncPolicy::SyncBlockedByPolicy,
            flow_uses_educational_ai: false,
            steps: vec![
                (
                    "build_request",
                    "Build a request",
                    StepProgressState::Completed,
                ),
                (
                    "send_request",
                    "Send the request",
                    StepProgressState::Completed,
                ),
                (
                    "inspect_response",
                    "Inspect the response",
                    StepProgressState::InProgress,
                ),
            ],
            resume_at: Some("inspect_response"),
            exported: false,
        }),
        build_snapshot(SnapshotSpec {
            token: "docs_browser_glossary",
            display_label: "Docs glossary walkthrough",
            family: M5LearningSurfaceFamily::DocsBrowser,
            flow_kind: LearningFlowKind::GlossaryWalkthrough,
            disclosure_state: SnapshotDisclosureState::Exported,
            sync_policy: DeviceSyncPolicy::LocalOnlyDefault,
            flow_uses_educational_ai: false,
            steps: vec![
                (
                    "trust_model",
                    "Read: trust model",
                    StepProgressState::Completed,
                ),
                (
                    "provenance",
                    "Read: provenance",
                    StepProgressState::Completed,
                ),
            ],
            resume_at: None,
            exported: true,
        }),
        build_snapshot(SnapshotSpec {
            token: "database_workspace_tour",
            display_label: "Database workspace tour",
            family: M5LearningSurfaceFamily::DatabaseWorkspace,
            flow_kind: LearningFlowKind::Tour,
            disclosure_state: SnapshotDisclosureState::SyncEligible,
            sync_policy: DeviceSyncPolicy::DeviceSyncEligibleDisclosed,
            flow_uses_educational_ai: true,
            steps: vec![
                (
                    "connect",
                    "Connect to a database",
                    StepProgressState::Completed,
                ),
                (
                    "statement_safety",
                    "Review statement safety",
                    StepProgressState::InProgress,
                ),
                (
                    "read_grid",
                    "Read the result grid",
                    StepProgressState::NotStarted,
                ),
            ],
            resume_at: Some("statement_safety"),
            exported: false,
        }),
    ];

    let local_refs: Vec<String> = snapshots
        .iter()
        .filter(|s| s.sync_policy != DeviceSyncPolicy::DeviceSyncEligibleDisclosed)
        .map(|s| s.snapshot_id.clone())
        .collect();
    let synced_refs: Vec<String> = snapshots
        .iter()
        .filter(|s| s.sync_policy == DeviceSyncPolicy::DeviceSyncEligibleDisclosed)
        .map(|s| s.snapshot_id.clone())
        .collect();

    let digests = vec![
        build_digest(
            "local_progress",
            "Learning progress (this device)",
            local_refs,
        ),
        build_digest("synced_progress", "Learning progress (synced)", synced_refs),
    ];

    let mut contract_refs = BTreeMap::new();
    contract_refs.insert(
        "schema".to_string(),
        M5_LEARNING_PROGRESS_SCHEMA_REF.to_string(),
    );
    contract_refs.insert("doc".to_string(), M5_LEARNING_PROGRESS_DOC_REF.to_string());
    contract_refs.insert(
        "artifact".to_string(),
        M5_LEARNING_PROGRESS_ARTIFACT_REF.to_string(),
    );
    contract_refs.insert(
        "feature_family_schema".to_string(),
        M5_FEATURE_FAMILY_LEARNING_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "guided_learning_contracts_schema".to_string(),
        GUIDED_LEARNING_CONTRACTS_SCHEMA_REF.to_string(),
    );

    let mut manifest = M5LearningProgressManifest {
        record_kind: M5_LEARNING_PROGRESS_MANIFEST_RECORD_KIND.to_string(),
        schema_version: M5_LEARNING_PROGRESS_SCHEMA_VERSION,
        manifest_id: "learning:m5:progress_manifest:v1".to_string(),
        generated_at: "2026-06-19T00:00:00Z".to_string(),
        contract_refs,
        snapshots,
        digests,
        overall_verdict: QualificationVerdict::QualifiedStable,
        overall_narrowing_reasons: Vec::new(),
    };
    manifest.sync_verdicts();
    manifest
}
