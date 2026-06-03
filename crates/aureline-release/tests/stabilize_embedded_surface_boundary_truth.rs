//! Protected tests binding the typed embedded-surface boundary truth register
//! to the checked-in artifact, the frozen CI validation capture, and the negative
//! fixtures.

use std::path::{Path, PathBuf};

use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};
use aureline_release::stabilize_embedded_surface_boundary_truth::{
    current_embedded_surface_boundary_truth, BoundaryState, EmbeddedSurfaceBoundaryTruth,
    EmbeddedSurfaceBoundaryTruthViolation, SurfaceKind, TruthState,
    EMBEDDED_SURFACE_BOUNDARY_TRUTH_RECORD_KIND,
    EMBEDDED_SURFACE_BOUNDARY_TRUTH_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/stabilize_embedded_surface_boundary_truth_validation_capture.json"
));

fn register() -> EmbeddedSurfaceBoundaryTruth {
    current_embedded_surface_boundary_truth()
        .expect("checked-in embedded surface boundary truth register parses into the model")
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
        EMBEDDED_SURFACE_BOUNDARY_TRUTH_SCHEMA_VERSION
    );
    assert_eq!(
        reg.record_kind,
        EMBEDDED_SURFACE_BOUNDARY_TRUTH_RECORD_KIND
    );
    let violations = reg.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_surface_kind() {
    let reg = register();
    for kind in SurfaceKind::ALL {
        assert!(
            !reg.rows_for_kind(kind).is_empty(),
            "surface kind {} must have at least one row",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_boundary_state() {
    let reg = register();
    let covered: Vec<BoundaryState> = reg.rows.iter().map(|row| row.boundary_state).collect();
    for state in BoundaryState::ALL {
        assert!(
            covered.contains(&state),
            "boundary state {} must appear on at least one row",
            state.as_str()
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
fn docs_help_rows_carry_source_truth() {
    let reg = register();
    let docs = reg.rows_for_kind(SurfaceKind::EmbeddedDocsHelp);
    assert!(
        docs.iter().any(|row| row.source_truth.is_some()),
        "at least one docs/help row must carry a source_truth snapshot"
    );
}

#[test]
fn auth_rows_carry_auth_handoff() {
    let reg = register();
    let auth = reg.rows_for_kind(SurfaceKind::EmbeddedAuthConfirmation);
    assert!(
        auth.iter().any(|row| row.auth_handoff.is_some()),
        "at least one auth row must carry an auth_handoff snapshot"
    );
}

#[test]
fn native_approval_remains_host_owned_on_all_rows() {
    let reg = register();
    for row in &reg.rows {
        assert!(
            row.native_approval.high_risk_approval_host_owned,
            "{}: high_risk_approval must be host-owned",
            row.entry_id
        );
        assert!(
            row.native_approval.destructive_confirmation_host_owned,
            "{}: destructive_confirmation must be host-owned",
            row.entry_id
        );
        assert!(
            row.native_approval.trust_elevation_host_owned,
            "{}: trust_elevation must be host-owned",
            row.entry_id
        );
        assert!(
            row.native_approval.update_verification_host_owned,
            "{}: update_verification must be host-owned",
            row.entry_id
        );
        assert!(
            row.native_approval.ai_apply_review_host_owned,
            "{}: ai_apply_review must be host-owned",
            row.entry_id
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let reg = register();
    let capture: serde_json::Value = serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(reg.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_entries"].as_u64().unwrap() as usize,
        reg.rows.len(),
        "capture entry count must match the model"
    );
    assert_eq!(
        summary["entries_published_stable"]
            .as_u64()
            .unwrap() as usize,
        reg.rows_published_stable().len(),
        "capture published count must match the model"
    );
    for (key, kind) in [
        ("docs_help_entries", SurfaceKind::EmbeddedDocsHelp),
        ("extension_entries", SurfaceKind::ExtensionHostedSurface),
        ("marketplace_entries", SurfaceKind::EmbeddedMarketplaceOrAccount),
        ("service_dashboard_entries", SurfaceKind::EmbeddedServiceDashboard),
        ("auth_confirmation_entries", SurfaceKind::EmbeddedAuthConfirmation),
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
    assert_eq!(
        summary["packets_missing"].as_u64().unwrap() as usize,
        reg.computed_summary().packets_missing,
        "capture missing-packet count must match the model"
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
fn register_narrows_a_row_under_a_still_stable_claim() {
    let reg = register();
    let narrowed = reg.rows.iter().find(|row| {
        row.release_blocking
            && row.claim_holds_stable()
            && !row.publishes_stable()
            && row.truth_state != TruthState::NarrowedClaimNarrowed
    });
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one release-blocking row under a still-stable claim"
    );
}

#[test]
fn register_shows_stale_service_dashboard_downgrade() {
    let reg = register();
    let stale = reg
        .rows
        .iter()
        .find(|row| {
            row.surface_kind == SurfaceKind::EmbeddedServiceDashboard
                && row.boundary_state == BoundaryState::StaleSnapshot
        })
        .expect("the register must show a stale service-dashboard row");
    assert_eq!(stale.truth_state, TruthState::NarrowedStale);
    assert!(!stale.publishes_stable());
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.truth_state == TruthState::NarrowedStale && row.claim_label == StableClaimLevel::Stable)
        .expect("register has a narrowed-stale row under a stable ceiling");
    row.published_label = StableClaimLevel::Stable;
    reg.summary = reg.computed_summary();
    reg.publication.decision = reg.computed_publication_decision();
    reg.publication.blocking_rule_ids = reg.computed_blocking_rule_ids();
    reg.publication.blocking_entry_ids = reg.computed_blocking_entry_ids();

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            EmbeddedSurfaceBoundaryTruthViolation::PublishedLabelNotNarrowed { .. }
        )),
        "a row that is not backed must narrow below the cutline"
    );
}

#[test]
fn docs_help_without_source_truth_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.surface_kind == SurfaceKind::EmbeddedDocsHelp)
        .expect("register has a docs/help row");
    row.source_truth = None;
    reg.summary = reg.computed_summary();

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            EmbeddedSurfaceBoundaryTruthViolation::DocsHelpWithoutSourceTruth { .. }
        )),
        "a docs/help row must carry a source_truth snapshot"
    );
}

#[test]
fn auth_without_handoff_snapshot_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.surface_kind == SurfaceKind::EmbeddedAuthConfirmation)
        .expect("register has an auth row");
    row.auth_handoff = None;
    reg.summary = reg.computed_summary();

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            EmbeddedSurfaceBoundaryTruthViolation::AuthWithoutHandoffSnapshot { .. }
        )),
        "an auth row must carry an auth_handoff snapshot"
    );
}

#[test]
fn native_approval_leak_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.native_approval.high_risk_approval_host_owned)
        .expect("register has a host-owned row");
    row.native_approval.high_risk_approval_host_owned = false;
    reg.summary = reg.computed_summary();

    assert!(
        reg.validate()
            .iter()
            .any(|v| matches!(v, EmbeddedSurfaceBoundaryTruthViolation::NativeApprovalLeaked { .. })),
        "a row with leaked native approval must fail validation"
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
            .any(|v| matches!(v, EmbeddedSurfaceBoundaryTruthViolation::HeldOnStalePacket { .. })),
        "a backed row may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn publication_proceed_while_a_rule_fires_fails() {
    let mut reg = register();
    reg.publication.decision = PromotionDecision::Proceed;

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            EmbeddedSurfaceBoundaryTruthViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/stabilize_embedded_surface_boundary_truth");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value = serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: EmbeddedSurfaceBoundaryTruth =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
    }
}
