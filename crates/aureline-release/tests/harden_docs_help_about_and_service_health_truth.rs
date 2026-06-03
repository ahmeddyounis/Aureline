//! Protected tests binding the typed docs/help/About/service-health truth register
//! to the checked-in artifact, the frozen CI validation capture, and the negative
//! fixtures.
//!
//! The positive case is the checked-in register; the capture cross-check proves the
//! typed model and the CI gate agree on the publication verdict, the surface-kind
//! coverage counts, the service-contract-state coverage, and the packet-freshness
//! counts; the negative cases mutate a parsed copy and the checked-in fixtures to
//! prove that a narrowing truth row that does not narrow, a service-health row
//! without a contract state, and a publication verdict that disagrees with the
//! firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::harden_docs_help_about_and_service_health_truth::{
    current_docs_help_about_service_health_truth, DestinationTrustClass,
    DocsHelpAboutServiceHealthTruth, DocsHelpAboutServiceHealthTruthViolation,
    ServiceContractState, SurfaceKind, TruthState,
    DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_RECORD_KIND,
    DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_SCHEMA_VERSION,
};
use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/harden_docs_help_about_and_service_health_truth_validation_capture.json"
));

fn register() -> DocsHelpAboutServiceHealthTruth {
    current_docs_help_about_service_health_truth()
        .expect("checked-in docs/help/About/service-health truth register parses into the model")
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
        DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_SCHEMA_VERSION
    );
    assert_eq!(
        reg.record_kind,
        DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_RECORD_KIND
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
fn covers_every_service_contract_state() {
    let reg = register();
    let covered: Vec<ServiceContractState> = reg
        .rows
        .iter()
        .filter_map(|row| row.service_contract_state)
        .collect();
    for state in ServiceContractState::ALL {
        assert!(
            covered.contains(&state),
            "service contract state {} must appear on at least one row",
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
fn about_row_carries_provenance_card_with_trust_class_destinations() {
    let reg = register();
    let about = reg
        .rows_for_kind(SurfaceKind::About)
        .into_iter()
        .find(|row| row.about_card.is_some())
        .expect("at least one About row carries a provenance card");
    let card = about.about_card.as_ref().unwrap();
    assert!(!card.version.is_empty());
    assert!(!card.channel.is_empty());
    assert!(!card.copy_build_info_action.is_empty());
    assert!(
        !card.destinations.is_empty(),
        "provenance card must list destinations"
    );
    let classes: Vec<DestinationTrustClass> =
        card.destinations.iter().map(|d| d.trust_class).collect();
    assert!(
        classes.contains(&DestinationTrustClass::Official),
        "card must have at least one official destination"
    );
    assert!(
        classes.contains(&DestinationTrustClass::Community),
        "card must have at least one community destination"
    );
}

#[test]
fn package_safety_row_carries_disclosure() {
    let reg = register();
    let pkg = reg
        .rows_for_kind(SurfaceKind::PackageSafety)
        .into_iter()
        .find(|row| row.package_safety.is_some())
        .expect("at least one package-safety row carries a disclosure");
    let disc = pkg.package_safety.as_ref().unwrap();
    assert!(!disc.manifest_scope.is_empty());
    assert!(!disc.registry_auth_source.is_empty());
    assert!(!disc.lockfile_impact.is_empty());
    assert!(!disc.license_advisory_delta.is_empty());
    assert!(!disc.validation_tasks.is_empty());
    assert!(!disc.rollback_path.is_empty());
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
        summary["entries_published_stable"].as_u64().unwrap() as usize,
        reg.rows_published_stable().len(),
        "capture published count must match the model"
    );
    for (key, kind) in [
        ("docs_entries", SurfaceKind::Docs),
        ("help_entries", SurfaceKind::Help),
        ("about_entries", SurfaceKind::About),
        ("service_health_entries", SurfaceKind::ServiceHealth),
        ("package_safety_entries", SurfaceKind::PackageSafety),
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
fn register_shows_service_health_stale_downgrade() {
    let reg = register();
    let stale = reg
        .rows
        .iter()
        .find(|row| {
            row.surface_kind == SurfaceKind::ServiceHealth
                && row.service_contract_state == Some(ServiceContractState::Stale)
        })
        .expect("the register must show a stale service-health row");
    assert_eq!(stale.truth_state, TruthState::NarrowedStale);
    assert!(!stale.publishes_stable());
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| {
            row.truth_state == TruthState::NarrowedStale
                && row.claim_label == StableClaimLevel::Stable
        })
        .expect("register has a narrowed-stale row under a stable ceiling");
    row.published_label = StableClaimLevel::Stable;
    reg.summary = reg.computed_summary();
    reg.publication.decision = reg.computed_publication_decision();
    reg.publication.blocking_rule_ids = reg.computed_blocking_rule_ids();
    reg.publication.blocking_entry_ids = reg.computed_blocking_entry_ids();

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            DocsHelpAboutServiceHealthTruthViolation::PublishedLabelNotNarrowed { .. }
        )),
        "a row that is not backed must narrow below the cutline"
    );
}

#[test]
fn service_health_without_contract_state_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.surface_kind == SurfaceKind::ServiceHealth)
        .expect("register has a service-health row");
    row.service_contract_state = None;
    reg.summary = reg.computed_summary();

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            DocsHelpAboutServiceHealthTruthViolation::ServiceHealthWithoutContractState { .. }
        )),
        "a service-health row must carry a service_contract_state"
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
        reg.validate().iter().any(|v| matches!(
            v,
            DocsHelpAboutServiceHealthTruthViolation::HeldOnStalePacket { .. }
        )),
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
            DocsHelpAboutServiceHealthTruthViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir =
        repo_root().join("fixtures/release/harden_docs_help_about_and_service_health_truth");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: DocsHelpAboutServiceHealthTruth =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
    }
}
