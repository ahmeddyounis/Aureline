//! Effective-value record and shadow-chain row.

use serde::{Deserialize, Serialize};

use crate::schema::{RestartPosture, SettingScope, SettingValue};

use super::lock::{LockReason, LockState, WriteDenialReason, WriteIntent};

/// Relation a candidate has to the resolved winner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShadowRelation {
    /// Candidate is the resolved value (after any policy ceiling).
    Winner,
    /// Candidate was a valid ordinary candidate but lost to a higher
    /// ordinary scope.
    Shadowed,
    /// Candidate was the layered ordinary winner but a policy ceiling
    /// narrowed or pinned the value.
    Capped,
    /// Candidate is an active policy ceiling. The ceiling row stays
    /// visible whether or not it actually capped the layered winner.
    PolicyCeiling,
}

impl ShadowRelation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Winner => "winner",
            Self::Shadowed => "shadowed",
            Self::Capped => "capped",
            Self::PolicyCeiling => "policy_ceiling",
        }
    }
}

/// One row in the shadow chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShadowChainEntry {
    /// Scope that contributed or capped this row.
    pub scope: SettingScope,
    /// Human-readable label for the row's source.
    pub source_label: String,
    /// Redacted preview of the value at this scope.
    pub value_preview: String,
    /// True when this scope had a concrete value or policy ceiling.
    #[serde(default = "default_true")]
    pub value_present: bool,
    /// True when this row is the effective winner.
    #[serde(default)]
    pub winner: bool,
    /// Relation between this row and the resolved winner.
    pub relation: ShadowRelation,
}

const fn default_true() -> bool {
    true
}

/// Resolved effective value for a `(setting_id, target)` pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveValue {
    /// Canonical setting id.
    pub setting_id: String,
    /// Current effective value.
    pub value: SettingValue,
    /// Scope that supplied the effective value.
    pub winning_scope: SettingScope,
    /// Human-readable source label for the winning scope.
    pub source_label: String,
    /// Ordered contributing source chain.
    pub shadow_chain: Vec<ShadowChainEntry>,
    /// Current lock state after policy and capability evaluation.
    pub lock_state: LockState,
    /// Typed reason for [`Self::lock_state`].
    pub lock_reason: LockReason,
    /// Restart or reload posture declared by the setting definition.
    pub restart_posture: RestartPosture,
    /// True when an admin-policy ceiling intersected the layered
    /// winner. Informational; the lock_state already names the
    /// effect.
    pub policy_ceiling_active: bool,
}

impl EffectiveValue {
    /// True when the resolved value comes from an admin-policy
    /// ceiling rather than an ordinary scope.
    pub fn pinned_by_policy(&self) -> bool {
        matches!(self.winning_scope, SettingScope::AdminPolicyNarrowing)
    }
}

/// Capability-dependency state copied into an effective-setting record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveCapabilityDependency {
    /// Dependency kind token.
    pub kind: String,
    /// Stable required-state reference, when declared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_ref: Option<String>,
    /// True when the dependency is known to be satisfied.
    pub satisfied: bool,
    /// Redaction-safe observed state, when a caller supplied one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_state: Option<String>,
}

/// Control-stack trace attached to an effective-setting record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveControlStack {
    /// Short source tag for the winning layer.
    pub source_label: String,
    /// Lifecycle token copied from the definition row.
    pub lifecycle_label: String,
    /// Last time the control stack evaluated the setting, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_refresh_at: Option<String>,
    /// Expiry timestamp for experiments or policy TTLs, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Offline fallback posture for the active authority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_fallback: Option<String>,
    /// Opaque handle for an explanation view.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explain_why_ref: Option<String>,
    /// Authority class currently influencing the setting.
    pub control_authority: String,
    /// True when admin policy is capping or constraining the value.
    pub narrowing_ceiling_active: bool,
}

/// Provenance for the most recent write known to the resolver.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveLastWritten {
    /// Redaction-safe monotonic timestamp.
    pub at: String,
    /// Actor class that produced the value.
    pub actor_class: String,
    /// Opaque mutation-journal reference, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_journal_ref: Option<String>,
    /// Opaque rollback checkpoint reference, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
}

/// Canonical effective-setting record exported to UI, CLI, sync, policy,
/// support, and docs/help consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveSettingRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Envelope schema version for the effective-setting vocabulary.
    pub settings_schema_version: u32,
    /// Canonical setting id.
    pub setting_id: String,
    /// Current effective value or a redacted JSON summary.
    pub value: serde_json::Value,
    /// Scope token that supplied the effective value.
    pub resolved_scope: String,
    /// Human-readable source tag for the winning scope.
    pub source_label: String,
    /// Lifecycle token copied from the definition row.
    pub lifecycle_label: String,
    /// Ordered source chain in precedence order.
    pub shadow_chain: Vec<ShadowChainEntry>,
    /// Lock-state token.
    pub lock_state: String,
    /// Lock-reason token.
    pub lock_reason: String,
    /// Write-intent token for the setting's current write posture.
    pub write_intent: String,
    /// Denial-reason token, or `none` when not denied.
    pub write_denial_reason: String,
    /// Restart or reload posture token.
    pub restart_posture: String,
    /// Preview class token copied from the definition row.
    pub preview_class: String,
    /// Capability dependencies with resolver-observed state.
    pub capability_dependencies: Vec<EffectiveCapabilityDependency>,
    /// Control-stack trace for explanation surfaces.
    pub control_stack: EffectiveControlStack,
    /// Last-known write provenance.
    pub last_written: EffectiveLastWritten,
    /// Setting-definition schema version targeted by this record.
    pub schema_version: String,
    /// Redaction class copied from the definition row.
    pub redaction_class: String,
}

impl EffectiveSettingRecord {
    /// Returns true when the record is currently locked by policy.
    pub fn is_policy_locked(&self) -> bool {
        matches!(
            self.lock_state.as_str(),
            "policy_locked" | "policy_constrained"
        )
    }
}

/// Converts a write intent into the schema token used by exported records.
pub const fn write_intent_token(intent: WriteIntent) -> &'static str {
    intent.as_str()
}

/// Converts an optional denial reason into the schema token used by exported records.
pub fn write_denial_token(reason: Option<&WriteDenialReason>) -> &'static str {
    match reason {
        Some(WriteDenialReason::UnknownSetting { .. }) => "setting_unknown_at_registry",
        Some(WriteDenialReason::ScopeNotAllowed) => "scope_not_allowed_for_setting",
        Some(WriteDenialReason::ScopeBroadeningWouldWidenTrust) => {
            "scope_broadening_would_widen_trust"
        }
        Some(WriteDenialReason::PolicyLocked) => "policy_locked_value",
        Some(WriteDenialReason::PolicyConstrainedValue) => "policy_constrained_value",
        Some(WriteDenialReason::CapabilityDependencyUnmet) => "capability_dependency_unmet",
        Some(WriteDenialReason::PreviewRequiredNotAcknowledged) => {
            "preview_required_not_acknowledged"
        }
        Some(WriteDenialReason::RollbackCheckpointNotCreated) => "rollback_checkpoint_not_created",
        Some(WriteDenialReason::ApprovalTicketMissing) => "approval_ticket_missing",
        Some(WriteDenialReason::RestartRequiredNotAcknowledged) => {
            "restart_required_not_acknowledged"
        }
        Some(WriteDenialReason::ValidationFailed { .. }) => "validation_failed",
        Some(WriteDenialReason::RetiredSetting) => "setting_retired",
        Some(WriteDenialReason::ManagedModeOnly) => "managed_mode_only",
        Some(WriteDenialReason::ReadOnlySurface) => "read_only_surface",
        None => "none",
    }
}
