use super::*;

fn seed() -> M5WindowRestoreSharedConsumersPacket {
    seeded_m5_window_restore_shared_consumers()
}

fn violations_of(packet: &M5WindowRestoreSharedConsumersPacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(
        packet.packet_id,
        M5_WINDOW_RESTORE_SHARED_CONSUMERS_PACKET_ID
    );
    assert_eq!(
        packet.record_kind,
        M5_WINDOW_RESTORE_SHARED_CONSUMERS_RECORD_KIND
    );
    assert_eq!(packet.consumer_bindings.len(), 15);
}

#[test]
fn every_family_is_adopted_by_two_or_more_consumers() {
    let packet = seed();
    let mut family_consumers: BTreeMap<
        M5WindowRestoreFamily,
        BTreeSet<M5WindowRestoreConsumerSurface>,
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
    for consumer in M5WindowRestoreConsumerSurface::ALL {
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
    for representation in WindowRestoreRepresentation::ALL {
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
    let mut profile_facets: BTreeMap<&str, &WindowRestoreStateFacetValues> = BTreeMap::new();
    for binding in &packet.consumer_bindings {
        match profile_facets.get(binding.restore_profile_id.as_str()) {
            None => {
                profile_facets.insert(binding.restore_profile_id.as_str(), &binding.state_facets);
            }
            Some(existing) => assert_eq!(
                **existing, binding.state_facets,
                "grammar drift on {}",
                binding.restore_profile_id
            ),
        }
    }
    // The five profiles each fan out to more than one consumer.
    assert_eq!(profile_facets.len(), 5);
}

#[test]
fn every_window_restore_role_word_is_a_frozen_role_token() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        assert!(
            binding
                .state_facets
                .window_restore_role_word_in_vocabulary(),
            "role word `{}` on {} is not a frozen role token",
            binding.state_facets.window_restore_role_word,
            binding.binding_id
        );
        assert!(binding.state_facets.all_present());
        assert!(binding.state_facets.session_continuity_satisfied());
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
                WindowRestoreParityState::FacetsDisclosedNarrowed
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
                WindowRestoreParityState::FacetsPreserved
            );
            assert!(binding.narrow_note.is_none());
        }
        if matches!(
            binding.representation,
            WindowRestoreRepresentation::RemoteProjected
        ) {
            assert!(!binding.remote_source_note.trim().is_empty());
        }
        if matches!(
            binding.representation,
            WindowRestoreRepresentation::ExportedRedacted
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
        !resolve_window_restore_render_disclosure(WindowRestoreRepresentation::DesktopFull)
            .needs_narrow_note
    );
    let compact =
        resolve_window_restore_render_disclosure(WindowRestoreRepresentation::CompactNarrowed);
    assert_eq!(
        compact.narrow_reason,
        Some(WindowRestoreNarrowReason::CompactionNarrowed)
    );
    assert!(compact.needs_narrow_note);
    assert!(!compact.needs_remote_source_note);
    let remote =
        resolve_window_restore_render_disclosure(WindowRestoreRepresentation::RemoteProjected);
    assert!(remote.needs_remote_source_note);
    let exported =
        resolve_window_restore_render_disclosure(WindowRestoreRepresentation::ExportedRedacted);
    assert!(exported.needs_export_detail_note);
}

#[test]
fn grammar_drift_is_rejected() {
    let mut packet = seed();
    // Reword one surface of a multi-binding profile to a different (still-valid) role token.
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.binding_id == "wrsc-shared-authority-shell")
        .unwrap();
    packet.consumer_bindings[target]
        .state_facets
        .window_restore_role_word = "pane_role".to_owned();
    assert!(violations_of(&packet).contains(&"window_restore_grammar_drift_across_surfaces"));
}

#[test]
fn window_restore_role_word_outside_vocabulary_is_rejected() {
    let mut packet = seed();
    packet.consumer_bindings[0]
        .state_facets
        .window_restore_role_word = "totally_made_up".to_owned();
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"window_restore_role_word_outside_vocabulary"));
    // Rewording one surface also trips drift, which is expected and fine.
}

#[test]
fn dropped_session_continuity_on_authority_role_is_rejected() {
    let mut packet = seed();
    // wrsc-shared-authority-coordinator carries the workspace_authority role, which must always keep a
    // real session continuity.
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.binding_id == "wrsc-shared-authority-coordinator")
        .unwrap();
    packet.consumer_bindings[target]
        .state_facets
        .session_continuity_word = "reran_session_scoped_work".to_owned();
    assert!(violations_of(&packet).contains(&"session_continuity_missing_for_authority_role"));
}

#[test]
fn reuse_below_two_consumers_is_rejected() {
    let mut packet = seed();
    // Drop every no-rerun-session-hydration binding except one, leaving the family with one consumer.
    let mut kept_one = false;
    packet.consumer_bindings.retain(|b| {
        if b.family == M5WindowRestoreFamily::NoRerunSessionHydration {
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
        vec![M5_WINDOW_RESTORE_MATRIX_DOC_REF.to_owned()];
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
    packet.consumer_bindings[target].narrow_note = Some(WindowRestoreNarrowNote {
        reason: WindowRestoreNarrowReason::CompactionNarrowed,
        preserved_grammar_note: "x".to_owned(),
        next_action: WindowRestoreNarrowNextAction::ExpandInDesktop,
        next_action_label: "x".to_owned(),
    });
    assert!(violations_of(&packet).contains(&"unexpected_narrow_note"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [(fn(&mut WindowRestoreConsumerBinding), &str); 5] = [
        (
            |b| {
                b.reruns_commands_or_reattaches_privileged_sessions_implicitly_during_restore = true
            },
            "reruns_commands_or_reattaches_privileged_sessions_implicitly_during_restore",
        ),
        (
            |b| b.deletes_layout_structure_silently_on_missing_extension_or_remote_target = true,
            "deletes_layout_structure_silently_on_missing_extension_or_remote_target",
        ),
        (
            |b| b.leaves_windows_or_dialogs_unreachable_after_display_topology_remap = true,
            "leaves_windows_or_dialogs_unreachable_after_display_topology_remap",
        ),
        (
            |b| b.merges_workspace_authority_and_window_topology_into_one_opaque_blob = true,
            "merges_workspace_authority_and_window_topology_into_one_opaque_blob",
        ),
        (
            |b| b.overclaims_restore_fidelity_when_only_context_or_evidence_reopened = true,
            "overclaims_restore_fidelity_when_only_context_or_evidence_reopened",
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
        .retain(|b| b.family != M5WindowRestoreFamily::DisplayTopologyRecovery);
    assert!(violations_of(&packet).contains(&"family_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_WINDOW_RESTORE_SHARED_CONSUMERS_DOC_REF);
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
        .starts_with("family,consumer,representation,window_restore_role_word,parity_state"));
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
    let from_disk = current_stable_m5_window_restore_shared_consumers_export()
        .expect("checked M5 window-restore shared-consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_WINDOW_RESTORE_SHARED_CONSUMERS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let compact = seeded_m5_window_restore_shared_consumers_compact_remote_narrowed();
    assert!(
        compact.validate().is_empty(),
        "{:?}",
        violations_of(&compact)
    );
    assert_eq!(compact.consumer_bindings.len(), 15);

    let exported = seeded_m5_window_restore_shared_consumers_exported_redaction_narrowed();
    assert!(
        exported.validate().is_empty(),
        "{:?}",
        violations_of(&exported)
    );
    assert_eq!(exported.consumer_bindings.len(), 15);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let compact: M5WindowRestoreSharedConsumersPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-window-restore-shared-consumers/compact_remote_narrowed.json"
        )))
        .expect("compact fixture parses");
    assert!(compact.validate().is_empty());
    assert_eq!(
        compact,
        seeded_m5_window_restore_shared_consumers_compact_remote_narrowed()
    );

    let exported: M5WindowRestoreSharedConsumersPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-window-restore-shared-consumers/exported_redaction_narrowed.json"
    )))
        .expect("exported fixture parses");
    assert!(exported.validate().is_empty());
    assert_eq!(
        exported,
        seeded_m5_window_restore_shared_consumers_exported_redaction_narrowed()
    );
}
