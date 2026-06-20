//! Source-language fallback inspector for the new M5 surfaces.
//!
//! The inspector answers, for any requested locale, whether each surface is
//! rendering the requested locale, a base-language fill, or the source language,
//! and how many message keys are missing. Both the user-facing Settings /
//! Help/About surfaces and the metadata-only support export read the same
//! [`LocaleFallbackInspectorView`], so the fallback state is never hidden in
//! debug-only logs.
//!
//! Every number is derived from the canonical [`aureline_i18n`] message
//! registry against stable message ids; no translated body text crosses this
//! boundary.

use serde::{Deserialize, Serialize};

use aureline_i18n::{
    seeded_m5_message_registry, DegradedLocalizationState, LocaleFallbackOriginClass,
    M5MessageRegistry, M5MessageSurface,
};

/// Record kind for [`LocaleFallbackInspectorView`].
pub const LOCALE_FALLBACK_INSPECTOR_RECORD_KIND: &str = "shell_locale_fallback_inspector_view";

/// Who is reading the inspector view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackInspectorAudience {
    /// User-facing Settings or Help/About surface.
    User,
    /// Metadata-only support export.
    SupportExport,
}

/// Per-surface localization coverage for one requested locale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceFallbackRow {
    /// M5 surface this row describes.
    pub surface: M5MessageSurface,
    /// Stable snake_case key for the surface.
    pub surface_key: String,
    /// Total registered message keys on this surface.
    pub total_keys: usize,
    /// Keys rendered in the requested locale (or its base fill).
    pub localized_key_count: usize,
    /// Keys falling back to the source language.
    pub missing_key_count: usize,
    /// Whether any key on this surface fell back to the source language.
    pub source_language_fallback_active: bool,
}

/// Inspectable locale and source-language fallback state for one requested
/// locale, shared by the user surfaces and the support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleFallbackInspectorView {
    /// Boundary record kind.
    pub record_kind: String,
    /// Who is reading the view.
    pub audience: FallbackInspectorAudience,
    /// Source registry packet id.
    pub registry_packet_id: String,
    /// Build identity the registry was minted for.
    pub target_build_identity_ref: String,
    /// User-requested locale.
    pub requested_locale: String,
    /// Locale that produced covered messages.
    pub effective_locale: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Ordered requested-to-base-to-source fallback chain.
    pub fallback_chain: Vec<String>,
    /// Why fallback did or did not occur.
    pub fallback_origin: LocaleFallbackOriginClass,
    /// Degraded localization state after fallback.
    pub degraded_state: DegradedLocalizationState,
    /// Whether a visible source-language route is active.
    pub source_language_route_active: bool,
    /// Total registered message keys across all surfaces.
    pub total_keys: usize,
    /// Total keys falling back to the source language.
    pub total_missing_key_count: usize,
    /// Whether any surface fell back to the source language.
    pub source_language_fallback_active: bool,
    /// Per-surface coverage rows.
    pub surface_rows: Vec<SurfaceFallbackRow>,
    /// Always true; translated body text never crosses this boundary.
    pub raw_translated_body_omitted: bool,
}

impl LocaleFallbackInspectorView {
    /// Returns true when every surface renders the requested locale.
    pub fn is_fully_localized(&self) -> bool {
        self.total_missing_key_count == 0
    }

    /// Returns the row for a surface, when present.
    pub fn surface_row(&self, surface: M5MessageSurface) -> Option<&SurfaceFallbackRow> {
        self.surface_rows.iter().find(|row| row.surface == surface)
    }
}

/// Projects the fallback inspector view for a requested locale and audience.
///
/// Reads the seeded [`M5MessageRegistry`]. When the registry declares a profile
/// for the locale, its fallback chain, origin, and degraded state are used
/// verbatim; otherwise the chain and origin are derived from per-message
/// coverage so any requested locale resolves to an honest view.
pub fn project_locale_fallback_inspector(
    audience: FallbackInspectorAudience,
    requested_locale: &str,
) -> LocaleFallbackInspectorView {
    let registry = seeded_m5_message_registry();
    build_view(&registry, audience, requested_locale)
}

/// Projects the user-facing fallback inspector view (Settings / Help/About).
pub fn project_user_locale_fallback_inspector(
    requested_locale: &str,
) -> LocaleFallbackInspectorView {
    project_locale_fallback_inspector(FallbackInspectorAudience::User, requested_locale)
}

/// Projects the metadata-only support-export fallback inspector view.
pub fn project_support_locale_fallback_inspector(
    requested_locale: &str,
) -> LocaleFallbackInspectorView {
    project_locale_fallback_inspector(FallbackInspectorAudience::SupportExport, requested_locale)
}

/// Builds the inspector view from a registry, audience, and requested locale.
fn build_view(
    registry: &M5MessageRegistry,
    audience: FallbackInspectorAudience,
    requested_locale: &str,
) -> LocaleFallbackInspectorView {
    let source = registry.source_language_locale.clone();
    let total_keys = registry.entries.len();
    let total_missing = registry.missing_key_count(requested_locale);

    let surface_rows = M5MessageSurface::ALL
        .into_iter()
        .map(|surface| {
            let surface_total = registry
                .entries
                .iter()
                .filter(|entry| entry.surface == surface)
                .count();
            let missing = registry.missing_key_count_for_surface(requested_locale, surface);
            SurfaceFallbackRow {
                surface,
                surface_key: surface.as_key().to_owned(),
                total_keys: surface_total,
                localized_key_count: surface_total - missing,
                missing_key_count: missing,
                source_language_fallback_active: missing > 0,
            }
        })
        .collect();

    let (effective_locale, fallback_chain, fallback_origin, degraded_state, route_active) =
        match registry.locale_profile(requested_locale) {
            Some(profile) => (
                profile.effective_locale.clone(),
                profile.fallback_chain.clone(),
                profile.fallback_origin,
                profile.degraded_state,
                profile.source_language_route_active,
            ),
            None => derive_unprofiled(requested_locale, &source, total_missing, total_keys),
        };

    LocaleFallbackInspectorView {
        record_kind: LOCALE_FALLBACK_INSPECTOR_RECORD_KIND.to_owned(),
        audience,
        registry_packet_id: registry.packet_id.clone(),
        target_build_identity_ref: registry.target_build_identity_ref.clone(),
        requested_locale: requested_locale.to_owned(),
        effective_locale,
        source_language_locale: source,
        fallback_chain,
        fallback_origin,
        degraded_state,
        source_language_route_active: route_active,
        total_keys,
        total_missing_key_count: total_missing,
        source_language_fallback_active: total_missing > 0,
        surface_rows,
        raw_translated_body_omitted: true,
    }
}

/// Derives the chain, origin, and degraded state for a locale the registry does
/// not declare a profile for, using only per-message coverage.
fn derive_unprofiled(
    requested_locale: &str,
    source: &str,
    total_missing: usize,
    total_keys: usize,
) -> (
    String,
    Vec<String>,
    LocaleFallbackOriginClass,
    DegradedLocalizationState,
    bool,
) {
    let mut chain = vec![requested_locale.to_owned()];
    if let Some((base, _)) = requested_locale.split_once('-') {
        if base != requested_locale {
            chain.push(base.to_owned());
        }
    }
    if source != requested_locale {
        chain.push(source.to_owned());
    }

    if total_keys == 0 || total_missing == 0 {
        (
            requested_locale.to_owned(),
            chain,
            LocaleFallbackOriginClass::RequestedLocaleAuthoritative,
            DegradedLocalizationState::FullyLocalized,
            false,
        )
    } else if total_missing == total_keys {
        (
            source.to_owned(),
            chain,
            LocaleFallbackOriginClass::PackMissingSourceLanguageOnly,
            DegradedLocalizationState::FailedPackSourceLanguageOnly,
            true,
        )
    } else {
        (
            requested_locale.to_owned(),
            chain,
            LocaleFallbackOriginClass::RequestedLocalePartialWithBaseFill,
            DegradedLocalizationState::PartialTranslationDisclosed,
            true,
        )
    }
}
