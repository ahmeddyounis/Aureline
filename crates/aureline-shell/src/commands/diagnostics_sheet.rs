// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Command diagnostics sheet projection.
//!
//! The diagnostics sheet explains why a command is currently unavailable using
//! structured enablement reason codes and repair hooks rather than surface-local
//! prose.

use aureline_commands::descriptor::RepairHookRef;
use aureline_commands::enablement::{DisabledReasonCode, EnablementDecisionClass};
use aureline_commands::invocation::ArgumentProvenanceEntry;
use aureline_commands::CommandRegistryEntryRecord;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::{
    materialize_command_review_packet, materialize_command_review_packet_with_arguments,
    CommandReviewPacketRecord, CommandReviewRuntimeInputs,
};

/// Machine-readable record for a command diagnostics sheet instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandDiagnosticsSheetRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub generated_at: String,

    pub packet: CommandReviewPacketRecord,
    pub runtime_context: DiagnosticsRuntimeContextRecord,
    pub disabled_reason: Option<DisabledReasonDetailsRecord>,
}

/// Runtime posture captured alongside a diagnostics sheet record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsRuntimeContextRecord {
    pub client_scope: String,
    pub workspace_trust_state: String,
    pub execution_context_available: bool,
    pub provider_linked: Option<bool>,
    pub credential_available: Option<bool>,
    pub policy_disabled: bool,
    pub policy_blocked_in_context: bool,
}

/// Expanded disabled-reason details quoted by the diagnostics sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisabledReasonDetailsRecord {
    pub disabled_reason_code: String,
    pub owner_boundary_class: Option<String>,
    pub explanation_ref: Option<String>,
    pub repair_hook_ref: Option<RepairHookRef>,
    pub fallback_command_id: Option<String>,
}

/// Materializes the diagnostics sheet record for the provided command entry and
/// runtime posture.
pub fn materialize_command_diagnostics_sheet_record(
    entry: &CommandRegistryEntryRecord,
    runtime: CommandReviewRuntimeInputs<'_>,
) -> CommandDiagnosticsSheetRecord {
    let packet = materialize_command_review_packet(entry, runtime);
    diagnostics_sheet_record_from_packet(entry, runtime, packet)
}

/// Materializes the diagnostics sheet record using the invocation argument
/// provenance map that produced the disabled decision.
pub fn materialize_command_diagnostics_sheet_record_with_arguments(
    entry: &CommandRegistryEntryRecord,
    runtime: CommandReviewRuntimeInputs<'_>,
    argument_provenance_map: Vec<ArgumentProvenanceEntry>,
) -> CommandDiagnosticsSheetRecord {
    let packet =
        materialize_command_review_packet_with_arguments(entry, runtime, argument_provenance_map);
    diagnostics_sheet_record_from_packet(entry, runtime, packet)
}

fn diagnostics_sheet_record_from_packet(
    entry: &CommandRegistryEntryRecord,
    runtime: CommandReviewRuntimeInputs<'_>,
    packet: CommandReviewPacketRecord,
) -> CommandDiagnosticsSheetRecord {
    let disabled_reason_code = packet.preflight.enablement_snapshot.disabled_reason_code;
    let disabled_reason = disabled_reason_code.map(|code| {
        let record = entry
            .disabled_reason_records
            .iter()
            .find(|row| row.disabled_reason_code == code);
        DisabledReasonDetailsRecord {
            disabled_reason_code: code.as_str().to_string(),
            owner_boundary_class: record.map(|r| r.owner_boundary_class.clone()),
            explanation_ref: record.map(|r| r.explanation_ref.clone()),
            repair_hook_ref: packet.preflight.enablement_snapshot.repair_hook_ref.clone(),
            fallback_command_id: record.and_then(|r| r.fallback_command_id.clone()),
        }
    });

    CommandDiagnosticsSheetRecord {
        record_kind: "command_diagnostics_sheet_record".to_string(),
        schema_version: 1,
        generated_at: packet.generated_at.clone(),
        packet,
        runtime_context: DiagnosticsRuntimeContextRecord {
            client_scope: runtime.client_scope.to_string(),
            workspace_trust_state: runtime.workspace_trust_state.to_string(),
            execution_context_available: runtime.execution_context_available,
            provider_linked: runtime.provider_linked,
            credential_available: runtime.credential_available,
            policy_disabled: runtime.policy_disabled,
            policy_blocked_in_context: runtime.policy_blocked_in_context,
        },
        disabled_reason,
    }
}

#[derive(Debug, Serialize)]
struct CommandDiagnosticsLogRecord {
    record_kind: &'static str,
    schema_version: u32,
    redaction_policy: &'static str,
    command_ref: String,
    client_scope_class: String,
    workspace_trust_state: String,
    execution_context_available: bool,
    provider_linked: Option<bool>,
    credential_available: Option<bool>,
    policy_disabled: bool,
    policy_blocked_in_context: bool,
    typed_argument_count: usize,
    resolved_argument_count: usize,
    preflight_decision: String,
    enablement_decision: String,
    disabled_reason_code: Option<String>,
    repair_hook_present: bool,
    owner_boundary_present: bool,
    explanation_present: bool,
    fallback_command_present: bool,
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

fn diagnostics_log_record(record: &CommandDiagnosticsSheetRecord) -> CommandDiagnosticsLogRecord {
    CommandDiagnosticsLogRecord {
        record_kind: "command_diagnostics_log_record",
        schema_version: 1,
        redaction_policy: "local_metadata_only_v1",
        command_ref: opaque_metadata_ref("command", &record.packet.command_id),
        client_scope_class: closed_class(
            &record.runtime_context.client_scope,
            &[
                "desktop_product",
                "cli",
                "headless",
                "automation",
                "web",
                "mobile",
            ],
        ),
        workspace_trust_state: closed_class(
            &record.runtime_context.workspace_trust_state,
            &[
                "trusted",
                "restricted",
                "pending_evaluation",
                "untrusted_unknown",
            ],
        ),
        execution_context_available: record.runtime_context.execution_context_available,
        provider_linked: record.runtime_context.provider_linked,
        credential_available: record.runtime_context.credential_available,
        policy_disabled: record.runtime_context.policy_disabled,
        policy_blocked_in_context: record.runtime_context.policy_blocked_in_context,
        typed_argument_count: record.packet.typed_arguments.len(),
        resolved_argument_count: record
            .packet
            .argument_provenance_map
            .iter()
            .filter(|entry| entry.resolved_value_ref.is_some())
            .count(),
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
        enablement_decision: record
            .packet
            .preflight
            .enablement_snapshot
            .decision_class
            .as_str()
            .to_string(),
        disabled_reason_code: record
            .packet
            .preflight
            .enablement_snapshot
            .disabled_reason_code
            .map(|code| code.as_str().to_string()),
        repair_hook_present: record
            .packet
            .preflight
            .enablement_snapshot
            .repair_hook_ref
            .is_some(),
        owner_boundary_present: record
            .disabled_reason
            .as_ref()
            .and_then(|reason| reason.owner_boundary_class.as_ref())
            .is_some(),
        explanation_present: record
            .disabled_reason
            .as_ref()
            .and_then(|reason| reason.explanation_ref.as_ref())
            .is_some(),
        fallback_command_present: record
            .disabled_reason
            .as_ref()
            .and_then(|reason| reason.fallback_command_id.as_ref())
            .is_some(),
    }
}

/// Writes a metadata-only diagnostics record under the configured logs root.
pub fn write_diagnostics_sheet_log(record: &CommandDiagnosticsSheetRecord) {
    write_diagnostics_sheet_log_at_root(record, &aureline_workspace::state_paths::logs_root());
}

fn write_diagnostics_sheet_log_at_root(record: &CommandDiagnosticsSheetRecord, logs_root: &Path) {
    let root = logs_root.join("review_sheets");
    if std::fs::create_dir_all(&root).is_err() {
        return;
    }
    let file_identity = format!("{}\0{}", record.packet.command_id, record.generated_at);
    let filename = format!(
        "{}.command_diagnostics_sheet.json",
        opaque_filename_id(&file_identity)
    );
    let projection = diagnostics_log_record(record);
    let Ok(json) = serde_json::to_string_pretty(&projection) else {
        return;
    };
    let _ = std::fs::write(root.join(filename), json);
}

/// Builds the human-readable lines used by the shell to render a diagnostics
/// sheet.
pub fn diagnostics_sheet_lines(record: &CommandDiagnosticsSheetRecord) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("Diagnostics — {}", record.packet.title));
    lines.push("Esc: close".to_string());
    lines.push("".to_string());

    lines.push(format!("command_id: {}", record.packet.command_id));
    lines.push(format!("canonical_verb: {}", record.packet.canonical_verb));
    lines.push(format!(
        "preflight: {}",
        record.packet.preflight.decision_class
    ));
    lines.push(format!(
        "enablement: {}",
        record
            .packet
            .preflight
            .enablement_snapshot
            .decision_class
            .as_str()
    ));

    if record.packet.preflight.enablement_snapshot.decision_class
        != EnablementDecisionClass::Enabled
    {
        let code = record
            .packet
            .preflight
            .enablement_snapshot
            .disabled_reason_code
            .map(DisabledReasonCode::as_str)
            .unwrap_or("unknown");
        lines.push(format!("disabled_reason: {code}"));
        if let Some(repair) = record
            .packet
            .preflight
            .enablement_snapshot
            .repair_hook_ref
            .as_ref()
        {
            lines.push(format!(
                "repair_hook: {} ({})",
                repair.display_label, repair.hook_kind
            ));
        }
        if let Some(details) = record.disabled_reason.as_ref() {
            if let Some(owner) = details.owner_boundary_class.as_deref() {
                lines.push(format!("owner_boundary: {owner}"));
            }
            if let Some(explanation) = details.explanation_ref.as_deref() {
                lines.push(format!("explanation_ref: {explanation}"));
            }
        }
    }

    lines.push("".to_string());
    lines.push(format!(
        "runtime: trust={} exec_ctx={} policy_disabled={} policy_blocked={} provider_linked={} credential_available={}",
        record.runtime_context.workspace_trust_state,
        if record.runtime_context.execution_context_available {
            "available"
        } else {
            "unavailable"
        },
        record.runtime_context.policy_disabled,
        record.runtime_context.policy_blocked_in_context,
        opt_bool(record.runtime_context.provider_linked),
        opt_bool(record.runtime_context.credential_available)
    ));

    lines
}

fn opt_bool(value: Option<bool>) -> &'static str {
    match value {
        None => "unknown",
        Some(true) => "true",
        Some(false) => "false",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use aureline_commands::registry::seeded_registry;
    use std::path::Path;

    #[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
    struct DiagnosticsFixtureRecord {
        command_id: String,
        expected: CommandDiagnosticsSheetRecord,
    }

    fn load_fixture(path: &Path) -> String {
        std::fs::read_to_string(path).expect("fixture must read")
    }

    #[test]
    fn materializes_diagnostics_sheet_cases_from_fixtures() {
        let registry = seeded_registry();
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/commands/review_sheets/diagnostics");

        let runtime = CommandReviewRuntimeInputs {
            client_scope: "desktop_product",
            workspace_trust_state: "trusted",
            execution_context_available: false,
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
            let fixture: DiagnosticsFixtureRecord =
                serde_json::from_str(&payload).expect("diagnostics fixture must parse");

            let Some(command) = registry.get(&fixture.command_id) else {
                panic!(
                    "fixture references unknown command_id: {}",
                    fixture.command_id
                );
            };

            let mut record = materialize_command_diagnostics_sheet_record(command, runtime);
            record.generated_at = fixture.expected.generated_at.clone();
            record.packet.generated_at = fixture.expected.packet.generated_at.clone();

            assert_eq!(
                record,
                fixture.expected,
                "diagnostics sheet record mismatch for fixture {}",
                path.display()
            );
        }
    }

    #[test]
    fn durable_diagnostics_log_excludes_labels_and_argument_values() {
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../fixtures/commands/review_sheets/diagnostics/workspace_clone_repository.exec_ctx_unavailable.json",
        );
        let fixture: DiagnosticsFixtureRecord = serde_json::from_str(
            &std::fs::read_to_string(fixture_path).expect("diagnostics fixture must read"),
        )
        .expect("diagnostics fixture must parse");
        let sentinel = "PRIVATE-DIAGNOSTICS-SENTINEL/user/repository/secret.rs";
        let mut record = fixture.expected;
        record.generated_at = sentinel.to_string();
        record.packet.generated_at = sentinel.to_string();
        record.packet.command_id = sentinel.to_string();
        record.packet.command_revision_ref = sentinel.to_string();
        record.packet.canonical_verb = sentinel.to_string();
        record.packet.title = sentinel.to_string();
        record.packet.summary = sentinel.to_string();
        record.packet.automation_labels = vec![sentinel.to_string()];
        record.runtime_context.client_scope = sentinel.to_string();
        record.runtime_context.workspace_trust_state = sentinel.to_string();
        for entry in &mut record.packet.argument_provenance_map {
            entry.argument_name = sentinel.to_string();
            entry.provenance = sentinel.to_string();
            entry.resolved_value_ref = Some(sentinel.to_string());
        }
        if let Some(reason) = &mut record.disabled_reason {
            reason.owner_boundary_class = Some(sentinel.to_string());
            reason.explanation_ref = Some(sentinel.to_string());
            reason.fallback_command_id = Some(sentinel.to_string());
        }

        let temp = tempfile::tempdir().expect("tempdir");
        write_diagnostics_sheet_log_at_root(&record, temp.path());
        let path = std::fs::read_dir(temp.path().join("review_sheets"))
            .expect("diagnostics log directory")
            .next()
            .expect("diagnostics log entry")
            .expect("read diagnostics log entry")
            .path();
        let json = std::fs::read_to_string(path).expect("read diagnostics metadata log");

        assert!(json.contains("local_metadata_only_v1"));
        assert!(!json.contains(sentinel));
    }
}
