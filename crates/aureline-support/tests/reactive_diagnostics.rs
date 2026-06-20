//! Replay and coverage gate for the reactive-diagnostics packet and fixtures.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_support::{
    compile_reactive_diagnostics_support_export_envelope, seeded_reactive_diagnostics_fixtures,
    seeded_reactive_diagnostics_packet, validate_reactive_diagnostics_fixture,
    validate_reactive_diagnostics_packet, ReactiveDiagnosticsFixture, ReactiveDiagnosticsPacket,
    ReactiveStateReasonCode, REACTIVE_DIAGNOSTICS_DOC_REF, REACTIVE_DIAGNOSTICS_FIXTURE_DIR,
    REACTIVE_DIAGNOSTICS_FIXTURE_MANIFEST_REF, REACTIVE_DIAGNOSTICS_PACKET_REF,
    REACTIVE_DIAGNOSTICS_REPORT_REF, REACTIVE_DIAGNOSTICS_RUNBOOK_REF,
    REACTIVE_DIAGNOSTICS_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> ReactiveDiagnosticsPacket {
    let path = repo_root().join(REACTIVE_DIAGNOSTICS_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<ReactiveDiagnosticsFixture> {
    let dir = repo_root().join(REACTIVE_DIAGNOSTICS_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: ReactiveDiagnosticsFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        out.push(fixture);
    }
    out.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert!(!out.is_empty(), "expected at least one fixture");
    out
}

#[test]
fn packet_matches_seeded_projection_and_validates() {
    let packet = load_packet();
    let seeded = seeded_reactive_diagnostics_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_reactive_diagnostics_packet(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let packet = load_packet();
    let on_disk = load_fixtures();
    let mut seeded = seeded_reactive_diagnostics_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_reactive_diagnostics_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        REACTIVE_DIAGNOSTICS_SCHEMA_REF,
        REACTIVE_DIAGNOSTICS_DOC_REF,
        REACTIVE_DIAGNOSTICS_PACKET_REF,
        REACTIVE_DIAGNOSTICS_REPORT_REF,
        REACTIVE_DIAGNOSTICS_RUNBOOK_REF,
        REACTIVE_DIAGNOSTICS_FIXTURE_MANIFEST_REF,
    ] {
        let path = root.join(rel);
        assert!(
            path.exists(),
            "required file must exist: {}",
            path.display()
        );
    }
    assert!(
        root.join(REACTIVE_DIAGNOSTICS_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn every_reason_code_has_a_probe_and_required_codes_are_named() {
    let packet = load_packet();
    let probed: BTreeSet<_> = packet
        .doctor_probes
        .iter()
        .map(|probe| probe.reason_code)
        .collect();
    for code in ReactiveStateReasonCode::all() {
        assert!(
            probed.contains(&code),
            "packet must carry a doctor probe for reason code {}",
            code.as_token()
        );
    }
    for required in ReactiveStateReasonCode::required_named() {
        assert!(
            probed.contains(&required),
            "packet must name required reason code {} directly",
            required.as_token()
        );
    }
}

#[test]
fn fixtures_cover_required_troubleshooting_scenarios() {
    use aureline_support::ReactiveDiagnosticsTroubleshootingScenario as Scenario;
    let scenarios: BTreeSet<_> = load_fixtures().iter().map(|fx| fx.scenario).collect();
    for required in [
        Scenario::EpochDrift,
        Scenario::InvalidationStorm,
        Scenario::LaggingConsumer,
        Scenario::PartialScopeStale,
    ] {
        assert!(
            scenarios.contains(&required),
            "fixture corpus must reproduce scenario {}",
            required.as_token()
        );
    }
}

#[test]
fn support_export_is_metadata_safe_and_complete() {
    let envelope = compile_reactive_diagnostics_support_export_envelope(
        "envelope:reactive_diagnostics:replay",
        "2026-06-19T09:10:00Z",
    )
    .expect("support export compiles");
    assert!(envelope.is_export_safe());
    let packet = load_packet();
    assert_eq!(envelope.rows.len(), packet.doctor_probes.len());
    for row in &envelope.rows {
        assert!(row.raw_payload_excluded);
        assert!(row.ambient_authority_excluded);
        assert_eq!(row.finding_code, row.reason_code.finding_code());
    }
}
