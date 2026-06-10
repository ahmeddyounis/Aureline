use super::*;

const PACKET_ID: &str = "m5-docs-recall-matrix:stable:0001";

fn lane_rows() -> Vec<M5DocsRecallMatrixLaneRow> {
    vec![
        M5DocsRecallMatrixLaneRow {
            lane: M5DocsRecallLane::DocsSemanticRecall,
            qualification: M5DocsRecallQualificationClass::Stable,
            scope_summary: "Mirror-aware docs semantic recall with source/version/freshness truth and explicit ranking reasons; pinned signed mirrors outrank live vendor docs".to_owned(),
            evidence_requirement: M5DocsRecallEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:semantic-recall-boundary-truth:m5".to_owned(),
                "evidence:docs-pack-mirror-lineage:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5DocsRecallDowngradeTrigger::ProofStale,
                M5DocsRecallDowngradeTrigger::MirrorOffline,
                M5DocsRecallDowngradeTrigger::SourceVersionMismatch,
                M5DocsRecallDowngradeTrigger::FreshnessExpired,
            ],
            rollback_posture: M5DocsRecallRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_DOCS_RECALL_MATRIX_DOCS_RECALL_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5DocsRecallConsumerSurface::DocsBrowser,
                M5DocsRecallConsumerSurface::RetrievalDebugInspector,
                M5DocsRecallConsumerSurface::CliHeadless,
                M5DocsRecallConsumerSurface::SupportExport,
            ],
        },
        M5DocsRecallMatrixLaneRow {
            lane: M5DocsRecallLane::CodebaseExplainer,
            qualification: M5DocsRecallQualificationClass::Stable,
            scope_summary: "Cited codebase explainers that preserve source class and confidence with open-raw/open-source escapes; no heuristic claim is presented as a verified graph fact".to_owned(),
            evidence_requirement: M5DocsRecallEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:codebase-explainer-citation-conformance:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5DocsRecallDowngradeTrigger::ProofStale,
                M5DocsRecallDowngradeTrigger::SourceVersionMismatch,
                M5DocsRecallDowngradeTrigger::TrustNarrowing,
                M5DocsRecallDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5DocsRecallRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_DOCS_RECALL_MATRIX_CODE_EXPLAINER_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5DocsRecallConsumerSurface::CodeExplainerPanel,
                M5DocsRecallConsumerSurface::RetrievalDebugInspector,
                M5DocsRecallConsumerSurface::CliHeadless,
                M5DocsRecallConsumerSurface::SupportExport,
            ],
        },
        M5DocsRecallMatrixLaneRow {
            lane: M5DocsRecallLane::RetrievalDebug,
            qualification: M5DocsRecallQualificationClass::Stable,
            scope_summary: "Retrieval-debug inspector exposing ranking reasons, recall lane, locality, and confidence for every docs/code recall result without leaking raw query text or provider payloads".to_owned(),
            evidence_requirement: M5DocsRecallEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:retrieval-inspector-conformance:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5DocsRecallDowngradeTrigger::ProofStale,
                M5DocsRecallDowngradeTrigger::PolicyBlocked,
                M5DocsRecallDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5DocsRecallRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_DOCS_RECALL_MATRIX_RETRIEVAL_DEBUG_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5DocsRecallConsumerSurface::RetrievalDebugInspector,
                M5DocsRecallConsumerSurface::CliHeadless,
                M5DocsRecallConsumerSurface::SupportExport,
                M5DocsRecallConsumerSurface::Diagnostics,
            ],
        },
        M5DocsRecallMatrixLaneRow {
            lane: M5DocsRecallLane::ScopedBrowserSurface,
            qualification: M5DocsRecallQualificationClass::Beta,
            scope_summary: "Narrow, attributable docs/review/light-edit browser surfaces with explicit captured-vs-live state and return-path-safe handoffs; general web-mode and browser-runtime claims stay out of scope".to_owned(),
            evidence_requirement: M5DocsRecallEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:docs-browser-truth-packet:m5".to_owned(),
                "evidence:browser-companion-handoff-eligibility:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5DocsRecallDowngradeTrigger::ProofStale,
                M5DocsRecallDowngradeTrigger::PolicyBlocked,
                M5DocsRecallDowngradeTrigger::TrustNarrowing,
                M5DocsRecallDowngradeTrigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: M5DocsRecallRollbackPosture::ReturnPathPreserved,
            source_contract_refs: vec![
                M5_DOCS_RECALL_MATRIX_BROWSER_SURFACE_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5DocsRecallConsumerSurface::DocsBrowser,
                M5DocsRecallConsumerSurface::BrowserCompanion,
                M5DocsRecallConsumerSurface::SupportExport,
                M5DocsRecallConsumerSurface::HelpAbout,
            ],
        },
    ]
}

fn trust_review() -> M5DocsRecallMatrixTrustReview {
    M5DocsRecallMatrixTrustReview {
        docs_recall_mirror_aware: true,
        explainers_cited_with_source_class: true,
        confidence_class_preserved: true,
        open_raw_open_source_escape_preserved: true,
        ranking_reasons_explicit: true,
        retrieval_debug_available: true,
        browser_surface_narrow_and_attributable: true,
        browser_handoff_return_path_safe: true,
        no_source_looks_more_authoritative_than_proven: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5DocsRecallMatrixConsumerProjection {
    M5DocsRecallMatrixConsumerProjection {
        docs_browser_shows_provenance_and_freshness: true,
        code_explainer_shows_source_class_and_confidence: true,
        retrieval_debug_shows_ranking_reasons: true,
        browser_companion_shows_captured_vs_live: true,
        cli_headless_shows_qualification: true,
        support_export_shows_qualification: true,
        diagnostics_shows_qualification: true,
        help_about_shows_qualification: true,
        preview_labs_label_for_unqualified_lanes: true,
    }
}

fn proof_freshness() -> M5DocsRecallMatrixProofFreshness {
    M5DocsRecallMatrixProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_DOCS_RECALL_MATRIX_SCHEMA_REF.to_owned(),
        M5_DOCS_RECALL_MATRIX_DOC_REF.to_owned(),
        M5_DOCS_RECALL_MATRIX_DOCS_RECALL_CONTRACT_REF.to_owned(),
        M5_DOCS_RECALL_MATRIX_CODE_EXPLAINER_CONTRACT_REF.to_owned(),
        M5_DOCS_RECALL_MATRIX_RETRIEVAL_DEBUG_CONTRACT_REF.to_owned(),
        M5_DOCS_RECALL_MATRIX_BROWSER_SURFACE_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> M5DocsRecallMatrixPacket {
    M5DocsRecallMatrixPacket::new(M5DocsRecallMatrixPacketInput {
        packet_id: PACKET_ID.to_owned(),
        matrix_label: "M5 Docs and Code-Recall Matrix".to_owned(),
        lane_rows: lane_rows(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

#[test]
fn m5_docs_recall_matrix_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_lane_fails_validation() {
    let mut packet = packet();
    packet
        .lane_rows
        .retain(|row| row.lane != M5DocsRecallLane::RetrievalDebug);
    assert!(packet
        .validate()
        .contains(&M5DocsRecallMatrixViolation::RequiredLaneMissing));
}

#[test]
fn stable_lane_missing_evidence_fails() {
    let mut packet = packet();
    packet.lane_rows[0].required_evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsRecallMatrixViolation::StableLaneMissingEvidence));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.lane_rows[1].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsRecallMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.lane_rows[2].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsRecallMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsRecallMatrixViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.browser_handoff_return_path_safe = false;
    assert!(packet
        .validate()
        .contains(&M5DocsRecallMatrixViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .preview_labs_label_for_unqualified_lanes = false;
    assert!(packet
        .validate()
        .contains(&M5DocsRecallMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DocsRecallMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_lane() {
    let summary = packet().render_markdown_summary();
    for lane in M5DocsRecallLane::ALL {
        assert!(
            summary.contains(lane.as_str()),
            "summary missing lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_docs_and_code_recall_matrix_export()
        .expect("checked M5 docs recall matrix export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract/mirror_offline_recall_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract/browser_surface_held.json"
        )),
    ] {
        let packet: M5DocsRecallMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
