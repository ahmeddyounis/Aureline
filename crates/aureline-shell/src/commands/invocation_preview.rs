//! Invocation preview sheet projection.
//!
//! The invocation preview sheet is a protected review surface shown before a
//! consequence-bearing command applies. It quotes the canonical command review
//! packet plus the in-flight invocation session so the preview and apply paths
//! stay aligned.

use aureline_commands::invocation::CommandInvocationSession;
use aureline_commands::CommandRegistryEntryRecord;
use serde::{Deserialize, Serialize};

use super::{
    materialize_command_review_packet_with_arguments, CommandReviewPacketRecord,
    CommandReviewRuntimeInputs,
};

/// Machine-readable record for an invocation preview sheet instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandInvocationPreviewSheetRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub generated_at: String,

    pub packet: CommandReviewPacketRecord,
    pub invocation_session: CommandInvocationSession,
}

/// Materializes an invocation preview sheet record for the provided command
/// entry and invocation session.
pub fn materialize_command_invocation_preview_sheet_record(
    entry: &CommandRegistryEntryRecord,
    session: &CommandInvocationSession,
    runtime: CommandReviewRuntimeInputs<'_>,
) -> CommandInvocationPreviewSheetRecord {
    let packet = materialize_command_review_packet_with_arguments(
        entry,
        runtime,
        session.argument_provenance_map.clone(),
    );
    CommandInvocationPreviewSheetRecord {
        record_kind: "command_invocation_preview_sheet_record".to_string(),
        schema_version: 1,
        generated_at: packet.generated_at.clone(),
        packet,
        invocation_session: session.clone(),
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

/// Writes an invocation preview sheet record into `.logs/review_sheets/`.
pub fn write_invocation_preview_sheet_log(record: &CommandInvocationPreviewSheetRecord) {
    let root = std::path::PathBuf::from(".logs").join("review_sheets");
    if std::fs::create_dir_all(&root).is_err() {
        return;
    }
    let filename = format!(
        "{}.{}.invocation_preview_sheet.json",
        sanitize_filename(&record.packet.command_id),
        sanitize_filename(&record.generated_at)
    );
    let Ok(json) = serde_json::to_string_pretty(record) else {
        return;
    };
    let _ = std::fs::write(root.join(filename), json);
}

/// Builds the human-readable lines used by the shell to render an invocation
/// preview sheet.
pub fn invocation_preview_sheet_lines(record: &CommandInvocationPreviewSheetRecord) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("Preview — {}", record.packet.title));
    lines.push("Esc: cancel   Enter: apply".to_string());
    lines.push("".to_string());

    lines.push(format!("command_id: {}", record.packet.command_id));
    lines.push(format!("canonical_verb: {}", record.packet.canonical_verb));
    lines.push(format!(
        "side_effects: {}   capability: {}",
        record.packet.dominant_side_effect_class, record.packet.capability_scope_class
    ));
    lines.push(format!(
        "preview: {}   approval: {}",
        record.packet.preview_class, record.packet.approval_posture_class
    ));
    lines.push(format!(
        "result_contract: {}   evidence_required: {}",
        record.packet.result_contract_class,
        if record.packet.evidence_ref_class_required.is_empty() {
            "<none>".to_string()
        } else {
            record.packet.evidence_ref_class_required.join(", ")
        }
    ));
    if !record.packet.automation_labels.is_empty() {
        lines.push(format!(
            "automation_labels: {}",
            record.packet.automation_labels.join(", ")
        ));
    }

    lines.push("".to_string());
    lines.push(format!(
        "invocation_session_id: {}   issuing_surface: {}   authority: {}",
        record.invocation_session.invocation_session_id,
        record.invocation_session.issuing_surface,
        record.invocation_session.authority_class
    ));
    lines.push(format!(
        "execution_intent: {}   preflight: {}",
        record.invocation_session.execution_intent, record.packet.preflight.decision_class
    ));
    lines.push(format!(
        "focused: {}   trust: {}   exec_ctx: {}",
        record
            .invocation_session
            .context_snapshot
            .focused_entity_ref
            .as_deref()
            .unwrap_or("<none>"),
        record
            .invocation_session
            .context_snapshot
            .workspace_trust_state,
        record
            .invocation_session
            .context_snapshot
            .execution_context_id
            .as_deref()
            .unwrap_or("<none>")
    ));
    lines.push(format!(
        "basis_snapshot_ref: {}",
        record
            .invocation_session
            .context_snapshot
            .basis_snapshot_ref
    ));

    if let Some(preview_ref) = record
        .invocation_session
        .preview_posture
        .preview_record_ref
        .as_ref()
    {
        lines.push(format!("preview_record_ref: {preview_ref}"));
    }
    if let Some(ticket_ref) = record
        .invocation_session
        .approval_posture
        .approval_ticket_ref
        .as_ref()
    {
        lines.push(format!("approval_ticket_ref: {ticket_ref}"));
    }

    if !record.packet.typed_arguments.is_empty() {
        lines.push("".to_string());
        lines.push("Arguments:".to_string());
        for arg in &record.packet.typed_arguments {
            let resolved = record
                .packet
                .argument_provenance_map
                .iter()
                .find(|row| row.argument_name == arg.argument_name)
                .and_then(|row| row.resolved_value_ref.as_deref())
                .unwrap_or("<unresolved>");
            lines.push(format!(
                "- {} ({}) => {}",
                arg.argument_name, arg.argument_kind, resolved
            ));
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    use aureline_commands::descriptor::PolicyContext;
    use aureline_commands::enablement::EnablementDecisionClass;
    use aureline_commands::invocation::{
        AliasUsedBlock, ApprovalPostureBlock, CommandInvocationSession, ContextRefsBlock,
        EnablementDecisionBlock, InvocationContextSnapshot, PreviewPostureBlock,
    };
    use aureline_commands::registry::seeded_registry;
    use std::path::Path;

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    struct PreviewFixtureRecord {
        command_id: String,
        expected: CommandInvocationPreviewSheetRecord,
    }

    fn load_fixture(path: &Path) -> String {
        std::fs::read_to_string(path).expect("fixture must read")
    }

    #[test]
    fn materializes_invocation_preview_cases_from_fixtures() {
        let registry = seeded_registry();
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/commands/review_sheets/invocation_preview");

        let runtime = CommandReviewRuntimeInputs {
            client_scope: "desktop_product",
            workspace_trust_state: "trusted",
            execution_context_available: true,
            provider_linked: None,
            credential_available: None,
            policy_disabled: false,
            policy_blocked_in_context: false,
            labs_enabled: false,
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

            let Some(command) = registry.get(&fixture.command_id) else {
                panic!(
                    "fixture references unknown command_id: {}",
                    fixture.command_id
                );
            };

            let session = CommandInvocationSession {
                invocation_session_id: "inv:fixture:01".to_string(),
                canonical_command_id: command.descriptor.command_id.clone(),
                command_revision_ref: command.descriptor.command_revision_ref.clone(),
                canonical_verb: command.descriptor.canonical_verb.clone(),
                issuing_surface: "command_palette".to_string(),
                authority_class: "user_initiated_local".to_string(),
                alias_used: AliasUsedBlock {
                    alias_kind: "canonical".to_string(),
                    alias_id: None,
                    alias_state: "not_applicable".to_string(),
                    resolves_to_canonical_command_id: command.descriptor.command_id.clone(),
                    migration_trace_ref: None,
                    support_window_ref: None,
                },
                argument_provenance_map: record_packet_argument_map(command),
                context_snapshot: InvocationContextSnapshot {
                    focused_entity_ref: Some("shell-zone:main_workspace".to_string()),
                    selection_ref: None,
                    workspace_trust_state: "trusted".to_string(),
                    execution_context_id: command
                        .descriptor
                        .policy_context
                        .execution_context_id
                        .clone(),
                    scope_filter_class_ref: None,
                    basis_snapshot_ref: "basis:fixture:01".to_string(),
                },
                context_refs: ContextRefsBlock {
                    focused_entity_ref: Some("shell-zone:main_workspace".to_string()),
                    selection_ref: None,
                    workspace_ref: None,
                    workspace_trust_state: "trusted".to_string(),
                    execution_context_id: command
                        .descriptor
                        .policy_context
                        .execution_context_id
                        .clone(),
                    scope_filter_class_ref: None,
                    basis_snapshot_ref: "basis:fixture:01".to_string(),
                    context_object_refs: Vec::new(),
                },
                enablement_decision: EnablementDecisionBlock {
                    decision_class: EnablementDecisionClass::Enabled,
                    disabled_reason_code: None,
                    repair_hook_ref: None,
                },
                preview_posture: PreviewPostureBlock {
                    preview_class_declared: command.descriptor.preview_class.clone(),
                    preview_shown: true,
                    preview_record_ref: Some("preview:fixture:01".to_string()),
                },
                approval_posture: ApprovalPostureBlock {
                    approval_posture_class_declared: command
                        .descriptor
                        .approval_posture_class
                        .clone(),
                    approval_state: "approval_pending".to_string(),
                    approval_ticket_ref: Some("approval-ticket:fixture:01".to_string()),
                },
                execution_intent: "apply_after_preview".to_string(),
                policy_context: PolicyContext {
                    policy_epoch: command.descriptor.policy_context.policy_epoch.clone(),
                    trust_state: "trusted".to_string(),
                    execution_context_id: command
                        .descriptor
                        .policy_context
                        .execution_context_id
                        .clone(),
                },
                redaction_class: command.descriptor.redaction_class.clone(),
            };

            let mut record =
                materialize_command_invocation_preview_sheet_record(command, &session, runtime);
            record.generated_at = fixture.expected.generated_at.clone();
            record.packet.generated_at = fixture.expected.packet.generated_at.clone();

            assert_eq!(
                record,
                fixture.expected,
                "invocation preview sheet record mismatch for fixture {}",
                path.display()
            );
        }
    }

    fn record_packet_argument_map(
        command: &CommandRegistryEntryRecord,
    ) -> Vec<aureline_commands::invocation::ArgumentProvenanceEntry> {
        // Use a resolved provenance map for one representative command so the preview sheet
        // includes argument refs.
        if command.descriptor.command_id == "cmd:workspace.import_profile" {
            return vec![
                aureline_commands::invocation::ArgumentProvenanceEntry {
                    argument_name: "import_source_ref".to_string(),
                    provenance: "user_selected_from_palette_suggestion".to_string(),
                    resolved_value_ref: Some("import-source:fixture:01".to_string()),
                },
                aureline_commands::invocation::ArgumentProvenanceEntry {
                    argument_name: "apply_scope".to_string(),
                    provenance: "default_from_descriptor".to_string(),
                    resolved_value_ref: Some(
                        "enum:workspace.import_profile:profile_only".to_string(),
                    ),
                },
                aureline_commands::invocation::ArgumentProvenanceEntry {
                    argument_name: "create_restore_checkpoint".to_string(),
                    provenance: "default_from_descriptor".to_string(),
                    resolved_value_ref: Some("value:bool:true".to_string()),
                },
            ];
        }

        command
            .descriptor
            .typed_arguments
            .iter()
            .map(
                |slot| aureline_commands::invocation::ArgumentProvenanceEntry {
                    argument_name: slot.argument_name.clone(),
                    provenance: slot
                        .default_provenance_when_omitted
                        .clone()
                        .unwrap_or_else(|| "user_typed".to_string()),
                    resolved_value_ref: None,
                },
            )
            .collect()
    }
}
