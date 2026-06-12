//! Canonical M5 workspace-admission-and-routing packet: one inspectable admission
//! checkpoint per M5 wedge that classifies the workspace, labels where the
//! classification came from, and routes the user to first useful work without a
//! hidden trust change, a forced setup wizard, or false archetype certainty.
//!
//! Each [`AdmissionCheckpoint`] covers a single M5 wedge ([`M5Wedge`]) and answers, before any
//! feature-local empty state or wizard could take over: how admission classifies the workspace
//! ([`AdmissionClass`] — certified, probable, mixed, unknown, restricted, or
//! missing-prerequisite), which source the classification came from ([`DetectionSource`]), how
//! confident archetype detection is ([`ArchetypeConfidence`]), where any bundle recommendation
//! originates ([`BundleRecommendationSource`]), and which first-useful-work route
//! ([`FirstUsefulWorkRoute`]) the wedge offers.
//!
//! Setup is separated by urgency, not funneled. Each [`SetupItem`] carries a [`SetupTiming`] of
//! blocking-now, recommended-soon, or optional-later, so a wedge can keep open-minimal and
//! set-up-later paths weighted equally with guided setup wherever that is safe. Users can defer
//! every non-blocking item and still reach minimal local-safe work
//! ([`AdmissionCheckpoint::local_safe_work_available`]); only an explicit policy restriction
//! removes local-safe work.
//!
//! Probable and mixed detection never widen trust on their own. Every checkpoint records four
//! guardrail flags — [`AdmissionCheckpoint::forces_wizard`],
//! [`AdmissionCheckpoint::auto_installs_packs`],
//! [`AdmissionCheckpoint::rewrites_layout_without_review`], and
//! [`AdmissionCheckpoint::widens_trust_without_review`] — that are always `false`, and no
//! [`SetupItem`] ever auto-runs ([`SetupItem::auto_runs`] is always `false`). A checkpoint may
//! only present as certified support when its admission class is actually
//! [`AdmissionClass::Certified`], and the admission class may never out-rank the archetype
//! confidence it derives from ([`AdmissionClass::permitted_under`]); a probable or mixed
//! detection therefore can never read as certified support.
//!
//! Provenance survives into support and help surfaces: each checkpoint carries a routing
//! provenance ref, an archetype-evidence ref, and a bundle-recommendation ref, plus diagnostics,
//! support-export, help-surface, docs-badge, and release-evidence refs, so those surfaces ingest
//! the *same* admission packet rather than re-deriving divergent status text.
//!
//! The packet is checked in at `artifacts/workspace/m5/m5-admission-and-routing.json` and
//! embedded here. It is metadata-only: every field is a typed state, a count, or an opaque ref,
//! and it carries no credential bodies, raw provider payloads, raw local paths, or workspace
//! contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5 admission-and-routing packet schema version.
pub const M5_ADMISSION_AND_ROUTING_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_ADMISSION_AND_ROUTING_RECORD_KIND: &str = "m5_admission_and_routing_packet";

/// Repo-relative path to the checked-in packet.
pub const M5_ADMISSION_AND_ROUTING_PATH: &str =
    "artifacts/workspace/m5/m5-admission-and-routing.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_ADMISSION_AND_ROUTING_SCHEMA_REF: &str =
    "schemas/workspace/m5-admission-and-routing.schema.json";

/// Repo-relative path to the companion document.
pub const M5_ADMISSION_AND_ROUTING_DOC_REF: &str = "docs/workspace/m5/m5-admission-and-routing.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_ADMISSION_AND_ROUTING_FIXTURE_DIR: &str =
    "fixtures/workspace/m5/m5-admission-and-routing";

/// Embedded checked-in packet JSON.
pub const M5_ADMISSION_AND_ROUTING_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/workspace/m5/m5-admission-and-routing.json"
));

/// The M5 wedge an admission checkpoint governs.
///
/// Every wedge that M5 claims carries a truthful admission checkpoint instead of a feature-local
/// empty state or forced wizard, so the closed set is exercised in full by the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5Wedge {
    /// The notebook workspace.
    NotebookWorkspace,
    /// The data and API workspace.
    DataAndApiWorkspace,
    /// The profiler workspace.
    ProfilerWorkspace,
    /// The framework-pack workspace.
    FrameworkPackWorkspace,
    /// The docs workspace.
    DocsWorkspace,
    /// The companion-entry workspace.
    CompanionWorkspace,
    /// The sync-handoff workspace.
    SyncHandoffWorkspace,
    /// An opened local folder.
    LocalFolderWorkspace,
}

impl M5Wedge {
    /// Every M5 wedge, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NotebookWorkspace,
        Self::DataAndApiWorkspace,
        Self::ProfilerWorkspace,
        Self::FrameworkPackWorkspace,
        Self::DocsWorkspace,
        Self::CompanionWorkspace,
        Self::SyncHandoffWorkspace,
        Self::LocalFolderWorkspace,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookWorkspace => "notebook_workspace",
            Self::DataAndApiWorkspace => "data_and_api_workspace",
            Self::ProfilerWorkspace => "profiler_workspace",
            Self::FrameworkPackWorkspace => "framework_pack_workspace",
            Self::DocsWorkspace => "docs_workspace",
            Self::CompanionWorkspace => "companion_workspace",
            Self::SyncHandoffWorkspace => "sync_handoff_workspace",
            Self::LocalFolderWorkspace => "local_folder_workspace",
        }
    }
}

/// How admission classifies an M5 workspace.
///
/// The six states stay distinct in UI, diagnostics, and docs. Only [`Self::Certified`] may be
/// presented as full, certified support; probable and mixed are explicitly weaker and never
/// equivalent to certified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionClass {
    /// A certified workspace with provable archetype support.
    Certified,
    /// A probable workspace inferred from heuristic signals.
    Probable,
    /// A workspace with mixed, conflicting signals.
    Mixed,
    /// A workspace with no usable detection signal.
    Unknown,
    /// A workspace whose admission is limited by policy.
    Restricted,
    /// A workspace missing a prerequisite before its feature can run.
    MissingPrerequisite,
}

impl AdmissionClass {
    /// Every admission class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Certified,
        Self::Probable,
        Self::Mixed,
        Self::Unknown,
        Self::Restricted,
        Self::MissingPrerequisite,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Probable => "probable",
            Self::Mixed => "mixed",
            Self::Unknown => "unknown",
            Self::Restricted => "restricted",
            Self::MissingPrerequisite => "missing_prerequisite",
        }
    }

    /// Whether this class may be presented to the user as certified support.
    ///
    /// Only a genuinely certified workspace may; a probable or mixed detection must never read as
    /// certified support.
    pub const fn presents_as_certified_support(self) -> bool {
        matches!(self, Self::Certified)
    }

    /// Whether this class is permitted under the given archetype confidence.
    ///
    /// The admission class may never out-rank the confidence it derives from: a certified
    /// admission needs certified confidence, a probable admission needs at least probable
    /// confidence, and a mixed admission needs at least mixed confidence. Unknown, restricted,
    /// and missing-prerequisite are orthogonal to confidence (they describe absent signal,
    /// policy, or prerequisites) and carry no confidence floor.
    pub const fn permitted_under(self, confidence: ArchetypeConfidence) -> bool {
        match self {
            Self::Certified => matches!(confidence, ArchetypeConfidence::Certified),
            Self::Probable => matches!(
                confidence,
                ArchetypeConfidence::Certified | ArchetypeConfidence::Probable
            ),
            Self::Mixed => !matches!(confidence, ArchetypeConfidence::Unknown),
            Self::Unknown | Self::Restricted | Self::MissingPrerequisite => true,
        }
    }
}

/// The source label explaining where an admission classification came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectionSource {
    /// A certified archetype manifest.
    CertifiedManifest,
    /// A framework-pack signal.
    FrameworkPackSignal,
    /// A heuristic probe of the workspace.
    HeuristicProbe,
    /// Conflicting signals blended together.
    MixedSignals,
    /// A user-declared archetype.
    UserDeclared,
    /// No usable signal was found.
    NoSignal,
    /// A policy restriction on admission.
    PolicyRestriction,
    /// A probe that found a missing toolchain prerequisite.
    MissingToolchainProbe,
}

impl DetectionSource {
    /// Every detection source, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CertifiedManifest,
        Self::FrameworkPackSignal,
        Self::HeuristicProbe,
        Self::MixedSignals,
        Self::UserDeclared,
        Self::NoSignal,
        Self::PolicyRestriction,
        Self::MissingToolchainProbe,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedManifest => "certified_manifest",
            Self::FrameworkPackSignal => "framework_pack_signal",
            Self::HeuristicProbe => "heuristic_probe",
            Self::MixedSignals => "mixed_signals",
            Self::UserDeclared => "user_declared",
            Self::NoSignal => "no_signal",
            Self::PolicyRestriction => "policy_restriction",
            Self::MissingToolchainProbe => "missing_toolchain_probe",
        }
    }

    /// Whether this source is canonical for the given admission class.
    ///
    /// The source label must match the class it justifies: a certified class only follows a
    /// certified manifest or a framework-pack signal, a probable class only follows a heuristic
    /// probe or a framework-pack signal, a mixed class only follows blended signals, an unknown
    /// class only follows no-signal or a user declaration, a restricted class only follows a
    /// policy restriction, and a missing-prerequisite class only follows a missing-toolchain
    /// probe.
    pub const fn is_canonical_for(self, class: AdmissionClass) -> bool {
        match class {
            AdmissionClass::Certified => {
                matches!(self, Self::CertifiedManifest | Self::FrameworkPackSignal)
            }
            AdmissionClass::Probable => {
                matches!(self, Self::HeuristicProbe | Self::FrameworkPackSignal)
            }
            AdmissionClass::Mixed => matches!(self, Self::MixedSignals),
            AdmissionClass::Unknown => matches!(self, Self::NoSignal | Self::UserDeclared),
            AdmissionClass::Restricted => matches!(self, Self::PolicyRestriction),
            AdmissionClass::MissingPrerequisite => matches!(self, Self::MissingToolchainProbe),
        }
    }
}

/// How confident archetype detection is for a workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchetypeConfidence {
    /// Certified, provable archetype.
    Certified,
    /// A probable archetype inferred from signals.
    Probable,
    /// Mixed, conflicting archetype signals.
    Mixed,
    /// No usable archetype signal.
    Unknown,
}

impl ArchetypeConfidence {
    /// Every archetype confidence, in declaration order.
    pub const ALL: [Self; 4] = [Self::Certified, Self::Probable, Self::Mixed, Self::Unknown];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Probable => "probable",
            Self::Mixed => "mixed",
            Self::Unknown => "unknown",
        }
    }
}

/// Where a bundle recommendation a checkpoint discloses originates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleRecommendationSource {
    /// A certified archetype match.
    CertifiedArchetypeMatch,
    /// A probable heuristic match.
    ProbableHeuristic,
    /// A blend of mixed signals.
    MixedSignalBlend,
    /// An explicit user selection.
    UserSelected,
    /// No bundle was recommended.
    NoRecommendation,
}

impl BundleRecommendationSource {
    /// Every bundle-recommendation source, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CertifiedArchetypeMatch,
        Self::ProbableHeuristic,
        Self::MixedSignalBlend,
        Self::UserSelected,
        Self::NoRecommendation,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedArchetypeMatch => "certified_archetype_match",
            Self::ProbableHeuristic => "probable_heuristic",
            Self::MixedSignalBlend => "mixed_signal_blend",
            Self::UserSelected => "user_selected",
            Self::NoRecommendation => "no_recommendation",
        }
    }

    /// Whether this recommendation source is permitted under the given archetype confidence.
    ///
    /// A bundle recommendation may not claim more certainty than the archetype it rests on: a
    /// certified-archetype match needs certified confidence, a probable-heuristic match needs at
    /// least probable confidence, and a mixed-signal blend needs at least mixed confidence. A
    /// user selection and an explicit no-recommendation carry no confidence floor.
    pub const fn permitted_under(self, confidence: ArchetypeConfidence) -> bool {
        match self {
            Self::CertifiedArchetypeMatch => matches!(confidence, ArchetypeConfidence::Certified),
            Self::ProbableHeuristic => matches!(
                confidence,
                ArchetypeConfidence::Certified | ArchetypeConfidence::Probable
            ),
            Self::MixedSignalBlend => !matches!(confidence, ArchetypeConfidence::Unknown),
            Self::UserSelected | Self::NoRecommendation => true,
        }
    }
}

/// The first-useful-work route a wedge offers after admission.
///
/// No route is a forced wizard: every certified, probable, mixed, or unknown wedge keeps an
/// open-minimal or set-up-later path, and guided setup is offered, never imposed. Restricted and
/// missing-prerequisite wedges still keep a local-safe fallback rather than a dead end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirstUsefulWorkRoute {
    /// Open straight into minimal editing; setup is optional.
    OpenMinimal,
    /// Work locally now and defer setup.
    SetUpLater,
    /// Offer guided setup without forcing it; minimal work stays available.
    GuidedSetupOffered,
    /// Only local-safe work until a blocking prerequisite is met.
    LocalSafeFallback,
    /// Policy-limited browse-only admission.
    RestrictedBrowse,
}

impl FirstUsefulWorkRoute {
    /// Every first-useful-work route, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OpenMinimal,
        Self::SetUpLater,
        Self::GuidedSetupOffered,
        Self::LocalSafeFallback,
        Self::RestrictedBrowse,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenMinimal => "open_minimal",
            Self::SetUpLater => "set_up_later",
            Self::GuidedSetupOffered => "guided_setup_offered",
            Self::LocalSafeFallback => "local_safe_fallback",
            Self::RestrictedBrowse => "restricted_browse",
        }
    }
}

/// When a setup item should run relative to first useful work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupTiming {
    /// Blocks the wedge's primary feature until it is done.
    BlockingNow,
    /// Recommended soon, but deferrable.
    RecommendedSoon,
    /// Optional and deferrable indefinitely.
    OptionalLater,
}

impl SetupTiming {
    /// Every setup timing, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::BlockingNow,
        Self::RecommendedSoon,
        Self::OptionalLater,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockingNow => "blocking_now",
            Self::RecommendedSoon => "recommended_soon",
            Self::OptionalLater => "optional_later",
        }
    }

    /// Whether an item with this timing blocks first useful work.
    pub const fn blocks_first_useful_work(self) -> bool {
        matches!(self, Self::BlockingNow)
    }
}

/// The kind of setup item a checkpoint discloses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupItemKind {
    /// Install a framework pack.
    InstallFrameworkPack,
    /// Restore packages.
    RestorePackages,
    /// Configure a runtime, interpreter, or toolchain.
    ConfigureRuntime,
    /// Grant trust to the workspace.
    GrantTrust,
    /// Hydrate data assets.
    HydrateData,
    /// Import a docs pack.
    ImportDocsPack,
    /// Recommend a workflow bundle.
    RecommendWorkflowBundle,
    /// Warm the symbol index.
    WarmIndex,
}

impl SetupItemKind {
    /// Every setup item kind, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::InstallFrameworkPack,
        Self::RestorePackages,
        Self::ConfigureRuntime,
        Self::GrantTrust,
        Self::HydrateData,
        Self::ImportDocsPack,
        Self::RecommendWorkflowBundle,
        Self::WarmIndex,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallFrameworkPack => "install_framework_pack",
            Self::RestorePackages => "restore_packages",
            Self::ConfigureRuntime => "configure_runtime",
            Self::GrantTrust => "grant_trust",
            Self::HydrateData => "hydrate_data",
            Self::ImportDocsPack => "import_docs_pack",
            Self::RecommendWorkflowBundle => "recommend_workflow_bundle",
            Self::WarmIndex => "warm_index",
        }
    }
}

/// One setup item disclosed by an admission checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetupItem {
    /// Which setup step this item describes.
    pub item_kind: SetupItemKind,
    /// When the item should run relative to first useful work.
    pub timing: SetupTiming,
    /// Whether the item blocks first useful work. Must equal the timing's own answer.
    pub blocks_first_useful_work: bool,
    /// Guardrail: the item never runs automatically. Always `false`.
    pub auto_runs: bool,
    /// Whether the item requires review before it can run.
    pub requires_review: bool,
    /// A human-readable, one-step action label.
    pub action_label: String,
    /// Opaque evidence ref backing the item.
    pub evidence_ref: String,
}

impl SetupItem {
    /// Whether this item is internally consistent and never auto-runs.
    pub fn is_consistent(&self) -> bool {
        !self.auto_runs
            && self.blocks_first_useful_work == self.timing.blocks_first_useful_work()
            && !self.action_label.trim().is_empty()
            && !self.evidence_ref.trim().is_empty()
    }
}

/// One admission checkpoint for a single M5 wedge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdmissionCheckpoint {
    /// Stable checkpoint identifier.
    pub checkpoint_id: String,
    /// The M5 wedge this checkpoint governs.
    pub wedge: M5Wedge,
    /// How admission classifies the workspace.
    pub admission_class: AdmissionClass,
    /// The source label for the classification.
    pub detection_source: DetectionSource,
    /// How confident archetype detection is.
    pub archetype_confidence: ArchetypeConfidence,
    /// Where any bundle recommendation originates.
    pub bundle_recommendation_source: BundleRecommendationSource,
    /// The first-useful-work route the wedge offers.
    pub first_useful_work_route: FirstUsefulWorkRoute,
    /// Whether minimal local-safe work is available now.
    pub local_safe_work_available: bool,
    /// Guardrail: the route never forces a setup wizard. Always `false`.
    pub forces_wizard: bool,
    /// Guardrail: probable or mixed detection never auto-installs packs. Always `false`.
    pub auto_installs_packs: bool,
    /// Guardrail: detection never rewrites layout without review. Always `false`.
    pub rewrites_layout_without_review: bool,
    /// Guardrail: detection never widens trust without review. Always `false`.
    pub widens_trust_without_review: bool,
    /// Whether the checkpoint presents as certified support. Must match the admission class.
    pub presented_as_certified_support: bool,
    /// Setup items disclosed, separated by timing.
    pub setup_items: Vec<SetupItem>,
    /// Opaque first-useful-work routing provenance ref.
    pub routing_provenance_ref: String,
    /// Opaque archetype-confidence evidence ref.
    pub archetype_evidence_ref: String,
    /// Opaque bundle-recommendation provenance ref.
    pub bundle_recommendation_ref: String,
    /// Accountable owner.
    pub owner: String,
    /// Caveats shown on the checkpoint.
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
    /// A reviewer note summarizing the checkpoint.
    pub note: String,
}

impl AdmissionCheckpoint {
    /// Whether any setup item blocks first useful work.
    pub fn has_blocking_setup(&self) -> bool {
        self.setup_items
            .iter()
            .any(|item| item.timing.blocks_first_useful_work())
    }

    /// Whether any setup item requires review before it can run.
    pub fn has_review_required_setup(&self) -> bool {
        self.setup_items.iter().any(|item| item.requires_review)
    }

    /// Setup items with the given timing.
    pub fn items_with_timing(&self, timing: SetupTiming) -> impl Iterator<Item = &SetupItem> {
        self.setup_items.iter().filter(move |i| i.timing == timing)
    }

    /// Whether any setup item is flagged to auto-run — always a violation if true.
    pub fn has_auto_run_setup(&self) -> bool {
        self.setup_items.iter().any(|item| item.auto_runs)
    }

    /// Whether all four trust-and-layout guardrail flags are held closed.
    pub fn guards_closed(&self) -> bool {
        !self.forces_wizard
            && !self.auto_installs_packs
            && !self.rewrites_layout_without_review
            && !self.widens_trust_without_review
    }

    /// The certified-support presentation the gate computes from the admission class.
    pub fn computed_presented_as_certified(&self) -> bool {
        self.admission_class.presents_as_certified_support()
    }

    /// The local-safe-work availability the gate computes from the admission class.
    ///
    /// Minimal local-safe work stays available everywhere except under an explicit policy
    /// restriction; even a missing prerequisite keeps local-safe work, so deferring setup never
    /// removes it.
    pub fn computed_local_safe_available(&self) -> bool {
        self.admission_class != AdmissionClass::Restricted
    }

    /// Whether the first-useful-work route is consistent with the class and the setup blocking.
    ///
    /// A restricted-browse route only fits a restricted class; a local-safe fallback fits a
    /// blocking prerequisite or a missing-prerequisite class; open-minimal and set-up-later fit
    /// any non-restricted, non-missing-prerequisite class with no blocking item; and guided
    /// setup is only offered for certified, probable, or mixed classes with no blocking item.
    pub fn route_is_consistent(&self) -> bool {
        let blocking = self.has_blocking_setup();
        match self.first_useful_work_route {
            FirstUsefulWorkRoute::RestrictedBrowse => {
                self.admission_class == AdmissionClass::Restricted
            }
            FirstUsefulWorkRoute::LocalSafeFallback => {
                blocking || self.admission_class == AdmissionClass::MissingPrerequisite
            }
            FirstUsefulWorkRoute::OpenMinimal | FirstUsefulWorkRoute::SetUpLater => {
                !blocking
                    && !matches!(
                        self.admission_class,
                        AdmissionClass::Restricted | AdmissionClass::MissingPrerequisite
                    )
            }
            FirstUsefulWorkRoute::GuidedSetupOffered => {
                !blocking
                    && matches!(
                        self.admission_class,
                        AdmissionClass::Certified
                            | AdmissionClass::Probable
                            | AdmissionClass::Mixed
                    )
            }
        }
    }

    /// Whether the admission class is permitted under the recorded archetype confidence.
    pub fn class_within_confidence(&self) -> bool {
        self.admission_class
            .permitted_under(self.archetype_confidence)
    }

    /// Whether the bundle recommendation source is permitted under the recorded confidence.
    pub fn bundle_within_confidence(&self) -> bool {
        self.bundle_recommendation_source
            .permitted_under(self.archetype_confidence)
    }

    /// Whether the detection source is canonical for the admission class.
    pub fn detection_source_canonical(&self) -> bool {
        self.detection_source.is_canonical_for(self.admission_class)
    }

    /// Whether the checkpoint carries complete routing, archetype, and bundle provenance.
    pub fn provenance_complete(&self) -> bool {
        !self.routing_provenance_ref.trim().is_empty()
            && !self.archetype_evidence_ref.trim().is_empty()
            && !self.bundle_recommendation_ref.trim().is_empty()
    }

    /// Whether a caveat is required on this checkpoint.
    ///
    /// Anything not certified, anything with a blocking setup item, and anything with a
    /// review-required setup item must carry a caveat so the weaker assurance is never silent.
    pub fn caveats_required(&self) -> bool {
        self.admission_class != AdmissionClass::Certified
            || self.has_blocking_setup()
            || self.has_review_required_setup()
    }
}

/// Summary counts rolled up across every admission checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5AdmissionAndRoutingSummary {
    /// Total checkpoints.
    pub total_checkpoints: usize,
    /// Checkpoint count (equals `total_checkpoints`; kept for export parity).
    pub checkpoint_count: usize,
    /// Checkpoints classified certified.
    pub certified_checkpoints: usize,
    /// Checkpoints classified probable.
    pub probable_checkpoints: usize,
    /// Checkpoints classified mixed.
    pub mixed_checkpoints: usize,
    /// Checkpoints classified unknown.
    pub unknown_checkpoints: usize,
    /// Checkpoints classified restricted.
    pub restricted_checkpoints: usize,
    /// Checkpoints classified missing-prerequisite.
    pub missing_prerequisite_checkpoints: usize,
    /// Checkpoints that present as certified support.
    pub checkpoints_presenting_certified_support: usize,
    /// Checkpoints with at least one blocking-now setup item.
    pub checkpoints_with_blocking_setup: usize,
    /// Checkpoints that keep minimal local-safe work available.
    pub checkpoints_with_local_safe_work: usize,
    /// Total setup items across all checkpoints.
    pub total_setup_items: usize,
    /// Blocking-now setup items.
    pub blocking_now_items: usize,
    /// Recommended-soon setup items.
    pub recommended_soon_items: usize,
    /// Optional-later setup items.
    pub optional_later_items: usize,
    /// Setup items that require review before running.
    pub setup_items_requiring_review: usize,
}

/// One redaction-safe export row projected from a checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdmissionAndRoutingExportRow {
    /// Checkpoint id.
    pub checkpoint_id: String,
    /// Wedge token.
    pub wedge: M5Wedge,
    /// Admission class token.
    pub admission_class: AdmissionClass,
    /// Detection source token.
    pub detection_source: DetectionSource,
    /// Archetype confidence token.
    pub archetype_confidence: ArchetypeConfidence,
    /// Bundle recommendation source token.
    pub bundle_recommendation_source: BundleRecommendationSource,
    /// First-useful-work route token.
    pub first_useful_work_route: FirstUsefulWorkRoute,
    /// Whether minimal local-safe work is available.
    pub local_safe_work_available: bool,
    /// Whether the checkpoint presents as certified support.
    pub presented_as_certified_support: bool,
    /// Setup item kinds disclosed.
    pub setup_item_kinds: Vec<SetupItemKind>,
    /// Blocking-now setup item kinds.
    pub blocking_now_item_kinds: Vec<SetupItemKind>,
    /// Routing provenance ref.
    pub routing_provenance_ref: String,
}

/// A redaction-safe export projection of the whole packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdmissionAndRoutingExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected checkpoint rows.
    pub checkpoints: Vec<M5AdmissionAndRoutingExportRow>,
    /// Whether every checkpoint is gate-consistent.
    pub all_checkpoints_consistent: bool,
    /// Checkpoints that keep local-safe work.
    pub checkpoints_with_local_safe_work: usize,
    /// Checkpoints with a blocking-now setup item.
    pub checkpoints_with_blocking_setup: usize,
}

/// The typed M5 admission-and-routing packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5AdmissionAndRoutingPacket {
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
    /// Scheme the packet mints stable checkpoint identities under.
    pub checkpoint_identity_scheme: String,
    /// Closed wedge vocabulary.
    pub wedges: Vec<M5Wedge>,
    /// Closed admission-class vocabulary.
    pub admission_classes: Vec<AdmissionClass>,
    /// Closed detection-source vocabulary.
    pub detection_sources: Vec<DetectionSource>,
    /// Closed archetype-confidence vocabulary.
    pub archetype_confidences: Vec<ArchetypeConfidence>,
    /// Closed bundle-recommendation-source vocabulary.
    pub bundle_recommendation_sources: Vec<BundleRecommendationSource>,
    /// Closed first-useful-work-route vocabulary.
    pub first_useful_work_routes: Vec<FirstUsefulWorkRoute>,
    /// Closed setup-timing vocabulary.
    pub setup_timings: Vec<SetupTiming>,
    /// Closed setup-item-kind vocabulary.
    pub setup_item_kinds: Vec<SetupItemKind>,
    /// Admission checkpoints, one or more per M5 wedge.
    #[serde(default)]
    pub checkpoints: Vec<AdmissionCheckpoint>,
    /// Summary counts.
    pub summary: M5AdmissionAndRoutingSummary,
}

impl M5AdmissionAndRoutingPacket {
    /// Returns the checkpoint with the given id.
    pub fn checkpoint(&self, checkpoint_id: &str) -> Option<&AdmissionCheckpoint> {
        self.checkpoints
            .iter()
            .find(|c| c.checkpoint_id == checkpoint_id)
    }

    /// Checkpoints governing the given wedge.
    pub fn checkpoints_for_wedge(
        &self,
        wedge: M5Wedge,
    ) -> impl Iterator<Item = &AdmissionCheckpoint> {
        self.checkpoints.iter().filter(move |c| c.wedge == wedge)
    }

    /// Checkpoints with the given admission class.
    pub fn checkpoints_with_class(
        &self,
        class: AdmissionClass,
    ) -> impl Iterator<Item = &AdmissionCheckpoint> {
        self.checkpoints
            .iter()
            .filter(move |c| c.admission_class == class)
    }

    /// Whether every M5 wedge is covered by at least one checkpoint.
    pub fn covers_every_wedge(&self) -> bool {
        M5Wedge::ALL
            .iter()
            .all(|wedge| self.checkpoints_for_wedge(*wedge).next().is_some())
    }

    /// Whether every checkpoint is internally consistent against the gate.
    pub fn all_checkpoints_consistent(&self) -> bool {
        self.checkpoints.iter().all(|cp| {
            cp.guards_closed()
                && !cp.has_auto_run_setup()
                && cp.class_within_confidence()
                && cp.bundle_within_confidence()
                && cp.detection_source_canonical()
                && cp.presented_as_certified_support == cp.computed_presented_as_certified()
                && cp.local_safe_work_available == cp.computed_local_safe_available()
                && cp.route_is_consistent()
                && cp.provenance_complete()
                && cp.setup_items.iter().all(SetupItem::is_consistent)
                && (!cp.caveats_required() || cp.caveats.iter().any(|c| !c.trim().is_empty()))
        })
    }

    /// Recomputes the summary from the checkpoints.
    pub fn computed_summary(&self) -> M5AdmissionAndRoutingSummary {
        let count_class = |class: AdmissionClass| self.checkpoints_with_class(class).count();
        let count_timing = |timing: SetupTiming| {
            self.checkpoints
                .iter()
                .flat_map(|c| c.setup_items.iter())
                .filter(|i| i.timing == timing)
                .count()
        };
        M5AdmissionAndRoutingSummary {
            total_checkpoints: self.checkpoints.len(),
            checkpoint_count: self.checkpoints.len(),
            certified_checkpoints: count_class(AdmissionClass::Certified),
            probable_checkpoints: count_class(AdmissionClass::Probable),
            mixed_checkpoints: count_class(AdmissionClass::Mixed),
            unknown_checkpoints: count_class(AdmissionClass::Unknown),
            restricted_checkpoints: count_class(AdmissionClass::Restricted),
            missing_prerequisite_checkpoints: count_class(AdmissionClass::MissingPrerequisite),
            checkpoints_presenting_certified_support: self
                .checkpoints
                .iter()
                .filter(|c| c.presented_as_certified_support)
                .count(),
            checkpoints_with_blocking_setup: self
                .checkpoints
                .iter()
                .filter(|c| c.has_blocking_setup())
                .count(),
            checkpoints_with_local_safe_work: self
                .checkpoints
                .iter()
                .filter(|c| c.local_safe_work_available)
                .count(),
            total_setup_items: self.checkpoints.iter().map(|c| c.setup_items.len()).sum(),
            blocking_now_items: count_timing(SetupTiming::BlockingNow),
            recommended_soon_items: count_timing(SetupTiming::RecommendedSoon),
            optional_later_items: count_timing(SetupTiming::OptionalLater),
            setup_items_requiring_review: self
                .checkpoints
                .iter()
                .flat_map(|c| c.setup_items.iter())
                .filter(|i| i.requires_review)
                .count(),
        }
    }

    /// Projects a redaction-safe export view of the packet.
    pub fn export_projection(&self) -> M5AdmissionAndRoutingExportProjection {
        let checkpoints = self
            .checkpoints
            .iter()
            .map(|cp| M5AdmissionAndRoutingExportRow {
                checkpoint_id: cp.checkpoint_id.clone(),
                wedge: cp.wedge,
                admission_class: cp.admission_class,
                detection_source: cp.detection_source,
                archetype_confidence: cp.archetype_confidence,
                bundle_recommendation_source: cp.bundle_recommendation_source,
                first_useful_work_route: cp.first_useful_work_route,
                local_safe_work_available: cp.local_safe_work_available,
                presented_as_certified_support: cp.presented_as_certified_support,
                setup_item_kinds: cp.setup_items.iter().map(|i| i.item_kind).collect(),
                blocking_now_item_kinds: cp
                    .setup_items
                    .iter()
                    .filter(|i| i.timing.blocks_first_useful_work())
                    .map(|i| i.item_kind)
                    .collect(),
                routing_provenance_ref: cp.routing_provenance_ref.clone(),
            })
            .collect();
        let summary = self.computed_summary();
        M5AdmissionAndRoutingExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            checkpoints,
            all_checkpoints_consistent: self.all_checkpoints_consistent(),
            checkpoints_with_local_safe_work: summary.checkpoints_with_local_safe_work,
            checkpoints_with_blocking_setup: summary.checkpoints_with_blocking_setup,
        }
    }

    /// Validates the packet against its honesty contract, returning every violation.
    pub fn validate(&self) -> Vec<M5AdmissionAndRoutingViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        let mut seen_ids = BTreeSet::new();
        for cp in &self.checkpoints {
            if !seen_ids.insert(cp.checkpoint_id.clone()) {
                violations.push(M5AdmissionAndRoutingViolation::DuplicateCheckpointId {
                    checkpoint_id: cp.checkpoint_id.clone(),
                });
            }
            self.validate_checkpoint(cp, &mut violations);
        }
        for wedge in M5Wedge::ALL {
            if self.checkpoints_for_wedge(wedge).next().is_none() {
                violations.push(M5AdmissionAndRoutingViolation::MissingWedgeCoverage { wedge });
            }
        }
        if self.summary != self.computed_summary() {
            violations.push(M5AdmissionAndRoutingViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5AdmissionAndRoutingViolation>) {
        if self.schema_version != M5_ADMISSION_AND_ROUTING_SCHEMA_VERSION {
            violations.push(M5AdmissionAndRoutingViolation::SchemaVersionMismatch {
                found: self.schema_version,
            });
        }
        if self.record_kind != M5_ADMISSION_AND_ROUTING_RECORD_KIND {
            violations.push(M5AdmissionAndRoutingViolation::RecordKindMismatch {
                found: self.record_kind.clone(),
            });
        }
        let vocab_ok = self.wedges == M5Wedge::ALL
            && self.admission_classes == AdmissionClass::ALL
            && self.detection_sources == DetectionSource::ALL
            && self.archetype_confidences == ArchetypeConfidence::ALL
            && self.bundle_recommendation_sources == BundleRecommendationSource::ALL
            && self.first_useful_work_routes == FirstUsefulWorkRoute::ALL
            && self.setup_timings == SetupTiming::ALL
            && self.setup_item_kinds == SetupItemKind::ALL;
        if !vocab_ok {
            violations.push(M5AdmissionAndRoutingViolation::VocabularyMismatch);
        }
        if self.checkpoints.is_empty() {
            violations.push(M5AdmissionAndRoutingViolation::NoCheckpoints);
        }
    }

    fn validate_checkpoint(
        &self,
        cp: &AdmissionCheckpoint,
        violations: &mut Vec<M5AdmissionAndRoutingViolation>,
    ) {
        let id = cp.checkpoint_id.clone();
        if id.trim().is_empty() {
            violations.push(M5AdmissionAndRoutingViolation::EmptyCheckpointId);
        }
        if !cp.guards_closed() {
            violations.push(M5AdmissionAndRoutingViolation::GuardOpen {
                checkpoint_id: id.clone(),
            });
        }
        if cp.has_auto_run_setup() {
            violations.push(M5AdmissionAndRoutingViolation::AutoRunSetupItem {
                checkpoint_id: id.clone(),
            });
        }
        if !cp.class_within_confidence() {
            violations.push(M5AdmissionAndRoutingViolation::AdmissionExceedsConfidence {
                checkpoint_id: id.clone(),
                class: cp.admission_class,
                confidence: cp.archetype_confidence,
            });
        }
        if !cp.bundle_within_confidence() {
            violations.push(M5AdmissionAndRoutingViolation::BundleExceedsConfidence {
                checkpoint_id: id.clone(),
            });
        }
        if !cp.detection_source_canonical() {
            violations.push(
                M5AdmissionAndRoutingViolation::DetectionSourceNotCanonical {
                    checkpoint_id: id.clone(),
                    source: cp.detection_source,
                    class: cp.admission_class,
                },
            );
        }
        if cp.presented_as_certified_support != cp.computed_presented_as_certified() {
            violations.push(
                M5AdmissionAndRoutingViolation::CertifiedPresentationMismatch {
                    checkpoint_id: id.clone(),
                    recorded: cp.presented_as_certified_support,
                    computed: cp.computed_presented_as_certified(),
                },
            );
        }
        if cp.local_safe_work_available != cp.computed_local_safe_available() {
            violations.push(M5AdmissionAndRoutingViolation::LocalSafeMismatch {
                checkpoint_id: id.clone(),
                recorded: cp.local_safe_work_available,
                computed: cp.computed_local_safe_available(),
            });
        }
        if !cp.route_is_consistent() {
            violations.push(M5AdmissionAndRoutingViolation::RouteInconsistent {
                checkpoint_id: id.clone(),
                route: cp.first_useful_work_route,
                class: cp.admission_class,
            });
        }
        if !cp.provenance_complete() {
            violations.push(M5AdmissionAndRoutingViolation::MissingProvenance {
                checkpoint_id: id.clone(),
            });
        }
        for item in &cp.setup_items {
            if !item.is_consistent() {
                violations.push(M5AdmissionAndRoutingViolation::InconsistentSetupItem {
                    checkpoint_id: id.clone(),
                    item_kind: item.item_kind,
                });
            }
        }
        if cp.caveats_required() && cp.caveats.iter().all(|c| c.trim().is_empty()) {
            violations.push(M5AdmissionAndRoutingViolation::MissingCaveat {
                checkpoint_id: id.clone(),
            });
        }
        let surface_refs = [
            &cp.owner,
            &cp.diagnostics_ref,
            &cp.support_export_ref,
            &cp.help_surface_ref,
            &cp.docs_badge_ref,
            &cp.release_evidence_ref,
            &cp.note,
        ];
        if surface_refs.iter().any(|r| r.trim().is_empty()) {
            violations
                .push(M5AdmissionAndRoutingViolation::MissingSurfaceRef { checkpoint_id: id });
        }
    }
}

/// A single way the packet can fail its honesty contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AdmissionAndRoutingViolation {
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
    /// The packet carries no checkpoints.
    NoCheckpoints,
    /// An M5 wedge has no admission checkpoint.
    MissingWedgeCoverage {
        /// The uncovered wedge.
        wedge: M5Wedge,
    },
    /// Two checkpoints share a checkpoint id.
    DuplicateCheckpointId {
        /// The duplicated id.
        checkpoint_id: String,
    },
    /// A checkpoint id is empty.
    EmptyCheckpointId,
    /// A checkpoint holds a trust-or-layout guardrail flag open.
    GuardOpen {
        /// The offending checkpoint id.
        checkpoint_id: String,
    },
    /// A checkpoint discloses a setup item flagged to auto-run.
    AutoRunSetupItem {
        /// The offending checkpoint id.
        checkpoint_id: String,
    },
    /// A checkpoint's admission class out-ranks its archetype confidence.
    AdmissionExceedsConfidence {
        /// The offending checkpoint id.
        checkpoint_id: String,
        /// The recorded admission class.
        class: AdmissionClass,
        /// The recorded archetype confidence.
        confidence: ArchetypeConfidence,
    },
    /// A checkpoint's bundle recommendation out-ranks its archetype confidence.
    BundleExceedsConfidence {
        /// The offending checkpoint id.
        checkpoint_id: String,
    },
    /// A checkpoint's detection source is not canonical for its admission class.
    DetectionSourceNotCanonical {
        /// The offending checkpoint id.
        checkpoint_id: String,
        /// The recorded detection source.
        source: DetectionSource,
        /// The recorded admission class.
        class: AdmissionClass,
    },
    /// A checkpoint presents as certified support when its class does not warrant it.
    CertifiedPresentationMismatch {
        /// The offending checkpoint id.
        checkpoint_id: String,
        /// The recorded value.
        recorded: bool,
        /// The recomputed value.
        computed: bool,
    },
    /// A checkpoint's local-safe availability diverges from the gate.
    LocalSafeMismatch {
        /// The offending checkpoint id.
        checkpoint_id: String,
        /// The recorded value.
        recorded: bool,
        /// The recomputed value.
        computed: bool,
    },
    /// A checkpoint's first-useful-work route is inconsistent with its class and blocking.
    RouteInconsistent {
        /// The offending checkpoint id.
        checkpoint_id: String,
        /// The recorded route.
        route: FirstUsefulWorkRoute,
        /// The recorded admission class.
        class: AdmissionClass,
    },
    /// A checkpoint lacks complete routing, archetype, or bundle provenance.
    MissingProvenance {
        /// The offending checkpoint id.
        checkpoint_id: String,
    },
    /// A setup item is internally inconsistent.
    InconsistentSetupItem {
        /// The offending checkpoint id.
        checkpoint_id: String,
        /// The item kind.
        item_kind: SetupItemKind,
    },
    /// A checkpoint that needs a caveat carries none.
    MissingCaveat {
        /// The offending checkpoint id.
        checkpoint_id: String,
    },
    /// A checkpoint is missing a required surface or owner ref.
    MissingSurfaceRef {
        /// The offending checkpoint id.
        checkpoint_id: String,
    },
    /// The recorded summary diverges from the recomputed summary.
    SummaryMismatch,
}

impl fmt::Display for M5AdmissionAndRoutingViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersionMismatch { found } => {
                write!(f, "schema_version mismatch: found {found}")
            }
            Self::RecordKindMismatch { found } => write!(f, "record_kind mismatch: found {found}"),
            Self::VocabularyMismatch => {
                write!(f, "a closed vocabulary array diverges from its canonical set")
            }
            Self::NoCheckpoints => write!(f, "packet carries no admission checkpoints"),
            Self::MissingWedgeCoverage { wedge } => {
                write!(f, "wedge {} has no admission checkpoint", wedge.as_str())
            }
            Self::DuplicateCheckpointId { checkpoint_id } => {
                write!(f, "duplicate checkpoint id: {checkpoint_id}")
            }
            Self::EmptyCheckpointId => write!(f, "a checkpoint has an empty id"),
            Self::GuardOpen { checkpoint_id } => {
                write!(f, "checkpoint {checkpoint_id} holds a guardrail flag open")
            }
            Self::AutoRunSetupItem { checkpoint_id } => {
                write!(f, "checkpoint {checkpoint_id} has a setup item that auto-runs")
            }
            Self::AdmissionExceedsConfidence {
                checkpoint_id,
                class,
                confidence,
            } => write!(
                f,
                "checkpoint {checkpoint_id} admission class {} out-ranks archetype confidence {}",
                class.as_str(),
                confidence.as_str()
            ),
            Self::BundleExceedsConfidence { checkpoint_id } => write!(
                f,
                "checkpoint {checkpoint_id} bundle recommendation out-ranks its archetype confidence"
            ),
            Self::DetectionSourceNotCanonical {
                checkpoint_id,
                source,
                class,
            } => write!(
                f,
                "checkpoint {checkpoint_id} detection source {} is not canonical for class {}",
                source.as_str(),
                class.as_str()
            ),
            Self::CertifiedPresentationMismatch {
                checkpoint_id,
                recorded,
                computed,
            } => write!(
                f,
                "checkpoint {checkpoint_id} certified presentation {recorded} diverges from gate {computed}"
            ),
            Self::LocalSafeMismatch {
                checkpoint_id,
                recorded,
                computed,
            } => write!(
                f,
                "checkpoint {checkpoint_id} local-safe availability {recorded} diverges from gate {computed}"
            ),
            Self::RouteInconsistent {
                checkpoint_id,
                route,
                class,
            } => write!(
                f,
                "checkpoint {checkpoint_id} route {} is inconsistent with class {}",
                route.as_str(),
                class.as_str()
            ),
            Self::MissingProvenance { checkpoint_id } => write!(
                f,
                "checkpoint {checkpoint_id} lacks routing, archetype, or bundle provenance"
            ),
            Self::InconsistentSetupItem {
                checkpoint_id,
                item_kind,
            } => write!(
                f,
                "checkpoint {checkpoint_id} setup item {} is inconsistent",
                item_kind.as_str()
            ),
            Self::MissingCaveat { checkpoint_id } => {
                write!(f, "checkpoint {checkpoint_id} needs a caveat but carries none")
            }
            Self::MissingSurfaceRef { checkpoint_id } => {
                write!(f, "checkpoint {checkpoint_id} is missing a required surface or owner ref")
            }
            Self::SummaryMismatch => write!(f, "summary diverges from the recomputed summary"),
        }
    }
}

impl Error for M5AdmissionAndRoutingViolation {}

/// Loads the embedded canonical M5 admission-and-routing packet.
///
/// # Errors
///
/// Returns a deserialization error if the embedded JSON does not parse into the typed packet.
pub fn current_m5_admission_and_routing_packet(
) -> Result<M5AdmissionAndRoutingPacket, serde_json::Error> {
    serde_json::from_str(M5_ADMISSION_AND_ROUTING_JSON)
}

#[cfg(test)]
mod tests;
