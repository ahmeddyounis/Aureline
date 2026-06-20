//! Hidden-surface render suppression and polling/animation throttles for the
//! richer product surfaces — notebooks, traces, previews, docs/browser panes,
//! pipelines, and incident workspaces.
//!
//! The parent [`crate::efficiency`] module already models the generic
//! efficiency state, the per-workload budget decisions, and a coarse
//! [`HiddenPaneRenderAudit`] over visibility states. This module specializes
//! that contract to the named product surfaces so a hidden notebook, an
//! off-screen trace viewer, or a backgrounded preview cannot quietly keep
//! painting, animating, refreshing rich content, or polling once the user is no
//! longer looking at it.
//!
//! For each surface it produces a [`HiddenSurfaceDecision`] that, per work
//! channel, records whether the channel was maintained, throttled, or
//! suppressed, and a [`ResumeContinuityContract`] that names exactly what is
//! restored when the surface becomes visible again. The invariant the policy
//! enforces is asymmetric on purpose: decorative paint, animation, rich-preview
//! refresh, and speculative polling are dropped to zero while hidden, but
//! correctness-critical channels (a running notebook cell, trace event capture,
//! pipeline completion tracking, an incident feed) are only throttled to a
//! non-zero floor — never silently dropped — so resuming a surface restores
//! truthful state without private-cache corruption or surprise reruns.
//!
//! The aggregate [`HiddenSurfaceSuppressionAudit`] attributes the saved work to
//! specific surface classes so energy/thermal traces
//! ([`HiddenSurfaceEnergyTrace`]) and operator diagnostics
//! ([`HiddenSurfaceDiagnosticsProjection`]) can answer "what did hiding this
//! pane save, and which class did it come from?" from one object rather than
//! cloning local low-power wording. The decisions also bridge back to the frozen
//! [`HiddenPaneBehavior`] vocabulary and the coarse
//! [`HiddenPaneRenderAudit`][super::HiddenPaneRenderAudit] so the two views can
//! never disagree about whether a hidden pane painted.

use serde::{Deserialize, Serialize};

use super::governance::HiddenPaneBehavior;
use super::{
    protected_interactions, EfficiencyDurabilityInvariants, EfficiencyState, HiddenPaneRenderAudit,
    RenderVisibilitySample, VisibilityState,
};
use crate::notifications::envelope::SourceSubsystem;

#[cfg(test)]
mod tests;

/// Stable record kind for [`HiddenSurfaceDecision`] payloads.
pub const HIDDEN_SURFACE_DECISION_RECORD_KIND: &str = "hidden_surface_decision";

/// Stable record kind for [`HiddenSurfaceSuppressionAudit`] payloads.
pub const HIDDEN_SURFACE_SUPPRESSION_AUDIT_RECORD_KIND: &str = "hidden_surface_suppression_audit";

/// Stable record kind for [`HiddenSurfaceEnergyTrace`] payloads.
pub const HIDDEN_SURFACE_ENERGY_TRACE_RECORD_KIND: &str = "hidden_surface_energy_trace";

/// Stable record kind for [`HiddenSurfaceDiagnosticsProjection`] payloads.
pub const HIDDEN_SURFACE_DIAGNOSTICS_RECORD_KIND: &str = "hidden_surface_diagnostics_projection";

/// Schema version shared by the hidden-surface audit, trace, and diagnostics
/// records.
pub const HIDDEN_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Command id a diagnostics surface invokes to open the hidden-surface details.
pub const HIDDEN_SURFACE_INSPECT_COMMAND_ID: &str = "cmd:runtime.hidden_surface.inspect";

/// Surface ref the hidden-surface open-details command opens.
pub const HIDDEN_SURFACE_DETAILS_SURFACE_REF: &str = "surface.runtime.hidden_surface_policy";

/// A product-surface class governed by hidden-surface render suppression.
///
/// These are the richer surfaces that keep background work alive — rendering,
/// refreshing, animating, or polling — and therefore must shed that work when
/// hidden or off-screen instead of draining battery, GPU, or background budget
/// behind the user's back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HiddenSurfaceClass {
    /// Notebook editor with live kernel output and cell rendering.
    Notebook,
    /// Execution/debug trace viewer that captures and renders trace events.
    Trace,
    /// Rich preview surface (rendered document, canvas, or media).
    Preview,
    /// Docs or in-app browser pane backed by a browser runtime.
    DocsBrowser,
    /// Pipeline run surface that tracks and renders run progress.
    Pipeline,
    /// Incident workspace that follows a live incident feed.
    Incident,
}

impl HiddenSurfaceClass {
    /// Every hidden-surface class the policy governs, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Notebook,
        Self::Trace,
        Self::Preview,
        Self::DocsBrowser,
        Self::Pipeline,
        Self::Incident,
    ];

    /// Stable token recorded in audits, traces, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::Trace => "trace",
            Self::Preview => "preview",
            Self::DocsBrowser => "docs_browser",
            Self::Pipeline => "pipeline",
            Self::Incident => "incident",
        }
    }

    /// Human-readable owner label rendered next to a suppression row.
    pub const fn owner_label(self) -> &'static str {
        match self {
            Self::Notebook => "Notebook kernel",
            Self::Trace => "Trace viewer",
            Self::Preview => "Preview",
            Self::DocsBrowser => "Docs/browser",
            Self::Pipeline => "Pipeline",
            Self::Incident => "Incident workspace",
        }
    }

    /// Canonical source subsystem that owns the surface, for traceability with
    /// the rest of the efficiency vocabulary.
    pub const fn source_subsystem(self) -> SourceSubsystem {
        match self {
            Self::Notebook => SourceSubsystem::NotebookKernel,
            Self::Trace => SourceSubsystem::DebugSession,
            Self::Preview => SourceSubsystem::ReviewAndDiff,
            Self::DocsBrowser => SourceSubsystem::DocsHelpServiceHealth,
            Self::Pipeline => SourceSubsystem::TaskRunner,
            Self::Incident => SourceSubsystem::Shell,
        }
    }

    /// Work channels this class can keep alive and that the policy governs.
    pub const fn applicable_channels(self) -> &'static [HiddenWorkChannel] {
        use HiddenWorkChannel as Channel;
        match self {
            Self::Notebook => &[
                Channel::Paint,
                Channel::Animation,
                Channel::RichRefresh,
                Channel::SpeculativePoll,
                Channel::CorrectnessPoll,
            ],
            Self::Trace => &[
                Channel::Paint,
                Channel::Animation,
                Channel::SpeculativePoll,
                Channel::CorrectnessPoll,
            ],
            Self::Preview => &[
                Channel::Paint,
                Channel::Animation,
                Channel::RichRefresh,
                Channel::SpeculativePoll,
            ],
            Self::DocsBrowser => &[
                Channel::Paint,
                Channel::Animation,
                Channel::RichRefresh,
                Channel::SpeculativePoll,
            ],
            Self::Pipeline => &[
                Channel::Paint,
                Channel::Animation,
                Channel::SpeculativePoll,
                Channel::CorrectnessPoll,
            ],
            Self::Incident => &[
                Channel::Paint,
                Channel::Animation,
                Channel::SpeculativePoll,
                Channel::CorrectnessPoll,
            ],
        }
    }

    /// The resume-continuity contract this class preserves while hidden. It names
    /// the exact state restored on resume and asserts the restore neither reruns
    /// work nor corrupts a private cache.
    pub fn resume_contract(self) -> ResumeContinuityContract {
        let (resume_token_kind, restored_state_label) = match self {
            Self::Notebook => (
                "notebook_kernel_session_and_committed_outputs",
                "Kernel session and last committed cell outputs are restored; no cell is re-run by becoming visible.",
            ),
            Self::Trace => (
                "buffered_trace_events",
                "Captured trace events are buffered while hidden and rendered on resume; no events are dropped or replayed.",
            ),
            Self::Preview => (
                "last_truthful_preview_snapshot",
                "The last truthful preview snapshot is restored; refresh resumes without a surprise rebuild.",
            ),
            Self::DocsBrowser => (
                "loaded_document_and_scroll_position",
                "The loaded document and scroll position are restored from cache; no navigation side effect is replayed.",
            ),
            Self::Pipeline => (
                "reconciled_run_state",
                "Run state is reconciled from its source on resume; the run is never restarted by becoming visible.",
            ),
            Self::Incident => (
                "buffered_incident_feed",
                "Buffered incident events are replayed in order on resume; the live feed is never silently truncated.",
            ),
        };
        ResumeContinuityContract {
            resume_token_kind: resume_token_kind.to_owned(),
            restores_without_rerun: true,
            restores_without_cache_corruption: true,
            restored_state_label: restored_state_label.to_owned(),
        }
    }

    /// Resolves a stable token back into its hidden-surface class, if known.
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|class| class.as_str() == token)
    }
}

/// A class of background work a surface can keep alive.
///
/// Every channel except [`HiddenWorkChannel::CorrectnessPoll`] is decorative or
/// speculative and is dropped to zero while hidden; the correctness channel is
/// only throttled to a non-zero floor so resume stays truthful.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HiddenWorkChannel {
    /// Committed render/paint passes.
    Paint,
    /// Decorative or non-essential animation ticks.
    Animation,
    /// Rich preview/content refresh (re-render of live content).
    RichRefresh,
    /// Speculative polling, prefetch, or background refresh.
    SpeculativePoll,
    /// Correctness-critical polling that must keep state truthful (a running
    /// task, captured events, or a live feed).
    CorrectnessPoll,
}

impl HiddenWorkChannel {
    /// Stable token recorded in channel decisions.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Paint => "paint",
            Self::Animation => "animation",
            Self::RichRefresh => "rich_refresh",
            Self::SpeculativePoll => "speculative_poll",
            Self::CorrectnessPoll => "correctness_poll",
        }
    }

    /// True when the channel keeps user-owned state or an active task truthful,
    /// so it may be throttled but never fully suppressed while hidden.
    pub const fn is_correctness_critical(self) -> bool {
        matches!(self, Self::CorrectnessPoll)
    }
}

/// Disposition the policy applied to one work channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HiddenWorkDisposition {
    /// The channel runs at its requested cadence.
    Maintained,
    /// The channel runs at a reduced cadence.
    Throttled,
    /// The channel is dropped to zero.
    Suppressed,
}

impl HiddenWorkDisposition {
    /// Stable token recorded in channel decisions.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Maintained => "maintained",
            Self::Throttled => "throttled",
            Self::Suppressed => "suppressed",
        }
    }
}

/// Coarse activity of a surface, derived from its [`VisibilityState`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SurfaceActivity {
    /// Visible and focused: the surface is contributing to the active task.
    Active,
    /// Visible but not focused: an inactive preview the user can still glance at.
    VisibleInactive,
    /// Hidden, occluded, collapsed, or off-screen.
    Hidden,
}

impl SurfaceActivity {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::VisibleInactive => "visible_inactive",
            Self::Hidden => "hidden",
        }
    }
}

fn activity_for(visibility: VisibilityState) -> SurfaceActivity {
    if visibility.is_hidden_or_offscreen() {
        SurfaceActivity::Hidden
    } else if matches!(visibility, VisibilityState::VisibleBackground) {
        SurfaceActivity::VisibleInactive
    } else {
        SurfaceActivity::Active
    }
}

/// Requested per-channel work counts for a surface before suppression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HiddenWorkRequest {
    /// Requested paint passes.
    #[serde(default)]
    pub paint_passes: u32,
    /// Requested animation ticks.
    #[serde(default)]
    pub animation_ticks: u32,
    /// Requested rich-content refreshes.
    #[serde(default)]
    pub rich_refreshes: u32,
    /// Requested speculative polls.
    #[serde(default)]
    pub speculative_polls: u32,
    /// Requested correctness-critical polls.
    #[serde(default)]
    pub correctness_polls: u32,
}

impl HiddenWorkRequest {
    /// Requested units for one channel.
    pub const fn units(&self, channel: HiddenWorkChannel) -> u32 {
        match channel {
            HiddenWorkChannel::Paint => self.paint_passes,
            HiddenWorkChannel::Animation => self.animation_ticks,
            HiddenWorkChannel::RichRefresh => self.rich_refreshes,
            HiddenWorkChannel::SpeculativePoll => self.speculative_polls,
            HiddenWorkChannel::CorrectnessPoll => self.correctness_polls,
        }
    }
}

/// One surface's request to the hidden-surface policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenSurfaceInput {
    /// Stable surface id.
    pub surface_id: String,
    /// Product-surface class.
    pub surface_class: HiddenSurfaceClass,
    /// Current visibility state.
    pub visibility_state: VisibilityState,
    /// Requested per-channel work before suppression.
    pub requested: HiddenWorkRequest,
}

/// The resume-continuity contract a surface preserves while suppressed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeContinuityContract {
    /// Token naming the state preserved for resume.
    pub resume_token_kind: String,
    /// True when resume restores state without re-running suppressed work.
    pub restores_without_rerun: bool,
    /// True when resume restores state without corrupting a private cache.
    pub restores_without_cache_corruption: bool,
    /// Human-readable description of what resume restores.
    pub restored_state_label: String,
}

impl ResumeContinuityContract {
    /// True when the contract guarantees a correct, side-effect-free resume.
    pub const fn is_correct(&self) -> bool {
        self.restores_without_rerun && self.restores_without_cache_corruption
    }
}

/// Decision for one work channel of a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenWorkChannelDecision {
    /// Channel token.
    pub channel: String,
    /// Disposition token applied to the channel.
    pub disposition: String,
    /// True when the channel is correctness-critical.
    pub correctness_critical: bool,
    /// Units requested before suppression.
    pub requested_units: u32,
    /// Units committed after suppression.
    pub committed_units: u32,
    /// Units saved by suppression or throttling.
    pub saved_units: u32,
    /// Human-readable rationale for the disposition.
    pub rationale: String,
}

/// Decision for one surface under the hidden-surface policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenSurfaceDecision {
    /// Stable record kind.
    pub record_kind: String,
    /// Stable surface id.
    pub surface_id: String,
    /// Product-surface class token.
    pub surface_class: String,
    /// Owner label for the surface class.
    pub owner_label: String,
    /// Visibility state token.
    pub visibility_state: String,
    /// Coarse activity token (active, visible_inactive, hidden).
    pub activity: String,
    /// Active efficiency state token.
    pub efficiency_state: String,
    /// True when the surface is hidden or off-screen.
    pub hidden: bool,
    /// Per-channel decisions.
    pub channels: Vec<HiddenWorkChannelDecision>,
    /// Frozen hidden-pane behaviour tokens this decision adopted.
    pub hidden_pane_behaviors: Vec<String>,
    /// Resume-continuity contract preserved while suppressed.
    pub resume: ResumeContinuityContract,
    /// True when correctness-critical work and resume continuity are preserved.
    pub correctness_preserved: bool,
    /// True when save durability and user-owned state stay preserved.
    pub durability_preserved: bool,
    /// Saved paint passes.
    pub saved_paint_passes: u32,
    /// Saved animation ticks.
    pub saved_animation_ticks: u32,
    /// Saved rich-content refreshes.
    pub saved_refreshes: u32,
    /// Saved speculative polls.
    pub saved_polls: u32,
    /// Maintained correctness polls (never dropped below this).
    pub maintained_correctness_polls: u32,
    /// Total units saved by suppression and throttling.
    pub total_saved_units: u32,
}

impl HiddenSurfaceDecision {
    /// Decides the per-channel suppression for one surface under `state`.
    pub fn decide(input: &HiddenSurfaceInput, state: EfficiencyState) -> Self {
        let class = input.surface_class;
        let activity = activity_for(input.visibility_state);
        let hidden = matches!(activity, SurfaceActivity::Hidden);

        let mut channels = Vec::new();
        let mut saved_paint_passes = 0;
        let mut saved_animation_ticks = 0;
        let mut saved_refreshes = 0;
        let mut saved_polls = 0;
        let mut maintained_correctness_polls = 0;
        let mut behaviors = Vec::new();

        for &channel in class.applicable_channels() {
            let requested = input.requested.units(channel);
            let committed = committed_units(channel, activity, state, requested);
            let saved = requested.saturating_sub(committed);
            let disposition = disposition_for(requested, committed);

            match channel {
                HiddenWorkChannel::Paint => saved_paint_passes += saved,
                HiddenWorkChannel::Animation => saved_animation_ticks += saved,
                HiddenWorkChannel::RichRefresh => saved_refreshes += saved,
                HiddenWorkChannel::SpeculativePoll => saved_polls += saved,
                HiddenWorkChannel::CorrectnessPoll => maintained_correctness_polls += committed,
            }

            if hidden {
                if let Some(behavior) = behavior_for(channel, disposition) {
                    if !behaviors.contains(&behavior) {
                        behaviors.push(behavior);
                    }
                }
            }

            channels.push(HiddenWorkChannelDecision {
                channel: channel.as_str().to_owned(),
                disposition: disposition.as_str().to_owned(),
                correctness_critical: channel.is_correctness_critical(),
                requested_units: requested,
                committed_units: committed,
                saved_units: saved,
                rationale: rationale_for(channel, activity, state, disposition),
            });
        }

        let has_correctness_channel = class
            .applicable_channels()
            .iter()
            .any(|channel| channel.is_correctness_critical());
        if hidden && !has_correctness_channel && !behaviors.is_empty() {
            behaviors.push(HiddenPaneBehavior::FullyQuiescent);
        }
        behaviors.sort();
        behaviors.dedup();
        let hidden_pane_behaviors = behaviors
            .iter()
            .map(|behavior| behavior.as_str().to_owned())
            .collect::<Vec<_>>();

        let resume = class.resume_contract();
        // Correctness holds when no critical channel was dropped below its
        // requested floor and the resume contract is side-effect free.
        let correctness_preserved = resume.is_correct()
            && channels
                .iter()
                .filter(|decision| decision.correctness_critical && decision.requested_units > 0)
                .all(|decision| decision.committed_units >= 1);

        let total_saved_units =
            saved_paint_passes + saved_animation_ticks + saved_refreshes + saved_polls;

        Self {
            record_kind: HIDDEN_SURFACE_DECISION_RECORD_KIND.to_owned(),
            surface_id: input.surface_id.clone(),
            surface_class: class.as_str().to_owned(),
            owner_label: class.owner_label().to_owned(),
            visibility_state: input.visibility_state.as_str().to_owned(),
            activity: activity.as_str().to_owned(),
            efficiency_state: state.as_str().to_owned(),
            hidden,
            channels,
            hidden_pane_behaviors,
            resume,
            correctness_preserved,
            durability_preserved: true,
            saved_paint_passes,
            saved_animation_ticks,
            saved_refreshes,
            saved_polls,
            maintained_correctness_polls,
            total_saved_units,
        }
    }

    /// True when a hidden surface still committed decorative or speculative work,
    /// which the policy forbids.
    pub fn violates_hidden_pane_policy(&self) -> bool {
        self.hidden
            && self
                .channels
                .iter()
                .any(|decision| !decision.correctness_critical && decision.committed_units > 0)
    }

    /// Projects the decision into a coarse [`RenderVisibilitySample`] so the
    /// frozen [`HiddenPaneRenderAudit`] can confirm the per-class decision agrees
    /// with the generic hidden-pane policy.
    pub fn as_render_sample(&self) -> RenderVisibilitySample {
        let committed_paint_count = self
            .channels
            .iter()
            .find(|decision| decision.channel == HiddenWorkChannel::Paint.as_str())
            .map(|decision| decision.committed_units)
            .unwrap_or(0);
        let hidden_pane_work = self
            .channels
            .iter()
            .filter(|decision| !decision.correctness_critical)
            .map(|decision| {
                if self.hidden {
                    decision.committed_units
                } else {
                    0
                }
            })
            .sum();
        RenderVisibilitySample {
            surface_id: self.surface_id.clone(),
            surface_class: self.surface_class.clone(),
            visibility_state: self.visibility_state.clone(),
            committed_paint_count,
            hidden_pane_work,
            offscreen_suppression_eligible: self.total_saved_units,
        }
    }
}

/// Energy/thermal savings attributed to one hidden-surface class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenSurfaceClassSaving {
    /// Surface-class token.
    pub surface_class: String,
    /// Owner label for the class.
    pub owner_label: String,
    /// Number of hidden surfaces of this class.
    pub hidden_surface_count: usize,
    /// Saved paint passes.
    pub saved_paint_passes: u32,
    /// Saved animation ticks.
    pub saved_animation_ticks: u32,
    /// Saved rich-content refreshes.
    pub saved_refreshes: u32,
    /// Saved speculative polls.
    pub saved_polls: u32,
    /// Total saved units.
    pub saved_units_total: u32,
}

/// Aggregate audit over hidden-surface decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenSurfaceSuppressionAudit {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Active efficiency state token.
    pub efficiency_state: String,
    /// Number of surfaces audited.
    pub audited_surface_count: usize,
    /// Number of hidden or off-screen surfaces audited.
    pub hidden_surface_count: usize,
    /// Number of hidden surfaces that still committed forbidden work.
    pub hidden_pane_violation_count: u32,
    /// True when no hidden surface painted, animated, refreshed, or polled
    /// speculatively.
    pub passes_policy: bool,
    /// True when every suppressed surface restores correctly on resume.
    pub all_resumes_correct: bool,
    /// True when durability stayed preserved across every decision.
    pub durability_preserved: bool,
    /// Total units saved by suppression and throttling.
    pub total_saved_units: u32,
    /// Per-surface decisions.
    pub decisions: Vec<HiddenSurfaceDecision>,
    /// Energy/thermal savings attributed per surface class.
    pub saved_by_class: Vec<HiddenSurfaceClassSaving>,
    /// Protected interactions the policy may not narrow.
    pub protected_interactions_preserved: Vec<String>,
    /// Observation timestamp.
    pub observed_at: String,
}

impl HiddenSurfaceSuppressionAudit {
    /// Builds an audit for the given surfaces under `state`.
    pub fn for_surfaces(
        state: EfficiencyState,
        surfaces: &[HiddenSurfaceInput],
        observed_at: impl Into<String>,
    ) -> Self {
        let decisions = surfaces
            .iter()
            .map(|input| HiddenSurfaceDecision::decide(input, state))
            .collect::<Vec<_>>();
        Self::from_decisions(state, decisions, observed_at)
    }

    /// Builds an audit from already-computed decisions.
    pub fn from_decisions(
        state: EfficiencyState,
        decisions: Vec<HiddenSurfaceDecision>,
        observed_at: impl Into<String>,
    ) -> Self {
        let hidden_surface_count = decisions.iter().filter(|decision| decision.hidden).count();
        let hidden_pane_violation_count = decisions
            .iter()
            .filter(|decision| decision.violates_hidden_pane_policy())
            .count() as u32;
        let all_resumes_correct = decisions
            .iter()
            .filter(|decision| decision.hidden)
            .all(|decision| decision.correctness_preserved);
        let durability_preserved = decisions
            .iter()
            .all(|decision| decision.durability_preserved);
        let total_saved_units = decisions
            .iter()
            .map(|decision| decision.total_saved_units)
            .sum();
        let saved_by_class = saved_by_class(&decisions);
        Self {
            record_kind: HIDDEN_SURFACE_SUPPRESSION_AUDIT_RECORD_KIND.to_owned(),
            schema_version: HIDDEN_SURFACE_SCHEMA_VERSION,
            efficiency_state: state.as_str().to_owned(),
            audited_surface_count: decisions.len(),
            hidden_surface_count,
            hidden_pane_violation_count,
            passes_policy: hidden_pane_violation_count == 0,
            all_resumes_correct,
            durability_preserved,
            total_saved_units,
            decisions,
            saved_by_class,
            protected_interactions_preserved: protected_interactions(),
            observed_at: observed_at.into(),
        }
    }

    /// Projects the audit into a coarse [`HiddenPaneRenderAudit`] so the
    /// per-class policy can be checked against the frozen hidden-pane vocabulary.
    pub fn as_hidden_pane_render_audit(&self) -> HiddenPaneRenderAudit {
        let samples = self
            .decisions
            .iter()
            .map(HiddenSurfaceDecision::as_render_sample)
            .collect::<Vec<_>>();
        HiddenPaneRenderAudit::from_samples(samples)
    }

    /// True when the audit proves the durability invariants for the lane.
    pub fn preserves_durability_truth(&self) -> bool {
        let invariants = EfficiencyDurabilityInvariants::default();
        self.durability_preserved
            && invariants.save_durability_preserved
            && invariants.dirty_buffers_preserved
            && invariants.user_owned_artifacts_preserved
    }
}

/// Per-surface trace mark for the energy/thermal trace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenSurfaceTraceMark {
    /// Stable surface id.
    pub surface_id: String,
    /// Surface-class token.
    pub surface_class: String,
    /// Visibility state token.
    pub visibility_state: String,
    /// Units saved by suppression for this surface.
    pub saved_units: u32,
    /// Hidden-pane behaviour tokens adopted by the surface.
    pub hidden_pane_behaviors: Vec<String>,
}

/// Energy/thermal trace projection of hidden-surface suppression.
///
/// This is the trace-surface consumer of the canonical audit. It attributes the
/// saved work to specific surface classes so an energy or thermal trace can show
/// what hiding a pane saved, and where it came from, without re-deriving the
/// policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenSurfaceEnergyTrace {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Active efficiency state token.
    pub efficiency_state: String,
    /// Human-readable trace window label.
    pub window_label: String,
    /// Observation timestamp.
    pub observed_at: String,
    /// Number of hidden surfaces traced.
    pub hidden_surface_count: usize,
    /// Total saved units.
    pub total_saved_units: u32,
    /// Total saved paint passes.
    pub total_saved_paint_passes: u32,
    /// Total saved animation ticks.
    pub total_saved_animation_ticks: u32,
    /// Total saved rich-content refreshes.
    pub total_saved_refreshes: u32,
    /// Total saved speculative polls.
    pub total_saved_polls: u32,
    /// Energy/thermal savings attributed per surface class.
    pub saved_by_class: Vec<HiddenSurfaceClassSaving>,
    /// Per-surface trace marks.
    pub trace_marks: Vec<HiddenSurfaceTraceMark>,
}

impl HiddenSurfaceEnergyTrace {
    /// Projects the canonical audit into an energy/thermal trace.
    pub fn from_audit(
        audit: &HiddenSurfaceSuppressionAudit,
        window_label: impl Into<String>,
    ) -> Self {
        let mut total_saved_paint_passes = 0;
        let mut total_saved_animation_ticks = 0;
        let mut total_saved_refreshes = 0;
        let mut total_saved_polls = 0;
        let mut trace_marks = Vec::new();
        for decision in &audit.decisions {
            total_saved_paint_passes += decision.saved_paint_passes;
            total_saved_animation_ticks += decision.saved_animation_ticks;
            total_saved_refreshes += decision.saved_refreshes;
            total_saved_polls += decision.saved_polls;
            if decision.hidden {
                trace_marks.push(HiddenSurfaceTraceMark {
                    surface_id: decision.surface_id.clone(),
                    surface_class: decision.surface_class.clone(),
                    visibility_state: decision.visibility_state.clone(),
                    saved_units: decision.total_saved_units,
                    hidden_pane_behaviors: decision.hidden_pane_behaviors.clone(),
                });
            }
        }
        Self {
            record_kind: HIDDEN_SURFACE_ENERGY_TRACE_RECORD_KIND.to_owned(),
            schema_version: HIDDEN_SURFACE_SCHEMA_VERSION,
            efficiency_state: audit.efficiency_state.clone(),
            window_label: window_label.into(),
            observed_at: audit.observed_at.clone(),
            hidden_surface_count: audit.hidden_surface_count,
            total_saved_units: audit.total_saved_units,
            total_saved_paint_passes,
            total_saved_animation_ticks,
            total_saved_refreshes,
            total_saved_polls,
            saved_by_class: audit.saved_by_class.clone(),
            trace_marks,
        }
    }
}

/// Operator-facing diagnostics projection of hidden-surface suppression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenSurfaceDiagnosticsProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Active efficiency state token.
    pub efficiency_state: String,
    /// One-sentence summary for the diagnostics header.
    pub summary_label: String,
    /// Number of surfaces audited.
    pub audited_surface_count: usize,
    /// Number of hidden or off-screen surfaces audited.
    pub hidden_surface_count: usize,
    /// Total units saved by suppression and throttling.
    pub total_saved_units: u32,
    /// True when no hidden surface kept forbidden work alive.
    pub passes_policy: bool,
    /// Number of hidden-pane violations the audit found.
    pub hidden_pane_violation_count: u32,
    /// True when every suppressed surface restores correctly on resume.
    pub all_resumes_correct: bool,
    /// Protected interactions the policy may not narrow.
    pub protected_interactions_preserved: Vec<String>,
    /// True when durability stayed preserved.
    pub durability_preserved: bool,
    /// Energy/thermal savings attributed per surface class.
    pub saved_by_class: Vec<HiddenSurfaceClassSaving>,
    /// Open-details command id.
    pub primary_command_id: String,
    /// Surface ref the open-details command opens.
    pub opens_surface_ref: String,
    /// Energy/thermal trace record kind this diagnostics view quotes.
    pub energy_trace_ref: String,
    /// Observation timestamp.
    pub observed_at: String,
}

impl HiddenSurfaceDiagnosticsProjection {
    /// Projects the canonical audit into an operator-facing diagnostics view.
    pub fn from_audit(audit: &HiddenSurfaceSuppressionAudit) -> Self {
        Self {
            record_kind: HIDDEN_SURFACE_DIAGNOSTICS_RECORD_KIND.to_owned(),
            schema_version: HIDDEN_SURFACE_SCHEMA_VERSION,
            efficiency_state: audit.efficiency_state.clone(),
            summary_label: summary_label_for(audit),
            audited_surface_count: audit.audited_surface_count,
            hidden_surface_count: audit.hidden_surface_count,
            total_saved_units: audit.total_saved_units,
            passes_policy: audit.passes_policy,
            hidden_pane_violation_count: audit.hidden_pane_violation_count,
            all_resumes_correct: audit.all_resumes_correct,
            protected_interactions_preserved: audit.protected_interactions_preserved.clone(),
            durability_preserved: audit.preserves_durability_truth(),
            saved_by_class: audit.saved_by_class.clone(),
            primary_command_id: HIDDEN_SURFACE_INSPECT_COMMAND_ID.to_owned(),
            opens_surface_ref: HIDDEN_SURFACE_DETAILS_SURFACE_REF.to_owned(),
            energy_trace_ref: HIDDEN_SURFACE_ENERGY_TRACE_RECORD_KIND.to_owned(),
            observed_at: audit.observed_at.clone(),
        }
    }
}

fn committed_units(
    channel: HiddenWorkChannel,
    activity: SurfaceActivity,
    state: EfficiencyState,
    requested: u32,
) -> u32 {
    if requested == 0 {
        return 0;
    }
    match activity {
        SurfaceActivity::Active => requested,
        SurfaceActivity::VisibleInactive => match channel {
            // A visible (if unfocused) surface must still paint, and its
            // correctness work stays truthful; only its decorative and
            // speculative channels throttle down.
            HiddenWorkChannel::Paint | HiddenWorkChannel::CorrectnessPoll => requested,
            _ => match state {
                EfficiencyState::ProtectCore => 0,
                EfficiencyState::ThermalConstrained => quarter(requested),
                _ => half(requested),
            },
        },
        SurfaceActivity::Hidden => {
            if channel.is_correctness_critical() {
                // Throttled to a non-zero floor so a running task, captured
                // events, or a live feed stay truthful for resume.
                match state {
                    EfficiencyState::Nominal => requested,
                    EfficiencyState::ProtectCore => 1,
                    _ => half(requested),
                }
            } else {
                // Decorative paint, animation, rich refresh, and speculative
                // polling are dropped to zero while hidden.
                0
            }
        }
    }
}

const fn half(requested: u32) -> u32 {
    let reduced = requested / 2;
    if reduced == 0 {
        1
    } else {
        reduced
    }
}

const fn quarter(requested: u32) -> u32 {
    let reduced = requested / 4;
    if reduced == 0 {
        1
    } else {
        reduced
    }
}

fn disposition_for(requested: u32, committed: u32) -> HiddenWorkDisposition {
    if requested == 0 || committed >= requested {
        HiddenWorkDisposition::Maintained
    } else if committed == 0 {
        HiddenWorkDisposition::Suppressed
    } else {
        HiddenWorkDisposition::Throttled
    }
}

fn behavior_for(
    channel: HiddenWorkChannel,
    disposition: HiddenWorkDisposition,
) -> Option<HiddenPaneBehavior> {
    use HiddenWorkChannel as Channel;
    use HiddenWorkDisposition as Disposition;
    match (channel, disposition) {
        (Channel::Paint | Channel::RichRefresh, Disposition::Suppressed) => {
            Some(HiddenPaneBehavior::RenderSuppressed)
        }
        (Channel::Animation, Disposition::Suppressed) => {
            Some(HiddenPaneBehavior::AnimationSuppressed)
        }
        (Channel::SpeculativePoll, Disposition::Suppressed) => {
            Some(HiddenPaneBehavior::PollingPaused)
        }
        (Channel::CorrectnessPoll, Disposition::Maintained | Disposition::Throttled) => {
            Some(HiddenPaneBehavior::CorrectnessPollOnly)
        }
        _ => None,
    }
}

fn rationale_for(
    channel: HiddenWorkChannel,
    activity: SurfaceActivity,
    state: EfficiencyState,
    disposition: HiddenWorkDisposition,
) -> String {
    match (activity, disposition) {
        (_, HiddenWorkDisposition::Maintained) => {
            format!("{} runs within its current budget.", channel.as_str())
        }
        (SurfaceActivity::Hidden, HiddenWorkDisposition::Suppressed) => format!(
            "{} is dropped while the surface is hidden or off-screen.",
            channel.as_str()
        ),
        (SurfaceActivity::VisibleInactive, HiddenWorkDisposition::Suppressed) => format!(
            "{} is dropped while the surface is in the background under {}.",
            channel.as_str(),
            state.label()
        ),
        (SurfaceActivity::Hidden, HiddenWorkDisposition::Throttled) => format!(
            "{} is throttled to a correctness floor while hidden under {}.",
            channel.as_str(),
            state.label()
        ),
        (_, HiddenWorkDisposition::Throttled) => format!(
            "{} is throttled while the surface is in the background under {}.",
            channel.as_str(),
            state.label()
        ),
        (_, HiddenWorkDisposition::Suppressed) => format!(
            "{} is dropped while the surface is not contributing under {}.",
            channel.as_str(),
            state.label()
        ),
    }
}

fn saved_by_class(decisions: &[HiddenSurfaceDecision]) -> Vec<HiddenSurfaceClassSaving> {
    HiddenSurfaceClass::ALL
        .iter()
        .filter_map(|class| {
            let token = class.as_str();
            let class_decisions = decisions
                .iter()
                .filter(|decision| decision.surface_class == token)
                .collect::<Vec<_>>();
            if class_decisions.is_empty() {
                return None;
            }
            let saved_paint_passes = class_decisions.iter().map(|d| d.saved_paint_passes).sum();
            let saved_animation_ticks = class_decisions
                .iter()
                .map(|d| d.saved_animation_ticks)
                .sum();
            let saved_refreshes = class_decisions.iter().map(|d| d.saved_refreshes).sum();
            let saved_polls = class_decisions.iter().map(|d| d.saved_polls).sum();
            let saved_units_total = class_decisions.iter().map(|d| d.total_saved_units).sum();
            let hidden_surface_count = class_decisions.iter().filter(|d| d.hidden).count();
            Some(HiddenSurfaceClassSaving {
                surface_class: token.to_owned(),
                owner_label: class.owner_label().to_owned(),
                hidden_surface_count,
                saved_paint_passes,
                saved_animation_ticks,
                saved_refreshes,
                saved_polls,
                saved_units_total,
            })
        })
        .collect()
}

fn summary_label_for(audit: &HiddenSurfaceSuppressionAudit) -> String {
    if audit.hidden_surface_count == 0 {
        return format!(
            "No hidden surfaces under {}; {} visible surface(s) keep their budget.",
            audit.efficiency_state, audit.audited_surface_count
        );
    }
    format!(
        "{} hidden surface(s) shed {} unit(s) of render, animation, refresh, and speculative polling under {}.",
        audit.hidden_surface_count, audit.total_saved_units, audit.efficiency_state
    )
}

/// Builds a deterministic audit, energy trace, and diagnostics projection for a
/// seeded scenario. Seeded scenarios back the dump example, the checked-in
/// fixtures, and cross-surface tests so the three surfaces always derive from
/// one object.
pub fn seed_hidden_surface_case(
    scenario_id: &str,
    state: EfficiencyState,
    surfaces: Vec<HiddenSurfaceInput>,
    window_label: &str,
    observed_at: &str,
) -> HiddenSurfaceCase {
    let audit = HiddenSurfaceSuppressionAudit::for_surfaces(state, &surfaces, observed_at);
    let energy_trace = HiddenSurfaceEnergyTrace::from_audit(&audit, window_label);
    let diagnostics = HiddenSurfaceDiagnosticsProjection::from_audit(&audit);
    HiddenSurfaceCase {
        scenario_id: scenario_id.to_owned(),
        efficiency_state: state,
        observed_at: observed_at.to_owned(),
        window_label: window_label.to_owned(),
        surfaces,
        audit,
        energy_trace,
        diagnostics,
    }
}

/// One seeded hidden-surface scenario together with the audit, energy trace, and
/// diagnostics projection it produces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenSurfaceCase {
    /// Stable scenario id.
    pub scenario_id: String,
    /// Active efficiency state.
    pub efficiency_state: EfficiencyState,
    /// Observation timestamp.
    pub observed_at: String,
    /// Trace window label.
    pub window_label: String,
    /// Surfaces in the scenario.
    pub surfaces: Vec<HiddenSurfaceInput>,
    /// Canonical suppression audit.
    pub audit: HiddenSurfaceSuppressionAudit,
    /// Energy/thermal trace projection.
    pub energy_trace: HiddenSurfaceEnergyTrace,
    /// Operator diagnostics projection.
    pub diagnostics: HiddenSurfaceDiagnosticsProjection,
}

/// A request that exercises every channel of a surface class, for seeds.
fn full_request() -> HiddenWorkRequest {
    HiddenWorkRequest {
        paint_passes: 6,
        animation_ticks: 12,
        rich_refreshes: 4,
        speculative_polls: 8,
        correctness_polls: 4,
    }
}

fn surface(
    surface_id: &str,
    class: HiddenSurfaceClass,
    visibility: VisibilityState,
) -> HiddenSurfaceInput {
    HiddenSurfaceInput {
        surface_id: surface_id.to_owned(),
        surface_class: class,
        visibility_state: visibility,
        requested: full_request(),
    }
}

/// The full set of representative hidden-surface scenarios. Together they cover
/// every governed surface class, the three suppression cases (hidden, off-screen,
/// inactive preview), and every efficiency state that changes suppression.
pub fn seeded_hidden_surface_cases() -> Vec<HiddenSurfaceCase> {
    use HiddenSurfaceClass as Class;
    use VisibilityState as Vis;
    vec![
        seed_hidden_surface_case(
            "all-classes-hidden-nominal",
            EfficiencyState::Nominal,
            vec![
                surface("notebook.hidden", Class::Notebook, Vis::HiddenTab),
                surface("trace.hidden", Class::Trace, Vis::HiddenTab),
                surface("preview.collapsed", Class::Preview, Vis::CollapsedSplit),
                surface("docs.occluded", Class::DocsBrowser, Vis::OccludedWindow),
                surface(
                    "pipeline.offscreen",
                    Class::Pipeline,
                    Vis::DetachedOffscreen,
                ),
                surface("incident.hidden", Class::Incident, Vis::HiddenTab),
            ],
            "hidden-surface shedding window",
            "2026-06-20T15:00:00Z",
        ),
        seed_hidden_surface_case(
            "thermal-mixed-visibility",
            EfficiencyState::ThermalConstrained,
            vec![
                surface("notebook.active", Class::Notebook, Vis::VisibleFocused),
                surface("preview.inactive", Class::Preview, Vis::VisibleBackground),
                surface("trace.hidden", Class::Trace, Vis::HiddenTab),
                surface(
                    "pipeline.offscreen",
                    Class::Pipeline,
                    Vis::DetachedOffscreen,
                ),
            ],
            "thermal-pressure shedding window",
            "2026-06-20T15:01:00Z",
        ),
        seed_hidden_surface_case(
            "protect-core-critical-battery",
            EfficiencyState::ProtectCore,
            vec![
                surface("notebook.hidden", Class::Notebook, Vis::HiddenTab),
                surface("preview.inactive", Class::Preview, Vis::VisibleBackground),
                surface(
                    "incident.offscreen",
                    Class::Incident,
                    Vis::DetachedOffscreen,
                ),
            ],
            "protect-core shedding window",
            "2026-06-20T15:02:00Z",
        ),
        seed_hidden_surface_case(
            "battery-saver-inactive-preview",
            EfficiencyState::EfficiencyAware,
            vec![
                surface("preview.inactive", Class::Preview, Vis::VisibleBackground),
                surface("docs.inactive", Class::DocsBrowser, Vis::VisibleBackground),
                surface("docs.hidden", Class::DocsBrowser, Vis::HiddenTab),
            ],
            "battery-saver shedding window",
            "2026-06-20T15:03:00Z",
        ),
        seed_hidden_surface_case(
            "recovery-staged",
            EfficiencyState::Recovery,
            vec![
                surface("notebook.hidden", Class::Notebook, Vis::HiddenTab),
                surface("trace.offscreen", Class::Trace, Vis::DetachedOffscreen),
            ],
            "recovery shedding window",
            "2026-06-20T15:04:00Z",
        ),
    ]
}
