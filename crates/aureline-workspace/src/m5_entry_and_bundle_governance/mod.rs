//! Canonical M5 workflow-bundle and project-entry governance matrix: the single
//! qualification report that freezes the workflow-bundle, source-acquisition, project-open,
//! project-import, session-resume, recent-work, and workspace-admission switching lanes into
//! one non-inheriting entry gate.
//!
//! Each [`EntryBundleRow`] governs one M5 switching/entry depth lane ([`EntryBundleLane`])
//! against the canonical entry-truth packet it draws from, and answers, for that lane, who
//! owns the evidence ([`EntryBundleRow::owner`]), how trusted its source is
//! ([`SourceTrust`]), how confidently the archetype was detected ([`ArchetypeConfidence`]),
//! whether its roots resolved ([`RootResolution`]), how faithfully it restores prior state
//! ([`RestoreFidelity`]), whether its bundle scorecard is current ([`BundleScorecard`]), and
//! whether its entry topology is supported ([`EntryTopologySupport`]). The row then publishes
//! an [`EntryAssurance`] no input can exceed.
//!
//! The [`EntryAssurance`] a lane may publish is the weakest ceiling implied by its observed
//! states, so an unverified source, a probable or mixed archetype, missing roots, a partial
//! restore, a stale bundle scorecard, or an unsupported entry topology all narrow or withhold
//! the published label automatically. The guardrail this enforces: probable or mixed
//! workspace detection never silently widens trust — a lane whose source is unverified, whose
//! archetype is only probable or mixed, whose roots did not resolve, whose restore is partial,
//! whose bundle scorecard is stale, or whose entry topology is unsupported is narrowed to a
//! bounded or retest-pending label, or refused entirely, rather than left quietly stable. The
//! [`AdmissionOutcome`] records the gate's routing action — admit the lane at full trust,
//! admit it bounded, admit it pending retest, or refuse it — and the recomputed
//! [`DowngradeReason`]s and [`DowngradePath`] explain it; all are validated against the gate.
//!
//! First-useful-work routing stays explicit. Each row carries a [`SetupQueueClass`] plus a
//! deferred-setup and a missing-root count, so the matrix keeps a ready entry distinct from a
//! setup-later, blocked-on-setup, or missing-root one instead of collapsing them into one
//! "not ready" answer, and a verified lane is forbidden from hiding deferred setup or missing
//! roots.
//!
//! The lane vocabulary is closed and provenance-bound. [`EntryBundleLane`] is the single
//! controlled vocabulary the matrix reuses, each lane is pinned to one [`EntryVerb`] so clone,
//! open, import, and resume stay distinct, and each lane is pinned to the canonical entry-truth
//! packet it governs via [`EntryBundleLane::source_packet`], so a clean open lane never lends
//! its trust to a refused admission lane and no probable-detection lane inherits a broader
//! stable claim.
//!
//! Because every row also carries a release-evidence ref, a help-surface ref, a docs-badge
//! ref, and a support-export ref, release evidence, help/start-center, docs, and support
//! exports ingest the *same* governance packet rather than parallel spreadsheets, so a
//! narrowed lane cannot stay stable in one surface while it is downgraded in another.
//!
//! The packet is checked in at
//! `artifacts/workspace/m5/m5-entry-and-bundle-governance.json` and embedded here. It is
//! metadata-only: every field is a typed state, a count, or an opaque ref, and it carries no
//! credential bodies, raw provider payloads, or workspace contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5 entry-and-bundle governance matrix schema version.
pub const M5_ENTRY_BUNDLE_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_ENTRY_BUNDLE_GOVERNANCE_RECORD_KIND: &str = "m5_entry_and_bundle_governance_matrix";

/// Repo-relative path to the checked-in packet.
pub const M5_ENTRY_BUNDLE_GOVERNANCE_PATH: &str =
    "artifacts/workspace/m5/m5-entry-and-bundle-governance.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_ENTRY_BUNDLE_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/workspace/m5-entry-and-bundle-governance.schema.json";

/// Repo-relative path to the companion document.
pub const M5_ENTRY_BUNDLE_GOVERNANCE_DOC_REF: &str =
    "docs/workspace/m5/m5-entry-and-bundle-governance.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_ENTRY_BUNDLE_GOVERNANCE_FIXTURE_DIR: &str =
    "fixtures/workspace/m5/m5-entry-and-bundle-governance";

/// Embedded checked-in packet JSON.
pub const M5_ENTRY_BUNDLE_GOVERNANCE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/workspace/m5/m5-entry-and-bundle-governance.json"
));

/// An M5 switching/entry depth lane the governance matrix gates.
///
/// Each lane is governed from the canonical entry-truth packet it draws its evidence from, so
/// the matrix aggregates the landed stable-line entry packets into one report instead of
/// re-deriving each lane's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryBundleLane {
    /// Workflow-bundle composition, install diff, and rollback checkpoint.
    WorkflowBundle,
    /// Source-locator resolution and checkout-plan acquisition.
    SourceAcquisition,
    /// Opening an existing local workspace root.
    ProjectOpen,
    /// Importing or migrating a workspace from another tool.
    ProjectImport,
    /// Resuming a prior session and restoring its state.
    SessionResume,
    /// Recent-work registry truth feeding the start center.
    RecentWork,
    /// Workspace-admission routing and first-useful-work selection.
    WorkspaceAdmission,
}

impl EntryBundleLane {
    /// Every entry lane, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::WorkflowBundle,
        Self::SourceAcquisition,
        Self::ProjectOpen,
        Self::ProjectImport,
        Self::SessionResume,
        Self::RecentWork,
        Self::WorkspaceAdmission,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkflowBundle => "workflow_bundle",
            Self::SourceAcquisition => "source_acquisition",
            Self::ProjectOpen => "project_open",
            Self::ProjectImport => "project_import",
            Self::SessionResume => "session_resume",
            Self::RecentWork => "recent_work",
            Self::WorkspaceAdmission => "workspace_admission",
        }
    }

    /// The distinct entry verb this lane is pinned to.
    ///
    /// Clone, open, import, and resume stay distinct verbs, and the `entry_verb` recorded on
    /// every row is validated against this.
    pub const fn entry_verb(self) -> EntryVerb {
        match self {
            Self::WorkflowBundle => EntryVerb::Install,
            Self::SourceAcquisition => EntryVerb::Clone,
            Self::ProjectOpen | Self::RecentWork | Self::WorkspaceAdmission => EntryVerb::Open,
            Self::ProjectImport => EntryVerb::Import,
            Self::SessionResume => EntryVerb::Resume,
        }
    }

    /// Repo-relative path to the canonical entry-truth packet this lane governs.
    ///
    /// The matrix is pinned to this packet so a lane never publishes a label its own source
    /// packet does not back, and the `packet_ref` recorded on every row is validated against
    /// it.
    pub const fn source_packet(self) -> &'static str {
        match self {
            Self::WorkflowBundle => "artifacts/bundles/python_launch_bundle_alpha.yaml",
            Self::SourceAcquisition => {
                "artifacts/workspace/m4/stabilize-source-locator-checkout-plan-bootstrap-result-and-queue.md"
            }
            Self::ProjectOpen => "artifacts/entry/open_flow_truth.md",
            Self::ProjectImport => {
                "artifacts/workspace/m4/harden-workspace-open-clone-import-and-resume-flows.md"
            }
            Self::SessionResume => {
                "artifacts/workspace/m4/finalize-portable-state-export-import-and-restore-provenance.md"
            }
            Self::RecentWork => "artifacts/entry/warm_start_chooser_contract.md",
            Self::WorkspaceAdmission => {
                "artifacts/workspace/m4/finalize-workspace-trust-gating-across-tasks-terminal-debug.md"
            }
        }
    }

    /// Whether this lane can silently widen trust — a clone, import, resume, bundle-install, or
    /// admission lane that must narrow safely rather than inherit a broader stable claim.
    pub const fn is_trust_sensitive(self) -> bool {
        matches!(
            self,
            Self::WorkflowBundle
                | Self::SourceAcquisition
                | Self::ProjectImport
                | Self::SessionResume
                | Self::WorkspaceAdmission
        )
    }
}

/// The distinct entry verb a lane drives.
///
/// Clone, open, import, and resume remain distinct verbs, and bundle install is its own verb,
/// so an entry surface never conflates acquiring a source with opening a known root or
/// resuming a prior session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryVerb {
    /// Clone a remote source into a new local root.
    Clone,
    /// Open an existing local workspace root.
    Open,
    /// Import or migrate a workspace from another tool.
    Import,
    /// Resume a prior session and restore its state.
    Resume,
    /// Install or update a workflow bundle.
    Install,
}

impl EntryVerb {
    /// Every entry verb, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Clone,
        Self::Open,
        Self::Import,
        Self::Resume,
        Self::Install,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clone => "clone",
            Self::Open => "open",
            Self::Import => "import",
            Self::Resume => "resume",
            Self::Install => "install",
        }
    }
}

/// The workflow-bundle class a lane composes, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleClass {
    /// A launch bundle bootstrapping a runnable project.
    LaunchBundle,
    /// A framework pack layering a framework onto a project.
    FrameworkPack,
    /// A template bundle scaffolding new content.
    TemplateBundle,
    /// An imported user-handoff bundle migrated from another setup.
    ImportedHandoffBundle,
    /// An organization-managed bundle pinned by policy.
    OrgManagedBundle,
    /// No bundle participates in this lane.
    NoBundle,
}

impl BundleClass {
    /// Every bundle class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LaunchBundle,
        Self::FrameworkPack,
        Self::TemplateBundle,
        Self::ImportedHandoffBundle,
        Self::OrgManagedBundle,
        Self::NoBundle,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchBundle => "launch_bundle",
            Self::FrameworkPack => "framework_pack",
            Self::TemplateBundle => "template_bundle",
            Self::ImportedHandoffBundle => "imported_handoff_bundle",
            Self::OrgManagedBundle => "org_managed_bundle",
            Self::NoBundle => "no_bundle",
        }
    }
}

/// The source locator type a lane resolves its workspace from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocatorType {
    /// A local filesystem path.
    LocalPath,
    /// A remote Git repository.
    GitRemote,
    /// An archive unpacked into a new root.
    ArchiveImport,
    /// A migration handoff from another editor or tool.
    ToolMigration,
    /// A recent-work handle re-resolved from the registry.
    RecentHandle,
    /// No locator participates in this lane.
    NotApplicable,
}

impl LocatorType {
    /// Every locator type, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalPath,
        Self::GitRemote,
        Self::ArchiveImport,
        Self::ToolMigration,
        Self::RecentHandle,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalPath => "local_path",
            Self::GitRemote => "git_remote",
            Self::ArchiveImport => "archive_import",
            Self::ToolMigration => "tool_migration",
            Self::RecentHandle => "recent_handle",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// How authoritative a lane's published entry-assurance label is.
///
/// Ordered low-to-high by [`EntryAssurance::rank`]: an [`EntryAssurance::Withheld`] lane has no
/// publishable label, and an [`EntryAssurance::Verified`] lane is backed by a first-party,
/// confirmed, fully-resolved, exact, current, supported entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryAssurance {
    /// A full-trust claim backed by a first-party, confirmed, exact entry.
    Verified,
    /// Bounded to an open-minimal, setup-later, or local-safe slice.
    Bounded,
    /// Held pending retest; probable, mixed, partial, or stale.
    RetestPending,
    /// Withheld from publication; no publishable entry label.
    Withheld,
}

impl EntryAssurance {
    /// Every entry-assurance label, in declaration order.
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

/// How trusted the source a lane acquired its workspace from is — the host class and trust
/// stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceTrust {
    /// A first-party, already-trusted source.
    FirstParty,
    /// A known, trusted remote host; bounded until admitted.
    TrustedRemote,
    /// An unverified remote or import source; needs a trust scan.
    UnverifiedRemote,
    /// An untrusted source; no publishable trust.
    Untrusted,
}

impl SourceTrust {
    /// Every source-trust state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FirstParty,
        Self::TrustedRemote,
        Self::UnverifiedRemote,
        Self::Untrusted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::TrustedRemote => "trusted_remote",
            Self::UnverifiedRemote => "unverified_remote",
            Self::Untrusted => "untrusted",
        }
    }

    /// Highest assurance this source-trust state permits a lane to publish.
    pub const fn assurance_ceiling(self) -> EntryAssurance {
        match self {
            Self::FirstParty => EntryAssurance::Verified,
            Self::TrustedRemote => EntryAssurance::Bounded,
            Self::UnverifiedRemote => EntryAssurance::RetestPending,
            Self::Untrusted => EntryAssurance::Withheld,
        }
    }

    /// Whether this state raises the [`DowngradeReason::UnverifiedSource`] trigger.
    pub const fn is_unverified_trigger(self) -> bool {
        !matches!(self, Self::FirstParty)
    }
}

/// How confidently a lane detected the workspace archetype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchetypeConfidence {
    /// The archetype was detected with high confidence.
    Confirmed,
    /// A single archetype is probable but unconfirmed; bounded.
    Probable,
    /// Detection is mixed or ambiguous; needs user confirmation.
    Mixed,
    /// No archetype could be resolved.
    Undetected,
}

impl ArchetypeConfidence {
    /// Every archetype-confidence state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Confirmed,
        Self::Probable,
        Self::Mixed,
        Self::Undetected,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Confirmed => "confirmed",
            Self::Probable => "probable",
            Self::Mixed => "mixed",
            Self::Undetected => "undetected",
        }
    }

    /// Highest assurance this confidence state permits a lane to publish.
    pub const fn assurance_ceiling(self) -> EntryAssurance {
        match self {
            Self::Confirmed => EntryAssurance::Verified,
            Self::Probable => EntryAssurance::Bounded,
            Self::Mixed => EntryAssurance::RetestPending,
            Self::Undetected => EntryAssurance::Withheld,
        }
    }

    /// Whether this state raises the [`DowngradeReason::ProbableOrMixedDetection`] trigger.
    pub const fn is_probable_or_mixed_trigger(self) -> bool {
        !matches!(self, Self::Confirmed)
    }
}

/// Whether a lane resolved the workspace roots it expected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootResolution {
    /// Every expected root resolved.
    Resolved,
    /// A single root is assumed where more may exist; bounded.
    SingleRootAssumed,
    /// A probable multi-root layout is unconfirmed; needs retest.
    ProbableMultiRoot,
    /// Expected roots are missing.
    Missing,
}

impl RootResolution {
    /// Every root-resolution state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Resolved,
        Self::SingleRootAssumed,
        Self::ProbableMultiRoot,
        Self::Missing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::SingleRootAssumed => "single_root_assumed",
            Self::ProbableMultiRoot => "probable_multi_root",
            Self::Missing => "missing",
        }
    }

    /// Highest assurance this root-resolution state permits a lane to publish.
    pub const fn assurance_ceiling(self) -> EntryAssurance {
        match self {
            Self::Resolved => EntryAssurance::Verified,
            Self::SingleRootAssumed => EntryAssurance::Bounded,
            Self::ProbableMultiRoot => EntryAssurance::RetestPending,
            Self::Missing => EntryAssurance::Withheld,
        }
    }

    /// Whether this state raises the [`DowngradeReason::MissingRoots`] trigger.
    pub const fn is_missing_roots_trigger(self) -> bool {
        !matches!(self, Self::Resolved)
    }
}

/// How faithfully a lane restores prior session state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreFidelity {
    /// Restore reproduces prior state exactly.
    Exact,
    /// Restore is partial; some state is dropped with disclosure.
    Partial,
    /// Restore is degraded; falls back to open-without-restore.
    Degraded,
    /// No restore data is available.
    Unavailable,
}

impl RestoreFidelity {
    /// Every restore-fidelity state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Exact,
        Self::Partial,
        Self::Degraded,
        Self::Unavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Partial => "partial",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
        }
    }

    /// Highest assurance this restore-fidelity state permits a lane to publish.
    pub const fn assurance_ceiling(self) -> EntryAssurance {
        match self {
            Self::Exact => EntryAssurance::Verified,
            Self::Partial => EntryAssurance::Bounded,
            Self::Degraded => EntryAssurance::RetestPending,
            Self::Unavailable => EntryAssurance::Withheld,
        }
    }

    /// Whether this state raises the [`DowngradeReason::PartialRestore`] trigger.
    pub const fn is_partial_restore_trigger(self) -> bool {
        !matches!(self, Self::Exact)
    }
}

/// How current a lane's bundle scorecard is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleScorecard {
    /// The bundle scorecard is current.
    Current,
    /// The bundle scorecard is aging but within tolerance; bounded.
    Aging,
    /// The bundle scorecard is stale; needs a refresh.
    Stale,
    /// The bundle scorecard is missing.
    Missing,
}

impl BundleScorecard {
    /// Every bundle-scorecard state, in declaration order.
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

    /// Highest assurance this bundle-scorecard state permits a lane to publish.
    pub const fn assurance_ceiling(self) -> EntryAssurance {
        match self {
            Self::Current => EntryAssurance::Verified,
            Self::Aging => EntryAssurance::Bounded,
            Self::Stale => EntryAssurance::RetestPending,
            Self::Missing => EntryAssurance::Withheld,
        }
    }

    /// Whether this state raises the [`DowngradeReason::StaleBundleScorecard`] trigger.
    pub const fn is_stale_trigger(self) -> bool {
        !matches!(self, Self::Current)
    }
}

/// Whether a lane's entry topology is supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryTopologySupport {
    /// The entry topology is fully supported.
    Supported,
    /// The entry topology is supported with known limits; bounded.
    DegradedSupport,
    /// The entry topology is experimental; needs retest.
    Experimental,
    /// The entry topology is unsupported.
    Unsupported,
}

impl EntryTopologySupport {
    /// Every entry-topology-support state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Supported,
        Self::DegradedSupport,
        Self::Experimental,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::DegradedSupport => "degraded_support",
            Self::Experimental => "experimental",
            Self::Unsupported => "unsupported",
        }
    }

    /// Highest assurance this entry-topology-support state permits a lane to publish.
    pub const fn assurance_ceiling(self) -> EntryAssurance {
        match self {
            Self::Supported => EntryAssurance::Verified,
            Self::DegradedSupport => EntryAssurance::Bounded,
            Self::Experimental => EntryAssurance::RetestPending,
            Self::Unsupported => EntryAssurance::Withheld,
        }
    }

    /// Whether this state raises the [`DowngradeReason::UnsupportedEntryTopology`] trigger.
    pub const fn is_unsupported_trigger(self) -> bool {
        !matches!(self, Self::Supported)
    }
}

/// How a lane's first-useful-work routing is classed, keeping a ready entry distinct from a
/// setup-later, blocked-on-setup, or missing-root one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupQueueClass {
    /// First-useful-work is available immediately; no follow-up setup is queued.
    Ready,
    /// Useful work is available now; setup is deferred to a follow-up queue.
    SetupLater,
    /// Useful work is blocked pending required setup.
    BlockedOnSetup,
    /// The entry has no resolvable root.
    MissingRoot,
}

impl SetupQueueClass {
    /// Every setup-queue class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Ready,
        Self::SetupLater,
        Self::BlockedOnSetup,
        Self::MissingRoot,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::SetupLater => "setup_later",
            Self::BlockedOnSetup => "blocked_on_setup",
            Self::MissingRoot => "missing_root",
        }
    }
}

/// The recovery path surfaced when a lane's label is narrowed or withheld.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradePath {
    /// Verify or scan the source before widening trust.
    VerifySource,
    /// Confirm the detected archetype with the user.
    ConfirmArchetype,
    /// Resolve the missing or ambiguous workspace roots.
    ResolveRoots,
    /// Repair the restore or open without restore.
    RepairRestore,
    /// Refresh the stale or missing bundle scorecard.
    RefreshBundleScorecard,
    /// Request support for the unsupported entry topology.
    RequestTopologySupport,
    /// Withhold the lane's label from publication.
    WithholdClaim,
    /// No downgrade is needed; only valid when the lane is published verified.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl DowngradePath {
    /// Every downgrade path, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::VerifySource,
        Self::ConfirmArchetype,
        Self::ResolveRoots,
        Self::RepairRestore,
        Self::RefreshBundleScorecard,
        Self::RequestTopologySupport,
        Self::WithholdClaim,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifySource => "verify_source",
            Self::ConfirmArchetype => "confirm_archetype",
            Self::ResolveRoots => "resolve_roots",
            Self::RepairRestore => "repair_restore",
            Self::RefreshBundleScorecard => "refresh_bundle_scorecard",
            Self::RequestTopologySupport => "request_topology_support",
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
/// These are the canonical downgrade reasons: an unverified source, probable or mixed
/// detection, missing roots, a partial restore, a stale bundle scorecard, and an unsupported
/// entry topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeReason {
    /// The lane's source is not first-party.
    UnverifiedSource,
    /// The lane's archetype detection is only probable or mixed.
    ProbableOrMixedDetection,
    /// The lane's expected workspace roots are missing or ambiguous.
    MissingRoots,
    /// The lane's restore is partial, degraded, or unavailable.
    PartialRestore,
    /// The lane's bundle scorecard is aging, stale, or missing.
    StaleBundleScorecard,
    /// The lane's entry topology is degraded, experimental, or unsupported.
    UnsupportedEntryTopology,
}

impl DowngradeReason {
    /// Every downgrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UnverifiedSource,
        Self::ProbableOrMixedDetection,
        Self::MissingRoots,
        Self::PartialRestore,
        Self::StaleBundleScorecard,
        Self::UnsupportedEntryTopology,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnverifiedSource => "unverified_source",
            Self::ProbableOrMixedDetection => "probable_or_mixed_detection",
            Self::MissingRoots => "missing_roots",
            Self::PartialRestore => "partial_restore",
            Self::StaleBundleScorecard => "stale_bundle_scorecard",
            Self::UnsupportedEntryTopology => "unsupported_entry_topology",
        }
    }
}

/// The admission outcome the governance gate routes a lane to, relative to a clean verified
/// entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionOutcome {
    /// No narrowing; the lane is admitted at full trust.
    AdmitFull,
    /// The lane is admitted bounded to an open-minimal or local-safe slice.
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
    pub const fn for_assurance(assurance: EntryAssurance) -> Self {
        match assurance {
            EntryAssurance::Verified => Self::AdmitFull,
            EntryAssurance::Bounded => Self::AdmitBounded,
            EntryAssurance::RetestPending => Self::AdmitRetest,
            EntryAssurance::Withheld => Self::Refuse,
        }
    }
}

/// One governance row for an M5 switching/entry depth lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntryBundleRow {
    /// Stable governance-row id.
    pub lane_id: String,
    /// Entry lane this row governs.
    pub lane: EntryBundleLane,
    /// Distinct entry verb this lane drives; must equal [`EntryBundleLane::entry_verb`].
    pub entry_verb: EntryVerb,
    /// Bundle class this lane composes, if any.
    pub bundle_class: BundleClass,
    /// Source locator type this lane resolves from.
    pub locator_type: LocatorType,
    /// Owner accountable for the lane's evidence and conformance.
    pub owner: String,
    /// How trusted the lane's source is.
    pub source_trust: SourceTrust,
    /// How confidently the lane detected the archetype.
    pub archetype_confidence: ArchetypeConfidence,
    /// Whether the lane resolved its expected roots.
    pub root_resolution: RootResolution,
    /// How faithfully the lane restores prior state.
    pub restore_fidelity: RestoreFidelity,
    /// How current the lane's bundle scorecard is.
    pub bundle_scorecard: BundleScorecard,
    /// Whether the lane's entry topology is supported.
    pub entry_topology_support: EntryTopologySupport,
    /// How the lane's first-useful-work routing is classed.
    pub setup_queue_class: SetupQueueClass,
    /// Setup steps deferred to a follow-up queue.
    pub deferred_setup_count: u64,
    /// Expected roots the lane did not resolve.
    pub missing_root_count: u64,
    /// Stable namespace the lane mints workspace-root identities under.
    pub root_id_namespace: String,
    /// Stable namespace the lane mints bundle identities under.
    pub bundle_id_namespace: String,
    /// Assurance the lane's own evidence asserts, before the gate.
    pub declared_assurance: EntryAssurance,
    /// Assurance actually published after the gate narrows the lane.
    ///
    /// Must equal [`EntryBundleRow::effective_assurance`].
    pub published_assurance: EntryAssurance,
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
    /// Ref to the canonical entry-truth packet this lane governs.
    ///
    /// Must equal [`EntryBundleLane::source_packet`].
    pub packet_ref: String,
    /// Ref to the entry-conformance suite backing the lane.
    pub conformance_ref: String,
    /// Ref to the lane's supporting evidence.
    pub evidence_ref: String,
    /// Ref to the machine-readable governance receipt for audit and release evidence.
    pub governance_receipt_ref: String,
    /// Ref binding this row into the release-evidence surface.
    pub release_evidence_ref: String,
    /// Ref binding this row into the help/start-center surface.
    pub help_surface_ref: String,
    /// Ref binding this row into the docs-badge surface.
    pub docs_badge_ref: String,
    /// Ref binding this row into the support-export surface.
    pub support_export_ref: String,
    /// Additional source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl EntryBundleRow {
    /// The label the lane's own evidence asserted, before environmental narrowing.
    pub fn capability_floor(&self) -> EntryAssurance {
        self.declared_assurance
    }

    /// The assurance label the gate permits this lane to publish.
    ///
    /// Lowers the capability floor to the weakest ceiling implied by the source trust,
    /// archetype confidence, root resolution, restore fidelity, bundle scorecard, and entry
    /// topology support, so an unverified source, a probable or mixed archetype, missing roots,
    /// a partial restore, a stale scorecard, or an unsupported topology can never publish a
    /// verified label.
    pub fn effective_assurance(&self) -> EntryAssurance {
        self.capability_floor()
            .min(self.source_trust.assurance_ceiling())
            .min(self.archetype_confidence.assurance_ceiling())
            .min(self.root_resolution.assurance_ceiling())
            .min(self.restore_fidelity.assurance_ceiling())
            .min(self.bundle_scorecard.assurance_ceiling())
            .min(self.entry_topology_support.assurance_ceiling())
    }

    /// The headline downgrade reasons recomputed from the lane's observed states.
    pub fn computed_downgrade_reasons(&self) -> Vec<DowngradeReason> {
        let mut reasons = Vec::new();
        if self.source_trust.is_unverified_trigger() {
            reasons.push(DowngradeReason::UnverifiedSource);
        }
        if self.archetype_confidence.is_probable_or_mixed_trigger() {
            reasons.push(DowngradeReason::ProbableOrMixedDetection);
        }
        if self.root_resolution.is_missing_roots_trigger() {
            reasons.push(DowngradeReason::MissingRoots);
        }
        if self.restore_fidelity.is_partial_restore_trigger() {
            reasons.push(DowngradeReason::PartialRestore);
        }
        if self.bundle_scorecard.is_stale_trigger() {
            reasons.push(DowngradeReason::StaleBundleScorecard);
        }
        if self.entry_topology_support.is_unsupported_trigger() {
            reasons.push(DowngradeReason::UnsupportedEntryTopology);
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
        self.effective_assurance() == EntryAssurance::Verified
    }

    /// Whether the gate narrowed the published label below what the lane declared.
    ///
    /// This is the automatic downgrade: an unverified, probable, root-missing, partially
    /// restored, stale, or unsupported lane that declared a stronger label has its published
    /// label lowered rather than left quietly stable.
    pub fn is_downgraded(&self) -> bool {
        self.effective_assurance().rank() < self.capability_floor().rank()
    }

    /// Whether the lane carries its own non-empty source, conformance, evidence, receipt, and
    /// downstream-consumer refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.packet_ref.trim().is_empty()
            && !self.conformance_ref.trim().is_empty()
            && !self.evidence_ref.trim().is_empty()
            && !self.governance_receipt_ref.trim().is_empty()
            && !self.release_evidence_ref.trim().is_empty()
            && !self.help_surface_ref.trim().is_empty()
            && !self.docs_badge_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
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
pub struct M5EntryBundleGovernanceSummary {
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
    /// Lanes whose archetype detection is probable or mixed.
    pub probable_or_mixed_lanes: usize,
    /// Lanes whose bundle scorecard is aging, stale, or missing.
    pub stale_bundle_lanes: usize,
    /// Lanes whose entry topology is degraded, experimental, or unsupported.
    pub unsupported_topology_lanes: usize,
    /// Lanes carrying at least one downgrade reason.
    pub lanes_with_downgrade_reasons: usize,
    /// Lanes reporting at least one deferred setup step.
    pub lanes_with_deferred_setup: usize,
    /// Lanes reporting at least one missing root.
    pub lanes_with_missing_roots: usize,
}

/// A redaction-safe export row projected from a governance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EntryBundleGovernanceExportRow {
    /// Governance-row id.
    pub lane_id: String,
    /// Lane token.
    pub lane: String,
    /// Entry-verb token.
    pub entry_verb: String,
    /// Bundle-class token.
    pub bundle_class: String,
    /// Locator-type token.
    pub locator_type: String,
    /// Owner accountable for the lane.
    pub owner: String,
    /// Source-trust token.
    pub source_trust: String,
    /// Archetype-confidence token.
    pub archetype_confidence: String,
    /// Root-resolution token.
    pub root_resolution: String,
    /// Restore-fidelity token.
    pub restore_fidelity: String,
    /// Bundle-scorecard token.
    pub bundle_scorecard: String,
    /// Entry-topology-support token.
    pub entry_topology_support: String,
    /// Setup-queue-class token.
    pub setup_queue_class: String,
    /// Setup steps deferred to a follow-up queue.
    pub deferred_setup_count: u64,
    /// Expected roots the lane did not resolve.
    pub missing_root_count: u64,
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
    /// Help-surface ref.
    pub help_surface_ref: String,
    /// Docs-badge ref.
    pub docs_badge_ref: String,
    /// Support-export ref.
    pub support_export_ref: String,
    /// Whether the lane is trust-sensitive.
    pub trust_sensitive: bool,
    /// Whether the lane publishes a verified label.
    pub verified: bool,
    /// Whether the published label was downgraded below the declared label.
    pub downgraded: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet — the canonical entry/bundle index
/// downstream surfaces render instead of restating each lane's label by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EntryBundleGovernanceExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub lanes: Vec<M5EntryBundleGovernanceExportRow>,
    /// Whether every lane's published label and outcome agree with the gate.
    pub all_lanes_gate_consistent: bool,
    /// Lanes that publish a verified label.
    pub verified_count: usize,
    /// Lanes the gate narrowed or refused.
    pub narrowed_count: usize,
    /// Lanes the gate refused entirely.
    pub refused_count: usize,
}

/// The typed M5 entry-and-bundle governance matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5EntryBundleGovernanceMatrix {
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
    /// Scheme the matrix mints stable workspace identities under.
    pub workspace_identity_scheme: String,
    /// Claimed lanes; one row per lane.
    pub lanes: Vec<EntryBundleLane>,
    /// Closed entry-verb vocabulary.
    pub entry_verbs: Vec<EntryVerb>,
    /// Closed bundle-class vocabulary.
    pub bundle_classes: Vec<BundleClass>,
    /// Closed locator-type vocabulary.
    pub locator_types: Vec<LocatorType>,
    /// Closed assurance-label vocabulary.
    pub assurance_labels: Vec<EntryAssurance>,
    /// Closed source-trust vocabulary.
    pub source_trust_states: Vec<SourceTrust>,
    /// Closed archetype-confidence vocabulary.
    pub archetype_confidence_states: Vec<ArchetypeConfidence>,
    /// Closed root-resolution vocabulary.
    pub root_resolution_states: Vec<RootResolution>,
    /// Closed restore-fidelity vocabulary.
    pub restore_fidelities: Vec<RestoreFidelity>,
    /// Closed bundle-scorecard vocabulary.
    pub bundle_scorecard_states: Vec<BundleScorecard>,
    /// Closed entry-topology-support vocabulary.
    pub entry_topology_support_states: Vec<EntryTopologySupport>,
    /// Closed setup-queue-class vocabulary.
    pub setup_queue_classes: Vec<SetupQueueClass>,
    /// Closed downgrade-path vocabulary.
    pub downgrade_paths: Vec<DowngradePath>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Closed admission-outcome vocabulary.
    pub admission_outcomes: Vec<AdmissionOutcome>,
    /// Governance rows, one per claimed lane.
    #[serde(default)]
    pub lane_rows: Vec<EntryBundleRow>,
    /// Summary counts.
    pub summary: M5EntryBundleGovernanceSummary,
}

impl M5EntryBundleGovernanceMatrix {
    /// Returns the row for a claimed lane.
    pub fn lane_row(&self, lane: EntryBundleLane) -> Option<&EntryBundleRow> {
        self.lane_rows.iter().find(|c| c.lane == lane)
    }

    /// Lanes that publish a verified label.
    pub fn verified_lanes(&self) -> impl Iterator<Item = &EntryBundleRow> {
        self.lane_rows.iter().filter(|c| c.is_verified())
    }

    /// Lanes the gate narrowed or refused in any way.
    pub fn narrowed_lanes(&self) -> impl Iterator<Item = &EntryBundleRow> {
        self.lane_rows
            .iter()
            .filter(|c| c.required_outcome().is_narrowed())
    }

    /// Lanes the gate refused entirely.
    pub fn refused_lanes(&self) -> impl Iterator<Item = &EntryBundleRow> {
        self.lane_rows
            .iter()
            .filter(|c| c.required_outcome() == AdmissionOutcome::Refuse)
    }

    /// Whether every lane's stored published label, outcome, and reasons agree with the
    /// recomputed gate decision.
    pub fn all_lanes_gate_consistent(&self) -> bool {
        self.lane_rows.iter().all(|c| c.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5EntryBundleGovernanceSummary {
        let count_published = |label: EntryAssurance| {
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
        M5EntryBundleGovernanceSummary {
            total_lanes: self.lane_rows.len(),
            lane_count: self.lanes.len(),
            verified_lanes: count_published(EntryAssurance::Verified),
            bounded_lanes: count_published(EntryAssurance::Bounded),
            retest_pending_lanes: count_published(EntryAssurance::RetestPending),
            withheld_lanes: count_published(EntryAssurance::Withheld),
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
            probable_or_mixed_lanes: self
                .lane_rows
                .iter()
                .filter(|c| c.archetype_confidence.is_probable_or_mixed_trigger())
                .count(),
            stale_bundle_lanes: self
                .lane_rows
                .iter()
                .filter(|c| c.bundle_scorecard.is_stale_trigger())
                .count(),
            unsupported_topology_lanes: self
                .lane_rows
                .iter()
                .filter(|c| c.entry_topology_support.is_unsupported_trigger())
                .count(),
            lanes_with_downgrade_reasons: self
                .lane_rows
                .iter()
                .filter(|c| !c.downgrade_reasons.is_empty())
                .count(),
            lanes_with_deferred_setup: self
                .lane_rows
                .iter()
                .filter(|c| c.deferred_setup_count > 0)
                .count(),
            lanes_with_missing_roots: self
                .lane_rows
                .iter()
                .filter(|c| c.missing_root_count > 0)
                .count(),
        }
    }

    /// Produces the entry/bundle index downstream surfaces — release evidence,
    /// help/start-center, docs badges, and support exports — render instead of restating each
    /// lane's entry posture by hand.
    pub fn export_projection(&self) -> M5EntryBundleGovernanceExportProjection {
        let lanes = self
            .lane_rows
            .iter()
            .map(|c| M5EntryBundleGovernanceExportRow {
                lane_id: c.lane_id.clone(),
                lane: c.lane.as_str().to_owned(),
                entry_verb: c.entry_verb.as_str().to_owned(),
                bundle_class: c.bundle_class.as_str().to_owned(),
                locator_type: c.locator_type.as_str().to_owned(),
                owner: c.owner.clone(),
                source_trust: c.source_trust.as_str().to_owned(),
                archetype_confidence: c.archetype_confidence.as_str().to_owned(),
                root_resolution: c.root_resolution.as_str().to_owned(),
                restore_fidelity: c.restore_fidelity.as_str().to_owned(),
                bundle_scorecard: c.bundle_scorecard.as_str().to_owned(),
                entry_topology_support: c.entry_topology_support.as_str().to_owned(),
                setup_queue_class: c.setup_queue_class.as_str().to_owned(),
                deferred_setup_count: c.deferred_setup_count,
                missing_root_count: c.missing_root_count,
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
                help_surface_ref: c.help_surface_ref.clone(),
                docs_badge_ref: c.docs_badge_ref.clone(),
                support_export_ref: c.support_export_ref.clone(),
                trust_sensitive: c.lane.is_trust_sensitive(),
                verified: c.is_verified(),
                downgraded: c.is_downgraded(),
                summary: format!(
                    "{} ({}): source {}, archetype {}, roots {}, restore {}, scorecard {}, topology {}, declared {}, published {} ({}), recovery {}",
                    c.lane.as_str(),
                    c.entry_verb.as_str(),
                    c.source_trust.as_str(),
                    c.archetype_confidence.as_str(),
                    c.root_resolution.as_str(),
                    c.restore_fidelity.as_str(),
                    c.bundle_scorecard.as_str(),
                    c.entry_topology_support.as_str(),
                    c.declared_assurance.as_str(),
                    c.published_assurance.as_str(),
                    c.admission_outcome.as_str(),
                    c.downgrade_path.as_str()
                ),
            })
            .collect();
        M5EntryBundleGovernanceExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            lanes,
            all_lanes_gate_consistent: self.all_lanes_gate_consistent(),
            verified_count: self.verified_lanes().count(),
            narrowed_count: self.narrowed_lanes().count(),
            refused_count: self.refused_lanes().count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5EntryBundleGovernanceViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<EntryBundleLane> = self.lanes.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_lanes = BTreeSet::new();
        for row in &self.lane_rows {
            if !seen_ids.insert(row.lane_id.clone()) {
                violations.push(M5EntryBundleGovernanceViolation::DuplicateLaneId {
                    lane_id: row.lane_id.clone(),
                });
            }
            if !seen_lanes.insert(row.lane) {
                violations.push(M5EntryBundleGovernanceViolation::DuplicateLaneRow {
                    lane: row.lane.as_str(),
                });
            }
            if !claimed.contains(&row.lane) {
                violations.push(M5EntryBundleGovernanceViolation::UnclaimedLaneRow {
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
                violations.push(M5EntryBundleGovernanceViolation::MissingLaneRow {
                    lane: lane.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5EntryBundleGovernanceViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5EntryBundleGovernanceViolation>) {
        if self.schema_version != M5_ENTRY_BUNDLE_GOVERNANCE_SCHEMA_VERSION {
            violations.push(M5EntryBundleGovernanceViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_ENTRY_BUNDLE_GOVERNANCE_RECORD_KIND {
            violations.push(M5EntryBundleGovernanceViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("workspace_identity_scheme", &self.workspace_identity_scheme),
        ] {
            if value.trim().is_empty() {
                violations.push(M5EntryBundleGovernanceViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            ("lanes", self.lanes == EntryBundleLane::ALL.to_vec()),
            ("entry_verbs", self.entry_verbs == EntryVerb::ALL.to_vec()),
            (
                "bundle_classes",
                self.bundle_classes == BundleClass::ALL.to_vec(),
            ),
            (
                "locator_types",
                self.locator_types == LocatorType::ALL.to_vec(),
            ),
            (
                "assurance_labels",
                self.assurance_labels == EntryAssurance::ALL.to_vec(),
            ),
            (
                "source_trust_states",
                self.source_trust_states == SourceTrust::ALL.to_vec(),
            ),
            (
                "archetype_confidence_states",
                self.archetype_confidence_states == ArchetypeConfidence::ALL.to_vec(),
            ),
            (
                "root_resolution_states",
                self.root_resolution_states == RootResolution::ALL.to_vec(),
            ),
            (
                "restore_fidelities",
                self.restore_fidelities == RestoreFidelity::ALL.to_vec(),
            ),
            (
                "bundle_scorecard_states",
                self.bundle_scorecard_states == BundleScorecard::ALL.to_vec(),
            ),
            (
                "entry_topology_support_states",
                self.entry_topology_support_states == EntryTopologySupport::ALL.to_vec(),
            ),
            (
                "setup_queue_classes",
                self.setup_queue_classes == SetupQueueClass::ALL.to_vec(),
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
        ] {
            if !ok {
                violations
                    .push(M5EntryBundleGovernanceViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(
        &self,
        row: &EntryBundleRow,
        violations: &mut Vec<M5EntryBundleGovernanceViolation>,
    ) {
        for (field, value) in [
            ("lane_id", &row.lane_id),
            ("owner", &row.owner),
            ("root_id_namespace", &row.root_id_namespace),
            ("bundle_id_namespace", &row.bundle_id_namespace),
            ("packet_ref", &row.packet_ref),
            ("conformance_ref", &row.conformance_ref),
            ("evidence_ref", &row.evidence_ref),
            ("governance_receipt_ref", &row.governance_receipt_ref),
            ("release_evidence_ref", &row.release_evidence_ref),
            ("help_surface_ref", &row.help_surface_ref),
            ("docs_badge_ref", &row.docs_badge_ref),
            ("support_export_ref", &row.support_export_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5EntryBundleGovernanceViolation::EmptyField {
                    id: row.lane_id.clone(),
                    field_name: field,
                });
            }
        }

        // The lane's source packet must be the canonical entry-truth packet it governs, so a
        // lane never publishes a label its own source packet does not back.
        if row.packet_ref != row.lane.source_packet() {
            violations.push(M5EntryBundleGovernanceViolation::SourcePacketMismatch {
                lane_id: row.lane_id.clone(),
                expected: row.lane.source_packet(),
            });
        }

        // Clone, open, import, and resume stay distinct: the row's verb must equal its lane's
        // pinned verb.
        if row.entry_verb != row.lane.entry_verb() {
            violations.push(M5EntryBundleGovernanceViolation::EntryVerbMismatch {
                lane_id: row.lane_id.clone(),
                expected: row.lane.entry_verb().as_str(),
            });
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.downgrade_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5EntryBundleGovernanceViolation::DuplicateDowngradeReason {
                    lane_id: row.lane_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // The published label must equal the gate's recomputed ceiling, so an unverified,
        // probable, root-missing, partially-restored, stale, or unsupported lane can never read
        // as verified.
        let effective = row.effective_assurance();
        if row.published_assurance != effective {
            violations.push(M5EntryBundleGovernanceViolation::OverstatedClaim {
                lane_id: row.lane_id.clone(),
                published: row.published_assurance.as_str(),
                computed: effective.as_str(),
            });
        }

        // The routed admission outcome must match the gate's recomputed outcome.
        let required = row.required_outcome();
        if row.admission_outcome != required {
            violations.push(M5EntryBundleGovernanceViolation::OutcomeMismatch {
                lane_id: row.lane_id.clone(),
                declared: row.admission_outcome.as_str(),
                required: required.as_str(),
            });
        }

        // The recorded downgrade reasons must equal the reasons recomputed from the observed
        // states, so a downgrade can never be asserted or hidden by hand.
        let computed = row.computed_downgrade_reasons();
        if row.downgrade_reasons != computed {
            violations.push(M5EntryBundleGovernanceViolation::DowngradeReasonsMismatch {
                lane_id: row.lane_id.clone(),
            });
        }

        // A narrowed or refused lane must offer a real recovery path, list at least one caveat,
        // and name what is stale or narrowing, so a degraded lane never drops its recovery
        // semantics or hides why it was narrowed.
        if row.admission_outcome.is_narrowed() {
            if !row.downgrade_path.is_offered() {
                violations.push(M5EntryBundleGovernanceViolation::MissingDowngradePath {
                    lane_id: row.lane_id.clone(),
                });
            }
            if row.caveats.is_empty() {
                violations.push(M5EntryBundleGovernanceViolation::EmptyField {
                    id: row.lane_id.clone(),
                    field_name: "caveats",
                });
            }
            if row.stale_or_missing_fields.is_empty() {
                violations.push(M5EntryBundleGovernanceViolation::EmptyField {
                    id: row.lane_id.clone(),
                    field_name: "stale_or_missing_fields",
                });
            }
        }

        // A lane that still backs a publishable label must name at least one supported scope or
        // slice label.
        if row.published_assurance != EntryAssurance::Withheld && row.supported_scopes.is_empty() {
            violations.push(M5EntryBundleGovernanceViolation::EmptyField {
                id: row.lane_id.clone(),
                field_name: "supported_scopes",
            });
        }

        // Setup-queue classes stay distinct: ready hides nothing, setup-later and
        // blocked-on-setup always report a non-zero deferred-setup count, and missing-root
        // always reports a non-zero missing-root count.
        let setup_ok = match row.setup_queue_class {
            SetupQueueClass::Ready => row.deferred_setup_count == 0 && row.missing_root_count == 0,
            SetupQueueClass::SetupLater | SetupQueueClass::BlockedOnSetup => {
                row.deferred_setup_count > 0
            }
            SetupQueueClass::MissingRoot => row.missing_root_count > 0,
        };
        if !setup_ok {
            violations.push(M5EntryBundleGovernanceViolation::SetupQueueCountMismatch {
                lane_id: row.lane_id.clone(),
                class: row.setup_queue_class.as_str(),
            });
        }

        // A verified lane must be genuinely whole-trust: a first-party source, a confirmed
        // archetype, fully-resolved roots, an exact restore, a current scorecard, a supported
        // topology, a declared verified floor, no downgrade reason, no caveat, no
        // stale-or-missing field, a no-op recovery path, a ready setup queue, and nothing
        // deferred or missing. This is the non-inheritance guardrail — a lane never widens
        // trust over a probable or unverified entry.
        if row.is_verified()
            && (row.source_trust.assurance_ceiling() != EntryAssurance::Verified
                || row.archetype_confidence.assurance_ceiling() != EntryAssurance::Verified
                || row.root_resolution.assurance_ceiling() != EntryAssurance::Verified
                || row.restore_fidelity.assurance_ceiling() != EntryAssurance::Verified
                || row.bundle_scorecard.assurance_ceiling() != EntryAssurance::Verified
                || row.entry_topology_support.assurance_ceiling() != EntryAssurance::Verified
                || row.capability_floor() != EntryAssurance::Verified
                || row.setup_queue_class != SetupQueueClass::Ready
                || !row.downgrade_reasons.is_empty()
                || !row.caveats.is_empty()
                || !row.stale_or_missing_fields.is_empty()
                || row.downgrade_path.is_offered()
                || row.deferred_setup_count != 0
                || row.missing_root_count != 0)
        {
            violations.push(M5EntryBundleGovernanceViolation::VerifiedLaneNotWhole {
                lane_id: row.lane_id.clone(),
            });
        }
    }
}

/// A validation violation for the M5 entry-and-bundle governance packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5EntryBundleGovernanceViolation {
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
    /// A row's entry verb is not the canonical verb for its lane.
    EntryVerbMismatch {
        /// Row id.
        lane_id: String,
        /// Expected verb token.
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
    /// A lane's setup-queue class disagrees with its deferred-setup or missing-root counts.
    SetupQueueCountMismatch {
        /// Row id.
        lane_id: String,
        /// Setup-queue-class token.
        class: &'static str,
    },
    /// A verified lane still narrows a state, defers setup, or carries a downgrade reason.
    VerifiedLaneNotWhole {
        /// Row id.
        lane_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5EntryBundleGovernanceViolation {
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
            Self::EntryVerbMismatch { lane_id, expected } => {
                write!(
                    f,
                    "row {lane_id} entry_verb must be the canonical verb {expected}"
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
            Self::SetupQueueCountMismatch { lane_id, class } => {
                write!(
                    f,
                    "row {lane_id} setup-queue class {class} disagrees with its deferred-setup or missing-root counts"
                )
            }
            Self::VerifiedLaneNotWhole { lane_id } => {
                write!(
                    f,
                    "row {lane_id} is verified but narrows a state, defers setup, or carries a downgrade reason"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for M5EntryBundleGovernanceViolation {}

/// Loads the embedded M5 entry-and-bundle governance matrix packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5EntryBundleGovernanceMatrix`].
pub fn current_m5_entry_bundle_governance_matrix(
) -> Result<M5EntryBundleGovernanceMatrix, serde_json::Error> {
    serde_json::from_str(M5_ENTRY_BUNDLE_GOVERNANCE_JSON)
}

#[cfg(test)]
mod tests;
