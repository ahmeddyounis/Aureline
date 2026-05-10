//! Setting definition row: stable id, value type, default, allowed
//! scopes, restart posture, and lifecycle.
//!
//! One row per canonical `setting_id`. Aliases redirect onto the
//! canonical id; alias retention is tracked by the schema-registry
//! seed and is out of scope for this slice.

use serde::{Deserialize, Serialize};

use super::restart::{LifecycleLabel, RestartPosture};
use super::scope::SettingScope;
use super::value::{SettingValue, SettingValueType};

/// One canonical setting definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingDefinition {
    /// Stable, machine-readable id (for example `editor.tab_size`).
    pub setting_id: String,
    /// Declared value-type and bounds. The resolver validates concrete
    /// values against this type before any write.
    pub value_type: SettingValueType,
    /// Built-in default value emitted at the `built_in_default` scope
    /// when no overlay is registered.
    pub default_value: SettingValue,
    /// Scopes that may carry a value for this setting. Writes to any
    /// other scope MUST be denied with `ScopeNotAllowed`.
    pub allowed_scopes: Vec<SettingScope>,
    /// What a consumer must do for the change to take effect.
    pub restart_posture: RestartPosture,
    /// Lifecycle label rendered as a badge by settings UI.
    pub lifecycle_label: LifecycleLabel,
    /// True for values that MUST NOT be carried across machines via
    /// optional sync (machine-local paths, GPU/process tuning, etc).
    pub is_machine_specific: bool,
    /// True when admin policy is allowed to narrow the allowed value
    /// set or pin the value. False values reject policy ceilings.
    pub is_policy_narrowable: bool,
    /// Short human-readable summary for inspectors.
    pub summary: String,
}

impl SettingDefinition {
    /// Returns true when `scope` is permitted as a write target for
    /// this setting (irrespective of any active policy lock).
    pub fn allows_scope(&self, scope: SettingScope) -> bool {
        self.allowed_scopes.iter().any(|s| *s == scope)
    }

    /// Returns true when `value` satisfies the declared value type
    /// and bounds.
    pub fn validate_value(&self, value: &SettingValue) -> Result<(), ValueValidationError> {
        if !value.matches_kind(&self.value_type) {
            return Err(ValueValidationError::TypeMismatch {
                expected_kind: self.value_type.kind_token(),
            });
        }
        match (&self.value_type, value) {
            (SettingValueType::Integer { min, max }, SettingValue::Integer(v)) => {
                if let Some(lo) = min {
                    if v < lo {
                        return Err(ValueValidationError::OutOfRange {
                            value: *v,
                            min: *min,
                            max: *max,
                        });
                    }
                }
                if let Some(hi) = max {
                    if v > hi {
                        return Err(ValueValidationError::OutOfRange {
                            value: *v,
                            min: *min,
                            max: *max,
                        });
                    }
                }
                Ok(())
            }
            (SettingValueType::Enum { allowed }, SettingValue::String(s)) => {
                if allowed.iter().any(|a| a == s) {
                    Ok(())
                } else {
                    Err(ValueValidationError::NotInEnum {
                        value: s.clone(),
                        allowed: allowed.clone(),
                    })
                }
            }
            _ => Ok(()),
        }
    }
}

/// Failure modes returned by [`SettingDefinition::validate_value`].
/// Surfaces MUST quote the typed reason; "validation failed" without
/// a reason is not enough.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueValidationError {
    TypeMismatch { expected_kind: &'static str },
    OutOfRange {
        value: i64,
        min: Option<i64>,
        max: Option<i64>,
    },
    NotInEnum {
        value: String,
        allowed: Vec<String>,
    },
}

impl std::fmt::Display for ValueValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeMismatch { expected_kind } => {
                write!(f, "value does not match declared kind {expected_kind}")
            }
            Self::OutOfRange { value, min, max } => {
                write!(f, "integer value {value} outside [{min:?}..={max:?}]")
            }
            Self::NotInEnum { value, allowed } => {
                write!(f, "value {value:?} not in allowed enum set {allowed:?}")
            }
        }
    }
}

impl std::error::Error for ValueValidationError {}
