use super::*;

fn seed() -> M5SettingsGovernanceSharedConsumersPacket {
    seeded_m5_settings_governance_shared_consumers()
}

fn violations_of(packet: &M5SettingsGovernanceSharedConsumersPacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(
        packet.packet_id,
        M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_PACKET_ID
    );
    assert_eq!(
        packet.record_kind,
        M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_RECORD_KIND
    );
    assert_eq!(packet.consumer_bindings.len(), 15);
}

#[test]
fn every_family_is_adopted_by_two_or_more_consumers() {
    let packet = seed();
    let mut family_consumers: BTreeMap<
        M5SettingsGovernanceFamily,
        BTreeSet<M5SettingsGovernanceConsumerSurface>,
    > = BTreeMap::new();
    for binding in &packet.consumer_bindings {
        family_consumers
            .entry(binding.family)
            .or_default()
            .insert(binding.consumer);
    }
    assert_eq!(family_consumers.len(), 5, "all five families adopted");
    for (family, consumers) in &family_consumers {
        assert!(
            consumers.len() >= 2,
            "family {} only adopted by {} consumers",
            family.as_str(),
            consumers.len()
        );
    }
}

#[test]
fn every_consumer_surface_and_representation_is_exercised() {
    let packet = seed();
    let consumers: BTreeSet<_> = packet
        .consumer_bindings
        .iter()
        .map(|b| b.consumer)
        .collect();
    for consumer in M5SettingsGovernanceConsumerSurface::ALL {
        assert!(
            consumers.contains(&consumer),
            "consumer {} missing",
            consumer.as_str()
        );
    }
    let representations: BTreeSet<_> = packet
        .consumer_bindings
        .iter()
        .map(|b| b.representation)
        .collect();
    for representation in SettingsGovernanceRepresentation::ALL {
        assert!(
            representations.contains(&representation),
            "representation {} missing",
            representation.as_str()
        );
    }
}

#[test]
fn same_profile_carries_identical_grammar_across_surfaces() {
    let packet = seed();
    let mut profile_facets: BTreeMap<&str, &SettingsGovernanceStateFacetValues> = BTreeMap::new();
    for binding in &packet.consumer_bindings {
        match profile_facets.get(binding.governance_profile_id.as_str()) {
            None => {
                profile_facets.insert(
                    binding.governance_profile_id.as_str(),
                    &binding.state_facets,
                );
            }
            Some(existing) => assert_eq!(
                **existing, binding.state_facets,
                "grammar drift on {}",
                binding.governance_profile_id
            ),
        }
    }
    // The five profiles each fan out to more than one consumer.
    assert_eq!(profile_facets.len(), 5);
}

#[test]
fn every_settings_governance_role_word_is_a_frozen_role_token() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        assert!(
            binding
                .state_facets
                .settings_governance_role_word_in_vocabulary(),
            "role word `{}` on {} is not a frozen role token",
            binding.state_facets.settings_governance_role_word,
            binding.binding_id
        );
        assert!(binding.state_facets.all_present());
        assert!(binding.state_facets.evidence_continuity_satisfied());
    }
}

#[test]
fn narrowed_bindings_disclose_and_full_bindings_do_not() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        let disclosure = binding.disclosure();
        if binding.is_narrowed() {
            assert_eq!(
                binding.parity_state,
                SettingsGovernanceParityState::FacetsDisclosedNarrowed
            );
            let note = binding
                .narrow_note
                .as_ref()
                .expect("narrowed binding carries a note");
            assert_eq!(Some(note.reason), disclosure.narrow_reason);
            assert_eq!(Some(note.next_action), disclosure.narrow_next_action);
            assert!(!note.preserved_grammar_note.trim().is_empty());
            assert!(!note.next_action_label.trim().is_empty());
        } else {
            assert_eq!(
                binding.parity_state,
                SettingsGovernanceParityState::FacetsPreserved
            );
            assert!(binding.narrow_note.is_none());
        }
        if matches!(
            binding.representation,
            SettingsGovernanceRepresentation::RemoteProjected
        ) {
            assert!(!binding.remote_source_note.trim().is_empty());
        }
        if matches!(
            binding.representation,
            SettingsGovernanceRepresentation::ExportedRedacted
        ) {
            assert!(!binding.export_detail_note.trim().is_empty());
        }
    }
}

#[test]
fn support_and_export_bindings_point_at_canonical_contracts() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        if consumer_must_reference_canonical(binding.consumer) {
            assert!(
                binding.points_at_canonical_contracts(),
                "binding {} must point at canonical contracts",
                binding.binding_id
            );
        }
    }
}

#[test]
fn disclosure_resolver_matches_representation() {
    assert!(
        !resolve_settings_governance_render_disclosure(
            SettingsGovernanceRepresentation::DesktopFull
        )
        .needs_narrow_note
    );
    let compact = resolve_settings_governance_render_disclosure(
        SettingsGovernanceRepresentation::CompactNarrowed,
    );
    assert_eq!(
        compact.narrow_reason,
        Some(SettingsGovernanceNarrowReason::CompactionNarrowed)
    );
    assert!(compact.needs_narrow_note);
    assert!(!compact.needs_remote_source_note);
    let remote = resolve_settings_governance_render_disclosure(
        SettingsGovernanceRepresentation::RemoteProjected,
    );
    assert!(remote.needs_remote_source_note);
    let exported = resolve_settings_governance_render_disclosure(
        SettingsGovernanceRepresentation::ExportedRedacted,
    );
    assert!(exported.needs_export_detail_note);
}

#[test]
fn grammar_drift_is_rejected() {
    let mut packet = seed();
    // Reword one surface of a multi-binding profile to a different (still-valid) role token.
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.binding_id == "sgsc-write-setting-shell")
        .unwrap();
    packet.consumer_bindings[target]
        .state_facets
        .settings_governance_role_word = "policy_constraint".to_owned();
    assert!(violations_of(&packet).contains(&"settings_governance_grammar_drift_across_surfaces"));
}

#[test]
fn settings_governance_role_word_outside_vocabulary_is_rejected() {
    let mut packet = seed();
    packet.consumer_bindings[0]
        .state_facets
        .settings_governance_role_word = "totally_made_up".to_owned();
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"settings_governance_role_word_outside_vocabulary"));
    // Rewording one surface also trips drift, which is expected and fine.
}

#[test]
fn dropped_evidence_continuity_on_trust_role_is_rejected() {
    let mut packet = seed();
    // sgsc-write-setting-policy carries the write_intent role, which must always keep a real
    // evidence continuity.
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.binding_id == "sgsc-write-setting-policy")
        .unwrap();
    packet.consumer_bindings[target]
        .state_facets
        .evidence_continuity_word = "widened_scoped_write_into_broader_scope".to_owned();
    assert!(violations_of(&packet).contains(&"evidence_continuity_missing_for_trust_role"));
}

#[test]
fn reuse_below_two_consumers_is_rejected() {
    let mut packet = seed();
    // Drop every write-setting binding except one, leaving the family with one consumer.
    let mut kept_one = false;
    packet.consumer_bindings.retain(|b| {
        if b.family == M5SettingsGovernanceFamily::WriteSetting {
            if kept_one {
                return false;
            }
            kept_one = true;
        }
        true
    });
    assert!(violations_of(&packet).contains(&"family_reuse_unproven"));
}

#[test]
fn missing_canonical_reference_on_export_is_rejected() {
    let mut packet = seed();
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| consumer_must_reference_canonical(b.consumer))
        .unwrap();
    packet.consumer_bindings[target].source_contract_refs =
        vec![M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF.to_owned()];
    assert!(violations_of(&packet).contains(&"support_export_reference_missing"));
}

#[test]
fn missing_narrow_note_is_rejected() {
    let mut packet = seed();
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .unwrap();
    packet.consumer_bindings[target].narrow_note = None;
    assert!(violations_of(&packet).contains(&"narrow_note_missing"));
}

#[test]
fn unexpected_narrow_note_on_full_binding_is_rejected() {
    let mut packet = seed();
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| !b.is_narrowed())
        .unwrap();
    packet.consumer_bindings[target].narrow_note = Some(SettingsGovernanceNarrowNote {
        reason: SettingsGovernanceNarrowReason::CompactionNarrowed,
        preserved_grammar_note: "x".to_owned(),
        next_action: SettingsGovernanceNarrowNextAction::ExpandInDesktop,
        next_action_label: "x".to_owned(),
    });
    assert!(violations_of(&packet).contains(&"unexpected_narrow_note"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [(fn(&mut SettingsGovernanceConsumerBinding), &str); 5] = [
        (
            |b| b.recycles_a_retired_setting_id = true,
            "recycles_a_retired_setting_id",
        ),
        (
            |b| b.rewrites_a_scoped_write_into_a_broader_scope = true,
            "rewrites_a_scoped_write_into_a_broader_scope",
        ),
        (
            |b| b.silently_overwrites_locked_or_machine_only_state_during_sync = true,
            "silently_overwrites_locked_or_machine_only_state_during_sync",
        ),
        (
            |b| b.hides_lifecycle_or_experiment_dependency_behind_unpublished_markers = true,
            "hides_lifecycle_or_experiment_dependency_behind_unpublished_markers",
        ),
        (
            |b| b.hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy = true,
            "hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy",
        ),
    ];
    for (mutate, token) in cases {
        let mut packet = seed();
        mutate(&mut packet.consumer_bindings[0]);
        assert!(violations_of(&packet).contains(&token), "expected {token}");
    }
}

#[test]
fn family_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .consumer_bindings
        .retain(|b| b.family != M5SettingsGovernanceFamily::RolloutCapability);
    assert!(violations_of(&packet).contains(&"family_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_DOC_REF);
    assert!(violations_of(&packet).contains(&"missing_source_contracts"));
}

#[test]
fn export_json_is_boundary_safe() {
    let json = seed().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_binding() {
    let packet = seed();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.consumer_bindings.len());
    assert!(lines[0]
        .starts_with("family,consumer,representation,settings_governance_role_word,parity_state"));
}

#[test]
fn markdown_summary_lists_every_profile() {
    let packet = seed();
    let summary = packet.render_markdown_summary();
    for binding in &packet.consumer_bindings {
        assert!(summary.contains(&binding.binding_id));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_settings_governance_shared_consumers_export()
        .expect("checked M5 settings-governance shared-consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let compact = seeded_m5_settings_governance_shared_consumers_compact_remote_narrowed();
    assert!(
        compact.validate().is_empty(),
        "{:?}",
        violations_of(&compact)
    );
    assert_eq!(compact.consumer_bindings.len(), 15);

    let exported = seeded_m5_settings_governance_shared_consumers_exported_redaction_narrowed();
    assert!(
        exported.validate().is_empty(),
        "{:?}",
        violations_of(&exported)
    );
    assert_eq!(exported.consumer_bindings.len(), 15);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let compact: M5SettingsGovernanceSharedConsumersPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/config/m5-settings-governance-shared-consumers/compact_remote_narrowed.json"
        )))
        .expect("compact fixture parses");
    assert!(compact.validate().is_empty());
    assert_eq!(
        compact,
        seeded_m5_settings_governance_shared_consumers_compact_remote_narrowed()
    );

    let exported: M5SettingsGovernanceSharedConsumersPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/config/m5-settings-governance-shared-consumers/exported_redaction_narrowed.json"
        )))
        .expect("exported fixture parses");
    assert!(exported.validate().is_empty());
    assert_eq!(
        exported,
        seeded_m5_settings_governance_shared_consumers_exported_redaction_narrowed()
    );
}
