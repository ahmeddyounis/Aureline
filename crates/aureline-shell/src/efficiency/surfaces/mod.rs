//! Diagnostics and support-export surfaces over the canonical efficiency state.
//!
//! The shell status bar already renders the active efficiency state from the
//! [`EfficiencyStatusSnapshot`][super::EfficiencyStatusSnapshot] embedded in an
//! [`EfficiencyStateSnapshot`][super::EfficiencyStateSnapshot]. This module adds
//! the other two surfaces the efficiency-state contract requires so all three
//! derive from one object instead of inventing local low-power wording:
//!
//! - [`EfficiencyDiagnosticsProjection`] — an operator-facing diagnostics view
//!   that names what changed, why it changed, which subsystems were affected,
//!   the override posture, and the recovery state, with an open-details command.
//!   It embeds the matrix-bound [`EfficiencyGovernanceProjection`] so its
//!   vocabulary is traceable to the frozen governance matrix.
//! - [`EfficiencyStateSupportExport`] — a metadata-only support/export packet so
//!   support tooling can reconstruct the low-power posture without scraping
//!   rendered UI text or reading raw logs.
//!
//! Both project from the same [`EfficiencyStateSnapshot`], so a diagnostics row,
//! a support export, and the status pill can never disagree about the active
//! state or its cause.

use serde::{Deserialize, Serialize};

use super::governance::{
    EfficiencyGovernanceProjection, EfficiencyRecoveryState, HiddenPaneBehavior, OverridePosture,
    M5_EFFICIENCY_GOVERNANCE_MATRIX_REF,
};
use super::{
    EfficiencyAffectedSubsystem, EfficiencyPressureSource, EfficiencyState, EfficiencyStateRuntime,
    EfficiencyStateSnapshot, HiddenPaneRenderAudit, ProtectedSurfaceClass, RenderVisibilityInput,
    VisibilityState, WorkloadFamily,
};

/// Stable record kind for [`EfficiencyDiagnosticsProjection`] payloads.
pub const EFFICIENCY_DIAGNOSTICS_RECORD_KIND: &str = "efficiency_state_diagnostics_projection";

/// Schema version for [`EfficiencyDiagnosticsProjection`] payloads.
pub const EFFICIENCY_DIAGNOSTICS_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for [`EfficiencyStateSupportExport`] payloads.
pub const EFFICIENCY_SUPPORT_EXPORT_RECORD_KIND: &str = "efficiency_state_support_export";

/// Stable record kind for [`EfficiencySupportExportRow`] payloads.
pub const EFFICIENCY_SUPPORT_EXPORT_ROW_RECORD_KIND: &str = "efficiency_state_support_export_row";

/// Schema version for [`EfficiencyStateSupportExport`] payloads.
pub const EFFICIENCY_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 1;

/// Command id a diagnostics or support surface invokes to open the full state.
pub const EFFICIENCY_INSPECT_COMMAND_ID: &str = "cmd:runtime.efficiency_state.inspect";

/// Surface ref the open-details command opens.
pub const EFFICIENCY_DETAILS_SURFACE_REF: &str = "surface.runtime.efficiency_state";

/// Operator-facing diagnostics projection of the active efficiency state.
///
/// This is the diagnostics-surface consumer of the canonical
/// [`EfficiencyStateSnapshot`]. It answers the three operator questions the
/// efficiency-state contract requires — what changed, why it changed, and which
/// subsystems were affected — and exposes the override posture and recovery
/// state so an operator never has to guess whether an adaptation can be lifted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyDiagnosticsProjection {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Active workspace id.
    pub workspace_id: String,
    /// Active efficiency-state token (what changed).
    pub active_state: String,
    /// Source-of-change tokens (why it changed).
    pub source_of_change: Vec<String>,
    /// True when runtime behavior changed.
    pub behavior_changed: bool,
    /// One-sentence summary of the posture for the diagnostics header.
    pub summary_label: String,
    /// Override-posture token for the adaptation.
    pub override_posture: String,
    /// Recovery-state token for the adaptation.
    pub recovery_state: String,
    /// Subsystems whose behavior changed (which subsystems were affected).
    pub affected_subsystems: Vec<EfficiencyAffectedSubsystem>,
    /// Count of affected subsystems.
    pub affected_subsystem_count: usize,
    /// True when no hidden pane painted, animated, or polled off-screen.
    pub hidden_pane_passes_policy: bool,
    /// Number of hidden-pane render violations the audit found.
    pub hidden_pane_violation_count: u32,
    /// Protected interactions the adaptation may not narrow.
    pub protected_interactions_preserved: Vec<String>,
    /// True when save durability and user-owned artifacts stay preserved.
    pub durability_preserved: bool,
    /// Matrix-bound governance projection for vocabulary traceability.
    pub governance: EfficiencyGovernanceProjection,
    /// Open-details command id.
    pub primary_command_id: String,
    /// Surface ref the open-details command opens.
    pub opens_surface_ref: String,
    /// Support-export packet id that quotes the same posture.
    pub support_export_ref: String,
    /// Observation timestamp.
    pub observed_at: String,
}

impl EfficiencyDiagnosticsProjection {
    /// Projects the canonical snapshot into an operator-facing diagnostics view.
    pub fn from_snapshot(snapshot: &EfficiencyStateSnapshot) -> Self {
        let hidden_pane_behaviors = hidden_pane_behaviors_for(&snapshot.hidden_pane_audit);
        let override_posture = OverridePosture::from_token(&snapshot.override_posture)
            .unwrap_or(OverridePosture::NotOverridable);
        let recovery_state = EfficiencyRecoveryState::from_token(&snapshot.recovery_state)
            .unwrap_or(EfficiencyRecoveryState::NotInRecovery);
        let governance = EfficiencyGovernanceProjection::from_snapshot(
            snapshot,
            &hidden_pane_behaviors,
            override_posture,
            recovery_state,
        );
        Self {
            record_kind: EFFICIENCY_DIAGNOSTICS_RECORD_KIND.to_owned(),
            schema_version: EFFICIENCY_DIAGNOSTICS_SCHEMA_VERSION,
            workspace_id: snapshot.workspace_id.clone(),
            active_state: snapshot.active_state.clone(),
            source_of_change: snapshot.pressure_sources.clone(),
            behavior_changed: snapshot.behavior_changed,
            summary_label: summary_label_for(snapshot),
            override_posture: snapshot.override_posture.clone(),
            recovery_state: snapshot.recovery_state.clone(),
            affected_subsystems: snapshot.affected_subsystems.clone(),
            affected_subsystem_count: snapshot.affected_subsystems.len(),
            hidden_pane_passes_policy: snapshot.hidden_pane_audit.passes_hidden_pane_policy,
            hidden_pane_violation_count: snapshot
                .hidden_pane_audit
                .hidden_pane_render_violation_count,
            protected_interactions_preserved: snapshot.protected_interactions_preserved.clone(),
            durability_preserved: snapshot.preserves_durability_truth(),
            governance,
            primary_command_id: EFFICIENCY_INSPECT_COMMAND_ID.to_owned(),
            opens_surface_ref: EFFICIENCY_DETAILS_SURFACE_REF.to_owned(),
            support_export_ref: support_export_id(snapshot),
            observed_at: snapshot.observed_at.clone(),
        }
    }
}

/// Metadata-only support/export packet for the active efficiency state.
///
/// Support tooling reads this packet to reconstruct the low-power posture —
/// what state is active, why, which subsystems it affected, and whether it can
/// be overridden — without scraping rendered UI text or relying on raw logs. It
/// carries no provider payloads or secret bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyStateSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export-safe timestamp.
    pub generated_at: String,
    /// Source snapshot record kind.
    pub packet_ref: String,
    /// Canonical governance matrix this export's vocabulary derives from.
    pub matrix_ref: String,
    /// Active workspace id.
    pub workspace_id: String,
    /// Active efficiency-state token.
    pub active_state: String,
    /// Source-of-change tokens.
    pub source_of_change: Vec<String>,
    /// Override-posture token.
    pub override_posture: String,
    /// Recovery-state token.
    pub recovery_state: String,
    /// True when runtime behavior changed.
    pub behavior_changed: bool,
    /// Per-subsystem export rows.
    pub rows: Vec<EfficiencySupportExportRow>,
    /// True when no hidden pane painted, animated, or polled off-screen.
    pub hidden_pane_passes_policy: bool,
    /// Protected interactions the adaptation may not narrow.
    pub protected_interactions_preserved: Vec<String>,
    /// True when durability stayed preserved.
    pub durability_preserved: bool,
    /// True when the state change can be reconstructed without UI text.
    pub reconstructs_state_change: bool,
    /// True when the change cause can be reconstructed without UI text.
    pub reconstructs_change_cause: bool,
    /// True when affected subsystems can be reconstructed without UI text.
    pub reconstructs_affected_subsystems: bool,
    /// True when the override posture can be reconstructed without UI text.
    pub reconstructs_override_posture: bool,
    /// True when support tooling must scrape rendered prose. Always false.
    pub ui_text_scrape_required: bool,
    /// Always false for this export.
    pub raw_provider_payloads_exported: bool,
    /// Always false for this export.
    pub raw_secret_values_exported: bool,
    /// Structured support fields used for reconstruction.
    pub support_field_refs: Vec<String>,
}

impl EfficiencyStateSupportExport {
    /// Projects the canonical snapshot into a metadata-only support export.
    pub fn from_snapshot(snapshot: &EfficiencyStateSnapshot) -> Self {
        let rows = snapshot
            .affected_subsystems
            .iter()
            .map(|subsystem| EfficiencySupportExportRow::from_subsystem(snapshot, subsystem))
            .collect::<Vec<_>>();
        Self {
            record_kind: EFFICIENCY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: EFFICIENCY_SUPPORT_EXPORT_SCHEMA_VERSION,
            export_id: support_export_id(snapshot),
            generated_at: snapshot.observed_at.clone(),
            packet_ref: snapshot.record_kind.clone(),
            matrix_ref: M5_EFFICIENCY_GOVERNANCE_MATRIX_REF.to_owned(),
            workspace_id: snapshot.workspace_id.clone(),
            active_state: snapshot.active_state.clone(),
            source_of_change: snapshot.pressure_sources.clone(),
            override_posture: snapshot.override_posture.clone(),
            recovery_state: snapshot.recovery_state.clone(),
            behavior_changed: snapshot.behavior_changed,
            rows,
            hidden_pane_passes_policy: snapshot.hidden_pane_audit.passes_hidden_pane_policy,
            protected_interactions_preserved: snapshot.protected_interactions_preserved.clone(),
            durability_preserved: snapshot.preserves_durability_truth(),
            reconstructs_state_change: true,
            reconstructs_change_cause: !snapshot.pressure_sources.is_empty(),
            reconstructs_affected_subsystems: snapshot.affected_subsystems.len()
                == snapshot.throttled_capabilities.len(),
            reconstructs_override_posture: !snapshot.override_posture.is_empty(),
            ui_text_scrape_required: false,
            raw_provider_payloads_exported: false,
            raw_secret_values_exported: false,
            support_field_refs: vec![
                "export.efficiency.state".to_owned(),
                "export.efficiency.source_of_change".to_owned(),
                "export.efficiency.override_posture".to_owned(),
                "export.efficiency.recovery_state".to_owned(),
                "export.efficiency.affected_subsystems".to_owned(),
            ],
        }
    }

    /// True when the export is safe for default support bundles.
    pub fn redaction_safe(&self) -> bool {
        !self.ui_text_scrape_required
            && !self.raw_provider_payloads_exported
            && !self.raw_secret_values_exported
    }

    /// True when the export lets support reconstruct the posture without logs.
    pub fn reconstructs_posture_without_logs(&self) -> bool {
        self.reconstructs_state_change
            && self.reconstructs_change_cause
            && self.reconstructs_affected_subsystems
            && self.reconstructs_override_posture
            && !self.ui_text_scrape_required
    }
}

/// One metadata-only support/export row for an affected subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencySupportExportRow {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Stable export row id.
    pub export_row_id: String,
    /// Workload-family token for the subsystem.
    pub subsystem_token: String,
    /// Source-subsystem owner label.
    pub owner_label: String,
    /// Budget-action token.
    pub action_token: String,
    /// Visible-capability-state token.
    pub visible_state_token: String,
    /// User-impact sentence.
    pub user_impact_label: String,
    /// Structured support fields used for reconstruction.
    pub support_field_refs: Vec<String>,
}

impl EfficiencySupportExportRow {
    fn from_subsystem(
        snapshot: &EfficiencyStateSnapshot,
        subsystem: &EfficiencyAffectedSubsystem,
    ) -> Self {
        Self {
            record_kind: EFFICIENCY_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            export_row_id: format!(
                "support.item.efficiency.{}.{}",
                snapshot.workspace_id, subsystem.subsystem_token
            ),
            subsystem_token: subsystem.subsystem_token.clone(),
            owner_label: subsystem.owner_label.clone(),
            action_token: subsystem.action.clone(),
            visible_state_token: subsystem.visible_state.clone(),
            user_impact_label: subsystem.user_impact_label.clone(),
            support_field_refs: vec![
                "export.efficiency.subsystem".to_owned(),
                "export.efficiency.action".to_owned(),
                "export.efficiency.user_impact".to_owned(),
            ],
        }
    }
}

fn support_export_id(snapshot: &EfficiencyStateSnapshot) -> String {
    format!(
        "support.export.efficiency.{}.{}",
        snapshot.workspace_id, snapshot.active_state
    )
}

fn summary_label_for(snapshot: &EfficiencyStateSnapshot) -> String {
    if let Some(status) = &snapshot.status {
        return status.explanation.clone();
    }
    if !snapshot.behavior_changed {
        return format!(
            "{} is active; no background work changed.",
            snapshot.active_state
        );
    }
    format!(
        "{} changed {} subsystem(s).",
        snapshot.active_state,
        snapshot.affected_subsystems.len()
    )
}

/// Derives the hidden-pane behaviours a hidden surface adopted from the render
/// audit. The audit proves no hidden pane painted or polled off-screen; when it
/// audited hidden surfaces, render, animation, and speculative polling were all
/// suppressed.
fn hidden_pane_behaviors_for(audit: &HiddenPaneRenderAudit) -> Vec<HiddenPaneBehavior> {
    if audit.hidden_surface_count == 0 {
        return Vec::new();
    }
    vec![
        HiddenPaneBehavior::RenderSuppressed,
        HiddenPaneBehavior::AnimationSuppressed,
        HiddenPaneBehavior::PollingPaused,
    ]
}

/// Builds a deterministic efficiency-state snapshot for the given posture.
///
/// Seeded snapshots back the diagnostics default panel, the checked-in fixtures,
/// and cross-surface tests so the three surfaces always derive from one object.
pub fn seed_efficiency_state_snapshot(
    workspace_id: &str,
    state: EfficiencyState,
    sources: &[EfficiencyPressureSource],
    workloads: &[WorkloadFamily],
    hidden_surfaces: &[(ProtectedSurfaceClass, VisibilityState)],
    reason: &str,
    observed_at: &str,
) -> EfficiencyStateSnapshot {
    let source = *sources
        .first()
        .unwrap_or(&EfficiencyPressureSource::AcPower);
    let mut runtime = EfficiencyStateRuntime::new();
    runtime.transition_to(state, source, reason, observed_at);
    let decisions = workloads
        .iter()
        .map(|workload| runtime.decide_workload(*workload, source, observed_at))
        .collect::<Vec<_>>();
    let render_decisions = hidden_surfaces
        .iter()
        .enumerate()
        .map(|(index, (class, visibility))| {
            runtime.decide_render(RenderVisibilityInput {
                surface_id: format!("surface.hidden.{index}"),
                surface_class: *class,
                visibility_state: *visibility,
                requested_paint_count: 4,
                requested_animation_tick_count: 12,
                correctness_polling_required: true,
            })
        })
        .collect::<Vec<_>>();
    let audit = HiddenPaneRenderAudit::from_decisions(&render_decisions);
    EfficiencyStateSnapshot::from_decisions(
        workspace_id,
        state,
        sources.to_vec(),
        true,
        decisions,
        audit,
        observed_at,
    )
}

/// The representative thermal-pressure snapshot used by the diagnostics default
/// panel and the example dump.
pub fn seeded_efficiency_state_snapshot() -> EfficiencyStateSnapshot {
    seed_efficiency_state_snapshot(
        "ws:efficiency-demo",
        EfficiencyState::ThermalConstrained,
        &[EfficiencyPressureSource::ThermalPressure],
        &[
            WorkloadFamily::IndexingRefresh,
            WorkloadFamily::PreviewRefresh,
            WorkloadFamily::GraphEnrichment,
        ],
        &[
            (
                ProtectedSurfaceClass::PreviewViewport,
                VisibilityState::HiddenTab,
            ),
            (
                ProtectedSurfaceClass::GraphPanel,
                VisibilityState::CollapsedSplit,
            ),
        ],
        "OS thermal pressure reported serious",
        "2026-06-20T14:00:00Z",
    )
}

/// The full set of representative snapshots covering the distinct causes the
/// efficiency-state contract must keep separate: OS battery saver, thermal
/// pressure, a policy-imposed cap, a critical-battery protect-core posture, and
/// staged recovery.
pub fn seeded_efficiency_state_snapshots() -> Vec<EfficiencyStateSnapshot> {
    vec![
        seed_efficiency_state_snapshot(
            "ws:battery-saver",
            EfficiencyState::EfficiencyAware,
            &[EfficiencyPressureSource::OsBatterySaver],
            &[
                WorkloadFamily::AiWarmup,
                WorkloadFamily::SpeculativePrefetch,
                WorkloadFamily::UploadTransfer,
            ],
            &[(
                ProtectedSurfaceClass::PreviewViewport,
                VisibilityState::HiddenTab,
            )],
            "OS battery saver active",
            "2026-06-20T14:01:00Z",
        ),
        seeded_efficiency_state_snapshot(),
        seed_efficiency_state_snapshot(
            "ws:policy-cap",
            EfficiencyState::EfficiencyAware,
            &[EfficiencyPressureSource::PolicyCap],
            &[
                WorkloadFamily::UploadTransfer,
                WorkloadFamily::ExtensionPolling,
            ],
            &[],
            "Admin policy capped background work",
            "2026-06-20T14:02:00Z",
        ),
        seed_efficiency_state_snapshot(
            "ws:critical-battery",
            EfficiencyState::ProtectCore,
            &[EfficiencyPressureSource::CriticalBattery],
            &[
                WorkloadFamily::IndexingRefresh,
                WorkloadFamily::ExtensionPolling,
                WorkloadFamily::PreviewRefresh,
            ],
            &[(
                ProtectedSurfaceClass::GraphPanel,
                VisibilityState::DetachedOffscreen,
            )],
            "Critical battery protecting core interaction",
            "2026-06-20T14:03:00Z",
        ),
        seed_efficiency_state_snapshot(
            "ws:recovery",
            EfficiencyState::Recovery,
            &[EfficiencyPressureSource::PressureCleared],
            &[
                WorkloadFamily::IndexingRefresh,
                WorkloadFamily::GraphEnrichment,
            ],
            &[],
            "Power and thermal pressure cleared",
            "2026-06-20T14:04:00Z",
        ),
    ]
}

#[cfg(test)]
mod tests;
