//! Implements the reusable build / run confidence primitive: an adapter-source
//! badge (with a confidence chip), a target-graph row, a capability-matrix
//! sheet, a raw-event drawer, and a fallback-confidence drawer that all resolve
//! from one build-target context and share one target identity and one disclosed
//! adapter source, so build and run confidence stays *inspectable before* any
//! target is trusted or executed.
//!
//! Where
//! [`crate::freeze_the_m5_manifest_editor_schema_validator_resource_link_build_adapter_target_graph_and_fallback_confidence_component_matrix`]
//! *freezes* the reusable manifest / build-confidence component families as a
//! governed contract, this module *narrows* the five remaining build / run
//! families of that matrix —
//! [`crate::M5ManifestBuildComponentFamily::AdapterSourceBadge`],
//! [`crate::M5ManifestBuildComponentFamily::TargetGraphRow`],
//! [`crate::M5ManifestBuildComponentFamily::CapabilityMatrix`],
//! [`crate::M5ManifestBuildComponentFamily::RawEventDrawer`], and
//! [`crate::M5ManifestBuildComponentFamily::FallbackConfidenceDrawer`] — into one
//! working primitive with a real **resolver**. A single build-target context
//! projects onto five surfaces that share one target identity and one disclosed
//! adapter source, so native / protocol-backed truth and heuristic / imported
//! fallback truth never blur across the badge, the target-graph row, the
//! capability matrix, the raw-event drawer, and the fallback-confidence drawer.
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — adapter provenance is never hidden.** The badge always renders its
//!   adapter source kind explicitly and keeps its confidence chip consistent with
//!   that source, so a heuristic parse or imported snapshot can never claim native
//!   authority, and a native lane can never masquerade as a fallback. The
//!   fallback-confidence drawer discloses the structured-versus-heuristic state in
//!   lockstep with the badge.
//! - **AC2 — target identity and confidence stay inspectable before action.** The
//!   target-graph row preserves the stable target id, owning module / root,
//!   freshness, supported verbs, and required environment, and the capability
//!   matrix explains supported verbs and downgraded actions — all before any
//!   run / test / debug action is offered.
//! - **AC3 — support and AI consumers reuse the same component truth.** The
//!   raw-event drawer redacts payloads to typed tokens, preserves stable event
//!   identity and payload lineage, names the adapter version, and offers
//!   export / copy actions, so support and AI surfaces reconstruct the same target
//!   ids, adapter kinds, and freshness / confidence states shown in-product
//!   instead of re-deriving them from logs.
//!
//! Raw build output, event payloads, credentials, and endpoint data never cross
//! this boundary; the resolver carries only opaque refs, typed class tokens,
//! booleans, and redacted labels, so support and diagnostics exports reconstruct
//! exactly what a surface would have shown without leaking build or event
//! payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-build-confidence-primitive.schema.json`](../../../../schemas/ui/m5-build-confidence-primitive.schema.json).
//! The contract doc is
//! [`docs/infra/m5_build_confidence_primitive.md`](../../../../docs/infra/m5_build_confidence_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    truth_mode_token, DegradedState, M5AdapterSourceKind, M5CapabilityState, M5DiscoveryConfidence,
    M5FallbackConfidenceState, M5FallbackReason, M5FallbackRecoveryRoute,
    M5ManifestBuildDowngradeTrigger, M5RawEventChannel, M5ResourceFreshness, M5TargetGraphNodeKind,
    TruthMode,
};

/// Stable record-kind tag carried by [`M5BuildConfidencePrimitivePacket`].
pub const M5_BUILD_CONFIDENCE_RECORD_KIND: &str = "m5_build_confidence_primitive";

/// Schema version for the build / run confidence primitive packet.
pub const M5_BUILD_CONFIDENCE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_BUILD_CONFIDENCE_SCHEMA_REF: &str =
    "schemas/ui/m5-build-confidence-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_BUILD_CONFIDENCE_DOC_REF: &str = "docs/infra/m5_build_confidence_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive
/// narrows.
pub const M5_BUILD_CONFIDENCE_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-manifest-build-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_BUILD_CONFIDENCE_FIXTURE_DIR: &str = "fixtures/ui/m5-build-confidence-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const M5_BUILD_CONFIDENCE_ARTIFACT_REF: &str =
    "artifacts/release/m5-build-confidence-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_BUILD_CONFIDENCE_CSV_REF: &str =
    "artifacts/release/m5-build-confidence-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_BUILD_CONFIDENCE_REPORT_REF: &str =
    "artifacts/release/m5-build-confidence-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed build / run confidence surface family. Each family is one parity
/// surface that ingests the shared primitive; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildConfidenceSurfaceFamily {
    /// The adapter-source badge and confidence chip.
    AdapterSourceBadge,
    /// The target-graph row preserving target identity, freshness, and verbs.
    TargetGraphRow,
    /// The capability-matrix sheet explaining supported and downgraded verbs.
    CapabilityMatrixSheet,
    /// The raw-event drawer disclosing payload lineage and adapter version.
    RawEventDrawer,
    /// The fallback-confidence drawer disclosing structured-versus-heuristic truth.
    FallbackConfidenceDrawer,
    /// The support / export replay surface that reconstructs confidence truth.
    SupportExportReplay,
}

impl M5BuildConfidenceSurfaceFamily {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AdapterSourceBadge,
        Self::TargetGraphRow,
        Self::CapabilityMatrixSheet,
        Self::RawEventDrawer,
        Self::FallbackConfidenceDrawer,
        Self::SupportExportReplay,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdapterSourceBadge => "adapter_source_badge",
            Self::TargetGraphRow => "target_graph_row",
            Self::CapabilityMatrixSheet => "capability_matrix_sheet",
            Self::RawEventDrawer => "raw_event_drawer",
            Self::FallbackConfidenceDrawer => "fallback_confidence_drawer",
            Self::SupportExportReplay => "support_export_replay",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AdapterSourceBadge => "Adapter-source badge",
            Self::TargetGraphRow => "Target-graph row",
            Self::CapabilityMatrixSheet => "Capability-matrix sheet",
            Self::RawEventDrawer => "Raw-event drawer",
            Self::FallbackConfidenceDrawer => "Fallback-confidence drawer",
            Self::SupportExportReplay => "Support / export replay",
        }
    }
}

/// Closed build-verb vocabulary. Names which run / test / debug / build actions a
/// target claims so a supported verb and a downgraded verb never read as one
/// another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildVerb {
    /// Build the target.
    Build,
    /// Run the target's tests.
    Test,
    /// Run / launch the target.
    Run,
    /// Debug the target.
    Debug,
    /// Collect coverage for the target.
    Coverage,
    /// Package the target.
    Package,
}

impl M5BuildVerb {
    /// Every build verb, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Build,
        Self::Test,
        Self::Run,
        Self::Debug,
        Self::Coverage,
        Self::Package,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Build => "build",
            Self::Test => "test",
            Self::Run => "run",
            Self::Debug => "debug",
            Self::Coverage => "coverage",
            Self::Package => "package",
        }
    }
}

/// Closed build-action vocabulary. Names the safe, read-only actions a confidence
/// surface offers so inspection and export stay available before any run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildActionKind {
    /// Inspect the capability matrix.
    InspectCapabilities,
    /// View the raw-event drawer.
    ViewRawEvents,
    /// Copy / export the confidence packet.
    CopyExport,
    /// Open the target-graph view.
    OpenTargetGraph,
    /// Open the canonical source truth.
    OpenSourceTruth,
}

impl M5BuildActionKind {
    /// Every action kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InspectCapabilities,
        Self::ViewRawEvents,
        Self::CopyExport,
        Self::OpenTargetGraph,
        Self::OpenSourceTruth,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectCapabilities => "inspect_capabilities",
            Self::ViewRawEvents => "view_raw_events",
            Self::CopyExport => "copy_export",
            Self::OpenTargetGraph => "open_target_graph",
            Self::OpenSourceTruth => "open_source_truth",
        }
    }

    /// True when this action exports or copies confidence truth for reuse.
    pub const fn is_export(self) -> bool {
        matches!(self, Self::CopyExport)
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet
/// must carry per surface; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildConfidenceExportField {
    /// The stable target identity shared across surfaces.
    TargetId,
    /// The typed target identity (node kind, stable id, owning module, root).
    TargetIdentity,
    /// The adapter source kind the truth came from.
    AdapterSource,
    /// The adapter version the raw-event drawer names.
    AdapterVersion,
    /// The discovery confidence.
    Confidence,
    /// The target / result freshness.
    Freshness,
    /// The structured-versus-fallback confidence state.
    FallbackState,
}

impl M5BuildConfidenceExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::TargetId,
        Self::TargetIdentity,
        Self::AdapterSource,
        Self::AdapterVersion,
        Self::Confidence,
        Self::Freshness,
        Self::FallbackState,
    ];

    /// The mandatory subset every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::TargetId,
        Self::TargetIdentity,
        Self::AdapterSource,
        Self::Confidence,
        Self::Freshness,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetId => "target_id",
            Self::TargetIdentity => "target_identity",
            Self::AdapterSource => "adapter_source",
            Self::AdapterVersion => "adapter_version",
            Self::Confidence => "confidence",
            Self::Freshness => "freshness",
            Self::FallbackState => "fallback_state",
        }
    }
}

// --- shared value structs ---

/// The typed, stable identity of a build / run target. Every slot is opaque; raw
/// paths and endpoint data never cross this boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TargetIdentity {
    /// The kind of target-graph node.
    pub node_kind: M5TargetGraphNodeKind,
    /// The stable target id (opaque; never a raw path or label).
    pub stable_id: String,
    /// The owning module the target belongs to.
    pub owning_module: String,
    /// The workspace root the module is rooted at.
    pub workspace_root: String,
}

impl M5TargetIdentity {
    /// True when the identity carries a stable id, an owning module, and a root.
    pub fn is_stable(&self) -> bool {
        !self.stable_id.trim().is_empty()
            && !self.owning_module.trim().is_empty()
            && !self.workspace_root.trim().is_empty()
    }
}

/// One requested capability cell: a build verb bound to a support state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilityCell {
    /// The build verb this cell describes.
    pub verb: M5BuildVerb,
    /// Whether the verb is supported, partial, unsupported, unknown, or gated.
    pub state: M5CapabilityState,
}

/// One resolved capability cell: a verb, its state, and whether it is downgraded
/// below full support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCapabilityCell {
    /// The build verb this cell describes.
    pub verb: M5BuildVerb,
    /// The support state disclosed for the verb.
    pub state: M5CapabilityState,
    /// True when the verb is anything less than fully supported.
    pub downgraded: bool,
}

// --- resolver input ---

/// The full input to the build / run confidence resolver for one target context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildConfidenceInput {
    /// The stable target identity that must survive across the badge, target-graph
    /// row, capability matrix, raw-event drawer, and fallback drawer.
    pub target_id: String,
    /// Opaque ref to the target object; never raw build bytes.
    pub target_ref: String,
    /// Human-readable target label.
    pub target_label: String,
    /// The typed target identity (node kind, stable id, owning module, root).
    pub identity: M5TargetIdentity,
    /// The truth class the target is shown in.
    pub truth_mode: TruthMode,
    /// Where the build / run truth came from.
    pub adapter_source: M5AdapterSourceKind,
    /// The adapter version the raw-event drawer names; opaque token.
    pub adapter_version: String,
    /// The confidence of the discovered target / capability truth.
    pub confidence: M5DiscoveryConfidence,
    /// The freshness of the target / result data.
    pub freshness: M5ResourceFreshness,
    /// The required environment keys the target-graph row preserves; opaque
    /// labels, never raw values.
    pub required_environment: Vec<String>,
    /// The provenance channel of the raw events.
    pub event_channel: M5RawEventChannel,
    /// The payload-lineage chain of channels the raw events flowed through (must be
    /// non-empty).
    pub payload_lineage: Vec<M5RawEventChannel>,
    /// The requested capability cells (must be non-empty).
    pub capabilities: Vec<M5CapabilityCell>,
    /// The structured-versus-heuristic confidence state.
    pub fallback_state: M5FallbackConfidenceState,
    /// Why confidence fell; required when the state is a fallback.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_reason: Option<M5FallbackReason>,
    /// The recovery route the fallback drawer offers.
    pub recovery_route: M5FallbackRecoveryRoute,
    /// A precise fallback note, required when the state is a fallback.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_note: Option<String>,
    /// Opaque ref to the target identity the surface acts against; never raw path
    /// or endpoint data.
    pub target_identity_ref: String,
    /// The safe, read-only actions offered (inspect capabilities / view raw events
    /// / copy-export / open target graph / open source truth).
    pub available_actions: Vec<M5BuildActionKind>,
    /// An externally-observed narrowing (adapter loss, channel loss, policy block)
    /// that degrades the surface before action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

// --- resolved projections ---

/// The resolved adapter-source badge and confidence chip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAdapterSourceBadge {
    /// The target identity — identical to every other surface.
    pub target_id: String,
    /// Where the build / run truth came from.
    pub adapter_source: M5AdapterSourceKind,
    /// The confidence the chip discloses.
    pub confidence: M5DiscoveryConfidence,
    /// The adapter source kind is rendered explicitly; always holds.
    pub source_kind_explicit: bool,
    /// True when the source is a native, authoritative build channel.
    pub is_native: bool,
    /// The confidence chip is consistent with the source kind; always holds.
    pub confidence_consistent: bool,
}

/// The resolved target-graph row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTargetGraphRow {
    /// The target identity — identical to every other surface.
    pub target_id: String,
    /// The typed target identity (node kind, stable id, owning module, root).
    pub identity: M5TargetIdentity,
    /// The kind of target-graph node.
    pub node_kind: M5TargetGraphNodeKind,
    /// The truth class the node is shown in.
    pub truth_mode: TruthMode,
    /// The confidence of the discovered node / edge.
    pub edge_confidence: M5DiscoveryConfidence,
    /// The freshness of the target data.
    pub freshness: M5ResourceFreshness,
    /// The verbs this target supports (fully or partially).
    pub supported_verbs: Vec<M5BuildVerb>,
    /// The required environment keys the row preserves.
    pub required_environment: Vec<String>,
    /// Target context is always visible on the target-graph row; always holds.
    pub target_context_visible: bool,
}

/// The resolved capability-matrix sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCapabilityMatrix {
    /// The target identity — identical to every other surface.
    pub target_id: String,
    /// The adapter source the capability truth is derived from.
    pub adapter_source: M5AdapterSourceKind,
    /// The confidence of the capability determination.
    pub confidence: M5DiscoveryConfidence,
    /// The resolved capability cells, one per requested verb.
    pub cells: Vec<M5ResolvedCapabilityCell>,
    /// The verbs downgraded below full support.
    pub downgraded_verbs: Vec<M5BuildVerb>,
    /// The sheet discloses its adapter source and confidence; always holds.
    pub discloses_source_and_confidence: bool,
}

/// The resolved raw-event drawer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRawEventDrawer {
    /// The target identity — identical to every other surface.
    pub target_id: String,
    /// The provenance channel of the raw events.
    pub event_channel: M5RawEventChannel,
    /// The adapter version the drawer names.
    pub adapter_version: String,
    /// The payload-lineage chain of channels the events flowed through.
    pub payload_lineage: Vec<M5RawEventChannel>,
    /// Raw payloads are redacted to typed tokens before export; always holds.
    pub redaction_applied: bool,
    /// The drawer preserves stable event identity across export; always holds.
    pub preserves_event_identity: bool,
    /// The export / copy actions offered on the drawer.
    pub export_actions: Vec<M5BuildActionKind>,
}

/// The resolved fallback-confidence drawer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedFallbackConfidenceDrawer {
    /// The target identity — identical to every other surface.
    pub target_id: String,
    /// The structured-versus-heuristic confidence state.
    pub confidence_state: M5FallbackConfidenceState,
    /// Why confidence fell, when the state is a fallback.
    pub fallback_reason: Option<M5FallbackReason>,
    /// The recovery route offered.
    pub recovery_route: M5FallbackRecoveryRoute,
    /// True when the state is a fallback below structured confidence.
    pub is_fallback: bool,
    /// The precise fallback note, when the state is a fallback.
    pub fallback_note: Option<String>,
    /// Lower-confidence fallback never overwrites structured truth silently; always
    /// holds.
    pub never_overwrites_structured_silently: bool,
    /// Why the surface is narrowed, when it is; names a real, reconstructable
    /// trigger.
    pub downgrade_trigger: Option<M5ManifestBuildDowngradeTrigger>,
}

/// The resolved build / run confidence truth shared across the badge,
/// target-graph row, capability matrix, raw-event drawer, and fallback drawer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBuildConfidence {
    /// The stable target identity.
    pub target_id: String,
    /// The resolved adapter-source badge and confidence chip.
    pub adapter_badge: M5ResolvedAdapterSourceBadge,
    /// The resolved target-graph row.
    pub target_graph_row: M5ResolvedTargetGraphRow,
    /// The resolved capability-matrix sheet.
    pub capability_matrix: M5ResolvedCapabilityMatrix,
    /// The resolved raw-event drawer.
    pub raw_event_drawer: M5ResolvedRawEventDrawer,
    /// The resolved fallback-confidence drawer.
    pub fallback_drawer: M5ResolvedFallbackConfidenceDrawer,
    /// Adapter provenance (native versus fallback) is disclosed, never hidden
    /// (AC1); always holds.
    pub provenance_disclosed: bool,
    /// Target identity and confidence are inspectable before action (AC2); always
    /// holds.
    pub identity_inspectable_before_action: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedBuildConfidence {
    /// True when the target identity is identical across the badge, target-graph
    /// row, capability matrix, raw-event drawer, and fallback drawer.
    pub fn identity_consistent(&self) -> bool {
        self.adapter_badge.target_id == self.target_id
            && self.target_graph_row.target_id == self.target_id
            && self.capability_matrix.target_id == self.target_id
            && self.raw_event_drawer.target_id == self.target_id
            && self.fallback_drawer.target_id == self.target_id
    }

    /// True when the badge and the capability matrix disclose the same adapter
    /// source, and the fallback drawer's structured-versus-fallback state agrees
    /// with the badge's native-ness — native / protocol-backed truth and heuristic
    /// fallback truth never blur across the surfaces (AC1).
    pub fn provenance_disclosed_consistently(&self) -> bool {
        self.adapter_badge.adapter_source == self.capability_matrix.adapter_source
            && self.adapter_badge.source_kind_explicit
            && self.adapter_badge.confidence_consistent
            && adapter_and_fallback_consistent(
                self.adapter_badge.adapter_source,
                self.fallback_drawer.confidence_state,
            )
    }

    /// True when the target identity, freshness, supported verbs, and capability
    /// truth are inspectable before any run / test / debug action starts (AC2).
    pub fn identity_and_confidence_inspectable(&self) -> bool {
        self.target_graph_row.target_context_visible
            && self.target_graph_row.identity.is_stable()
            && self.capability_matrix.discloses_source_and_confidence
            && !self.capability_matrix.cells.is_empty()
    }

    /// True when the raw-event drawer reconstructs redacted, identity-preserving,
    /// exportable truth so support and AI consumers reuse it rather than
    /// re-deriving it from logs (AC3).
    pub fn support_reuse_ready(&self) -> bool {
        self.raw_event_drawer.redaction_applied
            && self.raw_event_drawer.preserves_event_identity
            && !self.raw_event_drawer.payload_lineage.is_empty()
            && self.raw_event_drawer.export_actions.iter().any(|a| a.is_export())
    }
}

/// Errors returned by [`resolve_build_confidence`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5BuildConfidenceResolutionError {
    /// The target id was empty.
    EmptyTargetId,
    /// The target ref was empty.
    EmptyTargetRef,
    /// The target label was empty.
    EmptyTargetLabel,
    /// The target identity carried no stable id, owning module, or root.
    EmptyTargetIdentity,
    /// The target-identity ref was empty.
    EmptyTargetIdentityRef,
    /// The adapter version was empty.
    EmptyAdapterVersion,
    /// A label, ref, note, or env key carried forbidden material.
    ForbiddenMaterial,
    /// The confidence chip was inconsistent with the adapter source (a fallback /
    /// imported / unknown source claimed high confidence).
    AdapterConfidenceInconsistent,
    /// The structured-versus-fallback state blurred with the adapter source (a
    /// native lane claimed a fallback state, or a fallback lane claimed structured
    /// high).
    AdapterFallbackMismatch,
    /// The state was a fallback but no reason was given.
    FallbackWithoutReason,
    /// The state was structured but a fallback reason was given.
    StructuredWithFallbackReason,
    /// The state was a fallback but no precise note was given.
    FallbackWithoutNote,
    /// No capability cells were declared.
    NoCapabilitiesDeclared,
    /// A supported capability was claimed from an unknown-confidence source.
    SupportedCapabilityUnknownConfidence,
    /// The payload lineage was empty.
    EmptyPayloadLineage,
    /// No safe, read-only action was offered on the surface.
    NoActionsOffered,
    /// No export / copy action was offered for support / AI reuse.
    NoExportActionOffered,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5BuildConfidenceResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyTargetId => "empty_target_id",
            Self::EmptyTargetRef => "empty_target_ref",
            Self::EmptyTargetLabel => "empty_target_label",
            Self::EmptyTargetIdentity => "empty_target_identity",
            Self::EmptyTargetIdentityRef => "empty_target_identity_ref",
            Self::EmptyAdapterVersion => "empty_adapter_version",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::AdapterConfidenceInconsistent => "adapter_confidence_inconsistent",
            Self::AdapterFallbackMismatch => "adapter_fallback_mismatch",
            Self::FallbackWithoutReason => "fallback_without_reason",
            Self::StructuredWithFallbackReason => "structured_with_fallback_reason",
            Self::FallbackWithoutNote => "fallback_without_note",
            Self::NoCapabilitiesDeclared => "no_capabilities_declared",
            Self::SupportedCapabilityUnknownConfidence => {
                "supported_capability_unknown_confidence"
            }
            Self::EmptyPayloadLineage => "empty_payload_lineage",
            Self::NoActionsOffered => "no_actions_offered",
            Self::NoExportActionOffered => "no_export_action_offered",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5BuildConfidenceResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "build-confidence resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5BuildConfidenceResolutionError {}

/// Resolves one build / run confidence context into its shared adapter-source
/// badge, target-graph row, capability-matrix sheet, raw-event drawer, and
/// fallback-confidence drawer.
///
/// The five surfaces share one target identity and one disclosed adapter source,
/// so native / protocol-backed truth and heuristic / imported fallback truth
/// never blur. Adapter provenance is always disclosed (the badge renders its
/// source kind explicitly and keeps its confidence chip consistent); the
/// target-graph row and capability matrix keep target identity and confidence
/// inspectable before any run / test / debug action; and the raw-event drawer
/// reconstructs redacted, identity-preserving, exportable truth for support and
/// AI reuse.
pub fn resolve_build_confidence(
    input: &M5BuildConfidenceInput,
) -> Result<M5ResolvedBuildConfidence, M5BuildConfidenceResolutionError> {
    if input.target_id.trim().is_empty() {
        return Err(M5BuildConfidenceResolutionError::EmptyTargetId);
    }
    if input.target_ref.trim().is_empty() {
        return Err(M5BuildConfidenceResolutionError::EmptyTargetRef);
    }
    if input.target_label.trim().is_empty() {
        return Err(M5BuildConfidenceResolutionError::EmptyTargetLabel);
    }
    if !input.identity.is_stable() {
        return Err(M5BuildConfidenceResolutionError::EmptyTargetIdentity);
    }
    if input.target_identity_ref.trim().is_empty() {
        return Err(M5BuildConfidenceResolutionError::EmptyTargetIdentityRef);
    }
    if input.adapter_version.trim().is_empty() {
        return Err(M5BuildConfidenceResolutionError::EmptyAdapterVersion);
    }

    for value in [
        input.target_ref.as_str(),
        input.target_label.as_str(),
        input.identity.stable_id.as_str(),
        input.identity.owning_module.as_str(),
        input.identity.workspace_root.as_str(),
        input.target_identity_ref.as_str(),
        input.adapter_version.as_str(),
    ]
    .into_iter()
    .chain(input.required_environment.iter().map(String::as_str))
    .chain(input.fallback_note.as_deref())
    {
        if value_is_forbidden(value) {
            return Err(M5BuildConfidenceResolutionError::ForbiddenMaterial);
        }
    }

    // AC1: a heuristic / imported / unknown source can never claim high
    // confidence; a native source always may.
    if !input.adapter_source.confidence_consistent(input.confidence) {
        return Err(M5BuildConfidenceResolutionError::AdapterConfidenceInconsistent);
    }

    // AC1: the structured-versus-fallback state may never blur with the adapter
    // source — a native lane is structured, a fallback lane never claims
    // structured-high authority.
    if !adapter_and_fallback_consistent(input.adapter_source, input.fallback_state) {
        return Err(M5BuildConfidenceResolutionError::AdapterFallbackMismatch);
    }

    let is_fallback = input.fallback_state.is_fallback();

    // A fallback names why it fell and offers a precise note; a structured state
    // carries neither.
    if is_fallback {
        if input.fallback_reason.is_none() {
            return Err(M5BuildConfidenceResolutionError::FallbackWithoutReason);
        }
        if input.fallback_note.is_none() {
            return Err(M5BuildConfidenceResolutionError::FallbackWithoutNote);
        }
    } else if input.fallback_reason.is_some() {
        return Err(M5BuildConfidenceResolutionError::StructuredWithFallbackReason);
    }

    if input.capabilities.is_empty() {
        return Err(M5BuildConfidenceResolutionError::NoCapabilitiesDeclared);
    }

    // AC2: a supported verb may never be claimed from an unknown-confidence source.
    if input.confidence == M5DiscoveryConfidence::Unknown
        && input
            .capabilities
            .iter()
            .any(|cell| cell.state == M5CapabilityState::Supported)
    {
        return Err(M5BuildConfidenceResolutionError::SupportedCapabilityUnknownConfidence);
    }

    if input.payload_lineage.is_empty() {
        return Err(M5BuildConfidenceResolutionError::EmptyPayloadLineage);
    }

    if input.available_actions.is_empty() {
        return Err(M5BuildConfidenceResolutionError::NoActionsOffered);
    }

    // AC3: an export / copy action must be offered so support and AI consumers can
    // reuse the component truth.
    if !input.available_actions.iter().any(|a| a.is_export()) {
        return Err(M5BuildConfidenceResolutionError::NoExportActionOffered);
    }

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5BuildConfidenceResolutionError::DegradedLabelGeneric);
        }
    }

    let cells: Vec<M5ResolvedCapabilityCell> = input
        .capabilities
        .iter()
        .map(|cell| M5ResolvedCapabilityCell {
            verb: cell.verb,
            state: cell.state,
            downgraded: cell.state != M5CapabilityState::Supported,
        })
        .collect();

    // Verbs a target offers: fully or partially supported.
    let supported_verbs: Vec<M5BuildVerb> = input
        .capabilities
        .iter()
        .filter(|cell| {
            matches!(
                cell.state,
                M5CapabilityState::Supported | M5CapabilityState::Partial
            )
        })
        .map(|cell| cell.verb)
        .collect();

    // Verbs downgraded below full support.
    let downgraded_verbs: Vec<M5BuildVerb> = cells
        .iter()
        .filter(|cell| cell.downgraded)
        .map(|cell| cell.verb)
        .collect();

    let export_actions: Vec<M5BuildActionKind> = input
        .available_actions
        .iter()
        .copied()
        .filter(|a| a.is_export())
        .collect();

    let downgrade_trigger = if let Some(degraded) = &input.degraded {
        Some(degraded.trigger)
    } else {
        input.fallback_reason.map(fallback_reason_to_trigger)
    };

    let adapter_badge = M5ResolvedAdapterSourceBadge {
        target_id: input.target_id.clone(),
        adapter_source: input.adapter_source,
        confidence: input.confidence,
        source_kind_explicit: true,
        is_native: input.adapter_source.is_native(),
        confidence_consistent: true,
    };

    let target_graph_row = M5ResolvedTargetGraphRow {
        target_id: input.target_id.clone(),
        identity: input.identity.clone(),
        node_kind: input.identity.node_kind,
        truth_mode: input.truth_mode,
        edge_confidence: input.confidence,
        freshness: input.freshness,
        supported_verbs,
        required_environment: input.required_environment.clone(),
        target_context_visible: true,
    };

    let capability_matrix = M5ResolvedCapabilityMatrix {
        target_id: input.target_id.clone(),
        adapter_source: input.adapter_source,
        confidence: input.confidence,
        cells,
        downgraded_verbs,
        discloses_source_and_confidence: true,
    };

    let raw_event_drawer = M5ResolvedRawEventDrawer {
        target_id: input.target_id.clone(),
        event_channel: input.event_channel,
        adapter_version: input.adapter_version.clone(),
        payload_lineage: input.payload_lineage.clone(),
        redaction_applied: true,
        preserves_event_identity: true,
        export_actions,
    };

    let fallback_drawer = M5ResolvedFallbackConfidenceDrawer {
        target_id: input.target_id.clone(),
        confidence_state: input.fallback_state,
        fallback_reason: input.fallback_reason,
        recovery_route: input.recovery_route,
        is_fallback,
        fallback_note: input.fallback_note.clone(),
        never_overwrites_structured_silently: true,
        downgrade_trigger,
    };

    Ok(M5ResolvedBuildConfidence {
        target_id: input.target_id.clone(),
        adapter_badge,
        target_graph_row,
        capability_matrix,
        raw_event_drawer,
        fallback_drawer,
        provenance_disclosed: true,
        identity_inspectable_before_action: true,
        degraded: input.degraded.clone(),
    })
}

/// True when the structured-versus-fallback state is consistent with the adapter
/// source: a native lane is structured (never a fallback), and any non-native
/// lane never claims the top structured-high confidence.
const fn adapter_and_fallback_consistent(
    adapter: M5AdapterSourceKind,
    state: M5FallbackConfidenceState,
) -> bool {
    if adapter.is_native() {
        !state.is_fallback()
    } else {
        !matches!(state, M5FallbackConfidenceState::StructuredHigh)
    }
}

/// Maps a fallback reason to the reconstructable downgrade trigger it implies.
const fn fallback_reason_to_trigger(reason: M5FallbackReason) -> M5ManifestBuildDowngradeTrigger {
    match reason {
        M5FallbackReason::AdapterUnavailable => M5ManifestBuildDowngradeTrigger::AdapterUnavailable,
        M5FallbackReason::StructuredChannelLost => {
            M5ManifestBuildDowngradeTrigger::StructuredChannelLost
        }
        M5FallbackReason::SchemaDrift => M5ManifestBuildDowngradeTrigger::DriftFromSource,
        M5FallbackReason::ConnectorLoss => M5ManifestBuildDowngradeTrigger::ConnectorLoss,
        M5FallbackReason::PolicyBlock => M5ManifestBuildDowngradeTrigger::PolicyBlock,
    }
}

/// True when a label, ref, or note carries obviously forbidden material.
fn value_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// One worked resolution case carried in the packet so the support / export
/// packet reconstructs confidence truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildConfidenceCase {
    /// The resolver input.
    pub input: M5BuildConfidenceInput,
    /// The resolved confidence truth. Must equal
    /// `resolve_build_confidence(&input)`.
    pub resolved: M5ResolvedBuildConfidence,
}

impl M5BuildConfidenceCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5BuildConfidenceInput) -> Self {
        let resolved = resolve_build_confidence(&input).expect("seed confidence case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_build_confidence(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one confidence surface family bound to the
/// shared build-target contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildConfidenceSurfaceRow {
    /// The confidence surface family.
    pub surface_family: M5BuildConfidenceSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Adapter source kinds this surface can disclose (must be non-empty).
    pub adapter_source_kinds: Vec<M5AdapterSourceKind>,
    /// Truth classes this surface renders (must be non-empty).
    pub truth_modes: Vec<TruthMode>,
    /// Build verbs this surface reasons about (must be non-empty).
    pub build_verbs: Vec<M5BuildVerb>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5BuildConfidenceExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5ManifestBuildDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be
    /// non-empty).
    pub example_confidence: Vec<M5BuildConfidenceCase>,
    /// Hard invariant: this row never hides the adapter source. MUST be `false`.
    pub hides_adapter_source: bool,
    /// Hard invariant: this row never blurs structured and fallback truth. MUST be
    /// `false`.
    pub blurs_structured_and_fallback: bool,
    /// Hard invariant: this row never hides the target identity. MUST be `false`.
    pub hides_target_identity: bool,
    /// Hard invariant: this row never presents a fallback as structured truth. MUST
    /// be `false`.
    pub presents_fallback_as_structured: bool,
}

impl M5BuildConfidenceSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5BuildConfidenceExportField> =
            self.export_fields.iter().copied().collect();
        M5BuildConfidenceExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.hides_adapter_source
            && !self.blurs_structured_and_fallback
            && !self.hides_target_identity
            && !self.presents_fallback_as_structured
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildConfidenceVocabularySet {
    /// Confidence surface-family tokens.
    pub surface_families: Vec<String>,
    /// Build-verb tokens.
    pub build_verbs: Vec<String>,
    /// Build-action tokens.
    pub action_kinds: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Adapter-source tokens (reused from the frozen matrix).
    pub adapter_source_kinds: Vec<String>,
    /// Capability-state tokens (reused from the frozen matrix).
    pub capability_states: Vec<String>,
    /// Raw-event-channel tokens (reused from the frozen matrix).
    pub raw_event_channels: Vec<String>,
    /// Target-graph node-kind tokens (reused from the frozen matrix).
    pub target_graph_node_kinds: Vec<String>,
    /// Fallback-confidence-state tokens (reused from the frozen matrix).
    pub fallback_confidence_states: Vec<String>,
    /// Fallback-reason tokens (reused from the frozen matrix).
    pub fallback_reasons: Vec<String>,
    /// Fallback-recovery-route tokens (reused from the frozen matrix).
    pub fallback_recovery_routes: Vec<String>,
    /// Truth-class tokens (reused from the frozen matrix).
    pub truth_modes: Vec<String>,
    /// Resource-freshness tokens (reused from the frozen matrix).
    pub resource_freshness: Vec<String>,
    /// Discovery-confidence tokens (reused from the frozen matrix).
    pub discovery_confidence: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5BuildConfidenceVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5BuildConfidenceSurfaceFamily::ALL, |v| v.as_str()),
            build_verbs: tokens(&M5BuildVerb::ALL, |v| v.as_str()),
            action_kinds: tokens(&M5BuildActionKind::ALL, |v| v.as_str()),
            export_fields: tokens(&M5BuildConfidenceExportField::ALL, |v| v.as_str()),
            adapter_source_kinds: tokens(&ADAPTER_SOURCE_KIND_ALL, |v| v.as_str()),
            capability_states: tokens(&CAPABILITY_STATE_ALL, |v| v.as_str()),
            raw_event_channels: tokens(&RAW_EVENT_CHANNEL_ALL, |v| v.as_str()),
            target_graph_node_kinds: tokens(&TARGET_GRAPH_NODE_KIND_ALL, |v| v.as_str()),
            fallback_confidence_states: tokens(&FALLBACK_CONFIDENCE_STATE_ALL, |v| v.as_str()),
            fallback_reasons: tokens(&FALLBACK_REASON_ALL, |v| v.as_str()),
            fallback_recovery_routes: tokens(&FALLBACK_RECOVERY_ROUTE_ALL, |v| v.as_str()),
            truth_modes: tokens(&TRUTH_MODE_ALL, truth_mode_token),
            resource_freshness: tokens(&RESOURCE_FRESHNESS_ALL, |v| v.as_str()),
            discovery_confidence: tokens(&DISCOVERY_CONFIDENCE_ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&DOWNGRADE_TRIGGER_ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The adapter source kinds reused from the frozen matrix, in a stable order.
const ADAPTER_SOURCE_KIND_ALL: [M5AdapterSourceKind; 6] = [
    M5AdapterSourceKind::NativeBuildServer,
    M5AdapterSourceKind::NativeBuildEvent,
    M5AdapterSourceKind::HeuristicParse,
    M5AdapterSourceKind::ImportedSnapshot,
    M5AdapterSourceKind::ProviderOverlay,
    M5AdapterSourceKind::Unknown,
];

/// The capability states reused from the frozen matrix, in a stable order.
const CAPABILITY_STATE_ALL: [M5CapabilityState; 5] = [
    M5CapabilityState::Supported,
    M5CapabilityState::Partial,
    M5CapabilityState::Unsupported,
    M5CapabilityState::Unknown,
    M5CapabilityState::ProviderGated,
];

/// The raw-event channels reused from the frozen matrix, in a stable order.
const RAW_EVENT_CHANNEL_ALL: [M5RawEventChannel; 5] = [
    M5RawEventChannel::NativeBuildEvent,
    M5RawEventChannel::NativeBuildServer,
    M5RawEventChannel::TaskEventBus,
    M5RawEventChannel::HeuristicParse,
    M5RawEventChannel::ImportedLog,
];

/// The target-graph node kinds reused from the frozen matrix, in a stable order.
const TARGET_GRAPH_NODE_KIND_ALL: [M5TargetGraphNodeKind; 5] = [
    M5TargetGraphNodeKind::BuildTarget,
    M5TargetGraphNodeKind::TestTarget,
    M5TargetGraphNodeKind::RunTarget,
    M5TargetGraphNodeKind::DependencyEdge,
    M5TargetGraphNodeKind::ContainerTarget,
];

/// The fallback-confidence states reused from the frozen matrix, in a stable
/// order.
const FALLBACK_CONFIDENCE_STATE_ALL: [M5FallbackConfidenceState; 5] = [
    M5FallbackConfidenceState::StructuredHigh,
    M5FallbackConfidenceState::StructuredDegraded,
    M5FallbackConfidenceState::HeuristicFallback,
    M5FallbackConfidenceState::ImportedOnly,
    M5FallbackConfidenceState::Unknown,
];

/// The fallback reasons reused from the frozen matrix, in a stable order.
const FALLBACK_REASON_ALL: [M5FallbackReason; 5] = [
    M5FallbackReason::AdapterUnavailable,
    M5FallbackReason::StructuredChannelLost,
    M5FallbackReason::SchemaDrift,
    M5FallbackReason::ConnectorLoss,
    M5FallbackReason::PolicyBlock,
];

/// The fallback recovery routes reused from the frozen matrix, in a stable order.
const FALLBACK_RECOVERY_ROUTE_ALL: [M5FallbackRecoveryRoute; 5] = [
    M5FallbackRecoveryRoute::ReattachAdapter,
    M5FallbackRecoveryRoute::RerunDiscovery,
    M5FallbackRecoveryRoute::InspectOnly,
    M5FallbackRecoveryRoute::OpenSourceTruth,
    M5FallbackRecoveryRoute::RetryConnector,
];

/// The truth classes reused from the frozen matrix, in a stable order.
/// [`TruthMode`] is a pure token set, so the order is pinned here.
const TRUTH_MODE_ALL: [TruthMode; 5] = [
    TruthMode::Desired,
    TruthMode::Rendered,
    TruthMode::Plan,
    TruthMode::Live,
    TruthMode::ProviderOverlay,
];

/// The resource-freshness states reused from the frozen matrix, in a stable
/// order.
const RESOURCE_FRESHNESS_ALL: [M5ResourceFreshness; 5] = [
    M5ResourceFreshness::LiveFresh,
    M5ResourceFreshness::CachedStale,
    M5ResourceFreshness::ImportedSnapshot,
    M5ResourceFreshness::PlanOnly,
    M5ResourceFreshness::Unknown,
];

/// The discovery-confidence states reused from the frozen matrix, in a stable
/// order.
const DISCOVERY_CONFIDENCE_ALL: [M5DiscoveryConfidence; 4] = [
    M5DiscoveryConfidence::High,
    M5DiscoveryConfidence::Medium,
    M5DiscoveryConfidence::Low,
    M5DiscoveryConfidence::Unknown,
];

/// The downgrade triggers reused from the frozen matrix, in a stable order.
const DOWNGRADE_TRIGGER_ALL: [M5ManifestBuildDowngradeTrigger; 8] = [
    M5ManifestBuildDowngradeTrigger::SchemaStale,
    M5ManifestBuildDowngradeTrigger::AdapterUnavailable,
    M5ManifestBuildDowngradeTrigger::ConnectorLoss,
    M5ManifestBuildDowngradeTrigger::PolicyBlock,
    M5ManifestBuildDowngradeTrigger::DriftFromSource,
    M5ManifestBuildDowngradeTrigger::LowConfidenceDiscovery,
    M5ManifestBuildDowngradeTrigger::StructuredChannelLost,
    M5ManifestBuildDowngradeTrigger::TargetContextUnresolved,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildConfidenceGovernanceReview {
    /// One primitive carries badge / target-graph / capability / raw-event /
    /// fallback truth on every surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Target identity is preserved across the badge, target-graph row, capability
    /// matrix, raw-event drawer, and fallback drawer.
    pub target_identity_preserved_across_surfaces: bool,
    /// The adapter source (native versus fallback) is never hidden.
    pub adapter_source_never_hidden: bool,
    /// Structured and fallback truth are never blurred.
    pub structured_and_fallback_never_blurred: bool,
    /// Target identity and confidence are inspectable before action.
    pub identity_and_confidence_inspectable_before_action: bool,
    /// The support / export packet reconstructs confidence truth.
    pub support_export_reconstructs_confidence: bool,
    /// Later M5 rows cannot invent parallel build-confidence vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildConfidenceConsumerProjection {
    /// Badge / target-graph / capability / raw-event / fallback surfaces all
    /// consume the shared primitive.
    pub confidence_surfaces_consume_shared_primitive: bool,
    /// The confidence resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The capability matrix reads a single canonical source.
    pub capability_matrix_reads_single_source: bool,
    /// Support and AI consumers reuse the shared component truth.
    pub support_and_ai_reuse_shared_component: bool,
}

/// Release and support parity posture for the build / run confidence primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildConfidenceReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting confidence audit.
    pub confidence_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BuildConfidencePrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BuildConfidencePrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5BuildConfidenceSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BuildConfidenceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BuildConfidenceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BuildConfidenceConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5BuildConfidenceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 build / run confidence primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildConfidencePrimitivePacket {
    /// Record kind; must equal [`M5_BUILD_CONFIDENCE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BUILD_CONFIDENCE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5BuildConfidenceSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BuildConfidenceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BuildConfidenceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BuildConfidenceConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5BuildConfidenceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BuildConfidencePrimitivePacket {
    /// Builds an M5 build / run confidence primitive packet from stable-lane input.
    pub fn new(input: M5BuildConfidencePrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_BUILD_CONFIDENCE_RECORD_KIND.to_owned(),
            schema_version: M5_BUILD_CONFIDENCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 build / run confidence primitive invariants.
    pub fn validate(&self) -> Vec<M5BuildConfidenceViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BUILD_CONFIDENCE_RECORD_KIND {
            violations.push(M5BuildConfidenceViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BUILD_CONFIDENCE_SCHEMA_VERSION {
            violations.push(M5BuildConfidenceViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BuildConfidenceViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 build-confidence primitive serializes"),
        ) {
            violations.push(M5BuildConfidenceViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 build-confidence primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,owner,adapter_sources,truth_modes,build_verbs,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.adapter_source_kinds, |v| v.as_str()),
                join_tokens(&row.truth_modes, |v| truth_mode_token(*v)),
                join_tokens(&row.build_verbs, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_confidence.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Build / Run Confidence Primitive: Adapter Badge, Target-Graph Row, Capability Matrix, Raw-Event Drawer, and Fallback Drawer\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Confidence surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5BuildConfidenceSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Adapter sources: {}\n",
            self.vocabulary_set.adapter_source_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Build verbs: {}\n",
            self.vocabulary_set.build_verbs.join(", ")
        ));
        out.push_str(&format!(
            "- Fallback states: {}\n",
            self.vocabulary_set.fallback_confidence_states.join(", ")
        ));
        out.push_str("\n## Confidence surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked cases: {}\n",
                row.example_confidence.len()
            ));
            for case in &row.example_confidence {
                out.push_str(&format!(
                    "    - `{}` → {} ({}), confidence `{}`, {}\n",
                    case.resolved.target_id,
                    case.resolved.adapter_badge.adapter_source.as_str(),
                    truth_mode_token(case.resolved.target_graph_row.truth_mode),
                    case.resolved.adapter_badge.confidence.as_str(),
                    if case.resolved.fallback_drawer.is_fallback {
                        "fallback"
                    } else {
                        "structured"
                    },
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 build / run confidence export.
#[derive(Debug)]
pub enum M5BuildConfidenceArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BuildConfidenceViolation>),
}

impl fmt::Display for M5BuildConfidenceArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 build-confidence primitive export parse failed: {error}"
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
                    "m5 build-confidence primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BuildConfidenceArtifactError {}

/// Validation failures emitted by [`M5BuildConfidencePrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BuildConfidenceViolation {
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
    /// A required confidence surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row declares no adapter source kinds.
    AdapterSourceMissing,
    /// A surface row declares no truth classes.
    TruthModeMissing,
    /// A surface row declares no build verbs.
    BuildVerbMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked confidence cases.
    ExampleConfidenceMissing,
    /// A worked confidence case does not match a fresh resolve of its input.
    ExampleConfidenceDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves adapter provenance disclosed and structured / fallback
    /// truth kept distinct (AC1).
    ProvenanceDisclosureUnproven,
    /// No worked case proves target identity and confidence inspectable before
    /// action (AC2).
    IdentityInspectabilityUnproven,
    /// No worked case proves support / AI reuse of the shared component (AC3).
    SupportReuseUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5BuildConfidenceViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::AdapterSourceMissing => "adapter_source_missing",
            Self::TruthModeMissing => "truth_mode_missing",
            Self::BuildVerbMissing => "build_verb_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleConfidenceMissing => "example_confidence_missing",
            Self::ExampleConfidenceDrift => "example_confidence_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::ProvenanceDisclosureUnproven => "provenance_disclosure_unproven",
            Self::IdentityInspectabilityUnproven => "identity_inspectability_unproven",
            Self::SupportReuseUnproven => "support_reuse_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 build / run confidence export.
pub fn current_stable_m5_build_confidence_export(
) -> Result<M5BuildConfidencePrimitivePacket, M5BuildConfidenceArtifactError> {
    let packet: M5BuildConfidencePrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-build-confidence-primitive-proof/support_export.json"
    )))
    .map_err(M5BuildConfidenceArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BuildConfidenceArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5BuildConfidencePrimitivePacket,
    violations: &mut Vec<M5BuildConfidenceViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BUILD_CONFIDENCE_SCHEMA_REF,
        M5_BUILD_CONFIDENCE_DOC_REF,
        M5_BUILD_CONFIDENCE_COMPONENT_MATRIX_REF,
        M5_BUILD_CONFIDENCE_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BuildConfidenceViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5BuildConfidencePrimitivePacket,
    violations: &mut Vec<M5BuildConfidenceViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5BuildConfidenceViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5BuildConfidencePrimitivePacket,
    violations: &mut Vec<M5BuildConfidenceViolation>,
) {
    let present: BTreeSet<M5BuildConfidenceSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5BuildConfidenceSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5BuildConfidenceViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5BuildConfidenceViolation::SurfaceRowIncomplete);
        }
        if row.adapter_source_kinds.is_empty() {
            violations.push(M5BuildConfidenceViolation::AdapterSourceMissing);
        }
        if row.truth_modes.is_empty() {
            violations.push(M5BuildConfidenceViolation::TruthModeMissing);
        }
        if row.build_verbs.is_empty() {
            violations.push(M5BuildConfidenceViolation::BuildVerbMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5BuildConfidenceViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5BuildConfidenceViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5BuildConfidenceViolation::ConsumerSurfacesMissing);
        }
        if row.example_confidence.is_empty() {
            violations.push(M5BuildConfidenceViolation::ExampleConfidenceMissing);
        }
        if row
            .example_confidence
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5BuildConfidenceViolation::ExampleConfidenceDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5BuildConfidenceViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated by at least one worked case
/// across the matrix: adapter provenance disclosed and structured / fallback truth
/// kept distinct (AC1), target identity and confidence inspectable before action
/// (AC2), and the shared component reusable by support / AI (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5BuildConfidencePrimitivePacket,
    violations: &mut Vec<M5BuildConfidenceViolation>,
) {
    let cases: Vec<&M5ResolvedBuildConfidence> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_confidence.iter().map(|case| &case.resolved))
        .collect();

    // AC1: some case discloses a non-native fallback lane as a fallback (never
    // masked), and every case keeps provenance disclosed consistently.
    let provenance_proven = cases.iter().any(|resolved| {
        !resolved.adapter_badge.is_native && resolved.fallback_drawer.is_fallback
    }) && cases
        .iter()
        .all(|resolved| resolved.provenance_disclosed_consistently());
    if !provenance_proven {
        violations.push(M5BuildConfidenceViolation::ProvenanceDisclosureUnproven);
    }

    // AC2: some case exposes a downgraded verb (proving the matrix explains it),
    // and every case keeps identity and confidence inspectable before action.
    let identity_proven = cases
        .iter()
        .any(|resolved| !resolved.capability_matrix.downgraded_verbs.is_empty())
        && cases.iter().all(|resolved| {
            resolved.identity_consistent() && resolved.identity_and_confidence_inspectable()
        });
    if !identity_proven {
        violations.push(M5BuildConfidenceViolation::IdentityInspectabilityUnproven);
    }

    // AC3: every case reconstructs redacted, exportable, identity-preserving truth
    // for support / AI reuse, and at least one case proves it.
    let support_proven = cases.iter().any(|resolved| resolved.support_reuse_ready())
        && cases.iter().all(|resolved| resolved.support_reuse_ready());
    if !support_proven {
        violations.push(M5BuildConfidenceViolation::SupportReuseUnproven);
    }
}

fn validate_governance_review(
    packet: &M5BuildConfidencePrimitivePacket,
    violations: &mut Vec<M5BuildConfidenceViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.target_identity_preserved_across_surfaces,
        review.adapter_source_never_hidden,
        review.structured_and_fallback_never_blurred,
        review.identity_and_confidence_inspectable_before_action,
        review.support_export_reconstructs_confidence,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5BuildConfidenceViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BuildConfidencePrimitivePacket,
    violations: &mut Vec<M5BuildConfidenceViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.confidence_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.capability_matrix_reads_single_source,
        projection.support_and_ai_reuse_shared_component,
    ] {
        if !ok {
            violations.push(M5BuildConfidenceViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5BuildConfidencePrimitivePacket,
    violations: &mut Vec<M5BuildConfidenceViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.confidence_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BuildConfidenceViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
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
        serde_json::Value::String(s) => value_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

include!("seed.rs");
