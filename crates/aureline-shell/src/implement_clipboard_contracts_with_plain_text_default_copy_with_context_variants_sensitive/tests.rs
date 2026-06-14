use super::*;

fn packet() -> ClipboardContractPacket {
    seeded_clipboard_contract_packet()
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
    let violations = fixture_clipboard_contract_packet().validate();
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
    for required in CopyResolutionClass::ALL {
        assert!(
            resolutions.contains(&required),
            "missing resolution class {}",
            required.as_str()
        );
    }
}

#[test]
fn resolution_safety_ranks_are_strictly_ordered() {
    assert!(
        CopyResolutionClass::CopyWithContextVariant.safety_rank()
            > CopyResolutionClass::PlainTextDefaultCopy.safety_rank()
    );
    assert!(
        CopyResolutionClass::SensitiveLabeledBeforeCopy.safety_rank()
            > CopyResolutionClass::CopyWithContextVariant.safety_rank()
    );
    assert!(
        CopyResolutionClass::RelativizedOrRedactedCopy.safety_rank()
            > CopyResolutionClass::SensitiveLabeledBeforeCopy.safety_rank()
    );
    assert!(
        CopyResolutionClass::RejectedRichOnlyOrUnsafe.safety_rank()
            > CopyResolutionClass::RelativizedOrRedactedCopy.safety_rank()
    );
}

#[test]
fn context_copy_cannot_push_silently() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "clipboard:notebook:0001")
        .expect("notebook record");
    assert!(record.context_bundled);
    assert!(record.must_not_push_silently());
    // Force the context copy back onto the silent default lane: the packet must
    // reject it.
    record.resolution = CopyResolutionClass::PlainTextDefaultCopy;
    record.fired_triggers = record.computed_triggers().into_iter().collect();
    record.context_label = None;
    let violations = packet.validate();
    assert!(violations.contains(&ClipboardContractViolation::SilentPushOfUnsafeCopy));
}

#[test]
fn sensitive_copy_cannot_push_silently() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "clipboard:runtime:0001")
        .expect("runtime record");
    assert!(record.must_not_push_silently());
    record.resolution = CopyResolutionClass::PlainTextDefaultCopy;
    record.sensitive_label = None;
    let violations = packet.validate();
    assert!(violations.contains(&ClipboardContractViolation::SilentPushOfUnsafeCopy));
}

#[test]
fn private_path_must_relativize_or_redact() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "clipboard:editor-core:private-path:0001")
        .expect("private-path record");
    // Floor for a private absolute path is relativize/redact; a mere label is below it.
    assert_eq!(
        record.required_floor_rank(),
        CopyResolutionClass::RelativizedOrRedactedCopy.safety_rank()
    );
    record.resolution = CopyResolutionClass::SensitiveLabeledBeforeCopy;
    record.transform_note = None;
    record.sensitive_label = Some("This path is private".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&ClipboardContractViolation::ResolutionBelowRequiredFloor));
}

#[test]
fn rich_only_copy_must_reject() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "clipboard:docs:rich-only:0001")
        .expect("rich-only record");
    assert!(!record.plain_text_preserved());
    assert_eq!(
        record.required_floor_rank(),
        CopyResolutionClass::RejectedRichOnlyOrUnsafe.safety_rank()
    );
    // Anything short of reject is below the floor.
    record.resolution = CopyResolutionClass::SensitiveLabeledBeforeCopy;
    record.rejection_reason_label = None;
    record.sensitive_label = Some("Rich content".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&ClipboardContractViolation::ResolutionBelowRequiredFloor));
    assert!(violations.contains(&ClipboardContractViolation::PlainTextRepresentationMissing));
}

#[test]
fn recorded_triggers_must_match_computed() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "clipboard:runtime:0001")
        .expect("runtime record");
    record.fired_triggers = vec![]; // hides the support-link trigger
    let violations = packet.validate();
    assert!(violations.contains(&ClipboardContractViolation::TriggerSetInconsistent));
}

#[test]
fn representations_must_not_collapse_to_rich_blob() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "clipboard:editor-core:0001")
        .expect("editor-core record");
    record.representations_collapsed_to_rich_blob = true;
    let violations = packet.validate();
    assert!(violations.contains(&ClipboardContractViolation::RepresentationsCollapsedToRichBlob));
}

#[test]
fn empty_representations_is_rejected() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "clipboard:editor-core:0001")
        .expect("editor-core record");
    record.representations.clear();
    let violations = packet.validate();
    assert!(violations.contains(&ClipboardContractViolation::RepresentationsCollapsedToRichBlob));
}

#[test]
fn non_rejected_record_must_keep_plain_text() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "clipboard:review:0001")
        .expect("review record");
    // Drop the plain-text fallback, leaving only the markdown rich flavor.
    record.representations.retain(|rep| !rep.is_plain_text());
    assert!(!record.plain_text_preserved());
    let violations = packet.validate();
    assert!(violations.contains(&ClipboardContractViolation::PlainTextRepresentationMissing));
}

#[test]
fn resolution_detail_must_be_present_and_precise() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "clipboard:runtime:0001")
        .expect("runtime record");
    record.sensitive_label = Some("sensitive".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&ClipboardContractViolation::ResolutionDetailInconsistent));
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
    assert!(violations.contains(&ClipboardContractViolation::ImportedReadsAsLocal));
}

#[test]
fn stale_proof_forces_record_off_silent_lane() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "clipboard:editor-core:0001")
        .expect("editor-core record");
    record.verification.proof_currency = AxisProofCurrency::StaleExpired;
    // The record kept its silent default resolution but now a trigger fires: the
    // packet must reject the silent push (and flag the now-stale trigger set).
    assert!(record.must_not_push_silently());
    let violations = packet.validate();
    assert!(violations.contains(&ClipboardContractViolation::SilentPushOfUnsafeCopy));
    assert!(violations.contains(&ClipboardContractViolation::TriggerSetInconsistent));
}

#[test]
fn copy_object_must_be_present() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "clipboard:editor-core:0001")
        .expect("editor-core record");
    record.object.object_token = "  ".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&ClipboardContractViolation::CopyObjectMissing));
}

#[test]
fn raw_boundary_material_flag_is_rejected() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "clipboard:editor-core:0001")
        .expect("editor-core record");
    record.raw_secret_material_present = true;
    let violations = packet.validate();
    assert!(violations.contains(&ClipboardContractViolation::RawBoundaryMaterialPresent));
}

#[test]
fn fingerprint_must_be_independent_of_id() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "clipboard:docs:0001")
        .expect("docs record");
    record.subject.surface_fingerprint_token = record.subject.surface_id.clone();
    let violations = packet.validate();
    assert!(violations.contains(&ClipboardContractViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = packet();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&ClipboardContractViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|r| r != CLIPBOARD_CONTRACT_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&ClipboardContractViolation::MissingSourceContracts));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: ClipboardContractPacket = serde_json::from_str(&json).expect("export round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_records_and_resolutions() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Clipboard Contracts"));
    assert!(summary.contains("editor_core"));
    assert!(summary.contains("Sensitive-copy label:"));
    assert!(summary.contains("representations:"));
    assert!(summary.contains("Rejected:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked =
        current_clipboard_contract_export().expect("checked clipboard contract export validates");
    assert_eq!(checked, packet());
}
