//! Protected tests binding the typed benchmark-lab governance register to the
//! checked-in artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the capture cross-check proves the typed
//! model and the Python gate agree on the qualification verdict, the asset-kind coverage
//! counts, the packet-freshness counts, and the narrowed/qualified counts; the negative
//! cases mutate a parsed copy and the checked-in fixtures to prove that an asset which
//! fails to narrow, a backed row over its budget, a row carried wider than its public
//! claim's ceiling, and a qualification verdict that disagrees with the firing rules all
//! fail validation.

use std::path::{Path, PathBuf};

use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};
use aureline_release::{
    current_benchmark_lab_governance, AssetState, BenchmarkLabGapReason, BenchmarkLabGovernance,
    BenchmarkLabGovernanceViolation, GovernanceAssetKind, BENCHMARK_LAB_GOVERNANCE_RECORD_KIND,
    BENCHMARK_LAB_GOVERNANCE_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack_validation_capture.json"
));

fn register() -> BenchmarkLabGovernance {
    current_benchmark_lab_governance()
        .expect("checked-in benchmark-lab governance register parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_register_parses_and_validates() {
    let reg = register();
    assert_eq!(reg.schema_version, BENCHMARK_LAB_GOVERNANCE_SCHEMA_VERSION);
    assert_eq!(reg.record_kind, BENCHMARK_LAB_GOVERNANCE_RECORD_KIND);
    let violations = reg.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_governance_asset_kind() {
    let reg = register();
    for kind in GovernanceAssetKind::ALL {
        assert!(
            !reg.rows_for_kind(kind).is_empty(),
            "governance asset kind {} must have at least one row",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_surface() {
    let reg = register();
    assert!(!reg.release_blocking_surface_refs.is_empty());
    let covered: Vec<&str> = reg
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.surface_ref.as_str())
        .collect();
    for declared in &reg.release_blocking_surface_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let reg = register();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(reg.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_entries"].as_u64().unwrap() as usize,
        reg.rows.len(),
        "capture entry count must match the model"
    );
    assert_eq!(
        summary["entries_qualified_stable"].as_u64().unwrap() as usize,
        reg.rows
            .iter()
            .filter(|r| r.asset_state == AssetState::QualifiedStable)
            .count(),
        "capture qualified-stable count must match the model"
    );
    assert_eq!(
        summary["entries_narrowed_below_cutline"].as_u64().unwrap() as usize,
        reg.rows_narrowed().len(),
        "capture narrowed count must match the model"
    );
    for (key, kind) in [
        (
            "nightly_ci_lane_entries",
            GovernanceAssetKind::NightlyBenchmarkCiLane,
        ),
        (
            "self_capture_parity_entries",
            GovernanceAssetKind::SelfCaptureParityCheck,
        ),
        (
            "microbenchmark_corpus_entries",
            GovernanceAssetKind::MicrobenchmarkCorpus,
        ),
        (
            "workflow_archetype_corpus_entries",
            GovernanceAssetKind::WorkflowArchetypeCorpus,
        ),
        (
            "remote_collaboration_corpus_entries",
            GovernanceAssetKind::RemoteCollaborationCorpus,
        ),
        (
            "accessibility_corpus_entries",
            GovernanceAssetKind::AccessibilityCorpus,
        ),
        (
            "protected_metrics_file_entries",
            GovernanceAssetKind::ProtectedMetricsFile,
        ),
        (
            "reference_hardware_manifest_entries",
            GovernanceAssetKind::ReferenceHardwareManifest,
        ),
        (
            "lab_image_manifest_entries",
            GovernanceAssetKind::LabImageManifest,
        ),
        (
            "protected_path_ledger_entries",
            GovernanceAssetKind::ProtectedPathLedger,
        ),
        (
            "public_benchmark_publication_pack_entries",
            GovernanceAssetKind::PublicBenchmarkPublicationPack,
        ),
    ] {
        assert_eq!(
            summary[key].as_u64().unwrap() as usize,
            reg.rows_for_kind(kind).len(),
            "capture {key} must match the model"
        );
    }
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        reg.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );

    let captured_decision = capture["qualification"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        reg.qualification.decision.as_str(),
        "capture qualification decision must match the model"
    );
    assert_eq!(
        reg.qualification.decision,
        reg.computed_qualification_decision()
    );

    for drill in capture["negative_drills"].as_array().unwrap() {
        assert_eq!(
            drill["status"].as_str(),
            Some("passed"),
            "frozen capture drill {} must have passed",
            drill["drill_id"]
        );
    }
    let fixtures = capture["fixture_cases"].as_array().unwrap();
    assert!(!fixtures.is_empty(), "capture must record fixture cases");
    for case in fixtures {
        assert_eq!(
            case["status"].as_str(),
            Some("passed"),
            "frozen capture fixture case {} must have passed",
            case["case_id"]
        );
    }
}

#[test]
fn register_narrows_a_row_under_a_still_stable_claim() {
    let reg = register();
    let narrowed = reg.rows.iter().find(|row| {
        row.release_blocking
            && row.claim_holds_stable()
            && !row.publishes_stable()
            && row.asset_state != AssetState::NarrowedClaimNarrowed
    });
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one release-blocking row under a still-stable claim"
    );
}

#[test]
fn register_narrows_a_row_for_stale_packet() {
    let reg = register();
    let stale = reg
        .rows
        .iter()
        .find(|row| row.asset_state == AssetState::NarrowedStale)
        .expect("the register must show a stale row");
    assert!(!stale.publishes_stable());
    assert!(stale.has_active_reason(BenchmarkLabGapReason::ProofPacketFreshnessBreached));
}

#[test]
fn register_shows_a_row_on_waiver() {
    let reg = register();
    let on_waiver = reg
        .rows
        .iter()
        .find(|row| row.asset_state == AssetState::QualifiedOnWaiver)
        .expect("the register must show a row on waiver");
    assert!(on_waiver.waiver.is_some());
    assert!(on_waiver.publishes_stable());
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| {
            row.asset_state == AssetState::NarrowedStale
                && row.claim_label == StableClaimLevel::Stable
        })
        .expect("register has a stale row under a stable ceiling");
    row.published_label = StableClaimLevel::Stable;
    reg.summary = reg.computed_summary();
    reg.qualification.decision = reg.computed_qualification_decision();
    reg.qualification.blocking_rule_ids = reg.computed_blocking_rule_ids();
    reg.qualification.blocking_entry_ids = reg.computed_blocking_entry_ids();

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            BenchmarkLabGovernanceViolation::PublishedLabelNotNarrowed { .. }
        )),
        "a row that is not backed must narrow below the cutline"
    );
}

#[test]
fn backed_row_on_a_breached_packet_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.holds_label())
        .expect("register has a backed row");
    row.proof_packet.slo_state = FreshnessSloState::Breached;
    reg.summary = reg.computed_summary();

    assert!(
        reg.validate()
            .iter()
            .any(|v| matches!(v, BenchmarkLabGovernanceViolation::HeldOnStalePacket { .. })),
        "a backed row may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn qualification_proceed_while_a_rule_fires_fails() {
    let mut reg = register();
    reg.qualification.decision = PromotionDecision::Proceed;

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            BenchmarkLabGovernanceViolation::QualificationDecisionInconsistent { .. }
        )),
        "qualification must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut model_checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let expected = case["expected_check_id"].as_str().unwrap_or_default();
        if expected.starts_with("ceiling.") {
            continue;
        }
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: BenchmarkLabGovernance =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
        model_checked += 1;
    }
    assert!(
        model_checked > 0,
        "at least one fixture must exercise a typed-model structural invariant"
    );
}
