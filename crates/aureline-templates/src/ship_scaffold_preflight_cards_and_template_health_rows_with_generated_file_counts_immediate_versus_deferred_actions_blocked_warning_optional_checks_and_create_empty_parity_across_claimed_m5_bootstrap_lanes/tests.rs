use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_scaffold_readiness_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, SCAFFOLD_READINESS_CONTROLS_PACKET_ID);
    assert_eq!(packet.record_kind, SCAFFOLD_READINESS_CONTROLS_RECORD_KIND);
}

#[test]
fn every_component_carries_its_frozen_family() {
    let packet = seeded_scaffold_readiness_controls();
    assert!(!packet.preflight_cards.is_empty());
    assert!(!packet.health_rows.is_empty());
    for card in &packet.preflight_cards {
        assert_eq!(
            card.component,
            M5ScaffoldComponentFamily::ScaffoldPreflightCard
        );
    }
    for row in &packet.health_rows {
        assert_eq!(row.component, M5ScaffoldComponentFamily::TemplateHealthRow);
    }
}

#[test]
fn ac_side_effect_vocabulary_is_frozen_exactly() {
    // The acceptance criteria pin the exact side-effect labels a generic Create must never hide;
    // assert the exact tokens.
    let tokens: Vec<&str> = PreflightSideEffectKind::REAL
        .iter()
        .map(|k| k.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "package_install",
            "dependency_restore",
            "remote_provisioning",
            "trust_prompt",
            "script_execution",
            "extension_install",
        ]
    );
}

#[test]
fn ac_health_severity_vocabulary_is_frozen_exactly() {
    let tokens: Vec<&str> = HealthSeverity::ALL.iter().map(|s| s.as_str()).collect();
    assert_eq!(tokens, vec!["blocker", "warning", "info"]);
}

#[test]
fn preflight_severity_is_derived_never_asserted() {
    let packet = seeded_scaffold_readiness_controls();
    for card in &packet.preflight_cards {
        let disclosure = resolve_preflight_disclosure(card.result_state);
        assert_eq!(card.derived_severity, disclosure.severity);
        assert_eq!(card.claims_blocking_prerequisite, disclosure.is_blocking);
        assert_eq!(
            card.claims_side_effecting,
            card.side_effect_kind.is_side_effecting()
        );
        assert_eq!(
            card.claims_immediate_action,
            card.action_timing.is_immediate()
        );
    }
}

#[test]
fn health_posture_is_derived_never_asserted() {
    let packet = seeded_scaffold_readiness_controls();
    for row in &packet.health_rows {
        let disclosure = resolve_health_disclosure(row.freshness_state);
        assert_eq!(row.derived_freshness_posture, disclosure.freshness_posture);
        assert_eq!(row.claims_current, disclosure.is_current);
        assert_eq!(
            row.claims_blocking_prerequisite,
            row.severity.is_blocking_prerequisite()
        );
    }
}

#[test]
fn only_blocked_result_reads_as_blocking_prerequisite() {
    for state in [
        M5PreflightResultState::Passed,
        M5PreflightResultState::Warning,
        M5PreflightResultState::SkippedOptional,
        M5PreflightResultState::NotRun,
        M5PreflightResultState::Unknown,
    ] {
        assert!(
            !resolve_preflight_disclosure(state).is_blocking,
            "{state:?}"
        );
    }
    assert!(resolve_preflight_disclosure(M5PreflightResultState::Blocked).is_blocking);
}

#[test]
fn only_fresh_reads_as_current() {
    for state in [
        M5HealthFreshnessState::Aging,
        M5HealthFreshnessState::Stale,
        M5HealthFreshnessState::Expired,
        M5HealthFreshnessState::NeverChecked,
        M5HealthFreshnessState::Unavailable,
    ] {
        assert!(!resolve_health_disclosure(state).is_current, "{state:?}");
    }
    assert!(resolve_health_disclosure(M5HealthFreshnessState::Fresh).is_current);
}

#[test]
fn stale_and_expired_both_require_a_stale_note() {
    for state in [
        M5HealthFreshnessState::Stale,
        M5HealthFreshnessState::Expired,
    ] {
        assert!(
            resolve_health_disclosure(state).needs_stale_note,
            "{state:?}"
        );
    }
}

#[test]
fn preflight_cards_cover_every_check_result_side_effect_and_severity() {
    let packet = seeded_scaffold_readiness_controls();
    for check in M5PreflightCheckClass::ALL {
        assert!(
            packet
                .preflight_cards
                .iter()
                .any(|c| c.check_class == check),
            "missing check class {}",
            check.as_str()
        );
    }
    for state in M5PreflightResultState::ALL {
        assert!(
            packet
                .preflight_cards
                .iter()
                .any(|c| c.result_state == state),
            "missing result state {}",
            state.as_str()
        );
    }
    for side in PreflightSideEffectKind::REAL {
        assert!(
            packet
                .preflight_cards
                .iter()
                .any(|c| c.side_effect_kind == side),
            "missing side-effect kind {}",
            side.as_str()
        );
    }
    for severity in PreflightSeverity::ALL {
        assert!(
            packet
                .preflight_cards
                .iter()
                .any(|c| c.derived_severity == severity),
            "missing severity {}",
            severity.as_str()
        );
    }
}

#[test]
fn health_rows_cover_every_signal_freshness_severity_and_posture() {
    let packet = seeded_scaffold_readiness_controls();
    for signal in M5HealthSignalClass::ALL {
        assert!(
            packet.health_rows.iter().any(|r| r.signal_class == signal),
            "missing signal class {}",
            signal.as_str()
        );
    }
    for state in M5HealthFreshnessState::ALL {
        assert!(
            packet
                .health_rows
                .iter()
                .any(|r| r.freshness_state == state),
            "missing freshness state {}",
            state.as_str()
        );
    }
    for severity in HealthSeverity::ALL {
        assert!(
            packet.health_rows.iter().any(|r| r.severity == severity),
            "missing severity {}",
            severity.as_str()
        );
    }
    for posture in HealthFreshnessPosture::ALL {
        assert!(
            packet
                .health_rows
                .iter()
                .any(|r| r.derived_freshness_posture == posture),
            "missing posture {}",
            posture.as_str()
        );
    }
}

#[test]
fn every_component_offers_mandatory_actions_labels_and_keyboard_route() {
    let packet = seeded_scaffold_readiness_controls();
    for card in &packet.preflight_cards {
        for action in PreflightCardAction::MANDATORY {
            assert!(card.card_actions.contains(&action));
        }
        assert!(card.offers_create_empty());
        assert!(card.declares_mandatory_labels());
        assert!(card
            .accessibility_routes
            .contains(&M5ScaffoldAccessibilityRoute::KeyboardFocusable));
    }
    for row in &packet.health_rows {
        for action in HealthRowAction::MANDATORY {
            assert!(row.row_actions.contains(&action));
        }
        assert!(row.offers_create_without_starter());
        assert!(row.declares_mandatory_labels());
        assert!(row
            .accessibility_routes
            .contains(&M5ScaffoldAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn misrepresented_preflight_severity_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    packet.preflight_cards[0].claims_blocking_prerequisite = true;
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::PreflightSeverityMisrepresented));
}

#[test]
fn misrepresented_health_posture_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    packet.health_rows[0].claims_current = false;
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::HealthPostureMisrepresented));
}

#[test]
fn missing_side_effect_note_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    let card = packet
        .preflight_cards
        .iter_mut()
        .find(|c| c.side_effect_kind.is_side_effecting())
        .expect("a side-effecting card present");
    card.side_effect_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::SideEffectNoteMissing));
}

#[test]
fn missing_recovery_path_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    packet.preflight_cards[0].recovery_path_label = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::RecoveryPathMissing));
}

#[test]
fn missing_generated_impact_note_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    packet.preflight_cards[0].generated_impact_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::GeneratedImpactNoteMissing));
}

#[test]
fn health_row_without_create_without_starter_path_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    packet.health_rows[0].row_actions.retain(|a| {
        *a != HealthRowAction::CreateEmpty && *a != HealthRowAction::ContinueWithoutStarter
    });
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::HealthRecoveryPathMissing));
}

#[test]
fn preflight_card_without_create_empty_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    packet.preflight_cards[0]
        .card_actions
        .retain(|a| *a != PreflightCardAction::CreateEmpty);
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::PlainCreateWithoutStarterMonopolized));
}

#[test]
fn missing_mandatory_preflight_action_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    packet.preflight_cards[0]
        .card_actions
        .retain(|a| *a != PreflightCardAction::ReviewSideEffects);
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::PreflightCardActionsIncomplete));
}

#[test]
fn missing_mandatory_health_action_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    packet.health_rows[0]
        .row_actions
        .retain(|a| *a != HealthRowAction::RerunCheck);
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::HealthRowActionsIncomplete));
}

#[test]
fn missing_fix_note_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    let row = packet
        .health_rows
        .iter_mut()
        .find(|r| r.fix_kind.needs_fix_note())
        .expect("a fixable row present");
    row.fix_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::HealthFixNoteMissing));
}

#[test]
fn each_hard_invariant_fails_when_set() {
    let mut packet = seeded_scaffold_readiness_controls();
    packet.preflight_cards[0].hides_side_effect_behind_generic_create = true;
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::SideEffectBehindGenericCreate));

    let mut packet = seeded_scaffold_readiness_controls();
    packet.preflight_cards[0].hides_generated_impact_or_recovery_path = true;
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::GeneratedImpactOrRecoveryHidden));

    let mut packet = seeded_scaffold_readiness_controls();
    packet.health_rows[0].monopolizes_plain_create_without_starter_path = true;
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::PlainCreateWithoutStarterMonopolized));

    let mut packet = seeded_scaffold_readiness_controls();
    packet.health_rows[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::AlternateStateLabelInvented));
}

#[test]
fn deep_link_action_without_resolvable_kind_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    let card = packet
        .preflight_cards
        .iter_mut()
        .find(|c| c.card_actions.contains(&PreflightCardAction::OpenDeepLink))
        .expect("a card offering a deep link");
    card.deep_link_kind = DeepLinkKind::NoDeepLink;
    card.deep_link_ref = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::DeepLinkUnresolved));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    packet.health_rows[0].context_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::ContextNoteMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::MissingSourceContracts));
}

#[test]
fn readiness_review_incomplete_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    packet
        .readiness_review
        .create_never_generic_hides_side_effects = false;
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::ReadinessReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    packet
        .consumer_projection
        .support_export_shows_component_truth = false;
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_scaffold_readiness_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ScaffoldReadinessControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_component() {
    let summary = seeded_scaffold_readiness_controls().render_markdown_summary();
    for card in seeded_scaffold_readiness_controls().preflight_cards {
        assert!(summary.contains(&card.card_name));
    }
    for row in seeded_scaffold_readiness_controls().health_rows {
        assert!(summary.contains(&row.check_name));
    }
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let packet = seeded_scaffold_readiness_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + packet.preflight_cards.len() + packet.health_rows.len()
    );
    assert!(lines[0].starts_with("component,id,frozen_state,"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_scaffold_readiness_controls_export()
        .expect("checked scaffold-readiness controls export validates");
    assert_eq!(
        from_disk,
        seeded_scaffold_readiness_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn scenario_fixtures_validate_and_keep_full_coverage() {
    for packet in [
        seeded_scaffold_readiness_controls_preflight_card_blocked(),
        seeded_scaffold_readiness_controls_health_row_stale(),
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
    let blocked: ScaffoldPreflightCardTemplateHealthRowControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-scaffold-preflight-card-template-health-row-controls/preflight_card_blocked.json"
        )))
        .expect("preflight-card-blocked fixture parses");
    assert!(blocked.validate().is_empty());
    assert_eq!(
        blocked,
        seeded_scaffold_readiness_controls_preflight_card_blocked()
    );

    let stale: ScaffoldPreflightCardTemplateHealthRowControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-scaffold-preflight-card-template-health-row-controls/health_row_stale.json"
        )))
        .expect("health-row-stale fixture parses");
    assert!(stale.validate().is_empty());
    assert_eq!(stale, seeded_scaffold_readiness_controls_health_row_stale());
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_scaffold_readiness_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}
