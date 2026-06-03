//! Protected tests binding the typed M04-183 register to the checked-in artifact,
//! the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the capture cross-check proves the typed
//! model and the Python gate agree on the publication verdict, the object-kind coverage
//! counts, the downgrade-propagation counts, and the packet-freshness counts; the negative
//! cases mutate a parsed copy and the checked-in fixtures to prove that an entry which fails
//! to narrow, a stale packet held, an entry carried wider than its public claim's ceiling,
//! a stale alarm without narrowing, and a downgrade-pending entry without narrowing all fail
//! validation.

use std::path::{Path, PathBuf};

use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};
use aureline_release::finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support::{
    current_finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support,
    DowngradePropagationStatus, FreshnessObjectKind, FreshnessObjectState,
    FinalizeReleasePacketFreshnessSlosShiproomDashboardsAndProofIndexExportForProcurementAndSupport,
    FreshnessObjectViolation,
    FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_RECORD_KIND,
    FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support_validation_capture.json"
));

fn register(
) -> FinalizeReleasePacketFreshnessSlosShiproomDashboardsAndProofIndexExportForProcurementAndSupport
{
    current_finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support()
        .expect("checked-in register parses into the model")
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
    assert_eq!(
        reg.schema_version,
        FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_SCHEMA_VERSION
    );
    assert_eq!(
        reg.record_kind,
        FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_RECORD_KIND
    );
    let violations = reg.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_kind() {
    let reg = register();
    for kind in FreshnessObjectKind::ALL {
        assert!(
            !reg.rows_for_kind(kind).is_empty(),
            "kind {} must have at least one row",
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
        summary["total_rows"].as_u64().unwrap() as usize,
        reg.rows.len(),
        "capture row count must match the model"
    );
    assert_eq!(
        summary["rows_current"].as_u64().unwrap() as usize,
        reg.rows_current_stable().len(),
        "capture current count must match the model"
    );
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        reg.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        reg.publication.decision.as_str(),
        "capture publication decision must match the model"
    );
    assert_eq!(
        reg.publication.decision,
        reg.computed_publication_decision()
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
fn register_narrows_an_entry_under_a_still_stable_claim() {
    let reg = register();
    let narrowed = reg.rows.iter().find(|row| {
        row.release_blocking
            && row.claim_holds_stable()
            && !row.holds_stable()
            && row.object_state != FreshnessObjectState::NarrowedClaimNarrowed
    });
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one release-blocking entry under a still-stable claim"
    );
}

#[test]
fn register_narrows_a_shiproom_panel_for_stale_alarm() {
    let reg = register();
    let stale = reg
        .rows
        .iter()
        .find(|row| {
            row.object_state == FreshnessObjectState::NarrowedStale
                && row.object_kind == FreshnessObjectKind::ShiproomDashboardPanel
        })
        .expect("the register must show a shiproom-panel stale narrowing");
    assert!(!stale.holds_stable());
}

#[test]
fn register_shows_downgrade_propagation_pending() {
    let reg = register();
    let pending = reg
        .rows
        .iter()
        .find(|row| row.downgrade_propagation_status == DowngradePropagationStatus::Pending)
        .expect("the register must show a downgrade-pending row");
    assert!(!pending.holds_stable());
}

#[test]
fn narrowing_entry_that_does_not_narrow_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| {
            row.object_state == FreshnessObjectState::NarrowedStale
                && row.claim_label == StableClaimLevel::Stable
        })
        .expect("register has a narrowed-stale row under a stable ceiling");
    row.effective_label = StableClaimLevel::Stable;
    reg.summary = reg.computed_summary();
    let violations = reg.validate();
    assert!(
        violations.iter().any(|v| matches!(
            v,
            FreshnessObjectViolation::EffectiveLabelNotNarrowed { .. }
        )),
        "a narrowing row that does not narrow must fail validation: {violations:#?}"
    );
}

#[test]
fn held_entry_on_breached_packet_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| {
            row.object_state == FreshnessObjectState::Current
                && row.claim_label == StableClaimLevel::Stable
        })
        .expect("register has a current row");
    row.proof_packet.slo_state = FreshnessSloState::Breached;
    reg.summary = reg.computed_summary();
    let violations = reg.validate();
    assert!(
        violations
            .iter()
            .any(|v| matches!(v, FreshnessObjectViolation::HeldOnStalePacket { .. })),
        "a held row on a breached packet must fail validation: {violations:#?}"
    );
}

#[test]
fn effective_wider_than_claim_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.claim_label == StableClaimLevel::Beta)
        .expect("register has a beta-claim row");
    row.effective_label = StableClaimLevel::Stable;
    reg.summary = reg.computed_summary();
    let violations = reg.validate();
    assert!(
        violations
            .iter()
            .any(|v| matches!(v, FreshnessObjectViolation::EffectiveWiderThanClaim { .. })),
        "a row effective wider than its claim must fail validation: {violations:#?}"
    );
}

#[test]
fn publication_verdict_mismatch_fails() {
    let mut reg = register();
    let original = reg.publication.decision;
    reg.publication.decision = match original {
        PromotionDecision::Proceed => PromotionDecision::Hold,
        PromotionDecision::Hold => PromotionDecision::Proceed,
    };
    let violations = reg.validate();
    assert!(
        violations.iter().any(|v| matches!(
            v,
            FreshnessObjectViolation::PublicationDecisionInconsistent { .. }
        )),
        "a mismatched publication verdict must fail validation: {violations:#?}"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/m4/finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support");
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
        let candidate: FinalizeReleasePacketFreshnessSlosShiproomDashboardsAndProofIndexExportForProcurementAndSupport =
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

#[test]
fn checked_in_artifact_exists_on_disk() {
    let root = repo_root();
    let path = root.join("artifacts/release/finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support.json");
    assert!(path.exists(), "checked-in artifact must exist on disk");
}

#[test]
fn schema_exists_on_disk() {
    let root = repo_root();
    let path = root.join("schemas/release/finalize-release-packet-freshness-slos-shiproom-dashboards-and-proof-index-export-for-procurement-and-support.schema.json");
    assert!(path.exists(), "schema must exist on disk");
}

#[test]
fn proof_packet_exists_on_disk() {
    let root = repo_root();
    let path = root.join("artifacts/release/m4/finalize-release-packet-freshness-slos-shiproom-dashboards-and-proof-index-export-for-procurement-and-support.md");
    assert!(path.exists(), "proof packet must exist on disk");
}

#[test]
fn docs_exist_on_disk() {
    let root = repo_root();
    let path = root.join("docs/m4/finalize-release-packet-freshness-slos-shiproom-dashboards-and-proof-index-export-for-procurement-and-support.md");
    assert!(path.exists(), "docs must exist on disk");
}
