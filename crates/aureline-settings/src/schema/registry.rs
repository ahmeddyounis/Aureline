//! Schema registry: the canonical catalog of setting definitions.
//!
//! Each `setting_id` maps to exactly one definition. Re-registering
//! a setting with a different shape is a bug; the registry returns
//! [`SchemaRegistryError::AlreadyRegistered`] rather than silently
//! overwriting.

use std::collections::{BTreeMap, BTreeSet};

use super::definition::{
    CapabilityDependency, CapabilityDependencyKind, MigrationRule, MigrationTransformClass,
    PreviewClass, RedactionClass, SensitivityClass, SettingAlias, SettingDefinition,
};
use super::restart::{LifecycleLabel, RestartPosture};
use super::scope::SettingScope;
use super::value::{SettingValue, SettingValueType};

macro_rules! seed_definition {
    (
        setting_id: $setting_id:expr,
        value_type: $value_type:expr,
        default_value: $default_value:expr,
        allowed_scopes: $allowed_scopes:expr,
        restart_posture: $restart_posture:expr,
        lifecycle_label: $lifecycle_label:expr,
        preview_class: $preview_class:expr,
        redaction_class: $redaction_class:expr,
        sensitivity_class: $sensitivity_class:expr,
        capability_dependencies: $capability_dependencies:expr,
        is_machine_specific: $is_machine_specific:expr,
        is_synced_by_default: $is_synced_by_default:expr,
        is_policy_narrowable: $is_policy_narrowable:expr,
        summary: $summary:expr $(,)?
    ) => {
        seed_definition_impl(
            $setting_id,
            $value_type,
            $default_value,
            $allowed_scopes,
            $restart_posture,
            $lifecycle_label,
            $preview_class,
            $redaction_class,
            $sensitivity_class,
            $capability_dependencies,
            $is_machine_specific,
            $is_synced_by_default,
            $is_policy_narrowable,
            $summary,
        )
    };
}

/// Canonical catalog of setting definitions.
#[derive(Debug, Clone, Default)]
pub struct SchemaRegistry {
    definitions: BTreeMap<String, SettingDefinition>,
    retired_setting_ids: BTreeSet<String>,
}

/// Errors returned by [`SchemaRegistry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaRegistryError {
    AlreadyRegistered {
        setting_id: String,
    },
    RetiredIdReused {
        setting_id: String,
    },
    RegisteredIdCannotBeReserved {
        setting_id: String,
    },
    AliasMatchesCanonicalId {
        setting_id: String,
        alias_id: String,
    },
    AliasAlreadyRegistered {
        alias_id: String,
        canonical_id: String,
    },
}

impl std::fmt::Display for SchemaRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyRegistered { setting_id } => {
                write!(f, "setting_id {setting_id:?} already registered")
            }
            Self::RetiredIdReused { setting_id } => {
                write!(
                    f,
                    "setting_id {setting_id:?} is retired and cannot be reused"
                )
            }
            Self::RegisteredIdCannotBeReserved { setting_id } => write!(
                f,
                "setting_id {setting_id:?} is already registered and cannot be reserved"
            ),
            Self::AliasMatchesCanonicalId {
                setting_id,
                alias_id,
            } => write!(
                f,
                "alias {alias_id:?} for setting_id {setting_id:?} matches a canonical id"
            ),
            Self::AliasAlreadyRegistered {
                alias_id,
                canonical_id,
            } => write!(
                f,
                "alias {alias_id:?} already redirects to canonical setting {canonical_id:?}"
            ),
        }
    }
}

impl std::error::Error for SchemaRegistryError {}

impl SchemaRegistry {
    /// Build an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a setting definition. Returns
    /// `Err(AlreadyRegistered)` if the same `setting_id` already
    /// resolves to another definition.
    pub fn register(&mut self, def: SettingDefinition) -> Result<(), SchemaRegistryError> {
        if self.definitions.contains_key(&def.setting_id) {
            return Err(SchemaRegistryError::AlreadyRegistered {
                setting_id: def.setting_id,
            });
        }
        if self.retired_setting_ids.contains(&def.setting_id) {
            return Err(SchemaRegistryError::RetiredIdReused {
                setting_id: def.setting_id,
            });
        }
        for alias in &def.alias_set {
            if alias.from_id == def.setting_id || self.definitions.contains_key(&alias.from_id) {
                return Err(SchemaRegistryError::AliasMatchesCanonicalId {
                    setting_id: def.setting_id,
                    alias_id: alias.from_id.clone(),
                });
            }
            if let Some(canonical_id) = self.canonical_for_alias(&alias.from_id) {
                return Err(SchemaRegistryError::AliasAlreadyRegistered {
                    alias_id: alias.from_id.clone(),
                    canonical_id: canonical_id.to_owned(),
                });
            }
        }
        self.definitions.insert(def.setting_id.clone(), def);
        Ok(())
    }

    /// Look up a definition by canonical id.
    pub fn definition(&self, setting_id: &str) -> Option<&SettingDefinition> {
        self.definitions.get(setting_id)
    }

    /// Look up a definition by canonical id or migration alias.
    pub fn resolve_definition(&self, setting_id_or_alias: &str) -> Option<&SettingDefinition> {
        self.definition(setting_id_or_alias).or_else(|| {
            self.definitions.values().find(|def| {
                def.alias_set
                    .iter()
                    .any(|alias| alias.from_id == setting_id_or_alias)
            })
        })
    }

    /// Returns the canonical id for a migration alias.
    pub fn canonical_for_alias(&self, alias_id: &str) -> Option<&str> {
        self.definitions.values().find_map(|def| {
            def.alias_set
                .iter()
                .any(|alias| alias.from_id == alias_id)
                .then_some(def.setting_id.as_str())
        })
    }

    /// Reserve a retired setting id so it cannot be registered again.
    pub fn reserve_retired_setting_id(
        &mut self,
        setting_id: impl Into<String>,
    ) -> Result<(), SchemaRegistryError> {
        let setting_id = setting_id.into();
        if self.definitions.contains_key(&setting_id) {
            return Err(SchemaRegistryError::RegisteredIdCannotBeReserved { setting_id });
        }
        self.retired_setting_ids.insert(setting_id);
        Ok(())
    }

    /// Iterate over canonical ids in deterministic order.
    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.definitions.keys().map(String::as_str)
    }

    /// Iterate over every registered definition in deterministic
    /// order. Useful for export and conformance harnesses.
    pub fn definitions(&self) -> impl Iterator<Item = &SettingDefinition> {
        self.definitions.values()
    }

    /// Number of registered settings.
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// True when no settings are registered.
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    /// Build a registry pre-populated with a small seed catalog used
    /// by the protected dogfood walks. The seed is intentionally
    /// tiny: it covers one setting per major value-type and one
    /// policy-narrowable setting so the resolver and the lock flow
    /// can be exercised end-to-end without a docs/UI dependency.
    pub fn with_seed_catalog() -> Self {
        let mut registry = Self::new();
        registry
            .register(seed_definition!(
                setting_id: "editor.tab_size".to_owned(),
                value_type: SettingValueType::Integer {
                    min: Some(1),
                    max: Some(16),
                },
                default_value: SettingValue::Integer(4),
                allowed_scopes: vec![
                    SettingScope::BuiltInDefault,
                    SettingScope::ChannelOrExperimentDefault,
                    SettingScope::ImportedProfileDefault,
                    SettingScope::UserGlobal,
                    SettingScope::Workspace,
                    SettingScope::FolderOrModuleOverride,
                    SettingScope::LanguageOverride,
                    SettingScope::SessionOverride,
                ],
                restart_posture: RestartPosture::NoRestart,
                lifecycle_label: LifecycleLabel::Stable,
                preview_class: PreviewClass::SafeApply,
                redaction_class: RedactionClass::None,
                sensitivity_class: SensitivityClass::GeneralPreference,
                capability_dependencies: Vec::new(),
                is_machine_specific: false,
                is_synced_by_default: true,
                is_policy_narrowable: false,
                summary: "Visual width of one tab in spaces.".to_owned(),
            ))
            .expect("seed: editor.tab_size");
        registry
            .register(seed_definition!(
                setting_id: "editor.format_on_save".to_owned(),
                value_type: SettingValueType::Boolean,
                default_value: SettingValue::Boolean(false),
                allowed_scopes: vec![
                    SettingScope::BuiltInDefault,
                    SettingScope::ImportedProfileDefault,
                    SettingScope::UserGlobal,
                    SettingScope::Workspace,
                    SettingScope::FolderOrModuleOverride,
                    SettingScope::LanguageOverride,
                    SettingScope::SessionOverride,
                ],
                restart_posture: RestartPosture::NoRestart,
                lifecycle_label: LifecycleLabel::Stable,
                preview_class: PreviewClass::SafeApply,
                redaction_class: RedactionClass::None,
                sensitivity_class: SensitivityClass::GeneralPreference,
                capability_dependencies: Vec::new(),
                is_machine_specific: false,
                is_synced_by_default: true,
                is_policy_narrowable: false,
                summary: "Run the configured formatter on save.".to_owned(),
            ))
            .expect("seed: editor.format_on_save");
        registry
            .register(seed_definition!(
                setting_id: "shell.theme".to_owned(),
                value_type: SettingValueType::Enum {
                    allowed: vec!["light".into(), "dark".into(), "auto".into()],
                },
                default_value: SettingValue::String("auto".into()),
                allowed_scopes: vec![
                    SettingScope::BuiltInDefault,
                    SettingScope::ImportedProfileDefault,
                    SettingScope::UserGlobal,
                    SettingScope::MachineSpecific,
                    SettingScope::Workspace,
                    SettingScope::SessionOverride,
                ],
                restart_posture: RestartPosture::NoRestart,
                lifecycle_label: LifecycleLabel::Stable,
                preview_class: PreviewClass::SafeApply,
                redaction_class: RedactionClass::None,
                sensitivity_class: SensitivityClass::GeneralPreference,
                capability_dependencies: Vec::new(),
                is_machine_specific: false,
                is_synced_by_default: true,
                is_policy_narrowable: false,
                summary: "Active shell theme.".to_owned(),
            ))
            .expect("seed: shell.theme");
        registry
            .register(seed_definition!(
                setting_id: "ui.theme".to_owned(),
                value_type: SettingValueType::Enum {
                    allowed: vec!["light".into(), "dark".into(), "system".into()],
                },
                default_value: SettingValue::String("dark".into()),
                allowed_scopes: vec![
                    SettingScope::BuiltInDefault,
                    SettingScope::ImportedProfileDefault,
                    SettingScope::UserGlobal,
                    SettingScope::MachineSpecific,
                    SettingScope::Workspace,
                    SettingScope::SessionOverride,
                    SettingScope::AdminPolicyNarrowing,
                ],
                restart_posture: RestartPosture::NoRestart,
                lifecycle_label: LifecycleLabel::Stable,
                preview_class: PreviewClass::SafeApply,
                redaction_class: RedactionClass::None,
                sensitivity_class: SensitivityClass::GeneralPreference,
                capability_dependencies: Vec::new(),
                is_machine_specific: false,
                is_synced_by_default: true,
                is_policy_narrowable: true,
                summary: "Active UI theme mode.".to_owned(),
            ))
            .expect("seed: ui.theme");
        registry
            .register(seed_definition!(
                setting_id: "ui.density".to_owned(),
                value_type: SettingValueType::Enum {
                    allowed: vec!["compact".into(), "comfortable".into(), "spacious".into()],
                },
                default_value: SettingValue::String("comfortable".into()),
                allowed_scopes: vec![
                    SettingScope::BuiltInDefault,
                    SettingScope::ImportedProfileDefault,
                    SettingScope::UserGlobal,
                    SettingScope::MachineSpecific,
                    SettingScope::Workspace,
                    SettingScope::SessionOverride,
                    SettingScope::AdminPolicyNarrowing,
                ],
                restart_posture: RestartPosture::NoRestart,
                lifecycle_label: LifecycleLabel::Stable,
                preview_class: PreviewClass::SafeApply,
                redaction_class: RedactionClass::None,
                sensitivity_class: SensitivityClass::GeneralPreference,
                capability_dependencies: Vec::new(),
                is_machine_specific: false,
                is_synced_by_default: true,
                is_policy_narrowable: true,
                summary: "Active UI density mode.".to_owned(),
            ))
            .expect("seed: ui.density");
        registry
            .register(seed_definition!(
                setting_id: "ui.motion".to_owned(),
                value_type: SettingValueType::Enum {
                    allowed: vec!["full".into(), "reduced".into(), "none".into()],
                },
                default_value: SettingValue::String("full".into()),
                allowed_scopes: vec![
                    SettingScope::BuiltInDefault,
                    SettingScope::ImportedProfileDefault,
                    SettingScope::UserGlobal,
                    SettingScope::MachineSpecific,
                    SettingScope::Workspace,
                    SettingScope::SessionOverride,
                    SettingScope::AdminPolicyNarrowing,
                ],
                restart_posture: RestartPosture::NoRestart,
                lifecycle_label: LifecycleLabel::Stable,
                preview_class: PreviewClass::SafeApply,
                redaction_class: RedactionClass::None,
                sensitivity_class: SensitivityClass::GeneralPreference,
                capability_dependencies: Vec::new(),
                is_machine_specific: false,
                is_synced_by_default: true,
                is_policy_narrowable: true,
                summary: "Active UI motion mode.".to_owned(),
            ))
            .expect("seed: ui.motion");
        registry
            .register(seed_definition!(
                setting_id: "shell.labs.wedge_inspector_enabled".to_owned(),
                value_type: SettingValueType::Boolean,
                default_value: SettingValue::Boolean(false),
                allowed_scopes: vec![
                    SettingScope::BuiltInDefault,
                    SettingScope::UserGlobal,
                    SettingScope::Workspace,
                    SettingScope::SessionOverride,
                    SettingScope::AdminPolicyNarrowing,
                ],
                restart_posture: RestartPosture::NoRestart,
                lifecycle_label: LifecycleLabel::Experimental,
                preview_class: PreviewClass::PreviewRequired,
                redaction_class: RedactionClass::None,
                sensitivity_class: SensitivityClass::GeneralPreference,
                capability_dependencies: vec![CapabilityDependency::new(
                    CapabilityDependencyKind::FeatureFlagRequired,
                    Some("labs.wedge_inspector".to_owned()),
                )],
                is_machine_specific: false,
                is_synced_by_default: false,
                is_policy_narrowable: true,
                summary: "Enable the Labs wedge inspector overlay.".to_owned(),
            ))
            .expect("seed: shell.labs.wedge_inspector_enabled");
        registry
            .register(
                seed_definition!(
                    setting_id: "security.ai.egress_policy".to_owned(),
                    value_type: SettingValueType::Enum {
                        allowed: vec![
                            "any_hosted_provider".into(),
                            "approved_hosted_providers_only".into(),
                            "disabled".into(),
                        ],
                    },
                    default_value: SettingValue::String("disabled".into()),
                    allowed_scopes: vec![
                        SettingScope::BuiltInDefault,
                        SettingScope::UserGlobal,
                        SettingScope::Workspace,
                        SettingScope::AdminPolicyNarrowing,
                    ],
                    restart_posture: RestartPosture::RestartExtensions,
                    lifecycle_label: LifecycleLabel::Stable,
                    preview_class: PreviewClass::RollbackCheckpointAndApprovalRequired,
                    redaction_class: RedactionClass::UiStringOnly,
                    sensitivity_class: SensitivityClass::HighRiskControl,
                    capability_dependencies: vec![CapabilityDependency::new(
                        CapabilityDependencyKind::IdentityModeRequired,
                        Some("managed_convenience".to_owned()),
                    )],
                    is_machine_specific: false,
                    is_synced_by_default: false,
                    is_policy_narrowable: true,
                    summary: "Outbound AI provider egress policy.".to_owned(),
                )
                .with_alias(SettingAlias::active(
                    "ai.network.egress_policy",
                    "0.0.0-alpha",
                ))
                .with_migration(MigrationRule {
                    from_version: "0.0.0-alpha".to_owned(),
                    to_version: "0.0.0".to_owned(),
                    transform_class: MigrationTransformClass::RenameField,
                    is_lossy: false,
                    rollback_supported: true,
                }),
            )
            .expect("seed: security.ai.egress_policy");
        registry
            .register(seed_definition!(
                setting_id: "vfs.watcher.fallback_polling_ms".to_owned(),
                value_type: SettingValueType::Integer {
                    min: Some(100),
                    max: Some(60_000),
                },
                default_value: SettingValue::Integer(2_500),
                allowed_scopes: vec![
                    SettingScope::BuiltInDefault,
                    SettingScope::MachineSpecific,
                    SettingScope::Workspace,
                ],
                restart_posture: RestartPosture::ReloadWorkspace,
                lifecycle_label: LifecycleLabel::Stable,
                preview_class: PreviewClass::SafeApply,
                redaction_class: RedactionClass::None,
                sensitivity_class: SensitivityClass::MachineLocal,
                capability_dependencies: vec![CapabilityDependency::new(
                    CapabilityDependencyKind::WorkspaceCapability,
                    Some("workspace.filesystem_watcher".to_owned()),
                )],
                is_machine_specific: true,
                is_synced_by_default: false,
                is_policy_narrowable: false,
                summary: "Polling interval used when filesystem watchers are unavailable."
                    .to_owned(),
            ))
            .expect("seed: vfs.watcher.fallback_polling_ms");
        registry
    }
}

#[allow(clippy::too_many_arguments)]
fn seed_definition_impl(
    setting_id: String,
    value_type: SettingValueType,
    default_value: SettingValue,
    allowed_scopes: Vec<SettingScope>,
    restart_posture: RestartPosture,
    lifecycle_label: LifecycleLabel,
    preview_class: PreviewClass,
    redaction_class: RedactionClass,
    sensitivity_class: SensitivityClass,
    capability_dependencies: Vec<CapabilityDependency>,
    is_machine_specific: bool,
    is_synced_by_default: bool,
    is_policy_narrowable: bool,
    summary: String,
) -> SettingDefinition {
    SettingDefinition {
        help_doc_ref: Some(format!("docs:settings:{setting_id}")),
        evidence_refs: vec![
            "docs/settings/schema_registry_seed.md".to_owned(),
            "docs/settings/precedence_lock_and_write_scope_contract.md".to_owned(),
        ],
        since_version: Some("0.0.0-alpha".to_owned()),
        description: Some(summary.clone()),
        change_guidance: preview_class.requires_preview().then(|| {
            "Review the destination, source chain, and rollback posture before apply.".to_owned()
        }),
        decision_row_ref: preview_class
            .requires_checkpoint()
            .then(|| "decision:settings:high_risk_write_preview".to_owned()),
        setting_id,
        value_type,
        default_value,
        allowed_scopes,
        restart_posture,
        lifecycle_label,
        preview_class,
        redaction_class,
        sensitivity_class,
        alias_set: Vec::new(),
        migration_table: Vec::new(),
        capability_dependencies,
        is_machine_specific,
        is_synced_by_default,
        is_policy_narrowable,
        summary,
    }
}

trait SeedDefinitionExt {
    fn with_alias(self, alias: SettingAlias) -> Self;
    fn with_migration(self, migration: MigrationRule) -> Self;
}

impl SeedDefinitionExt for SettingDefinition {
    fn with_alias(mut self, alias: SettingAlias) -> Self {
        self.alias_set.push(alias);
        self
    }

    fn with_migration(mut self, migration: MigrationRule) -> Self {
        self.migration_table.push(migration);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut registry = SchemaRegistry::new();
        let def = seed_definition!(
            setting_id: "a.b".into(),
            value_type: SettingValueType::Boolean,
            default_value: SettingValue::Boolean(false),
            allowed_scopes: vec![SettingScope::BuiltInDefault],
            restart_posture: RestartPosture::NoRestart,
            lifecycle_label: LifecycleLabel::Stable,
            preview_class: PreviewClass::SafeApply,
            redaction_class: RedactionClass::None,
            sensitivity_class: SensitivityClass::GeneralPreference,
            capability_dependencies: Vec::new(),
            is_machine_specific: false,
            is_synced_by_default: false,
            is_policy_narrowable: false,
            summary: "Test setting.".to_owned(),
        );
        registry.register(def.clone()).unwrap();
        assert!(matches!(
            registry.register(def),
            Err(SchemaRegistryError::AlreadyRegistered { .. })
        ));
    }

    #[test]
    fn seed_catalog_registers_expected_settings() {
        let registry = SchemaRegistry::with_seed_catalog();
        assert!(registry.definition("editor.tab_size").is_some());
        assert!(registry.definition("editor.format_on_save").is_some());
        assert!(registry.definition("shell.theme").is_some());
        assert!(registry.definition("ui.theme").is_some());
        assert!(registry.definition("ui.density").is_some());
        assert!(registry.definition("ui.motion").is_some());
        assert!(registry
            .definition("shell.labs.wedge_inspector_enabled")
            .is_some());
        assert!(registry.definition("security.ai.egress_policy").is_some());
        assert!(registry
            .definition("vfs.watcher.fallback_polling_ms")
            .is_some());
        assert_eq!(registry.len(), 9);
    }

    #[test]
    fn seed_catalog_marks_machine_specific_setting() {
        let registry = SchemaRegistry::with_seed_catalog();
        let def = registry
            .definition("vfs.watcher.fallback_polling_ms")
            .expect("seed");
        assert!(def.is_machine_specific);
        assert!(!def.allows_scope(SettingScope::UserGlobal));
        assert!(def.allows_scope(SettingScope::MachineSpecific));
    }

    #[test]
    fn seed_catalog_marks_policy_narrowable_setting() {
        let registry = SchemaRegistry::with_seed_catalog();
        let def = registry
            .definition("security.ai.egress_policy")
            .expect("seed");
        assert!(def.is_policy_narrowable);
        assert!(def.allows_scope(SettingScope::AdminPolicyNarrowing));
    }

    #[test]
    fn retired_setting_id_cannot_be_reused() {
        let mut registry = SchemaRegistry::new();
        registry
            .reserve_retired_setting_id("legacy.removed")
            .unwrap();
        let def = seed_definition!(
            setting_id: "legacy.removed".into(),
            value_type: SettingValueType::Boolean,
            default_value: SettingValue::Boolean(false),
            allowed_scopes: vec![SettingScope::BuiltInDefault],
            restart_posture: RestartPosture::NoRestart,
            lifecycle_label: LifecycleLabel::Stable,
            preview_class: PreviewClass::SafeApply,
            redaction_class: RedactionClass::None,
            sensitivity_class: SensitivityClass::GeneralPreference,
            capability_dependencies: Vec::new(),
            is_machine_specific: false,
            is_synced_by_default: false,
            is_policy_narrowable: false,
            summary: "Retired id reuse fixture.".to_owned(),
        );
        assert!(matches!(
            registry.register(def),
            Err(SchemaRegistryError::RetiredIdReused { .. })
        ));
    }

    #[test]
    fn seed_catalog_exposes_alias_metadata() {
        let registry = SchemaRegistry::with_seed_catalog();
        assert_eq!(
            registry.canonical_for_alias("ai.network.egress_policy"),
            Some("security.ai.egress_policy")
        );
        let def = registry
            .resolve_definition("ai.network.egress_policy")
            .expect("alias resolves");
        assert_eq!(def.setting_id, "security.ai.egress_policy");
        assert!(!def.migration_table.is_empty());
        assert_eq!(
            def.preview_class,
            PreviewClass::RollbackCheckpointAndApprovalRequired
        );
    }
}
