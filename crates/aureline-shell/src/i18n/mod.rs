//! Shell-side localization inspectors for the new M5 surfaces.
//!
//! This module projects the canonical [`aureline_i18n`] message registry into
//! the user- and support-visible views the shell renders. It does not own any
//! localization truth: it reads stable message ids, locale profiles, and
//! source-language fallback state from the registry so Settings, Help/About,
//! and support exports all quote the same numbers.
//!
//! See [`fallback_inspector`] for the active-locale, requested-locale,
//! fallback-chain, and missing-key inspector; [`pack_compatibility`] for the
//! locale-pack skew and signature view; [`localized_surface`] for the
//! per-locale render of localized command, settings, help, error, and
//! notification labels with preserved command ids, keyboard paths, and
//! truncation that never hides scope or severity; [`contributed_support`]
//! for extension and companion locale support, host-stable label protection,
//! and per-source localization-issue attribution; and [`locale_diagnostics`]
//! for the consolidated localization diagnostics, Help/About, support-export,
//! and release-gate packet that joins active locale, installed pack versions,
//! compatibility state, fallback chain, missing-key counts, and
//! degraded-localization reasons into one inspectable truth packet.

pub mod attention_vocabulary;
pub mod contributed_support;
pub mod fallback_inspector;
pub mod locale_diagnostics;
pub mod localized_surface;
pub mod pack_compatibility;

pub use attention_vocabulary::{
    project_attention_vocabulary, project_support_attention_vocabulary,
    project_user_attention_vocabulary, AttentionSurfaceFamily, AttentionVocabularyAudience,
    AttentionVocabularyRow, AttentionVocabularyView, ATTENTION_VOCABULARY_VIEW_RECORD_KIND,
};

pub use fallback_inspector::{
    project_locale_fallback_inspector, project_support_locale_fallback_inspector,
    project_user_locale_fallback_inspector, FallbackInspectorAudience, LocaleFallbackInspectorView,
    SurfaceFallbackRow,
};

pub use localized_surface::{
    project_localized_surface, project_support_localized_surface, project_user_localized_surface,
    LocalizedSurfaceAudience, LocalizedSurfaceRow, LocalizedSurfaceView,
    LOCALIZED_SURFACE_VIEW_RECORD_KIND,
};

pub use pack_compatibility::{
    project_locale_pack_compatibility, project_support_locale_pack_compatibility,
    project_user_locale_pack_compatibility, CompatibilityViewAudience,
    LocalePackCompatibilityRowView, LocalePackCompatibilityView,
};

pub use contributed_support::{
    project_contributed_locale_support, project_support_contributed_locale_support,
    project_user_contributed_locale_support, ContributedLocaleSupportView,
    ContributedSupportAudience, ContributedSupportRowView, IssueCountsBySource,
    CONTRIBUTED_LOCALE_SUPPORT_VIEW_RECORD_KIND,
};

pub use locale_diagnostics::{
    seeded_locale_diagnostics_packet, seeded_locale_diagnostics_support_export,
    InstalledLocalePackRow, LocaleClaimGateState, LocaleClaimNarrowRow, LocaleDiagnosticsFinding,
    LocaleDiagnosticsHelpAboutCard, LocaleDiagnosticsPacket, LocaleDiagnosticsProfileRow,
    LocaleDiagnosticsReleaseGate, LocaleDiagnosticsSummary, LocaleDiagnosticsSupportExport,
    LocaleProblemOrigin, SupportExportPackRow, SupportExportProfileRow,
    LOCALE_DIAGNOSTICS_FIXTURE_REF, LOCALE_DIAGNOSTICS_HELP_ABOUT_RECORD_KIND,
    LOCALE_DIAGNOSTICS_PACKET_ID, LOCALE_DIAGNOSTICS_RECORD_KIND,
    LOCALE_DIAGNOSTICS_RELEASE_GATE_RECORD_KIND, LOCALE_DIAGNOSTICS_SCHEMA_REF,
    LOCALE_DIAGNOSTICS_SCHEMA_VERSION, LOCALE_DIAGNOSTICS_SUPPORT_EXPORT_FIXTURE_REF,
    LOCALE_DIAGNOSTICS_SUPPORT_EXPORT_RECORD_KIND,
};

#[cfg(test)]
mod tests;
