//! Fixture-driven coverage for the docs-pack manager packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_docs::{
    current_stable_docs_pack_manager_export, current_stable_docs_pack_manager_packet,
    seeded_stable_docs_pack_manager_input, DocsPackLifecycleFlow, DocsPackManagerFindingKind,
    DocsPackManagerPacket, DocsPackManagerPacketInput, DocsPackManagerProfile,
    DocsPackManagerPromotionState, DocsPackManagerSupportExport, DOCS_PACK_MANAGER_ARTIFACT_REF,
    DOCS_PACK_MANAGER_DOC_REF, DOCS_PACK_MANAGER_FIXTURE_DIR, DOCS_PACK_MANAGER_SCHEMA_REF,
    DOCS_PACK_MANAGER_SUMMARY_REF,
};
use serde::Deserialize;

/// Stable export id pinned for the checked-in support export.
const SUPPORT_EXPORT_ID: &str = "support-export:docs_pack_manager:001";
/// Stable export timestamp pinned for the checked-in support export.
const SUPPORT_EXPORTED_AT: &str = "2026-06-26T00:00:00Z";

#[derive(Debug, Deserialize)]
struct ManagerFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DocsPackManagerPacketInput,
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

fn load_fixture(file_name: &str) -> ManagerFixture {
    let path = repo_root()
        .join(DOCS_PACK_MANAGER_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("fixture {path:?} must read: {error}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|error| panic!("fixture {path:?} must parse: {error}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(fixture.record_kind, "docs_pack_manager_case");
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let packet = DocsPackManagerPacket::materialize(fixture.input);
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
    assert_exists(DOCS_PACK_MANAGER_DOC_REF);
    assert_exists(DOCS_PACK_MANAGER_SCHEMA_REF);
    assert_exists(DOCS_PACK_MANAGER_ARTIFACT_REF);
    assert_exists(DOCS_PACK_MANAGER_SUMMARY_REF);
    assert_exists(DOCS_PACK_MANAGER_FIXTURE_DIR);
}

#[test]
fn baseline_stable_fixture_certifies_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn manager_row_hides_mirror_source_fixture_blocks_stable() {
    assert_fixture_matches("manager_row_hides_mirror_source_blocks_stable.json");
}

#[test]
fn unavailable_payload_hidden_fixture_blocks_stable() {
    assert_fixture_matches("unavailable_payload_hidden_blocks_stable.json");
}

#[test]
fn mirror_offline_degraded_fixture_blocks_stable() {
    assert_fixture_matches("mirror_offline_degraded_to_cache_blocks_stable.json");
}

#[test]
fn import_export_continuity_lost_fixture_blocks_stable() {
    assert_fixture_matches("import_export_continuity_lost_blocks_stable.json");
}

#[test]
fn manager_action_reason_missing_fixture_blocks_stable() {
    assert_fixture_matches("manager_action_reason_missing_blocks_stable.json");
}

#[test]
fn lifecycle_flow_origin_mismatch_fixture_blocks_stable() {
    assert_fixture_matches("lifecycle_flow_origin_mismatch_blocks_stable.json");
}

#[test]
fn profile_projection_drops_truth_fixture_blocks_stable() {
    assert_fixture_matches("profile_projection_drops_truth_blocks_stable.json");
}

#[test]
fn checked_in_packet_manages_packs_across_every_profile() {
    let packet = current_stable_docs_pack_manager_packet()
        .expect("stable docs-pack manager packet validates");
    assert_eq!(
        packet.promotion_state,
        DocsPackManagerPromotionState::Stable
    );
    assert!(packet.validate().is_empty());

    for flow in DocsPackLifecycleFlow::REQUIRED {
        assert!(
            packet.lifecycle_flow_tokens().contains(&flow.as_str()),
            "stable packet must manage a {} pack",
            flow.as_str()
        );
    }
    for profile in DocsPackManagerProfile::REQUIRED {
        assert!(
            packet.has_projection_for(profile),
            "stable packet must project the {} profile",
            profile.as_str()
        );
    }
}

#[test]
fn checked_in_support_export_matches_seed() {
    let path = repo_root().join(DOCS_PACK_MANAGER_ARTIFACT_REF);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("artifact {path:?} must read: {error}"));
    let from_file: DocsPackManagerSupportExport = serde_json::from_str(&body)
        .unwrap_or_else(|error| panic!("artifact {path:?} must parse: {error}"));
    let from_seed = DocsPackManagerPacket::materialize(seeded_stable_docs_pack_manager_input())
        .support_export(SUPPORT_EXPORT_ID, SUPPORT_EXPORTED_AT);
    assert_eq!(
        from_file, from_seed,
        "checked-in support export drifted from the in-code seed; regenerate with \
         `cargo run -q -p aureline-docs --bin aureline_docs_pack_manager -- support-export > {}`",
        DOCS_PACK_MANAGER_ARTIFACT_REF,
    );
}

#[test]
fn checked_in_support_export_is_export_safe() {
    let export =
        current_stable_docs_pack_manager_export().expect("checked support export validates");
    assert!(export.is_export_safe());
    assert!(export.export_packet.validate().is_empty());
}

#[test]
fn closed_finding_tokens_are_pinned() {
    assert_eq!(
        DocsPackManagerFindingKind::ManagerRowHidesManifestTruth.as_str(),
        "manager_row_hides_manifest_truth"
    );
    assert_eq!(
        DocsPackManagerFindingKind::UnavailablePayloadHidden.as_str(),
        "unavailable_payload_hidden"
    );
    assert_eq!(
        DocsPackManagerFindingKind::MirrorOfflineDegraded.as_str(),
        "mirror_offline_degraded"
    );
    assert_eq!(
        DocsPackManagerFindingKind::ImportExportContinuityLost.as_str(),
        "import_export_continuity_lost"
    );
    assert_eq!(
        DocsPackManagerFindingKind::RequiredManagerActionMissing.as_str(),
        "required_manager_action_missing"
    );
}
