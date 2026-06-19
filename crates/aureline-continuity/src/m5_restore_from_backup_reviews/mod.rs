//! Restore-from-backup reviews, replay fences, and compare/export parity.
//!
//! This module gives a restored managed or support artifact a truthful
//! post-restore experience instead of implying full normal operation the moment
//! a backup or failover restore completes. Every claimed managed, self-hosted, or
//! sovereign row whose continuity depends on restored state must point to one
//! typed [`RestoreReviewEntry`] that answers the same questions everywhere:
//!
//! 1. Which artifact family was restored — managed records, policy bundles, sync
//!    metadata, or support/export records — and was this a *continuity* restore
//!    or an ordinary workspace/session restore?
//! 2. Did recovery reproduce the artifact **exactly**, or **narrower than
//!    normal**, and — when narrower — which capability class or data slice is
//!    affected, and is all replicated data present?
//! 3. Which privileged or externally mutating action lanes depend on the restored
//!    state, and is each one fenced so it cannot silently auto-replay before an
//!    explicit reviewed step?
//! 4. Can an operator compare restored-vs-current state and export that
//!    comparison before assuming full continuity?
//!
//! The descriptor is projected identically onto every claimed surface
//! (service-health, support-center, the managed action sheet, release-center, and
//! public claim-manifest generation) through a
//! [`RestoreReviewSurfaceProjection`], so the exact restore-identity, replay-fence,
//! and compare/export vocabulary stays byte-identical everywhere instead of
//! drifting per surface.
//!
//! Two guardrails are load-bearing and fail closed:
//!
//! - A review may **not** hide narrowed capability or missing replicated data
//!   behind green, full-normal status language. A review that asserts full normal
//!   status while the restore is narrower than normal — or while replicated data
//!   is incomplete — has its claim withdrawn.
//! - A managed, self-hosted, or sovereign row may **not** blur an ordinary
//!   workspace/session restore with a continuity restore. A review that labels a
//!   managed-continuity artifact family as an ordinary workspace restore is
//!   withdrawn.
//! - A privileged or externally mutating lane may **not** auto-replay after a
//!   restore. An unfenced privileged/external lane is withdrawn.
//!
//! The [`RestoreReviewRegistry`] is the typed consumer the service-health,
//! support-center, managed-action-sheet, release-center, and public claim-manifest
//! surfaces read. It indexes reviews by restored claim row and reports, per row,
//! whether a current clean review backs the claim — so any affected claim row
//! narrows automatically when post-restore truth, compare/export parity, or
//! replay-fence proof is missing.
//!
//! The record is metadata-only. It carries closed-vocabulary tokens, export-safe
//! plain-language labels, UTC timestamps, and opaque refs. Raw restored bytes, raw
//! provider payloads, raw hostnames, and secret material never cross this
//! boundary.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::m5_locality_tenant_keymode_and_drill_matrix::{
    ContinuityClaimQualificationClass, ContinuityProfileClass, RestoreIdentityClass,
};

#[cfg(test)]
mod tests;

/// Schema version carried on every record in this module.
pub const RESTORE_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const RESTORE_REVIEW_SHARED_CONTRACT_REF: &str = "continuity:m5_restore_from_backup_reviews:v1";

/// Record-kind tag for [`RestoreReviewPage`] payloads.
pub const RESTORE_REVIEW_PAGE_RECORD_KIND: &str = "restore_from_backup_review_page_record";

/// Record-kind tag for [`RestoreReviewSummary`] payloads.
pub const RESTORE_REVIEW_SUMMARY_RECORD_KIND: &str = "restore_from_backup_review_summary_record";

/// Record-kind tag for [`RestoreReviewDescriptor`] payloads.
pub const RESTORE_REVIEW_DESCRIPTOR_RECORD_KIND: &str =
    "restore_from_backup_review_descriptor_record";

/// Record-kind tag for [`RestoreReviewSurfaceProjection`] payloads.
pub const RESTORE_REVIEW_SURFACE_PROJECTION_RECORD_KIND: &str =
    "restore_from_backup_review_surface_projection_record";

/// Record-kind tag for [`RestoreReviewOutcome`] payloads.
pub const RESTORE_REVIEW_OUTCOME_RECORD_KIND: &str = "restore_from_backup_review_outcome_record";

/// Record-kind tag for [`RestoreReviewDefect`] payloads.
pub const RESTORE_REVIEW_DEFECT_RECORD_KIND: &str = "restore_from_backup_review_defect_record";

/// Record-kind tag for [`RestoreReviewRegistry`] payloads.
pub const RESTORE_REVIEW_REGISTRY_RECORD_KIND: &str = "restore_review_registry_record";

/// Record-kind tag for [`RestoreReviewCoverageRow`] payloads.
pub const RESTORE_REVIEW_COVERAGE_ROW_RECORD_KIND: &str = "restore_review_coverage_row_record";

/// Record-kind tag for [`RestoreReviewSupportExport`] payloads.
pub const RESTORE_REVIEW_SUPPORT_EXPORT_RECORD_KIND: &str =
    "restore_from_backup_review_support_export_record";

/// Repo-relative path of the canonical reviewer doc for this lane.
pub const RESTORE_REVIEW_DOC_REF: &str =
    "docs/m5/continuity/post-restore-truth-and-replay-fences.md";

/// Repo-relative path of the checked-in artifact for this lane.
pub const RESTORE_REVIEW_ARTIFACT_REF: &str =
    "artifacts/m5/continuity/post_restore_truth_and_replay_fences.md";

/// Repo-relative path of the canonical JSON schema for this lane.
pub const RESTORE_REVIEW_SCHEMA_REF: &str =
    "schemas/continuity/restore_identity_summary.schema.json";

/// Artifact family a restore review covers.
///
/// The point of an explicit family set is to make "what was restored" a typed
/// fact, and to let coverage prove compare/export parity for at least one managed
/// artifact family and one support/export family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreArtifactFamilyClass {
    /// Managed records (e.g. managed workspace or org records).
    ManagedRecord,
    /// Policy bundles distributed to managed surfaces.
    PolicyBundle,
    /// Sync metadata describing managed replication state.
    SyncMetadata,
    /// Support and export records (tickets, export packets, handoffs).
    SupportRecord,
    /// Ordinary local workspace or session state, restored on-device.
    LocalWorkspaceState,
}

impl RestoreArtifactFamilyClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedRecord => "managed_record",
            Self::PolicyBundle => "policy_bundle",
            Self::SyncMetadata => "sync_metadata",
            Self::SupportRecord => "support_record",
            Self::LocalWorkspaceState => "local_workspace_state",
        }
    }

    /// Plain-language label naming the family.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::ManagedRecord => "managed records",
            Self::PolicyBundle => "policy bundles",
            Self::SyncMetadata => "sync metadata",
            Self::SupportRecord => "support and export records",
            Self::LocalWorkspaceState => "local workspace state",
        }
    }

    /// True when this family is a managed-continuity artifact (not ordinary local state).
    pub const fn is_continuity_family(self) -> bool {
        !matches!(self, Self::LocalWorkspaceState)
    }

    /// True when this family counts as a managed artifact family for compare/export coverage.
    pub const fn is_managed_artifact_family(self) -> bool {
        matches!(
            self,
            Self::ManagedRecord | Self::PolicyBundle | Self::SyncMetadata
        )
    }

    /// True when this family counts as a support/export artifact family for compare/export coverage.
    pub const fn is_support_export_family(self) -> bool {
        matches!(self, Self::SupportRecord)
    }
}

/// Whether a restore is a continuity restore or an ordinary workspace restore.
///
/// Keeping these distinct is a guardrail: a managed, self-hosted, or sovereign row
/// may not present an ordinary workspace/session restore as continuity restore, or
/// vice versa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreLaneClass {
    /// A continuity restore from a backup, snapshot, or failover event.
    ContinuityRestore,
    /// An ordinary workspace or session restore (reopen buffers, local history).
    OrdinaryWorkspaceRestore,
}

impl RestoreLaneClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinuityRestore => "continuity_restore",
            Self::OrdinaryWorkspaceRestore => "ordinary_workspace_restore",
        }
    }

    /// Plain-language label naming the lane.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::ContinuityRestore => "continuity restore",
            Self::OrdinaryWorkspaceRestore => "ordinary workspace restore",
        }
    }
}

/// Whether recovery reproduced the artifact exactly or narrower than normal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreFidelityClass {
    /// Recovery reproduced the artifact exactly; normal operation is restored.
    ExactRestore,
    /// Recovery reproduced a narrower-than-normal subset; some capability is reduced.
    NarrowerThanNormalRestore,
    /// Restore fidelity is not disclosed; the claim must narrow.
    Undisclosed,
}

impl RestoreFidelityClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRestore => "exact_restore",
            Self::NarrowerThanNormalRestore => "narrower_than_normal_restore",
            Self::Undisclosed => "undisclosed",
        }
    }

    /// Plain-language label naming the fidelity.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::ExactRestore => "exact (normal operation restored)",
            Self::NarrowerThanNormalRestore => "narrower than normal",
            Self::Undisclosed => "not disclosed",
        }
    }

    /// True when the restore is narrower than normal and must name an affected slice.
    pub const fn is_narrower(self) -> bool {
        matches!(self, Self::NarrowerThanNormalRestore)
    }
}

/// The capability class or data slice affected when a restore is narrower than normal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AffectedSliceClass {
    /// No slice is narrowed; the restore is exact.
    NoneNarrowed,
    /// A bounded recent window of writes is missing.
    RecentWriteWindow,
    /// Queued or in-flight externally mutating actions are missing.
    QueuedExternalActions,
    /// Derived cache or index state is missing and must be rebuilt.
    DerivedCacheOrIndex,
    /// A gap in replicated history is present.
    ReplicatedHistoryGap,
    /// Recent policy bundle revisions were not replicated.
    PolicyBundleRevisionGap,
    /// Support/export records created during the outage window are pending re-ingest.
    SupportRecordGap,
}

impl AffectedSliceClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneNarrowed => "none_narrowed",
            Self::RecentWriteWindow => "recent_write_window",
            Self::QueuedExternalActions => "queued_external_actions",
            Self::DerivedCacheOrIndex => "derived_cache_or_index",
            Self::ReplicatedHistoryGap => "replicated_history_gap",
            Self::PolicyBundleRevisionGap => "policy_bundle_revision_gap",
            Self::SupportRecordGap => "support_record_gap",
        }
    }

    /// Plain-language label naming the affected slice.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::NoneNarrowed => "none narrowed",
            Self::RecentWriteWindow => "a bounded recent window of writes",
            Self::QueuedExternalActions => "queued or in-flight external actions",
            Self::DerivedCacheOrIndex => "derived cache or index state",
            Self::ReplicatedHistoryGap => "a gap in replicated history",
            Self::PolicyBundleRevisionGap => "recent policy bundle revisions",
            Self::SupportRecordGap => "support records from the outage window",
        }
    }
}

/// Replay posture of an action lane that depends on restored state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayPostureClass {
    /// A local, idempotent lane that is safe to replay automatically.
    LocalSafe,
    /// A privileged lane that must pass an explicit reviewed step before replay.
    Privileged,
    /// An externally mutating lane that must pass an explicit reviewed step before replay.
    ExternallyMutating,
}

impl ReplayPostureClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSafe => "local_safe",
            Self::Privileged => "privileged",
            Self::ExternallyMutating => "externally_mutating",
        }
    }

    /// Plain-language label naming the posture.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::LocalSafe => "local-safe",
            Self::Privileged => "privileged",
            Self::ExternallyMutating => "externally mutating",
        }
    }

    /// True when this posture must be fenced so it cannot auto-replay before review.
    pub const fn requires_fence(self) -> bool {
        matches!(self, Self::Privileged | Self::ExternallyMutating)
    }
}

/// Fence state of an action lane after a restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayFenceStateClass {
    /// The lane is held behind a fence and awaits an explicit reviewed step.
    HeldForReview,
    /// The lane was released only after an explicit reviewed step.
    ClearedAfterReview,
    /// The lane is local-safe and replays without a fence.
    NoFenceLocalSafe,
}

impl ReplayFenceStateClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeldForReview => "held_for_review",
            Self::ClearedAfterReview => "cleared_after_review",
            Self::NoFenceLocalSafe => "no_fence_local_safe",
        }
    }

    /// Plain-language label naming the fence state.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::HeldForReview => "held for review",
            Self::ClearedAfterReview => "cleared after review",
            Self::NoFenceLocalSafe => "local-safe, no fence",
        }
    }
}

/// Surface a restore-review descriptor is projected onto.
///
/// These are exactly the surfaces that reuse the review: service-health,
/// support-center, the managed action sheet, the release-center, and public
/// claim-manifest generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewSurfaceClass {
    /// The service-health surface.
    ServiceHealth,
    /// The support-center export surface.
    SupportCenter,
    /// The managed action sheet that depends on restored state.
    ManagedActionSheet,
    /// The release-center readiness surface.
    ReleaseCenter,
    /// Public claim-manifest generation.
    PublicClaimManifest,
}

impl ReviewSurfaceClass {
    /// Every surface in canonical projection order.
    pub const ALL: [ReviewSurfaceClass; 5] = [
        Self::ServiceHealth,
        Self::SupportCenter,
        Self::ManagedActionSheet,
        Self::ReleaseCenter,
        Self::PublicClaimManifest,
    ];

    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServiceHealth => "service_health",
            Self::SupportCenter => "support_center",
            Self::ManagedActionSheet => "managed_action_sheet",
            Self::ReleaseCenter => "release_center",
            Self::PublicClaimManifest => "public_claim_manifest",
        }
    }
}

/// Typed reason a restore-review claim narrowed below stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreReviewNarrowReasonClass {
    /// No narrowing is active.
    NotNarrowed,
    /// The review hides narrowed capability or missing replicated data behind green status.
    FullNormalStatusOverclaimed,
    /// A managed/self-hosted/sovereign row blurs an ordinary workspace restore with continuity restore.
    RestoreLaneConflated,
    /// A privileged or externally mutating lane is unfenced and would auto-replay.
    PrivilegedLaneAutoReplayed,
    /// A managed-continuity review does not disclose its restore fidelity.
    RestoreFidelityUndisclosed,
    /// A narrower-than-normal restore does not name the affected capability class or data slice.
    AffectedSliceUnnamed,
    /// A managed-continuity review does not declare the restore identity recovery reproduces.
    RestoreIdentityUndeclared,
    /// A fenced lane was cleared but names no explicit reviewed step.
    ReplayFenceReviewMissing,
    /// A managed-continuity review cannot export its restored-vs-current comparison.
    ExportParityMissing,
    /// A review is not projected onto every required surface.
    SurfaceReuseIncomplete,
    /// A managed-continuity review cannot compare restored-vs-current state.
    CompareParityMissing,
    /// No managed artifact family carries compare/export parity.
    ManagedFamilyCompareCoverageMissing,
    /// No support/export artifact family carries compare/export parity.
    SupportFamilyCompareCoverageMissing,
    /// A surface renders different restore-identity, replay-fence, or compare/export vocabulary.
    ReviewVocabularyDrift,
    /// A claimed restored row has no restore review at all.
    ReviewEvidenceMissing,
}

impl RestoreReviewNarrowReasonClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::FullNormalStatusOverclaimed => "full_normal_status_overclaimed",
            Self::RestoreLaneConflated => "restore_lane_conflated",
            Self::PrivilegedLaneAutoReplayed => "privileged_lane_auto_replayed",
            Self::RestoreFidelityUndisclosed => "restore_fidelity_undisclosed",
            Self::AffectedSliceUnnamed => "affected_slice_unnamed",
            Self::RestoreIdentityUndeclared => "restore_identity_undeclared",
            Self::ReplayFenceReviewMissing => "replay_fence_review_missing",
            Self::ExportParityMissing => "export_parity_missing",
            Self::SurfaceReuseIncomplete => "surface_reuse_incomplete",
            Self::CompareParityMissing => "compare_parity_missing",
            Self::ManagedFamilyCompareCoverageMissing => "managed_family_compare_coverage_missing",
            Self::SupportFamilyCompareCoverageMissing => "support_family_compare_coverage_missing",
            Self::ReviewVocabularyDrift => "review_vocabulary_drift",
            Self::ReviewEvidenceMissing => "review_evidence_missing",
        }
    }

    /// True when this reason withdraws the claim immediately (fails closed).
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::FullNormalStatusOverclaimed
                | Self::RestoreLaneConflated
                | Self::PrivilegedLaneAutoReplayed
        )
    }

    /// True when this reason holds the claim at preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(
            self,
            Self::CompareParityMissing
                | Self::ManagedFamilyCompareCoverageMissing
                | Self::SupportFamilyCompareCoverageMissing
                | Self::ReviewVocabularyDrift
                | Self::ReviewEvidenceMissing
        )
    }
}

/// Coverage state of a claimed restored row, derived from its review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewCoverageClass {
    /// A current, clean review backs the claim.
    CurrentReview,
    /// A review backs the claim but it narrowed and needs attention.
    NarrowedReviewNeedsAttention,
    /// A review backs the claim but its claim is withheld (fails closed).
    ReviewWithheld,
    /// No restore review backs the claim at all.
    NoReview,
}

impl ReviewCoverageClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentReview => "current_review",
            Self::NarrowedReviewNeedsAttention => "narrowed_review_needs_attention",
            Self::ReviewWithheld => "review_withheld",
            Self::NoReview => "no_review",
        }
    }

    /// True when the claim is backed by a current, clean review.
    pub const fn is_covered(self) -> bool {
        matches!(self, Self::CurrentReview)
    }
}

/// Derives a qualification from the restore-review narrow reasons present.
fn qualification_from_reasons<'a>(
    reasons: impl IntoIterator<Item = &'a RestoreReviewNarrowReasonClass>,
) -> ContinuityClaimQualificationClass {
    let mut saw_any = false;
    let mut saw_preview = false;
    for reason in reasons {
        saw_any = true;
        if reason.is_withdrawal_reason() {
            return ContinuityClaimQualificationClass::Withdrawn;
        }
        if reason.is_preview_reason() {
            saw_preview = true;
        }
    }
    if saw_preview {
        ContinuityClaimQualificationClass::Preview
    } else if saw_any {
        ContinuityClaimQualificationClass::Beta
    } else {
        ContinuityClaimQualificationClass::Stable
    }
}

/// The post-restore truth summary for one restored artifact.
///
/// This distinguishes exact restoration from narrower-than-normal restoration,
/// names the affected capability class or data slice when narrower, and records
/// whether the review is permitted to present full-normal status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreIdentitySummary {
    /// Whether recovery reproduced the artifact exactly or narrower than normal.
    pub restore_fidelity: RestoreFidelityClass,
    /// Stable token for [`Self::restore_fidelity`].
    pub restore_fidelity_token: String,
    /// Identity a successful restore reproduces.
    pub restore_identity: RestoreIdentityClass,
    /// Stable token for [`Self::restore_identity`].
    pub restore_identity_token: String,
    /// The capability class or data slice affected when narrower than normal.
    pub affected_slice: AffectedSliceClass,
    /// Stable token for [`Self::affected_slice`].
    pub affected_slice_token: String,
    /// Export-safe note naming what restored narrower than normal or is missing.
    pub affected_slice_note: String,
    /// True when all replicated data is present after the restore.
    pub replicated_data_complete: bool,
    /// True when the review presents full, normal status to the user.
    pub asserts_full_normal_status: bool,
}

impl RestoreIdentitySummary {
    /// Builds a restore-identity summary, computing its tokens.
    pub fn new(
        restore_fidelity: RestoreFidelityClass,
        restore_identity: RestoreIdentityClass,
        affected_slice: AffectedSliceClass,
        affected_slice_note: impl Into<String>,
        replicated_data_complete: bool,
        asserts_full_normal_status: bool,
    ) -> Self {
        Self {
            restore_fidelity,
            restore_fidelity_token: restore_fidelity.as_str().to_owned(),
            restore_identity,
            restore_identity_token: restore_identity.as_str().to_owned(),
            affected_slice,
            affected_slice_token: affected_slice.as_str().to_owned(),
            affected_slice_note: affected_slice_note.into(),
            replicated_data_complete,
            asserts_full_normal_status,
        }
    }

    /// True when the review hides narrowed capability or missing data behind green status.
    pub fn overclaims_full_normal_status(&self) -> bool {
        self.asserts_full_normal_status
            && (self.restore_fidelity.is_narrower() || !self.replicated_data_complete)
    }

    /// True when a narrower-than-normal restore fails to name its affected slice.
    pub fn missing_affected_slice(&self) -> bool {
        self.restore_fidelity.is_narrower()
            && (self.affected_slice == AffectedSliceClass::NoneNarrowed
                || self.affected_slice_note.trim().is_empty())
    }

    /// Canonical one-line restore-identity summary reused by every surface projection.
    pub fn summary_line(&self) -> String {
        let affected = if self.affected_slice == AffectedSliceClass::NoneNarrowed
            && self.affected_slice_note.trim().is_empty()
        {
            "none narrowed".to_owned()
        } else {
            let note = self.affected_slice_note.trim();
            if note.is_empty() {
                self.affected_slice.plain().to_owned()
            } else {
                format!("{} — {}", self.affected_slice.plain(), note)
            }
        };
        format!(
            "Restore {}; identity {}; affected: {}; replicated data {}.",
            self.restore_fidelity.plain(),
            restore_identity_plain(self.restore_identity),
            affected,
            if self.replicated_data_complete {
                "complete"
            } else {
                "incomplete"
            }
        )
    }
}

/// One privileged or externally mutating action lane that depends on restored state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayFence {
    /// Opaque identifier for the action lane.
    pub lane_id: String,
    /// Reviewable label naming the action lane.
    pub lane_label: String,
    /// Replay posture of the lane.
    pub replay_posture: ReplayPostureClass,
    /// Stable token for [`Self::replay_posture`].
    pub replay_posture_token: String,
    /// Fence state of the lane after the restore.
    pub fence_state: ReplayFenceStateClass,
    /// Stable token for [`Self::fence_state`].
    pub fence_state_token: String,
    /// Opaque ref to the explicit reviewed step, empty when none.
    pub reviewed_step_ref: String,
}

impl ReplayFence {
    /// Builds a replay-fence record, computing its tokens.
    pub fn new(
        lane_id: impl Into<String>,
        lane_label: impl Into<String>,
        replay_posture: ReplayPostureClass,
        fence_state: ReplayFenceStateClass,
        reviewed_step_ref: impl Into<String>,
    ) -> Self {
        Self {
            lane_id: lane_id.into(),
            lane_label: lane_label.into(),
            replay_posture,
            replay_posture_token: replay_posture.as_str().to_owned(),
            fence_state,
            fence_state_token: fence_state.as_str().to_owned(),
            reviewed_step_ref: reviewed_step_ref.into(),
        }
    }

    /// True when a fence-required lane is unfenced and would auto-replay.
    pub fn auto_replays_unsafely(&self) -> bool {
        self.replay_posture.requires_fence()
            && self.fence_state == ReplayFenceStateClass::NoFenceLocalSafe
    }

    /// True when a cleared fence-required lane names no explicit reviewed step.
    pub fn missing_review_ref(&self) -> bool {
        self.replay_posture.requires_fence()
            && self.fence_state == ReplayFenceStateClass::ClearedAfterReview
            && self.reviewed_step_ref.trim().is_empty()
    }

    /// True when this lane will not auto-replay before an explicit reviewed step.
    pub fn auto_replay_blocked(&self) -> bool {
        !self.replay_posture.requires_fence()
            || matches!(
                self.fence_state,
                ReplayFenceStateClass::HeldForReview | ReplayFenceStateClass::ClearedAfterReview
            )
    }
}

/// Restored-vs-current compare and export availability for one restored artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareExportParity {
    /// True when an operator can compare restored-vs-current state.
    pub restored_vs_current_available: bool,
    /// Opaque ref to the restored-vs-current comparison, empty when unavailable.
    pub compare_ref: String,
    /// True when an operator can export the restored-vs-current comparison.
    pub export_available: bool,
    /// Opaque ref to the exported comparison, empty when unavailable.
    pub export_ref: String,
}

impl CompareExportParity {
    /// Builds a compare/export parity record.
    pub fn new(
        restored_vs_current_available: bool,
        compare_ref: impl Into<String>,
        export_available: bool,
        export_ref: impl Into<String>,
    ) -> Self {
        Self {
            restored_vs_current_available,
            compare_ref: compare_ref.into(),
            export_available,
            export_ref: export_ref.into(),
        }
    }

    /// True when both compare and export of restored-vs-current state are available.
    pub fn is_full_parity(&self) -> bool {
        self.restored_vs_current_available && self.export_available
    }

    /// Canonical one-line compare/export summary reused by every surface projection.
    pub fn parity_line(&self) -> String {
        format!(
            "Restored-vs-current compare {}; export {}.",
            if self.restored_vs_current_available {
                "available"
            } else {
                "unavailable"
            },
            if self.export_available {
                "available"
            } else {
                "unavailable"
            }
        )
    }
}

/// One restored artifact review decorated with its post-restore truth facts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreReviewEntry {
    /// Opaque review identifier.
    pub review_id: String,
    /// Opaque id of the claimed restored row this review backs.
    pub claim_row_id: String,
    /// Reviewable label naming the restored artifact.
    pub surface_label: String,
    /// Claimed deployment profile.
    pub profile_class: ContinuityProfileClass,
    /// Stable token for [`Self::profile_class`].
    pub profile_class_token: String,
    /// Artifact family this review covers.
    pub artifact_family: RestoreArtifactFamilyClass,
    /// Stable token for [`Self::artifact_family`].
    pub artifact_family_token: String,
    /// Whether this was a continuity restore or an ordinary workspace restore.
    pub restore_lane: RestoreLaneClass,
    /// Stable token for [`Self::restore_lane`].
    pub restore_lane_token: String,
    /// The post-restore truth summary.
    pub identity_summary: RestoreIdentitySummary,
    /// Replay fences for the privileged or externally mutating lanes that depend on restored state.
    pub replay_fences: Vec<ReplayFence>,
    /// Restored-vs-current compare and export availability.
    pub compare_export: CompareExportParity,
    /// Surfaces this review is projected onto.
    pub projected_surfaces: Vec<ReviewSurfaceClass>,
}

impl RestoreReviewEntry {
    /// Builds a restore-review entry.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        review_id: impl Into<String>,
        claim_row_id: impl Into<String>,
        surface_label: impl Into<String>,
        profile_class: ContinuityProfileClass,
        artifact_family: RestoreArtifactFamilyClass,
        restore_lane: RestoreLaneClass,
        identity_summary: RestoreIdentitySummary,
        replay_fences: Vec<ReplayFence>,
        compare_export: CompareExportParity,
        projected_surfaces: Vec<ReviewSurfaceClass>,
    ) -> Self {
        Self {
            review_id: review_id.into(),
            claim_row_id: claim_row_id.into(),
            surface_label: surface_label.into(),
            profile_class,
            profile_class_token: profile_class.as_str().to_owned(),
            artifact_family,
            artifact_family_token: artifact_family.as_str().to_owned(),
            restore_lane,
            restore_lane_token: restore_lane.as_str().to_owned(),
            identity_summary,
            replay_fences,
            compare_export,
            projected_surfaces,
        }
    }

    /// Surfaces this review is required to reach (every surface).
    pub fn required_surfaces(&self) -> &'static [ReviewSurfaceClass] {
        &ReviewSurfaceClass::ALL
    }

    /// True when this review is held to managed-continuity post-restore requirements.
    ///
    /// A continuity restore of a managed or support artifact family on a
    /// non-local profile must carry typed post-restore truth, replay fences, and
    /// compare/export parity. An ordinary local restore is exempt.
    pub fn requires_managed_review(&self) -> bool {
        self.profile_class != ContinuityProfileClass::LocalOnly
            && self.restore_lane == RestoreLaneClass::ContinuityRestore
            && self.artifact_family.is_continuity_family()
    }

    /// True when a managed/self-hosted/sovereign row blurs ordinary restore with continuity restore.
    pub fn conflates_restore_lane(&self) -> bool {
        self.profile_class != ContinuityProfileClass::LocalOnly
            && self.restore_lane == RestoreLaneClass::OrdinaryWorkspaceRestore
            && self.artifact_family.is_continuity_family()
    }

    /// True when any privileged or externally mutating lane is set to auto-replay.
    pub fn has_unsafe_auto_replay(&self) -> bool {
        self.replay_fences
            .iter()
            .any(|fence| fence.auto_replays_unsafely())
    }
}

/// Plain-language descriptor for one restore review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreReviewDescriptor {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque descriptor identifier.
    pub descriptor_id: String,
    /// Review this descriptor describes.
    pub review_id: String,
    /// Claim row this review backs.
    pub claim_row_id: String,
    /// Reviewable label naming the restored artifact.
    pub surface_label: String,
    /// Stable token for the claimed profile.
    pub profile_class_token: String,
    /// Plain-language claimed profile.
    pub profile_class_plain: String,
    /// Stable token for the artifact family.
    pub artifact_family_token: String,
    /// Plain-language artifact family.
    pub artifact_family_plain: String,
    /// Stable token for the restore lane.
    pub restore_lane_token: String,
    /// Plain-language restore lane.
    pub restore_lane_plain: String,
    /// Stable token for the restore fidelity.
    pub restore_fidelity_token: String,
    /// Plain-language restore fidelity.
    pub restore_fidelity_plain: String,
    /// Stable token for the restore identity.
    pub restore_identity_token: String,
    /// Stable token for the affected slice.
    pub affected_slice_token: String,
    /// True when all replicated data is present after the restore.
    pub replicated_data_complete: bool,
    /// True when privileged and externally mutating lanes do not auto-replay.
    pub privileged_lanes_fenced: bool,
    /// True when restored-vs-current compare and export are both available.
    pub compare_export_available: bool,
    /// Canonical one-line restore-identity summary reused by every surface projection.
    pub restore_summary_line: String,
    /// Canonical one-line replay-fence summary reused by every surface projection.
    pub replay_fence_line: String,
    /// Canonical one-line compare/export summary reused by every surface projection.
    pub compare_export_line: String,
}

impl RestoreReviewDescriptor {
    /// Builds a descriptor from a decorated review entry.
    pub fn from_entry(entry: &RestoreReviewEntry) -> Self {
        Self {
            record_kind: RESTORE_REVIEW_DESCRIPTOR_RECORD_KIND.to_owned(),
            schema_version: RESTORE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: RESTORE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            descriptor_id: format!("continuity:restore-review-descriptor:{}", entry.review_id),
            review_id: entry.review_id.clone(),
            claim_row_id: entry.claim_row_id.clone(),
            surface_label: entry.surface_label.clone(),
            profile_class_token: entry.profile_class_token.clone(),
            profile_class_plain: profile_plain(entry.profile_class).to_owned(),
            artifact_family_token: entry.artifact_family_token.clone(),
            artifact_family_plain: entry.artifact_family.plain().to_owned(),
            restore_lane_token: entry.restore_lane_token.clone(),
            restore_lane_plain: entry.restore_lane.plain().to_owned(),
            restore_fidelity_token: entry.identity_summary.restore_fidelity_token.clone(),
            restore_fidelity_plain: entry.identity_summary.restore_fidelity.plain().to_owned(),
            restore_identity_token: entry.identity_summary.restore_identity_token.clone(),
            affected_slice_token: entry.identity_summary.affected_slice_token.clone(),
            replicated_data_complete: entry.identity_summary.replicated_data_complete,
            privileged_lanes_fenced: !entry.has_unsafe_auto_replay(),
            compare_export_available: entry.compare_export.is_full_parity(),
            restore_summary_line: entry.identity_summary.summary_line(),
            replay_fence_line: replay_fence_line(entry),
            compare_export_line: entry.compare_export.parity_line(),
        }
    }
}

/// One surface rendering of a restore-review descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreReviewSurfaceProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Surface this projection renders on.
    pub surface: ReviewSurfaceClass,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Review this projection describes.
    pub review_id: String,
    /// Descriptor id rendered on this surface.
    pub descriptor_id: String,
    /// Restore-identity summary line rendered on this surface.
    pub restore_summary_line: String,
    /// Replay-fence summary line rendered on this surface.
    pub replay_fence_line: String,
    /// Compare/export summary line rendered on this surface.
    pub compare_export_line: String,
}

/// Per-review verdict joining a review to its computed qualification and reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreReviewOutcome {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Review this outcome describes.
    pub review_id: String,
    /// Claim row this review backs.
    pub claim_row_id: String,
    /// Stable token for the artifact family.
    pub artifact_family_token: String,
    /// Computed qualification token for the review.
    pub qualification_token: String,
    /// True when the review narrowed below stable.
    pub narrowed: bool,
    /// True when the review's claim is withheld entirely.
    pub claim_withheld: bool,
    /// Stable token for the restore fidelity.
    pub restore_fidelity_token: String,
    /// Stable token for the restore identity.
    pub restore_identity_token: String,
    /// True when privileged and externally mutating lanes do not auto-replay.
    pub privileged_lanes_fenced: bool,
    /// True when restored-vs-current compare and export are both available.
    pub compare_export_available: bool,
    /// Stable narrow-reason tokens that applied to the review.
    pub narrow_reason_tokens: Vec<String>,
}

/// One claimed restored row's coverage verdict, derived from its review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreReviewCoverageRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Claim row this coverage row describes.
    pub claim_row_id: String,
    /// Coverage class derived from the backing review.
    pub coverage_class: ReviewCoverageClass,
    /// Stable token for [`Self::coverage_class`].
    pub coverage_class_token: String,
    /// Review id backing the claim, empty when none.
    pub review_id: String,
    /// Computed qualification token for the coverage.
    pub qualification_token: String,
    /// True when a current, clean review backs the claim.
    pub covered: bool,
    /// True when the coverage narrowed below stable.
    pub narrowed: bool,
}

/// Typed consumer that indexes restore reviews by claim row.
///
/// The service-health, support-center, managed-action-sheet, release-center, and
/// public claim-manifest surfaces read this registry instead of re-deriving
/// post-restore coverage by hand. It reports, per claimed restored row, whether a
/// current clean review backs the claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreReviewRegistry {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable registry identifier.
    pub registry_id: String,
    /// Per-claim-row coverage rows.
    pub coverage: Vec<RestoreReviewCoverageRow>,
    /// Claim row ids that point to a current clean review.
    pub covered_claim_row_ids: Vec<String>,
    /// Claim row ids that narrowed because their review is narrowed, withheld, or missing.
    pub uncovered_claim_row_ids: Vec<String>,
    /// True when at least one managed artifact family carries compare/export parity.
    pub managed_family_compare_covered: bool,
    /// True when at least one support/export artifact family carries compare/export parity.
    pub support_family_compare_covered: bool,
}

impl RestoreReviewRegistry {
    /// Builds a registry from a finished page's input and outcomes.
    pub fn from_page(page: &RestoreReviewPage) -> Self {
        build_registry(&page.input, &page.outcomes)
    }

    /// Returns the coverage row for a claim row id, if present.
    pub fn coverage_for_claim_row(&self, claim_row_id: &str) -> Option<&RestoreReviewCoverageRow> {
        self.coverage
            .iter()
            .find(|row| row.claim_row_id == claim_row_id)
    }

    /// True when a current clean review backs the claim row.
    pub fn is_claim_row_covered(&self, claim_row_id: &str) -> bool {
        self.coverage_for_claim_row(claim_row_id)
            .is_some_and(|row| row.covered)
    }

    /// Number of claim rows backed by a current clean review.
    pub fn covered_claim_count(&self) -> usize {
        self.covered_claim_row_ids.len()
    }

    /// True when every tracked claim row points to a current clean review.
    pub fn all_claims_covered(&self) -> bool {
        self.uncovered_claim_row_ids.is_empty()
    }
}

/// Typed defect emitted by the restore-review audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreReviewDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque defect identifier.
    pub defect_id: String,
    /// Typed narrow reason.
    pub narrow_reason: RestoreReviewNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Opaque source review id or claim row that triggered the defect.
    pub source: String,
    /// Export-safe explanation of the defect.
    pub note: String,
}

impl RestoreReviewDefect {
    fn new(
        narrow_reason: RestoreReviewNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source = source.into();
        Self {
            record_kind: RESTORE_REVIEW_DEFECT_RECORD_KIND.to_owned(),
            schema_version: RESTORE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: RESTORE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "continuity:defect:restore-review:{}:{}",
                narrow_reason.as_str(),
                source
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source,
            note: note.into(),
        }
    }
}

/// Aggregate summary for a restore-review page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreReviewSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Overall qualification for the page.
    pub overall_qualification_token: String,
    /// Number of reviews.
    pub review_count: usize,
    /// Number of distinct artifact families covered.
    pub family_count: usize,
    /// Number of managed-continuity reviews.
    pub managed_review_count: usize,
    /// Number of exact-restore reviews.
    pub exact_restore_count: usize,
    /// Number of narrower-than-normal reviews.
    pub narrower_than_normal_count: usize,
    /// Number of replay fences across all reviews.
    pub replay_fence_count: usize,
    /// Number of privileged or externally mutating lanes that are fenced.
    pub fenced_privileged_lane_count: usize,
    /// Number of reviews whose restored-vs-current compare and export are both available.
    pub compare_export_available_count: usize,
    /// Number of reviews that narrowed below stable.
    pub narrowed_count: usize,
    /// Number of reviews whose claim is withheld.
    pub withdrawn_count: usize,
    /// Number of tracked claim rows.
    pub claim_coverage_count: usize,
    /// Number of claim rows backed by a current clean review.
    pub covered_claim_count: usize,
    /// Number of claim rows that narrowed for lack of a current clean review.
    pub uncovered_claim_count: usize,
    /// Number of surface projections emitted.
    pub surface_projection_count: usize,
    /// True when every surface renders the same restore-identity/replay-fence/compare vocabulary.
    pub vocabulary_consistent: bool,
    /// True when no privileged or externally mutating lane auto-replays.
    pub no_unsafe_auto_replay: bool,
    /// True when no review hides narrowed capability behind green status.
    pub no_full_normal_status_overclaim: bool,
    /// True when no managed/self-hosted/sovereign row blurs ordinary with continuity restore.
    pub no_restore_lane_conflation: bool,
    /// True when every narrower-than-normal restore names its affected slice.
    pub all_narrower_restores_name_affected_slice: bool,
    /// True when at least one managed artifact family carries compare/export parity.
    pub managed_family_compare_covered: bool,
    /// True when at least one support/export artifact family carries compare/export parity.
    pub support_family_compare_covered: bool,
    /// True when every tracked claim row points to a current clean review.
    pub all_expected_claims_covered: bool,
    /// True when restore-identity and replay-fence fields are export-safe by default.
    pub restore_truth_export_safe: bool,
    /// True when no raw provider payload is carried anywhere in the review.
    pub raw_payloads_excluded: bool,
    /// Number of defects recorded for the page.
    pub defect_count: usize,
}

/// Full auditable input for a restore-review page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreReviewInput {
    /// Reviewable label for the page.
    pub input_label: String,
    /// Claimed restore reviews.
    pub reviews: Vec<RestoreReviewEntry>,
    /// Claim rows that depend on restored state and must point to a current clean review.
    pub expected_claim_row_ids: Vec<String>,
}

/// Canonical proof packet for the restore-from-backup review lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreReviewPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page identifier.
    pub page_id: String,
    /// Reviewable page label.
    pub page_label: String,
    /// UTC timestamp when the packet was generated.
    pub generated_at: String,
    /// Aggregate summary derived from the embedded input and defects.
    pub summary: RestoreReviewSummary,
    /// Typed defects for the packet.
    pub defects: Vec<RestoreReviewDefect>,
    /// Plain-language descriptors, one per review.
    pub descriptors: Vec<RestoreReviewDescriptor>,
    /// Per-surface projections proving identical vocabulary across surfaces.
    pub surface_projections: Vec<RestoreReviewSurfaceProjection>,
    /// Per-review verdicts joining each review to its computed qualification.
    pub outcomes: Vec<RestoreReviewOutcome>,
    /// The typed consumer registry of claim-row coverage.
    pub registry: RestoreReviewRegistry,
    /// The audited input embedded as evidence.
    pub input: RestoreReviewInput,
}

impl RestoreReviewPage {
    /// Builds a restore-review page from the supplied input.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        input: RestoreReviewInput,
    ) -> Self {
        let descriptors: Vec<RestoreReviewDescriptor> = input
            .reviews
            .iter()
            .map(RestoreReviewDescriptor::from_entry)
            .collect();
        let surface_projections = build_surface_projections(&input.reviews);
        let defects = audit(&input, &surface_projections);
        let outcomes = build_outcomes(&input, &defects);
        let registry = build_registry(&input, &outcomes);
        let summary = build_summary(&input, &surface_projections, &outcomes, &registry, &defects);
        Self {
            record_kind: RESTORE_REVIEW_PAGE_RECORD_KIND.to_owned(),
            schema_version: RESTORE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: RESTORE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            defects,
            descriptors,
            surface_projections,
            outcomes,
            registry,
            input,
        }
    }

    /// True when the page qualifies stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == ContinuityClaimQualificationClass::Stable.as_str()
    }

    /// True when every surface renders identical restore-identity/replay-fence/compare vocabulary.
    pub fn surfaces_share_vocabulary(&self) -> bool {
        self.summary.vocabulary_consistent
    }

    /// True when every tracked claim row points to a current clean review.
    pub fn every_claim_covered(&self) -> bool {
        self.summary.all_expected_claims_covered
    }

    /// Returns the descriptor for a review id, if present.
    pub fn descriptor(&self, review_id: &str) -> Option<&RestoreReviewDescriptor> {
        self.descriptors.iter().find(|d| d.review_id == review_id)
    }

    /// Returns the computed outcome for a review id, if present.
    pub fn outcome(&self, review_id: &str) -> Option<&RestoreReviewOutcome> {
        self.outcomes.iter().find(|o| o.review_id == review_id)
    }
}

/// Support-export wrapper for the restore-review page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreReviewSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export identifier.
    pub export_id: String,
    /// UTC timestamp when the export was produced.
    pub generated_at: String,
    /// The restore-review page embedded as evidence.
    pub page: RestoreReviewPage,
    /// Typed narrow reasons present in the embedded packet.
    pub narrow_reasons_present: Vec<RestoreReviewNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when restore-identity and replay-fence fields are export-safe by default.
    pub restore_truth_export_safe: bool,
    /// True when raw provider payloads are excluded from this export.
    pub raw_payloads_excluded: bool,
}

impl RestoreReviewSupportExport {
    /// Wraps a restore-review page inside a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: RestoreReviewPage,
    ) -> Self {
        let mut reasons: Vec<RestoreReviewNarrowReasonClass> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }
        reasons.sort();
        Self {
            record_kind: RESTORE_REVIEW_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: RESTORE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: RESTORE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            restore_truth_export_safe: true,
            raw_payloads_excluded: true,
        }
    }
}

/// Re-runs the restore-review audit over a page, including its projections.
///
/// Unlike [`RestoreReviewPage::new`], this validates the page's stored surface
/// projections against freshly derived canonical lines, so a tampered projection
/// (one that renders different vocabulary than its descriptor) is caught on
/// re-validation.
pub fn audit_restore_review_page(page: &RestoreReviewPage) -> Vec<RestoreReviewDefect> {
    audit(&page.input, &page.surface_projections)
}

/// Validates a restore-review page and returns `Ok(())` when clean.
pub fn validate_restore_review_page(
    page: &RestoreReviewPage,
) -> Result<(), Vec<RestoreReviewDefect>> {
    let defects = audit_restore_review_page(page);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Returns the seeded stable restore-review page.
pub fn seeded_restore_review_page() -> RestoreReviewPage {
    RestoreReviewPage::new(
        "continuity:restore-from-backup-review:seeded",
        "Restore-from-backup reviews, replay fences, and compare/export parity",
        "2026-06-01T00:00:00Z",
        seeded_restore_review_input(),
    )
}

/// Returns the seeded input used by the canonical restore-review page.
///
/// The seeded page carries one continuity-restore review for each managed
/// artifact family — managed records, policy bundles, and sync metadata — plus a
/// support/export-record review and an ordinary local workspace restore. Every
/// managed-continuity review distinguishes exact from narrower-than-normal restore
/// identity, names the affected slice when narrower, fences its privileged and
/// externally mutating lanes, and offers restored-vs-current compare and export.
/// Every claimed restored row points to a current clean review, so the page
/// qualifies stable.
pub fn seeded_restore_review_input() -> RestoreReviewInput {
    let all = ReviewSurfaceClass::ALL.to_vec();
    let reviews = vec![
        RestoreReviewEntry::new(
            "continuity-restore:managed-records",
            "continuity:row:managed-records-restore",
            "Managed records restore",
            ContinuityProfileClass::Managed,
            RestoreArtifactFamilyClass::ManagedRecord,
            RestoreLaneClass::ContinuityRestore,
            RestoreIdentitySummary::new(
                RestoreFidelityClass::ExactRestore,
                RestoreIdentityClass::SameIdentityRestore,
                AffectedSliceClass::NoneNarrowed,
                "",
                true,
                true,
            ),
            vec![
                ReplayFence::new(
                    "lane:managed-records:admin-policy-apply",
                    "Administrative policy apply",
                    ReplayPostureClass::Privileged,
                    ReplayFenceStateClass::HeldForReview,
                    "",
                ),
                ReplayFence::new(
                    "lane:managed-records:outbound-webhook-redelivery",
                    "Outbound webhook redelivery",
                    ReplayPostureClass::ExternallyMutating,
                    ReplayFenceStateClass::HeldForReview,
                    "",
                ),
                ReplayFence::new(
                    "lane:managed-records:reopen-record-views",
                    "Reopen record views",
                    ReplayPostureClass::LocalSafe,
                    ReplayFenceStateClass::NoFenceLocalSafe,
                    "",
                ),
            ],
            CompareExportParity::new(
                true,
                "compare:managed-records:restored-vs-current:2026-06-01",
                true,
                "export:managed-records:restored-vs-current:2026-06-01",
            ),
            all.clone(),
        ),
        RestoreReviewEntry::new(
            "continuity-restore:policy-bundle",
            "continuity:row:policy-bundle-restore",
            "Policy bundle restore",
            ContinuityProfileClass::Managed,
            RestoreArtifactFamilyClass::PolicyBundle,
            RestoreLaneClass::ContinuityRestore,
            RestoreIdentitySummary::new(
                RestoreFidelityClass::NarrowerThanNormalRestore,
                RestoreIdentityClass::ReissuedIdentityRestore,
                AffectedSliceClass::PolicyBundleRevisionGap,
                "the two most recent policy bundle revisions were not replicated and must be re-pushed before activation",
                false,
                false,
            ),
            vec![ReplayFence::new(
                "lane:policy-bundle:activation",
                "Policy bundle activation",
                ReplayPostureClass::Privileged,
                ReplayFenceStateClass::ClearedAfterReview,
                "review-step:policy-bundle-activation:2026-06-01",
            )],
            CompareExportParity::new(
                true,
                "compare:policy-bundle:restored-vs-current:2026-06-01",
                true,
                "export:policy-bundle:restored-vs-current:2026-06-01",
            ),
            all.clone(),
        ),
        RestoreReviewEntry::new(
            "continuity-restore:sync-metadata",
            "continuity:row:sync-metadata-restore",
            "Sync metadata restore",
            ContinuityProfileClass::SelfHosted,
            RestoreArtifactFamilyClass::SyncMetadata,
            RestoreLaneClass::ContinuityRestore,
            RestoreIdentitySummary::new(
                RestoreFidelityClass::ExactRestore,
                RestoreIdentityClass::SameIdentityRestore,
                AffectedSliceClass::NoneNarrowed,
                "",
                true,
                true,
            ),
            vec![ReplayFence::new(
                "lane:sync-metadata:push-to-peers",
                "Sync push to peers",
                ReplayPostureClass::ExternallyMutating,
                ReplayFenceStateClass::HeldForReview,
                "",
            )],
            CompareExportParity::new(
                true,
                "compare:sync-metadata:restored-vs-current:2026-06-01",
                true,
                "export:sync-metadata:restored-vs-current:2026-06-01",
            ),
            all.clone(),
        ),
        RestoreReviewEntry::new(
            "continuity-restore:support-records",
            "continuity:row:support-records-restore",
            "Support and export records restore",
            ContinuityProfileClass::Managed,
            RestoreArtifactFamilyClass::SupportRecord,
            RestoreLaneClass::ContinuityRestore,
            RestoreIdentitySummary::new(
                RestoreFidelityClass::NarrowerThanNormalRestore,
                RestoreIdentityClass::SameIdentityRestore,
                AffectedSliceClass::SupportRecordGap,
                "support tickets and export packets created during the outage window are pending re-ingest",
                false,
                false,
            ),
            vec![ReplayFence::new(
                "lane:support-records:re-export",
                "Support record re-export",
                ReplayPostureClass::Privileged,
                ReplayFenceStateClass::ClearedAfterReview,
                "review-step:support-record-re-export:2026-06-01",
            )],
            CompareExportParity::new(
                true,
                "compare:support-records:restored-vs-current:2026-06-01",
                true,
                "export:support-records:restored-vs-current:2026-06-01",
            ),
            all.clone(),
        ),
        RestoreReviewEntry::new(
            "ordinary-restore:local-workspace",
            "continuity:row:local-workspace-restore",
            "Local workspace restore",
            ContinuityProfileClass::LocalOnly,
            RestoreArtifactFamilyClass::LocalWorkspaceState,
            RestoreLaneClass::OrdinaryWorkspaceRestore,
            RestoreIdentitySummary::new(
                RestoreFidelityClass::ExactRestore,
                RestoreIdentityClass::NotApplicable,
                AffectedSliceClass::NoneNarrowed,
                "",
                true,
                true,
            ),
            vec![ReplayFence::new(
                "lane:local-workspace:reopen-recent-files",
                "Reopen recent files",
                ReplayPostureClass::LocalSafe,
                ReplayFenceStateClass::NoFenceLocalSafe,
                "",
            )],
            CompareExportParity::new(false, "", false, ""),
            all,
        ),
    ];
    RestoreReviewInput {
        input_label: "Restore-from-backup reviews across managed records, policy bundles, sync metadata, support records, and local workspace state".to_owned(),
        expected_claim_row_ids: vec![
            "continuity:row:managed-records-restore".to_owned(),
            "continuity:row:policy-bundle-restore".to_owned(),
            "continuity:row:sync-metadata-restore".to_owned(),
            "continuity:row:support-records-restore".to_owned(),
        ],
        reviews,
    }
}

fn audit(
    input: &RestoreReviewInput,
    projections: &[RestoreReviewSurfaceProjection],
) -> Vec<RestoreReviewDefect> {
    let mut defects = Vec::new();
    for review in &input.reviews {
        audit_review(review, &mut defects);
    }
    audit_vocabulary(input, projections, &mut defects);
    audit_compare_coverage(input, &mut defects);
    audit_review_coverage(input, &mut defects);
    defects
}

fn audit_review(review: &RestoreReviewEntry, defects: &mut Vec<RestoreReviewDefect>) {
    // Headline guardrail: a review may not hide narrowed capability or missing
    // replicated data behind green, full-normal status language.
    if review.identity_summary.overclaims_full_normal_status() {
        defects.push(RestoreReviewDefect::new(
            RestoreReviewNarrowReasonClass::FullNormalStatusOverclaimed,
            review.review_id.clone(),
            "a restore review may not present full normal status while the restore is narrower than normal or replicated data is incomplete",
        ));
    }

    // Hard guardrail: a managed/self-hosted/sovereign row may not blur an ordinary
    // workspace restore with a continuity restore.
    if review.conflates_restore_lane() {
        defects.push(RestoreReviewDefect::new(
            RestoreReviewNarrowReasonClass::RestoreLaneConflated,
            review.review_id.clone(),
            "a managed, self-hosted, or sovereign row may not present an ordinary workspace restore as continuity restore",
        ));
    }

    // Replay-fence guardrails apply to every review: a privileged or externally
    // mutating lane may not auto-replay, and a cleared fence must name its review.
    for fence in &review.replay_fences {
        if fence.auto_replays_unsafely() {
            defects.push(RestoreReviewDefect::new(
                RestoreReviewNarrowReasonClass::PrivilegedLaneAutoReplayed,
                review.review_id.clone(),
                "a privileged or externally mutating lane may not auto-replay after restore; an explicit reviewed step is required",
            ));
        } else if fence.missing_review_ref() {
            defects.push(RestoreReviewDefect::new(
                RestoreReviewNarrowReasonClass::ReplayFenceReviewMissing,
                review.review_id.clone(),
                "a fenced lane that was cleared must name the explicit reviewed step that cleared it",
            ));
        }
    }

    // Surface projection completeness.
    let missing = review
        .required_surfaces()
        .iter()
        .any(|surface| !review.projected_surfaces.contains(surface));
    if missing {
        defects.push(RestoreReviewDefect::new(
            RestoreReviewNarrowReasonClass::SurfaceReuseIncomplete,
            review.review_id.clone(),
            "every review must reach the service-health, support-center, managed-action-sheet, release-center, and public claim-manifest surfaces",
        ));
    }

    // The managed-continuity post-restore requirements only bind continuity
    // restores of managed or support artifact families on non-local profiles.
    if !review.requires_managed_review() {
        return;
    }

    // Restore fidelity must be disclosed.
    if review.identity_summary.restore_fidelity == RestoreFidelityClass::Undisclosed {
        defects.push(RestoreReviewDefect::new(
            RestoreReviewNarrowReasonClass::RestoreFidelityUndisclosed,
            review.review_id.clone(),
            "a managed-continuity review must disclose whether the restore was exact or narrower than normal",
        ));
    } else if review.identity_summary.missing_affected_slice() {
        // A narrower-than-normal restore must name the affected capability class
        // or data slice.
        defects.push(RestoreReviewDefect::new(
            RestoreReviewNarrowReasonClass::AffectedSliceUnnamed,
            review.review_id.clone(),
            "a narrower-than-normal restore must name the affected capability class or data slice",
        ));
    }

    // Restore identity must be declared for managed-continuity reviews.
    if review.identity_summary.restore_identity == RestoreIdentityClass::NotApplicable {
        defects.push(RestoreReviewDefect::new(
            RestoreReviewNarrowReasonClass::RestoreIdentityUndeclared,
            review.review_id.clone(),
            "a managed-continuity review must declare the restore identity recovery reproduces",
        ));
    }

    // Compare/export parity must be available so operators can inspect what
    // changed before assuming full continuity.
    if !review.compare_export.restored_vs_current_available {
        defects.push(RestoreReviewDefect::new(
            RestoreReviewNarrowReasonClass::CompareParityMissing,
            review.review_id.clone(),
            "a managed-continuity review must let operators compare restored-vs-current state",
        ));
    } else if !review.compare_export.export_available {
        defects.push(RestoreReviewDefect::new(
            RestoreReviewNarrowReasonClass::ExportParityMissing,
            review.review_id.clone(),
            "a managed-continuity review must let operators export the restored-vs-current comparison",
        ));
    }
}

fn audit_vocabulary(
    input: &RestoreReviewInput,
    projections: &[RestoreReviewSurfaceProjection],
    defects: &mut Vec<RestoreReviewDefect>,
) {
    for review in &input.reviews {
        let canonical_summary = review.identity_summary.summary_line();
        let canonical_fence = replay_fence_line(review);
        let canonical_compare = review.compare_export.parity_line();
        let drifted = projections
            .iter()
            .filter(|projection| projection.review_id == review.review_id)
            .any(|projection| {
                projection.restore_summary_line != canonical_summary
                    || projection.replay_fence_line != canonical_fence
                    || projection.compare_export_line != canonical_compare
            });
        if drifted {
            defects.push(RestoreReviewDefect::new(
                RestoreReviewNarrowReasonClass::ReviewVocabularyDrift,
                review.review_id.clone(),
                "a surface renders different restore-identity, replay-fence, or compare/export vocabulary than the descriptor",
            ));
        }
    }
}

fn audit_compare_coverage(input: &RestoreReviewInput, defects: &mut Vec<RestoreReviewDefect>) {
    let managed_covered = input.reviews.iter().any(|review| {
        review.requires_managed_review()
            && review.artifact_family.is_managed_artifact_family()
            && review.compare_export.is_full_parity()
    });
    if !managed_covered {
        defects.push(RestoreReviewDefect::new(
            RestoreReviewNarrowReasonClass::ManagedFamilyCompareCoverageMissing,
            "continuity:restore-review-registry",
            "at least one managed artifact family must offer restored-vs-current compare and export",
        ));
    }

    let support_covered = input.reviews.iter().any(|review| {
        review.requires_managed_review()
            && review.artifact_family.is_support_export_family()
            && review.compare_export.is_full_parity()
    });
    if !support_covered {
        defects.push(RestoreReviewDefect::new(
            RestoreReviewNarrowReasonClass::SupportFamilyCompareCoverageMissing,
            "continuity:restore-review-registry",
            "at least one support/export artifact family must offer restored-vs-current compare and export",
        ));
    }
}

fn audit_review_coverage(input: &RestoreReviewInput, defects: &mut Vec<RestoreReviewDefect>) {
    for claim_row_id in &input.expected_claim_row_ids {
        let has_review = input
            .reviews
            .iter()
            .any(|review| &review.claim_row_id == claim_row_id);
        if !has_review {
            defects.push(RestoreReviewDefect::new(
                RestoreReviewNarrowReasonClass::ReviewEvidenceMissing,
                claim_row_id.clone(),
                "a claimed restored row carries no restore review; the claim narrows",
            ));
        }
    }
}

fn build_surface_projections(
    reviews: &[RestoreReviewEntry],
) -> Vec<RestoreReviewSurfaceProjection> {
    let mut projections = Vec::new();
    for review in reviews {
        let restore_summary_line = review.identity_summary.summary_line();
        let replay_fence_line = replay_fence_line(review);
        let compare_export_line = review.compare_export.parity_line();
        let descriptor_id = format!("continuity:restore-review-descriptor:{}", review.review_id);
        for surface in ReviewSurfaceClass::ALL {
            if !review.projected_surfaces.contains(&surface) {
                continue;
            }
            projections.push(RestoreReviewSurfaceProjection {
                record_kind: RESTORE_REVIEW_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
                schema_version: RESTORE_REVIEW_SCHEMA_VERSION,
                shared_contract_ref: RESTORE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
                surface,
                surface_token: surface.as_str().to_owned(),
                review_id: review.review_id.clone(),
                descriptor_id: descriptor_id.clone(),
                restore_summary_line: restore_summary_line.clone(),
                replay_fence_line: replay_fence_line.clone(),
                compare_export_line: compare_export_line.clone(),
            });
        }
    }
    projections
}

fn build_outcomes(
    input: &RestoreReviewInput,
    defects: &[RestoreReviewDefect],
) -> Vec<RestoreReviewOutcome> {
    input
        .reviews
        .iter()
        .map(|review| {
            let reasons: Vec<RestoreReviewNarrowReasonClass> = defects
                .iter()
                .filter(|defect| defect.source == review.review_id)
                .map(|defect| defect.narrow_reason)
                .collect();
            let qualification = qualification_from_reasons(reasons.iter());
            let mut reason_tokens: Vec<String> = reasons
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect();
            reason_tokens.sort();
            reason_tokens.dedup();
            RestoreReviewOutcome {
                record_kind: RESTORE_REVIEW_OUTCOME_RECORD_KIND.to_owned(),
                schema_version: RESTORE_REVIEW_SCHEMA_VERSION,
                shared_contract_ref: RESTORE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
                review_id: review.review_id.clone(),
                claim_row_id: review.claim_row_id.clone(),
                artifact_family_token: review.artifact_family_token.clone(),
                qualification_token: qualification.as_str().to_owned(),
                narrowed: qualification != ContinuityClaimQualificationClass::Stable,
                claim_withheld: qualification == ContinuityClaimQualificationClass::Withdrawn,
                restore_fidelity_token: review.identity_summary.restore_fidelity_token.clone(),
                restore_identity_token: review.identity_summary.restore_identity_token.clone(),
                privileged_lanes_fenced: !review.has_unsafe_auto_replay(),
                compare_export_available: review.compare_export.is_full_parity(),
                narrow_reason_tokens: reason_tokens,
            }
        })
        .collect()
}

fn build_registry(
    input: &RestoreReviewInput,
    outcomes: &[RestoreReviewOutcome],
) -> RestoreReviewRegistry {
    // The tracked claim rows are the declared restored rows plus every row a
    // review actually backs, in stable sorted order.
    let mut claim_row_ids: Vec<String> = input.expected_claim_row_ids.clone();
    for review in &input.reviews {
        claim_row_ids.push(review.claim_row_id.clone());
    }
    claim_row_ids.sort();
    claim_row_ids.dedup();

    let mut coverage = Vec::new();
    let mut covered = Vec::new();
    let mut uncovered = Vec::new();
    for claim_row_id in claim_row_ids {
        let outcome = outcomes
            .iter()
            .find(|outcome| outcome.claim_row_id == claim_row_id);
        let (coverage_class, qualification_token, review_id) = match outcome {
            None => (
                ReviewCoverageClass::NoReview,
                ContinuityClaimQualificationClass::Preview
                    .as_str()
                    .to_owned(),
                String::new(),
            ),
            Some(outcome) if outcome.claim_withheld => (
                ReviewCoverageClass::ReviewWithheld,
                outcome.qualification_token.clone(),
                outcome.review_id.clone(),
            ),
            Some(outcome) if outcome.narrowed => (
                ReviewCoverageClass::NarrowedReviewNeedsAttention,
                outcome.qualification_token.clone(),
                outcome.review_id.clone(),
            ),
            Some(outcome) => (
                ReviewCoverageClass::CurrentReview,
                outcome.qualification_token.clone(),
                outcome.review_id.clone(),
            ),
        };
        let covered_now = coverage_class.is_covered();
        if covered_now {
            covered.push(claim_row_id.clone());
        } else {
            uncovered.push(claim_row_id.clone());
        }
        coverage.push(RestoreReviewCoverageRow {
            record_kind: RESTORE_REVIEW_COVERAGE_ROW_RECORD_KIND.to_owned(),
            schema_version: RESTORE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: RESTORE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            claim_row_id,
            coverage_class,
            coverage_class_token: coverage_class.as_str().to_owned(),
            review_id,
            qualification_token,
            covered: covered_now,
            narrowed: !covered_now,
        });
    }

    let managed_family_compare_covered = input.reviews.iter().any(|review| {
        review.requires_managed_review()
            && review.artifact_family.is_managed_artifact_family()
            && review.compare_export.is_full_parity()
    });
    let support_family_compare_covered = input.reviews.iter().any(|review| {
        review.requires_managed_review()
            && review.artifact_family.is_support_export_family()
            && review.compare_export.is_full_parity()
    });

    RestoreReviewRegistry {
        record_kind: RESTORE_REVIEW_REGISTRY_RECORD_KIND.to_owned(),
        schema_version: RESTORE_REVIEW_SCHEMA_VERSION,
        shared_contract_ref: RESTORE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
        registry_id: "continuity:restore-review-registry".to_owned(),
        coverage,
        covered_claim_row_ids: covered,
        uncovered_claim_row_ids: uncovered,
        managed_family_compare_covered,
        support_family_compare_covered,
    }
}

fn build_summary(
    input: &RestoreReviewInput,
    projections: &[RestoreReviewSurfaceProjection],
    outcomes: &[RestoreReviewOutcome],
    registry: &RestoreReviewRegistry,
    defects: &[RestoreReviewDefect],
) -> RestoreReviewSummary {
    let overall = if defects
        .iter()
        .any(|defect| defect.narrow_reason.is_withdrawal_reason())
    {
        ContinuityClaimQualificationClass::Withdrawn
    } else if defects
        .iter()
        .any(|defect| defect.narrow_reason.is_preview_reason())
    {
        ContinuityClaimQualificationClass::Preview
    } else if defects.is_empty() {
        ContinuityClaimQualificationClass::Stable
    } else {
        ContinuityClaimQualificationClass::Beta
    };

    let vocabulary_consistent = !defects.iter().any(|defect| {
        defect.narrow_reason == RestoreReviewNarrowReasonClass::ReviewVocabularyDrift
    });

    let mut families: Vec<RestoreArtifactFamilyClass> = input
        .reviews
        .iter()
        .map(|review| review.artifact_family)
        .collect();
    families.sort();
    families.dedup();

    let managed_reviews: Vec<&RestoreReviewEntry> = input
        .reviews
        .iter()
        .filter(|review| review.requires_managed_review())
        .collect();

    let replay_fence_count: usize = input
        .reviews
        .iter()
        .map(|review| review.replay_fences.len())
        .sum();
    let fenced_privileged_lane_count: usize = input
        .reviews
        .iter()
        .flat_map(|review| review.replay_fences.iter())
        .filter(|fence| fence.replay_posture.requires_fence() && fence.auto_replay_blocked())
        .count();

    RestoreReviewSummary {
        record_kind: RESTORE_REVIEW_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: RESTORE_REVIEW_SCHEMA_VERSION,
        shared_contract_ref: RESTORE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
        overall_qualification_token: overall.as_str().to_owned(),
        review_count: input.reviews.len(),
        family_count: families.len(),
        managed_review_count: managed_reviews.len(),
        exact_restore_count: fidelity_count(input, RestoreFidelityClass::ExactRestore),
        narrower_than_normal_count: fidelity_count(
            input,
            RestoreFidelityClass::NarrowerThanNormalRestore,
        ),
        replay_fence_count,
        fenced_privileged_lane_count,
        compare_export_available_count: input
            .reviews
            .iter()
            .filter(|review| review.compare_export.is_full_parity())
            .count(),
        narrowed_count: outcomes.iter().filter(|outcome| outcome.narrowed).count(),
        withdrawn_count: outcomes
            .iter()
            .filter(|outcome| outcome.claim_withheld)
            .count(),
        claim_coverage_count: registry.coverage.len(),
        covered_claim_count: registry.covered_claim_row_ids.len(),
        uncovered_claim_count: registry.uncovered_claim_row_ids.len(),
        surface_projection_count: projections.len(),
        vocabulary_consistent,
        no_unsafe_auto_replay: !input
            .reviews
            .iter()
            .any(|review| review.has_unsafe_auto_replay()),
        no_full_normal_status_overclaim: !input
            .reviews
            .iter()
            .any(|review| review.identity_summary.overclaims_full_normal_status()),
        no_restore_lane_conflation: !input
            .reviews
            .iter()
            .any(|review| review.conflates_restore_lane()),
        all_narrower_restores_name_affected_slice: !input
            .reviews
            .iter()
            .any(|review| review.identity_summary.missing_affected_slice()),
        managed_family_compare_covered: registry.managed_family_compare_covered,
        support_family_compare_covered: registry.support_family_compare_covered,
        all_expected_claims_covered: input
            .expected_claim_row_ids
            .iter()
            .all(|claim_row_id| registry.is_claim_row_covered(claim_row_id)),
        restore_truth_export_safe: true,
        raw_payloads_excluded: true,
        defect_count: defects.len(),
    }
}

fn fidelity_count(input: &RestoreReviewInput, fidelity: RestoreFidelityClass) -> usize {
    input
        .reviews
        .iter()
        .filter(|review| review.identity_summary.restore_fidelity == fidelity)
        .count()
}

fn replay_fence_line(entry: &RestoreReviewEntry) -> String {
    let mut held = 0;
    let mut cleared = 0;
    let mut local_safe = 0;
    for fence in &entry.replay_fences {
        match fence.fence_state {
            ReplayFenceStateClass::HeldForReview => held += 1,
            ReplayFenceStateClass::ClearedAfterReview => cleared += 1,
            ReplayFenceStateClass::NoFenceLocalSafe => local_safe += 1,
        }
    }
    let guard = if entry.has_unsafe_auto_replay() {
        "WARNING: a privileged or externally mutating lane is set to auto-replay without review"
    } else {
        "privileged and externally mutating lanes do not auto-replay"
    };
    format!(
        "Replay fences: {held} held for review, {cleared} cleared after review, {local_safe} local-safe; {guard}."
    )
}

fn profile_plain(class: ContinuityProfileClass) -> &'static str {
    match class {
        ContinuityProfileClass::Managed => "managed cloud",
        ContinuityProfileClass::SelfHosted => "self-hosted",
        ContinuityProfileClass::Sovereign => "sovereign",
        ContinuityProfileClass::LocalOnly => "local-only",
    }
}

fn restore_identity_plain(class: RestoreIdentityClass) -> &'static str {
    match class {
        RestoreIdentityClass::SameIdentityRestore => {
            "recovery reproduces the same durable identity"
        }
        RestoreIdentityClass::ReissuedIdentityRestore => {
            "recovery reissues a derived identity that must be re-trusted"
        }
        RestoreIdentityClass::NewInstallRebind => "recovery requires a new install rebind",
        RestoreIdentityClass::NotApplicable => "not applicable to this review",
    }
}
