//! Inline tests for the change-impact card-set lane.

use super::*;

fn packet() -> ChangeImpactCardSet {
    seeded_m5_change_impact_card_set()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_CHANGE_IMPACT_CARD_SET_PACKET_ID);
    assert_eq!(packet.record_kind, M5_CHANGE_IMPACT_CARD_SET_RECORD_KIND);
    assert_eq!(packet.cards.len(), ImpactDimension::ALL.len());
    assert_eq!(packet.consumers.len(), ImpactConsumer::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn every_dimension_is_carded_exactly_once() {
    let packet = packet();
    for dimension in ImpactDimension::ALL {
        let matches: Vec<&ChangeImpactCard> = packet
            .cards
            .iter()
            .filter(|c| c.dimension == dimension)
            .collect();
        assert_eq!(
            matches.len(),
            1,
            "dimension `{}` not carded once",
            dimension.as_str()
        );
        let card = matches[0];
        assert_eq!(
            card.primary_artifact_class,
            dimension.primary_artifact_class()
        );
        assert_eq!(card.owner_role, dimension.owner_role());
    }
}

#[test]
fn canonical_keeps_every_consumer_clear() {
    // Acceptance criterion: a routine update leaves every consumer clear to apply before restart.
    let packet = packet();
    for c in &packet.consumers {
        assert!(c.is_clear(), "consumer `{}` not clear", c.consumer.as_str());
        assert_eq!(c.review_readiness, ReviewReadiness::ClearToApply);
        assert!(c.gaps.is_empty());
        assert!(!c.requires_pre_restart_acknowledgement);
    }
    assert_eq!(
        packet.summary.clear_consumers,
        ImpactConsumer::ALL.len() as u32
    );
    assert!(!packet.requires_pre_restart_acknowledgement());
}

#[test]
fn card_discloses_affected_scope_risk_followup_and_rollback() {
    // Acceptance criterion: pre-restart surfaces show affected scope, risk class, follow-up tasks, and
    // rollback / pin choices.
    let packet = seeded_m5_change_impact_card_set_review();
    let card = packet.card(ImpactDimension::SchemaMigration).unwrap();
    assert_eq!(card.risk_class, RiskClass::MigrationRequired);
    assert!(card
        .affected_artifact_classes
        .contains(&ArtifactClass::SchemaContracts));
    // The primary class is always disclosed even when extra classes are added.
    assert!(card
        .affected_artifact_classes
        .contains(&card.primary_artifact_class));
    assert_eq!(
        card.follow_up.task_class,
        FollowUpTaskClass::MigrationScanRequired
    );
    assert_eq!(card.follow_up.timing, TaskTiming::BeforeRestart);
    assert!(card.rollback_choice.offers_recovery());
    assert_eq!(card.review_readiness, ReviewReadiness::ReviewRecommended);
}

#[test]
fn low_risk_cache_churn_is_distinguished_from_destructive() {
    // Acceptance criterion / guardrail: low-risk cache churn must not read like a destructive change.
    let packet = packet();
    let cache = packet.card(ImpactDimension::CacheMigration).unwrap();
    assert!(cache.risk_class.is_low_risk_cache_churn());
    assert!(!cache.risk_class.is_destructive_or_habit_breaking());
    assert_eq!(cache.gate, DescriptorGate::Governed); // low-risk churn stays clear
    assert!(!cache.requires_pre_restart_acknowledgement);

    let hold = seeded_m5_change_impact_card_set_hold();
    let ext = hold.card(ImpactDimension::ExtensionCompatibility).unwrap();
    assert!(ext.risk_class.is_destructive_or_habit_breaking());
    assert!(!ext.risk_class.is_low_risk_cache_churn());
    assert_eq!(ext.gate, DescriptorGate::Blocked); // destructive holds
    assert!(ext.requires_pre_restart_acknowledgement);
}

#[test]
fn confirmed_migration_narrows_exactly_the_consumers_that_read_it() {
    // Acceptance criterion: a real migration forecast surfaces before restart and narrows the right
    // consumers, without forcing an acknowledgement.
    let packet = seeded_m5_change_impact_card_set_review();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let dim = ImpactDimension::SchemaMigration;
    assert_eq!(packet.card(dim).unwrap().gate, DescriptorGate::Narrowed);
    for c in &packet.consumers {
        if c.read_dimensions.contains(&dim) {
            assert!(
                c.is_review(),
                "consumer `{}` reads migration but did not review",
                c.consumer.as_str()
            );
            assert!(c
                .gaps
                .iter()
                .any(|g| g.dimension == dim && g.gap_kind == ImpactGapKind::ReviewRecommended));
        } else {
            assert!(
                c.is_clear(),
                "consumer `{}` should stay clear",
                c.consumer.as_str()
            );
        }
    }
    assert!(!packet.requires_pre_restart_acknowledgement());
    assert!(packet
        .release_gate
        .affected_dimensions
        .contains(&dim.as_str().to_owned()));
}

#[test]
fn confirmed_destructive_holds_exactly_the_consumers_that_read_it() {
    // Acceptance criterion: a destructive change must surface before restart and require an explicit
    // acknowledgement from the consumers that read it.
    let packet = seeded_m5_change_impact_card_set_hold();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let dim = ImpactDimension::ExtensionCompatibility;
    assert_eq!(packet.card(dim).unwrap().gate, DescriptorGate::Blocked);
    for c in &packet.consumers {
        if c.read_dimensions.contains(&dim) {
            assert!(
                c.is_hold(),
                "consumer `{}` reads destructive change but did not hold",
                c.consumer.as_str()
            );
            assert!(c.requires_pre_restart_acknowledgement);
            assert!(c
                .gaps
                .iter()
                .any(|g| g.dimension == dim && g.gap_kind == ImpactGapKind::ResolveBeforeRestart));
        } else {
            assert!(
                c.is_clear(),
                "consumer `{}` should stay clear",
                c.consumer.as_str()
            );
        }
    }
    assert!(packet.requires_pre_restart_acknowledgement());
    assert!(packet.summary.hold_consumers >= 1);
}

#[test]
fn speculative_forecast_is_never_a_hard_failure() {
    // Guardrail: do not present speculative forecasts as hard failures; label unknown inputs honestly.
    let packet = seeded_m5_change_impact_card_set_speculative();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let dim = ImpactDimension::BehaviorChange;
    let card = packet.card(dim).unwrap();
    // The underlying risk is destructive, but unknown inputs cap the card at narrowed.
    assert_eq!(card.risk_class, RiskClass::DestructiveChange);
    assert_eq!(card.confidence, ForecastConfidence::Unknown);
    assert!(card.speculative);
    assert_eq!(card.gate, DescriptorGate::Narrowed);
    assert!(!card.requires_pre_restart_acknowledgement);
    assert!(card.unknown_input_message_id.is_some());
    // No consumer is forced into a hold; the unknown input is named as a gap.
    assert!(!packet.requires_pre_restart_acknowledgement());
    for c in &packet.consumers {
        if c.read_dimensions.contains(&dim) {
            assert!(c.is_review());
            assert!(c
                .gaps
                .iter()
                .any(|g| g.dimension == dim && g.gap_kind == ImpactGapKind::ForecastInputUnknown));
        }
    }
    assert!(packet.coverage.has_partial_coverage);
    assert_eq!(packet.coverage.unknown_input_cards, 1);
}

#[test]
fn tampering_a_speculative_card_to_blocked_is_rejected() {
    // The guardrail is enforced in validation, not just in the builder.
    let mut packet = seeded_m5_change_impact_card_set_speculative();
    let idx = packet
        .cards
        .iter()
        .position(|c| c.speculative)
        .expect("a speculative card exists");
    packet.cards[idx].gate = DescriptorGate::Blocked;
    let violations = packet.validate();
    assert!(
        violations.contains(&ChangeImpactViolation::SpeculativeHardFailure)
            || violations.contains(&ChangeImpactViolation::CardDerivationDrift),
        "{violations:?}"
    );
}

#[test]
fn forecast_coverage_labels_partial_inputs_honestly() {
    // Guardrail: partial coverage is disclosed rather than implied complete.
    let canonical = packet();
    // The canonical set has a not-applicable dimension but no speculative cards.
    assert!(!canonical.coverage.has_partial_coverage);
    assert!(canonical.coverage.not_applicable_cards >= 1);

    let speculative = seeded_m5_change_impact_card_set_speculative();
    assert!(speculative.coverage.has_partial_coverage);
    assert_eq!(speculative.summary.speculative_cards, 1);
}

#[test]
fn consumers_read_one_card_set() {
    // Acceptance criterion: every consumer reads one card set and derives its disclosed scope from it.
    let packet = packet();
    assert_eq!(
        packet.consumer_tokens,
        tokens(&ImpactConsumer::ALL, |c| c.as_str())
    );
    assert!(packet.disclosure.all_consume());
    assert!(packet.conformance.consumers_read_one_card_set);
    for c in &packet.consumers {
        let mut expected: Vec<ArtifactClass> = Vec::new();
        for &dimension in &c.read_dimensions {
            expected.extend(
                packet
                    .card(dimension)
                    .unwrap()
                    .affected_artifact_classes
                    .iter()
                    .copied(),
            );
        }
        expected.sort_by_key(|x| artifact_rank(*x));
        expected.dedup();
        assert_eq!(c.disclosed_artifact_classes, expected);
    }
}

#[test]
fn channels_produce_identical_output() {
    let packet = packet();
    let desktop = packet.render_for_channel(ImpactChannel::DesktopUi);
    let cli = packet.render_for_channel(ImpactChannel::CliHeadless);
    let export = packet.render_for_channel(ImpactChannel::OfflineExport);
    assert_eq!(desktop, cli);
    assert_eq!(cli, export);
    assert_eq!(desktop, packet.export_safe_json());
}

#[test]
fn controlled_vocabulary_is_frozen() {
    let vocab = ImpactVocabulary::canonical();
    assert_eq!(vocab.dimensions.len(), ImpactDimension::ALL.len());
    assert_eq!(vocab.consumers.len(), ImpactConsumer::ALL.len());
    for needle in [
        "workspace_migration",
        "schema_migration",
        "cache_migration",
        "extension_compatibility",
        "remote_helper_skew",
        "toolchain_floor",
        "toolchain_ceiling",
        "behavior_change",
    ] {
        assert!(
            vocab.dimensions.contains(&needle.to_owned()),
            "missing {needle}"
        );
    }
    for needle in [
        "no_impact",
        "low_risk_cache_churn",
        "habit_breaking_behavior_change",
        "destructive_change",
    ] {
        assert!(
            vocab.risk_classes.contains(&needle.to_owned()),
            "missing {needle}"
        );
    }
    for needle in ["rollback_supported", "pin_current_version", "no_rollback"] {
        assert!(
            vocab.rollback_choices.contains(&needle.to_owned()),
            "missing {needle}"
        );
    }
}

#[test]
fn packet_round_trips() {
    for packet in [
        seeded_m5_change_impact_card_set(),
        seeded_m5_change_impact_card_set_review(),
        seeded_m5_change_impact_card_set_hold(),
        seeded_m5_change_impact_card_set_speculative(),
    ] {
        let json = packet.export_safe_json();
        let parsed: ChangeImpactCardSet = serde_json::from_str(&json).expect("packet deserializes");
        assert_eq!(parsed, packet);
        assert!(parsed.validate().is_empty(), "{:?}", parsed.validate());
    }
}

#[test]
fn card_csv_enumerates_every_card() {
    let csv = packet().render_card_csv();
    let header = csv.lines().next().unwrap();
    assert!(header.starts_with("dimension,risk_class,confidence,"));
    assert!(header.contains("rollback_choice"));
    let rows = csv.lines().count() - 1;
    assert_eq!(rows, packet().cards.len());
}

#[test]
fn markdown_summary_names_dimensions_and_consumers() {
    let md = seeded_m5_change_impact_card_set_review().render_markdown_summary();
    assert!(md.contains("change-impact cards"));
    assert!(md.contains("Change-impact cards"));
    assert!(md.contains("schema_migration"));
    assert!(md.contains("gap:"));
}

#[test]
fn tampered_consumer_verdict_is_rejected() {
    let mut packet = seeded_m5_change_impact_card_set_review();
    let idx = packet
        .consumers
        .iter()
        .position(|c| c.is_review())
        .expect("a review consumer exists");
    packet.consumers[idx].gate_decision = DescriptorGate::Governed;
    packet.consumers[idx].review_readiness = ReviewReadiness::ClearToApply;
    assert!(packet
        .validate()
        .contains(&ChangeImpactViolation::ConsumerVerdictDrift));
}

#[test]
fn tampered_card_derivation_is_rejected() {
    let mut packet = packet();
    packet.cards[0].risk_class = RiskClass::DestructiveChange;
    let violations = packet.validate();
    assert!(
        violations.contains(&ChangeImpactViolation::CardDerivationDrift)
            || violations.contains(&ChangeImpactViolation::SummaryDrift),
        "{violations:?}"
    );
}

#[test]
fn dropping_a_dimension_is_rejected() {
    let mut packet = packet();
    packet
        .cards
        .retain(|c| c.dimension != ImpactDimension::ProfileMigration);
    assert!(packet
        .validate()
        .contains(&ChangeImpactViolation::DimensionCoverageDrift));
}

#[test]
fn export_carries_no_raw_material() {
    for packet in [
        seeded_m5_change_impact_card_set(),
        seeded_m5_change_impact_card_set_review(),
        seeded_m5_change_impact_card_set_hold(),
        seeded_m5_change_impact_card_set_speculative(),
    ] {
        assert!(packet.conformance.export_carries_no_raw_material);
        assert!(!packet
            .export_safe_json()
            .to_ascii_lowercase()
            .contains("bearer_token"));
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
