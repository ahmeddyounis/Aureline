//! Tests for the M05-811 visual-designer component qualification packet.

use super::*;

fn packet() -> VisualDesignerQualificationPacket {
    seeded_m5_visual_designer_component_qualification_packet()
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
    assert_eq!(p.record_kind, VISUAL_DESIGNER_QUALIFICATION_RECORD_KIND);
    assert_eq!(
        p.schema_version,
        VISUAL_DESIGNER_QUALIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        p.matrix_ref,
        VISUAL_DESIGNER_QUALIFICATION_COMPONENT_MATRIX_REF
    );
    assert_eq!(
        p.certification_bundle_ref,
        VISUAL_DESIGNER_QUALIFICATION_BUNDLE_REF
    );
}

#[test]
fn every_claimed_consumer_is_qualified() {
    let consumers = packet().represented_consumers();
    for consumer in M5QualifiedComponentConsumer::ALL {
        assert!(
            consumers.contains(&consumer),
            "consumer {consumer:?} not qualified"
        );
    }
    assert_eq!(consumers.len(), M5QualifiedComponentConsumer::ALL.len());
}

#[test]
fn evidence_consumers_are_present() {
    assert!(packet().evidence_consumers_present());
}

#[test]
fn bundle_consolidates_every_canonical_packet() {
    let p = packet();
    for canonical in canonical_component_packet_refs() {
        assert!(
            p.certified_component_packets.contains(&canonical),
            "bundle missing consolidated packet {canonical}"
        );
    }
    assert_eq!(p.certified_component_packets.len(), 7);
}

#[test]
fn every_row_covers_all_five_dimensions() {
    for row in &packet().rows {
        assert!(
            row.covers_all_dimensions(),
            "{} does not cover all dimensions",
            row.row_id
        );
        assert_eq!(
            row.dimensions.len(),
            M5ComponentQualificationDimension::ALL.len()
        );
    }
}

#[test]
fn summary_matches_computed() {
    let p = packet();
    assert_eq!(p.summary, p.computed_summary());
}

#[test]
fn seeded_status_split_is_five_green_three_yellow_zero_red() {
    let p = packet();
    assert_eq!(p.summary.green_count, 5, "green");
    assert_eq!(p.summary.yellow_count, 3, "yellow");
    assert_eq!(p.summary.red_count, 0, "red");
}

#[test]
fn ac1_every_consumer_uses_shared_components_and_tracks_truth() {
    for row in &packet().rows {
        assert!(
            row.uses_shared_components,
            "{} forks components",
            row.row_id
        );
        assert!(!row.hides_drift(), "{} hides drift", row.row_id);
        assert!(row.dimensions_honest(), "{} dishonest", row.row_id);
    }
}

#[test]
fn ac2_every_row_cites_the_one_bundle_and_consolidated_packets() {
    let p = packet();
    for row in &p.rows {
        assert_eq!(
            row.certification_bundle_ref, p.certification_bundle_ref,
            "{}",
            row.row_id
        );
        assert!(!row.canonical_component_refs.is_empty(), "{}", row.row_id);
        for component_ref in &row.canonical_component_refs {
            assert!(
                p.certified_component_packets.contains(component_ref),
                "{} cites uncited packet {component_ref}",
                row.row_id
            );
        }
    }
}

#[test]
fn ac3_every_export_preserves_truth() {
    for row in &packet().rows {
        assert!(row.export_preserves_truth(), "{} drops truth", row.row_id);
        assert!(
            row.narrowed_reason_exported(),
            "{} drops reason",
            row.row_id
        );
        for field in M5QualificationExportField::MANDATORY {
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
fn approximate_mapping_narrows_the_mapping_dimension() {
    let row = packet()
        .rows
        .into_iter()
        .find(|r| r.consumer == M5QualifiedComponentConsumer::PreviewRuntime)
        .expect("preview runtime row");
    let mapping = row
        .dimension(M5ComponentQualificationDimension::MappingQuality)
        .expect("mapping dimension");
    assert_eq!(mapping.state, AxisCertificationState::DisclosedNarrowed);
    assert_eq!(
        mapping.trigger,
        Some(M5VisualDesignerDowngradeTrigger::UnmappedSource)
    );
    assert_eq!(
        row.verdict(),
        M5ComponentQualificationVerdict::QualifiedWithNarrowing
    );
}

#[test]
fn round_trip_conflict_narrows_the_round_trip_dimension() {
    let row = packet()
        .rows
        .into_iter()
        .find(|r| r.consumer == M5QualifiedComponentConsumer::FrameworkPackPreview)
        .expect("framework-pack row");
    let dim = row
        .dimension(M5ComponentQualificationDimension::RoundTripState)
        .expect("round-trip dimension");
    assert_eq!(dim.state, AxisCertificationState::DisclosedNarrowed);
    assert_eq!(
        dim.trigger,
        Some(M5VisualDesignerDowngradeTrigger::RoundTripConflictOpen)
    );
}

#[test]
fn binding_drift_narrows_the_token_binding_dimension() {
    let row = packet()
        .rows
        .into_iter()
        .find(|r| r.consumer == M5QualifiedComponentConsumer::HandoffConsumer)
        .expect("handoff row");
    let dim = row
        .dimension(M5ComponentQualificationDimension::TokenBindingProvenance)
        .expect("token/binding dimension");
    assert_eq!(dim.state, AxisCertificationState::DisclosedNarrowed);
}

#[test]
fn a_dimension_hiding_drift_blocks_the_row() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.consumer == M5QualifiedComponentConsumer::VisualDesignSurface)
        .expect("visual design surface row");
    let dim = row
        .dimensions
        .iter_mut()
        .find(|d| d.dimension == M5ComponentQualificationDimension::SourceOwnership)
        .expect("source ownership dimension");
    dim.state = AxisCertificationState::UndisclosedDrift;
    assert!(row.hides_drift());
    assert_eq!(row.verdict(), M5ComponentQualificationVerdict::Blocked);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualDesignerQualificationViolation::DimensionHidesDrift { .. }
    )));
    assert!(violations
        .iter()
        .any(|v| matches!(v, VisualDesignerQualificationViolation::BlockedRow { .. })));
}

#[test]
fn a_consumer_forking_components_is_blocked() {
    let mut p = packet();
    p.rows[0].uses_shared_components = false;
    assert_eq!(
        p.rows[0].verdict(),
        M5ComponentQualificationVerdict::Blocked
    );
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerQualificationViolation::SharedComponentsNotUsed { .. }
    )));
}

#[test]
fn a_narrowed_dimension_without_trigger_is_dishonest() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.consumer == M5QualifiedComponentConsumer::PreviewRuntime)
        .expect("preview runtime row");
    row.dimensions
        .iter_mut()
        .find(|d| d.state == AxisCertificationState::DisclosedNarrowed)
        .expect("narrowed dimension")
        .trigger = None;
    assert!(!row.dimensions_honest());
    assert_eq!(row.verdict(), M5ComponentQualificationVerdict::Blocked);
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerQualificationViolation::DishonestNarrowing { .. }
    )));
}

#[test]
fn a_generic_narrow_reason_is_dishonest() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.consumer == M5QualifiedComponentConsumer::PreviewRuntime)
        .expect("preview runtime row");
    row.dimensions
        .iter_mut()
        .find(|d| d.state == AxisCertificationState::DisclosedNarrowed)
        .expect("narrowed dimension")
        .reason_label = "degraded".to_owned();
    assert!(!row.dimensions_honest());
}

#[test]
fn a_certified_dimension_with_a_trigger_is_dishonest() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.dimensions
        .iter_mut()
        .find(|d| d.dimension == M5ComponentQualificationDimension::SourceOwnership)
        .expect("source ownership dimension")
        .trigger = Some(M5VisualDesignerDowngradeTrigger::DriftedFromSource);
    assert!(!row.dimensions_honest());
}

#[test]
fn dropping_an_export_field_blocks_the_row() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.preserved_export_fields
        .retain(|f| *f != M5QualificationExportField::MappingQuality);
    assert!(!row.export_preserves_truth());
    assert_eq!(row.verdict(), M5ComponentQualificationVerdict::Blocked);
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerQualificationViolation::ExportDropsTruth { .. }
    )));
}

#[test]
fn a_narrowed_row_dropping_the_reason_field_is_blocked() {
    let mut p = packet();
    let row = p
        .rows
        .iter_mut()
        .find(|r| r.is_narrowed())
        .expect("a narrowed row");
    row.preserved_export_fields
        .retain(|f| *f != M5QualificationExportField::NarrowedReason);
    assert!(!row.narrowed_reason_exported());
    assert_eq!(row.verdict(), M5ComponentQualificationVerdict::Blocked);
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerQualificationViolation::NarrowedReasonNotExported { .. }
    )));
}

#[test]
fn screenshot_only_export_blocks_the_row() {
    let mut p = packet();
    p.rows[0].copy_export.formats = vec!["screenshot".to_owned()];
    assert!(!p.rows[0].export_preserves_truth());
    p.summary = p.computed_summary();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerQualificationViolation::ExportDropsTruth { .. }
    )));
}

#[test]
fn bundle_missing_a_consolidated_packet_is_flagged() {
    let mut p = packet();
    let matrix = VISUAL_DESIGNER_COMPONENT_MATRIX_ARTIFACT_REF.to_owned();
    p.certified_component_packets.retain(|r| *r != matrix);
    // Rows still cite the removed packet, so both the consolidation and the
    // citation checks fire.
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualDesignerQualificationViolation::MissingConsolidatedPacket { .. }
    )));
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualDesignerQualificationViolation::UncitedComponentPacket { .. }
    )));
}

#[test]
fn a_row_citing_an_uncited_packet_is_flagged() {
    let mut p = packet();
    p.rows[0]
        .canonical_component_refs
        .push("artifacts/release/not-in-bundle/support_export.json".to_owned());
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerQualificationViolation::UncitedComponentPacket { .. }
    )));
}

#[test]
fn bundle_ref_mismatch_is_flagged() {
    let mut p = packet();
    p.rows[0].certification_bundle_ref = "artifacts/other/bundle.json".to_owned();
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerQualificationViolation::BundleRefMismatch { .. }
    )));
}

#[test]
fn missing_consumer_coverage_is_flagged() {
    let mut p = packet();
    p.rows
        .retain(|r| r.consumer != M5QualifiedComponentConsumer::ReleaseEvidence);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualDesignerQualificationViolation::MissingConsumerCoverage { .. }
    )));
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualDesignerQualificationViolation::MissingEvidenceConsumer
    )));
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
        VisualDesignerQualificationViolation::MissingLabelCoverage { .. }
    )));
}

#[test]
fn missing_dimension_coverage_is_flagged() {
    let mut p = packet();
    p.rows[0]
        .dimensions
        .retain(|d| d.dimension != M5ComponentQualificationDimension::AccessibilityExportBehavior);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualDesignerQualificationViolation::MissingDimensionCoverage { .. }
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
        .any(|v| matches!(v, VisualDesignerQualificationViolation::DuplicateId { .. })));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut p = packet();
    p.summary.green_count += 1;
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, VisualDesignerQualificationViolation::SummaryMismatch)));
}

#[test]
fn forbidden_boundary_material_is_flagged() {
    let mut p = packet();
    p.rows[0].source_refs.push("bearer abc123".to_owned());
    assert!(p.validate().iter().any(|v| matches!(
        v,
        VisualDesignerQualificationViolation::RawBoundaryMaterialInExport
    )));
}

#[test]
fn on_disk_export_matches_builder() {
    let disk = current_m5_visual_designer_component_qualification_export()
        .expect("checked-in export must parse and validate");
    assert_eq!(disk, packet(), "on-disk export drifted from the builder");
}

#[test]
fn csv_has_a_row_per_consumer() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    assert_eq!(lines, packet().rows.len() + 1);
    assert!(csv.contains("visual_design_surface"));
    assert!(csv.contains("preview_runtime"));
}

#[test]
fn markdown_summary_names_every_consumer() {
    let md = packet().render_markdown_summary();
    for consumer in M5QualifiedComponentConsumer::ALL {
        assert!(
            md.contains(consumer.as_str()),
            "missing {}",
            consumer.as_str()
        );
    }
}

#[test]
fn export_is_deterministic() {
    assert_eq!(packet().export_safe_json(), packet().export_safe_json());
}
