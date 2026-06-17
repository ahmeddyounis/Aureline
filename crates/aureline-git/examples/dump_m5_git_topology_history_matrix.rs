//! Conformance dump for the frozen M5 repository-topology and history-surgery matrix.
//!
//! Prints the canonical export-safe packet as deterministic JSON. The optional
//! first argument selects a narrowed fixture variant instead of the canonical
//! packet:
//!
//! * (no argument) — the canonical frozen matrix
//! * `submodule` — an uninitialized submodule root narrowed to mutation-denied
//! * `reset` — a reset whose recovery falls back to reflog-only disclosure
//!
//! These three documents are the source of the checked-in support export and
//! the protected narrowing fixtures.

use aureline_git::{
    DegradedStateRow, DegradedTopologyState, HistorySurgerySession,
    M5GitTopologyHistoryMatrixPacket, M5GitTopologyHistoryMatrixPacketInput, MatrixConsumerParity,
    MatrixConsumerSurface, MatrixFreezePosture, MatrixGovernanceReview, OperationPreviewClass,
    OperationRecoveryClass, RiskyHistoryOperation, RiskyOperationRow, SessionObjectRow,
    TopologyClassRow, M5_GIT_TOPOLOGY_HISTORY_MATRIX_CONFLICT_SESSION_CONTRACT_REF,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_DOC_REF,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_RECOVERY_CHECKPOINT_CONTRACT_REF,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_REF_UPDATE_CONTRACT_REF,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_SCHEMA_REF,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_SEQUENCE_EDIT_CONTRACT_REF,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_STASH_CONTRACT_REF,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_TOPOLOGY_CONTRACT_REF,
};
use aureline_git::{RepositoryTopologyClass, TopologyHonestyLabel, TopologyOperationScope};

fn topology_row(
    topology_class: RepositoryTopologyClass,
    scope_summary: &str,
    honesty_label: TopologyHonestyLabel,
    degraded_states: Vec<DegradedTopologyState>,
    mutation_control: TopologyOperationScope,
    preview_class: OperationPreviewClass,
    recovery_class: OperationRecoveryClass,
) -> TopologyClassRow {
    TopologyClassRow {
        topology_class,
        scope_summary: scope_summary.to_owned(),
        honesty_label,
        degraded_states,
        mutation_control,
        preview_class,
        recovery_class,
        provider_overlay_never_overwrites_local_truth: true,
        consumer_surfaces: vec![
            MatrixConsumerSurface::Search,
            MatrixConsumerSurface::Review,
            MatrixConsumerSurface::AiContext,
            MatrixConsumerSurface::ProviderOverlay,
            MatrixConsumerSurface::Cli,
            MatrixConsumerSurface::SupportExport,
            MatrixConsumerSurface::Shell,
        ],
    }
}

fn topology_rows() -> Vec<TopologyClassRow> {
    vec![
        topology_row(
            RepositoryTopologyClass::SparseCheckoutRoot,
            "Sparse checkout / IDE slice: results outside the active slice are omitted, not missing; widening scope is previewed before it applies",
            TopologyHonestyLabel::OutsideCurrentSlice,
            vec![DegradedTopologyState::Omitted],
            TopologyOperationScope::ActiveRootOnly,
            OperationPreviewClass::ScopeWidenPreview,
            OperationRecoveryClass::CheckpointBeforeMutation,
        ),
        topology_row(
            RepositoryTopologyClass::PartialClonePromisorRoot,
            "Promisor partial clone: known objects may be unfetched until materialized; fetching is previewed and never silently completes a claim",
            TopologyHonestyLabel::NotFetched,
            vec![DegradedTopologyState::Unfetched],
            TopologyOperationScope::ActiveRootOnly,
            OperationPreviewClass::FetchOrDeepenPreview,
            OperationRecoveryClass::CheckpointBeforeMutation,
        ),
        topology_row(
            RepositoryTopologyClass::ShallowHistoryRoot,
            "Shallow history: blame and log stop at the shallow boundary; deepening is previewed before history truth widens",
            TopologyHonestyLabel::ShallowBoundary,
            vec![DegradedTopologyState::Unfetched],
            TopologyOperationScope::ActiveRootOnly,
            OperationPreviewClass::FetchOrDeepenPreview,
            OperationRecoveryClass::CheckpointBeforeMutation,
        ),
        topology_row(
            RepositoryTopologyClass::SubmoduleRoot,
            "Submodule pinned by a parent gitlink: the child must be targeted explicitly; uninitialized or dirty children narrow the parent claim",
            TopologyHonestyLabel::SubmoduleNotInitialized,
            vec![
                DegradedTopologyState::Uninitialized,
                DegradedTopologyState::DirtyChild,
            ],
            TopologyOperationScope::ChildRootOnly,
            OperationPreviewClass::MultiRootPreview,
            OperationRecoveryClass::CheckpointBeforeMutation,
        ),
        topology_row(
            RepositoryTopologyClass::NestedIndependentRepoRoot,
            "Nested independent repo: an independent .git boundary owns its objects; cross-root operations are previewed and dirty children block them",
            TopologyHonestyLabel::NestedRepoBoundary,
            vec![DegradedTopologyState::DirtyChild],
            TopologyOperationScope::ChildRootOnly,
            OperationPreviewClass::MultiRootPreview,
            OperationRecoveryClass::CheckpointBeforeMutation,
        ),
        topology_row(
            RepositoryTopologyClass::WorktreeRoot,
            "Linked worktree: the operation target is an explicit worktree; mutating the wrong worktree is prevented and diffs are previewed",
            TopologyHonestyLabel::WrongTargetRoot,
            vec![DegradedTopologyState::DirtyChild],
            TopologyOperationScope::ActiveRootOnly,
            OperationPreviewClass::DiffPreview,
            OperationRecoveryClass::CheckpointBeforeMutation,
        ),
        topology_row(
            RepositoryTopologyClass::LfsHydrationBoundary,
            "Git LFS pointer-backed asset: only pointer metadata is local until hydrated; content edits stay denied and hydration is previewed",
            TopologyHonestyLabel::PointerOnly,
            vec![DegradedTopologyState::PointerOnly],
            TopologyOperationScope::MetadataOnly,
            OperationPreviewClass::FetchOrDeepenPreview,
            OperationRecoveryClass::NoMutationNoRecovery,
        ),
        topology_row(
            RepositoryTopologyClass::GeneratedVendorRoot,
            "Generated / vendor root: content is intentionally outside editable source truth; mutation is denied and the root is read-only",
            TopologyHonestyLabel::GeneratedOrExcluded,
            vec![DegradedTopologyState::Omitted],
            TopologyOperationScope::MutationDenied,
            OperationPreviewClass::NoPreviewReadOnly,
            OperationRecoveryClass::NoMutationNoRecovery,
        ),
    ]
}

fn session_row(
    session: HistorySurgerySession,
    scope_summary: &str,
    preview_class: OperationPreviewClass,
    recovery_class: OperationRecoveryClass,
    mutation_control: TopologyOperationScope,
    recovery_checkpoint_required_before_mutation: bool,
    consumer_surfaces: Vec<MatrixConsumerSurface>,
) -> SessionObjectRow {
    SessionObjectRow {
        session,
        canonical_record_kind: session.canonical_record_kind().to_owned(),
        scope_summary: scope_summary.to_owned(),
        preview_class,
        recovery_class,
        mutation_control,
        recovery_checkpoint_required_before_mutation,
        consumer_surfaces,
    }
}

fn session_rows() -> Vec<SessionObjectRow> {
    vec![
        session_row(
            HistorySurgerySession::ConflictSession,
            "Conflict resolution for merge/rebase/cherry-pick: structured or raw resolution with provenance preserved; a checkpoint precedes any continue",
            OperationPreviewClass::DiffPreview,
            OperationRecoveryClass::CheckpointBeforeMutation,
            TopologyOperationScope::ActiveRootOnly,
            true,
            vec![
                MatrixConsumerSurface::Review,
                MatrixConsumerSurface::AiContext,
                MatrixConsumerSurface::Cli,
                MatrixConsumerSurface::SupportExport,
                MatrixConsumerSurface::Shell,
            ],
        ),
        session_row(
            HistorySurgerySession::SequenceEditSession,
            "Persisted interactive-rebase todo: the full sequence plan is previewed and a checkpoint precedes the run",
            OperationPreviewClass::SequencePlanPreview,
            OperationRecoveryClass::CheckpointBeforeMutation,
            TopologyOperationScope::ActiveRootOnly,
            true,
            vec![
                MatrixConsumerSurface::Review,
                MatrixConsumerSurface::Cli,
                MatrixConsumerSurface::SupportExport,
                MatrixConsumerSurface::Shell,
            ],
        ),
        session_row(
            HistorySurgerySession::RecoveryCheckpoint,
            "Pre-mutation checkpoint: the restore surface itself; restore options stay reviewable and reflog-only fallbacks are disclosed",
            OperationPreviewClass::NoPreviewReadOnly,
            OperationRecoveryClass::NoMutationNoRecovery,
            TopologyOperationScope::MetadataOnly,
            false,
            vec![
                MatrixConsumerSurface::Review,
                MatrixConsumerSurface::Cli,
                MatrixConsumerSurface::SupportExport,
                MatrixConsumerSurface::Shell,
                MatrixConsumerSurface::Diagnostics,
            ],
        ),
        session_row(
            HistorySurgerySession::PublishRefUpdateProposal,
            "Reviewable proposal to move a published ref: before/after positions are previewed and a rollback to the prior position stays available",
            OperationPreviewClass::RefUpdatePreview,
            OperationRecoveryClass::RefUpdateRollback,
            TopologyOperationScope::ActiveRootOnly,
            false,
            vec![
                MatrixConsumerSurface::ProviderOverlay,
                MatrixConsumerSurface::Review,
                MatrixConsumerSurface::Cli,
                MatrixConsumerSurface::SupportExport,
            ],
        ),
        session_row(
            HistorySurgerySession::StashShelfEntry,
            "Captured stash shelf: apply/pop/drop/branch stay distinct actions and the original state is restorable from the shelf",
            OperationPreviewClass::DiffPreview,
            OperationRecoveryClass::StashShelfRestore,
            TopologyOperationScope::ActiveRootOnly,
            false,
            vec![
                MatrixConsumerSurface::Review,
                MatrixConsumerSurface::Cli,
                MatrixConsumerSurface::SupportExport,
                MatrixConsumerSurface::Shell,
            ],
        ),
    ]
}

fn degraded_row(
    state: DegradedTopologyState,
    meaning: &str,
    blocks_mutation_until_resolved: bool,
    visible_before_destructive_op: bool,
) -> DegradedStateRow {
    DegradedStateRow {
        state,
        meaning: meaning.to_owned(),
        narrows_coverage_claim: true,
        blocks_mutation_until_resolved,
        visible_before_destructive_op,
    }
}

fn degraded_rows() -> Vec<DegradedStateRow> {
    vec![
        degraded_row(
            DegradedTopologyState::Omitted,
            "Content is omitted by the active sparse slice or workset scope; a result may exist outside the slice",
            false,
            false,
        ),
        degraded_row(
            DegradedTopologyState::Unfetched,
            "A known promisor or shallow object is not materialized locally; the answer is bounded until it is fetched or deepened",
            false,
            true,
        ),
        degraded_row(
            DegradedTopologyState::Uninitialized,
            "A submodule child checkout is not initialized; child operations are denied until it is initialized",
            true,
            true,
        ),
        degraded_row(
            DegradedTopologyState::PointerOnly,
            "Only Git LFS pointer metadata is local; content edits are denied until the object is hydrated",
            true,
            true,
        ),
        degraded_row(
            DegradedTopologyState::DirtyChild,
            "A submodule or nested child has uncommitted changes; parent operations are gated until the child is clean",
            true,
            true,
        ),
        degraded_row(
            DegradedTopologyState::ReflogOnlyFallback,
            "No checkpoint exists; only a reflog-only recovery fallback is offered and must be disclosed before the operation runs",
            false,
            true,
        ),
        degraded_row(
            DegradedTopologyState::StaleProviderOverlay,
            "A provider overlay is stale relative to local Git truth; local truth wins and the overlay never overwrites it",
            false,
            true,
        ),
    ]
}

fn risky_row(
    operation: RiskyHistoryOperation,
    preview_class: OperationPreviewClass,
    recovery_class: OperationRecoveryClass,
    requires_explicit_target: bool,
) -> RiskyOperationRow {
    RiskyOperationRow {
        operation,
        preview_class,
        recovery_class,
        requires_explicit_target,
        recovery_visible_before_execution: true,
    }
}

fn risky_rows() -> Vec<RiskyOperationRow> {
    vec![
        risky_row(
            RiskyHistoryOperation::Merge,
            OperationPreviewClass::DiffPreview,
            OperationRecoveryClass::CheckpointBeforeMutation,
            true,
        ),
        risky_row(
            RiskyHistoryOperation::Rebase,
            OperationPreviewClass::DestructiveRewritePreview,
            OperationRecoveryClass::CheckpointBeforeMutation,
            true,
        ),
        risky_row(
            RiskyHistoryOperation::InteractiveRebase,
            OperationPreviewClass::SequencePlanPreview,
            OperationRecoveryClass::CheckpointBeforeMutation,
            true,
        ),
        risky_row(
            RiskyHistoryOperation::CherryPick,
            OperationPreviewClass::DiffPreview,
            OperationRecoveryClass::CheckpointBeforeMutation,
            true,
        ),
        risky_row(
            RiskyHistoryOperation::Revert,
            OperationPreviewClass::DiffPreview,
            OperationRecoveryClass::CheckpointBeforeMutation,
            true,
        ),
        risky_row(
            RiskyHistoryOperation::Reset,
            OperationPreviewClass::DestructiveRewritePreview,
            OperationRecoveryClass::CheckpointBeforeMutation,
            true,
        ),
        risky_row(
            RiskyHistoryOperation::StashApply,
            OperationPreviewClass::DiffPreview,
            OperationRecoveryClass::StashShelfRestore,
            true,
        ),
        risky_row(
            RiskyHistoryOperation::StashPop,
            OperationPreviewClass::DiffPreview,
            OperationRecoveryClass::StashShelfRestore,
            true,
        ),
        risky_row(
            RiskyHistoryOperation::StashDrop,
            OperationPreviewClass::DiffPreview,
            OperationRecoveryClass::ReflogOnlyFallbackDisclosed,
            true,
        ),
        risky_row(
            RiskyHistoryOperation::BranchFromStash,
            OperationPreviewClass::DiffPreview,
            OperationRecoveryClass::StashShelfRestore,
            true,
        ),
        risky_row(
            RiskyHistoryOperation::Publish,
            OperationPreviewClass::RefUpdatePreview,
            OperationRecoveryClass::RefUpdateRollback,
            true,
        ),
        risky_row(
            RiskyHistoryOperation::ForcePushWithLease,
            OperationPreviewClass::RefUpdatePreview,
            OperationRecoveryClass::RefUpdateRollback,
            true,
        ),
    ]
}

fn governance_review() -> MatrixGovernanceReview {
    MatrixGovernanceReview {
        topology_truth_not_reduced_to_badges: true,
        matrix_controls_mutation_preview_recovery_export: true,
        provider_overlay_never_overwrites_local_truth: true,
        recovery_visible_before_destructive_ops: true,
        reflog_only_fallback_disclosed: true,
        one_shared_matrix_across_surfaces: true,
        degraded_vocabulary_shared_across_surfaces: true,
        risky_ops_have_preview_and_recovery: true,
        topology_classes_stay_distinct: true,
    }
}

fn consumer_parity() -> MatrixConsumerParity {
    MatrixConsumerParity {
        provider_overlay_expresses_vocabulary: true,
        ai_context_expresses_vocabulary: true,
        search_expresses_vocabulary: true,
        review_expresses_vocabulary: true,
        support_export_expresses_vocabulary: true,
        cli_expresses_vocabulary: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_SCHEMA_REF.to_owned(),
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_DOC_REF.to_owned(),
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_TOPOLOGY_CONTRACT_REF.to_owned(),
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_CONFLICT_SESSION_CONTRACT_REF.to_owned(),
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_SEQUENCE_EDIT_CONTRACT_REF.to_owned(),
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_RECOVERY_CHECKPOINT_CONTRACT_REF.to_owned(),
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_STASH_CONTRACT_REF.to_owned(),
        M5_GIT_TOPOLOGY_HISTORY_MATRIX_REF_UPDATE_CONTRACT_REF.to_owned(),
    ]
}

fn canonical_packet() -> M5GitTopologyHistoryMatrixPacket {
    M5GitTopologyHistoryMatrixPacket::new(M5GitTopologyHistoryMatrixPacketInput {
        packet_id: "m5-git-topology-history-matrix:frozen:0001".to_owned(),
        matrix_label: "M5 Repository-Topology and History-Surgery Matrix".to_owned(),
        topology_rows: topology_rows(),
        session_object_rows: session_rows(),
        degraded_state_rows: degraded_rows(),
        risky_operation_rows: risky_rows(),
        governance_review: governance_review(),
        consumer_parity: consumer_parity(),
        freeze_posture: MatrixFreezePosture {
            frozen: true,
            review_slo_hours: 720,
            last_reviewed_at: "2026-06-17T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-17T00:00:00Z".to_owned(),
    })
}

fn submodule_narrowed_packet() -> M5GitTopologyHistoryMatrixPacket {
    let mut packet = canonical_packet();
    packet.packet_id =
        "m5-git-topology-history-matrix:submodule-uninitialized-narrowed:0001".to_owned();
    for row in &mut packet.topology_rows {
        if row.topology_class == RepositoryTopologyClass::SubmoduleRoot {
            row.scope_summary =
                "Submodule child is uninitialized: child operations are denied until it is initialized; the parent claim is narrowed, not hidden"
                    .to_owned();
            row.degraded_states = vec![
                DegradedTopologyState::Uninitialized,
                DegradedTopologyState::DirtyChild,
            ];
            row.mutation_control = TopologyOperationScope::MutationDenied;
            row.preview_class = OperationPreviewClass::NoPreviewReadOnly;
            row.recovery_class = OperationRecoveryClass::NoRecoveryOperationBlocked;
        }
    }
    packet
}

fn reset_reflog_only_packet() -> M5GitTopologyHistoryMatrixPacket {
    let mut packet = canonical_packet();
    packet.packet_id = "m5-git-topology-history-matrix:reset-reflog-only-recovery:0001".to_owned();
    for row in &mut packet.risky_operation_rows {
        if row.operation == RiskyHistoryOperation::Reset {
            row.recovery_class = OperationRecoveryClass::ReflogOnlyFallbackDisclosed;
        }
    }
    packet
}

fn main() {
    let variant = std::env::args().nth(1).unwrap_or_default();
    let packet = match variant.as_str() {
        "submodule" => submodule_narrowed_packet(),
        "reset" => reset_reflog_only_packet(),
        _ => canonical_packet(),
    };
    let violations = packet.validate();
    assert!(violations.is_empty(), "matrix invalid: {violations:?}");
    println!("{}", packet.export_safe_json());
}
