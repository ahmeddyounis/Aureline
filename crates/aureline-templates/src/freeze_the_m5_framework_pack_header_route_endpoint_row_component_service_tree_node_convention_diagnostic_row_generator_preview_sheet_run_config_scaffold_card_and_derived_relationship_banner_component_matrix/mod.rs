//! Frozen M5 framework-pack-header, route-or-endpoint-row, component-or-service-tree-node,
//! convention-diagnostic-row, generator-preview-sheet, run-config-scaffold-card, and
//! derived-relationship-banner component matrix.
//!
//! This module locks Aureline's reusable framework-aware and topology-explorer components into
//! one export-safe packet. Every framework-aware insight M5 claims that still drifts too easily
//! by framework-pack, route-explorer, topology-explorer, convention-diagnostics, or
//! generator-review surface — the framework pack header, the route / endpoint row, the
//! component / service tree node, the convention-diagnostic row, the generator preview sheet,
//! the run-config scaffold card, and the derived-relationship banner — is named once here and
//! constrained by the same framework pack identity / version / support class, authored-versus-
//! generated status, exact-versus-heuristic-versus-runtime-confirmed certainty, proving-source
//! linkage, local-versus-remote execution boundary, file / dependency / config impact, and
//! rollback-or-regenerate posture regardless of the surface family that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components* themselves: the
//! component families; the one controlled certainty vocabulary every consumer binds
//! (`core_native`, `framework_pack`, `bridge`, `heuristic_convention`, `verified`,
//! `derived_by_convention`, `runtime_confirmed`, `partial`); the pack support classes and pack
//! identity states the pack header binds; the route evidence classes and authorship the
//! route / endpoint row binds; the topology node kinds and evidence classes the component /
//! service tree node binds; the convention confidence classes and diagnostic severities the
//! convention-diagnostic row binds; the generator impact classes and apply postures the
//! generator preview sheet binds; the execution boundary classes and mutation classes the
//! run-config scaffold card binds; the derived-relationship classes and proving states the
//! derived-relationship banner binds; the deployment lines every component must survive; the
//! non-visual accessibility routes; and the mandatory labels every component must be able to
//! show. It does not re-architect the framework analyzers, preview runtimes, generator
//! backends, or topology extractors that already own those records — it is the shared
//! framework-component contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 framework-pack, route
//! explorer, topology explorer, convention-diagnostics, or generator-review surface may publish
//! a framework pack header, a route / endpoint row, a component / service tree node, a
//! convention-diagnostic row, a generator preview sheet, a run-config scaffold card, or a
//! derived-relationship banner. Every consumer reads this packet so one pack header names which
//! pack and version is active and how it is supported, one route row names whether a route is
//! exact from source or a heuristic convention, one tree node names whether a relationship is
//! exact, heuristic, or runtime-confirmed, one convention row names its confidence and its
//! proving files, one generator sheet never implies a no-op write when it changes config or
//! dependencies, one run-config card never hides the local / container / SSH / managed boundary
//! behind framework convenience language, and one derived-relationship banner keeps its
//! derived-state label and proving source explicit. No M5 lane invents a second framework
//! grammar or an alternate label for a governed source, certainty, boundary, impact, or
//! recovery state.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5FrameworkComponentVocabularySet`] rather than minted per surface. Raw file bodies, raw
//! diffs, raw local paths, repository URLs, credentials, and secrets stay outside the export
//! boundary.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5FrameworkComponentMatrixPacket`].
pub const M5_FRAMEWORK_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix";

/// Schema version for M5 framework component-matrix records.
pub const M5_FRAMEWORK_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined framework-component boundary schema.
pub const M5_FRAMEWORK_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-framework-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_FRAMEWORK_COMPONENT_DOC_REF: &str =
    "docs/frameworks/m5/m5_framework_component_matrix.md";

/// Repo-relative path of the per-component framework-pack-header schema.
pub const M5_FRAMEWORK_PACK_HEADER_SCHEMA_REF: &str =
    "schemas/ui/m5-framework-pack-header.schema.json";

/// Repo-relative path of the per-component route-endpoint-row schema.
pub const M5_ROUTE_ENDPOINT_ROW_SCHEMA_REF: &str = "schemas/ui/m5-route-endpoint-row.schema.json";

/// Repo-relative path of the per-component component-service-tree-node schema.
pub const M5_COMPONENT_SERVICE_TREE_NODE_SCHEMA_REF: &str =
    "schemas/ui/m5-component-service-tree-node.schema.json";

/// Repo-relative path of the per-component convention-diagnostic-row schema.
pub const M5_CONVENTION_DIAGNOSTIC_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-convention-diagnostic-row.schema.json";

/// Repo-relative path of the per-component generator-preview-sheet schema.
pub const M5_GENERATOR_PREVIEW_SHEET_SCHEMA_REF: &str =
    "schemas/ui/m5-generator-preview-sheet.schema.json";

/// Repo-relative path of the per-component run-config-scaffold-card schema.
pub const M5_RUN_CONFIG_SCAFFOLD_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-run-config-scaffold-card.schema.json";

/// Repo-relative path of the per-component derived-relationship-banner schema.
pub const M5_DERIVED_RELATIONSHIP_BANNER_SCHEMA_REF: &str =
    "schemas/ui/m5-derived-relationship-banner.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_FRAMEWORK_COMPONENT_FIXTURE_DIR: &str = "fixtures/ui/m5-framework-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_FRAMEWORK_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-framework-component-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_FRAMEWORK_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-framework-component-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_FRAMEWORK_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-framework-component-matrix.md";

/// One of the seven governed framework-component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkComponentFamily {
    /// A framework pack header carrying its pack identity, version, and support class.
    FrameworkPackHeader,
    /// A route / endpoint row carrying its route evidence class and authorship.
    RouteEndpointRow,
    /// A component / service tree node carrying its node kind and evidence class.
    ComponentServiceTreeNode,
    /// A convention-diagnostic row carrying its confidence class and diagnostic severity.
    ConventionDiagnosticRow,
    /// A generator preview sheet carrying its impact class and apply posture.
    GeneratorPreviewSheet,
    /// A run-config scaffold card carrying its execution boundary class and mutation class.
    RunConfigScaffoldCard,
    /// A derived-relationship banner carrying its derived-relationship class and proving state.
    DerivedRelationshipBanner,
}

impl M5FrameworkComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FrameworkPackHeader,
        Self::RouteEndpointRow,
        Self::ComponentServiceTreeNode,
        Self::ConventionDiagnosticRow,
        Self::GeneratorPreviewSheet,
        Self::RunConfigScaffoldCard,
        Self::DerivedRelationshipBanner,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkPackHeader => "framework_pack_header",
            Self::RouteEndpointRow => "route_endpoint_row",
            Self::ComponentServiceTreeNode => "component_service_tree_node",
            Self::ConventionDiagnosticRow => "convention_diagnostic_row",
            Self::GeneratorPreviewSheet => "generator_preview_sheet",
            Self::RunConfigScaffoldCard => "run_config_scaffold_card",
            Self::DerivedRelationshipBanner => "derived_relationship_banner",
        }
    }

    /// `true` when this family is a framework pack header and must therefore declare its pack
    /// support classes and pack identity states.
    pub const fn is_framework_pack_header(self) -> bool {
        matches!(self, Self::FrameworkPackHeader)
    }

    /// `true` when this family is a route / endpoint row and must therefore declare its route
    /// evidence classes and authorship states.
    pub const fn is_route_endpoint_row(self) -> bool {
        matches!(self, Self::RouteEndpointRow)
    }

    /// `true` when this family is a component / service tree node and must therefore declare its
    /// topology node kinds and evidence classes.
    pub const fn is_component_service_tree_node(self) -> bool {
        matches!(self, Self::ComponentServiceTreeNode)
    }

    /// `true` when this family is a convention-diagnostic row and must therefore declare its
    /// convention confidence classes and diagnostic severities.
    pub const fn is_convention_diagnostic_row(self) -> bool {
        matches!(self, Self::ConventionDiagnosticRow)
    }

    /// `true` when this family is a generator preview sheet and must therefore declare its
    /// generator impact classes and apply postures.
    pub const fn is_generator_preview_sheet(self) -> bool {
        matches!(self, Self::GeneratorPreviewSheet)
    }

    /// `true` when this family is a run-config scaffold card and must therefore declare its
    /// execution boundary classes and mutation classes.
    pub const fn is_run_config_scaffold_card(self) -> bool {
        matches!(self, Self::RunConfigScaffoldCard)
    }

    /// `true` when this family is a derived-relationship banner and must therefore declare its
    /// derived-relationship classes and proving states.
    pub const fn is_derived_relationship_banner(self) -> bool {
        matches!(self, Self::DerivedRelationshipBanner)
    }
}

/// The one controlled certainty vocabulary every framework-component consumer binds. These are
/// the exact acceptance-criteria labels so no surface invents a parallel word for a core-native,
/// framework-pack, bridge, or heuristic-convention source, or for a verified, derived-by-
/// convention, runtime-confirmed, or partial certainty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkCertaintyDisposition {
    /// Core-native behavior owned by Aureline directly.
    CoreNative,
    /// Behavior provided by a framework pack.
    FrameworkPack,
    /// Bridge behavior, not exact core-native or first-party pack support.
    Bridge,
    /// A heuristic convention rather than an exact fact.
    HeuristicConvention,
    /// Verified exact truth from source.
    Verified,
    /// Derived by convention rather than proven from source.
    DerivedByConvention,
    /// Confirmed by observing the running application.
    RuntimeConfirmed,
    /// Partial truth; evidence is incomplete.
    Partial,
}

impl M5FrameworkCertaintyDisposition {
    /// Every certainty disposition, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CoreNative,
        Self::FrameworkPack,
        Self::Bridge,
        Self::HeuristicConvention,
        Self::Verified,
        Self::DerivedByConvention,
        Self::RuntimeConfirmed,
        Self::Partial,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreNative => "core_native",
            Self::FrameworkPack => "framework_pack",
            Self::Bridge => "bridge",
            Self::HeuristicConvention => "heuristic_convention",
            Self::Verified => "verified",
            Self::DerivedByConvention => "derived_by_convention",
            Self::RuntimeConfirmed => "runtime_confirmed",
            Self::Partial => "partial",
        }
    }
}

/// Controlled framework-pack support class — how the active pack behind a framework pack header
/// is supported, so bridge or heuristic behavior never reads as exact first-party support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkPackSupportClass {
    /// Officially supported.
    OfficiallySupported,
    /// Community-supported, best effort.
    CommunitySupported,
    /// Experimental.
    Experimental,
    /// Bridge-only behavior, not exact first-party generation.
    BridgeOnly,
    /// Deprecated.
    Deprecated,
    /// Unsupported.
    Unsupported,
}

impl M5FrameworkPackSupportClass {
    /// Every pack support class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OfficiallySupported,
        Self::CommunitySupported,
        Self::Experimental,
        Self::BridgeOnly,
        Self::Deprecated,
        Self::Unsupported,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfficiallySupported => "officially_supported",
            Self::CommunitySupported => "community_supported",
            Self::Experimental => "experimental",
            Self::BridgeOnly => "bridge_only",
            Self::Deprecated => "deprecated",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Controlled framework-pack identity state — how firmly a framework pack header pins the active
/// pack and version, so a card never leaves which pack / version is active implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkPackIdentityState {
    /// Pack identified and versioned.
    IdentifiedVersioned,
    /// Pack version explicitly pinned.
    VersionPinned,
    /// Pack version drifted from the pinned version.
    VersionDrifted,
    /// Multiple candidate packs detected.
    MultipleDetected,
    /// Pack detected but unversioned.
    Unversioned,
    /// Pack unknown.
    UnknownPack,
}

impl M5FrameworkPackIdentityState {
    /// Every pack identity state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::IdentifiedVersioned,
        Self::VersionPinned,
        Self::VersionDrifted,
        Self::MultipleDetected,
        Self::Unversioned,
        Self::UnknownPack,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentifiedVersioned => "identified_versioned",
            Self::VersionPinned => "version_pinned",
            Self::VersionDrifted => "version_drifted",
            Self::MultipleDetected => "multiple_detected",
            Self::Unversioned => "unversioned",
            Self::UnknownPack => "unknown_pack",
        }
    }
}

/// Controlled route evidence class — how a route / endpoint row knows a route, so a heuristic
/// route never masquerades as an exact one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RouteEvidenceClass {
    /// Exact, read directly from source.
    ExactFromSource,
    /// Inferred from a heuristic convention.
    HeuristicConvention,
    /// Confirmed by observing the running application.
    RuntimeConfirmed,
    /// Derived by convention rather than proven from source.
    DerivedByConvention,
    /// Partial evidence only.
    PartialEvidence,
    /// Unresolved.
    Unresolved,
}

impl M5RouteEvidenceClass {
    /// Every route evidence class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExactFromSource,
        Self::HeuristicConvention,
        Self::RuntimeConfirmed,
        Self::DerivedByConvention,
        Self::PartialEvidence,
        Self::Unresolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactFromSource => "exact_from_source",
            Self::HeuristicConvention => "heuristic_convention",
            Self::RuntimeConfirmed => "runtime_confirmed",
            Self::DerivedByConvention => "derived_by_convention",
            Self::PartialEvidence => "partial_evidence",
            Self::Unresolved => "unresolved",
        }
    }
}

/// Controlled route authorship — whether a route / endpoint was authored or generated, so a row
/// never leaves the authored-versus-generated boundary implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RouteAuthorship {
    /// Hand-authored.
    Authored,
    /// Generated by a tool.
    Generated,
    /// Generated then hand-edited.
    GeneratedThenEdited,
    /// Provided by the framework itself.
    FrameworkProvided,
    /// Runtime-only, no source form.
    RuntimeOnly,
    /// Unknown origin.
    UnknownOrigin,
}

impl M5RouteAuthorship {
    /// Every route authorship, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Authored,
        Self::Generated,
        Self::GeneratedThenEdited,
        Self::FrameworkProvided,
        Self::RuntimeOnly,
        Self::UnknownOrigin,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authored => "authored",
            Self::Generated => "generated",
            Self::GeneratedThenEdited => "generated_then_edited",
            Self::FrameworkProvided => "framework_provided",
            Self::RuntimeOnly => "runtime_only",
            Self::UnknownOrigin => "unknown_origin",
        }
    }
}

/// Controlled topology node kind — what a component / service tree node represents, so a node
/// never leaves what it is implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TopologyNodeKind {
    /// A component.
    ComponentNode,
    /// A service.
    ServiceNode,
    /// A module.
    ModuleNode,
    /// A dependency edge between nodes.
    DependencyEdge,
    /// An external boundary.
    ExternalBoundary,
    /// An unknown node.
    UnknownNode,
}

impl M5TopologyNodeKind {
    /// Every topology node kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ComponentNode,
        Self::ServiceNode,
        Self::ModuleNode,
        Self::DependencyEdge,
        Self::ExternalBoundary,
        Self::UnknownNode,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComponentNode => "component_node",
            Self::ServiceNode => "service_node",
            Self::ModuleNode => "module_node",
            Self::DependencyEdge => "dependency_edge",
            Self::ExternalBoundary => "external_boundary",
            Self::UnknownNode => "unknown_node",
        }
    }
}

/// Controlled topology evidence class — how a component / service tree node knows a
/// relationship, so an inferred relationship never masquerades as an exact one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TopologyEvidenceClass {
    /// Exact, read directly from source.
    ExactFromSource,
    /// Inferred by heuristic.
    HeuristicInferred,
    /// Confirmed by observing the running application.
    RuntimeConfirmed,
    /// Derived by convention rather than proven from source.
    DerivedByConvention,
    /// Partial evidence only.
    PartialEvidence,
    /// Unresolved.
    Unresolved,
}

impl M5TopologyEvidenceClass {
    /// Every topology evidence class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExactFromSource,
        Self::HeuristicInferred,
        Self::RuntimeConfirmed,
        Self::DerivedByConvention,
        Self::PartialEvidence,
        Self::Unresolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactFromSource => "exact_from_source",
            Self::HeuristicInferred => "heuristic_inferred",
            Self::RuntimeConfirmed => "runtime_confirmed",
            Self::DerivedByConvention => "derived_by_convention",
            Self::PartialEvidence => "partial_evidence",
            Self::Unresolved => "unresolved",
        }
    }
}

/// Controlled convention confidence class — how confident a convention-diagnostic row is, so a
/// heuristic guess never reads as a verified fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConventionConfidenceClass {
    /// Verified.
    Verified,
    /// High confidence.
    HighConfidence,
    /// A heuristic convention.
    HeuristicConvention,
    /// Derived by convention.
    DerivedByConvention,
    /// Low confidence.
    LowConfidence,
    /// Unknown.
    Unknown,
}

impl M5ConventionConfidenceClass {
    /// Every convention confidence class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Verified,
        Self::HighConfidence,
        Self::HeuristicConvention,
        Self::DerivedByConvention,
        Self::LowConfidence,
        Self::Unknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::HighConfidence => "high_confidence",
            Self::HeuristicConvention => "heuristic_convention",
            Self::DerivedByConvention => "derived_by_convention",
            Self::LowConfidence => "low_confidence",
            Self::Unknown => "unknown",
        }
    }
}

/// Controlled diagnostic severity — the severity a convention-diagnostic row reports, so a
/// suppressed or stale diagnostic is never presented as active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConventionDiagnosticSeverity {
    /// Error.
    Error,
    /// Warning.
    Warning,
    /// Hint.
    Hint,
    /// Informational.
    Info,
    /// Suppressed.
    Suppressed,
    /// Stale.
    Stale,
}

impl M5ConventionDiagnosticSeverity {
    /// Every diagnostic severity, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Error,
        Self::Warning,
        Self::Hint,
        Self::Info,
        Self::Suppressed,
        Self::Stale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Hint => "hint",
            Self::Info => "info",
            Self::Suppressed => "suppressed",
            Self::Stale => "stale",
        }
    }
}

/// Controlled generator impact class — what a generator preview sheet will change, so a
/// generator never implies a no-op write when it changes config or dependencies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GeneratorImpactClass {
    /// Writes files.
    FileWrite,
    /// Changes dependencies.
    DependencyChange,
    /// Changes configuration.
    ConfigChange,
    /// Changes a script or task.
    ScriptOrTaskChange,
    /// No change.
    NoChange,
    /// Unknown impact.
    UnknownImpact,
}

impl M5GeneratorImpactClass {
    /// Every generator impact class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FileWrite,
        Self::DependencyChange,
        Self::ConfigChange,
        Self::ScriptOrTaskChange,
        Self::NoChange,
        Self::UnknownImpact,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileWrite => "file_write",
            Self::DependencyChange => "dependency_change",
            Self::ConfigChange => "config_change",
            Self::ScriptOrTaskChange => "script_or_task_change",
            Self::NoChange => "no_change",
            Self::UnknownImpact => "unknown_impact",
        }
    }
}

/// Controlled generator apply posture — what a generator preview sheet permits, so no write
/// happens before review and rollback / regenerate stays explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GeneratorApplyPosture {
    /// A reviewable preview is ready.
    PreviewReady,
    /// Review is required before any write.
    ReviewRequired,
    /// Apply is ready after review.
    ApplyReady,
    /// Rollback is available.
    RollbackAvailable,
    /// Regenerate is available.
    RegenerateAvailable,
    /// Blocked.
    Blocked,
}

impl M5GeneratorApplyPosture {
    /// Every generator apply posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PreviewReady,
        Self::ReviewRequired,
        Self::ApplyReady,
        Self::RollbackAvailable,
        Self::RegenerateAvailable,
        Self::Blocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewReady => "preview_ready",
            Self::ReviewRequired => "review_required",
            Self::ApplyReady => "apply_ready",
            Self::RollbackAvailable => "rollback_available",
            Self::RegenerateAvailable => "regenerate_available",
            Self::Blocked => "blocked",
        }
    }
}

/// Controlled execution boundary class — where a run-config scaffold card's convenience action
/// will actually run, so the local / container / SSH / managed boundary never hides behind
/// framework convenience language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionBoundaryClass {
    /// A local process.
    LocalProcess,
    /// A container.
    Container,
    /// An SSH remote.
    SshRemote,
    /// A managed workspace.
    ManagedWorkspace,
    /// A cloud remote.
    CloudRemote,
    /// An unknown boundary.
    UnknownBoundary,
}

impl M5ExecutionBoundaryClass {
    /// Every execution boundary class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalProcess,
        Self::Container,
        Self::SshRemote,
        Self::ManagedWorkspace,
        Self::CloudRemote,
        Self::UnknownBoundary,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalProcess => "local_process",
            Self::Container => "container",
            Self::SshRemote => "ssh_remote",
            Self::ManagedWorkspace => "managed_workspace",
            Self::CloudRemote => "cloud_remote",
            Self::UnknownBoundary => "unknown_boundary",
        }
    }
}

/// Controlled run-config mutation class — what a run-config scaffold card writes, so a
/// convenience action never implies a no-op write when it creates or edits config or adds a
/// dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunConfigMutationClass {
    /// Creates a config file.
    CreatesConfigFile,
    /// Edits a config file.
    EditsConfigFile,
    /// Adds a dependency.
    AddsDependency,
    /// A no-write preview.
    NoWritePreview,
    /// Rollback is available.
    RollbackAvailable,
    /// Unknown mutation.
    UnknownMutation,
}

impl M5RunConfigMutationClass {
    /// Every run-config mutation class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CreatesConfigFile,
        Self::EditsConfigFile,
        Self::AddsDependency,
        Self::NoWritePreview,
        Self::RollbackAvailable,
        Self::UnknownMutation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CreatesConfigFile => "creates_config_file",
            Self::EditsConfigFile => "edits_config_file",
            Self::AddsDependency => "adds_dependency",
            Self::NoWritePreview => "no_write_preview",
            Self::RollbackAvailable => "rollback_available",
            Self::UnknownMutation => "unknown_mutation",
        }
    }
}

/// Controlled derived-relationship class — how a derived-relationship banner knows a
/// relationship, so a derived or inferred link never masquerades as an exact one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DerivedRelationshipClass {
    /// Exact, read directly from source.
    ExactFromSource,
    /// Inferred from the running application.
    InferredFromRuntime,
    /// A heuristic link.
    HeuristicLink,
    /// Derived by convention.
    DerivedByConvention,
    /// A partial link.
    PartialLink,
    /// An unresolved link.
    UnresolvedLink,
}

impl M5DerivedRelationshipClass {
    /// Every derived-relationship class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExactFromSource,
        Self::InferredFromRuntime,
        Self::HeuristicLink,
        Self::DerivedByConvention,
        Self::PartialLink,
        Self::UnresolvedLink,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactFromSource => "exact_from_source",
            Self::InferredFromRuntime => "inferred_from_runtime",
            Self::HeuristicLink => "heuristic_link",
            Self::DerivedByConvention => "derived_by_convention",
            Self::PartialLink => "partial_link",
            Self::UnresolvedLink => "unresolved_link",
        }
    }
}

/// Controlled relationship proving state — whether a derived-relationship banner links to its
/// proving source, so a derived state never leaves its proving evidence implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RelationshipProvingState {
    /// Linked to a proving source.
    ProvingSourceLinked,
    /// Partially linked to a proving source.
    SourceLinkedPartial,
    /// Runtime evidence only.
    RuntimeEvidenceOnly,
    /// Convention only.
    ConventionOnly,
    /// No proving source.
    NoProvingSource,
    /// Unknown proving.
    UnknownProving,
}

impl M5RelationshipProvingState {
    /// Every relationship proving state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProvingSourceLinked,
        Self::SourceLinkedPartial,
        Self::RuntimeEvidenceOnly,
        Self::ConventionOnly,
        Self::NoProvingSource,
        Self::UnknownProving,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvingSourceLinked => "proving_source_linked",
            Self::SourceLinkedPartial => "source_linked_partial",
            Self::RuntimeEvidenceOnly => "runtime_evidence_only",
            Self::ConventionOnly => "convention_only",
            Self::NoProvingSource => "no_proving_source",
            Self::UnknownProving => "unknown_proving",
        }
    }
}

/// Claimed M5 framework-aware / topology-explorer surface family that renders / consumes a
/// framework component. No component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkSurfaceFamily {
    /// The framework-pack surface.
    FrameworkPackSurface,
    /// The route-explorer surface.
    RouteExplorer,
    /// The topology-explorer surface.
    TopologyExplorer,
    /// The convention-diagnostics surface.
    ConventionDiagnostics,
    /// The generator-review surface.
    GeneratorReview,
    /// The CLI surface.
    CliSurface,
}

impl M5FrameworkSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FrameworkPackSurface,
        Self::RouteExplorer,
        Self::TopologyExplorer,
        Self::ConventionDiagnostics,
        Self::GeneratorReview,
        Self::CliSurface,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkPackSurface => "framework_pack_surface",
            Self::RouteExplorer => "route_explorer",
            Self::TopologyExplorer => "topology_explorer",
            Self::ConventionDiagnostics => "convention_diagnostics",
            Self::GeneratorReview => "generator_review",
            Self::CliSurface => "cli_surface",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's pack identity,
/// certainty, execution boundary, impact, or recovery truth never silently narrows or widens
/// between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5FrameworkDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkConsumerSurface {
    /// The framework-pack UI.
    FrameworkPackUi,
    /// The route-explorer UI.
    RouteExplorerUi,
    /// The topology-explorer UI.
    TopologyUi,
    /// The convention diagnostic-center UI.
    DiagnosticCenterUi,
    /// The generator-review UI.
    GeneratorReviewUi,
    /// The run-config UI.
    RunConfigUi,
    /// The editor gutter UI.
    EditorGutterUi,
    /// The CLI surface.
    CliSurface,
    /// The support export.
    SupportExport,
}

impl M5FrameworkConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::FrameworkPackUi,
        Self::RouteExplorerUi,
        Self::TopologyUi,
        Self::DiagnosticCenterUi,
        Self::GeneratorReviewUi,
        Self::RunConfigUi,
        Self::EditorGutterUi,
        Self::CliSurface,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkPackUi => "framework_pack_ui",
            Self::RouteExplorerUi => "route_explorer_ui",
            Self::TopologyUi => "topology_ui",
            Self::DiagnosticCenterUi => "diagnostic_center_ui",
            Self::GeneratorReviewUi => "generator_review_ui",
            Self::RunConfigUi => "run_config_ui",
            Self::EditorGutterUi => "editor_gutter_ui",
            Self::CliSurface => "cli_surface",
            Self::SupportExport => "support_export",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no framework truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5FrameworkAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed framework component must be able to show. The first three are hard
/// requirements on every component; the remaining three close the acceptance-criteria ambiguity
/// about pack source / certainty, execution boundary / impact, and proving-source / recovery
/// boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkRequiredLabel {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The pack source and the certainty class behind the component.
    PackSourceAndCertainty,
    /// The execution boundary and the file / dependency / config impact the component discloses.
    ExecutionBoundaryAndImpact,
    /// The proving-source linkage and the rollback / regenerate recovery path the component
    /// keeps.
    ProvingSourceAndRecoveryBoundary,
}

impl M5FrameworkRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::PackSourceAndCertainty,
        Self::ExecutionBoundaryAndImpact,
        Self::ProvingSourceAndRecoveryBoundary,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::PackSourceAndCertainty => "pack_source_and_certainty",
            Self::ExecutionBoundaryAndImpact => "execution_boundary_and_impact",
            Self::ProvingSourceAndRecoveryBoundary => "proving_source_and_recovery_boundary",
        }
    }
}

/// Qualification class for an M5 framework-component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5FrameworkQualificationClass {
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

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a framework component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkDowngradeTrigger {
    /// A pack header left its pack identity / version unstated.
    PackIdentityUnstated,
    /// A pack header left its support class unstated.
    SupportClassUnstated,
    /// A route / tree node left its exact-versus-heuristic certainty unstated.
    ExactVersusHeuristicUnstated,
    /// A route / tree node left its authored-versus-generated status unstated.
    AuthorshipUnstated,
    /// A run-config card left its local / container / SSH / managed boundary unstated.
    ExecutionBoundaryUnstated,
    /// A component left its file / dependency / config impact undisclosed.
    ImpactUndisclosed,
    /// A component omitted its proving-source linkage.
    ProvingSourceOmitted,
    /// A generator / run-config card omitted its rollback or regenerate path.
    RollbackPathOmitted,
    /// A banner left its derived state unlabeled.
    DerivedStateUnlabeled,
    /// A convention row overstated its confidence.
    ConventionConfidenceOverstated,
    /// A surface invented an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5FrameworkDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::PackIdentityUnstated,
        Self::SupportClassUnstated,
        Self::ExactVersusHeuristicUnstated,
        Self::AuthorshipUnstated,
        Self::ExecutionBoundaryUnstated,
        Self::ImpactUndisclosed,
        Self::ProvingSourceOmitted,
        Self::RollbackPathOmitted,
        Self::DerivedStateUnlabeled,
        Self::ConventionConfidenceOverstated,
        Self::AlternateStateLabelInvented,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackIdentityUnstated => "pack_identity_unstated",
            Self::SupportClassUnstated => "support_class_unstated",
            Self::ExactVersusHeuristicUnstated => "exact_versus_heuristic_unstated",
            Self::AuthorshipUnstated => "authorship_unstated",
            Self::ExecutionBoundaryUnstated => "execution_boundary_unstated",
            Self::ImpactUndisclosed => "impact_undisclosed",
            Self::ProvingSourceOmitted => "proving_source_omitted",
            Self::RollbackPathOmitted => "rollback_path_omitted",
            Self::DerivedStateUnlabeled => "derived_state_unlabeled",
            Self::ConventionConfidenceOverstated => "convention_confidence_overstated",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed framework-component family bound to the surface-specific
/// truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentRow {
    /// Governed component family.
    pub component_family: M5FrameworkComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5FrameworkQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 framework-aware / topology-explorer surface families that render / consume
    /// this component.
    pub surface_families: Vec<M5FrameworkSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5FrameworkDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5FrameworkRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5FrameworkRequiredLabel>,
    /// Controlled certainty dispositions this component binds (must be non-empty; drawn from the
    /// one shared [`M5FrameworkCertaintyDisposition`] vocabulary).
    pub dispositions: Vec<M5FrameworkCertaintyDisposition>,
    /// Pack support classes this component names (framework-pack-header only).
    pub pack_support_classes: Vec<M5FrameworkPackSupportClass>,
    /// Pack identity states this component names (framework-pack-header only).
    pub pack_identity_states: Vec<M5FrameworkPackIdentityState>,
    /// Route evidence classes this component names (route-endpoint-row only).
    pub route_evidence_classes: Vec<M5RouteEvidenceClass>,
    /// Route authorship states this component names (route-endpoint-row only).
    pub route_authorship_states: Vec<M5RouteAuthorship>,
    /// Topology node kinds this component names (component-service-tree-node only).
    pub topology_node_kinds: Vec<M5TopologyNodeKind>,
    /// Topology evidence classes this component names (component-service-tree-node only).
    pub topology_evidence_classes: Vec<M5TopologyEvidenceClass>,
    /// Convention confidence classes this component names (convention-diagnostic-row only).
    pub convention_confidence_classes: Vec<M5ConventionConfidenceClass>,
    /// Diagnostic severities this component names (convention-diagnostic-row only).
    pub diagnostic_severities: Vec<M5ConventionDiagnosticSeverity>,
    /// Generator impact classes this component names (generator-preview-sheet only).
    pub generator_impact_classes: Vec<M5GeneratorImpactClass>,
    /// Generator apply postures this component names (generator-preview-sheet only).
    pub generator_apply_postures: Vec<M5GeneratorApplyPosture>,
    /// Execution boundary classes this component names (run-config-scaffold-card only).
    pub execution_boundary_classes: Vec<M5ExecutionBoundaryClass>,
    /// Run-config mutation classes this component names (run-config-scaffold-card only).
    pub run_config_mutation_classes: Vec<M5RunConfigMutationClass>,
    /// Derived-relationship classes this component names (derived-relationship-banner only).
    pub derived_relationship_classes: Vec<M5DerivedRelationshipClass>,
    /// Relationship proving states this component names (derived-relationship-banner only).
    pub relationship_proving_states: Vec<M5RelationshipProvingState>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5FrameworkAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never hides its pack identity / version or support class.
    /// MUST be `false`.
    pub hides_pack_identity_version_or_support_class: bool,
    /// Hard invariant: this component never lets a heuristic route / tree masquerade as exact.
    /// MUST be `false`.
    pub lets_heuristic_masquerade_as_exact: bool,
    /// Hard invariant: this component never implies a no-op write while it mutates config or
    /// dependencies. MUST be `false`.
    pub implies_no_op_write_while_mutating_config_or_dependencies: bool,
    /// Hard invariant: this component never hides the local / container / SSH / managed
    /// execution boundary behind convenience language. MUST be `false`.
    pub hides_local_container_ssh_or_managed_boundary: bool,
    /// Hard invariant: this component never omits its proving-source linkage or rollback /
    /// regenerate path. MUST be `false`.
    pub omits_proving_source_or_rollback_path: bool,
    /// Hard invariant: this component never invents an alternate label for a governed state.
    /// MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl M5FrameworkComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5FrameworkRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5FrameworkRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.hides_pack_identity_version_or_support_class
            && !self.lets_heuristic_masquerade_as_exact
            && !self.implies_no_op_write_while_mutating_config_or_dependencies
            && !self.hides_local_container_ssh_or_managed_boundary
            && !self.omits_proving_source_or_rollback_path
            && !self.invents_alternate_state_label
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Certainty-disposition tokens (the one shared consumer vocabulary).
    pub dispositions: Vec<String>,
    /// Pack-support-class tokens.
    pub pack_support_classes: Vec<String>,
    /// Pack-identity-state tokens.
    pub pack_identity_states: Vec<String>,
    /// Route-evidence-class tokens.
    pub route_evidence_classes: Vec<String>,
    /// Route-authorship tokens.
    pub route_authorship_states: Vec<String>,
    /// Topology-node-kind tokens.
    pub topology_node_kinds: Vec<String>,
    /// Topology-evidence-class tokens.
    pub topology_evidence_classes: Vec<String>,
    /// Convention-confidence-class tokens.
    pub convention_confidence_classes: Vec<String>,
    /// Diagnostic-severity tokens.
    pub diagnostic_severities: Vec<String>,
    /// Generator-impact-class tokens.
    pub generator_impact_classes: Vec<String>,
    /// Generator-apply-posture tokens.
    pub generator_apply_postures: Vec<String>,
    /// Execution-boundary-class tokens.
    pub execution_boundary_classes: Vec<String>,
    /// Run-config-mutation-class tokens.
    pub run_config_mutation_classes: Vec<String>,
    /// Derived-relationship-class tokens.
    pub derived_relationship_classes: Vec<String>,
    /// Relationship-proving-state tokens.
    pub relationship_proving_states: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5FrameworkComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5FrameworkComponentFamily::ALL, |v| v.as_str()),
            dispositions: tokens(&M5FrameworkCertaintyDisposition::ALL, |v| v.as_str()),
            pack_support_classes: tokens(&M5FrameworkPackSupportClass::ALL, |v| v.as_str()),
            pack_identity_states: tokens(&M5FrameworkPackIdentityState::ALL, |v| v.as_str()),
            route_evidence_classes: tokens(&M5RouteEvidenceClass::ALL, |v| v.as_str()),
            route_authorship_states: tokens(&M5RouteAuthorship::ALL, |v| v.as_str()),
            topology_node_kinds: tokens(&M5TopologyNodeKind::ALL, |v| v.as_str()),
            topology_evidence_classes: tokens(&M5TopologyEvidenceClass::ALL, |v| v.as_str()),
            convention_confidence_classes: tokens(&M5ConventionConfidenceClass::ALL, |v| {
                v.as_str()
            }),
            diagnostic_severities: tokens(&M5ConventionDiagnosticSeverity::ALL, |v| v.as_str()),
            generator_impact_classes: tokens(&M5GeneratorImpactClass::ALL, |v| v.as_str()),
            generator_apply_postures: tokens(&M5GeneratorApplyPosture::ALL, |v| v.as_str()),
            execution_boundary_classes: tokens(&M5ExecutionBoundaryClass::ALL, |v| v.as_str()),
            run_config_mutation_classes: tokens(&M5RunConfigMutationClass::ALL, |v| v.as_str()),
            derived_relationship_classes: tokens(&M5DerivedRelationshipClass::ALL, |v| v.as_str()),
            relationship_proving_states: tokens(&M5RelationshipProvingState::ALL, |v| v.as_str()),
            surface_families: tokens(&M5FrameworkSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5FrameworkDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5FrameworkConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5FrameworkAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5FrameworkRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5FrameworkComponentGovernanceReview {
    /// The framework pack header shows its pack identity, version, and support class.
    pub pack_header_shows_identity_version_and_support: bool,
    /// The route / endpoint row shows its exact-versus-heuristic certainty and authorship.
    pub route_row_shows_exact_versus_heuristic_and_authorship: bool,
    /// The component / service tree node shows its node kind and evidence class.
    pub tree_node_shows_kind_and_evidence_class: bool,
    /// The convention-diagnostic row shows its confidence and proving source.
    pub convention_row_shows_confidence_and_proving_source: bool,
    /// The generator preview sheet shows its impact and rollback / regenerate posture.
    pub generator_sheet_shows_impact_and_rollback: bool,
    /// The run-config scaffold card shows its execution boundary and mutation.
    pub run_config_card_shows_execution_boundary_and_mutation: bool,
    /// The derived-relationship banner shows its derived state and proving source.
    pub derived_banner_shows_derived_state_and_proving_source: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// No heuristic route or tree masquerades as exact.
    pub no_heuristic_masquerades_as_exact: bool,
    /// No generator implies a no-op write while it mutates config or dependencies.
    pub no_generator_implies_no_op_while_mutating: bool,
    /// The local / container / SSH / managed execution boundary stays visible.
    pub execution_boundary_always_visible: bool,
    /// The proving-source linkage stays explicit.
    pub proving_source_always_linked: bool,
    /// The rollback / regenerate recovery path stays explicit.
    pub rollback_or_regenerate_always_explicit: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel framework vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentConsumerProjection {
    /// Framework-pack surfaces consume the pack identity and support vocabulary.
    pub framework_pack_surfaces_consume_identity_and_support_vocabulary: bool,
    /// Route and topology surfaces consume the exact-versus-heuristic vocabulary.
    pub route_and_topology_surfaces_consume_exact_versus_heuristic_vocabulary: bool,
    /// Convention surfaces consume the confidence and proving vocabulary.
    pub convention_surfaces_consume_confidence_and_proving_vocabulary: bool,
    /// Generator surfaces consume the impact and rollback vocabulary.
    pub generator_surfaces_consume_impact_and_rollback_vocabulary: bool,
    /// Run-config surfaces consume the execution-boundary vocabulary.
    pub run_config_surfaces_consume_execution_boundary_vocabulary: bool,
    /// Support / export reads a single canonical framework source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the framework-component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting framework-component audit for the lane.
    pub framework_component_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5FrameworkComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5FrameworkComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5FrameworkComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5FrameworkComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5FrameworkComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5FrameworkComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5FrameworkComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5FrameworkComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 framework-component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentMatrixPacket {
    /// Record kind; must equal [`M5_FRAMEWORK_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_FRAMEWORK_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5FrameworkComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5FrameworkComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5FrameworkComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5FrameworkComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5FrameworkComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5FrameworkComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5FrameworkComponentMatrixPacket {
    /// Builds an M5 framework-component matrix packet from stable-lane input.
    pub fn new(input: M5FrameworkComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_FRAMEWORK_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_FRAMEWORK_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 framework-component matrix invariants.
    pub fn validate(&self) -> Vec<M5FrameworkComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_FRAMEWORK_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5FrameworkComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_FRAMEWORK_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5FrameworkComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5FrameworkComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 framework component matrix packet serializes"),
        ) {
            violations.push(M5FrameworkComponentMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 framework component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,dispositions,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.dispositions, |v| v.as_str()),
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
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Framework-Pack-Header, Route-Endpoint-Row, Component-Service-Tree-Node, Convention-Diagnostic-Row, Generator-Preview-Sheet, Run-Config-Scaffold-Card, and Derived-Relationship-Banner Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Dispositions: {}\n",
            self.vocabulary_set.dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Dispositions: {}\n",
                row.dispositions
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
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

/// Errors emitted when reading the checked-in M5 framework matrix export.
#[derive(Debug)]
pub enum M5FrameworkComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5FrameworkComponentMatrixViolation>),
}

impl fmt::Display for M5FrameworkComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 framework component matrix export parse failed: {error}"
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
                    "m5 framework component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5FrameworkComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5FrameworkComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5FrameworkComponentMatrixViolation {
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
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A component row declares no dispositions.
    DispositionsMissing,
    /// A framework-pack-header component declares no pack support classes.
    PackSupportClassMissing,
    /// A framework-pack-header component declares no pack identity states.
    PackIdentityStateMissing,
    /// A route-endpoint-row component declares no route evidence classes.
    RouteEvidenceClassMissing,
    /// A route-endpoint-row component declares no route authorship states.
    RouteAuthorshipMissing,
    /// A component-service-tree-node component declares no topology node kinds.
    TopologyNodeKindMissing,
    /// A component-service-tree-node component declares no topology evidence classes.
    TopologyEvidenceClassMissing,
    /// A convention-diagnostic-row component declares no convention confidence classes.
    ConventionConfidenceClassMissing,
    /// A convention-diagnostic-row component declares no diagnostic severities.
    DiagnosticSeverityMissing,
    /// A generator-preview-sheet component declares no generator impact classes.
    GeneratorImpactClassMissing,
    /// A generator-preview-sheet component declares no generator apply postures.
    GeneratorApplyPostureMissing,
    /// A run-config-scaffold-card component declares no execution boundary classes.
    ExecutionBoundaryClassMissing,
    /// A run-config-scaffold-card component declares no run-config mutation classes.
    RunConfigMutationClassMissing,
    /// A derived-relationship-banner component declares no derived-relationship classes.
    DerivedRelationshipClassMissing,
    /// A derived-relationship-banner component declares no relationship proving states.
    RelationshipProvingStateMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (hidden pack identity / support, heuristic
    /// masquerading as exact, implied no-op write while mutating, hidden execution boundary,
    /// omitted proving-source / rollback path, or invented alternate state label).
    ComponentInvariantViolated,
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

impl M5FrameworkComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::PackSupportClassMissing => "pack_support_class_missing",
            Self::PackIdentityStateMissing => "pack_identity_state_missing",
            Self::RouteEvidenceClassMissing => "route_evidence_class_missing",
            Self::RouteAuthorshipMissing => "route_authorship_missing",
            Self::TopologyNodeKindMissing => "topology_node_kind_missing",
            Self::TopologyEvidenceClassMissing => "topology_evidence_class_missing",
            Self::ConventionConfidenceClassMissing => "convention_confidence_class_missing",
            Self::DiagnosticSeverityMissing => "diagnostic_severity_missing",
            Self::GeneratorImpactClassMissing => "generator_impact_class_missing",
            Self::GeneratorApplyPostureMissing => "generator_apply_posture_missing",
            Self::ExecutionBoundaryClassMissing => "execution_boundary_class_missing",
            Self::RunConfigMutationClassMissing => "run_config_mutation_class_missing",
            Self::DerivedRelationshipClassMissing => "derived_relationship_class_missing",
            Self::RelationshipProvingStateMissing => "relationship_proving_state_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 framework matrix export.
///
/// This is the first real consumer of the framework-component lane: a framework-pack, route
/// explorer, topology explorer, convention-diagnostics, generator-review, or support-export
/// surface calls it to ingest the canonical matrix rather than cloning status text.
///
/// # Errors
///
/// Returns [`M5FrameworkComponentMatrixArtifactError`] when the checked-in support export fails
/// to parse or fails validation.
pub fn current_stable_m5_framework_component_matrix_export(
) -> Result<M5FrameworkComponentMatrixPacket, M5FrameworkComponentMatrixArtifactError> {
    let packet: M5FrameworkComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-framework-component-proof/support_export.json"
    )))
    .map_err(M5FrameworkComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5FrameworkComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5FrameworkComponentMatrixPacket,
    violations: &mut Vec<M5FrameworkComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_DOC_REF,
        M5_FRAMEWORK_PACK_HEADER_SCHEMA_REF,
        M5_ROUTE_ENDPOINT_ROW_SCHEMA_REF,
        M5_COMPONENT_SERVICE_TREE_NODE_SCHEMA_REF,
        M5_CONVENTION_DIAGNOSTIC_ROW_SCHEMA_REF,
        M5_GENERATOR_PREVIEW_SHEET_SCHEMA_REF,
        M5_RUN_CONFIG_SCAFFOLD_CARD_SCHEMA_REF,
        M5_DERIVED_RELATIONSHIP_BANNER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5FrameworkComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5FrameworkComponentMatrixPacket,
    violations: &mut Vec<M5FrameworkComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5FrameworkComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5FrameworkComponentMatrixPacket,
    violations: &mut Vec<M5FrameworkComponentMatrixViolation>,
) {
    let present: BTreeSet<M5FrameworkComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5FrameworkComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5FrameworkComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5FrameworkComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5FrameworkComponentMatrixViolation::MandatoryLabelMissing);
        }
        if row.dispositions.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::DispositionsMissing);
        }
        if family.is_framework_pack_header() && row.pack_support_classes.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::PackSupportClassMissing);
        }
        if family.is_framework_pack_header() && row.pack_identity_states.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::PackIdentityStateMissing);
        }
        if family.is_route_endpoint_row() && row.route_evidence_classes.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::RouteEvidenceClassMissing);
        }
        if family.is_route_endpoint_row() && row.route_authorship_states.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::RouteAuthorshipMissing);
        }
        if family.is_component_service_tree_node() && row.topology_node_kinds.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::TopologyNodeKindMissing);
        }
        if family.is_component_service_tree_node() && row.topology_evidence_classes.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::TopologyEvidenceClassMissing);
        }
        if family.is_convention_diagnostic_row() && row.convention_confidence_classes.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::ConventionConfidenceClassMissing);
        }
        if family.is_convention_diagnostic_row() && row.diagnostic_severities.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::DiagnosticSeverityMissing);
        }
        if family.is_generator_preview_sheet() && row.generator_impact_classes.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::GeneratorImpactClassMissing);
        }
        if family.is_generator_preview_sheet() && row.generator_apply_postures.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::GeneratorApplyPostureMissing);
        }
        if family.is_run_config_scaffold_card() && row.execution_boundary_classes.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::ExecutionBoundaryClassMissing);
        }
        if family.is_run_config_scaffold_card() && row.run_config_mutation_classes.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::RunConfigMutationClassMissing);
        }
        if family.is_derived_relationship_banner() && row.derived_relationship_classes.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::DerivedRelationshipClassMissing);
        }
        if family.is_derived_relationship_banner() && row.relationship_proving_states.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::RelationshipProvingStateMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5FrameworkComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5FrameworkComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5FrameworkComponentMatrixPacket,
    violations: &mut Vec<M5FrameworkComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.pack_header_shows_identity_version_and_support,
        review.route_row_shows_exact_versus_heuristic_and_authorship,
        review.tree_node_shows_kind_and_evidence_class,
        review.convention_row_shows_confidence_and_proving_source,
        review.generator_sheet_shows_impact_and_rollback,
        review.run_config_card_shows_execution_boundary_and_mutation,
        review.derived_banner_shows_derived_state_and_proving_source,
        review.no_surface_invents_alternate_state_label,
        review.no_heuristic_masquerades_as_exact,
        review.no_generator_implies_no_op_while_mutating,
        review.execution_boundary_always_visible,
        review.proving_source_always_linked,
        review.rollback_or_regenerate_always_explicit,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5FrameworkComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5FrameworkComponentMatrixPacket,
    violations: &mut Vec<M5FrameworkComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.framework_pack_surfaces_consume_identity_and_support_vocabulary,
        projection.route_and_topology_surfaces_consume_exact_versus_heuristic_vocabulary,
        projection.convention_surfaces_consume_confidence_and_proving_vocabulary,
        projection.generator_surfaces_consume_impact_and_rollback_vocabulary,
        projection.run_config_surfaces_consume_execution_boundary_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5FrameworkComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5FrameworkComponentMatrixPacket,
    violations: &mut Vec<M5FrameworkComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5FrameworkComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5FrameworkComponentMatrixPacket,
    violations: &mut Vec<M5FrameworkComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.framework_component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5FrameworkComponentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Canonical seed builders
//
// These builders are the single producer of the checked-in support export and the narrowed
// fixtures. The headless emitter example and the inline tests both call them so the in-code
// matrix, the artifact, and the fixtures never drift.
// ---------------------------------------------------------------------------

/// Stable packet id for the canonical framework-component matrix.
pub const M5_FRAMEWORK_COMPONENT_MATRIX_PACKET_ID: &str = "m5-framework-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5FrameworkRequiredLabel> {
    M5FrameworkRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5FrameworkRequiredLabel]) -> Vec<M5FrameworkRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every family-specific
/// vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5FrameworkComponentFamily,
    qualification: M5FrameworkQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5FrameworkComponentRow {
    M5FrameworkComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5FrameworkSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5FrameworkDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        dispositions: vec![],
        pack_support_classes: vec![],
        pack_identity_states: vec![],
        route_evidence_classes: vec![],
        route_authorship_states: vec![],
        topology_node_kinds: vec![],
        topology_evidence_classes: vec![],
        convention_confidence_classes: vec![],
        diagnostic_severities: vec![],
        generator_impact_classes: vec![],
        generator_apply_postures: vec![],
        execution_boundary_classes: vec![],
        run_config_mutation_classes: vec![],
        derived_relationship_classes: vec![],
        relationship_proving_states: vec![],
        accessibility_routes: M5FrameworkAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5FrameworkConsumerSurface::FrameworkPackUi,
            M5FrameworkConsumerSurface::SupportExport,
        ],
        downgrade_triggers: vec![M5FrameworkDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        hides_pack_identity_version_or_support_class: false,
        lets_heuristic_masquerade_as_exact: false,
        implies_no_op_write_while_mutating_config_or_dependencies: false,
        hides_local_container_ssh_or_managed_boundary: false,
        omits_proving_source_or_rollback_path: false,
        invents_alternate_state_label: false,
    }
}

fn component_rows() -> Vec<M5FrameworkComponentRow> {
    use M5FrameworkCertaintyDisposition as DI;
    use M5FrameworkComponentFamily as F;
    use M5FrameworkConsumerSurface as C;
    use M5FrameworkDowngradeTrigger as D;
    use M5FrameworkQualificationClass as Q;
    use M5FrameworkRequiredLabel as L;
    use M5FrameworkSurfaceFamily as S;

    let mut rows = Vec::new();

    // 1. Framework pack header.
    let mut row = base_row(
        F::FrameworkPackHeader,
        Q::Stable,
        "Framework pack header owner",
        "One framework-pack-header model naming which pack and version is active (identified and versioned, version pinned, version drifted, multiple detected, unversioned, or unknown pack) and how it is supported (officially supported, community supported, experimental, bridge only, deprecated, or unsupported), so a header never leaves its pack identity, version, or support class implicit and never presents bridge or heuristic behavior as exact first-party support",
        "evidence:m5-framework-pack-header-parity:001",
        &[M5_FRAMEWORK_COMPONENT_SCHEMA_REF, M5_FRAMEWORK_PACK_HEADER_SCHEMA_REF],
    );
    row.dispositions = vec![DI::CoreNative, DI::FrameworkPack, DI::Bridge, DI::Partial];
    row.pack_support_classes = M5FrameworkPackSupportClass::ALL.to_vec();
    row.pack_identity_states = M5FrameworkPackIdentityState::ALL.to_vec();
    row.required_labels = labels_with(&[L::PackSourceAndCertainty]);
    row.surface_families = vec![S::FrameworkPackSurface, S::TopologyExplorer, S::CliSurface];
    row.consumer_surfaces = vec![C::FrameworkPackUi, C::TopologyUi, C::SupportExport];
    row.downgrade_triggers = vec![
        D::PackIdentityUnstated,
        D::SupportClassUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Route / endpoint row.
    let mut row = base_row(
        F::RouteEndpointRow,
        Q::Stable,
        "Route endpoint row owner",
        "One route-endpoint-row model naming how a route or endpoint is known (exact from source, a heuristic convention, runtime confirmed, derived by convention, partial evidence, or unresolved) and whether it is authored or generated (authored, generated, generated then edited, framework provided, runtime only, or unknown origin), so a heuristic route never masquerades as an exact one and the authored-versus-generated boundary is always explicit",
        "evidence:m5-route-endpoint-row-parity:001",
        &[M5_FRAMEWORK_COMPONENT_SCHEMA_REF, M5_ROUTE_ENDPOINT_ROW_SCHEMA_REF],
    );
    row.dispositions = vec![
        DI::Verified,
        DI::HeuristicConvention,
        DI::RuntimeConfirmed,
        DI::DerivedByConvention,
        DI::Partial,
    ];
    row.route_evidence_classes = M5RouteEvidenceClass::ALL.to_vec();
    row.route_authorship_states = M5RouteAuthorship::ALL.to_vec();
    row.required_labels = labels_with(&[
        L::PackSourceAndCertainty,
        L::ProvingSourceAndRecoveryBoundary,
    ]);
    row.surface_families = vec![S::RouteExplorer, S::TopologyExplorer, S::CliSurface];
    row.consumer_surfaces = vec![C::RouteExplorerUi, C::EditorGutterUi, C::SupportExport];
    row.downgrade_triggers = vec![
        D::ExactVersusHeuristicUnstated,
        D::AuthorshipUnstated,
        D::ProvingSourceOmitted,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Component / service tree node.
    let mut row = base_row(
        F::ComponentServiceTreeNode,
        Q::Stable,
        "Component service tree node owner",
        "One component-service-tree-node model naming what the node represents (a component, a service, a module, a dependency edge, an external boundary, or an unknown node) and how the relationship is known (exact from source, heuristic inferred, runtime confirmed, derived by convention, partial evidence, or unresolved), so an inferred component tree never masquerades as an exact one and every node names its proving source",
        "evidence:m5-component-service-tree-node-parity:001",
        &[M5_FRAMEWORK_COMPONENT_SCHEMA_REF, M5_COMPONENT_SERVICE_TREE_NODE_SCHEMA_REF],
    );
    row.dispositions = vec![
        DI::Verified,
        DI::HeuristicConvention,
        DI::RuntimeConfirmed,
        DI::DerivedByConvention,
        DI::Partial,
    ];
    row.topology_node_kinds = M5TopologyNodeKind::ALL.to_vec();
    row.topology_evidence_classes = M5TopologyEvidenceClass::ALL.to_vec();
    row.required_labels = labels_with(&[
        L::PackSourceAndCertainty,
        L::ProvingSourceAndRecoveryBoundary,
    ]);
    row.surface_families = vec![S::TopologyExplorer, S::RouteExplorer, S::CliSurface];
    row.consumer_surfaces = vec![C::TopologyUi, C::EditorGutterUi, C::SupportExport];
    row.downgrade_triggers = vec![
        D::ExactVersusHeuristicUnstated,
        D::ProvingSourceOmitted,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Convention-diagnostic row.
    let mut row = base_row(
        F::ConventionDiagnosticRow,
        Q::Stable,
        "Convention diagnostic row owner",
        "One convention-diagnostic-row model naming how confident the diagnostic is (verified, high confidence, a heuristic convention, derived by convention, low confidence, or unknown) and its severity (error, warning, hint, info, suppressed, or stale), so a heuristic convention guess never reads as a verified fact and every diagnostic names the files that prove it",
        "evidence:m5-convention-diagnostic-row-parity:001",
        &[M5_FRAMEWORK_COMPONENT_SCHEMA_REF, M5_CONVENTION_DIAGNOSTIC_ROW_SCHEMA_REF],
    );
    row.dispositions = vec![
        DI::Verified,
        DI::HeuristicConvention,
        DI::DerivedByConvention,
        DI::Partial,
    ];
    row.convention_confidence_classes = M5ConventionConfidenceClass::ALL.to_vec();
    row.diagnostic_severities = M5ConventionDiagnosticSeverity::ALL.to_vec();
    row.required_labels = labels_with(&[L::ProvingSourceAndRecoveryBoundary]);
    row.surface_families = vec![S::ConventionDiagnostics, S::RouteExplorer, S::CliSurface];
    row.consumer_surfaces = vec![C::DiagnosticCenterUi, C::EditorGutterUi, C::SupportExport];
    row.downgrade_triggers = vec![
        D::ConventionConfidenceOverstated,
        D::ProvingSourceOmitted,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Generator preview sheet.
    let mut row = base_row(
        F::GeneratorPreviewSheet,
        Q::Stable,
        "Generator preview sheet owner",
        "One generator-preview-sheet model naming what a generator or codemod will change (file write, dependency change, config change, script or task change, no change, or unknown impact) and what it permits (preview ready, review required, apply ready, rollback available, regenerate available, or blocked), so a generator never implies a no-op write when it changes config or dependencies and rollback or regenerate stays explicit",
        "evidence:m5-generator-preview-sheet-parity:001",
        &[M5_FRAMEWORK_COMPONENT_SCHEMA_REF, M5_GENERATOR_PREVIEW_SHEET_SCHEMA_REF],
    );
    row.dispositions = vec![DI::CoreNative, DI::FrameworkPack, DI::Bridge, DI::Partial];
    row.generator_impact_classes = M5GeneratorImpactClass::ALL.to_vec();
    row.generator_apply_postures = M5GeneratorApplyPosture::ALL.to_vec();
    row.required_labels = labels_with(&[
        L::ExecutionBoundaryAndImpact,
        L::ProvingSourceAndRecoveryBoundary,
    ]);
    row.surface_families = vec![S::GeneratorReview, S::FrameworkPackSurface, S::CliSurface];
    row.consumer_surfaces = vec![C::GeneratorReviewUi, C::RunConfigUi, C::SupportExport];
    row.downgrade_triggers = vec![
        D::ImpactUndisclosed,
        D::RollbackPathOmitted,
        D::ExecutionBoundaryUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Run-config scaffold card.
    let mut row = base_row(
        F::RunConfigScaffoldCard,
        Q::Stable,
        "Run-config scaffold card owner",
        "One run-config-scaffold-card model naming where a framework convenience action will actually run (a local process, a container, an SSH remote, a managed workspace, a cloud remote, or an unknown boundary) and what it writes (creates a config file, edits a config file, adds a dependency, a no-write preview, rollback available, or unknown mutation), so the local, container, SSH, or managed boundary never hides behind framework convenience language and a convenience action never implies a no-op write",
        "evidence:m5-run-config-scaffold-card-parity:001",
        &[M5_FRAMEWORK_COMPONENT_SCHEMA_REF, M5_RUN_CONFIG_SCAFFOLD_CARD_SCHEMA_REF],
    );
    row.dispositions = vec![DI::CoreNative, DI::FrameworkPack, DI::Bridge, DI::Partial];
    row.execution_boundary_classes = M5ExecutionBoundaryClass::ALL.to_vec();
    row.run_config_mutation_classes = M5RunConfigMutationClass::ALL.to_vec();
    row.required_labels = labels_with(&[
        L::ExecutionBoundaryAndImpact,
        L::ProvingSourceAndRecoveryBoundary,
    ]);
    row.surface_families = vec![S::GeneratorReview, S::FrameworkPackSurface, S::CliSurface];
    row.consumer_surfaces = vec![
        C::RunConfigUi,
        C::GeneratorReviewUi,
        C::CliSurface,
        C::SupportExport,
    ];
    row.downgrade_triggers = vec![
        D::ExecutionBoundaryUnstated,
        D::ImpactUndisclosed,
        D::RollbackPathOmitted,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Derived-relationship banner.
    let mut row = base_row(
        F::DerivedRelationshipBanner,
        Q::Stable,
        "Derived-relationship banner owner",
        "One derived-relationship-banner model naming how a relationship is known (exact from source, inferred from runtime, a heuristic link, derived by convention, a partial link, or an unresolved link) and how firmly it links to its proving source (proving source linked, source linked partial, runtime evidence only, convention only, no proving source, or unknown proving), so a derived or inferred link never masquerades as an exact one and every derived state names its proving evidence",
        "evidence:m5-derived-relationship-banner-parity:001",
        &[M5_FRAMEWORK_COMPONENT_SCHEMA_REF, M5_DERIVED_RELATIONSHIP_BANNER_SCHEMA_REF],
    );
    row.dispositions = vec![
        DI::Verified,
        DI::RuntimeConfirmed,
        DI::HeuristicConvention,
        DI::DerivedByConvention,
        DI::Partial,
    ];
    row.derived_relationship_classes = M5DerivedRelationshipClass::ALL.to_vec();
    row.relationship_proving_states = M5RelationshipProvingState::ALL.to_vec();
    row.required_labels = labels_with(&[L::ProvingSourceAndRecoveryBoundary]);
    row.surface_families = vec![S::TopologyExplorer, S::RouteExplorer, S::CliSurface];
    row.consumer_surfaces = vec![C::TopologyUi, C::RouteExplorerUi, C::SupportExport];
    row.downgrade_triggers = vec![
        D::DerivedStateUnlabeled,
        D::ProvingSourceOmitted,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5FrameworkComponentGovernanceReview {
    M5FrameworkComponentGovernanceReview {
        pack_header_shows_identity_version_and_support: true,
        route_row_shows_exact_versus_heuristic_and_authorship: true,
        tree_node_shows_kind_and_evidence_class: true,
        convention_row_shows_confidence_and_proving_source: true,
        generator_sheet_shows_impact_and_rollback: true,
        run_config_card_shows_execution_boundary_and_mutation: true,
        derived_banner_shows_derived_state_and_proving_source: true,
        no_surface_invents_alternate_state_label: true,
        no_heuristic_masquerades_as_exact: true,
        no_generator_implies_no_op_while_mutating: true,
        execution_boundary_always_visible: true,
        proving_source_always_linked: true,
        rollback_or_regenerate_always_explicit: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5FrameworkComponentConsumerProjection {
    M5FrameworkComponentConsumerProjection {
        framework_pack_surfaces_consume_identity_and_support_vocabulary: true,
        route_and_topology_surfaces_consume_exact_versus_heuristic_vocabulary: true,
        convention_surfaces_consume_confidence_and_proving_vocabulary: true,
        generator_surfaces_consume_impact_and_rollback_vocabulary: true,
        run_config_surfaces_consume_execution_boundary_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5FrameworkComponentProofFreshness {
    M5FrameworkComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5FrameworkComponentReleasePosture {
    M5FrameworkComponentReleasePosture {
        proof_packet_ref: M5_FRAMEWORK_COMPONENT_ARTIFACT_REF.to_owned(),
        framework_component_audit_ref: M5_FRAMEWORK_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_DOC_REF,
        M5_FRAMEWORK_PACK_HEADER_SCHEMA_REF,
        M5_ROUTE_ENDPOINT_ROW_SCHEMA_REF,
        M5_COMPONENT_SERVICE_TREE_NODE_SCHEMA_REF,
        M5_CONVENTION_DIAGNOSTIC_ROW_SCHEMA_REF,
        M5_GENERATOR_PREVIEW_SHEET_SCHEMA_REF,
        M5_RUN_CONFIG_SCAFFOLD_CARD_SCHEMA_REF,
        M5_DERIVED_RELATIONSHIP_BANNER_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 framework-component matrix packet.
pub fn seeded_m5_framework_component_matrix() -> M5FrameworkComponentMatrixPacket {
    M5FrameworkComponentMatrixPacket::new(M5FrameworkComponentMatrixPacketInput {
        packet_id: M5_FRAMEWORK_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 framework-pack-header, route-endpoint-row, component-service-tree-node, convention-diagnostic-row, generator-preview-sheet, run-config-scaffold-card, and derived-relationship-banner component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5FrameworkComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the route / endpoint row is held at Beta because route resolution is
/// convention- and runtime-dependent and exact-versus-heuristic parity for a slice of the route
/// evidence does not yet round-trip across every route-explorer surface; every component stays
/// visible.
pub fn seeded_m5_framework_component_matrix_route_endpoint_row_beta_narrowed(
) -> M5FrameworkComponentMatrixPacket {
    let mut packet = seeded_m5_framework_component_matrix();
    packet.packet_id = "m5-framework-components:route-endpoint-row-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5FrameworkComponentFamily::RouteEndpointRow)
        .expect("route-endpoint-row row present");
    row.qualification = M5FrameworkQualificationClass::Beta;
    packet
}

/// Narrowed variant: the generator preview sheet is narrowed to Preview pending apply / rollback
/// and execution-boundary parity proof across every generator-review surface; every component
/// stays visible.
pub fn seeded_m5_framework_component_matrix_generator_preview_sheet_preview_narrowed(
) -> M5FrameworkComponentMatrixPacket {
    let mut packet = seeded_m5_framework_component_matrix();
    packet.packet_id = "m5-framework-components:generator-preview-sheet-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5FrameworkComponentFamily::GeneratorPreviewSheet)
        .expect("generator-preview-sheet row present");
    row.qualification = M5FrameworkQualificationClass::Preview;
    packet
}
