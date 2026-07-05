//! Frozen reusable deployment/continuity component matrix: install-profile cards,
//! side-by-side import sheets, rollout-ring rows, deployment summary cards,
//! residual-dependency rows, control-plane/data-plane status strips, mirror/offline
//! artifact rows, mode-change review sheets, and channel-association review rows.
//!
//! Where [`crate::topology`] freezes the canonical *install-topology* alpha model,
//! [`crate::m5_coexistence_and_fleet_rollout`] carries *coexistence and fleet-rollout*
//! truth, [`crate::m5_install_and_portability_governance`] carries *portable-state
//! governance*, and [`crate::rollback`] carries *rollback drill* truth, this module
//! freezes the reusable **deployment/continuity component** contract: the cards, rows,
//! sheets, and strips users and admins actually rely on to understand operating mode,
//! rollout state, mirror freshness, residual dependency, and local-safe continuity
//! before acting, so later M5 rows reference one canonical component family instead of
//! restating install / about / admin truth in feature-local prose.
//!
//! One [`DeploymentContinuityComponentMatrix`] packet defines every reusable
//! primitive, its state vocabulary, its required labels, and its export / assistive
//! parity expectations, binding each onto the same install-mode, provenance /
//! freshness, client-scope, control-plane/data-plane, and degraded-state vocabulary
//! already used across Aureline's install-topology, coexistence, portability, and
//! restore contracts — never bespoke per-installer or per-admin chrome.
//!
//! The honesty rules the spec freezes, carried by every [`ComponentRow`]:
//!
//! - **Operating mode, ownership, and state roots stay explicit.** An install-profile
//!   card or deployment summary card never hides install mode, channel, updater owner,
//!   tenant / region, or durable state roots.
//! - **Side-by-side and channel association never devolve into last-writer-wins
//!   capture.** A side-by-side import sheet and a channel-association review row keep
//!   handler ownership inspectable and never silently capture a default handler.
//! - **Control-plane impairment never masquerades as local-runtime failure.** A
//!   control-plane/data-plane status strip keeps the two planes distinct so a managed
//!   control-plane outage never reads as a broken local runtime.
//! - **Mirror / offline freshness is never shown as current.** A mirror/offline
//!   artifact row discloses freshness and signature truth so stale mirrored content
//!   never reads as a live source.
//! - **Self-hosted claims never omit residual vendor dependency.** A residual-dependency
//!   row keeps any remaining vendor dependency explicit.
//! - **Mode switches / cache reuse / rollback are reviewed before durable boundary
//!   changes.** A mode-change review sheet shows the cache and rollback consequences
//!   before a durable boundary change occurs, never after.
//!
//! Raw config bytes, credentials, license keys, mirror URLs, provider cursors, and raw
//! device identifiers never cross this boundary; the packet carries only typed class
//! tokens, opaque install / channel / mirror / handler refs, booleans, and redacted
//! labels, so support and diagnostics exports can reconstruct exactly what a component
//! would have shown without leaking source or live payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-deployment-continuity-component-matrix.schema.json`](../../../../schemas/ui/m5-deployment-continuity-component-matrix.schema.json).
//! The contract doc is
//! [`docs/deployment/m5_deployment_continuity_component_matrix.md`](../../../../docs/deployment/m5_deployment_continuity_component_matrix.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-deployment-continuity-components/`](../../../../fixtures/ui/m5-deployment-continuity-components/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`DeploymentContinuityComponentMatrix`].
pub const DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_RECORD_KIND: &str =
    "m5_deployment_continuity_component_matrix";

/// Schema version for the deployment/continuity component matrix packet.
pub const DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-deployment-continuity-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_DOC_REF: &str =
    "docs/deployment/m5_deployment_continuity_component_matrix.md";

/// Repo-relative path of the protected fixture directory.
pub const DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_FIXTURE_DIR: &str =
    "fixtures/ui/m5-deployment-continuity-components";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_ARTIFACT_REF: &str =
    "artifacts/release/m5-deployment-continuity-component-proof/support_export.json";

/// Repo-relative path of the checked Markdown matrix summary.
pub const DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_SUMMARY_REF: &str =
    "artifacts/design/m5-deployment-continuity-component-matrix.md";

/// Repo-relative path of the checked certification artifact proving every claimed M5
/// deployment surface consumes the shared component family and narrows when parity is
/// degraded.
pub const DEPLOYMENT_CONTINUITY_COMPONENT_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/release/m5-deployment-continuity-component-proof/certification.json";

/// Repo-relative path of the certification CSV projection.
pub const DEPLOYMENT_CONTINUITY_COMPONENT_CERTIFICATION_CSV_REF: &str =
    "artifacts/release/m5-deployment-continuity-component-proof/certification.csv";

/// Repo-relative path of the certification report.
pub const DEPLOYMENT_CONTINUITY_COMPONENT_CERTIFICATION_REPORT_REF: &str =
    "artifacts/release/m5-deployment-continuity-component-proof/certification.md";

/// Closed reusable deployment/continuity component family. Each family is one governed
/// primitive later M5 rows reference by name; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentComponentFamily {
    /// An install-profile card framing install mode, channel, updater owner, and roots.
    InstallProfileCard,
    /// A side-by-side import sheet keeping handler ownership free of capture.
    SideBySideImportSheet,
    /// A rollout-ring row disclosing ring and promotion state.
    RolloutRingRow,
    /// A deployment summary card framing operating mode, tenant / region, and planes.
    DeploymentSummaryCard,
    /// A residual-dependency row keeping remaining vendor dependency explicit.
    ResidualDependencyRow,
    /// A control-plane/data-plane status strip keeping the two planes distinct.
    ControlPlaneDataPlaneStatusStrip,
    /// A mirror/offline artifact row disclosing freshness and signature truth.
    MirrorOfflineArtifactRow,
    /// A mode-change review sheet disclosing cache / rollback before durable change.
    ModeChangeReviewSheet,
    /// A channel-association review row keeping handler ownership free of capture.
    ChannelAssociationReviewRow,
}

impl M5DeploymentComponentFamily {
    /// Every reusable component family the matrix must define, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::InstallProfileCard,
        Self::SideBySideImportSheet,
        Self::RolloutRingRow,
        Self::DeploymentSummaryCard,
        Self::ResidualDependencyRow,
        Self::ControlPlaneDataPlaneStatusStrip,
        Self::MirrorOfflineArtifactRow,
        Self::ModeChangeReviewSheet,
        Self::ChannelAssociationReviewRow,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallProfileCard => "install_profile_card",
            Self::SideBySideImportSheet => "side_by_side_import_sheet",
            Self::RolloutRingRow => "rollout_ring_row",
            Self::DeploymentSummaryCard => "deployment_summary_card",
            Self::ResidualDependencyRow => "residual_dependency_row",
            Self::ControlPlaneDataPlaneStatusStrip => "control_plane_data_plane_status_strip",
            Self::MirrorOfflineArtifactRow => "mirror_offline_artifact_row",
            Self::ModeChangeReviewSheet => "mode_change_review_sheet",
            Self::ChannelAssociationReviewRow => "channel_association_review_row",
        }
    }
}

/// Closed operating-mode vocabulary. Names how the install is deployed so a managed,
/// self-hosted, portable, or air-gapped install never reads as a plain desktop one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentMode {
    /// A first-party desktop install owned by the local user.
    Desktop,
    /// An organization-managed, policy-controlled install.
    Managed,
    /// A self-hosted install operated by the customer.
    SelfHosted,
    /// A portable install carrying its own durable state root.
    Portable,
    /// An air-gapped install provisioned from offline media.
    AirGapped,
}

impl M5DeploymentMode {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Managed => "managed",
            Self::SelfHosted => "self_hosted",
            Self::Portable => "portable",
            Self::AirGapped => "air_gapped",
        }
    }

    /// True when the mode is a customer-operated install that must disclose any
    /// residual vendor dependency.
    pub const fn is_customer_operated(self) -> bool {
        matches!(self, Self::SelfHosted | Self::AirGapped)
    }
}

/// Closed provenance/freshness truth class. Names whether a component renders a live
/// first-party source, mirrored content, cached-offline content, imported external
/// truth, or a provider-reported overlay, so mirrored or cached content never reads as
/// a live source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentTruthMode {
    /// A live, first-party source reachable now.
    Live,
    /// Mirrored content served from a mirror source.
    Mirrored,
    /// Cached content served while offline.
    CachedOffline,
    /// Imported external truth that is read-only locally.
    Imported,
    /// A provider-reported overlay the provider owns and completes.
    ProviderReported,
}

impl M5DeploymentTruthMode {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Mirrored => "mirrored",
            Self::CachedOffline => "cached_offline",
            Self::Imported => "imported",
            Self::ProviderReported => "provider_reported",
        }
    }

    /// True when this truth class is a live first-party source rather than mirrored,
    /// cached, or imported content that must disclose its provenance.
    pub const fn is_current_source(self) -> bool {
        matches!(self, Self::Live)
    }
}

/// An install-profile card descriptor. Present only on a
/// [`M5DeploymentComponentFamily::InstallProfileCard`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallProfileCardDescriptor {
    /// Opaque ref to the install identity; never raw install bytes.
    pub install_id_ref: String,
    /// The operating mode the card renders; must match the row's deployment mode.
    pub install_mode: M5DeploymentMode,
    /// Opaque ref to the release channel.
    pub channel_ref: String,
    /// Opaque ref to the updater owner (self / managed / store).
    pub updater_owner_ref: String,
    /// Opaque ref to the durable state roots.
    pub state_root_ref: String,
    /// The card discloses its durable state roots; must always hold.
    pub discloses_state_roots: bool,
    /// The card discloses its updater owner; must always hold.
    pub discloses_updater_owner: bool,
}

impl InstallProfileCardDescriptor {
    /// Whether the install-profile card descriptor is internally complete and honest:
    /// it names install / channel / updater / state-root identity and discloses both
    /// state roots and updater ownership.
    pub fn is_honest(&self) -> bool {
        !self.install_id_ref.trim().is_empty()
            && !self.channel_ref.trim().is_empty()
            && !self.updater_owner_ref.trim().is_empty()
            && !self.state_root_ref.trim().is_empty()
            && self.discloses_state_roots
            && self.discloses_updater_owner
    }
}

/// A side-by-side import sheet descriptor. Present only on a
/// [`M5DeploymentComponentFamily::SideBySideImportSheet`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SideBySideImportSheetDescriptor {
    /// Opaque ref to the import source install.
    pub import_source_ref: String,
    /// Opaque ref to the handler-ownership record.
    pub handler_ownership_ref: String,
    /// The import silently captures a default handler; must always be false.
    pub last_writer_wins_capture: bool,
    /// The sheet discloses current handler ownership; must always hold.
    pub discloses_handler_ownership: bool,
    /// State-root isolation between installs is preserved; must always hold.
    pub isolation_preserved: bool,
}

impl SideBySideImportSheetDescriptor {
    /// Whether the side-by-side import sheet descriptor is internally complete and
    /// honest: it names its source and handler ownership, never captures a default
    /// handler, discloses ownership, and preserves isolation.
    pub fn is_honest(&self) -> bool {
        !self.import_source_ref.trim().is_empty()
            && !self.handler_ownership_ref.trim().is_empty()
            && !self.last_writer_wins_capture
            && self.discloses_handler_ownership
            && self.isolation_preserved
    }
}

/// Closed rollout-ring vocabulary. Names which rollout ring an install sits in so a
/// canary never reads as generally available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RolloutRing {
    /// The narrowest canary ring.
    Canary,
    /// An early-adopter ring.
    EarlyAdopter,
    /// A broad ring.
    Broad,
    /// General availability.
    GeneralAvailability,
    /// A paused ring holding promotion.
    Paused,
}

impl M5RolloutRing {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Canary => "canary",
            Self::EarlyAdopter => "early_adopter",
            Self::Broad => "broad",
            Self::GeneralAvailability => "general_availability",
            Self::Paused => "paused",
        }
    }
}

/// Closed rollout-promotion vocabulary. Names where the rollout is in its promotion
/// lifecycle so a held or rolled-back ring never reads as promoted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PromotionState {
    /// Held awaiting a promotion decision.
    Held,
    /// Actively promoting to the next ring.
    Promoting,
    /// Promoted to this ring.
    Promoted,
    /// Rolled back from a prior promotion.
    RolledBack,
    /// Promotion is blocked by a gate.
    Blocked,
}

impl M5PromotionState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Held => "held",
            Self::Promoting => "promoting",
            Self::Promoted => "promoted",
            Self::RolledBack => "rolled_back",
            Self::Blocked => "blocked",
        }
    }
}

/// A rollout-ring row descriptor. Present only on a
/// [`M5DeploymentComponentFamily::RolloutRingRow`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloutRingRowDescriptor {
    /// Which rollout ring this row renders.
    pub ring: M5RolloutRing,
    /// Where the rollout is in its promotion lifecycle.
    pub promotion_state: M5PromotionState,
    /// Opaque ref to the tenant / fleet scope the ring targets.
    pub target_scope_ref: String,
    /// A rollback path is available from this ring.
    pub rollback_available: bool,
    /// The row discloses its ring and promotion state; must always hold.
    pub discloses_ring: bool,
}

impl RolloutRingRowDescriptor {
    /// Whether the rollout-ring row descriptor is internally complete and honest: it
    /// names its target scope and discloses ring and promotion state.
    pub fn is_honest(&self) -> bool {
        !self.target_scope_ref.trim().is_empty() && self.discloses_ring
    }
}

/// A deployment summary card descriptor. Present only on a
/// [`M5DeploymentComponentFamily::DeploymentSummaryCard`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentSummaryCardDescriptor {
    /// Opaque ref to the tenant / region the deployment serves.
    pub tenant_region_ref: String,
    /// The operating mode the card renders; must match the row's deployment mode.
    pub operating_mode: M5DeploymentMode,
    /// The card keeps control-plane status visible; must always hold.
    pub control_plane_visible: bool,
    /// The card keeps data-plane status visible; must always hold.
    pub data_plane_visible: bool,
}

impl DeploymentSummaryCardDescriptor {
    /// Whether the deployment summary card descriptor is internally complete and
    /// honest: it names its tenant / region and keeps both planes visible.
    pub fn is_honest(&self) -> bool {
        !self.tenant_region_ref.trim().is_empty()
            && self.control_plane_visible
            && self.data_plane_visible
    }
}

/// Closed residual-dependency vocabulary. Names what kind of residual vendor dependency
/// remains so a self-hosted claim never omits it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResidualDependencyClass {
    /// A remaining license-activation dependency.
    LicenseActivation,
    /// A remaining update-delivery dependency.
    UpdateDelivery,
    /// A remaining identity-provider dependency.
    IdentityProvider,
    /// A remaining telemetry-channel dependency.
    TelemetryChannel,
    /// A remaining model / inference-service dependency.
    ModelInferenceService,
}

impl M5ResidualDependencyClass {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LicenseActivation => "license_activation",
            Self::UpdateDelivery => "update_delivery",
            Self::IdentityProvider => "identity_provider",
            Self::TelemetryChannel => "telemetry_channel",
            Self::ModelInferenceService => "model_inference_service",
        }
    }
}

/// A residual-dependency row descriptor. Present only on a
/// [`M5DeploymentComponentFamily::ResidualDependencyRow`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidualDependencyRowDescriptor {
    /// Opaque ref to the residual vendor dependency.
    pub vendor_dependency_ref: String,
    /// What kind of residual dependency remains.
    pub dependency_class: M5ResidualDependencyClass,
    /// The dependency is required for operation (versus optional / opt-in).
    pub required_for_operation: bool,
    /// The row discloses the residual dependency; must always hold.
    pub discloses_residual: bool,
}

impl ResidualDependencyRowDescriptor {
    /// Whether the residual-dependency row descriptor is internally complete and
    /// honest: it names the residual dependency and discloses it.
    pub fn is_honest(&self) -> bool {
        !self.vendor_dependency_ref.trim().is_empty() && self.discloses_residual
    }
}

/// Closed plane-state vocabulary. Names how a control or data plane is doing so an
/// impaired plane never reads as operational.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlaneState {
    /// Operational.
    Operational,
    /// Degraded (partial impairment).
    Degraded,
    /// Unavailable.
    Unavailable,
    /// State unknown (could not be determined).
    Unknown,
}

impl M5PlaneState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Operational => "operational",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
            Self::Unknown => "unknown",
        }
    }

    /// True when the plane is impaired rather than fully operational.
    pub const fn is_impaired(self) -> bool {
        matches!(self, Self::Degraded | Self::Unavailable)
    }
}

/// A control-plane/data-plane status strip descriptor. Present only on a
/// [`M5DeploymentComponentFamily::ControlPlaneDataPlaneStatusStrip`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlPlaneDataPlaneStatusStripDescriptor {
    /// The control-plane state.
    pub control_plane_state: M5PlaneState,
    /// The data-plane state.
    pub data_plane_state: M5PlaneState,
    /// Local runtime is unaffected by a control-plane impairment.
    pub local_runtime_unaffected: bool,
    /// A control-plane impairment is never masked as a local-runtime failure; must
    /// always hold.
    pub impairment_not_masked_as_local_failure: bool,
}

impl ControlPlaneDataPlaneStatusStripDescriptor {
    /// Whether the status-strip descriptor is internally complete and honest: a
    /// control-plane impairment is never masked as a local-runtime failure.
    pub fn is_honest(&self) -> bool {
        self.impairment_not_masked_as_local_failure
    }
}

/// Closed mirror-signature vocabulary. Names how a mirror artifact was verified so an
/// unverified artifact never reads as signed and current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MirrorSignatureState {
    /// Signature verified.
    Verified,
    /// Not yet verified.
    Unverified,
    /// Signature mismatch (verification failed).
    SignatureMismatch,
    /// Verification deferred (e.g. offline, pending).
    VerificationDeferred,
}

impl M5MirrorSignatureState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Unverified => "unverified",
            Self::SignatureMismatch => "signature_mismatch",
            Self::VerificationDeferred => "verification_deferred",
        }
    }
}

/// A mirror/offline artifact row descriptor. Present only on a
/// [`M5DeploymentComponentFamily::MirrorOfflineArtifactRow`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorOfflineArtifactRowDescriptor {
    /// Opaque ref to the mirror source; never a raw mirror URL.
    pub mirror_source_ref: String,
    /// The provenance/freshness class of the artifact; must match the row's truth mode.
    pub freshness: M5DeploymentTruthMode,
    /// How the artifact's signature was verified.
    pub signature_state: M5MirrorSignatureState,
    /// The row discloses the artifact's freshness; must always hold.
    pub discloses_freshness: bool,
    /// Mirrored / cached content is never shown as a current live source; required when
    /// the freshness is not a current first-party source.
    pub stale_not_shown_as_current: bool,
}

impl MirrorOfflineArtifactRowDescriptor {
    /// Whether the mirror/offline artifact row descriptor is internally complete and
    /// honest: it names its mirror source, discloses freshness, and never shows
    /// mirrored / cached content as a current live source.
    pub fn is_honest(&self) -> bool {
        if self.mirror_source_ref.trim().is_empty() || !self.discloses_freshness {
            return false;
        }
        if !self.freshness.is_current_source() && !self.stale_not_shown_as_current {
            return false;
        }
        true
    }
}

/// Closed boundary-change vocabulary. Names what durable boundary a mode change moves
/// so a state-root migration never reads as a harmless channel switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BoundaryChangeClass {
    /// A durable state-root migration.
    StateRootMigration,
    /// A release-channel switch.
    ChannelSwitch,
    /// An updater-ownership change.
    UpdaterOwnershipChange,
    /// A mirror re-attach / re-point.
    MirrorReattach,
    /// An online-to-offline (or offline-to-online) transition.
    OnlineOfflineTransition,
}

impl M5BoundaryChangeClass {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateRootMigration => "state_root_migration",
            Self::ChannelSwitch => "channel_switch",
            Self::UpdaterOwnershipChange => "updater_ownership_change",
            Self::MirrorReattach => "mirror_reattach",
            Self::OnlineOfflineTransition => "online_offline_transition",
        }
    }
}

/// A mode-change review sheet descriptor. Present only on a
/// [`M5DeploymentComponentFamily::ModeChangeReviewSheet`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeChangeReviewSheetDescriptor {
    /// The mode the install is moving from.
    pub from_mode: M5DeploymentMode,
    /// The mode the install is moving to.
    pub to_mode: M5DeploymentMode,
    /// What durable boundary the change moves.
    pub boundary_change: M5BoundaryChangeClass,
    /// The change is reviewed before the durable boundary change occurs; must hold.
    pub reviewed_before_durable_change: bool,
    /// The sheet discloses cache reuse and rollback consequences; must always hold.
    pub discloses_cache_and_rollback: bool,
}

impl ModeChangeReviewSheetDescriptor {
    /// Whether the mode-change review sheet descriptor is internally complete and
    /// honest: it reviews the change before the durable boundary change and discloses
    /// cache and rollback consequences.
    pub fn is_honest(&self) -> bool {
        self.reviewed_before_durable_change && self.discloses_cache_and_rollback
    }
}

/// A channel-association review row descriptor. Present only on a
/// [`M5DeploymentComponentFamily::ChannelAssociationReviewRow`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelAssociationReviewRowDescriptor {
    /// Opaque ref to the channel / protocol / file association.
    pub channel_ref: String,
    /// Opaque ref to the current handler association.
    pub handler_association_ref: String,
    /// The association silently captures a default handler; must always be false.
    pub last_writer_wins_capture: bool,
    /// The change is reviewed before it is applied; must always hold.
    pub reviewed_before_apply: bool,
    /// The row discloses the current owner before the change; must always hold.
    pub discloses_current_owner: bool,
}

impl ChannelAssociationReviewRowDescriptor {
    /// Whether the channel-association review row descriptor is internally complete and
    /// honest: it names its channel and handler, never captures a default handler,
    /// reviews before apply, and discloses the current owner.
    pub fn is_honest(&self) -> bool {
        !self.channel_ref.trim().is_empty()
            && !self.handler_association_ref.trim().is_empty()
            && !self.last_writer_wins_capture
            && self.reviewed_before_apply
            && self.discloses_current_owner
    }
}

/// Closed required-label vocabulary. Names the labels a reusable deployment/continuity
/// component must render; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentRequiredLabel {
    /// The component's stable identity.
    Identity,
    /// The operating / install mode the component acts in.
    OperatingMode,
    /// The updater / channel / state-root ownership or tenant / fleet scope.
    OwnershipOrScope,
    /// The provenance / freshness class.
    FreshnessClass,
    /// The rollout / continuity / plane state.
    ContinuityState,
    /// The keyboard / assistive route into the component.
    KeyboardRoute,
}

impl M5DeploymentRequiredLabel {
    /// Every required label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::OperatingMode,
        Self::OwnershipOrScope,
        Self::FreshnessClass,
        Self::ContinuityState,
        Self::KeyboardRoute,
    ];

    /// The mandatory subset that must appear on every row.
    pub const MANDATORY: [Self; 4] = [
        Self::Identity,
        Self::OperatingMode,
        Self::FreshnessClass,
        Self::KeyboardRoute,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::OperatingMode => "operating_mode",
            Self::OwnershipOrScope => "ownership_or_scope",
            Self::FreshnessClass => "freshness_class",
            Self::ContinuityState => "continuity_state",
            Self::KeyboardRoute => "keyboard_route",
        }
    }
}

/// Closed downgrade-trigger vocabulary. Names why a component row is in a degraded
/// state so support can reconstruct the narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentDowngradeTrigger {
    /// The managed control plane is impaired while the local runtime is unaffected.
    ControlPlaneImpaired,
    /// A mirror source is stale relative to the first-party source.
    MirrorStale,
    /// Only cached-offline content is available; no live source.
    OfflineCacheOnly,
    /// A mirror / artifact signature could not be verified.
    SignatureUnverified,
    /// A rollout ring is paused or held.
    RolloutPaused,
    /// Handler / channel ownership is contested across installs.
    HandlerOwnershipContested,
    /// A durable state root is unavailable or unresolved.
    StateRootUnavailable,
    /// A residual vendor dependency remains for a self-hosted install.
    ResidualVendorDependency,
    /// Provenance / freshness / scope could not be fully established.
    ProvenanceIncomplete,
}

impl M5DeploymentDowngradeTrigger {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlPlaneImpaired => "control_plane_impaired",
            Self::MirrorStale => "mirror_stale",
            Self::OfflineCacheOnly => "offline_cache_only",
            Self::SignatureUnverified => "signature_unverified",
            Self::RolloutPaused => "rollout_paused",
            Self::HandlerOwnershipContested => "handler_ownership_contested",
            Self::StateRootUnavailable => "state_root_unavailable",
            Self::ResidualVendorDependency => "residual_vendor_dependency",
            Self::ProvenanceIncomplete => "provenance_incomplete",
        }
    }
}

/// A typed degraded-state block. When present, the component is narrowed below its
/// full capability and names why with an explicit, non-generic label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradedState {
    /// Why the component is degraded.
    pub trigger: M5DeploymentDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub degraded_label: String,
}

impl DegradedState {
    /// Whether the degraded label is precise rather than a generic non-answer.
    pub fn is_honest(&self) -> bool {
        !label_is_generic(&self.degraded_label)
    }
}

/// One reusable deployment/continuity component: the shared truth row every consumer
/// surface ingests instead of cloning install / about / admin chrome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentRow {
    /// Stable component id.
    pub component_id: String,
    /// Which reusable component family this row is.
    pub family: M5DeploymentComponentFamily,
    /// Human-readable label of the surface the component appears on.
    pub surface_label: String,
    /// The provenance / freshness truth class the component binds to.
    pub truth_mode: M5DeploymentTruthMode,
    /// The operating / install mode the component acts in.
    pub deployment_mode: M5DeploymentMode,
    /// Opaque ref to the install / tenant / deployment context the component acts on;
    /// operating context stays visible on every surface, so this is never empty.
    pub operating_context_ref: String,
    /// The required labels this component renders; must include every mandatory label.
    pub required_labels: Vec<M5DeploymentRequiredLabel>,
    /// The component projects an export-safe support summary; must hold.
    pub export_safe: bool,
    /// The component exposes a keyboard / assistive route; must hold.
    pub assistive_ready: bool,
    /// The install-profile card descriptor, present only for a card row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub install_profile_card: Option<InstallProfileCardDescriptor>,
    /// The side-by-side import sheet descriptor, present only for an import-sheet row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side_by_side_import_sheet: Option<SideBySideImportSheetDescriptor>,
    /// The rollout-ring row descriptor, present only for a ring row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollout_ring_row: Option<RolloutRingRowDescriptor>,
    /// The deployment summary card descriptor, present only for a summary-card row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_summary_card: Option<DeploymentSummaryCardDescriptor>,
    /// The residual-dependency row descriptor, present only for a residual row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub residual_dependency_row: Option<ResidualDependencyRowDescriptor>,
    /// The control-plane/data-plane status strip descriptor, present only for a strip.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_plane_data_plane_status_strip: Option<ControlPlaneDataPlaneStatusStripDescriptor>,
    /// The mirror/offline artifact row descriptor, present only for a mirror row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_offline_artifact_row: Option<MirrorOfflineArtifactRowDescriptor>,
    /// The mode-change review sheet descriptor, present only for a review-sheet row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode_change_review_sheet: Option<ModeChangeReviewSheetDescriptor>,
    /// The channel-association review row descriptor, present only for an association row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_association_review_row: Option<ChannelAssociationReviewRowDescriptor>,
    /// The typed degraded-state block, present only when the component is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
    /// Human-readable label summary safe to render on the row.
    pub label_summary: String,
    /// ISO 8601 UTC timestamp the component state was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
}

impl ComponentRow {
    /// Whether the family-specific payload is present exactly for this family and
    /// absent for every other family.
    pub fn payload_matches_family(&self) -> bool {
        let present = [
            self.install_profile_card.is_some(),
            self.side_by_side_import_sheet.is_some(),
            self.rollout_ring_row.is_some(),
            self.deployment_summary_card.is_some(),
            self.residual_dependency_row.is_some(),
            self.control_plane_data_plane_status_strip.is_some(),
            self.mirror_offline_artifact_row.is_some(),
            self.mode_change_review_sheet.is_some(),
            self.channel_association_review_row.is_some(),
        ];
        // Exactly one payload present, and it is the one this family names.
        if present.iter().filter(|p| **p).count() != 1 {
            return false;
        }
        match self.family {
            M5DeploymentComponentFamily::InstallProfileCard => self.install_profile_card.is_some(),
            M5DeploymentComponentFamily::SideBySideImportSheet => {
                self.side_by_side_import_sheet.is_some()
            }
            M5DeploymentComponentFamily::RolloutRingRow => self.rollout_ring_row.is_some(),
            M5DeploymentComponentFamily::DeploymentSummaryCard => {
                self.deployment_summary_card.is_some()
            }
            M5DeploymentComponentFamily::ResidualDependencyRow => {
                self.residual_dependency_row.is_some()
            }
            M5DeploymentComponentFamily::ControlPlaneDataPlaneStatusStrip => {
                self.control_plane_data_plane_status_strip.is_some()
            }
            M5DeploymentComponentFamily::MirrorOfflineArtifactRow => {
                self.mirror_offline_artifact_row.is_some()
            }
            M5DeploymentComponentFamily::ModeChangeReviewSheet => {
                self.mode_change_review_sheet.is_some()
            }
            M5DeploymentComponentFamily::ChannelAssociationReviewRow => {
                self.channel_association_review_row.is_some()
            }
        }
    }

    /// Whether the family payload, where present, is internally honest.
    pub fn payload_honest(&self) -> bool {
        self.install_profile_card
            .as_ref()
            .map_or(true, |d| d.is_honest())
            && self
                .side_by_side_import_sheet
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .rollout_ring_row
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .deployment_summary_card
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .residual_dependency_row
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .control_plane_data_plane_status_strip
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .mirror_offline_artifact_row
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .mode_change_review_sheet
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .channel_association_review_row
                .as_ref()
                .map_or(true, |d| d.is_honest())
    }

    /// Whether a mode-bearing or freshness-bearing descriptor discloses the same
    /// operating mode and truth class the row records (a card / strip / mirror row
    /// never invents a second deployment story).
    pub fn descriptor_matches_row(&self) -> bool {
        let install_ok = self
            .install_profile_card
            .as_ref()
            .map_or(true, |c| c.install_mode == self.deployment_mode);
        let summary_ok = self
            .deployment_summary_card
            .as_ref()
            .map_or(true, |c| c.operating_mode == self.deployment_mode);
        let mirror_ok = self
            .mirror_offline_artifact_row
            .as_ref()
            .map_or(true, |m| m.freshness == self.truth_mode);
        install_ok && summary_ok && mirror_ok
    }

    /// Whether every mandatory required label is present on the row.
    pub fn mandatory_labels_present(&self) -> bool {
        let present: BTreeSet<M5DeploymentRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5DeploymentRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the degraded block, when present, is honest.
    pub fn degraded_ok(&self) -> bool {
        self.degraded.as_ref().map_or(true, |d| d.is_honest())
    }

    /// True when this row is a complete, honest degraded / narrowed component.
    pub fn is_degraded(&self) -> bool {
        self.degraded.is_some() && self.is_complete()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} truth={truth} mode={mode} \
export_safe={export_safe} assistive={assistive}",
            family = self.family.as_str(),
            truth = self.truth_mode.as_str(),
            mode = self.deployment_mode.as_str(),
            export_safe = self.export_safe,
            assistive = self.assistive_ready,
        )
    }

    /// Whether every dimension required to record this row is present and internally
    /// consistent.
    pub fn is_complete(&self) -> bool {
        !self.component_id.trim().is_empty()
            && !self.surface_label.trim().is_empty()
            && !self.operating_context_ref.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.observed_at.trim().is_empty()
            && self.export_safe
            && self.assistive_ready
            && self.payload_matches_family()
            && self.payload_honest()
            && self.descriptor_matches_row()
            && self.mandatory_labels_present()
            && self.degraded_ok()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block for the deployment/continuity component matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentContinuityGuardrails {
    /// Install mode, channel, updater owner, state roots, rollout ring, tenant / region,
    /// mirror source, and residual dependency stay explicit on every surface.
    pub operating_truth_explicit_on_every_surface: bool,
    /// Side-by-side and portable installs never devolve into last-writer-wins handler
    /// capture.
    pub no_last_writer_wins_handler_capture: bool,
    /// Control-plane impairment never masquerades as local-runtime failure.
    pub control_plane_impairment_never_masked_as_local: bool,
    /// Mirror / offline transitions disclose freshness and never show stale content as
    /// current.
    pub mirror_offline_freshness_never_shown_as_current: bool,
    /// Self-hosted claims never omit residual vendor dependency.
    pub self_hosted_never_omits_residual_dependency: bool,
    /// Mode switches / cache reuse / rollback are reviewed before durable boundary
    /// changes occur.
    pub mode_changes_reviewed_before_durable_change: bool,
    /// Exported evidence preserves the same install / channel / mirror IDs, modes, and
    /// states shown in-product.
    pub exported_evidence_preserves_ids_modes_and_states: bool,
    /// Components bind to the shared install-mode, provenance / freshness, client-scope,
    /// and degraded-state vocabulary rather than bespoke installer / admin chrome.
    pub components_bound_to_shared_vocabulary: bool,
    /// The matrix does not widen into new installers, rollout engines, managed services,
    /// or mirror protocols.
    pub no_new_installers_engines_or_protocols: bool,
}

impl DeploymentContinuityGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.operating_truth_explicit_on_every_surface
            && self.no_last_writer_wins_handler_capture
            && self.control_plane_impairment_never_masked_as_local
            && self.mirror_offline_freshness_never_shown_as_current
            && self.self_hosted_never_omits_residual_dependency
            && self.mode_changes_reviewed_before_durable_change
            && self.exported_evidence_preserves_ids_modes_and_states
            && self.components_bound_to_shared_vocabulary
            && self.no_new_installers_engines_or_protocols
    }
}

/// Consumer-projection block for the deployment/continuity component matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentContinuityConsumerProjection {
    /// Product surfaces (About / install / update) ingest these component rows instead
    /// of cloning chrome.
    pub product_ingests_components: bool,
    /// Docs / help ingests the same component rows.
    pub docs_help_ingests_components: bool,
    /// Diagnostics ingests the same component rows.
    pub diagnostics_ingests_components: bool,
    /// Admin ingests the same component rows.
    pub admin_ingests_components: bool,
    /// Support export ingests the same component rows.
    pub support_export_ingests_components: bool,
    /// Release-control surfaces ingest the same component rows.
    pub release_control_ingests_components: bool,
    /// Later M5 rows reference one canonical component family instead of restating
    /// install / deployment truth in feature-local prose.
    pub later_rows_reference_one_canonical_family: bool,
}

impl DeploymentContinuityConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_components
            && self.docs_help_ingests_components
            && self.diagnostics_ingests_components
            && self.admin_ingests_components
            && self.support_export_ingests_components
            && self.release_control_ingests_components
            && self.later_rows_reference_one_canonical_family
    }
}

/// Constructor input for [`DeploymentContinuityComponentMatrix::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentContinuityComponentMatrixInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub set_label: String,
    /// Per-component rows.
    pub components: Vec<ComponentRow>,
    /// Guardrail invariants block.
    pub guardrails: DeploymentContinuityGuardrails,
    /// Consumer projection block.
    pub consumer_projection: DeploymentContinuityConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe deployment/continuity component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentContinuityComponentMatrix {
    /// Record kind; must equal [`DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub set_label: String,
    /// Per-component rows.
    pub components: Vec<ComponentRow>,
    /// Guardrail invariants block.
    pub guardrails: DeploymentContinuityGuardrails,
    /// Consumer projection block.
    pub consumer_projection: DeploymentContinuityConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl DeploymentContinuityComponentMatrix {
    /// Builds a deployment/continuity component matrix packet.
    pub fn new(input: DeploymentContinuityComponentMatrixInput) -> Self {
        Self {
            record_kind: DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            set_label: input.set_label,
            components: input.components,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Families represented by some row in this matrix.
    pub fn represented_families(&self) -> BTreeSet<M5DeploymentComponentFamily> {
        self.components.iter().map(|r| r.family).collect()
    }

    /// Count of rows that are complete, honest degraded / narrowed components.
    pub fn degraded_row_count(&self) -> usize {
        self.components.iter().filter(|r| r.is_degraded()).count()
    }

    /// Validates the deployment/continuity component matrix invariants.
    pub fn validate(&self) -> Vec<DeploymentContinuityComponentViolation> {
        let mut violations = Vec::new();

        if self.record_kind != DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(DeploymentContinuityComponentViolation::WrongRecordKind);
        }
        if self.schema_version != DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(DeploymentContinuityComponentViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.set_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(DeploymentContinuityComponentViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("deployment/continuity component matrix serializes"),
        ) {
            violations.push(DeploymentContinuityComponentViolation::RawBoundaryMaterialInExport);
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
            .expect("deployment/continuity component matrix serializes")
    }

    /// Deterministic CSV of the component rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "component_id,family,truth_mode,deployment_mode,export_safe,assistive_ready,degraded\n",
        );
        for row in &self.components {
            out.push_str(&format!(
                "{id},{family},{truth},{mode},{export_safe},{assistive},{degraded}\n",
                id = row.component_id,
                family = row.family.as_str(),
                truth = row.truth_mode.as_str(),
                mode = row.deployment_mode.as_str(),
                export_safe = row.export_safe,
                assistive = row.assistive_ready,
                degraded = row.degraded.as_ref().map_or("none", |d| d.trigger.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Deployment/Continuity Component Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.set_label));
        out.push_str(&format!(
            "- Components: {} across {} / {} families ({} degraded)\n",
            self.components.len(),
            self.represented_families().len(),
            M5DeploymentComponentFamily::ALL.len(),
            self.degraded_row_count(),
        ));
        out.push_str("\n## Components\n\n");
        for row in &self.components {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.component_id,
                row.family.as_str(),
                row.surface_label,
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!("  - {}\n", row.chip_tokens()));
            if let Some(degraded) = &row.degraded {
                out.push_str(&format!(
                    "  - Degraded: trigger={} — {}\n",
                    degraded.trigger.as_str(),
                    degraded.degraded_label,
                ));
            }
        }
        out
    }
}

/// Claimed M5 deployment surface that must consume the shared component family instead
/// of inheriting healthier-lane labels from another surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ClaimedDeploymentSurface {
    /// Local-only desktop operation.
    LocalOnly,
    /// Managed online deployment.
    Managed,
    /// Customer self-hosted deployment.
    SelfHosted,
    /// Deployment served through a mirror.
    Mirrored,
    /// Sovereign / regulated deployment.
    Sovereign,
    /// Air-gapped deployment.
    AirGapped,
    /// Side-by-side channel coexistence.
    SideBySide,
    /// Portable install / portable state package.
    Portable,
    /// Fleet rollout surface.
    FleetRollout,
}

impl M5ClaimedDeploymentSurface {
    /// Every claimed surface in the order release evidence reports them.
    pub const ALL: [Self; 9] = [
        Self::LocalOnly,
        Self::Managed,
        Self::SelfHosted,
        Self::Mirrored,
        Self::Sovereign,
        Self::AirGapped,
        Self::SideBySide,
        Self::Portable,
        Self::FleetRollout,
    ];

    /// Stable token recorded in certification rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Managed => "managed",
            Self::SelfHosted => "self_hosted",
            Self::Mirrored => "mirrored",
            Self::Sovereign => "sovereign",
            Self::AirGapped => "air_gapped",
            Self::SideBySide => "side_by_side",
            Self::Portable => "portable",
            Self::FleetRollout => "fleet_rollout",
        }
    }
}

/// Required drill family for each claimed deployment surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentCertificationDrillKind {
    /// Functional behavior drill.
    Functional,
    /// Accessibility / non-visual parity drill.
    Accessibility,
    /// Support / release export reconstruction drill.
    Export,
    /// Failure / recovery degradation drill.
    Degradation,
}

impl M5DeploymentCertificationDrillKind {
    /// Every required drill family.
    pub const ALL: [Self; 4] = [
        Self::Functional,
        Self::Accessibility,
        Self::Export,
        Self::Degradation,
    ];

    /// Stable token recorded in certification rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Functional => "functional",
            Self::Accessibility => "accessibility",
            Self::Export => "export",
            Self::Degradation => "degradation",
        }
    }
}

/// Result of a required certification drill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentDrillOutcome {
    /// Drill passed.
    Passed,
    /// Drill failed and must narrow the claim.
    Failed,
}

impl M5DeploymentDrillOutcome {
    /// True when this drill result is passing.
    pub const fn is_passed(self) -> bool {
        matches!(self, Self::Passed)
    }
}

/// Compatibility dimension that must carry explicit notes for every claimed surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentCompatibilityDimension {
    /// Channel ownership truth.
    ChannelOwnership,
    /// Handler precedence truth.
    HandlerPrecedence,
    /// Mirror / offline freshness truth.
    MirrorOfflineFreshness,
    /// Control-plane/data-plane continuity truth.
    ControlPlaneDataPlaneContinuity,
}

impl M5DeploymentCompatibilityDimension {
    /// Every compatibility dimension the certification packet must capture.
    pub const ALL: [Self; 4] = [
        Self::ChannelOwnership,
        Self::HandlerPrecedence,
        Self::MirrorOfflineFreshness,
        Self::ControlPlaneDataPlaneContinuity,
    ];

    /// Stable token recorded in certification rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChannelOwnership => "channel_ownership",
            Self::HandlerPrecedence => "handler_precedence",
            Self::MirrorOfflineFreshness => "mirror_offline_freshness",
            Self::ControlPlaneDataPlaneContinuity => "control_plane_data_plane_continuity",
        }
    }
}

/// Compatibility posture for a claimed surface / dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentCompatibilityPosture {
    /// Current and fully compatible.
    Current,
    /// Compatible but degraded; the public claim must narrow.
    DegradedNarrowed,
    /// Unsupported for this surface; the public claim must narrow.
    UnsupportedNarrowed,
}

impl M5DeploymentCompatibilityPosture {
    /// True when this posture can keep a full-truth label.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// Published claim label after certification and auto-narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentClaimLabel {
    /// Full truth parity is current.
    FullTruth,
    /// The claim is supported but narrowed because a dependency, freshness, or plane
    /// state is degraded.
    DegradedNarrowed,
    /// The surface is local-safe only for the affected behavior.
    LocalSafeOnly,
    /// The path is unsupported and must not inherit a full-truth label.
    UnsupportedNarrowed,
}

impl M5DeploymentClaimLabel {
    /// True when this label is the full claim.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::FullTruth)
    }

    /// Stable token recorded in certification rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullTruth => "full_truth",
            Self::DegradedNarrowed => "degraded_narrowed",
            Self::LocalSafeOnly => "local_safe_only",
            Self::UnsupportedNarrowed => "unsupported_narrowed",
        }
    }
}

/// One drill result for one claimed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentContinuityDrillResult {
    /// Required drill kind.
    pub drill_kind: M5DeploymentCertificationDrillKind,
    /// Drill outcome.
    pub outcome: M5DeploymentDrillOutcome,
    /// Evidence ref for the drill.
    pub evidence_ref: String,
}

impl M5DeploymentContinuityDrillResult {
    /// Whether the drill row is complete and passing.
    pub fn is_complete_and_passing(&self) -> bool {
        self.outcome.is_passed() && !self.evidence_ref.trim().is_empty()
    }
}

/// One compatibility note for one claimed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentContinuityCompatibilityNote {
    /// Compatibility dimension.
    pub dimension: M5DeploymentCompatibilityDimension,
    /// Compatibility posture.
    pub posture: M5DeploymentCompatibilityPosture,
    /// Export-safe summary of the compatibility result.
    pub summary: String,
    /// Optional auto-narrowing trigger when posture is not current.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_narrowing_trigger: Option<M5DeploymentDowngradeTrigger>,
    /// Evidence ref backing this note.
    pub evidence_ref: String,
}

impl M5DeploymentContinuityCompatibilityNote {
    /// Whether the compatibility note is complete and honestly narrowed.
    pub fn is_complete(&self) -> bool {
        !self.summary.trim().is_empty()
            && !label_is_generic(&self.summary)
            && !self.evidence_ref.trim().is_empty()
            && (self.posture.is_current() || self.auto_narrowing_trigger.is_some())
    }
}

/// One claimed deployment surface certified against the shared component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentContinuitySurfaceCertificationRow {
    /// Claimed surface.
    pub surface: M5ClaimedDeploymentSurface,
    /// Component families consumed by this surface.
    pub component_families: Vec<M5DeploymentComponentFamily>,
    /// Component row refs consumed by this surface.
    pub component_refs: Vec<String>,
    /// Required drill results.
    pub drills: Vec<M5DeploymentContinuityDrillResult>,
    /// Compatibility notes for the required dimensions.
    pub compatibility_notes: Vec<M5DeploymentContinuityCompatibilityNote>,
    /// Label originally claimed by the surface.
    pub claimed_label: M5DeploymentClaimLabel,
    /// Label after certification and auto-narrowing.
    pub effective_label: M5DeploymentClaimLabel,
    /// True when the effective label was automatically narrowed.
    pub auto_narrowed: bool,
    /// Human-readable, export-safe narrowing reasons.
    pub narrowing_reasons: Vec<String>,
    /// The surface consumes shared component rows instead of feature-local chrome.
    pub consumes_shared_component_family: bool,
    /// Unsupported or degraded paths are visible.
    pub unsupported_or_degraded_visible: bool,
    /// Local-safe continuity is visible where applicable.
    pub local_safe_continuity_visible: bool,
}

impl M5DeploymentContinuitySurfaceCertificationRow {
    /// Dimensions represented by the row's compatibility notes.
    pub fn represented_dimensions(&self) -> BTreeSet<M5DeploymentCompatibilityDimension> {
        self.compatibility_notes
            .iter()
            .map(|note| note.dimension)
            .collect()
    }

    /// Drill kinds represented by the row's drill results.
    pub fn represented_drills(&self) -> BTreeSet<M5DeploymentCertificationDrillKind> {
        self.drills.iter().map(|drill| drill.drill_kind).collect()
    }

    /// Whether every drill passed.
    pub fn all_drills_passed(&self) -> bool {
        self.drills
            .iter()
            .all(M5DeploymentContinuityDrillResult::is_complete_and_passing)
    }

    /// Whether every compatibility note is current.
    pub fn all_compatibility_current(&self) -> bool {
        self.compatibility_notes
            .iter()
            .all(|note| note.posture.is_current())
    }

    /// Whether the row should carry a narrowed effective label.
    pub fn should_be_narrowed(&self) -> bool {
        !self.all_drills_passed() || !self.all_compatibility_current()
    }

    /// Whether all required dimensions and drill kinds are present.
    pub fn coverage_complete(&self) -> bool {
        let drills = self.represented_drills();
        let dimensions = self.represented_dimensions();
        M5DeploymentCertificationDrillKind::ALL
            .iter()
            .all(|kind| drills.contains(kind))
            && M5DeploymentCompatibilityDimension::ALL
                .iter()
                .all(|dimension| dimensions.contains(dimension))
    }

    /// Whether row-level claim narrowing is consistent with drill and compatibility
    /// posture.
    pub fn narrowing_consistent(&self) -> bool {
        let should_narrow = self.should_be_narrowed();
        if should_narrow {
            self.auto_narrowed
                && !self.effective_label.is_full()
                && !self.narrowing_reasons.is_empty()
                && self
                    .narrowing_reasons
                    .iter()
                    .all(|reason| !reason.trim().is_empty() && !label_is_generic(reason))
        } else {
            !self.auto_narrowed
                && self.effective_label.is_full()
                && self.narrowing_reasons.is_empty()
        }
    }

    /// Whether the row is complete enough to certify the claimed surface.
    pub fn is_complete(&self) -> bool {
        !self.component_families.is_empty()
            && !self.component_refs.is_empty()
            && self.component_refs.iter().all(|r| !r.trim().is_empty())
            && self.coverage_complete()
            && self
                .drills
                .iter()
                .all(|drill| !drill.evidence_ref.trim().is_empty())
            && self
                .compatibility_notes
                .iter()
                .all(|note| note.is_complete())
            && self.consumes_shared_component_family
            && self.unsupported_or_degraded_visible
            && self.local_safe_continuity_visible
            && self.narrowing_consistent()
    }
}

/// Release/support proof block for the surface certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentContinuityReleaseSupportProof {
    /// Functional drills are complete.
    pub functional_drills_complete: bool,
    /// Accessibility drills are complete.
    pub accessibility_drills_complete: bool,
    /// Export drills are complete.
    pub export_drills_complete: bool,
    /// Degradation drills are complete.
    pub degradation_drills_complete: bool,
    /// Compatibility notes cover required dimensions.
    pub compatibility_notes_complete: bool,
    /// Release/support proof packet is published.
    pub proof_packet_published: bool,
    /// Packet is stable enough to gate later M5 widening.
    pub later_m5_gating_asset: bool,
    /// Support/export refs that field teams can use.
    pub support_export_refs: Vec<String>,
}

impl M5DeploymentContinuityReleaseSupportProof {
    /// Whether all release/support proof invariants hold.
    pub fn all_hold(&self) -> bool {
        self.functional_drills_complete
            && self.accessibility_drills_complete
            && self.export_drills_complete
            && self.degradation_drills_complete
            && self.compatibility_notes_complete
            && self.proof_packet_published
            && self.later_m5_gating_asset
            && !self.support_export_refs.is_empty()
            && self
                .support_export_refs
                .iter()
                .all(|r| !r.trim().is_empty())
    }
}

/// Constructor input for [`M5DeploymentContinuitySurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DeploymentContinuitySurfaceCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Matrix packet ref.
    pub component_matrix_packet_ref: String,
    /// Per-surface rows.
    pub surface_rows: Vec<M5DeploymentContinuitySurfaceCertificationRow>,
    /// Release/support proof block.
    pub release_support_proof: M5DeploymentContinuityReleaseSupportProof,
    /// Source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Redaction token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

/// Certification packet proving shared deployment/continuity components across every
/// claimed M5 deployment surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentContinuitySurfaceCertificationPacket {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Matrix packet ref this packet certifies.
    pub component_matrix_packet_ref: String,
    /// Per-surface rows.
    pub surface_rows: Vec<M5DeploymentContinuitySurfaceCertificationRow>,
    /// Release/support proof block.
    pub release_support_proof: M5DeploymentContinuityReleaseSupportProof,
    /// Source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Redaction token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

/// Stable record-kind tag carried by
/// [`M5DeploymentContinuitySurfaceCertificationPacket`].
pub const M5_DEPLOYMENT_CONTINUITY_SURFACE_CERTIFICATION_RECORD_KIND: &str =
    "m5_deployment_continuity_surface_certification";

/// Schema version for the surface certification packet.
pub const M5_DEPLOYMENT_CONTINUITY_SURFACE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

impl M5DeploymentContinuitySurfaceCertificationPacket {
    /// Builds the certification packet.
    pub fn new(input: M5DeploymentContinuitySurfaceCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_DEPLOYMENT_CONTINUITY_SURFACE_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_DEPLOYMENT_CONTINUITY_SURFACE_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            component_matrix_packet_ref: input.component_matrix_packet_ref,
            surface_rows: input.surface_rows,
            release_support_proof: input.release_support_proof,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Claimed surfaces represented by the packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5ClaimedDeploymentSurface> {
        self.surface_rows.iter().map(|row| row.surface).collect()
    }

    /// Count of rows that auto-narrowed.
    pub fn narrowed_row_count(&self) -> usize {
        self.surface_rows
            .iter()
            .filter(|row| row.auto_narrowed)
            .count()
    }

    /// Validates the certification packet.
    pub fn validate(&self) -> Vec<M5DeploymentContinuitySurfaceCertificationViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_DEPLOYMENT_CONTINUITY_SURFACE_CERTIFICATION_RECORD_KIND {
            violations.push(M5DeploymentContinuitySurfaceCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DEPLOYMENT_CONTINUITY_SURFACE_CERTIFICATION_SCHEMA_VERSION {
            violations
                .push(M5DeploymentContinuitySurfaceCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.component_matrix_packet_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DeploymentContinuitySurfaceCertificationViolation::MissingIdentity);
        }
        validate_certification_source_contracts(self, &mut violations);
        validate_certification_surface_coverage(self, &mut violations);
        validate_certification_rows(self, &mut violations);
        if !self.release_support_proof.all_hold() {
            violations.push(
                M5DeploymentContinuitySurfaceCertificationViolation::ReleaseSupportProofIncomplete,
            );
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("deployment/continuity certification packet serializes"),
        ) {
            violations.push(
                M5DeploymentContinuitySurfaceCertificationViolation::RawBoundaryMaterialInExport,
            );
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
            .expect("deployment/continuity certification packet serializes")
    }

    /// Deterministic CSV projection.
    pub fn render_certification_csv(&self) -> String {
        let mut out = String::from(
            "surface,claimed_label,effective_label,auto_narrowed,component_family_count,drill_count,narrowing_reason_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{surface},{claimed},{effective},{auto},{components},{drills},{reasons}\n",
                surface = row.surface.as_str(),
                claimed = row.claimed_label.as_str(),
                effective = row.effective_label.as_str(),
                auto = row.auto_narrowed,
                components = row.component_families.len(),
                drills = row.drills.len(),
                reasons = row.narrowing_reasons.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report.
    pub fn render_certification_report(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Deployment/Continuity Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Matrix: `{}`\n",
            self.component_matrix_packet_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} ({} narrowed)\n\n",
            self.represented_surfaces().len(),
            M5ClaimedDeploymentSurface::ALL.len(),
            self.narrowed_row_count(),
        ));
        out.push_str("## Surface Rows\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}**: claimed={} effective={} auto_narrowed={}\n",
                row.surface.as_str(),
                row.claimed_label.as_str(),
                row.effective_label.as_str(),
                row.auto_narrowed,
            ));
            if !row.narrowing_reasons.is_empty() {
                out.push_str(&format!(
                    "  - Narrowing: {}\n",
                    row.narrowing_reasons.join("; ")
                ));
            }
        }
        out
    }
}

/// Validation failures emitted by
/// [`M5DeploymentContinuitySurfaceCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DeploymentContinuitySurfaceCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required source contract refs are incomplete.
    MissingSourceContracts,
    /// A claimed deployment surface is missing.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row is missing a required drill family.
    DrillCoverageMissing,
    /// A surface row is missing a required compatibility dimension.
    CompatibilityCoverageMissing,
    /// A degraded / unsupported row did not narrow visibly, or a healthy row narrowed.
    ClaimNarrowingInconsistent,
    /// The packet has no narrowed row, so it does not prove degradation behavior.
    NoNarrowedRows,
    /// The release/support proof block is incomplete.
    ReleaseSupportProofIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5DeploymentContinuitySurfaceCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::DrillCoverageMissing => "drill_coverage_missing",
            Self::CompatibilityCoverageMissing => "compatibility_coverage_missing",
            Self::ClaimNarrowingInconsistent => "claim_narrowing_inconsistent",
            Self::NoNarrowedRows => "no_narrowed_rows",
            Self::ReleaseSupportProofIncomplete => "release_support_proof_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Errors emitted when reading the checked-in deployment/continuity component export.
#[derive(Debug)]
pub enum DeploymentContinuityComponentArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<DeploymentContinuityComponentViolation>),
    /// Surface certification export failed to parse.
    SurfaceCertification(serde_json::Error),
    /// Surface certification export failed validation.
    SurfaceCertificationValidation(Vec<M5DeploymentContinuitySurfaceCertificationViolation>),
}

impl fmt::Display for DeploymentContinuityComponentArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "deployment/continuity component export parse failed: {error}"
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
                    "deployment/continuity component export failed validation: {tokens}"
                )
            }
            Self::SurfaceCertification(error) => write!(
                formatter,
                "deployment/continuity surface certification parse failed: {error}"
            ),
            Self::SurfaceCertificationValidation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "deployment/continuity surface certification failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DeploymentContinuityComponentArtifactError {}

/// Validation failures emitted by [`DeploymentContinuityComponentMatrix::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeploymentContinuityComponentViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required reusable component family is defined by no row.
    RequiredFamilyMissing,
    /// The matrix demonstrates no complete degraded / narrowed row.
    DegradedCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A row's family-specific payload is missing, extra, or wrong for its family.
    PayloadFamilyMismatch,
    /// A row's family payload is internally dishonest.
    PayloadDishonest,
    /// A mode- or freshness-bearing descriptor discloses a mode / class different from
    /// its row.
    DescriptorRowMismatch,
    /// A row omits a mandatory required label.
    MandatoryLabelMissing,
    /// A row is not export-safe or not assistive-ready.
    ParityMissing,
    /// A degraded block carries a generic non-answer label.
    DegradedLabelGeneric,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl DeploymentContinuityComponentViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::DegradedCaseMissing => "degraded_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::PayloadFamilyMismatch => "payload_family_mismatch",
            Self::PayloadDishonest => "payload_dishonest",
            Self::DescriptorRowMismatch => "descriptor_row_mismatch",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ParityMissing => "parity_missing",
            Self::DegradedLabelGeneric => "degraded_label_generic",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in deployment/continuity component export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_m5_deployment_continuity_component_matrix_export(
) -> Result<DeploymentContinuityComponentMatrix, DeploymentContinuityComponentArtifactError> {
    let packet: DeploymentContinuityComponentMatrix = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-deployment-continuity-component-proof/support_export.json"
    )))
    .map_err(DeploymentContinuityComponentArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DeploymentContinuityComponentArtifactError::Validation(
            violations,
        ))
    }
}

/// Reads and validates the checked-in deployment/continuity surface certification
/// export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_m5_deployment_continuity_surface_certification_export() -> Result<
    M5DeploymentContinuitySurfaceCertificationPacket,
    DeploymentContinuityComponentArtifactError,
> {
    let packet: M5DeploymentContinuitySurfaceCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-deployment-continuity-component-proof/certification.json"
        )))
        .map_err(DeploymentContinuityComponentArtifactError::SurfaceCertification)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DeploymentContinuityComponentArtifactError::SurfaceCertificationValidation(violations))
    }
}

fn validate_source_contracts(
    packet: &DeploymentContinuityComponentMatrix,
    violations: &mut Vec<DeploymentContinuityComponentViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_SCHEMA_REF,
        DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_DOC_REF,
        DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(DeploymentContinuityComponentViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &DeploymentContinuityComponentMatrix,
    violations: &mut Vec<DeploymentContinuityComponentViolation>,
) {
    let families = packet.represented_families();
    for required in M5DeploymentComponentFamily::ALL {
        if !families.contains(&required) {
            violations.push(DeploymentContinuityComponentViolation::RequiredFamilyMissing);
            break;
        }
    }
    if packet.degraded_row_count() == 0 {
        violations.push(DeploymentContinuityComponentViolation::DegradedCaseMissing);
    }
}

fn validate_rows(
    packet: &DeploymentContinuityComponentMatrix,
    violations: &mut Vec<DeploymentContinuityComponentViolation>,
) {
    for row in &packet.components {
        if !row.is_complete() {
            violations.push(DeploymentContinuityComponentViolation::RowIncomplete);
        }
        if !row.payload_matches_family() {
            violations.push(DeploymentContinuityComponentViolation::PayloadFamilyMismatch);
        }
        if !row.payload_honest() {
            violations.push(DeploymentContinuityComponentViolation::PayloadDishonest);
        }
        if !row.descriptor_matches_row() {
            violations.push(DeploymentContinuityComponentViolation::DescriptorRowMismatch);
        }
        if !row.mandatory_labels_present() {
            violations.push(DeploymentContinuityComponentViolation::MandatoryLabelMissing);
        }
        if !row.export_safe || !row.assistive_ready {
            violations.push(DeploymentContinuityComponentViolation::ParityMissing);
        }
        if !row.degraded_ok() {
            violations.push(DeploymentContinuityComponentViolation::DegradedLabelGeneric);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(DeploymentContinuityComponentViolation::RowEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &DeploymentContinuityComponentMatrix,
    violations: &mut Vec<DeploymentContinuityComponentViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(DeploymentContinuityComponentViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &DeploymentContinuityComponentMatrix,
    violations: &mut Vec<DeploymentContinuityComponentViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(DeploymentContinuityComponentViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_certification_source_contracts(
    packet: &M5DeploymentContinuitySurfaceCertificationPacket,
    violations: &mut Vec<M5DeploymentContinuitySurfaceCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_SCHEMA_REF,
        DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_DOC_REF,
        DEPLOYMENT_CONTINUITY_COMPONENT_CERTIFICATION_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(M5DeploymentContinuitySurfaceCertificationViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_certification_surface_coverage(
    packet: &M5DeploymentContinuitySurfaceCertificationPacket,
    violations: &mut Vec<M5DeploymentContinuitySurfaceCertificationViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in M5ClaimedDeploymentSurface::ALL {
        if !surfaces.contains(&required) {
            violations
                .push(M5DeploymentContinuitySurfaceCertificationViolation::RequiredSurfaceMissing);
            break;
        }
    }
    if packet.narrowed_row_count() == 0 {
        violations.push(M5DeploymentContinuitySurfaceCertificationViolation::NoNarrowedRows);
    }
}

fn validate_certification_rows(
    packet: &M5DeploymentContinuitySurfaceCertificationPacket,
    violations: &mut Vec<M5DeploymentContinuitySurfaceCertificationViolation>,
) {
    for row in &packet.surface_rows {
        if !row.is_complete() {
            violations
                .push(M5DeploymentContinuitySurfaceCertificationViolation::SurfaceRowIncomplete);
        }
        let drills = row.represented_drills();
        if !M5DeploymentCertificationDrillKind::ALL
            .iter()
            .all(|kind| drills.contains(kind))
        {
            violations
                .push(M5DeploymentContinuitySurfaceCertificationViolation::DrillCoverageMissing);
        }
        let dimensions = row.represented_dimensions();
        if !M5DeploymentCompatibilityDimension::ALL
            .iter()
            .all(|dimension| dimensions.contains(dimension))
        {
            violations.push(
                M5DeploymentContinuitySurfaceCertificationViolation::CompatibilityCoverageMissing,
            );
        }
        if !row.narrowing_consistent() {
            violations.push(
                M5DeploymentContinuitySurfaceCertificationViolation::ClaimNarrowingInconsistent,
            );
        }
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "stale"
            | "no data"
            | "blocked"
            | "degraded"
            | "offline"
            | "impaired"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds the canonical, checked-in deployment/continuity component matrix packet. This
/// is the one source of truth shared by the tests and the on-disk support export so
/// both stay byte-aligned.
pub fn seeded_deployment_continuity_component_matrix() -> DeploymentContinuityComponentMatrix {
    DeploymentContinuityComponentMatrix::new(DeploymentContinuityComponentMatrixInput {
        packet_id: "m5-deployment-continuity-component-matrix:stable:0001".to_owned(),
        set_label: "M5 Deployment/Continuity Component Matrix".to_owned(),
        components: seeded_components(),
        guardrails: seeded_guardrails(),
        consumer_projection: seeded_consumer_projection(),
        source_contract_refs: seeded_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-04T00:00:00Z".to_owned(),
    })
}

/// Builds the canonical certification packet proving every claimed M5 deployment
/// surface consumes the shared component family and narrows visibly when parity is
/// degraded.
pub fn seeded_deployment_continuity_surface_certification(
) -> M5DeploymentContinuitySurfaceCertificationPacket {
    M5DeploymentContinuitySurfaceCertificationPacket::new(
        M5DeploymentContinuitySurfaceCertificationPacketInput {
            packet_id: "m5-deployment-continuity-surface-certification:stable:0001".to_owned(),
            packet_label: "M5 Deployment/Continuity Surface Certification".to_owned(),
            component_matrix_packet_ref: "m5-deployment-continuity-component-matrix:stable:0001"
                .to_owned(),
            surface_rows: seeded_surface_certification_rows(),
            release_support_proof: seeded_release_support_proof(),
            source_contract_refs: seeded_certification_source_contract_refs(),
            redaction_class_token: "metadata_safe_default".to_owned(),
            minted_at: "2026-07-05T00:00:00Z".to_owned(),
        },
    )
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:deployment-continuity:{id}")]
}

fn drill(
    surface: M5ClaimedDeploymentSurface,
    kind: M5DeploymentCertificationDrillKind,
) -> M5DeploymentContinuityDrillResult {
    M5DeploymentContinuityDrillResult {
        drill_kind: kind,
        outcome: M5DeploymentDrillOutcome::Passed,
        evidence_ref: format!(
            "evidence:deployment-continuity:{}:{}",
            surface.as_str(),
            kind.as_str()
        ),
    }
}

fn all_drills(surface: M5ClaimedDeploymentSurface) -> Vec<M5DeploymentContinuityDrillResult> {
    M5DeploymentCertificationDrillKind::ALL
        .iter()
        .map(|kind| drill(surface, *kind))
        .collect()
}

fn note(
    surface: M5ClaimedDeploymentSurface,
    dimension: M5DeploymentCompatibilityDimension,
    posture: M5DeploymentCompatibilityPosture,
    summary: &str,
    trigger: Option<M5DeploymentDowngradeTrigger>,
) -> M5DeploymentContinuityCompatibilityNote {
    M5DeploymentContinuityCompatibilityNote {
        dimension,
        posture,
        summary: summary.to_owned(),
        auto_narrowing_trigger: trigger,
        evidence_ref: format!(
            "evidence:deployment-continuity:{}:{}",
            surface.as_str(),
            dimension.as_str()
        ),
    }
}

fn current_notes(
    surface: M5ClaimedDeploymentSurface,
) -> Vec<M5DeploymentContinuityCompatibilityNote> {
    vec![
        note(
            surface,
            M5DeploymentCompatibilityDimension::ChannelOwnership,
            M5DeploymentCompatibilityPosture::Current,
            "Channel owner is explicit and matches the active install profile",
            None,
        ),
        note(
            surface,
            M5DeploymentCompatibilityDimension::HandlerPrecedence,
            M5DeploymentCompatibilityPosture::Current,
            "Handler precedence is declared and avoids last-writer-wins capture",
            None,
        ),
        note(
            surface,
            M5DeploymentCompatibilityDimension::MirrorOfflineFreshness,
            M5DeploymentCompatibilityPosture::Current,
            "Mirror and offline freshness are either current or not applicable to this surface",
            None,
        ),
        note(
            surface,
            M5DeploymentCompatibilityDimension::ControlPlaneDataPlaneContinuity,
            M5DeploymentCompatibilityPosture::Current,
            "Control-plane and data-plane status remain distinct with local-safe continuation visible",
            None,
        ),
    ]
}

fn surface_row(
    surface: M5ClaimedDeploymentSurface,
    component_families: Vec<M5DeploymentComponentFamily>,
    component_refs: Vec<&str>,
    compatibility_notes: Vec<M5DeploymentContinuityCompatibilityNote>,
    effective_label: M5DeploymentClaimLabel,
    narrowing_reasons: Vec<&str>,
) -> M5DeploymentContinuitySurfaceCertificationRow {
    let auto_narrowed = !effective_label.is_full();
    M5DeploymentContinuitySurfaceCertificationRow {
        surface,
        component_families,
        component_refs: component_refs.into_iter().map(str::to_owned).collect(),
        drills: all_drills(surface),
        compatibility_notes,
        claimed_label: M5DeploymentClaimLabel::FullTruth,
        effective_label,
        auto_narrowed,
        narrowing_reasons: narrowing_reasons.into_iter().map(str::to_owned).collect(),
        consumes_shared_component_family: true,
        unsupported_or_degraded_visible: true,
        local_safe_continuity_visible: true,
    }
}

fn seeded_surface_certification_rows() -> Vec<M5DeploymentContinuitySurfaceCertificationRow> {
    let mut rows = Vec::new();

    rows.push(surface_row(
        M5ClaimedDeploymentSurface::LocalOnly,
        vec![
            M5DeploymentComponentFamily::InstallProfileCard,
            M5DeploymentComponentFamily::DeploymentSummaryCard,
            M5DeploymentComponentFamily::ControlPlaneDataPlaneStatusStrip,
        ],
        vec![
            "component:install-profile-card:0001",
            "component:deployment-summary-card:0001",
            "component:control-plane-data-plane-status-strip:0001",
        ],
        current_notes(M5ClaimedDeploymentSurface::LocalOnly),
        M5DeploymentClaimLabel::FullTruth,
        vec![],
    ));

    let mut notes = current_notes(M5ClaimedDeploymentSurface::Managed);
    notes[3] = note(
        M5ClaimedDeploymentSurface::Managed,
        M5DeploymentCompatibilityDimension::ControlPlaneDataPlaneContinuity,
        M5DeploymentCompatibilityPosture::DegradedNarrowed,
        "Managed control plane is unreachable while local runtime and data-plane work continue",
        Some(M5DeploymentDowngradeTrigger::ControlPlaneImpaired),
    );
    rows.push(surface_row(
        M5ClaimedDeploymentSurface::Managed,
        vec![
            M5DeploymentComponentFamily::InstallProfileCard,
            M5DeploymentComponentFamily::DeploymentSummaryCard,
            M5DeploymentComponentFamily::ControlPlaneDataPlaneStatusStrip,
        ],
        vec![
            "component:install-profile-card:0001",
            "component:deployment-summary-card:0001",
            "component:control-plane-data-plane-status-strip:0001",
        ],
        notes,
        M5DeploymentClaimLabel::LocalSafeOnly,
        vec!["Control-plane outage narrows managed continuity to local-safe operation"],
    ));

    let mut notes = current_notes(M5ClaimedDeploymentSurface::SelfHosted);
    notes[0] = note(
        M5ClaimedDeploymentSurface::SelfHosted,
        M5DeploymentCompatibilityDimension::ChannelOwnership,
        M5DeploymentCompatibilityPosture::DegradedNarrowed,
        "Self-hosted install retains a vendor license-activation dependency that must be disclosed",
        Some(M5DeploymentDowngradeTrigger::ResidualVendorDependency),
    );
    rows.push(surface_row(
        M5ClaimedDeploymentSurface::SelfHosted,
        vec![
            M5DeploymentComponentFamily::DeploymentSummaryCard,
            M5DeploymentComponentFamily::ResidualDependencyRow,
        ],
        vec![
            "component:deployment-summary-card:0001",
            "component:residual-dependency-row:0001",
        ],
        notes,
        M5DeploymentClaimLabel::DegradedNarrowed,
        vec!["Residual vendor dependency prevents a fully independent self-hosted label"],
    ));

    let mut notes = current_notes(M5ClaimedDeploymentSurface::Mirrored);
    notes[2] = note(
        M5ClaimedDeploymentSurface::Mirrored,
        M5DeploymentCompatibilityDimension::MirrorOfflineFreshness,
        M5DeploymentCompatibilityPosture::DegradedNarrowed,
        "Mirrored artifact is verified but stale and renders as cached-offline freshness",
        Some(M5DeploymentDowngradeTrigger::MirrorStale),
    );
    rows.push(surface_row(
        M5ClaimedDeploymentSurface::Mirrored,
        vec![
            M5DeploymentComponentFamily::MirrorOfflineArtifactRow,
            M5DeploymentComponentFamily::ModeChangeReviewSheet,
        ],
        vec![
            "component:mirror-offline-artifact-row:0001",
            "component:mode-change-review-sheet:0001",
        ],
        notes,
        M5DeploymentClaimLabel::DegradedNarrowed,
        vec!["Mirror freshness is stale, so mirrored deployment cannot inherit a live label"],
    ));

    let mut notes = current_notes(M5ClaimedDeploymentSurface::Sovereign);
    notes[0] = note(
        M5ClaimedDeploymentSurface::Sovereign,
        M5DeploymentCompatibilityDimension::ChannelOwnership,
        M5DeploymentCompatibilityPosture::DegradedNarrowed,
        "Sovereign deployment carries an explicit residual vendor dependency review row",
        Some(M5DeploymentDowngradeTrigger::ResidualVendorDependency),
    );
    rows.push(surface_row(
        M5ClaimedDeploymentSurface::Sovereign,
        vec![
            M5DeploymentComponentFamily::DeploymentSummaryCard,
            M5DeploymentComponentFamily::ResidualDependencyRow,
        ],
        vec![
            "component:deployment-summary-card:0001",
            "component:residual-dependency-row:0001",
        ],
        notes,
        M5DeploymentClaimLabel::DegradedNarrowed,
        vec!["Sovereign deployment is narrowed until residual dependency review is current"],
    ));

    let mut notes = current_notes(M5ClaimedDeploymentSurface::AirGapped);
    notes[2] = note(
        M5ClaimedDeploymentSurface::AirGapped,
        M5DeploymentCompatibilityDimension::MirrorOfflineFreshness,
        M5DeploymentCompatibilityPosture::DegradedNarrowed,
        "Air-gapped deployment is operating from cached-offline media rather than a live source",
        Some(M5DeploymentDowngradeTrigger::OfflineCacheOnly),
    );
    rows.push(surface_row(
        M5ClaimedDeploymentSurface::AirGapped,
        vec![
            M5DeploymentComponentFamily::MirrorOfflineArtifactRow,
            M5DeploymentComponentFamily::ModeChangeReviewSheet,
        ],
        vec![
            "component:mirror-offline-artifact-row:0001",
            "component:mode-change-review-sheet:0001",
        ],
        notes,
        M5DeploymentClaimLabel::LocalSafeOnly,
        vec!["Air-gapped deployment uses cached-offline truth and cannot claim live freshness"],
    ));

    rows.push(surface_row(
        M5ClaimedDeploymentSurface::SideBySide,
        vec![
            M5DeploymentComponentFamily::SideBySideImportSheet,
            M5DeploymentComponentFamily::ChannelAssociationReviewRow,
            M5DeploymentComponentFamily::InstallProfileCard,
        ],
        vec![
            "component:side-by-side-import-sheet:0001",
            "component:channel-association-review-row:0001",
            "component:install-profile-card:0001",
        ],
        current_notes(M5ClaimedDeploymentSurface::SideBySide),
        M5DeploymentClaimLabel::FullTruth,
        vec![],
    ));

    let mut notes = current_notes(M5ClaimedDeploymentSurface::Portable);
    notes[0] = note(
        M5ClaimedDeploymentSurface::Portable,
        M5DeploymentCompatibilityDimension::ChannelOwnership,
        M5DeploymentCompatibilityPosture::DegradedNarrowed,
        "Portable state root is unavailable and the install-profile card names the reattach route",
        Some(M5DeploymentDowngradeTrigger::StateRootUnavailable),
    );
    rows.push(surface_row(
        M5ClaimedDeploymentSurface::Portable,
        vec![M5DeploymentComponentFamily::InstallProfileCard],
        vec!["component:install-profile-card:0002"],
        notes,
        M5DeploymentClaimLabel::LocalSafeOnly,
        vec![
            "Portable state root is unavailable, so portable install narrows to reattach-required",
        ],
    ));

    let mut notes = current_notes(M5ClaimedDeploymentSurface::FleetRollout);
    notes[0] = note(
        M5ClaimedDeploymentSurface::FleetRollout,
        M5DeploymentCompatibilityDimension::ChannelOwnership,
        M5DeploymentCompatibilityPosture::DegradedNarrowed,
        "Fleet rollout is held in canary and cannot publish a broad rollout label",
        Some(M5DeploymentDowngradeTrigger::RolloutPaused),
    );
    rows.push(surface_row(
        M5ClaimedDeploymentSurface::FleetRollout,
        vec![M5DeploymentComponentFamily::RolloutRingRow],
        vec!["component:rollout-ring-row:0001"],
        notes,
        M5DeploymentClaimLabel::DegradedNarrowed,
        vec!["Rollout ring is held at canary pending promotion evidence"],
    ));

    rows
}

fn seeded_release_support_proof() -> M5DeploymentContinuityReleaseSupportProof {
    M5DeploymentContinuityReleaseSupportProof {
        functional_drills_complete: true,
        accessibility_drills_complete: true,
        export_drills_complete: true,
        degradation_drills_complete: true,
        compatibility_notes_complete: true,
        proof_packet_published: true,
        later_m5_gating_asset: true,
        support_export_refs: vec![
            DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_ARTIFACT_REF.to_owned(),
            DEPLOYMENT_CONTINUITY_COMPONENT_CERTIFICATION_ARTIFACT_REF.to_owned(),
            DEPLOYMENT_CONTINUITY_COMPONENT_CERTIFICATION_REPORT_REF.to_owned(),
        ],
    }
}

fn seeded_certification_source_contract_refs() -> Vec<String> {
    vec![
        DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_DOC_REF.to_owned(),
        DEPLOYMENT_CONTINUITY_COMPONENT_CERTIFICATION_ARTIFACT_REF.to_owned(),
        DEPLOYMENT_CONTINUITY_COMPONENT_CERTIFICATION_CSV_REF.to_owned(),
        DEPLOYMENT_CONTINUITY_COMPONENT_CERTIFICATION_REPORT_REF.to_owned(),
        "schemas/ui/m5-deployment-continuity-component-consumer.schema.json".to_owned(),
        "schemas/ui/m5-deployment-continuity-accessibility-fallback.schema.json".to_owned(),
    ]
}

fn mandatory_labels() -> Vec<M5DeploymentRequiredLabel> {
    vec![
        M5DeploymentRequiredLabel::Identity,
        M5DeploymentRequiredLabel::OperatingMode,
        M5DeploymentRequiredLabel::OwnershipOrScope,
        M5DeploymentRequiredLabel::FreshnessClass,
        M5DeploymentRequiredLabel::ContinuityState,
        M5DeploymentRequiredLabel::KeyboardRoute,
    ]
}

fn base_row(
    component_id: &str,
    family: M5DeploymentComponentFamily,
    surface_label: &str,
    modes: (M5DeploymentTruthMode, M5DeploymentMode),
    context_ref: &str,
    label_summary: &str,
    evidence_id: &str,
) -> ComponentRow {
    let (truth_mode, deployment_mode) = modes;
    ComponentRow {
        component_id: component_id.to_owned(),
        family,
        surface_label: surface_label.to_owned(),
        truth_mode,
        deployment_mode,
        operating_context_ref: context_ref.to_owned(),
        required_labels: mandatory_labels(),
        export_safe: true,
        assistive_ready: true,
        install_profile_card: None,
        side_by_side_import_sheet: None,
        rollout_ring_row: None,
        deployment_summary_card: None,
        residual_dependency_row: None,
        control_plane_data_plane_status_strip: None,
        mirror_offline_artifact_row: None,
        mode_change_review_sheet: None,
        channel_association_review_row: None,
        degraded: None,
        label_summary: label_summary.to_owned(),
        observed_at: "2026-07-04T00:00:00Z".to_owned(),
        evidence_refs: ev(evidence_id),
    }
}

fn seeded_components() -> Vec<ComponentRow> {
    let mut rows = Vec::new();

    // Install-profile card — a managed desktop install, ownership and roots explicit.
    let mut row = base_row(
        "component:install-profile-card:0001",
        M5DeploymentComponentFamily::InstallProfileCard,
        "Install-profile card on the About page for a managed install",
        (M5DeploymentTruthMode::Live, M5DeploymentMode::Managed),
        "operating_context:install:0001",
        "An install-profile card keeps install mode, channel, updater owner, and durable state roots explicit",
        "install-profile-card:0001",
    );
    row.install_profile_card = Some(InstallProfileCardDescriptor {
        install_id_ref: "install:managed:0001".to_owned(),
        install_mode: M5DeploymentMode::Managed,
        channel_ref: "channel:stable".to_owned(),
        updater_owner_ref: "updater_owner:managed_admin".to_owned(),
        state_root_ref: "state_root:managed:0001".to_owned(),
        discloses_state_roots: true,
        discloses_updater_owner: true,
    });
    rows.push(row);

    // Install-profile card — a portable install whose state root is unavailable, narrows.
    let mut row = base_row(
        "component:install-profile-card:0002",
        M5DeploymentComponentFamily::InstallProfileCard,
        "Install-profile card for a portable install with an unavailable state root",
        (M5DeploymentTruthMode::CachedOffline, M5DeploymentMode::Portable),
        "operating_context:install:0002",
        "An install-profile card discloses that a portable state root is currently unavailable rather than imply a fully resolved install",
        "install-profile-card:0002",
    );
    row.install_profile_card = Some(InstallProfileCardDescriptor {
        install_id_ref: "install:portable:0002".to_owned(),
        install_mode: M5DeploymentMode::Portable,
        channel_ref: "channel:preview".to_owned(),
        updater_owner_ref: "updater_owner:self".to_owned(),
        state_root_ref: "state_root:portable:0002".to_owned(),
        discloses_state_roots: true,
        discloses_updater_owner: true,
    });
    row.degraded = Some(DegradedState {
        trigger: M5DeploymentDowngradeTrigger::StateRootUnavailable,
        degraded_label: "The portable drive holding this install's durable state root is not mounted; the card names the expected root and offers a re-attach route".to_owned(),
    });
    rows.push(row);

    // Side-by-side import sheet — stable and preview coexisting, no handler capture.
    let mut row = base_row(
        "component:side-by-side-import-sheet:0001",
        M5DeploymentComponentFamily::SideBySideImportSheet,
        "Side-by-side import sheet for a preview install next to stable",
        (M5DeploymentTruthMode::Live, M5DeploymentMode::Desktop),
        "operating_context:install:0001",
        "A side-by-side import sheet keeps handler ownership inspectable and never captures the default handler from the other install",
        "side-by-side-import-sheet:0001",
    );
    row.side_by_side_import_sheet = Some(SideBySideImportSheetDescriptor {
        import_source_ref: "install:stable:0001".to_owned(),
        handler_ownership_ref: "handler_ownership:stable:0001".to_owned(),
        last_writer_wins_capture: false,
        discloses_handler_ownership: true,
        isolation_preserved: true,
    });
    rows.push(row);

    // Rollout-ring row — a canary ring holding promotion, narrows.
    let mut row = base_row(
        "component:rollout-ring-row:0001",
        M5DeploymentComponentFamily::RolloutRingRow,
        "Rollout-ring row for a held canary ring",
        (M5DeploymentTruthMode::Live, M5DeploymentMode::Managed),
        "operating_context:fleet:0001",
        "A rollout-ring row discloses that this fleet sits in a held canary ring rather than imply general availability",
        "rollout-ring-row:0001",
    );
    row.rollout_ring_row = Some(RolloutRingRowDescriptor {
        ring: M5RolloutRing::Canary,
        promotion_state: M5PromotionState::Held,
        target_scope_ref: "fleet_scope:canary:0001".to_owned(),
        rollback_available: true,
        discloses_ring: true,
    });
    row.degraded = Some(DegradedState {
        trigger: M5DeploymentDowngradeTrigger::RolloutPaused,
        degraded_label: "Promotion for this canary ring is held pending a gate; the row names the ring and keeps a rollback path available".to_owned(),
    });
    rows.push(row);

    // Deployment summary card — a self-hosted deployment, both planes visible.
    let mut row = base_row(
        "component:deployment-summary-card:0001",
        M5DeploymentComponentFamily::DeploymentSummaryCard,
        "Deployment summary card for a self-hosted tenant",
        (M5DeploymentTruthMode::Live, M5DeploymentMode::SelfHosted),
        "operating_context:tenant:0001",
        "A deployment summary card keeps operating mode, tenant/region, and both control-plane and data-plane status visible",
        "deployment-summary-card:0001",
    );
    row.deployment_summary_card = Some(DeploymentSummaryCardDescriptor {
        tenant_region_ref: "tenant_region:eu-west:0001".to_owned(),
        operating_mode: M5DeploymentMode::SelfHosted,
        control_plane_visible: true,
        data_plane_visible: true,
    });
    rows.push(row);

    // Residual-dependency row — a self-hosted install with a remaining license dep, narrows.
    let mut row = base_row(
        "component:residual-dependency-row:0001",
        M5DeploymentComponentFamily::ResidualDependencyRow,
        "Residual-dependency row for a self-hosted license-activation dependency",
        (M5DeploymentTruthMode::Live, M5DeploymentMode::SelfHosted),
        "operating_context:tenant:0001",
        "A residual-dependency row keeps a remaining vendor dependency explicit rather than let a self-hosted claim read as fully independent",
        "residual-dependency-row:0001",
    );
    row.residual_dependency_row = Some(ResidualDependencyRowDescriptor {
        vendor_dependency_ref: "residual_dependency:license:0001".to_owned(),
        dependency_class: M5ResidualDependencyClass::LicenseActivation,
        required_for_operation: true,
        discloses_residual: true,
    });
    row.degraded = Some(DegradedState {
        trigger: M5DeploymentDowngradeTrigger::ResidualVendorDependency,
        degraded_label: "This self-hosted install still contacts the vendor for periodic license activation; the row names the dependency and its cadence".to_owned(),
    });
    rows.push(row);

    // Control-plane/data-plane status strip — control plane impaired, local runtime ok, narrows.
    let mut row = base_row(
        "component:control-plane-data-plane-status-strip:0001",
        M5DeploymentComponentFamily::ControlPlaneDataPlaneStatusStrip,
        "Control-plane/data-plane status strip during a managed control-plane outage",
        (M5DeploymentTruthMode::Live, M5DeploymentMode::Managed),
        "operating_context:tenant:0002",
        "A status strip keeps control-plane and data-plane distinct so a control-plane outage never reads as a broken local runtime",
        "control-plane-data-plane-status-strip:0001",
    );
    row.control_plane_data_plane_status_strip = Some(ControlPlaneDataPlaneStatusStripDescriptor {
        control_plane_state: M5PlaneState::Unavailable,
        data_plane_state: M5PlaneState::Operational,
        local_runtime_unaffected: true,
        impairment_not_masked_as_local_failure: true,
    });
    row.degraded = Some(DegradedState {
        trigger: M5DeploymentDowngradeTrigger::ControlPlaneImpaired,
        degraded_label: "The managed control plane is unreachable; local editing and runtime continue, and policy sync will resume when the control plane returns".to_owned(),
    });
    rows.push(row);

    // Mirror/offline artifact row — a stale mirrored artifact, narrows.
    let mut row = base_row(
        "component:mirror-offline-artifact-row:0001",
        M5DeploymentComponentFamily::MirrorOfflineArtifactRow,
        "Mirror/offline artifact row for a stale mirrored update artifact",
        (M5DeploymentTruthMode::CachedOffline, M5DeploymentMode::AirGapped),
        "operating_context:mirror:0001",
        "A mirror/offline artifact row discloses freshness and signature truth so stale mirrored content never reads as a live source",
        "mirror-offline-artifact-row:0001",
    );
    row.mirror_offline_artifact_row = Some(MirrorOfflineArtifactRowDescriptor {
        mirror_source_ref: "mirror_source:offline-media:0001".to_owned(),
        freshness: M5DeploymentTruthMode::CachedOffline,
        signature_state: M5MirrorSignatureState::Verified,
        discloses_freshness: true,
        stale_not_shown_as_current: true,
    });
    row.degraded = Some(DegradedState {
        trigger: M5DeploymentDowngradeTrigger::MirrorStale,
        degraded_label: "This artifact came from offline media last synced 9 days ago; it is shown as cached-offline, not as a current live source".to_owned(),
    });
    rows.push(row);

    // Mode-change review sheet — a channel switch reviewed before dispatch.
    let mut row = base_row(
        "component:mode-change-review-sheet:0001",
        M5DeploymentComponentFamily::ModeChangeReviewSheet,
        "Mode-change review sheet for a desktop-to-managed channel switch",
        (M5DeploymentTruthMode::Live, M5DeploymentMode::Desktop),
        "operating_context:install:0003",
        "A mode-change review sheet shows the cache reuse and rollback consequences before a durable boundary change, never after",
        "mode-change-review-sheet:0001",
    );
    row.mode_change_review_sheet = Some(ModeChangeReviewSheetDescriptor {
        from_mode: M5DeploymentMode::Desktop,
        to_mode: M5DeploymentMode::Managed,
        boundary_change: M5BoundaryChangeClass::UpdaterOwnershipChange,
        reviewed_before_durable_change: true,
        discloses_cache_and_rollback: true,
    });
    rows.push(row);

    // Channel-association review row — a protocol handler change reviewed before apply.
    let mut row = base_row(
        "component:channel-association-review-row:0001",
        M5DeploymentComponentFamily::ChannelAssociationReviewRow,
        "Channel-association review row for a protocol-handler change",
        (M5DeploymentTruthMode::Live, M5DeploymentMode::Desktop),
        "operating_context:install:0003",
        "A channel-association review row discloses the current owner and reviews the change before apply, never silently capturing the handler",
        "channel-association-review-row:0001",
    );
    row.channel_association_review_row = Some(ChannelAssociationReviewRowDescriptor {
        channel_ref: "channel_association:protocol-handler:0001".to_owned(),
        handler_association_ref: "handler_association:current:0001".to_owned(),
        last_writer_wins_capture: false,
        reviewed_before_apply: true,
        discloses_current_owner: true,
    });
    rows.push(row);

    rows
}

fn seeded_guardrails() -> DeploymentContinuityGuardrails {
    DeploymentContinuityGuardrails {
        operating_truth_explicit_on_every_surface: true,
        no_last_writer_wins_handler_capture: true,
        control_plane_impairment_never_masked_as_local: true,
        mirror_offline_freshness_never_shown_as_current: true,
        self_hosted_never_omits_residual_dependency: true,
        mode_changes_reviewed_before_durable_change: true,
        exported_evidence_preserves_ids_modes_and_states: true,
        components_bound_to_shared_vocabulary: true,
        no_new_installers_engines_or_protocols: true,
    }
}

fn seeded_consumer_projection() -> DeploymentContinuityConsumerProjection {
    DeploymentContinuityConsumerProjection {
        product_ingests_components: true,
        docs_help_ingests_components: true,
        diagnostics_ingests_components: true,
        admin_ingests_components: true,
        support_export_ingests_components: true,
        release_control_ingests_components: true,
        later_rows_reference_one_canonical_family: true,
    }
}

fn seeded_source_contract_refs() -> Vec<String> {
    vec![
        DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_DOC_REF.to_owned(),
        DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_ARTIFACT_REF.to_owned(),
        "schemas/install/m5-coexistence-and-fleet-rollout.schema.json".to_owned(),
        "schemas/install/m5-install-and-portability-governance.schema.json".to_owned(),
    ]
}
