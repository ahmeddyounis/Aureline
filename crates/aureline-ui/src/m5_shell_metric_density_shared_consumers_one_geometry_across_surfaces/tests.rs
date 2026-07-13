use super::*;

fn seed() -> M5ShellGeometrySharedConsumersPacket {
    seeded_m5_shell_metric_density_shared_consumers()
}

fn violations_of(packet: &M5ShellGeometrySharedConsumersPacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(
        packet.packet_id,
        M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_PACKET_ID
    );
    assert_eq!(
        packet.record_kind,
        M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_RECORD_KIND
    );
    assert_eq!(packet.consumer_bindings.len(), 15);
}

#[test]
fn every_family_is_adopted_by_two_or_more_consumers() {
    let packet = seed();
    let mut family_consumers: BTreeMap<
        M5ShellGeometryFamily,
        BTreeSet<M5ShellGeometryConsumerSurface>,
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
    for consumer in M5ShellGeometryConsumerSurface::ALL {
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
    for representation in ShellGeometryRepresentation::ALL {
        assert!(
            representations.contains(&representation),
            "representation {} missing",
            representation.as_str()
        );
    }
}

#[test]
fn same_object_carries_identical_grammar_across_surfaces() {
    let packet = seed();
    let mut object_facets: BTreeMap<&str, &ShellGeometryStateFacetValues> = BTreeMap::new();
    for binding in &packet.consumer_bindings {
        match object_facets.get(binding.geometry_object_id.as_str()) {
            None => {
                object_facets.insert(binding.geometry_object_id.as_str(), &binding.state_facets);
            }
            Some(existing) => assert_eq!(
                **existing, binding.state_facets,
                "grammar drift on {}",
                binding.geometry_object_id
            ),
        }
    }
    // The five objects each fan out to more than one consumer.
    assert_eq!(object_facets.len(), 5);
}

#[test]
fn every_geometry_role_word_is_a_frozen_role_token() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        assert!(
            binding.state_facets.geometry_role_word_in_vocabulary(),
            "role word `{}` on {} is not a frozen role token",
            binding.state_facets.geometry_role_word,
            binding.binding_id
        );
        assert!(binding.state_facets.all_present());
        assert!(binding.state_facets.minimum_guarantee_satisfied());
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
                ShellGeometryParityState::FacetsDisclosedNarrowed
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
                ShellGeometryParityState::FacetsPreserved
            );
            assert!(binding.narrow_note.is_none());
        }
        if matches!(
            binding.representation,
            ShellGeometryRepresentation::RemoteProjected
        ) {
            assert!(!binding.remote_source_note.trim().is_empty());
        }
        if matches!(
            binding.representation,
            ShellGeometryRepresentation::ExportedRedacted
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
        !resolve_shell_geometry_render_disclosure(ShellGeometryRepresentation::DesktopFull)
            .needs_narrow_note
    );
    let compact =
        resolve_shell_geometry_render_disclosure(ShellGeometryRepresentation::CompactNarrowed);
    assert_eq!(
        compact.narrow_reason,
        Some(ShellGeometryNarrowReason::CompactionNarrowed)
    );
    assert!(compact.needs_narrow_note);
    assert!(!compact.needs_remote_source_note);
    let remote =
        resolve_shell_geometry_render_disclosure(ShellGeometryRepresentation::RemoteProjected);
    assert!(remote.needs_remote_source_note);
    let exported =
        resolve_shell_geometry_render_disclosure(ShellGeometryRepresentation::ExportedRedacted);
    assert!(exported.needs_export_detail_note);
}

#[test]
fn grammar_drift_is_rejected() {
    let mut packet = seed();
    // Reword one surface of a multi-binding object to a different (still-valid) role token.
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.binding_id == "sg-density-notebook")
        .unwrap();
    packet.consumer_bindings[target]
        .state_facets
        .geometry_role_word = "collapse".to_owned();
    assert!(violations_of(&packet).contains(&"geometry_grammar_drift_across_surfaces"));
}

#[test]
fn geometry_role_word_outside_vocabulary_is_rejected() {
    let mut packet = seed();
    packet.consumer_bindings[0].state_facets.geometry_role_word = "totally_made_up".to_owned();
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"geometry_role_word_outside_vocabulary"));
    // Rewording one surface also trips drift, which is expected and fine.
}

#[test]
fn dropped_minimum_guarantee_on_adaptive_role_is_rejected() {
    let mut packet = seed();
    // sg-density-shell carries the density role, which must always keep a real minimum guarantee.
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.binding_id == "sg-density-shell")
        .unwrap();
    packet.consumer_bindings[target]
        .state_facets
        .minimum_guarantee_word = "overlay_only".to_owned();
    assert!(violations_of(&packet).contains(&"minimum_guarantee_missing_for_adaptive_role"));
}

#[test]
fn reuse_below_two_consumers_is_rejected() {
    let mut packet = seed();
    // Drop every density-mode binding except one, leaving the family with one consumer.
    let mut kept_one = false;
    packet.consumer_bindings.retain(|b| {
        if b.family == M5ShellGeometryFamily::DensityMode {
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
        vec![M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF.to_owned()];
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
    packet.consumer_bindings[target].narrow_note = Some(ShellGeometryNarrowNote {
        reason: ShellGeometryNarrowReason::CompactionNarrowed,
        preserved_grammar_note: "x".to_owned(),
        next_action: ShellGeometryNarrowNextAction::ExpandInDesktop,
        next_action_label: "x".to_owned(),
    });
    assert!(violations_of(&packet).contains(&"unexpected_narrow_note"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [(fn(&mut ShellGeometryConsumerBinding), &str); 5] = [
        (
            |b| b.density_or_collapse_changes_command_focus_or_trust = true,
            "density_or_collapse_changes_command_focus_or_trust",
        ),
        (
            |b| b.extension_or_embedded_sets_private_fracturing_width = true,
            "extension_sets_private_fracturing_width",
        ),
        (
            |b| b.shrinks_hit_target_below_supported_minimum = true,
            "shrinks_hit_target_below_supported_minimum",
        ),
        (
            |b| b.hides_primary_workflow_behind_overlay_only_fallback = true,
            "hides_primary_workflow_behind_overlay_only_fallback",
        ),
        (
            |b| b.lets_zone_starve_main_workspace_below_minimum = true,
            "lets_zone_starve_main_workspace_below_minimum",
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
        .retain(|b| b.family != M5ShellGeometryFamily::CollapsePriority);
    assert!(violations_of(&packet).contains(&"family_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_DOC_REF);
    assert!(violations_of(&packet).contains(&"missing_source_contracts"));
}

#[test]
fn export_json_is_boundary_safe() {
    let json = seed().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("secret"));
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}

#[test]
fn csv_has_a_row_per_binding() {
    let packet = seed();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.consumer_bindings.len());
    assert!(lines[0].starts_with("family,consumer,representation,geometry_role_word,parity_state"));
}

#[test]
fn markdown_summary_lists_every_object() {
    let packet = seed();
    let summary = packet.render_markdown_summary();
    for binding in &packet.consumer_bindings {
        assert!(summary.contains(&binding.binding_id));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_shell_metric_density_shared_consumers_export()
        .expect("checked M5 shell-geometry shared-consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SHELL_METRIC_DENSITY_SHARED_CONSUMERS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let compact = seeded_m5_shell_metric_density_shared_consumers_compact_remote_narrowed();
    assert!(
        compact.validate().is_empty(),
        "{:?}",
        violations_of(&compact)
    );
    assert_eq!(compact.consumer_bindings.len(), 15);

    let exported = seeded_m5_shell_metric_density_shared_consumers_exported_redaction_narrowed();
    assert!(
        exported.validate().is_empty(),
        "{:?}",
        violations_of(&exported)
    );
    assert_eq!(exported.consumer_bindings.len(), 15);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let compact: M5ShellGeometrySharedConsumersPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-shell-metric-density-shared-consumers/compact_remote_narrowed.json"
    )))
        .expect("compact fixture parses");
    assert!(compact.validate().is_empty());
    assert_eq!(
        compact,
        seeded_m5_shell_metric_density_shared_consumers_compact_remote_narrowed()
    );

    let exported: M5ShellGeometrySharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-shell-metric-density-shared-consumers/exported_redaction_narrowed.json"
    )))
    .expect("exported fixture parses");
    assert!(exported.validate().is_empty());
    assert_eq!(
        exported,
        seeded_m5_shell_metric_density_shared_consumers_exported_redaction_narrowed()
    );
}
