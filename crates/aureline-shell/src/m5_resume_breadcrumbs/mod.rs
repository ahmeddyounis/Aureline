//! Canonical partial-truth and resume-breadcrumb certification for every long-lived M5 object.
//!
//! The [frozen lifecycle matrix][matrix] already binds each long-lived M5 object family to an
//! explicit state machine, a recovery affordance, a controlled last-failure reason, and an ordered
//! inventory of milestone checkpoints. This lane is the **certification capstone** that certifies,
//! for every one of those thirteen object families, that when a journey is degraded, resumed,
//! restored, partially replayed, or blocked it **preserves breadcrumbs** that let a user, support,
//! automation, and docs tell exactly what they are looking at and what Aureline did not silently do.
//!
//! For every object family the lane certifies four things the acceptance criteria require:
//!
//! - the object **distinguishes live truth, restored context, cached evidence, and a
//!   restart-required placeholder** with a labeled provenance header on every surface
//!   ([`ProvenanceLabelingState`]) — the criterion that restored, resumed, cached, and live states
//!   stay distinguishable;
//! - any resume/degrade breadcrumb **preserves source class, actor/subsystem, host or boundary, and
//!   checkpoint lineage** rather than collapsing into generic "recovered" wording
//!   ([`LineageBreadcrumbState`]);
//! - the object makes **intentionally not-resumed and not-reauthorized actions explicit** rather
//!   than silently absent ([`NotResumedDisclosureState`]);
//! - and the same breadcrumb truths **survive export, screenshot, and support-packet capture**
//!   ([`CaptureParityState`]) — the exit-gate criterion that recovery truth must not disappear in
//!   export paths.
//!
//! Three records carry the truth:
//!
//! - the per-family **certification row** ([`ResumeBreadcrumbRow`]): one row per
//!   [`M5LifecycleObjectFamily`] naming the four provenance classes it distinguishes (drawn from the
//!   [`M5ResumeProvenanceClass`] vocabulary), the four lineage facets it preserves (drawn from the
//!   [`M5BreadcrumbLineageFacet`] vocabulary), the checkpoint lineage it replays (drawn from the
//!   frozen [`M5JourneyCheckpoint`] vocabulary), its provenance-labeling / lineage-breadcrumb /
//!   not-resumed-disclosure / capture-parity posture, whether the same state-truth vocabulary
//!   survives headless/companion-adjacent execution, the consumer surfaces it evaluated, any active
//!   waiver, and a derived green/yellow/red [`ResumeBreadcrumbStatus`].
//! - the release **certification packet** ([`ResumeBreadcrumbPacket`]): the full set of rows with
//!   derived per-row status, aggregate green/yellow/red counts, the active waivers, the exact
//!   breadcrumb causes ([`ResumeBreadcrumbCause`]), and the blocking findings the lane refuses to
//!   ship with.
//! - the **certification dashboard** ([`ResumeBreadcrumbDashboard`]): a light projection the product
//!   UI / CLI / diagnostics / support / telemetry automation reads to auto-narrow a family's
//!   resume-breadcrumb claim when its certification falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment an
//! object discloses a coarse provenance grouping, discloses a partial lineage breadcrumb, keeps a
//! disclosed, waivered grouped not-resumed summary, or discloses a partial capture; it drops to
//! `red` if an object leaves its provenance class ambiguous or missing, shows only generic
//! "recovered" wording, leaves not-resumed actions silently absent, drops its breadcrumbs from
//! export/screenshot/support capture, loses the same state-truth vocabulary in a
//! headless/companion-adjacent execution, fails to distinguish all four provenance classes, fails to
//! preserve all four lineage facets, or fails to certify every consumer surface the matrix declares
//! for the family. That derivation is the auto-narrowing the acceptance criteria require, and the
//! consumer-surface, provenance-class, and lineage-facet completeness checks are the lints that
//! prevent a certification from silently regressing into a partial view — the exact regression that
//! would let a restored, cached, or restart-required state be mistaken for live truth on the
//! surfaces it did not certify.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local
//! paths, raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed
//! vocabulary, counts, refs, and short labels. The object-family, checkpoint, state,
//! recovery-affordance, last-failure-reason, consumer-surface, downgrade-trigger, journey, and
//! qualification vocabulary is re-exported by reference from the already frozen [matrix], and every
//! family's driving journey, explicit state machine, recovery affordance, checkpoint lineage, and
//! applicable triggers are pulled straight from that matrix's seeded packet, so this lane mints no
//! parallel lifecycle vocabulary and cannot certify a family the matrix does not anchor. Only the
//! resume-breadcrumb-specific vocabulary ([`M5ResumeProvenanceClass`], [`M5BreadcrumbLineageFacet`],
//! [`M5ResumeBreadcrumbDimension`], [`ResumeBreadcrumbStatus`], [`ProvenanceLabelingState`],
//! [`LineageBreadcrumbState`], [`NotResumedDisclosureState`], [`CaptureParityState`],
//! [`ResumeBreadcrumbWaiver`], [`ResumeBreadcrumbCause`], [`ResumeBreadcrumbFinding`]) is new.
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
    seeded_m5_resume_breadcrumbs_packet,
    seeded_m5_resume_breadcrumbs_packet_ai_capture_absent_blocked,
    seeded_m5_resume_breadcrumbs_packet_data_not_resumed_silent_blocked,
    seeded_m5_resume_breadcrumbs_packet_extension_headless_parity_lost_blocked,
    seeded_m5_resume_breadcrumbs_packet_notebook_provenance_ambiguous_blocked,
    seeded_m5_resume_breadcrumbs_packet_remote_generic_recovered_blocked, SEED_BUILD_IDENTITY_REF,
    SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_RESUME_BREADCRUMBS_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_RESUME_BREADCRUMBS_SHARED_CONTRACT_REF: &str = "lifecycle:m5_resume_breadcrumbs:v1";

/// Stable record kind for [`ResumeBreadcrumbPacket`] payloads.
pub const M5_RESUME_BREADCRUMBS_PACKET_RECORD_KIND: &str =
    "lifecycle_m5_resume_breadcrumbs_packet_record";

/// Stable record kind for [`ResumeBreadcrumbDashboard`] payloads.
pub const M5_RESUME_BREADCRUMBS_DASHBOARD_RECORD_KIND: &str =
    "lifecycle_m5_resume_breadcrumbs_dashboard_record";

/// Stable record kind for [`ResumeBreadcrumbSupportExport`] payloads.
pub const M5_RESUME_BREADCRUMBS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "lifecycle_m5_resume_breadcrumbs_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_RESUME_BREADCRUMBS_PACKET_ID: &str = "m5-resume-breadcrumbs:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_RESUME_BREADCRUMBS_DASHBOARD_ID: &str = "m5-resume-breadcrumbs-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_RESUME_BREADCRUMBS_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-resume-breadcrumbs:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_RESUME_BREADCRUMBS_SOURCE_SCHEMA_REF: &str =
    "schemas/lifecycle/m5-resume-breadcrumbs.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_RESUME_BREADCRUMBS_PUBLISHED_REPORT_REF: &str =
    "artifacts/lifecycle/m5-resume-breadcrumbs.md";

/// Published certification-packet artifact ref.
pub const M5_RESUME_BREADCRUMBS_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-resume-breadcrumbs-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_RESUME_BREADCRUMBS_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-resume-breadcrumbs-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_RESUME_BREADCRUMBS_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-resume-breadcrumbs-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_RESUME_BREADCRUMBS_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-resume-breadcrumbs-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_RESUME_BREADCRUMBS_PUBLISHED_DOC_REF: &str =
    "docs/lifecycle/m5_resume_breadcrumbs_contract.md";

/// Repo-relative ref to the frozen lifecycle object-state schema.
pub const M5_RESUME_BREADCRUMBS_OBJECT_STATE_SCHEMA_REF: &str =
    matrix::M5_LIFECYCLE_OBJECT_STATE_SCHEMA_REF;

/// Repo-relative ref to the frozen lifecycle journey-checkpoint schema.
pub const M5_RESUME_BREADCRUMBS_JOURNEY_CHECKPOINT_SCHEMA_REF: &str =
    matrix::M5_LIFECYCLE_JOURNEY_CHECKPOINT_SCHEMA_REF;

/// Frozen lifecycle-matrix contract doc this proof mirrors.
pub const M5_RESUME_BREADCRUMBS_MATRIX_DOC_REF: &str = matrix::M5_LIFECYCLE_MATRIX_DOC_REF;

/// State-object inventory this proof mirrors for the driving object families.
pub const M5_RESUME_BREADCRUMBS_STATE_OBJECT_INVENTORY_REF: &str =
    matrix::M5_LIFECYCLE_STATE_OBJECT_INVENTORY_REF;

/// State-class recovery reference this proof mirrors for the not-resumed-disclosure binding.
pub const M5_RESUME_BREADCRUMBS_STATE_CLASS_RECOVERY_REF: &str =
    matrix::M5_LIFECYCLE_STATE_CLASS_RECOVERY_REF;

/// Every object family the certification must cover, in canonical order. A certification that covers
/// fewer regresses into a partial view and blocks.
pub const REQUIRED_OBJECT_FAMILIES: [M5LifecycleObjectFamily; 13] = M5LifecycleObjectFamily::ALL;

/// Every breadcrumb dimension each family row certifies, in canonical order.
pub const REQUIRED_BREADCRUMB_DIMENSIONS: [M5ResumeBreadcrumbDimension; 4] =
    M5ResumeBreadcrumbDimension::ALL;

/// Every provenance class each family row must distinguish, in canonical order.
pub const REQUIRED_PROVENANCE_CLASSES: [M5ResumeProvenanceClass; 4] = M5ResumeProvenanceClass::ALL;

/// Every lineage facet each family row must preserve, in canonical order.
pub const REQUIRED_LINEAGE_FACETS: [M5BreadcrumbLineageFacet; 4] = M5BreadcrumbLineageFacet::ALL;

/// One of the four provenance classes a resume breadcrumb distinguishes so a user can tell whether
/// they are looking at live truth, restored context, cached evidence, or a restart-required
/// placeholder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResumeProvenanceClass {
    /// Live truth: the surface reflects the object's current, freshly computed state.
    LiveTruth,
    /// Restored context: the surface was rehydrated from a durable snapshot after a restart.
    RestoredContext,
    /// Cached evidence: the surface shows a prior value held past its freshness floor.
    CachedEvidence,
    /// Restart-required placeholder: the surface is a labeled placeholder that needs an explicit
    /// restart or reauthorize before it can carry live truth.
    RestartRequiredPlaceholder,
}

impl M5ResumeProvenanceClass {
    /// Every provenance class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LiveTruth,
        Self::RestoredContext,
        Self::CachedEvidence,
        Self::RestartRequiredPlaceholder,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTruth => "live_truth",
            Self::RestoredContext => "restored_context",
            Self::CachedEvidence => "cached_evidence",
            Self::RestartRequiredPlaceholder => "restart_required_placeholder",
        }
    }
}

/// One of the four lineage facets a resume breadcrumb preserves so a recovered surface names its
/// source, actor, boundary, and checkpoint lineage instead of a generic "recovered" label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BreadcrumbLineageFacet {
    /// Source class: whether the value is live, restored, cached, or a placeholder.
    SourceClass,
    /// Actor / subsystem: which controlled actor or subsystem produced the value.
    ActorSubsystem,
    /// Host / boundary: which host or trust boundary the value crossed.
    HostBoundary,
    /// Checkpoint lineage: which milestone checkpoint the value resumed from.
    CheckpointLineage,
}

impl M5BreadcrumbLineageFacet {
    /// Every lineage facet, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SourceClass,
        Self::ActorSubsystem,
        Self::HostBoundary,
        Self::CheckpointLineage,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceClass => "source_class",
            Self::ActorSubsystem => "actor_subsystem",
            Self::HostBoundary => "host_boundary",
            Self::CheckpointLineage => "checkpoint_lineage",
        }
    }
}

/// One of the four breadcrumb dimensions each object-family row certifies.
///
/// These are exactly the four ways the acceptance criteria require a degraded, resumed, or restored
/// M5 journey to preserve its breadcrumb truth: it distinguishes live truth, restored context,
/// cached evidence, and a restart-required placeholder; it preserves source/actor/boundary/checkpoint
/// lineage instead of generic "recovered" wording; it makes intentionally not-resumed actions
/// explicit; and its breadcrumbs survive export, screenshot, and support-packet capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResumeBreadcrumbDimension {
    /// The object distinguishes live truth, restored context, cached evidence, and a
    /// restart-required placeholder.
    ProvenanceLabeling,
    /// Breadcrumbs preserve source, actor, boundary, and checkpoint lineage.
    LineageBreadcrumb,
    /// Intentionally not-resumed / not-reauthorized actions are explicit.
    NotResumedDisclosure,
    /// Breadcrumb truths survive export, screenshot, and support-packet capture.
    CaptureParity,
}

impl M5ResumeBreadcrumbDimension {
    /// Every breadcrumb dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProvenanceLabeling,
        Self::LineageBreadcrumb,
        Self::NotResumedDisclosure,
        Self::CaptureParity,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvenanceLabeling => "provenance_labeling",
            Self::LineageBreadcrumb => "lineage_breadcrumb",
            Self::NotResumedDisclosure => "not_resumed_disclosure",
            Self::CaptureParity => "capture_parity",
        }
    }
}

/// The derived resume-breadcrumb certification light an object family carries.
///
/// `green` means the object distinguishes all four provenance classes, preserves all four lineage
/// facets instead of generic "recovered" wording, makes not-resumed actions explicit, and preserves
/// its breadcrumb truths through export/screenshot/support capture — across every declared consumer
/// surface and with the same state-truth vocabulary surviving a headless/companion-adjacent
/// execution. `yellow` is a disclosed narrowing (a disclosed coarse provenance grouping, a disclosed
/// partial lineage breadcrumb, a waivered grouped not-resumed summary, or a disclosed partial
/// capture). `red` is blocked: an ambiguous or missing provenance class, generic "recovered" wording
/// only, silently-absent not-resumed actions, breadcrumbs absent from capture, a
/// headless/companion-adjacent vocabulary loss, an incomplete provenance-class or lineage-facet set,
/// or a row that did not certify every declared consumer surface — and it may not keep a breadcrumb
/// claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResumeBreadcrumbStatus {
    /// Full standing: all four breadcrumb dimensions hold and headless parity is preserved.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl ResumeBreadcrumbStatus {
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

/// How the object distinguishes live truth, restored context, cached evidence, and a
/// restart-required placeholder.
///
/// `provenance_class_labeled_on_every_surface` means every surface shows a controlled provenance
/// header that names which of the four classes the value is. `disclosed_coarse_provenance_grouping`
/// means the object presents a disclosed coarse grouping of the provenance classes on a compact
/// surface — for example grouping restored context and cached evidence under one "recovered context"
/// header while still disclosing the grouping (a yellow narrowing). `provenance_class_ambiguous_or_missing`
/// means the object left the provenance class ambiguous or missing, so a restored, cached, or
/// restart-required value could be mistaken for live truth — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceLabelingState {
    /// The provenance class is labeled on every surface.
    ProvenanceClassLabeledOnEverySurface,
    /// The object presents a disclosed coarse provenance grouping.
    DisclosedCoarseProvenanceGrouping,
    /// The object left the provenance class ambiguous or missing — a blocker.
    ProvenanceClassAmbiguousOrMissing,
}

impl ProvenanceLabelingState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvenanceClassLabeledOnEverySurface => {
                "provenance_class_labeled_on_every_surface"
            }
            Self::DisclosedCoarseProvenanceGrouping => "disclosed_coarse_provenance_grouping",
            Self::ProvenanceClassAmbiguousOrMissing => "provenance_class_ambiguous_or_missing",
        }
    }

    /// `true` when the provenance class is labeled at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::ProvenanceClassLabeledOnEverySurface)
    }

    /// `true` when the object took a disclosed coarse-grouping narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedCoarseProvenanceGrouping)
    }
}

/// How the object preserves source, actor, boundary, and checkpoint lineage in its breadcrumbs.
///
/// `source_actor_boundary_checkpoint_preserved` means a resume/degrade breadcrumb always names the
/// source class, the actor/subsystem, the host/boundary, and the checkpoint lineage it resumed from.
/// `disclosed_partial_lineage_breadcrumb` means the object shows a disclosed partial lineage
/// breadcrumb on a compact surface — for example dropping the host/boundary detail while still
/// naming source, actor, and checkpoint — while still preserving the rest (a yellow narrowing).
/// `generic_recovered_wording_only` means the object shows only generic "recovered" wording with no
/// lineage, so the user cannot tell where the value came from — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageBreadcrumbState {
    /// Source, actor, boundary, and checkpoint lineage are preserved.
    SourceActorBoundaryCheckpointPreserved,
    /// The object shows a disclosed partial lineage breadcrumb.
    DisclosedPartialLineageBreadcrumb,
    /// The object shows only generic "recovered" wording — a blocker.
    GenericRecoveredWordingOnly,
}

impl LineageBreadcrumbState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceActorBoundaryCheckpointPreserved => {
                "source_actor_boundary_checkpoint_preserved"
            }
            Self::DisclosedPartialLineageBreadcrumb => "disclosed_partial_lineage_breadcrumb",
            Self::GenericRecoveredWordingOnly => "generic_recovered_wording_only",
        }
    }

    /// `true` when the full lineage is preserved at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::SourceActorBoundaryCheckpointPreserved)
    }

    /// `true` when the object took a disclosed partial-lineage narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialLineageBreadcrumb)
    }
}

/// How the object makes intentionally not-resumed and not-reauthorized actions explicit.
///
/// `not_resumed_actions_explicit` means the object names each action it intentionally did not rerun
/// or reauthorize after a restore or reconnect, rather than leaving the set silently absent.
/// `disclosed_grouped_not_resumed_summary` means the object presents a disclosed, waivered grouped
/// summary of the not-resumed set — for example naming a category rather than each action — while
/// still disclosing that actions were withheld (a yellow narrowing). `not_resumed_actions_silently_absent`
/// means the object silently dropped actions it did not rerun or reauthorize, so the user cannot tell
/// what Aureline intentionally did not do — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotResumedDisclosureState {
    /// Intentionally not-resumed actions are explicit.
    NotResumedActionsExplicit,
    /// The object presents a disclosed, waivered grouped not-resumed summary.
    DisclosedGroupedNotResumedSummary,
    /// The object silently dropped not-resumed actions — a blocker.
    NotResumedActionsSilentlyAbsent,
}

impl NotResumedDisclosureState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotResumedActionsExplicit => "not_resumed_actions_explicit",
            Self::DisclosedGroupedNotResumedSummary => "disclosed_grouped_not_resumed_summary",
            Self::NotResumedActionsSilentlyAbsent => "not_resumed_actions_silently_absent",
        }
    }

    /// `true` when not-resumed actions are explicit at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::NotResumedActionsExplicit)
    }

    /// `true` when the object took a disclosed grouped-summary narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedGroupedNotResumedSummary)
    }
}

/// How the object keeps its breadcrumb truths surviving export, screenshot, and support capture.
///
/// `breadcrumbs_captured_in_export_and_screenshot` means the same provenance headers, lineage
/// breadcrumbs, and not-resumed disclosures the user sees live are captured in a screenshot, a
/// support packet, and an export. `disclosed_partial_capture` means the object captures a disclosed
/// reduced subset of its breadcrumb detail — for example collapsing intermediate lineage in a compact
/// export — while still capturing the provenance header and terminal breadcrumb (a yellow narrowing).
/// `breadcrumbs_absent_from_capture` means the object's breadcrumbs did not survive
/// export/screenshot/support capture, so support and screenshots cannot reproduce the breadcrumb
/// truth the user saw — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureParityState {
    /// The breadcrumbs are captured in export and screenshot.
    BreadcrumbsCapturedInExportAndScreenshot,
    /// The object captures a disclosed reduced subset of its breadcrumb detail.
    DisclosedPartialCapture,
    /// The object's breadcrumbs did not survive capture — a blocker.
    BreadcrumbsAbsentFromCapture,
}

impl CaptureParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BreadcrumbsCapturedInExportAndScreenshot => {
                "breadcrumbs_captured_in_export_and_screenshot"
            }
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::BreadcrumbsAbsentFromCapture => "breadcrumbs_absent_from_capture",
        }
    }

    /// `true` when breadcrumbs are captured at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::BreadcrumbsCapturedInExportAndScreenshot)
    }

    /// `true` when the object took a disclosed partial-capture narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed (yellow)
/// rather than blocked — never lets an ambiguous provenance, a generic recovered label, a silently
/// dropped not-resumed action, or an uncaptured breadcrumb hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeBreadcrumbWaiver {
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

impl ResumeBreadcrumbWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked an object family's resume-breadcrumb certification.
///
/// The trigger token mirrors the frozen [`M5LifecycleDowngradeTrigger`] vocabulary so a cause never
/// mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeBreadcrumbCause {
    /// The object family the cause applies to.
    pub object_family: M5LifecycleObjectFamily,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5LifecycleDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed cause is
    /// a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl ResumeBreadcrumbCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One object family, certified across its provenance-labeling, lineage-breadcrumb,
/// not-resumed-disclosure, and capture-parity dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeBreadcrumbRow {
    /// The object family being certified.
    pub object_family: M5LifecycleObjectFamily,
    /// Short reviewer-facing family label.
    pub object_label: String,
    /// The frozen matrix journey this family drives its resume breadcrumbs through. Pulled from the
    /// matrix.
    pub matrix_journey: M5CriticalJourney,
    /// Qualification class the matrix earned for the object.
    pub qualification: M5LifecycleQualificationClass,
    /// Owner role accountable for keeping this family's breadcrumbs governed. Pulled from the matrix.
    pub owner_role: String,
    /// Short breadcrumb scope summary.
    pub scope_summary: String,
    /// The controlled states the object's explicit state machine admits. Pulled from the matrix.
    pub admitted_states: Vec<M5LifecycleState>,
    /// The one named recovery affordance the not-resumed disclosure anchors on. Pulled from the
    /// matrix.
    pub recovery_affordance: M5RecoveryAffordance,
    /// Controlled last-failure reason classes this family reports. Pulled from the matrix.
    pub last_failure_reason_classes: Vec<M5LastFailureReasonClass>,
    /// The ordered milestone checkpoints the resume breadcrumb replays lineage over. Pulled from the
    /// matrix journey row.
    pub checkpoint_lineage: Vec<M5JourneyCheckpoint>,
    /// The four provenance classes this row distinguishes (must be all four).
    pub distinguished_provenance_classes: Vec<M5ResumeProvenanceClass>,
    /// The four lineage facets this row preserves (must be all four).
    pub preserved_lineage_facets: Vec<M5BreadcrumbLineageFacet>,
    /// Consumer surfaces the matrix declares the object must project to.
    pub required_consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
    /// Consumer surfaces this certification evaluated. Pulled from the matrix.
    pub evaluated_consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
    /// Provenance-labeling posture.
    pub provenance_labeling: ProvenanceLabelingState,
    /// Lineage-breadcrumb posture.
    pub lineage_breadcrumb: LineageBreadcrumbState,
    /// Not-resumed-disclosure posture.
    pub not_resumed_disclosure: NotResumedDisclosureState,
    /// Capture-parity posture.
    pub capture_parity: CaptureParityState,
    /// `true` when the same state-truth vocabulary survives a headless or companion-adjacent
    /// execution; a hard invariant.
    pub headless_parity_preserved: bool,
    /// Downgrade triggers that apply to the object. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5LifecycleDowngradeTrigger>,
    /// Active waiver, when a disclosed grouped not-resumed summary is in force.
    pub active_waiver: Option<ResumeBreadcrumbWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: ResumeBreadcrumbStatus,
    /// The exact breadcrumb causes that narrowed or blocked this row.
    pub breadcrumb_causes: Vec<ResumeBreadcrumbCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl ResumeBreadcrumbRow {
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

    /// `true` when the row distinguishes every one of the four provenance classes — the structural
    /// proof that restored, resumed, cached, and live states stay distinguishable.
    pub fn provenance_classes_complete(&self) -> bool {
        let mut distinguished: Vec<&str> = self
            .distinguished_provenance_classes
            .iter()
            .map(|class| class.as_str())
            .collect();
        let mut required: Vec<&str> = REQUIRED_PROVENANCE_CLASSES
            .iter()
            .map(|class| class.as_str())
            .collect();
        distinguished.sort_unstable();
        distinguished.dedup();
        required.sort_unstable();
        distinguished == required
    }

    /// `true` when the row preserves every one of the four lineage facets — the structural proof
    /// that a breadcrumb names source, actor, boundary, and checkpoint lineage instead of a generic
    /// "recovered" label.
    pub fn lineage_facets_complete(&self) -> bool {
        let mut preserved: Vec<&str> = self
            .preserved_lineage_facets
            .iter()
            .map(|facet| facet.as_str())
            .collect();
        let mut required: Vec<&str> = REQUIRED_LINEAGE_FACETS
            .iter()
            .map(|facet| facet.as_str())
            .collect();
        preserved.sort_unstable();
        preserved.dedup();
        required.sort_unstable();
        preserved == required
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
        if !self.provenance_classes_complete() {
            return true;
        }
        if !self.lineage_facets_complete() {
            return true;
        }
        if !self.headless_parity_preserved {
            return true;
        }
        if matches!(
            self.provenance_labeling,
            ProvenanceLabelingState::ProvenanceClassAmbiguousOrMissing
        ) {
            return true;
        }
        if matches!(
            self.lineage_breadcrumb,
            LineageBreadcrumbState::GenericRecoveredWordingOnly
        ) {
            return true;
        }
        if matches!(
            self.not_resumed_disclosure,
            NotResumedDisclosureState::NotResumedActionsSilentlyAbsent
        ) {
            return true;
        }
        if matches!(
            self.capture_parity,
            CaptureParityState::BreadcrumbsAbsentFromCapture
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.provenance_labeling.is_disclosed_narrowing()
            || self.lineage_breadcrumb.is_disclosed_narrowing()
            || self.not_resumed_disclosure.is_disclosed_narrowing()
            || self.capture_parity.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the breadcrumb posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> ResumeBreadcrumbStatus {
        if self.has_hard_blocker() {
            ResumeBreadcrumbStatus::Red
        } else if self.has_narrowing() {
            ResumeBreadcrumbStatus::Yellow
        } else {
            ResumeBreadcrumbStatus::Green
        }
    }

    /// Recomputes the exact breadcrumb causes for the row, in deterministic order (provenance
    /// labeling, lineage breadcrumb, not-resumed disclosure, capture parity, then structural
    /// completeness and headless parity).
    pub fn recompute_causes(&self) -> Vec<ResumeBreadcrumbCause> {
        let mut causes = Vec::new();
        match self.provenance_labeling {
            ProvenanceLabelingState::ProvenanceClassLabeledOnEverySurface => {}
            ProvenanceLabelingState::DisclosedCoarseProvenanceGrouping => {
                causes.push(ResumeBreadcrumbCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The object presents a disclosed coarse provenance grouping on a compact \
                             surface — for example grouping restored context and cached evidence \
                             under one disclosed recovered-context header — while still distinguishing \
                             the four classes, so the provenance labeling is narrowed and disclosed \
                             rather than ambiguous."
                        .to_owned(),
                });
            }
            ProvenanceLabelingState::ProvenanceClassAmbiguousOrMissing => {
                causes.push(ResumeBreadcrumbCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                    disclosed: false,
                    detail: "The object left its provenance class ambiguous or missing, so a \
                             restored, cached, or restart-required value could be mistaken for live \
                             truth, and the four controlled provenance classes no longer keep the \
                             same meaning on the surface."
                        .to_owned(),
                });
            }
        }
        match self.lineage_breadcrumb {
            LineageBreadcrumbState::SourceActorBoundaryCheckpointPreserved => {}
            LineageBreadcrumbState::DisclosedPartialLineageBreadcrumb => {
                causes.push(ResumeBreadcrumbCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The object shows a disclosed partial lineage breadcrumb on a compact \
                             surface — dropping one facet such as the host/boundary detail while \
                             still naming the source class, actor/subsystem, and checkpoint lineage — \
                             so the breadcrumb lineage is narrowed and disclosed rather than \
                             collapsing into generic recovered wording."
                        .to_owned(),
                });
            }
            LineageBreadcrumbState::GenericRecoveredWordingOnly => {
                causes.push(ResumeBreadcrumbCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::LastFailureReasonMissing,
                    disclosed: false,
                    detail: "The object shows only generic recovered wording with no source class, \
                             actor/subsystem, host/boundary, or checkpoint lineage, so the user and \
                             support cannot attribute the recovered value to a controlled source."
                        .to_owned(),
                });
            }
        }
        match self.not_resumed_disclosure {
            NotResumedDisclosureState::NotResumedActionsExplicit => {}
            NotResumedDisclosureState::DisclosedGroupedNotResumedSummary => {
                causes.push(ResumeBreadcrumbCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The object presents a disclosed, waivered grouped summary of the \
                             actions it intentionally did not rerun or reauthorize after restore or \
                             reconnect — naming the withheld category rather than each action — while \
                             still disclosing that actions were withheld, so the not-resumed \
                             disclosure is narrowed and disclosed rather than silently absent."
                        .to_owned(),
                });
            }
            NotResumedDisclosureState::NotResumedActionsSilentlyAbsent => {
                causes.push(ResumeBreadcrumbCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::RecoveryAffordanceMissing,
                    disclosed: false,
                    detail: "The object silently dropped actions it did not rerun or reauthorize \
                             after restore or reconnect, so the user cannot tell what Aureline \
                             intentionally did not do and has no named affordance to rerun or \
                             reauthorize it."
                        .to_owned(),
                });
            }
        }
        match self.capture_parity {
            CaptureParityState::BreadcrumbsCapturedInExportAndScreenshot => {}
            CaptureParityState::DisclosedPartialCapture => {
                causes.push(ResumeBreadcrumbCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail:
                        "The object captures a disclosed reduced subset of its breadcrumb detail \
                             in a compact export while still capturing the provenance header and \
                             terminal breadcrumb, so the captured breadcrumb truth is narrowed and \
                             disclosed rather than absent."
                            .to_owned(),
                });
            }
            CaptureParityState::BreadcrumbsAbsentFromCapture => {
                causes.push(ResumeBreadcrumbCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::StatusCodeUnexportable,
                    disclosed: false,
                    detail:
                        "The object's provenance headers and lineage breadcrumbs did not survive \
                             export, screenshot, or support-packet capture, so support and \
                             screenshots cannot reproduce the breadcrumb truth the user saw live."
                            .to_owned(),
                });
            }
        }
        if !self.provenance_classes_complete() {
            causes.push(ResumeBreadcrumbCause {
                object_family: self.object_family,
                trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                disclosed: false,
                detail:
                    "The object does not distinguish all four provenance classes — live truth, \
                         restored context, cached evidence, and restart-required placeholder — so \
                         restored, resumed, cached, and live states cannot all be told apart."
                        .to_owned(),
            });
        }
        if !self.lineage_facets_complete() {
            causes.push(ResumeBreadcrumbCause {
                object_family: self.object_family,
                trigger: M5LifecycleDowngradeTrigger::LastFailureReasonMissing,
                disclosed: false,
                detail: "The object does not preserve all four lineage facets — source class, \
                         actor/subsystem, host/boundary, and checkpoint lineage — so a recovered \
                         breadcrumb cannot fully attribute its value."
                    .to_owned(),
            });
        }
        if !self.headless_parity_preserved {
            causes.push(ResumeBreadcrumbCause {
                object_family: self.object_family,
                trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                disclosed: false,
                detail: "A headless or companion-adjacent execution of this object lost the shared \
                         state-truth vocabulary for its resume breadcrumbs, so the same object \
                         reports a different provenance and lineage language depending on how it runs."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed grouped not-resumed summary may only stay yellow (rather than red) when a waiver
    /// discloses it — reducing the disclosure of what was not rerun or reauthorized is the sensitive
    /// narrowing.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.not_resumed_disclosure,
            NotResumedDisclosureState::DisclosedGroupedNotResumedSummary
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<ResumeBreadcrumbFinding> {
        let mut findings = Vec::new();
        let family = self.object_family.as_str().to_owned();

        if !self.consumer_surfaces_complete() {
            findings.push(ResumeBreadcrumbFinding::ConsumerSurfacesIncomplete {
                family: family.clone(),
            });
        }
        if !self.provenance_classes_complete() {
            findings.push(ResumeBreadcrumbFinding::ProvenanceClassesIncomplete {
                family: family.clone(),
            });
        }
        if !self.lineage_facets_complete() {
            findings.push(ResumeBreadcrumbFinding::LineageFacetsIncomplete {
                family: family.clone(),
            });
        }
        if !self.headless_parity_preserved {
            findings.push(ResumeBreadcrumbFinding::HeadlessParityLost {
                family: family.clone(),
            });
        }
        if matches!(
            self.provenance_labeling,
            ProvenanceLabelingState::ProvenanceClassAmbiguousOrMissing
        ) {
            findings.push(ResumeBreadcrumbFinding::ProvenanceAmbiguous {
                family: family.clone(),
            });
        }
        if matches!(
            self.lineage_breadcrumb,
            LineageBreadcrumbState::GenericRecoveredWordingOnly
        ) {
            findings.push(ResumeBreadcrumbFinding::LineageGenericRecoveredOnly {
                family: family.clone(),
            });
        }
        if matches!(
            self.not_resumed_disclosure,
            NotResumedDisclosureState::NotResumedActionsSilentlyAbsent
        ) {
            findings.push(ResumeBreadcrumbFinding::NotResumedActionsSilentlyAbsent {
                family: family.clone(),
            });
        }
        if matches!(
            self.capture_parity,
            CaptureParityState::BreadcrumbsAbsentFromCapture
        ) {
            findings.push(ResumeBreadcrumbFinding::BreadcrumbsAbsentFromCapture {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, ResumeBreadcrumbStatus::Green) && !self.has_reason() {
            findings.push(ResumeBreadcrumbFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active
        // waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(ResumeBreadcrumbFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.object_family != self.object_family {
                findings.push(ResumeBreadcrumbFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(ResumeBreadcrumbFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(ResumeBreadcrumbFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.breadcrumb_causes != self.recompute_causes() {
            findings.push(ResumeBreadcrumbFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} provenance={} lineage={} not_resumed={} capture={} headless={} classes={} facets={} surfaces={} waiver={}",
            self.object_family.as_str(),
            self.derived_status.as_str(),
            self.provenance_labeling.as_str(),
            self.lineage_breadcrumb.as_str(),
            self.not_resumed_disclosure.as_str(),
            self.capture_parity.as_str(),
            self.headless_parity_preserved,
            self.distinguished_provenance_classes.len(),
            self.preserved_lineage_facets.len(),
            self.evaluated_consumer_surfaces.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the resume-breadcrumb certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum ResumeBreadcrumbFinding {
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
    /// A row does not distinguish all four provenance classes.
    ProvenanceClassesIncomplete {
        /// The family token.
        family: String,
    },
    /// A row does not preserve all four lineage facets.
    LineageFacetsIncomplete {
        /// The family token.
        family: String,
    },
    /// A headless/companion-adjacent execution lost the shared state-truth vocabulary.
    HeadlessParityLost {
        /// The family token.
        family: String,
    },
    /// The object left its provenance class ambiguous or missing.
    ProvenanceAmbiguous {
        /// The family token.
        family: String,
    },
    /// The object shows only generic "recovered" wording with no lineage.
    LineageGenericRecoveredOnly {
        /// The family token.
        family: String,
    },
    /// The object silently dropped not-resumed / not-reauthorized actions.
    NotResumedActionsSilentlyAbsent {
        /// The family token.
        family: String,
    },
    /// The object's breadcrumbs did not survive export/screenshot/support capture.
    BreadcrumbsAbsentFromCapture {
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
    /// The declared breadcrumb causes do not match the recomputed causes.
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

impl ResumeBreadcrumbFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::ObjectFamilyMissing { .. } => "object_family_missing",
            Self::ConsumerSurfacesIncomplete { .. } => "consumer_surfaces_incomplete",
            Self::ProvenanceClassesIncomplete { .. } => "provenance_classes_incomplete",
            Self::LineageFacetsIncomplete { .. } => "lineage_facets_incomplete",
            Self::HeadlessParityLost { .. } => "headless_parity_lost",
            Self::ProvenanceAmbiguous { .. } => "provenance_ambiguous",
            Self::LineageGenericRecoveredOnly { .. } => "lineage_generic_recovered_only",
            Self::NotResumedActionsSilentlyAbsent { .. } => "not_resumed_actions_silently_absent",
            Self::BreadcrumbsAbsentFromCapture { .. } => "breadcrumbs_absent_from_capture",
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
            | Self::ProvenanceClassesIncomplete { family }
            | Self::LineageFacetsIncomplete { family }
            | Self::HeadlessParityLost { family }
            | Self::ProvenanceAmbiguous { family }
            | Self::LineageGenericRecoveredOnly { family }
            | Self::NotResumedActionsSilentlyAbsent { family }
            | Self::BreadcrumbsAbsentFromCapture { family }
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

/// The release resume-breadcrumb certification packet shared by the product UI / CLI / diagnostics /
/// support / telemetry automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeBreadcrumbPacket {
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
    /// State-class recovery reference this proof mirrors for the not-resumed-disclosure binding.
    pub state_class_recovery_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four breadcrumb dimensions every family row certifies.
    pub required_breadcrumb_dimensions: Vec<String>,
    /// The four provenance classes every family row must distinguish.
    pub required_provenance_classes: Vec<String>,
    /// The four lineage facets every family row must preserve.
    pub required_lineage_facets: Vec<String>,
    /// The thirteen object families the certification must cover.
    pub required_object_families: Vec<String>,
    /// Per-family certification rows, in canonical order.
    pub rows: Vec<ResumeBreadcrumbRow>,
    /// Object families certified, in canonical (sorted) order.
    pub covered_object_families: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (full-breadcrumb) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<ResumeBreadcrumbWaiver>,
    /// Every exact breadcrumb cause, in row then cause order.
    pub breadcrumb_causes: Vec<ResumeBreadcrumbCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<ResumeBreadcrumbFinding>,
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

impl ResumeBreadcrumbPacket {
    /// Returns the certification row for `family`, if present.
    pub fn row(&self, family: M5LifecycleObjectFamily) -> Option<&ResumeBreadcrumbRow> {
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
        for cause in &self.breadcrumb_causes {
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
    pub fn dashboard(&self) -> ResumeBreadcrumbDashboard {
        ResumeBreadcrumbDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 resume-breadcrumbs packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per object family naming its
    /// status, the four breadcrumb postures, headless parity, the provenance-class and lineage-facet
    /// counts, the evaluated-surface count, and the waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object_family,status,provenance_labeling,lineage_breadcrumb,not_resumed_disclosure,capture_parity,headless_parity,provenance_classes,lineage_facets,evaluated_surfaces,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.object_family.as_str(),
                row.derived_status.as_str(),
                row.provenance_labeling.as_str(),
                row.lineage_breadcrumb.as_str(),
                row.not_resumed_disclosure.as_str(),
                row.capture_parity.as_str(),
                row.headless_parity_preserved,
                row.distinguished_provenance_classes.len(),
                row.preserved_lineage_facets.len(),
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
            "# M5 resume breadcrumbs: partial-truth and resume lineage across degraded or restored M5 journeys\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_resume_breadcrumbs`](../../crates/aureline-shell/src/m5_resume_breadcrumbs/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_resume_breadcrumbs -- markdown > \\\n  artifacts/lifecycle/m5-resume-breadcrumbs.md\n",
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
            "- Required breadcrumb dimensions: {}\n",
            self.required_breadcrumb_dimensions
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
            "- Green (full breadcrumbs): {}\n",
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
            "| Object family | Status | Provenance | Lineage | Not resumed | Capture | Headless | Waiver |\n\
             | ------------- | ------ | ---------- | ------- | ----------- | ------- | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.object_label,
                row.derived_status.as_str(),
                row.provenance_labeling.as_str(),
                row.lineage_breadcrumb.as_str(),
                row.not_resumed_disclosure.as_str(),
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
        let narrowed: Vec<&ResumeBreadcrumbRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, ResumeBreadcrumbStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every long-lived M5 object distinguishes live truth, restored context, cached evidence, and a restart-required placeholder, preserves source/actor/boundary/checkpoint lineage instead of generic recovered wording, makes intentionally not-resumed actions explicit, and preserves its breadcrumb truths through export, screenshot, and support-packet capture across every declared consumer surface.\n\n",
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

        out.push_str("## Exact breadcrumb causes\n\n");
        if self.breadcrumb_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.breadcrumb_causes {
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_resume_breadcrumbs -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_resume_breadcrumbs_fixtures\n");
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeBreadcrumbDashboardRow {
    /// The object family.
    pub object_family: M5LifecycleObjectFamily,
    /// Short family label.
    pub object_label: String,
    /// The matrix journey the family drives.
    pub matrix_journey: M5CriticalJourney,
    /// Derived green/yellow/red status.
    pub status: ResumeBreadcrumbStatus,
    /// Number of provenance classes distinguished.
    pub provenance_class_count: usize,
    /// Number of lineage facets preserved.
    pub lineage_facet_count: usize,
    /// Number of declared consumer surfaces certified for this family.
    pub evaluated_surface_count: usize,
    /// Provenance-labeling posture.
    pub provenance_labeling: ProvenanceLabelingState,
    /// Lineage-breadcrumb posture.
    pub lineage_breadcrumb: LineageBreadcrumbState,
    /// Not-resumed-disclosure posture.
    pub not_resumed_disclosure: NotResumedDisclosureState,
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
/// automation reads to auto-narrow an object family's resume-breadcrumb claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeBreadcrumbDashboard {
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
    pub rows: Vec<ResumeBreadcrumbDashboardRow>,
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

impl ResumeBreadcrumbDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &ResumeBreadcrumbPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| ResumeBreadcrumbDashboardRow {
                object_family: row.object_family,
                object_label: row.object_label.clone(),
                matrix_journey: row.matrix_journey,
                status: row.derived_status,
                provenance_class_count: row.distinguished_provenance_classes.len(),
                lineage_facet_count: row.preserved_lineage_facets.len(),
                evaluated_surface_count: row.evaluated_consumer_surfaces.len(),
                provenance_labeling: row.provenance_labeling,
                lineage_breadcrumb: row.lineage_breadcrumb,
                not_resumed_disclosure: row.not_resumed_disclosure,
                capture_parity: row.capture_parity,
                headless_parity_preserved: row.headless_parity_preserved,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .breadcrumb_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_RESUME_BREADCRUMBS_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_RESUME_BREADCRUMBS_SCHEMA_VERSION,
            dashboard_id: M5_RESUME_BREADCRUMBS_DASHBOARD_ID.to_owned(),
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
        serde_json::to_string_pretty(self).expect("m5 resume-breadcrumbs dashboard serializes")
    }
}

/// Support-export wrapper for the resume-breadcrumb certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeBreadcrumbSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: ResumeBreadcrumbPacket,
    /// Dashboard quoted in full.
    pub dashboard: ResumeBreadcrumbDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl ResumeBreadcrumbSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each object family, and each
    /// active waiver id is quoted as a case id so a support reviewer — or the lifecycle automation —
    /// can name the same family and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: ResumeBreadcrumbPacket,
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
            record_kind: M5_RESUME_BREADCRUMBS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_RESUME_BREADCRUMBS_SCHEMA_VERSION,
            shared_contract_ref: M5_RESUME_BREADCRUMBS_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_resume_breadcrumbs_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResumeBreadcrumbInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen lifecycle matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family certification rows.
    pub rows: Vec<ResumeBreadcrumbRow>,
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

/// Builds a [`ResumeBreadcrumbPacket`] from the exact build identity, the frozen matrix ref, and the
/// per-family certification rows.
///
/// Each row's derived status and breadcrumb causes, the aggregate counts, the active waivers, and
/// the blocking findings are recomputed here so the packet is the single source of truth and the
/// auto-narrowing cannot be asserted.
pub fn build_m5_resume_breadcrumbs_packet(input: ResumeBreadcrumbInput) -> ResumeBreadcrumbPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<ResumeBreadcrumbRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.breadcrumb_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<ResumeBreadcrumbFinding> = Vec::new();

    // Every object family must carry a certification row.
    let present: BTreeSet<M5LifecycleObjectFamily> =
        rows.iter().map(|row| row.object_family).collect();
    for family in REQUIRED_OBJECT_FAMILIES {
        if !present.contains(&family) {
            blocking_findings.push(ResumeBreadcrumbFinding::ObjectFamilyMissing {
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
        .filter(|row| matches!(row.derived_status, ResumeBreadcrumbStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ResumeBreadcrumbStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ResumeBreadcrumbStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(ResumeBreadcrumbFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<ResumeBreadcrumbWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let breadcrumb_causes: Vec<ResumeBreadcrumbCause> = rows
        .iter()
        .flat_map(|row| row.breadcrumb_causes.clone())
        .collect();

    let required_breadcrumb_dimensions: Vec<String> = REQUIRED_BREADCRUMB_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    let required_provenance_classes: Vec<String> = REQUIRED_PROVENANCE_CLASSES
        .iter()
        .map(|class| class.as_str().to_owned())
        .collect();
    let required_lineage_facets: Vec<String> = REQUIRED_LINEAGE_FACETS
        .iter()
        .map(|facet| facet.as_str().to_owned())
        .collect();
    let required_object_families: Vec<String> = REQUIRED_OBJECT_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();

    let mut packet = ResumeBreadcrumbPacket {
        record_kind: M5_RESUME_BREADCRUMBS_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_RESUME_BREADCRUMBS_SCHEMA_VERSION,
        shared_contract_ref: M5_RESUME_BREADCRUMBS_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_RESUME_BREADCRUMBS_PACKET_ID.to_owned(),
        source_schema_ref: M5_RESUME_BREADCRUMBS_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Partial-truth and resume breadcrumbs on every long-lived M5 object: each of the \
                   thirteen governed object families certified so a degraded, resumed, or restored \
                   journey distinguishes live truth, restored context, cached evidence, and a \
                   restart-required placeholder; preserves source class, actor/subsystem, \
                   host/boundary, and checkpoint lineage instead of generic recovered wording; makes \
                   intentionally not-resumed and not-reauthorized actions explicit rather than \
                   silently absent; and preserves those breadcrumb truths through export, \
                   screenshot, and support capture — across every declared consumer surface, with \
                   the same state-truth vocabulary preserved in headless and companion-adjacent \
                   execution — and each family's green/yellow/red claim auto-narrowed from its four \
                   breadcrumb postures."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        object_state_schema_ref: M5_RESUME_BREADCRUMBS_OBJECT_STATE_SCHEMA_REF.to_owned(),
        journey_checkpoint_schema_ref: M5_RESUME_BREADCRUMBS_JOURNEY_CHECKPOINT_SCHEMA_REF.to_owned(),
        matrix_doc_ref: M5_RESUME_BREADCRUMBS_MATRIX_DOC_REF.to_owned(),
        state_object_inventory_ref: M5_RESUME_BREADCRUMBS_STATE_OBJECT_INVENTORY_REF.to_owned(),
        state_class_recovery_ref: M5_RESUME_BREADCRUMBS_STATE_CLASS_RECOVERY_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_breadcrumb_dimensions,
        required_provenance_classes,
        required_lineage_facets,
        required_object_families,
        rows,
        covered_object_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        breadcrumb_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        lifecycle_automation_refs: vec![
            "lifecycle_status.resume_breadcrumb_registry".to_owned(),
            "release_automation.auto_narrow.resume_breadcrumbs_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.resume_breadcrumbs".to_owned(),
            M5_RESUME_BREADCRUMBS_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_RESUME_BREADCRUMBS_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-resume-breadcrumbs".to_owned()],
        published_report_ref: M5_RESUME_BREADCRUMBS_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_RESUME_BREADCRUMBS_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_RESUME_BREADCRUMBS_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_RESUME_BREADCRUMBS_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(ResumeBreadcrumbFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_resume_breadcrumbs_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum ResumeBreadcrumbValidationError {
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
    /// The declared required breadcrumb dimensions do not match the lane constants.
    RequiredBreadcrumbDimensionsStale,
    /// The declared required provenance classes do not match the lane constants.
    RequiredProvenanceClassesStale,
    /// The declared required lineage facets do not match the lane constants.
    RequiredLineageFacetsStale,
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
    /// The declared breadcrumb causes do not match the recomputed causes.
    BreadcrumbCausesStale,
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

/// Validates a packet against the resume-breadcrumb certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every object family carries a
/// current certification row; each row's status is the derived auto-narrowed value, never asserted;
/// a green row cannot keep a claim while it leaves its provenance ambiguous, shows only generic
/// recovered wording, drops not-resumed actions silently, drops its breadcrumbs from capture, loses
/// headless/companion-adjacent parity, fails to distinguish all four provenance classes, fails to
/// preserve all four lineage facets, or fails to certify every declared consumer surface; and a
/// disclosed narrowing is backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_resume_breadcrumbs_packet(
    packet: &ResumeBreadcrumbPacket,
) -> Result<(), Vec<ResumeBreadcrumbValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(ResumeBreadcrumbValidationError::NoRows);
    }
    if packet.record_kind != M5_RESUME_BREADCRUMBS_PACKET_RECORD_KIND {
        errors.push(ResumeBreadcrumbValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_RESUME_BREADCRUMBS_SCHEMA_VERSION {
        errors.push(ResumeBreadcrumbValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(ResumeBreadcrumbValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(ResumeBreadcrumbValidationError::MatrixPacketRefMissing);
    }
    let expected_dimensions: Vec<String> = REQUIRED_BREADCRUMB_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    if packet.required_breadcrumb_dimensions != expected_dimensions {
        errors.push(ResumeBreadcrumbValidationError::RequiredBreadcrumbDimensionsStale);
    }
    let expected_classes: Vec<String> = REQUIRED_PROVENANCE_CLASSES
        .iter()
        .map(|class| class.as_str().to_owned())
        .collect();
    if packet.required_provenance_classes != expected_classes {
        errors.push(ResumeBreadcrumbValidationError::RequiredProvenanceClassesStale);
    }
    let expected_facets: Vec<String> = REQUIRED_LINEAGE_FACETS
        .iter()
        .map(|facet| facet.as_str().to_owned())
        .collect();
    if packet.required_lineage_facets != expected_facets {
        errors.push(ResumeBreadcrumbValidationError::RequiredLineageFacetsStale);
    }
    let expected_families: Vec<String> = REQUIRED_OBJECT_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();
    if packet.required_object_families != expected_families {
        errors.push(ResumeBreadcrumbValidationError::RequiredObjectFamiliesStale);
    }

    let present: BTreeSet<M5LifecycleObjectFamily> =
        packet.rows.iter().map(|row| row.object_family).collect();
    let coverage_complete = REQUIRED_OBJECT_FAMILIES
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != REQUIRED_OBJECT_FAMILIES.len() {
        errors.push(ResumeBreadcrumbValidationError::CoverageIncomplete);
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
        errors.push(ResumeBreadcrumbValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ResumeBreadcrumbStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ResumeBreadcrumbStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ResumeBreadcrumbStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(ResumeBreadcrumbValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<ResumeBreadcrumbWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(ResumeBreadcrumbValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<ResumeBreadcrumbCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.breadcrumb_causes {
        errors.push(ResumeBreadcrumbValidationError::BreadcrumbCausesStale);
    }

    let mut recomputed: Vec<ResumeBreadcrumbFinding> = Vec::new();
    for family in REQUIRED_OBJECT_FAMILIES {
        if !present.contains(&family) {
            recomputed.push(ResumeBreadcrumbFinding::ObjectFamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(ResumeBreadcrumbFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(ResumeBreadcrumbFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(ResumeBreadcrumbValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(ResumeBreadcrumbValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(ResumeBreadcrumbValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(ResumeBreadcrumbValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(ResumeBreadcrumbValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(ResumeBreadcrumbValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
