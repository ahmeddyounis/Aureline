//! Shell projection for runtime evidence packets.
//!
//! This module is the first consumer over the canonical
//! [`aureline_runtime::RuntimeEvidencePacketSupportExport`]. The shell does
//! not own evidence-packet truth; it projects the runtime-minted records
//! into deterministic plaintext rows reviewers can paste into a support
//! bundle without re-deriving target identity, toolchain lineage, or replay
//! compatibility from log text.

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    RuntimeEvidencePacket, RuntimeEvidencePacketSupportExport, RuntimeEvidenceReplayComparison,
};

/// Stable record-kind tag carried in serialized panels.
pub const RUNTIME_EVIDENCE_PANEL_RECORD_KIND: &str = "runtime_evidence_panel_record";

/// Schema version of the shell panel projection.
pub const RUNTIME_EVIDENCE_PANEL_SCHEMA_VERSION: u32 = 1;

/// Header notice rendered above the rows. The string is part of the closed
/// contract the support reader quotes when scanning a bundle.
pub const RUNTIME_EVIDENCE_PANEL_NOTICE: &str =
    "Runtime evidence packets (beta): rows whose replay compatibility is \
     blocked cannot be re-played without explicit reviewer approval.";

/// One reviewable runtime evidence packet row, joined to its comparison.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeEvidencePanelRow {
    pub evidence_packet_id: String,
    pub workspace_id: String,
    pub lane_token: String,
    pub lane_label: String,
    pub evidence_kind_token: String,
    pub subject_ref: String,
    pub captured_at: String,
    pub execution_context_ref: String,
    pub provenance_record_ref: String,
    pub target_id: String,
    pub target_class_token: String,
    pub toolchain_class_token: String,
    pub environment_capsule_ref: String,
    pub environment_capsule_drift_token: String,
    pub policy_epoch: u64,
    pub trust_state_token: String,
    pub redaction_class_token: String,
    pub compatibility_token: String,
    pub permits_replay_without_review: bool,
    pub blocks_replay: bool,
    pub incompatibility_reason_tokens: Vec<String>,
    pub summary: String,
}

impl RuntimeEvidencePanelRow {
    fn project(
        packet: &RuntimeEvidencePacket,
        comparison: Option<&RuntimeEvidenceReplayComparison>,
    ) -> Self {
        let (compatibility_token, permits_replay_without_review, blocks_replay, reason_tokens) =
            match comparison {
                Some(c) => (
                    c.compatibility_token.clone(),
                    c.permits_replay_without_review,
                    c.blocks_replay,
                    c.incompatibility_reason_tokens.clone(),
                ),
                None => (
                    "unknown_requires_review".to_owned(),
                    false,
                    true,
                    vec!["redaction_class_unsafe".to_owned()],
                ),
            };
        Self {
            evidence_packet_id: packet.evidence_packet_id.clone(),
            workspace_id: packet.workspace_id.clone(),
            lane_token: packet.lane_token.clone(),
            lane_label: packet.lane.label().to_owned(),
            evidence_kind_token: packet.evidence_kind_token.clone(),
            subject_ref: packet.subject_ref.clone(),
            captured_at: packet.captured_at.clone(),
            execution_context_ref: packet.context_provenance.execution_context_ref.clone(),
            provenance_record_ref: packet.context_provenance.provenance_record_ref.clone(),
            target_id: packet.context_provenance.target_id.clone(),
            target_class_token: packet.context_provenance.target_class_token.clone(),
            toolchain_class_token: packet.context_provenance.toolchain_class_token.clone(),
            environment_capsule_ref: packet.context_provenance.environment_capsule_ref.clone(),
            environment_capsule_drift_token: packet
                .context_provenance
                .environment_capsule_drift_token
                .clone(),
            policy_epoch: packet.context_provenance.policy_epoch,
            trust_state_token: packet.context_provenance.trust_state_token.clone(),
            redaction_class_token: packet.redaction_class_token.clone(),
            compatibility_token,
            permits_replay_without_review,
            blocks_replay,
            incompatibility_reason_tokens: reason_tokens,
            summary: packet.summary.clone(),
        }
    }
}

/// Runtime evidence-packet panel rendered by the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeEvidencePanel {
    pub record_kind: String,
    pub schema_version: u32,
    pub support_export_ref: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub notice: String,
    pub rows: Vec<RuntimeEvidencePanelRow>,
    pub any_row_blocks_replay: bool,
    pub any_minor_drift: bool,
    pub redaction_class_token: String,
}

impl RuntimeEvidencePanel {
    /// Build the shell panel directly from a runtime support export.
    pub fn from_support_export(export: &RuntimeEvidencePacketSupportExport) -> Self {
        let rows: Vec<RuntimeEvidencePanelRow> = export
            .packets
            .iter()
            .map(|packet| {
                let comparison = export
                    .comparisons
                    .iter()
                    .find(|c| c.evidence_packet_ref == packet.evidence_packet_id);
                RuntimeEvidencePanelRow::project(packet, comparison)
            })
            .collect();
        Self {
            record_kind: RUNTIME_EVIDENCE_PANEL_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_EVIDENCE_PANEL_SCHEMA_VERSION,
            support_export_ref: export.support_export_id.clone(),
            workspace_id: export.workspace_id.clone(),
            generated_at: export.generated_at.clone(),
            notice: RUNTIME_EVIDENCE_PANEL_NOTICE.to_owned(),
            rows,
            any_row_blocks_replay: export.any_comparison_blocks_replay,
            any_minor_drift: export.any_minor_drift,
            redaction_class_token: export.redaction_class_token.clone(),
        }
    }

    /// Deterministic plaintext block for the support-export clipboard
    /// action and CLI surfaces. Rows render in their canonical order so
    /// the support reader can grep tokens reliably.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Runtime evidence packets (beta)\n");
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Support export: {}\n", self.support_export_ref));
        out.push_str(&format!("Captured at: {}\n", self.generated_at));
        out.push_str(&format!("Redaction: {}\n", self.redaction_class_token));
        out.push_str(&format!("Notice: {}\n", self.notice));
        out.push_str(&format!(
            "Rows: {} (blocks_replay={}, minor_drift={})\n",
            self.rows.len(),
            self.any_row_blocks_replay,
            self.any_minor_drift,
        ));
        for row in &self.rows {
            out.push_str(&format!(
                "\n[{}] {} ({})\n",
                row.lane_token, row.evidence_kind_token, row.evidence_packet_id
            ));
            out.push_str(&format!("  Subject: {}\n", row.subject_ref));
            out.push_str(&format!(
                "  Target: {} ({}) | Toolchain: {}\n",
                row.target_id, row.target_class_token, row.toolchain_class_token,
            ));
            out.push_str(&format!(
                "  Capsule: {} (drift={}) | Policy epoch: {} | Trust: {}\n",
                row.environment_capsule_ref,
                row.environment_capsule_drift_token,
                row.policy_epoch,
                row.trust_state_token,
            ));
            out.push_str(&format!(
                "  Context: {} | Provenance: {}\n",
                row.execution_context_ref, row.provenance_record_ref,
            ));
            out.push_str(&format!(
                "  Replay: {} (permits_replay={}, blocks={})\n",
                row.compatibility_token, row.permits_replay_without_review, row.blocks_replay,
            ));
            if !row.incompatibility_reason_tokens.is_empty() {
                out.push_str(&format!(
                    "  Reasons: {}\n",
                    row.incompatibility_reason_tokens.join(",")
                ));
            }
            out.push_str(&format!("  Summary: {}\n", row.summary));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_runtime::{
        seeded_runtime_evidence_packet_support_export, RuntimeEvidencePacketSeededScenario,
    };

    #[test]
    fn panel_rows_match_support_export_packets() {
        let export = seeded_runtime_evidence_packet_support_export(
            "evpkt-support:shell-test",
            "2026-05-15T19:02:00Z",
        );
        let panel = RuntimeEvidencePanel::from_support_export(&export);
        assert_eq!(panel.record_kind, RUNTIME_EVIDENCE_PANEL_RECORD_KIND);
        assert_eq!(panel.support_export_ref, export.support_export_id);
        assert_eq!(panel.workspace_id, export.workspace_id);
        assert_eq!(panel.rows.len(), export.packets.len());
        assert!(panel.any_row_blocks_replay);
        assert!(panel.any_minor_drift);
        for (row, packet) in panel.rows.iter().zip(export.packets.iter()) {
            assert_eq!(row.evidence_packet_id, packet.evidence_packet_id);
            assert_eq!(row.subject_ref, packet.subject_ref);
            assert_eq!(row.target_id, packet.context_provenance.target_id);
        }
    }

    #[test]
    fn plaintext_block_quotes_every_seeded_compatibility_token() {
        let export = seeded_runtime_evidence_packet_support_export(
            "evpkt-support:shell-plaintext",
            "2026-05-15T19:02:00Z",
        );
        let panel = RuntimeEvidencePanel::from_support_export(&export);
        let text = panel.render_plaintext();
        for token in [
            "compatible_replay",
            "compatible_minor_drift",
            "incompatible_capsule_drift",
            "incompatible_trust_state_downgraded",
            "metadata_safe_default",
        ] {
            assert!(text.contains(token), "missing token '{token}'");
        }
        for scenario in RuntimeEvidencePacketSeededScenario::ALL {
            assert!(
                text.contains(scenario.evidence_kind().as_str()),
                "missing evidence-kind token for {scenario:?}"
            );
        }
    }

    #[test]
    fn panel_rows_carry_no_raw_secret_markers() {
        let export = seeded_runtime_evidence_packet_support_export(
            "evpkt-support:shell-redaction",
            "2026-05-15T19:02:00Z",
        );
        let panel = RuntimeEvidencePanel::from_support_export(&export);
        let json = serde_json::to_string(&panel).expect("serialize panel");
        assert!(!json.contains("BEARER"));
        assert!(!json.contains("AWS_SECRET_ACCESS_KEY"));
        assert!(!json.contains("SSH_PRIVATE_KEY"));
        assert!(!json.contains("LD_LIBRARY_PATH"));
    }
}
