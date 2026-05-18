//! Shell projection for resource-governor and background-work truth.
//!
//! This module consumes [`aureline_runtime::ResourceGovernorSnapshot`] and
//! materializes the status, lane rows, deferred-work banners, pressure cards,
//! override rows, and support-export records that shell surfaces render. The
//! projection deliberately quotes runtime lane, pressure, checkpoint, collapse,
//! and override vocabulary rather than deriving a private shell status model.

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    GovernorHealthState, OverrideDecisionClass, OverrideSheet, PressureInput, QueueLaneState,
    QueueLaneStateFlag, ResourceGovernorSnapshot,
};

/// Schema version stamped on background-work status records.
pub const BACKGROUND_WORK_STATUS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`BackgroundWorkStatusBundle`].
pub const BACKGROUND_WORK_STATUS_BUNDLE_RECORD_KIND: &str = "shell_background_work_status_bundle";

/// Stable record-kind tag for [`BackgroundWorkStatusItem`].
pub const BACKGROUND_WORK_STATUS_ITEM_RECORD_KIND: &str = "shell_background_work_status_item";

/// Stable record-kind tag for [`BackgroundWorkLaneRow`].
pub const BACKGROUND_WORK_LANE_ROW_RECORD_KIND: &str = "shell_background_work_lane_row";

/// Stable record-kind tag for [`DeferredWorkBanner`].
pub const DEFERRED_WORK_BANNER_RECORD_KIND: &str = "shell_deferred_work_banner";

/// Stable record-kind tag for [`BudgetPressureCard`].
pub const BUDGET_PRESSURE_CARD_RECORD_KIND: &str = "shell_budget_pressure_card";

/// Stable record-kind tag for [`OverrideStatusRow`].
pub const OVERRIDE_STATUS_ROW_RECORD_KIND: &str = "shell_background_work_override_row";

/// Stable record-kind tag for [`BackgroundWorkStatusSupportExport`].
pub const BACKGROUND_WORK_STATUS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_background_work_status_support_export";

/// Status-bar projection for the active resource-governor posture.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundWorkStatusItem {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable status item id.
    pub status_item_id: String,
    /// Current value rendered in the status bar.
    pub current_value_label: String,
    /// Current governor state token.
    pub governor_state: String,
    /// Most affected lane token.
    pub most_affected_lane: String,
    /// Total queue depth across all lanes.
    pub total_queue_depth: u32,
    /// Number of lanes whose behavior changed.
    pub affected_lane_count: usize,
    /// Explanation shown in status details.
    pub explanation: String,
    /// Screen-reader label.
    pub accessibility_label: String,
    /// Primary command id for details.
    pub primary_command_id: String,
    /// Surface ref opened by the primary command.
    pub opens_surface_ref: String,
    /// Runtime truth source referenced by this row.
    pub truth_source_ref: String,
}

/// Shell row for one queue lane.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundWorkLaneRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Queue lane token.
    pub lane_token: String,
    /// Queue lane label.
    pub lane_label: String,
    /// Primary state token.
    pub primary_state: String,
    /// Additional state flag tokens.
    pub state_flags: Vec<String>,
    /// Visible health-state token.
    pub visible_state: String,
    /// Queue depth for the lane.
    pub lane_depth: u32,
    /// Queue-age label shown on the row.
    pub queue_age_label: String,
    /// Collapse count shown on the row.
    pub collapse_count: u32,
    /// Checkpoint label when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_label: Option<String>,
    /// Work slowed, paused, coalesced, or denied.
    pub affected_work_label: String,
    /// What remains usable in the current workspace.
    pub remains_usable_label: String,
    /// Reviewable reason for the state.
    pub reason: String,
    /// Replay or resume note when deferred work is not complete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_resume_note: Option<String>,
    /// Row actions.
    pub actions: Vec<String>,
    /// Runtime truth source referenced by this row.
    pub truth_source_ref: String,
}

/// Contextual banner for paused or denied background work.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeferredWorkBanner {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable banner id.
    pub banner_id: String,
    /// Severity token for shell routing.
    pub severity: String,
    /// Banner title.
    pub title: String,
    /// Banner body.
    pub body: String,
    /// Work named by the banner.
    pub paused_or_denied_work: Vec<String>,
    /// Work that remains usable.
    pub still_usable: Vec<String>,
    /// Resume or recovery condition.
    pub resume_condition: String,
    /// Banner actions.
    pub actions: Vec<String>,
    /// Runtime truth source referenced by this banner.
    pub truth_source_ref: String,
}

/// Budget-pressure card for a pressure input.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BudgetPressureCard {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable card id.
    pub card_id: String,
    /// Pressure dimension token.
    pub pressure_dimension: String,
    /// Pressure dimension label.
    pub pressure_dimension_label: String,
    /// Governor state token implied by the pressure input.
    pub pressure_state: String,
    /// Current value label.
    pub current_value_label: String,
    /// Threshold label.
    pub threshold_label: String,
    /// Work shed by this pressure input.
    pub shed_work_label: String,
    /// Protected foreground state label.
    pub protected_state_label: String,
    /// Action label.
    pub action_label: String,
    /// Runtime truth source referenced by this card.
    pub truth_source_ref: String,
}

/// Row explaining an allowed or blocked override.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverrideStatusRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Override id.
    pub override_id: String,
    /// Queue lane token.
    pub lane_token: String,
    /// Decision token.
    pub decision: String,
    /// Override label.
    pub label: String,
    /// Reviewable reason.
    pub reason: String,
    /// Explanation rendered when controls are blocked.
    pub blocked_control_explanation: String,
    /// Action label.
    pub action_label: String,
    /// True when this row is blocked and must not render as a dead control.
    pub blocked: bool,
}

/// Shell support-export projection for background-work status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundWorkStatusSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Source resource-governor snapshot id.
    pub snapshot_id: String,
    /// Active workspace id.
    pub workspace_id: String,
    /// Status item included in the export.
    pub status_item: BackgroundWorkStatusItem,
    /// Lane rows included in the export.
    pub lane_rows: Vec<BackgroundWorkLaneRow>,
    /// Pressure cards included in the export.
    pub pressure_cards: Vec<BudgetPressureCard>,
    /// Override rows included in the export.
    pub override_rows: Vec<OverrideStatusRow>,
    /// True when raw content, paths, command lines, and provider payloads are excluded.
    pub raw_private_material_excluded: bool,
}

/// Complete shell projection for background-work status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackgroundWorkStatusBundle {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Source resource-governor snapshot id.
    pub snapshot_id: String,
    /// Active workspace id.
    pub workspace_id: String,
    /// Status item rendered in the status bar.
    pub status_item: BackgroundWorkStatusItem,
    /// Queue-lane rows rendered in the background-work surface.
    pub lane_rows: Vec<BackgroundWorkLaneRow>,
    /// Contextual banners for paused or denied work.
    pub deferred_work_banners: Vec<DeferredWorkBanner>,
    /// Budget pressure cards.
    pub pressure_cards: Vec<BudgetPressureCard>,
    /// Override rows.
    pub override_rows: Vec<OverrideStatusRow>,
    /// Support-export projection.
    pub support_export: BackgroundWorkStatusSupportExport,
    /// Observation timestamp.
    pub observed_at: String,
}

impl BackgroundWorkStatusBundle {
    /// Projects shell surfaces from a resource-governor snapshot.
    pub fn from_snapshot(snapshot: &ResourceGovernorSnapshot) -> Self {
        let status_item = status_item_from_snapshot(snapshot);
        let lane_rows = snapshot
            .lane_states
            .iter()
            .map(lane_row_from_state)
            .collect::<Vec<_>>();
        let deferred_work_banners = snapshot
            .lane_states
            .iter()
            .filter(|lane| lane_needs_banner(lane))
            .map(|lane| {
                banner_from_lane(
                    snapshot.governor_state,
                    lane,
                    &snapshot.last_transition.exit_condition,
                )
            })
            .collect::<Vec<_>>();
        let pressure_cards = snapshot
            .pressure_inputs
            .iter()
            .map(|pressure| pressure_card_from_input(snapshot, pressure))
            .collect::<Vec<_>>();
        let override_rows = snapshot
            .override_sheets
            .iter()
            .map(override_row_from_sheet)
            .collect::<Vec<_>>();
        let support_export = BackgroundWorkStatusSupportExport {
            record_kind: BACKGROUND_WORK_STATUS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: BACKGROUND_WORK_STATUS_SCHEMA_VERSION,
            snapshot_id: snapshot.snapshot_id.clone(),
            workspace_id: snapshot.workspace_id.clone(),
            status_item: status_item.clone(),
            lane_rows: lane_rows.clone(),
            pressure_cards: pressure_cards.clone(),
            override_rows: override_rows.clone(),
            raw_private_material_excluded: true,
        };
        Self {
            record_kind: BACKGROUND_WORK_STATUS_BUNDLE_RECORD_KIND.to_owned(),
            schema_version: BACKGROUND_WORK_STATUS_SCHEMA_VERSION,
            snapshot_id: snapshot.snapshot_id.clone(),
            workspace_id: snapshot.workspace_id.clone(),
            status_item,
            lane_rows,
            deferred_work_banners,
            pressure_cards,
            override_rows,
            support_export,
            observed_at: snapshot.observed_at.clone(),
        }
    }

    /// Validates shell/status/diagnostics parity for the projection.
    pub fn validate(&self, snapshot: &ResourceGovernorSnapshot) -> BackgroundWorkStatusValidation {
        let mut violations = Vec::new();
        for lane in &snapshot.lane_states {
            let row = self
                .lane_rows
                .iter()
                .find(|row| row.lane_token == lane.lane.as_str());
            match row {
                Some(row) => {
                    if lane.changes_behavior()
                        && (row.affected_work_label.trim().is_empty()
                            || row.remains_usable_label.trim().is_empty())
                    {
                        violations.push(
                            BackgroundWorkStatusViolation::ChangedLaneRowMissingTruth {
                                lane_token: lane.lane.as_str().to_owned(),
                            },
                        );
                    }
                }
                None => violations.push(BackgroundWorkStatusViolation::MissingLaneRow {
                    lane_token: lane.lane.as_str().to_owned(),
                }),
            }
            if lane_needs_banner(lane)
                && !self
                    .deferred_work_banners
                    .iter()
                    .any(|banner| banner.truth_source_ref == truth_source_for_lane(lane))
            {
                violations.push(
                    BackgroundWorkStatusViolation::PausedOrDeniedLaneMissingBanner {
                        lane_token: lane.lane.as_str().to_owned(),
                    },
                );
            }
        }
        for row in &self.override_rows {
            if row.blocked && row.blocked_control_explanation.trim().is_empty() {
                violations.push(
                    BackgroundWorkStatusViolation::BlockedOverrideMissingExplanation {
                        override_id: row.override_id.clone(),
                    },
                );
            }
        }
        if self.support_export.lane_rows.len() != self.lane_rows.len()
            || self.support_export.pressure_cards.len() != self.pressure_cards.len()
            || self.support_export.override_rows.len() != self.override_rows.len()
        {
            violations.push(BackgroundWorkStatusViolation::SupportExportParityDrift);
        }
        BackgroundWorkStatusValidation { violations }
    }
}

/// Validation report for the shell background-work projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackgroundWorkStatusValidation {
    /// Validation violations.
    pub violations: Vec<BackgroundWorkStatusViolation>,
}

impl BackgroundWorkStatusValidation {
    /// Returns true when no validation violations were found.
    pub fn is_ok(&self) -> bool {
        self.violations.is_empty()
    }
}

/// Validation failure for background-work shell projections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundWorkStatusViolation {
    /// A runtime lane is missing from the shell row set.
    MissingLaneRow {
        /// Missing lane token.
        lane_token: String,
    },
    /// A constrained row does not name affected and still-usable work.
    ChangedLaneRowMissingTruth {
        /// Lane token for the incomplete row.
        lane_token: String,
    },
    /// A paused or denied lane lacks a contextual banner.
    PausedOrDeniedLaneMissingBanner {
        /// Lane token for the missing banner.
        lane_token: String,
    },
    /// A blocked override would render as a dead disabled control.
    BlockedOverrideMissingExplanation {
        /// Override id with missing explanation.
        override_id: String,
    },
    /// The support export does not mirror shell rows and cards.
    SupportExportParityDrift,
}

fn status_item_from_snapshot(snapshot: &ResourceGovernorSnapshot) -> BackgroundWorkStatusItem {
    let affected_lane_count = snapshot
        .lane_states
        .iter()
        .filter(|lane| lane.changes_behavior())
        .count();
    let total_queue_depth = snapshot
        .lane_states
        .iter()
        .map(|lane| lane.lane_depth)
        .sum::<u32>();
    let most_affected = snapshot
        .lane_states
        .iter()
        .filter(|lane| lane.changes_behavior())
        .max_by(|left, right| {
            left.lane_depth.cmp(&right.lane_depth).then_with(|| {
                left.oldest_age_seconds
                    .partial_cmp(&right.oldest_age_seconds)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        })
        .or_else(|| snapshot.lane_states.first())
        .expect("resource-governor snapshot must have lane states");
    let current_value_label = format!(
        "{} · {} affected",
        snapshot.governor_state.label(),
        most_affected.lane.label()
    );
    let explanation = format!(
        "{}; {} lanes changed behavior, {} jobs are queued or running.",
        snapshot.status_summary, affected_lane_count, total_queue_depth
    );
    BackgroundWorkStatusItem {
        record_kind: BACKGROUND_WORK_STATUS_ITEM_RECORD_KIND.to_owned(),
        schema_version: BACKGROUND_WORK_STATUS_SCHEMA_VERSION,
        status_item_id: "status.item.background.resource_governor".to_owned(),
        current_value_label: current_value_label.clone(),
        governor_state: snapshot.governor_state.as_str().to_owned(),
        most_affected_lane: most_affected.lane.as_str().to_owned(),
        total_queue_depth,
        affected_lane_count,
        explanation: explanation.clone(),
        accessibility_label: format!(
            "Background work status: {current_value_label}. {explanation}"
        ),
        primary_command_id: "cmd:runtime.resource_governor.inspect".to_owned(),
        opens_surface_ref: "surface.runtime.resource_governor".to_owned(),
        truth_source_ref: snapshot.snapshot_id.clone(),
    }
}

fn lane_row_from_state(state: &QueueLaneState) -> BackgroundWorkLaneRow {
    BackgroundWorkLaneRow {
        record_kind: BACKGROUND_WORK_LANE_ROW_RECORD_KIND.to_owned(),
        schema_version: BACKGROUND_WORK_STATUS_SCHEMA_VERSION,
        row_id: format!("row.background_work.{}", state.lane.as_str()),
        lane_token: state.lane.as_str().to_owned(),
        lane_label: state.lane.label().to_owned(),
        primary_state: state.primary_state.as_str().to_owned(),
        state_flags: state
            .state_flags
            .iter()
            .map(|flag| flag.as_str().to_owned())
            .collect(),
        visible_state: state.visible_state.as_str().to_owned(),
        lane_depth: state.lane_depth,
        queue_age_label: queue_age_label(state.oldest_age_seconds),
        collapse_count: state.collapse_count,
        checkpoint_label: state
            .checkpoint
            .as_ref()
            .map(|checkpoint| checkpoint.boundary_label.clone()),
        affected_work_label: join_or_none(&state.affected_work_labels),
        remains_usable_label: join_or_none(&state.remains_usable),
        reason: state.reason.clone(),
        replay_resume_note: state.replay_resume_note.clone(),
        actions: actions_for_lane(state),
        truth_source_ref: truth_source_for_lane(state),
    }
}

fn banner_from_lane(
    governor_state: GovernorHealthState,
    state: &QueueLaneState,
    resume_condition: &str,
) -> DeferredWorkBanner {
    let work = join_or_none(&state.affected_work_labels);
    let still_usable = join_or_none(&state.remains_usable);
    DeferredWorkBanner {
        record_kind: DEFERRED_WORK_BANNER_RECORD_KIND.to_owned(),
        schema_version: BACKGROUND_WORK_STATUS_SCHEMA_VERSION,
        banner_id: format!("banner.background_work.{}", state.lane.as_str()),
        severity: if state.primary_state == QueueLaneStateFlag::Denied {
            "blocking"
        } else {
            "degraded"
        }
        .to_owned(),
        title: format!("{} constrained", state.lane.label()),
        body: format!(
            "{} is {} while {} is active. {} remains usable.",
            work,
            state.primary_state.as_str(),
            governor_state.label(),
            still_usable
        ),
        paused_or_denied_work: state.affected_work_labels.clone(),
        still_usable: state.remains_usable.clone(),
        resume_condition: resume_condition.to_owned(),
        actions: actions_for_lane(state),
        truth_source_ref: truth_source_for_lane(state),
    }
}

fn pressure_card_from_input(
    snapshot: &ResourceGovernorSnapshot,
    input: &PressureInput,
) -> BudgetPressureCard {
    BudgetPressureCard {
        record_kind: BUDGET_PRESSURE_CARD_RECORD_KIND.to_owned(),
        schema_version: BACKGROUND_WORK_STATUS_SCHEMA_VERSION,
        card_id: format!("card.background_pressure.{}", input.dimension.as_str()),
        pressure_dimension: input.dimension.as_str().to_owned(),
        pressure_dimension_label: input.dimension.label().to_owned(),
        pressure_state: input.state.as_str().to_owned(),
        current_value_label: input.current_value_label.clone(),
        threshold_label: input.threshold_label.clone(),
        shed_work_label: affected_lane_work_label(snapshot, input),
        protected_state_label:
            "Editing, save, explicit cancellation, quick open, and navigation remain protected."
                .to_owned(),
        action_label: "Inspect queue".to_owned(),
        truth_source_ref: format!(
            "{}#pressure:{}",
            snapshot.snapshot_id,
            input.dimension.as_str()
        ),
    }
}

fn override_row_from_sheet(sheet: &OverrideSheet) -> OverrideStatusRow {
    OverrideStatusRow {
        record_kind: OVERRIDE_STATUS_ROW_RECORD_KIND.to_owned(),
        schema_version: BACKGROUND_WORK_STATUS_SCHEMA_VERSION,
        row_id: format!("row.background_override.{}", sheet.override_id),
        override_id: sheet.override_id.clone(),
        lane_token: sheet.lane.as_str().to_owned(),
        decision: sheet.decision.as_str().to_owned(),
        label: sheet.label.clone(),
        reason: sheet.reason.clone(),
        blocked_control_explanation: sheet.blocked_control_explanation.clone(),
        action_label: sheet.action_label.clone(),
        blocked: sheet.decision != OverrideDecisionClass::Allowed,
    }
}

fn affected_lane_work_label(snapshot: &ResourceGovernorSnapshot, input: &PressureInput) -> String {
    let mut labels = Vec::new();
    for lane in &input.affected_lanes {
        if let Some(state) = snapshot
            .lane_states
            .iter()
            .find(|state| state.lane == *lane)
        {
            labels.extend(state.affected_work_labels.iter().cloned());
        }
    }
    labels.sort();
    labels.dedup();
    join_or_none(&labels)
}

fn actions_for_lane(state: &QueueLaneState) -> Vec<String> {
    let mut actions = vec!["Open details".to_owned()];
    if state.has_checkpoint_truth() {
        actions.push("Review checkpoint".to_owned());
    }
    if state.primary_state == QueueLaneStateFlag::Denied
        || state.state_flags.contains(&QueueLaneStateFlag::Paused)
    {
        actions.push("Resume when safe".to_owned());
    }
    if state.override_sheet.is_some() {
        actions.push("Review override".to_owned());
    }
    actions
}

fn lane_needs_banner(state: &QueueLaneState) -> bool {
    state.primary_state == QueueLaneStateFlag::Paused
        || state.primary_state == QueueLaneStateFlag::Denied
        || state.state_flags.contains(&QueueLaneStateFlag::Paused)
        || state.state_flags.contains(&QueueLaneStateFlag::Denied)
}

fn queue_age_label(age_seconds: Option<f64>) -> String {
    match age_seconds {
        Some(age) if age >= 60.0 => format!("{:.0}m {:.0}s", age / 60.0, age % 60.0),
        Some(age) => format!("{age:.0}s"),
        None => "none".to_owned(),
    }
}

fn join_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_owned()
    } else {
        values.join(", ")
    }
}

fn truth_source_for_lane(state: &QueueLaneState) -> String {
    format!("resource_governor.queue_lane.{}", state.lane.as_str())
}

#[cfg(test)]
mod tests {
    use aureline_runtime::seeded_resource_governor_snapshot;

    use super::*;

    #[test]
    fn projection_names_paused_work_and_keeps_support_export_in_parity() {
        let snapshot = seeded_resource_governor_snapshot(
            "snapshot",
            "workspace",
            "profile",
            "2026-05-18T19:00:00Z",
        );
        let bundle = BackgroundWorkStatusBundle::from_snapshot(&snapshot);
        let validation = bundle.validate(&snapshot);
        assert!(
            validation.is_ok(),
            "violations: {:?}",
            validation.violations
        );
        assert_eq!(bundle.lane_rows.len(), snapshot.lane_states.len());
        assert_eq!(bundle.support_export.lane_rows, bundle.lane_rows);
        assert!(bundle
            .deferred_work_banners
            .iter()
            .any(|banner| banner.body.contains("AI background context expansion")));
    }

    #[test]
    fn pressure_cards_show_shed_work_and_protected_state() {
        let snapshot = seeded_resource_governor_snapshot(
            "snapshot",
            "workspace",
            "profile",
            "2026-05-18T19:01:00Z",
        );
        let bundle = BackgroundWorkStatusBundle::from_snapshot(&snapshot);
        let card = bundle
            .pressure_cards
            .iter()
            .find(|card| card.pressure_dimension == "battery_thermal")
            .expect("battery/thermal pressure card");
        assert!(card
            .shed_work_label
            .contains("AI background context expansion"));
        assert!(card.protected_state_label.contains("save"));
    }
}
