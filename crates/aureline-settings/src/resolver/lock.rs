//! Lock-state, write-intent, and write-denial vocabulary.
//!
//! Surfaces MUST quote the typed reason. A disabled control without
//! a reason, or a denied write without a denial reason, is a bug.

use serde::{Deserialize, Serialize};

/// Lock-state of an effective setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockState {
    /// No lock; ordinary writes are permitted at allowed scopes.
    Open,
    /// Value is inherited from a lower scope (no override exists at
    /// the requested scope).
    Inherited,
    /// Admin policy pins the value to a single allowed value.
    PolicyLocked,
    /// Admin policy admits only a narrower allowed value set.
    PolicyConstrained,
    /// A declared capability dependency is not currently satisfied.
    CapabilityLocked,
    /// The setting cannot be read or written at the inspected scope.
    UnsupportedScope,
    /// Resolver can show a value but cannot safely mutate (degraded
    /// read-only).
    DegradedReadOnly,
}

impl LockState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Inherited => "inherited",
            Self::PolicyLocked => "policy_locked",
            Self::PolicyConstrained => "policy_constrained",
            Self::CapabilityLocked => "capability_locked",
            Self::UnsupportedScope => "unsupported_scope",
            Self::DegradedReadOnly => "degraded_read_only",
        }
    }
}

/// Reason describing why the lock-state is what it is. `None` is the
/// only valid reason when `LockState::Open`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockReason {
    None,
    Inherited,
    PolicyLocked,
    PolicyConstrainsAllowedSet,
    CapabilityDependencyUnmet,
    UnsupportedScope,
    DegradedReadOnly,
    SettingRetired,
    ManagedModeOnly,
}

impl LockReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Inherited => "inherited",
            Self::PolicyLocked => "policy_locked",
            Self::PolicyConstrainsAllowedSet => "policy_constrains_allowed_set",
            Self::CapabilityDependencyUnmet => "capability_dependency_unmet",
            Self::UnsupportedScope => "unsupported_scope",
            Self::DegradedReadOnly => "degraded_read_only",
            Self::SettingRetired => "setting_retired",
            Self::ManagedModeOnly => "managed_mode_only",
        }
    }
}

/// Verdict returned by [`super::engine::EffectiveSettingsResolver::attempt_write`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteIntent {
    /// Write is safe to apply immediately.
    Allowed,
    /// Write is allowed and requires the declared restart posture.
    AllowedWithRestart,
    /// Write is allowed only after a preview is acknowledged.
    AllowedWithPreview,
    /// Write is allowed after a rollback checkpoint exists.
    AllowedWithRollbackCheckpoint,
    /// Write is allowed after checkpoint and approval ticket exist.
    AllowedWithRollbackCheckpointAndApproval,
    /// Write is allowed only after an approval ticket exists.
    AllowedRequiresApprovalTicket,
    /// Write is refused and carries a typed denial reason.
    Denied,
}

impl WriteIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::AllowedWithRestart => "allowed_with_restart",
            Self::AllowedWithPreview => "allowed_with_preview",
            Self::AllowedWithRollbackCheckpoint => "allowed_with_rollback_checkpoint",
            Self::AllowedWithRollbackCheckpointAndApproval => {
                "allowed_with_rollback_checkpoint_and_approval"
            }
            Self::AllowedRequiresApprovalTicket => "allowed_requires_approval_ticket",
            Self::Denied => "denied",
        }
    }

    /// Returns true when the verdict admits a write after its named
    /// preview, checkpoint, approval, or restart requirement is met.
    pub const fn is_allowed(self) -> bool {
        !matches!(self, Self::Denied)
    }
}

/// Typed denial reason. The first matching reason wins; supporting
/// reasons stay visible in the shadow chain so support and migration
/// flows can explain what else would block the write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum WriteDenialReason {
    /// `setting_id` is not registered.
    UnknownSetting { setting_id: String },
    /// Target scope is not in the definition's `allowed_scopes`.
    ScopeNotAllowed,
    /// The proposed write would silently widen trust, egress, or
    /// authority beyond the selected scope.
    ScopeBroadeningWouldWidenTrust,
    /// Admin policy pins the value or refuses any write.
    PolicyLocked,
    /// Admin policy admits only a narrower value set; the proposed
    /// value is outside that set.
    PolicyConstrainedValue,
    /// A declared capability dependency is not currently satisfied.
    CapabilityDependencyUnmet,
    /// A preview-required setting has not been acknowledged.
    PreviewRequiredNotAcknowledged,
    /// A rollback-class setting has no checkpoint.
    RollbackCheckpointNotCreated,
    /// An approval-gated setting has no approval ticket.
    ApprovalTicketMissing,
    /// A restart-required setting has not been acknowledged.
    RestartRequiredNotAcknowledged,
    /// Value failed type / range / enum validation.
    ValidationFailed { detail: String },
    /// Setting is retired and refuses writes.
    RetiredSetting,
    /// Setting can only be written by a managed authority.
    ManagedModeOnly,
    /// Current surface can inspect but cannot mutate this setting.
    ReadOnlySurface,
}

impl WriteDenialReason {
    pub const fn code_token(&self) -> &'static str {
        match self {
            Self::UnknownSetting { .. } => "unknown_setting",
            Self::ScopeNotAllowed => "scope_not_allowed",
            Self::ScopeBroadeningWouldWidenTrust => "scope_broadening_would_widen_trust",
            Self::PolicyLocked => "policy_locked",
            Self::PolicyConstrainedValue => "policy_constrained_value",
            Self::CapabilityDependencyUnmet => "capability_dependency_unmet",
            Self::PreviewRequiredNotAcknowledged => "preview_required_not_acknowledged",
            Self::RollbackCheckpointNotCreated => "rollback_checkpoint_not_created",
            Self::ApprovalTicketMissing => "approval_ticket_missing",
            Self::RestartRequiredNotAcknowledged => "restart_required_not_acknowledged",
            Self::ValidationFailed { .. } => "validation_failed",
            Self::RetiredSetting => "retired_setting",
            Self::ManagedModeOnly => "managed_mode_only",
            Self::ReadOnlySurface => "read_only_surface",
        }
    }
}
