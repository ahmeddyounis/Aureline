//! Task / test / debug / terminal event-normalization truth packet
//! for the M4 stable lane.
//!
//! This module pins how task, test, debug, and terminal event streams
//! normalize into one export-safe execution ledger. Surfaces (editor
//! run surface, task panel, test runner surface, debug surface,
//! terminal pane, CLI/headless inspector, AI tool surface, review
//! surface, support export, release proof index, Help/About proof
//! card, and the conformance dashboard) read this packet verbatim;
//! they MUST NOT mint local envelope copies, paraphrase fields,
//! collapse the canonical lifecycle into display text, or flatten
//! adapter-isolated source-kind detail into a single undifferentiated
//! event stream.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `event_normalization_quality` row cannot prove:
//!
//! - the four wedges (`envelope_canonicalization`,
//!   `source_kind_negotiation`, `lifecycle_normalization`,
//!   `export_preservation`) each have a structured wedge_admission
//!   row,
//! - each canonical envelope field is admitted exactly once
//!   (`event_id`, `workspace_id`, `target_id`, `source_kind`,
//!   `confidence`, `timestamp`, `execution_context_id`,
//!   `payload_kind`, `raw_payload_ref`, `provenance`) via one
//!   `envelope_field_binding` row per field,
//! - each canonical source kind is admitted exactly once (`native`,
//!   `bsp`, `bazel_bep`, `structured_output`, `heuristic_parser`)
//!   via one `source_kind_binding` row per kind,
//! - each canonical lifecycle event is admitted exactly once
//!   (`task_queued`, `target_graph_ready`, `task_started`,
//!   `progress_updated`, `diagnostic_emitted`, `test_case_started`,
//!   `test_case_finished`, `artifact_published`, `task_finished`)
//!   via one `lifecycle_event_binding` row per event,
//! - one `raw_payload_retention_attestation` row attests that
//!   replay, export, and support packets preserve `source_kind`,
//!   `confidence`, and the adapter raw payload reference rather than
//!   flattening them away (`attests_raw_payload_retained: true`),
//! - one stable `execution_context_id` lineage object threads
//!   through every emitted envelope and downstream consumer surface.
//!
//! Every row binds a closed `event_normalization_lane_class`,
//! `event_normalization_row_class`, `support_class`, `wedge_class`,
//! `envelope_field_class`, `source_kind_class`,
//! `lifecycle_event_class`, `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and
//! `event_normalization_confidence_class` plus an `evidence_refs`
//! array and a `disclosure_ref` whenever the row is narrowed below
//! launch-stable, declares a non-`none_declared` known limit, or
//! binds a non-`none` downgrade automation.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! command lines, raw process environment bytes, raw scrollback
//! bodies, secrets, or ambient credentials past the boundary. A row
//! that claims `launch_stable` while leaving its support, known
//! limit, downgrade automation, or evidence class unbound is
//! refused; the validator narrows below launch-stable instead of
//! inheriting an adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`EventNormalizationTruthPacket`].
pub const EVENT_NORMALIZATION_TRUTH_PACKET_RECORD_KIND: &str =
    "harden_task_test_debug_and_terminal_event_normalization_truth_stable_packet";

/// Stable record-kind tag for [`EventNormalizationTruthSupportExport`].
pub const EVENT_NORMALIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "harden_task_test_debug_and_terminal_event_normalization_truth_support_export";

/// Integer schema version for the event-normalization truth packet.
pub const EVENT_NORMALIZATION_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const EVENT_NORMALIZATION_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/harden_task_test_debug_and_terminal_event_normalization_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const EVENT_NORMALIZATION_TRUTH_DOC_REF: &str =
    "docs/runtime/m4/harden-task-test-debug-and-terminal-event-normalization.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const EVENT_NORMALIZATION_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/harden-task-test-debug-and-terminal-event-normalization.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const EVENT_NORMALIZATION_TRUTH_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/harden_task_test_debug_and_terminal_event_normalization";

/// Repo-relative path of the checked-in stable packet.
pub const EVENT_NORMALIZATION_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/harden_task_test_debug_and_terminal_event_normalization_truth_packet.json";

/// Closed event-normalization lane vocabulary. Every required lane
/// MUST have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventNormalizationLaneClass {
    /// Task event-normalization lane (task runner output → canonical envelope).
    TaskLane,
    /// Test event-normalization lane (test runner output → canonical envelope).
    TestLane,
    /// Debug event-normalization lane (debug adapter output → canonical envelope).
    DebugLane,
    /// Terminal event-normalization lane (terminal scrollback / OSC → canonical envelope).
    TerminalLane,
}

impl EventNormalizationLaneClass {
    /// Every required event-normalization lane, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::TaskLane,
        Self::TestLane,
        Self::DebugLane,
        Self::TerminalLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskLane => "task_lane",
            Self::TestLane => "test_lane",
            Self::DebugLane => "debug_lane",
            Self::TerminalLane => "terminal_lane",
        }
    }
}

/// Closed event-normalization row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventNormalizationRowClass {
    /// The lane's headline event-normalization qualification row.
    EventNormalizationQuality,
    /// A row admitting one of the four wedges
    /// (`envelope_canonicalization`, `source_kind_negotiation`,
    /// `lifecycle_normalization`, `export_preservation`).
    WedgeAdmission,
    /// A row binding one canonical envelope field
    /// (`event_id`, `workspace_id`, `target_id`, `source_kind`,
    /// `confidence`, `timestamp`, `execution_context_id`,
    /// `payload_kind`, `raw_payload_ref`, `provenance`).
    EnvelopeFieldBinding,
    /// A row binding one canonical source kind (`native`, `bsp`,
    /// `bazel_bep`, `structured_output`, `heuristic_parser`).
    SourceKindBinding,
    /// A row binding one canonical lifecycle event
    /// (`task_queued`, `target_graph_ready`, `task_started`,
    /// `progress_updated`, `diagnostic_emitted`, `test_case_started`,
    /// `test_case_finished`, `artifact_published`, `task_finished`).
    LifecycleEventBinding,
    /// A row binding one downstream consumer surface to the shared
    /// execution ledger.
    ConsumerSurfaceBinding,
    /// A row attesting that replay, export, and support packets
    /// preserve `source_kind`, `confidence`, and the adapter raw
    /// payload reference (no flattening into one undifferentiated
    /// ledger).
    RawPayloadRetentionAttestation,
    /// A row binding the stable `execution_context_id` lineage
    /// object into emitted envelopes and downstream consumer
    /// surfaces.
    LineageAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl EventNormalizationRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EventNormalizationQuality => "event_normalization_quality",
            Self::WedgeAdmission => "wedge_admission",
            Self::EnvelopeFieldBinding => "envelope_field_binding",
            Self::SourceKindBinding => "source_kind_binding",
            Self::LifecycleEventBinding => "lifecycle_event_binding",
            Self::ConsumerSurfaceBinding => "consumer_surface_binding",
            Self::RawPayloadRetentionAttestation => "raw_payload_retention_attestation",
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

    /// True when this row class requires a bound source kind.
    pub const fn requires_source_kind(self) -> bool {
        matches!(self, Self::SourceKindBinding)
    }

    /// True when this row class requires a bound lifecycle event.
    pub const fn requires_lifecycle_event(self) -> bool {
        matches!(self, Self::LifecycleEventBinding)
    }

    /// True when this row class requires a bound consumer surface.
    pub const fn requires_consumer_surface(self) -> bool {
        matches!(self, Self::ConsumerSurfaceBinding)
    }
}

/// Closed support-class vocabulary applied to an event-normalization
/// row. A row is never `launch_stable` while its known limit,
/// downgrade automation, or evidence class is unbound; the validator
/// demotes it instead of inheriting an adjacent launch-stable row.
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

    /// True when this support class satisfies the support-binding
    /// invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::SupportUnbound)
    }

    /// True when the support class must surface a disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::LaunchStable)
    }
}

/// Closed event-normalization wedge vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `wedge_admission` row for each
/// required wedge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WedgeClass {
    /// Envelope canonicalization wedge (one envelope shape across
    /// task / test / debug / terminal lanes).
    EnvelopeCanonicalization,
    /// Source-kind negotiation wedge (adapter-isolated capability
    /// negotiation for native, BSP, Bazel BEP, structured-output,
    /// and heuristic-parser sources).
    SourceKindNegotiation,
    /// Lifecycle normalization wedge (the canonical lifecycle set
    /// emitted by every lane).
    LifecycleNormalization,
    /// Export preservation wedge (replay / export / support packets
    /// preserve source_kind, confidence, and adapter raw payload
    /// reference).
    ExportPreservation,
    /// The row is not bound to a wedge.
    NotApplicable,
}

impl WedgeClass {
    /// Every required wedge for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::EnvelopeCanonicalization,
        Self::SourceKindNegotiation,
        Self::LifecycleNormalization,
        Self::ExportPreservation,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnvelopeCanonicalization => "envelope_canonicalization",
            Self::SourceKindNegotiation => "source_kind_negotiation",
            Self::LifecycleNormalization => "lifecycle_normalization",
            Self::ExportPreservation => "export_preservation",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed canonical-envelope-field vocabulary. Every lane claiming
/// `launch_stable` MUST publish an `envelope_field_binding` row for
/// each required field so downstream surfaces never invent a second
/// truth model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvelopeFieldClass {
    /// Stable event id.
    EventId,
    /// Workspace id the event was emitted in.
    WorkspaceId,
    /// Target id the event was emitted for.
    TargetId,
    /// Source kind (native / bsp / bazel_bep / structured_output /
    /// heuristic_parser).
    SourceKind,
    /// Confidence flag on the envelope.
    Confidence,
    /// Capture timestamp.
    Timestamp,
    /// Execution-context id threading lineage.
    ExecutionContextId,
    /// Payload kind discriminator for the envelope payload.
    PayloadKind,
    /// Reference to the retained raw adapter payload.
    RawPayloadRef,
    /// Provenance object for the envelope.
    Provenance,
    /// The row is not bound to an envelope field.
    NotApplicable,
}

impl EnvelopeFieldClass {
    /// Every required canonical envelope field for a
    /// `launch_stable` lane, in declaration order.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 10] = [
        Self::EventId,
        Self::WorkspaceId,
        Self::TargetId,
        Self::SourceKind,
        Self::Confidence,
        Self::Timestamp,
        Self::ExecutionContextId,
        Self::PayloadKind,
        Self::RawPayloadRef,
        Self::Provenance,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EventId => "event_id",
            Self::WorkspaceId => "workspace_id",
            Self::TargetId => "target_id",
            Self::SourceKind => "source_kind",
            Self::Confidence => "confidence",
            Self::Timestamp => "timestamp",
            Self::ExecutionContextId => "execution_context_id",
            Self::PayloadKind => "payload_kind",
            Self::RawPayloadRef => "raw_payload_ref",
            Self::Provenance => "provenance",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed canonical source-kind vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `source_kind_binding` row for each
/// required source kind so adapter capability negotiation and raw
/// payload retention stay adapter-isolated rather than flattened
/// into one undifferentiated ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKindClass {
    /// Native adapter source.
    Native,
    /// Build Server Protocol (BSP) source.
    Bsp,
    /// Bazel Build Event Protocol (BEP) source.
    BazelBep,
    /// Structured-output source (machine-readable runner output).
    StructuredOutput,
    /// Heuristic-parser source (regex / pattern-recognized output).
    HeuristicParser,
    /// The row is not bound to a source kind.
    NotApplicable,
}

impl SourceKindClass {
    /// Every required source kind for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::Native,
        Self::Bsp,
        Self::BazelBep,
        Self::StructuredOutput,
        Self::HeuristicParser,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Bsp => "bsp",
            Self::BazelBep => "bazel_bep",
            Self::StructuredOutput => "structured_output",
            Self::HeuristicParser => "heuristic_parser",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed canonical lifecycle-event vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `lifecycle_event_binding` row for
/// each required lifecycle event so local, remote/helper, and
/// imported-provider lanes serialize into the same lifecycle set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleEventClass {
    /// Task queued event.
    TaskQueued,
    /// Target graph ready event.
    TargetGraphReady,
    /// Task started event.
    TaskStarted,
    /// Progress updated event.
    ProgressUpdated,
    /// Diagnostic emitted event.
    DiagnosticEmitted,
    /// Test case started event.
    TestCaseStarted,
    /// Test case finished event.
    TestCaseFinished,
    /// Artifact published event.
    ArtifactPublished,
    /// Task finished event.
    TaskFinished,
    /// The row is not bound to a lifecycle event.
    NotApplicable,
}

impl LifecycleEventClass {
    /// Every required lifecycle event for a `launch_stable` lane, in
    /// declaration order.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 9] = [
        Self::TaskQueued,
        Self::TargetGraphReady,
        Self::TaskStarted,
        Self::ProgressUpdated,
        Self::DiagnosticEmitted,
        Self::TestCaseStarted,
        Self::TestCaseFinished,
        Self::ArtifactPublished,
        Self::TaskFinished,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskQueued => "task_queued",
            Self::TargetGraphReady => "target_graph_ready",
            Self::TaskStarted => "task_started",
            Self::ProgressUpdated => "progress_updated",
            Self::DiagnosticEmitted => "diagnostic_emitted",
            Self::TestCaseStarted => "test_case_started",
            Self::TestCaseFinished => "test_case_finished",
            Self::ArtifactPublished => "artifact_published",
            Self::TaskFinished => "task_finished",
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
    /// The row is backed by a docs / help disclosure (gap label only).
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

    /// True when this evidence class satisfies the evidence-binding
    /// invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::EvidenceUnbound)
    }
}

/// Closed known-limit vocabulary attached to an event-normalization
/// row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The lane only certifies the task subset.
    TaskLaneSubsetOnly,
    /// The lane only certifies the test subset.
    TestLaneSubsetOnly,
    /// The lane only certifies the debug subset.
    DebugLaneSubsetOnly,
    /// The lane only certifies the terminal subset.
    TerminalLaneSubsetOnly,
    /// The lane only certifies a subset of the four required wedges.
    WedgeAdmissionSubsetOnly,
    /// The lane only certifies a subset of the ten envelope fields.
    EnvelopeFieldSubsetOnly,
    /// The lane only certifies a subset of the five source kinds.
    SourceKindSubsetOnly,
    /// The lane only certifies a subset of the nine lifecycle events.
    LifecycleEventSubsetOnly,
    /// The lane only certifies a subset of the required consumer surfaces.
    ConsumerSurfaceSubsetOnly,
    /// The lane admits flattening source_kind/confidence on export
    /// (never qualifies stable).
    ExportFlattensSourceKindOrConfidence,
    /// The lane is at beta-grade-only capability sample.
    BetaCapabilitySampleOnly,
    /// The row has no bound known-limit class; this never qualifies
    /// stable.
    LimitUnbound,
}

impl KnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::TaskLaneSubsetOnly => "task_lane_subset_only",
            Self::TestLaneSubsetOnly => "test_lane_subset_only",
            Self::DebugLaneSubsetOnly => "debug_lane_subset_only",
            Self::TerminalLaneSubsetOnly => "terminal_lane_subset_only",
            Self::WedgeAdmissionSubsetOnly => "wedge_admission_subset_only",
            Self::EnvelopeFieldSubsetOnly => "envelope_field_subset_only",
            Self::SourceKindSubsetOnly => "source_kind_subset_only",
            Self::LifecycleEventSubsetOnly => "lifecycle_event_subset_only",
            Self::ConsumerSurfaceSubsetOnly => "consumer_surface_subset_only",
            Self::ExportFlattensSourceKindOrConfidence => {
                "export_flattens_source_kind_or_confidence"
            }
            Self::BetaCapabilitySampleOnly => "beta_capability_sample_only",
            Self::LimitUnbound => "limit_unbound",
        }
    }

    /// True when this known-limit class satisfies the limit-binding
    /// invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::LimitUnbound)
    }

    /// True when this known-limit class must surface an explicit
    /// disclosure ref.
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
    /// Automatically narrow when a required source kind binding is missing.
    AutoNarrowOnSourceKindGap,
    /// Automatically narrow when a required lifecycle event binding is missing.
    AutoNarrowOnLifecycleEventGap,
    /// Automatically narrow when a required consumer surface binding is missing.
    AutoNarrowOnConsumerSurfaceGap,
    /// Automatically narrow when export flattens source_kind / confidence /
    /// raw payload retention.
    AutoNarrowOnExportFlattening,
    /// Automatically narrow when the lineage object breaks.
    AutoNarrowOnLineageBreak,
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
            Self::AutoNarrowOnSourceKindGap => "auto_narrow_on_source_kind_gap",
            Self::AutoNarrowOnLifecycleEventGap => "auto_narrow_on_lifecycle_event_gap",
            Self::AutoNarrowOnConsumerSurfaceGap => "auto_narrow_on_consumer_surface_gap",
            Self::AutoNarrowOnExportFlattening => "auto_narrow_on_export_flattening",
            Self::AutoNarrowOnLineageBreak => "auto_narrow_on_lineage_break",
            Self::AutoBlockOnMissingEvidence => "auto_block_on_missing_evidence",
            Self::ManualOnlyPendingReview => "manual_only_pending_review",
            Self::AutomationUnbound => "automation_unbound",
        }
    }

    /// True when this automation class satisfies the
    /// automation-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::AutomationUnbound)
    }

    /// True when this automation class must surface an explicit
    /// disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::None | Self::AutomationUnbound)
    }
}

/// Closed confidence-class vocabulary for an event-normalization row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventNormalizationConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until
    /// evidence grows.
    LowConfidence,
}

impl EventNormalizationConfidenceClass {
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
    /// A lane claiming launch_stable is missing a required source-kind binding.
    MissingSourceKindCoverage,
    /// A lane claiming launch_stable is missing a required lifecycle-event binding.
    MissingLifecycleEventCoverage,
    /// A lane claiming launch_stable is missing a required consumer-surface binding.
    MissingConsumerSurfaceCoverage,
    /// A lane claiming launch_stable is missing the raw-payload retention attestation.
    MissingRawPayloadRetentionAttestation,
    /// A raw-payload retention attestation admits flattening.
    RawPayloadRetentionAttestationAdmitsFlattening,
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
    /// A source-kind row drops its kind binding.
    SourceKindNotApplicable,
    /// A non-source-kind row binds a kind it cannot certify.
    SourceKindNotPermittedOnRowClass,
    /// A lifecycle-event row drops its event binding.
    LifecycleEventNotApplicable,
    /// A non-lifecycle-event row binds an event it cannot certify.
    LifecycleEventNotPermittedOnRowClass,
    /// A consumer-surface row drops its surface binding.
    ConsumerSurfaceNotApplicable,
    /// A non-consumer-surface row binds a surface it cannot certify.
    ConsumerSurfaceNotPermittedOnRowClass,
    /// A lineage-admission row does not bind a lineage object id.
    LineageAdmissionMissingExecutionContextId,
    /// A row admits raw command lines, env bytes, scrollback, or other private material.
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
    /// A projection collapses the source-kind vocabulary.
    SourceKindVocabularyCollapsed,
    /// A projection collapses the lifecycle-event vocabulary.
    LifecycleEventVocabularyCollapsed,
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
            Self::MissingSourceKindCoverage => "missing_source_kind_coverage",
            Self::MissingLifecycleEventCoverage => "missing_lifecycle_event_coverage",
            Self::MissingConsumerSurfaceCoverage => "missing_consumer_surface_coverage",
            Self::MissingRawPayloadRetentionAttestation => {
                "missing_raw_payload_retention_attestation"
            }
            Self::RawPayloadRetentionAttestationAdmitsFlattening => {
                "raw_payload_retention_attestation_admits_flattening"
            }
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
            Self::SourceKindNotApplicable => "source_kind_not_applicable",
            Self::SourceKindNotPermittedOnRowClass => "source_kind_not_permitted_on_row_class",
            Self::LifecycleEventNotApplicable => "lifecycle_event_not_applicable",
            Self::LifecycleEventNotPermittedOnRowClass => {
                "lifecycle_event_not_permitted_on_row_class"
            }
            Self::ConsumerSurfaceNotApplicable => "consumer_surface_not_applicable",
            Self::ConsumerSurfaceNotPermittedOnRowClass => {
                "consumer_surface_not_permitted_on_row_class"
            }
            Self::LineageAdmissionMissingExecutionContextId => {
                "lineage_admission_missing_execution_context_id"
            }
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
            Self::SourceKindVocabularyCollapsed => "source_kind_vocabulary_collapsed",
            Self::LifecycleEventVocabularyCollapsed => "lifecycle_event_vocabulary_collapsed",
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
    /// Editor run / launch surface.
    EditorRunSurface,
    /// Task panel surface.
    TaskPanel,
    /// Test runner surface.
    TestRunnerSurface,
    /// Debug surface.
    DebugSurface,
    /// Terminal pane surface.
    TerminalPane,
    /// CLI / headless inspection surface.
    CliHeadless,
    /// AI tool / agent surface.
    AiToolSurface,
    /// Review surface (PR / change review).
    ReviewSurface,
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
    pub const REQUIRED: [Self; 12] = [
        Self::EditorRunSurface,
        Self::TaskPanel,
        Self::TestRunnerSurface,
        Self::DebugSurface,
        Self::TerminalPane,
        Self::CliHeadless,
        Self::AiToolSurface,
        Self::ReviewSurface,
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
            Self::TestRunnerSurface => "test_runner_surface",
            Self::DebugSurface => "debug_surface",
            Self::TerminalPane => "terminal_pane",
            Self::CliHeadless => "cli_headless",
            Self::AiToolSurface => "ai_tool_surface",
            Self::ReviewSurface => "review_surface",
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

/// One event-normalization truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventNormalizationRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Event-normalization lane this row certifies.
    pub lane_class: EventNormalizationLaneClass,
    /// Row class.
    pub row_class: EventNormalizationRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Wedge bound by the row (or `not_applicable`).
    pub wedge_class: WedgeClass,
    /// Envelope field bound by the row (or `not_applicable`).
    pub envelope_field_class: EnvelopeFieldClass,
    /// Source kind bound by the row (or `not_applicable`).
    pub source_kind_class: SourceKindClass,
    /// Lifecycle event bound by the row (or `not_applicable`).
    pub lifecycle_event_class: LifecycleEventClass,
    /// Consumer surface bound by the row (or `not_applicable`).
    pub consumer_surface_class: ConsumerSurfaceBindingClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: EventNormalizationConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `launch_stable`, declares a non-`none_declared` known limit,
    /// or binds a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// For lineage_admission rows, the bound `execution_context_id`
    /// token. Required when `row_class == LineageAdmission`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id_binding: Option<String>,
    /// For raw_payload_retention_attestation rows, true when the row
    /// attests that replay/export/support packets preserve
    /// `source_kind`, `confidence`, and adapter raw payload reference.
    #[serde(default)]
    pub attests_raw_payload_retained: bool,
    /// True when raw command lines, raw process environment bytes,
    /// raw scrollback bodies, or raw capsule bodies are excluded
    /// from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl EventNormalizationRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Closed consumer-surface-binding vocabulary used by row-level
/// surface bindings. Distinct from the top-level [`ConsumerSurface`]
/// which is the full required projection set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurfaceBindingClass {
    /// Editor run surface.
    EditorRunSurface,
    /// Task panel surface.
    TaskPanel,
    /// Test runner surface.
    TestRunnerSurface,
    /// Debug surface.
    DebugSurface,
    /// Terminal pane surface.
    TerminalPane,
    /// CLI / headless inspector surface.
    CliHeadless,
    /// AI tool surface.
    AiToolSurface,
    /// Review surface.
    ReviewSurface,
    /// Support export surface.
    SupportExport,
    /// The row is not bound to a consumer surface.
    NotApplicable,
}

impl ConsumerSurfaceBindingClass {
    /// Every required consumer surface binding for a
    /// `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 9] = [
        Self::EditorRunSurface,
        Self::TaskPanel,
        Self::TestRunnerSurface,
        Self::DebugSurface,
        Self::TerminalPane,
        Self::CliHeadless,
        Self::AiToolSurface,
        Self::ReviewSurface,
        Self::SupportExport,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorRunSurface => "editor_run_surface",
            Self::TaskPanel => "task_panel",
            Self::TestRunnerSurface => "test_runner_surface",
            Self::DebugSurface => "debug_surface",
            Self::TerminalPane => "terminal_pane",
            Self::CliHeadless => "cli_headless",
            Self::AiToolSurface => "ai_tool_surface",
            Self::ReviewSurface => "review_surface",
            Self::SupportExport => "support_export",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventNormalizationConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Event-normalization packet id consumed by the projection.
    pub event_normalization_truth_packet_id_ref: String,
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
    /// True when the source-kind vocabulary is preserved verbatim.
    pub preserves_source_kind_vocabulary: bool,
    /// True when the lifecycle-event vocabulary is preserved verbatim.
    pub preserves_lifecycle_event_vocabulary: bool,
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

impl EventNormalizationConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.event_normalization_truth_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_wedge_vocabulary
            && self.preserves_envelope_field_vocabulary
            && self.preserves_source_kind_vocabulary
            && self.preserves_lifecycle_event_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for
/// [`EventNormalizationTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventNormalizationTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Event-normalization lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<EventNormalizationLaneClass>,
    /// Event-normalization rows.
    #[serde(default)]
    pub rows: Vec<EventNormalizationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<EventNormalizationConsumerProjection>,
    /// Source contracts consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Event-normalization truth packet certifying task, test, debug, and
/// terminal event-normalization lanes at the M4 launch-stable grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventNormalizationTruthPacket {
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
    /// Event-normalization lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<EventNormalizationLaneClass>,
    /// Event-normalization rows.
    #[serde(default)]
    pub rows: Vec<EventNormalizationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<EventNormalizationConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl EventNormalizationTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: EventNormalizationTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: EVENT_NORMALIZATION_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: EVENT_NORMALIZATION_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable event-normalization
    /// invariants.
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
            .map(EventNormalizationLaneClass::as_str)
            .collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(EventNormalizationRowClass::as_str)
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

    /// Returns the unique source-kind tokens observed across rows.
    pub fn source_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.source_kind_class);
        }
        set.into_iter().map(SourceKindClass::as_str).collect()
    }

    /// Returns the unique lifecycle-event tokens observed across rows.
    pub fn lifecycle_event_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.lifecycle_event_class);
        }
        set.into_iter().map(LifecycleEventClass::as_str).collect()
    }

    /// Returns the unique consumer-surface-binding tokens observed
    /// across rows.
    pub fn consumer_surface_binding_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.consumer_surface_class);
        }
        set.into_iter()
            .map(ConsumerSurfaceBindingClass::as_str)
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
    ) -> EventNormalizationTruthSupportExport {
        EventNormalizationTruthSupportExport {
            record_kind: EVENT_NORMALIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: EVENT_NORMALIZATION_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            event_normalization_truth_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            event_normalization_truth_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != EVENT_NORMALIZATION_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "event-normalization truth packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != EVENT_NORMALIZATION_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "event-normalization truth packet has the wrong schema version",
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
                "packet must declare at least one covered event-normalization lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers event-normalization lane {}", lane.as_str()),
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
                        "row {} admits raw command lines, raw env bytes, raw scrollback bodies, or raw capsule bodies past the boundary",
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

            if row.row_class.requires_source_kind()
                && matches!(row.source_kind_class, SourceKindClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::SourceKindNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a source_kind_binding but has no bound source kind",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_source_kind()
                && !matches!(row.source_kind_class, SourceKindClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::SourceKindNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds source kind {}; only source_kind_binding rows may bind a kind",
                        row.row_id,
                        row.row_class.as_str(),
                        row.source_kind_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_lifecycle_event()
                && matches!(
                    row.lifecycle_event_class,
                    LifecycleEventClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LifecycleEventNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a lifecycle_event_binding but has no bound lifecycle event",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_lifecycle_event()
                && !matches!(
                    row.lifecycle_event_class,
                    LifecycleEventClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LifecycleEventNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds lifecycle event {}; only lifecycle_event_binding rows may bind an event",
                        row.row_id,
                        row.row_class.as_str(),
                        row.lifecycle_event_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_consumer_surface()
                && matches!(
                    row.consumer_surface_class,
                    ConsumerSurfaceBindingClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerSurfaceNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a consumer_surface_binding but has no bound consumer surface",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_consumer_surface()
                && !matches!(
                    row.consumer_surface_class,
                    ConsumerSurfaceBindingClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerSurfaceNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds consumer surface {}; only consumer_surface_binding rows may bind a surface",
                        row.row_id,
                        row.row_class.as_str(),
                        row.consumer_surface_class.as_str()
                    ),
                ));
            }

            if matches!(row.row_class, EventNormalizationRowClass::LineageAdmission)
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
                EventNormalizationRowClass::RawPayloadRetentionAttestation
            ) && !row.attests_raw_payload_retained
            {
                findings.push(ValidationFinding::new(
                    FindingKind::RawPayloadRetentionAttestationAdmitsFlattening,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a raw_payload_retention_attestation but admits flattening source_kind, confidence, or raw payload retention",
                        row.row_id
                    ),
                ));
            }

            if matches!(
                row.confidence_class,
                EventNormalizationConfidenceClass::LowConfidence
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
                    && matches!(
                        row.row_class,
                        EventNormalizationRowClass::EventNormalizationQuality
                    )
                    && matches!(row.support_class, SupportClass::LaunchStable)
            });
            if !lane_claims_launch {
                continue;
            }

            for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, EventNormalizationRowClass::WedgeAdmission)
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
                        && matches!(
                            row.row_class,
                            EventNormalizationRowClass::EnvelopeFieldBinding
                        )
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

            for kind in SourceKindClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, EventNormalizationRowClass::SourceKindBinding)
                        && row.source_kind_class == kind
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingSourceKindCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no source_kind_binding row for {}",
                            lane.as_str(),
                            kind.as_str()
                        ),
                    ));
                }
            }

            for event in LifecycleEventClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            EventNormalizationRowClass::LifecycleEventBinding
                        )
                        && row.lifecycle_event_class == event
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingLifecycleEventCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no lifecycle_event_binding row for {}",
                            lane.as_str(),
                            event.as_str()
                        ),
                    ));
                }
            }

            for surface in ConsumerSurfaceBindingClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            EventNormalizationRowClass::ConsumerSurfaceBinding
                        )
                        && row.consumer_surface_class == surface
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingConsumerSurfaceCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no consumer_surface_binding row for {}",
                            lane.as_str(),
                            surface.as_str()
                        ),
                    ));
                }
            }

            let has_retention = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(
                        row.row_class,
                        EventNormalizationRowClass::RawPayloadRetentionAttestation
                    )
                    && row.attests_raw_payload_retained
            });
            if !has_retention {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingRawPayloadRetentionAttestation,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no raw_payload_retention_attestation row attesting preserved source_kind, confidence, and raw payload retention",
                        lane.as_str()
                    ),
                ));
            }

            let has_lineage = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, EventNormalizationRowClass::LineageAdmission)
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
                        "projection {} does not preserve event-normalization truth",
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
            if !projection.preserves_source_kind_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::SourceKindVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the source-kind vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_lifecycle_event_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::LifecycleEventVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the lifecycle-event vocabulary",
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
pub struct EventNormalizationTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub event_normalization_truth_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub event_normalization_truth_packet: EventNormalizationTruthPacket,
}

impl EventNormalizationTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == EVENT_NORMALIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == EVENT_NORMALIZATION_TRUTH_SCHEMA_VERSION
            && self.event_normalization_truth_packet_id_ref
                == self.event_normalization_truth_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.event_normalization_truth_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum EventNormalizationTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for EventNormalizationTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "event-normalization truth packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "event-normalization truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for EventNormalizationTruthArtifactError {}

/// Returns the checked-in stable event-normalization truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse
/// or validate.
pub fn current_stable_event_normalization_truth_packet(
) -> Result<EventNormalizationTruthPacket, EventNormalizationTruthArtifactError> {
    let packet: EventNormalizationTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/runtime/m4/harden_task_test_debug_and_terminal_event_normalization_truth_packet.json"
    )))
    .map_err(EventNormalizationTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(EventNormalizationTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        EVENT_NORMALIZATION_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        EVENT_NORMALIZATION_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn quality_row(prefix: &str, lane: EventNormalizationLaneClass) -> EventNormalizationRow {
        EventNormalizationRow {
            row_id: format!("row:{prefix}:quality"),
            lane_class: lane,
            row_class: EventNormalizationRowClass::EventNormalizationQuality,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            envelope_field_class: EnvelopeFieldClass::NotApplicable,
            source_kind_class: SourceKindClass::NotApplicable,
            lifecycle_event_class: LifecycleEventClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::ReleaseEvidenceReview,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: EventNormalizationConfidenceClass::HighConfidence,
            evidence_refs: vec![doc_ref(), fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_block_on_missing_evidence", doc_ref())),
            execution_context_id_binding: None,
            attests_raw_payload_retained: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn wedge_row(
        prefix: &str,
        lane: EventNormalizationLaneClass,
        wedge: WedgeClass,
    ) -> EventNormalizationRow {
        EventNormalizationRow {
            row_id: format!("row:{prefix}:wedge:{}", wedge.as_str()),
            lane_class: lane,
            row_class: EventNormalizationRowClass::WedgeAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: wedge,
            envelope_field_class: EnvelopeFieldClass::NotApplicable,
            source_kind_class: SourceKindClass::NotApplicable,
            lifecycle_event_class: LifecycleEventClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnWedgeAdmissionGap,
            confidence_class: EventNormalizationConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_wedge_admission_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_raw_payload_retained: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn envelope_row(
        prefix: &str,
        lane: EventNormalizationLaneClass,
        field: EnvelopeFieldClass,
    ) -> EventNormalizationRow {
        EventNormalizationRow {
            row_id: format!("row:{prefix}:field:{}", field.as_str()),
            lane_class: lane,
            row_class: EventNormalizationRowClass::EnvelopeFieldBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            envelope_field_class: field,
            source_kind_class: SourceKindClass::NotApplicable,
            lifecycle_event_class: LifecycleEventClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnEnvelopeFieldGap,
            confidence_class: EventNormalizationConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_envelope_field_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_raw_payload_retained: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn source_kind_row(
        prefix: &str,
        lane: EventNormalizationLaneClass,
        kind: SourceKindClass,
    ) -> EventNormalizationRow {
        EventNormalizationRow {
            row_id: format!("row:{prefix}:source_kind:{}", kind.as_str()),
            lane_class: lane,
            row_class: EventNormalizationRowClass::SourceKindBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            envelope_field_class: EnvelopeFieldClass::NotApplicable,
            source_kind_class: kind,
            lifecycle_event_class: LifecycleEventClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnSourceKindGap,
            confidence_class: EventNormalizationConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_source_kind_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_raw_payload_retained: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn lifecycle_row(
        prefix: &str,
        lane: EventNormalizationLaneClass,
        event: LifecycleEventClass,
    ) -> EventNormalizationRow {
        EventNormalizationRow {
            row_id: format!("row:{prefix}:lifecycle:{}", event.as_str()),
            lane_class: lane,
            row_class: EventNormalizationRowClass::LifecycleEventBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            envelope_field_class: EnvelopeFieldClass::NotApplicable,
            source_kind_class: SourceKindClass::NotApplicable,
            lifecycle_event_class: event,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnLifecycleEventGap,
            confidence_class: EventNormalizationConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_lifecycle_event_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_raw_payload_retained: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn consumer_row(
        prefix: &str,
        lane: EventNormalizationLaneClass,
        surface: ConsumerSurfaceBindingClass,
    ) -> EventNormalizationRow {
        EventNormalizationRow {
            row_id: format!("row:{prefix}:consumer:{}", surface.as_str()),
            lane_class: lane,
            row_class: EventNormalizationRowClass::ConsumerSurfaceBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            envelope_field_class: EnvelopeFieldClass::NotApplicable,
            source_kind_class: SourceKindClass::NotApplicable,
            lifecycle_event_class: LifecycleEventClass::NotApplicable,
            consumer_surface_class: surface,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnConsumerSurfaceGap,
            confidence_class: EventNormalizationConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_consumer_surface_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_raw_payload_retained: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn retention_row(prefix: &str, lane: EventNormalizationLaneClass) -> EventNormalizationRow {
        EventNormalizationRow {
            row_id: format!("row:{prefix}:retention"),
            lane_class: lane,
            row_class: EventNormalizationRowClass::RawPayloadRetentionAttestation,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            envelope_field_class: EnvelopeFieldClass::NotApplicable,
            source_kind_class: SourceKindClass::NotApplicable,
            lifecycle_event_class: LifecycleEventClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::FailureRecoveryDrillEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnExportFlattening,
            confidence_class: EventNormalizationConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_export_flattening", doc_ref())),
            execution_context_id_binding: None,
            attests_raw_payload_retained: true,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn lineage_row(prefix: &str, lane: EventNormalizationLaneClass) -> EventNormalizationRow {
        EventNormalizationRow {
            row_id: format!("row:{prefix}:lineage_admission"),
            lane_class: lane,
            row_class: EventNormalizationRowClass::LineageAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            envelope_field_class: EnvelopeFieldClass::NotApplicable,
            source_kind_class: SourceKindClass::NotApplicable,
            lifecycle_event_class: LifecycleEventClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnLineageBreak,
            confidence_class: EventNormalizationConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_lineage_break", doc_ref())),
            execution_context_id_binding: Some(format!("exec:m4:{prefix}:event_normalization")),
            attests_raw_payload_retained: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn projection(surface: ConsumerSurface) -> EventNormalizationConsumerProjection {
        EventNormalizationConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            event_normalization_truth_packet_id_ref:
                "packet:m4:harden_task_test_debug_and_terminal_event_normalization".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_wedge_vocabulary: true,
            preserves_envelope_field_vocabulary: true,
            preserves_source_kind_vocabulary: true,
            preserves_lifecycle_event_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(lane: EventNormalizationLaneClass, prefix: &str) -> Vec<EventNormalizationRow> {
        let mut out = vec![quality_row(prefix, lane)];
        for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(wedge_row(prefix, lane, wedge));
        }
        for field in EnvelopeFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(envelope_row(prefix, lane, field));
        }
        for kind in SourceKindClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(source_kind_row(prefix, lane, kind));
        }
        for event in LifecycleEventClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(lifecycle_row(prefix, lane, event));
        }
        for surface in ConsumerSurfaceBindingClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(consumer_row(prefix, lane, surface));
        }
        out.push(retention_row(prefix, lane));
        out.push(lineage_row(prefix, lane));
        out
    }

    fn sample_input() -> EventNormalizationTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(EventNormalizationLaneClass::TaskLane, "task"));
        rows.extend(lane_rows(EventNormalizationLaneClass::TestLane, "test"));
        rows.extend(lane_rows(EventNormalizationLaneClass::DebugLane, "debug"));
        rows.extend(lane_rows(
            EventNormalizationLaneClass::TerminalLane,
            "terminal",
        ));
        EventNormalizationTruthPacketInput {
            packet_id: "packet:m4:harden_task_test_debug_and_terminal_event_normalization"
                .to_owned(),
            workflow_or_surface_id:
                "workflow.runtime.harden_task_test_debug_and_terminal_event_normalization"
                    .to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: EventNormalizationLaneClass::REQUIRED.to_vec(),
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
        assert_eq!(EventNormalizationLaneClass::TaskLane.as_str(), "task_lane");
        assert_eq!(EventNormalizationLaneClass::TestLane.as_str(), "test_lane");
        assert_eq!(
            EventNormalizationLaneClass::DebugLane.as_str(),
            "debug_lane"
        );
        assert_eq!(
            EventNormalizationLaneClass::TerminalLane.as_str(),
            "terminal_lane"
        );
        assert_eq!(
            EventNormalizationRowClass::EventNormalizationQuality.as_str(),
            "event_normalization_quality"
        );
        assert_eq!(SupportClass::LaunchStable.as_str(), "launch_stable");
        assert_eq!(
            WedgeClass::EnvelopeCanonicalization.as_str(),
            "envelope_canonicalization"
        );
        assert_eq!(
            WedgeClass::ExportPreservation.as_str(),
            "export_preservation"
        );
        assert_eq!(EnvelopeFieldClass::EventId.as_str(), "event_id");
        assert_eq!(EnvelopeFieldClass::Provenance.as_str(), "provenance");
        assert_eq!(SourceKindClass::BazelBep.as_str(), "bazel_bep");
        assert_eq!(
            SourceKindClass::HeuristicParser.as_str(),
            "heuristic_parser"
        );
        assert_eq!(LifecycleEventClass::TaskQueued.as_str(), "task_queued");
        assert_eq!(LifecycleEventClass::TaskFinished.as_str(), "task_finished");
        assert_eq!(
            ConsumerSurfaceBindingClass::AiToolSurface.as_str(),
            "ai_tool_surface"
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
            FindingKind::RawPayloadRetentionAttestationAdmitsFlattening.as_str(),
            "raw_payload_retention_attestation_admits_flattening"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = EventNormalizationTruthPacket::materialize(sample_input());
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
                "support:m4:harden_task_test_debug_and_terminal_event_normalization",
                "2026-05-26T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = EventNormalizationTruthPacket::materialize(input);
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
    fn missing_source_kind_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(row.row_class, EventNormalizationRowClass::SourceKindBinding)
                && row.source_kind_class == SourceKindClass::BazelBep
                && row.lane_class == EventNormalizationLaneClass::TaskLane)
        });
        let packet = EventNormalizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingSourceKindCoverage));
    }

    #[test]
    fn missing_lifecycle_event_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                EventNormalizationRowClass::LifecycleEventBinding
            ) && row.lifecycle_event_class == LifecycleEventClass::TestCaseFinished
                && row.lane_class == EventNormalizationLaneClass::TestLane)
        });
        let packet = EventNormalizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingLifecycleEventCoverage));
    }

    #[test]
    fn retention_admitting_flattening_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(
                row.row_class,
                EventNormalizationRowClass::RawPayloadRetentionAttestation
            ) && row.lane_class == EventNormalizationLaneClass::TaskLane
            {
                row.attests_raw_payload_retained = false;
                break;
            }
        }
        let packet = EventNormalizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::RawPayloadRetentionAttestationAdmitsFlattening
        }));
    }

    #[test]
    fn lineage_admission_without_execution_context_id_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, EventNormalizationRowClass::LineageAdmission)
                && row.lane_class == EventNormalizationLaneClass::DebugLane
            {
                row.execution_context_id_binding = None;
                break;
            }
        }
        let packet = EventNormalizationTruthPacket::materialize(input);
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
        let packet = EventNormalizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input
            .consumer_projections
            .retain(|projection| projection.consumer_surface != ConsumerSurface::AiToolSurface);
        let packet = EventNormalizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_source_kind_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_source_kind_vocabulary = false;
            }
        }
        let packet = EventNormalizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::SourceKindVocabularyCollapsed));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = EventNormalizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
