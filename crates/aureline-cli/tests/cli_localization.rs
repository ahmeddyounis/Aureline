//! Fixture replay and cross-locale parity proofs for the CLI/help locale posture.

use std::path::{Path, PathBuf};

use aureline_cli::{seeded_cli_localization_packet, CliLocalizationPacket};

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
    let from_file: CliLocalizationPacket =
        load_json("fixtures/i18n/cli-doctor-support/cli-help-localization.json");
    let from_code = seeded_cli_localization_packet();

    assert_eq!(from_file, from_code);
    from_file.validate().expect("packet validates");
}

#[test]
fn published_schema_exists_and_is_draft_2020_12() {
    let schema_path = repo_root().join("schemas/i18n/cli-help-locale.schema.json");
    let body = std::fs::read_to_string(&schema_path)
        .unwrap_or_else(|err| panic!("missing {}: {err}", schema_path.display()));
    let parsed: serde_json::Value = serde_json::from_str(&body).expect("schema parses");
    assert_eq!(
        parsed["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(
        parsed["properties"]["record_kind"]["const"],
        "cli_help_localization_packet"
    );
}

#[test]
fn prose_localizes_without_breaking_automation_anchors() {
    let packet = seeded_cli_localization_packet();
    let report = packet.parity_report();
    assert!(
        report.parity_clean,
        "automation anchors drifted under localization"
    );

    // Flags, JSON keys, subcommand names, and exit classes are byte-identical
    // across every locale; only effective locale and fallback flags move.
    let source = packet.render("en-US");
    for locale in ["es-MX", "ja-JP", "ar-SA", "de-DE"] {
        let render = packet.render(locale);
        for (rendered, base) in render.iter().zip(&source) {
            assert_eq!(rendered.cli_refs, base.cli_refs, "{}", base.message_id);
        }
    }
}

#[test]
fn support_export_keeps_locale_state_inspectable_and_omits_bodies() {
    let packet = seeded_cli_localization_packet();
    let export = &packet.support_export;
    assert!(!export.raw_translated_bodies_exported);
    assert!(!export.fallback_chain.is_empty());
    assert!(export
        .rows
        .iter()
        .all(|row| row.raw_translated_body_omitted));
    assert!(export
        .omitted_material_classes
        .contains(&"raw_translated_message_bodies".to_owned()));
}
