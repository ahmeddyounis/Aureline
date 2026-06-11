//! Inline unit tests for the M5 desktop-and-handoff qualification audit.

use std::collections::BTreeSet;

use super::*;

#[test]
fn seeded_audit_passes_validation() {
    let report = seeded_m5_desktop_qualification_audit();
    validate_m5_desktop_qualification(&report).expect("seeded audit must validate");
}

#[test]
fn seeded_audit_qualifies_every_required_row() {
    let report = seeded_m5_desktop_qualification_audit();
    assert!(report.every_required_row_qualified());
}

#[test]
fn seeded_audit_has_no_blocking_findings() {
    let report = seeded_m5_desktop_qualification_audit();
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
    let report = seeded_m5_desktop_qualification_audit();
    let families: BTreeSet<M5DesktopSurfaceFamily> = report
        .rows
        .iter()
        .map(|surface| surface.descriptor.surface_family)
        .collect();
    for expected in [
        M5DesktopSurfaceFamily::NotebookCellChrome,
        M5DesktopSurfaceFamily::ResultGridRow,
        M5DesktopSurfaceFamily::ProfilerPanel,
        M5DesktopSurfaceFamily::TracePanel,
        M5DesktopSurfaceFamily::PipelineCard,
        M5DesktopSurfaceFamily::PreviewRouteBadge,
        M5DesktopSurfaceFamily::DocsBrowserPane,
        M5DesktopSurfaceFamily::CompanionSurface,
        M5DesktopSurfaceFamily::SyncStatusSurface,
        M5DesktopSurfaceFamily::OffboardingSurface,
        M5DesktopSurfaceFamily::IncidentPacket,
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
    let report = seeded_m5_desktop_qualification_audit();
    for row in M5DesktopRow::required_rows() {
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
fn seeded_audit_claims_every_desktop_profile() {
    let report = seeded_m5_desktop_qualification_audit();
    assert_eq!(
        report.claimed_desktop_profiles,
        M5DesktopProfile::all().to_vec()
    );
}

#[test]
fn high_salience_surfaces_project_a_boundary_cue_on_qualified_rows() {
    let report = seeded_m5_desktop_qualification_audit();
    assert!(report.high_salience_surface_count > 0);
    for surface in &report.rows {
        if !surface.high_salience {
            continue;
        }
        for binding in &surface.bindings {
            if binding.qualification_status == M5DesktopQualificationStatus::Qualified {
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
fn reopen_anchor_index_covers_every_surface() {
    let report = seeded_m5_desktop_qualification_audit();
    assert_eq!(report.reopen_anchor_index.len(), report.rows.len());
    for surface in &report.rows {
        let entry = report
            .reopen_anchor_index
            .iter()
            .find(|entry| entry.surface_id == surface.descriptor.surface_id)
            .expect("every surface must have a reopen-anchor entry");
        assert_eq!(
            entry.reopen_anchor_ref,
            surface.descriptor.reopen_anchor_ref
        );
        assert!(!entry.reopen_anchor_ref.is_empty());
    }
}

/// A minimal high-salience descriptor used to exercise binding findings.
fn high_salience_descriptor() -> M5DesktopDescriptor {
    M5DesktopDescriptor {
        surface_id: "surface:sync.status_surface".to_owned(),
        surface_family: M5DesktopSurfaceFamily::SyncStatusSurface,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        reopen_anchor_ref: "reopen:anchor:sync:status_surface".to_owned(),
        continuity_note: "note".to_owned(),
        semantic_salience: M5SemanticSalience::SeverityBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        claimed_desktop_profiles: M5DesktopProfile::all().to_vec(),
        marketed_on_desktop_rows: true,
        registered_on_platform_conformance: true,
    }
}

/// A fully-projected qualified binding for a window-topology row, ready to be
/// mutated into a red result by individual tests.
fn qualified_topology_binding(row: M5DesktopRow) -> M5DesktopBinding {
    M5DesktopBinding {
        row,
        dimension: row.canonical_dimension(),
        qualification_status: M5DesktopQualificationStatus::Qualified,
        marketed_on_row: true,
        projected_evidence_pack_ref: Some(
            "drill:surface:sync.status_surface:multi_window".to_owned(),
        ),
        projected_reopen_fidelity: Some(M5ReopenFidelity::ExactTargetPreserved),
        projected_layout_continuity: Some(M5LayoutContinuity::Preserved),
        projected_interruption_safety: Some(M5InterruptionSafety::Safe),
        projected_placeholder_honesty: Some(M5PlaceholderHonesty::Honest),
        projected_authority_context: None,
        projected_background_adaptation: None,
        projected_handoff_reason: None,
        projected_boundary_cue: Some(M5BoundaryCue::Present),
        evidence_freshness: Some(M5EvidenceFreshness::Fresh),
        evidence_captured_at: Some("2026-06-11T00:00:00Z".to_owned()),
        narrowing_reason: None,
        note: None,
    }
}

/// A fully-projected qualified binding for a handoff row.
fn qualified_handoff_binding(row: M5DesktopRow) -> M5DesktopBinding {
    let mut binding = qualified_topology_binding(row);
    binding.dimension = row.canonical_dimension();
    binding.projected_authority_context = Some(M5AuthorityContext::Preserved);
    binding.projected_handoff_reason = Some(M5HandoffReason::Preserved);
    binding
}

#[test]
fn unqualified_local_platform_path_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_topology_binding(M5DesktopRow::MultiWindow);
    binding.qualification_status = M5DesktopQualificationStatus::UnqualifiedLocalPlatformPath;
    let surface = build_m5_desktop_row(descriptor, vec![binding]);
    assert!(surface.blocking_findings.iter().any(|f| matches!(
        f,
        M5DesktopBlockingFinding::UnqualifiedLocalPlatformPath { .. }
    )));
}

#[test]
fn missing_evidence_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_topology_binding(M5DesktopRow::MultiMonitor);
    binding.qualification_status = M5DesktopQualificationStatus::MissingEvidence;
    let surface = build_m5_desktop_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5DesktopBlockingFinding::MissingEvidence { .. })));
}

#[test]
fn red_topology_results_block() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_topology_binding(M5DesktopRow::MixedDpi);
    binding.projected_reopen_fidelity = Some(M5ReopenFidelity::Lost);
    binding.projected_layout_continuity = Some(M5LayoutContinuity::Broken);
    binding.projected_interruption_safety = Some(M5InterruptionSafety::Corrupted);
    binding.projected_placeholder_honesty = Some(M5PlaceholderHonesty::Misleading);
    binding.projected_boundary_cue = Some(M5BoundaryCue::Hidden);
    let surface = build_m5_desktop_row(descriptor, vec![binding]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("reopen_target_lost"));
    assert!(tokens.contains("layout_continuity_broken"));
    assert!(tokens.contains("interruption_unsafe"));
    assert!(tokens.contains("placeholder_misleading"));
    assert!(tokens.contains("boundary_cue_hidden"));
}

#[test]
fn power_row_not_throttled_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_topology_binding(M5DesktopRow::BatterySaver);
    binding.dimension = M5DesktopDimension::PowerState;
    binding.projected_background_adaptation = Some(M5BackgroundAdaptation::NotThrottled);
    let surface = build_m5_desktop_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5DesktopBlockingFinding::BackgroundNotThrottled { .. })));
}

#[test]
fn handoff_row_dropped_reason_and_lost_authority_block() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_handoff_binding(M5DesktopRow::DeepLink);
    binding.projected_handoff_reason = Some(M5HandoffReason::Dropped);
    binding.projected_authority_context = Some(M5AuthorityContext::Lost);
    let surface = build_m5_desktop_row(descriptor, vec![binding]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("handoff_reason_dropped"));
    assert!(tokens.contains("authority_context_lost"));
}

#[test]
fn stale_evidence_on_marketed_row_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_topology_binding(M5DesktopRow::MultiWindow);
    binding.evidence_freshness = Some(M5EvidenceFreshness::Stale);
    let surface = build_m5_desktop_row(descriptor, vec![binding]);
    assert!(surface.blocking_findings.iter().any(|f| matches!(
        f,
        M5DesktopBlockingFinding::StaleEvidenceOnMarketedRow { .. }
    )));
}

#[test]
fn dimension_drift_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_topology_binding(M5DesktopRow::MultiWindow);
    binding.dimension = M5DesktopDimension::Handoff;
    let surface = build_m5_desktop_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5DesktopBlockingFinding::DimensionDrift { .. })));
}

#[test]
fn missing_projection_blocks_on_qualified_row() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_topology_binding(M5DesktopRow::MultiWindow);
    binding.projected_evidence_pack_ref = None;
    let surface = build_m5_desktop_row(descriptor, vec![binding]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("missing_projection"));
    assert!(tokens.contains("missing_evidence_pack"));
}

#[test]
fn missing_handoff_projection_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = qualified_handoff_binding(M5DesktopRow::FileAssociation);
    binding.projected_handoff_reason = None;
    binding.projected_authority_context = None;
    let surface = build_m5_desktop_row(descriptor, vec![binding]);
    let fields: BTreeSet<String> = surface
        .blocking_findings
        .iter()
        .filter_map(|f| match f {
            M5DesktopBlockingFinding::MissingProjection { field, .. } => Some(field.clone()),
            _ => None,
        })
        .collect();
    assert!(fields.contains("projected_handoff_reason"));
    assert!(fields.contains("projected_authority_context"));
}

#[test]
fn descriptor_level_findings_fire() {
    let descriptor = M5DesktopDescriptor {
        surface_id: "surface:sync.status_surface".to_owned(),
        surface_family: M5DesktopSurfaceFamily::SyncStatusSurface,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        reopen_anchor_ref: "  ".to_owned(),
        continuity_note: "  ".to_owned(),
        semantic_salience: M5SemanticSalience::SeverityBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        claimed_desktop_profiles: Vec::new(),
        marketed_on_desktop_rows: true,
        registered_on_platform_conformance: false,
    };
    let surface = build_m5_desktop_row(descriptor, vec![]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("descriptor_missing_reopen_anchor"));
    assert!(tokens.contains("missing_continuity_note"));
    assert!(tokens.contains("missing_claimed_profiles"));
    assert!(tokens.contains("surface_not_on_platform_conformance"));
}

#[test]
fn missing_narrowing_reason_blocks() {
    let mut report = seeded_m5_desktop_qualification_audit();
    let surface = report
        .rows
        .iter_mut()
        .find(|surface| surface.descriptor.surface_id == "surface:docs_browser.pane")
        .expect("docs browser surface present");
    let binding = surface
        .bindings
        .iter_mut()
        .find(|binding| {
            binding.qualification_status == M5DesktopQualificationStatus::DeclaredCaptureGap
        })
        .expect("declared capture gap binding present");
    binding.narrowing_reason = None;
    let rebuilt = build_m5_desktop_row(surface.descriptor.clone(), surface.bindings.clone());
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5DesktopBlockingFinding::MissingNarrowingReason { .. })));
}

#[test]
fn missing_required_row_blocks_validation() {
    let mut report = seeded_m5_desktop_qualification_audit();
    report.rows[0]
        .bindings
        .retain(|binding| binding.row != M5DesktopRow::MultiWindow);
    let result = validate_m5_desktop_qualification(&report);
    assert!(result.is_err());
}

#[test]
fn narrowable_rows_surface_a_blocking_marketed_row() {
    let report = seeded_m5_desktop_qualification_audit();
    let mut surfaces = report.rows.clone();
    let surface = surfaces
        .iter_mut()
        .find(|surface| surface.descriptor.surface_id == "surface:sync.status_surface")
        .expect("sync surface present");
    let mut bindings = surface.bindings.clone();
    let binding = bindings
        .iter_mut()
        .find(|binding| binding.row == M5DesktopRow::SuspendResume)
        .expect("suspend/resume binding present");
    binding.projected_reopen_fidelity = Some(M5ReopenFidelity::Lost);
    *surface = build_m5_desktop_row(surface.descriptor.clone(), bindings);
    let rebuilt = build_m5_desktop_qualification_audit(surfaces);
    assert!(!rebuilt.report_clean);
    assert!(rebuilt.narrowable_marketed_rows.iter().any(|narrowable| {
        narrowable.surface_id == "surface:sync.status_surface"
            && narrowable.row == M5DesktopRow::SuspendResume
    }));
}

#[test]
fn support_export_quotes_every_surface_id() {
    let report = seeded_m5_desktop_qualification_audit();
    let export = M5DesktopSupportExport::from_report(M5_DESKTOP_SUPPORT_EXPORT_ID, report.clone());
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
    let report = seeded_m5_desktop_qualification_audit();
    assert_eq!(report.render_markdown(), report.render_markdown());
}
