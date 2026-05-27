//! Environment-capsule resolution + prebuild-fingerprint truth packet for
//! the M4 stable lane.
//!
//! This module pins how the devcontainer, Nix, Compose, shell/SDK, and
//! template/prebuild lanes resolve declarative inputs into one typed
//! environment-capsule object with a visible prebuild fingerprint, a
//! requested-vs-materialized identity split, structured invalidation
//! reasons, and Project Doctor finding codes that flow into the
//! support/export bundle. Surfaces (editor run surface, terminal pane,
//! task panel, CLI/headless inspector, Project Doctor, support export,
//! release proof index, Help/About proof card, conformance dashboard)
//! MUST NOT mint local copies, paraphrase fields, or fork their own
//! capsule semantics; they read this packet verbatim.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `capsule_resolution_quality` row cannot prove:
//!
//! - each typed capsule field is admitted exactly once (host or
//!   base-image identity, target plan, resolved toolchain locks,
//!   projected environment variables, secret references, writable
//!   mount model, service startup ordering, trust/network posture, and
//!   provenance) via one `capsule_field_admission` row per field,
//! - each prebuild-fingerprint component is admitted exactly once
//!   (commit-or-tree identity, capsule hash, platform/arch, policy
//!   epoch, extension lock digest, and critical toolchain digest) via
//!   one `prebuild_fingerprint_admission` row per component,
//! - each visible invalidation reason is admitted exactly once
//!   (cold path, partially-warm path, fingerprint mismatch, untrusted
//!   template metadata, blocked hook, and stale prebuild) via one
//!   `invalidation_reason_admission` row per reason,
//! - one `materialized_identity_admission` row binds both the
//!   requested template/capsule/prebuild artifact and the materialized
//!   runtime instance, and attests that prebuild reuse is never
//!   silent,
//! - each Project Doctor finding code is admitted exactly once
//!   (wrong interpreter, stale prebuild, blocked activator, drifted
//!   toolchain, and untrusted template metadata) via one
//!   `project_doctor_finding_admission` row per finding code.
//!
//! Every row binds a closed `capsule_resolution_lane_class`,
//! `capsule_resolution_row_class`, `support_class`,
//! `capsule_field_class`, `prebuild_fingerprint_component_class`,
//! `invalidation_reason_class`, `project_doctor_finding_class`,
//! `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and
//! `capsule_resolution_confidence_class` plus an `evidence_refs` array
//! and a `disclosure_ref` whenever the row is narrowed below
//! launch-stable, declares a non-`none_declared` known limit, or
//! binds a non-`none` downgrade automation.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! command lines, raw process environment bytes, raw secret bodies,
//! raw capsule bodies, or ambient credentials past the boundary. A
//! row that claims `launch_stable` while leaving its support, known
//! limit, downgrade automation, or evidence class unbound is refused;
//! the validator narrows below launch-stable instead of inheriting an
//! adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`CapsuleResolutionTruthPacket`].
pub const CAPSULE_RESOLUTION_TRUTH_PACKET_RECORD_KIND: &str =
    "harden_environment_capsule_resolution_truth_stable_packet";

/// Stable record-kind tag for [`CapsuleResolutionTruthSupportExport`].
pub const CAPSULE_RESOLUTION_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "harden_environment_capsule_resolution_truth_support_export";

/// Integer schema version for the capsule-resolution truth packet.
pub const CAPSULE_RESOLUTION_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const CAPSULE_RESOLUTION_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/harden_environment_capsule_resolution_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const CAPSULE_RESOLUTION_TRUTH_DOC_REF: &str =
    "docs/runtime/m4/harden-environment-capsule-resolution.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const CAPSULE_RESOLUTION_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/harden-environment-capsule-resolution.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const CAPSULE_RESOLUTION_TRUTH_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/harden_environment_capsule_resolution";

/// Repo-relative path of the checked-in stable packet.
pub const CAPSULE_RESOLUTION_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/harden_environment_capsule_resolution_truth_packet.json";

/// Closed capsule-resolution lane vocabulary. Every required lane MUST
/// have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleResolutionLaneClass {
    /// Devcontainer lane (`.devcontainer/devcontainer.json`,
    /// Dockerfile or OCI ref consumed by the devcontainer build).
    DevcontainerLane,
    /// Nix lane (`flake.nix`, `shell.nix`, and pinned channels).
    NixLane,
    /// Compose lane (`compose.yaml` / `docker-compose.yaml` services
    /// and startup ordering).
    ComposeLane,
    /// Shell + SDK lane (`.tool-versions`, `.python-version`,
    /// `.nvmrc`, login shell, and SDK manager descriptors).
    ShellSdkLane,
    /// Template + prebuild lane (Aureline environment manifests,
    /// requested templates, and prebuild artifacts).
    TemplatePrebuildLane,
}

impl CapsuleResolutionLaneClass {
    /// Every required capsule-resolution lane, in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::DevcontainerLane,
        Self::NixLane,
        Self::ComposeLane,
        Self::ShellSdkLane,
        Self::TemplatePrebuildLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DevcontainerLane => "devcontainer_lane",
            Self::NixLane => "nix_lane",
            Self::ComposeLane => "compose_lane",
            Self::ShellSdkLane => "shell_sdk_lane",
            Self::TemplatePrebuildLane => "template_prebuild_lane",
        }
    }
}

/// Closed capsule-resolution row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleResolutionRowClass {
    /// The lane's headline capsule-resolution qualification row.
    CapsuleResolutionQuality,
    /// A row admitting one typed capsule field the resolver MUST
    /// produce.
    CapsuleFieldAdmission,
    /// A row admitting one prebuild-fingerprint component required to
    /// gate prebuild reuse.
    PrebuildFingerprintAdmission,
    /// A row admitting one visible invalidation reason a missed
    /// fingerprint MUST surface.
    InvalidationReasonAdmission,
    /// A row binding the requested template/capsule/prebuild artifact
    /// identity, the materialized runtime instance identity, and the
    /// no-silent-prebuild-reuse attestation.
    MaterializedIdentityAdmission,
    /// A row admitting one Project Doctor finding code routed through
    /// the support/export bundle.
    ProjectDoctorFindingAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl CapsuleResolutionRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapsuleResolutionQuality => "capsule_resolution_quality",
            Self::CapsuleFieldAdmission => "capsule_field_admission",
            Self::PrebuildFingerprintAdmission => "prebuild_fingerprint_admission",
            Self::InvalidationReasonAdmission => "invalidation_reason_admission",
            Self::MaterializedIdentityAdmission => "materialized_identity_admission",
            Self::ProjectDoctorFindingAdmission => "project_doctor_finding_admission",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when this row class requires a bound capsule-field token.
    pub const fn requires_capsule_field(self) -> bool {
        matches!(self, Self::CapsuleFieldAdmission)
    }

    /// True when this row class requires a bound prebuild-fingerprint
    /// component token.
    pub const fn requires_prebuild_fingerprint_component(self) -> bool {
        matches!(self, Self::PrebuildFingerprintAdmission)
    }

    /// True when this row class requires a bound invalidation-reason
    /// token.
    pub const fn requires_invalidation_reason(self) -> bool {
        matches!(self, Self::InvalidationReasonAdmission)
    }

    /// True when this row class requires a bound Project Doctor
    /// finding token.
    pub const fn requires_project_doctor_finding(self) -> bool {
        matches!(self, Self::ProjectDoctorFindingAdmission)
    }
}

/// Closed support-class vocabulary applied to a capsule-resolution row.
/// A row is never `launch_stable` while its known limit, downgrade
/// automation, or evidence class is unbound; the validator demotes it
/// instead of inheriting an adjacent launch-stable row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleResolutionSupportClass {
    /// Row claims M4 launch-stable grade for the capsule-resolution lane.
    LaunchStable,
    /// Row is intentionally narrowed below launch-stable; the narrowing is disclosed.
    LaunchStableBelow,
    /// Row is at beta-grade only (capability sample, not launch-stable).
    BetaGradeOnly,
    /// Row is at preview only (under-review wedge).
    PreviewOnly,
    /// Row carries a precisely labeled unsupported gap.
    Unsupported,
    /// Row has no bound support class; this never qualifies stable.
    SupportUnbound,
}

impl CapsuleResolutionSupportClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchStable => "launch_stable",
            Self::LaunchStableBelow => "launch_stable_below",
            Self::BetaGradeOnly => "beta_grade_only",
            Self::PreviewOnly => "preview_only",
            Self::Unsupported => "unsupported",
            Self::SupportUnbound => "support_unbound",
        }
    }

    /// True when this support class satisfies the support-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::SupportUnbound)
    }

    /// True when the support class must surface a disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::LaunchStable)
    }
}

/// Closed capsule-field vocabulary. Every lane claiming `launch_stable`
/// MUST publish a `capsule_field_admission` row for each required field
/// so the resolver produces one typed environment-capsule object
/// instead of surface-local heuristics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleFieldClass {
    /// Resolved host or base-image identity (e.g. devcontainer base
    /// image, Compose service image, host kernel descriptor).
    HostOrBaseImageIdentity,
    /// Resolved target plan (which capsule, which target, which
    /// service ordering).
    TargetPlan,
    /// Resolved toolchain locks (interpreter version, SDK version,
    /// package manager pins).
    ResolvedToolchainLocks,
    /// Projected environment variables (typed names + provenance, no
    /// raw values).
    ProjectedEnvironmentVariables,
    /// Secret references only (vault refs, OS keychain refs, never
    /// raw secret bodies).
    SecretReferences,
    /// Writable mount model (declared mounts, scopes, and write
    /// authority).
    WritableMountModel,
    /// Service startup ordering (Compose dependency graph, hook
    /// ordering, post-start scripts as named refs).
    ServiceStartupOrdering,
    /// Trust + network posture (egress class, ingress class, trusted
    /// host list, declared network policy).
    TrustNetworkPosture,
    /// Provenance describing why this capsule was chosen (source
    /// inputs, precedence ladder, hint origin).
    Provenance,
    /// The row is not bound to a capsule field (non-field row classes).
    NotApplicable,
}

impl CapsuleFieldClass {
    /// Every required capsule field per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 9] = [
        Self::HostOrBaseImageIdentity,
        Self::TargetPlan,
        Self::ResolvedToolchainLocks,
        Self::ProjectedEnvironmentVariables,
        Self::SecretReferences,
        Self::WritableMountModel,
        Self::ServiceStartupOrdering,
        Self::TrustNetworkPosture,
        Self::Provenance,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostOrBaseImageIdentity => "host_or_base_image_identity",
            Self::TargetPlan => "target_plan",
            Self::ResolvedToolchainLocks => "resolved_toolchain_locks",
            Self::ProjectedEnvironmentVariables => "projected_environment_variables",
            Self::SecretReferences => "secret_references",
            Self::WritableMountModel => "writable_mount_model",
            Self::ServiceStartupOrdering => "service_startup_ordering",
            Self::TrustNetworkPosture => "trust_network_posture",
            Self::Provenance => "provenance",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed prebuild-fingerprint component vocabulary. Every lane
/// claiming `launch_stable` MUST publish a
/// `prebuild_fingerprint_admission` row for each required component so
/// prebuild reuse is gated by an inspectable fingerprint instead of
/// snapshot folklore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrebuildFingerprintComponentClass {
    /// Commit or tree identity (Git commit hash or repo tree hash).
    CommitOrTreeIdentity,
    /// Capsule hash (digest of the typed capsule object).
    CapsuleHash,
    /// Platform / arch (OS family, CPU arch, libc).
    PlatformArch,
    /// Policy epoch (capability + trust envelope version).
    PolicyEpoch,
    /// Extension lock digest (installed extension set lock).
    ExtensionLockDigest,
    /// Critical toolchain digest (interpreter / SDK / package manager
    /// digests).
    CriticalToolchainDigest,
    /// The row is not bound to a prebuild-fingerprint component
    /// (non-component row classes).
    NotApplicable,
}

impl PrebuildFingerprintComponentClass {
    /// Every required prebuild-fingerprint component per
    /// `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::CommitOrTreeIdentity,
        Self::CapsuleHash,
        Self::PlatformArch,
        Self::PolicyEpoch,
        Self::ExtensionLockDigest,
        Self::CriticalToolchainDigest,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommitOrTreeIdentity => "commit_or_tree_identity",
            Self::CapsuleHash => "capsule_hash",
            Self::PlatformArch => "platform_arch",
            Self::PolicyEpoch => "policy_epoch",
            Self::ExtensionLockDigest => "extension_lock_digest",
            Self::CriticalToolchainDigest => "critical_toolchain_digest",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed invalidation-reason vocabulary. Every lane claiming
/// `launch_stable` MUST publish an `invalidation_reason_admission` row
/// for each visible invalidation reason so a missed fingerprint always
/// yields a cold or partially-warm path with an explicit explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvalidationReasonClass {
    /// No matching prebuild; cold path with the capsule materialized
    /// from scratch.
    ColdPath,
    /// Partial match; warm capsule with the drifted layers re-derived.
    PartiallyWarmPath,
    /// Fingerprint mismatch on commit/tree, capsule hash, or
    /// platform/arch.
    FingerprintMismatch,
    /// Untrusted template metadata; reuse refused until the template
    /// is reviewed.
    UntrustedTemplateMetadata,
    /// Repository hook would have to run to warm the capsule; reuse
    /// blocked and the cold path runs only after reviewer approval.
    BlockedHook,
    /// Reused prebuild detected as stale (extension lock or toolchain
    /// digest drifted); fallback to cold path with a visible reason.
    StalePrebuild,
    /// The row is not bound to an invalidation reason (non-reason row
    /// classes).
    NotApplicable,
}

impl InvalidationReasonClass {
    /// Every certified invalidation reason in declaration order. A
    /// `launch_stable` lane MUST cover every reason.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::ColdPath,
        Self::PartiallyWarmPath,
        Self::FingerprintMismatch,
        Self::UntrustedTemplateMetadata,
        Self::BlockedHook,
        Self::StalePrebuild,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ColdPath => "cold_path",
            Self::PartiallyWarmPath => "partially_warm_path",
            Self::FingerprintMismatch => "fingerprint_mismatch",
            Self::UntrustedTemplateMetadata => "untrusted_template_metadata",
            Self::BlockedHook => "blocked_hook",
            Self::StalePrebuild => "stale_prebuild",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed Project Doctor finding-code vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `project_doctor_finding_admission`
/// row for each required finding so Aureline can explain wrong
/// interpreter, stale prebuild, blocked activator, drifted toolchain,
/// and untrusted template metadata cases without hidden logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectDoctorFindingClass {
    /// Wrong interpreter resolved (e.g. system Python instead of the
    /// declared pyenv version).
    WrongInterpreter,
    /// Stale prebuild reused on an outdated fingerprint.
    StalePrebuild,
    /// Activator (env-manager, devcontainer build, capsule activator)
    /// was blocked by trust, policy, or capability.
    BlockedActivator,
    /// Drifted toolchain (interpreter, SDK, or package manager) on a
    /// reused capsule.
    DriftedToolchain,
    /// Template metadata was untrusted; capsule selection refused
    /// until the template is reviewed.
    UntrustedTemplateMetadata,
    /// The row is not bound to a Project Doctor finding (non-finding
    /// row classes).
    NotApplicable,
}

impl ProjectDoctorFindingClass {
    /// Every certified Project Doctor finding in declaration order. A
    /// `launch_stable` lane MUST cover every finding.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::WrongInterpreter,
        Self::StalePrebuild,
        Self::BlockedActivator,
        Self::DriftedToolchain,
        Self::UntrustedTemplateMetadata,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongInterpreter => "wrong_interpreter",
            Self::StalePrebuild => "stale_prebuild",
            Self::BlockedActivator => "blocked_activator",
            Self::DriftedToolchain => "drifted_toolchain",
            Self::UntrustedTemplateMetadata => "untrusted_template_metadata",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed evidence-class vocabulary describing what backs a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleResolutionEvidenceClass {
    /// The row is backed by an automated functional / unit suite.
    AutomatedFunctionalEvidence,
    /// The row is backed by a conformance / interoperability suite.
    ConformanceSuiteEvidence,
    /// The row is backed by a failure / recovery drill.
    FailureRecoveryDrillEvidence,
    /// The row is backed by design-QA / UX validation.
    DesignQaEvidence,
    /// The row is backed by release-evidence review.
    ReleaseEvidenceReview,
    /// The row is backed by a fixture-repo capture.
    FixtureRepoEvidence,
    /// The row is backed by a benchmark / fitness-function capture.
    BenchmarkEvidence,
    /// The row is backed by a docs/help disclosure (gap label only).
    DocsDisclosureEvidence,
    /// The row has no bound evidence class; this never qualifies stable.
    EvidenceUnbound,
}

impl CapsuleResolutionEvidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutomatedFunctionalEvidence => "automated_functional_evidence",
            Self::ConformanceSuiteEvidence => "conformance_suite_evidence",
            Self::FailureRecoveryDrillEvidence => "failure_recovery_drill_evidence",
            Self::DesignQaEvidence => "design_qa_evidence",
            Self::ReleaseEvidenceReview => "release_evidence_review",
            Self::FixtureRepoEvidence => "fixture_repo_evidence",
            Self::BenchmarkEvidence => "benchmark_evidence",
            Self::DocsDisclosureEvidence => "docs_disclosure_evidence",
            Self::EvidenceUnbound => "evidence_unbound",
        }
    }

    /// True when this evidence class satisfies the evidence-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::EvidenceUnbound)
    }
}

/// Closed known-limit vocabulary attached to a capsule-resolution row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleResolutionKnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The lane only certifies the devcontainer subset.
    DevcontainerSubsetOnly,
    /// The lane only certifies the Nix subset.
    NixSubsetOnly,
    /// The lane only certifies the Compose subset.
    ComposeSubsetOnly,
    /// The lane only certifies the shell/SDK subset.
    ShellSdkSubsetOnly,
    /// The lane only certifies the template/prebuild subset.
    TemplatePrebuildSubsetOnly,
    /// The lane only certifies a subset of the nine required typed
    /// capsule fields.
    CapsuleFieldSubsetOnly,
    /// The lane only certifies a subset of the six required prebuild
    /// fingerprint components.
    PrebuildFingerprintSubsetOnly,
    /// The lane only certifies a subset of the six required
    /// invalidation reasons.
    InvalidationReasonSubsetOnly,
    /// The lane only certifies a subset of the five required Project
    /// Doctor finding codes.
    ProjectDoctorFindingSubsetOnly,
    /// The lane certifies an unsupported declarative input gap.
    UnsupportedDeclarativeInput,
    /// The lane is at beta-grade-only capability sample.
    BetaCapabilitySampleOnly,
    /// The row has no bound known limit class; this never qualifies stable.
    LimitUnbound,
}

impl CapsuleResolutionKnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::DevcontainerSubsetOnly => "devcontainer_subset_only",
            Self::NixSubsetOnly => "nix_subset_only",
            Self::ComposeSubsetOnly => "compose_subset_only",
            Self::ShellSdkSubsetOnly => "shell_sdk_subset_only",
            Self::TemplatePrebuildSubsetOnly => "template_prebuild_subset_only",
            Self::CapsuleFieldSubsetOnly => "capsule_field_subset_only",
            Self::PrebuildFingerprintSubsetOnly => "prebuild_fingerprint_subset_only",
            Self::InvalidationReasonSubsetOnly => "invalidation_reason_subset_only",
            Self::ProjectDoctorFindingSubsetOnly => "project_doctor_finding_subset_only",
            Self::UnsupportedDeclarativeInput => "unsupported_declarative_input",
            Self::BetaCapabilitySampleOnly => "beta_capability_sample_only",
            Self::LimitUnbound => "limit_unbound",
        }
    }

    /// True when this known-limit class satisfies the limit-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::LimitUnbound)
    }

    /// True when this known-limit class must surface an explicit disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::NoneDeclared | Self::LimitUnbound)
    }
}

/// Closed downgrade-automation vocabulary attached to a
/// capsule-resolution row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleResolutionDowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when a required typed capsule field is
    /// unbound.
    AutoNarrowOnCapsuleFieldGap,
    /// Automatically narrow when a required prebuild-fingerprint
    /// component is unbound.
    AutoNarrowOnFingerprintGap,
    /// Automatically narrow when a required invalidation reason is
    /// unbound.
    AutoNarrowOnInvalidationReasonGap,
    /// Automatically narrow when a Project Doctor finding code is
    /// unbound.
    AutoNarrowOnProjectDoctorFindingGap,
    /// Automatically narrow when the requested-vs-materialized
    /// identity split drifts (one side missing or both equal without
    /// attestation).
    AutoNarrowOnMaterializedIdentityDrift,
    /// Automatically narrow when prebuild reuse would be silent (no
    /// invalidation reason surfaced on a missed fingerprint).
    AutoNarrowOnSilentPrebuildReuse,
    /// Automatically block when required evidence is missing.
    AutoBlockOnMissingEvidence,
    /// Manual-only review required until automation lands.
    ManualOnlyPendingReview,
    /// Automation is unbound; this never qualifies stable.
    AutomationUnbound,
}

impl CapsuleResolutionDowngradeAutomationClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AutoNarrowOnCapsuleFieldGap => "auto_narrow_on_capsule_field_gap",
            Self::AutoNarrowOnFingerprintGap => "auto_narrow_on_fingerprint_gap",
            Self::AutoNarrowOnInvalidationReasonGap => "auto_narrow_on_invalidation_reason_gap",
            Self::AutoNarrowOnProjectDoctorFindingGap => "auto_narrow_on_project_doctor_finding_gap",
            Self::AutoNarrowOnMaterializedIdentityDrift => {
                "auto_narrow_on_materialized_identity_drift"
            }
            Self::AutoNarrowOnSilentPrebuildReuse => "auto_narrow_on_silent_prebuild_reuse",
            Self::AutoBlockOnMissingEvidence => "auto_block_on_missing_evidence",
            Self::ManualOnlyPendingReview => "manual_only_pending_review",
            Self::AutomationUnbound => "automation_unbound",
        }
    }

    /// True when this automation class satisfies the automation-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::AutomationUnbound)
    }

    /// True when this automation class must surface an explicit disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::None | Self::AutomationUnbound)
    }
}

/// Closed confidence-class vocabulary for a capsule-resolution row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleResolutionConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until evidence grows.
    LowConfidence,
}

impl CapsuleResolutionConfidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighConfidence => "high_confidence",
            Self::MediumConfidence => "medium_confidence",
            Self::LowConfidence => "low_confidence",
        }
    }
}

/// Stable promotion state derived from packet validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleResolutionPromotionState {
    /// Packet certifies a stable claim across all required rows.
    Stable,
    /// Packet narrows below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl CapsuleResolutionPromotionState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleResolutionFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker finding that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for the capsule-resolution packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleResolutionFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required capsule-resolution lane has no row.
    MissingLaneCoverage,
    /// A lane claiming launch_stable is missing a required typed
    /// capsule-field admission row.
    MissingCapsuleFieldCoverage,
    /// A lane claiming launch_stable is missing a required
    /// prebuild-fingerprint component admission row.
    MissingPrebuildFingerprintCoverage,
    /// A lane claiming launch_stable is missing a required
    /// invalidation-reason admission row.
    MissingInvalidationReasonCoverage,
    /// A lane claiming launch_stable is missing a required Project
    /// Doctor finding admission row.
    MissingProjectDoctorFindingCoverage,
    /// A lane claiming launch_stable is missing the required
    /// materialized-identity admission row.
    MissingMaterializedIdentityAdmission,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A row claims launch_stable while one or more bindings is unbound.
    LaunchStableWithUnboundBinding,
    /// A row narrowed below launch_stable drops its disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row with a non-`none_declared` known limit drops its
    /// disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its
    /// disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// A capsule-field-admission row drops its capsule field binding.
    CapsuleFieldNotApplicable,
    /// A non-capsule-field row binds a field it cannot certify.
    CapsuleFieldNotPermittedOnRowClass,
    /// A prebuild-fingerprint-admission row drops its component binding.
    PrebuildFingerprintNotApplicable,
    /// A non-prebuild-fingerprint row binds a component it cannot
    /// certify.
    PrebuildFingerprintNotPermittedOnRowClass,
    /// An invalidation-reason-admission row drops its reason binding.
    InvalidationReasonNotApplicable,
    /// A non-invalidation row binds an invalidation reason it cannot
    /// certify.
    InvalidationReasonNotPermittedOnRowClass,
    /// A project-doctor-finding-admission row drops its finding-code
    /// binding.
    ProjectDoctorFindingNotApplicable,
    /// A non-project-doctor row binds a finding code it cannot
    /// certify.
    ProjectDoctorFindingNotPermittedOnRowClass,
    /// A materialized-identity-admission row does not bind a requested
    /// artifact id.
    MaterializedIdentityAdmissionMissingRequestedArtifact,
    /// A materialized-identity-admission row does not bind a
    /// materialized runtime instance id.
    MaterializedIdentityAdmissionMissingMaterializedRuntime,
    /// A materialized-identity-admission row equates the requested and
    /// materialized identities without attesting prebuild-reuse
    /// honesty.
    MaterializedIdentityAdmissionEquatesIdentitiesWithoutAttestation,
    /// A materialized-identity-admission row does not attest the
    /// no-silent-prebuild-reuse invariant.
    MaterializedIdentityAdmissionAdmitsSilentPrebuildReuse,
    /// A row admits raw command lines, process environment bytes, or
    /// other private material past the boundary.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops capsule-resolution truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the capsule-field vocabulary.
    CapsuleFieldVocabularyCollapsed,
    /// A projection collapses the prebuild-fingerprint component vocabulary.
    PrebuildFingerprintVocabularyCollapsed,
    /// A projection collapses the invalidation-reason vocabulary.
    InvalidationReasonVocabularyCollapsed,
    /// A projection collapses the project-doctor-finding vocabulary.
    ProjectDoctorFindingVocabularyCollapsed,
    /// A projection collapses the known-limit vocabulary.
    KnownLimitVocabularyCollapsed,
    /// A projection collapses the downgrade-automation vocabulary.
    DowngradeAutomationVocabularyCollapsed,
    /// A projection collapses the evidence-class vocabulary.
    EvidenceClassVocabularyCollapsed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl CapsuleResolutionFindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingLaneCoverage => "missing_lane_coverage",
            Self::MissingCapsuleFieldCoverage => "missing_capsule_field_coverage",
            Self::MissingPrebuildFingerprintCoverage => "missing_prebuild_fingerprint_coverage",
            Self::MissingInvalidationReasonCoverage => "missing_invalidation_reason_coverage",
            Self::MissingProjectDoctorFindingCoverage => {
                "missing_project_doctor_finding_coverage"
            }
            Self::MissingMaterializedIdentityAdmission => {
                "missing_materialized_identity_admission"
            }
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::LaunchStableWithUnboundBinding => "launch_stable_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::CapsuleFieldNotApplicable => "capsule_field_not_applicable",
            Self::CapsuleFieldNotPermittedOnRowClass => "capsule_field_not_permitted_on_row_class",
            Self::PrebuildFingerprintNotApplicable => "prebuild_fingerprint_not_applicable",
            Self::PrebuildFingerprintNotPermittedOnRowClass => {
                "prebuild_fingerprint_not_permitted_on_row_class"
            }
            Self::InvalidationReasonNotApplicable => "invalidation_reason_not_applicable",
            Self::InvalidationReasonNotPermittedOnRowClass => {
                "invalidation_reason_not_permitted_on_row_class"
            }
            Self::ProjectDoctorFindingNotApplicable => "project_doctor_finding_not_applicable",
            Self::ProjectDoctorFindingNotPermittedOnRowClass => {
                "project_doctor_finding_not_permitted_on_row_class"
            }
            Self::MaterializedIdentityAdmissionMissingRequestedArtifact => {
                "materialized_identity_admission_missing_requested_artifact"
            }
            Self::MaterializedIdentityAdmissionMissingMaterializedRuntime => {
                "materialized_identity_admission_missing_materialized_runtime"
            }
            Self::MaterializedIdentityAdmissionEquatesIdentitiesWithoutAttestation => {
                "materialized_identity_admission_equates_identities_without_attestation"
            }
            Self::MaterializedIdentityAdmissionAdmitsSilentPrebuildReuse => {
                "materialized_identity_admission_admits_silent_prebuild_reuse"
            }
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::CapsuleFieldVocabularyCollapsed => "capsule_field_vocabulary_collapsed",
            Self::PrebuildFingerprintVocabularyCollapsed => {
                "prebuild_fingerprint_vocabulary_collapsed"
            }
            Self::InvalidationReasonVocabularyCollapsed => {
                "invalidation_reason_vocabulary_collapsed"
            }
            Self::ProjectDoctorFindingVocabularyCollapsed => {
                "project_doctor_finding_vocabulary_collapsed"
            }
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the capsule-resolution packet
/// verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleResolutionConsumerSurface {
    /// Editor run / launch surface (per-pane "why this capsule?" chip).
    EditorRunSurface,
    /// Terminal pane chrome and session header.
    TerminalPane,
    /// Task panel chrome and per-run header.
    TaskPanel,
    /// CLI or headless inspection surface (`aureline env inspect`).
    CliHeadless,
    /// Project Doctor surface (capsule + prebuild diagnosis).
    ProjectDoctor,
    /// Support export bundle surface.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
    /// Help/About proof card surface.
    HelpAbout,
    /// Conformance dashboard surface.
    ConformanceDashboard,
}

impl CapsuleResolutionConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 9] = [
        Self::EditorRunSurface,
        Self::TerminalPane,
        Self::TaskPanel,
        Self::CliHeadless,
        Self::ProjectDoctor,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorRunSurface => "editor_run_surface",
            Self::TerminalPane => "terminal_pane",
            Self::TaskPanel => "task_panel",
            Self::CliHeadless => "cli_headless",
            Self::ProjectDoctor => "project_doctor",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
            Self::HelpAbout => "help_about",
            Self::ConformanceDashboard => "conformance_dashboard",
        }
    }
}

/// One validation finding emitted by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleResolutionValidationFinding {
    /// Closed finding kind.
    pub finding_kind: CapsuleResolutionFindingKind,
    /// Finding severity.
    pub severity: CapsuleResolutionFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl CapsuleResolutionValidationFinding {
    fn new(
        finding_kind: CapsuleResolutionFindingKind,
        severity: CapsuleResolutionFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// One capsule-resolution truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleResolutionRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Capsule-resolution lane this row certifies.
    pub lane_class: CapsuleResolutionLaneClass,
    /// Capsule-resolution row class.
    pub row_class: CapsuleResolutionRowClass,
    /// Support class claimed by the row.
    pub support_class: CapsuleResolutionSupportClass,
    /// Typed capsule field certified by the row (or `not_applicable`).
    pub capsule_field_class: CapsuleFieldClass,
    /// Prebuild-fingerprint component certified by the row (or
    /// `not_applicable`).
    pub prebuild_fingerprint_component_class: PrebuildFingerprintComponentClass,
    /// Invalidation reason admitted by the row (or `not_applicable`).
    pub invalidation_reason_class: InvalidationReasonClass,
    /// Project Doctor finding code admitted by the row (or
    /// `not_applicable`).
    pub project_doctor_finding_class: ProjectDoctorFindingClass,
    /// Evidence class backing the row.
    pub evidence_class: CapsuleResolutionEvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: CapsuleResolutionKnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: CapsuleResolutionDowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: CapsuleResolutionConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `launch_stable`, declares a non-`none_declared` known limit, or
    /// binds a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// For materialized_identity_admission rows, the bound requested
    /// template/capsule/prebuild artifact identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_artifact_identity_binding: Option<String>,
    /// For materialized_identity_admission rows, the bound
    /// materialized runtime instance identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub materialized_runtime_identity_binding: Option<String>,
    /// For materialized_identity_admission rows, true when the row
    /// attests that prebuild reuse always surfaces a visible
    /// invalidation reason and never reuses a fingerprint silently.
    #[serde(default)]
    pub no_silent_prebuild_reuse: bool,
    /// True when raw command lines / process env bytes / raw capsule
    /// bodies are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl CapsuleResolutionRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleResolutionConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: CapsuleResolutionConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Capsule-resolution packet id consumed by the projection.
    pub capsule_resolution_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the lane vocabulary is preserved verbatim.
    pub preserves_lane_vocabulary: bool,
    /// True when the row-class vocabulary is preserved verbatim.
    pub preserves_row_class_vocabulary: bool,
    /// True when the support-class vocabulary is preserved verbatim.
    pub preserves_support_class_vocabulary: bool,
    /// True when the capsule-field vocabulary is preserved verbatim.
    pub preserves_capsule_field_vocabulary: bool,
    /// True when the prebuild-fingerprint vocabulary is preserved
    /// verbatim.
    pub preserves_prebuild_fingerprint_vocabulary: bool,
    /// True when the invalidation-reason vocabulary is preserved
    /// verbatim.
    pub preserves_invalidation_reason_vocabulary: bool,
    /// True when the project-doctor-finding vocabulary is preserved
    /// verbatim.
    pub preserves_project_doctor_finding_vocabulary: bool,
    /// True when the known-limit vocabulary is preserved verbatim.
    pub preserves_known_limit_vocabulary: bool,
    /// True when the downgrade-automation vocabulary is preserved verbatim.
    pub preserves_downgrade_automation_vocabulary: bool,
    /// True when the evidence-class vocabulary is preserved verbatim.
    pub preserves_evidence_class_vocabulary: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl CapsuleResolutionConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.capsule_resolution_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_capsule_field_vocabulary
            && self.preserves_prebuild_fingerprint_vocabulary
            && self.preserves_invalidation_reason_vocabulary
            && self.preserves_project_doctor_finding_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`CapsuleResolutionTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleResolutionTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Capsule-resolution lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<CapsuleResolutionLaneClass>,
    /// Capsule-resolution rows.
    #[serde(default)]
    pub rows: Vec<CapsuleResolutionRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<CapsuleResolutionConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Runtime-owned packet certifying devcontainer, Nix, Compose,
/// shell/SDK, and template/prebuild capsule-resolution at the M4
/// launch-stable grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleResolutionTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Capsule-resolution lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<CapsuleResolutionLaneClass>,
    /// Capsule-resolution rows.
    #[serde(default)]
    pub rows: Vec<CapsuleResolutionRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<CapsuleResolutionConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: CapsuleResolutionPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<CapsuleResolutionValidationFinding>,
}

impl CapsuleResolutionTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: CapsuleResolutionTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: CAPSULE_RESOLUTION_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: CAPSULE_RESOLUTION_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_lanes: input.covered_lanes,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: CapsuleResolutionPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable capsule-resolution invariants.
    pub fn validate(&self) -> Vec<CapsuleResolutionValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == CapsuleResolutionFindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: CapsuleResolutionConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique lane tokens observed across rows.
    pub fn lane_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.lane_class);
        }
        set.into_iter()
            .map(CapsuleResolutionLaneClass::as_str)
            .collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(CapsuleResolutionRowClass::as_str)
            .collect()
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.support_class);
        }
        set.into_iter()
            .map(CapsuleResolutionSupportClass::as_str)
            .collect()
    }

    /// Returns the unique capsule-field tokens observed across rows.
    pub fn capsule_field_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.capsule_field_class);
        }
        set.into_iter().map(CapsuleFieldClass::as_str).collect()
    }

    /// Returns the unique prebuild-fingerprint component tokens observed across rows.
    pub fn prebuild_fingerprint_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.prebuild_fingerprint_component_class);
        }
        set.into_iter()
            .map(PrebuildFingerprintComponentClass::as_str)
            .collect()
    }

    /// Returns the unique invalidation-reason tokens observed across rows.
    pub fn invalidation_reason_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.invalidation_reason_class);
        }
        set.into_iter()
            .map(InvalidationReasonClass::as_str)
            .collect()
    }

    /// Returns the unique project-doctor-finding tokens observed across rows.
    pub fn project_doctor_finding_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.project_doctor_finding_class);
        }
        set.into_iter()
            .map(ProjectDoctorFindingClass::as_str)
            .collect()
    }

    /// Returns the unique evidence-class tokens observed across rows.
    pub fn evidence_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.evidence_class);
        }
        set.into_iter()
            .map(CapsuleResolutionEvidenceClass::as_str)
            .collect()
    }

    /// Returns the unique known-limit tokens observed across rows.
    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.known_limit_class);
        }
        set.into_iter()
            .map(CapsuleResolutionKnownLimitClass::as_str)
            .collect()
    }

    /// Returns the unique downgrade-automation tokens observed across rows.
    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_automation_class);
        }
        set.into_iter()
            .map(CapsuleResolutionDowngradeAutomationClass::as_str)
            .collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> CapsuleResolutionTruthSupportExport {
        CapsuleResolutionTruthSupportExport {
            record_kind: CAPSULE_RESOLUTION_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: CAPSULE_RESOLUTION_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            capsule_resolution_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            capsule_resolution_packet: self.clone(),
        }
    }

    fn derived_findings(
        &self,
        include_record_fields: bool,
    ) -> Vec<CapsuleResolutionValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != CAPSULE_RESOLUTION_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(CapsuleResolutionValidationFinding::new(
                CapsuleResolutionFindingKind::WrongRecordKind,
                CapsuleResolutionFindingSeverity::Blocker,
                "capsule-resolution packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != CAPSULE_RESOLUTION_TRUTH_SCHEMA_VERSION
        {
            findings.push(CapsuleResolutionValidationFinding::new(
                CapsuleResolutionFindingKind::WrongSchemaVersion,
                CapsuleResolutionFindingSeverity::Blocker,
                "capsule-resolution packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(CapsuleResolutionValidationFinding::new(
                CapsuleResolutionFindingKind::MissingIdentity,
                CapsuleResolutionFindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }
        if self.covered_lanes.is_empty() {
            findings.push(CapsuleResolutionValidationFinding::new(
                CapsuleResolutionFindingKind::MissingLaneCoverage,
                CapsuleResolutionFindingSeverity::Blocker,
                "packet must declare at least one covered capsule-resolution lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::MissingLaneCoverage,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!("no row covers capsule-resolution lane {}", lane.as_str()),
                ));
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::MissingIdentity,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!("row {} identity or timestamp is empty", row.row_id),
                ));
            }
            if !row.raw_source_material_excluded {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::RawSourceMaterialPresent,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "row {} admits raw command lines, raw env bytes, or raw capsule bodies past the boundary",
                        row.row_id
                    ),
                ));
            }
            if !row.secrets_excluded {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::SecretsPresent,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!("row {} admits secrets past the boundary", row.row_id),
                ));
            }
            if !row.ambient_authority_excluded {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::AmbientAuthorityPresent,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "row {} admits ambient authority/credentials past the boundary",
                        row.row_id
                    ),
                ));
            }

            if !row.support_class.is_bound() {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::MissingSupportClass,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!("row {} has no bound support class", row.row_id),
                ));
            }
            if !row.known_limit_class.is_bound() {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::MissingKnownLimit,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!("row {} has no bound known-limit class", row.row_id),
                ));
            }
            if !row.downgrade_automation_class.is_bound() {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::MissingDowngradeAutomation,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "row {} has no bound downgrade-automation class",
                        row.row_id
                    ),
                ));
            }
            if !row.evidence_class.is_bound() {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::MissingEvidenceClass,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!("row {} has no bound evidence class", row.row_id),
                ));
            }

            if matches!(
                row.support_class,
                CapsuleResolutionSupportClass::LaunchStable
            ) && !row.all_bindings_satisfied()
            {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::LaunchStableWithUnboundBinding,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "row {} claims launch_stable while a binding (support, known limit, downgrade automation, or evidence) is unbound",
                        row.row_id
                    ),
                ));
            }

            if row.support_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::NarrowedRowMissingDisclosureRef,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "row {} has support class {} without a disclosure ref",
                        row.row_id,
                        row.support_class.as_str()
                    ),
                ));
            }
            if row.known_limit_class.requires_explicit_disclosure() && row.disclosure_ref.is_none()
            {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::KnownLimitMissingDisclosureRef,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "row {} discloses known limit {} without a disclosure ref",
                        row.row_id,
                        row.known_limit_class.as_str()
                    ),
                ));
            }
            if row
                .downgrade_automation_class
                .requires_explicit_disclosure()
                && row.disclosure_ref.is_none()
            {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::DowngradeAutomationMissingDisclosureRef,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "row {} binds downgrade automation {} without a disclosure ref",
                        row.row_id,
                        row.downgrade_automation_class.as_str()
                    ),
                ));
            }

            if row.evidence_refs.is_empty() {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::MissingEvidenceRefs,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!("row {} carries no evidence refs", row.row_id),
                ));
            }

            if row.row_class.requires_capsule_field()
                && matches!(row.capsule_field_class, CapsuleFieldClass::NotApplicable)
            {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::CapsuleFieldNotApplicable,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "row {} is a capsule_field_admission but has no bound field",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_capsule_field()
                && !matches!(row.capsule_field_class, CapsuleFieldClass::NotApplicable)
            {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::CapsuleFieldNotPermittedOnRowClass,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds capsule field {}; only capsule_field_admission rows may bind a field",
                        row.row_id,
                        row.row_class.as_str(),
                        row.capsule_field_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_prebuild_fingerprint_component()
                && matches!(
                    row.prebuild_fingerprint_component_class,
                    PrebuildFingerprintComponentClass::NotApplicable
                )
            {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::PrebuildFingerprintNotApplicable,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "row {} is a prebuild_fingerprint_admission but has no bound component",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_prebuild_fingerprint_component()
                && !matches!(
                    row.prebuild_fingerprint_component_class,
                    PrebuildFingerprintComponentClass::NotApplicable
                )
            {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::PrebuildFingerprintNotPermittedOnRowClass,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds prebuild-fingerprint component {}; only prebuild_fingerprint_admission rows may bind a component",
                        row.row_id,
                        row.row_class.as_str(),
                        row.prebuild_fingerprint_component_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_invalidation_reason()
                && matches!(
                    row.invalidation_reason_class,
                    InvalidationReasonClass::NotApplicable
                )
            {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::InvalidationReasonNotApplicable,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "row {} is an invalidation_reason_admission but has no bound reason",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_invalidation_reason()
                && !matches!(
                    row.invalidation_reason_class,
                    InvalidationReasonClass::NotApplicable
                )
            {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::InvalidationReasonNotPermittedOnRowClass,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds invalidation reason {}; only invalidation_reason_admission rows may bind a reason",
                        row.row_id,
                        row.row_class.as_str(),
                        row.invalidation_reason_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_project_doctor_finding()
                && matches!(
                    row.project_doctor_finding_class,
                    ProjectDoctorFindingClass::NotApplicable
                )
            {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::ProjectDoctorFindingNotApplicable,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "row {} is a project_doctor_finding_admission but has no bound finding code",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_project_doctor_finding()
                && !matches!(
                    row.project_doctor_finding_class,
                    ProjectDoctorFindingClass::NotApplicable
                )
            {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::ProjectDoctorFindingNotPermittedOnRowClass,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds Project Doctor finding {}; only project_doctor_finding_admission rows may bind a finding",
                        row.row_id,
                        row.row_class.as_str(),
                        row.project_doctor_finding_class.as_str()
                    ),
                ));
            }

            if matches!(
                row.row_class,
                CapsuleResolutionRowClass::MaterializedIdentityAdmission
            ) {
                let requested_bound = row
                    .requested_artifact_identity_binding
                    .as_deref()
                    .map(str::trim)
                    .map(|value| !value.is_empty())
                    .unwrap_or(false);
                let materialized_bound = row
                    .materialized_runtime_identity_binding
                    .as_deref()
                    .map(str::trim)
                    .map(|value| !value.is_empty())
                    .unwrap_or(false);
                if !requested_bound {
                    findings.push(CapsuleResolutionValidationFinding::new(
                        CapsuleResolutionFindingKind::MaterializedIdentityAdmissionMissingRequestedArtifact,
                        CapsuleResolutionFindingSeverity::Blocker,
                        format!(
                            "row {} is a materialized_identity_admission but has no bound requested artifact id",
                            row.row_id
                        ),
                    ));
                }
                if !materialized_bound {
                    findings.push(CapsuleResolutionValidationFinding::new(
                        CapsuleResolutionFindingKind::MaterializedIdentityAdmissionMissingMaterializedRuntime,
                        CapsuleResolutionFindingSeverity::Blocker,
                        format!(
                            "row {} is a materialized_identity_admission but has no bound materialized runtime id",
                            row.row_id
                        ),
                    ));
                }
                if requested_bound
                    && materialized_bound
                    && row.requested_artifact_identity_binding == row.materialized_runtime_identity_binding
                    && !row.no_silent_prebuild_reuse
                {
                    findings.push(CapsuleResolutionValidationFinding::new(
                        CapsuleResolutionFindingKind::MaterializedIdentityAdmissionEquatesIdentitiesWithoutAttestation,
                        CapsuleResolutionFindingSeverity::Blocker,
                        format!(
                            "row {} equates requested and materialized identities without attesting no_silent_prebuild_reuse",
                            row.row_id
                        ),
                    ));
                }
                if !row.no_silent_prebuild_reuse {
                    findings.push(CapsuleResolutionValidationFinding::new(
                        CapsuleResolutionFindingKind::MaterializedIdentityAdmissionAdmitsSilentPrebuildReuse,
                        CapsuleResolutionFindingSeverity::Blocker,
                        format!(
                            "row {} is a materialized_identity_admission but does not attest no_silent_prebuild_reuse",
                            row.row_id
                        ),
                    ));
                }
            }

            if matches!(
                row.confidence_class,
                CapsuleResolutionConfidenceClass::LowConfidence
            ) && matches!(
                row.support_class,
                CapsuleResolutionSupportClass::LaunchStable
            ) {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::LaunchStableWithUnboundBinding,
                    CapsuleResolutionFindingSeverity::Warning,
                    format!(
                        "row {} claims launch_stable at low_confidence; narrowing until evidence grows",
                        row.row_id
                    ),
                ));
            }
        }

        for lane in &self.covered_lanes {
            let lane_claims_launch = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(
                        row.row_class,
                        CapsuleResolutionRowClass::CapsuleResolutionQuality
                    )
                    && matches!(
                        row.support_class,
                        CapsuleResolutionSupportClass::LaunchStable
                    )
            });
            if !lane_claims_launch {
                continue;
            }

            for field in CapsuleFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            CapsuleResolutionRowClass::CapsuleFieldAdmission
                        )
                        && row.capsule_field_class == field
                });
                if !covered {
                    findings.push(CapsuleResolutionValidationFinding::new(
                        CapsuleResolutionFindingKind::MissingCapsuleFieldCoverage,
                        CapsuleResolutionFindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no capsule_field_admission row for {}",
                            lane.as_str(),
                            field.as_str()
                        ),
                    ));
                }
            }

            for component in PrebuildFingerprintComponentClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            CapsuleResolutionRowClass::PrebuildFingerprintAdmission
                        )
                        && row.prebuild_fingerprint_component_class == component
                });
                if !covered {
                    findings.push(CapsuleResolutionValidationFinding::new(
                        CapsuleResolutionFindingKind::MissingPrebuildFingerprintCoverage,
                        CapsuleResolutionFindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no prebuild_fingerprint_admission row for {}",
                            lane.as_str(),
                            component.as_str()
                        ),
                    ));
                }
            }

            for reason in InvalidationReasonClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            CapsuleResolutionRowClass::InvalidationReasonAdmission
                        )
                        && row.invalidation_reason_class == reason
                });
                if !covered {
                    findings.push(CapsuleResolutionValidationFinding::new(
                        CapsuleResolutionFindingKind::MissingInvalidationReasonCoverage,
                        CapsuleResolutionFindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no invalidation_reason_admission row for {}",
                            lane.as_str(),
                            reason.as_str()
                        ),
                    ));
                }
            }

            for finding_code in ProjectDoctorFindingClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            CapsuleResolutionRowClass::ProjectDoctorFindingAdmission
                        )
                        && row.project_doctor_finding_class == finding_code
                });
                if !covered {
                    findings.push(CapsuleResolutionValidationFinding::new(
                        CapsuleResolutionFindingKind::MissingProjectDoctorFindingCoverage,
                        CapsuleResolutionFindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no project_doctor_finding_admission row for {}",
                            lane.as_str(),
                            finding_code.as_str()
                        ),
                    ));
                }
            }

            let has_materialized_identity = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(
                        row.row_class,
                        CapsuleResolutionRowClass::MaterializedIdentityAdmission
                    )
                    && row.no_silent_prebuild_reuse
                    && row
                        .requested_artifact_identity_binding
                        .as_deref()
                        .map(str::trim)
                        .map(|value| !value.is_empty())
                        .unwrap_or(false)
                    && row
                        .materialized_runtime_identity_binding
                        .as_deref()
                        .map(str::trim)
                        .map(|value| !value.is_empty())
                        .unwrap_or(false)
            });
            if !has_materialized_identity {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::MissingMaterializedIdentityAdmission,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no materialized_identity_admission row binding requested + materialized identity and attesting no-silent-prebuild-reuse",
                        lane.as_str()
                    ),
                ));
            }
        }

        for required_surface in CapsuleResolutionConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::MissingConsumerProjection,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::ConsumerProjectionDrift,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve capsule-resolution truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_lane_vocabulary {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::LaneVocabularyCollapsed,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the lane vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_row_class_vocabulary {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::RowClassVocabularyCollapsed,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the row-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_support_class_vocabulary {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::SupportClassVocabularyCollapsed,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the support-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_capsule_field_vocabulary {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::CapsuleFieldVocabularyCollapsed,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the capsule-field vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_prebuild_fingerprint_vocabulary {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::PrebuildFingerprintVocabularyCollapsed,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the prebuild-fingerprint vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_invalidation_reason_vocabulary {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::InvalidationReasonVocabularyCollapsed,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the invalidation-reason vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_project_doctor_finding_vocabulary {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::ProjectDoctorFindingVocabularyCollapsed,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the project-doctor-finding vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_known_limit_vocabulary {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::KnownLimitVocabularyCollapsed,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the known-limit vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_downgrade_automation_vocabulary {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::DowngradeAutomationVocabularyCollapsed,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the downgrade-automation vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_evidence_class_vocabulary {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::EvidenceClassVocabularyCollapsed,
                    CapsuleResolutionFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the evidence-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != CapsuleResolutionFindingKind::PromotionStateMismatch
            });
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(CapsuleResolutionValidationFinding::new(
                    CapsuleResolutionFindingKind::PromotionStateMismatch,
                    CapsuleResolutionFindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(
    findings: &[CapsuleResolutionValidationFinding],
) -> CapsuleResolutionPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == CapsuleResolutionFindingSeverity::Blocker)
    {
        CapsuleResolutionPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == CapsuleResolutionFindingSeverity::Warning)
    {
        CapsuleResolutionPromotionState::NarrowedBelowStable
    } else {
        CapsuleResolutionPromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleResolutionTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub capsule_resolution_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub capsule_resolution_packet: CapsuleResolutionTruthPacket,
}

impl CapsuleResolutionTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == CAPSULE_RESOLUTION_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == CAPSULE_RESOLUTION_TRUTH_SCHEMA_VERSION
            && self.capsule_resolution_packet_id_ref == self.capsule_resolution_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.capsule_resolution_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable capsule-resolution packet.
#[derive(Debug)]
pub enum CapsuleResolutionTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<CapsuleResolutionValidationFinding>),
}

impl fmt::Display for CapsuleResolutionTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "capsule-resolution packet parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "capsule-resolution packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CapsuleResolutionTruthArtifactError {}

/// Returns the checked-in stable capsule-resolution truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_capsule_resolution_truth_packet(
) -> Result<CapsuleResolutionTruthPacket, CapsuleResolutionTruthArtifactError> {
    let packet: CapsuleResolutionTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/runtime/m4/harden_environment_capsule_resolution_truth_packet.json"
    )))
    .map_err(CapsuleResolutionTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(CapsuleResolutionTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        CAPSULE_RESOLUTION_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        CAPSULE_RESOLUTION_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn quality_row(prefix: &str, lane: CapsuleResolutionLaneClass) -> CapsuleResolutionRow {
        CapsuleResolutionRow {
            row_id: format!("row:{prefix}:quality"),
            lane_class: lane,
            row_class: CapsuleResolutionRowClass::CapsuleResolutionQuality,
            support_class: CapsuleResolutionSupportClass::LaunchStable,
            capsule_field_class: CapsuleFieldClass::NotApplicable,
            prebuild_fingerprint_component_class: PrebuildFingerprintComponentClass::NotApplicable,
            invalidation_reason_class: InvalidationReasonClass::NotApplicable,
            project_doctor_finding_class: ProjectDoctorFindingClass::NotApplicable,
            evidence_class: CapsuleResolutionEvidenceClass::ReleaseEvidenceReview,
            known_limit_class: CapsuleResolutionKnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                CapsuleResolutionDowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: CapsuleResolutionConfidenceClass::HighConfidence,
            evidence_refs: vec![doc_ref(), fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_block_on_missing_evidence", doc_ref())),
            requested_artifact_identity_binding: None,
            materialized_runtime_identity_binding: None,
            no_silent_prebuild_reuse: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn capsule_field_row(
        prefix: &str,
        lane: CapsuleResolutionLaneClass,
        field: CapsuleFieldClass,
    ) -> CapsuleResolutionRow {
        CapsuleResolutionRow {
            row_id: format!("row:{prefix}:field:{}", field.as_str()),
            lane_class: lane,
            row_class: CapsuleResolutionRowClass::CapsuleFieldAdmission,
            support_class: CapsuleResolutionSupportClass::LaunchStable,
            capsule_field_class: field,
            prebuild_fingerprint_component_class: PrebuildFingerprintComponentClass::NotApplicable,
            invalidation_reason_class: InvalidationReasonClass::NotApplicable,
            project_doctor_finding_class: ProjectDoctorFindingClass::NotApplicable,
            evidence_class: CapsuleResolutionEvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: CapsuleResolutionKnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                CapsuleResolutionDowngradeAutomationClass::AutoNarrowOnCapsuleFieldGap,
            confidence_class: CapsuleResolutionConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_capsule_field_gap", doc_ref())),
            requested_artifact_identity_binding: None,
            materialized_runtime_identity_binding: None,
            no_silent_prebuild_reuse: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn fingerprint_row(
        prefix: &str,
        lane: CapsuleResolutionLaneClass,
        component: PrebuildFingerprintComponentClass,
    ) -> CapsuleResolutionRow {
        CapsuleResolutionRow {
            row_id: format!("row:{prefix}:fingerprint:{}", component.as_str()),
            lane_class: lane,
            row_class: CapsuleResolutionRowClass::PrebuildFingerprintAdmission,
            support_class: CapsuleResolutionSupportClass::LaunchStable,
            capsule_field_class: CapsuleFieldClass::NotApplicable,
            prebuild_fingerprint_component_class: component,
            invalidation_reason_class: InvalidationReasonClass::NotApplicable,
            project_doctor_finding_class: ProjectDoctorFindingClass::NotApplicable,
            evidence_class: CapsuleResolutionEvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: CapsuleResolutionKnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                CapsuleResolutionDowngradeAutomationClass::AutoNarrowOnFingerprintGap,
            confidence_class: CapsuleResolutionConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_fingerprint_gap", doc_ref())),
            requested_artifact_identity_binding: None,
            materialized_runtime_identity_binding: None,
            no_silent_prebuild_reuse: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn invalidation_row(
        prefix: &str,
        lane: CapsuleResolutionLaneClass,
        reason: InvalidationReasonClass,
    ) -> CapsuleResolutionRow {
        CapsuleResolutionRow {
            row_id: format!("row:{prefix}:invalidation:{}", reason.as_str()),
            lane_class: lane,
            row_class: CapsuleResolutionRowClass::InvalidationReasonAdmission,
            support_class: CapsuleResolutionSupportClass::LaunchStable,
            capsule_field_class: CapsuleFieldClass::NotApplicable,
            prebuild_fingerprint_component_class: PrebuildFingerprintComponentClass::NotApplicable,
            invalidation_reason_class: reason,
            project_doctor_finding_class: ProjectDoctorFindingClass::NotApplicable,
            evidence_class: CapsuleResolutionEvidenceClass::FailureRecoveryDrillEvidence,
            known_limit_class: CapsuleResolutionKnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                CapsuleResolutionDowngradeAutomationClass::AutoNarrowOnInvalidationReasonGap,
            confidence_class: CapsuleResolutionConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_invalidation_reason_gap", doc_ref())),
            requested_artifact_identity_binding: None,
            materialized_runtime_identity_binding: None,
            no_silent_prebuild_reuse: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn project_doctor_row(
        prefix: &str,
        lane: CapsuleResolutionLaneClass,
        finding: ProjectDoctorFindingClass,
    ) -> CapsuleResolutionRow {
        CapsuleResolutionRow {
            row_id: format!("row:{prefix}:doctor:{}", finding.as_str()),
            lane_class: lane,
            row_class: CapsuleResolutionRowClass::ProjectDoctorFindingAdmission,
            support_class: CapsuleResolutionSupportClass::LaunchStable,
            capsule_field_class: CapsuleFieldClass::NotApplicable,
            prebuild_fingerprint_component_class: PrebuildFingerprintComponentClass::NotApplicable,
            invalidation_reason_class: InvalidationReasonClass::NotApplicable,
            project_doctor_finding_class: finding,
            evidence_class: CapsuleResolutionEvidenceClass::FailureRecoveryDrillEvidence,
            known_limit_class: CapsuleResolutionKnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                CapsuleResolutionDowngradeAutomationClass::AutoNarrowOnProjectDoctorFindingGap,
            confidence_class: CapsuleResolutionConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_project_doctor_finding_gap",
                doc_ref()
            )),
            requested_artifact_identity_binding: None,
            materialized_runtime_identity_binding: None,
            no_silent_prebuild_reuse: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn materialized_identity_row(
        prefix: &str,
        lane: CapsuleResolutionLaneClass,
    ) -> CapsuleResolutionRow {
        CapsuleResolutionRow {
            row_id: format!("row:{prefix}:materialized_identity_admission"),
            lane_class: lane,
            row_class: CapsuleResolutionRowClass::MaterializedIdentityAdmission,
            support_class: CapsuleResolutionSupportClass::LaunchStable,
            capsule_field_class: CapsuleFieldClass::NotApplicable,
            prebuild_fingerprint_component_class: PrebuildFingerprintComponentClass::NotApplicable,
            invalidation_reason_class: InvalidationReasonClass::NotApplicable,
            project_doctor_finding_class: ProjectDoctorFindingClass::NotApplicable,
            evidence_class: CapsuleResolutionEvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: CapsuleResolutionKnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                CapsuleResolutionDowngradeAutomationClass::AutoNarrowOnMaterializedIdentityDrift,
            confidence_class: CapsuleResolutionConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_materialized_identity_drift",
                doc_ref()
            )),
            requested_artifact_identity_binding: Some(format!(
                "capsule_request:m4:{prefix}:requested"
            )),
            materialized_runtime_identity_binding: Some(format!(
                "capsule_instance:m4:{prefix}:materialized"
            )),
            no_silent_prebuild_reuse: true,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn projection(
        surface: CapsuleResolutionConsumerSurface,
    ) -> CapsuleResolutionConsumerProjection {
        CapsuleResolutionConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            capsule_resolution_packet_id_ref:
                "packet:m4:harden_environment_capsule_resolution".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_capsule_field_vocabulary: true,
            preserves_prebuild_fingerprint_vocabulary: true,
            preserves_invalidation_reason_vocabulary: true,
            preserves_project_doctor_finding_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(lane: CapsuleResolutionLaneClass, prefix: &str) -> Vec<CapsuleResolutionRow> {
        let mut out = vec![quality_row(prefix, lane)];
        for field in CapsuleFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(capsule_field_row(prefix, lane, field));
        }
        for component in PrebuildFingerprintComponentClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(fingerprint_row(prefix, lane, component));
        }
        for reason in InvalidationReasonClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(invalidation_row(prefix, lane, reason));
        }
        for finding in ProjectDoctorFindingClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(project_doctor_row(prefix, lane, finding));
        }
        out.push(materialized_identity_row(prefix, lane));
        out
    }

    fn sample_input() -> CapsuleResolutionTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(CapsuleResolutionLaneClass::DevcontainerLane, "devcontainer"));
        rows.extend(lane_rows(CapsuleResolutionLaneClass::NixLane, "nix"));
        rows.extend(lane_rows(CapsuleResolutionLaneClass::ComposeLane, "compose"));
        rows.extend(lane_rows(CapsuleResolutionLaneClass::ShellSdkLane, "shell_sdk"));
        rows.extend(lane_rows(
            CapsuleResolutionLaneClass::TemplatePrebuildLane,
            "template_prebuild",
        ));
        CapsuleResolutionTruthPacketInput {
            packet_id: "packet:m4:harden_environment_capsule_resolution".to_owned(),
            workflow_or_surface_id:
                "workflow.runtime.harden_environment_capsule_resolution".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: CapsuleResolutionLaneClass::REQUIRED.to_vec(),
            rows,
            consumer_projections: CapsuleResolutionConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(projection)
                .collect(),
            source_contract_refs: vec![doc_ref()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            CapsuleResolutionLaneClass::DevcontainerLane.as_str(),
            "devcontainer_lane"
        );
        assert_eq!(
            CapsuleResolutionLaneClass::TemplatePrebuildLane.as_str(),
            "template_prebuild_lane"
        );
        assert_eq!(
            CapsuleResolutionRowClass::CapsuleResolutionQuality.as_str(),
            "capsule_resolution_quality"
        );
        assert_eq!(
            CapsuleResolutionRowClass::MaterializedIdentityAdmission.as_str(),
            "materialized_identity_admission"
        );
        assert_eq!(
            CapsuleResolutionSupportClass::LaunchStable.as_str(),
            "launch_stable"
        );
        assert_eq!(
            CapsuleResolutionSupportClass::SupportUnbound.as_str(),
            "support_unbound"
        );
        assert_eq!(
            CapsuleFieldClass::HostOrBaseImageIdentity.as_str(),
            "host_or_base_image_identity"
        );
        assert_eq!(CapsuleFieldClass::Provenance.as_str(), "provenance");
        assert_eq!(
            PrebuildFingerprintComponentClass::CommitOrTreeIdentity.as_str(),
            "commit_or_tree_identity"
        );
        assert_eq!(
            PrebuildFingerprintComponentClass::CriticalToolchainDigest.as_str(),
            "critical_toolchain_digest"
        );
        assert_eq!(InvalidationReasonClass::ColdPath.as_str(), "cold_path");
        assert_eq!(
            InvalidationReasonClass::StalePrebuild.as_str(),
            "stale_prebuild"
        );
        assert_eq!(
            ProjectDoctorFindingClass::WrongInterpreter.as_str(),
            "wrong_interpreter"
        );
        assert_eq!(
            ProjectDoctorFindingClass::UntrustedTemplateMetadata.as_str(),
            "untrusted_template_metadata"
        );
        assert_eq!(
            CapsuleResolutionEvidenceClass::EvidenceUnbound.as_str(),
            "evidence_unbound"
        );
        assert_eq!(
            CapsuleResolutionKnownLimitClass::LimitUnbound.as_str(),
            "limit_unbound"
        );
        assert_eq!(
            CapsuleResolutionDowngradeAutomationClass::AutomationUnbound.as_str(),
            "automation_unbound"
        );
        assert_eq!(
            CapsuleResolutionConsumerSurface::ProjectDoctor.as_str(),
            "project_doctor"
        );
        assert_eq!(
            CapsuleResolutionConsumerSurface::ConformanceDashboard.as_str(),
            "conformance_dashboard"
        );
        assert_eq!(
            CapsuleResolutionPromotionState::BlocksStable.as_str(),
            "blocks_stable"
        );
        assert_eq!(
            CapsuleResolutionFindingKind::LaunchStableWithUnboundBinding.as_str(),
            "launch_stable_with_unbound_binding"
        );
        assert_eq!(
            CapsuleResolutionFindingKind::MaterializedIdentityAdmissionAdmitsSilentPrebuildReuse
                .as_str(),
            "materialized_identity_admission_admits_silent_prebuild_reuse"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = CapsuleResolutionTruthPacket::materialize(sample_input());
        assert_eq!(
            packet.promotion_state,
            CapsuleResolutionPromotionState::Stable,
            "expected stable but got findings: {:?}",
            packet
                .validation_findings
                .iter()
                .map(|f| f.finding_kind.as_str())
                .collect::<Vec<_>>()
        );
        assert!(packet.validation_findings.is_empty());
        assert!(packet.is_stable());
        assert!(packet
            .support_export(
                "support:m4:harden_environment_capsule_resolution",
                "2026-05-26T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = CapsuleResolutionEvidenceClass::EvidenceUnbound;
        let packet = CapsuleResolutionTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            CapsuleResolutionPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| finding
            .finding_kind
            == CapsuleResolutionFindingKind::MissingEvidenceClass));
        assert!(packet.validation_findings.iter().any(|finding| finding
            .finding_kind
            == CapsuleResolutionFindingKind::LaunchStableWithUnboundBinding));
    }

    #[test]
    fn missing_prebuild_fingerprint_component_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                CapsuleResolutionRowClass::PrebuildFingerprintAdmission
            ) && row.prebuild_fingerprint_component_class
                == PrebuildFingerprintComponentClass::CriticalToolchainDigest
                && row.lane_class == CapsuleResolutionLaneClass::DevcontainerLane)
        });
        let packet = CapsuleResolutionTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            CapsuleResolutionPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| finding
            .finding_kind
            == CapsuleResolutionFindingKind::MissingPrebuildFingerprintCoverage));
    }

    #[test]
    fn materialized_identity_admission_without_attestation_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(
                row.row_class,
                CapsuleResolutionRowClass::MaterializedIdentityAdmission
            ) && row.lane_class == CapsuleResolutionLaneClass::DevcontainerLane
            {
                row.no_silent_prebuild_reuse = false;
                break;
            }
        }
        let packet = CapsuleResolutionTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            CapsuleResolutionPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| finding
            .finding_kind
            == CapsuleResolutionFindingKind::MaterializedIdentityAdmissionAdmitsSilentPrebuildReuse));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = CapsuleResolutionSupportClass::LaunchStableBelow;
        input.rows[0].disclosure_ref = None;
        let packet = CapsuleResolutionTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            CapsuleResolutionPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| finding
            .finding_kind
            == CapsuleResolutionFindingKind::NarrowedRowMissingDisclosureRef));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input.consumer_projections.retain(|projection| {
            projection.consumer_surface != CapsuleResolutionConsumerSurface::ProjectDoctor
        });
        let packet = CapsuleResolutionTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            CapsuleResolutionPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| finding
            .finding_kind
            == CapsuleResolutionFindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_invalidation_reason_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == CapsuleResolutionConsumerSurface::ProjectDoctor {
                projection.preserves_invalidation_reason_vocabulary = false;
            }
        }
        let packet = CapsuleResolutionTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            CapsuleResolutionPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| finding
            .finding_kind
            == CapsuleResolutionFindingKind::InvalidationReasonVocabularyCollapsed));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = CapsuleResolutionTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            CapsuleResolutionPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| finding
            .finding_kind
            == CapsuleResolutionFindingKind::RawSourceMaterialPresent));
    }
}
