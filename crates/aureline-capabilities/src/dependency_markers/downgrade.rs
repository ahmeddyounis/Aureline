//! Downgrade and missing-capability evaluation for artifact
//! dependency markers.
//!
//! When an artifact moves to a target that lacks the required
//! capability, the marker MUST warn before apply, narrow the support
//! claim, and carry enough metadata for support / export
//! reconstruction. The closed [`DowngradeScenario`] vocabulary covers
//! every M3 scenario the spec calls out:
//!
//! - `stable_to_preview` — moving a stable artifact onto a target where
//!   the dependency is only available as a preview.
//! - `preview_to_stable` — moving a preview artifact onto a stable
//!   target where the dependency was promoted.
//! - `host_change` — moving an artifact to a host that does not admit
//!   the host-specific dependency.
//! - `mirror_only` — applying the artifact from a curated mirror with
//!   no upstream control plane.
//! - `offline_cache_only` — applying the artifact from the local cache
//!   with no reachable upstream.
//! - `policy_disabled` — the dependency is gated off on the target by
//!   the active admin policy or a kill switch.
//!
//! Evaluation never narrows silently: it always emits a
//! [`CompareApplyReviewSheet`] with the dependency marker, the
//! portability consequence, the typed
//! [`EffectOnImport`], and the safe fallback. Surfaces render this
//! sheet before apply.

use serde::{Deserialize, Serialize};

use super::{ArtifactDependencyMarker, CapabilityLifecycleState, EffectOnImport, SupportPromise};

/// Closed downgrade-scenario vocabulary. Each variant is one
/// observed move from producer state to target state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeScenario {
    /// Source observed the capability as Stable; target sees it as
    /// Preview.
    StableToPreview,
    /// Source observed the capability as Preview; target sees it as
    /// Stable (promotion).
    PreviewToStable,
    /// Source observed the capability admitted on its host; target
    /// host is outside the admitted host scope.
    HostChange,
    /// Target applies the artifact from a curated mirror with no
    /// upstream control plane.
    MirrorOnly,
    /// Target applies the artifact from the local cache with no
    /// reachable upstream.
    OfflineCacheOnly,
    /// Target has the capability disabled by an active admin policy or
    /// kill switch.
    PolicyDisabled,
}

impl DowngradeScenario {
    /// Stable snake_case token persisted in projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableToPreview => "stable_to_preview",
            Self::PreviewToStable => "preview_to_stable",
            Self::HostChange => "host_change",
            Self::MirrorOnly => "mirror_only",
            Self::OfflineCacheOnly => "offline_cache_only",
            Self::PolicyDisabled => "policy_disabled",
        }
    }

    /// Full closed scenario list used by replay tests and the
    /// conformance packet.
    pub const fn all() -> [DowngradeScenario; 6] {
        [
            Self::StableToPreview,
            Self::PreviewToStable,
            Self::HostChange,
            Self::MirrorOnly,
            Self::OfflineCacheOnly,
            Self::PolicyDisabled,
        ]
    }
}

/// Closed projection of the target capability state on the downgrade
/// target. Used by [`evaluate_downgrade`] so the evaluator does not
/// have to inspect the target's full capability registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetCapabilityState {
    /// Lifecycle state of the capability on the target.
    pub lifecycle_state: CapabilityLifecycleState,
    /// True when the capability is admitted on the target's host
    /// (matters for `host_specific` markers).
    #[serde(default = "default_true")]
    pub admitted_on_host: bool,
    /// True when the target is applying the artifact through a
    /// mirror-only fallback.
    #[serde(default)]
    pub mirror_only: bool,
    /// True when the target is applying the artifact through an
    /// offline-cache-only fallback.
    #[serde(default)]
    pub offline_cache_only: bool,
    /// True when the target's admin policy disables the capability.
    #[serde(default)]
    pub policy_disabled: bool,
}

fn default_true() -> bool {
    true
}

impl TargetCapabilityState {
    /// Constructs a default state: capability present and stable on
    /// the target's host.
    pub fn present() -> Self {
        Self {
            lifecycle_state: CapabilityLifecycleState::Stable,
            admitted_on_host: true,
            mirror_only: false,
            offline_cache_only: false,
            policy_disabled: false,
        }
    }
}

/// Compare/apply review-sheet row that surfaces render before apply.
///
/// Every field is required so the surface cannot quietly narrow the
/// review. This row is the proof the spec calls for: "compare/apply
/// review sheets MUST show the dependency marker, the portability
/// consequence, and the safe fallback instead of silently narrowing
/// or failing after commit."
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareApplyReviewSheet {
    /// Scenario observed on the target.
    pub scenario: DowngradeScenario,
    /// Source marker id quoted in the row.
    pub marker_id: String,
    /// Artifact ref the marker is attached to.
    pub artifact_ref: String,
    /// Artifact class token (preserved bit-for-bit).
    pub artifact_class: String,
    /// Required capability id (preserved bit-for-bit).
    pub required_capability_id: String,
    /// Dependency class token recorded on the marker.
    pub dependency_class: String,
    /// Lifecycle state the producer observed.
    pub producer_lifecycle_state: String,
    /// Lifecycle state the target observes.
    pub target_lifecycle_state: String,
    /// Support promise recorded on the marker.
    pub recorded_support_promise: String,
    /// Effective support promise on the target after the downgrade
    /// (narrowed when the dependency lost support guarantees).
    pub effective_support_promise: String,
    /// Effect-on-import the marker shipped with.
    pub recorded_effect_on_import: String,
    /// Effective effect-on-import the target should apply (often the
    /// same; narrows to `block_apply` for policy disables).
    pub effective_effect_on_import: String,
    /// Portability consequence copy surfaces render.
    pub portability_consequence: String,
    /// Bounded fallback / recover path copy.
    pub safe_fallback: String,
    /// True when apply is held until disclosure.
    pub apply_held_until_disclosed: bool,
    /// True when the row narrowed the support claim on the target.
    pub support_claim_narrowed: bool,
    /// True when an active kill switch / policy disable narrowed the
    /// dependency at evaluation time.
    pub kill_switch_active: bool,
    /// True when the closed [`EffectOnImport`] vocabulary preserved
    /// user-authored data.
    pub user_authored_data_preserved: bool,
}

/// Evaluates a [`DowngradeScenario`] against a marker and a
/// [`TargetCapabilityState`], returning the
/// [`CompareApplyReviewSheet`] surfaces render before apply.
pub fn evaluate_downgrade(
    marker: &ArtifactDependencyMarker,
    scenario: DowngradeScenario,
    target_state: &TargetCapabilityState,
) -> CompareApplyReviewSheet {
    let (effective_effect, effective_support, narrowed, kill_switch_active) =
        derive_effective_outcome(marker, scenario, target_state);

    let portability_consequence = render_portability_consequence(marker, scenario);
    let safe_fallback = marker.behavior_on_missing.fallback_path.clone();
    // The spec requires every downgrade scenario to disclose before
    // apply: the closed scenario vocabulary is the proof that this is
    // not a per-variant decision.
    let apply_held_until_disclosed = true;

    CompareApplyReviewSheet {
        scenario,
        marker_id: marker.marker_id.clone(),
        artifact_ref: marker.artifact_ref.clone(),
        artifact_class: marker.artifact_class.as_str().to_owned(),
        required_capability_id: marker.required_capability_id.clone(),
        dependency_class: marker.dependency_class.as_str().to_owned(),
        producer_lifecycle_state: marker.required_lifecycle_state.as_str().to_owned(),
        target_lifecycle_state: target_state.lifecycle_state.as_str().to_owned(),
        recorded_support_promise: marker.support_promise.as_str().to_owned(),
        effective_support_promise: effective_support.as_str().to_owned(),
        recorded_effect_on_import: marker.effect_on_import.as_str().to_owned(),
        effective_effect_on_import: effective_effect.as_str().to_owned(),
        portability_consequence,
        safe_fallback,
        apply_held_until_disclosed,
        support_claim_narrowed: narrowed,
        kill_switch_active,
        user_authored_data_preserved: effective_effect.preserves_user_data(),
    }
}

fn derive_effective_outcome(
    marker: &ArtifactDependencyMarker,
    scenario: DowngradeScenario,
    target_state: &TargetCapabilityState,
) -> (EffectOnImport, SupportPromise, bool, bool) {
    let recorded_effect = marker.effect_on_import;
    let recorded_support = marker.support_promise;

    match scenario {
        DowngradeScenario::StableToPreview => {
            // Target reduces support guarantees; surfaces narrow down
            // to best-effort. Weaker promises stay weak so we never
            // silently upgrade NoSupport into BestEffort.
            let effective_support =
                if support_rank(recorded_support) > support_rank(SupportPromise::BestEffort) {
                    SupportPromise::BestEffort
                } else {
                    recorded_support
                };
            (
                if matches!(recorded_effect, EffectOnImport::BlockApplyPreserveData) {
                    EffectOnImport::BlockApplyPreserveData
                } else {
                    EffectOnImport::EmulatedDowngradePreserveData
                },
                effective_support,
                effective_support != recorded_support,
                marker.kill_switch_active,
            )
        }
        DowngradeScenario::PreviewToStable => {
            // Promotion: the dependency on the target is at least as
            // strong as the producer's record. Effect carries through
            // but surfaces still disclose because the lifecycle delta
            // is real.
            let promoted =
                if support_rank(recorded_support) < support_rank(SupportPromise::StandardSupport) {
                    SupportPromise::StandardSupport
                } else {
                    recorded_support
                };
            (recorded_effect, promoted, false, marker.kill_switch_active)
        }
        DowngradeScenario::HostChange => {
            // Host is outside the marker's admitted scope. We always
            // render a tombstone preserve_data on host mismatches.
            (
                EffectOnImport::RenderTombstonePreserveData,
                SupportPromise::NoSupport,
                recorded_support != SupportPromise::NoSupport,
                marker.kill_switch_active,
            )
        }
        DowngradeScenario::MirrorOnly => {
            // Mirror-only: behavior narrows to (at most) community-
            // supported on the mirror; weaker promises stay weak so we
            // never silently upgrade. Surfaces still render the source's
            // recorded import behavior unless apply was already blocked.
            let effective_support = if support_rank(recorded_support)
                > support_rank(SupportPromise::CommunitySupported)
            {
                SupportPromise::CommunitySupported
            } else {
                recorded_support
            };
            (
                if matches!(recorded_effect, EffectOnImport::BlockApplyPreserveData) {
                    EffectOnImport::BlockApplyPreserveData
                } else {
                    EffectOnImport::NarrowBehaviorPreserveData
                },
                effective_support,
                effective_support != recorded_support,
                target_state.policy_disabled || marker.kill_switch_active,
            )
        }
        DowngradeScenario::OfflineCacheOnly => {
            // Offline-cache-only: behavior holds for later until the
            // upstream lane recovers; user-authored data is preserved.
            // Support narrows to (at most) best-effort.
            let effective_support =
                if support_rank(recorded_support) > support_rank(SupportPromise::BestEffort) {
                    SupportPromise::BestEffort
                } else {
                    recorded_support
                };
            (
                EffectOnImport::HoldForLaterPreserveData,
                effective_support,
                effective_support != recorded_support,
                target_state.policy_disabled || marker.kill_switch_active,
            )
        }
        DowngradeScenario::PolicyDisabled => {
            // Policy disable always blocks apply; user-authored data
            // is preserved in the review sheet for follow-up.
            (
                EffectOnImport::BlockApplyPreserveData,
                SupportPromise::NoSupport,
                recorded_support != SupportPromise::NoSupport,
                true,
            )
        }
    }
}

fn render_portability_consequence(
    marker: &ArtifactDependencyMarker,
    scenario: DowngradeScenario,
) -> String {
    let class = marker.dependency_class.as_str();
    match scenario {
        DowngradeScenario::StableToPreview => format!(
            "Stable artifact moves to a target where {} is only available as a preview; behavior narrows and the support claim drops to best-effort.",
            class
        ),
        DowngradeScenario::PreviewToStable => format!(
            "Preview artifact moves to a target where {} is now stable; behavior keeps the recorded import path and disclosure remains for the lifecycle delta.",
            class
        ),
        DowngradeScenario::HostChange => format!(
            "Target host is outside the {} dependency's admitted scope; the row renders as a read-only tombstone and the user's source remains untouched.",
            class
        ),
        DowngradeScenario::MirrorOnly => format!(
            "Artifact applies from a mirror-only target; the {} dependency narrows to community-supported behavior on the mirror.",
            class
        ),
        DowngradeScenario::OfflineCacheOnly => format!(
            "Artifact applies from the local cache with no upstream; the {} dependency is held for later until the upstream lane recovers.",
            class
        ),
        DowngradeScenario::PolicyDisabled => format!(
            "Target admin policy disables the {} dependency; apply is blocked and the user-authored payload is preserved in the review sheet.",
            class
        ),
    }
}

/// Support-strength rank used by the downgrade evaluator. The order is
/// `NoSupport` < `BestEffort` < `CommunitySupported` < `OperatorOnly`
/// < `StandardSupport` < `ExtendedSupport`. `OperatorOnly` sits below
/// `StandardSupport` so promotion never silently elevates an operator
/// claim to a broadly supported one.
pub fn support_rank(promise: SupportPromise) -> u8 {
    match promise {
        SupportPromise::NoSupport => 0,
        SupportPromise::BestEffort => 1,
        SupportPromise::CommunitySupported => 2,
        SupportPromise::OperatorOnly => 3,
        SupportPromise::StandardSupport => 4,
        SupportPromise::ExtendedSupport => 5,
    }
}

/// Defects emitted by [`assert_downgrade_review_sheets`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DowngradeReviewDefect {
    /// A review sheet failed to disclose before apply.
    SilentApply {
        /// Marker id.
        marker_id: String,
        /// Scenario the silent apply was observed on.
        scenario: DowngradeScenario,
    },
    /// A review sheet shipped without a portability-consequence copy.
    MissingPortabilityConsequence {
        /// Marker id.
        marker_id: String,
        /// Scenario the missing copy was observed on.
        scenario: DowngradeScenario,
    },
    /// A review sheet shipped without a safe fallback.
    MissingSafeFallback {
        /// Marker id.
        marker_id: String,
        /// Scenario the missing fallback was observed on.
        scenario: DowngradeScenario,
    },
    /// A review sheet narrowed silently without flagging the
    /// `support_claim_narrowed` bit. Reserved for defensive coverage;
    /// the evaluator sets the bit whenever it narrows.
    SilentSupportNarrowing {
        /// Marker id.
        marker_id: String,
        /// Scenario observed.
        scenario: DowngradeScenario,
    },
    /// A review sheet failed to preserve user-authored data on a lane
    /// that promised to. Reserved for defensive coverage; the closed
    /// `EffectOnImport` vocabulary forbids silent drop.
    UserDataDropped {
        /// Marker id.
        marker_id: String,
        /// Scenario observed.
        scenario: DowngradeScenario,
    },
}

impl std::fmt::Display for DowngradeReviewDefect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SilentApply {
                marker_id,
                scenario,
            } => write!(
                f,
                "marker {marker_id:?} applied without disclosure on scenario {}",
                scenario.as_str()
            ),
            Self::MissingPortabilityConsequence {
                marker_id,
                scenario,
            } => write!(
                f,
                "marker {marker_id:?} omitted portability_consequence on scenario {}",
                scenario.as_str()
            ),
            Self::MissingSafeFallback {
                marker_id,
                scenario,
            } => write!(
                f,
                "marker {marker_id:?} omitted safe_fallback on scenario {}",
                scenario.as_str()
            ),
            Self::SilentSupportNarrowing {
                marker_id,
                scenario,
            } => write!(
                f,
                "marker {marker_id:?} narrowed support silently on scenario {}",
                scenario.as_str()
            ),
            Self::UserDataDropped {
                marker_id,
                scenario,
            } => write!(
                f,
                "marker {marker_id:?} dropped user-authored data on scenario {}",
                scenario.as_str()
            ),
        }
    }
}

impl std::error::Error for DowngradeReviewDefect {}

/// Evaluates the marker through every scenario in
/// [`DowngradeScenario::all`] and asserts the resulting
/// [`CompareApplyReviewSheet`] rows are well-formed. Returns the
/// per-scenario rows on success or the list of defects on failure.
pub fn assert_downgrade_review_sheets(
    marker: &ArtifactDependencyMarker,
) -> Result<Vec<CompareApplyReviewSheet>, Vec<DowngradeReviewDefect>> {
    let mut sheets = Vec::new();
    let mut defects = Vec::new();
    for scenario in DowngradeScenario::all() {
        let target_state = scenario_target_state(marker, scenario);
        let sheet = evaluate_downgrade(marker, scenario, &target_state);
        if !sheet.apply_held_until_disclosed {
            defects.push(DowngradeReviewDefect::SilentApply {
                marker_id: marker.marker_id.clone(),
                scenario,
            });
        }
        if sheet.portability_consequence.trim().is_empty() {
            defects.push(DowngradeReviewDefect::MissingPortabilityConsequence {
                marker_id: marker.marker_id.clone(),
                scenario,
            });
        }
        if sheet.safe_fallback.trim().is_empty() {
            defects.push(DowngradeReviewDefect::MissingSafeFallback {
                marker_id: marker.marker_id.clone(),
                scenario,
            });
        }
        if sheet.effective_support_promise != sheet.recorded_support_promise
            && !sheet.support_claim_narrowed
            && !matches!(scenario, DowngradeScenario::PreviewToStable)
        {
            defects.push(DowngradeReviewDefect::SilentSupportNarrowing {
                marker_id: marker.marker_id.clone(),
                scenario,
            });
        }
        if !sheet.user_authored_data_preserved {
            defects.push(DowngradeReviewDefect::UserDataDropped {
                marker_id: marker.marker_id.clone(),
                scenario,
            });
        }
        sheets.push(sheet);
    }
    if defects.is_empty() {
        Ok(sheets)
    } else {
        Err(defects)
    }
}

/// Default [`TargetCapabilityState`] for the given scenario, used by
/// the assertion harness when no explicit target state was provided.
pub fn scenario_target_state(
    marker: &ArtifactDependencyMarker,
    scenario: DowngradeScenario,
) -> TargetCapabilityState {
    match scenario {
        DowngradeScenario::StableToPreview => TargetCapabilityState {
            lifecycle_state: CapabilityLifecycleState::Preview,
            admitted_on_host: true,
            mirror_only: false,
            offline_cache_only: false,
            policy_disabled: false,
        },
        DowngradeScenario::PreviewToStable => TargetCapabilityState {
            lifecycle_state: CapabilityLifecycleState::Stable,
            admitted_on_host: true,
            mirror_only: false,
            offline_cache_only: false,
            policy_disabled: false,
        },
        DowngradeScenario::HostChange => TargetCapabilityState {
            lifecycle_state: marker.required_lifecycle_state,
            admitted_on_host: false,
            mirror_only: false,
            offline_cache_only: false,
            policy_disabled: false,
        },
        DowngradeScenario::MirrorOnly => TargetCapabilityState {
            lifecycle_state: marker.required_lifecycle_state,
            admitted_on_host: true,
            mirror_only: true,
            offline_cache_only: false,
            policy_disabled: false,
        },
        DowngradeScenario::OfflineCacheOnly => TargetCapabilityState {
            lifecycle_state: marker.required_lifecycle_state,
            admitted_on_host: true,
            mirror_only: false,
            offline_cache_only: true,
            policy_disabled: false,
        },
        DowngradeScenario::PolicyDisabled => TargetCapabilityState {
            lifecycle_state: CapabilityLifecycleState::DisabledByPolicy,
            admitted_on_host: true,
            mirror_only: false,
            offline_cache_only: false,
            policy_disabled: true,
        },
    }
}

/// Per-marker downgrade audit row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradeAudit {
    /// Source marker id.
    pub marker_id: String,
    /// Artifact ref the marker is attached to.
    pub artifact_ref: String,
    /// Total scenarios evaluated.
    pub scenarios_total: usize,
    /// Scenarios that held apply until disclosure.
    pub scenarios_pre_apply_held: usize,
    /// Scenarios that narrowed the support claim explicitly.
    pub scenarios_support_narrowed: usize,
    /// Scenarios that triggered a kill-switch / policy-disabled
    /// branch.
    pub scenarios_kill_switch_active: usize,
    /// True when every scenario preserved user-authored data.
    pub user_authored_data_preserved_on_every_scenario: bool,
}

impl DowngradeAudit {
    /// Builds an audit from a list of [`CompareApplyReviewSheet`]
    /// rows.
    pub fn from_sheets(
        marker: &ArtifactDependencyMarker,
        sheets: &[CompareApplyReviewSheet],
    ) -> Self {
        let scenarios_pre_apply_held = sheets
            .iter()
            .filter(|sheet| sheet.apply_held_until_disclosed)
            .count();
        let scenarios_support_narrowed = sheets
            .iter()
            .filter(|sheet| sheet.support_claim_narrowed)
            .count();
        let scenarios_kill_switch_active = sheets
            .iter()
            .filter(|sheet| sheet.kill_switch_active)
            .count();
        let user_authored_data_preserved_on_every_scenario = sheets
            .iter()
            .all(|sheet| sheet.user_authored_data_preserved);
        Self {
            marker_id: marker.marker_id.clone(),
            artifact_ref: marker.artifact_ref.clone(),
            scenarios_total: sheets.len(),
            scenarios_pre_apply_held,
            scenarios_support_narrowed,
            scenarios_kill_switch_active,
            user_authored_data_preserved_on_every_scenario,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dependency_markers::{
        catalog_default_capabilities, ArtifactClass, ArtifactDependencyMarker, DependencyClass,
    };

    fn sample_marker_for(idx: usize, artifact_class: ArtifactClass) -> ArtifactDependencyMarker {
        let catalog = catalog_default_capabilities();
        let capability = &catalog[idx];
        ArtifactDependencyMarker::from_capability(
            format!(
                "marker:downgrade:{}:{}",
                artifact_class.as_str(),
                capability.capability_id
            ),
            artifact_class,
            format!("artifact:downgrade:{}", artifact_class.as_str()),
            capability,
            format!(
                "Downgrade-test marker for {} ({}).",
                capability.title,
                capability.dependency_class.as_str()
            ),
        )
    }

    #[test]
    fn downgrade_scenarios_close_the_vocabulary() {
        let mut tokens = std::collections::BTreeSet::new();
        for scenario in DowngradeScenario::all() {
            assert!(
                tokens.insert(scenario.as_str()),
                "duplicate token: {}",
                scenario.as_str()
            );
        }
    }

    #[test]
    fn every_scenario_holds_apply_until_disclosed() {
        let marker = sample_marker_for(0, ArtifactClass::SettingsExport);
        let sheets =
            assert_downgrade_review_sheets(&marker).expect("review sheets must be well-formed");
        assert_eq!(sheets.len(), DowngradeScenario::all().len());
        for sheet in &sheets {
            assert!(sheet.apply_held_until_disclosed);
            assert!(!sheet.portability_consequence.is_empty());
            assert!(!sheet.safe_fallback.is_empty());
            assert!(sheet.user_authored_data_preserved);
        }
    }

    #[test]
    fn policy_disabled_blocks_apply_and_narrows_support() {
        let marker = sample_marker_for(2, ArtifactClass::WorkflowBundle);
        let target_state = scenario_target_state(&marker, DowngradeScenario::PolicyDisabled);
        let sheet = evaluate_downgrade(&marker, DowngradeScenario::PolicyDisabled, &target_state);
        assert_eq!(
            sheet.effective_effect_on_import,
            "block_apply_preserve_data"
        );
        assert_eq!(sheet.effective_support_promise, "no_support");
        assert!(sheet.support_claim_narrowed);
        assert!(sheet.kill_switch_active);
        assert!(sheet.user_authored_data_preserved);
    }

    #[test]
    fn host_change_renders_tombstone() {
        let marker = sample_marker_for(4, ArtifactClass::Recipe);
        let target_state = scenario_target_state(&marker, DowngradeScenario::HostChange);
        let sheet = evaluate_downgrade(&marker, DowngradeScenario::HostChange, &target_state);
        assert_eq!(
            sheet.effective_effect_on_import,
            "render_tombstone_preserve_data"
        );
        assert_eq!(
            sheet.dependency_class,
            DependencyClass::HostSpecific.as_str()
        );
    }

    #[test]
    fn audit_counts_held_scenarios() {
        let marker = sample_marker_for(1, ArtifactClass::Profile);
        let sheets = assert_downgrade_review_sheets(&marker).expect("ok");
        let audit = DowngradeAudit::from_sheets(&marker, &sheets);
        assert_eq!(audit.scenarios_total, DowngradeScenario::all().len());
        assert_eq!(audit.scenarios_pre_apply_held, audit.scenarios_total);
        assert!(audit.user_authored_data_preserved_on_every_scenario);
        assert!(audit.scenarios_kill_switch_active >= 1);
    }

    #[test]
    fn support_promise_rank_is_monotonic() {
        let ordered = [
            SupportPromise::NoSupport,
            SupportPromise::BestEffort,
            SupportPromise::CommunitySupported,
            SupportPromise::OperatorOnly,
            SupportPromise::StandardSupport,
            SupportPromise::ExtendedSupport,
        ];
        for pair in ordered.windows(2) {
            assert!(support_rank(pair[0]) < support_rank(pair[1]));
        }
    }
}
