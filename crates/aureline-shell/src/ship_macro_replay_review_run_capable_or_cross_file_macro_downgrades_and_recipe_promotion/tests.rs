use super::*;

fn packet() -> MacroReplayReviewPacket {
    seeded_macro_replay_review_packet()
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
    let violations = fixture_macro_replay_review_packet().validate();
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
fn seeded_packet_covers_every_outcome_class() {
    let outcomes = packet().represented_outcomes();
    for required in MacroReplayOutcomeClass::ALL {
        assert!(
            outcomes.contains(&required),
            "missing outcome class {}",
            required.as_str()
        );
    }
}

#[test]
fn outcome_safety_ranks_are_strictly_ordered() {
    assert!(
        MacroReplayOutcomeClass::ReviewRequiredBeforeApply.safety_rank()
            > MacroReplayOutcomeClass::ExactReplayLocalEditorOnly.safety_rank()
    );
    assert!(
        MacroReplayOutcomeClass::DowngradedToObserverNoMutation.safety_rank()
            > MacroReplayOutcomeClass::ReviewRequiredBeforeApply.safety_rank()
    );
    assert!(
        MacroReplayOutcomeClass::PromotedToDeclarativeRecipe.safety_rank()
            > MacroReplayOutcomeClass::DowngradedToObserverNoMutation.safety_rank()
    );
    assert!(
        MacroReplayOutcomeClass::RejectedUnsafeReplay.safety_rank()
            > MacroReplayOutcomeClass::PromotedToDeclarativeRecipe.safety_rank()
    );
}

#[test]
fn cross_file_macro_cannot_replay_silently() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "macro-replay:data-api:0001")
        .expect("data/api record");
    assert!(record.scope.is_cross_file());
    assert!(record.must_not_replay_silently());
    // Force the cross-file macro back onto the silent exact lane: the packet must
    // reject it.
    record.outcome = MacroReplayOutcomeClass::ExactReplayLocalEditorOnly;
    record.fired_triggers = record.computed_triggers().into_iter().collect();
    record.downgrade_target_label = None;
    let violations = packet.validate();
    assert!(violations.contains(&MacroReplayReviewViolation::SilentReplayOfUnsafeMacro));
}

#[test]
fn run_capable_macro_cannot_replay_silently() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "macro-replay:notebook:0001")
        .expect("notebook record");
    assert!(record.must_not_replay_silently());
    record.outcome = MacroReplayOutcomeClass::ExactReplayLocalEditorOnly;
    record.review_reason_label = None;
    let violations = packet.validate();
    assert!(violations.contains(&MacroReplayReviewViolation::SilentReplayOfUnsafeMacro));
}

#[test]
fn unstable_or_cross_surface_macro_must_promote_or_reject() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "macro-replay:runtime:0001")
        .expect("runtime record");
    // Floor for unstable timing is recipe-promotion; a mere downgrade is below it.
    assert_eq!(
        record.required_floor_rank(),
        MacroReplayOutcomeClass::PromotedToDeclarativeRecipe.safety_rank()
    );
    record.outcome = MacroReplayOutcomeClass::DowngradedToObserverNoMutation;
    record.promoted_recipe_ref = None;
    record.downgrade_target_label = Some("Observe only".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&MacroReplayReviewViolation::OutcomeBelowRequiredFloor));
}

#[test]
fn unmapped_step_forces_reject() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "macro-replay:docs:0001")
        .expect("docs record");
    assert_eq!(
        record.required_floor_rank(),
        MacroReplayOutcomeClass::RejectedUnsafeReplay.safety_rank()
    );
    // Anything short of reject is below the floor.
    record.outcome = MacroReplayOutcomeClass::PromotedToDeclarativeRecipe;
    record.rejection_reason_label = None;
    record.promoted_recipe_ref = Some("recipe:should-not-be-allowed@v1".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&MacroReplayReviewViolation::OutcomeBelowRequiredFloor));
}

#[test]
fn recorded_triggers_must_match_computed() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "macro-replay:notebook:0001")
        .expect("notebook record");
    record.fired_triggers = vec![]; // hides the run-capable trigger
    let violations = packet.validate();
    assert!(violations.contains(&MacroReplayReviewViolation::TriggerSetInconsistent));
}

#[test]
fn lineage_must_not_collapse_to_opaque_text() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "macro-replay:editor-core:0001")
        .expect("editor-core record");
    record.lineage_collapsed_to_opaque_text = true;
    let violations = packet.validate();
    assert!(violations.contains(&MacroReplayReviewViolation::LineageCollapsedToOpaqueText));
}

#[test]
fn empty_lineage_is_rejected() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "macro-replay:editor-core:0001")
        .expect("editor-core record");
    record.command_lineage.clear();
    let violations = packet.validate();
    assert!(violations.contains(&MacroReplayReviewViolation::LineageCollapsedToOpaqueText));
}

#[test]
fn step_missing_command_id_is_rejected() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "macro-replay:editor-core:0001")
        .expect("editor-core record");
    record.command_lineage[0].command_id = "   ".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&MacroReplayReviewViolation::LineageCollapsedToOpaqueText));
}

#[test]
fn ai_tool_handle_step_requires_handle_ref() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "macro-replay:review:0001")
        .expect("review record");
    let step = record
        .command_lineage
        .iter_mut()
        .find(|step| step.lineage_class == MacroCommandLineageClass::AiToolHandle)
        .expect("ai tool handle step");
    step.ai_tool_handle_ref = None;
    let violations = packet.validate();
    assert!(violations.contains(&MacroReplayReviewViolation::LineageCollapsedToOpaqueText));
}

#[test]
fn scope_counts_must_match_scope_class() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "macro-replay:data-api:0001")
        .expect("data/api record");
    // Cross-file scope but only one file touched is inconsistent.
    record.files_touched = 1;
    let violations = packet.validate();
    assert!(violations.contains(&MacroReplayReviewViolation::ScopeCountsInconsistent));
}

#[test]
fn outcome_detail_must_be_present_and_precise() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "macro-replay:notebook:0001")
        .expect("notebook record");
    record.review_reason_label = Some("error".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&MacroReplayReviewViolation::OutcomeDetailInconsistent));
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
    assert!(violations.contains(&MacroReplayReviewViolation::ImportedReadsAsLocal));
}

#[test]
fn stale_proof_forces_record_off_exact_lane() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "macro-replay:editor-core:0001")
        .expect("editor-core record");
    record.verification.proof_currency = AxisProofCurrency::StaleExpired;
    // The record kept its exact outcome but now a trigger fires: the packet must
    // reject the silent replay (and flag the now-stale trigger set).
    assert!(record.must_not_replay_silently());
    let violations = packet.validate();
    assert!(violations.contains(&MacroReplayReviewViolation::SilentReplayOfUnsafeMacro));
    assert!(violations.contains(&MacroReplayReviewViolation::TriggerSetInconsistent));
}

#[test]
fn source_register_must_be_present() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "macro-replay:editor-core:0001")
        .expect("editor-core record");
    record.source_register.register_token = "  ".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&MacroReplayReviewViolation::SourceRegisterMissing));
}

#[test]
fn raw_boundary_material_flag_is_rejected() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "macro-replay:editor-core:0001")
        .expect("editor-core record");
    record.raw_keystroke_bytes_present = true;
    let violations = packet.validate();
    assert!(violations.contains(&MacroReplayReviewViolation::RawBoundaryMaterialPresent));
}

#[test]
fn fingerprint_must_be_independent_of_id() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "macro-replay:docs:0001")
        .expect("docs record");
    record.subject.surface_fingerprint_token = record.subject.surface_id.clone();
    let violations = packet.validate();
    assert!(violations.contains(&MacroReplayReviewViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = packet();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&MacroReplayReviewViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|r| r != MACRO_REPLAY_REVIEW_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&MacroReplayReviewViolation::MissingSourceContracts));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: MacroReplayReviewPacket = serde_json::from_str(&json).expect("export round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_records_and_outcomes() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Macro-Replay Review"));
    assert!(summary.contains("editor_core"));
    assert!(summary.contains("Promoted to recipe:"));
    assert!(summary.contains("command lineage:"));
    assert!(summary.contains("Rejected:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked =
        current_macro_replay_review_export().expect("checked macro replay review export validates");
    assert_eq!(checked, packet());
}
