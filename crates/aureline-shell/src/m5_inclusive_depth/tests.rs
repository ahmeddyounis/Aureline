//! Inline unit tests for the M5 accessibility-and-locale qualification audit.

use std::collections::BTreeSet;

use super::*;

#[test]
fn seeded_audit_passes_validation() {
    let report = seeded_m5_inclusive_depth_audit();
    validate_m5_inclusive_depth(&report).expect("seeded audit must validate");
}

#[test]
fn seeded_audit_qualifies_every_required_row() {
    let report = seeded_m5_inclusive_depth_audit();
    assert!(report.every_required_row_qualified());
}

#[test]
fn seeded_audit_has_no_blocking_findings() {
    let report = seeded_m5_inclusive_depth_audit();
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
    let report = seeded_m5_inclusive_depth_audit();
    let families: BTreeSet<M5InclusiveSurfaceFamily> = report
        .rows
        .iter()
        .map(|surface| surface.descriptor.surface_family)
        .collect();
    for expected in [
        M5InclusiveSurfaceFamily::NotebookCell,
        M5InclusiveSurfaceFamily::ResultGridRow,
        M5InclusiveSurfaceFamily::PipelineLogView,
        M5InclusiveSurfaceFamily::ProfilerTimeline,
        M5InclusiveSurfaceFamily::GuidedTour,
        M5InclusiveSurfaceFamily::DocsHelpPane,
        M5InclusiveSurfaceFamily::CompanionSurface,
        M5InclusiveSurfaceFamily::QueryConsole,
        M5InclusiveSurfaceFamily::PreviewRoutePane,
        M5InclusiveSurfaceFamily::GlossaryPanel,
        M5InclusiveSurfaceFamily::SupportPacket,
    ] {
        assert!(
            families.contains(&expected),
            "surface family {} is not registered",
            expected.as_str()
        );
    }
}

#[test]
fn seeded_audit_qualifies_every_scenario_row() {
    let report = seeded_m5_inclusive_depth_audit();
    for row in M5InclusiveRow::required_rows() {
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
fn seeded_audit_claims_every_locale() {
    let report = seeded_m5_inclusive_depth_audit();
    assert_eq!(report.claimed_locales, M5InclusiveLocale::all().to_vec());
}

#[test]
fn high_salience_surfaces_project_a_suspicious_cue_on_qualified_rows() {
    let report = seeded_m5_inclusive_depth_audit();
    assert!(report.high_salience_surface_count > 0);
    for surface in &report.rows {
        if !surface.high_salience {
            continue;
        }
        for binding in &surface.bindings {
            if binding.qualification_status == M5InclusiveQualificationStatus::Qualified {
                assert!(
                    binding.projected_suspicious_content_cue.is_some(),
                    "high-salience surface {} must project a suspicious-content cue on {}",
                    surface.descriptor.surface_id,
                    binding.row.as_str()
                );
            }
        }
    }
}

#[test]
fn locale_anchor_index_covers_every_surface() {
    let report = seeded_m5_inclusive_depth_audit();
    assert_eq!(report.locale_anchor_index.len(), report.rows.len());
    for surface in &report.rows {
        let entry = report
            .locale_anchor_index
            .iter()
            .find(|entry| entry.surface_id == surface.descriptor.surface_id)
            .expect("every surface must have a locale-anchor entry");
        assert_eq!(
            entry.locale_anchor_ref,
            surface.descriptor.locale_anchor_ref
        );
        assert!(!entry.locale_anchor_ref.is_empty());
    }
}

/// A minimal high-salience descriptor used to exercise binding findings.
fn high_salience_descriptor() -> M5InclusiveDescriptor {
    M5InclusiveDescriptor {
        surface_id: "surface:support.packet".to_owned(),
        surface_family: M5InclusiveSurfaceFamily::SupportPacket,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        locale_anchor_ref: "a11y:anchor:support:packet".to_owned(),
        inclusive_note: "note".to_owned(),
        semantic_salience: M5SemanticSalience::SeverityBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        claimed_locales: M5InclusiveLocale::all().to_vec(),
        marketed_on_inclusive_rows: true,
        registered_on_inclusive_harness: true,
    }
}

/// A fully-projected qualified binding for an interaction row, ready to be
/// mutated into a red result by individual tests.
fn qualified_interaction_binding(row: M5InclusiveRow) -> M5InclusiveBinding {
    M5InclusiveBinding {
        row,
        dimension: row.canonical_dimension(),
        qualification_status: M5InclusiveQualificationStatus::Qualified,
        marketed_on_row: true,
        projected_evidence_pack_ref: Some(
            "drill:surface:support.packet:keyboard_reachability".to_owned(),
        ),
        projected_keyboard_reachability: Some(M5KeyboardReachability::Reachable),
        projected_narration: Some(M5Narration::Narrated),
        projected_focus_visibility: Some(M5FocusVisibility::Visible),
        projected_text_correctness: Some(M5TextCorrectness::Correct),
        projected_ime_composition: None,
        projected_bidi_isolation: None,
        projected_zoom_reflow: None,
        projected_locale_parity: None,
        projected_suspicious_content_cue: Some(M5SuspiciousContentCue::Present),
        evidence_freshness: Some(M5EvidenceFreshness::Fresh),
        evidence_captured_at: Some("2026-06-11T00:00:00Z".to_owned()),
        narrowing_reason: None,
        note: None,
    }
}

/// A fully-projected qualified binding for a localization row.
fn qualified_localization_binding(row: M5InclusiveRow) -> M5InclusiveBinding {
    let mut binding = qualified_interaction_binding(row);
    binding.dimension = row.canonical_dimension();
    binding.projected_locale_parity = Some(M5LocaleParity::Parity);
    binding
}

#[test]
fn unqualified_local_a11y_path_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_interaction_binding(M5InclusiveRow::KeyboardReachability);
    binding.qualification_status = M5InclusiveQualificationStatus::UnqualifiedLocalA11yPath;
    let surface = build_m5_inclusive_row(descriptor, vec![binding]);
    assert!(surface.blocking_findings.iter().any(|f| matches!(
        f,
        M5InclusiveBlockingFinding::UnqualifiedLocalA11yPath { .. }
    )));
}

#[test]
fn missing_evidence_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_interaction_binding(M5InclusiveRow::ScreenReaderNarration);
    binding.qualification_status = M5InclusiveQualificationStatus::MissingEvidence;
    let surface = build_m5_inclusive_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5InclusiveBlockingFinding::MissingEvidence { .. })));
}

#[test]
fn red_interaction_results_block() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_interaction_binding(M5InclusiveRow::ScreenReaderNarration);
    binding.projected_keyboard_reachability = Some(M5KeyboardReachability::Trapped);
    binding.projected_narration = Some(M5Narration::Silent);
    binding.projected_focus_visibility = Some(M5FocusVisibility::Hidden);
    binding.projected_text_correctness = Some(M5TextCorrectness::Corrupted);
    binding.projected_suspicious_content_cue = Some(M5SuspiciousContentCue::Hidden);
    let surface = build_m5_inclusive_row(descriptor, vec![binding]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("keyboard_unreachable"));
    assert!(tokens.contains("narration_silent"));
    assert!(tokens.contains("focus_indicator_hidden"));
    assert!(tokens.contains("text_corrupted"));
    assert!(tokens.contains("suspicious_content_hidden"));
}

#[test]
fn misannounced_narration_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_interaction_binding(M5InclusiveRow::ScreenReaderNarration);
    binding.projected_narration = Some(M5Narration::Misannounced);
    let surface = build_m5_inclusive_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5InclusiveBlockingFinding::NarrationMisannounced { .. })));
}

#[test]
fn ime_composition_broken_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_interaction_binding(M5InclusiveRow::ImeComposition);
    binding.dimension = M5InclusiveDimension::Text;
    binding.projected_ime_composition = Some(M5ImeComposition::Broken);
    let surface = build_m5_inclusive_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5InclusiveBlockingFinding::ImeCompositionBroken { .. })));
}

#[test]
fn bidi_leaking_and_zoom_clipped_block() {
    let descriptor = high_salience_descriptor();
    let mut bidi = qualified_interaction_binding(M5InclusiveRow::BidiDirection);
    bidi.dimension = M5InclusiveDimension::Text;
    bidi.projected_bidi_isolation = Some(M5BidiIsolation::Leaking);
    let mut zoom = qualified_interaction_binding(M5InclusiveRow::HighZoom);
    zoom.projected_zoom_reflow = Some(M5ZoomReflow::Clipped);
    let surface = build_m5_inclusive_row(descriptor, vec![bidi, zoom]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("bidi_leaking"));
    assert!(tokens.contains("zoom_content_clipped"));
}

#[test]
fn locale_parity_lost_blocks() {
    let descriptor = high_salience_descriptor();
    let mut fallback = qualified_localization_binding(M5InclusiveRow::LocaleFallback);
    fallback.projected_locale_parity = Some(M5LocaleParity::SilentEnglishFallback);
    let mut help = qualified_localization_binding(M5InclusiveRow::TranslatedHelpParity);
    help.projected_locale_parity = Some(M5LocaleParity::Mismatched);
    let surface = build_m5_inclusive_row(descriptor, vec![fallback, help]);
    let count = surface
        .blocking_findings
        .iter()
        .filter(|f| matches!(f, M5InclusiveBlockingFinding::LocaleParityLost { .. }))
        .count();
    assert_eq!(count, 2);
}

#[test]
fn stale_evidence_on_marketed_row_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_interaction_binding(M5InclusiveRow::KeyboardReachability);
    binding.evidence_freshness = Some(M5EvidenceFreshness::Stale);
    let surface = build_m5_inclusive_row(descriptor, vec![binding]);
    assert!(surface.blocking_findings.iter().any(|f| matches!(
        f,
        M5InclusiveBlockingFinding::StaleEvidenceOnMarketedRow { .. }
    )));
}

#[test]
fn dimension_drift_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_interaction_binding(M5InclusiveRow::KeyboardReachability);
    binding.dimension = M5InclusiveDimension::Localization;
    let surface = build_m5_inclusive_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5InclusiveBlockingFinding::DimensionDrift { .. })));
}

#[test]
fn missing_projection_blocks_on_qualified_row() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_interaction_binding(M5InclusiveRow::KeyboardReachability);
    binding.projected_evidence_pack_ref = None;
    let surface = build_m5_inclusive_row(descriptor, vec![binding]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("missing_projection"));
    assert!(tokens.contains("missing_evidence_pack"));
}

#[test]
fn missing_conditional_projection_blocks() {
    let descriptor = high_salience_descriptor();
    let mut ime = qualified_interaction_binding(M5InclusiveRow::ImeComposition);
    ime.dimension = M5InclusiveDimension::Text;
    ime.projected_ime_composition = None;
    let mut locale = qualified_localization_binding(M5InclusiveRow::Pseudolocalization);
    locale.projected_locale_parity = None;
    let surface = build_m5_inclusive_row(descriptor, vec![ime, locale]);
    let fields: BTreeSet<String> = surface
        .blocking_findings
        .iter()
        .filter_map(|f| match f {
            M5InclusiveBlockingFinding::MissingProjection { field, .. } => Some(field.clone()),
            _ => None,
        })
        .collect();
    assert!(fields.contains("projected_ime_composition"));
    assert!(fields.contains("projected_locale_parity"));
}

#[test]
fn descriptor_level_findings_fire() {
    let descriptor = M5InclusiveDescriptor {
        surface_id: "surface:support.packet".to_owned(),
        surface_family: M5InclusiveSurfaceFamily::SupportPacket,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        locale_anchor_ref: "  ".to_owned(),
        inclusive_note: "  ".to_owned(),
        semantic_salience: M5SemanticSalience::SeverityBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        claimed_locales: Vec::new(),
        marketed_on_inclusive_rows: true,
        registered_on_inclusive_harness: false,
    };
    let surface = build_m5_inclusive_row(descriptor, vec![]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("descriptor_missing_locale_anchor"));
    assert!(tokens.contains("missing_inclusive_note"));
    assert!(tokens.contains("missing_claimed_locales"));
    assert!(tokens.contains("surface_not_on_inclusive_harness"));
}

#[test]
fn missing_narrowing_reason_blocks() {
    let mut report = seeded_m5_inclusive_depth_audit();
    let surface = report
        .rows
        .iter_mut()
        .find(|surface| surface.descriptor.surface_id == "surface:preview.route_pane")
        .expect("preview route surface present");
    let binding = surface
        .bindings
        .iter_mut()
        .find(|binding| {
            binding.qualification_status == M5InclusiveQualificationStatus::DeclaredCaptureGap
        })
        .expect("declared capture gap binding present");
    binding.narrowing_reason = None;
    let rebuilt = build_m5_inclusive_row(surface.descriptor.clone(), surface.bindings.clone());
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5InclusiveBlockingFinding::MissingNarrowingReason { .. })));
}

#[test]
fn missing_required_row_blocks_validation() {
    let mut report = seeded_m5_inclusive_depth_audit();
    report.rows[0]
        .bindings
        .retain(|binding| binding.row != M5InclusiveRow::KeyboardReachability);
    let result = validate_m5_inclusive_depth(&report);
    assert!(result.is_err());
}

#[test]
fn narrowable_rows_surface_a_blocking_marketed_row() {
    let report = seeded_m5_inclusive_depth_audit();
    let mut surfaces = report.rows.clone();
    let surface = surfaces
        .iter_mut()
        .find(|surface| surface.descriptor.surface_id == "surface:support.packet")
        .expect("support packet surface present");
    let mut bindings = surface.bindings.clone();
    let binding = bindings
        .iter_mut()
        .find(|binding| binding.row == M5InclusiveRow::ScreenReaderNarration)
        .expect("narration binding present");
    binding.projected_narration = Some(M5Narration::Silent);
    *surface = build_m5_inclusive_row(surface.descriptor.clone(), bindings);
    let rebuilt = build_m5_inclusive_depth_audit(surfaces);
    assert!(!rebuilt.report_clean);
    assert!(rebuilt.narrowable_marketed_rows.iter().any(|narrowable| {
        narrowable.surface_id == "surface:support.packet"
            && narrowable.row == M5InclusiveRow::ScreenReaderNarration
    }));
}

#[test]
fn support_export_quotes_every_surface_id() {
    let report = seeded_m5_inclusive_depth_audit();
    let export =
        M5InclusiveSupportExport::from_report(M5_INCLUSIVE_SUPPORT_EXPORT_ID, report.clone());
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
    let report = seeded_m5_inclusive_depth_audit();
    assert_eq!(report.render_markdown(), report.render_markdown());
}
