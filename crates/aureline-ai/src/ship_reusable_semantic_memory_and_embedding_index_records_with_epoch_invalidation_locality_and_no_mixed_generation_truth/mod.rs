//! Reusable semantic-memory and embedding-index records with graph/docs/model
//! epoch invalidation, local-versus-managed locality cues, and no-mixed-generation
//! retrieval truth.
//!
//! Where the frozen recall matrix
//! ([`crate::freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix`])
//! qualifies whole surfaces and the materialized memory-class lane
//! ([`crate::implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth`])
//! materializes the per-scope memory objects, this module materializes the
//! reusable *derived retrieval artifacts* those surfaces read: reusable semantic
//! memory and embedding indexes. Each [`SemanticRecallRecord`] binds one concrete
//! record to its artifact kind, its graph/docs/model epoch lineage, the epochs
//! whose bump invalidates it, its retrieval generation state, its
//! local-versus-managed locality cue, its delete/export posture, the recall
//! surfaces that read it, and a precise degraded label.
//!
//! The packet keeps three truths the spec requires. First, every record declares
//! a graph, docs, and model epoch token plus an embedding generation, and binds
//! at least one epoch whose bump invalidates it, so an embedding index can never
//! silently outlive the graph, docs, or model it was built from. Second,
//! mixed-generation or stale retrieval never masquerades as current truth: a
//! record whose embedding generations are mixed must be labeled
//! [`RetrievalGenerationState::MixedBlocked`] or
//! [`RetrievalGenerationState::Invalidated`], never
//! [`RetrievalGenerationState::Current`]. Third, the four locality states —
//! local-device, workspace-mirrored, managed-hosted, and policy-blocked — stay
//! distinct and export-safe rather than collapsing into one generic
//! semantic-search state, and a recomputing, invalidated, stale, or
//! policy-blocked record degrades to a precise label
//! ([`SemanticRecallRecord::degraded_label`]) rather than one generic
//! "retrieval unavailable" state.
//!
//! [`SemanticRecallRecordsPacket::validate`] rejects any record whose declared
//! state outruns its epoch, generation, locality, or delete/export evidence.
//!
//! Raw prompt bodies, cached result bodies, raw embeddings, raw provider
//! payloads, credentials, exact token counts, and exact cost amounts never cross
//! this boundary.
//!
//! The boundary schema is
//! [`schemas/ai/ship-reusable-semantic-memory-and-embedding-index-records-with-epoch-invalidation-locality-and-no-mixed-generation-truth.schema.json`](../../../../schemas/ai/ship-reusable-semantic-memory-and-embedding-index-records-with-epoch-invalidation-locality-and-no-mixed-generation-truth.schema.json).
//! The contract doc is
//! [`docs/ai/m5/ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth.md`](../../../../docs/ai/m5/ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth/`](../../../../fixtures/ai/m5/ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`SemanticRecallRecordsPacket`].
pub const SEMANTIC_RECALL_RECORDS_RECORD_KIND: &str =
    "ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth";

/// Schema version for reusable semantic-memory and embedding-index records.
pub const SEMANTIC_RECALL_RECORDS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SEMANTIC_RECALL_RECORDS_SCHEMA_REF: &str =
    "schemas/ai/ship-reusable-semantic-memory-and-embedding-index-records-with-epoch-invalidation-locality-and-no-mixed-generation-truth.schema.json";

/// Repo-relative path of the contract doc.
pub const SEMANTIC_RECALL_RECORDS_DOC_REF: &str =
    "docs/ai/m5/ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth.md";

/// Repo-relative path of the frozen recall matrix contract this packet realizes.
pub const SEMANTIC_RECALL_RECORDS_RECALL_MATRIX_CONTRACT_REF: &str =
    "docs/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix.md";

/// Repo-relative path of the frozen memory-class matrix contract.
pub const SEMANTIC_RECALL_RECORDS_MEMORY_CLASS_CONTRACT_REF: &str =
    "docs/ai/memory_class_matrix.md";

/// Repo-relative path of the frozen memory delete/export contract.
pub const SEMANTIC_RECALL_RECORDS_DELETE_EXPORT_CONTRACT_REF: &str =
    "docs/ai/ai-memory-delete-export.md";

/// Repo-relative path of the frozen context-assembly contract (retrieval input).
pub const SEMANTIC_RECALL_RECORDS_CONTEXT_ASSEMBLY_CONTRACT_REF: &str =
    "docs/ai/context_assembly_contract.md";

/// Repo-relative path of the frozen retrieval identity/ranking contract.
pub const SEMANTIC_RECALL_RECORDS_RETRIEVAL_CONTRACT_REF: &str =
    "docs/search/result_identity_and_ranking.md";

/// Repo-relative path of the protected fixture directory.
pub const SEMANTIC_RECALL_RECORDS_FIXTURE_DIR: &str =
    "fixtures/ai/m5/ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth";

/// Repo-relative path of the checked support-export artifact.
pub const SEMANTIC_RECALL_RECORDS_ARTIFACT_REF: &str =
    "artifacts/ai/m5/ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const SEMANTIC_RECALL_RECORDS_SUMMARY_REF: &str =
    "artifacts/ai/m5/ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth.md";

/// Derived retrieval artifact kind a record materializes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticArtifactKind {
    /// Reusable semantic memory derived from prior recall.
    ReusableSemanticMemory,
    /// Embedding index built over a corpus for semantic retrieval.
    EmbeddingIndex,
}

impl SemanticArtifactKind {
    /// Every artifact kind, in declaration order.
    pub const ALL: [Self; 2] = [Self::ReusableSemanticMemory, Self::EmbeddingIndex];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReusableSemanticMemory => "reusable_semantic_memory",
            Self::EmbeddingIndex => "embedding_index",
        }
    }

    /// Whether this kind is an embedding index governed by embedding generation.
    pub const fn is_embedding_index(self) -> bool {
        matches!(self, Self::EmbeddingIndex)
    }
}

/// Epoch dimension a record's freshness is anchored to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpochKind {
    /// Workspace knowledge-graph epoch.
    Graph,
    /// Docs/corpus epoch.
    Docs,
    /// Embedding-model epoch.
    Model,
}

impl EpochKind {
    /// Every epoch kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::Graph, Self::Docs, Self::Model];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Graph => "graph",
            Self::Docs => "docs",
            Self::Model => "model",
        }
    }

    /// The invalidation trigger that fires when this epoch bumps.
    pub const fn bump_trigger(self) -> EpochInvalidationTrigger {
        match self {
            Self::Graph => EpochInvalidationTrigger::GraphEpochBump,
            Self::Docs => EpochInvalidationTrigger::DocsEpochBump,
            Self::Model => EpochInvalidationTrigger::ModelEpochBump,
        }
    }
}

/// Invalidation trigger that drops or recomputes a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpochInvalidationTrigger {
    /// Workspace knowledge-graph epoch bumped.
    GraphEpochBump,
    /// Docs/corpus epoch bumped.
    DocsEpochBump,
    /// Embedding-model epoch bumped.
    ModelEpochBump,
    /// Embedding generation bumped; prior-generation entries are stale.
    EmbeddingGenerationBump,
    /// Policy epoch bumped.
    PolicyEpochBump,
    /// Content-hash key invalidates on input change.
    ContentHashKey,
    /// Manual purge by the user or operator.
    ManualPurge,
}

impl EpochInvalidationTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GraphEpochBump => "graph_epoch_bump",
            Self::DocsEpochBump => "docs_epoch_bump",
            Self::ModelEpochBump => "model_epoch_bump",
            Self::EmbeddingGenerationBump => "embedding_generation_bump",
            Self::PolicyEpochBump => "policy_epoch_bump",
            Self::ContentHashKey => "content_hash_key",
            Self::ManualPurge => "manual_purge",
        }
    }
}

/// Retrieval generation state, keeping stale or mixed generations from
/// masquerading as current truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalGenerationState {
    /// Single, fresh generation; safe to serve as current retrieval truth.
    Current,
    /// Being recomputed after an epoch or generation bump; labeled, not served.
    Recomputing,
    /// Invalidated by an epoch or generation bump; awaiting recompute.
    Invalidated,
    /// Past the freshness window; labeled stale, not served as current truth.
    Stale,
    /// Would mix embedding generations; blocked from masquerading as current.
    MixedBlocked,
}

impl RetrievalGenerationState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Recomputing => "recomputing",
            Self::Invalidated => "invalidated",
            Self::Stale => "stale",
            Self::MixedBlocked => "mixed_blocked",
        }
    }

    /// Whether this state may be served as current retrieval truth.
    pub const fn is_current_truth(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// Local-versus-managed locality cue for the data a record holds.
///
/// The four states stay distinct and export-safe rather than collapsing into one
/// generic semantic-search state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticLocalityClass {
    /// Built and held on the local device only.
    LocalDeviceOnly,
    /// Mirrored within the workspace; no cross-workspace recall.
    WorkspaceMirrored,
    /// Managed-hosted and region-pinned within the tenant.
    ManagedHosted,
    /// Blocked by a region or policy gate; retrieval withheld here.
    PolicyBlocked,
}

impl SemanticLocalityClass {
    /// Every locality class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalDeviceOnly,
        Self::WorkspaceMirrored,
        Self::ManagedHosted,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDeviceOnly => "local_device_only",
            Self::WorkspaceMirrored => "workspace_mirrored",
            Self::ManagedHosted => "managed_hosted",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// Whether this locality is blocked and so requires a precise label.
    pub const fn is_policy_blocked(self) -> bool {
        matches!(self, Self::PolicyBlocked)
    }
}

/// Delete or export posture for a durable derived retrieval record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticDeleteExportPosture {
    /// User-initiated delete or export of their own copy.
    UserScoped,
    /// Workspace-scoped delete or export.
    WorkspaceScoped,
    /// Tenant-scoped delete or export.
    TenantScoped,
    /// Org-scoped delete or export.
    OrgScoped,
    /// Not applicable; nothing durable is retained to delete or export.
    NotApplicable,
}

impl SemanticDeleteExportPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserScoped => "user_scoped",
            Self::WorkspaceScoped => "workspace_scoped",
            Self::TenantScoped => "tenant_scoped",
            Self::OrgScoped => "org_scoped",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this posture names a real, actionable delete/export operation.
    pub const fn is_actionable(self) -> bool {
        matches!(
            self,
            Self::UserScoped | Self::WorkspaceScoped | Self::TenantScoped | Self::OrgScoped
        )
    }
}

/// Recall surface that reads a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecallConsumerSurface {
    /// Composer inline assist and prompt-composer recall.
    ComposerAssist,
    /// Docs and in-app browser recall with provenance.
    DocsBrowserRecall,
    /// Codebase-understanding recall over the workspace graph.
    CodeUnderstanding,
    /// Semantic and hybrid search surface.
    SemanticSearch,
    /// Support / export packet projection.
    SupportExport,
    /// Managed or offline usage and locality reporting.
    ManagedOfflineReport,
}

impl RecallConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ComposerAssist,
        Self::DocsBrowserRecall,
        Self::CodeUnderstanding,
        Self::SemanticSearch,
        Self::SupportExport,
        Self::ManagedOfflineReport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComposerAssist => "composer_assist",
            Self::DocsBrowserRecall => "docs_browser_recall",
            Self::CodeUnderstanding => "code_understanding",
            Self::SemanticSearch => "semantic_search",
            Self::SupportExport => "support_export",
            Self::ManagedOfflineReport => "managed_offline_report",
        }
    }
}

/// Graph/docs/model epoch lineage plus embedding generation a record is built on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochLineage {
    /// Workspace knowledge-graph epoch token the record was built from.
    pub graph_epoch: String,
    /// Docs/corpus epoch token the record was built from.
    pub docs_epoch: String,
    /// Embedding-model epoch token the record was built from.
    pub model_epoch: String,
    /// Embedding generation token; distinguishes mixed generations.
    pub embedding_generation: String,
}

impl EpochLineage {
    /// Whether every epoch and generation token is present.
    pub fn is_complete(&self) -> bool {
        !self.graph_epoch.trim().is_empty()
            && !self.docs_epoch.trim().is_empty()
            && !self.model_epoch.trim().is_empty()
            && !self.embedding_generation.trim().is_empty()
    }
}

/// One reusable semantic-memory or embedding-index record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallRecord {
    /// Stable record id.
    pub record_id: String,
    /// Derived retrieval artifact kind.
    pub artifact_kind: SemanticArtifactKind,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Graph/docs/model epoch lineage plus embedding generation.
    pub epoch_lineage: EpochLineage,
    /// Epochs whose bump invalidates this record.
    pub bound_epochs: Vec<EpochKind>,
    /// Invalidation triggers that drop or recompute this record.
    pub invalidation_triggers: Vec<EpochInvalidationTrigger>,
    /// Retrieval generation state.
    pub generation_state: RetrievalGenerationState,
    /// True when the record spans more than one embedding generation.
    pub mixed_generation_detected: bool,
    /// Local-versus-managed locality cue.
    pub locality: SemanticLocalityClass,
    /// Delete posture for this record.
    pub delete_posture: SemanticDeleteExportPosture,
    /// Export posture for this record.
    pub export_posture: SemanticDeleteExportPosture,
    /// Recall surfaces that read this record.
    pub consumer_surfaces: Vec<RecallConsumerSurface>,
    /// Precise degraded label, required when this record is not current truth or
    /// is policy-blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,
    /// Evidence packet refs backing this record.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this record.
    pub source_contract_refs: Vec<String>,
}

impl SemanticRecallRecord {
    /// Whether every bound epoch has a matching bump trigger.
    pub fn bound_epochs_have_triggers(&self) -> bool {
        self.bound_epochs
            .iter()
            .all(|epoch| self.invalidation_triggers.contains(&epoch.bump_trigger()))
    }

    /// Whether an embedding index binds the model epoch and embedding-generation
    /// bump so it can never silently serve a prior generation.
    pub fn embedding_index_generation_governed(&self) -> bool {
        if !self.artifact_kind.is_embedding_index() {
            return true;
        }
        self.bound_epochs.contains(&EpochKind::Model)
            && self
                .invalidation_triggers
                .contains(&EpochInvalidationTrigger::EmbeddingGenerationBump)
    }

    /// Whether mixed generations never masquerade as current retrieval truth.
    ///
    /// A record whose embedding generations are mixed must be
    /// [`RetrievalGenerationState::MixedBlocked`] or
    /// [`RetrievalGenerationState::Invalidated`], never
    /// [`RetrievalGenerationState::Current`].
    pub fn no_mixed_generation_truth(&self) -> bool {
        if !self.mixed_generation_detected {
            return true;
        }
        matches!(
            self.generation_state,
            RetrievalGenerationState::MixedBlocked | RetrievalGenerationState::Invalidated
        )
    }

    /// Whether this record declares an actionable delete and export posture.
    ///
    /// Both artifact kinds are durable derived artifacts, so each must name an
    /// actionable delete and export posture.
    pub fn delete_export_actionable(&self) -> bool {
        self.delete_posture.is_actionable() && self.export_posture.is_actionable()
    }

    /// Whether a record that needs a precise degraded label carries one.
    ///
    /// A record needs a precise label when it is not current retrieval truth or
    /// its locality is policy-blocked.
    pub fn needs_degraded_label(&self) -> bool {
        !self.generation_state.is_current_truth() || self.locality.is_policy_blocked()
    }

    /// Whether the degraded-label requirement is satisfied.
    pub fn degraded_label_consistent(&self) -> bool {
        if !self.needs_degraded_label() {
            return true;
        }
        match &self.degraded_label {
            Some(label) => !label_is_generic(label),
            None => false,
        }
    }

    /// Whether every dimension required to materialize this record is present.
    pub fn is_complete(&self) -> bool {
        !self.record_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.epoch_lineage.is_complete()
            && !self.bound_epochs.is_empty()
            && !self.invalidation_triggers.is_empty()
            && self.bound_epochs_have_triggers()
            && self.embedding_index_generation_governed()
            && self.no_mixed_generation_truth()
            && self.delete_export_actionable()
            && self.degraded_label_consistent()
            && !self.consumer_surfaces.is_empty()
            && !self.evidence_refs.is_empty()
            && !self.source_contract_refs.is_empty()
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecordGuardrails {
    /// No cross-workspace recall happens by default.
    pub no_cross_workspace_recall_by_default: bool,
    /// No cross-tenant recall happens by default.
    pub no_cross_tenant_recall_by_default: bool,
    /// Mixed-generation embeddings never masquerade as current truth.
    pub mixed_generation_never_current_truth: bool,
    /// Invalidated or recomputing lanes are always labeled explicitly.
    pub invalidated_or_recomputing_lanes_labeled: bool,
    /// Every durable record declares its delete and export posture.
    pub every_durable_record_declares_delete_export: bool,
    /// Local, mirrored, managed, and policy-blocked locality stay distinct.
    pub locality_states_remain_distinct: bool,
    /// A graph, docs, or model epoch bump invalidates the records bound to it.
    pub epoch_bump_invalidates_bound_records: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecordConsumerProjection {
    /// Composer shows which memory and generation a record carries.
    pub composer_shows_memory_and_generation: bool,
    /// Docs/browser shows provenance and locality.
    pub docs_browser_shows_provenance_and_locality: bool,
    /// Search shows the retrieval generation state.
    pub search_shows_retrieval_generation: bool,
    /// Support export shows locality posture and epoch lineage.
    pub support_export_shows_locality_and_epochs: bool,
    /// Managed/offline reporting shows locality truth.
    pub managed_offline_shows_locality_truth: bool,
    /// Invalidated or recomputing lanes are visibly labeled below current.
    pub invalidated_or_recomputing_lanes_labeled_below_current: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecordProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the record.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`SemanticRecallRecordsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRecallRecordsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub records_label: String,
    /// Materialized records.
    pub records: Vec<SemanticRecallRecord>,
    /// Guardrail invariants block.
    pub guardrails: SemanticRecordGuardrails,
    /// Consumer projection block.
    pub consumer_projection: SemanticRecordConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: SemanticRecordProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe reusable semantic-memory and embedding-index records packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRecallRecordsPacket {
    /// Record kind; must equal [`SEMANTIC_RECALL_RECORDS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SEMANTIC_RECALL_RECORDS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub records_label: String,
    /// Materialized records.
    pub records: Vec<SemanticRecallRecord>,
    /// Guardrail invariants block.
    pub guardrails: SemanticRecordGuardrails,
    /// Consumer projection block.
    pub consumer_projection: SemanticRecordConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: SemanticRecordProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl SemanticRecallRecordsPacket {
    /// Builds a reusable semantic-memory and embedding-index records packet.
    pub fn new(input: SemanticRecallRecordsPacketInput) -> Self {
        Self {
            record_kind: SEMANTIC_RECALL_RECORDS_RECORD_KIND.to_owned(),
            schema_version: SEMANTIC_RECALL_RECORDS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            records_label: input.records_label,
            records: input.records,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Artifact kinds materialized by this packet.
    pub fn materialized_kinds(&self) -> BTreeSet<SemanticArtifactKind> {
        self.records.iter().map(|rec| rec.artifact_kind).collect()
    }

    /// Epoch kinds bound by some record in this packet.
    pub fn bound_epoch_kinds(&self) -> BTreeSet<EpochKind> {
        self.records
            .iter()
            .flat_map(|rec| rec.bound_epochs.iter().copied())
            .collect()
    }

    /// Locality classes represented in this packet.
    pub fn represented_localities(&self) -> BTreeSet<SemanticLocalityClass> {
        self.records.iter().map(|rec| rec.locality).collect()
    }

    /// Validates the reusable semantic-memory and embedding-index invariants.
    pub fn validate(&self) -> Vec<SemanticRecallRecordsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != SEMANTIC_RECALL_RECORDS_RECORD_KIND {
            violations.push(SemanticRecallRecordsViolation::WrongRecordKind);
        }
        if self.schema_version != SEMANTIC_RECALL_RECORDS_SCHEMA_VERSION {
            violations.push(SemanticRecallRecordsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.records_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(SemanticRecallRecordsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_records(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("semantic recall records packet serializes"),
        ) {
            violations.push(SemanticRecallRecordsViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("semantic recall records packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let current = self
            .records
            .iter()
            .filter(|rec| rec.generation_state.is_current_truth())
            .count();
        let mut out = String::new();
        out.push_str("# Reusable Semantic-Memory and Embedding-Index Records\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.records_label));
        out.push_str(&format!(
            "- Records: {} ({} current)\n",
            self.records.len(),
            current
        ));
        out.push_str(&format!(
            "- Kinds: {} / Localities: {}\n",
            self.materialized_kinds().len(),
            self.represented_localities().len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Records\n\n");
        for rec in &self.records {
            out.push_str(&format!(
                "- **{}** ({} / {}): `{}`\n",
                rec.record_id,
                rec.artifact_kind.as_str(),
                rec.locality.as_str(),
                rec.generation_state.as_str()
            ));
            out.push_str(&format!("  - {}\n", rec.label_summary));
            out.push_str(&format!(
                "  - Epochs: graph `{}`, docs `{}`, model `{}`, generation `{}`\n",
                rec.epoch_lineage.graph_epoch,
                rec.epoch_lineage.docs_epoch,
                rec.epoch_lineage.model_epoch,
                rec.epoch_lineage.embedding_generation
            ));
            out.push_str(&format!(
                "  - Delete/export: `{}` / `{}`\n",
                rec.delete_posture.as_str(),
                rec.export_posture.as_str()
            ));
            if let Some(label) = &rec.degraded_label {
                out.push_str(&format!("  - Degraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in semantic recall records export.
#[derive(Debug)]
pub enum SemanticRecallRecordsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<SemanticRecallRecordsViolation>),
}

impl fmt::Display for SemanticRecallRecordsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "semantic recall records export parse failed: {error}"
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
                    "semantic recall records export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SemanticRecallRecordsArtifactError {}

/// Validation failures emitted by [`SemanticRecallRecordsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticRecallRecordsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required artifact kind is missing from the packet.
    RequiredArtifactKindMissing,
    /// A required epoch kind is bound by no record.
    RequiredEpochCoverageMissing,
    /// A required locality class is represented by no record.
    RequiredLocalityCoverageMissing,
    /// No record demonstrates a non-current, explicitly labeled lane.
    InvalidatedLaneCaseMissing,
    /// A record is incomplete.
    RecordIncomplete,
    /// A record's epoch lineage is incomplete.
    EpochLineageIncomplete,
    /// A bound epoch has no matching invalidation trigger.
    BoundEpochTriggerMissing,
    /// An embedding index does not bind model epoch and embedding generation.
    EmbeddingIndexGenerationUngoverned,
    /// A mixed-generation record masquerades as current retrieval truth.
    MixedGenerationMasqueradesAsCurrent,
    /// A durable record does not declare an actionable delete/export posture.
    DurableRecordMissingDeleteExport,
    /// A record needing a precise degraded label lacks one.
    DegradedLabelMissing,
    /// A record has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl SemanticRecallRecordsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredArtifactKindMissing => "required_artifact_kind_missing",
            Self::RequiredEpochCoverageMissing => "required_epoch_coverage_missing",
            Self::RequiredLocalityCoverageMissing => "required_locality_coverage_missing",
            Self::InvalidatedLaneCaseMissing => "invalidated_lane_case_missing",
            Self::RecordIncomplete => "record_incomplete",
            Self::EpochLineageIncomplete => "epoch_lineage_incomplete",
            Self::BoundEpochTriggerMissing => "bound_epoch_trigger_missing",
            Self::EmbeddingIndexGenerationUngoverned => "embedding_index_generation_ungoverned",
            Self::MixedGenerationMasqueradesAsCurrent => "mixed_generation_masquerades_as_current",
            Self::DurableRecordMissingDeleteExport => "durable_record_missing_delete_export",
            Self::DegradedLabelMissing => "degraded_label_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable semantic recall records export.
pub fn current_stable_semantic_recall_records_export(
) -> Result<SemanticRecallRecordsPacket, SemanticRecallRecordsArtifactError> {
    let packet: SemanticRecallRecordsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth/support_export.json"
    )))
    .map_err(SemanticRecallRecordsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(SemanticRecallRecordsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &SemanticRecallRecordsPacket,
    violations: &mut Vec<SemanticRecallRecordsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        SEMANTIC_RECALL_RECORDS_SCHEMA_REF,
        SEMANTIC_RECALL_RECORDS_DOC_REF,
        SEMANTIC_RECALL_RECORDS_RECALL_MATRIX_CONTRACT_REF,
        SEMANTIC_RECALL_RECORDS_MEMORY_CLASS_CONTRACT_REF,
        SEMANTIC_RECALL_RECORDS_DELETE_EXPORT_CONTRACT_REF,
        SEMANTIC_RECALL_RECORDS_CONTEXT_ASSEMBLY_CONTRACT_REF,
        SEMANTIC_RECALL_RECORDS_RETRIEVAL_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(SemanticRecallRecordsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_coverage(
    packet: &SemanticRecallRecordsPacket,
    violations: &mut Vec<SemanticRecallRecordsViolation>,
) {
    let kinds = packet.materialized_kinds();
    for required in SemanticArtifactKind::ALL {
        if !kinds.contains(&required) {
            violations.push(SemanticRecallRecordsViolation::RequiredArtifactKindMissing);
            break;
        }
    }
    let epochs = packet.bound_epoch_kinds();
    for required in EpochKind::ALL {
        if !epochs.contains(&required) {
            violations.push(SemanticRecallRecordsViolation::RequiredEpochCoverageMissing);
            break;
        }
    }
    let localities = packet.represented_localities();
    for required in SemanticLocalityClass::ALL {
        if !localities.contains(&required) {
            violations.push(SemanticRecallRecordsViolation::RequiredLocalityCoverageMissing);
            break;
        }
    }
    let has_non_current_labeled = packet
        .records
        .iter()
        .any(|rec| !rec.generation_state.is_current_truth() && rec.degraded_label_consistent());
    if !has_non_current_labeled {
        violations.push(SemanticRecallRecordsViolation::InvalidatedLaneCaseMissing);
    }
}

fn validate_records(
    packet: &SemanticRecallRecordsPacket,
    violations: &mut Vec<SemanticRecallRecordsViolation>,
) {
    for rec in &packet.records {
        if !rec.is_complete() {
            violations.push(SemanticRecallRecordsViolation::RecordIncomplete);
        }
        if !rec.epoch_lineage.is_complete() {
            violations.push(SemanticRecallRecordsViolation::EpochLineageIncomplete);
        }
        if rec.bound_epochs.is_empty() || !rec.bound_epochs_have_triggers() {
            violations.push(SemanticRecallRecordsViolation::BoundEpochTriggerMissing);
        }
        if !rec.embedding_index_generation_governed() {
            violations.push(SemanticRecallRecordsViolation::EmbeddingIndexGenerationUngoverned);
        }
        if !rec.no_mixed_generation_truth() {
            violations.push(SemanticRecallRecordsViolation::MixedGenerationMasqueradesAsCurrent);
        }
        if !rec.delete_export_actionable() {
            violations.push(SemanticRecallRecordsViolation::DurableRecordMissingDeleteExport);
        }
        if !rec.degraded_label_consistent() {
            violations.push(SemanticRecallRecordsViolation::DegradedLabelMissing);
        }
        if rec.consumer_surfaces.is_empty() {
            violations.push(SemanticRecallRecordsViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_guardrails(
    packet: &SemanticRecallRecordsPacket,
    violations: &mut Vec<SemanticRecallRecordsViolation>,
) {
    let guardrails = &packet.guardrails;
    let ok = guardrails.no_cross_workspace_recall_by_default
        && guardrails.no_cross_tenant_recall_by_default
        && guardrails.mixed_generation_never_current_truth
        && guardrails.invalidated_or_recomputing_lanes_labeled
        && guardrails.every_durable_record_declares_delete_export
        && guardrails.locality_states_remain_distinct
        && guardrails.epoch_bump_invalidates_bound_records;
    if !ok {
        violations.push(SemanticRecallRecordsViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &SemanticRecallRecordsPacket,
    violations: &mut Vec<SemanticRecallRecordsViolation>,
) {
    let projection = &packet.consumer_projection;
    let ok = projection.composer_shows_memory_and_generation
        && projection.docs_browser_shows_provenance_and_locality
        && projection.search_shows_retrieval_generation
        && projection.support_export_shows_locality_and_epochs
        && projection.managed_offline_shows_locality_truth
        && projection.invalidated_or_recomputing_lanes_labeled_below_current;
    if !ok {
        violations.push(SemanticRecallRecordsViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_proof_freshness(
    packet: &SemanticRecallRecordsPacket,
    violations: &mut Vec<SemanticRecallRecordsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(SemanticRecallRecordsViolation::ProofFreshnessIncomplete);
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "retrieval unavailable"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "retrieval error"
            | "failed"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
