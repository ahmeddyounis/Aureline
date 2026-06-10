use super::*;

const PACKET_ID: &str = "ai-refactor-planner:m5:0001";
const PLAN_ID: &str = "ai-refactor-plan:m5:0001";

fn plan() -> RefactorPlanBlock {
    RefactorPlanBlock {
        refactor_kind: RefactorKind::RenameSymbol,
        state: RefactorPlanState::CandidatesPreviewed,
        intent_summary_ref: "ref:intent:rename-retry-budget".to_owned(),
        target_symbol_ref: "ref:symbol:retry-budget".to_owned(),
        scope_ref: "ref:scope:retry-module".to_owned(),
        impact_set_complete: true,
        preview_required_before_apply: true,
        context_input_refs: vec![
            "ref:context:retry-module".to_owned(),
            "ref:context:retry-tests".to_owned(),
        ],
    }
}

fn impact_set() -> ImpactSetBlock {
    ImpactSetBlock {
        impact_set_id: "impact-set:m5:0001".to_owned(),
        highest_safety_class: MultiFileSafetyClass::SemanticCrossBoundary,
        analysis_complete: true,
        partial_reason_disclosed: false,
        cross_boundary_present: true,
        site_rows: vec![
            ImpactSiteRow {
                site_id: "site:m5:0001:def".to_owned(),
                file_ref: "file:retry-module".to_owned(),
                site_class: ImpactSiteClass::DefinitionSite,
                confidence: ImpactConfidenceClass::Resolved,
                safety_class: MultiFileSafetyClass::MechanicalMultiFile,
                included_in_candidate: true,
                disclosed: true,
            },
            ImpactSiteRow {
                site_id: "site:m5:0001:ref".to_owned(),
                file_ref: "file:retry-callers".to_owned(),
                site_class: ImpactSiteClass::CrossCrateBoundary,
                confidence: ImpactConfidenceClass::Resolved,
                safety_class: MultiFileSafetyClass::SemanticCrossBoundary,
                included_in_candidate: true,
                disclosed: true,
            },
            ImpactSiteRow {
                site_id: "site:m5:0001:dyn".to_owned(),
                file_ref: "file:retry-config".to_owned(),
                site_class: ImpactSiteClass::DynamicOrReflective,
                confidence: ImpactConfidenceClass::Ambiguous,
                safety_class: MultiFileSafetyClass::SemanticLocal,
                included_in_candidate: false,
                disclosed: true,
            },
        ],
    }
}

fn candidate_preview() -> CandidatePreviewBlock {
    CandidatePreviewBlock {
        preview_id: "preview:m5:0001".to_owned(),
        selected_candidate_id: Some("candidate:m5:0001:mechanical".to_owned()),
        candidate_rows: vec![
            CandidateRow {
                candidate_id: "candidate:m5:0001:mechanical".to_owned(),
                safety_class: MultiFileSafetyClass::MechanicalMultiFile,
                state: CandidateState::Selected,
                file_count: 3,
                diff_packet_ref: "ai-patch-review:evidence-rich:m5:0001#diff".to_owned(),
                validation_receipt_ref: "ai-patch-review:evidence-rich:m5:0001#validation"
                    .to_owned(),
                rollback_handle_ref: "ai-patch-review:evidence-rich:m5:0001#rollback".to_owned(),
                review_required_before_apply: true,
                auto_apply_blocked_for_unsafe_class: false,
            },
            CandidateRow {
                candidate_id: "candidate:m5:0001:cross-boundary".to_owned(),
                safety_class: MultiFileSafetyClass::SemanticCrossBoundary,
                state: CandidateState::Previewed,
                file_count: 5,
                diff_packet_ref: "ai-patch-review:evidence-rich:m5:0002#diff".to_owned(),
                validation_receipt_ref: "ai-patch-review:evidence-rich:m5:0002#validation"
                    .to_owned(),
                rollback_handle_ref: "ai-patch-review:evidence-rich:m5:0002#rollback".to_owned(),
                review_required_before_apply: true,
                auto_apply_blocked_for_unsafe_class: true,
            },
        ],
        compare_to_base_available: true,
        produced_before_apply: true,
    }
}

fn consumer_surface_parity() -> Vec<RefactorSurfaceParityRow> {
    RefactorConsumerSurface::ALL
        .into_iter()
        .map(|surface| RefactorSurfaceParityRow {
            surface,
            shows_plan: true,
            shows_impact_set: true,
            shows_candidates: true,
            reachable: true,
            qualification: RefactorSurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        REFACTOR_PLANNER_DOC_REF.to_owned(),
        REFACTOR_PLANNER_SCHEMA_REF.to_owned(),
        REFACTOR_PLANNER_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
        REFACTOR_PLANNER_EVIDENCE_CONTRACT_REF.to_owned(),
        REFACTOR_PLANNER_M5_MATRIX_CONTRACT_REF.to_owned(),
    ]
}

fn packet_input() -> RefactorPlannerPacketInput {
    RefactorPlannerPacketInput {
        packet_id: PACKET_ID.to_owned(),
        plan_id: PLAN_ID.to_owned(),
        display_label: "M5 refactor planner for retry-budget rename".to_owned(),
        trust_state_token: "restricted".to_owned(),
        policy_epoch_ref: "policy-epoch:m5:2026-06-01".to_owned(),
        plan: plan(),
        impact_set: impact_set(),
        candidate_preview: candidate_preview(),
        consumer_surface_parity: consumer_surface_parity(),
        downgrade_triggers: RefactorDowngradeTrigger::ALL.to_vec(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T10:05:00Z".to_owned(),
    }
}

#[test]
fn packet_constructs_and_serializes() {
    let packet = RefactorPlannerPacket::new(packet_input());
    let json = packet.export_safe_json();
    assert!(json.contains("ai_refactor_planner_implementation"));
    assert!(json.contains(PACKET_ID));
}

#[test]
fn valid_packet_passes_validation() {
    let packet = RefactorPlannerPacket::new(packet_input());
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = RefactorPlannerPacket::new(packet_input());
    packet.record_kind = "wrong_kind".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::WrongRecordKind));
}

#[test]
fn wrong_schema_version_fails() {
    let mut packet = RefactorPlannerPacket::new(packet_input());
    packet.schema_version = 999;
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::WrongSchemaVersion));
}

#[test]
fn missing_identity_fails() {
    let mut input = packet_input();
    input.plan_id = "   ".to_owned();
    let packet = RefactorPlannerPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::MissingIdentity));
}

#[test]
fn missing_source_contracts_fails() {
    let mut input = packet_input();
    input.source_contract_refs = vec![];
    let packet = RefactorPlannerPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::MissingSourceContracts));
}

#[test]
fn preview_not_required_before_apply_fails() {
    let mut input = packet_input();
    input.plan.preview_required_before_apply = false;
    let packet = RefactorPlannerPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::PreviewNotRequiredBeforeApply));
}

#[test]
fn impact_completeness_mismatch_fails() {
    let mut input = packet_input();
    input.plan.impact_set_complete = false;
    let packet = RefactorPlannerPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::ImpactCompletenessMismatch));
}

#[test]
fn impact_set_gap_undisclosed_fails() {
    let mut input = packet_input();
    input.plan.impact_set_complete = false;
    input.impact_set.analysis_complete = false;
    input.impact_set.partial_reason_disclosed = false;
    let packet = RefactorPlannerPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::ImpactSetGapUndisclosed));
}

#[test]
fn hidden_impact_site_fails() {
    let mut input = packet_input();
    input.impact_set.site_rows[1].disclosed = false;
    let packet = RefactorPlannerPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::HiddenImpactSite));
}

#[test]
fn worst_case_safety_understated_fails() {
    let mut input = packet_input();
    input.impact_set.highest_safety_class = MultiFileSafetyClass::MechanicalSingleFile;
    let packet = RefactorPlannerPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::WorstCaseSafetyUnderstated));
}

#[test]
fn candidate_incomplete_fails() {
    let mut input = packet_input();
    input.candidate_preview.candidate_rows[0].diff_packet_ref = String::new();
    let packet = RefactorPlannerPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::CandidateIncomplete));
}

#[test]
fn candidate_review_not_required_fails() {
    let mut input = packet_input();
    input.candidate_preview.candidate_rows[0].review_required_before_apply = false;
    let packet = RefactorPlannerPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::CandidateReviewNotRequired));
}

#[test]
fn unsafe_candidate_not_blocked_fails() {
    let mut input = packet_input();
    input.candidate_preview.candidate_rows[1].auto_apply_blocked_for_unsafe_class = false;
    let packet = RefactorPlannerPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::UnsafeCandidateNotBlocked));
}

#[test]
fn candidates_not_produced_before_apply_fails() {
    let mut input = packet_input();
    input.candidate_preview.produced_before_apply = false;
    let packet = RefactorPlannerPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::CandidatesNotProducedBeforeApply));
}

#[test]
fn selected_candidate_missing_fails() {
    let mut input = packet_input();
    input.candidate_preview.selected_candidate_id = Some("candidate:does-not-exist".to_owned());
    let packet = RefactorPlannerPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::SelectedCandidateMissing));
}

#[test]
fn consumer_surface_coverage_missing_fails() {
    let mut input = packet_input();
    input.consumer_surface_parity = vec![input.consumer_surface_parity[0].clone()];
    let packet = RefactorPlannerPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::ConsumerSurfaceCoverageMissing));
}

#[test]
fn stable_claim_not_qualified_fails() {
    let mut input = packet_input();
    input.consumer_surface_parity[0].claimed_stable = true;
    input.consumer_surface_parity[0].reachable = false;
    let packet = RefactorPlannerPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::StableClaimNotQualified));
}

#[test]
fn raw_boundary_material_detected() {
    let mut input = packet_input();
    input.candidate_preview.candidate_rows[0].diff_packet_ref =
        "https://internal.aureline.dev/diff-secret".to_owned();
    let packet = RefactorPlannerPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&RefactorPlannerViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_renders() {
    let packet = RefactorPlannerPacket::new(packet_input());
    let md = packet.render_markdown_summary();
    assert!(md.starts_with("# AI Refactor Planner"));
    assert!(md.contains(PACKET_ID));
    assert!(md.contains(PLAN_ID));
}

#[test]
fn checked_in_export_loads_and_validates() {
    let result = current_stable_refactor_planner_export();
    assert!(
        result.is_ok(),
        "checked-in export should load and validate: {:?}",
        result
    );
}
