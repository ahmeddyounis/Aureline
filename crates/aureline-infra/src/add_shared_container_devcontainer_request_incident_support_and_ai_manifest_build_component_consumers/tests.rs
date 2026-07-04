//! Tests for the M05-817 manifest / build component consumer lane.

use super::*;

fn packet() -> ManifestBuildConsumerPacket {
    seeded_m5_manifest_build_consumer_packet()
}

#[test]
fn seeded_packet_is_valid() {
    let violations = packet().validate();
    assert!(violations.is_empty(), "unexpected violations: {violations:?}");
}

#[test]
fn all_four_consumer_groups_present() {
    let p = packet();
    for group in ConsumerGroup::ALL {
        assert!(
            p.rows.iter().any(|r| r.consumer_group == group),
            "missing consumer group {group:?}"
        );
    }
    assert_eq!(p.summary.consumer_group_count, ConsumerGroup::ALL.len());
}

#[test]
fn all_ten_component_families_adopted() {
    let p = packet();
    for family in M5ManifestBuildComponentFamily::ALL {
        assert!(
            p.rows.iter().any(|r| r.component_family == family),
            "missing component family {family:?}"
        );
    }
    assert_eq!(
        p.summary.component_family_count,
        M5ManifestBuildComponentFamily::ALL.len()
    );
}

#[test]
fn every_row_points_to_one_canonical_family() {
    for row in packet().rows {
        assert!(
            row.points_to_canonical_family(),
            "row {} does not point to its canonical family",
            row.row_id
        );
        assert_eq!(
            row.canonical_family_schema_ref,
            canonical_schema_ref_for(row.component_family)
        );
    }
}

#[test]
fn a_family_is_reused_across_groups() {
    assert!(packet().has_family_reused_across_groups());
}

#[test]
fn every_row_preserves_target_context_and_labels() {
    for row in packet().rows {
        assert!(row.preserves_target_context(), "row {} dropped target context", row.row_id);
        assert!(row.preserves_labels(), "row {} broke label parity", row.row_id);
    }
}

#[test]
fn narrowed_rows_carry_matching_banner() {
    for row in packet().rows {
        assert!(row.discloses_narrowing(), "row {} narrowing not disclosed", row.row_id);
        if row.is_narrowed() {
            let banner = row
                .reduced_capability_banner
                .as_ref()
                .expect("narrowed row has banner");
            assert_eq!(banner.capability_state, row.authority_mode.capability_state());
            assert_eq!(row.label_parity, LabelParityState::DisclosedNarrowed);
        } else {
            assert!(row.reduced_capability_banner.is_none());
            assert_eq!(row.label_parity, LabelParityState::Preserved);
        }
    }
}

#[test]
fn handoff_rows_carry_a_note() {
    for row in packet().rows {
        if row.handoff_target.requires_note() {
            assert!(
                !row.handoff_note_ref.trim().is_empty(),
                "row {} handoff without note",
                row.row_id
            );
        }
    }
}

#[test]
fn adapter_source_and_confidence_never_contradict() {
    for row in packet().rows {
        assert!(
            row.confidence_consistent(),
            "row {} claims a confidence its adapter cannot support",
            row.row_id
        );
    }
}

#[test]
fn help_support_release_surfaces_reference_canonical() {
    let p = packet();
    assert!(p.summary.help_support_release_reference_present);
    for surface in [
        M5ManifestBuildConsumerSurface::DocsHelp,
        M5ManifestBuildConsumerSurface::SupportExport,
        M5ManifestBuildConsumerSurface::ReleaseProof,
    ] {
        let row = p
            .rows
            .iter()
            .find(|r| r.consumer_surface == surface)
            .unwrap_or_else(|| panic!("missing {surface:?} row"));
        assert!(row.references_canonical_not_local_prose);
        assert!(row.points_to_canonical_family());
    }
}

#[test]
fn every_surface_matches_its_declared_group() {
    for row in packet().rows {
        assert_eq!(row.consumer_surface.consumer_group(), row.consumer_group);
    }
}

#[test]
fn copy_export_is_screenshot_safe() {
    for row in packet().rows {
        assert!(row.copy_export.is_export_safe(), "row {} not export-safe", row.row_id);
    }
}

#[test]
fn detects_broken_canonical_reference() {
    let mut p = packet();
    p.rows[0].canonical_family_schema_ref = "schemas/ui/made-up.schema.json".to_owned();
    p.summary = p.computed_summary();
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, ManifestBuildConsumerViolation::NotCanonicalFamily { .. })));
}

#[test]
fn detects_dropped_target_context() {
    let mut p = packet();
    p.rows[0].target_context_ref = "  ".to_owned();
    p.summary = p.computed_summary();
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, ManifestBuildConsumerViolation::TargetContextDropped { .. })));
}

#[test]
fn detects_undisclosed_narrowing() {
    let mut p = packet();
    // Find a narrowed row and strip its banner.
    let idx = p
        .rows
        .iter()
        .position(ManifestBuildConsumerRow::is_narrowed)
        .expect("a narrowed row");
    p.rows[idx].reduced_capability_banner = None;
    p.summary = p.computed_summary();
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, ManifestBuildConsumerViolation::NarrowedWithoutDisclosure { .. })));
}

#[test]
fn detects_confidence_inconsistency() {
    let mut p = packet();
    // Heuristic parse can never claim high confidence.
    let idx = p
        .rows
        .iter()
        .position(|r| r.adapter_source == M5AdapterSourceKind::HeuristicParse)
        .expect("a heuristic row");
    p.rows[idx].discovery_confidence = M5DiscoveryConfidence::High;
    p.summary = p.computed_summary();
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, ManifestBuildConsumerViolation::ConfidenceInconsistent { .. })));
}

#[test]
fn detects_summary_mismatch() {
    let mut p = packet();
    p.summary.row_count += 1;
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, ManifestBuildConsumerViolation::SummaryMismatch)));
}

#[test]
fn detects_surface_group_mismatch() {
    let mut p = packet();
    p.rows[0].consumer_group = ConsumerGroup::AiExplanation;
    p.summary = p.computed_summary();
    assert!(p
        .validate()
        .iter()
        .any(|v| matches!(v, ManifestBuildConsumerViolation::SurfaceGroupMismatch { .. })));
}

#[test]
fn checked_support_export_matches_builder() {
    let on_disk = current_stable_m5_manifest_build_consumer_export()
        .expect("checked-in support export is valid");
    assert_eq!(on_disk, packet(), "on-disk artifact drifted from builder");
    assert_eq!(
        MANIFEST_BUILD_CONSUMER_ARTIFACT_JSON.trim_end(),
        packet().export_safe_json().trim_end(),
        "on-disk JSON is not byte-aligned with the builder export"
    );
}

#[test]
fn csv_has_a_header_and_one_row_per_consumer() {
    let p = packet();
    let csv = p.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), p.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,consumer_group"));
}
