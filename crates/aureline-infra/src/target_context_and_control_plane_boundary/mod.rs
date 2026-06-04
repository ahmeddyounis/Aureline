//! Target-context, connector-class, and control-plane handoff qualification.
//!
//! The packet in this module is the shared evidence boundary for terminal,
//! logs, resource graph, incident, AI, CLI, support-export, and
//! browser-console handoff surfaces. It separates desired, rendered, planned,
//! observed, cached, permission-limited, unavailable, and provider-overlay
//! truth so static parsing never becomes live mutation authority.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Schema version for infrastructure boundary qualification packets.
pub const CONTROL_PLANE_BOUNDARY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind discriminator for [`InfraBoundaryPacket`].
pub const CONTROL_PLANE_BOUNDARY_PACKET_RECORD_KIND: &str =
    "infra_target_context_control_plane_boundary_packet";

/// JSON Schema reference for packet interchange.
pub const CONTROL_PLANE_BOUNDARY_SCHEMA_REF: &str =
    "schemas/infra/environment-context-and-action-safety.schema.json";

/// Reviewer-facing documentation reference.
pub const CONTROL_PLANE_BOUNDARY_DOC_REF: &str =
    "docs/infra/target-context-and-control-plane-boundary.md";

/// Fixture corpus directory for qualification and downgrade drills.
pub const CONTROL_PLANE_BOUNDARY_FIXTURE_DIR: &str =
    "fixtures/infra/target-context-and-control-plane-boundary";

/// Architecture-level connector classes for infrastructure surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectorClass {
    /// Repo parsers, renderers, and imported source artifacts only.
    StaticFileOnly,
    /// Local or remote CLI tools with explicitly resolved target context.
    CliMediated,
    /// Remote agent or managed helper with scoped live credentials.
    AgentMediatedLive,
    /// Provider API, browser, or console overlay that enriches or hands off.
    ProviderConsoleOverlay,
}

impl ConnectorClass {
    /// Stable label used in schema fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaticFileOnly => "static/file-only",
            Self::CliMediated => "CLI-mediated",
            Self::AgentMediatedLive => "agent-mediated live",
            Self::ProviderConsoleOverlay => "provider/console overlay",
        }
    }
}

/// Truth layer attached to manifests, diffs, graphs, logs, and explanations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruthClass {
    /// Repo-authored desired state.
    Desired,
    /// Generated state derived from desired inputs.
    Rendered,
    /// Plan, diff, dry-run, validation, or policy result.
    Planned,
    /// Live observation from a connector.
    Observed,
    /// Last-known-good or offline snapshot.
    Cached,
    /// Provider or connector returned only part of the scope.
    PermissionLimited,
    /// Live target or state class is unavailable.
    Unavailable,
    /// Provider-owned metadata or console-only context.
    ProviderOverlay,
}

/// Resource row state shown across infrastructure surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateClass {
    /// Row represents desired state.
    Desired,
    /// Row represents rendered output.
    Rendered,
    /// Row represents planned or validated output.
    Planned,
    /// Row represents observed live state.
    Observed,
    /// Row represents cached last-known-good state.
    Cached,
    /// Row is visible through narrowed permissions.
    PermissionLimited,
    /// Row cannot currently be fetched.
    Unavailable,
}

/// Freshness and provenance label rendered with target context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessLabel {
    /// Live and within the required freshness floor.
    Live,
    /// Current snapshot that is not streaming live.
    CurrentSnapshot,
    /// Cached but still within the accepted freshness floor.
    CachedWithinFloor,
    /// Cached or observed data is stale.
    Stale,
    /// Only part of the requested state was observed.
    Partial,
    /// Live target is offline.
    Offline,
    /// Provider denied part of the requested scope.
    PermissionLimited,
    /// No live evidence is available.
    Unavailable,
}

/// Effective action posture for a connector or review packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionPosture {
    /// Read-only inspection is allowed.
    InspectOnly,
    /// Compare, plan, or dry-run is allowed.
    DryRunOnly,
    /// Write requires explicit step-up and approval.
    StepUpRequired,
    /// Write is approved by a current packet.
    WriteApproved,
    /// Action is blocked.
    Blocked,
    /// Aureline does not claim this action.
    NotClaimed,
    /// Action leaves Aureline through an explicit handoff.
    HandoffOnly,
}

/// Stable promotion posture for a claimed row or surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationPosture {
    /// Stable claim is backed by current packet evidence.
    StableQualified,
    /// Surface is limited to source and rendered inspection.
    FileOnly,
    /// Surface can inspect live or cached evidence but cannot mutate.
    InspectOnly,
    /// Surface only hands off to a provider console or control plane.
    HandoffOnly,
    /// Surface must not be promoted.
    Downgraded,
}

/// Protected action kinds that raise target or authority boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
    /// Read-only inspection.
    Inspect,
    /// Plan, compare, or dry-run.
    DryRun,
    /// Apply or otherwise mutate desired state.
    Mutate,
    /// Open a port-forward.
    PortForward,
    /// Attach shell to a target.
    ShellAttach,
    /// Execute in a workload or container.
    Exec,
    /// Execute in a container-specific context.
    ContainerExec,
    /// Launch a provider console or browser handoff.
    BrowserConsoleLaunch,
}

impl ActionKind {
    /// True when the action must show duration, scope, and revocation path.
    pub const fn raises_boundary(self) -> bool {
        matches!(
            self,
            Self::Mutate
                | Self::PortForward
                | Self::ShellAttach
                | Self::Exec
                | Self::ContainerExec
                | Self::BrowserConsoleLaunch
        )
    }
}

/// Consumer surface that must show the same target truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKind {
    /// Integrated terminal header or command preview.
    Terminal,
    /// Log and event stream.
    Logs,
    /// Infrastructure resource graph.
    ResourceGraph,
    /// Incident workspace or runbook step.
    IncidentWorkspace,
    /// AI action or explanation sheet.
    AiActionSheet,
    /// Machine-readable CLI JSON.
    CliJson,
    /// Browser or provider console handoff.
    BrowserConsoleHandoff,
    /// Support bundle or qualification export.
    SupportExport,
}

/// Completeness state for the environment context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentCompleteness {
    /// Every applicable target identity field is populated.
    Complete,
    /// Optional fields are absent but acknowledged.
    PartialAcknowledged,
    /// Required target fields are missing.
    Incomplete,
}

/// Explicit environment-context object shared by all infrastructure surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentContext {
    /// Stable context id used across all surfaces.
    pub context_id: String,
    /// Provider family, such as Kubernetes, AWS, or Terraform.
    pub provider: String,
    /// Account, subscription, or project reference.
    pub account_subscription_project: String,
    /// Cluster reference when applicable.
    pub cluster: Option<String>,
    /// Namespace reference when applicable.
    pub namespace: Option<String>,
    /// Region or zone reference when applicable.
    pub region_zone: Option<String>,
    /// Tenant reference when applicable.
    pub tenant: Option<String>,
    /// Workspace root that supplied desired state.
    pub workspace_root: String,
    /// Branch, worktree, or commit ref.
    pub branch_worktree_or_commit: String,
    /// Execution-context profile ref.
    pub execution_context_profile: String,
    /// CLI or toolchain identity and version.
    pub toolchain_cli_identity: String,
    /// Credential handle class, never raw credential material.
    pub credential_handle_class: String,
    /// Credential issuance source.
    pub issuance_source: String,
    /// Credential expiry timestamp or explicit non-expiring marker.
    pub expiry: Option<String>,
    /// Effective write scope.
    pub write_scope: String,
    /// Observation timestamp for this context.
    pub observed_at: String,
    /// Completeness class for populated fields.
    pub completeness: EnvironmentCompleteness,
    /// True when local shell or ambient kube/cloud context is prohibited.
    pub ambient_context_prohibited: bool,
    /// True for production or other high-risk targets.
    pub high_risk: bool,
}

impl EnvironmentContext {
    fn target_signature(&self) -> (&str, &str, Option<&str>, Option<&str>, Option<&str>) {
        (
            self.provider.as_str(),
            self.account_subscription_project.as_str(),
            self.cluster.as_deref(),
            self.namespace.as_deref(),
            self.region_zone.as_deref(),
        )
    }
}

/// Connector-class policy row binding class to allowed action envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectorClassPolicy {
    /// Connector class covered by this row.
    pub connector_class: ConnectorClass,
    /// Allowed posture by action kind.
    pub allowed_actions: BTreeMap<ActionKind, ActionPosture>,
    /// Required freshness labels for stable claims.
    pub freshness_labels: Vec<FreshnessLabel>,
    /// Required approval or step-up posture.
    pub required_step_up_posture: ActionPosture,
    /// Export-safe summary.
    pub summary: String,
}

/// Relationship row connecting source, rendered, plan, live, and overlay truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceLinkRow {
    /// Stable relationship id.
    pub link_id: String,
    /// Resource display label.
    pub resource_label: String,
    /// Truth class for the row.
    pub truth_class: TruthClass,
    /// State class rendered in the row.
    pub state_class: StateClass,
    /// Relationship edge label.
    pub relationship_edge: String,
    /// Source resource ref when known.
    pub desired_ref: Option<String>,
    /// Rendered resource ref when known.
    pub rendered_ref: Option<String>,
    /// Planned resource ref when known.
    pub planned_ref: Option<String>,
    /// Observed live resource ref when known.
    pub observed_ref: Option<String>,
    /// Provider-overlay ref when known.
    pub provider_overlay_ref: Option<String>,
    /// Freshness label for the row.
    pub freshness: FreshnessLabel,
    /// True when mutation must be disabled for this row.
    pub mutation_downgraded: bool,
}

/// Target-context chip rendered by one consuming surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetContextChip {
    /// Referenced environment context.
    pub context_ref: String,
    /// Provider label shown on the chip.
    pub provider: String,
    /// Account, subscription, or project label.
    pub account_subscription_project: String,
    /// Cluster label.
    pub cluster: Option<String>,
    /// Namespace label.
    pub namespace: Option<String>,
    /// Region or zone label.
    pub region_zone: Option<String>,
    /// Execution origin label.
    pub execution_origin: String,
    /// Mismatch state shown before action.
    pub mismatch_state: String,
    /// Dry-run availability label.
    pub dry_run_available: bool,
    /// Rollback or checkpoint posture label.
    pub rollback_checkpoint_posture: String,
}

/// Surface binding proving all consumers share target-context truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceBinding {
    /// Surface being bound.
    pub surface: SurfaceKind,
    /// Qualification posture for this surface.
    pub qualification_posture: QualificationPosture,
    /// Target chip rendered by the surface.
    pub target_chip: TargetContextChip,
    /// Truth classes preserved by the surface.
    pub truth_classes: Vec<TruthClass>,
    /// State classes preserved by the surface.
    pub state_classes: Vec<StateClass>,
    /// Resource link rows shown or exported by the surface.
    pub resource_link_refs: Vec<String>,
    /// True when this surface consumes the same packet.
    pub uses_shared_packet: bool,
}

/// Review packet for a protected live or handoff action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryActionReview {
    /// Stable review id.
    pub review_id: String,
    /// Action kind under review.
    pub action_kind: ActionKind,
    /// Target context shown before action.
    pub target_context_ref: String,
    /// Connector class used by the action.
    pub connector_class: ConnectorClass,
    /// Effective action posture.
    pub action_posture: ActionPosture,
    /// Duration shown for boundary-raising actions.
    pub duration: Option<String>,
    /// Credential or secret scope shown before action.
    pub credential_scope: Option<String>,
    /// Revocation path shown before action.
    pub revocation_path: Option<String>,
    /// Previewable command or request envelope.
    pub preview_envelope: Option<ActionEnvelope>,
    /// Approval or step-up lineage ref.
    pub approval_lineage_ref: Option<String>,
}

/// Previewable command or request envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionEnvelope {
    /// Redaction-safe envelope ref.
    pub envelope_ref: String,
    /// Preview hash for command or request body.
    pub preview_hash: String,
    /// Dry-run result ref when available.
    pub dry_run_result_ref: Option<String>,
    /// Rollback or checkpoint ref when available.
    pub rollback_checkpoint_ref: Option<String>,
}

/// Explicit provider-console or browser handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlPlaneHandoff {
    /// Handoff id.
    pub handoff_id: String,
    /// Destination provider or console.
    pub destination: String,
    /// Target context carried into the handoff.
    pub target_context_ref: String,
    /// Connector class used by the handoff.
    pub connector_class: ConnectorClass,
    /// True when the destination is disclosed as outside Aureline authority.
    pub explicit_handoff_destination: bool,
    /// True when the handoff is not treated as substitute product truth.
    pub not_substitute_truth: bool,
    /// Return or revocation path.
    pub return_or_revocation_path: String,
    /// Audit lineage ref.
    pub audit_ref: String,
}

/// Qualification packet for an infrastructure or ops-facing surface group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfraBoundaryPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Shared environment context.
    pub environment_context: EnvironmentContext,
    /// Connector-class matrix.
    pub connector_policies: Vec<ConnectorClassPolicy>,
    /// Resource relationship rows.
    pub resource_links: Vec<ResourceLinkRow>,
    /// Surface projections that must agree on target truth.
    pub surface_bindings: Vec<SurfaceBinding>,
    /// Protected action review packets.
    pub action_reviews: Vec<BoundaryActionReview>,
    /// Provider-console handoff packets.
    pub control_plane_handoffs: Vec<ControlPlaneHandoff>,
    /// Export-safe support summary.
    pub support_summary: String,
}

impl InfraBoundaryPacket {
    /// Validate this packet against target-context and handoff invariants.
    pub fn validate(&self) -> InfraBoundaryValidationReport {
        validate_packet(self)
    }
}

/// Validates one infrastructure boundary packet.
pub fn validate_packet(packet: &InfraBoundaryPacket) -> InfraBoundaryValidationReport {
    let mut findings = Vec::new();
    let mut connector_classes = BTreeSet::new();
    let mut truth_classes = BTreeSet::new();
    let mut state_classes = BTreeSet::new();
    let mut surfaces = BTreeSet::new();
    let resource_ids: BTreeSet<_> = packet
        .resource_links
        .iter()
        .map(|row| row.link_id.as_str())
        .collect();

    if packet.record_kind != CONTROL_PLANE_BOUNDARY_PACKET_RECORD_KIND {
        findings.push(error(
            "record_kind",
            "Packet record_kind is not the infra boundary discriminator.",
        ));
    }
    if packet.schema_version != CONTROL_PLANE_BOUNDARY_SCHEMA_VERSION {
        findings.push(error(
            "schema_version",
            "Packet schema_version does not match this crate.",
        ));
    }
    if !packet.environment_context.ambient_context_prohibited {
        findings.push(error(
            "ambient_context",
            "Environment context allows ambient shell or kube/cloud inheritance.",
        ));
    }
    if packet.environment_context.completeness == EnvironmentCompleteness::Incomplete {
        findings.push(error(
            "environment_context",
            "Environment context is incomplete.",
        ));
    }

    for policy in &packet.connector_policies {
        connector_classes.insert(policy.connector_class);
        if matches!(policy.connector_class, ConnectorClass::StaticFileOnly) {
            for action in [
                ActionKind::Mutate,
                ActionKind::PortForward,
                ActionKind::ShellAttach,
                ActionKind::Exec,
                ActionKind::ContainerExec,
            ] {
                if matches!(
                    policy.allowed_actions.get(&action),
                    Some(ActionPosture::WriteApproved | ActionPosture::StepUpRequired)
                ) {
                    findings.push(error(
                        "static_file_only_authority",
                        "Static/file-only connector policy grants live or boundary-raising authority.",
                    ));
                }
            }
        }
    }

    for required in [
        ConnectorClass::StaticFileOnly,
        ConnectorClass::CliMediated,
        ConnectorClass::AgentMediatedLive,
        ConnectorClass::ProviderConsoleOverlay,
    ] {
        if !connector_classes.contains(&required) {
            findings.push(error(
                "connector_matrix",
                "Connector-class matrix is missing a required class.",
            ));
        }
    }

    for row in &packet.resource_links {
        truth_classes.insert(row.truth_class);
        state_classes.insert(row.state_class);
        if matches!(
            row.state_class,
            StateClass::Cached | StateClass::PermissionLimited | StateClass::Unavailable
        ) && !row.mutation_downgraded
        {
            findings.push(error(
                "resource_state_downgrade",
                "Cached, permission-limited, or unavailable resource row is not downgraded for mutation.",
            ));
        }
        if row.truth_class == TruthClass::Desired
            && row.observed_ref.is_some()
            && row.planned_ref.is_none()
        {
            findings.push(warning(
                "desired_live_gap",
                "Desired row links to observed state without a planned or validation ref.",
            ));
        }
    }

    for binding in &packet.surface_bindings {
        surfaces.insert(binding.surface);
        if !binding.uses_shared_packet {
            findings.push(error(
                "surface_packet",
                "Surface binding does not consume the shared packet.",
            ));
        }
        if binding.target_chip.context_ref != packet.environment_context.context_id {
            findings.push(error(
                "target_chip",
                "Surface target chip points at a different environment context.",
            ));
        }
        let expected = packet.environment_context.target_signature();
        let actual = (
            binding.target_chip.provider.as_str(),
            binding.target_chip.account_subscription_project.as_str(),
            binding.target_chip.cluster.as_deref(),
            binding.target_chip.namespace.as_deref(),
            binding.target_chip.region_zone.as_deref(),
        );
        if expected != actual {
            findings.push(error(
                "target_chip",
                "Surface target chip does not match the environment context.",
            ));
        }
        for link_ref in &binding.resource_link_refs {
            if !resource_ids.contains(link_ref.as_str()) {
                findings.push(error(
                    "resource_link",
                    "Surface references an unknown resource-link row.",
                ));
            }
        }
    }

    for required in [
        SurfaceKind::Terminal,
        SurfaceKind::Logs,
        SurfaceKind::ResourceGraph,
        SurfaceKind::IncidentWorkspace,
        SurfaceKind::AiActionSheet,
        SurfaceKind::CliJson,
        SurfaceKind::BrowserConsoleHandoff,
        SurfaceKind::SupportExport,
    ] {
        if !surfaces.contains(&required) {
            findings.push(error(
                "surface_parity",
                "Packet is missing a required consumer surface.",
            ));
        }
    }

    for required in [
        TruthClass::Desired,
        TruthClass::Rendered,
        TruthClass::Planned,
        TruthClass::Observed,
        TruthClass::Cached,
        TruthClass::PermissionLimited,
        TruthClass::Unavailable,
        TruthClass::ProviderOverlay,
    ] {
        if !truth_classes.contains(&required) {
            findings.push(error(
                "truth_class_coverage",
                "Packet is missing a required truth class.",
            ));
        }
    }
    for required in [
        StateClass::Desired,
        StateClass::Rendered,
        StateClass::Planned,
        StateClass::Observed,
        StateClass::Cached,
        StateClass::PermissionLimited,
        StateClass::Unavailable,
    ] {
        if !state_classes.contains(&required) {
            findings.push(error(
                "state_class_coverage",
                "Packet is missing a required state class.",
            ));
        }
    }

    for review in &packet.action_reviews {
        if review.target_context_ref != packet.environment_context.context_id {
            findings.push(error(
                "action_target",
                "Action review points at a different target context.",
            ));
        }
        if review.action_kind.raises_boundary()
            && (review.duration.is_none()
                || review.credential_scope.is_none()
                || review.revocation_path.is_none())
        {
            findings.push(error(
                "boundary_action_review",
                "Boundary-raising action lacks duration, credential scope, or revocation path.",
            ));
        }
        if packet.environment_context.high_risk
            && matches!(review.action_kind, ActionKind::Mutate)
            && matches!(review.action_posture, ActionPosture::WriteApproved)
            && (review.preview_envelope.is_none() || review.approval_lineage_ref.is_none())
        {
            findings.push(error(
                "high_risk_step_up",
                "High-risk mutation lacks preview envelope or approval lineage.",
            ));
        }
        if matches!(review.connector_class, ConnectorClass::StaticFileOnly)
            && review.action_kind.raises_boundary()
            && !matches!(
                review.action_posture,
                ActionPosture::Blocked | ActionPosture::NotClaimed
            )
        {
            findings.push(error(
                "static_file_action",
                "Static/file-only action review is not blocked for a boundary-raising action.",
            ));
        }
    }

    for handoff in &packet.control_plane_handoffs {
        if handoff.target_context_ref != packet.environment_context.context_id {
            findings.push(error(
                "handoff_target",
                "Control-plane handoff points at a different target context.",
            ));
        }
        if !matches!(
            handoff.connector_class,
            ConnectorClass::ProviderConsoleOverlay
        ) {
            findings.push(error(
                "handoff_connector",
                "Control-plane handoff does not use provider/console overlay class.",
            ));
        }
        if !handoff.explicit_handoff_destination || !handoff.not_substitute_truth {
            findings.push(error(
                "handoff_truth",
                "Control-plane handoff is not explicit or is treated as substitute truth.",
            ));
        }
    }

    let passed = findings
        .iter()
        .all(|finding| finding.severity != InfraBoundaryFindingSeverity::Error);
    InfraBoundaryValidationReport {
        record_kind: "infra_target_context_control_plane_boundary_validation_report".to_string(),
        schema_version: CONTROL_PLANE_BOUNDARY_SCHEMA_VERSION,
        packet_id: packet.packet_id.clone(),
        passed,
        connector_classes,
        truth_classes,
        state_classes,
        surfaces,
        findings,
    }
}

/// Validation report emitted for an infrastructure boundary packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfraBoundaryValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Packet id validated.
    pub packet_id: String,
    /// True when no error-severity finding was emitted.
    pub passed: bool,
    /// Connector classes covered by the packet.
    pub connector_classes: BTreeSet<ConnectorClass>,
    /// Truth classes covered by resource rows.
    pub truth_classes: BTreeSet<TruthClass>,
    /// State classes covered by resource rows.
    pub state_classes: BTreeSet<StateClass>,
    /// Surfaces covered by target-chip parity.
    pub surfaces: BTreeSet<SurfaceKind>,
    /// Findings emitted during validation.
    pub findings: Vec<InfraBoundaryFinding>,
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfraBoundaryFinding {
    /// Severity of the finding.
    pub severity: InfraBoundaryFindingSeverity,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe message.
    pub message: String,
}

/// Validation finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfraBoundaryFindingSeverity {
    /// Blocks qualification.
    Error,
    /// Keeps the packet reviewable with a visible caveat.
    Warning,
}

fn error(check_id: &str, message: &str) -> InfraBoundaryFinding {
    InfraBoundaryFinding {
        severity: InfraBoundaryFindingSeverity::Error,
        check_id: check_id.to_string(),
        message: message.to_string(),
    }
}

fn warning(check_id: &str, message: &str) -> InfraBoundaryFinding {
    InfraBoundaryFinding {
        severity: InfraBoundaryFindingSeverity::Warning,
        check_id: check_id.to_string(),
        message: message.to_string(),
    }
}
