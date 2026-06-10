use super::*;

const PACKET_ID: &str = "ai-review-findings:m5:0001";
const REVIEW_PASS_ID: &str = "ai-review-pass:m5:0001";

fn anchor(id: &str, strategy: AnchorStrategy, state: AnchorState) -> DurableAnchor {
    let disturbed = state.is_disturbed();
    DurableAnchor {
        anchor_id: id.to_owned(),
        strategy,
        target_ref: "ref:target:retry-scheduler-fn".to_owned(),
        scope_ref: "ref:scope:retry-module".to_owned(),
        state,
        drift_detected: disturbed,
        rebind_disclosed: disturbed,
        durable: true,
    }
}

fn findings() -> ReviewFindingsBlock {
    ReviewFindingsBlock {
        finding_set_id: "review-finding-set:m5:0001".to_owned(),
        uncited_findings_count: 1,
        no_authority_beyond_evidence: true,
        produced_before_apply: true,
        finding_rows: vec![
            ReviewFindingRow {
                finding_id: "finding:m5:0001:unbounded-retry".to_owned(),
                finding_class: ReviewFindingClass::BugRisk,
                severity: FindingSeverityClass::Blocker,
                confidence: FindingConfidenceClass::Grounded,
                anchor: anchor(
                    "anchor:m5:0001:retry-loop",
                    AnchorStrategy::SymbolPath,
                    AnchorState::Bound,
                ),
                cited_evidence_refs: vec![
                    "evidence:trace:retry-fanout".to_owned(),
                    "evidence:log:retry-storm".to_owned(),
                ],
                evidence_backed: true,
                resolution_state: FindingResolutionState::Open,
                requires_human_confirmation: false,
                disclosed: true,
            },
            ReviewFindingRow {
                finding_id: "finding:m5:0001:missing-test".to_owned(),
                finding_class: ReviewFindingClass::TestGap,
                severity: FindingSeverityClass::Major,
                confidence: FindingConfidenceClass::Probable,
                anchor: anchor(
                    "anchor:m5:0001:retry-test",
                    AnchorStrategy::StructuralNode,
                    AnchorState::Rebound,
                ),
                cited_evidence_refs: vec!["evidence:coverage:retry-module".to_owned()],
                evidence_backed: true,
                resolution_state: FindingResolutionState::Acknowledged,
                requires_human_confirmation: false,
                disclosed: true,
            },
            ReviewFindingRow {
                finding_id: "finding:m5:0001:naming-nit".to_owned(),
                finding_class: ReviewFindingClass::Style,
                severity: FindingSeverityClass::Nit,
                confidence: FindingConfidenceClass::Speculative,
                anchor: anchor(
                    "anchor:m5:0001:naming",
                    AnchorStrategy::LineRange,
                    AnchorState::Drifted,
                ),
                cited_evidence_refs: vec![],
                evidence_backed: false,
                resolution_state: FindingResolutionState::Deferred,
                requires_human_confirmation: true,
                disclosed: true,
            },
        ],
    }
}

fn ownership_hints() -> OwnershipHintsBlock {
    OwnershipHintsBlock {
        hint_set_id: "ownership-hint-set:m5:0001".to_owned(),
        hints_are_advisory: true,
        no_auto_assignment: true,
        hint_rows: vec![
            OwnershipHintRow {
                hint_id: "hint:m5:0001:retry-owner".to_owned(),
                ownership_source: OwnershipSource::CodeownersFile,
                owner_ref: "ref:owner:scheduling-team".to_owned(),
                confidence: OwnershipConfidenceClass::Strong,
                suggested_reviewer: true,
                scope_ref: "ref:scope:retry-module".to_owned(),
                advisory: true,
                disclosed: true,
            },
            OwnershipHintRow {
                hint_id: "hint:m5:0001:recent-author".to_owned(),
                ownership_source: OwnershipSource::CommitHistory,
                owner_ref: "ref:owner:recent-author-1".to_owned(),
                confidence: OwnershipConfidenceClass::Moderate,
                suggested_reviewer: true,
                scope_ref: "ref:scope:retry-module".to_owned(),
                advisory: true,
                disclosed: true,
            },
        ],
    }
}

fn workspace_integration() -> ReviewWorkspaceIntegrationBlock {
    ReviewWorkspaceIntegrationBlock {
        workspace_id: "review-workspace:m5:0001".to_owned(),
        publish_state: PublishState::PublishedToReview,
        publish_destination: PublishDestination::ReviewPack,
        human_gate_required: true,
        human_gated: true,
        review_pack_digest_ref: "ref:review-pack-digest:m5:0001".to_owned(),
        evidence_packet_lineage_refs: vec![
            "ref:evidence-packet:m5:0007".to_owned(),
            "ref:evidence-packet:m5:0008".to_owned(),
        ],
        published_finding_ids: vec![
            "finding:m5:0001:unbounded-retry".to_owned(),
            "finding:m5:0001:missing-test".to_owned(),
        ],
    }
}

fn consumer_surface_parity() -> Vec<ReviewSurfaceParityRow> {
    ReviewConsumerSurface::ALL
        .into_iter()
        .map(|surface| ReviewSurfaceParityRow {
            surface,
            shows_findings: true,
            shows_anchors: true,
            shows_ownership_hints: true,
            reachable: true,
            qualification: ReviewSurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        AI_REVIEW_FINDINGS_DOC_REF.to_owned(),
        AI_REVIEW_FINDINGS_SCHEMA_REF.to_owned(),
        AI_REVIEW_FINDINGS_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
        AI_REVIEW_FINDINGS_REVIEW_ASSIST_CONTRACT_REF.to_owned(),
        AI_REVIEW_FINDINGS_REVIEW_PACK_CONTRACT_REF.to_owned(),
        AI_REVIEW_FINDINGS_EVIDENCE_PACKET_CONTRACT_REF.to_owned(),
        AI_REVIEW_FINDINGS_M5_MATRIX_CONTRACT_REF.to_owned(),
    ]
}

fn packet_input() -> AiReviewFindingsPacketInput {
    AiReviewFindingsPacketInput {
        packet_id: PACKET_ID.to_owned(),
        review_pass_id: REVIEW_PASS_ID.to_owned(),
        display_label: "M5 AI review pass for retry-storm change".to_owned(),
        trust_state_token: "restricted".to_owned(),
        policy_epoch_ref: "policy-epoch:m5:2026-06-01".to_owned(),
        findings: findings(),
        ownership_hints: ownership_hints(),
        workspace_integration: workspace_integration(),
        consumer_surface_parity: consumer_surface_parity(),
        downgrade_triggers: ReviewDowngradeTrigger::ALL.to_vec(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T10:54:00Z".to_owned(),
    }
}

#[test]
fn packet_constructs_and_serializes() {
    let packet = AiReviewFindingsPacket::new(packet_input());
    let json = packet.export_safe_json();
    assert!(json.contains("ai_review_findings_ownership_anchors_implementation"));
    assert!(json.contains(PACKET_ID));
}

#[test]
fn valid_packet_passes_validation() {
    let packet = AiReviewFindingsPacket::new(packet_input());
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = AiReviewFindingsPacket::new(packet_input());
    packet.record_kind = "wrong_kind".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::WrongRecordKind));
}

#[test]
fn wrong_schema_version_fails() {
    let mut packet = AiReviewFindingsPacket::new(packet_input());
    packet.schema_version = 999;
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::WrongSchemaVersion));
}

#[test]
fn missing_identity_fails() {
    let mut input = packet_input();
    input.review_pass_id = "   ".to_owned();
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::MissingIdentity));
}

#[test]
fn missing_source_contracts_fails() {
    let mut input = packet_input();
    input.source_contract_refs = vec![];
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::MissingSourceContracts));
}

#[test]
fn finding_set_empty_fails() {
    let mut input = packet_input();
    input.findings.finding_rows = vec![];
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::FindingSetEmpty));
}

#[test]
fn hidden_finding_fails() {
    let mut input = packet_input();
    input.findings.finding_rows[0].disclosed = false;
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::HiddenFinding));
}

#[test]
fn evidence_backed_flag_mismatch_fails() {
    let mut input = packet_input();
    input.findings.finding_rows[0].evidence_backed = false;
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::EvidenceBackedFlagMismatch));
}

#[test]
fn uncited_finding_not_flagged_fails() {
    let mut input = packet_input();
    input.findings.finding_rows[2].requires_human_confirmation = false;
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::UncitedFindingNotFlagged));
}

#[test]
fn uncited_count_mismatch_fails() {
    let mut input = packet_input();
    input.findings.uncited_findings_count = 0;
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::UncitedCountMismatch));
}

#[test]
fn authority_beyond_evidence_fails() {
    let mut input = packet_input();
    input.findings.no_authority_beyond_evidence = false;
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::AuthorityBeyondEvidence));
}

#[test]
fn findings_not_produced_before_apply_fails() {
    let mut input = packet_input();
    input.findings.produced_before_apply = false;
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::FindingsNotProducedBeforeApply));
}

#[test]
fn anchor_incomplete_fails() {
    let mut input = packet_input();
    input.findings.finding_rows[0].anchor.target_ref = String::new();
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::AnchorIncomplete));
}

#[test]
fn anchor_not_durable_fails() {
    let mut input = packet_input();
    input.findings.finding_rows[0].anchor.durable = false;
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::AnchorNotDurable));
}

#[test]
fn anchor_drift_undisclosed_fails() {
    let mut input = packet_input();
    // The drifted anchor leaves its rebind disposition undisclosed.
    input.findings.finding_rows[2].anchor.rebind_disclosed = false;
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::AnchorDriftUndisclosed));
}

#[test]
fn ownership_hints_not_advisory_fails() {
    let mut input = packet_input();
    input.ownership_hints.hints_are_advisory = false;
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::OwnershipHintsNotAdvisory));
}

#[test]
fn ownership_auto_assignment_allowed_fails() {
    let mut input = packet_input();
    input.ownership_hints.no_auto_assignment = false;
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::OwnershipAutoAssignmentAllowed));
}

#[test]
fn ownership_hint_incomplete_fails() {
    let mut input = packet_input();
    input.ownership_hints.hint_rows[0].owner_ref = String::new();
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::OwnershipHintIncomplete));
}

#[test]
fn hidden_ownership_hint_fails() {
    let mut input = packet_input();
    input.ownership_hints.hint_rows[0].disclosed = false;
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::HiddenOwnershipHint));
}

#[test]
fn workspace_integration_incomplete_fails() {
    let mut input = packet_input();
    input.workspace_integration.review_pack_digest_ref = String::new();
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::WorkspaceIntegrationIncomplete));
}

#[test]
fn human_gate_not_required_fails() {
    let mut input = packet_input();
    input.workspace_integration.human_gate_required = false;
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::HumanGateNotRequired));
}

#[test]
fn published_without_human_gate_fails() {
    let mut input = packet_input();
    input.workspace_integration.human_gated = false;
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::PublishedWithoutHumanGate));
}

#[test]
fn dangling_published_finding_fails() {
    let mut input = packet_input();
    input.workspace_integration.published_finding_ids = vec!["finding:does-not-exist".to_owned()];
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::DanglingPublishedFinding));
}

#[test]
fn consumer_surface_coverage_missing_fails() {
    let mut input = packet_input();
    input.consumer_surface_parity = vec![input.consumer_surface_parity[0].clone()];
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::ConsumerSurfaceCoverageMissing));
}

#[test]
fn stable_claim_not_qualified_fails() {
    let mut input = packet_input();
    input.consumer_surface_parity[0].claimed_stable = true;
    input.consumer_surface_parity[0].reachable = false;
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::StableClaimNotQualified));
}

#[test]
fn raw_boundary_material_detected() {
    let mut input = packet_input();
    input.ownership_hints.hint_rows[0].owner_ref = "owner@internal.aureline.dev".to_owned();
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&AiReviewFindingsViolation::RawBoundaryMaterialInExport));
}

#[test]
fn draft_publish_state_needs_no_human_gate() {
    let mut input = packet_input();
    input.workspace_integration.publish_state = PublishState::Draft;
    input.workspace_integration.human_gated = false;
    input.workspace_integration.published_finding_ids = vec![];
    let packet = AiReviewFindingsPacket::new(input);
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn markdown_summary_renders() {
    let packet = AiReviewFindingsPacket::new(packet_input());
    let md = packet.render_markdown_summary();
    assert!(md.starts_with("# AI Review Findings, Ownership Hints, and Durable Anchors"));
    assert!(md.contains(PACKET_ID));
    assert!(md.contains(REVIEW_PASS_ID));
}

#[test]
fn checked_in_export_loads_and_validates() {
    let result = current_stable_ai_review_findings_export();
    assert!(
        result.is_ok(),
        "checked-in export should load and validate: {:?}",
        result
    );
}
