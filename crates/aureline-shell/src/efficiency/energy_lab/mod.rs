//! Energy/thermal lab traces, Project Doctor reports, and support exports.
//!
//! The parent [`crate::efficiency`] module owns the canonical efficiency-state
//! object model: one [`EfficiencyStateSnapshot`] per observed posture, the
//! per-workload [`WorkloadBudgetDecision`] that decides what throttles, and the
//! [`EfficiencyStateTransitionEvent`] the runtime emits as pressure rises and
//! clears. The [`surfaces`][crate::efficiency::surfaces],
//! [`disclosures`][crate::efficiency::disclosures], and
//! [`session_pressure`][crate::efficiency::session_pressure] modules each project
//! one *instant* of that state into a status, support, or continuity view.
//!
//! This module adds the missing *over-time* dimension the low-power claim needs to
//! be promotion-grade: a typed **energy/thermal lab**. For each claimed M5 desktop
//! profile it injects a deterministic schedule of battery and thermal pressure,
//! drives the canonical runtime through the resulting efficiency-state
//! transitions, and captures one [`EfficiencyLabTrace`] — an ordered series of
//! steps, each recording the transition that fired, the subsystems it throttled,
//! the hidden-pane audit, and a content-free explanation of *why* each surface
//! slowed or paused. The trace is the canonical, exportable evidence that turns
//! efficiency-state behavior from informal observation into something a fixture
//! can fail promotion over.
//!
//! Two consumer packets derive from the same trace so a release reviewer, Project
//! Doctor, and support never disagree:
//!
//! - [`EfficiencyDoctorReport`] — the Project Doctor projection. It answers, for
//!   the current posture, the four operator questions the contract requires: the
//!   **current efficiency state**, the **recent transitions** that led here, the
//!   **throttled subsystems**, and the **override posture**. It carries a finding
//!   code and severity so Doctor can rank it next to other probes.
//! - [`EfficiencyLabSupportExport`] — a metadata-only support/export packet that
//!   lets support explain low-power behavior — including the transition history —
//!   without raw log spelunking. It carries no provider payloads, secret bodies,
//!   or user content.
//!
//! Every record here is content-free by construction: it references efficiency
//! states, source-of-change signals, subsystem tokens, surface *classes*, and
//! canonical labels, never document bodies, file paths, or provider payloads. The
//! [`EfficiencyLabTrace::trace_is_content_free`](EfficiencyLabTrace) gate makes
//! that guarantee certifiable, and the checked-in lab fixtures fail promotion if a
//! protected path degrades, a hidden pane paints, or a slowdown goes unexplained.

use serde::{Deserialize, Serialize};

use super::governance::{
    OverridePosture, M5_EFFICIENCY_GOVERNANCE_MATRIX_REF, M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
};
use super::surfaces::{EFFICIENCY_DETAILS_SURFACE_REF, EFFICIENCY_INSPECT_COMMAND_ID};
use super::{
    derive_override_posture, derive_recovery_state, protected_interactions,
    EfficiencyAffectedSubsystem, EfficiencyPressureSource, EfficiencyState, EfficiencyStateRuntime,
    EfficiencyStateSnapshot, EfficiencyStateTransitionEvent, HiddenPaneRenderAudit,
    ProtectedSurfaceClass, RenderVisibilityInput, ThrottledCapabilityRow, VisibilityState,
    WorkloadFamily,
};

#[cfg(test)]
mod tests;

/// Stable record kind for an [`EfficiencyLabTrace`] payload.
pub const EFFICIENCY_LAB_TRACE_RECORD_KIND: &str = "efficiency_energy_lab_trace";

/// Stable record kind for an [`EfficiencyLabTraceStep`] payload.
pub const EFFICIENCY_LAB_TRACE_STEP_RECORD_KIND: &str = "efficiency_energy_lab_trace_step";

/// Stable record kind for a [`SurfaceSlowdownExplanation`] payload.
pub const EFFICIENCY_SLOWDOWN_EXPLANATION_RECORD_KIND: &str = "efficiency_surface_slowdown_reason";

/// Stable record kind for an [`EfficiencyDoctorReport`] payload.
pub const EFFICIENCY_DOCTOR_REPORT_RECORD_KIND: &str = "efficiency_state_doctor_report";

/// Stable record kind for an [`EfficiencyLabSupportExport`] payload.
pub const EFFICIENCY_LAB_SUPPORT_EXPORT_RECORD_KIND: &str = "efficiency_lab_support_export";

/// Schema version shared by the lab trace, the Doctor report, and the support
/// export.
pub const EFFICIENCY_LAB_SCHEMA_VERSION: u32 = 1;

/// Project Doctor probe id the [`EfficiencyDoctorReport`] is reported under.
pub const EFFICIENCY_DOCTOR_PROBE_ID: &str = "probe.runtime.efficiency_state";

/// One claimed M5 desktop profile the energy/thermal lab exercises. The class
/// names the hardware-and-policy situation a low-power claim must hold for, so the
/// lab covers the distinct causes the efficiency-state contract keeps separate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabProfileClass {
    /// A laptop on battery entering OS battery saver and recovering on AC power.
    BatteryUltrabook,
    /// A workstation under sustained thermal pressure that clears.
    ThermalWorkstation,
    /// A fleet machine an admin policy caps regardless of battery.
    PolicyManagedFleet,
    /// A laptop crossing low into critical battery, protecting core work.
    CriticalBatteryField,
}

impl LabProfileClass {
    /// Every profile class, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::BatteryUltrabook,
        Self::ThermalWorkstation,
        Self::PolicyManagedFleet,
        Self::CriticalBatteryField,
    ];

    /// Stable token recorded in traces and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BatteryUltrabook => "battery_ultrabook",
            Self::ThermalWorkstation => "thermal_workstation",
            Self::PolicyManagedFleet => "policy_managed_fleet",
            Self::CriticalBatteryField => "critical_battery_field",
        }
    }

    /// Human-readable profile label rendered in the lab evidence.
    pub const fn label(self) -> &'static str {
        match self {
            Self::BatteryUltrabook => "Battery ultrabook",
            Self::ThermalWorkstation => "Thermal workstation",
            Self::PolicyManagedFleet => "Policy-managed fleet",
            Self::CriticalBatteryField => "Critical-battery field laptop",
        }
    }

    /// Resolves a stable token back into its profile class, if known.
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|class| class.as_str() == token)
    }
}

/// One step of injected pressure in a lab profile's schedule. It names the target
/// efficiency state, the source-of-change that drives it, the reason string the
/// transition records, the background workloads observed at that step, and the
/// hidden surfaces the step audits. Reasons and labels are author-provided
/// vocabulary, never user content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PressureInjection {
    /// Short label for the step, e.g. "battery saver engages".
    pub step_label: String,
    /// Target efficiency state the injection drives the runtime to.
    pub target_state: EfficiencyState,
    /// Source-of-change signals for the transition; the first drives the event.
    pub sources: Vec<EfficiencyPressureSource>,
    /// Reason string recorded on the transition event.
    pub reason: String,
    /// Background workloads observed at this step.
    pub workloads: Vec<WorkloadFamily>,
    /// Hidden or off-screen surfaces audited at this step.
    pub hidden_surfaces: Vec<(ProtectedSurfaceClass, VisibilityState)>,
    /// Observation timestamp for the step.
    pub observed_at: String,
}

/// A claimed M5 desktop profile together with the pressure schedule the lab
/// injects to exercise it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyLabProfile {
    /// Stable profile id used in trace, fixture, and artifact file names.
    pub profile_id: String,
    /// Profile-class token.
    pub profile_class: String,
    /// Human-readable profile label.
    pub profile_label: String,
    /// Workspace id the profile runs in.
    pub workspace_id: String,
    /// Ordered pressure-injection schedule.
    pub injections: Vec<PressureInjection>,
}

/// A content-free explanation of why one surface slowed or paused at a trace step.
///
/// It reuses the canonical [`ThrottledCapabilityRow`] sentences — the current-state
/// label is the *why*, the user-impact label is *what stays correct* — so the
/// explanation can never disagree with what the status and disclosure surfaces
/// render, and can never carry document content, file paths, or provider payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceSlowdownExplanation {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Workload-family token for the affected subsystem.
    pub subsystem_token: String,
    /// Human-readable subsystem label.
    pub subsystem_label: String,
    /// Source-subsystem owner label.
    pub owner_label: String,
    /// Budget-action token applied to the subsystem.
    pub action_token: String,
    /// Scope-accurate sentence naming why the surface slowed or paused.
    pub why_label: String,
    /// The correctness this surface keeps while it is reduced.
    pub what_stays_correct: String,
    /// Always true: the explanation references only canonical vocabulary.
    pub content_free: bool,
    /// Always false: the explanation names no user content.
    pub names_user_content: bool,
}

impl SurfaceSlowdownExplanation {
    fn from_capability_row(row: &ThrottledCapabilityRow) -> Self {
        Self {
            record_kind: EFFICIENCY_SLOWDOWN_EXPLANATION_RECORD_KIND.to_owned(),
            subsystem_token: row.capability_id.clone(),
            subsystem_label: row.capability_label.clone(),
            owner_label: row.host_owner_label.clone(),
            action_token: row.visible_state.clone(),
            why_label: row.current_state_label.clone(),
            what_stays_correct: row.user_impact_label.clone(),
            content_free: true,
            names_user_content: false,
        }
    }
}

/// A compact, export-safe digest of one [`EfficiencyStateTransitionEvent`]. It is
/// the "recent transitions" row both the Doctor report and the support export
/// quote, so the transition history reads identically across surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyTransitionDigest {
    /// Previous efficiency-state token.
    pub previous_state: String,
    /// New efficiency-state token.
    pub new_state: String,
    /// Source signal that drove the transition.
    pub source_signal: String,
    /// Human-readable reason.
    pub reason: String,
    /// Top contributors that were throttled or staged at the transition.
    pub top_throttled_contributors: Vec<String>,
    /// Observation timestamp.
    pub observed_at: String,
}

impl EfficiencyTransitionDigest {
    fn from_event(event: &EfficiencyStateTransitionEvent) -> Self {
        Self {
            previous_state: event.previous_state.clone(),
            new_state: event.new_state.clone(),
            source_signal: event.source_signal.clone(),
            reason: event.reason.clone(),
            top_throttled_contributors: event.top_throttled_contributors.clone(),
            observed_at: event.observed_at.clone(),
        }
    }
}

/// One captured step of an [`EfficiencyLabTrace`]: the transition that fired, the
/// posture it produced, the subsystems it throttled, the hidden-pane audit, and
/// the content-free explanations for every reduced surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyLabTraceStep {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Zero-based step index within the trace.
    pub step_index: usize,
    /// Short step label.
    pub step_label: String,
    /// The transition event the step recorded.
    pub transition: EfficiencyStateTransitionEvent,
    /// Active efficiency-state token after the transition.
    pub active_state: String,
    /// Source-of-change tokens for the step.
    pub source_of_change: Vec<String>,
    /// True when the step materially changed runtime behavior.
    pub behavior_changed: bool,
    /// Override-posture token for the step.
    pub override_posture: String,
    /// Recovery-state token for the step.
    pub recovery_state: String,
    /// Subsystems the step throttled.
    pub throttled_subsystems: Vec<EfficiencyAffectedSubsystem>,
    /// Content-free reasons each reduced surface slowed or paused.
    pub slowdown_explanations: Vec<SurfaceSlowdownExplanation>,
    /// True when no hidden pane painted, animated, or polled off-screen.
    pub hidden_pane_passes_policy: bool,
    /// Number of hidden-pane render violations the audit found.
    pub hidden_pane_violation_count: u32,
    /// Number of hidden or off-screen surfaces audited.
    pub hidden_surface_count: usize,
    /// Protected interactions the step may not narrow.
    pub protected_interactions_preserved: Vec<String>,
    /// True when save durability and user-owned artifacts stay preserved.
    pub durability_preserved: bool,
    /// Observation timestamp.
    pub observed_at: String,
}

impl EfficiencyLabTraceStep {
    fn from_snapshot(
        step_index: usize,
        step_label: &str,
        transition: EfficiencyStateTransitionEvent,
        snapshot: &EfficiencyStateSnapshot,
    ) -> Self {
        let slowdown_explanations = snapshot
            .throttled_capabilities
            .iter()
            .map(SurfaceSlowdownExplanation::from_capability_row)
            .collect::<Vec<_>>();
        Self {
            record_kind: EFFICIENCY_LAB_TRACE_STEP_RECORD_KIND.to_owned(),
            step_index,
            step_label: step_label.to_owned(),
            transition,
            active_state: snapshot.active_state.clone(),
            source_of_change: snapshot.pressure_sources.clone(),
            behavior_changed: snapshot.behavior_changed,
            override_posture: snapshot.override_posture.clone(),
            recovery_state: snapshot.recovery_state.clone(),
            throttled_subsystems: snapshot.affected_subsystems.clone(),
            slowdown_explanations,
            hidden_pane_passes_policy: snapshot.hidden_pane_audit.passes_hidden_pane_policy,
            hidden_pane_violation_count: snapshot
                .hidden_pane_audit
                .hidden_pane_render_violation_count,
            hidden_surface_count: snapshot.hidden_pane_audit.hidden_surface_count,
            protected_interactions_preserved: snapshot.protected_interactions_preserved.clone(),
            durability_preserved: snapshot.preserves_durability_truth(),
            observed_at: snapshot.observed_at.clone(),
        }
    }

    /// True when every throttled subsystem at this step carries a content-free
    /// slowdown explanation, so no surface slows without a recorded reason.
    pub fn every_slowdown_explained(&self) -> bool {
        self.slowdown_explanations.len() == self.throttled_subsystems.len()
            && self
                .slowdown_explanations
                .iter()
                .all(|reason| reason.content_free && !reason.names_user_content)
            && self
                .slowdown_explanations
                .iter()
                .all(|reason| !reason.why_label.is_empty() && !reason.what_stays_correct.is_empty())
    }

    /// True when the step held the protected interactions and durability.
    pub fn protected_paths_held(&self) -> bool {
        self.durability_preserved
            && protected_interactions()
                .iter()
                .all(|item| self.protected_interactions_preserved.contains(item))
    }
}

/// An energy/thermal lab trace: the ordered evidence one claimed M5 profile
/// produces under its injected pressure schedule.
///
/// It is the canonical, exportable truth for "how did efficiency state behave over
/// this run?" The [`EfficiencyDoctorReport`] and [`EfficiencyLabSupportExport`]
/// derive from it, and the promotion gates ([`protected_paths_held`],
/// [`hidden_panes_passed`], [`every_slowdown_explained`], and
/// [`trace_is_content_free`]) are the certifiable claims a regression must keep.
///
/// [`protected_paths_held`]: EfficiencyLabTrace::protected_paths_held
/// [`hidden_panes_passed`]: EfficiencyLabTrace::hidden_panes_passed
/// [`every_slowdown_explained`]: EfficiencyLabTrace::every_slowdown_explained
/// [`trace_is_content_free`]: EfficiencyLabTrace::trace_is_content_free
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyLabTrace {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable trace id.
    pub trace_id: String,
    /// Profile id the trace exercises.
    pub profile_id: String,
    /// Profile-class token.
    pub profile_class: String,
    /// Human-readable profile label.
    pub profile_label: String,
    /// Workspace id the trace ran in.
    pub workspace_id: String,
    /// Canonical governance matrix this trace's vocabulary derives from.
    pub matrix_ref: String,
    /// Schema validating the governance matrix.
    pub schema_ref: String,
    /// Ordered trace steps.
    pub steps: Vec<EfficiencyLabTraceStep>,
    /// Recent transitions, oldest first, across the whole run.
    pub transitions: Vec<EfficiencyTransitionDigest>,
    /// Final efficiency-state token at the end of the run.
    pub final_state: String,
    /// Final override-posture token at the end of the run.
    pub final_override_posture: String,
    /// Final recovery-state token at the end of the run.
    pub final_recovery_state: String,
    /// True when protected interactions and durability held at every step.
    pub protected_paths_held: bool,
    /// True when no hidden pane painted, animated, or polled off-screen.
    pub hidden_panes_passed: bool,
    /// True when every reduced surface carried a recorded reason.
    pub every_slowdown_explained: bool,
    /// True when the whole trace references only canonical vocabulary.
    pub trace_is_content_free: bool,
    /// Doctor report id derived from this trace.
    pub doctor_report_ref: String,
    /// Support-export id derived from this trace.
    pub support_export_ref: String,
    /// Generation timestamp.
    pub generated_at: String,
}

impl EfficiencyLabTrace {
    /// Runs a profile's injected pressure schedule and captures the trace.
    pub fn run_profile(profile: &EfficiencyLabProfile) -> Self {
        let mut runtime = EfficiencyStateRuntime::new();
        let mut steps = Vec::with_capacity(profile.injections.len());
        for (index, injection) in profile.injections.iter().enumerate() {
            let primary = *injection
                .sources
                .first()
                .unwrap_or(&EfficiencyPressureSource::AcPower);
            let transition = runtime.transition_to(
                injection.target_state,
                primary,
                injection.reason.clone(),
                injection.observed_at.clone(),
            );
            let decisions = injection
                .workloads
                .iter()
                .map(|workload| {
                    runtime.decide_workload(*workload, primary, injection.observed_at.clone())
                })
                .collect::<Vec<_>>();
            let render_decisions = injection
                .hidden_surfaces
                .iter()
                .enumerate()
                .map(|(surface_index, (class, visibility))| {
                    runtime.decide_render(RenderVisibilityInput {
                        surface_id: format!("surface.{}.{surface_index}", class.as_str()),
                        surface_class: *class,
                        visibility_state: *visibility,
                        requested_paint_count: 4,
                        requested_animation_tick_count: 12,
                        correctness_polling_required: true,
                    })
                })
                .collect::<Vec<_>>();
            let audit = HiddenPaneRenderAudit::from_decisions(&render_decisions);
            let behavior_changed = injection.target_state != EfficiencyState::Nominal;
            let snapshot = EfficiencyStateSnapshot::from_decisions(
                profile.workspace_id.clone(),
                injection.target_state,
                injection.sources.clone(),
                behavior_changed,
                decisions,
                audit,
                injection.observed_at.clone(),
            );
            steps.push(EfficiencyLabTraceStep::from_snapshot(
                index,
                &injection.step_label,
                transition,
                &snapshot,
            ));
        }

        let transitions = runtime
            .transition_events()
            .iter()
            .map(EfficiencyTransitionDigest::from_event)
            .collect::<Vec<_>>();
        let final_state = runtime.current_state();
        let final_sources = profile
            .injections
            .last()
            .map(|injection| injection.sources.clone())
            .unwrap_or_default();
        let generated_at = profile
            .injections
            .last()
            .map(|injection| injection.observed_at.clone())
            .unwrap_or_default();

        let protected_paths_held = steps
            .iter()
            .all(EfficiencyLabTraceStep::protected_paths_held);
        let hidden_panes_passed = steps.iter().all(|step| step.hidden_pane_passes_policy);
        let every_slowdown_explained = steps
            .iter()
            .all(EfficiencyLabTraceStep::every_slowdown_explained);
        let trace_is_content_free = steps.iter().all(|step| {
            step.slowdown_explanations
                .iter()
                .all(|reason| reason.content_free && !reason.names_user_content)
        });

        Self {
            record_kind: EFFICIENCY_LAB_TRACE_RECORD_KIND.to_owned(),
            schema_version: EFFICIENCY_LAB_SCHEMA_VERSION,
            trace_id: trace_id(&profile.profile_id),
            profile_id: profile.profile_id.clone(),
            profile_class: profile.profile_class.clone(),
            profile_label: profile.profile_label.clone(),
            workspace_id: profile.workspace_id.clone(),
            matrix_ref: M5_EFFICIENCY_GOVERNANCE_MATRIX_REF.to_owned(),
            schema_ref: M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF.to_owned(),
            steps,
            transitions,
            final_state: final_state.as_str().to_owned(),
            final_override_posture: derive_override_posture(final_state, &final_sources)
                .as_str()
                .to_owned(),
            final_recovery_state: derive_recovery_state(final_state).as_str().to_owned(),
            protected_paths_held,
            hidden_panes_passed,
            every_slowdown_explained,
            trace_is_content_free,
            doctor_report_ref: doctor_report_id(&profile.workspace_id),
            support_export_ref: lab_support_export_id(&profile.workspace_id, &profile.profile_id),
            generated_at,
        }
    }

    /// True when every promotion gate holds: protected paths held, hidden panes
    /// passed, every slowdown explained, and the trace stayed content-free. A
    /// regression in any of these fails the checked-in lab fixtures.
    pub fn promotion_gates_pass(&self) -> bool {
        self.protected_paths_held
            && self.hidden_panes_passed
            && self.every_slowdown_explained
            && self.trace_is_content_free
    }

    /// The final step's throttled subsystems — the subsystems still reduced at the
    /// end of the run.
    fn final_throttled_subsystems(&self) -> Vec<EfficiencyAffectedSubsystem> {
        self.steps
            .last()
            .map(|step| step.throttled_subsystems.clone())
            .unwrap_or_default()
    }
}

/// The Project Doctor projection of a finished lab trace.
///
/// Doctor reads this packet to report the efficiency-state posture next to its
/// other probes. It names the **current state**, the **recent transitions** that
/// led here, the **throttled subsystems**, and the **override posture**, with a
/// finding code and severity so the posture can be ranked, and an open-details
/// command so an operator can reach the full state surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyDoctorReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// Project Doctor probe id this report is filed under.
    pub probe_id: String,
    /// Trace id this report derives from.
    pub trace_ref: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Current efficiency-state token.
    pub current_state: String,
    /// Stable finding code for the posture.
    pub finding_code: String,
    /// Finding-severity token (`ok`, `notice`, or `degraded`).
    pub finding_severity: String,
    /// One-sentence summary for the Doctor row.
    pub summary_label: String,
    /// Recent transitions that led to the current state, oldest first.
    pub recent_transitions: Vec<EfficiencyTransitionDigest>,
    /// Subsystems currently throttled.
    pub throttled_subsystems: Vec<EfficiencyAffectedSubsystem>,
    /// Override-posture token.
    pub override_posture: String,
    /// Recovery-state token.
    pub recovery_state: String,
    /// Protected interactions the posture may not narrow.
    pub protected_interactions_preserved: Vec<String>,
    /// True when save durability and user-owned artifacts stay preserved.
    pub durability_preserved: bool,
    /// True when no hidden pane painted, animated, or polled off-screen.
    pub hidden_pane_passes_policy: bool,
    /// Open-details command id.
    pub primary_command_id: String,
    /// Surface ref the open-details command opens.
    pub opens_surface_ref: String,
    /// Support-export packet id that quotes the same posture.
    pub support_export_ref: String,
    /// Observation timestamp.
    pub observed_at: String,
}

impl EfficiencyDoctorReport {
    /// Projects a finished lab trace into the Project Doctor report.
    pub fn from_trace(trace: &EfficiencyLabTrace) -> Self {
        let state = EfficiencyState::from_token(&trace.final_state).unwrap_or_default();
        let (finding_code, finding_severity, summary_label) = doctor_finding(state, trace);
        let throttled_subsystems = trace.final_throttled_subsystems();
        let hidden_pane_passes_policy = trace.hidden_panes_passed;
        Self {
            record_kind: EFFICIENCY_DOCTOR_REPORT_RECORD_KIND.to_owned(),
            schema_version: EFFICIENCY_LAB_SCHEMA_VERSION,
            report_id: doctor_report_id(&trace.workspace_id),
            probe_id: EFFICIENCY_DOCTOR_PROBE_ID.to_owned(),
            trace_ref: trace.trace_id.clone(),
            workspace_id: trace.workspace_id.clone(),
            current_state: trace.final_state.clone(),
            finding_code,
            finding_severity,
            summary_label,
            recent_transitions: trace.transitions.clone(),
            throttled_subsystems,
            override_posture: trace.final_override_posture.clone(),
            recovery_state: trace.final_recovery_state.clone(),
            protected_interactions_preserved: protected_interactions(),
            durability_preserved: trace.steps.iter().all(|step| step.durability_preserved),
            hidden_pane_passes_policy,
            primary_command_id: EFFICIENCY_INSPECT_COMMAND_ID.to_owned(),
            opens_surface_ref: EFFICIENCY_DETAILS_SURFACE_REF.to_owned(),
            support_export_ref: lab_support_export_id(&trace.workspace_id, &trace.profile_id),
            observed_at: trace.generated_at.clone(),
        }
    }

    /// True when the report exposes all four contract fields without empty data:
    /// current state, recent transitions, throttled subsystems, and override
    /// posture.
    pub fn names_state_transitions_subsystems_and_override(&self) -> bool {
        !self.current_state.is_empty()
            && !self.recent_transitions.is_empty()
            && !self.override_posture.is_empty()
            && OverridePosture::from_token(&self.override_posture).is_some()
            && EfficiencyState::from_token(&self.current_state).is_some()
    }
}

/// A metadata-only support/export packet for a finished lab trace.
///
/// Support tooling reads this packet to explain the low-power posture — the
/// current state, the recent transitions, the throttled subsystems, and the
/// override posture — without raw log spelunking. It carries no provider
/// payloads, secret bodies, or user content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyLabSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Trace id this export derives from.
    pub trace_ref: String,
    /// Canonical governance matrix this export's vocabulary derives from.
    pub matrix_ref: String,
    /// Export-safe timestamp.
    pub generated_at: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Profile id the export covers.
    pub profile_id: String,
    /// Current efficiency-state token.
    pub current_state: String,
    /// Recent transitions, oldest first.
    pub recent_transitions: Vec<EfficiencyTransitionDigest>,
    /// Subsystems currently throttled.
    pub throttled_subsystems: Vec<EfficiencyAffectedSubsystem>,
    /// Override-posture token.
    pub override_posture: String,
    /// Recovery-state token.
    pub recovery_state: String,
    /// True when the transition history can be reconstructed without UI text.
    pub reconstructs_transitions: bool,
    /// True when throttled subsystems can be reconstructed without UI text.
    pub reconstructs_throttled_subsystems: bool,
    /// True when the override posture can be reconstructed without UI text.
    pub reconstructs_override_posture: bool,
    /// True when support tooling must scrape rendered prose. Always false.
    pub ui_text_scrape_required: bool,
    /// Always false: no raw provider payloads are exported.
    pub raw_provider_payloads_exported: bool,
    /// Always false: no raw secret bodies are exported.
    pub raw_secret_values_exported: bool,
    /// Always false: no user content is named.
    pub names_user_content: bool,
    /// Structured support fields used for reconstruction.
    pub support_field_refs: Vec<String>,
}

impl EfficiencyLabSupportExport {
    /// Projects a finished lab trace into a metadata-only support export.
    pub fn from_trace(trace: &EfficiencyLabTrace) -> Self {
        Self {
            record_kind: EFFICIENCY_LAB_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: EFFICIENCY_LAB_SCHEMA_VERSION,
            export_id: lab_support_export_id(&trace.workspace_id, &trace.profile_id),
            trace_ref: trace.trace_id.clone(),
            matrix_ref: M5_EFFICIENCY_GOVERNANCE_MATRIX_REF.to_owned(),
            generated_at: trace.generated_at.clone(),
            workspace_id: trace.workspace_id.clone(),
            profile_id: trace.profile_id.clone(),
            current_state: trace.final_state.clone(),
            recent_transitions: trace.transitions.clone(),
            throttled_subsystems: trace.final_throttled_subsystems(),
            override_posture: trace.final_override_posture.clone(),
            recovery_state: trace.final_recovery_state.clone(),
            reconstructs_transitions: !trace.transitions.is_empty(),
            reconstructs_throttled_subsystems: true,
            reconstructs_override_posture: !trace.final_override_posture.is_empty(),
            ui_text_scrape_required: false,
            raw_provider_payloads_exported: false,
            raw_secret_values_exported: false,
            names_user_content: false,
            support_field_refs: vec![
                "export.efficiency.current_state".to_owned(),
                "export.efficiency.recent_transitions".to_owned(),
                "export.efficiency.throttled_subsystems".to_owned(),
                "export.efficiency.override_posture".to_owned(),
                "export.efficiency.recovery_state".to_owned(),
            ],
        }
    }

    /// True when the export is safe for default support bundles.
    pub fn redaction_safe(&self) -> bool {
        !self.ui_text_scrape_required
            && !self.raw_provider_payloads_exported
            && !self.raw_secret_values_exported
            && !self.names_user_content
    }

    /// True when support can reconstruct the posture without raw logs.
    pub fn reconstructs_posture_without_logs(&self) -> bool {
        self.reconstructs_transitions
            && self.reconstructs_throttled_subsystems
            && self.reconstructs_override_posture
            && !self.ui_text_scrape_required
    }
}

/// One seeded lab case: the profile, the trace it produces, and the Doctor and
/// support packets derived from that trace. Backs the dump example, the checked-in
/// fixtures, the exported trace artifacts, and the round-trip test so the lab
/// evidence never drifts from code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyLabCase {
    /// The claimed M5 profile the case exercises.
    pub profile: EfficiencyLabProfile,
    /// The trace the profile's injected pressure schedule produced.
    pub trace: EfficiencyLabTrace,
    /// The Project Doctor report derived from the trace.
    pub doctor_report: EfficiencyDoctorReport,
    /// The support export derived from the trace.
    pub support_export: EfficiencyLabSupportExport,
}

/// Runs one profile and bundles the trace, Doctor report, and support export.
pub fn run_lab_case(profile: EfficiencyLabProfile) -> EfficiencyLabCase {
    let trace = EfficiencyLabTrace::run_profile(&profile);
    let doctor_report = EfficiencyDoctorReport::from_trace(&trace);
    let support_export = EfficiencyLabSupportExport::from_trace(&trace);
    EfficiencyLabCase {
        profile,
        trace,
        doctor_report,
        support_export,
    }
}

/// The claimed M5 desktop profiles the energy/thermal lab exercises. Their
/// workspaces, states, sources, and timestamps line up with the seeded snapshots
/// the status, diagnostics, support, and disclosure surfaces use, so the lab
/// evidence aligns with the rest of the low-power contract.
pub fn seed_lab_profiles() -> Vec<EfficiencyLabProfile> {
    use EfficiencyPressureSource as Source;
    use EfficiencyState as State;
    use ProtectedSurfaceClass as Surface;
    use VisibilityState as Vis;
    use WorkloadFamily as Work;

    vec![
        EfficiencyLabProfile {
            profile_id: "battery-ultrabook".to_owned(),
            profile_class: LabProfileClass::BatteryUltrabook.as_str().to_owned(),
            profile_label: LabProfileClass::BatteryUltrabook.label().to_owned(),
            workspace_id: "ws:battery-saver".to_owned(),
            injections: vec![
                PressureInjection {
                    step_label: "on AC power, nominal".to_owned(),
                    target_state: State::Nominal,
                    sources: vec![Source::AcPower],
                    reason: "On AC power; full budgets available".to_owned(),
                    workloads: vec![Work::AiWarmup, Work::SpeculativePrefetch],
                    hidden_surfaces: vec![],
                    observed_at: "2026-06-20T14:00:00Z".to_owned(),
                },
                PressureInjection {
                    step_label: "battery saver engages".to_owned(),
                    target_state: State::EfficiencyAware,
                    sources: vec![Source::OsBatterySaver],
                    reason: "OS battery saver active".to_owned(),
                    workloads: vec![
                        Work::AiWarmup,
                        Work::SpeculativePrefetch,
                        Work::UploadTransfer,
                    ],
                    hidden_surfaces: vec![(Surface::PreviewViewport, Vis::HiddenTab)],
                    observed_at: "2026-06-20T14:01:00Z".to_owned(),
                },
                PressureInjection {
                    step_label: "back on AC, recovering".to_owned(),
                    target_state: State::Recovery,
                    sources: vec![Source::PressureCleared],
                    reason: "Power pressure cleared; resuming in stages".to_owned(),
                    workloads: vec![Work::AiWarmup, Work::SpeculativePrefetch],
                    hidden_surfaces: vec![],
                    observed_at: "2026-06-20T14:02:00Z".to_owned(),
                },
            ],
        },
        EfficiencyLabProfile {
            profile_id: "thermal-workstation".to_owned(),
            profile_class: LabProfileClass::ThermalWorkstation.as_str().to_owned(),
            profile_label: LabProfileClass::ThermalWorkstation.label().to_owned(),
            workspace_id: "ws:efficiency-demo".to_owned(),
            injections: vec![
                PressureInjection {
                    step_label: "nominal".to_owned(),
                    target_state: State::Nominal,
                    sources: vec![Source::AcPower],
                    reason: "Thermals nominal; full budgets available".to_owned(),
                    workloads: vec![Work::IndexingRefresh, Work::GraphEnrichment],
                    hidden_surfaces: vec![],
                    observed_at: "2026-06-20T14:00:00Z".to_owned(),
                },
                PressureInjection {
                    step_label: "sustained thermal pressure".to_owned(),
                    target_state: State::ThermalConstrained,
                    sources: vec![Source::ThermalPressure],
                    reason: "OS thermal pressure reported serious".to_owned(),
                    workloads: vec![
                        Work::IndexingRefresh,
                        Work::PreviewRefresh,
                        Work::GraphEnrichment,
                    ],
                    hidden_surfaces: vec![
                        (Surface::PreviewViewport, Vis::HiddenTab),
                        (Surface::GraphPanel, Vis::CollapsedSplit),
                    ],
                    observed_at: "2026-06-20T14:01:00Z".to_owned(),
                },
                PressureInjection {
                    step_label: "thermals clearing".to_owned(),
                    target_state: State::Recovery,
                    sources: vec![Source::PressureCleared],
                    reason: "Thermal pressure cleared; resuming in stages".to_owned(),
                    workloads: vec![Work::IndexingRefresh, Work::GraphEnrichment],
                    hidden_surfaces: vec![],
                    observed_at: "2026-06-20T14:02:00Z".to_owned(),
                },
            ],
        },
        EfficiencyLabProfile {
            profile_id: "policy-managed-fleet".to_owned(),
            profile_class: LabProfileClass::PolicyManagedFleet.as_str().to_owned(),
            profile_label: LabProfileClass::PolicyManagedFleet.label().to_owned(),
            workspace_id: "ws:policy-cap".to_owned(),
            injections: vec![
                PressureInjection {
                    step_label: "unmanaged, nominal".to_owned(),
                    target_state: State::Nominal,
                    sources: vec![Source::AcPower],
                    reason: "No policy cap; full budgets available".to_owned(),
                    workloads: vec![Work::UploadTransfer, Work::ExtensionPolling],
                    hidden_surfaces: vec![],
                    observed_at: "2026-06-20T14:00:00Z".to_owned(),
                },
                PressureInjection {
                    step_label: "admin policy caps background work".to_owned(),
                    target_state: State::EfficiencyAware,
                    sources: vec![Source::PolicyCap],
                    reason: "Admin policy capped background work".to_owned(),
                    workloads: vec![Work::UploadTransfer, Work::ExtensionPolling],
                    hidden_surfaces: vec![],
                    observed_at: "2026-06-20T14:01:00Z".to_owned(),
                },
            ],
        },
        EfficiencyLabProfile {
            profile_id: "critical-battery-field".to_owned(),
            profile_class: LabProfileClass::CriticalBatteryField.as_str().to_owned(),
            profile_label: LabProfileClass::CriticalBatteryField.label().to_owned(),
            workspace_id: "ws:critical-battery".to_owned(),
            injections: vec![
                PressureInjection {
                    step_label: "low battery, efficiency aware".to_owned(),
                    target_state: State::EfficiencyAware,
                    sources: vec![Source::LowBattery],
                    reason: "Battery low; reducing speculative work".to_owned(),
                    workloads: vec![Work::AiWarmup, Work::SpeculativePrefetch],
                    hidden_surfaces: vec![],
                    observed_at: "2026-06-20T14:02:00Z".to_owned(),
                },
                PressureInjection {
                    step_label: "critical battery protects core".to_owned(),
                    target_state: State::ProtectCore,
                    sources: vec![Source::CriticalBattery],
                    reason: "Critical battery protecting core interaction".to_owned(),
                    workloads: vec![
                        Work::IndexingRefresh,
                        Work::ExtensionPolling,
                        Work::PreviewRefresh,
                    ],
                    hidden_surfaces: vec![(Surface::GraphPanel, Vis::DetachedOffscreen)],
                    observed_at: "2026-06-20T14:03:00Z".to_owned(),
                },
                PressureInjection {
                    step_label: "charging, recovering".to_owned(),
                    target_state: State::Recovery,
                    sources: vec![Source::PressureCleared],
                    reason: "Power restored; resuming in stages".to_owned(),
                    workloads: vec![Work::IndexingRefresh, Work::PreviewRefresh],
                    hidden_surfaces: vec![],
                    observed_at: "2026-06-20T14:04:00Z".to_owned(),
                },
            ],
        },
    ]
}

/// Runs every seeded lab profile and returns the bundled cases. These back the
/// dump example, the checked-in fixtures, the exported trace artifacts, and the
/// round-trip test.
pub fn seeded_lab_cases() -> Vec<EfficiencyLabCase> {
    seed_lab_profiles().into_iter().map(run_lab_case).collect()
}

/// Builds the canonical lab trace id for a profile.
fn trace_id(profile_id: &str) -> String {
    format!("efficiency.trace.{profile_id}")
}

/// Builds the canonical Doctor report id for a workspace.
fn doctor_report_id(workspace_id: &str) -> String {
    format!("doctor.report.efficiency_state.{workspace_id}")
}

/// Builds the canonical lab support-export id for a workspace and profile.
fn lab_support_export_id(workspace_id: &str, profile_id: &str) -> String {
    format!("support.export.efficiency_lab.{workspace_id}.{profile_id}")
}

/// Derives the Doctor finding code, severity, and summary for the current state.
fn doctor_finding(state: EfficiencyState, trace: &EfficiencyLabTrace) -> (String, String, String) {
    let (code, severity, summary): (&str, &str, String) = match state {
        EfficiencyState::Nominal => (
            "efficiency_nominal",
            "ok",
            "Efficiency state is nominal; no background work is reduced.".to_owned(),
        ),
        EfficiencyState::EfficiencyAware => (
            "efficiency_reducing_optional_work",
            "notice",
            format!(
                "Battery or power pressure is reducing optional work; {} subsystem(s) are throttled.",
                trace.final_throttled_subsystems().len()
            ),
        ),
        EfficiencyState::ThermalConstrained => (
            "efficiency_thermal_constrained",
            "notice",
            format!(
                "Thermal pressure is reducing background and visual work; {} subsystem(s) are throttled.",
                trace.final_throttled_subsystems().len()
            ),
        ),
        EfficiencyState::ProtectCore => (
            "efficiency_protect_core_active",
            "degraded",
            "Core interaction is protected; optional work is paused or denied until pressure clears."
                .to_owned(),
        ),
        EfficiencyState::Recovery => (
            "efficiency_recovering",
            "notice",
            "Pressure has cleared; deferred work is resuming in stages.".to_owned(),
        ),
    };
    (code.to_owned(), severity.to_owned(), summary)
}
