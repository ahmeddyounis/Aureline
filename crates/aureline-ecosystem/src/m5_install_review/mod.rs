//! Canonical M5 install/update review sheets — one reviewed change model that
//! compares a package's current effective state against the proposed one.
//!
//! Where the
//! [`install-governance matrix`](crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix)
//! freezes one governance row per marketed M5 artifact family and the
//! [`marketplace fact-views`](crate::m5_marketplace_fact_views) project that truth
//! into the storefront, this module freezes how an install or update is *reviewed
//! before commit*. It reuses the governance vocabulary — [`ArtifactFamily`],
//! [`RuntimeOrigin`], [`SupportClass`], [`CompatibilityLabel`], and
//! [`RollbackPosture`] — and the marketplace [`SourceClass`], and adds the
//! transition vocabulary a reviewed change needs: [`ReviewChangeKind`],
//! [`InstallScope`], [`HostClass`], capability deltas, publisher-continuity states,
//! restart/open-work implications, and the [`CommitDisposition`] gate.
//!
//! An [`InstallReviewSheet`] turns install and update from a generic download
//! action into one coherent surface. It compares the current
//! [`PackageRevision`] (absent for a fresh install) with the proposed one and
//! makes every change explicit on one sheet:
//!
//! - **permission deltas** — a list of [`CapabilityDelta`] marking each capability
//!   as required-versus-optional, direct-versus-transitive, and added, removed,
//!   widened, narrowed, or unchanged, so transitive capability widening is named
//!   rather than buried;
//! - **runtime-origin and host-class changes** — moving a package from a signed
//!   build to a bridge or local-model runtime, or from a local host to a managed or
//!   container host, is surfaced rather than silent;
//! - **publisher continuity** — publisher transfer, signing-root continuity, and
//!   namespace/orphan state make a changed publisher visible before commit;
//! - **compatibility-floor changes** — a regression toward an unsupported target is
//!   computed from the current and proposed [`CompatibilityLabel`];
//! - **restart/reattach and open-work implications** — a change that restarts a
//!   host or interrupts an active session is disclosed; and
//! - **rollback** — a [`RollbackPlan`] carries a checkpoint handle, the rollback
//!   posture, and a current-package fallback route so recovery is intentional.
//!
//! The sheet is honest by construction. Its [`ReviewTrigger`] set and the
//! [`CommitDisposition`] it publishes are **not** stored by hand: they are
//! recomputed from the sheet's facts, and the stored values must equal that
//! recomputation or validation fails. This enforces the lane guardrail —
//! one-click install/update is never offered on newly widened permissions, a
//! changed publisher, or a changed runtime origin: any of those forces at least
//! [`CommitDisposition::UnifiedReviewRequired`], and a regression to an unsupported
//! target [`CommitDisposition::Blocked`]s the commit outright.
//!
//! Workspace-, profile-, and global-scope actions stay visibly distinct: every
//! [`ReviewAction`] is scoped to the sheet's [`InstallScope`], so a local
//! troubleshooting moment cannot silently widen or narrow the wrong scope.
//!
//! The packet is checked in at `artifacts/ecosystem/m5/m5-install-review.json` and
//! embedded here, so this typed consumer and any CI gate agree on every sheet
//! without a cargo build in CI. The model is metadata-only: every field is a typed
//! state or an opaque ref. It carries no credential bodies, raw provider payloads,
//! signing secrets, or registry tokens.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::{
    ArtifactFamily, CompatibilityLabel, RollbackPosture, RuntimeOrigin, SupportClass,
};
use crate::m5_marketplace_fact_views::SourceClass;

/// Supported M5 install-review schema version.
pub const M5_INSTALL_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_INSTALL_REVIEW_RECORD_KIND: &str = "m5_install_review";

/// Repo-relative path to the checked-in packet.
pub const M5_INSTALL_REVIEW_PATH: &str = "artifacts/ecosystem/m5/m5-install-review.json";

/// Embedded checked-in packet JSON.
pub const M5_INSTALL_REVIEW_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-install-review.json"
));

/// The kind of change a review sheet covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewChangeKind {
    /// A fresh install with no current revision.
    Install,
    /// An update from the current revision to a newer proposed one.
    Update,
    /// A reinstall of the same revision.
    Reinstall,
    /// A downgrade from the current revision to an older proposed one.
    Downgrade,
}

impl ReviewChangeKind {
    /// Every change kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Install,
        Self::Update,
        Self::Reinstall,
        Self::Downgrade,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Install => "install",
            Self::Update => "update",
            Self::Reinstall => "reinstall",
            Self::Downgrade => "downgrade",
        }
    }

    /// Whether this change kind expects a current revision to compare against.
    pub const fn expects_current(self) -> bool {
        !matches!(self, Self::Install)
    }
}

/// The scope an install/update action applies to.
///
/// Scopes stay visibly distinct so a local troubleshooting moment in one scope can
/// never silently widen or narrow another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallScope {
    /// Scoped to the current workspace.
    Workspace,
    /// Scoped to the active profile.
    Profile,
    /// Applies globally across the install.
    Global,
}

impl InstallScope {
    /// Every scope, in declaration order.
    pub const ALL: [Self; 3] = [Self::Workspace, Self::Profile, Self::Global];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Profile => "profile",
            Self::Global => "global",
        }
    }
}

/// The host class a package's runtime is bound to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostClass {
    /// Runs on the local host.
    Local,
    /// Runs on a managed workspace host.
    ManagedWorkspace,
    /// Runs on a remote host.
    RemoteHost,
    /// Runs inside a container host.
    Container,
}

impl HostClass {
    /// Every host class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Local,
        Self::ManagedWorkspace,
        Self::RemoteHost,
        Self::Container,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::ManagedWorkspace => "managed_workspace",
            Self::RemoteHost => "remote_host",
            Self::Container => "container",
        }
    }
}

/// Whether a capability is required for the package to function or optional.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityRequirement {
    /// The capability is required.
    Required,
    /// The capability is optional.
    Optional,
}

impl CapabilityRequirement {
    /// Every requirement, in declaration order.
    pub const ALL: [Self; 2] = [Self::Required, Self::Optional];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
        }
    }
}

/// Whether a capability comes directly from the package or transitively from a
/// dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityOrigin {
    /// Declared directly by the package.
    Direct,
    /// Introduced by a transitive dependency.
    Transitive,
}

impl CapabilityOrigin {
    /// Every origin, in declaration order.
    pub const ALL: [Self; 2] = [Self::Direct, Self::Transitive];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Transitive => "transitive",
        }
    }
}

/// How a single capability changes between the current and proposed revisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityChange {
    /// A capability not previously granted is newly requested.
    Added,
    /// A previously granted capability is dropped.
    Removed,
    /// A capability is unchanged.
    Unchanged,
    /// A capability's grant is broadened.
    Widened,
    /// A capability's grant is narrowed.
    Narrowed,
}

impl CapabilityChange {
    /// Every change, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Added,
        Self::Removed,
        Self::Unchanged,
        Self::Widened,
        Self::Narrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Unchanged => "unchanged",
            Self::Widened => "widened",
            Self::Narrowed => "narrowed",
        }
    }

    /// Whether this change widens the package's permissions.
    pub const fn is_widening(self) -> bool {
        matches!(self, Self::Added | Self::Widened)
    }
}

/// Restart or reattach implication of committing a change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartImpact {
    /// No restart or reattach is required.
    NoRestart,
    /// A live host or session must be reattached.
    ReattachRequired,
    /// The runtime host must restart.
    HostRestartRequired,
    /// The application must restart.
    AppRestartRequired,
}

impl RestartImpact {
    /// Every restart impact, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoRestart,
        Self::ReattachRequired,
        Self::HostRestartRequired,
        Self::AppRestartRequired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRestart => "no_restart",
            Self::ReattachRequired => "reattach_required",
            Self::HostRestartRequired => "host_restart_required",
            Self::AppRestartRequired => "app_restart_required",
        }
    }

    /// Whether committing requires a restart or reattach.
    pub const fn requires_restart_or_reattach(self) -> bool {
        !matches!(self, Self::NoRestart)
    }
}

/// Impact of committing a change on currently open work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenWorkImpact {
    /// No impact on open work.
    NoImpact,
    /// A background reindex runs but open work is untouched.
    BackgroundReindex,
    /// Open editors are affected.
    OpenEditorsAffected,
    /// An active session is interrupted.
    ActiveSessionInterrupted,
}

impl OpenWorkImpact {
    /// Every open-work impact, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoImpact,
        Self::BackgroundReindex,
        Self::OpenEditorsAffected,
        Self::ActiveSessionInterrupted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoImpact => "no_impact",
            Self::BackgroundReindex => "background_reindex",
            Self::OpenEditorsAffected => "open_editors_affected",
            Self::ActiveSessionInterrupted => "active_session_interrupted",
        }
    }

    /// Whether this impact disrupts currently open work.
    ///
    /// A background reindex is disclosed but does not disrupt open work, so it is
    /// not a review trigger on its own.
    pub const fn impacts_open_work(self) -> bool {
        matches!(
            self,
            Self::OpenEditorsAffected | Self::ActiveSessionInterrupted
        )
    }
}

/// State of a package's publisher relative to the current revision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublisherTransferState {
    /// The publisher is unchanged.
    SamePublisher,
    /// Ownership transferred to a verified publisher.
    TransferredVerified,
    /// Ownership transferred to an unverified publisher.
    TransferredUnverified,
    /// The publisher cannot be established.
    PublisherUnknown,
    /// No prior publisher to compare (fresh install).
    NotApplicable,
}

impl PublisherTransferState {
    /// Every transfer state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SamePublisher,
        Self::TransferredVerified,
        Self::TransferredUnverified,
        Self::PublisherUnknown,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SamePublisher => "same_publisher",
            Self::TransferredVerified => "transferred_verified",
            Self::TransferredUnverified => "transferred_unverified",
            Self::PublisherUnknown => "publisher_unknown",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this state is a publisher discontinuity.
    pub const fn is_discontinuity(self) -> bool {
        matches!(
            self,
            Self::TransferredVerified | Self::TransferredUnverified | Self::PublisherUnknown
        )
    }
}

/// Continuity of a package's signing root relative to the current revision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SigningRootContinuity {
    /// The signing root is continuous.
    Continuous,
    /// The signing root rotated under a disclosed, continuous chain.
    RotatedDisclosed,
    /// The signing root changed to an unrelated root.
    RootChanged,
    /// The proposed revision is unsigned.
    Unsigned,
    /// No signing root applies.
    NotApplicable,
}

impl SigningRootContinuity {
    /// Every signing-root continuity, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Continuous,
        Self::RotatedDisclosed,
        Self::RootChanged,
        Self::Unsigned,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Continuous => "continuous",
            Self::RotatedDisclosed => "rotated_disclosed",
            Self::RootChanged => "root_changed",
            Self::Unsigned => "unsigned",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this continuity is a discontinuity in the signing chain.
    ///
    /// A disclosed rotation preserves the chain and is not a discontinuity; an
    /// unrelated root change is.
    pub const fn is_discontinuity(self) -> bool {
        matches!(self, Self::RootChanged)
    }
}

/// State of a package's namespace relative to the current revision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamespaceState {
    /// The namespace is stable.
    Stable,
    /// The namespace was renamed under a disclosed redirect.
    Renamed,
    /// The namespace is orphaned (no active maintainer).
    Orphaned,
    /// The namespace was reclaimed by a different party.
    Reclaimed,
    /// No namespace comparison applies.
    NotApplicable,
}

impl NamespaceState {
    /// Every namespace state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Stable,
        Self::Renamed,
        Self::Orphaned,
        Self::Reclaimed,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Renamed => "renamed",
            Self::Orphaned => "orphaned",
            Self::Reclaimed => "reclaimed",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this state is a namespace discontinuity.
    ///
    /// A disclosed rename preserves identity; an orphaned or reclaimed namespace
    /// does not.
    pub const fn is_discontinuity(self) -> bool {
        matches!(self, Self::Orphaned | Self::Reclaimed)
    }
}

/// How the compatibility floor moves between the current and proposed revisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityFloorChange {
    /// The compatibility floor is unchanged.
    Unchanged,
    /// The compatibility floor relaxes (proposed is more compatible).
    Relaxed,
    /// The compatibility floor regresses but stays supported.
    Regressed,
    /// The compatibility floor regresses to an unsupported target.
    RegressedToUnsupported,
    /// No current revision to compare (fresh install).
    Initial,
}

impl CompatibilityFloorChange {
    /// Every floor change, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Unchanged,
        Self::Relaxed,
        Self::Regressed,
        Self::RegressedToUnsupported,
        Self::Initial,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::Relaxed => "relaxed",
            Self::Regressed => "regressed",
            Self::RegressedToUnsupported => "regressed_to_unsupported",
            Self::Initial => "initial",
        }
    }
}

/// A reason a sheet escalates above one-click commit.
///
/// Each trigger is recomputed from the sheet's facts; the sheet's stored
/// [`InstallReviewSheet::review_triggers`] must equal the recomputed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewTrigger {
    /// The proposed revision widens the package's permissions.
    PermissionsWidened,
    /// The publisher, signing root, or namespace is discontinuous.
    PublisherDiscontinuity,
    /// The runtime origin changes.
    RuntimeOriginChanged,
    /// The host class changes.
    HostClassChanged,
    /// The compatibility floor regresses but stays supported.
    CompatibilityFloorRegressed,
    /// The proposed revision is unsupported on the target.
    CompatibilityUnsupported,
    /// Committing requires a restart or reattach.
    RestartOrReattachRequired,
    /// Committing impacts currently open work.
    OpenWorkImpacted,
    /// No verified rollback path is established for a state-changing commit.
    RollbackNotEstablished,
}

impl ReviewTrigger {
    /// Every review trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::PermissionsWidened,
        Self::PublisherDiscontinuity,
        Self::RuntimeOriginChanged,
        Self::HostClassChanged,
        Self::CompatibilityFloorRegressed,
        Self::CompatibilityUnsupported,
        Self::RestartOrReattachRequired,
        Self::OpenWorkImpacted,
        Self::RollbackNotEstablished,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PermissionsWidened => "permissions_widened",
            Self::PublisherDiscontinuity => "publisher_discontinuity",
            Self::RuntimeOriginChanged => "runtime_origin_changed",
            Self::HostClassChanged => "host_class_changed",
            Self::CompatibilityFloorRegressed => "compatibility_floor_regressed",
            Self::CompatibilityUnsupported => "compatibility_unsupported",
            Self::RestartOrReattachRequired => "restart_or_reattach_required",
            Self::OpenWorkImpacted => "open_work_impacted",
            Self::RollbackNotEstablished => "rollback_not_established",
        }
    }

    /// The minimum commit disposition this trigger forces.
    pub const fn min_disposition(self) -> CommitDisposition {
        match self {
            // An unsupported target is a hard stop: a commit must not proceed.
            Self::CompatibilityUnsupported => CommitDisposition::Blocked,
            // The headline guardrail triggers — widened permissions, a changed
            // publisher, a changed runtime origin — plus the remaining transition
            // risks force the unified review sheet but do not block on their own.
            Self::PermissionsWidened
            | Self::PublisherDiscontinuity
            | Self::RuntimeOriginChanged
            | Self::HostClassChanged
            | Self::CompatibilityFloorRegressed
            | Self::RestartOrReattachRequired
            | Self::OpenWorkImpacted
            | Self::RollbackNotEstablished => CommitDisposition::UnifiedReviewRequired,
        }
    }
}

/// The commit disposition a sheet publishes.
///
/// Ordered low-to-high by [`CommitDisposition::rank`]: a
/// [`CommitDisposition::OneClickAllowed`] sheet may be committed directly, and a
/// [`CommitDisposition::Blocked`] sheet must not be committed at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitDisposition {
    /// A direct one-click commit is allowed; no trigger applies.
    OneClickAllowed,
    /// The unified review sheet must be acknowledged before commit.
    UnifiedReviewRequired,
    /// The commit is blocked.
    Blocked,
}

impl CommitDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::OneClickAllowed,
        Self::UnifiedReviewRequired,
        Self::Blocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OneClickAllowed => "one_click_allowed",
            Self::UnifiedReviewRequired => "unified_review_required",
            Self::Blocked => "blocked",
        }
    }

    /// Monotonic rank; higher means a stricter gate.
    pub const fn rank(self) -> u8 {
        match self {
            Self::OneClickAllowed => 0,
            Self::UnifiedReviewRequired => 1,
            Self::Blocked => 2,
        }
    }

    /// The stricter (higher-rank) of two dispositions.
    pub const fn widen(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }
}

/// A reviewed action offered through the sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewActionKind {
    /// Commit the proposed change.
    Commit,
    /// Create a rollback checkpoint before committing.
    CreateRollbackCheckpoint,
    /// Fall back to the current package revision.
    FallBackToCurrent,
    /// Disable the package in this scope.
    DisableInScope,
    /// Retry a failed install/update.
    Retry,
    /// Cancel the review without committing.
    Cancel,
}

impl ReviewActionKind {
    /// Every action kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Commit,
        Self::CreateRollbackCheckpoint,
        Self::FallBackToCurrent,
        Self::DisableInScope,
        Self::Retry,
        Self::Cancel,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Commit => "commit",
            Self::CreateRollbackCheckpoint => "create_rollback_checkpoint",
            Self::FallBackToCurrent => "fall_back_to_current",
            Self::DisableInScope => "disable_in_scope",
            Self::Retry => "retry",
            Self::Cancel => "cancel",
        }
    }
}

/// A snapshot of one package revision (current or proposed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageRevision {
    /// Opaque ref to the revision.
    pub revision_ref: String,
    /// Human-readable version label.
    pub version_label: String,
    /// Runtime origin of the revision.
    pub runtime_origin: RuntimeOrigin,
    /// Host class the revision binds to.
    pub host_class: HostClass,
    /// Publisher-trust source class.
    pub source_class: SourceClass,
    /// Opaque ref to the publisher identity.
    pub publisher_ref: String,
    /// Opaque ref to the signing root.
    pub signing_root_ref: String,
    /// Opaque ref to the namespace.
    pub namespace_ref: String,
    /// Compatibility label against the install target.
    pub compatibility_label: CompatibilityLabel,
    /// Published support class.
    pub support_class: SupportClass,
}

/// A single capability delta between the current and proposed revisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityDelta {
    /// Opaque ref to the capability.
    pub capability_ref: String,
    /// Human-readable capability label.
    pub display_label: String,
    /// Whether the capability is required or optional.
    pub requirement: CapabilityRequirement,
    /// Whether the capability is direct or transitive.
    pub origin: CapabilityOrigin,
    /// How the capability changes.
    pub change: CapabilityChange,
}

impl CapabilityDelta {
    /// Whether this delta widens the package's permissions.
    pub const fn is_widening(&self) -> bool {
        self.change.is_widening()
    }

    /// Whether this delta is a transitive widening.
    pub const fn is_transitive_widening(&self) -> bool {
        self.is_widening() && matches!(self.origin, CapabilityOrigin::Transitive)
    }
}

/// The rollback plan attached to a review sheet.
///
/// The plan keeps recovery intentional: it carries a durable checkpoint handle,
/// the rollback posture, and a current-package fallback route, so a user or admin
/// install can recover deliberately rather than by ad hoc reinstall.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RollbackPlan {
    /// Opaque ref to the durable rollback checkpoint handle.
    pub checkpoint_handle_ref: String,
    /// Rollback posture of the proposed change.
    pub rollback_posture: RollbackPosture,
    /// Opaque ref to the current-package fallback route.
    pub fallback_package_ref: String,
    /// Whether a rollback checkpoint has been created.
    pub checkpoint_created: bool,
}

impl RollbackPlan {
    /// Whether the rollback path is unestablished for a state-changing commit.
    ///
    /// A not-applicable posture (for example, a stateless docs pack) needs no
    /// checkpoint. Otherwise an unverified or irreversible posture, or a missing
    /// checkpoint, leaves recovery unestablished.
    pub const fn is_unestablished(&self) -> bool {
        if matches!(self.rollback_posture, RollbackPosture::NotApplicable) {
            return false;
        }
        self.rollback_posture.is_incomplete_trigger() || !self.checkpoint_created
    }
}

/// A scoped action offered through the review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewAction {
    /// The kind of action.
    pub action_kind: ReviewActionKind,
    /// The scope the action applies to; must equal the sheet's scope.
    pub scope: InstallScope,
    /// Opaque ref to the action.
    pub action_ref: String,
    /// Whether the action is currently enabled.
    pub enabled: bool,
}

/// One install/update review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstallReviewSheet {
    /// Stable sheet id.
    pub sheet_id: String,
    /// Opaque ref to the catalog listing under review.
    pub listing_ref: String,
    /// Human-readable listing label.
    pub display_label: String,
    /// Ref to the governance-matrix family this listing resolves through.
    pub governance_family_ref: String,
    /// Package kind / marketed artifact family.
    pub package_kind: ArtifactFamily,
    /// The change being reviewed.
    pub change_kind: ReviewChangeKind,
    /// The scope the change applies to.
    pub scope: InstallScope,
    /// The current effective revision, absent for a fresh install.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<PackageRevision>,
    /// The proposed revision.
    pub proposed: PackageRevision,
    /// Permission/capability deltas.
    #[serde(default)]
    pub capability_deltas: Vec<CapabilityDelta>,
    /// Publisher-transfer state.
    pub publisher_transfer: PublisherTransferState,
    /// Signing-root continuity.
    pub signing_root_continuity: SigningRootContinuity,
    /// Namespace state.
    pub namespace_state: NamespaceState,
    /// Restart/reattach implication.
    pub restart_impact: RestartImpact,
    /// Open-work impact.
    pub open_work_impact: OpenWorkImpact,
    /// Compatibility-floor change; must equal the recomputed value.
    pub compatibility_floor_change: CompatibilityFloorChange,
    /// Rollback plan.
    pub rollback: RollbackPlan,
    /// Scoped actions offered through the sheet.
    #[serde(default)]
    pub actions: Vec<ReviewAction>,
    /// Review triggers; must equal the recomputed set.
    #[serde(default)]
    pub review_triggers: Vec<ReviewTrigger>,
    /// Commit disposition; must equal the recomputed value.
    pub commit_disposition: CommitDisposition,
    /// Reviewer-facing summary.
    pub summary: String,
}

impl InstallReviewSheet {
    /// Whether the proposed revision widens the package's permissions.
    pub fn widens_permissions(&self) -> bool {
        self.capability_deltas
            .iter()
            .any(CapabilityDelta::is_widening)
    }

    /// Whether the proposed revision introduces a transitive capability widening.
    pub fn widens_transitive_permissions(&self) -> bool {
        self.capability_deltas
            .iter()
            .any(CapabilityDelta::is_transitive_widening)
    }

    /// Whether the runtime origin changes between current and proposed.
    pub fn runtime_origin_changed(&self) -> bool {
        matches!(&self.current, Some(c) if c.runtime_origin != self.proposed.runtime_origin)
    }

    /// Whether the host class changes between current and proposed.
    pub fn host_class_changed(&self) -> bool {
        matches!(&self.current, Some(c) if c.host_class != self.proposed.host_class)
    }

    /// Whether the publisher, signing root, or namespace is discontinuous.
    pub fn is_publisher_discontinuity(&self) -> bool {
        self.publisher_transfer.is_discontinuity()
            || self.signing_root_continuity.is_discontinuity()
            || self.namespace_state.is_discontinuity()
    }

    /// The compatibility-floor change recomputed from the current and proposed
    /// compatibility labels.
    pub fn computed_compatibility_floor_change(&self) -> CompatibilityFloorChange {
        let Some(current) = &self.current else {
            return CompatibilityFloorChange::Initial;
        };
        let proposed = self.proposed.compatibility_label;
        if proposed == CompatibilityLabel::UnsupportedOnTarget
            && current.compatibility_label != CompatibilityLabel::UnsupportedOnTarget
        {
            return CompatibilityFloorChange::RegressedToUnsupported;
        }
        let current_rank = current.compatibility_label.support_ceiling().rank();
        let proposed_rank = proposed.support_ceiling().rank();
        match proposed_rank.cmp(&current_rank) {
            std::cmp::Ordering::Less => CompatibilityFloorChange::Regressed,
            std::cmp::Ordering::Greater => CompatibilityFloorChange::Relaxed,
            std::cmp::Ordering::Equal => CompatibilityFloorChange::Unchanged,
        }
    }

    /// The review triggers recomputed from this sheet's facts, in canonical order.
    pub fn computed_review_triggers(&self) -> Vec<ReviewTrigger> {
        let mut triggers = Vec::new();
        if self.widens_permissions() {
            triggers.push(ReviewTrigger::PermissionsWidened);
        }
        if self.is_publisher_discontinuity() {
            triggers.push(ReviewTrigger::PublisherDiscontinuity);
        }
        if self.runtime_origin_changed() {
            triggers.push(ReviewTrigger::RuntimeOriginChanged);
        }
        if self.host_class_changed() {
            triggers.push(ReviewTrigger::HostClassChanged);
        }
        match self.computed_compatibility_floor_change() {
            CompatibilityFloorChange::Regressed => {
                triggers.push(ReviewTrigger::CompatibilityFloorRegressed);
            }
            CompatibilityFloorChange::RegressedToUnsupported => {
                triggers.push(ReviewTrigger::CompatibilityUnsupported);
            }
            _ => {}
        }
        if self.restart_impact.requires_restart_or_reattach() {
            triggers.push(ReviewTrigger::RestartOrReattachRequired);
        }
        if self.open_work_impact.impacts_open_work() {
            triggers.push(ReviewTrigger::OpenWorkImpacted);
        }
        if self.rollback.is_unestablished() {
            triggers.push(ReviewTrigger::RollbackNotEstablished);
        }
        triggers
    }

    /// The commit disposition recomputed from this sheet's triggers.
    pub fn computed_commit_disposition(&self) -> CommitDisposition {
        self.computed_review_triggers().into_iter().fold(
            CommitDisposition::OneClickAllowed,
            |disposition, trigger| disposition.widen(trigger.min_disposition()),
        )
    }

    /// Whether the stored triggers, floor change, and disposition agree with the
    /// recomputed values.
    pub fn gate_consistent(&self) -> bool {
        self.compatibility_floor_change == self.computed_compatibility_floor_change()
            && self.review_triggers == self.computed_review_triggers()
            && self.commit_disposition == self.computed_commit_disposition()
    }

    /// Whether a direct one-click commit is allowed.
    pub fn allows_one_click(&self) -> bool {
        self.commit_disposition == CommitDisposition::OneClickAllowed
    }

    /// The enabled commit action, if any.
    pub fn commit_action(&self) -> Option<&ReviewAction> {
        self.actions
            .iter()
            .find(|a| a.action_kind == ReviewActionKind::Commit)
    }

    /// Whether the sheet offers an action of the given kind.
    pub fn offers_action(&self, kind: ReviewActionKind) -> bool {
        self.actions.iter().any(|a| a.action_kind == kind)
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5InstallReviewSummary {
    /// Total review sheets.
    pub total_sheets: usize,
    /// Sheets that allow a one-click commit.
    pub one_click_allowed_sheets: usize,
    /// Sheets that require the unified review sheet.
    pub unified_review_required_sheets: usize,
    /// Sheets whose commit is blocked.
    pub blocked_sheets: usize,
    /// Sheets that widen permissions.
    pub permission_widening_sheets: usize,
    /// Sheets that widen permissions transitively.
    pub transitive_widening_sheets: usize,
    /// Sheets with a publisher discontinuity.
    pub publisher_discontinuity_sheets: usize,
    /// Sheets that change runtime origin.
    pub runtime_origin_change_sheets: usize,
    /// Sheets that change host class.
    pub host_class_change_sheets: usize,
    /// Sheets whose compatibility floor regresses (supported or unsupported).
    pub compatibility_regression_sheets: usize,
    /// Sheets that require a restart or reattach.
    pub restart_or_reattach_sheets: usize,
    /// Sheets that impact open work.
    pub open_work_impact_sheets: usize,
    /// Sheets with an unestablished rollback path.
    pub rollback_unestablished_sheets: usize,
    /// Sheets scoped to a workspace.
    pub workspace_scope_sheets: usize,
    /// Sheets scoped to a profile.
    pub profile_scope_sheets: usize,
    /// Sheets scoped globally.
    pub global_scope_sheets: usize,
    /// Distinct package kinds across sheets.
    pub distinct_package_kinds: usize,
}

/// A redaction-safe export row projected from a review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallReviewExportRow {
    /// Sheet id.
    pub sheet_id: String,
    /// Package-kind token.
    pub package_kind: String,
    /// Change-kind token.
    pub change_kind: String,
    /// Scope token.
    pub scope: String,
    /// Commit-disposition token.
    pub commit_disposition: String,
    /// Review-trigger tokens.
    pub review_triggers: Vec<String>,
    /// Compatibility-floor-change token.
    pub compatibility_floor_change: String,
    /// Governance-matrix family ref.
    pub governance_family_ref: String,
    /// Whether the change widens permissions.
    pub widens_permissions: bool,
    /// Whether a direct one-click commit is allowed.
    pub one_click_allowed: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallReviewExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5InstallReviewExportRow>,
    /// Whether every sheet's gate is consistent with its recomputation.
    pub all_gates_consistent: bool,
    /// Sheets that require the unified review sheet.
    pub unified_review_required_count: usize,
    /// Sheets whose commit is blocked.
    pub blocked_count: usize,
}

/// The typed M5 install-review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5InstallReview {
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
    /// Closed package-kind vocabulary (reused from the governance matrix).
    pub package_kinds: Vec<ArtifactFamily>,
    /// Closed source-class vocabulary (reused from the marketplace fact-views).
    pub source_classes: Vec<SourceClass>,
    /// Closed runtime-origin vocabulary (reused from the governance matrix).
    pub runtime_origins: Vec<RuntimeOrigin>,
    /// Closed support-class vocabulary (reused from the governance matrix).
    pub support_classes: Vec<SupportClass>,
    /// Closed compatibility-label vocabulary (reused from the governance matrix).
    pub compatibility_labels: Vec<CompatibilityLabel>,
    /// Closed rollback-posture vocabulary (reused from the governance matrix).
    pub rollback_postures: Vec<RollbackPosture>,
    /// Closed change-kind vocabulary.
    pub change_kinds: Vec<ReviewChangeKind>,
    /// Closed scope vocabulary.
    pub scopes: Vec<InstallScope>,
    /// Closed host-class vocabulary.
    pub host_classes: Vec<HostClass>,
    /// Closed capability-requirement vocabulary.
    pub capability_requirements: Vec<CapabilityRequirement>,
    /// Closed capability-origin vocabulary.
    pub capability_origins: Vec<CapabilityOrigin>,
    /// Closed capability-change vocabulary.
    pub capability_changes: Vec<CapabilityChange>,
    /// Closed publisher-transfer vocabulary.
    pub publisher_transfer_states: Vec<PublisherTransferState>,
    /// Closed signing-root-continuity vocabulary.
    pub signing_root_continuities: Vec<SigningRootContinuity>,
    /// Closed namespace-state vocabulary.
    pub namespace_states: Vec<NamespaceState>,
    /// Closed restart-impact vocabulary.
    pub restart_impacts: Vec<RestartImpact>,
    /// Closed open-work-impact vocabulary.
    pub open_work_impacts: Vec<OpenWorkImpact>,
    /// Closed compatibility-floor-change vocabulary.
    pub compatibility_floor_changes: Vec<CompatibilityFloorChange>,
    /// Closed review-trigger vocabulary.
    pub review_triggers: Vec<ReviewTrigger>,
    /// Closed commit-disposition vocabulary.
    pub commit_dispositions: Vec<CommitDisposition>,
    /// Closed review-action-kind vocabulary.
    pub review_action_kinds: Vec<ReviewActionKind>,
    /// The install/update review sheets.
    #[serde(default)]
    pub review_sheets: Vec<InstallReviewSheet>,
    /// Summary counts.
    pub summary: M5InstallReviewSummary,
}

impl M5InstallReview {
    /// Returns the review sheet with the given id.
    pub fn review_sheet(&self, sheet_id: &str) -> Option<&InstallReviewSheet> {
        self.review_sheets.iter().find(|s| s.sheet_id == sheet_id)
    }

    /// Review sheets scoped to the given scope.
    pub fn sheets_in_scope(
        &self,
        scope: InstallScope,
    ) -> impl Iterator<Item = &InstallReviewSheet> {
        self.review_sheets.iter().filter(move |s| s.scope == scope)
    }

    /// Review sheets that require the unified review sheet or are blocked.
    pub fn sheets_requiring_review(&self) -> impl Iterator<Item = &InstallReviewSheet> {
        self.review_sheets.iter().filter(|s| !s.allows_one_click())
    }

    /// Whether every sheet's stored gate agrees with its recomputation.
    pub fn all_gates_consistent(&self) -> bool {
        self.review_sheets.iter().all(|s| s.gate_consistent())
    }

    /// Recomputes the summary block from the review sheets.
    pub fn computed_summary(&self) -> M5InstallReviewSummary {
        let count_disposition = |disposition: CommitDisposition| {
            self.review_sheets
                .iter()
                .filter(|s| s.commit_disposition == disposition)
                .count()
        };
        let package_kinds: BTreeSet<ArtifactFamily> =
            self.review_sheets.iter().map(|s| s.package_kind).collect();
        M5InstallReviewSummary {
            total_sheets: self.review_sheets.len(),
            one_click_allowed_sheets: count_disposition(CommitDisposition::OneClickAllowed),
            unified_review_required_sheets: count_disposition(
                CommitDisposition::UnifiedReviewRequired,
            ),
            blocked_sheets: count_disposition(CommitDisposition::Blocked),
            permission_widening_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.widens_permissions())
                .count(),
            transitive_widening_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.widens_transitive_permissions())
                .count(),
            publisher_discontinuity_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.is_publisher_discontinuity())
                .count(),
            runtime_origin_change_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.runtime_origin_changed())
                .count(),
            host_class_change_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.host_class_changed())
                .count(),
            compatibility_regression_sheets: self
                .review_sheets
                .iter()
                .filter(|s| {
                    matches!(
                        s.computed_compatibility_floor_change(),
                        CompatibilityFloorChange::Regressed
                            | CompatibilityFloorChange::RegressedToUnsupported
                    )
                })
                .count(),
            restart_or_reattach_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.restart_impact.requires_restart_or_reattach())
                .count(),
            open_work_impact_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.open_work_impact.impacts_open_work())
                .count(),
            rollback_unestablished_sheets: self
                .review_sheets
                .iter()
                .filter(|s| s.rollback.is_unestablished())
                .count(),
            workspace_scope_sheets: self.sheets_in_scope(InstallScope::Workspace).count(),
            profile_scope_sheets: self.sheets_in_scope(InstallScope::Profile).count(),
            global_scope_sheets: self.sheets_in_scope(InstallScope::Global).count(),
            distinct_package_kinds: package_kinds.len(),
        }
    }

    /// Produces an export projection that downstream surfaces — support exports,
    /// docs/help, and release/public-truth packets — render instead of restating
    /// install/update change, scope, and commit-disposition text by hand.
    pub fn export_projection(&self) -> M5InstallReviewExportProjection {
        let rows = self
            .review_sheets
            .iter()
            .map(|s| M5InstallReviewExportRow {
                sheet_id: s.sheet_id.clone(),
                package_kind: s.package_kind.as_str().to_owned(),
                change_kind: s.change_kind.as_str().to_owned(),
                scope: s.scope.as_str().to_owned(),
                commit_disposition: s.commit_disposition.as_str().to_owned(),
                review_triggers: s
                    .review_triggers
                    .iter()
                    .map(|trigger| trigger.as_str().to_owned())
                    .collect(),
                compatibility_floor_change: s.compatibility_floor_change.as_str().to_owned(),
                governance_family_ref: s.governance_family_ref.clone(),
                widens_permissions: s.widens_permissions(),
                one_click_allowed: s.allows_one_click(),
                summary: format!(
                    "{}: {} in {} scope, disposition {}, floor {}, proposed runtime {}, host {}",
                    s.package_kind.as_str(),
                    s.change_kind.as_str(),
                    s.scope.as_str(),
                    s.commit_disposition.as_str(),
                    s.compatibility_floor_change.as_str(),
                    s.proposed.runtime_origin.as_str(),
                    s.proposed.host_class.as_str(),
                ),
            })
            .collect();
        M5InstallReviewExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_gates_consistent: self.all_gates_consistent(),
            unified_review_required_count: self
                .review_sheets
                .iter()
                .filter(|s| s.commit_disposition == CommitDisposition::UnifiedReviewRequired)
                .count(),
            blocked_count: self
                .review_sheets
                .iter()
                .filter(|s| s.commit_disposition == CommitDisposition::Blocked)
                .count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5InstallReviewViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_sheets = BTreeSet::new();
        for sheet in &self.review_sheets {
            if !seen_sheets.insert(sheet.sheet_id.clone()) {
                violations.push(M5InstallReviewViolation::DuplicateSheetId {
                    sheet_id: sheet.sheet_id.clone(),
                });
            }
            self.validate_sheet(sheet, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(M5InstallReviewViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5InstallReviewViolation>) {
        if self.schema_version != M5_INSTALL_REVIEW_SCHEMA_VERSION {
            violations.push(M5InstallReviewViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_INSTALL_REVIEW_RECORD_KIND {
            violations.push(M5InstallReviewViolation::UnsupportedRecordKind {
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
                violations.push(M5InstallReviewViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "package_kinds",
                self.package_kinds == ArtifactFamily::ALL.to_vec(),
            ),
            (
                "source_classes",
                self.source_classes == SourceClass::ALL.to_vec(),
            ),
            (
                "runtime_origins",
                self.runtime_origins == RuntimeOrigin::ALL.to_vec(),
            ),
            (
                "support_classes",
                self.support_classes == SupportClass::ALL.to_vec(),
            ),
            (
                "compatibility_labels",
                self.compatibility_labels == CompatibilityLabel::ALL.to_vec(),
            ),
            (
                "rollback_postures",
                self.rollback_postures == RollbackPosture::ALL.to_vec(),
            ),
            (
                "change_kinds",
                self.change_kinds == ReviewChangeKind::ALL.to_vec(),
            ),
            ("scopes", self.scopes == InstallScope::ALL.to_vec()),
            ("host_classes", self.host_classes == HostClass::ALL.to_vec()),
            (
                "capability_requirements",
                self.capability_requirements == CapabilityRequirement::ALL.to_vec(),
            ),
            (
                "capability_origins",
                self.capability_origins == CapabilityOrigin::ALL.to_vec(),
            ),
            (
                "capability_changes",
                self.capability_changes == CapabilityChange::ALL.to_vec(),
            ),
            (
                "publisher_transfer_states",
                self.publisher_transfer_states == PublisherTransferState::ALL.to_vec(),
            ),
            (
                "signing_root_continuities",
                self.signing_root_continuities == SigningRootContinuity::ALL.to_vec(),
            ),
            (
                "namespace_states",
                self.namespace_states == NamespaceState::ALL.to_vec(),
            ),
            (
                "restart_impacts",
                self.restart_impacts == RestartImpact::ALL.to_vec(),
            ),
            (
                "open_work_impacts",
                self.open_work_impacts == OpenWorkImpact::ALL.to_vec(),
            ),
            (
                "compatibility_floor_changes",
                self.compatibility_floor_changes == CompatibilityFloorChange::ALL.to_vec(),
            ),
            (
                "review_triggers",
                self.review_triggers == ReviewTrigger::ALL.to_vec(),
            ),
            (
                "commit_dispositions",
                self.commit_dispositions == CommitDisposition::ALL.to_vec(),
            ),
            (
                "review_action_kinds",
                self.review_action_kinds == ReviewActionKind::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5InstallReviewViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_sheet(
        &self,
        sheet: &InstallReviewSheet,
        violations: &mut Vec<M5InstallReviewViolation>,
    ) {
        for (field, value) in [
            ("sheet_id", &sheet.sheet_id),
            ("listing_ref", &sheet.listing_ref),
            ("display_label", &sheet.display_label),
            ("governance_family_ref", &sheet.governance_family_ref),
            ("summary", &sheet.summary),
        ] {
            if value.trim().is_empty() {
                violations.push(M5InstallReviewViolation::EmptyField {
                    id: sheet.sheet_id.clone(),
                    field_name: field,
                });
            }
        }

        // A change kind that expects a current revision must carry one, and a fresh
        // install must not, so the comparison model is honest about what it compares.
        match (&sheet.current, sheet.change_kind.expects_current()) {
            (None, true) => violations.push(M5InstallReviewViolation::MissingCurrentRevision {
                sheet_id: sheet.sheet_id.clone(),
            }),
            (Some(_), false) => {
                violations.push(M5InstallReviewViolation::UnexpectedCurrentRevision {
                    sheet_id: sheet.sheet_id.clone(),
                });
            }
            _ => {}
        }

        // The recorded publisher-transfer state must agree with the publisher refs,
        // so a changed publisher can never be hidden behind a "same publisher" label.
        if let Some(current) = &sheet.current {
            let refs_equal = current.publisher_ref == sheet.proposed.publisher_ref;
            let labelled_same = sheet.publisher_transfer == PublisherTransferState::SamePublisher;
            if refs_equal != labelled_same {
                violations.push(M5InstallReviewViolation::PublisherTransferInconsistent {
                    sheet_id: sheet.sheet_id.clone(),
                });
            }
        }

        for delta in &sheet.capability_deltas {
            for (field, value) in [
                ("capability_ref", &delta.capability_ref),
                ("display_label", &delta.display_label),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5InstallReviewViolation::EmptyField {
                        id: sheet.sheet_id.clone(),
                        field_name: field,
                    });
                }
            }
        }

        // A state-changing rollback plan must name a durable fallback route.
        if sheet.rollback.rollback_posture != RollbackPosture::NotApplicable {
            for (field, value) in [
                (
                    "checkpoint_handle_ref",
                    &sheet.rollback.checkpoint_handle_ref,
                ),
                ("fallback_package_ref", &sheet.rollback.fallback_package_ref),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5InstallReviewViolation::EmptyRollbackRef {
                        sheet_id: sheet.sheet_id.clone(),
                        field_name: field,
                    });
                }
            }
        }

        self.validate_sheet_actions(sheet, violations);
        self.validate_sheet_gate(sheet, violations);
    }

    fn validate_sheet_actions(
        &self,
        sheet: &InstallReviewSheet,
        violations: &mut Vec<M5InstallReviewViolation>,
    ) {
        for action in &sheet.actions {
            if action.action_ref.trim().is_empty() {
                violations.push(M5InstallReviewViolation::EmptyField {
                    id: sheet.sheet_id.clone(),
                    field_name: "action_ref",
                });
            }
            // Every action must carry the sheet's scope, so a workspace
            // troubleshooting action can never silently apply at profile or global
            // scope.
            if action.scope != sheet.scope {
                violations.push(M5InstallReviewViolation::ActionScopeMismatch {
                    sheet_id: sheet.sheet_id.clone(),
                    action: action.action_kind.as_str(),
                });
            }
        }

        if !sheet.offers_action(ReviewActionKind::Commit) {
            violations.push(M5InstallReviewViolation::MissingRequiredAction {
                sheet_id: sheet.sheet_id.clone(),
                action: ReviewActionKind::Commit.as_str(),
            });
        }
        if !sheet.offers_action(ReviewActionKind::Cancel) {
            violations.push(M5InstallReviewViolation::MissingRequiredAction {
                sheet_id: sheet.sheet_id.clone(),
                action: ReviewActionKind::Cancel.as_str(),
            });
        }

        // A blocked commit must not expose an enabled commit action.
        if sheet.commit_disposition == CommitDisposition::Blocked {
            if let Some(commit) = sheet.commit_action() {
                if commit.enabled {
                    violations.push(M5InstallReviewViolation::BlockedCommitEnabled {
                        sheet_id: sheet.sheet_id.clone(),
                    });
                }
            }
        }
    }

    fn validate_sheet_gate(
        &self,
        sheet: &InstallReviewSheet,
        violations: &mut Vec<M5InstallReviewViolation>,
    ) {
        let mut seen_triggers = BTreeSet::new();
        for trigger in &sheet.review_triggers {
            if !seen_triggers.insert(*trigger) {
                violations.push(M5InstallReviewViolation::DuplicateReviewTrigger {
                    sheet_id: sheet.sheet_id.clone(),
                    trigger: trigger.as_str(),
                });
            }
        }

        let computed_floor = sheet.computed_compatibility_floor_change();
        if sheet.compatibility_floor_change != computed_floor {
            violations.push(M5InstallReviewViolation::CompatibilityFloorMismatch {
                sheet_id: sheet.sheet_id.clone(),
                stored: sheet.compatibility_floor_change.as_str(),
                computed: computed_floor.as_str(),
            });
        }

        // The recorded triggers must equal the recomputed set, so a widening,
        // publisher change, or runtime-origin change can never be asserted or hidden
        // by hand.
        if sheet.review_triggers != sheet.computed_review_triggers() {
            violations.push(M5InstallReviewViolation::ReviewTriggersMismatch {
                sheet_id: sheet.sheet_id.clone(),
            });
        }

        // The published commit disposition must equal the recomputed gate, so a
        // newly widened, re-published, or re-hosted change can never present a
        // narrower commit path than its facts warrant.
        let computed_disposition = sheet.computed_commit_disposition();
        if sheet.commit_disposition != computed_disposition {
            violations.push(M5InstallReviewViolation::CommitDispositionMismatch {
                sheet_id: sheet.sheet_id.clone(),
                stored: sheet.commit_disposition.as_str(),
                computed: computed_disposition.as_str(),
            });
        }
    }
}

/// A validation violation for the M5 install-review packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5InstallReviewViolation {
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
        /// Sheet or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A sheet id appears more than once.
    DuplicateSheetId {
        /// Duplicate sheet id.
        sheet_id: String,
    },
    /// A change kind that expects a current revision has none.
    MissingCurrentRevision {
        /// Sheet id.
        sheet_id: String,
    },
    /// A fresh install carries a current revision.
    UnexpectedCurrentRevision {
        /// Sheet id.
        sheet_id: String,
    },
    /// The publisher-transfer state disagrees with the publisher refs.
    PublisherTransferInconsistent {
        /// Sheet id.
        sheet_id: String,
    },
    /// A state-changing rollback plan is missing a durable ref.
    EmptyRollbackRef {
        /// Sheet id.
        sheet_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// An action's scope disagrees with its sheet's scope.
    ActionScopeMismatch {
        /// Sheet id.
        sheet_id: String,
        /// Action token.
        action: &'static str,
    },
    /// A required action is missing from a sheet.
    MissingRequiredAction {
        /// Sheet id.
        sheet_id: String,
        /// Action token.
        action: &'static str,
    },
    /// A blocked commit exposes an enabled commit action.
    BlockedCommitEnabled {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet lists a review trigger more than once.
    DuplicateReviewTrigger {
        /// Sheet id.
        sheet_id: String,
        /// Trigger token.
        trigger: &'static str,
    },
    /// A sheet's stored compatibility-floor change disagrees with the recomputation.
    CompatibilityFloorMismatch {
        /// Sheet id.
        sheet_id: String,
        /// Stored floor-change token.
        stored: &'static str,
        /// Recomputed floor-change token.
        computed: &'static str,
    },
    /// A sheet's review triggers disagree with the recomputed set.
    ReviewTriggersMismatch {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet's commit disposition disagrees with the recomputed gate.
    CommitDispositionMismatch {
        /// Sheet id.
        sheet_id: String,
        /// Stored disposition token.
        stored: &'static str,
        /// Recomputed disposition token.
        computed: &'static str,
    },
    /// The summary counts disagree with the review sheets.
    SummaryMismatch,
}

impl fmt::Display for M5InstallReviewViolation {
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
            Self::DuplicateSheetId { sheet_id } => {
                write!(f, "duplicate review sheet id {sheet_id}")
            }
            Self::MissingCurrentRevision { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} change kind expects a current revision but has none"
                )
            }
            Self::UnexpectedCurrentRevision { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} is a fresh install but carries a current revision"
                )
            }
            Self::PublisherTransferInconsistent { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} publisher-transfer state disagrees with its publisher refs"
                )
            }
            Self::EmptyRollbackRef {
                sheet_id,
                field_name,
            } => {
                write!(f, "sheet {sheet_id} rollback plan is missing {field_name}")
            }
            Self::ActionScopeMismatch { sheet_id, action } => {
                write!(
                    f,
                    "sheet {sheet_id} action {action} is scoped outside the sheet's scope"
                )
            }
            Self::MissingRequiredAction { sheet_id, action } => {
                write!(f, "sheet {sheet_id} is missing required action {action}")
            }
            Self::BlockedCommitEnabled { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} blocks the commit but exposes an enabled commit action"
                )
            }
            Self::DuplicateReviewTrigger { sheet_id, trigger } => {
                write!(f, "sheet {sheet_id} repeats review trigger {trigger}")
            }
            Self::CompatibilityFloorMismatch {
                sheet_id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "sheet {sheet_id} records compatibility floor change {stored} but the recomputed change is {computed}"
                )
            }
            Self::ReviewTriggersMismatch { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} review triggers disagree with the recomputed set"
                )
            }
            Self::CommitDispositionMismatch {
                sheet_id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "sheet {sheet_id} publishes commit disposition {stored} but the recomputed gate is {computed}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the review sheets")
            }
        }
    }
}

impl Error for M5InstallReviewViolation {}

/// Loads the embedded M5 install-review packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5InstallReview`].
pub fn current_m5_install_review() -> Result<M5InstallReview, serde_json::Error> {
    serde_json::from_str(M5_INSTALL_REVIEW_JSON)
}

#[cfg(test)]
mod tests;
