//! Tests for the scanner-import quality-parity packet.

use super::*;
use crate::scanner_import::{
    CI_LOCAL_PARITY_VIEW_RECORD_KIND, CLI_PROJECTION_RECORD_KIND,
    DIAGNOSTIC_REVIEW_PACKET_RECORD_KIND, PIPELINE_VIEWER_PROJECTION_RECORD_KIND,
    PROBLEMS_PROJECTION_RECORD_KIND, RELEASE_PACKET_RECORD_KIND, SUPPORT_EXPORT_RECORD_KIND,
};

fn packet() -> ScannerImportQualityParity {
    current_scanner_import_quality_parity().expect("embedded packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        SCANNER_IMPORT_QUALITY_PARITY_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        SCANNER_IMPORT_QUALITY_PARITY_RECORD_KIND
    );
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_surface_and_parity_state_is_present_once() {
    let packet = packet();
    for surface in SurfaceClass::ALL {
        assert!(
            packet.surface(surface).is_some(),
            "missing surface {}",
            surface.as_str()
        );
    }
    for state in ParityStateClass::ALL {
        assert!(
            packet.parity_state(state).is_some(),
            "missing parity state {}",
            state.as_str()
        );
    }
    assert_eq!(
        packet.source_format_classes,
        ParitySourceFormatClass::ALL.to_vec()
    );
}

#[test]
fn surface_record_kinds_bind_to_the_scanner_import_lane() {
    let packet = packet();
    let expected = [
        (SurfaceClass::Problems, PROBLEMS_PROJECTION_RECORD_KIND),
        (
            SurfaceClass::ReviewWorkspace,
            DIAGNOSTIC_REVIEW_PACKET_RECORD_KIND,
        ),
        (
            SurfaceClass::PipelineViewer,
            PIPELINE_VIEWER_PROJECTION_RECORD_KIND,
        ),
        (SurfaceClass::SupportBundle, SUPPORT_EXPORT_RECORD_KIND),
        (SurfaceClass::ReleasePacket, RELEASE_PACKET_RECORD_KIND),
        (
            SurfaceClass::CiLocalParity,
            CI_LOCAL_PARITY_VIEW_RECORD_KIND,
        ),
        (SurfaceClass::Cli, CLI_PROJECTION_RECORD_KIND),
    ];
    for (surface, record_kind) in expected {
        let row = packet.surface(surface).expect("surface present");
        assert_eq!(
            row.record_kind_ref,
            record_kind,
            "surface {}",
            surface.as_str()
        );
        assert!(row.keeps_imported_truth());
        assert!(row.baseline_compatible);
        assert!(!row.downgrade_behaviors.is_empty());
    }
}

#[test]
fn parity_state_flags_match_canonical_semantics() {
    let packet = packet();
    for row in &packet.parity_states {
        let class = row.parity_state_class;
        assert_eq!(row.is_gap, class.is_gap(), "{}", class.as_str());
        assert_eq!(
            row.blocks_exact_delta,
            class.blocks_exact_delta(),
            "{}",
            class.as_str()
        );
        assert_eq!(
            row.blocks_promotion,
            class.blocks_promotion(),
            "{}",
            class.as_str()
        );
    }
    // The gaps the row must surface as first-class states are present.
    for gap in [
        ParityStateClass::ParityGapStaleImported,
        ParityStateClass::ParityGapUnmappedRule,
        ParityStateClass::UnsupportedScannerFamily,
    ] {
        assert!(packet.parity_state(gap).expect("gap present").is_gap);
    }
}

#[test]
fn summary_and_export_projection_are_consistent() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
    assert_eq!(packet.summary.total_surfaces, 7);
    assert_eq!(packet.summary.gap_states, 6);
    assert_eq!(packet.summary.promotion_blocking_states, 4);

    let projection = packet.export_projection();
    assert!(projection.redaction_safe);
    assert_eq!(projection.labeled_read_only_surfaces.len(), 7);
    assert!(projection.promotable);
    assert!(projection
        .gap_state_tokens
        .iter()
        .any(|token| token == "unsupported_scanner_family"));
}

#[test]
fn promotion_gate_keeps_requirements_non_waivable() {
    let packet = packet();
    assert!(packet.promotion_gate.promotable);
    assert!(packet.promotion_gate.blocking_reasons.is_empty());
    assert!(packet.computed_promotable());

    // Waiving delta compatibility flips the gate closed and is flagged.
    let mut weakened = packet.clone();
    weakened.promotion_gate.requires_delta_compatibility = false;
    assert!(!weakened.computed_promotable());
    let violations = weakened.validate();
    assert!(violations.contains(
        &ScannerImportQualityParityViolation::PromotionRequirementWaived {
            requirement: "requires_delta_compatibility",
        }
    ));
    assert!(violations.contains(&ScannerImportQualityParityViolation::PromotionFlagMismatch));
}

#[test]
fn dropping_a_surface_label_is_a_violation() {
    let mut packet = packet();
    packet.surfaces[0].imported_read_only = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ScannerImportQualityParityViolation::UnlabeledImportedSurface { .. }
    )));
    // An unlabeled surface also flips the gate closed.
    assert!(!packet.computed_promotable());
}

#[test]
fn round_trips_through_json() {
    let packet = packet();
    let json = serde_json::to_string(&packet).expect("serialize");
    let restored: ScannerImportQualityParity = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(packet, restored);
}
