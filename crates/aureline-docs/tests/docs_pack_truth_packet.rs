//! Fixture-driven coverage for the stable docs-pack truth packet
//! (docs-pack manifests, mirror/offline truth, stale-example detection, and
//! citation-set export).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_docs::{
    current_stable_docs_pack_truth_packet, seeded_stable_docs_pack_truth_packet_input,
    DocsPackConsumerSurface, DocsPackFindingKind, DocsPackLocalAvailability,
    DocsPackPromotionState, DocsPackSourceClass, DocsPackTruthPacket, DocsPackTruthPacketInput,
    DocsRenderMode, DOCS_PACK_TRUTH_PACKET_ARTIFACT_DOC_REF, DOCS_PACK_TRUTH_PACKET_ARTIFACT_REF,
    DOCS_PACK_TRUTH_PACKET_DOC_REF, DOCS_PACK_TRUTH_PACKET_FIXTURE_DIR,
    DOCS_PACK_TRUTH_PACKET_MILESTONE_DOC_REF, DOCS_PACK_TRUTH_PACKET_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DocsPackFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DocsPackTruthPacketInput,
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

fn load_fixture(file_name: &str) -> DocsPackFixture {
    let path = repo_root()
        .join(DOCS_PACK_TRUTH_PACKET_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "docs_pack_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let packet = DocsPackTruthPacket::materialize(fixture.input.clone());
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
        for kind in &fixture.expect.expected_finding_kinds {
            assert!(
                observed.contains(kind.as_str()),
                "fixture {} expected finding kind {kind}; observed {:?}",
                fixture.case_name,
                observed
            );
        }
    }
}

#[test]
fn doc_fixture_schema_and_artifact_exist_on_disk() {
    assert_exists(DOCS_PACK_TRUTH_PACKET_DOC_REF);
    assert_exists(DOCS_PACK_TRUTH_PACKET_MILESTONE_DOC_REF);
    assert_exists(DOCS_PACK_TRUTH_PACKET_ARTIFACT_DOC_REF);
    assert_exists(DOCS_PACK_TRUTH_PACKET_FIXTURE_DIR);
    assert_exists(DOCS_PACK_TRUTH_PACKET_ARTIFACT_REF);
    assert_exists(DOCS_PACK_TRUTH_PACKET_SCHEMA_REF);
}

#[test]
fn baseline_stable_fixture_certifies_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn offline_pack_loses_signer_identity_fixture_blocks_stable() {
    assert_fixture_matches("offline_pack_loses_signer_identity_blocks_stable.json");
}

#[test]
fn nearby_version_dropped_fixture_blocks_stable() {
    assert_fixture_matches("nearby_version_dropped_collapses_stale_state_blocks_stable.json");
}

#[test]
fn citation_set_bundles_raw_pack_fixture_blocks_stable() {
    assert_fixture_matches("citation_set_bundles_raw_pack_blocks_stable.json");
}

#[test]
fn stale_suppression_loses_attribution_fixture_blocks_stable() {
    assert_fixture_matches("stale_suppression_loses_attribution_blocks_stable.json");
}

#[test]
fn consumer_projection_drops_render_mode_fixture_blocks_stable() {
    assert_fixture_matches("consumer_projection_drops_render_mode_blocks_stable.json");
}

#[test]
fn quarantined_finding_collapsed_fixture_blocks_stable() {
    assert_fixture_matches("quarantined_finding_collapsed_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_required_surfaces() {
    let packet = current_stable_docs_pack_truth_packet()
        .expect("checked-in docs-pack truth packet validates");
    assert_eq!(packet.promotion_state, DocsPackPromotionState::Stable);
    assert!(packet.validate().is_empty());

    for surface in DocsPackConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }

    let source_class_tokens: BTreeSet<&str> = packet.source_class_tokens().into_iter().collect();
    for required in DocsPackSourceClass::REQUIRED {
        assert!(
            source_class_tokens.contains(required.as_str()),
            "checked-in packet must cover source class {}",
            required.as_str()
        );
    }

    let render_mode_tokens: BTreeSet<&str> = packet.render_mode_tokens().into_iter().collect();
    for required in DocsRenderMode::REQUIRED {
        assert!(
            render_mode_tokens.contains(required.as_str()),
            "checked-in packet must cover render mode {}",
            required.as_str()
        );
    }

    let availability_tokens: BTreeSet<&str> = packet
        .manifests
        .iter()
        .map(|manifest| manifest.local_availability.as_str())
        .collect();
    for required in DocsPackLocalAvailability::REQUIRED {
        assert!(
            availability_tokens.contains(required.as_str()),
            "checked-in packet must cover local-availability posture {}",
            required.as_str()
        );
    }
}

#[test]
fn artifact_file_matches_seeded_packet() {
    let path = repo_root().join(DOCS_PACK_TRUTH_PACKET_ARTIFACT_REF);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("artifact {path:?} must read: {err}"));
    let from_file: DocsPackTruthPacket = serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("artifact {path:?} must parse: {err}"));
    let from_seed = DocsPackTruthPacket::materialize(seeded_stable_docs_pack_truth_packet_input());
    assert_eq!(
        from_file, from_seed,
        "checked-in docs-pack truth packet drifted from the in-code seed; \
         regenerate with `cargo run -q -p aureline-docs --bin aureline_docs_pack_truth_packet -- packet > artifacts/search/m4/docs_pack_truth_packet.json`",
    );
}

#[test]
fn support_export_preserves_packet_safely() {
    let packet = current_stable_docs_pack_truth_packet()
        .expect("checked-in docs-pack truth packet validates");
    let export = packet.support_export(
        "support-export:docs_pack_truth:test",
        "2026-05-26T12:30:00Z",
    );
    assert!(export.is_export_safe());
    assert_eq!(export.export_packet_id_ref, packet.packet_id);
    assert_eq!(export.export_packet, packet);
}

#[test]
fn closed_docs_pack_tokens_are_pinned() {
    assert_eq!(DocsPackSourceClass::ProjectDocs.as_str(), "project_docs");
    assert_eq!(
        DocsPackSourceClass::MirroredOfficialDocs.as_str(),
        "mirrored_official_docs"
    );
    assert_eq!(
        DocsPackSourceClass::ExtensionDocsPack.as_str(),
        "extension_docs_pack"
    );
    assert_eq!(
        DocsPackLocalAvailability::MirrorOfflinePinned.as_str(),
        "mirror_offline_pinned"
    );
    assert_eq!(
        DocsPackLocalAvailability::Quarantined.as_str(),
        "quarantined"
    );
    assert_eq!(
        DocsRenderMode::BrowserHandoffOnly.as_str(),
        "browser_handoff_only"
    );
    assert_eq!(DocsRenderMode::NotRendered.as_str(), "not_rendered");
    assert_eq!(
        DocsPackFindingKind::PackIdentityLostWhenOffline.as_str(),
        "pack_identity_lost_when_offline"
    );
    assert_eq!(
        DocsPackFindingKind::StaleStateCollapsed.as_str(),
        "stale_state_collapsed"
    );
    assert_eq!(
        DocsPackFindingKind::CitationSetBundlesRawPack.as_str(),
        "citation_set_bundles_raw_pack"
    );
    assert_eq!(
        DocsPackConsumerSurface::MirrorOfflineConsole.as_str(),
        "mirror_offline_console"
    );
    assert_eq!(
        DocsPackConsumerSurface::CitationDrawer.as_str(),
        "citation_drawer"
    );
}
