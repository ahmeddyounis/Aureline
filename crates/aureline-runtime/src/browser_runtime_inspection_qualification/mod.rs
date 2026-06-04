//! Browser-runtime inspection qualification packet for stable web rows.
//!
//! The packet certifies that browser-runtime inspection surfaces keep runtime
//! identity, session freshness, source-map quality, cross-origin limits,
//! redaction posture, and mutation safety explicit before any surface claims
//! stable DOM, CSS, console, network, or storage depth.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`BrowserRuntimeInspectionQualificationPacket`].
pub const BROWSER_RUNTIME_INSPECTION_QUALIFICATION_RECORD_KIND: &str =
    "browser_runtime_inspection_qualification_packet";

/// Stable record-kind tag for [`BrowserRuntimeInspectionQualificationSupportExport`].
pub const BROWSER_RUNTIME_INSPECTION_QUALIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "browser_runtime_inspection_qualification_support_export";

/// Integer schema version for the browser-runtime inspection packet.
pub const BROWSER_RUNTIME_INSPECTION_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the browser-runtime inspection qualification schema.
pub const BROWSER_RUNTIME_INSPECTION_QUALIFICATION_SCHEMA_REF: &str =
    "schemas/runtime/browser-runtime-inspection-qualification.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const BROWSER_RUNTIME_INSPECTION_QUALIFICATION_DOC_REF: &str =
    "docs/runtime/m4/browser-runtime-inspection-qualification.md";

/// Repo-relative path of the reviewer artifact.
pub const BROWSER_RUNTIME_INSPECTION_QUALIFICATION_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/browser-runtime-inspection-qualification.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const BROWSER_RUNTIME_INSPECTION_QUALIFICATION_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/browser-runtime-inspection-qualification";

/// Closed target-kind vocabulary exposed by browser-runtime inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserRuntimeTargetKind {
    /// Browser runtime hosted inside the in-product preview surface.
    EmbeddedPreview,
    /// Runtime attached to a user-visible external browser tab.
    ExternalBrowserTab,
    /// Simulator or emulator webview runtime.
    SimulatorWebview,
    /// Physical-device browser runtime.
    DeviceBrowser,
    /// Physical-device webview runtime.
    DeviceWebview,
    /// Remote or managed preview session runtime.
    RemotePreviewSession,
    /// Captured, imported, or replayed browser-runtime snapshot.
    CapturedSnapshot,
    /// Row is not about a concrete runtime target.
    NotApplicable,
}

impl BrowserRuntimeTargetKind {
    /// Every target kind a stable packet must cover.
    pub const REQUIRED: [Self; 7] = [
        Self::EmbeddedPreview,
        Self::ExternalBrowserTab,
        Self::SimulatorWebview,
        Self::DeviceBrowser,
        Self::DeviceWebview,
        Self::RemotePreviewSession,
        Self::CapturedSnapshot,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmbeddedPreview => "embedded_preview",
            Self::ExternalBrowserTab => "external_browser_tab",
            Self::SimulatorWebview => "simulator_webview",
            Self::DeviceBrowser => "device_browser",
            Self::DeviceWebview => "device_webview",
            Self::RemotePreviewSession => "remote_preview_session",
            Self::CapturedSnapshot => "captured_snapshot",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed row vocabulary for the qualification packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserRuntimeQualificationRowClass {
    /// Row proving one browser-runtime target kind.
    TargetKindAdmission,
    /// Row proving a runtime object class stays distinct.
    ObjectClassAdmission,
    /// Row proving a source-map quality class.
    SourceMapQualityAdmission,
    /// Row proving one console, network, or storage state label.
    InspectionStateAdmission,
    /// Row proving one mutating action has review safety.
    MutationActionReview,
    /// Row binding docs, Help, support, release, or optional labels.
    ConsumerSurfaceBinding,
    /// Row naming an automatic downgrade trigger.
    DowngradeRule,
}

impl BrowserRuntimeQualificationRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetKindAdmission => "target_kind_admission",
            Self::ObjectClassAdmission => "object_class_admission",
            Self::SourceMapQualityAdmission => "source_map_quality_admission",
            Self::InspectionStateAdmission => "inspection_state_admission",
            Self::MutationActionReview => "mutation_action_review",
            Self::ConsumerSurfaceBinding => "consumer_surface_binding",
            Self::DowngradeRule => "downgrade_rule",
        }
    }
}

/// Closed support-class vocabulary for the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserRuntimeSupportClass {
    /// Surface may claim stable browser-runtime inspection for this row.
    Stable,
    /// Surface must label this row below stable.
    DowngradedBelowStable,
    /// Surface may inspect metadata but cannot claim live control.
    InspectOnly,
    /// Surface must hand off to an external browser or provider.
    HandoffOnly,
    /// Surface is blocked until evidence changes.
    Blocked,
    /// Support state is missing and must block stable claims.
    Unbound,
}

impl BrowserRuntimeSupportClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::DowngradedBelowStable => "downgraded_below_stable",
            Self::InspectOnly => "inspect_only",
            Self::HandoffOnly => "handoff_only",
            Self::Blocked => "blocked",
            Self::Unbound => "unbound",
        }
    }

    const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Stable)
    }
}

/// Runtime attach and protocol state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachProtocolState {
    /// Protocol is attached and live.
    AttachedLive,
    /// Runtime exists but must be attached before inspection.
    AttachRequired,
    /// Protocol is unavailable for this target.
    ProtocolUnavailable,
    /// Cross-origin policy limits the protocol surface.
    CrossOriginLimited,
    /// Target is available only through external browser handoff.
    ExternalBrowserOnly,
    /// Target is an immutable snapshot.
    SnapshotOnly,
    /// Row is not about attach protocol state.
    NotApplicable,
}

impl AttachProtocolState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AttachedLive => "attached_live",
            Self::AttachRequired => "attach_required",
            Self::ProtocolUnavailable => "protocol_unavailable",
            Self::CrossOriginLimited => "cross_origin_limited",
            Self::ExternalBrowserOnly => "external_browser_only",
            Self::SnapshotOnly => "snapshot_only",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Browser-runtime session freshness and drift posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionFreshnessState {
    /// Events and runtime state are live.
    Live,
    /// Runtime state is recent enough for read-only inspection.
    Recent,
    /// Runtime state is stale until a resync occurs.
    StaleRequiresResync,
    /// Runtime state comes from a captured snapshot.
    CapturedSnapshot,
    /// Runtime freshness is not known.
    Unknown,
    /// Row is not about session freshness.
    NotApplicable,
}

impl SessionFreshnessState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Recent => "recent",
            Self::StaleRequiresResync => "stale_requires_resync",
            Self::CapturedSnapshot => "captured_snapshot",
            Self::Unknown => "unknown",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Source-map quality presented by browser-runtime inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceMapQualityState {
    /// Runtime evidence maps exactly to source.
    Exact,
    /// Runtime evidence maps approximately to source.
    Approximate,
    /// Runtime evidence maps to framework component identity only.
    FrameworkOnly,
    /// Runtime identity is known without source mapping.
    RuntimeOnly,
    /// Source map exists but is stale.
    Stale,
    /// Source map is unavailable.
    Unavailable,
    /// Row is not about source-map quality.
    NotApplicable,
}

impl SourceMapQualityState {
    /// Every mapping quality the packet must expose.
    pub const REQUIRED: [Self; 6] = [
        Self::Exact,
        Self::Approximate,
        Self::FrameworkOnly,
        Self::RuntimeOnly,
        Self::Stale,
        Self::Unavailable,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::FrameworkOnly => "framework_only",
            Self::RuntimeOnly => "runtime_only",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
            Self::NotApplicable => "not_applicable",
        }
    }

    const fn blocks_stable_target(self) -> bool {
        matches!(self, Self::Stale | Self::Unavailable | Self::NotApplicable)
    }
}

/// Runtime object classes that must not be flattened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeObjectClass {
    /// Concrete runtime DOM node.
    DomNode,
    /// Framework-level component hint.
    FrameworkComponent,
    /// Source file and symbol target.
    SourceFileSymbol,
    /// Cross-origin frame placeholder or boundary object.
    CrossOriginFrame,
    /// Row is not about a runtime object.
    NotApplicable,
}

impl RuntimeObjectClass {
    /// Every runtime object class the packet must expose.
    pub const REQUIRED: [Self; 4] = [
        Self::DomNode,
        Self::FrameworkComponent,
        Self::SourceFileSymbol,
        Self::CrossOriginFrame,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DomNode => "dom_node",
            Self::FrameworkComponent => "framework_component",
            Self::SourceFileSymbol => "source_file_symbol",
            Self::CrossOriginFrame => "cross_origin_frame",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Console, network, and storage row states that must stay distinct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectionDataState {
    /// Data is available for this lane.
    DataAvailable,
    /// There is no data for the lane.
    NoData,
    /// Cross-origin policy limits this lane.
    CrossOriginLimited,
    /// Runtime protocol is unavailable.
    ProtocolUnavailable,
    /// Attach is required before data can be shown.
    AttachRequired,
    /// Data is available only by external browser handoff.
    ExternalBrowserOnly,
    /// Row is not about inspection data state.
    NotApplicable,
}

impl InspectionDataState {
    /// Every inspection state a stable packet must preserve.
    pub const REQUIRED: [Self; 6] = [
        Self::DataAvailable,
        Self::NoData,
        Self::CrossOriginLimited,
        Self::ProtocolUnavailable,
        Self::AttachRequired,
        Self::ExternalBrowserOnly,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DataAvailable => "data_available",
            Self::NoData => "no_data",
            Self::CrossOriginLimited => "cross_origin_limited",
            Self::ProtocolUnavailable => "protocol_unavailable",
            Self::AttachRequired => "attach_required",
            Self::ExternalBrowserOnly => "external_browser_only",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Mutating browser-runtime actions that require safety review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeMutationActionClass {
    /// Clear storage, cache, cookies, or session data.
    ClearStorage,
    /// Override cookie or client-state data.
    CookieOverride,
    /// Replay a network request.
    ReplayRequest,
    /// Edit style state in a live runtime.
    LiveStyleEdit,
    /// Force a runtime reload.
    ForceReload,
    /// Apply protocol-side override such as service-worker control.
    ProtocolOverride,
    /// Row is not about mutation.
    NotApplicable,
}

impl RuntimeMutationActionClass {
    /// Every mutation action a stable packet must review.
    pub const REQUIRED: [Self; 6] = [
        Self::ClearStorage,
        Self::CookieOverride,
        Self::ReplayRequest,
        Self::LiveStyleEdit,
        Self::ForceReload,
        Self::ProtocolOverride,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClearStorage => "clear_storage",
            Self::CookieOverride => "cookie_override",
            Self::ReplayRequest => "replay_request",
            Self::LiveStyleEdit => "live_style_edit",
            Self::ForceReload => "force_reload",
            Self::ProtocolOverride => "protocol_override",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surfaces that must read the same packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeInspectionConsumerSurface {
    /// In-product optional-surface labels and panes.
    ProductSurfaceLabel,
    /// Runtime docs and Help pages.
    DocsHelp,
    /// Support export and evidence bundles.
    SupportExport,
    /// Release packet or proof index.
    ReleasePacket,
    /// Optional-surface manifest.
    OptionalSurfaceManifest,
    /// Row is not about a consumer surface.
    NotApplicable,
}

impl RuntimeInspectionConsumerSurface {
    /// Every consumer surface a stable packet must bind.
    pub const REQUIRED: [Self; 5] = [
        Self::ProductSurfaceLabel,
        Self::DocsHelp,
        Self::SupportExport,
        Self::ReleasePacket,
        Self::OptionalSurfaceManifest,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductSurfaceLabel => "product_surface_label",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::ReleasePacket => "release_packet",
            Self::OptionalSurfaceManifest => "optional_surface_manifest",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Downgrade automation applied when qualification evidence is missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserRuntimeDowngradeRuleClass {
    /// No downgrade is active.
    None,
    /// Downgrade when runtime target identity is missing.
    TargetIdentityMissing,
    /// Downgrade when source mapping is missing, stale, or approximate beyond policy.
    SourceMapInsufficient,
    /// Downgrade when attach protocol is unavailable.
    ProtocolUnavailable,
    /// Downgrade when cross-origin boundaries limit DOM, style, storage, or network lanes.
    CrossOriginLimited,
    /// Downgrade when runtime session freshness is stale or unknown.
    SessionStale,
    /// Downgrade when mutation review lineage is missing.
    MutationSafetyMissing,
    /// Downgrade when redaction posture is unsafe or raw material crosses the boundary.
    RedactionUnsafe,
    /// Downgrade when a consumer surface is not bound to this packet.
    ConsumerSurfaceMissing,
    /// Row is not about downgrade automation.
    NotApplicable,
}

impl BrowserRuntimeDowngradeRuleClass {
    /// Downgrade rules that must be present in a stable packet.
    pub const REQUIRED: [Self; 8] = [
        Self::TargetIdentityMissing,
        Self::SourceMapInsufficient,
        Self::ProtocolUnavailable,
        Self::CrossOriginLimited,
        Self::SessionStale,
        Self::MutationSafetyMissing,
        Self::RedactionUnsafe,
        Self::ConsumerSurfaceMissing,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::TargetIdentityMissing => "target_identity_missing",
            Self::SourceMapInsufficient => "source_map_insufficient",
            Self::ProtocolUnavailable => "protocol_unavailable",
            Self::CrossOriginLimited => "cross_origin_limited",
            Self::SessionStale => "session_stale",
            Self::MutationSafetyMissing => "mutation_safety_missing",
            Self::RedactionUnsafe => "redaction_unsafe",
            Self::ConsumerSurfaceMissing => "consumer_surface_missing",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Evidence class bound to each packet row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserRuntimeEvidenceClass {
    /// Fixture corpus evidence.
    FixtureEvidence,
    /// Automated functional or validator evidence.
    AutomatedFunctionalEvidence,
    /// Design QA and accessibility evidence.
    DesignAccessibilityEvidence,
    /// Security or privacy review evidence.
    SecurityPrivacyEvidence,
    /// Failure and recovery drill evidence.
    FailureRecoveryDrillEvidence,
    /// Release evidence review.
    ReleaseEvidenceReview,
    /// Evidence is missing.
    Unbound,
}

impl BrowserRuntimeEvidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FixtureEvidence => "fixture_evidence",
            Self::AutomatedFunctionalEvidence => "automated_functional_evidence",
            Self::DesignAccessibilityEvidence => "design_accessibility_evidence",
            Self::SecurityPrivacyEvidence => "security_privacy_evidence",
            Self::FailureRecoveryDrillEvidence => "failure_recovery_drill_evidence",
            Self::ReleaseEvidenceReview => "release_evidence_review",
            Self::Unbound => "unbound",
        }
    }
}

/// Promotion state derived by the packet validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserRuntimePromotionState {
    /// All required evidence supports stable claims.
    Stable,
    /// Evidence exists but at least one row must be labeled below stable.
    NarrowedBelowStable,
    /// Evidence is missing or contradictory, blocking stable claims.
    BlocksStable,
}

impl BrowserRuntimePromotionState {
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
pub enum BrowserRuntimeFindingSeverity {
    /// Informational validation note.
    Info,
    /// Warning that narrows a row below stable.
    Warning,
    /// Finding that blocks stable promotion.
    Blocker,
}

impl BrowserRuntimeFindingSeverity {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Blocker => "blocker",
        }
    }
}

/// Finding kind emitted by the packet validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserRuntimeFindingKind {
    /// Packet metadata is incorrect.
    InvalidPacketMetadata,
    /// A required target kind row is missing.
    MissingTargetKindCoverage,
    /// A required runtime object class row is missing.
    MissingObjectClassCoverage,
    /// A required source-map quality row is missing.
    MissingSourceMapQualityCoverage,
    /// A required inspection data state row is missing.
    MissingInspectionStateCoverage,
    /// A required mutation action review row is missing.
    MissingMutationActionCoverage,
    /// A required consumer surface binding is missing.
    MissingConsumerSurfaceCoverage,
    /// A required downgrade rule is missing.
    MissingDowngradeRuleCoverage,
    /// Stable target row does not bind identity, origin, protocol, freshness, or drift.
    StableTargetMissingRuntimeTruth,
    /// Stable target row claims stale or unavailable source mapping.
    StableTargetWithInsufficientSourceMap,
    /// Runtime object classes are collapsed.
    RuntimeObjectClassCollapsed,
    /// Inspection state labels are collapsed.
    InspectionStateCollapsed,
    /// Mutation action lacks review, rollback/export, identity, redaction, or side-effect proof.
    MutationReviewUnsafe,
    /// Consumer surface does not consume the packet without overclaiming.
    ConsumerSurfaceNotBound,
    /// Row lacks evidence refs.
    MissingEvidenceRefs,
    /// Row uses unbound support or evidence.
    UnboundStableClaim,
    /// Downgraded row lacks disclosure.
    DisclosureMissing,
    /// Raw source, secrets, or ambient authority are present.
    UnsafeExportMaterialPresent,
    /// Preview and browser-runtime concepts are flattened.
    PreviewRuntimeConceptsFlattened,
}

impl BrowserRuntimeFindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidPacketMetadata => "invalid_packet_metadata",
            Self::MissingTargetKindCoverage => "missing_target_kind_coverage",
            Self::MissingObjectClassCoverage => "missing_object_class_coverage",
            Self::MissingSourceMapQualityCoverage => "missing_source_map_quality_coverage",
            Self::MissingInspectionStateCoverage => "missing_inspection_state_coverage",
            Self::MissingMutationActionCoverage => "missing_mutation_action_coverage",
            Self::MissingConsumerSurfaceCoverage => "missing_consumer_surface_coverage",
            Self::MissingDowngradeRuleCoverage => "missing_downgrade_rule_coverage",
            Self::StableTargetMissingRuntimeTruth => "stable_target_missing_runtime_truth",
            Self::StableTargetWithInsufficientSourceMap => {
                "stable_target_with_insufficient_source_map"
            }
            Self::RuntimeObjectClassCollapsed => "runtime_object_class_collapsed",
            Self::InspectionStateCollapsed => "inspection_state_collapsed",
            Self::MutationReviewUnsafe => "mutation_review_unsafe",
            Self::ConsumerSurfaceNotBound => "consumer_surface_not_bound",
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::UnboundStableClaim => "unbound_stable_claim",
            Self::DisclosureMissing => "disclosure_missing",
            Self::UnsafeExportMaterialPresent => "unsafe_export_material_present",
            Self::PreviewRuntimeConceptsFlattened => "preview_runtime_concepts_flattened",
        }
    }
}

/// One row in the browser-runtime inspection qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserRuntimeInspectionQualificationRow {
    /// Stable row identifier.
    pub row_id: String,
    /// Qualification row class.
    pub row_class: BrowserRuntimeQualificationRowClass,
    /// Stability claim for this row.
    pub support_class: BrowserRuntimeSupportClass,
    /// Browser-runtime target kind admitted by this row.
    pub target_kind: BrowserRuntimeTargetKind,
    /// Attach/protocol state for target rows.
    pub attach_protocol_state: AttachProtocolState,
    /// Session freshness state for target rows.
    pub session_freshness_state: SessionFreshnessState,
    /// Source-map quality represented by this row.
    pub source_map_quality: SourceMapQualityState,
    /// Runtime object class represented by this row.
    pub runtime_object_class: RuntimeObjectClass,
    /// Console, network, or storage state represented by this row.
    pub inspection_data_state: InspectionDataState,
    /// Mutation action represented by this row.
    pub mutation_action: RuntimeMutationActionClass,
    /// Consumer surface represented by this row.
    pub consumer_surface: RuntimeInspectionConsumerSurface,
    /// Downgrade rule represented by this row.
    pub downgrade_rule: BrowserRuntimeDowngradeRuleClass,
    /// Evidence class for this row.
    pub evidence_class: BrowserRuntimeEvidenceClass,
    /// Evidence refs proving the row.
    pub evidence_refs: Vec<String>,
    /// User-visible disclosure ref for narrowed or limited rows.
    pub disclosure_ref: Option<String>,
    /// Runtime target identity is bound by an opaque handle.
    pub target_identity_bound: bool,
    /// Origin scope is bound and distinct from preview share scope.
    pub origin_scope_bound: bool,
    /// Attach/protocol state is explicit.
    pub protocol_state_bound: bool,
    /// Session freshness is explicit.
    pub session_freshness_bound: bool,
    /// Drift and resync semantics are explicit.
    pub drift_resync_bound: bool,
    /// Preview and browser-runtime concepts remain distinct.
    pub preview_runtime_separation_preserved: bool,
    /// DOM node, component, source symbol, and frame classes remain distinct.
    pub object_class_distinction_preserved: bool,
    /// Inspection state labels remain distinct.
    pub inspection_state_distinction_preserved: bool,
    /// Mutation action routes through explicit review.
    pub approval_review_ref: Option<String>,
    /// Mutation action has rollback or export lineage.
    pub rollback_or_export_ref: Option<String>,
    /// Mutation action preserves target identity.
    pub target_identity_preserved_for_action: bool,
    /// Mutation action exports only redaction-safe evidence.
    pub redaction_safe_export: bool,
    /// Mutation action has no hidden side effects.
    pub hidden_side_effects_excluded: bool,
    /// Consumer reads this packet verbatim.
    pub consumes_packet_verbatim: bool,
    /// Consumer avoids devtools-depth overclaims.
    pub avoids_devtools_overclaim: bool,
    /// Consumer downgrades labels when the packet narrows.
    pub optional_labels_downgraded: bool,
    /// Raw DOM, URL, request, storage, and source material are excluded.
    pub raw_source_material_excluded: bool,
    /// Secrets are excluded.
    pub secrets_excluded: bool,
    /// Ambient live-control authority is excluded.
    pub ambient_authority_excluded: bool,
}

/// Input used to materialize a qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserRuntimeInspectionQualificationPacketInput {
    /// Stable packet identifier.
    pub packet_id: String,
    /// Workflow or surface family this packet qualifies.
    pub workflow_or_surface_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Rows to validate.
    pub rows: Vec<BrowserRuntimeInspectionQualificationRow>,
    /// Source contracts this packet composes by reference.
    pub source_contract_refs: Vec<String>,
}

/// A validation finding emitted while materializing the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserRuntimeInspectionQualificationFinding {
    /// Finding severity.
    pub severity: BrowserRuntimeFindingSeverity,
    /// Finding kind.
    pub finding_kind: BrowserRuntimeFindingKind,
    /// Row associated with the finding, if any.
    pub row_id: Option<String>,
    /// Human-readable summary.
    pub summary: String,
}

impl BrowserRuntimeInspectionQualificationFinding {
    fn blocker(
        finding_kind: BrowserRuntimeFindingKind,
        row_id: Option<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            severity: BrowserRuntimeFindingSeverity::Blocker,
            finding_kind,
            row_id,
            summary: summary.into(),
        }
    }

    fn warning(
        finding_kind: BrowserRuntimeFindingKind,
        row_id: Option<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            severity: BrowserRuntimeFindingSeverity::Warning,
            finding_kind,
            row_id,
            summary: summary.into(),
        }
    }
}

/// Canonical browser-runtime inspection qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserRuntimeInspectionQualificationPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Workflow or surface family this packet qualifies.
    pub workflow_or_surface_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Rows included in this packet.
    pub rows: Vec<BrowserRuntimeInspectionQualificationRow>,
    /// Source contracts this packet composes by reference.
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: BrowserRuntimePromotionState,
    /// Validation findings.
    pub validation_findings: Vec<BrowserRuntimeInspectionQualificationFinding>,
}

impl BrowserRuntimeInspectionQualificationPacket {
    /// Materializes and validates a browser-runtime inspection qualification packet.
    pub fn materialize(input: BrowserRuntimeInspectionQualificationPacketInput) -> Self {
        let mut findings = Vec::new();
        if input.packet_id.trim().is_empty()
            || input.workflow_or_surface_id.trim().is_empty()
            || input.generated_at.trim().is_empty()
        {
            findings.push(BrowserRuntimeInspectionQualificationFinding::blocker(
                BrowserRuntimeFindingKind::InvalidPacketMetadata,
                None,
                "Packet id, workflow id, and generated-at timestamp must be present.",
            ));
        }

        validate_coverage(
            &input.rows,
            &mut findings,
            BrowserRuntimeQualificationRowClass::TargetKindAdmission,
            BrowserRuntimeTargetKind::REQUIRED,
            |row| row.target_kind,
            BrowserRuntimeTargetKind::NotApplicable,
            BrowserRuntimeFindingKind::MissingTargetKindCoverage,
            "Target kind admission coverage is incomplete.",
        );
        validate_coverage(
            &input.rows,
            &mut findings,
            BrowserRuntimeQualificationRowClass::ObjectClassAdmission,
            RuntimeObjectClass::REQUIRED,
            |row| row.runtime_object_class,
            RuntimeObjectClass::NotApplicable,
            BrowserRuntimeFindingKind::MissingObjectClassCoverage,
            "Runtime object class coverage is incomplete.",
        );
        validate_coverage(
            &input.rows,
            &mut findings,
            BrowserRuntimeQualificationRowClass::SourceMapQualityAdmission,
            SourceMapQualityState::REQUIRED,
            |row| row.source_map_quality,
            SourceMapQualityState::NotApplicable,
            BrowserRuntimeFindingKind::MissingSourceMapQualityCoverage,
            "Source-map quality coverage is incomplete.",
        );
        validate_coverage(
            &input.rows,
            &mut findings,
            BrowserRuntimeQualificationRowClass::InspectionStateAdmission,
            InspectionDataState::REQUIRED,
            |row| row.inspection_data_state,
            InspectionDataState::NotApplicable,
            BrowserRuntimeFindingKind::MissingInspectionStateCoverage,
            "Inspection data state coverage is incomplete.",
        );
        validate_coverage(
            &input.rows,
            &mut findings,
            BrowserRuntimeQualificationRowClass::MutationActionReview,
            RuntimeMutationActionClass::REQUIRED,
            |row| row.mutation_action,
            RuntimeMutationActionClass::NotApplicable,
            BrowserRuntimeFindingKind::MissingMutationActionCoverage,
            "Mutation action review coverage is incomplete.",
        );
        validate_coverage(
            &input.rows,
            &mut findings,
            BrowserRuntimeQualificationRowClass::ConsumerSurfaceBinding,
            RuntimeInspectionConsumerSurface::REQUIRED,
            |row| row.consumer_surface,
            RuntimeInspectionConsumerSurface::NotApplicable,
            BrowserRuntimeFindingKind::MissingConsumerSurfaceCoverage,
            "Consumer surface coverage is incomplete.",
        );
        validate_coverage(
            &input.rows,
            &mut findings,
            BrowserRuntimeQualificationRowClass::DowngradeRule,
            BrowserRuntimeDowngradeRuleClass::REQUIRED,
            |row| row.downgrade_rule,
            BrowserRuntimeDowngradeRuleClass::NotApplicable,
            BrowserRuntimeFindingKind::MissingDowngradeRuleCoverage,
            "Downgrade rule coverage is incomplete.",
        );

        for row in &input.rows {
            validate_row(row, &mut findings);
        }

        let promotion_state = if findings
            .iter()
            .any(|finding| finding.severity == BrowserRuntimeFindingSeverity::Blocker)
        {
            BrowserRuntimePromotionState::BlocksStable
        } else if findings.is_empty() {
            BrowserRuntimePromotionState::Stable
        } else {
            BrowserRuntimePromotionState::NarrowedBelowStable
        };

        Self {
            record_kind: BROWSER_RUNTIME_INSPECTION_QUALIFICATION_RECORD_KIND.to_owned(),
            schema_version: BROWSER_RUNTIME_INSPECTION_QUALIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            rows: input.rows,
            source_contract_refs: input.source_contract_refs,
            promotion_state,
            validation_findings: findings,
        }
    }

    /// Returns true when the packet can support stable browser-runtime claims.
    pub const fn is_stable(&self) -> bool {
        matches!(self.promotion_state, BrowserRuntimePromotionState::Stable)
    }

    /// Builds the redaction-safe support export for this packet.
    pub fn support_export(
        &self,
        support_export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> BrowserRuntimeInspectionQualificationSupportExport {
        BrowserRuntimeInspectionQualificationSupportExport {
            record_kind: BROWSER_RUNTIME_INSPECTION_QUALIFICATION_SUPPORT_EXPORT_RECORD_KIND
                .to_owned(),
            schema_version: self.schema_version,
            support_export_id: support_export_id.into(),
            packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            promotion_state: self.promotion_state,
            target_kind_tokens: self
                .target_kind_tokens()
                .into_iter()
                .map(str::to_owned)
                .collect(),
            source_map_quality_tokens: self
                .source_map_quality_tokens()
                .into_iter()
                .map(str::to_owned)
                .collect(),
            inspection_data_state_tokens: self
                .inspection_data_state_tokens()
                .into_iter()
                .map(str::to_owned)
                .collect(),
            mutation_action_tokens: self
                .mutation_action_tokens()
                .into_iter()
                .map(str::to_owned)
                .collect(),
            consumer_surface_tokens: self
                .consumer_surface_tokens()
                .into_iter()
                .map(str::to_owned)
                .collect(),
            finding_kind_tokens: self
                .finding_kind_tokens()
                .into_iter()
                .map(str::to_owned)
                .collect(),
            redaction_safe: self.rows.iter().all(|row| {
                row.raw_source_material_excluded
                    && row.secrets_excluded
                    && row.ambient_authority_excluded
                    && row.redaction_safe_export
            }),
        }
    }

    /// Returns target-kind tokens present in the packet.
    pub fn target_kind_tokens(&self) -> Vec<&'static str> {
        token_set(
            self.rows
                .iter()
                .map(|row| row.target_kind.as_str())
                .filter(|token| *token != "not_applicable"),
        )
    }

    /// Returns source-map quality tokens present in the packet.
    pub fn source_map_quality_tokens(&self) -> Vec<&'static str> {
        token_set(
            self.rows
                .iter()
                .map(|row| row.source_map_quality.as_str())
                .filter(|token| *token != "not_applicable"),
        )
    }

    /// Returns inspection data state tokens present in the packet.
    pub fn inspection_data_state_tokens(&self) -> Vec<&'static str> {
        token_set(
            self.rows
                .iter()
                .map(|row| row.inspection_data_state.as_str())
                .filter(|token| *token != "not_applicable"),
        )
    }

    /// Returns mutation action tokens present in the packet.
    pub fn mutation_action_tokens(&self) -> Vec<&'static str> {
        token_set(
            self.rows
                .iter()
                .map(|row| row.mutation_action.as_str())
                .filter(|token| *token != "not_applicable"),
        )
    }

    /// Returns consumer surface tokens present in the packet.
    pub fn consumer_surface_tokens(&self) -> Vec<&'static str> {
        token_set(
            self.rows
                .iter()
                .map(|row| row.consumer_surface.as_str())
                .filter(|token| *token != "not_applicable"),
        )
    }

    /// Returns finding-kind tokens present in the packet.
    pub fn finding_kind_tokens(&self) -> Vec<&'static str> {
        token_set(
            self.validation_findings
                .iter()
                .map(|finding| finding.finding_kind.as_str()),
        )
    }
}

/// Redaction-safe support export for a browser-runtime inspection packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserRuntimeInspectionQualificationSupportExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support export identifier.
    pub support_export_id: String,
    /// Packet id this export cites.
    pub packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Derived promotion state.
    pub promotion_state: BrowserRuntimePromotionState,
    /// Browser-runtime target kind tokens.
    pub target_kind_tokens: Vec<String>,
    /// Source-map quality tokens.
    pub source_map_quality_tokens: Vec<String>,
    /// Inspection data state tokens.
    pub inspection_data_state_tokens: Vec<String>,
    /// Mutation action tokens.
    pub mutation_action_tokens: Vec<String>,
    /// Consumer surface tokens.
    pub consumer_surface_tokens: Vec<String>,
    /// Finding kind tokens.
    pub finding_kind_tokens: Vec<String>,
    /// True when export material excludes raw/private runtime state.
    pub redaction_safe: bool,
}

impl BrowserRuntimeInspectionQualificationSupportExport {
    /// Returns true when the support export is safe to include by default.
    pub const fn is_export_safe(&self) -> bool {
        self.redaction_safe
    }
}

/// Error returned when artifact parsing fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserRuntimeInspectionQualificationArtifactError {
    message: String,
}

impl BrowserRuntimeInspectionQualificationArtifactError {
    /// Builds an artifact error from a message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for BrowserRuntimeInspectionQualificationArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for BrowserRuntimeInspectionQualificationArtifactError {}

fn validate_row(
    row: &BrowserRuntimeInspectionQualificationRow,
    findings: &mut Vec<BrowserRuntimeInspectionQualificationFinding>,
) {
    if row.evidence_refs.is_empty() {
        findings.push(BrowserRuntimeInspectionQualificationFinding::blocker(
            BrowserRuntimeFindingKind::MissingEvidenceRefs,
            Some(row.row_id.clone()),
            "Every qualification row must bind evidence refs.",
        ));
    }
    if row.support_class == BrowserRuntimeSupportClass::Unbound
        || row.evidence_class == BrowserRuntimeEvidenceClass::Unbound
    {
        findings.push(BrowserRuntimeInspectionQualificationFinding::blocker(
            BrowserRuntimeFindingKind::UnboundStableClaim,
            Some(row.row_id.clone()),
            "Rows cannot carry unbound support or evidence.",
        ));
    }
    if row.support_class.requires_disclosure() && row.disclosure_ref.is_none() {
        findings.push(BrowserRuntimeInspectionQualificationFinding::warning(
            BrowserRuntimeFindingKind::DisclosureMissing,
            Some(row.row_id.clone()),
            "Rows below stable must carry a disclosure ref.",
        ));
    }
    if !row.raw_source_material_excluded || !row.secrets_excluded || !row.ambient_authority_excluded
    {
        findings.push(BrowserRuntimeInspectionQualificationFinding::blocker(
            BrowserRuntimeFindingKind::UnsafeExportMaterialPresent,
            Some(row.row_id.clone()),
            "Browser-runtime rows cannot export raw source/runtime material, secrets, or ambient authority.",
        ));
    }
    if !row.preview_runtime_separation_preserved {
        findings.push(BrowserRuntimeInspectionQualificationFinding::blocker(
            BrowserRuntimeFindingKind::PreviewRuntimeConceptsFlattened,
            Some(row.row_id.clone()),
            "Preview and browser-runtime inspection concepts must remain distinct.",
        ));
    }

    match row.row_class {
        BrowserRuntimeQualificationRowClass::TargetKindAdmission => {
            if row.support_class == BrowserRuntimeSupportClass::Stable
                && !(row.target_identity_bound
                    && row.origin_scope_bound
                    && row.protocol_state_bound
                    && row.session_freshness_bound
                    && row.drift_resync_bound)
            {
                findings.push(BrowserRuntimeInspectionQualificationFinding::blocker(
                    BrowserRuntimeFindingKind::StableTargetMissingRuntimeTruth,
                    Some(row.row_id.clone()),
                    "Stable target rows must bind identity, origin, protocol, freshness, and drift semantics.",
                ));
            }
            if row.support_class == BrowserRuntimeSupportClass::Stable
                && row.source_map_quality.blocks_stable_target()
            {
                findings.push(BrowserRuntimeInspectionQualificationFinding::blocker(
                    BrowserRuntimeFindingKind::StableTargetWithInsufficientSourceMap,
                    Some(row.row_id.clone()),
                    "Stable target rows cannot claim stale, unavailable, or missing source mapping.",
                ));
            }
        }
        BrowserRuntimeQualificationRowClass::ObjectClassAdmission => {
            if !row.object_class_distinction_preserved {
                findings.push(BrowserRuntimeInspectionQualificationFinding::blocker(
                    BrowserRuntimeFindingKind::RuntimeObjectClassCollapsed,
                    Some(row.row_id.clone()),
                    "DOM node, framework component, source symbol, and cross-origin frame classes must remain distinct.",
                ));
            }
        }
        BrowserRuntimeQualificationRowClass::InspectionStateAdmission => {
            if !row.inspection_state_distinction_preserved {
                findings.push(BrowserRuntimeInspectionQualificationFinding::blocker(
                    BrowserRuntimeFindingKind::InspectionStateCollapsed,
                    Some(row.row_id.clone()),
                    "No data, cross-origin limited, protocol unavailable, attach required, and external-browser-only states must remain distinct.",
                ));
            }
        }
        BrowserRuntimeQualificationRowClass::MutationActionReview => {
            if row.support_class == BrowserRuntimeSupportClass::Stable
                && !(row.approval_review_ref.is_some()
                    && row.rollback_or_export_ref.is_some()
                    && row.target_identity_preserved_for_action
                    && row.redaction_safe_export
                    && row.hidden_side_effects_excluded)
            {
                findings.push(BrowserRuntimeInspectionQualificationFinding::blocker(
                    BrowserRuntimeFindingKind::MutationReviewUnsafe,
                    Some(row.row_id.clone()),
                    "Stable mutation actions must prove review, rollback/export lineage, target identity, redaction, and no hidden side effects.",
                ));
            }
        }
        BrowserRuntimeQualificationRowClass::ConsumerSurfaceBinding => {
            if !(row.consumes_packet_verbatim
                && row.avoids_devtools_overclaim
                && row.optional_labels_downgraded)
            {
                findings.push(BrowserRuntimeInspectionQualificationFinding::blocker(
                    BrowserRuntimeFindingKind::ConsumerSurfaceNotBound,
                    Some(row.row_id.clone()),
                    "Consumer surfaces must read this packet, avoid devtools-depth overclaims, and downgrade labels from packet state.",
                ));
            }
        }
        BrowserRuntimeQualificationRowClass::SourceMapQualityAdmission
        | BrowserRuntimeQualificationRowClass::DowngradeRule => {}
    }
}

fn validate_coverage<T, const N: usize, F>(
    rows: &[BrowserRuntimeInspectionQualificationRow],
    findings: &mut Vec<BrowserRuntimeInspectionQualificationFinding>,
    row_class: BrowserRuntimeQualificationRowClass,
    required: [T; N],
    value_for_row: F,
    not_applicable: T,
    finding_kind: BrowserRuntimeFindingKind,
    summary: &'static str,
) where
    T: Copy + Ord + Eq,
    F: Fn(&BrowserRuntimeInspectionQualificationRow) -> T,
{
    let observed: BTreeSet<T> = rows
        .iter()
        .filter(|row| row.row_class == row_class)
        .map(value_for_row)
        .filter(|value| *value != not_applicable)
        .collect();
    for required_value in required {
        if !observed.contains(&required_value) {
            findings.push(BrowserRuntimeInspectionQualificationFinding::blocker(
                finding_kind,
                None,
                summary,
            ));
        }
    }
}

fn token_set(tokens: impl Iterator<Item = &'static str>) -> Vec<&'static str> {
    tokens.collect::<BTreeSet<_>>().into_iter().collect()
}

/// Builds the checked-in stable browser-runtime inspection qualification packet.
pub fn current_stable_browser_runtime_inspection_qualification_packet(
) -> BrowserRuntimeInspectionQualificationPacket {
    BrowserRuntimeInspectionQualificationPacket::materialize(
        current_stable_browser_runtime_inspection_qualification_input(),
    )
}

/// Builds the canonical stable packet input used by fixtures and artifacts.
pub fn current_stable_browser_runtime_inspection_qualification_input(
) -> BrowserRuntimeInspectionQualificationPacketInput {
    let mut rows = Vec::new();
    for target_kind in BrowserRuntimeTargetKind::REQUIRED {
        let (support_class, attach_protocol_state, session_freshness_state, source_map_quality) =
            match target_kind {
                BrowserRuntimeTargetKind::CapturedSnapshot => (
                    BrowserRuntimeSupportClass::InspectOnly,
                    AttachProtocolState::SnapshotOnly,
                    SessionFreshnessState::CapturedSnapshot,
                    SourceMapQualityState::RuntimeOnly,
                ),
                BrowserRuntimeTargetKind::ExternalBrowserTab
                | BrowserRuntimeTargetKind::SimulatorWebview
                | BrowserRuntimeTargetKind::DeviceBrowser
                | BrowserRuntimeTargetKind::DeviceWebview => (
                    BrowserRuntimeSupportClass::DowngradedBelowStable,
                    AttachProtocolState::AttachRequired,
                    SessionFreshnessState::Recent,
                    SourceMapQualityState::Approximate,
                ),
                BrowserRuntimeTargetKind::RemotePreviewSession => (
                    BrowserRuntimeSupportClass::Stable,
                    AttachProtocolState::AttachedLive,
                    SessionFreshnessState::Live,
                    SourceMapQualityState::Exact,
                ),
                BrowserRuntimeTargetKind::EmbeddedPreview => (
                    BrowserRuntimeSupportClass::Stable,
                    AttachProtocolState::AttachedLive,
                    SessionFreshnessState::Live,
                    SourceMapQualityState::Exact,
                ),
                BrowserRuntimeTargetKind::NotApplicable => unreachable!(),
            };
        rows.push(
            base_row(
                format!("target:{}", target_kind.as_str()),
                BrowserRuntimeQualificationRowClass::TargetKindAdmission,
                support_class,
                BrowserRuntimeEvidenceClass::AutomatedFunctionalEvidence,
            )
            .with_target(
                target_kind,
                attach_protocol_state,
                session_freshness_state,
                source_map_quality,
            ),
        );
    }
    for object_class in RuntimeObjectClass::REQUIRED {
        rows.push(
            base_row(
                format!("object:{}", object_class.as_str()),
                BrowserRuntimeQualificationRowClass::ObjectClassAdmission,
                BrowserRuntimeSupportClass::Stable,
                BrowserRuntimeEvidenceClass::DesignAccessibilityEvidence,
            )
            .with_object(object_class),
        );
    }
    for quality in SourceMapQualityState::REQUIRED {
        let support_class = if quality.blocks_stable_target() {
            BrowserRuntimeSupportClass::InspectOnly
        } else {
            BrowserRuntimeSupportClass::Stable
        };
        rows.push(
            base_row(
                format!("source-map:{}", quality.as_str()),
                BrowserRuntimeQualificationRowClass::SourceMapQualityAdmission,
                support_class,
                BrowserRuntimeEvidenceClass::FixtureEvidence,
            )
            .with_source_map(quality),
        );
    }
    for state in InspectionDataState::REQUIRED {
        let support_class = if state == InspectionDataState::DataAvailable {
            BrowserRuntimeSupportClass::Stable
        } else {
            BrowserRuntimeSupportClass::InspectOnly
        };
        rows.push(
            base_row(
                format!("inspection-state:{}", state.as_str()),
                BrowserRuntimeQualificationRowClass::InspectionStateAdmission,
                support_class,
                BrowserRuntimeEvidenceClass::FixtureEvidence,
            )
            .with_inspection_state(state),
        );
    }
    for action in RuntimeMutationActionClass::REQUIRED {
        rows.push(
            base_row(
                format!("mutation:{}", action.as_str()),
                BrowserRuntimeQualificationRowClass::MutationActionReview,
                BrowserRuntimeSupportClass::Stable,
                BrowserRuntimeEvidenceClass::SecurityPrivacyEvidence,
            )
            .with_mutation(action),
        );
    }
    for surface in RuntimeInspectionConsumerSurface::REQUIRED {
        rows.push(
            base_row(
                format!("consumer:{}", surface.as_str()),
                BrowserRuntimeQualificationRowClass::ConsumerSurfaceBinding,
                BrowserRuntimeSupportClass::Stable,
                BrowserRuntimeEvidenceClass::ReleaseEvidenceReview,
            )
            .with_consumer(surface),
        );
    }
    for rule in BrowserRuntimeDowngradeRuleClass::REQUIRED {
        rows.push(
            base_row(
                format!("downgrade:{}", rule.as_str()),
                BrowserRuntimeQualificationRowClass::DowngradeRule,
                BrowserRuntimeSupportClass::Stable,
                BrowserRuntimeEvidenceClass::FailureRecoveryDrillEvidence,
            )
            .with_downgrade_rule(rule),
        );
    }

    BrowserRuntimeInspectionQualificationPacketInput {
        packet_id: "packet:runtime:browser_runtime_inspection_qualification".to_owned(),
        workflow_or_surface_id: "workflow.runtime.browser_runtime_inspection".to_owned(),
        generated_at: "2026-06-04T18:30:00Z".to_owned(),
        rows,
        source_contract_refs: vec![
            "docs/runtime/browser_runtime_contract.md".to_owned(),
            "docs/runtime/browser_inspection_contract.md".to_owned(),
            "docs/architecture/preview_runtime_contract.md".to_owned(),
            "schemas/runtime/browser_runtime_session.schema.json".to_owned(),
            "schemas/runtime/console_event.schema.json".to_owned(),
            "schemas/runtime/network_event_ref.schema.json".to_owned(),
            "schemas/runtime/storage_object_state.schema.json".to_owned(),
        ],
    }
}

fn base_row(
    row_id: String,
    row_class: BrowserRuntimeQualificationRowClass,
    support_class: BrowserRuntimeSupportClass,
    evidence_class: BrowserRuntimeEvidenceClass,
) -> BrowserRuntimeInspectionQualificationRow {
    let disclosure_ref = support_class
        .requires_disclosure()
        .then(|| format!("docs/runtime/m4/browser-runtime-inspection-qualification.md#{row_id}"));
    BrowserRuntimeInspectionQualificationRow {
        row_id,
        row_class,
        support_class,
        target_kind: BrowserRuntimeTargetKind::NotApplicable,
        attach_protocol_state: AttachProtocolState::NotApplicable,
        session_freshness_state: SessionFreshnessState::NotApplicable,
        source_map_quality: SourceMapQualityState::NotApplicable,
        runtime_object_class: RuntimeObjectClass::NotApplicable,
        inspection_data_state: InspectionDataState::NotApplicable,
        mutation_action: RuntimeMutationActionClass::NotApplicable,
        consumer_surface: RuntimeInspectionConsumerSurface::NotApplicable,
        downgrade_rule: BrowserRuntimeDowngradeRuleClass::NotApplicable,
        evidence_class,
        evidence_refs: vec![BROWSER_RUNTIME_INSPECTION_QUALIFICATION_FIXTURE_DIR.to_owned()],
        disclosure_ref,
        target_identity_bound: true,
        origin_scope_bound: true,
        protocol_state_bound: true,
        session_freshness_bound: true,
        drift_resync_bound: true,
        preview_runtime_separation_preserved: true,
        object_class_distinction_preserved: true,
        inspection_state_distinction_preserved: true,
        approval_review_ref: None,
        rollback_or_export_ref: None,
        target_identity_preserved_for_action: true,
        redaction_safe_export: true,
        hidden_side_effects_excluded: true,
        consumes_packet_verbatim: true,
        avoids_devtools_overclaim: true,
        optional_labels_downgraded: true,
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
    }
}

trait RowBuilder {
    fn with_target(
        self,
        target_kind: BrowserRuntimeTargetKind,
        attach_protocol_state: AttachProtocolState,
        session_freshness_state: SessionFreshnessState,
        source_map_quality: SourceMapQualityState,
    ) -> Self;
    fn with_object(self, object_class: RuntimeObjectClass) -> Self;
    fn with_source_map(self, source_map_quality: SourceMapQualityState) -> Self;
    fn with_inspection_state(self, inspection_data_state: InspectionDataState) -> Self;
    fn with_mutation(self, mutation_action: RuntimeMutationActionClass) -> Self;
    fn with_consumer(self, consumer_surface: RuntimeInspectionConsumerSurface) -> Self;
    fn with_downgrade_rule(self, downgrade_rule: BrowserRuntimeDowngradeRuleClass) -> Self;
}

impl RowBuilder for BrowserRuntimeInspectionQualificationRow {
    fn with_target(
        mut self,
        target_kind: BrowserRuntimeTargetKind,
        attach_protocol_state: AttachProtocolState,
        session_freshness_state: SessionFreshnessState,
        source_map_quality: SourceMapQualityState,
    ) -> Self {
        self.target_kind = target_kind;
        self.attach_protocol_state = attach_protocol_state;
        self.session_freshness_state = session_freshness_state;
        self.source_map_quality = source_map_quality;
        self
    }

    fn with_object(mut self, object_class: RuntimeObjectClass) -> Self {
        self.runtime_object_class = object_class;
        self
    }

    fn with_source_map(mut self, source_map_quality: SourceMapQualityState) -> Self {
        self.source_map_quality = source_map_quality;
        self
    }

    fn with_inspection_state(mut self, inspection_data_state: InspectionDataState) -> Self {
        self.inspection_data_state = inspection_data_state;
        self
    }

    fn with_mutation(mut self, mutation_action: RuntimeMutationActionClass) -> Self {
        self.mutation_action = mutation_action;
        self.approval_review_ref = Some(format!(
            "review:browser-runtime:{}",
            mutation_action.as_str()
        ));
        self.rollback_or_export_ref = Some(format!(
            "export:browser-runtime:{}",
            mutation_action.as_str()
        ));
        self
    }

    fn with_consumer(mut self, consumer_surface: RuntimeInspectionConsumerSurface) -> Self {
        self.consumer_surface = consumer_surface;
        self
    }

    fn with_downgrade_rule(mut self, downgrade_rule: BrowserRuntimeDowngradeRuleClass) -> Self {
        self.downgrade_rule = downgrade_rule;
        self
    }
}
