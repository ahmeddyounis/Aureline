//! Command review-sheet projections used by the desktop shell.
//!
//! The shell uses two dedicated review surfaces:
//!
//! - a command diagnostics sheet (to explain why a command cannot run), and
//! - an invocation preview sheet (to preview scope and posture before apply).
//!
//! Both sheets render the same underlying [`CommandReviewPacketRecord`], so
//! command identity, enablement, preview posture, and reason codes remain
//! aligned across shell surfaces.

use aureline_commands::invocation::ArgumentProvenanceEntry;
use aureline_commands::{
    CommandEnablementContext, CommandRegistryEntryRecord, EnablementSnapshot,
    PreflightDecisionClass,
};
use serde::{Deserialize, Serialize};

pub mod diagnostics_sheet;
pub mod invocation_preview;
pub mod review_enforcement;

/// Runtime inputs required to evaluate command enablement and preflight posture
/// for review sheets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandReviewRuntimeInputs<'a> {
    /// Client scope the shell issues commands under (for the desktop shell,
    /// this is typically `desktop_product`).
    pub client_scope: &'a str,
    /// Current workspace trust state token (`trusted`, `restricted`, ...).
    pub workspace_trust_state: &'a str,
    /// Whether an execution context is available for command dispatch.
    pub execution_context_available: bool,
    /// Optional provider-linked state when a command depends on a provider.
    pub provider_linked: Option<bool>,
    /// Optional credential availability when a command depends on credentials.
    pub credential_available: Option<bool>,
    /// Whether the command is disabled globally by policy.
    pub policy_disabled: bool,
    /// Whether the command is blocked in the current context by policy.
    pub policy_blocked_in_context: bool,
    /// Whether Labs commands are explicitly enabled for this local session.
    pub labs_enabled: bool,
}

/// Canonical packet shared by the command diagnostics sheet and invocation
/// preview sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandReviewPacketRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub generated_at: String,

    pub command_id: String,
    pub command_revision_ref: String,
    pub canonical_verb: String,

    pub title: String,
    pub summary: String,

    pub dominant_side_effect_class: String,
    pub automation_labels: Vec<String>,

    pub lifecycle_state: String,
    pub support_class: String,
    pub capability_scope_class: String,
    pub preview_class: String,
    pub approval_posture_class: String,
    pub client_scopes: Vec<String>,
    pub result_contract_class: String,
    pub evidence_ref_class_required: Vec<String>,

    pub typed_arguments: Vec<ReviewTypedArgument>,
    pub argument_provenance_map: Vec<ArgumentProvenanceEntry>,

    pub preflight: ReviewPreflight,
}

/// Review-sheet projection for one typed command argument slot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewTypedArgument {
    pub argument_name: String,
    pub argument_kind: String,
    pub is_required: bool,
    pub default_provenance_when_omitted: Option<String>,
}

/// Preflight posture for a would-be invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPreflight {
    pub decision_class: String,
    pub enablement_snapshot: EnablementSnapshot,
}

fn preflight_decision_token(decision: PreflightDecisionClass) -> &'static str {
    match decision {
        PreflightDecisionClass::Allowed => "allowed",
        PreflightDecisionClass::BlockedByPolicy => "blocked_by_policy",
        PreflightDecisionClass::DisabledWithReason => "disabled_with_reason",
        PreflightDecisionClass::PreviewRequired => "preview_required",
        PreflightDecisionClass::ApprovalRequired => "approval_required",
    }
}

/// Returns the argument provenance map the shell should use for preflight and
/// dispatch evaluation.
///
/// The preview and dispatch paths share one provenance map so enablement and
/// preflight decisions stay identical across review sheets and execution.
pub fn argument_provenance_map_for(
    entry: &CommandRegistryEntryRecord,
) -> Vec<ArgumentProvenanceEntry> {
    match entry.descriptor.command_id.as_str() {
        "cmd:workspace.open_folder" => vec![
            ArgumentProvenanceEntry {
                argument_name: "workspace_scope_ref".to_string(),
                provenance: "user_selected_from_palette_suggestion".to_string(),
                resolved_value_ref: Some("workspace-scope:folder:recent:01".to_string()),
            },
            ArgumentProvenanceEntry {
                argument_name: "add_to_workspace".to_string(),
                provenance: "default_from_descriptor".to_string(),
                resolved_value_ref: Some("value:bool:false".to_string()),
            },
        ],
        "cmd:docs.open_in_browser" => vec![
            ArgumentProvenanceEntry {
                argument_name: "destination_anchor_ref".to_string(),
                provenance: "default_from_shell_context".to_string(),
                resolved_value_ref: Some("docs:anchor:docs:open_in_browser_overview".to_string()),
            },
            ArgumentProvenanceEntry {
                argument_name: "destination_descriptor_ref".to_string(),
                provenance: "default_from_descriptor".to_string(),
                resolved_value_ref: None,
            },
        ],
        "cmd:workspace.import_profile" => vec![
            ArgumentProvenanceEntry {
                argument_name: "import_source_ref".to_string(),
                provenance: "user_selected_from_palette_suggestion".to_string(),
                resolved_value_ref: Some("import-source:placeholder:01".to_string()),
            },
            ArgumentProvenanceEntry {
                argument_name: "apply_scope".to_string(),
                provenance: "default_from_descriptor".to_string(),
                resolved_value_ref: Some("enum:workspace.import_profile:profile_only".to_string()),
            },
            ArgumentProvenanceEntry {
                argument_name: "create_restore_checkpoint".to_string(),
                provenance: "default_from_descriptor".to_string(),
                resolved_value_ref: Some("value:bool:true".to_string()),
            },
        ],
        _ => entry
            .descriptor
            .typed_arguments
            .iter()
            .map(|slot| ArgumentProvenanceEntry {
                argument_name: slot.argument_name.clone(),
                provenance: slot
                    .default_provenance_when_omitted
                    .clone()
                    .unwrap_or_else(|| "user_typed".to_string()),
                resolved_value_ref: None,
            })
            .collect(),
    }
}

/// Materializes the canonical review packet for the provided command entry and
/// runtime posture.
pub fn materialize_command_review_packet(
    entry: &CommandRegistryEntryRecord,
    runtime: CommandReviewRuntimeInputs<'_>,
) -> CommandReviewPacketRecord {
    let argument_provenance_map = argument_provenance_map_for(entry);
    materialize_command_review_packet_with_arguments(entry, runtime, argument_provenance_map)
}

/// Materializes the canonical review packet using the provided argument
/// provenance map.
pub fn materialize_command_review_packet_with_arguments(
    entry: &CommandRegistryEntryRecord,
    runtime: CommandReviewRuntimeInputs<'_>,
    argument_provenance_map: Vec<ArgumentProvenanceEntry>,
) -> CommandReviewPacketRecord {
    let generated_at = aureline_commands::invocation::now_rfc3339();
    let context = CommandEnablementContext {
        client_scope: runtime.client_scope.to_string(),
        workspace_trust_state: runtime.workspace_trust_state.to_string(),
        execution_context_available: runtime.execution_context_available,
        provider_linked: runtime.provider_linked,
        credential_available: runtime.credential_available,
        policy_disabled: runtime.policy_disabled,
        policy_blocked_in_context: runtime.policy_blocked_in_context,
        labs_enabled: runtime.labs_enabled,
        argument_provenance_map: argument_provenance_map.clone(),
    };

    let preflight = entry.preflight(&context);
    let typed_arguments = entry
        .descriptor
        .typed_arguments
        .iter()
        .map(|arg| ReviewTypedArgument {
            argument_name: arg.argument_name.clone(),
            argument_kind: arg.argument_kind.clone(),
            is_required: arg.is_required,
            default_provenance_when_omitted: arg.default_provenance_when_omitted.clone(),
        })
        .collect();

    CommandReviewPacketRecord {
        record_kind: "command_review_packet_record".to_string(),
        schema_version: 1,
        generated_at,
        command_id: entry.descriptor.command_id.clone(),
        command_revision_ref: entry.descriptor.command_revision_ref.clone(),
        canonical_verb: entry.descriptor.canonical_verb.clone(),
        title: entry.title.clone(),
        summary: entry.summary.clone(),
        dominant_side_effect_class: entry.dominant_side_effect_class.clone(),
        automation_labels: entry.automation_labels.clone(),
        lifecycle_state: entry.descriptor.lifecycle_state.clone(),
        support_class: entry.descriptor.support_class.clone(),
        capability_scope_class: entry.descriptor.capability_scope_class.clone(),
        preview_class: entry.descriptor.preview_class.clone(),
        approval_posture_class: entry.descriptor.approval_posture_class.clone(),
        client_scopes: entry.descriptor.client_scopes.clone(),
        result_contract_class: entry
            .descriptor
            .result_contract
            .result_contract_class
            .clone(),
        evidence_ref_class_required: entry
            .descriptor
            .result_contract
            .evidence_ref_class_required
            .clone(),
        typed_arguments,
        argument_provenance_map,
        preflight: ReviewPreflight {
            decision_class: preflight_decision_token(preflight.decision_class).to_string(),
            enablement_snapshot: preflight.enablement_snapshot,
        },
    }
}
