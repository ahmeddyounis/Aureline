// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Invocation preview sheet projection.
//!
//! The invocation preview sheet is a protected review surface shown before a
//! consequence-bearing command applies. It quotes the canonical command review
//! packet plus the in-flight invocation session so the preview and apply paths
//! stay aligned.

use aureline_commands::invocation::CommandInvocationSession;
use aureline_commands::CommandRegistryEntryRecord;
use serde::{Deserialize, Serialize};
use std::path::Path;

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

#[derive(Debug, Serialize)]
struct CommandInvocationPreviewLogRecord {
    record_kind: &'static str,
    schema_version: u32,
    redaction_policy: &'static str,
    command_ref: String,
    invocation_session_ref: String,
    issuing_surface_class: String,
    authority_class: String,
    execution_intent_class: String,
    workspace_trust_state: String,
    typed_argument_count: usize,
    resolved_argument_count: usize,
    context_object_count: usize,
    focused_entity_present: bool,
    selection_present: bool,
    execution_context_present: bool,
    preflight_decision: String,
    enablement_decision: String,
    disabled_reason_code: Option<String>,
    preview_shown: bool,
    approval_state: String,
    approval_ticket_present: bool,
}

fn opaque_metadata_ref(class: &str, value: &str) -> String {
    format!(
        "{class}:{}",
        aureline_history::body_object_id(value.as_bytes())
    )
}

fn opaque_filename_id(value: &str) -> String {
    aureline_history::body_object_id(value.as_bytes())
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect()
}

fn closed_class(value: &str, allowed: &[&str]) -> String {
    if allowed.contains(&value) {
        value.to_string()
    } else {
        "unknown".to_string()
    }
}

fn invocation_preview_log_record(
    record: &CommandInvocationPreviewSheetRecord,
) -> CommandInvocationPreviewLogRecord {
    let session = &record.invocation_session;
    CommandInvocationPreviewLogRecord {
        record_kind: "command_invocation_preview_log_record",
        schema_version: 1,
        redaction_policy: "local_metadata_only_v1",
        command_ref: opaque_metadata_ref("command", &record.packet.command_id),
        invocation_session_ref: opaque_metadata_ref(
            "invocation-session",
            &session.invocation_session_id,
        ),
        issuing_surface_class: closed_class(
            &session.issuing_surface,
            &[
                "command_palette",
                "command_form",
                "menu",
                "keybinding",
                "start_center",
                "cli",
                "headless",
                "automation",
            ],
        ),
        authority_class: closed_class(
            &session.authority_class,
            &[
                "user_initiated_local",
                "user_initiated_remote",
                "automation_user_approved",
                "admin_initiated",
                "managed_policy",
            ],
        ),
        execution_intent_class: closed_class(
            &session.execution_intent,
            &[
                "inspect_only",
                "preview_only",
                "apply",
                "apply_after_preview",
                "apply_after_approval",
            ],
        ),
        workspace_trust_state: closed_class(
            &session.context_snapshot.workspace_trust_state,
            &[
                "trusted",
                "restricted",
                "pending_evaluation",
                "untrusted_unknown",
            ],
        ),
        typed_argument_count: record.packet.typed_arguments.len(),
        resolved_argument_count: session
            .argument_provenance_map
            .iter()
            .filter(|entry| entry.resolved_value_ref.is_some())
            .count(),
        context_object_count: session.context_refs.context_object_refs.len(),
        focused_entity_present: session.context_snapshot.focused_entity_ref.is_some(),
        selection_present: session.context_snapshot.selection_ref.is_some(),
        execution_context_present: session.context_snapshot.execution_context_id.is_some(),
        preflight_decision: closed_class(
            &record.packet.preflight.decision_class,
            &[
                "allowed",
                "blocked_by_policy",
                "disabled_with_reason",
                "preview_required",
                "approval_required",
            ],
        ),
        enablement_decision: session
            .enablement_decision
            .decision_class
            .as_str()
            .to_string(),
        disabled_reason_code: session
            .enablement_decision
            .disabled_reason_code
            .map(|code| code.as_str().to_string()),
        preview_shown: session.preview_posture.preview_shown,
        approval_state: closed_class(
            &session.approval_posture.approval_state,
            &[
                "not_required",
                "approval_pending",
                "approved",
                "denied",
                "expired",
                "revoked",
            ],
        ),
        approval_ticket_present: session.approval_posture.approval_ticket_ref.is_some(),
    }
}

/// Writes a metadata-only invocation preview under the configured logs root.
pub fn write_invocation_preview_sheet_log(record: &CommandInvocationPreviewSheetRecord) {
    write_invocation_preview_sheet_log_at_root(
        record,
        &aureline_workspace::state_paths::logs_root(),
    );
}

fn write_invocation_preview_sheet_log_at_root(
    record: &CommandInvocationPreviewSheetRecord,
    logs_root: &Path,
) {
    let root = logs_root.join("review_sheets");
    if std::fs::create_dir_all(&root).is_err() {
        return;
    }
    let file_identity = format!(
        "{}\0{}\0{}",
        record.packet.command_id,
        record.invocation_session.invocation_session_id,
        record.generated_at
    );
    let filename = format!(
        "{}.invocation_preview_sheet.json",
        opaque_filename_id(&file_identity)
    );
    let projection = invocation_preview_log_record(record);
    let Ok(json) = serde_json::to_string_pretty(&projection) else {
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

    #[test]
    fn durable_invocation_preview_excludes_context_and_argument_values() {
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../fixtures/commands/review_sheets/invocation_preview/workspace_import_profile.preview.json",
        );
        let fixture: PreviewFixtureRecord = serde_json::from_str(
            &std::fs::read_to_string(fixture_path).expect("invocation fixture must read"),
        )
        .expect("invocation fixture must parse");
        let sentinel = "PRIVATE-INVOCATION-SENTINEL/user/repository/secret.rs";
        let mut record = fixture.expected;
        record.generated_at = sentinel.to_string();
        record.packet.generated_at = sentinel.to_string();
        record.packet.command_id = sentinel.to_string();
        record.packet.command_revision_ref = sentinel.to_string();
        record.packet.canonical_verb = sentinel.to_string();
        record.packet.title = sentinel.to_string();
        record.packet.summary = sentinel.to_string();
        record.packet.automation_labels = vec![sentinel.to_string()];
        for entry in &mut record.packet.argument_provenance_map {
            entry.argument_name = sentinel.to_string();
            entry.provenance = sentinel.to_string();
            entry.resolved_value_ref = Some(sentinel.to_string());
        }
        let session = &mut record.invocation_session;
        session.invocation_session_id = sentinel.to_string();
        session.canonical_command_id = sentinel.to_string();
        session.command_revision_ref = sentinel.to_string();
        session.canonical_verb = sentinel.to_string();
        session.context_snapshot.focused_entity_ref = Some(sentinel.to_string());
        session.context_snapshot.selection_ref = Some(sentinel.to_string());
        session.context_snapshot.execution_context_id = Some(sentinel.to_string());
        session.context_snapshot.basis_snapshot_ref = sentinel.to_string();
        session.context_refs.context_object_refs = vec![sentinel.to_string()];
        for entry in &mut session.argument_provenance_map {
            entry.argument_name = sentinel.to_string();
            entry.provenance = sentinel.to_string();
            entry.resolved_value_ref = Some(sentinel.to_string());
        }

        let temp = tempfile::tempdir().expect("tempdir");
        write_invocation_preview_sheet_log_at_root(&record, temp.path());
        let path = std::fs::read_dir(temp.path().join("review_sheets"))
            .expect("review log directory")
            .next()
            .expect("review log entry")
            .expect("read review log entry")
            .path();
        let json = std::fs::read_to_string(path).expect("read invocation metadata log");

        assert!(json.contains("local_metadata_only_v1"));
        assert!(!json.contains(sentinel));
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
