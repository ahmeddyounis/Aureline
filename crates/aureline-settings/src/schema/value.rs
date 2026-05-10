//! Setting value type and concrete value representation.
//!
//! The value-type vocabulary is intentionally small in this slice
//! (boolean, integer with optional bounds, string, enum). New value
//! shapes (object, tagged union, credential alias) are tracked under
//! the schema-registry seed and will land as additive-minor schema
//! bumps.

use serde::{Deserialize, Serialize};

/// Value-type discriminant carried by every setting definition. The
/// resolver validates concrete values against the declared type
/// before accepting any write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SettingValueType {
    Boolean,
    Integer {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<i64>,
    },
    /// Free-form string. Length bounds are intentionally not modelled
    /// here; the resolver does not invent string-length opinions.
    String,
    /// Enumerated string. The resolver MUST reject any value not in
    /// `allowed`.
    Enum {
        allowed: Vec<String>,
    },
}

impl SettingValueType {
    /// Returns the stable kind token used in exported records.
    pub const fn kind_token(&self) -> &'static str {
        match self {
            Self::Boolean => "boolean",
            Self::Integer { .. } => "integer",
            Self::String => "string",
            Self::Enum { .. } => "enum",
        }
    }
}

/// Concrete setting value. Round-trips losslessly through JSON via
/// the `serde_json` adapter on the resolver.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SettingValue {
    Boolean(bool),
    Integer(i64),
    String(String),
}

impl SettingValue {
    /// True when `self` is shape-compatible with `value_type`. Bound
    /// and enum-membership checks live separately to keep validation
    /// errors localized.
    pub fn matches_kind(&self, value_type: &SettingValueType) -> bool {
        matches!(
            (self, value_type),
            (Self::Boolean(_), SettingValueType::Boolean)
                | (Self::Integer(_), SettingValueType::Integer { .. })
                | (Self::String(_), SettingValueType::String)
                | (Self::String(_), SettingValueType::Enum { .. })
        )
    }

    /// Format the value as a short, export-safe preview string.
    /// `redaction_class` callers MUST apply their own redaction;
    /// this helper does NOT redact.
    pub fn preview(&self) -> String {
        match self {
            Self::Boolean(v) => v.to_string(),
            Self::Integer(v) => v.to_string(),
            Self::String(v) => v.clone(),
        }
    }

    /// Convert to a `serde_json::Value` for export packets.
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Self::Boolean(v) => serde_json::Value::Bool(*v),
            Self::Integer(v) => serde_json::Value::Number(serde_json::Number::from(*v)),
            Self::String(v) => serde_json::Value::String(v.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_kind_distinguishes_each_shape() {
        let bool_v = SettingValue::Boolean(true);
        let int_v = SettingValue::Integer(4);
        let str_v = SettingValue::String("foo".into());

        assert!(bool_v.matches_kind(&SettingValueType::Boolean));
        assert!(int_v.matches_kind(&SettingValueType::Integer {
            min: None,
            max: None
        }));
        assert!(str_v.matches_kind(&SettingValueType::String));
        assert!(str_v.matches_kind(&SettingValueType::Enum {
            allowed: vec!["foo".into(), "bar".into()]
        }));

        assert!(!bool_v.matches_kind(&SettingValueType::Integer {
            min: None,
            max: None
        }));
        assert!(!int_v.matches_kind(&SettingValueType::String));
    }
}
