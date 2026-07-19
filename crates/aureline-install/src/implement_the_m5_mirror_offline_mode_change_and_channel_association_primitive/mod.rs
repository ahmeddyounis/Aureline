//! Implements the reusable mirror/offline transition primitive: a set of mirror/offline
//! artifact rows, a mode-change / disconnect review sheet, and a channel-association
//! review row that all resolve from one transition context and share one transition
//! identity, so switching between online, mirrored, offline, rebuilt, or disconnected
//! states stays reviewed, attributable, and reversible on every claimed deployment
//! surface.
//!
//! Where
//! [`crate::freeze_the_m5_deployment_continuity_component_matrix`] *freezes* the
//! reusable deployment / continuity component families as a governed contract, and
//! [`crate::implement_the_m5_install_profile_side_by_side_import_and_rollout_ring_primitive`]
//! and
//! [`crate::implement_the_m5_deployment_summary_residual_dependency_and_control_data_plane_primitive`]
//! narrow the install-profile / rollout and the deployment-summary / residual /
//! plane-status families, this module *narrows* the remaining three operational families
//! —
//! [`crate::freeze_the_m5_deployment_continuity_component_matrix::M5DeploymentComponentFamily::MirrorOfflineArtifactRow`],
//! [`crate::freeze_the_m5_deployment_continuity_component_matrix::M5DeploymentComponentFamily::ModeChangeReviewSheet`],
//! and
//! [`crate::freeze_the_m5_deployment_continuity_component_matrix::M5DeploymentComponentFamily::ChannelAssociationReviewRow`]
//! — into one working primitive with a real **resolver**. A single transition context
//! projects onto its mirror/offline artifact rows, a mode-change review sheet, and a
//! channel-association review row that all carry one transition identity, so mirror
//! freshness, signature verification, cache reuse / invalidation, rollback truth, and
//! handler ownership never blur across them.
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — offline and mirror transitions never read like generic warnings.** Every
//!   mirror/offline artifact row names its source class, artifact class, signature /
//!   digest verification, freshness, and one shared continuity state
//!   (`Mirror unavailable`, `Offline cache only`, `Verification failed`,
//!   `Needs refresh`, …), and the mode-change sheet names exactly what will stale, what
//!   remains usable, and how to reverse the change; mirrored / cached content is never
//!   shown as a current live source.
//! - **AC2 — artifact verification / manifests remain accessible from the same
//!   component family across deployment profiles.** Every artifact row keeps a
//!   verify-signature and an open-manifest action reachable regardless of the deployment
//!   mode; an artifact whose verification or manifest is not accessible is rejected.
//! - **AC3 — mode changes preserve export-before-change and rollback truth.** The
//!   review sheet keeps a preserved-local-state ref, an export-before-change action, and
//!   a rollback path; a change forced without review or without an export-before-change
//!   path is rejected rather than applied as a blind switch, and a channel association
//!   never silently captures a default handler.
//!
//! Raw config bytes, credentials, license keys, mirror URLs, provider cursors, and
//! device identifiers never cross this boundary; the resolver carries only opaque refs,
//! typed class tokens, booleans, and redacted labels, so support and diagnostics exports
//! reconstruct exactly what a surface would have shown without leaking source or live
//! payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-mirror-transition-primitive.schema.json`](../../../../schemas/ui/m5-mirror-transition-primitive.schema.json).
//! The contract doc is
//! [`docs/deployment/m5_mirror_transition_primitive.md`](../../../../docs/deployment/m5_mirror_transition_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_deployment_continuity_component_matrix::{
    DegradedState, M5BoundaryChangeClass, M5DeploymentDowngradeTrigger, M5DeploymentMode,
    M5DeploymentTruthMode, M5MirrorSignatureState,
};

/// Stable record-kind tag carried by [`M5MirrorTransitionPrimitivePacket`].
pub const M5_MIRROR_TRANSITION_RECORD_KIND: &str = "m5_mirror_transition_primitive";

/// Schema version for the mirror-transition primitive packet.
pub const M5_MIRROR_TRANSITION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_MIRROR_TRANSITION_SCHEMA_REF: &str =
    "schemas/ui/m5-mirror-transition-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_MIRROR_TRANSITION_DOC_REF: &str = "docs/deployment/m5_mirror_transition_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive narrows.
pub const M5_MIRROR_TRANSITION_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-deployment-continuity-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_MIRROR_TRANSITION_FIXTURE_DIR: &str = "fixtures/ui/m5-mirror-transition-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const M5_MIRROR_TRANSITION_ARTIFACT_REF: &str =
    "artifacts/release/m5-mirror-transition-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_MIRROR_TRANSITION_CSV_REF: &str =
    "artifacts/release/m5-mirror-transition-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_MIRROR_TRANSITION_REPORT_REF: &str =
    "artifacts/release/m5-mirror-transition-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed mirror-transition surface family. Each family is one parity surface that
/// ingests the shared primitive; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MirrorSurfaceFamily {
    /// The update-center mirror / offline artifact surface.
    UpdateCenter,
    /// The mirror-manager surface governing mirror sources and freshness.
    MirrorManager,
    /// The admin deployment console reviewing a mode change or disconnect.
    AdminDeploymentConsole,
    /// The diagnostics mirror / verification pane.
    DiagnosticsMirror,
    /// The support / export replay surface reconstructing transition truth.
    SupportExportReplay,
    /// The docs / help mirror-and-offline reference surface.
    DocsMirrorReference,
}

impl M5MirrorSurfaceFamily {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UpdateCenter,
        Self::MirrorManager,
        Self::AdminDeploymentConsole,
        Self::DiagnosticsMirror,
        Self::SupportExportReplay,
        Self::DocsMirrorReference,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpdateCenter => "update_center",
            Self::MirrorManager => "mirror_manager",
            Self::AdminDeploymentConsole => "admin_deployment_console",
            Self::DiagnosticsMirror => "diagnostics_mirror",
            Self::SupportExportReplay => "support_export_replay",
            Self::DocsMirrorReference => "docs_mirror_reference",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UpdateCenter => "Update-center mirror surface",
            Self::MirrorManager => "Mirror-manager surface",
            Self::AdminDeploymentConsole => "Admin deployment console",
            Self::DiagnosticsMirror => "Diagnostics mirror pane",
            Self::SupportExportReplay => "Support / export replay",
            Self::DocsMirrorReference => "Docs mirror reference",
        }
    }
}

/// Closed mirror-artifact-class vocabulary. Names the kind of artifact a mirror/offline
/// row governs so the same component family covers docs, extensions, models, updates,
/// and policy bundles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MirrorArtifactClass {
    /// Documentation / help content.
    Docs,
    /// Extensions / plugins.
    Extensions,
    /// Model weights / inference bundles.
    Models,
    /// Application updates / release bundles.
    Updates,
    /// Policy / configuration bundles.
    PolicyBundles,
}

impl M5MirrorArtifactClass {
    /// Every artifact class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Docs,
        Self::Extensions,
        Self::Models,
        Self::Updates,
        Self::PolicyBundles,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Docs => "docs",
            Self::Extensions => "extensions",
            Self::Models => "models",
            Self::Updates => "updates",
            Self::PolicyBundles => "policy_bundles",
        }
    }
}

/// Closed mirror-source-class vocabulary. Names where a mirror artifact came from so an
/// offline bundle or peer cache never reads as the same thing as a live first-party
/// source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MirrorSourceClass {
    /// A first-party vendor-operated mirror.
    FirstPartyMirror,
    /// A self-hosted mirror the customer operates.
    SelfHostedMirror,
    /// An offline / air-gapped bundle imported from media.
    OfflineBundle,
    /// A peer / local cache shared across installs.
    PeerCache,
    /// A vendor CDN edge.
    VendorCdn,
}

impl M5MirrorSourceClass {
    /// Every source class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FirstPartyMirror,
        Self::SelfHostedMirror,
        Self::OfflineBundle,
        Self::PeerCache,
        Self::VendorCdn,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyMirror => "first_party_mirror",
            Self::SelfHostedMirror => "self_hosted_mirror",
            Self::OfflineBundle => "offline_bundle",
            Self::PeerCache => "peer_cache",
            Self::VendorCdn => "vendor_cdn",
        }
    }
}

/// Closed mirror/offline continuity-state vocabulary. This is the single vocabulary the
/// spec requires be preserved across UI, docs / help, and support exports —
/// `Mirror unavailable`, `Offline cache only`, `Verification failed`, `Needs refresh`,
/// and their sibling states — so an offline or mirror transition never collapses into a
/// generic warning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MirrorContinuityState {
    /// The artifact is current and its signature is verified.
    CurrentVerified,
    /// The artifact is usable but stale relative to its live source.
    NeedsRefresh,
    /// Only cached-offline content is available; no live source right now.
    OfflineCacheOnly,
    /// The mirror source is unreachable.
    MirrorUnavailable,
    /// The signature / digest verification failed.
    VerificationFailed,
    /// The artifact is intentionally pinned for offline use.
    PinnedOffline,
}

impl M5MirrorContinuityState {
    /// Every continuity state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CurrentVerified,
        Self::NeedsRefresh,
        Self::OfflineCacheOnly,
        Self::MirrorUnavailable,
        Self::VerificationFailed,
        Self::PinnedOffline,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentVerified => "current_verified",
            Self::NeedsRefresh => "needs_refresh",
            Self::OfflineCacheOnly => "offline_cache_only",
            Self::MirrorUnavailable => "mirror_unavailable",
            Self::VerificationFailed => "verification_failed",
            Self::PinnedOffline => "pinned_offline",
        }
    }

    /// True when the state blocks use of the artifact (unreachable mirror or a failed
    /// verification).
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::MirrorUnavailable | Self::VerificationFailed)
    }

    /// True when the state is anything other than a current, verified, live source, so
    /// the row must never be shown as current.
    pub const fn is_stale_or_blocked(self) -> bool {
        !matches!(self, Self::CurrentVerified)
    }

    /// True when the state calls for a user / admin action (refresh, reconnect, verify).
    pub const fn requires_action(self) -> bool {
        matches!(
            self,
            Self::NeedsRefresh
                | Self::OfflineCacheOnly
                | Self::MirrorUnavailable
                | Self::VerificationFailed
        )
    }

    /// A coarse severity used to derive the mode-change sheet's overall artifact posture
    /// (the worst state across the transition's artifacts).
    const fn severity(self) -> u8 {
        match self {
            Self::CurrentVerified => 0,
            Self::PinnedOffline => 1,
            Self::NeedsRefresh => 2,
            Self::OfflineCacheOnly => 3,
            Self::MirrorUnavailable => 4,
            Self::VerificationFailed => 5,
        }
    }

    /// Derives the shared continuity state from an artifact's freshness, signature, and
    /// mirror / pin flags. This is the one derivation every surface shares so a mirror
    /// artifact renders the same state everywhere.
    pub const fn derive(
        freshness: M5DeploymentTruthMode,
        signature: M5MirrorSignatureState,
        mirror_reachable: bool,
        pinned_offline: bool,
        needs_refresh: bool,
    ) -> Self {
        if matches!(signature, M5MirrorSignatureState::SignatureMismatch) {
            return Self::VerificationFailed;
        }
        if !mirror_reachable {
            return Self::MirrorUnavailable;
        }
        if matches!(freshness, M5DeploymentTruthMode::CachedOffline) {
            return Self::OfflineCacheOnly;
        }
        if needs_refresh {
            return Self::NeedsRefresh;
        }
        if pinned_offline {
            return Self::PinnedOffline;
        }
        if freshness.is_current_source() && matches!(signature, M5MirrorSignatureState::Verified) {
            return Self::CurrentVerified;
        }
        Self::NeedsRefresh
    }
}

/// Closed mirror-artifact-action vocabulary. Names the actions a mirror/offline artifact
/// row always keeps reachable so verification and manifests stay accessible from the same
/// component family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MirrorArtifactAction {
    /// Verify the artifact's signature / digest.
    VerifySignature,
    /// Open the artifact's manifest.
    OpenManifest,
    /// Refresh the artifact from its live source.
    RefreshNow,
    /// Pin the artifact for offline use.
    PinOffline,
}

impl M5MirrorArtifactAction {
    /// Every artifact action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::VerifySignature,
        Self::OpenManifest,
        Self::RefreshNow,
        Self::PinOffline,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifySignature => "verify_signature",
            Self::OpenManifest => "open_manifest",
            Self::RefreshNow => "refresh_now",
            Self::PinOffline => "pin_offline",
        }
    }
}

/// Closed cache-disposition vocabulary. Names what happens to the local cache when the
/// mode changes so cache reuse and invalidation are reviewed rather than silent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CacheDisposition {
    /// The existing cache stays valid and is reused as-is.
    ReuseValid,
    /// The cache is invalidated because it is now stale.
    InvalidateStale,
    /// The cache must be rebuilt from a fresh source.
    RebuildRequired,
    /// The cache is preserved as an intentionally pinned offline copy.
    PreservePinned,
}

impl M5CacheDisposition {
    /// Every cache disposition, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReuseValid,
        Self::InvalidateStale,
        Self::RebuildRequired,
        Self::PreservePinned,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReuseValid => "reuse_valid",
            Self::InvalidateStale => "invalidate_stale",
            Self::RebuildRequired => "rebuild_required",
            Self::PreservePinned => "preserve_pinned",
        }
    }

    /// True when the disposition reuses (rather than discards) the existing cache.
    pub const fn reuses_cache(self) -> bool {
        matches!(self, Self::ReuseValid | Self::PreservePinned)
    }
}

/// Closed rollback-path vocabulary. Names how the mode change can be reversed so a
/// transition never forces a blind switch with no way back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RollbackPathState {
    /// A rollback target is established and can be applied directly.
    Available,
    /// A rollback is possible but requires a checkpoint to be taken first.
    RequiresCheckpoint,
    /// No rollback path exists for this change.
    Unavailable,
    /// Rollback does not apply (a reversible, non-durable change).
    NotApplicable,
}

impl M5RollbackPathState {
    /// Every rollback-path state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Available,
        Self::RequiresCheckpoint,
        Self::Unavailable,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::RequiresCheckpoint => "requires_checkpoint",
            Self::Unavailable => "unavailable",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the change can be reversed by a rollback (directly or after a
    /// checkpoint).
    pub const fn is_recoverable(self) -> bool {
        matches!(self, Self::Available | Self::RequiresCheckpoint)
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet must
/// carry per surface; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MirrorTransitionExportField {
    /// The stable transition identity shared across surfaces.
    TransitionId,
    /// The artifact class each row governs.
    ArtifactClass,
    /// The mirror source class.
    SourceClass,
    /// The artifact freshness / provenance class.
    ArtifactFreshness,
    /// The signature / digest verification state.
    SignatureVerification,
    /// The shared mirror/offline continuity state.
    ContinuityState,
    /// The cache reuse / invalidation disposition.
    CacheDisposition,
    /// The rollback path for the change.
    RollbackPath,
    /// The export-before-change action.
    ExportBeforeChange,
    /// The channel / handler owner reviewed before the change.
    ChannelOwner,
}

impl M5MirrorTransitionExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::TransitionId,
        Self::ArtifactClass,
        Self::SourceClass,
        Self::ArtifactFreshness,
        Self::SignatureVerification,
        Self::ContinuityState,
        Self::CacheDisposition,
        Self::RollbackPath,
        Self::ExportBeforeChange,
        Self::ChannelOwner,
    ];

    /// The mandatory subset every row must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::TransitionId,
        Self::ContinuityState,
        Self::SignatureVerification,
        Self::CacheDisposition,
        Self::RollbackPath,
        Self::ExportBeforeChange,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TransitionId => "transition_id",
            Self::ArtifactClass => "artifact_class",
            Self::SourceClass => "source_class",
            Self::ArtifactFreshness => "artifact_freshness",
            Self::SignatureVerification => "signature_verification",
            Self::ContinuityState => "continuity_state",
            Self::CacheDisposition => "cache_disposition",
            Self::RollbackPath => "rollback_path",
            Self::ExportBeforeChange => "export_before_change",
            Self::ChannelOwner => "channel_owner",
        }
    }
}

// --- resolver input ---

/// One mirror/offline artifact the transition context governs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MirrorArtifactInput {
    /// Opaque ref to the artifact; never a raw URL.
    pub artifact_ref: String,
    /// The kind of artifact this row governs.
    pub artifact_class: M5MirrorArtifactClass,
    /// Where the artifact came from.
    pub source_class: M5MirrorSourceClass,
    /// The provenance / freshness class of the artifact.
    pub freshness: M5DeploymentTruthMode,
    /// How the artifact's signature / digest was verified.
    pub signature_state: M5MirrorSignatureState,
    /// Whether the mirror source is reachable right now.
    pub mirror_reachable: bool,
    /// Whether the artifact is intentionally pinned for offline use.
    pub pinned_offline: bool,
    /// Whether the artifact is stale and needs a refresh.
    pub needs_refresh: bool,
    /// Opaque ref to the artifact manifest; never a raw URL.
    pub manifest_ref: String,
    /// The verify-signature action is reachable on the row; must hold (AC2).
    pub verify_available: bool,
    /// The open-manifest action is reachable on the row; must hold (AC2).
    pub open_manifest_available: bool,
    /// Mirrored / cached content is never shown as a current live source; required when
    /// the freshness is not a current first-party source (AC1).
    pub stale_not_shown_as_current: bool,
}

/// The full input to the mirror-transition resolver for one transition context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MirrorTransitionInput {
    /// The stable transition identity that must survive across the artifact rows, the
    /// mode-change review sheet, and the channel-association review row.
    pub transition_id: String,
    /// Human-readable surface / context label.
    pub surface_label: String,
    /// The operating / install mode the transition acts in.
    pub deployment_mode: M5DeploymentMode,
    /// The mirror/offline artifacts the transition context governs.
    pub artifacts: Vec<M5MirrorArtifactInput>,
    /// The mode the install is moving from.
    pub from_mode: M5DeploymentMode,
    /// The mode the install is moving to.
    pub to_mode: M5DeploymentMode,
    /// What durable boundary the change moves.
    pub boundary_change: M5BoundaryChangeClass,
    /// Opaque ref to the local state preserved across the change.
    pub preserved_local_state_ref: String,
    /// Opaque refs to the managed features the change affects.
    #[serde(default)]
    pub affected_managed_feature_refs: Vec<String>,
    /// What happens to the local cache when the mode changes.
    pub cache_disposition: M5CacheDisposition,
    /// How the change can be reversed.
    pub rollback_path_state: M5RollbackPathState,
    /// The change is reviewed before it is applied; must hold (AC3).
    pub reviewed_before_change: bool,
    /// An export-before-change action is available; must hold (AC3).
    pub export_before_change_available: bool,
    /// Opaque ref to the channel / protocol / file association reviewed with the change.
    pub channel_ref: String,
    /// Opaque ref to the current handler association.
    pub handler_association_ref: String,
    /// The association silently captures a default handler; must be `false`.
    pub last_writer_wins_capture: bool,
    /// The association change is reviewed before it is applied; must hold.
    pub reviewed_before_apply: bool,
    /// The row discloses the current owner before the change; must hold.
    pub discloses_current_owner: bool,
    /// An externally-observed narrowing (mirror stale, offline cache, signature
    /// unverified) that degrades the surface before action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

// --- resolved projections ---

/// The resolved mirror/offline artifact row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedMirrorArtifactRow {
    /// The transition identity — identical to the mode-change sheet and channel row.
    pub transition_id: String,
    /// The opaque artifact ref.
    pub artifact_ref: String,
    /// The kind of artifact this row governs.
    pub artifact_class: M5MirrorArtifactClass,
    /// Where the artifact came from.
    pub source_class: M5MirrorSourceClass,
    /// The provenance / freshness class of the artifact.
    pub freshness: M5DeploymentTruthMode,
    /// How the artifact's signature / digest was verified.
    pub signature_state: M5MirrorSignatureState,
    /// The shared mirror/offline continuity state derived for this artifact.
    pub continuity_state: M5MirrorContinuityState,
    /// Whether the artifact is intentionally pinned for offline use.
    pub pinned_offline: bool,
    /// The opaque manifest ref.
    pub manifest_ref: String,
    /// The actions kept reachable on the row (verify / manifest always present).
    pub actions: Vec<M5MirrorArtifactAction>,
    /// The row discloses the artifact freshness; always holds.
    pub discloses_freshness: bool,
    /// Mirrored / cached content is never shown as a current live source; always holds.
    pub stale_not_shown_as_current: bool,
    /// Verification and manifest stay accessible on the row (AC2); always holds.
    pub verification_accessible: bool,
    /// The row is export-safe and carried in the support export; always holds.
    pub exportable: bool,
}

/// The resolved mode-change / disconnect review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedModeChangeReviewSheet {
    /// The transition identity — identical to every other surface.
    pub transition_id: String,
    /// The mode the install is moving from.
    pub from_mode: M5DeploymentMode,
    /// The mode the install is moving to.
    pub to_mode: M5DeploymentMode,
    /// What durable boundary the change moves.
    pub boundary_change: M5BoundaryChangeClass,
    /// The opaque preserved-local-state ref.
    pub preserved_local_state_ref: String,
    /// The opaque affected-managed-feature refs.
    pub affected_managed_feature_refs: Vec<String>,
    /// What happens to the local cache when the mode changes.
    pub cache_disposition: M5CacheDisposition,
    /// The overall artifact posture across the transition (the worst continuity state).
    pub artifact_posture: M5MirrorContinuityState,
    /// How the change can be reversed.
    pub rollback_path_state: M5RollbackPathState,
    /// The change was reviewed before the durable boundary change; always holds.
    pub reviewed_before_change: bool,
    /// An export-before-change action is available; always holds (AC3).
    pub export_before_change_available: bool,
    /// The change can be reversed (a rollback path or an export-before-change exists);
    /// always holds.
    pub reversible: bool,
    /// The sheet names exactly what will stale and what remains usable (AC1); always
    /// holds.
    pub discloses_stale_and_usable: bool,
}

/// The resolved channel-association review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedChannelAssociationRow {
    /// The transition identity — identical to every other surface.
    pub transition_id: String,
    /// The opaque channel / protocol / file-association ref.
    pub channel_ref: String,
    /// The opaque current-handler-association ref.
    pub handler_association_ref: String,
    /// The association never silently captures a default handler (AC3); always `false`.
    pub last_writer_wins_capture: bool,
    /// The change is reviewed before it is applied; always holds.
    pub reviewed_before_apply: bool,
    /// The row discloses the current owner before the change; always holds.
    pub discloses_current_owner: bool,
}

/// The resolved mirror-transition truth shared across the artifact rows, the mode-change
/// review sheet, and the channel-association review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedMirrorTransition {
    /// The stable transition identity.
    pub transition_id: String,
    /// The resolved mirror/offline artifact rows.
    pub artifact_rows: Vec<M5ResolvedMirrorArtifactRow>,
    /// The resolved mode-change / disconnect review sheet.
    pub mode_change_sheet: M5ResolvedModeChangeReviewSheet,
    /// The resolved channel-association review row.
    pub channel_row: M5ResolvedChannelAssociationRow,
    /// The transition reads as an explicit, attributable change rather than a generic
    /// warning (AC1).
    pub transition_explicit_not_generic: bool,
    /// Artifact verification / manifests stay accessible across deployment profiles
    /// (AC2).
    pub verification_accessible_across_profiles: bool,
    /// Export-before-change and rollback truth are preserved (AC3).
    pub export_and_rollback_preserved: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedMirrorTransition {
    /// True when the transition identity is identical across the artifact rows, the
    /// mode-change sheet, and the channel row.
    pub fn identity_consistent(&self) -> bool {
        self.mode_change_sheet.transition_id == self.transition_id
            && self.channel_row.transition_id == self.transition_id
            && self
                .artifact_rows
                .iter()
                .all(|row| row.transition_id == self.transition_id)
    }

    /// True when at least one artifact row is stale or blocked relative to a current
    /// live source.
    pub fn has_stale_or_blocked_artifact(&self) -> bool {
        self.artifact_rows
            .iter()
            .any(|row| row.continuity_state.is_stale_or_blocked())
    }

    /// True when the transition reads as an explicit change (AC1).
    pub fn transition_explicit_not_generic(&self) -> bool {
        self.transition_explicit_not_generic
    }

    /// True when verification / manifests stay accessible across profiles (AC2).
    pub fn verification_accessible_across_profiles(&self) -> bool {
        self.verification_accessible_across_profiles
    }

    /// True when export-before-change and rollback truth are preserved (AC3).
    pub fn export_and_rollback_preserved(&self) -> bool {
        self.export_and_rollback_preserved
    }
}

/// Errors returned by [`resolve_mirror_transition`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5MirrorTransitionResolutionError {
    /// The transition identity was empty.
    EmptyTransitionId,
    /// The preserved-local-state ref was empty.
    EmptyPreservedStateRef,
    /// The channel ref was empty.
    EmptyChannelRef,
    /// The handler-association ref was empty.
    EmptyHandlerRef,
    /// An artifact ref was empty.
    EmptyArtifactRef,
    /// An artifact manifest ref was empty.
    EmptyManifestRef,
    /// The transition governed no artifacts.
    NoArtifacts,
    /// A label, ref, or note carried forbidden material.
    ForbiddenMaterial,
    /// Mirrored / cached / imported content was shown as a current live source.
    StaleShownAsCurrent,
    /// An artifact's verify-signature action was not accessible.
    VerificationNotAccessible,
    /// An artifact's open-manifest action was not accessible.
    ManifestNotAccessible,
    /// A mode change was not reviewed before the durable boundary change.
    ChangeNotReviewed,
    /// A mode change was forced without an export-before-change path.
    BlindSwitchWithoutExport,
    /// A channel association silently captured a default handler.
    LastWriterWinsCapture,
    /// A channel-association change was not reviewed before apply.
    ChannelChangeNotReviewed,
    /// A channel-association row hid the current owner before the change.
    CurrentOwnerHidden,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5MirrorTransitionResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyTransitionId => "empty_transition_id",
            Self::EmptyPreservedStateRef => "empty_preserved_state_ref",
            Self::EmptyChannelRef => "empty_channel_ref",
            Self::EmptyHandlerRef => "empty_handler_ref",
            Self::EmptyArtifactRef => "empty_artifact_ref",
            Self::EmptyManifestRef => "empty_manifest_ref",
            Self::NoArtifacts => "no_artifacts",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::StaleShownAsCurrent => "stale_shown_as_current",
            Self::VerificationNotAccessible => "verification_not_accessible",
            Self::ManifestNotAccessible => "manifest_not_accessible",
            Self::ChangeNotReviewed => "change_not_reviewed",
            Self::BlindSwitchWithoutExport => "blind_switch_without_export",
            Self::LastWriterWinsCapture => "last_writer_wins_capture",
            Self::ChannelChangeNotReviewed => "channel_change_not_reviewed",
            Self::CurrentOwnerHidden => "current_owner_hidden",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5MirrorTransitionResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "mirror-transition resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5MirrorTransitionResolutionError {}

/// Resolves one transition context into its shared mirror/offline artifact rows,
/// mode-change review sheet, and channel-association review row.
///
/// The three surfaces share one transition identity, so mirror freshness, signature
/// verification, cache reuse / invalidation, rollback truth, and handler ownership never
/// blur across them. Mirrored / cached content is never shown as a current live source;
/// verification and manifests stay reachable regardless of deployment mode; a mode change
/// is never forced without review or an export-before-change path; and a channel
/// association never silently captures a default handler.
pub fn resolve_mirror_transition(
    input: &M5MirrorTransitionInput,
) -> Result<M5ResolvedMirrorTransition, M5MirrorTransitionResolutionError> {
    if input.transition_id.trim().is_empty() {
        return Err(M5MirrorTransitionResolutionError::EmptyTransitionId);
    }
    if input.preserved_local_state_ref.trim().is_empty() {
        return Err(M5MirrorTransitionResolutionError::EmptyPreservedStateRef);
    }
    if input.channel_ref.trim().is_empty() {
        return Err(M5MirrorTransitionResolutionError::EmptyChannelRef);
    }
    if input.handler_association_ref.trim().is_empty() {
        return Err(M5MirrorTransitionResolutionError::EmptyHandlerRef);
    }
    if input.artifacts.is_empty() {
        return Err(M5MirrorTransitionResolutionError::NoArtifacts);
    }

    let mut forbidden_scan: Vec<&str> = vec![
        input.transition_id.as_str(),
        input.surface_label.as_str(),
        input.preserved_local_state_ref.as_str(),
        input.channel_ref.as_str(),
        input.handler_association_ref.as_str(),
    ];
    for feature in &input.affected_managed_feature_refs {
        forbidden_scan.push(feature.as_str());
    }
    for artifact in &input.artifacts {
        forbidden_scan.push(artifact.artifact_ref.as_str());
        forbidden_scan.push(artifact.manifest_ref.as_str());
    }
    for value in forbidden_scan {
        if value_is_forbidden(value) {
            return Err(M5MirrorTransitionResolutionError::ForbiddenMaterial);
        }
    }

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5MirrorTransitionResolutionError::DegradedLabelGeneric);
        }
    }

    // AC3: a mode change is reviewed before the durable boundary change and always keeps
    // an export-before-change path rather than forcing a blind switch.
    if !input.reviewed_before_change {
        return Err(M5MirrorTransitionResolutionError::ChangeNotReviewed);
    }
    if !input.export_before_change_available {
        return Err(M5MirrorTransitionResolutionError::BlindSwitchWithoutExport);
    }

    // AC3: a channel association never silently captures a default handler; it is
    // reviewed and discloses the current owner before the change.
    if input.last_writer_wins_capture {
        return Err(M5MirrorTransitionResolutionError::LastWriterWinsCapture);
    }
    if !input.reviewed_before_apply {
        return Err(M5MirrorTransitionResolutionError::ChannelChangeNotReviewed);
    }
    if !input.discloses_current_owner {
        return Err(M5MirrorTransitionResolutionError::CurrentOwnerHidden);
    }

    let mut artifact_rows = Vec::with_capacity(input.artifacts.len());
    for artifact in &input.artifacts {
        if artifact.artifact_ref.trim().is_empty() {
            return Err(M5MirrorTransitionResolutionError::EmptyArtifactRef);
        }
        if artifact.manifest_ref.trim().is_empty() {
            return Err(M5MirrorTransitionResolutionError::EmptyManifestRef);
        }
        // AC2: verification and manifests remain accessible from the same component
        // family regardless of the deployment profile.
        if !artifact.verify_available {
            return Err(M5MirrorTransitionResolutionError::VerificationNotAccessible);
        }
        if !artifact.open_manifest_available {
            return Err(M5MirrorTransitionResolutionError::ManifestNotAccessible);
        }
        // AC1: mirrored / cached / imported content is never shown as a current live
        // source.
        if !artifact.freshness.is_current_source() && !artifact.stale_not_shown_as_current {
            return Err(M5MirrorTransitionResolutionError::StaleShownAsCurrent);
        }

        let continuity_state = M5MirrorContinuityState::derive(
            artifact.freshness,
            artifact.signature_state,
            artifact.mirror_reachable,
            artifact.pinned_offline,
            artifact.needs_refresh,
        );

        // Verify + open-manifest are always present (AC2); refresh and pin are offered
        // when the state calls for them.
        let mut actions = vec![
            M5MirrorArtifactAction::VerifySignature,
            M5MirrorArtifactAction::OpenManifest,
        ];
        if continuity_state.requires_action() {
            actions.push(M5MirrorArtifactAction::RefreshNow);
        }
        if artifact.pinned_offline || continuity_state == M5MirrorContinuityState::OfflineCacheOnly
        {
            actions.push(M5MirrorArtifactAction::PinOffline);
        }

        artifact_rows.push(M5ResolvedMirrorArtifactRow {
            transition_id: input.transition_id.clone(),
            artifact_ref: artifact.artifact_ref.clone(),
            artifact_class: artifact.artifact_class,
            source_class: artifact.source_class,
            freshness: artifact.freshness,
            signature_state: artifact.signature_state,
            continuity_state,
            pinned_offline: artifact.pinned_offline,
            manifest_ref: artifact.manifest_ref.clone(),
            actions,
            discloses_freshness: true,
            stale_not_shown_as_current: true,
            verification_accessible: true,
            exportable: true,
        });
    }

    // The mode-change sheet's overall artifact posture is the worst continuity state
    // across the transition's artifacts.
    let artifact_posture = artifact_rows
        .iter()
        .map(|row| row.continuity_state)
        .max_by_key(|state| state.severity())
        .unwrap_or(M5MirrorContinuityState::CurrentVerified);

    let reversible =
        input.rollback_path_state.is_recoverable() || input.export_before_change_available;

    let mode_change_sheet = M5ResolvedModeChangeReviewSheet {
        transition_id: input.transition_id.clone(),
        from_mode: input.from_mode,
        to_mode: input.to_mode,
        boundary_change: input.boundary_change,
        preserved_local_state_ref: input.preserved_local_state_ref.clone(),
        affected_managed_feature_refs: input.affected_managed_feature_refs.clone(),
        cache_disposition: input.cache_disposition,
        artifact_posture,
        rollback_path_state: input.rollback_path_state,
        reviewed_before_change: true,
        export_before_change_available: true,
        reversible,
        discloses_stale_and_usable: true,
    };

    let channel_row = M5ResolvedChannelAssociationRow {
        transition_id: input.transition_id.clone(),
        channel_ref: input.channel_ref.clone(),
        handler_association_ref: input.handler_association_ref.clone(),
        last_writer_wins_capture: false,
        reviewed_before_apply: true,
        discloses_current_owner: true,
    };

    let verification_accessible_across_profiles =
        artifact_rows.iter().all(|row| row.verification_accessible);

    Ok(M5ResolvedMirrorTransition {
        transition_id: input.transition_id.clone(),
        artifact_rows,
        mode_change_sheet,
        channel_row,
        transition_explicit_not_generic: true,
        verification_accessible_across_profiles,
        export_and_rollback_preserved: true,
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
/// reconstructs transition truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MirrorTransitionCase {
    /// The resolver input.
    pub input: M5MirrorTransitionInput,
    /// The resolved transition truth. Must equal
    /// `resolve_mirror_transition(&input)`.
    pub resolved: M5ResolvedMirrorTransition,
}

impl M5MirrorTransitionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5MirrorTransitionInput) -> Self {
        let resolved = resolve_mirror_transition(&input).expect("seed transition case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_mirror_transition(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one mirror surface family bound to the shared
/// mirror-transition contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MirrorTransitionSurfaceRow {
    /// The mirror surface family.
    pub surface_family: M5MirrorSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Artifact classes this surface can disclose (must be non-empty).
    pub artifact_classes: Vec<M5MirrorArtifactClass>,
    /// Truth classes this surface renders (must be non-empty).
    pub truth_modes: Vec<M5DeploymentTruthMode>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5MirrorTransitionExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5DeploymentDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be non-empty).
    pub example_transitions: Vec<M5MirrorTransitionCase>,
    /// Hard invariant: this row never shows stale content as current. MUST be `false`.
    pub shows_stale_as_current: bool,
    /// Hard invariant: this row never hides verification / manifests. MUST be `false`.
    pub hides_verification: bool,
    /// Hard invariant: this row never forces a blind switch. MUST be `false`.
    pub forces_blind_switch: bool,
    /// Hard invariant: this row never captures a default handler. MUST be `false`.
    pub captures_default_handler: bool,
}

impl M5MirrorTransitionSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5MirrorTransitionExportField> =
            self.export_fields.iter().copied().collect();
        M5MirrorTransitionExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.shows_stale_as_current
            && !self.hides_verification
            && !self.forces_blind_switch
            && !self.captures_default_handler
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MirrorTransitionVocabularySet {
    /// Mirror surface-family tokens.
    pub surface_families: Vec<String>,
    /// Mirror-artifact-class tokens.
    pub artifact_classes: Vec<String>,
    /// Mirror-source-class tokens.
    pub source_classes: Vec<String>,
    /// Mirror/offline continuity-state tokens.
    pub continuity_states: Vec<String>,
    /// Mirror-artifact-action tokens.
    pub artifact_actions: Vec<String>,
    /// Cache-disposition tokens.
    pub cache_dispositions: Vec<String>,
    /// Rollback-path tokens.
    pub rollback_path_states: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Deployment-mode tokens (reused from the frozen matrix).
    pub deployment_modes: Vec<String>,
    /// Truth-class tokens (reused from the frozen matrix).
    pub truth_modes: Vec<String>,
    /// Mirror-signature-state tokens (reused from the frozen matrix).
    pub signature_states: Vec<String>,
    /// Boundary-change-class tokens (reused from the frozen matrix).
    pub boundary_changes: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5MirrorTransitionVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5MirrorSurfaceFamily::ALL, M5MirrorSurfaceFamily::as_str),
            artifact_classes: tokens(&M5MirrorArtifactClass::ALL, M5MirrorArtifactClass::as_str),
            source_classes: tokens(&M5MirrorSourceClass::ALL, M5MirrorSourceClass::as_str),
            continuity_states: tokens(
                &M5MirrorContinuityState::ALL,
                M5MirrorContinuityState::as_str,
            ),
            artifact_actions: tokens(&M5MirrorArtifactAction::ALL, M5MirrorArtifactAction::as_str),
            cache_dispositions: tokens(&M5CacheDisposition::ALL, M5CacheDisposition::as_str),
            rollback_path_states: tokens(&M5RollbackPathState::ALL, M5RollbackPathState::as_str),
            export_fields: tokens(
                &M5MirrorTransitionExportField::ALL,
                M5MirrorTransitionExportField::as_str,
            ),
            deployment_modes: tokens(&DEPLOYMENT_MODE_ALL, M5DeploymentMode::as_str),
            truth_modes: tokens(&TRUTH_MODE_ALL, M5DeploymentTruthMode::as_str),
            signature_states: tokens(&SIGNATURE_STATE_ALL, M5MirrorSignatureState::as_str),
            boundary_changes: tokens(&BOUNDARY_CHANGE_ALL, M5BoundaryChangeClass::as_str),
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

/// The mirror-signature states reused from the frozen matrix, in a stable order.
const SIGNATURE_STATE_ALL: [M5MirrorSignatureState; 4] = [
    M5MirrorSignatureState::Verified,
    M5MirrorSignatureState::Unverified,
    M5MirrorSignatureState::SignatureMismatch,
    M5MirrorSignatureState::VerificationDeferred,
];

/// The boundary-change classes reused from the frozen matrix, in a stable order.
const BOUNDARY_CHANGE_ALL: [M5BoundaryChangeClass; 5] = [
    M5BoundaryChangeClass::StateRootMigration,
    M5BoundaryChangeClass::ChannelSwitch,
    M5BoundaryChangeClass::UpdaterOwnershipChange,
    M5BoundaryChangeClass::MirrorReattach,
    M5BoundaryChangeClass::OnlineOfflineTransition,
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
pub struct M5MirrorTransitionGovernanceReview {
    /// One primitive carries artifact-row / mode-change-sheet / channel-row truth on
    /// every surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Transition identity is preserved across the artifact rows, the sheet, and the row.
    pub transition_identity_preserved_across_surfaces: bool,
    /// Mirrored / cached content is never shown as a current live source.
    pub stale_never_shown_as_current: bool,
    /// Verification and manifests are always accessible from the same component family.
    pub verification_manifest_always_accessible: bool,
    /// Export-before-change and rollback truth are always preserved.
    pub export_before_change_and_rollback_always_preserved: bool,
    /// The support / export packet reconstructs transition truth.
    pub support_export_reconstructs_transition: bool,
    /// Later M5 rows cannot invent parallel mirror/offline vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MirrorTransitionConsumerProjection {
    /// Update-center / mirror-manager / admin / diagnostics surfaces all consume the
    /// shared primitive.
    pub mirror_surfaces_consume_shared_primitive: bool,
    /// The transition resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The mode-change sheet reads a single canonical transition source.
    pub review_sheet_reads_single_transition_source: bool,
    /// Support / export reads a single canonical transition source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the mirror-transition primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MirrorTransitionReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting deployment audit.
    pub deployment_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5MirrorTransitionPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MirrorTransitionPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5MirrorTransitionSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MirrorTransitionVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MirrorTransitionGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MirrorTransitionConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5MirrorTransitionReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 mirror-transition primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MirrorTransitionPrimitivePacket {
    /// Record kind; must equal [`M5_MIRROR_TRANSITION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_MIRROR_TRANSITION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5MirrorTransitionSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MirrorTransitionVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MirrorTransitionGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MirrorTransitionConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5MirrorTransitionReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5MirrorTransitionPrimitivePacket {
    /// Builds an M5 mirror-transition primitive packet from stable-lane input.
    pub fn new(input: M5MirrorTransitionPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_MIRROR_TRANSITION_RECORD_KIND.to_owned(),
            schema_version: M5_MIRROR_TRANSITION_SCHEMA_VERSION,
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

    /// Validates the M5 mirror-transition primitive invariants.
    pub fn validate(&self) -> Vec<M5MirrorTransitionViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_MIRROR_TRANSITION_RECORD_KIND {
            violations.push(M5MirrorTransitionViolation::WrongRecordKind);
        }
        if self.schema_version != M5_MIRROR_TRANSITION_SCHEMA_VERSION {
            violations.push(M5MirrorTransitionViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5MirrorTransitionViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 mirror-transition primitive packet serializes"),
        ) {
            violations.push(M5MirrorTransitionViolation::RawMaterialInExport);
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
            .expect("m5 mirror-transition primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,owner,artifact_classes,truth_modes,export_fields,artifact_rows,example_count\n",
        );
        for row in &self.surface_rows {
            let artifact_rows: usize = row
                .example_transitions
                .iter()
                .map(|case| case.resolved.artifact_rows.len())
                .sum();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.artifact_classes, |v| v.as_str()),
                join_tokens(&row.truth_modes, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                artifact_rows,
                row.example_transitions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Mirror-Transition Primitive: Mirror/Offline Artifact Rows, Mode-Change Review Sheet, and Channel-Association Review Row\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Mirror surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5MirrorSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Artifact classes: {}\n",
            self.vocabulary_set.artifact_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Continuity states: {}\n",
            self.vocabulary_set.continuity_states.join(", ")
        ));
        out.push_str(&format!(
            "- Cache dispositions: {}\n",
            self.vocabulary_set.cache_dispositions.join(", ")
        ));
        out.push_str("\n## Mirror surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked cases: {}\n",
                row.example_transitions.len()
            ));
            for case in &row.example_transitions {
                out.push_str(&format!(
                    "    - `{}` → {}→{} via `{}`, artifacts `{}`, posture `{}`, cache `{}`, rollback `{}`\n",
                    case.resolved.transition_id,
                    case.resolved.mode_change_sheet.from_mode.as_str(),
                    case.resolved.mode_change_sheet.to_mode.as_str(),
                    case.resolved.mode_change_sheet.boundary_change.as_str(),
                    case.resolved.artifact_rows.len(),
                    case.resolved.mode_change_sheet.artifact_posture.as_str(),
                    case.resolved.mode_change_sheet.cache_disposition.as_str(),
                    case.resolved.mode_change_sheet.rollback_path_state.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 mirror-transition export.
#[derive(Debug)]
pub enum M5MirrorTransitionArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5MirrorTransitionViolation>),
}

impl fmt::Display for M5MirrorTransitionArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 mirror-transition primitive export parse failed: {error}"
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
                    "m5 mirror-transition primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5MirrorTransitionArtifactError {}

/// Validation failures emitted by [`M5MirrorTransitionPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5MirrorTransitionViolation {
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
    /// A required mirror surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row declares no artifact classes.
    ArtifactClassMissing,
    /// A surface row declares no truth classes.
    TruthModeMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked transition cases.
    ExampleTransitionsMissing,
    /// A worked transition case does not match a fresh resolve of its input.
    ExampleTransitionDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves an offline / mirror transition rendered explicitly rather
    /// than as a generic warning (AC1).
    TransitionExplicitnessUnproven,
    /// No worked case proves verification / manifests accessible across profiles (AC2).
    VerificationAccessibilityUnproven,
    /// No worked case proves export-before-change and rollback truth preserved (AC3).
    ExportRollbackUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5MirrorTransitionViolation {
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
            Self::ArtifactClassMissing => "artifact_class_missing",
            Self::TruthModeMissing => "truth_mode_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleTransitionsMissing => "example_transitions_missing",
            Self::ExampleTransitionDrift => "example_transition_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::TransitionExplicitnessUnproven => "transition_explicitness_unproven",
            Self::VerificationAccessibilityUnproven => "verification_accessibility_unproven",
            Self::ExportRollbackUnproven => "export_rollback_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 mirror-transition export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_stable_m5_mirror_transition_export(
) -> Result<M5MirrorTransitionPrimitivePacket, M5MirrorTransitionArtifactError> {
    let packet: M5MirrorTransitionPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-mirror-transition-primitive-proof/support_export.json"
    )))
    .map_err(M5MirrorTransitionArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5MirrorTransitionArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5MirrorTransitionPrimitivePacket,
    violations: &mut Vec<M5MirrorTransitionViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_MIRROR_TRANSITION_SCHEMA_REF,
        M5_MIRROR_TRANSITION_DOC_REF,
        M5_MIRROR_TRANSITION_COMPONENT_MATRIX_REF,
        M5_MIRROR_TRANSITION_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5MirrorTransitionViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5MirrorTransitionPrimitivePacket,
    violations: &mut Vec<M5MirrorTransitionViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5MirrorTransitionViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5MirrorTransitionPrimitivePacket,
    violations: &mut Vec<M5MirrorTransitionViolation>,
) {
    let present: BTreeSet<M5MirrorSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5MirrorSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5MirrorTransitionViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5MirrorTransitionViolation::SurfaceRowIncomplete);
        }
        if row.artifact_classes.is_empty() {
            violations.push(M5MirrorTransitionViolation::ArtifactClassMissing);
        }
        if row.truth_modes.is_empty() {
            violations.push(M5MirrorTransitionViolation::TruthModeMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5MirrorTransitionViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5MirrorTransitionViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5MirrorTransitionViolation::ConsumerSurfacesMissing);
        }
        if row.example_transitions.is_empty() {
            violations.push(M5MirrorTransitionViolation::ExampleTransitionsMissing);
        }
        if row
            .example_transitions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5MirrorTransitionViolation::ExampleTransitionDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5MirrorTransitionViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated by at least one worked case across
/// the matrix: an offline / mirror transition rendered explicitly rather than as a
/// generic warning (AC1), verification / manifests accessible across profiles (AC2), and
/// export-before-change and rollback truth preserved (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5MirrorTransitionPrimitivePacket,
    violations: &mut Vec<M5MirrorTransitionViolation>,
) {
    let cases: Vec<&M5ResolvedMirrorTransition> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_transitions.iter().map(|case| &case.resolved))
        .collect();

    // AC1: at least one case is a genuine offline / mirror transition (a stale or
    // blocked artifact) that the sheet still discloses as explicit and usable, and every
    // case keeps its transition explicit and identity consistent.
    let explicit_proven = cases.iter().any(|resolved| {
        resolved.has_stale_or_blocked_artifact()
            && resolved.mode_change_sheet.discloses_stale_and_usable
    }) && cases.iter().all(|resolved| {
        resolved.identity_consistent() && resolved.transition_explicit_not_generic()
    });
    if !explicit_proven {
        violations.push(M5MirrorTransitionViolation::TransitionExplicitnessUnproven);
    }

    // AC2: at least one case carries artifacts spanning more than one deployment profile
    // (source class), and every artifact row keeps verification / manifests accessible.
    let distinct_sources: BTreeSet<M5MirrorSourceClass> = cases
        .iter()
        .flat_map(|resolved| resolved.artifact_rows.iter().map(|row| row.source_class))
        .collect();
    let verification_proven = distinct_sources.len() >= 2
        && cases.iter().all(|resolved| {
            resolved.verification_accessible_across_profiles()
                && resolved.artifact_rows.iter().all(|row| {
                    row.verification_accessible
                        && row
                            .actions
                            .contains(&M5MirrorArtifactAction::VerifySignature)
                        && row.actions.contains(&M5MirrorArtifactAction::OpenManifest)
                })
        });
    if !verification_proven {
        violations.push(M5MirrorTransitionViolation::VerificationAccessibilityUnproven);
    }

    // AC3: at least one case is a durable change with a recoverable rollback path and an
    // export-before-change action, and every case preserves export and rollback truth.
    let export_proven = cases.iter().any(|resolved| {
        resolved
            .mode_change_sheet
            .rollback_path_state
            .is_recoverable()
            && resolved.mode_change_sheet.export_before_change_available
    }) && cases
        .iter()
        .all(|resolved| resolved.export_and_rollback_preserved());
    if !export_proven {
        violations.push(M5MirrorTransitionViolation::ExportRollbackUnproven);
    }
}

fn validate_governance_review(
    packet: &M5MirrorTransitionPrimitivePacket,
    violations: &mut Vec<M5MirrorTransitionViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.transition_identity_preserved_across_surfaces,
        review.stale_never_shown_as_current,
        review.verification_manifest_always_accessible,
        review.export_before_change_and_rollback_always_preserved,
        review.support_export_reconstructs_transition,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5MirrorTransitionViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5MirrorTransitionPrimitivePacket,
    violations: &mut Vec<M5MirrorTransitionViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.mirror_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.review_sheet_reads_single_transition_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5MirrorTransitionViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5MirrorTransitionPrimitivePacket,
    violations: &mut Vec<M5MirrorTransitionViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.deployment_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5MirrorTransitionViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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
