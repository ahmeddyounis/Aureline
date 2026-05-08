//! Preview-pane payloads and action-footer affordances for the command palette.
//!
//! The palette preview is a structured projection over the canonical command
//! registry entry. It exists so the shell can render a richer "what will this
//! do?" pane and modifier-aware footer actions without reauthoring command
//! identity, arguments, enablement reasons, or preview/approval posture.

use std::collections::HashMap;

use aureline_commands::invocation::ArgumentProvenanceEntry;
use aureline_commands::{
    CommandEnablementContext, CommandRegistry, CommandRegistryEntryRecord, EnablementSnapshot,
    PreflightDecisionClass,
};
use serde::{Deserialize, Serialize};

use super::query_session::PaletteItemKey;

/// Runtime inputs required to evaluate command enablement and preflight posture
/// for palette preview and footer actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PalettePreviewRuntimeInputs<'a> {
    /// Client scope the palette issues commands under (for the desktop shell,
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
}

/// Copy intent classes surfaced by the palette footer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteCopyIntent {
    /// Copy the stable canonical command id (`cmd:...`).
    CommandId,
    /// Copy a non-executable CLI/headless skeleton.
    CliSkeleton,
}

/// Structured preview payload for the currently selected palette row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PalettePreviewRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub generated_at: String,
    pub selection: PalettePreviewSelection,
}

/// Selected-row preview payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PalettePreviewSelection {
    /// No row is selected.
    None,
    /// Selected command row preview.
    Command(PaletteCommandPreview),
    /// Selected file row preview.
    File(PaletteFilePreview),
}

/// Command-row preview payload used by the preview pane and footer copy actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaletteCommandPreview {
    pub command_id: String,
    pub command_revision_ref: String,
    pub canonical_verb: String,
    pub title: String,
    pub summary: String,
    pub dominant_side_effect_class: String,
    pub lifecycle_state: String,
    pub support_class: String,
    pub capability_scope_class: String,
    pub preview_class: String,
    pub approval_posture_class: String,
    pub client_scopes: Vec<String>,
    pub shortcuts: Vec<String>,
    pub typed_arguments: Vec<PaletteTypedArgumentPreview>,
    pub argument_provenance_map: Vec<ArgumentProvenanceEntry>,
    pub preflight: PalettePreflightPreview,
    pub copy: PaletteCopyPreview,
}

/// File-row preview payload surfaced by the preview pane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaletteFilePreview {
    pub relative_path: String,
}

/// Preview projection for one typed command argument.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaletteTypedArgumentPreview {
    pub argument_name: String,
    pub argument_kind: String,
    pub is_required: bool,
    pub default_provenance_when_omitted: Option<String>,
}

/// Preflight posture for the selected command row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PalettePreflightPreview {
    pub decision_class: String,
    pub enablement_snapshot: EnablementSnapshot,
}

/// Copy payloads for footer actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaletteCopyPreview {
    pub command_id: String,
    pub cli_skeleton: Option<String>,
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

fn sanitize_filename(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            ':' | '/' | '\\' | ' ' | '\t' | '\n' | '\r' => '_',
            other => other,
        })
        .collect()
}

/// Returns the argument provenance map the palette should use for enablement
/// and preflight evaluation.
///
/// This mirrors the same placeholder provenance used by the shell dispatch
/// path so preview, footer actions, and execution share one enablement result.
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

/// Builds a non-executable CLI/headless skeleton for the command.
///
/// The skeleton starts with the canonical verb and lists the typed argument
/// slots as placeholder values. It does not embed live values and is intended
/// for inspection and automation-facing explainability.
pub fn cli_skeleton_for(entry: &CommandRegistryEntryRecord) -> Option<String> {
    if !entry
        .descriptor
        .client_scopes
        .iter()
        .any(|scope| scope == "cli")
    {
        return None;
    }

    let mut out = entry.descriptor.canonical_verb.clone();
    for arg in &entry.descriptor.typed_arguments {
        if arg.is_required {
            out.push_str(" --");
            out.push_str(&arg.argument_name);
            out.push_str(" <");
            out.push_str(&arg.argument_name);
            out.push('>');
        } else {
            out.push_str(" [--");
            out.push_str(&arg.argument_name);
            out.push_str(" <");
            out.push_str(&arg.argument_name);
            out.push_str(">]");
        }
    }
    Some(out)
}

/// Materializes a preview record for the given palette selection.
pub fn materialize_palette_preview_record(
    selection: Option<&PaletteItemKey>,
    registry: &CommandRegistry,
    shortcuts_by_command_id: &HashMap<String, Vec<String>>,
    runtime: PalettePreviewRuntimeInputs<'_>,
) -> PalettePreviewRecord {
    let generated_at = aureline_commands::invocation::now_rfc3339();
    let selection = match selection {
        None => PalettePreviewSelection::None,
        Some(PaletteItemKey::File { relative_path }) => PalettePreviewSelection::File(
            PaletteFilePreview {
                relative_path: relative_path.clone(),
            },
        ),
        Some(PaletteItemKey::Command { command_id }) => {
            let Some(entry) = registry.get(command_id) else {
                return PalettePreviewRecord {
                    record_kind: "palette_preview_record".to_string(),
                    schema_version: 1,
                    generated_at,
                    selection: PalettePreviewSelection::None,
                };
            };

            let shortcuts = shortcuts_by_command_id
                .get(command_id)
                .cloned()
                .unwrap_or_default();
            let argument_provenance_map = argument_provenance_map_for(entry);
            let context = CommandEnablementContext {
                client_scope: runtime.client_scope.to_string(),
                workspace_trust_state: runtime.workspace_trust_state.to_string(),
                execution_context_available: runtime.execution_context_available,
                provider_linked: runtime.provider_linked,
                credential_available: runtime.credential_available,
                policy_disabled: runtime.policy_disabled,
                policy_blocked_in_context: runtime.policy_blocked_in_context,
                argument_provenance_map: argument_provenance_map.clone(),
            };
            let preflight = entry.preflight(&context);

            let typed_arguments = entry
                .descriptor
                .typed_arguments
                .iter()
                .map(|arg| PaletteTypedArgumentPreview {
                    argument_name: arg.argument_name.clone(),
                    argument_kind: arg.argument_kind.clone(),
                    is_required: arg.is_required,
                    default_provenance_when_omitted: arg.default_provenance_when_omitted.clone(),
                })
                .collect();

            PalettePreviewSelection::Command(PaletteCommandPreview {
                command_id: entry.descriptor.command_id.clone(),
                command_revision_ref: entry.descriptor.command_revision_ref.clone(),
                canonical_verb: entry.descriptor.canonical_verb.clone(),
                title: entry.title.clone(),
                summary: entry.summary.clone(),
                dominant_side_effect_class: entry.dominant_side_effect_class.clone(),
                lifecycle_state: entry.descriptor.lifecycle_state.clone(),
                support_class: entry.descriptor.support_class.clone(),
                capability_scope_class: entry.descriptor.capability_scope_class.clone(),
                preview_class: entry.descriptor.preview_class.clone(),
                approval_posture_class: entry.descriptor.approval_posture_class.clone(),
                client_scopes: entry.descriptor.client_scopes.clone(),
                shortcuts,
                typed_arguments,
                argument_provenance_map,
                preflight: PalettePreflightPreview {
                    decision_class: preflight_decision_token(preflight.decision_class).to_string(),
                    enablement_snapshot: preflight.enablement_snapshot,
                },
                copy: PaletteCopyPreview {
                    command_id: entry.descriptor.command_id.clone(),
                    cli_skeleton: cli_skeleton_for(entry),
                },
            })
        }
    };

    PalettePreviewRecord {
        record_kind: "palette_preview_record".to_string(),
        schema_version: 1,
        generated_at,
        selection,
    }
}

/// Writes a palette preview record into `.logs/palette_previews/`.
pub fn write_preview_log(record: &PalettePreviewRecord) {
    let PalettePreviewSelection::Command(command) = &record.selection else {
        return;
    };
    let root = std::path::PathBuf::from(".logs").join("palette_previews");
    if std::fs::create_dir_all(&root).is_err() {
        return;
    }

    let filename = format!(
        "{}.{}.palette_preview.json",
        sanitize_filename(&command.command_id),
        sanitize_filename(&record.generated_at)
    );
    let Ok(json) = serde_json::to_string_pretty(record) else {
        return;
    };
    let _ = std::fs::write(root.join(filename), json);
}

/// Returns the string payload to place on the clipboard for the requested
/// copy intent.
pub fn copy_payload_for(
    preview: &PaletteCommandPreview,
    intent: PaletteCopyIntent,
) -> Option<&str> {
    match intent {
        PaletteCopyIntent::CommandId => Some(preview.copy.command_id.as_str()),
        PaletteCopyIntent::CliSkeleton => preview.copy.cli_skeleton.as_deref(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use aureline_commands::registry::seeded_registry;
    use std::path::Path;

    #[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
    struct PreviewFixtureRecord {
        command_id: String,
        expected: PalettePreviewRecord,
    }

    fn load_fixture(path: &Path) -> String {
        std::fs::read_to_string(path).expect("fixture must read")
    }

    #[test]
    fn materializes_preview_cases_from_fixtures() {
        let registry = seeded_registry();
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/commands/palette_preview_cases");

        let runtime = PalettePreviewRuntimeInputs {
            client_scope: "desktop_product",
            workspace_trust_state: "trusted",
            execution_context_available: true,
            provider_linked: None,
            credential_available: None,
            policy_disabled: false,
            policy_blocked_in_context: false,
        };

        for entry in std::fs::read_dir(&root).expect("fixture directory must exist") {
            let entry = entry.expect("fixture directory entry must read");
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }

            let payload = load_fixture(&path);
            let fixture: PreviewFixtureRecord =
                serde_json::from_str(&payload).expect("preview fixture must parse");

            let mut shortcuts: HashMap<String, Vec<String>> = HashMap::new();
            shortcuts.insert(fixture.command_id.clone(), vec!["Cmd+O".to_string()]);

            let selection = PaletteItemKey::Command {
                command_id: fixture.command_id.clone(),
            };
            let mut record = materialize_palette_preview_record(
                Some(&selection),
                registry,
                &shortcuts,
                runtime,
            );

            // `generated_at` is time-varying; pin it to the fixture's value so
            // equality focuses on the stable contract fields.
            record.generated_at = fixture.expected.generated_at.clone();

            assert_eq!(
                record, fixture.expected,
                "preview record mismatch for fixture {}",
                path.display()
            );
        }
    }
}
