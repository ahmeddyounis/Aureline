use aureline_shell::certify_keyboard_first_modal_parity_clipboard_drop_safety_grouped_history_honesty_and_orie::{
    current_interaction_parity_certification_export, AxisProofCurrency, ContinuityParityGrade,
    InteractionParityCertificationPacket, InteractionParityCertificationViolation,
    KeyboardSurfaceKind, ParityDimension, ParityDowngradeTrigger,
};

fn fixture(name: &str) -> InteractionParityCertificationPacket {
    let path = format!(
        "{}/../../fixtures/interaction/m5/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let contents = std::fs::read_to_string(path).expect("fixture should be readable");
    serde_json::from_str(&contents).expect("fixture should parse")
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_interaction_parity_certification_export()
        .expect("checked-in interaction parity certification export should validate");
    assert!(packet.validate().is_empty());
    for kind in KeyboardSurfaceKind::ALL {
        assert!(
            packet.represented_surface_kinds().contains(&kind),
            "missing surface kind {}",
            kind.as_str()
        );
    }
    for dimension in ParityDimension::ALL {
        assert!(
            packet.represented_dimensions().contains(&dimension),
            "missing dimension {}",
            dimension.as_str()
        );
    }
}

#[test]
fn narrow_drill_fixture_auto_narrows_on_stale_history() {
    let packet = fixture("editor_core_narrows_on_stale_history_proof.json");
    assert!(packet.validate().is_empty());

    let editor = packet
        .rows
        .iter()
        .find(|row| row.surface_kind == KeyboardSurfaceKind::EditorCore)
        .expect("editor-core row");
    assert!(editor.needs_narrow());
    assert_eq!(
        editor
            .certification(ParityDimension::GroupedHistoryContinuity)
            .map(|c| c.verification.proof_currency),
        Some(AxisProofCurrency::StaleExpired)
    );
    assert_eq!(
        editor.effective_grade,
        ContinuityParityGrade::ParityUnverified
    );
    assert!(editor.effective_grade.rank() < editor.claimed_grade.rank());
    assert_eq!(
        editor.narrow_trigger,
        Some(ParityDowngradeTrigger::StaleVerificationProof)
    );
}

#[test]
fn imported_companion_row_never_reads_as_local() {
    let packet = current_interaction_parity_certification_export().expect("export validates");
    let companion = packet
        .rows
        .iter()
        .find(|row| row.surface_kind == KeyboardSurfaceKind::CompanionSurface)
        .expect("companion row");
    assert!(companion.imported_surface);
    assert!(companion.subject.is_provider_or_imported());
    assert!(companion.imported_posture_consistent());
    for cert in &companion.certifications {
        assert_eq!(
            cert.verification.proof_currency,
            AxisProofCurrency::ImportedCurrent
        );
        assert!(cert.backs_claim(true));
        assert!(!cert.backs_claim(false));
    }
}

#[test]
fn claimed_row_losing_orientation_proof_must_narrow() {
    let mut packet = current_interaction_parity_certification_export().expect("export validates");
    let runtime = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_kind == KeyboardSurfaceKind::RuntimeSurface)
        .expect("runtime row");
    for cert in &mut runtime.certifications {
        if cert.dimension == ParityDimension::OrientationAidContinuity {
            cert.verification.proof_currency = AxisProofCurrency::StaleExpired;
        }
    }
    let violations = packet.validate();
    assert!(violations
        .contains(&InteractionParityCertificationViolation::RowNotNarrowedOnUncurrentProof));
}
