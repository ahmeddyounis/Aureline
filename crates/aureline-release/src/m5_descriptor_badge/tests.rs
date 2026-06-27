//! Inline tests for the M5 descriptor/badge matrix.

use super::*;

fn canonical() -> M5DescriptorBadgeMatrix {
    seeded_m5_descriptor_badge_matrix()
}

#[test]
fn canonical_packet_validates() {
    let packet = canonical();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DESCRIPTOR_BADGE_MATRIX_PACKET_ID);
    assert_eq!(packet.record_kind, M5_DESCRIPTOR_BADGE_RECORD_KIND);
    assert!(!packet.consumer_bindings.is_empty());
    assert_eq!(packet.descriptors.len(), DescriptorFamily::ALL.len());
    assert_eq!(
        packet.downgrade_rules.len(),
        canonical_downgrade_rules().len()
    );
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
    assert!(packet.disclosure.all_consume());
}

#[test]
fn every_family_has_a_descriptor_and_badge_family() {
    let packet = canonical();
    for family in DescriptorFamily::ALL {
        let d = packet.descriptor(family).expect("descriptor");
        assert!(d.validate().is_empty(), "{:?}", d.validate());
        assert_eq!(d.schema_ref, family.schema_ref());
        assert_eq!(d.proof_packet_ref, family.proof_packet_ref());
        assert_eq!(d.badge_family, family.badge_family());
        assert_eq!(d.value_tokens, d.badge_tokens);
        assert!(!d.value_tokens.is_empty());
    }
    for badge in BadgeFamily::ALL {
        assert!(packet.descriptors.iter().any(|d| d.badge_family == badge));
        assert_eq!(badge.descriptor_family().badge_family(), badge);
    }
}

#[test]
fn weaker_provenance_origins_are_first_class_tokens() {
    let packet = canonical();
    let provenance = packet
        .descriptor(DescriptorFamily::Provenance)
        .expect("provenance descriptor");
    for needle in ["mirror", "offline_bundle", "side_loaded", "not_provided"] {
        assert!(
            provenance.value_tokens.iter().any(|t| t == needle),
            "provenance vocabulary dropped `{needle}`"
        );
    }
    assert!(packet.conformance.weaker_origins_never_omitted);
}

#[test]
fn downgrade_rules_cover_every_weaker_value() {
    let packet = canonical();
    assert!(packet.conformance.downgrade_rules_cover_every_weaker_value);
    // Absent provenance and expired/missing freshness block; everything else narrows.
    let rule = |family: DescriptorFamily, token: &str| -> &DowngradeRule {
        packet
            .downgrade_rules
            .iter()
            .find(|r| r.trigger_family == family && r.trigger_token == token)
            .expect("rule")
    };
    assert_eq!(
        rule(DescriptorFamily::Provenance, "not_provided").effect,
        DowngradeEffect::Block
    );
    assert_eq!(
        rule(DescriptorFamily::Provenance, "mirror").effect,
        DowngradeEffect::Narrow
    );
    assert_eq!(
        rule(DescriptorFamily::Freshness, "stale").effect,
        DowngradeEffect::Narrow
    );
    assert_eq!(
        rule(DescriptorFamily::Freshness, "expired").effect,
        DowngradeEffect::Block
    );
    assert_eq!(
        rule(DescriptorFamily::ClientScope, "browser_reference").effect,
        DowngradeEffect::Narrow
    );
    // The authoritative origin and full desktop scope are never downgraded.
    assert!(!packet
        .downgrade_rules
        .iter()
        .any(|r| r.trigger_token == "first_party_signed" || r.trigger_token == "desktop_full"));
}

#[test]
fn canonical_consumers_are_all_governed() {
    let packet = canonical();
    for binding in &packet.consumer_bindings {
        assert!(
            binding.is_governed(),
            "{} not governed",
            binding.consumer.as_str()
        );
        assert!(
            binding.gaps.is_empty(),
            "{} has gaps",
            binding.consumer.as_str()
        );
        assert_eq!(binding.effective_qualification, QualificationClass::Stable);
        assert_eq!(binding.status, ConsumerStatus::Mapped);
        assert!(!binding.bound_families.is_empty());
        assert!(!binding.covered_badge_families.is_empty());
    }
    assert!(!packet.blocks_stable_promotion());
    assert_eq!(
        packet.summary.governed_consumer_count as usize,
        packet.consumer_bindings.len()
    );
    assert_eq!(packet.summary.blocked_consumer_count, 0);
    assert_eq!(packet.summary.narrowed_consumer_count, 0);
    assert_eq!(
        packet.summary.current_descriptor_count,
        DescriptorFamily::ALL.len() as u32
    );
}

#[test]
fn stale_descriptor_proof_narrows_bound_consumers_deterministically() {
    let packet = seeded_m5_descriptor_badge_matrix_stale_proof_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The freshness descriptor is stale; it does not block (nothing is missing).
    assert!(!packet.blocks_stable_promotion());

    let stale = packet
        .descriptor(DescriptorFamily::Freshness)
        .expect("freshness descriptor");
    assert_eq!(stale.proof_freshness, FreshnessState::Stale);

    for binding in &packet.consumer_bindings {
        let binds_freshness = binding
            .bound_families
            .contains(&DescriptorFamily::Freshness);
        if binds_freshness {
            assert!(
                binding.is_narrowed(),
                "{} should narrow",
                binding.consumer.as_str()
            );
            assert_eq!(binding.effective_qualification, QualificationClass::Beta);
            assert_eq!(binding.status, ConsumerStatus::Provisional);
            assert!(binding
                .gaps
                .iter()
                .any(|g| g.family == DescriptorFamily::Freshness
                    && g.gap_kind == DescriptorGapKind::ProofStale));
        } else {
            assert!(
                binding.is_governed(),
                "{} should stay governed",
                binding.consumer.as_str()
            );
            assert_eq!(binding.effective_qualification, QualificationClass::Stable);
        }
    }
    assert!(packet.summary.narrowed_consumer_count > 0);
    assert_eq!(packet.summary.stale_descriptor_count, 1);
}

#[test]
fn missing_descriptor_proof_blocks_bound_consumers() {
    let packet = seeded_m5_descriptor_badge_matrix_missing_proof_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.blocks_stable_promotion());

    let missing = packet
        .descriptor(DescriptorFamily::ClientScope)
        .expect("client-scope descriptor");
    assert_eq!(missing.proof_freshness, FreshnessState::Missing);

    for binding in &packet.consumer_bindings {
        let binds_scope = binding
            .bound_families
            .contains(&DescriptorFamily::ClientScope);
        if binds_scope {
            assert!(
                binding.is_blocked(),
                "{} should block",
                binding.consumer.as_str()
            );
            assert_eq!(
                binding.effective_qualification,
                QualificationClass::Unavailable
            );
            assert_eq!(binding.status, ConsumerStatus::Unmapped);
            assert!(binding
                .gaps
                .iter()
                .any(|g| g.family == DescriptorFamily::ClientScope
                    && g.gap_kind == DescriptorGapKind::ProofMissing));
        } else {
            assert!(
                !binding.is_blocked(),
                "{} should not block",
                binding.consumer.as_str()
            );
        }
    }
    assert!(packet.summary.blocked_consumer_count > 0);
    assert_eq!(packet.summary.missing_descriptor_count, 1);
    assert!(packet
        .release_gate
        .blocked_consumers
        .contains(&"release_center".to_owned()));
}

#[test]
fn consumer_verdict_drift_is_detected() {
    let mut packet = canonical();
    // Hand-edit a consumer's effective qualification to a stale value; validation catches it.
    packet.consumer_bindings[0].effective_qualification = QualificationClass::Preview;
    let violations = packet.validate();
    assert!(violations.contains(&M5DescriptorBadgeViolation::ConsumerVerdictDrift));
}

#[test]
fn downgrade_rule_drift_is_detected() {
    let mut packet = canonical();
    packet.downgrade_rules.pop();
    let violations = packet.validate();
    assert!(violations.contains(&M5DescriptorBadgeViolation::DowngradeRulesDrift));
}

#[test]
fn unmapped_descriptor_blocks_consumer() {
    // A consumer that binds a family the matrix does not govern must block, with a named gap.
    let descriptors: Vec<DescriptorContract> = DescriptorFamily::ALL
        .iter()
        .filter(|f| **f != DescriptorFamily::ClientScope)
        .map(|f| DescriptorContract::for_family(*f, FreshnessState::Current))
        .collect();
    let mut binding = ConsumerBinding::new(
        PublicTruthConsumer::Marketplace,
        QualificationClass::Stable,
        &[DescriptorFamily::Provenance, DescriptorFamily::ClientScope],
    );
    binding.recompute(&descriptors);
    assert!(binding.is_blocked());
    assert_eq!(
        binding.effective_qualification,
        QualificationClass::Unavailable
    );
    assert!(binding
        .gaps
        .iter()
        .any(|g| g.family == DescriptorFamily::ClientScope
            && g.gap_kind == DescriptorGapKind::DescriptorMappingMissing));
}

#[test]
fn export_carries_no_raw_material() {
    let packet = canonical();
    let json = packet.export_safe_json();
    let lower = json.to_ascii_lowercase();
    for needle in [
        "credential",
        "secret",
        "password",
        "api_key",
        "bearer_token",
    ] {
        assert!(!lower.contains(needle), "export leaked `{needle}`");
    }
}

#[test]
fn markdown_summary_renders_descriptors_consumers_and_rules() {
    let packet = canonical();
    let md = packet.render_markdown_summary();
    assert!(md.contains("# M5 Descriptor / Badge Governance Matrix"));
    assert!(md.contains("Descriptor objects and badge families"));
    assert!(md.contains("Downgrade rules"));
    assert!(md.contains("Public-truth consumers"));
    assert!(md.contains("release center, Help/About, marketplace, docs/help, support, companion"));
}

#[test]
fn standalone_descriptor_contracts_validate() {
    for family in DescriptorFamily::ALL {
        let contract = seeded_descriptor_contract(family, FreshnessState::Current);
        assert!(contract.validate().is_empty(), "{:?}", contract.validate());
        assert_eq!(contract.family, family);
        let json = serde_json::to_string_pretty(&contract).expect("serializes");
        let back: DescriptorContract = serde_json::from_str(&json).expect("round trips");
        assert_eq!(back, contract);
    }
}

#[test]
fn drill_packets_serialize_round_trip() {
    for packet in [
        seeded_m5_descriptor_badge_matrix(),
        seeded_m5_descriptor_badge_matrix_stale_proof_narrowed(),
        seeded_m5_descriptor_badge_matrix_missing_proof_blocked(),
    ] {
        let json = packet.export_safe_json();
        let back: M5DescriptorBadgeMatrix = serde_json::from_str(&json).expect("round trips");
        assert_eq!(back, packet);
        assert!(back.validate().is_empty(), "{:?}", back.validate());
    }
}
