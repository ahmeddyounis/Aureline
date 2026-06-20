//! Shell-side inspector for extension and companion locale support.
//!
//! This projects the canonical [`aureline_i18n`] contributed-locale support
//! report into the view the shell renders on Settings, Help/About, support
//! export, and the extensions surface. It owns no localization truth: it reads
//! the per-surface apply-or-degrade decision, the degrade reason, the
//! missing-support disclosure, and the localization issue source from the
//! report so every audience quotes the same numbers.
//!
//! The view answers the support question the first-party compatibility view
//! cannot: *whose* localization is wrong. Every row is attributed to a
//! [`LocalizationIssueSourceClass`], the view counts issues for all three source
//! classes (first-party, extension, companion) by joining the first-party
//! compatibility report, and it confirms that host-stable trust, policy,
//! capability, and lifecycle labels stayed canonical even where contributed
//! strings localized.

use serde::{Deserialize, Serialize};

use aureline_i18n::{
    seeded_contributed_locale_support_report, seeded_locale_pack_compatibility_report,
    ContributedDegradeReason, LocalizationIssueSourceClass, PackApplicationDecision,
    ALL_HOST_STABLE_LABEL_CLASSES,
};

/// Record kind for [`ContributedLocaleSupportView`].
pub const CONTRIBUTED_LOCALE_SUPPORT_VIEW_RECORD_KIND: &str =
    "shell_contributed_locale_support_view";

/// Who is reading the contributed-locale support view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContributedSupportAudience {
    /// User-facing Settings, Help/About, extensions, or diagnostics surface.
    User,
    /// Metadata-only support export.
    SupportExport,
}

/// One export-safe contributed-surface row rendered by the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContributedSupportRowView {
    /// Stable row id.
    pub row_id: String,
    /// Manifest this row resolves.
    pub manifest_id: String,
    /// Owning extension or companion-surface id.
    pub owner_id: String,
    /// Source attribution support uses to route the issue.
    pub issue_source_class: LocalizationIssueSourceClass,
    /// User-requested locale.
    pub requested_locale: String,
    /// Locale that produces rendered text after evaluation.
    pub effective_locale: String,
    /// Apply-or-degrade decision.
    pub application_decision: PackApplicationDecision,
    /// Reason a degrade occurred, when applicable.
    pub degrade_reason: ContributedDegradeReason,
    /// Whether the row degraded fully to host source language.
    pub degraded_to_source_language: bool,
    /// Whether the row lacks localized support on a claimed localized profile.
    pub missing_support_on_claimed_profile: bool,
    /// Whether host-stable trust/policy/capability/lifecycle labels stayed canonical.
    pub host_stable_labels_canonical: bool,
    /// Whether this row backs a claimed localized profile.
    pub claimed_localized_profile: bool,
    /// Same-surface host source-language route.
    pub open_in_source_language_route_ref: String,
    /// Export-safe label.
    pub presentation_label: String,
}

/// Localization issue counts attributed to each source class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssueCountsBySource {
    /// First-party host packs with a degraded row in the delivery lane.
    pub first_party_pack: usize,
    /// Extension packs with a support-defect degrade.
    pub extension_pack: usize,
    /// Companion overlays with a support-defect degrade.
    pub companion_overlay: usize,
}

/// Inspectable contributed-locale support view shared by shell surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContributedLocaleSupportView {
    /// Boundary record kind.
    pub record_kind: String,
    /// Who is reading the view.
    pub audience: ContributedSupportAudience,
    /// Source report id.
    pub report_id: String,
    /// First-party compatibility report this view joins against.
    pub first_party_compatibility_report_ref: String,
    /// Active build identity the report was evaluated against.
    pub target_build_identity_ref: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Host-stable label classes held canonical.
    pub host_stable_label_classes_protected: usize,
    /// Whether every contributed row kept host-stable labels canonical.
    pub host_stable_labels_canonical: bool,
    /// Whether no contributed pack overrides host-stable labels and degrades disclose.
    pub guardrail_clean: bool,
    /// Per-surface contributed rows (extension and companion).
    pub rows: Vec<ContributedSupportRowView>,
    /// Localization issue counts attributed to each source class.
    pub issue_counts_by_source: IssueCountsBySource,
    /// Always true; translated body text never crosses this boundary.
    pub raw_translated_body_omitted: bool,
}

impl ContributedLocaleSupportView {
    /// Returns the row for a row id, when present.
    pub fn row(&self, row_id: &str) -> Option<&ContributedSupportRowView> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }

    /// Returns the rows attributed to one issue source class.
    pub fn rows_for_source(
        &self,
        source: LocalizationIssueSourceClass,
    ) -> impl Iterator<Item = &ContributedSupportRowView> {
        self.rows
            .iter()
            .filter(move |row| row.issue_source_class == source)
    }

    /// Returns true when every contributed row kept host-stable labels canonical.
    pub fn host_labels_all_canonical(&self) -> bool {
        self.rows.iter().all(|row| row.host_stable_labels_canonical)
    }
}

/// Projects the contributed-locale support view for an audience.
pub fn project_contributed_locale_support(
    audience: ContributedSupportAudience,
) -> ContributedLocaleSupportView {
    let report = seeded_contributed_locale_support_report();
    let host_classes = ALL_HOST_STABLE_LABEL_CLASSES.to_vec();

    let rows: Vec<ContributedSupportRowView> = report
        .support_rows
        .iter()
        .map(|row| ContributedSupportRowView {
            row_id: row.row_id.clone(),
            manifest_id: row.manifest_id.clone(),
            owner_id: row.owner_id.clone(),
            issue_source_class: row.issue_source_class,
            requested_locale: row.requested_locale.clone(),
            effective_locale: row.effective_locale.clone(),
            application_decision: row.application_decision,
            degrade_reason: row.degrade_reason,
            degraded_to_source_language: row.degraded_to_source_language(),
            missing_support_on_claimed_profile: row.missing_support_on_claimed_profile,
            host_stable_labels_canonical: row.host_stable_labels_preserved == host_classes,
            claimed_localized_profile: row.claimed_localized_profile,
            open_in_source_language_route_ref: row.open_in_source_language_route_ref.clone(),
            presentation_label: row.presentation_label.clone(),
        })
        .collect();

    let extension_pack = report
        .support_rows
        .iter()
        .filter(|r| {
            r.issue_source_class == LocalizationIssueSourceClass::ExtensionPack
                && r.degrade_reason.is_support_defect()
        })
        .count();
    let companion_overlay = report
        .support_rows
        .iter()
        .filter(|r| {
            r.issue_source_class == LocalizationIssueSourceClass::CompanionOverlay
                && r.degrade_reason.is_support_defect()
        })
        .count();
    // First-party issues are owned by the delivery lane; join it so the single
    // support view can attribute all three source classes.
    let first_party_pack = seeded_locale_pack_compatibility_report()
        .rows
        .iter()
        .filter(|r| r.application_decision == PackApplicationDecision::DegradeToSourceLanguageOnly)
        .count();

    let host_stable_labels_canonical = rows.iter().all(|row| row.host_stable_labels_canonical);

    ContributedLocaleSupportView {
        record_kind: CONTRIBUTED_LOCALE_SUPPORT_VIEW_RECORD_KIND.to_owned(),
        audience,
        report_id: report.report_id.clone(),
        first_party_compatibility_report_ref: report.first_party_compatibility_report_ref.clone(),
        target_build_identity_ref: report.target_build_identity_ref.clone(),
        source_language_locale: report.source_language_locale.clone(),
        host_stable_label_classes_protected: report.summary.host_stable_label_classes_protected,
        host_stable_labels_canonical,
        guardrail_clean: report.summary.guardrail_clean,
        rows,
        issue_counts_by_source: IssueCountsBySource {
            first_party_pack,
            extension_pack,
            companion_overlay,
        },
        raw_translated_body_omitted: true,
    }
}

/// Projects the user-facing contributed-locale support view.
pub fn project_user_contributed_locale_support() -> ContributedLocaleSupportView {
    project_contributed_locale_support(ContributedSupportAudience::User)
}

/// Projects the metadata-only support-export contributed-locale view.
pub fn project_support_contributed_locale_support() -> ContributedLocaleSupportView {
    project_contributed_locale_support(ContributedSupportAudience::SupportExport)
}
