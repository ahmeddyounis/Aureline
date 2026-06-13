use super::*;

const PACKET_ID: &str = "m5-recall-row-certification:stable:0001";

fn ev(refs: &[&str]) -> Vec<String> {
    refs.iter().map(|r| (*r).to_owned()).collect()
}

fn current_proof(
    pillar: CertificationPillar,
    schema: &str,
    locality: LocalityClass,
    durable: bool,
    mixed: bool,
    evidence: &str,
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
        evidence_refs: ev(&[evidence]),
    }
}

fn current_pillars(
    locality: LocalityClass,
    hybrid_schema: &str,
    hybrid_mixed: bool,
    tag: &str,
) -> Vec<PillarProof> {
    vec![
        current_proof(
            CertificationPillar::MemoryClass,
            MEMORY_CLASS_MATERIALIZATION_SCHEMA_REF,
            locality,
            true,
            false,
            &format!("evidence:memory-class:{tag}"),
        ),
        current_proof(
            CertificationPillar::PromptCacheSessionArtifact,
            SESSION_ARTIFACT_SCHEMA_REF,
            locality,
            true,
            false,
            &format!("evidence:session-artifact:{tag}"),
        ),
        current_proof(
            CertificationPillar::HybridRetrievalLocality,
            hybrid_schema,
            locality,
            false,
            hybrid_mixed,
            &format!("evidence:retrieval-locality:{tag}"),
        ),
        current_proof(
            CertificationPillar::SpendReceipt,
            AI_RUN_RECEIPT_SCHEMA_REF,
            locality,
            true,
            false,
            &format!("evidence:spend-receipt:{tag}"),
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
        evidence_refs: ev(&[&format!("evidence:row:{row_id}")]),
        source_contract_refs: vec![M5_RECALL_ROW_CERTIFICATION_DOC_REF.to_owned()],
    }
}

fn narrowed_support_export_row() -> CertifiedRecallRow {
    let mut pillars = current_pillars(
        LocalityClass::ManagedHosted,
        SEMANTIC_RECALL_RECORDS_SCHEMA_REF,
        false,
        "support-export",
    );
    pillars[3].proof_state = ProofState::Stale;
    let mut support_row = row(
        "recall-row:support-export:0001",
        RecallSurface::SupportExport,
        "Support/export projection of recall provenance, retention, and spend receipts",
        M5AiWorkflowQualificationClass::Stable,
        pillars,
    );
    support_row.effective_qualification = M5AiWorkflowQualificationClass::Beta;
    support_row.narrow_trigger = Some(M5AiWorkflowDowngradeTrigger::ProofStale);
    support_row.degraded_label = Some(
        "Spend-receipt proof aged past its freshness bound; export narrowed to Beta until refreshed"
            .to_owned(),
    );
    support_row
}

fn rows() -> Vec<CertifiedRecallRow> {
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
        narrowed_support_export_row(),
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
    refs.push(MEMORY_FENCE_FALLBACK_SCHEMA_REF.to_owned());
    refs
}

fn packet() -> M5RecallRowCertificationPacket {
    M5RecallRowCertificationPacket::new(M5RecallRowCertificationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        certification_label: "M5 AI/Docs/Recall Row Certification".to_owned(),
        rows: rows(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

#[test]
fn recall_row_certification_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_claimed_surface_is_certified() {
    let packet = packet();
    let surfaces = packet.represented_surfaces();
    for surface in RecallSurface::ALL {
        assert!(
            surfaces.contains(&surface),
            "missing surface: {}",
            surface.as_str()
        );
    }
}

#[test]
fn every_row_covers_all_four_pillars() {
    for row in packet().rows {
        assert!(
            row.covers_all_pillars(),
            "row {} does not cover all pillars",
            row.row_id
        );
    }
}

#[test]
fn missing_surface_fails_validation() {
    let mut packet = packet();
    packet
        .rows
        .retain(|row| row.surface != RecallSurface::SupportExport);
    // Removing the support-export row also removes the only narrowed case.
    let violations = packet.validate();
    assert!(violations.contains(&M5RecallRowCertificationViolation::RequiredSurfaceMissing));
}

#[test]
fn auto_narrow_case_is_present() {
    let packet = packet();
    assert_eq!(packet.narrowed_row_count(), 1);
}

#[test]
fn missing_narrowed_case_fails_validation() {
    let mut packet = packet();
    // Refresh the stale spend-receipt proof on the support-export row and restore
    // its full claim, leaving no demonstrated auto-narrowing case.
    let support_row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == RecallSurface::SupportExport)
        .expect("support-export row");
    for proof in &mut support_row.pillar_proofs {
        proof.proof_state = ProofState::Current;
    }
    support_row.effective_qualification = support_row.claimed_qualification;
    support_row.narrow_trigger = None;
    support_row.degraded_label = None;
    assert!(packet
        .validate()
        .contains(&M5RecallRowCertificationViolation::NarrowedRowCaseMissing));
}

#[test]
fn stale_pillar_without_narrowing_fails() {
    let mut packet = packet();
    // Mark a current row's pillar stale but leave its effective claim untouched.
    let composer_row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == RecallSurface::ComposerInlineAssist)
        .expect("composer row");
    composer_row.pillar_proofs[0].proof_state = ProofState::Stale;
    let violations = packet.validate();
    assert!(violations.contains(&M5RecallRowCertificationViolation::RowNotNarrowedOnStaleProof));
    assert!(
        violations.contains(&M5RecallRowCertificationViolation::NarrowedRowMissingLabelOrTrigger)
    );
}

#[test]
fn missing_pillar_fails_validation() {
    let mut packet = packet();
    packet.rows[0].pillar_proofs.pop();
    assert!(packet
        .validate()
        .contains(&M5RecallRowCertificationViolation::PillarCoverageIncomplete));
}

#[test]
fn pillar_schema_ref_not_admitted_fails() {
    let mut packet = packet();
    packet.rows[0].pillar_proofs[0].canonical_schema_ref = AI_RUN_RECEIPT_SCHEMA_REF.to_owned();
    assert!(packet
        .validate()
        .contains(&M5RecallRowCertificationViolation::PillarSchemaRefNotAdmitted));
}

#[test]
fn undisclosed_managed_locality_fails() {
    let mut packet = packet();
    // The branch-agent row runs managed-hosted; hide its disclosure.
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == RecallSurface::BranchWorktreeAgent)
        .expect("branch agent row");
    row.pillar_proofs[0].locality_disclosed = false;
    assert!(packet
        .validate()
        .contains(&M5RecallRowCertificationViolation::ManagedLocalityNotDisclosed));
}

#[test]
fn unlabeled_mixed_generation_fails() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == RecallSurface::SemanticHybridSearch)
        .expect("semantic search row");
    let hybrid = row
        .pillar_proofs
        .iter_mut()
        .find(|proof| proof.pillar == CertificationPillar::HybridRetrievalLocality)
        .expect("hybrid pillar");
    hybrid.mixed_generation_labeled = false;
    assert!(packet
        .validate()
        .contains(&M5RecallRowCertificationViolation::MixedGenerationNotLabeled));
}

#[test]
fn durable_pillar_without_retention_export_fails() {
    let mut packet = packet();
    // The memory-class pillar is durable; drop its retention/export declaration.
    packet.rows[0].pillar_proofs[0].retention_export_declared = false;
    assert!(packet
        .validate()
        .contains(&M5RecallRowCertificationViolation::DurablePillarMissingRetentionExport));
}

#[test]
fn pillar_without_evidence_fails() {
    let mut packet = packet();
    packet.rows[0].pillar_proofs[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RecallRowCertificationViolation::PillarEvidenceMissing));
}

#[test]
fn missing_pillar_source_schema_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != AI_RUN_RECEIPT_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5RecallRowCertificationViolation::PillarCanonicalSchemaUnbound));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != M5_AI_RECALL_MATRIX_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5RecallRowCertificationViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet.guardrails.rows_auto_narrow_on_stale_proof = false;
    assert!(packet
        .validate()
        .contains(&M5RecallRowCertificationViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet.consumer_projection.docs_help_ingests_certification = false;
    assert!(packet
        .validate()
        .contains(&M5RecallRowCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn incomplete_proof_freshness_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5RecallRowCertificationViolation::ProofFreshnessIncomplete));
}

#[test]
fn generic_degraded_label_fails() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == RecallSurface::SupportExport)
        .expect("support-export row");
    row.degraded_label = Some("unavailable".to_owned());
    assert!(packet
        .validate()
        .contains(&M5RecallRowCertificationViolation::NarrowedRowMissingLabelOrTrigger));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&M5RecallRowCertificationViolation::WrongRecordKind));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5RecallRowCertificationPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 AI/Docs/Recall Row Certification"));
    assert!(summary.contains("composer_inline_assist"));
    assert!(summary.contains("Degraded:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_recall_row_certification_export()
        .expect("checked recall-row certification export validates");
    assert_eq!(checked, packet());
}
