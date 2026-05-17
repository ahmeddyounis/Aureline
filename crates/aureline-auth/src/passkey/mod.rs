//! Passkey-capable step-up, reauth, and recovery lane audit for claimed beta
//! rows.
//!
//! The `system_browser/beta.rs` projection already names a passkey-step-up
//! posture token on every claimed identity row. This module promotes that
//! single posture token into a page-level audit that proves, on every row that
//! claims a passkey lane (step-up, reauth, or recovery), that:
//!
//! 1. **Lane identity is named.** Every row quotes a closed
//!    [`PasskeyBetaLaneClass`] token — `step_up_lane`, `reauth_lane`,
//!    `recovery_lane`, or `not_applicable_account_free_local`. The lane,
//!    plus the originating target / action labels, is what the row preserves
//!    end-to-end.
//! 2. **Lifecycle state and client scope are disclosed.** Every claimed lane
//!    row names a closed [`PasskeyLifecycleStateClass`] token (e.g.
//!    `active_on_this_device`, `unavailable_this_platform`, `revoked`) plus a
//!    closed [`PasskeyClientScopeClass`] token tied to the action being
//!    authorized.
//! 3. **Step-up is satisfied, or a typed fallback is named.** Rows whose
//!    [`PasskeyOutcomeClass`] is not `step_up_satisfied` MUST name a closed
//!    [`PasskeyFallbackClass`] token. Silently dropping a row without a
//!    fallback path on an unsupported platform is a typed defect, and so is
//!    declaring a passkey lane while admin policy denies it without naming a
//!    `policy_denied_*` fallback.
//! 4. **Reauth and recovery preserve the original target/action identity.**
//!    Rows that claim a reauth or recovery lane MUST quote a
//!    [`PasskeyTargetActionPreservationClass`] token from the safe set
//!    (`target_action_preserved_exact`, `target_action_downscoped`). A
//!    `target_action_rerouted` or `target_action_widened` token on a reauth
//!    or recovery lane is a typed defect — the lane MUST NOT silently widen
//!    or reroute the request.
//! 5. **No authority widening on return.** The granted authority scope on the
//!    row MUST NOT widen beyond the requested authority scope. Widening is a
//!    typed defect even when the lane completed.
//! 6. **Support-export vocabulary parity.** The support row reuses the same
//!    closed-vocabulary tokens the live row paints (lane, lifecycle state,
//!    client scope, outcome, fallback, preservation, profile, requested vs
//!    granted scope). Drift is a contract bug.
//!
//! The seeded page seeds zero defects. The validator and the headless
//! inspector are what surface a regression when a row drops a required field,
//! widens authority on return, reroutes the target/action across reauth or
//! recovery, or drifts vocabulary across the live row and the support row.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Beta schema version exported with every record.
pub const PASSKEY_STEP_UP_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every record on the page.
pub const PASSKEY_STEP_UP_BETA_SHARED_CONTRACT_REF: &str = "auth:passkey_step_up_beta:v1";

/// Record kind for [`PasskeyStepUpBetaPage`] payloads.
pub const PASSKEY_STEP_UP_BETA_PAGE_RECORD_KIND: &str = "auth_passkey_step_up_beta_page_record";

/// Record kind for [`PasskeyStepUpBetaRow`] payloads.
pub const PASSKEY_STEP_UP_BETA_ROW_RECORD_KIND: &str = "auth_passkey_step_up_beta_row_record";

/// Record kind for [`PasskeyStepUpBetaSupportRow`] payloads.
pub const PASSKEY_STEP_UP_BETA_SUPPORT_ROW_RECORD_KIND: &str =
    "auth_passkey_step_up_beta_support_row_record";

/// Record kind for [`PasskeyStepUpBetaDefect`] payloads.
pub const PASSKEY_STEP_UP_BETA_DEFECT_RECORD_KIND: &str = "auth_passkey_step_up_beta_defect_record";

/// Record kind for [`PasskeyStepUpBetaSupportExport`] payloads.
pub const PASSKEY_STEP_UP_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "auth_passkey_step_up_beta_support_export_record";

/// Profile under which a row is inspected. Mirrors the OIDC / policy-pack beta
/// profile vocabulary so admin and support surfaces use one token set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasskeyBetaProfileClass {
    /// Connected beta profile with live identity issuer.
    Connected,
    /// Mirror-only profile served from a declared signed mirror.
    MirrorOnly,
    /// Offline profile served from a last-known-good or air-gapped snapshot.
    Offline,
    /// Enterprise-managed profile applying signed managed policy narrowing.
    EnterpriseManaged,
}

impl PasskeyBetaProfileClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::MirrorOnly => "mirror_only",
            Self::Offline => "offline",
            Self::EnterpriseManaged => "enterprise_managed",
        }
    }
}

/// Closed vocabulary naming which lane a row participates in. Every row picks
/// exactly one lane token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasskeyBetaLaneClass {
    /// Passkey step-up before a risky write or admin action on an already
    /// signed-in row.
    StepUpLane,
    /// Passkey-capable reauthentication after a session expiry or sensitive
    /// posture change.
    ReauthLane,
    /// Passkey-bound recovery after a lost device, revoked authenticator, or
    /// admin-initiated unbind.
    RecoveryLane,
    /// Account-free local row; no passkey lane is claimed.
    NotApplicableAccountFreeLocal,
}

impl PasskeyBetaLaneClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StepUpLane => "step_up_lane",
            Self::ReauthLane => "reauth_lane",
            Self::RecoveryLane => "recovery_lane",
            Self::NotApplicableAccountFreeLocal => "not_applicable_account_free_local",
        }
    }

    /// True when the lane is the account-free local sentinel.
    pub const fn is_account_free_local(self) -> bool {
        matches!(self, Self::NotApplicableAccountFreeLocal)
    }

    /// True when the lane MUST preserve the original target / action identity
    /// instead of silently widening or rerouting.
    pub const fn requires_target_action_preservation(self) -> bool {
        matches!(self, Self::ReauthLane | Self::RecoveryLane)
    }
}

/// Closed vocabulary naming the passkey lifecycle state visible on the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasskeyLifecycleStateClass {
    /// User has not enrolled a passkey on this account.
    NotEnrolled,
    /// User has started enrollment but has not completed it.
    EnrollmentPending,
    /// Passkey active and bound to the current device's authenticator.
    ActiveOnThisDevice,
    /// Passkey active on another device only; cross-device flow required.
    ActiveOnOtherDeviceOnly,
    /// Passkey revoked by admin or user; row MUST use a fallback path.
    Revoked,
    /// Passkey expired and a reattestation is required before it can be used.
    ExpiredAttestationRequired,
    /// Current platform / browser / runtime does not support WebAuthn /
    /// passkey at all on this row.
    UnavailableThisPlatform,
    /// Account-free local row; lifecycle state is not applicable.
    NotApplicableAccountFreeLocal,
}

impl PasskeyLifecycleStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotEnrolled => "not_enrolled",
            Self::EnrollmentPending => "enrollment_pending",
            Self::ActiveOnThisDevice => "active_on_this_device",
            Self::ActiveOnOtherDeviceOnly => "active_on_other_device_only",
            Self::Revoked => "revoked",
            Self::ExpiredAttestationRequired => "expired_attestation_required",
            Self::UnavailableThisPlatform => "unavailable_this_platform",
            Self::NotApplicableAccountFreeLocal => "not_applicable_account_free_local",
        }
    }

    /// True when the lifecycle state means a passkey assertion cannot be
    /// satisfied right now and a fallback MUST be named.
    pub const fn requires_fallback(self) -> bool {
        matches!(
            self,
            Self::NotEnrolled
                | Self::EnrollmentPending
                | Self::ActiveOnOtherDeviceOnly
                | Self::Revoked
                | Self::ExpiredAttestationRequired
                | Self::UnavailableThisPlatform
        )
    }
}

/// Closed vocabulary naming the client scope of the passkey assertion, tied
/// to the action being authorized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasskeyClientScopeClass {
    /// Step-up authority for one risky write action (e.g. publish, push).
    StepUpScopeRiskyWriteAction,
    /// Step-up authority for one risky admin action (e.g. rotate a key).
    StepUpScopeAdminAction,
    /// Reauth scope: refresh the existing session without widening.
    ReauthScopeRefreshSession,
    /// Recovery scope: rebind an authenticator on an existing account.
    RecoveryScopeRebindAuthenticator,
    /// No scope granted (lane pending or denied).
    NoScopeGranted,
    /// Account-free local row; no client scope.
    NotApplicableAccountFreeLocal,
}

impl PasskeyClientScopeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StepUpScopeRiskyWriteAction => "step_up_scope_risky_write_action",
            Self::StepUpScopeAdminAction => "step_up_scope_admin_action",
            Self::ReauthScopeRefreshSession => "reauth_scope_refresh_session",
            Self::RecoveryScopeRebindAuthenticator => "recovery_scope_rebind_authenticator",
            Self::NoScopeGranted => "no_scope_granted",
            Self::NotApplicableAccountFreeLocal => "not_applicable_account_free_local",
        }
    }
}

/// Closed authority-scope vocabulary the row uses to compare requested vs
/// granted authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasskeyAuthorityScopeClass {
    /// No scope granted (lane pending, denied, or account-free local).
    NoScopeGranted,
    /// Read-only access.
    ReadOnlyScope,
    /// Step-up authority for one risky action.
    StepUpScope,
    /// Read+write access on the bound workspace / tenant.
    ReadWriteScope,
    /// Workspace-admin authority.
    WorkspaceAdminScope,
    /// Tenant- or org-wide admin authority.
    TenantAdminScope,
}

impl PasskeyAuthorityScopeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoScopeGranted => "no_scope_granted",
            Self::ReadOnlyScope => "read_only_scope",
            Self::StepUpScope => "step_up_scope",
            Self::ReadWriteScope => "read_write_scope",
            Self::WorkspaceAdminScope => "workspace_admin_scope",
            Self::TenantAdminScope => "tenant_admin_scope",
        }
    }

    /// Comparable rank used by the no-widening check.
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

/// Closed vocabulary naming the visible outcome of the lane at the moment the
/// row is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasskeyOutcomeClass {
    /// Passkey assertion satisfied the lane.
    StepUpSatisfied,
    /// Lane waiting on user action (the user has not touched the
    /// authenticator yet).
    StepUpPendingUserAction,
    /// Admin policy denied the lane outright.
    StepUpDeniedByPolicy,
    /// Authenticator missing on this device / platform; lane cannot proceed
    /// without a fallback.
    StepUpDeniedAuthenticatorMissing,
    /// User canceled the prompt.
    StepUpUserCanceled,
    /// Lane fell back to a typed fallback path.
    FallbackEngaged,
    /// Account-free local row; outcome is not applicable.
    NotApplicableAccountFreeLocal,
}

impl PasskeyOutcomeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StepUpSatisfied => "step_up_satisfied",
            Self::StepUpPendingUserAction => "step_up_pending_user_action",
            Self::StepUpDeniedByPolicy => "step_up_denied_by_policy",
            Self::StepUpDeniedAuthenticatorMissing => "step_up_denied_authenticator_missing",
            Self::StepUpUserCanceled => "step_up_user_canceled",
            Self::FallbackEngaged => "fallback_engaged",
            Self::NotApplicableAccountFreeLocal => "not_applicable_account_free_local",
        }
    }

    /// True when the outcome leaves the lane unsatisfied and a fallback MUST
    /// be named on the row.
    pub const fn requires_fallback(self) -> bool {
        matches!(
            self,
            Self::StepUpPendingUserAction
                | Self::StepUpDeniedByPolicy
                | Self::StepUpDeniedAuthenticatorMissing
                | Self::StepUpUserCanceled
                | Self::FallbackEngaged
        )
    }
}

/// Closed vocabulary naming the typed fallback path when a passkey lane
/// cannot be satisfied right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasskeyFallbackClass {
    /// No fallback required — the lane was satisfied by the passkey assertion.
    NoFallbackRequired,
    /// Resume via a typed step-up flow in the system browser.
    ResumeAfterStepUpInSystemBrowser,
    /// One-time email-verified link tied to the bound account.
    EmailVerifiedLinkFallback,
    /// Pre-issued backup code tied to the bound account.
    BackupCodeFallback,
    /// Admin-approved device-code fallback (e.g. headless / VDI).
    AdminApprovedDeviceCodeFallback,
    /// Hardware-token (FIDO2 roaming authenticator) fallback.
    HardwareTokenFallback,
    /// Contact the admin for an out-of-band recovery flow.
    ContactAdminForRecovery,
    /// Continue working locally without sign-in (no managed authority).
    ContinueLocalWithoutSignIn,
    /// Admin policy denies passkey entirely on this row; the row MUST quote
    /// this token instead of silently falling back to a public endpoint.
    PolicyDeniedNoFallbackAvailable,
    /// Account-free local row; fallback is not applicable.
    NotApplicableAccountFreeLocal,
}

impl PasskeyFallbackClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoFallbackRequired => "no_fallback_required",
            Self::ResumeAfterStepUpInSystemBrowser => "resume_after_step_up_in_system_browser",
            Self::EmailVerifiedLinkFallback => "email_verified_link_fallback",
            Self::BackupCodeFallback => "backup_code_fallback",
            Self::AdminApprovedDeviceCodeFallback => "admin_approved_device_code_fallback",
            Self::HardwareTokenFallback => "hardware_token_fallback",
            Self::ContactAdminForRecovery => "contact_admin_for_recovery",
            Self::ContinueLocalWithoutSignIn => "continue_local_without_sign_in",
            Self::PolicyDeniedNoFallbackAvailable => "policy_denied_no_fallback_available",
            Self::NotApplicableAccountFreeLocal => "not_applicable_account_free_local",
        }
    }

    /// True when the fallback token names a real fallback path (i.e. not the
    /// sentinel values `no_fallback_required` or
    /// `not_applicable_account_free_local`).
    pub const fn names_real_fallback(self) -> bool {
        !matches!(
            self,
            Self::NoFallbackRequired | Self::NotApplicableAccountFreeLocal
        )
    }
}

/// Closed vocabulary naming whether the lane preserved the originating
/// target / action identity across reauth or recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasskeyTargetActionPreservationClass {
    /// The originating target and action identity were preserved exactly.
    TargetActionPreservedExact,
    /// The lane preserved the identity but downscoped the action (e.g.
    /// recovery granted read-only when read+write was requested).
    TargetActionDownscoped,
    /// The lane rerouted to a different target / action than requested.
    /// Reauth and recovery lanes MUST NOT quote this token.
    TargetActionRerouted,
    /// The lane widened the target / action beyond what was requested.
    /// Reauth and recovery lanes MUST NOT quote this token.
    TargetActionWidened,
    /// Account-free local row; target / action preservation is not
    /// applicable.
    NotApplicableAccountFreeLocal,
}

impl PasskeyTargetActionPreservationClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetActionPreservedExact => "target_action_preserved_exact",
            Self::TargetActionDownscoped => "target_action_downscoped",
            Self::TargetActionRerouted => "target_action_rerouted",
            Self::TargetActionWidened => "target_action_widened",
            Self::NotApplicableAccountFreeLocal => "not_applicable_account_free_local",
        }
    }

    /// True when the token is in the safe set (preserves identity or only
    /// downscopes it).
    pub const fn is_safe(self) -> bool {
        matches!(
            self,
            Self::TargetActionPreservedExact
                | Self::TargetActionDownscoped
                | Self::NotApplicableAccountFreeLocal
        )
    }
}

/// Closed semantic-axis vocabulary the audit verifies per row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasskeyStepUpBetaAxis {
    /// Row names its lane and binds it to a target / action label.
    LaneIdentifiedAndScopedToTargetAction,
    /// Row names the passkey lifecycle state and the client scope of the
    /// assertion.
    LifecycleStateAndClientScopeDisclosed,
    /// Row's outcome is `step_up_satisfied` or names a typed fallback path.
    StepUpSatisfiedOrFallbackNamed,
    /// Reauth and recovery rows preserve the original target / action
    /// identity instead of silently widening or rerouting.
    TargetActionIdentityPreservedAcrossReauthAndRecovery,
    /// Granted authority scope does not widen the requested scope.
    NoAuthorityWideningOnReturn,
    /// Support row reuses the same closed-vocabulary tokens as the live row.
    SupportExportVocabularyParity,
}

impl PasskeyStepUpBetaAxis {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaneIdentifiedAndScopedToTargetAction => {
                "lane_identified_and_scoped_to_target_action"
            }
            Self::LifecycleStateAndClientScopeDisclosed => {
                "lifecycle_state_and_client_scope_disclosed"
            }
            Self::StepUpSatisfiedOrFallbackNamed => "step_up_satisfied_or_fallback_named",
            Self::TargetActionIdentityPreservedAcrossReauthAndRecovery => {
                "target_action_identity_preserved_across_reauth_and_recovery"
            }
            Self::NoAuthorityWideningOnReturn => "no_authority_widening_on_return",
            Self::SupportExportVocabularyParity => "support_export_vocabulary_parity",
        }
    }
}

/// Closed defect vocabulary the audit emits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasskeyStepUpBetaDefectKind {
    /// Row claimed a lane but did not provide a target label.
    LaneTargetLabelMissing,
    /// Row claimed a lane but did not provide a requested-action label.
    LaneRequestedActionLabelMissing,
    /// Row's lifecycle state token is missing on a non-local lane.
    LifecycleStateMissing,
    /// Row's client scope token is missing on a non-local lane.
    ClientScopeMissing,
    /// Row outcome leaves the lane unsatisfied but no fallback was named.
    OutcomeUnsatisfiedWithoutFallback,
    /// Row's lifecycle state requires a fallback but the row picked
    /// `no_fallback_required`.
    LifecycleStateRequiresFallback,
    /// Admin policy denies passkey but the row did not quote the
    /// `policy_denied_no_fallback_available` token or another typed fallback.
    PolicyDeniesPasskeyWithoutFallback,
    /// Reauth or recovery row quoted `target_action_rerouted`.
    ReauthOrRecoveryRerouted,
    /// Reauth or recovery row quoted `target_action_widened`.
    ReauthOrRecoveryWidened,
    /// Granted authority scope widened beyond the requested scope.
    GrantedAuthorityWidensRequested,
    /// Row claims account-free local but quoted a non-local lane token.
    AccountFreeLocalLaneMislabeled,
    /// Support row drifted from the live row on a closed-vocabulary token.
    SupportRowVocabularyDrift,
}

impl PasskeyStepUpBetaDefectKind {
    /// Stable token recorded on the defect.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaneTargetLabelMissing => "lane_target_label_missing",
            Self::LaneRequestedActionLabelMissing => "lane_requested_action_label_missing",
            Self::LifecycleStateMissing => "lifecycle_state_missing",
            Self::ClientScopeMissing => "client_scope_missing",
            Self::OutcomeUnsatisfiedWithoutFallback => "outcome_unsatisfied_without_fallback",
            Self::LifecycleStateRequiresFallback => "lifecycle_state_requires_fallback",
            Self::PolicyDeniesPasskeyWithoutFallback => "policy_denies_passkey_without_fallback",
            Self::ReauthOrRecoveryRerouted => "reauth_or_recovery_rerouted",
            Self::ReauthOrRecoveryWidened => "reauth_or_recovery_widened",
            Self::GrantedAuthorityWidensRequested => "granted_authority_widens_requested",
            Self::AccountFreeLocalLaneMislabeled => "account_free_local_lane_mislabeled",
            Self::SupportRowVocabularyDrift => "support_row_vocabulary_drift",
        }
    }
}

/// Lane disclosure block. Account-free local rows quote the local sentinel
/// tokens and leave the target / action labels empty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasskeyLaneBlock {
    /// Closed lane token from [`PasskeyBetaLaneClass`].
    pub lane_token: String,
    /// Plain-language target label (e.g. "Settings → Rotate signing key").
    pub target_label: String,
    /// Plain-language label for the action the user requested before the lane
    /// began.
    pub requested_action_label: String,
    /// Stable opaque ref to the originating target row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originating_target_ref: Option<String>,
}

/// Lifecycle disclosure block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasskeyLifecycleBlock {
    /// Closed lifecycle token from [`PasskeyLifecycleStateClass`].
    pub state_token: String,
    /// Plain-language label rendered next to the state.
    pub state_label: String,
    /// Closed client-scope token from [`PasskeyClientScopeClass`].
    pub client_scope_token: String,
    /// Plain-language label for the client scope.
    pub client_scope_label: String,
}

/// Outcome / fallback disclosure block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasskeyOutcomeBlock {
    /// Closed outcome token from [`PasskeyOutcomeClass`].
    pub outcome_token: String,
    /// Plain-language outcome label.
    pub outcome_label: String,
    /// Closed fallback token from [`PasskeyFallbackClass`].
    pub fallback_token: String,
    /// Plain-language fallback label rendered to the user.
    pub fallback_label: String,
}

/// Target / action preservation disclosure block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasskeyTargetActionPreservationBlock {
    /// Closed preservation token from
    /// [`PasskeyTargetActionPreservationClass`].
    pub preservation_token: String,
    /// Plain-language label rendered next to the preservation posture.
    pub preservation_label: String,
}

/// Audited row for one passkey lane scenario in the beta projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasskeyStepUpBetaRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub row_id: String,
    pub source_claim_row_ref: String,
    pub profile_token: String,

    pub lane: PasskeyLaneBlock,
    pub lifecycle: PasskeyLifecycleBlock,
    pub outcome: PasskeyOutcomeBlock,
    pub target_action_preservation: PasskeyTargetActionPreservationBlock,

    pub admin_policy_denies_passkey: bool,
    pub requested_authority_scope_token: String,
    pub granted_authority_scope_token: String,
    pub authority_scope_summary_label: String,

    pub promised_audit_axes: Vec<PasskeyStepUpBetaAxis>,
    pub plain_language_summary: String,
    pub redaction_class_token: String,
}

/// Export-safe support row aligned 1:1 with [`PasskeyStepUpBetaRow`] by
/// `row_id`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasskeyStepUpBetaSupportRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub row_id: String,
    pub profile_token: String,
    pub lane_token: String,
    pub target_label: String,
    pub requested_action_label: String,
    pub lifecycle_state_token: String,
    pub client_scope_token: String,
    pub outcome_token: String,
    pub fallback_token: String,
    pub preservation_token: String,
    pub admin_policy_denies_passkey: bool,
    pub requested_authority_scope_token: String,
    pub granted_authority_scope_token: String,
    pub redaction_class_token: String,
}

impl PasskeyStepUpBetaSupportRow {
    /// Project an export-safe support row from a live audited row.
    pub fn from_row(row: &PasskeyStepUpBetaRow) -> Self {
        Self {
            record_kind: PASSKEY_STEP_UP_BETA_SUPPORT_ROW_RECORD_KIND.to_owned(),
            schema_version: PASSKEY_STEP_UP_BETA_SCHEMA_VERSION,
            shared_contract_ref: PASSKEY_STEP_UP_BETA_SHARED_CONTRACT_REF.to_owned(),
            case_id: row.case_id.clone(),
            row_id: row.row_id.clone(),
            profile_token: row.profile_token.clone(),
            lane_token: row.lane.lane_token.clone(),
            target_label: row.lane.target_label.clone(),
            requested_action_label: row.lane.requested_action_label.clone(),
            lifecycle_state_token: row.lifecycle.state_token.clone(),
            client_scope_token: row.lifecycle.client_scope_token.clone(),
            outcome_token: row.outcome.outcome_token.clone(),
            fallback_token: row.outcome.fallback_token.clone(),
            preservation_token: row.target_action_preservation.preservation_token.clone(),
            admin_policy_denies_passkey: row.admin_policy_denies_passkey,
            requested_authority_scope_token: row.requested_authority_scope_token.clone(),
            granted_authority_scope_token: row.granted_authority_scope_token.clone(),
            redaction_class_token: row.redaction_class_token.clone(),
        }
    }
}

/// Typed defect emitted by the audit validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasskeyStepUpBetaDefect {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub defect_id: String,
    pub defect_kind: PasskeyStepUpBetaDefectKind,
    pub defect_kind_token: String,
    pub row_id: String,
    pub field: String,
    pub note: String,
}

impl PasskeyStepUpBetaDefect {
    fn new(
        defect_kind: PasskeyStepUpBetaDefectKind,
        row_id: &str,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: PASSKEY_STEP_UP_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: PASSKEY_STEP_UP_BETA_SCHEMA_VERSION,
            shared_contract_ref: PASSKEY_STEP_UP_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "auth:defect:passkey-step-up:{}:{}",
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
pub struct PasskeyStepUpBetaSummary {
    pub row_count: usize,
    pub support_row_count: usize,
    pub defect_count: usize,
    pub step_up_row_count: usize,
    pub reauth_row_count: usize,
    pub recovery_row_count: usize,
    pub fallback_row_count: usize,
    pub profiles_present: Vec<String>,
    pub lanes_present: Vec<String>,
    pub lifecycle_states_present: Vec<String>,
    pub outcomes_present: Vec<String>,
    pub fallbacks_present: Vec<String>,
}

impl PasskeyStepUpBetaSummary {
    fn from_rows(
        rows: &[PasskeyStepUpBetaRow],
        support_rows: &[PasskeyStepUpBetaSupportRow],
        defects: &[PasskeyStepUpBetaDefect],
    ) -> Self {
        let mut profiles: Vec<String> = Vec::new();
        let mut lanes: Vec<String> = Vec::new();
        let mut states: Vec<String> = Vec::new();
        let mut outcomes: Vec<String> = Vec::new();
        let mut fallbacks: Vec<String> = Vec::new();
        let mut step_up = 0usize;
        let mut reauth = 0usize;
        let mut recovery = 0usize;
        let mut fallback_count = 0usize;
        let no_fallback = PasskeyFallbackClass::NoFallbackRequired.as_str();
        let local_fallback = PasskeyFallbackClass::NotApplicableAccountFreeLocal.as_str();
        for row in rows {
            if !profiles.contains(&row.profile_token) {
                profiles.push(row.profile_token.clone());
            }
            if !lanes.contains(&row.lane.lane_token) {
                lanes.push(row.lane.lane_token.clone());
            }
            if !states.contains(&row.lifecycle.state_token) {
                states.push(row.lifecycle.state_token.clone());
            }
            if !outcomes.contains(&row.outcome.outcome_token) {
                outcomes.push(row.outcome.outcome_token.clone());
            }
            if !fallbacks.contains(&row.outcome.fallback_token) {
                fallbacks.push(row.outcome.fallback_token.clone());
            }
            match row.lane.lane_token.as_str() {
                "step_up_lane" => step_up += 1,
                "reauth_lane" => reauth += 1,
                "recovery_lane" => recovery += 1,
                _ => {}
            }
            if row.outcome.fallback_token != no_fallback
                && row.outcome.fallback_token != local_fallback
            {
                fallback_count += 1;
            }
        }
        profiles.sort();
        lanes.sort();
        states.sort();
        outcomes.sort();
        fallbacks.sort();
        Self {
            row_count: rows.len(),
            support_row_count: support_rows.len(),
            defect_count: defects.len(),
            step_up_row_count: step_up,
            reauth_row_count: reauth,
            recovery_row_count: recovery,
            fallback_row_count: fallback_count,
            profiles_present: profiles,
            lanes_present: lanes,
            lifecycle_states_present: states,
            outcomes_present: outcomes,
            fallbacks_present: fallbacks,
        }
    }
}

/// Top-level beta audit page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasskeyStepUpBetaPage {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub page_id: String,
    pub page_label: String,
    pub generated_at: String,
    pub summary: PasskeyStepUpBetaSummary,
    pub rows: Vec<PasskeyStepUpBetaRow>,
    pub support_rows: Vec<PasskeyStepUpBetaSupportRow>,
    pub defects: Vec<PasskeyStepUpBetaDefect>,
}

impl PasskeyStepUpBetaPage {
    /// Build a page from rows. Support rows are projected automatically and
    /// the defect list is computed by [`audit_passkey_step_up_beta_rows`].
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        rows: Vec<PasskeyStepUpBetaRow>,
    ) -> Self {
        let support_rows: Vec<PasskeyStepUpBetaSupportRow> = rows
            .iter()
            .map(PasskeyStepUpBetaSupportRow::from_row)
            .collect();
        let defects = audit_passkey_step_up_beta_rows(&rows, &support_rows);
        let summary = PasskeyStepUpBetaSummary::from_rows(&rows, &support_rows, &defects);
        Self {
            record_kind: PASSKEY_STEP_UP_BETA_PAGE_RECORD_KIND.to_owned(),
            schema_version: PASSKEY_STEP_UP_BETA_SCHEMA_VERSION,
            shared_contract_ref: PASSKEY_STEP_UP_BETA_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows,
            support_rows,
            defects,
        }
    }

    /// True when every reauth / recovery row preserves the original target /
    /// action identity instead of silently widening or rerouting.
    pub fn reauth_and_recovery_preserve_target_action_identity(&self) -> bool {
        self.rows.iter().all(|row| {
            if row.lane.lane_token == PasskeyBetaLaneClass::ReauthLane.as_str()
                || row.lane.lane_token == PasskeyBetaLaneClass::RecoveryLane.as_str()
            {
                matches!(
                    row.target_action_preservation.preservation_token.as_str(),
                    "target_action_preserved_exact" | "target_action_downscoped"
                )
            } else {
                true
            }
        })
    }

    /// True when every row whose lifecycle / outcome makes the lane
    /// unsatisfied names a real fallback path.
    pub fn fallback_named_when_passkey_unavailable(&self) -> bool {
        let no_fallback = PasskeyFallbackClass::NoFallbackRequired.as_str();
        self.rows.iter().all(|row| {
            let satisfied = row.outcome.outcome_token
                == PasskeyOutcomeClass::StepUpSatisfied.as_str()
                || row.outcome.outcome_token
                    == PasskeyOutcomeClass::NotApplicableAccountFreeLocal.as_str();
            if satisfied {
                true
            } else {
                row.outcome.fallback_token != no_fallback
            }
        })
    }
}

/// Support-export wrapper that quotes the audited page plus a metadata-safe
/// defect roll-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasskeyStepUpBetaSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub export_id: String,
    pub generated_at: String,
    pub page: PasskeyStepUpBetaPage,
    pub defect_kinds_present: Vec<PasskeyStepUpBetaDefectKind>,
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    pub raw_private_material_excluded: bool,
}

impl PasskeyStepUpBetaSupportExport {
    /// Wrap a page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: PasskeyStepUpBetaPage,
    ) -> Self {
        let mut kinds: Vec<PasskeyStepUpBetaDefectKind> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !kinds.contains(&defect.defect_kind) {
                kinds.push(defect.defect_kind);
            }
            *counts.entry(defect.defect_kind_token.clone()).or_insert(0) += 1;
        }
        kinds.sort();
        Self {
            record_kind: PASSKEY_STEP_UP_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PASSKEY_STEP_UP_BETA_SCHEMA_VERSION,
            shared_contract_ref: PASSKEY_STEP_UP_BETA_SHARED_CONTRACT_REF.to_owned(),
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
pub fn validate_passkey_step_up_beta_page(
    page: &PasskeyStepUpBetaPage,
) -> Result<(), Vec<PasskeyStepUpBetaDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

fn parse_authority_scope(token: &str) -> Option<PasskeyAuthorityScopeClass> {
    match token {
        "no_scope_granted" => Some(PasskeyAuthorityScopeClass::NoScopeGranted),
        "read_only_scope" => Some(PasskeyAuthorityScopeClass::ReadOnlyScope),
        "step_up_scope" => Some(PasskeyAuthorityScopeClass::StepUpScope),
        "read_write_scope" => Some(PasskeyAuthorityScopeClass::ReadWriteScope),
        "workspace_admin_scope" => Some(PasskeyAuthorityScopeClass::WorkspaceAdminScope),
        "tenant_admin_scope" => Some(PasskeyAuthorityScopeClass::TenantAdminScope),
        _ => None,
    }
}

/// Re-run the audit over the row + support-row pair without rebuilding the
/// page. Tests and the headless inspector use this to surface a defect that
/// would otherwise hide behind a stale `page.defects` array.
pub fn audit_passkey_step_up_beta_rows(
    rows: &[PasskeyStepUpBetaRow],
    support_rows: &[PasskeyStepUpBetaSupportRow],
) -> Vec<PasskeyStepUpBetaDefect> {
    let mut defects: Vec<PasskeyStepUpBetaDefect> = Vec::new();
    let support_by_id: BTreeMap<&str, &PasskeyStepUpBetaSupportRow> = support_rows
        .iter()
        .map(|row| (row.row_id.as_str(), row))
        .collect();

    let local_lane = PasskeyBetaLaneClass::NotApplicableAccountFreeLocal.as_str();
    let local_lifecycle = PasskeyLifecycleStateClass::NotApplicableAccountFreeLocal.as_str();
    let local_scope = PasskeyClientScopeClass::NotApplicableAccountFreeLocal.as_str();
    let no_fallback = PasskeyFallbackClass::NoFallbackRequired.as_str();
    let local_fallback = PasskeyFallbackClass::NotApplicableAccountFreeLocal.as_str();
    let policy_denied = PasskeyFallbackClass::PolicyDeniedNoFallbackAvailable.as_str();
    let satisfied = PasskeyOutcomeClass::StepUpSatisfied.as_str();
    let local_outcome = PasskeyOutcomeClass::NotApplicableAccountFreeLocal.as_str();
    let preserved = PasskeyTargetActionPreservationClass::TargetActionPreservedExact.as_str();
    let downscoped = PasskeyTargetActionPreservationClass::TargetActionDownscoped.as_str();
    let rerouted = PasskeyTargetActionPreservationClass::TargetActionRerouted.as_str();
    let widened = PasskeyTargetActionPreservationClass::TargetActionWidened.as_str();

    for row in rows {
        let is_local = row.lane.lane_token == local_lane;

        // Axis 1: lane identified and scoped to a target / action.
        if !is_local {
            if row.lane.target_label.trim().is_empty() {
                defects.push(PasskeyStepUpBetaDefect::new(
                    PasskeyStepUpBetaDefectKind::LaneTargetLabelMissing,
                    &row.row_id,
                    "lane.target_label",
                    "claimed lane row must quote a target label",
                ));
            }
            if row.lane.requested_action_label.trim().is_empty() {
                defects.push(PasskeyStepUpBetaDefect::new(
                    PasskeyStepUpBetaDefectKind::LaneRequestedActionLabelMissing,
                    &row.row_id,
                    "lane.requested_action_label",
                    "claimed lane row must quote a requested-action label",
                ));
            }
        }

        // Axis 1 (cont.): account-free local must use the local sentinels.
        if is_local {
            let local_lifecycle_ok = row.lifecycle.state_token == local_lifecycle
                && row.lifecycle.client_scope_token == local_scope
                && row.outcome.outcome_token == local_outcome
                && row.outcome.fallback_token == local_fallback;
            if !local_lifecycle_ok {
                defects.push(PasskeyStepUpBetaDefect::new(
                    PasskeyStepUpBetaDefectKind::AccountFreeLocalLaneMislabeled,
                    &row.row_id,
                    "lane.lane_token",
                    "account-free local row must quote the local sentinel tokens across lifecycle, scope, outcome, and fallback",
                ));
            }
        }

        // Axis 2: lifecycle state and client scope disclosed for non-local rows.
        if !is_local {
            if row.lifecycle.state_token.trim().is_empty()
                || row.lifecycle.state_token == local_lifecycle
            {
                defects.push(PasskeyStepUpBetaDefect::new(
                    PasskeyStepUpBetaDefectKind::LifecycleStateMissing,
                    &row.row_id,
                    "lifecycle.state_token",
                    "claimed lane row must quote a non-local lifecycle state token",
                ));
            }
            if row.lifecycle.client_scope_token.trim().is_empty()
                || row.lifecycle.client_scope_token == local_scope
            {
                defects.push(PasskeyStepUpBetaDefect::new(
                    PasskeyStepUpBetaDefectKind::ClientScopeMissing,
                    &row.row_id,
                    "lifecycle.client_scope_token",
                    "claimed lane row must quote a non-local client-scope token",
                ));
            }
        }

        // Axis 3: step-up satisfied, or a typed fallback is named.
        let outcome_token = row.outcome.outcome_token.as_str();
        let fallback_token = row.outcome.fallback_token.as_str();
        if !is_local {
            let outcome_is_satisfied = outcome_token == satisfied || outcome_token == local_outcome;
            if !outcome_is_satisfied && fallback_token == no_fallback {
                defects.push(PasskeyStepUpBetaDefect::new(
                    PasskeyStepUpBetaDefectKind::OutcomeUnsatisfiedWithoutFallback,
                    &row.row_id,
                    "outcome.fallback_token",
                    "unsatisfied lane must name a typed fallback path",
                ));
            }
            let lifecycle_requires_fallback = matches!(
                row.lifecycle.state_token.as_str(),
                "not_enrolled"
                    | "enrollment_pending"
                    | "active_on_other_device_only"
                    | "revoked"
                    | "expired_attestation_required"
                    | "unavailable_this_platform"
            );
            if lifecycle_requires_fallback && fallback_token == no_fallback {
                defects.push(PasskeyStepUpBetaDefect::new(
                    PasskeyStepUpBetaDefectKind::LifecycleStateRequiresFallback,
                    &row.row_id,
                    "outcome.fallback_token",
                    "lifecycle state requires a typed fallback path",
                ));
            }
            if row.admin_policy_denies_passkey
                && fallback_token != policy_denied
                && fallback_token == no_fallback
            {
                defects.push(PasskeyStepUpBetaDefect::new(
                    PasskeyStepUpBetaDefectKind::PolicyDeniesPasskeyWithoutFallback,
                    &row.row_id,
                    "outcome.fallback_token",
                    "admin policy denies passkey but no fallback path was named",
                ));
            }
        }

        // Axis 4: reauth / recovery preserve the target / action identity.
        let preservation_token = row.target_action_preservation.preservation_token.as_str();
        let lane_requires_preservation = row.lane.lane_token
            == PasskeyBetaLaneClass::ReauthLane.as_str()
            || row.lane.lane_token == PasskeyBetaLaneClass::RecoveryLane.as_str();
        if lane_requires_preservation {
            if preservation_token == rerouted {
                defects.push(PasskeyStepUpBetaDefect::new(
                    PasskeyStepUpBetaDefectKind::ReauthOrRecoveryRerouted,
                    &row.row_id,
                    "target_action_preservation.preservation_token",
                    "reauth / recovery lane rerouted the target / action identity",
                ));
            }
            if preservation_token == widened {
                defects.push(PasskeyStepUpBetaDefect::new(
                    PasskeyStepUpBetaDefectKind::ReauthOrRecoveryWidened,
                    &row.row_id,
                    "target_action_preservation.preservation_token",
                    "reauth / recovery lane widened the target / action identity",
                ));
            }
            if preservation_token != preserved
                && preservation_token != downscoped
                && preservation_token != rerouted
                && preservation_token != widened
            {
                defects.push(PasskeyStepUpBetaDefect::new(
                    PasskeyStepUpBetaDefectKind::ReauthOrRecoveryRerouted,
                    &row.row_id,
                    "target_action_preservation.preservation_token",
                    "reauth / recovery lane must quote a preservation token from the safe set",
                ));
            }
        }

        // Axis 5: no authority widening on return.
        if let (Some(req), Some(grant)) = (
            parse_authority_scope(&row.requested_authority_scope_token),
            parse_authority_scope(&row.granted_authority_scope_token),
        ) {
            if PasskeyAuthorityScopeClass::widens(req, grant) {
                defects.push(PasskeyStepUpBetaDefect::new(
                    PasskeyStepUpBetaDefectKind::GrantedAuthorityWidensRequested,
                    &row.row_id,
                    "granted_authority_scope_token",
                    "granted scope widens beyond the requested scope",
                ));
            }
        }

        // Axis 6: support-row vocabulary parity.
        if let Some(support) = support_by_id.get(row.row_id.as_str()) {
            if support.profile_token != row.profile_token
                || support.lane_token != row.lane.lane_token
                || support.target_label != row.lane.target_label
                || support.requested_action_label != row.lane.requested_action_label
                || support.lifecycle_state_token != row.lifecycle.state_token
                || support.client_scope_token != row.lifecycle.client_scope_token
                || support.outcome_token != row.outcome.outcome_token
                || support.fallback_token != row.outcome.fallback_token
                || support.preservation_token != row.target_action_preservation.preservation_token
                || support.admin_policy_denies_passkey != row.admin_policy_denies_passkey
                || support.requested_authority_scope_token != row.requested_authority_scope_token
                || support.granted_authority_scope_token != row.granted_authority_scope_token
            {
                defects.push(PasskeyStepUpBetaDefect::new(
                    PasskeyStepUpBetaDefectKind::SupportRowVocabularyDrift,
                    &row.row_id,
                    "support_row",
                    "support row drifted from live row on a closed-vocabulary token",
                ));
            }
        } else {
            defects.push(PasskeyStepUpBetaDefect::new(
                PasskeyStepUpBetaDefectKind::SupportRowVocabularyDrift,
                &row.row_id,
                "support_row",
                "live row has no aligned support row",
            ));
        }
    }

    defects
}

/// Inputs used to seed a beta row.
#[derive(Debug, Clone)]
pub struct StagePasskeyStepUpBetaRowRequest<'a> {
    pub case_id: &'a str,
    pub row_id: &'a str,
    pub source_claim_row_ref: &'a str,
    pub profile: PasskeyBetaProfileClass,

    pub lane: PasskeyBetaLaneClass,
    pub target_label: &'a str,
    pub requested_action_label: &'a str,
    pub originating_target_ref: Option<&'a str>,

    pub lifecycle_state: PasskeyLifecycleStateClass,
    pub lifecycle_state_label: &'a str,
    pub client_scope: PasskeyClientScopeClass,
    pub client_scope_label: &'a str,

    pub outcome: PasskeyOutcomeClass,
    pub outcome_label: &'a str,
    pub fallback: PasskeyFallbackClass,
    pub fallback_label: &'a str,

    pub target_action_preservation: PasskeyTargetActionPreservationClass,
    pub target_action_preservation_label: &'a str,

    pub admin_policy_denies_passkey: bool,
    pub requested_authority_scope: PasskeyAuthorityScopeClass,
    pub granted_authority_scope: PasskeyAuthorityScopeClass,
    pub authority_scope_summary_label: &'a str,

    pub plain_language_summary: &'a str,
}

impl<'a> StagePasskeyStepUpBetaRowRequest<'a> {
    /// Mint a beta row with all closed-vocabulary tokens stamped from the
    /// passed inputs.
    pub fn stage(self) -> PasskeyStepUpBetaRow {
        let promised_axes = vec![
            PasskeyStepUpBetaAxis::LaneIdentifiedAndScopedToTargetAction,
            PasskeyStepUpBetaAxis::LifecycleStateAndClientScopeDisclosed,
            PasskeyStepUpBetaAxis::StepUpSatisfiedOrFallbackNamed,
            PasskeyStepUpBetaAxis::TargetActionIdentityPreservedAcrossReauthAndRecovery,
            PasskeyStepUpBetaAxis::NoAuthorityWideningOnReturn,
            PasskeyStepUpBetaAxis::SupportExportVocabularyParity,
        ];
        PasskeyStepUpBetaRow {
            record_kind: PASSKEY_STEP_UP_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: PASSKEY_STEP_UP_BETA_SCHEMA_VERSION,
            shared_contract_ref: PASSKEY_STEP_UP_BETA_SHARED_CONTRACT_REF.to_owned(),
            case_id: self.case_id.to_owned(),
            row_id: self.row_id.to_owned(),
            source_claim_row_ref: self.source_claim_row_ref.to_owned(),
            profile_token: self.profile.as_str().to_owned(),
            lane: PasskeyLaneBlock {
                lane_token: self.lane.as_str().to_owned(),
                target_label: self.target_label.to_owned(),
                requested_action_label: self.requested_action_label.to_owned(),
                originating_target_ref: self.originating_target_ref.map(str::to_owned),
            },
            lifecycle: PasskeyLifecycleBlock {
                state_token: self.lifecycle_state.as_str().to_owned(),
                state_label: self.lifecycle_state_label.to_owned(),
                client_scope_token: self.client_scope.as_str().to_owned(),
                client_scope_label: self.client_scope_label.to_owned(),
            },
            outcome: PasskeyOutcomeBlock {
                outcome_token: self.outcome.as_str().to_owned(),
                outcome_label: self.outcome_label.to_owned(),
                fallback_token: self.fallback.as_str().to_owned(),
                fallback_label: self.fallback_label.to_owned(),
            },
            target_action_preservation: PasskeyTargetActionPreservationBlock {
                preservation_token: self.target_action_preservation.as_str().to_owned(),
                preservation_label: self.target_action_preservation_label.to_owned(),
            },
            admin_policy_denies_passkey: self.admin_policy_denies_passkey,
            requested_authority_scope_token: self.requested_authority_scope.as_str().to_owned(),
            granted_authority_scope_token: self.granted_authority_scope.as_str().to_owned(),
            authority_scope_summary_label: self.authority_scope_summary_label.to_owned(),
            promised_audit_axes: promised_axes,
            plain_language_summary: self.plain_language_summary.to_owned(),
            redaction_class_token: "metadata_only_export_safe".to_owned(),
        }
    }
}

fn step_up_satisfied_row() -> PasskeyStepUpBetaRow {
    StagePasskeyStepUpBetaRowRequest {
        case_id: "step_up_satisfied_active_passkey",
        row_id: "passkey:claimed:payments-prod:step-up-publish",
        source_claim_row_ref: "claim:system-browser-alpha:claimed-identity:managed:payments-prod",
        profile: PasskeyBetaProfileClass::Connected,
        lane: PasskeyBetaLaneClass::StepUpLane,
        target_label: "Editor → Publish to payments-prod",
        requested_action_label: "Publish the current branch to payments-prod.",
        originating_target_ref: Some("target:editor:publish:payments-prod"),
        lifecycle_state: PasskeyLifecycleStateClass::ActiveOnThisDevice,
        lifecycle_state_label: "Passkey active on this device's authenticator.",
        client_scope: PasskeyClientScopeClass::StepUpScopeRiskyWriteAction,
        client_scope_label: "Step-up for one risky write action: publish to payments-prod.",
        outcome: PasskeyOutcomeClass::StepUpSatisfied,
        outcome_label: "Passkey assertion satisfied the step-up.",
        fallback: PasskeyFallbackClass::NoFallbackRequired,
        fallback_label: "No fallback required; passkey satisfied the lane.",
        target_action_preservation: PasskeyTargetActionPreservationClass::TargetActionPreservedExact,
        target_action_preservation_label:
            "Step-up satisfied for the original publish action; target identity preserved.",
        admin_policy_denies_passkey: false,
        requested_authority_scope: PasskeyAuthorityScopeClass::StepUpScope,
        granted_authority_scope: PasskeyAuthorityScopeClass::StepUpScope,
        authority_scope_summary_label:
            "Requested step-up authority for one publish action; granted scope equals requested.",
        plain_language_summary:
            "Step-up lane satisfied on payments-prod: the local authenticator's passkey approved one publish action, the original target was preserved, and no scope widened beyond step-up.",
    }
    .stage()
}

fn reauth_fallback_on_unsupported_platform_row() -> PasskeyStepUpBetaRow {
    StagePasskeyStepUpBetaRowRequest {
        case_id: "reauth_fallback_on_unsupported_platform",
        row_id: "passkey:claimed:payments-prod:reauth-fallback",
        source_claim_row_ref: "claim:system-browser-alpha:claimed-identity:managed:payments-prod",
        profile: PasskeyBetaProfileClass::Connected,
        lane: PasskeyBetaLaneClass::ReauthLane,
        target_label: "Activity center → Resume managed sign-in",
        requested_action_label: "Resume managed sign-in after refresh expired.",
        originating_target_ref: Some("target:activity-center:resume-managed-sign-in"),
        lifecycle_state: PasskeyLifecycleStateClass::UnavailableThisPlatform,
        lifecycle_state_label:
            "Current platform does not support WebAuthn; passkey unavailable.",
        client_scope: PasskeyClientScopeClass::ReauthScopeRefreshSession,
        client_scope_label: "Reauth scope: refresh the managed session without widening.",
        outcome: PasskeyOutcomeClass::FallbackEngaged,
        outcome_label: "Fallback engaged because WebAuthn is unsupported on this platform.",
        fallback: PasskeyFallbackClass::ResumeAfterStepUpInSystemBrowser,
        fallback_label: "Resume after a typed step-up via system browser.",
        target_action_preservation: PasskeyTargetActionPreservationClass::TargetActionPreservedExact,
        target_action_preservation_label:
            "Fallback resumes the same managed-sign-in target; target identity preserved.",
        admin_policy_denies_passkey: false,
        requested_authority_scope: PasskeyAuthorityScopeClass::ReadWriteScope,
        granted_authority_scope: PasskeyAuthorityScopeClass::NoScopeGranted,
        authority_scope_summary_label:
            "Reauth pending; the fallback path opens before scope is granted.",
        plain_language_summary:
            "Reauth lane on payments-prod with an unsupported platform: the row names resume-after-step-up-in-system-browser as the fallback, preserves the original managed-sign-in target, and grants no scope until the fallback completes.",
    }
    .stage()
}

fn recovery_lane_row_with_admin_policy_deny() -> PasskeyStepUpBetaRow {
    StagePasskeyStepUpBetaRowRequest {
        case_id: "recovery_lane_admin_policy_denies_passkey",
        row_id: "passkey:claimed:payments-prod:recovery-policy-deny",
        source_claim_row_ref: "claim:system-browser-alpha:claimed-identity:managed:payments-prod",
        profile: PasskeyBetaProfileClass::EnterpriseManaged,
        lane: PasskeyBetaLaneClass::RecoveryLane,
        target_label: "Account menu → Recover signing key",
        requested_action_label: "Rebind authenticator after device loss.",
        originating_target_ref: Some("target:account-menu:recover-signing-key"),
        lifecycle_state: PasskeyLifecycleStateClass::Revoked,
        lifecycle_state_label: "Passkey revoked after device loss; rebind required.",
        client_scope: PasskeyClientScopeClass::RecoveryScopeRebindAuthenticator,
        client_scope_label: "Recovery scope: rebind authenticator on the existing account.",
        outcome: PasskeyOutcomeClass::StepUpDeniedByPolicy,
        outcome_label: "Admin policy denies passkey rebind on this row.",
        fallback: PasskeyFallbackClass::ContactAdminForRecovery,
        fallback_label:
            "Contact your admin to complete recovery; an out-of-band flow will rebind your account.",
        target_action_preservation:
            PasskeyTargetActionPreservationClass::TargetActionDownscoped,
        target_action_preservation_label:
            "Recovery downscoped to read-only on the same workspace; the original target identity is preserved.",
        admin_policy_denies_passkey: true,
        requested_authority_scope: PasskeyAuthorityScopeClass::ReadWriteScope,
        granted_authority_scope: PasskeyAuthorityScopeClass::ReadOnlyScope,
        authority_scope_summary_label:
            "Recovery denied passkey rebind; granted read-only until the admin flow completes.",
        plain_language_summary:
            "Recovery lane on payments-prod: admin policy denies passkey rebind, the row names contact-admin-for-recovery as the fallback, preserves the original recovery target, and downscopes to read-only without widening beyond the requested scope.",
    }
    .stage()
}

fn account_free_local_row() -> PasskeyStepUpBetaRow {
    StagePasskeyStepUpBetaRowRequest {
        case_id: "account_free_local_no_passkey_lane",
        row_id: "passkey:local:account-free",
        source_claim_row_ref: "claim:system-browser-alpha:claimed-identity:local:account-free",
        profile: PasskeyBetaProfileClass::Connected,
        lane: PasskeyBetaLaneClass::NotApplicableAccountFreeLocal,
        target_label: "",
        requested_action_label: "",
        originating_target_ref: None,
        lifecycle_state: PasskeyLifecycleStateClass::NotApplicableAccountFreeLocal,
        lifecycle_state_label: "Account-free local mode: no passkey lifecycle to track.",
        client_scope: PasskeyClientScopeClass::NotApplicableAccountFreeLocal,
        client_scope_label: "Account-free local mode grants no remote scope.",
        outcome: PasskeyOutcomeClass::NotApplicableAccountFreeLocal,
        outcome_label: "No passkey lane on account-free local rows.",
        fallback: PasskeyFallbackClass::NotApplicableAccountFreeLocal,
        fallback_label: "Account-free local rows need no fallback path.",
        target_action_preservation:
            PasskeyTargetActionPreservationClass::NotApplicableAccountFreeLocal,
        target_action_preservation_label:
            "Account-free local row: no target / action identity to preserve.",
        admin_policy_denies_passkey: false,
        requested_authority_scope: PasskeyAuthorityScopeClass::NoScopeGranted,
        granted_authority_scope: PasskeyAuthorityScopeClass::NoScopeGranted,
        authority_scope_summary_label:
            "Account-free local mode grants no remote scope.",
        plain_language_summary:
            "Account-free local row: no passkey lane is claimed, no fallback is required, and no remote scope is granted; local editing remains the source of truth.",
    }
    .stage()
}

/// Build the seeded beta page that the live shell, the headless inspector, and
/// the integration test all consume.
pub fn seeded_passkey_step_up_beta_page() -> PasskeyStepUpBetaPage {
    PasskeyStepUpBetaPage::new(
        "auth:passkey_step_up_beta:default",
        "Passkey-capable step-up, reauth, and recovery lanes (beta)",
        "2026-05-16T00:00:00Z",
        vec![
            step_up_satisfied_row(),
            reauth_fallback_on_unsupported_platform_row(),
            recovery_lane_row_with_admin_policy_deny(),
            account_free_local_row(),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page() -> PasskeyStepUpBetaPage {
        seeded_passkey_step_up_beta_page()
    }

    #[test]
    fn seeded_page_seeds_zero_defects_and_audits_clean() {
        let page = page();
        assert_eq!(page.summary.defect_count, 0);
        assert!(page.defects.is_empty());
        assert!(validate_passkey_step_up_beta_page(&page).is_ok());
        assert!(page.reauth_and_recovery_preserve_target_action_identity());
        assert!(page.fallback_named_when_passkey_unavailable());
    }

    #[test]
    fn seeded_page_includes_step_up_reauth_and_recovery_rows() {
        let page = page();
        assert!(page.summary.step_up_row_count >= 1);
        assert!(page.summary.reauth_row_count >= 1);
        assert!(page.summary.recovery_row_count >= 1);
        assert!(page
            .summary
            .lanes_present
            .contains(&"step_up_lane".to_owned()));
        assert!(page
            .summary
            .lanes_present
            .contains(&"reauth_lane".to_owned()));
        assert!(page
            .summary
            .lanes_present
            .contains(&"recovery_lane".to_owned()));
        assert!(page
            .summary
            .lanes_present
            .contains(&"not_applicable_account_free_local".to_owned()));
    }

    #[test]
    fn defect_drill_recovery_rerouted_is_caught() {
        let mut page = page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.lane.lane_token == PasskeyBetaLaneClass::RecoveryLane.as_str())
            .unwrap();
        row.target_action_preservation.preservation_token =
            PasskeyTargetActionPreservationClass::TargetActionRerouted
                .as_str()
                .to_owned();
        let support_rows: Vec<PasskeyStepUpBetaSupportRow> = page
            .rows
            .iter()
            .map(PasskeyStepUpBetaSupportRow::from_row)
            .collect();
        let defects = audit_passkey_step_up_beta_rows(&page.rows, &support_rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == PasskeyStepUpBetaDefectKind::ReauthOrRecoveryRerouted));
    }

    #[test]
    fn defect_drill_reauth_widened_is_caught() {
        let mut page = page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.lane.lane_token == PasskeyBetaLaneClass::ReauthLane.as_str())
            .unwrap();
        row.target_action_preservation.preservation_token =
            PasskeyTargetActionPreservationClass::TargetActionWidened
                .as_str()
                .to_owned();
        let support_rows: Vec<PasskeyStepUpBetaSupportRow> = page
            .rows
            .iter()
            .map(PasskeyStepUpBetaSupportRow::from_row)
            .collect();
        let defects = audit_passkey_step_up_beta_rows(&page.rows, &support_rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == PasskeyStepUpBetaDefectKind::ReauthOrRecoveryWidened));
    }

    #[test]
    fn defect_drill_outcome_unsatisfied_without_fallback_is_caught() {
        let mut page = page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.lane.lane_token == PasskeyBetaLaneClass::ReauthLane.as_str())
            .unwrap();
        row.outcome.fallback_token = PasskeyFallbackClass::NoFallbackRequired.as_str().to_owned();
        let support_rows: Vec<PasskeyStepUpBetaSupportRow> = page
            .rows
            .iter()
            .map(PasskeyStepUpBetaSupportRow::from_row)
            .collect();
        let defects = audit_passkey_step_up_beta_rows(&page.rows, &support_rows);
        assert!(defects.iter().any(|d| d.defect_kind
            == PasskeyStepUpBetaDefectKind::OutcomeUnsatisfiedWithoutFallback
            || d.defect_kind == PasskeyStepUpBetaDefectKind::LifecycleStateRequiresFallback));
    }

    #[test]
    fn defect_drill_granted_widens_requested_is_caught() {
        let mut page = page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.lane.lane_token == PasskeyBetaLaneClass::StepUpLane.as_str())
            .unwrap();
        row.granted_authority_scope_token = PasskeyAuthorityScopeClass::TenantAdminScope
            .as_str()
            .to_owned();
        let support_rows: Vec<PasskeyStepUpBetaSupportRow> = page
            .rows
            .iter()
            .map(PasskeyStepUpBetaSupportRow::from_row)
            .collect();
        let defects = audit_passkey_step_up_beta_rows(&page.rows, &support_rows);
        assert!(
            defects
                .iter()
                .any(|d| d.defect_kind
                    == PasskeyStepUpBetaDefectKind::GrantedAuthorityWidensRequested)
        );
    }

    #[test]
    fn defect_drill_support_row_drift_is_caught() {
        let page = page();
        let mut support_rows = page.support_rows.clone();
        support_rows[0].granted_authority_scope_token =
            PasskeyAuthorityScopeClass::WorkspaceAdminScope
                .as_str()
                .to_owned();
        let defects = audit_passkey_step_up_beta_rows(&page.rows, &support_rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == PasskeyStepUpBetaDefectKind::SupportRowVocabularyDrift));
    }

    #[test]
    fn defect_drill_account_free_local_mislabeled_is_caught() {
        let mut page = page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| {
                r.lane.lane_token == PasskeyBetaLaneClass::NotApplicableAccountFreeLocal.as_str()
            })
            .unwrap();
        // Drift the lifecycle to a non-local state while keeping the local
        // lane token — the row should be flagged.
        row.lifecycle.state_token = PasskeyLifecycleStateClass::ActiveOnThisDevice
            .as_str()
            .to_owned();
        let support_rows: Vec<PasskeyStepUpBetaSupportRow> = page
            .rows
            .iter()
            .map(PasskeyStepUpBetaSupportRow::from_row)
            .collect();
        let defects = audit_passkey_step_up_beta_rows(&page.rows, &support_rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == PasskeyStepUpBetaDefectKind::AccountFreeLocalLaneMislabeled));
    }

    #[test]
    fn support_export_round_trips_with_zero_defects() {
        let page = page();
        let export = PasskeyStepUpBetaSupportExport::from_page(
            "support-export:passkey-step-up:001",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
        assert!(export.defect_counts_by_kind.is_empty());
    }
}
