//! Implements the reusable deployment-profile primitive: an install-profile card,
//! a side-by-side import sheet, and a rollout-ring row that all resolve from one
//! deployment context and share one deployment identity, so install / update /
//! admin surfaces are truthful about operating mode, ownership, rollback target,
//! shared-vs-isolated state, and rollout stage *before* users or admins act.
//!
//! Where
//! [`crate::freeze_the_m5_deployment_continuity_component_matrix`] *freezes* the
//! reusable deployment / continuity component families as a governed contract, this
//! module *narrows* three of those families —
//! [`crate::freeze_the_m5_deployment_continuity_component_matrix::M5DeploymentComponentFamily::InstallProfileCard`],
//! [`crate::freeze_the_m5_deployment_continuity_component_matrix::M5DeploymentComponentFamily::SideBySideImportSheet`],
//! and
//! [`crate::freeze_the_m5_deployment_continuity_component_matrix::M5DeploymentComponentFamily::RolloutRingRow`]
//! — into one working primitive with a real **resolver**. A single deployment
//! context projects onto three surfaces that share one deployment identity, so the
//! build / channel / install mode that owns the running app, the rollback target,
//! the shared-vs-isolated state model of a side-by-side sibling, and the rollout
//! ring never blur across the card, the import sheet, and the ring row.
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — install ownership and rollback target are never hidden.** The
//!   install-profile card names the install mode, scope, channel, updater owner,
//!   durable state roots, the build that owns the running app, and the rollback
//!   target, so a user can always tell which build / channel / install mode owns
//!   the app and what rollback exists.
//! - **AC2 — side-by-side handoff never depends on hidden state sharing.** The
//!   side-by-side import sheet names the shared-vs-isolated state model and the
//!   one-time import / copy choice explicitly, never captures a default handler,
//!   and preserves a rollback checkpoint before durable state moves across
//!   channels.
//! - **AC3 — managed rollout preserves ring identity and promotion evidence.** The
//!   rollout-ring row names the ring owner, the promotion state, the platform
//!   scope, the evidence freshness, and the rollback path, so a managed fleet never
//!   flattens every install into one generic version list.
//!
//! Raw config bytes, credentials, license keys, mirror URLs, and device
//! identifiers never cross this boundary; the resolver carries only opaque refs,
//! typed class tokens, booleans, and redacted labels, so support and diagnostics
//! exports reconstruct exactly what a surface would have shown without leaking
//! source or live payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-deployment-profile-primitive.schema.json`](../../../../schemas/ui/m5-deployment-profile-primitive.schema.json).
//! The contract doc is
//! [`docs/deployment/m5_deployment_profile_primitive.md`](../../../../docs/deployment/m5_deployment_profile_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_deployment_continuity_component_matrix::{
    DegradedState, M5DeploymentDowngradeTrigger, M5DeploymentMode, M5DeploymentTruthMode,
    M5PromotionState, M5RolloutRing,
};

/// Stable record-kind tag carried by [`M5DeploymentProfilePrimitivePacket`].
pub const M5_DEPLOYMENT_PROFILE_RECORD_KIND: &str = "m5_deployment_profile_primitive";

/// Schema version for the deployment-profile primitive packet.
pub const M5_DEPLOYMENT_PROFILE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_DEPLOYMENT_PROFILE_SCHEMA_REF: &str =
    "schemas/ui/m5-deployment-profile-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DEPLOYMENT_PROFILE_DOC_REF: &str =
    "docs/deployment/m5_deployment_profile_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive
/// narrows.
pub const M5_DEPLOYMENT_PROFILE_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-deployment-continuity-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DEPLOYMENT_PROFILE_FIXTURE_DIR: &str = "fixtures/ui/m5-deployment-profile-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const M5_DEPLOYMENT_PROFILE_ARTIFACT_REF: &str =
    "artifacts/release/m5-deployment-profile-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_DEPLOYMENT_PROFILE_CSV_REF: &str =
    "artifacts/release/m5-deployment-profile-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_DEPLOYMENT_PROFILE_REPORT_REF: &str =
    "artifacts/release/m5-deployment-profile-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed deployment-profile surface family. Each family is one parity surface that
/// ingests the shared primitive; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentSurfaceFamily {
    /// The About / install-profile card on the desktop app.
    AboutInstallCard,
    /// The update-center surface showing channel and rollout stage.
    UpdateCenter,
    /// The admin fleet console governing managed rollout rings.
    AdminFleetConsole,
    /// The side-by-side review surface before a channel / state handoff.
    SideBySideReview,
    /// The diagnostics deployment pane.
    DiagnosticsDeployment,
    /// The support / export replay surface reconstructing deployment truth.
    SupportExportReplay,
}

impl M5DeploymentSurfaceFamily {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AboutInstallCard,
        Self::UpdateCenter,
        Self::AdminFleetConsole,
        Self::SideBySideReview,
        Self::DiagnosticsDeployment,
        Self::SupportExportReplay,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AboutInstallCard => "about_install_card",
            Self::UpdateCenter => "update_center",
            Self::AdminFleetConsole => "admin_fleet_console",
            Self::SideBySideReview => "side_by_side_review",
            Self::DiagnosticsDeployment => "diagnostics_deployment",
            Self::SupportExportReplay => "support_export_replay",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AboutInstallCard => "About / install-profile card",
            Self::UpdateCenter => "Update center",
            Self::AdminFleetConsole => "Admin fleet console",
            Self::SideBySideReview => "Side-by-side review",
            Self::DiagnosticsDeployment => "Diagnostics deployment pane",
            Self::SupportExportReplay => "Support / export replay",
        }
    }
}

/// Closed install-scope vocabulary. Names the per-user / per-machine / portable /
/// offline distinction the spec requires so a portable or offline install never
/// reads as a plain per-user desktop one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallScope {
    /// A per-user install owned by the current account.
    PerUser,
    /// A per-machine install shared across accounts on the device.
    PerMachine,
    /// A portable install carrying its own durable state root.
    Portable,
    /// An offline-provisioned install with no live delivery channel.
    Offline,
}

impl M5InstallScope {
    /// Every install scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PerUser,
        Self::PerMachine,
        Self::Portable,
        Self::Offline,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerUser => "per_user",
            Self::PerMachine => "per_machine",
            Self::Portable => "portable",
            Self::Offline => "offline",
        }
    }
}

/// Closed updater-owner vocabulary. Names who owns delivery of the next build so a
/// managed or store-owned updater never reads as a self-managed one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5UpdaterOwner {
    /// The local user owns the updater.
    SelfManaged,
    /// An organization admin owns the updater.
    ManagedAdmin,
    /// An OS app store owns the updater.
    OsStore,
    /// An offline mirror owns delivery.
    OfflineMirror,
}

impl M5UpdaterOwner {
    /// Every updater owner, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SelfManaged,
        Self::ManagedAdmin,
        Self::OsStore,
        Self::OfflineMirror,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelfManaged => "self_managed",
            Self::ManagedAdmin => "managed_admin",
            Self::OsStore => "os_store",
            Self::OfflineMirror => "offline_mirror",
        }
    }
}

/// Closed rollback-target vocabulary. Names what rollback exists if the running
/// build must be reverted, so "what rollback target exists" is never left blank.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RollbackTargetState {
    /// A restore checkpoint / snapshot exists to roll back to.
    CheckpointAvailable,
    /// A prior build is retained and can be re-pinned.
    PriorBuildRetained,
    /// No rollback is available; a revert is not possible.
    NoRollback,
    /// The rollback target has not yet been established.
    Unknown,
}

impl M5RollbackTargetState {
    /// Every rollback-target state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CheckpointAvailable,
        Self::PriorBuildRetained,
        Self::NoRollback,
        Self::Unknown,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CheckpointAvailable => "checkpoint_available",
            Self::PriorBuildRetained => "prior_build_retained",
            Self::NoRollback => "no_rollback",
            Self::Unknown => "unknown",
        }
    }

    /// True when this rollback target can actually restore a prior state (a
    /// checkpoint or a retained prior build), used to guard a durable state move.
    pub const fn is_recoverable(self) -> bool {
        matches!(self, Self::CheckpointAvailable | Self::PriorBuildRetained)
    }

    /// True when the rollback target has been established (not `Unknown`).
    pub const fn is_established(self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

/// Closed state-sharing vocabulary. Names how a side-by-side sibling relates to this
/// install's durable state so a hidden state-sharing assumption never survives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateSharingModel {
    /// Each install keeps a fully isolated durable state root.
    Isolated,
    /// State is shared read-only from the sibling.
    SharedReadOnly,
    /// State is shared writable across both installs.
    SharedWritable,
    /// State is copied once from the sibling, then isolated.
    OneTimeCopy,
}

impl M5StateSharingModel {
    /// Every state-sharing model, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Isolated,
        Self::SharedReadOnly,
        Self::SharedWritable,
        Self::OneTimeCopy,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Isolated => "isolated",
            Self::SharedReadOnly => "shared_read_only",
            Self::SharedWritable => "shared_writable",
            Self::OneTimeCopy => "one_time_copy",
        }
    }

    /// True when this model keeps the install's durable state isolated after the
    /// handoff (fully isolated or copied once, then isolated).
    pub const fn preserves_isolation(self) -> bool {
        matches!(self, Self::Isolated | Self::OneTimeCopy)
    }

    /// True when this model requires a sibling install to be meaningful (anything
    /// other than a fully isolated standalone install).
    pub const fn requires_sibling(self) -> bool {
        !matches!(self, Self::Isolated)
    }
}

/// Closed import-choice vocabulary. Names the one-time import / copy choice a
/// side-by-side sheet previews so a durable move is a reviewed, explicit action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ImportChoice {
    /// Import the sibling's state once into this install.
    OneTimeImport,
    /// Copy the sibling's state once, leaving the sibling untouched.
    OneTimeCopy,
    /// Link to shared state rather than move it.
    LinkShared,
    /// Skip the import; keep this install's own state.
    Skip,
}

impl M5ImportChoice {
    /// Every import choice, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OneTimeImport,
        Self::OneTimeCopy,
        Self::LinkShared,
        Self::Skip,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OneTimeImport => "one_time_import",
            Self::OneTimeCopy => "one_time_copy",
            Self::LinkShared => "link_shared",
            Self::Skip => "skip",
        }
    }

    /// True when this choice requires a sibling install to import / copy / link
    /// from (anything other than skipping).
    pub const fn requires_sibling(self) -> bool {
        !matches!(self, Self::Skip)
    }

    /// True when this choice moves or links durable state (not a plain skip).
    pub const fn moves_state(self) -> bool {
        !matches!(self, Self::Skip)
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet must
/// carry per surface; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentProfileExportField {
    /// The stable deployment identity shared across surfaces.
    DeploymentId,
    /// The opaque install identity ref.
    InstallIdentity,
    /// The operating / install mode and scope.
    OperatingMode,
    /// The release channel and updater owner.
    ChannelAndUpdater,
    /// The provenance / freshness truth class.
    ProvenanceFreshness,
    /// The rollback target that exists for the running build.
    RollbackTarget,
    /// The rollout ring, promotion state, and platform scope.
    RolloutRing,
}

impl M5DeploymentProfileExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DeploymentId,
        Self::InstallIdentity,
        Self::OperatingMode,
        Self::ChannelAndUpdater,
        Self::ProvenanceFreshness,
        Self::RollbackTarget,
        Self::RolloutRing,
    ];

    /// The mandatory subset every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::DeploymentId,
        Self::InstallIdentity,
        Self::OperatingMode,
        Self::ProvenanceFreshness,
        Self::RollbackTarget,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeploymentId => "deployment_id",
            Self::InstallIdentity => "install_identity",
            Self::OperatingMode => "operating_mode",
            Self::ChannelAndUpdater => "channel_and_updater",
            Self::ProvenanceFreshness => "provenance_freshness",
            Self::RollbackTarget => "rollback_target",
            Self::RolloutRing => "rollout_ring",
        }
    }
}

// --- resolver input ---

/// The full input to the deployment-profile resolver for one deployment context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentProfileInput {
    /// The stable deployment identity that must survive across the install card,
    /// import sheet, and rollout-ring row.
    pub deployment_id: String,
    /// Human-readable surface / context label.
    pub surface_label: String,
    /// Opaque ref to the install identity; never raw install bytes.
    pub install_id_ref: String,
    /// The operating / install mode.
    pub install_mode: M5DeploymentMode,
    /// The per-user / per-machine / portable / offline scope.
    pub install_scope: M5InstallScope,
    /// Opaque ref to the release channel.
    pub channel_ref: String,
    /// Who owns delivery of the next build.
    pub updater_owner: M5UpdaterOwner,
    /// Opaque ref to the durable state roots.
    pub state_root_ref: String,
    /// Opaque ref to the build that owns the running app.
    pub build_ref: String,
    /// The provenance / freshness truth class the install profile binds to.
    pub truth_mode: M5DeploymentTruthMode,
    /// The rollback target that exists for the running build.
    pub rollback_target: M5RollbackTargetState,
    /// Opaque ref to the rollback target build / checkpoint, when established.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_target_ref: Option<String>,
    /// Opaque ref to a side-by-side sibling install, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sibling_install_ref: Option<String>,
    /// How a side-by-side sibling relates to this install's durable state.
    pub state_sharing: M5StateSharingModel,
    /// The one-time import / copy choice the side-by-side sheet previews.
    pub import_choice: M5ImportChoice,
    /// A default handler is silently captured from the sibling; must be `false`.
    pub handler_capture: bool,
    /// Whether this handoff moves durable state across channels.
    pub moves_state_across_channel: bool,
    /// Whether a managed rollout governs this install.
    pub managed_rollout: bool,
    /// Which rollout ring this install sits in.
    pub rollout_ring: M5RolloutRing,
    /// Where the rollout is in its promotion lifecycle.
    pub promotion_state: M5PromotionState,
    /// Opaque ref to the ring owner; required when a managed rollout governs it.
    pub ring_owner_ref: String,
    /// Opaque ref to the platform / fleet scope the ring targets; required when a
    /// managed rollout governs it.
    pub platform_scope_ref: String,
    /// The provenance / freshness of the rollout promotion evidence.
    pub evidence_freshness: M5DeploymentTruthMode,
    /// A rollback path is available from the current rollout ring.
    pub rollout_rollback_available: bool,
    /// An externally-observed narrowing (control-plane outage, stale mirror,
    /// unresolved state root) that degrades the surface before action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

// --- resolved projections ---

/// The resolved install-profile card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedInstallProfileCard {
    /// The deployment identity — identical to the import sheet and ring row.
    pub deployment_id: String,
    /// The opaque install identity ref.
    pub install_id_ref: String,
    /// The operating / install mode.
    pub install_mode: M5DeploymentMode,
    /// The per-user / per-machine / portable / offline scope.
    pub install_scope: M5InstallScope,
    /// The opaque channel ref.
    pub channel_ref: String,
    /// Who owns delivery of the next build.
    pub updater_owner: M5UpdaterOwner,
    /// The opaque durable state-root ref.
    pub state_root_ref: String,
    /// The opaque ref to the build that owns the running app.
    pub build_ref: String,
    /// The provenance / freshness truth class.
    pub truth_mode: M5DeploymentTruthMode,
    /// The rollback target that exists for the running build.
    pub rollback_target: M5RollbackTargetState,
    /// The opaque ref to the rollback target, when established.
    pub rollback_target_ref: Option<String>,
    /// The card discloses which build / channel / mode owns the running app and
    /// what rollback exists (AC1).
    pub owns_running_app_disclosed: bool,
    /// The card discloses its durable state roots; always holds.
    pub discloses_state_roots: bool,
}

/// The resolved side-by-side import sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSideBySideImportSheet {
    /// The deployment identity — identical to every other surface.
    pub deployment_id: String,
    /// The provenance / freshness truth class — identical to the install card.
    pub truth_mode: M5DeploymentTruthMode,
    /// The opaque sibling install ref, when one exists.
    pub sibling_install_ref: Option<String>,
    /// Whether a side-by-side sibling exists.
    pub has_sibling: bool,
    /// The shared-vs-isolated state model.
    pub state_sharing: M5StateSharingModel,
    /// The one-time import / copy choice being previewed.
    pub import_choice: M5ImportChoice,
    /// The state model keeps this install's durable state isolated after handoff.
    pub isolation_preserved: bool,
    /// A rollback checkpoint is preserved before durable state moves (AC2).
    pub rollback_checkpoint_preserved: bool,
    /// A default handler is silently captured; always `false`.
    pub handler_capture: bool,
    /// The shared-vs-isolated model is named explicitly, not assumed (AC2).
    pub state_sharing_explicit: bool,
}

/// The resolved rollout-ring row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRolloutRingRow {
    /// The deployment identity — identical to every other surface.
    pub deployment_id: String,
    /// The provenance / freshness of the promotion evidence.
    pub evidence_freshness: M5DeploymentTruthMode,
    /// Whether a managed rollout governs this install.
    pub managed: bool,
    /// The rollout ring this install sits in.
    pub ring: M5RolloutRing,
    /// Where the rollout is in its promotion lifecycle.
    pub promotion_state: M5PromotionState,
    /// The opaque ring owner ref.
    pub ring_owner_ref: String,
    /// The opaque platform / fleet scope ref.
    pub platform_scope_ref: String,
    /// A rollback path is available from the current ring.
    pub rollback_available: bool,
    /// The ring identity and promotion evidence are preserved, not flattened into a
    /// generic version list (AC3).
    pub ring_identity_preserved: bool,
}

/// The resolved deployment-profile truth shared across the install card, import
/// sheet, and rollout-ring row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDeploymentProfile {
    /// The stable deployment identity.
    pub deployment_id: String,
    /// The resolved install-profile card.
    pub install_card: M5ResolvedInstallProfileCard,
    /// The resolved side-by-side import sheet.
    pub import_sheet: M5ResolvedSideBySideImportSheet,
    /// The resolved rollout-ring row.
    pub rollout_row: M5ResolvedRolloutRingRow,
    /// The running build / channel / mode owner and rollback target are disclosed
    /// (AC1).
    pub running_owner_disclosed: bool,
    /// The shared-vs-isolated state model is explicit before handoff (AC2).
    pub state_sharing_explicit: bool,
    /// The rollout ring identity is preserved, not flattened (AC3).
    pub ring_identity_preserved: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedDeploymentProfile {
    /// True when the deployment identity is identical across the install card,
    /// import sheet, and rollout-ring row.
    pub fn identity_consistent(&self) -> bool {
        self.install_card.deployment_id == self.deployment_id
            && self.import_sheet.deployment_id == self.deployment_id
            && self.rollout_row.deployment_id == self.deployment_id
    }

    /// True when the install card and import sheet disclose the same provenance /
    /// freshness truth class — the install's provenance never blurs between them.
    pub fn truth_class_consistent(&self) -> bool {
        self.install_card.truth_mode == self.import_sheet.truth_mode
    }

    /// True when the running build / channel / mode owner and the rollback target
    /// are disclosed (AC1).
    pub fn running_owner_disclosed(&self) -> bool {
        self.running_owner_disclosed
    }

    /// True when the shared-vs-isolated state model is explicit and no default
    /// handler is captured (AC2).
    pub fn state_sharing_explicit(&self) -> bool {
        self.state_sharing_explicit && !self.import_sheet.handler_capture
    }

    /// True when the rollout ring identity and promotion evidence are preserved
    /// (AC3).
    pub fn ring_identity_preserved(&self) -> bool {
        self.ring_identity_preserved
    }
}

/// Errors returned by [`resolve_deployment_profile`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5DeploymentProfileResolutionError {
    /// The deployment identity was empty.
    EmptyDeploymentId,
    /// The install identity ref was empty.
    EmptyInstallIdRef,
    /// The channel ref was empty.
    EmptyChannelRef,
    /// The durable state-root ref was empty.
    EmptyStateRootRef,
    /// The running-build ref was empty.
    EmptyBuildRef,
    /// A label, ref, or note carried forbidden material.
    ForbiddenMaterial,
    /// A side-by-side sheet silently captured a default handler.
    LastWriterWinsCapture,
    /// An import / copy / link choice or a sharing model referenced a sibling that
    /// is not present.
    SharingWithoutSibling,
    /// A durable state move was offered with no rollback checkpoint to fall back to.
    StateMoveWithoutCheckpoint,
    /// A managed rollout omitted its ring owner or platform scope, flattening ring
    /// identity into a generic version list.
    RolloutIdentityFlattened,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5DeploymentProfileResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDeploymentId => "empty_deployment_id",
            Self::EmptyInstallIdRef => "empty_install_id_ref",
            Self::EmptyChannelRef => "empty_channel_ref",
            Self::EmptyStateRootRef => "empty_state_root_ref",
            Self::EmptyBuildRef => "empty_build_ref",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::LastWriterWinsCapture => "last_writer_wins_capture",
            Self::SharingWithoutSibling => "sharing_without_sibling",
            Self::StateMoveWithoutCheckpoint => "state_move_without_checkpoint",
            Self::RolloutIdentityFlattened => "rollout_identity_flattened",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5DeploymentProfileResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "deployment-profile resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DeploymentProfileResolutionError {}

/// Resolves one deployment context into its shared install-profile card,
/// side-by-side import sheet, and rollout-ring row.
///
/// The three surfaces share one deployment identity, so the build / channel /
/// install mode that owns the running app, the rollback target, the
/// shared-vs-isolated state of a side-by-side sibling, and the rollout ring never
/// blur across them. A side-by-side handoff never silently captures a default
/// handler and never moves durable state without a rollback checkpoint; a managed
/// rollout never flattens its ring identity; a degraded input narrows the surface
/// before action rather than after.
pub fn resolve_deployment_profile(
    input: &M5DeploymentProfileInput,
) -> Result<M5ResolvedDeploymentProfile, M5DeploymentProfileResolutionError> {
    if input.deployment_id.trim().is_empty() {
        return Err(M5DeploymentProfileResolutionError::EmptyDeploymentId);
    }
    if input.install_id_ref.trim().is_empty() {
        return Err(M5DeploymentProfileResolutionError::EmptyInstallIdRef);
    }
    if input.channel_ref.trim().is_empty() {
        return Err(M5DeploymentProfileResolutionError::EmptyChannelRef);
    }
    if input.state_root_ref.trim().is_empty() {
        return Err(M5DeploymentProfileResolutionError::EmptyStateRootRef);
    }
    if input.build_ref.trim().is_empty() {
        return Err(M5DeploymentProfileResolutionError::EmptyBuildRef);
    }

    for value in [
        input.deployment_id.as_str(),
        input.surface_label.as_str(),
        input.install_id_ref.as_str(),
        input.channel_ref.as_str(),
        input.state_root_ref.as_str(),
        input.build_ref.as_str(),
        input.ring_owner_ref.as_str(),
        input.platform_scope_ref.as_str(),
    ]
    .into_iter()
    .chain(input.rollback_target_ref.as_deref())
    .chain(input.sibling_install_ref.as_deref())
    {
        if value_is_forbidden(value) {
            return Err(M5DeploymentProfileResolutionError::ForbiddenMaterial);
        }
    }

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5DeploymentProfileResolutionError::DegradedLabelGeneric);
        }
    }

    let has_sibling = input.sibling_install_ref.is_some();

    // A side-by-side handoff never silently captures a default handler.
    if input.handler_capture {
        return Err(M5DeploymentProfileResolutionError::LastWriterWinsCapture);
    }
    // An import / copy / link choice or a sharing model that references a sibling
    // requires one to be present — hidden sharing never survives.
    if (input.import_choice.requires_sibling() || input.state_sharing.requires_sibling())
        && !has_sibling
    {
        return Err(M5DeploymentProfileResolutionError::SharingWithoutSibling);
    }

    let rollback_checkpoint_preserved = input.rollback_target.is_recoverable();
    // A durable state move is never offered with no rollback checkpoint to fall
    // back to.
    let moves_state = input.moves_state_across_channel || input.import_choice.moves_state();
    if moves_state && !rollback_checkpoint_preserved {
        return Err(M5DeploymentProfileResolutionError::StateMoveWithoutCheckpoint);
    }

    let ring_identity_preserved = !input.managed_rollout
        || (!input.ring_owner_ref.trim().is_empty() && !input.platform_scope_ref.trim().is_empty());
    // A managed rollout never flattens its ring identity into a generic version
    // list.
    if !ring_identity_preserved {
        return Err(M5DeploymentProfileResolutionError::RolloutIdentityFlattened);
    }

    // The running-app owner and rollback target are disclosed when the build and
    // channel are named and the rollback target has been established.
    let owns_running_app_disclosed = !input.build_ref.trim().is_empty()
        && !input.channel_ref.trim().is_empty()
        && input.rollback_target.is_established();
    let isolation_preserved = input.state_sharing.preserves_isolation();
    let state_sharing_explicit = !input.handler_capture;

    let install_card = M5ResolvedInstallProfileCard {
        deployment_id: input.deployment_id.clone(),
        install_id_ref: input.install_id_ref.clone(),
        install_mode: input.install_mode,
        install_scope: input.install_scope,
        channel_ref: input.channel_ref.clone(),
        updater_owner: input.updater_owner,
        state_root_ref: input.state_root_ref.clone(),
        build_ref: input.build_ref.clone(),
        truth_mode: input.truth_mode,
        rollback_target: input.rollback_target,
        rollback_target_ref: input.rollback_target_ref.clone(),
        owns_running_app_disclosed,
        discloses_state_roots: true,
    };

    let import_sheet = M5ResolvedSideBySideImportSheet {
        deployment_id: input.deployment_id.clone(),
        truth_mode: input.truth_mode,
        sibling_install_ref: input.sibling_install_ref.clone(),
        has_sibling,
        state_sharing: input.state_sharing,
        import_choice: input.import_choice,
        isolation_preserved,
        rollback_checkpoint_preserved,
        handler_capture: input.handler_capture,
        state_sharing_explicit,
    };

    let rollout_row = M5ResolvedRolloutRingRow {
        deployment_id: input.deployment_id.clone(),
        evidence_freshness: input.evidence_freshness,
        managed: input.managed_rollout,
        ring: input.rollout_ring,
        promotion_state: input.promotion_state,
        ring_owner_ref: input.ring_owner_ref.clone(),
        platform_scope_ref: input.platform_scope_ref.clone(),
        rollback_available: input.rollout_rollback_available,
        ring_identity_preserved,
    };

    Ok(M5ResolvedDeploymentProfile {
        deployment_id: input.deployment_id.clone(),
        install_card,
        import_sheet,
        rollout_row,
        running_owner_disclosed: owns_running_app_disclosed,
        state_sharing_explicit,
        ring_identity_preserved,
        degraded: input.degraded.clone(),
    })
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
/// reconstructs deployment truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentProfileCase {
    /// The resolver input.
    pub input: M5DeploymentProfileInput,
    /// The resolved deployment truth. Must equal
    /// `resolve_deployment_profile(&input)`.
    pub resolved: M5ResolvedDeploymentProfile,
}

impl M5DeploymentProfileCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5DeploymentProfileInput) -> Self {
        let resolved = resolve_deployment_profile(&input).expect("seed deployment case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_deployment_profile(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one deployment surface family bound to the
/// shared deployment-profile contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentProfileSurfaceRow {
    /// The deployment surface family.
    pub surface_family: M5DeploymentSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Install scopes this surface can disclose (must be non-empty).
    pub install_scopes: Vec<M5InstallScope>,
    /// Truth classes this surface renders (must be non-empty).
    pub truth_modes: Vec<M5DeploymentTruthMode>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5DeploymentProfileExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5DeploymentDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be
    /// non-empty).
    pub example_profiles: Vec<M5DeploymentProfileCase>,
    /// Hard invariant: this row never hides install ownership. MUST be `false`.
    pub hides_install_ownership: bool,
    /// Hard invariant: this row never assumes hidden state sharing. MUST be
    /// `false`.
    pub assumes_hidden_state_sharing: bool,
    /// Hard invariant: this row never flattens rollout identity. MUST be `false`.
    pub flattens_rollout_identity: bool,
    /// Hard invariant: this row never loses the rollback target. MUST be `false`.
    pub loses_rollback_target: bool,
}

impl M5DeploymentProfileSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5DeploymentProfileExportField> =
            self.export_fields.iter().copied().collect();
        M5DeploymentProfileExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.hides_install_ownership
            && !self.assumes_hidden_state_sharing
            && !self.flattens_rollout_identity
            && !self.loses_rollback_target
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentProfileVocabularySet {
    /// Deployment surface-family tokens.
    pub surface_families: Vec<String>,
    /// Install-scope tokens.
    pub install_scopes: Vec<String>,
    /// Updater-owner tokens.
    pub updater_owners: Vec<String>,
    /// Rollback-target tokens.
    pub rollback_targets: Vec<String>,
    /// State-sharing-model tokens.
    pub state_sharing_models: Vec<String>,
    /// Import-choice tokens.
    pub import_choices: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Deployment-mode tokens (reused from the frozen matrix).
    pub deployment_modes: Vec<String>,
    /// Truth-class tokens (reused from the frozen matrix).
    pub truth_modes: Vec<String>,
    /// Rollout-ring tokens (reused from the frozen matrix).
    pub rollout_rings: Vec<String>,
    /// Promotion-state tokens (reused from the frozen matrix).
    pub promotion_states: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5DeploymentProfileVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(
                &M5DeploymentSurfaceFamily::ALL,
                M5DeploymentSurfaceFamily::as_str,
            ),
            install_scopes: tokens(&M5InstallScope::ALL, M5InstallScope::as_str),
            updater_owners: tokens(&M5UpdaterOwner::ALL, M5UpdaterOwner::as_str),
            rollback_targets: tokens(&M5RollbackTargetState::ALL, M5RollbackTargetState::as_str),
            state_sharing_models: tokens(&M5StateSharingModel::ALL, M5StateSharingModel::as_str),
            import_choices: tokens(&M5ImportChoice::ALL, M5ImportChoice::as_str),
            export_fields: tokens(
                &M5DeploymentProfileExportField::ALL,
                M5DeploymentProfileExportField::as_str,
            ),
            deployment_modes: tokens(&DEPLOYMENT_MODE_ALL, M5DeploymentMode::as_str),
            truth_modes: tokens(&TRUTH_MODE_ALL, M5DeploymentTruthMode::as_str),
            rollout_rings: tokens(&ROLLOUT_RING_ALL, M5RolloutRing::as_str),
            promotion_states: tokens(&PROMOTION_STATE_ALL, M5PromotionState::as_str),
            downgrade_triggers: tokens(
                &DOWNGRADE_TRIGGER_ALL,
                M5DeploymentDowngradeTrigger::as_str,
            ),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The deployment modes reused from the frozen matrix, in a stable order.
const DEPLOYMENT_MODE_ALL: [M5DeploymentMode; 5] = [
    M5DeploymentMode::Desktop,
    M5DeploymentMode::Managed,
    M5DeploymentMode::SelfHosted,
    M5DeploymentMode::Portable,
    M5DeploymentMode::AirGapped,
];

/// The truth classes reused from the frozen matrix, in a stable order.
const TRUTH_MODE_ALL: [M5DeploymentTruthMode; 5] = [
    M5DeploymentTruthMode::Live,
    M5DeploymentTruthMode::Mirrored,
    M5DeploymentTruthMode::CachedOffline,
    M5DeploymentTruthMode::Imported,
    M5DeploymentTruthMode::ProviderReported,
];

/// The rollout rings reused from the frozen matrix, in a stable order.
const ROLLOUT_RING_ALL: [M5RolloutRing; 5] = [
    M5RolloutRing::Canary,
    M5RolloutRing::EarlyAdopter,
    M5RolloutRing::Broad,
    M5RolloutRing::GeneralAvailability,
    M5RolloutRing::Paused,
];

/// The promotion states reused from the frozen matrix, in a stable order.
const PROMOTION_STATE_ALL: [M5PromotionState; 5] = [
    M5PromotionState::Held,
    M5PromotionState::Promoting,
    M5PromotionState::Promoted,
    M5PromotionState::RolledBack,
    M5PromotionState::Blocked,
];

/// The downgrade triggers reused from the frozen matrix, in a stable order.
const DOWNGRADE_TRIGGER_ALL: [M5DeploymentDowngradeTrigger; 9] = [
    M5DeploymentDowngradeTrigger::ControlPlaneImpaired,
    M5DeploymentDowngradeTrigger::MirrorStale,
    M5DeploymentDowngradeTrigger::OfflineCacheOnly,
    M5DeploymentDowngradeTrigger::SignatureUnverified,
    M5DeploymentDowngradeTrigger::RolloutPaused,
    M5DeploymentDowngradeTrigger::HandlerOwnershipContested,
    M5DeploymentDowngradeTrigger::StateRootUnavailable,
    M5DeploymentDowngradeTrigger::ResidualVendorDependency,
    M5DeploymentDowngradeTrigger::ProvenanceIncomplete,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentProfileGovernanceReview {
    /// One primitive carries install-card / import-sheet / rollout-row truth on
    /// every surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Deployment identity is preserved across the card, sheet, and ring row.
    pub deployment_identity_preserved_across_surfaces: bool,
    /// Install ownership and rollback target are never hidden.
    pub install_ownership_and_rollback_never_hidden: bool,
    /// Shared-vs-isolated state is explicit before a side-by-side handoff.
    pub state_sharing_explicit_before_handoff: bool,
    /// Managed rollout preserves ring identity and promotion evidence.
    pub rollout_ring_identity_preserved: bool,
    /// The support / export packet reconstructs deployment truth.
    pub support_export_reconstructs_deployment: bool,
    /// Later M5 rows cannot invent parallel deployment-profile vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentProfileConsumerProjection {
    /// Install / update / admin / diagnostics surfaces all consume the shared
    /// primitive.
    pub deployment_surfaces_consume_shared_primitive: bool,
    /// The deployment resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The side-by-side sheet reads a single canonical state-sharing source.
    pub side_by_side_reads_single_state_source: bool,
    /// Support / export reads a single canonical deployment source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the deployment-profile primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentProfileReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting deployment audit.
    pub deployment_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DeploymentProfilePrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DeploymentProfilePrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5DeploymentProfileSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DeploymentProfileVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DeploymentProfileGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DeploymentProfileConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5DeploymentProfileReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 deployment-profile primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentProfilePrimitivePacket {
    /// Record kind; must equal [`M5_DEPLOYMENT_PROFILE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DEPLOYMENT_PROFILE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5DeploymentProfileSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DeploymentProfileVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DeploymentProfileGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DeploymentProfileConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5DeploymentProfileReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DeploymentProfilePrimitivePacket {
    /// Builds an M5 deployment-profile primitive packet from stable-lane input.
    pub fn new(input: M5DeploymentProfilePrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_DEPLOYMENT_PROFILE_RECORD_KIND.to_owned(),
            schema_version: M5_DEPLOYMENT_PROFILE_SCHEMA_VERSION,
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

    /// Validates the M5 deployment-profile primitive invariants.
    pub fn validate(&self) -> Vec<M5DeploymentProfileViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DEPLOYMENT_PROFILE_RECORD_KIND {
            violations.push(M5DeploymentProfileViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DEPLOYMENT_PROFILE_SCHEMA_VERSION {
            violations.push(M5DeploymentProfileViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DeploymentProfileViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 deployment-profile primitive packet serializes"),
        ) {
            violations.push(M5DeploymentProfileViolation::RawMaterialInExport);
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
            .expect("m5 deployment-profile primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,owner,install_scopes,truth_modes,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.install_scopes, |v| v.as_str()),
                join_tokens(&row.truth_modes, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_profiles.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Deployment-Profile Primitive: Install-Profile Card, Side-by-Side Import Sheet, and Rollout-Ring Row\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Deployment surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5DeploymentSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Install scopes: {}\n",
            self.vocabulary_set.install_scopes.join(", ")
        ));
        out.push_str(&format!(
            "- Rollback targets: {}\n",
            self.vocabulary_set.rollback_targets.join(", ")
        ));
        out.push_str(&format!(
            "- State-sharing models: {}\n",
            self.vocabulary_set.state_sharing_models.join(", ")
        ));
        out.push_str("\n## Deployment surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked cases: {}\n",
                row.example_profiles.len()
            ));
            for case in &row.example_profiles {
                out.push_str(&format!(
                    "    - `{}` → mode `{}`/`{}`, rollback `{}`, ring `{}`/`{}`\n",
                    case.resolved.deployment_id,
                    case.resolved.install_card.install_mode.as_str(),
                    case.resolved.install_card.install_scope.as_str(),
                    case.resolved.install_card.rollback_target.as_str(),
                    case.resolved.rollout_row.ring.as_str(),
                    case.resolved.rollout_row.promotion_state.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 deployment-profile export.
#[derive(Debug)]
pub enum M5DeploymentProfileArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DeploymentProfileViolation>),
}

impl fmt::Display for M5DeploymentProfileArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 deployment-profile primitive export parse failed: {error}"
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
                    "m5 deployment-profile primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DeploymentProfileArtifactError {}

/// Validation failures emitted by [`M5DeploymentProfilePrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DeploymentProfileViolation {
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
    /// A required deployment surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row declares no install scopes.
    InstallScopeMissing,
    /// A surface row declares no truth classes.
    TruthModeMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked deployment cases.
    ExampleProfilesMissing,
    /// A worked deployment case does not match a fresh resolve of its input.
    ExampleProfileDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves deployment identity preserved and running-app owner /
    /// rollback disclosed (AC1).
    RunningOwnerDisclosureUnproven,
    /// No worked case proves shared-vs-isolated state explicit before handoff
    /// (AC2).
    StateSharingHonestyUnproven,
    /// No worked case proves rollout ring identity preserved (AC3).
    RingIdentityUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5DeploymentProfileViolation {
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
            Self::InstallScopeMissing => "install_scope_missing",
            Self::TruthModeMissing => "truth_mode_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleProfilesMissing => "example_profiles_missing",
            Self::ExampleProfileDrift => "example_profile_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::RunningOwnerDisclosureUnproven => "running_owner_disclosure_unproven",
            Self::StateSharingHonestyUnproven => "state_sharing_honesty_unproven",
            Self::RingIdentityUnproven => "ring_identity_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 deployment-profile export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_stable_m5_deployment_profile_export(
) -> Result<M5DeploymentProfilePrimitivePacket, M5DeploymentProfileArtifactError> {
    let packet: M5DeploymentProfilePrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-deployment-profile-primitive-proof/support_export.json"
    )))
    .map_err(M5DeploymentProfileArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DeploymentProfileArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5DeploymentProfilePrimitivePacket,
    violations: &mut Vec<M5DeploymentProfileViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DEPLOYMENT_PROFILE_SCHEMA_REF,
        M5_DEPLOYMENT_PROFILE_DOC_REF,
        M5_DEPLOYMENT_PROFILE_COMPONENT_MATRIX_REF,
        M5_DEPLOYMENT_PROFILE_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DeploymentProfileViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DeploymentProfilePrimitivePacket,
    violations: &mut Vec<M5DeploymentProfileViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DeploymentProfileViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5DeploymentProfilePrimitivePacket,
    violations: &mut Vec<M5DeploymentProfileViolation>,
) {
    let present: BTreeSet<M5DeploymentSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5DeploymentSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5DeploymentProfileViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5DeploymentProfileViolation::SurfaceRowIncomplete);
        }
        if row.install_scopes.is_empty() {
            violations.push(M5DeploymentProfileViolation::InstallScopeMissing);
        }
        if row.truth_modes.is_empty() {
            violations.push(M5DeploymentProfileViolation::TruthModeMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DeploymentProfileViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DeploymentProfileViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DeploymentProfileViolation::ConsumerSurfacesMissing);
        }
        if row.example_profiles.is_empty() {
            violations.push(M5DeploymentProfileViolation::ExampleProfilesMissing);
        }
        if row
            .example_profiles
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5DeploymentProfileViolation::ExampleProfileDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5DeploymentProfileViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated by at least one worked case
/// across the matrix: deployment identity preserved and the running-app owner /
/// rollback disclosed (AC1), shared-vs-isolated state explicit before a
/// side-by-side handoff (AC2), and rollout ring identity preserved rather than
/// flattened (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5DeploymentProfilePrimitivePacket,
    violations: &mut Vec<M5DeploymentProfileViolation>,
) {
    let cases: Vec<&M5ResolvedDeploymentProfile> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_profiles.iter().map(|case| &case.resolved))
        .collect();

    // AC1: at least one case preserves identity, keeps the install provenance
    // consistent, and discloses the running-app owner / rollback target.
    let running_owner_proven = cases.iter().any(|resolved| {
        resolved.identity_consistent()
            && resolved.truth_class_consistent()
            && resolved.running_owner_disclosed()
    });
    if !running_owner_proven {
        violations.push(M5DeploymentProfileViolation::RunningOwnerDisclosureUnproven);
    }

    // AC2: at least one case proves an explicit shared-vs-isolated handoff with a
    // real sibling and a preserved checkpoint, and every case keeps its state
    // sharing explicit with no captured handler.
    let state_sharing_proven = cases.iter().any(|resolved| {
        resolved.import_sheet.has_sibling && resolved.import_sheet.rollback_checkpoint_preserved
    }) && cases
        .iter()
        .all(|resolved| resolved.state_sharing_explicit());
    if !state_sharing_proven {
        violations.push(M5DeploymentProfileViolation::StateSharingHonestyUnproven);
    }

    // AC3: at least one managed rollout case preserves ring identity, and every
    // case preserves ring identity (never flattened).
    let ring_identity_proven = cases
        .iter()
        .any(|resolved| resolved.rollout_row.managed && resolved.ring_identity_preserved())
        && cases
            .iter()
            .all(|resolved| resolved.ring_identity_preserved());
    if !ring_identity_proven {
        violations.push(M5DeploymentProfileViolation::RingIdentityUnproven);
    }
}

fn validate_governance_review(
    packet: &M5DeploymentProfilePrimitivePacket,
    violations: &mut Vec<M5DeploymentProfileViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.deployment_identity_preserved_across_surfaces,
        review.install_ownership_and_rollback_never_hidden,
        review.state_sharing_explicit_before_handoff,
        review.rollout_ring_identity_preserved,
        review.support_export_reconstructs_deployment,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5DeploymentProfileViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DeploymentProfilePrimitivePacket,
    violations: &mut Vec<M5DeploymentProfileViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.deployment_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.side_by_side_reads_single_state_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5DeploymentProfileViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5DeploymentProfilePrimitivePacket,
    violations: &mut Vec<M5DeploymentProfileViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.deployment_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DeploymentProfileViolation::ReleasePostureIncomplete);
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
