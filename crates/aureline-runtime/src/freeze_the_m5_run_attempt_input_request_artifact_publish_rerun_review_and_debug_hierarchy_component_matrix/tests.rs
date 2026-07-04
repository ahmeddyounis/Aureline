use super::*;

fn packet() -> ExecutionLifecycleComponentMatrix {
    seeded_execution_lifecycle_component_matrix()
}

fn row_mut<'a>(
    packet: &'a mut ExecutionLifecycleComponentMatrix,
    component_id: &str,
) -> &'a mut ComponentRow {
    packet
        .components
        .iter_mut()
        .find(|r| r.component_id == component_id)
        .unwrap_or_else(|| panic!("component {component_id}"))
}

#[test]
fn packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_family_is_defined() {
    let families = packet().represented_families();
    for family in M5ExecutionComponentFamily::ALL {
        assert!(
            families.contains(&family),
            "missing family: {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_carries_degraded_rows() {
    assert!(packet().degraded_row_count() >= 1);
}

#[test]
fn payload_is_present_only_for_its_family() {
    for row in &packet().components {
        assert!(
            row.payload_matches_family(),
            "payload mismatch for {}",
            row.component_id
        );
    }
}

#[test]
fn missing_family_fails() {
    let mut packet = packet();
    packet
        .components
        .retain(|r| r.family != M5ExecutionComponentFamily::ThreadProcessTree);
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::RequiredFamilyMissing));
}

#[test]
fn no_degraded_row_fails() {
    let mut packet = packet();
    for row in &mut packet.components {
        row.degraded = None;
    }
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::DegradedCaseMissing));
}

#[test]
fn wrong_payload_for_family_fails() {
    let mut packet = packet();
    // Attach a stray run/attempt-header payload to an input-request-prompt row.
    let row = row_mut(&mut packet, "component:input-request-prompt:0001");
    row.run_attempt_header = Some(RunAttemptHeaderDescriptor {
        run_identity_ref: "run:leak:0001".to_owned(),
        attempt_identity_ref: "attempt:leak:0001#1".to_owned(),
        attempt_ordinal: 1,
        outcome: M5RunOutcome::Running,
        truth_mode: M5ExecutionTruthMode::Live,
        run_and_attempt_distinct: true,
    });
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::PayloadFamilyMismatch));
}

#[test]
fn run_attempt_header_blurring_identity_fails() {
    let mut packet = packet();
    let header = row_mut(&mut packet, "component:run-attempt-header:0001")
        .run_attempt_header
        .as_mut()
        .expect("header");
    // Collapse run and attempt identity into one string.
    header.attempt_identity_ref = header.run_identity_ref.clone();
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::PayloadDishonest));
}

#[test]
fn run_attempt_header_marking_distinct_false_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:run-attempt-header:0001")
        .run_attempt_header
        .as_mut()
        .expect("header")
        .run_and_attempt_distinct = false;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::PayloadDishonest));
}

#[test]
fn stale_output_shown_as_live_fails() {
    let mut packet = packet();
    let row = row_mut(&mut packet, "component:run-attempt-header:0002");
    row.truth_mode = M5ExecutionTruthMode::Live;
    row.run_attempt_header.as_mut().expect("header").truth_mode = M5ExecutionTruthMode::Live;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::PayloadDishonest));
}

#[test]
fn input_prompt_hiding_timeout_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:input-request-prompt:0001")
        .input_request_prompt
        .as_mut()
        .expect("prompt")
        .discloses_timeout = false;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::PayloadDishonest));
}

#[test]
fn input_prompt_timeout_without_deadline_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:input-request-prompt:0001")
        .input_request_prompt
        .as_mut()
        .expect("prompt")
        .has_deadline = false;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::PayloadDishonest));
}

#[test]
fn artifact_row_losing_lineage_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:artifact-publish-row:0001")
        .artifact_publish_row
        .as_mut()
        .expect("artifact row")
        .lineage_preserved = false;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::PayloadDishonest));
}

#[test]
fn artifact_row_without_producing_run_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:artifact-publish-row:0001")
        .artifact_publish_row
        .as_mut()
        .expect("artifact row")
        .producing_run_ref = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::PayloadDishonest));
}

#[test]
fn rerun_sheet_hiding_context_delta_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:rerun-comparison-sheet:0002")
        .rerun_comparison_sheet
        .as_mut()
        .expect("rerun sheet")
        .discloses_context_delta = false;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::PayloadDishonest));
}

#[test]
fn rerun_sheet_dispatching_context_drift_without_diff_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:rerun-comparison-sheet:0002")
        .rerun_comparison_sheet
        .as_mut()
        .expect("rerun sheet")
        .context_diff_shown_before_dispatch = false;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::PayloadDishonest));
}

#[test]
fn debug_header_captured_mode_claiming_live_fails() {
    let mut packet = packet();
    // A replay session is captured and can never claim live control.
    let row = row_mut(&mut packet, "component:debug-session-header:0002");
    row.truth_mode = M5ExecutionTruthMode::Live;
    row.debug_session_header.as_mut().expect("debug").truth_mode = M5ExecutionTruthMode::Live;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::PayloadDishonest));
}

#[test]
fn debug_header_disclosing_different_locality_than_row_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:debug-session-header:0001")
        .debug_session_header
        .as_mut()
        .expect("debug")
        .locality = M5ExecutionLocality::Remote;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::DescriptorRowMismatch));
}

#[test]
fn header_disclosing_different_truth_than_row_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:run-attempt-header:0001")
        .run_attempt_header
        .as_mut()
        .expect("header")
        .truth_mode = M5ExecutionTruthMode::Captured;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::DescriptorRowMismatch));
}

#[test]
fn thread_tree_without_live_vs_captured_explicit_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:thread-process-tree:0001")
        .thread_process_tree
        .as_mut()
        .expect("tree")
        .live_vs_captured_explicit = false;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::PayloadDishonest));
}

#[test]
fn dump_card_claiming_live_fails() {
    let mut packet = packet();
    // A dump is captured evidence, never live control.
    row_mut(&mut packet, "component:dump-crash-artifact-card:0001")
        .dump_crash_artifact_card
        .as_mut()
        .expect("dump card")
        .captured_truth = false;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::PayloadDishonest));
}

#[test]
fn missing_execution_context_ref_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:run-attempt-header:0001").execution_context_ref =
        "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::RowIncomplete));
}

#[test]
fn missing_mandatory_label_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:run-attempt-header:0001")
        .required_labels
        .retain(|l| *l != M5ExecutionRequiredLabel::KeyboardRoute);
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::MandatoryLabelMissing));
}

#[test]
fn not_export_safe_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:run-attempt-header:0001").export_safe = false;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::ParityMissing));
}

#[test]
fn not_assistive_ready_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:run-attempt-header:0001").assistive_ready = false;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::ParityMissing));
}

#[test]
fn generic_degraded_label_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:run-attempt-header:0002")
        .degraded
        .as_mut()
        .expect("degraded")
        .degraded_label = "stale".to_owned();
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::DegradedLabelGeneric));
}

#[test]
fn row_without_evidence_fails() {
    let mut packet = packet();
    packet.components[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::RowEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != EXECUTION_LIFECYCLE_COMPONENT_MATRIX_DOC_REF);
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet.guardrails.artifacts_never_lose_lineage_or_retention = false;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .later_rows_reference_one_canonical_family = false;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::ConsumerProjectionIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::WrongRecordKind));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:run-attempt-header:0001").label_summary =
        "leaked Bearer abc123 token".to_owned();
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::RawBoundaryMaterialInExport));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: ExecutionLifecycleComponentMatrix =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn chip_tokens_name_governed_chips() {
    let row = &packet().components[0];
    let chips = row.chip_tokens();
    assert!(chips.contains("family=run_attempt_header"));
    assert!(chips.contains("truth=live"));
    assert!(chips.contains("locality=local"));
    assert!(chips.contains("export_safe=true"));
    assert!(chips.contains("assistive=true"));
}

#[test]
fn csv_names_every_component() {
    let csv = packet().render_matrix_csv();
    assert!(csv.contains("component_id,family,truth_mode"));
    assert!(csv.contains("component:dump-crash-artifact-card:0002"));
    assert!(csv.contains("symbols_unavailable"));
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Execution-Lifecycle Component Matrix"));
    assert!(summary.contains("component:rerun-comparison-sheet:0002"));
    assert!(summary.contains("Degraded:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_execution_lifecycle_component_matrix_export()
        .expect("checked execution-lifecycle component export validates");
    assert_eq!(checked, packet());
}
