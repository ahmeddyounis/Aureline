//! Effective-value record and shadow-chain row.

use serde::{Deserialize, Serialize};

use crate::schema::{RestartPosture, SettingScope, SettingValue};

use super::lock::{LockReason, LockState};

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
    pub scope: SettingScope,
    pub source_label: String,
    pub value_preview: String,
    pub relation: ShadowRelation,
}

/// Resolved effective value for a `(setting_id, target)` pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveValue {
    pub setting_id: String,
    pub value: SettingValue,
    pub winning_scope: SettingScope,
    pub source_label: String,
    pub shadow_chain: Vec<ShadowChainEntry>,
    pub lock_state: LockState,
    pub lock_reason: LockReason,
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
