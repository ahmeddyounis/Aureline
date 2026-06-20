//! Shell-side localization inspectors for the new M5 surfaces.
//!
//! This module projects the canonical [`aureline_i18n`] message registry into
//! the user- and support-visible views the shell renders. It does not own any
//! localization truth: it reads stable message ids, locale profiles, and
//! source-language fallback state from the registry so Settings, Help/About,
//! and support exports all quote the same numbers.
//!
//! See [`fallback_inspector`] for the active-locale, requested-locale,
//! fallback-chain, and missing-key inspector.

pub mod fallback_inspector;
pub mod pack_compatibility;

pub use fallback_inspector::{
    project_locale_fallback_inspector, project_support_locale_fallback_inspector,
    project_user_locale_fallback_inspector, FallbackInspectorAudience, LocaleFallbackInspectorView,
    SurfaceFallbackRow,
};

pub use pack_compatibility::{
    project_locale_pack_compatibility, project_support_locale_pack_compatibility,
    project_user_locale_pack_compatibility, CompatibilityViewAudience,
    LocalePackCompatibilityRowView, LocalePackCompatibilityView,
};

#[cfg(test)]
mod tests;
