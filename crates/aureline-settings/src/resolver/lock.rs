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
    UnsupportedScope,
    DegradedReadOnly,
}

impl LockReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Inherited => "inherited",
            Self::PolicyLocked => "policy_locked",
            Self::PolicyConstrainsAllowedSet => "policy_constrains_allowed_set",
            Self::UnsupportedScope => "unsupported_scope",
            Self::DegradedReadOnly => "degraded_read_only",
        }
    }
}

/// Verdict returned by [`super::engine::EffectiveSettingsResolver::attempt_write`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteIntent {
    Allowed,
    Denied,
}

impl WriteIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::Denied => "denied",
        }
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
    /// Admin policy pins the value or refuses any write.
    PolicyLocked,
    /// Admin policy admits only a narrower value set; the proposed
    /// value is outside that set.
    PolicyConstrainedValue,
    /// Value failed type / range / enum validation.
    ValidationFailed { detail: String },
    /// Setting is retired and refuses writes.
    RetiredSetting,
}

impl WriteDenialReason {
    pub const fn code_token(&self) -> &'static str {
        match self {
            Self::UnknownSetting { .. } => "unknown_setting",
            Self::ScopeNotAllowed => "scope_not_allowed",
            Self::PolicyLocked => "policy_locked",
            Self::PolicyConstrainedValue => "policy_constrained_value",
            Self::ValidationFailed { .. } => "validation_failed",
            Self::RetiredSetting => "retired_setting",
        }
    }
}
