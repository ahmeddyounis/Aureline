//! Inline tests for the M5 localized catalog, render, and parity report.

use crate::message_registry::seeded_m5_message_registry;

use super::{
    build_localization_parity_report, seeded_m5_localization_parity_report,
    seeded_m5_localized_catalog, seeded_m5_localized_render, LocalizationRenderState,
    RenderSeverityClass, TextDirection, CLAIMED_LOCALES, DEFAULT_TRUNCATION_BUDGET_GRAPHEMES,
};

#[test]
fn seeded_catalog_validates_against_the_registry() {
    let registry = seeded_m5_message_registry();
    let catalog = seeded_m5_localized_catalog();
    catalog.validate(&registry).expect("catalog validates");

    // Every claimed locale carries at least one translated string.
    for locale in CLAIMED_LOCALES {
        let count = catalog
            .strings
            .iter()
            .filter(|s| s.locale == locale)
            .count();
        assert!(count > 0, "no strings for {locale}");
    }
}

#[test]
fn render_ids_are_identical_across_every_locale() {
    let source_ids = seeded_m5_localized_render("en-US").message_ids();

    for locale in ["es-MX", "ja-JP", "ar-SA", "de-DE", "fr-FR"] {
        let ids = seeded_m5_localized_render(locale).message_ids();
        assert_eq!(ids, source_ids, "ids drifted under {locale}");
    }
}

#[test]
fn render_preserves_stable_refs_under_localization() {
    let registry = seeded_m5_message_registry();
    let render = seeded_m5_localized_render("es-MX");
    for row in &render.rows {
        let entry = registry.entry(&row.message_id).expect("entry exists");
        // The visible prose localized, but the routing refs are byte-identical.
        assert_eq!(row.stable_refs, entry.stable_refs, "{}", row.message_id);
    }
}

#[test]
fn fully_translated_locale_localizes_every_row() {
    let render = seeded_m5_localized_render("es-MX");
    // es-MX translates every seeded message.
    assert_eq!(render.summary.source_fallback_rows, 0);
    assert!(render.summary.nonlocalized_message_ids.is_empty());
    for row in &render.rows {
        assert_eq!(row.localization_state, LocalizationRenderState::Localized);
        assert_eq!(row.effective_locale, "es-MX");
    }
}

#[test]
fn partial_locale_marks_untranslated_rows_and_falls_back() {
    let render = seeded_m5_localized_render("ar-SA");
    assert!(render.summary.source_fallback_rows > 0);
    assert!(!render.summary.nonlocalized_message_ids.is_empty());

    // A row ar-SA does not translate renders in the source language, explicitly.
    let open_notebook = render.row("msg:command:open-notebook").expect("row");
    assert_eq!(
        open_notebook.localization_state,
        LocalizationRenderState::SourceLanguageFallback
    );
    assert_eq!(open_notebook.effective_locale, "en-US");
    assert!(render
        .summary
        .nonlocalized_message_ids
        .contains(&"msg:command:open-notebook".to_owned()));
}

#[test]
fn rtl_locale_renders_right_to_left() {
    let render = seeded_m5_localized_render("ar-SA");
    assert_eq!(render.text_direction, TextDirection::RightToLeft);
    // A localized ar-SA row is tagged right-to-left; a fallback row is not.
    let localized = render.row("msg:command:run-build").expect("translated row");
    assert_eq!(localized.text_direction, TextDirection::RightToLeft);
    let fallback = render
        .row("msg:command:open-notebook")
        .expect("fallback row");
    assert_eq!(fallback.text_direction, TextDirection::LeftToRight);
}

#[test]
fn placeholders_survive_localization() {
    let render = seeded_m5_localized_render("ja-JP");
    let bg = render
        .row("msg:shell:status-bar:background-work")
        .expect("row");
    assert!(bg.placeholders_preserved);
    assert!(bg.display_text.contains("{count}"));

    let disabled = render
        .row("msg:error:command:disabled-reason")
        .expect("row");
    assert!(disabled.display_text.contains("{command}"));
    assert!(disabled.display_text.contains("{reason}"));
}

#[test]
fn expansion_ratio_is_reported_for_text_growth() {
    let render = seeded_m5_localized_render("es-MX");
    // Spanish expands "Switch window" -> "Cambiar de ventana".
    let switcher = render.row("msg:shell:switcher:open-window").expect("row");
    assert!(switcher.expansion_ratio_pct > 100);
    assert!(render.summary.max_expansion_ratio_pct >= switcher.expansion_ratio_pct);
}

#[test]
fn truncation_preserves_severity_and_scope() {
    let render = seeded_m5_localized_render("es-MX");
    let error = render
        .row("msg:error:locale-pack:signature-failed")
        .expect("row");
    assert_eq!(error.severity, RenderSeverityClass::Warning);

    // A very tight budget shortens the prose but never demotes the severity or
    // loses the surface scope.
    let truncated = error.truncate(8);
    assert!(truncated.was_truncated);
    assert_eq!(truncated.severity, RenderSeverityClass::Warning);
    assert_eq!(truncated.scope_surface_key, error.surface_key);
    assert!(truncated.severity_preserved);
    assert!(truncated.display_text.chars().count() <= 8);

    // Neutral chrome stays neutral.
    let title = render
        .row("msg:shell:title-bar:workspace-name")
        .expect("row");
    assert_eq!(title.severity, RenderSeverityClass::Neutral);
}

#[test]
fn truncation_at_zero_budget_keeps_no_prose_but_keeps_scope() {
    let render = seeded_m5_localized_render("ja-JP");
    let row = render.row("msg:notification:update:ready").expect("row");
    let truncated = row.truncate(0);
    assert!(truncated.display_text.is_empty());
    assert!(truncated.severity_preserved);
    assert_eq!(truncated.severity, RenderSeverityClass::Notice);
    assert_eq!(truncated.scope_surface_key, "notification");
}

#[test]
fn parity_report_is_clean_for_every_claimed_locale() {
    let report = seeded_m5_localization_parity_report();
    report.validate().expect("parity report validates");
    assert!(report.parity_clean);
    assert_eq!(report.rows.len(), CLAIMED_LOCALES.len());
    assert_eq!(
        report.truncation_budget_graphemes,
        DEFAULT_TRUNCATION_BUDGET_GRAPHEMES
    );

    for locale in CLAIMED_LOCALES {
        let row = report.row(locale).expect("locale row");
        assert!(row.id_set_matches_source);
        assert!(row.all_stable_refs_preserved);
        assert!(row.all_placeholders_preserved);
        assert!(row.severity_preserved_under_truncation);
        assert!(row.is_parity_clean());
    }
}

#[test]
fn parity_report_marks_nonlocalized_rows_per_locale() {
    let report = seeded_m5_localization_parity_report();
    // es-MX is fully localized; ar-SA leaves some rows in source language.
    let es = report.row("es-MX").expect("es row");
    assert_eq!(es.source_fallback_count, 0);
    assert!(es.nonlocalized_message_ids.is_empty());

    let ar = report.row("ar-SA").expect("ar row");
    assert!(ar.source_fallback_count > 0);
    assert!(!ar.nonlocalized_message_ids.is_empty());
}

#[test]
fn dropping_a_placeholder_breaks_parity() {
    let registry = seeded_m5_message_registry();
    let mut catalog = seeded_m5_localized_catalog();

    // Corrupt one translation by dropping its {count} placeholder.
    let target = catalog
        .strings
        .iter_mut()
        .find(|s| s.message_id == "msg:shell:status-bar:background-work" && s.locale == "es-MX")
        .expect("string exists");
    target.localized_text = "tareas en segundo plano".to_owned();

    let report =
        build_localization_parity_report(&catalog, &registry, DEFAULT_TRUNCATION_BUDGET_GRAPHEMES);
    let es = report.row("es-MX").expect("es row");
    assert!(!es.all_placeholders_preserved);
    assert!(!report.parity_clean);
}

#[test]
fn render_serializes_to_export_safe_json() {
    let render = seeded_m5_localized_render("ar-SA");
    let json = serde_json::to_string(&render).expect("serializes");
    assert!(json.contains("m5_localized_render_packet"));
    assert!(json.contains("expansion_ratio_pct"));
    assert!(json.contains("text_direction"));
}
