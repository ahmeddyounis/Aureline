//! Unified diagnostic record plane for runtime, editor, review, CLI, AI, and support consumers.
//!
//! This module sits above the language diagnostic bus and task-event stream.
//! It preserves one stable diagnostic identity while projecting the same
//! source, freshness, remap, and causal-lineage fields into editor markers,
//! Problems rows, output/timeline entries, review packets, CLI explain output,
//! AI evidence, and support exports.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::tasks::{TaskDiagnosticSeverity, TaskEvent, TaskEventPayload, TaskWedgeClass};

/// Schema version emitted for unified diagnostic records.
pub const UNIFIED_DIAGNOSTIC_SCHEMA_VERSION: u32 = 1;
/// Stable record-kind tag for a canonical diagnostic record.
pub const DIAGNOSTIC_RECORD_KIND: &str = "diagnostic_record";
/// Stable record-kind tag for a diagnostic source descriptor.
pub const DIAGNOSTIC_SOURCE_RECORD_KIND: &str = "diagnostic_source";
/// Stable record-kind tag for append-only anchor remap evidence.
pub const DIAGNOSTIC_ANCHOR_REMAP_RECORD_KIND: &str = "diagnostic_anchor_remap";
/// Stable record-kind tag for a diagnostic surface projection.
pub const DIAGNOSTIC_SURFACE_PROJECTION_RECORD_KIND: &str = "diagnostic_surface_projection";
/// Stable record-kind tag for a diagnostic cluster projection.
pub const UNIFIED_DIAGNOSTIC_CLUSTER_RECORD_KIND: &str = "unified_diagnostic_cluster";
/// Stable record-kind tag for a diagnostic plane snapshot.
pub const UNIFIED_DIAGNOSTIC_PLANE_SNAPSHOT_RECORD_KIND: &str = "unified_diagnostic_plane_snapshot";
/// Stable record-kind tag for support-export diagnostic references.
pub const DIAGNOSTIC_SUPPORT_EXPORT_RECORD_KIND: &str = "diagnostic_support_export";
/// Stable record-kind tag for AI evidence diagnostic references.
pub const DIAGNOSTIC_AI_EVIDENCE_RECORD_KIND: &str = "diagnostic_ai_evidence_reference_packet";

/// Source kind that produced or preserved a diagnostic finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSourceKind {
    /// Parser, encoding, Unicode, generated-file, or structural editor guard.
    EditorStructural,
    /// Language service or native semantic analyzer.
    LanguageService,
    /// Compiler, build adapter, task adapter, or structured task output.
    BuildOrTask,
    /// Runtime, test, debug, notebook, or observed execution finding.
    RuntimeOrTest,
    /// Imported scanner, SARIF-like report, CI snapshot, or provider scan.
    ScannerImport,
    /// Policy, trust, compliance, license, or security finding.
    Policy,
    /// Heuristic parser, problem matcher, or fallback-only finding.
    Heuristic,
}

impl DiagnosticSourceKind {
    /// Beta-claimed source kinds that may emit normalized diagnostics.
    pub const ALL_BETA_CLAIMED: [Self; 7] = [
        Self::EditorStructural,
        Self::LanguageService,
        Self::BuildOrTask,
        Self::RuntimeOrTest,
        Self::ScannerImport,
        Self::Policy,
        Self::Heuristic,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorStructural => "editor_structural",
            Self::LanguageService => "language_service",
            Self::BuildOrTask => "build_or_task",
            Self::RuntimeOrTest => "runtime_or_test",
            Self::ScannerImport => "scanner_import",
            Self::Policy => "policy",
            Self::Heuristic => "heuristic",
        }
    }

    /// Projects a language diagnostic source family into the runtime plane.
    pub const fn from_language_source_family(
        source_family: aureline_language::DiagnosticSourceFamily,
    ) -> Self {
        match source_family {
            aureline_language::DiagnosticSourceFamily::EditorStructural => Self::EditorStructural,
            aureline_language::DiagnosticSourceFamily::LanguageServer
            | aureline_language::DiagnosticSourceFamily::ProjectGraph
            | aureline_language::DiagnosticSourceFamily::FrameworkOrSchemaAnalyzer
            | aureline_language::DiagnosticSourceFamily::LinterFormatterStyle => {
                Self::LanguageService
            }
            aureline_language::DiagnosticSourceFamily::CompilerOrBuild => Self::BuildOrTask,
            aureline_language::DiagnosticSourceFamily::RuntimeTestOrDebug => Self::RuntimeOrTest,
            aureline_language::DiagnosticSourceFamily::ScannerImport => Self::ScannerImport,
            aureline_language::DiagnosticSourceFamily::PolicyTrustOrSecurity => Self::Policy,
            aureline_language::DiagnosticSourceFamily::Heuristic => Self::Heuristic,
        }
    }

    /// Projects a task-event diagnostic into the runtime plane.
    pub const fn from_task_wedge(wedge: TaskWedgeClass) -> Self {
        match wedge {
            TaskWedgeClass::Build
            | TaskWedgeClass::Terminal
            | TaskWedgeClass::Package
            | TaskWedgeClass::Notebook
            | TaskWedgeClass::Generic => Self::BuildOrTask,
            TaskWedgeClass::Test | TaskWedgeClass::Debug => Self::RuntimeOrTest,
            TaskWedgeClass::AiTool => Self::Heuristic,
            TaskWedgeClass::Review => Self::Policy,
        }
    }
}

/// Origin of the diagnostic evidence copy currently held by the plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticOriginClass {
    /// Produced against the current local workspace session.
    LiveLocalSession,
    /// Produced against the current remote workspace or target session.
    LiveRemoteSession,
    /// Produced live by a managed or service-backed provider.
    ManagedProviderLive,
    /// Imported from scanner, release, review, support, or provider evidence.
    ImportedSnapshot,
    /// Replayed from preserved support evidence rather than rerun live.
    ReplayedSupportBundle,
    /// Restored from a local cache without fresh producer confirmation.
    LocalCache,
}

impl DiagnosticOriginClass {
    /// Projects a language diagnostic origin into the runtime plane.
    pub const fn from_language_origin(origin: aureline_language::DiagnosticOriginClass) -> Self {
        match origin {
            aureline_language::DiagnosticOriginClass::LiveLocalSession => Self::LiveLocalSession,
            aureline_language::DiagnosticOriginClass::LiveRemoteSession => Self::LiveRemoteSession,
            aureline_language::DiagnosticOriginClass::ManagedProviderLive => {
                Self::ManagedProviderLive
            }
            aureline_language::DiagnosticOriginClass::ImportedSnapshot => Self::ImportedSnapshot,
            aureline_language::DiagnosticOriginClass::ReplayedSupportBundle => {
                Self::ReplayedSupportBundle
            }
            aureline_language::DiagnosticOriginClass::LocalCache => Self::LocalCache,
        }
    }

    /// Returns true when this origin is imported or replayed evidence.
    pub const fn is_imported_or_replayed(self) -> bool {
        matches!(self, Self::ImportedSnapshot | Self::ReplayedSupportBundle)
    }
}

/// Plane of evidence behind a diagnostic finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticEvidencePlaneClass {
    /// Derived from source, syntax, schema, or semantic analysis.
    StaticAnalysis,
    /// Derived from compile, build, or structured task execution.
    BuildTimeExecution,
    /// Derived from a run, test, debug session, notebook, or live process.
    RuntimeOrTestExecution,
    /// Derived from policy, trust, security, compliance, or governed review logic.
    PolicyOrTrustEvaluation,
    /// Derived from imported evidence whose producer session is not current-local.
    ImportedSnapshotEvidence,
    /// Derived from heuristic parsing or correlation.
    HeuristicFallback,
}

impl DiagnosticEvidencePlaneClass {
    /// Projects a language diagnostic evidence plane into the runtime plane.
    pub const fn from_language_plane(
        evidence_plane: aureline_language::DiagnosticEvidencePlaneClass,
    ) -> Self {
        match evidence_plane {
            aureline_language::DiagnosticEvidencePlaneClass::StaticAnalysis => Self::StaticAnalysis,
            aureline_language::DiagnosticEvidencePlaneClass::BuildTimeExecution => {
                Self::BuildTimeExecution
            }
            aureline_language::DiagnosticEvidencePlaneClass::RuntimeOrTestExecution => {
                Self::RuntimeOrTestExecution
            }
            aureline_language::DiagnosticEvidencePlaneClass::PolicyOrTrustEvaluation => {
                Self::PolicyOrTrustEvaluation
            }
            aureline_language::DiagnosticEvidencePlaneClass::ImportedSnapshotEvidence => {
                Self::ImportedSnapshotEvidence
            }
            aureline_language::DiagnosticEvidencePlaneClass::HeuristicFallback => {
                Self::HeuristicFallback
            }
        }
    }
}

/// Normalized diagnostic severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverityClass {
    /// Blocking error.
    Error,
    /// Warning that does not necessarily block execution.
    Warning,
    /// Notice-level diagnostic.
    Notice,
    /// Hint-level diagnostic.
    Hint,
}

impl DiagnosticSeverityClass {
    /// Projects a language diagnostic severity into the runtime plane.
    pub const fn from_language_severity(
        severity: aureline_language::DiagnosticSeverityClass,
    ) -> Self {
        match severity {
            aureline_language::DiagnosticSeverityClass::Error => Self::Error,
            aureline_language::DiagnosticSeverityClass::Warning => Self::Warning,
            aureline_language::DiagnosticSeverityClass::Notice => Self::Notice,
            aureline_language::DiagnosticSeverityClass::Hint => Self::Hint,
        }
    }

    /// Projects a task-event diagnostic severity into the runtime plane.
    pub const fn from_task_severity(severity: TaskDiagnosticSeverity) -> Self {
        match severity {
            TaskDiagnosticSeverity::Info => Self::Notice,
            TaskDiagnosticSeverity::Warning => Self::Warning,
            TaskDiagnosticSeverity::Error | TaskDiagnosticSeverity::Fatal => Self::Error,
        }
    }
}

/// Freshness state for a diagnostic record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticFreshnessClass {
    /// Evidence is current for the admitted epoch and scope.
    Current,
    /// Evidence is reviewable but below exact-current posture.
    Recent,
    /// Cached result is warm enough to inspect with a cue.
    WarmCached,
    /// Cached result is degraded and must be disclosed.
    DegradedCached,
    /// Evidence belongs to an older epoch or target.
    Stale,
    /// Newer evidence exists and this row is retained for lineage.
    Superseded,
    /// Evidence was imported and is not live local truth.
    ImportedSnapshot,
    /// Freshness could not be proven.
    Unverified,
}

impl DiagnosticFreshnessClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Recent => "recent",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Superseded => "superseded",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::Unverified => "unverified",
        }
    }

    /// Projects a language diagnostic freshness into the runtime plane.
    pub const fn from_language_freshness(
        freshness: aureline_language::DiagnosticFreshnessClass,
    ) -> Self {
        match freshness {
            aureline_language::DiagnosticFreshnessClass::Current => Self::Current,
            aureline_language::DiagnosticFreshnessClass::Recent => Self::Recent,
            aureline_language::DiagnosticFreshnessClass::WarmCached => Self::WarmCached,
            aureline_language::DiagnosticFreshnessClass::DegradedCached => Self::DegradedCached,
            aureline_language::DiagnosticFreshnessClass::Stale => Self::Stale,
            aureline_language::DiagnosticFreshnessClass::Superseded => Self::Superseded,
            aureline_language::DiagnosticFreshnessClass::ImportedSnapshot => Self::ImportedSnapshot,
            aureline_language::DiagnosticFreshnessClass::Unverified => Self::Unverified,
        }
    }

    /// Returns true when this freshness requires visible disclosure.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Current)
    }

    /// Returns true when this freshness is imported evidence.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedSnapshot)
    }
}

/// Remap state for the current diagnostic anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticAnchorRemapStateClass {
    /// Current range is exact.
    Exact,
    /// Current range was remapped from surrounding context.
    Contextual,
    /// Current range belongs to a stale epoch.
    Stale,
    /// No current range can be shown.
    Unmapped,
    /// Imported static location has not been locally revalidated.
    ImportedStatic,
}

impl DiagnosticAnchorRemapStateClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Contextual => "contextual",
            Self::Stale => "stale",
            Self::Unmapped => "unmapped",
            Self::ImportedStatic => "imported_static",
        }
    }

    /// Projects a language diagnostic anchor remap into the runtime plane.
    pub const fn from_language_remap(
        remap: aureline_language::DiagnosticAnchorRemapStateClass,
    ) -> Self {
        match remap {
            aureline_language::DiagnosticAnchorRemapStateClass::Exact => Self::Exact,
            aureline_language::DiagnosticAnchorRemapStateClass::Contextual => Self::Contextual,
            aureline_language::DiagnosticAnchorRemapStateClass::Stale => Self::Stale,
            aureline_language::DiagnosticAnchorRemapStateClass::Unmapped
            | aureline_language::DiagnosticAnchorRemapStateClass::NotApplicable => Self::Unmapped,
            aureline_language::DiagnosticAnchorRemapStateClass::ImportedStatic => {
                Self::ImportedStatic
            }
        }
    }

    /// Returns true when this remap state requires visible disclosure.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Exact)
    }
}

/// Authority posture for a diagnostic record or source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSupportClass {
    /// Source is authoritative for the declared scope.
    Authoritative,
    /// Source is advisory and should not widen claims by itself.
    Advisory,
    /// Source is a fallback only.
    FallbackOnly,
    /// Source can be inspected but may not own live truth.
    InspectOnly,
    /// Source is unsupported for the claimed diagnostic.
    Unsupported,
}

impl DiagnosticSupportClass {
    /// Projects a language-router support class into the runtime plane.
    pub const fn from_language_support(support: aureline_language::RouterSupportClass) -> Self {
        match support {
            aureline_language::RouterSupportClass::Authoritative => Self::Authoritative,
            aureline_language::RouterSupportClass::Advisory => Self::Advisory,
            aureline_language::RouterSupportClass::FallbackOnly => Self::FallbackOnly,
            aureline_language::RouterSupportClass::InspectOnly => Self::InspectOnly,
            aureline_language::RouterSupportClass::Unsupported => Self::Unsupported,
        }
    }
}

/// Confidence class for a diagnostic source descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSourceConfidenceClass {
    /// Native or protocol-backed evidence supplied the finding directly.
    Authoritative,
    /// Structured records supplied the finding, but through a projection.
    DerivedStructured,
    /// Imported source is authoritative for its own snapshot.
    ImportedAuthoritative,
    /// Heuristic parser or fallback mapper supplied the finding.
    HeuristicParsed,
    /// Multiple signals suggest a relationship without proving causality.
    CorrelatedSuggestive,
    /// Source confidence is not established and requires review.
    UnknownRequiresReview,
}

/// Redaction posture for diagnostic exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticRedactionClass {
    /// Metadata-only diagnostic refs are safe by default.
    MetadataSafeDefault,
    /// Operator review is required before export.
    OperatorOnlyRestricted,
    /// Internal support review is required before export.
    InternalSupportRestricted,
    /// Signing or digest evidence only.
    SigningEvidenceOnly,
}

impl DiagnosticRedactionClass {
    /// Projects a language-router redaction class into the runtime plane.
    pub const fn from_language_redaction(redaction: aureline_language::RedactionClass) -> Self {
        match redaction {
            aureline_language::RedactionClass::MetadataSafeDefault => Self::MetadataSafeDefault,
            aureline_language::RedactionClass::OperatorOnlyRestricted => {
                Self::OperatorOnlyRestricted
            }
            aureline_language::RedactionClass::InternalSupportRestricted => {
                Self::InternalSupportRestricted
            }
            aureline_language::RedactionClass::SigningEvidenceOnly => Self::SigningEvidenceOnly,
        }
    }
}

/// Surface consuming a diagnostic projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSurfaceClass {
    /// Editor decoration, marker, or hover surface.
    Editor,
    /// Problems row or Problems detail surface.
    Problems,
    /// Output channel or timeline entry.
    Output,
    /// Review packet or review annotation.
    Review,
    /// CLI or headless explain output.
    CliExplain,
    /// AI evidence source reference.
    AiEvidence,
    /// Support export or support bundle reference.
    SupportExport,
}

impl DiagnosticSurfaceClass {
    /// Surfaces that must cite the same canonical diagnostic identity.
    pub const REQUIRED: [Self; 7] = [
        Self::Editor,
        Self::Problems,
        Self::Output,
        Self::Review,
        Self::CliExplain,
        Self::AiEvidence,
        Self::SupportExport,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Problems => "problems",
            Self::Output => "output",
            Self::Review => "review",
            Self::CliExplain => "cli_explain",
            Self::AiEvidence => "ai_evidence",
            Self::SupportExport => "support_export",
        }
    }
}

/// Causal link that explains what emitted or preserved a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCausalLinkKind {
    /// Task identity that emitted or consumed the diagnostic.
    Task,
    /// Run identity that emitted or confirmed the diagnostic.
    Run,
    /// Adapter or producer session identity.
    AdapterSession,
    /// Import session identity.
    ImportSession,
    /// Policy decision identity.
    PolicyDecision,
    /// Output entry identity.
    OutputEntry,
    /// Timeline entry identity.
    TimelineEntry,
    /// Rerun action identity.
    RerunAction,
    /// Review packet identity.
    ReviewPacket,
    /// Support replay identity.
    SupportReplay,
}

/// Export-safe causal link for one diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticCausalLink {
    /// Causal link kind.
    pub link_kind: DiagnosticCausalLinkKind,
    /// Opaque ref to the causal object.
    pub causal_ref: String,
    /// Export-safe causal summary.
    pub summary: String,
}

impl DiagnosticCausalLink {
    /// Builds a causal link to an opaque runtime, import, review, or support object.
    pub fn new(
        link_kind: DiagnosticCausalLinkKind,
        causal_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            link_kind,
            causal_ref: causal_ref.into(),
            summary: summary.into(),
        }
    }
}

/// Source descriptor for one canonical diagnostic record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticSource {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub diagnostic_source_schema_version: u32,
    /// Stable source descriptor id.
    pub source_id: String,
    /// Source kind that produced or preserved the finding.
    pub source_kind: DiagnosticSourceKind,
    /// Plane of evidence behind the finding.
    pub evidence_plane_class: DiagnosticEvidencePlaneClass,
    /// Origin of the evidence copy currently held by the plane.
    pub origin_class: DiagnosticOriginClass,
    /// Confidence class for the normalized source.
    pub confidence_class: DiagnosticSourceConfidenceClass,
    /// Authority posture for this source.
    pub support_class: DiagnosticSupportClass,
    /// Producer or tool reference.
    pub producer_ref: String,
    /// Tool identity reference.
    pub tool_ref: String,
    /// Tool version reference, when known.
    pub tool_version_ref: Option<String>,
    /// Adapter reference, when a task, language, or scanner adapter emitted the row.
    pub adapter_ref: Option<String>,
    /// Adapter version reference, when known.
    pub adapter_version_ref: Option<String>,
    /// Target or environment fingerprint reference.
    pub target_or_environment_ref: Option<String>,
    /// Live producer session reference, when the source is session-backed.
    pub originating_session_ref: Option<String>,
    /// Import session reference, when the source is imported.
    pub import_ref: Option<String>,
    /// Run reference, when the source came from task, test, debug, or build execution.
    pub run_ref: Option<String>,
    /// Task reference, when a task emitted the diagnostic.
    pub task_ref: Option<String>,
    /// Raw payload reference retained outside this record, when available.
    pub raw_payload_ref: Option<String>,
    /// Export-safe source summary.
    pub export_safe_summary: String,
}

impl DiagnosticSource {
    /// Builds a source descriptor with required record-kind and schema fields.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_id: impl Into<String>,
        source_kind: DiagnosticSourceKind,
        evidence_plane_class: DiagnosticEvidencePlaneClass,
        origin_class: DiagnosticOriginClass,
        confidence_class: DiagnosticSourceConfidenceClass,
        support_class: DiagnosticSupportClass,
        producer_ref: impl Into<String>,
        tool_ref: impl Into<String>,
        tool_version_ref: Option<String>,
        export_safe_summary: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: DIAGNOSTIC_SOURCE_RECORD_KIND.to_owned(),
            diagnostic_source_schema_version: UNIFIED_DIAGNOSTIC_SCHEMA_VERSION,
            source_id: source_id.into(),
            source_kind,
            evidence_plane_class,
            origin_class,
            confidence_class,
            support_class,
            producer_ref: producer_ref.into(),
            tool_ref: tool_ref.into(),
            tool_version_ref,
            adapter_ref: None,
            adapter_version_ref: None,
            target_or_environment_ref: None,
            originating_session_ref: None,
            import_ref: None,
            run_ref: None,
            task_ref: None,
            raw_payload_ref: None,
            export_safe_summary: export_safe_summary.into(),
        }
    }

    /// Returns the strongest available source-origin reference.
    pub fn origin_ref(&self) -> Option<&str> {
        self.originating_session_ref
            .as_deref()
            .or(self.import_ref.as_deref())
            .or(self.run_ref.as_deref())
            .or(self.task_ref.as_deref())
    }

    /// Returns true when the source can emit a beta diagnostic record.
    pub fn has_required_provenance(&self) -> bool {
        !self.source_id.is_empty()
            && !self.producer_ref.is_empty()
            && !self.tool_ref.is_empty()
            && self
                .tool_version_ref
                .as_ref()
                .is_some_and(|value| !value.is_empty())
            && self.origin_ref().is_some_and(|value| !value.is_empty())
    }
}

/// Append-only remap evidence for one diagnostic anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticAnchorRemap {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub diagnostic_anchor_remap_schema_version: u32,
    /// Stable remap record id.
    pub remap_id: String,
    /// Anchor family shared by compatible remaps.
    pub anchor_family_id: String,
    /// Original anchor reference, when available.
    pub original_anchor_ref: Option<String>,
    /// Current anchor reference, when available.
    pub current_anchor_ref: Option<String>,
    /// Current anchor remap state.
    pub remap_state_class: DiagnosticAnchorRemapStateClass,
    /// Evidence basis that admitted this remap.
    pub evidence_basis_ref: String,
    /// Source revision reference, when known.
    pub source_revision_ref: Option<String>,
    /// Current revision reference, when known.
    pub current_revision_ref: Option<String>,
    /// Actor or tool that produced the remap.
    pub actor_tool_ref: Option<String>,
    /// Timestamp or clock reference for the remap.
    pub produced_at: String,
    /// Export-safe remap summary.
    pub export_safe_summary: String,
}

impl DiagnosticAnchorRemap {
    /// Builds an anchor remap record with required record-kind and schema fields.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        remap_id: impl Into<String>,
        anchor_family_id: impl Into<String>,
        original_anchor_ref: Option<String>,
        current_anchor_ref: Option<String>,
        remap_state_class: DiagnosticAnchorRemapStateClass,
        evidence_basis_ref: impl Into<String>,
        produced_at: impl Into<String>,
        export_safe_summary: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: DIAGNOSTIC_ANCHOR_REMAP_RECORD_KIND.to_owned(),
            diagnostic_anchor_remap_schema_version: UNIFIED_DIAGNOSTIC_SCHEMA_VERSION,
            remap_id: remap_id.into(),
            anchor_family_id: anchor_family_id.into(),
            original_anchor_ref,
            current_anchor_ref,
            remap_state_class,
            evidence_basis_ref: evidence_basis_ref.into(),
            source_revision_ref: None,
            current_revision_ref: None,
            actor_tool_ref: None,
            produced_at: produced_at.into(),
            export_safe_summary: export_safe_summary.into(),
        }
    }

    /// Returns true when a surface must disclose the remap state.
    pub const fn requires_disclosure(&self) -> bool {
        self.remap_state_class.requires_disclosure()
    }

    /// Returns true when an editor marker may cite a current anchor.
    pub fn allows_inline_projection(&self) -> bool {
        self.current_anchor_ref.is_some()
            && matches!(
                self.remap_state_class,
                DiagnosticAnchorRemapStateClass::Exact
                    | DiagnosticAnchorRemapStateClass::Contextual
                    | DiagnosticAnchorRemapStateClass::Stale
                    | DiagnosticAnchorRemapStateClass::ImportedStatic
            )
    }
}

/// Cross-surface refs attached to one diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticSurfaceRefs {
    /// Editor decoration or marker ref.
    pub editor_decoration_ref: String,
    /// Problems row ref.
    pub problems_row_ref: String,
    /// Output entry ref.
    pub output_entry_ref: String,
    /// Timeline entry ref.
    pub timeline_entry_ref: String,
    /// Rerun action ref.
    pub rerun_action_ref: String,
    /// Review packet ref.
    pub review_packet_ref: String,
    /// CLI explain output ref.
    pub cli_explain_ref: String,
    /// AI evidence ref.
    pub ai_evidence_ref: String,
    /// Support export ref.
    pub support_export_ref: String,
}

impl DiagnosticSurfaceRefs {
    /// Returns the stable ref for one diagnostic surface.
    pub fn ref_for_surface(&self, surface_class: DiagnosticSurfaceClass) -> &str {
        match surface_class {
            DiagnosticSurfaceClass::Editor => &self.editor_decoration_ref,
            DiagnosticSurfaceClass::Problems => &self.problems_row_ref,
            DiagnosticSurfaceClass::Output => &self.output_entry_ref,
            DiagnosticSurfaceClass::Review => &self.review_packet_ref,
            DiagnosticSurfaceClass::CliExplain => &self.cli_explain_ref,
            DiagnosticSurfaceClass::AiEvidence => &self.ai_evidence_ref,
            DiagnosticSurfaceClass::SupportExport => &self.support_export_ref,
        }
    }
}

/// Canonical diagnostic record shared by all diagnostic surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub diagnostic_record_schema_version: u32,
    /// Stable diagnostic id.
    pub diagnostic_id: String,
    /// Opaque rule id reference.
    pub rule_id_ref: String,
    /// Opaque category reference.
    pub category_ref: String,
    /// Normalized severity.
    pub severity_class: DiagnosticSeverityClass,
    /// Source and provenance descriptor.
    pub source: DiagnosticSource,
    /// Freshness state for the diagnostic.
    pub freshness_class: DiagnosticFreshnessClass,
    /// Current anchor and remap state.
    pub anchor_remap: DiagnosticAnchorRemap,
    /// Support posture for this normalized record.
    pub support_class: DiagnosticSupportClass,
    /// Export-safe message reference, not raw display text.
    pub message_ref: String,
    /// Export-safe detail reference, not raw source content.
    pub detail_ref: Option<String>,
    /// Suppression refs affecting this finding.
    pub suppression_refs: Vec<String>,
    /// Baseline refs affecting this finding.
    pub baseline_refs: Vec<String>,
    /// Causal links explaining what emitted or preserved this finding.
    pub causal_links: Vec<DiagnosticCausalLink>,
    /// Cross-surface refs for this diagnostic.
    pub surface_refs: DiagnosticSurfaceRefs,
    /// Redaction posture for this record.
    pub redaction_class: DiagnosticRedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl DiagnosticRecord {
    /// Builds a canonical diagnostic record with required record-kind and schema fields.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        diagnostic_id: impl Into<String>,
        rule_id_ref: impl Into<String>,
        category_ref: impl Into<String>,
        severity_class: DiagnosticSeverityClass,
        source: DiagnosticSource,
        freshness_class: DiagnosticFreshnessClass,
        anchor_remap: DiagnosticAnchorRemap,
        support_class: DiagnosticSupportClass,
        message_ref: impl Into<String>,
        surface_refs: DiagnosticSurfaceRefs,
        captured_at: impl Into<String>,
        export_safe_summary: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: DIAGNOSTIC_RECORD_KIND.to_owned(),
            diagnostic_record_schema_version: UNIFIED_DIAGNOSTIC_SCHEMA_VERSION,
            diagnostic_id: diagnostic_id.into(),
            rule_id_ref: rule_id_ref.into(),
            category_ref: category_ref.into(),
            severity_class,
            source,
            freshness_class,
            anchor_remap,
            support_class,
            message_ref: message_ref.into(),
            detail_ref: None,
            suppression_refs: Vec::new(),
            baseline_refs: Vec::new(),
            causal_links: Vec::new(),
            surface_refs,
            redaction_class: DiagnosticRedactionClass::MetadataSafeDefault,
            captured_at: captured_at.into(),
            export_safe_summary: export_safe_summary.into(),
        }
    }

    /// Builds a runtime diagnostic record from a language diagnostic envelope.
    pub fn from_language_envelope(
        envelope: &aureline_language::DiagnosticEnvelope,
        surface_refs: DiagnosticSurfaceRefs,
        causal_links: Vec<DiagnosticCausalLink>,
    ) -> Self {
        let source_kind =
            DiagnosticSourceKind::from_language_source_family(envelope.source.source_family);
        let origin_class =
            DiagnosticOriginClass::from_language_origin(envelope.source.origin_class);
        let mut source = DiagnosticSource::new(
            envelope.source.source_descriptor_id.clone(),
            source_kind,
            DiagnosticEvidencePlaneClass::from_language_plane(envelope.source.evidence_plane_class),
            origin_class,
            if origin_class.is_imported_or_replayed() {
                DiagnosticSourceConfidenceClass::ImportedAuthoritative
            } else {
                DiagnosticSourceConfidenceClass::Authoritative
            },
            DiagnosticSupportClass::from_language_support(envelope.source.support_class),
            envelope.source.producer_ref.clone(),
            envelope.source.producer_ref.clone(),
            envelope.source.producer_version_ref.clone(),
            envelope.source.summary.clone(),
        );
        source.adapter_ref = envelope.source.provider_id.clone();
        source.target_or_environment_ref = Some(envelope.scope.target_ref.clone());
        source.originating_session_ref = envelope.freshness.epoch_ref.clone();
        if origin_class == DiagnosticOriginClass::ImportedSnapshot {
            source.import_ref = envelope.freshness.epoch_ref.clone();
        }
        source.raw_payload_ref = envelope
            .evidence_refs
            .iter()
            .find(|evidence| {
                evidence.evidence_role_class
                    == aureline_language::DiagnosticEvidenceRoleClass::PrimarySource
            })
            .map(|evidence| evidence.evidence_ref.clone());

        let mut anchor_remap = DiagnosticAnchorRemap::new(
            format!("remap:{}", envelope.anchor.anchor_family_id),
            envelope.anchor.anchor_family_id.clone(),
            envelope.anchor.current_anchor_ref.clone(),
            envelope.anchor.current_anchor_ref.clone(),
            DiagnosticAnchorRemapStateClass::from_language_remap(envelope.anchor.remap_state_class),
            envelope
                .freshness
                .epoch_ref
                .clone()
                .unwrap_or_else(|| envelope.collection_id.clone()),
            envelope.captured_at.clone(),
            envelope.anchor.summary.clone(),
        );
        anchor_remap.current_revision_ref = envelope.freshness.epoch_ref.clone();

        Self {
            record_kind: DIAGNOSTIC_RECORD_KIND.to_owned(),
            diagnostic_record_schema_version: UNIFIED_DIAGNOSTIC_SCHEMA_VERSION,
            diagnostic_id: envelope.diagnostic_id.clone(),
            rule_id_ref: envelope
                .rule_id_ref
                .clone()
                .unwrap_or_else(|| "rule:unknown_requires_review".to_owned()),
            category_ref: envelope
                .category_ref
                .clone()
                .unwrap_or_else(|| "category:unknown_requires_review".to_owned()),
            severity_class: DiagnosticSeverityClass::from_language_severity(
                envelope.severity_class,
            ),
            source,
            freshness_class: DiagnosticFreshnessClass::from_language_freshness(
                envelope.freshness.freshness_class,
            ),
            anchor_remap,
            support_class: DiagnosticSupportClass::from_language_support(
                envelope.source.support_class,
            ),
            message_ref: format!("message:{}", envelope.diagnostic_id),
            detail_ref: None,
            suppression_refs: Vec::new(),
            baseline_refs: Vec::new(),
            causal_links,
            surface_refs,
            redaction_class: DiagnosticRedactionClass::from_language_redaction(
                envelope.redaction_class,
            ),
            captured_at: envelope.captured_at.clone(),
            export_safe_summary: envelope.export_safe_summary.clone(),
        }
    }

    /// Builds a runtime diagnostic record from a task-event diagnostic payload.
    pub fn from_task_event(
        event: &TaskEvent,
        rule_id_ref: impl Into<String>,
        category_ref: impl Into<String>,
        anchor_remap: DiagnosticAnchorRemap,
        surface_refs: DiagnosticSurfaceRefs,
    ) -> Option<Self> {
        let TaskEventPayload::Diagnostic {
            diagnostic_ref,
            severity,
            tool_ref,
        } = &event.payload
        else {
            return None;
        };

        let source_kind = DiagnosticSourceKind::from_task_wedge(event.identity.wedge);
        let evidence_plane_class = match source_kind {
            DiagnosticSourceKind::RuntimeOrTest => {
                DiagnosticEvidencePlaneClass::RuntimeOrTestExecution
            }
            DiagnosticSourceKind::Heuristic => DiagnosticEvidencePlaneClass::HeuristicFallback,
            DiagnosticSourceKind::Policy => DiagnosticEvidencePlaneClass::PolicyOrTrustEvaluation,
            _ => DiagnosticEvidencePlaneClass::BuildTimeExecution,
        };
        let confidence_class = if source_kind == DiagnosticSourceKind::Heuristic {
            DiagnosticSourceConfidenceClass::HeuristicParsed
        } else {
            DiagnosticSourceConfidenceClass::DerivedStructured
        };
        let mut source = DiagnosticSource::new(
            format!("source:task:{}", event.event_id),
            source_kind,
            evidence_plane_class,
            DiagnosticOriginClass::LiveLocalSession,
            confidence_class,
            DiagnosticSupportClass::Authoritative,
            event.provenance.source_adapter_id.clone(),
            tool_ref.clone(),
            Some(event.provenance.adapter_version.clone()),
            event.summary.clone(),
        );
        source.adapter_ref = Some(event.provenance.source_adapter_id.clone());
        source.adapter_version_ref = Some(event.provenance.adapter_version.clone());
        source.target_or_environment_ref = Some(event.identity.target_id.clone());
        source.originating_session_ref = Some(event.identity.execution_context_id.clone());
        source.run_ref = Some(event.identity.run_id.clone());
        source.task_ref = Some(event.identity.task_id.clone());
        source.raw_payload_ref = Some(event.raw_envelope.raw_envelope_ref.clone());

        Some(Self {
            record_kind: DIAGNOSTIC_RECORD_KIND.to_owned(),
            diagnostic_record_schema_version: UNIFIED_DIAGNOSTIC_SCHEMA_VERSION,
            diagnostic_id: diagnostic_ref.clone(),
            rule_id_ref: rule_id_ref.into(),
            category_ref: category_ref.into(),
            severity_class: DiagnosticSeverityClass::from_task_severity(*severity),
            source,
            freshness_class: DiagnosticFreshnessClass::Current,
            anchor_remap,
            support_class: DiagnosticSupportClass::Authoritative,
            message_ref: format!("message:{}", diagnostic_ref),
            detail_ref: None,
            suppression_refs: Vec::new(),
            baseline_refs: Vec::new(),
            causal_links: vec![
                DiagnosticCausalLink::new(
                    DiagnosticCausalLinkKind::Task,
                    event.identity.task_id.clone(),
                    "Task emitted the diagnostic.",
                ),
                DiagnosticCausalLink::new(
                    DiagnosticCausalLinkKind::Run,
                    event.identity.run_id.clone(),
                    "Run emitted the diagnostic.",
                ),
                DiagnosticCausalLink::new(
                    DiagnosticCausalLinkKind::OutputEntry,
                    event.event_id.clone(),
                    "Task event carries the diagnostic payload.",
                ),
            ],
            surface_refs,
            redaction_class: DiagnosticRedactionClass::MetadataSafeDefault,
            captured_at: event.occurred_at.clone(),
            export_safe_summary: event.summary.clone(),
        })
    }

    /// Returns true when this diagnostic must disclose source or freshness labels.
    pub fn requires_disclosure(&self) -> bool {
        self.freshness_class.requires_disclosure()
            || self.anchor_remap.requires_disclosure()
            || self.source.origin_class.is_imported_or_replayed()
            || self.source.confidence_class != DiagnosticSourceConfidenceClass::Authoritative
    }

    /// Returns true when the record has stable identity and provenance metadata.
    pub fn can_emit_beta_source(&self) -> bool {
        self.record_kind == DIAGNOSTIC_RECORD_KIND
            && self.diagnostic_record_schema_version == UNIFIED_DIAGNOSTIC_SCHEMA_VERSION
            && !self.diagnostic_id.is_empty()
            && !self.rule_id_ref.is_empty()
            && !self.category_ref.is_empty()
            && !self.message_ref.is_empty()
            && self.source.has_required_provenance()
            && !self.causal_links.is_empty()
    }

    /// Returns a projection for one surface.
    pub fn surface_projection(
        &self,
        surface_class: DiagnosticSurfaceClass,
    ) -> DiagnosticSurfaceProjection {
        let stable_surface_ref = self.surface_refs.ref_for_surface(surface_class).to_owned();
        let open_origin_ref = if surface_class == DiagnosticSurfaceClass::Problems {
            self.source.origin_ref().map(str::to_owned).or_else(|| {
                self.causal_links
                    .first()
                    .map(|link| link.causal_ref.clone())
            })
        } else {
            None
        };
        let disclosure_labels = self.disclosure_labels();

        DiagnosticSurfaceProjection {
            record_kind: DIAGNOSTIC_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
            diagnostic_record_schema_version: UNIFIED_DIAGNOSTIC_SCHEMA_VERSION,
            projection_id: format!(
                "diagnostic_projection:{}:{}",
                surface_class.as_str(),
                sanitize_id(&self.diagnostic_id)
            ),
            diagnostic_id: self.diagnostic_id.clone(),
            stable_surface_ref,
            surface_class,
            source_kind: self.source.source_kind,
            freshness_class: self.freshness_class,
            remap_state_class: self.anchor_remap.remap_state_class,
            source_id: self.source.source_id.clone(),
            origin_ref: self.source.origin_ref().map(str::to_owned),
            open_origin_ref,
            output_entry_ref: Some(self.surface_refs.output_entry_ref.clone()),
            timeline_entry_ref: Some(self.surface_refs.timeline_entry_ref.clone()),
            rerun_action_ref: Some(self.surface_refs.rerun_action_ref.clone()),
            review_packet_ref: Some(self.surface_refs.review_packet_ref.clone()),
            cli_explain_ref: Some(self.surface_refs.cli_explain_ref.clone()),
            support_export_ref: Some(self.surface_refs.support_export_ref.clone()),
            ai_evidence_ref: Some(self.surface_refs.ai_evidence_ref.clone()),
            disclosure_labels,
            raw_source_content_included: false,
            raw_payload_included: false,
            export_safe_summary: format!(
                "{} projection for {} preserves diagnostic id {}.",
                surface_class.as_str(),
                self.source.source_kind.as_str(),
                self.diagnostic_id
            ),
        }
    }

    fn disclosure_labels(&self) -> Vec<String> {
        let mut labels = BTreeSet::new();
        labels.insert(self.source.source_kind.as_str().to_owned());
        labels.insert(self.freshness_class.as_str().to_owned());
        labels.insert(self.anchor_remap.remap_state_class.as_str().to_owned());
        if self.source.origin_class.is_imported_or_replayed() || self.freshness_class.is_imported()
        {
            labels.insert("imported".to_owned());
        }
        if self.freshness_class.requires_disclosure() {
            labels.insert("freshness_disclosed".to_owned());
        }
        if self.anchor_remap.requires_disclosure() {
            labels.insert("remap_disclosed".to_owned());
        }
        labels.into_iter().collect()
    }
}

/// Projection of one diagnostic record into one consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticSurfaceProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub diagnostic_record_schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// Canonical diagnostic id reused by every surface.
    pub diagnostic_id: String,
    /// Stable surface-local ref that cites the diagnostic id.
    pub stable_surface_ref: String,
    /// Surface consuming the projection.
    pub surface_class: DiagnosticSurfaceClass,
    /// Source kind copied from the diagnostic record.
    pub source_kind: DiagnosticSourceKind,
    /// Freshness copied from the diagnostic record.
    pub freshness_class: DiagnosticFreshnessClass,
    /// Remap state copied from the diagnostic record.
    pub remap_state_class: DiagnosticAnchorRemapStateClass,
    /// Source descriptor id copied from the diagnostic record.
    pub source_id: String,
    /// Origin ref users can inspect from durable surfaces.
    pub origin_ref: Option<String>,
    /// Problems-row open origin ref.
    pub open_origin_ref: Option<String>,
    /// Output entry ref associated with the diagnostic.
    pub output_entry_ref: Option<String>,
    /// Timeline entry ref associated with the diagnostic.
    pub timeline_entry_ref: Option<String>,
    /// Rerun action ref associated with the diagnostic.
    pub rerun_action_ref: Option<String>,
    /// Review packet ref associated with the diagnostic.
    pub review_packet_ref: Option<String>,
    /// CLI explain ref associated with the diagnostic.
    pub cli_explain_ref: Option<String>,
    /// Support export ref associated with the diagnostic.
    pub support_export_ref: Option<String>,
    /// AI evidence ref associated with the diagnostic.
    pub ai_evidence_ref: Option<String>,
    /// Labels that must survive compact rendering.
    pub disclosure_labels: Vec<String>,
    /// Whether raw source content is included in this projection.
    pub raw_source_content_included: bool,
    /// Whether raw payload content is included in this projection.
    pub raw_payload_included: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

/// Display cluster that preserves distinct contributing diagnostic truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnifiedDiagnosticCluster {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub diagnostic_record_schema_version: u32,
    /// Stable cluster id.
    pub cluster_id: String,
    /// Primary diagnostic id for compact rendering.
    pub primary_diagnostic_id: String,
    /// Diagnostic ids contributing to this cluster.
    pub contributing_diagnostic_ids: Vec<String>,
    /// Dedupe or clustering reason.
    pub dedupe_reason_ref: String,
    /// Source kinds preserved after clustering.
    pub preserved_source_kinds: Vec<DiagnosticSourceKind>,
    /// Freshness classes preserved after clustering.
    pub preserved_freshness_classes: Vec<DiagnosticFreshnessClass>,
    /// Remap states preserved after clustering.
    pub preserved_remap_states: Vec<DiagnosticAnchorRemapStateClass>,
    /// Whether imported, stale, or remapped disclosure remains required.
    pub disclosure_required: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl UnifiedDiagnosticCluster {
    /// Builds a cluster from contributing records while preserving truth labels.
    pub fn from_records(
        cluster_id: impl Into<String>,
        primary_diagnostic_id: impl Into<String>,
        dedupe_reason_ref: impl Into<String>,
        records: &[DiagnosticRecord],
        export_safe_summary: impl Into<String>,
    ) -> Self {
        let preserved_source_kinds = records
            .iter()
            .map(|record| record.source.source_kind)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let preserved_freshness_classes = records
            .iter()
            .map(|record| record.freshness_class)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let preserved_remap_states = records
            .iter()
            .map(|record| record.anchor_remap.remap_state_class)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let disclosure_required = records.iter().any(DiagnosticRecord::requires_disclosure);

        Self {
            record_kind: UNIFIED_DIAGNOSTIC_CLUSTER_RECORD_KIND.to_owned(),
            diagnostic_record_schema_version: UNIFIED_DIAGNOSTIC_SCHEMA_VERSION,
            cluster_id: cluster_id.into(),
            primary_diagnostic_id: primary_diagnostic_id.into(),
            contributing_diagnostic_ids: records
                .iter()
                .map(|record| record.diagnostic_id.clone())
                .collect(),
            dedupe_reason_ref: dedupe_reason_ref.into(),
            preserved_source_kinds,
            preserved_freshness_classes,
            preserved_remap_states,
            disclosure_required,
            export_safe_summary: export_safe_summary.into(),
        }
    }
}

/// Support-export packet that references diagnostics without copying raw source content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticSupportExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub diagnostic_record_schema_version: u32,
    /// Stable support export id.
    pub export_id: String,
    /// Workspace id covered by the export.
    pub workspace_id: String,
    /// Diagnostic ids cited by the export.
    pub diagnostic_record_refs: Vec<String>,
    /// Source refs cited by the export.
    pub source_refs: Vec<String>,
    /// Remap refs cited by the export.
    pub remap_refs: Vec<String>,
    /// Redaction posture for the export.
    pub redaction_class: DiagnosticRedactionClass,
    /// Whether raw source content is included by default.
    pub raw_source_content_included: bool,
    /// Whether raw payload content is included by default.
    pub raw_payload_included: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl DiagnosticSupportExport {
    /// Builds the default metadata-only support export for a diagnostic set.
    pub fn from_records(
        export_id: impl Into<String>,
        workspace_id: impl Into<String>,
        records: &[DiagnosticRecord],
    ) -> Self {
        Self {
            record_kind: DIAGNOSTIC_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            diagnostic_record_schema_version: UNIFIED_DIAGNOSTIC_SCHEMA_VERSION,
            export_id: export_id.into(),
            workspace_id: workspace_id.into(),
            diagnostic_record_refs: records
                .iter()
                .map(|record| record.diagnostic_id.clone())
                .collect(),
            source_refs: records
                .iter()
                .map(|record| record.source.source_id.clone())
                .collect(),
            remap_refs: records
                .iter()
                .map(|record| record.anchor_remap.remap_id.clone())
                .collect(),
            redaction_class: DiagnosticRedactionClass::MetadataSafeDefault,
            raw_source_content_included: false,
            raw_payload_included: false,
            export_safe_summary:
                "Support export cites diagnostic records by id with raw content omitted by default."
                    .to_owned(),
        }
    }
}

/// AI evidence packet reference that cites diagnostics without copying raw content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticAiEvidenceReferencePacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub diagnostic_record_schema_version: u32,
    /// Stable AI evidence packet id.
    pub evidence_packet_id: String,
    /// Diagnostic ids cited by AI evidence.
    pub diagnostic_record_refs: Vec<String>,
    /// Redaction posture for the packet.
    pub redaction_class: DiagnosticRedactionClass,
    /// Whether raw source content is included by default.
    pub raw_source_content_included: bool,
    /// Whether raw payload content is included by default.
    pub raw_payload_included: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl DiagnosticAiEvidenceReferencePacket {
    /// Builds the default metadata-only AI evidence reference packet.
    pub fn from_records(
        evidence_packet_id: impl Into<String>,
        records: &[DiagnosticRecord],
    ) -> Self {
        Self {
            record_kind: DIAGNOSTIC_AI_EVIDENCE_RECORD_KIND.to_owned(),
            diagnostic_record_schema_version: UNIFIED_DIAGNOSTIC_SCHEMA_VERSION,
            evidence_packet_id: evidence_packet_id.into(),
            diagnostic_record_refs: records
                .iter()
                .map(|record| record.diagnostic_id.clone())
                .collect(),
            redaction_class: DiagnosticRedactionClass::MetadataSafeDefault,
            raw_source_content_included: false,
            raw_payload_included: false,
            export_safe_summary:
                "AI evidence cites diagnostic records by id with raw content omitted by default."
                    .to_owned(),
        }
    }
}

/// Snapshot of the unified diagnostic plane for one workspace or workset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnifiedDiagnosticPlaneSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub diagnostic_record_schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Workspace id covered by the snapshot.
    pub workspace_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Canonical diagnostic records.
    pub diagnostics: Vec<DiagnosticRecord>,
    /// Surface projections for canonical records.
    pub surface_projections: Vec<DiagnosticSurfaceProjection>,
    /// Clusters derived from canonical records.
    pub clusters: Vec<UnifiedDiagnosticCluster>,
    /// Default support export for this snapshot.
    pub support_export: DiagnosticSupportExport,
    /// Default AI evidence references for this snapshot.
    pub ai_evidence: DiagnosticAiEvidenceReferencePacket,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl UnifiedDiagnosticPlaneSnapshot {
    /// Builds a snapshot and generates required surface projections.
    pub fn from_records(
        snapshot_id: impl Into<String>,
        workspace_id: impl Into<String>,
        captured_at: impl Into<String>,
        diagnostics: Vec<DiagnosticRecord>,
        clusters: Vec<UnifiedDiagnosticCluster>,
    ) -> Self {
        let snapshot_id = snapshot_id.into();
        let workspace_id = workspace_id.into();
        let captured_at = captured_at.into();
        let surface_projections = diagnostics
            .iter()
            .flat_map(|diagnostic| {
                DiagnosticSurfaceClass::REQUIRED
                    .into_iter()
                    .map(|surface| diagnostic.surface_projection(surface))
            })
            .collect::<Vec<_>>();
        let support_export = DiagnosticSupportExport::from_records(
            format!("support_export:{}", sanitize_id(&snapshot_id)),
            workspace_id.clone(),
            &diagnostics,
        );
        let ai_evidence = DiagnosticAiEvidenceReferencePacket::from_records(
            format!("ai_evidence:{}", sanitize_id(&snapshot_id)),
            &diagnostics,
        );
        let diagnostic_count = diagnostics.len();

        Self {
            record_kind: UNIFIED_DIAGNOSTIC_PLANE_SNAPSHOT_RECORD_KIND.to_owned(),
            diagnostic_record_schema_version: UNIFIED_DIAGNOSTIC_SCHEMA_VERSION,
            snapshot_id,
            workspace_id,
            captured_at,
            diagnostics,
            surface_projections,
            clusters,
            support_export,
            ai_evidence,
            export_safe_summary: format!(
                "Unified diagnostic plane snapshot contains {diagnostic_count} canonical diagnostics."
            ),
        }
    }

    /// Validates cross-surface identity, source provenance, clustering, and export defaults.
    pub fn validate(&self) -> DiagnosticPlaneValidationReport {
        let mut violations = Vec::new();
        let diagnostic_by_id = self
            .diagnostics
            .iter()
            .map(|diagnostic| (diagnostic.diagnostic_id.as_str(), diagnostic))
            .collect::<BTreeMap<_, _>>();

        for diagnostic in &self.diagnostics {
            if !diagnostic.can_emit_beta_source() {
                violations.push(DiagnosticPlaneViolation::DisplayTextOnlyDiagnostic {
                    diagnostic_id: diagnostic.diagnostic_id.clone(),
                });
            }

            for surface in DiagnosticSurfaceClass::REQUIRED {
                let matching = self.surface_projections.iter().find(|projection| {
                    projection.diagnostic_id == diagnostic.diagnostic_id
                        && projection.surface_class == surface
                });
                match matching {
                    Some(projection) => {
                        if projection.source_kind != diagnostic.source.source_kind
                            || projection.freshness_class != diagnostic.freshness_class
                            || projection.remap_state_class
                                != diagnostic.anchor_remap.remap_state_class
                        {
                            violations.push(DiagnosticPlaneViolation::ProjectionTruthMismatch {
                                diagnostic_id: diagnostic.diagnostic_id.clone(),
                                surface_class: surface,
                            });
                        }
                        if surface == DiagnosticSurfaceClass::Problems
                            && projection.open_origin_ref.is_none()
                        {
                            violations.push(DiagnosticPlaneViolation::ProblemsOriginMissing {
                                diagnostic_id: diagnostic.diagnostic_id.clone(),
                            });
                        }
                        if projection.raw_source_content_included || projection.raw_payload_included
                        {
                            violations.push(
                                DiagnosticPlaneViolation::RawContentInDefaultProjection {
                                    diagnostic_id: diagnostic.diagnostic_id.clone(),
                                    surface_class: surface,
                                },
                            );
                        }
                    }
                    None => violations.push(DiagnosticPlaneViolation::MissingSurfaceProjection {
                        diagnostic_id: diagnostic.diagnostic_id.clone(),
                        surface_class: surface,
                    }),
                }
            }
        }

        for cluster in &self.clusters {
            for diagnostic_id in &cluster.contributing_diagnostic_ids {
                let Some(record) = diagnostic_by_id.get(diagnostic_id.as_str()) else {
                    violations.push(
                        DiagnosticPlaneViolation::ClusterReferencesUnknownDiagnostic {
                            cluster_id: cluster.cluster_id.clone(),
                            diagnostic_id: diagnostic_id.clone(),
                        },
                    );
                    continue;
                };
                if !cluster
                    .preserved_source_kinds
                    .contains(&record.source.source_kind)
                    || !cluster
                        .preserved_freshness_classes
                        .contains(&record.freshness_class)
                    || !cluster
                        .preserved_remap_states
                        .contains(&record.anchor_remap.remap_state_class)
                {
                    violations.push(DiagnosticPlaneViolation::ClusterDroppedTruthLabel {
                        cluster_id: cluster.cluster_id.clone(),
                        diagnostic_id: diagnostic_id.clone(),
                    });
                }
            }
        }

        for diagnostic in &self.diagnostics {
            if !self
                .support_export
                .diagnostic_record_refs
                .contains(&diagnostic.diagnostic_id)
            {
                violations.push(DiagnosticPlaneViolation::SupportExportMissingDiagnostic {
                    diagnostic_id: diagnostic.diagnostic_id.clone(),
                });
            }
            if !self
                .ai_evidence
                .diagnostic_record_refs
                .contains(&diagnostic.diagnostic_id)
            {
                violations.push(DiagnosticPlaneViolation::AiEvidenceMissingDiagnostic {
                    diagnostic_id: diagnostic.diagnostic_id.clone(),
                });
            }
        }

        if self.support_export.raw_source_content_included
            || self.support_export.raw_payload_included
        {
            violations.push(DiagnosticPlaneViolation::SupportExportIncludesRawContent);
        }
        if self.ai_evidence.raw_source_content_included || self.ai_evidence.raw_payload_included {
            violations.push(DiagnosticPlaneViolation::AiEvidenceIncludesRawContent);
        }

        DiagnosticPlaneValidationReport { violations }
    }
}

/// Validation report for a unified diagnostic plane snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticPlaneValidationReport {
    /// Violations found during validation.
    pub violations: Vec<DiagnosticPlaneViolation>,
}

impl DiagnosticPlaneValidationReport {
    /// Returns true when validation found no violations.
    pub fn is_conformant(&self) -> bool {
        self.violations.is_empty()
    }
}

/// Validation failure for diagnostic plane conformance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "violation_kind")]
pub enum DiagnosticPlaneViolation {
    /// A beta-claimed source lacks stable id, rule, source, tool version, origin, or causal links.
    DisplayTextOnlyDiagnostic {
        /// Diagnostic id that failed validation.
        diagnostic_id: String,
    },
    /// A required surface projection is missing.
    MissingSurfaceProjection {
        /// Diagnostic id whose projection is missing.
        diagnostic_id: String,
        /// Missing surface.
        surface_class: DiagnosticSurfaceClass,
    },
    /// A projection changed source, freshness, or remap truth.
    ProjectionTruthMismatch {
        /// Diagnostic id with mismatched projection truth.
        diagnostic_id: String,
        /// Surface with the mismatch.
        surface_class: DiagnosticSurfaceClass,
    },
    /// A Problems row cannot open its origin task, run, adapter, policy, or import session.
    ProblemsOriginMissing {
        /// Diagnostic id whose Problems projection lacks an origin ref.
        diagnostic_id: String,
    },
    /// A projection includes raw source or raw payload content by default.
    RawContentInDefaultProjection {
        /// Diagnostic id whose projection includes raw content.
        diagnostic_id: String,
        /// Surface that includes raw content.
        surface_class: DiagnosticSurfaceClass,
    },
    /// A cluster references a diagnostic not present in the snapshot.
    ClusterReferencesUnknownDiagnostic {
        /// Cluster id with the unknown reference.
        cluster_id: String,
        /// Missing diagnostic id.
        diagnostic_id: String,
    },
    /// A cluster dropped a source, freshness, or remap label from a contributing diagnostic.
    ClusterDroppedTruthLabel {
        /// Cluster id that dropped a truth label.
        cluster_id: String,
        /// Diagnostic id whose label was dropped.
        diagnostic_id: String,
    },
    /// Support export does not cite a diagnostic record.
    SupportExportMissingDiagnostic {
        /// Diagnostic id missing from support export.
        diagnostic_id: String,
    },
    /// AI evidence does not cite a diagnostic record.
    AiEvidenceMissingDiagnostic {
        /// Diagnostic id missing from AI evidence.
        diagnostic_id: String,
    },
    /// Support export includes raw source or raw payload content by default.
    SupportExportIncludesRawContent,
    /// AI evidence includes raw source or raw payload content by default.
    AiEvidenceIncludesRawContent,
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}
