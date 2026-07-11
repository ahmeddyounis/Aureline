use super::*;

const PACKET_ID: &str = OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_PACKET_ID;

fn packet() -> OutputTrustBannerOutputProvenanceChipGroupControlsPacket {
    seeded_output_trust_banner_output_provenance_chip_group_controls()
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
        OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_VERSION
    );
}

#[test]
fn presentation_class_is_derived_not_asserted() {
    use M5OutputFreshnessState as Fresh;
    use M5OutputTrustClass as Trust;
    use OutputTrustPresentationClass as Class;

    // Trust class maps 1:1 to a presentation class.
    for (trust, class, active) in [
        (Trust::TrustedOutput, Class::TrustedLocalActive, true),
        (Trust::SanitizedOutput, Class::SanitizedRich, false),
        (Trust::SandboxedOutput, Class::IsolatedRemoteActive, true),
        (Trust::RawActiveOutput, Class::PlainText, false),
        (Trust::BlockedOutput, Class::BlockedContent, false),
        (Trust::UnknownTrust, Class::UnknownContent, false),
    ] {
        let d = resolve_output_trust_banner(trust, Fresh::LiveOutput);
        assert_eq!(d.presentation_class, class);
        assert_eq!(d.is_active_content, active);
    }

    // Only a live output may present as live.
    let d = resolve_output_trust_banner(Trust::TrustedOutput, Fresh::LiveOutput);
    assert!(d.is_live);
    assert!(d.may_present_as_live);

    // Stale / cached / cleared / superseded / no output are never live and each carries a note.
    let d = resolve_output_trust_banner(Trust::SanitizedOutput, Fresh::StaleOutput);
    assert!(!d.is_live);
    assert!(!d.may_present_as_live);
    assert!(d.needs_stale_note);

    let d = resolve_output_trust_banner(Trust::SandboxedOutput, Fresh::CachedOutput);
    assert!(!d.may_present_as_live);
    assert!(d.needs_cached_note);

    let d = resolve_output_trust_banner(Trust::RawActiveOutput, Fresh::SupersededOutput);
    assert!(!d.may_present_as_live);
    assert!(d.needs_stale_note);

    let d = resolve_output_trust_banner(Trust::BlockedOutput, Fresh::ClearedOutput);
    assert!(!d.may_present_as_live);
    assert!(d.needs_cleared_note);
    assert!(d.needs_blocked_note);

    let d = resolve_output_trust_banner(Trust::UnknownTrust, Fresh::NoOutput);
    assert!(!d.may_present_as_live);
    assert!(d.needs_cleared_note);
    assert!(d.needs_unknown_trust_note);

    // Isolated remote active content is flagged as isolated.
    let d = resolve_output_trust_banner(Trust::SandboxedOutput, Fresh::LiveOutput);
    assert!(d.is_isolated_active);
    assert!(d.needs_isolation_note);
}

#[test]
fn origin_and_lineage_are_derived_not_asserted() {
    use M5OutputProvenanceKind as Kind;
    use M5OutputProvenanceState as State;
    use OutputOriginClass as Origin;
    use OutputProvenanceResolution as Resolution;

    // Cell / run produced → internal, may claim current lineage when complete / pinned.
    let d = resolve_output_provenance_chip_group(Kind::ProducedByCell, State::ProvenanceComplete);
    assert_eq!(d.origin_class, Origin::CellProduced);
    assert_eq!(d.resolution_class, Resolution::FullyResolved);
    assert!(d.is_internal_origin);
    assert!(d.may_claim_current_lineage);

    let d = resolve_output_provenance_chip_group(Kind::ProducedByRun, State::ExecutionCountPinned);
    assert_eq!(d.origin_class, Origin::RunProduced);
    assert_eq!(d.resolution_class, Resolution::LineagePinned);
    assert!(d.may_claim_current_lineage);

    // Imported / restored / external / unknown → external, needs external note.
    for (kind, origin) in [
        (Kind::ImportedOutput, Origin::ImportedOrigin),
        (Kind::RestoredOutput, Origin::RestoredOrigin),
        (Kind::ExternalOutput, Origin::ExternalOrigin),
        (Kind::UnknownProvenance, Origin::UnknownOrigin),
    ] {
        let d = resolve_output_provenance_chip_group(kind, State::ProvenanceComplete);
        assert_eq!(d.origin_class, origin);
        assert!(!d.is_internal_origin);
        assert!(d.needs_external_note);
    }

    // Partial / missing / drifted / stale lineage cannot claim current lineage and carries a note.
    let d = resolve_output_provenance_chip_group(Kind::ProducedByCell, State::ProvenancePartial);
    assert!(!d.may_claim_current_lineage);
    assert!(d.needs_partial_note);

    let d = resolve_output_provenance_chip_group(Kind::ProducedByCell, State::ProvenanceMissing);
    assert!(!d.may_claim_current_lineage);
    assert!(d.needs_missing_note);

    let d = resolve_output_provenance_chip_group(Kind::ProducedByRun, State::ExecutionCountDrifted);
    assert!(!d.may_claim_current_lineage);
    assert!(d.needs_drift_note);

    let d = resolve_output_provenance_chip_group(Kind::ProducedByRun, State::ProvenanceStale);
    assert!(!d.may_claim_current_lineage);
    assert!(d.needs_stale_note);
}

#[test]
fn banner_trust_freshness_and_presentation_coverage_is_complete() {
    let packet = packet();
    let trusts: std::collections::BTreeSet<_> =
        packet.trust_banners.iter().map(|b| b.trust_class).collect();
    for trust in M5OutputTrustClass::ALL {
        assert!(trusts.contains(&trust), "missing trust class {trust:?}");
    }
    let freshness: std::collections::BTreeSet<_> = packet
        .trust_banners
        .iter()
        .map(|b| b.freshness_state)
        .collect();
    for state in M5OutputFreshnessState::ALL {
        assert!(
            freshness.contains(&state),
            "missing freshness state {state:?}"
        );
    }
    let presentation: std::collections::BTreeSet<_> = packet
        .trust_banners
        .iter()
        .map(|b| b.trust_disclosure().presentation_class)
        .collect();
    for class in OutputTrustPresentationClass::ALL {
        assert!(
            presentation.contains(&class),
            "missing presentation class {class:?}"
        );
    }
}

#[test]
fn chip_kind_state_origin_and_resolution_coverage_is_complete() {
    let packet = packet();
    let kinds: std::collections::BTreeSet<_> = packet
        .provenance_chip_groups
        .iter()
        .map(|g| g.provenance_kind)
        .collect();
    for kind in M5OutputProvenanceKind::ALL {
        assert!(kinds.contains(&kind), "missing provenance kind {kind:?}");
    }
    let states: std::collections::BTreeSet<_> = packet
        .provenance_chip_groups
        .iter()
        .map(|g| g.provenance_state)
        .collect();
    for state in M5OutputProvenanceState::ALL {
        assert!(
            states.contains(&state),
            "missing provenance state {state:?}"
        );
    }
    let origins: std::collections::BTreeSet<_> = packet
        .provenance_chip_groups
        .iter()
        .map(|g| g.provenance_disclosure().origin_class)
        .collect();
    for origin in OutputOriginClass::ALL {
        assert!(origins.contains(&origin), "missing origin class {origin:?}");
    }
    let resolutions: std::collections::BTreeSet<_> = packet
        .provenance_chip_groups
        .iter()
        .map(|g| g.provenance_disclosure().resolution_class)
        .collect();
    for resolution in OutputProvenanceResolution::ALL {
        assert!(
            resolutions.contains(&resolution),
            "missing lineage resolution {resolution:?}"
        );
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::MissingSourceContracts));
}

#[test]
fn empty_trust_banners_fails() {
    let mut packet = packet();
    packet.trust_banners.clear();
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::TrustBannersMissing));
}

#[test]
fn empty_chip_groups_fails() {
    let mut packet = packet();
    packet.provenance_chip_groups.clear();
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::ChipGroupsMissing));
}

#[test]
fn banner_wrong_component_class_fails() {
    let mut packet = packet();
    packet.trust_banners[0].component =
        M5NotebookKernelOutputComponentFamily::OutputProvenanceChipGroup;
    assert!(packet.validate().contains(
        &OutputTrustBannerOutputProvenanceChipGroupViolation::TrustBannerWrongComponentClass
    ));
}

#[test]
fn chip_wrong_component_class_fails() {
    let mut packet = packet();
    packet.provenance_chip_groups[0].component =
        M5NotebookKernelOutputComponentFamily::OutputTrustBanner;
    assert!(packet.validate().contains(
        &OutputTrustBannerOutputProvenanceChipGroupViolation::ChipGroupWrongComponentClass
    ));
}

#[test]
fn stale_banner_claiming_live_fails() {
    let mut packet = packet();
    // A non-live banner cannot claim to be live.
    let banner = packet
        .trust_banners
        .iter_mut()
        .find(|b| !b.trust_disclosure().may_present_as_live)
        .expect("non-live banner present");
    banner.claims_live = true;
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::StaleOutputClaimedLive));
}

#[test]
fn banner_misrepresenting_presentation_fails() {
    let mut packet = packet();
    let banner = packet
        .trust_banners
        .iter_mut()
        .find(|b| !b.trust_disclosure().is_active_content)
        .expect("inactive banner present");
    banner.claims_active_content = true;
    assert!(packet.validate().contains(
        &OutputTrustBannerOutputProvenanceChipGroupViolation::PresentationClassMisrepresented
    ));
}

#[test]
fn chip_overclaiming_current_lineage_fails() {
    let mut packet = packet();
    let group = packet
        .provenance_chip_groups
        .iter_mut()
        .find(|g| !g.provenance_disclosure().may_claim_current_lineage)
        .expect("non-current chip group present");
    group.claims_current_lineage = true;
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::CurrentLineageOverclaimed));
}

#[test]
fn missing_stale_note_fails() {
    let mut packet = packet();
    let banner = packet
        .trust_banners
        .iter_mut()
        .find(|b| b.trust_disclosure().needs_stale_note)
        .expect("stale banner present");
    banner.stale_note.clear();
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::StaleNoteMissing));
}

#[test]
fn missing_isolation_note_fails() {
    let mut packet = packet();
    let banner = packet
        .trust_banners
        .iter_mut()
        .find(|b| b.trust_disclosure().needs_isolation_note)
        .expect("isolated banner present");
    banner.isolation_note.clear();
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::IsolationNoteMissing));
}

#[test]
fn missing_external_note_fails() {
    let mut packet = packet();
    let group = packet
        .provenance_chip_groups
        .iter_mut()
        .find(|g| g.provenance_disclosure().needs_external_note)
        .expect("external chip group present");
    group.external_note.clear();
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::ExternalNoteMissing));
}

#[test]
fn missing_drift_note_fails() {
    let mut packet = packet();
    let group = packet
        .provenance_chip_groups
        .iter_mut()
        .find(|g| g.provenance_disclosure().needs_drift_note)
        .expect("drifted chip group present");
    group.drift_note.clear();
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::DriftNoteMissing));
}

#[test]
fn missing_trust_class_label_fails() {
    let mut packet = packet();
    packet.trust_banners[0].trust_class_label.clear();
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::TrustClassLabelMissing));
}

#[test]
fn missing_copy_export_choice_note_fails() {
    let mut packet = packet();
    packet.trust_banners[0].copy_export_choice_note.clear();
    assert!(packet.validate().contains(
        &OutputTrustBannerOutputProvenanceChipGroupViolation::CopyExportChoiceNoteMissing
    ));
}

#[test]
fn missing_cell_run_identity_label_fails() {
    let mut packet = packet();
    packet.provenance_chip_groups[0]
        .cell_run_identity_label
        .clear();
    assert!(packet.validate().contains(
        &OutputTrustBannerOutputProvenanceChipGroupViolation::CellRunIdentityLabelMissing
    ));
}

#[test]
fn missing_persistence_retention_note_fails() {
    let mut packet = packet();
    packet.provenance_chip_groups[0]
        .persistence_retention_note
        .clear();
    assert!(packet.validate().contains(
        &OutputTrustBannerOutputProvenanceChipGroupViolation::PersistenceRetentionNoteMissing
    ));
}

#[test]
fn banner_missing_copy_action_fails() {
    let mut packet = packet();
    packet.trust_banners[0].banner_actions = vec![OutputBannerAction::OpenRaw];
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::BannerActionsIncomplete));
}

#[test]
fn chip_missing_copy_action_fails() {
    let mut packet = packet();
    packet.provenance_chip_groups[0].chip_actions = vec![OutputChipAction::InspectProvenance];
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::ChipActionsIncomplete));
}

#[test]
fn deep_link_action_without_target_fails() {
    let mut packet = packet();
    packet.trust_banners[0].deep_link_kind = DeepLinkKind::NoDeepLink;
    packet.trust_banners[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::DeepLinkUnresolved));
}

#[test]
fn resolvable_deep_link_without_ref_fails() {
    let mut packet = packet();
    packet.trust_banners[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::DeepLinkRefMissing));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = packet();
    packet.provenance_chip_groups[0].context_note.clear();
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::ContextNoteMissing));
}

#[test]
fn missing_dispositions_fails() {
    let mut packet = packet();
    packet.trust_banners[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::DispositionsMissing));
}

#[test]
fn banner_presenting_stale_as_live_fails() {
    let mut packet = packet();
    packet.trust_banners[0].presents_stale_output_as_live = true;
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::StaleShownAsLive));
}

#[test]
fn banner_hiding_trust_class_behind_hover_only_fails() {
    let mut packet = packet();
    packet.trust_banners[0].hides_trust_class_behind_hover_only = true;
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::TrustClassHoverOnly));
}

#[test]
fn banner_flattening_output_fails() {
    let mut packet = packet();
    packet.trust_banners[0].flattens_output_into_ambiguous_evidence = true;
    assert!(packet.validate().contains(
        &OutputTrustBannerOutputProvenanceChipGroupViolation::OutputFlattenedIntoAmbiguousEvidence
    ));
}

#[test]
fn chip_severing_provenance_fails() {
    let mut packet = packet();
    packet.provenance_chip_groups[0].severs_output_provenance = true;
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::ProvenanceSevered));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.trust_banners[0].required_labels = vec![M5NotebookKernelOutputRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.provenance_chip_groups[0].accessibility_routes =
        vec![M5NotebookKernelOutputAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::AccessibilityRouteMissing));
}

#[test]
fn output_review_incomplete_fails() {
    let mut packet = packet();
    packet.output_review.stale_output_never_presented_as_live = false;
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::OutputReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .support_export_shows_trust_and_provenance = false;
    assert!(packet.validate().contains(
        &OutputTrustBannerOutputProvenanceChipGroupViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.trust_banners[0].deep_link_ref = "see https://internal.example/output".to_owned();
    assert!(packet
        .validate()
        .contains(&OutputTrustBannerOutputProvenanceChipGroupViolation::RawMaterialInExport));
}

#[test]
fn markdown_summary_lists_components() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Output trust banners"));
    assert!(summary.contains("## Output provenance chip groups"));
    assert!(summary.contains("trusted_local_active"));
    assert!(summary.contains("lineage_drifted"));
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 trust banners + 6 provenance chip groups
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("output_trust_banner"));
    assert!(csv.contains("output_provenance_chip_group"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_output_trust_banner_output_provenance_chip_group_export()
        .expect("checked output trust banner output provenance chip group export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-output-trust-banner-output-provenance-chip-group-controls/output_trust_banner_stale.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-output-trust-banner-output-provenance-chip-group-controls/output_provenance_chip_group_drifted.json"
        )),
    ] {
        let packet: OutputTrustBannerOutputProvenanceChipGroupControlsPacket =
            serde_json::from_str(raw).expect(
                "fixture parses as output trust banner output provenance chip group packet",
            );
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
        seeded_output_trust_banner_output_provenance_chip_group_controls_output_trust_banner_stale(),
        seeded_output_trust_banner_output_provenance_chip_group_controls_output_provenance_chip_group_drifted(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
