//! Task-discovery, launch-profile, rerun-last, and task-event truth
//! stabilization packet for the M4 stable lane.
//!
//! This module pins how the local, remote/helper, notebook, and
//! imported-provider runs serialize into one canonical task-event
//! vocabulary across the four launch wedges (`task_discovery`,
//! `launch_profile`, `rerun_last`, and `task_event`). Surfaces MUST
//! NOT mint local copies, fork their own task-event semantics, or
//! flatten additive detail into display text; downstream Problems,
//! output-channel, evidence-export, and rerun consumers read the
//! packet verbatim.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `task_event_truth_quality` row cannot prove:
//!
//! - the four launch wedges (`task_discovery`, `launch_profile`,
//!   `rerun_last`, `task_event`) each have a structured
//!   wedge_admission row,
//! - canonical task-event envelopes bind the six envelope fields
//!   (`event_id`, `execution_context_ref`, `adapter_identity`,
//!   `provider_identity`, `confidence_flag`, `fallback_flag`) so
//!   later Problems, output-channel, evidence-export, and rerun
//!   surfaces do not invent a second truth model,
//! - the four downstream surfaces (`problems`, `output_channel`,
//!   `evidence_export`, `rerun_surface`) carry surface-binding rows
//!   that read this packet verbatim,
//! - additive detail (structured payload, diagnostics, adapter
//!   metadata) is preserved instead of flattened into display text,
//! - one stable `execution_context_id` (or equivalent lineage
//!   object) threads through every emitted task-event envelope and
//!   downstream consumer surface.
//!
//! Every row binds a closed `task_event_truth_lane_class`,
//! `task_event_truth_row_class`, `support_class`, `wedge_class`,
//! `envelope_field_class`, `downstream_surface_class`,
//! `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and
//! `task_event_truth_confidence_class` plus an `evidence_refs`
//! array and a `disclosure_ref` whenever the row is narrowed below
//! launch-stable, declares a non-`none_declared` known limit, or
//! binds a non-`none` downgrade automation.
//!
//! The packet is metadata-only — it never admits raw command lines,
//! raw process environment bytes, raw secrets, raw capsule bodies,
//! or ambient credentials past the boundary. A row that claims
//! `launch_stable` while leaving its known limit, downgrade
//! automation, or evidence class unbound is refused; the validator
//! narrows below launch-stable instead of inheriting an adjacent
//! certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`TaskEventTruthPacket`].
pub const TASK_EVENT_TRUTH_PACKET_RECORD_KIND: &str =
    "stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_stable_packet";

/// Stable record-kind tag for [`TaskEventTruthSupportExport`].
pub const TASK_EVENT_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_support_export";

/// Integer schema version for the task-event truth packet.
pub const TASK_EVENT_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const TASK_EVENT_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const TASK_EVENT_TRUTH_DOC_REF: &str =
    "docs/runtime/m4/stabilize-task-discovery-launch-profiles-rerun-last-behavior.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const TASK_EVENT_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/stabilize-task-discovery-launch-profiles-rerun-last-behavior.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const TASK_EVENT_TRUTH_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/stabilize_task_discovery_launch_profiles_rerun_last_behavior";

/// Repo-relative path of the checked-in stable packet.
pub const TASK_EVENT_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.json";

/// Closed task-event lane vocabulary. Every required lane MUST have
/// at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskEventTruthLaneClass {
    /// Local-host run lane.
    LocalLane,
    /// Remote / helper attach run lane.
    RemoteHelperLane,
    /// Notebook (kernel) run lane.
    NotebookLane,
    /// Imported-provider run lane (CI mirror, imported task-runner).
    ImportedProviderLane,
}

impl TaskEventTruthLaneClass {
    /// Every required task-event lane, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::LocalLane,
        Self::RemoteHelperLane,
        Self::NotebookLane,
        Self::ImportedProviderLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalLane => "local_lane",
            Self::RemoteHelperLane => "remote_helper_lane",
            Self::NotebookLane => "notebook_lane",
            Self::ImportedProviderLane => "imported_provider_lane",
        }
    }
}

/// Closed task-event row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskEventTruthRowClass {
    /// The lane's headline task-event truth qualification row.
    TaskEventTruthQuality,
    /// A row admitting one of the four launch wedges (task_discovery,
    /// launch_profile, rerun_last, task_event).
    WedgeAdmission,
    /// A row binding one canonical envelope field (event_id,
    /// execution_context_ref, adapter_identity, provider_identity,
    /// confidence_flag, fallback_flag) emitted by the task-event
    /// stream.
    EnvelopeFieldBinding,
    /// A row binding one downstream consumer surface (problems,
    /// output_channel, evidence_export, rerun_surface) to the shared
    /// task-event truth.
    SurfaceBinding,
    /// A row attesting that additive task-event detail is preserved
    /// rather than flattened into display text.
    AdditiveDetailPreservation,
    /// A row binding the stable `execution_context_id` (or equivalent
    /// lineage object) into emitted task-event envelopes and
    /// downstream consumer surfaces.
    LineageAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl TaskEventTruthRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskEventTruthQuality => "task_event_truth_quality",
            Self::WedgeAdmission => "wedge_admission",
            Self::EnvelopeFieldBinding => "envelope_field_binding",
            Self::SurfaceBinding => "surface_binding",
            Self::AdditiveDetailPreservation => "additive_detail_preservation",
            Self::LineageAdmission => "lineage_admission",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when this row class requires a bound wedge.
    pub const fn requires_wedge(self) -> bool {
        matches!(self, Self::WedgeAdmission)
    }

    /// True when this row class requires a bound envelope field.
    pub const fn requires_envelope_field(self) -> bool {
        matches!(self, Self::EnvelopeFieldBinding)
    }

    /// True when this row class requires a bound downstream surface.
    pub const fn requires_downstream_surface(self) -> bool {
        matches!(self, Self::SurfaceBinding)
    }
}

/// Closed support-class vocabulary applied to a task-event row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Row claims M4 launch-stable grade for the lane.
    LaunchStable,
    /// Row is intentionally narrowed below launch-stable.
    LaunchStableBelow,
    /// Row is at beta-grade only.
    BetaGradeOnly,
    /// Row is at preview only.
    PreviewOnly,
    /// Row carries a precisely labeled unsupported gap.
    Unsupported,
    /// Row has no bound support class; this never qualifies stable.
    SupportUnbound,
}

impl SupportClass {
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

/// Closed launch-wedge vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `wedge_admission` row for each
/// required wedge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WedgeClass {
    /// The row admits the task_discovery wedge (package scripts,
    /// pytest, framework-specific discovery).
    TaskDiscovery,
    /// The row admits the launch_profile wedge (run/debug profiles
    /// and their stored bindings).
    LaunchProfile,
    /// The row admits the rerun_last wedge (rerun-last-task,
    /// rerun-last-test bindings).
    RerunLast,
    /// The row admits the task_event wedge (canonical task-event
    /// envelopes emitted across the lane).
    TaskEvent,
    /// The row is not bound to a wedge.
    NotApplicable,
}

impl WedgeClass {
    /// Every required wedge for a `launch_stable` lane, in
    /// declaration order.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::TaskDiscovery,
        Self::LaunchProfile,
        Self::RerunLast,
        Self::TaskEvent,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskDiscovery => "task_discovery",
            Self::LaunchProfile => "launch_profile",
            Self::RerunLast => "rerun_last",
            Self::TaskEvent => "task_event",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed envelope-field vocabulary. Every lane claiming
/// `launch_stable` MUST publish an `envelope_field_binding` row for
/// each required field so downstream surfaces never invent a second
/// truth model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvelopeFieldClass {
    /// Stable task-event id.
    EventId,
    /// Execution-context reference.
    ExecutionContextRef,
    /// Adapter identity (toolchain / runner identity).
    AdapterIdentity,
    /// Provider identity (task / launch-profile / framework provider).
    ProviderIdentity,
    /// Confidence flag (high/medium/low confidence on the envelope).
    ConfidenceFlag,
    /// Fallback flag (the envelope is a structured fallback rather
    /// than a primary capture).
    FallbackFlag,
    /// The row is not bound to an envelope field.
    NotApplicable,
}

impl EnvelopeFieldClass {
    /// Every required envelope field for a `launch_stable` lane, in
    /// declaration order.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::EventId,
        Self::ExecutionContextRef,
        Self::AdapterIdentity,
        Self::ProviderIdentity,
        Self::ConfidenceFlag,
        Self::FallbackFlag,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EventId => "event_id",
            Self::ExecutionContextRef => "execution_context_ref",
            Self::AdapterIdentity => "adapter_identity",
            Self::ProviderIdentity => "provider_identity",
            Self::ConfidenceFlag => "confidence_flag",
            Self::FallbackFlag => "fallback_flag",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed downstream-surface vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `surface_binding` row for each
/// downstream consumer surface so Problems, output-channel,
/// evidence-export, and rerun consumers all read the same packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DownstreamSurfaceClass {
    /// Problems panel / diagnostics surface.
    Problems,
    /// Output channel (task / test / debug output streams).
    OutputChannel,
    /// Evidence export bundle surface.
    EvidenceExport,
    /// Rerun-last / rerun-prepared-attempt surface.
    RerunSurface,
    /// The row is not bound to a downstream surface.
    NotApplicable,
}

impl DownstreamSurfaceClass {
    /// Every required downstream surface for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::Problems,
        Self::OutputChannel,
        Self::EvidenceExport,
        Self::RerunSurface,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Problems => "problems",
            Self::OutputChannel => "output_channel",
            Self::EvidenceExport => "evidence_export",
            Self::RerunSurface => "rerun_surface",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed evidence-class vocabulary describing what backs a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
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

impl EvidenceClass {
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

/// Closed known-limit vocabulary attached to a task-event row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The lane only certifies the local subset.
    LocalLaneSubsetOnly,
    /// The lane only certifies the remote/helper subset.
    RemoteHelperSubsetOnly,
    /// The lane only certifies the notebook subset.
    NotebookSubsetOnly,
    /// The lane only certifies the imported-provider subset.
    ImportedProviderSubsetOnly,
    /// The lane only certifies a subset of the four required wedges.
    WedgeAdmissionSubsetOnly,
    /// The lane only certifies a subset of the six required envelope fields.
    EnvelopeFieldSubsetOnly,
    /// The lane only certifies a subset of the four downstream surfaces.
    DownstreamSurfaceSubsetOnly,
    /// The lane drops additive detail (flattens into display text).
    AdditiveDetailDropped,
    /// The lane is at beta-grade-only capability sample.
    BetaCapabilitySampleOnly,
    /// The row has no bound known limit class; this never qualifies stable.
    LimitUnbound,
}

impl KnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::LocalLaneSubsetOnly => "local_lane_subset_only",
            Self::RemoteHelperSubsetOnly => "remote_helper_subset_only",
            Self::NotebookSubsetOnly => "notebook_subset_only",
            Self::ImportedProviderSubsetOnly => "imported_provider_subset_only",
            Self::WedgeAdmissionSubsetOnly => "wedge_admission_subset_only",
            Self::EnvelopeFieldSubsetOnly => "envelope_field_subset_only",
            Self::DownstreamSurfaceSubsetOnly => "downstream_surface_subset_only",
            Self::AdditiveDetailDropped => "additive_detail_dropped",
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

/// Closed downgrade-automation vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when a required wedge admission is missing.
    AutoNarrowOnWedgeAdmissionGap,
    /// Automatically narrow when a required envelope field is unbound.
    AutoNarrowOnEnvelopeFieldGap,
    /// Automatically narrow when a downstream surface binding is missing.
    AutoNarrowOnDownstreamSurfaceGap,
    /// Automatically narrow when additive detail is flattened into display text.
    AutoNarrowOnAdditiveDetailDropped,
    /// Automatically narrow when the lineage object breaks
    /// (`execution_context_id` does not thread through envelopes or
    /// downstream surfaces).
    AutoNarrowOnLineageBreak,
    /// Automatically narrow when adapter/provider identity is missing
    /// or unstable.
    AutoNarrowOnAdapterIdentityDrift,
    /// Automatically narrow when confidence / fallback flags are absent.
    AutoNarrowOnConfidenceFlagAbsent,
    /// Automatically block when required evidence is missing.
    AutoBlockOnMissingEvidence,
    /// Manual-only review required until automation lands.
    ManualOnlyPendingReview,
    /// Automation is unbound; this never qualifies stable.
    AutomationUnbound,
}

impl DowngradeAutomationClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AutoNarrowOnWedgeAdmissionGap => "auto_narrow_on_wedge_admission_gap",
            Self::AutoNarrowOnEnvelopeFieldGap => "auto_narrow_on_envelope_field_gap",
            Self::AutoNarrowOnDownstreamSurfaceGap => "auto_narrow_on_downstream_surface_gap",
            Self::AutoNarrowOnAdditiveDetailDropped => "auto_narrow_on_additive_detail_dropped",
            Self::AutoNarrowOnLineageBreak => "auto_narrow_on_lineage_break",
            Self::AutoNarrowOnAdapterIdentityDrift => "auto_narrow_on_adapter_identity_drift",
            Self::AutoNarrowOnConfidenceFlagAbsent => "auto_narrow_on_confidence_flag_absent",
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

/// Closed confidence-class vocabulary for a task-event row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskEventTruthConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until evidence grows.
    LowConfidence,
}

impl TaskEventTruthConfidenceClass {
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
pub enum PromotionState {
    /// Packet certifies a stable claim.
    Stable,
    /// Packet narrows below stable.
    NarrowedBelowStable,
    /// Packet has a blocker finding.
    BlocksStable,
}

impl PromotionState {
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
pub enum FindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker finding that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required lane has no row.
    MissingLaneCoverage,
    /// A lane claiming launch_stable is missing a required wedge admission.
    MissingWedgeAdmissionCoverage,
    /// A lane claiming launch_stable is missing a required envelope field binding.
    MissingEnvelopeFieldCoverage,
    /// A lane claiming launch_stable is missing a required downstream surface binding.
    MissingDownstreamSurfaceCoverage,
    /// A lane claiming launch_stable is missing the additive-detail row.
    MissingAdditiveDetailPreservation,
    /// A lane claiming launch_stable is missing the required lineage admission row.
    MissingLineageAdmission,
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
    /// A row with a non-`none_declared` known limit drops its disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// A wedge-admission row drops its wedge binding.
    WedgeNotApplicable,
    /// A non-wedge row binds a wedge it cannot certify.
    WedgeNotPermittedOnRowClass,
    /// An envelope-field row drops its field binding.
    EnvelopeFieldNotApplicable,
    /// A non-envelope-field row binds an envelope field it cannot certify.
    EnvelopeFieldNotPermittedOnRowClass,
    /// A downstream-surface row drops its surface binding.
    DownstreamSurfaceNotApplicable,
    /// A non-surface-binding row binds a downstream surface it cannot certify.
    DownstreamSurfaceNotPermittedOnRowClass,
    /// A lineage-admission row does not bind a lineage object id.
    LineageAdmissionMissingExecutionContextId,
    /// An additive-detail row admits flattening into display text.
    AdditiveDetailRowAdmitsFlattening,
    /// A row admits raw command lines, env bytes, or other private material.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the wedge vocabulary.
    WedgeVocabularyCollapsed,
    /// A projection collapses the envelope-field vocabulary.
    EnvelopeFieldVocabularyCollapsed,
    /// A projection collapses the downstream-surface vocabulary.
    DownstreamSurfaceVocabularyCollapsed,
    /// A projection collapses the known-limit vocabulary.
    KnownLimitVocabularyCollapsed,
    /// A projection collapses the downgrade-automation vocabulary.
    DowngradeAutomationVocabularyCollapsed,
    /// A projection collapses the evidence-class vocabulary.
    EvidenceClassVocabularyCollapsed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingLaneCoverage => "missing_lane_coverage",
            Self::MissingWedgeAdmissionCoverage => "missing_wedge_admission_coverage",
            Self::MissingEnvelopeFieldCoverage => "missing_envelope_field_coverage",
            Self::MissingDownstreamSurfaceCoverage => "missing_downstream_surface_coverage",
            Self::MissingAdditiveDetailPreservation => "missing_additive_detail_preservation",
            Self::MissingLineageAdmission => "missing_lineage_admission",
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
            Self::WedgeNotApplicable => "wedge_not_applicable",
            Self::WedgeNotPermittedOnRowClass => "wedge_not_permitted_on_row_class",
            Self::EnvelopeFieldNotApplicable => "envelope_field_not_applicable",
            Self::EnvelopeFieldNotPermittedOnRowClass => {
                "envelope_field_not_permitted_on_row_class"
            }
            Self::DownstreamSurfaceNotApplicable => "downstream_surface_not_applicable",
            Self::DownstreamSurfaceNotPermittedOnRowClass => {
                "downstream_surface_not_permitted_on_row_class"
            }
            Self::LineageAdmissionMissingExecutionContextId => {
                "lineage_admission_missing_execution_context_id"
            }
            Self::AdditiveDetailRowAdmitsFlattening => "additive_detail_row_admits_flattening",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::WedgeVocabularyCollapsed => "wedge_vocabulary_collapsed",
            Self::EnvelopeFieldVocabularyCollapsed => "envelope_field_vocabulary_collapsed",
            Self::DownstreamSurfaceVocabularyCollapsed => "downstream_surface_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Editor run / launch surface (per-pane "why this target?" chip).
    EditorRunSurface,
    /// Task panel chrome and per-run header.
    TaskPanel,
    /// Problems / diagnostics panel.
    ProblemsPanel,
    /// Output channel (task/test/debug output streams).
    OutputChannel,
    /// Evidence export bundle.
    EvidenceExport,
    /// Rerun-last / rerun-prepared-attempt surface.
    RerunSurface,
    /// CLI / headless inspection surface (`aureline tasks ...`).
    CliHeadless,
    /// Support export bundle surface.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
    /// Help/About proof card surface.
    HelpAbout,
    /// Conformance dashboard surface.
    ConformanceDashboard,
}

impl ConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 11] = [
        Self::EditorRunSurface,
        Self::TaskPanel,
        Self::ProblemsPanel,
        Self::OutputChannel,
        Self::EvidenceExport,
        Self::RerunSurface,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorRunSurface => "editor_run_surface",
            Self::TaskPanel => "task_panel",
            Self::ProblemsPanel => "problems_panel",
            Self::OutputChannel => "output_channel",
            Self::EvidenceExport => "evidence_export",
            Self::RerunSurface => "rerun_surface",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
            Self::HelpAbout => "help_about",
            Self::ConformanceDashboard => "conformance_dashboard",
        }
    }
}

/// One validation finding emitted by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Closed finding kind.
    pub finding_kind: FindingKind,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl ValidationFinding {
    fn new(
        finding_kind: FindingKind,
        severity: FindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// One task-event truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventTruthRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Task-event lane this row certifies.
    pub lane_class: TaskEventTruthLaneClass,
    /// Row class.
    pub row_class: TaskEventTruthRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Wedge bound by the row (or `not_applicable`).
    pub wedge_class: WedgeClass,
    /// Envelope field bound by the row (or `not_applicable`).
    pub envelope_field_class: EnvelopeFieldClass,
    /// Downstream surface bound by the row (or `not_applicable`).
    pub downstream_surface_class: DownstreamSurfaceClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: TaskEventTruthConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `launch_stable`, declares a non-`none_declared` known limit,
    /// or binds a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// For lineage_admission rows, the bound `execution_context_id`
    /// token (or equivalent lineage object reference). Required when
    /// `row_class == LineageAdmission`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id_binding: Option<String>,
    /// For additive_detail_preservation rows, true when the row
    /// attests that additive task-event detail is preserved instead
    /// of flattened into display text.
    #[serde(default)]
    pub additive_detail_preserved: bool,
    /// True when raw command lines, raw process environment bytes,
    /// or raw capsule bodies are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl TaskEventTruthRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventTruthConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Task-event packet id consumed by the projection.
    pub task_event_truth_packet_id_ref: String,
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
    /// True when the wedge vocabulary is preserved verbatim.
    pub preserves_wedge_vocabulary: bool,
    /// True when the envelope-field vocabulary is preserved verbatim.
    pub preserves_envelope_field_vocabulary: bool,
    /// True when the downstream-surface vocabulary is preserved verbatim.
    pub preserves_downstream_surface_vocabulary: bool,
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

impl TaskEventTruthConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.task_event_truth_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_wedge_vocabulary
            && self.preserves_envelope_field_vocabulary
            && self.preserves_downstream_surface_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`TaskEventTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Task-event lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<TaskEventTruthLaneClass>,
    /// Task-event rows.
    #[serde(default)]
    pub rows: Vec<TaskEventTruthRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<TaskEventTruthConsumerProjection>,
    /// Source contracts consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Runtime-owned packet certifying local, remote/helper, notebook,
/// and imported-provider task-event truth at the M4 launch-stable
/// grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventTruthPacket {
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
    /// Task-event lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<TaskEventTruthLaneClass>,
    /// Task-event rows.
    #[serde(default)]
    pub rows: Vec<TaskEventTruthRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<TaskEventTruthConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl TaskEventTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: TaskEventTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: TASK_EVENT_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: TASK_EVENT_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_lanes: input.covered_lanes,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: PromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable task-event invariants.
    pub fn validate(&self) -> Vec<ValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: ConsumerSurface) -> bool {
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
            .map(TaskEventTruthLaneClass::as_str)
            .collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(TaskEventTruthRowClass::as_str)
            .collect()
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.support_class);
        }
        set.into_iter().map(SupportClass::as_str).collect()
    }

    /// Returns the unique wedge tokens observed across rows.
    pub fn wedge_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.wedge_class);
        }
        set.into_iter().map(WedgeClass::as_str).collect()
    }

    /// Returns the unique envelope-field tokens observed across rows.
    pub fn envelope_field_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.envelope_field_class);
        }
        set.into_iter().map(EnvelopeFieldClass::as_str).collect()
    }

    /// Returns the unique downstream-surface tokens observed across rows.
    pub fn downstream_surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downstream_surface_class);
        }
        set.into_iter()
            .map(DownstreamSurfaceClass::as_str)
            .collect()
    }

    /// Returns the unique evidence-class tokens observed across rows.
    pub fn evidence_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.evidence_class);
        }
        set.into_iter().map(EvidenceClass::as_str).collect()
    }

    /// Returns the unique known-limit tokens observed across rows.
    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.known_limit_class);
        }
        set.into_iter().map(KnownLimitClass::as_str).collect()
    }

    /// Returns the unique downgrade-automation tokens observed across rows.
    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_automation_class);
        }
        set.into_iter()
            .map(DowngradeAutomationClass::as_str)
            .collect()
    }

    /// Builds a support export wrapping the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> TaskEventTruthSupportExport {
        TaskEventTruthSupportExport {
            record_kind: TASK_EVENT_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TASK_EVENT_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            task_event_truth_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            task_event_truth_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != TASK_EVENT_TRUTH_PACKET_RECORD_KIND {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "task-event truth packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != TASK_EVENT_TRUTH_SCHEMA_VERSION {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "task-event truth packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }
        if self.covered_lanes.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered task-event lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers task-event lane {}", lane.as_str()),
                ));
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingIdentity,
                    FindingSeverity::Blocker,
                    format!("row {} identity or timestamp is empty", row.row_id),
                ));
            }
            if !row.raw_source_material_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::RawSourceMaterialPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits raw command lines, raw env bytes, or raw capsule bodies past the boundary",
                        row.row_id
                    ),
                ));
            }
            if !row.secrets_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::SecretsPresent,
                    FindingSeverity::Blocker,
                    format!("row {} admits secrets past the boundary", row.row_id),
                ));
            }
            if !row.ambient_authority_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::AmbientAuthorityPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits ambient authority/credentials past the boundary",
                        row.row_id
                    ),
                ));
            }

            if !row.support_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingSupportClass,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound support class", row.row_id),
                ));
            }
            if !row.known_limit_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingKnownLimit,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound known-limit class", row.row_id),
                ));
            }
            if !row.downgrade_automation_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingDowngradeAutomation,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound downgrade-automation class", row.row_id),
                ));
            }
            if !row.evidence_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceClass,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound evidence class", row.row_id),
                ));
            }

            if matches!(row.support_class, SupportClass::LaunchStable)
                && !row.all_bindings_satisfied()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LaunchStableWithUnboundBinding,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} claims launch_stable while a binding (support, known limit, downgrade automation, or evidence) is unbound",
                        row.row_id
                    ),
                ));
            }

            if row.support_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
                findings.push(ValidationFinding::new(
                    FindingKind::NarrowedRowMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has support class {} without a disclosure ref",
                        row.row_id,
                        row.support_class.as_str()
                    ),
                ));
            }
            if row.known_limit_class.requires_explicit_disclosure() && row.disclosure_ref.is_none()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::KnownLimitMissingDisclosureRef,
                    FindingSeverity::Blocker,
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
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeAutomationMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds downgrade automation {} without a disclosure ref",
                        row.row_id,
                        row.downgrade_automation_class.as_str()
                    ),
                ));
            }

            if row.evidence_refs.is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceRefs,
                    FindingSeverity::Blocker,
                    format!("row {} carries no evidence refs", row.row_id),
                ));
            }

            if row.row_class.requires_wedge()
                && matches!(row.wedge_class, WedgeClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::WedgeNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a wedge_admission but has no bound wedge",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_wedge()
                && !matches!(row.wedge_class, WedgeClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::WedgeNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds wedge {}; only wedge_admission rows may bind a wedge",
                        row.row_id,
                        row.row_class.as_str(),
                        row.wedge_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_envelope_field()
                && matches!(row.envelope_field_class, EnvelopeFieldClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::EnvelopeFieldNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is an envelope_field_binding but has no bound envelope field",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_envelope_field()
                && !matches!(row.envelope_field_class, EnvelopeFieldClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::EnvelopeFieldNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds envelope field {}; only envelope_field_binding rows may bind a field",
                        row.row_id,
                        row.row_class.as_str(),
                        row.envelope_field_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_downstream_surface()
                && matches!(
                    row.downstream_surface_class,
                    DownstreamSurfaceClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DownstreamSurfaceNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a surface_binding but has no bound downstream surface",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_downstream_surface()
                && !matches!(
                    row.downstream_surface_class,
                    DownstreamSurfaceClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DownstreamSurfaceNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds downstream surface {}; only surface_binding rows may bind a surface",
                        row.row_id,
                        row.row_class.as_str(),
                        row.downstream_surface_class.as_str()
                    ),
                ));
            }

            if matches!(row.row_class, TaskEventTruthRowClass::LineageAdmission)
                && row
                    .execution_context_id_binding
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    .unwrap_or(true)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LineageAdmissionMissingExecutionContextId,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a lineage_admission but has no bound execution_context_id",
                        row.row_id
                    ),
                ));
            }

            if matches!(
                row.row_class,
                TaskEventTruthRowClass::AdditiveDetailPreservation
            ) && !row.additive_detail_preserved
            {
                findings.push(ValidationFinding::new(
                    FindingKind::AdditiveDetailRowAdmitsFlattening,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is an additive_detail_preservation row but admits flattening into display text",
                        row.row_id
                    ),
                ));
            }

            if matches!(
                row.confidence_class,
                TaskEventTruthConfidenceClass::LowConfidence
            ) && matches!(row.support_class, SupportClass::LaunchStable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LaunchStableWithUnboundBinding,
                    FindingSeverity::Warning,
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
                    && matches!(row.row_class, TaskEventTruthRowClass::TaskEventTruthQuality)
                    && matches!(row.support_class, SupportClass::LaunchStable)
            });
            if !lane_claims_launch {
                continue;
            }

            for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, TaskEventTruthRowClass::WedgeAdmission)
                        && row.wedge_class == wedge
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingWedgeAdmissionCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no wedge_admission row for {}",
                            lane.as_str(),
                            wedge.as_str()
                        ),
                    ));
                }
            }

            for field in EnvelopeFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, TaskEventTruthRowClass::EnvelopeFieldBinding)
                        && row.envelope_field_class == field
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingEnvelopeFieldCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no envelope_field_binding row for {}",
                            lane.as_str(),
                            field.as_str()
                        ),
                    ));
                }
            }

            for surface in DownstreamSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, TaskEventTruthRowClass::SurfaceBinding)
                        && row.downstream_surface_class == surface
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingDownstreamSurfaceCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no surface_binding row for {}",
                            lane.as_str(),
                            surface.as_str()
                        ),
                    ));
                }
            }

            let has_additive_detail = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(
                        row.row_class,
                        TaskEventTruthRowClass::AdditiveDetailPreservation
                    )
                    && row.additive_detail_preserved
            });
            if !has_additive_detail {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingAdditiveDetailPreservation,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no additive_detail_preservation row attesting preserved detail",
                        lane.as_str()
                    ),
                ));
            }

            let has_lineage = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, TaskEventTruthRowClass::LineageAdmission)
                    && row
                        .execution_context_id_binding
                        .as_deref()
                        .map(str::trim)
                        .map(|value| !value.is_empty())
                        .unwrap_or(false)
            });
            if !has_lineage {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLineageAdmission,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no lineage_admission row binding execution_context_id",
                        lane.as_str()
                    ),
                ));
            }
        }

        for required_surface in ConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingConsumerProjection,
                    FindingSeverity::Blocker,
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
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerProjectionDrift,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve task-event truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_lane_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::LaneVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the lane vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_row_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RowClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the row-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_support_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::SupportClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the support-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_wedge_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::WedgeVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the wedge vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_envelope_field_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::EnvelopeFieldVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the envelope-field vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_downstream_surface_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DownstreamSurfaceVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the downstream-surface vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_known_limit_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::KnownLimitVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the known-limit vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_downgrade_automation_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeAutomationVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the downgrade-automation vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_evidence_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::EvidenceClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the evidence-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion
                .retain(|finding| finding.finding_kind != FindingKind::PromotionStateMismatch);
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(ValidationFinding::new(
                    FindingKind::PromotionStateMismatch,
                    FindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(findings: &[ValidationFinding]) -> PromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Blocker)
    {
        PromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Warning)
    {
        PromotionState::NarrowedBelowStable
    } else {
        PromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub task_event_truth_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub task_event_truth_packet: TaskEventTruthPacket,
}

impl TaskEventTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == TASK_EVENT_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == TASK_EVENT_TRUTH_SCHEMA_VERSION
            && self.task_event_truth_packet_id_ref == self.task_event_truth_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.task_event_truth_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum TaskEventTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for TaskEventTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "task-event truth packet parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "task-event truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for TaskEventTruthArtifactError {}

/// Returns the checked-in stable task-event truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_task_event_truth_packet(
) -> Result<TaskEventTruthPacket, TaskEventTruthArtifactError> {
    let packet: TaskEventTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/runtime/m4/stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.json"
    )))
    .map_err(TaskEventTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(TaskEventTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        TASK_EVENT_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        TASK_EVENT_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn quality_row(prefix: &str, lane: TaskEventTruthLaneClass) -> TaskEventTruthRow {
        TaskEventTruthRow {
            row_id: format!("row:{prefix}:quality"),
            lane_class: lane,
            row_class: TaskEventTruthRowClass::TaskEventTruthQuality,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            envelope_field_class: EnvelopeFieldClass::NotApplicable,
            downstream_surface_class: DownstreamSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ReleaseEvidenceReview,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: TaskEventTruthConfidenceClass::HighConfidence,
            evidence_refs: vec![doc_ref(), fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_block_on_missing_evidence", doc_ref())),
            execution_context_id_binding: None,
            additive_detail_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn wedge_row(
        prefix: &str,
        lane: TaskEventTruthLaneClass,
        wedge: WedgeClass,
    ) -> TaskEventTruthRow {
        TaskEventTruthRow {
            row_id: format!("row:{prefix}:wedge:{}", wedge.as_str()),
            lane_class: lane,
            row_class: TaskEventTruthRowClass::WedgeAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: wedge,
            envelope_field_class: EnvelopeFieldClass::NotApplicable,
            downstream_surface_class: DownstreamSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnWedgeAdmissionGap,
            confidence_class: TaskEventTruthConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_wedge_admission_gap", doc_ref())),
            execution_context_id_binding: None,
            additive_detail_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn envelope_field_row(
        prefix: &str,
        lane: TaskEventTruthLaneClass,
        field: EnvelopeFieldClass,
    ) -> TaskEventTruthRow {
        TaskEventTruthRow {
            row_id: format!("row:{prefix}:field:{}", field.as_str()),
            lane_class: lane,
            row_class: TaskEventTruthRowClass::EnvelopeFieldBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            envelope_field_class: field,
            downstream_surface_class: DownstreamSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnEnvelopeFieldGap,
            confidence_class: TaskEventTruthConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_envelope_field_gap", doc_ref())),
            execution_context_id_binding: None,
            additive_detail_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn surface_row(
        prefix: &str,
        lane: TaskEventTruthLaneClass,
        surface: DownstreamSurfaceClass,
    ) -> TaskEventTruthRow {
        TaskEventTruthRow {
            row_id: format!("row:{prefix}:surface:{}", surface.as_str()),
            lane_class: lane,
            row_class: TaskEventTruthRowClass::SurfaceBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            envelope_field_class: EnvelopeFieldClass::NotApplicable,
            downstream_surface_class: surface,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnDownstreamSurfaceGap,
            confidence_class: TaskEventTruthConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_downstream_surface_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            additive_detail_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn additive_detail_row(prefix: &str, lane: TaskEventTruthLaneClass) -> TaskEventTruthRow {
        TaskEventTruthRow {
            row_id: format!("row:{prefix}:additive_detail"),
            lane_class: lane,
            row_class: TaskEventTruthRowClass::AdditiveDetailPreservation,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            envelope_field_class: EnvelopeFieldClass::NotApplicable,
            downstream_surface_class: DownstreamSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::FailureRecoveryDrillEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnAdditiveDetailDropped,
            confidence_class: TaskEventTruthConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_additive_detail_dropped",
                doc_ref()
            )),
            execution_context_id_binding: None,
            additive_detail_preserved: true,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn lineage_row(prefix: &str, lane: TaskEventTruthLaneClass) -> TaskEventTruthRow {
        TaskEventTruthRow {
            row_id: format!("row:{prefix}:lineage_admission"),
            lane_class: lane,
            row_class: TaskEventTruthRowClass::LineageAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            envelope_field_class: EnvelopeFieldClass::NotApplicable,
            downstream_surface_class: DownstreamSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnLineageBreak,
            confidence_class: TaskEventTruthConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_lineage_break", doc_ref())),
            execution_context_id_binding: Some(format!("exec:m4:{prefix}:task_event_lineage")),
            additive_detail_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn projection(surface: ConsumerSurface) -> TaskEventTruthConsumerProjection {
        TaskEventTruthConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            task_event_truth_packet_id_ref:
                "packet:m4:stabilize_task_discovery_launch_profiles_rerun_last_behavior".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_wedge_vocabulary: true,
            preserves_envelope_field_vocabulary: true,
            preserves_downstream_surface_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(lane: TaskEventTruthLaneClass, prefix: &str) -> Vec<TaskEventTruthRow> {
        let mut out = vec![quality_row(prefix, lane)];
        for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(wedge_row(prefix, lane, wedge));
        }
        for field in EnvelopeFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(envelope_field_row(prefix, lane, field));
        }
        for surface in DownstreamSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(surface_row(prefix, lane, surface));
        }
        out.push(additive_detail_row(prefix, lane));
        out.push(lineage_row(prefix, lane));
        out
    }

    fn sample_input() -> TaskEventTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(TaskEventTruthLaneClass::LocalLane, "local"));
        rows.extend(lane_rows(
            TaskEventTruthLaneClass::RemoteHelperLane,
            "remote",
        ));
        rows.extend(lane_rows(TaskEventTruthLaneClass::NotebookLane, "notebook"));
        rows.extend(lane_rows(
            TaskEventTruthLaneClass::ImportedProviderLane,
            "imported",
        ));
        TaskEventTruthPacketInput {
            packet_id: "packet:m4:stabilize_task_discovery_launch_profiles_rerun_last_behavior"
                .to_owned(),
            workflow_or_surface_id:
                "workflow.runtime.stabilize_task_discovery_launch_profiles_rerun_last_behavior"
                    .to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: TaskEventTruthLaneClass::REQUIRED.to_vec(),
            rows,
            consumer_projections: ConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(projection)
                .collect(),
            source_contract_refs: vec![doc_ref()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(TaskEventTruthLaneClass::LocalLane.as_str(), "local_lane");
        assert_eq!(
            TaskEventTruthLaneClass::RemoteHelperLane.as_str(),
            "remote_helper_lane"
        );
        assert_eq!(
            TaskEventTruthLaneClass::NotebookLane.as_str(),
            "notebook_lane"
        );
        assert_eq!(
            TaskEventTruthLaneClass::ImportedProviderLane.as_str(),
            "imported_provider_lane"
        );
        assert_eq!(
            TaskEventTruthRowClass::TaskEventTruthQuality.as_str(),
            "task_event_truth_quality"
        );
        assert_eq!(SupportClass::LaunchStable.as_str(), "launch_stable");
        assert_eq!(WedgeClass::TaskDiscovery.as_str(), "task_discovery");
        assert_eq!(WedgeClass::RerunLast.as_str(), "rerun_last");
        assert_eq!(EnvelopeFieldClass::EventId.as_str(), "event_id");
        assert_eq!(EnvelopeFieldClass::FallbackFlag.as_str(), "fallback_flag");
        assert_eq!(DownstreamSurfaceClass::Problems.as_str(), "problems");
        assert_eq!(
            DownstreamSurfaceClass::RerunSurface.as_str(),
            "rerun_surface"
        );
        assert_eq!(EvidenceClass::EvidenceUnbound.as_str(), "evidence_unbound");
        assert_eq!(KnownLimitClass::LimitUnbound.as_str(), "limit_unbound");
        assert_eq!(
            DowngradeAutomationClass::AutomationUnbound.as_str(),
            "automation_unbound"
        );
        assert_eq!(
            ConsumerSurface::ConformanceDashboard.as_str(),
            "conformance_dashboard"
        );
        assert_eq!(PromotionState::BlocksStable.as_str(), "blocks_stable");
        assert_eq!(
            FindingKind::LaunchStableWithUnboundBinding.as_str(),
            "launch_stable_with_unbound_binding"
        );
        assert_eq!(
            FindingKind::AdditiveDetailRowAdmitsFlattening.as_str(),
            "additive_detail_row_admits_flattening"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = TaskEventTruthPacket::materialize(sample_input());
        assert_eq!(
            packet.promotion_state,
            PromotionState::Stable,
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
                "support:m4:stabilize_task_discovery_launch_profiles_rerun_last_behavior",
                "2026-05-26T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = TaskEventTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingEvidenceClass));
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::LaunchStableWithUnboundBinding));
    }

    #[test]
    fn missing_wedge_admission_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(row.row_class, TaskEventTruthRowClass::WedgeAdmission)
                && row.wedge_class == WedgeClass::RerunLast
                && row.lane_class == TaskEventTruthLaneClass::LocalLane)
        });
        let packet = TaskEventTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingWedgeAdmissionCoverage));
    }

    #[test]
    fn missing_envelope_field_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(row.row_class, TaskEventTruthRowClass::EnvelopeFieldBinding)
                && row.envelope_field_class == EnvelopeFieldClass::FallbackFlag
                && row.lane_class == TaskEventTruthLaneClass::LocalLane)
        });
        let packet = TaskEventTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingEnvelopeFieldCoverage));
    }

    #[test]
    fn additive_detail_admitting_flattening_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(
                row.row_class,
                TaskEventTruthRowClass::AdditiveDetailPreservation
            ) && row.lane_class == TaskEventTruthLaneClass::LocalLane
            {
                row.additive_detail_preserved = false;
                break;
            }
        }
        let packet = TaskEventTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::AdditiveDetailRowAdmitsFlattening
        }));
    }

    #[test]
    fn lineage_admission_without_execution_context_id_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, TaskEventTruthRowClass::LineageAdmission)
                && row.lane_class == TaskEventTruthLaneClass::LocalLane
            {
                row.execution_context_id_binding = None;
                break;
            }
        }
        let packet = TaskEventTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::LineageAdmissionMissingExecutionContextId
        }));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = SupportClass::LaunchStableBelow;
        input.rows[0].disclosure_ref = None;
        let packet = TaskEventTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input.consumer_projections.retain(|projection| {
            projection.consumer_surface != ConsumerSurface::ConformanceDashboard
        });
        let packet = TaskEventTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_envelope_field_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_envelope_field_vocabulary = false;
            }
        }
        let packet = TaskEventTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::EnvelopeFieldVocabularyCollapsed));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = TaskEventTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
