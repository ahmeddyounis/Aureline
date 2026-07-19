use aureline_runtime::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix::{
    current_m5_execution_lifecycle_component_matrix_export, ExecutionLifecycleComponentMatrix,
    ExecutionLifecycleComponentViolation, M5ExecutionComponentFamily, M5ExecutionTruthMode,
    M5RunOutcome,
};

fn fixture(name: &str) -> ExecutionLifecycleComponentMatrix {
    let path = format!(
        "{}/../../fixtures/ui/m5-execution-lifecycle-components/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let contents = std::fs::read_to_string(path).expect("fixture should be readable");
    serde_json::from_str(&contents).expect("fixture should parse")
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_m5_execution_lifecycle_component_matrix_export()
        .expect("checked-in execution-lifecycle component export should validate");
    assert!(packet.validate().is_empty());
    for family in M5ExecutionComponentFamily::ALL {
        assert!(
            packet.represented_families().contains(&family),
            "missing family {}",
            family.as_str()
        );
    }
}

#[test]
fn fixture_copy_matches_checked_export() {
    let checked = current_m5_execution_lifecycle_component_matrix_export()
        .expect("checked-in export should validate");
    let fixture = fixture("m5-execution-lifecycle-component-matrix.json");
    assert!(fixture.validate().is_empty());
    assert_eq!(fixture, checked);
}

#[test]
fn matrix_carries_a_complete_degraded_row() {
    let packet =
        current_m5_execution_lifecycle_component_matrix_export().expect("export validates");
    assert!(packet.degraded_row_count() >= 1);
}

#[test]
fn stale_output_is_captured_not_live() {
    let packet =
        current_m5_execution_lifecycle_component_matrix_export().expect("export validates");
    let stale = packet
        .components
        .iter()
        .filter_map(|row| row.run_attempt_header.as_ref())
        .find(|header| header.outcome == M5RunOutcome::StaleOutput)
        .expect("a stale-output run/attempt header");
    assert_eq!(stale.truth_mode, M5ExecutionTruthMode::Captured);
    assert!(stale.run_and_attempt_distinct);
}

#[test]
fn dump_cards_keep_producing_run_lineage() {
    let packet =
        current_m5_execution_lifecycle_component_matrix_export().expect("export validates");
    let dumps: Vec<_> = packet
        .components
        .iter()
        .filter_map(|row| row.dump_crash_artifact_card.as_ref())
        .collect();
    assert!(!dumps.is_empty());
    for dump in dumps {
        assert!(!dump.producing_run_ref.trim().is_empty());
        assert!(dump.captured_truth);
    }
}

#[test]
fn debug_replay_never_reads_as_live() {
    // Corrupt a captured replay header into claiming live control; validation must
    // reject it.
    let mut packet = current_m5_execution_lifecycle_component_matrix_export()
        .expect("checked-in export should validate");
    let replay = packet
        .components
        .iter_mut()
        .find(|row| {
            row.debug_session_header
                .as_ref()
                .is_some_and(|d| !d.session_mode.is_live_control())
        })
        .expect("a captured debug session header");
    replay.truth_mode = M5ExecutionTruthMode::Live;
    replay.debug_session_header.as_mut().unwrap().truth_mode = M5ExecutionTruthMode::Live;
    assert!(packet
        .validate()
        .contains(&ExecutionLifecycleComponentViolation::PayloadDishonest));
}
