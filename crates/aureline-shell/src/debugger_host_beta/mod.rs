//! Beta debugger / DAP host shell projection.
//!
//! This module is a thin shell consumer over the canonical
//! [`aureline_runtime::DebugSessionSupportPacket`]. The shell does not
//! own session lifecycle truth; it renders the runtime-minted records into
//! reviewable rows and a deterministic plaintext block suitable for the
//! support-export clipboard action.

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    DebugAdapterNegotiationOutcome, DebugAdapterTransportClass, DebugSessionExitReasonClass,
    DebugSessionLifecycleEvent, DebugSessionMode, DebugSessionSnapshot, DebugSessionStateClass,
    DebugSessionSupportPacket,
};

/// Stable record-kind tag carried in serialized debugger-host projections.
pub const DEBUGGER_HOST_BETA_PROJECTION_RECORD_KIND: &str = "debugger_host_beta_projection_record";

/// Schema version for the projection payload.
pub const DEBUGGER_HOST_BETA_PROJECTION_SCHEMA_VERSION: u32 = 1;

/// Header notice rendered above the session rows.
pub const DEBUGGER_HOST_BETA_NOTICE: &str =
    "Debugger host (beta): session lifecycle is supervised. Adapter crashes \
     degrade only the affected session; quarantined sessions await an \
     explicit relaunch.";

/// One reviewable session row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebuggerHostBetaSessionRow {
    pub session_id: String,
    pub workspace_id: String,
    pub root_ref: String,
    pub language_id: String,
    pub execution_context_id: String,
    pub mode: DebugSessionMode,
    pub mode_token: String,
    pub adapter_id: String,
    pub adapter_label: String,
    pub adapter_version: String,
    pub transport_class: DebugAdapterTransportClass,
    pub transport_class_token: String,
    pub state_class: DebugSessionStateClass,
    pub state_class_token: String,
    pub state_requires_disclosure: bool,
    pub canonical_target_id: String,
    pub target_class_token: String,
    pub target_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub negotiation_outcome: Option<DebugAdapterNegotiationOutcome>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub negotiation_outcome_token: Option<String>,
    pub agreed_capability_tokens: Vec<String>,
    pub dropped_capability_tokens: Vec<String>,
    pub restart_strike_count: u32,
    pub restart_budget_in_window: u32,
    pub reconnect_attempt_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quarantine_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_exit_reason_class: Option<DebugSessionExitReasonClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_exit_reason_token: Option<String>,
    pub event_lineage_summary: Vec<String>,
    pub summary: String,
}

impl DebuggerHostBetaSessionRow {
    fn project(snapshot: &DebugSessionSnapshot) -> Self {
        Self {
            session_id: snapshot.identity.session_id.clone(),
            workspace_id: snapshot.identity.workspace_id.clone(),
            root_ref: snapshot.identity.root_ref.clone(),
            language_id: snapshot.identity.language_id.clone(),
            execution_context_id: snapshot.identity.execution_context_id.clone(),
            mode: snapshot.identity.mode,
            mode_token: snapshot.identity.mode_token.clone(),
            adapter_id: snapshot.identity.adapter.adapter_id.clone(),
            adapter_label: snapshot.identity.adapter.adapter_label.clone(),
            adapter_version: snapshot.identity.adapter.adapter_version.clone(),
            transport_class: snapshot.identity.adapter.transport_class,
            transport_class_token: snapshot.identity.adapter.transport_class_token.clone(),
            state_class: snapshot.state_class,
            state_class_token: snapshot.state_class_token.clone(),
            state_requires_disclosure: snapshot.state_class.requires_disclosure(),
            canonical_target_id: snapshot.identity.target.canonical_target_id.clone(),
            target_class_token: snapshot.identity.target.target_class_token.clone(),
            target_label: snapshot.identity.target.target_label.clone(),
            negotiation_outcome: snapshot.negotiation_outcome,
            negotiation_outcome_token: snapshot.negotiation_outcome_token.clone(),
            agreed_capability_tokens: snapshot
                .agreed_capabilities
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            dropped_capability_tokens: snapshot
                .dropped_capabilities
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            restart_strike_count: snapshot.restart_strike_count,
            restart_budget_in_window: snapshot.restart_budget_in_window,
            reconnect_attempt_count: snapshot.reconnect_attempt_count,
            quarantine_ref: snapshot.quarantine_ref.clone(),
            last_exit_reason_class: snapshot.last_exit_reason_class,
            last_exit_reason_token: snapshot.last_exit_reason_token.clone(),
            event_lineage_summary: snapshot.event_lineage.iter().map(summarize_event).collect(),
            summary: snapshot.summary.clone(),
        }
    }
}

fn summarize_event(event: &DebugSessionLifecycleEvent) -> String {
    format!(
        "{} -> {} @ {}",
        event.event_class_token, event.state_after_token, event.observed_at
    )
}

/// Beta debugger-host projection rendered into the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebuggerHostBetaProjection {
    pub record_kind: String,
    pub schema_version: u32,
    pub workspace_id: String,
    pub supervisor_session_id: String,
    pub captured_at: String,
    pub notice: String,
    pub sessions: Vec<DebuggerHostBetaSessionRow>,
    pub honesty_marker_present: bool,
    pub export_safe_summary: String,
}

impl DebuggerHostBetaProjection {
    /// Project a debugger-host beta surface from a runtime support packet.
    pub fn project(packet: &DebugSessionSupportPacket) -> Self {
        let sessions: Vec<DebuggerHostBetaSessionRow> = packet
            .session_rows
            .iter()
            .map(DebuggerHostBetaSessionRow::project)
            .collect();
        let honesty_marker_present = sessions.iter().any(|row| row.state_requires_disclosure);
        Self {
            record_kind: DEBUGGER_HOST_BETA_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: DEBUGGER_HOST_BETA_PROJECTION_SCHEMA_VERSION,
            workspace_id: packet.workspace_id.clone(),
            supervisor_session_id: packet.supervisor_session_id.clone(),
            captured_at: packet.captured_at.clone(),
            notice: DEBUGGER_HOST_BETA_NOTICE.to_owned(),
            sessions,
            honesty_marker_present,
            export_safe_summary: packet.export_safe_summary.clone(),
        }
    }

    /// Deterministic plaintext block for the support-export clipboard action.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Debugger host (beta)\n");
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Supervisor: {}\n", self.supervisor_session_id));
        out.push_str(&format!("Captured at: {}\n", self.captured_at));
        out.push_str(&format!("Notice: {}\n", self.notice));
        out.push_str(&format!("Sessions: {}\n", self.sessions.len()));
        for row in &self.sessions {
            out.push_str(&format!("\nSession: {}\n", row.session_id));
            out.push_str(&format!(
                "  Mode: {} | Adapter: {} ({})\n",
                row.mode_token, row.adapter_label, row.adapter_id
            ));
            out.push_str(&format!("  Transport: {}\n", row.transport_class_token));
            out.push_str(&format!(
                "  Target: {} [{}]\n",
                row.target_label, row.canonical_target_id
            ));
            out.push_str(&format!(
                "  State: {} (disclosure: {})\n",
                row.state_class_token, row.state_requires_disclosure
            ));
            if let Some(token) = &row.negotiation_outcome_token {
                out.push_str(&format!("  Negotiation outcome: {token}\n"));
            }
            if !row.agreed_capability_tokens.is_empty() {
                out.push_str(&format!(
                    "  Agreed capabilities: {}\n",
                    row.agreed_capability_tokens.join(",")
                ));
            }
            if !row.dropped_capability_tokens.is_empty() {
                out.push_str(&format!(
                    "  Dropped capabilities: {}\n",
                    row.dropped_capability_tokens.join(",")
                ));
            }
            out.push_str(&format!(
                "  Restart strikes: {}/{} (reconnects: {})\n",
                row.restart_strike_count, row.restart_budget_in_window, row.reconnect_attempt_count
            ));
            if let Some(quarantine) = &row.quarantine_ref {
                out.push_str(&format!("  Quarantine: {quarantine}\n"));
            }
            if let Some(token) = &row.last_exit_reason_token {
                out.push_str(&format!("  Last exit reason: {token}\n"));
            }
            out.push_str("  Event lineage:\n");
            for line in &row.event_lineage_summary {
                out.push_str(&format!("    - {line}\n"));
            }
            out.push_str(&format!("  Summary: {}\n", row.summary));
        }
        out
    }
}

#[cfg(test)]
mod tests;
