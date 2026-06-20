//! Fixture replay and cross-locale parity proofs for the Doctor report locale posture.

use std::path::{Path, PathBuf};

use aureline_doctor::{
    seeded_doctor_report_localization_packet, DoctorReportLocalizationPacket,
    DoctorReportSupportExport,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_json<T: serde::de::DeserializeOwned>(rel: &str) -> T {
    let path = repo_root().join(rel);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn fixture_matches_seeded_packet() {
    let from_file: DoctorReportLocalizationPacket =
        load_json("fixtures/i18n/cli-doctor-support/doctor-report-localization.json");
    let from_code = seeded_doctor_report_localization_packet();

    assert_eq!(from_file, from_code);
    from_file.validate().expect("packet validates");
}

#[test]
fn support_export_fixture_matches_derived_projection() {
    let from_file: DoctorReportSupportExport =
        load_json("fixtures/i18n/cli-doctor-support/doctor-report-support-export.json");
    let from_code = seeded_doctor_report_localization_packet().support_export;
    assert_eq!(from_file, from_code);
}

#[test]
fn published_schema_exists_and_is_draft_2020_12() {
    let schema_path = repo_root().join("schemas/i18n/doctor-report-locale.schema.json");
    let body = std::fs::read_to_string(&schema_path)
        .unwrap_or_else(|err| panic!("missing {}: {err}", schema_path.display()));
    let parsed: serde_json::Value = serde_json::from_str(&body).expect("schema parses");
    assert_eq!(
        parsed["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(
        parsed["properties"]["record_kind"]["const"],
        "doctor_report_localization_packet"
    );
}

#[test]
fn finding_identity_survives_localization() {
    let packet = seeded_doctor_report_localization_packet();
    let report = packet.parity_report();
    assert!(
        report.parity_clean,
        "finding identity drifted under localization"
    );

    // Finding codes, exit classes, evidence refs, and scope labels are
    // byte-identical across every locale.
    let source = packet.render("en-US");
    for locale in ["es-MX", "ja-JP", "ar-SA", "de-DE"] {
        let render = packet.render(locale);
        for (rendered, base) in render.iter().zip(&source) {
            assert_eq!(
                rendered.doctor_refs, base.doctor_refs,
                "{}",
                base.message_id
            );
        }
    }
}

#[test]
fn exported_artifact_keeps_locale_and_fallback_inspectable() {
    let export = seeded_doctor_report_localization_packet().support_export;
    assert!(!export.raw_translated_bodies_exported);
    assert!(!export.fallback_chain.is_empty());
    assert!(!export.preserved_finding_codes.is_empty());
    assert!(export
        .rows
        .iter()
        .all(|row| row.raw_translated_body_omitted));
    assert!(export
        .omitted_material_classes
        .contains(&"raw_evidence_payloads".to_owned()));
}
