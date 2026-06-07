use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_editor::{
    transfer_safety_corpus, TransferActionClass, TransferSafetyPacket, TransferSurfaceClass,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    record_kind: String,
    schema_version: u32,
    #[allow(dead_code)]
    as_of: String,
    scenario_count: usize,
    scenarios: Vec<ManifestScenario>,
}

#[derive(Debug, Deserialize)]
struct ManifestScenario {
    scenario_id: String,
    fixture_filename: String,
    expected_surface: String,
    expected_action: String,
    expects_sensitive_review: bool,
    expects_drop_preview: bool,
    expects_paste_guardrail: bool,
    expects_large_transfer: bool,
    expects_named_undo_group: bool,
}

#[test]
fn corpus_manifest_matches_live_scenarios() {
    let corpus = transfer_safety_corpus();
    let manifest = load_manifest();

    assert_eq!(manifest.record_kind, "transfer_safety_corpus_manifest");
    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.scenario_count, corpus.len());
    assert_eq!(manifest.scenarios.len(), corpus.len());

    for (live, manifest_entry) in corpus.iter().zip(&manifest.scenarios) {
        assert_eq!(live.scenario_id, manifest_entry.scenario_id);
        assert_eq!(live.fixture_filename, manifest_entry.fixture_filename);
        assert_eq!(
            live.expected_surface.as_str(),
            manifest_entry.expected_surface
        );
        assert_eq!(
            live.expected_action.as_str(),
            manifest_entry.expected_action
        );
        assert_eq!(
            live.expects_sensitive_review,
            manifest_entry.expects_sensitive_review
        );
        assert_eq!(
            live.expects_drop_preview,
            manifest_entry.expects_drop_preview
        );
        assert_eq!(
            live.expects_paste_guardrail,
            manifest_entry.expects_paste_guardrail
        );
        assert_eq!(
            live.expects_large_transfer,
            manifest_entry.expects_large_transfer
        );
        assert_eq!(
            live.expects_named_undo_group,
            manifest_entry.expects_named_undo_group
        );
    }
}

#[test]
fn every_fixture_is_contract_valid() {
    let fixture_dir = repo_root()
        .join("fixtures/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails");
    let corpus = transfer_safety_corpus();

    for scenario in &corpus {
        let path = fixture_dir.join(scenario.fixture_filename);
        let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
        let packet: TransferSafetyPacket =
            serde_json::from_str(&raw).unwrap_or_else(|err| panic!("parse {path:?}: {err}"));

        assert!(
            packet.is_contract_valid(),
            "{}: contract findings: {:?}",
            scenario.scenario_id,
            packet.contract_findings()
        );
        assert_eq!(
            packet.surface, scenario.expected_surface,
            "{}: surface mismatch",
            scenario.scenario_id
        );
        assert_eq!(
            packet.action, scenario.expected_action,
            "{}: action mismatch",
            scenario.scenario_id
        );
        assert_eq!(
            packet.sensitive_review.is_some(),
            scenario.expects_sensitive_review,
            "{}: sensitive review mismatch",
            scenario.scenario_id
        );
        assert_eq!(
            packet.drop_preview.is_some(),
            scenario.expects_drop_preview,
            "{}: drop preview mismatch",
            scenario.scenario_id
        );
        assert_eq!(
            packet.paste_guardrail.is_some(),
            scenario.expects_paste_guardrail,
            "{}: paste guardrail mismatch",
            scenario.scenario_id
        );
        assert_eq!(
            packet.large_transfer.is_some(),
            scenario.expects_large_transfer,
            "{}: large transfer mismatch",
            scenario.scenario_id
        );
        assert_eq!(
            packet.undo_group.as_ref().is_some_and(|undo| undo.named),
            scenario.expects_named_undo_group,
            "{}: named undo mismatch",
            scenario.scenario_id
        );
    }
}

#[test]
fn fixture_files_match_live_packet_serialization() {
    let fixture_dir = repo_root()
        .join("fixtures/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails");
    let corpus = transfer_safety_corpus();

    for scenario in &corpus {
        let path = fixture_dir.join(scenario.fixture_filename);
        let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
        let from_disk: serde_json::Value =
            serde_json::from_str(&raw).unwrap_or_else(|err| panic!("parse {path:?}: {err}"));
        let live = serde_json::to_value(scenario.packet())
            .unwrap_or_else(|err| panic!("serialize {}: {err}", scenario.scenario_id));
        assert_eq!(
            from_disk, live,
            "{}: fixture on disk does not match live packet serialization",
            scenario.scenario_id
        );
    }
}

#[test]
fn corpus_covers_required_surfaces() {
    let corpus = transfer_safety_corpus();
    let surfaces: BTreeSet<_> = corpus
        .iter()
        .map(|scenario| scenario.expected_surface)
        .collect();
    for required in [
        TransferSurfaceClass::Editor,
        TransferSurfaceClass::Terminal,
        TransferSurfaceClass::Notebook,
        TransferSurfaceClass::Docs,
        TransferSurfaceClass::Support,
        TransferSurfaceClass::Shell,
    ] {
        assert!(
            surfaces.contains(&required),
            "missing {required:?} scenario"
        );
    }
}

#[test]
fn corpus_covers_required_transfer_risks() {
    let corpus = transfer_safety_corpus();
    assert!(
        corpus.iter().any(|scenario| {
            let packet = scenario.packet();
            packet.action == TransferActionClass::Copy
                && packet.representation.rendered_copy_available
                && packet.representation.raw_copy_available
        }),
        "missing rich-copy versus raw-copy fixture"
    );
    assert!(
        corpus
            .iter()
            .any(|scenario| scenario.packet().sensitive_review.is_some()),
        "missing sensitive-copy preview fixture"
    );
    assert!(
        corpus.iter().any(|scenario| {
            let packet = scenario.packet();
            packet.summary.contains("OSC 52")
                && packet
                    .boundary_context
                    .as_ref()
                    .is_some_and(|boundary| boundary.shown_before_commit)
        }),
        "missing remote clipboard policy fixture"
    );
    assert!(
        corpus
            .iter()
            .any(|scenario| scenario.packet().paste_guardrail.is_some()),
        "missing multiline paste guardrail fixture"
    );
    assert!(
        corpus.iter().any(|scenario| {
            scenario
                .packet()
                .drop_preview
                .as_ref()
                .is_some_and(|preview| {
                    preview.insertion_indicator_visible
                        && preview.modifier_cues_visible
                        && preview.keyboard_route_available
                })
        }),
        "missing drag/drop verb cue fixture"
    );
    assert!(
        corpus
            .iter()
            .any(|scenario| scenario.packet().large_transfer.is_some()),
        "missing large-transfer feedback fixture"
    );
    assert!(
        corpus.iter().any(|scenario| {
            scenario
                .packet()
                .undo_group
                .as_ref()
                .is_some_and(|undo| undo.named && !undo.history_surfaces.is_empty())
        }),
        "missing named undo-group lineage fixture"
    );
}

fn load_manifest() -> Manifest {
    let path = repo_root()
        .join("fixtures/ux/m4/stabilize-clipboard-dragdrop-rich-content-and-paste-guardrails")
        .join("manifest.json");
    let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("repo root must resolve")
}
