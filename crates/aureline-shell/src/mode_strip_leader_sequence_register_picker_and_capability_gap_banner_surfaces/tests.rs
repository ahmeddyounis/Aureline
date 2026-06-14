use super::*;

fn packet() -> ModeStripSurfacePacket {
    seeded_mode_strip_surface_packet()
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
fn seeded_packet_covers_every_required_surface_kind() {
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
fn seeded_packet_covers_every_imported_modal_preset() {
    let presets = packet().represented_source_presets();
    for required in KeymapSourcePreset::MODAL_PRESETS {
        assert!(
            presets.contains(&required),
            "missing modal source preset {}",
            required.as_str()
        );
    }
}

#[test]
fn missing_modal_preset_is_rejected() {
    let mut packet = packet();
    // Re-target every Helix strip to a non-modal default so the Helix preset is no
    // longer represented; the switching-wedge coverage check must reject the packet.
    for strip in &mut packet.strips {
        if strip.source_preset == KeymapSourcePreset::HelixPreset {
            strip.source_preset = KeymapSourcePreset::NonModalDefault;
        }
    }
    assert!(packet
        .validate()
        .contains(&ModeStripViolation::SourcePresetCoverageMissing));
}

#[test]
fn exactly_one_seeded_strip_downgrades() {
    let packet = packet();
    assert_eq!(packet.downgraded_strip_count(), 1);
    let downgraded = packet
        .strips
        .iter()
        .find(|strip| strip.needs_downgrade())
        .expect("a downgraded strip");
    assert_eq!(downgraded.surface_kind, KeyboardSurfaceKind::DataApiSurface);
    assert_eq!(
        downgraded.pending_sequence.resolution,
        SequenceResolution::UnsupportedDowngraded
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
    // The downgrade is explained by a capability-gap banner, never silent.
    assert!(!downgraded.capability_gaps.is_empty());
}

#[test]
fn downgraded_strip_must_carry_a_capability_gap_banner() {
    let mut packet = packet();
    let strip = packet
        .strips
        .iter_mut()
        .find(|strip| strip.needs_downgrade())
        .expect("downgraded strip");
    // Stripping the explanatory banner means the surface narrowed silently.
    strip.capability_gaps.clear();
    let violations = packet.validate();
    assert!(violations.contains(&ModeStripViolation::CapabilityGapMissingForNarrowing));
}

#[test]
fn unsupported_sequence_never_silently_approximated() {
    let mut packet = packet();
    // Pretend the downgraded strip silently "approximated" the sequence by claiming
    // full parity again without lowering its effective grade.
    let strip = packet
        .strips
        .iter_mut()
        .find(|strip| strip.pending_sequence.is_downgraded())
        .expect("unsupported-sequence strip");
    strip.effective_grade = ContinuityParityGrade::ParityComplete;
    let violations = packet.validate();
    assert!(violations.contains(&ModeStripViolation::SequenceSilentlyApproximated));
    assert!(violations.contains(&ModeStripViolation::StripNotDowngradedOnUnidentifiedBehavior));
}

#[test]
fn unidentified_mode_must_downgrade() {
    let mut packet = packet();
    let strip = packet
        .strips
        .iter_mut()
        .find(|strip| strip.strip_id == "mode-strip:notebook:0001")
        .expect("notebook strip");
    // A surface that cannot identify its mode keeps its full claim: it must
    // auto-downgrade, so validation rejects the unchanged effective grade.
    strip.current_mode = ModeIndicator::ModeUnknownDowngraded;
    assert!(strip.needs_downgrade());
    let violations = packet.validate();
    assert!(violations.contains(&ModeStripViolation::CurrentModeNotVisible));
}

#[test]
fn claimed_strip_losing_current_proof_must_downgrade() {
    let mut packet = packet();
    let editor = packet
        .strips
        .iter_mut()
        .find(|strip| strip.strip_id == "mode-strip:editor-core:0001")
        .expect("editor core strip");
    editor.verification.proof_currency = AxisProofCurrency::StaleExpired;
    assert!(editor.needs_downgrade());
    let violations = packet.validate();
    assert!(violations.contains(&ModeStripViolation::StripNotDowngradedOnUnidentifiedBehavior));
}

#[test]
fn rich_only_clipboard_route_is_rejected_when_not_downgraded() {
    let mut packet = packet();
    let strip = packet
        .strips
        .iter_mut()
        .find(|strip| strip.strip_id == "mode-strip:docs:0001")
        .expect("docs strip");
    // A rich-only route loses plain text; if the strip keeps its full claim it must
    // be rejected.
    strip.register_picker.clipboard_route = ClipboardRouteClass::RichOnlyDenied;
    strip.register_picker.plain_text_representation_available = false;
    let violations = packet.validate();
    assert!(violations.contains(&ModeStripViolation::ClipboardPlainTextLost));
}

#[test]
fn capability_gap_must_be_keyboard_and_screen_reader_reachable() {
    let mut packet = packet();
    let strip = packet
        .strips
        .iter_mut()
        .find(|strip| !strip.capability_gaps.is_empty())
        .expect("a strip with a capability gap");
    strip.capability_gaps[0].hover_only = true;
    let violations = packet.validate();
    assert!(violations.contains(&ModeStripViolation::CapabilityGapNotReachable));
}

#[test]
fn mode_changes_must_be_reachable() {
    let mut packet = packet();
    let strip = packet
        .strips
        .iter_mut()
        .find(|strip| strip.strip_id == "mode-strip:preview:0001")
        .expect("preview strip");
    strip.accessibility.keyboard_reachable = false;
    let violations = packet.validate();
    assert!(violations.contains(&ModeStripViolation::ModeChangesNotReachable));
}

#[test]
fn pending_sequence_state_is_consistent() {
    let packet = packet();
    let docs = packet
        .strips
        .iter()
        .find(|strip| strip.strip_id == "mode-strip:docs:0001")
        .expect("docs strip");
    // A live pending operator + count prefix implies awaiting input with a real
    // timeout/resolution posture.
    assert!(docs.pending_sequence.has_pending_input());
    assert!(docs.pending_sequence.awaiting_more_input);
    assert_eq!(docs.pending_sequence.count_prefix, Some(2));
    assert!(docs.pending_sequence.is_well_formed());
}

#[test]
fn provider_strip_proof_never_reads_as_local() {
    let packet = packet();
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
fn provider_strip_with_local_proof_is_rejected() {
    let mut packet = packet();
    let companion = packet
        .strips
        .iter_mut()
        .find(|strip| strip.surface_kind == KeyboardSurfaceKind::CompanionSurface)
        .expect("companion strip");
    companion.verification.proof_currency = AxisProofCurrency::VerifiedCurrent;
    let violations = packet.validate();
    assert!(violations.contains(&ModeStripViolation::ImportedReadsAsLocal));
}

#[test]
fn surface_must_stay_keyboard_complete() {
    let mut packet = packet();
    let strip = packet
        .strips
        .iter_mut()
        .find(|strip| strip.strip_id == "mode-strip:notebook:0001")
        .expect("notebook strip");
    strip.keyboard_complete = false;
    let violations = packet.validate();
    assert!(violations.contains(&ModeStripViolation::SurfaceNotKeyboardComplete));
}

#[test]
fn macro_replay_must_be_explicit() {
    let mut packet = packet();
    let strip = packet
        .strips
        .iter_mut()
        .find(|strip| strip.strip_id == "mode-strip:editor-core:0001")
        .expect("editor core strip");
    strip.macro_replay_explicit = false;
    let violations = packet.validate();
    assert!(violations.contains(&ModeStripViolation::MacroReplayNotExplicit));
}

#[test]
fn named_register_route_requires_a_register() {
    let mut packet = packet();
    let strip = packet
        .strips
        .iter_mut()
        .find(|strip| strip.strip_id == "mode-strip:notebook:0001")
        .expect("notebook strip");
    strip.register_picker.active_register_token = None;
    strip.register_picker.available_register_tokens.clear();
    assert!(!strip.register_picker.is_well_formed());
    let violations = packet.validate();
    assert!(violations.contains(&ModeStripViolation::StripIncomplete));
}

#[test]
fn fingerprint_must_be_independent_of_id() {
    let mut packet = packet();
    let strip = packet
        .strips
        .iter_mut()
        .find(|strip| strip.strip_id == "mode-strip:docs:0001")
        .expect("docs strip");
    strip.subject.surface_fingerprint_token = strip.subject.surface_id.clone();
    let violations = packet.validate();
    assert!(violations.contains(&ModeStripViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn downgraded_label_must_not_be_generic() {
    let mut packet = packet();
    let strip = packet
        .strips
        .iter_mut()
        .find(|strip| strip.needs_downgrade())
        .expect("downgraded strip");
    strip.downgraded_label = Some("error".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&ModeStripViolation::DowngradedStripMissingLabelOrTrigger));
}

#[test]
fn capability_gap_explanation_must_not_be_generic() {
    let mut packet = packet();
    let strip = packet
        .strips
        .iter_mut()
        .find(|strip| !strip.capability_gaps.is_empty())
        .expect("a strip with a capability gap");
    strip.capability_gaps[0].explanation = "unsupported".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&ModeStripViolation::StripIncomplete));
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = packet();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&ModeStripViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|r| r != FROZEN_KEYBOARD_CONTINUITY_MATRIX_REF);
    assert!(packet
        .validate()
        .contains(&ModeStripViolation::MissingSourceContracts));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: ModeStripSurfacePacket = serde_json::from_str(&json).expect("export round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_strips_and_downgrade() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Capability-Gap Surfaces"));
    assert!(summary.contains("source_preset ="));
    assert!(summary.contains("capability gap"));
    assert!(summary.contains("Downgraded:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked =
        current_mode_strip_surface_export().expect("checked mode strip surface export validates");
    assert_eq!(checked, packet());
}
