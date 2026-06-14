use aureline_shell::mode_strip_leader_sequence_register_picker_and_capability_gap_banner_surfaces::{
    current_mode_strip_surface_export, AxisProofCurrency, CapabilityGapKind, ContinuityParityGrade,
    KeyboardSurfaceKind, KeymapSourcePreset, ModeStripSurfacePacket, ModeStripViolation,
    ParityDowngradeTrigger, SequenceResolution,
};

fn fixture(name: &str) -> ModeStripSurfacePacket {
    let path = format!(
        "{}/../../fixtures/interaction/m5/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let contents = std::fs::read_to_string(path).expect("fixture should be readable");
    serde_json::from_str(&contents).expect("fixture should parse")
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_mode_strip_surface_export()
        .expect("checked-in mode strip surface export should validate");
    assert!(packet.validate().is_empty());
    for kind in [
        KeyboardSurfaceKind::NotebookSurface,
        KeyboardSurfaceKind::DataApiSurface,
        KeyboardSurfaceKind::PreviewSurface,
        KeyboardSurfaceKind::DocsSurface,
        KeyboardSurfaceKind::CompanionSurface,
    ] {
        assert!(
            packet.represented_surface_kinds().contains(&kind),
            "missing surface kind {}",
            kind.as_str()
        );
    }
    for preset in KeymapSourcePreset::MODAL_PRESETS {
        assert!(
            packet.represented_source_presets().contains(&preset),
            "missing modal source preset {}",
            preset.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_reconstructs_capability_gap_posture() {
    let packet = current_mode_strip_surface_export().expect("export should validate");
    // Support / migration tooling can reconstruct exactly which surfaces narrowed or
    // rejected an affordance and why.
    assert!(packet
        .represented_gap_kinds()
        .contains(&CapabilityGapKind::ModalSequenceUnsupported));
    let downgraded = packet
        .strips
        .iter()
        .find(|strip| strip.needs_downgrade())
        .expect("a downgraded strip");
    assert!(!downgraded.capability_gaps.is_empty());
    let gap = &downgraded.capability_gaps[0];
    assert!(gap.is_reachable());
    assert!(!gap.export_safe_reason_token.is_empty());
}

#[test]
fn stale_proof_drill_fixture_auto_downgrades() {
    let packet = fixture("notebook_downgrades_on_stale_verification.json");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // Two strips downgrade: the notebook on stale proof, and the carried data-grid
    // strip that imported an unsupported leader sequence.
    assert_eq!(packet.downgraded_strip_count(), 2);

    let downgraded = packet
        .strip("mode-strip:notebook:0001")
        .expect("notebook strip");
    assert!(downgraded.needs_downgrade());
    assert_eq!(
        downgraded.surface_kind,
        KeyboardSurfaceKind::NotebookSurface
    );
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
        "downgraded strip must rank strictly below its claim"
    );
    assert_eq!(
        downgraded.downgrade_trigger,
        Some(ParityDowngradeTrigger::StaleVerificationProof)
    );
}

#[test]
fn provider_strip_never_reads_as_local() {
    let packet = fixture("notebook_downgrades_on_stale_verification.json");
    let companion = packet
        .strips
        .iter()
        .find(|strip| strip.surface_kind == KeyboardSurfaceKind::CompanionSurface)
        .expect("companion strip");
    assert!(companion.provider_or_imported());
    assert!(companion.imported_posture_consistent());
    assert!(companion.verification.backs_claim(true));
    assert!(!companion.verification.backs_claim(false));
}

#[test]
fn claimed_strip_losing_sequence_support_must_downgrade() {
    let mut packet = fixture("notebook_downgrades_on_stale_verification.json");
    // A docs strip that keeps its full claim but loses sequence support must be
    // rejected: the strip auto-downgrades before promotion.
    let docs = packet
        .strips
        .iter_mut()
        .find(|strip| strip.surface_kind == KeyboardSurfaceKind::DocsSurface)
        .expect("docs strip");
    docs.pending_sequence.resolution = SequenceResolution::UnsupportedDowngraded;
    docs.pending_sequence.sequence_guide =
        aureline_shell::freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix::SequenceGuideClass::SequenceUnsupportedDowngraded;
    let violations = packet.validate();
    assert!(violations.contains(&ModeStripViolation::StripNotDowngradedOnUnidentifiedBehavior));
}
