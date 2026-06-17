use super::*;

const CANONICAL_PACKET_ID: &str = "m5-git-topology-history-matrix:frozen:0001";

const CANONICAL_EXPORT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix/support_export.json"
));

const SUBMODULE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix/submodule_uninitialized_narrowed.json"
));

const RESET_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix/reset_reflog_only_recovery.json"
));

fn baseline() -> M5GitTopologyHistoryMatrixPacket {
    serde_json::from_str(CANONICAL_EXPORT).expect("canonical export deserializes")
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_git_topology_history_matrix_export()
        .expect("checked M5 git topology history matrix export validates");
    assert_eq!(packet.packet_id, CANONICAL_PACKET_ID);
}

#[test]
fn canonical_packet_validates_clean() {
    assert!(
        baseline().validate().is_empty(),
        "{:?}",
        baseline().validate()
    );
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [SUBMODULE_FIXTURE, RESET_FIXTURE] {
        let packet: M5GitTopologyHistoryMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn matrix_covers_every_frozen_topology_class() {
    let packet = baseline();
    for required in M5_GIT_TOPOLOGY_HISTORY_MATRIX_REQUIRED_CLASSES {
        assert!(
            packet
                .topology_rows
                .iter()
                .any(|row| row.topology_class == required),
            "missing topology class {}",
            required.as_str()
        );
    }
}

#[test]
fn session_rows_reference_canonical_record_kinds() {
    let packet = baseline();
    for session in HistorySurgerySession::ALL {
        let row = packet
            .session_object_rows
            .iter()
            .find(|row| row.session == session)
            .expect("session row present");
        assert_eq!(
            row.canonical_record_kind,
            session.canonical_record_kind(),
            "session {} must bind its canonical record kind",
            session.as_str()
        );
    }
}

#[test]
fn matrix_covers_every_degraded_state_and_risky_operation() {
    let packet = baseline();
    for state in DegradedTopologyState::ALL {
        assert!(
            packet
                .degraded_state_rows
                .iter()
                .any(|row| row.state == state),
            "missing degraded state {}",
            state.as_str()
        );
    }
    for operation in RiskyHistoryOperation::ALL {
        assert!(
            packet
                .risky_operation_rows
                .iter()
                .any(|row| row.operation == operation),
            "missing risky operation {}",
            operation.as_str()
        );
    }
}

#[test]
fn missing_topology_class_fails() {
    let mut packet = baseline();
    packet
        .topology_rows
        .retain(|row| row.topology_class != RepositoryTopologyClass::SubmoduleRoot);
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::RequiredTopologyClassMissing));
}

#[test]
fn duplicate_topology_class_fails() {
    let mut packet = baseline();
    let dup = packet.topology_rows[0].clone();
    packet.topology_rows.push(dup);
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::DuplicateTopologyClass));
}

#[test]
fn mutating_topology_row_without_recovery_fails() {
    let mut packet = baseline();
    // Force the sparse row (active_root_only => mutating) to drop its recovery.
    let row = packet
        .topology_rows
        .iter_mut()
        .find(|row| row.topology_class == RepositoryTopologyClass::SparseCheckoutRoot)
        .expect("sparse row present");
    row.recovery_class = OperationRecoveryClass::NoMutationNoRecovery;
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::MutatingRowMissingRecovery));
}

#[test]
fn missing_session_object_fails() {
    let mut packet = baseline();
    packet
        .session_object_rows
        .retain(|row| row.session != HistorySurgerySession::StashShelfEntry);
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::RequiredSessionObjectMissing));
}

#[test]
fn session_record_kind_mismatch_fails() {
    let mut packet = baseline();
    packet.session_object_rows[0].canonical_record_kind = "not_the_canonical_kind".to_owned();
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::SessionRecordKindMismatch));
}

#[test]
fn missing_degraded_state_fails() {
    let mut packet = baseline();
    packet
        .degraded_state_rows
        .retain(|row| row.state != DegradedTopologyState::ReflogOnlyFallback);
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::RequiredDegradedStateMissing));
}

#[test]
fn degraded_state_reduced_to_badge_fails() {
    let mut packet = baseline();
    packet.degraded_state_rows[0].narrows_coverage_claim = false;
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::DegradedStateRowIncomplete));
}

#[test]
fn reflog_only_invisible_before_destructive_op_fails() {
    let mut packet = baseline();
    let row = packet
        .degraded_state_rows
        .iter_mut()
        .find(|row| row.state == DegradedTopologyState::ReflogOnlyFallback)
        .expect("reflog-only row present");
    row.visible_before_destructive_op = false;
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::DegradedStateRowIncomplete));
}

#[test]
fn missing_risky_operation_fails() {
    let mut packet = baseline();
    packet
        .risky_operation_rows
        .retain(|row| row.operation != RiskyHistoryOperation::ForcePushWithLease);
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::RequiredRiskyOperationMissing));
}

#[test]
fn risky_operation_without_preview_fails() {
    let mut packet = baseline();
    packet.risky_operation_rows[0].preview_class = OperationPreviewClass::NoPreviewReadOnly;
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::RiskyOperationMissingPreviewOrRecovery));
}

#[test]
fn risky_operation_without_recovery_fails() {
    let mut packet = baseline();
    packet.risky_operation_rows[0].recovery_class = OperationRecoveryClass::NoMutationNoRecovery;
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::RiskyOperationMissingPreviewOrRecovery));
}

#[test]
fn missing_source_contract_fails() {
    let mut packet = baseline();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = baseline();
    packet
        .governance_review
        .provider_overlay_never_overwrites_local_truth = false;
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_parity_incomplete_fails() {
    let mut packet = baseline();
    packet.consumer_parity.support_export_expresses_vocabulary = false;
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::ConsumerParityIncomplete));
}

#[test]
fn freeze_posture_unfrozen_fails() {
    let mut packet = baseline();
    packet.freeze_posture.frozen = false;
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::FreezePostureIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = baseline();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::WrongRecordKind));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = baseline();
    packet.matrix_label = "leak: bearer abc123".to_owned();
    assert!(packet
        .validate()
        .contains(&M5GitTopologyHistoryMatrixViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_every_row() {
    let summary = baseline().render_markdown_summary();
    for class in M5_GIT_TOPOLOGY_HISTORY_MATRIX_REQUIRED_CLASSES {
        assert!(
            summary.contains(class.as_str()),
            "summary missing topology class {}",
            class.as_str()
        );
    }
    for session in HistorySurgerySession::ALL {
        assert!(
            summary.contains(session.as_str()),
            "summary missing session {}",
            session.as_str()
        );
    }
    for state in DegradedTopologyState::ALL {
        assert!(
            summary.contains(state.as_str()),
            "summary missing degraded state {}",
            state.as_str()
        );
    }
    for operation in RiskyHistoryOperation::ALL {
        assert!(
            summary.contains(operation.as_str()),
            "summary missing risky operation {}",
            operation.as_str()
        );
    }
}

#[test]
fn submodule_fixture_narrows_to_mutation_denied() {
    let packet: M5GitTopologyHistoryMatrixPacket =
        serde_json::from_str(SUBMODULE_FIXTURE).expect("submodule fixture parses");
    let row = packet
        .topology_rows
        .iter()
        .find(|row| row.topology_class == RepositoryTopologyClass::SubmoduleRoot)
        .expect("submodule row present");
    assert!(!row.permits_mutation());
    assert_eq!(
        row.recovery_class,
        OperationRecoveryClass::NoRecoveryOperationBlocked
    );
}

#[test]
fn reset_fixture_falls_back_to_reflog_only_recovery() {
    let packet: M5GitTopologyHistoryMatrixPacket =
        serde_json::from_str(RESET_FIXTURE).expect("reset fixture parses");
    let row = packet
        .risky_operation_rows
        .iter()
        .find(|row| row.operation == RiskyHistoryOperation::Reset)
        .expect("reset row present");
    assert_eq!(
        row.recovery_class,
        OperationRecoveryClass::ReflogOnlyFallbackDisclosed
    );
    assert!(row.recovery_visible_before_execution);
}
