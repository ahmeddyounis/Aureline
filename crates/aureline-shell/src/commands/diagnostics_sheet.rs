//! Command diagnostics sheet projection.
//!
//! The diagnostics sheet explains why a command is currently unavailable using
//! structured enablement reason codes and repair hooks rather than surface-local
//! prose.

use aureline_commands::descriptor::RepairHookRef;
use aureline_commands::enablement::{DisabledReasonCode, EnablementDecisionClass};
use aureline_commands::CommandRegistryEntryRecord;
use serde::{Deserialize, Serialize};

use super::{
    materialize_command_review_packet, CommandReviewPacketRecord, CommandReviewRuntimeInputs,
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

fn sanitize_filename(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            ':' | '/' | '\\' | ' ' | '\t' | '\n' | '\r' => '_',
            other => other,
        })
        .collect()
}

/// Writes a diagnostics sheet record into `.logs/review_sheets/`.
pub fn write_diagnostics_sheet_log(record: &CommandDiagnosticsSheetRecord) {
    let root = std::path::PathBuf::from(".logs").join("review_sheets");
    if std::fs::create_dir_all(&root).is_err() {
        return;
    }
    let filename = format!(
        "{}.{}.command_diagnostics_sheet.json",
        sanitize_filename(&record.packet.command_id),
        sanitize_filename(&record.generated_at)
    );
    let Ok(json) = serde_json::to_string_pretty(record) else {
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
    lines.push(format!("preflight: {}", record.packet.preflight.decision_class));
    lines.push(format!(
        "enablement: {}",
        record
            .packet
            .preflight
            .enablement_snapshot
            .decision_class
            .as_str()
    ));

    if record.packet.preflight.enablement_snapshot.decision_class != EnablementDecisionClass::Enabled
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
                panic!("fixture references unknown command_id: {}", fixture.command_id);
            };

            let mut record = materialize_command_diagnostics_sheet_record(command, runtime);
            record.generated_at = fixture.expected.generated_at.clone();
            record.packet.generated_at = fixture.expected.packet.generated_at.clone();

            assert_eq!(
                record, fixture.expected,
                "diagnostics sheet record mismatch for fixture {}",
                path.display()
            );
        }
    }
}
