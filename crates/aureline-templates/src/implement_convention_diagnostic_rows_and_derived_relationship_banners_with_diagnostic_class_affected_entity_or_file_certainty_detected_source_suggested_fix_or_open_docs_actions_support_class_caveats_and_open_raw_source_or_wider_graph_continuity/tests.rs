use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_convention_relationship_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, CONVENTION_RELATIONSHIP_CONTROLS_PACKET_ID);
    assert_eq!(
        packet.record_kind,
        CONVENTION_RELATIONSHIP_CONTROLS_RECORD_KIND
    );
}

#[test]
fn every_component_carries_its_frozen_family() {
    let packet = seeded_convention_relationship_controls();
    assert!(!packet.diagnostic_rows.is_empty());
    assert!(!packet.relationship_banners.is_empty());
    for row in &packet.diagnostic_rows {
        assert_eq!(
            row.component,
            M5FrameworkComponentFamily::ConventionDiagnosticRow
        );
    }
    for banner in &packet.relationship_banners {
        assert_eq!(
            banner.component,
            M5FrameworkComponentFamily::DerivedRelationshipBanner
        );
    }
}

#[test]
fn ac_certainty_posture_vocabulary_is_frozen_exactly() {
    // The acceptance criteria pin the exact certainty labels: exact from source, runtime confirmed,
    // heuristic, or partial / unresolved. Assert the exact tokens.
    let tokens: Vec<&str> = DerivedCertaintyPosture::ALL
        .iter()
        .map(|p| p.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "exact_from_source",
            "runtime_confirmed",
            "heuristic",
            "partial_or_unresolved"
        ]
    );
}

#[test]
fn ac_diagnostic_classes_never_collapse_into_one_state() {
    // AC #1: hard contract violations, pack limitations, version mismatches, and heuristic
    // suspicions must be distinct diagnostic classes, not one generic warning.
    let tokens: Vec<&str> = DiagnosticClass::ALL.iter().map(|c| c.as_str()).collect();
    assert!(tokens.contains(&"hard_contract_violation"));
    assert!(tokens.contains(&"pack_limitation"));
    assert!(tokens.contains(&"version_mismatch"));
    assert!(tokens.contains(&"heuristic_suspicion"));
    // Every distinct class is present in the seed so warnings never collapse.
    let packet = seeded_convention_relationship_controls();
    for class in DiagnosticClass::ALL {
        assert!(
            packet
                .diagnostic_rows
                .iter()
                .any(|r| r.diagnostic_class == class),
            "missing diagnostic class {}",
            class.as_str()
        );
    }
}

#[test]
fn posture_is_derived_never_asserted() {
    let packet = seeded_convention_relationship_controls();
    for row in &packet.diagnostic_rows {
        let disclosure = row.posture_disclosure();
        assert_eq!(row.derived_certainty_posture, disclosure.certainty_posture);
        assert_eq!(
            row.claims_exact_from_source,
            disclosure.is_exact_from_source
        );
        assert_eq!(row.has_proving_source_form, disclosure.has_source_form);
    }
    for banner in &packet.relationship_banners {
        let disclosure = banner.posture_disclosure();
        assert_eq!(
            banner.derived_certainty_posture,
            disclosure.certainty_posture
        );
        assert_eq!(
            banner.claims_exact_from_source,
            disclosure.is_exact_from_source
        );
        assert_eq!(banner.has_proving_source_form, disclosure.has_source_form);
    }
}

#[test]
fn only_verified_diagnostic_reads_as_exact() {
    for confidence in [
        M5ConventionConfidenceClass::HighConfidence,
        M5ConventionConfidenceClass::HeuristicConvention,
        M5ConventionConfidenceClass::DerivedByConvention,
        M5ConventionConfidenceClass::LowConfidence,
        M5ConventionConfidenceClass::Unknown,
    ] {
        let disclosure = resolve_convention_diagnostic_posture(confidence);
        assert!(!disclosure.is_exact_from_source, "{confidence:?}");
        assert!(disclosure.must_not_read_as_exact, "{confidence:?}");
    }
    let verified = resolve_convention_diagnostic_posture(M5ConventionConfidenceClass::Verified);
    assert!(verified.is_exact_from_source);
    assert!(!verified.must_not_read_as_exact);
}

#[test]
fn unknown_confidence_diagnostic_has_no_source_form() {
    let unknown = resolve_convention_diagnostic_posture(M5ConventionConfidenceClass::Unknown);
    assert!(!unknown.has_source_form);
    assert!(unknown.needs_no_source_form_note);
    let verified = resolve_convention_diagnostic_posture(M5ConventionConfidenceClass::Verified);
    assert!(verified.has_source_form);
}

#[test]
fn only_exact_relationship_reads_as_exact() {
    for class in [
        M5DerivedRelationshipClass::HeuristicLink,
        M5DerivedRelationshipClass::DerivedByConvention,
        M5DerivedRelationshipClass::PartialLink,
        M5DerivedRelationshipClass::UnresolvedLink,
    ] {
        let disclosure = resolve_derived_relationship_posture(
            class,
            M5RelationshipProvingState::SourceLinkedPartial,
        );
        assert!(!disclosure.is_exact_from_source, "{class:?}");
        assert!(disclosure.must_not_read_as_exact, "{class:?}");
    }
    let exact = resolve_derived_relationship_posture(
        M5DerivedRelationshipClass::ExactFromSource,
        M5RelationshipProvingState::ProvingSourceLinked,
    );
    assert!(exact.is_exact_from_source);
    // Runtime is a distinct strong state, not exact-from-source.
    let runtime = resolve_derived_relationship_posture(
        M5DerivedRelationshipClass::InferredFromRuntime,
        M5RelationshipProvingState::RuntimeEvidenceOnly,
    );
    assert!(!runtime.is_exact_from_source);
    assert!(runtime.is_runtime_confirmed);
    assert!(!runtime.must_not_read_as_exact);
}

#[test]
fn no_proving_or_unknown_proving_relationship_has_no_source_form() {
    for proving in [
        M5RelationshipProvingState::NoProvingSource,
        M5RelationshipProvingState::UnknownProving,
    ] {
        let disclosure =
            resolve_derived_relationship_posture(M5DerivedRelationshipClass::PartialLink, proving);
        assert!(!disclosure.has_source_form, "{proving:?}");
        assert!(disclosure.needs_no_source_form_note, "{proving:?}");
    }
    let linked = resolve_derived_relationship_posture(
        M5DerivedRelationshipClass::ExactFromSource,
        M5RelationshipProvingState::ProvingSourceLinked,
    );
    assert!(linked.has_source_form);
}

#[test]
fn components_cover_every_frozen_and_derived_vocabulary() {
    let packet = seeded_convention_relationship_controls();
    for confidence in M5ConventionConfidenceClass::ALL {
        assert!(
            packet
                .diagnostic_rows
                .iter()
                .any(|r| r.convention_confidence_class == confidence),
            "missing confidence {}",
            confidence.as_str()
        );
    }
    for severity in M5ConventionDiagnosticSeverity::ALL {
        assert!(
            packet
                .diagnostic_rows
                .iter()
                .any(|r| r.diagnostic_severity == severity),
            "missing severity {}",
            severity.as_str()
        );
    }
    for detection in DetectionSource::ALL {
        assert!(
            packet
                .diagnostic_rows
                .iter()
                .any(|r| r.detection_source == detection),
            "missing detection {}",
            detection.as_str()
        );
    }
    for caveat in SupportCaveatClass::ALL {
        assert!(
            packet
                .diagnostic_rows
                .iter()
                .any(|r| r.support_caveat == caveat),
            "missing caveat {}",
            caveat.as_str()
        );
    }
    for class in M5DerivedRelationshipClass::ALL {
        assert!(
            packet
                .relationship_banners
                .iter()
                .any(|b| b.derived_relationship_class == class),
            "missing relationship class {}",
            class.as_str()
        );
    }
    for proving in M5RelationshipProvingState::ALL {
        assert!(
            packet
                .relationship_banners
                .iter()
                .any(|b| b.relationship_proving_state == proving),
            "missing proving state {}",
            proving.as_str()
        );
    }
    for inference in InferenceSource::ALL {
        assert!(
            packet
                .relationship_banners
                .iter()
                .any(|b| b.inference_source == inference),
            "missing inference {}",
            inference.as_str()
        );
    }
    for refresh in RefreshState::ALL {
        assert!(
            packet
                .relationship_banners
                .iter()
                .any(|b| b.refresh_state == refresh),
            "missing refresh {}",
            refresh.as_str()
        );
    }
    for posture in DerivedCertaintyPosture::ALL {
        assert!(
            packet
                .diagnostic_rows
                .iter()
                .any(|r| r.derived_certainty_posture == posture)
                || packet
                    .relationship_banners
                    .iter()
                    .any(|b| b.derived_certainty_posture == posture),
            "missing certainty posture {}",
            posture.as_str()
        );
    }
    for link in ProvingSourceLink::ALL {
        assert!(
            packet
                .diagnostic_rows
                .iter()
                .any(|r| r.proving_source_kind == link)
                || packet
                    .relationship_banners
                    .iter()
                    .any(|b| b.proving_source_kind == link),
            "missing proving source link {}",
            link.as_str()
        );
    }
}

#[test]
fn every_component_offers_mandatory_actions_labels_and_keyboard_route() {
    let packet = seeded_convention_relationship_controls();
    for row in &packet.diagnostic_rows {
        for action in DiagnosticRowAction::MANDATORY {
            assert!(row.row_actions.contains(&action));
        }
        assert!(row.declares_mandatory_labels());
        assert!(row
            .accessibility_routes
            .contains(&M5FrameworkAccessibilityRoute::KeyboardFocusable));
    }
    for banner in &packet.relationship_banners {
        for action in BannerAction::MANDATORY {
            assert!(banner.banner_actions.contains(&action));
        }
        assert!(banner.declares_mandatory_labels());
        assert!(banner
            .accessibility_routes
            .contains(&M5FrameworkAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn misrepresented_diagnostic_posture_fails() {
    let mut packet = seeded_convention_relationship_controls();
    packet.diagnostic_rows[0].claims_exact_from_source = false;
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::DiagnosticPostureMisrepresented));
}

#[test]
fn heuristic_diagnostic_claiming_exact_fails() {
    let mut packet = seeded_convention_relationship_controls();
    let row = packet
        .diagnostic_rows
        .iter_mut()
        .find(|r| r.posture_disclosure().must_not_read_as_exact)
        .expect("a heuristic or partial diagnostic");
    row.claims_exact_from_source = true;
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::HeuristicClaimsExact));
}

#[test]
fn ungrounded_diagnostic_claiming_a_proving_source_fails() {
    let mut packet = seeded_convention_relationship_controls();
    let row = packet
        .diagnostic_rows
        .iter_mut()
        .find(|r| !r.has_proving_source_form)
        .expect("an ungrounded diagnostic");
    row.proving_source_kind = ProvingSourceLink::SourceFile;
    row.proving_source_ref = "src:fake/path.rs".to_owned();
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::ProvingSourceClaimedWithoutForm));
}

#[test]
fn source_form_banner_without_proving_source_fails() {
    let mut packet = seeded_convention_relationship_controls();
    let banner = packet
        .relationship_banners
        .iter_mut()
        .find(|b| b.has_proving_source_form)
        .expect("a banner with a source form");
    banner.proving_source_kind = ProvingSourceLink::NoProvingSource;
    banner.proving_source_ref = String::new();
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::ProvingSourceUnresolved));
}

#[test]
fn missing_no_source_form_note_fails() {
    let mut packet = seeded_convention_relationship_controls();
    let row = packet
        .diagnostic_rows
        .iter_mut()
        .find(|r| !r.has_proving_source_form)
        .expect("an ungrounded diagnostic");
    row.no_source_form_note = String::new();
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::NoSourceFormNoteMissing));
}

#[test]
fn missing_support_caveat_fails() {
    let mut packet = seeded_convention_relationship_controls();
    let row = packet
        .diagnostic_rows
        .iter_mut()
        .find(|r| r.support_caveat.needs_caveat_label())
        .expect("a caveated diagnostic");
    row.support_caveat_label = String::new();
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::SupportCaveatMissing));
}

#[test]
fn missing_suggested_fix_fails() {
    let mut packet = seeded_convention_relationship_controls();
    packet.diagnostic_rows[0].suggested_fix_label = String::new();
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::SuggestedFixMissing));
}

#[test]
fn missing_affected_entity_or_file_fails() {
    let mut packet = seeded_convention_relationship_controls();
    packet.diagnostic_rows[0].affected_file_label = String::new();
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::AffectedEntityOrFileMissing));
}

#[test]
fn missing_inference_source_fails() {
    let mut packet = seeded_convention_relationship_controls();
    packet.relationship_banners[0].inference_source_label = String::new();
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::InferenceSourceMissing));
}

#[test]
fn missing_consumed_context_fails() {
    let mut packet = seeded_convention_relationship_controls();
    packet.relationship_banners[0].consumed_context_label = String::new();
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::ConsumedContextMissing));
}

#[test]
fn missing_refresh_label_fails() {
    let mut packet = seeded_convention_relationship_controls();
    packet.relationship_banners[0].last_refresh_label = String::new();
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::RefreshLabelMissing));
}

#[test]
fn misrepresented_relationship_posture_fails() {
    let mut packet = seeded_convention_relationship_controls();
    packet.relationship_banners[0].claims_exact_from_source = false;
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::RelationshipPostureMisrepresented));
}

#[test]
fn missing_mandatory_diagnostic_action_fails() {
    let mut packet = seeded_convention_relationship_controls();
    packet.diagnostic_rows[0]
        .row_actions
        .retain(|a| *a != DiagnosticRowAction::OpenProvingFile);
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::DiagnosticRowActionsIncomplete));
}

#[test]
fn missing_mandatory_banner_action_fails() {
    let mut packet = seeded_convention_relationship_controls();
    packet.relationship_banners[0]
        .banner_actions
        .retain(|a| *a != BannerAction::OpenRawSource);
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::BannerActionsIncomplete));
}

#[test]
fn each_hard_invariant_fails_when_set() {
    let mut packet = seeded_convention_relationship_controls();
    packet.diagnostic_rows[0].lets_heuristic_masquerade_as_exact = true;
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::HeuristicMasqueradesAsExact));

    let mut packet = seeded_convention_relationship_controls();
    packet.diagnostic_rows[0].collapses_distinct_diagnostics_into_generic_warning = true;
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::DistinctDiagnosticsCollapsed));

    let mut packet = seeded_convention_relationship_controls();
    packet.diagnostic_rows[0].acts_as_hidden_parallel_model = true;
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::HiddenParallelModel));

    let mut packet = seeded_convention_relationship_controls();
    packet.relationship_banners[0].hides_approximation_in_background = true;
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::ApproximationHiddenInBackground));

    let mut packet = seeded_convention_relationship_controls();
    packet.relationship_banners[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::AlternateStateLabelInvented));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = seeded_convention_relationship_controls();
    packet.relationship_banners[0].context_note = String::new();
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::ContextNoteMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_convention_relationship_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::MissingSourceContracts));
}

#[test]
fn review_incomplete_fails() {
    let mut packet = seeded_convention_relationship_controls();
    packet
        .convention_relationship_review
        .distinct_diagnostics_never_collapsed = false;
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::ReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_convention_relationship_controls();
    packet
        .consumer_projection
        .banner_appears_where_inferred_truth_consumed = false;
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_convention_relationship_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ConventionRelationshipControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_component() {
    let packet = seeded_convention_relationship_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.diagnostic_rows {
        assert!(summary.contains(&row.diagnostic_message_label));
    }
    for banner in &packet.relationship_banners {
        assert!(summary.contains(&banner.relationship_label));
    }
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let packet = seeded_convention_relationship_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + packet.diagnostic_rows.len() + packet.relationship_banners.len()
    );
    assert!(lines[0].starts_with("component,id,primary_class,"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_convention_relationship_controls_export()
        .expect("checked convention relationship controls export validates");
    assert_eq!(
        from_disk,
        seeded_convention_relationship_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn scenario_fixtures_validate_and_keep_full_coverage() {
    for packet in [
        seeded_convention_relationship_controls_heuristic_diagnostic(),
        seeded_convention_relationship_controls_inferred_relationship(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn checked_scenario_fixtures_validate_and_match_seed_builders() {
    let diagnostic: ConventionDiagnosticDerivedRelationshipControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-convention-diagnostic-derived-relationship-controls/heuristic_diagnostic.json"
        )))
        .expect("heuristic-diagnostic fixture parses");
    assert!(diagnostic.validate().is_empty());
    assert_eq!(
        diagnostic,
        seeded_convention_relationship_controls_heuristic_diagnostic()
    );

    let relationship: ConventionDiagnosticDerivedRelationshipControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-convention-diagnostic-derived-relationship-controls/inferred_relationship.json"
        )))
        .expect("inferred-relationship fixture parses");
    assert!(relationship.validate().is_empty());
    assert_eq!(
        relationship,
        seeded_convention_relationship_controls_inferred_relationship()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_convention_relationship_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("secret"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}
