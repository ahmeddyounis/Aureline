//! Shell projections for active locale and locale-pack fallback state.

use aureline_i18n::{
    seeded_locale_pack_help_about_projection, seeded_locale_pack_settings_projection,
    seeded_locale_pack_support_projection, LocalePackSurfaceProjection,
};

/// Returns the shell row data used by Settings for locale-pack inspection.
pub fn project_locale_pack_settings_surface() -> LocalePackSurfaceProjection {
    seeded_locale_pack_settings_projection()
}

/// Returns the shell row data used by Help/About for locale-pack inspection.
pub fn project_locale_pack_help_about_surface() -> LocalePackSurfaceProjection {
    seeded_locale_pack_help_about_projection()
}

/// Returns the shell row data used by support export previews.
pub fn project_locale_pack_support_surface() -> LocalePackSurfaceProjection {
    seeded_locale_pack_support_projection()
}
