//! Tests for the M05-808 visual-designer component accessibility fallback packet.

use super::*;

fn packet() -> ComponentAccessibilityPacket {
    seeded_m5_visual_designer_a11y_fallback_packet()
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
    assert_eq!(p.record_kind, VISUAL_DESIGNER_A11Y_FALLBACK_RECORD_KIND);
    assert_eq!(
        p.schema_version,
        VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_VERSION
    );
    assert_eq!(
        p.matrix_ref,
        VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let families = packet().represented_families();
    for family in M5VisualDesignerComponentFamily::ALL {
        assert!(
            families.contains(&family),
            "family {family:?} is not certified"
        );
    }
    assert_eq!(families.len(), M5VisualDesignerComponentFamily::ALL.len());
}

#[test]
fn summary_matches_computed() {
    let p = packet();
    assert_eq!(p.summary, p.computed_summary());
}

#[test]
fn seeded_status_split_is_five_green_two_yellow_zero_red() {
    let p = packet();
    assert_eq!(p.summary.green_count, 5, "green");
    assert_eq!(p.summary.yellow_count, 2, "yellow");
    assert_eq!(p.summary.red_count, 0, "red");
}

#[test]
fn canvas_heavy_families_offer_a_non_visual_fallback() {
    for row in &packet().rows {
        if row.is_canvas_heavy() {
            assert!(
                row.fallback_modalities
                    .contains(&M5FallbackModality::Canvas),
                "{} must render a canvas",
                row.row_id
            );
            assert!(
                row.has_non_visual_fallback(),
                "{} must offer a non-visual fallback",
                row.row_id
            );
        }
    }
}

#[test]
fn ac1_no_row_is_drag_only() {
    for row in &packet().rows {
        assert!(row.no_drag_only_editing(), "{} is drag-only", row.row_id);
        if row.has_drag_affordance() {
            assert!(
                !row.command_backed_actions.is_empty(),
                "{} exposes a drag affordance but no command-backed action",
                row.row_id
            );
        }
    }
}

#[test]
fn ac2_every_row_reaches_source_backed_truth_via_at() {
    for row in &packet().rows {
        assert!(
            row.reaches_source_backed_truth_via_at(),
            "{} strands assistive tech",
            row.row_id
        );
        assert!(row.keyboard_reach.never_traps(), "{}", row.row_id);
        assert!(row.screen_reader_reach.never_traps(), "{}", row.row_id);
        assert!(row.low_resource_reach.never_traps(), "{}", row.row_id);
    }
}

#[test]
fn ac3_every_row_narrowing_is_disclosed() {
    for row in &packet().rows {
        assert!(
            row.narrowing_disclosed(),
            "{} narrows without honest disclosure",
            row.row_id
        );
    }
}

#[test]
fn reduced_rows_carry_an_honest_auto_narrow() {
    for row in &packet().rows {
        if row.is_reduced() {
            let narrow = row
                .auto_narrow
                .as_ref()
                .unwrap_or_else(|| panic!("{} is reduced but has no auto-narrow", row.row_id));
            assert!(narrow.is_honest(), "{} auto-narrow dishonest", row.row_id);
        } else {
            assert!(
                row.auto_narrow.is_none(),
                "{} carries a spurious auto-narrow",
                row.row_id
            );
        }
    }
}

#[test]
fn export_preserves_meaning_without_screenshot() {
    for row in &packet().rows {
        assert!(
            row.export_preserves_meaning(),
            "{} export needs a screenshot",
            row.row_id
        );
        assert!(row.copy_export.is_complete(), "{}", row.row_id);
    }
}

#[test]
fn drag_only_trap_makes_a_row_stranded() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.component_family == M5VisualDesignerComponentFamily::DesignCanvas)
        .expect("design canvas row");
    row.drag_editing = DragEditingState::DragOnlyTrap;
    assert_eq!(row.status(), ComponentAccessibilityStatus::Stranded);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ComponentAccessibilityViolation::DragOnlyEditing { .. })));
    assert!(violations
        .iter()
        .any(|v| matches!(v, ComponentAccessibilityViolation::StrandedRow { .. })));
}

#[test]
fn drag_affordant_family_without_command_backed_action_is_stranded() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.component_family == M5VisualDesignerComponentFamily::PropertyInspectorRow)
        .expect("property inspector row");
    row.command_backed_actions.clear();
    assert!(!row.no_drag_only_editing());
    p.summary = p.computed_summary();
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, ComponentAccessibilityViolation::DragOnlyEditing { .. })));
}

#[test]
fn view_only_trap_strands_assistive_tech() {
    let mut p = packet();
    let row = &mut p.rows[1];
    row.screen_reader_reach = NonVisualReachState::ViewOnlyTrap;
    assert!(!row.reaches_source_backed_truth_via_at());
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        ComponentAccessibilityViolation::AssistiveTechStranded { .. }
    )));
}

#[test]
fn canvas_heavy_without_non_visual_fallback_strands() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.component_family == M5VisualDesignerComponentFamily::BreakpointPreviewRow)
        .expect("breakpoint row");
    row.fallback_modalities = vec![M5FallbackModality::Canvas];
    assert!(!row.reaches_source_backed_truth_via_at());
}

#[test]
fn silent_narrowing_is_rejected() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.component_family == M5VisualDesignerComponentFamily::DesignCanvas)
        .expect("design canvas row");
    row.narrowing_disclosures[0].state = NarrowingDisclosureState::SilentlyDropped;
    assert!(!row.narrowing_disclosed());
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        ComponentAccessibilityViolation::NarrowingDropsContextSilently { .. }
    )));
}

#[test]
fn reduced_row_missing_auto_narrow_is_rejected() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.auto_narrow.is_some())
        .expect("a reduced row");
    row.auto_narrow = None;
    assert!(!row.narrowing_disclosed());
}

#[test]
fn dishonest_auto_narrow_label_is_rejected() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.auto_narrow.is_some())
        .expect("a reduced row");
    row.auto_narrow.as_mut().unwrap().narrowed_label = "degraded".to_owned();
    assert!(!row.narrowing_disclosed());
}

#[test]
fn auto_narrow_dropping_source_context_is_rejected() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.auto_narrow.is_some())
        .expect("a reduced row");
    row.auto_narrow
        .as_mut()
        .unwrap()
        .preserves_source_backed_context = false;
    assert!(!row.narrowing_disclosed());
}

#[test]
fn missing_family_coverage_is_flagged() {
    let mut p = packet();
    p.rows
        .retain(|r| r.component_family != M5VisualDesignerComponentFamily::SourceSyncChip);
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        ComponentAccessibilityViolation::MissingFamilyCoverage { .. }
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
        .any(|v| matches!(v, ComponentAccessibilityViolation::DuplicateId { .. })));
}

#[test]
fn single_consumer_surface_is_flagged() {
    let mut p = packet();
    p.rows[0].consumer_surfaces.truncate(1);
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        ComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut p = packet();
    p.summary.green_count += 1;
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, ComponentAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_boundary_material_is_flagged() {
    let mut p = packet();
    p.rows[0].export_summary_ref = "bearer abc123".to_owned();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        ComponentAccessibilityViolation::RawBoundaryMaterialInExport
    )));
}

#[test]
fn on_disk_export_matches_builder() {
    let disk = current_m5_visual_designer_a11y_fallback_export()
        .expect("checked-in export must parse and validate");
    assert_eq!(disk, packet(), "on-disk export drifted from the builder");
}

#[test]
fn csv_has_a_row_per_component() {
    let csv = packet().render_matrix_csv();
    // header + one line per row
    let lines = csv.lines().count();
    assert_eq!(lines, packet().rows.len() + 1);
    assert!(csv.contains("design_canvas"));
    assert!(csv.contains("breakpoint_preview_row"));
}

#[test]
fn markdown_summary_names_every_family() {
    let md = packet().render_markdown_summary();
    for family in M5VisualDesignerComponentFamily::ALL {
        assert!(md.contains(family.as_str()), "missing {}", family.as_str());
    }
}

#[test]
fn export_is_deterministic() {
    assert_eq!(packet().export_safe_json(), packet().export_safe_json());
}
