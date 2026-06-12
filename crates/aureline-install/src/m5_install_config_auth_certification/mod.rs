//! Canonical M5 install/config/auth certification packet: one inspectable object that aggregates the
//! install-topology, configuration-portability, sync/device, and auth-recovery evidence landed across
//! the install, settings, and identity lanes into one qualification report and one automatic narrowing
//! path for every claimed M5 desktop, managed, and mirror/offline profile.
//!
//! Where the coexistence-and-fleet-rollout packet proves install families do not collide and the
//! settings effective/portable, sync/device, and auth/recovery packets each prove one configuration or
//! identity domain, this packet sits above them and answers the graduation question: for each named
//! profile — the stable desktop install, the preview install, the portable package, the managed-fleet
//! install, and the mirror/air-gap install — is every certification domain ([`CertificationDomain`])
//! current and provable, and what support may the profile publish? One [`CertificationRow`] carries one
//! profile's four domain qualifications ([`DomainQualification`]), and the same packet is the object
//! release center, About, docs/help, admin docs, support export, CLI, and diagnostics render instead of
//! cloning blanket "enterprise-ready install", "seamless sync", or "account-safe managed mode" copy.
//!
//! The certification gate keeps a profile from inheriting broader support than its evidence backs. The
//! support a row may publish is the weakest ceiling implied by its domain qualifications: a domain whose
//! source packet declared below verified, whose evidence is aging or stale, whose evidence is missing, or
//! a profile missing a required domain row all lower or withhold the published support automatically. Each
//! domain row names the canonical source packet it draws truth from via [`SourcePacket::contract_ref`],
//! validated against the closed source-packet vocabulary, so a profile can never publish support beyond
//! the qualification of the packets it aggregates. This realizes the certification invariant: a claimed
//! M5 install/config/auth row is either qualified with current proof or automatically downgraded before
//! publication.
//!
//! Evidence freshness is first-class. Each domain qualification records how fresh its evidence is
//! ([`EvidenceFreshness`]) and publishes only the support that freshness backs, so a stale install,
//! portability, sync, or auth row cannot stay green by inertia.
//!
//! Drills are first-class too. Each [`CertificationDrill`] replays one install-topology, side-by-side,
//! portable, mirror/offline, settings-portability, sync/device, passkey/recovery, accessibility, or
//! downgrade scenario and proves the object detects it, so a certification failure is visible before
//! publication rather than after a profile graduates on stale evidence.
//!
//! Because every required consumer surface — release center, Help/About, docs/help, admin docs, support
//! export, diagnostics, and CLI — binds to this one packet via a [`CertificationConsumerBinding`] that
//! must ingest it, preserve its published support and downgrade paths, and narrow with it, a profile
//! narrowed here cannot read as supported on a release evidence row, an About panel, a docs/help page, an
//! admin doc badge, a CLI status line, a diagnostics surface, or a support export.
//!
//! The packet is checked in at `artifacts/install/m5/m5-install-config-auth-certification.json` and
//! embedded here. It is metadata-only: every field is a typed state or an opaque ref, and it carries no
//! credential bodies, raw provider payloads, or workspace contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_install_and_portability_governance::InstallAssurance;

/// Supported M5 install/config/auth certification schema version.
pub const M5_INSTALL_CONFIG_AUTH_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_INSTALL_CONFIG_AUTH_CERTIFICATION_RECORD_KIND: &str =
    "m5_install_config_auth_certification";

/// Repo-relative path to the checked-in packet.
pub const M5_INSTALL_CONFIG_AUTH_CERTIFICATION_PATH: &str =
    "artifacts/install/m5/m5-install-config-auth-certification.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_INSTALL_CONFIG_AUTH_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/install/m5-install-config-auth-certification.schema.json";

/// Repo-relative path to the companion document.
pub const M5_INSTALL_CONFIG_AUTH_CERTIFICATION_DOC_REF: &str =
    "docs/install/m5/m5-install-config-auth-certification.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_INSTALL_CONFIG_AUTH_CERTIFICATION_ARTIFACT_DOC_REF: &str =
    "artifacts/install/m5/m5-install-config-auth-certification.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_INSTALL_CONFIG_AUTH_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/install/m5/m5-install-config-auth-certification";

/// Embedded checked-in packet JSON.
pub const M5_INSTALL_CONFIG_AUTH_CERTIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/install/m5/m5-install-config-auth-certification.json"
));

/// A claimed M5 profile the certification packet qualifies.
///
/// These are the named desktop, managed, and mirror/offline profiles a graduation row covers, so install
/// topology, configuration portability, sync/device lineage, and auth recovery stay provable for each one
/// rather than assuming one connected happy path or one opaque managed default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationProfile {
    /// The stable-channel desktop install — the first-party local baseline.
    DesktopStable,
    /// A preview desktop install running side-by-side with stable.
    DesktopPreview,
    /// A portable install carrying its own durable state root.
    Portable,
    /// An organization-managed, policy-controlled fleet install.
    ManagedFleet,
    /// A mirror or air-gap install provisioned from offline media.
    MirrorOffline,
}

impl CertificationProfile {
    /// Every claimed profile, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DesktopStable,
        Self::DesktopPreview,
        Self::Portable,
        Self::ManagedFleet,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopStable => "desktop_stable",
            Self::DesktopPreview => "desktop_preview",
            Self::Portable => "portable",
            Self::ManagedFleet => "managed_fleet",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// A certification domain qualified for every claimed profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDomain {
    /// Install topology, side-by-side, portable, and mirror/offline install truth.
    InstallTopology,
    /// Effective-settings parity and portable/export configuration truth.
    ConfigPortability,
    /// Sync scope, device participation, and device-registry lineage.
    SyncDevice,
    /// Account sign-in, step-up, passkey, browser-handoff, and recovery depth.
    AuthRecovery,
}

impl CertificationDomain {
    /// Every certification domain a row must qualify, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::InstallTopology,
        Self::ConfigPortability,
        Self::SyncDevice,
        Self::AuthRecovery,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallTopology => "install_topology",
            Self::ConfigPortability => "config_portability",
            Self::SyncDevice => "sync_device",
            Self::AuthRecovery => "auth_recovery",
        }
    }
}

/// A B19 source packet this certification packet aggregates.
///
/// Each domain qualification draws its truth from exactly one source packet; the certification packet is
/// canonical only because it references the already-landed packets by their stable contract ref rather
/// than cloning their status text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourcePacket {
    /// The install-and-portability governance matrix.
    InstallGovernance,
    /// The coexistence-and-fleet-rollout packet.
    CoexistenceFleetRollout,
    /// The install-and-update diagnostics packet.
    InstallDiagnostics,
    /// The effective-settings parity certification.
    EffectiveSettings,
    /// The portable-state-and-restore packet.
    PortableStateAndRestore,
    /// The sync-and-device-review packet.
    SyncAndDeviceReview,
    /// The auth-and-recovery packet.
    AuthAndRecovery,
}

impl SourcePacket {
    /// Every aggregated source packet, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::InstallGovernance,
        Self::CoexistenceFleetRollout,
        Self::InstallDiagnostics,
        Self::EffectiveSettings,
        Self::PortableStateAndRestore,
        Self::SyncAndDeviceReview,
        Self::AuthAndRecovery,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallGovernance => "install_governance",
            Self::CoexistenceFleetRollout => "coexistence_fleet_rollout",
            Self::InstallDiagnostics => "install_diagnostics",
            Self::EffectiveSettings => "effective_settings",
            Self::PortableStateAndRestore => "portable_state_and_restore",
            Self::SyncAndDeviceReview => "sync_and_device_review",
            Self::AuthAndRecovery => "auth_and_recovery",
        }
    }

    /// The canonical contract ref the source packet publishes its truth under.
    ///
    /// A domain qualification's recorded `source_packet_ref` must equal this, so the certification packet
    /// can never bind a domain to a packet that does not exist.
    pub const fn contract_ref(self) -> &'static str {
        match self {
            Self::InstallGovernance => "m5-install-and-portability-governance:m5:v1",
            Self::CoexistenceFleetRollout => "m5-coexistence-and-fleet-rollout:m5:v1",
            Self::InstallDiagnostics => "m5-install-diagnostics:m5:v1",
            Self::EffectiveSettings => "settings:m5_effective_settings:v1",
            Self::PortableStateAndRestore => "settings:m5_portable_state_and_restore:v1",
            Self::SyncAndDeviceReview => "settings:m5_sync_and_device_review:v1",
            Self::AuthAndRecovery => "auth:m5_auth_and_recovery:v1",
        }
    }

    /// The certification domain this source packet certifies.
    pub const fn domain(self) -> CertificationDomain {
        match self {
            Self::InstallGovernance | Self::CoexistenceFleetRollout | Self::InstallDiagnostics => {
                CertificationDomain::InstallTopology
            }
            Self::EffectiveSettings | Self::PortableStateAndRestore => {
                CertificationDomain::ConfigPortability
            }
            Self::SyncAndDeviceReview => CertificationDomain::SyncDevice,
            Self::AuthAndRecovery => CertificationDomain::AuthRecovery,
        }
    }
}

/// How fresh a domain qualification's evidence is.
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

    /// Highest assurance this freshness permits a domain to publish.
    pub const fn assurance_ceiling(self) -> InstallAssurance {
        match self {
            Self::Current => InstallAssurance::Verified,
            Self::Aging => InstallAssurance::Bounded,
            Self::Stale => InstallAssurance::RetestPending,
            Self::Missing => InstallAssurance::Withheld,
        }
    }

    /// Whether this freshness raises the [`CertificationNarrowReason::StaleEvidence`] trigger.
    pub const fn is_stale_trigger(self) -> bool {
        matches!(self, Self::Aging | Self::Stale)
    }

    /// Whether this freshness raises the [`CertificationNarrowReason::MissingEvidence`] trigger.
    pub const fn is_missing_trigger(self) -> bool {
        matches!(self, Self::Missing)
    }
}

/// A headline reason the certification gate narrows a profile's published support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationNarrowReason {
    /// A domain's evidence is aging or stale.
    StaleEvidence,
    /// A domain's evidence is missing.
    MissingEvidence,
    /// A domain's source packet declared below verified.
    SourceUnqualified,
    /// The profile is missing a required domain qualification.
    IncompleteDomainCoverage,
}

impl CertificationNarrowReason {
    /// Every narrow reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StaleEvidence,
        Self::MissingEvidence,
        Self::SourceUnqualified,
        Self::IncompleteDomainCoverage,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaleEvidence => "stale_evidence",
            Self::MissingEvidence => "missing_evidence",
            Self::SourceUnqualified => "source_unqualified",
            Self::IncompleteDomainCoverage => "incomplete_domain_coverage",
        }
    }
}

/// The downgrade path surfaced when a profile's support is narrowed or withheld.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDowngradePath {
    /// Refresh the domain evidence before widening the claim.
    RefreshEvidence,
    /// Re-qualify the underlying source packet before widening the claim.
    RequalifySource,
    /// Add the missing required domain qualification.
    CompleteDomainCoverage,
    /// Narrow the published claim to the supported scope.
    NarrowToSupportedScope,
    /// Withhold the profile's support claim from publication.
    WithholdClaim,
    /// No downgrade is needed; only valid for a verified row.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl CertificationDowngradePath {
    /// Every downgrade path, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RefreshEvidence,
        Self::RequalifySource,
        Self::CompleteDomainCoverage,
        Self::NarrowToSupportedScope,
        Self::WithholdClaim,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshEvidence => "refresh_evidence",
            Self::RequalifySource => "requalify_source",
            Self::CompleteDomainCoverage => "complete_domain_coverage",
            Self::NarrowToSupportedScope => "narrow_to_supported_scope",
            Self::WithholdClaim => "withhold_claim",
            Self::NoneNeeded => "none",
        }
    }

    /// Whether this is a real downgrade path the operator can take.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::NoneNeeded)
    }
}

/// A certification scenario a drill replays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDrillClass {
    /// An install-topology, state-root, or channel drill.
    InstallTopology,
    /// A side-by-side coexistence drill.
    SideBySide,
    /// A portable-install drill.
    Portable,
    /// A mirror or air-gap import drill.
    MirrorOffline,
    /// A settings-portability or export/import drill.
    SettingsPortability,
    /// A sync-scope or device-registry drill.
    SyncDevice,
    /// A passkey or account-recovery drill.
    PasskeyRecovery,
    /// An accessibility validation drill.
    Accessibility,
    /// A downgrade-on-stale-evidence drill.
    Downgrade,
}

impl CertificationDrillClass {
    /// Every drill class a packet must cover, in declaration order.
    pub const REQUIRED: [Self; 9] = [
        Self::InstallTopology,
        Self::SideBySide,
        Self::Portable,
        Self::MirrorOffline,
        Self::SettingsPortability,
        Self::SyncDevice,
        Self::PasskeyRecovery,
        Self::Accessibility,
        Self::Downgrade,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallTopology => "install_topology",
            Self::SideBySide => "side_by_side",
            Self::Portable => "portable",
            Self::MirrorOffline => "mirror_offline",
            Self::SettingsPortability => "settings_portability",
            Self::SyncDevice => "sync_device",
            Self::PasskeyRecovery => "passkey_recovery",
            Self::Accessibility => "accessibility",
            Self::Downgrade => "downgrade",
        }
    }
}

/// A consumer surface that must ingest this packet and narrow with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationConsumer {
    /// The release-center qualification-evidence surface.
    ReleaseCenter,
    /// The Help/About panel.
    About,
    /// The docs/help certification surface.
    DocsHelp,
    /// The administrator documentation surface.
    AdminDocs,
    /// The support-export bundle.
    SupportExport,
    /// The install diagnostics surface.
    Diagnostics,
    /// The command-line certification-and-status surface.
    Cli,
}

impl CertificationConsumer {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 7] = [
        Self::ReleaseCenter,
        Self::About,
        Self::DocsHelp,
        Self::AdminDocs,
        Self::SupportExport,
        Self::Diagnostics,
        Self::Cli,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::About => "about",
            Self::DocsHelp => "docs_help",
            Self::AdminDocs => "admin_docs",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::Cli => "cli",
        }
    }
}

/// One domain qualification carried by a certification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainQualification {
    /// Certification domain this qualification covers.
    pub domain: CertificationDomain,
    /// Source packet this domain draws its truth from.
    pub source_packet: SourcePacket,
    /// Contract ref the source packet publishes under; must equal [`SourcePacket::contract_ref`].
    pub source_packet_ref: String,
    /// Opaque ref to the domain's qualification evidence.
    pub evidence_ref: String,
    /// How fresh the domain's evidence is.
    pub evidence_freshness: EvidenceFreshness,
    /// Support the source packet's own evidence asserts, before the gate.
    pub declared_support: InstallAssurance,
    /// Support actually published after the gate narrows the domain.
    ///
    /// Must equal [`DomainQualification::effective_support`].
    pub published_support: InstallAssurance,
    /// Caveats attached to the published support.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale, missing, or narrowing the support.
    #[serde(default)]
    pub stale_or_missing_fields: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl DomainQualification {
    /// The support label the source evidence asserted, before narrowing.
    pub fn capability_floor(&self) -> InstallAssurance {
        self.declared_support
    }

    /// The support label the gate permits this domain to publish.
    ///
    /// Lowers the capability floor to the ceiling implied by the evidence freshness, so a stale or
    /// missing domain can never publish a verified label.
    pub fn effective_support(&self) -> InstallAssurance {
        self.capability_floor()
            .min(self.evidence_freshness.assurance_ceiling())
    }

    /// Whether the domain publishes a clean verified label.
    pub fn is_verified(&self) -> bool {
        self.effective_support() == InstallAssurance::Verified
    }

    /// Whether the gate narrowed the published support below what the source declared.
    pub fn is_downgraded(&self) -> bool {
        self.effective_support().rank() < self.capability_floor().rank()
    }

    /// Whether the recorded source ref matches the source packet and the stored support agrees with the
    /// recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.source_packet_ref == self.source_packet.contract_ref()
            && self.source_packet.domain() == self.domain
            && self.published_support == self.effective_support()
    }
}

/// One certification row for a claimed M5 profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationRow {
    /// Stable row id.
    pub row_id: String,
    /// Profile this row qualifies.
    pub profile: CertificationProfile,
    /// Human-readable profile name.
    pub display_name: String,
    /// Owner accountable for the profile's certification evidence.
    pub owner: String,
    /// UTC date this profile's evidence is current as of.
    pub evidence_as_of: String,
    /// Domain qualifications, one per required domain.
    pub domains: Vec<DomainQualification>,
    /// Support the profile's own claim asserts, before the gate.
    pub declared_support: InstallAssurance,
    /// Support actually published after the gate narrows the profile.
    ///
    /// Must equal [`CertificationRow::effective_support`].
    pub published_support: InstallAssurance,
    /// Headline narrow reasons; must equal the recomputed set.
    #[serde(default)]
    pub narrow_reasons: Vec<CertificationNarrowReason>,
    /// Downgrade path surfaced when support is narrowed; must equal the recomputed path.
    pub downgrade_path: CertificationDowngradePath,
    /// Scope or slice labels this profile still backs.
    #[serde(default)]
    pub supported_scopes: Vec<String>,
    /// Caveats attached to the published support.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale, missing, or narrowing the support.
    #[serde(default)]
    pub stale_or_missing_fields: Vec<String>,
    /// Ref binding this row into the certification index.
    pub certification_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl CertificationRow {
    /// The support label the profile's own claim asserted, before narrowing.
    pub fn capability_floor(&self) -> InstallAssurance {
        self.declared_support
    }

    /// The weakest domain ceiling across the row's domain qualifications.
    pub fn worst_domain_support(&self) -> InstallAssurance {
        self.domains
            .iter()
            .map(DomainQualification::effective_support)
            .fold(InstallAssurance::Verified, InstallAssurance::min)
    }

    /// Whether the row qualifies every required certification domain exactly once.
    pub fn covers_all_domains(&self) -> bool {
        let seen: BTreeSet<CertificationDomain> = self.domains.iter().map(|d| d.domain).collect();
        seen.len() == self.domains.len()
            && CertificationDomain::REQUIRED
                .iter()
                .all(|d| seen.contains(d))
    }

    /// The support label the gate permits this profile to publish.
    ///
    /// Lowers the capability floor to the weakest domain ceiling, and withholds entirely when a required
    /// domain is missing, so a profile can never inherit broader support than every domain it aggregates
    /// can back.
    pub fn effective_support(&self) -> InstallAssurance {
        if !self.covers_all_domains() {
            return InstallAssurance::Withheld;
        }
        self.capability_floor().min(self.worst_domain_support())
    }

    /// The headline narrow reasons recomputed from the row's domain qualifications.
    pub fn computed_narrow_reasons(&self) -> Vec<CertificationNarrowReason> {
        let mut reasons = Vec::new();
        if self
            .domains
            .iter()
            .any(|d| d.evidence_freshness.is_stale_trigger())
        {
            reasons.push(CertificationNarrowReason::StaleEvidence);
        }
        if self
            .domains
            .iter()
            .any(|d| d.evidence_freshness.is_missing_trigger())
        {
            reasons.push(CertificationNarrowReason::MissingEvidence);
        }
        if self
            .domains
            .iter()
            .any(|d| d.declared_support != InstallAssurance::Verified)
        {
            reasons.push(CertificationNarrowReason::SourceUnqualified);
        }
        if !self.covers_all_domains() {
            reasons.push(CertificationNarrowReason::IncompleteDomainCoverage);
        }
        reasons
    }

    /// The downgrade path the gate surfaces for this profile.
    pub fn computed_downgrade_path(&self) -> CertificationDowngradePath {
        if !self.covers_all_domains() {
            CertificationDowngradePath::CompleteDomainCoverage
        } else if self.effective_support() == InstallAssurance::Withheld {
            CertificationDowngradePath::WithholdClaim
        } else if self.domains.iter().any(|d| {
            d.evidence_freshness.is_stale_trigger() || d.evidence_freshness.is_missing_trigger()
        }) {
            CertificationDowngradePath::RefreshEvidence
        } else if self
            .domains
            .iter()
            .any(|d| d.declared_support != InstallAssurance::Verified)
        {
            CertificationDowngradePath::RequalifySource
        } else if self.is_downgraded() {
            CertificationDowngradePath::NarrowToSupportedScope
        } else {
            CertificationDowngradePath::NoneNeeded
        }
    }

    /// Whether the profile publishes a clean verified label.
    pub fn is_verified(&self) -> bool {
        self.effective_support() == InstallAssurance::Verified
    }

    /// Whether the gate narrowed the published support below what the profile declared.
    pub fn is_downgraded(&self) -> bool {
        self.effective_support().rank() < self.capability_floor().rank()
    }

    /// Whether the stored published support, narrow reasons, and downgrade path all agree with the
    /// recomputed gate decision and every domain is gate-consistent.
    pub fn gate_consistent(&self) -> bool {
        self.published_support == self.effective_support()
            && self.narrow_reasons == self.computed_narrow_reasons()
            && self.downgrade_path == self.computed_downgrade_path()
            && self
                .domains
                .iter()
                .all(DomainQualification::gate_consistent)
    }
}

/// A drill that replays one certification scenario and proves the object detects it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Scenario this drill replays.
    pub drill_class: CertificationDrillClass,
    /// Row id the drill targets; must reference a claimed row.
    pub target_ref: String,
    /// Reviewer-facing scenario summary.
    pub scenario: String,
    /// Signal a healthy, fully-qualified profile would emit.
    pub expected_signal: String,
    /// Signal the scenario emits.
    pub observed_signal: String,
    /// Whether the object detects this scenario; must be true.
    pub detected: bool,
    /// Resolution the drill proves.
    pub resolves_to: String,
}

/// One binding wiring a consumer surface to this packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationConsumerBinding {
    /// Consumer surface this binding wires.
    pub consumer: CertificationConsumer,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Packet id this surface ingests.
    pub certification_packet_id_ref: String,
    /// True when the surface ingests this packet rather than a parallel summary.
    pub ingests_packet: bool,
    /// True when the surface preserves the published support verbatim.
    pub preserves_published_support: bool,
    /// True when the surface preserves the downgrade paths verbatim.
    pub preserves_downgrade_paths: bool,
    /// True when the surface narrows automatically as rows are downgraded.
    pub narrows_on_downgrade: bool,
    /// True when raw private material is excluded from the binding.
    pub raw_private_material_excluded: bool,
}

impl CertificationConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.certification_packet_id_ref == packet_id
            && self.ingests_packet
            && self.preserves_published_support
            && self.preserves_downgrade_paths
            && self.narrows_on_downgrade
            && self.raw_private_material_excluded
            && !self.binding_ref.trim().is_empty()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5InstallConfigAuthCertificationSummary {
    /// Total certification rows.
    pub total_rows: usize,
    /// Number of claimed profiles.
    pub profile_count: usize,
    /// Total domain qualifications across all rows.
    pub total_domain_rows: usize,
    /// Number of distinct source packets aggregated.
    pub source_packet_count: usize,
    /// Rows published as verified.
    pub verified_rows: usize,
    /// Rows narrowed to a bounded label.
    pub bounded_rows: usize,
    /// Rows narrowed to a retest-pending label.
    pub retest_pending_rows: usize,
    /// Rows withheld from publication.
    pub withheld_rows: usize,
    /// Rows whose published support was downgraded below what they declared.
    pub downgraded_rows: usize,
    /// Rows narrowed because a domain's evidence is aging or stale.
    pub stale_evidence_rows: usize,
    /// Rows narrowed because a domain's evidence is missing.
    pub missing_evidence_rows: usize,
    /// Rows narrowed because a domain's source declared below verified.
    pub source_unqualified_rows: usize,
    /// Total drills.
    pub drill_count: usize,
    /// Drills that detect their scenario.
    pub detected_drill_count: usize,
}

/// A redaction-safe export row projected from a certification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallConfigAuthCertificationExportRow {
    /// Row id.
    pub row_id: String,
    /// Profile token.
    pub profile: String,
    /// Human-readable profile name.
    pub display_name: String,
    /// Declared-support token.
    pub declared_support: String,
    /// Published-support token.
    pub published_support: String,
    /// Narrow-reason tokens.
    pub narrow_reasons: Vec<String>,
    /// Downgrade-path token.
    pub downgrade_path: String,
    /// Per-domain published-support tokens, keyed by domain token.
    pub domain_support: Vec<(String, String)>,
    /// Supported scope or slice labels.
    pub supported_scopes: Vec<String>,
    /// Caveats attached to the published support.
    pub caveats: Vec<String>,
    /// Whether the row publishes a verified label.
    pub verified: bool,
    /// Whether the published support was downgraded below the declared support.
    pub downgraded: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet downstream surfaces render instead of restating each
/// profile's certification posture by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallConfigAuthCertificationExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected certification rows.
    pub rows: Vec<M5InstallConfigAuthCertificationExportRow>,
    /// Whether every row's published support, reasons, and downgrade path agree with the gate.
    pub all_rows_gate_consistent: bool,
    /// Rows that publish a verified label.
    pub verified_count: usize,
    /// Rows the gate narrowed below their declared support.
    pub downgraded_count: usize,
    /// Rows the gate withheld entirely.
    pub withheld_count: usize,
}

/// The typed M5 install/config/auth certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5InstallConfigAuthCertification {
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
    /// Scheme the packet mints stable certification identities under.
    pub certification_identity_scheme: String,
    /// Claimed profiles; one row per profile.
    pub profiles: Vec<CertificationProfile>,
    /// Closed certification-domain vocabulary.
    pub domains: Vec<CertificationDomain>,
    /// Closed source-packet vocabulary.
    pub source_packets: Vec<SourcePacket>,
    /// Closed assurance-label vocabulary.
    pub assurance_labels: Vec<InstallAssurance>,
    /// Closed evidence-freshness vocabulary.
    pub evidence_freshness_states: Vec<EvidenceFreshness>,
    /// Closed narrow-reason vocabulary.
    pub narrow_reasons: Vec<CertificationNarrowReason>,
    /// Closed downgrade-path vocabulary.
    pub downgrade_paths: Vec<CertificationDowngradePath>,
    /// Closed drill-class vocabulary.
    pub drill_classes: Vec<CertificationDrillClass>,
    /// Closed consumer vocabulary.
    pub consumers: Vec<CertificationConsumer>,
    /// Certification rows, one per claimed profile.
    #[serde(default)]
    pub rows: Vec<CertificationRow>,
    /// Drills, one per required drill class.
    #[serde(default)]
    pub drills: Vec<CertificationDrill>,
    /// Consumer bindings, one per required surface.
    #[serde(default)]
    pub consumer_bindings: Vec<CertificationConsumerBinding>,
    /// Summary counts.
    pub summary: M5InstallConfigAuthCertificationSummary,
}

impl M5InstallConfigAuthCertification {
    /// Returns the row for a claimed profile.
    pub fn row(&self, profile: CertificationProfile) -> Option<&CertificationRow> {
        self.rows.iter().find(|r| r.profile == profile)
    }

    /// Returns the row with the given stable row id.
    pub fn row_by_id(&self, row_id: &str) -> Option<&CertificationRow> {
        self.rows.iter().find(|r| r.row_id == row_id)
    }

    /// Rows that publish a verified label.
    pub fn verified_rows(&self) -> impl Iterator<Item = &CertificationRow> {
        self.rows.iter().filter(|r| r.is_verified())
    }

    /// Rows the gate narrowed below their declared support.
    pub fn downgraded_rows(&self) -> impl Iterator<Item = &CertificationRow> {
        self.rows.iter().filter(|r| r.is_downgraded())
    }

    /// Whether a consumer binding preserves this packet for the given surface.
    pub fn has_binding_for(&self, consumer: CertificationConsumer) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer == consumer && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every row's stored support, reasons, and downgrade path agree with the gate.
    pub fn all_rows_gate_consistent(&self) -> bool {
        self.rows.iter().all(CertificationRow::gate_consistent)
    }

    /// The distinct source packets referenced across all domain qualifications.
    fn referenced_source_packets(&self) -> BTreeSet<SourcePacket> {
        self.rows
            .iter()
            .flat_map(|r| r.domains.iter().map(|d| d.source_packet))
            .collect()
    }

    /// Recomputes the summary block from the rows and drills.
    pub fn computed_summary(&self) -> M5InstallConfigAuthCertificationSummary {
        let count_row = |label: InstallAssurance| {
            self.rows
                .iter()
                .filter(|r| r.published_support == label)
                .count()
        };
        let has_reason = |reason: CertificationNarrowReason| {
            self.rows
                .iter()
                .filter(|r| r.narrow_reasons.contains(&reason))
                .count()
        };
        M5InstallConfigAuthCertificationSummary {
            total_rows: self.rows.len(),
            profile_count: self.profiles.len(),
            total_domain_rows: self.rows.iter().map(|r| r.domains.len()).sum(),
            source_packet_count: self.referenced_source_packets().len(),
            verified_rows: count_row(InstallAssurance::Verified),
            bounded_rows: count_row(InstallAssurance::Bounded),
            retest_pending_rows: count_row(InstallAssurance::RetestPending),
            withheld_rows: count_row(InstallAssurance::Withheld),
            downgraded_rows: self.rows.iter().filter(|r| r.is_downgraded()).count(),
            stale_evidence_rows: has_reason(CertificationNarrowReason::StaleEvidence),
            missing_evidence_rows: has_reason(CertificationNarrowReason::MissingEvidence),
            source_unqualified_rows: has_reason(CertificationNarrowReason::SourceUnqualified),
            drill_count: self.drills.len(),
            detected_drill_count: self.drills.iter().filter(|d| d.detected).count(),
        }
    }

    /// Produces the certification index downstream surfaces render instead of restating each profile's
    /// certification posture by hand.
    pub fn export_projection(&self) -> M5InstallConfigAuthCertificationExportProjection {
        let rows = self
            .rows
            .iter()
            .map(|r| M5InstallConfigAuthCertificationExportRow {
                row_id: r.row_id.clone(),
                profile: r.profile.as_str().to_owned(),
                display_name: r.display_name.clone(),
                declared_support: r.declared_support.as_str().to_owned(),
                published_support: r.published_support.as_str().to_owned(),
                narrow_reasons: r
                    .narrow_reasons
                    .iter()
                    .map(|x| x.as_str().to_owned())
                    .collect(),
                downgrade_path: r.downgrade_path.as_str().to_owned(),
                domain_support: r
                    .domains
                    .iter()
                    .map(|d| {
                        (
                            d.domain.as_str().to_owned(),
                            d.published_support.as_str().to_owned(),
                        )
                    })
                    .collect(),
                supported_scopes: r.supported_scopes.clone(),
                caveats: r.caveats.clone(),
                verified: r.is_verified(),
                downgraded: r.is_downgraded(),
                summary: format!(
                    "{}: declared {}, published {}, downgrade {} ({} domains)",
                    r.profile.as_str(),
                    r.declared_support.as_str(),
                    r.published_support.as_str(),
                    r.downgrade_path.as_str(),
                    r.domains.len()
                ),
            })
            .collect();
        M5InstallConfigAuthCertificationExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_rows_gate_consistent: self.all_rows_gate_consistent(),
            verified_count: self.verified_rows().count(),
            downgraded_count: self.downgraded_rows().count(),
            withheld_count: self
                .rows
                .iter()
                .filter(|r| r.published_support == InstallAssurance::Withheld)
                .count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5InstallConfigAuthCertificationSupportExport {
        M5InstallConfigAuthCertificationSupportExport {
            record_kind: M5_INSTALL_CONFIG_AUTH_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_INSTALL_CONFIG_AUTH_CERTIFICATION_SCHEMA_VERSION,
            export_id: export_id.into(),
            certification_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            certification: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5InstallConfigAuthCertificationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<CertificationProfile> = self.profiles.iter().copied().collect();
        let mut seen_ids = BTreeSet::new();
        let mut seen_profiles = BTreeSet::new();
        for row in &self.rows {
            if !seen_ids.insert(row.row_id.clone()) {
                violations.push(M5InstallConfigAuthCertificationViolation::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            if !seen_profiles.insert(row.profile) {
                violations.push(M5InstallConfigAuthCertificationViolation::DuplicateRow {
                    profile: row.profile.as_str(),
                });
            }
            if !claimed.contains(&row.profile) {
                violations.push(M5InstallConfigAuthCertificationViolation::UnclaimedRow {
                    row_id: row.row_id.clone(),
                    profile: row.profile.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        for &profile in &self.profiles {
            if !seen_profiles.contains(&profile) {
                violations.push(M5InstallConfigAuthCertificationViolation::MissingRow {
                    profile: profile.as_str(),
                });
            }
        }

        // Every declared source packet must be aggregated by at least one domain row, so the
        // certification packet provably covers every B19 packet it claims to.
        let referenced = self.referenced_source_packets();
        for &source in &self.source_packets {
            if !referenced.contains(&source) {
                violations.push(
                    M5InstallConfigAuthCertificationViolation::UnreferencedSourcePacket {
                        source: source.as_str(),
                    },
                );
            }
        }

        let row_ids: BTreeSet<String> = self.rows.iter().map(|r| r.row_id.clone()).collect();
        self.validate_drills(&row_ids, &mut violations);

        for consumer in CertificationConsumer::REQUIRED {
            if !self.has_binding_for(consumer) {
                violations.push(
                    M5InstallConfigAuthCertificationViolation::MissingConsumerBinding {
                        consumer: consumer.as_str(),
                    },
                );
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(
                    M5InstallConfigAuthCertificationViolation::ConsumerBindingDrift {
                        binding_ref: binding.binding_ref.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5InstallConfigAuthCertificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5InstallConfigAuthCertificationViolation>) {
        if self.schema_version != M5_INSTALL_CONFIG_AUTH_CERTIFICATION_SCHEMA_VERSION {
            violations.push(
                M5InstallConfigAuthCertificationViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != M5_INSTALL_CONFIG_AUTH_CERTIFICATION_RECORD_KIND {
            violations.push(
                M5InstallConfigAuthCertificationViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            (
                "certification_identity_scheme",
                &self.certification_identity_scheme,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(M5InstallConfigAuthCertificationViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "profiles",
                self.profiles == CertificationProfile::ALL.to_vec(),
            ),
            (
                "domains",
                self.domains == CertificationDomain::REQUIRED.to_vec(),
            ),
            (
                "source_packets",
                self.source_packets == SourcePacket::ALL.to_vec(),
            ),
            (
                "assurance_labels",
                self.assurance_labels == InstallAssurance::ALL.to_vec(),
            ),
            (
                "evidence_freshness_states",
                self.evidence_freshness_states == EvidenceFreshness::ALL.to_vec(),
            ),
            (
                "narrow_reasons",
                self.narrow_reasons == CertificationNarrowReason::ALL.to_vec(),
            ),
            (
                "downgrade_paths",
                self.downgrade_paths == CertificationDowngradePath::ALL.to_vec(),
            ),
            (
                "drill_classes",
                self.drill_classes == CertificationDrillClass::REQUIRED.to_vec(),
            ),
            (
                "consumers",
                self.consumers == CertificationConsumer::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(
                    M5InstallConfigAuthCertificationViolation::ClosedVocabularyMismatch { field },
                );
            }
        }
    }

    fn validate_row(
        &self,
        row: &CertificationRow,
        violations: &mut Vec<M5InstallConfigAuthCertificationViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("display_name", &row.display_name),
            ("owner", &row.owner),
            ("evidence_as_of", &row.evidence_as_of),
            ("certification_ref", &row.certification_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5InstallConfigAuthCertificationViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }

        // Every required domain must be qualified exactly once, drawing from a source packet that
        // certifies that domain.
        let mut seen_domains = BTreeSet::new();
        for domain in &row.domains {
            for (field, value) in [
                ("source_packet_ref", &domain.source_packet_ref),
                ("evidence_ref", &domain.evidence_ref),
                ("note", &domain.note),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5InstallConfigAuthCertificationViolation::EmptyField {
                        id: row.row_id.clone(),
                        field_name: field,
                    });
                }
            }
            if domain.source_packet_ref != domain.source_packet.contract_ref() {
                violations.push(
                    M5InstallConfigAuthCertificationViolation::SourceRefMismatch {
                        row_id: row.row_id.clone(),
                        domain: domain.domain.as_str(),
                        recorded: domain.source_packet_ref.clone(),
                        expected: domain.source_packet.contract_ref(),
                    },
                );
            }
            if domain.source_packet.domain() != domain.domain {
                violations.push(
                    M5InstallConfigAuthCertificationViolation::SourceDomainMismatch {
                        row_id: row.row_id.clone(),
                        domain: domain.domain.as_str(),
                        source: domain.source_packet.as_str(),
                    },
                );
            }
            let domain_effective = domain.effective_support();
            if domain.published_support != domain_effective {
                violations.push(
                    M5InstallConfigAuthCertificationViolation::DomainOverstatedSupport {
                        row_id: row.row_id.clone(),
                        domain: domain.domain.as_str(),
                        published: domain.published_support.as_str(),
                        computed: domain_effective.as_str(),
                    },
                );
            }
            if domain.is_downgraded() && domain.stale_or_missing_fields.is_empty() {
                violations.push(M5InstallConfigAuthCertificationViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: "domain.stale_or_missing_fields",
                });
            }
            if !seen_domains.insert(domain.domain) {
                violations.push(M5InstallConfigAuthCertificationViolation::DuplicateDomain {
                    row_id: row.row_id.clone(),
                    domain: domain.domain.as_str(),
                });
            }
        }
        for domain in CertificationDomain::REQUIRED {
            if !seen_domains.contains(&domain) {
                violations.push(M5InstallConfigAuthCertificationViolation::MissingDomain {
                    row_id: row.row_id.clone(),
                    domain: domain.as_str(),
                });
            }
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.narrow_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(
                    M5InstallConfigAuthCertificationViolation::DuplicateNarrowReason {
                        row_id: row.row_id.clone(),
                        reason: reason.as_str(),
                    },
                );
            }
        }

        // The published support must equal the gate's recomputed ceiling.
        let effective = row.effective_support();
        if row.published_support != effective {
            violations.push(
                M5InstallConfigAuthCertificationViolation::OverstatedSupport {
                    row_id: row.row_id.clone(),
                    published: row.published_support.as_str(),
                    computed: effective.as_str(),
                },
            );
        }
        if row.narrow_reasons != row.computed_narrow_reasons() {
            violations.push(
                M5InstallConfigAuthCertificationViolation::NarrowReasonsMismatch {
                    row_id: row.row_id.clone(),
                },
            );
        }
        if row.downgrade_path != row.computed_downgrade_path() {
            violations.push(
                M5InstallConfigAuthCertificationViolation::DowngradePathMismatch {
                    row_id: row.row_id.clone(),
                },
            );
        }

        // A narrowed row must offer a real downgrade path, list at least one caveat, and name what is
        // stale or missing.
        if row.is_downgraded() {
            if !row.downgrade_path.is_offered() {
                violations.push(
                    M5InstallConfigAuthCertificationViolation::MissingDowngradePath {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if row.caveats.is_empty() {
                violations.push(M5InstallConfigAuthCertificationViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: "caveats",
                });
            }
            if row.stale_or_missing_fields.is_empty() {
                violations.push(M5InstallConfigAuthCertificationViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: "stale_or_missing_fields",
                });
            }
        }

        // A row that still backs a publishable label must name at least one supported scope.
        if row.published_support != InstallAssurance::Withheld && row.supported_scopes.is_empty() {
            violations.push(M5InstallConfigAuthCertificationViolation::EmptyField {
                id: row.row_id.clone(),
                field_name: "supported_scopes",
            });
        }

        // A verified row must be genuinely whole-trust: a verified declared floor, full domain coverage,
        // every domain verified with current evidence, no narrow reason, no caveat, no stale field, and a
        // no-op downgrade path. This is the guardrail that forbids stale or source-narrowed evidence from
        // reading as a blanket verified profile.
        if row.is_verified()
            && (row.capability_floor() != InstallAssurance::Verified
                || !row.covers_all_domains()
                || !row.domains.iter().all(DomainQualification::is_verified)
                || !row.narrow_reasons.is_empty()
                || !row.caveats.is_empty()
                || !row.stale_or_missing_fields.is_empty()
                || row.downgrade_path.is_offered())
        {
            violations.push(
                M5InstallConfigAuthCertificationViolation::VerifiedRowNotWhole {
                    row_id: row.row_id.clone(),
                },
            );
        }
    }

    fn validate_drills(
        &self,
        row_ids: &BTreeSet<String>,
        violations: &mut Vec<M5InstallConfigAuthCertificationViolation>,
    ) {
        let mut seen_ids = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for drill in &self.drills {
            if !seen_ids.insert(drill.drill_id.clone()) {
                violations.push(
                    M5InstallConfigAuthCertificationViolation::DuplicateDrillId {
                        drill_id: drill.drill_id.clone(),
                    },
                );
            }
            covered.insert(drill.drill_class);
            for (field, value) in [
                ("drill_id", &drill.drill_id),
                ("scenario", &drill.scenario),
                ("expected_signal", &drill.expected_signal),
                ("observed_signal", &drill.observed_signal),
                ("resolves_to", &drill.resolves_to),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5InstallConfigAuthCertificationViolation::EmptyField {
                        id: drill.drill_id.clone(),
                        field_name: field,
                    });
                }
            }
            if !row_ids.contains(&drill.target_ref) {
                violations.push(
                    M5InstallConfigAuthCertificationViolation::DrillTargetUnknown {
                        drill_id: drill.drill_id.clone(),
                        target_ref: drill.target_ref.clone(),
                    },
                );
            }
            if !drill.detected {
                violations.push(
                    M5InstallConfigAuthCertificationViolation::DrillNotDetected {
                        drill_id: drill.drill_id.clone(),
                    },
                );
            }
        }
        for class in CertificationDrillClass::REQUIRED {
            if !covered.contains(&class) {
                violations.push(
                    M5InstallConfigAuthCertificationViolation::MissingDrillClass {
                        class: class.as_str(),
                    },
                );
            }
        }
    }
}

/// A validation violation for the M5 install/config/auth certification packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5InstallConfigAuthCertificationViolation {
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
        /// Row, domain, drill, or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// A claimed profile carries more than one row.
    DuplicateRow {
        /// Profile token.
        profile: &'static str,
    },
    /// A claimed profile has no row.
    MissingRow {
        /// Profile token.
        profile: &'static str,
    },
    /// A row covers a profile the packet does not claim.
    UnclaimedRow {
        /// Row id.
        row_id: String,
        /// Profile token.
        profile: &'static str,
    },
    /// A required certification domain is recorded more than once on a row.
    DuplicateDomain {
        /// Row id.
        row_id: String,
        /// Domain token.
        domain: &'static str,
    },
    /// A required certification domain is missing from a row.
    MissingDomain {
        /// Row id.
        row_id: String,
        /// Domain token.
        domain: &'static str,
    },
    /// A domain qualification records a source ref that is not the source packet's contract ref.
    SourceRefMismatch {
        /// Row id.
        row_id: String,
        /// Domain token.
        domain: &'static str,
        /// Recorded source ref.
        recorded: String,
        /// Expected contract ref.
        expected: &'static str,
    },
    /// A domain qualification draws from a source packet that does not certify that domain.
    SourceDomainMismatch {
        /// Row id.
        row_id: String,
        /// Domain token.
        domain: &'static str,
        /// Source-packet token.
        source: &'static str,
    },
    /// A declared source packet is aggregated by no domain row.
    UnreferencedSourcePacket {
        /// Source-packet token.
        source: &'static str,
    },
    /// A domain publishes support beyond what its evidence supports.
    DomainOverstatedSupport {
        /// Row id.
        row_id: String,
        /// Domain token.
        domain: &'static str,
        /// Published support token.
        published: &'static str,
        /// Computed effective support token.
        computed: &'static str,
    },
    /// A row lists a narrow reason more than once.
    DuplicateNarrowReason {
        /// Row id.
        row_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A row publishes support beyond what its evidence supports.
    OverstatedSupport {
        /// Row id.
        row_id: String,
        /// Published support token.
        published: &'static str,
        /// Computed effective support token.
        computed: &'static str,
    },
    /// A row's narrow reasons disagree with the recomputed reasons.
    NarrowReasonsMismatch {
        /// Row id.
        row_id: String,
    },
    /// A row's downgrade path disagrees with the recomputed path.
    DowngradePathMismatch {
        /// Row id.
        row_id: String,
    },
    /// A narrowed row offers no downgrade path.
    MissingDowngradePath {
        /// Row id.
        row_id: String,
    },
    /// A verified row still narrows a state or carries a narrow reason.
    VerifiedRowNotWhole {
        /// Row id.
        row_id: String,
    },
    /// A drill id appears more than once.
    DuplicateDrillId {
        /// Duplicate drill id.
        drill_id: String,
    },
    /// A drill references a row id with no row.
    DrillTargetUnknown {
        /// Drill id.
        drill_id: String,
        /// Referenced target id.
        target_ref: String,
    },
    /// A drill does not detect its scenario.
    DrillNotDetected {
        /// Drill id.
        drill_id: String,
    },
    /// A required drill class has no drill.
    MissingDrillClass {
        /// Drill-class token.
        class: &'static str,
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

impl fmt::Display for M5InstallConfigAuthCertificationViolation {
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
            Self::DuplicateRowId { row_id } => write!(f, "duplicate row id {row_id}"),
            Self::DuplicateRow { profile } => {
                write!(f, "duplicate row for profile {profile}")
            }
            Self::MissingRow { profile } => {
                write!(f, "missing row for claimed profile {profile}")
            }
            Self::UnclaimedRow { row_id, profile } => {
                write!(f, "row {row_id} covers unclaimed profile {profile}")
            }
            Self::DuplicateDomain { row_id, domain } => {
                write!(f, "row {row_id} repeats domain {domain}")
            }
            Self::MissingDomain { row_id, domain } => {
                write!(f, "row {row_id} is missing domain {domain}")
            }
            Self::SourceRefMismatch {
                row_id,
                domain,
                recorded,
                expected,
            } => write!(
                f,
                "row {row_id} domain {domain} records source ref {recorded} but the canonical ref is {expected}"
            ),
            Self::SourceDomainMismatch {
                row_id,
                domain,
                source,
            } => write!(
                f,
                "row {row_id} domain {domain} draws from source {source} which does not certify it"
            ),
            Self::UnreferencedSourcePacket { source } => {
                write!(f, "declared source packet {source} is aggregated by no domain row")
            }
            Self::DomainOverstatedSupport {
                row_id,
                domain,
                published,
                computed,
            } => write!(
                f,
                "row {row_id} domain {domain} publishes support {published} but the gate computes {computed}"
            ),
            Self::DuplicateNarrowReason { row_id, reason } => {
                write!(f, "row {row_id} repeats narrow reason {reason}")
            }
            Self::OverstatedSupport {
                row_id,
                published,
                computed,
            } => write!(
                f,
                "row {row_id} publishes support {published} but the gate computes {computed}"
            ),
            Self::NarrowReasonsMismatch { row_id } => {
                write!(f, "row {row_id} narrow reasons disagree with the gate")
            }
            Self::DowngradePathMismatch { row_id } => {
                write!(f, "row {row_id} downgrade path disagrees with the gate")
            }
            Self::MissingDowngradePath { row_id } => {
                write!(f, "row {row_id} is narrowed but offers no downgrade path")
            }
            Self::VerifiedRowNotWhole { row_id } => write!(
                f,
                "row {row_id} is verified but narrows a domain or carries a narrow reason"
            ),
            Self::DuplicateDrillId { drill_id } => write!(f, "duplicate drill id {drill_id}"),
            Self::DrillTargetUnknown {
                drill_id,
                target_ref,
            } => write!(f, "drill {drill_id} references unknown target {target_ref}"),
            Self::DrillNotDetected { drill_id } => {
                write!(f, "drill {drill_id} does not detect its scenario")
            }
            Self::MissingDrillClass { class } => {
                write!(f, "missing drill for class {class}")
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

impl Error for M5InstallConfigAuthCertificationViolation {}

/// Stable record-kind tag for [`M5InstallConfigAuthCertificationSupportExport`].
pub const M5_INSTALL_CONFIG_AUTH_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_install_config_auth_certification_support_export";

/// Support-export wrapper preserving the packet verbatim for support and evidence packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallConfigAuthCertificationSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub certification_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact packet preserved by the export.
    pub certification: M5InstallConfigAuthCertification,
}

impl M5InstallConfigAuthCertificationSupportExport {
    /// Whether the export preserves the same packet id and a clean packet.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_INSTALL_CONFIG_AUTH_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_INSTALL_CONFIG_AUTH_CERTIFICATION_SCHEMA_VERSION
            && self.certification_packet_id_ref == self.certification.packet_id
            && self.raw_private_material_excluded
            && self.certification.validate().is_empty()
    }
}

/// Loads the embedded M5 install/config/auth certification packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5InstallConfigAuthCertification`].
pub fn current_m5_install_config_auth_certification(
) -> Result<M5InstallConfigAuthCertification, serde_json::Error> {
    serde_json::from_str(M5_INSTALL_CONFIG_AUTH_CERTIFICATION_JSON)
}

#[cfg(test)]
mod tests;
