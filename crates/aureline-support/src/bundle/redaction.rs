//! Local-first default redaction profile.
//!
//! Mirrors the seed redaction profile at
//! `/fixtures/support/redaction_profiles/local_first_default.yaml`. The
//! profile is intentionally narrow: build/install/policy metadata embeds
//! by default, code-adjacent rows require explicit opt-in, raw secret /
//! shell-history rows are prohibited, and raw traces / dumps stay local-
//! only. The companion guide at
//! `/docs/support/support_bundle_redaction_guide.md` documents the rules
//! a reviewer needs to interpret a manifest produced by this seed.

use super::vocabulary::{
    DiagnosticDataClass, ExcludedReasonClass, HighRiskContentClass, RedactionState,
    ReviewDecisionClass,
};

/// Stable rule ref for the metadata-core rule. Mirrors
/// `support.redaction.local_first_default.metadata_core` in the YAML
/// profile.
pub const RULE_REF_METADATA_CORE: &str = "support.redaction.local_first_default.metadata_core";
/// Stable rule ref for the by-reference managed-packet rule.
pub const RULE_REF_MANAGED_REFS: &str = "support.redaction.local_first_default.managed_refs";
/// Stable rule ref for the local-only raw-trace rule.
pub const RULE_REF_HIGH_RISK_LOCAL: &str = "support.redaction.local_first_default.high_risk_local";
/// Stable rule ref for the code-adjacent review-required rule.
pub const RULE_REF_REVIEW_CODE: &str = "support.redaction.local_first_default.review_code";
/// Stable rule ref for the always-excluded full-shell-history rule.
pub const RULE_REF_EXCLUDE_SHELL_HISTORY: &str =
    "support.redaction.local_first_default.exclude_shell_history";

/// Stable id for the local-first default redaction profile.
pub const LOCAL_FIRST_DEFAULT_PROFILE_REF: &str = "support.redaction.local_first_default";

/// Resolved default posture for one preview row, derived from the row's
/// data class and high-risk subtype. Keeps the rule-application logic in
/// one place so the chrome and the export writer agree on the seed's
/// defaults.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefaultRedactionPosture {
    /// Visible state shown to the reviewer before export.
    pub redaction_state: RedactionState,
    /// Default review decision when no user override is in play.
    pub decision_class: ReviewDecisionClass,
    /// Stable rule refs that map this posture back to the YAML profile.
    pub rule_refs: Vec<&'static str>,
    /// True when the row is excluded from the export by default and the
    /// manifest must record an [`ExcludedClass`] entry for it.
    pub is_excluded_from_export: bool,
    /// When [`Self::is_excluded_from_export`] is true, the typed reason
    /// that explains why. `None` for included rows.
    pub exclusion_reason: Option<ExcludedReasonClass>,
}

/// The local-first default redaction profile.
pub struct LocalFirstDefaults;

impl LocalFirstDefaults {
    /// Stable profile ref written into the manifest.
    pub const PROFILE_REF: &'static str = LOCAL_FIRST_DEFAULT_PROFILE_REF;

    /// Reviewer-visible summary line written onto the manifest's
    /// `redaction_report.reviewer_visible_summary`. Kept short so the
    /// chrome can render it under the preview without truncation tricks.
    pub const REVIEWER_SUMMARY_DEFAULT_OK: &'static str =
        "Local-first defaults applied: metadata included, code-adjacent items require opt-in, \
         raw traces stay local-only, secret-bearing material is prohibited.";

    /// Reviewer-visible summary line when at least one prohibited item
    /// was queued and rewritten to `prohibited` in the preview.
    pub const REVIEWER_SUMMARY_PROHIBITED_PRESENT: &'static str =
        "Local-first defaults applied: at least one queued row was prohibited and removed from \
         export. The row stays visible in preview with a 'Prohibited — never exported' chip.";

    /// Project the default posture for one row.
    pub fn posture_for(
        data_class: DiagnosticDataClass,
        high_risk_content_class: HighRiskContentClass,
    ) -> DefaultRedactionPosture {
        match (data_class, high_risk_content_class) {
            (DiagnosticDataClass::MetadataOnly, _)
            | (DiagnosticDataClass::EnvironmentAdjacent, _) => DefaultRedactionPosture {
                redaction_state: RedactionState::NotRequiredMetadata,
                decision_class: ReviewDecisionClass::IncludedDefault,
                rule_refs: vec![RULE_REF_METADATA_CORE, RULE_REF_MANAGED_REFS],
                is_excluded_from_export: false,
                exclusion_reason: None,
            },
            (DiagnosticDataClass::CodeAdjacent, _) => DefaultRedactionPosture {
                redaction_state: RedactionState::OmittedPendingOptIn,
                decision_class: ReviewDecisionClass::OmittedUserDeselected,
                rule_refs: vec![RULE_REF_REVIEW_CODE],
                is_excluded_from_export: true,
                exclusion_reason: Some(ExcludedReasonClass::AwaitingExplicitOptIn),
            },
            (DiagnosticDataClass::HighRisk, HighRiskContentClass::SecretBearing) => {
                DefaultRedactionPosture {
                    redaction_state: RedactionState::Prohibited,
                    decision_class: ReviewDecisionClass::OmittedProhibited,
                    rule_refs: vec![RULE_REF_HIGH_RISK_LOCAL],
                    is_excluded_from_export: true,
                    exclusion_reason: Some(ExcludedReasonClass::ProhibitedSecretOrToken),
                }
            }
            (DiagnosticDataClass::HighRisk, HighRiskContentClass::FullShellHistory) => {
                DefaultRedactionPosture {
                    redaction_state: RedactionState::Prohibited,
                    decision_class: ReviewDecisionClass::OmittedProhibited,
                    rule_refs: vec![RULE_REF_EXCLUDE_SHELL_HISTORY],
                    is_excluded_from_export: true,
                    exclusion_reason: Some(ExcludedReasonClass::ProhibitedFullShellHistory),
                }
            }
            (DiagnosticDataClass::HighRisk, _) => DefaultRedactionPosture {
                redaction_state: RedactionState::RetainedLocalOnly,
                decision_class: ReviewDecisionClass::RetainedLocalOnly,
                rule_refs: vec![RULE_REF_HIGH_RISK_LOCAL],
                is_excluded_from_export: true,
                exclusion_reason: Some(ExcludedReasonClass::RetainedLocalOnlyPendingReview),
            },
        }
    }

    /// Explicit reason copy for the [`ExcludedClass.explicit_reason`]
    /// field. Reviewers see this verbatim in the manifest, so the
    /// sentences are kept short and free of jargon.
    pub fn explicit_reason_for(reason: ExcludedReasonClass) -> &'static str {
        match reason {
            ExcludedReasonClass::AwaitingExplicitOptIn => {
                "Code-adjacent row is omitted by default until the reviewer opts in explicitly."
            }
            ExcludedReasonClass::ProhibitedSecretOrToken => {
                "Secret-bearing content is prohibited from local-first export and stays out of \
                 the bundle even when queued."
            }
            ExcludedReasonClass::ProhibitedFullShellHistory => {
                "Full shell history is prohibited from local-first export by the local default \
                 redaction profile."
            }
            ExcludedReasonClass::RetainedLocalOnlyPendingReview => {
                "High-risk raw capture is retained on the local machine and not embedded in the \
                 export."
            }
            ExcludedReasonClass::PolicyDenied => {
                "Active policy denies including this row in the export."
            }
            ExcludedReasonClass::UserDeselected => {
                "Reviewer deselected this row from the bundle preview."
            }
            ExcludedReasonClass::NotRequested => {
                "Row class was not requested for this bundle preview."
            }
            ExcludedReasonClass::SourceUnavailableOrExpired => {
                "Underlying source for this row was unavailable or expired before preview."
            }
            ExcludedReasonClass::NotCollectedOnThisPlatform => {
                "Row class is not collected on this platform."
            }
        }
    }
}
