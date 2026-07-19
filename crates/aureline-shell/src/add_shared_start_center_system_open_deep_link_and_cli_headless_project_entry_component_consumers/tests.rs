//! Tests for the M05-842 project-entry component consumer adoption lane.

use super::*;

fn packet() -> EntryConsumerPacket {
    seeded_m5_project_entry_component_consumers_packet()
}

#[test]
fn seeded_packet_validates() {
    let violations = packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn packet_record_kind_and_schema_version_are_stamped() {
    let p = packet();
    assert_eq!(p.record_kind, ENTRY_CONSUMER_RECORD_KIND);
    assert_eq!(p.schema_version, ENTRY_CONSUMER_SCHEMA_VERSION);
}

#[test]
fn all_five_consumer_groups_present() {
    let p = packet();
    assert!(p.summary.start_center_palette_consumer_present);
    assert!(p.summary.system_open_intake_consumer_present);
    assert!(p.summary.deep_link_handoff_consumer_present);
    assert!(p.summary.cli_headless_consumer_present);
    assert!(p.summary.support_diagnostics_docs_consumer_present);
    assert_eq!(p.summary.consumer_group_count, ConsumerGroup::ALL.len());
}

#[test]
fn every_frozen_family_is_adopted() {
    let p = packet();
    let families = p.represented_families();
    for family in M5ProjectEntryComponentFamily::ALL {
        assert!(families.contains(&family), "missing family {family:?}");
    }
    assert_eq!(
        p.summary.component_family_count,
        M5ProjectEntryComponentFamily::ALL.len()
    );
}

#[test]
fn every_entry_verb_is_exercised() {
    let p = packet();
    let verbs: BTreeSet<M5EntryVerb> = p.rows.iter().map(|r| r.entry_verb).collect();
    for verb in M5EntryVerb::ALL {
        assert!(
            verbs.contains(&verb),
            "entry verb {verb:?} is not exercised"
        );
    }
    assert_eq!(p.summary.entry_verb_count, M5EntryVerb::ALL.len());
}

#[test]
fn command_ids_are_canonical_and_stable_across_surfaces() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.command_id_is_canonical(),
            "row {} forks the command id",
            row.row_id
        );
        assert_eq!(row.command_id, row.entry_verb.canonical_command_id());
    }
    assert!(p.command_ids_stable_across_surfaces());
    assert!(p.summary.command_ids_stable_across_surfaces);
    assert!(p.summary.all_rows_use_canonical_command_id);
}

#[test]
fn same_verb_shares_one_command_id_across_groups() {
    // The `open` verb appears on Start Center, palette, system-open, deep-link,
    // and docs surfaces; every one must dispatch the same command id.
    let p = packet();
    let open_ids: BTreeSet<&str> = p
        .rows
        .iter()
        .filter(|r| r.entry_verb == M5EntryVerb::Open)
        .map(|r| r.command_id.as_str())
        .collect();
    assert_eq!(open_ids.len(), 1, "open command id forked: {open_ids:?}");
}

#[test]
fn at_least_one_family_reused_across_groups() {
    let p = packet();
    assert!(
        p.families_reused_across_groups() >= 1,
        "expected a family adopted by two or more consumer groups"
    );
    assert_eq!(
        p.summary.families_reused_across_groups,
        p.families_reused_across_groups()
    );
}

#[test]
fn entry_chooser_row_is_reused_across_multiple_groups() {
    let p = packet();
    let groups: BTreeSet<ConsumerGroup> = p
        .rows
        .iter()
        .filter(|r| r.component_family == M5ProjectEntryComponentFamily::EntryChooserRow)
        .map(|r| r.consumer_group)
        .collect();
    assert!(
        groups.len() >= 3,
        "entry-chooser row should be adopted by >= 3 groups, saw {groups:?}"
    );
}

#[test]
fn all_rows_point_to_canonical_family() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.points_to_canonical_family(),
            "row {} does not point to canonical family",
            row.row_id
        );
    }
    assert!(p.summary.all_rows_point_to_canonical_family);
}

#[test]
fn canonical_schema_refs_are_per_family_and_distinct() {
    let refs: BTreeSet<&str> = M5ProjectEntryComponentFamily::ALL
        .iter()
        .map(|f| canonical_schema_ref_for(*f))
        .collect();
    assert_eq!(refs.len(), M5ProjectEntryComponentFamily::ALL.len());
    for family in M5ProjectEntryComponentFamily::ALL {
        assert_eq!(
            canonical_packet_ref_for(family),
            ENTRY_CONSUMER_CANONICAL_PACKET_REF
        );
    }
}

#[test]
fn all_rows_preserve_labels() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.preserves_labels(),
            "row {} does not preserve labels",
            row.row_id
        );
    }
    assert!(p.summary.all_rows_preserve_labels);
}

#[test]
fn label_family_coverage_is_complete() {
    let p = packet();
    let covered = p.covered_label_families();
    for family in REQUIRED_LABEL_FAMILIES {
        assert!(
            covered.contains(family),
            "label family {family} not covered"
        );
    }
    assert!(p.summary.label_family_coverage_complete);
}

#[test]
fn deep_link_and_system_open_rows_preserve_target_and_mode_truth() {
    let p = packet();
    let handoff_rows: Vec<&EntryConsumerRow> = p
        .rows
        .iter()
        .filter(|r| r.consumer_group.is_handoff_origin())
        .collect();
    assert!(!handoff_rows.is_empty());
    for row in handoff_rows {
        assert!(
            row.preserves_handoff_target_truth(),
            "handoff row {} drops target/mode truth",
            row.row_id
        );
        assert!(row
            .preserved_label_families
            .iter()
            .any(|f| f == "literal_target"));
        assert!(row
            .preserved_label_families
            .iter()
            .any(|f| f == "resulting_mode"));
    }
    assert!(p.summary.all_handoff_rows_preserve_target_truth);
}

#[test]
fn every_row_supports_entry_path_reconstruction() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.supports_entry_path_reconstruction(),
            "row {} cannot be reconstructed",
            row.row_id
        );
        assert!(row.entry_object_ref.starts_with("entry-object:"));
    }
    assert!(p.summary.all_rows_reconstructable);
}

#[test]
fn narrowed_rows_disclose_with_banner_and_note() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.discloses_narrowing(),
            "row {} does not disclose narrowing",
            row.row_id
        );
        if row.is_narrowed() {
            assert!(
                row.reduced_capability_banner.is_some(),
                "narrowed row {} lacks a banner",
                row.row_id
            );
            assert_eq!(row.label_parity, LabelParityState::DisclosedNarrowed);
        }
        if row.handoff_target.requires_note() {
            assert!(
                !row.handoff_note_ref.trim().is_empty(),
                "handoff row {} lacks a note",
                row.row_id
            );
        }
    }
    assert!(p.summary.all_narrowed_rows_disclose);
}

#[test]
fn full_interactive_rows_carry_no_banner() {
    for row in &packet().rows {
        if !row.is_narrowed() {
            assert!(
                row.reduced_capability_banner.is_none(),
                "full-interactive row {} carries a spurious banner",
                row.row_id
            );
            assert_eq!(row.label_parity, LabelParityState::Preserved);
        }
    }
}

#[test]
fn docs_help_reference_present() {
    let p = packet();
    assert!(p.has_docs_help_reference());
    assert!(p.summary.docs_help_reference_present);
}

#[test]
fn all_rows_have_copy_export_parity() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.copy_export.is_complete(),
            "row {} lacks copy/export parity",
            row.row_id
        );
    }
    assert!(p.summary.all_rows_have_copy_export);
}

#[test]
fn surface_group_is_consistent_for_every_row() {
    for row in &packet().rows {
        assert!(
            row.surface_group_consistent(),
            "row {} surface/group mismatch",
            row.row_id
        );
    }
}

#[test]
fn row_ids_are_unique() {
    let p = packet();
    let unique: BTreeSet<&str> = p.rows.iter().map(|r| r.row_id.as_str()).collect();
    assert_eq!(unique.len(), p.rows.len());
}

#[test]
fn degraded_state_vocab_is_canonical_on_every_row() {
    for row in &packet().rows {
        for token in &row.degraded_state_vocab {
            assert!(
                CANONICAL_DEGRADED_STATE_VOCAB.contains(&token.as_str()),
                "row {} carries non-canonical degraded token {token}",
                row.row_id
            );
        }
    }
}

#[test]
fn banner_capability_state_matches_authority() {
    for row in &packet().rows {
        if let Some(banner) = &row.reduced_capability_banner {
            assert_eq!(
                banner.capability_state,
                row.authority_mode.capability_state(),
                "row {} banner state mismatch",
                row.row_id
            );
        }
    }
}

#[test]
fn computed_summary_matches_stored_summary() {
    let p = packet();
    assert_eq!(p.summary, p.computed_summary());
}

#[test]
fn missing_consumer_group_is_rejected() {
    let mut p = packet();
    p.rows
        .retain(|r| r.consumer_group != ConsumerGroup::SupportDiagnosticsDocs);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryConsumerViolation::MissingConsumerGroup { .. })));
}

#[test]
fn dropped_family_is_rejected() {
    let mut p = packet();
    p.rows
        .retain(|r| r.component_family != M5ProjectEntryComponentFamily::ArchetypeReadinessRow);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryConsumerViolation::MissingFamilyCoverage { .. })));
}

#[test]
fn forked_command_id_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.entry_verb == M5EntryVerb::Open)
        .expect("an open row exists");
    p.rows[idx].command_id = "cmd:client.private.open".to_owned();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryConsumerViolation::NonCanonicalCommandId { .. })));
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryConsumerViolation::CommandIdForkedAcrossSurfaces)));
}

#[test]
fn deep_link_dropping_target_truth_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.consumer_group == ConsumerGroup::DeepLinkHandoff)
        .expect("a deep-link row exists");
    p.rows[idx]
        .preserved_label_families
        .retain(|f| f != "resulting_mode");
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryConsumerViolation::HandoffDropsTargetTruth { .. })));
}

#[test]
fn missing_entry_object_ref_breaks_reconstruction() {
    let mut p = packet();
    p.rows[0].entry_object_ref = String::new();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EntryConsumerViolation::EntryPathNotReconstructable { .. }
    )));
}

#[test]
fn renamed_label_parity_is_rejected() {
    let mut p = packet();
    p.rows[0].label_parity = LabelParityState::RenamedOrDropped;
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryConsumerViolation::LabelParityBroken { .. })));
}

#[test]
fn non_canonical_schema_ref_is_rejected() {
    let mut p = packet();
    p.rows[0].canonical_family_schema_ref = "schemas/ui/made-up.schema.json".to_owned();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryConsumerViolation::NotCanonicalFamily { .. })));
}

#[test]
fn narrowed_without_banner_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("a narrowed row exists");
    p.rows[idx].reduced_capability_banner = None;
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryConsumerViolation::NarrowedWithoutDisclosure { .. })));
}

#[test]
fn spurious_banner_on_full_row_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| !r.is_narrowed())
        .expect("a full-interactive row exists");
    p.rows[idx].reduced_capability_banner = Some(ReducedCapabilityBanner {
        banner_id: "banner:spurious".to_owned(),
        visible_label: "This should not be here".to_owned(),
        capability_state: "read_only".to_owned(),
        missing_capabilities: vec!["x".to_owned()],
    });
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryConsumerViolation::NarrowedWithoutDisclosure { .. })));
}

#[test]
fn generic_banner_label_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("a narrowed row exists");
    if let Some(banner) = p.rows[idx].reduced_capability_banner.as_mut() {
        banner.visible_label = "Get started".to_owned();
    }
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryConsumerViolation::NarrowedWithoutDisclosure { .. })));
}

#[test]
fn missing_handoff_note_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.handoff_target.requires_note())
        .expect("a handoff row exists");
    p.rows[idx].handoff_note_ref = String::new();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryConsumerViolation::NarrowedWithoutDisclosure { .. })));
}

#[test]
fn forbidden_material_in_export_is_rejected() {
    let mut p = packet();
    p.rows[0]
        .evidence_refs
        .push("bearer abc123def456".to_owned());
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EntryConsumerViolation::RawBoundaryMaterialInExport { .. }
    )));
}

#[test]
fn summary_mismatch_is_rejected() {
    let mut p = packet();
    p.summary.row_count += 1;
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryConsumerViolation::SummaryMismatch)));
}

#[test]
fn duplicate_row_id_is_rejected() {
    let mut p = packet();
    let dup = p.rows[0].clone();
    p.rows.push(dup);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryConsumerViolation::DuplicateId { .. })));
}

#[test]
fn export_json_is_deterministic() {
    let a = packet().export_safe_json();
    let b = packet().export_safe_json();
    assert_eq!(a, b);
}

#[test]
fn export_json_round_trips() {
    let p = packet();
    let json = p.export_safe_json();
    let back: EntryConsumerPacket = serde_json::from_str(&json).expect("round trips");
    assert_eq!(p, back);
    assert!(back.validate().is_empty());
}

#[test]
fn csv_has_header_and_one_line_per_row() {
    let p = packet();
    let csv = p.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), p.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,consumer_group,consumer_surface"));
    assert!(lines[0].contains("entry_verb,command_id"));
}

#[test]
fn markdown_summary_lists_every_row() {
    let p = packet();
    let md = p.render_markdown_summary();
    for row in &p.rows {
        assert!(
            md.contains(&row.row_id),
            "missing {} in markdown",
            row.row_id
        );
    }
}

#[test]
fn checked_in_export_matches_seeded_builder() {
    let on_disk = current_m5_project_entry_component_consumers_export().expect("export is valid");
    assert_eq!(
        on_disk.export_safe_json(),
        packet().export_safe_json(),
        "checked-in support export drifted from the seeded builder; regenerate the artifact"
    );
}

#[test]
fn surface_tokens_are_unique() {
    let tokens: BTreeSet<&str> = M5EntryConsumerSurface::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(tokens.len(), M5EntryConsumerSurface::ALL.len());
}

#[test]
fn every_authority_mode_maps_to_a_distinct_capability_state() {
    let states: BTreeSet<&str> = AuthorityMode::ALL
        .iter()
        .map(|a| a.capability_state())
        .collect();
    assert_eq!(states.len(), AuthorityMode::ALL.len());
}

#[test]
fn entry_verb_command_ids_are_distinct() {
    let ids: BTreeSet<&str> = M5EntryVerb::ALL
        .iter()
        .map(|v| v.canonical_command_id())
        .collect();
    assert_eq!(ids.len(), M5EntryVerb::ALL.len());
}
