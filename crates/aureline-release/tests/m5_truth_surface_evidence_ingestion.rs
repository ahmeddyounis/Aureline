//! Cross-crate integration tests for the M5 truth-surface evidence-ingestion
//! register, including a cross-check against the frozen validation capture.

use std::collections::BTreeSet;

use aureline_release::m5_truth_surface_evidence_ingestion::{
    current_m5_truth_surface_ingestion, FamilyKind, IngestState, TruthSurface,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/captures/m5_truth_surface_evidence_ingestion_validation_capture.json"
));

#[test]
fn checked_in_register_parses_and_validates() {
    let reg = current_m5_truth_surface_ingestion().expect("register parses");
    let violations = reg.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn covers_every_named_truth_surface_for_every_family() {
    let reg = current_m5_truth_surface_ingestion().expect("register parses");
    let pairs: BTreeSet<(FamilyKind, TruthSurface)> = reg
        .rows
        .iter()
        .map(|r| (r.family_kind, r.truth_surface))
        .collect();
    for kind in FamilyKind::ALL {
        for surface in TruthSurface::ALL {
            assert!(
                pairs.contains(&(kind, surface)),
                "missing {kind:?} on {surface:?}"
            );
        }
    }
}

#[test]
fn no_surface_advertises_a_wider_claim_than_its_source() {
    let reg = current_m5_truth_surface_ingestion().expect("register parses");
    for row in &reg.rows {
        assert!(
            row.published_label.rank() <= row.canonical_claim_label.rank(),
            "{} exceeds its ceiling",
            row.entry_id
        );
        assert_eq!(
            row.published_label, row.canonical_published_label,
            "{} clones a label that differs from the canonical effective label",
            row.entry_id
        );
    }
}

#[test]
fn a_release_blocking_family_is_surfaced_as_narrowed_or_underqualified() {
    let reg = current_m5_truth_surface_ingestion().expect("register parses");
    let surfaced = reg.rows.iter().any(|r| {
        r.release_blocking
            && matches!(
                r.ingest_state,
                IngestState::Narrowed
                    | IngestState::PolicyBlocked
                    | IngestState::PreviewOnly
                    | IngestState::Underqualified
            )
    });
    assert!(
        surfaced,
        "at least one release-blocking family must surface a below-stable posture"
    );
}

#[test]
fn model_matches_frozen_validation_capture() {
    let reg = current_m5_truth_surface_ingestion().expect("register parses");
    let capture: serde_json::Value = serde_json::from_str(CAPTURE_JSON).expect("capture parses");

    assert_eq!(
        capture["total_rows"].as_u64().unwrap() as usize,
        reg.summary.total_rows
    );
    assert_eq!(
        capture["release_blocking_rows"].as_u64().unwrap() as usize,
        reg.summary.release_blocking_rows
    );
    assert_eq!(
        capture["rows_below_cutline"].as_u64().unwrap() as usize,
        reg.summary.rows_below_cutline
    );
    assert_eq!(
        capture["decision"].as_str().unwrap(),
        reg.publication.decision
    );

    let by_state = &capture["rows_by_state"];
    assert_eq!(
        by_state["current"].as_u64().unwrap() as usize,
        reg.rows
            .iter()
            .filter(|r| r.ingest_state == IngestState::Current)
            .count()
    );
    assert_eq!(
        by_state["policy_blocked"].as_u64().unwrap() as usize,
        reg.rows
            .iter()
            .filter(|r| r.ingest_state == IngestState::PolicyBlocked)
            .count()
    );
}
