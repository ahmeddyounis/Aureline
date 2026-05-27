//! Integrated-terminal stabilization truth packet for the M4 stable
//! lane.
//!
//! This module pins how local, remote/helper, container, and restored
//! terminal sessions expose one boundary truth that downstream
//! consumers (terminal pane chips, transcript export surface, browser
//! handoff, restore surface, CLI/headless inspector, support export,
//! release proof index, Help/About proof card, and the conformance
//! dashboard) read verbatim. Surfaces MUST NOT mint local copies,
//! paraphrase fields, or fork their own host-boundary, clipboard, or
//! transcript-versus-live semantics; they project this packet.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `terminal_stabilization_quality` row cannot prove:
//!
//! - the four wedges (`host_boundary_chip`, `clipboard_posture`,
//!   `transcript_export`, `restore_no_rerun`) each have a structured
//!   wedge_admission row,
//! - each typed host-boundary field is admitted exactly once
//!   (`host_or_session_identity`, `route_cue`, `trust_state`,
//!   `restore_state`, `target_or_cwd_hint`) via one
//!   `host_boundary_field_binding` row per field,
//! - each clipboard-posture surface is admitted exactly once
//!   (`clipboard_route_local_vs_remote`, `bracketed_paste_state`,
//!   `multiline_paste_guardrail`, `admin_suppression`,
//!   `high_risk_paste_review`) via one `clipboard_posture_binding`
//!   row per surface,
//! - each transcript-export field is admitted exactly once
//!   (`transcript_versus_live_session`, `host_session_boundary_cue`,
//!   `redaction_state`) via one `transcript_export_field_binding`
//!   row per field,
//! - one `restore_no_rerun_attestation` row attests that restored
//!   sessions are transcript-only and that no silent rerun is
//!   permitted (`attests_no_silent_rerun: true`),
//! - one stable `execution_context_id` lineage object threads through
//!   every surface that consumes the terminal session.
//!
//! Every row binds a closed `terminal_stabilization_lane_class`,
//! `terminal_stabilization_row_class`, `support_class`,
//! `wedge_class`, `host_boundary_field_class`,
//! `clipboard_posture_class`, `transcript_export_field_class`,
//! `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and
//! `terminal_stabilization_confidence_class` plus an `evidence_refs`
//! array and a `disclosure_ref` whenever the row is narrowed below
//! launch-stable, declares a non-`none_declared` known limit, or
//! binds a non-`none` downgrade automation.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! command lines, raw process environment bytes, raw scrollback
//! bodies, raw secrets, or ambient credentials past the boundary. A
//! row that claims `launch_stable` while leaving its support, known
//! limit, downgrade automation, or evidence class unbound is refused;
//! the validator narrows below launch-stable instead of inheriting an
//! adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`TerminalStabilizationTruthPacket`].
pub const TERMINAL_STABILIZATION_TRUTH_PACKET_RECORD_KIND: &str =
    "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_stable_packet";

/// Stable record-kind tag for [`TerminalStabilizationTruthSupportExport`].
pub const TERMINAL_STABILIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_support_export";

/// Integer schema version for the terminal-stabilization truth packet.
pub const TERMINAL_STABILIZATION_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const TERMINAL_STABILIZATION_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const TERMINAL_STABILIZATION_TRUTH_DOC_REF: &str =
    "docs/runtime/m4/stabilize-integrated-terminal-boundaries-clipboard-posture-transcript-export.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const TERMINAL_STABILIZATION_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/stabilize-integrated-terminal-boundaries-clipboard-posture-transcript-export.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const TERMINAL_STABILIZATION_TRUTH_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export";

/// Repo-relative path of the checked-in stable packet.
pub const TERMINAL_STABILIZATION_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.json";

/// Closed terminal-stabilization lane vocabulary. Every required lane
/// MUST have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalStabilizationLaneClass {
    /// Local-host terminal sessions.
    LocalLane,
    /// Remote / helper attach terminal sessions (SSH, remote agent).
    RemoteHelperLane,
    /// Container-attached terminal sessions.
    ContainerLane,
    /// Restored transcript-only sessions (restore-no-rerun lane).
    RestoredLane,
}

impl TerminalStabilizationLaneClass {
    /// Every required terminal-stabilization lane, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::LocalLane,
        Self::RemoteHelperLane,
        Self::ContainerLane,
        Self::RestoredLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalLane => "local_lane",
            Self::RemoteHelperLane => "remote_helper_lane",
            Self::ContainerLane => "container_lane",
            Self::RestoredLane => "restored_lane",
        }
    }
}

/// Closed terminal-stabilization row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalStabilizationRowClass {
    /// The lane's headline terminal-stabilization qualification row.
    TerminalStabilizationQuality,
    /// A row admitting one of the four wedges (host_boundary_chip,
    /// clipboard_posture, transcript_export, restore_no_rerun).
    WedgeAdmission,
    /// A row binding one typed host-boundary field exposed to the
    /// user before copy, paste, rerun, browser handoff, or transcript
    /// export.
    HostBoundaryFieldBinding,
    /// A row binding one clipboard-posture surface (local-vs-remote
    /// route, bracketed paste, multiline paste guardrail, admin
    /// suppression, high-risk paste review).
    ClipboardPostureBinding,
    /// A row binding one transcript-export field (transcript vs live
    /// session, host/session boundary cue, redaction state).
    TranscriptExportFieldBinding,
    /// A row attesting restored sessions are transcript-only and that
    /// no silent rerun is permitted.
    RestoreNoRerunAttestation,
    /// A row binding the stable `execution_context_id` (or equivalent
    /// lineage object) into emitted terminal-session truth and
    /// downstream consumer surfaces.
    LineageAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl TerminalStabilizationRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TerminalStabilizationQuality => "terminal_stabilization_quality",
            Self::WedgeAdmission => "wedge_admission",
            Self::HostBoundaryFieldBinding => "host_boundary_field_binding",
            Self::ClipboardPostureBinding => "clipboard_posture_binding",
            Self::TranscriptExportFieldBinding => "transcript_export_field_binding",
            Self::RestoreNoRerunAttestation => "restore_no_rerun_attestation",
            Self::LineageAdmission => "lineage_admission",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when this row class requires a bound wedge.
    pub const fn requires_wedge(self) -> bool {
        matches!(self, Self::WedgeAdmission)
    }

    /// True when this row class requires a bound host-boundary field.
    pub const fn requires_host_boundary_field(self) -> bool {
        matches!(self, Self::HostBoundaryFieldBinding)
    }

    /// True when this row class requires a bound clipboard-posture
    /// surface.
    pub const fn requires_clipboard_posture(self) -> bool {
        matches!(self, Self::ClipboardPostureBinding)
    }

    /// True when this row class requires a bound transcript-export
    /// field.
    pub const fn requires_transcript_export_field(self) -> bool {
        matches!(self, Self::TranscriptExportFieldBinding)
    }
}

/// Closed support-class vocabulary applied to a terminal-stabilization
/// row. A row is never `launch_stable` while its known limit, downgrade
/// automation, or evidence class is unbound; the validator demotes it
/// instead of inheriting an adjacent launch-stable row.
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

/// Closed terminal-stabilization wedge vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `wedge_admission` row for each
/// required wedge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WedgeClass {
    /// Host-boundary chip wedge (the typed chip carrying host, route,
    /// trust, restore, and target/cwd hints shown before any mutating
    /// action).
    HostBoundaryChip,
    /// Clipboard posture wedge (local-vs-remote route, bracketed
    /// paste, multiline guardrail, admin suppression, high-risk paste
    /// review).
    ClipboardPosture,
    /// Transcript export wedge (transcript-vs-live, host/session
    /// boundary cue, redaction state).
    TranscriptExport,
    /// Restore-no-rerun wedge (restored sessions are transcript-only
    /// and never silently rerun).
    RestoreNoRerun,
    /// The row is not bound to a wedge.
    NotApplicable,
}

impl WedgeClass {
    /// Every required wedge for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::HostBoundaryChip,
        Self::ClipboardPosture,
        Self::TranscriptExport,
        Self::RestoreNoRerun,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostBoundaryChip => "host_boundary_chip",
            Self::ClipboardPosture => "clipboard_posture",
            Self::TranscriptExport => "transcript_export",
            Self::RestoreNoRerun => "restore_no_rerun",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed host-boundary-field vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `host_boundary_field_binding` row
/// for each required field so the typed host-boundary chip carries
/// one truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostBoundaryFieldClass {
    /// Host or session identity (host id, session id, attached
    /// transport).
    HostOrSessionIdentity,
    /// Local-versus-remote route cue.
    RouteCue,
    /// Trust state (trusted, untrusted, degraded).
    TrustState,
    /// Restore state (live, restored transcript, reconnecting,
    /// blocked).
    RestoreState,
    /// Target or cwd hint surfaced beside the chip.
    TargetOrCwdHint,
    /// The row is not bound to a host-boundary field.
    NotApplicable,
}

impl HostBoundaryFieldClass {
    /// Every required host-boundary field per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::HostOrSessionIdentity,
        Self::RouteCue,
        Self::TrustState,
        Self::RestoreState,
        Self::TargetOrCwdHint,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostOrSessionIdentity => "host_or_session_identity",
            Self::RouteCue => "route_cue",
            Self::TrustState => "trust_state",
            Self::RestoreState => "restore_state",
            Self::TargetOrCwdHint => "target_or_cwd_hint",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed clipboard-posture vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `clipboard_posture_binding` row for
/// each posture surface so copy, paste, and protocol-driven clipboard
/// writes never bypass the explicit policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardPostureClass {
    /// Local-versus-remote clipboard route.
    ClipboardRouteLocalVsRemote,
    /// Bracketed-paste state fidelity (mode tracked, escapes handled
    /// safely).
    BracketedPasteState,
    /// Multiline paste guardrail (review/confirm path for multiline
    /// or newline-terminating paste).
    MultilinePasteGuardrail,
    /// Admin / privileged-shell suppression of clipboard writes from
    /// protocol escapes.
    AdminSuppression,
    /// High-risk paste review (sensitive target / broad-input review
    /// step).
    HighRiskPasteReview,
    /// The row is not bound to a clipboard-posture surface.
    NotApplicable,
}

impl ClipboardPostureClass {
    /// Every required clipboard-posture surface per `launch_stable`
    /// lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::ClipboardRouteLocalVsRemote,
        Self::BracketedPasteState,
        Self::MultilinePasteGuardrail,
        Self::AdminSuppression,
        Self::HighRiskPasteReview,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClipboardRouteLocalVsRemote => "clipboard_route_local_vs_remote",
            Self::BracketedPasteState => "bracketed_paste_state",
            Self::MultilinePasteGuardrail => "multiline_paste_guardrail",
            Self::AdminSuppression => "admin_suppression",
            Self::HighRiskPasteReview => "high_risk_paste_review",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed transcript-export field vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `transcript_export_field_binding`
/// row for each required field so transcript export and browser
/// handoff never blur live-versus-transcript semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptExportFieldClass {
    /// Transcript-versus-live-session distinction surfaced before
    /// export / reopen.
    TranscriptVersusLiveSession,
    /// Host / session boundary cue carried in the export header.
    HostSessionBoundaryCue,
    /// Redaction state of the exported transcript (which scrollback
    /// classes are redacted, which are preserved).
    RedactionState,
    /// The row is not bound to a transcript-export field.
    NotApplicable,
}

impl TranscriptExportFieldClass {
    /// Every required transcript-export field per `launch_stable`
    /// lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 3] = [
        Self::TranscriptVersusLiveSession,
        Self::HostSessionBoundaryCue,
        Self::RedactionState,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TranscriptVersusLiveSession => "transcript_versus_live_session",
            Self::HostSessionBoundaryCue => "host_session_boundary_cue",
            Self::RedactionState => "redaction_state",
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

/// Closed known-limit vocabulary attached to a terminal-stabilization
/// row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The lane only certifies the local subset.
    LocalLaneSubsetOnly,
    /// The lane only certifies the remote/helper subset.
    RemoteHelperSubsetOnly,
    /// The lane only certifies the container subset.
    ContainerSubsetOnly,
    /// The lane only certifies the restored-transcript subset.
    RestoredSubsetOnly,
    /// The lane only certifies a subset of the four required wedges.
    WedgeAdmissionSubsetOnly,
    /// The lane only certifies a subset of the five host-boundary
    /// fields.
    HostBoundaryFieldSubsetOnly,
    /// The lane only certifies a subset of the five clipboard-posture
    /// surfaces.
    ClipboardPostureSubsetOnly,
    /// The lane only certifies a subset of the three transcript-export
    /// fields.
    TranscriptExportFieldSubsetOnly,
    /// The lane admits silent rerun on restore (never qualifies
    /// stable).
    RestoreAdmitsSilentRerun,
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
            Self::LocalLaneSubsetOnly => "local_lane_subset_only",
            Self::RemoteHelperSubsetOnly => "remote_helper_subset_only",
            Self::ContainerSubsetOnly => "container_subset_only",
            Self::RestoredSubsetOnly => "restored_subset_only",
            Self::WedgeAdmissionSubsetOnly => "wedge_admission_subset_only",
            Self::HostBoundaryFieldSubsetOnly => "host_boundary_field_subset_only",
            Self::ClipboardPostureSubsetOnly => "clipboard_posture_subset_only",
            Self::TranscriptExportFieldSubsetOnly => "transcript_export_field_subset_only",
            Self::RestoreAdmitsSilentRerun => "restore_admits_silent_rerun",
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
    /// Automatically narrow when a required host-boundary field is
    /// unbound.
    AutoNarrowOnHostBoundaryFieldGap,
    /// Automatically narrow when a clipboard-posture surface binding
    /// is missing.
    AutoNarrowOnClipboardPostureGap,
    /// Automatically narrow when a transcript-export field binding is
    /// missing.
    AutoNarrowOnTranscriptExportFieldGap,
    /// Automatically narrow when restored sessions admit silent
    /// rerun.
    AutoNarrowOnRestoreAdmitsSilentRerun,
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
            Self::AutoNarrowOnHostBoundaryFieldGap => "auto_narrow_on_host_boundary_field_gap",
            Self::AutoNarrowOnClipboardPostureGap => "auto_narrow_on_clipboard_posture_gap",
            Self::AutoNarrowOnTranscriptExportFieldGap => {
                "auto_narrow_on_transcript_export_field_gap"
            }
            Self::AutoNarrowOnRestoreAdmitsSilentRerun => {
                "auto_narrow_on_restore_admits_silent_rerun"
            }
            Self::AutoNarrowOnLineageBreak => "auto_narrow_on_lineage_break",
            Self::AutoBlockOnMissingEvidence => "auto_block_on_missing_evidence",
            Self::ManualOnlyPendingReview => "manual_only_pending_review",
            Self::AutomationUnbound => "automation_unbound",
        }
    }

    /// True when this automation class satisfies the automation-binding
    /// invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::AutomationUnbound)
    }

    /// True when this automation class must surface an explicit
    /// disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::None | Self::AutomationUnbound)
    }
}

/// Closed confidence-class vocabulary for a terminal-stabilization row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalStabilizationConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until
    /// evidence grows.
    LowConfidence,
}

impl TerminalStabilizationConfidenceClass {
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
    /// A lane claiming launch_stable is missing a required host-boundary field.
    MissingHostBoundaryFieldCoverage,
    /// A lane claiming launch_stable is missing a required clipboard-posture surface.
    MissingClipboardPostureCoverage,
    /// A lane claiming launch_stable is missing a required transcript-export field.
    MissingTranscriptExportFieldCoverage,
    /// A lane claiming launch_stable is missing the restore-no-rerun attestation row.
    MissingRestoreNoRerunAttestation,
    /// A restore-no-rerun attestation admits silent rerun.
    RestoreNoRerunAttestationAdmitsSilentRerun,
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
    /// A host-boundary-field row drops its field binding.
    HostBoundaryFieldNotApplicable,
    /// A non-host-boundary-field row binds a field it cannot certify.
    HostBoundaryFieldNotPermittedOnRowClass,
    /// A clipboard-posture row drops its surface binding.
    ClipboardPostureNotApplicable,
    /// A non-clipboard-posture row binds a surface it cannot certify.
    ClipboardPostureNotPermittedOnRowClass,
    /// A transcript-export-field row drops its field binding.
    TranscriptExportFieldNotApplicable,
    /// A non-transcript-export-field row binds a field it cannot certify.
    TranscriptExportFieldNotPermittedOnRowClass,
    /// A lineage-admission row does not bind a lineage object id.
    LineageAdmissionMissingExecutionContextId,
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
    /// A projection collapses the host-boundary-field vocabulary.
    HostBoundaryFieldVocabularyCollapsed,
    /// A projection collapses the clipboard-posture vocabulary.
    ClipboardPostureVocabularyCollapsed,
    /// A projection collapses the transcript-export-field vocabulary.
    TranscriptExportFieldVocabularyCollapsed,
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
            Self::MissingHostBoundaryFieldCoverage => "missing_host_boundary_field_coverage",
            Self::MissingClipboardPostureCoverage => "missing_clipboard_posture_coverage",
            Self::MissingTranscriptExportFieldCoverage => {
                "missing_transcript_export_field_coverage"
            }
            Self::MissingRestoreNoRerunAttestation => "missing_restore_no_rerun_attestation",
            Self::RestoreNoRerunAttestationAdmitsSilentRerun => {
                "restore_no_rerun_attestation_admits_silent_rerun"
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
            Self::HostBoundaryFieldNotApplicable => "host_boundary_field_not_applicable",
            Self::HostBoundaryFieldNotPermittedOnRowClass => {
                "host_boundary_field_not_permitted_on_row_class"
            }
            Self::ClipboardPostureNotApplicable => "clipboard_posture_not_applicable",
            Self::ClipboardPostureNotPermittedOnRowClass => {
                "clipboard_posture_not_permitted_on_row_class"
            }
            Self::TranscriptExportFieldNotApplicable => "transcript_export_field_not_applicable",
            Self::TranscriptExportFieldNotPermittedOnRowClass => {
                "transcript_export_field_not_permitted_on_row_class"
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
            Self::HostBoundaryFieldVocabularyCollapsed => "host_boundary_field_vocabulary_collapsed",
            Self::ClipboardPostureVocabularyCollapsed => "clipboard_posture_vocabulary_collapsed",
            Self::TranscriptExportFieldVocabularyCollapsed => {
                "transcript_export_field_vocabulary_collapsed"
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

/// Consumer surface that must inherit the packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Terminal pane chrome (per-pane host-boundary chip).
    TerminalPane,
    /// Transcript export surface (export header + body envelope).
    TranscriptExportSurface,
    /// Browser handoff surface (open-in-browser / preview handoff).
    BrowserHandoffSurface,
    /// Restore surface (transcript-only reopen, restore-no-rerun cue).
    RestoreSurface,
    /// CLI / headless inspection surface.
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
    pub const REQUIRED: [Self; 9] = [
        Self::TerminalPane,
        Self::TranscriptExportSurface,
        Self::BrowserHandoffSurface,
        Self::RestoreSurface,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TerminalPane => "terminal_pane",
            Self::TranscriptExportSurface => "transcript_export_surface",
            Self::BrowserHandoffSurface => "browser_handoff_surface",
            Self::RestoreSurface => "restore_surface",
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

/// One terminal-stabilization truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalStabilizationRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Terminal-stabilization lane this row certifies.
    pub lane_class: TerminalStabilizationLaneClass,
    /// Row class.
    pub row_class: TerminalStabilizationRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Wedge bound by the row (or `not_applicable`).
    pub wedge_class: WedgeClass,
    /// Host-boundary field bound by the row (or `not_applicable`).
    pub host_boundary_field_class: HostBoundaryFieldClass,
    /// Clipboard-posture surface bound by the row (or `not_applicable`).
    pub clipboard_posture_class: ClipboardPostureClass,
    /// Transcript-export field bound by the row (or `not_applicable`).
    pub transcript_export_field_class: TranscriptExportFieldClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: TerminalStabilizationConfidenceClass,
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
    /// For restore_no_rerun_attestation rows, true when the row
    /// attests no silent rerun on restore.
    #[serde(default)]
    pub attests_no_silent_rerun: bool,
    /// True when raw command lines, raw process environment bytes,
    /// or raw scrollback bodies are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl TerminalStabilizationRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalStabilizationConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Terminal-stabilization packet id consumed by the projection.
    pub terminal_stabilization_truth_packet_id_ref: String,
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
    /// True when the host-boundary-field vocabulary is preserved verbatim.
    pub preserves_host_boundary_field_vocabulary: bool,
    /// True when the clipboard-posture vocabulary is preserved verbatim.
    pub preserves_clipboard_posture_vocabulary: bool,
    /// True when the transcript-export-field vocabulary is preserved verbatim.
    pub preserves_transcript_export_field_vocabulary: bool,
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

impl TerminalStabilizationConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.terminal_stabilization_truth_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_wedge_vocabulary
            && self.preserves_host_boundary_field_vocabulary
            && self.preserves_clipboard_posture_vocabulary
            && self.preserves_transcript_export_field_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`TerminalStabilizationTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalStabilizationTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Terminal-stabilization lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<TerminalStabilizationLaneClass>,
    /// Terminal-stabilization rows.
    #[serde(default)]
    pub rows: Vec<TerminalStabilizationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<TerminalStabilizationConsumerProjection>,
    /// Source contracts consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Terminal-stabilization truth packet certifying local, remote/helper,
/// container, and restored terminal sessions at the M4 launch-stable
/// grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalStabilizationTruthPacket {
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
    /// Terminal-stabilization lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<TerminalStabilizationLaneClass>,
    /// Terminal-stabilization rows.
    #[serde(default)]
    pub rows: Vec<TerminalStabilizationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<TerminalStabilizationConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl TerminalStabilizationTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: TerminalStabilizationTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: TERMINAL_STABILIZATION_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: TERMINAL_STABILIZATION_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable terminal-stabilization
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
            .map(TerminalStabilizationLaneClass::as_str)
            .collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(TerminalStabilizationRowClass::as_str)
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

    /// Returns the unique host-boundary-field tokens observed across rows.
    pub fn host_boundary_field_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.host_boundary_field_class);
        }
        set.into_iter()
            .map(HostBoundaryFieldClass::as_str)
            .collect()
    }

    /// Returns the unique clipboard-posture tokens observed across rows.
    pub fn clipboard_posture_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.clipboard_posture_class);
        }
        set.into_iter().map(ClipboardPostureClass::as_str).collect()
    }

    /// Returns the unique transcript-export-field tokens observed across rows.
    pub fn transcript_export_field_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.transcript_export_field_class);
        }
        set.into_iter()
            .map(TranscriptExportFieldClass::as_str)
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
    ) -> TerminalStabilizationTruthSupportExport {
        TerminalStabilizationTruthSupportExport {
            record_kind: TERMINAL_STABILIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TERMINAL_STABILIZATION_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            terminal_stabilization_truth_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            terminal_stabilization_truth_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != TERMINAL_STABILIZATION_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "terminal-stabilization truth packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != TERMINAL_STABILIZATION_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "terminal-stabilization truth packet has the wrong schema version",
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
                "packet must declare at least one covered terminal-stabilization lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLaneCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "no row covers terminal-stabilization lane {}",
                        lane.as_str()
                    ),
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
                        "row {} admits raw command lines, raw env bytes, or raw scrollback bodies past the boundary",
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

            if row.row_class.requires_host_boundary_field()
                && matches!(
                    row.host_boundary_field_class,
                    HostBoundaryFieldClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::HostBoundaryFieldNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a host_boundary_field_binding but has no bound host-boundary field",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_host_boundary_field()
                && !matches!(
                    row.host_boundary_field_class,
                    HostBoundaryFieldClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::HostBoundaryFieldNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds host-boundary field {}; only host_boundary_field_binding rows may bind a field",
                        row.row_id,
                        row.row_class.as_str(),
                        row.host_boundary_field_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_clipboard_posture()
                && matches!(
                    row.clipboard_posture_class,
                    ClipboardPostureClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ClipboardPostureNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a clipboard_posture_binding but has no bound clipboard-posture surface",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_clipboard_posture()
                && !matches!(
                    row.clipboard_posture_class,
                    ClipboardPostureClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ClipboardPostureNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds clipboard-posture surface {}; only clipboard_posture_binding rows may bind a surface",
                        row.row_id,
                        row.row_class.as_str(),
                        row.clipboard_posture_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_transcript_export_field()
                && matches!(
                    row.transcript_export_field_class,
                    TranscriptExportFieldClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::TranscriptExportFieldNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a transcript_export_field_binding but has no bound transcript-export field",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_transcript_export_field()
                && !matches!(
                    row.transcript_export_field_class,
                    TranscriptExportFieldClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::TranscriptExportFieldNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds transcript-export field {}; only transcript_export_field_binding rows may bind a field",
                        row.row_id,
                        row.row_class.as_str(),
                        row.transcript_export_field_class.as_str()
                    ),
                ));
            }

            if matches!(row.row_class, TerminalStabilizationRowClass::LineageAdmission)
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
                TerminalStabilizationRowClass::RestoreNoRerunAttestation
            ) && !row.attests_no_silent_rerun
            {
                findings.push(ValidationFinding::new(
                    FindingKind::RestoreNoRerunAttestationAdmitsSilentRerun,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a restore_no_rerun_attestation but admits silent rerun",
                        row.row_id
                    ),
                ));
            }

            if matches!(
                row.confidence_class,
                TerminalStabilizationConfidenceClass::LowConfidence
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
                        TerminalStabilizationRowClass::TerminalStabilizationQuality
                    )
                    && matches!(row.support_class, SupportClass::LaunchStable)
            });
            if !lane_claims_launch {
                continue;
            }

            for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, TerminalStabilizationRowClass::WedgeAdmission)
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

            for field in HostBoundaryFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            TerminalStabilizationRowClass::HostBoundaryFieldBinding
                        )
                        && row.host_boundary_field_class == field
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingHostBoundaryFieldCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no host_boundary_field_binding row for {}",
                            lane.as_str(),
                            field.as_str()
                        ),
                    ));
                }
            }

            for surface in ClipboardPostureClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            TerminalStabilizationRowClass::ClipboardPostureBinding
                        )
                        && row.clipboard_posture_class == surface
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingClipboardPostureCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no clipboard_posture_binding row for {}",
                            lane.as_str(),
                            surface.as_str()
                        ),
                    ));
                }
            }

            for field in TranscriptExportFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            TerminalStabilizationRowClass::TranscriptExportFieldBinding
                        )
                        && row.transcript_export_field_class == field
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingTranscriptExportFieldCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no transcript_export_field_binding row for {}",
                            lane.as_str(),
                            field.as_str()
                        ),
                    ));
                }
            }

            let has_restore_no_rerun = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(
                        row.row_class,
                        TerminalStabilizationRowClass::RestoreNoRerunAttestation
                    )
                    && row.attests_no_silent_rerun
            });
            if !has_restore_no_rerun {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingRestoreNoRerunAttestation,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no restore_no_rerun_attestation row attesting no silent rerun",
                        lane.as_str()
                    ),
                ));
            }

            let has_lineage = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, TerminalStabilizationRowClass::LineageAdmission)
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
                        "projection {} does not preserve terminal-stabilization truth",
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
            if !projection.preserves_host_boundary_field_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::HostBoundaryFieldVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the host-boundary-field vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_clipboard_posture_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ClipboardPostureVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the clipboard-posture vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_transcript_export_field_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::TranscriptExportFieldVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the transcript-export-field vocabulary",
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
pub struct TerminalStabilizationTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub terminal_stabilization_truth_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub terminal_stabilization_truth_packet: TerminalStabilizationTruthPacket,
}

impl TerminalStabilizationTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == TERMINAL_STABILIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == TERMINAL_STABILIZATION_TRUTH_SCHEMA_VERSION
            && self.terminal_stabilization_truth_packet_id_ref
                == self.terminal_stabilization_truth_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.terminal_stabilization_truth_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum TerminalStabilizationTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for TerminalStabilizationTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "terminal-stabilization truth packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "terminal-stabilization truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for TerminalStabilizationTruthArtifactError {}

/// Returns the checked-in stable terminal-stabilization truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse
/// or validate.
pub fn current_stable_terminal_stabilization_truth_packet(
) -> Result<TerminalStabilizationTruthPacket, TerminalStabilizationTruthArtifactError> {
    let packet: TerminalStabilizationTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/runtime/m4/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.json"
    )))
    .map_err(TerminalStabilizationTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(TerminalStabilizationTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        TERMINAL_STABILIZATION_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        TERMINAL_STABILIZATION_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn base_row(
        prefix: &str,
        lane: TerminalStabilizationLaneClass,
        suffix: &str,
        row_class: TerminalStabilizationRowClass,
    ) -> TerminalStabilizationRow {
        TerminalStabilizationRow {
            row_id: format!("row:{prefix}:{suffix}"),
            lane_class: lane,
            row_class,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            host_boundary_field_class: HostBoundaryFieldClass::NotApplicable,
            clipboard_posture_class: ClipboardPostureClass::NotApplicable,
            transcript_export_field_class: TranscriptExportFieldClass::NotApplicable,
            evidence_class: EvidenceClass::FixtureRepoEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnLineageBreak,
            confidence_class: TerminalStabilizationConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_lineage_break", doc_ref())),
            execution_context_id_binding: None,
            attests_no_silent_rerun: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn quality_row(prefix: &str, lane: TerminalStabilizationLaneClass) -> TerminalStabilizationRow {
        let mut row = base_row(
            prefix,
            lane,
            "quality",
            TerminalStabilizationRowClass::TerminalStabilizationQuality,
        );
        row.evidence_class = EvidenceClass::ReleaseEvidenceReview;
        row.downgrade_automation_class = DowngradeAutomationClass::AutoBlockOnMissingEvidence;
        row.disclosure_ref = Some(format!("{}#auto_block_on_missing_evidence", doc_ref()));
        row.evidence_refs = vec![doc_ref(), fixture_ref()];
        row
    }

    fn wedge_row(
        prefix: &str,
        lane: TerminalStabilizationLaneClass,
        wedge: WedgeClass,
    ) -> TerminalStabilizationRow {
        let mut row = base_row(
            prefix,
            lane,
            &format!("wedge:{}", wedge.as_str()),
            TerminalStabilizationRowClass::WedgeAdmission,
        );
        row.wedge_class = wedge;
        row.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
        row.downgrade_automation_class = DowngradeAutomationClass::AutoNarrowOnWedgeAdmissionGap;
        row.disclosure_ref = Some(format!("{}#auto_narrow_on_wedge_admission_gap", doc_ref()));
        row
    }

    fn host_field_row(
        prefix: &str,
        lane: TerminalStabilizationLaneClass,
        field: HostBoundaryFieldClass,
    ) -> TerminalStabilizationRow {
        let mut row = base_row(
            prefix,
            lane,
            &format!("host_boundary:{}", field.as_str()),
            TerminalStabilizationRowClass::HostBoundaryFieldBinding,
        );
        row.host_boundary_field_class = field;
        row.evidence_class = EvidenceClass::AutomatedFunctionalEvidence;
        row.downgrade_automation_class = DowngradeAutomationClass::AutoNarrowOnHostBoundaryFieldGap;
        row.disclosure_ref = Some(format!(
            "{}#auto_narrow_on_host_boundary_field_gap",
            doc_ref()
        ));
        row
    }

    fn clipboard_row(
        prefix: &str,
        lane: TerminalStabilizationLaneClass,
        surface: ClipboardPostureClass,
    ) -> TerminalStabilizationRow {
        let mut row = base_row(
            prefix,
            lane,
            &format!("clipboard:{}", surface.as_str()),
            TerminalStabilizationRowClass::ClipboardPostureBinding,
        );
        row.clipboard_posture_class = surface;
        row.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
        row.downgrade_automation_class = DowngradeAutomationClass::AutoNarrowOnClipboardPostureGap;
        row.disclosure_ref = Some(format!("{}#auto_narrow_on_clipboard_posture_gap", doc_ref()));
        row
    }

    fn transcript_row(
        prefix: &str,
        lane: TerminalStabilizationLaneClass,
        field: TranscriptExportFieldClass,
    ) -> TerminalStabilizationRow {
        let mut row = base_row(
            prefix,
            lane,
            &format!("transcript_export:{}", field.as_str()),
            TerminalStabilizationRowClass::TranscriptExportFieldBinding,
        );
        row.transcript_export_field_class = field;
        row.evidence_class = EvidenceClass::AutomatedFunctionalEvidence;
        row.downgrade_automation_class =
            DowngradeAutomationClass::AutoNarrowOnTranscriptExportFieldGap;
        row.disclosure_ref = Some(format!(
            "{}#auto_narrow_on_transcript_export_field_gap",
            doc_ref()
        ));
        row
    }

    fn restore_row(prefix: &str, lane: TerminalStabilizationLaneClass) -> TerminalStabilizationRow {
        let mut row = base_row(
            prefix,
            lane,
            "restore_no_rerun",
            TerminalStabilizationRowClass::RestoreNoRerunAttestation,
        );
        row.evidence_class = EvidenceClass::FailureRecoveryDrillEvidence;
        row.downgrade_automation_class =
            DowngradeAutomationClass::AutoNarrowOnRestoreAdmitsSilentRerun;
        row.disclosure_ref = Some(format!(
            "{}#auto_narrow_on_restore_admits_silent_rerun",
            doc_ref()
        ));
        row.attests_no_silent_rerun = true;
        row
    }

    fn lineage_row(prefix: &str, lane: TerminalStabilizationLaneClass) -> TerminalStabilizationRow {
        let mut row = base_row(
            prefix,
            lane,
            "lineage_admission",
            TerminalStabilizationRowClass::LineageAdmission,
        );
        row.evidence_class = EvidenceClass::AutomatedFunctionalEvidence;
        row.downgrade_automation_class = DowngradeAutomationClass::AutoNarrowOnLineageBreak;
        row.disclosure_ref = Some(format!("{}#auto_narrow_on_lineage_break", doc_ref()));
        row.execution_context_id_binding =
            Some(format!("exec:m4:{prefix}:terminal_session_lineage"));
        row
    }

    fn projection(surface: ConsumerSurface) -> TerminalStabilizationConsumerProjection {
        TerminalStabilizationConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            terminal_stabilization_truth_packet_id_ref:
                "packet:m4:stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export"
                    .to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_wedge_vocabulary: true,
            preserves_host_boundary_field_vocabulary: true,
            preserves_clipboard_posture_vocabulary: true,
            preserves_transcript_export_field_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(
        lane: TerminalStabilizationLaneClass,
        prefix: &str,
    ) -> Vec<TerminalStabilizationRow> {
        let mut rows = vec![quality_row(prefix, lane)];
        for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(wedge_row(prefix, lane, wedge));
        }
        for field in HostBoundaryFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(host_field_row(prefix, lane, field));
        }
        for surface in ClipboardPostureClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(clipboard_row(prefix, lane, surface));
        }
        for field in TranscriptExportFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(transcript_row(prefix, lane, field));
        }
        rows.push(restore_row(prefix, lane));
        rows.push(lineage_row(prefix, lane));
        rows
    }

    fn sample_input() -> TerminalStabilizationTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(TerminalStabilizationLaneClass::LocalLane, "local"));
        rows.extend(lane_rows(
            TerminalStabilizationLaneClass::RemoteHelperLane,
            "remote",
        ));
        rows.extend(lane_rows(
            TerminalStabilizationLaneClass::ContainerLane,
            "container",
        ));
        rows.extend(lane_rows(
            TerminalStabilizationLaneClass::RestoredLane,
            "restored",
        ));
        TerminalStabilizationTruthPacketInput {
            packet_id:
                "packet:m4:stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export"
                    .to_owned(),
            workflow_or_surface_id:
                "workflow.runtime.stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export"
                    .to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: TerminalStabilizationLaneClass::REQUIRED.to_vec(),
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
        assert_eq!(
            TerminalStabilizationLaneClass::LocalLane.as_str(),
            "local_lane"
        );
        assert_eq!(
            TerminalStabilizationLaneClass::RestoredLane.as_str(),
            "restored_lane"
        );
        assert_eq!(WedgeClass::HostBoundaryChip.as_str(), "host_boundary_chip");
        assert_eq!(WedgeClass::RestoreNoRerun.as_str(), "restore_no_rerun");
        assert_eq!(
            HostBoundaryFieldClass::RouteCue.as_str(),
            "route_cue"
        );
        assert_eq!(
            ClipboardPostureClass::BracketedPasteState.as_str(),
            "bracketed_paste_state"
        );
        assert_eq!(
            ClipboardPostureClass::HighRiskPasteReview.as_str(),
            "high_risk_paste_review"
        );
        assert_eq!(
            TranscriptExportFieldClass::TranscriptVersusLiveSession.as_str(),
            "transcript_versus_live_session"
        );
        assert_eq!(
            ConsumerSurface::ConformanceDashboard.as_str(),
            "conformance_dashboard"
        );
        assert_eq!(
            FindingKind::RestoreNoRerunAttestationAdmitsSilentRerun.as_str(),
            "restore_no_rerun_attestation_admits_silent_rerun"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = TerminalStabilizationTruthPacket::materialize(sample_input());
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
                "support:m4:stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export",
                "2026-05-26T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = TerminalStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingEvidenceClass));
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::LaunchStableWithUnboundBinding));
    }

    #[test]
    fn missing_clipboard_posture_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                TerminalStabilizationRowClass::ClipboardPostureBinding
            ) && row.clipboard_posture_class == ClipboardPostureClass::HighRiskPasteReview
                && row.lane_class == TerminalStabilizationLaneClass::LocalLane)
        });
        let packet = TerminalStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingClipboardPostureCoverage));
    }

    #[test]
    fn restore_attestation_admits_silent_rerun_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(
                row.row_class,
                TerminalStabilizationRowClass::RestoreNoRerunAttestation
            ) && row.lane_class == TerminalStabilizationLaneClass::RestoredLane
            {
                row.attests_no_silent_rerun = false;
                break;
            }
        }
        let packet = TerminalStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|f| f.finding_kind
            == FindingKind::RestoreNoRerunAttestationAdmitsSilentRerun));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = SupportClass::LaunchStableBelow;
        input.rows[0].disclosure_ref = None;
        let packet = TerminalStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input
            .consumer_projections
            .retain(|p| p.consumer_surface != ConsumerSurface::ConformanceDashboard);
        let packet = TerminalStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_clipboard_posture_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_clipboard_posture_vocabulary = false;
            }
        }
        let packet = TerminalStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::ClipboardPostureVocabularyCollapsed));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = TerminalStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
