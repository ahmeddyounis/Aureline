//! Unit tests for the live-appearance evidence-linkage lane.

use super::*;

fn report() -> LiveAppearanceEvidenceReport {
    seeded_live_appearance_evidence_report()
}

#[test]
fn seeded_report_is_clean_and_validates() {
    let report = report();
    assert!(report.report_clean, "seeded report must be clean");
    assert!(report.blocking_findings.is_empty());
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    validate_live_appearance_evidence_report(&report).expect("seeded report must validate");
}

#[test]
fn envelope_uses_frozen_contract_constants() {
    let report = report();
    assert_eq!(report.record_kind, M5_LIVE_APPEARANCE_REPORT_RECORD_KIND);
    assert_eq!(report.schema_version, M5_LIVE_APPEARANCE_SCHEMA_VERSION);
    assert_eq!(
        report.shared_contract_ref,
        M5_LIVE_APPEARANCE_SHARED_CONTRACT_REF
    );
    assert_eq!(report.report_id, M5_LIVE_APPEARANCE_REPORT_ID);
    assert_eq!(
        report.source_schema_ref,
        M5_LIVE_APPEARANCE_SOURCE_SCHEMA_REF
    );
    assert_eq!(
        report.published_report_ref,
        M5_LIVE_APPEARANCE_PUBLISHED_REPORT_REF
    );
    assert_eq!(
        report.published_doc_ref,
        M5_LIVE_APPEARANCE_PUBLISHED_DOC_REF
    );
}

#[test]
fn every_capture_is_attributable_to_the_exact_build_and_row() {
    let report = report();
    assert!(report.all_captures_build_attributed);
    for row in &report.rows {
        let Some(evidence) = &row.evidence else {
            continue;
        };
        let attribution = &evidence.attribution;
        assert_eq!(attribution.build_identity_ref, report.build_identity_ref);
        assert_eq!(attribution.theme_package_ref, row.theme_package_ref);
        assert_eq!(
            attribution.appearance_session_ref,
            row.appearance_session_ref
        );
        assert_eq!(attribution.checkpoint_ref, row.checkpoint_ref);
        assert_eq!(attribution.platform, row.platform);
        assert_eq!(attribution.os_signal, row.os_signal);
        assert!(!attribution.build_identity_ref.trim().is_empty());
        assert!(!attribution.release_channel_class.trim().is_empty());
    }
}

#[test]
fn declared_axis_always_matches_os_signal() {
    for row in &report().rows {
        assert_eq!(
            row.changed_axis,
            row.os_signal.canonical_axis(),
            "row {} axis disagrees with its OS signal",
            row.row_id
        );
    }
}

#[test]
fn reload_or_restart_posture_is_always_disclosed() {
    let report = report();
    let mut saw_reload = false;
    let mut saw_restart = false;
    for row in &report.rows {
        if row.posture_needs_reload_or_restart() {
            assert!(
                row.restart_or_reload_disclosed,
                "row {} hides its reload/restart posture",
                row.row_id
            );
        }
        match row.apply_posture {
            LiveApplyCapability::RequiresSurfaceReload => saw_reload = true,
            LiveApplyCapability::RequiresAppRestart => saw_restart = true,
            _ => {}
        }
    }
    assert!(saw_reload, "fixture must exercise a surface-reload posture");
    assert!(saw_restart, "fixture must exercise an app-restart posture");
}

#[test]
fn marketed_axes_are_proven_on_at_least_two_platforms() {
    let report = report();
    assert!(
        !report.axis_platform_coverage.is_empty(),
        "report must declare axis coverage"
    );
    for coverage in &report.axis_platform_coverage {
        assert!(
            coverage.platforms.len() >= 2,
            "axis {} is proven on only {} platform(s)",
            coverage.axis.as_str(),
            coverage.platforms.len()
        );
    }
}

#[test]
fn required_surface_families_are_all_covered() {
    let report = report();
    for family in REQUIRED_SURFACE_FAMILIES {
        assert!(
            report
                .covered_surface_families
                .iter()
                .any(|s| s == family.as_str()),
            "surface family {} is not covered by a qualified row",
            family.as_str()
        );
    }
}

#[test]
fn qualified_rows_preserve_high_salience_cues() {
    for row in &report().rows {
        if !row.is_qualified() {
            continue;
        }
        let cues = row.cue_preservation.expect("qualified row carries cues");
        assert!(cues.structurally_intact());
        match row.semantic_salience {
            M5SemanticSalience::TrustBearing => {
                assert_eq!(cues.trust_cue, M5BoundaryCue::Present)
            }
            M5SemanticSalience::SeverityBearing => {
                assert_eq!(cues.severity_cue, M5BoundaryCue::Present)
            }
            M5SemanticSalience::LifecycleBearing => {
                assert_eq!(cues.lifecycle_cue, M5BoundaryCue::Present)
            }
            _ => {}
        }
    }
}

#[test]
fn fixture_covers_every_os_signal_and_platform() {
    let report = report();
    for signal in [
        OsAppearanceSignal::SystemThemeFlip,
        OsAppearanceSignal::ContrastIncreased,
        OsAppearanceSignal::ForcedColorsEnabled,
        OsAppearanceSignal::AccentColorChanged,
        OsAppearanceSignal::TextScaleIncreased,
        OsAppearanceSignal::ReducedMotionEnabled,
    ] {
        assert!(
            report.os_signal_coverage.contains(&signal),
            "fixture must exercise {}",
            signal.as_str()
        );
    }
    for platform in [
        DesktopPlatform::Macos,
        DesktopPlatform::Windows,
        DesktopPlatform::Linux,
    ] {
        assert!(
            report.rows.iter().any(|row| row.platform == platform),
            "fixture must exercise {}",
            platform.as_str()
        );
    }
}

#[test]
fn honest_platform_omission_is_accepted_not_blocking() {
    let report = report();
    let omitted = report
        .rows
        .iter()
        .find(|row| {
            matches!(
                row.qualification_status,
                M5QualificationStatus::PlatformOmitted
            )
        })
        .expect("fixture must include a disclosed platform omission");
    assert!(omitted.narrowing_reason.is_some());
    assert!(omitted.evidence.is_none());
    assert!(!omitted.is_marketed());
    // An omitted row does not count toward cross-platform or surface coverage and
    // does not block the report.
    assert!(report.report_clean);
}

#[test]
fn support_export_quotes_build_session_and_capture_refs() {
    let report = report();
    let export = LiveAppearanceEvidenceSupportExport::from_report(
        M5_LIVE_APPEARANCE_SUPPORT_EXPORT_ID,
        report.clone(),
    );
    assert!(export.case_ids.contains(&report.report_id));
    assert!(export.case_ids.contains(&report.build_identity_ref));
    for row in &report.rows {
        assert!(export.case_ids.contains(&row.row_id));
        assert!(export.case_ids.contains(&row.appearance_session_ref));
        assert!(export.case_ids.contains(&row.checkpoint_ref));
        if let Some(evidence) = &row.evidence {
            assert!(export.case_ids.contains(&evidence.screenshot_ref));
            assert!(export.case_ids.contains(&evidence.golden_baseline_ref));
        }
    }
}

#[test]
fn hidden_trust_cue_is_caught() {
    let mut report = report();
    let row = report
        .rows
        .iter_mut()
        .find(|row| row.semantic_salience == M5SemanticSalience::TrustBearing)
        .expect("a trust-bearing row");
    if let Some(cues) = row.cue_preservation.as_mut() {
        cues.trust_cue = M5BoundaryCue::Hidden;
    }
    let rebuilt = build_live_appearance_evidence_report(
        report.build_identity_ref.clone(),
        report.release_channel_class.clone(),
        report.rows.clone(),
    );
    assert!(!rebuilt.report_clean);
    assert!(rebuilt.blocking_findings.iter().any(
        |f| matches!(f, LiveAppearanceBlockingFinding::CueHidden { cue, .. } if cue == "trust")
    ));
}

#[test]
fn unattributed_build_is_caught() {
    let mut rows = report().rows;
    if let Some(evidence) = rows[0].evidence.as_mut() {
        evidence.attribution.build_identity_ref = "build-id:aureline:other".to_owned();
    }
    let rebuilt = build_live_appearance_evidence_report(
        SEED_BUILD_IDENTITY_REF,
        SEED_RELEASE_CHANNEL_CLASS,
        rows,
    );
    assert!(!rebuilt.report_clean);
    assert!(rebuilt.blocking_findings.iter().any(|f| matches!(
        f,
        LiveAppearanceBlockingFinding::BuildAttributionMismatch { .. }
    )));
    assert!(!rebuilt.all_captures_build_attributed);
}

#[test]
fn single_platform_axis_claim_is_caught() {
    // Drop every Windows + Linux contrast row, leaving contrast on macOS only.
    let mut rows = report().rows;
    rows.retain(|row| {
        !(row.changed_axis == AppearanceAxis::Contrast
            && row.platform != DesktopPlatform::Macos
            && row.is_marketed())
    });
    let rebuilt = build_live_appearance_evidence_report(
        SEED_BUILD_IDENTITY_REF,
        SEED_RELEASE_CHANNEL_CLASS,
        rows,
    );
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| matches!(f, LiveAppearanceBlockingFinding::SinglePlatformClaim { axis } if axis == "contrast")));
}

#[test]
fn static_only_evidence_for_a_live_change_is_caught() {
    let mut rows = report().rows;
    let row = rows
        .iter_mut()
        .find(|row| row.apply_posture.applies_live() && row.is_qualified())
        .expect("a live-applying qualified row");
    if let Some(evidence) = row.evidence.as_mut() {
        evidence.capture_kind = EvidenceCaptureKind::SteadyState;
    }
    let rebuilt = build_live_appearance_evidence_report(
        SEED_BUILD_IDENTITY_REF,
        SEED_RELEASE_CHANNEL_CLASS,
        rows,
    );
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| matches!(f, LiveAppearanceBlockingFinding::StaticEvidenceOnly { .. })));
}

#[test]
fn compact_lines_are_deterministic_and_nonempty() {
    let report = report();
    let first = report.compact_lines();
    let second = report.compact_lines();
    assert_eq!(first, second);
    assert!(first.iter().any(|line| line.starts_with("report:")));
    assert!(first.iter().any(|line| line.starts_with("axis:")));
}

#[test]
fn markdown_render_is_deterministic() {
    let report = report();
    assert_eq!(report.render_markdown(), report.render_markdown());
    assert!(report
        .render_markdown()
        .contains("# M5 live-appearance change & evidence-linkage report"));
}
