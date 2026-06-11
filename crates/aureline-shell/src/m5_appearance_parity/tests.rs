//! Inline unit tests for the M5 appearance-and-density qualification audit.

use std::collections::BTreeSet;

use super::*;

#[test]
fn seeded_audit_passes_validation() {
    let report = seeded_m5_appearance_qualification_audit();
    validate_m5_appearance_qualification(&report).expect("seeded audit must validate");
}

#[test]
fn seeded_audit_qualifies_every_required_row() {
    let report = seeded_m5_appearance_qualification_audit();
    assert!(report.every_required_row_qualified());
}

#[test]
fn seeded_audit_has_no_blocking_findings() {
    let report = seeded_m5_appearance_qualification_audit();
    assert!(report.report_clean);
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    assert!(report.narrowable_marketed_rows.is_empty());
    for surface in &report.rows {
        assert!(
            surface.blocking_findings.is_empty(),
            "surface {} carried blocking findings: {:?}",
            surface.descriptor.surface_id,
            surface.blocking_findings
        );
    }
}

#[test]
fn seeded_audit_covers_every_surface_family() {
    let report = seeded_m5_appearance_qualification_audit();
    let families: BTreeSet<M5AppearanceSurfaceFamily> = report
        .rows
        .iter()
        .map(|surface| surface.descriptor.surface_family)
        .collect();
    for expected in [
        M5AppearanceSurfaceFamily::NotebookCellChrome,
        M5AppearanceSurfaceFamily::ResultGridRow,
        M5AppearanceSurfaceFamily::ProfilerPanel,
        M5AppearanceSurfaceFamily::TracePanel,
        M5AppearanceSurfaceFamily::PipelineCard,
        M5AppearanceSurfaceFamily::PreviewRouteBadge,
        M5AppearanceSurfaceFamily::DocsBrowserPane,
        M5AppearanceSurfaceFamily::CompanionSurface,
        M5AppearanceSurfaceFamily::SyncStatusSurface,
        M5AppearanceSurfaceFamily::OffboardingSurface,
    ] {
        assert!(
            families.contains(&expected),
            "surface family {} is not registered",
            expected.as_str()
        );
    }
}

#[test]
fn seeded_audit_qualifies_all_three_themes_and_densities() {
    let report = seeded_m5_appearance_qualification_audit();
    for row in [
        M5AppearanceRow::ThemeDark,
        M5AppearanceRow::ThemeLight,
        M5AppearanceRow::ThemeHighContrast,
        M5AppearanceRow::DensityCompact,
        M5AppearanceRow::DensityStandard,
        M5AppearanceRow::DensityComfortable,
        M5AppearanceRow::ReducedMotion,
        M5AppearanceRow::LiveAppearanceChange,
    ] {
        let coverage = report
            .row_coverage
            .iter()
            .find(|coverage| coverage.row == row)
            .expect("every required row has a coverage summary");
        assert!(
            coverage.qualified_rows > 0,
            "row {} must be qualified by at least one surface",
            row.as_str()
        );
    }
}

#[test]
fn high_salience_surfaces_project_a_boundary_cue_on_qualified_rows() {
    let report = seeded_m5_appearance_qualification_audit();
    assert!(report.high_salience_surface_count > 0);
    for surface in &report.rows {
        if !surface.high_salience {
            continue;
        }
        for binding in &surface.bindings {
            if binding.qualification_status == M5QualificationStatus::Qualified {
                assert!(
                    binding.projected_boundary_cue.is_some(),
                    "high-salience surface {} must project a boundary cue on {}",
                    surface.descriptor.surface_id,
                    binding.row.as_str()
                );
            }
        }
    }
}

#[test]
fn appearance_anchor_index_covers_every_surface() {
    let report = seeded_m5_appearance_qualification_audit();
    assert_eq!(report.appearance_anchor_index.len(), report.rows.len());
    for surface in &report.rows {
        let entry = report
            .appearance_anchor_index
            .iter()
            .find(|entry| entry.surface_id == surface.descriptor.surface_id)
            .expect("every surface must have an appearance-anchor entry");
        assert_eq!(
            entry.appearance_anchor_ref,
            surface.descriptor.appearance_anchor_ref
        );
        assert!(!entry.appearance_anchor_ref.is_empty());
    }
}

/// A minimal high-salience descriptor used to exercise binding findings.
fn high_salience_descriptor() -> M5AppearanceDescriptor {
    M5AppearanceDescriptor {
        surface_id: "surface:sync.status_surface".to_owned(),
        surface_family: M5AppearanceSurfaceFamily::SyncStatusSurface,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        appearance_anchor_ref: "appearance:anchor:sync:status_surface".to_owned(),
        accessibility_note: "note".to_owned(),
        semantic_salience: M5SemanticSalience::SeverityBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        marketed_on_desktop_rows: true,
        registered_on_appearance_session: true,
    }
}

/// A fully-projected qualified binding for a theme row, ready to be mutated
/// into a red result by individual tests.
fn qualified_theme_binding(row: M5AppearanceRow) -> M5AppearanceBinding {
    M5AppearanceBinding {
        row,
        dimension: row.canonical_dimension(),
        qualification_status: M5QualificationStatus::Qualified,
        marketed_on_row: true,
        projected_screenshot_pack_ref: Some("capture:surface:sync.status_surface:theme".to_owned()),
        projected_contrast_result: Some(M5ContrastResult::MeetsAa),
        projected_focus_visibility: Some(M5FocusVisibility::VisibleFocusRing),
        projected_motion_treatment: None,
        projected_state_semantics: Some(M5StateSemantics::Preserved),
        projected_keyboard_check: Some(M5CheckOutcome::Pass),
        projected_screen_reader_check: Some(M5CheckOutcome::Pass),
        projected_reopen_affordance: Some(M5ReopenAffordance::NotApplicable),
        projected_boundary_cue: Some(M5BoundaryCue::Present),
        projected_layout_integrity: None,
        evidence_freshness: Some(M5EvidenceFreshness::Fresh),
        evidence_captured_at: Some("2026-06-11T00:00:00Z".to_owned()),
        narrowing_reason: None,
        note: None,
    }
}

#[test]
fn unqualified_local_appearance_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_theme_binding(M5AppearanceRow::ThemeDark);
    binding.qualification_status = M5QualificationStatus::UnqualifiedLocalAppearance;
    let surface = build_m5_appearance_row(descriptor, vec![binding]);
    assert!(surface.blocking_findings.iter().any(|f| matches!(
        f,
        M5AppearanceBlockingFinding::UnqualifiedLocalAppearance { .. }
    )));
}

#[test]
fn missing_evidence_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_theme_binding(M5AppearanceRow::ThemeLight);
    binding.qualification_status = M5QualificationStatus::MissingEvidence;
    let surface = build_m5_appearance_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5AppearanceBlockingFinding::MissingEvidence { .. })));
}

#[test]
fn contrast_below_threshold_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_theme_binding(M5AppearanceRow::ThemeHighContrast);
    binding.projected_contrast_result = Some(M5ContrastResult::BelowThreshold);
    let surface = build_m5_appearance_row(descriptor, vec![binding]);
    assert!(surface.blocking_findings.iter().any(|f| matches!(
        f,
        M5AppearanceBlockingFinding::ContrastBelowThreshold { .. }
    )));
}

#[test]
fn focus_not_visible_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_theme_binding(M5AppearanceRow::ThemeDark);
    binding.projected_focus_visibility = Some(M5FocusVisibility::NotVisible);
    let surface = build_m5_appearance_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5AppearanceBlockingFinding::FocusNotVisible { .. })));
}

#[test]
fn lost_state_semantics_and_failed_checks_block() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_theme_binding(M5AppearanceRow::ThemeDark);
    binding.projected_state_semantics = Some(M5StateSemantics::Lost);
    binding.projected_keyboard_check = Some(M5CheckOutcome::Fail);
    binding.projected_screen_reader_check = Some(M5CheckOutcome::Fail);
    binding.projected_reopen_affordance = Some(M5ReopenAffordance::Lost);
    binding.projected_boundary_cue = Some(M5BoundaryCue::Hidden);
    let surface = build_m5_appearance_row(descriptor, vec![binding]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("state_semantics_lost"));
    assert!(tokens.contains("keyboard_check_failed"));
    assert!(tokens.contains("screen_reader_check_failed"));
    assert!(tokens.contains("reopen_target_lost"));
    assert!(tokens.contains("boundary_cue_hidden"));
}

#[test]
fn reduced_motion_that_still_animates_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_theme_binding(M5AppearanceRow::ReducedMotion);
    binding.dimension = M5AppearanceDimension::Motion;
    binding.projected_contrast_result = None;
    binding.projected_motion_treatment = Some(M5MotionTreatment::Animated);
    let surface = build_m5_appearance_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5AppearanceBlockingFinding::MotionNotDowngraded { .. })));
}

#[test]
fn live_change_corruption_uses_live_change_classes() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_theme_binding(M5AppearanceRow::LiveAppearanceChange);
    binding.dimension = M5AppearanceDimension::LiveChange;
    binding.projected_contrast_result = None;
    binding.projected_layout_integrity = Some(M5LayoutIntegrity::Corrupted);
    binding.projected_focus_visibility = Some(M5FocusVisibility::NotVisible);
    binding.projected_state_semantics = Some(M5StateSemantics::Lost);
    let surface = build_m5_appearance_row(descriptor, vec![binding]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("live_change_layout_corruption"));
    assert!(tokens.contains("live_change_focus_loss"));
    assert!(tokens.contains("live_change_state_corruption"));
}

#[test]
fn stale_evidence_on_marketed_row_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_theme_binding(M5AppearanceRow::ThemeDark);
    binding.evidence_freshness = Some(M5EvidenceFreshness::Stale);
    let surface = build_m5_appearance_row(descriptor, vec![binding]);
    assert!(surface.blocking_findings.iter().any(|f| matches!(
        f,
        M5AppearanceBlockingFinding::StaleEvidenceOnMarketedRow { .. }
    )));
}

#[test]
fn dimension_drift_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_theme_binding(M5AppearanceRow::ThemeDark);
    binding.dimension = M5AppearanceDimension::Density;
    let surface = build_m5_appearance_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5AppearanceBlockingFinding::DimensionDrift { .. })));
}

#[test]
fn missing_projection_blocks_on_qualified_row() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_theme_binding(M5AppearanceRow::ThemeDark);
    binding.projected_screenshot_pack_ref = None;
    let surface = build_m5_appearance_row(descriptor, vec![binding]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("missing_projection"));
    assert!(tokens.contains("missing_screenshot_pack"));
}

#[test]
fn descriptor_level_findings_fire() {
    let descriptor = M5AppearanceDescriptor {
        surface_id: "surface:sync.status_surface".to_owned(),
        surface_family: M5AppearanceSurfaceFamily::SyncStatusSurface,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        appearance_anchor_ref: "  ".to_owned(),
        accessibility_note: "  ".to_owned(),
        semantic_salience: M5SemanticSalience::SeverityBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        marketed_on_desktop_rows: true,
        registered_on_appearance_session: false,
    };
    let surface = build_m5_appearance_row(descriptor, vec![]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("descriptor_missing_appearance_anchor"));
    assert!(tokens.contains("missing_accessibility_note"));
    assert!(tokens.contains("surface_not_on_appearance_session"));
}

#[test]
fn missing_narrowing_reason_blocks() {
    let mut report = seeded_m5_appearance_qualification_audit();
    let surface = report
        .rows
        .iter_mut()
        .find(|surface| surface.descriptor.surface_id == "surface:docs_browser.pane")
        .expect("docs browser surface present");
    let binding = surface
        .bindings
        .iter_mut()
        .find(|binding| binding.qualification_status == M5QualificationStatus::DeclaredCaptureGap)
        .expect("declared capture gap binding present");
    binding.narrowing_reason = None;
    let rebuilt = build_m5_appearance_row(surface.descriptor.clone(), surface.bindings.clone());
    assert!(rebuilt.blocking_findings.iter().any(|f| matches!(
        f,
        M5AppearanceBlockingFinding::MissingNarrowingReason { .. }
    )));
}

#[test]
fn missing_required_row_blocks_validation() {
    let mut report = seeded_m5_appearance_qualification_audit();
    report.rows[0]
        .bindings
        .retain(|binding| binding.row != M5AppearanceRow::ThemeDark);
    let result = validate_m5_appearance_qualification(&report);
    assert!(result.is_err());
}

#[test]
fn narrowable_rows_surface_a_blocking_marketed_row() {
    let report = seeded_m5_appearance_qualification_audit();
    let mut surfaces = report.rows.clone();
    let surface = surfaces
        .iter_mut()
        .find(|surface| surface.descriptor.surface_id == "surface:sync.status_surface")
        .expect("sync surface present");
    let mut bindings = surface.bindings.clone();
    let binding = bindings
        .iter_mut()
        .find(|binding| binding.row == M5AppearanceRow::ThemeHighContrast)
        .expect("high-contrast binding present");
    binding.projected_contrast_result = Some(M5ContrastResult::BelowThreshold);
    *surface = build_m5_appearance_row(surface.descriptor.clone(), bindings);
    let rebuilt = build_m5_appearance_qualification_audit(surfaces);
    assert!(!rebuilt.report_clean);
    assert!(rebuilt
        .narrowable_marketed_rows
        .iter()
        .any(
            |narrowable| narrowable.surface_id == "surface:sync.status_surface"
                && narrowable.row == M5AppearanceRow::ThemeHighContrast
        ));
}

#[test]
fn support_export_quotes_every_surface_id() {
    let report = seeded_m5_appearance_qualification_audit();
    let export =
        M5AppearanceSupportExport::from_report(M5_APPEARANCE_SUPPORT_EXPORT_ID, report.clone());
    assert!(export.case_ids.contains(&report.report_id));
    for surface in &report.rows {
        assert!(export.case_ids.contains(&surface.descriptor.surface_id));
        assert!(export
            .case_ids
            .contains(&surface.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn render_markdown_is_deterministic() {
    let report = seeded_m5_appearance_qualification_audit();
    assert_eq!(report.render_markdown(), report.render_markdown());
}
