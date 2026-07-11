//! Implemented M5 adapter-confidence-chip and discovery-diff-card primitives.
//!
//! The frozen [build/remote-boundary component matrix][matrix] names the reusable build / remote /
//! managed-workspace boundary UI components and locks their controlled vocabulary. This module is
//! the first implement lane over that matrix: it turns the two build-intelligence confidence
//! components — the **adapter-confidence chip** and the **discovery-diff card** — into resolvers
//! that produce export-safe, honest projections instead of feature-local confidence copy.
//!
//! Three acceptance criteria drive the resolvers:
//!
//! * **AC1 — users can see when a target is exact, compatible, heuristic, imported, downgraded, or
//!   stale before invoking the action.** [`resolve_adapter_confidence_chip`] refuses to read as a
//!   clean chip unless it names its adapter / source class, its confidence band, and its
//!   heuristic-vs-structured-vs-imported discovery mode, and attributes a current downgrade reason
//!   whenever the resolved certainty is genuinely reduced (downgraded or stale). A clean chip always
//!   carries one of the six [`M5AdapterDiscoveryCertainty`] states so a user can read the target's
//!   certainty before they run, debug, preview, or hand off work.
//! * **AC2 — material discovery drift produces an attributable review state instead of a silent
//!   relabel.** [`resolve_discovery_diff_card`] degrades to
//!   [`M5DiscoveryDiffCardDegradeReason::SilentRelabelWithoutReview`] the moment a material change is
//!   presented without an attributable review state, and never lets a card read as a clean relabel.
//! * **No-higher-confidence-overwrite** — [`resolve_discovery_diff_card`] degrades to
//!   [`M5DiscoveryDiffCardDegradeReason::LowerConfidenceOverwroteResolved`] whenever a weaker
//!   discovery result would replace a stronger resolved target without an explicit review state, so
//!   stale or lower-confidence discovery can never silently overwrite a higher-confidence target.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5BuildRemoteBoundaryDisposition`] boundary-disposition vocabulary, the frozen
//! [`M5BuildRemoteDowngradeTrigger`] downgrade-trigger vocabulary — and bind the adapter / source
//! class, confidence band, discovery mode, and current downgrade reason directly to the frozen M5
//! execution object models ([`TargetDiscoveryClass`], [`AdapterConfidence`], [`DiscoveryConfidence`],
//! and [`NarrowingReason`]), so this lane can never fork its own confidence, discovery, or
//! downgrade wording.
//!
//! [matrix]: crate::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_adapter_discovery_controls,
    seeded_m5_adapter_discovery_controls_preview_row_preview_narrowed,
    seeded_m5_adapter_discovery_controls_run_test_debug_beta_narrowed,
    M5_ADAPTER_DISCOVERY_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_execution::m5_build_and_host_governance::{
    AdapterConfidence, NarrowingReason, TargetDiscoveryClass, M5_BUILD_AND_HOST_GOVERNANCE_PATH,
};
use aureline_execution::m5_target_discovery::{DiscoveryConfidence, M5_TARGET_DISCOVERY_PATH};

use crate::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix::{
    M5BuildRemoteAccessibilityRoute, M5BuildRemoteBoundaryDisposition, M5BuildRemoteConsumerSurface,
    M5BuildRemoteDeploymentLine, M5BuildRemoteDowngradeTrigger, M5BuildRemoteQualificationClass,
    M5BuildRemoteRequiredLabel, M5_ADAPTER_CONFIDENCE_CHIP_SCHEMA_REF,
    M5_BUILD_REMOTE_BOUNDARY_COMPONENT_DOC_REF, M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
    M5_DISCOVERY_DIFF_CARD_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5AdapterDiscoveryControlsPacket`].
pub const M5_ADAPTER_DISCOVERY_CONTROLS_RECORD_KIND: &str =
    "implement_m5_adapter_confidence_chip_and_discovery_diff_card_controls";

/// Schema version for M5 adapter-confidence-chip / discovery-diff-card controls records.
pub const M5_ADAPTER_DISCOVERY_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls boundary schema.
pub const M5_ADAPTER_DISCOVERY_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-adapter-confidence-chip-discovery-diff-card-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_ADAPTER_DISCOVERY_CONTROLS_DOC_REF: &str =
    "docs/remote/m5_adapter_confidence_chip_and_discovery_diff_card_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_ADAPTER_DISCOVERY_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-adapter-confidence-chip-discovery-diff-card-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_ADAPTER_DISCOVERY_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-adapter-confidence-chip-discovery-diff-card-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_ADAPTER_DISCOVERY_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-adapter-confidence-chip-discovery-diff-card-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_ADAPTER_DISCOVERY_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-adapter-confidence-chip-discovery-diff-card-controls";

/// Consumer surface an adapter-discovery controls row projects onto. Reuses the frozen matrix
/// consumer-surface taxonomy so no lane invents a parallel surface set.
pub type M5AdapterDiscoveryConsumerSurface = M5BuildRemoteConsumerSurface;

/// The single controlled certainty a resolved adapter-confidence chip or discovery-diff card
/// carries. These are the exact states the spec requires a user to be able to read before they run,
/// debug, preview, or hand off work: exact, compatible, heuristic, imported, downgraded, or stale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdapterDiscoveryCertainty {
    /// Confirmed, verified target identity.
    Exact,
    /// Compatible, structured-signal target; strong but not exact.
    Compatible,
    /// Heuristic guess only.
    Heuristic,
    /// Reconstructed from a structured import.
    Imported,
    /// The target was downgraded below its earlier certainty.
    Downgraded,
    /// The resolved target is stale beyond its freshness window.
    Stale,
}

impl M5AdapterDiscoveryCertainty {
    /// Every certainty state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Exact,
        Self::Compatible,
        Self::Heuristic,
        Self::Imported,
        Self::Downgraded,
        Self::Stale,
    ];

    /// The five reduced certainties a user must be able to tell apart from an exact target.
    pub const REDUCED: [Self; 5] = [
        Self::Compatible,
        Self::Heuristic,
        Self::Imported,
        Self::Downgraded,
        Self::Stale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Compatible => "compatible",
            Self::Heuristic => "heuristic",
            Self::Imported => "imported",
            Self::Downgraded => "downgraded",
            Self::Stale => "stale",
        }
    }

    /// Whether this is the one clean, exact certainty.
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Whether a chip in this certainty must attribute a current downgrade reason. A downgraded or
    /// stale target is a genuine reduction that must never be presented without an attributed
    /// reason; compatible, heuristic, and imported are honest discovery modes that stand on their
    /// own.
    pub const fn requires_attributed_reason(self) -> bool {
        matches!(self, Self::Downgraded | Self::Stale)
    }
}

/// One mandatory rendered part an adapter-confidence chip or discovery-diff card must be able to
/// show, so no confidence or discovery truth is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdapterDiscoveryAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed certainty.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The adapter / source class behind the resolved target (chip).
    AdapterSourceClass,
    /// The adapter confidence band (chip).
    ConfidenceBand,
    /// The heuristic-vs-structured-vs-imported discovery mode (chip).
    DiscoveryMode,
    /// The current downgrade reason (chip).
    DowngradeReason,
    /// The previous target identity (card).
    PreviousTarget,
    /// The current target identity (card).
    CurrentTarget,
    /// The changed certainty relative to the prior target (card).
    ChangedCertainty,
    /// The review-before-switch affordance (card).
    ReviewAffordance,
}

impl M5AdapterDiscoveryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::AdapterSourceClass,
        Self::ConfidenceBand,
        Self::DiscoveryMode,
        Self::DowngradeReason,
        Self::PreviousTarget,
        Self::CurrentTarget,
        Self::ChangedCertainty,
        Self::ReviewAffordance,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::AdapterSourceClass => "adapter_source_class",
            Self::ConfidenceBand => "confidence_band",
            Self::DiscoveryMode => "discovery_mode",
            Self::DowngradeReason => "downgrade_reason",
            Self::PreviousTarget => "previous_target",
            Self::CurrentTarget => "current_target",
            Self::ChangedCertainty => "changed_certainty",
            Self::ReviewAffordance => "review_affordance",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdapterDiscoveryNextAction {
    /// Review the discovery drift before switching targets.
    ReviewDiscoveryDrift,
    /// View the basis for the adapter confidence and discovery mode.
    ViewConfidenceBasis,
    /// Keep the current, higher-confidence resolved target.
    KeepResolvedTarget,
    /// Switch to the new target after an explicit review.
    SwitchAfterReview,
    /// Review diagnostics for the unavailable signal.
    ReviewDiagnostics,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5AdapterDiscoveryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewDiscoveryDrift,
        Self::ViewConfidenceBasis,
        Self::KeepResolvedTarget,
        Self::SwitchAfterReview,
        Self::ReviewDiagnostics,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewDiscoveryDrift => "review_discovery_drift",
            Self::ViewConfidenceBasis => "view_confidence_basis",
            Self::KeepResolvedTarget => "keep_resolved_target",
            Self::SwitchAfterReview => "switch_after_review",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field an adapter-discovery controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdapterDiscoveryExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The certainties carried.
    Certainties,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The adapter / source class named by the chip.
    AdapterSourceClass,
    /// The confidence band named by the chip.
    ConfidenceBand,
    /// The discovery mode named by the chip.
    DiscoveryMode,
    /// The previous target identity named by the card.
    PreviousTarget,
    /// The current target identity named by the card.
    CurrentTarget,
    /// The accountable owner role.
    OwnerRole,
}

impl M5AdapterDiscoveryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Certainties,
        Self::DegradeReasons,
        Self::Qualification,
        Self::AdapterSourceClass,
        Self::ConfidenceBand,
        Self::DiscoveryMode,
        Self::PreviousTarget,
        Self::CurrentTarget,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Certainties,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::Certainties => "certainties",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::AdapterSourceClass => "adapter_source_class",
            Self::ConfidenceBand => "confidence_band",
            Self::DiscoveryMode => "discovery_mode",
            Self::PreviousTarget => "previous_target",
            Self::CurrentTarget => "current_target",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason an adapter-confidence chip degraded below a clean, fully-legible state. The degrade-first
/// ladder returns one of these instead of ever letting an anonymous or under-labelled chip read as
/// a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdapterConfidenceChipDegradeReason {
    /// The adapter / source class is unstated or undiscovered (AC1 violation).
    SourceClassUnstated,
    /// The confidence band is unstated (AC1 violation).
    ConfidenceBandUnstated,
    /// The heuristic-vs-structured-vs-imported discovery mode is unstated (AC1 violation).
    DiscoveryModeUnstated,
    /// A downgraded or stale target carries no attributed current downgrade reason (AC1 violation).
    DowngradeReasonUnattributed,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5AdapterConfidenceChipDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SourceClassUnstated,
        Self::ConfidenceBandUnstated,
        Self::DiscoveryModeUnstated,
        Self::DowngradeReasonUnattributed,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceClassUnstated => "source_class_unstated",
            Self::ConfidenceBandUnstated => "confidence_band_unstated",
            Self::DiscoveryModeUnstated => "discovery_mode_unstated",
            Self::DowngradeReasonUnattributed => "downgrade_reason_unattributed",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5AdapterDiscoveryNextAction {
        match self {
            Self::SourceClassUnstated | Self::ConfidenceBandUnstated | Self::ProofStale => {
                M5AdapterDiscoveryNextAction::ReviewDiagnostics
            }
            Self::DiscoveryModeUnstated => M5AdapterDiscoveryNextAction::ViewConfidenceBasis,
            Self::DowngradeReasonUnattributed => M5AdapterDiscoveryNextAction::ReviewDiscoveryDrift,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5BuildRemoteDowngradeTrigger {
        match self {
            Self::SourceClassUnstated => M5BuildRemoteDowngradeTrigger::GenericStatusWordingUsed,
            Self::ConfidenceBandUnstated => {
                M5BuildRemoteDowngradeTrigger::AdapterConfidenceUnstated
            }
            Self::DiscoveryModeUnstated => M5BuildRemoteDowngradeTrigger::DiscoveryDriftHidden,
            Self::DowngradeReasonUnattributed => {
                M5BuildRemoteDowngradeTrigger::GenericStatusWordingUsed
            }
            Self::ProofStale => M5BuildRemoteDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a discovery-diff card degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiscoveryDiffCardDegradeReason {
    /// The previous or current target identity is unstated.
    TargetIdentityUnstated,
    /// A material change carries no changed-certainty label.
    ChangedCertaintyUnstated,
    /// A material change is presented without an attributable review state (AC2 violation).
    SilentRelabelWithoutReview,
    /// A weaker discovery result would replace a stronger resolved target without an explicit review
    /// state (no-higher-confidence-overwrite / guardrail violation).
    LowerConfidenceOverwroteResolved,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5DiscoveryDiffCardDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TargetIdentityUnstated,
        Self::ChangedCertaintyUnstated,
        Self::SilentRelabelWithoutReview,
        Self::LowerConfidenceOverwroteResolved,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetIdentityUnstated => "target_identity_unstated",
            Self::ChangedCertaintyUnstated => "changed_certainty_unstated",
            Self::SilentRelabelWithoutReview => "silent_relabel_without_review",
            Self::LowerConfidenceOverwroteResolved => "lower_confidence_overwrote_resolved",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5AdapterDiscoveryNextAction {
        match self {
            Self::TargetIdentityUnstated | Self::ChangedCertaintyUnstated | Self::ProofStale => {
                M5AdapterDiscoveryNextAction::ReviewDiagnostics
            }
            Self::SilentRelabelWithoutReview => M5AdapterDiscoveryNextAction::ReviewDiscoveryDrift,
            Self::LowerConfidenceOverwroteResolved => {
                M5AdapterDiscoveryNextAction::KeepResolvedTarget
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5BuildRemoteDowngradeTrigger {
        match self {
            Self::TargetIdentityUnstated => M5BuildRemoteDowngradeTrigger::GenericStatusWordingUsed,
            Self::ChangedCertaintyUnstated | Self::SilentRelabelWithoutReview => {
                M5BuildRemoteDowngradeTrigger::DiscoveryDriftHidden
            }
            Self::LowerConfidenceOverwroteResolved => {
                M5BuildRemoteDowngradeTrigger::LowerConfidenceOverwroteResolvedTarget
            }
            Self::ProofStale => M5BuildRemoteDowngradeTrigger::ProofStale,
        }
    }
}

/// Maps an adapter confidence band, discovery mode, and staleness to the single controlled
/// certainty a user reads.
fn certainty_for_chip(
    confidence: AdapterConfidence,
    mode: DiscoveryConfidence,
    stale: bool,
) -> M5AdapterDiscoveryCertainty {
    use M5AdapterDiscoveryCertainty as C;
    if stale {
        return C::Stale;
    }
    if confidence.is_low_confidence_trigger() || mode == DiscoveryConfidence::Unresolved {
        return C::Downgraded;
    }
    match mode {
        DiscoveryConfidence::Exact => C::Exact,
        DiscoveryConfidence::Structured => C::Compatible,
        DiscoveryConfidence::Imported => C::Imported,
        DiscoveryConfidence::Heuristic => C::Heuristic,
        DiscoveryConfidence::Unresolved => C::Downgraded,
    }
}

/// Maps a previous / current discovery confidence pair to the changed certainty a card shows.
fn changed_certainty_for_card(
    previous: DiscoveryConfidence,
    current: DiscoveryConfidence,
) -> M5AdapterDiscoveryCertainty {
    use M5AdapterDiscoveryCertainty as C;
    if current.rank() < previous.rank() {
        return C::Downgraded;
    }
    match current {
        DiscoveryConfidence::Exact => C::Exact,
        DiscoveryConfidence::Structured => C::Compatible,
        DiscoveryConfidence::Imported => C::Imported,
        DiscoveryConfidence::Heuristic => C::Heuristic,
        DiscoveryConfidence::Unresolved => C::Downgraded,
    }
}

/// Input to [`resolve_adapter_confidence_chip`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AdapterConfidenceChipResolutionInput {
    /// Stable identity of the chip instance.
    pub chip_id: String,
    /// The adapter / source class behind the resolved target.
    pub adapter_source_class: TargetDiscoveryClass,
    /// True when the adapter / source class is disclosed on the chip, never menu-only.
    pub source_class_disclosed: bool,
    /// The adapter confidence band behind the resolved target.
    pub adapter_confidence: AdapterConfidence,
    /// True when the confidence band is disclosed on the chip.
    pub confidence_band_disclosed: bool,
    /// The heuristic-vs-structured-vs-imported discovery mode behind the resolved target.
    pub discovery_mode: DiscoveryConfidence,
    /// True when the discovery mode is disclosed on the chip.
    pub discovery_mode_disclosed: bool,
    /// True when the resolved target is stale beyond its freshness window.
    pub stale: bool,
    /// The current downgrade reason attributed to the chip, if any.
    pub current_downgrade_reason: Option<NarrowingReason>,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe adapter-confidence chip projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAdapterConfidenceChip {
    /// Stable identity of the chip instance.
    pub chip_id: String,
    /// Adapter / source class token named by the chip.
    pub adapter_source_class: String,
    /// Adapter confidence band token named by the chip.
    pub adapter_confidence: String,
    /// Discovery mode token named by the chip.
    pub discovery_mode: String,
    /// The single controlled certainty the chip carries.
    pub certainty: M5AdapterDiscoveryCertainty,
    /// The highest claim the confidence band permits.
    pub claim_ceiling: String,
    /// Current downgrade reason token attributed by the chip, if any.
    pub current_downgrade_reason: Option<String>,
    /// Degrade reason, if the chip could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5AdapterConfidenceChipDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5AdapterDiscoveryNextAction,
    /// AC1: whether the adapter / source class is disclosed on the chip.
    pub source_class_disclosed: bool,
    /// AC1: whether the confidence band is disclosed on the chip.
    pub confidence_band_disclosed: bool,
    /// AC1: whether the discovery mode is disclosed on the chip.
    pub discovery_mode_disclosed: bool,
    /// Whether the resolved target is stale.
    pub is_stale: bool,
}

impl M5ResolvedAdapterConfidenceChip {
    /// Whether this chip reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }

    /// Whether this chip hides its adapter / source class, confidence band, or discovery mode (an
    /// AC1 violation).
    pub fn hides_confidence_basis(&self) -> bool {
        !self.source_class_disclosed
            || !self.confidence_band_disclosed
            || !self.discovery_mode_disclosed
    }
}

/// Input to [`resolve_discovery_diff_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DiscoveryDiffCardResolutionInput {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// The previous target identity (empty means unstated).
    pub previous_target_identity: String,
    /// The current target identity (empty means unstated).
    pub current_target_identity: String,
    /// The previous target's discovery confidence.
    pub previous_confidence: DiscoveryConfidence,
    /// The current target's discovery confidence.
    pub current_confidence: DiscoveryConfidence,
    /// True when both target identities are disclosed on the card, never menu-only.
    pub target_identity_disclosed: bool,
    /// True when the resolved target changed materially.
    pub material_change: bool,
    /// True when the changed certainty is disclosed on the card.
    pub changed_certainty_disclosed: bool,
    /// True when a review-before-switch affordance is offered on the card.
    pub review_before_switch_available: bool,
    /// True when the material change is attributed to an explicit review state, not a silent
    /// relabel.
    pub attributed_review_state: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe discovery-diff card projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDiscoveryDiffCard {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// Previous target identity named by the card.
    pub previous_target_identity: String,
    /// Current target identity named by the card.
    pub current_target_identity: String,
    /// Previous discovery-confidence token named by the card.
    pub previous_confidence: String,
    /// Current discovery-confidence token named by the card.
    pub current_confidence: String,
    /// The changed certainty relative to the prior target.
    pub changed_certainty: M5AdapterDiscoveryCertainty,
    /// Whether the resolved target changed materially.
    pub material_change: bool,
    /// Whether a review-before-switch affordance is offered.
    pub review_before_switch_available: bool,
    /// Whether the material change is attributed to an explicit review state.
    pub attributed_review_state: bool,
    /// AC: whether both target identities are disclosed on the card.
    pub target_identity_disclosed: bool,
    /// AC: whether the changed certainty is disclosed on the card.
    pub changed_certainty_disclosed: bool,
    /// Degrade reason, if the card could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5DiscoveryDiffCardDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5AdapterDiscoveryNextAction,
    /// Guardrail (MUST be `false` on a clean card): a material change is presented as a silent
    /// relabel.
    pub renders_silent_relabel: bool,
    /// Guardrail (MUST be `false` on a clean card): a weaker discovery result would overwrite a
    /// stronger resolved target without an explicit review state.
    pub overwrites_higher_confidence_without_review: bool,
}

impl M5ResolvedDiscoveryDiffCard {
    /// Whether this card reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }

    /// Whether this card hides its target identities or the changed certainty (an AC violation).
    pub fn hides_target_or_certainty(&self) -> bool {
        !self.target_identity_disclosed
            || (self.material_change && !self.changed_certainty_disclosed)
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5AdapterDiscoveryResolutionError {
    /// The chip id was empty.
    EmptyChipId,
    /// The card id was empty.
    EmptyCardId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5AdapterDiscoveryResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyChipId => "empty_chip_id",
            Self::EmptyCardId => "empty_card_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5AdapterDiscoveryResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 adapter-discovery resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AdapterDiscoveryResolutionError {}

/// Resolves an adapter-confidence chip, proving AC1: a user can read the resolved target's adapter /
/// source class, confidence band, and heuristic-vs-structured-vs-imported discovery mode, and a
/// downgraded or stale target always carries an attributed current downgrade reason before the user
/// runs, debugs, previews, or hands off work.
pub fn resolve_adapter_confidence_chip(
    input: M5AdapterConfidenceChipResolutionInput,
) -> Result<M5ResolvedAdapterConfidenceChip, M5AdapterDiscoveryResolutionError> {
    if input.chip_id.trim().is_empty() {
        return Err(M5AdapterDiscoveryResolutionError::EmptyChipId);
    }
    if string_is_forbidden(&input.chip_id) {
        return Err(M5AdapterDiscoveryResolutionError::ForbiddenMaterial);
    }

    let source_class_disclosed = input.source_class_disclosed
        && input.adapter_source_class != TargetDiscoveryClass::Undiscovered;
    let certainty = certainty_for_chip(input.adapter_confidence, input.discovery_mode, input.stale);

    let degrade_reason = if !source_class_disclosed {
        Some(M5AdapterConfidenceChipDegradeReason::SourceClassUnstated)
    } else if !input.confidence_band_disclosed {
        Some(M5AdapterConfidenceChipDegradeReason::ConfidenceBandUnstated)
    } else if !input.discovery_mode_disclosed {
        Some(M5AdapterConfidenceChipDegradeReason::DiscoveryModeUnstated)
    } else if certainty.requires_attributed_reason() && input.current_downgrade_reason.is_none() {
        Some(M5AdapterConfidenceChipDegradeReason::DowngradeReasonUnattributed)
    } else if !input.proof_fresh {
        Some(M5AdapterConfidenceChipDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5AdapterDiscoveryNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedAdapterConfidenceChip {
        chip_id: input.chip_id,
        adapter_source_class: input.adapter_source_class.as_str().to_owned(),
        adapter_confidence: input.adapter_confidence.as_str().to_owned(),
        discovery_mode: input.discovery_mode.as_str().to_owned(),
        certainty,
        claim_ceiling: input.adapter_confidence.claim_ceiling().as_str().to_owned(),
        current_downgrade_reason: input
            .current_downgrade_reason
            .map(|r| r.as_str().to_owned()),
        degrade_reason,
        next_action,
        source_class_disclosed,
        confidence_band_disclosed: input.confidence_band_disclosed,
        discovery_mode_disclosed: input.discovery_mode_disclosed,
        is_stale: input.stale,
    })
}

/// Resolves a discovery-diff card, proving AC2 (material discovery drift produces an attributable
/// review state instead of a silent relabel) and the no-higher-confidence-overwrite semantics (a
/// weaker discovery result never silently replaces a stronger resolved target without an explicit
/// review state).
pub fn resolve_discovery_diff_card(
    input: M5DiscoveryDiffCardResolutionInput,
) -> Result<M5ResolvedDiscoveryDiffCard, M5AdapterDiscoveryResolutionError> {
    if input.card_id.trim().is_empty() {
        return Err(M5AdapterDiscoveryResolutionError::EmptyCardId);
    }
    if string_is_forbidden(&input.card_id)
        || string_is_forbidden(&input.previous_target_identity)
        || string_is_forbidden(&input.current_target_identity)
    {
        return Err(M5AdapterDiscoveryResolutionError::ForbiddenMaterial);
    }

    let target_identity_disclosed = input.target_identity_disclosed
        && !input.previous_target_identity.trim().is_empty()
        && !input.current_target_identity.trim().is_empty();
    let confidence_dropped = input.current_confidence.rank() < input.previous_confidence.rank();
    let renders_silent_relabel = input.material_change && !input.attributed_review_state;
    let overwrites_higher_confidence_without_review =
        input.material_change && confidence_dropped && !input.review_before_switch_available;
    let changed_certainty =
        changed_certainty_for_card(input.previous_confidence, input.current_confidence);

    let degrade_reason = if !target_identity_disclosed {
        Some(M5DiscoveryDiffCardDegradeReason::TargetIdentityUnstated)
    } else if input.material_change && !input.changed_certainty_disclosed {
        Some(M5DiscoveryDiffCardDegradeReason::ChangedCertaintyUnstated)
    } else if renders_silent_relabel {
        Some(M5DiscoveryDiffCardDegradeReason::SilentRelabelWithoutReview)
    } else if overwrites_higher_confidence_without_review {
        Some(M5DiscoveryDiffCardDegradeReason::LowerConfidenceOverwroteResolved)
    } else if !input.proof_fresh {
        Some(M5DiscoveryDiffCardDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None if input.material_change => M5AdapterDiscoveryNextAction::SwitchAfterReview,
        None => M5AdapterDiscoveryNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedDiscoveryDiffCard {
        card_id: input.card_id,
        previous_target_identity: input.previous_target_identity,
        current_target_identity: input.current_target_identity,
        previous_confidence: input.previous_confidence.as_str().to_owned(),
        current_confidence: input.current_confidence.as_str().to_owned(),
        changed_certainty,
        material_change: input.material_change,
        review_before_switch_available: input.review_before_switch_available,
        attributed_review_state: input.attributed_review_state,
        target_identity_disclosed,
        changed_certainty_disclosed: input.changed_certainty_disclosed,
        degrade_reason,
        next_action,
        renders_silent_relabel,
        overwrites_higher_confidence_without_review,
    })
}

/// One controls row: one consumer surface bound to the resolved adapter-confidence chip and
/// discovery-diff card examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdapterDiscoveryControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5AdapterDiscoveryConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5BuildRemoteQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5BuildRemoteDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5BuildRemoteRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5BuildRemoteAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5AdapterDiscoveryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5AdapterDiscoveryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5BuildRemoteDowngradeTrigger>,
    /// Resolved adapter-confidence chip examples.
    pub adapter_confidence_chip_examples: Vec<M5ResolvedAdapterConfidenceChip>,
    /// Resolved discovery-diff card examples.
    pub discovery_diff_card_examples: Vec<M5ResolvedDiscoveryDiffCard>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never relabel a materially changed target without an attributable review
    /// state.
    pub relabels_target_without_attributable_review: bool,
    /// Hard invariant: never let lower-confidence discovery overwrite a higher-confidence resolved
    /// target without review.
    pub lower_confidence_overwrites_resolved_without_review: bool,
    /// Hard invariant: never hide the adapter confidence, source class, or discovery mode.
    pub hides_adapter_confidence_or_discovery_mode: bool,
    /// Hard invariant: never conceal a downgrade or drift behind generic status wording.
    pub conceals_downgrade_or_drift_in_generic_status_wording: bool,
}

impl M5AdapterDiscoveryControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5AdapterDiscoveryAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5AdapterDiscoveryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5AdapterDiscoveryExportField> =
            self.export_fields.iter().copied().collect();
        M5AdapterDiscoveryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.relabels_target_without_attributable_review
            && !self.lower_confidence_overwrites_resolved_without_review
            && !self.hides_adapter_confidence_or_discovery_mode
            && !self.conceals_downgrade_or_drift_in_generic_status_wording
    }

    /// True when every resolved example on this row is honest: no clean chip hides its confidence
    /// basis, and no clean card renders a silent relabel or overwrites a higher-confidence target.
    fn examples_are_honest(&self) -> bool {
        self.adapter_confidence_chip_examples
            .iter()
            .all(|ex| !(ex.is_clean() && ex.hides_confidence_basis()))
            && self.discovery_diff_card_examples.iter().all(|ex| {
                !(ex.is_clean()
                    && (ex.renders_silent_relabel
                        || ex.overwrites_higher_confidence_without_review))
            })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdapterDiscoveryVocabularySet {
    /// Boundary-disposition tokens (bound from the frozen matrix).
    pub boundary_dispositions: Vec<String>,
    /// Certainty tokens.
    pub certainties: Vec<String>,
    /// Adapter / source-class tokens (bound from the build/host-governance object model).
    pub adapter_source_classes: Vec<String>,
    /// Adapter-confidence tokens (bound from the build/host-governance object model).
    pub adapter_confidences: Vec<String>,
    /// Discovery-confidence tokens (bound from the target-discovery object model).
    pub discovery_confidences: Vec<String>,
    /// Narrowing-reason tokens (bound from the build/host-governance object model).
    pub narrowing_reasons: Vec<String>,
    /// Chip degrade-reason tokens.
    pub chip_degrade_reasons: Vec<String>,
    /// Card degrade-reason tokens.
    pub card_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5AdapterDiscoveryVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            boundary_dispositions: tokens(&M5BuildRemoteBoundaryDisposition::ALL, |v| v.as_str()),
            certainties: tokens(&M5AdapterDiscoveryCertainty::ALL, |v| v.as_str()),
            adapter_source_classes: tokens(&TargetDiscoveryClass::ALL, |v| v.as_str()),
            adapter_confidences: tokens(&AdapterConfidence::ALL, |v| v.as_str()),
            discovery_confidences: tokens(&DiscoveryConfidence::ALL, |v| v.as_str()),
            narrowing_reasons: tokens(&NarrowingReason::ALL, |v| v.as_str()),
            chip_degrade_reasons: tokens(&M5AdapterConfidenceChipDegradeReason::ALL, |v| {
                v.as_str()
            }),
            card_degrade_reasons: tokens(&M5DiscoveryDiffCardDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5AdapterDiscoveryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5AdapterDiscoveryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5AdapterDiscoveryExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5BuildRemoteConsumerSurface::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdapterDiscoveryGovernanceReview {
    /// The chip always names its adapter / source class and confidence band.
    pub chip_names_source_class_and_confidence_band: bool,
    /// The chip always names its discovery mode and any current downgrade reason.
    pub chip_names_discovery_mode_and_downgrade_reason: bool,
    /// The confidence basis is always explicit, never menu-only.
    pub confidence_basis_always_explicit: bool,
    /// The card always shows the previous and current target identity.
    pub card_shows_previous_and_current_target: bool,
    /// The card always shows the changed certainty and its review state.
    pub card_shows_changed_certainty_and_review_state: bool,
    /// Material drift is never silently relabeled.
    pub material_drift_never_silently_relabeled: bool,
    /// Lower-confidence discovery never overwrites a resolved target without review.
    pub lower_confidence_never_overwrites_resolved_without_review: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdapterDiscoveryConsumerProjection {
    /// Run / test / debug surfaces consume the shared confidence vocabulary.
    pub run_test_debug_surfaces_consume_confidence_vocabulary: bool,
    /// Preview surfaces consume the shared confidence vocabulary.
    pub preview_surfaces_consume_confidence_vocabulary: bool,
    /// AI tool-routing surfaces consume the shared confidence vocabulary.
    pub ai_tool_routing_consumes_confidence_vocabulary: bool,
    /// Support / export reads a single canonical confidence source.
    pub support_export_reads_single_confidence_source: bool,
    /// Target-discovery language stays consistent across every surface.
    pub discovery_language_consistent_across_surfaces: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdapterDiscoveryProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdapterDiscoveryReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting boundary audit for the lane.
    pub boundary_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5AdapterDiscoveryControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AdapterDiscoveryControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5AdapterDiscoveryControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AdapterDiscoveryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AdapterDiscoveryGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AdapterDiscoveryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AdapterDiscoveryProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AdapterDiscoveryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 adapter-confidence-chip / discovery-diff-card controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdapterDiscoveryControlsPacket {
    /// Record kind; must equal [`M5_ADAPTER_DISCOVERY_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ADAPTER_DISCOVERY_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5AdapterDiscoveryControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AdapterDiscoveryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AdapterDiscoveryGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AdapterDiscoveryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AdapterDiscoveryProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AdapterDiscoveryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AdapterDiscoveryControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5AdapterDiscoveryControlsPacketInput) -> Self {
        Self {
            record_kind: M5_ADAPTER_DISCOVERY_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_ADAPTER_DISCOVERY_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            controls_label: input.controls_label,
            controls_rows: input.controls_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the controls-packet invariants.
    pub fn validate(&self) -> Vec<M5AdapterDiscoveryControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ADAPTER_DISCOVERY_CONTROLS_RECORD_KIND {
            violations.push(M5AdapterDiscoveryControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ADAPTER_DISCOVERY_CONTROLS_SCHEMA_VERSION {
            violations.push(M5AdapterDiscoveryControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AdapterDiscoveryControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5AdapterDiscoveryControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 adapter-discovery controls packet serializes"),
        ) {
            violations.push(M5AdapterDiscoveryControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 adapter-discovery controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,chip_examples,card_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .adapter_confidence_chip_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.discovery_diff_card_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.adapter_confidence_chip_examples.len(),
                row.discovery_diff_card_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Adapter-Confidence-Chip and Discovery-Diff-Card Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Certainties: {}\n",
            self.vocabulary_set.certainties.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.controls_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Chip examples: {} / card examples: {}\n",
                row.adapter_confidence_chip_examples.len(),
                row.discovery_diff_card_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5AdapterDiscoveryControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AdapterDiscoveryControlsViolation>),
}

impl fmt::Display for M5AdapterDiscoveryControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 adapter-discovery controls export parse failed: {error}"
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
                    "m5 adapter-discovery controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AdapterDiscoveryControlsArtifactError {}

/// Validation failures emitted by [`M5AdapterDiscoveryControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AdapterDiscoveryControlsViolation {
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
    /// The controls packet declares no rows.
    NoControlsRows,
    /// A controls row is incomplete.
    ControlsRowIncomplete,
    /// A controls row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row does not point at both component schemas.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (hidden confidence basis, silent relabel, or
    /// unreviewed overwrite).
    DishonestExample,
    /// A controls row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// AC1 is not proven: clean chips do not cover every certainty, no source/confidence/mode
    /// unstated chip degrades, or a clean chip hides its confidence basis.
    Ac1NotProven,
    /// AC2 / no-higher-confidence-overwrite is not proven: no silent-relabel or unreviewed-overwrite
    /// card degrades, no clean card shows an attributable review state, or a clean card silently
    /// relabels / overwrites.
    Ac2NotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5AdapterDiscoveryControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoControlsRows => "no_controls_rows",
            Self::ControlsRowIncomplete => "controls_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::Ac1NotProven => "ac1_not_proven",
            Self::Ac2NotProven => "ac2_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_adapter_discovery_controls_export(
) -> Result<M5AdapterDiscoveryControlsPacket, M5AdapterDiscoveryControlsArtifactError> {
    let packet: M5AdapterDiscoveryControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-adapter-confidence-chip-discovery-diff-card-controls-proof/support_export.json"
    )))
    .map_err(M5AdapterDiscoveryControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AdapterDiscoveryControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5AdapterDiscoveryControlsPacket,
    violations: &mut Vec<M5AdapterDiscoveryControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_ADAPTER_DISCOVERY_CONTROLS_SCHEMA_REF,
        M5_ADAPTER_DISCOVERY_CONTROLS_DOC_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_DOC_REF,
        M5_ADAPTER_CONFIDENCE_CHIP_SCHEMA_REF,
        M5_DISCOVERY_DIFF_CARD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AdapterDiscoveryControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5AdapterDiscoveryControlsPacket,
    violations: &mut Vec<M5AdapterDiscoveryControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5AdapterDiscoveryControlsViolation::NoControlsRows);
        return;
    }
    for row in &packet.controls_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5AdapterDiscoveryControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5AdapterDiscoveryControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5AdapterDiscoveryControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_ADAPTER_CONFIDENCE_CHIP_SCHEMA_REF)
            || !refs.contains(M5_DISCOVERY_DIFF_CARD_SCHEMA_REF)
        {
            violations.push(M5AdapterDiscoveryControlsViolation::ComponentSchemaRefMissing);
        }
        if row.adapter_confidence_chip_examples.is_empty()
            || row.discovery_diff_card_examples.is_empty()
        {
            violations.push(M5AdapterDiscoveryControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5AdapterDiscoveryControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5AdapterDiscoveryControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5AdapterDiscoveryControlsPacket,
    violations: &mut Vec<M5AdapterDiscoveryControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.chip_names_source_class_and_confidence_band,
        review.chip_names_discovery_mode_and_downgrade_reason,
        review.confidence_basis_always_explicit,
        review.card_shows_previous_and_current_target,
        review.card_shows_changed_certainty_and_review_state,
        review.material_drift_never_silently_relabeled,
        review.lower_confidence_never_overwrites_resolved_without_review,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5AdapterDiscoveryControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AdapterDiscoveryControlsPacket,
    violations: &mut Vec<M5AdapterDiscoveryControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.run_test_debug_surfaces_consume_confidence_vocabulary,
        projection.preview_surfaces_consume_confidence_vocabulary,
        projection.ai_tool_routing_consumes_confidence_vocabulary,
        projection.support_export_reads_single_confidence_source,
        projection.discovery_language_consistent_across_surfaces,
    ] {
        if !ok {
            violations.push(M5AdapterDiscoveryControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AdapterDiscoveryControlsPacket,
    violations: &mut Vec<M5AdapterDiscoveryControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AdapterDiscoveryControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AdapterDiscoveryControlsPacket,
    violations: &mut Vec<M5AdapterDiscoveryControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.boundary_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5AdapterDiscoveryControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5AdapterDiscoveryControlsPacket,
    violations: &mut Vec<M5AdapterDiscoveryControlsViolation>,
) {
    let chip_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.adapter_confidence_chip_examples.iter())
    };
    let card_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.discovery_diff_card_examples.iter())
    };

    // AC1: a user can read exact / compatible / heuristic / imported / downgraded / stale before
    // invoking — clean chips cover every certainty, a source-class-unstated chip degrades, a
    // confidence-band-unstated chip degrades, a discovery-mode-unstated chip degrades, and no clean
    // chip hides its confidence basis.
    let clean_certainties: BTreeSet<&str> = chip_examples()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.certainty.as_str())
        .collect();
    let covers_all_certainties = M5AdapterDiscoveryCertainty::ALL
        .iter()
        .all(|certainty| clean_certainties.contains(certainty.as_str()));
    let source_class_unstated_degrades = chip_examples().any(|ex| {
        ex.degrade_reason == Some(M5AdapterConfidenceChipDegradeReason::SourceClassUnstated)
    });
    let confidence_band_unstated_degrades = chip_examples().any(|ex| {
        ex.degrade_reason == Some(M5AdapterConfidenceChipDegradeReason::ConfidenceBandUnstated)
    });
    let discovery_mode_unstated_degrades = chip_examples().any(|ex| {
        ex.degrade_reason == Some(M5AdapterConfidenceChipDegradeReason::DiscoveryModeUnstated)
    });
    let no_clean_chip_hides =
        chip_examples().all(|ex| !(ex.is_clean() && ex.hides_confidence_basis()));
    if !(covers_all_certainties
        && source_class_unstated_degrades
        && confidence_band_unstated_degrades
        && discovery_mode_unstated_degrades
        && no_clean_chip_hides)
    {
        violations.push(M5AdapterDiscoveryControlsViolation::Ac1NotProven);
    }

    // AC2 + no-higher-confidence-overwrite: material drift produces an attributable review state,
    // never a silent relabel, and weaker discovery never overwrites a resolved target without
    // review — at least one silent-relabel card degrades, at least one unreviewed-overwrite card
    // degrades, at least one clean card shows an attributable review state, and no clean card
    // silently relabels or overwrites.
    let silent_relabel_degrades = card_examples().any(|ex| {
        ex.degrade_reason == Some(M5DiscoveryDiffCardDegradeReason::SilentRelabelWithoutReview)
            && ex.renders_silent_relabel
    });
    let overwrite_degrades = card_examples().any(|ex| {
        ex.degrade_reason
            == Some(M5DiscoveryDiffCardDegradeReason::LowerConfidenceOverwroteResolved)
            && ex.overwrites_higher_confidence_without_review
    });
    let clean_card_shows_review =
        card_examples().any(|ex| ex.is_clean() && ex.material_change && ex.attributed_review_state);
    let no_clean_card_overwrites = card_examples().all(|ex| {
        !(ex.is_clean()
            && (ex.renders_silent_relabel || ex.overwrites_higher_confidence_without_review))
    });
    if !(silent_relabel_degrades
        && overwrite_degrades
        && clean_card_shows_review
        && no_clean_card_overwrites)
    {
        violations.push(M5AdapterDiscoveryControlsViolation::Ac2NotProven);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
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

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Repo-relative path of the M5 build/host-governance object model bound by this lane.
pub const M5_ADAPTER_DISCOVERY_BUILD_GOVERNANCE_PATH: &str = M5_BUILD_AND_HOST_GOVERNANCE_PATH;

/// Repo-relative path of the M5 target-discovery object model bound by this lane.
pub const M5_ADAPTER_DISCOVERY_TARGET_DISCOVERY_PATH: &str = M5_TARGET_DISCOVERY_PATH;
