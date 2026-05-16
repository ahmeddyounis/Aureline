//! Shell projection for runtime replay packs.
//!
//! This module is the first consumer over the canonical
//! [`aureline_support::runtime_evidence::RuntimeReplayPackSupportExport`].
//! The shell does not own replay-pack truth; it projects the
//! support-minted records into deterministic plaintext rows reviewers can
//! paste into a support bundle without re-deriving fidelity, privilege, or
//! reopen-decision tokens from log text.

use serde::{Deserialize, Serialize};

use aureline_support::runtime_evidence::{RuntimeReplayPack, RuntimeReplayPackSupportExport};

/// Stable record-kind tag carried in serialized panels.
pub const RUNTIME_REPLAY_PACK_PANEL_RECORD_KIND: &str = "runtime_replay_pack_panel_record";

/// Schema version of the shell panel projection.
pub const RUNTIME_REPLAY_PACK_PANEL_SCHEMA_VERSION: u32 = 1;

/// Header notice rendered above the rows. The string is part of the closed
/// contract the support reader quotes when scanning a bundle.
pub const RUNTIME_REPLAY_PACK_PANEL_NOTICE: &str =
    "Runtime replay packs: packs whose reopen decision is not allow_replay \
     cannot be re-fired against the live target. Mutating and privileged \
     captures are always gated to allow_inspect_no_rerun.";

/// One reviewable replay-pack row, projected from the support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeReplayPackPanelRow {
    pub replay_pack_id: String,
    pub workspace_id: String,
    pub created_at: String,
    pub lane_token: String,
    pub evidence_kind_token: String,
    pub evidence_packet_id: String,
    pub subject_ref: String,
    pub fidelity_token: String,
    pub fidelity_label: String,
    pub subject_privilege_token: String,
    pub reopen_decision_token: String,
    pub forbids_live_rerun: bool,
    pub comparator_blocks_replay: bool,
    pub comparator_compatibility_token: String,
    pub comparator_incompatibility_reason_tokens: Vec<String>,
    pub artefact_class_tokens: Vec<String>,
    pub artefact_refs: Vec<String>,
    pub summary: String,
}

impl RuntimeReplayPackPanelRow {
    fn project(pack: &RuntimeReplayPack) -> Self {
        Self {
            replay_pack_id: pack.replay_pack_id.clone(),
            workspace_id: pack.workspace_id.clone(),
            created_at: pack.created_at.clone(),
            lane_token: pack.evidence_packet.lane_token.clone(),
            evidence_kind_token: pack.evidence_packet.evidence_kind_token.clone(),
            evidence_packet_id: pack.evidence_packet.evidence_packet_id.clone(),
            subject_ref: pack.evidence_packet.subject_ref.clone(),
            fidelity_token: pack.fidelity_token.clone(),
            fidelity_label: pack.fidelity.label().to_owned(),
            subject_privilege_token: pack.subject_privilege_token.clone(),
            reopen_decision_token: pack.reopen_decision_token.clone(),
            forbids_live_rerun: pack.forbids_live_rerun,
            comparator_blocks_replay: pack.comparator_blocks_replay,
            comparator_compatibility_token: pack.replay_comparison.compatibility_token.clone(),
            comparator_incompatibility_reason_tokens: pack
                .replay_comparison
                .incompatibility_reason_tokens
                .clone(),
            artefact_class_tokens: pack
                .artefacts
                .iter()
                .map(|a| a.artefact_class_token.clone())
                .collect(),
            artefact_refs: pack
                .artefacts
                .iter()
                .map(|a| a.artefact_ref.clone())
                .collect(),
            summary: pack.summary.clone(),
        }
    }
}

/// Runtime replay-pack panel rendered by the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeReplayPackPanel {
    pub record_kind: String,
    pub schema_version: u32,
    pub support_export_ref: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub notice: String,
    pub rows: Vec<RuntimeReplayPackPanelRow>,
    pub any_row_forbids_live_rerun: bool,
    pub any_row_comparator_blocks_replay: bool,
    pub every_row_covers_required_artefact_classes: bool,
    pub fidelity_class_tokens_present: Vec<String>,
    pub reopen_decision_tokens_present: Vec<String>,
}

impl RuntimeReplayPackPanel {
    /// Build the shell panel from a support-export bundle.
    pub fn from_support_export(export: &RuntimeReplayPackSupportExport) -> Self {
        let rows: Vec<RuntimeReplayPackPanelRow> = export
            .packs
            .iter()
            .map(RuntimeReplayPackPanelRow::project)
            .collect();
        Self {
            record_kind: RUNTIME_REPLAY_PACK_PANEL_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_REPLAY_PACK_PANEL_SCHEMA_VERSION,
            support_export_ref: export.support_export_id.clone(),
            workspace_id: export.workspace_id.clone(),
            generated_at: export.generated_at.clone(),
            notice: RUNTIME_REPLAY_PACK_PANEL_NOTICE.to_owned(),
            rows,
            any_row_forbids_live_rerun: export.any_pack_forbids_live_rerun,
            any_row_comparator_blocks_replay: export.any_pack_comparator_blocks_replay,
            every_row_covers_required_artefact_classes: export
                .every_pack_covers_required_artefact_classes,
            fidelity_class_tokens_present: export.fidelity_class_tokens_present.clone(),
            reopen_decision_tokens_present: export.reopen_decision_tokens_present.clone(),
        }
    }

    /// Deterministic plaintext block for the support-export clipboard and CLI
    /// surfaces. Rows render in their canonical order so the support reader
    /// can grep tokens reliably.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Runtime replay packs\n");
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Support export: {}\n", self.support_export_ref));
        out.push_str(&format!("Captured at: {}\n", self.generated_at));
        out.push_str(&format!("Notice: {}\n", self.notice));
        out.push_str(&format!(
            "Rows: {} (forbids_live_rerun={}, comparator_blocks_replay={}, artefact_coverage={})\n",
            self.rows.len(),
            self.any_row_forbids_live_rerun,
            self.any_row_comparator_blocks_replay,
            self.every_row_covers_required_artefact_classes,
        ));
        out.push_str(&format!(
            "Fidelity classes: {}\n",
            self.fidelity_class_tokens_present.join(",")
        ));
        out.push_str(&format!(
            "Reopen decisions: {}\n",
            self.reopen_decision_tokens_present.join(",")
        ));
        for row in &self.rows {
            out.push_str(&format!(
                "\n[{}] {} ({}) lane={} kind={}\n",
                row.fidelity_token,
                row.reopen_decision_token,
                row.replay_pack_id,
                row.lane_token,
                row.evidence_kind_token,
            ));
            out.push_str(&format!("  Subject: {}\n", row.subject_ref));
            out.push_str(&format!(
                "  Privilege: {} | Forbids live rerun: {}\n",
                row.subject_privilege_token, row.forbids_live_rerun,
            ));
            out.push_str(&format!(
                "  Evidence packet: {} | Comparator: {} (blocks={})\n",
                row.evidence_packet_id,
                row.comparator_compatibility_token,
                row.comparator_blocks_replay,
            ));
            if !row.comparator_incompatibility_reason_tokens.is_empty() {
                out.push_str(&format!(
                    "  Comparator reasons: {}\n",
                    row.comparator_incompatibility_reason_tokens.join(","),
                ));
            }
            for (class, value) in row
                .artefact_class_tokens
                .iter()
                .zip(row.artefact_refs.iter())
            {
                out.push_str(&format!("  Artefact[{class}]: {value}\n"));
            }
            out.push_str(&format!("  Summary: {}\n", row.summary));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_support::runtime_evidence::{
        seeded_runtime_replay_pack_support_export, RuntimeReplayPackSeededScenario,
    };

    #[test]
    fn panel_rows_match_support_export_packs() {
        let export = seeded_runtime_replay_pack_support_export(
            "replay-pack-support:shell-test",
            "2026-05-15T19:03:00Z",
        );
        let panel = RuntimeReplayPackPanel::from_support_export(&export);
        assert_eq!(panel.record_kind, RUNTIME_REPLAY_PACK_PANEL_RECORD_KIND);
        assert_eq!(panel.support_export_ref, export.support_export_id);
        assert_eq!(panel.workspace_id, export.workspace_id);
        assert_eq!(panel.rows.len(), export.packs.len());
        assert!(panel.any_row_forbids_live_rerun);
        assert!(panel.any_row_comparator_blocks_replay);
        assert!(panel.every_row_covers_required_artefact_classes);
        for (row, pack) in panel.rows.iter().zip(export.packs.iter()) {
            assert_eq!(row.replay_pack_id, pack.replay_pack_id);
            assert_eq!(row.subject_ref, pack.evidence_packet.subject_ref);
            assert_eq!(row.fidelity_token, pack.fidelity_token);
            assert_eq!(row.reopen_decision_token, pack.reopen_decision_token);
        }
    }

    #[test]
    fn plaintext_block_quotes_every_closed_token() {
        let export = seeded_runtime_replay_pack_support_export(
            "replay-pack-support:shell-plaintext",
            "2026-05-15T19:03:00Z",
        );
        let panel = RuntimeReplayPackPanel::from_support_export(&export);
        let text = panel.render_plaintext();
        for token in [
            "exact",
            "compatible",
            "layout_only",
            "allow_replay",
            "allow_inspect_no_rerun",
            "read_only",
            "mutating",
            "privileged",
            "transcript_ref",
            "runtime_log_ref",
            "evidence_packet_ref",
            "context_provenance_ref",
            "incompatible_capsule_drift",
            "incompatible_trust_state_downgraded",
        ] {
            assert!(text.contains(token), "missing token '{token}'");
        }
        for scenario in RuntimeReplayPackSeededScenario::ALL {
            assert!(
                text.contains(scenario.as_str()),
                "panel must quote scenario id '{}'",
                scenario.as_str(),
            );
        }
    }

    #[test]
    fn panel_carries_no_raw_secret_markers() {
        let export = seeded_runtime_replay_pack_support_export(
            "replay-pack-support:shell-redaction",
            "2026-05-15T19:03:00Z",
        );
        let panel = RuntimeReplayPackPanel::from_support_export(&export);
        let json = serde_json::to_string(&panel).expect("serialize panel");
        assert!(!json.contains("BEARER"));
        assert!(!json.contains("AWS_SECRET_ACCESS_KEY"));
        assert!(!json.contains("SSH_PRIVATE_KEY"));
        assert!(!json.contains("LD_LIBRARY_PATH"));
    }
}
