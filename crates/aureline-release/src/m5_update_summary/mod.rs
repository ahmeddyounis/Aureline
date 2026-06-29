//! Typed update-center summary objects — the concrete per-artifact-family update rows the update
//! center, release center, and Help/About panel inspect before and after a download, on top of the
//! [update / support-lifecycle governance matrix](crate::m5_update_lifecycle).
//!
//! The governance matrix freezes *which* lifecycle states and proof paths govern an update surface.
//! This lane is the layer below it: the actual update summaries a user reads. Each
//! [artifact family](ArtifactFamily) — desktop app, extension, docs pack, policy bundle, framework
//! pack, runtime / toolchain — gets its own [summary entry](UpdateSummaryEntry) rather than being
//! flattened into one generic "an update is available" row, so the user can see exactly which
//! artifact classes are changing, whether each verified cleanly, what restart it costs, and what
//! rollback path still exists.
//!
//! Every entry carries:
//!
//! - the current and target version and an [update posture](UpdatePosture)
//!   (available / downloaded / staged / applied / failed);
//! - one [artifact-class delta row](ArtifactDeltaRow) per artifact class the update touches, each
//!   with its own change kind, version delta, [verification state](VerificationState), and
//!   [restart impact](RestartImpact) — the entry's disclosed artifact classes are derived from those
//!   rows so an update can never hide a class it changes behind a generic desktop-app row;
//! - a [rollback availability](RollbackAvailability) that distinguishes a true version rollback from
//!   a side-by-side fallback or a reinstall-only path, so the summary never implies rollback exists
//!   when only reinstall remains; and
//! - a [release-data state](ReleaseDataState) that labels mirrored / offline / stale / not-provided
//!   live-release data honestly instead of letting it masquerade as live channel truth.
//!
//! The [consumer surfaces](SummaryConsumer) — release center, update center, Help/About — bind the
//! families they read and *derive* their effective qualification, gate decision, and coverage gaps
//! from those entries, so all three read the one [`M5UpdateCenterSummary`] packet rather than cloning
//! version / verification / rollback fields locally. A stale or mirrored entry narrows every consumer
//! that reads it; a not-provided (no live data) or verification-failed entry blocks Stable promotion,
//! with the gap named per family rather than implied.
//!
//! The packet is inspectable and serde-serializable; it carries metadata and refs only — no
//! credential bodies or raw provider payloads.
//!
//! - Packet schema:
//!   [`schemas/release/m5-update-center-summary-object.schema.json`](../../../../../schemas/release/m5-update-center-summary-object.schema.json)
//! - Delta-row schema:
//!   [`schemas/release/m5-artifact-delta-row.schema.json`](../../../../../schemas/release/m5-artifact-delta-row.schema.json)
//! - Contract doc:
//!   [`docs/release/m5-update-center-summary-contract.md`](../../../../../docs/release/m5-update-center-summary-contract.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_update_center_summary, seeded_m5_update_center_summary_not_provided_blocked,
    seeded_m5_update_center_summary_stale_data_narrowed, M5_UPDATE_CENTER_SUMMARY_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The summary objects reuse the governance matrix's frozen artifact-class, channel, and profile
// vocabularies, and the descriptor / badge runtime's gate / status / signal vocabulary, so the
// concrete object layer can never drift to a different vocabulary than the governance layer above it.
use crate::m5_descriptor_badge::{
    ConsumerStatus, DescriptorGate, DescriptorSignal, QualificationClass,
};
use crate::m5_update_lifecycle::{ArtifactClass, ChannelScope, DeploymentProfile};

/// Record-kind tag carried by [`M5UpdateCenterSummary`].
pub const M5_UPDATE_CENTER_SUMMARY_RECORD_KIND: &str = "m5_update_center_summary";

/// Schema version for the update-center summary packet.
pub const M5_UPDATE_CENTER_SUMMARY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the summary packet schema.
pub const M5_UPDATE_CENTER_SUMMARY_SCHEMA_REF: &str =
    "schemas/release/m5-update-center-summary-object.schema.json";

/// Repo-relative path of the artifact-class delta-row schema.
pub const M5_ARTIFACT_DELTA_ROW_SCHEMA_REF: &str =
    "schemas/release/m5-artifact-delta-row.schema.json";

/// Repo-relative path of the published summary inventory.
pub const M5_UPDATE_CENTER_SUMMARY_REF: &str = "artifacts/release/m5-update-center-summary.json";

/// Repo-relative path of the release-grade summary parity proof.
pub const M5_UPDATE_CENTER_SUMMARY_PROOF_REF: &str =
    "artifacts/release/m5-update-center-summary-proof/update-center-summary.json";

/// Repo-relative path of the machine-readable artifact-class delta export.
pub const M5_UPDATE_CENTER_SUMMARY_DELTA_CSV_REF: &str =
    "artifacts/release/m5-update-center-summary-delta.csv";

/// Repo-relative path of the summary contract doc.
pub const M5_UPDATE_CENTER_SUMMARY_DOC_REF: &str =
    "docs/release/m5-update-center-summary-contract.md";

/// Repo-relative directory of the per-state summary fixtures.
pub const M5_UPDATE_CENTER_SUMMARY_FIXTURE_DIR: &str = "fixtures/release/update-center-summary/";

/// Prefix every update-summary message id carries so consumers can route it.
pub const M5_UPDATE_CENTER_SUMMARY_MESSAGE_ID_PREFIX: &str = "release_update_summary.";

const REDACTION_CLASS: &str = "metadata_safe_default";

// ---------------------------------------------------------------------------
// Controlled vocabularies
// ---------------------------------------------------------------------------

/// One claimed update artifact family. Each family is summarized in its own row so an update never
/// flattens unrelated artifact classes into one generic desktop-app update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactFamily {
    /// The desktop application build.
    DesktopApp,
    /// An installed extension / plugin pack.
    Extension,
    /// A docs / help content pack.
    DocsPack,
    /// An administrator policy bundle.
    PolicyBundle,
    /// A project framework / scaffold pack.
    FrameworkPack,
    /// A language runtime / toolchain asset.
    RuntimeToolchain,
}

impl ArtifactFamily {
    /// Every artifact family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopApp,
        Self::Extension,
        Self::DocsPack,
        Self::PolicyBundle,
        Self::FrameworkPack,
        Self::RuntimeToolchain,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopApp => "desktop_app",
            Self::Extension => "extension",
            Self::DocsPack => "docs_pack",
            Self::PolicyBundle => "policy_bundle",
            Self::FrameworkPack => "framework_pack",
            Self::RuntimeToolchain => "runtime_toolchain",
        }
    }

    /// Human-facing label for the family.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DesktopApp => "Desktop app",
            Self::Extension => "Extension",
            Self::DocsPack => "Docs pack",
            Self::PolicyBundle => "Policy bundle",
            Self::FrameworkPack => "Framework pack",
            Self::RuntimeToolchain => "Runtime / toolchain",
        }
    }

    /// The primary artifact class this family installs. Delta rows may touch other classes too; the
    /// disclosed set is always the union of the delta rows, never just this primary class.
    pub const fn primary_artifact_class(self) -> ArtifactClass {
        match self {
            Self::DesktopApp => ArtifactClass::CoreRuntime,
            Self::Extension => ArtifactClass::ExtensionPacks,
            Self::DocsPack => ArtifactClass::DocsHelpContent,
            Self::PolicyBundle => ArtifactClass::Configuration,
            Self::FrameworkPack => ArtifactClass::SchemaContracts,
            Self::RuntimeToolchain => ArtifactClass::LanguageRuntimes,
        }
    }

    /// Accountable owner role for this family's update truth.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::DesktopApp => "desktop_release_owner",
            Self::Extension => "extension_release_owner",
            Self::DocsPack => "docs_release_owner",
            Self::PolicyBundle => "policy_governance_owner",
            Self::FrameworkPack => "framework_pack_owner",
            Self::RuntimeToolchain => "toolchain_release_owner",
        }
    }
}

/// The staged / downloaded / applied posture of an update for one family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePosture {
    /// The installed version equals the target; nothing pending.
    UpToDate,
    /// An update is offered but not yet downloaded.
    Available,
    /// The update is downloaded but not staged.
    Downloaded,
    /// The update is downloaded and staged for the next apply.
    Staged,
    /// The update has been applied and is active.
    Applied,
    /// The most recent download or apply failed.
    Failed,
}

impl UpdatePosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UpToDate,
        Self::Available,
        Self::Downloaded,
        Self::Staged,
        Self::Applied,
        Self::Failed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpToDate => "up_to_date",
            Self::Available => "available",
            Self::Downloaded => "downloaded",
            Self::Staged => "staged",
            Self::Applied => "applied",
            Self::Failed => "failed",
        }
    }
}

/// Per-artifact signature / provenance verification state. Declaration order is best→worst.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationState {
    /// Signature and provenance verified cleanly.
    Verified,
    /// Not yet verified (e.g. offered but not downloaded); narrows the claim.
    Pending,
    /// Present but could not be verified (e.g. mirror without a signature); narrows the claim.
    Unverified,
    /// Verification failed; blocks Stable promotion.
    Failed,
    /// No signature / provenance evidence is provided; blocks Stable promotion.
    NotProvided,
}

impl VerificationState {
    /// Every verification state, best→worst.
    pub const ALL: [Self; 5] = [
        Self::Verified,
        Self::Pending,
        Self::Unverified,
        Self::Failed,
        Self::NotProvided,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Pending => "pending",
            Self::Unverified => "unverified",
            Self::Failed => "failed",
            Self::NotProvided => "not_provided",
        }
    }

    /// Severity rank; higher is worse.
    const fn rank(self) -> u8 {
        match self {
            Self::Verified => 0,
            Self::Pending => 1,
            Self::Unverified => 2,
            Self::Failed => 3,
            Self::NotProvided => 4,
        }
    }

    /// The gate this verification state implies on its own.
    pub const fn gate(self) -> DescriptorGate {
        match self {
            Self::Verified => DescriptorGate::Governed,
            Self::Pending | Self::Unverified => DescriptorGate::Narrowed,
            Self::Failed | Self::NotProvided => DescriptorGate::Blocked,
        }
    }
}

/// The restart cost of applying an update. Declaration order is least→most disruptive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartImpact {
    /// Hot-applied; no restart needed.
    None,
    /// Requires reloading the editor window.
    ReloadWindow,
    /// Requires restarting the application.
    RestartApp,
    /// Requires restarting the host machine.
    RestartHost,
}

impl RestartImpact {
    /// Every restart impact, least→most disruptive.
    pub const ALL: [Self; 4] = [
        Self::None,
        Self::ReloadWindow,
        Self::RestartApp,
        Self::RestartHost,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ReloadWindow => "reload_window",
            Self::RestartApp => "restart_app",
            Self::RestartHost => "restart_host",
        }
    }

    /// Severity rank; higher is more disruptive.
    const fn rank(self) -> u8 {
        match self {
            Self::None => 0,
            Self::ReloadWindow => 1,
            Self::RestartApp => 2,
            Self::RestartHost => 3,
        }
    }
}

/// The rollback path available after an update. The kinds are deliberately distinct so the summary
/// never implies a true version rollback exists when only a side-by-side fallback or a reinstall of
/// the prior version remains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackAvailability {
    /// A true rollback to the prior version is supported.
    RollbackSupported,
    /// The prior version coexists; the user can fall back side-by-side without a true rollback.
    SideBySideFallback,
    /// No rollback; recovering the prior version requires a reinstall.
    ReinstallOnly,
    /// No rollback and no prior-version fallback.
    NoRollback,
}

impl RollbackAvailability {
    /// Every rollback availability, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RollbackSupported,
        Self::SideBySideFallback,
        Self::ReinstallOnly,
        Self::NoRollback,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RollbackSupported => "rollback_supported",
            Self::SideBySideFallback => "side_by_side_fallback",
            Self::ReinstallOnly => "reinstall_only",
            Self::NoRollback => "no_rollback",
        }
    }

    /// True only for a genuine version rollback. Side-by-side and reinstall-only deliberately return
    /// false so the summary cannot present them as rollback.
    pub const fn is_true_rollback(self) -> bool {
        matches!(self, Self::RollbackSupported)
    }
}

/// How current the live release data backing this summary actually is. Mirrored / offline / stale /
/// not-provided states are labeled explicitly so weaker data never masquerades as live channel truth.
/// Declaration order is most→least authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseDataState {
    /// Live, freshly fetched from the release channel.
    Live,
    /// Served from a labeled mirror; still usable, not presented as live.
    Mirrored,
    /// Served from an offline cache; still usable, not presented as live.
    Offline,
    /// Cached data has aged past its freshness window; narrows the claim.
    Stale,
    /// No live release data is available at all; blocks Stable promotion.
    NotProvided,
}

impl ReleaseDataState {
    /// Every release-data state, most→least authoritative.
    pub const ALL: [Self; 5] = [
        Self::Live,
        Self::Mirrored,
        Self::Offline,
        Self::Stale,
        Self::NotProvided,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Mirrored => "mirrored",
            Self::Offline => "offline",
            Self::Stale => "stale",
            Self::NotProvided => "not_provided",
        }
    }

    /// Severity rank; higher is less authoritative.
    const fn rank(self) -> u8 {
        match self {
            Self::Live => 0,
            Self::Mirrored => 1,
            Self::Offline => 2,
            Self::Stale => 3,
            Self::NotProvided => 4,
        }
    }

    /// True only for live channel data.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::Live)
    }

    /// The gate this data state implies on its own. Mirrored / offline data stays usable (governed)
    /// because it is labeled, not hidden; stale data narrows; no live data blocks.
    pub const fn gate(self) -> DescriptorGate {
        match self {
            Self::Live | Self::Mirrored | Self::Offline => DescriptorGate::Governed,
            Self::Stale => DescriptorGate::Narrowed,
            Self::NotProvided => DescriptorGate::Blocked,
        }
    }
}

/// The change a delta row records for one artifact class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeltaChangeKind {
    /// The class is newly added by this update.
    Added,
    /// The class is updated from one version to another.
    Updated,
    /// The class is removed by this update.
    Removed,
    /// The class is carried unchanged.
    Unchanged,
}

impl DeltaChangeKind {
    /// Every change kind, in declaration order.
    pub const ALL: [Self; 4] = [Self::Added, Self::Updated, Self::Removed, Self::Unchanged];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Updated => "updated",
            Self::Removed => "removed",
            Self::Unchanged => "unchanged",
        }
    }
}

/// The named cause of a consumer's coverage gap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SummaryGapKind {
    /// An update is not yet verified.
    VerificationPending,
    /// An update is present but could not be verified.
    VerificationUnverified,
    /// An update failed verification.
    VerificationFailed,
    /// The backing release data is offline / mirrored beyond its freshness window (stale).
    DataStale,
    /// No live release data is available for this family.
    DataNotProvided,
}

impl SummaryGapKind {
    /// Every gap kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::VerificationPending,
        Self::VerificationUnverified,
        Self::VerificationFailed,
        Self::DataStale,
        Self::DataNotProvided,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerificationPending => "verification_pending",
            Self::VerificationUnverified => "verification_unverified",
            Self::VerificationFailed => "verification_failed",
            Self::DataStale => "data_stale",
            Self::DataNotProvided => "data_not_provided",
        }
    }

    /// The gate this gap forces.
    const fn gate(self) -> DescriptorGate {
        match self {
            Self::VerificationPending | Self::VerificationUnverified | Self::DataStale => {
                DescriptorGate::Narrowed
            }
            Self::VerificationFailed | Self::DataNotProvided => DescriptorGate::Blocked,
        }
    }
}

/// One claimed consumer surface that reads the summary objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SummaryConsumer {
    /// The release center / public-truth automation.
    ReleaseCenter,
    /// The in-product update center.
    UpdateCenter,
    /// The Help/About panel.
    HelpAbout,
}

impl SummaryConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 3] = [Self::ReleaseCenter, Self::UpdateCenter, Self::HelpAbout];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::UpdateCenter => "update_center",
            Self::HelpAbout => "help_about",
        }
    }

    /// Human-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "Release center",
            Self::UpdateCenter => "Update center",
            Self::HelpAbout => "Help / About",
        }
    }

    /// Accountable owner role.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center_owner",
            Self::UpdateCenter => "update_center_owner",
            Self::HelpAbout => "help_about_owner",
        }
    }
}

// ---------------------------------------------------------------------------
// Ranking helpers for deterministic ordering
// ---------------------------------------------------------------------------

fn family_rank(f: ArtifactFamily) -> usize {
    ArtifactFamily::ALL
        .iter()
        .position(|x| *x == f)
        .unwrap_or(usize::MAX)
}

fn artifact_rank(c: ArtifactClass) -> usize {
    ArtifactClass::ALL
        .iter()
        .position(|x| *x == c)
        .unwrap_or(usize::MAX)
}

fn channel_rank(c: ChannelScope) -> usize {
    ChannelScope::ALL
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

// ---------------------------------------------------------------------------
// Artifact-class delta row
// ---------------------------------------------------------------------------

/// One artifact-class delta row inside an [`UpdateSummaryEntry`]: the change, version delta,
/// verification state, restart impact, and release-data state for a single artifact class an update
/// touches. The entry's disclosed artifact classes and gate are derived from its rows, so no class an
/// update changes can be omitted from the summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactDeltaRow {
    /// The artifact class this row changes.
    pub artifact_class: ArtifactClass,
    /// Human-facing label for the artifact class.
    pub artifact_class_label: String,
    /// The change recorded for the class.
    pub change_kind: DeltaChangeKind,
    /// The version the class moves from (absent for an added class).
    pub from_version: Option<String>,
    /// The version the class moves to (absent for a removed class).
    pub to_version: Option<String>,
    /// Signature / provenance verification for this class.
    pub verification_state: VerificationState,
    /// Restart cost contributed by this class.
    pub restart_impact: RestartImpact,
    /// Liveness of the release data backing this class.
    pub release_data_state: ReleaseDataState,
    /// Gate this row contributes, derived from its verification and data state.
    pub gate: DescriptorGate,
    /// Traffic-light signal mirroring [`gate`](Self::gate).
    pub signal: DescriptorSignal,
    /// Routable message id for this row's detail.
    pub detail_message_id: String,
}

impl ArtifactDeltaRow {
    /// Builds a delta row, deriving its gate and signal from the verification and data state.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        family: ArtifactFamily,
        artifact_class: ArtifactClass,
        change_kind: DeltaChangeKind,
        from_version: Option<&str>,
        to_version: Option<&str>,
        verification_state: VerificationState,
        restart_impact: RestartImpact,
        release_data_state: ReleaseDataState,
    ) -> Self {
        let gate = worst_gate(verification_state.gate(), release_data_state.gate());
        Self {
            artifact_class,
            artifact_class_label: artifact_class.as_str().replace('_', " "),
            change_kind,
            from_version: from_version.map(str::to_owned),
            to_version: to_version.map(str::to_owned),
            verification_state,
            restart_impact,
            release_data_state,
            gate,
            signal: signal_for_gate(gate),
            detail_message_id: format!(
                "{}delta.{}.{}.{}",
                M5_UPDATE_CENTER_SUMMARY_MESSAGE_ID_PREFIX,
                family.as_str(),
                artifact_class.as_str(),
                change_kind.as_str()
            ),
        }
    }

    /// Recomputes the derived gate and signal from this row's inputs.
    fn recompute(&mut self) {
        self.gate = worst_gate(
            self.verification_state.gate(),
            self.release_data_state.gate(),
        );
        self.signal = signal_for_gate(self.gate);
    }
}

fn signal_for_gate(gate: DescriptorGate) -> DescriptorSignal {
    match gate {
        DescriptorGate::Governed => DescriptorSignal::Green,
        DescriptorGate::Narrowed => DescriptorSignal::Yellow,
        DescriptorGate::Blocked => DescriptorSignal::Red,
    }
}

// ---------------------------------------------------------------------------
// Update summary entry (per family)
// ---------------------------------------------------------------------------

/// The typed update summary for one [artifact family](ArtifactFamily): current and target version,
/// posture, the rolled-up verification / restart / rollback / release-data truth, and the
/// [delta rows](ArtifactDeltaRow) the roll-ups derive from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateSummaryEntry {
    /// The artifact family this entry summarizes.
    pub family: ArtifactFamily,
    /// Human-facing label for the family.
    pub family_label: String,
    /// The family's primary artifact class.
    pub primary_artifact_class: ArtifactClass,
    /// Accountable owner role.
    pub owner_role: String,
    /// The channel this summary reflects.
    pub channel: ChannelScope,
    /// The deployment profiles this family covers.
    pub profiles: Vec<DeploymentProfile>,
    /// Installed version.
    pub current_version: String,
    /// Target version the update moves to (equals [`current_version`](Self::current_version) when up
    /// to date).
    pub target_version: String,
    /// True when an update is available (current and target differ).
    pub update_available: bool,
    /// The staged / downloaded / applied posture.
    pub posture: UpdatePosture,
    /// Worst verification state across the delta rows.
    pub verification_state: VerificationState,
    /// Worst restart impact across the delta rows.
    pub restart_impact: RestartImpact,
    /// The rollback path this update offers.
    pub rollback: RollbackAvailability,
    /// True only when [`rollback`](Self::rollback) is a genuine version rollback; side-by-side and
    /// reinstall-only resolve to false so the summary never overclaims rollback.
    pub rollback_disclosed: bool,
    /// Worst release-data state across the delta rows.
    pub release_data_state: ReleaseDataState,
    /// The artifact-class delta rows this update touches.
    pub delta_rows: Vec<ArtifactDeltaRow>,
    /// The union of artifact classes the delta rows change (always includes the primary class).
    pub affected_artifact_classes: Vec<ArtifactClass>,
    /// Gate derived from the rolled-up verification and data state.
    pub gate: DescriptorGate,
    /// Coverage status mirroring [`gate`](Self::gate).
    pub status: ConsumerStatus,
    /// Traffic-light signal mirroring [`gate`](Self::gate).
    pub signal: DescriptorSignal,
    /// True only when the update is verified and not blocked, i.e. safe to apply now.
    pub apply_ready: bool,
    /// Routable message id for this entry's summary line.
    pub summary_message_id: String,
    /// Routable message id for this entry's detail.
    pub detail_message_id: String,
}

impl UpdateSummaryEntry {
    /// Builds an entry from its family-level fields and delta rows, deriving the rolled-up
    /// verification / restart / release-data state, the disclosed artifact classes, and the gate.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        family: ArtifactFamily,
        channel: ChannelScope,
        profiles: &[DeploymentProfile],
        current_version: &str,
        target_version: &str,
        posture: UpdatePosture,
        rollback: RollbackAvailability,
        delta_rows: Vec<ArtifactDeltaRow>,
    ) -> Self {
        let mut entry = Self {
            family,
            family_label: family.label().to_owned(),
            primary_artifact_class: family.primary_artifact_class(),
            owner_role: family.owner_role().to_owned(),
            channel,
            profiles: profiles.to_vec(),
            current_version: current_version.to_owned(),
            target_version: target_version.to_owned(),
            update_available: current_version != target_version,
            posture,
            verification_state: VerificationState::Verified,
            restart_impact: RestartImpact::None,
            rollback,
            rollback_disclosed: rollback.is_true_rollback(),
            release_data_state: ReleaseDataState::Live,
            delta_rows,
            affected_artifact_classes: Vec::new(),
            gate: DescriptorGate::Governed,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            apply_ready: true,
            summary_message_id: format!(
                "{}entry.{}.summary",
                M5_UPDATE_CENTER_SUMMARY_MESSAGE_ID_PREFIX,
                family.as_str()
            ),
            detail_message_id: format!(
                "{}entry.{}.detail",
                M5_UPDATE_CENTER_SUMMARY_MESSAGE_ID_PREFIX,
                family.as_str()
            ),
        };
        entry.recompute();
        entry
    }

    /// Recomputes the roll-ups and derived verdict from the delta rows. The entry's verification,
    /// restart, and release-data state are the worst across its rows; the disclosed artifact classes
    /// are the union of the rows; the gate is the worst of the rolled-up verification and data gate.
    pub fn recompute(&mut self) {
        let mut profiles = self.profiles.clone();
        profiles.sort_by_key(|p| profile_rank(*p));
        profiles.dedup();
        self.profiles = profiles;

        for row in &mut self.delta_rows {
            row.recompute();
        }

        self.update_available = self.current_version != self.target_version;
        self.rollback_disclosed = self.rollback.is_true_rollback();

        let mut verification = VerificationState::Verified;
        let mut restart = RestartImpact::None;
        let mut data = ReleaseDataState::Live;
        let mut classes: Vec<ArtifactClass> = vec![self.primary_artifact_class];
        for row in &self.delta_rows {
            if row.verification_state.rank() > verification.rank() {
                verification = row.verification_state;
            }
            if row.restart_impact.rank() > restart.rank() {
                restart = row.restart_impact;
            }
            if row.release_data_state.rank() > data.rank() {
                data = row.release_data_state;
            }
            classes.push(row.artifact_class);
        }
        classes.sort_by_key(|c| artifact_rank(*c));
        classes.dedup();

        self.verification_state = verification;
        self.restart_impact = restart;
        self.release_data_state = data;
        self.affected_artifact_classes = classes;

        let gate = worst_gate(verification.gate(), data.gate());
        self.gate = gate;
        self.status = status_for_gate(gate);
        self.signal = signal_for_gate(gate);
        self.apply_ready = self.update_available
            && gate != DescriptorGate::Blocked
            && matches!(verification, VerificationState::Verified);
    }

    /// The gap kinds this entry contributes to a consumer that reads it.
    fn gap_kinds(&self) -> Vec<SummaryGapKind> {
        let mut kinds = Vec::new();
        match self.verification_state {
            VerificationState::Verified => {}
            VerificationState::Pending => kinds.push(SummaryGapKind::VerificationPending),
            VerificationState::Unverified => kinds.push(SummaryGapKind::VerificationUnverified),
            VerificationState::Failed | VerificationState::NotProvided => {
                kinds.push(SummaryGapKind::VerificationFailed)
            }
        }
        match self.release_data_state {
            ReleaseDataState::Live | ReleaseDataState::Mirrored | ReleaseDataState::Offline => {}
            ReleaseDataState::Stale => kinds.push(SummaryGapKind::DataStale),
            ReleaseDataState::NotProvided => kinds.push(SummaryGapKind::DataNotProvided),
        }
        kinds
    }
}

// ---------------------------------------------------------------------------
// Consumer rows
// ---------------------------------------------------------------------------

/// A coverage gap a consumer carries for one family it reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryGap {
    /// The consumer that carries the gap.
    pub consumer: SummaryConsumer,
    /// The family whose entry caused the gap.
    pub family: ArtifactFamily,
    /// The family's primary artifact class.
    pub artifact_class: ArtifactClass,
    /// The named cause of the gap.
    pub gap_kind: SummaryGapKind,
    /// Routable message id naming the cause.
    pub cause_message_id: String,
}

/// A consumer surface bound to the families it reads, with its effective qualification, gate, and
/// coverage gaps derived from those families' entries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryConsumerRow {
    /// The consumer surface.
    pub consumer: SummaryConsumer,
    /// Human-facing label.
    pub consumer_label: String,
    /// Accountable owner role.
    pub owner_role: String,
    /// The qualification the consumer claims when every entry it reads is governed.
    pub claimed_qualification: QualificationClass,
    /// The families this consumer reads.
    pub read_families: Vec<ArtifactFamily>,
    /// The union of artifact classes disclosed across the read families.
    pub disclosed_artifact_classes: Vec<ArtifactClass>,
    /// The union of channels across the read families.
    pub channels: Vec<ChannelScope>,
    /// The union of profiles across the read families.
    pub profiles: Vec<DeploymentProfile>,
    /// The qualification after applying the gaps.
    pub effective_qualification: QualificationClass,
    /// Coverage status.
    pub status: ConsumerStatus,
    /// Traffic-light signal.
    pub signal: DescriptorSignal,
    /// Gate decision.
    pub gate_decision: DescriptorGate,
    /// Coverage gaps, one per (family, cause).
    pub gaps: Vec<SummaryGap>,
    /// Routable status message id.
    pub status_message_id: String,
    /// Routable gate message id.
    pub gate_message_id: String,
}

impl SummaryConsumerRow {
    /// Builds a consumer row; the resolved unions, gaps, and verdict are recomputed against the
    /// packet's entries when the packet is assembled.
    pub fn new(
        consumer: SummaryConsumer,
        claimed_qualification: QualificationClass,
        read_families: &[ArtifactFamily],
    ) -> Self {
        Self {
            consumer,
            consumer_label: consumer.label().to_owned(),
            owner_role: consumer.owner_role().to_owned(),
            claimed_qualification,
            read_families: read_families.to_vec(),
            disclosed_artifact_classes: Vec::new(),
            channels: Vec::new(),
            profiles: Vec::new(),
            effective_qualification: claimed_qualification,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            gate_decision: DescriptorGate::Governed,
            gaps: Vec::new(),
            status_message_id: format!(
                "{}consumer.{}.status",
                M5_UPDATE_CENTER_SUMMARY_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
            gate_message_id: format!(
                "{}consumer.{}.gate",
                M5_UPDATE_CENTER_SUMMARY_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
        }
    }

    /// Recomputes the resolved unions, gaps, and verdict from the packet's entries, so a consumer's
    /// claim is always generated from the same checked-in summary objects rather than a
    /// hand-maintained status.
    pub fn recompute(&mut self, entries: &[UpdateSummaryEntry]) {
        let mut read = self.read_families.clone();
        read.sort_by_key(|f| family_rank(*f));
        read.dedup();
        self.read_families = read.clone();

        let entry_for = |family: ArtifactFamily| -> Option<&UpdateSummaryEntry> {
            entries.iter().find(|e| e.family == family)
        };

        let mut classes: Vec<ArtifactClass> = Vec::new();
        let mut channels: Vec<ChannelScope> = Vec::new();
        let mut profiles: Vec<DeploymentProfile> = Vec::new();
        let mut gaps: Vec<SummaryGap> = Vec::new();
        let consumer = self.consumer;
        for &family in &read {
            match entry_for(family) {
                None => {
                    gaps.push(SummaryGap {
                        consumer,
                        family,
                        artifact_class: family.primary_artifact_class(),
                        gap_kind: SummaryGapKind::DataNotProvided,
                        cause_message_id: format!(
                            "{}consumer.{}.{}.{}.gap",
                            M5_UPDATE_CENTER_SUMMARY_MESSAGE_ID_PREFIX,
                            consumer.as_str(),
                            family.as_str(),
                            SummaryGapKind::DataNotProvided.as_str()
                        ),
                    });
                }
                Some(entry) => {
                    classes.extend(entry.affected_artifact_classes.iter().copied());
                    channels.push(entry.channel);
                    profiles.extend(entry.profiles.iter().copied());
                    for kind in entry.gap_kinds() {
                        gaps.push(SummaryGap {
                            consumer,
                            family,
                            artifact_class: family.primary_artifact_class(),
                            gap_kind: kind,
                            cause_message_id: format!(
                                "{}consumer.{}.{}.{}.gap",
                                M5_UPDATE_CENTER_SUMMARY_MESSAGE_ID_PREFIX,
                                consumer.as_str(),
                                family.as_str(),
                                kind.as_str()
                            ),
                        });
                    }
                }
            }
        }
        classes.sort_by_key(|c| artifact_rank(*c));
        classes.dedup();
        channels.sort_by_key(|c| channel_rank(*c));
        channels.dedup();
        profiles.sort_by_key(|p| profile_rank(*p));
        profiles.dedup();
        gaps.sort_by(|a, b| {
            family_rank(a.family)
                .cmp(&family_rank(b.family))
                .then(a.gap_kind.as_str().cmp(b.gap_kind.as_str()))
        });

        self.disclosed_artifact_classes = classes;
        self.channels = channels;
        self.profiles = profiles;
        self.gaps = gaps;

        let mut gate = DescriptorGate::Governed;
        for gap in &self.gaps {
            gate = worst_gate(gate, gap.gap_kind.gate());
        }
        self.gate_decision = gate;
        self.status = status_for_gate(gate);
        self.signal = signal_for_gate(gate);
        self.effective_qualification = match gate {
            DescriptorGate::Governed => self.claimed_qualification,
            DescriptorGate::Narrowed => narrow_to_beta(self.claimed_qualification),
            DescriptorGate::Blocked => QualificationClass::Unavailable,
        };
    }

    /// True when the consumer reads every entry as governed.
    pub fn is_certified(&self) -> bool {
        self.gate_decision == DescriptorGate::Governed
    }

    /// True when at least one read entry narrows the consumer below Stable.
    pub fn is_narrowed(&self) -> bool {
        self.gate_decision == DescriptorGate::Narrowed
    }

    /// True when at least one read entry blocks the consumer from Stable promotion.
    pub fn is_blocked(&self) -> bool {
        self.gate_decision == DescriptorGate::Blocked
    }
}

fn narrow_to_beta(claimed: QualificationClass) -> QualificationClass {
    if (claimed as usize) <= (QualificationClass::Beta as usize) {
        QualificationClass::Beta
    } else {
        claimed
    }
}

// ---------------------------------------------------------------------------
// Aggregate sub-objects
// ---------------------------------------------------------------------------

/// Disclosure flags asserting every claimed consumer ingests this one summary packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryDisclosure {
    /// The release center consumes the summary packet.
    pub release_center_consumes_summary: bool,
    /// The update center consumes the summary packet.
    pub update_center_consumes_summary: bool,
    /// The Help/About panel consumes the summary packet.
    pub help_about_consumes_summary: bool,
}

impl SummaryDisclosure {
    fn canonical() -> Self {
        Self {
            release_center_consumes_summary: true,
            update_center_consumes_summary: true,
            help_about_consumes_summary: true,
        }
    }

    /// True when every consumer is asserted to consume the summary.
    pub fn all_consume(&self) -> bool {
        self.release_center_consumes_summary
            && self.update_center_consumes_summary
            && self.help_about_consumes_summary
    }
}

/// Roll-up counts over the entries and consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryCounts {
    /// Total summary entries.
    pub total_entries: u32,
    /// Entries with an update available.
    pub update_available_count: u32,
    /// Entries already up to date.
    pub up_to_date_count: u32,
    /// Entries whose gate is governed.
    pub governed_entries: u32,
    /// Entries whose gate is narrowed.
    pub narrowed_entries: u32,
    /// Entries whose gate is blocked.
    pub blocked_entries: u32,
    /// Total artifact-class delta rows across all entries.
    pub total_delta_rows: u32,
    /// Total consumers.
    pub total_consumers: u32,
    /// Consumers certified at their claimed qualification.
    pub certified_consumer_count: u32,
    /// Consumers narrowed below Stable.
    pub narrowed_consumer_count: u32,
    /// Consumers blocked from Stable promotion.
    pub blocked_consumer_count: u32,
    /// Whether the packet blocks Stable promotion.
    pub blocks_stable_promotion: bool,
}

/// The packet-level release gate aggregating the per-consumer gates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryReleaseGate {
    /// Whether any consumer is blocked.
    pub blocks_stable_promotion: bool,
    /// Tokens of the blocked consumers.
    pub blocked_consumers: Vec<String>,
    /// Tokens of the narrowed consumers.
    pub narrowed_consumers: Vec<String>,
    /// Tokens of the certified consumers.
    pub certified_consumers: Vec<String>,
    /// Tokens of the families that contributed a gap.
    pub affected_families: Vec<String>,
    /// Routable gate message id.
    pub gate_message_id: String,
}

/// The frozen controlled vocabulary the summary objects draw from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryVocabulary {
    /// Artifact-family tokens.
    pub families: Vec<String>,
    /// Artifact-class tokens.
    pub artifact_classes: Vec<String>,
    /// Update-posture tokens.
    pub postures: Vec<String>,
    /// Verification-state tokens.
    pub verification_states: Vec<String>,
    /// Restart-impact tokens.
    pub restart_impacts: Vec<String>,
    /// Rollback-availability tokens.
    pub rollback_availabilities: Vec<String>,
    /// Release-data-state tokens.
    pub release_data_states: Vec<String>,
    /// Channel tokens.
    pub channels: Vec<String>,
    /// Profile tokens.
    pub profiles: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
    /// Gap-kind tokens.
    pub gap_kinds: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
}

impl SummaryVocabulary {
    /// The canonical frozen vocabulary.
    pub fn canonical() -> Self {
        Self {
            families: tokens(&ArtifactFamily::ALL, |x| x.as_str()),
            artifact_classes: tokens(&ArtifactClass::ALL, |x| x.as_str()),
            postures: tokens(&UpdatePosture::ALL, |x| x.as_str()),
            verification_states: tokens(&VerificationState::ALL, |x| x.as_str()),
            restart_impacts: tokens(&RestartImpact::ALL, |x| x.as_str()),
            rollback_availabilities: tokens(&RollbackAvailability::ALL, |x| x.as_str()),
            release_data_states: tokens(&ReleaseDataState::ALL, |x| x.as_str()),
            channels: tokens(&ChannelScope::ALL, |x| x.as_str()),
            profiles: tokens(&DeploymentProfile::ALL, |x| x.as_str()),
            consumers: tokens(&SummaryConsumer::ALL, |x| x.as_str()),
            gap_kinds: tokens(&SummaryGapKind::ALL, |x| x.as_str()),
            gate_decisions: tokens(&DescriptorGate::ALL, |x| x.as_str()),
        }
    }

    /// True when this vocabulary equals the canonical frozen vocabulary.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Conformance flags every canonical summary packet asserts. They restate the acceptance bar so a
/// tampered packet that flips one to false fails [`M5UpdateCenterSummary::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryConformance {
    /// Every artifact family is summarized exactly once.
    pub every_family_summarized: bool,
    /// Every entry discloses at least one artifact-class delta row.
    pub every_entry_discloses_its_delta_rows: bool,
    /// Each entry's disclosed artifact classes equal the union of its delta rows plus its primary
    /// class — no changed class is hidden behind a generic update.
    pub affected_classes_match_delta_rows: bool,
    /// Verification state is disclosed for every artifact class.
    pub verification_state_disclosed_per_class: bool,
    /// Restart and rollback truth is disclosed on every entry.
    pub restart_and_rollback_truth_disclosed: bool,
    /// Rollback is never disclosed unless it is a genuine version rollback.
    pub rollback_never_overclaimed: bool,
    /// Mirrored / offline / stale release data is labeled, not masqueraded as live.
    pub stale_or_mirrored_data_labelled_not_masqueraded: bool,
    /// A not-provided (no live data) entry blocks Stable promotion.
    pub not_provided_data_blocks_stable_promotion: bool,
    /// Release center, update center, and Help/About read this one summary packet.
    pub consumers_read_one_summary: bool,
    /// Every consumer verdict is derived from the entries, not hand-maintained.
    pub consumer_verdict_derived_from_entries: bool,
    /// The controlled enums are frozen.
    pub controlled_enums_frozen: bool,
    /// The export carries metadata and refs only — no credential bodies or raw payloads.
    pub export_carries_no_raw_material: bool,
}

impl SummaryConformance {
    fn canonical() -> Self {
        Self {
            every_family_summarized: true,
            every_entry_discloses_its_delta_rows: true,
            affected_classes_match_delta_rows: true,
            verification_state_disclosed_per_class: true,
            restart_and_rollback_truth_disclosed: true,
            rollback_never_overclaimed: true,
            stale_or_mirrored_data_labelled_not_masqueraded: true,
            not_provided_data_blocks_stable_promotion: true,
            consumers_read_one_summary: true,
            consumer_verdict_derived_from_entries: true,
            controlled_enums_frozen: true,
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
pub enum SummaryChannel {
    /// The desktop update center.
    DesktopUi,
    /// The CLI / headless emitter.
    CliHeadless,
    /// The offline mirror surface.
    OfflineMirror,
}

// ---------------------------------------------------------------------------
// Validation violations
// ---------------------------------------------------------------------------

/// A reason a summary packet failed [`M5UpdateCenterSummary::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5UpdateCenterSummaryViolation {
    /// The record kind or schema version is wrong.
    HeaderDrift,
    /// A family is missing or summarized more than once.
    FamilyCoverageDrift,
    /// An entry's derived roll-ups (verification / restart / data / classes / gate) drifted.
    EntryRollupDrift,
    /// An entry has no delta rows.
    EntryMissingDeltaRows,
    /// A delta row's derived gate / signal drifted.
    DeltaRowDrift,
    /// An entry discloses rollback while only a fallback or reinstall is available.
    RollbackOverclaim,
    /// A consumer's derived verdict, unions, or gaps drifted.
    ConsumerVerdictDrift,
    /// The summary counts or release gate drifted.
    SummaryDrift,
    /// The disclosure flags do not all assert consumption of the one summary.
    DisclosureDrift,
    /// The controlled vocabulary drifted.
    VocabularyDrift,
    /// A conformance flag does not hold.
    ConformanceDrift,
    /// The export carried forbidden raw material.
    ForbiddenMaterial,
}

impl M5UpdateCenterSummaryViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeaderDrift => "header_drift",
            Self::FamilyCoverageDrift => "family_coverage_drift",
            Self::EntryRollupDrift => "entry_rollup_drift",
            Self::EntryMissingDeltaRows => "entry_missing_delta_rows",
            Self::DeltaRowDrift => "delta_row_drift",
            Self::RollbackOverclaim => "rollback_overclaim",
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

/// Builder input for [`M5UpdateCenterSummary::new`].
#[derive(Debug, Clone)]
pub struct M5UpdateCenterSummaryInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-facing report label.
    pub report_label: String,
    /// Evaluation timestamp.
    pub evaluated_at: String,
    /// The channel this summary reflects.
    pub channel: ChannelScope,
    /// The per-family summary entries.
    pub entries: Vec<UpdateSummaryEntry>,
    /// The claimed consumer rows.
    pub consumers: Vec<SummaryConsumerRow>,
    /// Redaction-class token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable update-center summary packet the release center, update
/// center, and Help/About panel consume.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5UpdateCenterSummary {
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
    /// The channel this summary reflects.
    pub channel: ChannelScope,
    /// The per-family summary entries.
    pub entries: Vec<UpdateSummaryEntry>,
    /// The family tokens this packet covers.
    pub families: Vec<String>,
    /// The consumer rows reading the summary.
    pub consumers: Vec<SummaryConsumerRow>,
    /// The consumer tokens, in canonical order.
    pub consumer_tokens: Vec<String>,
    /// Disclosure flags.
    pub disclosure: SummaryDisclosure,
    /// Roll-up counts.
    pub summary: SummaryCounts,
    /// Packet-level release gate.
    pub release_gate: SummaryReleaseGate,
    /// Controlled vocabulary.
    pub vocabulary: SummaryVocabulary,
    /// Conformance flags.
    pub conformance: SummaryConformance,
    /// Redaction-class token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5UpdateCenterSummary {
    /// Builds a packet from the given entries and consumer rows, recomputing every derived field so
    /// the published packet is always generated from the same checked-in summary objects.
    pub fn new(input: M5UpdateCenterSummaryInput) -> Self {
        let mut entries = input.entries;
        for entry in &mut entries {
            entry.recompute();
        }
        entries.sort_by_key(|e| family_rank(e.family));

        let mut consumers = input.consumers;
        for consumer in &mut consumers {
            consumer.recompute(&entries);
        }
        consumers.sort_by_key(|c| SummaryConsumer::ALL.iter().position(|x| *x == c.consumer));

        let summary = derive_counts(&entries, &consumers);
        let release_gate = derive_release_gate(&entries, &consumers);

        Self {
            record_kind: M5_UPDATE_CENTER_SUMMARY_RECORD_KIND.to_owned(),
            schema_version: M5_UPDATE_CENTER_SUMMARY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            channel: input.channel,
            families: tokens(&ArtifactFamily::ALL, |x| x.as_str()),
            entries,
            consumer_tokens: tokens(&SummaryConsumer::ALL, |x| x.as_str()),
            consumers,
            disclosure: SummaryDisclosure::canonical(),
            summary,
            release_gate,
            vocabulary: SummaryVocabulary::canonical(),
            conformance: SummaryConformance::canonical(),
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Looks up the entry for a family.
    pub fn entry(&self, family: ArtifactFamily) -> Option<&UpdateSummaryEntry> {
        self.entries.iter().find(|e| e.family == family)
    }

    /// Looks up the consumer row for a consumer.
    pub fn consumer(&self, consumer: SummaryConsumer) -> Option<&SummaryConsumerRow> {
        self.consumers.iter().find(|c| c.consumer == consumer)
    }

    /// Whether the packet blocks Stable promotion.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.release_gate.blocks_stable_promotion
    }

    /// Validates every derived field by recomputing it from the entries and comparing. Returns an
    /// empty vector when the packet is internally consistent.
    pub fn validate(&self) -> Vec<M5UpdateCenterSummaryViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_UPDATE_CENTER_SUMMARY_RECORD_KIND
            || self.schema_version != M5_UPDATE_CENTER_SUMMARY_SCHEMA_VERSION
        {
            violations.push(M5UpdateCenterSummaryViolation::HeaderDrift);
        }

        // Every family covered exactly once.
        for family in ArtifactFamily::ALL {
            let count = self.entries.iter().filter(|e| e.family == family).count();
            if count != 1 {
                violations.push(M5UpdateCenterSummaryViolation::FamilyCoverageDrift);
                break;
            }
        }

        for entry in &self.entries {
            if entry.delta_rows.is_empty() {
                violations.push(M5UpdateCenterSummaryViolation::EntryMissingDeltaRows);
            }
            // Recompute the entry from its rows and compare the roll-ups.
            let mut fresh = entry.clone();
            fresh.recompute();
            if fresh.verification_state != entry.verification_state
                || fresh.restart_impact != entry.restart_impact
                || fresh.release_data_state != entry.release_data_state
                || fresh.affected_artifact_classes != entry.affected_artifact_classes
                || fresh.gate != entry.gate
                || fresh.status != entry.status
                || fresh.signal != entry.signal
                || fresh.apply_ready != entry.apply_ready
                || fresh.update_available != entry.update_available
            {
                violations.push(M5UpdateCenterSummaryViolation::EntryRollupDrift);
            }
            for (row, fresh_row) in entry.delta_rows.iter().zip(fresh.delta_rows.iter()) {
                if row.gate != fresh_row.gate || row.signal != fresh_row.signal {
                    violations.push(M5UpdateCenterSummaryViolation::DeltaRowDrift);
                    break;
                }
            }
            // Rollback can never be disclosed unless it is a genuine version rollback.
            if entry.rollback_disclosed != entry.rollback.is_true_rollback() {
                violations.push(M5UpdateCenterSummaryViolation::RollbackOverclaim);
            }
        }

        // Consumers: recompute and compare verdict, unions, and gaps.
        for consumer in &self.consumers {
            let mut fresh = SummaryConsumerRow::new(
                consumer.consumer,
                consumer.claimed_qualification,
                &consumer.read_families,
            );
            fresh.recompute(&self.entries);
            if fresh.gate_decision != consumer.gate_decision
                || fresh.status != consumer.status
                || fresh.signal != consumer.signal
                || fresh.effective_qualification != consumer.effective_qualification
                || fresh.disclosed_artifact_classes != consumer.disclosed_artifact_classes
                || fresh.channels != consumer.channels
                || fresh.profiles != consumer.profiles
                || fresh.gaps != consumer.gaps
            {
                violations.push(M5UpdateCenterSummaryViolation::ConsumerVerdictDrift);
                break;
            }
        }

        // Summary counts and release gate.
        if self.summary != derive_counts(&self.entries, &self.consumers)
            || self.release_gate != derive_release_gate(&self.entries, &self.consumers)
        {
            violations.push(M5UpdateCenterSummaryViolation::SummaryDrift);
        }

        if !self.disclosure.all_consume()
            || self.consumer_tokens != tokens(&SummaryConsumer::ALL, |x| x.as_str())
            || self.families != tokens(&ArtifactFamily::ALL, |x| x.as_str())
        {
            violations.push(M5UpdateCenterSummaryViolation::DisclosureDrift);
        }

        if !self.vocabulary.matches_canonical() {
            violations.push(M5UpdateCenterSummaryViolation::VocabularyDrift);
        }

        if !self.conformance.all_hold() {
            violations.push(M5UpdateCenterSummaryViolation::ConformanceDrift);
        }

        if contains_forbidden_material(self) {
            violations.push(M5UpdateCenterSummaryViolation::ForbiddenMaterial);
        }

        violations
    }

    /// The canonical export form: pretty JSON, identical across every render channel.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("summary packet serializes")
    }

    /// Renders the packet for a channel. Every channel produces byte-identical output.
    pub fn render_for_channel(&self, _channel: SummaryChannel) -> String {
        self.export_safe_json()
    }

    /// A compact Markdown summary of the entries and consumer verdicts.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# {}\n\n", self.report_label));
        out.push_str(&format!(
            "Channel `{}` — {} families, {} delta rows, {} consumers.\n\n",
            self.channel.as_str(),
            self.summary.total_entries,
            self.summary.total_delta_rows,
            self.summary.total_consumers
        ));
        out.push_str("## Update summary entries\n\n");
        out.push_str(
            "| Family | Current | Target | Posture | Verification | Restart | Rollback | Data | Gate |\n",
        );
        out.push_str("|---|---|---|---|---|---|---|---|---|\n");
        for e in &self.entries {
            out.push_str(&format!(
                "| `{}` | {} | {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                e.family.as_str(),
                e.current_version,
                e.target_version,
                e.posture.as_str(),
                e.verification_state.as_str(),
                e.restart_impact.as_str(),
                e.rollback.as_str(),
                e.release_data_state.as_str(),
                e.gate.as_str(),
            ));
        }
        out.push_str("\n## Consumers\n\n");
        for c in &self.consumers {
            out.push_str(&format!(
                "- `{}` → `{}` ({}",
                c.consumer.as_str(),
                c.effective_qualification.as_str(),
                c.gate_decision.as_str()
            ));
            if c.gaps.is_empty() {
                out.push_str(")\n");
            } else {
                let gaps: Vec<String> = c
                    .gaps
                    .iter()
                    .map(|g| format!("{}:{}", g.family.as_str(), g.gap_kind.as_str()))
                    .collect();
                out.push_str(&format!("; gap: {})\n", gaps.join(", ")));
            }
        }
        out
    }

    /// A machine-readable CSV of every artifact-class delta row across the entries.
    pub fn render_delta_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "family,artifact_class,change_kind,from_version,to_version,verification_state,restart_impact,release_data_state,gate\n",
        );
        for e in &self.entries {
            for r in &e.delta_rows {
                out.push_str(&format!(
                    "{},{},{},{},{},{},{},{},{}\n",
                    e.family.as_str(),
                    r.artifact_class.as_str(),
                    r.change_kind.as_str(),
                    r.from_version.as_deref().unwrap_or(""),
                    r.to_version.as_deref().unwrap_or(""),
                    r.verification_state.as_str(),
                    r.restart_impact.as_str(),
                    r.release_data_state.as_str(),
                    r.gate.as_str(),
                ));
            }
        }
        out
    }
}

fn derive_counts(
    entries: &[UpdateSummaryEntry],
    consumers: &[SummaryConsumerRow],
) -> SummaryCounts {
    let governed = entries
        .iter()
        .filter(|e| e.gate == DescriptorGate::Governed)
        .count() as u32;
    let narrowed = entries
        .iter()
        .filter(|e| e.gate == DescriptorGate::Narrowed)
        .count() as u32;
    let blocked = entries
        .iter()
        .filter(|e| e.gate == DescriptorGate::Blocked)
        .count() as u32;
    let certified = consumers.iter().filter(|c| c.is_certified()).count() as u32;
    let c_narrowed = consumers.iter().filter(|c| c.is_narrowed()).count() as u32;
    let c_blocked = consumers.iter().filter(|c| c.is_blocked()).count() as u32;
    SummaryCounts {
        total_entries: entries.len() as u32,
        update_available_count: entries.iter().filter(|e| e.update_available).count() as u32,
        up_to_date_count: entries.iter().filter(|e| !e.update_available).count() as u32,
        governed_entries: governed,
        narrowed_entries: narrowed,
        blocked_entries: blocked,
        total_delta_rows: entries.iter().map(|e| e.delta_rows.len() as u32).sum(),
        total_consumers: consumers.len() as u32,
        certified_consumer_count: certified,
        narrowed_consumer_count: c_narrowed,
        blocked_consumer_count: c_blocked,
        blocks_stable_promotion: c_blocked > 0,
    }
}

fn derive_release_gate(
    entries: &[UpdateSummaryEntry],
    consumers: &[SummaryConsumerRow],
) -> SummaryReleaseGate {
    let collect = |pred: fn(&SummaryConsumerRow) -> bool| -> Vec<String> {
        consumers
            .iter()
            .filter(|c| pred(c))
            .map(|c| c.consumer.as_str().to_owned())
            .collect()
    };
    let mut affected: Vec<ArtifactFamily> = consumers
        .iter()
        .flat_map(|c| c.gaps.iter().map(|g| g.family))
        .collect();
    affected.sort_by_key(|f| family_rank(*f));
    affected.dedup();
    let _ = entries;
    let blocked = collect(SummaryConsumerRow::is_blocked);
    SummaryReleaseGate {
        blocks_stable_promotion: !blocked.is_empty(),
        blocked_consumers: blocked,
        narrowed_consumers: collect(SummaryConsumerRow::is_narrowed),
        certified_consumers: collect(SummaryConsumerRow::is_certified),
        affected_families: affected.iter().map(|f| f.as_str().to_owned()).collect(),
        gate_message_id: format!("{}release_gate", M5_UPDATE_CENTER_SUMMARY_MESSAGE_ID_PREFIX),
    }
}

/// Scans the export for forbidden raw material (credential bodies / raw provider payloads).
fn contains_forbidden_material(packet: &M5UpdateCenterSummary) -> bool {
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
