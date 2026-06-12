use std::collections::HashMap;
use std::fmt;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::descriptor::{CommandAlias, CommandDescriptorRecord, CommandId, PolicyContext};
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

/// Public command contract projected from one registry entry.
///
/// The projection normalizes descriptor-owned fields with registry-owned
/// discoverability and alias lifecycle metadata so consumers can read one
/// object when checking command identity, schemas, origin, enablement,
/// automation, and result packet posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandDescriptorPublicContractRecord {
    /// Boundary record kind.
    pub record_kind: String,
    /// Schema version for this public projection.
    pub schema_version: u32,
    /// Stable canonical command id.
    pub command_id: CommandId,
    /// Descriptor revision pinned for replay and support joins.
    pub command_revision_ref: String,
    /// Dotted machine verb used by CLI, AI, recipes, and headless callers.
    pub canonical_verb: String,
    /// Category refs used by palette, docs, onboarding, and support search.
    pub category_refs: Vec<String>,
    /// Origin class that surfaces disclose when source affects trust or support.
    pub origin_class: String,
    /// Schema ref for argument/provenance-bearing invocation sessions.
    pub invocation_schema_ref: String,
    /// Schema ref for structured command result packets.
    pub result_schema_ref: String,
    /// Descriptor lifecycle state.
    pub lifecycle_state: String,
    /// Capability class that drives preview, approval, policy, and audit posture.
    pub capability_scope_class: String,
    /// Enablement rule refs or disabled-reason refs read by every surface.
    pub enablement_rule_refs: Vec<String>,
    /// Discoverability record and projection refs shared by command surfaces.
    pub discoverability_record_refs: Vec<String>,
    /// Automation labels such as `recipe_safe`, `headless_safe`, or `ui_only`.
    pub automation_labels: Vec<String>,
    /// Alias set normalized with lifecycle and canonical-resolution metadata.
    pub aliases: Vec<CommandAlias>,
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
        self.validate_public_command_contract()?;
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

    /// Projects the stable command descriptor contract for this entry.
    pub fn public_contract(&self) -> CommandDescriptorPublicContractRecord {
        let category_refs = if self.descriptor.category_refs.is_empty() {
            string_array_field(&self.discoverability_record, "category_refs")
        } else {
            self.descriptor.category_refs.clone()
        };
        let discoverability_record_refs = if self.descriptor.discoverability_record_refs.is_empty()
        {
            discoverability_refs_from_record(&self.discoverability_record)
        } else {
            self.descriptor.discoverability_record_refs.clone()
        };
        let automation_labels = if self.descriptor.automation_labels.is_empty() {
            self.automation_labels.clone()
        } else {
            self.descriptor.automation_labels.clone()
        };

        CommandDescriptorPublicContractRecord {
            record_kind: "command_descriptor_public_contract_record".to_string(),
            schema_version: 1,
            command_id: self.descriptor.command_id.clone(),
            command_revision_ref: self.descriptor.command_revision_ref.clone(),
            canonical_verb: self.descriptor.canonical_verb.clone(),
            category_refs,
            origin_class: self
                .descriptor
                .origin
                .as_ref()
                .map(|origin| origin.origin_class.clone())
                .or_else(|| string_field(&self.origin_badge, "badge_class"))
                .unwrap_or_else(|| self.namespace_class.clone()),
            invocation_schema_ref: self
                .descriptor
                .invocation_schema_ref
                .clone()
                .unwrap_or_else(|| {
                    "schemas/commands/command_invocation_session.schema.json".to_string()
                }),
            result_schema_ref: self
                .descriptor
                .result_schema_ref
                .clone()
                .unwrap_or_else(|| {
                    "schemas/commands/command_result_packet.schema.json".to_string()
                }),
            lifecycle_state: self.descriptor.lifecycle_state.clone(),
            capability_scope_class: self.descriptor.capability_scope_class.clone(),
            enablement_rule_refs: if self.descriptor.enablement_rule_refs.is_empty() {
                let mut refs = self
                    .disabled_reason_records
                    .iter()
                    .map(|record| record.explanation_ref.clone())
                    .collect::<Vec<_>>();
                if refs.is_empty() {
                    refs.push(format!(
                        "enablement-rule:{}:seed_enablement_snapshot",
                        self.descriptor.canonical_verb
                    ));
                }
                refs
            } else {
                self.descriptor.enablement_rule_refs.clone()
            },
            discoverability_record_refs,
            automation_labels,
            aliases: normalized_aliases(
                &self.descriptor.command_id,
                &self.descriptor.aliases,
                &self.alias_records,
            ),
        }
    }

    fn validate_public_command_contract(&self) -> Result<(), &'static str> {
        if !matches!(
            self.descriptor.lifecycle_state.as_str(),
            "beta" | "stable" | "lts_facing" | "deprecated"
        ) {
            return Ok(());
        }

        let contract = self.public_contract();
        if contract.category_refs.is_empty() {
            return Err("stable command must declare category refs");
        }
        if contract.origin_class.trim().is_empty() {
            return Err("stable command must declare origin metadata");
        }
        if contract.invocation_schema_ref.trim().is_empty() {
            return Err("stable command must declare invocation schema ref");
        }
        if contract.result_schema_ref.trim().is_empty() {
            return Err("stable command must declare result schema ref");
        }
        if self
            .descriptor
            .docs_help_anchor_ref
            .anchor_id
            .trim()
            .is_empty()
        {
            return Err("stable command must declare help anchor");
        }
        if self.descriptor.lifecycle_state.trim().is_empty()
            || self.descriptor.support_class.trim().is_empty()
            || self.descriptor.release_channel.trim().is_empty()
            || self.descriptor.declared_freshness_class.trim().is_empty()
        {
            return Err("stable command must declare lifecycle metadata");
        }
        if contract.capability_scope_class.trim().is_empty() {
            return Err("stable command must declare capability class");
        }
        if contract.enablement_rule_refs.is_empty() {
            return Err("stable command must declare enablement rule refs");
        }
        if contract.discoverability_record_refs.is_empty() {
            return Err("stable command must declare discoverability refs");
        }
        if contract.automation_labels.is_empty() {
            return Err("stable command must declare automation labels");
        }
        if self.descriptor.capability_scope_class != "inert_metadata_only"
            && self.disabled_reason_records.is_empty()
        {
            return Err("stable command must declare disabled-reason records");
        }
        for alias in &contract.aliases {
            if alias.canonical_command_id.as_deref() != Some(self.descriptor.command_id.as_str()) {
                return Err("alias must resolve to canonical command id");
            }
            if alias.deprecation_state.as_deref().unwrap_or("").is_empty() {
                return Err("alias must declare lifecycle state");
            }
        }
        Ok(())
    }
}

fn string_field(value: &serde_json::Value, key: &str) -> Option<String> {
    value.get(key)?.as_str().map(str::to_string)
}

fn string_array_field(value: &serde_json::Value, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(|field| field.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn discoverability_refs_from_record(value: &serde_json::Value) -> Vec<String> {
    let mut refs = Vec::new();
    if let Some(id) = string_field(value, "discoverability_record_id") {
        refs.push(id);
    }
    if let Some(projection_refs) = value
        .get("projection_refs")
        .and_then(|field| field.as_object())
    {
        refs.extend(
            projection_refs
                .values()
                .filter_map(|field| field.as_str().map(str::to_string)),
        );
    }
    refs
}

fn alias_record_for<'a>(
    alias_id: &str,
    alias_records: &'a [serde_json::Value],
) -> Option<&'a serde_json::Value> {
    alias_records
        .iter()
        .find(|record| record.get("alias_id").and_then(|value| value.as_str()) == Some(alias_id))
}

fn normalized_aliases(
    command_id: &str,
    aliases: &[CommandAlias],
    alias_records: &[serde_json::Value],
) -> Vec<CommandAlias> {
    aliases
        .iter()
        .map(|alias| {
            let record = alias_record_for(&alias.alias_id, alias_records);
            CommandAlias {
                alias_id: alias.alias_id.clone(),
                alias_kind: alias.alias_kind.clone(),
                canonical_command_id: alias
                    .canonical_command_id
                    .clone()
                    .or_else(|| Some(command_id.to_string())),
                replacement_note_ref: alias
                    .replacement_note_ref
                    .clone()
                    .or_else(|| record.and_then(|record| string_field(record, "notes_ref"))),
                introduced_version: alias
                    .introduced_version
                    .clone()
                    .or_else(|| record.and_then(|record| string_field(record, "introduced_ref"))),
                deprecation_state: alias
                    .deprecation_state
                    .clone()
                    .or_else(|| record.and_then(|record| string_field(record, "alias_state")))
                    .or_else(|| Some("active".to_string())),
                retirement_version: alias
                    .retirement_version
                    .clone()
                    .or_else(|| record.and_then(|record| string_field(record, "retired_ref"))),
            }
        })
        .collect()
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
const SEEDED_M5_REGISTRY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/commands/m5_command_registry_seed.yaml"
));

static SEEDED_REGISTRY: OnceLock<CommandRegistry> = OnceLock::new();

fn merged_seed_record() -> Result<CommandRegistrySeedRecord, RegistryError> {
    let mut seed: CommandRegistrySeedRecord = serde_json::from_str(SEEDED_REGISTRY_JSON)?;
    let m5_seed: CommandRegistrySeedRecord = serde_json::from_str(SEEDED_M5_REGISTRY_JSON)?;
    seed.entries.extend(m5_seed.entries);
    Ok(seed)
}

pub fn seeded_registry() -> &'static CommandRegistry {
    SEEDED_REGISTRY.get_or_init(|| {
        CommandRegistry::from_seed(merged_seed_record().expect("seeded registry merge must parse"))
            .expect("seeded command registry must parse and validate")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_registry_loads() {
        let registry = seeded_registry();
        assert_eq!(registry.entries().len(), 39);
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
        assert!(registry.get("cmd:notebook.run_all_cells").is_some());
        assert!(registry.get("cmd:data_api.send_request").is_some());
        assert!(registry.get("cmd:profiler.start_capture").is_some());
        assert!(registry.get("cmd:docs_browser.open_external").is_some());
        assert!(registry
            .get("cmd:secret_broker.open_credential_review")
            .is_some());
        assert!(registry
            .get("cmd:infrastructure.reconcile_workspace")
            .is_some());
    }

    #[test]
    fn seeded_registry_public_contracts_have_required_metadata() {
        let registry = seeded_registry();
        for entry in registry.entries().iter().filter(|entry| {
            matches!(
                entry.descriptor.lifecycle_state.as_str(),
                "beta" | "stable" | "lts_facing" | "deprecated"
            )
        }) {
            let contract = entry.public_contract();
            assert_eq!(contract.command_id, entry.descriptor.command_id);
            assert_eq!(contract.canonical_verb, entry.descriptor.canonical_verb);
            assert!(
                !contract.category_refs.is_empty(),
                "{}",
                contract.command_id
            );
            assert!(
                !contract.origin_class.trim().is_empty(),
                "{}",
                contract.command_id
            );
            assert_eq!(
                contract.invocation_schema_ref,
                "schemas/commands/command_invocation_session.schema.json"
            );
            assert_eq!(
                contract.result_schema_ref,
                "schemas/commands/command_result_packet.schema.json"
            );
            assert!(
                !contract.enablement_rule_refs.is_empty(),
                "{}",
                contract.command_id
            );
            assert!(
                !contract.discoverability_record_refs.is_empty(),
                "{}",
                contract.command_id
            );
            assert!(
                !contract.automation_labels.is_empty(),
                "{}",
                contract.command_id
            );
            let command_id = contract.command_id.clone();
            for alias in contract.aliases {
                assert_eq!(
                    alias.canonical_command_id.as_deref(),
                    Some(command_id.as_str())
                );
                assert!(
                    alias
                        .deprecation_state
                        .as_deref()
                        .is_some_and(|state| !state.is_empty()),
                    "{}",
                    command_id
                );
            }
        }
    }

    #[test]
    fn stable_entry_without_discoverability_or_automation_metadata_is_rejected() {
        let mut entry = seeded_registry()
            .get("cmd:workspace.open_folder")
            .expect("seeded command must exist")
            .clone();
        entry.descriptor.category_refs = vec!["category:workspace".to_string()];
        entry.descriptor.discoverability_record_refs.clear();
        entry.discoverability_record = serde_json::json!({});
        entry.descriptor.automation_labels.clear();
        entry.automation_labels.clear();

        let err = entry.validate_public_command_contract().unwrap_err();
        assert!(matches!(
            err,
            "stable command must declare discoverability refs"
                | "stable command must declare automation labels"
        ));
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
