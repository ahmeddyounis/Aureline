//! Provider, diagnostic-cluster, and refactor-transaction matrix truth packet.
//!
//! This module is the language-owned contract that freezes the matrix
//! mapping each new code-understanding artifact family — framework
//! packs, notebook cells, generated source, structured artifacts, and
//! the code-understanding graph — to the language-provider family,
//! capability-negotiation outcome, provider-conflict class, diagnostic
//! source class, result-provenance state, semantic-layer mode,
//! refactor-transaction class, preview-completeness label,
//! generated-artifact policy, allowed downgrade label, and rollback
//! posture it may claim. The matrix is the single truth that the
//! framework-pack panel, notebook surface, request runner, preview
//! surface, docs surface, generated-artifact surface, support export,
//! release proof index, Help/About proof card, and the conformance
//! dashboard all read. Surfaces MUST NOT mint local copies or
//! paraphrase matrix posture; they read this packet verbatim.
//!
//! The matrix extends — it does not redefine — the launch-language
//! refactor transaction model. Where a row asserts a launch-language
//! refactor class it carries the same preview, completeness, and
//! rollback discipline the refactor transaction packet already pins;
//! the M5 matrix only adds framework, notebook, generated, and
//! structured-artifact rows on top, never weakening the existing
//! safety model. The crosswalk to the refactor transaction contract is
//! recorded in the reviewer doc.
//!
//! Every row binds a closed `artifact_family_lane_class`,
//! `matrix_row_class`, `support_class`, `provider_family_class`,
//! `capability_negotiation_class`, `conflict_class`,
//! `diagnostic_source_class`, `result_provenance_class`,
//! `semantic_layer_mode_class`, `refactor_transaction_class`,
//! `completeness_class`, `generated_artifact_policy_class`,
//! `downgrade_label_class`, `rollback_path_class`, `evidence_class`,
//! `known_limit_class`, `downgrade_automation_class`, and
//! `confidence_class` plus an `evidence_refs` array and a
//! `disclosure_ref` whenever the row is narrowed below certified,
//! declares a non-`none_declared` known limit, or binds a non-`none`
//! downgrade automation.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! source bodies, refactor diffs, generated artifact bodies, notebook
//! cell outputs, provider payloads, secrets, ambient credentials, or
//! any other private material past the boundary. A row that claims
//! `certified` while leaving a required binding unbound is refused; the
//! validator narrows below certified instead of inheriting an adjacent
//! certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`ProviderRefactorMatrixTruthPacket`].
pub const PROVIDER_REFACTOR_MATRIX_TRUTH_PACKET_RECORD_KIND: &str =
    "provider_refactor_matrix_truth_stable_packet";

/// Stable record-kind tag for [`ProviderRefactorMatrixTruthSupportExport`].
pub const PROVIDER_REFACTOR_MATRIX_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "provider_refactor_matrix_truth_support_export";

/// Integer schema version for the provider/refactor matrix truth packet.
pub const PROVIDER_REFACTOR_MATRIX_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PROVIDER_REFACTOR_MATRIX_TRUTH_SCHEMA_REF: &str =
    "schemas/language/provider_refactor_matrix_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const PROVIDER_REFACTOR_MATRIX_TRUTH_DOC_REF: &str =
    "docs/m5/freeze-the-language-provider-diagnostic-cluster-and-refactor-transaction-matrix.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const PROVIDER_REFACTOR_MATRIX_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m5/freeze-the-language-provider-diagnostic-cluster-and-refactor-transaction-matrix.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const PROVIDER_REFACTOR_MATRIX_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m5/provider_refactor_matrix_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const PROVIDER_REFACTOR_MATRIX_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m5/provider_refactor_matrix_truth_packet.json";

/// Closed artifact-family lane vocabulary. Every required lane MUST
/// have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactFamilyLaneClass {
    /// Framework analyzer / framework-pack lane.
    FrameworkPackLane,
    /// Notebook-aware cell semantics lane.
    NotebookCellLane,
    /// Generated / scaffolded source bridge lane.
    GeneratedSourceLane,
    /// Structured API / infra / preview artifact lane.
    StructuredArtifactLane,
    /// Code-understanding semantic-graph lane.
    CodeUnderstandingGraphLane,
}

impl ArtifactFamilyLaneClass {
    /// Every required artifact-family lane, in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::FrameworkPackLane,
        Self::NotebookCellLane,
        Self::GeneratedSourceLane,
        Self::StructuredArtifactLane,
        Self::CodeUnderstandingGraphLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkPackLane => "framework_pack_lane",
            Self::NotebookCellLane => "notebook_cell_lane",
            Self::GeneratedSourceLane => "generated_source_lane",
            Self::StructuredArtifactLane => "structured_artifact_lane",
            Self::CodeUnderstandingGraphLane => "code_understanding_graph_lane",
        }
    }
}

/// Closed matrix-row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixRowClass {
    /// The lane's headline qualification row binding provider family and support.
    MatrixLaneQuality,
    /// Capability-negotiation admission row binding one negotiation outcome.
    CapabilityNegotiationAdmission,
    /// Conflict-arbitration admission row binding one conflict class.
    ConflictArbitrationAdmission,
    /// Diagnostic-source admission row binding one diagnostic source class.
    DiagnosticSourceAdmission,
    /// Result-provenance admission row binding one provenance class.
    ResultProvenanceAdmission,
    /// Semantic-layer mode admission row binding one semantic-layer mode.
    SemanticLayerModeAdmission,
    /// Refactor-transaction admission row binding refactor class, completeness, and rollback.
    RefactorTransactionAdmission,
    /// Generated-artifact policy admission row binding one generated-asset policy.
    GeneratedArtifactPolicyAdmission,
    /// Downgrade-label admission row binding one allowed downgrade label.
    DowngradeLabelAdmission,
    /// Precisely labeled unsupported-gap row on a lane.
    UnsupportedGap,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl MatrixRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MatrixLaneQuality => "matrix_lane_quality",
            Self::CapabilityNegotiationAdmission => "capability_negotiation_admission",
            Self::ConflictArbitrationAdmission => "conflict_arbitration_admission",
            Self::DiagnosticSourceAdmission => "diagnostic_source_admission",
            Self::ResultProvenanceAdmission => "result_provenance_admission",
            Self::SemanticLayerModeAdmission => "semantic_layer_mode_admission",
            Self::RefactorTransactionAdmission => "refactor_transaction_admission",
            Self::GeneratedArtifactPolicyAdmission => "generated_artifact_policy_admission",
            Self::DowngradeLabelAdmission => "downgrade_label_admission",
            Self::UnsupportedGap => "unsupported_gap",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when the row class must name a concrete acting provider family.
    pub const fn requires_provider_family(self) -> bool {
        matches!(
            self,
            Self::MatrixLaneQuality | Self::SemanticLayerModeAdmission
        )
    }
}

/// Closed support-class vocabulary applied to a matrix row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Row claims the M5 certified grade.
    Certified,
    /// Row is intentionally narrowed below certified; narrowing is disclosed.
    CertifiedBelow,
    /// Row is at beta-grade only.
    BetaGradeOnly,
    /// Row is at preview only.
    PreviewOnly,
    /// Row carries a precisely labeled unsupported gap.
    Unsupported,
    /// Row has no bound support class; this never qualifies certified.
    SupportUnbound,
}

impl SupportClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::CertifiedBelow => "certified_below",
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
        !matches!(self, Self::Certified)
    }
}

/// Closed language-provider family vocabulary. Aureline MUST NOT present
/// these as interchangeable; the acting family stays inspectable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderFamilyClass {
    /// Language-server (LSP) provider.
    LspProvider,
    /// Framework analyzer pack provider.
    FrameworkAnalyzer,
    /// Semantic code-understanding graph lane.
    SemanticGraphLane,
    /// Notebook adapter provider.
    NotebookAdapter,
    /// Generated-source bridge provider.
    GeneratedSourceBridge,
    /// AI overlay provider.
    AiOverlay,
    /// Text/heuristic fallback provider.
    TextFallback,
    /// Row does not name an acting provider family.
    NotApplicable,
    /// Row has no bound provider family; this never qualifies certified.
    ProviderUnbound,
}

impl ProviderFamilyClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LspProvider => "lsp_provider",
            Self::FrameworkAnalyzer => "framework_analyzer",
            Self::SemanticGraphLane => "semantic_graph_lane",
            Self::NotebookAdapter => "notebook_adapter",
            Self::GeneratedSourceBridge => "generated_source_bridge",
            Self::AiOverlay => "ai_overlay",
            Self::TextFallback => "text_fallback",
            Self::NotApplicable => "not_applicable",
            Self::ProviderUnbound => "provider_unbound",
        }
    }

    /// True when this provider family is bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::ProviderUnbound)
    }

    /// True when this provider family names a concrete acting provider.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::ProviderUnbound)
    }
}

/// Closed capability-negotiation outcome vocabulary. A
/// `capability_negotiation_admission` row binds exactly one outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityNegotiationClass {
    /// Full semantic capability negotiated.
    FullSemanticNegotiated,
    /// Partial semantic capability negotiated.
    PartialSemanticNegotiated,
    /// Only text fallback negotiated.
    TextFallbackNegotiated,
    /// The provider declined the requested capability.
    CapabilityDeclined,
    /// Negotiation is pending provider readiness.
    NegotiationPending,
    /// Row is not a capability-negotiation admission row.
    NotApplicable,
    /// Row has no bound negotiation outcome; this never qualifies certified
    /// for a row class that requires a binding.
    NegotiationUnbound,
}

impl CapabilityNegotiationClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSemanticNegotiated => "full_semantic_negotiated",
            Self::PartialSemanticNegotiated => "partial_semantic_negotiated",
            Self::TextFallbackNegotiated => "text_fallback_negotiated",
            Self::CapabilityDeclined => "capability_declined",
            Self::NegotiationPending => "negotiation_pending",
            Self::NotApplicable => "not_applicable",
            Self::NegotiationUnbound => "negotiation_unbound",
        }
    }

    /// True when this negotiation class is a concrete, bound outcome.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::NegotiationUnbound)
    }

    /// True when this negotiation class is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::NegotiationUnbound)
    }
}

/// Closed provider-conflict vocabulary. A `conflict_arbitration_admission`
/// row binds exactly one conflict class. The losing provider and the
/// downgrade reason stay inspectable; conflict is never collapsed to a
/// ranking-only result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictClass {
    /// A single provider answered; no conflict exists.
    SingleProviderNoConflict,
    /// Providers disagreed; the winner was arbitrated and the loser is preserved.
    ArbitratedWinnerLoserPreserved,
    /// Providers disagreed and the disagreement is surfaced unresolved.
    UnresolvedDisagreementSurfaced,
    /// A policy/trust override decided the result and is recorded.
    PolicyOverrideRecorded,
    /// Row is not a conflict-arbitration admission row.
    NotApplicable,
    /// Row has no bound conflict class; this never qualifies certified
    /// for a row class that requires a binding.
    ConflictUnbound,
}

impl ConflictClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleProviderNoConflict => "single_provider_no_conflict",
            Self::ArbitratedWinnerLoserPreserved => "arbitrated_winner_loser_preserved",
            Self::UnresolvedDisagreementSurfaced => "unresolved_disagreement_surfaced",
            Self::PolicyOverrideRecorded => "policy_override_recorded",
            Self::NotApplicable => "not_applicable",
            Self::ConflictUnbound => "conflict_unbound",
        }
    }

    /// True when this conflict class is a concrete, bound outcome.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::ConflictUnbound)
    }

    /// True when this conflict class is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::ConflictUnbound)
    }
}

/// Closed diagnostic-source vocabulary. A `diagnostic_source_admission`
/// row binds exactly one diagnostic source class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSourceClass {
    /// Compiler / build diagnostics.
    CompilerBuild,
    /// Language-server diagnostics.
    Lsp,
    /// Linter / formatter diagnostics.
    LinterFormatter,
    /// Framework / schema diagnostics.
    FrameworkSchema,
    /// Runtime / test / debug diagnostics.
    RuntimeTestDebug,
    /// Policy / trust diagnostics.
    PolicyTrust,
    /// Generated-artifact validation diagnostics.
    GeneratedArtifactValidation,
    /// Notebook kernel diagnostics.
    NotebookKernel,
    /// Row is not a diagnostic-source admission row.
    NotApplicable,
    /// Row has no bound diagnostic source class; this never qualifies certified
    /// for a row class that requires a binding.
    SourceUnbound,
}

impl DiagnosticSourceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompilerBuild => "compiler_build",
            Self::Lsp => "lsp",
            Self::LinterFormatter => "linter_formatter",
            Self::FrameworkSchema => "framework_schema",
            Self::RuntimeTestDebug => "runtime_test_debug",
            Self::PolicyTrust => "policy_trust",
            Self::GeneratedArtifactValidation => "generated_artifact_validation",
            Self::NotebookKernel => "notebook_kernel",
            Self::NotApplicable => "not_applicable",
            Self::SourceUnbound => "source_unbound",
        }
    }

    /// True when this source class is a concrete, bound outcome.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::SourceUnbound)
    }

    /// True when this source class is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::SourceUnbound)
    }
}

/// Closed result-provenance vocabulary. A `result_provenance_admission`
/// row binds exactly one provenance class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultProvenanceClass {
    /// Live semantic result from a healthy provider.
    LiveSemantic,
    /// Cached semantic result.
    CachedSemantic,
    /// Partial semantic result with a visible label.
    PartialSemantic,
    /// Text/heuristic result, not semantic.
    TextHeuristic,
    /// Imported scan result.
    ImportedScan,
    /// Stale result pending refresh.
    StalePendingRefresh,
    /// Row is not a result-provenance admission row.
    NotApplicable,
    /// Row has no bound provenance class; this never qualifies certified
    /// for a row class that requires a binding.
    ProvenanceUnbound,
}

impl ResultProvenanceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveSemantic => "live_semantic",
            Self::CachedSemantic => "cached_semantic",
            Self::PartialSemantic => "partial_semantic",
            Self::TextHeuristic => "text_heuristic",
            Self::ImportedScan => "imported_scan",
            Self::StalePendingRefresh => "stale_pending_refresh",
            Self::NotApplicable => "not_applicable",
            Self::ProvenanceUnbound => "provenance_unbound",
        }
    }

    /// True when this provenance class is a concrete, bound outcome.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::ProvenanceUnbound)
    }

    /// True when this provenance class is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::ProvenanceUnbound)
    }
}

/// Closed semantic-layer mode vocabulary. A `semantic_layer_mode_admission`
/// row binds exactly one mode. This is the central matrix output: which
/// posture an artifact family may claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticLayerModeClass {
    /// Semantic rename across the symbol graph.
    SemanticRename,
    /// Previewable, typed refactor.
    PreviewableRefactor,
    /// Code-action mutation.
    CodeActionMutation,
    /// Text fallback only.
    TextFallback,
    /// Notebook / generated bridging mode.
    NotebookGeneratedBridge,
    /// Compare-only posture; no mutation.
    CompareOnly,
    /// The mode is unsupported on this lane.
    Unsupported,
    /// Row is not a semantic-layer mode admission row.
    NotApplicable,
    /// Row has no bound mode; this never qualifies certified
    /// for a row class that requires a binding.
    ModeUnbound,
}

impl SemanticLayerModeClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SemanticRename => "semantic_rename",
            Self::PreviewableRefactor => "previewable_refactor",
            Self::CodeActionMutation => "code_action_mutation",
            Self::TextFallback => "text_fallback",
            Self::NotebookGeneratedBridge => "notebook_generated_bridge",
            Self::CompareOnly => "compare_only",
            Self::Unsupported => "unsupported",
            Self::NotApplicable => "not_applicable",
            Self::ModeUnbound => "mode_unbound",
        }
    }

    /// True when this mode is a concrete, bound posture.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::ModeUnbound)
    }

    /// True when this mode is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::ModeUnbound)
    }

    /// True when this mode mutates source and so demands typed preview,
    /// completeness, and rollback discipline.
    pub const fn is_mutating(self) -> bool {
        matches!(
            self,
            Self::SemanticRename
                | Self::PreviewableRefactor
                | Self::CodeActionMutation
                | Self::NotebookGeneratedBridge
        )
    }
}

/// Closed refactor-transaction vocabulary. A `refactor_transaction_admission`
/// row binds exactly one refactor class plus its completeness and rollback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorTransactionClass {
    /// Symbol rename.
    Rename,
    /// Extract function / symbol.
    Extract,
    /// Inline symbol.
    Inline,
    /// Move symbol.
    Move,
    /// Organize imports.
    OrganizeImports,
    /// Schema / codegen rewrite.
    SchemaCodegenRewrite,
    /// AI-planned transform.
    AiPlannedTransform,
    /// Notebook / generated edit.
    NotebookGeneratedEdit,
    /// Compare-only; no mutation.
    CompareOnlyNoMutation,
    /// Row is not a refactor-transaction admission row.
    NotApplicable,
    /// Row has no bound refactor class; this never qualifies certified
    /// for a row class that requires a binding.
    RefactorUnbound,
}

impl RefactorTransactionClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rename => "rename",
            Self::Extract => "extract",
            Self::Inline => "inline",
            Self::Move => "move",
            Self::OrganizeImports => "organize_imports",
            Self::SchemaCodegenRewrite => "schema_codegen_rewrite",
            Self::AiPlannedTransform => "ai_planned_transform",
            Self::NotebookGeneratedEdit => "notebook_generated_edit",
            Self::CompareOnlyNoMutation => "compare_only_no_mutation",
            Self::NotApplicable => "not_applicable",
            Self::RefactorUnbound => "refactor_unbound",
        }
    }

    /// True when this refactor class is a concrete, bound transform.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::RefactorUnbound)
    }

    /// True when this refactor class is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::RefactorUnbound)
    }

    /// True when this refactor class mutates source.
    pub const fn is_mutating(self) -> bool {
        !matches!(self, Self::CompareOnlyNoMutation)
    }
}

/// Closed preview-completeness vocabulary. A `refactor_transaction_admission`
/// row binds exactly one completeness class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletenessClass {
    /// Preview captures the complete target set.
    Complete,
    /// Preview captures a partial target set with a visible label.
    Partial,
    /// Preview is blocked pending refresh, scope, or provider health.
    Blocked,
    /// Completeness is unsupported on this row.
    Unsupported,
    /// Row does not bind a completeness class.
    NotApplicable,
    /// Row has no bound completeness class; this never qualifies certified
    /// for a row class that requires a binding.
    CompletenessUnbound,
}

impl CompletenessClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Blocked => "blocked",
            Self::Unsupported => "unsupported",
            Self::NotApplicable => "not_applicable",
            Self::CompletenessUnbound => "completeness_unbound",
        }
    }

    /// True when this completeness class is a concrete, bound label.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::CompletenessUnbound)
    }

    /// True when this completeness class is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::CompletenessUnbound)
    }
}

/// Closed generated-artifact policy vocabulary. A
/// `generated_artifact_policy_admission` row binds exactly one policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedArtifactPolicyClass {
    /// The lane is not a generated artifact.
    NotGenerated,
    /// Regenerate from source before any edit.
    RegenerateBeforeEdit,
    /// Edit is allowed but a regeneration replay is required.
    EditWithRegenerationReplay,
    /// Direct edits to the generated source are blocked.
    EditBlockedGeneratedSource,
    /// Generated artifacts are compare-only.
    CompareOnlyGenerated,
    /// Row does not bind a generated-artifact policy.
    NotApplicable,
    /// Row has no bound policy; this never qualifies certified
    /// for a row class that requires a binding.
    PolicyUnbound,
}

impl GeneratedArtifactPolicyClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotGenerated => "not_generated",
            Self::RegenerateBeforeEdit => "regenerate_before_edit",
            Self::EditWithRegenerationReplay => "edit_with_regeneration_replay",
            Self::EditBlockedGeneratedSource => "edit_blocked_generated_source",
            Self::CompareOnlyGenerated => "compare_only_generated",
            Self::NotApplicable => "not_applicable",
            Self::PolicyUnbound => "policy_unbound",
        }
    }

    /// True when this policy class is a concrete, bound policy.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::PolicyUnbound)
    }

    /// True when this policy class is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::PolicyUnbound)
    }
}

/// Closed downgrade-label vocabulary. A `downgrade_label_admission` row
/// binds exactly one allowed downgrade label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeLabelClass {
    /// No downgrade is offered for the row.
    None,
    /// Semantic result downgrades to a text fallback.
    SemanticToTextFallback,
    /// Full completeness downgrades to a partial result.
    FullToPartialCompleteness,
    /// Previewable refactor downgrades to compare-only.
    PreviewableToCompareOnly,
    /// Code-action mutation downgrades to preview-only.
    MutationToPreviewOnly,
    /// Provider unavailable; text-only result.
    ProviderUnavailableTextOnly,
    /// Generated edit downgrades to regenerate-first.
    GeneratedEditToRegenerateFirst,
    /// Row does not bind a downgrade label.
    NotApplicable,
    /// Row has no bound downgrade label; this never qualifies certified
    /// for a row class that requires a binding.
    LabelUnbound,
}

impl DowngradeLabelClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SemanticToTextFallback => "semantic_to_text_fallback",
            Self::FullToPartialCompleteness => "full_to_partial_completeness",
            Self::PreviewableToCompareOnly => "previewable_to_compare_only",
            Self::MutationToPreviewOnly => "mutation_to_preview_only",
            Self::ProviderUnavailableTextOnly => "provider_unavailable_text_only",
            Self::GeneratedEditToRegenerateFirst => "generated_edit_to_regenerate_first",
            Self::NotApplicable => "not_applicable",
            Self::LabelUnbound => "label_unbound",
        }
    }

    /// True when this downgrade label is a concrete, bound label.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::LabelUnbound)
    }

    /// True when this downgrade label is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::LabelUnbound)
    }
}

/// Closed rollback-path vocabulary. A `refactor_transaction_admission` row
/// binds exactly one rollback path class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackPathClass {
    /// Exact undo is available through a local-history checkpoint.
    ExactUndoViaLocalHistoryCheckpoint,
    /// Revert is available as a compensating workspace diff.
    CompensatingRevertViaWorkspaceDiff,
    /// Grouped mutation-journal entry owns the revert.
    GroupedMutationJournalRevert,
    /// Generated artifacts must regenerate before replay.
    RegenerateFirstThenReplay,
    /// Manual review is required before an automatic route can be claimed.
    ManualReviewRequiredNoAutomaticPath,
    /// No safe automatic rollback exists.
    NoSafeRollbackAvailable,
    /// Row does not bind a rollback path.
    NotApplicable,
    /// Row has no bound rollback path; this never qualifies certified
    /// for a row class that requires a binding.
    RollbackUnbound,
}

impl RollbackPathClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactUndoViaLocalHistoryCheckpoint => "exact_undo_via_local_history_checkpoint",
            Self::CompensatingRevertViaWorkspaceDiff => "compensating_revert_via_workspace_diff",
            Self::GroupedMutationJournalRevert => "grouped_mutation_journal_revert",
            Self::RegenerateFirstThenReplay => "regenerate_first_then_replay",
            Self::ManualReviewRequiredNoAutomaticPath => "manual_review_required_no_automatic_path",
            Self::NoSafeRollbackAvailable => "no_safe_rollback_available",
            Self::NotApplicable => "not_applicable",
            Self::RollbackUnbound => "rollback_unbound",
        }
    }

    /// True when this rollback path is a concrete, bound route.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::RollbackUnbound)
    }

    /// True when this rollback path is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::RollbackUnbound)
    }
}

/// Closed evidence-class vocabulary describing what backs a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    /// Backed by certified archetype-repo evidence.
    ArchetypeRepoEvidence,
    /// Backed by framework- / formatter- / linter-migration evidence.
    FrameworkMigrationEvidence,
    /// Backed by design-partner evidence.
    DesignPartnerEvidence,
    /// Backed by a fixture-repo capture.
    FixtureRepoEvidence,
    /// Backed by a conformance suite run.
    ConformanceSuiteEvidence,
    /// Backed by a benchmark / fitness function capture.
    BenchmarkEvidence,
    /// Backed by a docs/help disclosure (gap label only).
    DocsDisclosureEvidence,
    /// Row has no bound evidence class; this never qualifies certified.
    EvidenceUnbound,
}

impl EvidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArchetypeRepoEvidence => "archetype_repo_evidence",
            Self::FrameworkMigrationEvidence => "framework_migration_evidence",
            Self::DesignPartnerEvidence => "design_partner_evidence",
            Self::FixtureRepoEvidence => "fixture_repo_evidence",
            Self::ConformanceSuiteEvidence => "conformance_suite_evidence",
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

/// Closed known-limit vocabulary attached to a matrix row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The row only certifies a provider-family subset.
    ProviderFamilySubsetOnly,
    /// The row only certifies an artifact-family subset.
    ArtifactFamilySubsetOnly,
    /// The row only certifies a semantic-mode subset.
    SemanticModeSubsetOnly,
    /// The row only certifies a refactor-class subset.
    RefactorClassSubsetOnly,
    /// The row only certifies a generated-policy subset.
    GeneratedPolicySubsetOnly,
    /// The row only certifies a diagnostic-source subset.
    DiagnosticSourceSubsetOnly,
    /// The row only certifies a compare-only (no mutation) posture.
    CompareOnlyNoMutationLimit,
    /// The row certifies an unsupported runtime target gap.
    UnsupportedRuntimeTarget,
    /// The row certifies a beta-grade-only capability sample.
    BetaCapabilitySampleOnly,
    /// The row has no bound known-limit class; this never qualifies certified.
    LimitUnbound,
}

impl KnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::ProviderFamilySubsetOnly => "provider_family_subset_only",
            Self::ArtifactFamilySubsetOnly => "artifact_family_subset_only",
            Self::SemanticModeSubsetOnly => "semantic_mode_subset_only",
            Self::RefactorClassSubsetOnly => "refactor_class_subset_only",
            Self::GeneratedPolicySubsetOnly => "generated_policy_subset_only",
            Self::DiagnosticSourceSubsetOnly => "diagnostic_source_subset_only",
            Self::CompareOnlyNoMutationLimit => "compare_only_no_mutation_limit",
            Self::UnsupportedRuntimeTarget => "unsupported_runtime_target",
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

/// Closed downgrade-automation vocabulary attached to a matrix row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when a certified fixture is missing or stale.
    AutoNarrowOnMissingFixture,
    /// Automatically narrow when the acting provider becomes unavailable.
    AutoNarrowOnProviderUnavailable,
    /// Automatically narrow when provider conflict is unresolved.
    AutoNarrowOnConflictUnresolved,
    /// Automatically narrow when preview drops below complete coverage.
    AutoNarrowOnPreviewPartial,
    /// Automatically narrow when result provenance goes stale.
    AutoNarrowOnStaleProvenance,
    /// Automatically demote when confidence drops below the certified bar.
    AutoDemoteOnLowConfidence,
    /// Automatically block when required evidence is missing.
    AutoBlockOnMissingEvidence,
    /// Manual-only review required until automation lands.
    ManualOnlyPendingReview,
    /// Automation is unbound; this never qualifies certified.
    AutomationUnbound,
}

impl DowngradeAutomationClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AutoNarrowOnMissingFixture => "auto_narrow_on_missing_fixture",
            Self::AutoNarrowOnProviderUnavailable => "auto_narrow_on_provider_unavailable",
            Self::AutoNarrowOnConflictUnresolved => "auto_narrow_on_conflict_unresolved",
            Self::AutoNarrowOnPreviewPartial => "auto_narrow_on_preview_partial",
            Self::AutoNarrowOnStaleProvenance => "auto_narrow_on_stale_provenance",
            Self::AutoDemoteOnLowConfidence => "auto_demote_on_low_confidence",
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

/// Closed confidence-class vocabulary for a matrix row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    /// High confidence — the lane can certify.
    HighConfidence,
    /// Medium confidence — the lane narrows below certified.
    MediumConfidence,
    /// Low confidence — the lane narrows below certified until evidence grows.
    LowConfidence,
}

impl ConfidenceClass {
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
    /// Packet certifies a stable claim across all required rows.
    Stable,
    /// Packet narrows below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
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

/// Closed validation-finding vocabulary for the matrix packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required artifact-family lane has no row.
    MissingArtifactFamilyLaneCoverage,
    /// A lane claiming certified is missing a capability-negotiation admission.
    MissingCapabilityNegotiationCoverage,
    /// A lane claiming certified is missing a conflict-arbitration admission.
    MissingConflictArbitrationCoverage,
    /// A lane claiming certified is missing a diagnostic-source admission.
    MissingDiagnosticSourceCoverage,
    /// A lane claiming certified is missing a result-provenance admission.
    MissingResultProvenanceCoverage,
    /// A lane claiming certified is missing a semantic-layer mode admission.
    MissingSemanticLayerModeCoverage,
    /// A lane claiming certified is missing a refactor-transaction admission.
    MissingRefactorTransactionCoverage,
    /// A lane claiming certified is missing a generated-artifact policy admission.
    MissingGeneratedArtifactPolicyCoverage,
    /// A lane claiming certified is missing a downgrade-label admission.
    MissingDowngradeLabelCoverage,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row that must name a provider family has no concrete provider.
    MissingProviderFamily,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A capability-negotiation admission row has no bound outcome.
    MissingCapabilityNegotiationClass,
    /// A conflict-arbitration admission row has no bound conflict class.
    MissingConflictClass,
    /// A diagnostic-source admission row has no bound source class.
    MissingDiagnosticSourceClass,
    /// A result-provenance admission row has no bound provenance class.
    MissingResultProvenanceClass,
    /// A semantic-layer mode admission row has no bound mode.
    MissingSemanticLayerModeClass,
    /// A refactor-transaction admission row has no bound refactor class.
    MissingRefactorTransactionClass,
    /// A refactor-transaction admission row has no bound completeness class.
    MissingCompletenessClass,
    /// A generated-artifact policy admission row has no bound policy.
    MissingGeneratedArtifactPolicyClass,
    /// A downgrade-label admission row has no bound label.
    MissingDowngradeLabelClass,
    /// A refactor-transaction admission row has no bound rollback path.
    MissingRollbackPathClass,
    /// A row claims certified while one or more bindings is unbound.
    CertifiedWithUnboundBinding,
    /// A row narrowed below certified drops its disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row with a non-`none_declared` known limit drops its disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// A capability-negotiation admission row drops its negotiation binding.
    CapabilityNegotiationNotApplicable,
    /// A non-capability-negotiation row binds a negotiation outcome.
    CapabilityNegotiationNotPermittedOnRowClass,
    /// A conflict-arbitration admission row drops its conflict binding.
    ConflictNotApplicable,
    /// A non-conflict-arbitration row binds a conflict class.
    ConflictNotPermittedOnRowClass,
    /// A diagnostic-source admission row drops its source binding.
    DiagnosticSourceNotApplicable,
    /// A non-diagnostic-source row binds a diagnostic source class.
    DiagnosticSourceNotPermittedOnRowClass,
    /// A result-provenance admission row drops its provenance binding.
    ResultProvenanceNotApplicable,
    /// A non-result-provenance row binds a provenance class.
    ResultProvenanceNotPermittedOnRowClass,
    /// A semantic-layer mode admission row drops its mode binding.
    SemanticLayerModeNotApplicable,
    /// A non-semantic-layer-mode row binds a semantic-layer mode.
    SemanticLayerModeNotPermittedOnRowClass,
    /// A refactor-transaction admission row drops its refactor binding.
    RefactorTransactionNotApplicable,
    /// A non-refactor-transaction row binds a refactor class.
    RefactorTransactionNotPermittedOnRowClass,
    /// A refactor-transaction admission row drops its completeness binding.
    CompletenessNotApplicable,
    /// A non-refactor-transaction row binds a completeness class.
    CompletenessNotPermittedOnRowClass,
    /// A generated-artifact policy admission row drops its policy binding.
    GeneratedArtifactPolicyNotApplicable,
    /// A non-generated-artifact-policy row binds a generated-artifact policy.
    GeneratedArtifactPolicyNotPermittedOnRowClass,
    /// A downgrade-label admission row drops its label binding.
    DowngradeLabelNotApplicable,
    /// A non-downgrade-label row binds a downgrade label.
    DowngradeLabelNotPermittedOnRowClass,
    /// A refactor-transaction admission row drops its rollback binding.
    RollbackPathNotApplicable,
    /// A non-refactor-transaction row binds a rollback path.
    RollbackPathNotPermittedOnRowClass,
    /// A mutating refactor admission row leaves preview/rollback unsafe.
    MutationBypassesPreviewOrRollback,
    /// A row admits raw source bodies or other private material.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops matrix truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the provider-family vocabulary.
    ProviderFamilyVocabularyCollapsed,
    /// A projection collapses the capability-negotiation vocabulary.
    CapabilityNegotiationVocabularyCollapsed,
    /// A projection collapses the conflict vocabulary.
    ConflictVocabularyCollapsed,
    /// A projection collapses the diagnostic-source vocabulary.
    DiagnosticSourceVocabularyCollapsed,
    /// A projection collapses the result-provenance vocabulary.
    ResultProvenanceVocabularyCollapsed,
    /// A projection collapses the semantic-layer mode vocabulary.
    SemanticLayerModeVocabularyCollapsed,
    /// A projection collapses the refactor-transaction vocabulary.
    RefactorTransactionVocabularyCollapsed,
    /// A projection collapses the completeness vocabulary.
    CompletenessVocabularyCollapsed,
    /// A projection collapses the generated-artifact policy vocabulary.
    GeneratedArtifactPolicyVocabularyCollapsed,
    /// A projection collapses the downgrade-label vocabulary.
    DowngradeLabelVocabularyCollapsed,
    /// A projection collapses the rollback-path vocabulary.
    RollbackPathVocabularyCollapsed,
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
            Self::MissingArtifactFamilyLaneCoverage => "missing_artifact_family_lane_coverage",
            Self::MissingCapabilityNegotiationCoverage => "missing_capability_negotiation_coverage",
            Self::MissingConflictArbitrationCoverage => "missing_conflict_arbitration_coverage",
            Self::MissingDiagnosticSourceCoverage => "missing_diagnostic_source_coverage",
            Self::MissingResultProvenanceCoverage => "missing_result_provenance_coverage",
            Self::MissingSemanticLayerModeCoverage => "missing_semantic_layer_mode_coverage",
            Self::MissingRefactorTransactionCoverage => "missing_refactor_transaction_coverage",
            Self::MissingGeneratedArtifactPolicyCoverage => {
                "missing_generated_artifact_policy_coverage"
            }
            Self::MissingDowngradeLabelCoverage => "missing_downgrade_label_coverage",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingProviderFamily => "missing_provider_family",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::MissingCapabilityNegotiationClass => "missing_capability_negotiation_class",
            Self::MissingConflictClass => "missing_conflict_class",
            Self::MissingDiagnosticSourceClass => "missing_diagnostic_source_class",
            Self::MissingResultProvenanceClass => "missing_result_provenance_class",
            Self::MissingSemanticLayerModeClass => "missing_semantic_layer_mode_class",
            Self::MissingRefactorTransactionClass => "missing_refactor_transaction_class",
            Self::MissingCompletenessClass => "missing_completeness_class",
            Self::MissingGeneratedArtifactPolicyClass => "missing_generated_artifact_policy_class",
            Self::MissingDowngradeLabelClass => "missing_downgrade_label_class",
            Self::MissingRollbackPathClass => "missing_rollback_path_class",
            Self::CertifiedWithUnboundBinding => "certified_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::CapabilityNegotiationNotApplicable => "capability_negotiation_not_applicable",
            Self::CapabilityNegotiationNotPermittedOnRowClass => {
                "capability_negotiation_not_permitted_on_row_class"
            }
            Self::ConflictNotApplicable => "conflict_not_applicable",
            Self::ConflictNotPermittedOnRowClass => "conflict_not_permitted_on_row_class",
            Self::DiagnosticSourceNotApplicable => "diagnostic_source_not_applicable",
            Self::DiagnosticSourceNotPermittedOnRowClass => {
                "diagnostic_source_not_permitted_on_row_class"
            }
            Self::ResultProvenanceNotApplicable => "result_provenance_not_applicable",
            Self::ResultProvenanceNotPermittedOnRowClass => {
                "result_provenance_not_permitted_on_row_class"
            }
            Self::SemanticLayerModeNotApplicable => "semantic_layer_mode_not_applicable",
            Self::SemanticLayerModeNotPermittedOnRowClass => {
                "semantic_layer_mode_not_permitted_on_row_class"
            }
            Self::RefactorTransactionNotApplicable => "refactor_transaction_not_applicable",
            Self::RefactorTransactionNotPermittedOnRowClass => {
                "refactor_transaction_not_permitted_on_row_class"
            }
            Self::CompletenessNotApplicable => "completeness_not_applicable",
            Self::CompletenessNotPermittedOnRowClass => "completeness_not_permitted_on_row_class",
            Self::GeneratedArtifactPolicyNotApplicable => {
                "generated_artifact_policy_not_applicable"
            }
            Self::GeneratedArtifactPolicyNotPermittedOnRowClass => {
                "generated_artifact_policy_not_permitted_on_row_class"
            }
            Self::DowngradeLabelNotApplicable => "downgrade_label_not_applicable",
            Self::DowngradeLabelNotPermittedOnRowClass => {
                "downgrade_label_not_permitted_on_row_class"
            }
            Self::RollbackPathNotApplicable => "rollback_path_not_applicable",
            Self::RollbackPathNotPermittedOnRowClass => "rollback_path_not_permitted_on_row_class",
            Self::MutationBypassesPreviewOrRollback => "mutation_bypasses_preview_or_rollback",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::ProviderFamilyVocabularyCollapsed => "provider_family_vocabulary_collapsed",
            Self::CapabilityNegotiationVocabularyCollapsed => {
                "capability_negotiation_vocabulary_collapsed"
            }
            Self::ConflictVocabularyCollapsed => "conflict_vocabulary_collapsed",
            Self::DiagnosticSourceVocabularyCollapsed => "diagnostic_source_vocabulary_collapsed",
            Self::ResultProvenanceVocabularyCollapsed => "result_provenance_vocabulary_collapsed",
            Self::SemanticLayerModeVocabularyCollapsed => {
                "semantic_layer_mode_vocabulary_collapsed"
            }
            Self::RefactorTransactionVocabularyCollapsed => {
                "refactor_transaction_vocabulary_collapsed"
            }
            Self::CompletenessVocabularyCollapsed => "completeness_vocabulary_collapsed",
            Self::GeneratedArtifactPolicyVocabularyCollapsed => {
                "generated_artifact_policy_vocabulary_collapsed"
            }
            Self::DowngradeLabelVocabularyCollapsed => "downgrade_label_vocabulary_collapsed",
            Self::RollbackPathVocabularyCollapsed => "rollback_path_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the matrix packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Framework-pack panel surface.
    FrameworkPackPanel,
    /// Notebook surface.
    NotebookSurface,
    /// Request / structured-artifact runner surface.
    RequestRunner,
    /// Preview surface.
    PreviewSurface,
    /// Docs surface.
    DocsSurface,
    /// Generated-artifact surface.
    GeneratedArtifactSurface,
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
    pub const REQUIRED: [Self; 10] = [
        Self::FrameworkPackPanel,
        Self::NotebookSurface,
        Self::RequestRunner,
        Self::PreviewSurface,
        Self::DocsSurface,
        Self::GeneratedArtifactSurface,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkPackPanel => "framework_pack_panel",
            Self::NotebookSurface => "notebook_surface",
            Self::RequestRunner => "request_runner",
            Self::PreviewSurface => "preview_surface",
            Self::DocsSurface => "docs_surface",
            Self::GeneratedArtifactSurface => "generated_artifact_surface",
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

/// One matrix row binding an artifact-family lane to the postures it may claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Artifact-family lane this row certifies.
    pub lane_class: ArtifactFamilyLaneClass,
    /// Row class.
    pub row_class: MatrixRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Acting provider family (or `not_applicable`).
    pub provider_family_class: ProviderFamilyClass,
    /// Capability-negotiation outcome (or `not_applicable`).
    pub capability_negotiation_class: CapabilityNegotiationClass,
    /// Provider-conflict class (or `not_applicable`).
    pub conflict_class: ConflictClass,
    /// Diagnostic source class (or `not_applicable`).
    pub diagnostic_source_class: DiagnosticSourceClass,
    /// Result-provenance class (or `not_applicable`).
    pub result_provenance_class: ResultProvenanceClass,
    /// Semantic-layer mode (or `not_applicable`).
    pub semantic_layer_mode_class: SemanticLayerModeClass,
    /// Refactor-transaction class (or `not_applicable`).
    pub refactor_transaction_class: RefactorTransactionClass,
    /// Preview-completeness class (or `not_applicable`).
    pub completeness_class: CompletenessClass,
    /// Generated-artifact policy class (or `not_applicable`).
    pub generated_artifact_policy_class: GeneratedArtifactPolicyClass,
    /// Allowed downgrade label (or `not_applicable`).
    pub downgrade_label_class: DowngradeLabelClass,
    /// Rollback path class (or `not_applicable`).
    pub rollback_path_class: RollbackPathClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: ConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `certified`, declares a non-`none_declared` known limit, or binds
    /// a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// True when raw source bodies are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl MatrixRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
            && self.provider_family_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Matrix packet id consumed by the projection.
    pub matrix_packet_id_ref: String,
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
    /// True when the provider-family vocabulary is preserved verbatim.
    pub preserves_provider_family_vocabulary: bool,
    /// True when the capability-negotiation vocabulary is preserved verbatim.
    pub preserves_capability_negotiation_vocabulary: bool,
    /// True when the conflict vocabulary is preserved verbatim.
    pub preserves_conflict_vocabulary: bool,
    /// True when the diagnostic-source vocabulary is preserved verbatim.
    pub preserves_diagnostic_source_vocabulary: bool,
    /// True when the result-provenance vocabulary is preserved verbatim.
    pub preserves_result_provenance_vocabulary: bool,
    /// True when the semantic-layer mode vocabulary is preserved verbatim.
    pub preserves_semantic_layer_mode_vocabulary: bool,
    /// True when the refactor-transaction vocabulary is preserved verbatim.
    pub preserves_refactor_transaction_vocabulary: bool,
    /// True when the completeness vocabulary is preserved verbatim.
    pub preserves_completeness_vocabulary: bool,
    /// True when the generated-artifact policy vocabulary is preserved verbatim.
    pub preserves_generated_artifact_policy_vocabulary: bool,
    /// True when the downgrade-label vocabulary is preserved verbatim.
    pub preserves_downgrade_label_vocabulary: bool,
    /// True when the rollback-path vocabulary is preserved verbatim.
    pub preserves_rollback_path_vocabulary: bool,
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

impl MatrixConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.matrix_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_provider_family_vocabulary
            && self.preserves_capability_negotiation_vocabulary
            && self.preserves_conflict_vocabulary
            && self.preserves_diagnostic_source_vocabulary
            && self.preserves_result_provenance_vocabulary
            && self.preserves_semantic_layer_mode_vocabulary
            && self.preserves_refactor_transaction_vocabulary
            && self.preserves_completeness_vocabulary
            && self.preserves_generated_artifact_policy_vocabulary
            && self.preserves_downgrade_label_vocabulary
            && self.preserves_rollback_path_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`ProviderRefactorMatrixTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRefactorMatrixTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Artifact-family lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<ArtifactFamilyLaneClass>,
    /// Matrix rows.
    #[serde(default)]
    pub rows: Vec<MatrixRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<MatrixConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet freezing the provider, diagnostic-cluster, and
/// refactor-transaction matrix across the M5 framework, notebook,
/// generated, and structured-artifact lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRefactorMatrixTruthPacket {
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
    /// Artifact-family lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<ArtifactFamilyLaneClass>,
    /// Matrix rows.
    #[serde(default)]
    pub rows: Vec<MatrixRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<MatrixConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl ProviderRefactorMatrixTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: ProviderRefactorMatrixTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: PROVIDER_REFACTOR_MATRIX_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_REFACTOR_MATRIX_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable matrix invariants.
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
        self.unique_tokens(|row| row.lane_class.as_str())
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.row_class.as_str())
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.support_class.as_str())
    }

    /// Returns the unique provider-family tokens observed across rows.
    pub fn provider_family_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.provider_family_class.as_str())
    }

    /// Returns the unique capability-negotiation tokens observed across rows.
    pub fn capability_negotiation_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.capability_negotiation_class.as_str())
    }

    /// Returns the unique conflict tokens observed across rows.
    pub fn conflict_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.conflict_class.as_str())
    }

    /// Returns the unique diagnostic-source tokens observed across rows.
    pub fn diagnostic_source_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.diagnostic_source_class.as_str())
    }

    /// Returns the unique result-provenance tokens observed across rows.
    pub fn result_provenance_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.result_provenance_class.as_str())
    }

    /// Returns the unique semantic-layer mode tokens observed across rows.
    pub fn semantic_layer_mode_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.semantic_layer_mode_class.as_str())
    }

    /// Returns the unique refactor-transaction tokens observed across rows.
    pub fn refactor_transaction_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.refactor_transaction_class.as_str())
    }

    /// Returns the unique completeness tokens observed across rows.
    pub fn completeness_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.completeness_class.as_str())
    }

    /// Returns the unique generated-artifact policy tokens observed across rows.
    pub fn generated_artifact_policy_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.generated_artifact_policy_class.as_str())
    }

    /// Returns the unique downgrade-label tokens observed across rows.
    pub fn downgrade_label_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.downgrade_label_class.as_str())
    }

    /// Returns the unique rollback-path tokens observed across rows.
    pub fn rollback_path_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.rollback_path_class.as_str())
    }

    /// Returns the unique known-limit tokens observed across rows.
    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.known_limit_class.as_str())
    }

    /// Returns the unique downgrade-automation tokens observed across rows.
    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.downgrade_automation_class.as_str())
    }

    /// Returns the unique evidence-class tokens observed across rows.
    pub fn evidence_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.evidence_class.as_str())
    }

    fn unique_tokens(&self, project: impl Fn(&MatrixRow) -> &'static str) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(project(row));
        }
        set.into_iter().collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> ProviderRefactorMatrixTruthSupportExport {
        ProviderRefactorMatrixTruthSupportExport {
            record_kind: PROVIDER_REFACTOR_MATRIX_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_REFACTOR_MATRIX_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            matrix_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            matrix_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != PROVIDER_REFACTOR_MATRIX_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "matrix packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != PROVIDER_REFACTOR_MATRIX_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "matrix packet has the wrong schema version",
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
                FindingKind::MissingArtifactFamilyLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered artifact-family lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingArtifactFamilyLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers artifact-family lane {}", lane.as_str()),
                ));
            }
        }

        for row in &self.rows {
            self.append_per_row_findings(row, &mut findings);
        }

        for lane in &self.covered_lanes {
            self.append_per_lane_coverage_findings(*lane, &mut findings);
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
            self.append_projection_findings(projection, &mut findings);
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

    fn append_per_row_findings(&self, row: &MatrixRow, findings: &mut Vec<ValidationFinding>) {
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
                    "row {} admits raw source bodies past the boundary",
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
        if row.row_class.requires_provider_family() && !row.provider_family_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingProviderFamily,
                FindingSeverity::Blocker,
                format!(
                    "row {} must name a concrete acting provider family",
                    row.row_id
                ),
            ));
        }

        if matches!(row.support_class, SupportClass::Certified) && !row.all_bindings_satisfied() {
            findings.push(ValidationFinding::new(
                FindingKind::CertifiedWithUnboundBinding,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims certified while a binding (support, provider family, known limit, downgrade automation, or evidence) is unbound",
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
        if row.known_limit_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
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

        self.append_dimension_gating_findings(row, findings);

        if matches!(row.confidence_class, ConfidenceClass::LowConfidence)
            && matches!(row.support_class, SupportClass::Certified)
        {
            findings.push(ValidationFinding::new(
                FindingKind::CertifiedWithUnboundBinding,
                FindingSeverity::Warning,
                format!(
                    "row {} claims certified at low_confidence; narrowing until evidence grows",
                    row.row_id
                ),
            ));
        }
    }

    fn append_dimension_gating_findings(
        &self,
        row: &MatrixRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let is_capability = matches!(
            row.row_class,
            MatrixRowClass::CapabilityNegotiationAdmission
        );
        let is_conflict = matches!(row.row_class, MatrixRowClass::ConflictArbitrationAdmission);
        let is_diagnostic = matches!(row.row_class, MatrixRowClass::DiagnosticSourceAdmission);
        let is_provenance = matches!(row.row_class, MatrixRowClass::ResultProvenanceAdmission);
        let is_semantic = matches!(row.row_class, MatrixRowClass::SemanticLayerModeAdmission);
        let is_refactor = matches!(row.row_class, MatrixRowClass::RefactorTransactionAdmission);
        let is_generated = matches!(
            row.row_class,
            MatrixRowClass::GeneratedArtifactPolicyAdmission
        );
        let is_downgrade_label = matches!(row.row_class, MatrixRowClass::DowngradeLabelAdmission);

        // Capability negotiation dimension.
        if is_capability && !row.capability_negotiation_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingCapabilityNegotiationClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has no bound capability-negotiation outcome",
                    row.row_id
                ),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::CapabilityNegotiationNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a capability_negotiation_admission but has no bound outcome",
                    row.row_id
                ),
            ));
        }
        if !is_capability && !row.capability_negotiation_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::CapabilityNegotiationNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds capability negotiation {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.capability_negotiation_class.as_str()
                ),
            ));
        }

        // Conflict dimension.
        if is_conflict && !row.conflict_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingConflictClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound conflict class", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::ConflictNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a conflict_arbitration_admission but has no bound conflict class",
                    row.row_id
                ),
            ));
        }
        if !is_conflict && !row.conflict_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::ConflictNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds conflict class {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.conflict_class.as_str()
                ),
            ));
        }

        // Diagnostic source dimension.
        if is_diagnostic && !row.diagnostic_source_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingDiagnosticSourceClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound diagnostic source class", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::DiagnosticSourceNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a diagnostic_source_admission but has no bound source class",
                    row.row_id
                ),
            ));
        }
        if !is_diagnostic && !row.diagnostic_source_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::DiagnosticSourceNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds diagnostic source {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.diagnostic_source_class.as_str()
                ),
            ));
        }

        // Result provenance dimension.
        if is_provenance && !row.result_provenance_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingResultProvenanceClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound result provenance class", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::ResultProvenanceNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a result_provenance_admission but has no bound provenance class",
                    row.row_id
                ),
            ));
        }
        if !is_provenance && !row.result_provenance_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::ResultProvenanceNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds result provenance {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.result_provenance_class.as_str()
                ),
            ));
        }

        // Semantic layer mode dimension.
        if is_semantic && !row.semantic_layer_mode_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingSemanticLayerModeClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound semantic-layer mode", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::SemanticLayerModeNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a semantic_layer_mode_admission but has no bound mode",
                    row.row_id
                ),
            ));
        }
        if !is_semantic && !row.semantic_layer_mode_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::SemanticLayerModeNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds semantic-layer mode {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.semantic_layer_mode_class.as_str()
                ),
            ));
        }

        // Refactor transaction dimension (with co-bound completeness and rollback).
        if is_refactor && !row.refactor_transaction_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingRefactorTransactionClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound refactor-transaction class", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::RefactorTransactionNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a refactor_transaction_admission but has no bound refactor class",
                    row.row_id
                ),
            ));
        }
        if !is_refactor && !row.refactor_transaction_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::RefactorTransactionNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds refactor class {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.refactor_transaction_class.as_str()
                ),
            ));
        }

        // Completeness dimension — owned by refactor_transaction_admission.
        if is_refactor && !row.completeness_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingCompletenessClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound completeness class", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::CompletenessNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a refactor_transaction_admission but has no bound completeness class",
                    row.row_id
                ),
            ));
        }
        if !is_refactor && !row.completeness_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::CompletenessNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds completeness {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.completeness_class.as_str()
                ),
            ));
        }

        // Rollback dimension — owned by refactor_transaction_admission.
        if is_refactor && !row.rollback_path_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingRollbackPathClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound rollback path class", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::RollbackPathNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a refactor_transaction_admission but has no bound rollback path",
                    row.row_id
                ),
            ));
        }
        if !is_refactor && !row.rollback_path_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::RollbackPathNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds rollback path {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.rollback_path_class.as_str()
                ),
            ));
        }

        // A mutating refactor must not bypass typed preview or rollback.
        if is_refactor
            && row.refactor_transaction_class.is_concrete()
            && row.refactor_transaction_class.is_mutating()
        {
            let preview_unsafe = matches!(
                row.completeness_class,
                CompletenessClass::Unsupported | CompletenessClass::CompletenessUnbound
            );
            let rollback_unsafe = matches!(
                row.rollback_path_class,
                RollbackPathClass::NoSafeRollbackAvailable | RollbackPathClass::RollbackUnbound
            );
            if preview_unsafe || rollback_unsafe {
                findings.push(ValidationFinding::new(
                    FindingKind::MutationBypassesPreviewOrRollback,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds mutating refactor {} without a typed preview completeness and a safe rollback path",
                        row.row_id,
                        row.refactor_transaction_class.as_str()
                    ),
                ));
            }
        }

        // Generated artifact policy dimension.
        if is_generated && !row.generated_artifact_policy_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingGeneratedArtifactPolicyClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound generated-artifact policy", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::GeneratedArtifactPolicyNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a generated_artifact_policy_admission but has no bound policy",
                    row.row_id
                ),
            ));
        }
        if !is_generated && !row.generated_artifact_policy_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::GeneratedArtifactPolicyNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds generated-artifact policy {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.generated_artifact_policy_class.as_str()
                ),
            ));
        }

        // Downgrade label dimension.
        if is_downgrade_label && !row.downgrade_label_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingDowngradeLabelClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound downgrade label", row.row_id),
            ));
            findings.push(ValidationFinding::new(
                FindingKind::DowngradeLabelNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a downgrade_label_admission but has no bound downgrade label",
                    row.row_id
                ),
            ));
        }
        if !is_downgrade_label && !row.downgrade_label_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::DowngradeLabelNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds downgrade label {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.downgrade_label_class.as_str()
                ),
            ));
        }
    }

    fn append_per_lane_coverage_findings(
        &self,
        lane: ArtifactFamilyLaneClass,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let lane_claims_stable = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(row.row_class, MatrixRowClass::MatrixLaneQuality)
                && matches!(row.support_class, SupportClass::Certified)
        });
        if !lane_claims_stable {
            return;
        }

        let required: [(MatrixRowClass, FindingKind, &str); 8] = [
            (
                MatrixRowClass::CapabilityNegotiationAdmission,
                FindingKind::MissingCapabilityNegotiationCoverage,
                "capability_negotiation_admission",
            ),
            (
                MatrixRowClass::ConflictArbitrationAdmission,
                FindingKind::MissingConflictArbitrationCoverage,
                "conflict_arbitration_admission",
            ),
            (
                MatrixRowClass::DiagnosticSourceAdmission,
                FindingKind::MissingDiagnosticSourceCoverage,
                "diagnostic_source_admission",
            ),
            (
                MatrixRowClass::ResultProvenanceAdmission,
                FindingKind::MissingResultProvenanceCoverage,
                "result_provenance_admission",
            ),
            (
                MatrixRowClass::SemanticLayerModeAdmission,
                FindingKind::MissingSemanticLayerModeCoverage,
                "semantic_layer_mode_admission",
            ),
            (
                MatrixRowClass::RefactorTransactionAdmission,
                FindingKind::MissingRefactorTransactionCoverage,
                "refactor_transaction_admission",
            ),
            (
                MatrixRowClass::GeneratedArtifactPolicyAdmission,
                FindingKind::MissingGeneratedArtifactPolicyCoverage,
                "generated_artifact_policy_admission",
            ),
            (
                MatrixRowClass::DowngradeLabelAdmission,
                FindingKind::MissingDowngradeLabelCoverage,
                "downgrade_label_admission",
            ),
        ];

        for (row_class, finding_kind, label) in required {
            let covered = self
                .rows
                .iter()
                .any(|row| row.lane_class == lane && row.row_class == row_class);
            if !covered {
                findings.push(ValidationFinding::new(
                    finding_kind,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims certified but has no {} row",
                        lane.as_str(),
                        label
                    ),
                ));
            }
        }
    }

    fn append_projection_findings(
        &self,
        projection: &MatrixConsumerProjection,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if !projection.preserves_truth_for(&self.packet_id) {
            findings.push(ValidationFinding::new(
                FindingKind::ConsumerProjectionDrift,
                FindingSeverity::Blocker,
                format!(
                    "projection {} does not preserve matrix truth",
                    projection.projection_ref
                ),
            ));
        }
        let collapses: [(bool, FindingKind, &str); 18] = [
            (
                projection.preserves_lane_vocabulary,
                FindingKind::LaneVocabularyCollapsed,
                "lane",
            ),
            (
                projection.preserves_row_class_vocabulary,
                FindingKind::RowClassVocabularyCollapsed,
                "row-class",
            ),
            (
                projection.preserves_support_class_vocabulary,
                FindingKind::SupportClassVocabularyCollapsed,
                "support-class",
            ),
            (
                projection.preserves_provider_family_vocabulary,
                FindingKind::ProviderFamilyVocabularyCollapsed,
                "provider-family",
            ),
            (
                projection.preserves_capability_negotiation_vocabulary,
                FindingKind::CapabilityNegotiationVocabularyCollapsed,
                "capability-negotiation",
            ),
            (
                projection.preserves_conflict_vocabulary,
                FindingKind::ConflictVocabularyCollapsed,
                "conflict",
            ),
            (
                projection.preserves_diagnostic_source_vocabulary,
                FindingKind::DiagnosticSourceVocabularyCollapsed,
                "diagnostic-source",
            ),
            (
                projection.preserves_result_provenance_vocabulary,
                FindingKind::ResultProvenanceVocabularyCollapsed,
                "result-provenance",
            ),
            (
                projection.preserves_semantic_layer_mode_vocabulary,
                FindingKind::SemanticLayerModeVocabularyCollapsed,
                "semantic-layer-mode",
            ),
            (
                projection.preserves_refactor_transaction_vocabulary,
                FindingKind::RefactorTransactionVocabularyCollapsed,
                "refactor-transaction",
            ),
            (
                projection.preserves_completeness_vocabulary,
                FindingKind::CompletenessVocabularyCollapsed,
                "completeness",
            ),
            (
                projection.preserves_generated_artifact_policy_vocabulary,
                FindingKind::GeneratedArtifactPolicyVocabularyCollapsed,
                "generated-artifact-policy",
            ),
            (
                projection.preserves_downgrade_label_vocabulary,
                FindingKind::DowngradeLabelVocabularyCollapsed,
                "downgrade-label",
            ),
            (
                projection.preserves_rollback_path_vocabulary,
                FindingKind::RollbackPathVocabularyCollapsed,
                "rollback-path",
            ),
            (
                projection.preserves_known_limit_vocabulary,
                FindingKind::KnownLimitVocabularyCollapsed,
                "known-limit",
            ),
            (
                projection.preserves_downgrade_automation_vocabulary,
                FindingKind::DowngradeAutomationVocabularyCollapsed,
                "downgrade-automation",
            ),
            (
                projection.preserves_evidence_class_vocabulary,
                FindingKind::EvidenceClassVocabularyCollapsed,
                "evidence-class",
            ),
            // Sentinel kept last so the table stays a fixed shape; always true.
            (true, FindingKind::ConsumerProjectionDrift, "_sentinel"),
        ];
        for (preserved, finding_kind, label) in collapses {
            if label == "_sentinel" {
                continue;
            }
            if !preserved {
                findings.push(ValidationFinding::new(
                    finding_kind,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the {} vocabulary",
                        projection.projection_ref, label
                    ),
                ));
            }
        }
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
pub struct ProviderRefactorMatrixTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub matrix_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub matrix_packet: ProviderRefactorMatrixTruthPacket,
}

impl ProviderRefactorMatrixTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == PROVIDER_REFACTOR_MATRIX_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == PROVIDER_REFACTOR_MATRIX_TRUTH_SCHEMA_VERSION
            && self.matrix_packet_id_ref == self.matrix_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.matrix_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable matrix packet.
#[derive(Debug)]
pub enum ProviderRefactorMatrixTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for ProviderRefactorMatrixTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(formatter, "matrix packet parse failed: {error}"),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "matrix packet failed validation: {tokens}")
            }
        }
    }
}

impl Error for ProviderRefactorMatrixTruthArtifactError {}

/// Returns the checked-in stable provider/refactor matrix truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_provider_refactor_matrix_truth_packet(
) -> Result<ProviderRefactorMatrixTruthPacket, ProviderRefactorMatrixTruthArtifactError> {
    let packet: ProviderRefactorMatrixTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/language/m5/provider_refactor_matrix_truth_packet.json"
    )))
    .map_err(ProviderRefactorMatrixTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(ProviderRefactorMatrixTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests;
