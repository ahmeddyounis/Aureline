//! The M5 update / support-lifecycle governance matrix — one frozen baseline every claimed
//! update-center, release-note, change-impact, migration, support-window, and end-of-support
//! surface qualifies against before deeper implementation widens a channel or lifecycle promise.
//!
//! The current release sheet already covers release-center publication, the artifact graph,
//! install / update diagnostics, rollback objects, and some Help/About truth, but it leaves the
//! actual update center, change-impact forecast, release-note evidence, migration assistant,
//! support window, compatibility window, and end-of-support contract too implicit to implement
//! directly. This lane closes that gap with one governed matrix rather than parallel release prose.
//!
//! The matrix has three parts:
//!
//! - The canonical [state families](LifecycleStateFamily): one ordered, gate-bound vocabulary each
//!   for [update](UpdateState), [readiness](ReadinessState), [migration](MigrationState),
//!   [support-window](SupportWindowState), and [end-of-support](EndOfSupportState) state. Every
//!   token binds to a shared [gate posture](crate::m5_descriptor_badge::DescriptorGate) and an
//!   effective [qualification floor](crate::m5_descriptor_badge::QualificationClass) so the
//!   lifecycle states are reused across surfaces instead of restated as ad hoc labels.
//! - The governed [lifecycle facets](LifecycleFacet): the eight product surfaces the source set
//!   treats as governed truth (update availability, change impact, release-note evidence, migration
//!   assistant, service health, support window, compatibility window, end-of-support). Each facet
//!   names its [dimension](LifecycleDimension), the state family that governs it, the artifact
//!   classes it discloses, the claimed channel scope, the managed / self-hosted profiles it covers,
//!   its stale-data behavior, an owner role, and the proof path plus
//!   [freshness](crate::m5_descriptor_badge::FreshnessState) that keeps it current.
//! - The claimed [consumer surfaces](LifecycleConsumer): release center, update center, Help/About,
//!   docs/help, diagnostics, support exports, shiproom, and companion handoff. Each binds the
//!   facets it reads, and the matrix *derives* its coverage gaps, gate decision, and effective
//!   qualification from those facets' proof freshness and current lifecycle state.
//!
//! Gaps in *proof* (a stale, expired, or missing facet proof, or a facet a consumer reads that the
//! matrix does not govern) and gaps in *lifecycle coverage* (a facet whose current canonical state
//! itself narrows or blocks) both fail the matrix rather than remaining implied: a stale facet
//! deterministically narrows every consumer that reads it below Stable, and an expired / missing /
//! ungoverned facet — or a facet in a blocking lifecycle state — blocks that consumer from Stable
//! promotion, with the gap named per consumer and its drifted dimension.
//!
//! The [`M5UpdateLifecycleGovernance`] packet is the one inspectable, serde-serializable governance
//! truth release, support, docs, diagnostics, and export surfaces consume rather than maintaining
//! parallel update / support-lifecycle inventories; it carries metadata and refs only — no
//! credential bodies or raw provider payloads.
//!
//! - Packet schema:
//!   [`schemas/release/m5-update-center-summary.schema.json`](../../../../../schemas/release/m5-update-center-summary.schema.json)
//! - Contract doc:
//!   [`docs/release/m5-update-lifecycle-contract.md`](../../../../../docs/release/m5-update-lifecycle-contract.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_update_lifecycle, seeded_m5_update_lifecycle_missing_proof_blocked,
    seeded_m5_update_lifecycle_stale_proof_narrowed, M5_UPDATE_LIFECYCLE_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The matrix reuses the descriptor / badge runtime's frozen gate vocabulary so the lifecycle
// governance layer and the public-truth descriptor layer can never drift to different gate tokens.
use crate::m5_descriptor_badge::{
    ConsumerStatus, DescriptorGate, DescriptorSignal, FreshnessState, QualificationClass,
};

/// Record-kind tag carried by [`M5UpdateLifecycleGovernance`].
pub const M5_UPDATE_LIFECYCLE_RECORD_KIND: &str = "m5_update_lifecycle_governance";

/// Schema version for the governance packet.
pub const M5_UPDATE_LIFECYCLE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the governance packet schema.
pub const M5_UPDATE_LIFECYCLE_SCHEMA_REF: &str =
    "schemas/release/m5-update-center-summary.schema.json";

/// Repo-relative path of the published governance inventory.
pub const M5_UPDATE_LIFECYCLE_REF: &str = "artifacts/release/m5-update-lifecycle-summary.json";

/// Repo-relative path of the rendered governance matrix document.
pub const M5_UPDATE_LIFECYCLE_GOVERNANCE_REF: &str =
    "artifacts/release/m5-update-lifecycle-governance.md";

/// Repo-relative path of the machine-readable matrix export.
pub const M5_UPDATE_LIFECYCLE_MATRIX_CSV_REF: &str =
    "artifacts/release/m5-update-lifecycle-matrix.csv";

/// Repo-relative path of the release-grade governance parity proof.
pub const M5_UPDATE_LIFECYCLE_PROOF_REF: &str =
    "artifacts/release-proof/m5-update-lifecycle/update-lifecycle-matrix.json";

/// Repo-relative path of the governance contract doc.
pub const M5_UPDATE_LIFECYCLE_DOC_REF: &str = "docs/release/m5-update-lifecycle-contract.md";

/// Repo-relative directory of the per-state governance fixtures.
pub const M5_UPDATE_LIFECYCLE_FIXTURE_DIR: &str = "fixtures/release/m5-update-center/";

/// Prefix every update-lifecycle message id carries so consumers can route it.
pub const M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX: &str = "release_update_lifecycle.";

/// One of the three lifecycle dimensions a [facet](LifecycleFacet) belongs to. Naming the dimension
/// on every drift is what lets the matrix say *which* of change disclosure, migration continuity, or
/// support lifecycle drifted rather than collapsing the cause into one flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleDimension {
    /// What an update touches and discloses: availability, change impact, release-note evidence.
    ChangeDisclosure,
    /// What carries a user across an update: migration work and what still works locally.
    MigrationContinuity,
    /// How long a build stays supported: support window, compatibility window, end-of-support.
    SupportLifecycle,
}

impl LifecycleDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ChangeDisclosure,
        Self::MigrationContinuity,
        Self::SupportLifecycle,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChangeDisclosure => "change_disclosure",
            Self::MigrationContinuity => "migration_continuity",
            Self::SupportLifecycle => "support_lifecycle",
        }
    }

    /// Reviewer-facing dimension label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ChangeDisclosure => "Change disclosure",
            Self::MigrationContinuity => "Migration continuity",
            Self::SupportLifecycle => "Support lifecycle",
        }
    }
}

/// One governed product surface the source set treats as update / support-lifecycle truth. Each
/// facet owns one proof path; binding a consumer to a facet is what makes that consumer's claim
/// depend on the facet's proof staying current and its lifecycle state staying governed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleFacet {
    /// Whether an update is offered and on which channels.
    UpdateAvailability,
    /// The change-impact forecast: affected artifact classes, restart / rollback cost.
    ChangeImpact,
    /// The release-note / what's-new evidence rows backing a claim.
    ReleaseNoteEvidence,
    /// The migration assistant: the migration tasks an update requires.
    MigrationAssistant,
    /// The service-health banner: what still works locally under stale / outage conditions.
    ServiceHealth,
    /// The support window: how long the build is supported and on which profile.
    SupportWindow,
    /// The compatibility window: which versions interoperate.
    CompatibilityWindow,
    /// The end-of-support state and successor.
    EndOfSupport,
}

impl LifecycleFacet {
    /// Every facet, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::UpdateAvailability,
        Self::ChangeImpact,
        Self::ReleaseNoteEvidence,
        Self::MigrationAssistant,
        Self::ServiceHealth,
        Self::SupportWindow,
        Self::CompatibilityWindow,
        Self::EndOfSupport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpdateAvailability => "update_availability",
            Self::ChangeImpact => "change_impact",
            Self::ReleaseNoteEvidence => "release_note_evidence",
            Self::MigrationAssistant => "migration_assistant",
            Self::ServiceHealth => "service_health",
            Self::SupportWindow => "support_window",
            Self::CompatibilityWindow => "compatibility_window",
            Self::EndOfSupport => "end_of_support",
        }
    }

    /// Reviewer-facing facet label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UpdateAvailability => "Update availability",
            Self::ChangeImpact => "Change impact",
            Self::ReleaseNoteEvidence => "Release-note evidence",
            Self::MigrationAssistant => "Migration assistant",
            Self::ServiceHealth => "Service health",
            Self::SupportWindow => "Support window",
            Self::CompatibilityWindow => "Compatibility window",
            Self::EndOfSupport => "End of support",
        }
    }

    /// The dimension this facet belongs to.
    pub const fn dimension(self) -> LifecycleDimension {
        match self {
            Self::UpdateAvailability | Self::ChangeImpact | Self::ReleaseNoteEvidence => {
                LifecycleDimension::ChangeDisclosure
            }
            Self::MigrationAssistant | Self::ServiceHealth => {
                LifecycleDimension::MigrationContinuity
            }
            Self::SupportWindow | Self::CompatibilityWindow | Self::EndOfSupport => {
                LifecycleDimension::SupportLifecycle
            }
        }
    }

    /// The canonical state family that governs this facet.
    pub const fn state_family(self) -> LifecycleStateFamily {
        match self {
            Self::UpdateAvailability => LifecycleStateFamily::Update,
            Self::ChangeImpact | Self::ReleaseNoteEvidence | Self::ServiceHealth => {
                LifecycleStateFamily::Readiness
            }
            Self::MigrationAssistant => LifecycleStateFamily::Migration,
            Self::SupportWindow | Self::CompatibilityWindow => LifecycleStateFamily::SupportWindow,
            Self::EndOfSupport => LifecycleStateFamily::EndOfSupport,
        }
    }

    /// Repo-relative release-grade proof path that keeps this facet current.
    pub const fn proof_ref(self) -> &'static str {
        match self {
            Self::UpdateAvailability => {
                "artifacts/release-proof/m5-update-lifecycle/update-availability.json"
            }
            Self::ChangeImpact => "artifacts/release-proof/m5-update-lifecycle/change-impact.json",
            Self::ReleaseNoteEvidence => {
                "artifacts/release-proof/m5-update-lifecycle/release-note-evidence.json"
            }
            Self::MigrationAssistant => {
                "artifacts/release-proof/m5-update-lifecycle/migration-assistant.json"
            }
            Self::ServiceHealth => {
                "artifacts/release-proof/m5-update-lifecycle/service-health.json"
            }
            Self::SupportWindow => {
                "artifacts/release-proof/m5-update-lifecycle/support-window.json"
            }
            Self::CompatibilityWindow => {
                "artifacts/release-proof/m5-update-lifecycle/compatibility-window.json"
            }
            Self::EndOfSupport => "artifacts/release-proof/m5-update-lifecycle/end-of-support.json",
        }
    }

    /// Owner role accountable for keeping this facet's proof current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::UpdateAvailability | Self::ChangeImpact => "release_update_center_owner",
            Self::ReleaseNoteEvidence => "release_notes_owner",
            Self::MigrationAssistant | Self::ServiceHealth => "migration_continuity_owner",
            Self::SupportWindow | Self::CompatibilityWindow | Self::EndOfSupport => {
                "support_lifecycle_owner"
            }
        }
    }
}

/// One canonical lifecycle state family. Each family is an ordered, gate-bound vocabulary reused
/// across the governed surfaces so a lifecycle state is never restated as an ad hoc label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleStateFamily {
    /// Whether and how strongly an update is offered.
    Update,
    /// Whether applying an update is ready, costs a restart / rollback, or needs action.
    Readiness,
    /// How much migration work an update requires.
    Migration,
    /// How a support window is scoped.
    SupportWindow,
    /// Where a build sits on the end-of-support ladder.
    EndOfSupport,
}

impl LifecycleStateFamily {
    /// Every family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Update,
        Self::Readiness,
        Self::Migration,
        Self::SupportWindow,
        Self::EndOfSupport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Update => "update",
            Self::Readiness => "readiness",
            Self::Migration => "migration",
            Self::SupportWindow => "support_window",
            Self::EndOfSupport => "end_of_support",
        }
    }

    /// Reviewer-facing family label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Update => "Update state",
            Self::Readiness => "Readiness state",
            Self::Migration => "Migration state",
            Self::SupportWindow => "Support-window state",
            Self::EndOfSupport => "End-of-support state",
        }
    }

    /// The ordered canonical state tokens of this family, each bound to a gate posture and floor.
    pub fn state_defs(self) -> Vec<LifecycleStateTokenDef> {
        match self {
            Self::Update => UpdateState::ALL.iter().map(|s| s.to_def()).collect(),
            Self::Readiness => ReadinessState::ALL.iter().map(|s| s.to_def()).collect(),
            Self::Migration => MigrationState::ALL.iter().map(|s| s.to_def()).collect(),
            Self::SupportWindow => SupportWindowState::ALL.iter().map(|s| s.to_def()).collect(),
            Self::EndOfSupport => EndOfSupportState::ALL.iter().map(|s| s.to_def()).collect(),
        }
    }

    /// True when `token` is a member of this family.
    pub fn contains_token(self, token: &str) -> bool {
        self.state_defs().iter().any(|d| d.token == token)
    }
}

/// Maps a gate posture to the effective qualification floor it implies, so a state's posture and
/// floor can never disagree: governed stands at Stable, narrowed floors at Beta, blocked at
/// Unavailable.
const fn floor_for_posture(posture: DescriptorGate) -> QualificationClass {
    match posture {
        DescriptorGate::Governed => QualificationClass::Stable,
        DescriptorGate::Narrowed => QualificationClass::Beta,
        DescriptorGate::Blocked => QualificationClass::Unavailable,
    }
}

/// Builds a state token definition from a stable token, label, and gate posture.
fn state_def(token: &str, label: &str, posture: DescriptorGate) -> LifecycleStateTokenDef {
    LifecycleStateTokenDef {
        token: token.to_owned(),
        label: label.to_owned(),
        gate_posture: posture,
        effective_floor: floor_for_posture(posture),
        message_id: format!("{M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX}state.{token}"),
    }
}

/// Canonical update-availability state vocabulary (most→least permissive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateState {
    /// The build is up to date; no update offered.
    UpToDate,
    /// An optional update is offered.
    UpdateOffered,
    /// An update is recommended; the claim narrows until it is applied.
    UpdateRecommended,
    /// An update is required; the claim narrows until it is applied.
    UpdateRequired,
    /// Updates are blocked (revoked / incompatible); Stable promotion is held.
    UpdateBlocked,
}

impl UpdateState {
    /// Every update state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::UpToDate,
        Self::UpdateOffered,
        Self::UpdateRecommended,
        Self::UpdateRequired,
        Self::UpdateBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpToDate => "up_to_date",
            Self::UpdateOffered => "update_offered",
            Self::UpdateRecommended => "update_recommended",
            Self::UpdateRequired => "update_required",
            Self::UpdateBlocked => "update_blocked",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UpToDate => "Up to date",
            Self::UpdateOffered => "Update offered",
            Self::UpdateRecommended => "Update recommended",
            Self::UpdateRequired => "Update required",
            Self::UpdateBlocked => "Update blocked",
        }
    }

    /// Gate posture this state binds to.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::UpToDate | Self::UpdateOffered => DescriptorGate::Governed,
            Self::UpdateRecommended | Self::UpdateRequired => DescriptorGate::Narrowed,
            Self::UpdateBlocked => DescriptorGate::Blocked,
        }
    }

    fn to_def(self) -> LifecycleStateTokenDef {
        state_def(self.as_str(), self.label(), self.gate_posture())
    }
}

/// Canonical readiness state vocabulary (most→least ready).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessState {
    /// Ready to apply with no restart.
    ReadyNoRestart,
    /// Ready, but applying it costs a restart (disclosed, still governed).
    RestartRequired,
    /// Rollback is available if the update regresses (disclosed, still governed).
    RollbackAvailable,
    /// Action is required before the surface is ready; the claim narrows.
    ActionRequired,
    /// Not ready; Stable promotion is held.
    NotReady,
}

impl ReadinessState {
    /// Every readiness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReadyNoRestart,
        Self::RestartRequired,
        Self::RollbackAvailable,
        Self::ActionRequired,
        Self::NotReady,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyNoRestart => "ready_no_restart",
            Self::RestartRequired => "restart_required",
            Self::RollbackAvailable => "rollback_available",
            Self::ActionRequired => "action_required",
            Self::NotReady => "not_ready",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReadyNoRestart => "Ready, no restart",
            Self::RestartRequired => "Restart required",
            Self::RollbackAvailable => "Rollback available",
            Self::ActionRequired => "Action required",
            Self::NotReady => "Not ready",
        }
    }

    /// Gate posture this state binds to.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::ReadyNoRestart | Self::RestartRequired | Self::RollbackAvailable => {
                DescriptorGate::Governed
            }
            Self::ActionRequired => DescriptorGate::Narrowed,
            Self::NotReady => DescriptorGate::Blocked,
        }
    }

    fn to_def(self) -> LifecycleStateTokenDef {
        state_def(self.as_str(), self.label(), self.gate_posture())
    }
}

/// Canonical migration state vocabulary (least→most disruptive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationState {
    /// No migration is required.
    NoMigration,
    /// Migration runs automatically with no user action.
    AutomaticMigration,
    /// Migration runs with assistance; the claim narrows until it completes.
    AssistedMigration,
    /// Migration requires manual steps; the claim narrows until it completes.
    ManualMigration,
    /// Migration is blocking and cannot complete; Stable promotion is held.
    BlockingMigration,
}

impl MigrationState {
    /// Every migration state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoMigration,
        Self::AutomaticMigration,
        Self::AssistedMigration,
        Self::ManualMigration,
        Self::BlockingMigration,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoMigration => "no_migration",
            Self::AutomaticMigration => "automatic_migration",
            Self::AssistedMigration => "assisted_migration",
            Self::ManualMigration => "manual_migration",
            Self::BlockingMigration => "blocking_migration",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoMigration => "No migration",
            Self::AutomaticMigration => "Automatic migration",
            Self::AssistedMigration => "Assisted migration",
            Self::ManualMigration => "Manual migration",
            Self::BlockingMigration => "Blocking migration",
        }
    }

    /// Gate posture this state binds to.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::NoMigration | Self::AutomaticMigration => DescriptorGate::Governed,
            Self::AssistedMigration | Self::ManualMigration => DescriptorGate::Narrowed,
            Self::BlockingMigration => DescriptorGate::Blocked,
        }
    }

    fn to_def(self) -> LifecycleStateTokenDef {
        state_def(self.as_str(), self.label(), self.gate_posture())
    }
}

/// Canonical support-window state vocabulary (most→least supported).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportWindowState {
    /// Full support: fixes, security, and compatibility.
    FullSupport,
    /// Maintenance only: critical fixes; the claim narrows.
    MaintenanceSupport,
    /// Security only: security fixes; the claim narrows.
    SecuritySupport,
    /// Grace window before end of support; the claim narrows.
    GraceWindow,
    /// Out of support; Stable promotion is held.
    OutOfSupport,
}

impl SupportWindowState {
    /// Every support-window state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullSupport,
        Self::MaintenanceSupport,
        Self::SecuritySupport,
        Self::GraceWindow,
        Self::OutOfSupport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSupport => "full_support",
            Self::MaintenanceSupport => "maintenance_support",
            Self::SecuritySupport => "security_support",
            Self::GraceWindow => "grace_window",
            Self::OutOfSupport => "out_of_support",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullSupport => "Full support",
            Self::MaintenanceSupport => "Maintenance support",
            Self::SecuritySupport => "Security support",
            Self::GraceWindow => "Grace window",
            Self::OutOfSupport => "Out of support",
        }
    }

    /// Gate posture this state binds to.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::FullSupport => DescriptorGate::Governed,
            Self::MaintenanceSupport | Self::SecuritySupport | Self::GraceWindow => {
                DescriptorGate::Narrowed
            }
            Self::OutOfSupport => DescriptorGate::Blocked,
        }
    }

    fn to_def(self) -> LifecycleStateTokenDef {
        state_def(self.as_str(), self.label(), self.gate_posture())
    }
}

/// Canonical end-of-support state vocabulary (most→least supported).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndOfSupportState {
    /// Supported, no sunset announced.
    Supported,
    /// A sunset has been announced; the claim narrows.
    SunsetAnnounced,
    /// Deprecated; superseded but still documented; the claim narrows.
    Deprecated,
    /// Retired; no longer offered; Stable promotion is held.
    Retired,
    /// Removed; absent from the channel; Stable promotion is held.
    Removed,
}

impl EndOfSupportState {
    /// Every end-of-support state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Supported,
        Self::SunsetAnnounced,
        Self::Deprecated,
        Self::Retired,
        Self::Removed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::SunsetAnnounced => "sunset_announced",
            Self::Deprecated => "deprecated",
            Self::Retired => "retired",
            Self::Removed => "removed",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Supported => "Supported",
            Self::SunsetAnnounced => "Sunset announced",
            Self::Deprecated => "Deprecated",
            Self::Retired => "Retired",
            Self::Removed => "Removed",
        }
    }

    /// Gate posture this state binds to.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::Supported => DescriptorGate::Governed,
            Self::SunsetAnnounced | Self::Deprecated => DescriptorGate::Narrowed,
            Self::Retired | Self::Removed => DescriptorGate::Blocked,
        }
    }

    fn to_def(self) -> LifecycleStateTokenDef {
        state_def(self.as_str(), self.label(), self.gate_posture())
    }
}

/// A typed current lifecycle state assigned to a facet — one value drawn from the facet's state
/// family. The matrix uses it to bind the facet to a gate posture and qualification floor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalState {
    /// An update-availability state.
    Update(UpdateState),
    /// A readiness state.
    Readiness(ReadinessState),
    /// A migration state.
    Migration(MigrationState),
    /// A support-window state.
    SupportWindow(SupportWindowState),
    /// An end-of-support state.
    EndOfSupport(EndOfSupportState),
}

impl CanonicalState {
    /// The family this state belongs to.
    pub const fn family(self) -> LifecycleStateFamily {
        match self {
            Self::Update(_) => LifecycleStateFamily::Update,
            Self::Readiness(_) => LifecycleStateFamily::Readiness,
            Self::Migration(_) => LifecycleStateFamily::Migration,
            Self::SupportWindow(_) => LifecycleStateFamily::SupportWindow,
            Self::EndOfSupport(_) => LifecycleStateFamily::EndOfSupport,
        }
    }

    /// Stable token recorded in the packet.
    pub const fn token(self) -> &'static str {
        match self {
            Self::Update(s) => s.as_str(),
            Self::Readiness(s) => s.as_str(),
            Self::Migration(s) => s.as_str(),
            Self::SupportWindow(s) => s.as_str(),
            Self::EndOfSupport(s) => s.as_str(),
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Update(s) => s.label(),
            Self::Readiness(s) => s.label(),
            Self::Migration(s) => s.label(),
            Self::SupportWindow(s) => s.label(),
            Self::EndOfSupport(s) => s.label(),
        }
    }

    /// Gate posture this state binds to.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::Update(s) => s.gate_posture(),
            Self::Readiness(s) => s.gate_posture(),
            Self::Migration(s) => s.gate_posture(),
            Self::SupportWindow(s) => s.gate_posture(),
            Self::EndOfSupport(s) => s.gate_posture(),
        }
    }
}

/// One affected artifact class an update path discloses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactClass {
    /// The core application runtime / shell binary set.
    CoreRuntime,
    /// Extension / plugin packs.
    ExtensionPacks,
    /// Published schema / contract packages.
    SchemaContracts,
    /// Persisted workspace state.
    WorkspaceState,
    /// User configuration / settings.
    Configuration,
    /// Language servers / runtimes.
    LanguageRuntimes,
    /// Docs / Help content.
    DocsHelpContent,
}

impl ArtifactClass {
    /// Every artifact class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::CoreRuntime,
        Self::ExtensionPacks,
        Self::SchemaContracts,
        Self::WorkspaceState,
        Self::Configuration,
        Self::LanguageRuntimes,
        Self::DocsHelpContent,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreRuntime => "core_runtime",
            Self::ExtensionPacks => "extension_packs",
            Self::SchemaContracts => "schema_contracts",
            Self::WorkspaceState => "workspace_state",
            Self::Configuration => "configuration",
            Self::LanguageRuntimes => "language_runtimes",
            Self::DocsHelpContent => "docs_help_content",
        }
    }
}

/// One claimed M5 release channel the matrix scopes to. The set is a subset of the frozen release
/// channel vocabulary; this lane does not invent channel families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelScope {
    /// The general-availability lane.
    Stable,
    /// The publicly announced pre-release lane.
    Beta,
    /// The gated pre-release lane.
    Preview,
    /// The automated daily lane.
    Nightly,
    /// The long-term-support line.
    Lts,
}

impl ChannelScope {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Nightly,
        Self::Lts,
    ];

    /// Stable token recorded in the packet; matches the frozen release-channel vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Nightly => "nightly",
            Self::Lts => "lts",
        }
    }
}

/// One deployment profile a support window covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfile {
    /// The managed profile.
    Managed,
    /// The self-hosted profile.
    SelfHosted,
}

impl DeploymentProfile {
    /// Every profile, in declaration order.
    pub const ALL: [Self; 2] = [Self::Managed, Self::SelfHosted];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Managed => "managed",
            Self::SelfHosted => "self_hosted",
        }
    }
}

/// How a facet behaves under stale, mirrored, or no-live-data conditions. Every behavior keeps the
/// surface local-safe: it labels the weaker state rather than dropping it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaleDataBehavior {
    /// The surface is showing live, verified data.
    LiveVerified,
    /// The surface is showing mirrored data, labelled as mirrored.
    MirroredLabelled,
    /// The surface is showing offline-cached data, labelled as offline.
    OfflineCached,
    /// The surface is showing data behind a stale banner.
    StaleBannerShown,
    /// No live data is reachable; the surface shows only what still works locally.
    LocalOnlyNoLiveData,
}

impl StaleDataBehavior {
    /// Every behavior, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LiveVerified,
        Self::MirroredLabelled,
        Self::OfflineCached,
        Self::StaleBannerShown,
        Self::LocalOnlyNoLiveData,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveVerified => "live_verified",
            Self::MirroredLabelled => "mirrored_labelled",
            Self::OfflineCached => "offline_cached",
            Self::StaleBannerShown => "stale_banner_shown",
            Self::LocalOnlyNoLiveData => "local_only_no_live_data",
        }
    }
}

/// The kind of coverage gap on a consumer's read facet: a proof-currency gap or a lifecycle-state
/// gap. Naming the kind is what lets the matrix fail proof *or* lifecycle coverage rather than
/// leaving it implied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleGapKind {
    /// The consumer reads a facet the matrix does not govern.
    FacetUngoverned,
    /// A read facet's proof is stale (narrows).
    ProofStale,
    /// A read facet's proof is expired (blocks).
    ProofExpired,
    /// A read facet's proof is missing (blocks).
    ProofMissing,
    /// A read facet's current lifecycle state itself narrows the claim.
    LifecycleStateNarrowed,
    /// A read facet's current lifecycle state itself blocks the claim.
    LifecycleStateBlocked,
}

impl LifecycleGapKind {
    /// Every gap kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FacetUngoverned,
        Self::ProofStale,
        Self::ProofExpired,
        Self::ProofMissing,
        Self::LifecycleStateNarrowed,
        Self::LifecycleStateBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetUngoverned => "facet_ungoverned",
            Self::ProofStale => "proof_stale",
            Self::ProofExpired => "proof_expired",
            Self::ProofMissing => "proof_missing",
            Self::LifecycleStateNarrowed => "lifecycle_state_narrowed",
            Self::LifecycleStateBlocked => "lifecycle_state_blocked",
        }
    }

    /// True when this gap blocks Stable promotion (vs only narrowing it).
    pub const fn is_blocking(self) -> bool {
        matches!(
            self,
            Self::FacetUngoverned
                | Self::ProofExpired
                | Self::ProofMissing
                | Self::LifecycleStateBlocked
        )
    }
}

/// One canonical lifecycle state token definition, bound to a gate posture and a qualification
/// floor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleStateTokenDef {
    /// Stable token.
    pub token: String,
    /// Reviewer-facing label.
    pub label: String,
    /// Gate posture this state binds to.
    pub gate_posture: DescriptorGate,
    /// Effective qualification floor implied by the posture.
    pub effective_floor: QualificationClass,
    /// Stable message id; prefixed [`M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX`].
    pub message_id: String,
}

/// One canonical lifecycle state family with its ordered, gate-bound token set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleStateFamilyDef {
    /// The family.
    pub family: LifecycleStateFamily,
    /// Reviewer-facing family label.
    pub family_label: String,
    /// The ordered state tokens.
    pub states: Vec<LifecycleStateTokenDef>,
}

impl LifecycleStateFamilyDef {
    /// Builds the family definition from the typed family.
    pub fn for_family(family: LifecycleStateFamily) -> Self {
        Self {
            family,
            family_label: family.label().to_owned(),
            states: family.state_defs(),
        }
    }

    /// Validates the family's internal invariants: every token binds a posture-consistent floor and
    /// carries a prefixed message id.
    fn validate(&self) -> Vec<M5UpdateLifecycleViolation> {
        let mut out = Vec::new();
        if self.family_label != self.family.label() || self.states != self.family.state_defs() {
            out.push(M5UpdateLifecycleViolation::StateFamilyDrift);
        }
        for state in &self.states {
            if state.effective_floor != floor_for_posture(state.gate_posture)
                || !state
                    .message_id
                    .starts_with(M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX)
            {
                out.push(M5UpdateLifecycleViolation::StateBindingInvalid);
            }
        }
        out
    }
}

/// The canonical state-family definitions, in family order.
pub fn canonical_state_families() -> Vec<LifecycleStateFamilyDef> {
    LifecycleStateFamily::ALL
        .iter()
        .map(|f| LifecycleStateFamilyDef::for_family(*f))
        .collect()
}

/// One governed lifecycle facet row: its dimension, the state family that governs it, its current
/// canonical state, the artifact classes / channels / profiles it discloses, its stale-data
/// behavior, the proof path and freshness backing it, and the status that proof implies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleFacetRow {
    /// The governed facet.
    pub facet: LifecycleFacet,
    /// Reviewer-facing facet label.
    pub facet_label: String,
    /// The dimension this facet belongs to.
    pub dimension: LifecycleDimension,
    /// The state family that governs this facet.
    pub state_family: LifecycleStateFamily,
    /// The facet's current canonical state token (a member of [`Self::state_family`]).
    pub current_state_token: String,
    /// Reviewer-facing current-state label.
    pub current_state_label: String,
    /// Gate posture the current state binds to.
    pub state_gate: DescriptorGate,
    /// Effective qualification floor implied by the current state.
    pub state_floor: QualificationClass,
    /// The artifact classes this facet discloses.
    pub artifact_classes: Vec<ArtifactClass>,
    /// The claimed channels this facet scopes to.
    pub channel_scope: Vec<ChannelScope>,
    /// The deployment profiles this facet covers.
    pub profiles: Vec<DeploymentProfile>,
    /// How this facet behaves under stale / mirrored / no-live-data conditions.
    pub stale_data_behavior: StaleDataBehavior,
    /// Owner role accountable for keeping the proof current.
    pub owner_role: String,
    /// Repo-relative release-grade proof path.
    pub proof_ref: String,
    /// Freshness of the facet's proof.
    pub proof_freshness: FreshnessState,
    /// Coverage status implied by the proof freshness.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl LifecycleFacetRow {
    /// Builds a facet row at a given proof freshness and current state, deriving every field from
    /// the facet so a row can never cite a field that drifts from it.
    pub fn new(
        facet: LifecycleFacet,
        state: CanonicalState,
        proof_freshness: FreshnessState,
        artifact_classes: &[ArtifactClass],
        channel_scope: &[ChannelScope],
        profiles: &[DeploymentProfile],
        stale_data_behavior: StaleDataBehavior,
    ) -> Self {
        let status = proof_status(proof_freshness);
        Self {
            facet,
            facet_label: facet.label().to_owned(),
            dimension: facet.dimension(),
            state_family: facet.state_family(),
            current_state_token: state.token().to_owned(),
            current_state_label: state.label().to_owned(),
            state_gate: state.gate_posture(),
            state_floor: floor_for_posture(state.gate_posture()),
            artifact_classes: artifact_classes.to_vec(),
            channel_scope: channel_scope.to_vec(),
            profiles: profiles.to_vec(),
            stale_data_behavior,
            owner_role: facet.owner_role().to_owned(),
            proof_ref: facet.proof_ref().to_owned(),
            proof_freshness,
            status,
            signal: status.signal(),
            detail_message_id: format!(
                "{}facet.{}",
                M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX,
                facet.as_str()
            ),
        }
    }

    /// The proof-currency gap kind a consumer that reads this facet inherits, if any.
    fn proof_gap_kind(&self) -> Option<LifecycleGapKind> {
        match self.proof_freshness {
            FreshnessState::Current => None,
            FreshnessState::Stale => Some(LifecycleGapKind::ProofStale),
            FreshnessState::Expired => Some(LifecycleGapKind::ProofExpired),
            FreshnessState::Missing => Some(LifecycleGapKind::ProofMissing),
        }
    }

    /// The lifecycle-state gap kind a consumer that reads this facet inherits, if any.
    fn state_gap_kind(&self) -> Option<LifecycleGapKind> {
        match self.state_gate {
            DescriptorGate::Governed => None,
            DescriptorGate::Narrowed => Some(LifecycleGapKind::LifecycleStateNarrowed),
            DescriptorGate::Blocked => Some(LifecycleGapKind::LifecycleStateBlocked),
        }
    }

    /// Validates the row's invariants: every derived field matches the facet, the current state is
    /// a member of its family, the status mirrors the proof freshness, and the message id carries
    /// the lane prefix.
    fn validate(&self) -> Vec<M5UpdateLifecycleViolation> {
        let mut out = Vec::new();
        if self.facet_label != self.facet.label()
            || self.dimension != self.facet.dimension()
            || self.state_family != self.facet.state_family()
            || self.owner_role != self.facet.owner_role()
            || self.proof_ref != self.facet.proof_ref()
        {
            out.push(M5UpdateLifecycleViolation::FacetFieldMismatch);
        }
        if !self.state_family.contains_token(&self.current_state_token)
            || self.state_floor != floor_for_posture(self.state_gate)
        {
            out.push(M5UpdateLifecycleViolation::FacetStateInvalid);
        }
        if self.artifact_classes.is_empty()
            || self.channel_scope.is_empty()
            || self.profiles.is_empty()
        {
            out.push(M5UpdateLifecycleViolation::FacetDisclosureEmpty);
        }
        let status = proof_status(self.proof_freshness);
        if self.status != status || self.signal != status.signal() {
            out.push(M5UpdateLifecycleViolation::FacetStatusDrift);
        }
        if !self
            .detail_message_id
            .starts_with(M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX)
        {
            out.push(M5UpdateLifecycleViolation::UnprefixedMessageId);
        }
        out
    }
}

/// Maps a proof freshness to the coverage status it implies: current is mapped, stale is
/// provisional (narrowed), expired / missing is unmapped (blocked).
fn proof_status(freshness: FreshnessState) -> ConsumerStatus {
    match freshness {
        FreshnessState::Current => ConsumerStatus::Mapped,
        FreshnessState::Stale => ConsumerStatus::Provisional,
        FreshnessState::Expired | FreshnessState::Missing => ConsumerStatus::Unmapped,
    }
}

/// One coverage gap on a claimed consumer: a facet it reads whose proof drifted, whose lifecycle
/// state narrows or blocks, or that the matrix does not govern at all.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleGap {
    /// Consumer this gap applies to.
    pub consumer: LifecycleConsumer,
    /// The facet the gap concerns.
    pub facet: LifecycleFacet,
    /// The dimension that drifted.
    pub dimension: LifecycleDimension,
    /// The kind of gap.
    pub gap_kind: LifecycleGapKind,
    /// Stable message id; prefixed [`M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX`].
    pub cause_message_id: String,
}

/// One claimed M5 update / support-lifecycle consumer surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleConsumer {
    /// The release center / public-truth automation.
    ReleaseCenter,
    /// The in-product update center.
    UpdateCenter,
    /// The Help/About panel.
    HelpAbout,
    /// The docs / help reference surface.
    DocsHelp,
    /// Install / update diagnostics.
    Diagnostics,
    /// Support exports / bundles.
    SupportExport,
    /// Shiproom / go-no-go packets.
    Shiproom,
    /// Companion handoff surfaces.
    CompanionHandoff,
}

impl LifecycleConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ReleaseCenter,
        Self::UpdateCenter,
        Self::HelpAbout,
        Self::DocsHelp,
        Self::Diagnostics,
        Self::SupportExport,
        Self::Shiproom,
        Self::CompanionHandoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::UpdateCenter => "update_center",
            Self::HelpAbout => "help_about",
            Self::DocsHelp => "docs_help",
            Self::Diagnostics => "diagnostics",
            Self::SupportExport => "support_export",
            Self::Shiproom => "shiproom",
            Self::CompanionHandoff => "companion_handoff",
        }
    }

    /// Reviewer-facing consumer label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "Release center",
            Self::UpdateCenter => "Update center",
            Self::HelpAbout => "Help / About",
            Self::DocsHelp => "Docs / Help",
            Self::Diagnostics => "Install / update diagnostics",
            Self::SupportExport => "Support export",
            Self::Shiproom => "Shiproom",
            Self::CompanionHandoff => "Companion handoff",
        }
    }

    /// Owner role accountable for keeping this consumer's binding current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center_owner",
            Self::UpdateCenter => "update_center_owner",
            Self::HelpAbout => "help_about_owner",
            Self::DocsHelp => "docs_help_owner",
            Self::Diagnostics => "diagnostics_owner",
            Self::SupportExport => "support_export_owner",
            Self::Shiproom => "shiproom_owner",
            Self::CompanionHandoff => "companion_owner",
        }
    }
}

/// Derived verdict for a consumer, computed from its gaps.
struct ConsumerVerdict {
    status: ConsumerStatus,
    signal: DescriptorSignal,
    gate: DescriptorGate,
    effective_qualification: QualificationClass,
}

/// Restrictiveness rank of a qualification class (least restrictive first).
fn qualification_rank(class: QualificationClass) -> usize {
    QualificationClass::ALL
        .iter()
        .position(|c| *c == class)
        .unwrap_or(QualificationClass::ALL.len())
}

/// The more restrictive of two qualification classes.
fn more_restrictive(a: QualificationClass, b: QualificationClass) -> QualificationClass {
    if qualification_rank(a) >= qualification_rank(b) {
        a
    } else {
        b
    }
}

/// Derives a consumer's verdict from its gaps: any blocking gap blocks Stable; any narrowing gap
/// narrows to at least Beta; an ungapped consumer stands at its claim.
fn derive_consumer_verdict(claimed: QualificationClass, gaps: &[LifecycleGap]) -> ConsumerVerdict {
    let any_blocking = gaps.iter().any(|g| g.gap_kind.is_blocking());
    let any_narrowing = gaps.iter().any(|g| !g.gap_kind.is_blocking());

    let status = if any_blocking {
        ConsumerStatus::Unmapped
    } else if any_narrowing {
        ConsumerStatus::Provisional
    } else {
        ConsumerStatus::Mapped
    };

    let gate = if any_blocking {
        DescriptorGate::Blocked
    } else if any_narrowing {
        DescriptorGate::Narrowed
    } else {
        DescriptorGate::Governed
    };

    let effective_qualification = match gate {
        DescriptorGate::Governed => claimed,
        DescriptorGate::Blocked => QualificationClass::Unavailable,
        DescriptorGate::Narrowed => more_restrictive(claimed, QualificationClass::Beta),
    };

    ConsumerVerdict {
        status,
        signal: status.signal(),
        gate,
        effective_qualification,
    }
}

/// One claimed consumer surface certified against the governed facets: the facets it reads, the
/// union of artifact classes / channels / profiles those facets disclose, the proof paths backing
/// them, the per-consumer gaps, and the verdict derived from those facets' proof freshness and
/// lifecycle state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleConsumerRow {
    /// The consumer surface.
    pub consumer: LifecycleConsumer,
    /// Reviewer-facing consumer label.
    pub consumer_label: String,
    /// Owner role accountable for keeping this consumer's binding current.
    pub owner_role: String,
    /// Public qualification the consumer wants to keep.
    pub claimed_qualification: QualificationClass,
    /// The facets this consumer reads.
    pub read_facets: Vec<LifecycleFacet>,
    /// The dimensions this consumer's read facets cover, in dimension order.
    pub covered_dimensions: Vec<LifecycleDimension>,
    /// The union of artifact classes the read facets disclose, in class order.
    pub disclosed_artifact_classes: Vec<ArtifactClass>,
    /// The union of claimed channels the read facets scope to, in channel order.
    pub channel_scope: Vec<ChannelScope>,
    /// The union of deployment profiles the read facets cover, in profile order.
    pub profiles: Vec<DeploymentProfile>,
    /// The proof paths backing the read facets — refs only.
    pub proof_refs: Vec<String>,
    /// Effective qualification after the gate applies.
    pub effective_qualification: QualificationClass,
    /// Coverage status.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Gate decision the release automation reads.
    pub gate_decision: DescriptorGate,
    /// Exact coverage gaps for this consumer.
    pub gaps: Vec<LifecycleGap>,
    /// Stable message id for the status; prefixed [`M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX`].
    pub status_message_id: String,
    /// Stable message id for the gate; prefixed [`M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

impl LifecycleConsumerRow {
    /// Builds a consumer row from its claimed qualification and the facets it reads; the resolved
    /// unions, gaps, and verdict are recomputed against the packet's facet rows when the packet is
    /// assembled.
    pub fn new(
        consumer: LifecycleConsumer,
        claimed_qualification: QualificationClass,
        read_facets: &[LifecycleFacet],
    ) -> Self {
        Self {
            consumer,
            consumer_label: consumer.label().to_owned(),
            owner_role: consumer.owner_role().to_owned(),
            claimed_qualification,
            read_facets: read_facets.to_vec(),
            covered_dimensions: Vec::new(),
            disclosed_artifact_classes: Vec::new(),
            channel_scope: Vec::new(),
            profiles: Vec::new(),
            proof_refs: Vec::new(),
            effective_qualification: claimed_qualification,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            gate_decision: DescriptorGate::Governed,
            gaps: Vec::new(),
            status_message_id: format!(
                "{}consumer.{}.status",
                M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
            gate_message_id: format!(
                "{}consumer.{}.gate",
                M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
        }
    }

    /// Recomputes the resolved unions, gaps, and verdict from the packet's facet rows, so a
    /// consumer's claim is always generated from the same checked-in proofs and lifecycle states
    /// the packet ships rather than a hand-maintained status.
    pub fn recompute(&mut self, facets: &[LifecycleFacetRow]) {
        let mut read = self.read_facets.clone();
        read.sort_by_key(facet_rank);
        read.dedup();
        self.read_facets = read.clone();

        let row_for = |facet: LifecycleFacet| -> Option<&LifecycleFacetRow> {
            facets.iter().find(|r| r.facet == facet)
        };

        // Union the disclosures across the read facets, canonically ordered and deduped.
        let mut dimensions: Vec<LifecycleDimension> = Vec::new();
        let mut artifact_classes: Vec<ArtifactClass> = Vec::new();
        let mut channels: Vec<ChannelScope> = Vec::new();
        let mut profiles: Vec<DeploymentProfile> = Vec::new();
        let mut proof_refs: Vec<String> = Vec::new();
        for &facet in &read {
            dimensions.push(facet.dimension());
            proof_refs.push(
                row_for(facet)
                    .map(|r| r.proof_ref.clone())
                    .unwrap_or_else(|| facet.proof_ref().to_owned()),
            );
            if let Some(row) = row_for(facet) {
                artifact_classes.extend(row.artifact_classes.iter().copied());
                channels.extend(row.channel_scope.iter().copied());
                profiles.extend(row.profiles.iter().copied());
            }
        }
        dimensions.sort_by_key(|d| dimension_rank(*d));
        dimensions.dedup();
        artifact_classes.sort_by_key(|c| artifact_rank(*c));
        artifact_classes.dedup();
        channels.sort_by_key(|c| channel_rank(*c));
        channels.dedup();
        profiles.sort_by_key(|p| profile_rank(*p));
        profiles.dedup();
        self.covered_dimensions = dimensions;
        self.disclosed_artifact_classes = artifact_classes;
        self.channel_scope = channels;
        self.profiles = profiles;
        self.proof_refs = proof_refs;

        // Derive the coverage gaps from each read facet's proof currency and lifecycle state.
        let consumer = self.consumer;
        let mut gaps = Vec::new();
        let mut push_gap = |facet: LifecycleFacet, kind: LifecycleGapKind| {
            gaps.push(LifecycleGap {
                consumer,
                facet,
                dimension: facet.dimension(),
                gap_kind: kind,
                cause_message_id: format!(
                    "{}consumer.{}.{}.{}.gap",
                    M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX,
                    consumer.as_str(),
                    facet.as_str(),
                    kind.as_str()
                ),
            });
        };
        for &facet in &read {
            match row_for(facet) {
                None => push_gap(facet, LifecycleGapKind::FacetUngoverned),
                Some(row) => {
                    if let Some(kind) = row.proof_gap_kind() {
                        push_gap(facet, kind);
                    }
                    if let Some(kind) = row.state_gap_kind() {
                        push_gap(facet, kind);
                    }
                }
            }
        }
        gaps.sort_by(|a, b| {
            a.facet
                .as_str()
                .cmp(b.facet.as_str())
                .then(a.gap_kind.as_str().cmp(b.gap_kind.as_str()))
        });
        self.gaps = gaps;

        let verdict = derive_consumer_verdict(self.claimed_qualification, &self.gaps);
        self.status = verdict.status;
        self.signal = verdict.signal;
        self.gate_decision = verdict.gate;
        self.effective_qualification = verdict.effective_qualification;
    }

    /// True when the consumer is blocked from Stable promotion.
    pub fn is_blocked(&self) -> bool {
        self.gate_decision.blocks()
    }

    /// True when the consumer auto-narrowed below its claim.
    pub fn is_narrowed(&self) -> bool {
        matches!(self.gate_decision, DescriptorGate::Narrowed)
    }

    /// True when the consumer is fully certified at its claim.
    pub fn is_certified(&self) -> bool {
        matches!(self.gate_decision, DescriptorGate::Governed)
    }

    /// Validates the consumer's static invariants.
    fn validate_static(&self) -> Vec<M5UpdateLifecycleViolation> {
        let mut out = Vec::new();
        if self.consumer_label != self.consumer.label()
            || self.owner_role != self.consumer.owner_role()
        {
            out.push(M5UpdateLifecycleViolation::MissingIdentity);
        }
        if self.read_facets.is_empty() {
            out.push(M5UpdateLifecycleViolation::ConsumerReadsNoFacets);
        }
        if !self
            .status_message_id
            .starts_with(M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX)
            || !self
                .gate_message_id
                .starts_with(M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX)
        {
            out.push(M5UpdateLifecycleViolation::UnprefixedMessageId);
        }
        for gap in &self.gaps {
            if !gap
                .cause_message_id
                .starts_with(M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX)
                || gap.consumer != self.consumer
                || gap.dimension != gap.facet.dimension()
            {
                out.push(M5UpdateLifecycleViolation::CoverageGapInvalid);
            }
        }
        out
    }
}

/// Position of a facet in the canonical ordering.
fn facet_rank(facet: &LifecycleFacet) -> usize {
    LifecycleFacet::ALL
        .iter()
        .position(|f| f == facet)
        .unwrap_or(LifecycleFacet::ALL.len())
}

/// Position of a dimension in the canonical ordering.
fn dimension_rank(dimension: LifecycleDimension) -> usize {
    LifecycleDimension::ALL
        .iter()
        .position(|d| *d == dimension)
        .unwrap_or(LifecycleDimension::ALL.len())
}

/// Position of an artifact class in the canonical ordering.
fn artifact_rank(class: ArtifactClass) -> usize {
    ArtifactClass::ALL
        .iter()
        .position(|c| *c == class)
        .unwrap_or(ArtifactClass::ALL.len())
}

/// Position of a channel in the canonical ordering.
fn channel_rank(channel: ChannelScope) -> usize {
    ChannelScope::ALL
        .iter()
        .position(|c| *c == channel)
        .unwrap_or(ChannelScope::ALL.len())
}

/// Position of a profile in the canonical ordering.
fn profile_rank(profile: DeploymentProfile) -> usize {
    DeploymentProfile::ALL
        .iter()
        .position(|p| *p == profile)
        .unwrap_or(DeploymentProfile::ALL.len())
}

/// Which surfaces consume the one governance matrix. Every flag must hold so no surface keeps a
/// parallel update / support-lifecycle inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleDisclosure {
    /// The release center consumes the matrix.
    pub release_center_consumes_matrix: bool,
    /// The update center consumes the matrix.
    pub update_center_consumes_matrix: bool,
    /// The Help/About panel consumes the matrix.
    pub help_about_consumes_matrix: bool,
    /// The docs / help surface consumes the matrix.
    pub docs_help_consumes_matrix: bool,
    /// Install / update diagnostics consume the matrix.
    pub diagnostics_consume_matrix: bool,
    /// Support exports consume the matrix.
    pub support_export_consumes_matrix: bool,
    /// Shiproom packets consume the matrix.
    pub shiproom_consumes_matrix: bool,
    /// Companion handoffs consume the matrix.
    pub companion_handoff_consumes_matrix: bool,
}

impl LifecycleDisclosure {
    /// The canonical disclosure: every surface consumes the matrix.
    pub const fn all_surfaces() -> Self {
        Self {
            release_center_consumes_matrix: true,
            update_center_consumes_matrix: true,
            help_about_consumes_matrix: true,
            docs_help_consumes_matrix: true,
            diagnostics_consume_matrix: true,
            support_export_consumes_matrix: true,
            shiproom_consumes_matrix: true,
            companion_handoff_consumes_matrix: true,
        }
    }

    /// True when every surface consumes the matrix.
    pub const fn all_consume(&self) -> bool {
        self.release_center_consumes_matrix
            && self.update_center_consumes_matrix
            && self.help_about_consumes_matrix
            && self.docs_help_consumes_matrix
            && self.diagnostics_consume_matrix
            && self.support_export_consumes_matrix
            && self.shiproom_consumes_matrix
            && self.companion_handoff_consumes_matrix
    }
}

/// Self-describing controlled-vocabulary set so the packet resolves every token it carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleVocabulary {
    /// Dimension tokens.
    pub dimensions: Vec<String>,
    /// Facet tokens.
    pub facets: Vec<String>,
    /// State-family tokens.
    pub state_families: Vec<String>,
    /// Artifact-class tokens.
    pub artifact_classes: Vec<String>,
    /// Channel tokens.
    pub channels: Vec<String>,
    /// Deployment-profile tokens.
    pub profiles: Vec<String>,
    /// Stale-data-behavior tokens.
    pub stale_data_behaviors: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
    /// Gap-kind tokens.
    pub gap_kinds: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Qualification-class tokens.
    pub qualification_classes: Vec<String>,
}

impl LifecycleVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dimensions: tokens(&LifecycleDimension::ALL, |d| d.as_str()),
            facets: tokens(&LifecycleFacet::ALL, |f| f.as_str()),
            state_families: tokens(&LifecycleStateFamily::ALL, |f| f.as_str()),
            artifact_classes: tokens(&ArtifactClass::ALL, |c| c.as_str()),
            channels: tokens(&ChannelScope::ALL, |c| c.as_str()),
            profiles: tokens(&DeploymentProfile::ALL, |p| p.as_str()),
            stale_data_behaviors: tokens(&StaleDataBehavior::ALL, |b| b.as_str()),
            consumers: tokens(&LifecycleConsumer::ALL, |c| c.as_str()),
            gap_kinds: tokens(&LifecycleGapKind::ALL, |k| k.as_str()),
            gate_decisions: tokens(&DescriptorGate::ALL, |g| g.as_str()),
            freshness_states: tokens(&FreshnessState::ALL, |f| f.as_str()),
            qualification_classes: tokens(&QualificationClass::ALL, |q| q.as_str()),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Maps a typed `ALL` array to its stable tokens.
fn tokens<T: Copy>(all: &[T], f: impl Fn(T) -> &'static str) -> Vec<String> {
    all.iter().map(|t| f(*t).to_owned()).collect()
}

/// Compact governance summary — the scoreboard release / support / docs surfaces read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleSummary {
    /// Total governed facets.
    pub total_facets: u32,
    /// Facets whose proof is current.
    pub current_facets: u32,
    /// Facets whose proof is stale.
    pub stale_facets: u32,
    /// Facets whose proof is expired.
    pub expired_facets: u32,
    /// Facets whose proof is missing.
    pub missing_facets: u32,
    /// Total canonical state families.
    pub total_state_families: u32,
    /// Total claimed consumers.
    pub total_consumers: u32,
    /// Consumers certified at their full claim.
    pub certified_consumer_count: u32,
    /// Consumers that auto-narrowed below their claim.
    pub narrowed_consumer_count: u32,
    /// Consumers blocked from Stable promotion.
    pub blocked_consumer_count: u32,
    /// True when at least one consumer is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
}

/// Packet-level release gate aggregating the per-consumer gates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleReleaseGate {
    /// True when at least one consumer is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
    /// Sorted consumer tokens blocked from Stable promotion.
    pub blocked_consumers: Vec<String>,
    /// Sorted consumer tokens that auto-narrowed below their claim.
    pub narrowed_consumers: Vec<String>,
    /// Sorted consumer tokens fully certified for Stable promotion.
    pub certified_consumers: Vec<String>,
    /// Sorted dimension tokens whose proof or lifecycle state drifted.
    pub drifted_dimensions: Vec<String>,
    /// Stable message id; prefixed [`M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

/// Conformance review. Every flag is a hard invariant; all must hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleConformance {
    /// Every facet is governed exactly once with a proof path.
    pub every_facet_governed_with_proof: bool,
    /// Every dimension is covered by at least one governed facet.
    pub every_dimension_covered: bool,
    /// Every state family is referenced by at least one governed facet.
    pub every_state_family_referenced: bool,
    /// Every claimed consumer maps to facets, disclosures, and proof paths.
    pub every_consumer_maps_to_facets_and_proof: bool,
    /// Every claimed consumer reads at least one governed facet.
    pub every_consumer_reads_at_least_one_facet: bool,
    /// A stale facet proof narrows the consumers that read it deterministically.
    pub stale_proof_narrows_deterministically: bool,
    /// An expired / missing / ungoverned facet proof blocks the consumers that read it.
    pub missing_proof_blocks_stable_promotion: bool,
    /// Exact coverage gaps are named per consumer with their drifted dimension.
    pub exact_gaps_named_per_consumer: bool,
    /// Every canonical lifecycle state binds to a gate posture and a consistent floor.
    pub state_vocabulary_bound_to_gate: bool,
    /// Release, update-center, support, docs, diagnostics, and export surfaces consume one matrix.
    pub surfaces_consume_one_matrix: bool,
    /// The matrix is generated from the same checked-in proofs and lifecycle states.
    pub generated_from_checked_in_proofs: bool,
    /// The controlled vocabularies match the canonical frozen tokens.
    pub controlled_enums_frozen: bool,
    /// The export carries no raw provider material.
    pub export_carries_no_raw_material: bool,
}

impl LifecycleConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.every_facet_governed_with_proof
            && self.every_dimension_covered
            && self.every_state_family_referenced
            && self.every_consumer_maps_to_facets_and_proof
            && self.every_consumer_reads_at_least_one_facet
            && self.stale_proof_narrows_deterministically
            && self.missing_proof_blocks_stable_promotion
            && self.exact_gaps_named_per_consumer
            && self.state_vocabulary_bound_to_gate
            && self.surfaces_consume_one_matrix
            && self.generated_from_checked_in_proofs
            && self.controlled_enums_frozen
            && self.export_carries_no_raw_material
    }
}

/// Constructor input for [`M5UpdateLifecycleGovernance::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5UpdateLifecycleInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The governed facet rows.
    pub facets: Vec<LifecycleFacetRow>,
    /// The claimed consumer rows (unions / gaps / verdict are recomputed from the facets).
    pub consumers: Vec<LifecycleConsumerRow>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable governance truth packet release, support, docs,
/// diagnostics, and export surfaces consume: the canonical state families, the governed facets, the
/// per-consumer matrix, the controlled vocabulary, a conformance review, a summary, and the
/// aggregate release gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5UpdateLifecycleGovernance {
    /// Record kind; must equal [`M5_UPDATE_LIFECYCLE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_UPDATE_LIFECYCLE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The canonical lifecycle state families.
    pub state_families: Vec<LifecycleStateFamilyDef>,
    /// The governed facet rows.
    pub facets: Vec<LifecycleFacetRow>,
    /// The claimed consumer rows with their derived verdicts.
    pub consumers: Vec<LifecycleConsumerRow>,
    /// The consumer tokens that read this matrix.
    pub consumer_tokens: Vec<String>,
    /// Which surfaces consume the matrix.
    pub disclosure: LifecycleDisclosure,
    /// Compact governance summary.
    pub summary: LifecycleSummary,
    /// Packet-level release gate.
    pub release_gate: LifecycleReleaseGate,
    /// Controlled-vocabulary set.
    pub vocabulary: LifecycleVocabulary,
    /// Conformance review block.
    pub conformance: LifecycleConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5UpdateLifecycleGovernance {
    /// Builds a governance packet from seed input, recomputing each consumer's verdict and deriving
    /// the state families, summary, release gate, and conformance review from the facets.
    pub fn new(input: M5UpdateLifecycleInput) -> Self {
        let facets = input.facets;
        let state_families = canonical_state_families();
        let mut consumers = input.consumers;
        for consumer in &mut consumers {
            consumer.recompute(&facets);
        }
        let consumer_tokens = tokens(&LifecycleConsumer::ALL, |c| c.as_str());
        let summary = derive_summary(&facets, &state_families, &consumers);
        let release_gate = derive_release_gate(&facets, &consumers);
        let conformance = derive_conformance(&facets, &state_families, &consumers);
        Self {
            record_kind: M5_UPDATE_LIFECYCLE_RECORD_KIND.to_owned(),
            schema_version: M5_UPDATE_LIFECYCLE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            state_families,
            facets,
            consumers,
            consumer_tokens,
            disclosure: LifecycleDisclosure::all_surfaces(),
            summary,
            release_gate,
            vocabulary: LifecycleVocabulary::canonical(),
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release automation must hold Stable promotion.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.release_gate.blocks_stable_promotion
    }

    /// Finds a governed facet by facet.
    pub fn facet(&self, facet: LifecycleFacet) -> Option<&LifecycleFacetRow> {
        self.facets.iter().find(|r| r.facet == facet)
    }

    /// Finds a consumer row by consumer.
    pub fn consumer(&self, consumer: LifecycleConsumer) -> Option<&LifecycleConsumerRow> {
        self.consumers.iter().find(|c| c.consumer == consumer)
    }

    /// Renders the packet for a generation channel. The output is identical for every channel —
    /// the channel parameter exists only to prove desktop, CLI/headless, and offline / mirror
    /// generation produce byte-identical output.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn render_for_channel(&self, _channel: LifecycleChannel) -> String {
        self.export_safe_json()
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 update lifecycle serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per (consumer, read facet) join, naming
    /// the consumer, its owner, the facet, the facet owner, the proof path, and any gap.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer,consumer_owner,claimed_qualification,effective_qualification,gate_decision,facet,dimension,state_family,current_state,facet_owner,proof_ref,proof_freshness,facet_status,artifact_classes,channel_scope,profiles,stale_data_behavior,gap_kind\n",
        );
        for c in &self.consumers {
            for &facet in &c.read_facets {
                let row = self.facet(facet);
                let gap_kind = c
                    .gaps
                    .iter()
                    .find(|g| g.facet == facet)
                    .map(|g| g.gap_kind.as_str())
                    .unwrap_or("");
                let (
                    current_state,
                    facet_owner,
                    proof_ref,
                    proof_freshness,
                    facet_status,
                    artifact_classes,
                    channels,
                    profiles,
                    stale,
                ) = match row {
                    Some(r) => (
                        r.current_state_token.clone(),
                        r.owner_role.clone(),
                        r.proof_ref.clone(),
                        r.proof_freshness.as_str().to_owned(),
                        r.status.as_str().to_owned(),
                        join_tokens(&r.artifact_classes, |x| x.as_str()),
                        join_tokens(&r.channel_scope, |x| x.as_str()),
                        join_tokens(&r.profiles, |x| x.as_str()),
                        r.stale_data_behavior.as_str().to_owned(),
                    ),
                    None => (
                        String::new(),
                        String::new(),
                        facet.proof_ref().to_owned(),
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                    ),
                };
                out.push_str(&format!(
                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                    c.consumer.as_str(),
                    c.owner_role,
                    c.claimed_qualification.as_str(),
                    c.effective_qualification.as_str(),
                    c.gate_decision.as_str(),
                    facet.as_str(),
                    facet.dimension().as_str(),
                    facet.state_family().as_str(),
                    current_state,
                    facet_owner,
                    proof_ref,
                    proof_freshness,
                    facet_status,
                    artifact_classes,
                    channels,
                    profiles,
                    stale,
                    gap_kind,
                ));
            }
        }
        out
    }

    /// Deterministic governance matrix document for review, support, docs, or shiproom handoff.
    pub fn render_governance_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Update / Support-Lifecycle Governance Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Facets: {} ({} current, {} stale, {} expired, {} missing)\n",
            self.summary.total_facets,
            self.summary.current_facets,
            self.summary.stale_facets,
            self.summary.expired_facets,
            self.summary.missing_facets
        ));
        out.push_str(&format!(
            "- Consumers: {} ({} certified, {} narrowed, {} blocked)\n",
            self.summary.total_consumers,
            self.summary.certified_consumer_count,
            self.summary.narrowed_consumer_count,
            self.summary.blocked_consumer_count
        ));
        out.push_str(&format!(
            "- Release gate: {}\n",
            if self.summary.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));
        out.push_str(
            "- Consumed by: release center, update center, Help/About, docs/help, diagnostics, support, shiproom, companion\n",
        );

        out.push_str("\n## Canonical lifecycle state families\n\n");
        out.push_str("| Family | State | Gate posture | Effective floor |\n");
        out.push_str("|--------|-------|--------------|-----------------|\n");
        for family in &self.state_families {
            for state in &family.states {
                out.push_str(&format!(
                    "| `{}` | `{}` | `{}` | `{}` |\n",
                    family.family.as_str(),
                    state.token,
                    state.gate_posture.as_str(),
                    state.effective_floor.as_str()
                ));
            }
        }

        out.push_str("\n## Governed facets\n\n");
        out.push_str(
            "| Facet | Dimension | State family | Current state | Channels | Profiles | Stale-data | Owner | Proof | Freshness | Status |\n",
        );
        out.push_str(
            "|-------|-----------|--------------|---------------|----------|----------|-----------|-------|-------|-----------|--------|\n",
        );
        for f in &self.facets {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | {} | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                f.facet.as_str(),
                f.dimension.as_str(),
                f.state_family.as_str(),
                f.current_state_token,
                join_tokens(&f.channel_scope, |x| x.as_str()),
                join_tokens(&f.profiles, |x| x.as_str()),
                f.stale_data_behavior.as_str(),
                f.owner_role,
                f.proof_ref,
                f.proof_freshness.as_str(),
                f.status.as_str()
            ));
        }

        out.push_str("\n## Claimed consumers\n\n");
        out.push_str(
            "| Consumer | Owner | Status | Claim → effective | Gate | Reads | Artifact classes |\n",
        );
        out.push_str(
            "|----------|-------|--------|-------------------|------|-------|------------------|\n",
        );
        for c in &self.consumers {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` → `{}` | `{}` | {} | {} |\n",
                c.consumer.as_str(),
                c.owner_role,
                c.status.as_str(),
                c.claimed_qualification.as_str(),
                c.effective_qualification.as_str(),
                c.gate_decision.as_str(),
                join_tokens(&c.read_facets, |x| x.as_str()),
                join_tokens(&c.disclosed_artifact_classes, |x| x.as_str())
            ));
            for gap in &c.gaps {
                out.push_str(&format!(
                    "| | | gap: `{}` on `{}` (`{}`) | | | | |\n",
                    gap.gap_kind.as_str(),
                    gap.facet.as_str(),
                    gap.dimension.as_str()
                ));
            }
        }
        out
    }

    /// Compact Markdown report for the release-grade parity proof.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Update / Support-Lifecycle Governance — Proof\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Facets: {} ({} current, {} stale, {} expired, {} missing)\n",
            self.summary.total_facets,
            self.summary.current_facets,
            self.summary.stale_facets,
            self.summary.expired_facets,
            self.summary.missing_facets
        ));
        out.push_str(&format!(
            "- Consumers: {} ({} certified, {} narrowed, {} blocked)\n",
            self.summary.total_consumers,
            self.summary.certified_consumer_count,
            self.summary.narrowed_consumer_count,
            self.summary.blocked_consumer_count
        ));
        out.push_str(&format!(
            "- Release gate: {}\n",
            if self.summary.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));
        if !self.release_gate.drifted_dimensions.is_empty() {
            out.push_str(&format!(
                "- Drifted dimensions: {}\n",
                self.release_gate
                    .drifted_dimensions
                    .iter()
                    .map(|d| format!("`{d}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out.push_str(&format!(
            "- Matrix CSV: `{}`\n",
            M5_UPDATE_LIFECYCLE_MATRIX_CSV_REF
        ));
        out
    }

    /// Validates the packet's invariants.
    pub fn validate(&self) -> Vec<M5UpdateLifecycleViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_UPDATE_LIFECYCLE_RECORD_KIND {
            out.push(M5UpdateLifecycleViolation::WrongRecordKind);
        }
        if self.schema_version != M5_UPDATE_LIFECYCLE_SCHEMA_VERSION {
            out.push(M5UpdateLifecycleViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5UpdateLifecycleViolation::MissingIdentity);
        }

        // Canonical state families.
        if self.state_families != canonical_state_families() {
            out.push(M5UpdateLifecycleViolation::StateFamilyDrift);
        }
        for family in &self.state_families {
            out.extend(family.validate());
        }

        // Every facet governed exactly once and self-consistent.
        let mut seen_facets = std::collections::BTreeSet::new();
        for facet in &self.facets {
            if !seen_facets.insert(facet.facet) {
                out.push(M5UpdateLifecycleViolation::DuplicateFacet);
            }
            out.extend(facet.validate());
        }
        for facet in LifecycleFacet::ALL {
            if !self.facets.iter().any(|r| r.facet == facet) {
                out.push(M5UpdateLifecycleViolation::FacetNotGoverned);
            }
        }

        if self.consumers.is_empty() {
            out.push(M5UpdateLifecycleViolation::PacketHasNoConsumers);
        }
        let mut seen_consumers = std::collections::BTreeSet::new();
        for consumer in &self.consumers {
            if !seen_consumers.insert(consumer.consumer) {
                out.push(M5UpdateLifecycleViolation::DuplicateConsumer);
            }
            out.extend(consumer.validate_static());
            // The stored verdict must match a fresh recompute from the facets.
            let mut probe = consumer.clone();
            probe.recompute(&self.facets);
            if probe != *consumer {
                out.push(M5UpdateLifecycleViolation::ConsumerVerdictDrift);
            }
        }

        let expected_tokens = tokens(&LifecycleConsumer::ALL, |c| c.as_str());
        if self.consumer_tokens != expected_tokens {
            out.push(M5UpdateLifecycleViolation::ConsumerSetMismatch);
        }
        if !self.disclosure.all_consume() {
            out.push(M5UpdateLifecycleViolation::DisclosureIncomplete);
        }
        if self.summary != derive_summary(&self.facets, &self.state_families, &self.consumers) {
            out.push(M5UpdateLifecycleViolation::SummaryDrift);
        }
        if self.release_gate != derive_release_gate(&self.facets, &self.consumers) {
            out.push(M5UpdateLifecycleViolation::ReleaseGateAggregateMismatch);
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5UpdateLifecycleViolation::VocabularyMismatch);
        }
        if self.conformance
            != derive_conformance(&self.facets, &self.state_families, &self.consumers)
            || !self.conformance.all_hold()
        {
            out.push(M5UpdateLifecycleViolation::ConformanceReviewFailed);
        }
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 update lifecycle serializes"),
        ) {
            out.push(M5UpdateLifecycleViolation::RawMaterialInExport);
        }
        out
    }
}

/// The generation channel a governance packet is produced on. Every channel produces byte-identical
/// output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleChannel {
    /// The desktop product UI.
    DesktopUi,
    /// The CLI / headless structured-output path.
    CliHeadless,
    /// Offline / mirror-safe packet generation.
    OfflineMirror,
}

impl LifecycleChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 3] = [Self::DesktopUi, Self::CliHeadless, Self::OfflineMirror];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopUi => "desktop_ui",
            Self::CliHeadless => "cli_headless",
            Self::OfflineMirror => "offline_mirror",
        }
    }
}

/// Joins a token list for table / CSV rendering, comma-space separated.
fn join_tokens<T: Copy>(items: &[T], f: impl Fn(T) -> &'static str) -> String {
    items
        .iter()
        .map(|t| f(*t))
        .collect::<Vec<_>>()
        .join(if items.len() > 8 { "," } else { " " })
}

/// Derives the governance summary from the facets, state families, and consumers.
fn derive_summary(
    facets: &[LifecycleFacetRow],
    state_families: &[LifecycleStateFamilyDef],
    consumers: &[LifecycleConsumerRow],
) -> LifecycleSummary {
    let facet_count = |state: FreshnessState| -> u32 {
        facets.iter().filter(|f| f.proof_freshness == state).count() as u32
    };
    let blocked = consumers.iter().filter(|c| c.is_blocked()).count() as u32;
    LifecycleSummary {
        total_facets: facets.len() as u32,
        current_facets: facet_count(FreshnessState::Current),
        stale_facets: facet_count(FreshnessState::Stale),
        expired_facets: facet_count(FreshnessState::Expired),
        missing_facets: facet_count(FreshnessState::Missing),
        total_state_families: state_families.len() as u32,
        total_consumers: consumers.len() as u32,
        certified_consumer_count: consumers.iter().filter(|c| c.is_certified()).count() as u32,
        narrowed_consumer_count: consumers.iter().filter(|c| c.is_narrowed()).count() as u32,
        blocked_consumer_count: blocked,
        blocks_stable_promotion: blocked > 0,
    }
}

/// Derives the aggregate release gate from the per-consumer gates and the drifted facets.
fn derive_release_gate(
    facets: &[LifecycleFacetRow],
    consumers: &[LifecycleConsumerRow],
) -> LifecycleReleaseGate {
    let pick = |f: &dyn Fn(&LifecycleConsumerRow) -> bool| -> Vec<String> {
        let mut t: Vec<String> = consumers
            .iter()
            .filter(|c| f(c))
            .map(|c| c.consumer.as_str().to_owned())
            .collect();
        t.sort();
        t
    };
    let mut drifted_dimensions: Vec<String> = facets
        .iter()
        .filter(|f| {
            !matches!(f.proof_freshness, FreshnessState::Current)
                || !matches!(f.state_gate, DescriptorGate::Governed)
        })
        .map(|f| f.dimension.as_str().to_owned())
        .collect();
    drifted_dimensions.sort();
    drifted_dimensions.dedup();
    let blocked = pick(&|c| c.is_blocked());
    LifecycleReleaseGate {
        blocks_stable_promotion: !blocked.is_empty(),
        blocked_consumers: blocked,
        narrowed_consumers: pick(&|c| c.is_narrowed()),
        certified_consumers: pick(&|c| c.is_certified()),
        drifted_dimensions,
        gate_message_id: format!("{M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX}release_gate"),
    }
}

/// Derives the conformance review, so the stored block reflects the actual packet.
fn derive_conformance(
    facets: &[LifecycleFacetRow],
    state_families: &[LifecycleStateFamilyDef],
    consumers: &[LifecycleConsumerRow],
) -> LifecycleConformance {
    let every_facet = LifecycleFacet::ALL.iter().all(|f| {
        facets
            .iter()
            .filter(|r| r.facet == *f)
            .filter(|r| !r.proof_ref.trim().is_empty())
            .count()
            == 1
    });

    let every_dimension = LifecycleDimension::ALL
        .iter()
        .all(|d| facets.iter().any(|r| r.dimension == *d));

    let every_family = LifecycleStateFamily::ALL
        .iter()
        .all(|fam| facets.iter().any(|r| r.state_family == *fam));

    let maps_to_proof = !consumers.is_empty()
        && consumers.iter().all(|c| {
            !c.read_facets.is_empty()
                && c.proof_refs.len() == c.read_facets.len()
                && !c.disclosed_artifact_classes.is_empty()
                && !c.channel_scope.is_empty()
                && !c.profiles.is_empty()
        });

    let every_reads_facet = consumers.iter().all(|c| !c.read_facets.is_empty());

    let posture_of = |facet: LifecycleFacet| -> Option<(FreshnessState, DescriptorGate)> {
        facets
            .iter()
            .find(|r| r.facet == facet)
            .map(|r| (r.proof_freshness, r.state_gate))
    };

    // A facet that only narrows (stale proof or narrowing state) narrows every consumer that reads
    // it, unless a blocking facet already blocks that consumer.
    let stale_narrows = consumers.iter().all(|c| {
        let reads_narrowing = c.read_facets.iter().any(|f| {
            matches!(
                posture_of(*f),
                Some((FreshnessState::Stale, _)) | Some((_, DescriptorGate::Narrowed))
            )
        });
        let reads_blocking = c.read_facets.iter().any(|f| {
            matches!(
                posture_of(*f),
                Some((FreshnessState::Expired, _))
                    | Some((FreshnessState::Missing, _))
                    | Some((_, DescriptorGate::Blocked))
                    | None
            )
        });
        !reads_narrowing || reads_blocking || c.is_narrowed()
    });

    // A blocking facet (expired / missing / ungoverned proof or blocking state) blocks every
    // consumer that reads it.
    let missing_blocks = consumers.iter().all(|c| {
        let reads_blocking = c.read_facets.iter().any(|f| {
            matches!(
                posture_of(*f),
                Some((FreshnessState::Expired, _))
                    | Some((FreshnessState::Missing, _))
                    | Some((_, DescriptorGate::Blocked))
                    | None
            )
        });
        !reads_blocking || c.is_blocked()
    });

    let gaps_named = consumers.iter().all(|c| {
        c.gaps.iter().all(|g| {
            g.cause_message_id
                .starts_with(M5_UPDATE_LIFECYCLE_MESSAGE_ID_PREFIX)
                && g.consumer == c.consumer
                && g.dimension == g.facet.dimension()
        })
    });

    let state_bound = state_families == canonical_state_families()
        && state_families.iter().all(|fam| {
            fam.states
                .iter()
                .all(|s| s.effective_floor == floor_for_posture(s.gate_posture))
        });

    let generated = consumers.iter().all(|c| {
        let mut probe = c.clone();
        probe.recompute(facets);
        probe == *c
    });

    let export_clean =
        !json_contains_forbidden_material(&serde_json::to_value(facets).expect("facets serialize"))
            && !json_contains_forbidden_material(
                &serde_json::to_value(consumers).expect("consumers serialize"),
            );

    LifecycleConformance {
        every_facet_governed_with_proof: every_facet,
        every_dimension_covered: every_dimension,
        every_state_family_referenced: every_family,
        every_consumer_maps_to_facets_and_proof: maps_to_proof,
        every_consumer_reads_at_least_one_facet: every_reads_facet,
        stale_proof_narrows_deterministically: stale_narrows,
        missing_proof_blocks_stable_promotion: missing_blocks,
        exact_gaps_named_per_consumer: gaps_named,
        state_vocabulary_bound_to_gate: state_bound,
        surfaces_consume_one_matrix: true,
        generated_from_checked_in_proofs: generated,
        controlled_enums_frozen: LifecycleVocabulary::canonical().matches_canonical(),
        export_carries_no_raw_material: export_clean,
    }
}

/// Validation failures for the update-lifecycle governance lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5UpdateLifecycleViolation {
    /// The packet record kind is wrong.
    WrongRecordKind,
    /// The packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// The canonical state families drifted.
    StateFamilyDrift,
    /// A state token binds an inconsistent floor or unprefixed message id.
    StateBindingInvalid,
    /// A facet cites a field that does not match its facet.
    FacetFieldMismatch,
    /// A facet's current state is not a member of its family, or its floor is inconsistent.
    FacetStateInvalid,
    /// A facet discloses no artifact classes, channels, or profiles.
    FacetDisclosureEmpty,
    /// A facet's status drifted from its proof freshness.
    FacetStatusDrift,
    /// Two facets name the same facet.
    DuplicateFacet,
    /// A facet has no governed entry.
    FacetNotGoverned,
    /// The packet declares no claimed consumers.
    PacketHasNoConsumers,
    /// Two consumers share a consumer token.
    DuplicateConsumer,
    /// A claimed consumer reads no facets.
    ConsumerReadsNoFacets,
    /// A consumer's stored verdict drifted from a fresh recompute.
    ConsumerVerdictDrift,
    /// A coverage gap is malformed (wrong consumer, dimension, or unprefixed message id).
    CoverageGapInvalid,
    /// The consumer-token set does not match the canonical consumers.
    ConsumerSetMismatch,
    /// A disclosure surface does not consume the matrix.
    DisclosureIncomplete,
    /// The summary disagrees with the facets / consumers.
    SummaryDrift,
    /// The aggregate release gate disagrees with the per-consumer gates.
    ReleaseGateAggregateMismatch,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// The export contains raw provider material.
    RawMaterialInExport,
}

impl M5UpdateLifecycleViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::StateFamilyDrift => "state_family_drift",
            Self::StateBindingInvalid => "state_binding_invalid",
            Self::FacetFieldMismatch => "facet_field_mismatch",
            Self::FacetStateInvalid => "facet_state_invalid",
            Self::FacetDisclosureEmpty => "facet_disclosure_empty",
            Self::FacetStatusDrift => "facet_status_drift",
            Self::DuplicateFacet => "duplicate_facet",
            Self::FacetNotGoverned => "facet_not_governed",
            Self::PacketHasNoConsumers => "packet_has_no_consumers",
            Self::DuplicateConsumer => "duplicate_consumer",
            Self::ConsumerReadsNoFacets => "consumer_reads_no_facets",
            Self::ConsumerVerdictDrift => "consumer_verdict_drift",
            Self::CoverageGapInvalid => "coverage_gap_invalid",
            Self::ConsumerSetMismatch => "consumer_set_mismatch",
            Self::DisclosureIncomplete => "disclosure_incomplete",
            Self::SummaryDrift => "summary_drift",
            Self::ReleaseGateAggregateMismatch => "release_gate_aggregate_mismatch",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked raw material. Mirrors the redaction posture of
/// the upstream descriptor lanes.
const FORBIDDEN_KEY_SUBSTRINGS: [&str; 6] = [
    "credential",
    "secret",
    "password",
    "api_key",
    "raw_payload",
    "bearer_token",
];

/// Scans a serialized value for forbidden material. Returns true when a key (case-insensitive)
/// contains a forbidden substring.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => map.iter().any(|(key, child)| {
            let lower = key.to_ascii_lowercase();
            FORBIDDEN_KEY_SUBSTRINGS
                .iter()
                .any(|needle| lower.contains(needle))
                || json_contains_forbidden_material(child)
        }),
        serde_json::Value::Array(items) => items.iter().any(json_contains_forbidden_material),
        _ => false,
    }
}
