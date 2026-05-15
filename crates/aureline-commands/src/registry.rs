use std::collections::HashMap;
use std::fmt;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::descriptor::{CommandDescriptorRecord, CommandId, PolicyContext};
use crate::enablement::{
    evaluate_enablement as evaluate_enablement_impl, preflight as preflight_impl,
    CommandEnablementContext, DisabledReasonRecord, EnablementSnapshot, PreflightDecision,
};

const NO_PREVIEW_REQUIRED: &str = "no_preview_required";
const NO_APPROVAL_REQUIRED: &str = "no_approval_required";

/// Registry metadata that binds a high-effect command to its preview or
/// equivalent review lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandPreviewGateMetadata {
    /// Boundary record kind.
    pub record_kind: String,
    /// Schema version for this metadata block.
    pub schema_version: u32,
    /// Stable class for the gate or equivalent review lane.
    pub gate_class: String,
    /// Requirement class enforced by the owning lane.
    pub requirement_class: String,
    /// Review or disclosure surfaces that carry the gate.
    pub review_surface_refs: Vec<String>,
    /// Guard ref checked before the command mutates or exposes effects.
    pub apply_guard_ref: String,
    /// Recovery or revert posture exposed by the gate.
    pub revert_posture_class: String,
    /// Evidence refs emitted or required by the gate.
    pub evidence_ref_class_required: Vec<String>,
}

impl CommandPreviewGateMetadata {
    fn validate_minimal(&self) -> Result<(), &'static str> {
        if self.record_kind != "command_preview_gate_metadata" {
            return Err("preview gate record_kind must be command_preview_gate_metadata");
        }
        if self.schema_version != 1 {
            return Err("unsupported preview gate schema_version");
        }
        if self.gate_class.trim().is_empty() {
            return Err("preview gate gate_class must be non-empty");
        }
        if self.requirement_class.trim().is_empty() {
            return Err("preview gate requirement_class must be non-empty");
        }
        if self.review_surface_refs.is_empty() {
            return Err("preview gate review_surface_refs must be non-empty");
        }
        if self.apply_guard_ref.trim().is_empty() {
            return Err("preview gate apply_guard_ref must be non-empty");
        }
        if self.revert_posture_class.trim().is_empty() {
            return Err("preview gate revert_posture_class must be non-empty");
        }
        if self.evidence_ref_class_required.is_empty() {
            return Err("preview gate evidence_ref_class_required must be non-empty");
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum RegistryError {
    Json(serde_json::Error),
    InvalidSeed(&'static str),
    InvalidEntry {
        command_id: String,
        detail: &'static str,
    },
    DuplicateCommandId(String),
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(err) => write!(f, "failed to parse registry JSON: {err}"),
            Self::InvalidSeed(detail) => write!(f, "invalid registry seed: {detail}"),
            Self::InvalidEntry { command_id, detail } => {
                write!(f, "invalid registry entry {command_id}: {detail}")
            }
            Self::DuplicateCommandId(command_id) => write!(f, "duplicate command_id: {command_id}"),
        }
    }
}

impl std::error::Error for RegistryError {}

impl From<serde_json::Error> for RegistryError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

/// One command registry entry that embeds a command descriptor and adds registry-owned
/// metadata for discoverability, alias lifecycle, and cross-surface projections.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandRegistryEntryRecord {
    pub record_kind: String,
    pub command_registry_schema_version: u32,
    pub registry_entry_id: String,
    pub title: String,
    pub summary: String,
    pub namespace_class: String,
    pub descriptor: CommandDescriptorRecord,
    pub seed_enablement_snapshot: EnablementSnapshot,

    pub alias_records: Vec<serde_json::Value>,
    pub discoverability_record: serde_json::Value,
    pub automation_labels: Vec<String>,
    pub dominant_side_effect_class: String,
    #[serde(default)]
    pub preview_gate_metadata: Option<CommandPreviewGateMetadata>,
    pub current_keybinding_refs: Vec<serde_json::Value>,
    pub disabled_reason_records: Vec<DisabledReasonRecord>,
    pub origin_badge: serde_json::Value,
    pub target_badges: Vec<serde_json::Value>,
    pub diagnostic_projection_refs: serde_json::Value,
    pub preferred_surface_exposures: Vec<serde_json::Value>,
    pub machine_name_records: Vec<serde_json::Value>,
    pub policy_context: PolicyContext,
    pub redaction_class: String,
    pub minted_at: String,
}

impl CommandRegistryEntryRecord {
    fn validate_minimal(&self) -> Result<(), &'static str> {
        if self.record_kind != "command_registry_entry_record" {
            return Err("entry record_kind must be command_registry_entry_record");
        }
        if self.command_registry_schema_version != 1 {
            return Err("unsupported command_registry_schema_version");
        }
        self.descriptor.validate_minimal()?;
        if self.title.trim().is_empty() {
            return Err("title must be non-empty");
        }
        if let Some(metadata) = self.preview_gate_metadata.as_ref() {
            metadata.validate_minimal()?;
        }
        if self.destructive_or_external_effect_class().is_some()
            && !self.has_preview_or_gate_metadata()
        {
            return Err("destructive or external-effect entry must declare preview metadata");
        }
        Ok(())
    }

    pub fn command_id(&self) -> &CommandId {
        &self.descriptor.command_id
    }

    /// Returns the high-effect class that requires preview metadata, if any.
    pub fn destructive_or_external_effect_class(&self) -> Option<&'static str> {
        if matches!(
            self.descriptor.capability_scope_class.as_str(),
            "externally_visible_mutation"
                | "credential_or_secret_bearing"
                | "managed_workspace_control"
                | "policy_authoring_or_waiver"
        ) || matches!(
            self.dominant_side_effect_class.as_str(),
            "network_call" | "runs_process" | "remote_mutation" | "provider_visible_mutation"
        ) {
            return Some("external_effect");
        }

        if matches!(
            self.descriptor.capability_scope_class.as_str(),
            "recoverable_durable_mutation" | "irreversible_high_blast_mutation"
        ) || self.dominant_side_effect_class == "writes_files"
        {
            return Some("destructive_or_durable_mutation");
        }

        None
    }

    /// Returns true when the descriptor itself forces preview or approval.
    pub fn descriptor_requires_preview_or_approval(&self) -> bool {
        self.descriptor.preview_class != NO_PREVIEW_REQUIRED
            || self.descriptor.approval_posture_class != NO_APPROVAL_REQUIRED
    }

    /// Returns true when a high-effect command has preview metadata.
    pub fn has_preview_or_gate_metadata(&self) -> bool {
        self.descriptor_requires_preview_or_approval() || self.preview_gate_metadata.is_some()
    }

    /// Evaluates the enablement snapshot for this command entry.
    pub fn evaluate_enablement(&self, context: &CommandEnablementContext) -> EnablementSnapshot {
        evaluate_enablement_impl(
            &self.descriptor.client_scopes,
            &self.descriptor.lifecycle_state,
            self.descriptor.default_enablement_repair_hook_ref.as_ref(),
            &self.descriptor.typed_arguments,
            &self.seed_enablement_snapshot,
            &self.disabled_reason_records,
            context,
        )
    }

    /// Computes the preflight decision a surface should use before dispatch.
    pub fn preflight(&self, context: &CommandEnablementContext) -> PreflightDecision {
        preflight_impl(
            &self.descriptor.client_scopes,
            &self.descriptor.lifecycle_state,
            self.descriptor.default_enablement_repair_hook_ref.as_ref(),
            &self.descriptor.typed_arguments,
            &self.descriptor.preview_class,
            &self.descriptor.approval_posture_class,
            &self.seed_enablement_snapshot,
            &self.disabled_reason_records,
            context,
        )
    }
}

/// Seed manifest record for the canonical command registry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandRegistrySeedRecord {
    pub record_kind: String,
    pub command_registry_schema_version: u32,
    #[serde(default)]
    pub entries: Vec<CommandRegistryEntryRecord>,
}

impl CommandRegistrySeedRecord {
    fn validate_minimal(&self) -> Result<(), &'static str> {
        if self.record_kind != "command_registry_seed_record" {
            return Err("seed record_kind must be command_registry_seed_record");
        }
        if self.command_registry_schema_version != 1 {
            return Err("unsupported command_registry_schema_version");
        }
        if self.entries.is_empty() {
            return Err("seed entries must be non-empty");
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CommandRegistry {
    entries: Vec<CommandRegistryEntryRecord>,
    by_command_id: HashMap<CommandId, usize>,
}

impl CommandRegistry {
    pub fn from_seed(seed: CommandRegistrySeedRecord) -> Result<Self, RegistryError> {
        seed.validate_minimal()
            .map_err(RegistryError::InvalidSeed)?;

        let mut by_command_id: HashMap<CommandId, usize> = HashMap::new();
        let mut entries: Vec<CommandRegistryEntryRecord> = Vec::with_capacity(seed.entries.len());
        for entry in seed.entries {
            entry
                .validate_minimal()
                .map_err(|detail| RegistryError::InvalidEntry {
                    command_id: entry.descriptor.command_id.clone(),
                    detail,
                })?;

            let command_id = entry.descriptor.command_id.clone();
            if by_command_id.contains_key(&command_id) {
                return Err(RegistryError::DuplicateCommandId(command_id));
            }
            let idx = entries.len();
            by_command_id.insert(command_id, idx);
            entries.push(entry);
        }

        Ok(Self {
            entries,
            by_command_id,
        })
    }

    pub fn from_seed_json(seed_json: &str) -> Result<Self, RegistryError> {
        let seed: CommandRegistrySeedRecord = serde_json::from_str(seed_json)?;
        Self::from_seed(seed)
    }

    pub fn entries(&self) -> &[CommandRegistryEntryRecord] {
        &self.entries
    }

    pub fn get(&self, command_id: &str) -> Option<&CommandRegistryEntryRecord> {
        let idx = *self.by_command_id.get(command_id)?;
        self.entries.get(idx)
    }
}

const SEEDED_REGISTRY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/commands/command_registry_seed.yaml"
));

static SEEDED_REGISTRY: OnceLock<CommandRegistry> = OnceLock::new();

pub fn seeded_registry() -> &'static CommandRegistry {
    SEEDED_REGISTRY.get_or_init(|| {
        CommandRegistry::from_seed_json(SEEDED_REGISTRY_JSON)
            .expect("seeded command registry must parse and validate")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_registry_loads() {
        let registry = seeded_registry();
        assert_eq!(registry.entries().len(), 24);
        assert!(registry.get("cmd:workspace.open_folder").is_some());
        assert!(registry.get("cmd:command_palette.open").is_some());
        assert!(registry.get("cmd:explorer.toggle").is_some());
        assert!(registry.get("cmd:terminal.toggle").is_some());
        assert!(registry.get("cmd:editor.find").is_some());
        assert!(registry.get("cmd:editor.replace").is_some());
        assert!(registry.get("cmd:editor.save").is_some());
        assert!(registry.get("cmd:editor.copy").is_some());
        assert!(registry.get("cmd:editor.paste").is_some());
        assert!(registry.get("cmd:editor.cut").is_some());
        assert!(registry.get("cmd:editor.undo").is_some());
        assert!(registry.get("cmd:editor.redo").is_some());
        assert!(registry.get("cmd:editor.find_next").is_some());
        assert!(registry.get("cmd:editor.find_previous").is_some());
        assert!(registry.get("cmd:quick_open.toggle").is_some());
        assert!(registry.get("cmd:settings.open").is_some());
        assert!(registry.get("cmd:labs.open_wedge_inspector").is_some());
        assert!(registry.get("cmd:task.rerun_last").is_some());
        assert!(registry.get("cmd:test.rerun_last").is_some());
    }

    #[test]
    fn rejects_missing_required_descriptor_fields() {
        let bad = r#"{
          "record_kind":"command_registry_seed_record",
          "command_registry_schema_version":1,
          "entries":[
            {
              "record_kind":"command_registry_entry_record",
              "command_registry_schema_version":1,
              "registry_entry_id":"registry-entry:bad",
              "title":"Bad",
              "summary":"Bad",
              "namespace_class":"core",
              "descriptor":{
                "record_kind":"command_descriptor_record",
                "command_descriptor_schema_version":1,
                "command_revision_ref":"cmd-rev:bad:0",
                "canonical_verb":"bad.verb",
                "primary_label_ref":"label:bad",
                "typed_arguments":[],
                "capability_scope_class":"inert_metadata_only",
                "preview_class":"no_preview_required",
                "approval_posture_class":"no_approval_required",
                "ai_tool_surfacing_class":"not_ai_callable",
                "palette_visibility":"always_visible",
                "ui_slot_hints":[],
                "lifecycle_state":"stable",
                "support_class":"standard_support",
                "release_channel":"stable_channel",
                "declared_freshness_class":"authoritative_live",
                "client_scopes":[],
                "result_contract":{
                  "result_contract_class":"no_result_emitted",
                  "artifact_kind_ref":null,
                  "typed_value_shape_ref":null,
                  "evidence_ref_class_required":[]
                },
                "default_enablement_repair_hook_ref":null,
                "policy_context":{
                  "policy_epoch":"pe:0",
                  "trust_state":"trusted",
                  "execution_context_id":"exec:0"
                },
                "redaction_class":"metadata_safe_default",
                "minted_at":"2026-01-01T00:00:00Z"
              },
              "seed_enablement_snapshot":{
                "decision_class":"enabled",
                "disabled_reason_code":null,
                "repair_hook_ref":null
              }
            }
          ]
        }"#;
        let err = CommandRegistry::from_seed_json(bad).unwrap_err();
        match err {
            RegistryError::Json(_) => {}
            _ => panic!("expected JSON error for missing command_id, got {err:?}"),
        }
    }
}
