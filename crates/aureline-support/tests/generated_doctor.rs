//! Replay and parity gate for the generated-artifact Project Doctor packet.
//!
//! These tests prove the checked-in proof packet and fixture corpus match the
//! seeded projection, that the lane's files exist on disk, and that the
//! docs/help surface reuses the exact controlled vocabulary the packet emits.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_support::generated_doctor::{
    compile_generated_doctor_support_export, seeded_generated_doctor_findings_packet,
    seeded_generated_doctor_fixtures, validate_generated_doctor_findings_packet,
    validate_generated_doctor_fixture, GeneratedDoctorFindingClass, GeneratedDoctorFindingsPacket,
    GeneratedDoctorFixture, GENERATED_DOCTOR_DOC_REF, GENERATED_DOCTOR_FIXTURE_DIR,
    GENERATED_DOCTOR_FIXTURE_MANIFEST_REF, GENERATED_DOCTOR_PACKET_REF,
    GENERATED_DOCTOR_REPORT_REF, GENERATED_DOCTOR_SCHEMA_REF, RESOLUTION_ORDER,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> GeneratedDoctorFindingsPacket {
    let path = repo_root().join(GENERATED_DOCTOR_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<GeneratedDoctorFixture> {
    let dir = repo_root().join(GENERATED_DOCTOR_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: GeneratedDoctorFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_generated_doctor_findings_packet();
    assert_eq!(
        packet, seeded,
        "doctor packet drifted from seeded projection"
    );
    let violations = validate_generated_doctor_findings_packet(&packet);
    assert!(violations.is_empty(), "packet violations: {violations:?}");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let on_disk = load_fixtures();
    let mut seeded = seeded_generated_doctor_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        let violations = validate_generated_doctor_fixture(fixture);
        assert!(
            violations.is_empty(),
            "fixture {} invalid: {violations:?}",
            fixture.fixture_id
        );
    }
}

#[test]
fn lane_files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        GENERATED_DOCTOR_SCHEMA_REF,
        GENERATED_DOCTOR_DOC_REF,
        GENERATED_DOCTOR_PACKET_REF,
        GENERATED_DOCTOR_REPORT_REF,
        GENERATED_DOCTOR_FIXTURE_MANIFEST_REF,
    ] {
        let path = root.join(rel);
        assert!(path.exists(), "lane file missing: {rel}");
    }
}

#[test]
fn docs_help_reuse_the_controlled_vocabulary() {
    // The docs/help surface must reuse the exact summary, next-action, and
    // class token vocabulary the packet emits, with an anchor per finding class.
    let doc = fs::read_to_string(repo_root().join(GENERATED_DOCTOR_DOC_REF))
        .expect("generated-doctor doc must read");
    for class in RESOLUTION_ORDER {
        assert!(
            doc.contains(class.token()),
            "doc must name the {} class token",
            class.token()
        );
        assert!(
            doc.contains(class.summary()),
            "doc must quote the {} summary verbatim",
            class.token()
        );
        assert!(
            doc.contains(class.next_action()),
            "doc must quote the {} next action verbatim",
            class.token()
        );
    }
}

#[test]
fn support_export_is_redaction_safe() {
    let export = compile_generated_doctor_support_export(
        "envelope:generated-doctor:replay",
        "2026-06-20T10:00:00Z",
    );
    assert!(export.is_export_safe());
    // Lineage traceback survives the export: every finding keeps a generator
    // identity and a checkpoint ref, and only source-missing omits the source.
    for finding in &export.packet.findings {
        assert!(!finding.generator.name.is_empty());
        assert!(!finding.checkpoint_lineage_ref.is_empty());
        if finding.finding_class != GeneratedDoctorFindingClass::SourceMissing {
            assert!(finding.canonical_source_ref.is_some());
        }
    }
}
