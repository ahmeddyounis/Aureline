//! Stable dense-collection contracts shared by search, review, admin, package,
//! test, diagnostics, and notebook data-grid surfaces.
//!
//! This crate lifts the existing collection filter AST, saved-view, scope
//! counter, selection, and batch-review primitives into a governed M4 packet so
//! stable surfaces do not fork filter grammar, count wording, export semantics,
//! or bulk-action scope rules.

#![doc(html_root_url = "https://docs.rs/aureline-collections/0.0.0")]

pub mod freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix;
pub mod implement_filter_asts_saved_views_column_presets_and_privacy_scoped_persistence;
pub mod implement_selection_bars_range_anchor_and_stale_snapshot_guards;
pub mod ship_result_scope_counters_and_hidden_narrowing_chips;
pub mod stabilize_filter_ast_saved_view_scope_pack_column_preset;
pub mod stabilize_selection_scope_and_batch_result_truth;

pub use freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix::{
    current_m5_collection_qualification_matrix_export, BatchActionDescriptor, BatchActionKind,
    BatchActionScopeClass, CollectionMatrixDowngradeTrigger, CollectionMatrixQualificationClass,
    CollectionQualificationMatrixArtifactError, CollectionQualificationMatrixPacket,
    CollectionQualificationMatrixPacketInput, CollectionQualificationMatrixViolation,
    CollectionQualificationRow, ColumnPresetDeclaration, DenseCollectionSurface, FilterAstClass,
    MatrixConsumerProjection as CollectionMatrixConsumerProjection,
    MatrixEvidenceFreshness as CollectionMatrixEvidenceFreshness,
    MatrixGuardrails as CollectionMatrixGuardrails, ResultCounterClass, SavedViewDeclaration,
    M5_COLLECTION_QUALIFICATION_MATRIX_ARTIFACT_REF, M5_COLLECTION_QUALIFICATION_MATRIX_DOC_REF,
    M5_COLLECTION_QUALIFICATION_MATRIX_FIXTURE_DIR, M5_COLLECTION_QUALIFICATION_MATRIX_RECORD_KIND,
    M5_COLLECTION_QUALIFICATION_MATRIX_SCHEMA_REF,
    M5_COLLECTION_QUALIFICATION_MATRIX_SCHEMA_VERSION,
    M5_COLLECTION_QUALIFICATION_MATRIX_SUMMARY_REF,
};

pub use implement_filter_asts_saved_views_column_presets_and_privacy_scoped_persistence::{
    current_m5_collection_persistence_export, CollectionStateReconstruction,
    IncompatibilityResolution, M5CollectionPersistenceArtifactError, M5CollectionPersistencePacket,
    M5CollectionPersistencePacketInput, M5CollectionPersistenceViolation, PersistedCollectionState,
    PersistedColumnPreset, PersistenceCompatibility, PersistenceConsumerProjection,
    PersistenceGuardrails, ReopenOutcome, M5_COLLECTION_PERSISTENCE_ARTIFACT_REF,
    M5_COLLECTION_PERSISTENCE_DOC_REF, M5_COLLECTION_PERSISTENCE_FIXTURE_DIR,
    M5_COLLECTION_PERSISTENCE_RECORD_KIND, M5_COLLECTION_PERSISTENCE_SCHEMA_REF,
    M5_COLLECTION_PERSISTENCE_SCHEMA_VERSION, M5_COLLECTION_PERSISTENCE_SUMMARY_REF,
    M5_PERSISTED_STATE_SCHEMA_VERSION,
};

pub use implement_selection_bars_range_anchor_and_stale_snapshot_guards::{
    current_m5_selection_bar_continuity_export, CollectionDataMode, DatasetIdentityChange,
    RangeAnchor, SelectionBar, SelectionBarConsumerProjection, SelectionBarContinuityArtifactError,
    SelectionBarContinuityPacket, SelectionBarContinuityPacketInput,
    SelectionBarContinuityViolation, SelectionBarCounts, SelectionBarGuardrails,
    SelectionBarReconstruction, SelectionMembership, SelectionMembershipBasis, StableSelectionItem,
    StaleGuardOutcome, StaleQuerySnapshotGuard, SELECTION_BAR_CONTINUITY_ARTIFACT_REF,
    SELECTION_BAR_CONTINUITY_DOC_REF, SELECTION_BAR_CONTINUITY_FIXTURE_DIR,
    SELECTION_BAR_CONTINUITY_RECORD_KIND, SELECTION_BAR_CONTINUITY_SCHEMA_REF,
    SELECTION_BAR_CONTINUITY_SCHEMA_VERSION, SELECTION_BAR_CONTINUITY_SUMMARY_REF,
};

pub use ship_result_scope_counters_and_hidden_narrowing_chips::{
    current_m5_result_scope_counter_export, CollectionViewKind, CountExactness, CountFreshness,
    CounterPlacement, HiddenNarrowingChip, NarrowingCause, ResultCountKind,
    ResultScopeConsumerProjection, ResultScopeCount, ResultScopeCounterArtifactError,
    ResultScopeCounterBinding, ResultScopeCounterPacket, ResultScopeCounterPacketInput,
    ResultScopeCounterViolation, ResultScopeGuardrails, ResultScopePosture,
    ResultScopeReconstruction, RESULT_SCOPE_COUNTER_ARTIFACT_REF, RESULT_SCOPE_COUNTER_DOC_REF,
    RESULT_SCOPE_COUNTER_FIXTURE_DIR, RESULT_SCOPE_COUNTER_RECORD_KIND,
    RESULT_SCOPE_COUNTER_SCHEMA_REF, RESULT_SCOPE_COUNTER_SCHEMA_VERSION,
    RESULT_SCOPE_COUNTER_SUMMARY_REF,
};
pub use stabilize_filter_ast_saved_view_scope_pack_column_preset::{
    current_stable_dense_collection_contract_packet, seeded_dense_collection_contract_packet,
    BatchProtectionPosture, CollectionContractArtifactError, CollectionContractColumnPreset,
    CollectionContractConsumerProjection, CollectionContractConsumerSurface,
    CollectionContractFindingKind, CollectionContractFindingSeverity, CollectionContractPacket,
    CollectionContractPromotionState, CollectionContractQueryHistory, CollectionContractRow,
    CollectionContractScopePack, CollectionContractSupportExport, DenseCollectionSurfaceFamily,
    DenseCollectionSurfaceOwnership, ReopenScopePosture, ScopeCounterVocabularyTerm,
    SelectAllMeaning, StableDenseCollectionObjectKind, COLLECTION_CONTRACT_ARTIFACT_DOC_REF,
    COLLECTION_CONTRACT_DOC_REF, COLLECTION_CONTRACT_FIXTURE_DIR,
    COLLECTION_CONTRACT_PACKET_ARTIFACT_REF, COLLECTION_CONTRACT_PACKET_RECORD_KIND,
    COLLECTION_CONTRACT_SCHEMA_VERSION, COLLECTION_CONTRACT_SUPPORT_EXPORT_RECORD_KIND,
};
pub use stabilize_selection_scope_and_batch_result_truth::{
    current_selection_scope_packet, seeded_selection_scope_packet, BatchItemOutcome,
    BatchItemResult, BatchMemberDisposition as SelectionScopeBatchMemberDisposition,
    BatchPrivacyClass, BatchReviewMember as SelectionScopeBatchReviewMember, BatchReviewTruth,
    CountTruth, DatasetPosture, ExecutionOriginClass, RangeTraversalContract, SelectionBasisRefs,
    SelectionScopeArtifactError, SelectionScopeClass, SelectionScopeConsumerProjection,
    SelectionScopeFindingKind, SelectionScopeFindingSeverity, SelectionScopeObject,
    SelectionScopePacket, SelectionScopeProjectionProof, SelectionScopePromotionState,
    SelectionScopeSupportExport, SelectionScopeSurfaceFamily, SelectionScopeValidationFinding,
    StableSelectionItemRef, SELECTION_SCOPE_ARTIFACT_DOC_REF, SELECTION_SCOPE_DOC_REF,
    SELECTION_SCOPE_FIXTURE_DIR, SELECTION_SCOPE_PACKET_ARTIFACT_REF,
    SELECTION_SCOPE_PACKET_RECORD_KIND, SELECTION_SCOPE_SCHEMA_REF, SELECTION_SCOPE_SCHEMA_VERSION,
    SELECTION_SCOPE_SUPPORT_EXPORT_RECORD_KIND,
};
