//! Fixture-driven coverage for the derived-explanation citation-sets packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_docs::{
    current_stable_derived_explanation_citation_export,
    current_stable_derived_explanation_citation_packet,
    seeded_stable_derived_explanation_citation_input, DerivedExplanationCitationPacket,
    DerivedExplanationCitationPacketInput, DerivedExplanationCitationPromotionState,
    DerivedExplanationCitationSupportExport, DerivedExplanationSurface,
    DERIVED_EXPLANATION_CITATION_ARTIFACT_REF, DERIVED_EXPLANATION_CITATION_DOC_REF,
    DERIVED_EXPLANATION_CITATION_FIXTURE_DIR, DERIVED_EXPLANATION_CITATION_SCHEMA_REF,
    DERIVED_EXPLANATION_CITATION_SUMMARY_REF,
};
use serde::Deserialize;

/// Stable export id pinned for the checked-in support export.
const SUPPORT_EXPORT_ID: &str = "support-export:derived_explanation_citation_sets:001";
/// Stable export timestamp pinned for the checked-in support export.
const SUPPORT_EXPORTED_AT: &str = "2026-06-26T00:00:00Z";

#[derive(Debug, Deserialize)]
struct CitationFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DerivedExplanationCitationPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    #[serde(default)]
    expected_finding_kinds: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

fn assert_exists(rel: &str) {
    let path = repo_root().join(rel);
    assert!(
        path.exists(),
        "expected path to exist on disk: {} ({})",
        rel,
        path.display()
    );
}

fn load_fixture(file_name: &str) -> CitationFixture {
    let path = repo_root()
        .join(DERIVED_EXPLANATION_CITATION_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("fixture {path:?} must read: {error}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|error| panic!("fixture {path:?} must parse: {error}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind,
        "derived_explanation_citation_sets_case"
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let packet = DerivedExplanationCitationPacket::materialize(fixture.input);
    assert_eq!(
        packet.promotion_state.as_str(),
        fixture.expect.promotion_state,
        "fixture {} expected promotion {}, got {:?}; findings: {:?}",
        fixture.case_name,
        fixture.expect.promotion_state,
        packet.promotion_state,
        packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect::<Vec<_>>()
    );

    if !fixture.expect.expected_finding_kinds.is_empty() {
        let observed: BTreeSet<&str> = packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect();
        for expected in &fixture.expect.expected_finding_kinds {
            assert!(
                observed.contains(expected.as_str()),
                "fixture {} expected finding {expected}; observed {:?}",
                fixture.case_name,
                observed
            );
        }
    }
}

#[test]
fn doc_schema_artifact_and_fixtures_exist_on_disk() {
    assert_exists(DERIVED_EXPLANATION_CITATION_DOC_REF);
    assert_exists(DERIVED_EXPLANATION_CITATION_SCHEMA_REF);
    assert_exists(DERIVED_EXPLANATION_CITATION_ARTIFACT_REF);
    assert_exists(DERIVED_EXPLANATION_CITATION_SUMMARY_REF);
    assert_exists(DERIVED_EXPLANATION_CITATION_FIXTURE_DIR);
}

#[test]
fn baseline_stable_fixture_certifies_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn direct_citation_without_evidence_fixture_blocks_stable() {
    assert_fixture_matches("direct_citation_without_evidence_blocks_stable.json");
}

#[test]
fn inference_without_label_fixture_blocks_stable() {
    assert_fixture_matches("inference_without_label_blocks_stable.json");
}

#[test]
fn inference_claims_authority_fixture_blocks_stable() {
    assert_fixture_matches("inference_claims_authority_blocks_stable.json");
}

#[test]
fn redaction_drops_basis_fixture_blocks_stable() {
    assert_fixture_matches("redaction_drops_basis_blocks_stable.json");
}

#[test]
fn surface_coverage_missing_fixture_blocks_stable() {
    assert_fixture_matches("surface_coverage_missing_blocks_stable.json");
}

#[test]
fn support_export_drops_basis_fixture_blocks_stable() {
    assert_fixture_matches("support_export_drops_basis_blocks_stable.json");
}

#[test]
fn projection_drops_reuse_fixture_blocks_stable() {
    assert_fixture_matches("projection_drops_reuse_blocks_stable.json");
}

#[test]
fn stale_citation_fixture_narrows_below_stable() {
    assert_fixture_matches("stale_citation_narrows_below_stable.json");
}

#[test]
fn speculative_inference_fixture_narrows_below_stable() {
    assert_fixture_matches("speculative_inference_narrows_below_stable.json");
}

#[test]
fn checked_in_packet_binds_every_required_surface() {
    let packet = current_stable_derived_explanation_citation_packet()
        .expect("stable derived-explanation citation packet validates");
    assert_eq!(
        packet.promotion_state,
        DerivedExplanationCitationPromotionState::Stable
    );
    assert!(packet.validate().is_empty());

    for surface in DerivedExplanationSurface::REQUIRED {
        assert!(
            packet.covered_surfaces().contains(&surface),
            "stable packet must bind a citation set on the {} surface",
            surface.as_str()
        );
        assert!(
            packet.has_projection_for(surface),
            "stable packet must project the {} surface",
            surface.as_str()
        );
    }

    // The support export preserves the citation basis of every set.
    let support_projection = packet
        .consumer_projections
        .iter()
        .find(|projection| projection.surface == DerivedExplanationSurface::SupportExportNote)
        .expect("support-export projection present");
    for set in &packet.citation_sets {
        assert!(
            support_projection
                .citation_set_id_refs
                .contains(&set.citation_set_id),
            "support export must preserve citation set {}",
            set.citation_set_id
        );
    }
}

#[test]
fn checked_in_support_export_matches_seed() {
    let path = repo_root().join(DERIVED_EXPLANATION_CITATION_ARTIFACT_REF);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("artifact {path:?} must read: {error}"));
    let from_file: DerivedExplanationCitationSupportExport = serde_json::from_str(&body)
        .unwrap_or_else(|error| panic!("artifact {path:?} must parse: {error}"));
    let from_seed = DerivedExplanationCitationPacket::materialize(
        seeded_stable_derived_explanation_citation_input(),
    )
    .support_export(SUPPORT_EXPORT_ID, SUPPORT_EXPORTED_AT);
    assert_eq!(
        from_file, from_seed,
        "checked-in support export drifted from the in-code seed; regenerate with \
         `cargo run -q -p aureline-docs --bin aureline_docs_derived_explanation_citation_sets -- support-export > {}`",
        DERIVED_EXPLANATION_CITATION_ARTIFACT_REF,
    );
}

#[test]
fn checked_in_support_export_is_export_safe() {
    let export = current_stable_derived_explanation_citation_export()
        .expect("checked support export validates");
    assert!(export.is_export_safe());
    assert!(export.export_packet.validate().is_empty());
}
