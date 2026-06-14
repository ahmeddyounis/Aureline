use super::*;

fn packet() -> KeyboardContinuityMatrixPacket {
    seeded_keyboard_continuity_matrix_packet()
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
fn seeded_packet_covers_every_surface_kind() {
    let kinds = packet().represented_surface_kinds();
    for required in KeyboardSurfaceKind::ALL {
        assert!(
            kinds.contains(&required),
            "missing surface kind {}",
            required.as_str()
        );
    }
}

#[test]
fn distinct_undo_classes_present_and_unflattened() {
    let undo = packet().represented_undo_classes();
    for required in UndoClass::DISTINCT_CORE {
        assert!(
            undo.contains(&required),
            "missing distinct undo class {}",
            required.as_str()
        );
    }
    // Exact, compensating, and checkpoint never collapse into one another.
    assert_ne!(UndoClass::ExactUndo, UndoClass::CompensatingUndo);
    assert_ne!(UndoClass::CompensatingUndo, UndoClass::CheckpointRestore);
}

#[test]
fn exactly_one_seeded_row_downgrades() {
    let packet = packet();
    assert_eq!(packet.downgraded_row_count(), 1);
    let downgraded = packet
        .rows
        .iter()
        .find(|row| row.needs_downgrade())
        .expect("a downgraded row");
    assert_eq!(downgraded.surface_kind, KeyboardSurfaceKind::DataApiSurface);
    assert_eq!(
        downgraded.sequence_guide,
        SequenceGuideClass::SequenceUnsupportedDowngraded
    );
    assert_eq!(
        downgraded.effective_grade,
        ContinuityParityGrade::ParityUnverified
    );
    assert!(downgraded.properly_downgraded());
    assert_eq!(
        downgraded.downgrade_trigger,
        Some(ParityDowngradeTrigger::UnsupportedSequenceDowngraded)
    );
}

#[test]
fn claimed_row_losing_current_proof_must_downgrade() {
    let mut packet = packet();
    // The editor-core row keeps its switching-certified claim but loses current
    // proof: the row must auto-downgrade before promotion, so validation rejects
    // it when the effective grade still matches the claim.
    let editor = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "kbd-cont:editor-core:0001")
        .expect("editor core row");
    editor.verification.proof_currency = AxisProofCurrency::StaleExpired;
    assert!(editor.needs_downgrade());
    let violations = packet.validate();
    assert!(
        violations.contains(&KeyboardContinuityViolation::RowNotDowngradedOnUnidentifiedBehavior)
    );
}

#[test]
fn unsupported_sequence_never_silently_approximated() {
    let mut packet = packet();
    // Pretend the downgraded row silently "approximated" the sequence by claiming
    // full parity again without lowering its effective grade.
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.sequence_guide == SequenceGuideClass::SequenceUnsupportedDowngraded)
        .expect("unsupported-sequence row");
    row.effective_grade = ContinuityParityGrade::ParityComplete;
    let violations = packet.validate();
    assert!(
        violations.contains(&KeyboardContinuityViolation::RowNotDowngradedOnUnidentifiedBehavior)
    );
}

#[test]
fn rich_only_clipboard_route_is_rejected_when_not_downgraded() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "kbd-cont:docs:0001")
        .expect("docs row");
    // A rich-only route loses plain text; if the row keeps its full claim it must
    // be rejected.
    row.clipboard_route = ClipboardRouteClass::RichOnlyDenied;
    let violations = packet.validate();
    assert!(violations.contains(&KeyboardContinuityViolation::ClipboardPlainTextLost));
}

#[test]
fn destructive_drag_drop_default_is_rejected_when_not_downgraded() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "kbd-cont:notebook:0001")
        .expect("notebook row");
    row.drag_drop_verb = DragDropVerbClass::DestructiveDefaultDenied;
    let violations = packet.validate();
    assert!(violations.contains(&KeyboardContinuityViolation::DragDropDestructiveDefault));
}

#[test]
fn orientation_aids_collapse_is_rejected_when_not_downgraded() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "kbd-cont:preview:0001")
        .expect("preview row");
    row.orientation_aid = OrientationAidClass::OrientationAidsAbsentDowngraded;
    let violations = packet.validate();
    assert!(violations.contains(&KeyboardContinuityViolation::OrientationAidsCollapsedSilently));
}

#[test]
fn provider_row_proof_never_reads_as_local() {
    let packet = packet();
    let companion = packet
        .rows
        .iter()
        .find(|row| row.surface_kind == KeyboardSurfaceKind::CompanionSurface)
        .expect("companion row");
    assert!(companion.provider_or_imported());
    assert!(companion.imported_posture_consistent());
    // Imported proof backs the provider row's claim but never a local one.
    assert!(companion.verification.backs_claim(true));
    assert!(!companion.verification.backs_claim(false));
}

#[test]
fn provider_row_with_local_proof_is_rejected() {
    let mut packet = packet();
    let companion = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_kind == KeyboardSurfaceKind::CompanionSurface)
        .expect("companion row");
    // A provider surface claiming locally verified proof reads as a local rerun.
    companion.verification.proof_currency = AxisProofCurrency::VerifiedCurrent;
    let violations = packet.validate();
    assert!(violations.contains(&KeyboardContinuityViolation::ImportedReadsAsLocal));
}

#[test]
fn surface_must_stay_keyboard_complete() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "kbd-cont:review:0001")
        .expect("review row");
    row.keyboard_complete = false;
    let violations = packet.validate();
    assert!(violations.contains(&KeyboardContinuityViolation::SurfaceNotKeyboardComplete));
}

#[test]
fn macro_replay_must_be_explicit() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "kbd-cont:editor-core:0001")
        .expect("editor core row");
    row.macro_replay_explicit = false;
    let violations = packet.validate();
    assert!(violations.contains(&KeyboardContinuityViolation::MacroReplayNotExplicit));
}

#[test]
fn fingerprint_must_be_independent_of_id() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "kbd-cont:docs:0001")
        .expect("docs row");
    row.subject.surface_fingerprint_token = row.subject.surface_id.clone();
    let violations = packet.validate();
    assert!(violations.contains(&KeyboardContinuityViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn downgraded_label_must_not_be_generic() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.needs_downgrade())
        .expect("downgraded row");
    row.downgraded_label = Some("error".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&KeyboardContinuityViolation::DowngradedRowMissingLabelOrTrigger));
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = packet();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&KeyboardContinuityViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|r| r != KEYBOARD_CONTINUITY_MATRIX_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&KeyboardContinuityViolation::MissingSourceContracts));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: KeyboardContinuityMatrixPacket =
        serde_json::from_str(&json).expect("export round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_rows_and_downgrade() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Continuity Matrix"));
    assert!(summary.contains("editor_core"));
    assert!(summary.contains("Downgraded:"));
    assert!(summary.contains("mode_strip ="));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_keyboard_continuity_matrix_export()
        .expect("checked keyboard continuity export validates");
    assert_eq!(checked, packet());
}

#[test]
fn grade_ranks_are_strictly_ordered() {
    assert!(
        ContinuityParityGrade::SwitchingCertified.rank()
            > ContinuityParityGrade::ParityComplete.rank()
    );
    assert!(
        ContinuityParityGrade::ParityComplete.rank() > ContinuityParityGrade::ParityPartial.rank()
    );
    assert!(
        ContinuityParityGrade::ParityPartial.rank()
            > ContinuityParityGrade::ParityUnverified.rank()
    );
    assert!(
        ContinuityParityGrade::ParityUnverified.rank()
            > ContinuityParityGrade::NotApplicable.rank()
    );
}
