//! Stabilize system-browser auth, passkey-capable step-up, recovery flows, and
//! exact return-path labeling.
//!
//! This module promotes the beta audit rows in
//! [`crate::system_browser::beta`] and [`crate::passkey`] to one
//! evidence-backed stable proof packet whose qualification is derived
//! from both audits rather than asserted from a spreadsheet row.
//!
//! The stable claim holds when **both** beta pages audit clean *and*:
//!
//! 1. Every claimed identity row defaults to system-browser auth or quotes an
//!    explicit closed policy exception ([`SystemBrowserPolicyExceptionClass`]).
//! 2. Every row that claims passkey capability names a closed passkey step-up
//!    posture from the safe set ([`PasskeyStepUpPostureClass`]).
//! 3. Every reauth and recovery passkey lane preserves the original target /
//!    action identity instead of silently widening or rerouting.
//! 4. Every passkey lane whose lifecycle or outcome leaves it unsatisfied names
//!    a typed fallback path.
//!
//! When any of those conditions fails, the row's qualification is
//! automatically narrowed below Stable and a
//! [`SystemBrowserAuthStabilizeNarrowReasonClass`] token explains why. A
//! narrowed row never inherits adjacent green rows' stability claim.
//!
//! Two hard guardrails cannot be papered over:
//!
//! - **No authority widening.** A defect of kind
//!   [`AuthorityWideningOnReturn`][crate::system_browser::beta::SystemBrowserReturnPathBetaDefectKind::ReturnWidensAuthorityScope]
//!   or [`GrantedAuthorityWidensRequested`][crate::passkey::PasskeyStepUpBetaDefectKind::GrantedAuthorityWidensRequested]
//!   withdraws the row to
//!   [`SystemBrowserAuthStabilizeQualificationClass::Withdrawn`].
//! - **No identity widening.** A defect of kind
//!   [`ReauthOrRecoveryWidened`][crate::passkey::PasskeyStepUpBetaDefectKind::ReauthOrRecoveryWidened]
//!   also withdraws the row to
//!   [`SystemBrowserAuthStabilizeQualificationClass::Withdrawn`].
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! tokens, closed-vocabulary tokens, plain-language labels, and opaque refs
//! only. Raw credentials, session tokens, plaintext user identity, and raw
//! callback parameters stay outside the support boundary.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/enterprise/m4/stabilize-system-browser-auth-passkey-capable-step-up.md`
//! - Artifact summary: `artifacts/enterprise/m4/stabilize-system-browser-auth-passkey-capable-step-up.md`
//! - Contract refs consumed: [`SYSTEM_BROWSER_AUTH_STABILIZE_SHARED_CONTRACT_REF`]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::passkey::{
    seeded_passkey_step_up_beta_page, PasskeyStepUpBetaDefectKind, PasskeyStepUpBetaPage,
    PasskeyStepUpBetaSupportExport,
};
use crate::system_browser::beta::{
    seeded_system_browser_return_paths_beta_page, SystemBrowserReturnPathBetaDefectKind,
    SystemBrowserReturnPathsBetaPage, SystemBrowserReturnPathsBetaSupportExport,
};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const SYSTEM_BROWSER_AUTH_STABILIZE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const SYSTEM_BROWSER_AUTH_STABILIZE_SHARED_CONTRACT_REF: &str =
    "auth:system_browser_auth_stabilize:v1";

/// Record-kind tag for [`SystemBrowserAuthStabilizePage`] payloads.
pub const SYSTEM_BROWSER_AUTH_STABILIZE_PAGE_RECORD_KIND: &str =
    "auth_system_browser_auth_stabilize_page_record";

/// Record-kind tag for [`SystemBrowserAuthStabilizeRow`] payloads.
pub const SYSTEM_BROWSER_AUTH_STABILIZE_ROW_RECORD_KIND: &str =
    "auth_system_browser_auth_stabilize_row_record";

/// Record-kind tag for [`SystemBrowserAuthStabilizeDefect`] payloads.
pub const SYSTEM_BROWSER_AUTH_STABILIZE_DEFECT_RECORD_KIND: &str =
    "auth_system_browser_auth_stabilize_defect_record";

/// Record-kind tag for [`SystemBrowserAuthStabilizeSupportExport`] payloads.
pub const SYSTEM_BROWSER_AUTH_STABILIZE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "auth_system_browser_auth_stabilize_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const SYSTEM_BROWSER_AUTH_STABILIZE_DOC_REF: &str =
    "docs/enterprise/m4/stabilize-system-browser-auth-passkey-capable-step-up.md";

/// Repo-relative path of the artifact summary for this lane.
pub const SYSTEM_BROWSER_AUTH_STABILIZE_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/stabilize-system-browser-auth-passkey-capable-step-up.md";

/// Repo-relative path of the system-browser return-paths beta contract doc.
pub const SYSTEM_BROWSER_RETURN_PATHS_CONTRACT_REF: &str =
    "docs/auth/system_browser_callback_packet.md";

/// Repo-relative path of the passkey step-up beta contract doc.
pub const PASSKEY_STEP_UP_CONTRACT_REF: &str =
    "docs/auth/managed_auth_and_session_continuity_contract.md";

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall packet and for individual rows.
///
/// The tier is derived, not asserted: it is set by comparing the audit defect
/// lists from both beta pages. A caller may never bump a row to `stable`
/// without a clean audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemBrowserAuthStabilizeQualificationClass {
    /// Both beta pages audit clean and all four stability conditions hold.
    Stable,
    /// One or more non-critical defects prevent the stable claim.
    Beta,
    /// Structural gaps prevent a beta claim (e.g. missing return-path labels).
    Preview,
    /// A critical defect (authority widening, identity widening) withdraws the
    /// row entirely.
    Withdrawn,
}

impl SystemBrowserAuthStabilizeQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when this qualification tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// True when the row is in a claimed-stable posture (stable or beta).
    pub const fn is_claimable(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }
}

/// Typed reason a row was narrowed below
/// [`SystemBrowserAuthStabilizeQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemBrowserAuthStabilizeNarrowReasonClass {
    /// No narrowing required — the row qualifies as stable.
    NotNarrowed,
    /// The system-browser return-paths beta page has one or more defects.
    ReturnPathsBetaPageHasDefects,
    /// The passkey step-up beta page has one or more defects.
    PasskeyBetaPageHasDefects,
    /// The packet does not default to system-browser auth or a declared
    /// exception for every claimed row.
    SystemBrowserDefaultMissing,
    /// The packet does not cover passkey step-up for every row that claims
    /// passkey capability.
    PasskeyStepUpNotCoveredWhenClaimed,
    /// A reauth or recovery lane did not preserve the target / action identity.
    ReauthOrRecoveryIdentityNotPreserved,
    /// A passkey lane left unsatisfied did not name a typed fallback.
    UnsatisfiedPasskeyWithoutFallback,
    /// A granted authority scope widens the requested scope (withdraws the row).
    AuthorityWideningOnReturn,
    /// A reauth or recovery lane widened the target / action identity
    /// (withdraws the row).
    IdentityWideningOnReturn,
}

impl SystemBrowserAuthStabilizeNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::ReturnPathsBetaPageHasDefects => "return_paths_beta_page_has_defects",
            Self::PasskeyBetaPageHasDefects => "passkey_beta_page_has_defects",
            Self::SystemBrowserDefaultMissing => "system_browser_default_missing",
            Self::PasskeyStepUpNotCoveredWhenClaimed => "passkey_step_up_not_covered_when_claimed",
            Self::ReauthOrRecoveryIdentityNotPreserved => {
                "reauth_or_recovery_identity_not_preserved"
            }
            Self::UnsatisfiedPasskeyWithoutFallback => "unsatisfied_passkey_without_fallback",
            Self::AuthorityWideningOnReturn => "authority_widening_on_return",
            Self::IdentityWideningOnReturn => "identity_widening_on_return",
        }
    }

    /// True when this reason is a hard guardrail that withdraws the row.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::AuthorityWideningOnReturn | Self::IdentityWideningOnReturn
        )
    }
}

// ---------------------------------------------------------------------------
// Row and summary types
// ---------------------------------------------------------------------------

/// Stability qualification for one auth flow row in the stabilize packet.
///
/// Each row binds the row id from the system-browser return-paths beta page
/// and the row id from the passkey step-up beta page (when one exists for the
/// same flow), plus the derived qualification tier and any narrow reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemBrowserAuthStabilizeRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    /// Row id from [`SystemBrowserReturnPathsBetaPage`] that this row is
    /// bound to.
    pub return_path_row_id: String,
    /// Optional row id from [`PasskeyStepUpBetaPage`] covering the same
    /// auth flow. `None` when the flow does not claim a passkey lane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passkey_row_id: Option<String>,
    /// Derived qualification tier.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

/// Aggregate banner emitted with the stabilize page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SystemBrowserAuthStabilizeSummary {
    pub row_count: usize,
    pub stable_row_count: usize,
    pub beta_row_count: usize,
    pub preview_row_count: usize,
    pub withdrawn_row_count: usize,
    pub return_path_beta_defect_count: usize,
    pub passkey_beta_defect_count: usize,
    pub system_browser_default_row_count: usize,
    pub passkey_capable_row_count: usize,
    pub overall_qualification_token: String,
}

impl SystemBrowserAuthStabilizeSummary {
    fn from_rows(
        rows: &[SystemBrowserAuthStabilizeRow],
        return_path_page: &SystemBrowserReturnPathsBetaPage,
        passkey_page: &PasskeyStepUpBetaPage,
    ) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
        }
        let overall = if withdrawn > 0 {
            SystemBrowserAuthStabilizeQualificationClass::Withdrawn
        } else if preview > 0 {
            SystemBrowserAuthStabilizeQualificationClass::Preview
        } else if beta > 0 {
            SystemBrowserAuthStabilizeQualificationClass::Beta
        } else {
            SystemBrowserAuthStabilizeQualificationClass::Stable
        };
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            return_path_beta_defect_count: return_path_page.defects.len(),
            passkey_beta_defect_count: passkey_page.defects.len(),
            system_browser_default_row_count: return_path_page
                .summary
                .system_browser_default_row_count,
            passkey_capable_row_count: return_path_page.summary.passkey_capable_row_count,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defects
// ---------------------------------------------------------------------------

/// Typed defect emitted by the stabilize page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemBrowserAuthStabilizeDefect {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub defect_id: String,
    pub narrow_reason: SystemBrowserAuthStabilizeNarrowReasonClass,
    pub narrow_reason_token: String,
    pub source: String,
    pub note: String,
}

impl SystemBrowserAuthStabilizeDefect {
    fn new(
        narrow_reason: SystemBrowserAuthStabilizeNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: SYSTEM_BROWSER_AUTH_STABILIZE_DEFECT_RECORD_KIND.to_owned(),
            schema_version: SYSTEM_BROWSER_AUTH_STABILIZE_SCHEMA_VERSION,
            shared_contract_ref: SYSTEM_BROWSER_AUTH_STABILIZE_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "auth:defect:stabilize-system-browser-auth:{}:{}",
                narrow_reason.as_str(),
                &source_str
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source: source_str,
            note: note.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Main page
// ---------------------------------------------------------------------------

/// Stable proof packet for system-browser auth, passkey-capable step-up,
/// recovery flows, and exact return-path labeling.
///
/// The packet is the single inspectable record that proves the stable claim
/// for this lane. Dashboards, docs, Help/About surfaces, and support exports
/// should ingest it rather than cloning status text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemBrowserAuthStabilizePage {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub page_id: String,
    pub page_label: String,
    pub generated_at: String,
    pub summary: SystemBrowserAuthStabilizeSummary,
    pub rows: Vec<SystemBrowserAuthStabilizeRow>,
    pub defects: Vec<SystemBrowserAuthStabilizeDefect>,
    /// The system-browser return-paths beta page embedded as evidence.
    pub return_paths_beta_page: SystemBrowserReturnPathsBetaPage,
    /// The passkey step-up beta page embedded as evidence.
    pub passkey_step_up_beta_page: PasskeyStepUpBetaPage,
}

impl SystemBrowserAuthStabilizePage {
    /// Build the stabilize page from the two beta pages.
    ///
    /// Rows are derived by correlating return-path rows with passkey rows
    /// that share the same `source_claim_row_ref`. The stability
    /// qualification for each row is computed from the combined defect lists.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        return_paths_beta_page: SystemBrowserReturnPathsBetaPage,
        passkey_step_up_beta_page: PasskeyStepUpBetaPage,
    ) -> Self {
        let defects = audit_stabilize_pages(&return_paths_beta_page, &passkey_step_up_beta_page);
        let rows = derive_stabilize_rows(
            &return_paths_beta_page,
            &passkey_step_up_beta_page,
            &defects,
        );
        let summary = SystemBrowserAuthStabilizeSummary::from_rows(
            &rows,
            &return_paths_beta_page,
            &passkey_step_up_beta_page,
        );
        Self {
            record_kind: SYSTEM_BROWSER_AUTH_STABILIZE_PAGE_RECORD_KIND.to_owned(),
            schema_version: SYSTEM_BROWSER_AUTH_STABILIZE_SCHEMA_VERSION,
            shared_contract_ref: SYSTEM_BROWSER_AUTH_STABILIZE_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows,
            defects,
            return_paths_beta_page,
            passkey_step_up_beta_page,
        }
    }

    /// True when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == SystemBrowserAuthStabilizeQualificationClass::Stable.as_str()
    }

    /// True when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// True when every claimed row defaults to system-browser auth or quotes
    /// an explicit policy exception.
    pub fn defaults_to_system_browser_or_explicit_exception(&self) -> bool {
        self.return_paths_beta_page
            .defaults_to_system_browser_or_explicit_exception()
    }

    /// True when every row that claims passkey capability names a closed
    /// step-up posture.
    pub fn passkey_step_up_present_when_claimed(&self) -> bool {
        self.return_paths_beta_page
            .passkey_step_up_present_when_claimed()
    }

    /// True when every reauth / recovery row preserves the original target /
    /// action identity.
    pub fn reauth_and_recovery_preserve_target_action_identity(&self) -> bool {
        self.passkey_step_up_beta_page
            .reauth_and_recovery_preserve_target_action_identity()
    }

    /// True when every unsatisfied passkey lane names a typed fallback.
    pub fn fallback_named_when_passkey_unavailable(&self) -> bool {
        self.passkey_step_up_beta_page
            .fallback_named_when_passkey_unavailable()
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that quotes the stabilize page plus a metadata-safe
/// defect roll-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemBrowserAuthStabilizeSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub export_id: String,
    pub generated_at: String,
    pub page: SystemBrowserAuthStabilizePage,
    pub narrow_reasons_present: Vec<SystemBrowserAuthStabilizeNarrowReasonClass>,
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    pub return_paths_support_export: SystemBrowserReturnPathsBetaSupportExport,
    pub passkey_support_export: PasskeyStepUpBetaSupportExport,
    pub raw_private_material_excluded: bool,
}

impl SystemBrowserAuthStabilizeSupportExport {
    /// Wrap a stabilize page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: SystemBrowserAuthStabilizePage,
    ) -> Self {
        let mut reasons: Vec<SystemBrowserAuthStabilizeNarrowReasonClass> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }
        reasons.sort();
        let generated = generated_at.into();
        let return_paths_export = SystemBrowserReturnPathsBetaSupportExport::from_page(
            format!("{}-return-paths", export_id.into()),
            generated.clone(),
            page.return_paths_beta_page.clone(),
        );
        let passkey_export = PasskeyStepUpBetaSupportExport::from_page(
            format!("{}-passkey", return_paths_export.export_id),
            generated.clone(),
            page.passkey_step_up_beta_page.clone(),
        );
        Self {
            record_kind: SYSTEM_BROWSER_AUTH_STABILIZE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SYSTEM_BROWSER_AUTH_STABILIZE_SCHEMA_VERSION,
            shared_contract_ref: SYSTEM_BROWSER_AUTH_STABILIZE_SHARED_CONTRACT_REF.to_owned(),
            export_id: return_paths_export.export_id.clone(),
            generated_at: generated,
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            return_paths_support_export: return_paths_export,
            passkey_support_export: passkey_export,
            raw_private_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Audit and validate functions
// ---------------------------------------------------------------------------

/// Re-run the stabilize audit over the two beta pages without rebuilding the
/// full page. The headless inspector and integration tests use this to surface
/// a regression when a change is made to either beta page.
pub fn audit_stabilize_pages(
    return_paths_page: &SystemBrowserReturnPathsBetaPage,
    passkey_page: &PasskeyStepUpBetaPage,
) -> Vec<SystemBrowserAuthStabilizeDefect> {
    let mut defects: Vec<SystemBrowserAuthStabilizeDefect> = Vec::new();

    // Check 1: return-paths beta page must be clean.
    if !return_paths_page.defects.is_empty() {
        let has_widening = return_paths_page.defects.iter().any(|d| {
            d.defect_kind == SystemBrowserReturnPathBetaDefectKind::ReturnWidensAuthorityScope
        });
        if has_widening {
            defects.push(SystemBrowserAuthStabilizeDefect::new(
                SystemBrowserAuthStabilizeNarrowReasonClass::AuthorityWideningOnReturn,
                "return_paths_beta_page",
                "system-browser return-paths beta page has an authority-widening defect; row is withdrawn",
            ));
        } else {
            defects.push(SystemBrowserAuthStabilizeDefect::new(
                SystemBrowserAuthStabilizeNarrowReasonClass::ReturnPathsBetaPageHasDefects,
                "return_paths_beta_page",
                "system-browser return-paths beta page has one or more defects; row is narrowed to beta",
            ));
        }
    }

    // Check 2: passkey step-up beta page must be clean.
    if !passkey_page.defects.is_empty() {
        let has_widening = passkey_page
            .defects
            .iter()
            .any(|d| d.defect_kind == PasskeyStepUpBetaDefectKind::GrantedAuthorityWidensRequested);
        let has_identity_widening = passkey_page
            .defects
            .iter()
            .any(|d| d.defect_kind == PasskeyStepUpBetaDefectKind::ReauthOrRecoveryWidened);
        if has_widening {
            defects.push(SystemBrowserAuthStabilizeDefect::new(
                SystemBrowserAuthStabilizeNarrowReasonClass::AuthorityWideningOnReturn,
                "passkey_step_up_beta_page",
                "passkey step-up beta page has an authority-widening defect; row is withdrawn",
            ));
        } else if has_identity_widening {
            defects.push(SystemBrowserAuthStabilizeDefect::new(
                SystemBrowserAuthStabilizeNarrowReasonClass::IdentityWideningOnReturn,
                "passkey_step_up_beta_page",
                "passkey step-up beta page has a reauth/recovery identity-widening defect; row is withdrawn",
            ));
        } else {
            defects.push(SystemBrowserAuthStabilizeDefect::new(
                SystemBrowserAuthStabilizeNarrowReasonClass::PasskeyBetaPageHasDefects,
                "passkey_step_up_beta_page",
                "passkey step-up beta page has one or more defects; row is narrowed to beta",
            ));
        }
    }

    // Check 3: system-browser default or explicit exception for every row.
    if !return_paths_page.defaults_to_system_browser_or_explicit_exception() {
        defects.push(SystemBrowserAuthStabilizeDefect::new(
            SystemBrowserAuthStabilizeNarrowReasonClass::SystemBrowserDefaultMissing,
            "return_paths_beta_page",
            "not every claimed identity row defaults to system-browser auth or quotes an explicit policy exception",
        ));
    }

    // Check 4: passkey step-up present when claimed.
    if !return_paths_page.passkey_step_up_present_when_claimed() {
        defects.push(SystemBrowserAuthStabilizeDefect::new(
            SystemBrowserAuthStabilizeNarrowReasonClass::PasskeyStepUpNotCoveredWhenClaimed,
            "return_paths_beta_page",
            "not every row that claims passkey capability names a closed step-up posture token",
        ));
    }

    // Check 5: reauth and recovery preserve the target / action identity.
    if !passkey_page.reauth_and_recovery_preserve_target_action_identity() {
        defects.push(SystemBrowserAuthStabilizeDefect::new(
            SystemBrowserAuthStabilizeNarrowReasonClass::ReauthOrRecoveryIdentityNotPreserved,
            "passkey_step_up_beta_page",
            "a reauth or recovery lane did not preserve the original target / action identity",
        ));
    }

    // Check 6: unsatisfied passkey lanes name a typed fallback.
    if !passkey_page.fallback_named_when_passkey_unavailable() {
        defects.push(SystemBrowserAuthStabilizeDefect::new(
            SystemBrowserAuthStabilizeNarrowReasonClass::UnsatisfiedPasskeyWithoutFallback,
            "passkey_step_up_beta_page",
            "a passkey lane left unsatisfied did not name a typed fallback path",
        ));
    }

    defects
}

/// Validate a stabilize page; returns `Ok` on a clean audit.
pub fn validate_system_browser_auth_stabilize_page(
    page: &SystemBrowserAuthStabilizePage,
) -> Result<(), Vec<SystemBrowserAuthStabilizeDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Derive per-row stability qualification by correlating the return-path rows
/// and passkey rows using their `source_claim_row_ref`.
fn derive_stabilize_rows(
    return_paths_page: &SystemBrowserReturnPathsBetaPage,
    passkey_page: &PasskeyStepUpBetaPage,
    page_defects: &[SystemBrowserAuthStabilizeDefect],
) -> Vec<SystemBrowserAuthStabilizeRow> {
    let has_authority_widening = page_defects.iter().any(|d| {
        d.narrow_reason == SystemBrowserAuthStabilizeNarrowReasonClass::AuthorityWideningOnReturn
    });
    let has_identity_widening = page_defects.iter().any(|d| {
        d.narrow_reason == SystemBrowserAuthStabilizeNarrowReasonClass::IdentityWideningOnReturn
    });
    let has_critical = has_authority_widening || has_identity_widening;

    let overall_qual = if has_critical {
        SystemBrowserAuthStabilizeQualificationClass::Withdrawn
    } else if !page_defects.is_empty() {
        SystemBrowserAuthStabilizeQualificationClass::Beta
    } else {
        SystemBrowserAuthStabilizeQualificationClass::Stable
    };

    let narrow_reason = if has_authority_widening {
        SystemBrowserAuthStabilizeNarrowReasonClass::AuthorityWideningOnReturn
    } else if has_identity_widening {
        SystemBrowserAuthStabilizeNarrowReasonClass::IdentityWideningOnReturn
    } else if !page_defects.is_empty() {
        page_defects[0].narrow_reason
    } else {
        SystemBrowserAuthStabilizeNarrowReasonClass::NotNarrowed
    };

    // Build a lookup from source_claim_row_ref → passkey row id.
    let passkey_by_claim_ref: BTreeMap<&str, &str> = passkey_page
        .rows
        .iter()
        .map(|r| (r.source_claim_row_ref.as_str(), r.row_id.as_str()))
        .collect();

    return_paths_page
        .rows
        .iter()
        .map(|rp_row| {
            let passkey_row_id = passkey_by_claim_ref
                .get(rp_row.source_claim_row_ref.as_str())
                .map(|s| s.to_string());
            let summary = build_row_summary(
                rp_row,
                passkey_row_id.as_deref(),
                &overall_qual,
                narrow_reason,
            );
            SystemBrowserAuthStabilizeRow {
                record_kind: SYSTEM_BROWSER_AUTH_STABILIZE_ROW_RECORD_KIND.to_owned(),
                schema_version: SYSTEM_BROWSER_AUTH_STABILIZE_SCHEMA_VERSION,
                shared_contract_ref: SYSTEM_BROWSER_AUTH_STABILIZE_SHARED_CONTRACT_REF.to_owned(),
                return_path_row_id: rp_row.row_id.clone(),
                passkey_row_id,
                qualification_token: overall_qual.as_str().to_owned(),
                narrow_reason_token: narrow_reason.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn build_row_summary(
    rp_row: &crate::system_browser::beta::SystemBrowserReturnPathBetaRow,
    passkey_row_id: Option<&str>,
    qual: &SystemBrowserAuthStabilizeQualificationClass,
    narrow_reason: SystemBrowserAuthStabilizeNarrowReasonClass,
) -> String {
    match qual {
        SystemBrowserAuthStabilizeQualificationClass::Stable => format!(
            "Row '{}' qualifies stable: system-browser default / passkey posture verified, \
             return-path labels complete, no authority widening, identity preserved across \
             reauth and recovery, and fallback named when passkey unavailable. \
             Passkey row: {}.",
            rp_row.row_id,
            passkey_row_id.unwrap_or("none (account-free local)")
        ),
        _ => format!(
            "Row '{}' narrowed to {} ({}): see defect list for details. \
             Passkey row: {}.",
            rp_row.row_id,
            qual.as_str(),
            narrow_reason.as_str(),
            passkey_row_id.unwrap_or("none")
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded stable packet that the live shell, the headless inspector,
/// and the integration test all consume.
///
/// The seeded page seeds zero defects: both beta pages audit clean, the
/// system-browser default is verified for every claimed row, passkey step-up
/// is verified for every row that claims it, reauth and recovery lanes
/// preserve the original target / action identity, and every unsatisfied lane
/// names a typed fallback.
pub fn seeded_system_browser_auth_stabilize_page() -> SystemBrowserAuthStabilizePage {
    SystemBrowserAuthStabilizePage::new(
        "auth:system_browser_auth_stabilize:default",
        "System-browser auth, passkey-capable step-up, recovery flows, and exact return-path labeling (stable)",
        "2026-06-01T00:00:00Z",
        seeded_system_browser_return_paths_beta_page(),
        seeded_passkey_step_up_beta_page(),
    )
}
