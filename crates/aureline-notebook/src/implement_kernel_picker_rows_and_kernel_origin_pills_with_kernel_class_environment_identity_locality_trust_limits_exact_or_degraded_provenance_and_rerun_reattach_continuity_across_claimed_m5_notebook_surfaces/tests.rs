use super::*;

const PACKET_ID: &str = KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_PACKET_ID;

fn packet() -> KernelPickerRowKernelOriginPillControlsPacket {
    seeded_kernel_picker_row_kernel_origin_pill_controls()
}

#[test]
fn seed_packet_validates() {
    let packet = packet();
    assert!(
        packet.validate().is_empty(),
        "seed packet failed validation: {:?}",
        packet.validate()
    );
    assert_eq!(packet.packet_id, PACKET_ID);
    assert_eq!(
        packet.record_kind,
        KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_SCHEMA_VERSION
    );
}

#[test]
fn choice_state_is_derived_not_asserted() {
    use KernelChoiceState as Class;
    use M5KernelCandidateKind as Kind;
    use M5KernelSelectionState as Sel;

    // Selected / recommended / available → selectable now.
    for (sel, class, current) in [
        (Sel::Selected, Class::CurrentlySelected, true),
        (Sel::Recommended, Class::RecommendedChoice, false),
        (Sel::Available, Class::AvailableChoice, false),
    ] {
        let d = resolve_kernel_picker_row(Kind::LocalInterpreter, sel);
        assert_eq!(d.choice_state, class);
        assert!(d.is_selectable_now);
        assert_eq!(d.is_current, current);
    }

    // Incompatible → not selectable, needs incompatible note.
    let d = resolve_kernel_picker_row(Kind::ContainerKernel, Sel::Incompatible);
    assert_eq!(d.choice_state, Class::IncompatibleChoice);
    assert!(!d.is_selectable_now);
    assert!(d.needs_incompatible_note);

    // Unavailable → not selectable, needs unavailable note.
    let d = resolve_kernel_picker_row(Kind::ManagedKernel, Sel::Unavailable);
    assert_eq!(d.choice_state, Class::UnavailableChoice);
    assert!(!d.is_selectable_now);
    assert!(d.needs_unavailable_note);

    // Needs install → not selectable, needs install note.
    let d = resolve_kernel_picker_row(Kind::RemoteKernel, Sel::NeedsInstall);
    assert_eq!(d.choice_state, Class::NeedsSetupFirst);
    assert!(!d.is_selectable_now);
    assert!(d.needs_install_note);
}

#[test]
fn provenance_and_continuity_are_derived_not_asserted() {
    use KernelFingerprintState as Fp;
    use KernelProvenanceClass as Class;
    use M5KernelOriginClass as Origin;
    use M5KernelOriginTrustState as Trust;

    // Trusted / first-party → exact provenance.
    for trust in [Trust::TrustedOrigin, Trust::FirstParty] {
        let d = resolve_kernel_origin_pill(Origin::LocalHost, trust, Fp::FingerprintMatched);
        assert_eq!(d.provenance_class, Class::ExactProvenance);
        assert!(d.is_exact_provenance);
        // Matched fingerprint + exact provenance → may claim exact continuity.
        assert!(d.may_claim_exact_continuity);
    }

    // Third-party / unverified → degraded, needs degraded note.
    for trust in [Trust::ThirdParty, Trust::UnverifiedOrigin] {
        let d = resolve_kernel_origin_pill(Origin::Container, trust, Fp::FingerprintMatched);
        assert_eq!(d.provenance_class, Class::DegradedProvenance);
        assert!(!d.is_exact_provenance);
        assert!(d.needs_degraded_note);
        // Even with a matched fingerprint, degraded provenance cannot claim exact continuity.
        assert!(!d.may_claim_exact_continuity);
    }

    // Restricted → restricted, needs restricted note.
    let d = resolve_kernel_origin_pill(
        Origin::ManagedWorkspace,
        Trust::RestrictedOrigin,
        Fp::FingerprintUnknown,
    );
    assert_eq!(d.provenance_class, Class::RestrictedProvenance);
    assert!(d.needs_restricted_note);

    // Unknown → unknown, needs unknown-origin note.
    let d = resolve_kernel_origin_pill(
        Origin::BrowserBridge,
        Trust::UnknownOrigin,
        Fp::FingerprintNotEvaluated,
    );
    assert_eq!(d.provenance_class, Class::UnknownProvenance);
    assert!(d.needs_unknown_origin_note);

    // A drifted fingerprint blocks exact continuity even for an exact origin, and needs a drift
    // note.
    let d = resolve_kernel_origin_pill(
        Origin::LocalHost,
        Trust::TrustedOrigin,
        Fp::FingerprintDrifted,
    );
    assert!(d.is_exact_provenance);
    assert!(!d.may_claim_exact_continuity);
    assert!(d.needs_drift_note);

    // Local host is flagged as a local origin.
    let d =
        resolve_kernel_origin_pill(Origin::LocalHost, Trust::FirstParty, Fp::FingerprintMatched);
    assert!(d.is_local_origin);
    let d =
        resolve_kernel_origin_pill(Origin::SshRemote, Trust::FirstParty, Fp::FingerprintMatched);
    assert!(!d.is_local_origin);
}

#[test]
fn picker_kind_selection_and_choice_coverage_is_complete() {
    let packet = packet();
    let kinds: std::collections::BTreeSet<_> = packet
        .picker_rows
        .iter()
        .map(|r| r.candidate_kind)
        .collect();
    for kind in M5KernelCandidateKind::ALL {
        assert!(kinds.contains(&kind), "missing candidate kind {kind:?}");
    }
    let selections: std::collections::BTreeSet<_> = packet
        .picker_rows
        .iter()
        .map(|r| r.selection_state)
        .collect();
    for sel in M5KernelSelectionState::ALL {
        assert!(selections.contains(&sel), "missing selection state {sel:?}");
    }
    let choices: std::collections::BTreeSet<_> = packet
        .picker_rows
        .iter()
        .map(|r| r.choice_disclosure().choice_state)
        .collect();
    for class in KernelChoiceState::ALL {
        assert!(choices.contains(&class), "missing choice state {class:?}");
    }
}

#[test]
fn pill_origin_trust_provenance_and_fingerprint_coverage_is_complete() {
    let packet = packet();
    let origins: std::collections::BTreeSet<_> =
        packet.origin_pills.iter().map(|p| p.origin_class).collect();
    for origin in M5KernelOriginClass::ALL {
        assert!(origins.contains(&origin), "missing origin class {origin:?}");
    }
    let trusts: std::collections::BTreeSet<_> =
        packet.origin_pills.iter().map(|p| p.trust_state).collect();
    for trust in M5KernelOriginTrustState::ALL {
        assert!(trusts.contains(&trust), "missing trust state {trust:?}");
    }
    let provenance: std::collections::BTreeSet<_> = packet
        .origin_pills
        .iter()
        .map(|p| p.origin_disclosure().provenance_class)
        .collect();
    for class in KernelProvenanceClass::ALL {
        assert!(
            provenance.contains(&class),
            "missing provenance class {class:?}"
        );
    }
    let fingerprints: std::collections::BTreeSet<_> = packet
        .origin_pills
        .iter()
        .map(|p| p.fingerprint_state)
        .collect();
    for fp in KernelFingerprintState::ALL {
        assert!(
            fingerprints.contains(&fp),
            "missing fingerprint state {fp:?}"
        );
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::MissingSourceContracts));
}

#[test]
fn empty_picker_rows_fails() {
    let mut packet = packet();
    packet.picker_rows.clear();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::PickerRowsMissing));
}

#[test]
fn empty_origin_pills_fails() {
    let mut packet = packet();
    packet.origin_pills.clear();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::OriginPillsMissing));
}

#[test]
fn picker_wrong_component_class_fails() {
    let mut packet = packet();
    packet.picker_rows[0].component = M5NotebookKernelOutputComponentFamily::KernelOriginPill;
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::PickerRowWrongComponentClass));
}

#[test]
fn pill_wrong_component_class_fails() {
    let mut packet = packet();
    packet.origin_pills[0].component = M5NotebookKernelOutputComponentFamily::KernelPickerRow;
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::OriginPillWrongComponentClass));
}

#[test]
fn incompatible_row_claiming_selectable_fails() {
    let mut packet = packet();
    let row = packet
        .picker_rows
        .iter_mut()
        .find(|r| r.choice_state == KernelChoiceState::IncompatibleChoice)
        .expect("incompatible row present");
    row.claims_selectable_now = true;
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::ChoiceStateMisrepresented));
}

#[test]
fn degraded_pill_claiming_exact_provenance_fails() {
    let mut packet = packet();
    let pill = packet
        .origin_pills
        .iter_mut()
        .find(|p| p.provenance_class == KernelProvenanceClass::DegradedProvenance)
        .expect("degraded pill present");
    pill.claims_exact_provenance = true;
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::ProvenanceMisrepresented));
}

#[test]
fn pill_overclaiming_exact_continuity_fails() {
    let mut packet = packet();
    // A drifted / degraded pill cannot claim exact continuity.
    let pill = packet
        .origin_pills
        .iter_mut()
        .find(|p| !p.origin_disclosure().may_claim_exact_continuity)
        .expect("non-continuity pill present");
    pill.claims_exact_continuity = true;
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::ExactContinuityOverclaimed));
}

#[test]
fn missing_incompatible_note_fails() {
    let mut packet = packet();
    let row = packet
        .picker_rows
        .iter_mut()
        .find(|r| r.choice_state == KernelChoiceState::IncompatibleChoice)
        .expect("incompatible row present");
    row.incompatible_note.clear();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::IncompatibleNoteMissing));
}

#[test]
fn missing_install_note_fails() {
    let mut packet = packet();
    let row = packet
        .picker_rows
        .iter_mut()
        .find(|r| r.choice_state == KernelChoiceState::NeedsSetupFirst)
        .expect("needs-install row present");
    row.install_note.clear();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::InstallNoteMissing));
}

#[test]
fn missing_degraded_note_fails() {
    let mut packet = packet();
    let pill = packet
        .origin_pills
        .iter_mut()
        .find(|p| p.provenance_class == KernelProvenanceClass::DegradedProvenance)
        .expect("degraded pill present");
    pill.degraded_note.clear();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::DegradedNoteMissing));
}

#[test]
fn missing_drift_note_fails() {
    let mut packet = packet();
    let pill = packet
        .origin_pills
        .iter_mut()
        .find(|p| p.origin_disclosure().needs_drift_note)
        .expect("drifted pill present");
    pill.drift_note.clear();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::DriftNoteMissing));
}

#[test]
fn missing_kernel_class_label_fails() {
    let mut packet = packet();
    packet.picker_rows[0].kernel_class_label.clear();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::KernelClassLabelMissing));
}

#[test]
fn missing_trust_policy_limit_note_fails() {
    let mut packet = packet();
    packet.picker_rows[0].trust_policy_limit_note.clear();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::TrustPolicyLimitNoteMissing));
}

#[test]
fn missing_origin_label_fails() {
    let mut packet = packet();
    packet.origin_pills[0].origin_label.clear();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::OriginLabelMissing));
}

#[test]
fn missing_continuity_note_fails() {
    let mut packet = packet();
    packet.origin_pills[0].continuity_note.clear();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::ContinuityNoteMissing));
}

#[test]
fn picker_missing_view_compatibility_action_fails() {
    let mut packet = packet();
    packet.picker_rows[0].picker_actions = vec![KernelPickerAction::ChooseKernel];
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::PickerActionsIncomplete));
}

#[test]
fn pill_missing_copy_action_fails() {
    let mut packet = packet();
    packet.origin_pills[0].pill_actions = vec![KernelPillAction::InspectOrigin];
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::PillActionsIncomplete));
}

#[test]
fn deep_link_action_without_target_fails() {
    let mut packet = packet();
    packet.picker_rows[0].deep_link_kind = DeepLinkKind::NoDeepLink;
    packet.picker_rows[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::DeepLinkUnresolved));
}

#[test]
fn resolvable_deep_link_without_ref_fails() {
    let mut packet = packet();
    packet.picker_rows[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::DeepLinkRefMissing));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = packet();
    packet.origin_pills[0].context_note.clear();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::ContextNoteMissing));
}

#[test]
fn missing_dispositions_fails() {
    let mut packet = packet();
    packet.picker_rows[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::DispositionsMissing));
}

#[test]
fn picker_collapsing_kernel_origins_fails() {
    let mut packet = packet();
    packet.picker_rows[0].collapses_kernel_origins_into_one_badge = true;
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::KernelOriginsCollapsed));
}

#[test]
fn pill_implying_exact_continuity_on_drift_fails() {
    let mut packet = packet();
    packet.origin_pills[0].implies_exact_continuity_on_material_drift = true;
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::ExactContinuityImpliedOnDrift));
}

#[test]
fn pill_hiding_trust_behind_hover_only_fails() {
    let mut packet = packet();
    packet.origin_pills[0].hides_trust_or_compatibility_behind_hover_only = true;
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::TrustOrCompatibilityHoverOnly));
}

#[test]
fn pill_overwriting_provenance_without_review_fails() {
    let mut packet = packet();
    packet.origin_pills[0].overwrites_provenance_without_review = true;
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::ProvenanceOverwrittenWithoutReview));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.picker_rows[0].required_labels = vec![M5NotebookKernelOutputRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.origin_pills[0].accessibility_routes =
        vec![M5NotebookKernelOutputAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::AccessibilityRouteMissing));
}

#[test]
fn kernel_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .kernel_review
        .exact_continuity_never_implied_on_material_drift = false;
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::KernelReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .support_export_shows_kernel_origin = false;
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.picker_rows[0].deep_link_ref = "see https://internal.example/kernel".to_owned();
    assert!(packet
        .validate()
        .contains(&KernelPickerRowKernelOriginPillViolation::RawMaterialInExport));
}

#[test]
fn markdown_summary_lists_components() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Kernel picker rows"));
    assert!(summary.contains("## Kernel origin pills"));
    assert!(summary.contains("incompatible_choice"));
    assert!(summary.contains("degraded_provenance"));
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 picker rows + 6 origin pills
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("kernel_picker_row"));
    assert!(csv.contains("kernel_origin_pill"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_kernel_picker_row_kernel_origin_pill_export()
        .expect("checked kernel picker row kernel origin pill export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-kernel-picker-row-kernel-origin-pill-controls/kernel_picker_row_incompatible.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-kernel-picker-row-kernel-origin-pill-controls/kernel_origin_pill_degraded.json"
        )),
    ] {
        let packet: KernelPickerRowKernelOriginPillControlsPacket = serde_json::from_str(raw)
            .expect("fixture parses as kernel picker row kernel origin pill packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn scenario_fixtures_stay_valid_and_covered() {
    for packet in [
        seeded_kernel_picker_row_kernel_origin_pill_controls_kernel_picker_row_incompatible(),
        seeded_kernel_picker_row_kernel_origin_pill_controls_kernel_origin_pill_degraded(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
