use super::*;

const PACKET_ID: &str = "ai-flow-evidence:m5:0001";
const FLOW_ID: &str = "ai-flow:m5:0001";

fn flow() -> AiFlowBlock {
    AiFlowBlock {
        flow_kind: AiFlowKind::Debug,
        state: AiFlowState::AnswerComposed,
        intent_summary_ref: "ref:intent:debug-retry-storm".to_owned(),
        target_ref: "ref:target:retry-scheduler".to_owned(),
        scope_ref: "ref:scope:retry-module".to_owned(),
        evidence_required_for_claims: true,
        read_only: true,
        apply_handoff_ref: None,
        context_input_refs: vec![
            "ref:context:retry-module".to_owned(),
            "ref:context:retry-incident".to_owned(),
        ],
    }
}

fn evidence_links() -> EvidenceLinkBlock {
    EvidenceLinkBlock {
        evidence_set_id: "evidence-set:m5:0001".to_owned(),
        all_links_resolved: false,
        unresolved_reason_disclosed: true,
        stale_evidence_present: true,
        stale_disclosed: true,
        link_rows: vec![
            EvidenceLinkRow {
                link_id: "link:m5:0001:log".to_owned(),
                evidence_kind: EvidenceKind::Log,
                source_surface: EvidenceSourceSurface::LogStore,
                evidence_ref: "evidence:log:retry-storm".to_owned(),
                freshness: EvidenceFreshnessClass::Fresh,
                provenance: EvidenceProvenanceClass::RecordedRun,
                trust: EvidenceTrustClass::Trusted,
                scope_ref: "ref:scope:retry-module".to_owned(),
                resolved: true,
                disclosed: true,
            },
            EvidenceLinkRow {
                link_id: "link:m5:0001:trace".to_owned(),
                evidence_kind: EvidenceKind::Trace,
                source_surface: EvidenceSourceSurface::TraceStore,
                evidence_ref: "evidence:trace:retry-fanout".to_owned(),
                freshness: EvidenceFreshnessClass::Stale,
                provenance: EvidenceProvenanceClass::RecordedRun,
                trust: EvidenceTrustClass::Trusted,
                scope_ref: "ref:scope:retry-module".to_owned(),
                resolved: true,
                disclosed: true,
            },
            EvidenceLinkRow {
                link_id: "link:m5:0001:profile".to_owned(),
                evidence_kind: EvidenceKind::Profile,
                source_surface: EvidenceSourceSurface::ProfileStore,
                evidence_ref: "evidence:profile:retry-cpu".to_owned(),
                freshness: EvidenceFreshnessClass::Unknown,
                provenance: EvidenceProvenanceClass::ImportedArtifact,
                trust: EvidenceTrustClass::Unverified,
                scope_ref: "ref:scope:retry-module".to_owned(),
                resolved: false,
                disclosed: true,
            },
            EvidenceLinkRow {
                link_id: "link:m5:0001:runbook".to_owned(),
                evidence_kind: EvidenceKind::Runbook,
                source_surface: EvidenceSourceSurface::RunbookRegistry,
                evidence_ref: "evidence:runbook:retry-mitigation".to_owned(),
                freshness: EvidenceFreshnessClass::Aging,
                provenance: EvidenceProvenanceClass::RecordedRun,
                trust: EvidenceTrustClass::Trusted,
                scope_ref: "ref:scope:retry-module".to_owned(),
                resolved: true,
                disclosed: true,
            },
        ],
    }
}

fn findings() -> FlowFindingBlock {
    FlowFindingBlock {
        finding_set_id: "finding-set:m5:0001".to_owned(),
        uncited_findings_count: 1,
        no_authority_beyond_evidence: true,
        produced_before_apply: true,
        finding_rows: vec![
            FlowFindingRow {
                finding_id: "finding:m5:0001:root-cause".to_owned(),
                finding_kind: FlowFindingKind::RootCauseHypothesis,
                confidence: FindingConfidenceClass::Grounded,
                cited_evidence_link_ids: vec![
                    "link:m5:0001:log".to_owned(),
                    "link:m5:0001:trace".to_owned(),
                ],
                evidence_backed: true,
                requires_human_confirmation: false,
                disclosed: true,
            },
            FlowFindingRow {
                finding_id: "finding:m5:0001:repro".to_owned(),
                finding_kind: FlowFindingKind::ReproStep,
                confidence: FindingConfidenceClass::Probable,
                cited_evidence_link_ids: vec!["link:m5:0001:runbook".to_owned()],
                evidence_backed: true,
                requires_human_confirmation: false,
                disclosed: true,
            },
            FlowFindingRow {
                finding_id: "finding:m5:0001:caveat".to_owned(),
                finding_kind: FlowFindingKind::Caveat,
                confidence: FindingConfidenceClass::Speculative,
                cited_evidence_link_ids: vec![],
                evidence_backed: false,
                requires_human_confirmation: true,
                disclosed: true,
            },
        ],
    }
}

fn consumer_surface_parity() -> Vec<FlowSurfaceParityRow> {
    FlowConsumerSurface::ALL
        .into_iter()
        .map(|surface| FlowSurfaceParityRow {
            surface,
            shows_flow: true,
            shows_evidence_links: true,
            shows_findings: true,
            reachable: true,
            qualification: FlowSurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        AI_FLOW_EVIDENCE_DOC_REF.to_owned(),
        AI_FLOW_EVIDENCE_SCHEMA_REF.to_owned(),
        AI_FLOW_EVIDENCE_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
        AI_FLOW_EVIDENCE_PATCH_REVIEW_CONTRACT_REF.to_owned(),
        AI_FLOW_EVIDENCE_M5_MATRIX_CONTRACT_REF.to_owned(),
    ]
}

fn packet_input() -> AiFlowEvidencePacketInput {
    AiFlowEvidencePacketInput {
        packet_id: PACKET_ID.to_owned(),
        flow_id: FLOW_ID.to_owned(),
        display_label: "M5 debug flow for retry-storm incident".to_owned(),
        trust_state_token: "restricted".to_owned(),
        policy_epoch_ref: "policy-epoch:m5:2026-06-01".to_owned(),
        flow: flow(),
        evidence_links: evidence_links(),
        findings: findings(),
        consumer_surface_parity: consumer_surface_parity(),
        downgrade_triggers: FlowDowngradeTrigger::ALL.to_vec(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T10:30:00Z".to_owned(),
    }
}

#[test]
fn packet_constructs_and_serializes() {
    let packet = AiFlowEvidencePacket::new(packet_input());
    let json = packet.export_safe_json();
    assert!(json.contains("ai_explain_debug_test_flows_implementation"));
    assert!(json.contains(PACKET_ID));
}

#[test]
fn valid_packet_passes_validation() {
    let packet = AiFlowEvidencePacket::new(packet_input());
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = AiFlowEvidencePacket::new(packet_input());
    packet.record_kind = "wrong_kind".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::WrongRecordKind));
}

#[test]
fn wrong_schema_version_fails() {
    let mut packet = AiFlowEvidencePacket::new(packet_input());
    packet.schema_version = 999;
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::WrongSchemaVersion));
}

#[test]
fn missing_identity_fails() {
    let mut input = packet_input();
    input.flow_id = "   ".to_owned();
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::MissingIdentity));
}

#[test]
fn missing_source_contracts_fails() {
    let mut input = packet_input();
    input.source_contract_refs = vec![];
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::MissingSourceContracts));
}

#[test]
fn evidence_not_required_for_claims_fails() {
    let mut input = packet_input();
    input.flow.evidence_required_for_claims = false;
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::EvidenceNotRequiredForClaims));
}

#[test]
fn flow_not_read_only_fails() {
    let mut input = packet_input();
    input.flow.read_only = false;
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::FlowNotReadOnly));
}

#[test]
fn flow_incomplete_fails() {
    let mut input = packet_input();
    input.flow.target_ref = String::new();
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::FlowIncomplete));
}

#[test]
fn evidence_set_empty_fails() {
    let mut input = packet_input();
    input.evidence_links.link_rows = vec![];
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::EvidenceSetEmpty));
}

#[test]
fn hidden_evidence_link_fails() {
    let mut input = packet_input();
    input.evidence_links.link_rows[0].disclosed = false;
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::HiddenEvidenceLink));
}

#[test]
fn evidence_link_incomplete_fails() {
    let mut input = packet_input();
    input.evidence_links.link_rows[0].evidence_ref = String::new();
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::EvidenceLinkIncomplete));
}

#[test]
fn unresolved_evidence_undisclosed_fails() {
    let mut input = packet_input();
    // A link stays unresolved but the gap is not disclosed.
    input.evidence_links.unresolved_reason_disclosed = false;
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::UnresolvedEvidenceUndisclosed));
}

#[test]
fn stale_evidence_undisclosed_fails() {
    let mut input = packet_input();
    // Stale evidence present but staleness not disclosed.
    input.evidence_links.stale_disclosed = false;
    input.evidence_links.stale_evidence_present = false;
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::StaleEvidenceUndisclosed));
}

#[test]
fn finding_set_empty_fails() {
    let mut input = packet_input();
    input.findings.finding_rows = vec![];
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::FindingSetEmpty));
}

#[test]
fn hidden_finding_fails() {
    let mut input = packet_input();
    input.findings.finding_rows[0].disclosed = false;
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::HiddenFinding));
}

#[test]
fn dangling_evidence_citation_fails() {
    let mut input = packet_input();
    input.findings.finding_rows[0].cited_evidence_link_ids = vec!["link:does-not-exist".to_owned()];
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::DanglingEvidenceCitation));
}

#[test]
fn evidence_backed_flag_mismatch_fails() {
    let mut input = packet_input();
    input.findings.finding_rows[0].evidence_backed = false;
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::EvidenceBackedFlagMismatch));
}

#[test]
fn uncited_finding_not_flagged_fails() {
    let mut input = packet_input();
    input.findings.finding_rows[2].requires_human_confirmation = false;
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::UncitedFindingNotFlagged));
}

#[test]
fn uncited_count_mismatch_fails() {
    let mut input = packet_input();
    input.findings.uncited_findings_count = 0;
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::UncitedCountMismatch));
}

#[test]
fn authority_beyond_evidence_fails() {
    let mut input = packet_input();
    input.findings.no_authority_beyond_evidence = false;
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::AuthorityBeyondEvidence));
}

#[test]
fn findings_not_produced_before_apply_fails() {
    let mut input = packet_input();
    input.findings.produced_before_apply = false;
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::FindingsNotProducedBeforeApply));
}

#[test]
fn consumer_surface_coverage_missing_fails() {
    let mut input = packet_input();
    input.consumer_surface_parity = vec![input.consumer_surface_parity[0].clone()];
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::ConsumerSurfaceCoverageMissing));
}

#[test]
fn stable_claim_not_qualified_fails() {
    let mut input = packet_input();
    input.consumer_surface_parity[0].claimed_stable = true;
    input.consumer_surface_parity[0].reachable = false;
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::StableClaimNotQualified));
}

#[test]
fn raw_boundary_material_detected() {
    let mut input = packet_input();
    input.evidence_links.link_rows[0].evidence_ref =
        "https://internal.aureline.dev/log-secret".to_owned();
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiFlowEvidenceViolation::RawBoundaryMaterialInExport));
}

#[test]
fn test_flow_apply_handoff_is_accepted() {
    let mut input = packet_input();
    input.flow.flow_kind = AiFlowKind::Test;
    input.flow.state = AiFlowState::HandedToApply;
    input.flow.apply_handoff_ref = Some("ai-patch-review:evidence-rich:m5:0007#diff".to_owned());
    let packet = AiFlowEvidencePacket::new(input);
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn evidence_source_surface_round_trips_kind() {
    for kind in EvidenceKind::ALL {
        assert_eq!(kind.canonical_source().evidence_kind(), kind);
    }
}

#[test]
fn markdown_summary_renders() {
    let packet = AiFlowEvidencePacket::new(packet_input());
    let md = packet.render_markdown_summary();
    assert!(md.starts_with("# AI Explain/Debug/Test Flows"));
    assert!(md.contains(PACKET_ID));
    assert!(md.contains(FLOW_ID));
}

#[test]
fn checked_in_export_loads_and_validates() {
    let result = current_stable_ai_flow_evidence_export();
    assert!(
        result.is_ok(),
        "checked-in export should load and validate: {:?}",
        result
    );
}
