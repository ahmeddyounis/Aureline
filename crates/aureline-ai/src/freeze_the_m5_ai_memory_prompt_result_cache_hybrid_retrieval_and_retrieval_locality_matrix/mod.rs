//! Frozen M5 AI memory, prompt-result-cache, hybrid-retrieval, and
//! retrieval-locality matrix.
//!
//! This module locks the canonical M5 recall qualification for every claimed AI
//! or recall surface — composer assist, docs/browser recall, code understanding,
//! semantic search, support/export, and managed/offline reporting — into one
//! export-safe packet. Each [`M5AiRecallMatrixSurfaceRow`] binds a surface to its
//! memory/cache classes, retrieval lanes, locality posture, cache-invalidation
//! classes, delete/export posture, budget/receipt expectation, downgrade
//! triggers, required evidence packet refs, source contracts, and consumer
//! surface parity.
//!
//! The matrix is the single source of truth for whether a recall surface may
//! ship as Stable, Beta, Preview, or must narrow further. It references upstream
//! memory, retrieval, and spend contracts by id rather than embedding their
//! content. Raw prompt bodies, cached result bodies, raw embeddings, raw provider
//! payloads, credentials, exact token counts, and exact cost amounts stay outside
//! the support boundary.
//!
//! A surface that is missing a locality posture, a cache-invalidation class, or a
//! delete/export posture for its durable memory cannot retain a Stable claim:
//! [`M5AiRecallMatrixSurfaceRow::effective_qualification`] narrows it to Preview,
//! and [`M5AiRecallMatrixPacket::validate`] rejects any packet whose declared
//! Stable claim outruns its recall evidence.
//!
//! The boundary schema is
//! [`schemas/ai/freeze-the-m5-ai-memory-prompt-result-cache-hybrid-retrieval-and-retrieval-locality-matrix.schema.json`](../../../../schemas/ai/freeze-the-m5-ai-memory-prompt-result-cache-hybrid-retrieval-and-retrieval-locality-matrix.schema.json).
//! The contract doc is
//! [`docs/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix.md`](../../../../docs/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix/`](../../../../fixtures/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5AiRecallMatrixPacket`].
pub const M5_AI_RECALL_MATRIX_RECORD_KIND: &str =
    "freeze_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix";

/// Schema version for M5 AI recall matrix records.
pub const M5_AI_RECALL_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_AI_RECALL_MATRIX_SCHEMA_REF: &str =
    "schemas/ai/freeze-the-m5-ai-memory-prompt-result-cache-hybrid-retrieval-and-retrieval-locality-matrix.schema.json";

/// Repo-relative path of the M5 AI recall matrix contract doc.
pub const M5_AI_RECALL_MATRIX_DOC_REF: &str =
    "docs/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix.md";

/// Repo-relative path of the frozen memory-class matrix contract.
pub const M5_AI_RECALL_MATRIX_MEMORY_CLASS_CONTRACT_REF: &str = "docs/ai/memory_class_matrix.md";

/// Repo-relative path of the frozen memory delete/export contract.
pub const M5_AI_RECALL_MATRIX_DELETE_EXPORT_CONTRACT_REF: &str =
    "docs/ai/ai-memory-delete-export.md";

/// Repo-relative path of the frozen spend/route receipt contract.
pub const M5_AI_RECALL_MATRIX_SPEND_RECEIPT_CONTRACT_REF: &str =
    "docs/ai/spend_and_route_receipt_contract.md";

/// Repo-relative path of the frozen context-assembly contract (retrieval input).
pub const M5_AI_RECALL_MATRIX_CONTEXT_ASSEMBLY_CONTRACT_REF: &str =
    "docs/ai/context_assembly_contract.md";

/// Repo-relative path of the frozen retrieval identity/ranking contract.
pub const M5_AI_RECALL_MATRIX_RETRIEVAL_CONTRACT_REF: &str =
    "docs/search/result_identity_and_ranking.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_AI_RECALL_MATRIX_FIXTURE_DIR: &str =
    "fixtures/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AI_RECALL_MATRIX_ARTIFACT_REF: &str =
    "artifacts/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_AI_RECALL_MATRIX_SUMMARY_REF: &str =
    "artifacts/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix.md";

/// One claimed M5 AI or recall surface governed by this matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RecallSurface {
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
    ManagedOffline,
}

impl M5RecallSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ComposerAssist,
        Self::DocsBrowserRecall,
        Self::CodeUnderstanding,
        Self::SemanticSearch,
        Self::SupportExport,
        Self::ManagedOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComposerAssist => "composer_assist",
            Self::DocsBrowserRecall => "docs_browser_recall",
            Self::CodeUnderstanding => "code_understanding",
            Self::SemanticSearch => "semantic_search",
            Self::SupportExport => "support_export",
            Self::ManagedOffline => "managed_offline",
        }
    }
}

/// Qualification class for an M5 recall surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RecallQualificationClass {
    /// Surface qualifies for the Stable claim.
    Stable,
    /// Surface is narrowed to Beta.
    Beta,
    /// Surface is narrowed to Preview.
    Preview,
    /// Surface is experimental and not claimed.
    Experimental,
    /// Surface is unavailable on this build.
    Unavailable,
    /// Surface is held pending upstream resolution.
    Held,
}

impl M5RecallQualificationClass {
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

    /// Whether the surface may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Memory or cache class held or consumed by a recall surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MemoryCacheClass {
    /// Reusable prompt-result cache keyed by content hash.
    PromptResultCache,
    /// Durable reusable semantic memory.
    ReusableSemanticMemory,
    /// Durable embedding index for retrieval.
    EmbeddingIndex,
    /// Ephemeral, session-scoped working state.
    EphemeralSessionState,
    /// No durable memory; surface is stateless across sessions.
    NoDurableMemory,
}

impl M5MemoryCacheClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PromptResultCache => "prompt_result_cache",
            Self::ReusableSemanticMemory => "reusable_semantic_memory",
            Self::EmbeddingIndex => "embedding_index",
            Self::EphemeralSessionState => "ephemeral_session_state",
            Self::NoDurableMemory => "no_durable_memory",
        }
    }

    /// Whether this class persists beyond a single session and so must declare a
    /// retention, delete, and export posture.
    pub const fn is_durable(self) -> bool {
        matches!(
            self,
            Self::PromptResultCache | Self::ReusableSemanticMemory | Self::EmbeddingIndex
        )
    }
}

/// Retrieval lane contributing to a recall surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetrievalLane {
    /// Lexical / keyword retrieval.
    LexicalKeyword,
    /// Semantic embedding retrieval.
    SemanticEmbedding,
    /// Hybrid fusion of lexical and semantic lanes.
    HybridFusion,
    /// Graph traversal over the workspace knowledge graph.
    GraphTraversal,
    /// No retrieval; surface does not recall external context.
    NoRetrieval,
}

impl M5RetrievalLane {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LexicalKeyword => "lexical_keyword",
            Self::SemanticEmbedding => "semantic_embedding",
            Self::HybridFusion => "hybrid_fusion",
            Self::GraphTraversal => "graph_traversal",
            Self::NoRetrieval => "no_retrieval",
        }
    }
}

/// Locality posture for the data a recall surface holds or reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalityPosture {
    /// Data stays on the local device only.
    LocalDeviceOnly,
    /// Data is scoped to the workspace, no cross-workspace recall.
    WorkspaceLocal,
    /// Data is tenant-scoped and region-pinned, no cross-tenant recall.
    TenantRegionPinned,
    /// Data is managed-hosted and region-pinned.
    ManagedHostedRegionPinned,
}

impl M5LocalityPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDeviceOnly => "local_device_only",
            Self::WorkspaceLocal => "workspace_local",
            Self::TenantRegionPinned => "tenant_region_pinned",
            Self::ManagedHostedRegionPinned => "managed_hosted_region_pinned",
        }
    }
}

/// Delete and export posture for durable recall artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeleteExportPosture {
    /// User-initiated delete and export are supported.
    UserDeletableExportable,
    /// Workspace-scoped delete and export are supported.
    WorkspaceDeletableExportable,
    /// Tenant-scoped delete and export are supported.
    TenantDeletableExportable,
    /// Artifact expires automatically and is never durably retained.
    EphemeralAutoExpire,
    /// Not applicable; surface holds no durable memory.
    NotApplicable,
}

impl M5DeleteExportPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserDeletableExportable => "user_deletable_exportable",
            Self::WorkspaceDeletableExportable => "workspace_deletable_exportable",
            Self::TenantDeletableExportable => "tenant_deletable_exportable",
            Self::EphemeralAutoExpire => "ephemeral_auto_expire",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Budget and receipt expectation for a recall surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BudgetReceiptExpectation {
    /// Both route and spend receipts are required per call.
    RouteAndSpendReceiptRequired,
    /// A spend receipt is required per call.
    SpendReceiptRequired,
    /// Local model path; no spend, no remote route receipt.
    LocalNoSpendReceipt,
    /// Budget is capped with a named fallback chain.
    BudgetCappedWithFallback,
}

impl M5BudgetReceiptExpectation {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RouteAndSpendReceiptRequired => "route_and_spend_receipt_required",
            Self::SpendReceiptRequired => "spend_receipt_required",
            Self::LocalNoSpendReceipt => "local_no_spend_receipt",
            Self::BudgetCappedWithFallback => "budget_capped_with_fallback",
        }
    }
}

/// Cache or memory invalidation class that keeps recall from masquerading as
/// current truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CacheInvalidationClass {
    /// Time-to-live expiry.
    TtlExpiry,
    /// Content-hash keying invalidates on input change.
    ContentHashKey,
    /// Policy-epoch bump invalidates prior entries.
    PolicyEpochBump,
    /// Trust narrowing invalidates wider-trust entries.
    TrustNarrowing,
    /// Embedding-generation bump invalidates mixed-generation entries.
    EmbeddingGenerationBump,
    /// Manual purge by the user or operator.
    ManualPurge,
}

impl M5CacheInvalidationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TtlExpiry => "ttl_expiry",
            Self::ContentHashKey => "content_hash_key",
            Self::PolicyEpochBump => "policy_epoch_bump",
            Self::TrustNarrowing => "trust_narrowing",
            Self::EmbeddingGenerationBump => "embedding_generation_bump",
            Self::ManualPurge => "manual_purge",
        }
    }
}

/// Evidence requirement level for a recall surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RecallEvidenceRequirement {
    /// At least one evidence packet is required.
    Required,
    /// Evidence is recommended but not blocking.
    Recommended,
    /// Evidence is optional.
    Optional,
    /// Not applicable for this surface's current qualification.
    NotApplicable,
}

impl M5RecallEvidenceRequirement {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Downgrade trigger that can narrow a recall surface below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RecallDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Required locality posture is unavailable on this build.
    LocalityUnavailable,
    /// Embedding generations are mixed without labeling.
    EmbeddingGenerationMismatch,
    /// Hybrid retrieval is stale and would misrepresent current truth.
    StaleHybridRetrieval,
    /// Budget is exhausted for the session or family.
    BudgetExhausted,
    /// Delete or export behavior could not be verified.
    DeleteExportUnverified,
    /// Required provider or model is unavailable.
    ProviderUnavailable,
    /// An upstream dependency surface narrowed.
    UpstreamDependencyNarrowed,
}

impl M5RecallDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ProofStale,
        Self::LocalityUnavailable,
        Self::EmbeddingGenerationMismatch,
        Self::StaleHybridRetrieval,
        Self::BudgetExhausted,
        Self::DeleteExportUnverified,
        Self::ProviderUnavailable,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::LocalityUnavailable => "locality_unavailable",
            Self::EmbeddingGenerationMismatch => "embedding_generation_mismatch",
            Self::StaleHybridRetrieval => "stale_hybrid_retrieval",
            Self::BudgetExhausted => "budget_exhausted",
            Self::DeleteExportUnverified => "delete_export_unverified",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project a recall row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RecallConsumerSurface {
    /// Desktop composer or inline assist UI.
    DesktopComposer,
    /// Docs or in-app browser companion.
    DocsBrowserCompanion,
    /// Search surface.
    SearchSurface,
    /// Support/export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Managed or offline usage report.
    ManagedOfflineReport,
}

impl M5RecallConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopComposer,
        Self::DocsBrowserCompanion,
        Self::SearchSurface,
        Self::SupportExport,
        Self::Diagnostics,
        Self::ManagedOfflineReport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopComposer => "desktop_composer",
            Self::DocsBrowserCompanion => "docs_browser_companion",
            Self::SearchSurface => "search_surface",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::ManagedOfflineReport => "managed_offline_report",
        }
    }
}

/// One row in the M5 AI recall matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiRecallMatrixSurfaceRow {
    /// Recall surface.
    pub surface: M5RecallSurface,
    /// Qualification class declared for this surface.
    pub qualification: M5RecallQualificationClass,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Memory and cache classes held or consumed by this surface.
    pub memory_cache_classes: Vec<M5MemoryCacheClass>,
    /// Retrieval lanes contributing to this surface.
    pub retrieval_lanes: Vec<M5RetrievalLane>,
    /// Locality posture for the data this surface holds or reads.
    pub locality_posture: M5LocalityPosture,
    /// Delete and export posture for durable artifacts.
    pub delete_export_posture: M5DeleteExportPosture,
    /// Budget and receipt expectation.
    pub budget_receipt_expectation: M5BudgetReceiptExpectation,
    /// Cache and memory invalidation classes that apply to this surface.
    pub cache_invalidation_classes: Vec<M5CacheInvalidationClass>,
    /// Evidence requirement level.
    pub evidence_requirement: M5RecallEvidenceRequirement,
    /// Required evidence packet refs for this qualification.
    pub required_evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<M5RecallDowngradeTrigger>,
    /// Source contract refs consumed by this surface.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this surface.
    pub consumer_surfaces: Vec<M5RecallConsumerSurface>,
}

impl M5AiRecallMatrixSurfaceRow {
    /// Whether durable memory on this row declares a real delete/export posture.
    pub fn delete_export_complete(&self) -> bool {
        if self
            .memory_cache_classes
            .iter()
            .any(|class| class.is_durable())
        {
            self.delete_export_posture != M5DeleteExportPosture::NotApplicable
        } else {
            true
        }
    }

    /// Whether every recall dimension required for a Stable claim is present.
    ///
    /// A surface that omits its memory/cache classes, retrieval lanes,
    /// cache-invalidation classes, downgrade triggers, or a delete/export posture
    /// for durable memory is not recall-complete and cannot hold Stable.
    pub fn is_recall_complete(&self) -> bool {
        !self.memory_cache_classes.is_empty()
            && !self.retrieval_lanes.is_empty()
            && !self.cache_invalidation_classes.is_empty()
            && !self.downgrade_triggers.is_empty()
            && !self.required_evidence_packet_refs.is_empty()
            && self.delete_export_complete()
    }

    /// Effective qualification after auto-narrowing on incomplete recall evidence.
    ///
    /// A declared Stable surface that is not [recall-complete](Self::is_recall_complete)
    /// narrows to Preview so the claim never outruns its evidence.
    pub fn effective_qualification(&self) -> M5RecallQualificationClass {
        if self.qualification.is_stable() && !self.is_recall_complete() {
            M5RecallQualificationClass::Preview
        } else {
            self.qualification
        }
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiRecallMatrixGuardrails {
    /// No cross-workspace recall happens by default.
    pub no_cross_workspace_recall_by_default: bool,
    /// No cross-tenant recall happens by default.
    pub no_cross_tenant_recall_by_default: bool,
    /// Mixed-generation embeddings are always labeled, never silently merged.
    pub mixed_generation_embeddings_labeled: bool,
    /// Stale hybrid retrieval never masquerades as current truth.
    pub stale_hybrid_retrieval_never_current_truth: bool,
    /// Every durable artifact declares its retention/delete/export posture.
    pub every_durable_artifact_declares_retention: bool,
    /// Prompt-result caches are not used as shadow telemetry stores.
    pub caches_are_not_shadow_telemetry: bool,
    /// Spend or route failures resolve to a precise fallback, not a generic error.
    pub spend_or_route_failures_have_precise_fallback: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiRecallMatrixConsumerProjection {
    /// Composer shows which memory and retrieval lanes were used.
    pub composer_shows_memory_and_retrieval: bool,
    /// Docs/browser shows provenance and locality.
    pub docs_browser_shows_provenance_and_locality: bool,
    /// Search shows which retrieval lanes contributed.
    pub search_shows_retrieval_lanes: bool,
    /// Support export shows locality posture and receipts.
    pub support_export_shows_locality_and_receipts: bool,
    /// Diagnostics shows cache and budget state.
    pub diagnostics_shows_cache_and_budget: bool,
    /// Managed/offline reporting shows locality truth.
    pub managed_offline_shows_locality_truth: bool,
    /// Surfaces below Stable are visibly labeled, never presented as Stable.
    pub unqualified_surfaces_labeled_below_stable: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiRecallMatrixProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the surface.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5AiRecallMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AiRecallMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5AiRecallMatrixSurfaceRow>,
    /// Guardrail invariants block.
    pub guardrails: M5AiRecallMatrixGuardrails,
    /// Consumer projection block.
    pub consumer_projection: M5AiRecallMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiRecallMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 AI recall matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiRecallMatrixPacket {
    /// Record kind; must equal [`M5_AI_RECALL_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AI_RECALL_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5AiRecallMatrixSurfaceRow>,
    /// Guardrail invariants block.
    pub guardrails: M5AiRecallMatrixGuardrails,
    /// Consumer projection block.
    pub consumer_projection: M5AiRecallMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiRecallMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AiRecallMatrixPacket {
    /// Builds an M5 AI recall matrix packet from surface input.
    pub fn new(input: M5AiRecallMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_AI_RECALL_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_AI_RECALL_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 AI recall matrix invariants.
    pub fn validate(&self) -> Vec<M5AiRecallMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_AI_RECALL_MATRIX_RECORD_KIND {
            violations.push(M5AiRecallMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AI_RECALL_MATRIX_SCHEMA_VERSION {
            violations.push(M5AiRecallMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AiRecallMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 ai recall matrix packet serializes"),
        ) {
            violations.push(M5AiRecallMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 ai recall matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_surfaces = self
            .surface_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 AI Recall Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Surfaces: {} ({} stable)\n",
            self.surface_rows.len(),
            stable_surfaces
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Locality: `{}`\n",
                row.locality_posture.as_str()
            ));
            out.push_str(&format!(
                "  - Delete/export: `{}`\n",
                row.delete_export_posture.as_str()
            ));
            out.push_str(&format!(
                "  - Budget/receipt: `{}`\n",
                row.budget_receipt_expectation.as_str()
            ));
            out.push_str(&format!(
                "  - Invalidation: {} classes\n",
                row.cache_invalidation_classes.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 AI recall matrix export.
#[derive(Debug)]
pub enum M5AiRecallMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AiRecallMatrixViolation>),
}

impl fmt::Display for M5AiRecallMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 ai recall matrix export parse failed: {error}"
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
                    "m5 ai recall matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AiRecallMatrixArtifactError {}

/// Validation failures emitted by [`M5AiRecallMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AiRecallMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required surface is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface claiming Stable is missing required evidence packet refs.
    StableSurfaceMissingEvidence,
    /// A surface has no cache-invalidation classes.
    MissingCacheInvalidation,
    /// A surface holding durable memory does not declare a delete/export posture.
    DurableMemoryMissingDeleteExport,
    /// A surface has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A declared Stable claim outruns its recall evidence.
    StableClaimExceedsRecallEvidence,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5AiRecallMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::StableSurfaceMissingEvidence => "stable_surface_missing_evidence",
            Self::MissingCacheInvalidation => "missing_cache_invalidation",
            Self::DurableMemoryMissingDeleteExport => "durable_memory_missing_delete_export",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::StableClaimExceedsRecallEvidence => "stable_claim_exceeds_recall_evidence",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 AI recall matrix export.
pub fn current_stable_m5_ai_recall_matrix_export(
) -> Result<M5AiRecallMatrixPacket, M5AiRecallMatrixArtifactError> {
    let packet: M5AiRecallMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix/support_export.json"
    )))
    .map_err(M5AiRecallMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AiRecallMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5AiRecallMatrixPacket,
    violations: &mut Vec<M5AiRecallMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_AI_RECALL_MATRIX_SCHEMA_REF,
        M5_AI_RECALL_MATRIX_DOC_REF,
        M5_AI_RECALL_MATRIX_MEMORY_CLASS_CONTRACT_REF,
        M5_AI_RECALL_MATRIX_DELETE_EXPORT_CONTRACT_REF,
        M5_AI_RECALL_MATRIX_SPEND_RECEIPT_CONTRACT_REF,
        M5_AI_RECALL_MATRIX_CONTEXT_ASSEMBLY_CONTRACT_REF,
        M5_AI_RECALL_MATRIX_RETRIEVAL_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AiRecallMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_surface_rows(
    packet: &M5AiRecallMatrixPacket,
    violations: &mut Vec<M5AiRecallMatrixViolation>,
) {
    let present: BTreeSet<M5RecallSurface> =
        packet.surface_rows.iter().map(|row| row.surface).collect();
    for required in M5RecallSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5AiRecallMatrixViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.memory_cache_classes.is_empty()
            || row.retrieval_lanes.is_empty()
        {
            violations.push(M5AiRecallMatrixViolation::SurfaceRowIncomplete);
        }
        if row.cache_invalidation_classes.is_empty() {
            violations.push(M5AiRecallMatrixViolation::MissingCacheInvalidation);
        }
        if !row.delete_export_complete() {
            violations.push(M5AiRecallMatrixViolation::DurableMemoryMissingDeleteExport);
        }
        if row.qualification.is_stable() && row.required_evidence_packet_refs.is_empty() {
            violations.push(M5AiRecallMatrixViolation::StableSurfaceMissingEvidence);
        }
        if row.qualification.is_stable() && row.effective_qualification() != row.qualification {
            violations.push(M5AiRecallMatrixViolation::StableClaimExceedsRecallEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5AiRecallMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5AiRecallMatrixViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_guardrails(
    packet: &M5AiRecallMatrixPacket,
    violations: &mut Vec<M5AiRecallMatrixViolation>,
) {
    let guardrails = &packet.guardrails;
    let ok = guardrails.no_cross_workspace_recall_by_default
        && guardrails.no_cross_tenant_recall_by_default
        && guardrails.mixed_generation_embeddings_labeled
        && guardrails.stale_hybrid_retrieval_never_current_truth
        && guardrails.every_durable_artifact_declares_retention
        && guardrails.caches_are_not_shadow_telemetry
        && guardrails.spend_or_route_failures_have_precise_fallback;
    if !ok {
        violations.push(M5AiRecallMatrixViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &M5AiRecallMatrixPacket,
    violations: &mut Vec<M5AiRecallMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    let ok = projection.composer_shows_memory_and_retrieval
        && projection.docs_browser_shows_provenance_and_locality
        && projection.search_shows_retrieval_lanes
        && projection.support_export_shows_locality_and_receipts
        && projection.diagnostics_shows_cache_and_budget
        && projection.managed_offline_shows_locality_truth
        && projection.unqualified_surfaces_labeled_below_stable;
    if !ok {
        violations.push(M5AiRecallMatrixViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_proof_freshness(
    packet: &M5AiRecallMatrixPacket,
    violations: &mut Vec<M5AiRecallMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AiRecallMatrixViolation::ProofFreshnessIncomplete);
    }
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
