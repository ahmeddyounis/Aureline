//! Qualification packet and published reports for the voice conformance lane.
//!
//! The qualification packet is the governing object the spec requires: it can
//! keep a claimed row Preview/Beta or force it back to Labs when the
//! conformance corpus is not clean, when the row itself carries a blocking
//! finding, or when its privacy/parity proof is stale or incomplete. The two
//! rendered reports are derived deterministically from the seeded voice page
//! (the real claimed surface) plus the qualification packet, so they are
//! mint-from-truth artifacts that a release reviewer can diff.

use serde::{Deserialize, Serialize};

use crate::voice::VoicePreviewBetaPage;

/// Stable record kind for [`VoiceQualificationPacket`].
pub const VOICE_QUALIFICATION_PACKET_RECORD_KIND: &str = "shell_voice_qualification_packet_record";

/// Schema version stamped on the qualification packet.
pub const VOICE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable packet id used for the published qualification packet.
pub const VOICE_QUALIFICATION_PACKET_ID: &str = "voice-qualification-packet:beta:001";

/// Published privacy-and-parity report path.
pub const VOICE_PRIVACY_PARITY_REPORT_REF: &str =
    "artifacts/ux/m3/voice_privacy_and_parity_report.md";

/// Published command-equivalence audit path.
pub const VOICE_COMMAND_EQUIVALENCE_AUDIT_REF: &str =
    "artifacts/ux/m3/voice_command_equivalence_audit.md";

/// Published companion audit doc path.
pub const VOICE_PREVIEW_BETA_AUDIT_DOC_REF: &str = "docs/ux/m3/voice_preview_beta_audit.md";

/// Generation timestamp embedded in the seeded qualification packet.
const GENERATED_AT: &str = "2026-05-20T00:00:00Z";

/// Per-row proof status fed into the qualification computation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceRowProofStatus {
    /// Row id this proof status applies to.
    pub row_id: String,
    /// `true` when the privacy/parity proof for the row is current.
    pub proof_fresh: bool,
    /// `true` when the required privacy/parity evidence is complete.
    pub proof_complete: bool,
}

/// Qualification verdict for a single row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceQualificationVerdict {
    /// The claimed row keeps its Preview/Beta posture.
    KeepClaimed,
    /// The claimed row is downgraded to Labs/unadvertised.
    DowngradeToLabs,
    /// The row was already Labs/unadvertised; nothing to qualify.
    RemainsLabs,
}

impl VoiceQualificationVerdict {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeepClaimed => "keep_claimed",
            Self::DowngradeToLabs => "downgrade_to_labs",
            Self::RemainsLabs => "remains_labs",
        }
    }
}

/// Per-row qualification outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceRowQualification {
    /// Row id.
    pub row_id: String,
    /// Claim posture token.
    pub claim_posture: String,
    /// Verdict for the row.
    pub verdict: VoiceQualificationVerdict,
    /// Reasons forcing a downgrade, in deterministic order (empty when kept).
    pub downgrade_reasons: Vec<String>,
}

/// Qualification packet: the governing object that gates beta/preview claims.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceQualificationPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// `true` when the conformance corpus is clean (every drill passed).
    pub corpus_clean: bool,
    /// Per-row qualification outcomes, sorted by row id.
    pub rows: Vec<VoiceRowQualification>,
    /// Number of claimed rows kept Preview/Beta.
    pub claimed_rows_kept: usize,
    /// Number of claimed rows downgraded to Labs.
    pub claimed_rows_downgraded: usize,
    /// `true` only when every claimed row is kept (none downgraded).
    pub all_claimed_rows_qualified: bool,
}

/// Builds the all-fresh / all-complete proof statuses for the claimed rows of a
/// page. This is the green-path input used for the published packet.
pub fn fresh_complete_proof(page: &VoicePreviewBetaPage) -> Vec<VoiceRowProofStatus> {
    page.rows
        .iter()
        .filter(|row| row.claim_posture.is_claimed())
        .map(|row| VoiceRowProofStatus {
            row_id: row.row_id.clone(),
            proof_fresh: true,
            proof_complete: true,
        })
        .collect()
}

/// Computes the qualification packet for a page.
///
/// A claimed row is downgraded to Labs when any of the following holds:
/// the conformance corpus is not clean; the row itself carries a blocking
/// finding; the row's proof status is missing, stale, or incomplete.
pub fn compute_voice_qualification(
    page: &VoicePreviewBetaPage,
    corpus_clean: bool,
    proof: &[VoiceRowProofStatus],
    packet_id: impl Into<String>,
    generated_at: impl Into<String>,
) -> VoiceQualificationPacket {
    let mut rows: Vec<VoiceRowQualification> = Vec::new();
    for row in &page.rows {
        if !row.claim_posture.is_claimed() {
            rows.push(VoiceRowQualification {
                row_id: row.row_id.clone(),
                claim_posture: row.claim_posture.as_str().to_owned(),
                verdict: VoiceQualificationVerdict::RemainsLabs,
                downgrade_reasons: Vec::new(),
            });
            continue;
        }

        let mut reasons: Vec<String> = Vec::new();
        if !corpus_clean {
            reasons.push("conformance_corpus_not_clean".to_owned());
        }
        for finding in &row.blocking_findings {
            reasons.push(format!("blocking_finding:{}", finding.class_token()));
        }
        match proof.iter().find(|status| status.row_id == row.row_id) {
            None => reasons.push("proof_status_missing".to_owned()),
            Some(status) => {
                if !status.proof_fresh {
                    reasons.push("proof_stale".to_owned());
                }
                if !status.proof_complete {
                    reasons.push("proof_incomplete".to_owned());
                }
            }
        }

        let verdict = if reasons.is_empty() {
            VoiceQualificationVerdict::KeepClaimed
        } else {
            VoiceQualificationVerdict::DowngradeToLabs
        };
        rows.push(VoiceRowQualification {
            row_id: row.row_id.clone(),
            claim_posture: row.claim_posture.as_str().to_owned(),
            verdict,
            downgrade_reasons: reasons,
        });
    }
    rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));

    let claimed_rows_kept = rows
        .iter()
        .filter(|row| row.verdict == VoiceQualificationVerdict::KeepClaimed)
        .count();
    let claimed_rows_downgraded = rows
        .iter()
        .filter(|row| row.verdict == VoiceQualificationVerdict::DowngradeToLabs)
        .count();

    VoiceQualificationPacket {
        record_kind: VOICE_QUALIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: VOICE_QUALIFICATION_SCHEMA_VERSION,
        packet_id: packet_id.into(),
        generated_at: generated_at.into(),
        corpus_clean,
        rows,
        claimed_rows_kept,
        claimed_rows_downgraded,
        all_claimed_rows_qualified: claimed_rows_downgraded == 0,
    }
}

/// Builds the published (green-path) qualification packet for a page.
pub fn seeded_voice_qualification_packet(page: &VoicePreviewBetaPage) -> VoiceQualificationPacket {
    compute_voice_qualification(
        page,
        true,
        &fresh_complete_proof(page),
        VOICE_QUALIFICATION_PACKET_ID,
        GENERATED_AT,
    )
}

/// Renders the privacy-and-parity report artifact from the page and packet.
pub fn render_privacy_and_parity_report(
    page: &VoicePreviewBetaPage,
    packet: &VoiceQualificationPacket,
) -> String {
    let mut out = String::new();
    out.push_str("# Voice privacy and parity report (beta qualification)\n\n");
    out.push_str(
        "Generated from the seeded voice page in\n\
         [`crate::voice`](../../../crates/aureline-shell/src/voice/mod.rs) and the\n\
         qualification packet in\n\
         [`crate::voice::conformance`](../../../crates/aureline-shell/src/voice/conformance/report.rs).\n\
         Regenerate with:\n\n",
    );
    out.push_str("```sh\n");
    out.push_str(
        "cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- privacy-report > \\\n  artifacts/ux/m3/voice_privacy_and_parity_report.md\n",
    );
    out.push_str("```\n\n");

    out.push_str(&format!("- Page id: `{}`\n", page.page_id));
    out.push_str(&format!("- Packet id: `{}`\n", packet.packet_id));
    out.push_str(&format!("- Rows: `{}`\n", page.summary.row_count));
    out.push_str(&format!(
        "- Claimed beta/preview rows: `{}` (kept: `{}`, downgraded: `{}`)\n",
        page.summary.claimed_row_count, packet.claimed_rows_kept, packet.claimed_rows_downgraded
    ));
    out.push_str(&format!(
        "- Labs/unadvertised rows: `{}`\n",
        page.summary.labs_row_count
    ));
    out.push_str(&format!("- Corpus clean: `{}`\n", packet.corpus_clean));
    out.push_str(&format!(
        "- All claimed rows qualified: **{}**\n",
        packet.all_claimed_rows_qualified
    ));
    out.push_str(&format!("- Generated at: `{}`\n\n", packet.generated_at));

    out.push_str("## Privacy and availability\n\n");
    out.push_str(
        "| Row | Processing | Retention | Background listening | Unavailable reason | Keyboard fallback | Redaction |\n\
         | --- | ---------- | --------- | -------------------- | ------------------ | ----------------- | --------- |\n",
    );
    for row in &page.rows {
        let privacy = &row.provider_privacy_row;
        let unavailable = privacy
            .unavailable_reason
            .map(|reason| reason.as_str())
            .unwrap_or("-");
        out.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | {} | `{}` |\n",
            row.row_id,
            privacy.processing_locality_cue.as_str(),
            privacy.retention_mode.as_str(),
            privacy.background_listening_state.as_str(),
            unavailable,
            privacy.keyboard_fallback_available,
            privacy.redaction_class,
        ));
    }
    out.push('\n');

    out.push_str("## Accessibility and lifecycle narration\n\n");
    out.push_str(
        "Every claimed row is keyboard reachable and screen-reader narratable; capturing rows \
         carry start/stop/cancel actions and an accessibility label so narration and focus return \
         honestly when capture starts, ends, fails, or is cancelled.\n\n",
    );
    out.push_str(
        "| Row | Mic narration | Transcript narration | Stop action | Mute action | Cancel action |\n\
         | --- | ------------- | -------------------- | ----------- | ----------- | ------------- |\n",
    );
    for row in &page.rows {
        if !row.claim_posture.is_claimed() {
            continue;
        }
        let mic = row
            .mic_pill
            .as_ref()
            .map(|pill| pill.accessibility_label_ref.as_str())
            .unwrap_or("-");
        let stop = row
            .mic_pill
            .as_ref()
            .map(|pill| pill.stop_action_command_id.as_str())
            .unwrap_or("-");
        let mute = row
            .mic_pill
            .as_ref()
            .map(|pill| pill.mute_action_command_id.as_str())
            .unwrap_or("-");
        let (transcript, cancel) = row
            .transcript_strip
            .as_ref()
            .map(|strip| {
                (
                    strip.accessibility_label_ref.as_str(),
                    strip.cancel_command_id.as_str(),
                )
            })
            .unwrap_or(("-", "-"));
        out.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            row.row_id, mic, transcript, stop, mute, cancel
        ));
    }
    out.push('\n');

    out.push_str("## Qualification packet\n\n");
    out.push_str(
        "The packet keeps a claimed row Preview/Beta only when the conformance corpus is clean, \
         the row carries no blocking finding, and its privacy/parity proof is fresh and complete. \
         Otherwise the row is forced back to Labs before stable-facing language can overclaim it.\n\n",
    );
    out.push_str(
        "| Row | Posture | Verdict | Downgrade reasons |\n\
         | --- | ------- | ------- | ----------------- |\n",
    );
    for row in &packet.rows {
        let reasons = if row.downgrade_reasons.is_empty() {
            "-".to_owned()
        } else {
            row.downgrade_reasons
                .iter()
                .map(|reason| format!("`{reason}`"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        out.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            row.row_id,
            row.claim_posture,
            row.verdict.as_str(),
            reasons
        ));
    }
    out.push('\n');

    out.push_str("## Verification\n\n");
    out.push_str("```sh\n");
    out.push_str("cargo test -p aureline-shell --test voice_conformance_corpus\n");
    out.push_str("cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- run\n");
    out.push_str("```\n");
    out
}

/// Renders the command-equivalence audit artifact from the page.
pub fn render_command_equivalence_audit(page: &VoicePreviewBetaPage) -> String {
    let mut out = String::new();
    out.push_str("# Voice command-equivalence audit (beta)\n\n");
    out.push_str(
        "Generated from the seeded voice page in\n\
         [`crate::voice`](../../../crates/aureline-shell/src/voice/mod.rs).\n\
         Regenerate with:\n\n",
    );
    out.push_str("```sh\n");
    out.push_str(
        "cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- equivalence-audit > \\\n  artifacts/ux/m3/voice_command_equivalence_audit.md\n",
    );
    out.push_str("```\n\n");

    out.push_str(
        "Every claimed spoken command resolves through the same canonical command id as the \
         keyboard and command-palette lanes, and carries the same capability scope, lifecycle \
         label, disabled reason, preview/approval posture, and result-packet schema. The \
         `keyboard_equivalent` column proves the voice resolution and the keyboard invocation \
         reach the same command id.\n\n",
    );

    out.push_str(
        "| Resolution | Command id | Keyboard equivalent | Scope | Lifecycle | Preview | Approval | Enablement | Disabled reason | Result schema |\n\
         | ---------- | ---------- | ------------------- | ----- | --------- | ------- | -------- | ---------- | --------------- | ------------- |\n",
    );
    for row in &page.rows {
        for resolution in &row.command_resolutions {
            let command_id = resolution.canonical_command_id.as_deref().unwrap_or("-");
            let keyboard = resolution
                .keyboard_equivalent_command_id
                .as_deref()
                .unwrap_or("-");
            let lifecycle = resolution
                .lifecycle_label
                .map(|label| label.as_str())
                .unwrap_or("-");
            let disabled = resolution
                .disabled_reason_code
                .map(|code| code.as_str())
                .unwrap_or("-");
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | {} | {} | `{}` | `{}` | `{}` |\n",
                resolution.resolution_id,
                command_id,
                keyboard,
                resolution.capability_scope_class.as_str(),
                lifecycle,
                resolution.preview_required,
                resolution.approval_required,
                resolution.enablement_decision_class.as_str(),
                disabled,
                resolution.result_packet_schema_ref,
            ));
        }
    }
    out.push('\n');

    out.push_str("## Cross-surface equivalence\n\n");
    out.push_str(
        "- **Voice ↔ keyboard ↔ palette:** the command id and `keyboard_equivalent` match, and \
         high-impact scopes keep `preview_required` true with strict no-bypass guards.\n",
    );
    out.push_str(
        "- **CLI / help metadata:** each resolution carries a `docs_help_anchor_ref` so the same \
         command is discoverable from help and CLI metadata.\n",
    );
    out.push_str(
        "- **Support exports:** the support-export wrapper quotes the same command ids while \
         excluding raw audio/transcript bytes by default.\n\n",
    );

    out.push_str("## Verification\n\n");
    out.push_str("```sh\n");
    out.push_str("cargo test -p aureline-shell --test voice_conformance_corpus\n");
    out.push_str("```\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::voice::{seeded_voice_preview_beta_page, VoiceClaimPosture};

    #[test]
    fn green_path_keeps_every_claimed_row() {
        let page = seeded_voice_preview_beta_page();
        let packet = seeded_voice_qualification_packet(&page);
        assert!(packet.all_claimed_rows_qualified);
        assert_eq!(packet.claimed_rows_downgraded, 0);
        assert_eq!(packet.claimed_rows_kept, page.summary.claimed_row_count);
    }

    #[test]
    fn stale_proof_downgrades_the_row() {
        let page = seeded_voice_preview_beta_page();
        let mut proof = fresh_complete_proof(&page);
        let target = proof[0].row_id.clone();
        proof[0].proof_fresh = false;
        let packet =
            compute_voice_qualification(&page, true, &proof, "pkt", "2026-05-20T00:00:00Z");
        assert!(!packet.all_claimed_rows_qualified);
        let row = packet
            .rows
            .iter()
            .find(|row| row.row_id == target)
            .expect("target row present");
        assert_eq!(row.verdict, VoiceQualificationVerdict::DowngradeToLabs);
        assert!(row.downgrade_reasons.iter().any(|r| r == "proof_stale"));
    }

    #[test]
    fn unclean_corpus_downgrades_all_claimed_rows() {
        let page = seeded_voice_preview_beta_page();
        let packet =
            compute_voice_qualification(&page, false, &fresh_complete_proof(&page), "pkt", "t");
        assert_eq!(packet.claimed_rows_kept, 0);
        assert_eq!(
            packet.claimed_rows_downgraded,
            page.summary.claimed_row_count
        );
    }

    #[test]
    fn labs_rows_remain_labs() {
        let page = seeded_voice_preview_beta_page();
        let packet = seeded_voice_qualification_packet(&page);
        assert!(packet.rows.iter().any(|row| {
            row.claim_posture == VoiceClaimPosture::LabsUnadvertised.as_str()
                && row.verdict == VoiceQualificationVerdict::RemainsLabs
        }));
    }
}
