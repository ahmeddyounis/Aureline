//! Tests for the M05-809 visual-designer component consumer packet.

use super::*;

fn packet() -> VisualDesignerConsumerPacket {
    seeded_m5_visual_designer_component_consumers_packet()
}

#[test]
fn seeded_packet_validates_clean() {
    let violations = packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn packet_identity_is_stamped() {
    let p = packet();
    assert_eq!(p.record_kind, VISUAL_DESIGNER_CONSUMER_RECORD_KIND);
    assert_eq!(p.schema_version, VISUAL_DESIGNER_CONSUMER_SCHEMA_VERSION);
    assert_eq!(p.matrix_ref, VISUAL_DESIGNER_CONSUMER_MATRIX_REF);
}

#[test]
fn every_consumer_group_is_adopted() {
    let p = packet();
    for group in ConsumerGroup::ALL {
        assert!(
            p.rows.iter().any(|r| r.consumer_group == group),
            "consumer group {group:?} is not adopted"
        );
    }
    assert!(p.summary.framework_pack_consumer_present);
    assert!(p.summary.preview_runtime_consumer_present);
    assert!(p.summary.browser_runtime_demo_consumer_present);
    assert!(p.summary.docs_onboarding_consumer_present);
}

#[test]
fn every_frozen_family_is_adopted() {
    let families = packet().represented_families();
    for family in M5VisualDesignerComponentFamily::ALL {
        assert!(families.contains(&family), "family {family:?} not adopted");
    }
    assert_eq!(families.len(), M5VisualDesignerComponentFamily::ALL.len());
}

#[test]
fn multiple_surfaces_point_to_one_canonical_family() {
    // AC1: at least one family is reused across two or more consumer groups.
    let p = packet();
    assert!(p.families_reused_across_groups() >= 1);
    assert_eq!(
        p.summary.families_reused_across_groups,
        p.families_reused_across_groups()
    );
}

#[test]
fn ac1_every_row_points_to_canonical_family() {
    for row in &packet().rows {
        assert!(
            row.points_to_canonical_family(),
            "{} does not point to a canonical family",
            row.row_id
        );
        assert_eq!(
            row.canonical_family_schema_ref,
            canonical_schema_ref_for(row.component_family),
            "{}",
            row.row_id
        );
        assert!(row
            .canonical_packet_refs
            .contains(&canonical_packet_ref_for(row.component_family).to_owned()));
    }
}

#[test]
fn ac2_every_row_preserves_label_families() {
    for row in &packet().rows {
        assert!(row.preserves_labels(), "{} broke label parity", row.row_id);
    }
    // The union covers every controlled label family.
    let covered = packet().covered_label_families();
    for family in REQUIRED_LABEL_FAMILIES {
        assert!(
            covered.contains(family),
            "label family {family} not covered"
        );
    }
    assert!(packet().summary.label_family_coverage_complete);
}

#[test]
fn ac3_docs_onboarding_references_canonical_not_local_prose() {
    let p = packet();
    assert!(p.rows.iter().any(|r| {
        r.consumer_group == ConsumerGroup::DocsOnboarding && r.references_canonical_not_local_prose
    }));
    for row in &p.rows {
        if row.consumer_group == ConsumerGroup::DocsOnboarding {
            assert!(
                row.references_canonical_not_local_prose,
                "{} clones local prose",
                row.row_id
            );
        }
    }
}

#[test]
fn every_row_keeps_design_system_fidelity_even_when_narrowed() {
    for row in &packet().rows {
        assert!(
            row.keeps_design_system_fidelity(),
            "{} drifts from the design-system contract",
            row.row_id
        );
    }
    assert!(packet().summary.all_rows_keep_design_system_fidelity);
}

#[test]
fn narrowed_rows_disclose_with_a_banner() {
    for row in &packet().rows {
        assert!(row.discloses_narrowing(), "{} narrows silently", row.row_id);
        if row.is_narrowed() {
            let banner = row
                .reduced_capability_banner
                .as_ref()
                .unwrap_or_else(|| panic!("{} is narrowed but has no banner", row.row_id));
            assert_eq!(
                banner.capability_state,
                row.authority_mode.capability_state()
            );
            assert!(!banner.missing_capabilities.is_empty());
        } else {
            assert!(row.reduced_capability_banner.is_none());
        }
    }
}

#[test]
fn surfaces_belong_to_their_declared_group() {
    for row in &packet().rows {
        assert!(row.surface_group_consistent(), "{}", row.row_id);
    }
}

#[test]
fn copy_export_parity_holds_for_every_row() {
    for row in &packet().rows {
        assert!(row.copy_export.is_complete(), "{}", row.row_id);
    }
    assert!(packet().summary.all_rows_have_copy_export);
}

#[test]
fn summary_matches_computed() {
    let p = packet();
    assert_eq!(p.summary, p.computed_summary());
}

// --- Negative / drift guards ------------------------------------------------

#[test]
fn wrong_canonical_schema_is_flagged() {
    let mut p = packet();
    p.rows[0].canonical_family_schema_ref = "schemas/ui/made-up.schema.json".to_owned();
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerConsumerViolation::NotCanonicalFamily { .. }
    )));
}

#[test]
fn cloning_local_prose_is_flagged() {
    let mut p = packet();
    p.rows[0].references_canonical_not_local_prose = false;
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerConsumerViolation::NotCanonicalFamily { .. }
    )));
}

#[test]
fn renaming_a_label_family_is_flagged() {
    let mut p = packet();
    p.rows[0].label_parity = LabelParityState::RenamedOrDropped;
    p.summary = p.computed_summary();
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, VisualDesignerConsumerViolation::LabelParityBroken { .. })));
}

#[test]
fn unknown_label_family_is_flagged() {
    let mut p = packet();
    p.rows[0].preserved_label_families = vec!["made_up_family".to_owned()];
    p.summary = p.computed_summary();
    let v = p.validate();
    assert!(v
        .iter()
        .any(|v| matches!(v, VisualDesignerConsumerViolation::LabelParityBroken { .. })));
}

#[test]
fn design_system_drift_is_flagged() {
    let mut p = packet();
    p.rows[0].design_system.motion_consistent = false;
    p.summary = p.computed_summary();
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, VisualDesignerConsumerViolation::DesignSystemDrift { .. })));
}

#[test]
fn narrowed_row_without_banner_is_flagged() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.is_narrowed())
        .expect("a narrowed row");
    row.reduced_capability_banner = None;
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerConsumerViolation::NarrowedWithoutDisclosure { .. }
    )));
}

#[test]
fn banner_state_mismatch_is_flagged() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.is_narrowed())
        .expect("a narrowed row");
    if let Some(banner) = row.reduced_capability_banner.as_mut() {
        banner.capability_state = "full".to_owned();
    }
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerConsumerViolation::NarrowedWithoutDisclosure { .. }
    )));
}

#[test]
fn full_interactive_row_with_spurious_banner_is_flagged() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| !r.is_narrowed())
        .expect("a full-interactive row");
    row.reduced_capability_banner = Some(ReducedCapabilityBanner {
        banner_id: "spurious".to_owned(),
        visible_label: "should not be here".to_owned(),
        capability_state: "read_only".to_owned(),
        missing_capabilities: vec!["x".to_owned()],
    });
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerConsumerViolation::NarrowedWithoutDisclosure { .. }
    )));
}

#[test]
fn handoff_without_note_is_flagged() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.handoff_target.requires_note())
        .expect("a row with a handoff");
    row.handoff_note_ref = String::new();
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerConsumerViolation::NarrowedWithoutDisclosure { .. }
    )));
}

#[test]
fn surface_group_mismatch_is_flagged() {
    let mut p = packet();
    p.rows[0].consumer_group = ConsumerGroup::DocsOnboarding;
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerConsumerViolation::SurfaceGroupMismatch { .. }
    )));
}

#[test]
fn missing_consumer_group_is_flagged() {
    let mut p = packet();
    p.rows
        .retain(|r| r.consumer_group != ConsumerGroup::FrameworkPack);
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerConsumerViolation::MissingConsumerGroup { .. }
    )));
}

#[test]
fn missing_family_coverage_is_flagged() {
    let mut p = packet();
    p.rows
        .retain(|r| r.component_family != M5VisualDesignerComponentFamily::PropertyInspectorRow);
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerConsumerViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn no_family_reused_across_groups_is_flagged() {
    // Keep only one row per family so nothing is reused across groups.
    let mut p = packet();
    let mut seen = BTreeSet::new();
    p.rows.retain(|r| seen.insert(r.component_family));
    p.summary = p.computed_summary();
    assert_eq!(p.families_reused_across_groups(), 0);
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerConsumerViolation::NoFamilyReusedAcrossGroups
    )));
}

#[test]
fn missing_docs_onboarding_reference_is_flagged() {
    let mut p = packet();
    p.rows
        .retain(|r| r.consumer_group != ConsumerGroup::DocsOnboarding);
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerConsumerViolation::MissingDocsOnboardingReference
    )));
}

#[test]
fn duplicate_row_ids_are_flagged() {
    let mut p = packet();
    let dup = p.rows[0].clone();
    p.rows.push(dup);
    p.summary = p.computed_summary();
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, VisualDesignerConsumerViolation::DuplicateId { .. })));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut p = packet();
    p.summary.row_count += 1;
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, VisualDesignerConsumerViolation::SummaryMismatch)));
}

#[test]
fn forbidden_boundary_material_is_flagged() {
    let mut p = packet();
    p.rows[0].handoff_note_ref = "bearer abc123".to_owned();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerConsumerViolation::RawBoundaryMaterialInExport
    )));
}

#[test]
fn on_disk_export_matches_builder() {
    let disk = current_m5_visual_designer_component_consumers_export()
        .expect("checked-in export must parse and validate");
    assert_eq!(disk, packet(), "on-disk export drifted from the builder");
}

#[test]
fn csv_has_a_row_per_consumer() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    assert_eq!(lines, packet().rows.len() + 1);
    assert!(csv.contains("framework_pack"));
    assert!(csv.contains("docs_onboarding"));
}

#[test]
fn markdown_summary_names_every_row() {
    let md = packet().render_markdown_summary();
    for row in &packet().rows {
        assert!(md.contains(&row.row_id), "missing {}", row.row_id);
    }
}

#[test]
fn export_is_deterministic() {
    assert_eq!(packet().export_safe_json(), packet().export_safe_json());
}
