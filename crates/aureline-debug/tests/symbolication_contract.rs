//! Integration tests for the embedded symbolication contract packet.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_debug::{
    current_symbolication_contract, BuildMatchState, SymbolicationFidelityLabel,
    SymbolicationReportRow, SymbolicationSurfaceKind, SYMBOLICATION_CONTRACT_ARTIFACT_DOC_REF,
    SYMBOLICATION_CONTRACT_DOC_REF, SYMBOLICATION_CONTRACT_FIXTURE_DIR,
    SYMBOLICATION_CONTRACT_PACKET_PATH, SYMBOLICATION_CONTRACT_RECORD_KIND,
    SYMBOLICATION_CONTRACT_SCHEMA_REF, SYMBOLICATION_CONTRACT_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = current_symbolication_contract().expect("embedded packet must parse");
    assert_eq!(packet.record_kind, SYMBOLICATION_CONTRACT_RECORD_KIND);
    assert_eq!(packet.schema_version, SYMBOLICATION_CONTRACT_SCHEMA_VERSION);
    assert_eq!(packet.doc_ref, SYMBOLICATION_CONTRACT_DOC_REF);
    assert_eq!(packet.schema_ref, SYMBOLICATION_CONTRACT_SCHEMA_REF);
    assert_eq!(
        packet.artifact_doc_ref,
        SYMBOLICATION_CONTRACT_ARTIFACT_DOC_REF
    );
    assert!(
        packet.validate().is_empty(),
        "expected no validation violations, got {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_summary_matches_computed_summary() {
    let packet = current_symbolication_contract().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn embedded_packet_covers_required_surface_families() {
    let packet = current_symbolication_contract().expect("embedded packet must parse");
    for surface in [
        SymbolicationSurfaceKind::DebugFrameStack,
        SymbolicationSurfaceKind::CrashDumpCard,
        SymbolicationSurfaceKind::ProfilerHotspotWorkspace,
        SymbolicationSurfaceKind::ProfilerTraceViewer,
        SymbolicationSurfaceKind::PreviewRuntimeFrame,
        SymbolicationSurfaceKind::BrowserRuntimeStack,
        SymbolicationSurfaceKind::SupportExportPacket,
        SymbolicationSurfaceKind::IncidentCrashCard,
    ] {
        assert!(
            packet
                .surfaces
                .iter()
                .any(|row| row.surface_kind == surface),
            "missing surface kind {}",
            surface.as_str()
        );
    }
}

#[test]
fn report_labels_preserve_exact_build_mismatch_honesty() {
    let packet = current_symbolication_contract().expect("embedded packet must parse");
    let mismatch_report = packet
        .reports
        .iter()
        .find(|report| report.build_match_state == BuildMatchState::MismatchedCandidateRejected)
        .expect("mismatch report");
    assert_eq!(
        mismatch_report.fidelity_label,
        SymbolicationFidelityLabel::Unresolved
    );
    assert_eq!(mismatch_report.unresolved_frame_count, 7);
}

#[test]
fn checked_in_docs_schema_artifact_and_fixtures_exist() {
    let root = repo_root();
    for rel in [
        SYMBOLICATION_CONTRACT_DOC_REF,
        SYMBOLICATION_CONTRACT_SCHEMA_REF,
        SYMBOLICATION_CONTRACT_ARTIFACT_DOC_REF,
        SYMBOLICATION_CONTRACT_PACKET_PATH,
        "fixtures/debug/symbolication/README.md",
        "fixtures/debug/symbolication/manifest.yaml",
        "fixtures/debug/symbolication/packet.json",
        "fixtures/debug/symbolication/exact_local_report.json",
        "fixtures/debug/symbolication/approximate_mirrored_report.json",
        "fixtures/debug/symbolication/symbol_only_report.json",
        "fixtures/debug/symbolication/unresolved_mismatch_report.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}

#[test]
fn checked_in_report_fixtures_parse() {
    let root = repo_root().join(SYMBOLICATION_CONTRACT_FIXTURE_DIR);
    for fixture in [
        "exact_local_report.json",
        "approximate_mirrored_report.json",
        "symbol_only_report.json",
        "unresolved_mismatch_report.json",
    ] {
        let path = root.join(fixture);
        let bytes =
            fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
        let report: SymbolicationReportRow = serde_json::from_slice(&bytes)
            .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()));
        assert!(!report.report_id.is_empty());
    }
}
