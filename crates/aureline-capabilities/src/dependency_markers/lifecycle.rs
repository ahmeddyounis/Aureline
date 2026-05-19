//! Capability lifecycle vocabulary.
//!
//! The vocabulary mirrors
//! `schemas/governance/capability_lifecycle.schema.json` so artifact
//! markers carry the same closed token set the governance-owned
//! lifecycle registry already publishes. This module exists so the
//! capabilities crate does not pull a dependency on the experiments
//! inventory crate just to share the vocabulary.

use serde::{Deserialize, Serialize};

/// Closed lifecycle-state vocabulary persisted in capability records
/// and artifact dependency markers.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityLifecycleState {
    /// Opt-in exploratory surface.
    Labs,
    /// Named cohort or preview surface.
    Preview,
    /// Broader rollout with support intent.
    Beta,
    /// Generally enabled and supported.
    Stable,
    /// Long-term-support facing.
    LtsFacing,
    /// Still visible with replacement guidance.
    Deprecated,
    /// Present but unavailable because policy or a kill switch blocks
    /// it.
    DisabledByPolicy,
    /// No longer runnable; retained as tombstone or migration truth.
    Retired,
}

impl CapabilityLifecycleState {
    /// Returns the stable snake_case token rendered by the marker.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Labs => "labs",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
            Self::LtsFacing => "lts_facing",
            Self::Deprecated => "deprecated",
            Self::DisabledByPolicy => "disabled_by_policy",
            Self::Retired => "retired",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_tokens_match_governance_vocabulary() {
        for (state, token) in [
            (CapabilityLifecycleState::Labs, "labs"),
            (CapabilityLifecycleState::Preview, "preview"),
            (CapabilityLifecycleState::Beta, "beta"),
            (CapabilityLifecycleState::Stable, "stable"),
            (CapabilityLifecycleState::LtsFacing, "lts_facing"),
            (CapabilityLifecycleState::Deprecated, "deprecated"),
            (CapabilityLifecycleState::DisabledByPolicy, "disabled_by_policy"),
            (CapabilityLifecycleState::Retired, "retired"),
        ] {
            assert_eq!(state.as_str(), token);
        }
    }
}
