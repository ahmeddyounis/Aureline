//! Stable dense-collection contracts shared by search, review, admin, package,
//! test, diagnostics, and notebook data-grid surfaces.
//!
//! This crate lifts the existing collection filter AST, saved-view, scope
//! counter, selection, and batch-review primitives into a governed M4 packet so
//! stable surfaces do not fork filter grammar, count wording, export semantics,
//! or bulk-action scope rules.

#![doc(html_root_url = "https://docs.rs/aureline-collections/0.0.0")]

pub mod stabilize_filter_ast_saved_view_scope_pack_column_preset;
pub mod stabilize_selection_scope_and_batch_result_truth;

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
