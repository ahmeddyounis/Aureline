//! Frozen M5 repository-bootstrap, checkout-plan, trust-stage, and post-open-queue execution matrix.
//!
//! This module locks Aureline's concrete repository-acquisition and workspace-bootstrap behavior into one
//! export-safe packet. Every claimed M5 project-entry verb — open a local checkout, clone a remote source,
//! open an archive, import a bundle, and resume a partial-acquisition snapshot — is named once here and
//! constrained by the same shared repository-bootstrap-role taxonomy (source_locator, checkout_plan,
//! credential_posture, evidence_packet, staged_trust, resumable_acquisition, post_open_queue), the same
//! clone-and-open-stay-distinct-verbs rule, the same
//! checkout-cost-topology-and-credential-posture-visible-before-mutation rule, the same
//! trust-is-staged-so-repo-owned-actions-never-run-implicitly rule, the same
//! signer-and-mirror-provenance-preserved-across-offline-or-mirrored-fetches rule, and the same
//! interrupted-acquisition-stays-resumable-or-discardable-with-evidence rule regardless of the surface that
//! renders it.
//!
//! The matrix does not redesign start-center cards or generic onboarding prose — it is the shared reusable
//! acquisition-and-bootstrap engine contract those already-governed surfaces consume, and it binds back to
//! the already-landed repository-acquisition and source-acquisition-review packets instead of leaving
//! acquisition truth split across scattered onboarding copy and hand-copied entry notes. The controlled
//! vocabularies are frozen in one self-describing [`M5RepositoryBootstrapVocabularySet`] rather than minted
//! per surface. The single controlled repository-bootstrap-role vocabulary consumers bind to —
//! source_locator, checkout_plan, credential_posture, evidence_packet, staged_trust, resumable_acquisition,
//! and post_open_queue — keeps the source locator and the checkout plan separately inspectable; keeps clone
//! and open distinct verbs even when a local checkout already exists; keeps checkout cost, topology, and
//! credential posture visible before any network or disk mutation; keeps repo hooks, repo-defined tasks,
//! extensions, package restores, submodule or LFS hydration, and generator installs from running implicitly
//! during acquisition; keeps signer and mirror provenance continuous across offline and mirrored fetches;
//! and keeps interrupted acquisition resumable or discardable with evidence. Raw secret values and private
//! endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_repository_bootstrap_matrix,
    seeded_m5_repository_bootstrap_matrix_import_bundle_beta_narrowed,
    seeded_m5_repository_bootstrap_matrix_resume_snapshot_preview_narrowed,
    M5_REPOSITORY_BOOTSTRAP_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5RepositoryBootstrapMatrixPacket`].
pub const M5_REPOSITORY_BOOTSTRAP_MATRIX_RECORD_KIND: &str =
    "freeze_m5_repository_bootstrap_checkout_plan_trust_stage_and_post_open_queue_matrix";

/// Schema version for M5 repository-bootstrap matrix records.
pub const M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined repository-bootstrap matrix schema.
pub const M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF: &str =
    "schemas/workspaces/m5-repository-bootstrap-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF: &str =
    "docs/workspaces/m5_repository_bootstrap_contract.md";

/// Repo-relative path of the canonical source-locator domain schema (open-local and open-archive families:
/// how a source is located and its checkout root or archive container resolved before mutation).
pub const M5_SOURCE_LOCATOR_DOMAIN_SCHEMA_REF: &str =
    "schemas/workspaces/m5-source-locator.schema.json";

/// Repo-relative path of the canonical checkout-plan domain schema (clone-remote family: checkout cost,
/// topology, sparse/partial plan, and credential posture shown before network or disk mutation).
pub const M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF: &str =
    "schemas/workspaces/m5-checkout-plan.schema.json";

/// Repo-relative path of the canonical bootstrap-evidence domain schema (import-bundle and resume-snapshot
/// families: signer / mirror provenance, digest continuity, staged trust, and resumable evidence).
pub const M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF: &str =
    "schemas/workspaces/m5-bootstrap-evidence.schema.json";

/// Repo-relative path of the already-landed repository-acquisition schema the matrix binds back to.
pub const M5_REPOSITORY_ACQUISITION_SCHEMA_REF: &str =
    "schemas/workspace/repository_acquisition.schema.json";

/// Repo-relative path of the already-landed source-acquisition-review schema the repository-bootstrap matrix
/// binds back to.
pub const M5_SOURCE_ACQUISITION_REVIEW_SCHEMA_REF: &str =
    "schemas/workspace/m5-source-acquisition-review.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_REPOSITORY_BOOTSTRAP_FIXTURE_DIR: &str = "fixtures/workspaces/m5-repository-bootstrap";

/// Repo-relative path of the checked support-export artifact.
pub const M5_REPOSITORY_BOOTSTRAP_ARTIFACT_REF: &str =
    "artifacts/release/m5-repository-bootstrap-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_REPOSITORY_BOOTSTRAP_CSV_REF: &str =
    "artifacts/release/m5-repository-bootstrap-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_REPOSITORY_BOOTSTRAP_REPORT_REF: &str =
    "artifacts/workspaces/m5-repository-bootstrap-matrix.md";

/// One of the five governed project-entry acquisition families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapFamily {
    /// Open a local checkout that already exists on disk, never recloning over it.
    OpenLocal,
    /// Clone a remote source, showing checkout cost, topology, and credential posture before the fetch.
    CloneRemote,
    /// Open an archive container, verifying its digest and extraction plan before disk mutation.
    OpenArchive,
    /// Import a bundle, preserving signer and mirror provenance across offline or mirrored fetches.
    ImportBundle,
    /// Resume (or discard) a partial-acquisition snapshot with evidence and a typed post-open queue.
    ResumeSnapshot,
}

impl M5RepositoryBootstrapFamily {
    /// Every governed acquisition family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OpenLocal,
        Self::CloneRemote,
        Self::OpenArchive,
        Self::ImportBundle,
        Self::ResumeSnapshot,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenLocal => "open_local",
            Self::CloneRemote => "clone_remote",
            Self::OpenArchive => "open_archive",
            Self::ImportBundle => "import_bundle",
            Self::ResumeSnapshot => "resume_snapshot",
        }
    }

    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// family's source-locator, checkout-plan, or bootstrap-evidence meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::OpenLocal | Self::OpenArchive => M5_SOURCE_LOCATOR_DOMAIN_SCHEMA_REF,
            Self::CloneRemote => M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
            Self::ImportBundle | Self::ResumeSnapshot => M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF,
        }
    }

    /// `true` when this family must name a controlled open-local role.
    pub const fn declares_open_local_roles(self) -> bool {
        matches!(self, Self::OpenLocal)
    }

    /// `true` when this family must name a controlled clone-remote role.
    pub const fn declares_clone_remote_roles(self) -> bool {
        matches!(self, Self::CloneRemote)
    }

    /// `true` when this family must name a controlled open-archive role.
    pub const fn declares_open_archive_roles(self) -> bool {
        matches!(self, Self::OpenArchive)
    }

    /// `true` when this family must name a controlled import-bundle role.
    pub const fn declares_import_bundle_roles(self) -> bool {
        matches!(self, Self::ImportBundle)
    }

    /// `true` when this family must name a controlled resume-snapshot role.
    pub const fn declares_resume_snapshot_roles(self) -> bool {
        matches!(self, Self::ResumeSnapshot)
    }
}

/// The single controlled repository-bootstrap-role vocabulary every shell, entry, diagnostics, admin, docs,
/// or support consumer binds to. These are the exact acceptance-criteria tokens that keep `source_locator`,
/// `checkout_plan`, `credential_posture`, `evidence_packet`, `staged_trust`, `resumable_acquisition`, and
/// `post_open_queue` meaning the same thing everywhere the repository-bootstrap grammar ships. No surface
/// invents a parallel word for any of these roles, and the credential-posture / evidence-packet /
/// staged-trust / post-open-queue roles may never hide the credential posture, lose signer or mirror
/// provenance, run a repo-owned action implicitly, or auto-execute a post-open bootstrap queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapRole {
    /// Source-locator role (where a source is and how its checkout root or container is resolved).
    SourceLocator,
    /// Checkout-plan role (the checkout cost, topology, and sparse/partial plan shown before mutation).
    CheckoutPlan,
    /// Credential-posture role (the bootstrap credential posture disclosed before network access).
    CredentialPosture,
    /// Evidence-packet role (the signer, digest, and provenance evidence a path materializes).
    EvidencePacket,
    /// Staged-trust role (trust staged so repo-owned actions never run implicitly during acquisition).
    StagedTrust,
    /// Resumable-acquisition role (interrupted acquisition resumable or discardable with evidence).
    ResumableAcquisition,
    /// Post-open-queue role (the typed post-open bootstrap queue that never auto-executes).
    PostOpenQueue,
}

impl M5RepositoryBootstrapRole {
    /// Every repository-bootstrap role token, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SourceLocator,
        Self::CheckoutPlan,
        Self::CredentialPosture,
        Self::EvidencePacket,
        Self::StagedTrust,
        Self::ResumableAcquisition,
        Self::PostOpenQueue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceLocator => "source_locator",
            Self::CheckoutPlan => "checkout_plan",
            Self::CredentialPosture => "credential_posture",
            Self::EvidencePacket => "evidence_packet",
            Self::StagedTrust => "staged_trust",
            Self::ResumableAcquisition => "resumable_acquisition",
            Self::PostOpenQueue => "post_open_queue",
        }
    }

    /// Whether this role carries credential, evidence, staged-trust, or post-open-queue truth whose
    /// per-family behavior must never hide the bootstrap credential posture, lose signer or mirror
    /// provenance, run a repo-owned action implicitly, or auto-execute a post-open bootstrap queue
    /// (`credential_posture`, `evidence_packet`, `staged_trust`, `post_open_queue`). The descriptive
    /// structure roles (`source_locator`, `checkout_plan`, `resumable_acquisition`) are inspectable
    /// descriptors rather than trust-carrying truth and so do not carry this requirement.
    pub const fn must_stage_trust_and_disclose_provenance_before_bootstrap(self) -> bool {
        matches!(
            self,
            Self::CredentialPosture
                | Self::EvidencePacket
                | Self::StagedTrust
                | Self::PostOpenQueue
        )
    }
}

/// Controlled open-local role — how opening a local checkout is named, so the located checkout root, the
/// existing checkout detected rather than recloned, the working-tree-versus-git-dir distinction, and the
/// read-only partial root offered when incomplete follow one bootstrap registry rather than rewriting clone
/// into open because a local checkout already exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OpenLocalRole {
    /// Local checkout root located.
    LocalCheckoutRootLocated,
    /// Existing checkout detected, not recloned.
    ExistingCheckoutDetectedNotRecloned,
    /// Working tree and git dir distinguished.
    WorkingTreeAndGitDirDistinguished,
    /// Read-only partial root offered when incomplete.
    ReadOnlyPartialRootOfferedWhenIncomplete,
    /// A role bound to the single bootstrap registry.
    BoundToRepositoryBootstrapRegistry,
    /// A reclone over an existing local checkout, which is disallowed.
    RecloneOverExistingLocalCheckoutDisallowed,
}

impl M5OpenLocalRole {
    /// Every open-local role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalCheckoutRootLocated,
        Self::ExistingCheckoutDetectedNotRecloned,
        Self::WorkingTreeAndGitDirDistinguished,
        Self::ReadOnlyPartialRootOfferedWhenIncomplete,
        Self::BoundToRepositoryBootstrapRegistry,
        Self::RecloneOverExistingLocalCheckoutDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCheckoutRootLocated => "local_checkout_root_located",
            Self::ExistingCheckoutDetectedNotRecloned => "existing_checkout_detected_not_recloned",
            Self::WorkingTreeAndGitDirDistinguished => "working_tree_and_git_dir_distinguished",
            Self::ReadOnlyPartialRootOfferedWhenIncomplete => {
                "read_only_partial_root_offered_when_incomplete"
            }
            Self::BoundToRepositoryBootstrapRegistry => "bound_to_repository_bootstrap_registry",
            Self::RecloneOverExistingLocalCheckoutDisallowed => {
                "reclone_over_existing_local_checkout_disallowed"
            }
        }
    }
}

/// Controlled clone-remote role — how cloning a remote source is named, so the resolved remote source
/// locator, the checkout cost and topology shown before the fetch, the credential posture disclosed before
/// network access, and the declared sparse/partial checkout plan follow one bootstrap registry rather than
/// running a repo-owned action implicitly during the clone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CloneRemoteRole {
    /// Remote source locator resolved.
    RemoteSourceLocatorResolved,
    /// Checkout cost and topology shown before fetch.
    CheckoutCostAndTopologyShownBeforeFetch,
    /// Credential posture disclosed before network access.
    CredentialPostureDisclosedBeforeNetwork,
    /// Sparse or partial checkout plan declared.
    SparseOrPartialCheckoutPlanDeclared,
    /// A role bound to the single bootstrap registry.
    BoundToRepositoryBootstrapRegistry,
    /// An implicit hook or task execution during clone, which is disallowed.
    ImplicitHookOrTaskExecutionDuringCloneDisallowed,
}

impl M5CloneRemoteRole {
    /// Every clone-remote role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RemoteSourceLocatorResolved,
        Self::CheckoutCostAndTopologyShownBeforeFetch,
        Self::CredentialPostureDisclosedBeforeNetwork,
        Self::SparseOrPartialCheckoutPlanDeclared,
        Self::BoundToRepositoryBootstrapRegistry,
        Self::ImplicitHookOrTaskExecutionDuringCloneDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemoteSourceLocatorResolved => "remote_source_locator_resolved",
            Self::CheckoutCostAndTopologyShownBeforeFetch => {
                "checkout_cost_and_topology_shown_before_fetch"
            }
            Self::CredentialPostureDisclosedBeforeNetwork => {
                "credential_posture_disclosed_before_network"
            }
            Self::SparseOrPartialCheckoutPlanDeclared => "sparse_or_partial_checkout_plan_declared",
            Self::BoundToRepositoryBootstrapRegistry => "bound_to_repository_bootstrap_registry",
            Self::ImplicitHookOrTaskExecutionDuringCloneDisallowed => {
                "implicit_hook_or_task_execution_during_clone_disallowed"
            }
        }
    }
}

/// Controlled open-archive role — how opening an archive container is named, so the located archive
/// container, the archive digest verified before extract, the extraction plan shown before disk mutation,
/// and the disclosed nested-archive topology follow one bootstrap registry rather than silently overwriting
/// a working tree during extraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OpenArchiveRole {
    /// Archive container located.
    ArchiveContainerLocated,
    /// Archive digest verified before extract.
    ArchiveDigestVerifiedBeforeExtract,
    /// Extraction plan shown before disk mutation.
    ExtractionPlanShownBeforeDiskMutation,
    /// Nested-archive topology disclosed.
    NestedArchiveTopologyDisclosed,
    /// A role bound to the single bootstrap registry.
    BoundToRepositoryBootstrapRegistry,
    /// A silent archive overwrite of a working tree, which is disallowed.
    SilentArchiveOverwriteOfWorkingTreeDisallowed,
}

impl M5OpenArchiveRole {
    /// Every open-archive role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ArchiveContainerLocated,
        Self::ArchiveDigestVerifiedBeforeExtract,
        Self::ExtractionPlanShownBeforeDiskMutation,
        Self::NestedArchiveTopologyDisclosed,
        Self::BoundToRepositoryBootstrapRegistry,
        Self::SilentArchiveOverwriteOfWorkingTreeDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArchiveContainerLocated => "archive_container_located",
            Self::ArchiveDigestVerifiedBeforeExtract => "archive_digest_verified_before_extract",
            Self::ExtractionPlanShownBeforeDiskMutation => {
                "extraction_plan_shown_before_disk_mutation"
            }
            Self::NestedArchiveTopologyDisclosed => "nested_archive_topology_disclosed",
            Self::BoundToRepositoryBootstrapRegistry => "bound_to_repository_bootstrap_registry",
            Self::SilentArchiveOverwriteOfWorkingTreeDisallowed => {
                "silent_archive_overwrite_of_working_tree_disallowed"
            }
        }
    }
}

/// Controlled import-bundle role — how importing a bundle is named, so the verified bundle signer
/// continuity, the preserved mirror and air-gap provenance, the bundle digest verified before import, and
/// the recorded offline-import evidence follow one bootstrap registry rather than losing signer provenance
/// on a mirrored fetch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ImportBundleRole {
    /// Bundle signer continuity verified.
    BundleSignerContinuityVerified,
    /// Mirror and air-gap provenance preserved.
    MirrorAndAirGapProvenancePreserved,
    /// Bundle digest verified before import.
    BundleDigestVerifiedBeforeImport,
    /// Offline-import evidence recorded.
    OfflineImportEvidenceRecorded,
    /// A role bound to the single bootstrap registry.
    BoundToRepositoryBootstrapRegistry,
    /// A lost signer provenance on a mirrored fetch, which is disallowed.
    LostSignerProvenanceOnMirroredFetchDisallowed,
}

impl M5ImportBundleRole {
    /// Every import-bundle role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BundleSignerContinuityVerified,
        Self::MirrorAndAirGapProvenancePreserved,
        Self::BundleDigestVerifiedBeforeImport,
        Self::OfflineImportEvidenceRecorded,
        Self::BoundToRepositoryBootstrapRegistry,
        Self::LostSignerProvenanceOnMirroredFetchDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundleSignerContinuityVerified => "bundle_signer_continuity_verified",
            Self::MirrorAndAirGapProvenancePreserved => "mirror_and_air_gap_provenance_preserved",
            Self::BundleDigestVerifiedBeforeImport => "bundle_digest_verified_before_import",
            Self::OfflineImportEvidenceRecorded => "offline_import_evidence_recorded",
            Self::BoundToRepositoryBootstrapRegistry => "bound_to_repository_bootstrap_registry",
            Self::LostSignerProvenanceOnMirroredFetchDisallowed => {
                "lost_signer_provenance_on_mirrored_fetch_disallowed"
            }
        }
    }
}

/// Controlled resume-snapshot role — how resuming a partial-acquisition snapshot is named, so the resumable
/// partial-acquisition state, the offered Resume / Discard / Open-read-only-partial-root choice, the typed
/// post-open bootstrap queue, and the preserved resume evidence follow one bootstrap registry rather than
/// stranding partial acquisition state without a choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResumeSnapshotRole {
    /// Partial-acquisition state resumable.
    PartialAcquisitionStateResumable,
    /// Resume / Discard / read-only choice offered.
    ResumeDiscardOrReadonlyChoiceOffered,
    /// Post-open bootstrap queue typed.
    PostOpenBootstrapQueueTyped,
    /// Resume evidence preserved.
    ResumeEvidencePreserved,
    /// A role bound to the single bootstrap registry.
    BoundToRepositoryBootstrapRegistry,
    /// A stranded partial acquisition without a choice, which is disallowed.
    StrandedPartialAcquisitionWithoutChoiceDisallowed,
}

impl M5ResumeSnapshotRole {
    /// Every resume-snapshot role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PartialAcquisitionStateResumable,
        Self::ResumeDiscardOrReadonlyChoiceOffered,
        Self::PostOpenBootstrapQueueTyped,
        Self::ResumeEvidencePreserved,
        Self::BoundToRepositoryBootstrapRegistry,
        Self::StrandedPartialAcquisitionWithoutChoiceDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PartialAcquisitionStateResumable => "partial_acquisition_state_resumable",
            Self::ResumeDiscardOrReadonlyChoiceOffered => {
                "resume_discard_or_readonly_choice_offered"
            }
            Self::PostOpenBootstrapQueueTyped => "post_open_bootstrap_queue_typed",
            Self::ResumeEvidencePreserved => "resume_evidence_preserved",
            Self::BoundToRepositoryBootstrapRegistry => "bound_to_repository_bootstrap_registry",
            Self::StrandedPartialAcquisitionWithoutChoiceDisallowed => {
                "stranded_partial_acquisition_without_choice_disallowed"
            }
        }
    }
}

/// Claimed M5 surface family that renders / consumes a repository-bootstrap family. No family may invent a
/// parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapSurfaceFamily {
    /// The shell surface.
    Shell,
    /// The project-entry surface.
    Entry,
    /// The diagnostics surface.
    Diagnostics,
    /// The admin surface.
    Admin,
    /// The docs / help surface.
    DocsHelp,
    /// The support export.
    SupportExport,
}

impl M5RepositoryBootstrapSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Shell,
        Self::Entry,
        Self::Diagnostics,
        Self::Admin,
        Self::DocsHelp,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Entry => "entry",
            Self::Diagnostics => "diagnostics",
            Self::Admin => "admin",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
        }
    }
}

/// Acquisition context a family must survive with the same truth, so a family's source-locator,
/// checkout-plan, credential-posture, evidence-packet, staged-trust, resumable, or post-open-queue meaning
/// never silently narrows or widens between acquisition shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapDeploymentLine {
    /// A first-run acquisition on a fresh machine.
    FirstRun,
    /// A returning-workspace acquisition.
    ReturningWorkspace,
    /// An offline or air-gapped acquisition.
    OfflineOrAirGapped,
    /// A mirrored-registry acquisition.
    MirroredRegistry,
    /// A resumed-after-interrupt acquisition.
    ResumedAfterInterrupt,
}

impl M5RepositoryBootstrapDeploymentLine {
    /// Every acquisition context, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FirstRun,
        Self::ReturningWorkspace,
        Self::OfflineOrAirGapped,
        Self::MirroredRegistry,
        Self::ResumedAfterInterrupt,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstRun => "first_run",
            Self::ReturningWorkspace => "returning_workspace",
            Self::OfflineOrAirGapped => "offline_or_air_gapped",
            Self::MirroredRegistry => "mirrored_registry",
            Self::ResumedAfterInterrupt => "resumed_after_interrupt",
        }
    }
}

/// Subsystem that consumes a family's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapConsumerSurface {
    /// The acquisition engine.
    AcquisitionEngine,
    /// The shell UI.
    ShellUi,
    /// The workspace service.
    WorkspaceService,
    /// The git service.
    GitService,
    /// The trust service.
    TrustService,
    /// The diagnostics surface.
    Diagnostics,
    /// The docs / help surface.
    DocsHelp,
    /// The CLI / export path.
    CliExport,
    /// The support export.
    SupportExport,
}

impl M5RepositoryBootstrapConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::AcquisitionEngine,
        Self::ShellUi,
        Self::WorkspaceService,
        Self::GitService,
        Self::TrustService,
        Self::Diagnostics,
        Self::DocsHelp,
        Self::CliExport,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AcquisitionEngine => "acquisition_engine",
            Self::ShellUi => "shell_ui",
            Self::WorkspaceService => "workspace_service",
            Self::GitService => "git_service",
            Self::TrustService => "trust_service",
            Self::Diagnostics => "diagnostics",
            Self::DocsHelp => "docs_help",
            Self::CliExport => "cli_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Non-visual / accessibility route every family must offer so no repository-bootstrap meaning disappears
/// under zoom, high contrast, keyboard-only use, or export. Records the keyboard, screen-reader, high-zoom,
/// high-contrast, CLI/export, and support-packet requirements up front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-visual cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Preserves truth under high-contrast and forced-colors modes.
    HighContrastSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5RepositoryBootstrapAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::HighContrastSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a repository-bootstrap family has degraded below its qualified state. Required on every row so a
/// stale, unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The source-locator registry source is unavailable.
    SourceLocatorSourceUnavailable,
    /// The checkout-plan source is unavailable.
    CheckoutPlanSourceUnavailable,
    /// Credential-posture disclosure evidence is unverified.
    CredentialPostureEvidenceUnverified,
    /// Signer or mirror provenance evidence is unverified.
    SignerOrMirrorProvenanceUnverified,
    /// Resume evidence is unavailable.
    ResumeEvidenceUnavailable,
}

impl M5RepositoryBootstrapDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::SourceLocatorSourceUnavailable,
        Self::CheckoutPlanSourceUnavailable,
        Self::CredentialPostureEvidenceUnverified,
        Self::SignerOrMirrorProvenanceUnverified,
        Self::ResumeEvidenceUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::SourceLocatorSourceUnavailable => "source_locator_source_unavailable",
            Self::CheckoutPlanSourceUnavailable => "checkout_plan_source_unavailable",
            Self::CredentialPostureEvidenceUnverified => "credential_posture_evidence_unverified",
            Self::SignerOrMirrorProvenanceUnverified => "signer_or_mirror_provenance_unverified",
            Self::ResumeEvidenceUnavailable => "resume_evidence_unavailable",
        }
    }
}

/// Mandatory label a claimed repository-bootstrap family must be able to show. The first three are hard
/// requirements on every family; the remaining three close the acceptance-criteria ambiguity about the
/// source locator, the checkout plan, and the credential posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapRequiredLabel {
    /// The family's stable identity.
    Identity,
    /// The family's repository-bootstrap role.
    SemanticRole,
    /// The canonical registry reference the family points at.
    RegistryReference,
    /// The source locator the family resolves.
    SourceLocator,
    /// The checkout plan the family declares.
    CheckoutPlan,
    /// The credential posture the family discloses.
    CredentialPosture,
}

impl M5RepositoryBootstrapRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::SourceLocator,
        Self::CheckoutPlan,
        Self::CredentialPosture,
    ];

    /// The three labels every claimed family must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::SourceLocator => "source_locator",
            Self::CheckoutPlan => "checkout_plan",
            Self::CredentialPosture => "credential_posture",
        }
    }
}

/// Qualification class for an M5 repository-bootstrap row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapQualificationClass {
    /// Family qualifies for the Stable claim.
    Stable,
    /// Family is narrowed to Beta.
    Beta,
    /// Family is narrowed to Preview.
    Preview,
    /// Family is experimental and not claimed.
    Experimental,
    /// Family is unavailable on this build.
    Unavailable,
    /// Family is held pending upstream resolution.
    Held,
}

impl M5RepositoryBootstrapQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the family may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a repository-bootstrap family below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapDowngradeTrigger {
    /// Acquisition rewrote clone into open because a local checkout already existed.
    RewroteCloneIntoOpenWhenLocalCheckoutAlreadyExists,
    /// Acquisition ran repo-owned actions (hooks, tasks, extensions, restores, generators) implicitly.
    RanRepoOwnedActionsImplicitlyDuringAcquisition,
    /// Signer or mirror provenance was lost across an offline or mirrored fetch.
    LostSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches,
    /// Partial acquisition was stranded without Resume / Discard / read-only choices.
    StrandedPartialAcquisitionWithoutResumeDiscardOrReadonlyChoices,
    /// Bootstrap credential posture was hidden behind generic connected-state copy.
    HidBootstrapCredentialPostureBehindGenericConnectedStateCopy,
    /// A checkout-plan boundary drifted by surface instead of following one registry.
    CheckoutPlanBoundaryDriftedBySurface,
    /// A family left its source locator unstated.
    SourceLocatorUnstated,
    /// A family left its checkout plan unstated.
    CheckoutPlanUnstated,
    /// A family left its credential posture unstated.
    CredentialPostureUnstated,
    /// A family left its canonical registry reference unstated.
    RegistryReferenceUnstated,
    /// A family left its staged-trust rule unstated.
    StagedTrustRuleUnstated,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5RepositoryBootstrapDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::RewroteCloneIntoOpenWhenLocalCheckoutAlreadyExists,
        Self::RanRepoOwnedActionsImplicitlyDuringAcquisition,
        Self::LostSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches,
        Self::StrandedPartialAcquisitionWithoutResumeDiscardOrReadonlyChoices,
        Self::HidBootstrapCredentialPostureBehindGenericConnectedStateCopy,
        Self::CheckoutPlanBoundaryDriftedBySurface,
        Self::SourceLocatorUnstated,
        Self::CheckoutPlanUnstated,
        Self::CredentialPostureUnstated,
        Self::RegistryReferenceUnstated,
        Self::StagedTrustRuleUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RewroteCloneIntoOpenWhenLocalCheckoutAlreadyExists => {
                "rewrote_clone_into_open_when_local_checkout_already_exists"
            }
            Self::RanRepoOwnedActionsImplicitlyDuringAcquisition => {
                "ran_repo_owned_actions_implicitly_during_acquisition"
            }
            Self::LostSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches => {
                "lost_signer_or_mirror_provenance_across_offline_or_mirrored_fetches"
            }
            Self::StrandedPartialAcquisitionWithoutResumeDiscardOrReadonlyChoices => {
                "stranded_partial_acquisition_without_resume_discard_or_readonly_choices"
            }
            Self::HidBootstrapCredentialPostureBehindGenericConnectedStateCopy => {
                "hid_bootstrap_credential_posture_behind_generic_connected_state_copy"
            }
            Self::CheckoutPlanBoundaryDriftedBySurface => {
                "checkout_plan_boundary_drifted_by_surface"
            }
            Self::SourceLocatorUnstated => "source_locator_unstated",
            Self::CheckoutPlanUnstated => "checkout_plan_unstated",
            Self::CredentialPostureUnstated => "credential_posture_unstated",
            Self::RegistryReferenceUnstated => "registry_reference_unstated",
            Self::StagedTrustRuleUnstated => "staged_trust_rule_unstated",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed repository-bootstrap family bound to the surface-specific truth it
/// must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepositoryBootstrapRow {
    /// Governed repository-bootstrap family.
    pub repository_bootstrap_family: M5RepositoryBootstrapFamily,
    /// Qualification class earned by this family.
    pub qualification: M5RepositoryBootstrapQualificationClass,
    /// Owner role accountable for keeping this family governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this family.
    pub surface_families: Vec<M5RepositoryBootstrapSurfaceFamily>,
    /// Acquisition contexts this family keeps the same truth across.
    pub deployment_lines: Vec<M5RepositoryBootstrapDeploymentLine>,
    /// Mandatory labels this family must be able to show (must include the three
    /// [`M5RepositoryBootstrapRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5RepositoryBootstrapRequiredLabel>,
    /// Repository-bootstrap roles this family can carry (the frozen AC vocabulary; required on every family).
    pub semantic_roles: Vec<M5RepositoryBootstrapRole>,
    /// Open-local roles this family names (open-local family only).
    pub open_local_roles: Vec<M5OpenLocalRole>,
    /// Clone-remote roles this family names (clone-remote family only).
    pub clone_remote_roles: Vec<M5CloneRemoteRole>,
    /// Open-archive roles this family names (open-archive family only).
    pub open_archive_roles: Vec<M5OpenArchiveRole>,
    /// Import-bundle roles this family names (import-bundle family only).
    pub import_bundle_roles: Vec<M5ImportBundleRole>,
    /// Resume-snapshot roles this family names (resume-snapshot family only).
    pub resume_snapshot_roles: Vec<M5ResumeSnapshotRole>,
    /// Degraded reasons this family can name (required on every family).
    pub degraded_reasons: Vec<M5RepositoryBootstrapDegradedReason>,
    /// Non-visual accessibility routes this family offers.
    pub accessibility_routes: Vec<M5RepositoryBootstrapAccessibilityRoute>,
    /// Subsystems that consume this family's projection.
    pub consumer_surfaces: Vec<M5RepositoryBootstrapConsumerSurface>,
    /// Downgrade triggers that apply to this family.
    pub downgrade_triggers: Vec<M5RepositoryBootstrapDowngradeTrigger>,
    /// Proof packet refs that keep this family current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this family (must include its own canonical domain schema so
    /// downstream surfaces have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this family never rewrites clone into open because a local checkout already exists.
    /// MUST be `false`.
    pub rewrites_clone_into_open_when_local_checkout_already_exists: bool,
    /// Hard invariant: this family never runs repo-owned actions (hooks, repo tasks, extensions, package
    /// restores, submodule or LFS hydration, generator installs) implicitly during acquisition. MUST be
    /// `false`.
    pub runs_repo_owned_actions_implicitly_during_acquisition: bool,
    /// Hard invariant: this family never loses signer or mirror provenance across offline or mirrored
    /// fetches. MUST be `false`.
    pub loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches: bool,
    /// Hard invariant: this family never strands partial acquisition state without Resume / Discard /
    /// Open-read-only-partial-root choices. MUST be `false`.
    pub strands_partial_acquisition_without_resume_discard_or_readonly_choices: bool,
    /// Hard invariant: this family never hides bootstrap credential posture behind generic connected-state
    /// copy. MUST be `false`.
    pub hides_bootstrap_credential_posture_behind_generic_connected_state_copy: bool,
}

impl M5RepositoryBootstrapRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5RepositoryBootstrapRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5RepositoryBootstrapRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.rewrites_clone_into_open_when_local_checkout_already_exists
            && !self.runs_repo_owned_actions_implicitly_during_acquisition
            && !self.loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches
            && !self.strands_partial_acquisition_without_resume_discard_or_readonly_choices
            && !self.hides_bootstrap_credential_posture_behind_generic_connected_state_copy
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepositoryBootstrapVocabularySet {
    /// Repository-bootstrap-family tokens.
    pub repository_bootstrap_families: Vec<String>,
    /// Repository-bootstrap-role tokens.
    pub semantic_roles: Vec<String>,
    /// Open-local-role tokens.
    pub open_local_roles: Vec<String>,
    /// Clone-remote-role tokens.
    pub clone_remote_roles: Vec<String>,
    /// Open-archive-role tokens.
    pub open_archive_roles: Vec<String>,
    /// Import-bundle-role tokens.
    pub import_bundle_roles: Vec<String>,
    /// Resume-snapshot-role tokens.
    pub resume_snapshot_roles: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Acquisition-context tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5RepositoryBootstrapVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            repository_bootstrap_families: tokens(&M5RepositoryBootstrapFamily::ALL, |v| {
                v.as_str()
            }),
            semantic_roles: tokens(&M5RepositoryBootstrapRole::ALL, |v| v.as_str()),
            open_local_roles: tokens(&M5OpenLocalRole::ALL, |v| v.as_str()),
            clone_remote_roles: tokens(&M5CloneRemoteRole::ALL, |v| v.as_str()),
            open_archive_roles: tokens(&M5OpenArchiveRole::ALL, |v| v.as_str()),
            import_bundle_roles: tokens(&M5ImportBundleRole::ALL, |v| v.as_str()),
            resume_snapshot_roles: tokens(&M5ResumeSnapshotRole::ALL, |v| v.as_str()),
            surface_families: tokens(&M5RepositoryBootstrapSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5RepositoryBootstrapDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5RepositoryBootstrapConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5RepositoryBootstrapAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            degraded_reasons: tokens(&M5RepositoryBootstrapDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5RepositoryBootstrapRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5RepositoryBootstrapDowngradeTrigger::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepositoryBootstrapGovernanceReview {
    /// Source locator and checkout plan stay separately inspectable.
    pub source_locator_and_checkout_plan_stay_separately_inspectable: bool,
    /// Repo-owned actions never run implicitly during acquisition.
    pub repo_owned_actions_never_run_implicitly_during_acquisition: bool,
    /// Clone is never rewritten into open because a local checkout already exists.
    pub clone_is_never_rewritten_into_open_when_local_checkout_exists: bool,
    /// Checkout cost, topology, and credential posture are shown before network or disk mutation.
    pub checkout_cost_topology_and_credential_posture_shown_before_mutation: bool,
    /// Signer and mirror provenance are preserved across offline or mirrored fetches.
    pub signer_and_mirror_provenance_preserved_across_offline_or_mirrored_fetches: bool,
    /// Interrupted acquisition stays resumable or discardable with evidence.
    pub interrupted_acquisition_stays_resumable_or_discardable_with_evidence: bool,
    /// Trust is staged so hooks, tasks, extensions, restores, and generators never run implicitly.
    pub trust_is_staged_so_repo_owned_actions_never_run_implicitly: bool,
    /// Bootstrap credential posture is never hidden behind generic connected-state copy.
    pub bootstrap_credential_posture_never_hidden_behind_generic_connected_state_copy: bool,
    /// The post-open bootstrap queue is typed and never auto-executed.
    pub post_open_bootstrap_queue_is_typed_and_never_auto_executed: bool,
    /// Every family keeps the same truth across every acquisition context.
    pub every_family_declares_acquisition_contexts: bool,
    /// Every family declares a non-visual accessibility route.
    pub every_family_declares_accessibility_route: bool,
    /// Support / export reads a single canonical repository-bootstrap source.
    pub support_export_reads_single_repository_bootstrap_source: bool,
    /// Shell, entry, diagnostics, and admin bind to a single canonical repository-bootstrap source.
    pub shell_entry_diagnostics_admin_bind_to_single_repository_bootstrap_source: bool,
    /// Later M5 rows cannot invent parallel repository-bootstrap vocabulary.
    pub later_rows_cannot_invent_parallel_repository_bootstrap_vocabulary: bool,
    /// Bootstrap truth survives zoom and high contrast.
    pub bootstrap_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when the registry is missing, stale, or not yet qualified.
    pub claims_narrow_automatically_when_registry_missing_or_stale: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepositoryBootstrapConsumerProjection {
    /// Shell and entry consume the shared repository-bootstrap truth.
    pub shell_and_entry_consume_shared_repository_bootstrap_truth: bool,
    /// Diagnostics and admin consume the shared trust-stage boundaries.
    pub diagnostics_and_admin_consume_shared_trust_stage_boundaries: bool,
    /// Git and workspace services consume the shared source locator and checkout plan.
    pub git_and_workspace_services_consume_shared_source_locator_and_checkout_plan: bool,
    /// Docs, help, and screenshots read a single repository-bootstrap source.
    pub docs_help_and_screenshots_read_single_repository_bootstrap_source: bool,
    /// Hooks, tasks, extensions, and generators bind to the shared staged-trust rule.
    pub hooks_tasks_extensions_and_generators_bind_to_shared_staged_trust_rule: bool,
    /// Support / export reads a single canonical repository-bootstrap source.
    pub support_export_reads_single_repository_bootstrap_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepositoryBootstrapProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the family.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the repository-bootstrap lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepositoryBootstrapReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting repository-bootstrap audit for the lane.
    pub repository_bootstrap_audit_ref: String,
    /// True when support/export parity is required for every family.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every family.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5RepositoryBootstrapMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RepositoryBootstrapMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Repository-bootstrap rows.
    pub repository_bootstrap_rows: Vec<M5RepositoryBootstrapRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RepositoryBootstrapVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RepositoryBootstrapGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RepositoryBootstrapConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RepositoryBootstrapProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RepositoryBootstrapReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 repository-bootstrap matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepositoryBootstrapMatrixPacket {
    /// Record kind; must equal [`M5_REPOSITORY_BOOTSTRAP_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Repository-bootstrap rows.
    pub repository_bootstrap_rows: Vec<M5RepositoryBootstrapRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RepositoryBootstrapVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RepositoryBootstrapGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RepositoryBootstrapConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RepositoryBootstrapProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RepositoryBootstrapReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RepositoryBootstrapMatrixPacket {
    /// Builds an M5 repository-bootstrap matrix packet from stable-lane input.
    pub fn new(input: M5RepositoryBootstrapMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_REPOSITORY_BOOTSTRAP_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            repository_bootstrap_rows: input.repository_bootstrap_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 repository-bootstrap matrix invariants.
    pub fn validate(&self) -> Vec<M5RepositoryBootstrapMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_REPOSITORY_BOOTSTRAP_MATRIX_RECORD_KIND {
            violations.push(M5RepositoryBootstrapMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_VERSION {
            violations.push(M5RepositoryBootstrapMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RepositoryBootstrapMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_repository_bootstrap_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 repository-bootstrap matrix serializes"),
        ) {
            violations.push(M5RepositoryBootstrapMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 repository-bootstrap matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "repository_bootstrap_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.repository_bootstrap_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.repository_bootstrap_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.repository_bootstrap_family
                    .canonical_domain_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_families = self
            .repository_bootstrap_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Repository-Bootstrap, Checkout-Plan, Trust-Stage, and Post-Open-Queue Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Repository-bootstrap families: {} ({} stable)\n",
            self.repository_bootstrap_rows.len(),
            stable_families
        ));
        out.push_str(&format!(
            "- Repository-bootstrap roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Open-local roles: {}\n",
            self.vocabulary_set.open_local_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Repository-bootstrap families\n\n");
        for row in &self.repository_bootstrap_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.repository_bootstrap_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.repository_bootstrap_family
                    .canonical_domain_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 repository-bootstrap matrix export.
#[derive(Debug)]
pub enum M5RepositoryBootstrapMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RepositoryBootstrapMatrixViolation>),
}

impl fmt::Display for M5RepositoryBootstrapMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 repository-bootstrap matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 repository-bootstrap matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RepositoryBootstrapMatrixArtifactError {}

/// Validation failures emitted by [`M5RepositoryBootstrapMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RepositoryBootstrapMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed repository-bootstrap family is missing from the matrix.
    RequiredFamilyMissing,
    /// A repository-bootstrap row is incomplete.
    RepositoryBootstrapRowIncomplete,
    /// A repository-bootstrap row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A repository-bootstrap row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A family declares no repository-bootstrap roles.
    SemanticRoleMissing,
    /// The open-local family declares no open-local roles.
    OpenLocalRoleMissing,
    /// The clone-remote family declares no clone-remote roles.
    CloneRemoteRoleMissing,
    /// The open-archive family declares no open-archive roles.
    OpenArchiveRoleMissing,
    /// The import-bundle family declares no import-bundle roles.
    ImportBundleRoleMissing,
    /// The resume-snapshot family declares no resume-snapshot roles.
    ResumeSnapshotRoleMissing,
    /// A family declares no degraded reasons.
    DegradedReasonMissing,
    /// A family declares no surface families.
    SurfaceFamilyMissing,
    /// A family declares no acquisition contexts.
    DeploymentLineMissing,
    /// A family declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A family declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A family declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A family claiming Stable is missing required proof packet refs.
    StableFamilyMissingProof,
    /// A family violates a hard invariant (rewriting clone into open when a local checkout already exists,
    /// running repo-owned actions implicitly during acquisition, losing signer or mirror provenance across
    /// an offline or mirrored fetch, stranding partial acquisition state without Resume / Discard /
    /// read-only choices, or hiding bootstrap credential posture behind generic connected-state copy).
    RepositoryBootstrapInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5RepositoryBootstrapMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::RepositoryBootstrapRowIncomplete => "repository_bootstrap_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::OpenLocalRoleMissing => "open_local_role_missing",
            Self::CloneRemoteRoleMissing => "clone_remote_role_missing",
            Self::OpenArchiveRoleMissing => "open_archive_role_missing",
            Self::ImportBundleRoleMissing => "import_bundle_role_missing",
            Self::ResumeSnapshotRoleMissing => "resume_snapshot_role_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableFamilyMissingProof => "stable_family_missing_proof",
            Self::RepositoryBootstrapInvariantViolated => "repository_bootstrap_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 repository-bootstrap matrix export.
pub fn current_stable_m5_repository_bootstrap_matrix_export(
) -> Result<M5RepositoryBootstrapMatrixPacket, M5RepositoryBootstrapMatrixArtifactError> {
    let packet: M5RepositoryBootstrapMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-repository-bootstrap-proof/support_export.json"
    )))
    .map_err(M5RepositoryBootstrapMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RepositoryBootstrapMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5RepositoryBootstrapMatrixPacket,
    violations: &mut Vec<M5RepositoryBootstrapMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF,
        M5_SOURCE_LOCATOR_DOMAIN_SCHEMA_REF,
        M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
        M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF,
        M5_REPOSITORY_ACQUISITION_SCHEMA_REF,
        M5_SOURCE_ACQUISITION_REVIEW_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5RepositoryBootstrapMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5RepositoryBootstrapMatrixPacket,
    violations: &mut Vec<M5RepositoryBootstrapMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5RepositoryBootstrapMatrixViolation::VocabularySetDrift);
    }
}

fn validate_repository_bootstrap_rows(
    packet: &M5RepositoryBootstrapMatrixPacket,
    violations: &mut Vec<M5RepositoryBootstrapMatrixViolation>,
) {
    let present: BTreeSet<M5RepositoryBootstrapFamily> = packet
        .repository_bootstrap_rows
        .iter()
        .map(|row| row.repository_bootstrap_family)
        .collect();
    for required in M5RepositoryBootstrapFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5RepositoryBootstrapMatrixViolation::RequiredFamilyMissing);
            return;
        }
    }

    for row in &packet.repository_bootstrap_rows {
        let family = row.repository_bootstrap_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5RepositoryBootstrapMatrixViolation::RepositoryBootstrapRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5RepositoryBootstrapMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_domain_schema_ref())
        {
            violations.push(M5RepositoryBootstrapMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5RepositoryBootstrapMatrixViolation::SemanticRoleMissing);
        }
        if family.declares_open_local_roles() && row.open_local_roles.is_empty() {
            violations.push(M5RepositoryBootstrapMatrixViolation::OpenLocalRoleMissing);
        }
        if family.declares_clone_remote_roles() && row.clone_remote_roles.is_empty() {
            violations.push(M5RepositoryBootstrapMatrixViolation::CloneRemoteRoleMissing);
        }
        if family.declares_open_archive_roles() && row.open_archive_roles.is_empty() {
            violations.push(M5RepositoryBootstrapMatrixViolation::OpenArchiveRoleMissing);
        }
        if family.declares_import_bundle_roles() && row.import_bundle_roles.is_empty() {
            violations.push(M5RepositoryBootstrapMatrixViolation::ImportBundleRoleMissing);
        }
        if family.declares_resume_snapshot_roles() && row.resume_snapshot_roles.is_empty() {
            violations.push(M5RepositoryBootstrapMatrixViolation::ResumeSnapshotRoleMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5RepositoryBootstrapMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5RepositoryBootstrapMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5RepositoryBootstrapMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5RepositoryBootstrapMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5RepositoryBootstrapMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5RepositoryBootstrapMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5RepositoryBootstrapMatrixViolation::StableFamilyMissingProof);
        }
        if !row.honours_invariants() {
            violations
                .push(M5RepositoryBootstrapMatrixViolation::RepositoryBootstrapInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5RepositoryBootstrapMatrixPacket,
    violations: &mut Vec<M5RepositoryBootstrapMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.source_locator_and_checkout_plan_stay_separately_inspectable,
        review.repo_owned_actions_never_run_implicitly_during_acquisition,
        review.clone_is_never_rewritten_into_open_when_local_checkout_exists,
        review.checkout_cost_topology_and_credential_posture_shown_before_mutation,
        review.signer_and_mirror_provenance_preserved_across_offline_or_mirrored_fetches,
        review.interrupted_acquisition_stays_resumable_or_discardable_with_evidence,
        review.trust_is_staged_so_repo_owned_actions_never_run_implicitly,
        review.bootstrap_credential_posture_never_hidden_behind_generic_connected_state_copy,
        review.post_open_bootstrap_queue_is_typed_and_never_auto_executed,
        review.every_family_declares_acquisition_contexts,
        review.every_family_declares_accessibility_route,
        review.support_export_reads_single_repository_bootstrap_source,
        review.shell_entry_diagnostics_admin_bind_to_single_repository_bootstrap_source,
        review.later_rows_cannot_invent_parallel_repository_bootstrap_vocabulary,
        review.bootstrap_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_registry_missing_or_stale,
    ] {
        if !ok {
            violations.push(M5RepositoryBootstrapMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RepositoryBootstrapMatrixPacket,
    violations: &mut Vec<M5RepositoryBootstrapMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_and_entry_consume_shared_repository_bootstrap_truth,
        projection.diagnostics_and_admin_consume_shared_trust_stage_boundaries,
        projection.git_and_workspace_services_consume_shared_source_locator_and_checkout_plan,
        projection.docs_help_and_screenshots_read_single_repository_bootstrap_source,
        projection.hooks_tasks_extensions_and_generators_bind_to_shared_staged_trust_rule,
        projection.support_export_reads_single_repository_bootstrap_source,
    ] {
        if !ok {
            violations.push(M5RepositoryBootstrapMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5RepositoryBootstrapMatrixPacket,
    violations: &mut Vec<M5RepositoryBootstrapMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5RepositoryBootstrapMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5RepositoryBootstrapMatrixPacket,
    violations: &mut Vec<M5RepositoryBootstrapMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.repository_bootstrap_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5RepositoryBootstrapMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled vocabulary
/// deliberately uses repository / clone / archive / bundle / checkout / credential-posture words; what is
/// rejected is a raw secret *value* shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a
/// PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
