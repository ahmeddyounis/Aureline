//! Support/export projection for runtime host lanes and fault domains.
//!
//! This module consumes the runtime topology-inspector records from
//! `aureline-runtime` and projects them into a metadata-only support packet.
//! The packet keeps fault-domain ids, restart-budget state, preserved
//! checkpoints, crash-loop or quarantine banners, reattach review decisions,
//! lane-filtered event markers, and partial-truth result refs together so
//! support exports match what users saw in the shell.

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    CrashLoopQuarantineBanner, FaultDomainRestartCard, HostLaneHealthClass, HostLaneRecord,
    LaneFilteredEventViewer, ReattachReviewSheet, TopologyInspectorRecord,
    FAULT_DOMAIN_RESTART_CARD_RECORD_KIND, HOST_TOPOLOGY_SCHEMA_VERSION,
};

/// Stable record-kind tag for the support fault-domain view packet.
pub const FAULT_DOMAIN_VIEW_PACKET_RECORD_KIND: &str = "fault_domain_view_packet";

/// Stable record-kind tag for one support fault-domain view row.
pub const FAULT_DOMAIN_VIEW_ROW_RECORD_KIND: &str = "fault_domain_view_row";

/// One support/export row for a host lane and its fault-domain state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FaultDomainViewRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Host lane id.
    pub host_lane_ref: String,
    /// Plain-language host family label.
    pub host_family_label: String,
    /// Host health token.
    pub health_token: String,
    /// Fault-domain id.
    pub fault_domain_id: String,
    /// Fault-domain class token.
    pub fault_domain_token: String,
    /// Restart-budget ref.
    pub restart_budget_ref: String,
    /// Restart-budget state token.
    pub restart_budget_state_token: String,
    /// Counted restart strikes.
    pub restart_strike_count: u32,
    /// Automatic restarts allowed in the window.
    pub restart_budget_in_window: u32,
    /// Capabilities affected by this lane state.
    pub affected_capability_tokens: Vec<String>,
    /// Checkpoints preserved for recovery or review.
    pub preserved_checkpoint_refs: Vec<String>,
    /// Result refs that are stale, partial, rebuilding, or awaiting refresh.
    pub partial_truth_result_refs: Vec<String>,
    /// Surface tokens where this lane appeared.
    pub surface_tokens: Vec<String>,
    /// Event ids retained for this lane.
    pub lane_event_ids: Vec<String>,
    /// Reattach review decision, when this lane participates in one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reattach_decision_token: Option<String>,
    /// Crash-loop or quarantine banner id, when active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crash_banner_ref: Option<String>,
    /// True when the row contains metadata only.
    pub export_safe: bool,
}

impl FaultDomainViewRow {
    fn from_lane(
        lane: &HostLaneRecord,
        card: &FaultDomainRestartCard,
        inspector: &TopologyInspectorRecord,
        event_viewer: &LaneFilteredEventViewer,
        reattach_reviews: &[ReattachReviewSheet],
        crash_banner: Option<&CrashLoopQuarantineBanner>,
    ) -> Self {
        let mut surface_tokens = inspector
            .results
            .iter()
            .filter(|result| result.host_lane_ids.contains(&lane.lane_id))
            .map(|result| result.surface_token.clone())
            .collect::<Vec<_>>();
        surface_tokens.sort();
        surface_tokens.dedup();

        let lane_event_ids = event_viewer
            .rows
            .iter()
            .filter(|row| row.host_lane_ref == lane.lane_id)
            .map(|row| row.event_id.clone())
            .collect::<Vec<_>>();

        let reattach_decision_token = reattach_reviews
            .iter()
            .find(|review| {
                review.previous_host_lane_ref == lane.lane_id
                    || review.current_host_lane_ref == lane.lane_id
            })
            .map(|review| review.decision_token.clone());

        let mut partial_truth_result_refs = lane.stale_result_refs.clone();
        for result in &inspector.results {
            if result.host_lane_ids.contains(&lane.lane_id)
                && result.freshness_class.needs_disclosure()
                && !partial_truth_result_refs.contains(&result.result_id)
            {
                partial_truth_result_refs.push(result.result_id.clone());
            }
        }

        Self {
            record_kind: FAULT_DOMAIN_VIEW_ROW_RECORD_KIND.to_owned(),
            schema_version: HOST_TOPOLOGY_SCHEMA_VERSION,
            row_id: format!("fault-domain-view:{}", lane.lane_id),
            host_lane_ref: lane.lane_id.clone(),
            host_family_label: lane.family_label.clone(),
            health_token: lane.health_token.clone(),
            fault_domain_id: lane.fault_domain_id.clone(),
            fault_domain_token: lane.fault_domain_token.clone(),
            restart_budget_ref: lane.restart_budget_ref.clone(),
            restart_budget_state_token: card.restart_budget_state_token.clone(),
            restart_strike_count: lane.restart_strike_count,
            restart_budget_in_window: lane.restart_budget_in_window,
            affected_capability_tokens: lane.affected_capability_tokens.clone(),
            preserved_checkpoint_refs: lane.preserved_checkpoint_refs.clone(),
            partial_truth_result_refs,
            surface_tokens,
            lane_event_ids,
            reattach_decision_token,
            crash_banner_ref: crash_banner.map(|banner| banner.banner_id.clone()),
            export_safe: true,
        }
    }
}

/// Support/export packet for host-lane fault-domain state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FaultDomainViewPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Source topology inspector id.
    pub topology_inspector_ref: String,
    /// Lane rows.
    pub rows: Vec<FaultDomainViewRow>,
    /// Fault-domain restart cards.
    pub restart_cards: Vec<FaultDomainRestartCard>,
    /// Reattach reviews included in the export.
    pub reattach_reviews: Vec<ReattachReviewSheet>,
    /// Crash-loop or quarantine banners included in the export.
    pub crash_banners: Vec<CrashLoopQuarantineBanner>,
    /// Lane-filtered event viewer.
    pub event_viewer: LaneFilteredEventViewer,
    /// Count of rows with stale or partial visible truth.
    pub partial_truth_row_count: u32,
    /// Count of rows that block healthy presentation.
    pub blocked_healthy_claim_count: u32,
    /// Export-safe packet summary.
    pub export_safe_summary: String,
}

impl FaultDomainViewPacket {
    /// Builds a packet from runtime topology records.
    pub fn from_topology(
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
        inspector: &TopologyInspectorRecord,
        reattach_reviews: Vec<ReattachReviewSheet>,
        event_viewer: LaneFilteredEventViewer,
    ) -> Self {
        let restart_cards = inspector
            .lanes
            .iter()
            .map(|lane| {
                FaultDomainRestartCard::from_lane(
                    format!("fault-domain-card:{}", lane.lane_id),
                    lane,
                )
            })
            .collect::<Vec<_>>();

        let crash_banners = inspector
            .lanes
            .iter()
            .filter_map(|lane| {
                CrashLoopQuarantineBanner::maybe_from_lane(
                    format!("crash-banner:{}", lane.lane_id),
                    lane,
                    lane.stale_result_refs.clone(),
                    lane.quarantine_ref
                        .clone()
                        .unwrap_or_else(|| format!("evidence:{}", lane.lane_id)),
                )
            })
            .collect::<Vec<_>>();

        let rows = inspector
            .lanes
            .iter()
            .zip(restart_cards.iter())
            .map(|(lane, card)| {
                let crash_banner = crash_banners
                    .iter()
                    .find(|banner| banner.failing_host_lane_ref == lane.lane_id);
                FaultDomainViewRow::from_lane(
                    lane,
                    card,
                    inspector,
                    &event_viewer,
                    &reattach_reviews,
                    crash_banner,
                )
            })
            .collect::<Vec<_>>();

        let partial_truth_row_count = rows
            .iter()
            .filter(|row| !row.partial_truth_result_refs.is_empty())
            .count() as u32;
        let blocked_healthy_claim_count = inspector
            .lanes
            .iter()
            .filter(|lane| lane.health_class.blocks_healthy_claim())
            .count() as u32;

        Self {
            record_kind: FAULT_DOMAIN_VIEW_PACKET_RECORD_KIND.to_owned(),
            schema_version: HOST_TOPOLOGY_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            workspace_id: inspector.workspace_id.clone(),
            generated_at: generated_at.into(),
            topology_inspector_ref: inspector.inspector_id.clone(),
            rows,
            restart_cards,
            reattach_reviews,
            crash_banners,
            event_viewer,
            partial_truth_row_count,
            blocked_healthy_claim_count,
            export_safe_summary:
                "Fault-domain view packet is metadata-only and preserves host-lane partial-truth markers."
                    .to_owned(),
        }
    }

    /// Returns true when the packet contains only metadata-safe rows.
    pub fn is_export_safe(&self) -> bool {
        self.rows.iter().all(|row| row.export_safe)
            && self
                .event_viewer
                .rows
                .iter()
                .all(|row| row.redaction_class_token == "metadata_safe_default")
    }

    /// Validates support packet parity with topology and event state.
    pub fn validate(&self, inspector: &TopologyInspectorRecord) -> Vec<FaultDomainViewViolation> {
        let mut violations = Vec::new();
        if self.record_kind != FAULT_DOMAIN_VIEW_PACKET_RECORD_KIND {
            push_fault_view_violation(
                &mut violations,
                "record_kind",
                &self.packet_id,
                "record_kind must be fault_domain_view_packet",
            );
        }
        if self.rows.len() != inspector.lanes.len() {
            push_fault_view_violation(
                &mut violations,
                "rows",
                &self.packet_id,
                "packet must include one row per host lane",
            );
        }
        if self.restart_cards.len() != inspector.lanes.len() {
            push_fault_view_violation(
                &mut violations,
                "restart_cards",
                &self.packet_id,
                "packet must include one restart card per host lane",
            );
        }
        if !self.event_viewer.rows_preserve_provenance() {
            push_fault_view_violation(
                &mut violations,
                "event_viewer.rows",
                &self.packet_id,
                "event rows must preserve ids, lane markers, and provenance refs",
            );
        }
        for card in &self.restart_cards {
            if card.record_kind != FAULT_DOMAIN_RESTART_CARD_RECORD_KIND {
                push_fault_view_violation(
                    &mut violations,
                    "restart_cards.record_kind",
                    &card.card_id,
                    "restart card record_kind is invalid",
                );
            }
        }
        for lane in &inspector.lanes {
            if lane.health_class.blocks_healthy_claim()
                && !self
                    .crash_banners
                    .iter()
                    .any(|banner| banner.failing_host_lane_ref == lane.lane_id)
            {
                push_fault_view_violation(
                    &mut violations,
                    "crash_banners",
                    &lane.lane_id,
                    "blocked healthy states must have crash-loop or quarantine banners",
                );
            }
            if lane.health_class != HostLaneHealthClass::Healthy
                && !self.rows.iter().any(|row| {
                    row.host_lane_ref == lane.lane_id
                        && (!row.partial_truth_result_refs.is_empty()
                            || row.restart_strike_count > 0)
                })
            {
                push_fault_view_violation(
                    &mut violations,
                    "rows.partial_truth_result_refs",
                    &lane.lane_id,
                    "degraded lanes must preserve restart or partial-truth evidence",
                );
            }
        }
        violations
    }

    /// Renders deterministic plaintext suitable for support clipboard export.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Host lane and fault-domain packet\n");
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Topology: {}\n", self.topology_inspector_ref));
        out.push_str(&format!("Generated: {}\n", self.generated_at));
        out.push_str(&format!("Rows: {}\n", self.rows.len()));
        out.push_str(&format!(
            "Partial-truth rows: {}\n",
            self.partial_truth_row_count
        ));
        out.push_str(&format!(
            "Healthy claims blocked: {}\n",
            self.blocked_healthy_claim_count
        ));
        for row in &self.rows {
            out.push_str(&format!("\nLane: {}\n", row.host_lane_ref));
            out.push_str(&format!("  Host: {}\n", row.host_family_label));
            out.push_str(&format!("  Health: {}\n", row.health_token));
            out.push_str(&format!("  Fault domain: {}\n", row.fault_domain_id));
            out.push_str(&format!(
                "  Restart budget: {} {}/{}\n",
                row.restart_budget_state_token,
                row.restart_strike_count,
                row.restart_budget_in_window
            ));
            if !row.partial_truth_result_refs.is_empty() {
                out.push_str(&format!(
                    "  Partial truth: {}\n",
                    row.partial_truth_result_refs.join(",")
                ));
            }
            if let Some(decision) = &row.reattach_decision_token {
                out.push_str(&format!("  Reattach: {decision}\n"));
            }
            if let Some(banner) = &row.crash_banner_ref {
                out.push_str(&format!("  Banner: {banner}\n"));
            }
        }
        out
    }
}

/// Validation issue emitted by fault-domain view packet checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FaultDomainViewViolation {
    /// Field or path that failed validation.
    pub path: String,
    /// Subject record reference.
    pub subject_ref: String,
    /// Export-safe validation summary.
    pub summary: String,
}

fn push_fault_view_violation(
    violations: &mut Vec<FaultDomainViewViolation>,
    path: impl Into<String>,
    subject_ref: impl Into<String>,
    summary: impl Into<String>,
) {
    violations.push(FaultDomainViewViolation {
        path: path.into(),
        subject_ref: subject_ref.into(),
        summary: summary.into(),
    });
}

/// Builds the canonical seeded support fault-domain view packet.
pub fn seeded_fault_domain_view_packet() -> FaultDomainViewPacket {
    let inspector = aureline_runtime::seeded_host_topology_inspector();
    let reattach = aureline_runtime::seeded_reattach_review_sheet();
    let event_viewer = aureline_runtime::seeded_lane_filtered_event_viewer();
    FaultDomainViewPacket::from_topology(
        "fault-domain-view:seed",
        "2026-05-18T12:05:00Z",
        &inspector,
        vec![reattach],
        event_viewer,
    )
}
