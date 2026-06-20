//! Inline tests for the shell source-language fallback inspector.

use aureline_i18n::{LocaleFallbackOriginClass, M5MessageSurface};

use super::fallback_inspector::{
    project_support_locale_fallback_inspector, project_user_locale_fallback_inspector,
    FallbackInspectorAudience,
};

#[test]
fn source_locale_is_fully_localized_with_no_missing_keys() {
    let view = project_user_locale_fallback_inspector("en-US");
    assert!(view.is_fully_localized());
    assert_eq!(view.total_missing_key_count, 0);
    assert!(!view.source_language_fallback_active);
    assert_eq!(view.effective_locale, "en-US");
    assert_eq!(
        view.fallback_origin,
        LocaleFallbackOriginClass::RequestedLocaleAuthoritative
    );
    for row in &view.surface_rows {
        assert_eq!(row.missing_key_count, 0);
        assert_eq!(row.localized_key_count, row.total_keys);
    }
}

#[test]
fn fully_translated_locale_reports_authoritative() {
    let view = project_user_locale_fallback_inspector("es-MX");
    assert!(view.is_fully_localized());
    assert_eq!(
        view.fallback_origin,
        LocaleFallbackOriginClass::RequestedLocaleAuthoritative
    );
    assert_eq!(view.requested_locale, "es-MX");
    assert_eq!(view.effective_locale, "es-MX");
}

#[test]
fn partial_locale_discloses_missing_keys_per_surface() {
    let view = project_user_locale_fallback_inspector("ja-JP");
    assert!(!view.is_fully_localized());
    assert!(view.source_language_fallback_active);
    assert_eq!(
        view.fallback_origin,
        LocaleFallbackOriginClass::RequestedLocalePartialWithBaseFill
    );
    assert!(view.source_language_route_active);

    // The total missing count is the sum of per-surface missing counts.
    let summed: usize = view.surface_rows.iter().map(|r| r.missing_key_count).sum();
    assert_eq!(summed, view.total_missing_key_count);
    assert!(view.total_missing_key_count > 0);

    // A surface with a newly added, not-yet-translated key shows the gap.
    let command = view
        .surface_row(M5MessageSurface::CommandPalette)
        .expect("command surface row present");
    assert!(command.missing_key_count >= 1);
}

#[test]
fn signature_failed_locale_falls_back_to_source_for_every_key() {
    let view = project_user_locale_fallback_inspector("de-DE");
    assert_eq!(view.effective_locale, "en-US");
    assert_eq!(view.total_missing_key_count, view.total_keys);
    assert_eq!(
        view.fallback_origin,
        LocaleFallbackOriginClass::PackSignatureFailedSourceLanguageOnly
    );
    for row in &view.surface_rows {
        assert_eq!(row.localized_key_count, 0);
        assert!(row.source_language_fallback_active || row.total_keys == 0);
    }
}

#[test]
fn unprofiled_locale_still_resolves_to_an_honest_view() {
    // fr-FR has no declared profile and no translated keys, so the inspector
    // derives a full source-language fallback rather than failing.
    let view = project_user_locale_fallback_inspector("fr-FR");
    assert_eq!(view.requested_locale, "fr-FR");
    assert_eq!(
        view.fallback_chain.first().map(String::as_str),
        Some("fr-FR")
    );
    assert_eq!(
        view.fallback_chain.last().map(String::as_str),
        Some("en-US")
    );
    assert_eq!(view.total_missing_key_count, view.total_keys);
    assert_eq!(
        view.fallback_origin,
        LocaleFallbackOriginClass::PackMissingSourceLanguageOnly
    );
}

#[test]
fn user_and_support_views_agree_on_the_numbers() {
    let user = project_user_locale_fallback_inspector("ar-SA");
    let support = project_support_locale_fallback_inspector("ar-SA");

    assert_eq!(user.audience, FallbackInspectorAudience::User);
    assert_eq!(support.audience, FallbackInspectorAudience::SupportExport);

    // Both surfaces quote the same fallback truth.
    assert_eq!(user.fallback_chain, support.fallback_chain);
    assert_eq!(user.fallback_origin, support.fallback_origin);
    assert_eq!(
        user.total_missing_key_count,
        support.total_missing_key_count
    );
    assert_eq!(user.surface_rows, support.surface_rows);

    // Neither surface leaks translated body text.
    assert!(user.raw_translated_body_omitted);
    assert!(support.raw_translated_body_omitted);
}

#[test]
fn view_serializes_to_export_safe_json() {
    let view = project_support_locale_fallback_inspector("ja-JP");
    let json = serde_json::to_string(&view).expect("view serializes");
    assert!(json.contains("shell_locale_fallback_inspector_view"));
    assert!(json.contains("missing_key_count"));
    // Stable ids are present; no translated prose body field exists.
    assert!(json.contains("registry_packet_id"));
}
