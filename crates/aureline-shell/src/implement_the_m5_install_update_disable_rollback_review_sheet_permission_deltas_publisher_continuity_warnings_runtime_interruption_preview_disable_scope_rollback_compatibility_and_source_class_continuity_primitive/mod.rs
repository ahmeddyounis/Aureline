//! Implemented M5 install / update / disable / rollback review-sheet primitive.
//!
//! The frozen [marketplace / install-review component matrix][matrix] names the reusable
//! extension-marketplace UI components and locks their controlled vocabulary. This module is the
//! final implement lane over that matrix: it turns the lifecycle-mutation component — the
//! **install / update / disable / rollback review sheet** — into a resolver that produces an
//! export-safe, honest projection, so a user can read one reviewed transaction grammar with the
//! permission delta, publisher-continuity warning, runtime-interruption preview, disable scope,
//! rollback compatibility, and registry source class before committing a mutation of a contributed
//! artifact rather than discovering the consequence after a disabled or restarted extension
//! surprises them.
//!
//! Three implementation requirements drive the resolver:
//!
//! * **Render install / update / disable / rollback review sheets with permission deltas, publisher
//!   continuity warnings, runtime interruption preview, and disable-scope clarity.**
//!   [`resolve_install_review_sheet`] refuses to read as a clean sheet when the reviewed transaction
//!   grammar is incomplete, when the permission delta is unverified, when a transferred / deprecated
//!   publisher reads as continuous, when the runtime-interruption preview is unresolved, or when a
//!   disable flow leaves its scope unstated; it degrades instead.
//! * **Keep public / mirror / enterprise / side-load source class explicit throughout review.**
//!   The resolver reuses the frozen matrix [`M5RegistrySourceClass`] directly, degrades when the
//!   registry source is unresolved, and never lets a collapsed source class read as a clean sheet,
//!   so the user always knows which registry or artifact origin they are changing.
//! * **Expose rollback compatibility and runtime-interruption consequences before commit.** The
//!   resolver degrades when a rollback flow leaves its rollback compatibility unresolved and never
//!   lets a data-loss or incompatible rollback read as a clean revert, so the consequence is legible
//!   before commit, not after.
//!
//! The resolver reuses the frozen matrix vocabulary directly — the single controlled
//! [`M5MarketplaceInstallDisposition`] marketplace / install-disposition vocabulary, the
//! [`M5RegistrySourceClass`] source vocabulary, the [`M5CompatibilityState`] compatibility
//! vocabulary, the [`M5PermissionPostureState`] permission vocabulary, the
//! [`M5PublisherContinuityState`] continuity vocabulary, the [`M5DisableScopeClass`] disable-scope
//! vocabulary, and the [`M5RollbackCompatibilityState`] rollback vocabulary — so marketplace,
//! extensions, install-review, help, and support surfaces can never fork their own review wording
//! or invent feature-local badges. Raw secret values and private endpoints stay outside the export
//! boundary.
//!
//! [matrix]: crate::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_install_review_sheet_controls,
    seeded_m5_install_review_sheet_controls_install_review_ui_beta_narrowed,
    seeded_m5_install_review_sheet_controls_marketplace_ui_preview_narrowed,
    M5_INSTALL_REVIEW_SHEET_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix::{
    M5CompatibilityState, M5DisableScopeClass, M5MarketplaceInstallAccessibilityRoute,
    M5MarketplaceInstallComponentFamily, M5MarketplaceInstallConsumerSurface,
    M5MarketplaceInstallDeploymentLine, M5MarketplaceInstallDisposition,
    M5MarketplaceInstallDowngradeTrigger, M5MarketplaceInstallQualificationClass,
    M5MarketplaceInstallRequiredLabel, M5PermissionPostureState, M5PublisherContinuityState,
    M5RegistrySourceClass, M5RollbackCompatibilityState,
    M5_INSTALL_UPDATE_DISABLE_ROLLBACK_REVIEW_SHEET_SCHEMA_REF,
    M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF, M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5InstallReviewSheetControlsPacket`].
pub const M5_INSTALL_REVIEW_SHEET_CONTROLS_RECORD_KIND: &str =
    "implement_m5_install_update_disable_rollback_review_sheet_controls";

/// Schema version for M5 install / update / disable / rollback review-sheet controls records.
pub const M5_INSTALL_REVIEW_SHEET_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_INSTALL_REVIEW_SHEET_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-install-update-disable-rollback-review-sheet-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_INSTALL_REVIEW_SHEET_CONTROLS_DOC_REF: &str =
    "docs/marketplace/m5_install_update_disable_rollback_review_sheet_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_INSTALL_REVIEW_SHEET_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-install-update-disable-rollback-review-sheet-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_INSTALL_REVIEW_SHEET_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-install-update-disable-rollback-review-sheet-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_INSTALL_REVIEW_SHEET_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-install-update-disable-rollback-review-sheet-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_INSTALL_REVIEW_SHEET_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-install-update-disable-rollback-review-sheet-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface
/// taxonomy so no lane invents a parallel surface set.
pub type M5InstallReviewSheetConsumerSurface = M5MarketplaceInstallConsumerSurface;

/// Controlled lifecycle-mutation flow a review sheet reviews. The same reviewed transaction grammar
/// covers all four flows so a user never faces a different, opaque mutation dialog per lifecycle
/// action. Minted by this lane because the frozen matrix carries the disable-scope and rollback
/// vocabularies but not the mutation-flow taxonomy the review sheet renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallReviewMutationFlow {
    /// Install a contributed artifact.
    Install,
    /// Update a contributed artifact to a new version.
    Update,
    /// Disable / uninstall a contributed artifact.
    Disable,
    /// Roll a contributed artifact back to a prior version.
    Rollback,
}

impl M5InstallReviewMutationFlow {
    /// Every mutation flow, in declaration order.
    pub const ALL: [Self; 4] = [Self::Install, Self::Update, Self::Disable, Self::Rollback];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Install => "install",
            Self::Update => "update",
            Self::Disable => "disable",
            Self::Rollback => "rollback",
        }
    }

    /// True when this flow must name a disable scope (a disable / uninstall).
    pub const fn reviews_disable_scope(self) -> bool {
        matches!(self, Self::Disable)
    }

    /// True when this flow must name a rollback-compatibility class (an update or rollback, both of
    /// which can be reverted and so must disclose how safely).
    pub const fn reviews_rollback(self) -> bool {
        matches!(self, Self::Update | Self::Rollback)
    }
}

/// Controlled permission-delta class a review sheet renders — the change to the artifact's
/// permission posture the mutation would apply, so permission widening is never hidden behind a
/// compact review dialog. Minted by this lane; it refines the frozen [`M5PermissionPostureState`]
/// into the before / after delta the review sheet shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallReviewPermissionDelta {
    /// No change to the permission posture.
    NoChange,
    /// The mutation widens the permission posture.
    Widened,
    /// The mutation widens the permission posture transitively through dependencies.
    WidenedTransitive,
    /// The mutation narrows the permission posture.
    Narrowed,
    /// The permission posture is restricted by policy.
    PolicyRestricted,
    /// The permission delta cannot currently be verified.
    DeltaUnknown,
}

impl M5InstallReviewPermissionDelta {
    /// Every permission delta, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoChange,
        Self::Widened,
        Self::WidenedTransitive,
        Self::Narrowed,
        Self::PolicyRestricted,
        Self::DeltaUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoChange => "no_change",
            Self::Widened => "widened",
            Self::WidenedTransitive => "widened_transitive",
            Self::Narrowed => "narrowed",
            Self::PolicyRestricted => "policy_restricted",
            Self::DeltaUnknown => "delta_unknown",
        }
    }

    /// True when this delta widens the permission posture.
    pub const fn is_widening(self) -> bool {
        matches!(self, Self::Widened | Self::WidenedTransitive)
    }
}

/// Controlled runtime-interruption class a review sheet previews — what happens to the running
/// artifact and its host when the mutation commits, so a restart or ended session is never a
/// surprise. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallReviewRuntimeInterruption {
    /// No runtime interruption; the mutation applies cleanly.
    NoInterruption,
    /// A reload of the artifact is required.
    ReloadRequired,
    /// A full host restart is required.
    RestartRequired,
    /// Active sessions using the artifact will be ended.
    ActiveSessionsEnded,
    /// In-flight background work will be paused.
    BackgroundWorkPaused,
    /// The runtime-interruption consequence cannot currently be resolved.
    InterruptionUnknown,
}

impl M5InstallReviewRuntimeInterruption {
    /// Every runtime-interruption class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoInterruption,
        Self::ReloadRequired,
        Self::RestartRequired,
        Self::ActiveSessionsEnded,
        Self::BackgroundWorkPaused,
        Self::InterruptionUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoInterruption => "no_interruption",
            Self::ReloadRequired => "reload_required",
            Self::RestartRequired => "restart_required",
            Self::ActiveSessionsEnded => "active_sessions_ended",
            Self::BackgroundWorkPaused => "background_work_paused",
            Self::InterruptionUnknown => "interruption_unknown",
        }
    }
}

/// Controlled review action a review sheet offers. The reviewed transaction grammar requires the
/// review, confirm, and cancel actions to be present together so a mutation is always reviewed and
/// reversible rather than one-click opaque. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallReviewAction {
    /// Review the full mutation transaction (anchor).
    ReviewTransaction,
    /// Confirm and commit the mutation.
    ConfirmMutation,
    /// Cancel the mutation.
    CancelMutation,
    /// Inspect the permission delta.
    InspectPermissionDelta,
    /// Inspect the runtime-interruption preview.
    InspectRuntimeInterruption,
    /// Inspect the disable scope or rollback compatibility.
    InspectDisableOrRollbackScope,
}

impl M5InstallReviewAction {
    /// Every review action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewTransaction,
        Self::ConfirmMutation,
        Self::CancelMutation,
        Self::InspectPermissionDelta,
        Self::InspectRuntimeInterruption,
        Self::InspectDisableOrRollbackScope,
    ];

    /// The three actions the reviewed transaction grammar always requires together.
    pub const REVIEWED_GRAMMAR: [Self; 3] = [
        Self::ReviewTransaction,
        Self::ConfirmMutation,
        Self::CancelMutation,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewTransaction => "review_transaction",
            Self::ConfirmMutation => "confirm_mutation",
            Self::CancelMutation => "cancel_mutation",
            Self::InspectPermissionDelta => "inspect_permission_delta",
            Self::InspectRuntimeInterruption => "inspect_runtime_interruption",
            Self::InspectDisableOrRollbackScope => "inspect_disable_or_rollback_scope",
        }
    }
}

/// One mandatory rendered part an install / update / disable / rollback review sheet must be able to
/// show, so no source, permission, continuity, interruption, disable-scope, or rollback fact is left
/// implicit behind a compact review dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallReviewAnatomyPart {
    /// The sheet's stable identity / what artifact it mutates.
    Identity,
    /// The sheet's current typed state.
    State,
    /// The non-visual keyboard route to the sheet.
    KeyboardRoute,
    /// The lifecycle-mutation flow (install / update / disable / rollback).
    MutationFlow,
    /// The registry source class of the artifact being changed.
    RegistrySourceClass,
    /// The compatibility state of the artifact.
    CompatibilityState,
    /// The permission delta the mutation applies.
    PermissionDelta,
    /// The publisher-continuity warning where the publisher changed.
    PublisherContinuityWarning,
    /// The runtime-interruption preview before commit.
    RuntimeInterruptionPreview,
    /// The disable scope (disable / uninstall flow).
    DisableScope,
    /// The rollback-compatibility class (update / rollback flow).
    RollbackCompatibility,
    /// The one reviewed transaction grammar shared across flows.
    TransactionGrammar,
    /// The review / confirm / cancel action set.
    ReviewActions,
    /// The evidence-freshness disclosure.
    EvidenceFreshness,
}

impl M5InstallReviewAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::MutationFlow,
        Self::RegistrySourceClass,
        Self::CompatibilityState,
        Self::PermissionDelta,
        Self::PublisherContinuityWarning,
        Self::RuntimeInterruptionPreview,
        Self::DisableScope,
        Self::RollbackCompatibility,
        Self::TransactionGrammar,
        Self::ReviewActions,
        Self::EvidenceFreshness,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::MutationFlow => "mutation_flow",
            Self::RegistrySourceClass => "registry_source_class",
            Self::CompatibilityState => "compatibility_state",
            Self::PermissionDelta => "permission_delta",
            Self::PublisherContinuityWarning => "publisher_continuity_warning",
            Self::RuntimeInterruptionPreview => "runtime_interruption_preview",
            Self::DisableScope => "disable_scope",
            Self::RollbackCompatibility => "rollback_compatibility",
            Self::TransactionGrammar => "transaction_grammar",
            Self::ReviewActions => "review_actions",
            Self::EvidenceFreshness => "evidence_freshness",
        }
    }
}

/// Next safe action a review sheet surfaces so a user is never left without a route to review the
/// fact behind a degraded sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallReviewNextAction {
    /// Review the full mutation transaction.
    ReviewTransaction,
    /// Review the permission delta.
    ReviewPermissionDelta,
    /// Review the runtime-interruption preview.
    ReviewRuntimeInterruption,
    /// Review the disable scope or rollback compatibility.
    ReviewDisableOrRollbackScope,
    /// Review the registry source class and publisher continuity.
    ReviewSourceContinuity,
    /// Review the evidence freshness for a stale signal.
    ReviewEvidenceFreshness,
    /// No action is needed; the sheet is clean.
    NoActionNeeded,
}

impl M5InstallReviewNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ReviewTransaction,
        Self::ReviewPermissionDelta,
        Self::ReviewRuntimeInterruption,
        Self::ReviewDisableOrRollbackScope,
        Self::ReviewSourceContinuity,
        Self::ReviewEvidenceFreshness,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewTransaction => "review_transaction",
            Self::ReviewPermissionDelta => "review_permission_delta",
            Self::ReviewRuntimeInterruption => "review_runtime_interruption",
            Self::ReviewDisableOrRollbackScope => "review_disable_or_rollback_scope",
            Self::ReviewSourceContinuity => "review_source_continuity",
            Self::ReviewEvidenceFreshness => "review_evidence_freshness",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallReviewExportField {
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
    /// The lifecycle-mutation flows reviewed.
    MutationFlow,
    /// The registry source class named by the sheet.
    RegistrySourceClass,
    /// The permission delta named by the sheet.
    PermissionDelta,
    /// The runtime-interruption preview named by the sheet.
    RuntimeInterruption,
    /// The disable scope named by the sheet.
    DisableScope,
    /// The rollback compatibility named by the sheet.
    RollbackCompatibility,
}

impl M5InstallReviewExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::MutationFlow,
        Self::RegistrySourceClass,
        Self::PermissionDelta,
        Self::RuntimeInterruption,
        Self::DisableScope,
        Self::RollbackCompatibility,
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
            Self::MutationFlow => "mutation_flow",
            Self::RegistrySourceClass => "registry_source_class",
            Self::PermissionDelta => "permission_delta",
            Self::RuntimeInterruption => "runtime_interruption",
            Self::DisableScope => "disable_scope",
            Self::RollbackCompatibility => "rollback_compatibility",
        }
    }
}

/// Reason an install / update / disable / rollback review sheet degraded below a clean, fully-legible
/// state. The degrade-first ladder returns one of these instead of ever letting an ambiguous sheet
/// read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallReviewSheetDegradeReason {
    /// The artifact identity is unstated.
    ArtifactIdentityUnstated,
    /// The registry source class cannot currently be resolved.
    RegistrySourceUnresolved,
    /// The registry source class is collapsed across public / mirrored / enterprise.
    RegistrySourceClassCollapsed,
    /// The reviewed transaction grammar (review / confirm / cancel) is incomplete.
    TransactionGrammarIncomplete,
    /// The permission delta could not be verified.
    PermissionDeltaUnverified,
    /// An incompatible artifact reads as ready to mutate.
    IncompatibleShownReady,
    /// A transferred / deprecated publisher reads as continuous with no warning.
    PublisherContinuityWarningMissing,
    /// The runtime-interruption preview cannot currently be resolved.
    RuntimeInterruptionUnresolved,
    /// A disable flow leaves its disable scope unstated.
    DisableScopeUnstated,
    /// A rollback flow leaves its rollback-compatibility class unresolved.
    RollbackCompatibilityUnresolved,
    /// A data-loss / incompatible rollback reads as a clean revert.
    RollbackIncompatibilityHidden,
    /// Certified / Supported language is left in place while the evidence is stale.
    StaleEvidenceCertifiedOverclaim,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5InstallReviewSheetDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::ArtifactIdentityUnstated,
        Self::RegistrySourceUnresolved,
        Self::RegistrySourceClassCollapsed,
        Self::TransactionGrammarIncomplete,
        Self::PermissionDeltaUnverified,
        Self::IncompatibleShownReady,
        Self::PublisherContinuityWarningMissing,
        Self::RuntimeInterruptionUnresolved,
        Self::DisableScopeUnstated,
        Self::RollbackCompatibilityUnresolved,
        Self::RollbackIncompatibilityHidden,
        Self::StaleEvidenceCertifiedOverclaim,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactIdentityUnstated => "artifact_identity_unstated",
            Self::RegistrySourceUnresolved => "registry_source_unresolved",
            Self::RegistrySourceClassCollapsed => "registry_source_class_collapsed",
            Self::TransactionGrammarIncomplete => "transaction_grammar_incomplete",
            Self::PermissionDeltaUnverified => "permission_delta_unverified",
            Self::IncompatibleShownReady => "incompatible_shown_ready",
            Self::PublisherContinuityWarningMissing => "publisher_continuity_warning_missing",
            Self::RuntimeInterruptionUnresolved => "runtime_interruption_unresolved",
            Self::DisableScopeUnstated => "disable_scope_unstated",
            Self::RollbackCompatibilityUnresolved => "rollback_compatibility_unresolved",
            Self::RollbackIncompatibilityHidden => "rollback_incompatibility_hidden",
            Self::StaleEvidenceCertifiedOverclaim => "stale_evidence_certified_overclaim",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5InstallReviewNextAction {
        match self {
            Self::ArtifactIdentityUnstated | Self::TransactionGrammarIncomplete => {
                M5InstallReviewNextAction::ReviewTransaction
            }
            Self::RegistrySourceUnresolved
            | Self::RegistrySourceClassCollapsed
            | Self::PublisherContinuityWarningMissing => {
                M5InstallReviewNextAction::ReviewSourceContinuity
            }
            Self::PermissionDeltaUnverified => M5InstallReviewNextAction::ReviewPermissionDelta,
            Self::IncompatibleShownReady | Self::RuntimeInterruptionUnresolved => {
                M5InstallReviewNextAction::ReviewRuntimeInterruption
            }
            Self::DisableScopeUnstated
            | Self::RollbackCompatibilityUnresolved
            | Self::RollbackIncompatibilityHidden => {
                M5InstallReviewNextAction::ReviewDisableOrRollbackScope
            }
            Self::StaleEvidenceCertifiedOverclaim | Self::ProofStale => {
                M5InstallReviewNextAction::ReviewEvidenceFreshness
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5MarketplaceInstallDowngradeTrigger {
        match self {
            Self::ArtifactIdentityUnstated
            | Self::TransactionGrammarIncomplete
            | Self::RuntimeInterruptionUnresolved => {
                M5MarketplaceInstallDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::RegistrySourceUnresolved | Self::RegistrySourceClassCollapsed => {
                M5MarketplaceInstallDowngradeTrigger::RegistrySourceClassCollapsed
            }
            Self::PermissionDeltaUnverified => {
                M5MarketplaceInstallDowngradeTrigger::PermissionWideningHidden
            }
            Self::IncompatibleShownReady => {
                M5MarketplaceInstallDowngradeTrigger::CompatibilityRangeUnstated
            }
            Self::PublisherContinuityWarningMissing => {
                M5MarketplaceInstallDowngradeTrigger::PublisherTransferHidden
            }
            Self::DisableScopeUnstated => {
                M5MarketplaceInstallDowngradeTrigger::DisableScopeUnstated
            }
            Self::RollbackCompatibilityUnresolved | Self::RollbackIncompatibilityHidden => {
                M5MarketplaceInstallDowngradeTrigger::RollbackIncompatibilityHidden
            }
            Self::StaleEvidenceCertifiedOverclaim | Self::ProofStale => {
                M5MarketplaceInstallDowngradeTrigger::ProofStale
            }
        }
    }
}

/// True when the compatibility state blocks a clean install / mutation.
fn compat_is_blocking(state: M5CompatibilityState) -> bool {
    matches!(
        state,
        M5CompatibilityState::Incompatible
            | M5CompatibilityState::DegradedHost
            | M5CompatibilityState::UnsupportedRuntime
    )
}

/// True when the publisher continuity means the publishing account changed hands or lapsed.
fn publisher_changed(state: M5PublisherContinuityState) -> bool {
    matches!(
        state,
        M5PublisherContinuityState::Transferred
            | M5PublisherContinuityState::Deprecated
            | M5PublisherContinuityState::Abandoned
    )
}

/// True when the rollback-compatibility state carries a real reversal risk.
fn rollback_is_risky(state: M5RollbackCompatibilityState) -> bool {
    matches!(
        state,
        M5RollbackCompatibilityState::RollbackIncompatible
            | M5RollbackCompatibilityState::RollbackDataLoss
            | M5RollbackCompatibilityState::NoPriorVersion
    )
}

/// Input to [`resolve_install_review_sheet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5InstallReviewSheetResolutionInput {
    /// Stable identity of the sheet instance.
    pub sheet_id: String,
    /// The artifact identity (name / id) shown; empty means unstated.
    pub artifact_identity: String,
    /// The lifecycle-mutation flow this sheet reviews.
    pub mutation_flow: M5InstallReviewMutationFlow,
    /// The registry source class of the artifact being changed.
    pub registry_source: M5RegistrySourceClass,
    /// The compatibility state of the artifact.
    pub compatibility: M5CompatibilityState,
    /// The permission delta the mutation applies.
    pub permission_delta: M5InstallReviewPermissionDelta,
    /// The publisher-continuity state of the artifact.
    pub publisher_continuity: M5PublisherContinuityState,
    /// The runtime-interruption consequence of the mutation.
    pub runtime_interruption: M5InstallReviewRuntimeInterruption,
    /// The disable scope, where the flow is a disable / uninstall.
    pub disable_scope: Option<M5DisableScopeClass>,
    /// The rollback-compatibility class, where the flow is an update / rollback.
    pub rollback_compatibility: Option<M5RollbackCompatibilityState>,
    /// The review actions the sheet offers.
    pub review_actions: Vec<M5InstallReviewAction>,
    /// True when the sheet carries Certified / Supported language.
    pub certified_or_supported_claimed: bool,
    /// True when the underlying review evidence is current.
    pub evidence_fresh: bool,
    /// True when the sheet reads an incompatible artifact as ready to mutate.
    pub reads_incompatible_as_ready: bool,
    /// True when the sheet reads a transferred / deprecated publisher as continuous.
    pub reads_transferred_as_continuous: bool,
    /// True when the sheet collapses the registry source class across public / mirrored / enterprise.
    pub collapses_source_class: bool,
    /// True when the sheet reads a data-loss / incompatible rollback as a clean revert.
    pub reads_rollback_as_clean: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe install / update / disable / rollback review-sheet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedInstallReviewSheet {
    /// Stable identity of the sheet instance.
    pub sheet_id: String,
    /// The artifact identity named by the sheet.
    pub artifact_identity: String,
    /// The lifecycle-mutation flow token named by the sheet.
    pub mutation_flow: String,
    /// The registry source-class token named by the sheet.
    pub registry_source: String,
    /// The compatibility token named by the sheet.
    pub compatibility: String,
    /// The permission-delta token named by the sheet.
    pub permission_delta: String,
    /// The publisher-continuity token named by the sheet.
    pub publisher_continuity: String,
    /// The runtime-interruption token named by the sheet.
    pub runtime_interruption: String,
    /// The disable-scope token named by the sheet, or `null` when not applicable.
    pub disable_scope: Option<String>,
    /// The rollback-compatibility token named by the sheet, or `null` when not applicable.
    pub rollback_compatibility: Option<String>,
    /// The review-action tokens named by the sheet.
    pub review_actions: Vec<String>,
    /// Whether this flow reviews a disable scope.
    pub reviews_disable_scope: bool,
    /// Whether this flow reviews a rollback-compatibility class.
    pub reviews_rollback: bool,
    /// Whether the reviewed transaction grammar (review / confirm / cancel) is present in full.
    pub has_transaction_grammar: bool,
    /// Whether the sheet names an explicit disable scope where the flow requires one.
    pub names_disable_scope: bool,
    /// Whether the sheet names an explicit rollback compatibility where the flow requires one.
    pub names_rollback_compatibility: bool,
    /// Whether Certified / Supported language is claimed.
    pub certified_or_supported_claimed: bool,
    /// Whether the underlying evidence is current.
    pub evidence_fresh: bool,
    /// Guardrail (MUST be `false` on a clean sheet): an incompatible artifact reads as ready.
    pub presents_incompatible_as_ready: bool,
    /// Guardrail (MUST be `false` on a clean sheet): a transferred publisher reads as continuous.
    pub hides_publisher_transfer: bool,
    /// Guardrail (MUST be `false` on a clean sheet): the registry source class is collapsed.
    pub collapses_source_class: bool,
    /// Guardrail (MUST be `false` on a clean sheet): a disable flow hides its disable scope.
    pub hides_disable_scope: bool,
    /// Guardrail (MUST be `false` on a clean sheet): a risky rollback reads as a clean revert.
    pub hides_rollback_incompatibility: bool,
    /// Guardrail (MUST be `false` on a clean sheet): stale evidence leaves a Certified / Supported
    /// overclaim in place.
    pub leaves_stale_certified_overclaim: bool,
    /// Degrade reason, if the sheet could not read as a clean state.
    pub degrade_reason: Option<M5InstallReviewSheetDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5InstallReviewNextAction,
    /// Whether the review facts are legible in full (clean sheet naming every fact).
    pub fully_legible: bool,
}

impl M5ResolvedInstallReviewSheet {
    /// Whether this sheet reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5InstallReviewSheetResolutionError {
    /// The sheet id was empty.
    EmptySheetId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5InstallReviewSheetResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptySheetId => "empty_sheet_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5InstallReviewSheetResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 install-review-sheet resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5InstallReviewSheetResolutionError {}

/// Resolves an install / update / disable / rollback review sheet, keeping the lifecycle mutation
/// reviewed and reversible: the sheet names its mutation flow, registry source class, compatibility,
/// permission delta, publisher-continuity warning, runtime-interruption preview, disable scope, and
/// rollback compatibility under one reviewed transaction grammar; it never reads an incompatible
/// artifact as ready, never presents a transferred publisher as continuous, never leaves a disable
/// flow's scope or a rollback flow's compatibility unstated, never reads a risky rollback as a clean
/// revert, and narrows the claim the moment Certified / Supported evidence goes stale.
pub fn resolve_install_review_sheet(
    input: M5InstallReviewSheetResolutionInput,
) -> Result<M5ResolvedInstallReviewSheet, M5InstallReviewSheetResolutionError> {
    if input.sheet_id.trim().is_empty() {
        return Err(M5InstallReviewSheetResolutionError::EmptySheetId);
    }
    if string_is_forbidden(&input.sheet_id) || string_is_forbidden(&input.artifact_identity) {
        return Err(M5InstallReviewSheetResolutionError::ForbiddenMaterial);
    }

    let reviews_disable_scope = input.mutation_flow.reviews_disable_scope();
    let reviews_rollback = input.mutation_flow.reviews_rollback();

    let action_set: BTreeSet<M5InstallReviewAction> =
        input.review_actions.iter().copied().collect();
    let has_transaction_grammar = M5InstallReviewAction::REVIEWED_GRAMMAR
        .iter()
        .all(|a| action_set.contains(a));

    let disable_scope_named = matches!(
        input.disable_scope,
        Some(scope) if scope != M5DisableScopeClass::ScopeUnknown
    );
    let names_disable_scope = reviews_disable_scope && disable_scope_named;
    let rollback_named = matches!(
        input.rollback_compatibility,
        Some(state) if state != M5RollbackCompatibilityState::RollbackUnknown
    );
    let names_rollback_compatibility = reviews_rollback && rollback_named;

    let presents_incompatible_as_ready =
        compat_is_blocking(input.compatibility) && input.reads_incompatible_as_ready;
    let hides_publisher_transfer =
        publisher_changed(input.publisher_continuity) && input.reads_transferred_as_continuous;
    let collapses_source_class = input.collapses_source_class;
    let hides_disable_scope = reviews_disable_scope && !disable_scope_named;
    let rollback_risky =
        reviews_rollback && input.rollback_compatibility.is_some_and(rollback_is_risky);
    let hides_rollback_incompatibility = rollback_risky && input.reads_rollback_as_clean;
    let leaves_stale_certified_overclaim =
        input.certified_or_supported_claimed && !input.evidence_fresh;

    let degrade_reason = if input.artifact_identity.trim().is_empty() {
        Some(M5InstallReviewSheetDegradeReason::ArtifactIdentityUnstated)
    } else if matches!(input.registry_source, M5RegistrySourceClass::SourceUnknown) {
        Some(M5InstallReviewSheetDegradeReason::RegistrySourceUnresolved)
    } else if collapses_source_class {
        Some(M5InstallReviewSheetDegradeReason::RegistrySourceClassCollapsed)
    } else if !has_transaction_grammar {
        Some(M5InstallReviewSheetDegradeReason::TransactionGrammarIncomplete)
    } else if matches!(
        input.permission_delta,
        M5InstallReviewPermissionDelta::DeltaUnknown
    ) {
        Some(M5InstallReviewSheetDegradeReason::PermissionDeltaUnverified)
    } else if presents_incompatible_as_ready {
        Some(M5InstallReviewSheetDegradeReason::IncompatibleShownReady)
    } else if hides_publisher_transfer {
        Some(M5InstallReviewSheetDegradeReason::PublisherContinuityWarningMissing)
    } else if matches!(
        input.runtime_interruption,
        M5InstallReviewRuntimeInterruption::InterruptionUnknown
    ) {
        Some(M5InstallReviewSheetDegradeReason::RuntimeInterruptionUnresolved)
    } else if hides_disable_scope {
        Some(M5InstallReviewSheetDegradeReason::DisableScopeUnstated)
    } else if reviews_rollback && !rollback_named {
        Some(M5InstallReviewSheetDegradeReason::RollbackCompatibilityUnresolved)
    } else if hides_rollback_incompatibility {
        Some(M5InstallReviewSheetDegradeReason::RollbackIncompatibilityHidden)
    } else if leaves_stale_certified_overclaim {
        Some(M5InstallReviewSheetDegradeReason::StaleEvidenceCertifiedOverclaim)
    } else if !input.proof_fresh {
        Some(M5InstallReviewSheetDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5InstallReviewNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedInstallReviewSheet {
        sheet_id: input.sheet_id,
        artifact_identity: input.artifact_identity,
        mutation_flow: input.mutation_flow.as_str().to_owned(),
        registry_source: input.registry_source.as_str().to_owned(),
        compatibility: input.compatibility.as_str().to_owned(),
        permission_delta: input.permission_delta.as_str().to_owned(),
        publisher_continuity: input.publisher_continuity.as_str().to_owned(),
        runtime_interruption: input.runtime_interruption.as_str().to_owned(),
        disable_scope: input.disable_scope.map(|s| s.as_str().to_owned()),
        rollback_compatibility: input.rollback_compatibility.map(|s| s.as_str().to_owned()),
        review_actions: input
            .review_actions
            .iter()
            .map(|a| a.as_str().to_owned())
            .collect(),
        reviews_disable_scope,
        reviews_rollback,
        has_transaction_grammar,
        names_disable_scope,
        names_rollback_compatibility,
        certified_or_supported_claimed: input.certified_or_supported_claimed,
        evidence_fresh: input.evidence_fresh,
        presents_incompatible_as_ready,
        hides_publisher_transfer,
        collapses_source_class,
        hides_disable_scope,
        hides_rollback_incompatibility,
        leaves_stale_certified_overclaim,
        degrade_reason,
        next_action,
        fully_legible: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved install / update / disable / rollback
/// review-sheet examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallReviewSheetControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5InstallReviewSheetConsumerSurface,
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
    pub anatomy_parts: Vec<M5InstallReviewAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5InstallReviewExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5MarketplaceInstallDowngradeTrigger>,
    /// Resolved review-sheet examples.
    pub review_sheet_examples: Vec<M5ResolvedInstallReviewSheet>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the review-sheet component schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hide a permission delta or a runtime-interruption consequence.
    pub hides_permission_delta_or_runtime_interruption: bool,
    /// Hard invariant: never hide a publisher transfer, a disable scope, or a rollback
    /// incompatibility.
    pub hides_publisher_transfer_disable_scope_or_rollback_incompatibility: bool,
    /// Hard invariant: never collapse the registry source class across public / mirrored / enterprise.
    pub collapses_registry_source_class_across_public_mirrored_enterprise: bool,
    /// Hard invariant: never present an incompatible or over-budget artifact as ready to mutate.
    pub presents_incompatible_or_over_budget_as_ready: bool,
}

impl M5InstallReviewSheetControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5InstallReviewAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5InstallReviewAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5InstallReviewExportField> =
            self.export_fields.iter().copied().collect();
        M5InstallReviewExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.hides_permission_delta_or_runtime_interruption
            && !self.hides_publisher_transfer_disable_scope_or_rollback_incompatibility
            && !self.collapses_registry_source_class_across_public_mirrored_enterprise
            && !self.presents_incompatible_or_over_budget_as_ready
    }

    /// True when every resolved example on this row is honest: no clean sheet presents an
    /// incompatible artifact as ready, hides a publisher transfer, collapses the source class, hides
    /// a disable scope, hides a rollback incompatibility, or leaves a stale-certified overclaim.
    fn examples_are_honest(&self) -> bool {
        self.review_sheet_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.presents_incompatible_as_ready
                    || ex.hides_publisher_transfer
                    || ex.collapses_source_class
                    || ex.hides_disable_scope
                    || ex.hides_rollback_incompatibility
                    || ex.leaves_stale_certified_overclaim))
        })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallReviewSheetVocabularySet {
    /// Marketplace / install-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Registry source-class tokens (bound from the frozen matrix).
    pub registry_source_classes: Vec<String>,
    /// Compatibility-state tokens (bound from the frozen matrix).
    pub compatibility_states: Vec<String>,
    /// Permission-posture tokens (bound from the frozen matrix).
    pub permission_postures: Vec<String>,
    /// Publisher-continuity tokens (bound from the frozen matrix).
    pub publisher_continuity_states: Vec<String>,
    /// Disable-scope tokens (bound from the frozen matrix).
    pub disable_scope_classes: Vec<String>,
    /// Rollback-compatibility tokens (bound from the frozen matrix).
    pub rollback_compatibility_states: Vec<String>,
    /// Mutation-flow tokens (minted by this lane).
    pub mutation_flows: Vec<String>,
    /// Permission-delta tokens (minted by this lane).
    pub permission_deltas: Vec<String>,
    /// Runtime-interruption tokens (minted by this lane).
    pub runtime_interruptions: Vec<String>,
    /// Review-action tokens (minted by this lane).
    pub review_actions: Vec<String>,
    /// Review-sheet degrade-reason tokens.
    pub review_sheet_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5InstallReviewSheetVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5MarketplaceInstallDisposition::ALL, |v| v.as_str()),
            registry_source_classes: tokens(&M5RegistrySourceClass::ALL, |v| v.as_str()),
            compatibility_states: tokens(&M5CompatibilityState::ALL, |v| v.as_str()),
            permission_postures: tokens(&M5PermissionPostureState::ALL, |v| v.as_str()),
            publisher_continuity_states: tokens(&M5PublisherContinuityState::ALL, |v| v.as_str()),
            disable_scope_classes: tokens(&M5DisableScopeClass::ALL, |v| v.as_str()),
            rollback_compatibility_states: tokens(&M5RollbackCompatibilityState::ALL, |v| {
                v.as_str()
            }),
            mutation_flows: tokens(&M5InstallReviewMutationFlow::ALL, |v| v.as_str()),
            permission_deltas: tokens(&M5InstallReviewPermissionDelta::ALL, |v| v.as_str()),
            runtime_interruptions: tokens(&M5InstallReviewRuntimeInterruption::ALL, |v| v.as_str()),
            review_actions: tokens(&M5InstallReviewAction::ALL, |v| v.as_str()),
            review_sheet_degrade_reasons: tokens(&M5InstallReviewSheetDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5InstallReviewAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5InstallReviewNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5InstallReviewExportField::ALL, |v| v.as_str()),
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
pub struct M5InstallReviewSheetGovernanceReview {
    /// The review sheet renders one reviewed transaction grammar across install / update / disable /
    /// rollback.
    pub one_reviewed_transaction_grammar_across_flows: bool,
    /// The review sheet names the permission delta the mutation applies.
    pub names_permission_delta: bool,
    /// The review sheet warns when the publisher transferred / deprecated / abandoned.
    pub warns_on_publisher_continuity_change: bool,
    /// The review sheet previews the runtime-interruption consequence before commit.
    pub previews_runtime_interruption_before_commit: bool,
    /// A disable flow always names its disable scope; a workspace disable never reads as global.
    pub disable_scope_always_explicit: bool,
    /// A rollback flow always names its rollback compatibility; a risky rollback never reads clean.
    pub rollback_compatibility_always_explicit: bool,
    /// The registry source class stays explicit across public / mirrored / enterprise / side-load.
    pub source_class_always_explicit: bool,
    /// An incompatible artifact is never presented as ready to mutate.
    pub incompatible_never_shown_ready: bool,
    /// Stale evidence never leaves Certified / Supported language in place.
    pub stale_evidence_never_leaves_certified_language: bool,
    /// Registry / source continuity remains visible from review through help / support / export.
    pub source_continuity_visible_through_handoff: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallReviewSheetConsumerProjection {
    /// Marketplace / install surfaces consume the shared review-sheet vocabulary.
    pub install_surfaces_consume_review_vocabulary: bool,
    /// Disable / rollback truth traces back to one canonical component contract.
    pub disable_rollback_traces_to_single_contract: bool,
    /// Registry / source continuity is carried into help / support / export handoff.
    pub source_continuity_carried_into_handoff: bool,
    /// Support / export reads a single canonical review-sheet source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallReviewSheetProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallReviewSheetReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5InstallReviewSheetControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5InstallReviewSheetControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5InstallReviewSheetControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5InstallReviewSheetVocabularySet,
    /// Governance-review block.
    pub governance_review: M5InstallReviewSheetGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5InstallReviewSheetConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5InstallReviewSheetProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5InstallReviewSheetReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 install / update / disable / rollback review-sheet controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallReviewSheetControlsPacket {
    /// Record kind; must equal [`M5_INSTALL_REVIEW_SHEET_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_INSTALL_REVIEW_SHEET_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5InstallReviewSheetControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5InstallReviewSheetVocabularySet,
    /// Governance-review block.
    pub governance_review: M5InstallReviewSheetGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5InstallReviewSheetConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5InstallReviewSheetProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5InstallReviewSheetReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5InstallReviewSheetControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5InstallReviewSheetControlsPacketInput) -> Self {
        Self {
            record_kind: M5_INSTALL_REVIEW_SHEET_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_INSTALL_REVIEW_SHEET_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5InstallReviewSheetControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_INSTALL_REVIEW_SHEET_CONTROLS_RECORD_KIND {
            violations.push(M5InstallReviewSheetControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_INSTALL_REVIEW_SHEET_CONTROLS_SCHEMA_VERSION {
            violations.push(M5InstallReviewSheetControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5InstallReviewSheetControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5InstallReviewSheetControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 install-review-sheet controls packet serializes"),
        ) {
            violations.push(M5InstallReviewSheetControlsViolation::RawMaterialInExport);
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
            .expect("m5 install-review-sheet controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str("consumer_surface,qualification,owner,sheet_examples,degrade_reasons,downgrade_triggers\n");
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .review_sheet_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.review_sheet_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Install / Update / Disable / Rollback Review-Sheet Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Mutation flows: {}\n",
            self.vocabulary_set.mutation_flows.join(", ")
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
                "  - Review-sheet examples: {}\n",
                row.review_sheet_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5InstallReviewSheetControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5InstallReviewSheetControlsViolation>),
}

impl fmt::Display for M5InstallReviewSheetControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 install-review-sheet controls export parse failed: {error}"
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
                    "m5 install-review-sheet controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5InstallReviewSheetControlsArtifactError {}

/// Validation failures emitted by [`M5InstallReviewSheetControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5InstallReviewSheetControlsViolation {
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
    /// A controls row does not point at the review-sheet component schema.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (incompatible-ready, hidden transfer,
    /// collapsed source, hidden disable scope, hidden rollback, or stale overclaim).
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
    /// The one-reviewed-transaction-grammar acceptance criterion is not proven: no clean sheet
    /// carries the reviewed grammar for one of the four flows, no grammar-incomplete sheet degrades,
    /// or a clean sheet lacks the grammar.
    TransactionGrammarNotProven,
    /// The disable-scope-and-rollback-truth acceptance criterion is not proven: no clean sheet names
    /// a disable scope or rollback compatibility, no disable-scope-unstated or rollback-truth sheet
    /// degrades, or a clean sheet hides a disable scope or rollback incompatibility.
    DisableScopeAndRollbackTruthNotProven,
    /// The source-continuity acceptance criterion is not proven: no clean sheet names its source
    /// class and publisher continuity, no source-collapsed or transfer-hidden sheet degrades, or a
    /// clean sheet collapses the source class or hides a publisher transfer.
    SourceContinuityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5InstallReviewSheetControlsViolation {
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
            Self::TransactionGrammarNotProven => "transaction_grammar_not_proven",
            Self::DisableScopeAndRollbackTruthNotProven => {
                "disable_scope_and_rollback_truth_not_proven"
            }
            Self::SourceContinuityNotProven => "source_continuity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_install_review_sheet_controls_export(
) -> Result<M5InstallReviewSheetControlsPacket, M5InstallReviewSheetControlsArtifactError> {
    let packet: M5InstallReviewSheetControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-install-update-disable-rollback-review-sheet-controls-proof/support_export.json"
    )))
    .map_err(M5InstallReviewSheetControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5InstallReviewSheetControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5InstallReviewSheetControlsPacket,
    violations: &mut Vec<M5InstallReviewSheetControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_INSTALL_REVIEW_SHEET_CONTROLS_SCHEMA_REF,
        M5_INSTALL_REVIEW_SHEET_CONTROLS_DOC_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
        M5_INSTALL_UPDATE_DISABLE_ROLLBACK_REVIEW_SHEET_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5InstallReviewSheetControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5InstallReviewSheetControlsPacket,
    violations: &mut Vec<M5InstallReviewSheetControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5InstallReviewSheetControlsViolation::NoControlsRows);
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
            violations.push(M5InstallReviewSheetControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5InstallReviewSheetControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5InstallReviewSheetControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_INSTALL_UPDATE_DISABLE_ROLLBACK_REVIEW_SHEET_SCHEMA_REF) {
            violations.push(M5InstallReviewSheetControlsViolation::ComponentSchemaRefMissing);
        }
        if row.review_sheet_examples.is_empty() {
            violations.push(M5InstallReviewSheetControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5InstallReviewSheetControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5InstallReviewSheetControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5InstallReviewSheetControlsPacket,
    violations: &mut Vec<M5InstallReviewSheetControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_reviewed_transaction_grammar_across_flows,
        review.names_permission_delta,
        review.warns_on_publisher_continuity_change,
        review.previews_runtime_interruption_before_commit,
        review.disable_scope_always_explicit,
        review.rollback_compatibility_always_explicit,
        review.source_class_always_explicit,
        review.incompatible_never_shown_ready,
        review.stale_evidence_never_leaves_certified_language,
        review.source_continuity_visible_through_handoff,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5InstallReviewSheetControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5InstallReviewSheetControlsPacket,
    violations: &mut Vec<M5InstallReviewSheetControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.install_surfaces_consume_review_vocabulary,
        projection.disable_rollback_traces_to_single_contract,
        projection.source_continuity_carried_into_handoff,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5InstallReviewSheetControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5InstallReviewSheetControlsPacket,
    violations: &mut Vec<M5InstallReviewSheetControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5InstallReviewSheetControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5InstallReviewSheetControlsPacket,
    violations: &mut Vec<M5InstallReviewSheetControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5InstallReviewSheetControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5InstallReviewSheetControlsPacket,
    violations: &mut Vec<M5InstallReviewSheetControlsViolation>,
) {
    let sheets = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.review_sheet_examples.iter())
    };

    // AC1: one reviewed transaction grammar across install, update, disable, and rollback. A clean
    // sheet carries the reviewed grammar for each of the four flows, a grammar-incomplete sheet
    // degrades, and no clean sheet lacks the grammar.
    let every_flow_has_clean_grammar = M5InstallReviewMutationFlow::ALL
        .iter()
        .all(|flow| sheets().any(|ex| ex.is_clean() && ex.mutation_flow == flow.as_str()));
    let grammar_incomplete_degrades = sheets().any(|ex| {
        ex.degrade_reason == Some(M5InstallReviewSheetDegradeReason::TransactionGrammarIncomplete)
    });
    let no_clean_without_grammar = sheets().all(|ex| !ex.is_clean() || ex.has_transaction_grammar);
    if !(every_flow_has_clean_grammar && grammar_incomplete_degrades && no_clean_without_grammar) {
        violations.push(M5InstallReviewSheetControlsViolation::TransactionGrammarNotProven);
    }

    // AC2: disable scope and rollback compatibility are explicit, not buried. A clean disable sheet
    // names its scope, a clean rollback sheet names its compatibility, a disable-scope-unstated sheet
    // degrades, a rollback-truth sheet degrades, and no clean sheet hides a disable scope or rollback
    // incompatibility.
    let clean_disable_names_scope =
        sheets().any(|ex| ex.is_clean() && ex.reviews_disable_scope && ex.names_disable_scope);
    let clean_rollback_names_compat =
        sheets().any(|ex| ex.is_clean() && ex.reviews_rollback && ex.names_rollback_compatibility);
    let disable_scope_unstated_degrades = sheets().any(|ex| {
        ex.degrade_reason == Some(M5InstallReviewSheetDegradeReason::DisableScopeUnstated)
    });
    let rollback_truth_degrades = sheets().any(|ex| {
        matches!(
            ex.degrade_reason,
            Some(M5InstallReviewSheetDegradeReason::RollbackCompatibilityUnresolved)
                | Some(M5InstallReviewSheetDegradeReason::RollbackIncompatibilityHidden)
        )
    });
    let no_clean_hides_scope_or_rollback = sheets().all(|ex| {
        !(ex.is_clean() && (ex.hides_disable_scope || ex.hides_rollback_incompatibility))
    });
    if !(clean_disable_names_scope
        && clean_rollback_names_compat
        && disable_scope_unstated_degrades
        && rollback_truth_degrades
        && no_clean_hides_scope_or_rollback)
    {
        violations
            .push(M5InstallReviewSheetControlsViolation::DisableScopeAndRollbackTruthNotProven);
    }

    // AC3: registry / source continuity remains visible from review through help / support / export.
    // A clean sheet names a resolved source class and a stated publisher continuity, a
    // source-class-collapsed sheet degrades, a publisher-transfer-hidden sheet degrades, and no clean
    // sheet collapses the source class or hides a publisher transfer.
    let clean_names_source_and_continuity = sheets().any(|ex| {
        ex.is_clean()
            && ex.registry_source != M5RegistrySourceClass::SourceUnknown.as_str()
            && ex.publisher_continuity != M5PublisherContinuityState::ContinuityUnknown.as_str()
    });
    let source_collapsed_degrades = sheets().any(|ex| {
        ex.degrade_reason == Some(M5InstallReviewSheetDegradeReason::RegistrySourceClassCollapsed)
    });
    let transfer_hidden_degrades = sheets().any(|ex| {
        ex.degrade_reason
            == Some(M5InstallReviewSheetDegradeReason::PublisherContinuityWarningMissing)
    });
    let no_clean_collapses_or_hides = sheets()
        .all(|ex| !(ex.is_clean() && (ex.collapses_source_class || ex.hides_publisher_transfer)));
    if !(clean_names_source_and_continuity
        && source_collapsed_degrades
        && transfer_hidden_degrades
        && no_clean_collapses_or_hides)
    {
        violations.push(M5InstallReviewSheetControlsViolation::SourceContinuityNotProven);
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

/// The single component family this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5MarketplaceInstallComponentFamily; 1] =
    [M5MarketplaceInstallComponentFamily::InstallUpdateDisableRollbackReviewSheet];
