//! Canonical critical-journey checkpoint certification for the five highest-value M5 journeys.
//!
//! The [frozen lifecycle matrix][matrix] already binds each long-lived M5 object family to an
//! explicit state machine and freezes an inventory of protected journeys that must show named
//! milestone checkpoints instead of an anonymous spinner. This lane is the **certification
//! capstone** that materializes visible checkpoint surfaces for the five highest-value M5
//! journeys the spec protects by name — the **warm startup** flow (skeleton shell → command
//! system ready → session restore note → first interactive editor), the **large-repo open** flow
//! (partial tree → warm search fallback → indexing progress → first jump confidence note), the
//! **AI multi-file apply** flow (context resolving → approval requirement → reviewable patch →
//! verification result → rollback handle), the **remote attach-and-run** flow (auth/policy stage →
//! environment probe → sync warming → structured task stream), and the **collaboration
//! join-follow** flow (publish/join → role assignment → follow state → control transfer
//! visibility → archived outcome).
//!
//! For every one of those five journeys the lane certifies four things the acceptance criteria
//! require:
//!
//! - the journey **exposes visible milestone checkpoints instead of one anonymous spinner**
//!   ([`CheckpointVisibilityState`]);
//! - any **partial or degraded behavior stays labeled and attributable**
//!   ([`PartialTruthLabelingState`]);
//! - the journey **keeps the user's place and a next-safe-action** rather than dropping them onto
//!   a generic shell ([`PlaceContinuityState`]);
//! - and the same checkpoint truths **survive export, screenshot, and support-packet capture**
//!   ([`CaptureParityState`]).
//!
//! Three records carry the truth:
//!
//! - the per-journey **certification row** ([`CriticalJourneyRow`]): one row per
//!   [`M5ProtectedJourney`] naming the ordered milestone checkpoint sequence it shows (drawn from
//!   the frozen [`M5JourneyCheckpoint`] vocabulary), the object family it drives, the matrix
//!   journey it binds to (when one exists), its checkpoint-visibility / partial-truth-labeling /
//!   place-continuity / capture-parity posture, whether the same state-truth vocabulary survives
//!   headless/companion-adjacent execution, the consumer surfaces it evaluated, any active waiver,
//!   and a derived green/yellow/red [`CriticalJourneyStatus`].
//! - the release **certification packet** ([`CriticalJourneyPacket`]): the full set of rows with
//!   derived per-row status, aggregate green/yellow/red counts, the active waivers, the exact
//!   journey causes ([`CriticalJourneyCause`]), and the blocking findings the lane refuses to ship
//!   with.
//! - the **certification dashboard** ([`CriticalJourneyDashboard`]): a light projection the product
//!   UI / CLI / diagnostics / support / telemetry automation reads to auto-narrow a protected
//!   journey's checkpoint claim when its certification falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment a
//! journey discloses compacted milestones, discloses a coarse (rather than exact) partial-truth
//! label, keeps a disclosed, waivered reduced next-safe-action, or discloses a partial capture; it
//! drops to `red` if a journey falls back to an anonymous spinner, leaves a partial state
//! unlabeled or unattributed, loses the user's place or its recovery affordance, drops its
//! checkpoints from export/screenshot/support capture, loses the same state-truth vocabulary in a
//! headless/companion-adjacent execution, presents a malformed checkpoint sequence, or fails to
//! certify every consumer surface the matrix declares for the journey's driving object family. That
//! derivation is the auto-narrowing the acceptance criteria require, and the consumer-surface and
//! checkpoint-sequence completeness checks are the lints that prevent a certification from silently
//! regressing into a partial view — the exact regression that would let a protected flow hide a
//! half-ready or maybe-applied state behind one generic spinner on the surfaces it did not certify.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local
//! paths, raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed
//! vocabulary, counts, refs, and short labels. The object-family, journey, checkpoint, state,
//! recovery-affordance, last-failure-reason, consumer-surface, downgrade-trigger, and qualification
//! vocabulary is re-exported by reference from the already frozen [matrix], and every journey's
//! driving object family, explicit state machine, recovery affordance, and applicable triggers are
//! pulled straight from that matrix's seeded packet, so this lane mints no parallel lifecycle
//! vocabulary and cannot certify a journey the matrix does not anchor. Only the
//! critical-journey-specific vocabulary ([`M5ProtectedJourney`], [`M5CriticalJourneyDimension`],
//! [`CriticalJourneyStatus`], [`CheckpointVisibilityState`], [`PartialTruthLabelingState`],
//! [`PlaceContinuityState`], [`CaptureParityState`], [`CriticalJourneyWaiver`],
//! [`CriticalJourneyCause`], [`CriticalJourneyFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix as matrix;

pub use matrix::{
    M5CriticalJourney, M5JourneyCheckpoint, M5LastFailureReasonClass, M5LifecycleConsumerSurface,
    M5LifecycleDowngradeTrigger, M5LifecycleObjectFamily, M5LifecycleQualificationClass,
    M5LifecycleState, M5RecoveryAffordance,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_critical_journey_checkpoints_packet,
    seeded_m5_critical_journey_checkpoints_packet_ai_apply_capture_absent_blocked,
    seeded_m5_critical_journey_checkpoints_packet_collaboration_place_lost_blocked,
    seeded_m5_critical_journey_checkpoints_packet_large_repo_partial_unlabeled_blocked,
    seeded_m5_critical_journey_checkpoints_packet_remote_headless_parity_lost_blocked,
    seeded_m5_critical_journey_checkpoints_packet_warm_startup_anonymous_spinner_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_SHARED_CONTRACT_REF: &str =
    "lifecycle:m5_critical_journey_checkpoints:v1";

/// Stable record kind for [`CriticalJourneyPacket`] payloads.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_PACKET_RECORD_KIND: &str =
    "lifecycle_m5_critical_journey_checkpoints_packet_record";

/// Stable record kind for [`CriticalJourneyDashboard`] payloads.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_DASHBOARD_RECORD_KIND: &str =
    "lifecycle_m5_critical_journey_checkpoints_dashboard_record";

/// Stable record kind for [`CriticalJourneySupportExport`] payloads.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "lifecycle_m5_critical_journey_checkpoints_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_PACKET_ID: &str =
    "m5-critical-journey-checkpoints:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_DASHBOARD_ID: &str =
    "m5-critical-journey-checkpoints-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-critical-journey-checkpoints:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_SOURCE_SCHEMA_REF: &str =
    "schemas/lifecycle/m5-critical-journey-checkpoints.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_PUBLISHED_REPORT_REF: &str =
    "artifacts/lifecycle/m5-critical-journey-checkpoints.md";

/// Published certification-packet artifact ref.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-critical-journey-checkpoints-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-critical-journey-checkpoints-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-critical-journey-checkpoints-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-critical-journey-checkpoints-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_PUBLISHED_DOC_REF: &str =
    "docs/lifecycle/m5_critical_journey_checkpoints_contract.md";

/// Repo-relative ref to the frozen lifecycle object-state schema.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_OBJECT_STATE_SCHEMA_REF: &str =
    matrix::M5_LIFECYCLE_OBJECT_STATE_SCHEMA_REF;

/// Repo-relative ref to the frozen lifecycle journey-checkpoint schema.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_JOURNEY_CHECKPOINT_SCHEMA_REF: &str =
    matrix::M5_LIFECYCLE_JOURNEY_CHECKPOINT_SCHEMA_REF;

/// Frozen lifecycle-matrix contract doc this proof mirrors.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_MATRIX_DOC_REF: &str =
    matrix::M5_LIFECYCLE_MATRIX_DOC_REF;

/// State-object inventory this proof mirrors for the driving object families.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_STATE_OBJECT_INVENTORY_REF: &str =
    matrix::M5_LIFECYCLE_STATE_OBJECT_INVENTORY_REF;

/// State-class recovery reference this proof mirrors for the place-continuity binding.
pub const M5_CRITICAL_JOURNEY_CHECKPOINTS_STATE_CLASS_RECOVERY_REF: &str =
    matrix::M5_LIFECYCLE_STATE_CLASS_RECOVERY_REF;

/// Every protected journey the certification must cover, in canonical order. These are exactly the
/// five highest-value M5 journeys the spec protects by name; a certification that covers fewer
/// regresses into a partial view and blocks.
pub const REQUIRED_PROTECTED_JOURNEYS: [M5ProtectedJourney; 5] = M5ProtectedJourney::ALL;

/// Every checkpoint dimension each journey row certifies, in canonical order.
pub const REQUIRED_JOURNEY_DIMENSIONS: [M5CriticalJourneyDimension; 4] =
    M5CriticalJourneyDimension::ALL;

/// One of the five highest-value M5 journeys this lane materializes visible checkpoint surfaces for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProtectedJourney {
    /// Warm startup: skeleton shell → command system ready → session restore note → first
    /// interactive editor.
    WarmStartup,
    /// Large-repo open: partial tree → warm search fallback → indexing progress → first jump
    /// confidence note.
    LargeRepoOpen,
    /// AI multi-file apply: context resolving → approval requirement → reviewable patch →
    /// verification result → rollback handle.
    AiMultiFileApply,
    /// Remote attach-and-run: auth/policy stage → environment probe → sync warming → structured
    /// task stream.
    RemoteAttachRun,
    /// Collaboration join-follow: publish/join → role assignment → follow state → control transfer
    /// visibility → archived outcome.
    CollaborationJoinFollow,
}

impl M5ProtectedJourney {
    /// Every protected journey, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::WarmStartup,
        Self::LargeRepoOpen,
        Self::AiMultiFileApply,
        Self::RemoteAttachRun,
        Self::CollaborationJoinFollow,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WarmStartup => "warm_startup",
            Self::LargeRepoOpen => "large_repo_open",
            Self::AiMultiFileApply => "ai_multi_file_apply",
            Self::RemoteAttachRun => "remote_attach_run",
            Self::CollaborationJoinFollow => "collaboration_join_follow",
        }
    }
}

/// One of the four checkpoint dimensions each protected-journey row certifies.
///
/// These are exactly the four ways the acceptance criteria require a protected M5 journey to show
/// its checkpoint truth: it exposes visible milestone checkpoints instead of one anonymous spinner,
/// it keeps any partial or degraded behavior labeled and attributable, it keeps the user's place
/// and a next-safe-action, and its checkpoints survive export, screenshot, and support-packet
/// capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CriticalJourneyDimension {
    /// The journey exposes visible milestone checkpoints instead of one anonymous spinner.
    CheckpointVisibility,
    /// Partial or degraded behavior stays labeled and attributable.
    PartialTruthLabeling,
    /// The journey keeps the user's place and a next-safe-action.
    PlaceContinuity,
    /// The checkpoint truths survive export, screenshot, and support-packet capture.
    CaptureParity,
}

impl M5CriticalJourneyDimension {
    /// Every checkpoint dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CheckpointVisibility,
        Self::PartialTruthLabeling,
        Self::PlaceContinuity,
        Self::CaptureParity,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CheckpointVisibility => "checkpoint_visibility",
            Self::PartialTruthLabeling => "partial_truth_labeling",
            Self::PlaceContinuity => "place_continuity",
            Self::CaptureParity => "capture_parity",
        }
    }
}

/// The derived checkpoint-certification light a protected journey carries.
///
/// `green` means the journey exposes visible milestone checkpoints instead of one anonymous
/// spinner, keeps any partial or degraded behavior labeled and attributable, keeps the user's place
/// and a next-safe-action, and preserves its checkpoint truths through export/screenshot/support
/// capture — across every declared consumer surface and with the same state-truth vocabulary
/// surviving a headless/companion-adjacent execution. `yellow` is a disclosed narrowing (disclosed
/// compacted milestones, a disclosed coarse partial-truth label, a waivered reduced next-safe-action,
/// or a disclosed partial capture). `red` is blocked: an anonymous spinner, an unlabeled or
/// unattributed partial state, a lost place or recovery affordance, checkpoints absent from capture,
/// a headless/companion-adjacent vocabulary loss, a malformed checkpoint sequence, or a row that did
/// not certify every declared consumer surface — and it may not keep a checkpoint claim until
/// repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CriticalJourneyStatus {
    /// Full standing: all four checkpoint dimensions hold and headless parity is preserved.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl CriticalJourneyStatus {
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

/// How the journey exposes visible milestone checkpoints instead of one anonymous spinner.
///
/// `named_milestones_replace_spinner` means the journey shows its ordered, named milestone
/// checkpoints in place of a single opaque progress indicator. `disclosed_compacted_milestones`
/// means the journey presents its milestones in a disclosed compacted form on a compact surface
/// while still naming each one (a yellow narrowing). `anonymous_spinner_shown` means the journey
/// fell back to one anonymous monolithic spinner, hiding its milestone boundaries — always a
/// blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointVisibilityState {
    /// Named milestone checkpoints replace the anonymous spinner.
    NamedMilestonesReplaceSpinner,
    /// The journey presents its milestones in a disclosed compacted form.
    DisclosedCompactedMilestones,
    /// The journey fell back to one anonymous monolithic spinner — a blocker.
    AnonymousSpinnerShown,
}

impl CheckpointVisibilityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NamedMilestonesReplaceSpinner => "named_milestones_replace_spinner",
            Self::DisclosedCompactedMilestones => "disclosed_compacted_milestones",
            Self::AnonymousSpinnerShown => "anonymous_spinner_shown",
        }
    }

    /// `true` when named milestones are shown at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::NamedMilestonesReplaceSpinner)
    }

    /// `true` when the journey took a disclosed compacted-milestones narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedCompactedMilestones)
    }
}

/// How the journey keeps any partial or degraded behavior labeled and attributable.
///
/// `partial_state_labeled_and_attributed` means a partial-ready or degraded milestone always shows
/// a controlled label naming what is and is not ready and attributes the partial truth to a named
/// cause. `disclosed_coarse_partial_label` means the journey shows a disclosed coarse partial label
/// — for example naming a stage group rather than the exact sub-step — while still labeling and
/// attributing the partial state (a yellow narrowing). `partial_state_unlabeled_or_unattributed`
/// means a partial or degraded milestone went unlabeled or unattributed, so the user cannot tell
/// what is ready or why — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartialTruthLabelingState {
    /// Partial or degraded behavior is labeled and attributed.
    PartialStateLabeledAndAttributed,
    /// The journey shows a disclosed coarse partial label.
    DisclosedCoarsePartialLabel,
    /// A partial or degraded milestone went unlabeled or unattributed — a blocker.
    PartialStateUnlabeledOrUnattributed,
}

impl PartialTruthLabelingState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PartialStateLabeledAndAttributed => "partial_state_labeled_and_attributed",
            Self::DisclosedCoarsePartialLabel => "disclosed_coarse_partial_label",
            Self::PartialStateUnlabeledOrUnattributed => "partial_state_unlabeled_or_unattributed",
        }
    }

    /// `true` when partial truth is labeled and attributed at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::PartialStateLabeledAndAttributed)
    }

    /// `true` when the journey took a disclosed coarse-label narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedCoarsePartialLabel)
    }
}

/// How the journey keeps the user's place and a next-safe-action rather than dropping them onto a
/// generic shell.
///
/// `place_and_next_action_preserved` means that at every checkpoint — including a recoverable
/// failure — the journey keeps the user oriented at their place in the sequence and offers a named
/// next-safe-action (the recovery affordance). `disclosed_reduced_next_action` means the journey
/// keeps a disclosed, waivered reduced next-safe-action — for example deferring one recovery path
/// until a dependency resolves — while still keeping the user's place and a safe action (a yellow
/// narrowing). `place_or_recovery_lost` means the journey lost the user's place or its named
/// recovery affordance, dropping them onto a generic shell with no next-safe-action — always a
/// blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceContinuityState {
    /// The user's place and a named next-safe-action are preserved.
    PlaceAndNextActionPreserved,
    /// The journey keeps a disclosed, waivered reduced next-safe-action.
    DisclosedReducedNextAction,
    /// The journey lost the user's place or its recovery affordance — a blocker.
    PlaceOrRecoveryLost,
}

impl PlaceContinuityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlaceAndNextActionPreserved => "place_and_next_action_preserved",
            Self::DisclosedReducedNextAction => "disclosed_reduced_next_action",
            Self::PlaceOrRecoveryLost => "place_or_recovery_lost",
        }
    }

    /// `true` when place and next-safe-action are preserved at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::PlaceAndNextActionPreserved)
    }

    /// `true` when the journey took a disclosed reduced-next-action narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedNextAction)
    }
}

/// How the journey keeps its checkpoint truths surviving export, screenshot, and support capture.
///
/// `checkpoints_captured_in_export_and_screenshot` means the same named milestone checkpoints,
/// partial-truth labels, and next-safe-actions the user sees live are captured in a screenshot, a
/// support packet, and an export. `disclosed_partial_capture` means the journey captures a
/// disclosed reduced subset of its checkpoint detail — for example collapsing intermediate
/// milestones in a compact export — while still capturing the milestone boundaries and terminal (a
/// yellow narrowing). `checkpoints_absent_from_capture` means the journey's checkpoints did not
/// survive export/screenshot/support capture, so support and screenshots cannot reproduce the
/// checkpoint truth the user saw — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureParityState {
    /// The checkpoints are captured in export and screenshot.
    CheckpointsCapturedInExportAndScreenshot,
    /// The journey captures a disclosed reduced subset of its checkpoint detail.
    DisclosedPartialCapture,
    /// The journey's checkpoints did not survive capture — a blocker.
    CheckpointsAbsentFromCapture,
}

impl CaptureParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CheckpointsCapturedInExportAndScreenshot => {
                "checkpoints_captured_in_export_and_screenshot"
            }
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::CheckpointsAbsentFromCapture => "checkpoints_absent_from_capture",
        }
    }

    /// `true` when checkpoints are captured at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::CheckpointsCapturedInExportAndScreenshot)
    }

    /// `true` when the journey took a disclosed partial-capture narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed (yellow)
/// rather than blocked — never lets an anonymous spinner, an unlabeled partial state, a lost place,
/// or an uncaptured checkpoint hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CriticalJourneyWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The protected journey the waiver applies to.
    pub journey: M5ProtectedJourney,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl CriticalJourneyWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a protected journey's checkpoint certification.
///
/// The trigger token mirrors the frozen [`M5LifecycleDowngradeTrigger`] vocabulary so a cause never
/// mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CriticalJourneyCause {
    /// The protected journey the cause applies to.
    pub journey: M5ProtectedJourney,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5LifecycleDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed cause is
    /// a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl CriticalJourneyCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One protected journey, certified across its checkpoint-visibility, partial-truth-labeling,
/// place-continuity, and capture-parity dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CriticalJourneyRow {
    /// The protected journey being certified.
    pub journey: M5ProtectedJourney,
    /// Short reviewer-facing journey label.
    pub journey_label: String,
    /// The object family this journey drives. Pulled from the matrix.
    pub object_family: M5LifecycleObjectFamily,
    /// The frozen matrix journey this protected journey binds to, when one exists.
    pub matrix_journey: Option<M5CriticalJourney>,
    /// Qualification class the matrix earned for the driving object.
    pub qualification: M5LifecycleQualificationClass,
    /// Owner role accountable for keeping this journey governed. Pulled from the matrix.
    pub owner_role: String,
    /// Short journey scope summary.
    pub scope_summary: String,
    /// The controlled states the driving object's explicit state machine admits. Pulled from the
    /// matrix.
    pub admitted_states: Vec<M5LifecycleState>,
    /// The success terminal state the journey resolves to.
    pub success_state: M5LifecycleState,
    /// The one named recovery affordance the next-safe-action anchors on. Pulled from the matrix.
    pub recovery_affordance: M5RecoveryAffordance,
    /// Controlled last-failure reason classes this journey reports. Pulled from the matrix.
    pub last_failure_reason_classes: Vec<M5LastFailureReasonClass>,
    /// The ordered milestone checkpoints the journey shows instead of an anonymous spinner (at
    /// least two, unique, ending in a terminal).
    pub checkpoint_sequence: Vec<M5JourneyCheckpoint>,
    /// Consumer surfaces the matrix declares the driving object must project to.
    pub required_consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
    /// Consumer surfaces this certification evaluated. Pulled from the matrix.
    pub evaluated_consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
    /// Checkpoint-visibility posture.
    pub checkpoint_visibility: CheckpointVisibilityState,
    /// Partial-truth-labeling posture.
    pub partial_truth_labeling: PartialTruthLabelingState,
    /// Place-continuity posture.
    pub place_continuity: PlaceContinuityState,
    /// Capture-parity posture.
    pub capture_parity: CaptureParityState,
    /// `true` when the same state-truth vocabulary survives a headless or companion-adjacent
    /// execution; a hard invariant.
    pub headless_parity_preserved: bool,
    /// Downgrade triggers that apply to the driving object. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5LifecycleDowngradeTrigger>,
    /// Active waiver, when a disclosed reduced next-safe-action is in force.
    pub active_waiver: Option<CriticalJourneyWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: CriticalJourneyStatus,
    /// The exact journey causes that narrowed or blocked this row.
    pub journey_causes: Vec<CriticalJourneyCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl CriticalJourneyRow {
    /// `true` when the row certified every consumer surface the matrix declares for the driving
    /// object — no declared surface is left uncertified and none is invented.
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

    /// `true` when the checkpoint sequence is well-formed: at least two, unique, ending in a
    /// terminal milestone (ready / partial-ready / recoverable-failure). This is the structural
    /// proof that the journey shows named milestones rather than one anonymous spinner.
    pub fn checkpoint_sequence_well_formed(&self) -> bool {
        if self.checkpoint_sequence.len() < 2 {
            return false;
        }
        let unique: BTreeSet<M5JourneyCheckpoint> =
            self.checkpoint_sequence.iter().copied().collect();
        if unique.len() != self.checkpoint_sequence.len() {
            return false;
        }
        self.checkpoint_sequence
            .last()
            .is_some_and(|checkpoint| checkpoint.is_terminal())
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
        if !self.checkpoint_sequence_well_formed() {
            return true;
        }
        if !self.headless_parity_preserved {
            return true;
        }
        if matches!(
            self.checkpoint_visibility,
            CheckpointVisibilityState::AnonymousSpinnerShown
        ) {
            return true;
        }
        if matches!(
            self.partial_truth_labeling,
            PartialTruthLabelingState::PartialStateUnlabeledOrUnattributed
        ) {
            return true;
        }
        if matches!(
            self.place_continuity,
            PlaceContinuityState::PlaceOrRecoveryLost
        ) {
            return true;
        }
        if matches!(
            self.capture_parity,
            CaptureParityState::CheckpointsAbsentFromCapture
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.checkpoint_visibility.is_disclosed_narrowing()
            || self.partial_truth_labeling.is_disclosed_narrowing()
            || self.place_continuity.is_disclosed_narrowing()
            || self.capture_parity.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the journey posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> CriticalJourneyStatus {
        if self.has_hard_blocker() {
            CriticalJourneyStatus::Red
        } else if self.has_narrowing() {
            CriticalJourneyStatus::Yellow
        } else {
            CriticalJourneyStatus::Green
        }
    }

    /// Recomputes the exact journey causes for the row, in deterministic order (checkpoint
    /// visibility, partial-truth labeling, place continuity, capture parity, then headless parity
    /// and structural completeness).
    pub fn recompute_causes(&self) -> Vec<CriticalJourneyCause> {
        let mut causes = Vec::new();
        match self.checkpoint_visibility {
            CheckpointVisibilityState::NamedMilestonesReplaceSpinner => {}
            CheckpointVisibilityState::DisclosedCompactedMilestones => {
                causes.push(CriticalJourneyCause {
                    journey: self.journey,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The journey presents its milestone checkpoints in a disclosed \
                             compacted form on a compact surface while still naming each milestone \
                             individually, so the checkpoint sequence is narrowed and disclosed \
                             rather than collapsing into an anonymous spinner."
                        .to_owned(),
                });
            }
            CheckpointVisibilityState::AnonymousSpinnerShown => {
                causes.push(CriticalJourneyCause {
                    journey: self.journey,
                    trigger: M5LifecycleDowngradeTrigger::AnonymousCheckpoint,
                    disclosed: false,
                    detail: "The protected journey fell back to one anonymous monolithic spinner \
                             instead of its named milestone checkpoints, so a half-ready or \
                             maybe-applied state hides behind a single opaque progress indicator."
                        .to_owned(),
                });
            }
        }
        match self.partial_truth_labeling {
            PartialTruthLabelingState::PartialStateLabeledAndAttributed => {}
            PartialTruthLabelingState::DisclosedCoarsePartialLabel => {
                causes.push(CriticalJourneyCause {
                    journey: self.journey,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The journey shows a disclosed coarse partial-truth label — naming a \
                             stage group rather than the exact sub-step — while still labeling and \
                             attributing the partial state, so the partial truth is narrowed and \
                             disclosed rather than unlabeled."
                        .to_owned(),
                });
            }
            PartialTruthLabelingState::PartialStateUnlabeledOrUnattributed => {
                causes.push(CriticalJourneyCause {
                    journey: self.journey,
                    trigger: M5LifecycleDowngradeTrigger::LastFailureReasonMissing,
                    disclosed: false,
                    detail: "A partial or degraded milestone in the journey went unlabeled or \
                             unattributed, so the user cannot tell what is ready or why the flow \
                             narrowed, and support cannot attribute the partial state to a \
                             controlled cause."
                        .to_owned(),
                });
            }
        }
        match self.place_continuity {
            PlaceContinuityState::PlaceAndNextActionPreserved => {}
            PlaceContinuityState::DisclosedReducedNextAction => {
                causes.push(CriticalJourneyCause {
                    journey: self.journey,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The journey keeps a disclosed, waivered reduced next-safe-action — for \
                             example deferring one recovery path until a dependency resolves — while \
                             still keeping the user's place and a safe action, so the affordance is \
                             narrowed and disclosed rather than lost."
                        .to_owned(),
                });
            }
            PlaceContinuityState::PlaceOrRecoveryLost => {
                causes.push(CriticalJourneyCause {
                    journey: self.journey,
                    trigger: M5LifecycleDowngradeTrigger::RecoveryAffordanceMissing,
                    disclosed: false,
                    detail: "The journey lost the user's place or its named recovery affordance, \
                             dropping the user onto a generic shell with no next-safe-action to \
                             resume or recover the flow."
                        .to_owned(),
                });
            }
        }
        match self.capture_parity {
            CaptureParityState::CheckpointsCapturedInExportAndScreenshot => {}
            CaptureParityState::DisclosedPartialCapture => {
                causes.push(CriticalJourneyCause {
                    journey: self.journey,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The journey captures a disclosed reduced subset of its checkpoint \
                             detail in a compact export while still capturing the milestone \
                             boundaries and terminal, so the captured checkpoint truth is narrowed \
                             and disclosed rather than absent."
                        .to_owned(),
                });
            }
            CaptureParityState::CheckpointsAbsentFromCapture => {
                causes.push(CriticalJourneyCause {
                    journey: self.journey,
                    trigger: M5LifecycleDowngradeTrigger::StatusCodeUnexportable,
                    disclosed: false,
                    detail:
                        "The journey's named checkpoints did not survive export, screenshot, or \
                             support-packet capture, so support and screenshots cannot reproduce \
                             the checkpoint truth the user saw live."
                            .to_owned(),
                });
            }
        }
        if !self.checkpoint_sequence_well_formed() {
            causes.push(CriticalJourneyCause {
                journey: self.journey,
                trigger: M5LifecycleDowngradeTrigger::AnonymousCheckpoint,
                disclosed: false,
                detail: "The journey's declared checkpoint sequence is malformed — fewer than two \
                         milestones, a repeated milestone, or no terminal — so it cannot prove it \
                         shows named milestones instead of an anonymous spinner."
                    .to_owned(),
            });
        }
        if !self.headless_parity_preserved {
            causes.push(CriticalJourneyCause {
                journey: self.journey,
                trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                disclosed: false,
                detail:
                    "A headless or companion-adjacent execution of this journey lost the shared \
                         state-truth vocabulary for its checkpoints, so the same journey reports a \
                         different checkpoint and state language depending on how it runs."
                        .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed reduced next-safe-action may only stay yellow (rather than red) when a waiver
    /// discloses it — reducing the protected recovery affordance is the sensitive narrowing.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.place_continuity,
            PlaceContinuityState::DisclosedReducedNextAction
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<CriticalJourneyFinding> {
        let mut findings = Vec::new();
        let journey = self.journey.as_str().to_owned();

        if !self.consumer_surfaces_complete() {
            findings.push(CriticalJourneyFinding::ConsumerSurfacesIncomplete {
                journey: journey.clone(),
            });
        }
        if !self.checkpoint_sequence_well_formed() {
            findings.push(CriticalJourneyFinding::CheckpointSequenceMalformed {
                journey: journey.clone(),
            });
        }
        if !self.headless_parity_preserved {
            findings.push(CriticalJourneyFinding::HeadlessParityLost {
                journey: journey.clone(),
            });
        }
        if matches!(
            self.checkpoint_visibility,
            CheckpointVisibilityState::AnonymousSpinnerShown
        ) {
            findings.push(CriticalJourneyFinding::AnonymousSpinnerShown {
                journey: journey.clone(),
            });
        }
        if matches!(
            self.partial_truth_labeling,
            PartialTruthLabelingState::PartialStateUnlabeledOrUnattributed
        ) {
            findings.push(CriticalJourneyFinding::PartialStateUnlabeled {
                journey: journey.clone(),
            });
        }
        if matches!(
            self.place_continuity,
            PlaceContinuityState::PlaceOrRecoveryLost
        ) {
            findings.push(CriticalJourneyFinding::PlaceOrRecoveryLost {
                journey: journey.clone(),
            });
        }
        if matches!(
            self.capture_parity,
            CaptureParityState::CheckpointsAbsentFromCapture
        ) {
            findings.push(CriticalJourneyFinding::CheckpointsAbsentFromCapture {
                journey: journey.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, CriticalJourneyStatus::Green) && !self.has_reason() {
            findings.push(CriticalJourneyFinding::NarrowedRowWithoutReason {
                journey: journey.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active
        // waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(CriticalJourneyFinding::NarrowedRowWithoutWaiver {
                journey: journey.clone(),
            });
        }
        // An attached waiver must still be active and must point at this journey.
        if let Some(waiver) = &self.active_waiver {
            if waiver.journey != self.journey {
                findings.push(CriticalJourneyFinding::WaiverJourneyMismatch {
                    journey: journey.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(CriticalJourneyFinding::WaiverExpired {
                    journey: journey.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(CriticalJourneyFinding::RowStatusStale {
                journey: journey.clone(),
            });
        }
        if self.journey_causes != self.recompute_causes() {
            findings.push(CriticalJourneyFinding::RowCausesStale { journey });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} visibility={} partial={} place={} capture={} headless={} checkpoints={} surfaces={} waiver={}",
            self.journey.as_str(),
            self.derived_status.as_str(),
            self.checkpoint_visibility.as_str(),
            self.partial_truth_labeling.as_str(),
            self.place_continuity.as_str(),
            self.capture_parity.as_str(),
            self.headless_parity_preserved,
            self.checkpoint_sequence.len(),
            self.evaluated_consumer_surfaces.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the critical-journey certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum CriticalJourneyFinding {
    /// A protected journey has no certification row.
    JourneyMissing {
        /// The missing journey token.
        journey: String,
    },
    /// A row did not certify every declared consumer surface.
    ConsumerSurfacesIncomplete {
        /// The journey token.
        journey: String,
    },
    /// A row's checkpoint sequence is malformed (fewer than two, repeated, or no terminal).
    CheckpointSequenceMalformed {
        /// The journey token.
        journey: String,
    },
    /// A headless/companion-adjacent execution lost the shared state-truth vocabulary.
    HeadlessParityLost {
        /// The journey token.
        journey: String,
    },
    /// The journey fell back to one anonymous monolithic spinner.
    AnonymousSpinnerShown {
        /// The journey token.
        journey: String,
    },
    /// A partial or degraded milestone went unlabeled or unattributed.
    PartialStateUnlabeled {
        /// The journey token.
        journey: String,
    },
    /// The journey lost the user's place or its recovery affordance.
    PlaceOrRecoveryLost {
        /// The journey token.
        journey: String,
    },
    /// The journey's checkpoints did not survive export/screenshot/support capture.
    CheckpointsAbsentFromCapture {
        /// The journey token.
        journey: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The journey token.
        journey: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The journey token.
        journey: String,
    },
    /// An attached waiver does not point at the row's journey.
    WaiverJourneyMismatch {
        /// The journey token.
        journey: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The journey token.
        journey: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The journey token.
        journey: String,
    },
    /// The declared journey causes do not match the recomputed causes.
    RowCausesStale {
        /// The journey token.
        journey: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered journeys do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl CriticalJourneyFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::JourneyMissing { .. } => "journey_missing",
            Self::ConsumerSurfacesIncomplete { .. } => "consumer_surfaces_incomplete",
            Self::CheckpointSequenceMalformed { .. } => "checkpoint_sequence_malformed",
            Self::HeadlessParityLost { .. } => "headless_parity_lost",
            Self::AnonymousSpinnerShown { .. } => "anonymous_spinner_shown",
            Self::PartialStateUnlabeled { .. } => "partial_state_unlabeled",
            Self::PlaceOrRecoveryLost { .. } => "place_or_recovery_lost",
            Self::CheckpointsAbsentFromCapture { .. } => "checkpoints_absent_from_capture",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverJourneyMismatch { .. } => "waiver_journey_mismatch",
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
            Self::JourneyMissing { journey }
            | Self::ConsumerSurfacesIncomplete { journey }
            | Self::CheckpointSequenceMalformed { journey }
            | Self::HeadlessParityLost { journey }
            | Self::AnonymousSpinnerShown { journey }
            | Self::PartialStateUnlabeled { journey }
            | Self::PlaceOrRecoveryLost { journey }
            | Self::CheckpointsAbsentFromCapture { journey }
            | Self::NarrowedRowWithoutReason { journey }
            | Self::NarrowedRowWithoutWaiver { journey }
            | Self::WaiverJourneyMismatch { journey, .. }
            | Self::WaiverExpired { journey, .. }
            | Self::RowStatusStale { journey }
            | Self::RowCausesStale { journey } => journey,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release critical-journey certification packet shared by the product UI / CLI / diagnostics /
/// support / telemetry automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CriticalJourneyPacket {
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
    /// State-class recovery reference this proof mirrors for the place-continuity binding.
    pub state_class_recovery_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four checkpoint dimensions every journey row certifies.
    pub required_journey_dimensions: Vec<String>,
    /// The five protected journeys the certification must cover.
    pub required_protected_journeys: Vec<String>,
    /// Per-journey certification rows, in canonical order.
    pub rows: Vec<CriticalJourneyRow>,
    /// Protected journeys certified, in canonical (sorted) order.
    pub covered_protected_journeys: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (full-visibility) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<CriticalJourneyWaiver>,
    /// Every exact journey cause, in row then cause order.
    pub journey_causes: Vec<CriticalJourneyCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<CriticalJourneyFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Lifecycle / release automation refs that consume this packet to auto-narrow protected
    /// journeys.
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

impl CriticalJourneyPacket {
    /// Returns the certification row for `journey`, if present.
    pub fn row(&self, journey: M5ProtectedJourney) -> Option<&CriticalJourneyRow> {
        self.rows.iter().find(|row| row.journey == journey)
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
                waiver.journey.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.journey_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.journey.as_str(),
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
    pub fn dashboard(&self) -> CriticalJourneyDashboard {
        CriticalJourneyDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 critical-journey-checkpoints packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per protected journey naming its
    /// status, the four checkpoint postures, headless parity, the checkpoint count, the
    /// evaluated-surface count, and the waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "journey,status,checkpoint_visibility,partial_truth_labeling,place_continuity,capture_parity,headless_parity,checkpoints,evaluated_surfaces,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.journey.as_str(),
                row.derived_status.as_str(),
                row.checkpoint_visibility.as_str(),
                row.partial_truth_labeling.as_str(),
                row.place_continuity.as_str(),
                row.capture_parity.as_str(),
                row.headless_parity_preserved,
                row.checkpoint_sequence.len(),
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
            "# M5 critical-journey checkpoints: visible milestone surfaces for warm startup, large-repo open, AI multi-file apply, remote attach-and-run, and collaboration join-follow\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_critical_journey_checkpoints`](../../crates/aureline-shell/src/m5_critical_journey_checkpoints/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_critical_journey_checkpoints -- markdown > \\\n  artifacts/lifecycle/m5-critical-journey-checkpoints.md\n",
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
            "- Required checkpoint dimensions: {}\n",
            self.required_journey_dimensions
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Protected journeys certified: {}\n",
            self.row_count
        ));
        out.push_str(&format!(
            "- Green (full visibility): {}\n",
            self.green_row_count
        ));
        out.push_str(&format!(
            "- Yellow (auto-narrowed): {}\n",
            self.yellow_row_count
        ));
        out.push_str(&format!("- Red (blocked): {}\n", self.red_row_count));
        out.push_str(&format!(
            "- All rows publishable: `{}`\n",
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
            "| Journey | Status | Checkpoints | Visibility | Partial truth | Place | Capture | Headless | Waiver |\n\
             | ------- | ------ | ----------- | ---------- | ------------- | ----- | ------- | -------- | ------ |\n",
        );
        for row in &self.rows {
            let checkpoints = row
                .checkpoint_sequence
                .iter()
                .map(|checkpoint| checkpoint.as_str())
                .collect::<Vec<_>>()
                .join(" → ");
            out.push_str(&format!(
                "| {} | `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.journey_label,
                row.derived_status.as_str(),
                checkpoints,
                row.checkpoint_visibility.as_str(),
                row.partial_truth_labeling.as_str(),
                row.place_continuity.as_str(),
                row.capture_parity.as_str(),
                row.headless_parity_preserved,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&CriticalJourneyRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, CriticalJourneyStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every protected M5 journey exposes visible milestone checkpoints instead of one anonymous spinner, keeps partial behavior labeled and attributable, keeps the user's place and a next-safe-action, and preserves its checkpoint truths through export, screenshot, and support-packet capture across every declared consumer surface.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.journey.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact journey causes\n\n");
        if self.journey_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.journey_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.journey.as_str(),
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
                    waiver.journey.as_str(),
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_critical_journey_checkpoints -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_critical_journey_checkpoints_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CriticalJourneyDashboardRow {
    /// The protected journey.
    pub journey: M5ProtectedJourney,
    /// Short journey label.
    pub journey_label: String,
    /// The object family the journey drives.
    pub object_family: M5LifecycleObjectFamily,
    /// Derived green/yellow/red status.
    pub status: CriticalJourneyStatus,
    /// Number of milestone checkpoints in the sequence.
    pub checkpoint_count: usize,
    /// Number of declared consumer surfaces certified for this journey.
    pub evaluated_surface_count: usize,
    /// Checkpoint-visibility posture.
    pub checkpoint_visibility: CheckpointVisibilityState,
    /// Partial-truth-labeling posture.
    pub partial_truth_labeling: PartialTruthLabelingState,
    /// Place-continuity posture.
    pub place_continuity: PlaceContinuityState,
    /// Capture-parity posture.
    pub capture_parity: CaptureParityState,
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

/// The light certification dashboard the product UI / CLI / diagnostics / support / telemetry
/// automation reads to auto-narrow a protected journey's checkpoint claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CriticalJourneyDashboard {
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
    pub rows: Vec<CriticalJourneyDashboardRow>,
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

impl CriticalJourneyDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &CriticalJourneyPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| CriticalJourneyDashboardRow {
                journey: row.journey,
                journey_label: row.journey_label.clone(),
                object_family: row.object_family,
                status: row.derived_status,
                checkpoint_count: row.checkpoint_sequence.len(),
                evaluated_surface_count: row.evaluated_consumer_surfaces.len(),
                checkpoint_visibility: row.checkpoint_visibility,
                partial_truth_labeling: row.partial_truth_labeling,
                place_continuity: row.place_continuity,
                capture_parity: row.capture_parity,
                headless_parity_preserved: row.headless_parity_preserved,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .journey_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_CRITICAL_JOURNEY_CHECKPOINTS_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_CRITICAL_JOURNEY_CHECKPOINTS_SCHEMA_VERSION,
            dashboard_id: M5_CRITICAL_JOURNEY_CHECKPOINTS_DASHBOARD_ID.to_owned(),
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
        serde_json::to_string_pretty(self)
            .expect("m5 critical-journey-checkpoints dashboard serializes")
    }
}

/// Support-export wrapper for the critical-journey certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CriticalJourneySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: CriticalJourneyPacket,
    /// Dashboard quoted in full.
    pub dashboard: CriticalJourneyDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl CriticalJourneySupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each protected journey, and each
    /// active waiver id is quoted as a case id so a support reviewer — or the lifecycle automation —
    /// can name the same journey and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: CriticalJourneyPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.journey.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_CRITICAL_JOURNEY_CHECKPOINTS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_CRITICAL_JOURNEY_CHECKPOINTS_SCHEMA_VERSION,
            shared_contract_ref: M5_CRITICAL_JOURNEY_CHECKPOINTS_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_critical_journey_checkpoints_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CriticalJourneyInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen lifecycle matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-journey certification rows.
    pub rows: Vec<CriticalJourneyRow>,
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

/// Builds a [`CriticalJourneyPacket`] from the exact build identity, the frozen matrix ref, and the
/// per-journey certification rows.
///
/// Each row's derived status and journey causes, the aggregate counts, the active waivers, and the
/// blocking findings are recomputed here so the packet is the single source of truth and the
/// auto-narrowing cannot be asserted.
pub fn build_m5_critical_journey_checkpoints_packet(
    input: CriticalJourneyInput,
) -> CriticalJourneyPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<CriticalJourneyRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.journey_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<CriticalJourneyFinding> = Vec::new();

    // Every protected journey must carry a certification row.
    let present: BTreeSet<M5ProtectedJourney> = rows.iter().map(|row| row.journey).collect();
    for journey in REQUIRED_PROTECTED_JOURNEYS {
        if !present.contains(&journey) {
            blocking_findings.push(CriticalJourneyFinding::JourneyMissing {
                journey: journey.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_protected_journeys: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|journey| journey.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, CriticalJourneyStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, CriticalJourneyStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, CriticalJourneyStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(CriticalJourneyFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<CriticalJourneyWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let journey_causes: Vec<CriticalJourneyCause> = rows
        .iter()
        .flat_map(|row| row.journey_causes.clone())
        .collect();

    let required_journey_dimensions: Vec<String> = REQUIRED_JOURNEY_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    let required_protected_journeys: Vec<String> = REQUIRED_PROTECTED_JOURNEYS
        .iter()
        .map(|journey| journey.as_str().to_owned())
        .collect();

    let mut packet = CriticalJourneyPacket {
        record_kind: M5_CRITICAL_JOURNEY_CHECKPOINTS_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_CRITICAL_JOURNEY_CHECKPOINTS_SCHEMA_VERSION,
        shared_contract_ref: M5_CRITICAL_JOURNEY_CHECKPOINTS_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_CRITICAL_JOURNEY_CHECKPOINTS_PACKET_ID.to_owned(),
        source_schema_ref: M5_CRITICAL_JOURNEY_CHECKPOINTS_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Visible milestone checkpoint surfaces on the five highest-value M5 journeys: \
                   warm startup (skeleton shell → command system ready → session restore note → \
                   first interactive editor), large-repo open (partial tree → warm search fallback \
                   → indexing progress → first jump confidence note), AI multi-file apply (context \
                   resolving → approval requirement → reviewable patch → verification result → \
                   rollback handle), remote attach-and-run (auth/policy stage → environment probe → \
                   sync warming → structured task stream), and collaboration join-follow \
                   (publish/join → role assignment → follow state → control transfer visibility → \
                   archived outcome) each certified so the journey exposes visible milestones \
                   instead of one anonymous spinner, keeps partial behavior labeled and \
                   attributable, keeps the user's place and a next-safe-action, and preserves its \
                   checkpoint truths through export, screenshot, and support capture — across every \
                   declared consumer surface, with the same state-truth vocabulary preserved in \
                   headless and companion-adjacent execution — and each journey's green/yellow/red \
                   claim auto-narrowed from its four checkpoint postures."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        object_state_schema_ref: M5_CRITICAL_JOURNEY_CHECKPOINTS_OBJECT_STATE_SCHEMA_REF.to_owned(),
        journey_checkpoint_schema_ref:
            M5_CRITICAL_JOURNEY_CHECKPOINTS_JOURNEY_CHECKPOINT_SCHEMA_REF.to_owned(),
        matrix_doc_ref: M5_CRITICAL_JOURNEY_CHECKPOINTS_MATRIX_DOC_REF.to_owned(),
        state_object_inventory_ref: M5_CRITICAL_JOURNEY_CHECKPOINTS_STATE_OBJECT_INVENTORY_REF
            .to_owned(),
        state_class_recovery_ref: M5_CRITICAL_JOURNEY_CHECKPOINTS_STATE_CLASS_RECOVERY_REF
            .to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_journey_dimensions,
        required_protected_journeys,
        rows,
        covered_protected_journeys,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        journey_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        lifecycle_automation_refs: vec![
            "lifecycle_status.critical_journey_checkpoint_registry".to_owned(),
            "release_automation.auto_narrow.critical_journey_checkpoints_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.critical_journey_checkpoints".to_owned(),
            M5_CRITICAL_JOURNEY_CHECKPOINTS_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_CRITICAL_JOURNEY_CHECKPOINTS_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-critical-journey-checkpoints".to_owned()],
        published_report_ref: M5_CRITICAL_JOURNEY_CHECKPOINTS_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_CRITICAL_JOURNEY_CHECKPOINTS_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_CRITICAL_JOURNEY_CHECKPOINTS_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_CRITICAL_JOURNEY_CHECKPOINTS_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(CriticalJourneyFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_critical_journey_checkpoints_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum CriticalJourneyValidationError {
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
    /// The declared required checkpoint dimensions do not match the lane constants.
    RequiredJourneyDimensionsStale,
    /// The declared required protected journeys do not match the lane constants.
    RequiredProtectedJourneysStale,
    /// The rows do not cover all five protected journeys.
    CoverageIncomplete,
    /// The declared covered journeys do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared journey causes do not match the recomputed causes.
    JourneyCausesStale,
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

/// Validates a packet against the critical-journey certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every protected journey carries a
/// current certification row; each row's status is the derived auto-narrowed value, never asserted;
/// a green row cannot keep a claim while it shows an anonymous spinner, leaves a partial state
/// unlabeled, loses the user's place or recovery affordance, drops its checkpoints from capture,
/// loses headless/companion-adjacent parity, presents a malformed checkpoint sequence, or fails to
/// certify every declared consumer surface; and a disclosed narrowing is backed by a reason and,
/// where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_critical_journey_checkpoints_packet(
    packet: &CriticalJourneyPacket,
) -> Result<(), Vec<CriticalJourneyValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(CriticalJourneyValidationError::NoRows);
    }
    if packet.record_kind != M5_CRITICAL_JOURNEY_CHECKPOINTS_PACKET_RECORD_KIND {
        errors.push(CriticalJourneyValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_CRITICAL_JOURNEY_CHECKPOINTS_SCHEMA_VERSION {
        errors.push(CriticalJourneyValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(CriticalJourneyValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(CriticalJourneyValidationError::MatrixPacketRefMissing);
    }
    let expected_dimensions: Vec<String> = REQUIRED_JOURNEY_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    if packet.required_journey_dimensions != expected_dimensions {
        errors.push(CriticalJourneyValidationError::RequiredJourneyDimensionsStale);
    }
    let expected_journeys: Vec<String> = REQUIRED_PROTECTED_JOURNEYS
        .iter()
        .map(|journey| journey.as_str().to_owned())
        .collect();
    if packet.required_protected_journeys != expected_journeys {
        errors.push(CriticalJourneyValidationError::RequiredProtectedJourneysStale);
    }

    let present: BTreeSet<M5ProtectedJourney> = packet.rows.iter().map(|row| row.journey).collect();
    let coverage_complete = REQUIRED_PROTECTED_JOURNEYS
        .iter()
        .all(|journey| present.contains(journey));
    if !coverage_complete || packet.rows.len() != REQUIRED_PROTECTED_JOURNEYS.len() {
        errors.push(CriticalJourneyValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|journey| journey.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_protected_journeys {
        errors.push(CriticalJourneyValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), CriticalJourneyStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), CriticalJourneyStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), CriticalJourneyStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(CriticalJourneyValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<CriticalJourneyWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(CriticalJourneyValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<CriticalJourneyCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.journey_causes {
        errors.push(CriticalJourneyValidationError::JourneyCausesStale);
    }

    let mut recomputed: Vec<CriticalJourneyFinding> = Vec::new();
    for journey in REQUIRED_PROTECTED_JOURNEYS {
        if !present.contains(&journey) {
            recomputed.push(CriticalJourneyFinding::JourneyMissing {
                journey: journey.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(CriticalJourneyFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(CriticalJourneyFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(CriticalJourneyValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(CriticalJourneyValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(CriticalJourneyValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(CriticalJourneyValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(CriticalJourneyValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(CriticalJourneyValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
