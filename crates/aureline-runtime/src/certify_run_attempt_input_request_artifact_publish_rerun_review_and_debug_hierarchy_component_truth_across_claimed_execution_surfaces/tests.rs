//! Tests for the M05-827 execution-lifecycle component surface certification packet.

use super::*;

fn packet() -> ExecutionSurfaceCertPacket {
    seeded_m5_execution_surface_cert_packet()
}

fn row(id: &str) -> ExecutionSurfaceCertRow {
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
    assert_eq!(p.record_kind, EXECUTION_SURFACE_CERT_RECORD_KIND);
    assert_eq!(p.schema_version, EXECUTION_SURFACE_CERT_SCHEMA_VERSION);
    assert_eq!(p.matrix_ref, EXECUTION_SURFACE_CERT_COMPONENT_MATRIX_REF);
    assert_eq!(
        p.certification_bundle_ref,
        EXECUTION_SURFACE_CERT_BUNDLE_REF
    );
}

#[test]
fn every_claimed_surface_is_certified() {
    let surfaces = packet().represented_surfaces();
    for surface in M5ExecutionClaimedSurface::ALL {
        assert!(
            surfaces.contains(&surface),
            "surface {surface:?} not certified"
        );
    }
    assert_eq!(surfaces.len(), M5ExecutionClaimedSurface::ALL.len());
}

#[test]
fn every_component_group_is_consumed() {
    let consumed = packet().consumed_groups();
    for group in M5ExecutionComponentGroup::ALL {
        assert!(consumed.contains(&group), "group {group:?} never consumed");
    }
}

#[test]
fn every_path_class_is_exercised() {
    let paths = packet().covered_path_classes();
    for path in M5ExecutionPathClass::ALL {
        assert!(paths.contains(&path), "path class {path:?} never exercised");
    }
}

#[test]
fn evidence_surfaces_are_present() {
    let surfaces = packet().represented_surfaces();
    for surface in M5ExecutionClaimedSurface::EVIDENCE_SURFACES {
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
fn seeded_status_split_is_seven_green_five_yellow_zero_red() {
    let p = packet();
    assert_eq!(p.summary.green_count, 7, "green");
    assert_eq!(p.summary.yellow_count, 5, "yellow");
    assert_eq!(p.summary.red_count, 0, "red");
}

#[test]
fn coverage_flags_are_complete() {
    let s = packet().summary;
    assert!(s.group_coverage_complete);
    assert!(s.path_class_coverage_complete);
    assert!(s.all_claims_honest);
    assert!(s.all_export_preserve_truth);
    assert!(s.all_unsupported_paths_narrowed);
}

#[test]
fn ac1_certified_surface_asserts_declared_claim_without_narrowing() {
    let r = row("cert:task-execution");
    assert!(!r.claim_narrowed());
    assert!(r.claim_auto_narrow.is_none());
    assert_eq!(r.declared_claim, r.effective_claim);
    assert!(r.claim_is_honest());
    assert_eq!(r.status(), M5ExecutionSurfaceCertStatus::Certified);
}

#[test]
fn ac1_degraded_artifact_narrows_to_read_only() {
    let r = row("cert:database-execution");
    assert!(r.claim_narrowed());
    assert_eq!(r.effective_claim, M5ExecutionInteractiveClaim::ReadOnly);
    assert_eq!(
        r.binding_group(),
        Some(M5ExecutionComponentGroup::ArtifactPublish)
    );
    let narrow = r.claim_auto_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ExecutionDowngradeTrigger::ArtifactRetentionExpired
    );
    assert!(r.claim_is_honest());
    assert_eq!(r.status(), M5ExecutionSurfaceCertStatus::NarrowedDisclosed);
}

#[test]
fn ac1_provider_input_narrows_to_review_required() {
    let r = row("cert:ai-execution");
    assert_eq!(
        r.binding_group(),
        Some(M5ExecutionComponentGroup::InputRequest)
    );
    assert_eq!(
        r.effective_claim,
        M5ExecutionInteractiveClaim::ReviewRequired
    );
    assert!(r.claim_is_honest());
}

#[test]
fn ac1_debug_connector_loss_narrows_to_inspect_only() {
    let r = row("cert:debug-execution");
    assert_eq!(
        r.binding_group(),
        Some(M5ExecutionComponentGroup::DebugHierarchy)
    );
    assert_eq!(r.effective_claim, M5ExecutionInteractiveClaim::InspectOnly);
    let narrow = r.claim_auto_narrow.as_ref().expect("narrow present");
    assert_eq!(narrow.trigger, M5ExecutionDowngradeTrigger::ConnectorLost);
    assert!(r.claim_is_honest());
}

#[test]
fn ac1_over_asserting_control_is_blocked() {
    let mut r = row("cert:database-execution");
    // Drop the narrow while keeping the degraded axis: the surface now over-claims.
    r.claim_auto_narrow = None;
    r.effective_claim = M5ExecutionInteractiveClaim::FullInteractive;
    assert!(!r.claim_is_honest());
    assert_eq!(r.status(), M5ExecutionSurfaceCertStatus::Blocked);
}

#[test]
fn ac1_spurious_narrow_on_certified_surface_is_dishonest() {
    let mut r = row("cert:task-execution");
    r.effective_claim = M5ExecutionInteractiveClaim::ReadOnly;
    r.claim_auto_narrow = narrow(
        M5ExecutionInteractiveClaim::ReadOnly,
        M5ExecutionComponentGroup::RunAttempt,
        "spurious",
    );
    assert!(!r.claim_is_honest());
}

#[test]
fn ac1_effective_claim_may_not_exceed_declared() {
    let mut r = row("cert:docs-help-embeds");
    r.effective_claim = M5ExecutionInteractiveClaim::FullInteractive;
    assert!(!r.claim_is_honest());
}

#[test]
fn ac1_narrow_with_generic_label_is_dishonest() {
    let mut r = row("cert:debug-execution");
    if let Some(n) = r.claim_auto_narrow.as_mut() {
        n.narrowed_label = "degraded".to_owned();
    }
    assert!(!r.claim_is_honest());
}

#[test]
fn ac1_narrow_dropping_identity_is_dishonest() {
    let mut r = row("cert:debug-execution");
    if let Some(n) = r.claim_auto_narrow.as_mut() {
        n.preserves_component_identity = false;
    }
    assert!(!r.claim_is_honest());
}

#[test]
fn ac1_narrow_to_wrong_group_is_dishonest() {
    let mut r = row("cert:database-execution");
    if let Some(n) = r.claim_auto_narrow.as_mut() {
        n.binding_group = M5ExecutionComponentGroup::RunAttempt;
        n.trigger = M5ExecutionComponentGroup::RunAttempt.default_trigger();
    }
    assert!(!r.claim_is_honest());
}

#[test]
fn ac2_non_current_path_without_narrowing_is_blocked() {
    let mut r = row("cert:task-execution");
    r.compatibility_notes.push(degraded(
        M5ExecutionPathClass::Managed,
        "Managed path retention degraded",
    ));
    // The surface still claims full control despite a degraded path.
    assert!(!r.unsupported_paths_narrowed());
    assert_eq!(r.status(), M5ExecutionSurfaceCertStatus::Blocked);
}

#[test]
fn ac2_generic_compatibility_note_is_blocked() {
    let mut r = row("cert:database-execution");
    if let Some(note) = r
        .compatibility_notes
        .iter_mut()
        .find(|c| !c.parity.is_current())
    {
        note.note = "unsupported".to_owned();
    }
    assert!(!r.compatibility_notes_valid());
    assert_eq!(r.status(), M5ExecutionSurfaceCertStatus::Blocked);
}

#[test]
fn ac2_dropped_export_is_blocked() {
    let mut r = row("cert:task-execution");
    r.export_parity = ClaimExportParityState::Dropped;
    assert!(!r.export_preserves_truth());
    assert_eq!(r.status(), M5ExecutionSurfaceCertStatus::Blocked);
}

#[test]
fn ac2_missing_export_format_is_blocked() {
    let mut r = row("cert:task-execution");
    r.copy_export.formats = vec!["text".to_owned()];
    assert!(!r.export_preserves_truth());
}

#[test]
fn ac2_missing_mandatory_export_field_is_blocked() {
    let mut r = row("cert:task-execution");
    r.export_fields
        .retain(|f| *f != M5ExecutionCertExportField::CertificationBundleRef);
    assert!(!r.export_preserves_truth());
}

#[test]
fn ac3_axis_applicable_without_group_is_mismatch() {
    let mut r = row("cert:preview-execution");
    // Preview does not consume the debug-hierarchy group; making its axis applicable
    // breaks the invariant.
    r.debug_hierarchy_truth = DebugHierarchyTruthState::Certified;
    assert!(!r.axes_match_consumed_groups());
    assert_eq!(r.status(), M5ExecutionSurfaceCertStatus::Blocked);
}

#[test]
fn ac3_group_consumed_without_axis_is_mismatch() {
    let mut r = row("cert:preview-execution");
    // Preview consumes artifact-publish; marking that axis not-applicable breaks it.
    r.artifact_publish_truth = ArtifactPublishTruthState::NotApplicable;
    assert!(!r.axes_match_consumed_groups());
}

#[test]
fn ac3_missing_canonical_family_is_blocked() {
    let mut r = row("cert:task-execution");
    r.consumer_families
        .retain(|f| *f != M5ExecutionComponentFamily::ThreadProcessTree);
    assert!(!r.references_canonical_families());
    assert_eq!(r.status(), M5ExecutionSurfaceCertStatus::Blocked);
}

#[test]
fn ac3_bundle_ref_mismatch_is_flagged() {
    let mut p = packet();
    p.rows[0].certification_bundle_ref = "artifacts/release/other.json".to_owned();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ExecutionSurfaceCertViolation::BundleRefMismatch { .. })));
}

#[test]
fn debug_hierarchy_group_maps_three_families() {
    let families = M5ExecutionComponentGroup::DebugHierarchy.families();
    assert_eq!(families.len(), 3);
    assert!(families.contains(&M5ExecutionComponentFamily::DebugSessionHeader));
    assert!(families.contains(&M5ExecutionComponentFamily::ThreadProcessTree));
    assert!(families.contains(&M5ExecutionComponentFamily::DumpCrashArtifactCard));
}

#[test]
fn missing_surface_coverage_is_flagged() {
    let mut p = packet();
    p.rows
        .retain(|r| r.claimed_surface != M5ExecutionClaimedSurface::ReleaseProof);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ExecutionSurfaceCertViolation::MissingSurfaceCoverage { .. }
    )));
    assert!(violations.iter().any(|v| matches!(
        v,
        ExecutionSurfaceCertViolation::MissingEvidenceSurface { .. }
    )));
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
        .any(|v| matches!(v, ExecutionSurfaceCertViolation::DuplicateId { .. })));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut p = packet();
    p.rows[0].source_refs.push("bearer abc123".to_owned());
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ExecutionSurfaceCertViolation::RawBoundaryMaterialInExport
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut p = packet();
    p.summary.green_count += 1;
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ExecutionSurfaceCertViolation::SummaryMismatch)));
}

#[test]
fn claim_capability_ranks_are_ordered() {
    assert!(
        M5ExecutionInteractiveClaim::FullInteractive.capability_rank()
            > M5ExecutionInteractiveClaim::ReviewRequired.capability_rank()
    );
    assert!(
        M5ExecutionInteractiveClaim::ReadOnly.capability_rank()
            > M5ExecutionInteractiveClaim::InspectOnly.capability_rank()
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
    let r = row("cert:database-execution");
    let chip = r.chip_tokens();
    assert!(chip.contains("surface=database_execution"));
    assert!(chip.contains("effective=read_only"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn export_json_roundtrips() {
    let p = packet();
    let json = p.export_safe_json();
    let back: ExecutionSurfaceCertPacket = serde_json::from_str(&json).expect("roundtrips");
    assert_eq!(p, back);
}

#[test]
fn on_disk_export_matches_builder() {
    let disk = current_m5_execution_surface_cert_export().expect("checked-in export validates");
    assert_eq!(
        disk,
        seeded_m5_execution_surface_cert_packet(),
        "checked-in support export drifted from the seeded builder; regenerate the artifact"
    );
}
