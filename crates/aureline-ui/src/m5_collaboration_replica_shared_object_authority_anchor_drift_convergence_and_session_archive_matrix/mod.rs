//! Frozen M5 collaboration-replica, shared-object-authority, anchor-drift, convergence-state, and
//! session-archive matrix.
//!
//! This module locks Aureline's collaboration *state* model — the underlying shared-object authority,
//! convergence, drift, downgrade, and export contract that sits beneath the already-frozen collaboration-control
//! surfaces. It names, once, which shared objects Aureline exposes to a claimed M5 session surface — the
//! CRDT-backed shared text, the sampled presence / cursors / selections, the server-ordered comments /
//! annotations / review pins, the presenter / follow state, the separate higher-risk control plane, and the
//! sealed session archive — and binds each to its authority model, its merge / drift semantics, its downgrade
//! behavior, its export posture, and the surfaces allowed to claim support for it, so a session surface reads one
//! governed object-class row instead of inferring behavior from a generic session-state pill. Every covered
//! object class is constrained by the same shared collaboration-state role taxonomy (authority_model_disclosure,
//! local_truth_preservation_disclosure, merge_and_drift_semantics_disclosure, downgrade_behavior_disclosure,
//! anchor_drift_history_disclosure, export_posture_disclosure, provenance_and_freshness_disclosure), the same
//! required visible state (surface label, authority model, convergence state, local-truth disposition, merge /
//! drift summary, export posture, and provenance / freshness), the same
//! no-replica-overwrites-local-buffer-VFS-or-Git-truth rule, the same
//! no-discarding-unsent-local-edits-on-downgrade rule, the same
//! no-rebinding-comments-annotations-or-review-pins-without-drift-history rule, the same
//! no-collapsing-convergence-or-awareness-degraded-state-into-a-generic-stale-badge rule, and the same
//! no-exporting-op-logs-snapshots-or-archives-without-policy-labeled-redaction-and-lineage rule regardless of the
//! surface that renders it.
//!
//! The matrix makes a converged shared object mechanically distinct from one that is still converging,
//! server-ordered, host-authoritative, locally pending, convergence-degraded, awareness-degraded,
//! anchor-unresolved, anchor-rebound, relay-partitioned, reconciliation-required, compaction-pending, sealed /
//! archived, local-canonical-preserved, sampled-presence-only, or provenance-stale (see
//! [`M5ConvergenceState`]) so search, AI, review, docs, and support consumers can key off the convergence state,
//! authority model, and export posture rather than guessing from a generic stale or broken pill. It does not
//! redesign the relay service, the full shared-control plane, or a broad archive / compliance program — it reuses
//! the already-landed companion session-follow / incident-awareness surfaces, presence-avatar stacks and role /
//! follow badges, lifecycle / status vocabulary, collaboration-control grants and consent envelopes, no-hidden-
//! rerun restore work, and export / redaction infrastructure — it is the shared reusable collaboration-state
//! contract those consumers read, and it binds back to the already-frozen collaboration-control component matrix,
//! the stable-proof-index, and the migration-task-row packets so collaboration-state truth is not split across
//! surfaces. The controlled vocabularies are frozen in one self-describing
//! [`M5CollaborationStateVocabularySet`] rather than minted per surface. Raw buffer bodies, raw op-log payloads,
//! raw snapshots, private endpoints, and unredacted archive contents stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_collaboration_state_matrix,
    seeded_m5_collaboration_state_matrix_higher_risk_control_plane_beta_narrowed,
    seeded_m5_collaboration_state_matrix_sealed_session_archive_preview_narrowed,
    M5_COLLABORATION_STATE_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5CollaborationStateMatrixPacket`].
pub const M5_COLLABORATION_STATE_MATRIX_RECORD_KIND: &str =
    "freeze_m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix";

/// Schema version for M5 collaboration-state matrix records.
pub const M5_COLLABORATION_STATE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined collaboration-state authority matrix schema.
pub const M5_COLLABORATION_STATE_MATRIX_SCHEMA_REF: &str =
    "schemas/collaboration/m5-collaboration-state-authority-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_COLLABORATION_STATE_MATRIX_DOC_REF: &str =
    "docs/collaboration/m5-collaboration-convergence-ops.md";

/// Repo-relative path of the canonical CRDT-backed shared-text replica-state domain schema (the CRDT replica of
/// shared text, its convergence, and the local buffer it never replaces).
pub const M5_CRDT_BACKED_SHARED_TEXT_DOMAIN_SCHEMA_REF: &str =
    "schemas/collaboration/m5-collaboration-replica-state.schema.json";

/// Repo-relative path of the canonical sampled-presence descriptor domain schema (the sampled, non-authoritative
/// presence / cursors / selections shared-object descriptor).
pub const M5_SAMPLED_PRESENCE_CURSORS_SELECTIONS_DOMAIN_SCHEMA_REF: &str =
    "schemas/collaboration/m5-shared-object-descriptor.schema.json";

/// Repo-relative path of the canonical server-ordered comments / annotations / review-pin anchor-history domain
/// schema (server-ordered pins with their append-only anchor-drift history).
pub const M5_SERVER_ORDERED_COMMENTS_ANNOTATIONS_REVIEW_PINS_DOMAIN_SCHEMA_REF: &str =
    "schemas/collaboration/m5-collaboration-anchor-history.schema.json";

/// Repo-relative path of the canonical presenter / follow convergence-state domain schema (presenter / follow
/// state and its convergence posture).
pub const M5_PRESENTER_FOLLOW_STATE_DOMAIN_SCHEMA_REF: &str =
    "schemas/collaboration/m5-collaboration-convergence-state.schema.json";

/// Repo-relative path of the canonical higher-risk control-plane degradation-banner domain schema (the separate
/// higher-risk control plane and the convergence / awareness degradation banner it drives).
pub const M5_HIGHER_RISK_CONTROL_PLANE_DOMAIN_SCHEMA_REF: &str =
    "schemas/ui/m5-collaboration-degradation-banner.schema.json";

/// Repo-relative path of the canonical sealed session-archive compaction-manifest domain schema (the sealed
/// session archive, its bounded compaction lineage, and its policy-labeled redaction).
pub const M5_SEALED_SESSION_ARCHIVE_DOMAIN_SCHEMA_REF: &str =
    "schemas/collaboration/m5-session-compaction-manifest.schema.json";

/// Repo-relative path of the already-frozen collaboration-control component matrix the matrix binds back to so
/// collaboration-state and collaboration-control truth share one contract.
pub const M5_COLLABORATION_CONTROL_MATRIX_LANDED_SCHEMA_REF: &str =
    "schemas/collaboration/m5-collaboration-control-component-matrix.schema.json";

/// Repo-relative path of the already-landed stable-proof-index schema the matrix binds back to.
pub const M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF: &str =
    "schemas/release/stable_proof_index.schema.json";

/// Repo-relative path of the already-landed migration-task-row schema the matrix binds back to.
pub const M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF: &str =
    "schemas/release/m5-migration-task-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_COLLABORATION_STATE_FIXTURE_DIR: &str = "fixtures/collaboration/m5-convergence";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COLLABORATION_STATE_ARTIFACT_REF: &str =
    "artifacts/release/m5-collaboration-convergence-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_COLLABORATION_STATE_CSV_REF: &str =
    "artifacts/release/m5-collaboration-convergence-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_COLLABORATION_STATE_REPORT_REF: &str =
    "artifacts/design/m5-collaboration-state-authority-matrix.md";

/// Repo-relative path of the checked collaboration-convergence-health dashboard.
pub const M5_COLLABORATION_STATE_DASHBOARD_REF: &str =
    "dashboards/m5-collaboration-convergence-health.json";

/// One of the six governed collaboration-state shared-object classes this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationStateObject {
    /// CRDT-backed shared text: the convergent replica of shared text that merges concurrent edits without ever replacing the canonical local buffer, VFS, or Git truth.
    CrdtBackedSharedText,
    /// Sampled presence / cursors / selections: the sampled, non-authoritative presence, cursor, and selection stream that expires when stale and never edits the buffer.
    SampledPresenceCursorsSelections,
    /// Server-ordered comments / annotations / review pins: the server-ordered comment, annotation, and review-pin objects whose anchors drift append-only and reviewably, never rebinding silently.
    ServerOrderedCommentsAnnotationsReviewPins,
    /// Presenter / follow state: the presenter and follow state whose follow is view-only and whose handoff is provenance-tracked, never implying convergence or control.
    PresenterFollowState,
    /// Higher-risk control plane: the separate higher-risk control plane that keeps convergence-degraded distinct from awareness-degraded and preserves local unsent work first on downgrade.
    HigherRiskControlPlane,
    /// Sealed session archive: the sealed session archive with bounded compaction lineage, actor lineage, and policy-labeled redaction, never exported without both.
    SealedSessionArchive,
}

impl M5CollaborationStateObject {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CrdtBackedSharedText,
        Self::SampledPresenceCursorsSelections,
        Self::ServerOrderedCommentsAnnotationsReviewPins,
        Self::PresenterFollowState,
        Self::HigherRiskControlPlane,
        Self::SealedSessionArchive,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrdtBackedSharedText => "crdt_backed_shared_text",
            Self::SampledPresenceCursorsSelections => "sampled_presence_cursors_selections",
            Self::ServerOrderedCommentsAnnotationsReviewPins => {
                "server_ordered_comments_annotations_review_pins"
            }
            Self::PresenterFollowState => "presenter_follow_state",
            Self::HigherRiskControlPlane => "higher_risk_control_plane",
            Self::SealedSessionArchive => "sealed_session_archive",
        }
    }
    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// class's replica, presence, anchor-history, convergence, degradation, or archive meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::CrdtBackedSharedText => M5_CRDT_BACKED_SHARED_TEXT_DOMAIN_SCHEMA_REF,
            Self::SampledPresenceCursorsSelections => {
                M5_SAMPLED_PRESENCE_CURSORS_SELECTIONS_DOMAIN_SCHEMA_REF
            }
            Self::ServerOrderedCommentsAnnotationsReviewPins => {
                M5_SERVER_ORDERED_COMMENTS_ANNOTATIONS_REVIEW_PINS_DOMAIN_SCHEMA_REF
            }
            Self::PresenterFollowState => M5_PRESENTER_FOLLOW_STATE_DOMAIN_SCHEMA_REF,
            Self::HigherRiskControlPlane => M5_HIGHER_RISK_CONTROL_PLANE_DOMAIN_SCHEMA_REF,
            Self::SealedSessionArchive => M5_SEALED_SESSION_ARCHIVE_DOMAIN_SCHEMA_REF,
        }
    }

    /// `true` when this class must name a controlled CRDT-backed shared-text role.
    pub const fn declares_crdt_backed_shared_text_roles(self) -> bool {
        matches!(self, Self::CrdtBackedSharedText)
    }

    /// `true` when this class must name a controlled sampled-presence role.
    pub const fn declares_sampled_presence_cursors_selections_roles(self) -> bool {
        matches!(self, Self::SampledPresenceCursorsSelections)
    }

    /// `true` when this class must name a controlled server-ordered comment / pin role.
    pub const fn declares_server_ordered_comments_annotations_review_pins_roles(self) -> bool {
        matches!(self, Self::ServerOrderedCommentsAnnotationsReviewPins)
    }

    /// `true` when this class must name a controlled presenter / follow role.
    pub const fn declares_presenter_follow_state_roles(self) -> bool {
        matches!(self, Self::PresenterFollowState)
    }

    /// `true` when this class must name a controlled higher-risk control-plane role.
    pub const fn declares_higher_risk_control_plane_roles(self) -> bool {
        matches!(self, Self::HigherRiskControlPlane)
    }

    /// `true` when this class must name a controlled sealed session-archive role.
    pub const fn declares_sealed_session_archive_roles(self) -> bool {
        matches!(self, Self::SealedSessionArchive)
    }
}

/// The single controlled collaboration-state role vocabulary every replica, presence, pin, presenter / follow, degradation-banner, archive, help / docs, or support / export consumer binds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationStateRole {
    /// The authority model — whether the object converges, stays server-ordered, is host-authoritative, or defers to local canonical truth — disclosed on every claimed surface.
    AuthorityModelDisclosure,
    /// The local-truth preservation posture disclosed so a replica never replaces the canonical local buffer, VFS, or Git truth.
    LocalTruthPreservationDisclosure,
    /// The merge and drift semantics disclosed so how concurrent edits merge and how anchors drift is never implicit.
    MergeAndDriftSemanticsDisclosure,
    /// The downgrade behavior disclosed so a permission or relay downgrade preserves local unsent work first.
    DowngradeBehaviorDisclosure,
    /// The anchor-drift history disclosed as an append-only, reviewable record, never a silent rebind.
    AnchorDriftHistoryDisclosure,
    /// The export posture disclosed so op-logs, snapshots, and archives carry policy-labeled redaction and actor lineage.
    ExportPostureDisclosure,
    /// The session provenance and freshness disclosed so search, AI, review, docs, and support never consume stale collaboration state as current.
    ProvenanceAndFreshnessDisclosure,
}

impl M5CollaborationStateRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::AuthorityModelDisclosure,
        Self::LocalTruthPreservationDisclosure,
        Self::MergeAndDriftSemanticsDisclosure,
        Self::DowngradeBehaviorDisclosure,
        Self::AnchorDriftHistoryDisclosure,
        Self::ExportPostureDisclosure,
        Self::ProvenanceAndFreshnessDisclosure,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthorityModelDisclosure => "authority_model_disclosure",
            Self::LocalTruthPreservationDisclosure => "local_truth_preservation_disclosure",
            Self::MergeAndDriftSemanticsDisclosure => "merge_and_drift_semantics_disclosure",
            Self::DowngradeBehaviorDisclosure => "downgrade_behavior_disclosure",
            Self::AnchorDriftHistoryDisclosure => "anchor_drift_history_disclosure",
            Self::ExportPostureDisclosure => "export_posture_disclosure",
            Self::ProvenanceAndFreshnessDisclosure => "provenance_and_freshness_disclosure",
        }
    }
    /// Whether this role is a hard posture requirement that must be present before a class may be
    /// surfaced as a collaboration-state result (`authority_model_disclosure`,
    /// `local_truth_preservation_disclosure`, `merge_and_drift_semantics_disclosure`,
    /// `downgrade_behavior_disclosure`). The contextual roles (`anchor_drift_history_disclosure`,
    /// `export_posture_disclosure`, `provenance_and_freshness_disclosure`) apply where the object class calls
    /// for them.
    pub const fn must_be_present_before_surfacing_as_a_collaboration_state_result(self) -> bool {
        matches!(
            self,
            Self::AuthorityModelDisclosure
                | Self::LocalTruthPreservationDisclosure
                | Self::MergeAndDriftSemanticsDisclosure
                | Self::DowngradeBehaviorDisclosure
        )
    }
}

/// Convergence / authority state that makes a converged shared object mechanically distinct from one that is still converging, server-ordered, host-authoritative, locally pending, convergence-degraded, awareness-degraded, anchor-unresolved, anchor-rebound, relay-partitioned, reconciliation-required, compaction-pending, sealed / archived, local-canonical-preserved, sampled-presence-only, or provenance-stale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConvergenceState {
    /// Converged: every replica of the object has converged on one agreed value.
    Converged,
    /// Converging (pending ops): the object is converging and has unmerged operations in flight.
    ConvergingPendingOps,
    /// Server-ordered: the object's order is fixed by a server total order, not peer convergence.
    ServerOrdered,
    /// Host-authoritative: the object's value is owned by an authoritative host, not merged.
    HostAuthoritative,
    /// Locally pending (unsent): local unsent work exists that has not yet reached the shared object.
    LocallyPendingUnsent,
    /// Convergence-degraded: replicas cannot currently converge and the object is not agreed.
    ConvergenceDegraded,
    /// Awareness-degraded: presence / awareness is degraded while the object itself may still converge.
    AwarenessDegraded,
    /// Anchor-unresolved: a comment, annotation, or review pin has no resolvable anchor yet.
    AnchorUnresolved,
    /// Anchor-rebound (append-only): an anchor rebound to a new position, recorded append-only in drift history.
    AnchorReboundAppendOnly,
    /// Relay-partitioned: the relay is partitioned and awareness is degraded without overwriting local truth.
    RelayPartitioned,
    /// Reconciliation-required: deferred intent must be reconciled before the object is trustworthy again.
    ReconciliationRequired,
    /// Compaction-pending: the session is pending bounded compaction into a lineage-preserving manifest.
    CompactionPending,
    /// Sealed / archived: the session object is sealed into a retained, policy-labeled archive.
    SealedArchived,
    /// Local-canonical preserved: the canonical local buffer / VFS / Git truth is preserved and never replaced.
    LocalCanonicalPreserved,
    /// Sampled-presence only: only sampled, non-authoritative presence is available for the object.
    SampledPresenceOnly,
    /// Provenance-stale: the object's provenance or freshness is stale and must not be consumed as current.
    ProvenanceStale,
}

impl M5ConvergenceState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 16] = [
        Self::Converged,
        Self::ConvergingPendingOps,
        Self::ServerOrdered,
        Self::HostAuthoritative,
        Self::LocallyPendingUnsent,
        Self::ConvergenceDegraded,
        Self::AwarenessDegraded,
        Self::AnchorUnresolved,
        Self::AnchorReboundAppendOnly,
        Self::RelayPartitioned,
        Self::ReconciliationRequired,
        Self::CompactionPending,
        Self::SealedArchived,
        Self::LocalCanonicalPreserved,
        Self::SampledPresenceOnly,
        Self::ProvenanceStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Converged => "converged",
            Self::ConvergingPendingOps => "converging_pending_ops",
            Self::ServerOrdered => "server_ordered",
            Self::HostAuthoritative => "host_authoritative",
            Self::LocallyPendingUnsent => "locally_pending_unsent",
            Self::ConvergenceDegraded => "convergence_degraded",
            Self::AwarenessDegraded => "awareness_degraded",
            Self::AnchorUnresolved => "anchor_unresolved",
            Self::AnchorReboundAppendOnly => "anchor_rebound_append_only",
            Self::RelayPartitioned => "relay_partitioned",
            Self::ReconciliationRequired => "reconciliation_required",
            Self::CompactionPending => "compaction_pending",
            Self::SealedArchived => "sealed_archived",
            Self::LocalCanonicalPreserved => "local_canonical_preserved",
            Self::SampledPresenceOnly => "sampled_presence_only",
            Self::ProvenanceStale => "provenance_stale",
        }
    }
    /// `true` only for the converged state, so downstream shared-editor replica views, presence layers, the
    /// degradation banner, and support / export packets can key off a genuinely converged object rather than
    /// confusing it with a converging, server-ordered, host-authoritative, locally-pending, or any
    /// degraded / anchor / archive state.
    pub const fn is_converged(self) -> bool {
        matches!(self, Self::Converged)
    }
}

/// Named shared-object authority model (a CRDT-convergent replica, a server-ordered sequence, a host-authoritative state, or local canonical truth that a replica never replaces) so the four authority kinds are never flattened into one generic shared-state badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationAuthorityModel {
    /// A CRDT-convergent replica: peers converge on one value without a central ordering authority.
    CrdtConvergentReplica,
    /// A server-ordered sequence: order is fixed by a server total order, not by peer convergence.
    ServerOrderedSequence,
    /// A host-authoritative state: an authoritative host owns the value; peers observe, not merge.
    HostAuthoritativeState,
    /// Local canonical truth never replaced: the local buffer / VFS / Git truth is canonical and a replica never overwrites it.
    LocalCanonicalNeverReplaced,
}

impl M5CollaborationAuthorityModel {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CrdtConvergentReplica,
        Self::ServerOrderedSequence,
        Self::HostAuthoritativeState,
        Self::LocalCanonicalNeverReplaced,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrdtConvergentReplica => "crdt_convergent_replica",
            Self::ServerOrderedSequence => "server_ordered_sequence",
            Self::HostAuthoritativeState => "host_authoritative_state",
            Self::LocalCanonicalNeverReplaced => "local_canonical_never_replaced",
        }
    }
    /// `true` only for a CRDT-convergent replica, so a consumer can mechanically refuse to treat a
    /// server-ordered, host-authoritative, or local-canonical object as a merge-convergent replica.
    pub const fn is_convergent_replica(self) -> bool {
        matches!(self, Self::CrdtConvergentReplica)
    }
}

/// Named downgrade gate (converged with local work preserved, blocked because unsent local work is at risk, blocked by permission downgrade, blocked by relay partition, blocked by unreviewed anchor drift) so no claimed surface lacks a named state for a downgrade that must preserve local truth first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationDowngradeGate {
    /// Converged, local work preserved: the object is converged and no unsent local work is at risk.
    ConvergedLocalWorkPreserved,
    /// Blocked by unsent local work at risk: local unsent edits must be preserved before the downgrade proceeds.
    BlockedByUnsentLocalWorkAtRisk,
    /// Blocked by permission downgrade: a permission downgrade must preserve local work before write access narrows.
    BlockedByPermissionDowngrade,
    /// Blocked by relay partition: a relay partition degrades awareness and must not overwrite local truth.
    BlockedByRelayPartition,
    /// Blocked by unreviewed anchor drift: anchor drift must be recorded and reviewable before rebinding proceeds.
    BlockedByUnreviewedAnchorDrift,
}

impl M5CollaborationDowngradeGate {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ConvergedLocalWorkPreserved,
        Self::BlockedByUnsentLocalWorkAtRisk,
        Self::BlockedByPermissionDowngrade,
        Self::BlockedByRelayPartition,
        Self::BlockedByUnreviewedAnchorDrift,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConvergedLocalWorkPreserved => "converged_local_work_preserved",
            Self::BlockedByUnsentLocalWorkAtRisk => "blocked_by_unsent_local_work_at_risk",
            Self::BlockedByPermissionDowngrade => "blocked_by_permission_downgrade",
            Self::BlockedByRelayPartition => "blocked_by_relay_partition",
            Self::BlockedByUnreviewedAnchorDrift => "blocked_by_unreviewed_anchor_drift",
        }
    }
    /// `true` for the blocked states (`blocked_by_unsent_local_work_at_risk`,
    /// `blocked_by_permission_downgrade`, `blocked_by_relay_partition`,
    /// `blocked_by_unreviewed_anchor_drift`) so a consumer can mechanically refuse to complete a downgrade or
    /// rebind while local unsent work, permission, relay, or anchor-drift review is still pending.
    pub const fn is_blocked_pending_local_preservation(self) -> bool {
        matches!(
            self,
            Self::BlockedByUnsentLocalWorkAtRisk
                | Self::BlockedByPermissionDowngrade
                | Self::BlockedByRelayPartition
                | Self::BlockedByUnreviewedAnchorDrift
        )
    }
}

/// Controlled CRDT-backed shared-text role for one convergent shared-text replica.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CrdtBackedSharedTextRole {
    /// The CRDT convergence shown so participants see whether the shared text has converged.
    CrdtConvergenceShown,
    /// The local buffer shown as canonical so the replica never replaces local, VFS, or Git truth.
    LocalBufferRemainsCanonicalShown,
    /// The merge semantics shown so how concurrent edits merge is never implicit.
    MergeSemanticsShown,
    /// The unsent local edits preserved on downgrade shown so a downgrade never discards local work.
    UnsentLocalEditsPreservedOnDowngradeShown,
    /// A role bound to the single collaboration-state registry.
    BoundToCollaborationStateRegistry,
    /// A replica overwriting the local buffer, which is disallowed.
    ReplicaOverwritesLocalBufferDisallowed,
}

impl M5CrdtBackedSharedTextRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CrdtConvergenceShown,
        Self::LocalBufferRemainsCanonicalShown,
        Self::MergeSemanticsShown,
        Self::UnsentLocalEditsPreservedOnDowngradeShown,
        Self::BoundToCollaborationStateRegistry,
        Self::ReplicaOverwritesLocalBufferDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrdtConvergenceShown => "crdt_convergence_shown",
            Self::LocalBufferRemainsCanonicalShown => "local_buffer_remains_canonical_shown",
            Self::MergeSemanticsShown => "merge_semantics_shown",
            Self::UnsentLocalEditsPreservedOnDowngradeShown => {
                "unsent_local_edits_preserved_on_downgrade_shown"
            }
            Self::BoundToCollaborationStateRegistry => "bound_to_collaboration_state_registry",
            Self::ReplicaOverwritesLocalBufferDisallowed => {
                "replica_overwrites_local_buffer_disallowed"
            }
        }
    }
}

/// Controlled sampled-presence role for the sampled presence / cursors / selections stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SampledPresenceCursorsSelectionsRole {
    /// The sampled presence rate shown so presence is understood as sampled, not continuous truth.
    SampledPresenceRateShown,
    /// The presence-is-non-authoritative posture shown so presence never reads as convergence.
    PresenceIsNonAuthoritativeShown,
    /// The stale-presence expiry shown so stale cursors / selections expire rather than mislead.
    StalePresenceExpiryShown,
    /// The presence-never-edits-buffer posture shown so a cursor / selection never mutates the buffer.
    PresenceNeverEditsBufferShown,
    /// A role bound to the single collaboration-state registry.
    BoundToCollaborationStateRegistry,
    /// Treating sampled presence as converged truth, which is disallowed.
    PresenceTreatedAsConvergedTruthDisallowed,
}

impl M5SampledPresenceCursorsSelectionsRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SampledPresenceRateShown,
        Self::PresenceIsNonAuthoritativeShown,
        Self::StalePresenceExpiryShown,
        Self::PresenceNeverEditsBufferShown,
        Self::BoundToCollaborationStateRegistry,
        Self::PresenceTreatedAsConvergedTruthDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SampledPresenceRateShown => "sampled_presence_rate_shown",
            Self::PresenceIsNonAuthoritativeShown => "presence_is_non_authoritative_shown",
            Self::StalePresenceExpiryShown => "stale_presence_expiry_shown",
            Self::PresenceNeverEditsBufferShown => "presence_never_edits_buffer_shown",
            Self::BoundToCollaborationStateRegistry => "bound_to_collaboration_state_registry",
            Self::PresenceTreatedAsConvergedTruthDisallowed => {
                "presence_treated_as_converged_truth_disallowed"
            }
        }
    }
}

/// Controlled server-ordered comment / annotation / review-pin role and its anchor drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ServerOrderedCommentsAnnotationsReviewPinsRole {
    /// The server ordering shown so comment / pin order is understood as server-fixed.
    ServerOrderingShown,
    /// The anchor-drift history shown append-only so an anchor's movement is auditable.
    AnchorDriftHistoryShownAppendOnly,
    /// The rebind shown reviewable so an anchor rebind is never silent.
    RebindReviewableShown,
    /// The pin-resolution provenance shown so who resolved a pin and when is never ambiguous.
    PinResolutionProvenanceShown,
    /// A role bound to the single collaboration-state registry.
    BoundToCollaborationStateRegistry,
    /// Silently rebinding a comment or pin without drift history, which is disallowed.
    SilentPinRebindWithoutDriftHistoryDisallowed,
}

impl M5ServerOrderedCommentsAnnotationsReviewPinsRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ServerOrderingShown,
        Self::AnchorDriftHistoryShownAppendOnly,
        Self::RebindReviewableShown,
        Self::PinResolutionProvenanceShown,
        Self::BoundToCollaborationStateRegistry,
        Self::SilentPinRebindWithoutDriftHistoryDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServerOrderingShown => "server_ordering_shown",
            Self::AnchorDriftHistoryShownAppendOnly => "anchor_drift_history_shown_append_only",
            Self::RebindReviewableShown => "rebind_reviewable_shown",
            Self::PinResolutionProvenanceShown => "pin_resolution_provenance_shown",
            Self::BoundToCollaborationStateRegistry => "bound_to_collaboration_state_registry",
            Self::SilentPinRebindWithoutDriftHistoryDisallowed => {
                "silent_pin_rebind_without_drift_history_disallowed"
            }
        }
    }
}

/// Controlled presenter / follow-state role for the presenter and follow state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PresenterFollowStateRole {
    /// The presenter holder shown so the surface names who currently presents.
    PresenterHolderShown,
    /// The follow target shown so a follower knows whose viewport they track.
    FollowTargetShown,
    /// The follow-is-view-only posture shown so following never implies control.
    FollowIsViewOnlyShown,
    /// The presenter handoff provenance shown so a handoff names who handed off to whom.
    PresenterHandoffProvenanceShown,
    /// A role bound to the single collaboration-state registry.
    BoundToCollaborationStateRegistry,
    /// Treating follow as implying control or convergence, which is disallowed.
    FollowImpliesControlDisallowed,
}

impl M5PresenterFollowStateRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PresenterHolderShown,
        Self::FollowTargetShown,
        Self::FollowIsViewOnlyShown,
        Self::PresenterHandoffProvenanceShown,
        Self::BoundToCollaborationStateRegistry,
        Self::FollowImpliesControlDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PresenterHolderShown => "presenter_holder_shown",
            Self::FollowTargetShown => "follow_target_shown",
            Self::FollowIsViewOnlyShown => "follow_is_view_only_shown",
            Self::PresenterHandoffProvenanceShown => "presenter_handoff_provenance_shown",
            Self::BoundToCollaborationStateRegistry => "bound_to_collaboration_state_registry",
            Self::FollowImpliesControlDisallowed => "follow_implies_control_disallowed",
        }
    }
}

/// Controlled higher-risk control-plane role for the separate control plane and its degradation banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HigherRiskControlPlaneRole {
    /// The separate control plane shown so a higher-risk control plane is distinct from convergent objects.
    SeparateControlPlaneShown,
    /// Convergence-degraded distinguished from awareness-degraded so the two are never collapsed.
    ConvergenceVersusAwarenessDegradedDistinguished,
    /// The anchor-unresolved state shown so an unresolved anchor is never hidden behind a generic badge.
    AnchorUnresolvedStateShown,
    /// The local unsent work preserved first shown so a downgrade preserves local work before anything else.
    LocalUnsentWorkPreservedFirstShown,
    /// A role bound to the single collaboration-state registry.
    BoundToCollaborationStateRegistry,
    /// Collapsing a degraded state into a generic stale badge, which is disallowed.
    DegradedStateCollapsedIntoGenericStaleDisallowed,
}

impl M5HigherRiskControlPlaneRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SeparateControlPlaneShown,
        Self::ConvergenceVersusAwarenessDegradedDistinguished,
        Self::AnchorUnresolvedStateShown,
        Self::LocalUnsentWorkPreservedFirstShown,
        Self::BoundToCollaborationStateRegistry,
        Self::DegradedStateCollapsedIntoGenericStaleDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SeparateControlPlaneShown => "separate_control_plane_shown",
            Self::ConvergenceVersusAwarenessDegradedDistinguished => {
                "convergence_versus_awareness_degraded_distinguished"
            }
            Self::AnchorUnresolvedStateShown => "anchor_unresolved_state_shown",
            Self::LocalUnsentWorkPreservedFirstShown => "local_unsent_work_preserved_first_shown",
            Self::BoundToCollaborationStateRegistry => "bound_to_collaboration_state_registry",
            Self::DegradedStateCollapsedIntoGenericStaleDisallowed => {
                "degraded_state_collapsed_into_generic_stale_disallowed"
            }
        }
    }
}

/// Controlled sealed session-archive role for the sealed archive and its compaction lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SealedSessionArchiveRole {
    /// The compaction lineage shown so a compacted archive names what it descended from.
    CompactionLineageShown,
    /// The retention and export posture shown so an archive names how long it is kept and how it exports.
    RetentionAndExportPostureShown,
    /// The actor lineage shown so an archive names who produced and sealed it.
    ActorLineageShown,
    /// The policy-labeled redaction shown so an archive export names its redaction policy.
    PolicyLabeledRedactionShown,
    /// A role bound to the single collaboration-state registry.
    BoundToCollaborationStateRegistry,
    /// Exporting an archive without policy-labeled redaction or actor lineage, which is disallowed.
    ArchiveExportedWithoutRedactionOrLineageDisallowed,
}

impl M5SealedSessionArchiveRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CompactionLineageShown,
        Self::RetentionAndExportPostureShown,
        Self::ActorLineageShown,
        Self::PolicyLabeledRedactionShown,
        Self::BoundToCollaborationStateRegistry,
        Self::ArchiveExportedWithoutRedactionOrLineageDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactionLineageShown => "compaction_lineage_shown",
            Self::RetentionAndExportPostureShown => "retention_and_export_posture_shown",
            Self::ActorLineageShown => "actor_lineage_shown",
            Self::PolicyLabeledRedactionShown => "policy_labeled_redaction_shown",
            Self::BoundToCollaborationStateRegistry => "bound_to_collaboration_state_registry",
            Self::ArchiveExportedWithoutRedactionOrLineageDisallowed => {
                "archive_exported_without_redaction_or_lineage_disallowed"
            }
        }
    }
}

/// Claimed M5 surface family that renders / consumes a collaboration-state object class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationStateSurfaceFamily {
    /// The shared editor / buffer collaboration surface.
    SharedEditorSurface,
    /// The shared terminal / debugger collaboration surface.
    SharedTerminalDebugSurface,
    /// The review and comment / annotation surface.
    ReviewAndCommentSurface,
    /// The companion follow / observe surface.
    CompanionFollowSurface,
    /// The search / AI consumer surface.
    SearchAndAiConsumerSurface,
    /// The support / export surface.
    SupportExportSurface,
}

impl M5CollaborationStateSurfaceFamily {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SharedEditorSurface,
        Self::SharedTerminalDebugSurface,
        Self::ReviewAndCommentSurface,
        Self::CompanionFollowSurface,
        Self::SearchAndAiConsumerSurface,
        Self::SupportExportSurface,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedEditorSurface => "shared_editor_surface",
            Self::SharedTerminalDebugSurface => "shared_terminal_debug_surface",
            Self::ReviewAndCommentSurface => "review_and_comment_surface",
            Self::CompanionFollowSurface => "companion_follow_surface",
            Self::SearchAndAiConsumerSurface => "search_and_ai_consumer_surface",
            Self::SupportExportSurface => "support_export_surface",
        }
    }
}

/// Classification stage a class passes through from a declared shared object to a joined and synced replica, an established convergence or ordering, a handled downgrade or drift, and a sealed or compacted session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationStateClassificationStage {
    /// The shared-object-declared stage: the shared object and its authority model are declared.
    SharedObjectDeclared,
    /// The replica-joined-and-synced stage: a replica joins and syncs against the shared object.
    ReplicaJoinedAndSynced,
    /// The convergence-or-ordering-established stage: convergence or server ordering is established.
    ConvergenceOrOrderingEstablished,
    /// The downgrade-or-drift-handled stage: a permission / relay downgrade or anchor drift is handled.
    DowngradeOrDriftHandled,
    /// The session-sealed-or-compacted stage: the session is sealed or compacted into a lineage-preserving manifest.
    SessionSealedOrCompacted,
}

impl M5CollaborationStateClassificationStage {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SharedObjectDeclared,
        Self::ReplicaJoinedAndSynced,
        Self::ConvergenceOrOrderingEstablished,
        Self::DowngradeOrDriftHandled,
        Self::SessionSealedOrCompacted,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedObjectDeclared => "shared_object_declared",
            Self::ReplicaJoinedAndSynced => "replica_joined_and_synced",
            Self::ConvergenceOrOrderingEstablished => "convergence_or_ordering_established",
            Self::DowngradeOrDriftHandled => "downgrade_or_drift_handled",
            Self::SessionSealedOrCompacted => "session_sealed_or_compacted",
        }
    }
}

/// Shared consumer surface that must agree on a class's collaboration-state truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationStateConsumerSurface {
    /// The shared editor replica view.
    SharedEditorReplicaView,
    /// The presence / cursor layer.
    PresenceCursorLayer,
    /// The comment / annotation / review-pin layer.
    CommentAnnotationReviewPinLayer,
    /// The presenter / follow banner.
    PresenterFollowBanner,
    /// The collaboration degradation banner.
    CollaborationDegradationBanner,
    /// The session archive and compaction view.
    SessionArchiveAndCompactionView,
    /// The search / AI provenance consumer.
    SearchAndAiProvenanceConsumer,
    /// The support / export packet.
    SupportExportPacket,
    /// The help / docs surface.
    HelpDocs,
}

impl M5CollaborationStateConsumerSurface {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::SharedEditorReplicaView,
        Self::PresenceCursorLayer,
        Self::CommentAnnotationReviewPinLayer,
        Self::PresenterFollowBanner,
        Self::CollaborationDegradationBanner,
        Self::SessionArchiveAndCompactionView,
        Self::SearchAndAiProvenanceConsumer,
        Self::SupportExportPacket,
        Self::HelpDocs,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedEditorReplicaView => "shared_editor_replica_view",
            Self::PresenceCursorLayer => "presence_cursor_layer",
            Self::CommentAnnotationReviewPinLayer => "comment_annotation_review_pin_layer",
            Self::PresenterFollowBanner => "presenter_follow_banner",
            Self::CollaborationDegradationBanner => "collaboration_degradation_banner",
            Self::SessionArchiveAndCompactionView => "session_archive_and_compaction_view",
            Self::SearchAndAiProvenanceConsumer => "search_and_ai_provenance_consumer",
            Self::SupportExportPacket => "support_export_packet",
            Self::HelpDocs => "help_docs",
        }
    }
}

/// Non-visual / accessibility route every class must offer so no collaboration-state meaning disappears under zoom, high contrast, keyboard-only use, or export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationStateAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-visual cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Preserves truth under high-contrast and forced-colors modes.
    HighContrastSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5CollaborationStateAccessibilityRoute {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::HighContrastSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a class has degraded below its qualified collaboration-state-handling state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationStateDegradedReason {
    /// The authority model for the shared object is unresolved.
    AuthorityModelUnresolved,
    /// The convergence state is unknown.
    ConvergenceStateUnknown,
    /// Local unsent work is at risk of being lost.
    LocalUnsentWorkAtRisk,
    /// Anchor drift is unreviewed.
    AnchorDriftUnreviewed,
    /// The export posture is unknown.
    ExportPostureUnknown,
    /// The provenance or freshness of the collaboration state is unknown.
    ProvenanceOrFreshnessUnknown,
}

impl M5CollaborationStateDegradedReason {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AuthorityModelUnresolved,
        Self::ConvergenceStateUnknown,
        Self::LocalUnsentWorkAtRisk,
        Self::AnchorDriftUnreviewed,
        Self::ExportPostureUnknown,
        Self::ProvenanceOrFreshnessUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthorityModelUnresolved => "authority_model_unresolved",
            Self::ConvergenceStateUnknown => "convergence_state_unknown",
            Self::LocalUnsentWorkAtRisk => "local_unsent_work_at_risk",
            Self::AnchorDriftUnreviewed => "anchor_drift_unreviewed",
            Self::ExportPostureUnknown => "export_posture_unknown",
            Self::ProvenanceOrFreshnessUnknown => "provenance_or_freshness_unknown",
        }
    }
}

/// Mandatory label a claimed collaboration-state class must be able to show. The first three are hard requirements; the remaining three make the authority model, the convergence state, and the export posture mechanically distinct for every covered class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationStateRequiredLabel {
    /// The class's stable identity.
    Identity,
    /// The class's collaboration-state lifecycle role.
    LifecycleRole,
    /// The canonical per-domain descriptor the class points at.
    CanonicalReference,
    /// The authority model the class must state.
    AuthorityModel,
    /// The convergence state the class must show.
    ConvergenceState,
    /// The export posture the class must state.
    ExportPosture,
}

impl M5CollaborationStateRequiredLabel {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::LifecycleRole,
        Self::CanonicalReference,
        Self::AuthorityModel,
        Self::ConvergenceState,
        Self::ExportPosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::LifecycleRole => "lifecycle_role",
            Self::CanonicalReference => "canonical_reference",
            Self::AuthorityModel => "authority_model",
            Self::ConvergenceState => "convergence_state",
            Self::ExportPosture => "export_posture",
        }
    }
    /// The three labels every claimed class must be able to show.
    pub const MANDATORY: [Self; 3] = [
        Self::Identity,
        Self::LifecycleRole,
        Self::CanonicalReference,
    ];
}

/// Qualification class for an M5 collaboration-state row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationStateQualificationClass {
    /// Class collaboration-state handling qualifies for the Stable claim.
    Stable,
    /// Class collaboration-state handling is narrowed to Beta.
    Beta,
    /// Class collaboration-state handling is narrowed to Preview.
    Preview,
    /// Class collaboration-state handling is experimental and not claimed.
    Experimental,
    /// Class collaboration-state handling is unavailable on this build.
    Unavailable,
    /// Class collaboration-state handling is held pending review.
    Held,
}

impl M5CollaborationStateQualificationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Experimental,
        Self::Unavailable,
        Self::Held,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }
    /// Whether the class may carry a public Stable collaboration-state-handling claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a collaboration-state class below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationStateDowngradeTrigger {
    /// A replica overwrote the canonical local buffer, VFS, or Git truth.
    ReplicaOverwroteLocalCanonicalTruth,
    /// Unsent local edits were discarded on a permission downgrade, relay failure, or leave-session flow.
    UnsentLocalEditsDiscardedOnDowngrade,
    /// A comment, annotation, or review pin was rebound without append-only drift history.
    CommentOrPinReboundWithoutDriftHistory,
    /// A convergence-degraded or awareness-degraded state was collapsed into a generic stale badge.
    ConvergenceOrAwarenessDegradedCollapsedIntoGenericStale,
    /// An op-log, snapshot, or archive was exported without policy-labeled redaction and actor lineage.
    OpLogOrArchiveExportedWithoutRedactionOrLineage,
    /// A class left its authority model unstated.
    AuthorityModelUnstated,
    /// A class left its convergence state unstated.
    ConvergenceStateUnstated,
    /// A class left its local-truth preservation posture unstated.
    LocalTruthPreservationUnstated,
    /// A class left its anchor-drift history unstated.
    AnchorDriftHistoryUnstated,
    /// A class left its export posture unstated.
    ExportPostureUnstated,
    /// A class left its provenance or freshness unstated.
    ProvenanceOrFreshnessUnstated,
    /// The collaboration-state matrix packet has gone stale.
    CollaborationStateMatrixStale,
}

impl M5CollaborationStateDowngradeTrigger {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ReplicaOverwroteLocalCanonicalTruth,
        Self::UnsentLocalEditsDiscardedOnDowngrade,
        Self::CommentOrPinReboundWithoutDriftHistory,
        Self::ConvergenceOrAwarenessDegradedCollapsedIntoGenericStale,
        Self::OpLogOrArchiveExportedWithoutRedactionOrLineage,
        Self::AuthorityModelUnstated,
        Self::ConvergenceStateUnstated,
        Self::LocalTruthPreservationUnstated,
        Self::AnchorDriftHistoryUnstated,
        Self::ExportPostureUnstated,
        Self::ProvenanceOrFreshnessUnstated,
        Self::CollaborationStateMatrixStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReplicaOverwroteLocalCanonicalTruth => "replica_overwrote_local_canonical_truth",
            Self::UnsentLocalEditsDiscardedOnDowngrade => {
                "unsent_local_edits_discarded_on_downgrade"
            }
            Self::CommentOrPinReboundWithoutDriftHistory => {
                "comment_or_pin_rebound_without_drift_history"
            }
            Self::ConvergenceOrAwarenessDegradedCollapsedIntoGenericStale => {
                "convergence_or_awareness_degraded_collapsed_into_generic_stale"
            }
            Self::OpLogOrArchiveExportedWithoutRedactionOrLineage => {
                "op_log_or_archive_exported_without_redaction_or_lineage"
            }
            Self::AuthorityModelUnstated => "authority_model_unstated",
            Self::ConvergenceStateUnstated => "convergence_state_unstated",
            Self::LocalTruthPreservationUnstated => "local_truth_preservation_unstated",
            Self::AnchorDriftHistoryUnstated => "anchor_drift_history_unstated",
            Self::ExportPostureUnstated => "export_posture_unstated",
            Self::ProvenanceOrFreshnessUnstated => "provenance_or_freshness_unstated",
            Self::CollaborationStateMatrixStale => "collaboration_state_matrix_stale",
        }
    }
}

/// Required visible state a class must carry so a collaboration-state result never reads without its authority
/// model, convergence state, local-truth disposition, merge / drift summary, export posture, or provenance /
/// freshness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationStateVisibleState {
    /// Class / surface label shown on the surface (replica view, presence layer, pin layer, degradation banner, archive view).
    pub surface_label: String,
    /// Authority model — whether the object converges, is server-ordered, host-authoritative, or defers to local canonical truth.
    pub authority_model: String,
    /// Convergence state — converged, converging, degraded, anchor-unresolved, sealed, or otherwise.
    pub convergence_state: String,
    /// Local-truth disposition — how the canonical local buffer / VFS / Git truth is preserved and never replaced.
    pub local_truth_disposition: String,
    /// Merge and drift summary — how concurrent edits merge and how anchors drift append-only.
    pub merge_and_drift_summary: String,
    /// Export posture — policy-labeled redaction and actor lineage applied to op-logs, snapshots, and archives.
    pub export_posture: String,
    /// Provenance and freshness backing the collaboration state consumed by search, AI, review, docs, and support.
    pub provenance_and_freshness: String,
}

impl M5CollaborationStateVisibleState {
    /// `true` when every required visible-state field is present.
    fn is_complete(&self) -> bool {
        !self.surface_label.trim().is_empty()
            && !self.authority_model.trim().is_empty()
            && !self.convergence_state.trim().is_empty()
            && !self.local_truth_disposition.trim().is_empty()
            && !self.merge_and_drift_summary.trim().is_empty()
            && !self.export_posture.trim().is_empty()
            && !self.provenance_and_freshness.trim().is_empty()
    }
}

/// One row in the matrix: one governed collaboration-state object class bound to the surface-specific
/// collaboration-state truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationStateRow {
    /// Governed collaboration-state object class.
    pub object_class: M5CollaborationStateObject,
    /// Qualification class earned by this class's collaboration-state handling.
    pub qualification: M5CollaborationStateQualificationClass,
    /// Convergence state this row governs (distinguishes a converged object from a converging, degraded, anchor-unresolved, or sealed object).
    pub convergence_state: M5ConvergenceState,
    /// Owner role accountable for keeping this class's collaboration-state governed.
    pub owner_role: String,
    /// Backup owner role accountable when the primary owner is unavailable.
    pub backup_owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Required visible state that keeps this class's collaboration-state result visibly owned, authority-declared, and export-honest.
    pub required_visible_state: M5CollaborationStateVisibleState,
    /// Claimed M5 surface families that render / consume this class.
    pub surface_families: Vec<M5CollaborationStateSurfaceFamily>,
    /// Classification stages this class passes through from a declared shared object to a sealed or compacted session.
    pub classification_stages: Vec<M5CollaborationStateClassificationStage>,
    /// Mandatory labels this class must be able to show (must include the three
    /// [`M5CollaborationStateRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5CollaborationStateRequiredLabel>,
    /// Collaboration-state roles this class can carry (the frozen AC vocabulary; required on every class).
    pub semantic_roles: Vec<M5CollaborationStateRole>,
    /// CRDT-backed shared-text roles this class names (CrdtBackedSharedText only).
    pub crdt_backed_shared_text_roles: Vec<M5CrdtBackedSharedTextRole>,
    /// Sampled-presence roles this class names (SampledPresenceCursorsSelections only).
    pub sampled_presence_cursors_selections_roles: Vec<M5SampledPresenceCursorsSelectionsRole>,
    /// Server-ordered comment / pin roles this class names (ServerOrderedCommentsAnnotationsReviewPins only).
    pub server_ordered_comments_annotations_review_pins_roles:
        Vec<M5ServerOrderedCommentsAnnotationsReviewPinsRole>,
    /// Presenter / follow roles this class names (PresenterFollowState only).
    pub presenter_follow_state_roles: Vec<M5PresenterFollowStateRole>,
    /// Higher-risk control-plane roles this class names (HigherRiskControlPlane only).
    pub higher_risk_control_plane_roles: Vec<M5HigherRiskControlPlaneRole>,
    /// Sealed session-archive roles this class names (SealedSessionArchive only).
    pub sealed_session_archive_roles: Vec<M5SealedSessionArchiveRole>,
    /// Degraded reasons this class can name (required on every class).
    pub degraded_reasons: Vec<M5CollaborationStateDegradedReason>,
    /// Non-visual accessibility routes this class offers.
    pub accessibility_routes: Vec<M5CollaborationStateAccessibilityRoute>,
    /// First consumer surfaces that consume this class's collaboration-state projection.
    pub consumer_surfaces: Vec<M5CollaborationStateConsumerSurface>,
    /// Downgrade triggers that apply to this class.
    pub downgrade_triggers: Vec<M5CollaborationStateDowngradeTrigger>,
    /// Required closure-artifact refs that keep this class's collaboration-state provable.
    pub required_closure_artifact_refs: Vec<String>,
    /// Source contract refs consumed by this class (must include its own canonical domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this class never lets a collaboration replica overwrite the canonical local buffer, VFS, or Git truth implicitly. MUST be `false`.
    pub lets_a_replica_overwrite_local_buffer_vfs_or_git_truth_implicitly: bool,
    /// Hard invariant: this class never discards unsent local edits on a permission downgrade, relay failure, or leave-session flow. MUST be `false`.
    pub discards_unsent_local_edits_on_permission_downgrade_relay_failure_or_leave: bool,
    /// Hard invariant: this class never rebinds comments, annotations, or review pins without append-only drift history. MUST be `false`.
    pub rebinds_comments_annotations_or_review_pins_without_drift_history: bool,
    /// Hard invariant: this class never collapses a convergence-degraded or awareness-degraded state into a generic stale or broken badge. MUST be `false`.
    pub collapses_convergence_or_awareness_degraded_state_into_a_generic_stale_badge: bool,
    /// Hard invariant: this class never exports op-logs, snapshots, or archives without policy-labeled redaction and actor lineage. MUST be `false`.
    pub exports_op_logs_snapshots_or_archives_without_policy_labeled_redaction_and_lineage: bool,
}

impl M5CollaborationStateRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CollaborationStateRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CollaborationStateRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.lets_a_replica_overwrite_local_buffer_vfs_or_git_truth_implicitly
            && !self.discards_unsent_local_edits_on_permission_downgrade_relay_failure_or_leave
            && !self.rebinds_comments_annotations_or_review_pins_without_drift_history
            && !self.collapses_convergence_or_awareness_degraded_state_into_a_generic_stale_badge
            && !self
                .exports_op_logs_snapshots_or_archives_without_policy_labeled_redaction_and_lineage
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationStateVocabularySet {
    /// Object classes tokens.
    pub object_classes: Vec<String>,
    /// Convergence states tokens.
    pub convergence_states: Vec<String>,
    /// Authority models tokens.
    pub authority_models: Vec<String>,
    /// Downgrade gates tokens.
    pub downgrade_gates: Vec<String>,
    /// Semantic roles tokens.
    pub semantic_roles: Vec<String>,
    /// CRDT-backed shared-text roles tokens.
    pub crdt_backed_shared_text_roles: Vec<String>,
    /// Sampled-presence roles tokens.
    pub sampled_presence_cursors_selections_roles: Vec<String>,
    /// Server-ordered comment / pin roles tokens.
    pub server_ordered_comments_annotations_review_pins_roles: Vec<String>,
    /// Presenter / follow roles tokens.
    pub presenter_follow_state_roles: Vec<String>,
    /// Higher-risk control-plane roles tokens.
    pub higher_risk_control_plane_roles: Vec<String>,
    /// Sealed session-archive roles tokens.
    pub sealed_session_archive_roles: Vec<String>,
    /// Surface families tokens.
    pub surface_families: Vec<String>,
    /// Classification stages tokens.
    pub classification_stages: Vec<String>,
    /// Consumer surfaces tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility routes tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded reasons tokens.
    pub degraded_reasons: Vec<String>,
    /// Required labels tokens.
    pub required_labels: Vec<String>,
    /// Downgrade triggers tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5CollaborationStateVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            object_classes: tokens(&M5CollaborationStateObject::ALL, |v| v.as_str()),
            convergence_states: tokens(&M5ConvergenceState::ALL, |v| v.as_str()),
            authority_models: tokens(&M5CollaborationAuthorityModel::ALL, |v| v.as_str()),
            downgrade_gates: tokens(&M5CollaborationDowngradeGate::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5CollaborationStateRole::ALL, |v| v.as_str()),
            crdt_backed_shared_text_roles: tokens(&M5CrdtBackedSharedTextRole::ALL, |v| v.as_str()),
            sampled_presence_cursors_selections_roles: tokens(
                &M5SampledPresenceCursorsSelectionsRole::ALL,
                |v| v.as_str(),
            ),
            server_ordered_comments_annotations_review_pins_roles: tokens(
                &M5ServerOrderedCommentsAnnotationsReviewPinsRole::ALL,
                |v| v.as_str(),
            ),
            presenter_follow_state_roles: tokens(&M5PresenterFollowStateRole::ALL, |v| v.as_str()),
            higher_risk_control_plane_roles: tokens(&M5HigherRiskControlPlaneRole::ALL, |v| {
                v.as_str()
            }),
            sealed_session_archive_roles: tokens(&M5SealedSessionArchiveRole::ALL, |v| v.as_str()),
            surface_families: tokens(&M5CollaborationStateSurfaceFamily::ALL, |v| v.as_str()),
            classification_stages: tokens(&M5CollaborationStateClassificationStage::ALL, |v| {
                v.as_str()
            }),
            consumer_surfaces: tokens(&M5CollaborationStateConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5CollaborationStateAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            degraded_reasons: tokens(&M5CollaborationStateDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5CollaborationStateRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5CollaborationStateDowngradeTrigger::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationStateGovernanceReview {
    /// No collaboration replica overwrites the canonical local buffer, VFS, or Git truth.
    pub no_replica_overwrites_local_buffer_vfs_or_git_truth: bool,
    /// Every covered object class names owner backup owner and first consumer.
    pub every_covered_object_class_names_owner_backup_owner_and_first_consumer: bool,
    /// A converged object state is mechanically distinct from a degraded one.
    pub converged_state_is_mechanically_distinct_from_degraded: bool,
    /// Every shared object declares its authority model.
    pub every_shared_object_declares_its_authority_model: bool,
    /// Permission or relay downgrade preserves local unsent work first.
    pub permission_or_relay_downgrade_preserves_local_unsent_work_first: bool,
    /// Every comment or pin rebind carries append-only drift history.
    pub every_comment_or_pin_rebind_carries_append_only_drift_history: bool,
    /// Convergence and awareness degraded states are never collapsed into generic stale.
    pub convergence_and_awareness_degraded_states_are_never_collapsed_into_generic_stale: bool,
    /// Presence and follow never imply convergence or control.
    pub presence_and_follow_never_imply_convergence_or_control: bool,
    /// No op-log or archive exports without policy-labeled redaction and lineage.
    pub no_op_log_or_archive_exports_without_policy_labeled_redaction_and_lineage: bool,
    /// Every object declares classification stages.
    pub every_object_declares_classification_stages: bool,
    /// Every object declares accessibility route.
    pub every_object_declares_accessibility_route: bool,
    /// Support export reads single collaboration-state source.
    pub support_export_reads_single_collaboration_state_source: bool,
    /// Editor, terminal, review, companion, search, and support bind to a single source.
    pub editor_terminal_review_companion_search_and_support_bind_to_single_source: bool,
    /// Later rows cannot invent parallel collaboration-state vocabulary.
    pub later_rows_cannot_invent_parallel_collaboration_state_vocabulary: bool,
    /// Collaboration-state truth survives zoom and high contrast.
    pub collaboration_state_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when matrix row missing or stale.
    pub claims_narrow_automatically_when_matrix_row_missing_or_stale: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationStateConsumerProjection {
    /// Shared editor and presence layers consume shared collaboration-state truth.
    pub shared_editor_and_presence_layers_consume_shared_collaboration_state_truth: bool,
    /// Comment / pin and presenter / follow consume shared authority and drift truth.
    pub comment_pin_and_presenter_follow_consume_shared_authority_and_drift_truth: bool,
    /// Help and support export consume shared convergence and export truth.
    pub help_and_support_export_consume_shared_convergence_and_export_truth: bool,
    /// Docs help and screenshots read single collaboration-state source.
    pub docs_help_and_screenshots_read_single_collaboration_state_source: bool,
    /// Companion and search / AI surfaces bind to shared convergence-state source.
    pub companion_and_search_ai_surfaces_bind_to_shared_convergence_state_source: bool,
    /// Support export reads single collaboration-state source.
    pub support_export_reads_single_collaboration_state_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationStateProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof / audit refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the class.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the collaboration-state lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationStateReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting collaboration-state audit for the lane.
    pub collaboration_state_audit_ref: String,
    /// True when support/export parity is required for every class.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every class.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5CollaborationStateMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CollaborationStateMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Collaboration-state rows.
    pub collaboration_state_rows: Vec<M5CollaborationStateRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CollaborationStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CollaborationStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CollaborationStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CollaborationStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CollaborationStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 collaboration-state matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationStateMatrixPacket {
    /// Record kind; must equal [`M5_COLLABORATION_STATE_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COLLABORATION_STATE_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Collaboration-state rows.
    pub collaboration_state_rows: Vec<M5CollaborationStateRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CollaborationStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CollaborationStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CollaborationStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CollaborationStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CollaborationStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CollaborationStateMatrixPacket {
    /// Builds an M5 collaboration-state matrix packet from input.
    pub fn new(input: M5CollaborationStateMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_COLLABORATION_STATE_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_COLLABORATION_STATE_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            collaboration_state_rows: input.collaboration_state_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 collaboration-state matrix invariants.
    pub fn validate(&self) -> Vec<M5CollaborationStateMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_COLLABORATION_STATE_MATRIX_RECORD_KIND {
            violations.push(M5CollaborationStateMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COLLABORATION_STATE_MATRIX_SCHEMA_VERSION {
            violations.push(M5CollaborationStateMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CollaborationStateMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_collaboration_state_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 collaboration-state matrix serializes"),
        ) {
            violations.push(M5CollaborationStateMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 collaboration-state matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed collaboration-state class.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object_class,qualification,convergence_state,owner,backup_owner,canonical_schema,surface_families,classification_stages,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.collaboration_state_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.convergence_state.as_str(),
                csv_field(&row.owner_role),
                csv_field(&row.backup_owner_role),
                row.object_class.canonical_domain_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.classification_stages, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic collaboration-convergence-health dashboard JSON that session and support surfaces render from
    /// one canonical matrix instead of hand-authoring readiness chrome.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn render_dashboard_json(&self) -> String {
        let objects: Vec<serde_json::Value> = self
            .collaboration_state_rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "object_class": row.object_class.as_str(),
                    "qualification": row.qualification.as_str(),
                    "convergence_state": row.convergence_state.as_str(),
                    "canonical_schema": row.object_class.canonical_domain_schema_ref(),
                    "classification_stages": row
                        .classification_stages
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                    "consumer_surfaces": row
                        .consumer_surfaces
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                })
            })
            .collect();
        let dashboard = serde_json::json!({
            "record_kind": "m5_collaboration_convergence_health",
            "packet_id": self.packet_id,
            "matrix_label": self.matrix_label,
            "matrix_schema_ref": M5_COLLABORATION_STATE_MATRIX_SCHEMA_REF,
            "support_export_ref": M5_COLLABORATION_STATE_ARTIFACT_REF,
            "classification_stages": self.vocabulary_set.classification_stages,
            "downgrade_triggers": self.vocabulary_set.downgrade_triggers,
            "objects": objects,
        });
        serde_json::to_string_pretty(&dashboard)
            .expect("m5 collaboration-convergence-health dashboard serializes")
    }

    /// Deterministic Markdown report for support, docs, or collaboration handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_objects = self
            .collaboration_state_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Collaboration-Replica, Shared-Object-Authority, Anchor-Drift, Convergence-State, and Session-Archive Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Object classes: {} ({} stable)\n",
            self.collaboration_state_rows.len(),
            stable_objects
        ));
        out.push_str(&format!(
            "- Collaboration-state roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Classification stages: {}\n",
            self.vocabulary_set.classification_stages.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last audit: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Object classes\n\n");
        for row in &self.collaboration_state_rows {
            out.push_str(&format!(
                "- **{}**: `{}` (convergence_state: `{}`)\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.convergence_state.as_str()
            ));
            out.push_str(&format!(
                "  - Owner: {} (backup: {})\n",
                row.owner_role, row.backup_owner_role
            ));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.object_class.canonical_domain_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Authority model: {}\n",
                row.required_visible_state.authority_model
            ));
            out.push_str(&format!(
                "  - Export posture: {}\n",
                row.required_visible_state.export_posture
            ));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 collaboration-state matrix export.
#[derive(Debug)]
pub enum M5CollaborationStateMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CollaborationStateMatrixViolation>),
}

impl fmt::Display for M5CollaborationStateMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 collaboration-state matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 collaboration-state matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CollaborationStateMatrixArtifactError {}

/// Validation failures emitted by [`M5CollaborationStateMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CollaborationStateMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed object class is missing from the matrix.
    RequiredObjectMissing,
    /// A collaboration-state row is incomplete.
    CollaborationStateRowIncomplete,
    /// A collaboration-state row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A collaboration-state row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A class declares no collaboration-state roles.
    SemanticRoleMissing,
    /// The CrdtBackedSharedText class declares no CrdtBackedSharedText roles.
    CrdtBackedSharedTextRoleMissing,
    /// The SampledPresenceCursorsSelections class declares no SampledPresenceCursorsSelections roles.
    SampledPresenceCursorsSelectionsRoleMissing,
    /// The ServerOrderedCommentsAnnotationsReviewPins class declares no such roles.
    ServerOrderedCommentsAnnotationsReviewPinsRoleMissing,
    /// The PresenterFollowState class declares no PresenterFollowState roles.
    PresenterFollowStateRoleMissing,
    /// The HigherRiskControlPlane class declares no HigherRiskControlPlane roles.
    HigherRiskControlPlaneRoleMissing,
    /// The SealedSessionArchive class declares no SealedSessionArchive roles.
    SealedSessionArchiveRoleMissing,
    /// A class omits required visible-state fields.
    VisibleStateIncomplete,
    /// A class declares no degraded reasons.
    DegradedReasonMissing,
    /// A class declares no surface families.
    SurfaceFamilyMissing,
    /// A class declares no classification stages.
    ClassificationStageMissing,
    /// A class declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A class declares no first consumer surfaces.
    ConsumerSurfacesMissing,
    /// A class declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A class claiming Stable is missing required closure-artifact refs.
    StableObjectMissingClosureArtifact,
    /// A class violates a hard collaboration-state invariant.
    CollaborationStateInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5CollaborationStateMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredObjectMissing => "required_object_missing",
            Self::CollaborationStateRowIncomplete => "collaboration_state_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::CrdtBackedSharedTextRoleMissing => "crdt_backed_shared_text_role_missing",
            Self::SampledPresenceCursorsSelectionsRoleMissing => {
                "sampled_presence_cursors_selections_role_missing"
            }
            Self::ServerOrderedCommentsAnnotationsReviewPinsRoleMissing => {
                "server_ordered_comments_annotations_review_pins_role_missing"
            }
            Self::PresenterFollowStateRoleMissing => "presenter_follow_state_role_missing",
            Self::HigherRiskControlPlaneRoleMissing => "higher_risk_control_plane_role_missing",
            Self::SealedSessionArchiveRoleMissing => "sealed_session_archive_role_missing",
            Self::VisibleStateIncomplete => "visible_state_incomplete",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::ClassificationStageMissing => "classification_stage_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableObjectMissingClosureArtifact => "stable_object_missing_closure_artifact",
            Self::CollaborationStateInvariantViolated => "collaboration_state_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 collaboration-state matrix export.
pub fn current_stable_m5_collaboration_state_matrix_export(
) -> Result<M5CollaborationStateMatrixPacket, M5CollaborationStateMatrixArtifactError> {
    let packet: M5CollaborationStateMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-collaboration-convergence-proof/support_export.json"
    )))
    .map_err(M5CollaborationStateMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CollaborationStateMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5CollaborationStateMatrixPacket,
    violations: &mut Vec<M5CollaborationStateMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_COLLABORATION_STATE_MATRIX_SCHEMA_REF,
        M5_COLLABORATION_STATE_MATRIX_DOC_REF,
        M5_CRDT_BACKED_SHARED_TEXT_DOMAIN_SCHEMA_REF,
        M5_SAMPLED_PRESENCE_CURSORS_SELECTIONS_DOMAIN_SCHEMA_REF,
        M5_SERVER_ORDERED_COMMENTS_ANNOTATIONS_REVIEW_PINS_DOMAIN_SCHEMA_REF,
        M5_PRESENTER_FOLLOW_STATE_DOMAIN_SCHEMA_REF,
        M5_HIGHER_RISK_CONTROL_PLANE_DOMAIN_SCHEMA_REF,
        M5_SEALED_SESSION_ARCHIVE_DOMAIN_SCHEMA_REF,
        M5_COLLABORATION_CONTROL_MATRIX_LANDED_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CollaborationStateMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5CollaborationStateMatrixPacket,
    violations: &mut Vec<M5CollaborationStateMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5CollaborationStateMatrixViolation::VocabularySetDrift);
    }
}

fn validate_collaboration_state_rows(
    packet: &M5CollaborationStateMatrixPacket,
    violations: &mut Vec<M5CollaborationStateMatrixViolation>,
) {
    let present: BTreeSet<M5CollaborationStateObject> = packet
        .collaboration_state_rows
        .iter()
        .map(|row| row.object_class)
        .collect();
    for required in M5CollaborationStateObject::ALL {
        if !present.contains(&required) {
            violations.push(M5CollaborationStateMatrixViolation::RequiredObjectMissing);
            return;
        }
    }

    for row in &packet.collaboration_state_rows {
        let class = row.object_class;
        if row.owner_role.trim().is_empty()
            || row.backup_owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5CollaborationStateMatrixViolation::CollaborationStateRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5CollaborationStateMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == class.canonical_domain_schema_ref())
        {
            violations.push(M5CollaborationStateMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5CollaborationStateMatrixViolation::SemanticRoleMissing);
        }
        if class.declares_crdt_backed_shared_text_roles()
            && row.crdt_backed_shared_text_roles.is_empty()
        {
            violations.push(M5CollaborationStateMatrixViolation::CrdtBackedSharedTextRoleMissing);
        }
        if class.declares_sampled_presence_cursors_selections_roles()
            && row.sampled_presence_cursors_selections_roles.is_empty()
        {
            violations.push(
                M5CollaborationStateMatrixViolation::SampledPresenceCursorsSelectionsRoleMissing,
            );
        }
        if class.declares_server_ordered_comments_annotations_review_pins_roles()
            && row
                .server_ordered_comments_annotations_review_pins_roles
                .is_empty()
        {
            violations.push(
                M5CollaborationStateMatrixViolation::ServerOrderedCommentsAnnotationsReviewPinsRoleMissing,
            );
        }
        if class.declares_presenter_follow_state_roles()
            && row.presenter_follow_state_roles.is_empty()
        {
            violations.push(M5CollaborationStateMatrixViolation::PresenterFollowStateRoleMissing);
        }
        if class.declares_higher_risk_control_plane_roles()
            && row.higher_risk_control_plane_roles.is_empty()
        {
            violations.push(M5CollaborationStateMatrixViolation::HigherRiskControlPlaneRoleMissing);
        }
        if class.declares_sealed_session_archive_roles()
            && row.sealed_session_archive_roles.is_empty()
        {
            violations.push(M5CollaborationStateMatrixViolation::SealedSessionArchiveRoleMissing);
        }
        if !row.required_visible_state.is_complete() {
            violations.push(M5CollaborationStateMatrixViolation::VisibleStateIncomplete);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5CollaborationStateMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5CollaborationStateMatrixViolation::SurfaceFamilyMissing);
        }
        if row.classification_stages.is_empty() {
            violations.push(M5CollaborationStateMatrixViolation::ClassificationStageMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5CollaborationStateMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5CollaborationStateMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5CollaborationStateMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_closure_artifact_refs.is_empty() {
            violations
                .push(M5CollaborationStateMatrixViolation::StableObjectMissingClosureArtifact);
        }
        if !row.honours_invariants() {
            violations
                .push(M5CollaborationStateMatrixViolation::CollaborationStateInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5CollaborationStateMatrixPacket,
    violations: &mut Vec<M5CollaborationStateMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.no_replica_overwrites_local_buffer_vfs_or_git_truth,
        review.every_covered_object_class_names_owner_backup_owner_and_first_consumer,
        review.converged_state_is_mechanically_distinct_from_degraded,
        review.every_shared_object_declares_its_authority_model,
        review.permission_or_relay_downgrade_preserves_local_unsent_work_first,
        review.every_comment_or_pin_rebind_carries_append_only_drift_history,
        review.convergence_and_awareness_degraded_states_are_never_collapsed_into_generic_stale,
        review.presence_and_follow_never_imply_convergence_or_control,
        review.no_op_log_or_archive_exports_without_policy_labeled_redaction_and_lineage,
        review.every_object_declares_classification_stages,
        review.every_object_declares_accessibility_route,
        review.support_export_reads_single_collaboration_state_source,
        review.editor_terminal_review_companion_search_and_support_bind_to_single_source,
        review.later_rows_cannot_invent_parallel_collaboration_state_vocabulary,
        review.collaboration_state_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_matrix_row_missing_or_stale,
    ] {
        if !ok {
            violations.push(M5CollaborationStateMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CollaborationStateMatrixPacket,
    violations: &mut Vec<M5CollaborationStateMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shared_editor_and_presence_layers_consume_shared_collaboration_state_truth,
        projection.comment_pin_and_presenter_follow_consume_shared_authority_and_drift_truth,
        projection.help_and_support_export_consume_shared_convergence_and_export_truth,
        projection.docs_help_and_screenshots_read_single_collaboration_state_source,
        projection.companion_and_search_ai_surfaces_bind_to_shared_convergence_state_source,
        projection.support_export_reads_single_collaboration_state_source,
    ] {
        if !ok {
            violations.push(M5CollaborationStateMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CollaborationStateMatrixPacket,
    violations: &mut Vec<M5CollaborationStateMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CollaborationStateMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5CollaborationStateMatrixPacket,
    violations: &mut Vec<M5CollaborationStateMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.collaboration_state_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5CollaborationStateMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled vocabulary
/// deliberately uses convergence / replica / anchor / archive words; what is rejected is a raw secret / payload
/// *value* shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
