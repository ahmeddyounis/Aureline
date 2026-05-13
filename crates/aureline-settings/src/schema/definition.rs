//! Setting definition row: stable id, value type, default, allowed
//! scopes, migration metadata, restart posture, and lifecycle.
//!
//! One row per canonical `setting_id`. Aliases redirect onto the
//! canonical id; alias retention is tracked in the row so generated
//! settings UI, CLI inspection, support export, import, and migration
//! review all describe the same setting identity.

use serde::{Deserialize, Serialize};

use super::restart::{LifecycleLabel, RestartPosture};
use super::scope::SettingScope;
use super::value::{SettingValue, SettingValueType};

/// Preview and rollback posture declared by a setting definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewClass {
    /// The value can be applied without a separate preview.
    SafeApply,
    /// A change summary must be presented before apply.
    PreviewRequired,
    /// A rollback checkpoint is required before apply.
    RollbackCheckpointRequired,
    /// Both rollback checkpoint and approval are required before apply.
    RollbackCheckpointAndApprovalRequired,
    /// The setting can only be changed by a managed authority.
    ManagedActionOnly,
}

impl PreviewClass {
    /// Returns the stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeApply => "safe_apply",
            Self::PreviewRequired => "preview_required",
            Self::RollbackCheckpointRequired => "rollback_checkpoint_required",
            Self::RollbackCheckpointAndApprovalRequired => {
                "rollback_checkpoint_and_approval_required"
            }
            Self::ManagedActionOnly => "managed_action_only",
        }
    }

    /// Returns true when a change preview must be shown before apply.
    pub const fn requires_preview(self) -> bool {
        !matches!(self, Self::SafeApply)
    }

    /// Returns true when a rollback checkpoint is required before apply.
    pub const fn requires_checkpoint(self) -> bool {
        matches!(
            self,
            Self::RollbackCheckpointRequired | Self::RollbackCheckpointAndApprovalRequired
        )
    }

    /// Returns true when an approval ticket is required before apply.
    pub const fn requires_approval(self) -> bool {
        matches!(self, Self::RollbackCheckpointAndApprovalRequired)
    }
}

/// Redaction class applied when a setting value reaches an exportable surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// The value is safe to show and export as-is.
    None,
    /// The value may be shown in UI strings but is reviewed for export.
    UiStringOnly,
    /// The value body is redacted while preserving its structural shape.
    RedactValuePreserveShape,
    /// Only a class label may be exported.
    RedactToClassLabel,
    /// The value is excluded from export.
    ExcludeFromExport,
}

impl RedactionClass {
    /// Returns the stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::UiStringOnly => "ui_string_only",
            Self::RedactValuePreserveShape => "redact_value_preserve_shape",
            Self::RedactToClassLabel => "redact_to_class_label",
            Self::ExcludeFromExport => "exclude_from_export",
        }
    }
}

/// Sensitivity class used by inspector and support-export projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SensitivityClass {
    /// Ordinary non-sensitive preference.
    GeneralPreference,
    /// Machine-local or topology-specific preference.
    MachineLocal,
    /// Trust, AI, network, extension, route, or automation control.
    HighRiskControl,
    /// Credential-adjacent value represented by a brokered handle.
    CredentialReference,
}

impl SensitivityClass {
    /// Returns the stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GeneralPreference => "general_preference",
            Self::MachineLocal => "machine_local",
            Self::HighRiskControl => "high_risk_control",
            Self::CredentialReference => "credential_reference",
        }
    }
}

/// Direction for a migration alias.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AliasDirection {
    /// Legacy ids redirect to the canonical setting id.
    RedirectToCanonical,
}

impl AliasDirection {
    /// Returns the stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RedirectToCanonical => "redirect_to_canonical",
        }
    }
}

/// One legacy id that redirects to a canonical setting id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingAlias {
    /// Legacy setting id.
    pub from_id: String,
    /// Product version in which the alias began redirecting.
    pub since_version: String,
    /// Product version in which the alias became deprecated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_in_version: Option<String>,
    /// Product version in which the alias is scheduled for removal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removal_target_version: Option<String>,
    /// Alias direction. Only redirects to canonical ids are allowed.
    pub alias_direction: AliasDirection,
}

impl SettingAlias {
    /// Creates an active alias that redirects to the canonical setting id.
    pub fn active(from_id: impl Into<String>, since_version: impl Into<String>) -> Self {
        Self {
            from_id: from_id.into(),
            since_version: since_version.into(),
            deprecated_in_version: None,
            removal_target_version: None,
            alias_direction: AliasDirection::RedirectToCanonical,
        }
    }
}

/// Transform class used by a migration row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationTransformClass {
    /// The previous value shape is identical.
    Identity,
    /// An enum value set became narrower.
    NarrowEnum,
    /// An enum value set widened additively.
    WidenEnumAdditive,
    /// A field split into multiple fields.
    SplitField,
    /// Multiple fields merged into one field.
    MergeField,
    /// A field was renamed without changing value shape.
    RenameField,
    /// Only the default value changed.
    ChangeDefaultOnly,
}

impl MigrationTransformClass {
    /// Returns the stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::NarrowEnum => "narrow_enum",
            Self::WidenEnumAdditive => "widen_enum_additive",
            Self::SplitField => "split_field",
            Self::MergeField => "merge_field",
            Self::RenameField => "rename_field",
            Self::ChangeDefaultOnly => "change_default_only",
        }
    }
}

/// One versioned migration row for a setting value shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRule {
    /// Source schema or product version.
    pub from_version: String,
    /// Target schema or product version.
    pub to_version: String,
    /// Transform class applied by the migration.
    pub transform_class: MigrationTransformClass,
    /// True when the prior value cannot be represented exactly.
    pub is_lossy: bool,
    /// True when the migration can be reversed from a checkpoint.
    pub rollback_supported: bool,
}

/// Capability dependency kind declared by a setting definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityDependencyKind {
    /// A feature flag must be enabled.
    FeatureFlagRequired,
    /// A required identity mode must be active.
    IdentityModeRequired,
    /// A minimum trust state must be active.
    TrustStateMinimum,
    /// A minimum policy epoch must be present.
    PolicyEpochMinimum,
    /// A workspace capability must be available.
    WorkspaceCapability,
    /// An extension capability must be available.
    ExtensionCapability,
    /// A credential-handle class must be bound.
    CredentialHandleClass,
}

impl CapabilityDependencyKind {
    /// Returns the stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FeatureFlagRequired => "feature_flag_required",
            Self::IdentityModeRequired => "identity_mode_required",
            Self::TrustStateMinimum => "trust_state_minimum",
            Self::PolicyEpochMinimum => "policy_epoch_minimum",
            Self::WorkspaceCapability => "workspace_capability",
            Self::ExtensionCapability => "extension_capability",
            Self::CredentialHandleClass => "credential_handle_class",
        }
    }
}

/// One capability dependency declared by a setting definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityDependency {
    /// Dependency kind.
    pub kind: CapabilityDependencyKind,
    /// Stable required-state reference, such as a capability id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_ref: Option<String>,
}

impl CapabilityDependency {
    /// Builds a dependency row with an optional required-state reference.
    pub fn new(kind: CapabilityDependencyKind, required_ref: Option<String>) -> Self {
        Self { kind, required_ref }
    }

    /// Returns a deterministic key for capability-state lookup.
    pub fn key(&self) -> String {
        match &self.required_ref {
            Some(required_ref) => format!("{}:{required_ref}", self.kind.as_str()),
            None => self.kind.as_str().to_owned(),
        }
    }
}

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
    /// Preview, checkpoint, and approval posture for writes.
    pub preview_class: PreviewClass,
    /// Redaction posture for support/export surfaces.
    pub redaction_class: RedactionClass,
    /// Sensitivity posture used by inspectors and write previews.
    pub sensitivity_class: SensitivityClass,
    /// Legacy ids that redirect to this canonical setting.
    #[serde(default)]
    pub alias_set: Vec<SettingAlias>,
    /// Versioned value-shape migrations for this setting.
    #[serde(default)]
    pub migration_table: Vec<MigrationRule>,
    /// Capability dependencies that can lock or degrade this setting.
    #[serde(default)]
    pub capability_dependencies: Vec<CapabilityDependency>,
    /// Stable docs handle for the setting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_doc_ref: Option<String>,
    /// Evidence refs used by docs, support exports, and inspectors.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Governance decision ref for high-risk or policy-narrowable settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision_row_ref: Option<String>,
    /// First product version that shipped this setting id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since_version: Option<String>,
    /// Longer description for detail surfaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Guidance shown before applying risky changes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_guidance: Option<String>,
    /// True for values that MUST NOT be carried across machines via
    /// optional sync (machine-local paths, GPU/process tuning, etc).
    pub is_machine_specific: bool,
    /// True when optional sync carries this setting by default.
    pub is_synced_by_default: bool,
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
    TypeMismatch {
        expected_kind: &'static str,
    },
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
