//! Tests for the M05-851 workflow-bundle component surface certification packet.

use super::*;

fn packet() -> BundleSurfaceCertPacket {
    seeded_m5_bundle_surface_cert_packet()
}

fn row(id: &str) -> BundleSurfaceCertRow {
    packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
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
    assert_eq!(p.record_kind, BUNDLE_SURFACE_CERT_RECORD_KIND);
    assert_eq!(p.schema_version, BUNDLE_SURFACE_CERT_SCHEMA_VERSION);
    assert_eq!(p.matrix_ref, BUNDLE_SURFACE_CERT_COMPONENT_MATRIX_REF);
    assert_eq!(p.certification_bundle_ref, BUNDLE_SURFACE_CERT_BUNDLE_REF);
}

#[test]
fn every_claimed_surface_is_certified() {
    let surfaces = packet().represented_surfaces();
    for surface in M5WorkflowBundleClaimedSurface::ALL {
        assert!(
            surfaces.contains(&surface),
            "surface {surface:?} not certified"
        );
    }
    assert_eq!(surfaces.len(), M5WorkflowBundleClaimedSurface::ALL.len());
}

#[test]
fn every_component_group_is_consumed() {
    let consumed = packet().consumed_groups();
    for group in M5WorkflowBundleComponentGroup::ALL {
        assert!(consumed.contains(&group), "group {group:?} never consumed");
    }
}

#[test]
fn every_distribution_path_is_exercised() {
    let paths = packet().covered_paths();
    for path in M5BundleDistributionPath::ALL {
        assert!(paths.contains(&path), "path {path:?} never exercised");
    }
}

#[test]
fn evidence_surfaces_are_present() {
    let surfaces = packet().represented_surfaces();
    for surface in M5WorkflowBundleClaimedSurface::EVIDENCE_SURFACES {
        assert!(
            surfaces.contains(&surface),
            "evidence surface {surface:?} missing"
        );
    }
}

#[test]
fn summary_matches_computed() {
    let p = packet();
    assert_eq!(p.summary, p.computed_summary());
}

#[test]
fn seeded_status_split_is_six_green_three_yellow_zero_red() {
    let p = packet();
    assert_eq!(p.summary.green_count, 6, "green");
    assert_eq!(p.summary.yellow_count, 3, "yellow");
    assert_eq!(p.summary.red_count, 0, "red");
}

#[test]
fn coverage_flags_are_complete() {
    let s = packet().summary;
    assert!(s.group_coverage_complete);
    assert!(s.path_coverage_complete);
    assert!(s.all_claims_honest);
    assert!(s.all_export_preserve_truth);
    assert!(s.all_unsupported_paths_narrowed);
}

#[test]
fn ac1_certified_surface_asserts_declared_claim_without_narrowing() {
    let r = row("cert:start-center-picker");
    assert!(!r.claim_narrowed());
    assert!(r.claim_auto_narrow.is_none());
    assert_eq!(r.declared_claim, r.effective_claim);
    assert!(r.claim_is_honest());
    assert_eq!(r.status(), M5BundleSurfaceCertStatus::Certified);
}

#[test]
fn ac1_imported_handoff_narrows_to_imported() {
    let r = row("cert:migration-center");
    assert!(r.claim_narrowed());
    assert_eq!(r.effective_claim, M5BundleSupportClaim::Imported);
    assert_eq!(
        r.binding_group(),
        Some(M5WorkflowBundleComponentGroup::ClassDisclosure)
    );
    let narrow = r.claim_auto_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5BundleComponentDowngradeTrigger::ImportedNotNative
    );
    assert!(r.claim_is_honest());
    assert_eq!(r.status(), M5BundleSurfaceCertStatus::NarrowedDisclosed);
}

#[test]
fn ac1_local_override_drift_narrows_to_limited() {
    let r = row("cert:diagnostics");
    assert_eq!(
        r.binding_group(),
        Some(M5WorkflowBundleComponentGroup::DriftOverride)
    );
    assert_eq!(r.effective_claim, M5BundleSupportClaim::Limited);
    let narrow = r.claim_auto_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5BundleComponentDowngradeTrigger::LocalOverrideDrift
    );
    assert!(r.claim_is_honest());
}

#[test]
fn ac1_managed_entitlement_narrows_to_limited() {
    let r = row("cert:cli-headless");
    assert_eq!(
        r.binding_group(),
        Some(M5WorkflowBundleComponentGroup::DetailReview)
    );
    assert_eq!(r.effective_claim, M5BundleSupportClaim::Limited);
    let narrow = r.claim_auto_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5BundleComponentDowngradeTrigger::EntitlementDependencyUnmet
    );
    assert!(r.claim_is_honest());
}

#[test]
fn ac1_over_asserting_support_is_blocked() {
    let mut r = row("cert:migration-center");
    // Drop the narrow while keeping the degraded axis: the surface now over-claims.
    r.claim_auto_narrow = None;
    r.effective_claim = M5BundleSupportClaim::Supported;
    assert!(!r.claim_is_honest());
    assert_eq!(r.status(), M5BundleSurfaceCertStatus::Blocked);
}

#[test]
fn ac1_spurious_narrow_on_certified_surface_is_dishonest() {
    let mut r = row("cert:start-center-picker");
    r.effective_claim = M5BundleSupportClaim::Limited;
    r.claim_auto_narrow = narrow(
        M5BundleSupportClaim::Limited,
        M5WorkflowBundleComponentGroup::LaunchWedge,
        "spurious",
    );
    assert!(!r.claim_is_honest());
}

#[test]
fn ac1_effective_claim_may_not_exceed_declared() {
    let mut r = row("cert:docs-help-embeds");
    // Declared drops below the effective claim: the surface now over-asserts.
    r.declared_claim = M5BundleSupportClaim::Limited;
    assert!(!r.claim_is_honest());
}

#[test]
fn ac1_narrow_with_generic_label_is_dishonest() {
    let mut r = row("cert:diagnostics");
    if let Some(n) = r.claim_auto_narrow.as_mut() {
        n.narrowed_label = "degraded".to_owned();
    }
    assert!(!r.claim_is_honest());
}

#[test]
fn ac1_narrow_dropping_identity_is_dishonest() {
    let mut r = row("cert:diagnostics");
    if let Some(n) = r.claim_auto_narrow.as_mut() {
        n.preserves_component_identity = false;
    }
    assert!(!r.claim_is_honest());
}

#[test]
fn ac1_narrow_to_wrong_group_is_dishonest() {
    let mut r = row("cert:diagnostics");
    if let Some(n) = r.claim_auto_narrow.as_mut() {
        n.binding_group = M5WorkflowBundleComponentGroup::DetailReview;
        n.trigger = M5WorkflowBundleComponentGroup::DetailReview.default_trigger();
    }
    assert!(!r.claim_is_honest());
}

#[test]
fn ac2_non_current_path_without_narrowing_is_blocked() {
    let mut r = row("cert:start-center-picker");
    r.compatibility_notes.push(degraded(
        M5BundleDistributionPath::Mirror,
        "Mirror snapshot aged past its freshness budget",
    ));
    // The surface still claims full certification despite a degraded path.
    assert!(!r.unsupported_paths_narrowed());
    assert_eq!(r.status(), M5BundleSurfaceCertStatus::Blocked);
}

#[test]
fn ac2_generic_compatibility_note_is_blocked() {
    let mut r = row("cert:migration-center");
    if let Some(note) = r
        .compatibility_notes
        .iter_mut()
        .find(|c| !c.parity.is_current())
    {
        note.note = "unsupported".to_owned();
    }
    assert!(!r.compatibility_notes_valid());
    assert_eq!(r.status(), M5BundleSurfaceCertStatus::Blocked);
}

#[test]
fn ac2_dropped_export_is_blocked() {
    let mut r = row("cert:start-center-picker");
    r.export_parity = ClaimExportParityState::Dropped;
    assert!(!r.export_preserves_truth());
    assert_eq!(r.status(), M5BundleSurfaceCertStatus::Blocked);
}

#[test]
fn ac2_missing_export_format_is_blocked() {
    let mut r = row("cert:start-center-picker");
    r.copy_export.formats = vec!["text".to_owned()];
    assert!(!r.export_preserves_truth());
}

#[test]
fn ac2_missing_mandatory_export_field_is_blocked() {
    let mut r = row("cert:start-center-picker");
    r.export_fields
        .retain(|f| *f != M5BundleCertExportField::CertificationBundleRef);
    assert!(!r.export_preserves_truth());
}

#[test]
fn ac3_axis_applicable_without_group_is_mismatch() {
    let mut r = row("cert:onboarding-flow");
    // Onboarding does not consume the drift/override group; making its axis
    // applicable breaks the invariant.
    r.drift_override_truth = DriftOverrideTruthState::Certified;
    assert!(!r.axes_match_consumed_groups());
    assert_eq!(r.status(), M5BundleSurfaceCertStatus::Blocked);
}

#[test]
fn ac3_group_consumed_without_axis_is_mismatch() {
    let mut r = row("cert:onboarding-flow");
    // Onboarding consumes detail-review; marking that axis not-applicable breaks it.
    r.detail_review_truth = DetailReviewTruthState::NotApplicable;
    assert!(!r.axes_match_consumed_groups());
}

#[test]
fn ac3_missing_canonical_family_is_blocked() {
    let mut r = row("cert:start-center-picker");
    r.consumer_families
        .retain(|f| *f != M5WorkflowBundleComponentFamily::CertifiedArchetypeBadgeGroup);
    assert!(!r.references_canonical_families());
    assert_eq!(r.status(), M5BundleSurfaceCertStatus::Blocked);
}

#[test]
fn ac3_bundle_ref_mismatch_is_flagged() {
    let mut p = packet();
    p.rows[0].certification_bundle_ref = "artifacts/release/other.json".to_owned();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, BundleSurfaceCertViolation::BundleRefMismatch { .. })));
}

#[test]
fn launch_wedge_group_maps_two_families() {
    let families = M5WorkflowBundleComponentGroup::LaunchWedge.families();
    assert_eq!(families.len(), 2);
    assert!(families.contains(&M5WorkflowBundleComponentFamily::StartCenterBundleCard));
    assert!(families.contains(&M5WorkflowBundleComponentFamily::CertifiedArchetypeBadgeGroup));
}

#[test]
fn every_matrix_family_is_covered_by_a_group() {
    // Every one of the 9 frozen families maps to exactly one component group.
    for family in M5WorkflowBundleComponentFamily::ALL {
        let count = M5WorkflowBundleComponentGroup::ALL
            .iter()
            .filter(|g| g.families().contains(&family))
            .count();
        assert_eq!(count, 1, "family {family:?} mapped {count} times");
    }
}

#[test]
fn missing_surface_coverage_is_flagged() {
    let mut p = packet();
    p.rows
        .retain(|r| r.claimed_surface != M5WorkflowBundleClaimedSurface::ReleaseProof);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, BundleSurfaceCertViolation::MissingSurfaceCoverage { .. })));
    assert!(violations
        .iter()
        .any(|v| matches!(v, BundleSurfaceCertViolation::MissingEvidenceSurface { .. })));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut p = packet();
    let dup = p.rows[0].clone();
    p.rows.push(dup);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, BundleSurfaceCertViolation::DuplicateId { .. })));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut p = packet();
    p.rows[0].source_refs.push("bearer abc123".to_owned());
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, BundleSurfaceCertViolation::RawBoundaryMaterialInExport)));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut p = packet();
    p.summary.green_count += 1;
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, BundleSurfaceCertViolation::SummaryMismatch)));
}

#[test]
fn claim_capability_ranks_are_ordered() {
    assert!(
        M5BundleSupportClaim::Certified.capability_rank()
            > M5BundleSupportClaim::Supported.capability_rank()
    );
    assert!(
        M5BundleSupportClaim::Limited.capability_rank()
            > M5BundleSupportClaim::Imported.capability_rank()
    );
}

#[test]
fn csv_has_a_header_and_one_row_each() {
    let csv = packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet().rows.len());
    assert!(lines[0].starts_with("row_id,claimed_surface,"));
}

#[test]
fn markdown_summary_lists_every_row() {
    let md = packet().render_markdown_summary();
    for row in &packet().rows {
        assert!(md.contains(&row.row_id), "summary missing {}", row.row_id);
    }
}

#[test]
fn chip_tokens_are_deterministic_and_named() {
    let r = row("cert:migration-center");
    let chip = r.chip_tokens();
    assert!(chip.contains("surface=migration_center"));
    assert!(chip.contains("effective=imported"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn export_json_roundtrips() {
    let p = packet();
    let json = p.export_safe_json();
    let back: BundleSurfaceCertPacket = serde_json::from_str(&json).expect("roundtrips");
    assert_eq!(p, back);
}

// --- checked-in artifacts ---

#[test]
fn on_disk_export_matches_builder() {
    let disk = current_m5_bundle_surface_cert_export().expect("checked-in export validates");
    assert_eq!(
        disk,
        seeded_m5_bundle_surface_cert_packet(),
        "checked-in support export drifted from the seeded builder; regenerate the artifact"
    );
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_bundle_surface_cert_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-workflow-bundle-surface-certification-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_bundle_surface_cert_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-workflow-bundle-surface-certification-proof/report.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so
/// it never runs in the normal suite. Run with
/// `GEN_BUNDLE_SURFACE_CERT_ARTIFACTS=1 cargo test -p aureline-workspace generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_BUNDLE_SURFACE_CERT_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_bundle_surface_cert_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-workflow-bundle-surface-certification-proof");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(art.join("report.md"), &report).expect("write report");

    let fixtures =
        Path::new(manifest).join("../../fixtures/ui/m5-workflow-bundle-surface-certification");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
    fs::write(
        fixtures.join("README.md"),
        "# M5 workflow-bundle surface certification fixtures\n\n\
         Mirror of `artifacts/release/m5-workflow-bundle-surface-certification-proof/`.\n\
         Regenerate with `GEN_BUNDLE_SURFACE_CERT_ARTIFACTS=1 cargo test -p aureline-workspace generate_artifacts`.\n",
    )
    .expect("write fixture readme");
}
