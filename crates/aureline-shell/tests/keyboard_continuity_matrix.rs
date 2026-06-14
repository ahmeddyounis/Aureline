use aureline_shell::freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix::{
    current_keyboard_continuity_matrix_export, AxisProofCurrency, ContinuityParityGrade,
    KeyboardContinuityMatrixPacket, KeyboardContinuityViolation, KeyboardSurfaceKind,
    ParityDowngradeTrigger, SequenceGuideClass, UndoClass,
};

fn fixture(name: &str) -> KeyboardContinuityMatrixPacket {
    let path = format!(
        "{}/../../fixtures/interaction/m5/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let contents = std::fs::read_to_string(path).expect("fixture should be readable");
    serde_json::from_str(&contents).expect("fixture should parse")
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_keyboard_continuity_matrix_export()
        .expect("checked-in keyboard continuity export should validate");
    assert!(packet.validate().is_empty());
    for kind in KeyboardSurfaceKind::ALL {
        assert!(
            packet.represented_surface_kinds().contains(&kind),
            "missing surface kind {}",
            kind.as_str()
        );
    }
    for undo in UndoClass::DISTINCT_CORE {
        assert!(
            packet.represented_undo_classes().contains(&undo),
            "missing distinct undo class {}",
            undo.as_str()
        );
    }
}

#[test]
fn stale_proof_drill_fixture_auto_downgrades() {
    let packet = fixture("editor_core_downgrades_on_stale_verification.json");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.downgraded_row_count(), 1);

    let downgraded = packet
        .rows
        .iter()
        .find(|row| row.needs_downgrade())
        .expect("downgraded row");
    assert_eq!(downgraded.surface_kind, KeyboardSurfaceKind::EditorCore);
    assert_eq!(
        downgraded.verification.proof_currency,
        AxisProofCurrency::StaleExpired
    );
    assert_eq!(
        downgraded.effective_grade,
        ContinuityParityGrade::ParityUnverified
    );
    assert!(
        downgraded.effective_grade.rank() < downgraded.claimed_grade.rank(),
        "downgraded row must rank strictly below its claim"
    );
    assert_eq!(
        downgraded.downgrade_trigger,
        Some(ParityDowngradeTrigger::StaleVerificationProof)
    );
}

#[test]
fn provider_row_never_reads_as_local() {
    let packet = fixture("editor_core_downgrades_on_stale_verification.json");
    let companion = packet
        .rows
        .iter()
        .find(|row| row.surface_kind == KeyboardSurfaceKind::CompanionSurface)
        .expect("companion row");
    assert!(companion.provider_or_imported());
    assert!(companion.imported_posture_consistent());
    assert!(companion.verification.backs_claim(true));
    assert!(!companion.verification.backs_claim(false));
}

#[test]
fn claimed_row_losing_sequence_support_must_downgrade() {
    let mut packet = fixture("editor_core_downgrades_on_stale_verification.json");
    // A docs row that keeps its full claim but loses sequence support must be
    // rejected: the row auto-downgrades before promotion.
    let docs = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_kind == KeyboardSurfaceKind::DocsSurface)
        .expect("docs row");
    docs.sequence_guide = SequenceGuideClass::SequenceUnsupportedDowngraded;
    let violations = packet.validate();
    assert!(
        violations.contains(&KeyboardContinuityViolation::RowNotDowngradedOnUnidentifiedBehavior)
    );
}
