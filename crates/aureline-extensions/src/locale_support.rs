//! Extension-facing locale-support declarations.

use aureline_i18n::{seeded_locale_pack_beta_contract, ExtensionLocaleDeclaration};

/// Returns extension locale declarations from the governed locale-pack contract.
pub fn seeded_extension_locale_declarations() -> Vec<ExtensionLocaleDeclaration> {
    seeded_locale_pack_beta_contract().extension_locale_declarations
}
