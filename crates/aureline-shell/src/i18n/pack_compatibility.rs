//! Shell-side locale-pack compatibility and skew inspector.
//!
//! This projects the canonical [`aureline_i18n`] locale-pack compatibility
//! report into the view the shell renders on Settings, Help/About, and
//! diagnostics. It owns no localization truth: it reads pack versions,
//! compatibility and signature state, missing-key counts, and degraded-
//! localization reasons from the report so every audience quotes the same
//! numbers.
//!
//! The view exists so pack skew or signature failure never leaves a
//! half-localized surface in an ambiguous state: every pack resolves to either
//! an applied (possibly partial, with disclosed missing keys) or a fully
//! degraded source-language state, with the reason attached.

use serde::{Deserialize, Serialize};

use aureline_i18n::{
    seeded_locale_pack_compatibility_report, LocalePackSignatureState, PackApplicationDecision,
    SkewDegradeReason, VersionMatchState,
};

/// Record kind for [`LocalePackCompatibilityView`].
pub const LOCALE_PACK_COMPATIBILITY_VIEW_RECORD_KIND: &str =
    "shell_locale_pack_compatibility_view";

/// Who is reading the compatibility view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityViewAudience {
    /// User-facing Settings, Help/About, or diagnostics surface.
    User,
    /// Metadata-only support export.
    SupportExport,
}

/// One export-safe pack row rendered by the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackCompatibilityRowView {
    /// Stable pack id.
    pub pack_id: String,
    /// Pack version quoted for support and release.
    pub pack_version: String,
    /// User-requested locale.
    pub requested_locale: String,
    /// Locale that produces rendered text after evaluation.
    pub effective_locale: String,
    /// Observed signature state.
    pub signature_state: LocalePackSignatureState,
    /// Observed version-match state.
    pub version_match_state: VersionMatchState,
    /// Whether the active build is inside the pack's compatibility range.
    pub target_build_in_compatibility_range: bool,
    /// Apply-or-degrade decision.
    pub application_decision: PackApplicationDecision,
    /// Reason a degrade occurred, when applicable.
    pub skew_degrade_reason: SkewDegradeReason,
    /// Keys falling back to source language.
    pub missing_key_count: usize,
    /// Total translatable keys.
    pub total_key_count: usize,
    /// Whether this pack backs a claimed localized profile.
    pub claimed_localized_profile: bool,
    /// Same-surface source-language route.
    pub open_in_source_language_route_ref: String,
    /// Short export-safe label.
    pub presentation_label: String,
}

impl LocalePackCompatibilityRowView {
    /// Returns true when the pack degraded fully to source language.
    pub fn degraded_to_source_language(&self) -> bool {
        self.application_decision == PackApplicationDecision::DegradeToSourceLanguageOnly
    }

    /// Returns true when the row sits in a defined, non-ambiguous state.
    ///
    /// A degraded pack must show every key on source language; an applied pack
    /// must keep its missing-key count within its total. There is no undefined
    /// half-localized middle.
    pub fn is_resolved(&self) -> bool {
        if self.degraded_to_source_language() {
            self.skew_degrade_reason != SkewDegradeReason::NotDegraded
                && self.missing_key_count == self.total_key_count
        } else {
            self.skew_degrade_reason == SkewDegradeReason::NotDegraded
                && self.missing_key_count <= self.total_key_count
        }
    }
}

/// Inspectable locale-pack compatibility view shared by shell surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackCompatibilityView {
    /// Boundary record kind.
    pub record_kind: String,
    /// Who is reading the view.
    pub audience: CompatibilityViewAudience,
    /// Source report id.
    pub report_id: String,
    /// Active build identity the report was evaluated against.
    pub target_build_identity_ref: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Number of evaluated packs.
    pub total_packs: usize,
    /// Packs that applied their translations.
    pub renderable_packs: usize,
    /// Packs that degraded fully to source language.
    pub degraded_source_language_packs: usize,
    /// Total missing keys across evaluated packs.
    pub total_missing_keys: usize,
    /// Whether no unsigned or incompatible pack claims a localized profile.
    pub guardrail_clean: bool,
    /// Per-pack rows.
    pub rows: Vec<LocalePackCompatibilityRowView>,
    /// Always true; translated body text never crosses this boundary.
    pub raw_translated_body_omitted: bool,
}

impl LocalePackCompatibilityView {
    /// Returns the row for a pack id, when present.
    pub fn row(&self, pack_id: &str) -> Option<&LocalePackCompatibilityRowView> {
        self.rows.iter().find(|row| row.pack_id == pack_id)
    }

    /// Returns true when every pack resolves to a defined state.
    pub fn all_states_resolved(&self) -> bool {
        self.rows.iter().all(LocalePackCompatibilityRowView::is_resolved)
    }
}

/// Projects the compatibility view for an audience from the seeded report.
pub fn project_locale_pack_compatibility(
    audience: CompatibilityViewAudience,
) -> LocalePackCompatibilityView {
    let report = seeded_locale_pack_compatibility_report();
    let rows = report
        .rows
        .iter()
        .map(|row| LocalePackCompatibilityRowView {
            pack_id: row.pack_id.clone(),
            pack_version: row.pack_version.clone(),
            requested_locale: row.requested_locale.clone(),
            effective_locale: row.effective_locale.clone(),
            signature_state: row.signature_state,
            version_match_state: row.version_match_state,
            target_build_in_compatibility_range: row.target_build_in_compatibility_range,
            application_decision: row.application_decision,
            skew_degrade_reason: row.skew_degrade_reason,
            missing_key_count: row.missing_key_count,
            total_key_count: row.total_key_count,
            claimed_localized_profile: row.claimed_localized_profile,
            open_in_source_language_route_ref: row.open_in_source_language_route_ref.clone(),
            presentation_label: row.presentation_label.clone(),
        })
        .collect();

    LocalePackCompatibilityView {
        record_kind: LOCALE_PACK_COMPATIBILITY_VIEW_RECORD_KIND.to_owned(),
        audience,
        report_id: report.report_id.clone(),
        target_build_identity_ref: report.target_build_identity_ref.clone(),
        source_language_locale: report.source_language_locale.clone(),
        total_packs: report.summary.total_packs,
        renderable_packs: report.summary.renderable_packs,
        degraded_source_language_packs: report.summary.degraded_source_language_packs,
        total_missing_keys: report.summary.total_missing_keys,
        guardrail_clean: report.summary.guardrail_clean,
        rows,
        raw_translated_body_omitted: true,
    }
}

/// Projects the user-facing compatibility view (Settings / Help/About / diagnostics).
pub fn project_user_locale_pack_compatibility() -> LocalePackCompatibilityView {
    project_locale_pack_compatibility(CompatibilityViewAudience::User)
}

/// Projects the metadata-only support-export compatibility view.
pub fn project_support_locale_pack_compatibility() -> LocalePackCompatibilityView {
    project_locale_pack_compatibility(CompatibilityViewAudience::SupportExport)
}
