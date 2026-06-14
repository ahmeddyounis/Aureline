//! Diagnostic-cluster, semantic-layer banner, freshness/scope-label, and
//! detail-sheet truth packet.
//!
//! This module is the language-owned contract for the clustered diagnostic
//! surface that keeps Problems and in-context findings trustworthy across the
//! M5 **notebook, framework, preview, and generated-code** consumers. Where the
//! sibling [`crate::semantic_result_arbitration_truth_packet`] certifies the
//! *answer* a definition/references/hierarchy/completion lane returns, this
//! packet certifies the *diagnostic cluster* those surfaces render: which
//! source families (compiler, linter, language-server, framework, runtime,
//! notebook, policy) converged into one cluster, whether per-provider detail,
//! timestamps/epochs, suppression/baseline state, and related symbol/file
//! evidence survived the deduplication, which semantic-layer banner the surface
//! may show (semantic, graph-warm, syntax-only, cached, runtime-only, or
//! partial), the freshness and scope labels the cluster may claim, and — when a
//! fix is offered — the acting provider, freshness/scope posture, and (for a
//! mutating fix) the typed preview completeness and rollback checkpoint the
//! launch-language refactor safety model still requires.
//!
//! Each row binds one cluster lane on one consumer surface together with:
//!
//! - a **cluster identity** block — the diagnostic source families that
//!   converged, whether deduplication preserved per-provider detail,
//!   timestamps/epochs, suppression/baseline state, and related evidence,
//!   whether the source families stayed differentiated rather than fused into
//!   one undifferentiated row, and the route that opens the cluster detail
//!   sheet;
//! - a **semantic-layer banner** block — which posture the surface is in and
//!   the freshness and scope labels it may claim; and
//! - a **fix-offer** block — whether a fix is offered, the acting provider and
//!   freshness/scope posture named alongside it, and (for a mutating fix) the
//!   typed preview completeness and rollback checkpoint required before any
//!   organize-imports, schema/codegen, AI-planned, or notebook/generated edit
//!   mutates source.
//!
//! The packet reuses the closed provider-family, conflict, diagnostic-source,
//! completeness, support, evidence, known-limit, downgrade-automation,
//! confidence, and consumer-surface vocabularies frozen by the
//! [`crate::provider_refactor_matrix_truth_packet`] matrix instead of minting a
//! local synonym set, and adds only the cluster-provenance,
//! source-differentiation, detail-sheet, semantic-layer banner, freshness,
//! scope-label, provider-disagreement-visibility, and fix-offer vocabulary the
//! clustered diagnostic surfaces need on top.
//!
//! The validator narrows below stable — it never silently publishes — whenever
//! a row would hide truth the source documents require to stay inspectable: a
//! multi-source cluster that dropped per-provider detail, timestamps,
//! suppression/baseline state, or related evidence; runtime evidence, policy /
//! security findings, and static analysis fused into one undifferentiated row;
//! a provider disagreement collapsed into ranking-only output that drops the
//! losing provider; an opaque spinner standing in for a real detail-sheet
//! route; a `semantic` banner claimed on stale or non-semantic evidence; a
//! whole-workspace scope claimed on stale or runtime-only evidence; a fix
//! offered without naming the acting provider and freshness/scope posture; or a
//! mutating fix that bypasses typed preview completeness and a rollback
//! checkpoint.
//!
//! The packet is metadata-only: it never admits raw source bodies, raw
//! notebook outputs, raw generated artifacts, provider payloads, secrets, or
//! ambient credentials past the boundary. It carries opaque ids, closed
//! vocabulary tokens, and export-safe refs only.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::provider_refactor_matrix_truth_packet::{
    CompletenessClass, ConfidenceClass, ConflictClass, ConsumerSurface, DiagnosticSourceClass,
    DowngradeAutomationClass, EvidenceClass, FindingSeverity, KnownLimitClass, PromotionState,
    ProviderFamilyClass, SupportClass,
};

/// Stable record-kind tag for [`DiagnosticClusterSemanticLayerTruthPacket`].
pub const DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_PACKET_RECORD_KIND: &str =
    "diagnostic_cluster_semantic_layer_truth_stable_packet";

/// Stable record-kind tag for [`DiagnosticClusterSemanticLayerTruthSupportExport`].
pub const DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "diagnostic_cluster_semantic_layer_truth_support_export";

/// Integer schema version for the diagnostic-cluster semantic-layer truth packet.
pub const DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_SCHEMA_REF: &str =
    "schemas/language/diagnostic_cluster_semantic_layer_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_DOC_REF: &str =
    "docs/m5/diagnostic-clustering-semantic-layer-banners-and-detail-sheets.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m5/diagnostic-clustering-semantic-layer-banners-and-detail-sheets.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m5/diagnostic_cluster_semantic_layer_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m5/diagnostic_cluster_semantic_layer_truth_packet.json";

/// Repo-relative path of the sibling provider/refactor matrix packet whose
/// closed vocabulary this packet reuses.
pub const DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_MATRIX_SOURCE_REF: &str =
    "artifacts/language/m5/provider_refactor_matrix_truth_packet.json";

/// Closed host-surface vocabulary. Every required surface MUST have rows in any
/// stable packet so the same semantic-layer banner and detail-sheet model stays
/// inspectable across the notebook, framework, preview, and generated-code
/// consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// Notebook-cell diagnostic surface.
    NotebookSurface,
    /// Framework-route / framework-component diagnostic surface.
    FrameworkSurface,
    /// Preview-linked code diagnostic surface.
    PreviewSurface,
    /// Generated / scaffolded source diagnostic surface.
    GeneratedCodeSurface,
}

impl SurfaceClass {
    /// Every required host surface, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::NotebookSurface,
        Self::FrameworkSurface,
        Self::PreviewSurface,
        Self::GeneratedCodeSurface,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookSurface => "notebook_surface",
            Self::FrameworkSurface => "framework_surface",
            Self::PreviewSurface => "preview_surface",
            Self::GeneratedCodeSurface => "generated_code_surface",
        }
    }
}

/// Closed cluster-lane vocabulary: the primary diagnostic family a cluster
/// centers on. The packet certifies that compiler, linter, language-server,
/// framework, runtime, notebook, and policy findings each cluster without
/// losing provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClusterLaneClass {
    /// Compiler / build diagnostic cluster.
    Compiler,
    /// Linter / formatter diagnostic cluster.
    Linter,
    /// Language-server diagnostic cluster.
    LanguageServer,
    /// Framework / schema diagnostic cluster.
    Framework,
    /// Runtime / test / debug diagnostic cluster.
    Runtime,
    /// Notebook-kernel diagnostic cluster.
    Notebook,
    /// Policy / trust / security diagnostic cluster.
    Policy,
}

impl ClusterLaneClass {
    /// Every required cluster lane, in declaration order.
    pub const REQUIRED: [Self; 7] = [
        Self::Compiler,
        Self::Linter,
        Self::LanguageServer,
        Self::Framework,
        Self::Runtime,
        Self::Notebook,
        Self::Policy,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compiler => "compiler",
            Self::Linter => "linter",
            Self::LanguageServer => "language_server",
            Self::Framework => "framework",
            Self::Runtime => "runtime",
            Self::Notebook => "notebook",
            Self::Policy => "policy",
        }
    }
}

/// Closed cluster-provenance vocabulary: whether deduplication preserved the
/// per-provider detail behind a cluster. A multi-source cluster that drops its
/// per-provider detail collapses truth and is refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClusterProvenanceClass {
    /// One provider contributed; there is no per-provider detail to preserve.
    SingleProviderCluster,
    /// Multiple providers converged and each provider's detail stays inspectable.
    PerProviderPreserved,
    /// Multiple providers converged but the per-provider detail was collapsed.
    CollapsedLossy,
}

impl ClusterProvenanceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleProviderCluster => "single_provider_cluster",
            Self::PerProviderPreserved => "per_provider_preserved",
            Self::CollapsedLossy => "collapsed_lossy",
        }
    }

    /// True when per-provider detail was dropped from a clustered result.
    pub const fn is_collapsed(self) -> bool {
        matches!(self, Self::CollapsedLossy)
    }
}

/// Closed source-differentiation vocabulary: whether the source families behind
/// a cluster stay distinguishable. Runtime evidence, policy / security
/// findings, and static analysis must not collapse into one undifferentiated
/// error row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceDifferentiationClass {
    /// A single source family contributed; differentiation is moot.
    SingleSourceNotApplicable,
    /// The source families stay differentiated within the cluster.
    DifferentiatedBySource,
    /// The source families were fused into one undifferentiated row.
    FusedUndifferentiated,
}

impl SourceDifferentiationClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleSourceNotApplicable => "single_source_not_applicable",
            Self::DifferentiatedBySource => "differentiated_by_source",
            Self::FusedUndifferentiated => "fused_undifferentiated",
        }
    }

    /// True when the source families were fused into one undifferentiated row.
    pub const fn is_fused(self) -> bool {
        matches!(self, Self::FusedUndifferentiated)
    }

    /// True when the source families stay distinguishable.
    pub const fn is_differentiated(self) -> bool {
        matches!(
            self,
            Self::DifferentiatedBySource | Self::SingleSourceNotApplicable
        )
    }
}

/// Closed detail-sheet route vocabulary: how a user opens the cluster detail
/// sheet that holds per-provider detail, timestamps, suppression state, and
/// related evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetailSheetRouteClass {
    /// No detail route is bound (only valid when nothing needs inspecting).
    NotApplicable,
    /// Opens the cluster detail sheet.
    OpenClusterDetailSheet,
    /// Opens the per-provider breakdown.
    OpenProviderBreakdown,
    /// An opaque loading spinner stands in for a real detail route.
    OpaqueSpinner,
}

impl DetailSheetRouteClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::OpenClusterDetailSheet => "open_cluster_detail_sheet",
            Self::OpenProviderBreakdown => "open_provider_breakdown",
            Self::OpaqueSpinner => "opaque_spinner",
        }
    }

    /// True when this route actually opens an inspectable detail sheet.
    pub const fn is_inspectable(self) -> bool {
        matches!(
            self,
            Self::OpenClusterDetailSheet | Self::OpenProviderBreakdown
        )
    }
}

/// Closed semantic-layer banner vocabulary: the posture the surface explains it
/// is in. This is the central output of the row — which layer the surface may
/// claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticLayerBannerClass {
    /// Full semantic mode: a healthy semantic provider answered.
    Semantic,
    /// Graph-warm mode: the semantic graph is still warming.
    GraphWarm,
    /// Syntax-only mode: only the syntax substrate is available.
    SyntaxOnly,
    /// Cached mode: a cached result is being reused.
    Cached,
    /// Runtime-only mode: only runtime / test / debug evidence is available.
    RuntimeOnly,
    /// Partial mode: a partial semantic result with a visible label.
    Partial,
    /// Row has no bound banner; this never qualifies certified.
    BannerUnbound,
}

impl SemanticLayerBannerClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Semantic => "semantic",
            Self::GraphWarm => "graph_warm",
            Self::SyntaxOnly => "syntax_only",
            Self::Cached => "cached",
            Self::RuntimeOnly => "runtime_only",
            Self::Partial => "partial",
            Self::BannerUnbound => "banner_unbound",
        }
    }

    /// True when this banner is bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::BannerUnbound)
    }

    /// True when this banner claims the full semantic posture.
    pub const fn claims_full_semantic(self) -> bool {
        matches!(self, Self::Semantic)
    }

    /// True when this banner rests on non-semantic evidence and so may not stand
    /// in for a live semantic answer or back a whole-workspace claim.
    pub const fn rests_on_non_semantic_evidence(self) -> bool {
        matches!(self, Self::SyntaxOnly | Self::Cached | Self::RuntimeOnly)
    }
}

/// Closed freshness vocabulary: how fresh the cluster evidence is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Live evidence proven against the current epoch.
    Live,
    /// Warm evidence from a recent epoch.
    Warm,
    /// Cached evidence reused with a visible label.
    Cached,
    /// Stale evidence pending refresh.
    Stale,
    /// Row has no bound freshness label; this never qualifies certified.
    FreshnessUnbound,
}

impl FreshnessClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Warm => "warm",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::FreshnessUnbound => "freshness_unbound",
        }
    }

    /// True when this freshness label is bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::FreshnessUnbound)
    }

    /// True when this freshness label supports a live semantic posture.
    pub const fn is_live_or_warm(self) -> bool {
        matches!(self, Self::Live | Self::Warm)
    }

    /// True when this freshness label is stale.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::Stale)
    }
}

/// Closed scope-label vocabulary: the breadth the cluster may claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeLabelClass {
    /// Whole-workspace coverage.
    WholeWorkspace,
    /// Coverage limited to the currently loaded slice.
    LoadedSlice,
    /// Coverage limited to the active file.
    ActiveFile,
    /// Coverage limited to the open notebook cells.
    OpenCells,
    /// Coverage limited to a single artifact.
    SingleArtifact,
    /// Coverage that explicitly excludes generated edges.
    GeneratedExcluded,
    /// Row has no bound scope label; this never qualifies certified.
    ScopeUnbound,
}

impl ScopeLabelClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WholeWorkspace => "whole_workspace",
            Self::LoadedSlice => "loaded_slice",
            Self::ActiveFile => "active_file",
            Self::OpenCells => "open_cells",
            Self::SingleArtifact => "single_artifact",
            Self::GeneratedExcluded => "generated_excluded",
            Self::ScopeUnbound => "scope_unbound",
        }
    }

    /// True when this scope label is bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::ScopeUnbound)
    }

    /// True when this scope claims the whole workspace.
    pub const fn is_whole_workspace(self) -> bool {
        matches!(self, Self::WholeWorkspace)
    }
}

/// Closed provider-disagreement visibility vocabulary: whether the providers
/// that lost a disagreement stay inspectable. A disagreement that drops its
/// loser collapses truth and is refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderDisagreementVisibilityClass {
    /// A single provider answered; there is no loser to preserve.
    NotApplicableSingleProvider,
    /// The losing providers and what they reported stay inspectable.
    LosersPreservedInspectable,
    /// The disagreement was collapsed into a ranking-only result.
    LosersCollapsedRankingOnly,
    /// The cluster recorded a conflict but exposed no loser at all.
    NoLosersExposed,
}

impl ProviderDisagreementVisibilityClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicableSingleProvider => "not_applicable_single_provider",
            Self::LosersPreservedInspectable => "losers_preserved_inspectable",
            Self::LosersCollapsedRankingOnly => "losers_collapsed_ranking_only",
            Self::NoLosersExposed => "no_losers_exposed",
        }
    }

    /// True when the losing providers are dropped from the cluster.
    pub const fn loses_losers(self) -> bool {
        matches!(
            self,
            Self::LosersCollapsedRankingOnly | Self::NoLosersExposed
        )
    }
}

/// Closed fix-offer vocabulary: what fix, if any, the cluster offers. A
/// mutating fix must bind typed preview completeness and a rollback checkpoint;
/// any fix must name its acting provider and freshness/scope posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FixOfferClass {
    /// No fix is offered.
    NoFixOffered,
    /// A non-mutating fix (navigate, explain, suppress-with-review).
    NonMutatingFix,
    /// A mutating quick-fix / code action.
    MutatingQuickFix,
    /// An organize-imports rewrite.
    OrganizeImportsFix,
    /// A schema / codegen rewrite.
    SchemaCodegenFix,
    /// An AI-planned transform.
    AiPlannedFix,
    /// A notebook / generated-source edit.
    NotebookGeneratedFix,
}

impl FixOfferClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoFixOffered => "no_fix_offered",
            Self::NonMutatingFix => "non_mutating_fix",
            Self::MutatingQuickFix => "mutating_quick_fix",
            Self::OrganizeImportsFix => "organize_imports_fix",
            Self::SchemaCodegenFix => "schema_codegen_fix",
            Self::AiPlannedFix => "ai_planned_fix",
            Self::NotebookGeneratedFix => "notebook_generated_fix",
        }
    }

    /// True when a fix is offered for the cluster.
    pub const fn offers_fix(self) -> bool {
        !matches!(self, Self::NoFixOffered)
    }

    /// True when the offered fix mutates source and so owes typed preview
    /// completeness and a rollback checkpoint.
    pub const fn is_mutating(self) -> bool {
        matches!(
            self,
            Self::MutatingQuickFix
                | Self::OrganizeImportsFix
                | Self::SchemaCodegenFix
                | Self::AiPlannedFix
                | Self::NotebookGeneratedFix
        )
    }
}

/// Coarse kind a diagnostic source family belongs to, used to enforce that
/// runtime evidence, policy findings, and static analysis stay differentiated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SourceKind {
    Static,
    Runtime,
    Policy,
    Other,
}

fn source_kind(source: DiagnosticSourceClass) -> SourceKind {
    match source {
        DiagnosticSourceClass::CompilerBuild
        | DiagnosticSourceClass::Lsp
        | DiagnosticSourceClass::LinterFormatter
        | DiagnosticSourceClass::FrameworkSchema
        | DiagnosticSourceClass::GeneratedArtifactValidation => SourceKind::Static,
        DiagnosticSourceClass::RuntimeTestDebug | DiagnosticSourceClass::NotebookKernel => {
            SourceKind::Runtime
        }
        DiagnosticSourceClass::PolicyTrust => SourceKind::Policy,
        DiagnosticSourceClass::NotApplicable | DiagnosticSourceClass::SourceUnbound => {
            SourceKind::Other
        }
    }
}

/// Closed validation-finding vocabulary for the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// A required host surface has no row.
    MissingSurfaceCoverage,
    /// A required cluster lane has no row.
    MissingClusterLaneCoverage,
    /// A row admits raw source bodies past the boundary.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority / credentials past the boundary.
    AmbientAuthorityPresent,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A row does not name a concrete acting provider family.
    MissingProviderFamily,
    /// A row lists no concrete diagnostic source families.
    MissingDiagnosticSources,
    /// A row has no bound semantic-layer banner.
    MissingSemanticLayerBanner,
    /// A row has no bound freshness label.
    MissingFreshnessLabel,
    /// A row has no bound scope label.
    MissingScopeLabel,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// A row claims certified while a required binding is unbound.
    CertifiedWithUnboundBinding,
    /// A narrowed row carries no disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row declares a known limit without a disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row binds a downgrade automation without a disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A required consumer projection is missing or does not preserve the packet.
    MissingConsumerProjection,
    /// A multi-source cluster dropped per-provider detail, timestamps,
    /// suppression/baseline state, or related evidence.
    ClusterProvenanceCollapsed,
    /// Runtime evidence, policy findings, and static analysis were fused into
    /// one undifferentiated row.
    SourcesFusedUndifferentiated,
    /// A provider disagreement collapsed the losing provider into ranking-only.
    LosingProviderCollapsed,
    /// An opaque spinner stands in for a real detail-sheet route.
    OpaqueDetailSheetRoute,
    /// A multi-source or disagreeing cluster offers no inspectable detail sheet.
    DetailSheetRouteMissing,
    /// A `semantic` banner was claimed on stale or non-semantic evidence.
    SemanticLayerOverclaimed,
    /// A whole-workspace scope was claimed on stale or runtime-only evidence.
    OverclaimedScopeOnStaleEvidence,
    /// A fix was offered without naming the acting provider or freshness/scope.
    FixOfferedWithoutProviderOrFreshness,
    /// A mutating fix bypasses typed preview completeness or a rollback checkpoint.
    MutatingFixBypassesPreview,
    /// A row claims certified at low confidence; it narrows until evidence grows.
    CertifiedAtLowConfidence,
    /// The stored promotion state does not match the derived findings.
    PromotionStateMismatch,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSurfaceCoverage => "missing_surface_coverage",
            Self::MissingClusterLaneCoverage => "missing_cluster_lane_coverage",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::MissingProviderFamily => "missing_provider_family",
            Self::MissingDiagnosticSources => "missing_diagnostic_sources",
            Self::MissingSemanticLayerBanner => "missing_semantic_layer_banner",
            Self::MissingFreshnessLabel => "missing_freshness_label",
            Self::MissingScopeLabel => "missing_scope_label",
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::CertifiedWithUnboundBinding => "certified_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ClusterProvenanceCollapsed => "cluster_provenance_collapsed",
            Self::SourcesFusedUndifferentiated => "sources_fused_undifferentiated",
            Self::LosingProviderCollapsed => "losing_provider_collapsed",
            Self::OpaqueDetailSheetRoute => "opaque_detail_sheet_route",
            Self::DetailSheetRouteMissing => "detail_sheet_route_missing",
            Self::SemanticLayerOverclaimed => "semantic_layer_overclaimed",
            Self::OverclaimedScopeOnStaleEvidence => "overclaimed_scope_on_stale_evidence",
            Self::FixOfferedWithoutProviderOrFreshness => {
                "fix_offered_without_provider_or_freshness"
            }
            Self::MutatingFixBypassesPreview => "mutating_fix_bypasses_preview",
            Self::CertifiedAtLowConfidence => "certified_at_low_confidence",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
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

/// One diagnostic-cluster row binding a host surface and cluster lane to the
/// cluster-provenance, semantic-layer banner, freshness/scope, and fix-offer
/// truth it must show.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticClusterRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Host surface this row renders on.
    pub surface_class: SurfaceClass,
    /// Primary diagnostic family the cluster centers on.
    pub cluster_lane_class: ClusterLaneClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Diagnostic source families that converged into the cluster.
    #[serde(default)]
    pub diagnostic_source_classes: Vec<DiagnosticSourceClass>,
    /// Whether deduplication preserved the per-provider detail.
    pub cluster_provenance_class: ClusterProvenanceClass,
    /// Whether the source families stay differentiated within the cluster.
    pub source_differentiation_class: SourceDifferentiationClass,
    /// True when per-provider detail is preserved in the detail sheet.
    pub preserves_per_provider_detail: bool,
    /// True when per-provider timestamps / epochs are preserved.
    pub preserves_timestamps_epochs: bool,
    /// True when suppression / baseline state is preserved.
    pub preserves_suppression_baseline: bool,
    /// True when related symbol / file evidence is preserved.
    pub preserves_related_evidence: bool,
    /// Route that opens the cluster detail sheet.
    pub detail_sheet_route_class: DetailSheetRouteClass,
    /// Semantic-layer banner the surface may show.
    pub semantic_layer_banner_class: SemanticLayerBannerClass,
    /// Freshness label the cluster may claim.
    pub freshness_class: FreshnessClass,
    /// Scope label the cluster may claim.
    pub scope_label_class: ScopeLabelClass,
    /// Acting (winning) provider family for the cluster.
    pub acting_provider_family_class: ProviderFamilyClass,
    /// Conflict class for the cluster.
    pub conflict_class: ConflictClass,
    /// Whether the losing providers stay inspectable.
    pub provider_disagreement_visibility_class: ProviderDisagreementVisibilityClass,
    /// What fix the cluster offers.
    pub fix_offer_class: FixOfferClass,
    /// Typed preview completeness for a mutating fix (or `not_applicable`).
    pub preview_completeness_class: CompletenessClass,
    /// Rollback checkpoint ref required for a mutating fix.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
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
    /// Disclosure ref required when the row is narrowed, declares a known
    /// limit, or binds a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// True when raw source bodies are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority / credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl DiagnosticClusterRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
            && self.acting_provider_family_class.is_concrete()
            && self.semantic_layer_banner_class.is_bound()
            && self.freshness_class.is_bound()
            && self.scope_label_class.is_bound()
            && self.has_concrete_sources()
    }

    fn concrete_sources(&self) -> Vec<DiagnosticSourceClass> {
        self.diagnostic_source_classes
            .iter()
            .copied()
            .filter(|source| source.is_concrete())
            .collect()
    }

    fn has_concrete_sources(&self) -> bool {
        self.diagnostic_source_classes
            .iter()
            .any(|source| source.is_concrete())
    }

    /// True when more than one concrete provider source converged here.
    fn is_multi_source(&self) -> bool {
        self.concrete_sources().len() > 1
    }

    /// True when this cluster records a provider disagreement.
    fn has_disagreement(&self) -> bool {
        matches!(
            self.conflict_class,
            ConflictClass::ArbitratedWinnerLoserPreserved
                | ConflictClass::UnresolvedDisagreementSurfaced
        )
    }

    /// True when this cluster mixes runtime, policy, and static source families
    /// and so must keep them differentiated.
    fn mixes_runtime_policy_static(&self) -> bool {
        let mut runtime = false;
        let mut policy = false;
        let mut statik = false;
        for source in self.concrete_sources() {
            match source_kind(source) {
                SourceKind::Runtime => runtime = true,
                SourceKind::Policy => policy = true,
                SourceKind::Static => statik = true,
                SourceKind::Other => {}
            }
        }
        runtime && policy && statik
    }

    fn preserves_all_detail(&self) -> bool {
        self.preserves_per_provider_detail
            && self.preserves_timestamps_epochs
            && self.preserves_suppression_baseline
            && self.preserves_related_evidence
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticClusterConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Surface packet id consumed by the projection.
    pub surface_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the host-surface vocabulary is preserved verbatim.
    pub preserves_surface_vocabulary: bool,
    /// True when the cluster-lane vocabulary is preserved verbatim.
    pub preserves_cluster_lane_vocabulary: bool,
    /// True when the support-class vocabulary is preserved verbatim.
    pub preserves_support_class_vocabulary: bool,
    /// True when the diagnostic-source vocabulary is preserved verbatim.
    pub preserves_diagnostic_source_vocabulary: bool,
    /// True when the cluster-provenance vocabulary is preserved verbatim.
    pub preserves_cluster_provenance_vocabulary: bool,
    /// True when the source-differentiation vocabulary is preserved verbatim.
    pub preserves_source_differentiation_vocabulary: bool,
    /// True when the detail-sheet route vocabulary is preserved verbatim.
    pub preserves_detail_sheet_route_vocabulary: bool,
    /// True when the semantic-layer banner vocabulary is preserved verbatim.
    pub preserves_semantic_layer_banner_vocabulary: bool,
    /// True when the freshness vocabulary is preserved verbatim.
    pub preserves_freshness_vocabulary: bool,
    /// True when the scope-label vocabulary is preserved verbatim.
    pub preserves_scope_label_vocabulary: bool,
    /// True when the provider-family vocabulary is preserved verbatim.
    pub preserves_provider_family_vocabulary: bool,
    /// True when the conflict vocabulary is preserved verbatim.
    pub preserves_conflict_vocabulary: bool,
    /// True when the provider-disagreement-visibility vocabulary is preserved.
    pub preserves_provider_disagreement_visibility_vocabulary: bool,
    /// True when the fix-offer vocabulary is preserved verbatim.
    pub preserves_fix_offer_vocabulary: bool,
    /// True when the completeness vocabulary is preserved verbatim.
    pub preserves_completeness_vocabulary: bool,
    /// True when the evidence-class vocabulary is preserved verbatim.
    pub preserves_evidence_class_vocabulary: bool,
    /// True when the known-limit vocabulary is preserved verbatim.
    pub preserves_known_limit_vocabulary: bool,
    /// True when the downgrade-automation vocabulary is preserved verbatim.
    pub preserves_downgrade_automation_vocabulary: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority / credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl DiagnosticClusterConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.surface_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_surface_vocabulary
            && self.preserves_cluster_lane_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_diagnostic_source_vocabulary
            && self.preserves_cluster_provenance_vocabulary
            && self.preserves_source_differentiation_vocabulary
            && self.preserves_detail_sheet_route_vocabulary
            && self.preserves_semantic_layer_banner_vocabulary
            && self.preserves_freshness_vocabulary
            && self.preserves_scope_label_vocabulary
            && self.preserves_provider_family_vocabulary
            && self.preserves_conflict_vocabulary
            && self.preserves_provider_disagreement_visibility_vocabulary
            && self.preserves_fix_offer_vocabulary
            && self.preserves_completeness_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`DiagnosticClusterSemanticLayerTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticClusterSemanticLayerTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Surfaces the packet covers.
    #[serde(default)]
    pub covered_surfaces: Vec<SurfaceClass>,
    /// Diagnostic-cluster rows.
    #[serde(default)]
    pub rows: Vec<DiagnosticClusterRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<DiagnosticClusterConsumerProjection>,
    /// Source contracts (matrix packet / docs / schema / fixtures) consumed.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet binding the clustered diagnostic surface, semantic-layer
/// banner, freshness/scope labels, and detail-sheet model across the M5
/// notebook, framework, preview, and generated-code surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticClusterSemanticLayerTruthPacket {
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
    /// Surfaces the packet covers.
    #[serde(default)]
    pub covered_surfaces: Vec<SurfaceClass>,
    /// Diagnostic-cluster rows.
    #[serde(default)]
    pub rows: Vec<DiagnosticClusterRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<DiagnosticClusterConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl DiagnosticClusterSemanticLayerTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: DiagnosticClusterSemanticLayerTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_surfaces: input.covered_surfaces,
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

    /// Re-validates the packet against stable diagnostic-cluster invariants.
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

    /// Returns the unique host-surface tokens observed across rows.
    pub fn surface_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.surface_class.as_str())
    }

    /// Returns the unique cluster-lane tokens observed across rows.
    pub fn cluster_lane_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.cluster_lane_class.as_str())
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.support_class.as_str())
    }

    /// Returns the unique diagnostic-source tokens observed across rows.
    pub fn diagnostic_source_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            for source in &row.diagnostic_source_classes {
                set.insert(source.as_str());
            }
        }
        set.into_iter().collect()
    }

    /// Returns the unique cluster-provenance tokens observed across rows.
    pub fn cluster_provenance_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.cluster_provenance_class.as_str())
    }

    /// Returns the unique source-differentiation tokens observed across rows.
    pub fn source_differentiation_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.source_differentiation_class.as_str())
    }

    /// Returns the unique detail-sheet route tokens observed across rows.
    pub fn detail_sheet_route_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.detail_sheet_route_class.as_str())
    }

    /// Returns the unique semantic-layer banner tokens observed across rows.
    pub fn semantic_layer_banner_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.semantic_layer_banner_class.as_str())
    }

    /// Returns the unique freshness tokens observed across rows.
    pub fn freshness_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.freshness_class.as_str())
    }

    /// Returns the unique scope-label tokens observed across rows.
    pub fn scope_label_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.scope_label_class.as_str())
    }

    /// Returns the unique provider-family tokens observed across rows.
    pub fn provider_family_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.acting_provider_family_class.as_str())
    }

    /// Returns the unique conflict tokens observed across rows.
    pub fn conflict_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.conflict_class.as_str())
    }

    /// Returns the unique provider-disagreement-visibility tokens across rows.
    pub fn provider_disagreement_visibility_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.provider_disagreement_visibility_class.as_str())
    }

    /// Returns the unique fix-offer tokens observed across rows.
    pub fn fix_offer_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.fix_offer_class.as_str())
    }

    /// Returns the unique completeness tokens observed across rows.
    pub fn completeness_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.preview_completeness_class.as_str())
    }

    /// Returns the unique evidence-class tokens observed across rows.
    pub fn evidence_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.evidence_class.as_str())
    }

    /// Returns the unique known-limit tokens observed across rows.
    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.known_limit_class.as_str())
    }

    /// Returns the unique downgrade-automation tokens observed across rows.
    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.downgrade_automation_class.as_str())
    }

    fn unique_tokens(
        &self,
        project: impl Fn(&DiagnosticClusterRow) -> &'static str,
    ) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(project(row));
        }
        set.into_iter().collect()
    }

    /// Builds a support export wrapping the exact packet shown to product
    /// surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> DiagnosticClusterSemanticLayerTruthSupportExport {
        DiagnosticClusterSemanticLayerTruthSupportExport {
            record_kind: DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_SUPPORT_EXPORT_RECORD_KIND
                .to_owned(),
            schema_version: DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            surface_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            cluster_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "cluster packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "cluster packet has the wrong schema version",
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
        if self.covered_surfaces.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingSurfaceCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered surface",
            ));
        }

        for surface in SurfaceClass::REQUIRED {
            let present = self.rows.iter().any(|row| row.surface_class == surface);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingSurfaceCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers surface {}", surface.as_str()),
                ));
            }
        }
        for lane in ClusterLaneClass::REQUIRED {
            let present = self.rows.iter().any(|row| row.cluster_lane_class == lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingClusterLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers cluster lane {}", lane.as_str()),
                ));
            }
        }

        for row in &self.rows {
            self.append_per_row_findings(row, &mut findings);
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

    fn append_per_row_findings(
        &self,
        row: &DiagnosticClusterRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                format!("row {} identity or timestamp is empty", row.row_id),
            ));
        }

        // Boundary discipline: no raw bodies, secrets, or ambient authority.
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

        // Binding discipline.
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
        if !row.acting_provider_family_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingProviderFamily,
                FindingSeverity::Blocker,
                format!(
                    "row {} must name a concrete acting provider family",
                    row.row_id
                ),
            ));
        }
        if !row.has_concrete_sources() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingDiagnosticSources,
                FindingSeverity::Blocker,
                format!(
                    "row {} lists no concrete diagnostic source families",
                    row.row_id
                ),
            ));
        }
        if !row.semantic_layer_banner_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingSemanticLayerBanner,
                FindingSeverity::Blocker,
                format!("row {} has no bound semantic-layer banner", row.row_id),
            ));
        }
        if !row.freshness_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingFreshnessLabel,
                FindingSeverity::Blocker,
                format!("row {} has no bound freshness label", row.row_id),
            ));
        }
        if !row.scope_label_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingScopeLabel,
                FindingSeverity::Blocker,
                format!("row {} has no bound scope label", row.row_id),
            ));
        }
        if row.evidence_refs.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingEvidenceRefs,
                FindingSeverity::Blocker,
                format!("row {} carries no evidence refs", row.row_id),
            ));
        }

        if matches!(row.support_class, SupportClass::Certified) && !row.all_bindings_satisfied() {
            findings.push(ValidationFinding::new(
                FindingKind::CertifiedWithUnboundBinding,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims certified while a binding (support, provider family, diagnostic sources, banner, freshness, scope, known limit, downgrade automation, or evidence) is unbound",
                    row.row_id
                ),
            ));
        }

        // Disclosure discipline.
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

        self.append_cluster_findings(row, findings);
        self.append_banner_findings(row, findings);
        self.append_fix_findings(row, findings);

        if matches!(row.confidence_class, ConfidenceClass::LowConfidence)
            && matches!(row.support_class, SupportClass::Certified)
        {
            findings.push(ValidationFinding::new(
                FindingKind::CertifiedAtLowConfidence,
                FindingSeverity::Warning,
                format!(
                    "row {} claims certified at low_confidence; narrowing until evidence grows",
                    row.row_id
                ),
            ));
        }
    }

    fn append_cluster_findings(
        &self,
        row: &DiagnosticClusterRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        // Deduplication must preserve per-provider detail, timestamps/epochs,
        // suppression/baseline state, and related evidence.
        if row.is_multi_source()
            && (row.cluster_provenance_class.is_collapsed() || !row.preserves_all_detail())
        {
            findings.push(ValidationFinding::new(
                FindingKind::ClusterProvenanceCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "row {} clusters multiple providers but drops per-provider detail, timestamps, suppression/baseline state, or related evidence",
                    row.row_id
                ),
            ));
        }

        // Runtime evidence, policy/security findings, and static analysis must
        // not collapse into one undifferentiated row.
        if row.source_differentiation_class.is_fused()
            || (row.mixes_runtime_policy_static()
                && !row.source_differentiation_class.is_differentiated())
        {
            findings.push(ValidationFinding::new(
                FindingKind::SourcesFusedUndifferentiated,
                FindingSeverity::Blocker,
                format!(
                    "row {} fuses runtime, policy, and static findings into one undifferentiated row",
                    row.row_id
                ),
            ));
        }

        // The losing provider must stay inspectable; a disagreement must never
        // collapse into a ranking-only result.
        if row.has_disagreement() && row.provider_disagreement_visibility_class.loses_losers() {
            findings.push(ValidationFinding::new(
                FindingKind::LosingProviderCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "row {} records a disagreement but collapses the losing provider into ranking-only output",
                    row.row_id
                ),
            ));
        }

        // An opaque spinner can never stand in for a real detail-sheet route.
        if matches!(
            row.detail_sheet_route_class,
            DetailSheetRouteClass::OpaqueSpinner
        ) {
            findings.push(ValidationFinding::new(
                FindingKind::OpaqueDetailSheetRoute,
                FindingSeverity::Blocker,
                format!(
                    "row {} uses an opaque spinner in place of a detail-sheet route",
                    row.row_id
                ),
            ));
        }

        // A multi-source or disagreeing cluster owes an inspectable detail sheet.
        if (row.is_multi_source() || row.has_disagreement())
            && !row.detail_sheet_route_class.is_inspectable()
        {
            findings.push(ValidationFinding::new(
                FindingKind::DetailSheetRouteMissing,
                FindingSeverity::Blocker,
                format!(
                    "row {} clusters multiple providers or records a disagreement but offers no inspectable detail sheet",
                    row.row_id
                ),
            ));
        }
    }

    fn append_banner_findings(
        &self,
        row: &DiagnosticClusterRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        // A `semantic` banner may only be claimed on live or warm evidence.
        if row.semantic_layer_banner_class.claims_full_semantic()
            && !row.freshness_class.is_live_or_warm()
        {
            findings.push(ValidationFinding::new(
                FindingKind::SemanticLayerOverclaimed,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims a semantic banner on {} evidence instead of a degraded banner",
                    row.row_id,
                    row.freshness_class.as_str()
                ),
            ));
        }

        // A whole-workspace scope must not rest on stale or non-semantic
        // evidence.
        if row.scope_label_class.is_whole_workspace()
            && (row.freshness_class.is_stale()
                || row
                    .semantic_layer_banner_class
                    .rests_on_non_semantic_evidence())
        {
            findings.push(ValidationFinding::new(
                FindingKind::OverclaimedScopeOnStaleEvidence,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims whole-workspace scope on stale or non-semantic evidence",
                    row.row_id
                ),
            ));
        }
    }

    fn append_fix_findings(
        &self,
        row: &DiagnosticClusterRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        // A fix may not be offered without naming the acting provider and the
        // freshness/scope posture.
        if row.fix_offer_class.offers_fix()
            && (!row.acting_provider_family_class.is_concrete()
                || !row.freshness_class.is_bound()
                || !row.scope_label_class.is_bound())
        {
            findings.push(ValidationFinding::new(
                FindingKind::FixOfferedWithoutProviderOrFreshness,
                FindingSeverity::Blocker,
                format!(
                    "row {} offers a fix without naming the acting provider and freshness/scope posture",
                    row.row_id
                ),
            ));
        }

        // A mutating fix must bind typed preview completeness and a rollback
        // checkpoint; this preserves the launch-language refactor safety model
        // while extending it to M5-only artifacts and framework packs.
        if row.fix_offer_class.is_mutating()
            && (!row.preview_completeness_class.is_concrete()
                || row.rollback_checkpoint_ref.is_none())
        {
            findings.push(ValidationFinding::new(
                FindingKind::MutatingFixBypassesPreview,
                FindingSeverity::Blocker,
                format!(
                    "row {} offers a mutating fix without typed preview completeness and a rollback checkpoint",
                    row.row_id
                ),
            ));
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
pub struct DiagnosticClusterSemanticLayerTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub surface_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials / authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub cluster_packet: DiagnosticClusterSemanticLayerTruthPacket,
}

impl DiagnosticClusterSemanticLayerTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == DIAGNOSTIC_CLUSTER_SEMANTIC_LAYER_TRUTH_SCHEMA_VERSION
            && self.surface_packet_id_ref == self.cluster_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.cluster_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable cluster packet.
#[derive(Debug)]
pub enum DiagnosticClusterSemanticLayerTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for DiagnosticClusterSemanticLayerTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(formatter, "cluster packet parse failed: {error}"),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "cluster packet failed validation: {tokens}")
            }
        }
    }
}

impl Error for DiagnosticClusterSemanticLayerTruthArtifactError {}

/// Returns the checked-in stable diagnostic-cluster semantic-layer truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or
/// validate.
pub fn current_stable_diagnostic_cluster_semantic_layer_truth_packet() -> Result<
    DiagnosticClusterSemanticLayerTruthPacket,
    DiagnosticClusterSemanticLayerTruthArtifactError,
> {
    let packet: DiagnosticClusterSemanticLayerTruthPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/language/m5/diagnostic_cluster_semantic_layer_truth_packet.json"
        )))
        .map_err(DiagnosticClusterSemanticLayerTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(DiagnosticClusterSemanticLayerTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests;
