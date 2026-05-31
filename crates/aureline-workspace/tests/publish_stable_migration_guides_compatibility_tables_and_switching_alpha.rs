//! Fixture-driven coverage for migration-switching publication packets.
//!
//! These tests load every fixture under
//! `fixtures/review/m4/publish-stable-migration-guides-compatibility-tables-and-switching/`
//! and assert that:
//!
//! 1. Every fixture's input builds, validates, and projects without error.
//! 2. The effective tier, support claim, and narrowing verdict match the
//!    fixture's recorded expectation — proving the automatic narrowing below
//!    Stable.
//! 3. A `stable` effective tier only renders when it resolves to a current
//!    compatibility table; switch-readiness is never implied from prose alone.
//! 4. Compatibility rows carry one of the five outcome labels generated from
//!    imported artifacts, and switching known-limits stay disclosed.
//! 5. Provider/browser-handoff behavior is explicit and attributed, and no
//!    published guide allows hidden provider mutation or silent irreversibility.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    project_migration_switching_publication, MigrationSwitchingPublicationInput,
    MigrationSwitchingPublicationPacket, COMPATIBILITY_OUTCOME_LABELS, STABLE_TIERS,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PublicationFixture {
    #[allow(dead_code)]
    record_kind: String,
    #[allow(dead_code)]
    schema_version: u32,
    case_name: String,
    publication_input: MigrationSwitchingPublicationInput,
    expected: ExpectedPublication,
}

#[derive(Debug, Deserialize)]
struct ExpectedPublication {
    claimed_tier: String,
    effective_tier: String,
    support_claim_class: String,
    stable_claim: bool,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
    resolves_to_current_compatibility_table: bool,
    no_prose_only_stable_claim: bool,
    reversible: bool,
    attribution_complete: bool,
    compatibility_row_count: usize,
    known_limit_count: usize,
    blocking_known_limit_count: usize,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../fixtures/review/m4/publish-stable-migration-guides-compatibility-tables-and-switching",
    )
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("publication fixtures dir must exist")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn load_fixture(path: &Path) -> PublicationFixture {
    let payload = std::fs::read_to_string(path).expect("fixture must read");
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

#[test]
fn every_fixture_builds_validates_and_matches_expectations() {
    let paths = load_fixture_paths();
    assert!(
        !paths.is_empty(),
        "publication fixtures dir must not be empty"
    );

    for path in &paths {
        let fixture = load_fixture(path);
        let packet =
            MigrationSwitchingPublicationPacket::from_input(fixture.publication_input.clone())
                .unwrap_or_else(|err| panic!("fixture {:?} must build: {err}", fixture.case_name));

        // Re-validation and JSON round-trip must both hold.
        packet.validate().expect("packet must re-validate");
        let payload = serde_json::to_string(&packet).expect("serialize");
        let projection = project_migration_switching_publication(&payload)
            .unwrap_or_else(|err| panic!("fixture {:?} must project: {err}", fixture.case_name));

        let expected = &fixture.expected;
        assert_eq!(
            packet.claim.claimed_tier, expected.claimed_tier,
            "fixture {:?} claimed tier",
            fixture.case_name
        );
        assert_eq!(
            packet.claim.effective_tier, expected.effective_tier,
            "fixture {:?} effective tier",
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
            packet.inspection.resolves_to_current_compatibility_table,
            expected.resolves_to_current_compatibility_table,
            "fixture {:?} resolves to current table",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.no_prose_only_stable_claim, expected.no_prose_only_stable_claim,
            "fixture {:?} no prose-only stable claim",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.reversible, expected.reversible,
            "fixture {:?} reversible",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.attribution_complete, expected.attribution_complete,
            "fixture {:?} attribution complete",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.blocking_known_limit_count, expected.blocking_known_limit_count,
            "fixture {:?} blocking known-limit count",
            fixture.case_name
        );
        assert_eq!(
            projection.compatibility_row_count, expected.compatibility_row_count,
            "fixture {:?} compatibility row count",
            fixture.case_name
        );
        assert_eq!(
            projection.known_limit_count, expected.known_limit_count,
            "fixture {:?} known-limit count",
            fixture.case_name
        );

        // Cross-cutting invariants that must hold for every fixture.
        assert!(
            packet.no_prose_only_stable_claim(),
            "fixture {:?} must never imply a stable claim from prose",
            fixture.case_name
        );
        assert!(
            !packet.allows_hidden_provider_mutation
                && !packet.allows_unattributed_handoff
                && !packet.allows_irreversible_switch_without_disclosure,
            "fixture {:?} must not allow hidden authority or silent irreversibility",
            fixture.case_name
        );

        // Every compatibility row uses one of the five published outcome labels.
        for row in &packet.compatibility_table.rows {
            assert!(
                COMPATIBILITY_OUTCOME_LABELS.contains(&row.outcome_label.as_str()),
                "fixture {:?} row {} has an off-vocabulary outcome label",
                fixture.case_name,
                row.row_id
            );
        }

        // A stable effective tier must resolve to a current table and never be
        // downgraded.
        if STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()) {
            assert!(
                packet.inspection.resolves_to_current_compatibility_table,
                "fixture {:?} stable tier must resolve to a current table",
                fixture.case_name
            );
            assert!(
                !packet.claim.downgraded,
                "fixture {:?} stable tier must not be downgraded",
                fixture.case_name
            );
            assert!(
                packet.identity.reversible,
                "fixture {:?} stable tier must be reversible",
                fixture.case_name
            );
            assert!(
                packet.handoff_disclosure.is_complete(),
                "fixture {:?} stable tier must be fully attributed",
                fixture.case_name
            );
        }
    }
}

#[test]
fn stale_evidence_forces_a_narrowing() {
    let path = fixtures_dir().join("jetbrains_stale_evidence_auto_narrow.json");
    let fixture = load_fixture(&path);
    let packet = MigrationSwitchingPublicationPacket::from_input(fixture.publication_input)
        .expect("must build");
    assert!(
        packet.claim.downgraded,
        "stale evidence must narrow the claim"
    );
    assert!(
        !STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()),
        "stale evidence must not keep a stable tier"
    );
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"evidence_freshness_expired".to_string()));
}

#[test]
fn unsupported_core_and_blocking_limit_preserve_disclosure() {
    let path = fixtures_dir().join("vim_unsupported_core_narrows_to_preview.json");
    let fixture = load_fixture(&path);
    let packet = MigrationSwitchingPublicationPacket::from_input(fixture.publication_input)
        .expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    // The unsupported outcome and the blocking limit stay visible rather than
    // collapsing into a single green status.
    assert!(packet
        .compatibility_table
        .rows
        .iter()
        .any(|r| r.outcome_label == "unsupported" && r.is_core_capability));
    assert_eq!(packet.inspection.blocking_known_limit_count, 1);
}

#[test]
fn prose_only_stable_claim_is_narrowed() {
    let path = fixtures_dir().join("imported_user_prose_only_narrowed.json");
    let fixture = load_fixture(&path);
    let packet = MigrationSwitchingPublicationPacket::from_input(fixture.publication_input)
        .expect("must build");
    assert!(packet.claim.downgraded);
    assert!(!STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()));
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"prose_only_claim".to_string()));
}
