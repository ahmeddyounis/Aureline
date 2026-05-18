use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::Value;

use aureline_terminal::{
    TerminalExportPacket, TerminalSessionSummary, TERMINAL_EXPORT_PACKET_RECORD_KIND,
    TERMINAL_PROTOCOL_CONTRACT_SCHEMA_VERSION, TERMINAL_SESSION_SUMMARY_RECORD_KIND,
};

#[derive(Debug, Deserialize)]
struct Manifest {
    record_kind: String,
    schema_version: u32,
    fixture_set_id: String,
    required_coverage: RequiredCoverage,
    cases: Vec<ManifestCase>,
}

#[derive(Debug, Deserialize)]
struct RequiredCoverage {
    session_class_tokens: Vec<String>,
    live_authority_tokens: Vec<String>,
    clipboard_posture_tokens: Vec<String>,
    denial_reason_tokens: Vec<String>,
    export_class_tokens: Vec<String>,
    reconnect_drift_tokens: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ManifestCase {
    case_id: String,
    record_kind: String,
    #[allow(dead_code)]
    scenario: String,
    path: String,
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("manifest dir has at least three ancestors")
        .join("fixtures")
        .join("runtime")
        .join("m3")
        .join("terminal_protocol_and_restore")
}

fn load_json(path: &Path) -> Value {
    let bytes = std::fs::read(path)
        .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    serde_json::from_slice(&bytes)
        .unwrap_or_else(|err| panic!("parse {}: {err}", path.display()))
}

#[test]
fn terminal_protocol_and_restore_fixtures_validate_against_contract() {
    let root = fixture_root();
    let manifest_value = load_json(&root.join("manifest.json"));
    let manifest: Manifest = serde_json::from_value(manifest_value)
        .expect("manifest deserializes against expected shape");

    assert_eq!(manifest.record_kind, "terminal_protocol_and_restore_manifest");
    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.fixture_set_id, "terminal_protocol_and_restore");

    let mut observed_session_classes: BTreeSet<String> = BTreeSet::new();
    let mut observed_live_authorities: BTreeSet<String> = BTreeSet::new();
    let mut observed_clipboard_postures: BTreeSet<String> = BTreeSet::new();
    let mut observed_denials: BTreeSet<String> = BTreeSet::new();
    let mut observed_export_classes: BTreeSet<String> = BTreeSet::new();
    let mut observed_drifts: BTreeSet<String> = BTreeSet::new();

    for case in &manifest.cases {
        let path = root.join(&case.path);
        let value = load_json(&path);
        match case.record_kind.as_str() {
            TERMINAL_SESSION_SUMMARY_RECORD_KIND => {
                let summary: TerminalSessionSummary = serde_json::from_value(value).unwrap_or_else(
                    |err| {
                        panic!(
                            "case {} must parse as terminal_session_summary_record: {err}",
                            case.case_id
                        )
                    },
                );
                assert_eq!(summary.schema_version, TERMINAL_PROTOCOL_CONTRACT_SCHEMA_VERSION);
                let report = summary.validate();
                assert!(
                    report.passed,
                    "case {} failed validation: {:?}",
                    case.case_id, report.errors
                );
                observed_session_classes.insert(summary.session_class_token.clone());
                observed_live_authorities.insert(summary.live_authority_token.clone());
                observed_clipboard_postures.insert(summary.clipboard_posture_token.clone());
                observed_denials.insert(summary.denial_reason_token.clone());
                observed_drifts.insert(summary.recovery.reconnect_drift_token.clone());
            }
            TERMINAL_EXPORT_PACKET_RECORD_KIND => {
                let packet: TerminalExportPacket = serde_json::from_value(value).unwrap_or_else(
                    |err| {
                        panic!(
                            "case {} must parse as terminal_export_packet_record: {err}",
                            case.case_id
                        )
                    },
                );
                assert_eq!(packet.schema_version, TERMINAL_PROTOCOL_CONTRACT_SCHEMA_VERSION);
                let errors = packet.validate();
                assert!(
                    errors.is_empty(),
                    "case {} failed export packet validation: {errors:?}",
                    case.case_id
                );
                observed_export_classes.insert(packet.export_class_token.clone());
                observed_session_classes.insert(packet.summary.session_class_token.clone());
                observed_live_authorities.insert(packet.summary.live_authority_token.clone());
                observed_clipboard_postures
                    .insert(packet.summary.clipboard_posture_token.clone());
                observed_denials.insert(packet.summary.denial_reason_token.clone());
                observed_drifts.insert(packet.summary.recovery.reconnect_drift_token.clone());
            }
            other => panic!("case {} declares unknown record_kind {other}", case.case_id),
        }
    }

    for token in &manifest.required_coverage.session_class_tokens {
        assert!(
            observed_session_classes.contains(token),
            "missing session_class coverage token {token}"
        );
    }
    for token in &manifest.required_coverage.live_authority_tokens {
        assert!(
            observed_live_authorities.contains(token),
            "missing live_authority coverage token {token}"
        );
    }
    for token in &manifest.required_coverage.clipboard_posture_tokens {
        assert!(
            observed_clipboard_postures.contains(token),
            "missing clipboard_posture coverage token {token}"
        );
    }
    for token in &manifest.required_coverage.denial_reason_tokens {
        assert!(
            observed_denials.contains(token),
            "missing denial_reason coverage token {token}"
        );
    }
    for token in &manifest.required_coverage.export_class_tokens {
        assert!(
            observed_export_classes.contains(token),
            "missing export_class coverage token {token}"
        );
    }
    for token in &manifest.required_coverage.reconnect_drift_tokens {
        assert!(
            observed_drifts.contains(token),
            "missing reconnect_drift coverage token {token}"
        );
    }
}
