//! Canonical M5 coexistence-and-fleet-rollout packet: one inspectable object that keeps stable,
//! preview, portable, mirror/offline, and managed install families from colliding and proves the
//! canary-to-LTS rollout rings are evidence-backed before publication.
//!
//! Where the install-and-portability governance matrix freezes the per-lane assurance gate and the
//! install-and-update diagnostics packet answers where each artifact lives, this packet answers the
//! coexistence-and-rollout question that sits between them: for each side-by-side install family —
//! the stable-broad install, the preview install running beside it, the portable package, the
//! managed-fleet install, and the mirror/air-gap install — how its durable state root is separated
//! ([`StateRootSeparation`]), what import choice it took on first run ([`ImportChoice`]), who owns
//! its update marker ([`UpdateMarkerOwnership`]), and who wins each file-association, deep-link, and
//! protocol-handler registration ([`HandlerPrecedenceClass`]). One [`CoexistenceLaneRow`] carries
//! all of that for one install family, and the same packet is the object release center, About,
//! CLI, diagnostics, support export, and admin docs render instead of divergent hand-written
//! coexistence claims.
//!
//! The coexistence gate keeps a rollout row from claiming support a topology cannot back. The
//! [`InstallAssurance`] a lane may publish is the weakest ceiling implied by its observed states —
//! an unverified binary, a shared or colliding state root, a contested update marker, a
//! last-writer-wins handler, or a governance lane that was itself narrowed all lower or withhold the
//! published support automatically. Each lane is pinned to the canonical governance lane it draws
//! verification truth from via [`CoexistenceFamily::governs_lane`], and `governs_assurance` is
//! validated against the embedded governance matrix, so an install family can never publish support
//! beyond the lane the governance gate already narrowed. This realizes the rollout invariant: a new
//! M5 channel cannot rely on last-writer-wins handler ownership or undocumented state-root sharing.
//!
//! Ring evidence is first-class. Each [`RolloutRingRow`] records, for one of the canary, pilot,
//! broad, and LTS rings, its rollout posture ([`RingPosture`]), the rollback target it can still
//! fall back to ([`RolloutRollbackState`]), and how fresh its rollout evidence is
//! ([`EvidenceFreshness`]), and publishes only the assurance its evidence backs. Mirror and air-gap
//! imports are reviewed through a [`MirrorImportRow`] that records its detached-signature
//! verification state ([`MirrorSignatureVerification`]) and freshness so an offline or managed-fleet
//! profile cannot import an unverified or stale package as if it were trusted.
//!
//! Troubleshooting drills are first-class too. Each [`RolloutDrill`] replays one rollout or
//! coexistence incident — a wrong-target launch, a last-writer-wins handler takeover, a stale
//! mirror, or a managed-package drift — and proves the object detects it, so a coexistence or
//! rollout failure is visible before publication rather than after a wrong-target launch.
//!
//! Because every required consumer surface — release center, Help/About, support export,
//! diagnostics, CLI, and admin docs — binds to this one packet via a [`RolloutConsumerBinding`]
//! that must ingest it, preserve its published support and recovery paths, and narrow with it, an
//! install family narrowed here cannot read as supported on a release evidence row, an About panel,
//! a CLI status line, an admin doc badge, or a support export.
//!
//! The packet is checked in at `artifacts/install/m5/m5-coexistence-and-fleet-rollout.json` and
//! embedded here. It is metadata-only: every field is a typed state or an opaque ref, and it carries
//! no credential bodies, raw provider payloads, or workspace contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_install_and_portability_governance::{
    current_m5_install_portability_governance_matrix, ChannelRing, InstallAssurance,
    InstallConfigLane, InstallMode, InstallVerification,
};

/// Supported M5 coexistence-and-fleet-rollout schema version.
pub const M5_COEXISTENCE_FLEET_ROLLOUT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_COEXISTENCE_FLEET_ROLLOUT_RECORD_KIND: &str = "m5_coexistence_and_fleet_rollout";

/// Repo-relative path to the checked-in packet.
pub const M5_COEXISTENCE_FLEET_ROLLOUT_PATH: &str =
    "artifacts/install/m5/m5-coexistence-and-fleet-rollout.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_COEXISTENCE_FLEET_ROLLOUT_SCHEMA_REF: &str =
    "schemas/install/m5-coexistence-and-fleet-rollout.schema.json";

/// Repo-relative path to the companion document.
pub const M5_COEXISTENCE_FLEET_ROLLOUT_DOC_REF: &str =
    "docs/install/m5/m5-coexistence-and-fleet-rollout.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_COEXISTENCE_FLEET_ROLLOUT_ARTIFACT_DOC_REF: &str =
    "artifacts/install/m5/m5-coexistence-and-fleet-rollout.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_COEXISTENCE_FLEET_ROLLOUT_FIXTURE_DIR: &str =
    "fixtures/install/m5/m5-coexistence-and-fleet-rollout";

/// Embedded checked-in packet JSON.
pub const M5_COEXISTENCE_FLEET_ROLLOUT_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/install/m5/m5-coexistence-and-fleet-rollout.json"
));

/// An M5 side-by-side install family the coexistence packet covers.
///
/// These are the install families that can land on one machine at the same time, so state-root
/// separation, import choices, update-marker ownership, and handler precedence stay inspectable for
/// each one rather than assuming one connected happy path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoexistenceFamily {
    /// The stable-channel broad install — the first-party local baseline.
    StableBroad,
    /// A preview install running side-by-side with stable.
    PreviewSideBySide,
    /// A portable install carrying its own durable state root.
    Portable,
    /// An organization-managed, policy-controlled fleet install.
    Managed,
    /// A mirror or air-gap install provisioned from offline media.
    MirrorOffline,
}

impl CoexistenceFamily {
    /// Every coexistence family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::StableBroad,
        Self::PreviewSideBySide,
        Self::Portable,
        Self::Managed,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableBroad => "stable_broad",
            Self::PreviewSideBySide => "preview_side_by_side",
            Self::Portable => "portable",
            Self::Managed => "managed",
            Self::MirrorOffline => "mirror_offline",
        }
    }

    /// The canonical governance lane this family draws its verification truth from.
    ///
    /// An install family never publishes support beyond the governance lane it is pinned to, so the
    /// coexistence object inherits the gate the governance matrix already applied.
    pub const fn governs_lane(self) -> InstallConfigLane {
        match self {
            Self::StableBroad => InstallConfigLane::DesktopStable,
            Self::PreviewSideBySide => InstallConfigLane::DesktopPreview,
            Self::Portable => InstallConfigLane::PortableInstall,
            Self::Managed | Self::MirrorOffline => InstallConfigLane::ManagedFleet,
        }
    }
}

/// How separated a coexisting install family's durable state root is from its siblings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateRootSeparation {
    /// A fully isolated state root owned by this install alone.
    Isolated,
    /// A namespaced state root under a shared parent; bounded.
    BoundedNamespaced,
    /// A state root linked to a sibling via an explicit import; needs retest.
    ImportLinked,
    /// A state root that shares a writable location with a sibling.
    Colliding,
}

impl StateRootSeparation {
    /// Every state-root-separation state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Isolated,
        Self::BoundedNamespaced,
        Self::ImportLinked,
        Self::Colliding,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Isolated => "isolated",
            Self::BoundedNamespaced => "bounded_namespaced",
            Self::ImportLinked => "import_linked",
            Self::Colliding => "colliding",
        }
    }

    /// Highest assurance this separation state permits a lane to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::Isolated => InstallAssurance::Verified,
            Self::BoundedNamespaced => InstallAssurance::Bounded,
            Self::ImportLinked => InstallAssurance::RetestPending,
            Self::Colliding => InstallAssurance::Withheld,
        }
    }

    /// Whether this state raises the [`CoexistenceNarrowReason::SharedStateRoot`] trigger.
    pub const fn is_shared_trigger(self) -> bool {
        !matches!(self, Self::Isolated)
    }
}

/// The first-run import decision a coexisting install family took.
///
/// This is disclosed so a side-by-side install can never silently inherit a sibling's state; it
/// does not itself gate the published support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportChoice {
    /// Start from fresh default state.
    FreshState,
    /// Take a one-time copy of a sibling's state.
    CopyFromSibling,
    /// Decline to import any sibling state.
    DeclineImport,
    /// Import state from a mirror or air-gap package.
    MirrorImport,
    /// Share a sibling's state root by link.
    LinkShared,
}

impl ImportChoice {
    /// Every import choice, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FreshState,
        Self::CopyFromSibling,
        Self::DeclineImport,
        Self::MirrorImport,
        Self::LinkShared,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshState => "fresh_state",
            Self::CopyFromSibling => "copy_from_sibling",
            Self::DeclineImport => "decline_import",
            Self::MirrorImport => "mirror_import",
            Self::LinkShared => "link_shared",
        }
    }
}

/// Who owns a coexisting install family's update marker.
///
/// A contested marker is the last-writer-wins failure mode this packet exists to forbid: two
/// installs writing one shared marker race to claim the update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateMarkerOwnership {
    /// This install exclusively owns its own update marker.
    ExclusiveOwned,
    /// A shared marker scoped by channel; bounded.
    ScopedShared,
    /// A contested marker resolved last-writer-wins.
    ContestedLastWriter,
}

impl UpdateMarkerOwnership {
    /// Every update-marker-ownership state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ExclusiveOwned,
        Self::ScopedShared,
        Self::ContestedLastWriter,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExclusiveOwned => "exclusive_owned",
            Self::ScopedShared => "scoped_shared",
            Self::ContestedLastWriter => "contested_last_writer",
        }
    }

    /// Highest assurance this marker-ownership state permits a lane to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::ExclusiveOwned => InstallAssurance::Verified,
            Self::ScopedShared => InstallAssurance::Bounded,
            Self::ContestedLastWriter => InstallAssurance::Withheld,
        }
    }

    /// Whether this state raises the [`CoexistenceNarrowReason::ContestedUpdateMarker`] trigger.
    pub const fn is_contested_trigger(self) -> bool {
        !matches!(self, Self::ExclusiveOwned)
    }
}

/// A registration surface whose handler precedence is recorded per install family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandlerSurface {
    /// A file-association registration.
    FileAssociation,
    /// A deep-link registration.
    DeepLink,
    /// A protocol-handler registration.
    ProtocolHandler,
}

impl HandlerSurface {
    /// Every handler surface a lane must record, in declaration order.
    pub const REQUIRED: [Self; 3] = [Self::FileAssociation, Self::DeepLink, Self::ProtocolHandler];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileAssociation => "file_association",
            Self::DeepLink => "deep_link",
            Self::ProtocolHandler => "protocol_handler",
        }
    }
}

/// How a coexisting install family wins or loses a handler registration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandlerPrecedenceClass {
    /// This install is the sole registered owner of the handler.
    SoleOwner,
    /// Multiple installs register, but precedence is explicitly declared; bounded.
    PrecedenceDeclared,
    /// The user arbitrates the handler at launch time; bounded.
    UserArbitrated,
    /// Whichever install registered last wins.
    LastWriterWins,
}

impl HandlerPrecedenceClass {
    /// Every handler-precedence class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SoleOwner,
        Self::PrecedenceDeclared,
        Self::UserArbitrated,
        Self::LastWriterWins,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SoleOwner => "sole_owner",
            Self::PrecedenceDeclared => "precedence_declared",
            Self::UserArbitrated => "user_arbitrated",
            Self::LastWriterWins => "last_writer_wins",
        }
    }

    /// Highest assurance this precedence class permits a lane to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::SoleOwner => InstallAssurance::Verified,
            Self::PrecedenceDeclared | Self::UserArbitrated => InstallAssurance::Bounded,
            Self::LastWriterWins => InstallAssurance::Withheld,
        }
    }

    /// Whether this state raises the [`CoexistenceNarrowReason::ContestedHandler`] trigger.
    pub const fn is_contested_trigger(self) -> bool {
        !matches!(self, Self::SoleOwner)
    }
}

/// A fleet-rollout ring that carries posture and rollback evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutRing {
    /// The earliest canary ring.
    Canary,
    /// The pilot ring following canary.
    Pilot,
    /// The broad ring serving the general channel.
    Broad,
    /// The long-term-support ring.
    Lts,
}

impl RolloutRing {
    /// Every rollout ring a packet must publish, in declaration order.
    pub const REQUIRED: [Self; 4] = [Self::Canary, Self::Pilot, Self::Broad, Self::Lts];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Canary => "canary",
            Self::Pilot => "pilot",
            Self::Broad => "broad",
            Self::Lts => "lts",
        }
    }
}

/// The rollout posture of a ring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RingPosture {
    /// The ring is fully promoted with current evidence.
    Promoted,
    /// The ring is soaking inside its evaluation window; bounded.
    Soaking,
    /// The ring is held pending evidence; needs retest.
    Held,
    /// The ring was rolled back.
    RolledBack,
}

impl RingPosture {
    /// Every ring posture, in declaration order.
    pub const ALL: [Self; 4] = [Self::Promoted, Self::Soaking, Self::Held, Self::RolledBack];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Promoted => "promoted",
            Self::Soaking => "soaking",
            Self::Held => "held",
            Self::RolledBack => "rolled_back",
        }
    }

    /// Highest assurance this posture permits a ring to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::Promoted => InstallAssurance::Verified,
            Self::Soaking => InstallAssurance::Bounded,
            Self::Held => InstallAssurance::RetestPending,
            Self::RolledBack => InstallAssurance::Withheld,
        }
    }
}

/// What rollback target a ring or lane can still fall back to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutRollbackState {
    /// A verified prior build is retained and applicable.
    Available,
    /// A prior build is retained but bounded to a slice; bounded.
    AvailableBounded,
    /// The retained prior build has expired; needs retest.
    Expired,
    /// No rollback target is retained.
    Missing,
}

impl RolloutRollbackState {
    /// Every rollback state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Available,
        Self::AvailableBounded,
        Self::Expired,
        Self::Missing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::AvailableBounded => "available_bounded",
            Self::Expired => "expired",
            Self::Missing => "missing",
        }
    }

    /// Highest assurance this rollback state permits a ring to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::Available => InstallAssurance::Verified,
            Self::AvailableBounded => InstallAssurance::Bounded,
            Self::Expired => InstallAssurance::RetestPending,
            Self::Missing => InstallAssurance::Withheld,
        }
    }
}

/// How fresh a ring's or import's rollout evidence is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// Evidence verified against the current build.
    Current,
    /// Evidence aging within tolerance; bounded.
    Aging,
    /// Evidence is stale; needs retest.
    Stale,
    /// No evidence is recorded.
    Missing,
}

impl EvidenceFreshness {
    /// Every evidence-freshness state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Current, Self::Aging, Self::Stale, Self::Missing];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Aging => "aging",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }

    /// Highest assurance this freshness permits a ring or import to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::Current => InstallAssurance::Verified,
            Self::Aging => InstallAssurance::Bounded,
            Self::Stale => InstallAssurance::RetestPending,
            Self::Missing => InstallAssurance::Withheld,
        }
    }
}

/// The source a mirror or air-gap import was provisioned from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorSource {
    /// A managed package mirror served inside an organization boundary.
    ManagedMirror,
    /// Air-gap media physically carried across the boundary.
    AirGapMedia,
}

impl MirrorSource {
    /// Every mirror source, in declaration order.
    pub const ALL: [Self; 2] = [Self::ManagedMirror, Self::AirGapMedia];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedMirror => "managed_mirror",
            Self::AirGapMedia => "air_gap_media",
        }
    }
}

/// The detached-signature verification state of a mirror or air-gap import.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorSignatureVerification {
    /// A detached signature was verified against the trusted key.
    DetachedSignatureVerified,
    /// Only a content checksum was confirmed; bounded.
    ChecksumOnly,
    /// The package was not verified.
    Unverified,
}

impl MirrorSignatureVerification {
    /// Every mirror-signature-verification state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::DetachedSignatureVerified,
        Self::ChecksumOnly,
        Self::Unverified,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DetachedSignatureVerified => "detached_signature_verified",
            Self::ChecksumOnly => "checksum_only",
            Self::Unverified => "unverified",
        }
    }

    /// Highest assurance this verification state permits an import to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::DetachedSignatureVerified => InstallAssurance::Verified,
            Self::ChecksumOnly => InstallAssurance::Bounded,
            Self::Unverified => InstallAssurance::Withheld,
        }
    }
}

/// The review state of a mirror or air-gap import.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorReviewState {
    /// The import was reviewed and admitted.
    Reviewed,
    /// The import is pending review; needs retest.
    PendingReview,
    /// The import was rejected.
    Rejected,
}

impl MirrorReviewState {
    /// Every mirror-review state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Reviewed, Self::PendingReview, Self::Rejected];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reviewed => "reviewed",
            Self::PendingReview => "pending_review",
            Self::Rejected => "rejected",
        }
    }

    /// Highest assurance this review state permits an import to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::Reviewed => InstallAssurance::Verified,
            Self::PendingReview => InstallAssurance::RetestPending,
            Self::Rejected => InstallAssurance::Withheld,
        }
    }
}

/// A headline reason the coexistence gate narrows a lane's published support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoexistenceNarrowReason {
    /// The install family's binary is not signed and verified.
    UnverifiedInstall,
    /// The install family's state root is not fully isolated.
    SharedStateRoot,
    /// The install family's update marker is not exclusively owned.
    ContestedUpdateMarker,
    /// A handler registration is not solely owned by this install.
    ContestedHandler,
    /// The governance lane this install family is pinned to was itself narrowed.
    GovernanceNarrowed,
}

impl CoexistenceNarrowReason {
    /// Every narrow reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::UnverifiedInstall,
        Self::SharedStateRoot,
        Self::ContestedUpdateMarker,
        Self::ContestedHandler,
        Self::GovernanceNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnverifiedInstall => "unverified_install",
            Self::SharedStateRoot => "shared_state_root",
            Self::ContestedUpdateMarker => "contested_update_marker",
            Self::ContestedHandler => "contested_handler",
            Self::GovernanceNarrowed => "governance_narrowed",
        }
    }
}

/// The recovery path surfaced when a lane's support is narrowed or withheld.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoexistenceRecoveryPath {
    /// Re-verify the install family's signature before widening trust.
    ReverifyInstall,
    /// Isolate the install family's state root from its siblings.
    IsolateStateRoot,
    /// Claim an exclusive update marker for the install family.
    ClaimUpdateMarker,
    /// Declare an explicit handler precedence for the install family.
    DeclareHandlerPrecedence,
    /// Follow the governance lane's own recovery path.
    FollowGovernanceRecovery,
    /// Withhold the install family's support claim from publication.
    WithholdClaim,
    /// No recovery is needed; only valid for a verified lane.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl CoexistenceRecoveryPath {
    /// Every recovery path, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ReverifyInstall,
        Self::IsolateStateRoot,
        Self::ClaimUpdateMarker,
        Self::DeclareHandlerPrecedence,
        Self::FollowGovernanceRecovery,
        Self::WithholdClaim,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReverifyInstall => "reverify_install",
            Self::IsolateStateRoot => "isolate_state_root",
            Self::ClaimUpdateMarker => "claim_update_marker",
            Self::DeclareHandlerPrecedence => "declare_handler_precedence",
            Self::FollowGovernanceRecovery => "follow_governance_recovery",
            Self::WithholdClaim => "withhold_claim",
            Self::NoneNeeded => "none",
        }
    }

    /// Whether this is a real recovery path the operator can take.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::NoneNeeded)
    }
}

/// A rollout or coexistence incident a troubleshooting drill replays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutIncident {
    /// A launch routed to the wrong install target.
    WrongTargetLaunch,
    /// A sibling install took over a handler last-writer-wins.
    HandlerTakeover,
    /// A mirror or air-gap import served stale state.
    StaleMirror,
    /// A managed package drifted from its policy-pinned build.
    ManagedPackageDrift,
}

impl RolloutIncident {
    /// Every incident a drill must cover, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::WrongTargetLaunch,
        Self::HandlerTakeover,
        Self::StaleMirror,
        Self::ManagedPackageDrift,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongTargetLaunch => "wrong_target_launch",
            Self::HandlerTakeover => "handler_takeover",
            Self::StaleMirror => "stale_mirror",
            Self::ManagedPackageDrift => "managed_package_drift",
        }
    }
}

/// A consumer surface that must ingest this packet and narrow with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutConsumer {
    /// The release-center rollout-evidence surface.
    ReleaseCenter,
    /// The Help/About panel.
    About,
    /// The support-export bundle.
    SupportExport,
    /// The install diagnostics surface.
    Diagnostics,
    /// The command-line install-and-status surface.
    Cli,
    /// The administrator documentation surface.
    AdminDocs,
}

impl RolloutConsumer {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::ReleaseCenter,
        Self::About,
        Self::SupportExport,
        Self::Diagnostics,
        Self::Cli,
        Self::AdminDocs,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::About => "about",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::Cli => "cli",
            Self::AdminDocs => "admin_docs",
        }
    }
}

/// One handler-precedence record carried by a coexistence lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandlerPrecedenceRow {
    /// Registration surface this record covers.
    pub surface: HandlerSurface,
    /// Opaque label naming the registered owner.
    pub registered_owner: String,
    /// How this install family wins or loses the registration.
    pub precedence: HandlerPrecedenceClass,
    /// Reviewer-facing precedence rule.
    pub precedence_rule: String,
}

/// One coexistence lane for an M5 side-by-side install family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoexistenceLaneRow {
    /// Stable lane id.
    pub lane_id: String,
    /// Install family this lane covers.
    pub family: CoexistenceFamily,
    /// Human-readable install family name.
    pub display_name: String,
    /// How this install family was installed.
    pub install_mode: InstallMode,
    /// Channel-and-ring this install family installs from.
    pub channel: ChannelRing,
    /// How separated this install family's durable state root is.
    pub state_root_separation: StateRootSeparation,
    /// Opaque, human-readable durable-state-root location label.
    pub state_root_label: String,
    /// First-run import decision this install family took.
    pub import_choice: ImportChoice,
    /// Who owns this install family's update marker.
    pub update_marker_ownership: UpdateMarkerOwnership,
    /// Handler-precedence records, one per required surface.
    pub handler_precedence: Vec<HandlerPrecedenceRow>,
    /// Last verification state of the install family's binary.
    pub install_verification: InstallVerification,
    /// Owner accountable for the install family's coexistence evidence.
    pub owner: String,
    /// Governance lane this install family draws verification truth from; must equal
    /// [`CoexistenceFamily::governs_lane`].
    pub governs_lane: InstallConfigLane,
    /// Published assurance of the governance lane, snapshotted from the governance matrix.
    pub governs_assurance: InstallAssurance,
    /// Support the install family's own evidence asserts, before the gate.
    pub declared_support: InstallAssurance,
    /// Support actually published after the gate narrows the install family.
    ///
    /// Must equal [`CoexistenceLaneRow::effective_support`].
    pub published_support: InstallAssurance,
    /// Headline narrow reasons; must equal the recomputed set.
    #[serde(default)]
    pub narrow_reasons: Vec<CoexistenceNarrowReason>,
    /// Recovery path surfaced when support is narrowed; must equal the recomputed path.
    pub recovery_path: CoexistenceRecoveryPath,
    /// Scope or slice labels this install family still backs.
    #[serde(default)]
    pub supported_scopes: Vec<String>,
    /// Caveats attached to the published support.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields whose evidence is shared, contested, or narrowing the support.
    #[serde(default)]
    pub stale_or_missing_fields: Vec<String>,
    /// Ref binding this lane into the coexistence surface.
    pub coexistence_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl CoexistenceLaneRow {
    /// The support label the install family's own evidence asserted, before narrowing.
    pub fn capability_floor(&self) -> InstallAssurance {
        self.declared_support
    }

    /// The weakest handler-precedence ceiling across the lane's handler records.
    pub fn worst_handler_ceiling(&self) -> InstallAssurance {
        self.handler_precedence
            .iter()
            .map(|h| h.precedence.assurance_ceiling())
            .fold(InstallAssurance::Verified, InstallAssurance::min)
    }

    /// The support label the gate permits this install family to publish.
    ///
    /// Lowers the capability floor to the weakest ceiling implied by the install verification,
    /// state-root separation, update-marker ownership, the weakest handler precedence, and the
    /// governance lane's published assurance, so an unverified binary, a shared state root, a
    /// contested update marker, a last-writer-wins handler, or a governance-narrowed lane can never
    /// publish a verified label.
    pub fn effective_support(&self) -> InstallAssurance {
        self.capability_floor()
            .min(self.install_verification.assurance_ceiling())
            .min(self.state_root_separation.assurance_ceiling())
            .min(self.update_marker_ownership.assurance_ceiling())
            .min(self.worst_handler_ceiling())
            .min(self.governs_assurance)
    }

    /// Whether any handler record is not solely owned by this install.
    pub fn has_contested_handler(&self) -> bool {
        self.handler_precedence
            .iter()
            .any(|h| h.precedence.is_contested_trigger())
    }

    /// The headline narrow reasons recomputed from the install family's observed states.
    pub fn computed_narrow_reasons(&self) -> Vec<CoexistenceNarrowReason> {
        let mut reasons = Vec::new();
        if self.install_verification.is_unverified_trigger() {
            reasons.push(CoexistenceNarrowReason::UnverifiedInstall);
        }
        if self.state_root_separation.is_shared_trigger() {
            reasons.push(CoexistenceNarrowReason::SharedStateRoot);
        }
        if self.update_marker_ownership.is_contested_trigger() {
            reasons.push(CoexistenceNarrowReason::ContestedUpdateMarker);
        }
        if self.has_contested_handler() {
            reasons.push(CoexistenceNarrowReason::ContestedHandler);
        }
        if self.governs_assurance != InstallAssurance::Verified {
            reasons.push(CoexistenceNarrowReason::GovernanceNarrowed);
        }
        reasons
    }

    /// The recovery path the gate surfaces for this install family.
    pub fn computed_recovery_path(&self) -> CoexistenceRecoveryPath {
        if self.effective_support() == InstallAssurance::Withheld {
            CoexistenceRecoveryPath::WithholdClaim
        } else if self.governs_assurance != InstallAssurance::Verified {
            CoexistenceRecoveryPath::FollowGovernanceRecovery
        } else if self.state_root_separation.is_shared_trigger() {
            CoexistenceRecoveryPath::IsolateStateRoot
        } else if self.update_marker_ownership.is_contested_trigger() {
            CoexistenceRecoveryPath::ClaimUpdateMarker
        } else if self.has_contested_handler() {
            CoexistenceRecoveryPath::DeclareHandlerPrecedence
        } else if self.install_verification.is_unverified_trigger() {
            CoexistenceRecoveryPath::ReverifyInstall
        } else {
            CoexistenceRecoveryPath::NoneNeeded
        }
    }

    /// Whether the install family publishes a clean verified label.
    pub fn is_verified(&self) -> bool {
        self.effective_support() == InstallAssurance::Verified
    }

    /// Whether the gate narrowed the published support below what the family declared.
    pub fn is_downgraded(&self) -> bool {
        self.effective_support().rank() < self.capability_floor().rank()
    }

    /// Whether the stored published support, narrow reasons, and recovery path all agree with the
    /// recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_support == self.effective_support()
            && self.narrow_reasons == self.computed_narrow_reasons()
            && self.recovery_path == self.computed_recovery_path()
    }
}

/// One fleet-rollout ring row carrying posture and rollback evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RolloutRingRow {
    /// Stable ring id.
    pub ring_id: String,
    /// Rollout ring this row covers.
    pub ring: RolloutRing,
    /// Coexistence family this ring rolls out.
    pub lane_ref: CoexistenceFamily,
    /// Rollout posture of the ring.
    pub posture: RingPosture,
    /// Rollback target the ring can fall back to.
    pub rollback_state: RolloutRollbackState,
    /// How fresh the ring's rollout evidence is.
    pub evidence_freshness: EvidenceFreshness,
    /// Opaque ref to the ring's rollout evidence.
    pub evidence_ref: String,
    /// Support the ring's own evidence asserts, before the gate.
    pub declared_support: InstallAssurance,
    /// Support actually published after the gate narrows the ring.
    ///
    /// Must equal [`RolloutRingRow::effective_support`].
    pub published_support: InstallAssurance,
    /// Caveats attached to the published support.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields whose evidence is narrowing the support.
    #[serde(default)]
    pub stale_or_missing_fields: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl RolloutRingRow {
    /// The support label the ring evidence asserted, before narrowing.
    pub fn capability_floor(&self) -> InstallAssurance {
        self.declared_support
    }

    /// The support label the gate permits this ring to publish.
    pub fn effective_support(&self) -> InstallAssurance {
        self.capability_floor()
            .min(self.posture.assurance_ceiling())
            .min(self.rollback_state.assurance_ceiling())
            .min(self.evidence_freshness.assurance_ceiling())
    }

    /// Whether the ring publishes a clean verified label.
    pub fn is_verified(&self) -> bool {
        self.effective_support() == InstallAssurance::Verified
    }

    /// Whether the gate narrowed the published support below what the ring declared.
    pub fn is_downgraded(&self) -> bool {
        self.effective_support().rank() < self.capability_floor().rank()
    }

    /// Whether the stored published support agrees with the recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_support == self.effective_support()
    }
}

/// One mirror or air-gap import review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MirrorImportRow {
    /// Stable import id.
    pub import_id: String,
    /// Source the import was provisioned from.
    pub source: MirrorSource,
    /// Coexistence family this import targets.
    pub target_family: CoexistenceFamily,
    /// Detached-signature verification state of the import.
    pub signature_verification: MirrorSignatureVerification,
    /// Opaque ref to the detached-signature evidence.
    pub signature_ref: String,
    /// How fresh the imported package is.
    pub freshness: EvidenceFreshness,
    /// Review state of the import.
    pub review_state: MirrorReviewState,
    /// Support the import's own evidence asserts, before the gate.
    pub declared_support: InstallAssurance,
    /// Support actually published after the gate narrows the import.
    ///
    /// Must equal [`MirrorImportRow::effective_support`].
    pub published_support: InstallAssurance,
    /// Caveats attached to the published support.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields whose evidence is narrowing the support.
    #[serde(default)]
    pub stale_or_missing_fields: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl MirrorImportRow {
    /// The support label the import evidence asserted, before narrowing.
    pub fn capability_floor(&self) -> InstallAssurance {
        self.declared_support
    }

    /// The support label the gate permits this import to publish.
    pub fn effective_support(&self) -> InstallAssurance {
        self.capability_floor()
            .min(self.signature_verification.assurance_ceiling())
            .min(self.freshness.assurance_ceiling())
            .min(self.review_state.assurance_ceiling())
    }

    /// Whether the import publishes a clean verified label.
    pub fn is_verified(&self) -> bool {
        self.effective_support() == InstallAssurance::Verified
    }

    /// Whether the gate narrowed the published support below what the import declared.
    pub fn is_downgraded(&self) -> bool {
        self.effective_support().rank() < self.capability_floor().rank()
    }

    /// Whether the stored published support agrees with the recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_support == self.effective_support()
    }
}

/// A troubleshooting drill that replays one rollout or coexistence incident and proves the object
/// detects it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RolloutDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Incident this drill replays.
    pub incident: RolloutIncident,
    /// Lane, ring, or import id the drill targets; must reference a claimed row.
    pub target_ref: String,
    /// Reviewer-facing scenario summary.
    pub scenario: String,
    /// Signal a healthy topology would emit.
    pub expected_signal: String,
    /// Signal the incident emits.
    pub observed_signal: String,
    /// Whether the object detects this incident; must be true.
    pub detected: bool,
    /// Resolution the drill proves.
    pub resolves_to: String,
}

/// One binding wiring a consumer surface to this packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RolloutConsumerBinding {
    /// Consumer surface this binding wires.
    pub consumer: RolloutConsumer,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Packet id this surface ingests.
    pub rollout_packet_id_ref: String,
    /// True when the surface ingests this packet rather than a parallel summary.
    pub ingests_packet: bool,
    /// True when the surface preserves the published support verbatim.
    pub preserves_published_support: bool,
    /// True when the surface preserves the recovery paths verbatim.
    pub preserves_recovery_paths: bool,
    /// True when the surface narrows automatically as lanes are downgraded.
    pub narrows_on_downgrade: bool,
    /// True when raw private material is excluded from the binding.
    pub raw_private_material_excluded: bool,
}

impl RolloutConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.rollout_packet_id_ref == packet_id
            && self.ingests_packet
            && self.preserves_published_support
            && self.preserves_recovery_paths
            && self.narrows_on_downgrade
            && self.raw_private_material_excluded
            && !self.binding_ref.trim().is_empty()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5CoexistenceFleetSummary {
    /// Total coexistence lanes.
    pub total_lanes: usize,
    /// Number of claimed install families.
    pub family_count: usize,
    /// Lanes published as verified.
    pub verified_lanes: usize,
    /// Lanes narrowed to a bounded label.
    pub bounded_lanes: usize,
    /// Lanes narrowed to a retest-pending label.
    pub retest_pending_lanes: usize,
    /// Lanes withheld from publication.
    pub withheld_lanes: usize,
    /// Lanes whose published support was downgraded below what they declared.
    pub downgraded_lanes: usize,
    /// Lanes whose state root is not fully isolated.
    pub shared_state_root_lanes: usize,
    /// Lanes whose update marker is not exclusively owned.
    pub contested_marker_lanes: usize,
    /// Lanes with a contested handler registration.
    pub contested_handler_lanes: usize,
    /// Lanes narrowed because their governance lane was narrowed.
    pub governance_narrowed_lanes: usize,
    /// Total handler-precedence records across all lanes.
    pub total_handler_rows: usize,
    /// Total rollout rings.
    pub total_rings: usize,
    /// Rings published as verified.
    pub verified_rings: usize,
    /// Rings whose published support was downgraded below what they declared.
    pub downgraded_rings: usize,
    /// Total mirror or air-gap imports.
    pub total_mirror_imports: usize,
    /// Imports whose detached signature was verified.
    pub detached_signature_verified_imports: usize,
    /// Total troubleshooting drills.
    pub drill_count: usize,
    /// Drills that detect their incident.
    pub detected_drill_count: usize,
}

/// A redaction-safe export row projected from a coexistence lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoexistenceFleetExportRow {
    /// Lane id.
    pub lane_id: String,
    /// Install-family token.
    pub family: String,
    /// Human-readable install family name.
    pub display_name: String,
    /// Install-mode token.
    pub install_mode: String,
    /// Channel-and-ring token.
    pub channel: String,
    /// State-root-separation token.
    pub state_root_separation: String,
    /// Import-choice token.
    pub import_choice: String,
    /// Update-marker-ownership token.
    pub update_marker_ownership: String,
    /// Governance-lane token.
    pub governs_lane: String,
    /// Governance published-assurance token.
    pub governs_assurance: String,
    /// Declared-support token.
    pub declared_support: String,
    /// Published-support token.
    pub published_support: String,
    /// Narrow-reason tokens.
    pub narrow_reasons: Vec<String>,
    /// Recovery-path token.
    pub recovery_path: String,
    /// Supported scope or slice labels.
    pub supported_scopes: Vec<String>,
    /// Caveats attached to the published support.
    pub caveats: Vec<String>,
    /// Whether the lane publishes a verified label.
    pub verified: bool,
    /// Whether the published support was downgraded below the declared support.
    pub downgraded: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet downstream surfaces render instead of restating
/// each install family's coexistence and rollout posture by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoexistenceFleetExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected lane rows.
    pub lanes: Vec<M5CoexistenceFleetExportRow>,
    /// Whether every lane's published support and recovery path agree with the gate.
    pub all_lanes_gate_consistent: bool,
    /// Lanes that publish a verified label.
    pub verified_count: usize,
    /// Lanes the gate narrowed below their declared support.
    pub downgraded_count: usize,
    /// Lanes the gate withheld entirely.
    pub withheld_count: usize,
}

/// The typed M5 coexistence-and-fleet-rollout packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5CoexistenceFleetRollout {
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
    /// Scheme the packet mints stable coexistence identities under.
    pub coexistence_identity_scheme: String,
    /// Governance matrix packet id this packet is bound to.
    pub governance_packet_id_ref: String,
    /// Claimed install families; one lane per family.
    pub families: Vec<CoexistenceFamily>,
    /// Closed install-mode vocabulary.
    pub install_modes: Vec<InstallMode>,
    /// Closed channel-and-ring vocabulary.
    pub channels: Vec<ChannelRing>,
    /// Closed state-root-separation vocabulary.
    pub state_root_separations: Vec<StateRootSeparation>,
    /// Closed import-choice vocabulary.
    pub import_choices: Vec<ImportChoice>,
    /// Closed update-marker-ownership vocabulary.
    pub update_marker_ownerships: Vec<UpdateMarkerOwnership>,
    /// Closed handler-surface vocabulary.
    pub handler_surfaces: Vec<HandlerSurface>,
    /// Closed handler-precedence-class vocabulary.
    pub handler_precedence_classes: Vec<HandlerPrecedenceClass>,
    /// Closed install-verification vocabulary.
    pub install_verification_states: Vec<InstallVerification>,
    /// Closed rollout-ring vocabulary.
    pub rollout_rings: Vec<RolloutRing>,
    /// Closed ring-posture vocabulary.
    pub ring_postures: Vec<RingPosture>,
    /// Closed rollback-state vocabulary.
    pub rollback_states: Vec<RolloutRollbackState>,
    /// Closed evidence-freshness vocabulary.
    pub evidence_freshness_states: Vec<EvidenceFreshness>,
    /// Closed mirror-source vocabulary.
    pub mirror_sources: Vec<MirrorSource>,
    /// Closed mirror-signature-verification vocabulary.
    pub mirror_signature_verifications: Vec<MirrorSignatureVerification>,
    /// Closed mirror-review vocabulary.
    pub mirror_review_states: Vec<MirrorReviewState>,
    /// Closed assurance-label vocabulary.
    pub assurance_labels: Vec<InstallAssurance>,
    /// Closed narrow-reason vocabulary.
    pub narrow_reasons: Vec<CoexistenceNarrowReason>,
    /// Closed recovery-path vocabulary.
    pub recovery_paths: Vec<CoexistenceRecoveryPath>,
    /// Closed incident vocabulary.
    pub incidents: Vec<RolloutIncident>,
    /// Closed consumer vocabulary.
    pub consumers: Vec<RolloutConsumer>,
    /// Coexistence lanes, one per claimed install family.
    #[serde(default)]
    pub lanes: Vec<CoexistenceLaneRow>,
    /// Rollout ring rows, one per required ring.
    #[serde(default)]
    pub rings: Vec<RolloutRingRow>,
    /// Mirror or air-gap import review rows.
    #[serde(default)]
    pub mirror_imports: Vec<MirrorImportRow>,
    /// Troubleshooting drills, one per required incident.
    #[serde(default)]
    pub drills: Vec<RolloutDrill>,
    /// Consumer bindings, one per required surface.
    #[serde(default)]
    pub consumer_bindings: Vec<RolloutConsumerBinding>,
    /// Summary counts.
    pub summary: M5CoexistenceFleetSummary,
}

impl M5CoexistenceFleetRollout {
    /// Returns the lane for a claimed install family.
    pub fn lane(&self, family: CoexistenceFamily) -> Option<&CoexistenceLaneRow> {
        self.lanes.iter().find(|l| l.family == family)
    }

    /// Returns the lane with the given stable lane id.
    pub fn lane_by_id(&self, lane_id: &str) -> Option<&CoexistenceLaneRow> {
        self.lanes.iter().find(|l| l.lane_id == lane_id)
    }

    /// Returns the ring row for a required ring.
    pub fn ring_row(&self, ring: RolloutRing) -> Option<&RolloutRingRow> {
        self.rings.iter().find(|r| r.ring == ring)
    }

    /// Lanes that publish a verified label.
    pub fn verified_lanes(&self) -> impl Iterator<Item = &CoexistenceLaneRow> {
        self.lanes.iter().filter(|l| l.is_verified())
    }

    /// Lanes the gate narrowed below their declared support.
    pub fn downgraded_lanes(&self) -> impl Iterator<Item = &CoexistenceLaneRow> {
        self.lanes.iter().filter(|l| l.is_downgraded())
    }

    /// Whether a consumer binding preserves this packet for the given surface.
    pub fn has_binding_for(&self, consumer: RolloutConsumer) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer == consumer && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every lane's stored support, reasons, and recovery path agree with the gate.
    pub fn all_lanes_gate_consistent(&self) -> bool {
        self.lanes.iter().all(|l| l.gate_consistent())
            && self.rings.iter().all(|r| r.gate_consistent())
            && self.mirror_imports.iter().all(|m| m.gate_consistent())
    }

    /// Every stable id the packet mints across lanes, rings, and imports.
    fn all_target_ids(&self) -> BTreeSet<String> {
        let mut ids = BTreeSet::new();
        ids.extend(self.lanes.iter().map(|l| l.lane_id.clone()));
        ids.extend(self.rings.iter().map(|r| r.ring_id.clone()));
        ids.extend(self.mirror_imports.iter().map(|m| m.import_id.clone()));
        ids
    }

    /// Recomputes the summary block from the rows, rings, imports, and drills.
    pub fn computed_summary(&self) -> M5CoexistenceFleetSummary {
        let count_lane = |label: InstallAssurance| {
            self.lanes
                .iter()
                .filter(|l| l.published_support == label)
                .count()
        };
        M5CoexistenceFleetSummary {
            total_lanes: self.lanes.len(),
            family_count: self.families.len(),
            verified_lanes: count_lane(InstallAssurance::Verified),
            bounded_lanes: count_lane(InstallAssurance::Bounded),
            retest_pending_lanes: count_lane(InstallAssurance::RetestPending),
            withheld_lanes: count_lane(InstallAssurance::Withheld),
            downgraded_lanes: self.lanes.iter().filter(|l| l.is_downgraded()).count(),
            shared_state_root_lanes: self
                .lanes
                .iter()
                .filter(|l| l.state_root_separation.is_shared_trigger())
                .count(),
            contested_marker_lanes: self
                .lanes
                .iter()
                .filter(|l| l.update_marker_ownership.is_contested_trigger())
                .count(),
            contested_handler_lanes: self
                .lanes
                .iter()
                .filter(|l| l.has_contested_handler())
                .count(),
            governance_narrowed_lanes: self
                .lanes
                .iter()
                .filter(|l| l.governs_assurance != InstallAssurance::Verified)
                .count(),
            total_handler_rows: self.lanes.iter().map(|l| l.handler_precedence.len()).sum(),
            total_rings: self.rings.len(),
            verified_rings: self.rings.iter().filter(|r| r.is_verified()).count(),
            downgraded_rings: self.rings.iter().filter(|r| r.is_downgraded()).count(),
            total_mirror_imports: self.mirror_imports.len(),
            detached_signature_verified_imports: self
                .mirror_imports
                .iter()
                .filter(|m| {
                    m.signature_verification
                        == MirrorSignatureVerification::DetachedSignatureVerified
                })
                .count(),
            drill_count: self.drills.len(),
            detected_drill_count: self.drills.iter().filter(|d| d.detected).count(),
        }
    }

    /// Produces the coexistence index downstream surfaces render instead of restating each install
    /// family's coexistence and rollout posture by hand.
    pub fn export_projection(&self) -> M5CoexistenceFleetExportProjection {
        let lanes = self
            .lanes
            .iter()
            .map(|l| M5CoexistenceFleetExportRow {
                lane_id: l.lane_id.clone(),
                family: l.family.as_str().to_owned(),
                display_name: l.display_name.clone(),
                install_mode: l.install_mode.as_str().to_owned(),
                channel: l.channel.as_str().to_owned(),
                state_root_separation: l.state_root_separation.as_str().to_owned(),
                import_choice: l.import_choice.as_str().to_owned(),
                update_marker_ownership: l.update_marker_ownership.as_str().to_owned(),
                governs_lane: l.governs_lane.as_str().to_owned(),
                governs_assurance: l.governs_assurance.as_str().to_owned(),
                declared_support: l.declared_support.as_str().to_owned(),
                published_support: l.published_support.as_str().to_owned(),
                narrow_reasons: l
                    .narrow_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                recovery_path: l.recovery_path.as_str().to_owned(),
                supported_scopes: l.supported_scopes.clone(),
                caveats: l.caveats.clone(),
                verified: l.is_verified(),
                downgraded: l.is_downgraded(),
                summary: format!(
                    "{} ({} / {}): state root {}, import {}, marker {}, governs {} ({}), declared {}, published {}, recovery {}",
                    l.family.as_str(),
                    l.install_mode.as_str(),
                    l.channel.as_str(),
                    l.state_root_separation.as_str(),
                    l.import_choice.as_str(),
                    l.update_marker_ownership.as_str(),
                    l.governs_lane.as_str(),
                    l.governs_assurance.as_str(),
                    l.declared_support.as_str(),
                    l.published_support.as_str(),
                    l.recovery_path.as_str()
                ),
            })
            .collect();
        M5CoexistenceFleetExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            lanes,
            all_lanes_gate_consistent: self.all_lanes_gate_consistent(),
            verified_count: self.verified_lanes().count(),
            downgraded_count: self.downgraded_lanes().count(),
            withheld_count: self
                .lanes
                .iter()
                .filter(|l| l.published_support == InstallAssurance::Withheld)
                .count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5CoexistenceFleetSupportExport {
        M5CoexistenceFleetSupportExport {
            record_kind: M5_COEXISTENCE_FLEET_ROLLOUT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_COEXISTENCE_FLEET_ROLLOUT_SCHEMA_VERSION,
            export_id: export_id.into(),
            rollout_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            rollout: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5CoexistenceFleetViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        // Bind to the governance matrix: the governance packet id and each lane's snapshotted
        // governance assurance must match the embedded governance matrix.
        let governance = current_m5_install_portability_governance_matrix().ok();
        match &governance {
            Some(matrix) => {
                if self.governance_packet_id_ref != matrix.packet_id {
                    violations.push(M5CoexistenceFleetViolation::GovernancePacketMismatch {
                        actual: self.governance_packet_id_ref.clone(),
                        expected: matrix.packet_id.clone(),
                    });
                }
            }
            None => violations.push(M5CoexistenceFleetViolation::GovernanceMatrixUnavailable),
        }

        let claimed: BTreeSet<CoexistenceFamily> = self.families.iter().copied().collect();
        let mut seen_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for lane in &self.lanes {
            if !seen_ids.insert(lane.lane_id.clone()) {
                violations.push(M5CoexistenceFleetViolation::DuplicateLaneId {
                    lane_id: lane.lane_id.clone(),
                });
            }
            if !seen_families.insert(lane.family) {
                violations.push(M5CoexistenceFleetViolation::DuplicateLane {
                    family: lane.family.as_str(),
                });
            }
            if !claimed.contains(&lane.family) {
                violations.push(M5CoexistenceFleetViolation::UnclaimedLane {
                    lane_id: lane.lane_id.clone(),
                    family: lane.family.as_str(),
                });
            }
            self.validate_lane(lane, governance.as_ref(), &mut violations);
        }
        for &family in &self.families {
            if !seen_families.contains(&family) {
                violations.push(M5CoexistenceFleetViolation::MissingLane {
                    family: family.as_str(),
                });
            }
        }

        self.validate_rings(&claimed, &mut violations);
        self.validate_mirror_imports(&claimed, &mut violations);

        let target_ids = self.all_target_ids();
        self.validate_drills(&target_ids, &mut violations);

        for consumer in RolloutConsumer::REQUIRED {
            if !self.has_binding_for(consumer) {
                violations.push(M5CoexistenceFleetViolation::MissingConsumerBinding {
                    consumer: consumer.as_str(),
                });
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(M5CoexistenceFleetViolation::ConsumerBindingDrift {
                    binding_ref: binding.binding_ref.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5CoexistenceFleetViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5CoexistenceFleetViolation>) {
        if self.schema_version != M5_COEXISTENCE_FLEET_ROLLOUT_SCHEMA_VERSION {
            violations.push(M5CoexistenceFleetViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_COEXISTENCE_FLEET_ROLLOUT_RECORD_KIND {
            violations.push(M5CoexistenceFleetViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            (
                "coexistence_identity_scheme",
                &self.coexistence_identity_scheme,
            ),
            ("governance_packet_id_ref", &self.governance_packet_id_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5CoexistenceFleetViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            ("families", self.families == CoexistenceFamily::ALL.to_vec()),
            (
                "install_modes",
                self.install_modes == InstallMode::ALL.to_vec(),
            ),
            ("channels", self.channels == ChannelRing::ALL.to_vec()),
            (
                "state_root_separations",
                self.state_root_separations == StateRootSeparation::ALL.to_vec(),
            ),
            (
                "import_choices",
                self.import_choices == ImportChoice::ALL.to_vec(),
            ),
            (
                "update_marker_ownerships",
                self.update_marker_ownerships == UpdateMarkerOwnership::ALL.to_vec(),
            ),
            (
                "handler_surfaces",
                self.handler_surfaces == HandlerSurface::REQUIRED.to_vec(),
            ),
            (
                "handler_precedence_classes",
                self.handler_precedence_classes == HandlerPrecedenceClass::ALL.to_vec(),
            ),
            (
                "install_verification_states",
                self.install_verification_states == InstallVerification::ALL.to_vec(),
            ),
            (
                "rollout_rings",
                self.rollout_rings == RolloutRing::REQUIRED.to_vec(),
            ),
            (
                "ring_postures",
                self.ring_postures == RingPosture::ALL.to_vec(),
            ),
            (
                "rollback_states",
                self.rollback_states == RolloutRollbackState::ALL.to_vec(),
            ),
            (
                "evidence_freshness_states",
                self.evidence_freshness_states == EvidenceFreshness::ALL.to_vec(),
            ),
            (
                "mirror_sources",
                self.mirror_sources == MirrorSource::ALL.to_vec(),
            ),
            (
                "mirror_signature_verifications",
                self.mirror_signature_verifications == MirrorSignatureVerification::ALL.to_vec(),
            ),
            (
                "mirror_review_states",
                self.mirror_review_states == MirrorReviewState::ALL.to_vec(),
            ),
            (
                "assurance_labels",
                self.assurance_labels == InstallAssurance::ALL.to_vec(),
            ),
            (
                "narrow_reasons",
                self.narrow_reasons == CoexistenceNarrowReason::ALL.to_vec(),
            ),
            (
                "recovery_paths",
                self.recovery_paths == CoexistenceRecoveryPath::ALL.to_vec(),
            ),
            (
                "incidents",
                self.incidents == RolloutIncident::REQUIRED.to_vec(),
            ),
            (
                "consumers",
                self.consumers == RolloutConsumer::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5CoexistenceFleetViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_lane(
        &self,
        lane: &CoexistenceLaneRow,
        governance: Option<
            &crate::m5_install_and_portability_governance::M5InstallPortabilityGovernanceMatrix,
        >,
        violations: &mut Vec<M5CoexistenceFleetViolation>,
    ) {
        for (field, value) in [
            ("lane_id", &lane.lane_id),
            ("display_name", &lane.display_name),
            ("state_root_label", &lane.state_root_label),
            ("owner", &lane.owner),
            ("coexistence_ref", &lane.coexistence_ref),
            ("note", &lane.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5CoexistenceFleetViolation::EmptyField {
                    id: lane.lane_id.clone(),
                    field_name: field,
                });
            }
        }

        // Each lane is pinned to the canonical governance lane it draws verification truth from.
        if lane.governs_lane != lane.family.governs_lane() {
            violations.push(M5CoexistenceFleetViolation::GovernsLaneMismatch {
                lane_id: lane.lane_id.clone(),
                expected: lane.family.governs_lane().as_str(),
            });
        }

        // The snapshotted governance assurance must equal the lane's published assurance in the
        // embedded governance matrix.
        if let Some(matrix) = governance {
            match matrix.lane_row(lane.governs_lane) {
                Some(gov_row) if gov_row.published_assurance != lane.governs_assurance => {
                    violations.push(M5CoexistenceFleetViolation::GovernsAssuranceMismatch {
                        lane_id: lane.lane_id.clone(),
                        recorded: lane.governs_assurance.as_str(),
                        governed: gov_row.published_assurance.as_str(),
                    });
                }
                Some(_) => {}
                None => violations.push(M5CoexistenceFleetViolation::GovernsLaneNotInMatrix {
                    lane_id: lane.lane_id.clone(),
                    lane: lane.governs_lane.as_str(),
                }),
            }
        }

        // Every required handler surface must be recorded exactly once.
        let mut seen_surfaces = BTreeSet::new();
        for row in &lane.handler_precedence {
            if row.registered_owner.trim().is_empty() {
                violations.push(M5CoexistenceFleetViolation::EmptyField {
                    id: lane.lane_id.clone(),
                    field_name: "registered_owner",
                });
            }
            if row.precedence_rule.trim().is_empty() {
                violations.push(M5CoexistenceFleetViolation::EmptyField {
                    id: lane.lane_id.clone(),
                    field_name: "precedence_rule",
                });
            }
            if !seen_surfaces.insert(row.surface) {
                violations.push(M5CoexistenceFleetViolation::DuplicateHandlerSurface {
                    lane_id: lane.lane_id.clone(),
                    surface: row.surface.as_str(),
                });
            }
        }
        for surface in HandlerSurface::REQUIRED {
            if !seen_surfaces.contains(&surface) {
                violations.push(M5CoexistenceFleetViolation::MissingHandlerSurface {
                    lane_id: lane.lane_id.clone(),
                    surface: surface.as_str(),
                });
            }
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &lane.narrow_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5CoexistenceFleetViolation::DuplicateNarrowReason {
                    lane_id: lane.lane_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // The published support must equal the gate's recomputed ceiling.
        let effective = lane.effective_support();
        if lane.published_support != effective {
            violations.push(M5CoexistenceFleetViolation::OverstatedSupport {
                lane_id: lane.lane_id.clone(),
                published: lane.published_support.as_str(),
                computed: effective.as_str(),
            });
        }
        if lane.narrow_reasons != lane.computed_narrow_reasons() {
            violations.push(M5CoexistenceFleetViolation::NarrowReasonsMismatch {
                lane_id: lane.lane_id.clone(),
            });
        }
        if lane.recovery_path != lane.computed_recovery_path() {
            violations.push(M5CoexistenceFleetViolation::RecoveryPathMismatch {
                lane_id: lane.lane_id.clone(),
            });
        }

        // A narrowed lane must offer a real recovery path, list at least one caveat, and name what
        // is shared or contested.
        if lane.is_downgraded() {
            if !lane.recovery_path.is_offered() {
                violations.push(M5CoexistenceFleetViolation::MissingRecoveryPath {
                    lane_id: lane.lane_id.clone(),
                });
            }
            if lane.caveats.is_empty() {
                violations.push(M5CoexistenceFleetViolation::EmptyField {
                    id: lane.lane_id.clone(),
                    field_name: "caveats",
                });
            }
            if lane.stale_or_missing_fields.is_empty() {
                violations.push(M5CoexistenceFleetViolation::EmptyField {
                    id: lane.lane_id.clone(),
                    field_name: "stale_or_missing_fields",
                });
            }
        }

        // A lane that still backs a publishable label must name at least one supported scope.
        if lane.published_support != InstallAssurance::Withheld && lane.supported_scopes.is_empty()
        {
            violations.push(M5CoexistenceFleetViolation::EmptyField {
                id: lane.lane_id.clone(),
                field_name: "supported_scopes",
            });
        }

        // A verified lane must be genuinely whole-trust: a signed binary, an isolated state root, an
        // exclusively owned update marker, sole-owner handlers, a verified governance lane, a
        // declared verified floor, no narrow reason, no caveat, no stale field, and a no-op recovery
        // path. This is the guardrail that forbids last-writer-wins or shared-root coexistence from
        // reading as verified.
        if lane.is_verified()
            && (lane.install_verification != InstallVerification::SignedVerified
                || lane.state_root_separation != StateRootSeparation::Isolated
                || lane.update_marker_ownership != UpdateMarkerOwnership::ExclusiveOwned
                || lane.has_contested_handler()
                || lane.governs_assurance != InstallAssurance::Verified
                || lane.capability_floor() != InstallAssurance::Verified
                || !lane.narrow_reasons.is_empty()
                || !lane.caveats.is_empty()
                || !lane.stale_or_missing_fields.is_empty()
                || lane.recovery_path.is_offered())
        {
            violations.push(M5CoexistenceFleetViolation::VerifiedLaneNotWhole {
                lane_id: lane.lane_id.clone(),
            });
        }
    }

    fn validate_rings(
        &self,
        claimed: &BTreeSet<CoexistenceFamily>,
        violations: &mut Vec<M5CoexistenceFleetViolation>,
    ) {
        let mut seen_ids = BTreeSet::new();
        let mut seen_rings = BTreeSet::new();
        for ring in &self.rings {
            if !seen_ids.insert(ring.ring_id.clone()) {
                violations.push(M5CoexistenceFleetViolation::DuplicateRingId {
                    ring_id: ring.ring_id.clone(),
                });
            }
            if !seen_rings.insert(ring.ring) {
                violations.push(M5CoexistenceFleetViolation::DuplicateRing {
                    ring: ring.ring.as_str(),
                });
            }
            for (field, value) in [
                ("ring_id", &ring.ring_id),
                ("evidence_ref", &ring.evidence_ref),
                ("note", &ring.note),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5CoexistenceFleetViolation::EmptyField {
                        id: ring.ring_id.clone(),
                        field_name: field,
                    });
                }
            }
            if !claimed.contains(&ring.lane_ref) {
                violations.push(M5CoexistenceFleetViolation::RingLaneUnknown {
                    ring_id: ring.ring_id.clone(),
                    family: ring.lane_ref.as_str(),
                });
            }
            let effective = ring.effective_support();
            if ring.published_support != effective {
                violations.push(M5CoexistenceFleetViolation::RingOverstatedSupport {
                    ring_id: ring.ring_id.clone(),
                    published: ring.published_support.as_str(),
                    computed: effective.as_str(),
                });
            }
            if ring.is_downgraded() && ring.caveats.is_empty() {
                violations.push(M5CoexistenceFleetViolation::EmptyField {
                    id: ring.ring_id.clone(),
                    field_name: "caveats",
                });
            }
            // A verified ring must be whole: promoted, with an available rollback target and current
            // evidence, declared verified, and carrying no caveat or stale field.
            if ring.is_verified()
                && (ring.posture != RingPosture::Promoted
                    || ring.rollback_state != RolloutRollbackState::Available
                    || ring.evidence_freshness != EvidenceFreshness::Current
                    || ring.capability_floor() != InstallAssurance::Verified
                    || !ring.caveats.is_empty()
                    || !ring.stale_or_missing_fields.is_empty())
            {
                violations.push(M5CoexistenceFleetViolation::VerifiedRingNotWhole {
                    ring_id: ring.ring_id.clone(),
                });
            }
        }
        for ring in RolloutRing::REQUIRED {
            if !seen_rings.contains(&ring) {
                violations.push(M5CoexistenceFleetViolation::MissingRing {
                    ring: ring.as_str(),
                });
            }
        }
    }

    fn validate_mirror_imports(
        &self,
        claimed: &BTreeSet<CoexistenceFamily>,
        violations: &mut Vec<M5CoexistenceFleetViolation>,
    ) {
        let mut seen_ids = BTreeSet::new();
        for import in &self.mirror_imports {
            if !seen_ids.insert(import.import_id.clone()) {
                violations.push(M5CoexistenceFleetViolation::DuplicateImportId {
                    import_id: import.import_id.clone(),
                });
            }
            for (field, value) in [
                ("import_id", &import.import_id),
                ("signature_ref", &import.signature_ref),
                ("note", &import.note),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5CoexistenceFleetViolation::EmptyField {
                        id: import.import_id.clone(),
                        field_name: field,
                    });
                }
            }
            if !claimed.contains(&import.target_family) {
                violations.push(M5CoexistenceFleetViolation::ImportTargetUnknown {
                    import_id: import.import_id.clone(),
                    family: import.target_family.as_str(),
                });
            }
            let effective = import.effective_support();
            if import.published_support != effective {
                violations.push(M5CoexistenceFleetViolation::ImportOverstatedSupport {
                    import_id: import.import_id.clone(),
                    published: import.published_support.as_str(),
                    computed: effective.as_str(),
                });
            }
            if import.is_downgraded() && import.caveats.is_empty() {
                violations.push(M5CoexistenceFleetViolation::EmptyField {
                    id: import.import_id.clone(),
                    field_name: "caveats",
                });
            }
        }
        // At least one mirror import must prove the detached-signature verification path.
        if !self.mirror_imports.iter().any(|m| {
            m.signature_verification == MirrorSignatureVerification::DetachedSignatureVerified
        }) {
            violations.push(M5CoexistenceFleetViolation::NoDetachedSignatureImport);
        }
    }

    fn validate_drills(
        &self,
        target_ids: &BTreeSet<String>,
        violations: &mut Vec<M5CoexistenceFleetViolation>,
    ) {
        let mut seen_ids = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for drill in &self.drills {
            if !seen_ids.insert(drill.drill_id.clone()) {
                violations.push(M5CoexistenceFleetViolation::DuplicateDrillId {
                    drill_id: drill.drill_id.clone(),
                });
            }
            covered.insert(drill.incident);
            for (field, value) in [
                ("drill_id", &drill.drill_id),
                ("scenario", &drill.scenario),
                ("expected_signal", &drill.expected_signal),
                ("observed_signal", &drill.observed_signal),
                ("resolves_to", &drill.resolves_to),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5CoexistenceFleetViolation::EmptyField {
                        id: drill.drill_id.clone(),
                        field_name: field,
                    });
                }
            }
            if !target_ids.contains(&drill.target_ref) {
                violations.push(M5CoexistenceFleetViolation::DrillTargetUnknown {
                    drill_id: drill.drill_id.clone(),
                    target_ref: drill.target_ref.clone(),
                });
            }
            if !drill.detected {
                violations.push(M5CoexistenceFleetViolation::DrillNotDetected {
                    drill_id: drill.drill_id.clone(),
                });
            }
        }
        for incident in RolloutIncident::REQUIRED {
            if !covered.contains(&incident) {
                violations.push(M5CoexistenceFleetViolation::MissingIncidentDrill {
                    incident: incident.as_str(),
                });
            }
        }
    }
}

/// A validation violation for the M5 coexistence-and-fleet-rollout packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5CoexistenceFleetViolation {
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
        /// Lane, ring, import, drill, or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// The packet binds to a governance packet id that is not the embedded matrix.
    GovernancePacketMismatch {
        /// Recorded governance packet id.
        actual: String,
        /// Expected governance packet id.
        expected: String,
    },
    /// The embedded governance matrix failed to load.
    GovernanceMatrixUnavailable,
    /// A lane id appears more than once.
    DuplicateLaneId {
        /// Duplicate lane id.
        lane_id: String,
    },
    /// A claimed install family carries more than one lane.
    DuplicateLane {
        /// Family token.
        family: &'static str,
    },
    /// A claimed install family has no lane.
    MissingLane {
        /// Family token.
        family: &'static str,
    },
    /// A lane covers a family the packet does not claim.
    UnclaimedLane {
        /// Lane id.
        lane_id: String,
        /// Family token.
        family: &'static str,
    },
    /// A lane's governance lane is not the canonical lane for its family.
    GovernsLaneMismatch {
        /// Lane id.
        lane_id: String,
        /// Expected lane token.
        expected: &'static str,
    },
    /// A lane's snapshotted governance assurance disagrees with the governance matrix.
    GovernsAssuranceMismatch {
        /// Lane id.
        lane_id: String,
        /// Recorded assurance token.
        recorded: &'static str,
        /// Governed assurance token.
        governed: &'static str,
    },
    /// A lane's governance lane is missing from the governance matrix.
    GovernsLaneNotInMatrix {
        /// Lane id.
        lane_id: String,
        /// Lane token.
        lane: &'static str,
    },
    /// A handler surface is recorded more than once on a lane.
    DuplicateHandlerSurface {
        /// Lane id.
        lane_id: String,
        /// Surface token.
        surface: &'static str,
    },
    /// A required handler surface is missing from a lane.
    MissingHandlerSurface {
        /// Lane id.
        lane_id: String,
        /// Surface token.
        surface: &'static str,
    },
    /// A lane lists a narrow reason more than once.
    DuplicateNarrowReason {
        /// Lane id.
        lane_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A lane publishes support beyond what its evidence supports.
    OverstatedSupport {
        /// Lane id.
        lane_id: String,
        /// Published support token.
        published: &'static str,
        /// Computed effective support token.
        computed: &'static str,
    },
    /// A lane's narrow reasons disagree with the recomputed reasons.
    NarrowReasonsMismatch {
        /// Lane id.
        lane_id: String,
    },
    /// A lane's recovery path disagrees with the recomputed path.
    RecoveryPathMismatch {
        /// Lane id.
        lane_id: String,
    },
    /// A narrowed lane offers no recovery path.
    MissingRecoveryPath {
        /// Lane id.
        lane_id: String,
    },
    /// A verified lane still narrows a state or carries a narrow reason.
    VerifiedLaneNotWhole {
        /// Lane id.
        lane_id: String,
    },
    /// A ring id appears more than once.
    DuplicateRingId {
        /// Duplicate ring id.
        ring_id: String,
    },
    /// A rollout ring is recorded more than once.
    DuplicateRing {
        /// Ring token.
        ring: &'static str,
    },
    /// A required rollout ring has no row.
    MissingRing {
        /// Ring token.
        ring: &'static str,
    },
    /// A ring references an install family the packet does not claim.
    RingLaneUnknown {
        /// Ring id.
        ring_id: String,
        /// Family token.
        family: &'static str,
    },
    /// A ring publishes support beyond what its evidence supports.
    RingOverstatedSupport {
        /// Ring id.
        ring_id: String,
        /// Published support token.
        published: &'static str,
        /// Computed effective support token.
        computed: &'static str,
    },
    /// A verified ring still narrows a state or carries a caveat.
    VerifiedRingNotWhole {
        /// Ring id.
        ring_id: String,
    },
    /// An import id appears more than once.
    DuplicateImportId {
        /// Duplicate import id.
        import_id: String,
    },
    /// An import references an install family the packet does not claim.
    ImportTargetUnknown {
        /// Import id.
        import_id: String,
        /// Family token.
        family: &'static str,
    },
    /// An import publishes support beyond what its evidence supports.
    ImportOverstatedSupport {
        /// Import id.
        import_id: String,
        /// Published support token.
        published: &'static str,
        /// Computed effective support token.
        computed: &'static str,
    },
    /// No mirror import proves the detached-signature verification path.
    NoDetachedSignatureImport,
    /// A drill id appears more than once.
    DuplicateDrillId {
        /// Duplicate drill id.
        drill_id: String,
    },
    /// A drill references a target id with no lane, ring, or import.
    DrillTargetUnknown {
        /// Drill id.
        drill_id: String,
        /// Referenced target id.
        target_ref: String,
    },
    /// A drill does not detect its incident.
    DrillNotDetected {
        /// Drill id.
        drill_id: String,
    },
    /// A required incident has no drill.
    MissingIncidentDrill {
        /// Incident token.
        incident: &'static str,
    },
    /// A required consumer surface has no binding.
    MissingConsumerBinding {
        /// Consumer token.
        consumer: &'static str,
    },
    /// A consumer binding drops or remints packet truth.
    ConsumerBindingDrift {
        /// Binding ref.
        binding_ref: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5CoexistenceFleetViolation {
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
            Self::GovernancePacketMismatch { actual, expected } => write!(
                f,
                "packet binds governance {actual} but the embedded matrix is {expected}"
            ),
            Self::GovernanceMatrixUnavailable => {
                write!(f, "embedded governance matrix could not be loaded")
            }
            Self::DuplicateLaneId { lane_id } => write!(f, "duplicate lane id {lane_id}"),
            Self::DuplicateLane { family } => {
                write!(f, "duplicate lane for install family {family}")
            }
            Self::MissingLane { family } => {
                write!(f, "missing lane for claimed install family {family}")
            }
            Self::UnclaimedLane { lane_id, family } => {
                write!(f, "lane {lane_id} covers unclaimed family {family}")
            }
            Self::GovernsLaneMismatch { lane_id, expected } => write!(
                f,
                "lane {lane_id} governs_lane must be the canonical lane {expected}"
            ),
            Self::GovernsAssuranceMismatch {
                lane_id,
                recorded,
                governed,
            } => write!(
                f,
                "lane {lane_id} records governs_assurance {recorded} but the matrix publishes {governed}"
            ),
            Self::GovernsLaneNotInMatrix { lane_id, lane } => {
                write!(f, "lane {lane_id} governs lane {lane} is not in the matrix")
            }
            Self::DuplicateHandlerSurface { lane_id, surface } => {
                write!(f, "lane {lane_id} repeats handler surface {surface}")
            }
            Self::MissingHandlerSurface { lane_id, surface } => {
                write!(f, "lane {lane_id} is missing handler surface {surface}")
            }
            Self::DuplicateNarrowReason { lane_id, reason } => {
                write!(f, "lane {lane_id} repeats narrow reason {reason}")
            }
            Self::OverstatedSupport {
                lane_id,
                published,
                computed,
            } => write!(
                f,
                "lane {lane_id} publishes support {published} but the gate computes {computed}"
            ),
            Self::NarrowReasonsMismatch { lane_id } => {
                write!(f, "lane {lane_id} narrow reasons disagree with the gate")
            }
            Self::RecoveryPathMismatch { lane_id } => {
                write!(f, "lane {lane_id} recovery path disagrees with the gate")
            }
            Self::MissingRecoveryPath { lane_id } => {
                write!(f, "lane {lane_id} is narrowed but offers no recovery path")
            }
            Self::VerifiedLaneNotWhole { lane_id } => write!(
                f,
                "lane {lane_id} is verified but narrows a state or carries a narrow reason"
            ),
            Self::DuplicateRingId { ring_id } => write!(f, "duplicate ring id {ring_id}"),
            Self::DuplicateRing { ring } => write!(f, "duplicate row for rollout ring {ring}"),
            Self::MissingRing { ring } => write!(f, "missing row for rollout ring {ring}"),
            Self::RingLaneUnknown { ring_id, family } => {
                write!(f, "ring {ring_id} references unknown family {family}")
            }
            Self::RingOverstatedSupport {
                ring_id,
                published,
                computed,
            } => write!(
                f,
                "ring {ring_id} publishes support {published} but the gate computes {computed}"
            ),
            Self::VerifiedRingNotWhole { ring_id } => write!(
                f,
                "ring {ring_id} is verified but narrows a state or carries a caveat"
            ),
            Self::DuplicateImportId { import_id } => write!(f, "duplicate import id {import_id}"),
            Self::ImportTargetUnknown { import_id, family } => {
                write!(f, "import {import_id} references unknown family {family}")
            }
            Self::ImportOverstatedSupport {
                import_id,
                published,
                computed,
            } => write!(
                f,
                "import {import_id} publishes support {published} but the gate computes {computed}"
            ),
            Self::NoDetachedSignatureImport => write!(
                f,
                "no mirror import proves the detached-signature verification path"
            ),
            Self::DuplicateDrillId { drill_id } => write!(f, "duplicate drill id {drill_id}"),
            Self::DrillTargetUnknown {
                drill_id,
                target_ref,
            } => write!(f, "drill {drill_id} references unknown target {target_ref}"),
            Self::DrillNotDetected { drill_id } => {
                write!(f, "drill {drill_id} does not detect its incident")
            }
            Self::MissingIncidentDrill { incident } => {
                write!(f, "missing drill for incident {incident}")
            }
            Self::MissingConsumerBinding { consumer } => {
                write!(f, "missing consumer binding for surface {consumer}")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(f, "binding {binding_ref} does not preserve packet truth")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the rows"),
        }
    }
}

impl Error for M5CoexistenceFleetViolation {}

/// Stable record-kind tag for [`M5CoexistenceFleetSupportExport`].
pub const M5_COEXISTENCE_FLEET_ROLLOUT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_coexistence_and_fleet_rollout_support_export";

/// Support-export wrapper preserving the packet verbatim for support and evidence packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoexistenceFleetSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub rollout_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact packet preserved by the export.
    pub rollout: M5CoexistenceFleetRollout,
}

impl M5CoexistenceFleetSupportExport {
    /// Whether the export preserves the same packet id and a clean packet.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_COEXISTENCE_FLEET_ROLLOUT_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_COEXISTENCE_FLEET_ROLLOUT_SCHEMA_VERSION
            && self.rollout_packet_id_ref == self.rollout.packet_id
            && self.raw_private_material_excluded
            && self.rollout.validate().is_empty()
    }
}

/// Loads the embedded M5 coexistence-and-fleet-rollout packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5CoexistenceFleetRollout`].
pub fn current_m5_coexistence_and_fleet_rollout(
) -> Result<M5CoexistenceFleetRollout, serde_json::Error> {
    serde_json::from_str(M5_COEXISTENCE_FLEET_ROLLOUT_JSON)
}

#[cfg(test)]
mod tests;
