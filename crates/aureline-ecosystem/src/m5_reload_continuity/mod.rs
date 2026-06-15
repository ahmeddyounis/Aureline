//! Canonical M5 hot-reload/relaunch banners and last-loaded-build continuity for
//! local or side-loaded extension packages.
//!
//! Where [`crate::m5_workspace_strip`] is the always-on authoring chrome and
//! [`crate::m5_author_and_publish_preview`] is the publish-control gate, this module
//! freezes the **reload-continuity card**: the banner an author reads when local
//! package code or its manifest changes, plus the last-loaded-build continuity that
//! keeps a package from disappearing when its source path moves or a new build fails.
//! Each [`ReloadContinuityCard`] reuses the shared [`ArtifactFamily`], [`RuntimeClass`],
//! [`HostAbiClass`], [`SignatureState`], [`TrustPosture`], [`HotReloadPosture`],
//! [`WorkspaceOrigin`], [`BuildFreshness`], and [`LoadState`] vocabulary so the banner,
//! the strip, and the publish gate describe one artifact without a parallel synonym set.
//!
//! The card is a render-truth object. From the observed build, load, hot-reload, and
//! source-availability states it recomputes:
//!
//! - the **rendered trust posture** the banner may display — capped by *both* the
//!   signing state *and* the workspace origin, so a local-dev or side-loaded package
//!   never inherits a verified-publisher or enterprise-approved badge through a reload;
//! - the **continuity state** the package degrades to — [`ContinuityState::SourceUnavailable`],
//!   [`ContinuityState::BuildFailed`], or [`ContinuityState::LastLoadedBuildStillActive`]
//!   rather than disappearing — so a package whose source path moved or whose new build
//!   failed keeps a last-loaded build visible instead of vanishing;
//! - the **state-impact banner**: a [`RestartScope`] (what restarts), a [`PreservedState`]
//!   (what state is preserved versus reset), a [`WideningReview`] (what permission/ABI
//!   drift forces a fresh review), and a [`RollbackPath`] (what rollback path exists); and
//! - whether the card **requires a fresh review** — a hot reload that would widen the
//!   runtime class, expand permissions, or add an external executable holds the running
//!   instance in [`LoadState::ReloadHeldForReview`] / [`RestartScope::HeldPendingReview`]
//!   rather than widening authority silently.
//!
//! Two continuity invariants make the row more than a banner painter. A
//! [`LoadState::LoadedCurrentBuild`] card must actually be running the current build
//! from present source, so a "loaded current" claim stays honest; and a card whose
//! running instance is serving a last-loaded build must carry its
//! [`ReloadContinuityCard::last_loaded_build_ref`], so the continuity record is never
//! lost when the source path or build is broken. [`M5ReloadContinuityBoard::validate`]
//! enforces both, and [`M5ReloadContinuityBoard::cross_check_matrix`] proves the banner
//! never renders a stronger badge than the publish-preview gate would grant the same
//! family, so authoring chrome, install/update flows, diagnostics, support, and release
//! surfaces project one trust truth.
//!
//! The packet is checked in at
//! `artifacts/ecosystem/m5/m5-reload-continuity.json` and embedded here, so this typed
//! consumer and any CI gate agree on every card without a cargo build in CI. The model
//! is metadata-only: every field is a typed state or an opaque ref. It carries no
//! credential bodies, raw provider payloads, signing secrets, or source bodies — the
//! `source_path_ref` is an opaque workspace ref, never a verbatim filesystem path.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::m5_author_and_publish_preview::{
    ArtifactFamily, HostAbiClass, HotReloadPosture, M5AuthorPublishMatrix, RuntimeClass,
    SignatureState, TrustPosture,
};
pub use crate::m5_workspace_strip::{
    hot_reload_widens_authority, BuildFreshness, LoadState, WorkspaceOrigin,
};

/// Supported M5 reload-continuity board schema version.
pub const M5_RELOAD_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_RELOAD_CONTINUITY_RECORD_KIND: &str = "m5_reload_continuity_board";

/// Repo-relative path to the checked-in packet.
pub const M5_RELOAD_CONTINUITY_PATH: &str = "artifacts/ecosystem/m5/m5-reload-continuity.json";

/// Embedded checked-in packet JSON.
pub const M5_RELOAD_CONTINUITY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-reload-continuity.json"
));

/// Availability of the package's workspace source path.
///
/// The source-availability fact distinguishes a present source from one that moved or
/// disappeared, so a package whose source path is gone can degrade to a continuity state
/// rather than vanishing from the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceAvailability {
    /// The source path resolves at its recorded location.
    SourcePresent,
    /// The source path moved; it is no longer at its recorded location.
    SourceMoved,
    /// The source path is gone and does not resolve.
    SourceUnavailable,
}

impl SourceAvailability {
    /// Every source-availability state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::SourcePresent,
        Self::SourceMoved,
        Self::SourceUnavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourcePresent => "source_present",
            Self::SourceMoved => "source_moved",
            Self::SourceUnavailable => "source_unavailable",
        }
    }

    /// Whether the source is resolvable at its recorded location.
    pub const fn is_present(self) -> bool {
        matches!(self, Self::SourcePresent)
    }

    /// Whether the source path moved or disappeared.
    pub const fn is_lost(self) -> bool {
        matches!(self, Self::SourceMoved | Self::SourceUnavailable)
    }
}

/// Continuity state a local or side-loaded package degrades to.
///
/// A package never disappears when its source path moves or a new build fails: it
/// degrades to [`ContinuityState::SourceUnavailable`], [`ContinuityState::BuildFailed`],
/// or [`ContinuityState::LastLoadedBuildStillActive`] — the last loaded build keeps
/// running while the broken state is named explicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityState {
    /// The running instance is serving the current build from present source.
    LoadedCurrentBuild,
    /// A running instance is still serving the last loaded build while the source or a
    /// newer build has moved on; the last-loaded record is retained.
    LastLoadedBuildStillActive,
    /// The source path moved or disappeared and no instance is running it.
    SourceUnavailable,
    /// The current build failed or failed to load and no instance is running it.
    BuildFailed,
    /// The package is not currently loaded for a benign reason; no failure.
    NotLoaded,
}

impl ContinuityState {
    /// Every continuity state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LoadedCurrentBuild,
        Self::LastLoadedBuildStillActive,
        Self::SourceUnavailable,
        Self::BuildFailed,
        Self::NotLoaded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LoadedCurrentBuild => "loaded_current_build",
            Self::LastLoadedBuildStillActive => "last_loaded_build_still_active",
            Self::SourceUnavailable => "source_unavailable",
            Self::BuildFailed => "build_failed",
            Self::NotLoaded => "not_loaded",
        }
    }

    /// Whether this state is a degraded state the package fell back to.
    pub const fn is_degraded(self) -> bool {
        matches!(
            self,
            Self::LastLoadedBuildStillActive | Self::SourceUnavailable | Self::BuildFailed
        )
    }

    /// Whether this state requires a retained last-loaded-build record.
    ///
    /// A running instance serving a last-loaded build must carry its continuity record,
    /// so the record is never lost when the source path or build is broken.
    pub const fn requires_retained_record(self) -> bool {
        matches!(self, Self::LastLoadedBuildStillActive)
    }
}

/// What restarts when the pending reload or relaunch is applied — the banner's
/// "what restarts" answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartScope {
    /// Hot reload applies in place; nothing restarts.
    NothingRestarts,
    /// The host instance relaunches to pick up the build.
    HostInstanceRelaunches,
    /// A widening reload is held pending a fresh review; nothing restarts yet.
    HeldPendingReview,
    /// There is no running instance to restart.
    NoRunningInstance,
}

impl RestartScope {
    /// Every restart scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NothingRestarts,
        Self::HostInstanceRelaunches,
        Self::HeldPendingReview,
        Self::NoRunningInstance,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NothingRestarts => "nothing_restarts",
            Self::HostInstanceRelaunches => "host_instance_relaunches",
            Self::HeldPendingReview => "held_pending_review",
            Self::NoRunningInstance => "no_running_instance",
        }
    }

    /// The preserved-state the banner pairs with this restart scope.
    pub const fn preserved_state(self) -> PreservedState {
        match self {
            Self::NothingRestarts => PreservedState::InMemoryAndPersistedPreserved,
            Self::HostInstanceRelaunches => PreservedState::PersistedPreservedInMemoryReset,
            Self::HeldPendingReview => PreservedState::RunningInstanceUnchanged,
            Self::NoRunningInstance => PreservedState::NoRunningState,
        }
    }
}

/// What state is preserved versus reset across the reload — the banner's
/// "what state is preserved" answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreservedState {
    /// A hot reload preserves both in-memory and persisted state.
    InMemoryAndPersistedPreserved,
    /// A relaunch preserves persisted/user state but resets in-memory state.
    PersistedPreservedInMemoryReset,
    /// The running instance is held unchanged; all its state survives.
    RunningInstanceUnchanged,
    /// Nothing is running, so there is no running state to preserve.
    NoRunningState,
}

impl PreservedState {
    /// Every preserved-state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InMemoryAndPersistedPreserved,
        Self::PersistedPreservedInMemoryReset,
        Self::RunningInstanceUnchanged,
        Self::NoRunningState,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InMemoryAndPersistedPreserved => "in_memory_and_persisted_preserved",
            Self::PersistedPreservedInMemoryReset => "persisted_preserved_in_memory_reset",
            Self::RunningInstanceUnchanged => "running_instance_unchanged",
            Self::NoRunningState => "no_running_state",
        }
    }
}

/// What permission/ABI drift forces a fresh review — the banner's
/// "what requires fresh review" answer.
///
/// Derived 1:1 from the [`HotReloadPosture`] so the banner and the publish gate name the
/// same widening cause.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WideningReview {
    /// No widening; the reload does not require a fresh review.
    NoWidening,
    /// The reload would widen the runtime class; a fresh review is required.
    RuntimeClassWideningRequiresReview,
    /// The reload would expand permissions; a fresh review is required.
    PermissionWideningRequiresReview,
    /// The reload would add an external executable; a fresh review is required.
    ExternalExecutableRequiresReview,
}

impl WideningReview {
    /// Every widening-review state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoWidening,
        Self::RuntimeClassWideningRequiresReview,
        Self::PermissionWideningRequiresReview,
        Self::ExternalExecutableRequiresReview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoWidening => "no_widening",
            Self::RuntimeClassWideningRequiresReview => "runtime_class_widening_requires_review",
            Self::PermissionWideningRequiresReview => "permission_widening_requires_review",
            Self::ExternalExecutableRequiresReview => "external_executable_requires_review",
        }
    }

    /// The widening review a hot-reload posture maps to.
    pub const fn from_posture(posture: HotReloadPosture) -> Self {
        match posture {
            HotReloadPosture::NoWidening | HotReloadPosture::RelaunchOnly => Self::NoWidening,
            HotReloadPosture::RuntimeClassWidenedPendingReview => {
                Self::RuntimeClassWideningRequiresReview
            }
            HotReloadPosture::PermissionsWidenedPendingReview => {
                Self::PermissionWideningRequiresReview
            }
            HotReloadPosture::ExternalExecutableAddedPendingReview => {
                Self::ExternalExecutableRequiresReview
            }
        }
    }

    /// Whether this widening review requires a fresh review step.
    pub const fn requires_review(self) -> bool {
        !matches!(self, Self::NoWidening)
    }
}

/// What rollback path exists for the package — the banner's "what rollback path" answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackPath {
    /// Revert to the retained last-loaded build (running or recorded).
    RevertToLastLoadedBuild,
    /// Rebuild and relaunch from the current source.
    RelaunchFromCurrentSource,
    /// No rollback: the source is unavailable and no last-loaded record was retained.
    NoRollbackSourceUnavailable,
    /// No rollback path exists.
    NoRollbackPath,
}

impl RollbackPath {
    /// Every rollback path, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RevertToLastLoadedBuild,
        Self::RelaunchFromCurrentSource,
        Self::NoRollbackSourceUnavailable,
        Self::NoRollbackPath,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevertToLastLoadedBuild => "revert_to_last_loaded_build",
            Self::RelaunchFromCurrentSource => "relaunch_from_current_source",
            Self::NoRollbackSourceUnavailable => "no_rollback_source_unavailable",
            Self::NoRollbackPath => "no_rollback_path",
        }
    }
}

/// One reload-continuity card for a marketed M5 artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReloadContinuityCard {
    /// Stable card id.
    pub card_id: String,
    /// Marketed M5 artifact family this card governs.
    pub artifact_family: ArtifactFamily,
    /// Author-facing package identity (display name/id; no secrets).
    pub package_identity: String,
    /// Opaque ref to the workspace source path (never a verbatim filesystem path).
    pub source_path_ref: String,
    /// Origin the workspace is anchored to.
    pub origin: WorkspaceOrigin,
    /// Runtime class of the authored artifact.
    pub runtime_class: RuntimeClass,
    /// Host/ABI execution locus.
    pub host_abi: HostAbiClass,
    /// Signing/provenance state.
    pub signature_state: SignatureState,
    /// Trust posture the author requests, before the card caps it.
    pub declared_trust_posture: TrustPosture,
    /// Trust posture the banner actually renders after capping.
    ///
    /// Must equal [`ReloadContinuityCard::effective_trust_posture`].
    pub rendered_trust_posture: TrustPosture,
    /// Build freshness / last-built fact.
    pub build_freshness: BuildFreshness,
    /// Load state / last-loaded and hot-reload fact.
    pub load_state: LoadState,
    /// Hot-reload/relaunch posture.
    pub hot_reload_posture: HotReloadPosture,
    /// Availability of the workspace source path.
    pub source_availability: SourceAvailability,
    /// Continuity state the package degrades to.
    ///
    /// Must equal [`ReloadContinuityCard::computed_continuity_state`].
    pub continuity_state: ContinuityState,
    /// What restarts when the pending reload/relaunch is applied.
    ///
    /// Must equal [`ReloadContinuityCard::computed_restart_scope`].
    pub restart_scope: RestartScope,
    /// What state is preserved versus reset.
    ///
    /// Must equal [`ReloadContinuityCard::computed_preserved_state`].
    pub preserved_state: PreservedState,
    /// What permission/ABI drift forces a fresh review.
    ///
    /// Must equal [`ReloadContinuityCard::computed_widening_review`].
    pub widening_review: WideningReview,
    /// What rollback path exists.
    ///
    /// Must equal [`ReloadContinuityCard::computed_rollback_path`].
    pub rollback_path: RollbackPath,
    /// Opaque ref to the retained last-loaded-build continuity record, when one exists.
    #[serde(default)]
    pub last_loaded_build_ref: Option<String>,
    /// Ref to the family's local workspace strip.
    pub workspace_strip_ref: String,
    /// Ref to the family's publish-preview gate row.
    pub publish_preview_ref: String,
    /// Ref binding this card into diagnostics, support, and release surfaces.
    pub support_export_ref: String,
    /// Additional source refs backing the card.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl ReloadContinuityCard {
    /// The trust posture the banner may render for this package.
    ///
    /// Lowers the author's declared posture to the *minimum* of the signing-state
    /// ceiling and the origin ceiling, so a local-dev or side-loaded package renders
    /// local-only even when signed, and an unsigned or revoked artifact never inherits a
    /// trusted-publisher badge through a reload.
    pub fn effective_trust_posture(&self) -> TrustPosture {
        self.declared_trust_posture
            .min(self.signature_state.trust_ceiling())
            .min(self.origin.trust_ceiling())
    }

    /// Whether the banner renders as a local-only artifact (no inherited trust badge).
    pub fn is_local_only(&self) -> bool {
        self.effective_trust_posture() == TrustPosture::UnsignedLocalOnly
    }

    /// Whether the card requires a fresh review before a hot reload takes effect.
    pub fn requires_fresh_review(&self) -> bool {
        hot_reload_widens_authority(self.hot_reload_posture)
    }

    /// Whether a running instance is currently serving a build.
    pub fn is_running(&self) -> bool {
        matches!(
            self.load_state,
            LoadState::LoadedCurrentBuild
                | LoadState::ReloadPendingRelaunch
                | LoadState::ReloadHeldForReview
        )
    }

    /// Whether the continuity record is retained.
    pub fn continuity_record_retained(&self) -> bool {
        self.last_loaded_build_ref
            .as_ref()
            .is_some_and(|r| !r.trim().is_empty())
    }

    /// The continuity state recomputed from the observed facts.
    pub fn computed_continuity_state(&self) -> ContinuityState {
        if self.is_running() {
            if self.load_state == LoadState::LoadedCurrentBuild {
                ContinuityState::LoadedCurrentBuild
            } else {
                // A reload is pending or held; the instance is still serving the build it
                // last loaded while the source or a newer build has moved on.
                ContinuityState::LastLoadedBuildStillActive
            }
        } else if self.source_availability.is_lost() {
            ContinuityState::SourceUnavailable
        } else if self.build_freshness == BuildFreshness::BuildFailed
            || self.load_state == LoadState::LoadFailed
        {
            ContinuityState::BuildFailed
        } else {
            ContinuityState::NotLoaded
        }
    }

    /// The restart scope recomputed from the load state.
    pub fn computed_restart_scope(&self) -> RestartScope {
        match self.load_state {
            LoadState::ReloadHeldForReview => RestartScope::HeldPendingReview,
            LoadState::ReloadPendingRelaunch => RestartScope::HostInstanceRelaunches,
            LoadState::LoadedCurrentBuild => RestartScope::NothingRestarts,
            LoadState::NotLoaded | LoadState::LoadFailed => RestartScope::NoRunningInstance,
        }
    }

    /// The preserved-state recomputed from the restart scope.
    pub fn computed_preserved_state(&self) -> PreservedState {
        self.computed_restart_scope().preserved_state()
    }

    /// The widening review recomputed from the hot-reload posture.
    pub fn computed_widening_review(&self) -> WideningReview {
        WideningReview::from_posture(self.hot_reload_posture)
    }

    /// The rollback path recomputed from the continuity state and retained record.
    pub fn computed_rollback_path(&self) -> RollbackPath {
        match self.computed_continuity_state() {
            ContinuityState::LoadedCurrentBuild => RollbackPath::RelaunchFromCurrentSource,
            ContinuityState::LastLoadedBuildStillActive => RollbackPath::RevertToLastLoadedBuild,
            ContinuityState::BuildFailed => {
                if self.continuity_record_retained() {
                    RollbackPath::RevertToLastLoadedBuild
                } else {
                    RollbackPath::NoRollbackPath
                }
            }
            ContinuityState::SourceUnavailable => {
                if self.continuity_record_retained() {
                    RollbackPath::RevertToLastLoadedBuild
                } else {
                    RollbackPath::NoRollbackSourceUnavailable
                }
            }
            ContinuityState::NotLoaded => {
                if self.continuity_record_retained() {
                    RollbackPath::RevertToLastLoadedBuild
                } else {
                    RollbackPath::RelaunchFromCurrentSource
                }
            }
        }
    }

    /// Whether the stored rendered posture and every derived banner field agree with the
    /// recomputed card decision.
    pub fn card_consistent(&self) -> bool {
        self.rendered_trust_posture == self.effective_trust_posture()
            && self.continuity_state == self.computed_continuity_state()
            && self.restart_scope == self.computed_restart_scope()
            && self.preserved_state == self.computed_preserved_state()
            && self.widening_review == self.computed_widening_review()
            && self.rollback_path == self.computed_rollback_path()
            && self.review_gate_consistent()
            && self.loaded_current_build_honest()
            && self.continuity_record_present_when_required()
    }

    /// Whether the load state and hot-reload posture agree on the review gate.
    ///
    /// A widening hot reload must hold the running instance for review, and an instance
    /// held for review must be backed by a widening hot reload.
    fn review_gate_consistent(&self) -> bool {
        self.requires_fresh_review() == (self.load_state == LoadState::ReloadHeldForReview)
    }

    /// Whether a `loaded_current_build` load is backed by a current build and present
    /// source, so a "loaded current" claim stays honest.
    fn loaded_current_build_honest(&self) -> bool {
        if self.load_state != LoadState::LoadedCurrentBuild {
            return true;
        }
        self.build_freshness == BuildFreshness::BuiltFromCurrentSource
            && self.source_availability == SourceAvailability::SourcePresent
    }

    /// Whether the continuity record is present when the state requires it, so the
    /// last-loaded-build record is never lost when source or build is broken.
    fn continuity_record_present_when_required(&self) -> bool {
        !self.computed_continuity_state().requires_retained_record()
            || self.continuity_record_retained()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ReloadContinuitySummary {
    /// Total cards.
    pub total_cards: usize,
    /// Number of marketed families claimed.
    pub family_count: usize,
    /// Cards rendered as local-only (no inherited trust badge).
    pub local_only_cards: usize,
    /// Cards running the current build.
    pub loaded_current_build_cards: usize,
    /// Cards whose last-loaded build is still active while source/build moved on.
    pub last_loaded_still_active_cards: usize,
    /// Cards degraded to source unavailable.
    pub source_unavailable_cards: usize,
    /// Cards degraded to build failed.
    pub build_failed_cards: usize,
    /// Cards not currently loaded for a benign reason.
    pub not_loaded_cards: usize,
    /// Cards whose running instance is held pending a fresh review.
    pub held_pending_review_cards: usize,
    /// Cards with a host relaunch pending.
    pub relaunch_pending_cards: usize,
    /// Cards whose reload widens authority and requires a fresh review.
    pub widening_review_cards: usize,
    /// Cards that retained a last-loaded-build continuity record.
    pub retained_continuity_record_cards: usize,
    /// Cards rendering a verified-publisher or enterprise-approved badge.
    pub verified_or_enterprise_rendered_cards: usize,
}

/// A redaction-safe export row projected from a reload-continuity card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReloadContinuityExportRow {
    /// Card id.
    pub card_id: String,
    /// Artifact-family token.
    pub artifact_family: String,
    /// Author-facing package identity.
    pub package_identity: String,
    /// Origin token.
    pub origin: String,
    /// Runtime-class token.
    pub runtime_class: String,
    /// Host/ABI token.
    pub host_abi: String,
    /// Signing-state token.
    pub signature_state: String,
    /// Rendered trust-posture token.
    pub rendered_trust_posture: String,
    /// Build-freshness token.
    pub build_freshness: String,
    /// Load-state token.
    pub load_state: String,
    /// Hot-reload-posture token.
    pub hot_reload_posture: String,
    /// Source-availability token.
    pub source_availability: String,
    /// Continuity-state token.
    pub continuity_state: String,
    /// Restart-scope token.
    pub restart_scope: String,
    /// Preserved-state token.
    pub preserved_state: String,
    /// Widening-review token.
    pub widening_review: String,
    /// Rollback-path token.
    pub rollback_path: String,
    /// Whether the banner renders as local-only.
    pub local_only: bool,
    /// Whether the reload requires a fresh review.
    pub requires_fresh_review: bool,
    /// Whether the continuity record is retained.
    pub continuity_record_retained: bool,
    /// Human-readable banner summary.
    pub banner: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReloadContinuityExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected cards.
    pub cards: Vec<M5ReloadContinuityExportRow>,
    /// Whether every card's stored decision agrees with the recomputed card.
    pub all_cards_consistent: bool,
    /// Cards rendered as local-only.
    pub local_only_count: usize,
    /// Cards held pending a fresh review.
    pub held_pending_review_count: usize,
    /// Cards in a degraded continuity state.
    pub degraded_count: usize,
}

/// The typed M5 reload-continuity board packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ReloadContinuityBoard {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Marketed families the packet claims; one card per family.
    pub artifact_families: Vec<ArtifactFamily>,
    /// Closed runtime-class vocabulary.
    pub runtime_classes: Vec<RuntimeClass>,
    /// Closed host/ABI vocabulary.
    pub host_abi_classes: Vec<HostAbiClass>,
    /// Closed workspace-origin vocabulary.
    pub workspace_origins: Vec<WorkspaceOrigin>,
    /// Closed signing-state vocabulary.
    pub signature_states: Vec<SignatureState>,
    /// Closed trust-posture vocabulary.
    pub trust_postures: Vec<TrustPosture>,
    /// Closed build-freshness vocabulary.
    pub build_freshness_states: Vec<BuildFreshness>,
    /// Closed load-state vocabulary.
    pub load_states: Vec<LoadState>,
    /// Closed hot-reload-posture vocabulary.
    pub hot_reload_postures: Vec<HotReloadPosture>,
    /// Closed source-availability vocabulary.
    pub source_availability_states: Vec<SourceAvailability>,
    /// Closed continuity-state vocabulary.
    pub continuity_states: Vec<ContinuityState>,
    /// Closed restart-scope vocabulary.
    pub restart_scopes: Vec<RestartScope>,
    /// Closed preserved-state vocabulary.
    pub preserved_states: Vec<PreservedState>,
    /// Closed widening-review vocabulary.
    pub widening_reviews: Vec<WideningReview>,
    /// Closed rollback-path vocabulary.
    pub rollback_paths: Vec<RollbackPath>,
    /// Cards, one per marketed family.
    #[serde(default)]
    pub cards: Vec<ReloadContinuityCard>,
    /// Summary counts.
    pub summary: M5ReloadContinuitySummary,
}

impl M5ReloadContinuityBoard {
    /// Returns the card for a marketed family.
    pub fn card(&self, family: ArtifactFamily) -> Option<&ReloadContinuityCard> {
        self.cards.iter().find(|c| c.artifact_family == family)
    }

    /// Cards rendered as local-only.
    pub fn local_only_cards(&self) -> impl Iterator<Item = &ReloadContinuityCard> {
        self.cards.iter().filter(|c| c.is_local_only())
    }

    /// Cards whose running instance is held pending a fresh review.
    pub fn held_pending_review_cards(&self) -> impl Iterator<Item = &ReloadContinuityCard> {
        self.cards
            .iter()
            .filter(|c| c.restart_scope == RestartScope::HeldPendingReview)
    }

    /// Cards in a degraded continuity state.
    pub fn degraded_cards(&self) -> impl Iterator<Item = &ReloadContinuityCard> {
        self.cards
            .iter()
            .filter(|c| c.continuity_state.is_degraded())
    }

    /// Whether every card's stored decision agrees with the recomputed card.
    pub fn all_cards_consistent(&self) -> bool {
        self.cards.iter().all(|c| c.card_consistent())
    }

    /// Recomputes the summary block from the cards.
    pub fn computed_summary(&self) -> M5ReloadContinuitySummary {
        let count_continuity = |state: ContinuityState| {
            self.cards
                .iter()
                .filter(|c| c.continuity_state == state)
                .count()
        };
        M5ReloadContinuitySummary {
            total_cards: self.cards.len(),
            family_count: self.artifact_families.len(),
            local_only_cards: self.local_only_cards().count(),
            loaded_current_build_cards: count_continuity(ContinuityState::LoadedCurrentBuild),
            last_loaded_still_active_cards: count_continuity(
                ContinuityState::LastLoadedBuildStillActive,
            ),
            source_unavailable_cards: count_continuity(ContinuityState::SourceUnavailable),
            build_failed_cards: count_continuity(ContinuityState::BuildFailed),
            not_loaded_cards: count_continuity(ContinuityState::NotLoaded),
            held_pending_review_cards: self.held_pending_review_cards().count(),
            relaunch_pending_cards: self
                .cards
                .iter()
                .filter(|c| c.restart_scope == RestartScope::HostInstanceRelaunches)
                .count(),
            widening_review_cards: self
                .cards
                .iter()
                .filter(|c| c.widening_review.requires_review())
                .count(),
            retained_continuity_record_cards: self
                .cards
                .iter()
                .filter(|c| c.continuity_record_retained())
                .count(),
            verified_or_enterprise_rendered_cards: self
                .cards
                .iter()
                .filter(|c| c.rendered_trust_posture.is_trusted_badge())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — authoring chrome,
    /// install/update flows, diagnostics, support, and release surfaces — render instead
    /// of restating reload-continuity status text by hand.
    pub fn export_projection(&self) -> M5ReloadContinuityExportProjection {
        let cards = self
            .cards
            .iter()
            .map(|c| M5ReloadContinuityExportRow {
                card_id: c.card_id.clone(),
                artifact_family: c.artifact_family.as_str().to_owned(),
                package_identity: c.package_identity.clone(),
                origin: c.origin.as_str().to_owned(),
                runtime_class: c.runtime_class.as_str().to_owned(),
                host_abi: c.host_abi.as_str().to_owned(),
                signature_state: c.signature_state.as_str().to_owned(),
                rendered_trust_posture: c.rendered_trust_posture.as_str().to_owned(),
                build_freshness: c.build_freshness.as_str().to_owned(),
                load_state: c.load_state.as_str().to_owned(),
                hot_reload_posture: c.hot_reload_posture.as_str().to_owned(),
                source_availability: c.source_availability.as_str().to_owned(),
                continuity_state: c.continuity_state.as_str().to_owned(),
                restart_scope: c.restart_scope.as_str().to_owned(),
                preserved_state: c.preserved_state.as_str().to_owned(),
                widening_review: c.widening_review.as_str().to_owned(),
                rollback_path: c.rollback_path.as_str().to_owned(),
                local_only: c.is_local_only(),
                requires_fresh_review: c.requires_fresh_review(),
                continuity_record_retained: c.continuity_record_retained(),
                banner: format!(
                    "{}: continuity {}, restarts {}, preserves {}, review {}, rollback {}; rendered {}{}",
                    c.artifact_family.as_str(),
                    c.continuity_state.as_str(),
                    c.restart_scope.as_str(),
                    c.preserved_state.as_str(),
                    c.widening_review.as_str(),
                    c.rollback_path.as_str(),
                    c.rendered_trust_posture.as_str(),
                    if c.is_local_only() { " (local-only)" } else { "" },
                ),
            })
            .collect();
        M5ReloadContinuityExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            cards,
            all_cards_consistent: self.all_cards_consistent(),
            local_only_count: self.local_only_cards().count(),
            held_pending_review_count: self.held_pending_review_cards().count(),
            degraded_count: self.degraded_cards().count(),
        }
    }

    /// Cross-checks the cards against the publish-preview gate.
    ///
    /// Proves the banner never renders a *stronger* trust badge than the publish-preview
    /// gate would grant the same family — a reload can never widen the rendered trust
    /// above the publish gate — so the banner and the publish preview project one trust
    /// truth.
    pub fn cross_check_matrix(
        &self,
        matrix: &M5AuthorPublishMatrix,
    ) -> Vec<M5ReloadContinuityViolation> {
        let mut violations = Vec::new();
        for card in &self.cards {
            match matrix.family(card.artifact_family) {
                None => violations.push(M5ReloadContinuityViolation::MissingMatrixRow {
                    card_id: card.card_id.clone(),
                    family: card.artifact_family.as_str(),
                }),
                Some(row) => {
                    if card.rendered_trust_posture.rank() > row.published_trust_posture.rank() {
                        violations.push(M5ReloadContinuityViolation::CardExceedsPublishGate {
                            card_id: card.card_id.clone(),
                            rendered: card.rendered_trust_posture.as_str(),
                            published: row.published_trust_posture.as_str(),
                        });
                    }
                }
            }
        }
        violations
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5ReloadContinuityViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<ArtifactFamily> = self.artifact_families.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for card in &self.cards {
            if !seen_ids.insert(card.card_id.clone()) {
                violations.push(M5ReloadContinuityViolation::DuplicateCardId {
                    card_id: card.card_id.clone(),
                });
            }
            if !seen_families.insert(card.artifact_family) {
                violations.push(M5ReloadContinuityViolation::DuplicateFamilyCard {
                    family: card.artifact_family.as_str(),
                });
            }
            if !claimed.contains(&card.artifact_family) {
                violations.push(M5ReloadContinuityViolation::UnclaimedFamilyCard {
                    card_id: card.card_id.clone(),
                    family: card.artifact_family.as_str(),
                });
            }
            self.validate_card(card, &mut violations);
        }

        // Every claimed family must carry its own card, so a degraded package never
        // disappears from the board by losing its row.
        for &family in &self.artifact_families {
            if !seen_families.contains(&family) {
                violations.push(M5ReloadContinuityViolation::MissingFamilyCard {
                    family: family.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5ReloadContinuityViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5ReloadContinuityViolation>) {
        if self.schema_version != M5_RELOAD_CONTINUITY_SCHEMA_VERSION {
            violations.push(M5ReloadContinuityViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_RELOAD_CONTINUITY_RECORD_KIND {
            violations.push(M5ReloadContinuityViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ReloadContinuityViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "artifact_families",
                self.artifact_families == ArtifactFamily::ALL.to_vec(),
            ),
            (
                "runtime_classes",
                self.runtime_classes == RuntimeClass::ALL.to_vec(),
            ),
            (
                "host_abi_classes",
                self.host_abi_classes == HostAbiClass::ALL.to_vec(),
            ),
            (
                "workspace_origins",
                self.workspace_origins == WorkspaceOrigin::ALL.to_vec(),
            ),
            (
                "signature_states",
                self.signature_states == SignatureState::ALL.to_vec(),
            ),
            (
                "trust_postures",
                self.trust_postures == TrustPosture::ALL.to_vec(),
            ),
            (
                "build_freshness_states",
                self.build_freshness_states == BuildFreshness::ALL.to_vec(),
            ),
            ("load_states", self.load_states == LoadState::ALL.to_vec()),
            (
                "hot_reload_postures",
                self.hot_reload_postures == HotReloadPosture::ALL.to_vec(),
            ),
            (
                "source_availability_states",
                self.source_availability_states == SourceAvailability::ALL.to_vec(),
            ),
            (
                "continuity_states",
                self.continuity_states == ContinuityState::ALL.to_vec(),
            ),
            (
                "restart_scopes",
                self.restart_scopes == RestartScope::ALL.to_vec(),
            ),
            (
                "preserved_states",
                self.preserved_states == PreservedState::ALL.to_vec(),
            ),
            (
                "widening_reviews",
                self.widening_reviews == WideningReview::ALL.to_vec(),
            ),
            (
                "rollback_paths",
                self.rollback_paths == RollbackPath::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5ReloadContinuityViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_card(
        &self,
        card: &ReloadContinuityCard,
        violations: &mut Vec<M5ReloadContinuityViolation>,
    ) {
        for (field, value) in [
            ("card_id", &card.card_id),
            ("package_identity", &card.package_identity),
            ("source_path_ref", &card.source_path_ref),
            ("workspace_strip_ref", &card.workspace_strip_ref),
            ("publish_preview_ref", &card.publish_preview_ref),
            ("support_export_ref", &card.support_export_ref),
            ("note", &card.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ReloadContinuityViolation::EmptyField {
                    id: card.card_id.clone(),
                    field_name: field,
                });
            }
        }
        if card
            .last_loaded_build_ref
            .as_ref()
            .is_some_and(|r| r.trim().is_empty())
        {
            violations.push(M5ReloadContinuityViolation::EmptyField {
                id: card.card_id.clone(),
                field_name: "last_loaded_build_ref",
            });
        }
        for evidence in &card.evidence_refs {
            if evidence.trim().is_empty() {
                violations.push(M5ReloadContinuityViolation::EmptyField {
                    id: card.card_id.clone(),
                    field_name: "evidence_refs",
                });
            }
        }

        // The rendered trust posture must equal the card's recomputed posture, so a
        // reload can never render a stronger badge than the signing state and origin
        // support.
        let effective = card.effective_trust_posture();
        if card.rendered_trust_posture != effective {
            violations.push(M5ReloadContinuityViolation::RenderedTrustOverstated {
                card_id: card.card_id.clone(),
                rendered: card.rendered_trust_posture.as_str(),
                computed: effective.as_str(),
            });
        }

        // Non-inheritance: a local-dev or side-loaded workspace, or an unsigned/revoked
        // artifact, must render local-only and never inherit a trusted badge through a
        // reload — even when signed on a trusted machine.
        if (card.signature_state.is_local_or_untrusted() || card.origin.caps_to_local_only())
            && card.rendered_trust_posture != TrustPosture::UnsignedLocalOnly
        {
            violations.push(M5ReloadContinuityViolation::LocalPackageInheritedTrust {
                card_id: card.card_id.clone(),
                origin: card.origin.as_str(),
                signature_state: card.signature_state.as_str(),
                rendered: card.rendered_trust_posture.as_str(),
            });
        }

        // A hot reload that would widen authority must hold the running instance for a
        // fresh review, so authority can never widen through a hot reload silently.
        if card.requires_fresh_review() && card.load_state != LoadState::ReloadHeldForReview {
            violations.push(M5ReloadContinuityViolation::HotReloadWideningNotHeld {
                card_id: card.card_id.clone(),
                hot_reload_posture: card.hot_reload_posture.as_str(),
                load_state: card.load_state.as_str(),
            });
        }

        // Conversely, an instance held for review must be backed by a widening hot reload.
        if card.load_state == LoadState::ReloadHeldForReview && !card.requires_fresh_review() {
            violations.push(M5ReloadContinuityViolation::ReloadHeldWithoutWidening {
                card_id: card.card_id.clone(),
                hot_reload_posture: card.hot_reload_posture.as_str(),
            });
        }

        // A `loaded_current_build` claim must be backed by a current build and present
        // source, so a "loaded current" claim stays honest.
        if card.load_state == LoadState::LoadedCurrentBuild
            && (card.build_freshness != BuildFreshness::BuiltFromCurrentSource
                || card.source_availability != SourceAvailability::SourcePresent)
        {
            violations.push(
                M5ReloadContinuityViolation::LoadedCurrentBuildInconsistent {
                    card_id: card.card_id.clone(),
                    build_freshness: card.build_freshness.as_str(),
                    source_availability: card.source_availability.as_str(),
                },
            );
        }

        // A running instance serving a last-loaded build must carry its continuity
        // record, so the last-loaded-build record is never lost when source/build breaks.
        if card.computed_continuity_state().requires_retained_record()
            && !card.continuity_record_retained()
        {
            violations.push(M5ReloadContinuityViolation::LastLoadedRecordMissing {
                card_id: card.card_id.clone(),
                continuity_state: card.computed_continuity_state().as_str(),
            });
        }

        // Each stored derived banner field must equal the recomputed value, so a card can
        // never publish a banner that contradicts the observed facts.
        for (field, stored, computed) in [
            (
                "continuity_state",
                card.continuity_state.as_str(),
                card.computed_continuity_state().as_str(),
            ),
            (
                "restart_scope",
                card.restart_scope.as_str(),
                card.computed_restart_scope().as_str(),
            ),
            (
                "preserved_state",
                card.preserved_state.as_str(),
                card.computed_preserved_state().as_str(),
            ),
            (
                "widening_review",
                card.widening_review.as_str(),
                card.computed_widening_review().as_str(),
            ),
            (
                "rollback_path",
                card.rollback_path.as_str(),
                card.computed_rollback_path().as_str(),
            ),
        ] {
            if stored != computed {
                violations.push(M5ReloadContinuityViolation::DerivedFieldMismatch {
                    card_id: card.card_id.clone(),
                    field,
                    stored,
                    computed,
                });
            }
        }
    }
}

/// A validation violation for the M5 reload-continuity board packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ReloadContinuityViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Card or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A card id appears more than once.
    DuplicateCardId {
        /// Duplicate card id.
        card_id: String,
    },
    /// A marketed family carries more than one card.
    DuplicateFamilyCard {
        /// Family token.
        family: &'static str,
    },
    /// A claimed marketed family has no card.
    MissingFamilyCard {
        /// Family token.
        family: &'static str,
    },
    /// A card covers a family the packet does not claim.
    UnclaimedFamilyCard {
        /// Card id.
        card_id: String,
        /// Family token.
        family: &'static str,
    },
    /// A card renders a trust posture beyond what its signing state and origin support.
    RenderedTrustOverstated {
        /// Card id.
        card_id: String,
        /// Rendered trust-posture token.
        rendered: &'static str,
        /// Computed effective trust-posture token.
        computed: &'static str,
    },
    /// A local/side-loaded package or unsigned/revoked artifact rendered a trusted badge.
    LocalPackageInheritedTrust {
        /// Card id.
        card_id: String,
        /// Origin token.
        origin: &'static str,
        /// Signing-state token.
        signature_state: &'static str,
        /// Rendered trust-posture token.
        rendered: &'static str,
    },
    /// A widening hot reload did not hold the running instance for review.
    HotReloadWideningNotHeld {
        /// Card id.
        card_id: String,
        /// Hot-reload-posture token.
        hot_reload_posture: &'static str,
        /// Load-state token.
        load_state: &'static str,
    },
    /// An instance held for review is not backed by a widening hot reload.
    ReloadHeldWithoutWidening {
        /// Card id.
        card_id: String,
        /// Hot-reload-posture token.
        hot_reload_posture: &'static str,
    },
    /// A `loaded_current_build` claim is not backed by a current build and present source.
    LoadedCurrentBuildInconsistent {
        /// Card id.
        card_id: String,
        /// Build-freshness token.
        build_freshness: &'static str,
        /// Source-availability token.
        source_availability: &'static str,
    },
    /// A card serving a last-loaded build lost its continuity record.
    LastLoadedRecordMissing {
        /// Card id.
        card_id: String,
        /// Continuity-state token.
        continuity_state: &'static str,
    },
    /// A stored derived banner field disagrees with the recomputed value.
    DerivedFieldMismatch {
        /// Card id.
        card_id: String,
        /// Field name.
        field: &'static str,
        /// Stored token.
        stored: &'static str,
        /// Computed token.
        computed: &'static str,
    },
    /// A card covers a family the publish-preview gate does not.
    MissingMatrixRow {
        /// Card id.
        card_id: String,
        /// Family token.
        family: &'static str,
    },
    /// A card renders a stronger badge than the publish-preview gate would grant.
    CardExceedsPublishGate {
        /// Card id.
        card_id: String,
        /// Rendered trust-posture token.
        rendered: &'static str,
        /// Published trust-posture token from the gate.
        published: &'static str,
    },
    /// The summary counts disagree with the cards.
    SummaryMismatch,
}

impl fmt::Display for M5ReloadContinuityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateCardId { card_id } => {
                write!(f, "duplicate card id {card_id}")
            }
            Self::DuplicateFamilyCard { family } => {
                write!(f, "duplicate card for family {family}")
            }
            Self::MissingFamilyCard { family } => {
                write!(f, "missing card for claimed family {family}")
            }
            Self::UnclaimedFamilyCard { card_id, family } => {
                write!(f, "card {card_id} covers unclaimed family {family}")
            }
            Self::RenderedTrustOverstated {
                card_id,
                rendered,
                computed,
            } => {
                write!(
                    f,
                    "card {card_id} renders trust posture {rendered} but the card computes {computed}"
                )
            }
            Self::LocalPackageInheritedTrust {
                card_id,
                origin,
                signature_state,
                rendered,
            } => {
                write!(
                    f,
                    "card {card_id} is {origin}/{signature_state} but renders {rendered}; local packages must render unsigned_local_only"
                )
            }
            Self::HotReloadWideningNotHeld {
                card_id,
                hot_reload_posture,
                load_state,
            } => {
                write!(
                    f,
                    "card {card_id} hot reload {hot_reload_posture} would widen authority but the instance is {load_state} rather than held for review"
                )
            }
            Self::ReloadHeldWithoutWidening {
                card_id,
                hot_reload_posture,
            } => {
                write!(
                    f,
                    "card {card_id} holds the instance for review but its hot reload {hot_reload_posture} does not widen authority"
                )
            }
            Self::LoadedCurrentBuildInconsistent {
                card_id,
                build_freshness,
                source_availability,
            } => {
                write!(
                    f,
                    "card {card_id} reports loaded_current_build but its build is {build_freshness} and source is {source_availability}"
                )
            }
            Self::LastLoadedRecordMissing {
                card_id,
                continuity_state,
            } => {
                write!(
                    f,
                    "card {card_id} is {continuity_state} but carries no last_loaded_build_ref; the continuity record must not be lost"
                )
            }
            Self::DerivedFieldMismatch {
                card_id,
                field,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "card {card_id} stores {field} {stored} but the card computes {computed}"
                )
            }
            Self::MissingMatrixRow { card_id, family } => {
                write!(
                    f,
                    "card {card_id} covers family {family} but the publish-preview gate has no row for it"
                )
            }
            Self::CardExceedsPublishGate {
                card_id,
                rendered,
                published,
            } => {
                write!(
                    f,
                    "card {card_id} renders {rendered} but the publish-preview gate grants only {published}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the cards")
            }
        }
    }
}

impl Error for M5ReloadContinuityViolation {}

/// Loads the embedded M5 reload-continuity board packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5ReloadContinuityBoard`].
pub fn current_m5_reload_continuity_board() -> Result<M5ReloadContinuityBoard, serde_json::Error> {
    serde_json::from_str(M5_RELOAD_CONTINUITY_JSON)
}

#[cfg(test)]
mod tests;
