//! Settings projection for active locale and locale-pack fallback state.

use aureline_i18n::{seeded_locale_pack_settings_projection, LocalePackSurfaceProjection};

/// Returns the settings-facing locale-pack inspector projection.
pub fn project_locale_beta_settings_panel() -> LocalePackSurfaceProjection {
    seeded_locale_pack_settings_projection()
}
