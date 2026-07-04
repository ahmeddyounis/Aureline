//! Frozen reusable manifest / build-confidence component matrix: manifest-editor
//! headers, schema/validator rows, target-context chip groups, resource-link and
//! resource-explorer rows, adapter-source badges, target-graph rows, capability
//! matrices, raw-event drawers, and fallback-confidence drawers.
//!
//! Where [`crate::infrastructure_surface_qualification`] freezes the
//! *qualification* of each claimed infrastructure surface,
//! [`crate::cluster_context_and_live_resource`] materializes the *per-surface*
//! target-context and truth-mode state, and
//! [`crate::source_intelligence_and_resource_relationships`] materializes the
//! *per-object* relationship truth, this module freezes the reusable
//! **manifest / build-confidence component** contract: the headers, rows, badges,
//! chips, and drawers users actually rely on to understand target scope and
//! confidence before acting, so later M5 rows reference one canonical component
//! family instead of restating infra / build confidence truth in feature-local
//! prose.
//!
//! One [`ManifestBuildComponentMatrix`] packet defines every reusable primitive,
//! its state vocabulary, its required labels, and its export / assistive parity
//! expectations, binding each onto the same authored / rendered / planned / live /
//! provider-overlay truth classes ([`crate::TruthMode`]), preview/apply/review
//! vocabulary, and degraded-state language already used across Aureline — never
//! bespoke per-adapter or per-connector chrome.
//!
//! The honesty rules the spec freezes, carried by every [`ComponentRow`]:
//!
//! - **Authored, rendered, planned, live, cached, and provider-overlay truth
//!   never blurs.** Every component binds to one [`crate::TruthMode`] and never
//!   presents one truth class as another.
//! - **Target context stays visible on every read- or mutate-capable surface.** A
//!   row always carries a resolvable target-context ref; a manifest header, a
//!   resource explorer row, and a target-graph row never hide the target they act
//!   on.
//! - **Schema freshness and adapter source kind are explicit.** A schema/validator
//!   row names whether its schema is fresh, stale, unversioned, or unavailable; an
//!   adapter-source badge names whether truth came from a native build server /
//!   event stream, a heuristic parse, an imported snapshot, or a provider overlay.
//! - **Lower-confidence discovery never silently overwrites higher-confidence
//!   truth.** Resource-link, capability, and fallback-confidence components declare
//!   they never overwrite a higher-confidence result silently.
//! - **Drift, connector loss, and policy blocks narrow actions before execution.**
//!   They degrade to a typed [`DegradedState`] that names a real recovery route
//!   rather than failing after a run starts.
//!
//! Raw manifest bodies, file contents, credentials, connector tokens, and raw
//! provider / build-event payloads never cross this boundary; the packet carries
//! only typed class tokens, opaque target / span / evidence refs, booleans, and
//! redacted labels, so support and diagnostics exports can reconstruct exactly what
//! a component would have shown without leaking source or live payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-manifest-build-component-matrix.schema.json`](../../../../schemas/ui/m5-manifest-build-component-matrix.schema.json).
//! The contract doc is
//! [`docs/infra/m5_manifest_build_component_matrix.md`](../../../../docs/infra/m5_manifest_build_component_matrix.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-manifest-build-components/`](../../../../fixtures/ui/m5-manifest-build-components/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::TruthMode;

/// Stable record-kind tag carried by [`ManifestBuildComponentMatrix`].
pub const MANIFEST_BUILD_COMPONENT_MATRIX_RECORD_KIND: &str = "m5_manifest_build_component_matrix";

/// Schema version for the manifest / build-confidence component matrix packet.
pub const MANIFEST_BUILD_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const MANIFEST_BUILD_COMPONENT_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-manifest-build-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const MANIFEST_BUILD_COMPONENT_MATRIX_DOC_REF: &str =
    "docs/infra/m5_manifest_build_component_matrix.md";

/// Repo-relative path of the protected fixture directory.
pub const MANIFEST_BUILD_COMPONENT_MATRIX_FIXTURE_DIR: &str =
    "fixtures/ui/m5-manifest-build-components";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const MANIFEST_BUILD_COMPONENT_MATRIX_ARTIFACT_REF: &str =
    "artifacts/infra/m5-manifest-build-component-matrix/support_export.json";

/// Repo-relative path of the checked Markdown matrix summary.
pub const MANIFEST_BUILD_COMPONENT_MATRIX_SUMMARY_REF: &str =
    "artifacts/design/m5-manifest-build-component-matrix.md";

/// Closed reusable manifest / build-confidence component family. Each family is
/// one governed primitive later M5 rows reference by name; the matrix must define
/// every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManifestBuildComponentFamily {
    /// A manifest-editor header that frames target context and edit posture.
    ManifestEditorHeader,
    /// A schema / validator row disclosing schema freshness and validation state.
    SchemaValidatorRow,
    /// A target-context chip group pinned to a read- or mutate-capable surface.
    TargetContextChipGroup,
    /// A resource-link row joining two truth classes for the same resource.
    ResourceLinkRow,
    /// A resource-explorer row disclosing freshness and confidence.
    ResourceExplorerRow,
    /// An adapter-source badge naming where build truth came from.
    AdapterSourceBadge,
    /// A target-graph row for a build / test / run / dependency node.
    TargetGraphRow,
    /// A capability matrix cell disclosing supported / partial / unsupported state.
    CapabilityMatrix,
    /// A raw-event drawer disclosing redacted native / heuristic event provenance.
    RawEventDrawer,
    /// A fallback-confidence drawer disclosing structured-versus-heuristic posture.
    FallbackConfidenceDrawer,
}

impl M5ManifestBuildComponentFamily {
    /// Every reusable component family the matrix must define, in declaration
    /// order.
    pub const ALL: [Self; 10] = [
        Self::ManifestEditorHeader,
        Self::SchemaValidatorRow,
        Self::TargetContextChipGroup,
        Self::ResourceLinkRow,
        Self::ResourceExplorerRow,
        Self::AdapterSourceBadge,
        Self::TargetGraphRow,
        Self::CapabilityMatrix,
        Self::RawEventDrawer,
        Self::FallbackConfidenceDrawer,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManifestEditorHeader => "manifest_editor_header",
            Self::SchemaValidatorRow => "schema_validator_row",
            Self::TargetContextChipGroup => "target_context_chip_group",
            Self::ResourceLinkRow => "resource_link_row",
            Self::ResourceExplorerRow => "resource_explorer_row",
            Self::AdapterSourceBadge => "adapter_source_badge",
            Self::TargetGraphRow => "target_graph_row",
            Self::CapabilityMatrix => "capability_matrix",
            Self::RawEventDrawer => "raw_event_drawer",
            Self::FallbackConfidenceDrawer => "fallback_confidence_drawer",
        }
    }
}

/// Stable token for the reused [`crate::TruthMode`] truth class, so CSV / chip /
/// summary renders never depend on serde formatting of a foreign enum.
pub const fn truth_mode_token(mode: TruthMode) -> &'static str {
    match mode {
        TruthMode::Desired => "authored_desired",
        TruthMode::Rendered => "rendered",
        TruthMode::Plan => "planned",
        TruthMode::Live => "live",
        TruthMode::ProviderOverlay => "provider_overlay",
    }
}

/// Closed schema-freshness vocabulary. Names how current a schema is so a stale or
/// unversioned schema never masquerades as fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SchemaFreshness {
    /// Schema resolved and current for the target.
    Fresh,
    /// Schema resolved but known to be stale relative to the target.
    Stale,
    /// Schema resolved but carries no resolvable version.
    Unversioned,
    /// Schema could not be resolved at all.
    Unavailable,
}

impl M5SchemaFreshness {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::Unversioned => "unversioned",
            Self::Unavailable => "unavailable",
        }
    }

    /// True when the schema is current and safe to treat as authoritative.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Fresh)
    }
}

/// Closed discovery-confidence vocabulary. Names how confident a discovered link,
/// resource, or edge is so lower-confidence truth is never presented as certain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiscoveryConfidence {
    /// High confidence: native / authoritative source.
    High,
    /// Medium confidence: derived but corroborated.
    Medium,
    /// Low confidence: heuristic or partial.
    Low,
    /// Confidence not yet established.
    Unknown,
}

impl M5DiscoveryConfidence {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::Unknown => "unknown",
        }
    }

    /// Confidence rank, higher is more confident; used to reason about overwrites.
    pub const fn rank(self) -> u8 {
        match self {
            Self::High => 3,
            Self::Medium => 2,
            Self::Low => 1,
            Self::Unknown => 0,
        }
    }
}

/// Closed adapter-source vocabulary. Names where build / target truth came from so
/// a heuristic or imported result never claims native authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdapterSourceKind {
    /// A native build-server protocol adapter (e.g. BSP).
    NativeBuildServer,
    /// A native build-event protocol stream (e.g. BEP).
    NativeBuildEvent,
    /// A heuristic parse of build output or config.
    HeuristicParse,
    /// An imported snapshot from a prior run.
    ImportedSnapshot,
    /// Provider-owned overlay / console-only context.
    ProviderOverlay,
    /// Source kind not yet established.
    Unknown,
}

impl M5AdapterSourceKind {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeBuildServer => "native_build_server",
            Self::NativeBuildEvent => "native_build_event",
            Self::HeuristicParse => "heuristic_parse",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::ProviderOverlay => "provider_overlay",
            Self::Unknown => "unknown",
        }
    }

    /// True when this source is a native, authoritative build channel.
    pub const fn is_native(self) -> bool {
        matches!(self, Self::NativeBuildServer | Self::NativeBuildEvent)
    }

    /// Whether a confidence label is consistent with this source kind: a heuristic,
    /// imported, or unknown source can never claim high confidence.
    pub const fn confidence_consistent(self, confidence: M5DiscoveryConfidence) -> bool {
        match self {
            Self::NativeBuildServer | Self::NativeBuildEvent => true,
            Self::HeuristicParse
            | Self::ImportedSnapshot
            | Self::ProviderOverlay
            | Self::Unknown => !matches!(confidence, M5DiscoveryConfidence::High),
        }
    }
}

/// Closed manifest-editor edit-posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManifestEditPosture {
    /// Read-only inspection; no write path is offered.
    ReadOnly,
    /// Editable through the shared preview / apply / review path.
    PreviewApplyReview,
    /// A blocked protected path; edits are refused.
    BlockedProtected,
}

impl M5ManifestEditPosture {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::PreviewApplyReview => "preview_apply_review",
            Self::BlockedProtected => "blocked_protected",
        }
    }

    /// True when this posture writes the manifest.
    pub const fn writes_manifest(self) -> bool {
        matches!(self, Self::PreviewApplyReview)
    }
}

/// A manifest-editor header descriptor. Present only on a
/// [`M5ManifestBuildComponentFamily::ManifestEditorHeader`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestEditorHeaderDescriptor {
    /// The truth class the manifest is shown in.
    pub truth_mode: TruthMode,
    /// The freshness of the schema backing the manifest.
    pub schema_freshness: M5SchemaFreshness,
    /// The edit posture the header offers.
    pub edit_posture: M5ManifestEditPosture,
    /// The header keeps target context visible; must always hold.
    pub target_context_visible: bool,
    /// Opaque ref to the manifest object; never raw manifest bytes.
    pub manifest_ref: String,
}

impl ManifestEditorHeaderDescriptor {
    /// Whether the manifest header descriptor is internally complete and honest: it
    /// keeps target context visible, names its manifest, and never offers a write
    /// path over a blocked protected path or an unresolved schema.
    pub fn is_honest(&self) -> bool {
        if !self.target_context_visible || self.manifest_ref.trim().is_empty() {
            return false;
        }
        if self.edit_posture.writes_manifest()
            && self.schema_freshness == M5SchemaFreshness::Unavailable
        {
            // A writable manifest cannot claim an editable posture with no schema.
            return false;
        }
        true
    }
}

/// Closed schema-validation-state vocabulary. Names the validator verdict so an
/// error state can never quietly permit an apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SchemaValidationState {
    /// Valid against the resolved schema.
    Valid,
    /// Valid but carrying advisory warnings.
    Warnings,
    /// Invalid against the resolved schema.
    Errors,
    /// The schema itself could not be resolved.
    SchemaUnavailable,
    /// The schema resolved but carries no version to validate against.
    Unversioned,
}

impl M5SchemaValidationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::Warnings => "warnings",
            Self::Errors => "errors",
            Self::SchemaUnavailable => "schema_unavailable",
            Self::Unversioned => "unversioned",
        }
    }

    /// True when an apply must be blocked in this validation state.
    pub const fn must_block_apply(self) -> bool {
        matches!(self, Self::Errors | Self::SchemaUnavailable)
    }
}

/// A schema / validator row descriptor. Present only on a
/// [`M5ManifestBuildComponentFamily::SchemaValidatorRow`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaValidatorRowDescriptor {
    /// The validator verdict.
    pub validation_state: M5SchemaValidationState,
    /// The freshness of the schema being validated against.
    pub schema_freshness: M5SchemaFreshness,
    /// The row blocks apply when the validation state requires it; must be
    /// consistent with [`M5SchemaValidationState::must_block_apply`].
    pub blocks_apply_on_error: bool,
}

impl SchemaValidatorRowDescriptor {
    /// Whether the validator row descriptor is internally complete and honest.
    pub fn is_honest(&self) -> bool {
        if self.validation_state.must_block_apply() {
            return self.blocks_apply_on_error;
        }
        // A valid / warnings / unversioned state discloses schema freshness but does
        // not force a block. A resolved schema state with an unavailable schema is a
        // contradiction.
        if self.validation_state != M5SchemaValidationState::SchemaUnavailable
            && self.schema_freshness == M5SchemaFreshness::Unavailable
        {
            return false;
        }
        true
    }
}

/// A target-context chip group descriptor. Present only on a
/// [`M5ManifestBuildComponentFamily::TargetContextChipGroup`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetContextChipGroupDescriptor {
    /// The truth class the chip group discloses; must match the row's truth mode.
    pub truth_mode: TruthMode,
    /// Opaque ref to the target identity the chips name; never raw endpoint data.
    pub target_identity_ref: String,
    /// Target identity, environment, and scope are all shown; must hold.
    pub context_complete: bool,
    /// The chip group stays visible as the surface scrolls; must hold.
    pub stays_visible_on_scroll: bool,
}

impl TargetContextChipGroupDescriptor {
    /// Whether the chip group descriptor is internally complete and honest.
    pub fn is_honest(&self) -> bool {
        !self.target_identity_ref.trim().is_empty()
            && self.context_complete
            && self.stays_visible_on_scroll
    }
}

/// Closed resource-link-class vocabulary. Names which two truth classes a link
/// joins so authored / rendered / live truth never blurs across the link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResourceLinkClass {
    /// Links an authored resource to its rendered output.
    AuthoredToRendered,
    /// Links a rendered resource to its live counterpart.
    RenderedToLive,
    /// Links a planned change to its live target.
    PlanToLive,
    /// Links a resource to its backing schema.
    SchemaBacked,
    /// Links a resource across targets / scopes.
    CrossTarget,
}

impl M5ResourceLinkClass {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoredToRendered => "authored_to_rendered",
            Self::RenderedToLive => "rendered_to_live",
            Self::PlanToLive => "plan_to_live",
            Self::SchemaBacked => "schema_backed",
            Self::CrossTarget => "cross_target",
        }
    }
}

/// A resource-link row descriptor. Present only on a
/// [`M5ManifestBuildComponentFamily::ResourceLinkRow`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceLinkRowDescriptor {
    /// Which two truth classes this link joins.
    pub link_class: M5ResourceLinkClass,
    /// The truth class on the "from" side of the link.
    pub from_truth: TruthMode,
    /// The truth class on the "to" side of the link.
    pub to_truth: TruthMode,
    /// The confidence of the discovered link.
    pub confidence: M5DiscoveryConfidence,
    /// The link never overwrites a higher-confidence resource silently; must hold.
    pub never_overwrites_higher_confidence: bool,
}

impl ResourceLinkRowDescriptor {
    /// Whether the resource-link descriptor is internally complete and honest: it
    /// preserves the never-silent-overwrite invariant and does not collapse the two
    /// sides of the link into a single blurred truth class.
    pub fn is_honest(&self) -> bool {
        self.never_overwrites_higher_confidence && self.from_truth != self.to_truth
    }
}

/// Closed resource-freshness vocabulary. Names how fresh an explorer row's data is
/// so a cached or imported snapshot never reads as live truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResourceFreshness {
    /// Live and freshly observed.
    LiveFresh,
    /// Cached and known to be stale.
    CachedStale,
    /// An imported snapshot from a prior run.
    ImportedSnapshot,
    /// Planned / desired only, not yet live.
    PlanOnly,
    /// Freshness not yet established.
    Unknown,
}

impl M5ResourceFreshness {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveFresh => "live_fresh",
            Self::CachedStale => "cached_stale",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::PlanOnly => "plan_only",
            Self::Unknown => "unknown",
        }
    }

    /// True when the row is live and current.
    pub const fn is_live_fresh(self) -> bool {
        matches!(self, Self::LiveFresh)
    }
}

/// A resource-explorer row descriptor. Present only on a
/// [`M5ManifestBuildComponentFamily::ResourceExplorerRow`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceExplorerRowDescriptor {
    /// The truth class the resource is shown in.
    pub truth_mode: TruthMode,
    /// How fresh the resource data is.
    pub freshness: M5ResourceFreshness,
    /// The confidence of the discovered resource.
    pub confidence: M5DiscoveryConfidence,
    /// The explorer row keeps target context visible; must hold.
    pub target_context_visible: bool,
}

impl ResourceExplorerRowDescriptor {
    /// Whether the resource-explorer descriptor is internally complete and honest: a
    /// live-fresh row must be shown in a live truth class, and target context is
    /// always visible.
    pub fn is_honest(&self) -> bool {
        if !self.target_context_visible {
            return false;
        }
        // Live-fresh data must be presented as live truth, never as authored or
        // planned truth.
        if self.freshness.is_live_fresh() && self.truth_mode != TruthMode::Live {
            return false;
        }
        true
    }
}

/// An adapter-source badge descriptor. Present only on a
/// [`M5ManifestBuildComponentFamily::AdapterSourceBadge`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterSourceBadgeDescriptor {
    /// Where the build / target truth came from; must match the row's adapter
    /// source.
    pub adapter_source: M5AdapterSourceKind,
    /// The confidence the badge discloses; must be consistent with the source kind.
    pub confidence: M5DiscoveryConfidence,
    /// The adapter source kind is rendered explicitly; must hold.
    pub source_kind_explicit: bool,
}

impl AdapterSourceBadgeDescriptor {
    /// Whether the adapter-source badge descriptor is internally complete and
    /// honest: the source kind is explicit and the confidence is consistent with the
    /// source.
    pub fn is_honest(&self) -> bool {
        self.source_kind_explicit && self.adapter_source.confidence_consistent(self.confidence)
    }
}

/// Closed target-graph node-kind vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TargetGraphNodeKind {
    /// A build target node.
    BuildTarget,
    /// A test target node.
    TestTarget,
    /// A run / launch target node.
    RunTarget,
    /// A dependency edge between targets.
    DependencyEdge,
    /// A container / runtime target node.
    ContainerTarget,
}

impl M5TargetGraphNodeKind {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildTarget => "build_target",
            Self::TestTarget => "test_target",
            Self::RunTarget => "run_target",
            Self::DependencyEdge => "dependency_edge",
            Self::ContainerTarget => "container_target",
        }
    }
}

/// A target-graph row descriptor. Present only on a
/// [`M5ManifestBuildComponentFamily::TargetGraphRow`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetGraphRowDescriptor {
    /// What kind of graph node this row is.
    pub node_kind: M5TargetGraphNodeKind,
    /// The truth class the node is shown in.
    pub truth_mode: TruthMode,
    /// The confidence of the discovered node / edge.
    pub edge_confidence: M5DiscoveryConfidence,
    /// Opaque ref to the target identity; never a raw path or label.
    pub target_identity_ref: String,
}

impl TargetGraphRowDescriptor {
    /// Whether the target-graph descriptor is internally complete and honest: it
    /// keeps a resolvable target identity visible.
    pub fn is_honest(&self) -> bool {
        !self.target_identity_ref.trim().is_empty()
    }
}

/// Closed capability-state vocabulary. Names whether a capability is supported so an
/// unknown or unsupported capability never reads as available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilityState {
    /// Fully supported.
    Supported,
    /// Partially supported.
    Partial,
    /// Not supported.
    Unsupported,
    /// Support not yet established.
    Unknown,
    /// Gated behind provider / policy configuration.
    ProviderGated,
}

impl M5CapabilityState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Partial => "partial",
            Self::Unsupported => "unsupported",
            Self::Unknown => "unknown",
            Self::ProviderGated => "provider_gated",
        }
    }
}

/// A capability-matrix cell descriptor. Present only on a
/// [`M5ManifestBuildComponentFamily::CapabilityMatrix`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityMatrixDescriptor {
    /// The capability state this cell discloses.
    pub capability_state: M5CapabilityState,
    /// The adapter source the capability is derived from.
    pub adapter_source: M5AdapterSourceKind,
    /// The cell discloses its adapter source and confidence; must hold.
    pub discloses_source_and_confidence: bool,
    /// The confidence of the capability determination.
    pub confidence: M5DiscoveryConfidence,
}

impl CapabilityMatrixDescriptor {
    /// Whether the capability descriptor is internally complete and honest: it
    /// discloses source and confidence, and a supported capability is never claimed
    /// from an unknown-confidence source.
    pub fn is_honest(&self) -> bool {
        if !self.discloses_source_and_confidence {
            return false;
        }
        if self.capability_state == M5CapabilityState::Supported
            && self.confidence == M5DiscoveryConfidence::Unknown
        {
            return false;
        }
        self.adapter_source.confidence_consistent(self.confidence)
    }
}

/// Closed raw-event channel vocabulary. Names the provenance of a raw event so a
/// heuristic parse is never presented as a native event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RawEventChannel {
    /// Native build-event protocol stream (e.g. BEP).
    NativeBuildEvent,
    /// Native build-server protocol stream (e.g. BSP).
    NativeBuildServer,
    /// The internal task-event bus.
    TaskEventBus,
    /// A heuristic parse of build / test output.
    HeuristicParse,
    /// An imported log from a prior run.
    ImportedLog,
}

impl M5RawEventChannel {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeBuildEvent => "native_build_event",
            Self::NativeBuildServer => "native_build_server",
            Self::TaskEventBus => "task_event_bus",
            Self::HeuristicParse => "heuristic_parse",
            Self::ImportedLog => "imported_log",
        }
    }
}

/// A raw-event drawer descriptor. Present only on a
/// [`M5ManifestBuildComponentFamily::RawEventDrawer`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawEventDrawerDescriptor {
    /// The provenance channel of the raw events.
    pub event_channel: M5RawEventChannel,
    /// Raw payloads are redacted to typed tokens before export; must hold.
    pub redaction_applied: bool,
    /// The drawer preserves stable event identity across export; must hold.
    pub preserves_event_identity: bool,
}

impl RawEventDrawerDescriptor {
    /// Whether the raw-event drawer descriptor is internally complete and honest.
    pub fn is_honest(&self) -> bool {
        self.redaction_applied && self.preserves_event_identity
    }
}

/// Closed fallback-confidence-state vocabulary. Names whether truth came from a
/// structured channel or a heuristic fallback so a fallback never claims structured
/// confidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FallbackConfidenceState {
    /// Structured, high-confidence native truth.
    StructuredHigh,
    /// Structured but degraded (partial native truth).
    StructuredDegraded,
    /// Heuristic fallback truth.
    HeuristicFallback,
    /// Imported-only truth from a prior run.
    ImportedOnly,
    /// Confidence not yet established.
    Unknown,
}

impl M5FallbackConfidenceState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StructuredHigh => "structured_high",
            Self::StructuredDegraded => "structured_degraded",
            Self::HeuristicFallback => "heuristic_fallback",
            Self::ImportedOnly => "imported_only",
            Self::Unknown => "unknown",
        }
    }

    /// True when this state is a fallback below structured confidence.
    pub const fn is_fallback(self) -> bool {
        matches!(
            self,
            Self::HeuristicFallback | Self::ImportedOnly | Self::Unknown
        )
    }
}

/// Closed fallback-reason vocabulary. Names why confidence fell so the narrowing is
/// reconstructable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FallbackReason {
    /// The build adapter was unavailable.
    AdapterUnavailable,
    /// The structured event channel was lost mid-session.
    StructuredChannelLost,
    /// The schema drifted from the target.
    SchemaDrift,
    /// A live connector was lost.
    ConnectorLoss,
    /// A policy / capability block prevented structured access.
    PolicyBlock,
}

impl M5FallbackReason {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdapterUnavailable => "adapter_unavailable",
            Self::StructuredChannelLost => "structured_channel_lost",
            Self::SchemaDrift => "schema_drift",
            Self::ConnectorLoss => "connector_loss",
            Self::PolicyBlock => "policy_block",
        }
    }
}

/// Closed fallback-recovery-route vocabulary. Every fallback names a real recovery
/// route rather than a dead end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FallbackRecoveryRoute {
    /// Reattach the build adapter.
    ReattachAdapter,
    /// Re-run discovery from the target.
    RerunDiscovery,
    /// Inspect-only; no structured recovery is offered.
    InspectOnly,
    /// Open the canonical source truth.
    OpenSourceTruth,
    /// Retry the live connector.
    RetryConnector,
}

impl M5FallbackRecoveryRoute {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReattachAdapter => "reattach_adapter",
            Self::RerunDiscovery => "rerun_discovery",
            Self::InspectOnly => "inspect_only",
            Self::OpenSourceTruth => "open_source_truth",
            Self::RetryConnector => "retry_connector",
        }
    }
}

/// A fallback-confidence drawer descriptor. Present only on a
/// [`M5ManifestBuildComponentFamily::FallbackConfidenceDrawer`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackConfidenceDrawerDescriptor {
    /// The structured-versus-heuristic confidence state.
    pub confidence_state: M5FallbackConfidenceState,
    /// Why confidence fell; present only when the state is a fallback.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_reason: Option<M5FallbackReason>,
    /// The recovery route offered.
    pub recovery_route: M5FallbackRecoveryRoute,
    /// Lower-confidence fallback never overwrites structured truth silently; must
    /// hold.
    pub never_overwrites_structured_silently: bool,
}

impl FallbackConfidenceDrawerDescriptor {
    /// Whether the fallback drawer descriptor is internally complete and honest: a
    /// fallback names why it fell and offers a real recovery route, and a structured
    /// state never claims a fallback reason.
    pub fn is_honest(&self) -> bool {
        if !self.never_overwrites_structured_silently {
            return false;
        }
        if self.confidence_state.is_fallback() {
            // A real fallback names its reason and offers a route other than a dead
            // end that is not merely inspect-only when structured truth is missing.
            self.fallback_reason.is_some()
        } else {
            // A structured state carries no fallback reason.
            self.fallback_reason.is_none()
        }
    }
}

/// Closed required-label vocabulary. Names the labels a reusable manifest / build
/// component must render; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManifestBuildRequiredLabel {
    /// The component's stable identity.
    Identity,
    /// The target context the component acts on.
    TargetContext,
    /// The authored / rendered / planned / live / provider-overlay truth class.
    TruthClass,
    /// Schema freshness or discovery confidence.
    FreshnessOrConfidence,
    /// The adapter source kind, where applicable.
    AdapterSource,
    /// The keyboard / assistive route into the component.
    KeyboardRoute,
}

impl M5ManifestBuildRequiredLabel {
    /// Every required label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::TargetContext,
        Self::TruthClass,
        Self::FreshnessOrConfidence,
        Self::AdapterSource,
        Self::KeyboardRoute,
    ];

    /// The mandatory subset that must appear on every row.
    pub const MANDATORY: [Self; 4] = [
        Self::Identity,
        Self::TargetContext,
        Self::TruthClass,
        Self::KeyboardRoute,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::TargetContext => "target_context",
            Self::TruthClass => "truth_class",
            Self::FreshnessOrConfidence => "freshness_or_confidence",
            Self::AdapterSource => "adapter_source",
            Self::KeyboardRoute => "keyboard_route",
        }
    }
}

/// Closed downgrade-trigger vocabulary. Names why a component row is in a degraded
/// state so support can reconstruct the narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManifestBuildDowngradeTrigger {
    /// The backing schema is stale, unversioned, or unavailable.
    SchemaStale,
    /// The build adapter is unavailable.
    AdapterUnavailable,
    /// A live connector was lost.
    ConnectorLoss,
    /// A policy / capability block narrowed the action.
    PolicyBlock,
    /// The surface drifted from canonical source / target.
    DriftFromSource,
    /// A discovery result is low-confidence.
    LowConfidenceDiscovery,
    /// The structured event / adapter channel was lost, forcing a fallback.
    StructuredChannelLost,
    /// The target context could not be resolved.
    TargetContextUnresolved,
}

impl M5ManifestBuildDowngradeTrigger {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchemaStale => "schema_stale",
            Self::AdapterUnavailable => "adapter_unavailable",
            Self::ConnectorLoss => "connector_loss",
            Self::PolicyBlock => "policy_block",
            Self::DriftFromSource => "drift_from_source",
            Self::LowConfidenceDiscovery => "low_confidence_discovery",
            Self::StructuredChannelLost => "structured_channel_lost",
            Self::TargetContextUnresolved => "target_context_unresolved",
        }
    }
}

/// A typed degraded-state block. When present, the component is narrowed below its
/// full capability and names why with an explicit, non-generic label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradedState {
    /// Why the component is degraded.
    pub trigger: M5ManifestBuildDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub degraded_label: String,
}

impl DegradedState {
    /// Whether the degraded label is precise rather than a generic non-answer.
    pub fn is_honest(&self) -> bool {
        !label_is_generic(&self.degraded_label)
    }
}

/// One reusable manifest / build-confidence component: the shared truth row every
/// consumer surface ingests instead of cloning infra / build chrome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentRow {
    /// Stable component id.
    pub component_id: String,
    /// Which reusable component family this row is.
    pub family: M5ManifestBuildComponentFamily,
    /// Human-readable label of the surface the component appears on.
    pub surface_label: String,
    /// The authored / rendered / planned / live / provider-overlay truth class the
    /// component binds to (reused vocabulary).
    pub truth_mode: TruthMode,
    /// Opaque ref to the target context the component acts on; target context stays
    /// visible on every read- or mutate-capable surface, so this is never empty.
    pub target_context_ref: String,
    /// The adapter source kind the component derives from (explicit on every row).
    pub adapter_source: M5AdapterSourceKind,
    /// The required labels this component renders; must include every mandatory
    /// label.
    pub required_labels: Vec<M5ManifestBuildRequiredLabel>,
    /// The component projects an export-safe support summary; must hold.
    pub export_safe: bool,
    /// The component exposes a keyboard / assistive route; must hold.
    pub assistive_ready: bool,
    /// The manifest-editor header descriptor, present only for a header row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_editor_header: Option<ManifestEditorHeaderDescriptor>,
    /// The schema / validator descriptor, present only for a validator row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_validator_row: Option<SchemaValidatorRowDescriptor>,
    /// The target-context chip group descriptor, present only for a chip-group row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_context_chip_group: Option<TargetContextChipGroupDescriptor>,
    /// The resource-link descriptor, present only for a resource-link row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_link_row: Option<ResourceLinkRowDescriptor>,
    /// The resource-explorer descriptor, present only for a resource-explorer row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_explorer_row: Option<ResourceExplorerRowDescriptor>,
    /// The adapter-source badge descriptor, present only for a badge row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adapter_source_badge: Option<AdapterSourceBadgeDescriptor>,
    /// The target-graph descriptor, present only for a target-graph row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_graph_row: Option<TargetGraphRowDescriptor>,
    /// The capability-matrix descriptor, present only for a capability-matrix row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability_matrix: Option<CapabilityMatrixDescriptor>,
    /// The raw-event drawer descriptor, present only for a raw-event-drawer row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_event_drawer: Option<RawEventDrawerDescriptor>,
    /// The fallback-confidence drawer descriptor, present only for a fallback row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_confidence_drawer: Option<FallbackConfidenceDrawerDescriptor>,
    /// The typed degraded-state block, present only when the component is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
    /// Human-readable label summary safe to render on the row.
    pub label_summary: String,
    /// ISO 8601 UTC timestamp the component state was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
}

impl ComponentRow {
    /// Whether the family-specific payload is present exactly for this family and
    /// absent for every other family.
    pub fn payload_matches_family(&self) -> bool {
        let present = [
            self.manifest_editor_header.is_some(),
            self.schema_validator_row.is_some(),
            self.target_context_chip_group.is_some(),
            self.resource_link_row.is_some(),
            self.resource_explorer_row.is_some(),
            self.adapter_source_badge.is_some(),
            self.target_graph_row.is_some(),
            self.capability_matrix.is_some(),
            self.raw_event_drawer.is_some(),
            self.fallback_confidence_drawer.is_some(),
        ];
        // Exactly one payload present, and it is the one this family names.
        if present.iter().filter(|p| **p).count() != 1 {
            return false;
        }
        match self.family {
            M5ManifestBuildComponentFamily::ManifestEditorHeader => {
                self.manifest_editor_header.is_some()
            }
            M5ManifestBuildComponentFamily::SchemaValidatorRow => {
                self.schema_validator_row.is_some()
            }
            M5ManifestBuildComponentFamily::TargetContextChipGroup => {
                self.target_context_chip_group.is_some()
            }
            M5ManifestBuildComponentFamily::ResourceLinkRow => self.resource_link_row.is_some(),
            M5ManifestBuildComponentFamily::ResourceExplorerRow => {
                self.resource_explorer_row.is_some()
            }
            M5ManifestBuildComponentFamily::AdapterSourceBadge => {
                self.adapter_source_badge.is_some()
            }
            M5ManifestBuildComponentFamily::TargetGraphRow => self.target_graph_row.is_some(),
            M5ManifestBuildComponentFamily::CapabilityMatrix => self.capability_matrix.is_some(),
            M5ManifestBuildComponentFamily::RawEventDrawer => self.raw_event_drawer.is_some(),
            M5ManifestBuildComponentFamily::FallbackConfidenceDrawer => {
                self.fallback_confidence_drawer.is_some()
            }
        }
    }

    /// Whether the family payload, where present, is internally honest.
    pub fn payload_honest(&self) -> bool {
        self.manifest_editor_header
            .as_ref()
            .map_or(true, |d| d.is_honest())
            && self
                .schema_validator_row
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .target_context_chip_group
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .resource_link_row
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .resource_explorer_row
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .adapter_source_badge
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .target_graph_row
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .capability_matrix
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .raw_event_drawer
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .fallback_confidence_drawer
                .as_ref()
                .map_or(true, |d| d.is_honest())
    }

    /// Whether a truth-bearing descriptor discloses the same truth class the row
    /// records (a chip / explorer / graph / link never invents a second truth
    /// story), and the adapter-source badge matches the row's adapter source.
    pub fn descriptor_matches_row(&self) -> bool {
        let chip_ok = self
            .target_context_chip_group
            .as_ref()
            .map_or(true, |c| c.truth_mode == self.truth_mode);
        let explorer_ok = self
            .resource_explorer_row
            .as_ref()
            .map_or(true, |e| e.truth_mode == self.truth_mode);
        let graph_ok = self
            .target_graph_row
            .as_ref()
            .map_or(true, |g| g.truth_mode == self.truth_mode);
        let header_ok = self
            .manifest_editor_header
            .as_ref()
            .map_or(true, |h| h.truth_mode == self.truth_mode);
        let badge_ok = self
            .adapter_source_badge
            .as_ref()
            .map_or(true, |b| b.adapter_source == self.adapter_source);
        chip_ok && explorer_ok && graph_ok && header_ok && badge_ok
    }

    /// Whether every mandatory required label is present on the row.
    pub fn mandatory_labels_present(&self) -> bool {
        let present: BTreeSet<M5ManifestBuildRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ManifestBuildRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the degraded block, when present, is honest.
    pub fn degraded_ok(&self) -> bool {
        self.degraded.as_ref().map_or(true, |d| d.is_honest())
    }

    /// True when this row is a complete, honest degraded / narrowed component.
    pub fn is_degraded(&self) -> bool {
        self.degraded.is_some() && self.is_complete()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} truth={truth} adapter={adapter} \
export_safe={export_safe} assistive={assistive}",
            family = self.family.as_str(),
            truth = truth_mode_token(self.truth_mode),
            adapter = self.adapter_source.as_str(),
            export_safe = self.export_safe,
            assistive = self.assistive_ready,
        )
    }

    /// Whether every dimension required to record this row is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.component_id.trim().is_empty()
            && !self.surface_label.trim().is_empty()
            && !self.target_context_ref.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.observed_at.trim().is_empty()
            && self.export_safe
            && self.assistive_ready
            && self.payload_matches_family()
            && self.payload_honest()
            && self.descriptor_matches_row()
            && self.mandatory_labels_present()
            && self.degraded_ok()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block for the manifest / build-confidence component matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestBuildGuardrails {
    /// Authored, rendered, planned, live, cached, and provider-overlay truth never
    /// blurs.
    pub truth_classes_never_blur: bool,
    /// Target context stays visible on every read- or mutate-capable surface.
    pub target_context_visible_on_every_surface: bool,
    /// Schema freshness and adapter source kind are explicit.
    pub schema_freshness_and_adapter_source_explicit: bool,
    /// Lower-confidence discovery / results never overwrite higher-confidence truth
    /// silently.
    pub lower_confidence_never_overwrites_silently: bool,
    /// Drift, connector loss, and policy blocks narrow actions before execution.
    pub drift_connector_loss_policy_narrow_before_execution: bool,
    /// Exported evidence preserves the same target IDs, adapter kinds, and
    /// freshness / confidence states shown in-product.
    pub exported_evidence_preserves_ids_kinds_and_states: bool,
    /// Components bind to the shared truth, preview/apply/review, and degraded-state
    /// vocabulary rather than bespoke adapter / connector chrome.
    pub components_bound_to_shared_vocabulary: bool,
    /// The matrix does not widen into new build adapters, live-resource connectors,
    /// or infra mutation engines.
    pub no_new_adapters_connectors_or_engines: bool,
}

impl ManifestBuildGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.truth_classes_never_blur
            && self.target_context_visible_on_every_surface
            && self.schema_freshness_and_adapter_source_explicit
            && self.lower_confidence_never_overwrites_silently
            && self.drift_connector_loss_policy_narrow_before_execution
            && self.exported_evidence_preserves_ids_kinds_and_states
            && self.components_bound_to_shared_vocabulary
            && self.no_new_adapters_connectors_or_engines
    }
}

/// Consumer-projection block for the manifest / build-confidence component matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestBuildConsumerProjection {
    /// Product surfaces ingest these component rows instead of cloning chrome.
    pub product_ingests_components: bool,
    /// Docs / help ingests the same component rows.
    pub docs_help_ingests_components: bool,
    /// Diagnostics ingests the same component rows.
    pub diagnostics_ingests_components: bool,
    /// Support export ingests the same component rows.
    pub support_export_ingests_components: bool,
    /// Release-control surfaces ingest the same component rows.
    pub release_control_ingests_components: bool,
    /// Later M5 rows reference one canonical component family instead of restating
    /// infra / build confidence truth in feature-local prose.
    pub later_rows_reference_one_canonical_family: bool,
}

impl ManifestBuildConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_components
            && self.docs_help_ingests_components
            && self.diagnostics_ingests_components
            && self.support_export_ingests_components
            && self.release_control_ingests_components
            && self.later_rows_reference_one_canonical_family
    }
}

/// Constructor input for [`ManifestBuildComponentMatrix::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestBuildComponentMatrixInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub set_label: String,
    /// Per-component rows.
    pub components: Vec<ComponentRow>,
    /// Guardrail invariants block.
    pub guardrails: ManifestBuildGuardrails,
    /// Consumer projection block.
    pub consumer_projection: ManifestBuildConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe manifest / build-confidence component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestBuildComponentMatrix {
    /// Record kind; must equal [`MANIFEST_BUILD_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MANIFEST_BUILD_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub set_label: String,
    /// Per-component rows.
    pub components: Vec<ComponentRow>,
    /// Guardrail invariants block.
    pub guardrails: ManifestBuildGuardrails,
    /// Consumer projection block.
    pub consumer_projection: ManifestBuildConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ManifestBuildComponentMatrix {
    /// Builds a manifest / build-confidence component matrix packet.
    pub fn new(input: ManifestBuildComponentMatrixInput) -> Self {
        Self {
            record_kind: MANIFEST_BUILD_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: MANIFEST_BUILD_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            set_label: input.set_label,
            components: input.components,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Families represented by some row in this matrix.
    pub fn represented_families(&self) -> BTreeSet<M5ManifestBuildComponentFamily> {
        self.components.iter().map(|r| r.family).collect()
    }

    /// Count of rows that are complete, honest degraded / narrowed components.
    pub fn degraded_row_count(&self) -> usize {
        self.components.iter().filter(|r| r.is_degraded()).count()
    }

    /// Validates the manifest / build-confidence component matrix invariants.
    pub fn validate(&self) -> Vec<ManifestBuildComponentViolation> {
        let mut violations = Vec::new();

        if self.record_kind != MANIFEST_BUILD_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(ManifestBuildComponentViolation::WrongRecordKind);
        }
        if self.schema_version != MANIFEST_BUILD_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(ManifestBuildComponentViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.set_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ManifestBuildComponentViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("manifest/build component matrix serializes"),
        ) {
            violations.push(ManifestBuildComponentViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("manifest/build component matrix serializes")
    }

    /// Deterministic CSV of the component rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "component_id,family,truth_mode,adapter_source,export_safe,assistive_ready,degraded\n",
        );
        for row in &self.components {
            out.push_str(&format!(
                "{id},{family},{truth},{adapter},{export_safe},{assistive},{degraded}\n",
                id = row.component_id,
                family = row.family.as_str(),
                truth = truth_mode_token(row.truth_mode),
                adapter = row.adapter_source.as_str(),
                export_safe = row.export_safe,
                assistive = row.assistive_ready,
                degraded = row.degraded.as_ref().map_or("none", |d| d.trigger.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Manifest / Build Component Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.set_label));
        out.push_str(&format!(
            "- Components: {} across {} / {} families ({} degraded)\n",
            self.components.len(),
            self.represented_families().len(),
            M5ManifestBuildComponentFamily::ALL.len(),
            self.degraded_row_count(),
        ));
        out.push_str("\n## Components\n\n");
        for row in &self.components {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.component_id,
                row.family.as_str(),
                row.surface_label,
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!("  - {}\n", row.chip_tokens()));
            if let Some(degraded) = &row.degraded {
                out.push_str(&format!(
                    "  - Degraded: trigger={} — {}\n",
                    degraded.trigger.as_str(),
                    degraded.degraded_label,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in manifest / build component export.
#[derive(Debug)]
pub enum ManifestBuildComponentArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ManifestBuildComponentViolation>),
}

impl fmt::Display for ManifestBuildComponentArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "manifest/build component export parse failed: {error}"
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
                    "manifest/build component export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ManifestBuildComponentArtifactError {}

/// Validation failures emitted by [`ManifestBuildComponentMatrix::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManifestBuildComponentViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required reusable component family is defined by no row.
    RequiredFamilyMissing,
    /// The matrix demonstrates no complete degraded / narrowed row.
    DegradedCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A row's family-specific payload is missing, extra, or wrong for its family.
    PayloadFamilyMismatch,
    /// A row's family payload is internally dishonest.
    PayloadDishonest,
    /// A truth-bearing descriptor discloses a class different from its row, or a
    /// badge discloses a different adapter source than its row.
    DescriptorRowMismatch,
    /// A row omits a mandatory required label.
    MandatoryLabelMissing,
    /// A row is not export-safe or not assistive-ready.
    ParityMissing,
    /// A degraded block carries a generic non-answer label.
    DegradedLabelGeneric,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ManifestBuildComponentViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::DegradedCaseMissing => "degraded_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::PayloadFamilyMismatch => "payload_family_mismatch",
            Self::PayloadDishonest => "payload_dishonest",
            Self::DescriptorRowMismatch => "descriptor_row_mismatch",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ParityMissing => "parity_missing",
            Self::DegradedLabelGeneric => "degraded_label_generic",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in manifest / build component export.
pub fn current_m5_manifest_build_component_matrix_export(
) -> Result<ManifestBuildComponentMatrix, ManifestBuildComponentArtifactError> {
    let packet: ManifestBuildComponentMatrix = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/infra/m5-manifest-build-component-matrix/support_export.json"
    )))
    .map_err(ManifestBuildComponentArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ManifestBuildComponentArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &ManifestBuildComponentMatrix,
    violations: &mut Vec<ManifestBuildComponentViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        MANIFEST_BUILD_COMPONENT_MATRIX_SCHEMA_REF,
        MANIFEST_BUILD_COMPONENT_MATRIX_DOC_REF,
        MANIFEST_BUILD_COMPONENT_MATRIX_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ManifestBuildComponentViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &ManifestBuildComponentMatrix,
    violations: &mut Vec<ManifestBuildComponentViolation>,
) {
    let families = packet.represented_families();
    for required in M5ManifestBuildComponentFamily::ALL {
        if !families.contains(&required) {
            violations.push(ManifestBuildComponentViolation::RequiredFamilyMissing);
            break;
        }
    }
    if packet.degraded_row_count() == 0 {
        violations.push(ManifestBuildComponentViolation::DegradedCaseMissing);
    }
}

fn validate_rows(
    packet: &ManifestBuildComponentMatrix,
    violations: &mut Vec<ManifestBuildComponentViolation>,
) {
    for row in &packet.components {
        if !row.is_complete() {
            violations.push(ManifestBuildComponentViolation::RowIncomplete);
        }
        if !row.payload_matches_family() {
            violations.push(ManifestBuildComponentViolation::PayloadFamilyMismatch);
        }
        if !row.payload_honest() {
            violations.push(ManifestBuildComponentViolation::PayloadDishonest);
        }
        if !row.descriptor_matches_row() {
            violations.push(ManifestBuildComponentViolation::DescriptorRowMismatch);
        }
        if !row.mandatory_labels_present() {
            violations.push(ManifestBuildComponentViolation::MandatoryLabelMissing);
        }
        if !row.export_safe || !row.assistive_ready {
            violations.push(ManifestBuildComponentViolation::ParityMissing);
        }
        if !row.degraded_ok() {
            violations.push(ManifestBuildComponentViolation::DegradedLabelGeneric);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(ManifestBuildComponentViolation::RowEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &ManifestBuildComponentMatrix,
    violations: &mut Vec<ManifestBuildComponentViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(ManifestBuildComponentViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &ManifestBuildComponentMatrix,
    violations: &mut Vec<ManifestBuildComponentViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(ManifestBuildComponentViolation::ConsumerProjectionIncomplete);
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "stale"
            | "no data"
            | "blocked"
            | "degraded"
            | "fallback"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds the canonical, checked-in manifest / build component matrix packet. This
/// is the one source of truth shared by the tests, the fixture-emitting bin, and
/// the on-disk support export so all three stay byte-aligned.
pub fn seeded_manifest_build_component_matrix() -> ManifestBuildComponentMatrix {
    ManifestBuildComponentMatrix::new(ManifestBuildComponentMatrixInput {
        packet_id: "m5-manifest-build-component-matrix:stable:0001".to_owned(),
        set_label: "M5 Manifest / Build Component Matrix".to_owned(),
        components: seeded_components(),
        guardrails: seeded_guardrails(),
        consumer_projection: seeded_consumer_projection(),
        source_contract_refs: seeded_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-04T00:00:00Z".to_owned(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:manifest-build:{id}")]
}

fn mandatory_labels() -> Vec<M5ManifestBuildRequiredLabel> {
    vec![
        M5ManifestBuildRequiredLabel::Identity,
        M5ManifestBuildRequiredLabel::TargetContext,
        M5ManifestBuildRequiredLabel::TruthClass,
        M5ManifestBuildRequiredLabel::FreshnessOrConfidence,
        M5ManifestBuildRequiredLabel::AdapterSource,
        M5ManifestBuildRequiredLabel::KeyboardRoute,
    ]
}

fn seeded_components() -> Vec<ComponentRow> {
    vec![
        // Manifest-editor header — editable through the shared preview/apply/review
        // path over a fresh schema, target context visible.
        ComponentRow {
            component_id: "component:manifest-editor-header:0001".to_owned(),
            family: M5ManifestBuildComponentFamily::ManifestEditorHeader,
            surface_label: "Manifest editor header for an authored infrastructure manifest".to_owned(),
            truth_mode: TruthMode::Desired,
            target_context_ref: "target_context:manifest:0001".to_owned(),
            adapter_source: M5AdapterSourceKind::NativeBuildServer,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            manifest_editor_header: Some(ManifestEditorHeaderDescriptor {
                truth_mode: TruthMode::Desired,
                schema_freshness: M5SchemaFreshness::Fresh,
                edit_posture: M5ManifestEditPosture::PreviewApplyReview,
                target_context_visible: true,
                manifest_ref: "manifest:infra:0001".to_owned(),
            }),
            schema_validator_row: None,
            target_context_chip_group: None,
            resource_link_row: None,
            resource_explorer_row: None,
            adapter_source_badge: None,
            target_graph_row: None,
            capability_matrix: None,
            raw_event_drawer: None,
            fallback_confidence_drawer: None,
            degraded: None,
            label_summary: "An authored manifest header keeps target context visible and offers a preview/apply/review edit posture over a fresh schema".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("manifest-editor-header:0001"),
        },
        // Schema/validator row — errors block apply.
        ComponentRow {
            component_id: "component:schema-validator-row:0001".to_owned(),
            family: M5ManifestBuildComponentFamily::SchemaValidatorRow,
            surface_label: "Schema validator row reporting errors against a fresh schema".to_owned(),
            truth_mode: TruthMode::Plan,
            target_context_ref: "target_context:manifest:0001".to_owned(),
            adapter_source: M5AdapterSourceKind::NativeBuildServer,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            manifest_editor_header: None,
            schema_validator_row: Some(SchemaValidatorRowDescriptor {
                validation_state: M5SchemaValidationState::Errors,
                schema_freshness: M5SchemaFreshness::Fresh,
                blocks_apply_on_error: true,
            }),
            target_context_chip_group: None,
            resource_link_row: None,
            resource_explorer_row: None,
            adapter_source_badge: None,
            target_graph_row: None,
            capability_matrix: None,
            raw_event_drawer: None,
            fallback_confidence_drawer: None,
            degraded: Some(DegradedState {
                trigger: M5ManifestBuildDowngradeTrigger::PolicyBlock,
                degraded_label: "The manifest fails schema validation; apply is blocked until the reported errors are resolved".to_owned(),
            }),
            label_summary: "A validator row reports schema errors and blocks apply rather than let an invalid manifest through".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("schema-validator-row:0001"),
        },
        // Schema/validator row — stale schema narrows.
        ComponentRow {
            component_id: "component:schema-validator-row:0002".to_owned(),
            family: M5ManifestBuildComponentFamily::SchemaValidatorRow,
            surface_label: "Schema validator row over a stale schema".to_owned(),
            truth_mode: TruthMode::Plan,
            target_context_ref: "target_context:manifest:0002".to_owned(),
            adapter_source: M5AdapterSourceKind::HeuristicParse,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            manifest_editor_header: None,
            schema_validator_row: Some(SchemaValidatorRowDescriptor {
                validation_state: M5SchemaValidationState::Warnings,
                schema_freshness: M5SchemaFreshness::Stale,
                blocks_apply_on_error: false,
            }),
            target_context_chip_group: None,
            resource_link_row: None,
            resource_explorer_row: None,
            adapter_source_badge: None,
            target_graph_row: None,
            capability_matrix: None,
            raw_event_drawer: None,
            fallback_confidence_drawer: None,
            degraded: Some(DegradedState {
                trigger: M5ManifestBuildDowngradeTrigger::SchemaStale,
                degraded_label: "The backing schema is stale; validation warnings are advisory until the schema is refreshed".to_owned(),
            }),
            label_summary: "A validator row discloses a stale schema and marks its warnings advisory rather than authoritative".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("schema-validator-row:0002"),
        },
        // Target-context chip group — live truth, complete and pinned.
        ComponentRow {
            component_id: "component:target-context-chip-group:0001".to_owned(),
            family: M5ManifestBuildComponentFamily::TargetContextChipGroup,
            surface_label: "Target-context chip group pinned to a live cluster surface".to_owned(),
            truth_mode: TruthMode::Live,
            target_context_ref: "target_context:cluster:0001".to_owned(),
            adapter_source: M5AdapterSourceKind::ProviderOverlay,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            manifest_editor_header: None,
            schema_validator_row: None,
            target_context_chip_group: Some(TargetContextChipGroupDescriptor {
                truth_mode: TruthMode::Live,
                target_identity_ref: "target_identity:cluster:prod-eu".to_owned(),
                context_complete: true,
                stays_visible_on_scroll: true,
            }),
            resource_link_row: None,
            resource_explorer_row: None,
            adapter_source_badge: None,
            target_graph_row: None,
            capability_matrix: None,
            raw_event_drawer: None,
            fallback_confidence_drawer: None,
            degraded: None,
            label_summary: "A target-context chip group names the live target identity, environment, and scope and stays visible as the surface scrolls".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("target-context-chip-group:0001"),
        },
        // Resource-link row — rendered-to-live join at high confidence.
        ComponentRow {
            component_id: "component:resource-link-row:0001".to_owned(),
            family: M5ManifestBuildComponentFamily::ResourceLinkRow,
            surface_label: "Resource-link row joining a rendered resource to its live counterpart".to_owned(),
            truth_mode: TruthMode::Rendered,
            target_context_ref: "target_context:cluster:0001".to_owned(),
            adapter_source: M5AdapterSourceKind::NativeBuildServer,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            manifest_editor_header: None,
            schema_validator_row: None,
            target_context_chip_group: None,
            resource_link_row: Some(ResourceLinkRowDescriptor {
                link_class: M5ResourceLinkClass::RenderedToLive,
                from_truth: TruthMode::Rendered,
                to_truth: TruthMode::Live,
                confidence: M5DiscoveryConfidence::High,
                never_overwrites_higher_confidence: true,
            }),
            resource_explorer_row: None,
            adapter_source_badge: None,
            target_graph_row: None,
            capability_matrix: None,
            raw_event_drawer: None,
            fallback_confidence_drawer: None,
            degraded: None,
            label_summary: "A resource-link row joins rendered to live truth at high confidence and never overwrites a higher-confidence resource silently".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("resource-link-row:0001"),
        },
        // Resource-explorer row — cached-stale data disclosed, narrows.
        ComponentRow {
            component_id: "component:resource-explorer-row:0001".to_owned(),
            family: M5ManifestBuildComponentFamily::ResourceExplorerRow,
            surface_label: "Resource-explorer row showing cached data after a connector loss".to_owned(),
            truth_mode: TruthMode::Live,
            target_context_ref: "target_context:cluster:0001".to_owned(),
            adapter_source: M5AdapterSourceKind::ImportedSnapshot,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            manifest_editor_header: None,
            schema_validator_row: None,
            target_context_chip_group: None,
            resource_link_row: None,
            resource_explorer_row: Some(ResourceExplorerRowDescriptor {
                truth_mode: TruthMode::Live,
                freshness: M5ResourceFreshness::CachedStale,
                confidence: M5DiscoveryConfidence::Medium,
                target_context_visible: true,
            }),
            adapter_source_badge: None,
            target_graph_row: None,
            capability_matrix: None,
            raw_event_drawer: None,
            fallback_confidence_drawer: None,
            degraded: Some(DegradedState {
                trigger: M5ManifestBuildDowngradeTrigger::ConnectorLoss,
                degraded_label: "The live connector was lost; this row shows the last cached snapshot and is marked stale until the connector is restored".to_owned(),
            }),
            label_summary: "A resource-explorer row discloses cached-stale data after a connector loss rather than present it as fresh live truth".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("resource-explorer-row:0001"),
        },
        // Adapter-source badge — native build event at high confidence.
        ComponentRow {
            component_id: "component:adapter-source-badge:0001".to_owned(),
            family: M5ManifestBuildComponentFamily::AdapterSourceBadge,
            surface_label: "Adapter-source badge for a native build-event stream".to_owned(),
            truth_mode: TruthMode::Live,
            target_context_ref: "target_context:build:0001".to_owned(),
            adapter_source: M5AdapterSourceKind::NativeBuildEvent,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            manifest_editor_header: None,
            schema_validator_row: None,
            target_context_chip_group: None,
            resource_link_row: None,
            resource_explorer_row: None,
            adapter_source_badge: Some(AdapterSourceBadgeDescriptor {
                adapter_source: M5AdapterSourceKind::NativeBuildEvent,
                confidence: M5DiscoveryConfidence::High,
                source_kind_explicit: true,
            }),
            target_graph_row: None,
            capability_matrix: None,
            raw_event_drawer: None,
            fallback_confidence_drawer: None,
            degraded: None,
            label_summary: "An adapter-source badge names its native build-event provenance explicitly and claims high confidence only because the source is native".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("adapter-source-badge:0001"),
        },
        // Adapter-source badge — heuristic parse, low confidence, narrows.
        ComponentRow {
            component_id: "component:adapter-source-badge:0002".to_owned(),
            family: M5ManifestBuildComponentFamily::AdapterSourceBadge,
            surface_label: "Adapter-source badge for a heuristic parse of build output".to_owned(),
            truth_mode: TruthMode::Rendered,
            target_context_ref: "target_context:build:0002".to_owned(),
            adapter_source: M5AdapterSourceKind::HeuristicParse,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            manifest_editor_header: None,
            schema_validator_row: None,
            target_context_chip_group: None,
            resource_link_row: None,
            resource_explorer_row: None,
            adapter_source_badge: Some(AdapterSourceBadgeDescriptor {
                adapter_source: M5AdapterSourceKind::HeuristicParse,
                confidence: M5DiscoveryConfidence::Low,
                source_kind_explicit: true,
            }),
            target_graph_row: None,
            capability_matrix: None,
            raw_event_drawer: None,
            fallback_confidence_drawer: None,
            degraded: Some(DegradedState {
                trigger: M5ManifestBuildDowngradeTrigger::AdapterUnavailable,
                degraded_label: "No native build adapter answered; targets came from a heuristic parse and are marked low confidence".to_owned(),
            }),
            label_summary: "A heuristic-parse badge marks itself low confidence and never claims the authority of a native build channel".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("adapter-source-badge:0002"),
        },
        // Target-graph row — a build target node.
        ComponentRow {
            component_id: "component:target-graph-row:0001".to_owned(),
            family: M5ManifestBuildComponentFamily::TargetGraphRow,
            surface_label: "Target-graph row for a build target node".to_owned(),
            truth_mode: TruthMode::Rendered,
            target_context_ref: "target_context:build:0001".to_owned(),
            adapter_source: M5AdapterSourceKind::NativeBuildServer,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            manifest_editor_header: None,
            schema_validator_row: None,
            target_context_chip_group: None,
            resource_link_row: None,
            resource_explorer_row: None,
            adapter_source_badge: None,
            target_graph_row: Some(TargetGraphRowDescriptor {
                node_kind: M5TargetGraphNodeKind::BuildTarget,
                truth_mode: TruthMode::Rendered,
                edge_confidence: M5DiscoveryConfidence::High,
                target_identity_ref: "target_identity:build://app:server".to_owned(),
            }),
            capability_matrix: None,
            raw_event_drawer: None,
            fallback_confidence_drawer: None,
            degraded: None,
            label_summary: "A target-graph row names a build target node, its truth class, and its identity at high edge confidence".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("target-graph-row:0001"),
        },
        // Capability matrix — supported capability from a native source.
        ComponentRow {
            component_id: "component:capability-matrix:0001".to_owned(),
            family: M5ManifestBuildComponentFamily::CapabilityMatrix,
            surface_label: "Capability matrix cell for native test-with-coverage support".to_owned(),
            truth_mode: TruthMode::Live,
            target_context_ref: "target_context:build:0001".to_owned(),
            adapter_source: M5AdapterSourceKind::NativeBuildServer,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            manifest_editor_header: None,
            schema_validator_row: None,
            target_context_chip_group: None,
            resource_link_row: None,
            resource_explorer_row: None,
            adapter_source_badge: None,
            target_graph_row: None,
            capability_matrix: Some(CapabilityMatrixDescriptor {
                capability_state: M5CapabilityState::Supported,
                adapter_source: M5AdapterSourceKind::NativeBuildServer,
                discloses_source_and_confidence: true,
                confidence: M5DiscoveryConfidence::High,
            }),
            raw_event_drawer: None,
            fallback_confidence_drawer: None,
            degraded: None,
            label_summary: "A capability-matrix cell reports supported test-with-coverage capability, disclosing its native source and high confidence".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("capability-matrix:0001"),
        },
        // Capability matrix — unknown capability from a heuristic source, narrows.
        ComponentRow {
            component_id: "component:capability-matrix:0002".to_owned(),
            family: M5ManifestBuildComponentFamily::CapabilityMatrix,
            surface_label: "Capability matrix cell for an undetermined capability".to_owned(),
            truth_mode: TruthMode::Rendered,
            target_context_ref: "target_context:build:0002".to_owned(),
            adapter_source: M5AdapterSourceKind::HeuristicParse,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            manifest_editor_header: None,
            schema_validator_row: None,
            target_context_chip_group: None,
            resource_link_row: None,
            resource_explorer_row: None,
            adapter_source_badge: None,
            target_graph_row: None,
            capability_matrix: Some(CapabilityMatrixDescriptor {
                capability_state: M5CapabilityState::Unknown,
                adapter_source: M5AdapterSourceKind::HeuristicParse,
                discloses_source_and_confidence: true,
                confidence: M5DiscoveryConfidence::Unknown,
            }),
            raw_event_drawer: None,
            fallback_confidence_drawer: None,
            degraded: Some(DegradedState {
                trigger: M5ManifestBuildDowngradeTrigger::LowConfidenceDiscovery,
                degraded_label: "No native adapter reported this capability; it is shown as undetermined rather than assumed supported".to_owned(),
            }),
            label_summary: "A capability-matrix cell shows an undetermined capability from a heuristic source rather than assume support".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("capability-matrix:0002"),
        },
        // Raw-event drawer — native build-event stream, redacted.
        ComponentRow {
            component_id: "component:raw-event-drawer:0001".to_owned(),
            family: M5ManifestBuildComponentFamily::RawEventDrawer,
            surface_label: "Raw-event drawer over a native build-event stream".to_owned(),
            truth_mode: TruthMode::Live,
            target_context_ref: "target_context:build:0001".to_owned(),
            adapter_source: M5AdapterSourceKind::NativeBuildEvent,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            manifest_editor_header: None,
            schema_validator_row: None,
            target_context_chip_group: None,
            resource_link_row: None,
            resource_explorer_row: None,
            adapter_source_badge: None,
            target_graph_row: None,
            capability_matrix: None,
            raw_event_drawer: Some(RawEventDrawerDescriptor {
                event_channel: M5RawEventChannel::NativeBuildEvent,
                redaction_applied: true,
                preserves_event_identity: true,
            }),
            fallback_confidence_drawer: None,
            degraded: None,
            label_summary: "A raw-event drawer discloses native build-event provenance, redacts raw payloads to typed tokens, and preserves stable event identity on export".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("raw-event-drawer:0001"),
        },
        // Fallback-confidence drawer — structured channel lost, heuristic fallback.
        ComponentRow {
            component_id: "component:fallback-confidence-drawer:0001".to_owned(),
            family: M5ManifestBuildComponentFamily::FallbackConfidenceDrawer,
            surface_label: "Fallback-confidence drawer after a structured build channel was lost".to_owned(),
            truth_mode: TruthMode::Rendered,
            target_context_ref: "target_context:build:0002".to_owned(),
            adapter_source: M5AdapterSourceKind::HeuristicParse,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            manifest_editor_header: None,
            schema_validator_row: None,
            target_context_chip_group: None,
            resource_link_row: None,
            resource_explorer_row: None,
            adapter_source_badge: None,
            target_graph_row: None,
            capability_matrix: None,
            raw_event_drawer: None,
            fallback_confidence_drawer: Some(FallbackConfidenceDrawerDescriptor {
                confidence_state: M5FallbackConfidenceState::HeuristicFallback,
                fallback_reason: Some(M5FallbackReason::StructuredChannelLost),
                recovery_route: M5FallbackRecoveryRoute::ReattachAdapter,
                never_overwrites_structured_silently: true,
            }),
            degraded: Some(DegradedState {
                trigger: M5ManifestBuildDowngradeTrigger::StructuredChannelLost,
                degraded_label: "The structured build channel dropped; results are now a heuristic fallback and never overwrite the last structured truth silently".to_owned(),
            }),
            label_summary: "A fallback-confidence drawer names the structured-channel loss, marks results a heuristic fallback, and offers a reattach-adapter recovery route".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("fallback-confidence-drawer:0001"),
        },
        // Fallback-confidence drawer — structured high confidence, no fallback.
        ComponentRow {
            component_id: "component:fallback-confidence-drawer:0002".to_owned(),
            family: M5ManifestBuildComponentFamily::FallbackConfidenceDrawer,
            surface_label: "Fallback-confidence drawer confirming structured high-confidence truth".to_owned(),
            truth_mode: TruthMode::Live,
            target_context_ref: "target_context:build:0001".to_owned(),
            adapter_source: M5AdapterSourceKind::NativeBuildEvent,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            manifest_editor_header: None,
            schema_validator_row: None,
            target_context_chip_group: None,
            resource_link_row: None,
            resource_explorer_row: None,
            adapter_source_badge: None,
            target_graph_row: None,
            capability_matrix: None,
            raw_event_drawer: None,
            fallback_confidence_drawer: Some(FallbackConfidenceDrawerDescriptor {
                confidence_state: M5FallbackConfidenceState::StructuredHigh,
                fallback_reason: None,
                recovery_route: M5FallbackRecoveryRoute::InspectOnly,
                never_overwrites_structured_silently: true,
            }),
            degraded: None,
            label_summary: "A fallback-confidence drawer confirms structured high-confidence native truth and carries no fallback reason".to_owned(),
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("fallback-confidence-drawer:0002"),
        },
    ]
}

fn seeded_guardrails() -> ManifestBuildGuardrails {
    ManifestBuildGuardrails {
        truth_classes_never_blur: true,
        target_context_visible_on_every_surface: true,
        schema_freshness_and_adapter_source_explicit: true,
        lower_confidence_never_overwrites_silently: true,
        drift_connector_loss_policy_narrow_before_execution: true,
        exported_evidence_preserves_ids_kinds_and_states: true,
        components_bound_to_shared_vocabulary: true,
        no_new_adapters_connectors_or_engines: true,
    }
}

fn seeded_consumer_projection() -> ManifestBuildConsumerProjection {
    ManifestBuildConsumerProjection {
        product_ingests_components: true,
        docs_help_ingests_components: true,
        diagnostics_ingests_components: true,
        support_export_ingests_components: true,
        release_control_ingests_components: true,
        later_rows_reference_one_canonical_family: true,
    }
}

fn seeded_source_contract_refs() -> Vec<String> {
    vec![
        MANIFEST_BUILD_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        MANIFEST_BUILD_COMPONENT_MATRIX_DOC_REF.to_owned(),
        MANIFEST_BUILD_COMPONENT_MATRIX_ARTIFACT_REF.to_owned(),
        "schemas/infra/infrastructure-surface-qualification.schema.json".to_owned(),
        "schemas/infra/cluster-context-and-live-resource.schema.json".to_owned(),
    ]
}
