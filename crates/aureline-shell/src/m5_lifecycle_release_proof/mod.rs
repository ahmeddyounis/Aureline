//! Release-grade lifecycle-state, checkpoint, and recovery-affordance truth proof for every claimed
//! M5 object family, profile, and exported truth surface.
//!
//! The [frozen lifecycle matrix][matrix] already binds each long-lived M5 object family to an
//! explicit state machine, one visible primary status surface, one exportable status code, one
//! controlled last-failure reason, one named recovery affordance, and an ordered inventory of
//! milestone checkpoints. This lane is the **release-evidence capstone** that certifies, for every one
//! of those thirteen object families, that its lifecycle-state truth, checkpoint truth, and
//! recovery-affordance truth hold across **every claimed M5 desktop profile** and survive **every
//! exported truth surface** — UI, CLI/headless, docs/help, diagnostics, support exports, telemetry, and
//! claim publication — so a family that still collapses state into generic loading or error behavior is
//! automatically narrowed or blocked from stable promotion rather than shipping an over-claim.
//!
//! For every object family the lane certifies four things the acceptance criteria and implementation
//! requirements demand:
//!
//! - the object keeps its **explicit lifecycle-state truth** rather than collapsing state into a
//!   generic loading or error behavior ([`LifecycleStateTruthState`]);
//! - the object keeps its **named milestone checkpoint truth** rather than collapsing its journey into
//!   an anonymous spinner ([`CheckpointTruthState`]);
//! - the object keeps its **named recovery affordance and controlled last-failure reason truth**
//!   rather than dropping the affordance or reason ([`RecoveryAffordanceTruthState`]);
//! - and every **exported truth surface reflects the same current proof** — claim publication,
//!   docs/help, diagnostics, and support exports agree rather than drifting or going stale
//!   ([`ExportedProofParityState`]).
//!
//! Three records carry the truth:
//!
//! - the per-family **certification row** ([`LifecycleReleaseProofRow`]): one row per
//!   [`M5LifecycleObjectFamily`] naming the claimed desktop profiles it certifies the truth across
//!   (drawn from the [`M5DesktopProfile`] vocabulary), the truth pillars it keeps (drawn from the
//!   [`M5LifecycleTruthPillar`] vocabulary), the frozen primary status surface, status-code export
//!   field, and last-failure-reason field it emits, its lifecycle-state / checkpoint /
//!   recovery-affordance / exported-proof-parity posture, whether the same state-truth vocabulary
//!   survives headless/companion-adjacent execution, the consumer surfaces it evaluated, any active
//!   waiver, and a derived green/yellow/red [`LifecycleReleaseProofStatus`].
//! - the release **certification packet** ([`LifecycleReleaseProofPacket`]): the full set of rows with
//!   derived per-row status, aggregate green/yellow/red counts, the active waivers, the exact
//!   conformance causes ([`LifecycleReleaseProofCause`]), and the blocking findings the lane refuses
//!   to ship with.
//! - the **certification dashboard** ([`LifecycleReleaseProofDashboard`]): a light projection the
//!   Shiproom / Support Center / product UI / CLI / diagnostics / claim-publication automation reads to
//!   auto-narrow a family's release-proof claim when its certification falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment an
//! object discloses a reduced lifecycle-state truth, discloses compacted checkpoint truth, keeps a
//! disclosed, waivered reduced recovery-affordance truth, or discloses a partial export refresh; it
//! drops to `red` if an object collapses its state into generic loading/error behavior, collapses its
//! checkpoints into an anonymous spinner, loses its recovery affordance or last-failure reason, lets an
//! exported truth surface go stale or divergent, loses the same state-truth vocabulary in a
//! headless/companion-adjacent execution, fails to certify across all six claimed desktop profiles,
//! fails to keep all three truth pillars, or fails to certify every consumer surface the matrix
//! declares for the family. That derivation is the auto-narrowing the acceptance criteria require, and
//! the consumer-surface, profile, and truth-pillar completeness checks are the conformance lints that
//! gate stable promotion when a family diverges from the controlled state vocabulary or leaves a
//! claimed profile or truth pillar uncertified.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local
//! paths, raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed vocabulary,
//! counts, refs, and short labels. The object-family, checkpoint, state, recovery-affordance,
//! last-failure-reason, primary-status-surface, consumer-surface, downgrade-trigger, journey, and
//! qualification vocabulary is re-exported by reference from the already frozen [matrix], the claimed
//! desktop-profile vocabulary is re-exported from the desktop-profile certification lane, and every
//! family's driving journey, explicit state machine, primary status surface, status-code export field,
//! last-failure-reason field, recovery affordance, checkpoint lineage, and applicable triggers are
//! pulled straight from that matrix's seeded packet, so this lane mints no parallel lifecycle
//! vocabulary and cannot certify a family the matrix does not anchor. Only the release-proof-specific
//! vocabulary ([`M5LifecycleTruthPillar`], [`M5LifecycleProofDimension`],
//! [`LifecycleReleaseProofStatus`], [`LifecycleStateTruthState`], [`CheckpointTruthState`],
//! [`RecoveryAffordanceTruthState`], [`ExportedProofParityState`], [`LifecycleReleaseProofWaiver`],
//! [`LifecycleReleaseProofCause`], [`LifecycleReleaseProofFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix as matrix;

pub use crate::m5_desktop_profile_certification::M5DesktopProfile;
pub use matrix::{
    M5CriticalJourney, M5JourneyCheckpoint, M5LastFailureReasonClass, M5LifecycleConsumerSurface,
    M5LifecycleDowngradeTrigger, M5LifecycleObjectFamily, M5LifecycleQualificationClass,
    M5LifecycleState, M5PrimaryStatusSurface, M5RecoveryAffordance,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_lifecycle_release_proof_packet,
    seeded_m5_lifecycle_release_proof_packet_ai_exported_proof_stale_blocked,
    seeded_m5_lifecycle_release_proof_packet_data_recovery_truth_missing_blocked,
    seeded_m5_lifecycle_release_proof_packet_extension_headless_parity_lost_blocked,
    seeded_m5_lifecycle_release_proof_packet_notebook_state_collapsed_blocked,
    seeded_m5_lifecycle_release_proof_packet_remote_checkpoints_collapsed_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_LIFECYCLE_RELEASE_PROOF_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_LIFECYCLE_RELEASE_PROOF_SHARED_CONTRACT_REF: &str =
    "lifecycle:m5_lifecycle_release_proof:v1";

/// Stable record kind for [`LifecycleReleaseProofPacket`] payloads.
pub const M5_LIFECYCLE_RELEASE_PROOF_PACKET_RECORD_KIND: &str =
    "lifecycle_m5_lifecycle_release_proof_packet_record";

/// Stable record kind for [`LifecycleReleaseProofDashboard`] payloads.
pub const M5_LIFECYCLE_RELEASE_PROOF_DASHBOARD_RECORD_KIND: &str =
    "lifecycle_m5_lifecycle_release_proof_dashboard_record";

/// Stable record kind for [`LifecycleReleaseProofSupportExport`] payloads.
pub const M5_LIFECYCLE_RELEASE_PROOF_SUPPORT_EXPORT_RECORD_KIND: &str =
    "lifecycle_m5_lifecycle_release_proof_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_LIFECYCLE_RELEASE_PROOF_PACKET_ID: &str = "m5-lifecycle-release-proof:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_LIFECYCLE_RELEASE_PROOF_DASHBOARD_ID: &str =
    "m5-lifecycle-release-proof-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_LIFECYCLE_RELEASE_PROOF_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-lifecycle-release-proof:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_LIFECYCLE_RELEASE_PROOF_SOURCE_SCHEMA_REF: &str =
    "schemas/lifecycle/m5-lifecycle-release-proof.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_LIFECYCLE_RELEASE_PROOF_PUBLISHED_REPORT_REF: &str =
    "artifacts/lifecycle/m5-lifecycle-release-proof.md";

/// Published certification-packet artifact ref.
pub const M5_LIFECYCLE_RELEASE_PROOF_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-lifecycle-release-proof-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_LIFECYCLE_RELEASE_PROOF_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-lifecycle-release-proof-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_LIFECYCLE_RELEASE_PROOF_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-lifecycle-release-proof-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_LIFECYCLE_RELEASE_PROOF_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-lifecycle-release-proof-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_LIFECYCLE_RELEASE_PROOF_PUBLISHED_DOC_REF: &str =
    "docs/lifecycle/m5_lifecycle_release_proof_contract.md";

/// Repo-relative ref to the frozen lifecycle object-state schema.
pub const M5_LIFECYCLE_RELEASE_PROOF_OBJECT_STATE_SCHEMA_REF: &str =
    matrix::M5_LIFECYCLE_OBJECT_STATE_SCHEMA_REF;

/// Repo-relative ref to the frozen lifecycle journey-checkpoint schema.
pub const M5_LIFECYCLE_RELEASE_PROOF_JOURNEY_CHECKPOINT_SCHEMA_REF: &str =
    matrix::M5_LIFECYCLE_JOURNEY_CHECKPOINT_SCHEMA_REF;

/// Frozen lifecycle-matrix contract doc this proof mirrors.
pub const M5_LIFECYCLE_RELEASE_PROOF_MATRIX_DOC_REF: &str = matrix::M5_LIFECYCLE_MATRIX_DOC_REF;

/// State-object inventory this proof mirrors for the driving object families.
pub const M5_LIFECYCLE_RELEASE_PROOF_STATE_OBJECT_INVENTORY_REF: &str =
    matrix::M5_LIFECYCLE_STATE_OBJECT_INVENTORY_REF;

/// State-class recovery reference this proof mirrors for the recovery-affordance truth binding.
pub const M5_LIFECYCLE_RELEASE_PROOF_STATE_CLASS_RECOVERY_REF: &str =
    matrix::M5_LIFECYCLE_STATE_CLASS_RECOVERY_REF;

/// Every object family the certification must cover, in canonical order. A certification that covers
/// fewer regresses into a partial view and blocks.
pub const REQUIRED_OBJECT_FAMILIES: [M5LifecycleObjectFamily; 13] = M5LifecycleObjectFamily::ALL;

/// Every proof dimension each family row certifies, in canonical order.
pub const REQUIRED_PROOF_DIMENSIONS: [M5LifecycleProofDimension; 4] =
    M5LifecycleProofDimension::ALL;

/// Every claimed M5 desktop profile each family row must certify its truth across, in canonical order.
pub const REQUIRED_PROFILES: [M5DesktopProfile; 6] = M5DesktopProfile::ALL;

/// Every truth pillar each family row must keep, in canonical order.
pub const REQUIRED_TRUTH_PILLARS: [M5LifecycleTruthPillar; 3] = M5LifecycleTruthPillar::ALL;

/// One of the three lifecycle truth pillars the release proof requires every family to keep — the
/// exact truths the spec names: lifecycle-state truth, checkpoint truth, and recovery-affordance
/// truth. A row that drops one blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleTruthPillar {
    /// Explicit lifecycle-state truth.
    LifecycleState,
    /// Named milestone checkpoint truth.
    Checkpoint,
    /// Named recovery affordance and controlled last-failure reason truth.
    RecoveryAffordance,
}

impl M5LifecycleTruthPillar {
    /// Every truth pillar, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::LifecycleState,
        Self::Checkpoint,
        Self::RecoveryAffordance,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LifecycleState => "lifecycle_state",
            Self::Checkpoint => "checkpoint",
            Self::RecoveryAffordance => "recovery_affordance",
        }
    }
}

/// One of the four proof dimensions each object-family row certifies.
///
/// These are exactly the four ways the acceptance criteria and implementation requirements demand a
/// claimed M5 object keep its state truth honest across every claimed profile and exported surface: it
/// keeps its explicit lifecycle-state truth; it keeps its named milestone checkpoint truth; it keeps
/// its named recovery affordance and last-failure reason; and every exported truth surface reflects the
/// same current proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleProofDimension {
    /// Explicit lifecycle-state truth, not collapsed into generic loading/error behavior.
    LifecycleStateTruth,
    /// Named milestone checkpoint truth, not collapsed into an anonymous spinner.
    CheckpointTruth,
    /// Named recovery affordance and controlled last-failure reason truth.
    RecoveryAffordanceTruth,
    /// Every exported truth surface reflects the same current proof.
    ExportedProofParity,
}

impl M5LifecycleProofDimension {
    /// Every proof dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LifecycleStateTruth,
        Self::CheckpointTruth,
        Self::RecoveryAffordanceTruth,
        Self::ExportedProofParity,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LifecycleStateTruth => "lifecycle_state_truth",
            Self::CheckpointTruth => "checkpoint_truth",
            Self::RecoveryAffordanceTruth => "recovery_affordance_truth",
            Self::ExportedProofParity => "exported_proof_parity",
        }
    }
}

/// The derived release-proof certification light an object family carries.
///
/// `green` means the object keeps its explicit lifecycle-state truth, its named checkpoint truth, and
/// its named recovery-affordance and last-failure-reason truth across all six claimed desktop profiles
/// and every exported truth surface — and with the same state-truth vocabulary surviving a
/// headless/companion-adjacent execution. `yellow` is a disclosed narrowing (a disclosed reduced
/// lifecycle-state truth, disclosed compacted checkpoint truth, a waivered reduced recovery-affordance
/// truth, or a disclosed partial export refresh). `red` is blocked: state collapsed into generic
/// loading/error behavior, checkpoints collapsed into an anonymous spinner, recovery affordance or
/// reason missing, an exported truth surface stale or divergent, a headless/companion-adjacent
/// vocabulary loss, an incomplete profile or truth-pillar set, or a row that did not certify every
/// declared consumer surface — and it may not keep a release-proof claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleReleaseProofStatus {
    /// Full standing: all four proof dimensions hold and headless parity is preserved.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl LifecycleReleaseProofStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// `true` when the row keeps a publishable (green or yellow) claim.
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Green | Self::Yellow)
    }
}

/// How the object keeps its explicit lifecycle-state truth.
///
/// `explicit_state_truth_certified` means the object exposes its explicit state machine's controlled
/// state across every claimed profile and exported surface rather than showing a generic spinner or
/// error. `disclosed_reduced_state_truth` means the object exposes a disclosed reduced state truth on a
/// constrained build — for example collapsing a handful of intermediate states into one disclosed
/// grouped state while still naming the terminal controlled state (a yellow narrowing).
/// `state_collapsed_into_generic_loading_or_error` means the object hid its controlled state behind a
/// generic loading or error behavior, so the state is no longer diagnosable from the controlled
/// vocabulary — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleStateTruthState {
    /// Explicit lifecycle-state truth is certified across every profile and surface.
    ExplicitStateTruthCertified,
    /// The object exposes a disclosed reduced state truth.
    DisclosedReducedStateTruth,
    /// The object collapsed its state into generic loading or error behavior — a blocker.
    StateCollapsedIntoGenericLoadingOrError,
}

impl LifecycleStateTruthState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitStateTruthCertified => "explicit_state_truth_certified",
            Self::DisclosedReducedStateTruth => "disclosed_reduced_state_truth",
            Self::StateCollapsedIntoGenericLoadingOrError => {
                "state_collapsed_into_generic_loading_or_error"
            }
        }
    }

    /// `true` when explicit state truth is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::ExplicitStateTruthCertified)
    }

    /// `true` when the object took a disclosed reduced-state-truth narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedStateTruth)
    }
}

/// How the object keeps its named milestone checkpoint truth.
///
/// `named_checkpoint_truth_certified` means the object shows its ordered named milestone checkpoints
/// across every claimed profile and exported surface rather than one anonymous spinner.
/// `disclosed_compacted_checkpoint_truth` means the object shows a disclosed compacted checkpoint
/// sequence on a constrained build — for example folding two adjacent milestones into one disclosed
/// compacted milestone while still naming each terminal checkpoint (a yellow narrowing).
/// `checkpoints_collapsed_to_anonymous_spinner` means the object collapsed its milestone checkpoints
/// into a single anonymous spinner with no named boundaries — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointTruthState {
    /// Named milestone checkpoint truth is certified.
    NamedCheckpointTruthCertified,
    /// The object shows a disclosed compacted checkpoint sequence.
    DisclosedCompactedCheckpointTruth,
    /// The object collapsed its checkpoints into an anonymous spinner — a blocker.
    CheckpointsCollapsedToAnonymousSpinner,
}

impl CheckpointTruthState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NamedCheckpointTruthCertified => "named_checkpoint_truth_certified",
            Self::DisclosedCompactedCheckpointTruth => "disclosed_compacted_checkpoint_truth",
            Self::CheckpointsCollapsedToAnonymousSpinner => {
                "checkpoints_collapsed_to_anonymous_spinner"
            }
        }
    }

    /// `true` when checkpoint truth is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::NamedCheckpointTruthCertified)
    }

    /// `true` when the object took a disclosed compacted-checkpoint narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedCompactedCheckpointTruth)
    }
}

/// How the object keeps its named recovery affordance and controlled last-failure reason truth.
///
/// `named_recovery_and_reason_certified` means every degraded, failed, or recoverable state exposes the
/// one named recovery affordance and the controlled last-failure reason the matrix binds for the
/// family. `disclosed_reduced_recovery_truth` means the object exposes a disclosed reduced recovery
/// truth on a constrained surface — for example deferring the recovery affordance to a linked action
/// while still naming the last-failure reason (a yellow narrowing that reduces the recovery truth, so it
/// **requires an active waiver**). `recovery_or_reason_truth_missing` means the object dropped the named
/// recovery affordance or the controlled last-failure reason, so a failed state cannot be recovered from
/// or diagnosed — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryAffordanceTruthState {
    /// The named recovery affordance and last-failure reason are certified.
    NamedRecoveryAndReasonCertified,
    /// The object exposes a disclosed reduced recovery truth.
    DisclosedReducedRecoveryTruth,
    /// The object dropped the recovery affordance or the last-failure reason — a blocker.
    RecoveryOrReasonTruthMissing,
}

impl RecoveryAffordanceTruthState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NamedRecoveryAndReasonCertified => "named_recovery_and_reason_certified",
            Self::DisclosedReducedRecoveryTruth => "disclosed_reduced_recovery_truth",
            Self::RecoveryOrReasonTruthMissing => "recovery_or_reason_truth_missing",
        }
    }

    /// `true` when recovery-affordance truth is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::NamedRecoveryAndReasonCertified)
    }

    /// `true` when the object took a disclosed reduced-recovery-truth narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedRecoveryTruth)
    }
}

/// How every exported truth surface reflects the current proof.
///
/// `exported_surfaces_reflect_current_proof` means claim publication, docs/help, diagnostics, and
/// support exports all reflect the same current lifecycle proof. `disclosed_partial_export_refresh`
/// means one exported surface takes a disclosed partial refresh cadence on a legacy surface — for
/// example a legacy diagnostics export refreshing on a slower cadence while still disclosing the lag
/// (a yellow narrowing). `exported_proof_stale_or_divergent` means an exported surface reflects a stale
/// or divergent proof, so claim publication or a support export overclaims relative to the current
/// truth — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportedProofParityState {
    /// Every exported truth surface reflects the same current proof.
    ExportedSurfacesReflectCurrentProof,
    /// One exported surface takes a disclosed partial refresh.
    DisclosedPartialExportRefresh,
    /// An exported surface reflects a stale or divergent proof — a blocker.
    ExportedProofStaleOrDivergent,
}

impl ExportedProofParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportedSurfacesReflectCurrentProof => "exported_surfaces_reflect_current_proof",
            Self::DisclosedPartialExportRefresh => "disclosed_partial_export_refresh",
            Self::ExportedProofStaleOrDivergent => "exported_proof_stale_or_divergent",
        }
    }

    /// `true` when every exported surface reflects the current proof at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::ExportedSurfacesReflectCurrentProof)
    }

    /// `true` when the object took a disclosed partial-export-refresh narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialExportRefresh)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed (yellow) rather
/// than blocked — never lets collapsed state, an anonymous spinner, a missing recovery affordance, or a
/// stale exported proof hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleReleaseProofWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The object family the waiver applies to.
    pub object_family: M5LifecycleObjectFamily,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl LifecycleReleaseProofWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked an object family's release-proof certification.
///
/// The trigger token mirrors the frozen [`M5LifecycleDowngradeTrigger`] vocabulary so a cause never
/// mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleReleaseProofCause {
    /// The object family the cause applies to.
    pub object_family: M5LifecycleObjectFamily,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5LifecycleDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed cause is a
    /// blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl LifecycleReleaseProofCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One object family, certified across its lifecycle-state-truth, checkpoint-truth,
/// recovery-affordance-truth, and exported-proof-parity dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleReleaseProofRow {
    /// The object family being certified.
    pub object_family: M5LifecycleObjectFamily,
    /// Short reviewer-facing family label.
    pub object_label: String,
    /// The frozen matrix journey this family drives. Pulled from the matrix.
    pub matrix_journey: M5CriticalJourney,
    /// Qualification class the matrix earned for the object.
    pub qualification: M5LifecycleQualificationClass,
    /// Owner role accountable for keeping this family's lifecycle truth governed. Pulled from the
    /// matrix.
    pub owner_role: String,
    /// Short conformance scope summary.
    pub scope_summary: String,
    /// The controlled states the object's explicit state machine admits. Pulled from the matrix.
    pub admitted_states: Vec<M5LifecycleState>,
    /// The one visible primary status surface the state truth is shown on. Pulled from the matrix.
    pub primary_status_surface: M5PrimaryStatusSurface,
    /// The one exportable status-code field. Pulled from the matrix.
    pub status_code_export_field: String,
    /// The one last-failure-reason field. Pulled from the matrix.
    pub last_failure_reason_field: String,
    /// The one named recovery affordance the recovery-truth pillar anchors on. Pulled from the matrix.
    pub recovery_affordance: M5RecoveryAffordance,
    /// Controlled last-failure reason classes this family reports. Pulled from the matrix.
    pub last_failure_reason_classes: Vec<M5LastFailureReasonClass>,
    /// The ordered milestone checkpoints the checkpoint truth is shown over. Pulled from the matrix
    /// journey row.
    pub checkpoint_lineage: Vec<M5JourneyCheckpoint>,
    /// The claimed desktop profiles this row certifies its truth across (must be all six).
    pub certified_profiles: Vec<M5DesktopProfile>,
    /// The truth pillars this row keeps (must be all three).
    pub certified_truth_pillars: Vec<M5LifecycleTruthPillar>,
    /// Consumer surfaces the matrix declares the object must project to.
    pub required_consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
    /// Consumer surfaces this certification evaluated. Pulled from the matrix.
    pub evaluated_consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
    /// Lifecycle-state-truth posture.
    pub lifecycle_state_truth: LifecycleStateTruthState,
    /// Checkpoint-truth posture.
    pub checkpoint_truth: CheckpointTruthState,
    /// Recovery-affordance-truth posture.
    pub recovery_affordance_truth: RecoveryAffordanceTruthState,
    /// Exported-proof-parity posture.
    pub exported_proof_parity: ExportedProofParityState,
    /// `true` when the same state-truth vocabulary survives a headless or companion-adjacent
    /// execution; a hard invariant.
    pub headless_parity_preserved: bool,
    /// Downgrade triggers that apply to the object. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5LifecycleDowngradeTrigger>,
    /// Active waiver, when a disclosed reduced-recovery-truth narrowing is in force.
    pub active_waiver: Option<LifecycleReleaseProofWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: LifecycleReleaseProofStatus,
    /// The exact conformance causes that narrowed or blocked this row.
    pub conformance_causes: Vec<LifecycleReleaseProofCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl LifecycleReleaseProofRow {
    /// `true` when the row certified every consumer surface the matrix declares for the object — no
    /// declared surface is left uncertified and none is invented.
    pub fn consumer_surfaces_complete(&self) -> bool {
        let mut evaluated: Vec<&str> = self
            .evaluated_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        let mut required: Vec<&str> = self
            .required_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        evaluated.sort_unstable();
        required.sort_unstable();
        !required.is_empty() && evaluated == required
    }

    /// `true` when the row certifies its truth across every one of the six claimed desktop profiles —
    /// the structural proof that lifecycle truth holds across all claimed M5 profiles.
    pub fn profiles_complete(&self) -> bool {
        let mut certified: Vec<&str> = self
            .certified_profiles
            .iter()
            .map(|profile| profile.as_str())
            .collect();
        let mut required: Vec<&str> = REQUIRED_PROFILES
            .iter()
            .map(|profile| profile.as_str())
            .collect();
        certified.sort_unstable();
        certified.dedup();
        required.sort_unstable();
        certified == required
    }

    /// `true` when the row keeps every one of the three truth pillars — the structural proof that the
    /// certification fails when a row drops lifecycle-state, checkpoint, or recovery-affordance truth.
    pub fn truth_pillars_complete(&self) -> bool {
        let mut kept: Vec<&str> = self
            .certified_truth_pillars
            .iter()
            .map(|pillar| pillar.as_str())
            .collect();
        let mut required: Vec<&str> = REQUIRED_TRUTH_PILLARS
            .iter()
            .map(|pillar| pillar.as_str())
            .collect();
        kept.sort_unstable();
        kept.dedup();
        required.sort_unstable();
        kept == required
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.consumer_surfaces_complete() {
            return true;
        }
        if !self.profiles_complete() {
            return true;
        }
        if !self.truth_pillars_complete() {
            return true;
        }
        if !self.headless_parity_preserved {
            return true;
        }
        if matches!(
            self.lifecycle_state_truth,
            LifecycleStateTruthState::StateCollapsedIntoGenericLoadingOrError
        ) {
            return true;
        }
        if matches!(
            self.checkpoint_truth,
            CheckpointTruthState::CheckpointsCollapsedToAnonymousSpinner
        ) {
            return true;
        }
        if matches!(
            self.recovery_affordance_truth,
            RecoveryAffordanceTruthState::RecoveryOrReasonTruthMissing
        ) {
            return true;
        }
        if matches!(
            self.exported_proof_parity,
            ExportedProofParityState::ExportedProofStaleOrDivergent
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.lifecycle_state_truth.is_disclosed_narrowing()
            || self.checkpoint_truth.is_disclosed_narrowing()
            || self.recovery_affordance_truth.is_disclosed_narrowing()
            || self.exported_proof_parity.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the release-proof posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> LifecycleReleaseProofStatus {
        if self.has_hard_blocker() {
            LifecycleReleaseProofStatus::Red
        } else if self.has_narrowing() {
            LifecycleReleaseProofStatus::Yellow
        } else {
            LifecycleReleaseProofStatus::Green
        }
    }

    /// Recomputes the exact conformance causes for the row, in deterministic order (lifecycle-state
    /// truth, checkpoint truth, recovery-affordance truth, exported-proof parity, then structural
    /// completeness and headless parity).
    pub fn recompute_causes(&self) -> Vec<LifecycleReleaseProofCause> {
        let mut causes = Vec::new();
        match self.lifecycle_state_truth {
            LifecycleStateTruthState::ExplicitStateTruthCertified => {}
            LifecycleStateTruthState::DisclosedReducedStateTruth => {
                causes.push(LifecycleReleaseProofCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "On a constrained build the object exposes a disclosed reduced \
                             lifecycle-state truth — a handful of intermediate states are grouped into \
                             one disclosed grouped state while the terminal controlled state is still \
                             named — so the state truth is narrowed and disclosed rather than collapsed \
                             into a generic loading or error behavior."
                        .to_owned(),
                });
            }
            LifecycleStateTruthState::StateCollapsedIntoGenericLoadingOrError => {
                causes.push(LifecycleReleaseProofCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                    disclosed: false,
                    detail: "The object hid its controlled lifecycle state behind a generic loading or \
                             error behavior, so the state is no longer diagnosable from the controlled \
                             state vocabulary on any claimed profile or exported surface."
                        .to_owned(),
                });
            }
        }
        match self.checkpoint_truth {
            CheckpointTruthState::NamedCheckpointTruthCertified => {}
            CheckpointTruthState::DisclosedCompactedCheckpointTruth => {
                causes.push(LifecycleReleaseProofCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "On a long-running journey the object shows a disclosed compacted \
                             checkpoint sequence — two adjacent milestones are folded into one \
                             disclosed compacted milestone while each terminal checkpoint is still \
                             named — so the checkpoint truth is narrowed and disclosed rather than \
                             collapsed into an anonymous spinner."
                        .to_owned(),
                });
            }
            CheckpointTruthState::CheckpointsCollapsedToAnonymousSpinner => {
                causes.push(LifecycleReleaseProofCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::AnonymousCheckpoint,
                    disclosed: false,
                    detail: "The object collapsed its ordered milestone checkpoints into a single \
                             anonymous spinner with no named boundaries, so the journey shows no \
                             attributable checkpoint the user or support can name."
                        .to_owned(),
                });
            }
        }
        match self.recovery_affordance_truth {
            RecoveryAffordanceTruthState::NamedRecoveryAndReasonCertified => {}
            RecoveryAffordanceTruthState::DisclosedReducedRecoveryTruth => {
                causes.push(LifecycleReleaseProofCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "On a constrained surface the object exposes a disclosed, waivered reduced \
                             recovery truth — the named recovery affordance is deferred to a linked \
                             action while the controlled last-failure reason is still named — so the \
                             recovery truth is narrowed and disclosed rather than dropped."
                        .to_owned(),
                });
            }
            RecoveryAffordanceTruthState::RecoveryOrReasonTruthMissing => {
                causes.push(LifecycleReleaseProofCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::RecoveryAffordanceMissing,
                    disclosed: false,
                    detail: "The object dropped the named recovery affordance or the controlled \
                             last-failure reason for a degraded or failed state, so the state cannot be \
                             recovered from or diagnosed from the controlled vocabulary."
                        .to_owned(),
                });
            }
        }
        match self.exported_proof_parity {
            ExportedProofParityState::ExportedSurfacesReflectCurrentProof => {}
            ExportedProofParityState::DisclosedPartialExportRefresh => {
                causes.push(LifecycleReleaseProofCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "One exported truth surface takes a disclosed partial refresh cadence on a \
                             legacy surface — a legacy diagnostics export refreshes on a slower cadence \
                             while still disclosing the lag — so the exported parity is narrowed and \
                             disclosed rather than stale or divergent."
                        .to_owned(),
                });
            }
            ExportedProofParityState::ExportedProofStaleOrDivergent => {
                causes.push(LifecycleReleaseProofCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::ProofStale,
                    disclosed: false,
                    detail: "An exported truth surface — claim publication, docs/help, diagnostics, or \
                             a support export — reflects a stale or divergent proof, so the published \
                             claim overclaims relative to the current lifecycle truth."
                        .to_owned(),
                });
            }
        }
        if !self.profiles_complete() {
            causes.push(LifecycleReleaseProofCause {
                object_family: self.object_family,
                trigger: M5LifecycleDowngradeTrigger::ProofStale,
                disclosed: false,
                detail:
                    "The object does not certify its lifecycle, checkpoint, and recovery truth \
                         across all six claimed desktop profiles, so the release proof leaves a \
                         claimed profile uncertified."
                        .to_owned(),
            });
        }
        if !self.truth_pillars_complete() {
            causes.push(LifecycleReleaseProofCause {
                object_family: self.object_family,
                trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                disclosed: false,
                detail:
                    "The object does not keep all three truth pillars — lifecycle-state truth, \
                         checkpoint truth, and recovery-affordance truth — so the certification is \
                         missing a required pillar."
                        .to_owned(),
            });
        }
        if !self.headless_parity_preserved {
            causes.push(LifecycleReleaseProofCause {
                object_family: self.object_family,
                trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                disclosed: false,
                detail: "A headless or companion-adjacent execution of this object lost the shared \
                         state-truth vocabulary, so the same object reports a different lifecycle and \
                         checkpoint language depending on how it runs."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed reduced recovery truth may only stay yellow (rather than red) when a waiver
    /// discloses it — reducing the recovery affordance a failed state exposes is the sensitive
    /// narrowing.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.recovery_affordance_truth,
            RecoveryAffordanceTruthState::DisclosedReducedRecoveryTruth
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<LifecycleReleaseProofFinding> {
        let mut findings = Vec::new();
        let family = self.object_family.as_str().to_owned();

        if !self.consumer_surfaces_complete() {
            findings.push(LifecycleReleaseProofFinding::ConsumerSurfacesIncomplete {
                family: family.clone(),
            });
        }
        if !self.profiles_complete() {
            findings.push(LifecycleReleaseProofFinding::ProfilesIncomplete {
                family: family.clone(),
            });
        }
        if !self.truth_pillars_complete() {
            findings.push(LifecycleReleaseProofFinding::TruthPillarsIncomplete {
                family: family.clone(),
            });
        }
        if !self.headless_parity_preserved {
            findings.push(LifecycleReleaseProofFinding::HeadlessParityLost {
                family: family.clone(),
            });
        }
        if matches!(
            self.lifecycle_state_truth,
            LifecycleStateTruthState::StateCollapsedIntoGenericLoadingOrError
        ) {
            findings.push(LifecycleReleaseProofFinding::StateTruthCollapsed {
                family: family.clone(),
            });
        }
        if matches!(
            self.checkpoint_truth,
            CheckpointTruthState::CheckpointsCollapsedToAnonymousSpinner
        ) {
            findings.push(LifecycleReleaseProofFinding::CheckpointTruthCollapsed {
                family: family.clone(),
            });
        }
        if matches!(
            self.recovery_affordance_truth,
            RecoveryAffordanceTruthState::RecoveryOrReasonTruthMissing
        ) {
            findings.push(LifecycleReleaseProofFinding::RecoveryTruthMissing {
                family: family.clone(),
            });
        }
        if matches!(
            self.exported_proof_parity,
            ExportedProofParityState::ExportedProofStaleOrDivergent
        ) {
            findings.push(LifecycleReleaseProofFinding::ExportedProofStale {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, LifecycleReleaseProofStatus::Green) && !self.has_reason() {
            findings.push(LifecycleReleaseProofFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(LifecycleReleaseProofFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.object_family != self.object_family {
                findings.push(LifecycleReleaseProofFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(LifecycleReleaseProofFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(LifecycleReleaseProofFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.conformance_causes != self.recompute_causes() {
            findings.push(LifecycleReleaseProofFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} state={} checkpoint={} recovery={} export={} headless={} profiles={} pillars={} surfaces={} waiver={}",
            self.object_family.as_str(),
            self.derived_status.as_str(),
            self.lifecycle_state_truth.as_str(),
            self.checkpoint_truth.as_str(),
            self.recovery_affordance_truth.as_str(),
            self.exported_proof_parity.as_str(),
            self.headless_parity_preserved,
            self.certified_profiles.len(),
            self.certified_truth_pillars.len(),
            self.evaluated_consumer_surfaces.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the release-proof certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum LifecycleReleaseProofFinding {
    /// An object family has no certification row.
    ObjectFamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A row did not certify every declared consumer surface.
    ConsumerSurfacesIncomplete {
        /// The family token.
        family: String,
    },
    /// A row does not certify its truth across all six claimed desktop profiles.
    ProfilesIncomplete {
        /// The family token.
        family: String,
    },
    /// A row does not keep all three truth pillars.
    TruthPillarsIncomplete {
        /// The family token.
        family: String,
    },
    /// A headless/companion-adjacent execution lost the shared state-truth vocabulary.
    HeadlessParityLost {
        /// The family token.
        family: String,
    },
    /// The object collapsed its state into generic loading or error behavior.
    StateTruthCollapsed {
        /// The family token.
        family: String,
    },
    /// The object collapsed its checkpoints into an anonymous spinner.
    CheckpointTruthCollapsed {
        /// The family token.
        family: String,
    },
    /// The object dropped the recovery affordance or the last-failure reason.
    RecoveryTruthMissing {
        /// The family token.
        family: String,
    },
    /// An exported truth surface reflects a stale or divergent proof.
    ExportedProofStale {
        /// The family token.
        family: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The family token.
        family: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The family token.
        family: String,
    },
    /// An attached waiver does not point at the row's family.
    WaiverFamilyMismatch {
        /// The family token.
        family: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The family token.
        family: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The family token.
        family: String,
    },
    /// The declared conformance causes do not match the recomputed causes.
    RowCausesStale {
        /// The family token.
        family: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl LifecycleReleaseProofFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::ObjectFamilyMissing { .. } => "object_family_missing",
            Self::ConsumerSurfacesIncomplete { .. } => "consumer_surfaces_incomplete",
            Self::ProfilesIncomplete { .. } => "profiles_incomplete",
            Self::TruthPillarsIncomplete { .. } => "truth_pillars_incomplete",
            Self::HeadlessParityLost { .. } => "headless_parity_lost",
            Self::StateTruthCollapsed { .. } => "state_truth_collapsed",
            Self::CheckpointTruthCollapsed { .. } => "checkpoint_truth_collapsed",
            Self::RecoveryTruthMissing { .. } => "recovery_truth_missing",
            Self::ExportedProofStale { .. } => "exported_proof_stale",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverFamilyMismatch { .. } => "waiver_family_mismatch",
            Self::WaiverExpired { .. } => "waiver_expired",
            Self::RowStatusStale { .. } => "row_status_stale",
            Self::RowCausesStale { .. } => "row_causes_stale",
            Self::StatusCountsStale => "status_counts_stale",
            Self::CoverageStale => "coverage_stale",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::ObjectFamilyMissing { family }
            | Self::ConsumerSurfacesIncomplete { family }
            | Self::ProfilesIncomplete { family }
            | Self::TruthPillarsIncomplete { family }
            | Self::HeadlessParityLost { family }
            | Self::StateTruthCollapsed { family }
            | Self::CheckpointTruthCollapsed { family }
            | Self::RecoveryTruthMissing { family }
            | Self::ExportedProofStale { family }
            | Self::NarrowedRowWithoutReason { family }
            | Self::NarrowedRowWithoutWaiver { family }
            | Self::WaiverFamilyMismatch { family, .. }
            | Self::WaiverExpired { family, .. }
            | Self::RowStatusStale { family }
            | Self::RowCausesStale { family } => family,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release-proof certification packet shared by the Shiproom / Support Center / product UI / CLI /
/// diagnostics / claim-publication automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleReleaseProofPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the packet.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable packet id used to pivot across surfaces.
    pub packet_id: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// The frozen lifecycle matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen lifecycle object-state schema.
    pub object_state_schema_ref: String,
    /// Repo-relative ref to the frozen lifecycle journey-checkpoint schema.
    pub journey_checkpoint_schema_ref: String,
    /// Frozen lifecycle-matrix contract doc this proof mirrors.
    pub matrix_doc_ref: String,
    /// State-object inventory this proof mirrors for the driving object families.
    pub state_object_inventory_ref: String,
    /// State-class recovery reference this proof mirrors for the recovery-affordance truth binding.
    pub state_class_recovery_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four proof dimensions every family row certifies.
    pub required_proof_dimensions: Vec<String>,
    /// The six claimed desktop profiles every family row must certify its truth across.
    pub required_profiles: Vec<String>,
    /// The three truth pillars every family row must keep.
    pub required_truth_pillars: Vec<String>,
    /// The thirteen object families the certification must cover.
    pub required_object_families: Vec<String>,
    /// Per-family certification rows, in canonical order.
    pub rows: Vec<LifecycleReleaseProofRow>,
    /// Object families certified, in canonical (sorted) order.
    pub covered_object_families: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (full-conformance) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked — the stable-promotion gate.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<LifecycleReleaseProofWaiver>,
    /// Every exact conformance cause, in row then cause order.
    pub conformance_causes: Vec<LifecycleReleaseProofCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<LifecycleReleaseProofFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Lifecycle / release automation refs that consume this packet to auto-narrow object families.
    pub lifecycle_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published certification-packet ref.
    pub published_packet_ref: String,
    /// Published certification-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl LifecycleReleaseProofPacket {
    /// Returns the certification row for `family`, if present.
    pub fn row(&self, family: M5LifecycleObjectFamily) -> Option<&LifecycleReleaseProofRow> {
        self.rows.iter().find(|row| row.object_family == family)
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "packet: id={}, rows={}, green={}, yellow={}, red={}, clean={}",
                self.packet_id,
                self.row_count,
                self.green_row_count,
                self.yellow_row_count,
                self.red_row_count,
                self.report_clean,
            ),
            format!(
                "matrix={} build={} channel={} publishable={}",
                self.matrix_packet_ref,
                self.build_identity_ref,
                self.release_channel_class,
                self.all_rows_publishable,
            ),
        ];
        for row in &self.rows {
            lines.push(row.compact_line());
        }
        for waiver in &self.active_waivers {
            lines.push(format!(
                "  waiver {} -> {} (expires {})",
                waiver.waiver_id,
                waiver.object_family.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.conformance_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.object_family.as_str(),
                cause.cause_token(),
                cause.disclosed
            ));
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "  blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref()
            ));
        }
        lines
    }

    /// Projects the light certification dashboard the lifecycle automation consumes.
    pub fn dashboard(&self) -> LifecycleReleaseProofDashboard {
        LifecycleReleaseProofDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 lifecycle-release-proof packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per object family naming its status,
    /// the four truth postures, headless parity, the profile and truth-pillar counts, the
    /// evaluated-surface count, and the waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object_family,status,lifecycle_state_truth,checkpoint_truth,recovery_affordance_truth,exported_proof_parity,headless_parity,profiles,truth_pillars,evaluated_surfaces,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.object_family.as_str(),
                row.derived_status.as_str(),
                row.lifecycle_state_truth.as_str(),
                row.checkpoint_truth.as_str(),
                row.recovery_affordance_truth.as_str(),
                row.exported_proof_parity.as_str(),
                row.headless_parity_preserved,
                row.certified_profiles.len(),
                row.certified_truth_pillars.len(),
                row.evaluated_consumer_surfaces.len(),
                row.active_waiver
                    .as_ref()
                    .map(|w| w.waiver_id.as_str())
                    .unwrap_or("none"),
            ));
        }
        out
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 lifecycle release proof: lifecycle-state, checkpoint, and recovery-affordance truth across every claimed M5 profile and exported truth surface\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_lifecycle_release_proof`](../../crates/aureline-shell/src/m5_lifecycle_release_proof/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_release_proof -- markdown > \\\n  artifacts/lifecycle/m5-lifecycle-release-proof.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Certifies matrix packet: `{}`\n",
            self.matrix_packet_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!(
            "- Release channel: `{}`\n",
            self.release_channel_class
        ));
        out.push_str(&format!(
            "- Required proof dimensions: {}\n",
            self.required_proof_dimensions
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Claimed profiles certified: {}\n",
            self.required_profiles
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Object families certified: {}\n",
            self.row_count
        ));
        out.push_str(&format!(
            "- Green (full conformance): {}\n",
            self.green_row_count
        ));
        out.push_str(&format!(
            "- Yellow (auto-narrowed): {}\n",
            self.yellow_row_count
        ));
        out.push_str(&format!("- Red (blocked): {}\n", self.red_row_count));
        out.push_str(&format!(
            "- All rows publishable (stable-promotion gate): `{}`\n",
            self.all_rows_publishable
        ));
        out.push_str(&format!(
            "- Blocking findings: {}\n",
            self.blocking_findings.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Certification rows\n\n");
        out.push_str(
            "| Object family | Status | Lifecycle-state truth | Checkpoint truth | Recovery truth | Exported proof | Headless | Waiver |\n\
             | ------------- | ------ | --------------------- | ---------------- | -------------- | -------------- | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.object_label,
                row.derived_status.as_str(),
                row.lifecycle_state_truth.as_str(),
                row.checkpoint_truth.as_str(),
                row.recovery_affordance_truth.as_str(),
                row.exported_proof_parity.as_str(),
                row.headless_parity_preserved,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&LifecycleReleaseProofRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, LifecycleReleaseProofStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every claimed M5 object keeps its explicit lifecycle-state truth, its named milestone checkpoint truth, and its named recovery-affordance and last-failure-reason truth across all six claimed desktop profiles and every exported truth surface — UI, CLI, docs/help, diagnostics, support exports, telemetry, and claim publication.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.object_family.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact conformance causes\n\n");
        if self.conformance_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.conformance_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.object_family.as_str(),
                    cause.cause_token(),
                    cause.disclosed,
                    cause.detail,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Active waivers\n\n");
        if self.active_waivers.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for waiver in &self.active_waivers {
                out.push_str(&format!(
                    "- `{}` (`{}`, owner: {}, expires `{}`) — {}\n",
                    waiver.waiver_id,
                    waiver.object_family.as_str(),
                    waiver.owner_role,
                    waiver.expires_at,
                    waiver.reason,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Findings\n\n");
        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_release_proof -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_lifecycle_release_proof_fixtures\n");
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleReleaseProofDashboardRow {
    /// The object family.
    pub object_family: M5LifecycleObjectFamily,
    /// Short family label.
    pub object_label: String,
    /// The matrix journey the family drives.
    pub matrix_journey: M5CriticalJourney,
    /// Derived green/yellow/red status.
    pub status: LifecycleReleaseProofStatus,
    /// Number of claimed desktop profiles the truth is certified across.
    pub certified_profile_count: usize,
    /// Number of truth pillars kept.
    pub truth_pillar_count: usize,
    /// Number of declared consumer surfaces certified for this family.
    pub evaluated_surface_count: usize,
    /// Lifecycle-state-truth posture.
    pub lifecycle_state_truth: LifecycleStateTruthState,
    /// Checkpoint-truth posture.
    pub checkpoint_truth: CheckpointTruthState,
    /// Recovery-affordance-truth posture.
    pub recovery_affordance_truth: RecoveryAffordanceTruthState,
    /// Exported-proof-parity posture.
    pub exported_proof_parity: ExportedProofParityState,
    /// `true` when headless/companion-adjacent parity is preserved.
    pub headless_parity_preserved: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the Shiproom / Support Center / product UI / CLI / diagnostics /
/// claim-publication automation reads to auto-narrow an object family's release-proof claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleReleaseProofDashboard {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the dashboard.
    pub schema_version: u32,
    /// Stable dashboard id.
    pub dashboard_id: String,
    /// The packet id this dashboard projects.
    pub source_packet_ref: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Dashboard rows, in canonical order.
    pub rows: Vec<LifecycleReleaseProofDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Lifecycle / release automation refs that consume the dashboard.
    pub lifecycle_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl LifecycleReleaseProofDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &LifecycleReleaseProofPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| LifecycleReleaseProofDashboardRow {
                object_family: row.object_family,
                object_label: row.object_label.clone(),
                matrix_journey: row.matrix_journey,
                status: row.derived_status,
                certified_profile_count: row.certified_profiles.len(),
                truth_pillar_count: row.certified_truth_pillars.len(),
                evaluated_surface_count: row.evaluated_consumer_surfaces.len(),
                lifecycle_state_truth: row.lifecycle_state_truth,
                checkpoint_truth: row.checkpoint_truth,
                recovery_affordance_truth: row.recovery_affordance_truth,
                exported_proof_parity: row.exported_proof_parity,
                headless_parity_preserved: row.headless_parity_preserved,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .conformance_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_LIFECYCLE_RELEASE_PROOF_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_LIFECYCLE_RELEASE_PROOF_SCHEMA_VERSION,
            dashboard_id: M5_LIFECYCLE_RELEASE_PROOF_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            lifecycle_automation_refs: packet.lifecycle_automation_refs.clone(),
            generated_at: packet.generated_at.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 lifecycle-release-proof dashboard serializes")
    }
}

/// Support-export wrapper for the release-proof certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleReleaseProofSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: LifecycleReleaseProofPacket,
    /// Dashboard quoted in full.
    pub dashboard: LifecycleReleaseProofDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl LifecycleReleaseProofSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each object family, and each active
    /// waiver id is quoted as a case id so a support reviewer — or the lifecycle automation — can name
    /// the same family and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: LifecycleReleaseProofPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.object_family.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_LIFECYCLE_RELEASE_PROOF_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_LIFECYCLE_RELEASE_PROOF_SCHEMA_VERSION,
            shared_contract_ref: M5_LIFECYCLE_RELEASE_PROOF_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_lifecycle_release_proof_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleReleaseProofInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen lifecycle matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family certification rows.
    pub rows: Vec<LifecycleReleaseProofRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The certification packet carries only closed vocabulary, refs, and short labels, so raw URLs,
/// credentials, or tokens must never appear.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds a [`LifecycleReleaseProofPacket`] from the exact build identity, the frozen matrix ref, and
/// the per-family certification rows.
///
/// Each row's derived status and conformance causes, the aggregate counts, the active waivers, and the
/// blocking findings are recomputed here so the packet is the single source of truth and the
/// auto-narrowing cannot be asserted.
pub fn build_m5_lifecycle_release_proof_packet(
    input: LifecycleReleaseProofInput,
) -> LifecycleReleaseProofPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<LifecycleReleaseProofRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.conformance_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<LifecycleReleaseProofFinding> = Vec::new();

    // Every object family must carry a certification row.
    let present: BTreeSet<M5LifecycleObjectFamily> =
        rows.iter().map(|row| row.object_family).collect();
    for family in REQUIRED_OBJECT_FAMILIES {
        if !present.contains(&family) {
            blocking_findings.push(LifecycleReleaseProofFinding::ObjectFamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_object_families: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, LifecycleReleaseProofStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, LifecycleReleaseProofStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, LifecycleReleaseProofStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(LifecycleReleaseProofFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<LifecycleReleaseProofWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let conformance_causes: Vec<LifecycleReleaseProofCause> = rows
        .iter()
        .flat_map(|row| row.conformance_causes.clone())
        .collect();

    let required_proof_dimensions: Vec<String> = REQUIRED_PROOF_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    let required_profiles: Vec<String> = REQUIRED_PROFILES
        .iter()
        .map(|profile| profile.as_str().to_owned())
        .collect();
    let required_truth_pillars: Vec<String> = REQUIRED_TRUTH_PILLARS
        .iter()
        .map(|pillar| pillar.as_str().to_owned())
        .collect();
    let required_object_families: Vec<String> = REQUIRED_OBJECT_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();

    let mut packet = LifecycleReleaseProofPacket {
        record_kind: M5_LIFECYCLE_RELEASE_PROOF_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_LIFECYCLE_RELEASE_PROOF_SCHEMA_VERSION,
        shared_contract_ref: M5_LIFECYCLE_RELEASE_PROOF_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_LIFECYCLE_RELEASE_PROOF_PACKET_ID.to_owned(),
        source_schema_ref: M5_LIFECYCLE_RELEASE_PROOF_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Release-grade lifecycle proof for every claimed M5 object family, profile, and \
                   exported truth surface: each of the thirteen governed object families certified so \
                   its explicit lifecycle-state truth, its named milestone checkpoint truth, and its \
                   named recovery-affordance and controlled last-failure-reason truth hold across all \
                   six claimed desktop profiles and every exported truth surface — UI, CLI, docs/help, \
                   diagnostics, support exports, telemetry, and claim publication — with the same \
                   state-truth vocabulary preserved in headless and companion-adjacent execution, each \
                   family's green/yellow/red claim auto-narrowed from its four truth postures, and any \
                   family that still collapses state into generic loading or error behavior blocked \
                   from stable promotion."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        object_state_schema_ref: M5_LIFECYCLE_RELEASE_PROOF_OBJECT_STATE_SCHEMA_REF.to_owned(),
        journey_checkpoint_schema_ref: M5_LIFECYCLE_RELEASE_PROOF_JOURNEY_CHECKPOINT_SCHEMA_REF
            .to_owned(),
        matrix_doc_ref: M5_LIFECYCLE_RELEASE_PROOF_MATRIX_DOC_REF.to_owned(),
        state_object_inventory_ref: M5_LIFECYCLE_RELEASE_PROOF_STATE_OBJECT_INVENTORY_REF.to_owned(),
        state_class_recovery_ref: M5_LIFECYCLE_RELEASE_PROOF_STATE_CLASS_RECOVERY_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_proof_dimensions,
        required_profiles,
        required_truth_pillars,
        required_object_families,
        rows,
        covered_object_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        conformance_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        lifecycle_automation_refs: vec![
            "lifecycle_status.release_proof_registry".to_owned(),
            "release_automation.auto_narrow.release_proof_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.release_proof".to_owned(),
            M5_LIFECYCLE_RELEASE_PROOF_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_LIFECYCLE_RELEASE_PROOF_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-lifecycle-release-proof".to_owned()],
        published_report_ref: M5_LIFECYCLE_RELEASE_PROOF_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_LIFECYCLE_RELEASE_PROOF_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_LIFECYCLE_RELEASE_PROOF_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_LIFECYCLE_RELEASE_PROOF_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(LifecycleReleaseProofFinding::RawBoundaryMaterialInExport);
    }

    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    packet.report_clean = blocking_findings.is_empty();
    packet.blocking_findings = blocking_findings;

    packet
}

/// Validation error produced by [`validate_m5_lifecycle_release_proof_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum LifecycleReleaseProofValidationError {
    /// The packet has no rows.
    NoRows,
    /// The packet's record kind is wrong.
    WrongRecordKind,
    /// The packet's schema version is wrong.
    WrongSchemaVersion,
    /// The packet's exact-build identity ref is empty.
    BuildIdentityRefMissing,
    /// The packet does not certify a frozen matrix packet.
    MatrixPacketRefMissing,
    /// The declared required proof dimensions do not match the lane constants.
    RequiredProofDimensionsStale,
    /// The declared required profiles do not match the lane constants.
    RequiredProfilesStale,
    /// The declared required truth pillars do not match the lane constants.
    RequiredTruthPillarsStale,
    /// The declared required object families do not match the lane constants.
    RequiredObjectFamiliesStale,
    /// The rows do not cover all thirteen object families.
    CoverageIncomplete,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared conformance causes do not match the recomputed causes.
    ConformanceCausesStale,
    /// The declared blocking findings do not match the recomputed findings.
    BlockingFindingsStale,
    /// A blocking finding remains in the packet.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning subject ref.
        subject_ref: String,
    },
    /// The published report ref is empty.
    PublishedReportRefMissing,
    /// The published packet ref is empty.
    PublishedPacketRefMissing,
    /// The published dashboard ref is empty.
    PublishedDashboardRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a packet against the release-proof certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every object family carries a
/// current certification row; each row's status is the derived auto-narrowed value, never asserted; a
/// green row cannot keep a claim while it collapses its state into generic loading/error behavior,
/// collapses its checkpoints into an anonymous spinner, drops its recovery affordance or last-failure
/// reason, lets an exported truth surface go stale or divergent, loses headless/companion-adjacent
/// parity, fails to certify across all six claimed desktop profiles, fails to keep all three truth
/// pillars, or fails to certify every declared consumer surface; and a disclosed narrowing is backed by
/// a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_lifecycle_release_proof_packet(
    packet: &LifecycleReleaseProofPacket,
) -> Result<(), Vec<LifecycleReleaseProofValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(LifecycleReleaseProofValidationError::NoRows);
    }
    if packet.record_kind != M5_LIFECYCLE_RELEASE_PROOF_PACKET_RECORD_KIND {
        errors.push(LifecycleReleaseProofValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_LIFECYCLE_RELEASE_PROOF_SCHEMA_VERSION {
        errors.push(LifecycleReleaseProofValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(LifecycleReleaseProofValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(LifecycleReleaseProofValidationError::MatrixPacketRefMissing);
    }
    let expected_dimensions: Vec<String> = REQUIRED_PROOF_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    if packet.required_proof_dimensions != expected_dimensions {
        errors.push(LifecycleReleaseProofValidationError::RequiredProofDimensionsStale);
    }
    let expected_profiles: Vec<String> = REQUIRED_PROFILES
        .iter()
        .map(|profile| profile.as_str().to_owned())
        .collect();
    if packet.required_profiles != expected_profiles {
        errors.push(LifecycleReleaseProofValidationError::RequiredProfilesStale);
    }
    let expected_pillars: Vec<String> = REQUIRED_TRUTH_PILLARS
        .iter()
        .map(|pillar| pillar.as_str().to_owned())
        .collect();
    if packet.required_truth_pillars != expected_pillars {
        errors.push(LifecycleReleaseProofValidationError::RequiredTruthPillarsStale);
    }
    let expected_families: Vec<String> = REQUIRED_OBJECT_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();
    if packet.required_object_families != expected_families {
        errors.push(LifecycleReleaseProofValidationError::RequiredObjectFamiliesStale);
    }

    let present: BTreeSet<M5LifecycleObjectFamily> =
        packet.rows.iter().map(|row| row.object_family).collect();
    let coverage_complete = REQUIRED_OBJECT_FAMILIES
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != REQUIRED_OBJECT_FAMILIES.len() {
        errors.push(LifecycleReleaseProofValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_object_families {
        errors.push(LifecycleReleaseProofValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), LifecycleReleaseProofStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), LifecycleReleaseProofStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), LifecycleReleaseProofStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(LifecycleReleaseProofValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<LifecycleReleaseProofWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(LifecycleReleaseProofValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<LifecycleReleaseProofCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.conformance_causes {
        errors.push(LifecycleReleaseProofValidationError::ConformanceCausesStale);
    }

    let mut recomputed: Vec<LifecycleReleaseProofFinding> = Vec::new();
    for family in REQUIRED_OBJECT_FAMILIES {
        if !present.contains(&family) {
            recomputed.push(LifecycleReleaseProofFinding::ObjectFamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(LifecycleReleaseProofFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(LifecycleReleaseProofFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(LifecycleReleaseProofValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(
            LifecycleReleaseProofValidationError::BlockingFindingPresent {
                class: finding.class_token().to_owned(),
                subject_ref: finding.subject_ref().to_owned(),
            },
        );
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(LifecycleReleaseProofValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(LifecycleReleaseProofValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(LifecycleReleaseProofValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(LifecycleReleaseProofValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
