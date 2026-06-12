//! Canonical M5 source-acquisition review-sheet packet: one inspectable review
//! sheet per M5 starter family that keeps clone, open, import, and resume distinct
//! verbs while disclosing source-locator, topology, cost, and follow-up-setup truth
//! before any irreversible network or disk action runs.
//!
//! Each [`SourceAcquisitionReviewSheet`] covers a single source-acquisition action and answers,
//! before the user commits: which [`EntryVerb`] it is (clone, open, import, or resume — never
//! silently rewritten), what [`SourceKind`], [`HostOrMirrorClass`], [`ProtocolClass`], and
//! [`CheckoutMode`] it resolves, where it lands ([`SourceAcquisitionReviewSheet::target_path_ref`]),
//! what it will cost ([`CostBand`]), and how trusted the source is ([`TrustStage`]). The sheet
//! then surfaces every [`TopologyCue`] — nested repository, submodule, shallow or sparse
//! checkout, LFS pointer, interrupted bootstrap, or omitted data — as an explicit state with a
//! one-step [`RecoveryActionClass`], so a partial, sparse, shallow, LFS, or interrupted entry is
//! visible and recoverable rather than looking like missing or unsupported data.
//!
//! First-useful-work routing stays explicit and nothing runs implicitly. The
//! [`FollowUpQueueItem`]s — submodule init, LFS hydrate, docs import, package-restore
//! suggestion, index warm-up, or bundle recommendation — are *previewed* with a
//! [`FollowUpRunPosture`] and an explicit [`FollowUpQueueItem::runs_implicitly`] guard that is
//! always `false`, so the sheet shows the consequence of a follow-up without performing it.
//!
//! The verb is provenance-bound and locked. Each sheet records the [`EntryVerb`] it drives, and
//! the gate forbids recording a verb that does not match the [`SourceKind`]
//! ([`EntryVerb::is_canonical_for`]); a clone is never rewritten into an open because a local
//! copy already exists, and an import is never rewritten into a resume because a bundle looks
//! resumable. Each sheet also carries its source-locator and checkout-plan provenance refs so
//! diagnostics and support exports can reconstruct exactly which locator and checkout plan
//! Aureline used on a wrong-target or half-bootstrap incident.
//!
//! Because every sheet carries diagnostics, support-export, help-surface, docs-badge, and
//! release-evidence refs, those surfaces ingest the *same* review packet rather than parallel
//! spreadsheets.
//!
//! The packet is checked in at `artifacts/workspace/m5/m5-source-acquisition-review.json` and
//! embedded here. It is metadata-only: every field is a typed state, a count, or an opaque ref,
//! and it carries no credential bodies, raw repository URLs with credentials, raw local paths,
//! or workspace contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5 source-acquisition review packet schema version.
pub const M5_SOURCE_ACQUISITION_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_SOURCE_ACQUISITION_REVIEW_RECORD_KIND: &str = "m5_source_acquisition_review_packet";

/// Repo-relative path to the checked-in packet.
pub const M5_SOURCE_ACQUISITION_REVIEW_PATH: &str =
    "artifacts/workspace/m5/m5-source-acquisition-review.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_SOURCE_ACQUISITION_REVIEW_SCHEMA_REF: &str =
    "schemas/workspace/m5-source-acquisition-review.schema.json";

/// Repo-relative path to the companion document.
pub const M5_SOURCE_ACQUISITION_REVIEW_DOC_REF: &str =
    "docs/workspace/m5/m5-source-acquisition-review.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_SOURCE_ACQUISITION_REVIEW_FIXTURE_DIR: &str =
    "fixtures/workspace/m5/m5-source-acquisition-review";

/// Embedded checked-in packet JSON.
pub const M5_SOURCE_ACQUISITION_REVIEW_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/workspace/m5/m5-source-acquisition-review.json"
));

/// The distinct verb a source-acquisition review sheet drives.
///
/// Clone, open, import, and resume remain distinct verbs; the gate forbids a sheet from
/// recording a verb its source kind does not back, so a local copy never silently turns a clone
/// into an open and a resumable-looking bundle never silently turns an import into a resume.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryVerb {
    /// Clone a remote or mirror source into a new local root.
    Clone,
    /// Open an existing local root or materialize a local template.
    Open,
    /// Import or migrate an archive or handoff bundle into a new root.
    Import,
    /// Resume a prior live session and reattach its state.
    Resume,
}

impl EntryVerb {
    /// Every entry verb, in declaration order.
    pub const ALL: [Self; 4] = [Self::Clone, Self::Open, Self::Import, Self::Resume];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clone => "clone",
            Self::Open => "open",
            Self::Import => "import",
            Self::Resume => "resume",
        }
    }

    /// Whether this verb is the canonical verb for the given source kind.
    ///
    /// This is the guardrail against silently rewriting clone into open or import into resume:
    /// a clone only fits a remote or mirror source, an open only fits a local folder or
    /// template, an import only fits an archive or handoff bundle, and a resume only fits a live
    /// session.
    pub const fn is_canonical_for(self, kind: SourceKind) -> bool {
        match self {
            Self::Clone => matches!(
                kind,
                SourceKind::RemoteRepository | SourceKind::MirrorOrProxyRepository
            ),
            Self::Open => matches!(
                kind,
                SourceKind::LocalFolder | SourceKind::TemplateOrPrebuild
            ),
            Self::Import => matches!(
                kind,
                SourceKind::ImportedArchive | SourceKind::HandoffBundle
            ),
            Self::Resume => matches!(kind, SourceKind::LiveResumeSession),
        }
    }
}

/// The M5 starter family a review sheet belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarterFamily {
    /// A template-gallery starter materialized locally.
    TemplateStarter,
    /// A framework-pack starter cloned from a remote.
    FrameworkPackStarter,
    /// A generic remote-clone starter.
    RemoteCloneStarter,
    /// A sync-handoff clone served from a mirror or proxy.
    SyncHandoff,
    /// A companion-entry handoff bundle imported into a new root.
    CompanionHandoff,
    /// A migration import from another editor or tool.
    MigrationImport,
    /// A resumed prior live session.
    SessionResume,
    /// An existing local folder opened in place.
    LocalFolderOpen,
}

impl StarterFamily {
    /// Every starter family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::TemplateStarter,
        Self::FrameworkPackStarter,
        Self::RemoteCloneStarter,
        Self::SyncHandoff,
        Self::CompanionHandoff,
        Self::MigrationImport,
        Self::SessionResume,
        Self::LocalFolderOpen,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TemplateStarter => "template_starter",
            Self::FrameworkPackStarter => "framework_pack_starter",
            Self::RemoteCloneStarter => "remote_clone_starter",
            Self::SyncHandoff => "sync_handoff",
            Self::CompanionHandoff => "companion_handoff",
            Self::MigrationImport => "migration_import",
            Self::SessionResume => "session_resume",
            Self::LocalFolderOpen => "local_folder_open",
        }
    }
}

/// The kind of source a review sheet acquires its workspace from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    /// An existing local folder.
    LocalFolder,
    /// A local template or prebuild materialized into a new root.
    TemplateOrPrebuild,
    /// A live remote repository.
    RemoteRepository,
    /// A mirror or proxy of a remote repository.
    MirrorOrProxyRepository,
    /// An imported archive from another tool.
    ImportedArchive,
    /// A handoff bundle from a companion session.
    HandoffBundle,
    /// A live, resumable managed session.
    LiveResumeSession,
}

impl SourceKind {
    /// Every source kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LocalFolder,
        Self::TemplateOrPrebuild,
        Self::RemoteRepository,
        Self::MirrorOrProxyRepository,
        Self::ImportedArchive,
        Self::HandoffBundle,
        Self::LiveResumeSession,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFolder => "local_folder",
            Self::TemplateOrPrebuild => "template_or_prebuild",
            Self::RemoteRepository => "remote_repository",
            Self::MirrorOrProxyRepository => "mirror_or_proxy_repository",
            Self::ImportedArchive => "imported_archive",
            Self::HandoffBundle => "handoff_bundle",
            Self::LiveResumeSession => "live_resume_session",
        }
    }
}

/// The host or mirror class a sheet resolves its source from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostOrMirrorClass {
    /// The local filesystem.
    LocalFilesystem,
    /// A live forge or generic remote host.
    ForgeHost,
    /// An explicit mirror.
    Mirror,
    /// An explicit proxy.
    Proxy,
    /// Offline or air-gapped media.
    OfflineMedia,
    /// A managed live workspace or remote attach target.
    ManagedWorkspace,
    /// An imported archive, bundle, or handoff artifact.
    ImportedArtifact,
    /// No host class applies.
    NotApplicable,
}

impl HostOrMirrorClass {
    /// Every host-or-mirror class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::LocalFilesystem,
        Self::ForgeHost,
        Self::Mirror,
        Self::Proxy,
        Self::OfflineMedia,
        Self::ManagedWorkspace,
        Self::ImportedArtifact,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFilesystem => "local_filesystem",
            Self::ForgeHost => "forge_host",
            Self::Mirror => "mirror",
            Self::Proxy => "proxy",
            Self::OfflineMedia => "offline_media",
            Self::ManagedWorkspace => "managed_workspace",
            Self::ImportedArtifact => "imported_artifact",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// The transport protocol a sheet uses, when applicable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolClass {
    /// HTTPS transport.
    Https,
    /// SSH transport.
    Ssh,
    /// Native git protocol.
    GitProtocol,
    /// Local filesystem access.
    FileLocal,
    /// A mirror-specific transport.
    MirrorProtocol,
    /// A proxy-routed transport.
    ProxyProtocol,
    /// Air-gapped media transfer.
    AirGappedMedia,
    /// No transport protocol applies.
    NotApplicable,
}

impl ProtocolClass {
    /// Every protocol class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Https,
        Self::Ssh,
        Self::GitProtocol,
        Self::FileLocal,
        Self::MirrorProtocol,
        Self::ProxyProtocol,
        Self::AirGappedMedia,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Https => "https",
            Self::Ssh => "ssh",
            Self::GitProtocol => "git_protocol",
            Self::FileLocal => "file_local",
            Self::MirrorProtocol => "mirror_protocol",
            Self::ProxyProtocol => "proxy_protocol",
            Self::AirGappedMedia => "air_gapped_media",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// The checkout shape a sheet plans before acquisition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutMode {
    /// Full history and working tree.
    FullCheckout,
    /// Shallow history.
    ShallowHistory,
    /// Partial clone with a promisor filter.
    PartialClone,
    /// Sparse checkout narrowed to a workset.
    SparseCheckout,
    /// Archive extraction into a new root.
    ArchiveExtract,
    /// Live attach to a managed session.
    LiveAttach,
    /// No checkout shape applies.
    NotApplicable,
}

impl CheckoutMode {
    /// Every checkout mode, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FullCheckout,
        Self::ShallowHistory,
        Self::PartialClone,
        Self::SparseCheckout,
        Self::ArchiveExtract,
        Self::LiveAttach,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullCheckout => "full_checkout",
            Self::ShallowHistory => "shallow_history",
            Self::PartialClone => "partial_clone",
            Self::SparseCheckout => "sparse_checkout",
            Self::ArchiveExtract => "archive_extract",
            Self::LiveAttach => "live_attach",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// The expected network and disk cost band a sheet discloses before acquisition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CostBand {
    /// No network fetch; only local work.
    LocalNoFetch,
    /// A light fetch.
    LightFetch,
    /// A moderate fetch.
    ModerateFetch,
    /// A heavy fetch.
    HeavyFetch,
    /// A very heavy fetch.
    VeryHeavyFetch,
}

impl CostBand {
    /// Every cost band, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalNoFetch,
        Self::LightFetch,
        Self::ModerateFetch,
        Self::HeavyFetch,
        Self::VeryHeavyFetch,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalNoFetch => "local_no_fetch",
            Self::LightFetch => "light_fetch",
            Self::ModerateFetch => "moderate_fetch",
            Self::HeavyFetch => "heavy_fetch",
            Self::VeryHeavyFetch => "very_heavy_fetch",
        }
    }

    /// Whether this band implies an irreversible network fetch before first-useful-work.
    pub const fn implies_network_fetch(self) -> bool {
        !matches!(self, Self::LocalNoFetch)
    }
}

/// How trusted the source a sheet acquires from is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustStage {
    /// A first-party, already-trusted source.
    FirstPartyTrusted,
    /// A trusted continuation of a prior acquisition.
    TrustedContinuation,
    /// A source that needs trust review before admission.
    ReviewRequired,
    /// An untrusted source, browse-only until admitted.
    UntrustedBrowseOnly,
}

impl TrustStage {
    /// Every trust stage, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FirstPartyTrusted,
        Self::TrustedContinuation,
        Self::ReviewRequired,
        Self::UntrustedBrowseOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyTrusted => "first_party_trusted",
            Self::TrustedContinuation => "trusted_continuation",
            Self::ReviewRequired => "review_required",
            Self::UntrustedBrowseOnly => "untrusted_browse_only",
        }
    }
}

/// The kind of topology cue a sheet surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyCueKind {
    /// A nested repository under the destination root.
    NestedRepo,
    /// A declared submodule.
    Submodule,
    /// Shallow history.
    ShallowHistory,
    /// A sparse checkout narrowed to a workset.
    SparseCheckout,
    /// An LFS-pointer asset not yet hydrated.
    LfsPointer,
    /// An interrupted bootstrap with a resumable checkpoint.
    InterruptedBootstrap,
    /// Omitted data behind a promisor or partial bundle.
    OmittedData,
}

impl TopologyCueKind {
    /// Every topology cue kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::NestedRepo,
        Self::Submodule,
        Self::ShallowHistory,
        Self::SparseCheckout,
        Self::LfsPointer,
        Self::InterruptedBootstrap,
        Self::OmittedData,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NestedRepo => "nested_repo",
            Self::Submodule => "submodule",
            Self::ShallowHistory => "shallow_history",
            Self::SparseCheckout => "sparse_checkout",
            Self::LfsPointer => "lfs_pointer",
            Self::InterruptedBootstrap => "interrupted_bootstrap",
            Self::OmittedData => "omitted_data",
        }
    }
}

/// The observed state of a topology cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyCueState {
    /// The cue is active and shapes the working tree now.
    Active,
    /// The cue is recorded but its action has not yet run.
    Pending,
    /// The cue applies partially.
    Partial,
    /// The cue does not apply to this source.
    NotPresent,
}

impl TopologyCueState {
    /// Every topology cue state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Active, Self::Pending, Self::Partial, Self::NotPresent];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Pending => "pending",
            Self::Partial => "partial",
            Self::NotPresent => "not_present",
        }
    }

    /// Whether the cue applies to this source at all.
    pub const fn applies(self) -> bool {
        !matches!(self, Self::NotPresent)
    }
}

/// The one-step recovery or widen action a topology cue offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionClass {
    /// Initialize declared submodules.
    InitSubmodules,
    /// Hydrate LFS-pointer assets.
    HydrateLfs,
    /// Widen the sparse-checkout scope.
    WidenSparseScope,
    /// Fetch the full history of a shallow clone.
    UnshallowHistory,
    /// Resume an interrupted bootstrap.
    ResumeBootstrap,
    /// Fetch omitted data on demand.
    FetchOmittedData,
    /// Review a detected nested repository.
    ReviewNestedRepo,
    /// No recovery action applies.
    None,
}

impl RecoveryActionClass {
    /// Every recovery action class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::InitSubmodules,
        Self::HydrateLfs,
        Self::WidenSparseScope,
        Self::UnshallowHistory,
        Self::ResumeBootstrap,
        Self::FetchOmittedData,
        Self::ReviewNestedRepo,
        Self::None,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InitSubmodules => "init_submodules",
            Self::HydrateLfs => "hydrate_lfs",
            Self::WidenSparseScope => "widen_sparse_scope",
            Self::UnshallowHistory => "unshallow_history",
            Self::ResumeBootstrap => "resume_bootstrap",
            Self::FetchOmittedData => "fetch_omitted_data",
            Self::ReviewNestedRepo => "review_nested_repo",
            Self::None => "none",
        }
    }

    /// Whether a recovery action is offered.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// The kind of follow-up setup-queue item a sheet previews.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowUpKind {
    /// Initialize submodules.
    SubmoduleInit,
    /// Hydrate LFS assets.
    LfsHydrate,
    /// Import a docs pack.
    DocsImport,
    /// Suggest restoring packages.
    PackageRestoreSuggestion,
    /// Warm the symbol index.
    IndexWarmUp,
    /// Recommend a workflow bundle.
    BundleRecommendation,
}

impl FollowUpKind {
    /// Every follow-up kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SubmoduleInit,
        Self::LfsHydrate,
        Self::DocsImport,
        Self::PackageRestoreSuggestion,
        Self::IndexWarmUp,
        Self::BundleRecommendation,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SubmoduleInit => "submodule_init",
            Self::LfsHydrate => "lfs_hydrate",
            Self::DocsImport => "docs_import",
            Self::PackageRestoreSuggestion => "package_restore_suggestion",
            Self::IndexWarmUp => "index_warm_up",
            Self::BundleRecommendation => "bundle_recommendation",
        }
    }
}

/// How a previewed follow-up item is queued — it is never run implicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowUpRunPosture {
    /// Deferred to an explicit follow-up queue run.
    Deferred,
    /// Awaiting a user action before it can run.
    AwaitingUserAction,
    /// Awaiting trust admission before it can run.
    AwaitingTrustAdmission,
    /// A suggestion the user may opt into.
    Suggested,
}

impl FollowUpRunPosture {
    /// Every follow-up run posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Deferred,
        Self::AwaitingUserAction,
        Self::AwaitingTrustAdmission,
        Self::Suggested,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deferred => "deferred",
            Self::AwaitingUserAction => "awaiting_user_action",
            Self::AwaitingTrustAdmission => "awaiting_trust_admission",
            Self::Suggested => "suggested",
        }
    }

    /// Whether this posture is a deferred setup step rather than a bare suggestion.
    ///
    /// A deferred step forces the sheet to require review before acquisition; a suggestion does
    /// not on its own.
    pub const fn is_deferred_setup(self) -> bool {
        !matches!(self, Self::Suggested)
    }
}

/// One topology cue surfaced by a review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TopologyCue {
    /// Which topology condition this cue describes.
    pub cue_kind: TopologyCueKind,
    /// The observed state of the cue.
    pub state: TopologyCueState,
    /// A human-readable, redacted detail label.
    pub detail_label: String,
    /// The one-step recovery or widen action offered.
    pub recovery_action: RecoveryActionClass,
    /// Whether the cue is one-step recoverable.
    pub recoverable: bool,
    /// Whether the cue blocks first-useful-work until it is acted on.
    pub blocks_first_useful_work: bool,
}

impl TopologyCue {
    /// Whether this cue is internally consistent.
    ///
    /// A cue that applies must offer a recovery action; a cue that does not apply must not; and
    /// a cue that blocks first-useful-work must be recoverable so the block is never a dead end.
    pub fn is_consistent(&self) -> bool {
        if self.detail_label.trim().is_empty() {
            return false;
        }
        if self.state.applies() {
            if !self.recovery_action.is_offered() {
                return false;
            }
        } else if self.recovery_action.is_offered()
            || self.recoverable
            || self.blocks_first_useful_work
        {
            return false;
        }
        if self.blocks_first_useful_work && !self.recoverable {
            return false;
        }
        true
    }
}

/// One previewed follow-up setup-queue item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FollowUpQueueItem {
    /// Which follow-up setup step this item previews.
    pub item_kind: FollowUpKind,
    /// How the item is queued.
    pub run_posture: FollowUpRunPosture,
    /// Guardrail: the item never runs implicitly. Always `false`.
    pub runs_implicitly: bool,
    /// A human-readable, one-step action label.
    pub one_step_action_label: String,
    /// Opaque scope ref the item applies to.
    pub scope_ref: String,
    /// Opaque evidence ref backing the item.
    pub evidence_ref: String,
}

impl FollowUpQueueItem {
    /// Whether this item is internally consistent and never runs implicitly.
    pub fn is_consistent(&self) -> bool {
        !self.runs_implicitly
            && !self.one_step_action_label.trim().is_empty()
            && !self.scope_ref.trim().is_empty()
            && !self.evidence_ref.trim().is_empty()
    }
}

/// One source-acquisition review sheet for a single clone, open, import, or resume action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceAcquisitionReviewSheet {
    /// Stable sheet identifier.
    pub sheet_id: String,
    /// The M5 starter family this sheet belongs to.
    pub starter_family: StarterFamily,
    /// The distinct verb the sheet drives.
    pub entry_verb: EntryVerb,
    /// The kind of source the sheet acquires from.
    pub source_kind: SourceKind,
    /// The host or mirror class.
    pub host_or_mirror_class: HostOrMirrorClass,
    /// The transport protocol.
    pub protocol: ProtocolClass,
    /// The planned checkout shape.
    pub checkout_mode: CheckoutMode,
    /// Opaque destination-path identity.
    pub target_path_ref: String,
    /// The disclosed cost band.
    pub expected_cost_band: CostBand,
    /// The source trust stage.
    pub trust_stage: TrustStage,
    /// Whether review is required before any irreversible network or disk action.
    pub review_required_before_acquisition: bool,
    /// Whether the recorded verb is locked against silent rewriting.
    pub verb_locked: bool,
    /// Whether a local copy already exists; recorded so a clone is never silently reopened.
    pub local_copy_present: bool,
    /// Opaque source-locator provenance ref for diagnostics and support export.
    pub source_locator_ref: String,
    /// Opaque checkout-plan provenance ref for diagnostics and support export.
    pub checkout_plan_ref: String,
    /// Accountable owner.
    pub owner: String,
    /// Topology cues surfaced before acquisition.
    pub topology_cues: Vec<TopologyCue>,
    /// Previewed follow-up setup-queue items.
    pub follow_up_queue: Vec<FollowUpQueueItem>,
    /// Caveats shown on the sheet.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Diagnostics surface ref.
    pub diagnostics_ref: String,
    /// Support-export surface ref.
    pub support_export_ref: String,
    /// Help surface ref.
    pub help_surface_ref: String,
    /// Docs-badge surface ref.
    pub docs_badge_ref: String,
    /// Release-evidence surface ref.
    pub release_evidence_ref: String,
    /// A reviewer note summarizing the sheet.
    pub note: String,
}

impl SourceAcquisitionReviewSheet {
    /// Whether the recorded verb is canonical for the source kind.
    pub fn verb_is_canonical(&self) -> bool {
        self.entry_verb.is_canonical_for(self.source_kind)
    }

    /// Topology cues that are currently active.
    pub fn active_cues(&self) -> impl Iterator<Item = &TopologyCue> {
        self.topology_cues
            .iter()
            .filter(|cue| cue.state == TopologyCueState::Active)
    }

    /// Topology cues that apply at all (active, pending, or partial).
    pub fn applicable_cues(&self) -> impl Iterator<Item = &TopologyCue> {
        self.topology_cues.iter().filter(|cue| cue.state.applies())
    }

    /// Topology cues that block first-useful-work.
    pub fn blocking_cues(&self) -> impl Iterator<Item = &TopologyCue> {
        self.topology_cues
            .iter()
            .filter(|cue| cue.blocks_first_useful_work)
    }

    /// Whether any applicable topology cue is present.
    pub fn has_applicable_cue(&self) -> bool {
        self.applicable_cues().next().is_some()
    }

    /// Whether any follow-up item is a deferred setup step (not a bare suggestion).
    pub fn has_deferred_follow_up(&self) -> bool {
        self.follow_up_queue
            .iter()
            .any(|item| item.run_posture.is_deferred_setup())
    }

    /// Whether any follow-up item would run implicitly — always a violation if true.
    pub fn has_implicit_follow_up(&self) -> bool {
        self.follow_up_queue.iter().any(|item| item.runs_implicitly)
    }

    /// The review requirement the gate computes from the sheet's observed states.
    ///
    /// A clone, import, or resume always requires review; so does any sheet that implies a
    /// network fetch, surfaces an applicable topology cue, or queues a deferred setup step. A
    /// clean local open with no fetch, no cue, and no deferred step requires no review.
    pub fn computed_review_required(&self) -> bool {
        !matches!(self.entry_verb, EntryVerb::Open)
            || self.expected_cost_band.implies_network_fetch()
            || self.has_applicable_cue()
            || self.has_deferred_follow_up()
    }

    /// Whether the sheet carries complete source-locator and checkout-plan provenance.
    pub fn provenance_complete(&self) -> bool {
        !self.source_locator_ref.trim().is_empty() && !self.checkout_plan_ref.trim().is_empty()
    }

    /// Whether caveats are required on this sheet — when any cue applies or a setup step defers.
    pub fn caveats_required(&self) -> bool {
        self.has_applicable_cue() || self.has_deferred_follow_up()
    }
}

/// Summary counts rolled up across every review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SourceAcquisitionReviewSummary {
    /// Total review sheets.
    pub total_sheets: usize,
    /// Sheet count (equals `total_sheets`; kept for export parity).
    pub sheet_count: usize,
    /// Sheets driving a clone verb.
    pub clone_sheets: usize,
    /// Sheets driving an open verb.
    pub open_sheets: usize,
    /// Sheets driving an import verb.
    pub import_sheets: usize,
    /// Sheets driving a resume verb.
    pub resume_sheets: usize,
    /// Sheets that require review before acquisition.
    pub sheets_requiring_review: usize,
    /// Sheets carrying at least one active topology cue.
    pub sheets_with_active_topology_cues: usize,
    /// Sheets carrying at least one cue that blocks first-useful-work.
    pub sheets_with_blocking_cues: usize,
    /// Sheets carrying at least one deferred follow-up setup step.
    pub sheets_with_deferred_follow_ups: usize,
    /// Total topology cues across all sheets.
    pub total_topology_cues: usize,
    /// Total follow-up items across all sheets.
    pub total_follow_up_items: usize,
    /// Topology cues that are one-step recoverable.
    pub recoverable_cue_count: usize,
    /// Sheets whose verb is locked against silent rewriting.
    pub verb_locked_sheets: usize,
    /// Sheets where a local copy already exists.
    pub local_copy_present_sheets: usize,
}

/// One redaction-safe export row projected from a review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SourceAcquisitionReviewExportRow {
    /// Sheet id.
    pub sheet_id: String,
    /// Starter family token.
    pub starter_family: StarterFamily,
    /// Entry verb token.
    pub entry_verb: EntryVerb,
    /// Source kind token.
    pub source_kind: SourceKind,
    /// Host or mirror class token.
    pub host_or_mirror_class: HostOrMirrorClass,
    /// Protocol token.
    pub protocol: ProtocolClass,
    /// Checkout mode token.
    pub checkout_mode: CheckoutMode,
    /// Cost band token.
    pub expected_cost_band: CostBand,
    /// Trust stage token.
    pub trust_stage: TrustStage,
    /// Whether review is required before acquisition.
    pub review_required_before_acquisition: bool,
    /// Source-locator provenance ref.
    pub source_locator_ref: String,
    /// Checkout-plan provenance ref.
    pub checkout_plan_ref: String,
    /// Applicable topology cue kinds.
    pub topology_cue_kinds: Vec<TopologyCueKind>,
    /// Follow-up item kinds previewed.
    pub follow_up_kinds: Vec<FollowUpKind>,
    /// Whether the verb is canonical and locked.
    pub verb_canonical_and_locked: bool,
}

/// A redaction-safe export projection of the whole packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SourceAcquisitionReviewExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected sheet rows.
    pub sheets: Vec<M5SourceAcquisitionReviewExportRow>,
    /// Whether every sheet is gate-consistent.
    pub all_sheets_consistent: bool,
    /// Sheets requiring review.
    pub sheets_requiring_review: usize,
    /// Sheets with an active topology cue.
    pub sheets_with_active_topology_cues: usize,
}

/// The typed M5 source-acquisition review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SourceAcquisitionReviewPacket {
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
    /// Scheme the packet mints stable locator identities under.
    pub locator_identity_scheme: String,
    /// Closed entry-verb vocabulary.
    pub entry_verbs: Vec<EntryVerb>,
    /// Closed starter-family vocabulary.
    pub starter_families: Vec<StarterFamily>,
    /// Closed source-kind vocabulary.
    pub source_kinds: Vec<SourceKind>,
    /// Closed host-or-mirror-class vocabulary.
    pub host_or_mirror_classes: Vec<HostOrMirrorClass>,
    /// Closed protocol vocabulary.
    pub protocols: Vec<ProtocolClass>,
    /// Closed checkout-mode vocabulary.
    pub checkout_modes: Vec<CheckoutMode>,
    /// Closed cost-band vocabulary.
    pub cost_bands: Vec<CostBand>,
    /// Closed trust-stage vocabulary.
    pub trust_stages: Vec<TrustStage>,
    /// Closed topology-cue-kind vocabulary.
    pub topology_cue_kinds: Vec<TopologyCueKind>,
    /// Closed topology-cue-state vocabulary.
    pub topology_cue_states: Vec<TopologyCueState>,
    /// Closed recovery-action vocabulary.
    pub recovery_actions: Vec<RecoveryActionClass>,
    /// Closed follow-up-kind vocabulary.
    pub follow_up_kinds: Vec<FollowUpKind>,
    /// Closed follow-up-posture vocabulary.
    pub follow_up_postures: Vec<FollowUpRunPosture>,
    /// Review sheets, one per source-acquisition action.
    #[serde(default)]
    pub sheets: Vec<SourceAcquisitionReviewSheet>,
    /// Summary counts.
    pub summary: M5SourceAcquisitionReviewSummary,
}

impl M5SourceAcquisitionReviewPacket {
    /// Returns the sheet with the given id.
    pub fn sheet(&self, sheet_id: &str) -> Option<&SourceAcquisitionReviewSheet> {
        self.sheets.iter().find(|s| s.sheet_id == sheet_id)
    }

    /// Sheets driving the given verb.
    pub fn sheets_with_verb(
        &self,
        verb: EntryVerb,
    ) -> impl Iterator<Item = &SourceAcquisitionReviewSheet> {
        self.sheets.iter().filter(move |s| s.entry_verb == verb)
    }

    /// Whether every sheet is internally consistent against the gate.
    pub fn all_sheets_consistent(&self) -> bool {
        self.sheets.iter().all(|sheet| {
            sheet.verb_is_canonical()
                && sheet.verb_locked
                && !sheet.has_implicit_follow_up()
                && sheet.provenance_complete()
                && sheet.review_required_before_acquisition == sheet.computed_review_required()
                && sheet.topology_cues.iter().all(TopologyCue::is_consistent)
                && sheet
                    .follow_up_queue
                    .iter()
                    .all(FollowUpQueueItem::is_consistent)
        })
    }

    /// Recomputes the summary from the sheets.
    pub fn computed_summary(&self) -> M5SourceAcquisitionReviewSummary {
        let count_verb = |verb: EntryVerb| self.sheets_with_verb(verb).count();
        M5SourceAcquisitionReviewSummary {
            total_sheets: self.sheets.len(),
            sheet_count: self.sheets.len(),
            clone_sheets: count_verb(EntryVerb::Clone),
            open_sheets: count_verb(EntryVerb::Open),
            import_sheets: count_verb(EntryVerb::Import),
            resume_sheets: count_verb(EntryVerb::Resume),
            sheets_requiring_review: self
                .sheets
                .iter()
                .filter(|s| s.review_required_before_acquisition)
                .count(),
            sheets_with_active_topology_cues: self
                .sheets
                .iter()
                .filter(|s| s.active_cues().next().is_some())
                .count(),
            sheets_with_blocking_cues: self
                .sheets
                .iter()
                .filter(|s| s.blocking_cues().next().is_some())
                .count(),
            sheets_with_deferred_follow_ups: self
                .sheets
                .iter()
                .filter(|s| s.has_deferred_follow_up())
                .count(),
            total_topology_cues: self.sheets.iter().map(|s| s.topology_cues.len()).sum(),
            total_follow_up_items: self.sheets.iter().map(|s| s.follow_up_queue.len()).sum(),
            recoverable_cue_count: self
                .sheets
                .iter()
                .flat_map(|s| s.topology_cues.iter())
                .filter(|cue| cue.recoverable)
                .count(),
            verb_locked_sheets: self.sheets.iter().filter(|s| s.verb_locked).count(),
            local_copy_present_sheets: self.sheets.iter().filter(|s| s.local_copy_present).count(),
        }
    }

    /// Projects a redaction-safe export view of the packet.
    pub fn export_projection(&self) -> M5SourceAcquisitionReviewExportProjection {
        let sheets = self
            .sheets
            .iter()
            .map(|sheet| M5SourceAcquisitionReviewExportRow {
                sheet_id: sheet.sheet_id.clone(),
                starter_family: sheet.starter_family,
                entry_verb: sheet.entry_verb,
                source_kind: sheet.source_kind,
                host_or_mirror_class: sheet.host_or_mirror_class,
                protocol: sheet.protocol,
                checkout_mode: sheet.checkout_mode,
                expected_cost_band: sheet.expected_cost_band,
                trust_stage: sheet.trust_stage,
                review_required_before_acquisition: sheet.review_required_before_acquisition,
                source_locator_ref: sheet.source_locator_ref.clone(),
                checkout_plan_ref: sheet.checkout_plan_ref.clone(),
                topology_cue_kinds: sheet.applicable_cues().map(|cue| cue.cue_kind).collect(),
                follow_up_kinds: sheet
                    .follow_up_queue
                    .iter()
                    .map(|item| item.item_kind)
                    .collect(),
                verb_canonical_and_locked: sheet.verb_is_canonical() && sheet.verb_locked,
            })
            .collect();
        M5SourceAcquisitionReviewExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            sheets,
            all_sheets_consistent: self.all_sheets_consistent(),
            sheets_requiring_review: self.computed_summary().sheets_requiring_review,
            sheets_with_active_topology_cues: self
                .computed_summary()
                .sheets_with_active_topology_cues,
        }
    }

    /// Validates the packet against its honesty contract, returning every violation.
    pub fn validate(&self) -> Vec<M5SourceAcquisitionReviewViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        let mut seen_ids = BTreeSet::new();
        for sheet in &self.sheets {
            if !seen_ids.insert(sheet.sheet_id.clone()) {
                violations.push(M5SourceAcquisitionReviewViolation::DuplicateSheetId {
                    sheet_id: sheet.sheet_id.clone(),
                });
            }
            self.validate_sheet(sheet, &mut violations);
        }
        if self.summary != self.computed_summary() {
            violations.push(M5SourceAcquisitionReviewViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5SourceAcquisitionReviewViolation>) {
        if self.schema_version != M5_SOURCE_ACQUISITION_REVIEW_SCHEMA_VERSION {
            violations.push(M5SourceAcquisitionReviewViolation::SchemaVersionMismatch {
                found: self.schema_version,
            });
        }
        if self.record_kind != M5_SOURCE_ACQUISITION_REVIEW_RECORD_KIND {
            violations.push(M5SourceAcquisitionReviewViolation::RecordKindMismatch {
                found: self.record_kind.clone(),
            });
        }
        let vocab_ok = self.entry_verbs == EntryVerb::ALL
            && self.starter_families == StarterFamily::ALL
            && self.source_kinds == SourceKind::ALL
            && self.host_or_mirror_classes == HostOrMirrorClass::ALL
            && self.protocols == ProtocolClass::ALL
            && self.checkout_modes == CheckoutMode::ALL
            && self.cost_bands == CostBand::ALL
            && self.trust_stages == TrustStage::ALL
            && self.topology_cue_kinds == TopologyCueKind::ALL
            && self.topology_cue_states == TopologyCueState::ALL
            && self.recovery_actions == RecoveryActionClass::ALL
            && self.follow_up_kinds == FollowUpKind::ALL
            && self.follow_up_postures == FollowUpRunPosture::ALL;
        if !vocab_ok {
            violations.push(M5SourceAcquisitionReviewViolation::VocabularyMismatch);
        }
        if self.sheets.is_empty() {
            violations.push(M5SourceAcquisitionReviewViolation::NoSheets);
        }
    }

    fn validate_sheet(
        &self,
        sheet: &SourceAcquisitionReviewSheet,
        violations: &mut Vec<M5SourceAcquisitionReviewViolation>,
    ) {
        let id = sheet.sheet_id.clone();
        if id.trim().is_empty() {
            violations.push(M5SourceAcquisitionReviewViolation::EmptySheetId);
        }
        if !sheet.verb_is_canonical() {
            violations.push(M5SourceAcquisitionReviewViolation::VerbNotCanonical {
                sheet_id: id.clone(),
                verb: sheet.entry_verb,
                source_kind: sheet.source_kind,
            });
        }
        if !sheet.verb_locked {
            violations.push(M5SourceAcquisitionReviewViolation::VerbNotLocked {
                sheet_id: id.clone(),
            });
        }
        if sheet.has_implicit_follow_up() {
            violations.push(M5SourceAcquisitionReviewViolation::ImplicitFollowUp {
                sheet_id: id.clone(),
            });
        }
        if !sheet.provenance_complete() {
            violations.push(M5SourceAcquisitionReviewViolation::MissingProvenance {
                sheet_id: id.clone(),
            });
        }
        if sheet.review_required_before_acquisition != sheet.computed_review_required() {
            violations.push(
                M5SourceAcquisitionReviewViolation::ReviewRequirementMismatch {
                    sheet_id: id.clone(),
                    recorded: sheet.review_required_before_acquisition,
                    computed: sheet.computed_review_required(),
                },
            );
        }
        for cue in &sheet.topology_cues {
            if !cue.is_consistent() {
                violations.push(M5SourceAcquisitionReviewViolation::InconsistentCue {
                    sheet_id: id.clone(),
                    cue_kind: cue.cue_kind,
                });
            }
        }
        for item in &sheet.follow_up_queue {
            if !item.is_consistent() {
                violations.push(M5SourceAcquisitionReviewViolation::InconsistentFollowUp {
                    sheet_id: id.clone(),
                    item_kind: item.item_kind,
                });
            }
        }
        if sheet.caveats_required() && sheet.caveats.iter().all(|c| c.trim().is_empty()) {
            violations.push(M5SourceAcquisitionReviewViolation::MissingCaveat {
                sheet_id: id.clone(),
            });
        }
        let surface_refs = [
            &sheet.target_path_ref,
            &sheet.owner,
            &sheet.diagnostics_ref,
            &sheet.support_export_ref,
            &sheet.help_surface_ref,
            &sheet.docs_badge_ref,
            &sheet.release_evidence_ref,
            &sheet.note,
        ];
        if surface_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(M5SourceAcquisitionReviewViolation::MissingSurfaceRef { sheet_id: id });
        }
    }
}

/// A single way the packet can fail its honesty contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SourceAcquisitionReviewViolation {
    /// The schema version does not match the supported version.
    SchemaVersionMismatch {
        /// The version found in the packet.
        found: u32,
    },
    /// The record kind does not match the canonical tag.
    RecordKindMismatch {
        /// The record kind found in the packet.
        found: String,
    },
    /// A closed vocabulary array does not match its canonical `ALL`.
    VocabularyMismatch,
    /// The packet carries no review sheets.
    NoSheets,
    /// Two sheets share a sheet id.
    DuplicateSheetId {
        /// The duplicated id.
        sheet_id: String,
    },
    /// A sheet id is empty.
    EmptySheetId,
    /// A sheet records a verb that is not canonical for its source kind.
    VerbNotCanonical {
        /// The offending sheet id.
        sheet_id: String,
        /// The recorded verb.
        verb: EntryVerb,
        /// The recorded source kind.
        source_kind: SourceKind,
    },
    /// A sheet's verb is not locked against silent rewriting.
    VerbNotLocked {
        /// The offending sheet id.
        sheet_id: String,
    },
    /// A sheet previews a follow-up item flagged to run implicitly.
    ImplicitFollowUp {
        /// The offending sheet id.
        sheet_id: String,
    },
    /// A sheet lacks complete source-locator and checkout-plan provenance.
    MissingProvenance {
        /// The offending sheet id.
        sheet_id: String,
    },
    /// A sheet's recorded review requirement diverges from the gate.
    ReviewRequirementMismatch {
        /// The offending sheet id.
        sheet_id: String,
        /// The recorded value.
        recorded: bool,
        /// The recomputed value.
        computed: bool,
    },
    /// A topology cue is internally inconsistent.
    InconsistentCue {
        /// The offending sheet id.
        sheet_id: String,
        /// The cue kind.
        cue_kind: TopologyCueKind,
    },
    /// A follow-up item is internally inconsistent.
    InconsistentFollowUp {
        /// The offending sheet id.
        sheet_id: String,
        /// The item kind.
        item_kind: FollowUpKind,
    },
    /// A sheet that needs a caveat carries none.
    MissingCaveat {
        /// The offending sheet id.
        sheet_id: String,
    },
    /// A sheet is missing a required surface or owner ref.
    MissingSurfaceRef {
        /// The offending sheet id.
        sheet_id: String,
    },
    /// The recorded summary diverges from the recomputed summary.
    SummaryMismatch,
}

impl fmt::Display for M5SourceAcquisitionReviewViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersionMismatch { found } => {
                write!(f, "schema_version mismatch: found {found}")
            }
            Self::RecordKindMismatch { found } => {
                write!(f, "record_kind mismatch: found {found}")
            }
            Self::VocabularyMismatch => {
                write!(
                    f,
                    "a closed vocabulary array diverges from its canonical set"
                )
            }
            Self::NoSheets => write!(f, "packet carries no review sheets"),
            Self::DuplicateSheetId { sheet_id } => {
                write!(f, "duplicate sheet id: {sheet_id}")
            }
            Self::EmptySheetId => write!(f, "a sheet has an empty id"),
            Self::VerbNotCanonical {
                sheet_id,
                verb,
                source_kind,
            } => write!(
                f,
                "sheet {sheet_id} records verb {} which is not canonical for source kind {}",
                verb.as_str(),
                source_kind.as_str()
            ),
            Self::VerbNotLocked { sheet_id } => {
                write!(f, "sheet {sheet_id} verb is not locked")
            }
            Self::ImplicitFollowUp { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} previews a follow-up that runs implicitly"
                )
            }
            Self::MissingProvenance { sheet_id } => write!(
                f,
                "sheet {sheet_id} lacks source-locator or checkout-plan provenance"
            ),
            Self::ReviewRequirementMismatch {
                sheet_id,
                recorded,
                computed,
            } => write!(
                f,
                "sheet {sheet_id} review requirement {recorded} diverges from gate {computed}"
            ),
            Self::InconsistentCue { sheet_id, cue_kind } => write!(
                f,
                "sheet {sheet_id} topology cue {} is inconsistent",
                cue_kind.as_str()
            ),
            Self::InconsistentFollowUp {
                sheet_id,
                item_kind,
            } => write!(
                f,
                "sheet {sheet_id} follow-up {} is inconsistent",
                item_kind.as_str()
            ),
            Self::MissingCaveat { sheet_id } => {
                write!(f, "sheet {sheet_id} needs a caveat but carries none")
            }
            Self::MissingSurfaceRef { sheet_id } => {
                write!(
                    f,
                    "sheet {sheet_id} is missing a required surface or owner ref"
                )
            }
            Self::SummaryMismatch => write!(f, "summary diverges from the recomputed summary"),
        }
    }
}

impl Error for M5SourceAcquisitionReviewViolation {}

/// Loads the embedded canonical M5 source-acquisition review packet.
///
/// # Errors
///
/// Returns a deserialization error if the embedded JSON does not parse into the typed packet.
pub fn current_m5_source_acquisition_review_packet(
) -> Result<M5SourceAcquisitionReviewPacket, serde_json::Error> {
    serde_json::from_str(M5_SOURCE_ACQUISITION_REVIEW_JSON)
}

#[cfg(test)]
mod tests;
