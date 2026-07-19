//! Implements the reusable execution-confidence primitive: an adapter-drift
//! banner (prior versus current adapter, capability delta, affected targets, and
//! recompute / open-diagnostics actions), a run / test / debug launcher-state
//! projection, a launcher-state-parity fan-out onto problem surfaces, artifact
//! views, and follow-on automation / AI actions, and a no-higher-confidence
//! overwrite guard — all resolved from one execution-target context that shares
//! one target identity and one disclosed adapter source, so downgraded discovery
//! or fallback results can never masquerade as native, protocol-backed truth
//! across the claimed M5 execution lanes.
//!
//! Where
//! [`crate::implement_the_m5_adapter_source_badge_target_graph_capability_matrix_raw_event_and_fallback_confidence_primitive`]
//! narrows the *static* build / run confidence surfaces of the frozen matrix (the
//! badge, the target-graph row, the capability matrix, the raw-event drawer, and
//! the fallback drawer), this module narrows the *execution-lane* surfaces those
//! surfaces feed: when an adapter drifts between runs, a launcher must narrow its
//! affordances **before** launch, users must see the drift and the affected
//! targets **without** waiting for a failed rerun or debug attempt, and a
//! lower-confidence result must never silently overwrite existing
//! higher-confidence target or event truth.
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — execution lanes narrow affordances before launch when adapter
//!   capability drops.** The launcher derives its per-verb affordance from the
//!   prior-versus-current capability delta, so a verb that was lost is blocked and
//!   a verb that was downgraded is narrowed to inspect-only *before* any run /
//!   test / debug action is offered.
//! - **AC2 — adapter drift and affected targets are visible before action.** The
//!   drift banner always renders prior versus current adapter, the capability
//!   delta, and the affected targets, and — whenever drift is detected — carries a
//!   precise divergence note and offers recompute and open-diagnostics actions, so
//!   users never have to trigger a failed rerun to discover the drift.
//! - **AC3 — lower-confidence results never masquerade as native truth.** The
//!   overwrite guard refuses to replace existing higher-confidence (or native)
//!   truth with lower-confidence (or fallback) truth unless an explicit downgrade
//!   is acknowledged, in which case the higher-confidence truth is preserved and
//!   the downgrade is named; and the adapter source and confidence ride along into
//!   every launcher-state-parity consumer.
//!
//! Raw build output, event payloads, credentials, and endpoint data never cross
//! this boundary; the resolver carries only opaque refs, typed class tokens,
//! booleans, and redacted labels, so support and diagnostics exports reconstruct
//! exactly what a surface would have shown without leaking build or event
//! payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-execution-confidence-primitive.schema.json`](../../../../schemas/ui/m5-execution-confidence-primitive.schema.json).
//! The contract doc is
//! [`docs/infra/m5_execution_confidence_primitive.md`](../../../../docs/infra/m5_execution_confidence_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    truth_mode_token, DegradedState, M5AdapterSourceKind, M5BuildVerb, M5CapabilityState,
    M5DiscoveryConfidence, M5FallbackConfidenceState, M5ManifestBuildDowngradeTrigger,
    M5ResourceFreshness, M5TargetIdentity, TruthMode,
};

/// Stable record-kind tag carried by [`M5ExecutionConfidencePrimitivePacket`].
pub const M5_EXECUTION_CONFIDENCE_RECORD_KIND: &str = "m5_execution_confidence_primitive";

/// Schema version for the execution-confidence primitive packet.
pub const M5_EXECUTION_CONFIDENCE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_EXECUTION_CONFIDENCE_SCHEMA_REF: &str =
    "schemas/ui/m5-execution-confidence-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_EXECUTION_CONFIDENCE_DOC_REF: &str = "docs/infra/m5_execution_confidence_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive
/// narrows.
pub const M5_EXECUTION_CONFIDENCE_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-manifest-build-component-matrix.schema.json";

/// Repo-relative path of the sibling build / run confidence primitive this lane
/// extends into execution.
pub const M5_EXECUTION_CONFIDENCE_BUILD_PRIMITIVE_REF: &str =
    "schemas/ui/m5-build-confidence-primitive.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_EXECUTION_CONFIDENCE_FIXTURE_DIR: &str =
    "fixtures/ui/m5-execution-confidence-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const M5_EXECUTION_CONFIDENCE_ARTIFACT_REF: &str =
    "artifacts/release/m5-execution-confidence-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_EXECUTION_CONFIDENCE_CSV_REF: &str =
    "artifacts/release/m5-execution-confidence-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_EXECUTION_CONFIDENCE_REPORT_REF: &str =
    "artifacts/release/m5-execution-confidence-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed execution-confidence surface family. Each family is one parity surface
/// that ingests the shared primitive; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionSurfaceFamily {
    /// The adapter-drift banner (prior versus current adapter, capability delta,
    /// affected targets, recompute / diagnostics actions).
    AdapterDriftBanner,
    /// The run / test / debug launcher state that narrows affordances before launch.
    ExecutionLauncher,
    /// The launcher-state-parity fan-out onto problem surfaces, artifact views, and
    /// follow-on automation / AI actions.
    LauncherStateParity,
    /// The no-higher-confidence overwrite guard.
    OverwriteGuard,
    /// The support / export replay surface that reconstructs execution confidence.
    SupportExportReplay,
}

impl M5ExecutionSurfaceFamily {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AdapterDriftBanner,
        Self::ExecutionLauncher,
        Self::LauncherStateParity,
        Self::OverwriteGuard,
        Self::SupportExportReplay,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdapterDriftBanner => "adapter_drift_banner",
            Self::ExecutionLauncher => "execution_launcher",
            Self::LauncherStateParity => "launcher_state_parity",
            Self::OverwriteGuard => "overwrite_guard",
            Self::SupportExportReplay => "support_export_replay",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AdapterDriftBanner => "Adapter-drift banner",
            Self::ExecutionLauncher => "Execution launcher",
            Self::LauncherStateParity => "Launcher-state parity",
            Self::OverwriteGuard => "Overwrite guard",
            Self::SupportExportReplay => "Support / export replay",
        }
    }
}

/// Closed launcher-state-parity consumer vocabulary. Names the downstream surfaces
/// that must carry the same adapter source kind and confidence the launcher shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionParitySurface {
    /// A problem / diagnostics surface consuming the execution truth.
    ProblemSurface,
    /// An artifact / output view consuming the execution truth.
    ArtifactView,
    /// A follow-on automation action consuming the execution truth.
    FollowOnAutomation,
    /// A follow-on AI action consuming the execution truth.
    AiAction,
}

impl M5ExecutionParitySurface {
    /// Every parity consumer, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProblemSurface,
        Self::ArtifactView,
        Self::FollowOnAutomation,
        Self::AiAction,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProblemSurface => "problem_surface",
            Self::ArtifactView => "artifact_view",
            Self::FollowOnAutomation => "follow_on_automation",
            Self::AiAction => "ai_action",
        }
    }
}

/// Closed capability-delta vocabulary. Names how a verb's support changed between
/// the prior and current adapter so a drop is never read as a gain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilityDeltaKind {
    /// The verb gained support (current stronger than prior).
    Gained,
    /// The verb's support is unchanged.
    Retained,
    /// The verb was downgraded (current weaker than prior, but not fully lost).
    Downgraded,
    /// The verb was lost entirely (current unsupported).
    Lost,
}

impl M5CapabilityDeltaKind {
    /// Every delta kind, in declaration order.
    pub const ALL: [Self; 4] = [Self::Gained, Self::Retained, Self::Downgraded, Self::Lost];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gained => "gained",
            Self::Retained => "retained",
            Self::Downgraded => "downgraded",
            Self::Lost => "lost",
        }
    }

    /// True when this delta is a drop in capability (downgraded or lost).
    pub const fn is_drop(self) -> bool {
        matches!(self, Self::Downgraded | Self::Lost)
    }
}

/// Closed launcher-affordance vocabulary. Names how far a launcher narrows a verb
/// before launch when its adapter capability drops.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AffordanceState {
    /// The verb is launchable.
    Available,
    /// The verb is narrowed to inspect-only before launch.
    Narrowed,
    /// The verb is blocked before launch.
    Blocked,
}

impl M5AffordanceState {
    /// Every affordance state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Available, Self::Narrowed, Self::Blocked];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Narrowed => "narrowed",
            Self::Blocked => "blocked",
        }
    }

    /// True when the verb can be launched as-is.
    pub const fn is_launchable(self) -> bool {
        matches!(self, Self::Available)
    }
}

/// Closed overwrite-verdict vocabulary. Names the decision the no-higher-confidence
/// overwrite guard reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverwriteVerdict {
    /// The incoming truth is higher-confidence and promoted.
    PromotedHigherConfidence,
    /// The incoming truth matches the existing confidence and is accepted.
    MatchedExistingConfidence,
    /// The incoming truth is lower-confidence and recorded as an explicit downgrade,
    /// preserving the existing higher-confidence truth.
    RecordedExplicitDowngrade,
}

impl M5OverwriteVerdict {
    /// Every verdict, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::PromotedHigherConfidence,
        Self::MatchedExistingConfidence,
        Self::RecordedExplicitDowngrade,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PromotedHigherConfidence => "promoted_higher_confidence",
            Self::MatchedExistingConfidence => "matched_existing_confidence",
            Self::RecordedExplicitDowngrade => "recorded_explicit_downgrade",
        }
    }

    /// True when the verdict recorded an explicit downgrade of confidence.
    pub const fn is_downgrade(self) -> bool {
        matches!(self, Self::RecordedExplicitDowngrade)
    }
}

/// Closed execution-action vocabulary. Names the safe actions a drift banner or
/// launcher offers so recompute and diagnostics stay available before any run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionActionKind {
    /// Recompute discovery to re-establish adapter confidence.
    Recompute,
    /// Open diagnostics for the drifted adapter.
    OpenDiagnostics,
    /// Inspect the capability matrix.
    InspectCapabilities,
    /// Copy / export the execution-confidence packet.
    CopyExport,
    /// Open the canonical source truth.
    OpenSourceTruth,
    /// Acknowledge the explicit downgrade before proceeding.
    AcknowledgeDowngrade,
}

impl M5ExecutionActionKind {
    /// Every action kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Recompute,
        Self::OpenDiagnostics,
        Self::InspectCapabilities,
        Self::CopyExport,
        Self::OpenSourceTruth,
        Self::AcknowledgeDowngrade,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Recompute => "recompute",
            Self::OpenDiagnostics => "open_diagnostics",
            Self::InspectCapabilities => "inspect_capabilities",
            Self::CopyExport => "copy_export",
            Self::OpenSourceTruth => "open_source_truth",
            Self::AcknowledgeDowngrade => "acknowledge_downgrade",
        }
    }

    /// True when this action exports or copies confidence truth for reuse.
    pub const fn is_export(self) -> bool {
        matches!(self, Self::CopyExport)
    }

    /// True when this action recomputes discovery.
    pub const fn is_recompute(self) -> bool {
        matches!(self, Self::Recompute)
    }

    /// True when this action opens diagnostics.
    pub const fn is_diagnostics(self) -> bool {
        matches!(self, Self::OpenDiagnostics)
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet
/// must carry per surface; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionExportField {
    /// The stable target identity shared across surfaces.
    TargetId,
    /// The typed target identity (node kind, stable id, owning module, root).
    TargetIdentity,
    /// The prior adapter source the drift banner names.
    PriorAdapter,
    /// The current adapter source truth came from.
    CurrentAdapter,
    /// The discovery confidence.
    Confidence,
    /// The target / result freshness.
    Freshness,
    /// The no-higher-confidence overwrite verdict.
    OverwriteVerdict,
}

impl M5ExecutionExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::TargetId,
        Self::TargetIdentity,
        Self::PriorAdapter,
        Self::CurrentAdapter,
        Self::Confidence,
        Self::Freshness,
        Self::OverwriteVerdict,
    ];

    /// The mandatory subset every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::TargetId,
        Self::TargetIdentity,
        Self::CurrentAdapter,
        Self::Confidence,
        Self::OverwriteVerdict,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetId => "target_id",
            Self::TargetIdentity => "target_identity",
            Self::PriorAdapter => "prior_adapter",
            Self::CurrentAdapter => "current_adapter",
            Self::Confidence => "confidence",
            Self::Freshness => "freshness",
            Self::OverwriteVerdict => "overwrite_verdict",
        }
    }
}

// --- shared value structs ---

/// One requested verb with its prior and current support state, so the resolver
/// can compute the capability delta between adapters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionVerbInput {
    /// The build / run verb this input describes.
    pub verb: M5BuildVerb,
    /// The support state under the prior adapter.
    pub prior_state: M5CapabilityState,
    /// The support state under the current adapter.
    pub current_state: M5CapabilityState,
}

/// One resolved verb delta: prior versus current state and the classified change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedVerbDelta {
    /// The build / run verb this delta describes.
    pub verb: M5BuildVerb,
    /// The support state under the prior adapter.
    pub prior_state: M5CapabilityState,
    /// The support state under the current adapter.
    pub current_state: M5CapabilityState,
    /// How the verb's support changed.
    pub delta: M5CapabilityDeltaKind,
}

/// One resolved launcher affordance: the verb, its current state, and how far the
/// launcher narrows it before launch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedLaunchAffordance {
    /// The build / run verb this affordance describes.
    pub verb: M5BuildVerb,
    /// The support state under the current adapter.
    pub current_state: M5CapabilityState,
    /// How far the launcher narrows the verb before launch.
    pub affordance: M5AffordanceState,
    /// True when the verb can be launched as-is (affordance is available).
    pub launchable_before_run: bool,
}

// --- resolver input ---

/// The full input to the execution-confidence resolver for one target context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionConfidenceInput {
    /// The stable target identity that must survive across the drift banner, the
    /// launcher, the parity consumers, and the overwrite guard.
    pub target_id: String,
    /// Opaque ref to the target object; never raw build bytes.
    pub target_ref: String,
    /// Human-readable target label.
    pub target_label: String,
    /// The typed target identity (node kind, stable id, owning module, root).
    pub identity: M5TargetIdentity,
    /// The truth class the target is shown in.
    pub truth_mode: TruthMode,
    /// The adapter source the prior run resolved.
    pub prior_adapter: M5AdapterSourceKind,
    /// The adapter source the current run resolved.
    pub current_adapter: M5AdapterSourceKind,
    /// The adapter version the banner names; opaque token.
    pub adapter_version: String,
    /// The confidence of the current discovered target / capability truth.
    pub confidence: M5DiscoveryConfidence,
    /// The freshness of the current target / result data.
    pub freshness: M5ResourceFreshness,
    /// The structured-versus-heuristic confidence state.
    pub fallback_state: M5FallbackConfidenceState,
    /// The requested verbs with prior and current support (must be non-empty).
    pub verbs: Vec<M5ExecutionVerbInput>,
    /// The targets affected by the drift; each must be a stable identity. Required
    /// when drift is detected.
    pub affected_targets: Vec<M5TargetIdentity>,
    /// A precise divergence note; required when drift is detected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divergence_note: Option<String>,
    /// The launcher-state-parity consumers that must carry adapter source and
    /// confidence (must be non-empty).
    pub parity_consumers: Vec<M5ExecutionParitySurface>,
    /// The confidence of the existing target / event truth this run would replace.
    pub existing_confidence: M5DiscoveryConfidence,
    /// The adapter source of the existing target / event truth.
    pub existing_adapter: M5AdapterSourceKind,
    /// Whether an explicit downgrade of confidence was acknowledged.
    pub downgrade_acknowledged: bool,
    /// A precise downgrade note; required when a downgrade is recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_note: Option<String>,
    /// The safe actions offered on the drift banner / launcher (recompute / open
    /// diagnostics / inspect / copy-export / open source truth / acknowledge).
    pub available_actions: Vec<M5ExecutionActionKind>,
    /// An externally-observed narrowing (adapter loss, channel loss, policy block)
    /// that degrades the surface before action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

// --- resolved projections ---

/// The resolved adapter-drift banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAdapterDriftBanner {
    /// The target identity — identical to every other surface.
    pub target_id: String,
    /// The adapter source the prior run resolved.
    pub prior_adapter: M5AdapterSourceKind,
    /// The adapter source the current run resolved.
    pub current_adapter: M5AdapterSourceKind,
    /// True when the adapter changed between runs.
    pub adapter_changed: bool,
    /// The per-verb capability delta between adapters.
    pub capability_delta: Vec<M5ResolvedVerbDelta>,
    /// The verbs that gained support.
    pub gained_verbs: Vec<M5BuildVerb>,
    /// The verbs that were downgraded (weaker but not fully lost).
    pub downgraded_verbs: Vec<M5BuildVerb>,
    /// The verbs that were lost entirely.
    pub lost_verbs: Vec<M5BuildVerb>,
    /// The targets affected by the drift.
    pub affected_targets: Vec<M5TargetIdentity>,
    /// True when adapter drift or a capability drop was detected.
    pub drift_detected: bool,
    /// The precise divergence note, when drift is detected.
    pub divergence_note: Option<String>,
    /// The actions the banner offers.
    pub actions: Vec<M5ExecutionActionKind>,
    /// Drift and affected targets are visible before any action; always holds.
    pub visible_before_action: bool,
}

/// The resolved run / test / debug launcher state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedExecutionLauncher {
    /// The target identity — identical to every other surface.
    pub target_id: String,
    /// The current adapter source the launcher discloses.
    pub adapter_source: M5AdapterSourceKind,
    /// The confidence the launcher discloses.
    pub confidence: M5DiscoveryConfidence,
    /// The freshness the launcher discloses.
    pub freshness: M5ResourceFreshness,
    /// The per-verb affordances the launcher offers.
    pub affordances: Vec<M5ResolvedLaunchAffordance>,
    /// The verbs blocked before launch.
    pub blocked_verbs: Vec<M5BuildVerb>,
    /// True when any verb was narrowed or blocked before launch.
    pub narrowed_before_launch: bool,
    /// The launcher carries the adapter source and confidence; always holds.
    pub carries_adapter_source_and_confidence: bool,
}

/// One resolved launcher-state-parity consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedParityConsumer {
    /// Which downstream consumer this is.
    pub surface: M5ExecutionParitySurface,
    /// The adapter source the consumer carries.
    pub adapter_source: M5AdapterSourceKind,
    /// The confidence the consumer carries.
    pub confidence: M5DiscoveryConfidence,
    /// The consumer carries the adapter source; always holds.
    pub carries_adapter_source: bool,
    /// The consumer carries the confidence; always holds.
    pub carries_confidence: bool,
}

/// The resolved no-higher-confidence overwrite guard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedOverwriteGuard {
    /// The target identity — identical to every other surface.
    pub target_id: String,
    /// The adapter source of the existing truth.
    pub existing_adapter: M5AdapterSourceKind,
    /// The confidence of the existing truth.
    pub existing_confidence: M5DiscoveryConfidence,
    /// The adapter source of the incoming truth (the current adapter).
    pub incoming_adapter: M5AdapterSourceKind,
    /// The confidence of the incoming truth.
    pub incoming_confidence: M5DiscoveryConfidence,
    /// The overwrite verdict.
    pub verdict: M5OverwriteVerdict,
    /// True when the guard recorded an explicit downgrade.
    pub explicit_downgrade_recorded: bool,
    /// The existing higher-confidence truth is preserved and inspectable; always
    /// holds.
    pub preserves_higher_confidence_truth: bool,
    /// The precise downgrade note, when a downgrade is recorded.
    pub downgrade_note: Option<String>,
    /// Lower-confidence truth never overwrites higher-confidence truth silently;
    /// always holds.
    pub never_overwrites_higher_silently: bool,
    /// Why the surface is narrowed, when it is; names a real, reconstructable
    /// trigger.
    pub downgrade_trigger: Option<M5ManifestBuildDowngradeTrigger>,
}

/// The resolved execution-confidence truth shared across the drift banner, the
/// launcher, the parity consumers, and the overwrite guard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedExecutionConfidence {
    /// The stable target identity.
    pub target_id: String,
    /// The resolved adapter-drift banner.
    pub drift_banner: M5ResolvedAdapterDriftBanner,
    /// The resolved run / test / debug launcher state.
    pub launcher: M5ResolvedExecutionLauncher,
    /// The resolved launcher-state-parity consumers.
    pub parity_consumers: Vec<M5ResolvedParityConsumer>,
    /// The resolved no-higher-confidence overwrite guard.
    pub overwrite_guard: M5ResolvedOverwriteGuard,
    /// Execution affordances narrow before launch when capability drops (AC1);
    /// always holds.
    pub affordances_narrowed_before_launch: bool,
    /// Adapter drift and affected targets are visible before action (AC2); always
    /// holds.
    pub drift_visible_before_action: bool,
    /// Lower-confidence results never masquerade as native truth (AC3); always
    /// holds.
    pub lower_confidence_never_masquerades: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedExecutionConfidence {
    /// True when the target identity is identical across the drift banner, the
    /// launcher, and the overwrite guard.
    pub fn identity_consistent(&self) -> bool {
        self.drift_banner.target_id == self.target_id
            && self.launcher.target_id == self.target_id
            && self.overwrite_guard.target_id == self.target_id
    }

    /// True when the drift banner reports a capability drop (any downgraded or lost
    /// verb).
    pub fn capability_drop_present(&self) -> bool {
        !self.drift_banner.downgraded_verbs.is_empty() || !self.drift_banner.lost_verbs.is_empty()
    }

    /// True when the execution lanes narrow affordances before launch whenever
    /// adapter capability drops: every lost verb is blocked and every downgraded
    /// verb is at least narrowed, before any run / test / debug action (AC1).
    pub fn affordances_narrow_when_capability_drops(&self) -> bool {
        if !self.capability_drop_present() {
            return true;
        }
        if !self.launcher.narrowed_before_launch {
            return false;
        }
        let lost_blocked = self.drift_banner.lost_verbs.iter().all(|verb| {
            self.launcher
                .affordances
                .iter()
                .find(|a| a.verb == *verb)
                .is_some_and(|a| a.affordance == M5AffordanceState::Blocked)
        });
        let downgraded_narrowed = self.drift_banner.downgraded_verbs.iter().all(|verb| {
            self.launcher
                .affordances
                .iter()
                .find(|a| a.verb == *verb)
                .is_some_and(|a| !a.affordance.is_launchable())
        });
        lost_blocked && downgraded_narrowed
    }

    /// True when adapter drift and the affected targets are visible before action:
    /// whenever drift is detected the banner names the divergence, lists affected
    /// targets, and offers recompute and open-diagnostics actions (AC2).
    pub fn drift_visible_and_actionable(&self) -> bool {
        if !self.drift_banner.visible_before_action {
            return false;
        }
        if !self.drift_banner.drift_detected {
            return true;
        }
        self.drift_banner.divergence_note.is_some()
            && !self.drift_banner.affected_targets.is_empty()
            && self.drift_banner.actions.iter().any(|a| a.is_recompute())
            && self.drift_banner.actions.iter().any(|a| a.is_diagnostics())
    }

    /// True when lower-confidence results never masquerade as native / higher truth:
    /// the guard never overwrites silently, any recorded downgrade preserves the
    /// higher-confidence truth and names it, and every parity consumer carries the
    /// adapter source and confidence (AC3).
    pub fn no_higher_confidence_masquerade(&self) -> bool {
        if !self.overwrite_guard.never_overwrites_higher_silently {
            return false;
        }
        if self.overwrite_guard.verdict.is_downgrade()
            && !(self.overwrite_guard.explicit_downgrade_recorded
                && self.overwrite_guard.preserves_higher_confidence_truth
                && self.overwrite_guard.downgrade_note.is_some())
        {
            return false;
        }
        !self.parity_consumers.is_empty()
            && self
                .parity_consumers
                .iter()
                .all(|c| c.carries_adapter_source && c.carries_confidence)
    }
}

/// Errors returned by [`resolve_execution_confidence`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ExecutionConfidenceResolutionError {
    /// The target id was empty.
    EmptyTargetId,
    /// The target ref was empty.
    EmptyTargetRef,
    /// The target label was empty.
    EmptyTargetLabel,
    /// The target identity carried no stable id, owning module, or root.
    EmptyTargetIdentity,
    /// The adapter version was empty.
    EmptyAdapterVersion,
    /// A label, ref, note, or identity carried forbidden material.
    ForbiddenMaterial,
    /// The confidence was inconsistent with the current adapter source (a fallback /
    /// imported / unknown source claimed high confidence).
    AdapterConfidenceInconsistent,
    /// No verbs were declared.
    NoVerbsDeclared,
    /// A supported verb was claimed from an unknown-confidence source.
    SupportedVerbUnknownConfidence,
    /// Drift was detected but no affected targets were named.
    DriftWithoutAffectedTargets,
    /// Drift was detected but no precise divergence note was given.
    DriftWithoutDivergenceDetail,
    /// Drift was detected but recompute / open-diagnostics actions were not offered.
    DriftWithoutRecoveryActions,
    /// An affected target was not a stable identity.
    AffectedTargetNotStable,
    /// No launcher-state-parity consumers were declared.
    NoParityConsumers,
    /// No safe action was offered on the surface.
    NoActionsOffered,
    /// No export / copy action was offered for support / AI reuse.
    NoExportActionOffered,
    /// A lower-confidence result would silently overwrite existing higher-confidence
    /// truth without an acknowledged downgrade.
    SilentHigherConfidenceOverwrite,
    /// A fallback / non-native result would silently replace existing native truth
    /// without an acknowledged downgrade.
    SilentNativeMasquerade,
    /// A downgrade was recorded but no precise note was given.
    DowngradeWithoutNote,
    /// A downgrade note was given but no downgrade was recorded.
    DowngradeNoteWithoutDowngrade,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5ExecutionConfidenceResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyTargetId => "empty_target_id",
            Self::EmptyTargetRef => "empty_target_ref",
            Self::EmptyTargetLabel => "empty_target_label",
            Self::EmptyTargetIdentity => "empty_target_identity",
            Self::EmptyAdapterVersion => "empty_adapter_version",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::AdapterConfidenceInconsistent => "adapter_confidence_inconsistent",
            Self::NoVerbsDeclared => "no_verbs_declared",
            Self::SupportedVerbUnknownConfidence => "supported_verb_unknown_confidence",
            Self::DriftWithoutAffectedTargets => "drift_without_affected_targets",
            Self::DriftWithoutDivergenceDetail => "drift_without_divergence_detail",
            Self::DriftWithoutRecoveryActions => "drift_without_recovery_actions",
            Self::AffectedTargetNotStable => "affected_target_not_stable",
            Self::NoParityConsumers => "no_parity_consumers",
            Self::NoActionsOffered => "no_actions_offered",
            Self::NoExportActionOffered => "no_export_action_offered",
            Self::SilentHigherConfidenceOverwrite => "silent_higher_confidence_overwrite",
            Self::SilentNativeMasquerade => "silent_native_masquerade",
            Self::DowngradeWithoutNote => "downgrade_without_note",
            Self::DowngradeNoteWithoutDowngrade => "downgrade_note_without_downgrade",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5ExecutionConfidenceResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "execution-confidence resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ExecutionConfidenceResolutionError {}

/// Resolves one execution-confidence context into its shared adapter-drift banner,
/// run / test / debug launcher state, launcher-state-parity consumers, and
/// no-higher-confidence overwrite guard.
///
/// The surfaces share one target identity and one disclosed adapter source, so
/// downgraded discovery or fallback results can never masquerade as native,
/// protocol-backed truth. Execution affordances narrow before launch when adapter
/// capability drops (AC1); the drift banner keeps drift and affected targets
/// visible before action (AC2); and the overwrite guard refuses to replace existing
/// higher-confidence truth with lower-confidence truth without an acknowledged
/// downgrade (AC3).
pub fn resolve_execution_confidence(
    input: &M5ExecutionConfidenceInput,
) -> Result<M5ResolvedExecutionConfidence, M5ExecutionConfidenceResolutionError> {
    if input.target_id.trim().is_empty() {
        return Err(M5ExecutionConfidenceResolutionError::EmptyTargetId);
    }
    if input.target_ref.trim().is_empty() {
        return Err(M5ExecutionConfidenceResolutionError::EmptyTargetRef);
    }
    if input.target_label.trim().is_empty() {
        return Err(M5ExecutionConfidenceResolutionError::EmptyTargetLabel);
    }
    if !input.identity.is_stable() {
        return Err(M5ExecutionConfidenceResolutionError::EmptyTargetIdentity);
    }
    if input.adapter_version.trim().is_empty() {
        return Err(M5ExecutionConfidenceResolutionError::EmptyAdapterVersion);
    }

    for value in [
        input.target_ref.as_str(),
        input.target_label.as_str(),
        input.identity.stable_id.as_str(),
        input.identity.owning_module.as_str(),
        input.identity.workspace_root.as_str(),
        input.adapter_version.as_str(),
    ]
    .into_iter()
    .chain(input.divergence_note.as_deref())
    .chain(input.downgrade_note.as_deref())
    .chain(
        input
            .affected_targets
            .iter()
            .flat_map(identity_strings)
            .map(|(_, value)| value),
    ) {
        if value_is_forbidden(value) {
            return Err(M5ExecutionConfidenceResolutionError::ForbiddenMaterial);
        }
    }

    // AC3: a heuristic / imported / unknown source can never claim high confidence.
    if !input
        .current_adapter
        .confidence_consistent(input.confidence)
    {
        return Err(M5ExecutionConfidenceResolutionError::AdapterConfidenceInconsistent);
    }

    if input.verbs.is_empty() {
        return Err(M5ExecutionConfidenceResolutionError::NoVerbsDeclared);
    }

    // AC1 / AC3: a supported verb may never be claimed from an unknown-confidence
    // source.
    if input.confidence == M5DiscoveryConfidence::Unknown
        && input
            .verbs
            .iter()
            .any(|v| v.current_state == M5CapabilityState::Supported)
    {
        return Err(M5ExecutionConfidenceResolutionError::SupportedVerbUnknownConfidence);
    }

    for affected in &input.affected_targets {
        if !affected.is_stable() {
            return Err(M5ExecutionConfidenceResolutionError::AffectedTargetNotStable);
        }
    }

    // Compute the per-verb capability delta and the drop sets.
    let capability_delta: Vec<M5ResolvedVerbDelta> = input
        .verbs
        .iter()
        .map(|v| M5ResolvedVerbDelta {
            verb: v.verb,
            prior_state: v.prior_state,
            current_state: v.current_state,
            delta: classify_delta(v.prior_state, v.current_state),
        })
        .collect();

    let gained_verbs = verbs_where(&capability_delta, M5CapabilityDeltaKind::Gained);
    let downgraded_verbs = verbs_where(&capability_delta, M5CapabilityDeltaKind::Downgraded);
    let lost_verbs = verbs_where(&capability_delta, M5CapabilityDeltaKind::Lost);

    let adapter_changed = input.prior_adapter != input.current_adapter;
    let capability_drop = !downgraded_verbs.is_empty() || !lost_verbs.is_empty();
    let drift_detected = adapter_changed || capability_drop;

    // AC2: when drift is detected, the banner must name the divergence, list
    // affected targets, and offer recompute and open-diagnostics actions.
    if drift_detected {
        if input.affected_targets.is_empty() {
            return Err(M5ExecutionConfidenceResolutionError::DriftWithoutAffectedTargets);
        }
        if input.divergence_note.is_none() {
            return Err(M5ExecutionConfidenceResolutionError::DriftWithoutDivergenceDetail);
        }
        let has_recompute = input.available_actions.iter().any(|a| a.is_recompute());
        let has_diagnostics = input.available_actions.iter().any(|a| a.is_diagnostics());
        if !has_recompute || !has_diagnostics {
            return Err(M5ExecutionConfidenceResolutionError::DriftWithoutRecoveryActions);
        }
    }

    if input.parity_consumers.is_empty() {
        return Err(M5ExecutionConfidenceResolutionError::NoParityConsumers);
    }

    if input.available_actions.is_empty() {
        return Err(M5ExecutionConfidenceResolutionError::NoActionsOffered);
    }
    // AC3: an export / copy action must be offered so support and AI consumers reuse
    // the component truth.
    if !input.available_actions.iter().any(|a| a.is_export()) {
        return Err(M5ExecutionConfidenceResolutionError::NoExportActionOffered);
    }

    // AC3: the no-higher-confidence overwrite rule.
    let incoming_rank = input.confidence.rank();
    let existing_rank = input.existing_confidence.rank();
    let would_mask_native =
        input.existing_adapter.is_native() && !input.current_adapter.is_native();
    let lower_confidence = incoming_rank < existing_rank;

    if lower_confidence && !input.downgrade_acknowledged {
        return Err(M5ExecutionConfidenceResolutionError::SilentHigherConfidenceOverwrite);
    }
    if would_mask_native && !lower_confidence && !input.downgrade_acknowledged {
        return Err(M5ExecutionConfidenceResolutionError::SilentNativeMasquerade);
    }

    let explicit_downgrade = lower_confidence || would_mask_native;
    if explicit_downgrade && input.downgrade_note.is_none() {
        return Err(M5ExecutionConfidenceResolutionError::DowngradeWithoutNote);
    }
    if !explicit_downgrade && input.downgrade_note.is_some() {
        return Err(M5ExecutionConfidenceResolutionError::DowngradeNoteWithoutDowngrade);
    }

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5ExecutionConfidenceResolutionError::DegradedLabelGeneric);
        }
    }

    let verdict = if explicit_downgrade {
        M5OverwriteVerdict::RecordedExplicitDowngrade
    } else if incoming_rank > existing_rank {
        M5OverwriteVerdict::PromotedHigherConfidence
    } else {
        M5OverwriteVerdict::MatchedExistingConfidence
    };

    let affordances: Vec<M5ResolvedLaunchAffordance> = capability_delta
        .iter()
        .map(|d| {
            let affordance = affordance_for(d.current_state, d.delta);
            M5ResolvedLaunchAffordance {
                verb: d.verb,
                current_state: d.current_state,
                affordance,
                launchable_before_run: affordance.is_launchable(),
            }
        })
        .collect();

    let blocked_verbs: Vec<M5BuildVerb> = affordances
        .iter()
        .filter(|a| a.affordance == M5AffordanceState::Blocked)
        .map(|a| a.verb)
        .collect();
    let narrowed_before_launch = affordances.iter().any(|a| !a.affordance.is_launchable());

    let downgrade_trigger = if let Some(degraded) = &input.degraded {
        Some(degraded.trigger)
    } else if explicit_downgrade {
        Some(overwrite_trigger(would_mask_native, lower_confidence))
    } else if drift_detected {
        Some(M5ManifestBuildDowngradeTrigger::DriftFromSource)
    } else {
        None
    };

    let drift_banner = M5ResolvedAdapterDriftBanner {
        target_id: input.target_id.clone(),
        prior_adapter: input.prior_adapter,
        current_adapter: input.current_adapter,
        adapter_changed,
        capability_delta,
        gained_verbs,
        downgraded_verbs,
        lost_verbs,
        affected_targets: input.affected_targets.clone(),
        drift_detected,
        divergence_note: input.divergence_note.clone(),
        actions: input.available_actions.clone(),
        visible_before_action: true,
    };

    let launcher = M5ResolvedExecutionLauncher {
        target_id: input.target_id.clone(),
        adapter_source: input.current_adapter,
        confidence: input.confidence,
        freshness: input.freshness,
        affordances,
        blocked_verbs,
        narrowed_before_launch,
        carries_adapter_source_and_confidence: true,
    };

    let parity_consumers: Vec<M5ResolvedParityConsumer> = input
        .parity_consumers
        .iter()
        .map(|surface| M5ResolvedParityConsumer {
            surface: *surface,
            adapter_source: input.current_adapter,
            confidence: input.confidence,
            carries_adapter_source: true,
            carries_confidence: true,
        })
        .collect();

    let overwrite_guard = M5ResolvedOverwriteGuard {
        target_id: input.target_id.clone(),
        existing_adapter: input.existing_adapter,
        existing_confidence: input.existing_confidence,
        incoming_adapter: input.current_adapter,
        incoming_confidence: input.confidence,
        verdict,
        explicit_downgrade_recorded: explicit_downgrade,
        preserves_higher_confidence_truth: true,
        downgrade_note: input.downgrade_note.clone(),
        never_overwrites_higher_silently: true,
        downgrade_trigger,
    };

    Ok(M5ResolvedExecutionConfidence {
        target_id: input.target_id.clone(),
        drift_banner,
        launcher,
        parity_consumers,
        overwrite_guard,
        affordances_narrowed_before_launch: narrowed_before_launch,
        drift_visible_before_action: true,
        lower_confidence_never_masquerades: true,
        degraded: input.degraded.clone(),
    })
}

/// Classifies how a verb's support changed between the prior and current adapter.
fn classify_delta(prior: M5CapabilityState, current: M5CapabilityState) -> M5CapabilityDeltaKind {
    let prior_rank = capability_rank(prior);
    let current_rank = capability_rank(current);
    if current_rank > prior_rank {
        M5CapabilityDeltaKind::Gained
    } else if current_rank == prior_rank {
        M5CapabilityDeltaKind::Retained
    } else if current == M5CapabilityState::Unsupported {
        M5CapabilityDeltaKind::Lost
    } else {
        M5CapabilityDeltaKind::Downgraded
    }
}

/// Ranks a capability state; higher is more capable. Used to classify deltas.
const fn capability_rank(state: M5CapabilityState) -> u8 {
    match state {
        M5CapabilityState::Supported => 4,
        M5CapabilityState::Partial => 3,
        M5CapabilityState::ProviderGated => 2,
        M5CapabilityState::Unknown => 1,
        M5CapabilityState::Unsupported => 0,
    }
}

/// Derives the launcher affordance from the current state and the delta: a lost
/// verb is blocked, a downgraded or otherwise-not-supported verb is narrowed, and a
/// retained / gained fully-supported verb stays available.
const fn affordance_for(
    current: M5CapabilityState,
    delta: M5CapabilityDeltaKind,
) -> M5AffordanceState {
    match delta {
        M5CapabilityDeltaKind::Lost => M5AffordanceState::Blocked,
        M5CapabilityDeltaKind::Downgraded => M5AffordanceState::Narrowed,
        M5CapabilityDeltaKind::Gained | M5CapabilityDeltaKind::Retained => match current {
            M5CapabilityState::Supported => M5AffordanceState::Available,
            M5CapabilityState::Unsupported => M5AffordanceState::Blocked,
            _ => M5AffordanceState::Narrowed,
        },
    }
}

/// Maps the overwrite situation to the reconstructable downgrade trigger it implies.
const fn overwrite_trigger(
    would_mask_native: bool,
    lower_confidence: bool,
) -> M5ManifestBuildDowngradeTrigger {
    if would_mask_native {
        M5ManifestBuildDowngradeTrigger::StructuredChannelLost
    } else if lower_confidence {
        M5ManifestBuildDowngradeTrigger::LowConfidenceDiscovery
    } else {
        M5ManifestBuildDowngradeTrigger::DriftFromSource
    }
}

/// Returns the verbs whose delta matches `kind`, in declaration order.
fn verbs_where(deltas: &[M5ResolvedVerbDelta], kind: M5CapabilityDeltaKind) -> Vec<M5BuildVerb> {
    deltas
        .iter()
        .filter(|d| d.delta == kind)
        .map(|d| d.verb)
        .collect()
}

/// The opaque string slots of a target identity, tagged for the forbidden scan.
fn identity_strings(identity: &M5TargetIdentity) -> Vec<(&'static str, &str)> {
    vec![
        ("stable_id", identity.stable_id.as_str()),
        ("owning_module", identity.owning_module.as_str()),
        ("workspace_root", identity.workspace_root.as_str()),
    ]
}

/// True when a label, ref, or note carries obviously forbidden material.
fn value_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs execution confidence from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionConfidenceCase {
    /// The resolver input.
    pub input: M5ExecutionConfidenceInput,
    /// The resolved execution confidence. Must equal
    /// `resolve_execution_confidence(&input)`.
    pub resolved: M5ResolvedExecutionConfidence,
}

impl M5ExecutionConfidenceCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ExecutionConfidenceInput) -> Self {
        let resolved =
            resolve_execution_confidence(&input).expect("seed execution-confidence case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_execution_confidence(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one execution surface family bound to the
/// shared execution-target contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionSurfaceRow {
    /// The execution surface family.
    pub surface_family: M5ExecutionSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Adapter source kinds this surface can disclose (must be non-empty).
    pub adapter_source_kinds: Vec<M5AdapterSourceKind>,
    /// Truth classes this surface renders (must be non-empty).
    pub truth_modes: Vec<TruthMode>,
    /// Build verbs this surface reasons about (must be non-empty).
    pub build_verbs: Vec<M5BuildVerb>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5ExecutionExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5ManifestBuildDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be
    /// non-empty).
    pub example_confidence: Vec<M5ExecutionConfidenceCase>,
    /// Hard invariant: this row never hides the adapter source. MUST be `false`.
    pub hides_adapter_source: bool,
    /// Hard invariant: this row never narrows affordances only after launch. MUST be
    /// `false`.
    pub narrows_after_launch: bool,
    /// Hard invariant: this row never hides adapter drift. MUST be `false`.
    pub hides_drift: bool,
    /// Hard invariant: this row never allows a silent higher-confidence overwrite.
    /// MUST be `false`.
    pub allows_silent_overwrite: bool,
}

impl M5ExecutionSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ExecutionExportField> =
            self.export_fields.iter().copied().collect();
        M5ExecutionExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.hides_adapter_source
            && !self.narrows_after_launch
            && !self.hides_drift
            && !self.allows_silent_overwrite
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionConfidenceVocabularySet {
    /// Execution surface-family tokens.
    pub surface_families: Vec<String>,
    /// Launcher-state-parity consumer tokens.
    pub parity_surfaces: Vec<String>,
    /// Capability-delta tokens.
    pub capability_delta_kinds: Vec<String>,
    /// Launcher-affordance tokens.
    pub affordance_states: Vec<String>,
    /// Overwrite-verdict tokens.
    pub overwrite_verdicts: Vec<String>,
    /// Execution-action tokens.
    pub action_kinds: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Build-verb tokens (reused from the build-confidence primitive).
    pub build_verbs: Vec<String>,
    /// Adapter-source tokens (reused from the frozen matrix).
    pub adapter_source_kinds: Vec<String>,
    /// Capability-state tokens (reused from the frozen matrix).
    pub capability_states: Vec<String>,
    /// Fallback-confidence-state tokens (reused from the frozen matrix).
    pub fallback_confidence_states: Vec<String>,
    /// Truth-class tokens (reused from the frozen matrix).
    pub truth_modes: Vec<String>,
    /// Resource-freshness tokens (reused from the frozen matrix).
    pub resource_freshness: Vec<String>,
    /// Discovery-confidence tokens (reused from the frozen matrix).
    pub discovery_confidence: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5ExecutionConfidenceVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5ExecutionSurfaceFamily::ALL, |v| v.as_str()),
            parity_surfaces: tokens(&M5ExecutionParitySurface::ALL, |v| v.as_str()),
            capability_delta_kinds: tokens(&M5CapabilityDeltaKind::ALL, |v| v.as_str()),
            affordance_states: tokens(&M5AffordanceState::ALL, |v| v.as_str()),
            overwrite_verdicts: tokens(&M5OverwriteVerdict::ALL, |v| v.as_str()),
            action_kinds: tokens(&M5ExecutionActionKind::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ExecutionExportField::ALL, |v| v.as_str()),
            build_verbs: tokens(&M5BuildVerb::ALL, |v| v.as_str()),
            adapter_source_kinds: tokens(&ADAPTER_SOURCE_KIND_ALL, |v| v.as_str()),
            capability_states: tokens(&CAPABILITY_STATE_ALL, |v| v.as_str()),
            fallback_confidence_states: tokens(&FALLBACK_CONFIDENCE_STATE_ALL, |v| v.as_str()),
            truth_modes: tokens(&TRUTH_MODE_ALL, truth_mode_token),
            resource_freshness: tokens(&RESOURCE_FRESHNESS_ALL, |v| v.as_str()),
            discovery_confidence: tokens(&DISCOVERY_CONFIDENCE_ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&DOWNGRADE_TRIGGER_ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The adapter source kinds reused from the frozen matrix, in a stable order.
const ADAPTER_SOURCE_KIND_ALL: [M5AdapterSourceKind; 6] = [
    M5AdapterSourceKind::NativeBuildServer,
    M5AdapterSourceKind::NativeBuildEvent,
    M5AdapterSourceKind::HeuristicParse,
    M5AdapterSourceKind::ImportedSnapshot,
    M5AdapterSourceKind::ProviderOverlay,
    M5AdapterSourceKind::Unknown,
];

/// The capability states reused from the frozen matrix, in a stable order.
const CAPABILITY_STATE_ALL: [M5CapabilityState; 5] = [
    M5CapabilityState::Supported,
    M5CapabilityState::Partial,
    M5CapabilityState::Unsupported,
    M5CapabilityState::Unknown,
    M5CapabilityState::ProviderGated,
];

/// The fallback-confidence states reused from the frozen matrix, in a stable order.
const FALLBACK_CONFIDENCE_STATE_ALL: [M5FallbackConfidenceState; 5] = [
    M5FallbackConfidenceState::StructuredHigh,
    M5FallbackConfidenceState::StructuredDegraded,
    M5FallbackConfidenceState::HeuristicFallback,
    M5FallbackConfidenceState::ImportedOnly,
    M5FallbackConfidenceState::Unknown,
];

/// The truth classes reused from the frozen matrix, in a stable order.
/// [`TruthMode`] is a pure token set, so the order is pinned here.
const TRUTH_MODE_ALL: [TruthMode; 5] = [
    TruthMode::Desired,
    TruthMode::Rendered,
    TruthMode::Plan,
    TruthMode::Live,
    TruthMode::ProviderOverlay,
];

/// The resource-freshness states reused from the frozen matrix, in a stable order.
const RESOURCE_FRESHNESS_ALL: [M5ResourceFreshness; 5] = [
    M5ResourceFreshness::LiveFresh,
    M5ResourceFreshness::CachedStale,
    M5ResourceFreshness::ImportedSnapshot,
    M5ResourceFreshness::PlanOnly,
    M5ResourceFreshness::Unknown,
];

/// The discovery-confidence states reused from the frozen matrix, in a stable
/// order.
const DISCOVERY_CONFIDENCE_ALL: [M5DiscoveryConfidence; 4] = [
    M5DiscoveryConfidence::High,
    M5DiscoveryConfidence::Medium,
    M5DiscoveryConfidence::Low,
    M5DiscoveryConfidence::Unknown,
];

/// The downgrade triggers reused from the frozen matrix, in a stable order.
const DOWNGRADE_TRIGGER_ALL: [M5ManifestBuildDowngradeTrigger; 8] = [
    M5ManifestBuildDowngradeTrigger::SchemaStale,
    M5ManifestBuildDowngradeTrigger::AdapterUnavailable,
    M5ManifestBuildDowngradeTrigger::ConnectorLoss,
    M5ManifestBuildDowngradeTrigger::PolicyBlock,
    M5ManifestBuildDowngradeTrigger::DriftFromSource,
    M5ManifestBuildDowngradeTrigger::LowConfidenceDiscovery,
    M5ManifestBuildDowngradeTrigger::StructuredChannelLost,
    M5ManifestBuildDowngradeTrigger::TargetContextUnresolved,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionConfidenceGovernanceReview {
    /// One primitive carries drift / launcher / parity / overwrite truth on every
    /// surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Target identity is preserved across the drift banner, launcher, parity
    /// consumers, and overwrite guard.
    pub target_identity_preserved_across_surfaces: bool,
    /// Adapter drift and affected targets are visible before action.
    pub adapter_drift_visible_before_action: bool,
    /// Execution affordances narrow before launch when capability drops.
    pub affordances_narrow_before_launch: bool,
    /// Lower-confidence truth never overwrites higher-confidence truth silently.
    pub lower_confidence_never_overwrites_silently: bool,
    /// Launcher-state parity carries adapter source and confidence downstream.
    pub launcher_state_parity_carries_source_and_confidence: bool,
    /// Later M5 rows cannot invent parallel execution-confidence vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionConfidenceConsumerProjection {
    /// Drift / launcher / parity / overwrite surfaces all consume the shared
    /// primitive.
    pub execution_surfaces_consume_shared_primitive: bool,
    /// The execution resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The launchers read a single canonical confidence source.
    pub launchers_read_single_confidence_source: bool,
    /// Support and AI consumers reuse the shared component truth.
    pub support_and_ai_reuse_shared_component: bool,
}

/// Release and support parity posture for the execution-confidence primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionConfidenceReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting confidence audit.
    pub confidence_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ExecutionConfidencePrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ExecutionConfidencePrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5ExecutionSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ExecutionConfidenceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ExecutionConfidenceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ExecutionConfidenceConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5ExecutionConfidenceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 execution-confidence primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionConfidencePrimitivePacket {
    /// Record kind; must equal [`M5_EXECUTION_CONFIDENCE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EXECUTION_CONFIDENCE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5ExecutionSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ExecutionConfidenceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ExecutionConfidenceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ExecutionConfidenceConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5ExecutionConfidenceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ExecutionConfidencePrimitivePacket {
    /// Builds an M5 execution-confidence primitive packet from stable-lane input.
    pub fn new(input: M5ExecutionConfidencePrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_EXECUTION_CONFIDENCE_RECORD_KIND.to_owned(),
            schema_version: M5_EXECUTION_CONFIDENCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 execution-confidence primitive invariants.
    pub fn validate(&self) -> Vec<M5ExecutionConfidenceViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_EXECUTION_CONFIDENCE_RECORD_KIND {
            violations.push(M5ExecutionConfidenceViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EXECUTION_CONFIDENCE_SCHEMA_VERSION {
            violations.push(M5ExecutionConfidenceViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ExecutionConfidenceViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 execution-confidence primitive serializes"),
        ) {
            violations.push(M5ExecutionConfidenceViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 execution-confidence primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,owner,adapter_sources,truth_modes,build_verbs,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.adapter_source_kinds, |v| v.as_str()),
                join_tokens(&row.truth_modes, |v| truth_mode_token(*v)),
                join_tokens(&row.build_verbs, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_confidence.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Execution-Confidence Primitive: Adapter-Drift Banner, Launcher State, Launcher-State Parity, and Overwrite Guard\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Execution surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5ExecutionSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Adapter sources: {}\n",
            self.vocabulary_set.adapter_source_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Build verbs: {}\n",
            self.vocabulary_set.build_verbs.join(", ")
        ));
        out.push_str(&format!(
            "- Overwrite verdicts: {}\n",
            self.vocabulary_set.overwrite_verdicts.join(", ")
        ));
        out.push_str("\n## Execution surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked cases: {}\n",
                row.example_confidence.len()
            ));
            for case in &row.example_confidence {
                out.push_str(&format!(
                    "    - `{}` → {} → {} ({}), confidence `{}`, {}\n",
                    case.resolved.target_id,
                    case.resolved.drift_banner.prior_adapter.as_str(),
                    case.resolved.drift_banner.current_adapter.as_str(),
                    truth_mode_token(case.input.truth_mode),
                    case.resolved.launcher.confidence.as_str(),
                    case.resolved.overwrite_guard.verdict.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 execution-confidence export.
#[derive(Debug)]
pub enum M5ExecutionConfidenceArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ExecutionConfidenceViolation>),
}

impl fmt::Display for M5ExecutionConfidenceArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 execution-confidence primitive export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 execution-confidence primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ExecutionConfidenceArtifactError {}

/// Validation failures emitted by [`M5ExecutionConfidencePrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ExecutionConfidenceViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required execution surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row declares no adapter source kinds.
    AdapterSourceMissing,
    /// A surface row declares no truth classes.
    TruthModeMissing,
    /// A surface row declares no build verbs.
    BuildVerbMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked confidence cases.
    ExampleConfidenceMissing,
    /// A worked confidence case does not match a fresh resolve of its input.
    ExampleConfidenceDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves affordances narrow before launch on a capability drop
    /// (AC1).
    AffordanceNarrowingUnproven,
    /// No worked case proves adapter drift and affected targets visible before action
    /// (AC2).
    DriftVisibilityUnproven,
    /// No worked case proves lower-confidence truth never masquerades as native
    /// (AC3).
    OverwriteGuardUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ExecutionConfidenceViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::AdapterSourceMissing => "adapter_source_missing",
            Self::TruthModeMissing => "truth_mode_missing",
            Self::BuildVerbMissing => "build_verb_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleConfidenceMissing => "example_confidence_missing",
            Self::ExampleConfidenceDrift => "example_confidence_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::AffordanceNarrowingUnproven => "affordance_narrowing_unproven",
            Self::DriftVisibilityUnproven => "drift_visibility_unproven",
            Self::OverwriteGuardUnproven => "overwrite_guard_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 execution-confidence export.
pub fn current_stable_m5_execution_confidence_export(
) -> Result<M5ExecutionConfidencePrimitivePacket, M5ExecutionConfidenceArtifactError> {
    let packet: M5ExecutionConfidencePrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-execution-confidence-primitive-proof/support_export.json"
    )))
    .map_err(M5ExecutionConfidenceArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ExecutionConfidenceArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ExecutionConfidencePrimitivePacket,
    violations: &mut Vec<M5ExecutionConfidenceViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_EXECUTION_CONFIDENCE_SCHEMA_REF,
        M5_EXECUTION_CONFIDENCE_DOC_REF,
        M5_EXECUTION_CONFIDENCE_COMPONENT_MATRIX_REF,
        M5_EXECUTION_CONFIDENCE_BUILD_PRIMITIVE_REF,
        M5_EXECUTION_CONFIDENCE_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ExecutionConfidenceViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ExecutionConfidencePrimitivePacket,
    violations: &mut Vec<M5ExecutionConfidenceViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ExecutionConfidenceViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5ExecutionConfidencePrimitivePacket,
    violations: &mut Vec<M5ExecutionConfidenceViolation>,
) {
    let present: BTreeSet<M5ExecutionSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5ExecutionSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5ExecutionConfidenceViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5ExecutionConfidenceViolation::SurfaceRowIncomplete);
        }
        if row.adapter_source_kinds.is_empty() {
            violations.push(M5ExecutionConfidenceViolation::AdapterSourceMissing);
        }
        if row.truth_modes.is_empty() {
            violations.push(M5ExecutionConfidenceViolation::TruthModeMissing);
        }
        if row.build_verbs.is_empty() {
            violations.push(M5ExecutionConfidenceViolation::BuildVerbMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ExecutionConfidenceViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ExecutionConfidenceViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ExecutionConfidenceViolation::ConsumerSurfacesMissing);
        }
        if row.example_confidence.is_empty() {
            violations.push(M5ExecutionConfidenceViolation::ExampleConfidenceMissing);
        }
        if row
            .example_confidence
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5ExecutionConfidenceViolation::ExampleConfidenceDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5ExecutionConfidenceViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated by at least one worked case
/// across the matrix: affordances narrow before launch on a capability drop (AC1),
/// adapter drift and affected targets visible before action (AC2), and
/// lower-confidence truth never masquerades as native (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5ExecutionConfidencePrimitivePacket,
    violations: &mut Vec<M5ExecutionConfidenceViolation>,
) {
    let cases: Vec<&M5ResolvedExecutionConfidence> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_confidence.iter().map(|case| &case.resolved))
        .collect();

    // AC1: some case exercises a capability drop and narrows the launcher, and every
    // case narrows affordances before launch whenever capability drops.
    let affordance_proven = cases.iter().any(|resolved| {
        resolved.capability_drop_present() && resolved.affordances_narrowed_before_launch
    }) && cases
        .iter()
        .all(|resolved| resolved.affordances_narrow_when_capability_drops());
    if !affordance_proven {
        violations.push(M5ExecutionConfidenceViolation::AffordanceNarrowingUnproven);
    }

    // AC2: some case detects drift, and every case keeps drift and affected targets
    // visible and actionable before action.
    let drift_proven = cases
        .iter()
        .any(|resolved| resolved.drift_banner.drift_detected)
        && cases.iter().all(|resolved| {
            resolved.identity_consistent() && resolved.drift_visible_and_actionable()
        });
    if !drift_proven {
        violations.push(M5ExecutionConfidenceViolation::DriftVisibilityUnproven);
    }

    // AC3: some case records an explicit downgrade (proving a lower-confidence lane
    // is not masked), and every case keeps the overwrite guard honest.
    let overwrite_proven = cases
        .iter()
        .any(|resolved| resolved.overwrite_guard.verdict.is_downgrade())
        && cases
            .iter()
            .all(|resolved| resolved.no_higher_confidence_masquerade());
    if !overwrite_proven {
        violations.push(M5ExecutionConfidenceViolation::OverwriteGuardUnproven);
    }
}

fn validate_governance_review(
    packet: &M5ExecutionConfidencePrimitivePacket,
    violations: &mut Vec<M5ExecutionConfidenceViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.target_identity_preserved_across_surfaces,
        review.adapter_drift_visible_before_action,
        review.affordances_narrow_before_launch,
        review.lower_confidence_never_overwrites_silently,
        review.launcher_state_parity_carries_source_and_confidence,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5ExecutionConfidenceViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ExecutionConfidencePrimitivePacket,
    violations: &mut Vec<M5ExecutionConfidenceViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.execution_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.launchers_read_single_confidence_source,
        projection.support_and_ai_reuse_shared_component,
    ] {
        if !ok {
            violations.push(M5ExecutionConfidenceViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5ExecutionConfidencePrimitivePacket,
    violations: &mut Vec<M5ExecutionConfidenceViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.confidence_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ExecutionConfidenceViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

include!("seed.rs");
