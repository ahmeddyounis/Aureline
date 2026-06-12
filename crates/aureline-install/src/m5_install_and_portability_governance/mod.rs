//! Canonical M5 install-topology, configuration-portability, sync-device, and auth-recovery
//! governance matrix: the single qualification report that freezes the desktop-stable,
//! desktop-preview, portable-install, managed-fleet, marketplace-companion, CLI/headless, and
//! sync-device install/config/auth lanes into one non-inheriting governance gate.
//!
//! Each [`InstallConfigRow`] governs one M5 install/config/auth lane ([`InstallConfigLane`])
//! against the canonical install-truth packet it draws evidence from, and answers, for that
//! lane, who owns the evidence ([`InstallConfigRow::owner`]), how it is installed
//! ([`InstallMode`]), on which channel and ring ([`ChannelRing`]), where its durable state lives
//! ([`StateRootClass`]), how its state is carried across machines ([`PortableExportClass`]), how
//! its effective settings are scoped ([`EffectiveSettingScope`]), whether its binary is verified
//! ([`InstallVerification`]), whether its install topology is supported
//! ([`InstallTopologySupport`]), whether its portable/export state is fresh
//! ([`PortableStateFreshness`]), whether its device participates in sync ([`SyncDeviceState`]),
//! and what account-recovery posture it can still reach ([`AuthRecoveryPosture`]). The row then
//! publishes an [`InstallAssurance`] no input can exceed.
//!
//! The [`InstallAssurance`] a lane may publish is the weakest ceiling implied by its observed
//! states, so an unverified binary, an unsupported install topology, a stale portable-state
//! package, a blocked sync apply, a missing passkey, or a policy-limited recovery all narrow or
//! withhold the published label automatically. The guardrail this enforces: a desktop,
//! marketplace, companion, managed, preview, portable, or sync surface never inherits stronger
//! install or auth language from an adjacent stable row — a lane whose binary is unverified,
//! whose topology is unsupported, whose portable package is stale, whose device is blocked from
//! sync, whose passkey is missing, or whose recovery is policy-limited is narrowed to a bounded
//! or retest-pending label, or refused entirely, rather than left quietly stable. The
//! [`AdmissionOutcome`] records the gate's routing action — admit the lane at full trust, admit
//! it bounded, admit it pending retest, or refuse it — and the recomputed [`DowngradeReason`]s
//! explain it; all are validated against the gate.
//!
//! Local-first continuity stays explicit. Each row carries a [`LocalContinuity`] state, so the
//! matrix keeps a lane whose local durable state remains authoritative distinct from one that
//! degraded to local-only because sync or auth degraded, and from one whose local-safe work a
//! policy restricts. A verified lane is forbidden from degrading local continuity, and a lane
//! whose sync or auth degraded is forbidden from claiming its local state is still fully
//! authoritative without saying so.
//!
//! The lane vocabulary is closed and provenance-bound. [`InstallConfigLane`] is the single
//! controlled vocabulary the matrix reuses, each lane is pinned to one [`InstallMode`] so
//! system, user, portable, managed, and marketplace installs stay distinct, and each lane is
//! pinned to the canonical install-truth packet it governs via
//! [`InstallConfigLane::source_packet`], so a clean stable-desktop lane never lends its trust to
//! a refused managed lane and no preview or companion lane inherits a broader stable claim.
//!
//! Because every required downstream surface — release center, Help/About, support export,
//! diagnostics, CLI, and admin docs — binds to this one packet via an
//! [`InstallConsumerBinding`] that must ingest the matrix, preserve its labels and recovery
//! paths, and narrow with it, a lane narrowed here cannot stay authoritative on a release
//! evidence row, an About panel, an admin doc badge, or a support export.
//!
//! The packet is checked in at
//! `artifacts/install/m5/m5-install-and-portability-governance.json` and embedded here. It is
//! metadata-only: every field is a typed state or an opaque ref, and it carries no credential
//! bodies, raw provider payloads, or workspace contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5 install-and-portability governance matrix schema version.
pub const M5_INSTALL_PORTABILITY_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_INSTALL_PORTABILITY_GOVERNANCE_RECORD_KIND: &str =
    "m5_install_and_portability_governance_matrix";

/// Repo-relative path to the checked-in packet.
pub const M5_INSTALL_PORTABILITY_GOVERNANCE_PATH: &str =
    "artifacts/install/m5/m5-install-and-portability-governance.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_INSTALL_PORTABILITY_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/install/m5-install-and-portability-governance.schema.json";

/// Repo-relative path to the companion document.
pub const M5_INSTALL_PORTABILITY_GOVERNANCE_DOC_REF: &str =
    "docs/install/m5/m5-install-and-portability-governance.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_INSTALL_PORTABILITY_GOVERNANCE_ARTIFACT_DOC_REF: &str =
    "artifacts/install/m5/m5-install-and-portability-governance.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_INSTALL_PORTABILITY_GOVERNANCE_FIXTURE_DIR: &str =
    "fixtures/install/m5/m5-install-and-portability-governance";

/// Embedded checked-in packet JSON.
pub const M5_INSTALL_PORTABILITY_GOVERNANCE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/install/m5/m5-install-and-portability-governance.json"
));

/// An M5 install/config/auth lane the governance matrix gates.
///
/// Each lane is governed from the canonical install-truth packet it draws its evidence from, so
/// the matrix aggregates the landed stable-line install, settings, sync, and identity packets
/// into one report instead of re-deriving each lane's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallConfigLane {
    /// Stable-channel desktop install — the first-party local baseline.
    DesktopStable,
    /// Preview-channel desktop install running side-by-side with stable.
    DesktopPreview,
    /// Portable install carrying its own durable state root.
    PortableInstall,
    /// Organization-managed, policy-controlled fleet install.
    ManagedFleet,
    /// Marketplace or companion surface paired to a host install.
    MarketplaceCompanion,
    /// Headless command-line install.
    CliHeadless,
    /// Sync and device-registry participation for a paired install.
    SyncDevice,
}

impl InstallConfigLane {
    /// Every install/config/auth lane, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DesktopStable,
        Self::DesktopPreview,
        Self::PortableInstall,
        Self::ManagedFleet,
        Self::MarketplaceCompanion,
        Self::CliHeadless,
        Self::SyncDevice,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopStable => "desktop_stable",
            Self::DesktopPreview => "desktop_preview",
            Self::PortableInstall => "portable_install",
            Self::ManagedFleet => "managed_fleet",
            Self::MarketplaceCompanion => "marketplace_companion",
            Self::CliHeadless => "cli_headless",
            Self::SyncDevice => "sync_device",
        }
    }

    /// The install mode this lane is pinned to.
    ///
    /// System, user, portable, managed, and marketplace installs stay distinct, and the
    /// `install_mode` recorded on every row is validated against this.
    pub const fn install_mode(self) -> InstallMode {
        match self {
            Self::DesktopStable | Self::DesktopPreview | Self::SyncDevice => InstallMode::System,
            Self::PortableInstall => InstallMode::Portable,
            Self::ManagedFleet => InstallMode::Managed,
            Self::MarketplaceCompanion => InstallMode::Marketplace,
            Self::CliHeadless => InstallMode::User,
        }
    }

    /// Repo-relative path to the canonical install-truth packet this lane governs.
    ///
    /// The matrix is pinned to this packet so a lane never publishes a label its own source
    /// packet does not back, and the `packet_ref` recorded on every row is validated against
    /// it.
    pub const fn source_packet(self) -> &'static str {
        match self {
            Self::DesktopStable => "artifacts/install/state_root_matrix.yaml",
            Self::DesktopPreview => "docs/install/install_topology_alpha.md",
            Self::PortableInstall => "docs/settings/portable_profile_alpha.md",
            Self::ManagedFleet => "docs/admin/org_admin_seat_and_fleet_contract.md",
            Self::MarketplaceCompanion => {
                "docs/auth/managed_auth_and_session_continuity_contract.md"
            }
            Self::CliHeadless => "docs/install/m1_install_topology_truth.md",
            Self::SyncDevice => "docs/settings/sync_and_device_registry_seed.md",
        }
    }

    /// Whether this lane can silently widen install or auth language — a preview, portable,
    /// managed, marketplace/companion, or sync lane that must narrow safely rather than inherit
    /// a broader stable claim.
    pub const fn is_trust_sensitive(self) -> bool {
        matches!(
            self,
            Self::DesktopPreview
                | Self::PortableInstall
                | Self::ManagedFleet
                | Self::MarketplaceCompanion
                | Self::SyncDevice
        )
    }
}

/// How a lane's binary is installed onto the machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallMode {
    /// A system-wide install under a machine-managed root.
    System,
    /// A per-user install under a user-owned root.
    User,
    /// A portable install carrying its own root.
    Portable,
    /// An organization-managed, policy-controlled install.
    Managed,
    /// A marketplace or companion-distributed install.
    Marketplace,
}

impl InstallMode {
    /// Every install mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::System,
        Self::User,
        Self::Portable,
        Self::Managed,
        Self::Marketplace,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
            Self::Portable => "portable",
            Self::Managed => "managed",
            Self::Marketplace => "marketplace",
        }
    }
}

/// The channel-and-ring topology a lane installs from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelRing {
    /// The broad stable channel.
    StableBroad,
    /// A pinned stable channel held at a fixed build.
    StablePinned,
    /// The preview channel's early-access ring.
    PreviewEarlyAccess,
    /// The preview channel's canary ring.
    PreviewCanary,
    /// The nightly channel's canary ring.
    NightlyCanary,
    /// A managed channel pinned by organization policy.
    ManagedPinned,
}

impl ChannelRing {
    /// Every channel-and-ring state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StableBroad,
        Self::StablePinned,
        Self::PreviewEarlyAccess,
        Self::PreviewCanary,
        Self::NightlyCanary,
        Self::ManagedPinned,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableBroad => "stable_broad",
            Self::StablePinned => "stable_pinned",
            Self::PreviewEarlyAccess => "preview_early_access",
            Self::PreviewCanary => "preview_canary",
            Self::NightlyCanary => "nightly_canary",
            Self::ManagedPinned => "managed_pinned",
        }
    }
}

/// Where a lane's durable state root lives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateRootClass {
    /// An isolated state root owned by this install alone.
    IsolatedPerInstall,
    /// A shared user-scoped state root.
    SharedUserScope,
    /// A portable state root carried with the install.
    PortableRoot,
    /// A managed state root provisioned by policy.
    ManagedRoot,
    /// An ephemeral sandbox state root discarded on exit.
    EphemeralSandbox,
}

impl StateRootClass {
    /// Every state-root class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::IsolatedPerInstall,
        Self::SharedUserScope,
        Self::PortableRoot,
        Self::ManagedRoot,
        Self::EphemeralSandbox,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IsolatedPerInstall => "isolated_per_install",
            Self::SharedUserScope => "shared_user_scope",
            Self::PortableRoot => "portable_root",
            Self::ManagedRoot => "managed_root",
            Self::EphemeralSandbox => "ephemeral_sandbox",
        }
    }
}

/// How a lane carries its durable state across machines, if at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortableExportClass {
    /// Native non-portable state owned by this machine.
    NativeState,
    /// A portable package carrying state between machines.
    PortablePackage,
    /// A point-in-time export archive of state.
    ExportArchive,
    /// State imported or migrated from another install.
    ImportedPackage,
    /// State that is not portable.
    NotPortable,
}

impl PortableExportClass {
    /// Every portable/export class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NativeState,
        Self::PortablePackage,
        Self::ExportArchive,
        Self::ImportedPackage,
        Self::NotPortable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeState => "native_state",
            Self::PortablePackage => "portable_package",
            Self::ExportArchive => "export_archive",
            Self::ImportedPackage => "imported_package",
            Self::NotPortable => "not_portable",
        }
    }
}

/// The effective-setting dependency marker for a lane — the scope its effective settings resolve
/// under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectiveSettingScope {
    /// Default baseline settings only.
    DefaultBaseline,
    /// User-scoped overrides on top of the baseline.
    UserScoped,
    /// Workspace-scoped overrides on top of the user scope.
    WorkspaceScoped,
    /// Organization-enforced managed settings.
    ManagedEnforced,
    /// A synced overlay applied from another device.
    SyncedOverlay,
}

impl EffectiveSettingScope {
    /// Every effective-setting scope, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DefaultBaseline,
        Self::UserScoped,
        Self::WorkspaceScoped,
        Self::ManagedEnforced,
        Self::SyncedOverlay,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultBaseline => "default_baseline",
            Self::UserScoped => "user_scoped",
            Self::WorkspaceScoped => "workspace_scoped",
            Self::ManagedEnforced => "managed_enforced",
            Self::SyncedOverlay => "synced_overlay",
        }
    }
}

/// How authoritative a lane's published install-assurance label is.
///
/// Ordered low-to-high by [`InstallAssurance::rank`]: an [`InstallAssurance::Withheld`] lane has
/// no publishable label, and an [`InstallAssurance::Verified`] lane is backed by a verified,
/// supported, fresh, sync-active, passkey-ready install.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallAssurance {
    /// A full-trust claim backed by a verified, supported, fresh install.
    Verified,
    /// Bounded to a side-by-side, platform-trusted, or local-safe slice.
    Bounded,
    /// Held pending retest; self-signed, stale, offline, or fallback-only.
    RetestPending,
    /// Withheld from publication; no publishable install label.
    Withheld,
}

impl InstallAssurance {
    /// Every install-assurance label, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Verified,
        Self::Bounded,
        Self::RetestPending,
        Self::Withheld,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Bounded => "bounded",
            Self::RetestPending => "retest_pending",
            Self::Withheld => "withheld",
        }
    }

    /// Monotonic rank; higher means more authoritative.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Withheld => 0,
            Self::RetestPending => 1,
            Self::Bounded => 2,
            Self::Verified => 3,
        }
    }

    /// The weaker (lower-rank) of two assurance labels.
    pub const fn min(self, other: Self) -> Self {
        if other.rank() < self.rank() {
            other
        } else {
            self
        }
    }
}

/// Whether a lane's installed binary is verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallVerification {
    /// A first-party signed and notarized binary.
    SignedVerified,
    /// A platform-trusted binary without a first-party signature; bounded.
    PlatformTrusted,
    /// A self-signed binary; needs retest.
    SelfSigned,
    /// An unverified binary.
    Unverified,
}

impl InstallVerification {
    /// Every install-verification state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SignedVerified,
        Self::PlatformTrusted,
        Self::SelfSigned,
        Self::Unverified,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedVerified => "signed_verified",
            Self::PlatformTrusted => "platform_trusted",
            Self::SelfSigned => "self_signed",
            Self::Unverified => "unverified",
        }
    }

    /// Highest assurance this verification state permits a lane to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::SignedVerified => InstallAssurance::Verified,
            Self::PlatformTrusted => InstallAssurance::Bounded,
            Self::SelfSigned => InstallAssurance::RetestPending,
            Self::Unverified => InstallAssurance::Withheld,
        }
    }

    /// Whether this state raises the [`DowngradeReason::UnverifiedInstall`] trigger.
    pub const fn is_unverified_trigger(self) -> bool {
        !matches!(self, Self::SignedVerified)
    }
}

/// Whether a lane's install topology is supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallTopologySupport {
    /// The install topology is fully supported.
    Supported,
    /// A side-by-side or coexisting topology supported with known limits; bounded.
    SideBySideBounded,
    /// An experimental install topology; needs retest.
    Experimental,
    /// An unsupported or colliding install topology.
    Unsupported,
}

impl InstallTopologySupport {
    /// Every install-topology-support state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Supported,
        Self::SideBySideBounded,
        Self::Experimental,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::SideBySideBounded => "side_by_side_bounded",
            Self::Experimental => "experimental",
            Self::Unsupported => "unsupported",
        }
    }

    /// Highest assurance this install-topology-support state permits a lane to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::Supported => InstallAssurance::Verified,
            Self::SideBySideBounded => InstallAssurance::Bounded,
            Self::Experimental => InstallAssurance::RetestPending,
            Self::Unsupported => InstallAssurance::Withheld,
        }
    }

    /// Whether this state raises the [`DowngradeReason::UnsupportedInstallTopology`] trigger.
    pub const fn is_unsupported_trigger(self) -> bool {
        !matches!(self, Self::Supported)
    }
}

/// How fresh a lane's portable or exported state package is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortableStateFreshness {
    /// The portable/export state is current.
    Current,
    /// The portable/export state is aging but in tolerance; bounded.
    Aging,
    /// The portable/export state is stale; needs retest.
    Stale,
    /// The portable/export state is missing.
    Missing,
}

impl PortableStateFreshness {
    /// Every portable-state-freshness state, in declaration order.
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

    /// Highest assurance this portable-state-freshness state permits a lane to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::Current => InstallAssurance::Verified,
            Self::Aging => InstallAssurance::Bounded,
            Self::Stale => InstallAssurance::RetestPending,
            Self::Missing => InstallAssurance::Withheld,
        }
    }

    /// Whether this state raises the [`DowngradeReason::StalePortableState`] trigger.
    pub const fn is_stale_trigger(self) -> bool {
        !matches!(self, Self::Current)
    }
}

/// Whether a lane's device participates in sync, and how an apply is routed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncDeviceState {
    /// The device is enrolled and sync applies cleanly.
    Active,
    /// The device is enrolled but sync is degraded; bounded.
    Degraded,
    /// The device is offline; sync applies are deferred and the claim needs retest.
    Offline,
    /// The device is blocked from sync; applies are refused.
    Blocked,
}

impl SyncDeviceState {
    /// Every sync-device state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Active, Self::Degraded, Self::Offline, Self::Blocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Degraded => "degraded",
            Self::Offline => "offline",
            Self::Blocked => "blocked",
        }
    }

    /// Highest assurance this sync-device state permits a lane to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::Active => InstallAssurance::Verified,
            Self::Degraded => InstallAssurance::Bounded,
            Self::Offline => InstallAssurance::RetestPending,
            Self::Blocked => InstallAssurance::Withheld,
        }
    }

    /// Whether this state raises the [`DowngradeReason::BlockedSyncApply`] trigger.
    pub const fn is_blocked_trigger(self) -> bool {
        !matches!(self, Self::Active)
    }

    /// Whether this state degrades sync so local durable state must stay authoritative.
    pub const fn degrades_local(self) -> bool {
        !matches!(self, Self::Active)
    }
}

/// What account-recovery posture a lane can still reach.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthRecoveryPosture {
    /// A passkey is enrolled and verified.
    PasskeyVerified,
    /// No passkey; recovery falls back to the system browser; bounded.
    SystemBrowserFallback,
    /// No passkey and no browser recovery; only local-only continuity remains; needs retest.
    LocalOnlyContinuity,
    /// Recovery is blocked by policy.
    RecoveryBlocked,
}

impl AuthRecoveryPosture {
    /// Every auth-recovery posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PasskeyVerified,
        Self::SystemBrowserFallback,
        Self::LocalOnlyContinuity,
        Self::RecoveryBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PasskeyVerified => "passkey_verified",
            Self::SystemBrowserFallback => "system_browser_fallback",
            Self::LocalOnlyContinuity => "local_only_continuity",
            Self::RecoveryBlocked => "recovery_blocked",
        }
    }

    /// Highest assurance this auth-recovery posture permits a lane to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::PasskeyVerified => InstallAssurance::Verified,
            Self::SystemBrowserFallback => InstallAssurance::Bounded,
            Self::LocalOnlyContinuity => InstallAssurance::RetestPending,
            Self::RecoveryBlocked => InstallAssurance::Withheld,
        }
    }

    /// Whether this posture raises the [`DowngradeReason::MissingPasskey`] trigger.
    ///
    /// A system-browser fallback or local-only continuity posture means no verified passkey is
    /// present.
    pub const fn is_missing_passkey_trigger(self) -> bool {
        matches!(
            self,
            Self::SystemBrowserFallback | Self::LocalOnlyContinuity
        )
    }

    /// Whether this posture raises the [`DowngradeReason::PolicyLimitedRecovery`] trigger.
    pub const fn is_policy_limited_trigger(self) -> bool {
        matches!(self, Self::RecoveryBlocked)
    }

    /// Whether this posture degrades recovery so local durable state must stay authoritative.
    pub const fn degrades_local(self) -> bool {
        !matches!(self, Self::PasskeyVerified)
    }
}

/// Whether a lane's local durable state remains authoritative when sync or auth degrades.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalContinuity {
    /// Local durable state is authoritative; sync and auth are optional overlays.
    Authoritative,
    /// Sync or auth degraded, so the lane fell back to local-only durable state that still
    /// works.
    LocalOnlyFallback,
    /// A policy restricts the lane's local-safe work.
    PolicyRestricted,
}

impl LocalContinuity {
    /// Every local-continuity state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::Authoritative,
        Self::LocalOnlyFallback,
        Self::PolicyRestricted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::LocalOnlyFallback => "local_only_fallback",
            Self::PolicyRestricted => "policy_restricted",
        }
    }

    /// Whether the lane still preserves local durable state — true unless a policy restricts it.
    pub const fn preserves_local_work(self) -> bool {
        !matches!(self, Self::PolicyRestricted)
    }
}

/// The recovery path surfaced when a lane's label is narrowed or withheld.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradePath {
    /// Verify the install signature before widening trust.
    VerifyInstallSignature,
    /// Switch to or request a supported install topology.
    SwitchSupportedTopology,
    /// Refresh the stale or missing portable/export state.
    RefreshPortableState,
    /// Restore the blocked or offline sync apply by re-enrolling the device.
    RestoreSyncApply,
    /// Enroll a passkey to restore full recovery.
    EnrollPasskey,
    /// Request a recovery policy change for the policy-limited lane.
    RequestRecoveryPolicy,
    /// Withhold the lane's label from publication.
    WithholdClaim,
    /// No downgrade is needed; only valid when the lane is published verified.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl DowngradePath {
    /// Every downgrade path, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::VerifyInstallSignature,
        Self::SwitchSupportedTopology,
        Self::RefreshPortableState,
        Self::RestoreSyncApply,
        Self::EnrollPasskey,
        Self::RequestRecoveryPolicy,
        Self::WithholdClaim,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifyInstallSignature => "verify_install_signature",
            Self::SwitchSupportedTopology => "switch_supported_topology",
            Self::RefreshPortableState => "refresh_portable_state",
            Self::RestoreSyncApply => "restore_sync_apply",
            Self::EnrollPasskey => "enroll_passkey",
            Self::RequestRecoveryPolicy => "request_recovery_policy",
            Self::WithholdClaim => "withhold_claim",
            Self::NoneNeeded => "none",
        }
    }

    /// Whether this is a real recovery path the lane owner can take.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::NoneNeeded)
    }
}

/// A headline reason the governance gate narrows a lane.
///
/// These are the canonical install/config/auth downgrade reasons: an unverified binary, an
/// unsupported install topology, a stale portable-state package, a blocked sync apply, a missing
/// passkey, and a policy-limited recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeReason {
    /// The lane's installed binary is not signed and verified.
    UnverifiedInstall,
    /// The lane's install topology is side-by-side bounded, experimental, or unsupported.
    UnsupportedInstallTopology,
    /// The lane's portable or exported state is aging, stale, or missing.
    StalePortableState,
    /// The lane's device is degraded, offline, or blocked from sync.
    BlockedSyncApply,
    /// The lane has no verified passkey and fell back to a weaker recovery.
    MissingPasskey,
    /// The lane's account recovery is limited by policy.
    PolicyLimitedRecovery,
}

impl DowngradeReason {
    /// Every downgrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UnverifiedInstall,
        Self::UnsupportedInstallTopology,
        Self::StalePortableState,
        Self::BlockedSyncApply,
        Self::MissingPasskey,
        Self::PolicyLimitedRecovery,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnverifiedInstall => "unverified_install",
            Self::UnsupportedInstallTopology => "unsupported_install_topology",
            Self::StalePortableState => "stale_portable_state",
            Self::BlockedSyncApply => "blocked_sync_apply",
            Self::MissingPasskey => "missing_passkey",
            Self::PolicyLimitedRecovery => "policy_limited_recovery",
        }
    }
}

/// The admission outcome the governance gate routes a lane to, relative to a clean verified
/// install.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionOutcome {
    /// No narrowing; the lane is admitted at full trust.
    AdmitFull,
    /// The lane is admitted bounded to a side-by-side or local-safe slice.
    AdmitBounded,
    /// The lane is admitted pending retest.
    AdmitRetest,
    /// The lane's admission is refused.
    Refuse,
}

impl AdmissionOutcome {
    /// Every admission outcome, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AdmitFull,
        Self::AdmitBounded,
        Self::AdmitRetest,
        Self::Refuse,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmitFull => "admit_full",
            Self::AdmitBounded => "admit_bounded",
            Self::AdmitRetest => "admit_retest",
            Self::Refuse => "refuse",
        }
    }

    /// Whether the gate narrowed or refused the lane's admission.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::AdmitFull)
    }

    /// The admission outcome implied by a published assurance label.
    pub const fn for_assurance(assurance: InstallAssurance) -> Self {
        match assurance {
            InstallAssurance::Verified => Self::AdmitFull,
            InstallAssurance::Bounded => Self::AdmitBounded,
            InstallAssurance::RetestPending => Self::AdmitRetest,
            InstallAssurance::Withheld => Self::Refuse,
        }
    }
}

/// A downstream surface that must ingest this governance packet and narrow with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Release center evidence and proof index.
    ReleaseCenter,
    /// Help and About product-surface copy.
    HelpAbout,
    /// Support export bundle.
    SupportExport,
    /// Install and update diagnostics surface.
    Diagnostics,
    /// Command-line install and status surface.
    Cli,
    /// Administrator-facing documentation.
    AdminDocs,
}

impl ConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::ReleaseCenter,
        Self::HelpAbout,
        Self::SupportExport,
        Self::Diagnostics,
        Self::Cli,
        Self::AdminDocs,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::HelpAbout => "help_about",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::Cli => "cli",
            Self::AdminDocs => "admin_docs",
        }
    }
}

/// One governance row for an M5 install/config/auth lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstallConfigRow {
    /// Stable governance-row id.
    pub lane_id: String,
    /// Install/config/auth lane this row governs.
    pub lane: InstallConfigLane,
    /// Install mode this lane uses; must equal [`InstallConfigLane::install_mode`].
    pub install_mode: InstallMode,
    /// Channel-and-ring topology this lane installs from.
    pub channel_ring: ChannelRing,
    /// Where this lane's durable state root lives.
    pub state_root_class: StateRootClass,
    /// How this lane carries its durable state across machines.
    pub portable_export_class: PortableExportClass,
    /// The effective-setting scope this lane resolves under.
    pub effective_setting_scope: EffectiveSettingScope,
    /// Owner accountable for the lane's evidence and conformance.
    pub owner: String,
    /// Whether the lane's installed binary is verified.
    pub install_verification: InstallVerification,
    /// Whether the lane's install topology is supported.
    pub install_topology_support: InstallTopologySupport,
    /// How fresh the lane's portable/export state is.
    pub portable_state_freshness: PortableStateFreshness,
    /// Whether the lane's device participates in sync.
    pub sync_device_state: SyncDeviceState,
    /// What account-recovery posture the lane can reach.
    pub auth_recovery_posture: AuthRecoveryPosture,
    /// Whether the lane's local durable state remains authoritative.
    pub local_continuity: LocalContinuity,
    /// Stable namespace the lane mints install-root identities under.
    pub install_root_namespace: String,
    /// Stable namespace the lane mints state-root identities under.
    pub state_root_namespace: String,
    /// Assurance the lane's own evidence asserts, before the gate.
    pub declared_assurance: InstallAssurance,
    /// Assurance actually published after the gate narrows the lane.
    ///
    /// Must equal [`InstallConfigRow::effective_assurance`].
    pub published_assurance: InstallAssurance,
    /// Admission outcome the gate routes to; must equal the recomputed outcome.
    pub admission_outcome: AdmissionOutcome,
    /// Headline downgrade reasons; must equal the recomputed set.
    #[serde(default)]
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Recovery path surfaced when the label is narrowed or withheld.
    pub downgrade_path: DowngradePath,
    /// Scope or slice labels this lane still backs.
    #[serde(default)]
    pub supported_scopes: Vec<String>,
    /// Caveats attached to the published label.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale, missing, or narrowing the label.
    #[serde(default)]
    pub stale_or_missing_fields: Vec<String>,
    /// Ref to the canonical install-truth packet this lane governs.
    ///
    /// Must equal [`InstallConfigLane::source_packet`].
    pub packet_ref: String,
    /// Ref to the install-conformance suite backing the lane.
    pub conformance_ref: String,
    /// Ref to the lane's supporting evidence.
    pub evidence_ref: String,
    /// Ref to the machine-readable governance receipt for audit and release evidence.
    pub governance_receipt_ref: String,
    /// Ref binding this row into the release-evidence surface.
    pub release_evidence_ref: String,
    /// Ref binding this row into the Help/About surface.
    pub help_about_ref: String,
    /// Ref binding this row into the support-export surface.
    pub support_export_ref: String,
    /// Ref binding this row into the diagnostics surface.
    pub diagnostics_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl InstallConfigRow {
    /// The label the lane's own evidence asserted, before environmental narrowing.
    pub fn capability_floor(&self) -> InstallAssurance {
        self.declared_assurance
    }

    /// The assurance label the gate permits this lane to publish.
    ///
    /// Lowers the capability floor to the weakest ceiling implied by the install verification,
    /// topology support, portable-state freshness, sync-device state, and auth-recovery posture,
    /// so an unverified binary, an unsupported topology, a stale portable package, a blocked
    /// sync apply, a missing passkey, or a policy-limited recovery can never publish a verified
    /// label.
    pub fn effective_assurance(&self) -> InstallAssurance {
        self.capability_floor()
            .min(self.install_verification.assurance_ceiling())
            .min(self.install_topology_support.assurance_ceiling())
            .min(self.portable_state_freshness.assurance_ceiling())
            .min(self.sync_device_state.assurance_ceiling())
            .min(self.auth_recovery_posture.assurance_ceiling())
    }

    /// The headline downgrade reasons recomputed from the lane's observed states.
    pub fn computed_downgrade_reasons(&self) -> Vec<DowngradeReason> {
        let mut reasons = Vec::new();
        if self.install_verification.is_unverified_trigger() {
            reasons.push(DowngradeReason::UnverifiedInstall);
        }
        if self.install_topology_support.is_unsupported_trigger() {
            reasons.push(DowngradeReason::UnsupportedInstallTopology);
        }
        if self.portable_state_freshness.is_stale_trigger() {
            reasons.push(DowngradeReason::StalePortableState);
        }
        if self.sync_device_state.is_blocked_trigger() {
            reasons.push(DowngradeReason::BlockedSyncApply);
        }
        if self.auth_recovery_posture.is_missing_passkey_trigger() {
            reasons.push(DowngradeReason::MissingPasskey);
        }
        if self.auth_recovery_posture.is_policy_limited_trigger() {
            reasons.push(DowngradeReason::PolicyLimitedRecovery);
        }
        reasons
    }

    /// The admission outcome the gate must route for this lane, derived from its effective
    /// assurance.
    pub fn required_outcome(&self) -> AdmissionOutcome {
        AdmissionOutcome::for_assurance(self.effective_assurance())
    }

    /// Whether the lane publishes a clean verified label.
    pub fn is_verified(&self) -> bool {
        self.effective_assurance() == InstallAssurance::Verified
    }

    /// Whether the gate narrowed the published label below what the lane declared.
    ///
    /// This is the automatic downgrade: an unverified, unsupported, stale, sync-blocked, or
    /// recovery-degraded lane that declared a stronger label has its published label lowered
    /// rather than left quietly stable.
    pub fn is_downgraded(&self) -> bool {
        self.effective_assurance().rank() < self.capability_floor().rank()
    }

    /// Whether the lane's sync or auth degraded, so local durable state must stay authoritative.
    pub fn degrades_local(&self) -> bool {
        self.sync_device_state.degrades_local() || self.auth_recovery_posture.degrades_local()
    }

    /// Whether the lane carries its own non-empty source, conformance, evidence, receipt, and
    /// downstream-consumer refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.packet_ref.trim().is_empty()
            && !self.conformance_ref.trim().is_empty()
            && !self.evidence_ref.trim().is_empty()
            && !self.governance_receipt_ref.trim().is_empty()
            && !self.release_evidence_ref.trim().is_empty()
            && !self.help_about_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
            && !self.diagnostics_ref.trim().is_empty()
    }

    /// Whether the stored published label, outcome, and downgrade reasons all agree with the
    /// recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_assurance == self.effective_assurance()
            && self.admission_outcome == self.required_outcome()
            && self.downgrade_reasons == self.computed_downgrade_reasons()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5InstallPortabilityGovernanceSummary {
    /// Total lane rows.
    pub total_lanes: usize,
    /// Number of claimed lanes.
    pub lane_count: usize,
    /// Lanes published as verified.
    pub verified_lanes: usize,
    /// Lanes narrowed to a bounded label.
    pub bounded_lanes: usize,
    /// Lanes narrowed to a retest-pending label.
    pub retest_pending_lanes: usize,
    /// Lanes withheld from publication.
    pub withheld_lanes: usize,
    /// Lanes the gate admitted at full trust.
    pub admit_full_decisions: usize,
    /// Lanes the gate admitted bounded.
    pub admit_bounded_decisions: usize,
    /// Lanes the gate admitted pending retest.
    pub admit_retest_decisions: usize,
    /// Lanes the gate refused.
    pub refuse_decisions: usize,
    /// Lanes whose published label was downgraded below what they declared.
    pub downgraded_lanes: usize,
    /// Trust-sensitive lanes.
    pub trust_sensitive_lanes: usize,
    /// Lanes whose binary is not signed and verified.
    pub unverified_install_lanes: usize,
    /// Lanes whose install topology is not fully supported.
    pub unsupported_topology_lanes: usize,
    /// Lanes whose portable/export state is aging, stale, or missing.
    pub stale_portable_lanes: usize,
    /// Lanes whose device is degraded, offline, or blocked from sync.
    pub blocked_sync_lanes: usize,
    /// Lanes whose account recovery is degraded below a verified passkey.
    pub degraded_recovery_lanes: usize,
    /// Lanes carrying at least one downgrade reason.
    pub lanes_with_downgrade_reasons: usize,
    /// Lanes that fell back to local-only durable state.
    pub local_only_fallback_lanes: usize,
    /// Lanes whose local-safe work is restricted by policy.
    pub policy_restricted_lanes: usize,
}

/// A redaction-safe export row projected from a governance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallPortabilityGovernanceExportRow {
    /// Governance-row id.
    pub lane_id: String,
    /// Lane token.
    pub lane: String,
    /// Install-mode token.
    pub install_mode: String,
    /// Channel-and-ring token.
    pub channel_ring: String,
    /// State-root-class token.
    pub state_root_class: String,
    /// Portable/export-class token.
    pub portable_export_class: String,
    /// Effective-setting-scope token.
    pub effective_setting_scope: String,
    /// Owner accountable for the lane.
    pub owner: String,
    /// Install-verification token.
    pub install_verification: String,
    /// Install-topology-support token.
    pub install_topology_support: String,
    /// Portable-state-freshness token.
    pub portable_state_freshness: String,
    /// Sync-device-state token.
    pub sync_device_state: String,
    /// Auth-recovery-posture token.
    pub auth_recovery_posture: String,
    /// Local-continuity token.
    pub local_continuity: String,
    /// Declared-assurance token.
    pub declared_assurance: String,
    /// Published-assurance token.
    pub published_assurance: String,
    /// Admission-outcome token.
    pub admission_outcome: String,
    /// Downgrade-reason tokens.
    pub downgrade_reasons: Vec<String>,
    /// Downgrade-path token.
    pub downgrade_path: String,
    /// Supported scope or slice labels.
    pub supported_scopes: Vec<String>,
    /// Caveats attached to the published label.
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale or missing.
    pub stale_or_missing_fields: Vec<String>,
    /// Source-packet ref this lane governs.
    pub packet_ref: String,
    /// Governance-receipt ref.
    pub governance_receipt_ref: String,
    /// Release-evidence ref.
    pub release_evidence_ref: String,
    /// Help/About ref.
    pub help_about_ref: String,
    /// Support-export ref.
    pub support_export_ref: String,
    /// Diagnostics ref.
    pub diagnostics_ref: String,
    /// Whether the lane is trust-sensitive.
    pub trust_sensitive: bool,
    /// Whether the lane publishes a verified label.
    pub verified: bool,
    /// Whether the published label was downgraded below the declared label.
    pub downgraded: bool,
    /// Whether the lane still preserves local durable work.
    pub preserves_local_work: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet — the canonical install/config index
/// downstream surfaces render instead of restating each lane's label by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallPortabilityGovernanceExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub lanes: Vec<M5InstallPortabilityGovernanceExportRow>,
    /// Whether every lane's published label and outcome agree with the gate.
    pub all_lanes_gate_consistent: bool,
    /// Lanes that publish a verified label.
    pub verified_count: usize,
    /// Lanes the gate narrowed or refused.
    pub narrowed_count: usize,
    /// Lanes the gate refused entirely.
    pub refused_count: usize,
}

/// One binding wiring a downstream surface to this governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstallConsumerBinding {
    /// Consumer surface this binding wires.
    pub consumer_surface: ConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Governance packet id this surface ingests.
    pub governance_packet_id_ref: String,
    /// True when the surface ingests this governance packet rather than a parallel sheet.
    pub ingests_governance_matrix: bool,
    /// True when the surface preserves the published labels verbatim.
    pub preserves_published_labels: bool,
    /// True when the surface preserves the recovery paths verbatim.
    pub preserves_downgrade_paths: bool,
    /// True when the surface narrows automatically as rows are downgraded.
    pub narrows_on_downgrade: bool,
    /// True when raw private material is excluded from the binding.
    pub raw_private_material_excluded: bool,
}

impl InstallConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.governance_packet_id_ref == packet_id
            && self.ingests_governance_matrix
            && self.preserves_published_labels
            && self.preserves_downgrade_paths
            && self.narrows_on_downgrade
            && self.raw_private_material_excluded
            && !self.binding_ref.trim().is_empty()
    }
}

/// The typed M5 install-and-portability governance matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5InstallPortabilityGovernanceMatrix {
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
    /// Scheme the matrix mints stable install identities under.
    pub install_identity_scheme: String,
    /// Claimed lanes; one row per lane.
    pub lanes: Vec<InstallConfigLane>,
    /// Closed install-mode vocabulary.
    pub install_modes: Vec<InstallMode>,
    /// Closed channel-and-ring vocabulary.
    pub channel_rings: Vec<ChannelRing>,
    /// Closed state-root-class vocabulary.
    pub state_root_classes: Vec<StateRootClass>,
    /// Closed portable/export-class vocabulary.
    pub portable_export_classes: Vec<PortableExportClass>,
    /// Closed effective-setting-scope vocabulary.
    pub effective_setting_scopes: Vec<EffectiveSettingScope>,
    /// Closed assurance-label vocabulary.
    pub assurance_labels: Vec<InstallAssurance>,
    /// Closed install-verification vocabulary.
    pub install_verification_states: Vec<InstallVerification>,
    /// Closed install-topology-support vocabulary.
    pub install_topology_support_states: Vec<InstallTopologySupport>,
    /// Closed portable-state-freshness vocabulary.
    pub portable_state_freshness_states: Vec<PortableStateFreshness>,
    /// Closed sync-device-state vocabulary.
    pub sync_device_states: Vec<SyncDeviceState>,
    /// Closed auth-recovery-posture vocabulary.
    pub auth_recovery_postures: Vec<AuthRecoveryPosture>,
    /// Closed local-continuity vocabulary.
    pub local_continuity_states: Vec<LocalContinuity>,
    /// Closed downgrade-path vocabulary.
    pub downgrade_paths: Vec<DowngradePath>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Closed admission-outcome vocabulary.
    pub admission_outcomes: Vec<AdmissionOutcome>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<ConsumerSurface>,
    /// Governance rows, one per claimed lane.
    #[serde(default)]
    pub lane_rows: Vec<InstallConfigRow>,
    /// Consumer bindings, one per required surface.
    #[serde(default)]
    pub consumer_bindings: Vec<InstallConsumerBinding>,
    /// Summary counts.
    pub summary: M5InstallPortabilityGovernanceSummary,
}

impl M5InstallPortabilityGovernanceMatrix {
    /// Returns the row for a claimed lane.
    pub fn lane_row(&self, lane: InstallConfigLane) -> Option<&InstallConfigRow> {
        self.lane_rows.iter().find(|c| c.lane == lane)
    }

    /// Lanes that publish a verified label.
    pub fn verified_lanes(&self) -> impl Iterator<Item = &InstallConfigRow> {
        self.lane_rows.iter().filter(|c| c.is_verified())
    }

    /// Lanes the gate narrowed or refused in any way.
    pub fn narrowed_lanes(&self) -> impl Iterator<Item = &InstallConfigRow> {
        self.lane_rows
            .iter()
            .filter(|c| c.required_outcome().is_narrowed())
    }

    /// Lanes the gate refused entirely.
    pub fn refused_lanes(&self) -> impl Iterator<Item = &InstallConfigRow> {
        self.lane_rows
            .iter()
            .filter(|c| c.required_outcome() == AdmissionOutcome::Refuse)
    }

    /// Whether a consumer binding preserves this packet for the given surface.
    pub fn has_binding_for(&self, surface: ConsumerSurface) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer_surface == surface && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every lane's stored published label, outcome, and reasons agree with the
    /// recomputed gate decision.
    pub fn all_lanes_gate_consistent(&self) -> bool {
        self.lane_rows.iter().all(|c| c.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5InstallPortabilityGovernanceSummary {
        let count_published = |label: InstallAssurance| {
            self.lane_rows
                .iter()
                .filter(|c| c.published_assurance == label)
                .count()
        };
        let count_outcome = |outcome: AdmissionOutcome| {
            self.lane_rows
                .iter()
                .filter(|c| c.admission_outcome == outcome)
                .count()
        };
        let count_continuity = |state: LocalContinuity| {
            self.lane_rows
                .iter()
                .filter(|c| c.local_continuity == state)
                .count()
        };
        M5InstallPortabilityGovernanceSummary {
            total_lanes: self.lane_rows.len(),
            lane_count: self.lanes.len(),
            verified_lanes: count_published(InstallAssurance::Verified),
            bounded_lanes: count_published(InstallAssurance::Bounded),
            retest_pending_lanes: count_published(InstallAssurance::RetestPending),
            withheld_lanes: count_published(InstallAssurance::Withheld),
            admit_full_decisions: count_outcome(AdmissionOutcome::AdmitFull),
            admit_bounded_decisions: count_outcome(AdmissionOutcome::AdmitBounded),
            admit_retest_decisions: count_outcome(AdmissionOutcome::AdmitRetest),
            refuse_decisions: count_outcome(AdmissionOutcome::Refuse),
            downgraded_lanes: self.lane_rows.iter().filter(|c| c.is_downgraded()).count(),
            trust_sensitive_lanes: self
                .lane_rows
                .iter()
                .filter(|c| c.lane.is_trust_sensitive())
                .count(),
            unverified_install_lanes: self
                .lane_rows
                .iter()
                .filter(|c| c.install_verification.is_unverified_trigger())
                .count(),
            unsupported_topology_lanes: self
                .lane_rows
                .iter()
                .filter(|c| c.install_topology_support.is_unsupported_trigger())
                .count(),
            stale_portable_lanes: self
                .lane_rows
                .iter()
                .filter(|c| c.portable_state_freshness.is_stale_trigger())
                .count(),
            blocked_sync_lanes: self
                .lane_rows
                .iter()
                .filter(|c| c.sync_device_state.is_blocked_trigger())
                .count(),
            degraded_recovery_lanes: self
                .lane_rows
                .iter()
                .filter(|c| c.auth_recovery_posture.degrades_local())
                .count(),
            lanes_with_downgrade_reasons: self
                .lane_rows
                .iter()
                .filter(|c| !c.downgrade_reasons.is_empty())
                .count(),
            local_only_fallback_lanes: count_continuity(LocalContinuity::LocalOnlyFallback),
            policy_restricted_lanes: count_continuity(LocalContinuity::PolicyRestricted),
        }
    }

    /// Produces the install/config index downstream surfaces — release center, Help/About,
    /// support export, diagnostics, CLI, and admin docs — render instead of restating each
    /// lane's posture by hand.
    pub fn export_projection(&self) -> M5InstallPortabilityGovernanceExportProjection {
        let lanes = self
            .lane_rows
            .iter()
            .map(|c| M5InstallPortabilityGovernanceExportRow {
                lane_id: c.lane_id.clone(),
                lane: c.lane.as_str().to_owned(),
                install_mode: c.install_mode.as_str().to_owned(),
                channel_ring: c.channel_ring.as_str().to_owned(),
                state_root_class: c.state_root_class.as_str().to_owned(),
                portable_export_class: c.portable_export_class.as_str().to_owned(),
                effective_setting_scope: c.effective_setting_scope.as_str().to_owned(),
                owner: c.owner.clone(),
                install_verification: c.install_verification.as_str().to_owned(),
                install_topology_support: c.install_topology_support.as_str().to_owned(),
                portable_state_freshness: c.portable_state_freshness.as_str().to_owned(),
                sync_device_state: c.sync_device_state.as_str().to_owned(),
                auth_recovery_posture: c.auth_recovery_posture.as_str().to_owned(),
                local_continuity: c.local_continuity.as_str().to_owned(),
                declared_assurance: c.declared_assurance.as_str().to_owned(),
                published_assurance: c.published_assurance.as_str().to_owned(),
                admission_outcome: c.admission_outcome.as_str().to_owned(),
                downgrade_reasons: c
                    .downgrade_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                downgrade_path: c.downgrade_path.as_str().to_owned(),
                supported_scopes: c.supported_scopes.clone(),
                caveats: c.caveats.clone(),
                stale_or_missing_fields: c.stale_or_missing_fields.clone(),
                packet_ref: c.packet_ref.clone(),
                governance_receipt_ref: c.governance_receipt_ref.clone(),
                release_evidence_ref: c.release_evidence_ref.clone(),
                help_about_ref: c.help_about_ref.clone(),
                support_export_ref: c.support_export_ref.clone(),
                diagnostics_ref: c.diagnostics_ref.clone(),
                trust_sensitive: c.lane.is_trust_sensitive(),
                verified: c.is_verified(),
                downgraded: c.is_downgraded(),
                preserves_local_work: c.local_continuity.preserves_local_work(),
                summary: format!(
                    "{} ({} / {}): verification {}, topology {}, portable {}, sync {}, recovery {}, declared {}, published {} ({}), recovery {}",
                    c.lane.as_str(),
                    c.install_mode.as_str(),
                    c.channel_ring.as_str(),
                    c.install_verification.as_str(),
                    c.install_topology_support.as_str(),
                    c.portable_state_freshness.as_str(),
                    c.sync_device_state.as_str(),
                    c.auth_recovery_posture.as_str(),
                    c.declared_assurance.as_str(),
                    c.published_assurance.as_str(),
                    c.admission_outcome.as_str(),
                    c.downgrade_path.as_str()
                ),
            })
            .collect();
        M5InstallPortabilityGovernanceExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            lanes,
            all_lanes_gate_consistent: self.all_lanes_gate_consistent(),
            verified_count: self.verified_lanes().count(),
            narrowed_count: self.narrowed_lanes().count(),
            refused_count: self.refused_lanes().count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact governance matrix.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5InstallPortabilityGovernanceSupportExport {
        M5InstallPortabilityGovernanceSupportExport {
            record_kind: M5_INSTALL_PORTABILITY_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_INSTALL_PORTABILITY_GOVERNANCE_SCHEMA_VERSION,
            export_id: export_id.into(),
            governance_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            governance_matrix: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5InstallPortabilityGovernanceViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<InstallConfigLane> = self.lanes.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_lanes = BTreeSet::new();
        for row in &self.lane_rows {
            if !seen_ids.insert(row.lane_id.clone()) {
                violations.push(M5InstallPortabilityGovernanceViolation::DuplicateLaneId {
                    lane_id: row.lane_id.clone(),
                });
            }
            if !seen_lanes.insert(row.lane) {
                violations.push(M5InstallPortabilityGovernanceViolation::DuplicateLaneRow {
                    lane: row.lane.as_str(),
                });
            }
            if !claimed.contains(&row.lane) {
                violations.push(M5InstallPortabilityGovernanceViolation::UnclaimedLaneRow {
                    lane_id: row.lane_id.clone(),
                    lane: row.lane.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed lane must carry its own row, so a lane never inherits a verified label
        // from an adjacent one.
        for &lane in &self.lanes {
            if !seen_lanes.contains(&lane) {
                violations.push(M5InstallPortabilityGovernanceViolation::MissingLaneRow {
                    lane: lane.as_str(),
                });
            }
        }

        // Every required consumer surface must bind to this packet and narrow with it, so a
        // narrowed row cannot stay stable on a downstream surface by inertia.
        for surface in ConsumerSurface::REQUIRED {
            if !self.has_binding_for(surface) {
                violations.push(
                    M5InstallPortabilityGovernanceViolation::MissingConsumerBinding {
                        surface: surface.as_str(),
                    },
                );
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(
                    M5InstallPortabilityGovernanceViolation::ConsumerBindingDrift {
                        binding_ref: binding.binding_ref.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5InstallPortabilityGovernanceViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5InstallPortabilityGovernanceViolation>) {
        if self.schema_version != M5_INSTALL_PORTABILITY_GOVERNANCE_SCHEMA_VERSION {
            violations.push(
                M5InstallPortabilityGovernanceViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != M5_INSTALL_PORTABILITY_GOVERNANCE_RECORD_KIND {
            violations.push(
                M5InstallPortabilityGovernanceViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("install_identity_scheme", &self.install_identity_scheme),
        ] {
            if value.trim().is_empty() {
                violations.push(M5InstallPortabilityGovernanceViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            ("lanes", self.lanes == InstallConfigLane::ALL.to_vec()),
            (
                "install_modes",
                self.install_modes == InstallMode::ALL.to_vec(),
            ),
            (
                "channel_rings",
                self.channel_rings == ChannelRing::ALL.to_vec(),
            ),
            (
                "state_root_classes",
                self.state_root_classes == StateRootClass::ALL.to_vec(),
            ),
            (
                "portable_export_classes",
                self.portable_export_classes == PortableExportClass::ALL.to_vec(),
            ),
            (
                "effective_setting_scopes",
                self.effective_setting_scopes == EffectiveSettingScope::ALL.to_vec(),
            ),
            (
                "assurance_labels",
                self.assurance_labels == InstallAssurance::ALL.to_vec(),
            ),
            (
                "install_verification_states",
                self.install_verification_states == InstallVerification::ALL.to_vec(),
            ),
            (
                "install_topology_support_states",
                self.install_topology_support_states == InstallTopologySupport::ALL.to_vec(),
            ),
            (
                "portable_state_freshness_states",
                self.portable_state_freshness_states == PortableStateFreshness::ALL.to_vec(),
            ),
            (
                "sync_device_states",
                self.sync_device_states == SyncDeviceState::ALL.to_vec(),
            ),
            (
                "auth_recovery_postures",
                self.auth_recovery_postures == AuthRecoveryPosture::ALL.to_vec(),
            ),
            (
                "local_continuity_states",
                self.local_continuity_states == LocalContinuity::ALL.to_vec(),
            ),
            (
                "downgrade_paths",
                self.downgrade_paths == DowngradePath::ALL.to_vec(),
            ),
            (
                "downgrade_reasons",
                self.downgrade_reasons == DowngradeReason::ALL.to_vec(),
            ),
            (
                "admission_outcomes",
                self.admission_outcomes == AdmissionOutcome::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == ConsumerSurface::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(
                    M5InstallPortabilityGovernanceViolation::ClosedVocabularyMismatch { field },
                );
            }
        }
    }

    fn validate_row(
        &self,
        row: &InstallConfigRow,
        violations: &mut Vec<M5InstallPortabilityGovernanceViolation>,
    ) {
        for (field, value) in [
            ("lane_id", &row.lane_id),
            ("owner", &row.owner),
            ("install_root_namespace", &row.install_root_namespace),
            ("state_root_namespace", &row.state_root_namespace),
            ("packet_ref", &row.packet_ref),
            ("conformance_ref", &row.conformance_ref),
            ("evidence_ref", &row.evidence_ref),
            ("governance_receipt_ref", &row.governance_receipt_ref),
            ("release_evidence_ref", &row.release_evidence_ref),
            ("help_about_ref", &row.help_about_ref),
            ("support_export_ref", &row.support_export_ref),
            ("diagnostics_ref", &row.diagnostics_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5InstallPortabilityGovernanceViolation::EmptyField {
                    id: row.lane_id.clone(),
                    field_name: field,
                });
            }
        }

        // The lane's source packet must be the canonical install-truth packet it governs, so a
        // lane never publishes a label its own source packet does not back.
        if row.packet_ref != row.lane.source_packet() {
            violations.push(
                M5InstallPortabilityGovernanceViolation::SourcePacketMismatch {
                    lane_id: row.lane_id.clone(),
                    expected: row.lane.source_packet(),
                },
            );
        }

        // System, user, portable, managed, and marketplace installs stay distinct: the row's
        // install mode must equal its lane's pinned mode.
        if row.install_mode != row.lane.install_mode() {
            violations.push(
                M5InstallPortabilityGovernanceViolation::InstallModeMismatch {
                    lane_id: row.lane_id.clone(),
                    expected: row.lane.install_mode().as_str(),
                },
            );
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.downgrade_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(
                    M5InstallPortabilityGovernanceViolation::DuplicateDowngradeReason {
                        lane_id: row.lane_id.clone(),
                        reason: reason.as_str(),
                    },
                );
            }
        }

        // The published label must equal the gate's recomputed ceiling, so an unverified,
        // unsupported, stale, sync-blocked, or recovery-degraded lane can never read as verified.
        let effective = row.effective_assurance();
        if row.published_assurance != effective {
            violations.push(M5InstallPortabilityGovernanceViolation::OverstatedClaim {
                lane_id: row.lane_id.clone(),
                published: row.published_assurance.as_str(),
                computed: effective.as_str(),
            });
        }

        // The routed admission outcome must match the gate's recomputed outcome.
        let required = row.required_outcome();
        if row.admission_outcome != required {
            violations.push(M5InstallPortabilityGovernanceViolation::OutcomeMismatch {
                lane_id: row.lane_id.clone(),
                declared: row.admission_outcome.as_str(),
                required: required.as_str(),
            });
        }

        // The recorded downgrade reasons must equal the reasons recomputed from the observed
        // states, so a downgrade can never be asserted or hidden by hand.
        let computed = row.computed_downgrade_reasons();
        if row.downgrade_reasons != computed {
            violations.push(
                M5InstallPortabilityGovernanceViolation::DowngradeReasonsMismatch {
                    lane_id: row.lane_id.clone(),
                },
            );
        }

        // A narrowed or refused lane must offer a real recovery path, list at least one caveat,
        // and name what is stale or narrowing, so a degraded lane never drops its recovery
        // semantics or hides why it was narrowed.
        if row.admission_outcome.is_narrowed() {
            if !row.downgrade_path.is_offered() {
                violations.push(
                    M5InstallPortabilityGovernanceViolation::MissingDowngradePath {
                        lane_id: row.lane_id.clone(),
                    },
                );
            }
            if row.caveats.is_empty() {
                violations.push(M5InstallPortabilityGovernanceViolation::EmptyField {
                    id: row.lane_id.clone(),
                    field_name: "caveats",
                });
            }
            if row.stale_or_missing_fields.is_empty() {
                violations.push(M5InstallPortabilityGovernanceViolation::EmptyField {
                    id: row.lane_id.clone(),
                    field_name: "stale_or_missing_fields",
                });
            }
        }

        // A lane that still backs a publishable label must name at least one supported scope or
        // slice label.
        if row.published_assurance != InstallAssurance::Withheld && row.supported_scopes.is_empty()
        {
            violations.push(M5InstallPortabilityGovernanceViolation::EmptyField {
                id: row.lane_id.clone(),
                field_name: "supported_scopes",
            });
        }

        // Local-first continuity stays truthful: a lane whose sync or auth degraded must report
        // a local-only fallback or a policy restriction rather than claim its local state is
        // still fully authoritative, and a lane whose sync and auth are both healthy must not
        // falsely claim a degraded local fallback.
        let continuity_ok = if row.degrades_local() {
            row.local_continuity != LocalContinuity::Authoritative
        } else {
            row.local_continuity == LocalContinuity::Authoritative
        };
        if !continuity_ok {
            violations.push(
                M5InstallPortabilityGovernanceViolation::LocalContinuityMismatch {
                    lane_id: row.lane_id.clone(),
                    continuity: row.local_continuity.as_str(),
                },
            );
        }

        // A verified lane must be genuinely whole-trust: a signed binary, a supported topology,
        // current portable state, an active sync device, a verified passkey, a declared verified
        // floor, authoritative local continuity, no downgrade reason, no caveat, no
        // stale-or-missing field, and a no-op recovery path. This is the non-inheritance
        // guardrail — a lane never widens install or auth language over an unverified or
        // degraded install.
        if row.is_verified()
            && (row.install_verification.assurance_ceiling() != InstallAssurance::Verified
                || row.install_topology_support.assurance_ceiling() != InstallAssurance::Verified
                || row.portable_state_freshness.assurance_ceiling() != InstallAssurance::Verified
                || row.sync_device_state.assurance_ceiling() != InstallAssurance::Verified
                || row.auth_recovery_posture.assurance_ceiling() != InstallAssurance::Verified
                || row.capability_floor() != InstallAssurance::Verified
                || row.local_continuity != LocalContinuity::Authoritative
                || !row.downgrade_reasons.is_empty()
                || !row.caveats.is_empty()
                || !row.stale_or_missing_fields.is_empty()
                || row.downgrade_path.is_offered())
        {
            violations.push(
                M5InstallPortabilityGovernanceViolation::VerifiedLaneNotWhole {
                    lane_id: row.lane_id.clone(),
                },
            );
        }
    }
}

/// A validation violation for the M5 install-and-portability governance packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5InstallPortabilityGovernanceViolation {
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
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A governance-row id appears more than once.
    DuplicateLaneId {
        /// Duplicate lane id.
        lane_id: String,
    },
    /// A claimed lane carries more than one row.
    DuplicateLaneRow {
        /// Lane token.
        lane: &'static str,
    },
    /// A claimed lane has no row.
    MissingLaneRow {
        /// Lane token.
        lane: &'static str,
    },
    /// A row covers a lane the packet does not claim.
    UnclaimedLaneRow {
        /// Row id.
        lane_id: String,
        /// Lane token.
        lane: &'static str,
    },
    /// A row's source packet is not the canonical packet for its lane.
    SourcePacketMismatch {
        /// Row id.
        lane_id: String,
        /// Expected source-packet path.
        expected: &'static str,
    },
    /// A row's install mode is not the canonical mode for its lane.
    InstallModeMismatch {
        /// Row id.
        lane_id: String,
        /// Expected install-mode token.
        expected: &'static str,
    },
    /// A row lists a downgrade reason more than once.
    DuplicateDowngradeReason {
        /// Row id.
        lane_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A lane publishes a label beyond what its evidence supports.
    OverstatedClaim {
        /// Row id.
        lane_id: String,
        /// Published label token.
        published: &'static str,
        /// Computed effective label token.
        computed: &'static str,
    },
    /// A lane's admission outcome disagrees with its gate decision.
    OutcomeMismatch {
        /// Row id.
        lane_id: String,
        /// Declared outcome token.
        declared: &'static str,
        /// Required outcome token.
        required: &'static str,
    },
    /// A lane's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Row id.
        lane_id: String,
    },
    /// A narrowed or refused lane offers no recovery path.
    MissingDowngradePath {
        /// Row id.
        lane_id: String,
    },
    /// A lane's local-continuity state disagrees with its sync and auth posture.
    LocalContinuityMismatch {
        /// Row id.
        lane_id: String,
        /// Local-continuity token.
        continuity: &'static str,
    },
    /// A verified lane still narrows a state, degrades local continuity, or carries a downgrade
    /// reason.
    VerifiedLaneNotWhole {
        /// Row id.
        lane_id: String,
    },
    /// A required consumer surface has no binding.
    MissingConsumerBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A consumer binding drops or remints governance truth.
    ConsumerBindingDrift {
        /// Binding ref.
        binding_ref: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5InstallPortabilityGovernanceViolation {
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
            Self::DuplicateLaneId { lane_id } => {
                write!(f, "duplicate lane id {lane_id}")
            }
            Self::DuplicateLaneRow { lane } => {
                write!(f, "duplicate row for lane {lane}")
            }
            Self::MissingLaneRow { lane } => {
                write!(f, "missing row for claimed lane {lane}")
            }
            Self::UnclaimedLaneRow { lane_id, lane } => {
                write!(f, "row {lane_id} covers unclaimed lane {lane}")
            }
            Self::SourcePacketMismatch { lane_id, expected } => {
                write!(
                    f,
                    "row {lane_id} packet_ref must be the canonical source packet {expected}"
                )
            }
            Self::InstallModeMismatch { lane_id, expected } => {
                write!(
                    f,
                    "row {lane_id} install_mode must be the canonical mode {expected}"
                )
            }
            Self::DuplicateDowngradeReason { lane_id, reason } => {
                write!(f, "row {lane_id} repeats downgrade reason {reason}")
            }
            Self::OverstatedClaim {
                lane_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "row {lane_id} publishes label {published} but the gate computes {computed}"
                )
            }
            Self::OutcomeMismatch {
                lane_id,
                declared,
                required,
            } => {
                write!(
                    f,
                    "row {lane_id} records outcome {declared} but the gate requires {required}"
                )
            }
            Self::DowngradeReasonsMismatch { lane_id } => {
                write!(f, "row {lane_id} downgrade reasons disagree with the gate")
            }
            Self::MissingDowngradePath { lane_id } => {
                write!(
                    f,
                    "row {lane_id} is narrowed or refused but offers no recovery path"
                )
            }
            Self::LocalContinuityMismatch {
                lane_id,
                continuity,
            } => {
                write!(
                    f,
                    "row {lane_id} local continuity {continuity} disagrees with its sync and auth posture"
                )
            }
            Self::VerifiedLaneNotWhole { lane_id } => {
                write!(
                    f,
                    "row {lane_id} is verified but narrows a state, degrades local continuity, or carries a downgrade reason"
                )
            }
            Self::MissingConsumerBinding { surface } => {
                write!(f, "missing consumer binding for surface {surface}")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(
                    f,
                    "binding {binding_ref} does not preserve governance truth"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for M5InstallPortabilityGovernanceViolation {}

/// Stable record-kind tag for [`M5InstallPortabilityGovernanceSupportExport`].
pub const M5_INSTALL_PORTABILITY_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_install_and_portability_governance_support_export";

/// Support-export wrapper preserving the governance matrix verbatim for support and evidence
/// packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallPortabilityGovernanceSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub governance_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact governance matrix preserved by the export.
    pub governance_matrix: M5InstallPortabilityGovernanceMatrix,
}

impl M5InstallPortabilityGovernanceSupportExport {
    /// Whether the export preserves the same packet id and a clean matrix.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_INSTALL_PORTABILITY_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_INSTALL_PORTABILITY_GOVERNANCE_SCHEMA_VERSION
            && self.governance_packet_id_ref == self.governance_matrix.packet_id
            && self.raw_private_material_excluded
            && self.governance_matrix.validate().is_empty()
    }
}

/// Loads the embedded M5 install-and-portability governance matrix packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5InstallPortabilityGovernanceMatrix`].
pub fn current_m5_install_portability_governance_matrix(
) -> Result<M5InstallPortabilityGovernanceMatrix, serde_json::Error> {
    serde_json::from_str(M5_INSTALL_PORTABILITY_GOVERNANCE_JSON)
}

#[cfg(test)]
mod tests;
