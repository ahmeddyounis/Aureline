//! Canonical M5 command-truth evidence index.
//!
//! The M5 command-governance, capability-state, and rollout packets already
//! prove descriptor parity, result-packet reuse, lifecycle truth, and rollout
//! posture for each claimed depth-surface command family. This module turns
//! those lower-level packets into one downstream-facing index that Help/About,
//! release-center truth, support exports, and public-truth/docs consumers can
//! cite without re-deriving maturity or rollout state from partial inputs.
//!
//! Each row in [`M5CommandTruthIndexPacket`] binds one claimed M5 command family
//! to:
//!
//! - its command-governance row and capability-state row;
//! - the effective lifecycle state that downstream consumers must publish;
//! - the active truth posture (`certified`, `narrowed`, `policy_blocked`,
//!   `retest_pending`, or `underqualified`);
//! - explicit help/About, release-center, support-export, and public-truth
//!   projection rows; and
//! - the evidence refs that later support, release, and docs surfaces join
//!   rather than cloning status text by hand.
//!
//! The index intentionally narrows automatically: a command family may only
//! publish stable wording when authority parity, result-packet reuse, lifecycle
//! visibility, and the effective capability state all remain fully backed. Any
//! weaker posture remains visible as a typed narrowing state instead.

use std::error::Error;
use std::fmt;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::m5_capability_state_truth::{
    current_m5_capability_state_truth_export, M5CapabilityProjectionSurfaceClass,
    M5CapabilityStateClass, M5CapabilityStateTruthPacket, M5CapabilityStateTruthRow,
};
use crate::m5_command_governance::{
    current_m5_command_governance_export, M5CommandGovernancePacket, M5CommandGovernanceRow,
};
use crate::m5_rollout_inventory::M5RolloutConsumerSurfaceClass;

#[cfg(test)]
mod tests;

/// Stable record-kind tag carried by [`M5CommandTruthIndexPacket`].
pub const M5_COMMAND_TRUTH_INDEX_RECORD_KIND: &str = "m5_command_truth_index_packet";

/// Schema version for M5 command-truth index packets.
pub const M5_COMMAND_TRUTH_INDEX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the M5 command-truth index schema.
pub const M5_COMMAND_TRUTH_INDEX_SCHEMA_REF: &str =
    "schemas/commands/m5_command_truth_index.schema.json";

/// Repo-relative path of the companion doc.
pub const M5_COMMAND_TRUTH_INDEX_DOC_REF: &str = "docs/commands/m5_command_truth_index.md";

/// Repo-relative path of the checked fixture directory.
pub const M5_COMMAND_TRUTH_INDEX_FIXTURE_DIR: &str = "fixtures/commands/m5_command_truth_index";

/// Repo-relative path of the checked packet artifact.
pub const M5_COMMAND_TRUTH_INDEX_PACKET_REF: &str =
    "artifacts/commands/m5_command_truth_index/packet.json";

/// Repo-relative path of the checked support export.
pub const M5_COMMAND_TRUTH_INDEX_SUPPORT_EXPORT_REF: &str =
    "artifacts/commands/m5_command_truth_index/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_COMMAND_TRUTH_INDEX_SUMMARY_REF: &str =
    "artifacts/commands/m5_command_truth_index/summary.md";

/// Stable packet id used by the seeded export.
pub const M5_COMMAND_TRUTH_INDEX_PACKET_ID: &str = "m5-command-truth-index:stable:0001";

/// Stable support-export id used by [`M5CommandTruthIndexSupportExport`].
pub const M5_COMMAND_TRUTH_INDEX_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-command-truth-index:0001";

const GENERATED_AT: &str = "2026-06-12T00:00:00Z";

static SEEDED_PACKET: OnceLock<M5CommandTruthIndexPacket> = OnceLock::new();

/// Downstream surface that must ingest command-truth posture from this index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CommandTruthSurfaceClass {
    /// In-product Help/About or similar lifecycle-disclosure chrome.
    HelpAbout,
    /// Release-center or claim-review surface.
    ReleaseCenter,
    /// Support export or escalation packet.
    SupportExport,
    /// Public-truth / docs publication surface.
    PublicTruth,
}

impl M5CommandTruthSurfaceClass {
    /// Required surface coverage per command row.
    pub const ALL: [Self; 4] = [
        Self::HelpAbout,
        Self::ReleaseCenter,
        Self::SupportExport,
        Self::PublicTruth,
    ];

    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about",
            Self::ReleaseCenter => "release_center",
            Self::SupportExport => "support_export",
            Self::PublicTruth => "public_truth",
        }
    }
}

/// Effective downstream truth posture for one claimed M5 command family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CommandTruthStateClass {
    /// Fully backed and allowed to publish stable wording.
    Certified,
    /// Narrowed because the effective lifecycle state remains below stable.
    Narrowed,
    /// Narrowed because policy or a winning kill switch blocks stable posture.
    PolicyBlocked,
    /// Narrowed because freshness or retest status lapsed.
    RetestPending,
    /// Narrowed because descriptor/result/lifecycle evidence is incomplete.
    Underqualified,
}

impl M5CommandTruthStateClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Narrowed => "narrowed",
            Self::PolicyBlocked => "policy_blocked",
            Self::RetestPending => "retest_pending",
            Self::Underqualified => "underqualified",
        }
    }
}

/// One downstream-facing projection row backed by the canonical command truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CommandTruthSurfaceProjectionRow {
    /// Surface that consumes this row.
    pub surface_class: M5CommandTruthSurfaceClass,
    /// Stable projection ref consumed by the surface.
    pub surface_ref: String,
    /// Upstream capability/rollout projection ref this surface reuses.
    pub source_projection_ref: String,
    /// Effective lifecycle state the consumer must render.
    pub effective_state_class: M5CapabilityStateClass,
    /// Whether stable wording is still allowed on this surface.
    pub stable_wording_visible: bool,
    /// Whether support wording remains visible on this surface.
    pub support_wording_visible: bool,
    /// Machine-readable narrowing reasons the surface must disclose.
    pub visible_narrowing_reason_codes: Vec<String>,
}

/// One command row in the canonical M5 command-truth evidence index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CommandTruthIndexRow {
    /// Stable command id anchoring the family.
    pub command_id: String,
    /// Stable capability id shared across support, docs, and rollout.
    pub capability_id: String,
    /// Human-facing label.
    pub display_label: String,
    /// Current owner ref.
    pub owner_ref: String,
    /// Rollout ring shown by downstream truth surfaces.
    pub rollout_ring: String,
    /// Named cohort shown by downstream truth surfaces.
    pub cohort: String,
    /// Review or expiry date carried by rollout truth.
    pub review_or_expiry_date: String,
    /// Active lifecycle label that downstream truth surfaces publish.
    pub lifecycle_label: String,
    /// Support posture from the canonical descriptor/lifecycle lane.
    pub support_class: String,
    /// Effective capability-state class after narrowing.
    pub effective_state_class: M5CapabilityStateClass,
    /// Effective downstream truth posture.
    pub truth_state_class: M5CommandTruthStateClass,
    /// Whether authority parity remains fully backed.
    pub authority_parity_complete: bool,
    /// Whether result-packet reuse and export joins remain fully backed.
    pub result_packet_reuse_complete: bool,
    /// Whether lifecycle state remains visible on all downstream claim surfaces.
    pub lifecycle_truth_visible: bool,
    /// Whether stable wording is still allowed anywhere downstream.
    pub stable_wording_allowed: bool,
    /// Active kill-switch source class when present.
    pub active_kill_switch_source: Option<String>,
    /// Settings projection ref downstream diagnostics/help surfaces may quote.
    pub settings_projection_ref: String,
    /// Help/About projection ref.
    pub help_about_projection_ref: String,
    /// Diagnostics projection ref.
    pub diagnostics_projection_ref: String,
    /// Release-center projection ref.
    pub release_center_projection_ref: String,
    /// Support-export projection ref.
    pub support_export_projection_ref: String,
    /// Public-truth projection ref.
    pub public_truth_projection_ref: String,
    /// Evidence refs downstream surfaces join instead of cloning state text.
    pub evidence_refs: Vec<String>,
    /// Machine-readable findings from upstream packets. Empty means conforming.
    pub finding_codes: Vec<String>,
    /// Ordered downstream projection rows.
    pub surface_rows: Vec<M5CommandTruthSurfaceProjectionRow>,
    /// Reviewable rationale for the published posture.
    pub rationale: String,
}

/// Packet summary for support and release consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CommandTruthIndexSummary {
    /// Number of command rows under audit.
    pub command_count: usize,
    /// Number of downstream truth-surface rows.
    pub truth_surface_row_count: usize,
    /// Number of certified rows.
    pub certified_command_count: usize,
    /// Number of lifecycle-narrowed rows.
    pub narrowed_command_count: usize,
    /// Number of policy-blocked rows.
    pub policy_blocked_command_count: usize,
    /// Number of retest-pending rows.
    pub retest_pending_command_count: usize,
    /// Number of underqualified rows.
    pub underqualified_command_count: usize,
    /// Number of rows carrying an active kill switch.
    pub active_kill_switch_command_count: usize,
    /// Number of rows still allowed to publish stable wording.
    pub stable_wording_allowed_command_count: usize,
    /// Number of total findings.
    pub finding_count: usize,
}

/// Canonical M5 command-truth evidence index packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CommandTruthIndexPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Schema ref for this packet.
    pub schema_ref: String,
    /// Companion doc ref.
    pub doc_ref: String,
    /// Upstream command-governance source ref.
    pub source_command_governance_ref: String,
    /// Upstream capability-state source ref.
    pub source_capability_state_ref: String,
    /// Upstream rollout inventory source ref.
    pub source_rollout_inventory_ref: String,
    /// Ordered command-truth rows.
    pub rows: Vec<M5CommandTruthIndexRow>,
    /// Roll-up counts.
    pub summary: M5CommandTruthIndexSummary,
}

impl M5CommandTruthIndexPacket {
    /// Renders a compact Markdown summary for checked artifacts.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Command Truth Index\n\n");
        out.push_str("| Metric | Value |\n|---|---:|\n");
        out.push_str(&format!("| Commands | {} |\n", self.summary.command_count));
        out.push_str(&format!(
            "| Truth-surface rows | {} |\n",
            self.summary.truth_surface_row_count
        ));
        out.push_str(&format!(
            "| Certified rows | {} |\n",
            self.summary.certified_command_count
        ));
        out.push_str(&format!(
            "| Narrowed rows | {} |\n",
            self.summary.narrowed_command_count
        ));
        out.push_str(&format!(
            "| Policy-blocked rows | {} |\n",
            self.summary.policy_blocked_command_count
        ));
        out.push_str(&format!(
            "| Retest-pending rows | {} |\n",
            self.summary.retest_pending_command_count
        ));
        out.push_str(&format!(
            "| Underqualified rows | {} |\n",
            self.summary.underqualified_command_count
        ));
        out.push_str(&format!(
            "| Active kill switches | {} |\n",
            self.summary.active_kill_switch_command_count
        ));
        out.push_str(&format!(
            "| Stable wording allowed | {} |\n",
            self.summary.stable_wording_allowed_command_count
        ));
        out.push_str(&format!(
            "| Findings | {} |\n\n",
            self.summary.finding_count
        ));

        out.push_str("| Command | State | Truth posture | Stable wording | Help/About | Release-center | Support | Public truth |\n");
        out.push_str("|---|---|---|---|---|---|---|---|\n");
        for row in &self.rows {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                row.command_id,
                row.effective_state_class.display_label(),
                row.truth_state_class.as_str(),
                if row.stable_wording_allowed { "allowed" } else { "narrowed" },
                row.help_about_projection_ref,
                row.release_center_projection_ref,
                row.support_export_projection_ref,
                row.public_truth_projection_ref
            ));
        }
        out.push('\n');
        out
    }
}

/// Support-export wrapper for the M5 command-truth evidence index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CommandTruthIndexSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet schema ref.
    pub schema_ref: String,
    /// Case ids useful for support joins.
    pub case_ids: Vec<String>,
    /// Quoted command-truth packet.
    pub packet: M5CommandTruthIndexPacket,
}

impl M5CommandTruthIndexSupportExport {
    /// Builds a deterministic support-export wrapper from a packet.
    pub fn from_packet(support_export_id: String, packet: M5CommandTruthIndexPacket) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.source_command_governance_ref.clone(),
            packet.source_capability_state_ref.clone(),
            packet.source_rollout_inventory_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.command_id.clone());
            case_ids.push(row.capability_id.clone());
            case_ids.push(row.owner_ref.clone());
            case_ids.push(row.settings_projection_ref.clone());
            case_ids.push(row.help_about_projection_ref.clone());
            case_ids.push(row.diagnostics_projection_ref.clone());
            case_ids.push(row.release_center_projection_ref.clone());
            case_ids.push(row.support_export_projection_ref.clone());
            case_ids.push(row.public_truth_projection_ref.clone());
            case_ids.extend(row.evidence_refs.iter().cloned());
            for surface in &row.surface_rows {
                case_ids.push(surface.surface_ref.clone());
                case_ids.push(surface.source_projection_ref.clone());
            }
        }
        case_ids.sort();
        case_ids.dedup();
        Self {
            record_kind: "m5_command_truth_index_support_export".to_string(),
            schema_version: 1,
            support_export_id,
            schema_ref: M5_COMMAND_TRUTH_INDEX_SCHEMA_REF.to_string(),
            case_ids,
            packet,
        }
    }
}

/// Validation error raised by [`validate_m5_command_truth_index_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5CommandTruthIndexValidationError {
    /// The packet has no rows.
    NoRows,
    /// A command row is missing one of the required downstream truth surfaces.
    MissingTruthSurface {
        /// Command id that regressed.
        command_id: String,
        /// Missing surface token.
        surface_class: String,
    },
    /// A row is missing one of the required projection refs.
    MissingProjectionRef {
        /// Command id that regressed.
        command_id: String,
        /// Projection field that regressed.
        field: String,
    },
    /// A row that is not certified still publishes stable wording.
    StableWordingOverclaim {
        /// Command id that regressed.
        command_id: String,
        /// Surface that regressed.
        surface_class: String,
    },
    /// A certified row is not actually backed by stable lifecycle and parity evidence.
    CertifiedRowNotBacked {
        /// Command id that regressed.
        command_id: String,
    },
    /// A truth-state classification contradicts the effective capability state.
    TruthStateMismatch {
        /// Command id that regressed.
        command_id: String,
    },
    /// Evidence refs disappeared from a row.
    MissingEvidenceRefs {
        /// Command id that regressed.
        command_id: String,
    },
}

impl fmt::Display for M5CommandTruthIndexValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoRows => write!(f, "m5 command truth index packet has no rows"),
            Self::MissingTruthSurface {
                command_id,
                surface_class,
            } => write!(
                f,
                "command {command_id} is missing downstream truth surface {surface_class}"
            ),
            Self::MissingProjectionRef { command_id, field } => write!(
                f,
                "command {command_id} is missing required projection ref {field}"
            ),
            Self::StableWordingOverclaim {
                command_id,
                surface_class,
            } => write!(
                f,
                "command {command_id} still publishes stable wording on {surface_class}"
            ),
            Self::CertifiedRowNotBacked { command_id } => write!(
                f,
                "command {command_id} is marked certified without stable lifecycle/parity backing"
            ),
            Self::TruthStateMismatch { command_id } => write!(
                f,
                "command {command_id} has a truth-state classification that contradicts its lifecycle state"
            ),
            Self::MissingEvidenceRefs { command_id } => write!(
                f,
                "command {command_id} is missing downstream evidence refs"
            ),
        }
    }
}

impl Error for M5CommandTruthIndexValidationError {}

fn capability_row<'a>(
    packet: &'a M5CapabilityStateTruthPacket,
    command_id: &str,
) -> &'a M5CapabilityStateTruthRow {
    packet
        .rows
        .iter()
        .find(|row| row.command_id == command_id)
        .expect("every command-governance row has a capability-state row")
}

fn capability_projection_ref(
    row: &M5CapabilityStateTruthRow,
    surface_class: M5CapabilityProjectionSurfaceClass,
) -> String {
    row.projection_rows
        .iter()
        .find(|projection| projection.surface_class == surface_class)
        .map(|projection| projection.projection_ref.clone())
        .unwrap_or_else(|| format!("projection:{}:{}", row.command_id, surface_class.as_str()))
}

fn capability_projection(
    row: &M5CapabilityStateTruthRow,
    surface_class: M5CapabilityProjectionSurfaceClass,
) -> crate::m5_capability_state_truth::M5CapabilityProjectionRow {
    row.projection_rows
        .iter()
        .find(|projection| projection.surface_class == surface_class)
        .cloned()
        .expect("validated capability-state row must cover required projection surfaces")
}

fn rollout_surface_ref(
    row: &crate::m5_rollout_inventory::M5RolloutInventoryRow,
    surface_class: M5RolloutConsumerSurfaceClass,
) -> String {
    row.surfaced_in
        .iter()
        .find(|projection| projection.surface_class == surface_class)
        .map(|projection| projection.surface_ref.clone())
        .unwrap_or_else(|| format!("rollout:{}:{}", row.command_id, surface_class.as_str()))
}

fn authority_parity_complete(row: &M5CommandGovernanceRow) -> bool {
    row.surface_rows.iter().all(|surface| {
        surface.preview_parity_preserved
            && surface.approval_parity_preserved
            && surface.disabled_reason_parity_preserved
            && surface.approval_parity_packet.no_bypass_rule_preserved
            && surface.approval_parity_packet.support_export_safe
            && surface
                .disabled_reason_packets
                .iter()
                .all(|packet| packet.support_export_safe)
    })
}

fn result_packet_reuse_complete(row: &M5CommandGovernanceRow) -> bool {
    let result = &row.result_packet_governance;
    let activity_parity_ok = !result.joins_activity_center
        || row
            .surface_rows
            .iter()
            .all(|surface| surface.activity_join_parity_preserved);
    result.preserves_copy_safe_summary
        && result.preserves_raw_packet_export
        && result.joins_support_export
        && result.joins_release_evidence
        && row
            .surface_rows
            .iter()
            .all(|surface| surface.result_packet_parity_preserved && surface.export_join_parity_preserved)
        && activity_parity_ok
}

fn lifecycle_truth_visible(row: &M5CapabilityStateTruthRow) -> bool {
    [
        M5CapabilityProjectionSurfaceClass::HelpAbout,
        M5CapabilityProjectionSurfaceClass::DocsPack,
        M5CapabilityProjectionSurfaceClass::ReleaseRow,
        M5CapabilityProjectionSurfaceClass::WorkflowBundle,
        M5CapabilityProjectionSurfaceClass::ProfileExport,
        M5CapabilityProjectionSurfaceClass::SupportPacket,
    ]
    .into_iter()
    .all(|surface_class| {
        let projection = capability_projection(row, surface_class);
        projection.export_safe
            && !projection.inspect_detail_ref.is_empty()
            && (!projection.dependency_marker_refs.is_empty() || projection.dependency_markers_visible)
    })
}

fn truth_state(
    governance_row: &M5CommandGovernanceRow,
    capability_row: &M5CapabilityStateTruthRow,
) -> M5CommandTruthStateClass {
    if !authority_parity_complete(governance_row)
        || !result_packet_reuse_complete(governance_row)
        || !lifecycle_truth_visible(capability_row)
    {
        return M5CommandTruthStateClass::Underqualified;
    }

    match capability_row.effective_state_class {
        M5CapabilityStateClass::Stable => M5CommandTruthStateClass::Certified,
        M5CapabilityStateClass::DisabledByPolicy => M5CommandTruthStateClass::PolicyBlocked,
        M5CapabilityStateClass::RetestPending => M5CommandTruthStateClass::RetestPending,
        M5CapabilityStateClass::Labs
        | M5CapabilityStateClass::Preview
        | M5CapabilityStateClass::Beta
        | M5CapabilityStateClass::Deprecated
        | M5CapabilityStateClass::Removed => M5CommandTruthStateClass::Narrowed,
    }
}

fn narrowing_reason_codes(
    governance_row: &M5CommandGovernanceRow,
    capability_row: &M5CapabilityStateTruthRow,
    truth_state_class: M5CommandTruthStateClass,
) -> Vec<String> {
    let mut reasons = Vec::new();
    match truth_state_class {
        M5CommandTruthStateClass::Certified => {}
        M5CommandTruthStateClass::Narrowed => {
            reasons.push("lifecycle_below_stable".to_string());
        }
        M5CommandTruthStateClass::PolicyBlocked => {
            reasons.push("policy_blocked".to_string());
        }
        M5CommandTruthStateClass::RetestPending => {
            reasons.push("proof_freshness_lapsed".to_string());
        }
        M5CommandTruthStateClass::Underqualified => {
            reasons.push("descriptor_or_result_or_lifecycle_evidence_incomplete".to_string());
        }
    }
    reasons.extend(
        capability_row
            .dependency_markers
            .iter()
            .map(|marker| marker.marker_class.as_str().to_string()),
    );
    reasons.extend(governance_row.finding_codes.iter().cloned());
    reasons.extend(capability_row.finding_codes.iter().cloned());
    reasons.sort();
    reasons.dedup();
    reasons
}

fn surface_projection_rows(
    governance_row: &M5CommandGovernanceRow,
    capability_row: &M5CapabilityStateTruthRow,
    truth_state_class: M5CommandTruthStateClass,
) -> Vec<M5CommandTruthSurfaceProjectionRow> {
    let help_about = capability_projection(capability_row, M5CapabilityProjectionSurfaceClass::HelpAbout);
    let release = capability_projection(capability_row, M5CapabilityProjectionSurfaceClass::ReleaseRow);
    let support = capability_projection(capability_row, M5CapabilityProjectionSurfaceClass::SupportPacket);
    let public_truth = capability_projection(capability_row, M5CapabilityProjectionSurfaceClass::DocsPack);
    let stable_wording_visible = matches!(truth_state_class, M5CommandTruthStateClass::Certified);
    let support_wording_visible = help_about.support_wording_visible;
    let reasons = narrowing_reason_codes(governance_row, capability_row, truth_state_class);

    vec![
        M5CommandTruthSurfaceProjectionRow {
            surface_class: M5CommandTruthSurfaceClass::HelpAbout,
            surface_ref: rollout_surface_ref(
                &governance_row.rollout_governance,
                M5RolloutConsumerSurfaceClass::HelpAbout,
            ),
            source_projection_ref: help_about.projection_ref.clone(),
            effective_state_class: capability_row.effective_state_class,
            stable_wording_visible,
            support_wording_visible: help_about.support_wording_visible,
            visible_narrowing_reason_codes: reasons.clone(),
        },
        M5CommandTruthSurfaceProjectionRow {
            surface_class: M5CommandTruthSurfaceClass::ReleaseCenter,
            surface_ref: release.projection_ref.clone(),
            source_projection_ref: release.projection_ref.clone(),
            effective_state_class: capability_row.effective_state_class,
            stable_wording_visible,
            support_wording_visible: release.support_wording_visible,
            visible_narrowing_reason_codes: reasons.clone(),
        },
        M5CommandTruthSurfaceProjectionRow {
            surface_class: M5CommandTruthSurfaceClass::SupportExport,
            surface_ref: rollout_surface_ref(
                &governance_row.rollout_governance,
                M5RolloutConsumerSurfaceClass::SupportExport,
            ),
            source_projection_ref: support.projection_ref.clone(),
            effective_state_class: capability_row.effective_state_class,
            stable_wording_visible,
            support_wording_visible: support.support_wording_visible,
            visible_narrowing_reason_codes: reasons.clone(),
        },
        M5CommandTruthSurfaceProjectionRow {
            surface_class: M5CommandTruthSurfaceClass::PublicTruth,
            surface_ref: public_truth.projection_ref.clone(),
            source_projection_ref: public_truth.projection_ref,
            effective_state_class: capability_row.effective_state_class,
            stable_wording_visible,
            support_wording_visible,
            visible_narrowing_reason_codes: reasons,
        },
    ]
}

fn evidence_refs(
    packet: &M5CommandGovernancePacket,
    governance_row: &M5CommandGovernanceRow,
    capability_packet: &M5CapabilityStateTruthPacket,
    capability_row: &M5CapabilityStateTruthRow,
) -> Vec<String> {
    let mut refs = vec![
        packet.packet_id.clone(),
        packet.schema_ref.clone(),
        capability_packet.packet_id.clone(),
        capability_packet.schema_ref.clone(),
        governance_row.command_revision_ref.clone(),
        governance_row.capability_class_ref.clone(),
        governance_row.lifecycle_disclosure.lifecycle_ref.clone(),
        governance_row.lifecycle_disclosure.rollout_state_ref.clone(),
        governance_row.rollout_governance.rollout_state_ref.clone(),
        capability_row.capability_row_id.clone(),
        capability_row.lifecycle_ref.clone(),
        capability_row.rollout_state_ref.clone(),
        capability_projection_ref(capability_row, M5CapabilityProjectionSurfaceClass::SettingsRow),
        capability_projection_ref(capability_row, M5CapabilityProjectionSurfaceClass::HelpAbout),
        capability_projection_ref(capability_row, M5CapabilityProjectionSurfaceClass::ReleaseRow),
        capability_projection_ref(capability_row, M5CapabilityProjectionSurfaceClass::SupportPacket),
        capability_projection_ref(capability_row, M5CapabilityProjectionSurfaceClass::DocsPack),
        rollout_surface_ref(
            &governance_row.rollout_governance,
            M5RolloutConsumerSurfaceClass::HelpAbout,
        ),
        rollout_surface_ref(
            &governance_row.rollout_governance,
            M5RolloutConsumerSurfaceClass::SupportExport,
        ),
        rollout_surface_ref(
            &governance_row.rollout_governance,
            M5RolloutConsumerSurfaceClass::Diagnostics,
        ),
        rollout_surface_ref(
            &governance_row.rollout_governance,
            M5RolloutConsumerSurfaceClass::DocsRelease,
        ),
    ];
    refs.extend(governance_row.finding_codes.iter().cloned());
    refs.extend(capability_row.finding_codes.iter().cloned());
    for outcome in &governance_row.result_packet_governance.outcome_rows {
        refs.push(outcome.export_safe_summary_ref.clone());
        refs.push(outcome.support_export_case_ref.clone());
        refs.push(outcome.release_evidence_ref.clone());
    }
    refs.extend(
        capability_row
            .dependency_markers
            .iter()
            .map(|marker| marker.marker_ref.clone()),
    );
    refs.sort();
    refs.dedup();
    refs
}

fn build_row(
    governance_row: &M5CommandGovernanceRow,
    capability_row: &M5CapabilityStateTruthRow,
    command_packet: &M5CommandGovernancePacket,
    capability_packet: &M5CapabilityStateTruthPacket,
) -> M5CommandTruthIndexRow {
    let truth_state_class = truth_state(governance_row, capability_row);
    let settings_projection_ref =
        capability_projection_ref(capability_row, M5CapabilityProjectionSurfaceClass::SettingsRow);
    let help_about_projection_ref = rollout_surface_ref(
        &governance_row.rollout_governance,
        M5RolloutConsumerSurfaceClass::HelpAbout,
    );
    let release_center_projection_ref =
        capability_projection_ref(capability_row, M5CapabilityProjectionSurfaceClass::ReleaseRow);
    let support_export_projection_ref = rollout_surface_ref(
        &governance_row.rollout_governance,
        M5RolloutConsumerSurfaceClass::SupportExport,
    );
    let public_truth_projection_ref =
        capability_projection_ref(capability_row, M5CapabilityProjectionSurfaceClass::DocsPack);
    let active_kill_switch_source = governance_row
        .rollout_governance
        .active_kill_switches()
        .first()
        .map(|kill| kill.source_class.as_str().to_string());
    let authority_parity_complete = authority_parity_complete(governance_row);
    let result_packet_reuse_complete = result_packet_reuse_complete(governance_row);
    let lifecycle_truth_visible = lifecycle_truth_visible(capability_row);
    let stable_wording_allowed = matches!(truth_state_class, M5CommandTruthStateClass::Certified)
        && capability_row.effective_state_class == M5CapabilityStateClass::Stable;
    let mut finding_codes = governance_row.finding_codes.clone();
    finding_codes.extend(capability_row.finding_codes.iter().cloned());
    finding_codes.sort();
    finding_codes.dedup();
    let evidence_refs = evidence_refs(
        command_packet,
        governance_row,
        capability_packet,
        capability_row,
    );
    let rationale = match truth_state_class {
        M5CommandTruthStateClass::Certified => format!(
            "{} remains fully backed across descriptor parity, result reuse, lifecycle truth, and stable rollout state.",
            governance_row.display_label()
        ),
        M5CommandTruthStateClass::Narrowed => format!(
            "{} remains visible with a narrower {} lifecycle label instead of inheriting stable wording.",
            governance_row.display_label(),
            capability_row.effective_state_class.display_label()
        ),
        M5CommandTruthStateClass::PolicyBlocked => format!(
            "{} remains visible as policy-blocked; downstream surfaces must not overclaim stable support.",
            governance_row.display_label()
        ),
        M5CommandTruthStateClass::RetestPending => format!(
            "{} remains visible as retest pending because freshness-backed command evidence lapsed.",
            governance_row.display_label()
        ),
        M5CommandTruthStateClass::Underqualified => format!(
            "{} narrows automatically because command-authority, result-packet, or lifecycle evidence is incomplete.",
            governance_row.display_label()
        ),
    };

    M5CommandTruthIndexRow {
        command_id: governance_row.command_id.clone(),
        capability_id: governance_row.rollout_governance.capability_id.clone(),
        display_label: governance_row.rollout_governance.display_label.clone(),
        owner_ref: governance_row.rollout_governance.owner_ref.clone(),
        rollout_ring: governance_row.rollout_governance.rollout_ring.clone(),
        cohort: governance_row.rollout_governance.cohort.clone(),
        review_or_expiry_date: governance_row.rollout_governance.review_or_expiry_date.clone(),
        lifecycle_label: capability_row.effective_state_class.display_label().to_string(),
        support_class: governance_row.lifecycle_disclosure.support_class.clone(),
        effective_state_class: capability_row.effective_state_class,
        truth_state_class,
        authority_parity_complete,
        result_packet_reuse_complete,
        lifecycle_truth_visible,
        stable_wording_allowed,
        active_kill_switch_source,
        settings_projection_ref,
        help_about_projection_ref,
        diagnostics_projection_ref: rollout_surface_ref(
            &governance_row.rollout_governance,
            M5RolloutConsumerSurfaceClass::Diagnostics,
        ),
        release_center_projection_ref,
        support_export_projection_ref,
        public_truth_projection_ref,
        evidence_refs,
        finding_codes,
        surface_rows: surface_projection_rows(governance_row, capability_row, truth_state_class),
        rationale,
    }
}

fn build_summary(rows: &[M5CommandTruthIndexRow]) -> M5CommandTruthIndexSummary {
    M5CommandTruthIndexSummary {
        command_count: rows.len(),
        truth_surface_row_count: rows.iter().map(|row| row.surface_rows.len()).sum(),
        certified_command_count: rows
            .iter()
            .filter(|row| row.truth_state_class == M5CommandTruthStateClass::Certified)
            .count(),
        narrowed_command_count: rows
            .iter()
            .filter(|row| row.truth_state_class == M5CommandTruthStateClass::Narrowed)
            .count(),
        policy_blocked_command_count: rows
            .iter()
            .filter(|row| row.truth_state_class == M5CommandTruthStateClass::PolicyBlocked)
            .count(),
        retest_pending_command_count: rows
            .iter()
            .filter(|row| row.truth_state_class == M5CommandTruthStateClass::RetestPending)
            .count(),
        underqualified_command_count: rows
            .iter()
            .filter(|row| row.truth_state_class == M5CommandTruthStateClass::Underqualified)
            .count(),
        active_kill_switch_command_count: rows
            .iter()
            .filter(|row| row.active_kill_switch_source.is_some())
            .count(),
        stable_wording_allowed_command_count: rows
            .iter()
            .filter(|row| row.stable_wording_allowed)
            .count(),
        finding_count: rows.iter().map(|row| row.finding_codes.len()).sum(),
    }
}

/// Builds the seeded M5 command-truth evidence index packet.
pub fn seeded_m5_command_truth_index_packet() -> M5CommandTruthIndexPacket {
    SEEDED_PACKET
        .get_or_init(|| {
            let command_packet = current_m5_command_governance_export()
                .expect("checked M5 command-governance export validates");
            let capability_packet = current_m5_capability_state_truth_export()
                .expect("checked M5 capability-state truth export validates");

            let rows = command_packet
                .rows
                .iter()
                .map(|governance_row| {
                    let capability_row = capability_row(&capability_packet, &governance_row.command_id);
                    build_row(
                        governance_row,
                        capability_row,
                        &command_packet,
                        &capability_packet,
                    )
                })
                .collect::<Vec<_>>();
            let summary = build_summary(&rows);

            M5CommandTruthIndexPacket {
                record_kind: M5_COMMAND_TRUTH_INDEX_RECORD_KIND.to_string(),
                schema_version: M5_COMMAND_TRUTH_INDEX_SCHEMA_VERSION,
                packet_id: M5_COMMAND_TRUTH_INDEX_PACKET_ID.to_string(),
                generated_at: GENERATED_AT.to_string(),
                schema_ref: M5_COMMAND_TRUTH_INDEX_SCHEMA_REF.to_string(),
                doc_ref: M5_COMMAND_TRUTH_INDEX_DOC_REF.to_string(),
                source_command_governance_ref: command_packet.schema_ref.clone(),
                source_capability_state_ref: capability_packet.schema_ref.clone(),
                source_rollout_inventory_ref: command_packet.source_rollout_inventory_ref.clone(),
                rows,
                summary,
            }
        })
        .clone()
}

/// Validates a seeded or caller-provided M5 command-truth evidence index packet.
pub fn validate_m5_command_truth_index_packet(
    packet: &M5CommandTruthIndexPacket,
) -> Result<(), Vec<M5CommandTruthIndexValidationError>> {
    let mut errors = Vec::new();
    if packet.rows.is_empty() {
        errors.push(M5CommandTruthIndexValidationError::NoRows);
    }

    for row in &packet.rows {
        for required in M5CommandTruthSurfaceClass::ALL {
            if !row
                .surface_rows
                .iter()
                .any(|surface| surface.surface_class == required)
            {
                errors.push(M5CommandTruthIndexValidationError::MissingTruthSurface {
                    command_id: row.command_id.clone(),
                    surface_class: required.as_str().to_string(),
                });
            }
        }

        for (field, value) in [
            ("settings_projection_ref", row.settings_projection_ref.as_str()),
            ("help_about_projection_ref", row.help_about_projection_ref.as_str()),
            (
                "diagnostics_projection_ref",
                row.diagnostics_projection_ref.as_str(),
            ),
            (
                "release_center_projection_ref",
                row.release_center_projection_ref.as_str(),
            ),
            (
                "support_export_projection_ref",
                row.support_export_projection_ref.as_str(),
            ),
            ("public_truth_projection_ref", row.public_truth_projection_ref.as_str()),
        ] {
            if value.is_empty() {
                errors.push(M5CommandTruthIndexValidationError::MissingProjectionRef {
                    command_id: row.command_id.clone(),
                    field: field.to_string(),
                });
            }
        }

        if row.evidence_refs.is_empty() {
            errors.push(M5CommandTruthIndexValidationError::MissingEvidenceRefs {
                command_id: row.command_id.clone(),
            });
        }

        if row.truth_state_class == M5CommandTruthStateClass::Certified
            && !(row.effective_state_class == M5CapabilityStateClass::Stable
                && row.authority_parity_complete
                && row.result_packet_reuse_complete
                && row.lifecycle_truth_visible
                && row.finding_codes.is_empty()
                && row.stable_wording_allowed)
        {
            errors.push(M5CommandTruthIndexValidationError::CertifiedRowNotBacked {
                command_id: row.command_id.clone(),
            });
        }

        for surface in &row.surface_rows {
            if !row.stable_wording_allowed && surface.stable_wording_visible {
                errors.push(M5CommandTruthIndexValidationError::StableWordingOverclaim {
                    command_id: row.command_id.clone(),
                    surface_class: surface.surface_class.as_str().to_string(),
                });
            }
        }

        let state_matches = match row.truth_state_class {
            M5CommandTruthStateClass::Certified => row.effective_state_class == M5CapabilityStateClass::Stable,
            M5CommandTruthStateClass::Narrowed => matches!(
                row.effective_state_class,
                M5CapabilityStateClass::Labs
                    | M5CapabilityStateClass::Preview
                    | M5CapabilityStateClass::Beta
                    | M5CapabilityStateClass::Deprecated
                    | M5CapabilityStateClass::Removed
            ),
            M5CommandTruthStateClass::PolicyBlocked => {
                row.effective_state_class == M5CapabilityStateClass::DisabledByPolicy
            }
            M5CommandTruthStateClass::RetestPending => {
                row.effective_state_class == M5CapabilityStateClass::RetestPending
            }
            M5CommandTruthStateClass::Underqualified => !row.stable_wording_allowed,
        };
        if !state_matches {
            errors.push(M5CommandTruthIndexValidationError::TruthStateMismatch {
                command_id: row.command_id.clone(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Returns the current seeded M5 command-truth evidence index export.
pub fn current_m5_command_truth_index_export(
) -> Result<M5CommandTruthIndexPacket, Vec<M5CommandTruthIndexValidationError>> {
    let packet = seeded_m5_command_truth_index_packet();
    validate_m5_command_truth_index_packet(&packet)?;
    Ok(packet)
}

trait GovernanceRowDisplayLabel {
    fn display_label(&self) -> &str;
}

impl GovernanceRowDisplayLabel for M5CommandGovernanceRow {
    fn display_label(&self) -> &str {
        &self.rollout_governance.display_label
    }
}
