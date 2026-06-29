//! Typed support-lifecycle cards — the per-channel and per-subject truth a user or admin reads to
//! decide whether to upgrade, pin, postpone, file a bug, or roll a channel out broadly, without
//! digging through release prose or website copy.
//!
//! Where the [update-center summary](crate::m5_update_summary) answers "what is changing" and the
//! [change-impact cards](crate::m5_change_impact_card) answer "what will the change do before
//! restart", this lane answers the exit-gate question: *what support, compatibility, deprecation, and
//! end-of-support promises does each channel actually carry, and how do I stay safe across them*.
//! Stable, beta, preview, nightly, and LTS no longer share one vague readiness label once Aureline
//! starts making support-window and deprecation promises.
//!
//! The packet carries two card families, both gate-bound to the shared
//! [descriptor/badge](crate::m5_descriptor_badge) vocabulary so docs, help, update, and export
//! surfaces read one set of states rather than re-deriving them:
//!
//! - one [channel card](ChannelSupportCard) per [channel](crate::m5_update_lifecycle::ChannelScope),
//!   carrying channel identity, the [support window](SupportWindowDates) and its
//!   [state](crate::m5_update_lifecycle::SupportWindowState), the [overlap window](OverlapWindow) with
//!   the prior version, the [deprecation horizon](DeprecationHorizon) and removal target, the
//!   [pin-or-postpone path](PinPostponeGuidance), and the known [compatibility caveats](CompatibilityCaveat); and
//! - one [compatibility-subject card](CompatibilitySubjectCard) per
//!   [subject](CompatibilitySubject) — workspace/profile files, extension SDKs, extension manifests,
//!   remote helpers, and public schemas — carrying its [end-of-support](crate::m5_update_lifecycle::EndOfSupportState)
//!   posture and [compatibility window](CompatibilityWindow).
//!
//! A card's gate is the *worse* of its support-window and end-of-support (or compatibility-window)
//! postures, so a card can never advertise a wider support commitment than the weakest promise it
//! carries — the lane's guardrail against broadening support, enforced by
//! [`SupportWindowCardSet::validate`]. A card under lifecycle pressure (deprecated, sunset, out of
//! support, or outside its compatibility window) must carry replacement, overlap, and recovery
//! guidance rather than a bare warning; a card missing that guidance fails validation.
//!
//! The [consumer surfaces](SupportConsumer) — Help/About, docs/help, the update center, the
//! compatibility report, support export, the admin console, and the release center — each read the
//! cards and *derive* their [readiness](SupportReadiness) and [gaps](SupportGap) from them, so all of
//! them present the same support-window data rather than cloning state locally.
//!
//! The packet is inspectable and serde-serializable; it carries metadata, refs, and message ids only
//! — no credential bodies or raw provider payloads — so the support-lifecycle truth is exportable and
//! reviewable outside the app and stays honest under stale, mirrored, or no-live-data conditions.
//!
//! - Packet schema:
//!   [`schemas/release/m5-support-window-card.schema.json`](../../../../../schemas/release/m5-support-window-card.schema.json)
//! - Contract doc:
//!   [`docs/release/m5-support-window-card-contract.md`](../../../../../docs/release/m5-support-window-card-contract.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_support_window_card_set, seeded_m5_support_window_card_set_deprecation,
    seeded_m5_support_window_card_set_end_of_support,
    seeded_m5_support_window_card_set_subject_compat, M5_SUPPORT_WINDOW_CARD_SET_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The support-lifecycle cards reuse the update / lifecycle governance vocabularies for channel,
// artifact class, deployment profile, support-window state, end-of-support state, and stale-data
// behavior, and the descriptor / badge runtime's gate / status / signal vocabulary, so this card
// layer can never drift to a different vocabulary than the layers above.
use crate::m5_descriptor_badge::{ConsumerStatus, DescriptorGate, DescriptorSignal};
use crate::m5_update_lifecycle::{
    ArtifactClass, ChannelScope, DeploymentProfile, EndOfSupportState, StaleDataBehavior,
    SupportWindowState,
};

/// Record-kind tag carried by [`SupportWindowCardSet`].
pub const M5_SUPPORT_WINDOW_CARD_SET_RECORD_KIND: &str = "m5_support_window_card_set";

/// Schema version for the support-window card-set packet.
pub const M5_SUPPORT_WINDOW_CARD_SET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the card-set packet schema.
pub const M5_SUPPORT_WINDOW_CARD_SCHEMA_REF: &str =
    "schemas/release/m5-support-window-card.schema.json";

/// Repo-relative path of the published card-set inventory.
pub const M5_SUPPORT_WINDOW_CARD_SET_REF: &str = "artifacts/release/m5-support-window-cards.json";

/// Repo-relative path of the release-grade channel-lifecycle parity proof.
pub const M5_SUPPORT_WINDOW_CARD_SET_PROOF_REF: &str =
    "artifacts/release/m5-channel-lifecycle-proof/support-window-cards.json";

/// Repo-relative path of the machine-readable per-card export.
pub const M5_SUPPORT_WINDOW_CARD_SET_CSV_REF: &str =
    "artifacts/release/m5-support-window-cards.csv";

/// Repo-relative path of the card-set contract doc.
pub const M5_SUPPORT_WINDOW_CARD_SET_DOC_REF: &str =
    "docs/release/m5-support-window-card-contract.md";

/// Repo-relative directory of the per-state card-set fixtures.
pub const M5_SUPPORT_WINDOW_CARD_SET_FIXTURE_DIR: &str = "fixtures/release/support-window-and-eos/";

/// Prefix every support-window message id carries so consumers can route it.
pub const M5_SUPPORT_WINDOW_MESSAGE_ID_PREFIX: &str = "release_support_window.";

const REDACTION_CLASS: &str = "metadata_safe_default";

// ---------------------------------------------------------------------------
// Controlled vocabularies
// ---------------------------------------------------------------------------

/// The compatibility-window / end-of-support subject a card discloses posture for. The set is the
/// claimed boundary surfaces whose lifecycle a user must reason about beyond the channel itself; this
/// lane does not invent new subjects to exercise the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilitySubject {
    /// Persisted workspace and profile files.
    WorkspaceProfileFiles,
    /// The extension SDK surface extensions build against.
    ExtensionSdk,
    /// The extension manifest format.
    ExtensionManifest,
    /// The remote helper binaries.
    RemoteHelper,
    /// The published public schemas / contracts.
    PublicSchema,
}

impl CompatibilitySubject {
    /// Every subject, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::WorkspaceProfileFiles,
        Self::ExtensionSdk,
        Self::ExtensionManifest,
        Self::RemoteHelper,
        Self::PublicSchema,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceProfileFiles => "workspace_profile_files",
            Self::ExtensionSdk => "extension_sdk",
            Self::ExtensionManifest => "extension_manifest",
            Self::RemoteHelper => "remote_helper",
            Self::PublicSchema => "public_schema",
        }
    }

    /// Human-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceProfileFiles => "Workspace & profile files",
            Self::ExtensionSdk => "Extension SDK",
            Self::ExtensionManifest => "Extension manifest",
            Self::RemoteHelper => "Remote helper",
            Self::PublicSchema => "Public schema",
        }
    }

    /// The primary artifact class this subject's compatibility window governs.
    pub const fn primary_artifact_class(self) -> ArtifactClass {
        match self {
            Self::WorkspaceProfileFiles => ArtifactClass::WorkspaceState,
            Self::ExtensionSdk | Self::ExtensionManifest => ArtifactClass::ExtensionPacks,
            Self::RemoteHelper => ArtifactClass::CoreRuntime,
            Self::PublicSchema => ArtifactClass::SchemaContracts,
        }
    }

    /// Accountable owner role for this subject's lifecycle.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::WorkspaceProfileFiles => "workspace_state_owner",
            Self::ExtensionSdk => "extension_sdk_owner",
            Self::ExtensionManifest => "extension_manifest_owner",
            Self::RemoteHelper => "remote_helper_owner",
            Self::PublicSchema => "schema_owner",
        }
    }
}

/// Where a subject's installed version sits inside its declared compatibility window. Declaration
/// order is best→worst; each posture binds to a gate so the window posture is read identically to the
/// support-window and end-of-support states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityWindowPosture {
    /// Inside the supported floor→ceiling window.
    WithinWindow,
    /// Inside the window but nearing the ceiling; the claim narrows.
    NearingCeiling,
    /// Outside the window (below floor or above ceiling); Stable support is held.
    OutsideWindow,
}

impl CompatibilityWindowPosture {
    /// Every posture, best→worst.
    pub const ALL: [Self; 3] = [
        Self::WithinWindow,
        Self::NearingCeiling,
        Self::OutsideWindow,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinWindow => "within_window",
            Self::NearingCeiling => "nearing_ceiling",
            Self::OutsideWindow => "outside_window",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WithinWindow => "Within window",
            Self::NearingCeiling => "Nearing ceiling",
            Self::OutsideWindow => "Outside window",
        }
    }

    /// Gate posture this window posture binds to.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::WithinWindow => DescriptorGate::Governed,
            Self::NearingCeiling => DescriptorGate::Narrowed,
            Self::OutsideWindow => DescriptorGate::Blocked,
        }
    }
}

/// The pin-or-postpone path a card discloses, so a user always sees a way to defer or recover rather
/// than only a warning. The kinds are distinct so a card never implies a true channel move when only a
/// pin, a postpone, or a side-by-side overlap remains, and so an out-of-support card states plainly
/// that an upgrade is the only path left.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PinPostponeChoice {
    /// No pin-or-postpone path applies (nothing to defer).
    NotApplicable,
    /// Stay on the channel; it is fully supported.
    StayOnChannel,
    /// Pin the current version to defer the change.
    PinCurrentVersion,
    /// Postpone the upgrade within the overlap window.
    PostponeUpgrade,
    /// Move to the successor channel.
    MoveToSuccessorChannel,
    /// Run the prior and current versions side-by-side during the overlap window.
    SideBySideDuringOverlap,
    /// No deferral remains; an upgrade is required.
    UpgradeRequired,
}

impl PinPostponeChoice {
    /// Every choice, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::NotApplicable,
        Self::StayOnChannel,
        Self::PinCurrentVersion,
        Self::PostponeUpgrade,
        Self::MoveToSuccessorChannel,
        Self::SideBySideDuringOverlap,
        Self::UpgradeRequired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::StayOnChannel => "stay_on_channel",
            Self::PinCurrentVersion => "pin_current_version",
            Self::PostponeUpgrade => "postpone_upgrade",
            Self::MoveToSuccessorChannel => "move_to_successor_channel",
            Self::SideBySideDuringOverlap => "side_by_side_during_overlap",
            Self::UpgradeRequired => "upgrade_required",
        }
    }

    /// True when the choice names a real recovery / deferral / upgrade path a card under lifecycle
    /// pressure must offer (everything except "not applicable" and the no-op "stay on channel").
    pub const fn is_active_path(self) -> bool {
        matches!(
            self,
            Self::PinCurrentVersion
                | Self::PostponeUpgrade
                | Self::MoveToSuccessorChannel
                | Self::SideBySideDuringOverlap
                | Self::UpgradeRequired
        )
    }
}

/// The support readiness a card or consumer resolves to. A direct, one-to-one reading of a
/// [`DescriptorGate`] in support-lifecycle language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportReadiness {
    /// Fully supported; safe to stay or roll out broadly.
    Supported,
    /// Under lifecycle pressure (maintenance / security / grace / deprecated / nearing ceiling); plan
    /// a migration.
    PlanMigration,
    /// Out of support or removed; action is required before relying on it.
    ActionRequired,
}

impl SupportReadiness {
    /// Every readiness, in declaration order.
    pub const ALL: [Self; 3] = [Self::Supported, Self::PlanMigration, Self::ActionRequired];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::PlanMigration => "plan_migration",
            Self::ActionRequired => "action_required",
        }
    }

    /// The readiness a gate resolves to.
    pub const fn from_gate(gate: DescriptorGate) -> Self {
        match gate {
            DescriptorGate::Governed => Self::Supported,
            DescriptorGate::Narrowed => Self::PlanMigration,
            DescriptorGate::Blocked => Self::ActionRequired,
        }
    }
}

/// The named cause of a consumer's gap on one card it reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportGapKind {
    /// A read card is under lifecycle pressure; plan a migration.
    MigrationRecommended,
    /// A read card is out of support / removed / outside its window; action required.
    ActionRequiredBeforeUpgrade,
    /// A channel the consumer reads is not carded in the packet.
    ChannelNotPublished,
    /// A subject the consumer reads is not carded in the packet.
    SubjectNotPublished,
}

impl SupportGapKind {
    /// Every gap kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::MigrationRecommended,
        Self::ActionRequiredBeforeUpgrade,
        Self::ChannelNotPublished,
        Self::SubjectNotPublished,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MigrationRecommended => "migration_recommended",
            Self::ActionRequiredBeforeUpgrade => "action_required_before_upgrade",
            Self::ChannelNotPublished => "channel_not_published",
            Self::SubjectNotPublished => "subject_not_published",
        }
    }

    /// The gate this gap forces.
    const fn gate(self) -> DescriptorGate {
        match self {
            Self::MigrationRecommended => DescriptorGate::Narrowed,
            Self::ActionRequiredBeforeUpgrade
            | Self::ChannelNotPublished
            | Self::SubjectNotPublished => DescriptorGate::Blocked,
        }
    }
}

/// Whether a gap points at a channel card or a subject card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportTargetKind {
    /// The gap points at a channel card.
    Channel,
    /// The gap points at a compatibility-subject card.
    Subject,
}

impl SupportTargetKind {
    /// Every target kind, in declaration order.
    pub const ALL: [Self; 2] = [Self::Channel, Self::Subject];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Channel => "channel",
            Self::Subject => "subject",
        }
    }
}

/// One claimed consumer surface that reads the support-lifecycle cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportConsumer {
    /// The in-product Help / About surface.
    HelpAbout,
    /// The docs / help content.
    DocsHelp,
    /// The in-product update center.
    UpdateCenter,
    /// The compatibility report surface.
    CompatibilityReport,
    /// The support export.
    SupportExport,
    /// The admin console.
    AdminConsole,
    /// The release center / public-truth automation.
    ReleaseCenter,
}

impl SupportConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::HelpAbout,
        Self::DocsHelp,
        Self::UpdateCenter,
        Self::CompatibilityReport,
        Self::SupportExport,
        Self::AdminConsole,
        Self::ReleaseCenter,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about",
            Self::DocsHelp => "docs_help",
            Self::UpdateCenter => "update_center",
            Self::CompatibilityReport => "compatibility_report",
            Self::SupportExport => "support_export",
            Self::AdminConsole => "admin_console",
            Self::ReleaseCenter => "release_center",
        }
    }

    /// Human-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::HelpAbout => "Help & About",
            Self::DocsHelp => "Docs / Help",
            Self::UpdateCenter => "Update center",
            Self::CompatibilityReport => "Compatibility report",
            Self::SupportExport => "Support export",
            Self::AdminConsole => "Admin console",
            Self::ReleaseCenter => "Release center",
        }
    }

    /// Accountable owner role.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about_owner",
            Self::DocsHelp => "docs_help_owner",
            Self::UpdateCenter => "update_center_owner",
            Self::CompatibilityReport => "compatibility_report_owner",
            Self::SupportExport => "support_export_owner",
            Self::AdminConsole => "admin_console_owner",
            Self::ReleaseCenter => "release_center_owner",
        }
    }
}

// ---------------------------------------------------------------------------
// Channel identity helpers (ChannelScope is owned by m5_update_lifecycle)
// ---------------------------------------------------------------------------

/// Human-facing label for a channel.
const fn channel_label(channel: ChannelScope) -> &'static str {
    match channel {
        ChannelScope::Stable => "Stable",
        ChannelScope::Beta => "Beta",
        ChannelScope::Preview => "Preview",
        ChannelScope::Nightly => "Nightly",
        ChannelScope::Lts => "Long-term support",
    }
}

/// One-line channel identity blurb, so the card states what the channel *is* rather than only its
/// support state.
const fn channel_description(channel: ChannelScope) -> &'static str {
    match channel {
        ChannelScope::Stable => "General-availability line with the longest support commitment.",
        ChannelScope::Beta => "Publicly announced pre-release line ahead of Stable.",
        ChannelScope::Preview => "Gated pre-release line for early evaluation.",
        ChannelScope::Nightly => "Automated daily line; best-effort, not a support commitment.",
        ChannelScope::Lts => "Long-term-support line with an extended maintenance window.",
    }
}

/// Accountable owner role for a channel's lifecycle.
const fn channel_owner_role(channel: ChannelScope) -> &'static str {
    match channel {
        ChannelScope::Stable => "stable_line_owner",
        ChannelScope::Beta => "beta_line_owner",
        ChannelScope::Preview => "preview_line_owner",
        ChannelScope::Nightly => "nightly_line_owner",
        ChannelScope::Lts => "lts_line_owner",
    }
}

// ---------------------------------------------------------------------------
// Ranking helpers for deterministic ordering
// ---------------------------------------------------------------------------

fn channel_rank(c: ChannelScope) -> usize {
    ChannelScope::ALL
        .iter()
        .position(|x| *x == c)
        .unwrap_or(usize::MAX)
}

fn subject_rank(s: CompatibilitySubject) -> usize {
    CompatibilitySubject::ALL
        .iter()
        .position(|x| *x == s)
        .unwrap_or(usize::MAX)
}

fn artifact_rank(c: ArtifactClass) -> usize {
    ArtifactClass::ALL
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

fn consumer_rank(c: SupportConsumer) -> usize {
    SupportConsumer::ALL
        .iter()
        .position(|x| *x == c)
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

fn sort_caveats(caveats: &mut [CompatibilityCaveat]) {
    caveats.sort_by(|a, b| {
        artifact_rank(a.affected_artifact_class)
            .cmp(&artifact_rank(b.affected_artifact_class))
            .then(a.caveat_message_id.cmp(&b.caveat_message_id))
    });
}

fn sort_profiles(profiles: &mut Vec<DeploymentProfile>) {
    profiles.sort_by_key(|p| profile_rank(*p));
    profiles.dedup();
}

// ---------------------------------------------------------------------------
// Card sub-objects
// ---------------------------------------------------------------------------

/// The support-window dates a channel card discloses. Dates are opaque strings (or absent), so the
/// packet never depends on a clock and stays exportable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportWindowDates {
    /// The date through which the channel carries full support (absent when not committed).
    pub full_support_until: Option<String>,
    /// The end-of-support date (absent when not committed).
    pub end_of_support_on: Option<String>,
}

impl SupportWindowDates {
    /// A fully-committed window with both dates.
    pub fn committed(full_support_until: &str, end_of_support_on: &str) -> Self {
        Self {
            full_support_until: Some(full_support_until.to_owned()),
            end_of_support_on: Some(end_of_support_on.to_owned()),
        }
    }
}

/// The overlap window during which a channel's prior version stays supported alongside the new one, so
/// a user can postpone or run side-by-side rather than upgrade immediately.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverlapWindow {
    /// True when a prior version is supported in parallel.
    pub has_overlap: bool,
    /// The predecessor version kept supported during the overlap (set when [`has_overlap`](Self::has_overlap)).
    pub predecessor_version: Option<String>,
    /// The date the overlap window closes (set when [`has_overlap`](Self::has_overlap)).
    pub overlap_until: Option<String>,
    /// Routable message id describing the overlap window (always set, even when closed).
    pub overlap_message_id: String,
}

impl OverlapWindow {
    /// An open overlap window with a predecessor version and a close date.
    pub fn overlapping(target_token: &str, predecessor_version: &str, overlap_until: &str) -> Self {
        Self {
            has_overlap: true,
            predecessor_version: Some(predecessor_version.to_owned()),
            overlap_until: Some(overlap_until.to_owned()),
            overlap_message_id: overlap_message_id(target_token),
        }
    }

    /// A closed (or never-opened) overlap window, disclosed honestly.
    pub fn none(target_token: &str) -> Self {
        Self {
            has_overlap: false,
            predecessor_version: None,
            overlap_until: None,
            overlap_message_id: overlap_message_id(target_token),
        }
    }

    /// True when the overlap is disclosed coherently: an open overlap names its predecessor and close
    /// date, and the overlap message id is always present.
    fn is_disclosed(&self) -> bool {
        !self.overlap_message_id.is_empty()
            && (!self.has_overlap
                || (self.predecessor_version.is_some() && self.overlap_until.is_some()))
    }
}

fn overlap_message_id(target_token: &str) -> String {
    format!(
        "{}overlap.{}",
        M5_SUPPORT_WINDOW_MESSAGE_ID_PREFIX, target_token
    )
}

/// The deprecation horizon and removal target a channel card discloses: the successor channel, the
/// deprecation and removal dates, and the version a removal targets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeprecationHorizon {
    /// The successor channel a user should move to (absent when none is named).
    pub successor_channel: Option<ChannelScope>,
    /// The date the channel is (or was) deprecated (absent when none).
    pub deprecation_on: Option<String>,
    /// The version a removal targets (absent when none).
    pub removal_target_version: Option<String>,
    /// The date a removal is (or was) scheduled (absent when none).
    pub removal_on: Option<String>,
    /// Routable message id naming the replacement guidance (set when a successor / replacement exists).
    pub replacement_message_id: Option<String>,
}

impl DeprecationHorizon {
    /// A horizon with no deprecation or removal scheduled.
    pub fn none() -> Self {
        Self {
            successor_channel: None,
            deprecation_on: None,
            removal_target_version: None,
            removal_on: None,
            replacement_message_id: None,
        }
    }

    /// True when the card names a replacement path (a successor channel or a replacement message).
    fn names_replacement(&self) -> bool {
        self.successor_channel.is_some() || self.replacement_message_id.is_some()
    }
}

/// The pin-or-postpone guidance a card discloses: the chosen path, opaque guidance refs, and a
/// routable recovery message id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinPostponeGuidance {
    /// The pin-or-postpone path.
    pub choice: PinPostponeChoice,
    /// Opaque refs to guidance (no raw payloads).
    pub guidance_refs: Vec<String>,
    /// Routable message id for the recovery / deferral guidance.
    pub recovery_message_id: String,
}

impl PinPostponeGuidance {
    /// Builds pin-or-postpone guidance for a target with the given path and refs.
    pub fn new(target_token: &str, choice: PinPostponeChoice, guidance_refs: &[&str]) -> Self {
        Self {
            choice,
            guidance_refs: guidance_refs.iter().map(|s| (*s).to_owned()).collect(),
            recovery_message_id: format!(
                "{}recovery.{}",
                M5_SUPPORT_WINDOW_MESSAGE_ID_PREFIX, target_token
            ),
        }
    }

    /// The no-op "stay on channel" guidance for a fully-supported target.
    pub fn stay(target_token: &str) -> Self {
        Self::new(target_token, PinPostponeChoice::StayOnChannel, &[])
    }

    /// True when the guidance names a real recovery / deferral / upgrade path with backing refs.
    fn is_active(&self) -> bool {
        self.choice.is_active_path()
            && !self.guidance_refs.is_empty()
            && !self.recovery_message_id.is_empty()
    }
}

/// One known compatibility caveat a card discloses, scoped to the artifact class it affects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityCaveat {
    /// Routable message id naming the caveat.
    pub caveat_message_id: String,
    /// The artifact class the caveat affects.
    pub affected_artifact_class: ArtifactClass,
    /// Opaque refs backing the caveat (no raw payloads).
    pub refs: Vec<String>,
}

impl CompatibilityCaveat {
    /// Builds a caveat for a target, scoped to an artifact class.
    pub fn new(
        target_token: &str,
        slug: &str,
        affected_artifact_class: ArtifactClass,
        refs: &[&str],
    ) -> Self {
        Self {
            caveat_message_id: format!(
                "{}caveat.{}.{}",
                M5_SUPPORT_WINDOW_MESSAGE_ID_PREFIX, target_token, slug
            ),
            affected_artifact_class,
            refs: refs.iter().map(|s| (*s).to_owned()).collect(),
        }
    }
}

/// The compatibility window a subject card discloses: the supported floor→ceiling versions and where
/// the installed version sits inside it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityWindow {
    /// The supported floor version (absent when open-ended below).
    pub floor_version: Option<String>,
    /// The installed / current version (absent when unknown).
    pub current_version: Option<String>,
    /// The supported ceiling version (absent when open-ended above).
    pub ceiling_version: Option<String>,
    /// Where the current version sits inside the window.
    pub posture: CompatibilityWindowPosture,
}

// ---------------------------------------------------------------------------
// Channel support card
// ---------------------------------------------------------------------------

/// Builder input for [`ChannelSupportCard::new`].
#[derive(Debug, Clone)]
pub struct ChannelSupportCardInput {
    /// The channel this card covers.
    pub channel: ChannelScope,
    /// The current support-window state.
    pub support_window_state: SupportWindowState,
    /// The current end-of-support state.
    pub end_of_support_state: EndOfSupportState,
    /// The support-window dates.
    pub support_window: SupportWindowDates,
    /// The overlap window with the prior version.
    pub overlap_window: OverlapWindow,
    /// The deprecation horizon and removal target.
    pub deprecation_horizon: DeprecationHorizon,
    /// The pin-or-postpone guidance.
    pub pin_postpone: PinPostponeGuidance,
    /// Known compatibility caveats.
    pub compatibility_caveats: Vec<CompatibilityCaveat>,
    /// Deployment profiles this channel covers.
    pub profiles: Vec<DeploymentProfile>,
    /// Opaque evidence refs backing the card (no raw payloads).
    pub evidence_refs: Vec<String>,
}

/// The typed support-lifecycle card for one [channel](ChannelScope): its identity, support window,
/// overlap window, deprecation horizon, removal target, pin-or-postpone path, compatibility caveats,
/// and derived readiness. The card's gate is the *worse* of the support-window and end-of-support
/// postures, so a card never advertises a wider commitment than its weakest promise.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelSupportCard {
    /// The channel.
    pub channel: ChannelScope,
    /// Human-facing channel label.
    pub channel_label: String,
    /// One-line channel identity blurb.
    pub channel_description: String,
    /// Accountable owner role.
    pub owner_role: String,
    /// The current support-window state.
    pub support_window_state: SupportWindowState,
    /// Reviewer-facing support-window-state label.
    pub support_window_state_label: String,
    /// The current end-of-support state.
    pub end_of_support_state: EndOfSupportState,
    /// Reviewer-facing end-of-support-state label.
    pub end_of_support_state_label: String,
    /// The support-window dates.
    pub support_window: SupportWindowDates,
    /// The overlap window.
    pub overlap_window: OverlapWindow,
    /// The deprecation horizon and removal target.
    pub deprecation_horizon: DeprecationHorizon,
    /// The pin-or-postpone guidance.
    pub pin_postpone: PinPostponeGuidance,
    /// Known compatibility caveats.
    pub compatibility_caveats: Vec<CompatibilityCaveat>,
    /// The deployment profiles this channel covers.
    pub profiles: Vec<DeploymentProfile>,
    /// Opaque evidence refs backing the card.
    pub evidence_refs: Vec<String>,
    /// True when the card carries the replacement / overlap / recovery guidance a card under
    /// lifecycle pressure must carry instead of a bare warning.
    pub carries_recovery_guidance: bool,
    /// Gate: the worse of the support-window and end-of-support postures.
    pub gate: DescriptorGate,
    /// Readiness mirroring [`gate`](Self::gate).
    pub readiness: SupportReadiness,
    /// Coverage status mirroring [`gate`](Self::gate).
    pub status: ConsumerStatus,
    /// Traffic-light signal mirroring [`gate`](Self::gate).
    pub signal: DescriptorSignal,
    /// True when the card is out of support / removed and an upgrade action is required.
    pub requires_migration_action: bool,
    /// Routable message id for the card's summary line.
    pub summary_message_id: String,
    /// Routable message id for the card's detail.
    pub detail_message_id: String,
}

impl ChannelSupportCard {
    /// Builds a channel card from its inputs, deriving the gate, readiness, and recovery-guidance
    /// flag.
    pub fn new(input: ChannelSupportCardInput) -> Self {
        let channel = input.channel;
        let mut card = Self {
            channel,
            channel_label: channel_label(channel).to_owned(),
            channel_description: channel_description(channel).to_owned(),
            owner_role: channel_owner_role(channel).to_owned(),
            support_window_state: input.support_window_state,
            support_window_state_label: input.support_window_state.label().to_owned(),
            end_of_support_state: input.end_of_support_state,
            end_of_support_state_label: input.end_of_support_state.label().to_owned(),
            support_window: input.support_window,
            overlap_window: input.overlap_window,
            deprecation_horizon: input.deprecation_horizon,
            pin_postpone: input.pin_postpone,
            compatibility_caveats: input.compatibility_caveats,
            profiles: input.profiles,
            evidence_refs: input.evidence_refs,
            carries_recovery_guidance: false,
            gate: DescriptorGate::Governed,
            readiness: SupportReadiness::Supported,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            requires_migration_action: false,
            summary_message_id: format!(
                "{}channel.{}.summary",
                M5_SUPPORT_WINDOW_MESSAGE_ID_PREFIX,
                channel.as_str()
            ),
            detail_message_id: format!(
                "{}channel.{}.detail",
                M5_SUPPORT_WINDOW_MESSAGE_ID_PREFIX,
                channel.as_str()
            ),
        };
        card.recompute();
        card
    }

    /// Recomputes the derived verdict, recovery-guidance flag, and sorted scope.
    pub fn recompute(&mut self) {
        sort_profiles(&mut self.profiles);
        sort_caveats(&mut self.compatibility_caveats);

        let gate = worst_gate(
            self.support_window_state.gate_posture(),
            self.end_of_support_state.gate_posture(),
        );
        self.gate = gate;
        self.readiness = SupportReadiness::from_gate(gate);
        self.status = status_for_gate(gate);
        self.signal = signal_for_gate(gate);
        self.requires_migration_action = gate == DescriptorGate::Blocked;
        self.carries_recovery_guidance = self.has_recovery_guidance();
    }

    /// True when the channel is under any lifecycle pressure (not full support and supported).
    fn needs_recovery_guidance(&self) -> bool {
        self.gate != DescriptorGate::Governed
    }

    /// True when the card carries replacement, overlap, and recovery guidance.
    fn has_recovery_guidance(&self) -> bool {
        self.deprecation_horizon.names_replacement()
            && self.overlap_window.is_disclosed()
            && self.pin_postpone.is_active()
    }

    /// The gap kind this card contributes to a consumer that reads it, if any.
    fn gap_kind(&self) -> Option<SupportGapKind> {
        match self.gate {
            DescriptorGate::Governed => None,
            DescriptorGate::Narrowed => Some(SupportGapKind::MigrationRecommended),
            DescriptorGate::Blocked => Some(SupportGapKind::ActionRequiredBeforeUpgrade),
        }
    }
}

// ---------------------------------------------------------------------------
// Compatibility-subject card
// ---------------------------------------------------------------------------

/// Builder input for [`CompatibilitySubjectCard::new`].
#[derive(Debug, Clone)]
pub struct CompatibilitySubjectCardInput {
    /// The subject this card covers.
    pub subject: CompatibilitySubject,
    /// The current end-of-support state.
    pub end_of_support_state: EndOfSupportState,
    /// The compatibility window.
    pub compatibility_window: CompatibilityWindow,
    /// Routable message id naming the successor / replacement (set when the subject is deprecated or
    /// outside its window).
    pub successor_message_id: Option<String>,
    /// The pin-or-postpone guidance.
    pub pin_postpone: PinPostponeGuidance,
    /// Known compatibility caveats.
    pub compatibility_caveats: Vec<CompatibilityCaveat>,
    /// Deployment profiles this subject covers.
    pub profiles: Vec<DeploymentProfile>,
    /// Opaque evidence refs backing the card (no raw payloads).
    pub evidence_refs: Vec<String>,
}

/// The typed end-of-support / compatibility-window card for one [subject](CompatibilitySubject):
/// workspace/profile files, extension SDKs/manifests, remote helpers, or public schemas. The card's
/// gate is the worse of the end-of-support and compatibility-window postures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilitySubjectCard {
    /// The subject.
    pub subject: CompatibilitySubject,
    /// Human-facing subject label.
    pub subject_label: String,
    /// The subject's primary artifact class.
    pub primary_artifact_class: ArtifactClass,
    /// Accountable owner role.
    pub owner_role: String,
    /// The current end-of-support state.
    pub end_of_support_state: EndOfSupportState,
    /// Reviewer-facing end-of-support-state label.
    pub end_of_support_state_label: String,
    /// The compatibility window.
    pub compatibility_window: CompatibilityWindow,
    /// Routable message id naming the successor / replacement.
    pub successor_message_id: Option<String>,
    /// The pin-or-postpone guidance.
    pub pin_postpone: PinPostponeGuidance,
    /// Known compatibility caveats.
    pub compatibility_caveats: Vec<CompatibilityCaveat>,
    /// The deployment profiles this subject covers.
    pub profiles: Vec<DeploymentProfile>,
    /// Opaque evidence refs backing the card.
    pub evidence_refs: Vec<String>,
    /// True when the card carries replacement / recovery guidance instead of a bare warning.
    pub carries_recovery_guidance: bool,
    /// Gate: the worse of the end-of-support and compatibility-window postures.
    pub gate: DescriptorGate,
    /// Readiness mirroring [`gate`](Self::gate).
    pub readiness: SupportReadiness,
    /// Coverage status mirroring [`gate`](Self::gate).
    pub status: ConsumerStatus,
    /// Traffic-light signal mirroring [`gate`](Self::gate).
    pub signal: DescriptorSignal,
    /// True when the subject is out of support / outside its window and an upgrade action is required.
    pub requires_migration_action: bool,
    /// Routable message id for the card's summary line.
    pub summary_message_id: String,
    /// Routable message id for the card's detail.
    pub detail_message_id: String,
}

impl CompatibilitySubjectCard {
    /// Builds a subject card from its inputs, deriving the gate, readiness, and recovery-guidance
    /// flag.
    pub fn new(input: CompatibilitySubjectCardInput) -> Self {
        let subject = input.subject;
        let mut card = Self {
            subject,
            subject_label: subject.label().to_owned(),
            primary_artifact_class: subject.primary_artifact_class(),
            owner_role: subject.owner_role().to_owned(),
            end_of_support_state: input.end_of_support_state,
            end_of_support_state_label: input.end_of_support_state.label().to_owned(),
            compatibility_window: input.compatibility_window,
            successor_message_id: input.successor_message_id,
            pin_postpone: input.pin_postpone,
            compatibility_caveats: input.compatibility_caveats,
            profiles: input.profiles,
            evidence_refs: input.evidence_refs,
            carries_recovery_guidance: false,
            gate: DescriptorGate::Governed,
            readiness: SupportReadiness::Supported,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            requires_migration_action: false,
            summary_message_id: format!(
                "{}subject.{}.summary",
                M5_SUPPORT_WINDOW_MESSAGE_ID_PREFIX,
                subject.as_str()
            ),
            detail_message_id: format!(
                "{}subject.{}.detail",
                M5_SUPPORT_WINDOW_MESSAGE_ID_PREFIX,
                subject.as_str()
            ),
        };
        card.recompute();
        card
    }

    /// Recomputes the derived verdict, recovery-guidance flag, and sorted scope.
    pub fn recompute(&mut self) {
        sort_profiles(&mut self.profiles);
        sort_caveats(&mut self.compatibility_caveats);

        let gate = worst_gate(
            self.end_of_support_state.gate_posture(),
            self.compatibility_window.posture.gate_posture(),
        );
        self.gate = gate;
        self.readiness = SupportReadiness::from_gate(gate);
        self.status = status_for_gate(gate);
        self.signal = signal_for_gate(gate);
        self.requires_migration_action = gate == DescriptorGate::Blocked;
        self.carries_recovery_guidance = self.has_recovery_guidance();
    }

    /// True when the subject is under any lifecycle pressure (not supported and within window).
    fn needs_recovery_guidance(&self) -> bool {
        self.gate != DescriptorGate::Governed
    }

    /// True when the card carries replacement and recovery guidance.
    fn has_recovery_guidance(&self) -> bool {
        self.successor_message_id.is_some() && self.pin_postpone.is_active()
    }

    /// The gap kind this card contributes to a consumer that reads it, if any.
    fn gap_kind(&self) -> Option<SupportGapKind> {
        match self.gate {
            DescriptorGate::Governed => None,
            DescriptorGate::Narrowed => Some(SupportGapKind::MigrationRecommended),
            DescriptorGate::Blocked => Some(SupportGapKind::ActionRequiredBeforeUpgrade),
        }
    }
}

// ---------------------------------------------------------------------------
// Consumer rows
// ---------------------------------------------------------------------------

/// A gap a consumer carries for one channel or subject card it reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportGap {
    /// The consumer that carries the gap.
    pub consumer: SupportConsumer,
    /// Whether the gap points at a channel or a subject.
    pub target_kind: SupportTargetKind,
    /// The channel / subject token the gap points at.
    pub target_token: String,
    /// The named cause of the gap.
    pub gap_kind: SupportGapKind,
    /// Routable message id naming the cause.
    pub cause_message_id: String,
}

fn make_gap(
    consumer: SupportConsumer,
    target_kind: SupportTargetKind,
    target_token: &str,
    kind: SupportGapKind,
) -> SupportGap {
    SupportGap {
        consumer,
        target_kind,
        target_token: target_token.to_owned(),
        gap_kind: kind,
        cause_message_id: format!(
            "{}consumer.{}.{}.{}.{}.gap",
            M5_SUPPORT_WINDOW_MESSAGE_ID_PREFIX,
            consumer.as_str(),
            target_kind.as_str(),
            target_token,
            kind.as_str()
        ),
    }
}

/// A consumer surface bound to the channels and subjects it reads, with its readiness, decision, and
/// gaps derived from those cards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportConsumerRow {
    /// The consumer surface.
    pub consumer: SupportConsumer,
    /// Human-facing label.
    pub consumer_label: String,
    /// Accountable owner role.
    pub owner_role: String,
    /// The channels this consumer reads.
    pub read_channels: Vec<ChannelScope>,
    /// The subjects this consumer reads.
    pub read_subjects: Vec<CompatibilitySubject>,
    /// The union of deployment profiles across the read cards.
    pub profiles: Vec<DeploymentProfile>,
    /// The derived readiness.
    pub readiness: SupportReadiness,
    /// Coverage status.
    pub status: ConsumerStatus,
    /// Traffic-light signal.
    pub signal: DescriptorSignal,
    /// Gate decision.
    pub gate_decision: DescriptorGate,
    /// True when at least one read card is out of support / removed.
    pub requires_migration_action: bool,
    /// Gaps, one per (target, cause).
    pub gaps: Vec<SupportGap>,
    /// Routable status message id.
    pub status_message_id: String,
    /// Routable decision message id.
    pub decision_message_id: String,
}

impl SupportConsumerRow {
    /// Builds a consumer row; the resolved unions, gaps, and verdict are recomputed against the
    /// packet's cards when the packet is assembled.
    pub fn new(
        consumer: SupportConsumer,
        read_channels: &[ChannelScope],
        read_subjects: &[CompatibilitySubject],
    ) -> Self {
        Self {
            consumer,
            consumer_label: consumer.label().to_owned(),
            owner_role: consumer.owner_role().to_owned(),
            read_channels: read_channels.to_vec(),
            read_subjects: read_subjects.to_vec(),
            profiles: Vec::new(),
            readiness: SupportReadiness::Supported,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            gate_decision: DescriptorGate::Governed,
            requires_migration_action: false,
            gaps: Vec::new(),
            status_message_id: format!(
                "{}consumer.{}.status",
                M5_SUPPORT_WINDOW_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
            decision_message_id: format!(
                "{}consumer.{}.decision",
                M5_SUPPORT_WINDOW_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
        }
    }

    /// Recomputes the resolved unions, gaps, and verdict from the packet's cards, so a consumer's
    /// readiness is always generated from the same checked-in cards rather than a hand-maintained
    /// status.
    pub fn recompute(
        &mut self,
        channels: &[ChannelSupportCard],
        subjects: &[CompatibilitySubjectCard],
    ) {
        let mut read_channels = self.read_channels.clone();
        read_channels.sort_by_key(|c| channel_rank(*c));
        read_channels.dedup();
        self.read_channels = read_channels.clone();

        let mut read_subjects = self.read_subjects.clone();
        read_subjects.sort_by_key(|s| subject_rank(*s));
        read_subjects.dedup();
        self.read_subjects = read_subjects.clone();

        let consumer = self.consumer;
        let mut profiles: Vec<DeploymentProfile> = Vec::new();
        let mut gaps: Vec<SupportGap> = Vec::new();

        for &channel in &read_channels {
            match channels.iter().find(|c| c.channel == channel) {
                None => gaps.push(make_gap(
                    consumer,
                    SupportTargetKind::Channel,
                    channel.as_str(),
                    SupportGapKind::ChannelNotPublished,
                )),
                Some(card) => {
                    profiles.extend(card.profiles.iter().copied());
                    if let Some(kind) = card.gap_kind() {
                        gaps.push(make_gap(
                            consumer,
                            SupportTargetKind::Channel,
                            channel.as_str(),
                            kind,
                        ));
                    }
                }
            }
        }

        for &subject in &read_subjects {
            match subjects.iter().find(|c| c.subject == subject) {
                None => gaps.push(make_gap(
                    consumer,
                    SupportTargetKind::Subject,
                    subject.as_str(),
                    SupportGapKind::SubjectNotPublished,
                )),
                Some(card) => {
                    profiles.extend(card.profiles.iter().copied());
                    if let Some(kind) = card.gap_kind() {
                        gaps.push(make_gap(
                            consumer,
                            SupportTargetKind::Subject,
                            subject.as_str(),
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

        let mut gate = DescriptorGate::Governed;
        for gap in &self.gaps {
            gate = worst_gate(gate, gap.gap_kind.gate());
        }
        self.gate_decision = gate;
        self.readiness = SupportReadiness::from_gate(gate);
        self.status = status_for_gate(gate);
        self.signal = signal_for_gate(gate);
        self.requires_migration_action = gate == DescriptorGate::Blocked;
    }

    /// True when every read card is fully supported.
    pub fn is_supported(&self) -> bool {
        self.gate_decision == DescriptorGate::Governed
    }

    /// True when at least one read card narrows the consumer to plan-migration.
    pub fn is_plan_migration(&self) -> bool {
        self.gate_decision == DescriptorGate::Narrowed
    }

    /// True when at least one read card forces an action-required state.
    pub fn is_action_required(&self) -> bool {
        self.gate_decision == DescriptorGate::Blocked
    }
}

// ---------------------------------------------------------------------------
// Aggregate sub-objects
// ---------------------------------------------------------------------------

/// Disclosure flags asserting every claimed consumer ingests this one card set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportDisclosure {
    /// Help / About consumes the card set.
    pub help_about_reads_packet: bool,
    /// Docs / help consumes the card set.
    pub docs_help_reads_packet: bool,
    /// The update center consumes the card set.
    pub update_center_reads_packet: bool,
    /// The compatibility report consumes the card set.
    pub compatibility_report_reads_packet: bool,
    /// Support export consumes the card set.
    pub support_export_reads_packet: bool,
    /// The admin console consumes the card set.
    pub admin_console_reads_packet: bool,
    /// The release center consumes the card set.
    pub release_center_reads_packet: bool,
}

impl SupportDisclosure {
    fn canonical() -> Self {
        Self {
            help_about_reads_packet: true,
            docs_help_reads_packet: true,
            update_center_reads_packet: true,
            compatibility_report_reads_packet: true,
            support_export_reads_packet: true,
            admin_console_reads_packet: true,
            release_center_reads_packet: true,
        }
    }

    /// True when every consumer is asserted to consume the card set.
    pub fn all_consume(&self) -> bool {
        self.help_about_reads_packet
            && self.docs_help_reads_packet
            && self.update_center_reads_packet
            && self.compatibility_report_reads_packet
            && self.support_export_reads_packet
            && self.admin_console_reads_packet
            && self.release_center_reads_packet
    }
}

/// Roll-up counts over the cards and consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCounts {
    /// Total channel cards.
    pub total_channels: u32,
    /// Channel cards fully supported.
    pub supported_channels: u32,
    /// Channel cards under lifecycle pressure (plan-migration).
    pub plan_migration_channels: u32,
    /// Channel cards out of support / removed (action-required).
    pub action_required_channels: u32,
    /// Total subject cards.
    pub total_subjects: u32,
    /// Subject cards fully supported.
    pub supported_subjects: u32,
    /// Subject cards under lifecycle pressure (plan-migration).
    pub plan_migration_subjects: u32,
    /// Subject cards outside support / window (action-required).
    pub action_required_subjects: u32,
    /// Cards (channel or subject) under any lifecycle pressure.
    pub deprecated_or_eos_cards: u32,
    /// Total consumers.
    pub total_consumers: u32,
    /// Consumers fully supported.
    pub supported_consumers: u32,
    /// Consumers under plan-migration.
    pub plan_migration_consumers: u32,
    /// Consumers under action-required.
    pub action_required_consumers: u32,
    /// Whether the packet requires a migration action.
    pub requires_migration_action: bool,
}

/// The packet-level support-pressure honesty block: how much of the lifecycle is fully supported vs.
/// narrowing or blocking, so pressure is disclosed rather than implied absent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCoverage {
    /// Cards fully supported (governed).
    pub fully_supported_cards: u32,
    /// Cards narrowing (plan-migration).
    pub narrowing_cards: u32,
    /// Cards blocking (action-required).
    pub blocking_cards: u32,
    /// True when at least one card is under lifecycle pressure.
    pub has_lifecycle_pressure: bool,
    /// The data state the packet was rendered under, labelled honestly.
    pub data_state: StaleDataBehavior,
    /// True when the packet is showing live, verified data.
    pub live_data: bool,
}

/// The packet-level migration gate aggregating the per-consumer decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportReleaseGate {
    /// Whether any consumer requires a migration action.
    pub requires_migration_action: bool,
    /// Tokens of the action-required consumers.
    pub action_required_consumers: Vec<String>,
    /// Tokens of the plan-migration consumers.
    pub plan_migration_consumers: Vec<String>,
    /// Tokens of the supported consumers.
    pub supported_consumers: Vec<String>,
    /// Tokens of the channels that contributed a gap.
    pub affected_channels: Vec<String>,
    /// Tokens of the subjects that contributed a gap.
    pub affected_subjects: Vec<String>,
    /// Routable gate message id.
    pub gate_message_id: String,
}

/// The frozen controlled vocabulary the cards draw from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportVocabulary {
    /// Channel tokens.
    pub channels: Vec<String>,
    /// Support-window-state tokens.
    pub support_window_states: Vec<String>,
    /// End-of-support-state tokens.
    pub end_of_support_states: Vec<String>,
    /// Compatibility-window-posture tokens.
    pub compatibility_window_postures: Vec<String>,
    /// Pin-or-postpone-choice tokens.
    pub pin_postpone_choices: Vec<String>,
    /// Compatibility-subject tokens.
    pub compatibility_subjects: Vec<String>,
    /// Artifact-class tokens.
    pub artifact_classes: Vec<String>,
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

impl SupportVocabulary {
    /// The canonical frozen vocabulary.
    pub fn canonical() -> Self {
        Self {
            channels: tokens(&ChannelScope::ALL, |x| x.as_str()),
            support_window_states: tokens(&SupportWindowState::ALL, |x| x.as_str()),
            end_of_support_states: tokens(&EndOfSupportState::ALL, |x| x.as_str()),
            compatibility_window_postures: tokens(&CompatibilityWindowPosture::ALL, |x| x.as_str()),
            pin_postpone_choices: tokens(&PinPostponeChoice::ALL, |x| x.as_str()),
            compatibility_subjects: tokens(&CompatibilitySubject::ALL, |x| x.as_str()),
            artifact_classes: tokens(&ArtifactClass::ALL, |x| x.as_str()),
            profiles: tokens(&DeploymentProfile::ALL, |x| x.as_str()),
            consumers: tokens(&SupportConsumer::ALL, |x| x.as_str()),
            gap_kinds: tokens(&SupportGapKind::ALL, |x| x.as_str()),
            target_kinds: tokens(&SupportTargetKind::ALL, |x| x.as_str()),
            gate_decisions: tokens(&DescriptorGate::ALL, |x| x.as_str()),
            readiness: tokens(&SupportReadiness::ALL, |x| x.as_str()),
            stale_data_behaviors: tokens(&StaleDataBehavior::ALL, |x| x.as_str()),
        }
    }

    /// True when this vocabulary equals the canonical frozen vocabulary.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Conformance flags every canonical card set asserts. They restate the acceptance bar so a tampered
/// packet that flips one to false fails [`SupportWindowCardSet::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportConformance {
    /// Every channel is carded exactly once.
    pub every_channel_carded: bool,
    /// Every compatibility subject is carded exactly once.
    pub every_subject_carded: bool,
    /// Channel identity (label + description) is disclosed per channel.
    pub channel_identity_disclosed: bool,
    /// The support window is disclosed per channel.
    pub support_window_disclosed: bool,
    /// The overlap window is disclosed per channel.
    pub overlap_window_disclosed: bool,
    /// The deprecation horizon is disclosed per channel.
    pub deprecation_horizon_disclosed: bool,
    /// The removal target is disclosed where a removal is scheduled.
    pub removal_target_disclosed: bool,
    /// A pin-or-postpone path is disclosed per card.
    pub pin_or_postpone_path_disclosed: bool,
    /// Known compatibility caveats are disclosed per card.
    pub compatibility_caveats_disclosed: bool,
    /// Deprecated / end-of-support states carry replacement, overlap, and recovery guidance.
    pub deprecated_states_carry_replacement_overlap_recovery: bool,
    /// End-of-support and compatibility-window posture is shown for every claimed subject.
    pub eos_and_compatibility_posture_shown_for_subjects: bool,
    /// Help, update, and compatibility-report surfaces share this one packet.
    pub help_update_compatibility_share_one_packet: bool,
    /// No card advertises a wider support commitment than its weakest promise.
    pub support_commitments_not_broadened: bool,
    /// Every consumer verdict is derived from the cards, not hand-maintained.
    pub consumer_verdict_derived_from_cards: bool,
    /// The data state is labelled and local-safe.
    pub data_state_labelled_local_safe: bool,
    /// The controlled enums are frozen.
    pub controlled_enums_frozen: bool,
    /// The card set is exportable and reviewable outside the app.
    pub exportable_outside_app: bool,
    /// The export carries metadata and refs only — no credential bodies or raw payloads.
    pub export_carries_no_raw_material: bool,
}

impl SupportConformance {
    fn canonical() -> Self {
        Self {
            every_channel_carded: true,
            every_subject_carded: true,
            channel_identity_disclosed: true,
            support_window_disclosed: true,
            overlap_window_disclosed: true,
            deprecation_horizon_disclosed: true,
            removal_target_disclosed: true,
            pin_or_postpone_path_disclosed: true,
            compatibility_caveats_disclosed: true,
            deprecated_states_carry_replacement_overlap_recovery: true,
            eos_and_compatibility_posture_shown_for_subjects: true,
            help_update_compatibility_share_one_packet: true,
            support_commitments_not_broadened: true,
            consumer_verdict_derived_from_cards: true,
            data_state_labelled_local_safe: true,
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
    /// The desktop update center / Help / About.
    DesktopUi,
    /// The CLI / headless emitter.
    CliHeadless,
    /// The offline / exported review surface.
    OfflineExport,
}

// ---------------------------------------------------------------------------
// Validation violations
// ---------------------------------------------------------------------------

/// A reason a card set failed [`SupportWindowCardSet::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportWindowViolation {
    /// The record kind or schema version is wrong.
    HeaderDrift,
    /// A channel is missing or carded more than once.
    ChannelCoverageDrift,
    /// A subject is missing or carded more than once.
    SubjectCoverageDrift,
    /// A channel card's derived verdict / scope / recovery flag drifted.
    ChannelDerivationDrift,
    /// A subject card's derived verdict / scope / recovery flag drifted.
    SubjectDerivationDrift,
    /// A card advertises a wider support commitment than its weakest promise — the lane's guardrail.
    OverBroadenedCommitment,
    /// A card under lifecycle pressure lacks replacement, overlap, or recovery guidance.
    MissingRecoveryGuidance,
    /// A consumer's derived verdict, unions, or gaps drifted.
    ConsumerVerdictDrift,
    /// The summary counts, coverage, or migration gate drifted.
    SummaryDrift,
    /// The disclosure flags do not all assert consumption of the one card set.
    DisclosureDrift,
    /// The controlled vocabulary drifted.
    VocabularyDrift,
    /// A conformance flag does not hold.
    ConformanceDrift,
    /// The export carried forbidden raw material.
    ForbiddenMaterial,
}

impl SupportWindowViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeaderDrift => "header_drift",
            Self::ChannelCoverageDrift => "channel_coverage_drift",
            Self::SubjectCoverageDrift => "subject_coverage_drift",
            Self::ChannelDerivationDrift => "channel_derivation_drift",
            Self::SubjectDerivationDrift => "subject_derivation_drift",
            Self::OverBroadenedCommitment => "over_broadened_commitment",
            Self::MissingRecoveryGuidance => "missing_recovery_guidance",
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

/// Builder input for [`SupportWindowCardSet::new`].
#[derive(Debug, Clone)]
pub struct SupportWindowCardSetInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-facing report label.
    pub report_label: String,
    /// Evaluation timestamp.
    pub evaluated_at: String,
    /// The data state the packet was rendered under.
    pub data_state: StaleDataBehavior,
    /// The per-channel cards.
    pub channels: Vec<ChannelSupportCard>,
    /// The per-subject cards.
    pub subjects: Vec<CompatibilitySubjectCard>,
    /// The claimed consumer rows.
    pub consumers: Vec<SupportConsumerRow>,
    /// Redaction-class token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable support-lifecycle card set Help/About, docs/help, the
/// update center, the compatibility report, support export, the admin console, and the release center
/// consume to decide whether to upgrade, pin, postpone, or roll out a channel broadly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportWindowCardSet {
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
    /// The per-channel cards.
    pub channels: Vec<ChannelSupportCard>,
    /// The channel tokens, in canonical order.
    pub channel_tokens: Vec<String>,
    /// The per-subject cards.
    pub subjects: Vec<CompatibilitySubjectCard>,
    /// The subject tokens, in canonical order.
    pub subject_tokens: Vec<String>,
    /// The consumer rows reading the cards.
    pub consumers: Vec<SupportConsumerRow>,
    /// The consumer tokens, in canonical order.
    pub consumer_tokens: Vec<String>,
    /// Disclosure flags.
    pub disclosure: SupportDisclosure,
    /// Roll-up counts.
    pub summary: SupportCounts,
    /// Support-pressure honesty block.
    pub coverage: SupportCoverage,
    /// Packet-level migration gate.
    pub release_gate: SupportReleaseGate,
    /// Controlled vocabulary.
    pub vocabulary: SupportVocabulary,
    /// Conformance flags.
    pub conformance: SupportConformance,
    /// Redaction-class token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl SupportWindowCardSet {
    /// Builds a packet from the given cards and consumer rows, recomputing every derived field so the
    /// published packet is always generated from the same checked-in cards.
    pub fn new(input: SupportWindowCardSetInput) -> Self {
        let mut channels = input.channels;
        for card in &mut channels {
            card.recompute();
        }
        channels.sort_by_key(|c| channel_rank(c.channel));

        let mut subjects = input.subjects;
        for card in &mut subjects {
            card.recompute();
        }
        subjects.sort_by_key(|c| subject_rank(c.subject));

        let mut consumers = input.consumers;
        for consumer in &mut consumers {
            consumer.recompute(&channels, &subjects);
        }
        consumers.sort_by_key(|c| consumer_rank(c.consumer));

        let summary = derive_counts(&channels, &subjects, &consumers);
        let coverage = derive_coverage(&channels, &subjects, input.data_state);
        let release_gate = derive_release_gate(&consumers);

        Self {
            record_kind: M5_SUPPORT_WINDOW_CARD_SET_RECORD_KIND.to_owned(),
            schema_version: M5_SUPPORT_WINDOW_CARD_SET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            data_state: input.data_state,
            channel_tokens: tokens(&ChannelScope::ALL, |x| x.as_str()),
            channels,
            subject_tokens: tokens(&CompatibilitySubject::ALL, |x| x.as_str()),
            subjects,
            consumer_tokens: tokens(&SupportConsumer::ALL, |x| x.as_str()),
            consumers,
            disclosure: SupportDisclosure::canonical(),
            summary,
            coverage,
            release_gate,
            vocabulary: SupportVocabulary::canonical(),
            conformance: SupportConformance::canonical(),
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Looks up the card for a channel.
    pub fn channel(&self, channel: ChannelScope) -> Option<&ChannelSupportCard> {
        self.channels.iter().find(|c| c.channel == channel)
    }

    /// Looks up the card for a subject.
    pub fn subject(&self, subject: CompatibilitySubject) -> Option<&CompatibilitySubjectCard> {
        self.subjects.iter().find(|c| c.subject == subject)
    }

    /// Looks up the consumer row for a consumer.
    pub fn consumer(&self, consumer: SupportConsumer) -> Option<&SupportConsumerRow> {
        self.consumers.iter().find(|c| c.consumer == consumer)
    }

    /// Whether the packet requires a migration action.
    pub fn requires_migration_action(&self) -> bool {
        self.release_gate.requires_migration_action
    }

    /// Validates every derived field by recomputing it from the cards and comparing. Returns an empty
    /// vector when the packet is internally consistent.
    pub fn validate(&self) -> Vec<SupportWindowViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SUPPORT_WINDOW_CARD_SET_RECORD_KIND
            || self.schema_version != M5_SUPPORT_WINDOW_CARD_SET_SCHEMA_VERSION
        {
            violations.push(SupportWindowViolation::HeaderDrift);
        }

        // Every channel carded exactly once.
        for channel in ChannelScope::ALL {
            if self
                .channels
                .iter()
                .filter(|c| c.channel == channel)
                .count()
                != 1
            {
                violations.push(SupportWindowViolation::ChannelCoverageDrift);
                break;
            }
        }
        // Every subject carded exactly once.
        for subject in CompatibilitySubject::ALL {
            if self
                .subjects
                .iter()
                .filter(|c| c.subject == subject)
                .count()
                != 1
            {
                violations.push(SupportWindowViolation::SubjectCoverageDrift);
                break;
            }
        }

        for card in &self.channels {
            let mut fresh = card.clone();
            fresh.recompute();
            if fresh.gate != card.gate
                || fresh.readiness != card.readiness
                || fresh.status != card.status
                || fresh.signal != card.signal
                || fresh.requires_migration_action != card.requires_migration_action
                || fresh.carries_recovery_guidance != card.carries_recovery_guidance
                || fresh.profiles != card.profiles
                || fresh.compatibility_caveats != card.compatibility_caveats
            {
                violations.push(SupportWindowViolation::ChannelDerivationDrift);
            }
            // Guardrail: the gate may never be less severe than the weakest promise warrants.
            let warranted = worst_gate(
                card.support_window_state.gate_posture(),
                card.end_of_support_state.gate_posture(),
            );
            if gate_rank(card.gate) < gate_rank(warranted) {
                violations.push(SupportWindowViolation::OverBroadenedCommitment);
            }
            // A card under lifecycle pressure must carry replacement, overlap, and recovery guidance.
            if card.needs_recovery_guidance() && !card.carries_recovery_guidance {
                violations.push(SupportWindowViolation::MissingRecoveryGuidance);
            }
        }

        for card in &self.subjects {
            let mut fresh = card.clone();
            fresh.recompute();
            if fresh.gate != card.gate
                || fresh.readiness != card.readiness
                || fresh.status != card.status
                || fresh.signal != card.signal
                || fresh.requires_migration_action != card.requires_migration_action
                || fresh.carries_recovery_guidance != card.carries_recovery_guidance
                || fresh.profiles != card.profiles
                || fresh.compatibility_caveats != card.compatibility_caveats
            {
                violations.push(SupportWindowViolation::SubjectDerivationDrift);
            }
            let warranted = worst_gate(
                card.end_of_support_state.gate_posture(),
                card.compatibility_window.posture.gate_posture(),
            );
            if gate_rank(card.gate) < gate_rank(warranted) {
                violations.push(SupportWindowViolation::OverBroadenedCommitment);
            }
            if card.needs_recovery_guidance() && !card.carries_recovery_guidance {
                violations.push(SupportWindowViolation::MissingRecoveryGuidance);
            }
        }

        for consumer in &self.consumers {
            let mut fresh = SupportConsumerRow::new(
                consumer.consumer,
                &consumer.read_channels,
                &consumer.read_subjects,
            );
            fresh.recompute(&self.channels, &self.subjects);
            if fresh.gate_decision != consumer.gate_decision
                || fresh.readiness != consumer.readiness
                || fresh.status != consumer.status
                || fresh.signal != consumer.signal
                || fresh.requires_migration_action != consumer.requires_migration_action
                || fresh.profiles != consumer.profiles
                || fresh.gaps != consumer.gaps
            {
                violations.push(SupportWindowViolation::ConsumerVerdictDrift);
                break;
            }
        }

        if self.summary != derive_counts(&self.channels, &self.subjects, &self.consumers)
            || self.coverage != derive_coverage(&self.channels, &self.subjects, self.data_state)
            || self.release_gate != derive_release_gate(&self.consumers)
        {
            violations.push(SupportWindowViolation::SummaryDrift);
        }

        if !self.disclosure.all_consume()
            || self.channel_tokens != tokens(&ChannelScope::ALL, |x| x.as_str())
            || self.subject_tokens != tokens(&CompatibilitySubject::ALL, |x| x.as_str())
            || self.consumer_tokens != tokens(&SupportConsumer::ALL, |x| x.as_str())
        {
            violations.push(SupportWindowViolation::DisclosureDrift);
        }

        if !self.vocabulary.matches_canonical() {
            violations.push(SupportWindowViolation::VocabularyDrift);
        }

        if !self.conformance.all_hold() {
            violations.push(SupportWindowViolation::ConformanceDrift);
        }

        if contains_forbidden_material(self) {
            violations.push(SupportWindowViolation::ForbiddenMaterial);
        }

        violations
    }

    /// The canonical export form: pretty JSON, identical across every render channel.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("card set serializes")
    }

    /// Renders the packet for a channel. Every channel produces byte-identical output.
    pub fn render_for_channel(&self, _channel: RenderChannel) -> String {
        self.export_safe_json()
    }

    /// A compact Markdown summary of the cards and consumer verdicts, for export and review outside
    /// the app.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# {}\n\n", self.report_label));
        out.push_str(&format!(
            "{} channels ({} plan-migration, {} action-required), {} subjects, {} consumers — data state `{}`.\n\n",
            self.summary.total_channels,
            self.summary.plan_migration_channels,
            self.summary.action_required_channels,
            self.summary.total_subjects,
            self.summary.total_consumers,
            self.data_state.as_str(),
        ));
        if self.coverage.has_lifecycle_pressure {
            out.push_str(&format!(
                "> Lifecycle pressure: {} narrowing, {} blocking card(s) carry replacement/overlap/recovery guidance.\n\n",
                self.coverage.narrowing_cards, self.coverage.blocking_cards,
            ));
        }
        out.push_str("## Channel support lifecycle\n\n");
        out.push_str(
            "| Channel | Support window | End of support | Readiness | Overlap until | Successor | Pin / postpone |\n",
        );
        out.push_str("|---|---|---|---|---|---|---|\n");
        for c in &self.channels {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | {} | {} | `{}` |\n",
                c.channel.as_str(),
                c.support_window_state.as_str(),
                c.end_of_support_state.as_str(),
                c.readiness.as_str(),
                c.overlap_window.overlap_until.as_deref().unwrap_or("—"),
                c.deprecation_horizon
                    .successor_channel
                    .map(|s| s.as_str())
                    .unwrap_or("—"),
                c.pin_postpone.choice.as_str(),
            ));
        }
        out.push_str("\n## Compatibility-window subjects\n\n");
        out.push_str(
            "| Subject | End of support | Window posture | Floor → ceiling | Readiness |\n",
        );
        out.push_str("|---|---|---|---|---|\n");
        for s in &self.subjects {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} → {} | `{}` |\n",
                s.subject.as_str(),
                s.end_of_support_state.as_str(),
                s.compatibility_window.posture.as_str(),
                s.compatibility_window
                    .floor_version
                    .as_deref()
                    .unwrap_or("—"),
                s.compatibility_window
                    .ceiling_version
                    .as_deref()
                    .unwrap_or("—"),
                s.readiness.as_str(),
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
            "card_kind,target,support_window_state,end_of_support_state,compatibility_posture,readiness,pin_postpone,successor,gate\n",
        );
        for c in &self.channels {
            out.push_str(&format!(
                "channel,{},{},{},,{},{},{},{}\n",
                c.channel.as_str(),
                c.support_window_state.as_str(),
                c.end_of_support_state.as_str(),
                c.readiness.as_str(),
                c.pin_postpone.choice.as_str(),
                c.deprecation_horizon
                    .successor_channel
                    .map(|s| s.as_str())
                    .unwrap_or(""),
                c.gate.as_str(),
            ));
        }
        for s in &self.subjects {
            out.push_str(&format!(
                "subject,{},,{},{},{},{},{},{}\n",
                s.subject.as_str(),
                s.end_of_support_state.as_str(),
                s.compatibility_window.posture.as_str(),
                s.readiness.as_str(),
                s.pin_postpone.choice.as_str(),
                if s.successor_message_id.is_some() {
                    "yes"
                } else {
                    ""
                },
                s.gate.as_str(),
            ));
        }
        out
    }
}

fn derive_counts(
    channels: &[ChannelSupportCard],
    subjects: &[CompatibilitySubjectCard],
    consumers: &[SupportConsumerRow],
) -> SupportCounts {
    let count_channels =
        |gate: DescriptorGate| channels.iter().filter(|c| c.gate == gate).count() as u32;
    let count_subjects =
        |gate: DescriptorGate| subjects.iter().filter(|c| c.gate == gate).count() as u32;
    let deprecated_or_eos = channels
        .iter()
        .filter(|c| c.gate != DescriptorGate::Governed)
        .count()
        + subjects
            .iter()
            .filter(|c| c.gate != DescriptorGate::Governed)
            .count();
    let action_required_consumers =
        consumers.iter().filter(|c| c.is_action_required()).count() as u32;
    SupportCounts {
        total_channels: channels.len() as u32,
        supported_channels: count_channels(DescriptorGate::Governed),
        plan_migration_channels: count_channels(DescriptorGate::Narrowed),
        action_required_channels: count_channels(DescriptorGate::Blocked),
        total_subjects: subjects.len() as u32,
        supported_subjects: count_subjects(DescriptorGate::Governed),
        plan_migration_subjects: count_subjects(DescriptorGate::Narrowed),
        action_required_subjects: count_subjects(DescriptorGate::Blocked),
        deprecated_or_eos_cards: deprecated_or_eos as u32,
        total_consumers: consumers.len() as u32,
        supported_consumers: consumers.iter().filter(|c| c.is_supported()).count() as u32,
        plan_migration_consumers: consumers.iter().filter(|c| c.is_plan_migration()).count() as u32,
        action_required_consumers,
        requires_migration_action: action_required_consumers > 0,
    }
}

fn derive_coverage(
    channels: &[ChannelSupportCard],
    subjects: &[CompatibilitySubjectCard],
    data_state: StaleDataBehavior,
) -> SupportCoverage {
    let count = |gate: DescriptorGate| {
        channels.iter().filter(|c| c.gate == gate).count() as u32
            + subjects.iter().filter(|c| c.gate == gate).count() as u32
    };
    let narrowing = count(DescriptorGate::Narrowed);
    let blocking = count(DescriptorGate::Blocked);
    SupportCoverage {
        fully_supported_cards: count(DescriptorGate::Governed),
        narrowing_cards: narrowing,
        blocking_cards: blocking,
        has_lifecycle_pressure: narrowing > 0 || blocking > 0,
        data_state,
        live_data: data_state == StaleDataBehavior::LiveVerified,
    }
}

fn derive_release_gate(consumers: &[SupportConsumerRow]) -> SupportReleaseGate {
    let collect = |pred: fn(&SupportConsumerRow) -> bool| -> Vec<String> {
        consumers
            .iter()
            .filter(|c| pred(c))
            .map(|c| c.consumer.as_str().to_owned())
            .collect()
    };
    let mut affected_channels: Vec<String> = Vec::new();
    let mut affected_subjects: Vec<String> = Vec::new();
    for consumer in consumers {
        for gap in &consumer.gaps {
            match gap.target_kind {
                SupportTargetKind::Channel => affected_channels.push(gap.target_token.clone()),
                SupportTargetKind::Subject => affected_subjects.push(gap.target_token.clone()),
            }
        }
    }
    affected_channels.sort();
    affected_channels.dedup();
    affected_subjects.sort();
    affected_subjects.dedup();
    let action = collect(SupportConsumerRow::is_action_required);
    SupportReleaseGate {
        requires_migration_action: !action.is_empty(),
        action_required_consumers: action,
        plan_migration_consumers: collect(SupportConsumerRow::is_plan_migration),
        supported_consumers: collect(SupportConsumerRow::is_supported),
        affected_channels,
        affected_subjects,
        gate_message_id: format!("{}release_gate", M5_SUPPORT_WINDOW_MESSAGE_ID_PREFIX),
    }
}

/// Scans the export for forbidden raw material (credential bodies / raw provider payloads).
fn contains_forbidden_material(packet: &SupportWindowCardSet) -> bool {
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
