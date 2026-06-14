use super::*;

fn packet() -> TransferSafetyPacket {
    seeded_transfer_safety_packet()
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
    let violations = fixture_transfer_safety_packet().validate();
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
fn seeded_packet_covers_every_resolution_class() {
    let resolutions = packet().represented_resolutions();
    for required in TransferDisclosureClass::ALL {
        assert!(
            resolutions.contains(&required),
            "missing resolution class {}",
            required.as_str()
        );
    }
}

#[test]
fn resolution_safety_ranks_are_strictly_ordered() {
    let ranks: Vec<u8> = TransferDisclosureClass::ALL
        .iter()
        .map(|class| class.safety_rank())
        .collect();
    for window in ranks.windows(2) {
        assert!(window[0] < window[1], "ranks must be strictly increasing");
    }
}

#[test]
fn multi_verb_drop_cannot_commit_silently() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "transfer:data-api:0001")
        .expect("data-api record");
    assert!(record.must_not_commit_silently());
    // Force the multi-verb drop back onto the inline lane: the packet must reject it.
    record.resolution = TransferDisclosureClass::DisclosedInlineCommit;
    record.verb_choice_label = None;
    let violations = packet.validate();
    assert!(violations.contains(&TransferSafetyViolation::SilentCommitOfUnsafeTransfer));
}

#[test]
fn cross_window_detach_requires_continuity() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "transfer:review:0001")
        .expect("review record");
    assert_eq!(
        record.required_floor_rank(),
        TransferDisclosureClass::CrossWindowContinuityPreserved.safety_rank()
    );
    // A mere verb-choice disclosure is below the cross-window floor.
    record.resolution = TransferDisclosureClass::ExplicitVerbChoiceDisclosed;
    record.continuity_note = None;
    record.verb_choice_label = Some("Move it".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&TransferSafetyViolation::ResolutionBelowRequiredFloor));
}

#[test]
fn large_transfer_requires_progress() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "transfer:preview:0001")
        .expect("preview record");
    assert!(record.transfer_magnitude.needs_progress());
    assert_eq!(
        record.required_floor_rank(),
        TransferDisclosureClass::ProgressCancelSummaryTracked.safety_rank()
    );
    // Cross-window continuity is below the long-transfer floor.
    record.resolution = TransferDisclosureClass::CrossWindowContinuityPreserved;
    record.progress_note = None;
    record.continuity_note = Some("Keeps context".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&TransferSafetyViolation::ResolutionBelowRequiredFloor));
}

#[test]
fn import_drop_requires_confirmation() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "transfer:notebook:import:0001")
        .expect("import record");
    assert!(record.import_or_destructive);
    assert_eq!(
        record.required_floor_rank(),
        TransferDisclosureClass::ConfirmedBeforeMutation.safety_rank()
    );
    record.resolution = TransferDisclosureClass::ProgressCancelSummaryTracked;
    record.confirmation_label = None;
    record.progress_note = Some("Streams in".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&TransferSafetyViolation::ResolutionBelowRequiredFloor));
}

#[test]
fn destructive_default_verb_must_reject() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "transfer:runtime:reject:0001")
        .expect("runtime record");
    assert_eq!(
        record.required_floor_rank(),
        TransferDisclosureClass::RejectedAmbiguousOrUnsafe.safety_rank()
    );
    // Anything short of reject is below the floor.
    record.resolution = TransferDisclosureClass::ConfirmedBeforeMutation;
    record.rejection_reason_label = None;
    record.confirmation_label = Some("Confirm overwrite".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&TransferSafetyViolation::ResolutionBelowRequiredFloor));
}

#[test]
fn recorded_triggers_must_match_computed() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "transfer:preview:0001")
        .expect("preview record");
    record.fired_triggers = vec![]; // hides the large-transfer trigger
    let violations = packet.validate();
    assert!(violations.contains(&TransferSafetyViolation::TriggerSetInconsistent));
}

#[test]
fn verb_must_be_disclosed_before_commit() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "transfer:editor-core:0001")
        .expect("editor-core record");
    record.verb_disclosed_before_commit = false;
    let violations = packet.validate();
    assert!(violations.contains(&TransferSafetyViolation::VerbDisclosureMissing));
}

#[test]
fn committing_record_must_disclose_insertion_point() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "transfer:notebook:0001")
        .expect("notebook record");
    record.insertion_point_disclosed_before_commit = false;
    let violations = packet.validate();
    assert!(violations.contains(&TransferSafetyViolation::InsertionDisclosureMissing));
}

#[test]
fn rejected_record_need_not_disclose_insertion_point() {
    let packet = packet();
    let record = packet
        .records
        .iter()
        .find(|record| record.record_id == "transfer:runtime:reject:0001")
        .expect("runtime record");
    assert!(!record.insertion_point_disclosed_before_commit);
    assert!(record.insertion_disclosure_holds());
    assert!(record.is_complete());
}

#[test]
fn resolution_detail_must_be_present_and_precise() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "transfer:data-api:0001")
        .expect("data-api record");
    record.verb_choice_label = Some("copied".to_owned()); // generic non-answer
    let violations = packet.validate();
    assert!(violations.contains(&TransferSafetyViolation::ResolutionDetailInconsistent));
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
    assert!(violations.contains(&TransferSafetyViolation::ImportedReadsAsLocal));
}

#[test]
fn stale_proof_forces_record_off_inline_lane() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "transfer:editor-core:0001")
        .expect("editor-core record");
    record.verification.proof_currency = AxisProofCurrency::StaleExpired;
    // The record kept its inline resolution but now a trigger fires: the packet
    // must reject the silent commit (and flag the now-stale trigger set).
    assert!(record.must_not_commit_silently());
    let violations = packet.validate();
    assert!(violations.contains(&TransferSafetyViolation::SilentCommitOfUnsafeTransfer));
    assert!(violations.contains(&TransferSafetyViolation::TriggerSetInconsistent));
}

#[test]
fn transfer_object_must_be_present() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "transfer:editor-core:0001")
        .expect("editor-core record");
    record.object.object_token = "  ".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&TransferSafetyViolation::TransferObjectMissing));
}

#[test]
fn orphaned_detach_flag_is_rejected() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "transfer:review:0001")
        .expect("review record");
    record.orphaned_on_detach = true;
    let violations = packet.validate();
    assert!(violations.contains(&TransferSafetyViolation::RawBoundaryMaterialPresent));
}

#[test]
fn destructive_default_commit_flag_is_rejected() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "transfer:editor-core:0001")
        .expect("editor-core record");
    record.destructive_default_commit_taken = true;
    let violations = packet.validate();
    assert!(violations.contains(&TransferSafetyViolation::RawBoundaryMaterialPresent));
}

#[test]
fn fingerprint_must_be_independent_of_id() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "transfer:notebook:0001")
        .expect("notebook record");
    record.subject.surface_fingerprint_token = record.subject.surface_id.clone();
    let violations = packet.validate();
    assert!(violations.contains(&TransferSafetyViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = packet();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&TransferSafetyViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|r| r != TRANSFER_SAFETY_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&TransferSafetyViolation::MissingSourceContracts));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: TransferSafetyPacket = serde_json::from_str(&json).expect("export round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_records_and_resolutions() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Transfer Safety"));
    assert!(summary.contains("editor_core"));
    assert!(summary.contains("Cross-window continuity:"));
    assert!(summary.contains("Progress/cancel/summary:"));
    assert!(summary.contains("Rejected:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked =
        current_transfer_safety_export().expect("checked transfer safety export validates");
    assert_eq!(checked, packet());
}
