//! Support-export projection for active locale and locale-pack fallback state.

use aureline_i18n::{seeded_locale_pack_support_export, LocalePackSupportExport};

/// Returns the metadata-only support export for locale-pack state.
pub fn current_locale_pack_support_export() -> LocalePackSupportExport {
    seeded_locale_pack_support_export()
}
