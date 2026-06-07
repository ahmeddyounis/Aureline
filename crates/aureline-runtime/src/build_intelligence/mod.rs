//! Build-intelligence health, target, receipt, and discovery-diff records.
//!
//! This module extends target-discovery confidence into the build/run/test
//! surfaces that must explain where target truth came from, whether that truth
//! is live or imported, how healthy the producing adapter is, and what changed
//! after a refresh. The records are intentionally export-safe: they carry
//! stable identities, closed reason vocabularies, action refs, and lineage
//! refs, while keeping raw paths, raw command lines, environment bodies, and
//! secrets outside the boundary.
//!
//! The cross-tool health and receipt boundary lives at
//! [`/schemas/runtime/adapter_health_strip.schema.json`](../../../../schemas/runtime/adapter_health_strip.schema.json).
//! Discovery refresh reviews live at
//! [`/schemas/runtime/discovery_diff.schema.json`](../../../../schemas/runtime/discovery_diff.schema.json).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::target_discovery::DiscoverySourceClass;

/// Schema version shared by build-intelligence records.
pub const BUILD_INTELLIGENCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for adapter-health strips.
pub const ADAPTER_HEALTH_STRIP_RECORD_KIND: &str = "adapter_health_strip_record";
/// Stable record-kind tag for build-intelligence target rows.
pub const BUILD_INTELLIGENCE_TARGET_ROW_RECORD_KIND: &str = "build_intelligence_target_row_record";
/// Stable record-kind tag for run-configuration cards.
pub const BUILD_INTELLIGENCE_RUN_CONFIG_CARD_RECORD_KIND: &str =
    "build_intelligence_run_config_card_record";
/// Stable record-kind tag for build summary receipts.
pub const BUILD_INTELLIGENCE_RECEIPT_RECORD_KIND: &str = "build_intelligence_receipt_record";
/// Stable record-kind tag for build-intelligence support exports.
pub const BUILD_INTELLIGENCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "build_intelligence_support_export_record";
/// Stable record-kind tag for build-intelligence coverage manifests.
pub const BUILD_INTELLIGENCE_COVERAGE_MANIFEST_RECORD_KIND: &str =
    "build_intelligence_coverage_manifest_record";
/// Stable record-kind tag for discovery-diff reviews.
pub const DISCOVERY_DIFF_REVIEW_RECORD_KIND: &str = "discovery_diff_review_record";
/// Repo-relative path of the stable tooling adapter-confidence schema.
pub const ADAPTER_CONFIDENCE_TOOLING_SCHEMA_REF: &str =
    "schemas/tooling/adapter-confidence.schema.json";
/// Repo-relative path of the stable adapter-confidence fixture corpus.
pub const ADAPTER_CONFIDENCE_TOOLING_FIXTURE_DIR: &str =
    "fixtures/tooling/m4/stabilize-build-intelligence-and-adapter-confidence";
/// Repo-relative path of the reviewer-facing adapter-confidence artifact.
pub const ADAPTER_CONFIDENCE_TOOLING_ARTIFACT_DOC_REF: &str =
    "artifacts/tooling/m4/stabilize-build-intelligence-and-adapter-confidence.md";
/// Repo-relative path of the stable adapter-confidence contract doc.
pub const ADAPTER_CONFIDENCE_TOOLING_DOC_REF: &str =
    "docs/m4/stabilize-build-intelligence-and-adapter-confidence.md";

/// Lane class that produced target or receipt truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildIntelligenceLaneType {
    /// First-party or certified adapter understands the build tool model.
    NativeAdapter,
    /// Structured protocol such as BSP or a comparable provider protocol.
    StructuredProtocol,
    /// Structured build-event stream such as BEP or BES.
    BuildEventStream,
    /// Imported machine-readable artifact without a current live adapter.
    StructuredOutputImport,
    /// File, script, output, or command-history heuristic.
    HeuristicFallback,
}

impl BuildIntelligenceLaneType {
    /// All lane types in the build-intelligence beta vocabulary.
    pub const ALL: [Self; 5] = [
        Self::NativeAdapter,
        Self::StructuredProtocol,
        Self::BuildEventStream,
        Self::StructuredOutputImport,
        Self::HeuristicFallback,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeAdapter => "native_adapter",
            Self::StructuredProtocol => "structured_protocol",
            Self::BuildEventStream => "build_event_stream",
            Self::StructuredOutputImport => "structured_output_import",
            Self::HeuristicFallback => "heuristic_fallback",
        }
    }

    /// Short label safe for UI, CLI, and support exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NativeAdapter => "Native adapter",
            Self::StructuredProtocol => "Structured protocol",
            Self::BuildEventStream => "Build-event stream",
            Self::StructuredOutputImport => "Structured output import",
            Self::HeuristicFallback => "Heuristic fallback",
        }
    }

    /// Maps the existing target-discovery source vocabulary into the build
    /// intelligence lane vocabulary.
    pub const fn from_discovery_source(source: DiscoverySourceClass) -> Self {
        match source {
            DiscoverySourceClass::NativeProtocol | DiscoverySourceClass::UserDeclared => {
                Self::NativeAdapter
            }
            DiscoverySourceClass::StructuredAdapter => Self::StructuredProtocol,
            DiscoverySourceClass::ImportedMetadata => Self::StructuredOutputImport,
            DiscoverySourceClass::HeuristicParser | DiscoverySourceClass::ResolverUnavailable => {
                Self::HeuristicFallback
            }
        }
    }
}

/// Current health state of the producer behind a build-intelligence lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterHealthState {
    /// Producer refreshed successfully and the current row is live.
    Healthy,
    /// Producer is usable but some capabilities are missing or stale.
    Partial,
    /// Producer failed a health dimension and cannot be trusted for all work.
    Degraded,
    /// Producer is unavailable for the current workspace.
    Unavailable,
    /// Row comes only from imported or replayed history.
    ImportedOnly,
}

impl AdapterHealthState {
    /// All adapter-health states in the build-intelligence vocabulary.
    pub const ALL: [Self; 5] = [
        Self::Healthy,
        Self::Partial,
        Self::Degraded,
        Self::Unavailable,
        Self::ImportedOnly,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Partial => "partial",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
            Self::ImportedOnly => "imported_only",
        }
    }
}

/// Precise reason for degraded, partial, imported, or unavailable health.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterHealthReason {
    /// Transport to the adapter, protocol server, or event stream failed.
    TransportFailure,
    /// Authentication or credential renewal blocked live refresh.
    AuthFailure,
    /// Producer and workspace roots or manifests no longer match.
    WorkspaceMismatch,
    /// Adapter, protocol, helper, or tool version is outside the trusted window.
    VersionSkew,
    /// Producer does not support a requested discovery or execution feature.
    UnsupportedFeatures,
    /// Structured parsing found ambiguous or conflicting target candidates.
    ParseAmbiguity,
    /// Imported artifact or cached snapshot is older than the current source.
    StaleArtifact,
    /// Managed or remote control plane is unavailable.
    ControlPlaneOutage,
    /// Managed workspace changed lifecycle state during or after refresh.
    ManagedWorkspaceLifecycleChange,
}

impl AdapterHealthReason {
    /// All health-reason tokens in the build-intelligence vocabulary.
    pub const ALL: [Self; 9] = [
        Self::TransportFailure,
        Self::AuthFailure,
        Self::WorkspaceMismatch,
        Self::VersionSkew,
        Self::UnsupportedFeatures,
        Self::ParseAmbiguity,
        Self::StaleArtifact,
        Self::ControlPlaneOutage,
        Self::ManagedWorkspaceLifecycleChange,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TransportFailure => "transport_failure",
            Self::AuthFailure => "auth_failure",
            Self::WorkspaceMismatch => "workspace_mismatch",
            Self::VersionSkew => "version_skew",
            Self::UnsupportedFeatures => "unsupported_features",
            Self::ParseAmbiguity => "parse_ambiguity",
            Self::StaleArtifact => "stale_artifact",
            Self::ControlPlaneOutage => "control_plane_outage",
            Self::ManagedWorkspaceLifecycleChange => "managed_workspace_lifecycle_change",
        }
    }
}

/// Action classes exposed by build-intelligence rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildIntelligenceActionClass {
    /// Refresh target discovery through the same lane.
    RefreshDiscovery,
    /// Repair the adapter or protocol binding.
    RepairAdapter,
    /// Reauthenticate the adapter, helper, or managed workspace.
    Reauthenticate,
    /// Open the producer details inspector.
    OpenDetails,
    /// Open the source file or authored declaration.
    OpenSource,
    /// Open the relevant build, launch, or adapter configuration.
    OpenConfig,
    /// Inspect the imported raw artifact or event stream.
    InspectRawArtifact,
    /// Continue with a local target while remote or managed discovery is partial.
    ContinueLocal,
    /// Keep the row available for inspect-only workflows.
    InspectOnly,
}

impl BuildIntelligenceActionClass {
    /// All action classes in the build-intelligence vocabulary.
    pub const ALL: [Self; 9] = [
        Self::RefreshDiscovery,
        Self::RepairAdapter,
        Self::Reauthenticate,
        Self::OpenDetails,
        Self::OpenSource,
        Self::OpenConfig,
        Self::InspectRawArtifact,
        Self::ContinueLocal,
        Self::InspectOnly,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshDiscovery => "refresh_discovery",
            Self::RepairAdapter => "repair_adapter",
            Self::Reauthenticate => "reauthenticate",
            Self::OpenDetails => "open_details",
            Self::OpenSource => "open_source",
            Self::OpenConfig => "open_config",
            Self::InspectRawArtifact => "inspect_raw_artifact",
            Self::ContinueLocal => "continue_local",
            Self::InspectOnly => "inspect_only",
        }
    }

    /// Short label safe for UI, CLI, and support exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RefreshDiscovery => "Refresh discovery",
            Self::RepairAdapter => "Repair adapter",
            Self::Reauthenticate => "Reauthenticate",
            Self::OpenDetails => "Open details",
            Self::OpenSource => "Open source",
            Self::OpenConfig => "Open config",
            Self::InspectRawArtifact => "Inspect raw artifact",
            Self::ContinueLocal => "Continue local",
            Self::InspectOnly => "Inspect only",
        }
    }
}

/// Provenance class distinguishing live inspection from stale or inferred truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportedLiveState {
    /// Current workspace inspection produced this row.
    LiveWorkspaceInspection,
    /// Row came from an imported structured artifact.
    ImportedArtifact,
    /// Row came from replaying a retained receipt or support packet.
    ReplayedReceipt,
    /// Row was inferred heuristically in the current workspace.
    HeuristicInference,
    /// Row combines current inspection with imported or replayed context.
    MixedLiveAndImported,
}

impl ImportedLiveState {
    /// All imported-versus-live provenance classes.
    pub const ALL: [Self; 5] = [
        Self::LiveWorkspaceInspection,
        Self::ImportedArtifact,
        Self::ReplayedReceipt,
        Self::HeuristicInference,
        Self::MixedLiveAndImported,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveWorkspaceInspection => "live_workspace_inspection",
            Self::ImportedArtifact => "imported_artifact",
            Self::ReplayedReceipt => "replayed_receipt",
            Self::HeuristicInference => "heuristic_inference",
            Self::MixedLiveAndImported => "mixed_live_and_imported",
        }
    }

    /// True when high-trust actions require refresh or review before dispatch.
    pub const fn is_not_live(self) -> bool {
        !matches!(self, Self::LiveWorkspaceInspection)
    }
}

/// Exactness class shown on target rows and run-configuration cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetExactnessStatus {
    /// Target identity is exact and backed by current adapter truth.
    Exact,
    /// Target is exact within a structured protocol or provider envelope.
    ProtocolBacked,
    /// Target identity came from imported or replayed evidence.
    Imported,
    /// Target was inferred by a heuristic fallback.
    Heuristic,
    /// Target is currently unresolved and cannot dispatch protected work.
    Unresolved,
}

impl TargetExactnessStatus {
    /// All target-exactness statuses.
    pub const ALL: [Self; 5] = [
        Self::Exact,
        Self::ProtocolBacked,
        Self::Imported,
        Self::Heuristic,
        Self::Unresolved,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::ProtocolBacked => "protocol_backed",
            Self::Imported => "imported",
            Self::Heuristic => "heuristic",
            Self::Unresolved => "unresolved",
        }
    }
}

/// Artifact source recorded in a build summary or receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactSourceClass {
    /// Artifact came from a live adapter result.
    LiveAdapter,
    /// Artifact came from a structured build-event stream.
    BuildEventStream,
    /// Artifact came from an imported structured output file.
    StructuredImport,
    /// Artifact came from a replayed receipt.
    ReplayedReceipt,
    /// Artifact was inferred by a heuristic parser or matcher.
    HeuristicInference,
    /// No artifact was produced.
    None,
}

impl ArtifactSourceClass {
    /// All artifact-source classes.
    pub const ALL: [Self; 6] = [
        Self::LiveAdapter,
        Self::BuildEventStream,
        Self::StructuredImport,
        Self::ReplayedReceipt,
        Self::HeuristicInference,
        Self::None,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveAdapter => "live_adapter",
            Self::BuildEventStream => "build_event_stream",
            Self::StructuredImport => "structured_import",
            Self::ReplayedReceipt => "replayed_receipt",
            Self::HeuristicInference => "heuristic_inference",
            Self::None => "none",
        }
    }
}

/// Posture for high-trust actions such as rerun, export, or publish evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighTrustActionPosture {
    /// Current live truth is sufficient for the action.
    LiveActionsAllowed,
    /// User or policy review is required before dispatch.
    ReviewBeforeDispatch,
    /// A refresh is required before dispatch.
    RefreshRequired,
    /// Only inspect/export surfaces may use this row.
    InspectOnly,
    /// A local alternative is available while remote or managed truth is partial.
    ContinueLocalAvailable,
    /// The adapter or workspace must be repaired before dispatch.
    BlockedUntilRepair,
}

impl HighTrustActionPosture {
    /// All high-trust action posture classes.
    pub const ALL: [Self; 6] = [
        Self::LiveActionsAllowed,
        Self::ReviewBeforeDispatch,
        Self::RefreshRequired,
        Self::InspectOnly,
        Self::ContinueLocalAvailable,
        Self::BlockedUntilRepair,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveActionsAllowed => "live_actions_allowed",
            Self::ReviewBeforeDispatch => "review_before_dispatch",
            Self::RefreshRequired => "refresh_required",
            Self::InspectOnly => "inspect_only",
            Self::ContinueLocalAvailable => "continue_local_available",
            Self::BlockedUntilRepair => "blocked_until_repair",
        }
    }
}

/// Discovery-refresh change class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryDiffChangeClass {
    /// Target exists only in the refreshed set.
    Added,
    /// Target existed only before refresh.
    Removed,
    /// Stable target id survived but the display name changed.
    Renamed,
    /// Confidence rank decreased after refresh.
    DowngradedConfidence,
    /// Target became heuristic after refresh.
    NewlyHeuristic,
    /// Target became exact after refresh.
    NewlyExact,
    /// Target became unresolved after refresh.
    NowUnresolved,
}

impl DiscoveryDiffChangeClass {
    /// All discovery-diff change classes.
    pub const ALL: [Self; 7] = [
        Self::Added,
        Self::Removed,
        Self::Renamed,
        Self::DowngradedConfidence,
        Self::NewlyHeuristic,
        Self::NewlyExact,
        Self::NowUnresolved,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Renamed => "renamed",
            Self::DowngradedConfidence => "downgraded_confidence",
            Self::NewlyHeuristic => "newly_heuristic",
            Self::NewlyExact => "newly_exact",
            Self::NowUnresolved => "now_unresolved",
        }
    }
}

/// Stable adapter or protocol identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterIdentity {
    /// Stable adapter id.
    pub adapter_id: String,
    /// Human-readable adapter or protocol label.
    pub adapter_label: String,
    /// Adapter implementation version, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adapter_version: Option<String>,
    /// Protocol id, when a structured protocol is involved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol_id: Option<String>,
    /// Protocol version, when negotiated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<String>,
}

impl AdapterIdentity {
    /// Builds a stable adapter identity.
    pub fn new(
        adapter_id: impl Into<String>,
        adapter_label: impl Into<String>,
        adapter_version: impl Into<Option<String>>,
        protocol_id: impl Into<Option<String>>,
        protocol_version: impl Into<Option<String>>,
    ) -> Self {
        Self {
            adapter_id: adapter_id.into(),
            adapter_label: adapter_label.into(),
            adapter_version: adapter_version.into(),
            protocol_id: protocol_id.into(),
            protocol_version: protocol_version.into(),
        }
    }
}

/// Action reference exposed by build-intelligence records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIntelligenceAction {
    /// Action class.
    pub action_class: BuildIntelligenceActionClass,
    /// Stable action token.
    pub action_token: String,
    /// Opaque command or deep-link ref.
    pub action_ref: String,
    /// Short label safe for UI, CLI, and support exports.
    pub label: String,
    /// True when the action is currently enabled.
    pub enabled: bool,
    /// Export-safe reason when disabled or narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl BuildIntelligenceAction {
    /// Builds an enabled action with the canonical token and label.
    pub fn enabled(
        action_class: BuildIntelligenceActionClass,
        action_ref: impl Into<String>,
    ) -> Self {
        Self {
            action_class,
            action_token: action_class.as_str().to_owned(),
            action_ref: action_ref.into(),
            label: action_class.label().to_owned(),
            enabled: true,
            reason: None,
        }
    }

    /// Builds a disabled action with the canonical token and label.
    pub fn disabled(
        action_class: BuildIntelligenceActionClass,
        action_ref: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            action_class,
            action_token: action_class.as_str().to_owned(),
            action_ref: action_ref.into(),
            label: action_class.label().to_owned(),
            enabled: false,
            reason: Some(reason.into()),
        }
    }
}

/// Refresh lineage copied into target rows, health strips, and receipts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefreshLineage {
    /// Stable refresh id for this snapshot.
    pub refresh_id: String,
    /// Previous refresh id, when the lineage is known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_refresh_id: Option<String>,
    /// Refresh request timestamp.
    pub requested_at: String,
    /// Refresh completion timestamp, when a refresh completed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// Snapshot or graph ref used as the source of current truth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_snapshot_ref: Option<String>,
    /// Raw payload ref for support replay, when retained.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_payload_ref: Option<String>,
    /// Current workspace inspection ref, when live truth exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_workspace_inspection_ref: Option<String>,
    /// Imported history or replay ref, when used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_history_ref: Option<String>,
}

impl RefreshLineage {
    /// Builds lineage with required refresh identity and timestamps.
    pub fn new(
        refresh_id: impl Into<String>,
        previous_refresh_id: impl Into<Option<String>>,
        requested_at: impl Into<String>,
        completed_at: impl Into<Option<String>>,
    ) -> Self {
        Self {
            refresh_id: refresh_id.into(),
            previous_refresh_id: previous_refresh_id.into(),
            requested_at: requested_at.into(),
            completed_at: completed_at.into(),
            source_snapshot_ref: None,
            raw_payload_ref: None,
            current_workspace_inspection_ref: None,
            imported_history_ref: None,
        }
    }

    /// Attaches source refs used to reconstruct the refresh.
    pub fn with_refs(
        mut self,
        source_snapshot_ref: impl Into<Option<String>>,
        raw_payload_ref: impl Into<Option<String>>,
        current_workspace_inspection_ref: impl Into<Option<String>>,
        imported_history_ref: impl Into<Option<String>>,
    ) -> Self {
        self.source_snapshot_ref = source_snapshot_ref.into();
        self.raw_payload_ref = raw_payload_ref.into();
        self.current_workspace_inspection_ref = current_workspace_inspection_ref.into();
        self.imported_history_ref = imported_history_ref.into();
        self
    }
}

/// Adapter-health strip shown above target rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterHealthStrip {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable strip id.
    pub strip_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Lane type.
    pub lane_type: BuildIntelligenceLaneType,
    /// Stable lane token.
    pub lane_type_token: String,
    /// Short lane label.
    pub lane_type_label: String,
    /// Adapter or protocol identity.
    pub adapter_identity: AdapterIdentity,
    /// Current health state.
    pub state: AdapterHealthState,
    /// Stable health-state token.
    pub state_token: String,
    /// Last successful refresh timestamp, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_successful_refresh_at: Option<String>,
    /// Precise degraded or partial reason, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_reason: Option<AdapterHealthReason>,
    /// Stable health-reason token, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_reason_token: Option<String>,
    /// Provenance class for imported-versus-live truth.
    pub imported_live_state: ImportedLiveState,
    /// Stable provenance token.
    pub imported_live_state_token: String,
    /// Refresh lineage for support export and replay.
    pub refresh_lineage: RefreshLineage,
    /// Primary repair or refresh action.
    pub repair_action: BuildIntelligenceAction,
    /// Details action opening the adapter or protocol inspector.
    pub details_action: BuildIntelligenceAction,
    /// Local continuation action when a remote or managed lane degrades.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continue_local_action: Option<BuildIntelligenceAction>,
    /// Inspect-only action when live dispatch is unsafe.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inspect_only_action: Option<BuildIntelligenceAction>,
    /// Export-safe strip summary.
    pub summary: String,
    /// True because raw paths, command lines, env bodies, and secrets are excluded.
    pub redaction_safe: bool,
}

impl AdapterHealthStrip {
    /// Builds an adapter-health strip.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        strip_id: impl Into<String>,
        workspace_id: impl Into<String>,
        lane_type: BuildIntelligenceLaneType,
        adapter_identity: AdapterIdentity,
        state: AdapterHealthState,
        health_reason: Option<AdapterHealthReason>,
        last_successful_refresh_at: impl Into<Option<String>>,
        imported_live_state: ImportedLiveState,
        refresh_lineage: RefreshLineage,
        repair_action: BuildIntelligenceAction,
        details_action: BuildIntelligenceAction,
    ) -> Self {
        let strip_id = strip_id.into();
        let lane_type_token = lane_type.as_str().to_owned();
        let state_token = state.as_str().to_owned();
        let health_reason_token = health_reason.map(|reason| reason.as_str().to_owned());
        let imported_live_state_token = imported_live_state.as_str().to_owned();
        let summary = match health_reason_token.as_deref() {
            Some(reason) => format!(
                "lane={lane_type_token}; adapter={}; state={state_token}; reason={reason}; provenance={imported_live_state_token}",
                adapter_identity.adapter_id
            ),
            None => format!(
                "lane={lane_type_token}; adapter={}; state={state_token}; provenance={imported_live_state_token}",
                adapter_identity.adapter_id
            ),
        };
        Self {
            record_kind: ADAPTER_HEALTH_STRIP_RECORD_KIND.to_owned(),
            schema_version: BUILD_INTELLIGENCE_SCHEMA_VERSION,
            strip_id,
            workspace_id: workspace_id.into(),
            lane_type,
            lane_type_token,
            lane_type_label: lane_type.label().to_owned(),
            adapter_identity,
            state,
            state_token,
            last_successful_refresh_at: last_successful_refresh_at.into(),
            health_reason,
            health_reason_token,
            imported_live_state,
            imported_live_state_token,
            refresh_lineage,
            repair_action,
            details_action,
            continue_local_action: None,
            inspect_only_action: None,
            summary,
            redaction_safe: true,
        }
    }

    /// Adds continuation actions for partial remote or managed lanes.
    pub fn with_continuation_actions(
        mut self,
        continue_local_action: Option<BuildIntelligenceAction>,
        inspect_only_action: Option<BuildIntelligenceAction>,
    ) -> Self {
        self.continue_local_action = continue_local_action;
        self.inspect_only_action = inspect_only_action;
        self
    }
}

/// Target row carrying exactness, source actions, and imported-versus-live truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIntelligenceTargetRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Stable target id used across refreshes.
    pub stable_target_id: String,
    /// User-visible target label.
    pub display_name: String,
    /// Lane type that discovered the target.
    pub lane_type: BuildIntelligenceLaneType,
    /// Stable lane token.
    pub lane_type_token: String,
    /// Health strip ref for this target.
    pub adapter_health_strip_ref: String,
    /// Exactness status.
    pub exactness_status: TargetExactnessStatus,
    /// Stable exactness token.
    pub exactness_status_token: String,
    /// Archetype binding, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archetype_binding: Option<String>,
    /// Framework binding, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framework_binding: Option<String>,
    /// Action opening source that declared or implied this target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_source_action: Option<BuildIntelligenceAction>,
    /// Action opening relevant build or adapter configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_config_action: Option<BuildIntelligenceAction>,
    /// Provenance class distinguishing live inspection from imports.
    pub imported_live_state: ImportedLiveState,
    /// Stable imported-versus-live token.
    pub imported_live_state_token: String,
    /// Export-safe provenance note.
    pub imported_vs_live_note: String,
    /// Current workspace inspection ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_workspace_inspection_ref: Option<String>,
    /// Imported history ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_history_ref: Option<String>,
    /// Refresh lineage.
    pub refresh_lineage: RefreshLineage,
    /// Precise unresolved reason, when the target is unresolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unresolved_reason: Option<AdapterHealthReason>,
    /// Stable unresolved-reason token, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unresolved_reason_token: Option<String>,
    /// Deterministic rank used by discovery-diff review.
    pub source_confidence_rank: u8,
    /// True when the producing tool supports stable target identity.
    pub stable_identity_supported: bool,
    /// True because raw paths, command lines, env bodies, and secrets are excluded.
    pub redaction_safe: bool,
}

impl BuildIntelligenceTargetRow {
    /// Builds a target row and derives stable tokens.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        row_id: impl Into<String>,
        stable_target_id: impl Into<String>,
        display_name: impl Into<String>,
        lane_type: BuildIntelligenceLaneType,
        adapter_health_strip_ref: impl Into<String>,
        exactness_status: TargetExactnessStatus,
        imported_live_state: ImportedLiveState,
        imported_vs_live_note: impl Into<String>,
        refresh_lineage: RefreshLineage,
    ) -> Self {
        let unresolved_reason = if exactness_status == TargetExactnessStatus::Unresolved {
            Some(AdapterHealthReason::ParseAmbiguity)
        } else {
            None
        };
        let unresolved_reason_token = unresolved_reason.map(|reason| reason.as_str().to_owned());
        let source_confidence_rank = confidence_rank(lane_type, exactness_status);
        let stable_identity_supported = !matches!(
            lane_type,
            BuildIntelligenceLaneType::HeuristicFallback
                | BuildIntelligenceLaneType::StructuredOutputImport
        );
        Self {
            record_kind: BUILD_INTELLIGENCE_TARGET_ROW_RECORD_KIND.to_owned(),
            schema_version: BUILD_INTELLIGENCE_SCHEMA_VERSION,
            row_id: row_id.into(),
            stable_target_id: stable_target_id.into(),
            display_name: display_name.into(),
            lane_type,
            lane_type_token: lane_type.as_str().to_owned(),
            adapter_health_strip_ref: adapter_health_strip_ref.into(),
            exactness_status,
            exactness_status_token: exactness_status.as_str().to_owned(),
            archetype_binding: None,
            framework_binding: None,
            open_source_action: None,
            open_config_action: None,
            imported_live_state,
            imported_live_state_token: imported_live_state.as_str().to_owned(),
            imported_vs_live_note: imported_vs_live_note.into(),
            current_workspace_inspection_ref: refresh_lineage
                .current_workspace_inspection_ref
                .clone(),
            imported_history_ref: refresh_lineage.imported_history_ref.clone(),
            refresh_lineage,
            unresolved_reason,
            unresolved_reason_token,
            source_confidence_rank,
            stable_identity_supported,
            redaction_safe: true,
        }
    }

    /// Attaches known archetype and framework bindings.
    pub fn with_bindings(
        mut self,
        archetype_binding: impl Into<Option<String>>,
        framework_binding: impl Into<Option<String>>,
    ) -> Self {
        self.archetype_binding = archetype_binding.into();
        self.framework_binding = framework_binding.into();
        self
    }

    /// Attaches source and config actions.
    pub fn with_actions(
        mut self,
        open_source_action: Option<BuildIntelligenceAction>,
        open_config_action: Option<BuildIntelligenceAction>,
    ) -> Self {
        self.open_source_action = open_source_action;
        self.open_config_action = open_config_action;
        self
    }

    /// Overrides the unresolved reason for an unresolved row.
    pub fn with_unresolved_reason(mut self, reason: AdapterHealthReason) -> Self {
        self.unresolved_reason = Some(reason);
        self.unresolved_reason_token = Some(reason.as_str().to_owned());
        self
    }
}

/// Run-configuration card projected from a build-intelligence target row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIntelligenceRunConfigCard {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable card id.
    pub card_id: String,
    /// Command id that would dispatch the run configuration.
    pub command_id: String,
    /// Target row ref.
    pub target_row_ref: String,
    /// Stable target id.
    pub stable_target_id: String,
    /// User-visible target label.
    pub display_name: String,
    /// Lane token copied from the target row.
    pub lane_type_token: String,
    /// Exactness token copied from the target row.
    pub exactness_status_token: String,
    /// Provenance token copied from the target row.
    pub imported_live_state_token: String,
    /// Export-safe imported-versus-live note.
    pub imported_vs_live_note: String,
    /// Open-source action copied from the target row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_source_action: Option<BuildIntelligenceAction>,
    /// Open-config action copied from the target row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_config_action: Option<BuildIntelligenceAction>,
    /// High-trust action posture for dispatch/rerun/publish decisions.
    pub high_trust_action_posture: HighTrustActionPosture,
    /// Stable high-trust posture token.
    pub high_trust_action_posture_token: String,
    /// True because raw paths, command lines, env bodies, and secrets are excluded.
    pub redaction_safe: bool,
}

impl BuildIntelligenceRunConfigCard {
    /// Builds a run-configuration card from a target row.
    pub fn from_target_row(
        card_id: impl Into<String>,
        command_id: impl Into<String>,
        row: &BuildIntelligenceTargetRow,
        high_trust_action_posture: HighTrustActionPosture,
    ) -> Self {
        Self {
            record_kind: BUILD_INTELLIGENCE_RUN_CONFIG_CARD_RECORD_KIND.to_owned(),
            schema_version: BUILD_INTELLIGENCE_SCHEMA_VERSION,
            card_id: card_id.into(),
            command_id: command_id.into(),
            target_row_ref: row.row_id.clone(),
            stable_target_id: row.stable_target_id.clone(),
            display_name: row.display_name.clone(),
            lane_type_token: row.lane_type_token.clone(),
            exactness_status_token: row.exactness_status_token.clone(),
            imported_live_state_token: row.imported_live_state_token.clone(),
            imported_vs_live_note: row.imported_vs_live_note.clone(),
            open_source_action: row.open_source_action.clone(),
            open_config_action: row.open_config_action.clone(),
            high_trust_action_posture,
            high_trust_action_posture_token: high_trust_action_posture.as_str().to_owned(),
            redaction_safe: true,
        }
    }
}

/// Build summary and receipt object for one completed or imported run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIntelligenceReceipt {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable receipt id.
    pub receipt_id: String,
    /// Command id that produced or imported the receipt.
    pub command_id: String,
    /// Run id or imported-run id.
    pub run_id: String,
    /// Target row ref.
    pub target_row_ref: String,
    /// Adapter health strip ref.
    pub adapter_health_strip_ref: String,
    /// Stable target id.
    pub stable_target_id: String,
    /// Target label at receipt time.
    pub target_display_name: String,
    /// Lane type.
    pub lane_type: BuildIntelligenceLaneType,
    /// Stable lane token.
    pub lane_type_token: String,
    /// Environment or host label.
    pub environment_or_host: String,
    /// Artifact source class.
    pub artifact_source: ArtifactSourceClass,
    /// Stable artifact-source token.
    pub artifact_source_token: String,
    /// Artifact ref, when an artifact exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_ref: Option<String>,
    /// Imported or replayed note, explicit even when live.
    pub imported_or_replayed_note: String,
    /// Imported-versus-live provenance class.
    pub imported_live_state: ImportedLiveState,
    /// Stable imported-versus-live token.
    pub imported_live_state_token: String,
    /// Refresh lineage used by the run or import.
    pub refresh_lineage: RefreshLineage,
    /// High-trust action posture preserved for rerun/export/publish.
    pub high_trust_action_posture: HighTrustActionPosture,
    /// Stable high-trust posture token.
    pub high_trust_action_posture_token: String,
    /// True when the receipt is safe to replay for inspect/support without dispatch.
    pub support_replay_safe: bool,
    /// True because raw paths, command lines, env bodies, and secrets are excluded.
    pub redaction_safe: bool,
}

impl BuildIntelligenceReceipt {
    /// Builds a receipt from a target row and health strip.
    #[allow(clippy::too_many_arguments)]
    pub fn from_target_row(
        receipt_id: impl Into<String>,
        command_id: impl Into<String>,
        run_id: impl Into<String>,
        row: &BuildIntelligenceTargetRow,
        strip: &AdapterHealthStrip,
        environment_or_host: impl Into<String>,
        artifact_source: ArtifactSourceClass,
        artifact_ref: impl Into<Option<String>>,
        imported_or_replayed_note: impl Into<String>,
        high_trust_action_posture: HighTrustActionPosture,
    ) -> Self {
        Self {
            record_kind: BUILD_INTELLIGENCE_RECEIPT_RECORD_KIND.to_owned(),
            schema_version: BUILD_INTELLIGENCE_SCHEMA_VERSION,
            receipt_id: receipt_id.into(),
            command_id: command_id.into(),
            run_id: run_id.into(),
            target_row_ref: row.row_id.clone(),
            adapter_health_strip_ref: strip.strip_id.clone(),
            stable_target_id: row.stable_target_id.clone(),
            target_display_name: row.display_name.clone(),
            lane_type: row.lane_type,
            lane_type_token: row.lane_type_token.clone(),
            environment_or_host: environment_or_host.into(),
            artifact_source,
            artifact_source_token: artifact_source.as_str().to_owned(),
            artifact_ref: artifact_ref.into(),
            imported_or_replayed_note: imported_or_replayed_note.into(),
            imported_live_state: row.imported_live_state,
            imported_live_state_token: row.imported_live_state_token.clone(),
            refresh_lineage: row.refresh_lineage.clone(),
            high_trust_action_posture,
            high_trust_action_posture_token: high_trust_action_posture.as_str().to_owned(),
            support_replay_safe: true,
            redaction_safe: true,
        }
    }
}

/// One item in a discovery-diff review bucket.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveryDiffItem {
    /// Change class.
    pub change_class: DiscoveryDiffChangeClass,
    /// Stable change token.
    pub change_token: String,
    /// Previous stable target id, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_target_id: Option<String>,
    /// Current stable target id, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_target_id: Option<String>,
    /// Previous display name, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<String>,
    /// Current display name, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_name: Option<String>,
    /// Previous lane token, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_lane_type_token: Option<String>,
    /// Current lane token, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_lane_type_token: Option<String>,
    /// Previous exactness token, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_exactness_status_token: Option<String>,
    /// Current exactness token, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_exactness_status_token: Option<String>,
    /// Review action ref.
    pub action_ref: String,
    /// Export-safe summary.
    pub summary: String,
}

impl DiscoveryDiffItem {
    fn for_change(
        change_class: DiscoveryDiffChangeClass,
        previous: Option<&BuildIntelligenceTargetRow>,
        current: Option<&BuildIntelligenceTargetRow>,
    ) -> Self {
        let change_token = change_class.as_str().to_owned();
        let previous_target_id = previous.map(|row| row.stable_target_id.clone());
        let current_target_id = current.map(|row| row.stable_target_id.clone());
        let target_for_action = current
            .or(previous)
            .map(|row| stable_token(&row.stable_target_id))
            .unwrap_or_else(|| "unknown".to_owned());
        let action_ref =
            format!("action:build-intelligence:review-diff:{change_token}:{target_for_action}");
        let previous_name = previous.map(|row| row.display_name.clone());
        let current_name = current.map(|row| row.display_name.clone());
        let summary = diff_summary(change_class, previous, current);
        Self {
            change_class,
            change_token,
            previous_target_id,
            current_target_id,
            previous_name,
            current_name,
            previous_lane_type_token: previous.map(|row| row.lane_type_token.clone()),
            current_lane_type_token: current.map(|row| row.lane_type_token.clone()),
            previous_exactness_status_token: previous.map(|row| row.exactness_status_token.clone()),
            current_exactness_status_token: current.map(|row| row.exactness_status_token.clone()),
            action_ref,
            summary,
        }
    }
}

/// Discovery-diff review produced after an adapter or config refresh.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveryDiffReview {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable diff id.
    pub diff_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Previous refresh id.
    pub previous_refresh_id: String,
    /// Current refresh id.
    pub current_refresh_id: String,
    /// Review timestamp.
    pub reviewed_at: String,
    /// Added targets.
    pub added: Vec<DiscoveryDiffItem>,
    /// Removed targets.
    pub removed: Vec<DiscoveryDiffItem>,
    /// Renamed targets.
    pub renamed: Vec<DiscoveryDiffItem>,
    /// Confidence downgrades.
    pub downgraded_confidence: Vec<DiscoveryDiffItem>,
    /// Targets that became heuristic.
    pub newly_heuristic: Vec<DiscoveryDiffItem>,
    /// Targets that became exact.
    pub newly_exact: Vec<DiscoveryDiffItem>,
    /// Targets that became unresolved.
    pub now_unresolved: Vec<DiscoveryDiffItem>,
    /// Export-safe summary line.
    pub summary: String,
    /// True because raw paths, command lines, env bodies, and secrets are excluded.
    pub redaction_safe: bool,
}

impl DiscoveryDiffReview {
    /// Builds a deterministic discovery-diff review from two target snapshots.
    #[allow(clippy::too_many_arguments)]
    pub fn between(
        diff_id: impl Into<String>,
        workspace_id: impl Into<String>,
        previous_refresh_id: impl Into<String>,
        current_refresh_id: impl Into<String>,
        reviewed_at: impl Into<String>,
        previous_rows: &[BuildIntelligenceTargetRow],
        current_rows: &[BuildIntelligenceTargetRow],
    ) -> Self {
        let diff_id = diff_id.into();
        let previous_by_id = rows_by_target_id(previous_rows);
        let current_by_id = rows_by_target_id(current_rows);
        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut renamed = Vec::new();
        let mut downgraded_confidence = Vec::new();
        let mut newly_heuristic = Vec::new();
        let mut newly_exact = Vec::new();
        let mut now_unresolved = Vec::new();

        for (target_id, current) in &current_by_id {
            match previous_by_id.get(target_id) {
                None => added.push(DiscoveryDiffItem::for_change(
                    DiscoveryDiffChangeClass::Added,
                    None,
                    Some(current),
                )),
                Some(previous) => {
                    if previous.display_name != current.display_name {
                        renamed.push(DiscoveryDiffItem::for_change(
                            DiscoveryDiffChangeClass::Renamed,
                            Some(previous),
                            Some(current),
                        ));
                    }
                    if current.source_confidence_rank < previous.source_confidence_rank {
                        downgraded_confidence.push(DiscoveryDiffItem::for_change(
                            DiscoveryDiffChangeClass::DowngradedConfidence,
                            Some(previous),
                            Some(current),
                        ));
                    }
                    if previous.exactness_status != TargetExactnessStatus::Heuristic
                        && current.exactness_status == TargetExactnessStatus::Heuristic
                    {
                        newly_heuristic.push(DiscoveryDiffItem::for_change(
                            DiscoveryDiffChangeClass::NewlyHeuristic,
                            Some(previous),
                            Some(current),
                        ));
                    }
                    if previous.exactness_status != TargetExactnessStatus::Exact
                        && current.exactness_status == TargetExactnessStatus::Exact
                    {
                        newly_exact.push(DiscoveryDiffItem::for_change(
                            DiscoveryDiffChangeClass::NewlyExact,
                            Some(previous),
                            Some(current),
                        ));
                    }
                    if previous.exactness_status != TargetExactnessStatus::Unresolved
                        && current.exactness_status == TargetExactnessStatus::Unresolved
                    {
                        now_unresolved.push(DiscoveryDiffItem::for_change(
                            DiscoveryDiffChangeClass::NowUnresolved,
                            Some(previous),
                            Some(current),
                        ));
                    }
                }
            }
        }

        for (target_id, previous) in &previous_by_id {
            if !current_by_id.contains_key(target_id) {
                removed.push(DiscoveryDiffItem::for_change(
                    DiscoveryDiffChangeClass::Removed,
                    Some(previous),
                    None,
                ));
            }
        }

        let summary = format!(
            "added={}; removed={}; renamed={}; downgraded_confidence={}; newly_heuristic={}; newly_exact={}; now_unresolved={}",
            added.len(),
            removed.len(),
            renamed.len(),
            downgraded_confidence.len(),
            newly_heuristic.len(),
            newly_exact.len(),
            now_unresolved.len(),
        );

        Self {
            record_kind: DISCOVERY_DIFF_REVIEW_RECORD_KIND.to_owned(),
            schema_version: BUILD_INTELLIGENCE_SCHEMA_VERSION,
            diff_id,
            workspace_id: workspace_id.into(),
            previous_refresh_id: previous_refresh_id.into(),
            current_refresh_id: current_refresh_id.into(),
            reviewed_at: reviewed_at.into(),
            added,
            removed,
            renamed,
            downgraded_confidence,
            newly_heuristic,
            newly_exact,
            now_unresolved,
            summary,
            redaction_safe: true,
        }
    }

    /// Returns true when any diff bucket is non-empty.
    pub fn has_changes(&self) -> bool {
        !self.added.is_empty()
            || !self.removed.is_empty()
            || !self.renamed.is_empty()
            || !self.downgraded_confidence.is_empty()
            || !self.newly_heuristic.is_empty()
            || !self.newly_exact.is_empty()
            || !self.now_unresolved.is_empty()
    }
}

/// Coverage manifest pinning build-intelligence closed vocabularies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIntelligenceCoverageManifest {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Manifest id.
    pub manifest_id: String,
    /// Manifest timestamp.
    pub generated_at: String,
    /// Lane-type tokens.
    pub lane_type_tokens: Vec<String>,
    /// Health-state tokens.
    pub adapter_health_state_tokens: Vec<String>,
    /// Health-reason tokens.
    pub adapter_health_reason_tokens: Vec<String>,
    /// Action-class tokens.
    pub action_class_tokens: Vec<String>,
    /// Imported-versus-live tokens.
    pub imported_live_state_tokens: Vec<String>,
    /// Target-exactness tokens.
    pub target_exactness_status_tokens: Vec<String>,
    /// Artifact-source tokens.
    pub artifact_source_tokens: Vec<String>,
    /// High-trust posture tokens.
    pub high_trust_action_posture_tokens: Vec<String>,
    /// Discovery-diff change tokens.
    pub discovery_diff_change_tokens: Vec<String>,
}

impl BuildIntelligenceCoverageManifest {
    /// Builds the canonical coverage manifest.
    pub fn canonical(manifest_id: impl Into<String>, generated_at: impl Into<String>) -> Self {
        Self {
            record_kind: BUILD_INTELLIGENCE_COVERAGE_MANIFEST_RECORD_KIND.to_owned(),
            schema_version: BUILD_INTELLIGENCE_SCHEMA_VERSION,
            manifest_id: manifest_id.into(),
            generated_at: generated_at.into(),
            lane_type_tokens: BuildIntelligenceLaneType::ALL
                .iter()
                .map(|lane| lane.as_str().to_owned())
                .collect(),
            adapter_health_state_tokens: AdapterHealthState::ALL
                .iter()
                .map(|state| state.as_str().to_owned())
                .collect(),
            adapter_health_reason_tokens: AdapterHealthReason::ALL
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect(),
            action_class_tokens: BuildIntelligenceActionClass::ALL
                .iter()
                .map(|action| action.as_str().to_owned())
                .collect(),
            imported_live_state_tokens: ImportedLiveState::ALL
                .iter()
                .map(|state| state.as_str().to_owned())
                .collect(),
            target_exactness_status_tokens: TargetExactnessStatus::ALL
                .iter()
                .map(|status| status.as_str().to_owned())
                .collect(),
            artifact_source_tokens: ArtifactSourceClass::ALL
                .iter()
                .map(|source| source.as_str().to_owned())
                .collect(),
            high_trust_action_posture_tokens: HighTrustActionPosture::ALL
                .iter()
                .map(|posture| posture.as_str().to_owned())
                .collect(),
            discovery_diff_change_tokens: DiscoveryDiffChangeClass::ALL
                .iter()
                .map(|change| change.as_str().to_owned())
                .collect(),
        }
    }
}

/// Support/export packet for build-intelligence surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIntelligenceSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Support-export id.
    pub support_export_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Coverage manifest at export time.
    pub coverage_manifest: BuildIntelligenceCoverageManifest,
    /// Adapter-health strips.
    pub adapter_health_strips: Vec<AdapterHealthStrip>,
    /// Target rows.
    pub target_rows: Vec<BuildIntelligenceTargetRow>,
    /// Run-configuration cards.
    pub run_config_cards: Vec<BuildIntelligenceRunConfigCard>,
    /// Build summary receipts.
    pub receipts: Vec<BuildIntelligenceReceipt>,
    /// Discovery-diff reviews.
    pub discovery_diffs: Vec<DiscoveryDiffReview>,
    /// True because raw paths, command lines, env bodies, and secrets are excluded.
    pub redaction_safe: bool,
}

impl BuildIntelligenceSupportExport {
    /// Builds a support-export packet from already-minted rows.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        support_export_id: impl Into<String>,
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        adapter_health_strips: Vec<AdapterHealthStrip>,
        target_rows: Vec<BuildIntelligenceTargetRow>,
        run_config_cards: Vec<BuildIntelligenceRunConfigCard>,
        receipts: Vec<BuildIntelligenceReceipt>,
        discovery_diffs: Vec<DiscoveryDiffReview>,
    ) -> Self {
        let support_export_id = support_export_id.into();
        let generated_at = generated_at.into();
        Self {
            record_kind: BUILD_INTELLIGENCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: BUILD_INTELLIGENCE_SCHEMA_VERSION,
            support_export_id: support_export_id.clone(),
            workspace_id: workspace_id.into(),
            coverage_manifest: BuildIntelligenceCoverageManifest::canonical(
                format!("{support_export_id}:coverage"),
                generated_at.clone(),
            ),
            generated_at,
            adapter_health_strips,
            target_rows,
            run_config_cards,
            receipts,
            discovery_diffs,
            redaction_safe: true,
        }
    }

    /// Renders deterministic plaintext for CLI/headless and support review.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!(
            "Build intelligence support export: {}\nWorkspace: {}\nGenerated: {}\n",
            self.support_export_id, self.workspace_id, self.generated_at
        );
        out.push_str("[Adapter health]\n");
        for strip in &self.adapter_health_strips {
            out.push_str(&format!(
                "- strip={}; lane={}; adapter={}; state={}; reason={}; last_success={}; provenance={}; repair={}; details={}\n",
                strip.strip_id,
                strip.lane_type_token,
                strip.adapter_identity.adapter_id,
                strip.state_token,
                strip.health_reason_token.as_deref().unwrap_or("none"),
                strip.last_successful_refresh_at.as_deref().unwrap_or("none"),
                strip.imported_live_state_token,
                strip.repair_action.action_ref,
                strip.details_action.action_ref,
            ));
            if let Some(action) = &strip.continue_local_action {
                out.push_str(&format!("  continue_local={}\n", action.action_ref));
            }
            if let Some(action) = &strip.inspect_only_action {
                out.push_str(&format!("  inspect_only={}\n", action.action_ref));
            }
        }
        out.push_str("[Targets]\n");
        for row in &self.target_rows {
            out.push_str(&format!(
                "- row={}; target={}({}); lane={}; exactness={}; archetype={}; framework={}; provenance={}; note={}\n",
                row.row_id,
                row.stable_target_id,
                row.display_name,
                row.lane_type_token,
                row.exactness_status_token,
                row.archetype_binding.as_deref().unwrap_or("unknown"),
                row.framework_binding.as_deref().unwrap_or("unknown"),
                row.imported_live_state_token,
                row.imported_vs_live_note,
            ));
        }
        out.push_str("[Run configs]\n");
        for card in &self.run_config_cards {
            out.push_str(&format!(
                "- card={}; command={}; target={}; lane={}; exactness={}; posture={}; note={}\n",
                card.card_id,
                card.command_id,
                card.stable_target_id,
                card.lane_type_token,
                card.exactness_status_token,
                card.high_trust_action_posture_token,
                card.imported_vs_live_note,
            ));
        }
        out.push_str("[Receipts]\n");
        for receipt in &self.receipts {
            out.push_str(&format!(
                "- receipt={}; command={}; run={}; target={}; lane={}; host={}; artifact_source={}; provenance={}; posture={}; note={}\n",
                receipt.receipt_id,
                receipt.command_id,
                receipt.run_id,
                receipt.stable_target_id,
                receipt.lane_type_token,
                receipt.environment_or_host,
                receipt.artifact_source_token,
                receipt.imported_live_state_token,
                receipt.high_trust_action_posture_token,
                receipt.imported_or_replayed_note,
            ));
        }
        out.push_str("[Discovery diffs]\n");
        for diff in &self.discovery_diffs {
            out.push_str(&format!(
                "- diff={}; previous={}; current={}; {}\n",
                diff.diff_id, diff.previous_refresh_id, diff.current_refresh_id, diff.summary
            ));
        }
        out
    }
}

/// Builds the stable adapter-confidence fixture export used by tooling,
/// runtime, shell, and support surfaces.
pub fn current_stable_adapter_confidence_support_export() -> BuildIntelligenceSupportExport {
    fn action(class: BuildIntelligenceActionClass, suffix: &str) -> BuildIntelligenceAction {
        BuildIntelligenceAction::enabled(class, format!("action:adapter-confidence:{suffix}"))
    }

    fn lineage(
        refresh: &str,
        previous: Option<&str>,
        live_ref: Option<&str>,
        import_ref: Option<&str>,
    ) -> RefreshLineage {
        RefreshLineage::new(
            refresh.to_owned(),
            previous.map(str::to_owned),
            "2026-05-18T15:00:00Z",
            Some("2026-05-18T15:00:05Z".to_owned()),
        )
        .with_refs(
            Some(format!("snapshot:{refresh}")),
            Some(format!("raw:{refresh}")),
            live_ref.map(str::to_owned),
            import_ref.map(str::to_owned),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn strip(
        id: &str,
        label: &str,
        lane: BuildIntelligenceLaneType,
        state: AdapterHealthState,
        reason: Option<AdapterHealthReason>,
        provenance: ImportedLiveState,
        live_ref: Option<&str>,
        import_ref: Option<&str>,
    ) -> AdapterHealthStrip {
        let protocol_id = match lane {
            BuildIntelligenceLaneType::StructuredProtocol => Some("bsp".to_owned()),
            BuildIntelligenceLaneType::BuildEventStream => Some("bep".to_owned()),
            _ => None,
        };
        AdapterHealthStrip::new(
            format!("strip:{id}"),
            "workspace:stable-build-intelligence",
            lane,
            AdapterIdentity::new(
                format!("adapter:{id}"),
                label.to_owned(),
                Some("1.0".to_owned()),
                protocol_id,
                Some("1".to_owned()),
            ),
            state,
            reason,
            Some("2026-05-18T15:00:05Z".to_owned()),
            provenance,
            lineage(
                "refresh:current",
                Some("refresh:previous"),
                live_ref,
                import_ref,
            ),
            action(
                BuildIntelligenceActionClass::RefreshDiscovery,
                &format!("{id}:refresh"),
            ),
            action(
                BuildIntelligenceActionClass::OpenDetails,
                &format!("{id}:details"),
            ),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn target(
        refresh: &str,
        id: &str,
        name: &str,
        lane: BuildIntelligenceLaneType,
        exactness: TargetExactnessStatus,
        strip_ref: &str,
        provenance: ImportedLiveState,
        note: &str,
    ) -> BuildIntelligenceTargetRow {
        let live_ref = matches!(
            provenance,
            ImportedLiveState::LiveWorkspaceInspection | ImportedLiveState::MixedLiveAndImported
        )
        .then_some("inspection:current");
        let import_ref = matches!(
            provenance,
            ImportedLiveState::ImportedArtifact
                | ImportedLiveState::ReplayedReceipt
                | ImportedLiveState::MixedLiveAndImported
        )
        .then_some("artifact:imported");
        BuildIntelligenceTargetRow::new(
            format!("row:{id}:{refresh}"),
            id,
            name,
            lane,
            strip_ref,
            exactness,
            provenance,
            note,
            lineage(refresh, Some("refresh:previous"), live_ref, import_ref),
        )
        .with_bindings(
            Some("typescript_web_app".to_owned()),
            Some("vite".to_owned()),
        )
        .with_actions(
            Some(action(
                BuildIntelligenceActionClass::OpenSource,
                &format!("{id}:source"),
            )),
            Some(action(
                BuildIntelligenceActionClass::OpenConfig,
                &format!("{id}:config"),
            )),
        )
    }

    let native_strip = strip(
        "native",
        "Native Cargo adapter",
        BuildIntelligenceLaneType::NativeAdapter,
        AdapterHealthState::Healthy,
        None,
        ImportedLiveState::LiveWorkspaceInspection,
        Some("inspection:native"),
        None,
    );
    let protocol_strip = strip(
        "protocol",
        "Build server protocol adapter",
        BuildIntelligenceLaneType::StructuredProtocol,
        AdapterHealthState::Partial,
        Some(AdapterHealthReason::VersionSkew),
        ImportedLiveState::MixedLiveAndImported,
        Some("inspection:protocol"),
        Some("artifact:protocol:previous"),
    )
    .with_continuation_actions(
        Some(action(
            BuildIntelligenceActionClass::ContinueLocal,
            "protocol:continue-local",
        )),
        Some(action(
            BuildIntelligenceActionClass::InspectOnly,
            "protocol:inspect-only",
        )),
    );
    let event_strip = strip(
        "event",
        "Build-event stream adapter",
        BuildIntelligenceLaneType::BuildEventStream,
        AdapterHealthState::ImportedOnly,
        Some(AdapterHealthReason::ControlPlaneOutage),
        ImportedLiveState::ReplayedReceipt,
        None,
        Some("receipt:bep:51"),
    );
    let import_strip = strip(
        "import",
        "Structured output importer",
        BuildIntelligenceLaneType::StructuredOutputImport,
        AdapterHealthState::ImportedOnly,
        Some(AdapterHealthReason::StaleArtifact),
        ImportedLiveState::ImportedArtifact,
        None,
        Some("artifact:junit:51"),
    );
    let heuristic_strip = strip(
        "heuristic",
        "Heuristic target fallback",
        BuildIntelligenceLaneType::HeuristicFallback,
        AdapterHealthState::Degraded,
        Some(AdapterHealthReason::ParseAmbiguity),
        ImportedLiveState::HeuristicInference,
        Some("inspection:heuristic"),
        None,
    );

    let previous_rows = vec![
        target(
            "refresh:previous",
            "target:web",
            "web",
            BuildIntelligenceLaneType::NativeAdapter,
            TargetExactnessStatus::Exact,
            &native_strip.strip_id,
            ImportedLiveState::LiveWorkspaceInspection,
            "live native adapter target",
        ),
        target(
            "refresh:previous",
            "target:api",
            "api",
            BuildIntelligenceLaneType::StructuredProtocol,
            TargetExactnessStatus::ProtocolBacked,
            &protocol_strip.strip_id,
            ImportedLiveState::LiveWorkspaceInspection,
            "protocol-backed target",
        ),
        target(
            "refresh:previous",
            "target:legacy",
            "legacy",
            BuildIntelligenceLaneType::HeuristicFallback,
            TargetExactnessStatus::Heuristic,
            &heuristic_strip.strip_id,
            ImportedLiveState::HeuristicInference,
            "heuristic target before adapter support landed",
        ),
        target(
            "refresh:previous",
            "target:removed",
            "removed",
            BuildIntelligenceLaneType::StructuredProtocol,
            TargetExactnessStatus::ProtocolBacked,
            &protocol_strip.strip_id,
            ImportedLiveState::LiveWorkspaceInspection,
            "removed after refresh",
        ),
    ];
    let current_rows = vec![
        target(
            "refresh:current",
            "target:web",
            "web-test",
            BuildIntelligenceLaneType::HeuristicFallback,
            TargetExactnessStatus::Heuristic,
            &heuristic_strip.strip_id,
            ImportedLiveState::HeuristicInference,
            "heuristic fallback; review before rerun",
        ),
        target(
            "refresh:current",
            "target:api",
            "api",
            BuildIntelligenceLaneType::StructuredProtocol,
            TargetExactnessStatus::Unresolved,
            &protocol_strip.strip_id,
            ImportedLiveState::MixedLiveAndImported,
            "protocol target unresolved after version skew",
        )
        .with_unresolved_reason(AdapterHealthReason::VersionSkew),
        target(
            "refresh:current",
            "target:legacy",
            "legacy",
            BuildIntelligenceLaneType::NativeAdapter,
            TargetExactnessStatus::Exact,
            &native_strip.strip_id,
            ImportedLiveState::LiveWorkspaceInspection,
            "now exact through native adapter",
        ),
        target(
            "refresh:current",
            "target:bep",
            "bep imported test",
            BuildIntelligenceLaneType::BuildEventStream,
            TargetExactnessStatus::Imported,
            &event_strip.strip_id,
            ImportedLiveState::ReplayedReceipt,
            "replayed build-event stream; inspect only until live refresh",
        ),
        target(
            "refresh:current",
            "target:junit",
            "junit import",
            BuildIntelligenceLaneType::StructuredOutputImport,
            TargetExactnessStatus::Imported,
            &import_strip.strip_id,
            ImportedLiveState::ImportedArtifact,
            "structured output import; refresh before rerun",
        ),
    ];
    let cards = current_rows
        .iter()
        .map(|row| {
            let posture = match row.imported_live_state {
                ImportedLiveState::LiveWorkspaceInspection => {
                    HighTrustActionPosture::LiveActionsAllowed
                }
                ImportedLiveState::MixedLiveAndImported => {
                    HighTrustActionPosture::ContinueLocalAvailable
                }
                ImportedLiveState::ImportedArtifact => HighTrustActionPosture::RefreshRequired,
                ImportedLiveState::ReplayedReceipt => HighTrustActionPosture::InspectOnly,
                ImportedLiveState::HeuristicInference => {
                    HighTrustActionPosture::ReviewBeforeDispatch
                }
            };
            BuildIntelligenceRunConfigCard::from_target_row(
                format!("card:{}", row.stable_target_id),
                "task.run",
                row,
                posture,
            )
        })
        .collect::<Vec<_>>();
    let receipts = vec![
        BuildIntelligenceReceipt::from_target_row(
            "receipt:legacy",
            "task.run",
            "run:legacy:1",
            &current_rows[2],
            &native_strip,
            "local/macos",
            ArtifactSourceClass::LiveAdapter,
            Some("artifact:legacy:1".to_owned()),
            "live native adapter result",
            HighTrustActionPosture::LiveActionsAllowed,
        ),
        BuildIntelligenceReceipt::from_target_row(
            "receipt:bep",
            "test.run",
            "run:bep:51",
            &current_rows[3],
            &event_strip,
            "ci/linux",
            ArtifactSourceClass::ReplayedReceipt,
            Some("artifact:bep:51".to_owned()),
            "replayed build-event stream; not current live discovery",
            HighTrustActionPosture::InspectOnly,
        ),
        BuildIntelligenceReceipt::from_target_row(
            "receipt:junit",
            "test.run",
            "run:junit:51",
            &current_rows[4],
            &import_strip,
            "ci/linux",
            ArtifactSourceClass::StructuredImport,
            Some("artifact:junit:51".to_owned()),
            "imported structured output; refresh required before rerun",
            HighTrustActionPosture::RefreshRequired,
        ),
    ];
    let diff = DiscoveryDiffReview::between(
        "diff:all-lanes",
        "workspace:stable-build-intelligence",
        "refresh:previous",
        "refresh:current",
        "2026-05-18T15:01:00Z",
        &previous_rows,
        &current_rows,
    );
    BuildIntelligenceSupportExport::new(
        "support:stable-build-intelligence",
        "workspace:stable-build-intelligence",
        "2026-05-18T15:02:00Z",
        vec![
            native_strip,
            protocol_strip,
            event_strip,
            import_strip,
            heuristic_strip,
        ],
        current_rows,
        cards,
        receipts,
        vec![diff],
    )
}

fn rows_by_target_id(
    rows: &[BuildIntelligenceTargetRow],
) -> BTreeMap<String, &BuildIntelligenceTargetRow> {
    rows.iter()
        .map(|row| (row.stable_target_id.clone(), row))
        .collect()
}

fn confidence_rank(lane: BuildIntelligenceLaneType, exactness: TargetExactnessStatus) -> u8 {
    let lane_rank = match lane {
        BuildIntelligenceLaneType::NativeAdapter => 50,
        BuildIntelligenceLaneType::StructuredProtocol => 45,
        BuildIntelligenceLaneType::BuildEventStream => 40,
        BuildIntelligenceLaneType::StructuredOutputImport => 30,
        BuildIntelligenceLaneType::HeuristicFallback => 10,
    };
    let exactness_rank = match exactness {
        TargetExactnessStatus::Exact => 10,
        TargetExactnessStatus::ProtocolBacked => 8,
        TargetExactnessStatus::Imported => 5,
        TargetExactnessStatus::Heuristic => 2,
        TargetExactnessStatus::Unresolved => 0,
    };
    lane_rank + exactness_rank
}

fn diff_summary(
    change_class: DiscoveryDiffChangeClass,
    previous: Option<&BuildIntelligenceTargetRow>,
    current: Option<&BuildIntelligenceTargetRow>,
) -> String {
    match change_class {
        DiscoveryDiffChangeClass::Added => format!(
            "Target {} was added via {}.",
            current
                .map(|row| row.stable_target_id.as_str())
                .unwrap_or("unknown"),
            current
                .map(|row| row.lane_type_token.as_str())
                .unwrap_or("unknown"),
        ),
        DiscoveryDiffChangeClass::Removed => format!(
            "Target {} was removed from refreshed discovery.",
            previous
                .map(|row| row.stable_target_id.as_str())
                .unwrap_or("unknown"),
        ),
        DiscoveryDiffChangeClass::Renamed => format!(
            "Target {} was renamed from {} to {}.",
            current
                .map(|row| row.stable_target_id.as_str())
                .unwrap_or("unknown"),
            previous
                .map(|row| row.display_name.as_str())
                .unwrap_or("unknown"),
            current
                .map(|row| row.display_name.as_str())
                .unwrap_or("unknown"),
        ),
        DiscoveryDiffChangeClass::DowngradedConfidence => format!(
            "Target {} downgraded from {} to {}.",
            current
                .map(|row| row.stable_target_id.as_str())
                .unwrap_or("unknown"),
            previous
                .map(|row| row.exactness_status_token.as_str())
                .unwrap_or("unknown"),
            current
                .map(|row| row.exactness_status_token.as_str())
                .unwrap_or("unknown"),
        ),
        DiscoveryDiffChangeClass::NewlyHeuristic => format!(
            "Target {} is now heuristic and must be reviewed before dispatch.",
            current
                .map(|row| row.stable_target_id.as_str())
                .unwrap_or("unknown"),
        ),
        DiscoveryDiffChangeClass::NewlyExact => format!(
            "Target {} is now exact after refresh.",
            current
                .map(|row| row.stable_target_id.as_str())
                .unwrap_or("unknown"),
        ),
        DiscoveryDiffChangeClass::NowUnresolved => format!(
            "Target {} is now unresolved after refresh.",
            current
                .map(|row| row.stable_target_id.as_str())
                .unwrap_or("unknown"),
        ),
    }
}

fn stable_token(raw: &str) -> String {
    let mut token = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            token.push(ch.to_ascii_lowercase());
        } else if !token.ends_with('_') {
            token.push('_');
        }
    }
    let token = token.trim_matches('_').to_owned();
    if token.is_empty() {
        "unnamed".to_owned()
    } else {
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn action(class: BuildIntelligenceActionClass, suffix: &str) -> BuildIntelligenceAction {
        BuildIntelligenceAction::enabled(class, format!("action:test:{suffix}"))
    }

    fn lineage(refresh: &str, previous: Option<&str>) -> RefreshLineage {
        RefreshLineage::new(
            refresh.to_owned(),
            previous.map(str::to_owned),
            "2026-05-18T12:00:00Z",
            Some("2026-05-18T12:00:05Z".to_owned()),
        )
        .with_refs(
            Some(format!("snapshot:{refresh}")),
            Some(format!("raw:{refresh}")),
            Some(format!("inspection:{refresh}")),
            None,
        )
    }

    fn strip(id: &str, lane: BuildIntelligenceLaneType) -> AdapterHealthStrip {
        AdapterHealthStrip::new(
            id,
            "workspace:test",
            lane,
            AdapterIdentity::new(
                format!("adapter:{id}"),
                "Adapter",
                Some("1.0".to_owned()),
                None,
                None,
            ),
            AdapterHealthState::Healthy,
            None,
            Some("2026-05-18T12:00:05Z".to_owned()),
            ImportedLiveState::LiveWorkspaceInspection,
            lineage("refresh:current", Some("refresh:previous")),
            action(BuildIntelligenceActionClass::RefreshDiscovery, "refresh"),
            action(BuildIntelligenceActionClass::OpenDetails, "details"),
        )
    }

    fn row(
        id: &str,
        name: &str,
        lane: BuildIntelligenceLaneType,
        exactness: TargetExactnessStatus,
        refresh: &str,
    ) -> BuildIntelligenceTargetRow {
        BuildIntelligenceTargetRow::new(
            format!("row:{id}:{refresh}"),
            id,
            name,
            lane,
            format!("strip:{refresh}"),
            exactness,
            ImportedLiveState::LiveWorkspaceInspection,
            "current live workspace inspection",
            lineage(refresh, Some("refresh:previous")),
        )
    }

    #[test]
    fn coverage_manifest_pins_closed_vocabularies() {
        let manifest = BuildIntelligenceCoverageManifest::canonical(
            "manifest:build-intelligence",
            "2026-05-18T12:00:00Z",
        );
        assert_eq!(
            manifest.lane_type_tokens,
            vec![
                "native_adapter",
                "structured_protocol",
                "build_event_stream",
                "structured_output_import",
                "heuristic_fallback"
            ]
        );
        assert!(manifest
            .adapter_health_reason_tokens
            .contains(&"transport_failure".to_owned()));
        assert!(manifest
            .adapter_health_reason_tokens
            .contains(&"stale_artifact".to_owned()));
        assert!(manifest
            .discovery_diff_change_tokens
            .contains(&"newly_exact".to_owned()));
    }

    #[test]
    fn discovery_diff_separates_refresh_change_classes() {
        let previous = vec![
            row(
                "target:web",
                "web",
                BuildIntelligenceLaneType::NativeAdapter,
                TargetExactnessStatus::Exact,
                "refresh:previous",
            ),
            row(
                "target:api",
                "api",
                BuildIntelligenceLaneType::StructuredProtocol,
                TargetExactnessStatus::ProtocolBacked,
                "refresh:previous",
            ),
            row(
                "target:old",
                "old",
                BuildIntelligenceLaneType::StructuredProtocol,
                TargetExactnessStatus::ProtocolBacked,
                "refresh:previous",
            ),
            row(
                "target:cli",
                "cli",
                BuildIntelligenceLaneType::StructuredOutputImport,
                TargetExactnessStatus::Imported,
                "refresh:previous",
            ),
        ];
        let current = vec![
            row(
                "target:web",
                "web-test",
                BuildIntelligenceLaneType::HeuristicFallback,
                TargetExactnessStatus::Heuristic,
                "refresh:current",
            ),
            row(
                "target:api",
                "api",
                BuildIntelligenceLaneType::StructuredProtocol,
                TargetExactnessStatus::Unresolved,
                "refresh:current",
            ),
            row(
                "target:cli",
                "cli",
                BuildIntelligenceLaneType::NativeAdapter,
                TargetExactnessStatus::Exact,
                "refresh:current",
            ),
            row(
                "target:new",
                "new",
                BuildIntelligenceLaneType::BuildEventStream,
                TargetExactnessStatus::Imported,
                "refresh:current",
            ),
        ];

        let diff = DiscoveryDiffReview::between(
            "diff:refresh",
            "workspace:test",
            "refresh:previous",
            "refresh:current",
            "2026-05-18T12:01:00Z",
            &previous,
            &current,
        );

        assert!(diff.has_changes());
        assert_eq!(diff.added.len(), 1);
        assert_eq!(diff.removed.len(), 1);
        assert_eq!(diff.renamed.len(), 1);
        assert_eq!(diff.downgraded_confidence.len(), 2);
        assert_eq!(diff.newly_heuristic.len(), 1);
        assert_eq!(diff.newly_exact.len(), 1);
        assert_eq!(diff.now_unresolved.len(), 1);
        assert!(diff.summary.contains("newly_heuristic=1"));
    }

    #[test]
    fn receipt_keeps_imported_truth_distinct_from_live_truth() {
        let strip = strip("import", BuildIntelligenceLaneType::StructuredOutputImport);
        let row = BuildIntelligenceTargetRow::new(
            "row:imported",
            "target:ci:test",
            "ci test",
            BuildIntelligenceLaneType::StructuredOutputImport,
            strip.strip_id.clone(),
            TargetExactnessStatus::Imported,
            ImportedLiveState::ImportedArtifact,
            "imported from retained CI artifact; refresh before rerun",
            lineage("refresh:import", Some("refresh:previous")).with_refs(
                Some("snapshot:import".to_owned()),
                Some("raw:ci-artifact".to_owned()),
                None,
                Some("artifact:ci:51".to_owned()),
            ),
        );
        let receipt = BuildIntelligenceReceipt::from_target_row(
            "receipt:imported",
            "test.run",
            "run:ci:51",
            &row,
            &strip,
            "ci/linux",
            ArtifactSourceClass::StructuredImport,
            Some("artifact:junit:51".to_owned()),
            "imported artifact; no current live adapter result",
            HighTrustActionPosture::RefreshRequired,
        );

        assert_eq!(
            receipt.imported_live_state,
            ImportedLiveState::ImportedArtifact
        );
        assert_eq!(
            receipt.high_trust_action_posture,
            HighTrustActionPosture::RefreshRequired
        );
        assert!(receipt
            .imported_or_replayed_note
            .contains("no current live"));
    }

    #[test]
    fn support_export_renders_lanes_health_receipts_and_diffs() {
        let native = strip("native", BuildIntelligenceLaneType::NativeAdapter);
        let partial = AdapterHealthStrip::new(
            "strip:managed",
            "workspace:test",
            BuildIntelligenceLaneType::StructuredProtocol,
            AdapterIdentity::new(
                "adapter:managed",
                "Managed workspace protocol",
                Some("2.0".to_owned()),
                Some("aureline-managed".to_owned()),
                Some("1".to_owned()),
            ),
            AdapterHealthState::Partial,
            Some(AdapterHealthReason::ControlPlaneOutage),
            Some("2026-05-18T11:59:00Z".to_owned()),
            ImportedLiveState::MixedLiveAndImported,
            lineage("refresh:managed", Some("refresh:previous")),
            action(
                BuildIntelligenceActionClass::RefreshDiscovery,
                "managed-refresh",
            ),
            action(BuildIntelligenceActionClass::OpenDetails, "managed-details"),
        )
        .with_continuation_actions(
            Some(action(
                BuildIntelligenceActionClass::ContinueLocal,
                "continue-local",
            )),
            Some(action(
                BuildIntelligenceActionClass::InspectOnly,
                "inspect-only",
            )),
        );
        let row = row(
            "target:web",
            "web",
            BuildIntelligenceLaneType::NativeAdapter,
            TargetExactnessStatus::Exact,
            "refresh:current",
        )
        .with_bindings(
            Some("typescript_web_app".to_owned()),
            Some("vite".to_owned()),
        )
        .with_actions(
            Some(action(BuildIntelligenceActionClass::OpenSource, "source")),
            Some(action(BuildIntelligenceActionClass::OpenConfig, "config")),
        );
        let card = BuildIntelligenceRunConfigCard::from_target_row(
            "card:web",
            "task.run.web",
            &row,
            HighTrustActionPosture::LiveActionsAllowed,
        );
        let receipt = BuildIntelligenceReceipt::from_target_row(
            "receipt:web",
            "task.run.web",
            "run:web:1",
            &row,
            &native,
            "local/macos",
            ArtifactSourceClass::LiveAdapter,
            Some("artifact:web:1".to_owned()),
            "live adapter result",
            HighTrustActionPosture::LiveActionsAllowed,
        );
        let diff = DiscoveryDiffReview::between(
            "diff:none",
            "workspace:test",
            "refresh:previous",
            "refresh:current",
            "2026-05-18T12:01:00Z",
            std::slice::from_ref(&row),
            std::slice::from_ref(&row),
        );
        let export = BuildIntelligenceSupportExport::new(
            "support:build-intelligence",
            "workspace:test",
            "2026-05-18T12:02:00Z",
            vec![native, partial],
            vec![row],
            vec![card],
            vec![receipt],
            vec![diff],
        );
        let plaintext = export.render_plaintext();
        assert!(plaintext.contains("lane=native_adapter"));
        assert!(plaintext.contains("reason=control_plane_outage"));
        assert!(plaintext.contains("continue_local=action:test:continue-local"));
        assert!(plaintext.contains("artifact_source=live_adapter"));
        assert!(!plaintext.contains("/Users/"));
    }
}
