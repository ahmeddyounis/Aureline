//! Typed service-health and stale-or-mirrored release-data communication — the per-boundary truth a
//! user or admin reads to tell *which* boundary is actually in trouble, *how trustworthy* the
//! release/service data on screen is, *what admin notes* apply, and *what still works locally*
//! regardless.
//!
//! Where the [update-center summary](crate::m5_update_summary) answers "what is changing" and the
//! [support-window cards](crate::m5_support_window_card) answer "what support promises a channel
//! carries", this lane answers the exit-gate question the source set treats as governed truth:
//! *release and outage communication becomes unsafe when cached, mirrored, or policy-limited data
//! looks live, or when one degraded service makes the whole product look broken*. The packet keeps the
//! four boundaries a user must distinguish separate — the local machine, a remote target, the
//! enterprise control plane, and optional vendor-hosted services — so a vendor outage never reads as a
//! local failure and a stale mirror never reads as live.
//!
//! The packet carries two card families, both gate-bound to the shared
//! [descriptor/badge](crate::m5_descriptor_badge) vocabulary so the service-health panel, Help/About,
//! docs/help, support export, the admin console, and the release center read one set of states rather
//! than re-deriving them:
//!
//! - one [service-tier health card](ServiceTierHealthCard) per [tier](ServiceTier), carrying the
//!   tier's [health state](HealthState), the [release-data state](ReleaseDataState) of the data shown
//!   for it (live, mirrored, cached, stale, policy-limited, local-only, or unavailable), the
//!   [source-age truth](SourceAge) behind that data, the [local-safe continuation](ContinuationStatement)
//!   statement of what still works locally, and the recovery path; and
//! - one [admin-note card](AdminNoteCard) per propagated [note kind](AdminNoteKind) — channel, mirror,
//!   or deployment change — carrying the same release-data vocabulary and an export-safe evidence path,
//!   so an admin note reads identically on every surface.
//!
//! A card's gate is the *worse* of its health and release-data postures, so a card can never make
//! cached, mirrored, stale, or policy-limited data look live — the lane's guardrail against
//! over-stating freshness, enforced by [`ServiceHealthCommunication::validate`]. A card whose data is
//! downgraded or whose tier is in trouble must carry a continuation statement and a recovery path
//! rather than a bare red banner; a card missing that fails validation. The local machine is the only
//! boundary whose trouble can mark local editing unsafe — a remote, control-plane, or vendor card is
//! always local-safe — so [`HealthContinuity::local_editing_safe`] stays explicit and no surface
//! collapses to "everything broken".
//!
//! The [consumer surfaces](HealthConsumer) each read the cards and *derive* their
//! [readiness](HealthReadiness) and [gaps](HealthGap) from them, and every consumer carries the
//! packet's local-continuation truth, so the admin notes and service-health messages stay consistent
//! across the UI, docs/help, and support exports.
//!
//! The packet is inspectable and serde-serializable; it carries metadata, refs, source-age labels, and
//! message ids only — no credential bodies or raw provider payloads — so the service-health truth is
//! exportable and reviewable outside the app and stays honest under stale, mirrored, policy-limited, or
//! no-live-data conditions.
//!
//! - Packet schema:
//!   [`schemas/release/m5-service-health-communication.schema.json`](../../../../../schemas/release/m5-service-health-communication.schema.json)
//! - Contract doc:
//!   [`docs/release/m5-service-health-communication-contract.md`](../../../../../docs/release/m5-service-health-communication-contract.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_service_health_communication, seeded_m5_service_health_communication_local_only,
    seeded_m5_service_health_communication_mirror_note,
    seeded_m5_service_health_communication_vendor_outage,
    M5_SERVICE_HEALTH_COMMUNICATION_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The service-health cards reuse the update / lifecycle governance vocabularies for channel, artifact
// class, deployment profile, and packet-level stale-data behaviour, and the descriptor / badge
// runtime's gate / status / signal vocabulary, so this layer can never drift to a different vocabulary
// than the layers above.
use crate::m5_descriptor_badge::{ConsumerStatus, DescriptorGate, DescriptorSignal};
use crate::m5_update_lifecycle::{ChannelScope, DeploymentProfile, StaleDataBehavior};

/// Record-kind tag carried by [`ServiceHealthCommunication`].
pub const M5_SERVICE_HEALTH_COMMUNICATION_RECORD_KIND: &str = "m5_service_health_communication";

/// Schema version for the service-health communication packet.
pub const M5_SERVICE_HEALTH_COMMUNICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the packet schema.
pub const M5_SERVICE_HEALTH_COMMUNICATION_SCHEMA_REF: &str =
    "schemas/release/m5-service-health-communication.schema.json";

/// Repo-relative path of the published packet inventory.
pub const M5_SERVICE_HEALTH_COMMUNICATION_REF: &str =
    "artifacts/release/m5-service-health-communication.json";

/// Repo-relative path of the release-grade stale-release-data parity proof.
pub const M5_SERVICE_HEALTH_COMMUNICATION_PROOF_REF: &str =
    "artifacts/release/m5-stale-release-data-proof/service-health-communication.json";

/// Repo-relative path of the machine-readable per-card export.
pub const M5_SERVICE_HEALTH_COMMUNICATION_CSV_REF: &str =
    "artifacts/release/m5-service-health-communication.csv";

/// Repo-relative path of the packet contract doc.
pub const M5_SERVICE_HEALTH_COMMUNICATION_DOC_REF: &str =
    "docs/release/m5-service-health-communication-contract.md";

/// Repo-relative directory of the per-state packet fixtures.
pub const M5_SERVICE_HEALTH_COMMUNICATION_FIXTURE_DIR: &str =
    "fixtures/release/service-health-and-admin-notes/";

/// Prefix every service-health message id carries so consumers can route it.
pub const M5_SERVICE_HEALTH_MESSAGE_ID_PREFIX: &str = "release_service_health.";

const REDACTION_CLASS: &str = "metadata_safe_default";

// ---------------------------------------------------------------------------
// Controlled vocabularies
// ---------------------------------------------------------------------------

/// One boundary a user must be able to tell apart from the others. Keeping the four tiers distinct is
/// what lets the panel say *which* boundary is in trouble instead of collapsing every outage into one
/// "service down" banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceTier {
    /// The local editor / workspace on this machine.
    LocalMachine,
    /// A remote development target the user connected to (SSH host, container, remote workspace).
    RemoteTarget,
    /// The organisation's managed control plane (policy, licensing, admin).
    EnterpriseControlPlane,
    /// An optional vendor-hosted service (update mirror, hosted AI provider, telemetry).
    VendorHostedService,
}

impl ServiceTier {
    /// Every tier, in declaration order (local→remote→control-plane→vendor).
    pub const ALL: [Self; 4] = [
        Self::LocalMachine,
        Self::RemoteTarget,
        Self::EnterpriseControlPlane,
        Self::VendorHostedService,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalMachine => "local_machine",
            Self::RemoteTarget => "remote_target",
            Self::EnterpriseControlPlane => "enterprise_control_plane",
            Self::VendorHostedService => "vendor_hosted_service",
        }
    }

    /// Human-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalMachine => "Local machine",
            Self::RemoteTarget => "Remote target",
            Self::EnterpriseControlPlane => "Enterprise control plane",
            Self::VendorHostedService => "Vendor-hosted service",
        }
    }

    /// One-line identity blurb so the card states what the boundary *is*.
    pub const fn description(self) -> &'static str {
        match self {
            Self::LocalMachine => "Your local editor and workspace; edits and recovery run here.",
            Self::RemoteTarget => "A remote development target you connected to.",
            Self::EnterpriseControlPlane => {
                "Your organisation's managed policy and licensing plane."
            }
            Self::VendorHostedService => {
                "Optional vendor-hosted services such as the update mirror."
            }
        }
    }

    /// Accountable owner role for this tier's health truth.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::LocalMachine => "local_workspace_owner",
            Self::RemoteTarget => "remote_target_owner",
            Self::EnterpriseControlPlane => "control_plane_owner",
            Self::VendorHostedService => "vendor_service_owner",
        }
    }

    /// True only for the one boundary whose trouble can make local editing unsafe — the local machine.
    /// A remote, control-plane, or vendor outage never impairs local editing.
    pub const fn affects_local_editing(self) -> bool {
        matches!(self, Self::LocalMachine)
    }

    /// True for the optional vendor-hosted boundary, whose absence is expected rather than a failure.
    pub const fn is_optional(self) -> bool {
        matches!(self, Self::VendorHostedService)
    }
}

/// The health posture of a service tier. The set stays scoped to whether the tier can serve live
/// release/update data — it is deliberately not a generic alert vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthState {
    /// Reachable and serving live release/update data.
    Operational,
    /// Reachable but slow or partial; data may lag.
    Degraded,
    /// In a planned maintenance window; live checks are paused.
    Maintenance,
    /// Health could not be determined.
    Unknown,
    /// Unreachable; no live release/update data from this tier.
    Outage,
}

impl HealthState {
    /// Every health state, in declaration order (best→worst).
    pub const ALL: [Self; 5] = [
        Self::Operational,
        Self::Degraded,
        Self::Maintenance,
        Self::Unknown,
        Self::Outage,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Operational => "operational",
            Self::Degraded => "degraded",
            Self::Maintenance => "maintenance",
            Self::Unknown => "unknown",
            Self::Outage => "outage",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Operational => "Operational",
            Self::Degraded => "Degraded",
            Self::Maintenance => "Maintenance",
            Self::Unknown => "Unknown",
            Self::Outage => "Outage",
        }
    }

    /// Gate posture this health state binds to.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::Operational => DescriptorGate::Governed,
            Self::Degraded | Self::Maintenance | Self::Unknown => DescriptorGate::Narrowed,
            Self::Outage => DescriptorGate::Blocked,
        }
    }
}

/// How trustworthy the release/service data shown for a card is. Every weaker state is a first-class
/// token so cached, mirrored, stale, policy-limited, local-only, and unavailable data can never
/// disappear into a "live" rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseDataState {
    /// Live, verified data from the tier.
    LiveVerified,
    /// A mirror copy, labelled as mirrored.
    Mirrored,
    /// An offline-cached copy, labelled as cached.
    OfflineCached,
    /// Behind a stale banner; older than its freshness window.
    Stale,
    /// Limited by an enterprise control-plane policy, labelled as policy-limited.
    PolicyLimited,
    /// No live data; only what is known locally, labelled local-safe.
    LocalOnlySafe,
    /// No data is reachable at all from the tier.
    Unavailable,
}

impl ReleaseDataState {
    /// Every release-data state, in declaration order (most→least trustworthy).
    pub const ALL: [Self; 7] = [
        Self::LiveVerified,
        Self::Mirrored,
        Self::OfflineCached,
        Self::Stale,
        Self::PolicyLimited,
        Self::LocalOnlySafe,
        Self::Unavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveVerified => "live_verified",
            Self::Mirrored => "mirrored",
            Self::OfflineCached => "offline_cached",
            Self::Stale => "stale",
            Self::PolicyLimited => "policy_limited",
            Self::LocalOnlySafe => "local_only_safe",
            Self::Unavailable => "unavailable",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LiveVerified => "Live, verified",
            Self::Mirrored => "Mirrored copy",
            Self::OfflineCached => "Offline cached",
            Self::Stale => "Stale",
            Self::PolicyLimited => "Policy-limited",
            Self::LocalOnlySafe => "Local-only (safe)",
            Self::Unavailable => "Unavailable",
        }
    }

    /// Gate posture this data state binds to. Only live data is governed; every downgraded state
    /// narrows; only unavailable blocks.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::LiveVerified => DescriptorGate::Governed,
            Self::Mirrored
            | Self::OfflineCached
            | Self::Stale
            | Self::PolicyLimited
            | Self::LocalOnlySafe => DescriptorGate::Narrowed,
            Self::Unavailable => DescriptorGate::Blocked,
        }
    }

    /// True for the one state that may render as live.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::LiveVerified)
    }

    /// True when the state is a visibly downgraded copy that must not look live.
    pub const fn is_downgraded(self) -> bool {
        !self.is_live()
    }
}

/// One admin-propagated change a card communicates. The set stays scoped to channel, mirror, and
/// deployment changes — the release/update/support lifecycle changes the source set governs — rather
/// than a generic operational feed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminNoteKind {
    /// The release channel an audience is pointed at changed.
    ChannelChange,
    /// The update mirror / source endpoint changed.
    MirrorChange,
    /// The deployment posture (managed, self-hosted, air-gapped) changed.
    DeploymentChange,
}

impl AdminNoteKind {
    /// Every note kind, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ChannelChange,
        Self::MirrorChange,
        Self::DeploymentChange,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChannelChange => "channel_change",
            Self::MirrorChange => "mirror_change",
            Self::DeploymentChange => "deployment_change",
        }
    }

    /// Human-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ChannelChange => "Channel change",
            Self::MirrorChange => "Mirror change",
            Self::DeploymentChange => "Deployment change",
        }
    }

    /// Accountable owner role.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::ChannelChange => "release_channel_owner",
            Self::MirrorChange => "mirror_owner",
            Self::DeploymentChange => "deployment_owner",
        }
    }
}

/// The recovery / continuation path a card discloses, so a card under trouble always shows a way
/// forward rather than only a red banner. `continue_locally` is the explicit local-safe path; the
/// remote / reconnect / admin / maintenance paths name a real action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryAction {
    /// No recovery path applies (the tier is healthy and live).
    NotApplicable,
    /// Continue working locally; the data shown is downgraded but local work is safe.
    ContinueLocally,
    /// Show the labelled mirror / cached copy while live data is unavailable.
    UseMirrorCopy,
    /// Retry the tier when it becomes reachable again.
    RetryWhenReachable,
    /// Reconnect the remote target to restore its features.
    ReconnectTarget,
    /// Contact an administrator about an enterprise control-plane change or limit.
    ContactAdmin,
    /// Wait for the planned maintenance window to end.
    WaitForMaintenance,
}

impl RecoveryAction {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::NotApplicable,
        Self::ContinueLocally,
        Self::UseMirrorCopy,
        Self::RetryWhenReachable,
        Self::ReconnectTarget,
        Self::ContactAdmin,
        Self::WaitForMaintenance,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::ContinueLocally => "continue_locally",
            Self::UseMirrorCopy => "use_mirror_copy",
            Self::RetryWhenReachable => "retry_when_reachable",
            Self::ReconnectTarget => "reconnect_target",
            Self::ContactAdmin => "contact_admin",
            Self::WaitForMaintenance => "wait_for_maintenance",
        }
    }

    /// True when the action names a real continuation / recovery path a card under trouble must offer
    /// (everything except the no-op "not applicable").
    pub const fn is_active_path(self) -> bool {
        !matches!(self, Self::NotApplicable)
    }
}

/// The readiness a card or consumer resolves to — a direct reading of a [`DescriptorGate`] in
/// service-health language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthReadiness {
    /// Live, trustworthy data; nothing downgraded.
    LiveTrusted,
    /// Showing visibly downgraded (mirrored / cached / stale / policy-limited / local-only) data.
    ShowingDowngraded,
    /// No live data from at least one boundary it reads.
    NoLiveData,
}

impl HealthReadiness {
    /// Every readiness, in declaration order.
    pub const ALL: [Self; 3] = [Self::LiveTrusted, Self::ShowingDowngraded, Self::NoLiveData];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTrusted => "live_trusted",
            Self::ShowingDowngraded => "showing_downgraded",
            Self::NoLiveData => "no_live_data",
        }
    }

    /// The readiness a gate resolves to.
    pub const fn from_gate(gate: DescriptorGate) -> Self {
        match gate {
            DescriptorGate::Governed => Self::LiveTrusted,
            DescriptorGate::Narrowed => Self::ShowingDowngraded,
            DescriptorGate::Blocked => Self::NoLiveData,
        }
    }
}

/// The named cause of a consumer's gap on one card it reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthGapKind {
    /// A read card's data is visibly downgraded (narrows).
    DataDowngraded,
    /// A read tier has no live data (outage / unavailable) (blocks).
    NoLiveDataFromTier,
    /// A tier the consumer reads is not carded in the packet (blocks).
    TierNotCarded,
    /// An admin note the consumer reads is not propagated to the packet (blocks).
    AdminNoteNotPropagated,
}

impl HealthGapKind {
    /// Every gap kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DataDowngraded,
        Self::NoLiveDataFromTier,
        Self::TierNotCarded,
        Self::AdminNoteNotPropagated,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DataDowngraded => "data_downgraded",
            Self::NoLiveDataFromTier => "no_live_data_from_tier",
            Self::TierNotCarded => "tier_not_carded",
            Self::AdminNoteNotPropagated => "admin_note_not_propagated",
        }
    }

    /// The gate this gap forces.
    const fn gate(self) -> DescriptorGate {
        match self {
            Self::DataDowngraded => DescriptorGate::Narrowed,
            Self::NoLiveDataFromTier | Self::TierNotCarded | Self::AdminNoteNotPropagated => {
                DescriptorGate::Blocked
            }
        }
    }
}

/// Whether a gap points at a tier card or an admin-note card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthTargetKind {
    /// The gap points at a service-tier health card.
    Tier,
    /// The gap points at an admin-note card.
    AdminNote,
}

impl HealthTargetKind {
    /// Every target kind, in declaration order.
    pub const ALL: [Self; 2] = [Self::Tier, Self::AdminNote];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tier => "tier",
            Self::AdminNote => "admin_note",
        }
    }
}

/// One claimed consumer surface that reads the service-health cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthConsumer {
    /// The in-product service-health / update-center panel.
    ServiceHealthPanel,
    /// The Help / About surface.
    HelpAbout,
    /// The docs / help content.
    DocsHelp,
    /// The support export.
    SupportExport,
    /// The admin console.
    AdminConsole,
    /// The release center / public-truth automation.
    ReleaseCenter,
}

impl HealthConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ServiceHealthPanel,
        Self::HelpAbout,
        Self::DocsHelp,
        Self::SupportExport,
        Self::AdminConsole,
        Self::ReleaseCenter,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServiceHealthPanel => "service_health_panel",
            Self::HelpAbout => "help_about",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::AdminConsole => "admin_console",
            Self::ReleaseCenter => "release_center",
        }
    }

    /// Human-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ServiceHealthPanel => "Service-health panel",
            Self::HelpAbout => "Help & About",
            Self::DocsHelp => "Docs / Help",
            Self::SupportExport => "Support export",
            Self::AdminConsole => "Admin console",
            Self::ReleaseCenter => "Release center",
        }
    }

    /// Accountable owner role.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::ServiceHealthPanel => "service_health_panel_owner",
            Self::HelpAbout => "help_about_owner",
            Self::DocsHelp => "docs_help_owner",
            Self::SupportExport => "support_export_owner",
            Self::AdminConsole => "admin_console_owner",
            Self::ReleaseCenter => "release_center_owner",
        }
    }
}

// ---------------------------------------------------------------------------
// Ranking helpers for deterministic ordering
// ---------------------------------------------------------------------------

fn tier_rank(t: ServiceTier) -> usize {
    ServiceTier::ALL
        .iter()
        .position(|x| *x == t)
        .unwrap_or(usize::MAX)
}

fn note_rank(k: AdminNoteKind) -> usize {
    AdminNoteKind::ALL
        .iter()
        .position(|x| *x == k)
        .unwrap_or(usize::MAX)
}

fn consumer_rank(c: HealthConsumer) -> usize {
    HealthConsumer::ALL
        .iter()
        .position(|x| *x == c)
        .unwrap_or(usize::MAX)
}

fn profile_rank(p: DeploymentProfile) -> usize {
    DeploymentProfile::ALL
        .iter()
        .position(|x| *x == p)
        .unwrap_or(usize::MAX)
}

fn gate_rank(g: DescriptorGate) -> u8 {
    match g {
        DescriptorGate::Governed => 0,
        DescriptorGate::Narrowed => 1,
        DescriptorGate::Blocked => 2,
    }
}

fn worst_gate(a: DescriptorGate, b: DescriptorGate) -> DescriptorGate {
    if gate_rank(a) >= gate_rank(b) {
        a
    } else {
        b
    }
}

fn status_for_gate(gate: DescriptorGate) -> ConsumerStatus {
    match gate {
        DescriptorGate::Governed => ConsumerStatus::Mapped,
        DescriptorGate::Narrowed => ConsumerStatus::Provisional,
        DescriptorGate::Blocked => ConsumerStatus::Unmapped,
    }
}

fn signal_for_gate(gate: DescriptorGate) -> DescriptorSignal {
    match gate {
        DescriptorGate::Governed => DescriptorSignal::Green,
        DescriptorGate::Narrowed => DescriptorSignal::Yellow,
        DescriptorGate::Blocked => DescriptorSignal::Red,
    }
}

fn sort_profiles(profiles: &mut Vec<DeploymentProfile>) {
    profiles.sort_by_key(|p| profile_rank(*p));
    profiles.dedup();
}

// ---------------------------------------------------------------------------
// Card sub-objects
// ---------------------------------------------------------------------------

/// The source-age truth a card discloses for the data it shows: when it was observed, the moment the
/// data is current as of, and a human-facing age label. Dates are opaque strings (or absent), so the
/// packet never depends on a clock and stays exportable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceAge {
    /// When the data was last observed / fetched (absent when never observed).
    pub observed_at: Option<String>,
    /// The moment the data is current as of (absent when unknown).
    pub as_of: Option<String>,
    /// A human-facing age label, e.g. "live", "4h old", "cached 2026-06-20". Always present.
    pub age_label: String,
}

impl SourceAge {
    /// A live source observed and current at the same instant.
    pub fn live(observed_at: &str) -> Self {
        Self {
            observed_at: Some(observed_at.to_owned()),
            as_of: Some(observed_at.to_owned()),
            age_label: "live".to_owned(),
        }
    }

    /// A downgraded source with an explicit age label.
    pub fn aged(observed_at: Option<&str>, as_of: Option<&str>, age_label: &str) -> Self {
        Self {
            observed_at: observed_at.map(str::to_owned),
            as_of: as_of.map(str::to_owned),
            age_label: age_label.to_owned(),
        }
    }

    /// True when the source-age is disclosed coherently: an age label is always present.
    fn is_disclosed(&self) -> bool {
        !self.age_label.is_empty()
    }
}

/// The recovery / continuation guidance a card discloses: the chosen path, opaque guidance refs, and a
/// routable recovery message id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryGuidance {
    /// The recovery / continuation path.
    pub action: RecoveryAction,
    /// Opaque refs to guidance (no raw payloads).
    pub guidance_refs: Vec<String>,
    /// Routable message id for the recovery / continuation guidance.
    pub recovery_message_id: String,
}

impl RecoveryGuidance {
    /// Builds recovery guidance for a target with the given path and refs.
    pub fn new(target_token: &str, action: RecoveryAction, guidance_refs: &[&str]) -> Self {
        Self {
            action,
            guidance_refs: guidance_refs.iter().map(|s| (*s).to_owned()).collect(),
            recovery_message_id: format!(
                "{}recovery.{}",
                M5_SERVICE_HEALTH_MESSAGE_ID_PREFIX, target_token
            ),
        }
    }

    /// The no-op "not applicable" guidance for a healthy, live target.
    pub fn none(target_token: &str) -> Self {
        Self::new(target_token, RecoveryAction::NotApplicable, &[])
    }

    /// True when the guidance names a real continuation / recovery path with backing refs.
    fn is_active(&self) -> bool {
        self.action.is_active_path()
            && !self.guidance_refs.is_empty()
            && !self.recovery_message_id.is_empty()
    }
}

/// The local-safe continuation statement a card discloses: whether local editing is safe given this
/// boundary, a routable message id naming what still works locally, and the recovery path. This is the
/// lane's guarantee that a managed or vendor outage never implies local editing or recovery is unsafe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuationStatement {
    /// True when local editing / recovery is safe given this boundary's trouble.
    pub local_safe: bool,
    /// Routable message id naming what still works locally (always present).
    pub what_works_locally_message_id: String,
    /// The recovery / continuation guidance.
    pub recovery: RecoveryGuidance,
}

impl ContinuationStatement {
    /// Builds a continuation statement for a target.
    pub fn new(target_token: &str, local_safe: bool, recovery: RecoveryGuidance) -> Self {
        Self {
            local_safe,
            what_works_locally_message_id: format!(
                "{}continuation.{}",
                M5_SERVICE_HEALTH_MESSAGE_ID_PREFIX, target_token
            ),
            recovery,
        }
    }

    /// True when the statement carries a real continuation / recovery path and names what works
    /// locally.
    fn is_active(&self) -> bool {
        !self.what_works_locally_message_id.is_empty() && self.recovery.is_active()
    }
}

// ---------------------------------------------------------------------------
// Service-tier health card
// ---------------------------------------------------------------------------

/// Builder input for [`ServiceTierHealthCard::new`].
#[derive(Debug, Clone)]
pub struct ServiceTierHealthCardInput {
    /// The tier this card covers.
    pub tier: ServiceTier,
    /// The tier's current health state.
    pub health_state: HealthState,
    /// The release-data state of the data shown for it.
    pub release_data_state: ReleaseDataState,
    /// The source-age truth behind that data.
    pub source_age: SourceAge,
    /// The local-safe continuation statement.
    pub continuation: ContinuationStatement,
    /// The deployment profiles this tier covers.
    pub profiles: Vec<DeploymentProfile>,
    /// Opaque evidence refs backing the card (no raw payloads).
    pub evidence_refs: Vec<String>,
}

/// The typed service-health card for one [tier](ServiceTier): its identity, health state, the
/// release-data state of the data shown for it, the source-age truth, the local-safe continuation
/// statement, and the derived verdict. The card's gate is the *worse* of the health and release-data
/// postures, so a card never makes downgraded data look live.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceTierHealthCard {
    /// The tier.
    pub tier: ServiceTier,
    /// Human-facing tier label.
    pub tier_label: String,
    /// One-line tier identity blurb.
    pub tier_description: String,
    /// Accountable owner role.
    pub owner_role: String,
    /// True when this boundary's trouble can make local editing unsafe (the local machine only).
    pub affects_local_editing: bool,
    /// True for the optional vendor-hosted boundary.
    pub is_optional: bool,
    /// The current health state.
    pub health_state: HealthState,
    /// Reviewer-facing health-state label.
    pub health_state_label: String,
    /// The release-data state of the data shown for the tier.
    pub release_data_state: ReleaseDataState,
    /// Reviewer-facing release-data-state label.
    pub release_data_state_label: String,
    /// The source-age truth behind the data.
    pub source_age: SourceAge,
    /// The local-safe continuation statement.
    pub continuation: ContinuationStatement,
    /// The deployment profiles this tier covers.
    pub profiles: Vec<DeploymentProfile>,
    /// Opaque evidence refs backing the card.
    pub evidence_refs: Vec<String>,
    /// True when the card carries the continuation / recovery guidance a card under trouble must carry
    /// instead of a bare red banner.
    pub carries_recovery_guidance: bool,
    /// True when local editing is safe given this boundary (derived).
    pub local_editing_safe: bool,
    /// Gate: the worse of the health and release-data postures.
    pub gate: DescriptorGate,
    /// Readiness mirroring [`gate`](Self::gate).
    pub readiness: HealthReadiness,
    /// Coverage status mirroring [`gate`](Self::gate).
    pub status: ConsumerStatus,
    /// Traffic-light signal mirroring [`gate`](Self::gate).
    pub signal: DescriptorSignal,
    /// True when the tier has no live data (outage / unavailable).
    pub no_live_data: bool,
    /// Routable message id for the card's summary line.
    pub summary_message_id: String,
    /// Routable message id for the card's detail.
    pub detail_message_id: String,
}

impl ServiceTierHealthCard {
    /// Builds a tier card from its inputs, deriving the gate, readiness, local-editing-safe flag, and
    /// recovery-guidance flag.
    pub fn new(input: ServiceTierHealthCardInput) -> Self {
        let tier = input.tier;
        let mut card = Self {
            tier,
            tier_label: tier.label().to_owned(),
            tier_description: tier.description().to_owned(),
            owner_role: tier.owner_role().to_owned(),
            affects_local_editing: tier.affects_local_editing(),
            is_optional: tier.is_optional(),
            health_state: input.health_state,
            health_state_label: input.health_state.label().to_owned(),
            release_data_state: input.release_data_state,
            release_data_state_label: input.release_data_state.label().to_owned(),
            source_age: input.source_age,
            continuation: input.continuation,
            profiles: input.profiles,
            evidence_refs: input.evidence_refs,
            carries_recovery_guidance: false,
            local_editing_safe: true,
            gate: DescriptorGate::Governed,
            readiness: HealthReadiness::LiveTrusted,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            no_live_data: false,
            summary_message_id: format!(
                "{}tier.{}.summary",
                M5_SERVICE_HEALTH_MESSAGE_ID_PREFIX,
                tier.as_str()
            ),
            detail_message_id: format!(
                "{}tier.{}.detail",
                M5_SERVICE_HEALTH_MESSAGE_ID_PREFIX,
                tier.as_str()
            ),
        };
        card.recompute();
        card
    }

    /// Recomputes the derived verdict, local-editing-safe flag, recovery-guidance flag, and sorted
    /// scope.
    pub fn recompute(&mut self) {
        sort_profiles(&mut self.profiles);

        let gate = worst_gate(
            self.health_state.gate_posture(),
            self.release_data_state.gate_posture(),
        );
        self.gate = gate;
        self.readiness = HealthReadiness::from_gate(gate);
        self.status = status_for_gate(gate);
        self.signal = signal_for_gate(gate);
        self.no_live_data = gate == DescriptorGate::Blocked;
        // The local machine is the only boundary whose trouble can make local editing unsafe; a
        // remote, control-plane, or vendor card is always local-safe.
        self.local_editing_safe = if self.tier.affects_local_editing() {
            self.health_state != HealthState::Outage
        } else {
            true
        };
        self.carries_recovery_guidance = self.continuation.is_active();
    }

    /// True when the tier is under any trouble (health or data not fully live).
    fn needs_recovery_guidance(&self) -> bool {
        self.gate != DescriptorGate::Governed
    }

    /// The gap kind this card contributes to a consumer that reads it, if any.
    fn gap_kind(&self) -> Option<HealthGapKind> {
        match self.gate {
            DescriptorGate::Governed => None,
            DescriptorGate::Narrowed => Some(HealthGapKind::DataDowngraded),
            DescriptorGate::Blocked => Some(HealthGapKind::NoLiveDataFromTier),
        }
    }
}

// ---------------------------------------------------------------------------
// Admin-note card
// ---------------------------------------------------------------------------

/// Builder input for [`AdminNoteCard::new`].
#[derive(Debug, Clone)]
pub struct AdminNoteCardInput {
    /// The note kind this card covers.
    pub kind: AdminNoteKind,
    /// The tier the note affects.
    pub affected_tier: ServiceTier,
    /// The channel the note affects, if any.
    pub affected_channel: Option<ChannelScope>,
    /// The release-data state the note sets / explains.
    pub release_data_state: ReleaseDataState,
    /// The date the note is effective from, if any.
    pub effective_from: Option<String>,
    /// The source-age truth behind the note.
    pub source_age: SourceAge,
    /// True when the note has been acknowledged.
    pub acknowledged: bool,
    /// The local-safe continuation statement.
    pub continuation: ContinuationStatement,
    /// Opaque evidence refs backing the note (no raw payloads).
    pub evidence_refs: Vec<String>,
}

/// The typed admin-note card for one [kind](AdminNoteKind): channel, mirror, or deployment change.
/// It reuses the same release-data vocabulary and an export-safe evidence path so an admin note reads
/// identically across the UI, docs/help, and support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminNoteCard {
    /// The note kind.
    pub kind: AdminNoteKind,
    /// Human-facing note label.
    pub kind_label: String,
    /// Accountable owner role.
    pub owner_role: String,
    /// The tier the note affects.
    pub affected_tier: ServiceTier,
    /// The channel the note affects, if any.
    pub affected_channel: Option<ChannelScope>,
    /// The release-data state the note sets / explains.
    pub release_data_state: ReleaseDataState,
    /// Reviewer-facing release-data-state label.
    pub release_data_state_label: String,
    /// The date the note is effective from, if any.
    pub effective_from: Option<String>,
    /// The source-age truth behind the note.
    pub source_age: SourceAge,
    /// True when the note has been acknowledged.
    pub acknowledged: bool,
    /// The local-safe continuation statement.
    pub continuation: ContinuationStatement,
    /// Opaque evidence refs backing the note.
    pub evidence_refs: Vec<String>,
    /// True when the card carries continuation / recovery guidance instead of a bare note.
    pub carries_recovery_guidance: bool,
    /// Gate: the release-data posture the note sets.
    pub gate: DescriptorGate,
    /// Readiness mirroring [`gate`](Self::gate).
    pub readiness: HealthReadiness,
    /// Coverage status mirroring [`gate`](Self::gate).
    pub status: ConsumerStatus,
    /// Traffic-light signal mirroring [`gate`](Self::gate).
    pub signal: DescriptorSignal,
    /// Routable message id for the note's summary line.
    pub note_message_id: String,
    /// Routable message id for the note's detail.
    pub detail_message_id: String,
}

impl AdminNoteCard {
    /// Builds an admin-note card from its inputs, deriving the gate, readiness, and recovery-guidance
    /// flag.
    pub fn new(input: AdminNoteCardInput) -> Self {
        let kind = input.kind;
        let mut card = Self {
            kind,
            kind_label: kind.label().to_owned(),
            owner_role: kind.owner_role().to_owned(),
            affected_tier: input.affected_tier,
            affected_channel: input.affected_channel,
            release_data_state: input.release_data_state,
            release_data_state_label: input.release_data_state.label().to_owned(),
            effective_from: input.effective_from,
            source_age: input.source_age,
            acknowledged: input.acknowledged,
            continuation: input.continuation,
            evidence_refs: input.evidence_refs,
            carries_recovery_guidance: false,
            gate: DescriptorGate::Governed,
            readiness: HealthReadiness::LiveTrusted,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            note_message_id: format!(
                "{}note.{}.summary",
                M5_SERVICE_HEALTH_MESSAGE_ID_PREFIX,
                kind.as_str()
            ),
            detail_message_id: format!(
                "{}note.{}.detail",
                M5_SERVICE_HEALTH_MESSAGE_ID_PREFIX,
                kind.as_str()
            ),
        };
        card.recompute();
        card
    }

    /// Recomputes the derived verdict and recovery-guidance flag.
    pub fn recompute(&mut self) {
        let gate = self.release_data_state.gate_posture();
        self.gate = gate;
        self.readiness = HealthReadiness::from_gate(gate);
        self.status = status_for_gate(gate);
        self.signal = signal_for_gate(gate);
        self.carries_recovery_guidance = self.continuation.is_active();
    }

    /// True when the note downgrades the data and so must carry continuation / recovery guidance.
    fn needs_recovery_guidance(&self) -> bool {
        self.release_data_state.is_downgraded()
    }

    /// The gap kind this note contributes to a consumer that reads it, if any.
    fn gap_kind(&self) -> Option<HealthGapKind> {
        match self.gate {
            DescriptorGate::Governed => None,
            DescriptorGate::Narrowed => Some(HealthGapKind::DataDowngraded),
            DescriptorGate::Blocked => Some(HealthGapKind::NoLiveDataFromTier),
        }
    }
}

// ---------------------------------------------------------------------------
// Boundary summary
// ---------------------------------------------------------------------------

/// A one-line, exportable per-tier status row, so a user can tell the four boundaries apart at a
/// glance: which is in trouble, how trustworthy its data is, and whether it is local-safe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryStatus {
    /// The tier.
    pub tier: ServiceTier,
    /// Human-facing tier label.
    pub tier_label: String,
    /// The current health state.
    pub health_state: HealthState,
    /// The release-data state of the data shown for it.
    pub release_data_state: ReleaseDataState,
    /// True when local editing is safe given this boundary.
    pub local_editing_safe: bool,
    /// True for the optional vendor-hosted boundary.
    pub is_optional: bool,
    /// Traffic-light signal.
    pub signal: DescriptorSignal,
    /// Routable message id for the boundary status.
    pub status_message_id: String,
}

impl BoundaryStatus {
    fn from_card(card: &ServiceTierHealthCard) -> Self {
        Self {
            tier: card.tier,
            tier_label: card.tier_label.clone(),
            health_state: card.health_state,
            release_data_state: card.release_data_state,
            local_editing_safe: card.local_editing_safe,
            is_optional: card.is_optional,
            signal: card.signal,
            status_message_id: format!(
                "{}boundary.{}.status",
                M5_SERVICE_HEALTH_MESSAGE_ID_PREFIX,
                card.tier.as_str()
            ),
        }
    }
}

// ---------------------------------------------------------------------------
// Consumer rows
// ---------------------------------------------------------------------------

/// A gap a consumer carries for one tier or admin-note card it reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthGap {
    /// The consumer that carries the gap.
    pub consumer: HealthConsumer,
    /// Whether the gap points at a tier or an admin note.
    pub target_kind: HealthTargetKind,
    /// The tier / note token the gap points at.
    pub target_token: String,
    /// The named cause of the gap.
    pub gap_kind: HealthGapKind,
    /// Routable message id naming the cause.
    pub cause_message_id: String,
}

fn make_gap(
    consumer: HealthConsumer,
    target_kind: HealthTargetKind,
    target_token: &str,
    kind: HealthGapKind,
) -> HealthGap {
    HealthGap {
        consumer,
        target_kind,
        target_token: target_token.to_owned(),
        gap_kind: kind,
        cause_message_id: format!(
            "{}consumer.{}.{}.{}.{}.gap",
            M5_SERVICE_HEALTH_MESSAGE_ID_PREFIX,
            consumer.as_str(),
            target_kind.as_str(),
            target_token,
            kind.as_str()
        ),
    }
}

/// A consumer surface bound to the tiers and admin notes it reads, with its readiness, decision, and
/// gaps derived from those cards. Every consumer also carries the packet's local-continuation truth, so
/// no surface reads as fully broken when only a remote / vendor boundary is in trouble.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthConsumerRow {
    /// The consumer surface.
    pub consumer: HealthConsumer,
    /// Human-facing label.
    pub consumer_label: String,
    /// Accountable owner role.
    pub owner_role: String,
    /// The tiers this consumer reads.
    pub read_tiers: Vec<ServiceTier>,
    /// The admin notes this consumer reads.
    pub read_notes: Vec<AdminNoteKind>,
    /// The union of deployment profiles across the read cards.
    pub profiles: Vec<DeploymentProfile>,
    /// The derived readiness.
    pub readiness: HealthReadiness,
    /// Coverage status.
    pub status: ConsumerStatus,
    /// Traffic-light signal.
    pub signal: DescriptorSignal,
    /// Gate decision.
    pub gate_decision: DescriptorGate,
    /// True when local editing remains safe (carried from the local-machine card, regardless of the
    /// gate decision).
    pub local_continuation_safe: bool,
    /// Gaps, one per (target, cause).
    pub gaps: Vec<HealthGap>,
    /// Routable status message id.
    pub status_message_id: String,
    /// Routable decision message id.
    pub decision_message_id: String,
}

impl HealthConsumerRow {
    /// Builds a consumer row; the resolved unions, gaps, and verdict are recomputed against the
    /// packet's cards when the packet is assembled.
    pub fn new(
        consumer: HealthConsumer,
        read_tiers: &[ServiceTier],
        read_notes: &[AdminNoteKind],
    ) -> Self {
        Self {
            consumer,
            consumer_label: consumer.label().to_owned(),
            owner_role: consumer.owner_role().to_owned(),
            read_tiers: read_tiers.to_vec(),
            read_notes: read_notes.to_vec(),
            profiles: Vec::new(),
            readiness: HealthReadiness::LiveTrusted,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            gate_decision: DescriptorGate::Governed,
            local_continuation_safe: true,
            gaps: Vec::new(),
            status_message_id: format!(
                "{}consumer.{}.status",
                M5_SERVICE_HEALTH_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
            decision_message_id: format!(
                "{}consumer.{}.decision",
                M5_SERVICE_HEALTH_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
        }
    }

    /// Recomputes the resolved unions, gaps, and verdict from the packet's cards, so a consumer's
    /// readiness is always generated from the same checked-in cards rather than a hand-maintained
    /// status.
    pub fn recompute(
        &mut self,
        tiers: &[ServiceTierHealthCard],
        notes: &[AdminNoteCard],
        local_continuation_safe: bool,
    ) {
        let mut read_tiers = self.read_tiers.clone();
        read_tiers.sort_by_key(|t| tier_rank(*t));
        read_tiers.dedup();
        self.read_tiers = read_tiers.clone();

        let mut read_notes = self.read_notes.clone();
        read_notes.sort_by_key(|k| note_rank(*k));
        read_notes.dedup();
        self.read_notes = read_notes.clone();

        let consumer = self.consumer;
        let mut profiles: Vec<DeploymentProfile> = Vec::new();
        let mut gaps: Vec<HealthGap> = Vec::new();

        for &tier in &read_tiers {
            match tiers.iter().find(|c| c.tier == tier) {
                None => gaps.push(make_gap(
                    consumer,
                    HealthTargetKind::Tier,
                    tier.as_str(),
                    HealthGapKind::TierNotCarded,
                )),
                Some(card) => {
                    profiles.extend(card.profiles.iter().copied());
                    if let Some(kind) = card.gap_kind() {
                        gaps.push(make_gap(
                            consumer,
                            HealthTargetKind::Tier,
                            tier.as_str(),
                            kind,
                        ));
                    }
                }
            }
        }

        for &note in &read_notes {
            match notes.iter().find(|c| c.kind == note) {
                None => gaps.push(make_gap(
                    consumer,
                    HealthTargetKind::AdminNote,
                    note.as_str(),
                    HealthGapKind::AdminNoteNotPropagated,
                )),
                Some(card) => {
                    if let Some(kind) = card.gap_kind() {
                        gaps.push(make_gap(
                            consumer,
                            HealthTargetKind::AdminNote,
                            note.as_str(),
                            kind,
                        ));
                    }
                }
            }
        }

        sort_profiles(&mut profiles);
        gaps.sort_by(|a, b| {
            a.target_kind
                .as_str()
                .cmp(b.target_kind.as_str())
                .then(a.target_token.cmp(&b.target_token))
                .then(a.gap_kind.as_str().cmp(b.gap_kind.as_str()))
        });

        self.profiles = profiles;
        self.gaps = gaps;
        self.local_continuation_safe = local_continuation_safe;

        let mut gate = DescriptorGate::Governed;
        for gap in &self.gaps {
            gate = worst_gate(gate, gap.gap_kind.gate());
        }
        self.gate_decision = gate;
        self.readiness = HealthReadiness::from_gate(gate);
        self.status = status_for_gate(gate);
        self.signal = signal_for_gate(gate);
    }

    /// True when every read card is live and trusted.
    pub fn is_live_trusted(&self) -> bool {
        self.gate_decision == DescriptorGate::Governed
    }

    /// True when at least one read card is showing downgraded data.
    pub fn is_showing_downgraded(&self) -> bool {
        self.gate_decision == DescriptorGate::Narrowed
    }

    /// True when at least one read tier / note has no live data.
    pub fn is_no_live_data(&self) -> bool {
        self.gate_decision == DescriptorGate::Blocked
    }
}

// ---------------------------------------------------------------------------
// Aggregate sub-objects
// ---------------------------------------------------------------------------

/// Disclosure flags asserting every claimed consumer ingests this one packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthDisclosure {
    /// The service-health panel consumes the packet.
    pub service_health_panel_reads_packet: bool,
    /// Help / About consumes the packet.
    pub help_about_reads_packet: bool,
    /// Docs / help consumes the packet.
    pub docs_help_reads_packet: bool,
    /// Support export consumes the packet.
    pub support_export_reads_packet: bool,
    /// The admin console consumes the packet.
    pub admin_console_reads_packet: bool,
    /// The release center consumes the packet.
    pub release_center_reads_packet: bool,
}

impl HealthDisclosure {
    fn canonical() -> Self {
        Self {
            service_health_panel_reads_packet: true,
            help_about_reads_packet: true,
            docs_help_reads_packet: true,
            support_export_reads_packet: true,
            admin_console_reads_packet: true,
            release_center_reads_packet: true,
        }
    }

    /// True when every consumer is asserted to consume the packet.
    pub fn all_consume(&self) -> bool {
        self.service_health_panel_reads_packet
            && self.help_about_reads_packet
            && self.docs_help_reads_packet
            && self.support_export_reads_packet
            && self.admin_console_reads_packet
            && self.release_center_reads_packet
    }
}

/// Roll-up counts over the cards and consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthCounts {
    /// Total tier cards.
    pub total_tiers: u32,
    /// Tier cards live and trusted.
    pub live_tiers: u32,
    /// Tier cards showing downgraded data.
    pub downgraded_tiers: u32,
    /// Tier cards with no live data.
    pub no_live_data_tiers: u32,
    /// Total admin-note cards.
    pub total_notes: u32,
    /// Admin-note cards that downgrade data.
    pub downgrading_notes: u32,
    /// Admin-note cards not yet acknowledged.
    pub unacknowledged_notes: u32,
    /// Total consumers.
    pub total_consumers: u32,
    /// Consumers live and trusted.
    pub live_trusted_consumers: u32,
    /// Consumers showing downgraded data.
    pub showing_downgraded_consumers: u32,
    /// Consumers with no live data.
    pub no_live_data_consumers: u32,
    /// Whether local editing remains safe.
    pub local_editing_safe: bool,
}

/// The packet-level data-trust honesty block: how much of what is shown is live vs. downgraded or
/// unavailable, so staleness is disclosed rather than implied absent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthCoverage {
    /// Cards (tier or note) showing live data.
    pub live_data_cards: u32,
    /// Cards showing visibly downgraded data.
    pub downgraded_data_cards: u32,
    /// Cards with no live data.
    pub no_live_data_cards: u32,
    /// True when at least one card is showing downgraded or no live data.
    pub has_data_downgrade: bool,
    /// True when local editing remains safe.
    pub local_editing_safe: bool,
    /// The data state the packet was rendered under, labelled honestly.
    pub data_state: StaleDataBehavior,
    /// True when the packet is showing live, verified data end-to-end.
    pub live_data: bool,
}

/// The packet-level continuity block aggregating the per-tier boundaries, so one degraded service
/// never reads as the whole product being broken and local-only continuation stays explicit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthContinuity {
    /// True when local editing / recovery remains safe.
    pub local_editing_safe: bool,
    /// Tokens of the tiers serving live data.
    pub live_tiers: Vec<String>,
    /// Tokens of the tiers showing downgraded data.
    pub degraded_tiers: Vec<String>,
    /// Tokens of the tiers with no live data (outage / unavailable).
    pub outage_tiers: Vec<String>,
    /// Tokens of the boundaries in any trouble (degraded or outage).
    pub affected_boundaries: Vec<String>,
    /// Tokens of the admin notes not yet acknowledged.
    pub unacknowledged_notes: Vec<String>,
    /// Routable continuity message id.
    pub continuity_message_id: String,
}

/// The frozen controlled vocabulary the cards draw from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthVocabulary {
    /// Tier tokens.
    pub tiers: Vec<String>,
    /// Health-state tokens.
    pub health_states: Vec<String>,
    /// Release-data-state tokens.
    pub release_data_states: Vec<String>,
    /// Admin-note-kind tokens.
    pub admin_note_kinds: Vec<String>,
    /// Recovery-action tokens.
    pub recovery_actions: Vec<String>,
    /// Channel tokens.
    pub channels: Vec<String>,
    /// Profile tokens.
    pub profiles: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
    /// Gap-kind tokens.
    pub gap_kinds: Vec<String>,
    /// Target-kind tokens.
    pub target_kinds: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
    /// Readiness tokens.
    pub readiness: Vec<String>,
    /// Stale-data-behavior tokens.
    pub stale_data_behaviors: Vec<String>,
}

impl HealthVocabulary {
    /// The canonical frozen vocabulary.
    pub fn canonical() -> Self {
        Self {
            tiers: tokens(&ServiceTier::ALL, |x| x.as_str()),
            health_states: tokens(&HealthState::ALL, |x| x.as_str()),
            release_data_states: tokens(&ReleaseDataState::ALL, |x| x.as_str()),
            admin_note_kinds: tokens(&AdminNoteKind::ALL, |x| x.as_str()),
            recovery_actions: tokens(&RecoveryAction::ALL, |x| x.as_str()),
            channels: tokens(&ChannelScope::ALL, |x| x.as_str()),
            profiles: tokens(&DeploymentProfile::ALL, |x| x.as_str()),
            consumers: tokens(&HealthConsumer::ALL, |x| x.as_str()),
            gap_kinds: tokens(&HealthGapKind::ALL, |x| x.as_str()),
            target_kinds: tokens(&HealthTargetKind::ALL, |x| x.as_str()),
            gate_decisions: tokens(&DescriptorGate::ALL, |x| x.as_str()),
            readiness: tokens(&HealthReadiness::ALL, |x| x.as_str()),
            stale_data_behaviors: tokens(&StaleDataBehavior::ALL, |x| x.as_str()),
        }
    }

    /// True when this vocabulary equals the canonical frozen vocabulary.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Conformance flags every canonical packet asserts. They restate the acceptance bar so a tampered
/// packet that flips one to false fails [`ServiceHealthCommunication::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthConformance {
    /// Every tier is carded exactly once.
    pub every_tier_carded: bool,
    /// Every admin-note kind is carded exactly once.
    pub every_admin_note_carded: bool,
    /// The four boundaries are distinguishable: local, remote, control-plane, vendor each carry their
    /// own health.
    pub boundaries_distinguishable: bool,
    /// Cached / mirrored / stale / policy-limited / local-only / unavailable data is labelled
    /// distinctly and never looks live.
    pub data_states_labelled_distinctly: bool,
    /// Stale or mirrored data is visibly downgraded and exportable with source-age truth.
    pub stale_data_downgraded_with_source_age: bool,
    /// Admin notes are propagated using the same vocabulary and an export-safe evidence path.
    pub admin_notes_propagated_one_vocabulary: bool,
    /// Local-only continuity is explicit; a managed / vendor outage never implies local editing is
    /// unsafe.
    pub local_continuity_explicit: bool,
    /// A card under trouble carries continuation and recovery guidance instead of a bare banner.
    pub trouble_cards_carry_continuation_recovery: bool,
    /// No card makes downgraded data look live (no over-stated freshness).
    pub freshness_not_overstated: bool,
    /// Admin notes and service-health messages stay consistent across UI, docs/help, and support
    /// exports.
    pub messages_consistent_across_surfaces: bool,
    /// Every consumer verdict is derived from the cards, not hand-maintained.
    pub consumer_verdict_derived_from_cards: bool,
    /// The panel stays scoped to release/update/support continuity, not a generic alert feed.
    pub scoped_to_release_update_support_continuity: bool,
    /// The controlled enums are frozen.
    pub controlled_enums_frozen: bool,
    /// The packet is exportable and reviewable outside the app.
    pub exportable_outside_app: bool,
    /// The export carries metadata and refs only — no credential bodies or raw payloads.
    pub export_carries_no_raw_material: bool,
}

impl HealthConformance {
    fn canonical() -> Self {
        Self {
            every_tier_carded: true,
            every_admin_note_carded: true,
            boundaries_distinguishable: true,
            data_states_labelled_distinctly: true,
            stale_data_downgraded_with_source_age: true,
            admin_notes_propagated_one_vocabulary: true,
            local_continuity_explicit: true,
            trouble_cards_carry_continuation_recovery: true,
            freshness_not_overstated: true,
            messages_consistent_across_surfaces: true,
            consumer_verdict_derived_from_cards: true,
            scoped_to_release_update_support_continuity: true,
            controlled_enums_frozen: true,
            exportable_outside_app: true,
            export_carries_no_raw_material: true,
        }
    }

    /// True when every conformance flag holds.
    pub fn all_hold(&self) -> bool {
        *self == Self::canonical()
    }
}

// ---------------------------------------------------------------------------
// Render channel
// ---------------------------------------------------------------------------

/// The render channels the packet must serialize identically across.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderChannel {
    /// The desktop service-health panel / Help / About.
    DesktopUi,
    /// The CLI / headless emitter.
    CliHeadless,
    /// The offline / exported review surface.
    OfflineExport,
}

// ---------------------------------------------------------------------------
// Validation violations
// ---------------------------------------------------------------------------

/// A reason a packet failed [`ServiceHealthCommunication::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthViolation {
    /// The record kind or schema version is wrong.
    HeaderDrift,
    /// A tier is missing or carded more than once.
    TierCoverageDrift,
    /// An admin-note kind is missing or carded more than once.
    NoteCoverageDrift,
    /// A tier card's derived verdict / scope / flags drifted.
    TierDerivationDrift,
    /// An admin-note card's derived verdict / flags drifted.
    NoteDerivationDrift,
    /// A card makes downgraded data look live — the lane's guardrail against over-stated freshness.
    OverstatedDataFreshness,
    /// A card under trouble lacks continuation / recovery guidance.
    MissingContinuationGuidance,
    /// A non-local boundary is marked local-unsafe, or the local-machine card's local-safe flag is
    /// wrong — local continuity was misreported.
    MisreportedLocalContinuity,
    /// A source-age label is missing where data is shown.
    MissingSourceAge,
    /// A consumer's derived verdict, unions, or gaps drifted.
    ConsumerVerdictDrift,
    /// The summary counts, coverage, continuity, or boundary summary drifted.
    SummaryDrift,
    /// The disclosure flags do not all assert consumption of the one packet.
    DisclosureDrift,
    /// The controlled vocabulary drifted.
    VocabularyDrift,
    /// A conformance flag does not hold.
    ConformanceDrift,
    /// The export carried forbidden raw material.
    ForbiddenMaterial,
}

impl HealthViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeaderDrift => "header_drift",
            Self::TierCoverageDrift => "tier_coverage_drift",
            Self::NoteCoverageDrift => "note_coverage_drift",
            Self::TierDerivationDrift => "tier_derivation_drift",
            Self::NoteDerivationDrift => "note_derivation_drift",
            Self::OverstatedDataFreshness => "overstated_data_freshness",
            Self::MissingContinuationGuidance => "missing_continuation_guidance",
            Self::MisreportedLocalContinuity => "misreported_local_continuity",
            Self::MissingSourceAge => "missing_source_age",
            Self::ConsumerVerdictDrift => "consumer_verdict_drift",
            Self::SummaryDrift => "summary_drift",
            Self::DisclosureDrift => "disclosure_drift",
            Self::VocabularyDrift => "vocabulary_drift",
            Self::ConformanceDrift => "conformance_drift",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

// ---------------------------------------------------------------------------
// Packet
// ---------------------------------------------------------------------------

/// Builder input for [`ServiceHealthCommunication::new`].
#[derive(Debug, Clone)]
pub struct ServiceHealthCommunicationInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-facing report label.
    pub report_label: String,
    /// Evaluation timestamp.
    pub evaluated_at: String,
    /// The data state the packet was rendered under.
    pub data_state: StaleDataBehavior,
    /// The per-tier health cards.
    pub tiers: Vec<ServiceTierHealthCard>,
    /// The per-kind admin-note cards.
    pub notes: Vec<AdminNoteCard>,
    /// The claimed consumer rows.
    pub consumers: Vec<HealthConsumerRow>,
    /// Redaction-class token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable service-health and stale-or-mirrored release-data packet the
/// service-health panel, Help/About, docs/help, support export, the admin console, and the release
/// center consume to tell which boundary is in trouble, how trustworthy the data is, what admin notes
/// apply, and what still works locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthCommunication {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-facing report label.
    pub report_label: String,
    /// Evaluation timestamp.
    pub evaluated_at: String,
    /// The data state the packet was rendered under, labelled honestly.
    pub data_state: StaleDataBehavior,
    /// The per-tier health cards.
    pub tiers: Vec<ServiceTierHealthCard>,
    /// The tier tokens, in canonical order.
    pub tier_tokens: Vec<String>,
    /// The per-kind admin-note cards.
    pub notes: Vec<AdminNoteCard>,
    /// The admin-note tokens, in canonical order.
    pub note_tokens: Vec<String>,
    /// The per-tier boundary summary, so the four boundaries are distinguishable at a glance.
    pub boundaries: Vec<BoundaryStatus>,
    /// The consumer rows reading the cards.
    pub consumers: Vec<HealthConsumerRow>,
    /// The consumer tokens, in canonical order.
    pub consumer_tokens: Vec<String>,
    /// Disclosure flags.
    pub disclosure: HealthDisclosure,
    /// Roll-up counts.
    pub summary: HealthCounts,
    /// Data-trust honesty block.
    pub coverage: HealthCoverage,
    /// Packet-level continuity block.
    pub continuity: HealthContinuity,
    /// Controlled vocabulary.
    pub vocabulary: HealthVocabulary,
    /// Conformance flags.
    pub conformance: HealthConformance,
    /// Redaction-class token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl ServiceHealthCommunication {
    /// Builds a packet from the given cards and consumer rows, recomputing every derived field so the
    /// published packet is always generated from the same checked-in cards.
    pub fn new(input: ServiceHealthCommunicationInput) -> Self {
        let mut tiers = input.tiers;
        for card in &mut tiers {
            card.recompute();
        }
        tiers.sort_by_key(|c| tier_rank(c.tier));

        let mut notes = input.notes;
        for card in &mut notes {
            card.recompute();
        }
        notes.sort_by_key(|c| note_rank(c.kind));

        let local_editing_safe = local_editing_safe(&tiers);

        let mut consumers = input.consumers;
        for consumer in &mut consumers {
            consumer.recompute(&tiers, &notes, local_editing_safe);
        }
        consumers.sort_by_key(|c| consumer_rank(c.consumer));

        let boundaries: Vec<BoundaryStatus> = tiers.iter().map(BoundaryStatus::from_card).collect();
        let summary = derive_counts(&tiers, &notes, &consumers, local_editing_safe);
        let coverage = derive_coverage(&tiers, &notes, input.data_state, local_editing_safe);
        let continuity = derive_continuity(&tiers, &notes, local_editing_safe);

        Self {
            record_kind: M5_SERVICE_HEALTH_COMMUNICATION_RECORD_KIND.to_owned(),
            schema_version: M5_SERVICE_HEALTH_COMMUNICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            data_state: input.data_state,
            tier_tokens: tokens(&ServiceTier::ALL, |x| x.as_str()),
            tiers,
            note_tokens: tokens(&AdminNoteKind::ALL, |x| x.as_str()),
            notes,
            boundaries,
            consumer_tokens: tokens(&HealthConsumer::ALL, |x| x.as_str()),
            consumers,
            disclosure: HealthDisclosure::canonical(),
            summary,
            coverage,
            continuity,
            vocabulary: HealthVocabulary::canonical(),
            conformance: HealthConformance::canonical(),
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Looks up the card for a tier.
    pub fn tier(&self, tier: ServiceTier) -> Option<&ServiceTierHealthCard> {
        self.tiers.iter().find(|c| c.tier == tier)
    }

    /// Looks up the card for an admin-note kind.
    pub fn note(&self, kind: AdminNoteKind) -> Option<&AdminNoteCard> {
        self.notes.iter().find(|c| c.kind == kind)
    }

    /// Looks up the consumer row for a consumer.
    pub fn consumer(&self, consumer: HealthConsumer) -> Option<&HealthConsumerRow> {
        self.consumers.iter().find(|c| c.consumer == consumer)
    }

    /// Whether local editing / recovery remains safe.
    pub fn local_editing_safe(&self) -> bool {
        self.continuity.local_editing_safe
    }

    /// Validates every derived field by recomputing it from the cards and comparing. Returns an empty
    /// vector when the packet is internally consistent.
    pub fn validate(&self) -> Vec<HealthViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SERVICE_HEALTH_COMMUNICATION_RECORD_KIND
            || self.schema_version != M5_SERVICE_HEALTH_COMMUNICATION_SCHEMA_VERSION
        {
            violations.push(HealthViolation::HeaderDrift);
        }

        // Every tier carded exactly once.
        for tier in ServiceTier::ALL {
            if self.tiers.iter().filter(|c| c.tier == tier).count() != 1 {
                violations.push(HealthViolation::TierCoverageDrift);
                break;
            }
        }
        // Every admin-note kind carded exactly once.
        for kind in AdminNoteKind::ALL {
            if self.notes.iter().filter(|c| c.kind == kind).count() != 1 {
                violations.push(HealthViolation::NoteCoverageDrift);
                break;
            }
        }

        for card in &self.tiers {
            let mut fresh = card.clone();
            fresh.recompute();
            if fresh.gate != card.gate
                || fresh.readiness != card.readiness
                || fresh.status != card.status
                || fresh.signal != card.signal
                || fresh.no_live_data != card.no_live_data
                || fresh.local_editing_safe != card.local_editing_safe
                || fresh.carries_recovery_guidance != card.carries_recovery_guidance
                || fresh.profiles != card.profiles
            {
                violations.push(HealthViolation::TierDerivationDrift);
            }
            // Guardrail: the gate may never be less severe than the weakest posture warrants — so
            // mirrored / cached / stale / policy-limited data can never render as live.
            let warranted = worst_gate(
                card.health_state.gate_posture(),
                card.release_data_state.gate_posture(),
            );
            if gate_rank(card.gate) < gate_rank(warranted) {
                violations.push(HealthViolation::OverstatedDataFreshness);
            }
            // A card under trouble must carry continuation and recovery guidance.
            if card.needs_recovery_guidance() && !card.carries_recovery_guidance {
                violations.push(HealthViolation::MissingContinuationGuidance);
            }
            // A non-local boundary is always local-safe; the local machine is local-safe unless it is
            // itself in outage.
            let warranted_local_safe = if card.tier.affects_local_editing() {
                card.health_state != HealthState::Outage
            } else {
                true
            };
            if card.local_editing_safe != warranted_local_safe
                || card.continuation.local_safe != card.local_editing_safe
            {
                violations.push(HealthViolation::MisreportedLocalContinuity);
            }
            if !card.source_age.is_disclosed() {
                violations.push(HealthViolation::MissingSourceAge);
            }
        }

        for card in &self.notes {
            let mut fresh = card.clone();
            fresh.recompute();
            if fresh.gate != card.gate
                || fresh.readiness != card.readiness
                || fresh.status != card.status
                || fresh.signal != card.signal
                || fresh.carries_recovery_guidance != card.carries_recovery_guidance
            {
                violations.push(HealthViolation::NoteDerivationDrift);
            }
            if gate_rank(card.gate) < gate_rank(card.release_data_state.gate_posture()) {
                violations.push(HealthViolation::OverstatedDataFreshness);
            }
            if card.needs_recovery_guidance() && !card.carries_recovery_guidance {
                violations.push(HealthViolation::MissingContinuationGuidance);
            }
            if !card.source_age.is_disclosed() {
                violations.push(HealthViolation::MissingSourceAge);
            }
        }

        let local_safe = local_editing_safe(&self.tiers);
        for consumer in &self.consumers {
            let mut fresh = HealthConsumerRow::new(
                consumer.consumer,
                &consumer.read_tiers,
                &consumer.read_notes,
            );
            fresh.recompute(&self.tiers, &self.notes, local_safe);
            if fresh.gate_decision != consumer.gate_decision
                || fresh.readiness != consumer.readiness
                || fresh.status != consumer.status
                || fresh.signal != consumer.signal
                || fresh.local_continuation_safe != consumer.local_continuation_safe
                || fresh.profiles != consumer.profiles
                || fresh.gaps != consumer.gaps
            {
                violations.push(HealthViolation::ConsumerVerdictDrift);
                break;
            }
        }

        let boundaries: Vec<BoundaryStatus> =
            self.tiers.iter().map(BoundaryStatus::from_card).collect();
        if self.summary != derive_counts(&self.tiers, &self.notes, &self.consumers, local_safe)
            || self.coverage
                != derive_coverage(&self.tiers, &self.notes, self.data_state, local_safe)
            || self.continuity != derive_continuity(&self.tiers, &self.notes, local_safe)
            || self.boundaries != boundaries
        {
            violations.push(HealthViolation::SummaryDrift);
        }

        if !self.disclosure.all_consume()
            || self.tier_tokens != tokens(&ServiceTier::ALL, |x| x.as_str())
            || self.note_tokens != tokens(&AdminNoteKind::ALL, |x| x.as_str())
            || self.consumer_tokens != tokens(&HealthConsumer::ALL, |x| x.as_str())
        {
            violations.push(HealthViolation::DisclosureDrift);
        }

        if !self.vocabulary.matches_canonical() {
            violations.push(HealthViolation::VocabularyDrift);
        }

        if !self.conformance.all_hold() {
            violations.push(HealthViolation::ConformanceDrift);
        }

        if contains_forbidden_material(self) {
            violations.push(HealthViolation::ForbiddenMaterial);
        }

        violations
    }

    /// The canonical export form: pretty JSON, identical across every render channel.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("packet serializes")
    }

    /// Renders the packet for a channel. Every channel produces byte-identical output.
    pub fn render_for_channel(&self, _channel: RenderChannel) -> String {
        self.export_safe_json()
    }

    /// A compact Markdown summary of the boundaries, admin notes, and consumer verdicts, for export and
    /// review outside the app.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# {}\n\n", self.report_label));
        out.push_str(&format!(
            "{} tiers ({} downgraded, {} no-live-data), {} admin notes, {} consumers — data state `{}`; local editing {}.\n\n",
            self.summary.total_tiers,
            self.summary.downgraded_tiers,
            self.summary.no_live_data_tiers,
            self.summary.total_notes,
            self.summary.total_consumers,
            self.data_state.as_str(),
            if self.continuity.local_editing_safe { "safe" } else { "AT RISK" },
        ));
        out.push_str("## Service boundaries\n\n");
        out.push_str("| Boundary | Health | Data | Local-safe | Optional |\n");
        out.push_str("|---|---|---|---|---|\n");
        for b in &self.boundaries {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} | {} |\n",
                b.tier.as_str(),
                b.health_state.as_str(),
                b.release_data_state.as_str(),
                if b.local_editing_safe { "yes" } else { "no" },
                if b.is_optional { "yes" } else { "no" },
            ));
        }
        out.push_str("\n## Admin notes\n\n");
        out.push_str("| Note | Affected tier | Data | Acknowledged | Effective from |\n");
        out.push_str("|---|---|---|---|---|\n");
        for n in &self.notes {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} | {} |\n",
                n.kind.as_str(),
                n.affected_tier.as_str(),
                n.release_data_state.as_str(),
                if n.acknowledged { "yes" } else { "no" },
                n.effective_from.as_deref().unwrap_or("—"),
            ));
        }
        out.push_str("\n## Consumers\n\n");
        for c in &self.consumers {
            out.push_str(&format!(
                "- `{}` → `{}` ({}",
                c.consumer.as_str(),
                c.readiness.as_str(),
                c.gate_decision.as_str(),
            ));
            if c.gaps.is_empty() {
                out.push_str(")\n");
            } else {
                let gaps: Vec<String> = c
                    .gaps
                    .iter()
                    .map(|g| format!("{}:{}", g.target_token, g.gap_kind.as_str()))
                    .collect();
                out.push_str(&format!("; gap: {})\n", gaps.join(", ")));
            }
        }
        out
    }

    /// A machine-readable CSV of every card, for export and review outside the app.
    pub fn render_card_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "card_kind,target,health_state,release_data_state,source_age,local_safe,readiness,recovery,gate\n",
        );
        for c in &self.tiers {
            out.push_str(&format!(
                "tier,{},{},{},{},{},{},{},{}\n",
                c.tier.as_str(),
                c.health_state.as_str(),
                c.release_data_state.as_str(),
                csv_field(&c.source_age.age_label),
                if c.local_editing_safe { "yes" } else { "no" },
                c.readiness.as_str(),
                c.continuation.recovery.action.as_str(),
                c.gate.as_str(),
            ));
        }
        for n in &self.notes {
            out.push_str(&format!(
                "admin_note,{},,{},{},{},{},{},{}\n",
                n.kind.as_str(),
                n.release_data_state.as_str(),
                csv_field(&n.source_age.age_label),
                if n.continuation.local_safe {
                    "yes"
                } else {
                    "no"
                },
                n.readiness.as_str(),
                n.continuation.recovery.action.as_str(),
                n.gate.as_str(),
            ));
        }
        out
    }
}

/// True when local editing remains safe: the local-machine card is not itself in outage.
fn local_editing_safe(tiers: &[ServiceTierHealthCard]) -> bool {
    tiers
        .iter()
        .find(|c| c.tier == ServiceTier::LocalMachine)
        .map(|c| c.local_editing_safe)
        .unwrap_or(true)
}

fn derive_counts(
    tiers: &[ServiceTierHealthCard],
    notes: &[AdminNoteCard],
    consumers: &[HealthConsumerRow],
    local_editing_safe: bool,
) -> HealthCounts {
    let count_tiers = |gate: DescriptorGate| tiers.iter().filter(|c| c.gate == gate).count() as u32;
    HealthCounts {
        total_tiers: tiers.len() as u32,
        live_tiers: count_tiers(DescriptorGate::Governed),
        downgraded_tiers: count_tiers(DescriptorGate::Narrowed),
        no_live_data_tiers: count_tiers(DescriptorGate::Blocked),
        total_notes: notes.len() as u32,
        downgrading_notes: notes
            .iter()
            .filter(|c| c.release_data_state.is_downgraded())
            .count() as u32,
        unacknowledged_notes: notes.iter().filter(|c| !c.acknowledged).count() as u32,
        total_consumers: consumers.len() as u32,
        live_trusted_consumers: consumers.iter().filter(|c| c.is_live_trusted()).count() as u32,
        showing_downgraded_consumers: consumers
            .iter()
            .filter(|c| c.is_showing_downgraded())
            .count() as u32,
        no_live_data_consumers: consumers.iter().filter(|c| c.is_no_live_data()).count() as u32,
        local_editing_safe,
    }
}

fn derive_coverage(
    tiers: &[ServiceTierHealthCard],
    notes: &[AdminNoteCard],
    data_state: StaleDataBehavior,
    local_editing_safe: bool,
) -> HealthCoverage {
    let live = tiers
        .iter()
        .filter(|c| c.gate == DescriptorGate::Governed)
        .count()
        + notes
            .iter()
            .filter(|c| c.gate == DescriptorGate::Governed)
            .count();
    let downgraded = tiers
        .iter()
        .filter(|c| c.gate == DescriptorGate::Narrowed)
        .count()
        + notes
            .iter()
            .filter(|c| c.gate == DescriptorGate::Narrowed)
            .count();
    let no_live = tiers
        .iter()
        .filter(|c| c.gate == DescriptorGate::Blocked)
        .count()
        + notes
            .iter()
            .filter(|c| c.gate == DescriptorGate::Blocked)
            .count();
    HealthCoverage {
        live_data_cards: live as u32,
        downgraded_data_cards: downgraded as u32,
        no_live_data_cards: no_live as u32,
        has_data_downgrade: downgraded > 0 || no_live > 0,
        local_editing_safe,
        data_state,
        live_data: data_state == StaleDataBehavior::LiveVerified,
    }
}

fn derive_continuity(
    tiers: &[ServiceTierHealthCard],
    notes: &[AdminNoteCard],
    local_editing_safe: bool,
) -> HealthContinuity {
    let mut live_tiers = Vec::new();
    let mut degraded_tiers = Vec::new();
    let mut outage_tiers = Vec::new();
    let mut affected = Vec::new();
    for card in tiers {
        match card.gate {
            DescriptorGate::Governed => live_tiers.push(card.tier.as_str().to_owned()),
            DescriptorGate::Narrowed => {
                degraded_tiers.push(card.tier.as_str().to_owned());
                affected.push(card.tier.as_str().to_owned());
            }
            DescriptorGate::Blocked => {
                outage_tiers.push(card.tier.as_str().to_owned());
                affected.push(card.tier.as_str().to_owned());
            }
        }
    }
    let mut unacknowledged: Vec<String> = notes
        .iter()
        .filter(|c| !c.acknowledged)
        .map(|c| c.kind.as_str().to_owned())
        .collect();
    unacknowledged.sort();
    affected.sort();
    affected.dedup();
    HealthContinuity {
        local_editing_safe,
        live_tiers,
        degraded_tiers,
        outage_tiers,
        affected_boundaries: affected,
        unacknowledged_notes: unacknowledged,
        continuity_message_id: format!("{}continuity", M5_SERVICE_HEALTH_MESSAGE_ID_PREFIX),
    }
}

/// Escapes a CSV field so commas / quotes in age labels do not break the export.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Scans the export for forbidden raw material (credential bodies / raw provider payloads).
fn contains_forbidden_material(packet: &ServiceHealthCommunication) -> bool {
    let json = serde_json::to_string(packet)
        .unwrap_or_default()
        .to_ascii_lowercase();
    const FORBIDDEN: [&str; 6] = [
        "bearer_token",
        "authorization:",
        "private_key",
        "begin rsa",
        "set-cookie",
        "client_secret",
    ];
    FORBIDDEN.iter().any(|needle| json.contains(needle))
}

/// Maps each variant of an `as_str`-bearing enum to its token, in declaration order.
fn tokens<T: Copy, const N: usize>(all: &[T; N], f: impl Fn(&T) -> &'static str) -> Vec<String> {
    all.iter().map(|x| f(x).to_owned()).collect()
}
