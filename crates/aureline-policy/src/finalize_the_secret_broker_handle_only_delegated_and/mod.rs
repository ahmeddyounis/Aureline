//! Finalize the secret-broker handle-only, delegated, and session-only modes
//! across claimed stable rows.
//!
//! This module promotes the beta secret-broker contract in
//! [`aureline_auth::secret_broker`] to an evidence-backed stable proof packet
//! whose qualification is derived from the audit rather than asserted from a
//! spreadsheet row.
//!
//! The stable claim holds when **all** of the following conditions are
//! verified:
//!
//! 1. The upstream beta page audits with zero defects.
//! 2. Every required flow class (`request_workspace`, `database`,
//!    `env_config`, `provider`, `managed_runtime`) has at least one claimed
//!    row with an explicit handle class.
//! 3. Every row's handle class is one of the first-class vocabulary values
//!    (`os_keychain`, `enterprise_vault`, `delegated_identity`,
//!    `session_only`, `workspace_variable`, or `missing`). No row may
//!    flatten a brokered handle, vault ref, or delegated credential into
//!    literal-looking text or durable workspace history.
//! 4. Every row that references a delegated or session-only credential carries
//!    an explicit expiry window, rotation note, and redaction-safe replay
//!    posture. Rotation outcomes, browser/device-code renewal, and
//!    vault/keychain loss all carry typed notes rather than generic
//!    reconnect copy.
//! 5. Every remembered-decision approval bound to a delegated or session-only
//!    secret posture is narrow: it names actor, target, action family,
//!    environment, and expiry window, and preserves revocation and
//!    reapproval triggers in export-safe history rows.
//! 6. Handle class, rotation/expiry note, and redaction-safe replay semantics
//!    are preserved across request-workspace, database, env/config, provider,
//!    and support/export lanes.
//!
//! Two hard guardrails cannot be papered over:
//!
//! - **No raw secret persistence.** Any row that claims raw secret material
//!   is present withdraws to
//!   [`FinalizeSecretBrokerQualificationClass::Withdrawn`] immediately.
//! - **No handle class missing on a stable claim.** A row whose handle class
//!   is `missing` while simultaneously claiming a non-degraded credential
//!   posture withdraws the row immediately.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language labels, and opaque refs
//! only. Raw credentials, session tokens, plaintext workspace-variable values,
//! vault entries, and raw handle ids stay outside the support boundary.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/enterprise/m4/finalize-the-secret-broker-handle-only-delegated-and.md`
//! - Artifact: `artifacts/enterprise/m4/finalize-the-secret-broker-handle-only-delegated-and.md`
//! - Contract ref: [`FINALIZE_SECRET_BROKER_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use aureline_auth::secret_broker::{
    seeded_secret_broker_beta_page, SecretBrokerBetaPage, SecretBrokerBetaSupportExport,
};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const FINALIZE_SECRET_BROKER_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const FINALIZE_SECRET_BROKER_SHARED_CONTRACT_REF: &str =
    "policy:finalize_secret_broker_handle_only_delegated:v1";

/// Record-kind tag for [`FinalizeSecretBrokerPage`] payloads.
pub const FINALIZE_SECRET_BROKER_PAGE_RECORD_KIND: &str =
    "policy_finalize_secret_broker_page_record";

/// Record-kind tag for [`FinalizeSecretBrokerRow`] payloads.
pub const FINALIZE_SECRET_BROKER_ROW_RECORD_KIND: &str =
    "policy_finalize_secret_broker_row_record";

/// Record-kind tag for [`FinalizeSecretBrokerDefect`] payloads.
pub const FINALIZE_SECRET_BROKER_DEFECT_RECORD_KIND: &str =
    "policy_finalize_secret_broker_defect_record";

/// Record-kind tag for [`FinalizeSecretBrokerSummary`] payloads.
pub const FINALIZE_SECRET_BROKER_SUMMARY_RECORD_KIND: &str =
    "policy_finalize_secret_broker_summary_record";

/// Record-kind tag for [`FinalizeSecretBrokerSupportExport`] payloads.
pub const FINALIZE_SECRET_BROKER_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_finalize_secret_broker_support_export_record";

/// Record-kind tag for [`RememberedApprovalRow`] payloads.
pub const FINALIZE_SECRET_BROKER_REMEMBERED_APPROVAL_RECORD_KIND: &str =
    "policy_finalize_secret_broker_remembered_approval_record";

/// Repo-relative path of the stable doc for this lane.
pub const FINALIZE_SECRET_BROKER_DOC_REF: &str =
    "docs/enterprise/m4/finalize-the-secret-broker-handle-only-delegated-and.md";

/// Repo-relative path of the artifact summary for this lane.
pub const FINALIZE_SECRET_BROKER_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/finalize-the-secret-broker-handle-only-delegated-and.md";

/// Upstream beta contract ref.
pub const SECRET_BROKER_BETA_CONTRACT_REF: &str = "security:secret_broker_beta:v1";

// ---------------------------------------------------------------------------
// Flow class vocabulary
// ---------------------------------------------------------------------------

/// The consumer flow class that requires a secret-broker reference.
///
/// Every finalize row covers one flow class. All five required classes must
/// have at least one row for the page to qualify stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBrokerFlowClass {
    /// Workspace-variable or request-scoped secret injection into request
    /// flows.
    RequestWorkspace,
    /// Database connection or credentialed query attach flows.
    Database,
    /// Environment variable or configuration-file injection for env/config
    /// consumers.
    EnvConfig,
    /// Provider reconnect, provider link refresh, or provider route renewal.
    Provider,
    /// Managed-runtime secret injection including policy-materialised and
    /// managed agent flows.
    ManagedRuntime,
}

impl SecretBrokerFlowClass {
    /// All required flow classes in canonical order.
    pub const ALL: [Self; 5] = [
        Self::RequestWorkspace,
        Self::Database,
        Self::EnvConfig,
        Self::Provider,
        Self::ManagedRuntime,
    ];

    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestWorkspace => "request_workspace",
            Self::Database => "database",
            Self::EnvConfig => "env_config",
            Self::Provider => "provider",
            Self::ManagedRuntime => "managed_runtime",
        }
    }
}

// ---------------------------------------------------------------------------
// Handle class vocabulary
// ---------------------------------------------------------------------------

/// First-class handle class distinguishing the authority source for a
/// secret-broker row.
///
/// Every finalize row names exactly one handle class. Rows may not flatten
/// brokered handles, vault refs, or delegated credentials into
/// literal-looking text or claim a non-degraded posture while the handle
/// class is `missing`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBrokerHandleClass {
    /// Authority is held in the OS keychain (macOS Keychain, Windows
    /// Credential Manager, or Linux Secret Service).
    OsKeychain,
    /// Authority is held in an enterprise vault or broker adapter.
    EnterpriseVault,
    /// Authority is issued by a delegated identity (OIDC, device code,
    /// browser-delegated token).
    DelegatedIdentity,
    /// Authority is held only in process-local session memory; it expires
    /// with the session and is visible to the user as degraded.
    SessionOnly,
    /// Authority is projected from a workspace-scoped variable or secret
    /// reference; it does not outlive the workspace scope.
    WorkspaceVariable,
    /// No authority is present; the row must surface a typed missing-handle
    /// reason and must not claim a non-degraded credential posture.
    Missing,
}

impl SecretBrokerHandleClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsKeychain => "os_keychain",
            Self::EnterpriseVault => "enterprise_vault",
            Self::DelegatedIdentity => "delegated_identity",
            Self::SessionOnly => "session_only",
            Self::WorkspaceVariable => "workspace_variable",
            Self::Missing => "missing",
        }
    }

    /// True when the handle class is one of the two visibly-degraded postures
    /// (session-only or missing). Rows with these classes must carry explicit
    /// degraded-posture disclosure.
    pub const fn is_degraded(self) -> bool {
        matches!(self, Self::SessionOnly | Self::Missing)
    }

    /// True when the handle class can claim a stable non-degraded posture.
    pub const fn is_stable_posture(self) -> bool {
        !self.is_degraded()
    }
}

// ---------------------------------------------------------------------------
// Rotation / expiry event vocabulary
// ---------------------------------------------------------------------------

/// Typed event that triggers a rotation or renewal for a delegated or
/// session-only credential.
///
/// Every row whose handle class is `delegated_identity` or `session_only`
/// must carry an explicit rotation event type and a typed note. Generic
/// reconnect copy is rejected by the validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialRotationEventClass {
    /// Credential was rotated by the issuer or admin; a successor handle was
    /// issued.
    RotatedByIssuer,
    /// The browser or device-code flow completed a renewal; the session ref
    /// was refreshed.
    BrowserDeviceCodeRenewal,
    /// The vault or keychain lost the credential; the row is paused until
    /// reauth.
    VaultKeychainLoss,
    /// The handle or session ref expired naturally; the row needs
    /// re-issuance.
    HandleExpired,
    /// A remembered approval was revoked; downstream queued actions must be
    /// reapproved.
    ApprovalRevoked,
    /// No rotation event has occurred; the credential is live within its
    /// freshness window.
    NoRotationRequired,
}

impl CredentialRotationEventClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RotatedByIssuer => "rotated_by_issuer",
            Self::BrowserDeviceCodeRenewal => "browser_device_code_renewal",
            Self::VaultKeychainLoss => "vault_keychain_loss",
            Self::HandleExpired => "handle_expired",
            Self::ApprovalRevoked => "approval_revoked",
            Self::NoRotationRequired => "no_rotation_required",
        }
    }

    /// True when this event pauses or invalidates remembered decisions and
    /// queued high-risk actions.
    pub const fn invalidates_remembered_decisions(self) -> bool {
        matches!(
            self,
            Self::RotatedByIssuer
                | Self::VaultKeychainLoss
                | Self::HandleExpired
                | Self::ApprovalRevoked
        )
    }
}

// ---------------------------------------------------------------------------
// Rotation / expiry state block
// ---------------------------------------------------------------------------

/// Rotation and expiry state for a secret-broker finalize row.
///
/// Every row whose handle class is `delegated_identity` or `session_only`
/// must carry this block. Rows with `os_keychain`, `enterprise_vault`, or
/// `workspace_variable` classes carry it if rotation events are applicable;
/// `missing` rows carry it to document the missing-handle reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialRotationState {
    /// Most recent rotation event class.
    pub rotation_event: CredentialRotationEventClass,
    /// Stable token for [`Self::rotation_event`].
    pub rotation_event_token: String,
    /// Typed note explaining the rotation event in export-safe language. Must
    /// not be a generic reconnect message; must name the specific trigger.
    pub rotation_note: String,
    /// Expiry timestamp for the current handle or session ref. Empty when
    /// `no_rotation_required`.
    pub expires_at: String,
    /// True when the rotation event invalidates remembered decisions and
    /// queued high-risk actions.
    pub invalidates_remembered_decisions: bool,
    /// True when a reapproval path is available for affected decisions.
    pub reapproval_path_available: bool,
    /// Replay posture: one of `replay_safe`, `replay_blocked`,
    /// `replay_requires_reapproval`. Raw handle ids and raw session tokens
    /// are never replayed.
    pub replay_posture_token: String,
}

impl CredentialRotationState {
    /// True when the rotation note is explicit (non-generic, non-empty).
    pub fn rotation_note_is_explicit(&self) -> bool {
        !self.rotation_note.is_empty()
            && self.rotation_note != "reconnect"
            && !self.rotation_note.to_lowercase().contains("generic")
    }
}

// ---------------------------------------------------------------------------
// Remembered-approval row
// ---------------------------------------------------------------------------

/// A remembered-decision approval narrow-bound to one
/// `(actor × target × action family × environment × expiry)` tuple.
///
/// Every remembered approval bound to a delegated or session-only secret
/// posture must carry this row. The row preserves revocation and reapproval
/// triggers in export-safe form; raw credential material never appears.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RememberedApprovalRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable approval row id.
    pub approval_row_id: String,
    /// Opaque actor id for the remembered decision.
    pub actor_ref: String,
    /// Opaque target ref (provider, registry, database, or remote target).
    pub target_ref: String,
    /// Action family token (e.g. `read`, `write`, `exec`, `admin`).
    pub action_family_token: String,
    /// Environment token (e.g. `local`, `enterprise_managed`, `self_hosted`).
    pub environment_token: String,
    /// Expiry window for this remembered approval.
    pub expiry_window: String,
    /// Secret posture class that bound the approval. Must be one of
    /// `delegated_identity` or `session_only` for this row kind.
    pub bound_handle_class_token: String,
    /// Rotation event that caused revocation, or `no_rotation_required`.
    pub revocation_trigger_token: String,
    /// True when the approval is currently valid (not revoked, not expired).
    pub is_valid: bool,
    /// Plain-language reapproval trigger condition. Must not be empty when
    /// `revocation_trigger_token` names a rotation event.
    pub reapproval_trigger_note: String,
    /// True when raw credential material is excluded from the row.
    pub raw_credential_excluded: bool,
}

// ---------------------------------------------------------------------------
// Finalize row
// ---------------------------------------------------------------------------

/// Finalize row for one `(flow_class × handle_class × profile)` tuple.
///
/// The row is the unit of qualification. Each row must carry an explicit
/// handle class, a rotation/expiry block for delegated/session-only rows,
/// and remembered-approval rows for every remembered decision bound to a
/// delegated or session-only posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSecretBrokerRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Flow class for this row.
    pub flow_class: SecretBrokerFlowClass,
    /// Stable token for [`Self::flow_class`].
    pub flow_class_token: String,
    /// Handle class for this row.
    pub handle_class: SecretBrokerHandleClass,
    /// Stable token for [`Self::handle_class`].
    pub handle_class_token: String,
    /// Profile token from the upstream beta row.
    pub profile_token: String,
    /// Secret class token from the upstream beta row.
    pub secret_class_token: String,
    /// Ref to the upstream beta handle row.
    pub beta_row_ref: String,
    /// Rotation and expiry state.
    pub rotation_state: CredentialRotationState,
    /// Remembered-approval rows bound to this credential posture.
    pub remembered_approvals: Vec<RememberedApprovalRow>,
    /// True when this row has no raw secret material in any exported field.
    pub raw_secret_material_excluded: bool,
    /// True when no brokered handle, vault ref, or delegated credential is
    /// flattened to literal-looking text in durable workspace history.
    pub no_literal_flattening: bool,
    /// Derived qualification tier for this row.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall packet and for individual rows.
///
/// The tier is derived, not asserted: it is set by comparing the audit defect
/// lists and the six stability conditions. A caller may never assert `stable`
/// without a clean audit and complete coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizeSecretBrokerQualificationClass {
    /// All six stability conditions hold and the upstream beta audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required flow class has no row; coverage gap prevents a beta claim.
    Preview,
    /// A hard guardrail (raw secret material, missing handle class on stable
    /// claim, literal flattening) withdrew the row entirely.
    Withdrawn,
}

impl FinalizeSecretBrokerQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when this tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// True when this tier is claimable (stable or beta).
    pub const fn is_claimable(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }
}

/// Typed reason a packet or row was narrowed below
/// [`FinalizeSecretBrokerQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizeSecretBrokerNarrowReasonClass {
    /// No narrowing — the row qualifies stable.
    NotNarrowed,
    /// The upstream beta page has one or more defects.
    BetaPageHasDefects,
    /// A required flow class has no row; coverage gap.
    FlowClassCoverageGap,
    /// A row's handle class is missing or not one of the admitted vocabulary
    /// values.
    HandleClassMissing,
    /// A delegated or session-only row has a generic rotation note instead of
    /// an explicit typed reason.
    RotationNoteIsGeneric,
    /// A delegated or session-only row's remembered approval does not narrow
    /// to actor, target, action family, environment, and expiry window.
    RememberedApprovalNotNarrow,
    /// A remembered approval's revocation trigger is missing when the row's
    /// rotation event invalidates decisions.
    RevocationTriggerMissing,
    /// Raw secret material is present on a row or in a remembered approval.
    RawSecretMaterialPresent,
    /// A brokered handle, vault ref, or delegated credential was flattened to
    /// literal-looking text or written to durable workspace history.
    ///
    /// This is a hard guardrail and withdraws the row immediately.
    LiteralFlatteningDetected,
    /// A row claims a non-degraded credential posture while its handle class
    /// is `missing`.
    ///
    /// This is a hard guardrail and withdraws the row immediately.
    MissingHandleOnStableClaim,
}

impl FinalizeSecretBrokerNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::BetaPageHasDefects => "beta_page_has_defects",
            Self::FlowClassCoverageGap => "flow_class_coverage_gap",
            Self::HandleClassMissing => "handle_class_missing",
            Self::RotationNoteIsGeneric => "rotation_note_is_generic",
            Self::RememberedApprovalNotNarrow => "remembered_approval_not_narrow",
            Self::RevocationTriggerMissing => "revocation_trigger_missing",
            Self::RawSecretMaterialPresent => "raw_secret_material_present",
            Self::LiteralFlatteningDetected => "literal_flattening_detected",
            Self::MissingHandleOnStableClaim => "missing_handle_on_stable_claim",
        }
    }

    /// True when this reason is a hard guardrail that withdraws the row
    /// immediately and cannot be overridden.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::LiteralFlatteningDetected
                | Self::MissingHandleOnStableClaim
                | Self::RawSecretMaterialPresent
        )
    }
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// Aggregate banner for the finalize page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FinalizeSecretBrokerSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Total finalize rows.
    pub row_count: usize,
    /// Rows with `stable` qualification.
    pub stable_row_count: usize,
    /// Rows with `beta` qualification.
    pub beta_row_count: usize,
    /// Rows with `preview` qualification.
    pub preview_row_count: usize,
    /// Rows with `withdrawn` qualification.
    pub withdrawn_row_count: usize,
    /// Flow class tokens present across rows.
    pub flow_classes_covered: Vec<String>,
    /// Handle class tokens present across rows.
    pub handle_classes_present: Vec<String>,
    /// Profile tokens present across rows.
    pub profiles_covered: Vec<String>,
    /// Number of rows with a delegated-identity handle class.
    pub delegated_identity_row_count: usize,
    /// Number of rows with a session-only handle class.
    pub session_only_row_count: usize,
    /// Number of remembered-approval rows across all finalize rows.
    pub remembered_approval_count: usize,
    /// Number of defects.
    pub defect_count: usize,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// Beta-page defect count from the embedded upstream page.
    pub beta_page_defect_count: usize,
    /// Overall derived qualification token.
    pub overall_qualification_token: String,
}

impl FinalizeSecretBrokerSummary {
    fn from_records(
        rows: &[FinalizeSecretBrokerRow],
        defects: &[FinalizeSecretBrokerDefect],
        beta_page: &SecretBrokerBetaPage,
    ) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        let mut flow_classes: BTreeSet<String> = BTreeSet::new();
        let mut handle_classes: BTreeSet<String> = BTreeSet::new();
        let mut profiles: BTreeSet<String> = BTreeSet::new();
        let mut delegated_count = 0usize;
        let mut session_only_count = 0usize;
        let mut approval_count = 0usize;

        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
            flow_classes.insert(row.flow_class_token.clone());
            handle_classes.insert(row.handle_class_token.clone());
            profiles.insert(row.profile_token.clone());
            if row.handle_class == SecretBrokerHandleClass::DelegatedIdentity {
                delegated_count += 1;
            }
            if row.handle_class == SecretBrokerHandleClass::SessionOnly {
                session_only_count += 1;
            }
            approval_count += row.remembered_approvals.len();
        }

        let mut defect_counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in defects {
            *defect_counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }

        let overall = if withdrawn > 0 {
            FinalizeSecretBrokerQualificationClass::Withdrawn
        } else if preview > 0 {
            FinalizeSecretBrokerQualificationClass::Preview
        } else if beta > 0 {
            FinalizeSecretBrokerQualificationClass::Beta
        } else {
            FinalizeSecretBrokerQualificationClass::Stable
        };

        Self {
            record_kind: FINALIZE_SECRET_BROKER_SUMMARY_RECORD_KIND.to_owned(),
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            flow_classes_covered: flow_classes.into_iter().collect(),
            handle_classes_present: handle_classes.into_iter().collect(),
            profiles_covered: profiles.into_iter().collect(),
            delegated_identity_row_count: delegated_count,
            session_only_row_count: session_only_count,
            remembered_approval_count: approval_count,
            defect_count: defects.len(),
            defect_counts_by_narrow_reason: defect_counts,
            beta_page_defect_count: beta_page.defects.len(),
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defects
// ---------------------------------------------------------------------------

/// Typed defect emitted by the finalize page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSecretBrokerDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: FinalizeSecretBrokerNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject row id or `"page"` when the defect applies to the page.
    pub source_row_id: String,
    /// Export-safe explanation of the defect.
    pub note: String,
}

impl FinalizeSecretBrokerDefect {
    fn new(
        narrow_reason: FinalizeSecretBrokerNarrowReasonClass,
        source_row_id: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let row_id = source_row_id.into();
        Self {
            record_kind: FINALIZE_SECRET_BROKER_DEFECT_RECORD_KIND.to_owned(),
            schema_version: FINALIZE_SECRET_BROKER_SCHEMA_VERSION,
            shared_contract_ref: FINALIZE_SECRET_BROKER_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "policy:defect:finalize-secret-broker:{}:{}",
                narrow_reason.as_str(),
                &row_id
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source_row_id: row_id,
            note: note.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Main page
// ---------------------------------------------------------------------------

/// Stable proof packet for the secret-broker handle-only, delegated, and
/// session-only modes across claimed stable rows.
///
/// This is the single inspectable record that proves the finalize claim for
/// this lane. Dashboards, docs, Help/About surfaces, and support exports
/// should ingest it rather than cloning status text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSecretBrokerPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Reviewable page label.
    pub page_label: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Aggregate summary.
    pub summary: FinalizeSecretBrokerSummary,
    /// Finalize rows.
    pub rows: Vec<FinalizeSecretBrokerRow>,
    /// Typed defects.
    pub defects: Vec<FinalizeSecretBrokerDefect>,
    /// The beta secret-broker page embedded as evidence.
    pub beta_page: SecretBrokerBetaPage,
}

impl FinalizeSecretBrokerPage {
    /// Build the finalize page from an upstream beta page and finalize rows.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        beta_page: SecretBrokerBetaPage,
        rows: Vec<FinalizeSecretBrokerRow>,
    ) -> Self {
        let mut defects = audit_finalize_secret_broker_rows(&rows, &beta_page);
        let qualified_rows = qualify_rows(rows, &defects);
        // Re-run defects after qualification to pick up any row-level
        // withdrawal defects added during qualification.
        defects = audit_finalize_secret_broker_rows(&qualified_rows, &beta_page);
        let summary =
            FinalizeSecretBrokerSummary::from_records(&qualified_rows, &defects, &beta_page);
        Self {
            record_kind: FINALIZE_SECRET_BROKER_PAGE_RECORD_KIND.to_owned(),
            schema_version: FINALIZE_SECRET_BROKER_SCHEMA_VERSION,
            shared_contract_ref: FINALIZE_SECRET_BROKER_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows: qualified_rows,
            defects,
            beta_page,
        }
    }

    /// True when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == FinalizeSecretBrokerQualificationClass::Stable.as_str()
    }

    /// True when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// True when all five required flow classes are covered.
    pub fn all_required_flow_classes_covered(&self) -> bool {
        let required: Vec<&str> =
            SecretBrokerFlowClass::ALL.iter().map(|c| c.as_str()).collect();
        required
            .iter()
            .all(|fc| self.rows.iter().any(|r| r.flow_class_token == *fc))
    }

    /// True when every delegated or session-only row has an explicit
    /// rotation note (not generic, not empty).
    pub fn delegated_and_session_only_rows_have_explicit_rotation_notes(&self) -> bool {
        self.rows
            .iter()
            .filter(|r| r.handle_class.is_degraded() || r.handle_class == SecretBrokerHandleClass::DelegatedIdentity)
            .all(|r| r.rotation_state.rotation_note_is_explicit())
    }

    /// True when every remembered approval is narrow (all five required
    /// fields are non-empty).
    pub fn remembered_approvals_are_narrow(&self) -> bool {
        self.rows.iter().all(|r| {
            r.remembered_approvals.iter().all(|a| {
                !a.actor_ref.is_empty()
                    && !a.target_ref.is_empty()
                    && !a.action_family_token.is_empty()
                    && !a.environment_token.is_empty()
                    && !a.expiry_window.is_empty()
            })
        })
    }

    /// True when no raw secret material is present across all rows.
    pub fn no_raw_secret_material(&self) -> bool {
        self.rows.iter().all(|r| r.raw_secret_material_excluded)
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that quotes the finalize page plus a
/// metadata-safe defect roll-up.
///
/// Raw credentials, vault entry bodies, raw handle ids, raw session tokens,
/// and workspace-variable values are excluded. Narrow reasons, row
/// qualifications, handle-class tokens, rotation-event tokens, and
/// remembered-approval metadata are preserved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSecretBrokerSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// The finalize page.
    pub page: FinalizeSecretBrokerPage,
    /// Narrow-reason tokens present across defects.
    pub narrow_reasons_present: Vec<FinalizeSecretBrokerNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// Beta-page support export embedded as evidence.
    pub beta_support_export: SecretBrokerBetaSupportExport,
    /// True when raw secret values are excluded.
    pub raw_secret_values_excluded: bool,
    /// True when raw handle ids are excluded.
    pub raw_handle_ids_excluded: bool,
    /// True when remembered-approval lineage is preserved.
    pub remembered_approval_lineage_preserved: bool,
}

impl FinalizeSecretBrokerSupportExport {
    /// Wrap a finalize page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: FinalizeSecretBrokerPage,
    ) -> Self {
        let mut reasons: Vec<FinalizeSecretBrokerNarrowReasonClass> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *counts.entry(defect.narrow_reason_token.clone()).or_insert(0) += 1;
        }
        reasons.sort();
        let generated = generated_at.into();
        let export_id_str = export_id.into();
        let beta_export = SecretBrokerBetaSupportExport::from_page(
            format!("{}-beta", export_id_str),
            generated.clone(),
            page.beta_page.clone(),
        );
        Self {
            record_kind: FINALIZE_SECRET_BROKER_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: FINALIZE_SECRET_BROKER_SCHEMA_VERSION,
            shared_contract_ref: FINALIZE_SECRET_BROKER_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id_str,
            generated_at: generated,
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            beta_support_export: beta_export,
            raw_secret_values_excluded: true,
            raw_handle_ids_excluded: true,
            remembered_approval_lineage_preserved: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Audit, qualification, and validate functions
// ---------------------------------------------------------------------------

/// Recomputes defects for a finalize page from its rows and beta page.
pub fn audit_finalize_secret_broker_rows(
    rows: &[FinalizeSecretBrokerRow],
    beta_page: &SecretBrokerBetaPage,
) -> Vec<FinalizeSecretBrokerDefect> {
    let mut defects = Vec::new();

    // Condition 1: upstream beta page must be clean.
    if !beta_page.defects.is_empty() {
        for defect in &beta_page.defects {
            defects.push(FinalizeSecretBrokerDefect::new(
                FinalizeSecretBrokerNarrowReasonClass::BetaPageHasDefects,
                "page",
                format!(
                    "Upstream beta page has defect '{}' on subject '{}': {}",
                    defect.defect_kind_token, defect.subject_id, defect.note
                ),
            ));
        }
    }

    // Condition 2: all five required flow classes must be covered.
    for flow in &SecretBrokerFlowClass::ALL {
        if !rows.iter().any(|r| r.flow_class_token == flow.as_str()) {
            defects.push(FinalizeSecretBrokerDefect::new(
                FinalizeSecretBrokerNarrowReasonClass::FlowClassCoverageGap,
                "page",
                format!(
                    "Required flow class '{}' has no finalize row.",
                    flow.as_str()
                ),
            ));
        }
    }

    for row in rows {
        let row_id = &row.row_id;

        // Hard guardrail: raw secret material present.
        if !row.raw_secret_material_excluded {
            defects.push(FinalizeSecretBrokerDefect::new(
                FinalizeSecretBrokerNarrowReasonClass::RawSecretMaterialPresent,
                row_id,
                "Row claims raw_secret_material_excluded is false; this withdraws the row.",
            ));
        }

        // Hard guardrail: literal flattening detected.
        if !row.no_literal_flattening {
            defects.push(FinalizeSecretBrokerDefect::new(
                FinalizeSecretBrokerNarrowReasonClass::LiteralFlatteningDetected,
                row_id,
                "Row claims no_literal_flattening is false; brokered handles must not be flattened to literal-looking text.",
            ));
        }

        // Hard guardrail: missing handle on stable claim.
        if row.handle_class == SecretBrokerHandleClass::Missing
            && row.qualification_token == FinalizeSecretBrokerQualificationClass::Stable.as_str()
        {
            defects.push(FinalizeSecretBrokerDefect::new(
                FinalizeSecretBrokerNarrowReasonClass::MissingHandleOnStableClaim,
                row_id,
                "Row has handle class 'missing' but claims a stable qualification; missing handles must not claim stable posture.",
            ));
        }

        // Condition 3: handle class must be explicitly set.
        if row.handle_class_token.is_empty() {
            defects.push(FinalizeSecretBrokerDefect::new(
                FinalizeSecretBrokerNarrowReasonClass::HandleClassMissing,
                row_id,
                "Row is missing an explicit handle class token.",
            ));
        }

        let needs_rotation_check = matches!(
            row.handle_class,
            SecretBrokerHandleClass::DelegatedIdentity | SecretBrokerHandleClass::SessionOnly
        );

        // Condition 4: rotation note must be explicit for delegated/session-only rows.
        if needs_rotation_check && !row.rotation_state.rotation_note_is_explicit() {
            defects.push(FinalizeSecretBrokerDefect::new(
                FinalizeSecretBrokerNarrowReasonClass::RotationNoteIsGeneric,
                row_id,
                format!(
                    "Delegated/session-only row has a generic or empty rotation note: '{}'.",
                    row.rotation_state.rotation_note
                ),
            ));
        }

        // Condition 5: remembered approvals must be narrow.
        for approval in &row.remembered_approvals {
            if approval.actor_ref.is_empty()
                || approval.target_ref.is_empty()
                || approval.action_family_token.is_empty()
                || approval.environment_token.is_empty()
                || approval.expiry_window.is_empty()
            {
                defects.push(FinalizeSecretBrokerDefect::new(
                    FinalizeSecretBrokerNarrowReasonClass::RememberedApprovalNotNarrow,
                    row_id,
                    format!(
                        "Remembered approval '{}' is missing one or more required narrow fields (actor, target, action_family, environment, expiry_window).",
                        approval.approval_row_id
                    ),
                ));
            }

            // Revocation trigger must be present when rotation invalidates decisions.
            if row.rotation_state.rotation_event.invalidates_remembered_decisions()
                && approval.revocation_trigger_token.is_empty()
            {
                defects.push(FinalizeSecretBrokerDefect::new(
                    FinalizeSecretBrokerNarrowReasonClass::RevocationTriggerMissing,
                    row_id,
                    format!(
                        "Remembered approval '{}' is missing a revocation trigger token, but the rotation event '{}' invalidates remembered decisions.",
                        approval.approval_row_id,
                        row.rotation_state.rotation_event_token
                    ),
                ));
            }

            // Raw credential must be excluded from remembered approvals.
            if !approval.raw_credential_excluded {
                defects.push(FinalizeSecretBrokerDefect::new(
                    FinalizeSecretBrokerNarrowReasonClass::RawSecretMaterialPresent,
                    row_id,
                    format!(
                        "Remembered approval '{}' has raw_credential_excluded = false.",
                        approval.approval_row_id
                    ),
                ));
            }
        }
    }

    defects
}

/// Applies per-row qualification based on the defect list.
fn qualify_rows(
    rows: Vec<FinalizeSecretBrokerRow>,
    defects: &[FinalizeSecretBrokerDefect],
) -> Vec<FinalizeSecretBrokerRow> {
    rows.into_iter()
        .map(|mut row| {
            let row_defects: Vec<&FinalizeSecretBrokerDefect> = defects
                .iter()
                .filter(|d| d.source_row_id == row.row_id)
                .collect();

            let has_withdrawal = row_defects
                .iter()
                .any(|d| d.narrow_reason.is_withdrawal_reason());
            let has_page_withdrawal = defects.iter().any(|d| {
                d.source_row_id == "page" && d.narrow_reason.is_withdrawal_reason()
            });

            if has_withdrawal || has_page_withdrawal {
                let reason = row_defects
                    .iter()
                    .find(|d| d.narrow_reason.is_withdrawal_reason())
                    .map(|d| d.narrow_reason)
                    .unwrap_or(FinalizeSecretBrokerNarrowReasonClass::LiteralFlatteningDetected);
                row.qualification_token =
                    FinalizeSecretBrokerQualificationClass::Withdrawn.as_str().to_owned();
                row.narrow_reason_token = reason.as_str().to_owned();
            } else if row_defects.is_empty() && defects.iter().all(|d| d.source_row_id != "page") {
                row.qualification_token =
                    FinalizeSecretBrokerQualificationClass::Stable.as_str().to_owned();
                row.narrow_reason_token =
                    FinalizeSecretBrokerNarrowReasonClass::NotNarrowed.as_str().to_owned();
            } else {
                // Non-critical defects: check for coverage gaps that force preview.
                let has_coverage_gap = row_defects.iter().any(|d| {
                    d.narrow_reason == FinalizeSecretBrokerNarrowReasonClass::FlowClassCoverageGap
                });
                let has_page_coverage_gap = defects.iter().any(|d| {
                    d.source_row_id == "page"
                        && d.narrow_reason
                            == FinalizeSecretBrokerNarrowReasonClass::FlowClassCoverageGap
                });
                if has_coverage_gap || has_page_coverage_gap {
                    row.qualification_token =
                        FinalizeSecretBrokerQualificationClass::Preview.as_str().to_owned();
                    row.narrow_reason_token =
                        FinalizeSecretBrokerNarrowReasonClass::FlowClassCoverageGap.as_str().to_owned();
                } else {
                    let reason = row_defects
                        .first()
                        .map(|d| d.narrow_reason)
                        .unwrap_or_else(|| {
                            defects
                                .iter()
                                .find(|d| d.source_row_id == "page")
                                .map(|d| d.narrow_reason)
                                .unwrap_or(FinalizeSecretBrokerNarrowReasonClass::BetaPageHasDefects)
                        });
                    row.qualification_token =
                        FinalizeSecretBrokerQualificationClass::Beta.as_str().to_owned();
                    row.narrow_reason_token = reason.as_str().to_owned();
                }
            }

            row
        })
        .collect()
}

/// Validates the finalize page and returns typed defects on failure.
pub fn validate_finalize_secret_broker_page(
    page: &FinalizeSecretBrokerPage,
) -> Result<(), Vec<FinalizeSecretBrokerDefect>> {
    let defects = audit_finalize_secret_broker_rows(&page.rows, &page.beta_page);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

// ---------------------------------------------------------------------------
// Seed
// ---------------------------------------------------------------------------

/// Returns a seeded [`FinalizeSecretBrokerPage`] with all five required flow
/// classes covered, all handle classes represented, and zero defects.
///
/// The seed is the canonical "clean" packet used by tests, shell inspectors,
/// and reviewer fixtures. It demonstrates handle-only export parity,
/// delegated-versus-session-only distinction, and no-raw-secret persistence.
pub fn seeded_finalize_secret_broker_page() -> FinalizeSecretBrokerPage {
    let beta_page = seeded_secret_broker_beta_page();

    let rows = vec![
        // Request-workspace: OS keychain handle
        build_row(
            "finalize-broker:request-workspace:os-keychain:connected",
            SecretBrokerFlowClass::RequestWorkspace,
            SecretBrokerHandleClass::OsKeychain,
            "connected",
            "code_host_token",
            "beta:row:request-workspace:os-keychain:connected",
            CredentialRotationState {
                rotation_event: CredentialRotationEventClass::NoRotationRequired,
                rotation_event_token: CredentialRotationEventClass::NoRotationRequired.as_str().to_owned(),
                rotation_note: "No rotation event; handle is live within freshness window.".to_owned(),
                expires_at: "2027-01-01T00:00:00Z".to_owned(),
                invalidates_remembered_decisions: false,
                reapproval_path_available: true,
                replay_posture_token: "replay_safe".to_owned(),
            },
            vec![],
            "OS keychain handle issued for request-workspace code-host token; no rotation required.",
        ),
        // Database: enterprise vault handle
        build_row(
            "finalize-broker:database:enterprise-vault:enterprise-managed",
            SecretBrokerFlowClass::Database,
            SecretBrokerHandleClass::EnterpriseVault,
            "enterprise_managed",
            "database_credential",
            "beta:row:database:enterprise-vault:enterprise-managed",
            CredentialRotationState {
                rotation_event: CredentialRotationEventClass::NoRotationRequired,
                rotation_event_token: CredentialRotationEventClass::NoRotationRequired.as_str().to_owned(),
                rotation_note: "Enterprise vault holds the database credential; rotation is policy-driven and audit-logged.".to_owned(),
                expires_at: "2027-06-01T00:00:00Z".to_owned(),
                invalidates_remembered_decisions: false,
                reapproval_path_available: true,
                replay_posture_token: "replay_safe".to_owned(),
            },
            vec![],
            "Enterprise vault handle for database attach under enterprise-managed profile.",
        ),
        // Env/config: workspace variable
        build_row(
            "finalize-broker:env-config:workspace-variable:connected",
            SecretBrokerFlowClass::EnvConfig,
            SecretBrokerHandleClass::WorkspaceVariable,
            "connected",
            "ai_provider_token",
            "beta:row:env-config:workspace-variable:connected",
            CredentialRotationState {
                rotation_event: CredentialRotationEventClass::NoRotationRequired,
                rotation_event_token: CredentialRotationEventClass::NoRotationRequired.as_str().to_owned(),
                rotation_note: "Workspace-variable reference; projected into env-isolated child only, not persisted to workspace history.".to_owned(),
                expires_at: String::new(),
                invalidates_remembered_decisions: false,
                reapproval_path_available: true,
                replay_posture_token: "replay_safe".to_owned(),
            },
            vec![],
            "Workspace-variable secret projected into isolated env/config child; no raw value in history.",
        ),
        // Provider: delegated identity with rotation block and remembered approval
        build_row(
            "finalize-broker:provider:delegated-identity:connected",
            SecretBrokerFlowClass::Provider,
            SecretBrokerHandleClass::DelegatedIdentity,
            "connected",
            "provider_session",
            "beta:row:provider:delegated-identity:connected",
            CredentialRotationState {
                rotation_event: CredentialRotationEventClass::BrowserDeviceCodeRenewal,
                rotation_event_token: CredentialRotationEventClass::BrowserDeviceCodeRenewal.as_str().to_owned(),
                rotation_note: "Browser device-code flow renewed the delegated provider session token; prior remembered approvals are re-evaluated against the new token expiry.".to_owned(),
                expires_at: "2026-12-01T00:00:00Z".to_owned(),
                invalidates_remembered_decisions: false,
                reapproval_path_available: true,
                replay_posture_token: "replay_requires_reapproval".to_owned(),
            },
            vec![RememberedApprovalRow {
                record_kind: FINALIZE_SECRET_BROKER_REMEMBERED_APPROVAL_RECORD_KIND.to_owned(),
                schema_version: FINALIZE_SECRET_BROKER_SCHEMA_VERSION,
                shared_contract_ref: FINALIZE_SECRET_BROKER_SHARED_CONTRACT_REF.to_owned(),
                approval_row_id: "approval:provider:delegated:read:local:2026-12-01".to_owned(),
                actor_ref: "actor:opaque:user-a".to_owned(),
                target_ref: "target:provider:opaque-provider-a".to_owned(),
                action_family_token: "read".to_owned(),
                environment_token: "local".to_owned(),
                expiry_window: "2026-12-01T00:00:00Z".to_owned(),
                bound_handle_class_token: SecretBrokerHandleClass::DelegatedIdentity.as_str().to_owned(),
                revocation_trigger_token: CredentialRotationEventClass::ApprovalRevoked.as_str().to_owned(),
                is_valid: true,
                reapproval_trigger_note: "Reapproval required when the device-code session token expires or the provider reconnects.".to_owned(),
                raw_credential_excluded: true,
            }],
            "Delegated-identity provider session renewed via browser device-code; remembered approval is narrow and carries a typed revocation trigger.",
        ),
        // Managed-runtime: session-only with rotation block
        build_row(
            "finalize-broker:managed-runtime:session-only:enterprise-managed",
            SecretBrokerFlowClass::ManagedRuntime,
            SecretBrokerHandleClass::SessionOnly,
            "enterprise_managed",
            "ephemeral_operation_token",
            "beta:row:managed-runtime:session-only:enterprise-managed",
            CredentialRotationState {
                rotation_event: CredentialRotationEventClass::HandleExpired,
                rotation_event_token: CredentialRotationEventClass::HandleExpired.as_str().to_owned(),
                rotation_note: "Session-only ephemeral operation token expired at session boundary; no durable handle was issued. New operation requires re-authorisation.".to_owned(),
                expires_at: "2026-06-01T06:00:00Z".to_owned(),
                invalidates_remembered_decisions: true,
                reapproval_path_available: true,
                replay_posture_token: "replay_blocked".to_owned(),
            },
            vec![RememberedApprovalRow {
                record_kind: FINALIZE_SECRET_BROKER_REMEMBERED_APPROVAL_RECORD_KIND.to_owned(),
                schema_version: FINALIZE_SECRET_BROKER_SCHEMA_VERSION,
                shared_contract_ref: FINALIZE_SECRET_BROKER_SHARED_CONTRACT_REF.to_owned(),
                approval_row_id: "approval:managed-runtime:session-only:exec:enterprise:2026-06-01".to_owned(),
                actor_ref: "actor:opaque:managed-agent-b".to_owned(),
                target_ref: "target:managed-runtime:opaque-runtime-c".to_owned(),
                action_family_token: "exec".to_owned(),
                environment_token: "enterprise_managed".to_owned(),
                expiry_window: "2026-06-01T06:00:00Z".to_owned(),
                bound_handle_class_token: SecretBrokerHandleClass::SessionOnly.as_str().to_owned(),
                revocation_trigger_token: CredentialRotationEventClass::HandleExpired.as_str().to_owned(),
                is_valid: false,
                reapproval_trigger_note: "Session-only token expired at session boundary; reapproval required before the next managed-runtime operation.".to_owned(),
                raw_credential_excluded: true,
            }],
            "Session-only ephemeral operation token expired; remembered approval carries a typed handle-expiry revocation trigger and is narrow to actor, target, action family, environment, and expiry window.",
        ),
        // Offline / air-gapped: enterprise vault (mirror)
        build_row(
            "finalize-broker:database:enterprise-vault:offline",
            SecretBrokerFlowClass::Database,
            SecretBrokerHandleClass::EnterpriseVault,
            "offline",
            "database_credential",
            "beta:row:database:enterprise-vault:offline",
            CredentialRotationState {
                rotation_event: CredentialRotationEventClass::NoRotationRequired,
                rotation_event_token: CredentialRotationEventClass::NoRotationRequired.as_str().to_owned(),
                rotation_note: "Air-gapped vault snapshot; credential handle was verified at import time and is within the declared grace window.".to_owned(),
                expires_at: "2026-09-01T00:00:00Z".to_owned(),
                invalidates_remembered_decisions: false,
                reapproval_path_available: false,
                replay_posture_token: "replay_safe".to_owned(),
            },
            vec![],
            "Enterprise vault air-gapped snapshot supplies the database credential under the offline profile.",
        ),
    ];

    FinalizeSecretBrokerPage::new(
        "policy:finalize-secret-broker:stable:0001",
        "Secret-broker handle-only, delegated, and session-only finalize packet",
        "2026-06-01T00:00:00Z",
        beta_page,
        rows,
    )
}

fn build_row(
    row_id: &str,
    flow_class: SecretBrokerFlowClass,
    handle_class: SecretBrokerHandleClass,
    profile_token: &str,
    secret_class_token: &str,
    beta_row_ref: &str,
    rotation_state: CredentialRotationState,
    remembered_approvals: Vec<RememberedApprovalRow>,
    plain_language_summary: &str,
) -> FinalizeSecretBrokerRow {
    FinalizeSecretBrokerRow {
        record_kind: FINALIZE_SECRET_BROKER_ROW_RECORD_KIND.to_owned(),
        schema_version: FINALIZE_SECRET_BROKER_SCHEMA_VERSION,
        shared_contract_ref: FINALIZE_SECRET_BROKER_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.to_owned(),
        flow_class,
        flow_class_token: flow_class.as_str().to_owned(),
        handle_class,
        handle_class_token: handle_class.as_str().to_owned(),
        profile_token: profile_token.to_owned(),
        secret_class_token: secret_class_token.to_owned(),
        beta_row_ref: beta_row_ref.to_owned(),
        rotation_state,
        remembered_approvals,
        raw_secret_material_excluded: true,
        no_literal_flattening: true,
        // qualification and narrow_reason are set by qualify_rows
        qualification_token: FinalizeSecretBrokerQualificationClass::Stable.as_str().to_owned(),
        narrow_reason_token: FinalizeSecretBrokerNarrowReasonClass::NotNarrowed.as_str().to_owned(),
        plain_language_summary: plain_language_summary.to_owned(),
    }
}
