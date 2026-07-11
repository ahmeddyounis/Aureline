//! Implemented M5 compatibility-label-strip and publisher-continuity-row primitives.
//!
//! The frozen [marketplace / install-review component matrix][matrix] names the reusable
//! extension-marketplace UI components and locks their controlled vocabulary. This module is the
//! second implement lane over that matrix: it turns the two lifecycle-and-provenance components —
//! the **compatibility label strip** and the **publisher continuity row** — into resolvers that
//! produce export-safe, honest projections, so a user can read the compatibility range, manifest
//! schema or host-version range, lifecycle state, replacement path, publisher continuity, and
//! transfer history from the listing, detail, install, diagnostics, and exported surfaces without
//! quietly carrying stale trust forward.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render compatibility-label strips with host/version range, schema version, lifecycle state,
//!   replacement path, and evidence freshness.** [`resolve_compatibility_label_strip`] refuses to
//!   read as a clean strip when the compatibility state, host/runtime model, host-version range, or
//!   manifest-schema version is unstated, when an incompatible artifact reads as ready, when a
//!   deprecated / end-of-life / yanked artifact carries no replacement path, or when Certified /
//!   Supported language is left in place while the underlying evidence is no longer current; it
//!   degrades instead.
//! * **Render publisher-continuity rows with verified, transferred, lost, mirrored, or unverifiable
//!   continuity state plus history where available.** [`resolve_publisher_continuity_row`] degrades
//!   when the registry source is unresolved or collapsed, when a transferred / deprecated / lost
//!   publisher carries no visible replacement / continuity language, when available transfer history
//!   is hidden, or when Certified / Supported language survives on stale or unverifiable evidence.
//! * **Prevent stale evidence from leaving Certified / Supported language in place when the
//!   underlying report is no longer current.** Both resolvers narrow the claim the moment evidence
//!   goes stale or unverifiable, and the packet proves — by resolved examples, not governance bools
//!   — that a stale-certified strip and a stale-or-unverifiable-certified row both degrade and that
//!   no clean example leaves the overclaim in place.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5MarketplaceInstallDisposition`] marketplace / install-disposition vocabulary, the
//! [`M5RegistrySourceClass`] registry-source vocabulary, the [`M5CompatibilityState`] compatibility
//! vocabulary, the [`M5HostRuntimeModel`] host/runtime vocabulary, and the
//! [`M5PublisherContinuityState`] publisher-continuity vocabulary — so marketplace, extensions,
//! registry, help, and support surfaces can never fork their own source, compatibility, lifecycle,
//! or publisher wording or invent feature-local badges. Raw secret values and private endpoints stay
//! outside the export boundary.
//!
//! [matrix]: crate::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_compatibility_continuity_controls,
    seeded_m5_compatibility_continuity_controls_marketplace_ui_beta_narrowed,
    seeded_m5_compatibility_continuity_controls_registry_ui_preview_narrowed,
    M5_COMPATIBILITY_CONTINUITY_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix::{
    M5CompatibilityState, M5HostRuntimeModel, M5MarketplaceInstallAccessibilityRoute,
    M5MarketplaceInstallComponentFamily, M5MarketplaceInstallConsumerSurface,
    M5MarketplaceInstallDeploymentLine, M5MarketplaceInstallDisposition,
    M5MarketplaceInstallDowngradeTrigger, M5MarketplaceInstallQualificationClass,
    M5MarketplaceInstallRequiredLabel, M5PublisherContinuityState, M5RegistrySourceClass,
    M5_COMPATIBILITY_LABEL_STRIP_SCHEMA_REF, M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
    M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF, M5_PUBLISHER_CONTINUITY_ROW_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5CompatibilityContinuityControlsPacket`].
pub const M5_COMPATIBILITY_CONTINUITY_CONTROLS_RECORD_KIND: &str =
    "implement_m5_compatibility_label_strip_and_publisher_continuity_row_controls";

/// Schema version for M5 compatibility-label-strip / publisher-continuity-row controls records.
pub const M5_COMPATIBILITY_CONTINUITY_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_COMPATIBILITY_CONTINUITY_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-compatibility-label-strip-publisher-continuity-row-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_COMPATIBILITY_CONTINUITY_CONTROLS_DOC_REF: &str =
    "docs/marketplace/m5_compatibility_label_strip_and_publisher_continuity_row_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COMPATIBILITY_CONTINUITY_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-compatibility-label-strip-publisher-continuity-row-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_COMPATIBILITY_CONTINUITY_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-compatibility-label-strip-publisher-continuity-row-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_COMPATIBILITY_CONTINUITY_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-compatibility-label-strip-publisher-continuity-row-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_COMPATIBILITY_CONTINUITY_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-compatibility-label-strip-publisher-continuity-row-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface
/// taxonomy so no lane invents a parallel surface set.
pub type M5CompatibilityContinuityConsumerSurface = M5MarketplaceInstallConsumerSurface;

/// Controlled lifecycle state a compatibility-label strip names, so a deprecated, end-of-life, or
/// yanked artifact is never presented as freshly active without a replacement path. Minted by this
/// lane because the frozen matrix carries compatibility and publisher continuity but not a
/// per-artifact lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityLifecycleState {
    /// Actively maintained.
    Active,
    /// Available as a preview / prerelease.
    Preview,
    /// Deprecated, still installable, with a replacement path.
    Deprecated,
    /// End of life, no longer maintained.
    EndOfLife,
    /// Yanked / withdrawn.
    Yanked,
    /// The lifecycle state cannot currently be resolved.
    LifecycleUnknown,
}

impl M5CompatibilityLifecycleState {
    /// Every lifecycle state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Active,
        Self::Preview,
        Self::Deprecated,
        Self::EndOfLife,
        Self::Yanked,
        Self::LifecycleUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Preview => "preview",
            Self::Deprecated => "deprecated",
            Self::EndOfLife => "end_of_life",
            Self::Yanked => "yanked",
            Self::LifecycleUnknown => "lifecycle_unknown",
        }
    }

    /// True when this lifecycle state requires a visible replacement path before the strip can read
    /// clean.
    pub const fn requires_replacement_path(self) -> bool {
        matches!(self, Self::Deprecated | Self::EndOfLife | Self::Yanked)
    }
}

/// Controlled publisher-continuity presentation a publisher-continuity row names. Minted by this lane
/// to project the frozen [`M5PublisherContinuityState`] plus registry source into the five states the
/// spec requires — verified, transferred, lost, mirrored, or unverifiable — plus the continuous and
/// deprecated baselines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublisherContinuityPresentation {
    /// The publisher is verified.
    Verified,
    /// The publisher is continuous with the original owner.
    Continuous,
    /// The publisher was transferred to a new owner.
    Transferred,
    /// The publisher deprecated the artifact and named a replacement.
    Deprecated,
    /// The publisher abandoned the artifact; continuity is lost.
    Lost,
    /// Continuity is preserved through a mirror / offline registry.
    Mirrored,
    /// The publisher continuity cannot be verified.
    Unverifiable,
}

impl M5PublisherContinuityPresentation {
    /// Every presentation, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Verified,
        Self::Continuous,
        Self::Transferred,
        Self::Deprecated,
        Self::Lost,
        Self::Mirrored,
        Self::Unverifiable,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Continuous => "continuous",
            Self::Transferred => "transferred",
            Self::Deprecated => "deprecated",
            Self::Lost => "lost",
            Self::Mirrored => "mirrored",
            Self::Unverifiable => "unverifiable",
        }
    }
}

/// One mandatory rendered part a compatibility-label strip or publisher-continuity row must be able
/// to show, so no lifecycle, replacement, or continuity fact is left implicit behind compact chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityContinuityAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The compatibility range / state (strip).
    CompatibilityRange,
    /// The host / runtime model (strip).
    HostRuntimeModel,
    /// The host-version range (strip).
    HostVersionRange,
    /// The manifest-schema version (strip).
    ManifestSchemaVersion,
    /// The lifecycle state (strip).
    LifecycleState,
    /// The replacement path for a deprecated / end-of-life / yanked artifact (strip).
    ReplacementPath,
    /// The evidence-freshness disclosure (both components).
    EvidenceFreshness,
    /// The publisher-continuity presentation (row).
    PublisherContinuity,
    /// The registry source class behind the artifact (row).
    RegistrySourceClass,
    /// The transfer / continuity history (row).
    TransferHistory,
    /// The visible replacement / continuity language for a changed publisher (row).
    ContinuityLanguage,
}

impl M5CompatibilityContinuityAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::CompatibilityRange,
        Self::HostRuntimeModel,
        Self::HostVersionRange,
        Self::ManifestSchemaVersion,
        Self::LifecycleState,
        Self::ReplacementPath,
        Self::EvidenceFreshness,
        Self::PublisherContinuity,
        Self::RegistrySourceClass,
        Self::TransferHistory,
        Self::ContinuityLanguage,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::CompatibilityRange => "compatibility_range",
            Self::HostRuntimeModel => "host_runtime_model",
            Self::HostVersionRange => "host_version_range",
            Self::ManifestSchemaVersion => "manifest_schema_version",
            Self::LifecycleState => "lifecycle_state",
            Self::ReplacementPath => "replacement_path",
            Self::EvidenceFreshness => "evidence_freshness",
            Self::PublisherContinuity => "publisher_continuity",
            Self::RegistrySourceClass => "registry_source_class",
            Self::TransferHistory => "transfer_history",
            Self::ContinuityLanguage => "continuity_language",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route to review the fact
/// behind a degraded compatibility or continuity component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityContinuityNextAction {
    /// Review the compatibility range, host model, and version / schema range.
    ReviewCompatibility,
    /// Review the lifecycle state and replacement path.
    ReviewReplacementPath,
    /// Review the publisher continuity and registry source class.
    ReviewPublisherContinuity,
    /// Review the transfer / continuity history.
    ReviewTransferHistory,
    /// Review the evidence freshness for a stale or unverifiable signal.
    ReviewEvidenceFreshness,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5CompatibilityContinuityNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewCompatibility,
        Self::ReviewReplacementPath,
        Self::ReviewPublisherContinuity,
        Self::ReviewTransferHistory,
        Self::ReviewEvidenceFreshness,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewCompatibility => "review_compatibility",
            Self::ReviewReplacementPath => "review_replacement_path",
            Self::ReviewPublisherContinuity => "review_publisher_continuity",
            Self::ReviewTransferHistory => "review_transfer_history",
            Self::ReviewEvidenceFreshness => "review_evidence_freshness",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityContinuityExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The marketplace dispositions carried.
    Dispositions,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The compatibility state named by the strip.
    CompatibilityState,
    /// The lifecycle state named by the strip.
    LifecycleState,
    /// The replacement path named by the strip.
    ReplacementPath,
    /// The publisher-continuity presentation named by the row.
    PublisherContinuity,
    /// The registry source class named by the row.
    RegistrySourceClass,
    /// The evidence-freshness disclosure named by both components.
    EvidenceFreshness,
}

impl M5CompatibilityContinuityExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::CompatibilityState,
        Self::LifecycleState,
        Self::ReplacementPath,
        Self::PublisherContinuity,
        Self::RegistrySourceClass,
        Self::EvidenceFreshness,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::Dispositions => "dispositions",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::CompatibilityState => "compatibility_state",
            Self::LifecycleState => "lifecycle_state",
            Self::ReplacementPath => "replacement_path",
            Self::PublisherContinuity => "publisher_continuity",
            Self::RegistrySourceClass => "registry_source_class",
            Self::EvidenceFreshness => "evidence_freshness",
        }
    }
}

/// Reason a compatibility-label strip degraded below a clean, fully-legible state. The degrade-first
/// ladder returns one of these instead of ever letting an ambiguous strip read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityLabelStripDegradeReason {
    /// The artifact identity is unstated.
    ArtifactIdentityUnstated,
    /// The compatibility state cannot currently be resolved.
    CompatibilityUnresolved,
    /// The host / runtime model cannot currently be resolved.
    HostModelUnresolved,
    /// The host-version range is unstated.
    HostVersionRangeUnstated,
    /// The manifest-schema version is unstated.
    ManifestSchemaVersionUnstated,
    /// An incompatible artifact reads as ready to install.
    IncompatibleShownAsReady,
    /// The lifecycle state is unstated.
    LifecycleStateUnstated,
    /// A deprecated / end-of-life / yanked artifact carries no replacement path.
    ReplacementPathMissing,
    /// Certified / Supported language is left in place while the evidence is stale.
    StaleEvidenceCertifiedOverclaim,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5CompatibilityLabelStripDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ArtifactIdentityUnstated,
        Self::CompatibilityUnresolved,
        Self::HostModelUnresolved,
        Self::HostVersionRangeUnstated,
        Self::ManifestSchemaVersionUnstated,
        Self::IncompatibleShownAsReady,
        Self::LifecycleStateUnstated,
        Self::ReplacementPathMissing,
        Self::StaleEvidenceCertifiedOverclaim,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactIdentityUnstated => "artifact_identity_unstated",
            Self::CompatibilityUnresolved => "compatibility_unresolved",
            Self::HostModelUnresolved => "host_model_unresolved",
            Self::HostVersionRangeUnstated => "host_version_range_unstated",
            Self::ManifestSchemaVersionUnstated => "manifest_schema_version_unstated",
            Self::IncompatibleShownAsReady => "incompatible_shown_as_ready",
            Self::LifecycleStateUnstated => "lifecycle_state_unstated",
            Self::ReplacementPathMissing => "replacement_path_missing",
            Self::StaleEvidenceCertifiedOverclaim => "stale_evidence_certified_overclaim",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5CompatibilityContinuityNextAction {
        match self {
            Self::ArtifactIdentityUnstated
            | Self::CompatibilityUnresolved
            | Self::HostModelUnresolved
            | Self::HostVersionRangeUnstated
            | Self::ManifestSchemaVersionUnstated
            | Self::IncompatibleShownAsReady => {
                M5CompatibilityContinuityNextAction::ReviewCompatibility
            }
            Self::LifecycleStateUnstated | Self::ReplacementPathMissing => {
                M5CompatibilityContinuityNextAction::ReviewReplacementPath
            }
            Self::StaleEvidenceCertifiedOverclaim | Self::ProofStale => {
                M5CompatibilityContinuityNextAction::ReviewEvidenceFreshness
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5MarketplaceInstallDowngradeTrigger {
        match self {
            Self::ArtifactIdentityUnstated
            | Self::LifecycleStateUnstated
            | Self::ReplacementPathMissing => {
                M5MarketplaceInstallDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::CompatibilityUnresolved
            | Self::HostVersionRangeUnstated
            | Self::ManifestSchemaVersionUnstated
            | Self::IncompatibleShownAsReady => {
                M5MarketplaceInstallDowngradeTrigger::CompatibilityRangeUnstated
            }
            Self::HostModelUnresolved => M5MarketplaceInstallDowngradeTrigger::HostModelUnstated,
            Self::StaleEvidenceCertifiedOverclaim | Self::ProofStale => {
                M5MarketplaceInstallDowngradeTrigger::ProofStale
            }
        }
    }
}

/// Reason a publisher-continuity row degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublisherContinuityRowDegradeReason {
    /// The artifact identity is unstated.
    ArtifactIdentityUnstated,
    /// The registry source cannot currently be resolved.
    RegistrySourceUnresolved,
    /// The registry source class is collapsed into one ambiguous origin.
    SourceClassCollapsedIntoAmbiguousOrigin,
    /// A transferred / deprecated / lost publisher carries no visible replacement / continuity
    /// language.
    ContinuityLanguageHidden,
    /// Available transfer / continuity history is hidden.
    TransferHistoryHidden,
    /// Certified / Supported language is left in place while the evidence is stale or unverifiable.
    StaleOrUnverifiableCertifiedOverclaim,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5PublisherContinuityRowDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ArtifactIdentityUnstated,
        Self::RegistrySourceUnresolved,
        Self::SourceClassCollapsedIntoAmbiguousOrigin,
        Self::ContinuityLanguageHidden,
        Self::TransferHistoryHidden,
        Self::StaleOrUnverifiableCertifiedOverclaim,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactIdentityUnstated => "artifact_identity_unstated",
            Self::RegistrySourceUnresolved => "registry_source_unresolved",
            Self::SourceClassCollapsedIntoAmbiguousOrigin => {
                "source_class_collapsed_into_ambiguous_origin"
            }
            Self::ContinuityLanguageHidden => "continuity_language_hidden",
            Self::TransferHistoryHidden => "transfer_history_hidden",
            Self::StaleOrUnverifiableCertifiedOverclaim => {
                "stale_or_unverifiable_certified_overclaim"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5CompatibilityContinuityNextAction {
        match self {
            Self::ArtifactIdentityUnstated
            | Self::RegistrySourceUnresolved
            | Self::SourceClassCollapsedIntoAmbiguousOrigin
            | Self::ContinuityLanguageHidden => {
                M5CompatibilityContinuityNextAction::ReviewPublisherContinuity
            }
            Self::TransferHistoryHidden => {
                M5CompatibilityContinuityNextAction::ReviewTransferHistory
            }
            Self::StaleOrUnverifiableCertifiedOverclaim | Self::ProofStale => {
                M5CompatibilityContinuityNextAction::ReviewEvidenceFreshness
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5MarketplaceInstallDowngradeTrigger {
        match self {
            Self::ArtifactIdentityUnstated => {
                M5MarketplaceInstallDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::RegistrySourceUnresolved | Self::SourceClassCollapsedIntoAmbiguousOrigin => {
                M5MarketplaceInstallDowngradeTrigger::RegistrySourceClassCollapsed
            }
            Self::ContinuityLanguageHidden | Self::TransferHistoryHidden => {
                M5MarketplaceInstallDowngradeTrigger::PublisherTransferHidden
            }
            Self::StaleOrUnverifiableCertifiedOverclaim | Self::ProofStale => {
                M5MarketplaceInstallDowngradeTrigger::ProofStale
            }
        }
    }
}

/// Maps a registry source class to the single controlled marketplace disposition, or `None` when the
/// source cannot be resolved — an unresolved source never borrows a public / mirrored / enterprise
/// word.
fn disposition_for_source(
    source: M5RegistrySourceClass,
) -> Option<M5MarketplaceInstallDisposition> {
    use M5MarketplaceInstallDisposition as D;
    match source {
        M5RegistrySourceClass::PublicRegistry => Some(D::Public),
        M5RegistrySourceClass::MirroredRegistry => Some(D::Mirrored),
        M5RegistrySourceClass::EnterpriseRegistry => Some(D::Enterprise),
        M5RegistrySourceClass::SideLoaded => Some(D::SideLoad),
        M5RegistrySourceClass::VerifiedPartner => Some(D::Verified),
        M5RegistrySourceClass::SourceUnknown => None,
    }
}

/// True when the compatibility state reads as freely installable.
fn compatibility_is_installable(state: M5CompatibilityState) -> bool {
    matches!(
        state,
        M5CompatibilityState::Compatible | M5CompatibilityState::CompatibleWithWarnings
    )
}

/// True when the compatibility state is an incompatible / degraded one.
fn compatibility_is_incompatible(state: M5CompatibilityState) -> bool {
    matches!(
        state,
        M5CompatibilityState::Incompatible
            | M5CompatibilityState::DegradedHost
            | M5CompatibilityState::UnsupportedRuntime
    )
}

/// True when the publisher continuity represents a change the row must state with visible
/// replacement / continuity language.
fn publisher_changed(state: M5PublisherContinuityState) -> bool {
    matches!(
        state,
        M5PublisherContinuityState::Transferred
            | M5PublisherContinuityState::Deprecated
            | M5PublisherContinuityState::Abandoned
    )
}

/// Projects the frozen publisher-continuity state plus registry source into the controlled
/// presentation the row renders.
fn presentation_for(
    state: M5PublisherContinuityState,
    source: M5RegistrySourceClass,
) -> M5PublisherContinuityPresentation {
    use M5PublisherContinuityPresentation as P;
    match state {
        M5PublisherContinuityState::VerifiedPublisher => P::Verified,
        M5PublisherContinuityState::Continuous => {
            if matches!(source, M5RegistrySourceClass::MirroredRegistry) {
                P::Mirrored
            } else {
                P::Continuous
            }
        }
        M5PublisherContinuityState::Transferred => P::Transferred,
        M5PublisherContinuityState::Deprecated => P::Deprecated,
        M5PublisherContinuityState::Abandoned => P::Lost,
        M5PublisherContinuityState::ContinuityUnknown => P::Unverifiable,
    }
}

/// Input to [`resolve_compatibility_label_strip`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CompatibilityLabelStripResolutionInput {
    /// Stable identity of the strip instance.
    pub strip_id: String,
    /// The artifact identity (name / id) shown; empty means unstated.
    pub artifact_identity: String,
    /// The compatibility state.
    pub compatibility: M5CompatibilityState,
    /// The host / runtime model.
    pub host_runtime_model: M5HostRuntimeModel,
    /// The host-version range; empty means unstated.
    pub host_version_range: String,
    /// The manifest-schema version; empty means unstated.
    pub manifest_schema_version: String,
    /// The lifecycle state.
    pub lifecycle: M5CompatibilityLifecycleState,
    /// The replacement path for a deprecated / end-of-life / yanked artifact; empty means missing.
    pub replacement_path: String,
    /// True when the strip carries Certified / Supported language.
    pub certified_or_supported_claimed: bool,
    /// True when the underlying compatibility evidence is current.
    pub evidence_fresh: bool,
    /// True when the strip reads an incompatible artifact as ready to install.
    pub reads_incompatible_as_ready: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe compatibility-label strip projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCompatibilityLabelStrip {
    /// Stable identity of the strip instance.
    pub strip_id: String,
    /// The artifact identity named by the strip.
    pub artifact_identity: String,
    /// The compatibility token named by the strip.
    pub compatibility: String,
    /// Whether the artifact reads as installable (compatible).
    pub is_installable: bool,
    /// The host / runtime token named by the strip.
    pub host_runtime_model: String,
    /// The host-version range named by the strip.
    pub host_version_range: String,
    /// The manifest-schema version named by the strip.
    pub manifest_schema_version: String,
    /// The lifecycle token named by the strip.
    pub lifecycle: String,
    /// Whether the lifecycle state requires a replacement path.
    pub requires_replacement_path: bool,
    /// The replacement path named by the strip.
    pub replacement_path: String,
    /// Whether Certified / Supported language is claimed.
    pub certified_or_supported_claimed: bool,
    /// Whether the underlying evidence is current.
    pub evidence_fresh: bool,
    /// Guardrail (MUST be `false` on a clean strip): an incompatible artifact reads as ready.
    pub presents_incompatible_as_ready: bool,
    /// Guardrail (MUST be `false` on a clean strip): stale evidence leaves a Certified / Supported
    /// overclaim in place.
    pub leaves_stale_certified_overclaim: bool,
    /// Degrade reason, if the strip could not read as a clean state.
    pub degrade_reason: Option<M5CompatibilityLabelStripDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5CompatibilityContinuityNextAction,
    /// Whether the compatibility facts are legible in full (clean strip naming every fact).
    pub fully_legible: bool,
}

impl M5ResolvedCompatibilityLabelStrip {
    /// Whether this strip reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_publisher_continuity_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PublisherContinuityRowResolutionInput {
    /// Stable identity of the row instance.
    pub row_id: String,
    /// The artifact identity (name / id) shown; empty means unstated.
    pub artifact_identity: String,
    /// The publisher continuity state.
    pub continuity: M5PublisherContinuityState,
    /// Where the artifact comes from.
    pub registry_source: M5RegistrySourceClass,
    /// The visible replacement / continuity language (successor / new owner); empty means absent.
    pub continuity_language: String,
    /// True when transfer / continuity history is available for this artifact.
    pub transfer_history_available: bool,
    /// True when available transfer / continuity history is stated on the row.
    pub transfer_history_stated: bool,
    /// True when the row carries Certified / Supported language.
    pub certified_or_supported_claimed: bool,
    /// True when the underlying continuity evidence is current.
    pub evidence_fresh: bool,
    /// True when the row reads the source class as one ambiguous origin across public / mirrored /
    /// enterprise.
    pub collapses_source_class: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe publisher-continuity row projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPublisherContinuityRow {
    /// Stable identity of the row instance.
    pub row_id: String,
    /// The artifact identity named by the row.
    pub artifact_identity: String,
    /// The publisher-continuity token named by the row.
    pub continuity: String,
    /// The controlled continuity presentation named by the row.
    pub presentation: String,
    /// Whether the publisher continuity represents a change requiring continuity language.
    pub publisher_changed: bool,
    /// The visible replacement / continuity language named by the row.
    pub continuity_language: String,
    /// The registry source token named by the row.
    pub registry_source: String,
    /// Single controlled source disposition, or `null` when the source is unresolved.
    pub source_disposition: Option<M5MarketplaceInstallDisposition>,
    /// Whether transfer / continuity history is available.
    pub transfer_history_available: bool,
    /// Whether available transfer / continuity history is stated.
    pub transfer_history_stated: bool,
    /// Whether Certified / Supported language is claimed.
    pub certified_or_supported_claimed: bool,
    /// Whether the underlying evidence is current.
    pub evidence_fresh: bool,
    /// Guardrail (MUST be `false` on a clean row): the source class is collapsed into one origin.
    pub collapses_source_class: bool,
    /// Guardrail (MUST be `false` on a clean row): a changed publisher hides its continuity language.
    pub hides_continuity_language: bool,
    /// Guardrail (MUST be `false` on a clean row): stale or unverifiable evidence leaves a Certified
    /// / Supported overclaim in place.
    pub leaves_stale_certified_overclaim: bool,
    /// Degrade reason, if the row could not read as a clean state.
    pub degrade_reason: Option<M5PublisherContinuityRowDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5CompatibilityContinuityNextAction,
    /// Whether the continuity facts are legible in full (clean row naming every fact).
    pub fully_legible: bool,
}

impl M5ResolvedPublisherContinuityRow {
    /// Whether this row reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5CompatibilityContinuityResolutionError {
    /// The strip id was empty.
    EmptyStripId,
    /// The row id was empty.
    EmptyRowId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5CompatibilityContinuityResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyStripId => "empty_strip_id",
            Self::EmptyRowId => "empty_row_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5CompatibilityContinuityResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 compatibility-continuity resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5CompatibilityContinuityResolutionError {}

/// Resolves a compatibility-label strip, keeping the lifecycle and compatibility truth explicit: the
/// strip names its compatibility range, host / runtime model, host-version range, manifest-schema
/// version, lifecycle state, and replacement path, never reads an incompatible artifact as ready, and
/// narrows the claim the moment Certified / Supported evidence goes stale.
pub fn resolve_compatibility_label_strip(
    input: M5CompatibilityLabelStripResolutionInput,
) -> Result<M5ResolvedCompatibilityLabelStrip, M5CompatibilityContinuityResolutionError> {
    if input.strip_id.trim().is_empty() {
        return Err(M5CompatibilityContinuityResolutionError::EmptyStripId);
    }
    if string_is_forbidden(&input.strip_id)
        || string_is_forbidden(&input.artifact_identity)
        || string_is_forbidden(&input.host_version_range)
        || string_is_forbidden(&input.manifest_schema_version)
        || string_is_forbidden(&input.replacement_path)
    {
        return Err(M5CompatibilityContinuityResolutionError::ForbiddenMaterial);
    }

    let is_installable = compatibility_is_installable(input.compatibility);
    let requires_replacement_path = input.lifecycle.requires_replacement_path();
    let presents_incompatible_as_ready =
        compatibility_is_incompatible(input.compatibility) && input.reads_incompatible_as_ready;
    let leaves_stale_certified_overclaim =
        input.certified_or_supported_claimed && !input.evidence_fresh;

    let degrade_reason = if input.artifact_identity.trim().is_empty() {
        Some(M5CompatibilityLabelStripDegradeReason::ArtifactIdentityUnstated)
    } else if matches!(
        input.compatibility,
        M5CompatibilityState::CompatibilityUnknown
    ) {
        Some(M5CompatibilityLabelStripDegradeReason::CompatibilityUnresolved)
    } else if matches!(input.host_runtime_model, M5HostRuntimeModel::HostUnknown) {
        Some(M5CompatibilityLabelStripDegradeReason::HostModelUnresolved)
    } else if input.host_version_range.trim().is_empty() {
        Some(M5CompatibilityLabelStripDegradeReason::HostVersionRangeUnstated)
    } else if input.manifest_schema_version.trim().is_empty() {
        Some(M5CompatibilityLabelStripDegradeReason::ManifestSchemaVersionUnstated)
    } else if presents_incompatible_as_ready {
        Some(M5CompatibilityLabelStripDegradeReason::IncompatibleShownAsReady)
    } else if matches!(
        input.lifecycle,
        M5CompatibilityLifecycleState::LifecycleUnknown
    ) {
        Some(M5CompatibilityLabelStripDegradeReason::LifecycleStateUnstated)
    } else if requires_replacement_path && input.replacement_path.trim().is_empty() {
        Some(M5CompatibilityLabelStripDegradeReason::ReplacementPathMissing)
    } else if leaves_stale_certified_overclaim {
        Some(M5CompatibilityLabelStripDegradeReason::StaleEvidenceCertifiedOverclaim)
    } else if !input.proof_fresh {
        Some(M5CompatibilityLabelStripDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5CompatibilityContinuityNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedCompatibilityLabelStrip {
        strip_id: input.strip_id,
        artifact_identity: input.artifact_identity,
        compatibility: input.compatibility.as_str().to_owned(),
        is_installable,
        host_runtime_model: input.host_runtime_model.as_str().to_owned(),
        host_version_range: input.host_version_range,
        manifest_schema_version: input.manifest_schema_version,
        lifecycle: input.lifecycle.as_str().to_owned(),
        requires_replacement_path,
        replacement_path: input.replacement_path,
        certified_or_supported_claimed: input.certified_or_supported_claimed,
        evidence_fresh: input.evidence_fresh,
        presents_incompatible_as_ready,
        leaves_stale_certified_overclaim,
        degrade_reason,
        next_action,
        fully_legible: degrade_reason.is_none(),
    })
}

/// Resolves a publisher-continuity row, keeping the publisher continuity explicit before install
/// trust silently continues: the row names its continuity presentation, registry source class,
/// replacement / continuity language, and transfer history, never collapses the source class, and
/// narrows the claim the moment Certified / Supported evidence goes stale or unverifiable.
pub fn resolve_publisher_continuity_row(
    input: M5PublisherContinuityRowResolutionInput,
) -> Result<M5ResolvedPublisherContinuityRow, M5CompatibilityContinuityResolutionError> {
    if input.row_id.trim().is_empty() {
        return Err(M5CompatibilityContinuityResolutionError::EmptyRowId);
    }
    if string_is_forbidden(&input.row_id)
        || string_is_forbidden(&input.artifact_identity)
        || string_is_forbidden(&input.continuity_language)
    {
        return Err(M5CompatibilityContinuityResolutionError::ForbiddenMaterial);
    }

    let publisher_changed_now = publisher_changed(input.continuity);
    let continuity_language_present = !input.continuity_language.trim().is_empty();
    let hides_continuity_language = publisher_changed_now && !continuity_language_present;
    let history_hidden = input.transfer_history_available && !input.transfer_history_stated;
    let unverifiable = matches!(
        input.continuity,
        M5PublisherContinuityState::ContinuityUnknown
    );
    let leaves_stale_certified_overclaim =
        input.certified_or_supported_claimed && (!input.evidence_fresh || unverifiable);

    let degrade_reason = if input.artifact_identity.trim().is_empty() {
        Some(M5PublisherContinuityRowDegradeReason::ArtifactIdentityUnstated)
    } else if matches!(input.registry_source, M5RegistrySourceClass::SourceUnknown) {
        Some(M5PublisherContinuityRowDegradeReason::RegistrySourceUnresolved)
    } else if input.collapses_source_class {
        Some(M5PublisherContinuityRowDegradeReason::SourceClassCollapsedIntoAmbiguousOrigin)
    } else if hides_continuity_language {
        Some(M5PublisherContinuityRowDegradeReason::ContinuityLanguageHidden)
    } else if history_hidden {
        Some(M5PublisherContinuityRowDegradeReason::TransferHistoryHidden)
    } else if leaves_stale_certified_overclaim {
        Some(M5PublisherContinuityRowDegradeReason::StaleOrUnverifiableCertifiedOverclaim)
    } else if !input.proof_fresh {
        Some(M5PublisherContinuityRowDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5CompatibilityContinuityNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedPublisherContinuityRow {
        row_id: input.row_id,
        artifact_identity: input.artifact_identity,
        continuity: input.continuity.as_str().to_owned(),
        presentation: presentation_for(input.continuity, input.registry_source)
            .as_str()
            .to_owned(),
        publisher_changed: publisher_changed_now,
        continuity_language: input.continuity_language,
        registry_source: input.registry_source.as_str().to_owned(),
        source_disposition: disposition_for_source(input.registry_source),
        transfer_history_available: input.transfer_history_available,
        transfer_history_stated: input.transfer_history_stated,
        certified_or_supported_claimed: input.certified_or_supported_claimed,
        evidence_fresh: input.evidence_fresh,
        collapses_source_class: input.collapses_source_class,
        hides_continuity_language,
        leaves_stale_certified_overclaim,
        degrade_reason,
        next_action,
        fully_legible: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved compatibility-label strip and
/// publisher-continuity row examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityContinuityControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5CompatibilityContinuityConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5MarketplaceInstallQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5MarketplaceInstallDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5MarketplaceInstallRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5MarketplaceInstallAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5CompatibilityContinuityAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5CompatibilityContinuityExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5MarketplaceInstallDowngradeTrigger>,
    /// Resolved compatibility-label strip examples.
    pub compatibility_label_strip_examples: Vec<M5ResolvedCompatibilityLabelStrip>,
    /// Resolved publisher-continuity row examples.
    pub publisher_continuity_row_examples: Vec<M5ResolvedPublisherContinuityRow>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never collapse the registry source class across public / mirrored /
    /// enterprise.
    pub collapses_registry_source_class_across_public_mirrored_enterprise: bool,
    /// Hard invariant: never hide the replacement path or lifecycle state behind compact chrome.
    pub hides_replacement_path_or_lifecycle_state: bool,
    /// Hard invariant: never hide a publisher transfer or its continuity language.
    pub hides_publisher_transfer_or_continuity_language: bool,
    /// Hard invariant: never leave Certified / Supported language on stale or unverifiable evidence.
    pub leaves_stale_evidence_certified_or_supported: bool,
}

impl M5CompatibilityContinuityControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5CompatibilityContinuityAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5CompatibilityContinuityAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5CompatibilityContinuityExportField> =
            self.export_fields.iter().copied().collect();
        M5CompatibilityContinuityExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.collapses_registry_source_class_across_public_mirrored_enterprise
            && !self.hides_replacement_path_or_lifecycle_state
            && !self.hides_publisher_transfer_or_continuity_language
            && !self.leaves_stale_evidence_certified_or_supported
    }

    /// True when every resolved example on this row is honest: no clean strip presents an
    /// incompatible artifact as ready or leaves a stale-certified overclaim, and no clean row
    /// collapses the source class, hides its continuity language, or leaves a stale / unverifiable
    /// certified overclaim.
    fn examples_are_honest(&self) -> bool {
        self.compatibility_label_strip_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.presents_incompatible_as_ready || ex.leaves_stale_certified_overclaim))
        }) && self.publisher_continuity_row_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.collapses_source_class
                    || ex.hides_continuity_language
                    || ex.leaves_stale_certified_overclaim))
        })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityContinuityVocabularySet {
    /// Marketplace / install-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Registry source-class tokens (bound from the frozen matrix).
    pub registry_source_classes: Vec<String>,
    /// Compatibility-state tokens (bound from the frozen matrix).
    pub compatibility_states: Vec<String>,
    /// Host / runtime-model tokens (bound from the frozen matrix).
    pub host_runtime_models: Vec<String>,
    /// Publisher-continuity tokens (bound from the frozen matrix).
    pub publisher_continuity_states: Vec<String>,
    /// Lifecycle-state tokens (minted by this lane).
    pub lifecycle_states: Vec<String>,
    /// Continuity-presentation tokens (minted by this lane).
    pub continuity_presentations: Vec<String>,
    /// Compatibility-label-strip degrade-reason tokens.
    pub compatibility_label_strip_degrade_reasons: Vec<String>,
    /// Publisher-continuity-row degrade-reason tokens.
    pub publisher_continuity_row_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5CompatibilityContinuityVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5MarketplaceInstallDisposition::ALL, |v| v.as_str()),
            registry_source_classes: tokens(&M5RegistrySourceClass::ALL, |v| v.as_str()),
            compatibility_states: tokens(&M5CompatibilityState::ALL, |v| v.as_str()),
            host_runtime_models: tokens(&M5HostRuntimeModel::ALL, |v| v.as_str()),
            publisher_continuity_states: tokens(&M5PublisherContinuityState::ALL, |v| v.as_str()),
            lifecycle_states: tokens(&M5CompatibilityLifecycleState::ALL, |v| v.as_str()),
            continuity_presentations: tokens(&M5PublisherContinuityPresentation::ALL, |v| {
                v.as_str()
            }),
            compatibility_label_strip_degrade_reasons: tokens(
                &M5CompatibilityLabelStripDegradeReason::ALL,
                |v| v.as_str(),
            ),
            publisher_continuity_row_degrade_reasons: tokens(
                &M5PublisherContinuityRowDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5CompatibilityContinuityAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5CompatibilityContinuityNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5CompatibilityContinuityExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5MarketplaceInstallConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5CompatibilityContinuityGovernanceReview {
    /// The compatibility-label strip names its compatibility range, host model, and host-version /
    /// manifest-schema range.
    pub strip_names_compatibility_host_and_ranges: bool,
    /// The compatibility-label strip names its lifecycle state and replacement path.
    pub strip_names_lifecycle_and_replacement: bool,
    /// The publisher-continuity row names its continuity presentation and registry source class.
    pub row_names_continuity_and_source_class: bool,
    /// The publisher-continuity row names its transfer history where available.
    pub row_names_transfer_history_where_available: bool,
    /// Deprecated or transferred artifacts carry visible replacement / continuity language.
    pub deprecated_or_transferred_carry_replacement_language: bool,
    /// The registry source class is always explicit, never collapsed into one origin.
    pub source_class_always_explicit_never_collapsed: bool,
    /// An incompatible artifact is never presented as ready to install.
    pub incompatible_never_ready: bool,
    /// Stale or unverifiable evidence never leaves Certified / Supported language in place.
    pub stale_evidence_never_leaves_certified_language: bool,
    /// Compatibility and continuity states stay explicit across listing, detail, install,
    /// diagnostics, and exported views.
    pub states_explicit_across_all_surfaces: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityContinuityConsumerProjection {
    /// Marketplace surfaces consume the shared compatibility / lifecycle vocabulary.
    pub marketplace_surfaces_consume_compatibility_and_lifecycle_vocabulary: bool,
    /// Registry surfaces consume the shared publisher / source-class vocabulary.
    pub registry_surfaces_consume_publisher_and_source_vocabulary: bool,
    /// Compatibility and continuity facts trace back to one canonical component contract.
    pub facts_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical compatibility / continuity source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityContinuityProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityContinuityReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5CompatibilityContinuityControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CompatibilityContinuityControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5CompatibilityContinuityControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CompatibilityContinuityVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CompatibilityContinuityGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CompatibilityContinuityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CompatibilityContinuityProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CompatibilityContinuityReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 compatibility-label-strip / publisher-continuity-row controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityContinuityControlsPacket {
    /// Record kind; must equal [`M5_COMPATIBILITY_CONTINUITY_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COMPATIBILITY_CONTINUITY_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5CompatibilityContinuityControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CompatibilityContinuityVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CompatibilityContinuityGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CompatibilityContinuityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CompatibilityContinuityProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CompatibilityContinuityReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CompatibilityContinuityControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5CompatibilityContinuityControlsPacketInput) -> Self {
        Self {
            record_kind: M5_COMPATIBILITY_CONTINUITY_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_COMPATIBILITY_CONTINUITY_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5CompatibilityContinuityControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COMPATIBILITY_CONTINUITY_CONTROLS_RECORD_KIND {
            violations.push(M5CompatibilityContinuityControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COMPATIBILITY_CONTINUITY_CONTROLS_SCHEMA_VERSION {
            violations.push(M5CompatibilityContinuityControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CompatibilityContinuityControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5CompatibilityContinuityControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 compatibility-continuity controls packet serializes"),
        ) {
            violations.push(M5CompatibilityContinuityControlsViolation::RawMaterialInExport);
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
            .expect("m5 compatibility-continuity controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,strip_examples,row_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .compatibility_label_strip_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.publisher_continuity_row_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.compatibility_label_strip_examples.len(),
                row.publisher_continuity_row_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Compatibility-Label-Strip and Publisher-Continuity-Row Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Registry source classes: {}\n",
            self.vocabulary_set.registry_source_classes.join(", ")
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
                "  - Compatibility-strip examples: {} / publisher-continuity-row examples: {}\n",
                row.compatibility_label_strip_examples.len(),
                row.publisher_continuity_row_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5CompatibilityContinuityControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CompatibilityContinuityControlsViolation>),
}

impl fmt::Display for M5CompatibilityContinuityControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 compatibility-continuity controls export parse failed: {error}"
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
                    "m5 compatibility-continuity controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CompatibilityContinuityControlsArtifactError {}

/// Validation failures emitted by [`M5CompatibilityContinuityControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CompatibilityContinuityControlsViolation {
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
    /// A controls row carries a dishonest clean example (false-ready, stale overclaim, collapse, or
    /// hidden continuity language).
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
    /// Replacement / continuity honesty is not proven: deprecated or transferred artifacts are not
    /// shown carrying visible replacement / continuity language, or no missing-replacement /
    /// hidden-continuity example degrades.
    ReplacementContinuityHonestyNotProven,
    /// Stale-certified overclaim narrowing is not proven: no stale-certified strip or
    /// stale/unverifiable-certified row degrades, or a clean example leaves the overclaim in place.
    StaleCertifiedOverclaimNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5CompatibilityContinuityControlsViolation {
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
            Self::ReplacementContinuityHonestyNotProven => {
                "replacement_continuity_honesty_not_proven"
            }
            Self::StaleCertifiedOverclaimNotProven => "stale_certified_overclaim_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_compatibility_continuity_controls_export(
) -> Result<M5CompatibilityContinuityControlsPacket, M5CompatibilityContinuityControlsArtifactError>
{
    let packet: M5CompatibilityContinuityControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-compatibility-label-strip-publisher-continuity-row-controls-proof/support_export.json"
    )))
    .map_err(M5CompatibilityContinuityControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CompatibilityContinuityControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5CompatibilityContinuityControlsPacket,
    violations: &mut Vec<M5CompatibilityContinuityControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_COMPATIBILITY_CONTINUITY_CONTROLS_SCHEMA_REF,
        M5_COMPATIBILITY_CONTINUITY_CONTROLS_DOC_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
        M5_COMPATIBILITY_LABEL_STRIP_SCHEMA_REF,
        M5_PUBLISHER_CONTINUITY_ROW_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CompatibilityContinuityControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5CompatibilityContinuityControlsPacket,
    violations: &mut Vec<M5CompatibilityContinuityControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5CompatibilityContinuityControlsViolation::NoControlsRows);
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
            violations.push(M5CompatibilityContinuityControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5CompatibilityContinuityControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations
                .push(M5CompatibilityContinuityControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_COMPATIBILITY_LABEL_STRIP_SCHEMA_REF)
            || !refs.contains(M5_PUBLISHER_CONTINUITY_ROW_SCHEMA_REF)
        {
            violations.push(M5CompatibilityContinuityControlsViolation::ComponentSchemaRefMissing);
        }
        if row.compatibility_label_strip_examples.is_empty()
            || row.publisher_continuity_row_examples.is_empty()
        {
            violations.push(M5CompatibilityContinuityControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5CompatibilityContinuityControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5CompatibilityContinuityControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5CompatibilityContinuityControlsPacket,
    violations: &mut Vec<M5CompatibilityContinuityControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.strip_names_compatibility_host_and_ranges,
        review.strip_names_lifecycle_and_replacement,
        review.row_names_continuity_and_source_class,
        review.row_names_transfer_history_where_available,
        review.deprecated_or_transferred_carry_replacement_language,
        review.source_class_always_explicit_never_collapsed,
        review.incompatible_never_ready,
        review.stale_evidence_never_leaves_certified_language,
        review.states_explicit_across_all_surfaces,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5CompatibilityContinuityControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CompatibilityContinuityControlsPacket,
    violations: &mut Vec<M5CompatibilityContinuityControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.marketplace_surfaces_consume_compatibility_and_lifecycle_vocabulary,
        projection.registry_surfaces_consume_publisher_and_source_vocabulary,
        projection.facts_trace_to_single_component_contract,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations
                .push(M5CompatibilityContinuityControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CompatibilityContinuityControlsPacket,
    violations: &mut Vec<M5CompatibilityContinuityControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CompatibilityContinuityControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5CompatibilityContinuityControlsPacket,
    violations: &mut Vec<M5CompatibilityContinuityControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5CompatibilityContinuityControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5CompatibilityContinuityControlsPacket,
    violations: &mut Vec<M5CompatibilityContinuityControlsViolation>,
) {
    let strips = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.compatibility_label_strip_examples.iter())
    };
    let rows = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.publisher_continuity_row_examples.iter())
    };

    // AC: deprecated or transferred artifacts carry visible replacement / continuity language
    // instead of quiet trust carry-forward. A clean strip covers a deprecated / end-of-life / yanked
    // lifecycle carrying a replacement path, a clean row covers a changed publisher carrying
    // continuity language, a missing-replacement strip degrades, a hidden-continuity row degrades,
    // and no clean example carries a deprecated lifecycle without a replacement path or a changed
    // publisher without continuity language.
    let clean_replacement_shown = strips().any(|ex| {
        ex.is_clean() && ex.requires_replacement_path && !ex.replacement_path.trim().is_empty()
    });
    let clean_continuity_shown = rows().any(|ex| {
        ex.is_clean() && ex.publisher_changed && !ex.continuity_language.trim().is_empty()
    });
    let replacement_missing_degrades = strips().any(|ex| {
        ex.degrade_reason == Some(M5CompatibilityLabelStripDegradeReason::ReplacementPathMissing)
    });
    let continuity_hidden_degrades = rows().any(|ex| {
        ex.degrade_reason == Some(M5PublisherContinuityRowDegradeReason::ContinuityLanguageHidden)
    });
    let no_clean_carry_forward = strips().all(|ex| {
        !(ex.is_clean() && ex.requires_replacement_path && ex.replacement_path.trim().is_empty())
    }) && rows()
        .all(|ex| !(ex.is_clean() && ex.hides_continuity_language));
    if !(clean_replacement_shown
        && clean_continuity_shown
        && replacement_missing_degrades
        && continuity_hidden_degrades
        && no_clean_carry_forward)
    {
        violations.push(
            M5CompatibilityContinuityControlsViolation::ReplacementContinuityHonestyNotProven,
        );
    }

    // AC: claim narrowing triggers when compatibility or continuity evidence becomes stale or
    // unverifiable. A stale-certified strip degrades, a stale-or-unverifiable-certified row degrades,
    // and no clean strip or row leaves a Certified / Supported overclaim in place.
    let stale_strip_degrades = strips().any(|ex| {
        ex.degrade_reason
            == Some(M5CompatibilityLabelStripDegradeReason::StaleEvidenceCertifiedOverclaim)
    });
    let stale_row_degrades = rows().any(|ex| {
        ex.degrade_reason
            == Some(M5PublisherContinuityRowDegradeReason::StaleOrUnverifiableCertifiedOverclaim)
    });
    let no_clean_overclaim = strips()
        .all(|ex| !(ex.is_clean() && ex.leaves_stale_certified_overclaim))
        && rows().all(|ex| !(ex.is_clean() && ex.leaves_stale_certified_overclaim));
    if !(stale_strip_degrades && stale_row_degrades && no_clean_overclaim) {
        violations
            .push(M5CompatibilityContinuityControlsViolation::StaleCertifiedOverclaimNotProven);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
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

/// The two component families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5MarketplaceInstallComponentFamily; 2] = [
    M5MarketplaceInstallComponentFamily::CompatibilityLabelStrip,
    M5MarketplaceInstallComponentFamily::PublisherContinuityRow,
];
