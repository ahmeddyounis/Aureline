//! Fixture replay and cross-locale parity proofs for the M5 localized catalog.

use std::path::{Path, PathBuf};

use aureline_i18n::{
    seeded_m5_localization_parity_report, seeded_m5_localized_catalog, seeded_m5_localized_render,
    seeded_m5_message_registry, LocalizationRenderState, M5LocalizationParityReport,
    M5LocalizedCatalog, M5LocalizedRender, TextDirection,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/i18n/shell-command-help")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn catalog_fixture_matches_seeded_packet() {
    let from_file: M5LocalizedCatalog = load_json("localized-catalog.json");
    let from_code = seeded_m5_localized_catalog();
    assert_eq!(from_file, from_code);

    let registry = seeded_m5_message_registry();
    from_file.validate(&registry).expect("catalog validates");
}

#[test]
fn parity_fixture_matches_seeded_report() {
    let from_file: M5LocalizationParityReport = load_json("localization-parity.json");
    let from_code = seeded_m5_localization_parity_report();
    assert_eq!(from_file, from_code);
    from_file.validate().expect("parity report validates");
    assert!(from_file.parity_clean);
}

#[test]
fn rtl_render_fixture_matches_seeded_render() {
    let from_file: M5LocalizedRender = load_json("render-ar-SA.json");
    let from_code = seeded_m5_localized_render("ar-SA");
    assert_eq!(from_file, from_code);
    assert_eq!(from_file.text_direction, TextDirection::RightToLeft);
}

#[test]
fn rendered_ids_and_refs_are_identical_across_claimed_locales() {
    let registry = seeded_m5_message_registry();
    let source_ids: Vec<String> =
        seeded_m5_localized_render(&registry.source_language_locale).message_ids();

    for locale in ["es-MX", "ja-JP", "ar-SA"] {
        let render = seeded_m5_localized_render(locale);
        assert_eq!(
            render.message_ids(),
            source_ids,
            "ids drifted under {locale}"
        );
        for row in &render.rows {
            let entry = registry.entry(&row.message_id).expect("entry exists");
            // The localized prose changed; the stable routing refs did not.
            assert_eq!(row.stable_refs, entry.stable_refs, "{}", row.message_id);
            assert!(row.placeholders_preserved, "{}", row.message_id);
        }
    }
}

#[test]
fn untranslated_rows_are_marked_and_fall_back_not_hidden() {
    let report = seeded_m5_localization_parity_report();
    for row in &report.rows {
        // The marked count and the rendered fallback count agree exactly.
        assert_eq!(
            row.nonlocalized_message_ids.len(),
            row.source_fallback_count,
            "{} fallback bookkeeping",
            row.locale
        );
        let render = seeded_m5_localized_render(&row.locale);
        for message_id in &row.nonlocalized_message_ids {
            let rendered = render.row(message_id).expect("row present");
            assert_eq!(
                rendered.localization_state,
                LocalizationRenderState::SourceLanguageFallback
            );
            assert_eq!(rendered.effective_locale, render.source_language_locale);
        }
    }
}

#[test]
fn schemas_exist_for_the_published_fixtures() {
    let schema_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas/i18n");
    for schema in [
        "m5-localized-catalog.schema.json",
        "m5-localization-parity.schema.json",
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
