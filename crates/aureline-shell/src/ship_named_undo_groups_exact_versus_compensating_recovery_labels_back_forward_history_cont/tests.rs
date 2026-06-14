use super::*;

fn packet() -> HistoryContinuityPacket {
    seeded_history_continuity_packet()
}

#[test]
fn seeded_packet_validates() {
    let violations = packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn fixture_packet_validates() {
    let violations = fixture_history_continuity_packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected fixture violations: {violations:?}"
    );
}

#[test]
fn seeded_packet_covers_required_surface_kinds() {
    let kinds = packet().represented_surface_kinds();
    for required in REQUIRED_SURFACE_KINDS {
        assert!(
            kinds.contains(&required),
            "missing surface kind {}",
            required.as_str()
        );
    }
}

#[test]
fn seeded_packet_covers_required_object_classes() {
    let classes = packet().represented_object_classes();
    for required in REQUIRED_OBJECT_CLASSES {
        assert!(
            classes.contains(&required),
            "missing object class {}",
            required.as_str()
        );
    }
}

#[test]
fn seeded_packet_covers_every_recovery_class() {
    let resolutions = packet().represented_resolutions();
    for required in RecoveryAffordanceClass::ALL {
        assert!(
            resolutions.contains(&required),
            "missing recovery class {}",
            required.as_str()
        );
    }
}

#[test]
fn recovery_safety_ranks_are_strictly_ordered() {
    let ranks: Vec<u8> = RecoveryAffordanceClass::ALL
        .iter()
        .map(|class| class.safety_rank())
        .collect();
    for window in ranks.windows(2) {
        assert!(window[0] < window[1], "ranks must be strictly increasing");
    }
}

#[test]
fn broad_mutation_cannot_flatten_to_one_undo() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:notebook:0001")
        .expect("notebook record");
    assert!(record.must_not_flatten());
    // Force the broad mutation back onto the flat single-step lane: reject it.
    record.resolution = RecoveryAffordanceClass::ExactStepUndo;
    record.undo_class = UndoClass::ExactUndo;
    record.group_label = None;
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::SilentFlatteningOfHistory));
}

#[test]
fn cross_surface_navigation_requires_continuity() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:preview:0001")
        .expect("preview record");
    assert_eq!(
        record.required_floor_rank(),
        RecoveryAffordanceClass::BackForwardContinuityPreserved.safety_rank()
    );
    // A mere named group is below the cross-surface continuity floor.
    record.resolution = RecoveryAffordanceClass::NamedGroupExactUndo;
    record.continuity_note = None;
    record.group_label = Some("Some group".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::ResolutionBelowRequiredFloor));
}

#[test]
fn non_invertible_change_requires_compensating_action() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:docs:0001")
        .expect("docs record");
    assert!(!record.literally_invertible);
    assert_eq!(
        record.required_floor_rank(),
        RecoveryAffordanceClass::CompensatingActionLabeled.safety_rank()
    );
    record.resolution = RecoveryAffordanceClass::BackForwardContinuityPreserved;
    record.compensating_label = None;
    record.continuity_note = Some("Keeps identity".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::ResolutionBelowRequiredFloor));
}

#[test]
fn generated_change_requires_regenerate_from_source() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:data-api:0001")
        .expect("data-api record");
    assert!(record.generated_or_automated);
    assert_eq!(
        record.required_floor_rank(),
        RecoveryAffordanceClass::RegenerateFromSource.safety_rank()
    );
    record.resolution = RecoveryAffordanceClass::CompensatingActionLabeled;
    record.regenerate_source_label = None;
    record.compensating_label = Some("Undo it somehow".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::ResolutionBelowRequiredFloor));
}

#[test]
fn checkpoint_only_requires_checkpoint_restore() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:review:0001")
        .expect("review record");
    assert!(record.checkpoint_only);
    assert_eq!(
        record.required_floor_rank(),
        RecoveryAffordanceClass::CheckpointRestoreLabeled.safety_rank()
    );
    record.resolution = RecoveryAffordanceClass::RegenerateFromSource;
    record.checkpoint_label = None;
    record.regenerate_source_label = Some("Regenerate it".to_owned());
    record.undo_class = UndoClass::CompensatingUndo;
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::ResolutionBelowRequiredFloor));
}

#[test]
fn closed_surface_requires_reopen_recover() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:runtime:0001")
        .expect("runtime record");
    assert!(record.surface_closed_or_lost());
    assert_eq!(
        record.required_floor_rank(),
        RecoveryAffordanceClass::ReopenOrRecoverLabeled.safety_rank()
    );
    // Anything short of reopen/recover is below the floor.
    record.resolution = RecoveryAffordanceClass::CheckpointRestoreLabeled;
    record.reopen_label = None;
    record.checkpoint_label = Some("Restore from checkpoint".to_owned());
    record.undo_class = UndoClass::CheckpointRestore;
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::ResolutionBelowRequiredFloor));
}

#[test]
fn reopen_distinguishes_intentional_close_from_loss() {
    let packet = packet();
    let intentional = packet
        .record("history:companion:0001")
        .expect("companion record");
    let loss = packet
        .record("history:runtime:0001")
        .expect("runtime record");
    assert_eq!(intentional.loss_cause, SurfaceLossCause::IntentionalClose);
    assert!(loss.loss_cause.is_unintended_loss());
    assert!(intentional.closed_at.is_some());
    assert!(loss.closed_at.is_some());
}

#[test]
fn closed_surface_must_carry_close_timestamp() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:runtime:0001")
        .expect("runtime record");
    record.closed_at = None;
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::LossCauseInconsistent));
}

#[test]
fn open_surface_must_not_carry_close_timestamp() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:editor-core:0001")
        .expect("editor record");
    record.closed_at = Some("2026-06-13T00:00:00Z".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::LossCauseInconsistent));
}

#[test]
fn recorded_triggers_must_match_computed() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:notebook:0001")
        .expect("notebook record");
    record.fired_triggers = vec![]; // hides the broad-mutation trigger
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::TriggerSetInconsistent));
}

#[test]
fn undo_class_must_match_resolution() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:docs:0001")
        .expect("docs record");
    // Compensating resolution must not read as an exact undo.
    record.undo_class = UndoClass::ExactUndo;
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::UndoClassInconsistent));
}

#[test]
fn resolution_detail_must_be_present_and_precise() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:notebook:0001")
        .expect("notebook record");
    record.group_label = Some("group".to_owned()); // generic non-answer
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::ResolutionDetailInconsistent));
}

#[test]
fn reopen_recover_must_not_claim_unavailable() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:companion:0001")
        .expect("companion record");
    record.reopen_recover = ReopenRecoverClass::ReopenUnavailableHonest;
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::ReopenRecoverInconsistent));
}

#[test]
fn provider_record_proof_never_reads_as_local() {
    let packet = packet();
    let companion = packet
        .records
        .iter()
        .find(|record| record.surface_kind == KeyboardSurfaceKind::CompanionSurface)
        .expect("companion record");
    assert!(companion.provider_or_imported());
    assert!(companion.imported_posture_consistent());
    assert!(companion.verification.backs_claim(true));
    assert!(!companion.verification.backs_claim(false));
}

#[test]
fn provider_record_with_local_proof_is_rejected() {
    let mut packet = packet();
    let companion = packet
        .records
        .iter_mut()
        .find(|record| record.surface_kind == KeyboardSurfaceKind::CompanionSurface)
        .expect("companion record");
    companion.verification.proof_currency = AxisProofCurrency::VerifiedCurrent;
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::ImportedReadsAsLocal));
}

#[test]
fn stale_proof_forces_record_off_flat_lane() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:editor-core:0001")
        .expect("editor record");
    record.verification.proof_currency = AxisProofCurrency::StaleExpired;
    // The record kept its flat resolution but now a trigger fires: the packet
    // must reject the silent flattening (and flag the now-stale trigger set).
    assert!(record.must_not_flatten());
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::SilentFlatteningOfHistory));
    assert!(violations.contains(&HistoryContinuityViolation::TriggerSetInconsistent));
}

#[test]
fn affected_object_must_be_present() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:editor-core:0001")
        .expect("editor record");
    record.affected_object.object_token = "  ".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::AffectedObjectMissing));
}

#[test]
fn flattened_classes_flag_is_rejected() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:editor-core:0001")
        .expect("editor record");
    record.distinct_classes_flattened = true;
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::RawBoundaryMaterialPresent));
}

#[test]
fn conflated_loss_cause_flag_is_rejected() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:runtime:0001")
        .expect("runtime record");
    record.reopen_loss_cause_conflated = true;
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::RawBoundaryMaterialPresent));
}

#[test]
fn fingerprint_must_be_independent_of_id() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "history:notebook:0001")
        .expect("notebook record");
    record.subject.surface_fingerprint_token = record.subject.surface_id.clone();
    let violations = packet.validate();
    assert!(violations.contains(&HistoryContinuityViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = packet();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&HistoryContinuityViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|r| r != HISTORY_CONTINUITY_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&HistoryContinuityViolation::MissingSourceContracts));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: HistoryContinuityPacket = serde_json::from_str(&json).expect("export round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_records_and_recoveries() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Grouped-History Continuity"));
    assert!(summary.contains("editor_core"));
    assert!(summary.contains("Named group:"));
    assert!(summary.contains("Compensating action:"));
    assert!(summary.contains("Regenerate from source:"));
    assert!(summary.contains("Checkpoint restore:"));
    assert!(summary.contains("Reopen/recover:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked =
        current_history_continuity_export().expect("checked history continuity export validates");
    assert_eq!(checked, packet());
}
