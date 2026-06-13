//! Certification of AI memory classes, prompt-result-cache and session-artifact
//! governance, hybrid-retrieval or embedding-locality, and spend-receipt truth on
//! every claimed M5 AI/docs/recall row.
//!
//! This module is the B32 capstone: where each earlier lane materializes one
//! truth object — memory-class objects
//! ([`crate::implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth`]),
//! prompt-composer draft and session-artifact records
//! ([`crate::ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts`]),
//! reusable semantic-memory and embedding-index records
//! ([`crate::ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth`]),
//! retrieval-locality inspectors
//! ([`crate::add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em`]),
//! and spend/route receipts
//! ([`crate::implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella`]) —
//! this packet binds those objects to **every claimed M5 AI/docs/recall row** and
//! makes them release-bearing. It reuses the qualification and downgrade
//! vocabularies frozen by the M5 AI workflow matrix lane rather than inventing
//! parallel terms, and it binds every certified row to the canonical source
//! schema of each pillar by requiring that schema in `source_contract_refs`, so no
//! surface may stay greener than this packet.
//!
//! Each [`CertifiedRecallRow`] binds one claimed AI/docs/recall surface to four
//! interlocking proof pillars ([`CertificationPillar`]): a **memory-class** proof,
//! a **prompt-result-cache and session-artifact governance** proof, a
//! **hybrid-retrieval or embedding-locality** proof, and a **spend-receipt** proof.
//! Each [`PillarProof`] declares whether its proof is
//! [`ProofState::Current`], where the data lived ([`LocalityClass`]), whether a
//! durable artifact declares retention/export posture, and whether a mixed
//! retrieval generation is labeled. The certification *auto-narrows*: a claimed
//! row whose any required pillar is not current must carry an
//! `effective_qualification` strictly below its claim, a recorded narrowing
//! trigger, and a precise degraded label — so completeness never outruns evidence.
//!
//! [`M5RecallRowCertificationPacket::validate`] refuses a row that hides managed
//! locality, mixes retrieval generations without labeling, leaves a durable pillar
//! without retention/export truth, or keeps a public claim while a pillar's proof
//! is stale or missing.
//!
//! Raw prompt bodies, cached result bodies, raw embeddings, raw provider payloads,
//! credentials, raw endpoint URLs, exact token counts, and exact cost amounts
//! never cross this boundary; the packet carries only typed proof states, class
//! tokens, and evidence refs.
//!
//! The boundary schema is
//! [`schemas/ai/certify-ai-memory-classes-prompt-result-cache-and-session-artifact-governance-hybrid-retrieval-or-embedding-locality-and.schema.json`](../../../../schemas/ai/certify-ai-memory-classes-prompt-result-cache-and-session-artifact-governance-hybrid-retrieval-or-embedding-locality-and.schema.json).
//! The contract doc is
//! [`docs/ai/m5/certify_ai_memory_classes_prompt_result_cache_and_session_artifact_governance_hybrid_retrieval_or_embedding_locality_and.md`](../../../../docs/ai/m5/certify_ai_memory_classes_prompt_result_cache_and_session_artifact_governance_hybrid_retrieval_or_embedding_locality_and.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/certify_ai_memory_classes_prompt_result_cache_and_session_artifact_governance_hybrid_retrieval_or_embedding_locality_and/`](../../../../fixtures/ai/m5/certify_ai_memory_classes_prompt_result_cache_and_session_artifact_governance_hybrid_retrieval_or_embedding_locality_and/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr::MEMORY_FENCE_FALLBACK_SCHEMA_REF;
use crate::add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em::RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_REF;
use crate::freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix::{
    M5_AI_RECALL_MATRIX_ARTIFACT_REF, M5_AI_RECALL_MATRIX_SCHEMA_REF,
};
use crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass,
};
use crate::implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella::AI_RUN_RECEIPT_SCHEMA_REF;
use crate::implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth::MEMORY_CLASS_MATERIALIZATION_SCHEMA_REF;
use crate::ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts::SESSION_ARTIFACT_SCHEMA_REF;
use crate::ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth::SEMANTIC_RECALL_RECORDS_SCHEMA_REF;

/// Stable record-kind tag carried by [`M5RecallRowCertificationPacket`].
pub const M5_RECALL_ROW_CERTIFICATION_RECORD_KIND: &str =
    "certify_ai_memory_classes_prompt_result_cache_and_session_artifact_governance_hybrid_retrieval_or_embedding_locality_and";

/// Schema version for M5 recall-row certification records.
pub const M5_RECALL_ROW_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_RECALL_ROW_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/ai/certify-ai-memory-classes-prompt-result-cache-and-session-artifact-governance-hybrid-retrieval-or-embedding-locality-and.schema.json";

/// Repo-relative path of the certification contract doc.
pub const M5_RECALL_ROW_CERTIFICATION_DOC_REF: &str =
    "docs/ai/m5/certify_ai_memory_classes_prompt_result_cache_and_session_artifact_governance_hybrid_retrieval_or_embedding_locality_and.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_RECALL_ROW_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/ai/m5/certify_ai_memory_classes_prompt_result_cache_and_session_artifact_governance_hybrid_retrieval_or_embedding_locality_and";

/// Repo-relative path of the checked support-export artifact.
pub const M5_RECALL_ROW_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/ai/m5/certify_ai_memory_classes_prompt_result_cache_and_session_artifact_governance_hybrid_retrieval_or_embedding_locality_and/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_RECALL_ROW_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/ai/m5/certify_ai_memory_classes_prompt_result_cache_and_session_artifact_governance_hybrid_retrieval_or_embedding_locality_and.md";

/// One claimed M5 AI/docs/recall surface a certified row covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecallSurface {
    /// Composer inline assist and prompt-composer recall.
    ComposerInlineAssist,
    /// Evidence-rich patch review recall.
    PatchReview,
    /// Branch or worktree side-agent recall.
    BranchWorktreeAgent,
    /// Docs and in-app browser recall with provenance.
    DocsBrowserRecall,
    /// Codebase-understanding recall over the workspace graph.
    CodeUnderstanding,
    /// Semantic and hybrid search recall.
    SemanticHybridSearch,
    /// Managed or offline usage and locality reporting.
    ManagedOfflineReport,
    /// Support / export packet projection.
    SupportExport,
}

impl RecallSurface {
    /// Every claimed surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ComposerInlineAssist,
        Self::PatchReview,
        Self::BranchWorktreeAgent,
        Self::DocsBrowserRecall,
        Self::CodeUnderstanding,
        Self::SemanticHybridSearch,
        Self::ManagedOfflineReport,
        Self::SupportExport,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComposerInlineAssist => "composer_inline_assist",
            Self::PatchReview => "patch_review",
            Self::BranchWorktreeAgent => "branch_worktree_agent",
            Self::DocsBrowserRecall => "docs_browser_recall",
            Self::CodeUnderstanding => "code_understanding",
            Self::SemanticHybridSearch => "semantic_hybrid_search",
            Self::ManagedOfflineReport => "managed_offline_report",
            Self::SupportExport => "support_export",
        }
    }
}

/// One proof pillar every claimed AI/docs/recall row must carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationPillar {
    /// Turn/thread/workspace/org memory-class objects with deletion/export truth.
    MemoryClass,
    /// Prompt-result-cache and session-artifact governance records.
    PromptCacheSessionArtifact,
    /// Hybrid-retrieval lanes or embedding-locality records.
    HybridRetrievalLocality,
    /// Spend and route receipts.
    SpendReceipt,
}

impl CertificationPillar {
    /// Every required proof pillar, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::MemoryClass,
        Self::PromptCacheSessionArtifact,
        Self::HybridRetrievalLocality,
        Self::SpendReceipt,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MemoryClass => "memory_class",
            Self::PromptCacheSessionArtifact => "prompt_cache_session_artifact",
            Self::HybridRetrievalLocality => "hybrid_retrieval_locality",
            Self::SpendReceipt => "spend_receipt",
        }
    }

    /// Canonical source schemas this pillar's proof must stay aligned with.
    ///
    /// The certification binds every pillar to the schema of the first-consumer
    /// packet that materializes it and requires each ref in
    /// `source_contract_refs`, so a pillar can never claim more than its canonical
    /// source schemas admit.
    pub const fn canonical_schema_refs(self) -> &'static [&'static str] {
        match self {
            Self::MemoryClass => &[
                MEMORY_CLASS_MATERIALIZATION_SCHEMA_REF,
                MEMORY_FENCE_FALLBACK_SCHEMA_REF,
            ],
            Self::PromptCacheSessionArtifact => &[SESSION_ARTIFACT_SCHEMA_REF],
            Self::HybridRetrievalLocality => &[
                RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_REF,
                SEMANTIC_RECALL_RECORDS_SCHEMA_REF,
            ],
            Self::SpendReceipt => &[AI_RUN_RECEIPT_SCHEMA_REF],
        }
    }

    /// Whether `schema_ref` is one of this pillar's canonical source schemas.
    pub fn admits_schema(self, schema_ref: &str) -> bool {
        self.canonical_schema_refs()
            .iter()
            .any(|candidate| *candidate == schema_ref)
    }
}

/// Whether a pillar's checked-in proof is current, stale, or missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofState {
    /// The proof exists and is within its freshness bound.
    Current,
    /// The proof exists but has aged past its freshness bound.
    Stale,
    /// No proof exists for this pillar on this row.
    Missing,
}

impl ProofState {
    /// Every proof state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Current, Self::Stale, Self::Missing];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }

    /// Whether this state is current and so keeps a public claim alive.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// Where a pillar's recall data lived.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalityClass {
    /// On-device only.
    LocalOnDevice,
    /// First-party managed endpoint; must be disclosed, never implied local.
    ManagedHosted,
    /// On-device from a mirrored or offline pack channel.
    MirroredOffline,
    /// Mixed local and managed contribution, explicitly labeled per lane.
    MixedLabeled,
}

impl LocalityClass {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnDevice => "local_on_device",
            Self::ManagedHosted => "managed_hosted",
            Self::MirroredOffline => "mirrored_offline",
            Self::MixedLabeled => "mixed_labeled",
        }
    }

    /// Whether this locality crosses to a managed endpoint and so must be
    /// disclosed rather than implied local.
    pub const fn is_managed(self) -> bool {
        matches!(self, Self::ManagedHosted | Self::MixedLabeled)
    }
}

/// One pillar's proof binding for a certified recall row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PillarProof {
    /// Pillar this proof covers.
    pub pillar: CertificationPillar,
    /// Whether the checked-in proof is current, stale, or missing.
    pub proof_state: ProofState,
    /// Canonical source schema ref this proof is drawn from; must be one of the
    /// pillar's [`CertificationPillar::canonical_schema_refs`].
    pub canonical_schema_ref: String,
    /// Where the recall data lived.
    pub locality: LocalityClass,
    /// True when the locality is disclosed in-product, required for managed or
    /// mixed locality so it never masquerades as local.
    pub locality_disclosed: bool,
    /// True when the pillar holds durable material and so must declare a
    /// retention/delete/export posture.
    pub durable: bool,
    /// True when a durable pillar's retention/delete/export posture is declared.
    pub retention_export_declared: bool,
    /// True when this pillar mixes retrieval or embedding generations.
    pub mixed_generation_present: bool,
    /// True when a mixed generation is labeled rather than passed off as current.
    pub mixed_generation_labeled: bool,
    /// Evidence packet refs backing this proof.
    pub evidence_refs: Vec<String>,
}

impl PillarProof {
    /// Whether the proof is current.
    pub fn is_current(&self) -> bool {
        self.proof_state.is_current()
    }

    /// Whether the canonical schema ref names one of the pillar's source schemas.
    pub fn schema_ref_admitted(&self) -> bool {
        self.pillar.admits_schema(self.canonical_schema_ref.trim())
    }

    /// Whether managed or mixed locality is disclosed.
    pub fn locality_ok(&self) -> bool {
        if self.locality.is_managed() {
            self.locality_disclosed
        } else {
            true
        }
    }

    /// Whether a durable pillar declares its retention/delete/export posture.
    pub fn retention_ok(&self) -> bool {
        if self.durable {
            self.retention_export_declared
        } else {
            true
        }
    }

    /// Whether a mixed retrieval generation is labeled.
    pub fn generation_ok(&self) -> bool {
        if self.mixed_generation_present {
            self.mixed_generation_labeled
        } else {
            true
        }
    }

    /// Whether this proof is structurally complete and internally consistent.
    pub fn is_complete(&self) -> bool {
        self.schema_ref_admitted()
            && self.locality_ok()
            && self.retention_ok()
            && self.generation_ok()
            && !self.evidence_refs.is_empty()
            && self
                .evidence_refs
                .iter()
                .all(|reference| !reference.trim().is_empty())
    }
}

/// One certified M5 AI/docs/recall row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedRecallRow {
    /// Stable row id.
    pub row_id: String,
    /// Claimed AI/docs/recall surface.
    pub surface: RecallSurface,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Headline qualification publicly claimed for this row.
    pub claimed_qualification: M5AiWorkflowQualificationClass,
    /// Effective qualification after auto-narrowing; equals the claim when every
    /// pillar is current, and ranks strictly below it otherwise.
    pub effective_qualification: M5AiWorkflowQualificationClass,
    /// The four pillar proofs; exactly one per [`CertificationPillar`].
    pub pillar_proofs: Vec<PillarProof>,
    /// Trigger that fired the narrowing, required when the row is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrow_trigger: Option<M5AiWorkflowDowngradeTrigger>,
    /// Precise degraded label, required when the row is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl CertifiedRecallRow {
    /// Whether this row carries a public claim.
    pub fn is_claimed(&self) -> bool {
        is_claimed_qualification(self.claimed_qualification)
    }

    /// Whether every required pillar appears exactly once.
    pub fn covers_all_pillars(&self) -> bool {
        let mut seen: BTreeSet<CertificationPillar> = BTreeSet::new();
        for proof in &self.pillar_proofs {
            if !seen.insert(proof.pillar) {
                return false;
            }
        }
        CertificationPillar::ALL
            .iter()
            .all(|pillar| seen.contains(pillar))
    }

    /// Whether every pillar proof is current.
    pub fn all_pillars_current(&self) -> bool {
        self.pillar_proofs.iter().all(PillarProof::is_current)
    }

    /// Whether the row must narrow below its claim because a pillar is not current.
    pub fn needs_narrowing(&self) -> bool {
        !self.all_pillars_current()
    }

    /// Whether the effective qualification and narrowing evidence are consistent.
    ///
    /// When every pillar is current the effective qualification equals the claim;
    /// otherwise it must rank strictly below the claim and carry both a recorded
    /// narrowing trigger and a precise degraded label.
    pub fn narrowing_consistent(&self) -> bool {
        if self.needs_narrowing() {
            qualification_rank(self.effective_qualification)
                < qualification_rank(self.claimed_qualification)
                && self.narrow_trigger.is_some()
                && self
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.effective_qualification == self.claimed_qualification
        }
    }

    /// Whether every dimension required to certify this row is present.
    pub fn is_complete(&self) -> bool {
        !self.row_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.covers_all_pillars()
            && self.pillar_proofs.iter().all(PillarProof::is_complete)
            && self.narrowing_consistent()
            && !self.evidence_refs.is_empty()
            && !self.source_contract_refs.is_empty()
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowCertGuardrails {
    /// No cross-workspace or cross-tenant recall happens by default.
    pub no_default_cross_scope_recall: bool,
    /// Prompt-result caches never behave like shadow-telemetry stores.
    pub prompt_result_caches_are_not_shadow_telemetry: bool,
    /// Mixed-generation retrieval is always labeled, never passed as current.
    pub mixed_generation_always_labeled: bool,
    /// Managed locality is always disclosed, never implied local.
    pub managed_locality_always_disclosed: bool,
    /// Every durable artifact declares its retention/delete/export posture.
    pub every_durable_artifact_declares_retention_export: bool,
    /// Spend or route failures keep a precise fallback rather than a generic error.
    pub spend_route_failures_keep_precise_fallback: bool,
    /// Any row lacking current pillar proof auto-narrows below its claim.
    pub rows_auto_narrow_on_stale_proof: bool,
}

impl RowCertGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.no_default_cross_scope_recall
            && self.prompt_result_caches_are_not_shadow_telemetry
            && self.mixed_generation_always_labeled
            && self.managed_locality_always_disclosed
            && self.every_durable_artifact_declares_retention_export
            && self.spend_route_failures_keep_precise_fallback
            && self.rows_auto_narrow_on_stale_proof
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowCertConsumerProjection {
    /// Product surfaces ingest this certification instead of cloning behavior text.
    pub product_ingests_certification: bool,
    /// Docs/help ingests the same certification result.
    pub docs_help_ingests_certification: bool,
    /// Diagnostics ingests the same certification result.
    pub diagnostics_ingests_certification: bool,
    /// Release surfaces ingest the same certification result.
    pub release_ingests_certification: bool,
    /// Narrowed rows are visibly labeled below current in every surface.
    pub narrowed_rows_labeled_below_current: bool,
}

impl RowCertConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_certification
            && self.docs_help_ingests_certification
            && self.diagnostics_ingests_certification
            && self.release_ingests_certification
            && self.narrowed_rows_labeled_below_current
    }
}

/// Proof freshness block for the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowCertProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed rows.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5RecallRowCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RecallRowCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Per-row certifications.
    pub rows: Vec<CertifiedRecallRow>,
    /// Guardrail invariants block.
    pub guardrails: RowCertGuardrails,
    /// Consumer projection block.
    pub consumer_projection: RowCertConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RowCertProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 recall-row certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RecallRowCertificationPacket {
    /// Record kind; must equal [`M5_RECALL_ROW_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RECALL_ROW_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Per-row certifications.
    pub rows: Vec<CertifiedRecallRow>,
    /// Guardrail invariants block.
    pub guardrails: RowCertGuardrails,
    /// Consumer projection block.
    pub consumer_projection: RowCertConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RowCertProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RecallRowCertificationPacket {
    /// Builds an M5 recall-row certification packet.
    pub fn new(input: M5RecallRowCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_RECALL_ROW_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_RECALL_ROW_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<RecallSurface> {
        self.rows.iter().map(|row| row.surface).collect()
    }

    /// Count of rows whose effective qualification was narrowed below its claim.
    pub fn narrowed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.needs_narrowing()).count()
    }

    /// Count of rows holding a public claim.
    pub fn claimed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.is_claimed()).count()
    }

    /// Validates the M5 recall-row certification invariants.
    pub fn validate(&self) -> Vec<M5RecallRowCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RECALL_ROW_CERTIFICATION_RECORD_KIND {
            violations.push(M5RecallRowCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RECALL_ROW_CERTIFICATION_SCHEMA_VERSION {
            violations.push(M5RecallRowCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RecallRowCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 recall row certification packet serializes"),
        ) {
            violations.push(M5RecallRowCertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 recall row certification packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 AI/Docs/Recall Row Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Rows: {} ({} claimed, {} narrowed)\n",
            self.rows.len(),
            self.claimed_row_count(),
            self.narrowed_row_count()
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {}\n",
            self.represented_surfaces().len(),
            RecallSurface::ALL.len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}): claim `{}` -> effective `{}`\n",
                row.row_id,
                row.surface.as_str(),
                row.claimed_qualification.as_str(),
                row.effective_qualification.as_str()
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            for proof in &row.pillar_proofs {
                out.push_str(&format!(
                    "  - {}: `{}` ({})\n",
                    proof.pillar.as_str(),
                    proof.proof_state.as_str(),
                    proof.locality.as_str()
                ));
            }
            if let Some(label) = &row.degraded_label {
                out.push_str(&format!("  - Degraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in recall-row certification export.
#[derive(Debug)]
pub enum M5RecallRowCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RecallRowCertificationViolation>),
}

impl fmt::Display for M5RecallRowCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 recall row certification export parse failed: {error}"
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
                    "m5 recall row certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RecallRowCertificationArtifactError {}

/// Validation failures emitted by [`M5RecallRowCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RecallRowCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A pillar's canonical source schema is not bound in source contracts.
    PillarCanonicalSchemaUnbound,
    /// A required AI/docs/recall surface is represented by no row.
    RequiredSurfaceMissing,
    /// No row demonstrates auto-narrowing on a non-current pillar proof.
    NarrowedRowCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A row does not carry exactly one proof per pillar.
    PillarCoverageIncomplete,
    /// A pillar proof names a schema outside the pillar's canonical sources.
    PillarSchemaRefNotAdmitted,
    /// Managed or mixed locality is not disclosed.
    ManagedLocalityNotDisclosed,
    /// A mixed retrieval generation is not labeled.
    MixedGenerationNotLabeled,
    /// A durable pillar does not declare a retention/delete/export posture.
    DurablePillarMissingRetentionExport,
    /// A pillar proof lacks evidence refs.
    PillarEvidenceMissing,
    /// A row lacking current pillar proof was not narrowed below its claim.
    RowNotNarrowedOnStaleProof,
    /// A narrowed row lacks a precise degraded label or narrowing trigger.
    NarrowedRowMissingLabelOrTrigger,
    /// A claimed row is missing required evidence refs.
    ClaimedRowMissingEvidence,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5RecallRowCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::PillarCanonicalSchemaUnbound => "pillar_canonical_schema_unbound",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::NarrowedRowCaseMissing => "narrowed_row_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::PillarCoverageIncomplete => "pillar_coverage_incomplete",
            Self::PillarSchemaRefNotAdmitted => "pillar_schema_ref_not_admitted",
            Self::ManagedLocalityNotDisclosed => "managed_locality_not_disclosed",
            Self::MixedGenerationNotLabeled => "mixed_generation_not_labeled",
            Self::DurablePillarMissingRetentionExport => "durable_pillar_missing_retention_export",
            Self::PillarEvidenceMissing => "pillar_evidence_missing",
            Self::RowNotNarrowedOnStaleProof => "row_not_narrowed_on_stale_proof",
            Self::NarrowedRowMissingLabelOrTrigger => "narrowed_row_missing_label_or_trigger",
            Self::ClaimedRowMissingEvidence => "claimed_row_missing_evidence",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable recall-row certification export.
pub fn current_m5_recall_row_certification_export(
) -> Result<M5RecallRowCertificationPacket, M5RecallRowCertificationArtifactError> {
    let packet: M5RecallRowCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/certify_ai_memory_classes_prompt_result_cache_and_session_artifact_governance_hybrid_retrieval_or_embedding_locality_and/support_export.json"
    )))
    .map_err(M5RecallRowCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RecallRowCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Whether a qualification class is a publicly claimed lane.
///
/// Stable, Beta, and Preview are claimed; Experimental, Held, and Unavailable
/// are not.
fn is_claimed_qualification(class: M5AiWorkflowQualificationClass) -> bool {
    matches!(
        class,
        M5AiWorkflowQualificationClass::Stable
            | M5AiWorkflowQualificationClass::Beta
            | M5AiWorkflowQualificationClass::Preview
    )
}

/// Ordinal rank used to compare qualification severity for auto-narrowing.
///
/// Higher means a stronger public claim, so a narrowing must move to a strictly
/// lower rank.
fn qualification_rank(class: M5AiWorkflowQualificationClass) -> u8 {
    match class {
        M5AiWorkflowQualificationClass::Unavailable => 0,
        M5AiWorkflowQualificationClass::Held => 1,
        M5AiWorkflowQualificationClass::Experimental => 2,
        M5AiWorkflowQualificationClass::Preview => 3,
        M5AiWorkflowQualificationClass::Beta => 4,
        M5AiWorkflowQualificationClass::Stable => 5,
    }
}

fn validate_source_contracts(
    packet: &M5RecallRowCertificationPacket,
    violations: &mut Vec<M5RecallRowCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_RECALL_ROW_CERTIFICATION_SCHEMA_REF,
        M5_RECALL_ROW_CERTIFICATION_DOC_REF,
        M5_AI_RECALL_MATRIX_SCHEMA_REF,
        M5_AI_RECALL_MATRIX_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5RecallRowCertificationViolation::MissingSourceContracts);
            break;
        }
    }

    for pillar in CertificationPillar::ALL {
        if !pillar
            .canonical_schema_refs()
            .iter()
            .all(|schema_ref| refs.contains(schema_ref))
        {
            violations.push(M5RecallRowCertificationViolation::PillarCanonicalSchemaUnbound);
            break;
        }
    }
}

fn validate_coverage(
    packet: &M5RecallRowCertificationPacket,
    violations: &mut Vec<M5RecallRowCertificationViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in RecallSurface::ALL {
        if !surfaces.contains(&required) {
            violations.push(M5RecallRowCertificationViolation::RequiredSurfaceMissing);
            break;
        }
    }

    if !packet
        .rows
        .iter()
        .any(|row| row.needs_narrowing() && row.narrowing_consistent())
    {
        violations.push(M5RecallRowCertificationViolation::NarrowedRowCaseMissing);
    }
}

fn validate_rows(
    packet: &M5RecallRowCertificationPacket,
    violations: &mut Vec<M5RecallRowCertificationViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(M5RecallRowCertificationViolation::RowIncomplete);
        }
        if !row.covers_all_pillars() {
            violations.push(M5RecallRowCertificationViolation::PillarCoverageIncomplete);
        }
        for proof in &row.pillar_proofs {
            if !proof.schema_ref_admitted() {
                violations.push(M5RecallRowCertificationViolation::PillarSchemaRefNotAdmitted);
            }
            if !proof.locality_ok() {
                violations.push(M5RecallRowCertificationViolation::ManagedLocalityNotDisclosed);
            }
            if !proof.generation_ok() {
                violations.push(M5RecallRowCertificationViolation::MixedGenerationNotLabeled);
            }
            if !proof.retention_ok() {
                violations
                    .push(M5RecallRowCertificationViolation::DurablePillarMissingRetentionExport);
            }
            if proof.evidence_refs.is_empty()
                || proof
                    .evidence_refs
                    .iter()
                    .any(|reference| reference.trim().is_empty())
            {
                violations.push(M5RecallRowCertificationViolation::PillarEvidenceMissing);
            }
        }
        if row.needs_narrowing()
            && qualification_rank(row.effective_qualification)
                >= qualification_rank(row.claimed_qualification)
        {
            violations.push(M5RecallRowCertificationViolation::RowNotNarrowedOnStaleProof);
        }
        if row.needs_narrowing()
            && (row.narrow_trigger.is_none()
                || !row
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations.push(M5RecallRowCertificationViolation::NarrowedRowMissingLabelOrTrigger);
        }
        if row.is_claimed() && row.evidence_refs.is_empty() {
            violations.push(M5RecallRowCertificationViolation::ClaimedRowMissingEvidence);
        }
    }
}

fn validate_guardrails(
    packet: &M5RecallRowCertificationPacket,
    violations: &mut Vec<M5RecallRowCertificationViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(M5RecallRowCertificationViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &M5RecallRowCertificationPacket,
    violations: &mut Vec<M5RecallRowCertificationViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(M5RecallRowCertificationViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_proof_freshness(
    packet: &M5RecallRowCertificationPacket,
    violations: &mut Vec<M5RecallRowCertificationViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5RecallRowCertificationViolation::ProofFreshnessIncomplete);
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
///
/// A generic provider error must never stand in for a precise narrowing truth.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "narrowed"
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
