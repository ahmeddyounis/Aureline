//! Fixture-driven coverage for bundle-archetype certification packets.
//!
//! These tests load every fixture under
//! `fixtures/review/m4/certify-launch-bundles-imported-user-handoff-bundles-and/`
//! and assert that:
//!
//! 1. Every fixture's input builds, validates, and projects without error.
//! 2. The effective badge, support claim, and downgrade verdict match the
//!    fixture's recorded expectation — proving the automatic downgrade.
//! 3. Stable (`certified` / `managed_approved`) effective badges only render
//!    when the claim resolves to a current scorecard row; no stable claim is
//!    implied from prose alone.
//! 4. Imported-user handoff bundles preserve a migration report and unsupported
//!    items rather than collapsing into one green banner.
//! 5. Offline/mirror distribution keeps the same machine-readable scorecard
//!    vocabulary, and no bundle application allows hidden authority widening.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    project_bundle_archetype_certification, BundleArchetypeCertificationInput,
    BundleArchetypeCertificationPacket, STABLE_BADGE_CLASSES,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CertificationFixture {
    #[allow(dead_code)]
    record_kind: String,
    #[allow(dead_code)]
    schema_version: u32,
    case_name: String,
    certification_input: BundleArchetypeCertificationInput,
    expected: ExpectedCertification,
}

#[derive(Debug, Deserialize)]
struct ExpectedCertification {
    effective_badge_class: String,
    support_claim_class: String,
    stable_claim: bool,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
    resolves_to_current_scorecard_row: bool,
    no_prose_only_stable_claim: bool,
    imported_handoff_preserved: bool,
    offline_parity_preserved: bool,
    scorecard_row_count: usize,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../fixtures/review/m4/certify-launch-bundles-imported-user-handoff-bundles-and",
    )
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("certification fixtures dir must exist")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn load_fixture(path: &Path) -> CertificationFixture {
    let payload = std::fs::read_to_string(path).expect("fixture must read");
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

#[test]
fn every_fixture_builds_validates_and_matches_expectations() {
    let paths = load_fixture_paths();
    assert!(!paths.is_empty(), "certification fixtures dir must not be empty");

    for path in &paths {
        let fixture = load_fixture(path);
        let packet =
            BundleArchetypeCertificationPacket::from_input(fixture.certification_input.clone())
                .unwrap_or_else(|err| panic!("fixture {:?} must build: {err}", fixture.case_name));

        // Re-validation and JSON round-trip must both hold.
        packet.validate().expect("packet must re-validate");
        let payload = serde_json::to_string(&packet).expect("serialize");
        let projection = project_bundle_archetype_certification(&payload)
            .unwrap_or_else(|err| panic!("fixture {:?} must project: {err}", fixture.case_name));

        let expected = &fixture.expected;
        assert_eq!(
            packet.claim.effective_badge_class, expected.effective_badge_class,
            "fixture {:?} effective badge",
            fixture.case_name
        );
        assert_eq!(
            packet.claim.support_claim_class, expected.support_claim_class,
            "fixture {:?} support claim",
            fixture.case_name
        );
        assert_eq!(
            packet.claim.downgraded, expected.downgraded,
            "fixture {:?} downgraded",
            fixture.case_name
        );
        let mut got_reasons = packet.claim.downgrade_reasons.clone();
        got_reasons.sort();
        let mut want_reasons = expected.downgrade_reasons.clone();
        want_reasons.sort();
        assert_eq!(
            got_reasons, want_reasons,
            "fixture {:?} downgrade reasons",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.stable_claim, expected.stable_claim,
            "fixture {:?} stable claim",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.resolves_to_current_scorecard_row,
            expected.resolves_to_current_scorecard_row,
            "fixture {:?} resolves to current row",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.no_prose_only_stable_claim, expected.no_prose_only_stable_claim,
            "fixture {:?} no prose-only stable claim",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.imported_handoff_preserved, expected.imported_handoff_preserved,
            "fixture {:?} imported handoff preserved",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.offline_parity_preserved, expected.offline_parity_preserved,
            "fixture {:?} offline parity preserved",
            fixture.case_name
        );
        assert_eq!(
            projection.scorecard_row_count, expected.scorecard_row_count,
            "fixture {:?} scorecard row count",
            fixture.case_name
        );

        // Cross-cutting invariants that must hold for every fixture.
        assert!(
            packet.no_prose_only_stable_claim(),
            "fixture {:?} must never imply a stable claim from prose",
            fixture.case_name
        );
        assert!(
            !packet.allows_hidden_setup_hooks
                && !packet.allows_secret_injection
                && !packet.allows_silent_trust_widening,
            "fixture {:?} must not allow hidden authority widening",
            fixture.case_name
        );

        // A stable effective badge must resolve to a current scorecard row.
        if STABLE_BADGE_CLASSES.contains(&packet.claim.effective_badge_class.as_str()) {
            assert!(
                packet.inspection.resolves_to_current_scorecard_row,
                "fixture {:?} stable badge must resolve to a current row",
                fixture.case_name
            );
            assert!(
                !packet.claim.downgraded,
                "fixture {:?} stable badge must not be downgraded",
                fixture.case_name
            );
        }
    }
}

#[test]
fn imported_user_bundles_preserve_migration_truth() {
    for path in load_fixture_paths() {
        let fixture = load_fixture(&path);
        if fixture.certification_input.identity.bundle_class != "imported_user_bundle" {
            continue;
        }
        let packet =
            BundleArchetypeCertificationPacket::from_input(fixture.certification_input.clone())
                .expect("imported bundle fixture must build");
        let handoff = packet
            .imported_handoff
            .as_ref()
            .expect("imported_user_bundle must carry a handoff report");
        assert!(handoff.preserved, "handoff must be preserved");
        assert!(
            !handoff.migration_report_ref.trim().is_empty(),
            "migration report must be preserved"
        );
    }
}

#[test]
fn stale_evidence_forces_a_downgrade() {
    let path = fixtures_dir().join("stale_certification_auto_downgrade.json");
    let fixture = load_fixture(&path);
    let packet = BundleArchetypeCertificationPacket::from_input(fixture.certification_input)
        .expect("must build");
    assert!(packet.claim.downgraded, "stale evidence must downgrade");
    assert!(
        !STABLE_BADGE_CLASSES.contains(&packet.claim.effective_badge_class.as_str()),
        "stale evidence must not keep a stable badge"
    );
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"scorecard_freshness_expired".to_string()));
}