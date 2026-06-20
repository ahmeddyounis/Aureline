//! Inline tests for the M5 translated-help pack, render, and parity report.

use super::{
    build_translated_help_parity_report, seeded_m5_translated_help_pack,
    seeded_m5_translated_help_parity_report, seeded_m5_translated_help_render, AssetCoverageState,
    AssetLocalizationState, MirrorOfflinePosture, TextDirection, TranslatedAssetFamily,
    TranslationBadgeClass, TranslationFreshnessClass, OPEN_IN_SOURCE_LANGUAGE_ACTION_LABEL,
};

const CLAIMED_LOCALES: [&str; 3] = ["es-MX", "ja-JP", "ar-SA"];

#[test]
fn seeded_pack_validates() {
    let pack = seeded_m5_translated_help_pack();
    pack.validate().expect("pack validates");

    // Every claimed locale carries at least one translated asset.
    for locale in CLAIMED_LOCALES {
        let count = pack
            .translations
            .iter()
            .filter(|t| t.requested_locale == locale)
            .count();
        assert!(count > 0, "no translations for {locale}");
    }
    // Every named family appears in the source assets.
    for family in TranslatedAssetFamily::ALL {
        assert!(
            pack.source_assets.iter().any(|a| a.asset_family == family),
            "missing family {}",
            family.as_key()
        );
    }
}

#[test]
fn render_asset_ids_are_identical_across_every_locale() {
    let source_ids = seeded_m5_translated_help_render("en-US").asset_ids();
    for locale in ["es-MX", "ja-JP", "ar-SA", "de-DE", "fr-FR"] {
        let ids = seeded_m5_translated_help_render(locale).asset_ids();
        assert_eq!(ids, source_ids, "asset ids drifted under {locale}");
    }
}

#[test]
fn translations_preserve_source_refs_byte_for_byte() {
    let pack = seeded_m5_translated_help_pack();
    for translation in &pack.translations {
        let source = pack
            .source_asset(&translation.asset_id)
            .expect("source asset exists");
        // The prose and title localized, but the routable refs are identical.
        assert_eq!(
            translation.preserved_refs, source.preserved_refs,
            "{} dropped or rewrote a ref",
            translation.asset_id
        );
        assert_eq!(
            translation.escape_hatch, source.escape_hatch,
            "{} escape hatch drifted",
            translation.asset_id
        );
    }
}

#[test]
fn fully_translated_locale_translates_every_asset() {
    let render = seeded_m5_translated_help_render("es-MX");
    assert_eq!(render.summary.source_fallback_rows, 0);
    assert!(render.summary.nontranslated_asset_ids.is_empty());
    for row in &render.rows {
        assert_eq!(
            row.localization_state,
            AssetLocalizationState::TranslatedRequestedLocale
        );
        assert_eq!(row.effective_locale, "es-MX");
    }
}

#[test]
fn partial_locale_marks_untranslated_assets_and_falls_back() {
    let render = seeded_m5_translated_help_render("ja-JP");
    assert!(render.summary.source_fallback_rows > 0);
    assert!(!render.summary.nontranslated_asset_ids.is_empty());

    // ja-JP does not translate the recovery card; it falls back, not hidden.
    let recovery = render
        .row("asset:recovery:restore-checkpoint")
        .expect("row");
    assert_eq!(
        recovery.localization_state,
        AssetLocalizationState::SourceLanguageFallback
    );
    assert_eq!(recovery.effective_locale, "en-US");
    // The fallback still preserves the source refs and the escape hatch.
    assert!(!recovery.preserved_refs.command_id_refs.is_empty());
    assert!(recovery.escape_hatch.is_valid());
}

#[test]
fn every_row_exposes_open_in_source_language() {
    for locale in CLAIMED_LOCALES {
        let render = seeded_m5_translated_help_render(locale);
        for row in &render.rows {
            assert_eq!(
                row.escape_hatch.action_label,
                OPEN_IN_SOURCE_LANGUAGE_ACTION_LABEL
            );
            assert!(row.escape_hatch.is_valid(), "{} hatch", row.asset_id);
        }
    }
}

#[test]
fn rtl_locale_renders_right_to_left() {
    let render = seeded_m5_translated_help_render("ar-SA");
    assert_eq!(render.text_direction, TextDirection::RightToLeft);

    // A translated ar-SA row is right-to-left; a fallback row is the LTR source.
    let translated = render.row("asset:docs:getting-started").expect("row");
    assert_eq!(
        translated.localization_state,
        AssetLocalizationState::TranslatedRequestedLocale
    );
    assert_eq!(translated.text_direction, TextDirection::RightToLeft);

    let fallback = render.row("asset:tour:first-build").expect("row");
    assert_eq!(
        fallback.localization_state,
        AssetLocalizationState::SourceLanguageFallback
    );
    assert_eq!(fallback.text_direction, TextDirection::LeftToRight);
}

#[test]
fn stale_translation_is_visibly_distinct_from_live_source() {
    let render = seeded_m5_translated_help_render("ar-SA");
    let stale = render
        .row("asset:recovery:restore-checkpoint")
        .expect("row");
    assert_eq!(stale.badge_class, TranslationBadgeClass::StaleTranslation);
    assert_eq!(
        stale.freshness_class,
        TranslationFreshnessClass::StaleBehindSource
    );
    assert!(stale.distinct_from_live_source);
    // The stale basis is older than the current source revision.
    assert_ne!(
        stale.rendered_source_revision_ref,
        stale.source_revision_ref
    );
    // Escalation routes survive staleness.
    assert!(stale
        .preserved_refs
        .command_id_refs
        .contains(&"cmd:support.open_recovery_runbook".to_owned()));
}

#[test]
fn warm_cached_partial_is_distinct_but_complete_is_not() {
    let render = seeded_m5_translated_help_render("ja-JP");
    let cached = render.row("asset:glossary:truth-source").expect("row");
    assert_eq!(
        cached.freshness_class,
        TranslationFreshnessClass::WarmCached
    );
    assert!(cached.distinct_from_live_source);

    let live = render.row("asset:docs:getting-started").expect("row");
    assert_eq!(
        live.freshness_class,
        TranslationFreshnessClass::CurrentWithLiveSource
    );
    assert!(!live.distinct_from_live_source);
}

#[test]
fn fallback_rows_disclose_not_installed_posture() {
    let render = seeded_m5_translated_help_render("ar-SA");
    let fallback = render.row("asset:glossary:truth-source").expect("row");
    assert_eq!(
        fallback.localization_state,
        AssetLocalizationState::SourceLanguageFallback
    );
    assert_eq!(
        fallback.mirror_offline_posture,
        MirrorOfflinePosture::NotInstalled
    );
}

#[test]
fn parity_report_is_clean_for_every_claimed_locale() {
    let report = seeded_m5_translated_help_parity_report();
    report.validate().expect("parity report validates");
    assert!(report.parity_clean);
    assert_eq!(report.rows.len(), CLAIMED_LOCALES.len());

    for locale in CLAIMED_LOCALES {
        let row = report.row(locale).expect("locale row");
        assert!(row.asset_id_set_matches_source);
        assert!(row.citation_faithful);
        assert!(row.command_faithful);
        assert!(row.all_refs_preserved);
        assert!(row.all_escape_hatches_present);
        assert!(row.stale_or_offline_distinct_from_live);
        assert!(row.escalation_routes_preserved);
        assert!(row.is_parity_clean());
    }
}

#[test]
fn parity_report_marks_nontranslated_assets_per_locale() {
    let report = seeded_m5_translated_help_parity_report();
    let es = report.row("es-MX").expect("es row");
    assert_eq!(es.source_fallback_count, 0);
    assert!(es.nontranslated_asset_ids.is_empty());

    let ar = report.row("ar-SA").expect("ar row");
    assert!(ar.source_fallback_count > 0);
    assert!(!ar.nontranslated_asset_ids.is_empty());
    assert_eq!(
        ar.nontranslated_asset_ids.len(),
        ar.source_fallback_count,
        "fallback bookkeeping"
    );
}

#[test]
fn dropping_a_command_ref_breaks_parity() {
    let mut pack = seeded_m5_translated_help_pack();
    // Corrupt one translation by dropping an escalation command route.
    let target = pack
        .translations
        .iter_mut()
        .find(|t| t.asset_id == "asset:auth:sign-in" && t.requested_locale == "es-MX")
        .expect("translation exists");
    target.preserved_refs.command_id_refs = vec!["cmd:auth.sign_in".to_owned()];

    let report = build_translated_help_parity_report(&pack);
    let es = report.row("es-MX").expect("es row");
    assert!(!es.command_faithful);
    assert!(!es.all_refs_preserved);
    assert!(!report.parity_clean);
}

#[test]
fn dropping_an_escape_hatch_breaks_parity() {
    let mut pack = seeded_m5_translated_help_pack();
    let target = pack
        .translations
        .iter_mut()
        .find(|t| t.requested_locale == "ja-JP")
        .expect("translation exists");
    target.escape_hatch.keyboard_reachable = false;

    let report = build_translated_help_parity_report(&pack);
    let ja = report.row("ja-JP").expect("ja row");
    assert!(!ja.all_escape_hatches_present);
    assert!(!report.parity_clean);
}

#[test]
fn complete_coverage_validation_rejects_stale_basis() {
    let mut pack = seeded_m5_translated_help_pack();
    let target = pack
        .translations
        .iter_mut()
        .find(|t| t.asset_id == "asset:docs:getting-started" && t.requested_locale == "es-MX")
        .expect("translation exists");
    // A complete badge with a diverged basis is incoherent and must be rejected.
    target.coverage_state = AssetCoverageState::TranslatedComplete;
    target.freshness_class = TranslationFreshnessClass::StaleBehindSource;

    let findings = pack.validate().expect_err("validation fails");
    assert!(findings.iter().any(|f| f
        .message
        .contains("complete translation must match the current live source")));
}

#[test]
fn pack_serializes_to_export_safe_json() {
    let pack = seeded_m5_translated_help_pack();
    let json = serde_json::to_string(&pack).expect("serializes");
    assert!(json.contains("m5_translated_help_pack_packet"));
    assert!(json.contains("preserved_refs"));
    assert!(json.contains("Open in source language"));
}
