//! Canonical M5 efficiency-state governance matrix binding.
//!
//! This module binds the shell efficiency runtime to the frozen, checked-in
//! [M5 efficiency-state governance matrix][matrix]. The matrix freezes one typed
//! low-power vocabulary — efficiency state, source-of-change, throttled
//! subsystem, hidden-pane behaviour, visibility state, override posture, and
//! recovery state — so every M5 surface (notebooks, previews, docs/browser
//! panes, traces, pipelines, remote sessions, support exports, and
//! companion-adjacent views) reuses one vocabulary instead of inventing local
//! low-power wording.
//!
//! The three vocabularies this module *defines* —
//! [`HiddenPaneBehavior`], [`OverridePosture`], and [`EfficiencyRecoveryState`]
//! — extend the efficiency-state, source-of-change, throttled-subsystem, and
//! visibility vocabularies already in [`crate::efficiency`]. A conformance test
//! reads the frozen matrix from disk and asserts every closed vocabulary in the
//! artifact equals the tokens these enums and their parents emit, so the matrix
//! can never drift from what ships.
//!
//! Shell status and diagnostics surfaces consume
//! [`EfficiencyGovernanceProjection`], which stamps the matrix reference onto a
//! redaction-safe view of the active efficiency state rather than cloning prose.
//!
//! [matrix]: M5_EFFICIENCY_GOVERNANCE_MATRIX_REF

use serde::{Deserialize, Serialize};

use super::{
    EfficiencyPressureSource, EfficiencyState, EfficiencyStateSnapshot, VisibilityState,
    WorkloadFamily,
};

/// Repo-relative path to the canonical frozen M5 efficiency-governance matrix.
pub const M5_EFFICIENCY_GOVERNANCE_MATRIX_REF: &str =
    "artifacts/efficiency/m5-efficiency-governance.json";

/// Repo-relative path to the schema that validates the governance matrix.
pub const M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/efficiency/m5-efficiency-governance.schema.json";

/// Stable record kind carried by the frozen governance matrix.
pub const M5_EFFICIENCY_GOVERNANCE_RECORD_KIND: &str = "efficiency_m5_governance_matrix";

/// Behaviour a hidden, occluded, or off-screen pane adopts under the governance
/// matrix. These are the suppression outcomes a claim-bearing surface must prove
/// rather than leaving a hidden pane painting, animating, or polling off-screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HiddenPaneBehavior {
    /// Committed paint is dropped to zero for the hidden surface.
    RenderSuppressed,
    /// Decorative animation ticks are suppressed.
    AnimationSuppressed,
    /// Speculative polling is paused while hidden.
    PollingPaused,
    /// Only correctness polling continues; no render budget is committed.
    CorrectnessPollOnly,
    /// No paint, animation, or speculative poll runs while hidden.
    FullyQuiescent,
}

impl HiddenPaneBehavior {
    /// Stable token recorded in the governance matrix and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RenderSuppressed => "render_suppressed",
            Self::AnimationSuppressed => "animation_suppressed",
            Self::PollingPaused => "polling_paused",
            Self::CorrectnessPollOnly => "correctness_poll_only",
            Self::FullyQuiescent => "fully_quiescent",
        }
    }

    /// Every hidden-pane behaviour the frozen vocabulary admits, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::RenderSuppressed,
        Self::AnimationSuppressed,
        Self::PollingPaused,
        Self::CorrectnessPollOnly,
        Self::FullyQuiescent,
    ];
}

/// Whether and how a user may override an efficiency adaptation. Overrides are
/// explicit and policy-aware: a user-overridable posture must carry a policy
/// reference, and admin or local policy can block an override outright.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverridePosture {
    /// The adaptation protects core interaction and cannot be overridden.
    NotOverridable,
    /// The user may override the adaptation for the current session only.
    UserOverrideSessionOnly,
    /// The user may override the adaptation persistently.
    UserOverridePersistent,
    /// Admin or local policy blocks the override.
    PolicyBlocked,
    /// Only admin policy controls whether the adaptation applies.
    AdminControlled,
}

impl OverridePosture {
    /// Stable token recorded in the governance matrix and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotOverridable => "not_overridable",
            Self::UserOverrideSessionOnly => "user_override_session_only",
            Self::UserOverridePersistent => "user_override_persistent",
            Self::PolicyBlocked => "policy_blocked",
            Self::AdminControlled => "admin_controlled",
        }
    }

    /// True when the posture lets the user override the adaptation, so the
    /// matrix requires an explicit, policy-aware override reference.
    pub const fn is_user_overridable(self) -> bool {
        matches!(
            self,
            Self::UserOverrideSessionOnly | Self::UserOverridePersistent
        )
    }

    /// Every override posture the frozen vocabulary admits, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::NotOverridable,
        Self::UserOverrideSessionOnly,
        Self::UserOverridePersistent,
        Self::PolicyBlocked,
        Self::AdminControlled,
    ];
}

/// Recovery state of an efficiency posture as pressure clears. Recovery is
/// staged: deferred work resumes in order rather than thrashing back at once.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EfficiencyRecoveryState {
    /// No recovery is underway.
    NotInRecovery,
    /// Deferred work is resuming in stages.
    StagedResume,
    /// Recovery waits for the user to restore power.
    AwaitingUserRestorePower,
    /// Recovery waits for a remote reconnect.
    AwaitingReconnect,
    /// Recovery waits on admin policy.
    AwaitingAdminPolicy,
    /// The posture has fully recovered to nominal.
    Recovered,
}

impl EfficiencyRecoveryState {
    /// Stable token recorded in the governance matrix and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotInRecovery => "not_in_recovery",
            Self::StagedResume => "staged_resume",
            Self::AwaitingUserRestorePower => "awaiting_user_restore_power",
            Self::AwaitingReconnect => "awaiting_reconnect",
            Self::AwaitingAdminPolicy => "awaiting_admin_policy",
            Self::Recovered => "recovered",
        }
    }

    /// Every recovery state the frozen vocabulary admits, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::NotInRecovery,
        Self::StagedResume,
        Self::AwaitingUserRestorePower,
        Self::AwaitingReconnect,
        Self::AwaitingAdminPolicy,
        Self::Recovered,
    ];
}

/// Redaction-safe projection a shell status or diagnostics surface renders from
/// the canonical governance matrix instead of cloning low-power prose. It stamps
/// the matrix reference so the surface's vocabulary is traceable to one source
/// of truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyGovernanceProjection {
    /// Path to the canonical governance matrix this projection derives from.
    pub matrix_ref: String,
    /// Path to the schema validating the governance matrix.
    pub schema_ref: String,
    /// Active efficiency-state token.
    pub active_state: String,
    /// Source-of-change tokens that drove the active state.
    pub source_of_change: Vec<String>,
    /// Hidden-pane behaviours applied while the surface is hidden or off-screen.
    pub hidden_pane_behaviors: Vec<String>,
    /// Override posture token for the adaptation.
    pub override_posture: String,
    /// Recovery state token for the adaptation.
    pub recovery_state: String,
}

impl EfficiencyGovernanceProjection {
    /// Builds a projection from an [`EfficiencyStateSnapshot`] and the governed
    /// override and recovery vocabulary, binding it to the canonical matrix.
    ///
    /// This is the shell-surface consumer of the frozen matrix: the surface
    /// reads the snapshot's already-tokenized state and source, stamps the
    /// matrix reference, and renders the shared vocabulary rather than minting
    /// its own low-power wording.
    pub fn from_snapshot(
        snapshot: &EfficiencyStateSnapshot,
        hidden_pane_behaviors: &[HiddenPaneBehavior],
        override_posture: OverridePosture,
        recovery_state: EfficiencyRecoveryState,
    ) -> Self {
        Self {
            matrix_ref: M5_EFFICIENCY_GOVERNANCE_MATRIX_REF.to_owned(),
            schema_ref: M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF.to_owned(),
            active_state: snapshot.active_state.clone(),
            source_of_change: snapshot.pressure_sources.clone(),
            hidden_pane_behaviors: hidden_pane_behaviors
                .iter()
                .map(|behavior| behavior.as_str().to_owned())
                .collect(),
            override_posture: override_posture.as_str().to_owned(),
            recovery_state: recovery_state.as_str().to_owned(),
        }
    }
}

/// Returns the canonical closed vocabularies, keyed by the matrix vocabulary
/// name, that the frozen governance matrix must mirror. The shell is the source
/// of truth for these tokens; the matrix is a downstream projection.
pub fn canonical_vocabularies() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        (
            "efficiency_state",
            [
                EfficiencyState::Nominal,
                EfficiencyState::EfficiencyAware,
                EfficiencyState::ThermalConstrained,
                EfficiencyState::ProtectCore,
                EfficiencyState::Recovery,
            ]
            .iter()
            .map(|state| state.as_str())
            .collect(),
        ),
        (
            "source_of_change",
            [
                EfficiencyPressureSource::AcPower,
                EfficiencyPressureSource::Battery,
                EfficiencyPressureSource::OsBatterySaver,
                EfficiencyPressureSource::UserLowPowerMode,
                EfficiencyPressureSource::LowBattery,
                EfficiencyPressureSource::CriticalBattery,
                EfficiencyPressureSource::ThermalPressure,
                EfficiencyPressureSource::FrameMissPressure,
                EfficiencyPressureSource::PolicyCap,
                EfficiencyPressureSource::PressureCleared,
            ]
            .iter()
            .map(|source| source.as_str())
            .collect(),
        ),
        (
            "throttled_subsystem",
            [
                WorkloadFamily::AiWarmup,
                WorkloadFamily::SpeculativePrefetch,
                WorkloadFamily::UploadTransfer,
                WorkloadFamily::NonEssentialAnimation,
                WorkloadFamily::IndexingRefresh,
                WorkloadFamily::ExtensionPolling,
                WorkloadFamily::PreviewRefresh,
                WorkloadFamily::GraphEnrichment,
                WorkloadFamily::RemoteSessionHelper,
            ]
            .iter()
            .map(|workload| workload.as_str())
            .collect(),
        ),
        (
            "visibility_state",
            [
                VisibilityState::VisibleFocused,
                VisibilityState::VisibleBackground,
                VisibilityState::OccludedWindow,
                VisibilityState::HiddenTab,
                VisibilityState::CollapsedSplit,
                VisibilityState::DetachedOffscreen,
            ]
            .iter()
            .map(|visibility| visibility.as_str())
            .collect(),
        ),
        (
            "hidden_pane_behavior",
            HiddenPaneBehavior::ALL
                .iter()
                .map(|behavior| behavior.as_str())
                .collect(),
        ),
        (
            "override_posture",
            OverridePosture::ALL
                .iter()
                .map(|posture| posture.as_str())
                .collect(),
        ),
        (
            "recovery_state",
            EfficiencyRecoveryState::ALL
                .iter()
                .map(|state| state.as_str())
                .collect(),
        ),
    ]
}

#[cfg(test)]
mod tests;
