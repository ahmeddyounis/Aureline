use aureline_runtime::coverage_overlays_and_snapshot_golden_review::{
    current_coverage_review_export, CoverageEvidenceProvenance, CoverageMetricMode,
    CoverageReviewPacket, RawFallbackAvailability, SnapshotReviewDecision,
};
use aureline_runtime::durable_test_items_and_partial_discovery::DurableTestNodeKind;

fn fixture(name: &str) -> CoverageReviewPacket {
    let path = format!(
        "{}/../../fixtures/testing/m5/coverage-overlays-and-snapshot-golden-review/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let contents = std::fs::read_to_string(path).expect("fixture should be readable");
    serde_json::from_str(&contents).expect("fixture should parse")
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_coverage_review_export()
        .expect("checked-in coverage review export should validate");
    assert!(packet.validate().is_empty());

    // The provenance vocabulary is exercised, not merely declared.
    for provenance in CoverageEvidenceProvenance::REQUIRED {
        assert!(
            packet.represented_provenances().contains(&provenance),
            "missing provenance {}",
            provenance.as_str()
        );
    }

    // Both branch and line modes are represented.
    let modes = packet.represented_metric_modes();
    assert!(modes.contains(&CoverageMetricMode::LineCoverage));
    assert!(modes.contains(&CoverageMetricMode::BranchCoverage));
}

#[test]
fn only_verified_runs_present_as_authoritative_green() {
    let packet = current_coverage_review_export().expect("export validates");
    for overlay in &packet.overlays {
        if overlay.presents_as_authoritative {
            assert_eq!(
                overlay.provenance,
                CoverageEvidenceProvenance::VerifiedCurrentRun,
                "overlay {} presents as authoritative without a verified current run",
                overlay.overlay_id
            );
        }
    }
}

#[test]
fn imported_overlays_never_read_as_local() {
    let packet = current_coverage_review_export().expect("export validates");
    let imported = packet
        .overlays
        .iter()
        .find(|o| o.provenance.is_imported())
        .expect("an imported overlay");
    assert!(imported.origin_provider_ref.is_some());
    assert!(!imported.presents_as_authoritative);
}

#[test]
fn merge_sheets_disclose_omissions_and_never_imply_false_certainty() {
    let packet = current_coverage_review_export().expect("export validates");
    let merge = packet
        .merges
        .iter()
        .find(|m| m.excluded_run_count() >= 1)
        .expect("a merge with an excluded run");
    assert!(!merge.omitted_scopes.is_empty());
    assert!(!merge.implies_complete_certainty);
}

#[test]
fn binary_snapshots_cannot_be_blind_accepted() {
    let packet = current_coverage_review_export().expect("export validates");
    for card in &packet.snapshot_cards {
        if card.decision.is_applied() {
            assert!(
                card.raw_fallback.supports_reviewed_accept(),
                "card {} was accepted without a reviewable fallback",
                card.card_id
            );
        }
    }
    // The binary raw-inspection gate is exercised.
    assert!(packet.snapshot_cards.iter().any(|c| {
        c.raw_fallback == RawFallbackAvailability::UnavailableBinaryOnly
            && c.decision == SnapshotReviewDecision::NeedsRawInspection
    }));
}

#[test]
fn snapshot_template_and_invocation_identities_stay_distinct() {
    let packet = current_coverage_review_export().expect("export validates");
    let kinds = packet.represented_snapshot_subject_kinds();
    assert!(kinds.contains(&DurableTestNodeKind::ParameterizedTemplate));
    assert!(kinds.contains(&DurableTestNodeKind::ConcreteInvocation));
}

#[test]
fn fixture_binary_snapshot_requires_raw_inspection() {
    let packet = fixture("binary_snapshot_requires_raw_inspection.json");
    assert!(packet.validate().is_empty());

    let binary = packet
        .snapshot_cards
        .iter()
        .find(|c| c.raw_fallback == RawFallbackAvailability::UnavailableBinaryOnly)
        .expect("a binary-only card");
    assert_eq!(binary.decision, SnapshotReviewDecision::NeedsRawInspection);
    assert!(!binary.decision.is_applied());

    // An imported card stays read-only and is never accepted as a local baseline.
    let imported = packet
        .snapshot_cards
        .iter()
        .find(|c| c.imported)
        .expect("an imported card");
    assert!(!imported.decision.is_applied());
}
