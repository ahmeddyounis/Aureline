//! Framework support strip and framework-object certainty record model.
//!
//! See the crate-level docs for what this module owns and why. The shapes,
//! vocabularies, and validators here mirror the boundary schemas at
//! `/schemas/framework/framework_support_strip.schema.json` and
//! `/schemas/framework/framework_object_certainty.schema.json` so route
//! explorers, component / service trees, convention-diagnostic lanes,
//! generator-preview review sheets, notebook framework-context inspectors,
//! AI-assist context lanes, and support exports never invent divergent
//! shapes.

use serde::{Deserialize, Serialize};

mod object_certainty;
mod support_strip;

pub use object_certainty::{
    AuthoredOriginClass, CertaintyLabelClass, ConventionCertaintyClass,
    ConventionDiagnosticBlock, ConventionDiagnosticClass, ConventionFixActionClass,
    DependencyImpactClass, EvidenceAnchor, EvidenceAnchorKindClass, FileEffectClass,
    FileOwnershipClass, FrameworkObjectCertainty, FrameworkObjectKind,
    FrameworkObjectRowBlock, FrameworkObjectRowKind, GeneratorFileEffectRow, GeneratorKindClass,
    GeneratorPreviewBlock, RollbackClass, RowActionClass,
    FRAMEWORK_OBJECT_CERTAINTY_RECORD_KIND, FRAMEWORK_OBJECT_CERTAINTY_SCHEMA_VERSION,
};
pub use support_strip::{
    CompatibilityBlock, FrameworkFamilyClass, FrameworkIdentityBlock,
    FrameworkSupportActionClass, FrameworkSupportStrip, HealthBlock, HealthClass, LocalityClass,
    PackOrBridgeSourceBlock, PackSourceClass, ScopeBlock, SupportClass, VersionCompatibilityClass,
    FRAMEWORK_FRESHNESS_AUTHORITATIVE_LIVE, FRAMEWORK_FRESHNESS_DEGRADED_CACHED,
    FRAMEWORK_FRESHNESS_STALE, FRAMEWORK_FRESHNESS_UNVERIFIED, FRAMEWORK_FRESHNESS_WARM_CACHED,
    FRAMEWORK_SUPPORT_STRIP_RECORD_KIND, FRAMEWORK_SUPPORT_STRIP_SCHEMA_VERSION,
};

/// Closed surface vocabulary shared by both records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    RouteExplorerSurface,
    ComponentTreeSurface,
    ServiceGraphSurface,
    ConventionDiagnosticsSurface,
    GeneratorPreviewSurface,
    NotebookFrameworkContextSurface,
    AiAssistContextSurface,
    SupportExportSurface,
}

impl SurfaceClass {
    /// Stable closed-vocabulary token recorded in records, schemas, and
    /// exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RouteExplorerSurface => "route_explorer_surface",
            Self::ComponentTreeSurface => "component_tree_surface",
            Self::ServiceGraphSurface => "service_graph_surface",
            Self::ConventionDiagnosticsSurface => "convention_diagnostics_surface",
            Self::GeneratorPreviewSurface => "generator_preview_surface",
            Self::NotebookFrameworkContextSurface => "notebook_framework_context_surface",
            Self::AiAssistContextSurface => "ai_assist_context_surface",
            Self::SupportExportSurface => "support_export_surface",
        }
    }
}

/// Closed freshness vocabulary re-exported from the language provider-graph
/// contract. Kept as string constants because the support strip carries the
/// raw token alongside the typed `HealthBlock`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    AuthoritativeLive,
    WarmCached,
    DegradedCached,
    Stale,
    Unverified,
}

impl FreshnessClass {
    /// Stable closed-vocabulary token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
        }
    }

    /// Whether this freshness class admits a "healthy live" runtime health.
    pub const fn admits_healthy_live(self) -> bool {
        matches!(self, Self::AuthoritativeLive | Self::WarmCached | Self::DegradedCached)
    }
}

/// Typed validation finding shared by both records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    /// Stable check id.
    pub check_id: String,
    /// Subject row id.
    pub subject_ref: String,
    /// Export-safe finding message.
    pub message: String,
}

impl Finding {
    pub(crate) fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests;
