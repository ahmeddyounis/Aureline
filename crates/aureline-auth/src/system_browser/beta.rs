//! Beta projection for system-browser-default claimed identity rows with
//! passkey-capable step-up and exact return-path labeling.
//!
//! This module promotes the alpha [`super::ClaimedIdentityRow`] seed to a
//! page-level audit that proves, on every claimed identity row, that:
//!
//! 1. **System-browser default unless explicit exception.** Every claimed
//!    identity row defaults to system-browser auth. When the row picks a
//!    different default, the row MUST quote a closed
//!    [`SystemBrowserPolicyExceptionClass`] token; the validator rejects
//!    `system_browser_default_no_exception` on rows that do not pick
//!    `open_system_browser`.
//! 2. **Exact return-path labeling.** Every row MUST quote the workspace
//!    label, target label, requested-action label, return-mode token,
//!    return-anchor ref, return-origin-validation token, and the
//!    tenant/workspace match-rule token. The post-return granted scope
//!    MUST NOT widen the requested scope; widening is a typed defect.
//! 3. **Passkey-capable reauth.** Rows that claim passkey capability MUST
//!    quote a [`PasskeyStepUpPostureClass`] token from the closed safe set
//!    (`passkey_required`, `passkey_capable_offered`,
//!    `passkey_unavailable_with_fallback`). Rows that do not claim passkey
//!    capability MUST quote `passkey_not_applicable`. Falling back without
//!    naming the fallback retry-path token is a typed defect.
//! 4. **Support-export vocabulary parity.** The support row reuses the
//!    same closed-vocabulary tokens the live row paints (default action,
//!    policy exception, return-mode, requested vs granted scope, passkey
//!    posture, fallback retry path). Drift is a contract bug.
//!
//! The seeded page seeds zero defects; the validator and the headless
//! inspector are what surface a regression when a row drops a required
//! field, drifts vocabulary across the live row and the support row, or
//! widens authority without an explicit exception.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::browser_callback::{
    AccountBoundaryClass, BrowserLaunchPolicyClass, IdentityModeAlias, PreservedLocalWork,
    PreservedLocalWorkPostureClass, RetryPathClass, ReturnModeClass, ReturnOriginValidationClass,
    ReturnTenantOrWorkspaceMatchRule, TrustState,
};

use super::{
    ClaimedIdentityDefaultActionClass, ClaimedIdentityRow, ClaimedIdentityStateClass,
    StageClaimedIdentityRowRequest,
};

/// Beta schema version exported with every record.
pub const SYSTEM_BROWSER_RETURN_PATHS_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every record on the page.
pub const SYSTEM_BROWSER_RETURN_PATHS_BETA_SHARED_CONTRACT_REF: &str =
    "auth:system_browser_return_paths_beta:v1";

/// Stable record kind for [`SystemBrowserReturnPathsBetaPage`] payloads.
pub const SYSTEM_BROWSER_RETURN_PATHS_BETA_PAGE_RECORD_KIND: &str =
    "auth_system_browser_return_paths_beta_page_record";

/// Stable record kind for [`SystemBrowserReturnPathBetaRow`] payloads.
pub const SYSTEM_BROWSER_RETURN_PATHS_BETA_ROW_RECORD_KIND: &str =
    "auth_system_browser_return_paths_beta_row_record";

/// Stable record kind for [`SystemBrowserReturnPathBetaSupportRow`] payloads.
pub const SYSTEM_BROWSER_RETURN_PATHS_BETA_SUPPORT_ROW_RECORD_KIND: &str =
    "auth_system_browser_return_paths_beta_support_row_record";

/// Stable record kind for [`SystemBrowserReturnPathBetaDefect`] payloads.
pub const SYSTEM_BROWSER_RETURN_PATHS_BETA_DEFECT_RECORD_KIND: &str =
    "auth_system_browser_return_paths_beta_defect_record";

/// Stable record kind for [`SystemBrowserReturnPathsBetaSupportExport`] payloads.
pub const SYSTEM_BROWSER_RETURN_PATHS_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "auth_system_browser_return_paths_beta_support_export_record";

/// Closed vocabulary naming why a claimed row picked a non-`open_system_browser`
/// default. Rows with the system-browser default quote
/// [`SystemBrowserDefaultNoException`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemBrowserPolicyExceptionClass {
    /// Default and required posture: the row prefers system-browser auth and
    /// no policy exception was needed.
    SystemBrowserDefaultNoException,
    /// Admin policy explicitly requires device-code on this row (e.g. headless
    /// VDI, locked-down kiosk).
    AdminPolicyDeviceCodeRequired,
    /// Admin policy explicitly requires a manual-resume return on this row.
    AdminPolicyManualResumeRequired,
    /// Browser launch is unavailable (no system browser registered or platform
    /// blocked); the row falls back to device-code.
    BrowserLaunchUnavailableUseDeviceCode,
    /// The host is offline; the row falls back to stay-local continuation.
    BrowserLaunchOfflineUseStayLocal,
    /// The row is account-free local; system-browser auth is not applicable.
    AccountFreeLocalNoAuthRequired,
}

impl SystemBrowserPolicyExceptionClass {
    /// Stable schema token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemBrowserDefaultNoException => "system_browser_default_no_exception",
            Self::AdminPolicyDeviceCodeRequired => "admin_policy_device_code_required",
            Self::AdminPolicyManualResumeRequired => "admin_policy_manual_resume_required",
            Self::BrowserLaunchUnavailableUseDeviceCode => {
                "browser_launch_unavailable_use_device_code"
            }
            Self::BrowserLaunchOfflineUseStayLocal => "browser_launch_offline_use_stay_local",
            Self::AccountFreeLocalNoAuthRequired => "account_free_local_no_auth_required",
        }
    }

    /// True when the exception is the no-exception default; rows that pick a
    /// non-`open_system_browser` default MUST NOT quote this token.
    pub const fn is_default_no_exception(self) -> bool {
        matches!(self, Self::SystemBrowserDefaultNoException)
    }

    /// True when the exception explicitly applies to a non-system-browser
    /// default (admin policy, offline, or unavailable browser).
    pub const fn is_explicit_exception(self) -> bool {
        matches!(
            self,
            Self::AdminPolicyDeviceCodeRequired
                | Self::AdminPolicyManualResumeRequired
                | Self::BrowserLaunchUnavailableUseDeviceCode
                | Self::BrowserLaunchOfflineUseStayLocal
                | Self::AccountFreeLocalNoAuthRequired
        )
    }
}

/// Closed authority-scope vocabulary surfaced on the row to disclose the
/// requested vs granted scope of a claimed identity row's auth handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityScopeClass {
    /// Read-only access. Cannot mutate workspace / tenant state.
    ReadOnlyScope,
    /// Read + write access on the bound workspace.
    ReadWriteScope,
    /// Workspace-admin authority (manage members, policies).
    WorkspaceAdminScope,
    /// Tenant- or org-wide admin authority.
    TenantAdminScope,
    /// Step-up authority for a single risky action (passkey-style).
    StepUpScope,
    /// No granted scope — auth has not completed (or was denied).
    NoScopeGranted,
}

impl AuthorityScopeClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyScope => "read_only_scope",
            Self::ReadWriteScope => "read_write_scope",
            Self::WorkspaceAdminScope => "workspace_admin_scope",
            Self::TenantAdminScope => "tenant_admin_scope",
            Self::StepUpScope => "step_up_scope",
            Self::NoScopeGranted => "no_scope_granted",
        }
    }

    /// Comparable ordering for "is widening" detection. Higher numbers grant
    /// more authority than lower numbers.
    pub const fn rank(self) -> u8 {
        match self {
            Self::NoScopeGranted => 0,
            Self::ReadOnlyScope => 1,
            Self::StepUpScope => 2,
            Self::ReadWriteScope => 3,
            Self::WorkspaceAdminScope => 4,
            Self::TenantAdminScope => 5,
        }
    }

    /// True when `granted` would widen authority beyond `requested`.
    pub fn widens(requested: Self, granted: Self) -> bool {
        granted.rank() > requested.rank()
    }
}

/// Closed vocabulary naming the passkey / WebAuthn step-up posture for a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasskeyStepUpPostureClass {
    /// Row claims passkey capability and a passkey-bound step-up MUST be
    /// satisfied before auth proceeds.
    PasskeyRequired,
    /// Row claims passkey capability and a passkey-bound step-up is offered as
    /// the preferred path (with a typed fallback when the platform cannot
    /// satisfy it).
    PasskeyCapableOffered,
    /// Row claims passkey capability but the platform cannot satisfy it right
    /// now; the fallback retry-path is named explicitly.
    PasskeyUnavailableWithFallback,
    /// Row does not claim passkey capability; passkey step-up is not
    /// applicable.
    PasskeyNotApplicable,
}

impl PasskeyStepUpPostureClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PasskeyRequired => "passkey_required",
            Self::PasskeyCapableOffered => "passkey_capable_offered",
            Self::PasskeyUnavailableWithFallback => "passkey_unavailable_with_fallback",
            Self::PasskeyNotApplicable => "passkey_not_applicable",
        }
    }

    /// True when the row claims passkey capability and a step-up block MUST
    /// appear on the row.
    pub const fn requires_step_up_block(self) -> bool {
        matches!(
            self,
            Self::PasskeyRequired | Self::PasskeyCapableOffered | Self::PasskeyUnavailableWithFallback
        )
    }

    /// True when the row MUST quote a fallback retry-path token because the
    /// platform cannot satisfy a claimed passkey path right now.
    pub const fn requires_fallback_retry_path(self) -> bool {
        matches!(
            self,
            Self::PasskeyUnavailableWithFallback | Self::PasskeyCapableOffered
        )
    }
}

/// Closed semantic-axis vocabulary the audit verifies per row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemBrowserReturnPathBetaAxis {
    /// Row defaults to `open_system_browser` unless an explicit
    /// [`SystemBrowserPolicyExceptionClass`] is quoted.
    SystemBrowserDefaultUnlessExplicitException,
    /// Row quotes workspace / target / requested-action labels, return-mode
    /// token, return-anchor ref, return-origin-validation token, and
    /// tenant/workspace match-rule token.
    ExactReturnPathLabelPreserved,
    /// Granted post-return scope does not widen requested scope.
    WorkspaceTargetActionIdentityPreserved,
    /// Rows that claim passkey capability quote a closed
    /// [`PasskeyStepUpPostureClass`] token; rows that do not claim it quote
    /// `passkey_not_applicable`.
    PasskeyCapableStepUpWhenClaimed,
    /// Support row reuses the same closed-vocabulary tokens as the live row.
    SupportExportVocabularyParity,
}

impl SystemBrowserReturnPathBetaAxis {
    /// Stable schema token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemBrowserDefaultUnlessExplicitException => {
                "system_browser_default_unless_explicit_exception"
            }
            Self::ExactReturnPathLabelPreserved => "exact_return_path_label_preserved",
            Self::WorkspaceTargetActionIdentityPreserved => {
                "workspace_target_action_identity_preserved"
            }
            Self::PasskeyCapableStepUpWhenClaimed => "passkey_capable_step_up_when_claimed",
            Self::SupportExportVocabularyParity => "support_export_vocabulary_parity",
        }
    }
}

/// Closed defect vocabulary the audit emits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemBrowserReturnPathBetaDefectKind {
    /// Row picked a non-`open_system_browser` default while quoting
    /// `system_browser_default_no_exception`.
    SystemBrowserNotDefaultWithoutExplicitException,
    /// Row's `policy_exception_label` is empty.
    PolicyExceptionLabelMissing,
    /// Row's `return_path_workspace_label` is missing or drifted from the bound
    /// workspace ref.
    ReturnPathWorkspaceDrift,
    /// Row's `return_path_target_label` is missing.
    ReturnPathTargetDrift,
    /// Row's `return_path_requested_action_label` is missing.
    ReturnPathActionDrift,
    /// Row's `return_anchor_ref` is missing while the row claims a non-local
    /// return.
    ReturnAnchorRefMissing,
    /// Granted scope would widen the requested scope.
    ReturnWidensAuthorityScope,
    /// Row claimed passkey capability but did not quote a
    /// [`PasskeyStepUpPostureClass`] from the safe set.
    PasskeyClaimedWithoutStepUpBlock,
    /// Row quoted `passkey_unavailable_with_fallback` or
    /// `passkey_capable_offered` without naming a typed fallback retry-path.
    PasskeyUnavailableWithoutHonestFallback,
    /// Row that does not claim passkey capability quoted a posture other than
    /// `passkey_not_applicable`.
    PasskeyNotApplicableMislabeled,
    /// Support row's default-action, policy-exception, return-mode,
    /// requested-scope, granted-scope, passkey posture, or fallback retry-path
    /// drifted from the live row.
    SupportRowVocabularyDrift,
    /// Row picked the system-browser default but the auth policy quoted
    /// `browser_launch_policy_blocked` — these two states are inconsistent.
    SystemBrowserDefaultInconsistentWithBlockedLaunch,
}

impl SystemBrowserReturnPathBetaDefectKind {
    /// Stable schema token recorded on the defect.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemBrowserNotDefaultWithoutExplicitException => {
                "system_browser_not_default_without_explicit_exception"
            }
            Self::PolicyExceptionLabelMissing => "policy_exception_label_missing",
            Self::ReturnPathWorkspaceDrift => "return_path_workspace_drift",
            Self::ReturnPathTargetDrift => "return_path_target_drift",
            Self::ReturnPathActionDrift => "return_path_action_drift",
            Self::ReturnAnchorRefMissing => "return_anchor_ref_missing",
            Self::ReturnWidensAuthorityScope => "return_widens_authority_scope",
            Self::PasskeyClaimedWithoutStepUpBlock => "passkey_claimed_without_step_up_block",
            Self::PasskeyUnavailableWithoutHonestFallback => {
                "passkey_unavailable_without_honest_fallback"
            }
            Self::PasskeyNotApplicableMislabeled => "passkey_not_applicable_mislabeled",
            Self::SupportRowVocabularyDrift => "support_row_vocabulary_drift",
            Self::SystemBrowserDefaultInconsistentWithBlockedLaunch => {
                "system_browser_default_inconsistent_with_blocked_launch"
            }
        }
    }
}

/// Exact return-path labels carried on every audited row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReturnPathLabel {
    /// Workspace label rendered on the return interstitial.
    pub workspace_label: String,
    /// Target label (e.g. "Settings → Profile", "Editor → Open file").
    pub target_label: String,
    /// Plain-language label for the action the user requested before sign-in.
    pub requested_action_label: String,
    /// Closed return-mode token from [`ReturnModeClass`].
    pub return_mode_token: String,
    /// Stable opaque ref to the return anchor (loopback URL alias / deep-link
    /// scheme alias).
    pub return_anchor_ref: String,
    /// Origin-validation token from [`ReturnOriginValidationClass`].
    pub return_origin_validation_token: String,
    /// Tenant/workspace match-rule token from
    /// [`ReturnTenantOrWorkspaceMatchRule`].
    pub return_tenant_or_workspace_match_rule_token: String,
}

/// Passkey-capable step-up block. Optional on rows that quote
/// `passkey_not_applicable`; required on rows that claim passkey capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasskeyStepUpBlock {
    /// Closed posture token from [`PasskeyStepUpPostureClass`].
    pub posture_token: String,
    /// Plain-language reason rendered next to the posture.
    pub reason_label: String,
    /// Optional fallback retry-path token when the platform cannot satisfy a
    /// claimed passkey path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_retry_path_token: Option<String>,
    /// Optional plain-language fallback label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_retry_path_label: Option<String>,
}

/// Audited row for one claimed identity scenario in the beta projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemBrowserReturnPathBetaRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub row_id: String,
    pub source_claim_row_ref: String,

    pub account_boundary_class_token: String,
    pub identity_mode_token: String,
    pub trust_state_token: String,

    pub provider_label: String,
    pub provider_domain_label: String,
    pub provider_scope_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_workspace_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_tenant_or_org_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_actor_subject_ref: Option<String>,

    pub default_action_token: String,
    pub system_browser_default: bool,
    pub policy_exception_token: String,
    pub policy_exception_label: String,
    pub browser_launch_policy_class_token: String,

    pub return_path_label: ReturnPathLabel,
    pub requested_authority_scope_token: String,
    pub granted_authority_scope_token: String,
    pub authority_scope_summary_label: String,

    pub passkey_capability_claimed: bool,
    pub passkey_step_up: PasskeyStepUpBlock,

    pub promised_audit_axes: Vec<SystemBrowserReturnPathBetaAxis>,
    pub plain_language_summary: String,
    pub redaction_class_token: String,
}

/// Export-safe support row aligned 1:1 with [`SystemBrowserReturnPathBetaRow`]
/// by `row_id`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemBrowserReturnPathBetaSupportRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub row_id: String,
    pub account_boundary_class_token: String,
    pub identity_mode_token: String,
    pub trust_state_token: String,
    pub default_action_token: String,
    pub policy_exception_token: String,
    pub return_mode_token: String,
    pub return_origin_validation_token: String,
    pub return_tenant_or_workspace_match_rule_token: String,
    pub return_anchor_ref: String,
    pub workspace_label: String,
    pub target_label: String,
    pub requested_action_label: String,
    pub requested_authority_scope_token: String,
    pub granted_authority_scope_token: String,
    pub passkey_step_up_posture_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passkey_fallback_retry_path_token: Option<String>,
    pub redaction_class_token: String,
}

impl SystemBrowserReturnPathBetaSupportRow {
    /// Project an export-safe support row from a live audited row.
    pub fn from_row(row: &SystemBrowserReturnPathBetaRow) -> Self {
        Self {
            record_kind: SYSTEM_BROWSER_RETURN_PATHS_BETA_SUPPORT_ROW_RECORD_KIND.to_owned(),
            schema_version: SYSTEM_BROWSER_RETURN_PATHS_BETA_SCHEMA_VERSION,
            shared_contract_ref: SYSTEM_BROWSER_RETURN_PATHS_BETA_SHARED_CONTRACT_REF.to_owned(),
            case_id: row.case_id.clone(),
            row_id: row.row_id.clone(),
            account_boundary_class_token: row.account_boundary_class_token.clone(),
            identity_mode_token: row.identity_mode_token.clone(),
            trust_state_token: row.trust_state_token.clone(),
            default_action_token: row.default_action_token.clone(),
            policy_exception_token: row.policy_exception_token.clone(),
            return_mode_token: row.return_path_label.return_mode_token.clone(),
            return_origin_validation_token: row
                .return_path_label
                .return_origin_validation_token
                .clone(),
            return_tenant_or_workspace_match_rule_token: row
                .return_path_label
                .return_tenant_or_workspace_match_rule_token
                .clone(),
            return_anchor_ref: row.return_path_label.return_anchor_ref.clone(),
            workspace_label: row.return_path_label.workspace_label.clone(),
            target_label: row.return_path_label.target_label.clone(),
            requested_action_label: row.return_path_label.requested_action_label.clone(),
            requested_authority_scope_token: row.requested_authority_scope_token.clone(),
            granted_authority_scope_token: row.granted_authority_scope_token.clone(),
            passkey_step_up_posture_token: row.passkey_step_up.posture_token.clone(),
            passkey_fallback_retry_path_token: row
                .passkey_step_up
                .fallback_retry_path_token
                .clone(),
            redaction_class_token: row.redaction_class_token.clone(),
        }
    }
}

/// Typed defect emitted by the audit validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemBrowserReturnPathBetaDefect {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub defect_id: String,
    pub defect_kind: SystemBrowserReturnPathBetaDefectKind,
    pub defect_kind_token: String,
    pub row_id: String,
    pub field: String,
    pub note: String,
}

impl SystemBrowserReturnPathBetaDefect {
    fn new(
        defect_kind: SystemBrowserReturnPathBetaDefectKind,
        row_id: &str,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: SYSTEM_BROWSER_RETURN_PATHS_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: SYSTEM_BROWSER_RETURN_PATHS_BETA_SCHEMA_VERSION,
            shared_contract_ref: SYSTEM_BROWSER_RETURN_PATHS_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "auth:defect:system-browser-return-paths:{}:{}",
                defect_kind.as_str(),
                row_id
            ),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            row_id: row_id.to_owned(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate banner emitted with the page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SystemBrowserReturnPathsBetaSummary {
    pub row_count: usize,
    pub support_row_count: usize,
    pub defect_count: usize,
    pub system_browser_default_row_count: usize,
    pub explicit_exception_row_count: usize,
    pub passkey_capable_row_count: usize,
    pub return_modes_present: Vec<String>,
    pub policy_exceptions_present: Vec<String>,
    pub passkey_postures_present: Vec<String>,
}

impl SystemBrowserReturnPathsBetaSummary {
    fn from_rows(
        rows: &[SystemBrowserReturnPathBetaRow],
        support_rows: &[SystemBrowserReturnPathBetaSupportRow],
        defects: &[SystemBrowserReturnPathBetaDefect],
    ) -> Self {
        let mut return_modes: Vec<String> = Vec::new();
        let mut exceptions: Vec<String> = Vec::new();
        let mut postures: Vec<String> = Vec::new();
        let mut system_browser_default = 0usize;
        let mut explicit_exception = 0usize;
        let mut passkey_capable = 0usize;
        for row in rows {
            if !return_modes.contains(&row.return_path_label.return_mode_token) {
                return_modes.push(row.return_path_label.return_mode_token.clone());
            }
            if !exceptions.contains(&row.policy_exception_token) {
                exceptions.push(row.policy_exception_token.clone());
            }
            if !postures.contains(&row.passkey_step_up.posture_token) {
                postures.push(row.passkey_step_up.posture_token.clone());
            }
            if row.system_browser_default {
                system_browser_default += 1;
            }
            if row.policy_exception_token
                != SystemBrowserPolicyExceptionClass::SystemBrowserDefaultNoException.as_str()
            {
                explicit_exception += 1;
            }
            if row.passkey_capability_claimed {
                passkey_capable += 1;
            }
        }
        return_modes.sort();
        exceptions.sort();
        postures.sort();
        Self {
            row_count: rows.len(),
            support_row_count: support_rows.len(),
            defect_count: defects.len(),
            system_browser_default_row_count: system_browser_default,
            explicit_exception_row_count: explicit_exception,
            passkey_capable_row_count: passkey_capable,
            return_modes_present: return_modes,
            policy_exceptions_present: exceptions,
            passkey_postures_present: postures,
        }
    }
}

/// Top-level beta audit page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemBrowserReturnPathsBetaPage {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub page_id: String,
    pub page_label: String,
    pub generated_at: String,
    pub summary: SystemBrowserReturnPathsBetaSummary,
    pub rows: Vec<SystemBrowserReturnPathBetaRow>,
    pub support_rows: Vec<SystemBrowserReturnPathBetaSupportRow>,
    pub defects: Vec<SystemBrowserReturnPathBetaDefect>,
}

impl SystemBrowserReturnPathsBetaPage {
    /// Build a page from rows. Support rows are projected automatically and the
    /// defect list is computed by [`audit_rows`].
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        rows: Vec<SystemBrowserReturnPathBetaRow>,
    ) -> Self {
        let support_rows: Vec<SystemBrowserReturnPathBetaSupportRow> = rows
            .iter()
            .map(SystemBrowserReturnPathBetaSupportRow::from_row)
            .collect();
        let defects = audit_rows(&rows, &support_rows);
        let summary =
            SystemBrowserReturnPathsBetaSummary::from_rows(&rows, &support_rows, &defects);
        Self {
            record_kind: SYSTEM_BROWSER_RETURN_PATHS_BETA_PAGE_RECORD_KIND.to_owned(),
            schema_version: SYSTEM_BROWSER_RETURN_PATHS_BETA_SCHEMA_VERSION,
            shared_contract_ref: SYSTEM_BROWSER_RETURN_PATHS_BETA_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows,
            support_rows,
            defects,
        }
    }

    /// True when every claimed row defaults to `open_system_browser` or quotes
    /// an explicit policy exception that justifies a different default.
    pub fn defaults_to_system_browser_or_explicit_exception(&self) -> bool {
        self.rows.iter().all(|row| {
            row.system_browser_default
                || row.policy_exception_token
                    != SystemBrowserPolicyExceptionClass::SystemBrowserDefaultNoException.as_str()
        })
    }

    /// True when every row that claims passkey capability quotes a closed
    /// step-up posture.
    pub fn passkey_step_up_present_when_claimed(&self) -> bool {
        self.rows.iter().all(|row| {
            !row.passkey_capability_claimed
                || (row.passkey_step_up.posture_token
                    != PasskeyStepUpPostureClass::PasskeyNotApplicable.as_str())
        })
    }
}

/// Support-export wrapper that quotes the audited page plus a
/// metadata-safe defect roll-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemBrowserReturnPathsBetaSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub export_id: String,
    pub generated_at: String,
    pub page: SystemBrowserReturnPathsBetaPage,
    pub defect_kinds_present: Vec<SystemBrowserReturnPathBetaDefectKind>,
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    pub raw_private_material_excluded: bool,
}

impl SystemBrowserReturnPathsBetaSupportExport {
    /// Wrap a page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: SystemBrowserReturnPathsBetaPage,
    ) -> Self {
        let mut kinds: Vec<SystemBrowserReturnPathBetaDefectKind> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !kinds.contains(&defect.defect_kind) {
                kinds.push(defect.defect_kind);
            }
            *counts.entry(defect.defect_kind_token.clone()).or_insert(0) += 1;
        }
        kinds.sort();
        Self {
            record_kind: SYSTEM_BROWSER_RETURN_PATHS_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SYSTEM_BROWSER_RETURN_PATHS_BETA_SCHEMA_VERSION,
            shared_contract_ref: SYSTEM_BROWSER_RETURN_PATHS_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            defect_kinds_present: kinds,
            defect_counts_by_kind: counts,
            raw_private_material_excluded: true,
        }
    }
}

/// Validate a beta page; returns `Ok` on a clean audit and the typed defect
/// list otherwise.
pub fn validate_system_browser_return_paths_beta_page(
    page: &SystemBrowserReturnPathsBetaPage,
) -> Result<(), Vec<SystemBrowserReturnPathBetaDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

fn parse_authority_scope(token: &str) -> Option<AuthorityScopeClass> {
    match token {
        "no_scope_granted" => Some(AuthorityScopeClass::NoScopeGranted),
        "read_only_scope" => Some(AuthorityScopeClass::ReadOnlyScope),
        "step_up_scope" => Some(AuthorityScopeClass::StepUpScope),
        "read_write_scope" => Some(AuthorityScopeClass::ReadWriteScope),
        "workspace_admin_scope" => Some(AuthorityScopeClass::WorkspaceAdminScope),
        "tenant_admin_scope" => Some(AuthorityScopeClass::TenantAdminScope),
        _ => None,
    }
}

/// Re-run the audit over the row + support-row pair without rebuilding the
/// page. Tests and the headless inspector use this to surface a defect that
/// would otherwise hide behind a stale `page.defects` array.
pub fn audit_rows(
    rows: &[SystemBrowserReturnPathBetaRow],
    support_rows: &[SystemBrowserReturnPathBetaSupportRow],
) -> Vec<SystemBrowserReturnPathBetaDefect> {
    let mut defects: Vec<SystemBrowserReturnPathBetaDefect> = Vec::new();
    let support_by_id: BTreeMap<&str, &SystemBrowserReturnPathBetaSupportRow> = support_rows
        .iter()
        .map(|row| (row.row_id.as_str(), row))
        .collect();

    for row in rows {
        let no_exception_token =
            SystemBrowserPolicyExceptionClass::SystemBrowserDefaultNoException.as_str();
        let local_only_token = AccountBoundaryClass::LocalOnly.as_str();
        let local_no_auth_token =
            SystemBrowserPolicyExceptionClass::AccountFreeLocalNoAuthRequired.as_str();
        let blocked_launch_token = BrowserLaunchPolicyClass::BrowserLaunchPolicyBlocked.as_str();

        // Axis 1: system-browser default unless explicit exception.
        if !row.system_browser_default && row.policy_exception_token == no_exception_token {
            defects.push(SystemBrowserReturnPathBetaDefect::new(
                SystemBrowserReturnPathBetaDefectKind::SystemBrowserNotDefaultWithoutExplicitException,
                &row.row_id,
                "policy_exception_token",
                "row picked a non-`open_system_browser` default but did not name an explicit policy exception",
            ));
        }
        if row.policy_exception_token != no_exception_token && row.policy_exception_label.is_empty()
        {
            defects.push(SystemBrowserReturnPathBetaDefect::new(
                SystemBrowserReturnPathBetaDefectKind::PolicyExceptionLabelMissing,
                &row.row_id,
                "policy_exception_label",
                "explicit policy exception requires a plain-language label",
            ));
        }
        if row.system_browser_default && row.browser_launch_policy_class_token == blocked_launch_token
        {
            defects.push(SystemBrowserReturnPathBetaDefect::new(
                SystemBrowserReturnPathBetaDefectKind::SystemBrowserDefaultInconsistentWithBlockedLaunch,
                &row.row_id,
                "browser_launch_policy_class_token",
                "row claims system-browser default while declaring browser_launch_policy_blocked",
            ));
        }

        // Axis 2: exact return-path label preserved.
        let local_only = row.account_boundary_class_token == local_only_token
            || row.policy_exception_token == local_no_auth_token;
        if row.return_path_label.workspace_label.is_empty() && !local_only {
            defects.push(SystemBrowserReturnPathBetaDefect::new(
                SystemBrowserReturnPathBetaDefectKind::ReturnPathWorkspaceDrift,
                &row.row_id,
                "return_path_label.workspace_label",
                "non-local row must quote a workspace label on the return interstitial",
            ));
        }
        if row.return_path_label.target_label.is_empty() {
            defects.push(SystemBrowserReturnPathBetaDefect::new(
                SystemBrowserReturnPathBetaDefectKind::ReturnPathTargetDrift,
                &row.row_id,
                "return_path_label.target_label",
                "row must quote a target label",
            ));
        }
        if row.return_path_label.requested_action_label.is_empty() {
            defects.push(SystemBrowserReturnPathBetaDefect::new(
                SystemBrowserReturnPathBetaDefectKind::ReturnPathActionDrift,
                &row.row_id,
                "return_path_label.requested_action_label",
                "row must quote a requested-action label",
            ));
        }
        if row.return_path_label.return_anchor_ref.is_empty() && !local_only {
            defects.push(SystemBrowserReturnPathBetaDefect::new(
                SystemBrowserReturnPathBetaDefectKind::ReturnAnchorRefMissing,
                &row.row_id,
                "return_path_label.return_anchor_ref",
                "non-local row must quote a return-anchor ref",
            ));
        }

        // Axis 3: workspace/target/action identity preserved (no scope widening).
        if let (Some(req), Some(grant)) = (
            parse_authority_scope(&row.requested_authority_scope_token),
            parse_authority_scope(&row.granted_authority_scope_token),
        ) {
            if AuthorityScopeClass::widens(req, grant) {
                defects.push(SystemBrowserReturnPathBetaDefect::new(
                    SystemBrowserReturnPathBetaDefectKind::ReturnWidensAuthorityScope,
                    &row.row_id,
                    "granted_authority_scope_token",
                    "granted scope widens beyond the requested scope",
                ));
            }
        }

        // Axis 4: passkey-capable step-up when claimed.
        let posture = &row.passkey_step_up.posture_token;
        if row.passkey_capability_claimed {
            let valid = matches!(
                posture.as_str(),
                "passkey_required"
                    | "passkey_capable_offered"
                    | "passkey_unavailable_with_fallback"
            );
            if !valid {
                defects.push(SystemBrowserReturnPathBetaDefect::new(
                    SystemBrowserReturnPathBetaDefectKind::PasskeyClaimedWithoutStepUpBlock,
                    &row.row_id,
                    "passkey_step_up.posture_token",
                    "row claims passkey capability but did not quote a closed posture token",
                ));
            }
            let needs_fallback = matches!(
                posture.as_str(),
                "passkey_capable_offered" | "passkey_unavailable_with_fallback"
            );
            if needs_fallback && row.passkey_step_up.fallback_retry_path_token.is_none() {
                defects.push(SystemBrowserReturnPathBetaDefect::new(
                    SystemBrowserReturnPathBetaDefectKind::PasskeyUnavailableWithoutHonestFallback,
                    &row.row_id,
                    "passkey_step_up.fallback_retry_path_token",
                    "passkey-offered or unavailable posture must name a fallback retry-path token",
                ));
            }
        } else if posture != PasskeyStepUpPostureClass::PasskeyNotApplicable.as_str() {
            defects.push(SystemBrowserReturnPathBetaDefect::new(
                SystemBrowserReturnPathBetaDefectKind::PasskeyNotApplicableMislabeled,
                &row.row_id,
                "passkey_step_up.posture_token",
                "row does not claim passkey capability — posture must be `passkey_not_applicable`",
            ));
        }

        // Axis 5: support-row vocabulary parity.
        if let Some(support) = support_by_id.get(row.row_id.as_str()) {
            if support.default_action_token != row.default_action_token
                || support.policy_exception_token != row.policy_exception_token
                || support.return_mode_token != row.return_path_label.return_mode_token
                || support.return_origin_validation_token
                    != row.return_path_label.return_origin_validation_token
                || support.return_tenant_or_workspace_match_rule_token
                    != row.return_path_label.return_tenant_or_workspace_match_rule_token
                || support.return_anchor_ref != row.return_path_label.return_anchor_ref
                || support.workspace_label != row.return_path_label.workspace_label
                || support.target_label != row.return_path_label.target_label
                || support.requested_action_label != row.return_path_label.requested_action_label
                || support.requested_authority_scope_token != row.requested_authority_scope_token
                || support.granted_authority_scope_token != row.granted_authority_scope_token
                || support.passkey_step_up_posture_token != row.passkey_step_up.posture_token
                || support.passkey_fallback_retry_path_token
                    != row.passkey_step_up.fallback_retry_path_token
            {
                defects.push(SystemBrowserReturnPathBetaDefect::new(
                    SystemBrowserReturnPathBetaDefectKind::SupportRowVocabularyDrift,
                    &row.row_id,
                    "support_row",
                    "support row drifted from live row on a closed-vocabulary token",
                ));
            }
        } else {
            defects.push(SystemBrowserReturnPathBetaDefect::new(
                SystemBrowserReturnPathBetaDefectKind::SupportRowVocabularyDrift,
                &row.row_id,
                "support_row",
                "live row has no aligned support row",
            ));
        }
    }

    defects
}

/// Inputs used to seed a beta row from a claimed identity row plus return-path
/// labels and a passkey-step-up block.
#[derive(Debug, Clone)]
pub struct StageSystemBrowserReturnPathBetaRowRequest<'a> {
    pub case_id: &'a str,
    pub claimed_row: &'a ClaimedIdentityRow,
    pub source_claim_row_ref: &'a str,
    pub policy_exception: SystemBrowserPolicyExceptionClass,
    pub policy_exception_label: &'a str,
    pub return_workspace_label: &'a str,
    pub return_target_label: &'a str,
    pub return_requested_action_label: &'a str,
    pub return_mode_class: ReturnModeClass,
    pub return_anchor_ref: &'a str,
    pub return_origin_validation_class: ReturnOriginValidationClass,
    pub return_tenant_or_workspace_match_rule: ReturnTenantOrWorkspaceMatchRule,
    pub requested_authority_scope: AuthorityScopeClass,
    pub granted_authority_scope: AuthorityScopeClass,
    pub authority_scope_summary_label: &'a str,
    pub passkey_capability_claimed: bool,
    pub passkey_step_up_posture: PasskeyStepUpPostureClass,
    pub passkey_step_up_reason_label: &'a str,
    pub passkey_fallback_retry_path: Option<RetryPathClass>,
    pub passkey_fallback_retry_path_label: Option<&'a str>,
    pub plain_language_summary: &'a str,
}

impl<'a> StageSystemBrowserReturnPathBetaRowRequest<'a> {
    /// Mint a beta row with all closed-vocabulary tokens stamped from the
    /// alpha claimed-row truth and the passed return-path / passkey inputs.
    pub fn stage(self) -> SystemBrowserReturnPathBetaRow {
        let claimed = self.claimed_row;
        let system_browser_default = claimed.default_action
            == ClaimedIdentityDefaultActionClass::OpenSystemBrowser;
        let return_path_label = ReturnPathLabel {
            workspace_label: self.return_workspace_label.to_owned(),
            target_label: self.return_target_label.to_owned(),
            requested_action_label: self.return_requested_action_label.to_owned(),
            return_mode_token: self.return_mode_class.as_str().to_owned(),
            return_anchor_ref: self.return_anchor_ref.to_owned(),
            return_origin_validation_token: self.return_origin_validation_class.as_str().to_owned(),
            return_tenant_or_workspace_match_rule_token: self
                .return_tenant_or_workspace_match_rule
                .as_str()
                .to_owned(),
        };
        let passkey_step_up = PasskeyStepUpBlock {
            posture_token: self.passkey_step_up_posture.as_str().to_owned(),
            reason_label: self.passkey_step_up_reason_label.to_owned(),
            fallback_retry_path_token: self
                .passkey_fallback_retry_path
                .map(|class| class.as_str().to_owned()),
            fallback_retry_path_label: self
                .passkey_fallback_retry_path_label
                .map(str::to_owned),
        };
        let promised_axes = vec![
            SystemBrowserReturnPathBetaAxis::SystemBrowserDefaultUnlessExplicitException,
            SystemBrowserReturnPathBetaAxis::ExactReturnPathLabelPreserved,
            SystemBrowserReturnPathBetaAxis::WorkspaceTargetActionIdentityPreserved,
            SystemBrowserReturnPathBetaAxis::PasskeyCapableStepUpWhenClaimed,
            SystemBrowserReturnPathBetaAxis::SupportExportVocabularyParity,
        ];
        SystemBrowserReturnPathBetaRow {
            record_kind: SYSTEM_BROWSER_RETURN_PATHS_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: SYSTEM_BROWSER_RETURN_PATHS_BETA_SCHEMA_VERSION,
            shared_contract_ref: SYSTEM_BROWSER_RETURN_PATHS_BETA_SHARED_CONTRACT_REF.to_owned(),
            case_id: self.case_id.to_owned(),
            row_id: claimed.row_id.clone(),
            source_claim_row_ref: self.source_claim_row_ref.to_owned(),
            account_boundary_class_token: claimed.account_boundary_class_token.clone(),
            identity_mode_token: identity_mode_token(claimed.identity_mode).to_owned(),
            trust_state_token: trust_state_token(claimed.trust_state).to_owned(),
            provider_label: claimed.provider_scope.provider_label.clone(),
            provider_domain_label: claimed.provider_scope.provider_domain_label.clone(),
            provider_scope_label: claimed.provider_scope.provider_scope_label.clone(),
            bound_workspace_ref: claimed.provider_scope.bound_workspace_ref.clone(),
            bound_tenant_or_org_ref: claimed.provider_scope.bound_tenant_or_org_ref.clone(),
            bound_actor_subject_ref: claimed.provider_scope.bound_actor_subject_ref.clone(),
            default_action_token: claimed.default_action_token.clone(),
            system_browser_default,
            policy_exception_token: self.policy_exception.as_str().to_owned(),
            policy_exception_label: self.policy_exception_label.to_owned(),
            browser_launch_policy_class_token: claimed
                .auth_policy
                .browser_launch_policy_class
                .as_str()
                .to_owned(),
            return_path_label,
            requested_authority_scope_token: self
                .requested_authority_scope
                .as_str()
                .to_owned(),
            granted_authority_scope_token: self.granted_authority_scope.as_str().to_owned(),
            authority_scope_summary_label: self.authority_scope_summary_label.to_owned(),
            passkey_capability_claimed: self.passkey_capability_claimed,
            passkey_step_up,
            promised_audit_axes: promised_axes,
            plain_language_summary: self.plain_language_summary.to_owned(),
            redaction_class_token: "metadata_only_export_safe".to_owned(),
        }
    }
}

fn identity_mode_token(mode: IdentityModeAlias) -> &'static str {
    match mode {
        IdentityModeAlias::AccountFreeLocal => "account_free_local",
        IdentityModeAlias::SelfHostedOrg => "self_hosted_org",
        IdentityModeAlias::ManagedConvenience => "managed_convenience",
    }
}

fn trust_state_token(state: TrustState) -> &'static str {
    state.as_str()
}

fn passkey_capable_managed_request() -> StageClaimedIdentityRowRequest<'static> {
    StageClaimedIdentityRowRequest {
        row_id: "claimed-identity:managed:payments-prod",
        state_class: ClaimedIdentityStateClass::AwaitingSystemBrowser,
        account_boundary_class: AccountBoundaryClass::Managed,
        identity_mode: IdentityModeAlias::ManagedConvenience,
        trust_state: TrustState::Trusted,
        provider_label: "Acme identity",
        provider_domain_label: "login.acme.example",
        provider_scope_label: "payments-prod tenant",
        bound_workspace_ref: Some("workspace:payments-prod"),
        bound_tenant_or_org_ref: Some("tenant:acme-prod"),
        bound_actor_subject_ref: Some("actor-subject:sam.acme"),
        browser_launch_policy_class: BrowserLaunchPolicyClass::SystemDefaultBrowserRequired,
        system_browser_supported: true,
        device_code_supported: true,
        stay_local_supported: true,
        issued_at: Some("2026-05-13T08:00:00Z"),
        expires_at: Some("2026-05-13T08:10:00Z"),
        expiry_summary_label: "System-browser handoff expires in 10 minutes.",
        device_code_expires_at: Some("2026-05-13T08:15:00Z"),
        device_code_ref: Some("device-code:managed:payments-prod"),
        local_continuity_label: "Local files and unsaved edits remain available.",
        preserved_local_work: PreservedLocalWork {
            posture_class: PreservedLocalWorkPostureClass::LocalWorkIntactWithManagedNarrowed,
            note: "Local work remains available while managed auth is incomplete.".to_owned(),
            retained_capabilities: vec![
                "Edit local files.".to_owned(),
                "Save local files.".to_owned(),
                "Use local Git.".to_owned(),
            ],
            blocked_capabilities: vec!["Managed settings sync waits for sign-in.".to_owned()],
        },
        auth_callback_packet_ref: Some("auth-callback:managed:payments-prod"),
        browser_handoff_packet_ref: Some("browser-handoff:managed:payments-prod"),
        native_boundary_handoff_ref: Some("native-handoff:auth-callback:payments-prod"),
        embedded_boundary_card_ref: None,
        managed_session_state_ref: Some("managed-session:payments-prod"),
        recovery_copy_label: "Continue sign-in in your browser or stay local.",
        primary_recovery_action: None,
        support_export_ref: Some("support-export:auth:payments-prod"),
        execution_context_ref: Some("execution-context:auth:payments-prod"),
        minted_at: "2026-05-13T08:00:01Z",
    }
}

fn admin_locked_device_code_request() -> StageClaimedIdentityRowRequest<'static> {
    StageClaimedIdentityRowRequest {
        row_id: "claimed-identity:managed:vdi-locked",
        state_class: ClaimedIdentityStateClass::AwaitingDeviceCode,
        account_boundary_class: AccountBoundaryClass::Managed,
        identity_mode: IdentityModeAlias::ManagedConvenience,
        trust_state: TrustState::Trusted,
        provider_label: "Acme identity",
        provider_domain_label: "login.acme.example",
        provider_scope_label: "locked-vdi tenant",
        bound_workspace_ref: Some("workspace:locked-vdi"),
        bound_tenant_or_org_ref: Some("tenant:acme-prod"),
        bound_actor_subject_ref: Some("actor-subject:sam.acme"),
        browser_launch_policy_class: BrowserLaunchPolicyClass::BrowserLaunchPolicyBlocked,
        system_browser_supported: false,
        device_code_supported: true,
        stay_local_supported: true,
        issued_at: Some("2026-05-13T08:30:00Z"),
        expires_at: Some("2026-05-13T08:40:00Z"),
        expiry_summary_label: "Device-code handoff expires in 10 minutes.",
        device_code_expires_at: Some("2026-05-13T08:40:00Z"),
        device_code_ref: Some("device-code:managed:locked-vdi"),
        local_continuity_label: "Local files and unsaved edits remain available.",
        preserved_local_work: PreservedLocalWork {
            posture_class: PreservedLocalWorkPostureClass::LocalWorkIntactWithManagedNarrowed,
            note: "Browser launch is admin-blocked on this VDI; local work remains usable.".to_owned(),
            retained_capabilities: vec![
                "Edit local files.".to_owned(),
                "Save local files.".to_owned(),
            ],
            blocked_capabilities: vec!["Managed sign-in waits for the device-code flow.".to_owned()],
        },
        auth_callback_packet_ref: Some("auth-callback:managed:locked-vdi"),
        browser_handoff_packet_ref: None,
        native_boundary_handoff_ref: Some("native-handoff:auth-callback:locked-vdi"),
        embedded_boundary_card_ref: None,
        managed_session_state_ref: Some("managed-session:locked-vdi"),
        recovery_copy_label: "Use device-code sign-in or stay local.",
        primary_recovery_action: None,
        support_export_ref: Some("support-export:auth:locked-vdi"),
        execution_context_ref: Some("execution-context:auth:locked-vdi"),
        minted_at: "2026-05-13T08:30:01Z",
    }
}

fn account_free_local_request() -> StageClaimedIdentityRowRequest<'static> {
    StageClaimedIdentityRowRequest {
        row_id: "claimed-identity:local:account-free",
        state_class: ClaimedIdentityStateClass::AccountFreeLocal,
        account_boundary_class: AccountBoundaryClass::LocalOnly,
        identity_mode: IdentityModeAlias::AccountFreeLocal,
        trust_state: TrustState::Trusted,
        provider_label: "",
        provider_domain_label: "",
        provider_scope_label: "",
        bound_workspace_ref: None,
        bound_tenant_or_org_ref: None,
        bound_actor_subject_ref: None,
        browser_launch_policy_class: BrowserLaunchPolicyClass::SystemDefaultBrowserRequired,
        system_browser_supported: false,
        device_code_supported: false,
        stay_local_supported: true,
        issued_at: None,
        expires_at: None,
        expiry_summary_label: "",
        device_code_expires_at: None,
        device_code_ref: None,
        local_continuity_label: "Account-free local mode keeps editing, save, undo, search, local Git, local tasks, and BYOK AI usable.",
        preserved_local_work: PreservedLocalWork {
            posture_class: PreservedLocalWorkPostureClass::LocalWorkIntact,
            note: "Account-free local mode preserves all local work.".to_owned(),
            retained_capabilities: vec![
                "Edit local files.".to_owned(),
                "Save local files.".to_owned(),
                "Local Git.".to_owned(),
                "Local search.".to_owned(),
                "Local tasks.".to_owned(),
                "BYOK AI.".to_owned(),
            ],
            blocked_capabilities: vec![],
        },
        auth_callback_packet_ref: None,
        browser_handoff_packet_ref: None,
        native_boundary_handoff_ref: None,
        embedded_boundary_card_ref: None,
        managed_session_state_ref: None,
        recovery_copy_label: "No sign-in required for account-free local mode.",
        primary_recovery_action: None,
        support_export_ref: None,
        execution_context_ref: Some("execution-context:auth:account-free-local"),
        minted_at: "2026-05-13T08:00:00Z",
    }
}

/// Build the seeded beta page that the live shell, the headless inspector, and
/// the integration test all consume.
pub fn seeded_system_browser_return_paths_beta_page() -> SystemBrowserReturnPathsBetaPage {
    let managed_claim =
        ClaimedIdentityRow::stage(passkey_capable_managed_request()).expect("managed claim stages");
    let admin_locked = ClaimedIdentityRow::stage(admin_locked_device_code_request())
        .expect("admin-locked claim stages");
    let account_free =
        ClaimedIdentityRow::stage(account_free_local_request()).expect("local claim stages");

    let managed_row = StageSystemBrowserReturnPathBetaRowRequest {
        case_id: "system_browser_default_with_passkey_step_up",
        claimed_row: &managed_claim,
        source_claim_row_ref:
            "claim:system-browser-alpha:claimed-identity:managed:payments-prod",
        policy_exception: SystemBrowserPolicyExceptionClass::SystemBrowserDefaultNoException,
        policy_exception_label: "",
        return_workspace_label: "Workspace · payments-prod",
        return_target_label: "Settings → Provider sync",
        return_requested_action_label:
            "Resume Provider sync (read+write) after sign-in.",
        return_mode_class: ReturnModeClass::LoopbackHttpReturn,
        return_anchor_ref: "return-anchor:loopback:payments-prod:provider-sync",
        return_origin_validation_class: ReturnOriginValidationClass::LoopbackPortPinned,
        return_tenant_or_workspace_match_rule:
            ReturnTenantOrWorkspaceMatchRule::MustMatchBoundWorkspaceAndTenant,
        requested_authority_scope: AuthorityScopeClass::ReadWriteScope,
        granted_authority_scope: AuthorityScopeClass::ReadWriteScope,
        authority_scope_summary_label:
            "Requested read+write on payments-prod; granted scope MUST equal requested.",
        passkey_capability_claimed: true,
        passkey_step_up_posture: PasskeyStepUpPostureClass::PasskeyCapableOffered,
        passkey_step_up_reason_label:
            "Passkey step-up offered before write actions on payments-prod.",
        passkey_fallback_retry_path: Some(RetryPathClass::ResumeAfterStepUp),
        passkey_fallback_retry_path_label:
            Some("If passkey is unavailable, resume after a typed step-up via system browser."),
        plain_language_summary:
            "Managed claim defaults to system-browser auth and offers passkey step-up before write actions; the return preserves payments-prod workspace, the Settings → Provider sync target, and the read+write scope request.",
    }
    .stage();

    let admin_locked_row = StageSystemBrowserReturnPathBetaRowRequest {
        case_id: "admin_policy_device_code_required",
        claimed_row: &admin_locked,
        source_claim_row_ref:
            "claim:system-browser-alpha:claimed-identity:managed:vdi-locked",
        policy_exception: SystemBrowserPolicyExceptionClass::AdminPolicyDeviceCodeRequired,
        policy_exception_label:
            "Admin policy on this VDI blocks browser launch; the row uses device-code auth instead.",
        return_workspace_label: "Workspace · locked-vdi",
        return_target_label: "Activity center → Resume managed sign-in",
        return_requested_action_label:
            "Resume managed sign-in after device-code completes.",
        return_mode_class: ReturnModeClass::DeviceCodePollReturn,
        return_anchor_ref: "return-anchor:device-code:locked-vdi",
        return_origin_validation_class: ReturnOriginValidationClass::DeviceCodePollOnly,
        return_tenant_or_workspace_match_rule:
            ReturnTenantOrWorkspaceMatchRule::MustMatchBoundWorkspaceAndTenant,
        requested_authority_scope: AuthorityScopeClass::ReadOnlyScope,
        granted_authority_scope: AuthorityScopeClass::ReadOnlyScope,
        authority_scope_summary_label:
            "Requested read-only on locked-vdi; granted scope MUST equal requested.",
        passkey_capability_claimed: false,
        passkey_step_up_posture: PasskeyStepUpPostureClass::PasskeyNotApplicable,
        passkey_step_up_reason_label:
            "Locked VDI: no passkey authenticator available; explicit device-code policy.",
        passkey_fallback_retry_path: None,
        passkey_fallback_retry_path_label: None,
        plain_language_summary:
            "Admin policy on this VDI blocks browser launch; the row explicitly quotes admin_policy_device_code_required and falls back to device-code with a labeled return anchor.",
    }
    .stage();

    let account_free_row = StageSystemBrowserReturnPathBetaRowRequest {
        case_id: "account_free_local_no_auth_required",
        claimed_row: &account_free,
        source_claim_row_ref:
            "claim:system-browser-alpha:claimed-identity:local:account-free",
        policy_exception: SystemBrowserPolicyExceptionClass::AccountFreeLocalNoAuthRequired,
        policy_exception_label:
            "Account-free local mode does not require system-browser auth.",
        return_workspace_label: "",
        return_target_label: "Welcome → Continue without an account",
        return_requested_action_label: "Continue without signing in.",
        return_mode_class: ReturnModeClass::NotApplicable,
        return_anchor_ref: "",
        return_origin_validation_class: ReturnOriginValidationClass::ManualResumeOnly,
        return_tenant_or_workspace_match_rule:
            ReturnTenantOrWorkspaceMatchRule::NoTenantOrWorkspaceBinding,
        requested_authority_scope: AuthorityScopeClass::NoScopeGranted,
        granted_authority_scope: AuthorityScopeClass::NoScopeGranted,
        authority_scope_summary_label:
            "Account-free local mode grants no remote scope.",
        passkey_capability_claimed: false,
        passkey_step_up_posture: PasskeyStepUpPostureClass::PasskeyNotApplicable,
        passkey_step_up_reason_label: "No remote auth required.",
        passkey_fallback_retry_path: None,
        passkey_fallback_retry_path_label: None,
        plain_language_summary:
            "Account-free local mode requires no system-browser auth and grants no remote scope; the row explicitly quotes account_free_local_no_auth_required.",
    }
    .stage();

    SystemBrowserReturnPathsBetaPage::new(
        "auth:system_browser_return_paths_beta:default",
        "System-browser default + passkey step-up + return-path labeling (beta)",
        "2026-05-15T00:00:00Z",
        vec![managed_row, admin_locked_row, account_free_row],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page() -> SystemBrowserReturnPathsBetaPage {
        seeded_system_browser_return_paths_beta_page()
    }

    #[test]
    fn seeded_page_seeds_zero_defects_and_audits_clean() {
        let page = page();
        assert_eq!(page.summary.defect_count, 0);
        assert_eq!(page.defects.len(), 0);
        assert!(validate_system_browser_return_paths_beta_page(&page).is_ok());
        assert!(page.defaults_to_system_browser_or_explicit_exception());
        assert!(page.passkey_step_up_present_when_claimed());
    }

    #[test]
    fn seeded_page_includes_passkey_capable_row_with_offered_posture() {
        let page = page();
        let row = page
            .rows
            .iter()
            .find(|r| r.passkey_capability_claimed)
            .expect("at least one passkey-capable row");
        assert_eq!(
            row.passkey_step_up.posture_token,
            PasskeyStepUpPostureClass::PasskeyCapableOffered.as_str()
        );
        assert!(row.passkey_step_up.fallback_retry_path_token.is_some());
    }

    #[test]
    fn admin_policy_exception_row_is_explicit_and_uses_device_code() {
        let page = page();
        let row = page
            .rows
            .iter()
            .find(|r| {
                r.policy_exception_token
                    == SystemBrowserPolicyExceptionClass::AdminPolicyDeviceCodeRequired.as_str()
            })
            .expect("admin-policy exception row present");
        assert!(!row.system_browser_default);
        assert!(!row.policy_exception_label.is_empty());
        assert_eq!(
            row.return_path_label.return_mode_token,
            ReturnModeClass::DeviceCodePollReturn.as_str()
        );
        assert_eq!(
            row.passkey_step_up.posture_token,
            PasskeyStepUpPostureClass::PasskeyNotApplicable.as_str()
        );
    }

    #[test]
    fn defect_drill_widening_authority_scope_is_caught() {
        let mut page = page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.passkey_capability_claimed)
            .unwrap();
        row.granted_authority_scope_token = AuthorityScopeClass::TenantAdminScope.as_str().to_owned();
        let support_rows: Vec<SystemBrowserReturnPathBetaSupportRow> = page
            .rows
            .iter()
            .map(SystemBrowserReturnPathBetaSupportRow::from_row)
            .collect();
        let defects = audit_rows(&page.rows, &support_rows);
        assert!(defects.iter().any(|d| d.defect_kind
            == SystemBrowserReturnPathBetaDefectKind::ReturnWidensAuthorityScope));
    }

    #[test]
    fn defect_drill_passkey_unavailable_without_fallback_is_caught() {
        let mut page = page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.passkey_capability_claimed)
            .unwrap();
        row.passkey_step_up.posture_token =
            PasskeyStepUpPostureClass::PasskeyUnavailableWithFallback
                .as_str()
                .to_owned();
        row.passkey_step_up.fallback_retry_path_token = None;
        let support_rows: Vec<SystemBrowserReturnPathBetaSupportRow> = page
            .rows
            .iter()
            .map(SystemBrowserReturnPathBetaSupportRow::from_row)
            .collect();
        let defects = audit_rows(&page.rows, &support_rows);
        assert!(defects.iter().any(|d| d.defect_kind
            == SystemBrowserReturnPathBetaDefectKind::PasskeyUnavailableWithoutHonestFallback));
    }

    #[test]
    fn defect_drill_system_browser_not_default_without_exception_is_caught() {
        let mut page = page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.passkey_capability_claimed)
            .unwrap();
        row.system_browser_default = false;
        row.policy_exception_token =
            SystemBrowserPolicyExceptionClass::SystemBrowserDefaultNoException
                .as_str()
                .to_owned();
        row.default_action_token = "use_device_code".to_owned();
        let support_rows: Vec<SystemBrowserReturnPathBetaSupportRow> = page
            .rows
            .iter()
            .map(SystemBrowserReturnPathBetaSupportRow::from_row)
            .collect();
        let defects = audit_rows(&page.rows, &support_rows);
        assert!(defects.iter().any(|d| d.defect_kind
            == SystemBrowserReturnPathBetaDefectKind::SystemBrowserNotDefaultWithoutExplicitException));
    }

    #[test]
    fn defect_drill_support_row_drift_is_caught() {
        let page = page();
        let mut support_rows = page.support_rows.clone();
        support_rows[0].granted_authority_scope_token =
            AuthorityScopeClass::WorkspaceAdminScope.as_str().to_owned();
        let defects = audit_rows(&page.rows, &support_rows);
        assert!(defects.iter().any(|d| d.defect_kind
            == SystemBrowserReturnPathBetaDefectKind::SupportRowVocabularyDrift));
    }

    #[test]
    fn defect_drill_missing_return_target_label_is_caught() {
        let mut page = page();
        page.rows[0].return_path_label.target_label.clear();
        let support_rows: Vec<SystemBrowserReturnPathBetaSupportRow> = page
            .rows
            .iter()
            .map(SystemBrowserReturnPathBetaSupportRow::from_row)
            .collect();
        let defects = audit_rows(&page.rows, &support_rows);
        assert!(defects.iter().any(|d| d.defect_kind
            == SystemBrowserReturnPathBetaDefectKind::ReturnPathTargetDrift));
    }

    #[test]
    fn defect_drill_passkey_not_applicable_mislabeled_is_caught() {
        let mut page = page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| !r.passkey_capability_claimed)
            .unwrap();
        row.passkey_step_up.posture_token =
            PasskeyStepUpPostureClass::PasskeyRequired.as_str().to_owned();
        let support_rows: Vec<SystemBrowserReturnPathBetaSupportRow> = page
            .rows
            .iter()
            .map(SystemBrowserReturnPathBetaSupportRow::from_row)
            .collect();
        let defects = audit_rows(&page.rows, &support_rows);
        assert!(defects.iter().any(|d| d.defect_kind
            == SystemBrowserReturnPathBetaDefectKind::PasskeyNotApplicableMislabeled));
    }

    #[test]
    fn support_export_round_trips_with_zero_defects() {
        let page = page();
        let export = SystemBrowserReturnPathsBetaSupportExport::from_page(
            "support-export:system-browser-return-paths:001",
            "2026-05-15T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
        assert!(export.defect_counts_by_kind.is_empty());
    }
}
