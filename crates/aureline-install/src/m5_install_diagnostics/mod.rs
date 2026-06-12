//! Canonical M5 install-and-update diagnostics packet: one inspectable object that explains, for
//! each M5-added artifact family, where it lives, who owns its updates, and what rollback target
//! is still valid.
//!
//! Where the install-and-portability governance matrix freezes the per-lane assurance gate, this
//! packet answers the operator-and-support question that sits underneath it: for each M5 artifact
//! the desktop app, the companion surface, the marketplace/helper extension, the local-model
//! runtime, and the portable/export package what [`InstallMode`] installed it, on which
//! [`ChannelRing`], who owns its updates ([`UpdaterOwner`]), where its artifact, mutable-state,
//! and policy roots live ([`DiagnosticRoot`]), what its last verification state and freshness are
//! ([`InstallVerification`] / [`VerificationFreshness`]), and what rollback target it can still
//! fall back to ([`RollbackTargetState`]). One [`ArtifactDiagnosticRow`] carries all of that for
//! one artifact, and the same packet is the object desktop, CLI, support export, and About all
//! render instead of divergent hand-written summaries.
//!
//! The diagnostics gate keeps a rollout row from claiming support a topology cannot back. The
//! [`InstallAssurance`] an artifact may publish is the weakest ceiling implied by its observed
//! states an unverified binary, a stale verification, an unavailable rollback target, or a
//! governance lane that was itself narrowed all lower or withhold the published support
//! automatically. Each row is bound to the canonical governance lane it draws verification truth
//! from via [`ArtifactDiagnosticRow::governs_lane`], and [`ArtifactDiagnosticRow::governs_assurance`]
//! is validated against the embedded governance matrix, so an artifact can never publish support
//! beyond the lane the governance gate already narrowed. This realizes the rollout invariant: an
//! M5 topology with no current install diagnostics and verification state cannot claim support.
//!
//! Secrecy boundaries stay intact. Every [`DiagnosticRoot`] classifies its store by
//! [`RootSensitivity`] and role, but a [`RootSensitivity::SecretBearing`] or
//! [`RootSensitivity::MachineProtected`] root must be redacted, so the packet names *where* a
//! credential, policy, or machine-identity store is without dumping its contents, token values,
//! or machine-unique protected material. The packet is metadata-only: every field is a typed
//! state or an opaque ref.
//!
//! Troubleshooting drills are first-class. Each [`DiagnosticDrill`] replays one support incident
//! a wrong-root mismatch, a stale verification, a missing rollback target, or a wrong-root
//! support pointer and proves the diagnostics object detects it, so support can reproduce the
//! signal instead of guessing from a version string.
//!
//! Because every required consumer surface desktop, CLI, support export, and About binds to
//! this one packet via a [`DiagnosticsConsumerBinding`] that must ingest it, preserve its
//! published support and recovery paths, and narrow with it, an artifact narrowed here cannot
//! read as supported on an About panel, a CLI status line, or a support export.
//!
//! The packet is checked in at `artifacts/install/m5/m5-install-diagnostics.json` and embedded
//! here.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_install_and_portability_governance::{
    current_m5_install_portability_governance_matrix, ChannelRing, InstallAssurance,
    InstallConfigLane, InstallMode, InstallVerification,
};

/// Supported M5 install-and-update diagnostics schema version.
pub const M5_INSTALL_DIAGNOSTICS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_INSTALL_DIAGNOSTICS_RECORD_KIND: &str = "m5_install_update_diagnostics";

/// Repo-relative path to the checked-in packet.
pub const M5_INSTALL_DIAGNOSTICS_PATH: &str = "artifacts/install/m5/m5-install-diagnostics.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_INSTALL_DIAGNOSTICS_SCHEMA_REF: &str =
    "schemas/install/m5-install-diagnostics.schema.json";

/// Repo-relative path to the companion document.
pub const M5_INSTALL_DIAGNOSTICS_DOC_REF: &str = "docs/install/m5/m5-install-diagnostics.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_INSTALL_DIAGNOSTICS_ARTIFACT_DOC_REF: &str =
    "artifacts/install/m5/m5-install-diagnostics.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_INSTALL_DIAGNOSTICS_FIXTURE_DIR: &str = "fixtures/install/m5/m5-install-diagnostics";

/// Embedded checked-in packet JSON.
pub const M5_INSTALL_DIAGNOSTICS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/install/m5/m5-install-diagnostics.json"
));

/// An M5-added artifact family the diagnostics packet covers.
///
/// These are the artifact families M5 introduces beyond the primary app binary, so install and
/// update topology stays inspectable for each one rather than only for the desktop app.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ArtifactFamily {
    /// The first-party desktop application binary.
    DesktopApp,
    /// A companion surface paired to a host install.
    Companion,
    /// A marketplace or helper extension artifact.
    MarketplaceHelper,
    /// A local-model runtime and its on-disk weights.
    LocalModelRuntime,
    /// A portable or exported state package.
    PortableExport,
}

impl M5ArtifactFamily {
    /// Every M5 artifact family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DesktopApp,
        Self::Companion,
        Self::MarketplaceHelper,
        Self::LocalModelRuntime,
        Self::PortableExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopApp => "desktop_app",
            Self::Companion => "companion",
            Self::MarketplaceHelper => "marketplace_helper",
            Self::LocalModelRuntime => "local_model_runtime",
            Self::PortableExport => "portable_export",
        }
    }

    /// The canonical governance lane this artifact family draws its verification truth from.
    ///
    /// An artifact never publishes support beyond the governance lane it is pinned to, so the
    /// diagnostics object inherits the gate the governance matrix already applied.
    pub const fn governs_lane(self) -> InstallConfigLane {
        match self {
            Self::DesktopApp | Self::LocalModelRuntime => InstallConfigLane::DesktopStable,
            Self::Companion | Self::MarketplaceHelper => InstallConfigLane::MarketplaceCompanion,
            Self::PortableExport => InstallConfigLane::PortableInstall,
        }
    }
}

/// Who owns updates for an artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdaterOwner {
    /// First-party auto-updater owns the update.
    FirstPartyAuto,
    /// A managed fleet controller owns the update.
    ManagedFleet,
    /// A marketplace host owns the update.
    MarketplaceHost,
    /// An operating-system package manager owns the update.
    OsPackageManager,
    /// The user applies updates manually.
    ManualUser,
    /// No updater owns this artifact.
    #[serde(rename = "none")]
    NoneOwned,
}

impl UpdaterOwner {
    /// Every updater owner, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FirstPartyAuto,
        Self::ManagedFleet,
        Self::MarketplaceHost,
        Self::OsPackageManager,
        Self::ManualUser,
        Self::NoneOwned,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyAuto => "first_party_auto",
            Self::ManagedFleet => "managed_fleet",
            Self::MarketplaceHost => "marketplace_host",
            Self::OsPackageManager => "os_package_manager",
            Self::ManualUser => "manual_user",
            Self::NoneOwned => "none",
        }
    }
}

/// The category bucket a [`DiagnosticRoot`] is recorded under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootCategory {
    /// Read-mostly artifact and resource roots.
    Artifact,
    /// Mutable durable-state, cache, log, and credential roots.
    MutableState,
    /// Policy and machine-identity roots.
    Policy,
}

/// The role a diagnostics root plays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootRole {
    /// The installed executable root.
    ArtifactBinary,
    /// A read-mostly resource or asset bundle root.
    ResourceBundle,
    /// A mutable durable-state root.
    MutableState,
    /// A regenerable cache root.
    Cache,
    /// A diagnostics or log root.
    Log,
    /// A credential or token store root.
    CredentialStore,
    /// A policy or configuration-enforcement root.
    PolicyStore,
    /// A machine-identity or device-binding root.
    MachineIdentity,
}

impl RootRole {
    /// Every root role, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ArtifactBinary,
        Self::ResourceBundle,
        Self::MutableState,
        Self::Cache,
        Self::Log,
        Self::CredentialStore,
        Self::PolicyStore,
        Self::MachineIdentity,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactBinary => "artifact_binary",
            Self::ResourceBundle => "resource_bundle",
            Self::MutableState => "mutable_state",
            Self::Cache => "cache",
            Self::Log => "log",
            Self::CredentialStore => "credential_store",
            Self::PolicyStore => "policy_store",
            Self::MachineIdentity => "machine_identity",
        }
    }

    /// The root-list category this role must be recorded under.
    pub const fn category(self) -> RootCategory {
        match self {
            Self::ArtifactBinary | Self::ResourceBundle => RootCategory::Artifact,
            Self::MutableState | Self::Cache | Self::Log | Self::CredentialStore => {
                RootCategory::MutableState
            }
            Self::PolicyStore | Self::MachineIdentity => RootCategory::Policy,
        }
    }
}

/// How sensitive a diagnostics root's contents are.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootSensitivity {
    /// A public, non-sensitive path label.
    PublicPath,
    /// A user-scoped path with no secret material.
    UserScoped,
    /// A machine-protected store; must be redacted.
    MachineProtected,
    /// A secret- or credential-bearing store; must be redacted.
    SecretBearing,
}

impl RootSensitivity {
    /// Every root sensitivity, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PublicPath,
        Self::UserScoped,
        Self::MachineProtected,
        Self::SecretBearing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicPath => "public_path",
            Self::UserScoped => "user_scoped",
            Self::MachineProtected => "machine_protected",
            Self::SecretBearing => "secret_bearing",
        }
    }

    /// Whether a root at this sensitivity must be redacted, so its contents are never dumped.
    pub const fn requires_redaction(self) -> bool {
        matches!(self, Self::MachineProtected | Self::SecretBearing)
    }
}

/// How fresh an artifact's last verification is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationFreshness {
    /// Verified against the current build.
    Current,
    /// Verified but aging within tolerance; bounded.
    Aging,
    /// The verification is stale; needs retest.
    Stale,
    /// The artifact has never been verified.
    NeverVerified,
}

impl VerificationFreshness {
    /// Every verification-freshness state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Current, Self::Aging, Self::Stale, Self::NeverVerified];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Aging => "aging",
            Self::Stale => "stale",
            Self::NeverVerified => "never_verified",
        }
    }

    /// Highest support this freshness permits an artifact to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::Current => InstallAssurance::Verified,
            Self::Aging => InstallAssurance::Bounded,
            Self::Stale => InstallAssurance::RetestPending,
            Self::NeverVerified => InstallAssurance::Withheld,
        }
    }

    /// Whether this state raises the [`DiagnosticNarrowReason::StaleVerification`] trigger.
    pub const fn is_stale_trigger(self) -> bool {
        !matches!(self, Self::Current)
    }
}

/// What rollback target an artifact can still fall back to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackTargetState {
    /// A verified prior build is retained and applicable.
    Available,
    /// A prior build is retained but bounded to a slice; bounded.
    AvailableBounded,
    /// The retained prior build has expired; needs retest.
    Expired,
    /// No rollback target is retained.
    Missing,
}

impl RollbackTargetState {
    /// Every rollback-target state, in declaration order.
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

    /// Highest support this rollback-target state permits an artifact to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::Available => InstallAssurance::Verified,
            Self::AvailableBounded => InstallAssurance::Bounded,
            Self::Expired => InstallAssurance::RetestPending,
            Self::Missing => InstallAssurance::Withheld,
        }
    }

    /// Whether this state raises the [`DiagnosticNarrowReason::RollbackUnavailable`] trigger.
    pub const fn is_unavailable_trigger(self) -> bool {
        !matches!(self, Self::Available)
    }
}

/// A headline reason the diagnostics gate narrows an artifact's published support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticNarrowReason {
    /// The artifact's binary is not signed and verified.
    UnverifiedArtifact,
    /// The artifact's last verification is aging, stale, or missing.
    StaleVerification,
    /// The artifact has no available rollback target.
    RollbackUnavailable,
    /// The governance lane this artifact is pinned to was itself narrowed.
    GovernanceNarrowed,
}

impl DiagnosticNarrowReason {
    /// Every narrow reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::UnverifiedArtifact,
        Self::StaleVerification,
        Self::RollbackUnavailable,
        Self::GovernanceNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnverifiedArtifact => "unverified_artifact",
            Self::StaleVerification => "stale_verification",
            Self::RollbackUnavailable => "rollback_unavailable",
            Self::GovernanceNarrowed => "governance_narrowed",
        }
    }
}

/// The recovery path surfaced when an artifact's support is narrowed or withheld.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticRecoveryPath {
    /// Re-verify the artifact's signature before widening trust.
    ReverifyArtifact,
    /// Refresh the stale verification record.
    RefreshVerification,
    /// Restore a valid rollback target.
    RestoreRollbackTarget,
    /// Follow the governance lane's own recovery path.
    FollowGovernanceRecovery,
    /// Withhold the artifact's support claim from publication.
    WithholdClaim,
    /// No recovery is needed; only valid for a verified artifact.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl DiagnosticRecoveryPath {
    /// Every recovery path, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReverifyArtifact,
        Self::RefreshVerification,
        Self::RestoreRollbackTarget,
        Self::FollowGovernanceRecovery,
        Self::WithholdClaim,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReverifyArtifact => "reverify_artifact",
            Self::RefreshVerification => "refresh_verification",
            Self::RestoreRollbackTarget => "restore_rollback_target",
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

/// A support incident a troubleshooting drill replays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticIncident {
    /// An artifact resolved under an unexpected root.
    RootMismatch,
    /// An artifact's last verification is stale.
    StaleVerification,
    /// An artifact has no valid rollback target.
    MissingRollbackTarget,
    /// Support inspected the wrong state root for an artifact.
    WrongRootSupport,
}

impl DiagnosticIncident {
    /// Every support incident a drill must cover, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::RootMismatch,
        Self::StaleVerification,
        Self::MissingRollbackTarget,
        Self::WrongRootSupport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RootMismatch => "root_mismatch",
            Self::StaleVerification => "stale_verification",
            Self::MissingRollbackTarget => "missing_rollback_target",
            Self::WrongRootSupport => "wrong_root_support",
        }
    }
}

/// A consumer surface that must ingest this diagnostics packet and narrow with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticsConsumer {
    /// The desktop install-and-update diagnostics surface.
    Desktop,
    /// The command-line install-and-status surface.
    Cli,
    /// The Help/About panel.
    About,
    /// The support-export bundle.
    SupportExport,
}

impl DiagnosticsConsumer {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 4] = [Self::Desktop, Self::Cli, Self::About, Self::SupportExport];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Cli => "cli",
            Self::About => "about",
            Self::SupportExport => "support_export",
        }
    }
}

/// One classified install root carried by an artifact diagnostics row.
///
/// The label names *where* a store is, never its contents: a [`RootSensitivity::SecretBearing`]
/// or [`RootSensitivity::MachineProtected`] root must be redacted, so the packet classifies
/// credential, policy, and machine-identity stores without dumping secrets, token values, or
/// machine-unique protected material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticRoot {
    /// Role this root plays.
    pub role: RootRole,
    /// Opaque, human-readable location label never a raw secret or sensitive path.
    pub location_label: String,
    /// Sensitivity of this root's contents.
    pub sensitivity: RootSensitivity,
    /// Whether this root's contents are redacted from the packet.
    pub redacted: bool,
    /// Whether this root is writable by the artifact at runtime.
    pub writable: bool,
}

impl DiagnosticRoot {
    /// Whether this root honors its redaction requirement.
    ///
    /// A machine-protected or secret-bearing root must be redacted.
    pub fn redaction_ok(&self) -> bool {
        !self.sensitivity.requires_redaction() || self.redacted
    }
}

/// One install-and-update diagnostics row for an M5 artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactDiagnosticRow {
    /// Stable diagnostics-row id.
    pub artifact_id: String,
    /// Artifact family this row covers.
    pub artifact_family: M5ArtifactFamily,
    /// Human-readable artifact name.
    pub display_name: String,
    /// How this artifact was installed.
    pub install_mode: InstallMode,
    /// Channel-and-ring this artifact installs from.
    pub channel: ChannelRing,
    /// Who owns updates for this artifact.
    pub updater_owner: UpdaterOwner,
    /// Owner accountable for the artifact's diagnostics.
    pub owner: String,
    /// Read-mostly artifact and resource roots.
    pub artifact_roots: Vec<DiagnosticRoot>,
    /// Mutable durable-state, cache, log, and credential roots.
    pub mutable_state_roots: Vec<DiagnosticRoot>,
    /// Policy and machine-identity roots.
    #[serde(default)]
    pub policy_roots: Vec<DiagnosticRoot>,
    /// Last verification state of the artifact's binary.
    pub verification_state: InstallVerification,
    /// How fresh the last verification is.
    pub verification_freshness: VerificationFreshness,
    /// Opaque ref to the verification evidence.
    pub verification_evidence_ref: String,
    /// Rollback target this artifact can fall back to.
    pub rollback_target: RollbackTargetState,
    /// Opaque ref to the retained rollback build.
    pub rollback_target_ref: String,
    /// Governance lane this artifact draws verification truth from; must equal
    /// [`M5ArtifactFamily::governs_lane`].
    pub governs_lane: InstallConfigLane,
    /// Published assurance of the governance lane, snapshotted from the governance matrix.
    pub governs_assurance: InstallAssurance,
    /// Support the artifact's own diagnostics assert, before the gate.
    pub declared_support: InstallAssurance,
    /// Support actually published after the gate narrows the artifact.
    ///
    /// Must equal [`ArtifactDiagnosticRow::effective_support`].
    pub published_support: InstallAssurance,
    /// Headline narrow reasons; must equal the recomputed set.
    #[serde(default)]
    pub narrow_reasons: Vec<DiagnosticNarrowReason>,
    /// Recovery path surfaced when support is narrowed; must equal the recomputed path.
    pub recovery_path: DiagnosticRecoveryPath,
    /// Scope or slice labels this artifact still backs.
    #[serde(default)]
    pub supported_scopes: Vec<String>,
    /// Caveats attached to the published support.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale, missing, or narrowing the support.
    #[serde(default)]
    pub stale_or_missing_fields: Vec<String>,
    /// Ref binding this row into the diagnostics surface.
    pub diagnostics_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl ArtifactDiagnosticRow {
    /// The support label the artifact's own diagnostics asserted, before narrowing.
    pub fn capability_floor(&self) -> InstallAssurance {
        self.declared_support
    }

    /// The support label the gate permits this artifact to publish.
    ///
    /// Lowers the capability floor to the weakest ceiling implied by the verification state,
    /// verification freshness, rollback-target state, and the governance lane's published
    /// assurance, so an unverified binary, a stale verification, a missing rollback target, or a
    /// governance-narrowed lane can never publish a verified label.
    pub fn effective_support(&self) -> InstallAssurance {
        self.capability_floor()
            .min(self.verification_state.assurance_ceiling())
            .min(self.verification_freshness.assurance_ceiling())
            .min(self.rollback_target.assurance_ceiling())
            .min(self.governs_assurance)
    }

    /// The headline narrow reasons recomputed from the artifact's observed states.
    pub fn computed_narrow_reasons(&self) -> Vec<DiagnosticNarrowReason> {
        let mut reasons = Vec::new();
        if self.verification_state.is_unverified_trigger() {
            reasons.push(DiagnosticNarrowReason::UnverifiedArtifact);
        }
        if self.verification_freshness.is_stale_trigger() {
            reasons.push(DiagnosticNarrowReason::StaleVerification);
        }
        if self.rollback_target.is_unavailable_trigger() {
            reasons.push(DiagnosticNarrowReason::RollbackUnavailable);
        }
        if self.governs_assurance != InstallAssurance::Verified {
            reasons.push(DiagnosticNarrowReason::GovernanceNarrowed);
        }
        reasons
    }

    /// The recovery path the gate surfaces for this artifact, derived from its observed states.
    pub fn computed_recovery_path(&self) -> DiagnosticRecoveryPath {
        if self.effective_support() == InstallAssurance::Withheld {
            DiagnosticRecoveryPath::WithholdClaim
        } else if self.governs_assurance != InstallAssurance::Verified {
            DiagnosticRecoveryPath::FollowGovernanceRecovery
        } else if self.rollback_target.is_unavailable_trigger() {
            DiagnosticRecoveryPath::RestoreRollbackTarget
        } else if self.verification_freshness.is_stale_trigger() {
            DiagnosticRecoveryPath::RefreshVerification
        } else if self.verification_state.is_unverified_trigger() {
            DiagnosticRecoveryPath::ReverifyArtifact
        } else {
            DiagnosticRecoveryPath::NoneNeeded
        }
    }

    /// Whether the artifact publishes a clean verified label.
    pub fn is_verified(&self) -> bool {
        self.effective_support() == InstallAssurance::Verified
    }

    /// Whether the gate narrowed the published support below what the artifact declared.
    pub fn is_downgraded(&self) -> bool {
        self.effective_support().rank() < self.capability_floor().rank()
    }

    /// Every root the row carries, across all three categories.
    pub fn all_roots(&self) -> impl Iterator<Item = &DiagnosticRoot> {
        self.artifact_roots
            .iter()
            .chain(self.mutable_state_roots.iter())
            .chain(self.policy_roots.iter())
    }

    /// Whether every root honors its redaction requirement.
    pub fn roots_redaction_ok(&self) -> bool {
        self.all_roots().all(DiagnosticRoot::redaction_ok)
    }

    /// Whether the stored published support, narrow reasons, and recovery path all agree with the
    /// recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_support == self.effective_support()
            && self.narrow_reasons == self.computed_narrow_reasons()
            && self.recovery_path == self.computed_recovery_path()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5InstallDiagnosticsSummary {
    /// Total artifact rows.
    pub total_artifacts: usize,
    /// Number of claimed artifact families.
    pub family_count: usize,
    /// Artifacts published as verified.
    pub verified_artifacts: usize,
    /// Artifacts narrowed to a bounded label.
    pub bounded_artifacts: usize,
    /// Artifacts narrowed to a retest-pending label.
    pub retest_pending_artifacts: usize,
    /// Artifacts withheld from publication.
    pub withheld_artifacts: usize,
    /// Artifacts whose published support was downgraded below what they declared.
    pub downgraded_artifacts: usize,
    /// Artifacts whose binary is not signed and verified.
    pub unverified_artifacts: usize,
    /// Artifacts whose last verification is aging, stale, or missing.
    pub stale_verification_artifacts: usize,
    /// Artifacts with no available rollback target.
    pub rollback_unavailable_artifacts: usize,
    /// Artifacts narrowed because their governance lane was narrowed.
    pub governance_narrowed_artifacts: usize,
    /// Total roots across all artifacts.
    pub total_roots: usize,
    /// Roots redacted because they are machine-protected or secret-bearing.
    pub redacted_roots: usize,
    /// Total troubleshooting drills.
    pub drill_count: usize,
    /// Drills that detect their incident.
    pub detected_drill_count: usize,
}

/// A troubleshooting drill that replays one support incident and proves the diagnostics object
/// detects it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Support incident this drill replays.
    pub incident: DiagnosticIncident,
    /// Artifact the drill targets; must reference a claimed row.
    pub artifact_id: String,
    /// Reviewer-facing scenario summary.
    pub scenario: String,
    /// Signal a healthy artifact would emit.
    pub expected_signal: String,
    /// Signal the incident emits.
    pub observed_signal: String,
    /// Whether the diagnostics object detects this incident; must be true.
    pub detected: bool,
    /// Recovery path the drill points the operator to.
    pub remediation: DiagnosticRecoveryPath,
    /// Resolution the drill proves.
    pub resolves_to: String,
}

/// One binding wiring a consumer surface to this diagnostics packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticsConsumerBinding {
    /// Consumer surface this binding wires.
    pub consumer: DiagnosticsConsumer,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Diagnostics packet id this surface ingests.
    pub diagnostics_packet_id_ref: String,
    /// True when the surface ingests this packet rather than a parallel summary.
    pub ingests_diagnostics: bool,
    /// True when the surface preserves the published support verbatim.
    pub preserves_published_support: bool,
    /// True when the surface preserves the recovery paths verbatim.
    pub preserves_recovery_paths: bool,
    /// True when the surface narrows automatically as artifacts are downgraded.
    pub narrows_on_downgrade: bool,
    /// True when raw private material is excluded from the binding.
    pub raw_private_material_excluded: bool,
}

impl DiagnosticsConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.diagnostics_packet_id_ref == packet_id
            && self.ingests_diagnostics
            && self.preserves_published_support
            && self.preserves_recovery_paths
            && self.narrows_on_downgrade
            && self.raw_private_material_excluded
            && !self.binding_ref.trim().is_empty()
    }
}

/// A redaction-safe export row projected from a diagnostics row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallDiagnosticsExportRow {
    /// Diagnostics-row id.
    pub artifact_id: String,
    /// Artifact-family token.
    pub artifact_family: String,
    /// Human-readable artifact name.
    pub display_name: String,
    /// Install-mode token.
    pub install_mode: String,
    /// Channel-and-ring token.
    pub channel: String,
    /// Updater-owner token.
    pub updater_owner: String,
    /// Owner accountable for the artifact.
    pub owner: String,
    /// Verification-state token.
    pub verification_state: String,
    /// Verification-freshness token.
    pub verification_freshness: String,
    /// Rollback-target token.
    pub rollback_target: String,
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
    /// Fields whose evidence is stale or missing.
    pub stale_or_missing_fields: Vec<String>,
    /// Count of artifact roots.
    pub artifact_root_count: usize,
    /// Count of mutable-state roots.
    pub mutable_state_root_count: usize,
    /// Count of policy roots.
    pub policy_root_count: usize,
    /// Count of redacted roots.
    pub redacted_root_count: usize,
    /// Whether the artifact publishes a verified label.
    pub verified: bool,
    /// Whether the published support was downgraded below the declared support.
    pub downgraded: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet the canonical install-diagnostics index
/// downstream surfaces render instead of restating each artifact's topology by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallDiagnosticsExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub artifacts: Vec<M5InstallDiagnosticsExportRow>,
    /// Whether every artifact's published support and recovery path agree with the gate.
    pub all_artifacts_gate_consistent: bool,
    /// Artifacts that publish a verified label.
    pub verified_count: usize,
    /// Artifacts the gate narrowed below their declared support.
    pub downgraded_count: usize,
    /// Artifacts the gate withheld entirely.
    pub withheld_count: usize,
}

/// The typed M5 install-and-update diagnostics packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5InstallUpdateDiagnostics {
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
    /// Scheme the packet mints stable diagnostics identities under.
    pub diagnostics_identity_scheme: String,
    /// Governance matrix packet id this diagnostics packet is bound to.
    pub governance_packet_id_ref: String,
    /// Claimed artifact families; one row per family.
    pub artifact_families: Vec<M5ArtifactFamily>,
    /// Closed install-mode vocabulary.
    pub install_modes: Vec<InstallMode>,
    /// Closed channel-and-ring vocabulary.
    pub channels: Vec<ChannelRing>,
    /// Closed updater-owner vocabulary.
    pub updater_owners: Vec<UpdaterOwner>,
    /// Closed root-role vocabulary.
    pub root_roles: Vec<RootRole>,
    /// Closed root-sensitivity vocabulary.
    pub root_sensitivities: Vec<RootSensitivity>,
    /// Closed verification-state vocabulary.
    pub verification_states: Vec<InstallVerification>,
    /// Closed verification-freshness vocabulary.
    pub verification_freshness_states: Vec<VerificationFreshness>,
    /// Closed rollback-target vocabulary.
    pub rollback_target_states: Vec<RollbackTargetState>,
    /// Closed assurance-label vocabulary.
    pub assurance_labels: Vec<InstallAssurance>,
    /// Closed narrow-reason vocabulary.
    pub narrow_reasons: Vec<DiagnosticNarrowReason>,
    /// Closed recovery-path vocabulary.
    pub recovery_paths: Vec<DiagnosticRecoveryPath>,
    /// Closed incident vocabulary.
    pub incidents: Vec<DiagnosticIncident>,
    /// Closed consumer vocabulary.
    pub consumers: Vec<DiagnosticsConsumer>,
    /// Diagnostics rows, one per claimed artifact family.
    #[serde(default)]
    pub artifacts: Vec<ArtifactDiagnosticRow>,
    /// Troubleshooting drills, one per required incident.
    #[serde(default)]
    pub drills: Vec<DiagnosticDrill>,
    /// Consumer bindings, one per required surface.
    #[serde(default)]
    pub consumer_bindings: Vec<DiagnosticsConsumerBinding>,
    /// Summary counts.
    pub summary: M5InstallDiagnosticsSummary,
}

impl M5InstallUpdateDiagnostics {
    /// Returns the row for a claimed artifact family.
    pub fn artifact_row(&self, family: M5ArtifactFamily) -> Option<&ArtifactDiagnosticRow> {
        self.artifacts.iter().find(|a| a.artifact_family == family)
    }

    /// Returns the row with the given stable artifact id.
    pub fn artifact_row_by_id(&self, artifact_id: &str) -> Option<&ArtifactDiagnosticRow> {
        self.artifacts.iter().find(|a| a.artifact_id == artifact_id)
    }

    /// Artifacts that publish a verified label.
    pub fn verified_artifacts(&self) -> impl Iterator<Item = &ArtifactDiagnosticRow> {
        self.artifacts.iter().filter(|a| a.is_verified())
    }

    /// Artifacts the gate narrowed below their declared support.
    pub fn downgraded_artifacts(&self) -> impl Iterator<Item = &ArtifactDiagnosticRow> {
        self.artifacts.iter().filter(|a| a.is_downgraded())
    }

    /// Whether a consumer binding preserves this packet for the given surface.
    pub fn has_binding_for(&self, consumer: DiagnosticsConsumer) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer == consumer && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every artifact's stored support, reasons, and recovery path agree with the gate.
    pub fn all_artifacts_gate_consistent(&self) -> bool {
        self.artifacts.iter().all(|a| a.gate_consistent())
    }

    /// Recomputes the summary block from the rows and drills.
    pub fn computed_summary(&self) -> M5InstallDiagnosticsSummary {
        let count_published = |label: InstallAssurance| {
            self.artifacts
                .iter()
                .filter(|a| a.published_support == label)
                .count()
        };
        let total_roots: usize = self.artifacts.iter().map(|a| a.all_roots().count()).sum();
        let redacted_roots: usize = self
            .artifacts
            .iter()
            .flat_map(|a| a.all_roots())
            .filter(|r| r.redacted)
            .count();
        M5InstallDiagnosticsSummary {
            total_artifacts: self.artifacts.len(),
            family_count: self.artifact_families.len(),
            verified_artifacts: count_published(InstallAssurance::Verified),
            bounded_artifacts: count_published(InstallAssurance::Bounded),
            retest_pending_artifacts: count_published(InstallAssurance::RetestPending),
            withheld_artifacts: count_published(InstallAssurance::Withheld),
            downgraded_artifacts: self.artifacts.iter().filter(|a| a.is_downgraded()).count(),
            unverified_artifacts: self
                .artifacts
                .iter()
                .filter(|a| a.verification_state.is_unverified_trigger())
                .count(),
            stale_verification_artifacts: self
                .artifacts
                .iter()
                .filter(|a| a.verification_freshness.is_stale_trigger())
                .count(),
            rollback_unavailable_artifacts: self
                .artifacts
                .iter()
                .filter(|a| a.rollback_target.is_unavailable_trigger())
                .count(),
            governance_narrowed_artifacts: self
                .artifacts
                .iter()
                .filter(|a| a.governs_assurance != InstallAssurance::Verified)
                .count(),
            total_roots,
            redacted_roots,
            drill_count: self.drills.len(),
            detected_drill_count: self.drills.iter().filter(|d| d.detected).count(),
        }
    }

    /// Produces the install-diagnostics index downstream surfaces desktop, CLI, support
    /// export, and About render instead of restating each artifact's topology by hand.
    pub fn export_projection(&self) -> M5InstallDiagnosticsExportProjection {
        let artifacts = self
            .artifacts
            .iter()
            .map(|a| M5InstallDiagnosticsExportRow {
                artifact_id: a.artifact_id.clone(),
                artifact_family: a.artifact_family.as_str().to_owned(),
                display_name: a.display_name.clone(),
                install_mode: a.install_mode.as_str().to_owned(),
                channel: a.channel.as_str().to_owned(),
                updater_owner: a.updater_owner.as_str().to_owned(),
                owner: a.owner.clone(),
                verification_state: a.verification_state.as_str().to_owned(),
                verification_freshness: a.verification_freshness.as_str().to_owned(),
                rollback_target: a.rollback_target.as_str().to_owned(),
                governs_lane: a.governs_lane.as_str().to_owned(),
                governs_assurance: a.governs_assurance.as_str().to_owned(),
                declared_support: a.declared_support.as_str().to_owned(),
                published_support: a.published_support.as_str().to_owned(),
                narrow_reasons: a
                    .narrow_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                recovery_path: a.recovery_path.as_str().to_owned(),
                supported_scopes: a.supported_scopes.clone(),
                caveats: a.caveats.clone(),
                stale_or_missing_fields: a.stale_or_missing_fields.clone(),
                artifact_root_count: a.artifact_roots.len(),
                mutable_state_root_count: a.mutable_state_roots.len(),
                policy_root_count: a.policy_roots.len(),
                redacted_root_count: a.all_roots().filter(|r| r.redacted).count(),
                verified: a.is_verified(),
                downgraded: a.is_downgraded(),
                summary: format!(
                    "{} ({} / {}): updater {}, verification {} ({}), rollback {}, governs {} ({}), declared {}, published {}, recovery {}",
                    a.artifact_family.as_str(),
                    a.install_mode.as_str(),
                    a.channel.as_str(),
                    a.updater_owner.as_str(),
                    a.verification_state.as_str(),
                    a.verification_freshness.as_str(),
                    a.rollback_target.as_str(),
                    a.governs_lane.as_str(),
                    a.governs_assurance.as_str(),
                    a.declared_support.as_str(),
                    a.published_support.as_str(),
                    a.recovery_path.as_str()
                ),
            })
            .collect();
        M5InstallDiagnosticsExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            artifacts,
            all_artifacts_gate_consistent: self.all_artifacts_gate_consistent(),
            verified_count: self.verified_artifacts().count(),
            downgraded_count: self.downgraded_artifacts().count(),
            withheld_count: self
                .artifacts
                .iter()
                .filter(|a| a.published_support == InstallAssurance::Withheld)
                .count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact diagnostics packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5InstallDiagnosticsSupportExport {
        M5InstallDiagnosticsSupportExport {
            record_kind: M5_INSTALL_DIAGNOSTICS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_INSTALL_DIAGNOSTICS_SCHEMA_VERSION,
            export_id: export_id.into(),
            diagnostics_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            diagnostics: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5InstallDiagnosticsViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        // Bind to the governance matrix: the governance packet id and each artifact's snapshotted
        // governance assurance must match the embedded governance matrix, so an artifact can never
        // publish support beyond the lane the governance gate already narrowed.
        let governance = current_m5_install_portability_governance_matrix().ok();
        match &governance {
            Some(matrix) => {
                if self.governance_packet_id_ref != matrix.packet_id {
                    violations.push(M5InstallDiagnosticsViolation::GovernancePacketMismatch {
                        actual: self.governance_packet_id_ref.clone(),
                        expected: matrix.packet_id.clone(),
                    });
                }
            }
            None => violations.push(M5InstallDiagnosticsViolation::GovernanceMatrixUnavailable),
        }

        let claimed: BTreeSet<M5ArtifactFamily> = self.artifact_families.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.artifacts {
            if !seen_ids.insert(row.artifact_id.clone()) {
                violations.push(M5InstallDiagnosticsViolation::DuplicateArtifactId {
                    artifact_id: row.artifact_id.clone(),
                });
            }
            if !seen_families.insert(row.artifact_family) {
                violations.push(M5InstallDiagnosticsViolation::DuplicateArtifactRow {
                    family: row.artifact_family.as_str(),
                });
            }
            if !claimed.contains(&row.artifact_family) {
                violations.push(M5InstallDiagnosticsViolation::UnclaimedArtifactRow {
                    artifact_id: row.artifact_id.clone(),
                    family: row.artifact_family.as_str(),
                });
            }
            self.validate_row(row, governance.as_ref(), &mut violations);
        }

        // Every claimed family must carry its own row, so an artifact never inherits a verified
        // label from an adjacent one.
        for &family in &self.artifact_families {
            if !seen_families.contains(&family) {
                violations.push(M5InstallDiagnosticsViolation::MissingArtifactRow {
                    family: family.as_str(),
                });
            }
        }

        self.validate_drills(&seen_ids, &mut violations);

        // Every required consumer surface must bind to this packet and narrow with it.
        for consumer in DiagnosticsConsumer::REQUIRED {
            if !self.has_binding_for(consumer) {
                violations.push(M5InstallDiagnosticsViolation::MissingConsumerBinding {
                    consumer: consumer.as_str(),
                });
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(M5InstallDiagnosticsViolation::ConsumerBindingDrift {
                    binding_ref: binding.binding_ref.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5InstallDiagnosticsViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5InstallDiagnosticsViolation>) {
        if self.schema_version != M5_INSTALL_DIAGNOSTICS_SCHEMA_VERSION {
            violations.push(M5InstallDiagnosticsViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_INSTALL_DIAGNOSTICS_RECORD_KIND {
            violations.push(M5InstallDiagnosticsViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            (
                "diagnostics_identity_scheme",
                &self.diagnostics_identity_scheme,
            ),
            ("governance_packet_id_ref", &self.governance_packet_id_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5InstallDiagnosticsViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "artifact_families",
                self.artifact_families == M5ArtifactFamily::ALL.to_vec(),
            ),
            (
                "install_modes",
                self.install_modes == InstallMode::ALL.to_vec(),
            ),
            ("channels", self.channels == ChannelRing::ALL.to_vec()),
            (
                "updater_owners",
                self.updater_owners == UpdaterOwner::ALL.to_vec(),
            ),
            ("root_roles", self.root_roles == RootRole::ALL.to_vec()),
            (
                "root_sensitivities",
                self.root_sensitivities == RootSensitivity::ALL.to_vec(),
            ),
            (
                "verification_states",
                self.verification_states == InstallVerification::ALL.to_vec(),
            ),
            (
                "verification_freshness_states",
                self.verification_freshness_states == VerificationFreshness::ALL.to_vec(),
            ),
            (
                "rollback_target_states",
                self.rollback_target_states == RollbackTargetState::ALL.to_vec(),
            ),
            (
                "assurance_labels",
                self.assurance_labels == InstallAssurance::ALL.to_vec(),
            ),
            (
                "narrow_reasons",
                self.narrow_reasons == DiagnosticNarrowReason::ALL.to_vec(),
            ),
            (
                "recovery_paths",
                self.recovery_paths == DiagnosticRecoveryPath::ALL.to_vec(),
            ),
            (
                "incidents",
                self.incidents == DiagnosticIncident::REQUIRED.to_vec(),
            ),
            (
                "consumers",
                self.consumers == DiagnosticsConsumer::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5InstallDiagnosticsViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(
        &self,
        row: &ArtifactDiagnosticRow,
        governance: Option<
            &crate::m5_install_and_portability_governance::M5InstallPortabilityGovernanceMatrix,
        >,
        violations: &mut Vec<M5InstallDiagnosticsViolation>,
    ) {
        for (field, value) in [
            ("artifact_id", &row.artifact_id),
            ("display_name", &row.display_name),
            ("owner", &row.owner),
            ("verification_evidence_ref", &row.verification_evidence_ref),
            ("rollback_target_ref", &row.rollback_target_ref),
            ("diagnostics_ref", &row.diagnostics_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5InstallDiagnosticsViolation::EmptyField {
                    id: row.artifact_id.clone(),
                    field_name: field,
                });
            }
        }

        // Each artifact is pinned to the canonical governance lane it draws verification truth
        // from.
        if row.governs_lane != row.artifact_family.governs_lane() {
            violations.push(M5InstallDiagnosticsViolation::GovernsLaneMismatch {
                artifact_id: row.artifact_id.clone(),
                expected: row.artifact_family.governs_lane().as_str(),
            });
        }

        // The snapshotted governance assurance must equal the lane's published assurance in the
        // embedded governance matrix, so the diagnostics object never claims support beyond the
        // gate the governance matrix already applied.
        if let Some(matrix) = governance {
            match matrix.lane_row(row.governs_lane) {
                Some(lane_row) if lane_row.published_assurance != row.governs_assurance => {
                    violations.push(M5InstallDiagnosticsViolation::GovernsAssuranceMismatch {
                        artifact_id: row.artifact_id.clone(),
                        recorded: row.governs_assurance.as_str(),
                        governed: lane_row.published_assurance.as_str(),
                    });
                }
                Some(_) => {}
                None => violations.push(M5InstallDiagnosticsViolation::GovernsLaneNotInMatrix {
                    artifact_id: row.artifact_id.clone(),
                    lane: row.governs_lane.as_str(),
                }),
            }
        }

        // Roots must be at least one artifact root and one mutable-state root, recorded under the
        // correct category, with each store labelled and machine-protected or secret-bearing roots
        // redacted, so the packet classifies stores without dumping secrets.
        if row.artifact_roots.is_empty() {
            violations.push(M5InstallDiagnosticsViolation::EmptyField {
                id: row.artifact_id.clone(),
                field_name: "artifact_roots",
            });
        }
        if row.mutable_state_roots.is_empty() {
            violations.push(M5InstallDiagnosticsViolation::EmptyField {
                id: row.artifact_id.clone(),
                field_name: "mutable_state_roots",
            });
        }
        self.validate_roots(row, &row.artifact_roots, RootCategory::Artifact, violations);
        self.validate_roots(
            row,
            &row.mutable_state_roots,
            RootCategory::MutableState,
            violations,
        );
        self.validate_roots(row, &row.policy_roots, RootCategory::Policy, violations);

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.narrow_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5InstallDiagnosticsViolation::DuplicateNarrowReason {
                    artifact_id: row.artifact_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // The published support must equal the gate's recomputed ceiling.
        let effective = row.effective_support();
        if row.published_support != effective {
            violations.push(M5InstallDiagnosticsViolation::OverstatedSupport {
                artifact_id: row.artifact_id.clone(),
                published: row.published_support.as_str(),
                computed: effective.as_str(),
            });
        }

        // The recorded narrow reasons must equal the recomputed reasons.
        if row.narrow_reasons != row.computed_narrow_reasons() {
            violations.push(M5InstallDiagnosticsViolation::NarrowReasonsMismatch {
                artifact_id: row.artifact_id.clone(),
            });
        }

        // The recorded recovery path must equal the recomputed path.
        if row.recovery_path != row.computed_recovery_path() {
            violations.push(M5InstallDiagnosticsViolation::RecoveryPathMismatch {
                artifact_id: row.artifact_id.clone(),
            });
        }

        // A narrowed artifact must offer a real recovery path, list at least one caveat, and name
        // what is stale or narrowing.
        if row.is_downgraded() {
            if !row.recovery_path.is_offered() {
                violations.push(M5InstallDiagnosticsViolation::MissingRecoveryPath {
                    artifact_id: row.artifact_id.clone(),
                });
            }
            if row.caveats.is_empty() {
                violations.push(M5InstallDiagnosticsViolation::EmptyField {
                    id: row.artifact_id.clone(),
                    field_name: "caveats",
                });
            }
            if row.stale_or_missing_fields.is_empty() {
                violations.push(M5InstallDiagnosticsViolation::EmptyField {
                    id: row.artifact_id.clone(),
                    field_name: "stale_or_missing_fields",
                });
            }
        }

        // An artifact that still backs a publishable label must name at least one supported scope.
        if row.published_support != InstallAssurance::Withheld && row.supported_scopes.is_empty() {
            violations.push(M5InstallDiagnosticsViolation::EmptyField {
                id: row.artifact_id.clone(),
                field_name: "supported_scopes",
            });
        }

        // A verified artifact must be genuinely whole-trust: a signed binary, current
        // verification, an available rollback target, a verified governance lane, a declared
        // verified floor, no narrow reason, no caveat, no stale-or-missing field, and a no-op
        // recovery path.
        if row.is_verified()
            && (row.verification_state.assurance_ceiling() != InstallAssurance::Verified
                || row.verification_freshness != VerificationFreshness::Current
                || row.rollback_target != RollbackTargetState::Available
                || row.governs_assurance != InstallAssurance::Verified
                || row.capability_floor() != InstallAssurance::Verified
                || !row.narrow_reasons.is_empty()
                || !row.caveats.is_empty()
                || !row.stale_or_missing_fields.is_empty()
                || row.recovery_path.is_offered())
        {
            violations.push(M5InstallDiagnosticsViolation::VerifiedArtifactNotWhole {
                artifact_id: row.artifact_id.clone(),
            });
        }
    }

    fn validate_roots(
        &self,
        row: &ArtifactDiagnosticRow,
        roots: &[DiagnosticRoot],
        category: RootCategory,
        violations: &mut Vec<M5InstallDiagnosticsViolation>,
    ) {
        for root in roots {
            if root.location_label.trim().is_empty() {
                violations.push(M5InstallDiagnosticsViolation::EmptyField {
                    id: row.artifact_id.clone(),
                    field_name: "location_label",
                });
            }
            if root.role.category() != category {
                violations.push(M5InstallDiagnosticsViolation::RootCategoryMismatch {
                    artifact_id: row.artifact_id.clone(),
                    role: root.role.as_str(),
                });
            }
            if !root.redaction_ok() {
                violations.push(M5InstallDiagnosticsViolation::UnredactedSensitiveRoot {
                    artifact_id: row.artifact_id.clone(),
                    role: root.role.as_str(),
                    sensitivity: root.sensitivity.as_str(),
                });
            }
        }
    }

    fn validate_drills(
        &self,
        artifact_ids: &BTreeSet<String>,
        violations: &mut Vec<M5InstallDiagnosticsViolation>,
    ) {
        let mut seen_ids = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for drill in &self.drills {
            if !seen_ids.insert(drill.drill_id.clone()) {
                violations.push(M5InstallDiagnosticsViolation::DuplicateDrillId {
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
                    violations.push(M5InstallDiagnosticsViolation::EmptyField {
                        id: drill.drill_id.clone(),
                        field_name: field,
                    });
                }
            }
            if !artifact_ids.contains(&drill.artifact_id) {
                violations.push(M5InstallDiagnosticsViolation::DrillArtifactUnknown {
                    drill_id: drill.drill_id.clone(),
                    artifact_id: drill.artifact_id.clone(),
                });
            }
            // A drill exists to prove the diagnostics object detects its incident.
            if !drill.detected {
                violations.push(M5InstallDiagnosticsViolation::DrillNotDetected {
                    drill_id: drill.drill_id.clone(),
                });
            }
        }
        for incident in DiagnosticIncident::REQUIRED {
            if !covered.contains(&incident) {
                violations.push(M5InstallDiagnosticsViolation::MissingIncidentDrill {
                    incident: incident.as_str(),
                });
            }
        }
    }
}

/// A validation violation for the M5 install-and-update diagnostics packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5InstallDiagnosticsViolation {
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
        /// Row, drill, or packet id.
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
    /// A diagnostics-row id appears more than once.
    DuplicateArtifactId {
        /// Duplicate artifact id.
        artifact_id: String,
    },
    /// A claimed artifact family carries more than one row.
    DuplicateArtifactRow {
        /// Family token.
        family: &'static str,
    },
    /// A claimed artifact family has no row.
    MissingArtifactRow {
        /// Family token.
        family: &'static str,
    },
    /// A row covers a family the packet does not claim.
    UnclaimedArtifactRow {
        /// Row id.
        artifact_id: String,
        /// Family token.
        family: &'static str,
    },
    /// A row's governance lane is not the canonical lane for its family.
    GovernsLaneMismatch {
        /// Row id.
        artifact_id: String,
        /// Expected lane token.
        expected: &'static str,
    },
    /// A row's snapshotted governance assurance disagrees with the governance matrix.
    GovernsAssuranceMismatch {
        /// Row id.
        artifact_id: String,
        /// Recorded assurance token.
        recorded: &'static str,
        /// Governed assurance token.
        governed: &'static str,
    },
    /// A row's governance lane is missing from the governance matrix.
    GovernsLaneNotInMatrix {
        /// Row id.
        artifact_id: String,
        /// Lane token.
        lane: &'static str,
    },
    /// A root is recorded under the wrong category.
    RootCategoryMismatch {
        /// Row id.
        artifact_id: String,
        /// Root-role token.
        role: &'static str,
    },
    /// A machine-protected or secret-bearing root is not redacted.
    UnredactedSensitiveRoot {
        /// Row id.
        artifact_id: String,
        /// Root-role token.
        role: &'static str,
        /// Sensitivity token.
        sensitivity: &'static str,
    },
    /// A row lists a narrow reason more than once.
    DuplicateNarrowReason {
        /// Row id.
        artifact_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// An artifact publishes support beyond what its evidence supports.
    OverstatedSupport {
        /// Row id.
        artifact_id: String,
        /// Published support token.
        published: &'static str,
        /// Computed effective support token.
        computed: &'static str,
    },
    /// An artifact's narrow reasons disagree with the recomputed reasons.
    NarrowReasonsMismatch {
        /// Row id.
        artifact_id: String,
    },
    /// An artifact's recovery path disagrees with the recomputed path.
    RecoveryPathMismatch {
        /// Row id.
        artifact_id: String,
    },
    /// A narrowed artifact offers no recovery path.
    MissingRecoveryPath {
        /// Row id.
        artifact_id: String,
    },
    /// A verified artifact still narrows a state or carries a narrow reason.
    VerifiedArtifactNotWhole {
        /// Row id.
        artifact_id: String,
    },
    /// A drill id appears more than once.
    DuplicateDrillId {
        /// Duplicate drill id.
        drill_id: String,
    },
    /// A drill references an artifact id with no row.
    DrillArtifactUnknown {
        /// Drill id.
        drill_id: String,
        /// Referenced artifact id.
        artifact_id: String,
    },
    /// A drill does not detect its incident.
    DrillNotDetected {
        /// Drill id.
        drill_id: String,
    },
    /// A required support incident has no drill.
    MissingIncidentDrill {
        /// Incident token.
        incident: &'static str,
    },
    /// A required consumer surface has no binding.
    MissingConsumerBinding {
        /// Consumer token.
        consumer: &'static str,
    },
    /// A consumer binding drops or remints diagnostics truth.
    ConsumerBindingDrift {
        /// Binding ref.
        binding_ref: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5InstallDiagnosticsViolation {
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
            Self::GovernancePacketMismatch { actual, expected } => {
                write!(
                    f,
                    "packet binds governance {actual} but the embedded matrix is {expected}"
                )
            }
            Self::GovernanceMatrixUnavailable => {
                write!(f, "embedded governance matrix could not be loaded")
            }
            Self::DuplicateArtifactId { artifact_id } => {
                write!(f, "duplicate artifact id {artifact_id}")
            }
            Self::DuplicateArtifactRow { family } => {
                write!(f, "duplicate row for artifact family {family}")
            }
            Self::MissingArtifactRow { family } => {
                write!(f, "missing row for claimed artifact family {family}")
            }
            Self::UnclaimedArtifactRow {
                artifact_id,
                family,
            } => {
                write!(f, "row {artifact_id} covers unclaimed family {family}")
            }
            Self::GovernsLaneMismatch {
                artifact_id,
                expected,
            } => {
                write!(
                    f,
                    "row {artifact_id} governs_lane must be the canonical lane {expected}"
                )
            }
            Self::GovernsAssuranceMismatch {
                artifact_id,
                recorded,
                governed,
            } => {
                write!(
                    f,
                    "row {artifact_id} records governs_assurance {recorded} but the matrix publishes {governed}"
                )
            }
            Self::GovernsLaneNotInMatrix { artifact_id, lane } => {
                write!(
                    f,
                    "row {artifact_id} governs lane {lane} is not in the matrix"
                )
            }
            Self::RootCategoryMismatch { artifact_id, role } => {
                write!(
                    f,
                    "row {artifact_id} records root role {role} under the wrong category"
                )
            }
            Self::UnredactedSensitiveRoot {
                artifact_id,
                role,
                sensitivity,
            } => {
                write!(
                    f,
                    "row {artifact_id} root {role} is {sensitivity} but not redacted"
                )
            }
            Self::DuplicateNarrowReason {
                artifact_id,
                reason,
            } => {
                write!(f, "row {artifact_id} repeats narrow reason {reason}")
            }
            Self::OverstatedSupport {
                artifact_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "row {artifact_id} publishes support {published} but the gate computes {computed}"
                )
            }
            Self::NarrowReasonsMismatch { artifact_id } => {
                write!(f, "row {artifact_id} narrow reasons disagree with the gate")
            }
            Self::RecoveryPathMismatch { artifact_id } => {
                write!(f, "row {artifact_id} recovery path disagrees with the gate")
            }
            Self::MissingRecoveryPath { artifact_id } => {
                write!(
                    f,
                    "row {artifact_id} is narrowed but offers no recovery path"
                )
            }
            Self::VerifiedArtifactNotWhole { artifact_id } => {
                write!(
                    f,
                    "row {artifact_id} is verified but narrows a state or carries a narrow reason"
                )
            }
            Self::DuplicateDrillId { drill_id } => {
                write!(f, "duplicate drill id {drill_id}")
            }
            Self::DrillArtifactUnknown {
                drill_id,
                artifact_id,
            } => {
                write!(
                    f,
                    "drill {drill_id} references unknown artifact {artifact_id}"
                )
            }
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
                write!(
                    f,
                    "binding {binding_ref} does not preserve diagnostics truth"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for M5InstallDiagnosticsViolation {}

/// Stable record-kind tag for [`M5InstallDiagnosticsSupportExport`].
pub const M5_INSTALL_DIAGNOSTICS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_install_update_diagnostics_support_export";

/// Support-export wrapper preserving the diagnostics packet verbatim for support and evidence
/// packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallDiagnosticsSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub diagnostics_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact diagnostics packet preserved by the export.
    pub diagnostics: M5InstallUpdateDiagnostics,
}

impl M5InstallDiagnosticsSupportExport {
    /// Whether the export preserves the same packet id and a clean diagnostics packet.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_INSTALL_DIAGNOSTICS_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_INSTALL_DIAGNOSTICS_SCHEMA_VERSION
            && self.diagnostics_packet_id_ref == self.diagnostics.packet_id
            && self.raw_private_material_excluded
            && self.diagnostics.validate().is_empty()
    }
}

/// Loads the embedded M5 install-and-update diagnostics packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5InstallUpdateDiagnostics`].
pub fn current_m5_install_update_diagnostics(
) -> Result<M5InstallUpdateDiagnostics, serde_json::Error> {
    serde_json::from_str(M5_INSTALL_DIAGNOSTICS_JSON)
}

#[cfg(test)]
mod tests;
