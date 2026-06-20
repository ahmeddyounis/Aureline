//! Fixture replay, body-file faithfulness, and cross-locale parity proofs for the
//! M5 translated help/docs/tour/auth/recovery/onboarding pack.

use std::path::{Path, PathBuf};

use aureline_i18n::{
    seeded_m5_translated_help_pack, seeded_m5_translated_help_parity_report,
    seeded_m5_translated_help_render, AssetLocalizationState, M5TranslatedHelpPack,
    M5TranslatedHelpParityReport, M5TranslatedHelpRender, TextDirection,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixtures_root() -> PathBuf {
    repo_root().join("fixtures/i18n/docs-tour-auth-recovery")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn pack_fixture_matches_seeded_packet() {
    let from_file: M5TranslatedHelpPack = load_json("translated-help-packs.json");
    let from_code = seeded_m5_translated_help_pack();
    assert_eq!(from_file, from_code);
    from_file.validate().expect("pack validates");
}

#[test]
fn parity_fixture_matches_seeded_report() {
    let from_file: M5TranslatedHelpParityReport = load_json("translated-help-parity.json");
    let from_code = seeded_m5_translated_help_parity_report();
    assert_eq!(from_file, from_code);
    from_file.validate().expect("parity report validates");
    assert!(from_file.parity_clean);
}

#[test]
fn rtl_render_fixture_matches_seeded_render() {
    let from_file: M5TranslatedHelpRender = load_json("render-ar-SA.json");
    let from_code = seeded_m5_translated_help_render("ar-SA");
    assert_eq!(from_file, from_code);
    assert_eq!(from_file.text_direction, TextDirection::RightToLeft);
}

#[test]
fn every_referenced_body_exists_and_is_command_and_citation_faithful() {
    let pack = seeded_m5_translated_help_pack();
    let root = repo_root();

    // Source bodies carry the canonical refs.
    for asset in &pack.source_assets {
        assert_body_carries_refs(
            &root,
            &asset.source_body_ref,
            &asset.preserved_refs.command_id_refs,
            &asset.preserved_refs.citation_anchor_refs,
        );
    }

    // Every translated body carries the same refs verbatim, and the explicit
    // "Open in source language" command route.
    for translation in &pack.translations {
        assert_body_carries_refs(
            &root,
            &translation.translated_body_ref,
            &translation.preserved_refs.command_id_refs,
            &translation.preserved_refs.citation_anchor_refs,
        );
        let body = read_body(&root, &translation.translated_body_ref);
        assert!(
            body.contains(&translation.escape_hatch.command_id_ref),
            "{} body drops the source-language escape command",
            translation.translated_body_ref
        );
    }
}

fn read_body(root: &Path, body_ref: &str) -> String {
    let path = root.join(body_ref);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("missing body {}: {err}", path.display()))
}

fn assert_body_carries_refs(
    root: &Path,
    body_ref: &str,
    command_id_refs: &[String],
    citation_anchor_refs: &[String],
) {
    let body = read_body(root, body_ref);
    for command in command_id_refs {
        assert!(
            body.contains(command),
            "{body_ref} dropped command ref {command}"
        );
    }
    for citation in citation_anchor_refs {
        assert!(
            body.contains(citation),
            "{body_ref} dropped citation anchor {citation}"
        );
    }
}

#[test]
fn fallback_assets_are_marked_not_hidden() {
    let report = seeded_m5_translated_help_parity_report();
    for row in &report.rows {
        // The marked count and the rendered fallback count agree exactly.
        assert_eq!(
            row.nontranslated_asset_ids.len(),
            row.source_fallback_count,
            "{} fallback bookkeeping",
            row.locale
        );
        let render = seeded_m5_translated_help_render(&row.locale);
        for asset_id in &row.nontranslated_asset_ids {
            let rendered = render.row(asset_id).expect("row present");
            assert_eq!(
                rendered.localization_state,
                AssetLocalizationState::SourceLanguageFallback
            );
            assert_eq!(rendered.effective_locale, render.source_language_locale);
            // Fallback still carries the source refs and the escape hatch.
            assert!(!rendered.preserved_refs.command_id_refs.is_empty());
            assert!(rendered.escape_hatch.is_valid());
        }
    }
}

#[test]
fn schemas_exist_for_the_published_fixtures() {
    let schema_root = repo_root().join("schemas/help");
    for schema in [
        "translated-doc-pack.schema.json",
        "translated-doc-pack-parity.schema.json",
    ] {
        let path = schema_root.join(schema);
        let body = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
        let parsed: serde_json::Value = serde_json::from_str(&body).expect("schema parses as JSON");
        assert_eq!(
            parsed["$schema"],
            "https://json-schema.org/draft/2020-12/schema"
        );
    }
}
