//! Schema registry: the canonical catalog of setting definitions.
//!
//! Each `setting_id` maps to exactly one definition. Re-registering
//! a setting with a different shape is a bug; the registry returns
//! [`SchemaRegistryError::AlreadyRegistered`] rather than silently
//! overwriting.

use std::collections::BTreeMap;

use super::definition::SettingDefinition;
use super::restart::{LifecycleLabel, RestartPosture};
use super::scope::SettingScope;
use super::value::{SettingValue, SettingValueType};

/// Canonical catalog of setting definitions.
#[derive(Debug, Clone, Default)]
pub struct SchemaRegistry {
    definitions: BTreeMap<String, SettingDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaRegistryError {
    AlreadyRegistered { setting_id: String },
}

impl std::fmt::Display for SchemaRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyRegistered { setting_id } => {
                write!(f, "setting_id {setting_id:?} already registered")
            }
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
        self.definitions.insert(def.setting_id.clone(), def);
        Ok(())
    }

    /// Look up a definition by canonical id.
    pub fn definition(&self, setting_id: &str) -> Option<&SettingDefinition> {
        self.definitions.get(setting_id)
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
    /// by the M1 protected dogfood walks. The seed is intentionally
    /// tiny: it covers one setting per major value-type and one
    /// policy-narrowable setting so the resolver and the lock flow
    /// can be exercised end-to-end without a docs/UI dependency.
    pub fn with_seed_catalog() -> Self {
        let mut registry = Self::new();
        registry
            .register(SettingDefinition {
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
                is_machine_specific: false,
                is_policy_narrowable: false,
                summary: "Visual width of one tab in spaces.".to_owned(),
            })
            .expect("seed: editor.tab_size");
        registry
            .register(SettingDefinition {
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
                is_machine_specific: false,
                is_policy_narrowable: false,
                summary: "Run the configured formatter on save.".to_owned(),
            })
            .expect("seed: editor.format_on_save");
        registry
            .register(SettingDefinition {
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
                is_machine_specific: false,
                is_policy_narrowable: false,
                summary: "Active shell theme.".to_owned(),
            })
            .expect("seed: shell.theme");
        registry
            .register(SettingDefinition {
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
                is_machine_specific: false,
                is_policy_narrowable: true,
                summary: "Outbound AI provider egress policy.".to_owned(),
            })
            .expect("seed: security.ai.egress_policy");
        registry
            .register(SettingDefinition {
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
                is_machine_specific: true,
                is_policy_narrowable: false,
                summary: "Polling interval used when filesystem watchers are unavailable.".to_owned(),
            })
            .expect("seed: vfs.watcher.fallback_polling_ms");
        registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut registry = SchemaRegistry::new();
        let def = SettingDefinition {
            setting_id: "a.b".into(),
            value_type: SettingValueType::Boolean,
            default_value: SettingValue::Boolean(false),
            allowed_scopes: vec![SettingScope::BuiltInDefault],
            restart_posture: RestartPosture::NoRestart,
            lifecycle_label: LifecycleLabel::Stable,
            is_machine_specific: false,
            is_policy_narrowable: false,
            summary: String::new(),
        };
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
        assert!(registry.definition("security.ai.egress_policy").is_some());
        assert!(registry.definition("vfs.watcher.fallback_polling_ms").is_some());
        assert_eq!(registry.len(), 5);
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
}
