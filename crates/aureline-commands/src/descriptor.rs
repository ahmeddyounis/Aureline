use serde::{Deserialize, Serialize};

/// Opaque stable id safe to log and safe to serialize on boundaries.
pub type OpaqueId = String;

/// Stable command id (e.g. `cmd:workspace.open_folder`).
pub type CommandId = String;

/// A stable reference that ties a command descriptor to a specific revision.
pub type CommandRevisionRef = String;

/// Lightweight policy/trust context carried alongside descriptors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContext {
    pub policy_epoch: String,
    pub trust_state: String,
    #[serde(default)]
    pub execution_context_id: Option<String>,
}

/// Structured repair hook reference attached to a disabled reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairHookRef {
    pub hook_kind: String,
    pub hook_id: String,
    pub display_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityLabelPath {
    pub primary_label_ref: String,
    pub short_label_ref: String,
    pub long_description_ref: String,
    pub role_class: String,
    pub keyboard_shortcut_narration_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsHelpAnchorRef {
    pub pack_id: String,
    pub anchor_id: String,
    pub anchor_kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutNarrationHint {
    pub when_bound_narration_ref: String,
    pub when_unbound_narration_ref: String,
    pub chord_class_hint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandAlias {
    pub alias_id: String,
    pub alias_kind: String,
}

/// One typed argument declared by a command descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedArgument {
    pub argument_name: String,
    pub argument_kind: String,
    pub is_required: bool,
    pub default_provenance_when_omitted: Option<String>,
    pub enum_value_refs: Vec<String>,
    pub minimum_inclusive: Option<f64>,
    pub maximum_inclusive: Option<f64>,
    pub policy_pinned_when_trust_state_is: Vec<String>,
    pub narration_label_ref: String,
}

/// One discoverability-slot hint declared by a command descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSlotHint {
    pub ui_slot_class: String,
    pub menu_path_refs: Vec<String>,
    pub primary_or_secondary_toolbar_position_hint: String,
    pub weight_hint: i32,
    pub contextual_filter_class_ref: Option<String>,
}

/// The result contract advertised by a descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultContract {
    pub result_contract_class: String,
    pub artifact_kind_ref: Option<String>,
    pub typed_value_shape_ref: Option<String>,
    pub evidence_ref_class_required: Vec<String>,
}

/// Canonical command descriptor record.
///
/// This mirrors the boundary described by `schemas/commands/command_descriptor.schema.json`,
/// but intentionally keeps the struct centered on the runtime needs of the shell/palette
/// and registry consumers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandDescriptorRecord {
    pub record_kind: String,
    pub command_descriptor_schema_version: u32,

    pub command_id: CommandId,
    pub command_revision_ref: CommandRevisionRef,
    pub canonical_verb: String,

    pub primary_label_ref: String,
    pub accessibility_label_path: AccessibilityLabelPath,
    pub docs_help_anchor_ref: DocsHelpAnchorRef,
    pub shortcut_narration_hint: ShortcutNarrationHint,

    pub aliases: Vec<CommandAlias>,

    pub typed_arguments: Vec<TypedArgument>,

    pub capability_scope_class: String,
    pub preview_class: String,
    pub approval_posture_class: String,
    pub ai_tool_surfacing_class: String,
    pub palette_visibility: String,

    pub ui_slot_hints: Vec<UiSlotHint>,

    pub lifecycle_state: String,
    pub support_class: String,
    pub release_channel: String,
    pub declared_freshness_class: String,

    pub client_scopes: Vec<String>,

    pub result_contract: ResultContract,

    pub default_enablement_repair_hook_ref: Option<RepairHookRef>,
    pub policy_context: PolicyContext,
    pub redaction_class: String,
    pub minted_at: String,
}

impl CommandDescriptorRecord {
    pub fn validate_minimal(&self) -> Result<(), &'static str> {
        if self.record_kind != "command_descriptor_record" {
            return Err("descriptor record_kind must be command_descriptor_record");
        }
        if self.command_descriptor_schema_version != 1 {
            return Err("unsupported command_descriptor_schema_version");
        }
        if self.command_id.trim().is_empty() {
            return Err("command_id must be non-empty");
        }
        if self.canonical_verb.trim().is_empty() {
            return Err("canonical_verb must be non-empty");
        }
        Ok(())
    }
}
