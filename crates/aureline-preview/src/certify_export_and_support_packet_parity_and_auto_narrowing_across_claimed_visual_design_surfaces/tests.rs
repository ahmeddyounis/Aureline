//! Tests for the M05-810 visual-designer surface-certification packet.

use super::*;

fn packet() -> VisualDesignSurfaceCertPacket {
    seeded_m5_visual_designer_surface_certification_packet()
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
    assert_eq!(p.record_kind, VISUAL_DESIGNER_SURFACE_CERT_RECORD_KIND);
    assert_eq!(
        p.schema_version,
        VISUAL_DESIGNER_SURFACE_CERT_SCHEMA_VERSION
    );
    assert_eq!(
        p.matrix_ref,
        VISUAL_DESIGNER_SURFACE_CERT_COMPONENT_MATRIX_REF
    );
    assert_eq!(
        p.certification_bundle_ref,
        VISUAL_DESIGNER_SURFACE_CERT_BUNDLE_REF
    );
}

#[test]
fn every_claimed_surface_is_certified() {
    let surfaces = packet().represented_surfaces();
    for surface in M5VisualDesignClaimedSurface::ALL {
        assert!(
            surfaces.contains(&surface),
            "surface {surface:?} not certified"
        );
    }
    assert_eq!(surfaces.len(), M5VisualDesignClaimedSurface::ALL.len());
}

#[test]
fn evidence_surfaces_are_present() {
    assert!(packet().evidence_surfaces_present());
}

#[test]
fn summary_matches_computed() {
    let p = packet();
    assert_eq!(p.summary, p.computed_summary());
}

#[test]
fn seeded_status_split_is_seven_green_three_yellow_zero_red() {
    let p = packet();
    assert_eq!(p.summary.green_count, 7, "green");
    assert_eq!(p.summary.yellow_count, 3, "yellow");
    assert_eq!(p.summary.red_count, 0, "red");
}

#[test]
fn ac1_every_claim_tracks_truth() {
    for row in &packet().rows {
        assert!(
            row.claim_tracks_truth(),
            "{} does not track truth",
            row.row_id
        );
        assert!(!row.hides_drift(), "{} hides drift", row.row_id);
        assert!(!row.over_narrowed(), "{} over-narrows", row.row_id);
    }
}

#[test]
fn ac2_every_export_preserves_truth() {
    for row in &packet().rows {
        assert!(row.export_preserves_truth(), "{} drops truth", row.row_id);
        for field in M5VisualDesignCertExportField::MANDATORY {
            assert!(
                row.preserved_export_fields.contains(&field),
                "{} missing export field {:?}",
                row.row_id,
                field
            );
        }
    }
}

#[test]
fn ac3_every_narrowing_is_disclosed() {
    for row in &packet().rows {
        assert!(row.narrowing_disclosed(), "{} narrows silently", row.row_id);
    }
}

#[test]
fn narrowed_rows_carry_a_matching_auto_narrow() {
    for row in &packet().rows {
        let narrowed = row.effective_claim.capability_rank() < row.declared_claim.capability_rank();
        if narrowed {
            let narrow = row
                .auto_narrow
                .as_ref()
                .unwrap_or_else(|| panic!("{} narrowed but has no auto-narrow", row.row_id));
            assert!(narrow.is_honest(), "{} auto-narrow dishonest", row.row_id);
            assert_eq!(narrow.narrowed_from, row.declared_claim, "{}", row.row_id);
            assert_eq!(narrow.narrowed_to, row.effective_claim, "{}", row.row_id);
            assert_eq!(
                narrow.weakened_dimension,
                row.binding_dimension(),
                "{}",
                row.row_id
            );
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
fn approximate_mapping_narrows_writable_to_inspect_only() {
    let row = packet()
        .rows
        .into_iter()
        .find(|r| r.claimed_surface == M5VisualDesignClaimedSurface::SourceRoundTripRail)
        .expect("source round-trip rail row");
    assert_eq!(
        row.declared_claim,
        M5VisualDesignClaimTier::FullyInteractiveWritable
    );
    assert_eq!(row.effective_claim, M5VisualDesignClaimTier::InspectOnly);
    assert_eq!(
        row.binding_dimension(),
        M5VisualDesignTruthDimension::MappingQuality
    );
    assert_eq!(row.status(), SurfaceCertStatus::NarrowedDisclosed);
}

#[test]
fn stale_runtime_narrows_writable_to_read_only() {
    let row = packet()
        .rows
        .into_iter()
        .find(|r| r.claimed_surface == M5VisualDesignClaimedSurface::BreakpointDevicePreviewDeck)
        .expect("breakpoint deck row");
    assert_eq!(row.effective_claim, M5VisualDesignClaimTier::ReadOnly);
    assert_eq!(
        row.binding_dimension(),
        M5VisualDesignTruthDimension::PreviewRuntimeFreshness
    );
}

#[test]
fn a_surface_hiding_drift_is_blocked() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.claimed_surface == M5VisualDesignClaimedSurface::SourceRoundTripRail)
        .expect("source round-trip rail row");
    // The mapping is only approximate, but the surface keeps a fully-writable claim.
    row.effective_claim = M5VisualDesignClaimTier::FullyInteractiveWritable;
    assert!(row.hides_drift());
    assert_eq!(row.status(), SurfaceCertStatus::Blocked);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, VisualDesignSurfaceCertViolation::ClaimHidesDrift { .. })));
    assert!(violations
        .iter()
        .any(|v| matches!(v, VisualDesignSurfaceCertViolation::BlockedRow { .. })));
}

#[test]
fn unmapped_surface_cannot_claim_writable() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.claimed_surface == M5VisualDesignClaimedSurface::DesignCanvasWorkspace)
        .expect("design canvas row");
    row.mapping_quality = M5BreakpointMappingQuality::Unmapped;
    // Declared claim stays fully writable; the unmapped source can only support
    // source-only, so the row now hides drift.
    assert_eq!(
        row.overall_supported_ceiling(),
        M5VisualDesignClaimTier::SourceOnly
    );
    assert!(row.hides_drift());
}

#[test]
fn dropping_an_export_field_blocks_the_row() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.preserved_export_fields
        .retain(|f| *f != M5VisualDesignCertExportField::MappingQuality);
    assert!(!row.export_preserves_truth());
    assert_eq!(row.status(), SurfaceCertStatus::Blocked);
    p.summary = p.computed_summary();
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, VisualDesignSurfaceCertViolation::ExportDropsTruth { .. })));
}

#[test]
fn screenshot_only_export_blocks_the_row() {
    let mut p = packet();
    p.rows[0].copy_export.formats = vec!["screenshot".to_owned()];
    assert!(!p.rows[0].export_preserves_truth());
    p.summary = p.computed_summary();
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, VisualDesignSurfaceCertViolation::ExportDropsTruth { .. })));
}

#[test]
fn spurious_auto_narrow_on_a_green_row_is_rejected() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.claimed_surface == M5VisualDesignClaimedSurface::DesignCanvasWorkspace)
        .expect("design canvas row");
    row.auto_narrow = Some(ClaimAutoNarrow {
        trigger: M5VisualDesignerDowngradeTrigger::RuntimeUnavailable,
        weakened_dimension: M5VisualDesignTruthDimension::PreviewRuntimeFreshness,
        narrowed_from: M5VisualDesignClaimTier::FullyInteractiveWritable,
        narrowed_to: M5VisualDesignClaimTier::InspectOnly,
        reason_label: "spurious".to_owned(),
        preserves_source_truth: true,
    });
    assert!(!row.narrowing_disclosed());
    assert_eq!(row.status(), SurfaceCertStatus::Blocked);
}

#[test]
fn narrowed_row_missing_auto_narrow_is_rejected() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.auto_narrow.is_some())
        .expect("a narrowed row");
    row.auto_narrow = None;
    assert!(!row.narrowing_disclosed());
}

#[test]
fn generic_auto_narrow_reason_is_rejected() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.auto_narrow.is_some())
        .expect("a narrowed row");
    row.auto_narrow.as_mut().unwrap().reason_label = "degraded".to_owned();
    assert!(!row.narrowing_disclosed());
}

#[test]
fn auto_narrow_dropping_source_truth_is_rejected() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.auto_narrow.is_some())
        .expect("a narrowed row");
    row.auto_narrow.as_mut().unwrap().preserves_source_truth = false;
    assert!(!row.narrowing_disclosed());
}

#[test]
fn wrong_dimension_on_auto_narrow_is_rejected() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.claimed_surface == M5VisualDesignClaimedSurface::SourceRoundTripRail)
        .expect("source round-trip rail row");
    row.auto_narrow.as_mut().unwrap().weakened_dimension =
        M5VisualDesignTruthDimension::RoundTripSupport;
    assert!(!row.narrowing_disclosed());
}

#[test]
fn bundle_ref_mismatch_is_flagged() {
    let mut p = packet();
    p.rows[0].certification_bundle_ref = "artifacts/other/bundle.json".to_owned();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignSurfaceCertViolation::BundleRefMismatch { .. }
    )));
}

#[test]
fn missing_surface_coverage_is_flagged() {
    let mut p = packet();
    p.rows
        .retain(|r| r.claimed_surface != M5VisualDesignClaimedSurface::ReleaseProof);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualDesignSurfaceCertViolation::MissingSurfaceCoverage { .. }
    )));
    assert!(violations
        .iter()
        .any(|v| matches!(v, VisualDesignSurfaceCertViolation::MissingEvidenceSurface)));
}

#[test]
fn missing_label_coverage_is_flagged() {
    let mut p = packet();
    for row in &mut p.rows {
        row.required_labels
            .retain(|l| *l != M5VisualDesignerRequiredLabel::KeyboardRoute);
    }
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignSurfaceCertViolation::MissingLabelCoverage { .. }
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
        .any(|v| matches!(v, VisualDesignSurfaceCertViolation::DuplicateId { .. })));
}

#[test]
fn single_consumer_surface_is_flagged() {
    let mut p = packet();
    p.rows[0].consumer_surfaces.truncate(1);
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignSurfaceCertViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut p = packet();
    p.summary.green_count += 1;
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, VisualDesignSurfaceCertViolation::SummaryMismatch)));
}

#[test]
fn forbidden_boundary_material_is_flagged() {
    let mut p = packet();
    p.rows[0].source_refs.push("bearer abc123".to_owned());
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignSurfaceCertViolation::RawBoundaryMaterialInExport
    )));
}

#[test]
fn on_disk_export_matches_builder() {
    let disk = current_m5_visual_designer_surface_certification_export()
        .expect("checked-in export must parse and validate");
    assert_eq!(disk, packet(), "on-disk export drifted from the builder");
}

#[test]
fn csv_has_a_row_per_surface() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    assert_eq!(lines, packet().rows.len() + 1);
    assert!(csv.contains("design_canvas_workspace"));
    assert!(csv.contains("source_round_trip_rail"));
}

#[test]
fn markdown_summary_names_every_surface() {
    let md = packet().render_markdown_summary();
    for surface in M5VisualDesignClaimedSurface::ALL {
        assert!(
            md.contains(surface.as_str()),
            "missing {}",
            surface.as_str()
        );
    }
}

#[test]
fn export_is_deterministic() {
    assert_eq!(packet().export_safe_json(), packet().export_safe_json());
}
