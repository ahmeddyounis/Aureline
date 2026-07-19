//! Tests for the M05-818 manifest / build component accessibility fallback packet.

use super::*;

fn packet() -> ComponentAccessibilityPacket {
    seeded_m5_manifest_build_a11y_fallback_packet()
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
    assert_eq!(p.record_kind, MANIFEST_BUILD_A11Y_FALLBACK_RECORD_KIND);
    assert_eq!(
        p.schema_version,
        MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION
    );
    assert_eq!(
        p.matrix_ref,
        MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let families = packet().represented_families();
    for family in M5ManifestBuildComponentFamily::ALL {
        assert!(
            families.contains(&family),
            "family {family:?} is not certified"
        );
    }
    assert_eq!(families.len(), M5ManifestBuildComponentFamily::ALL.len());
}

#[test]
fn summary_matches_computed() {
    let p = packet();
    assert_eq!(p.summary, p.computed_summary());
}

#[test]
fn seeded_status_split_is_six_green_four_yellow_zero_red() {
    let p = packet();
    assert_eq!(p.summary.green_count, 6, "green");
    assert_eq!(p.summary.yellow_count, 4, "yellow");
    assert_eq!(p.summary.red_count, 0, "red");
}

#[test]
fn visual_heavy_families_offer_a_non_visual_fallback() {
    for row in &packet().rows {
        if row.is_visual_heavy() {
            assert!(
                row.fallback_modalities
                    .contains(&M5ManifestBuildFallbackModality::Graph),
                "{} must render a graph",
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
fn ac1_no_row_overclaims_execution() {
    for row in &packet().rows {
        assert!(row.claim_is_honest(), "{} overclaims", row.row_id);
        // A weak-truth row can never present as fully executable.
        if row.truth_signals.is_weak() {
            assert!(
                !row.granted_claim.is_fully_executable(),
                "{} presents fully executable on weak truth",
                row.row_id
            );
        }
    }
}

#[test]
fn ac2_every_row_reaches_target_backed_truth_via_at() {
    for row in &packet().rows {
        assert!(
            row.reaches_target_backed_truth_via_at(),
            "{} strands assistive tech / CLI",
            row.row_id
        );
        assert!(row.keyboard_reach.never_traps(), "{}", row.row_id);
        assert!(row.screen_reader_reach.never_traps(), "{}", row.row_id);
        assert!(row.cli_reach.never_traps(), "{}", row.row_id);
    }
}

#[test]
fn ac2_export_preserves_target_identity_and_confidence() {
    for row in &packet().rows {
        assert!(
            row.export_preserves_meaning(),
            "{} export drops target truth",
            row.row_id
        );
        assert!(!row.target_id.trim().is_empty(), "{}", row.row_id);
        assert!(!row.target_context_ref.trim().is_empty(), "{}", row.row_id);
        assert!(row.copy_export.is_complete(), "{}", row.row_id);
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
fn ac3_claim_publication_and_field_triage_are_aligned() {
    let p = packet();
    assert!(p.field_triage_and_publication_aligned());
    assert!(p.summary.field_triage_and_publication_aligned);
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
fn overclaimed_execution_on_weak_truth_is_stranded() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.component_family == M5ManifestBuildComponentFamily::ManifestEditorHeader)
        .expect("manifest header row");
    // Stale truth but the lane still claims fully executable and presents matches.
    row.granted_claim = NarrowedClaimTier::FullyExecutable;
    row.claim_affordance = ClaimAffordanceState::MatchesTruth;
    assert!(!row.claim_is_honest());
    assert_eq!(row.status(), ComponentAccessibilityStatus::Stranded);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ComponentAccessibilityViolation::OverclaimedExecutable { .. }
    )));
    assert!(violations
        .iter()
        .any(|v| matches!(v, ComponentAccessibilityViolation::StrandedRow { .. })));
}

#[test]
fn granted_claim_above_baseline_is_dishonest() {
    let mut p = packet();
    let row = &mut p.rows[1];
    // Read-only baseline cannot grant a higher tier.
    row.granted_claim = NarrowedClaimTier::FullyExecutable;
    assert!(!row.claim_is_honest());
}

#[test]
fn spurious_auto_narrow_declaration_is_dishonest() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.component_family == M5ManifestBuildComponentFamily::ResourceExplorerRow)
        .expect("resource explorer row (green)");
    // Full-parity row declaring AutoNarrowedDisclosed without narrowing.
    row.claim_affordance = ClaimAffordanceState::AutoNarrowedDisclosed;
    assert!(!row.claim_is_honest());
}

#[test]
fn view_only_trap_strands_assistive_tech() {
    let mut p = packet();
    let row = &mut p.rows[1];
    row.screen_reader_reach = NonVisualReachState::ViewOnlyTrap;
    assert!(!row.reaches_target_backed_truth_via_at());
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        ComponentAccessibilityViolation::AssistiveTechStranded { .. }
    )));
}

#[test]
fn cli_trap_strands_headless_users() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.cli_reach = NonVisualReachState::ViewOnlyTrap;
    assert!(!row.reaches_target_backed_truth_via_at());
}

#[test]
fn visual_heavy_without_non_visual_fallback_strands() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.component_family == M5ManifestBuildComponentFamily::TargetGraphRow)
        .expect("target graph row");
    row.fallback_modalities = vec![M5ManifestBuildFallbackModality::Graph];
    assert!(!row.reaches_target_backed_truth_via_at());
}

#[test]
fn export_dropping_target_id_is_flagged() {
    let mut p = packet();
    p.rows[0].target_id = "   ".to_owned();
    assert!(!p.rows[0].export_preserves_meaning());
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        ComponentAccessibilityViolation::ExportDropsTargetTruth { .. }
    )));
}

#[test]
fn silent_narrowing_is_rejected() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.component_family == M5ManifestBuildComponentFamily::TargetGraphRow)
        .expect("target graph row");
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
fn auto_narrow_dropping_target_context_is_rejected() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.auto_narrow.is_some())
        .expect("a reduced row");
    row.auto_narrow.as_mut().unwrap().preserves_target_context = false;
    assert!(!row.narrowing_disclosed());
}

#[test]
fn missing_mandatory_label_is_flagged() {
    let mut p = packet();
    p.rows[0]
        .required_labels
        .retain(|l| *l != M5ManifestBuildRequiredLabel::KeyboardRoute);
    assert!(!p.rows[0].preserves_mandatory_labels());
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        ComponentAccessibilityViolation::MissingMandatoryLabels { .. }
    )));
}

#[test]
fn missing_family_coverage_is_flagged() {
    let mut p = packet();
    p.rows
        .retain(|r| r.component_family != M5ManifestBuildComponentFamily::RawEventDrawer);
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        ComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn triage_publication_misalignment_is_flagged() {
    let mut p = packet();
    // Drop every incident-support consumer surface.
    for row in &mut p.rows {
        row.consumer_surfaces
            .retain(|s| *s != M5ManifestBuildConsumerSurface::IncidentSupport);
    }
    // Restore two-consumer parity where the drop left a single surface.
    for row in &mut p.rows {
        if row.consumer_surfaces.len() < 2 {
            row.consumer_surfaces
                .push(M5ManifestBuildConsumerSurface::DocsHelp);
            row.consumer_surfaces.dedup();
        }
    }
    p.summary = p.computed_summary();
    assert!(!p.field_triage_and_publication_aligned());
    assert!(p.validate().iter().any(|v| matches!(
        v,
        ComponentAccessibilityViolation::TriagePublicationMisaligned
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
    let disk = current_m5_manifest_build_a11y_fallback_export()
        .expect("checked-in export must parse and validate");
    assert_eq!(disk, packet(), "on-disk export drifted from the builder");
}

#[test]
fn csv_has_a_row_per_component() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    assert_eq!(lines, packet().rows.len() + 1);
    assert!(csv.contains("target_graph_row"));
    assert!(csv.contains("capability_matrix"));
}

#[test]
fn markdown_summary_names_every_family() {
    let md = packet().render_markdown_summary();
    for family in M5ManifestBuildComponentFamily::ALL {
        assert!(md.contains(family.as_str()), "missing {}", family.as_str());
    }
}

#[test]
fn export_is_deterministic() {
    assert_eq!(packet().export_safe_json(), packet().export_safe_json());
}
