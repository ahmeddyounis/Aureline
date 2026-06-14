use super::*;

fn packet() -> InteractionParityCertificationPacket {
    seeded_interaction_parity_certification_packet()
}

#[test]
fn seeded_packet_validates() {
    let packet = packet();
    assert!(
        packet.validate().is_empty(),
        "seed must validate: {:?}",
        packet.validate()
    );
}

#[test]
fn fixture_packet_validates() {
    let packet = fixture_interaction_parity_certification_packet();
    assert!(
        packet.validate().is_empty(),
        "fixture must validate: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_kind() {
    let packet = packet();
    let kinds = packet.represented_surface_kinds();
    for kind in KeyboardSurfaceKind::ALL {
        assert!(
            kinds.contains(&kind),
            "missing surface kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn seeded_packet_covers_every_dimension() {
    let packet = packet();
    let dimensions = packet.represented_dimensions();
    for dimension in ParityDimension::ALL {
        assert!(
            dimensions.contains(&dimension),
            "missing dimension {}",
            dimension.as_str()
        );
    }
}

#[test]
fn seeded_packet_has_a_narrowed_row() {
    let packet = packet();
    assert!(packet.narrowed_row_count() >= 1);
    let narrowed = packet
        .rows
        .iter()
        .find(|row| row.needs_narrow())
        .expect("seed carries a narrowed row");
    assert!(narrowed.effective_grade.rank() < narrowed.claimed_grade.rank());
    assert!(narrowed.narrow_trigger.is_some());
    assert!(narrowed.narrow_consistent());
}

#[test]
fn required_core_dimensions_are_four() {
    assert_eq!(ParityDimension::REQUIRED_CORE.len(), 4);
    assert!(!ParityDimension::MacroReplaySafety.is_core());
    for dimension in ParityDimension::REQUIRED_CORE {
        assert!(dimension.is_core());
    }
}

#[test]
fn imported_row_never_reads_as_local() {
    let packet = packet();
    let imported = packet
        .rows
        .iter()
        .find(|row| row.imported_surface)
        .expect("seed carries an imported row");
    assert!(imported.subject.is_provider_or_imported());
    assert!(imported.imported_posture_consistent());
    for cert in &imported.certifications {
        assert_eq!(
            cert.verification.proof_currency,
            AxisProofCurrency::ImportedCurrent
        );
        assert!(cert.backs_claim(true));
        assert!(!cert.backs_claim(false));
    }
}

#[test]
fn claimed_row_losing_history_proof_must_narrow() {
    let mut packet = packet();
    let editor = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_kind == KeyboardSurfaceKind::EditorCore)
        .expect("editor-core row");
    for cert in &mut editor.certifications {
        if cert.dimension == ParityDimension::GroupedHistoryContinuity {
            cert.verification.proof_currency = AxisProofCurrency::StaleExpired;
        }
    }
    let violations = packet.validate();
    assert!(violations
        .contains(&InteractionParityCertificationViolation::RowNotNarrowedOnUncurrentProof));
}

#[test]
fn missing_required_core_dimension_forces_narrow() {
    let mut packet = packet();
    let notebook = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_kind == KeyboardSurfaceKind::NotebookSurface)
        .expect("notebook row");
    notebook
        .certifications
        .retain(|c| c.dimension != ParityDimension::OrientationAidContinuity);
    // The row still claims parity-complete, so it must be rejected for not
    // narrowing despite a missing required-core dimension.
    assert!(notebook.needs_narrow());
    let violations = packet.validate();
    assert!(violations
        .contains(&InteractionParityCertificationViolation::RowNotNarrowedOnUncurrentProof));
}

#[test]
fn silent_modal_approximation_is_rejected() {
    let mut packet = packet();
    packet.rows[0].modal_sequences_never_silently_approximated = false;
    let violations = packet.validate();
    assert!(
        violations.contains(&InteractionParityCertificationViolation::ModalSequenceApproximated)
    );
}

#[test]
fn rich_only_copy_is_rejected() {
    let mut packet = packet();
    packet.rows[0].plain_text_copy_preserved = false;
    let violations = packet.validate();
    assert!(violations.contains(&InteractionParityCertificationViolation::PlainTextCopyLost));
}

#[test]
fn hidden_drag_drop_verb_is_rejected() {
    let mut packet = packet();
    packet.rows[0].drag_drop_verbs_and_scope_disclosed = false;
    let violations = packet.validate();
    assert!(violations.contains(&InteractionParityCertificationViolation::DragDropVerbHidden));
}

#[test]
fn flattened_undo_classes_are_rejected() {
    let mut packet = packet();
    packet.rows[0].undo_classes_distinct = false;
    let violations = packet.validate();
    assert!(violations.contains(&InteractionParityCertificationViolation::UndoClassesFlattened));
}

#[test]
fn dropped_orientation_truth_is_rejected() {
    let mut packet = packet();
    packet.rows[0].orientation_aids_degrade_honestly = false;
    let violations = packet.validate();
    assert!(violations.contains(&InteractionParityCertificationViolation::OrientationTruthDropped));
}

#[test]
fn generic_narrow_label_is_rejected() {
    let mut packet = packet();
    let narrowed = packet
        .rows
        .iter_mut()
        .find(|row| row.needs_narrow())
        .expect("narrowed row");
    narrowed.narrowed_label = Some("unavailable".to_owned());
    let violations = packet.validate();
    assert!(violations
        .contains(&InteractionParityCertificationViolation::NarrowedRowMissingLabelOrTrigger));
}

#[test]
fn fingerprint_must_not_substitute_identity() {
    let mut packet = packet();
    let row = &mut packet.rows[0];
    row.subject.surface_fingerprint_token = row.subject.surface_id.clone();
    let violations = packet.validate();
    assert!(violations
        .contains(&InteractionParityCertificationViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: InteractionParityCertificationPacket =
        serde_json::from_str(&json).expect("round trips");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_rows_and_dimensions() {
    let packet = packet();
    let md = packet.render_markdown_summary();
    assert!(md.contains("interaction-cert:editor-core:0001"));
    assert!(md.contains("modal_keyboard_parity"));
    assert!(md.contains("Narrowed:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_interaction_parity_certification_export()
        .expect("checked interaction parity export validates");
    assert_eq!(checked, packet());
}
