//! Tests for the M05-825 execution-lifecycle component consumer adoption lane.

use super::*;

fn packet() -> ExecutionConsumerPacket {
    seeded_m5_execution_lifecycle_component_consumers_packet()
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
    assert_eq!(p.record_kind, EXECUTION_CONSUMER_RECORD_KIND);
    assert_eq!(p.schema_version, EXECUTION_CONSUMER_SCHEMA_VERSION);
}

#[test]
fn all_five_consumer_groups_present() {
    let p = packet();
    assert!(p.summary.task_test_consumer_present);
    assert!(p.summary.request_database_consumer_present);
    assert!(p.summary.notebook_preview_consumer_present);
    assert!(p.summary.ai_publish_consumer_present);
    assert!(p.summary.support_export_consumer_present);
    assert_eq!(p.summary.consumer_group_count, ConsumerGroup::ALL.len());
}

#[test]
fn every_frozen_family_is_adopted() {
    let p = packet();
    let families = p.represented_families();
    for family in M5ExecutionComponentFamily::ALL {
        assert!(families.contains(&family), "missing family {family:?}");
    }
    assert_eq!(
        p.summary.component_family_count,
        M5ExecutionComponentFamily::ALL.len()
    );
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
fn run_attempt_header_is_reused_across_four_groups() {
    let p = packet();
    let groups: BTreeSet<ConsumerGroup> = p
        .rows
        .iter()
        .filter(|r| r.component_family == M5ExecutionComponentFamily::RunAttemptHeader)
        .map(|r| r.consumer_group)
        .collect();
    assert!(
        groups.len() >= 4,
        "run/attempt header should be adopted by >= 4 groups, saw {groups:?}"
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
fn canonical_refs_match_sibling_primitive_constants() {
    // The input-request prompt and artifact-publish row share the M05-822
    // execution-interaction primitive.
    assert_eq!(
        M5ExecutionComponentFamily::InputRequestPrompt.canonical_schema_ref(),
        M5ExecutionComponentFamily::ArtifactPublishRow.canonical_schema_ref()
    );
    assert_eq!(
        M5ExecutionComponentFamily::InputRequestPrompt.canonical_packet_ref(),
        M5ExecutionComponentFamily::ArtifactPublishRow.canonical_packet_ref()
    );
    assert_eq!(
        M5ExecutionComponentFamily::RunAttemptHeader.canonical_schema_ref(),
        M5_RUN_ATTEMPT_HEADER_SCHEMA_REF
    );
    assert_eq!(
        M5ExecutionComponentFamily::DebugHierarchy.canonical_packet_ref(),
        M5_DEBUG_HIERARCHY_ARTIFACT_REF
    );
    assert_eq!(
        M5ExecutionComponentFamily::RerunReview.canonical_schema_ref(),
        M5_RERUN_REVIEW_SCHEMA_REF
    );
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
        .retain(|r| r.consumer_group != ConsumerGroup::SupportExport);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ExecutionConsumerViolation::MissingConsumerGroup { .. })));
}

#[test]
fn dropped_family_is_rejected() {
    let mut p = packet();
    p.rows
        .retain(|r| r.component_family != M5ExecutionComponentFamily::DebugHierarchy);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ExecutionConsumerViolation::MissingFamilyCoverage { .. })));
}

#[test]
fn renamed_label_parity_is_rejected() {
    let mut p = packet();
    p.rows[0].label_parity = LabelParityState::RenamedOrDropped;
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ExecutionConsumerViolation::LabelParityBroken { .. })));
}

#[test]
fn non_canonical_schema_ref_is_rejected() {
    let mut p = packet();
    p.rows[0].canonical_family_schema_ref = "schemas/ui/made-up.schema.json".to_owned();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ExecutionConsumerViolation::NotCanonicalFamily { .. })));
}

#[test]
fn narrowed_without_banner_is_rejected() {
    let mut p = packet();
    // Find a narrowed row and strip its banner.
    let idx = p
        .rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("a narrowed row exists");
    p.rows[idx].reduced_capability_banner = None;
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ExecutionConsumerViolation::NarrowedWithoutDisclosure { .. }
    )));
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
    assert!(violations.iter().any(|v| matches!(
        v,
        ExecutionConsumerViolation::NarrowedWithoutDisclosure { .. }
    )));
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
        banner.visible_label = "read only".to_owned();
    }
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ExecutionConsumerViolation::NarrowedWithoutDisclosure { .. }
    )));
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
    assert!(violations.iter().any(|v| matches!(
        v,
        ExecutionConsumerViolation::NarrowedWithoutDisclosure { .. }
    )));
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
        ExecutionConsumerViolation::RawBoundaryMaterialInExport { .. }
    )));
}

#[test]
fn summary_mismatch_is_rejected() {
    let mut p = packet();
    p.summary.row_count += 1;
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ExecutionConsumerViolation::SummaryMismatch)));
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
        .any(|v| matches!(v, ExecutionConsumerViolation::DuplicateId { .. })));
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
    let back: ExecutionConsumerPacket = serde_json::from_str(&json).expect("round trips");
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
    let on_disk =
        current_m5_execution_lifecycle_component_consumers_export().expect("export is valid");
    assert_eq!(
        on_disk.export_safe_json(),
        packet().export_safe_json(),
        "checked-in support export drifted from the seeded builder; regenerate the artifact"
    );
}

#[test]
fn surface_tokens_are_unique() {
    let tokens: BTreeSet<&str> = M5ExecutionConsumerSurface::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(tokens.len(), M5ExecutionConsumerSurface::ALL.len());
}

#[test]
fn every_authority_mode_maps_to_a_distinct_capability_state() {
    let states: BTreeSet<&str> = AuthorityMode::ALL
        .iter()
        .map(|a| a.capability_state())
        .collect();
    assert_eq!(states.len(), AuthorityMode::ALL.len());
}
