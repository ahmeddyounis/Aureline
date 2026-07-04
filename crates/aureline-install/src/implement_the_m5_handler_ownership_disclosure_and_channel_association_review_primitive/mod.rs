//! Implements the reusable handler-ownership primitive: a handler-ownership / precedence
//! disclosure card, a set of channel-association review rows for protocol / file / recent-item
//! / notification handlers, and a recovery-alignment block that all resolve from one
//! handler-ownership context and share one ownership identity, so a side-by-side install can
//! always explain which build currently owns file associations and why, handler changes stay
//! previewable and reversible instead of silent takeovers, and system-open / deep-link /
//! recent-item / notification recovery stays aligned with channel ownership and rollback
//! identity on every claimed deployment surface.
//!
//! Where
//! [`crate::freeze_the_m5_deployment_continuity_component_matrix`] *freezes* the reusable
//! deployment / continuity component families as a governed contract, and the sibling
//! `implement_the_m5_*` lanes narrow the install-profile / rollout, deployment-summary /
//! residual / plane-status, and mirror / offline / mode-change / channel-association families,
//! this module *narrows* the handler-ownership and channel-association truth those surfaces
//! already claim — the frozen
//! [`crate::freeze_the_m5_deployment_continuity_component_matrix::M5DeploymentComponentFamily::ChannelAssociationReviewRow`]
//! and install-profile handler-ownership descriptors — into one working primitive with a real
//! **resolver**. A single handler-ownership context projects onto a disclosure card, its
//! channel-association review rows, and a recovery-alignment block that all carry one ownership
//! identity, so current owner, proposed owner, precedence, user-facing impact, and rollback
//! identity never blur across them.
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — a side-by-side install can explain which build owns file associations and why.**
//!   The disclosure card names the owning install, its owner class, the precedence state
//!   (`Sole owner`, `Primary among installs`, `Shared contested`, `Superseded`,
//!   `Not registered`), and a precise ownership reason; every channel row discloses its
//!   current owner. A card that hides the current owner is rejected.
//! - **AC2 — handler changes are previewable and reversible instead of silent takeovers.**
//!   Every channel-association review row keeps bounded keep / reassign / cancel actions, names
//!   the proposed owner and user-facing impact, and — when a change is proposed — is previewable
//!   and reversible. A row that silently captures a default handler, or a change that is not
//!   previewable or not reversible, is rejected rather than applied.
//! - **AC3 — support packets preserve handler ownership and precedence truth.** The card keeps
//!   a rollback identity and stays inspectable without manual installer inspection, and every
//!   system-open / deep-link / recent-item / notification recovery path resolves to the current
//!   channel owner and carries the rollback identity. A recovery path that routes away from the
//!   disclosed owner, or a change forced without a rollback identity, is rejected.
//!
//! Raw config bytes, credentials, license keys, handler URIs, registry paths, and device
//! identifiers never cross this boundary; the resolver carries only opaque refs, typed class
//! tokens, booleans, and redacted labels, so support and diagnostics exports reconstruct
//! exactly what a surface would have shown without leaking source or live payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-handler-ownership-primitive.schema.json`](../../../../schemas/ui/m5-handler-ownership-primitive.schema.json).
//! The contract doc is
//! [`docs/deployment/m5_handler_ownership_primitive.md`](../../../../docs/deployment/m5_handler_ownership_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_deployment_continuity_component_matrix::{
    DegradedState, M5DeploymentDowngradeTrigger, M5DeploymentMode,
};

/// Stable record-kind tag carried by [`M5HandlerOwnershipPrimitivePacket`].
pub const M5_HANDLER_OWNERSHIP_RECORD_KIND: &str = "m5_handler_ownership_primitive";

/// Schema version for the handler-ownership primitive packet.
pub const M5_HANDLER_OWNERSHIP_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_HANDLER_OWNERSHIP_SCHEMA_REF: &str =
    "schemas/ui/m5-handler-ownership-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_HANDLER_OWNERSHIP_DOC_REF: &str = "docs/deployment/m5_handler_ownership_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive narrows.
pub const M5_HANDLER_OWNERSHIP_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-deployment-continuity-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_HANDLER_OWNERSHIP_FIXTURE_DIR: &str = "fixtures/ui/m5-handler-ownership-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const M5_HANDLER_OWNERSHIP_ARTIFACT_REF: &str =
    "artifacts/release/m5-handler-ownership-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_HANDLER_OWNERSHIP_CSV_REF: &str =
    "artifacts/release/m5-handler-ownership-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_HANDLER_OWNERSHIP_REPORT_REF: &str =
    "artifacts/release/m5-handler-ownership-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed handler-ownership surface family. Each family is one parity surface that ingests
/// the shared primitive; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandlerSurfaceFamily {
    /// The About page's desktop-integration section.
    AboutIntegration,
    /// The diagnostics handler-ownership pane.
    DiagnosticsHandlers,
    /// The install / side-by-side review surface.
    InstallReview,
    /// The support / export replay surface reconstructing ownership truth.
    SupportExportReplay,
    /// The notification / activity center routing surface.
    NotificationCenter,
    /// The docs / help handler-reference surface.
    DocsHandlerReference,
}

impl M5HandlerSurfaceFamily {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AboutIntegration,
        Self::DiagnosticsHandlers,
        Self::InstallReview,
        Self::SupportExportReplay,
        Self::NotificationCenter,
        Self::DocsHandlerReference,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AboutIntegration => "about_integration",
            Self::DiagnosticsHandlers => "diagnostics_handlers",
            Self::InstallReview => "install_review",
            Self::SupportExportReplay => "support_export_replay",
            Self::NotificationCenter => "notification_center",
            Self::DocsHandlerReference => "docs_handler_reference",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AboutIntegration => "About desktop-integration section",
            Self::DiagnosticsHandlers => "Diagnostics handler-ownership pane",
            Self::InstallReview => "Install / side-by-side review",
            Self::SupportExportReplay => "Support / export replay",
            Self::NotificationCenter => "Notification / activity center",
            Self::DocsHandlerReference => "Docs handler reference",
        }
    }
}

/// Closed handler-channel-class vocabulary. Names the kind of channel a review row governs so
/// the same component family covers file associations, protocol handlers, recent-item reopen
/// paths, notification actions, deep links, and system-open routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandlerChannelClass {
    /// A file-extension association (which build opens a file type).
    FileAssociation,
    /// A protocol / URL-scheme handler.
    ProtocolHandler,
    /// A recent-item reopen path.
    RecentItemReopen,
    /// A notification action route.
    NotificationAction,
    /// A deep link into the application.
    DeepLink,
    /// A system-open ("open with") route.
    SystemOpen,
}

impl M5HandlerChannelClass {
    /// Every channel class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FileAssociation,
        Self::ProtocolHandler,
        Self::RecentItemReopen,
        Self::NotificationAction,
        Self::DeepLink,
        Self::SystemOpen,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileAssociation => "file_association",
            Self::ProtocolHandler => "protocol_handler",
            Self::RecentItemReopen => "recent_item_reopen",
            Self::NotificationAction => "notification_action",
            Self::DeepLink => "deep_link",
            Self::SystemOpen => "system_open",
        }
    }

    /// True when the channel is a recovery route that must stay aligned with the disclosed
    /// owner and carry the rollback identity: system-open, deep-link, recent-item, and
    /// notification-action recovery.
    pub const fn is_recovery_path(self) -> bool {
        matches!(
            self,
            Self::RecentItemReopen | Self::NotificationAction | Self::DeepLink | Self::SystemOpen
        )
    }
}

/// Closed handler-owner-class vocabulary. Names which build currently owns a handler so a
/// side-by-side beta, a portable install, or a non-Aureline app never blur into one another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandlerOwnerClass {
    /// The primary stable install.
    PrimaryStableInstall,
    /// A side-by-side beta / insider install.
    SideBySideBetaInstall,
    /// A portable install carrying its own state root.
    PortableInstall,
    /// A managed-fleet install.
    ManagedFleetInstall,
    /// A non-Aureline external application.
    ExternalNonAureline,
    /// No current owner is registered.
    Unowned,
}

impl M5HandlerOwnerClass {
    /// Every owner class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PrimaryStableInstall,
        Self::SideBySideBetaInstall,
        Self::PortableInstall,
        Self::ManagedFleetInstall,
        Self::ExternalNonAureline,
        Self::Unowned,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryStableInstall => "primary_stable_install",
            Self::SideBySideBetaInstall => "side_by_side_beta_install",
            Self::PortableInstall => "portable_install",
            Self::ManagedFleetInstall => "managed_fleet_install",
            Self::ExternalNonAureline => "external_non_aureline",
            Self::Unowned => "unowned",
        }
    }
}

/// Closed handler-precedence-state vocabulary. Names where an install stands in the ownership
/// order for a handler, so a side-by-side install can explain which build wins and why.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandlerPrecedenceState {
    /// This install is the sole registered owner.
    SoleOwner,
    /// This install owns and wins, but other installs are registered at lower precedence.
    PrimaryAmongInstalls,
    /// Multiple installs contest ownership; the order is disclosed for review.
    SharedContested,
    /// Another install currently wins; this install is superseded.
    Superseded,
    /// This install is not registered for the channel.
    NotRegistered,
}

impl M5HandlerPrecedenceState {
    /// Every precedence state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SoleOwner,
        Self::PrimaryAmongInstalls,
        Self::SharedContested,
        Self::Superseded,
        Self::NotRegistered,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SoleOwner => "sole_owner",
            Self::PrimaryAmongInstalls => "primary_among_installs",
            Self::SharedContested => "shared_contested",
            Self::Superseded => "superseded",
            Self::NotRegistered => "not_registered",
        }
    }

    /// True when the precedence state describes more than one install participating in
    /// ownership, so a side-by-side story must be told.
    pub const fn indicates_multiple_installs(self) -> bool {
        matches!(
            self,
            Self::PrimaryAmongInstalls | Self::SharedContested | Self::Superseded
        )
    }
}

/// Closed handler-change-state vocabulary. Names what change (if any) a channel-association
/// review row proposes so a takeover is never applied silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandlerChangeState {
    /// No change is proposed; the current owner stays.
    NoChange,
    /// The handler is being reassigned to this install.
    ReassignToThisInstall,
    /// The handler is being released to another install.
    ReleaseToOtherInstall,
    /// The handler is contested and awaits a review decision.
    ContestedAwaitingReview,
    /// The handler is being reverted to its previous owner.
    RevertToPrevious,
}

impl M5HandlerChangeState {
    /// Every change state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoChange,
        Self::ReassignToThisInstall,
        Self::ReleaseToOtherInstall,
        Self::ContestedAwaitingReview,
        Self::RevertToPrevious,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoChange => "no_change",
            Self::ReassignToThisInstall => "reassign_to_this_install",
            Self::ReleaseToOtherInstall => "release_to_other_install",
            Self::ContestedAwaitingReview => "contested_awaiting_review",
            Self::RevertToPrevious => "revert_to_previous",
        }
    }

    /// True when the row proposes an actual handler change (anything other than
    /// [`Self::NoChange`]).
    pub const fn is_change(self) -> bool {
        !matches!(self, Self::NoChange)
    }
}

/// Closed user-facing-impact vocabulary. Names the observable consequence of a handler change
/// so review rows explain what the user will notice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandlerImpactClass {
    /// Files of this type will open in this build.
    OpensInThisBuild,
    /// Files of this type will open in another build.
    OpensInOtherBuild,
    /// Links / protocols will route to this build.
    RoutesToThisBuild,
    /// Items resolve in place with no owner change.
    ResolvesInPlace,
    /// No user-visible change results.
    NoUserVisibleChange,
    /// The user must choose an owner before the change applies.
    RequiresUserChoice,
}

impl M5HandlerImpactClass {
    /// Every impact class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpensInThisBuild,
        Self::OpensInOtherBuild,
        Self::RoutesToThisBuild,
        Self::ResolvesInPlace,
        Self::NoUserVisibleChange,
        Self::RequiresUserChoice,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpensInThisBuild => "opens_in_this_build",
            Self::OpensInOtherBuild => "opens_in_other_build",
            Self::RoutesToThisBuild => "routes_to_this_build",
            Self::ResolvesInPlace => "resolves_in_place",
            Self::NoUserVisibleChange => "no_user_visible_change",
            Self::RequiresUserChoice => "requires_user_choice",
        }
    }
}

/// Closed channel-association-action vocabulary. Names the bounded actions a channel-association
/// review row always keeps reachable so a handler change is reviewed, not silently captured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChannelAssociationAction {
    /// Keep the current owner.
    Keep,
    /// Reassign the handler to the proposed owner.
    Reassign,
    /// Cancel the proposed change.
    Cancel,
    /// Preview the change before applying it.
    PreviewChange,
}

impl M5ChannelAssociationAction {
    /// Every association action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Keep,
        Self::Reassign,
        Self::Cancel,
        Self::PreviewChange,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Keep => "keep",
            Self::Reassign => "reassign",
            Self::Cancel => "cancel",
            Self::PreviewChange => "preview_change",
        }
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet must carry per
/// surface; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandlerOwnershipExportField {
    /// The stable ownership identity shared across surfaces.
    OwnershipId,
    /// The channel class each row governs.
    ChannelClass,
    /// The current owner of the channel.
    CurrentOwner,
    /// The proposed owner of the channel.
    ProposedOwner,
    /// The handler precedence state.
    PrecedenceState,
    /// The user-facing impact of the change.
    UserFacingImpact,
    /// The bounded keep / reassign / cancel actions.
    ChannelActions,
    /// The rollback identity for the change.
    RollbackIdentity,
    /// The recovery-path alignment.
    RecoveryAlignment,
    /// The reason the disclosed build owns the handler.
    OwnershipReason,
}

impl M5HandlerOwnershipExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::OwnershipId,
        Self::ChannelClass,
        Self::CurrentOwner,
        Self::ProposedOwner,
        Self::PrecedenceState,
        Self::UserFacingImpact,
        Self::ChannelActions,
        Self::RollbackIdentity,
        Self::RecoveryAlignment,
        Self::OwnershipReason,
    ];

    /// The mandatory subset every row must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::OwnershipId,
        Self::CurrentOwner,
        Self::PrecedenceState,
        Self::UserFacingImpact,
        Self::ChannelActions,
        Self::RollbackIdentity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnershipId => "ownership_id",
            Self::ChannelClass => "channel_class",
            Self::CurrentOwner => "current_owner",
            Self::ProposedOwner => "proposed_owner",
            Self::PrecedenceState => "precedence_state",
            Self::UserFacingImpact => "user_facing_impact",
            Self::ChannelActions => "channel_actions",
            Self::RollbackIdentity => "rollback_identity",
            Self::RecoveryAlignment => "recovery_alignment",
            Self::OwnershipReason => "ownership_reason",
        }
    }
}

// --- resolver input ---

/// One channel association the handler-ownership context governs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChannelAssociationInput {
    /// Opaque ref to the channel / protocol / file association; never a raw URI.
    pub channel_ref: String,
    /// The kind of channel this row governs.
    pub channel_class: M5HandlerChannelClass,
    /// Opaque ref to the current owner build.
    pub current_owner_ref: String,
    /// The class of the current owner build.
    pub current_owner_class: M5HandlerOwnerClass,
    /// Opaque ref to the proposed owner build (equal to the current owner when no change).
    pub proposed_owner_ref: String,
    /// The class of the proposed owner build.
    pub proposed_owner_class: M5HandlerOwnerClass,
    /// The change (if any) this row proposes.
    pub change_state: M5HandlerChangeState,
    /// The user-facing impact of the change.
    pub impact_class: M5HandlerImpactClass,
    /// The change silently captures a default handler; must be `false` (AC2).
    pub last_writer_wins_capture: bool,
    /// The change is reviewed before it is applied; must hold (AC2).
    pub reviewed_before_apply: bool,
    /// The change can be previewed before applying; must hold when a change is proposed (AC2).
    pub previewable: bool,
    /// The change can be reversed; must hold (AC2).
    pub reversible: bool,
    /// The row discloses the current owner before the change; must hold (AC1).
    pub discloses_current_owner: bool,
}

/// The full input to the handler-ownership resolver for one ownership context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandlerOwnershipInput {
    /// The stable ownership identity that must survive across the disclosure card, the
    /// channel-association review rows, and the recovery-alignment block.
    pub ownership_id: String,
    /// Human-readable surface / context label.
    pub surface_label: String,
    /// The operating / install mode the ownership context acts in.
    pub deployment_mode: M5DeploymentMode,
    /// Opaque ref to the install whose handler ownership this context discloses.
    pub install_identity_ref: String,
    /// The class of the disclosed owning build.
    pub owner_class: M5HandlerOwnerClass,
    /// Where the disclosed install stands in the ownership order.
    pub precedence_state: M5HandlerPrecedenceState,
    /// A precise, non-generic reason the disclosed build owns the handler (the "and why").
    pub ownership_reason: String,
    /// Opaque ref to the rollback identity for handler changes; must be non-empty (AC3).
    pub rollback_identity_ref: String,
    /// The disclosure card is inspectable without manual installer inspection; must hold (AC2).
    pub inspectable_without_installer: bool,
    /// The card discloses the current owner; must hold (AC1).
    pub discloses_current_owner: bool,
    /// The channel associations the ownership context governs.
    pub channels: Vec<M5ChannelAssociationInput>,
    /// An externally-observed narrowing (handler ownership contested) that degrades the surface
    /// before action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

// --- resolved projections ---

/// The resolved handler-ownership / precedence disclosure card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedHandlerOwnershipCard {
    /// The ownership identity — identical to the review rows and recovery block.
    pub ownership_id: String,
    /// The opaque install identity ref.
    pub install_identity_ref: String,
    /// The class of the disclosed owning build.
    pub owner_class: M5HandlerOwnerClass,
    /// The operating / install mode the card acts in.
    pub deployment_mode: M5DeploymentMode,
    /// Where the disclosed install stands in the ownership order.
    pub precedence_state: M5HandlerPrecedenceState,
    /// The precise reason the disclosed build owns the handler.
    pub ownership_reason: String,
    /// The opaque rollback identity ref.
    pub rollback_identity_ref: String,
    /// The card discloses the current owner (AC1); always holds.
    pub discloses_current_owner: bool,
    /// The card discloses the precedence order (AC1); always holds.
    pub discloses_precedence: bool,
    /// The card is inspectable without manual installer inspection (AC2); always holds.
    pub inspectable_without_installer: bool,
}

/// The resolved channel-association review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedChannelAssociationReviewRow {
    /// The ownership identity — identical to every other surface.
    pub ownership_id: String,
    /// The opaque channel ref.
    pub channel_ref: String,
    /// The kind of channel this row governs.
    pub channel_class: M5HandlerChannelClass,
    /// The opaque current-owner ref.
    pub current_owner_ref: String,
    /// The class of the current owner.
    pub current_owner_class: M5HandlerOwnerClass,
    /// The opaque proposed-owner ref.
    pub proposed_owner_ref: String,
    /// The class of the proposed owner.
    pub proposed_owner_class: M5HandlerOwnerClass,
    /// The change (if any) this row proposes.
    pub change_state: M5HandlerChangeState,
    /// The user-facing impact of the change.
    pub impact_class: M5HandlerImpactClass,
    /// The bounded actions kept reachable on the row (keep / reassign / cancel always present).
    pub actions: Vec<M5ChannelAssociationAction>,
    /// The change never silently captures a default handler (AC2); always `false`.
    pub last_writer_wins_capture: bool,
    /// The change is reviewed before it is applied; always holds.
    pub reviewed_before_apply: bool,
    /// The change can be previewed before applying; always holds when a change is proposed.
    pub previewable: bool,
    /// The change can be reversed; always holds.
    pub reversible: bool,
    /// The row discloses the current owner (AC1); always holds.
    pub discloses_current_owner: bool,
    /// The channel is a recovery route aligned with the disclosed owner.
    pub is_recovery_path: bool,
}

/// One resolved recovery-path alignment entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRecoveryPath {
    /// The ownership identity — identical to every other surface.
    pub ownership_id: String,
    /// The recovery channel class.
    pub channel_class: M5HandlerChannelClass,
    /// The opaque owner the recovery path resolves to (the current channel owner).
    pub resolves_to_owner_ref: String,
    /// The recovery path resolves to the disclosed channel / install owner (AC3); always holds.
    pub aligned_with_channel_owner: bool,
    /// The recovery path carries the rollback identity (AC3); always holds.
    pub carries_rollback_identity: bool,
}

/// The resolved recovery-alignment block covering system-open, deep-link, recent-item, and
/// notification-action recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRecoveryAlignment {
    /// The ownership identity — identical to every other surface.
    pub ownership_id: String,
    /// The opaque rollback identity ref shared by every recovery path.
    pub rollback_identity_ref: String,
    /// The resolved recovery paths (one per recovery-class channel).
    pub recovery_paths: Vec<M5ResolvedRecoveryPath>,
    /// Every recovery path resolves to the disclosed owner (AC3); always holds.
    pub all_paths_aligned_with_owner: bool,
    /// Every recovery path carries the rollback identity (AC3); always holds.
    pub all_paths_carry_rollback_identity: bool,
}

/// The resolved handler-ownership truth shared across the disclosure card, the
/// channel-association review rows, and the recovery-alignment block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedHandlerOwnership {
    /// The stable ownership identity.
    pub ownership_id: String,
    /// The resolved handler-ownership / precedence disclosure card.
    pub disclosure_card: M5ResolvedHandlerOwnershipCard,
    /// The resolved channel-association review rows.
    pub association_rows: Vec<M5ResolvedChannelAssociationReviewRow>,
    /// The resolved recovery-alignment block.
    pub recovery_alignment: M5ResolvedRecoveryAlignment,
    /// The current owner and precedence are disclosed without manual installer inspection
    /// (AC1).
    pub owner_and_precedence_disclosed: bool,
    /// Handler changes are previewable and reversible rather than silent takeovers (AC2).
    pub changes_previewable_and_reversible: bool,
    /// Handler ownership and precedence truth are preserved in the export, with recovery paths
    /// aligned to the owner and carrying the rollback identity (AC3).
    pub ownership_precedence_preserved_in_export: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedHandlerOwnership {
    /// True when the ownership identity is identical across the card, the review rows, and the
    /// recovery block.
    pub fn identity_consistent(&self) -> bool {
        self.disclosure_card.ownership_id == self.ownership_id
            && self.recovery_alignment.ownership_id == self.ownership_id
            && self
                .association_rows
                .iter()
                .all(|row| row.ownership_id == self.ownership_id)
            && self
                .recovery_alignment
                .recovery_paths
                .iter()
                .all(|path| path.ownership_id == self.ownership_id)
    }

    /// True when the disclosed precedence describes more than one install (a genuine
    /// side-by-side story).
    pub fn has_contested_or_multi_install(&self) -> bool {
        self.disclosure_card
            .precedence_state
            .indicates_multiple_installs()
    }

    /// True when at least one review row proposes an actual handler change.
    pub fn has_proposed_change(&self) -> bool {
        self.association_rows
            .iter()
            .any(|row| row.change_state.is_change())
    }

    /// True when at least one recovery path was resolved.
    pub fn has_recovery_path(&self) -> bool {
        !self.recovery_alignment.recovery_paths.is_empty()
    }

    /// True when the current owner and precedence are disclosed (AC1).
    pub fn owner_and_precedence_disclosed(&self) -> bool {
        self.owner_and_precedence_disclosed
    }

    /// True when handler changes are previewable and reversible (AC2).
    pub fn changes_previewable_and_reversible(&self) -> bool {
        self.changes_previewable_and_reversible
    }

    /// True when ownership and precedence truth are preserved in the export (AC3).
    pub fn ownership_precedence_preserved_in_export(&self) -> bool {
        self.ownership_precedence_preserved_in_export
    }
}

/// Errors returned by [`resolve_handler_ownership`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5HandlerOwnershipResolutionError {
    /// The ownership identity was empty.
    EmptyOwnershipId,
    /// The install identity ref was empty.
    EmptyInstallIdentityRef,
    /// The rollback identity ref was empty.
    EmptyRollbackIdentityRef,
    /// The ownership reason was empty.
    OwnershipReasonMissing,
    /// A channel ref was empty.
    EmptyChannelRef,
    /// A current- or proposed-owner ref was empty.
    EmptyOwnerRef,
    /// The ownership context governed no channels.
    NoChannels,
    /// A label, ref, or note carried forbidden material.
    ForbiddenMaterial,
    /// The disclosure card hid the current owner.
    OwnerNotDisclosed,
    /// The disclosure card required manual installer inspection.
    RequiresManualInstallerInspection,
    /// A channel association silently captured a default handler.
    SilentTakeover,
    /// A channel-association change was not reviewed before apply.
    ChannelChangeNotReviewed,
    /// A proposed change was not previewable.
    ChangeNotPreviewable,
    /// A proposed change was not reversible.
    ChangeNotReversible,
    /// A channel-association row hid the current owner before the change.
    ChannelOwnerHidden,
    /// A recovery path routed away from the disclosed owner.
    RecoveryPathMisaligned,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5HandlerOwnershipResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyOwnershipId => "empty_ownership_id",
            Self::EmptyInstallIdentityRef => "empty_install_identity_ref",
            Self::EmptyRollbackIdentityRef => "empty_rollback_identity_ref",
            Self::OwnershipReasonMissing => "ownership_reason_missing",
            Self::EmptyChannelRef => "empty_channel_ref",
            Self::EmptyOwnerRef => "empty_owner_ref",
            Self::NoChannels => "no_channels",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::OwnerNotDisclosed => "owner_not_disclosed",
            Self::RequiresManualInstallerInspection => "requires_manual_installer_inspection",
            Self::SilentTakeover => "silent_takeover",
            Self::ChannelChangeNotReviewed => "channel_change_not_reviewed",
            Self::ChangeNotPreviewable => "change_not_previewable",
            Self::ChangeNotReversible => "change_not_reversible",
            Self::ChannelOwnerHidden => "channel_owner_hidden",
            Self::RecoveryPathMisaligned => "recovery_path_misaligned",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5HandlerOwnershipResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "handler-ownership resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5HandlerOwnershipResolutionError {}

/// Resolves one handler-ownership context into its shared disclosure card, channel-association
/// review rows, and recovery-alignment block.
///
/// The three surfaces share one ownership identity, so current owner, proposed owner,
/// precedence, user-facing impact, and rollback identity never blur across them. A side-by-side
/// install can always explain which build owns file associations and why; handler changes keep
/// bounded keep / reassign / cancel actions and stay previewable and reversible rather than
/// silent takeovers; and system-open / deep-link / recent-item / notification recovery stays
/// aligned with the disclosed owner and carries the rollback identity.
pub fn resolve_handler_ownership(
    input: &M5HandlerOwnershipInput,
) -> Result<M5ResolvedHandlerOwnership, M5HandlerOwnershipResolutionError> {
    if input.ownership_id.trim().is_empty() {
        return Err(M5HandlerOwnershipResolutionError::EmptyOwnershipId);
    }
    if input.install_identity_ref.trim().is_empty() {
        return Err(M5HandlerOwnershipResolutionError::EmptyInstallIdentityRef);
    }
    if input.rollback_identity_ref.trim().is_empty() {
        return Err(M5HandlerOwnershipResolutionError::EmptyRollbackIdentityRef);
    }
    if input.ownership_reason.trim().is_empty() {
        return Err(M5HandlerOwnershipResolutionError::OwnershipReasonMissing);
    }
    if input.channels.is_empty() {
        return Err(M5HandlerOwnershipResolutionError::NoChannels);
    }

    let mut forbidden_scan: Vec<&str> = vec![
        input.ownership_id.as_str(),
        input.surface_label.as_str(),
        input.install_identity_ref.as_str(),
        input.ownership_reason.as_str(),
        input.rollback_identity_ref.as_str(),
    ];
    for channel in &input.channels {
        forbidden_scan.push(channel.channel_ref.as_str());
        forbidden_scan.push(channel.current_owner_ref.as_str());
        forbidden_scan.push(channel.proposed_owner_ref.as_str());
    }
    for value in forbidden_scan {
        if value_is_forbidden(value) {
            return Err(M5HandlerOwnershipResolutionError::ForbiddenMaterial);
        }
    }

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5HandlerOwnershipResolutionError::DegradedLabelGeneric);
        }
    }

    // AC1: the disclosure card names the current owner; AC2: it stays inspectable without
    // manual installer inspection.
    if !input.discloses_current_owner {
        return Err(M5HandlerOwnershipResolutionError::OwnerNotDisclosed);
    }
    if !input.inspectable_without_installer {
        return Err(M5HandlerOwnershipResolutionError::RequiresManualInstallerInspection);
    }

    let mut association_rows = Vec::with_capacity(input.channels.len());
    let mut recovery_paths = Vec::new();
    for channel in &input.channels {
        if channel.channel_ref.trim().is_empty() {
            return Err(M5HandlerOwnershipResolutionError::EmptyChannelRef);
        }
        if channel.current_owner_ref.trim().is_empty()
            || channel.proposed_owner_ref.trim().is_empty()
        {
            return Err(M5HandlerOwnershipResolutionError::EmptyOwnerRef);
        }
        // AC2: a channel association never silently captures a default handler; changes stay
        // reviewed, previewable, and reversible.
        if channel.last_writer_wins_capture {
            return Err(M5HandlerOwnershipResolutionError::SilentTakeover);
        }
        if !channel.reviewed_before_apply {
            return Err(M5HandlerOwnershipResolutionError::ChannelChangeNotReviewed);
        }
        if channel.change_state.is_change() && !channel.previewable {
            return Err(M5HandlerOwnershipResolutionError::ChangeNotPreviewable);
        }
        if !channel.reversible {
            return Err(M5HandlerOwnershipResolutionError::ChangeNotReversible);
        }
        // AC1: every row discloses the current owner before the change.
        if !channel.discloses_current_owner {
            return Err(M5HandlerOwnershipResolutionError::ChannelOwnerHidden);
        }
        // AC3: recovery routes resolve to the disclosed owner (not a stale or other build).
        let is_recovery_path = channel.channel_class.is_recovery_path();
        if is_recovery_path && channel.current_owner_class != input.owner_class {
            return Err(M5HandlerOwnershipResolutionError::RecoveryPathMisaligned);
        }

        // The bounded keep / reassign / cancel actions are always present; a preview action is
        // added when a change is proposed.
        let mut actions = vec![
            M5ChannelAssociationAction::Keep,
            M5ChannelAssociationAction::Reassign,
            M5ChannelAssociationAction::Cancel,
        ];
        if channel.change_state.is_change() {
            actions.push(M5ChannelAssociationAction::PreviewChange);
        }

        association_rows.push(M5ResolvedChannelAssociationReviewRow {
            ownership_id: input.ownership_id.clone(),
            channel_ref: channel.channel_ref.clone(),
            channel_class: channel.channel_class,
            current_owner_ref: channel.current_owner_ref.clone(),
            current_owner_class: channel.current_owner_class,
            proposed_owner_ref: channel.proposed_owner_ref.clone(),
            proposed_owner_class: channel.proposed_owner_class,
            change_state: channel.change_state,
            impact_class: channel.impact_class,
            actions,
            last_writer_wins_capture: false,
            reviewed_before_apply: true,
            previewable: channel.previewable,
            reversible: true,
            discloses_current_owner: true,
            is_recovery_path,
        });

        if is_recovery_path {
            recovery_paths.push(M5ResolvedRecoveryPath {
                ownership_id: input.ownership_id.clone(),
                channel_class: channel.channel_class,
                resolves_to_owner_ref: channel.current_owner_ref.clone(),
                aligned_with_channel_owner: true,
                carries_rollback_identity: true,
            });
        }
    }

    let disclosure_card = M5ResolvedHandlerOwnershipCard {
        ownership_id: input.ownership_id.clone(),
        install_identity_ref: input.install_identity_ref.clone(),
        owner_class: input.owner_class,
        deployment_mode: input.deployment_mode,
        precedence_state: input.precedence_state,
        ownership_reason: input.ownership_reason.clone(),
        rollback_identity_ref: input.rollback_identity_ref.clone(),
        discloses_current_owner: true,
        discloses_precedence: true,
        inspectable_without_installer: true,
    };

    let recovery_alignment = M5ResolvedRecoveryAlignment {
        ownership_id: input.ownership_id.clone(),
        rollback_identity_ref: input.rollback_identity_ref.clone(),
        recovery_paths,
        all_paths_aligned_with_owner: true,
        all_paths_carry_rollback_identity: true,
    };

    Ok(M5ResolvedHandlerOwnership {
        ownership_id: input.ownership_id.clone(),
        disclosure_card,
        association_rows,
        recovery_alignment,
        owner_and_precedence_disclosed: true,
        changes_previewable_and_reversible: true,
        ownership_precedence_preserved_in_export: true,
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

/// One worked resolution case carried in the packet so the support / export packet reconstructs
/// ownership truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandlerOwnershipCase {
    /// The resolver input.
    pub input: M5HandlerOwnershipInput,
    /// The resolved ownership truth. Must equal `resolve_handler_ownership(&input)`.
    pub resolved: M5ResolvedHandlerOwnership,
}

impl M5HandlerOwnershipCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5HandlerOwnershipInput) -> Self {
        let resolved = resolve_handler_ownership(&input).expect("seed ownership case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_handler_ownership(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one handler surface family bound to the shared
/// handler-ownership contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandlerOwnershipSurfaceRow {
    /// The handler surface family.
    pub surface_family: M5HandlerSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Channel classes this surface can disclose (must be non-empty).
    pub channel_classes: Vec<M5HandlerChannelClass>,
    /// Precedence states this surface renders (must be non-empty).
    pub precedence_states: Vec<M5HandlerPrecedenceState>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5HandlerOwnershipExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5DeploymentDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be non-empty).
    pub example_cases: Vec<M5HandlerOwnershipCase>,
    /// Hard invariant: this row never applies a silent takeover. MUST be `false`.
    pub shows_silent_takeover: bool,
    /// Hard invariant: this row never hides the current owner. MUST be `false`.
    pub hides_current_owner: bool,
    /// Hard invariant: this row never forces manual installer inspection. MUST be `false`.
    pub forces_manual_installer_inspection: bool,
    /// Hard invariant: this row never drops the rollback identity. MUST be `false`.
    pub drops_rollback_identity: bool,
}

impl M5HandlerOwnershipSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5HandlerOwnershipExportField> =
            self.export_fields.iter().copied().collect();
        M5HandlerOwnershipExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.shows_silent_takeover
            && !self.hides_current_owner
            && !self.forces_manual_installer_inspection
            && !self.drops_rollback_identity
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandlerOwnershipVocabularySet {
    /// Handler surface-family tokens.
    pub surface_families: Vec<String>,
    /// Handler-channel-class tokens.
    pub channel_classes: Vec<String>,
    /// Handler-owner-class tokens.
    pub owner_classes: Vec<String>,
    /// Handler-precedence-state tokens.
    pub precedence_states: Vec<String>,
    /// Handler-change-state tokens.
    pub change_states: Vec<String>,
    /// User-facing-impact tokens.
    pub impact_classes: Vec<String>,
    /// Channel-association-action tokens.
    pub association_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Deployment-mode tokens (reused from the frozen matrix).
    pub deployment_modes: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5HandlerOwnershipVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5HandlerSurfaceFamily::ALL, M5HandlerSurfaceFamily::as_str),
            channel_classes: tokens(&M5HandlerChannelClass::ALL, M5HandlerChannelClass::as_str),
            owner_classes: tokens(&M5HandlerOwnerClass::ALL, M5HandlerOwnerClass::as_str),
            precedence_states: tokens(
                &M5HandlerPrecedenceState::ALL,
                M5HandlerPrecedenceState::as_str,
            ),
            change_states: tokens(&M5HandlerChangeState::ALL, M5HandlerChangeState::as_str),
            impact_classes: tokens(&M5HandlerImpactClass::ALL, M5HandlerImpactClass::as_str),
            association_actions: tokens(
                &M5ChannelAssociationAction::ALL,
                M5ChannelAssociationAction::as_str,
            ),
            export_fields: tokens(
                &M5HandlerOwnershipExportField::ALL,
                M5HandlerOwnershipExportField::as_str,
            ),
            deployment_modes: tokens(&DEPLOYMENT_MODE_ALL, M5DeploymentMode::as_str),
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
pub struct M5HandlerOwnershipGovernanceReview {
    /// One primitive carries card / review-row / recovery truth on every surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Ownership identity is preserved across the card, the rows, and the recovery block.
    pub ownership_identity_preserved_across_surfaces: bool,
    /// The current owner and precedence are always disclosed.
    pub current_owner_and_precedence_always_disclosed: bool,
    /// Handler changes are always previewable and reversible, never silent.
    pub changes_previewable_and_reversible_never_silent: bool,
    /// Recovery paths always align with the owner and carry the rollback identity.
    pub recovery_aligned_with_owner_and_rollback_identity: bool,
    /// The support / export packet reconstructs ownership truth.
    pub support_export_reconstructs_ownership: bool,
    /// Later M5 rows cannot invent parallel handler-ownership vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandlerOwnershipConsumerProjection {
    /// About / diagnostics / install-review / support surfaces all consume the shared primitive.
    pub integration_surfaces_consume_shared_primitive: bool,
    /// The ownership resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The disclosure card reads a single canonical ownership source.
    pub disclosure_card_reads_single_ownership_source: bool,
    /// Support / export reads a single canonical ownership source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the handler-ownership primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandlerOwnershipReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting deployment audit.
    pub deployment_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5HandlerOwnershipPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5HandlerOwnershipPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5HandlerOwnershipSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5HandlerOwnershipVocabularySet,
    /// Governance-review block.
    pub governance_review: M5HandlerOwnershipGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5HandlerOwnershipConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5HandlerOwnershipReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 handler-ownership primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandlerOwnershipPrimitivePacket {
    /// Record kind; must equal [`M5_HANDLER_OWNERSHIP_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_HANDLER_OWNERSHIP_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5HandlerOwnershipSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5HandlerOwnershipVocabularySet,
    /// Governance-review block.
    pub governance_review: M5HandlerOwnershipGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5HandlerOwnershipConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5HandlerOwnershipReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5HandlerOwnershipPrimitivePacket {
    /// Builds an M5 handler-ownership primitive packet from stable-lane input.
    pub fn new(input: M5HandlerOwnershipPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_HANDLER_OWNERSHIP_RECORD_KIND.to_owned(),
            schema_version: M5_HANDLER_OWNERSHIP_SCHEMA_VERSION,
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

    /// Validates the M5 handler-ownership primitive invariants.
    pub fn validate(&self) -> Vec<M5HandlerOwnershipViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_HANDLER_OWNERSHIP_RECORD_KIND {
            violations.push(M5HandlerOwnershipViolation::WrongRecordKind);
        }
        if self.schema_version != M5_HANDLER_OWNERSHIP_SCHEMA_VERSION {
            violations.push(M5HandlerOwnershipViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5HandlerOwnershipViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 handler-ownership primitive packet serializes"),
        ) {
            violations.push(M5HandlerOwnershipViolation::RawMaterialInExport);
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
            .expect("m5 handler-ownership primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,owner,channel_classes,precedence_states,export_fields,association_rows,example_count\n",
        );
        for row in &self.surface_rows {
            let association_rows: usize = row
                .example_cases
                .iter()
                .map(|case| case.resolved.association_rows.len())
                .sum();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.channel_classes, |v| v.as_str()),
                join_tokens(&row.precedence_states, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                association_rows,
                row.example_cases.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Handler-Ownership Primitive: Ownership / Precedence Disclosure Card, Channel-Association Review Rows, and Recovery Alignment\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Handler surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5HandlerSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Channel classes: {}\n",
            self.vocabulary_set.channel_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Precedence states: {}\n",
            self.vocabulary_set.precedence_states.join(", ")
        ));
        out.push_str(&format!(
            "- Association actions: {}\n",
            self.vocabulary_set.association_actions.join(", ")
        ));
        out.push_str("\n## Handler surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!("  - Worked cases: {}\n", row.example_cases.len()));
            for case in &row.example_cases {
                out.push_str(&format!(
                    "    - `{}` → owner `{}` ({}), {} channels, {} recovery paths\n",
                    case.resolved.ownership_id,
                    case.resolved.disclosure_card.owner_class.as_str(),
                    case.resolved.disclosure_card.precedence_state.as_str(),
                    case.resolved.association_rows.len(),
                    case.resolved.recovery_alignment.recovery_paths.len(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 handler-ownership export.
#[derive(Debug)]
pub enum M5HandlerOwnershipArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5HandlerOwnershipViolation>),
}

impl fmt::Display for M5HandlerOwnershipArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 handler-ownership primitive export parse failed: {error}"
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
                    "m5 handler-ownership primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5HandlerOwnershipArtifactError {}

/// Validation failures emitted by [`M5HandlerOwnershipPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5HandlerOwnershipViolation {
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
    /// A required handler surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row declares no channel classes.
    ChannelClassMissing,
    /// A surface row declares no precedence states.
    PrecedenceStateMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked ownership cases.
    ExampleCasesMissing,
    /// A worked ownership case does not match a fresh resolve of its input.
    ExampleCaseDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves a side-by-side install disclosing which build owns and why (AC1).
    OwnershipDisclosureUnproven,
    /// No worked case proves a handler change previewable and reversible (AC2).
    ChangePreviewabilityUnproven,
    /// No worked case proves ownership / precedence truth preserved with aligned recovery (AC3).
    OwnershipPrecedencePreservationUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5HandlerOwnershipViolation {
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
            Self::ChannelClassMissing => "channel_class_missing",
            Self::PrecedenceStateMissing => "precedence_state_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleCasesMissing => "example_cases_missing",
            Self::ExampleCaseDrift => "example_case_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::OwnershipDisclosureUnproven => "ownership_disclosure_unproven",
            Self::ChangePreviewabilityUnproven => "change_previewability_unproven",
            Self::OwnershipPrecedencePreservationUnproven => {
                "ownership_precedence_preservation_unproven"
            }
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 handler-ownership export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_stable_m5_handler_ownership_export(
) -> Result<M5HandlerOwnershipPrimitivePacket, M5HandlerOwnershipArtifactError> {
    let packet: M5HandlerOwnershipPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-handler-ownership-primitive-proof/support_export.json"
    )))
    .map_err(M5HandlerOwnershipArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5HandlerOwnershipArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5HandlerOwnershipPrimitivePacket,
    violations: &mut Vec<M5HandlerOwnershipViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_HANDLER_OWNERSHIP_SCHEMA_REF,
        M5_HANDLER_OWNERSHIP_DOC_REF,
        M5_HANDLER_OWNERSHIP_COMPONENT_MATRIX_REF,
        M5_HANDLER_OWNERSHIP_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5HandlerOwnershipViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5HandlerOwnershipPrimitivePacket,
    violations: &mut Vec<M5HandlerOwnershipViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5HandlerOwnershipViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5HandlerOwnershipPrimitivePacket,
    violations: &mut Vec<M5HandlerOwnershipViolation>,
) {
    let present: BTreeSet<M5HandlerSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5HandlerSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5HandlerOwnershipViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5HandlerOwnershipViolation::SurfaceRowIncomplete);
        }
        if row.channel_classes.is_empty() {
            violations.push(M5HandlerOwnershipViolation::ChannelClassMissing);
        }
        if row.precedence_states.is_empty() {
            violations.push(M5HandlerOwnershipViolation::PrecedenceStateMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5HandlerOwnershipViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5HandlerOwnershipViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5HandlerOwnershipViolation::ConsumerSurfacesMissing);
        }
        if row.example_cases.is_empty() {
            violations.push(M5HandlerOwnershipViolation::ExampleCasesMissing);
        }
        if row.example_cases.iter().any(|case| !case.is_self_consistent()) {
            violations.push(M5HandlerOwnershipViolation::ExampleCaseDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5HandlerOwnershipViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated by at least one worked case across the
/// matrix: a side-by-side install disclosing which build owns and why (AC1), a handler change
/// previewable and reversible (AC2), and ownership / precedence truth preserved with recovery
/// aligned to the owner and carrying the rollback identity (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5HandlerOwnershipPrimitivePacket,
    violations: &mut Vec<M5HandlerOwnershipViolation>,
) {
    let cases: Vec<&M5ResolvedHandlerOwnership> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_cases.iter().map(|case| &case.resolved))
        .collect();

    // AC1: at least one case is a genuine side-by-side story (precedence names more than one
    // install) whose card still discloses the owner and reason, at least two distinct precedence
    // states appear across the matrix, and every case keeps its owner / precedence disclosed and
    // identity consistent.
    let distinct_precedence: BTreeSet<M5HandlerPrecedenceState> = cases
        .iter()
        .map(|resolved| resolved.disclosure_card.precedence_state)
        .collect();
    let disclosure_proven = cases.iter().any(|resolved| {
        resolved.has_contested_or_multi_install()
            && resolved.disclosure_card.discloses_current_owner
            && !resolved.disclosure_card.ownership_reason.trim().is_empty()
    }) && distinct_precedence.len() >= 2
        && cases.iter().all(|resolved| {
            resolved.identity_consistent() && resolved.owner_and_precedence_disclosed()
        });
    if !disclosure_proven {
        violations.push(M5HandlerOwnershipViolation::OwnershipDisclosureUnproven);
    }

    // AC2: at least one case proposes an actual handler change that is previewable and
    // reversible with bounded keep / reassign / cancel actions, and every case keeps its changes
    // previewable and reversible.
    let preview_proven = cases.iter().any(|resolved| {
        resolved.has_proposed_change()
            && resolved.association_rows.iter().any(|row| {
                row.change_state.is_change()
                    && row.previewable
                    && row.reversible
                    && row.actions.contains(&M5ChannelAssociationAction::Keep)
                    && row.actions.contains(&M5ChannelAssociationAction::Reassign)
                    && row.actions.contains(&M5ChannelAssociationAction::Cancel)
            })
    }) && cases
        .iter()
        .all(|resolved| resolved.changes_previewable_and_reversible());
    if !preview_proven {
        violations.push(M5HandlerOwnershipViolation::ChangePreviewabilityUnproven);
    }

    // AC3: at least one case carries recovery paths that all carry the rollback identity, and
    // every case preserves ownership / precedence truth with aligned recovery.
    let preservation_proven = cases.iter().any(|resolved| {
        resolved.has_recovery_path()
            && resolved
                .recovery_alignment
                .recovery_paths
                .iter()
                .all(|path| path.carries_rollback_identity && path.aligned_with_channel_owner)
    }) && cases.iter().all(|resolved| {
        resolved.ownership_precedence_preserved_in_export()
            && resolved.recovery_alignment.all_paths_aligned_with_owner
            && resolved.recovery_alignment.all_paths_carry_rollback_identity
    });
    if !preservation_proven {
        violations.push(M5HandlerOwnershipViolation::OwnershipPrecedencePreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5HandlerOwnershipPrimitivePacket,
    violations: &mut Vec<M5HandlerOwnershipViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.ownership_identity_preserved_across_surfaces,
        review.current_owner_and_precedence_always_disclosed,
        review.changes_previewable_and_reversible_never_silent,
        review.recovery_aligned_with_owner_and_rollback_identity,
        review.support_export_reconstructs_ownership,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5HandlerOwnershipViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5HandlerOwnershipPrimitivePacket,
    violations: &mut Vec<M5HandlerOwnershipViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.integration_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.disclosure_card_reads_single_ownership_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5HandlerOwnershipViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5HandlerOwnershipPrimitivePacket,
    violations: &mut Vec<M5HandlerOwnershipViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.deployment_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5HandlerOwnershipViolation::ReleasePostureIncomplete);
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
