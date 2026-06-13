//! Conformance dump for the M5 AI/docs/recall row certification packet.
//!
//! Prints the canonical support export (default), the Markdown summary
//! (`summary` argument), or the narrowed fixture (`fixture` argument) so the
//! checked-in artifact and fixtures stay byte-aligned with the in-crate builder.

use aureline_ai::add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr::MEMORY_FENCE_FALLBACK_SCHEMA_REF;
use aureline_ai::add_retrieval_locality_inspectors_contribution_lanes_ranking_or_chunking_reasons_and_lexical_or_graph_or_docs_pack_or_em::RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_REF;
use aureline_ai::certify_ai_memory_classes_prompt_result_cache_and_session_artifact_governance_hybrid_retrieval_or_embedding_locality_and::*;
use aureline_ai::freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix::{
    M5_AI_RECALL_MATRIX_ARTIFACT_REF, M5_AI_RECALL_MATRIX_SCHEMA_REF,
};
use aureline_ai::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass,
};
use aureline_ai::implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella::AI_RUN_RECEIPT_SCHEMA_REF;
use aureline_ai::implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth::MEMORY_CLASS_MATERIALIZATION_SCHEMA_REF;
use aureline_ai::ship_prompt_composer_draft_and_session_artifact_records_attachment_and_mention_provenance_context_add_or_remove_receipts::SESSION_ARTIFACT_SCHEMA_REF;
use aureline_ai::ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth::SEMANTIC_RECALL_RECORDS_SCHEMA_REF;

fn evidence(refs: &[&str]) -> Vec<String> {
    refs.iter().map(|r| (*r).to_owned()).collect()
}

/// A current proof for one pillar.
fn current_proof(
    pillar: CertificationPillar,
    schema: &str,
    locality: LocalityClass,
    durable: bool,
    mixed: bool,
    ev: &str,
) -> PillarProof {
    PillarProof {
        pillar,
        proof_state: ProofState::Current,
        canonical_schema_ref: schema.to_owned(),
        locality,
        locality_disclosed: locality.is_managed(),
        durable,
        retention_export_declared: durable,
        mixed_generation_present: mixed,
        mixed_generation_labeled: mixed,
        evidence_refs: evidence(&[ev]),
    }
}

/// The four current pillar proofs for a fully proven row at the given locality.
fn current_pillars(
    locality: LocalityClass,
    hybrid_schema: &str,
    hybrid_mixed: bool,
    ev: &str,
) -> Vec<PillarProof> {
    vec![
        current_proof(
            CertificationPillar::MemoryClass,
            MEMORY_CLASS_MATERIALIZATION_SCHEMA_REF,
            locality,
            true,
            false,
            &format!("evidence:memory-class:{ev}"),
        ),
        current_proof(
            CertificationPillar::PromptCacheSessionArtifact,
            SESSION_ARTIFACT_SCHEMA_REF,
            locality,
            true,
            false,
            &format!("evidence:session-artifact:{ev}"),
        ),
        current_proof(
            CertificationPillar::HybridRetrievalLocality,
            hybrid_schema,
            locality,
            false,
            hybrid_mixed,
            &format!("evidence:retrieval-locality:{ev}"),
        ),
        current_proof(
            CertificationPillar::SpendReceipt,
            AI_RUN_RECEIPT_SCHEMA_REF,
            locality,
            true,
            false,
            &format!("evidence:spend-receipt:{ev}"),
        ),
    ]
}

fn row(
    row_id: &str,
    surface: RecallSurface,
    label: &str,
    claimed: M5AiWorkflowQualificationClass,
    pillars: Vec<PillarProof>,
) -> CertifiedRecallRow {
    CertifiedRecallRow {
        row_id: row_id.to_owned(),
        surface,
        label_summary: label.to_owned(),
        claimed_qualification: claimed,
        effective_qualification: claimed,
        pillar_proofs: pillars,
        narrow_trigger: None,
        degraded_label: None,
        evidence_refs: evidence(&[&format!("evidence:row:{row_id}")]),
        source_contract_refs: vec![M5_RECALL_ROW_CERTIFICATION_DOC_REF.to_owned()],
    }
}

/// The capstone rows for every claimed M5 AI/docs/recall surface. The
/// support-export row that drives managed/offline reporting carries a stale
/// spend-receipt proof and so auto-narrows from Stable to Beta with a precise
/// degraded label.
fn rows(narrowed_support_export: bool) -> Vec<CertifiedRecallRow> {
    let mut support_pillars = current_pillars(
        LocalityClass::ManagedHosted,
        SEMANTIC_RECALL_RECORDS_SCHEMA_REF,
        false,
        "support-export",
    );
    if narrowed_support_export {
        support_pillars[3].proof_state = ProofState::Stale;
    }
    let mut support_row = row(
        "recall-row:support-export:0001",
        RecallSurface::SupportExport,
        "Support/export projection of recall provenance, retention, and spend receipts",
        M5AiWorkflowQualificationClass::Stable,
        support_pillars,
    );
    if narrowed_support_export {
        support_row.effective_qualification = M5AiWorkflowQualificationClass::Beta;
        support_row.narrow_trigger = Some(M5AiWorkflowDowngradeTrigger::ProofStale);
        support_row.degraded_label = Some(
            "Spend-receipt proof aged past its freshness bound; export narrowed to Beta until refreshed"
                .to_owned(),
        );
    }

    vec![
        row(
            "recall-row:composer-inline-assist:0001",
            RecallSurface::ComposerInlineAssist,
            "Composer inline assist recall over turn/thread memory, prompt cache, and hybrid retrieval",
            M5AiWorkflowQualificationClass::Stable,
            current_pillars(LocalityClass::LocalOnDevice, RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_REF, false, "composer"),
        ),
        row(
            "recall-row:patch-review:0001",
            RecallSurface::PatchReview,
            "Evidence-rich patch review recall with session-artifact provenance and spend receipts",
            M5AiWorkflowQualificationClass::Stable,
            current_pillars(LocalityClass::LocalOnDevice, RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_REF, false, "patch-review"),
        ),
        row(
            "recall-row:branch-worktree-agent:0001",
            RecallSurface::BranchWorktreeAgent,
            "Side-branch agent recall under cumulative budget ceilings and checkpoint-aware receipts",
            M5AiWorkflowQualificationClass::Beta,
            current_pillars(LocalityClass::ManagedHosted, RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_REF, false, "branch-agent"),
        ),
        row(
            "recall-row:docs-browser-recall:0001",
            RecallSurface::DocsBrowserRecall,
            "Docs and in-app browser recall with provenance, locality cues, and retention posture",
            M5AiWorkflowQualificationClass::Beta,
            current_pillars(LocalityClass::ManagedHosted, RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_REF, false, "docs-browser"),
        ),
        row(
            "recall-row:code-understanding:0001",
            RecallSurface::CodeUnderstanding,
            "Codebase-understanding recall over the workspace graph and reusable semantic memory",
            M5AiWorkflowQualificationClass::Beta,
            current_pillars(LocalityClass::LocalOnDevice, SEMANTIC_RECALL_RECORDS_SCHEMA_REF, false, "code-understanding"),
        ),
        row(
            "recall-row:semantic-hybrid-search:0001",
            RecallSurface::SemanticHybridSearch,
            "Semantic and hybrid search recall blending lexical, graph, and embedding lanes",
            M5AiWorkflowQualificationClass::Stable,
            current_pillars(LocalityClass::MixedLabeled, SEMANTIC_RECALL_RECORDS_SCHEMA_REF, true, "semantic-search"),
        ),
        row(
            "recall-row:managed-offline-report:0001",
            RecallSurface::ManagedOfflineReport,
            "Managed and offline usage/locality reporting with mirror-safe fallback truth",
            M5AiWorkflowQualificationClass::Stable,
            current_pillars(LocalityClass::MirroredOffline, RETRIEVAL_LOCALITY_INSPECTOR_SCHEMA_REF, false, "managed-offline"),
        ),
        support_row,
    ]
}

fn guardrails() -> RowCertGuardrails {
    RowCertGuardrails {
        no_default_cross_scope_recall: true,
        prompt_result_caches_are_not_shadow_telemetry: true,
        mixed_generation_always_labeled: true,
        managed_locality_always_disclosed: true,
        every_durable_artifact_declares_retention_export: true,
        spend_route_failures_keep_precise_fallback: true,
        rows_auto_narrow_on_stale_proof: true,
    }
}

fn consumer_projection() -> RowCertConsumerProjection {
    RowCertConsumerProjection {
        product_ingests_certification: true,
        docs_help_ingests_certification: true,
        diagnostics_ingests_certification: true,
        release_ingests_certification: true,
        narrowed_rows_labeled_below_current: true,
    }
}

fn proof_freshness() -> RowCertProofFreshness {
    RowCertProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_RECALL_ROW_CERTIFICATION_SCHEMA_REF.to_owned(),
        M5_RECALL_ROW_CERTIFICATION_DOC_REF.to_owned(),
        M5_AI_RECALL_MATRIX_SCHEMA_REF.to_owned(),
        M5_AI_RECALL_MATRIX_ARTIFACT_REF.to_owned(),
    ];
    for pillar in CertificationPillar::ALL {
        for schema_ref in pillar.canonical_schema_refs() {
            refs.push((*schema_ref).to_owned());
        }
    }
    // The memory-fence-fallback schema backs the cross-scope fence guardrail.
    refs.push(MEMORY_FENCE_FALLBACK_SCHEMA_REF.to_owned());
    refs
}

fn packet(narrowed_support_export: bool) -> M5RecallRowCertificationPacket {
    M5RecallRowCertificationPacket::new(M5RecallRowCertificationPacketInput {
        packet_id: "m5-recall-row-certification:stable:0001".to_owned(),
        certification_label: "M5 AI/Docs/Recall Row Certification".to_owned(),
        rows: rows(narrowed_support_export),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = match which.as_str() {
        "fixture" => {
            let mut packet = packet(true);
            packet.packet_id = "m5-recall-row-certification:fixture:narrowed:0001".to_owned();
            packet.certification_label =
                "Support-Export Row Narrows On Stale Spend Receipt".to_owned();
            packet
        }
        _ => packet(true),
    };

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
