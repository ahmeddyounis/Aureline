use super::*;

#[test]
fn seeded_catalog_validates() {
    let packet = seeded_m5_announcement_grammar_catalog();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_ANNOUNCEMENT_GRAMMAR_CATALOG_PACKET_ID);
}

#[test]
fn seeded_catalog_covers_every_event_class() {
    let packet = seeded_m5_announcement_grammar_catalog();
    let present: std::collections::BTreeSet<_> =
        packet.classes.iter().map(|c| c.event_class).collect();
    for event_class in M5AnnouncementEventClass::ALL {
        assert!(
            present.contains(&event_class),
            "missing event class {}",
            event_class.as_str()
        );
    }
}

#[test]
fn missing_event_class_fails_validation() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet
        .classes
        .retain(|c| c.event_class != M5AnnouncementEventClass::DegradedOrStaleTruth);
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::RequiredEventClassMissing));
}

#[test]
fn shared_vocabulary_drift_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.shared_vocabulary_set.coalescing_strategies.pop();
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::VocabularySetDrift));
}

#[test]
fn grammar_vocabulary_drift_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.grammar_vocabulary_set.event_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::VocabularySetDrift));
}

#[test]
fn duplicate_class_id_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    let mut clone = packet.classes[0].clone();
    // Keep the event class distinct so the duplicate-id check is what fires.
    clone.event_class = M5AnnouncementEventClass::BlockerRaised;
    clone.channel = A11yAnnouncementPoliteness::Assertive;
    packet.classes.push(clone);
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::DuplicateClassId));
}

#[test]
fn duplicate_event_class_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    let mut clone = packet.classes[0].clone();
    clone.class_id = "announcement:mode-or-state-change-dup".to_owned();
    packet.classes.push(clone);
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::DuplicateEventClass));
}

#[test]
fn message_id_without_prefix_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.classes[0].message_template.message_id = "mode_change.entered".to_owned();
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::MessageIdPrefixMissing));
}

#[test]
fn undeclared_template_token_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    // Reference a placeholder the declaration does not carry — the kind of
    // ad hoc concatenation the grammar forbids.
    packet.classes[0].message_template.template =
        "{surface_name} entered {mode_name} mode at {timestamp}.".to_owned();
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::MessageTemplatePlaceholderMismatch));
}

#[test]
fn orphan_declared_placeholder_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    // Declare a placeholder the template never inserts.
    packet.classes[0]
        .message_template
        .placeholders
        .push(M5AnnouncementPlaceholder {
            name: "unused".to_owned(),
            value_kind: M5AnnouncementValueKind::DurationLabel,
            required: false,
        });
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::MessageTemplatePlaceholderMismatch));
}

#[test]
fn malformed_template_braces_fail() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.classes[0].message_template.template = "{surface_name entered mode.".to_owned();
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::MessageTemplatePlaceholderMismatch));
}

#[test]
fn required_fields_not_matching_placeholders_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.classes[0].required_fields = vec!["surface_name".to_owned()];
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::RequiredFieldPlaceholderMismatch));
}

#[test]
fn non_blocker_assertive_channel_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    let class = packet
        .classes
        .iter_mut()
        .find(|c| c.event_class == M5AnnouncementEventClass::ProgressMilestone)
        .expect("progress milestone class present");
    class.channel = A11yAnnouncementPoliteness::Assertive;
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::ChannelRuleViolated));
}

#[test]
fn blocker_without_assertive_channel_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    let class = packet
        .classes
        .iter_mut()
        .find(|c| c.event_class == M5AnnouncementEventClass::BlockerRaised)
        .expect("blocker class present");
    class.channel = A11yAnnouncementPoliteness::Polite;
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::ChannelRuleViolated));
}

#[test]
fn coalescing_strategy_none_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.classes[0].coalescing_budget.strategy = A11yCoalescingStrategy::None;
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::CoalescingStrategyMissing));
}

#[test]
fn zero_budget_window_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.classes[0].coalescing_budget.window_seconds = 0;
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::CoalescingBudgetInvalid));
}

#[test]
fn zero_budget_max_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.classes[0]
        .coalescing_budget
        .max_announcements_per_window = 0;
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::CoalescingBudgetInvalid));
}

#[test]
fn missing_suppression_rules_fail() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.classes[0].suppression_rules.clear();
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::SuppressionRulesMissing));
}

#[test]
fn non_reopenable_durable_fallback_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.classes[0].durable_fallback.reopenable = false;
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::DurableFallbackMissing));
}

#[test]
fn empty_durable_fallback_ref_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.classes[0].durable_fallback.surface_ref = "  ".to_owned();
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::DurableFallbackMissing));
}

#[test]
fn every_class_points_to_a_reopenable_durable_fallback() {
    let packet = seeded_m5_announcement_grammar_catalog();
    for class in &packet.classes {
        assert!(
            class.durable_fallback.reopenable
                && !class.durable_fallback.surface_ref.trim().is_empty(),
            "class {} lacks a reopenable durable fallback",
            class.class_id
        );
    }
}

#[test]
fn only_blocker_uses_assertive_channel() {
    let packet = seeded_m5_announcement_grammar_catalog();
    for class in &packet.classes {
        let assertive = class.channel == A11yAnnouncementPoliteness::Assertive;
        assert_eq!(
            assertive,
            class.event_class == M5AnnouncementEventClass::BlockerRaised,
            "class {} has an unexpected assertive posture",
            class.class_id
        );
    }
}

#[test]
fn stable_class_missing_proof_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.classes[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::StableClassMissingProof));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.classes[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.classes[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::MissingSourceContracts));
}

#[test]
fn conformance_review_incomplete_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet
        .conformance_review
        .every_high_value_announcement_has_durable_fallback = false;
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::ConformanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.consumer_projection.notifications_consume_grammar = false;
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_announcement_grammar_catalog();
    packet
        .release_posture
        .stable_promotion_blocks_without_mapped_proof = false;
    assert!(packet
        .validate()
        .contains(&M5AnnouncementGrammarViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_class() {
    let summary = seeded_m5_announcement_grammar_catalog().render_markdown_summary();
    for class in seeded_m5_announcement_grammar_catalog().classes {
        assert!(
            summary.contains(&class.class_id),
            "summary missing class {}",
            class.class_id
        );
        assert!(
            summary.contains(&class.message_template.message_id),
            "summary missing message id {}",
            class.message_template.message_id
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_announcement_grammar_export()
        .expect("checked M5 announcement grammar export validates");
    assert_eq!(packet.packet_id, M5_ANNOUNCEMENT_GRAMMAR_CATALOG_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_announcement_grammar_export()
        .expect("checked M5 announcement grammar export validates");
    assert_eq!(
        from_disk,
        seeded_m5_announcement_grammar_catalog(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_classes_visible() {
    for packet in [
        seeded_m5_announcement_grammar_catalog_proof_stale_narrowed(),
        seeded_m5_announcement_grammar_catalog_live_region_unavailable_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        // Downgrade narrows the claim without removing the class.
        assert_eq!(packet.classes.len(), M5AnnouncementEventClass::ALL.len());
    }

    let proof_stale = seeded_m5_announcement_grammar_catalog_proof_stale_narrowed();
    let success = proof_stale
        .classes
        .iter()
        .find(|c| c.event_class == M5AnnouncementEventClass::SuccessWithRecovery)
        .expect("success-with-recovery class present");
    assert_eq!(
        success.qualification,
        M5DynamicSurfaceA11yQualificationClass::Beta
    );

    let no_live_region = seeded_m5_announcement_grammar_catalog_live_region_unavailable_narrowed();
    let progress = no_live_region
        .classes
        .iter()
        .find(|c| c.event_class == M5AnnouncementEventClass::ProgressMilestone)
        .expect("progress-milestone class present");
    assert_eq!(
        progress.qualification,
        M5DynamicSurfaceA11yQualificationClass::Preview
    );
    assert_eq!(
        progress.fallback_durability,
        A11yFallbackDurability::DurableSurfaceOnly
    );
    // The announcement still has a durable counterpart with the live region gone.
    assert!(progress.durable_fallback.reopenable);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-announcements/proof_stale_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-announcements/live_region_unavailable_narrowed.json"
        )),
    ] {
        let packet: M5AnnouncementGrammarCatalogPacket =
            serde_json::from_str(raw).expect("fixture parses as announcement grammar packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_announcement_grammar_catalog().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}
