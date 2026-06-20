//! Extension-facing locale-support declarations and host-stable label protection.
//!
//! Extensions declare locale support in two governed ways. The legacy
//! [`ExtensionLocaleDeclaration`] rows ride on the beta locale-pack contract;
//! the richer [`ContributedLocaleManifest`] rows model extension-owned and
//! companion locale packs with a compatibility build range, fallback behavior,
//! owned surface families, and a reserved namespace. Both are read-only
//! projections of the canonical [`aureline_i18n`] truth: this crate adds no
//! localization state of its own.
//!
//! The host invariant this module surfaces is that a contributed pack can never
//! widen authority or hide host-controlled semantics. Trust, policy, capability,
//! and lifecycle vocabulary is host-owned; [`host_stable_labels_protected`]
//! confirms no contributed manifest claims to override it.

use aureline_i18n::{
    seeded_contributed_locale_support_report, seeded_locale_pack_beta_contract,
    ContributedLocaleManifest, ExtensionLocaleDeclaration,
};

/// Returns extension locale declarations from the governed locale-pack contract.
pub fn seeded_extension_locale_declarations() -> Vec<ExtensionLocaleDeclaration> {
    seeded_locale_pack_beta_contract().extension_locale_declarations
}

/// Returns the extension- and companion-owned contributed locale manifests.
pub fn seeded_contributed_locale_manifests() -> Vec<ContributedLocaleManifest> {
    seeded_contributed_locale_support_report().manifests
}

/// Returns true when no contributed locale manifest overrides host-stable labels.
///
/// Host-stable trust, policy, capability, and lifecycle labels stay canonical
/// regardless of what surrounding extension or companion strings localize. A
/// manifest that asks to override them, or claims a reserved host namespace,
/// fails the report's own validation; this helper reports the high-level
/// posture the extensions surface renders.
pub fn host_stable_labels_protected() -> bool {
    let report = seeded_contributed_locale_support_report();
    report.validate().is_ok()
        && report
            .manifests
            .iter()
            .all(|manifest| !manifest.may_override_host_stable_labels)
}
