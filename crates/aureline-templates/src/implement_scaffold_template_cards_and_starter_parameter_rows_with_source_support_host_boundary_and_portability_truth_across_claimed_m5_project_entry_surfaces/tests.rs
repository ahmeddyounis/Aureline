use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_scaffold_entry_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, SCAFFOLD_ENTRY_CONTROLS_PACKET_ID);
    assert_eq!(packet.record_kind, SCAFFOLD_ENTRY_CONTROLS_RECORD_KIND);
}

#[test]
fn every_component_carries_its_frozen_family() {
    let packet = seeded_scaffold_entry_controls();
    assert!(!packet.template_cards.is_empty());
    assert!(!packet.parameter_rows.is_empty());
    for card in &packet.template_cards {
        assert_eq!(
            card.component,
            M5ScaffoldComponentFamily::ScaffoldTemplateCard
        );
    }
    for row in &packet.parameter_rows {
        assert_eq!(
            row.component,
            M5ScaffoldComponentFamily::StarterParameterRow
        );
    }
}

#[test]
fn ac_parameter_origin_vocabulary_is_frozen_exactly() {
    // The acceptance criteria pin the exact precedence labels; assert the exact tokens.
    let tokens: Vec<&str> = ParameterOriginClass::ALL
        .iter()
        .map(|o| o.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "template_default",
            "user_input",
            "workspace_value",
            "policy_value",
            "secret_reference",
        ]
    );
}

#[test]
fn template_posture_is_derived_never_asserted() {
    let packet = seeded_scaffold_entry_controls();
    for card in &packet.template_cards {
        let disclosure = resolve_template_posture(card.source_class, card.support_class);
        assert_eq!(card.derived_source_class, disclosure.source_class);
        assert_eq!(card.derived_support_posture, disclosure.support_posture);
        assert_eq!(
            card.claims_governed_first_party,
            disclosure.is_governed_first_party
        );
        assert_eq!(
            card.claims_exact_first_party_support,
            disclosure.is_exact_first_party_support
        );
    }
}

#[test]
fn parameter_portability_is_derived_never_asserted() {
    let packet = seeded_scaffold_entry_controls();
    for row in &packet.parameter_rows {
        let disclosure = resolve_parameter_disclosure(row.origin_class, row.action_timing);
        assert_eq!(row.derived_portability_class, disclosure.portability_class);
        assert_eq!(row.claims_portable, disclosure.is_portable);
    }
}

#[test]
fn non_first_party_sources_never_claim_governed_first_party() {
    for source in [
        M5StarterSourceClass::CommunityStarter,
        M5StarterSourceClass::LocalOnlyStarter,
        M5StarterSourceClass::MirroredStarter,
        M5StarterSourceClass::UnknownSourceStarter,
    ] {
        let disclosure =
            resolve_template_posture(source, M5TemplateSupportClass::OfficiallySupported);
        assert!(!disclosure.is_governed_first_party, "{source:?}");
    }
    for source in [
        M5StarterSourceClass::FirstPartyStarter,
        M5StarterSourceClass::TeamManagedStarter,
    ] {
        let disclosure =
            resolve_template_posture(source, M5TemplateSupportClass::OfficiallySupported);
        assert!(disclosure.is_governed_first_party, "{source:?}");
    }
}

#[test]
fn only_officially_supported_reads_as_exact_first_party() {
    for support in [
        M5TemplateSupportClass::CommunitySupported,
        M5TemplateSupportClass::Experimental,
        M5TemplateSupportClass::BridgeBehavior,
        M5TemplateSupportClass::Deprecated,
        M5TemplateSupportClass::Unsupported,
    ] {
        let disclosure = resolve_template_posture(M5StarterSourceClass::FirstPartyStarter, support);
        assert!(!disclosure.is_exact_first_party_support, "{support:?}");
        assert!(disclosure.needs_nonexact_support_note, "{support:?}");
    }
    let disclosure = resolve_template_posture(
        M5StarterSourceClass::FirstPartyStarter,
        M5TemplateSupportClass::OfficiallySupported,
    );
    assert!(disclosure.is_exact_first_party_support);
}

#[test]
fn only_template_and_user_values_are_portable() {
    for origin in [
        ParameterOriginClass::WorkspaceValue,
        ParameterOriginClass::PolicyValue,
        ParameterOriginClass::SecretReference,
    ] {
        let disclosure =
            resolve_parameter_disclosure(origin, M5ParameterActionTiming::DeferredAfterCreate);
        assert!(!disclosure.is_portable, "{origin:?}");
    }
    for origin in [
        ParameterOriginClass::TemplateDefault,
        ParameterOriginClass::UserInput,
    ] {
        let disclosure =
            resolve_parameter_disclosure(origin, M5ParameterActionTiming::DeferredAfterCreate);
        assert!(disclosure.is_portable, "{origin:?}");
    }
}

#[test]
fn secret_reference_requires_a_note() {
    let disclosure = resolve_parameter_disclosure(
        ParameterOriginClass::SecretReference,
        M5ParameterActionTiming::NotApplicable,
    );
    assert!(disclosure.is_secret_reference);
    assert!(disclosure.needs_secret_note);
}

#[test]
fn template_cards_cover_every_source_and_support_vocabulary() {
    let packet = seeded_scaffold_entry_controls();
    for source in M5StarterSourceClass::ALL {
        assert!(
            packet
                .template_cards
                .iter()
                .any(|c| c.source_class == source),
            "missing starter source class {}",
            source.as_str()
        );
    }
    for support in M5TemplateSupportClass::ALL {
        assert!(
            packet
                .template_cards
                .iter()
                .any(|c| c.support_class == support),
            "missing template support class {}",
            support.as_str()
        );
    }
    for class in TemplateSourceClass::ALL {
        assert!(
            packet
                .template_cards
                .iter()
                .any(|c| c.derived_source_class == class),
            "missing derived source class {}",
            class.as_str()
        );
    }
    for posture in TemplateSupportPosture::ALL {
        assert!(
            packet
                .template_cards
                .iter()
                .any(|c| c.derived_support_posture == posture),
            "missing support posture {}",
            posture.as_str()
        );
    }
}

#[test]
fn parameter_rows_cover_every_origin_layer_timing_and_portability() {
    let packet = seeded_scaffold_entry_controls();
    for origin in ParameterOriginClass::ALL {
        assert!(
            packet
                .parameter_rows
                .iter()
                .any(|r| r.origin_class == origin),
            "missing origin class {}",
            origin.as_str()
        );
    }
    for layer in M5ParameterSourceLayer::ALL {
        assert!(
            packet
                .parameter_rows
                .iter()
                .any(|r| r.source_layer == layer),
            "missing source layer {}",
            layer.as_str()
        );
    }
    for timing in M5ParameterActionTiming::ALL {
        assert!(
            packet
                .parameter_rows
                .iter()
                .any(|r| r.action_timing == timing),
            "missing action timing {}",
            timing.as_str()
        );
    }
    for portability in ParameterPortabilityClass::ALL {
        assert!(
            packet
                .parameter_rows
                .iter()
                .any(|r| r.derived_portability_class == portability),
            "missing portability class {}",
            portability.as_str()
        );
    }
}

#[test]
fn every_component_offers_mandatory_actions_labels_and_keyboard_route() {
    let packet = seeded_scaffold_entry_controls();
    for card in &packet.template_cards {
        for action in TemplateCardAction::MANDATORY {
            assert!(card.card_actions.contains(&action));
        }
        assert!(card.declares_mandatory_labels());
        assert!(card
            .accessibility_routes
            .contains(&M5ScaffoldAccessibilityRoute::KeyboardFocusable));
    }
    for row in &packet.parameter_rows {
        for action in ParameterRowAction::MANDATORY {
            assert!(row.row_actions.contains(&action));
        }
        assert!(row.declares_mandatory_labels());
        assert!(row
            .accessibility_routes
            .contains(&M5ScaffoldAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn misrepresented_template_posture_fails() {
    let mut packet = seeded_scaffold_entry_controls();
    packet.template_cards[0].claims_governed_first_party = false;
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::TemplatePostureMisrepresented));
}

#[test]
fn misrepresented_portability_fails() {
    let mut packet = seeded_scaffold_entry_controls();
    packet.parameter_rows[2].claims_portable = true;
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::PortabilityMisrepresented));
}

#[test]
fn missing_host_boundary_fails() {
    let mut packet = seeded_scaffold_entry_controls();
    packet.template_cards[0].host_boundary_label = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::HostBoundaryMissing));
}

#[test]
fn missing_secret_note_fails() {
    let mut packet = seeded_scaffold_entry_controls();
    let row = packet
        .parameter_rows
        .iter_mut()
        .find(|r| r.origin_class == ParameterOriginClass::SecretReference)
        .expect("secret-reference row present");
    row.secret_reference_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::SecretReferenceNoteMissing));
}

#[test]
fn missing_mandatory_template_action_fails() {
    let mut packet = seeded_scaffold_entry_controls();
    packet.template_cards[0]
        .card_actions
        .retain(|a| *a != TemplateCardAction::OpenManifest);
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::TemplateCardActionsIncomplete));
}

#[test]
fn missing_mandatory_parameter_action_fails() {
    let mut packet = seeded_scaffold_entry_controls();
    packet.parameter_rows[0]
        .row_actions
        .retain(|a| *a != ParameterRowAction::InspectSource);
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::ParameterRowActionsIncomplete));
}

#[test]
fn each_hard_invariant_fails_when_set() {
    let mut packet = seeded_scaffold_entry_controls();
    packet.template_cards[0].hides_starter_source_or_support_class = true;
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::StarterSourceOrSupportHidden));

    let mut packet = seeded_scaffold_entry_controls();
    packet.template_cards[0].hides_side_effect_or_host_boundary = true;
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::SideEffectOrHostBoundaryHidden));

    let mut packet = seeded_scaffold_entry_controls();
    packet.parameter_rows[0].exposes_secret_or_raw_value_by_default = true;
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::SecretOrRawValueExposed));

    let mut packet = seeded_scaffold_entry_controls();
    packet.parameter_rows[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::AlternateStateLabelInvented));
}

#[test]
fn deep_link_action_without_resolvable_kind_fails() {
    let mut packet = seeded_scaffold_entry_controls();
    let card = packet
        .template_cards
        .iter_mut()
        .find(|c| c.card_actions.contains(&TemplateCardAction::OpenDeepLink))
        .expect("a card offering a deep link");
    card.deep_link_kind = DeepLinkKind::NoDeepLink;
    card.deep_link_ref = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::DeepLinkUnresolved));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = seeded_scaffold_entry_controls();
    packet.parameter_rows[0].context_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::ContextNoteMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_scaffold_entry_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::MissingSourceContracts));
}

#[test]
fn scaffold_review_incomplete_fails() {
    let mut packet = seeded_scaffold_entry_controls();
    packet
        .scaffold_review
        .create_never_generic_hides_side_effects = false;
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::ScaffoldReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_scaffold_entry_controls();
    packet
        .consumer_projection
        .support_export_shows_component_truth = false;
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_scaffold_entry_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ScaffoldEntryControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_component() {
    let summary = seeded_scaffold_entry_controls().render_markdown_summary();
    for card in seeded_scaffold_entry_controls().template_cards {
        assert!(summary.contains(&card.template_name));
    }
    for row in seeded_scaffold_entry_controls().parameter_rows {
        assert!(summary.contains(&row.parameter_name));
    }
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let packet = seeded_scaffold_entry_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + packet.template_cards.len() + packet.parameter_rows.len()
    );
    assert!(lines[0].starts_with("component,id,frozen_state,"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_scaffold_entry_controls_export()
        .expect("checked scaffold-entry controls export validates");
    assert_eq!(
        from_disk,
        seeded_scaffold_entry_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn scenario_fixtures_validate_and_keep_full_coverage() {
    for packet in [
        seeded_scaffold_entry_controls_template_card_community(),
        seeded_scaffold_entry_controls_parameter_row_secret_reference(),
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
    let community: ScaffoldTemplateCardStarterParameterRowControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-scaffold-template-card-starter-parameter-row-controls/template_card_community.json"
        )))
        .expect("template-card-community fixture parses");
    assert!(community.validate().is_empty());
    assert_eq!(
        community,
        seeded_scaffold_entry_controls_template_card_community()
    );

    let secret: ScaffoldTemplateCardStarterParameterRowControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-scaffold-template-card-starter-parameter-row-controls/parameter_row_secret_reference.json"
        )))
        .expect("parameter-row-secret-reference fixture parses");
    assert!(secret.validate().is_empty());
    assert_eq!(
        secret,
        seeded_scaffold_entry_controls_parameter_row_secret_reference()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_scaffold_entry_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}
