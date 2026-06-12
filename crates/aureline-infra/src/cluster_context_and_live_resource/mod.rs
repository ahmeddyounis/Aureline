//! Cluster-context strips, truth-mode views, and console-handoff truth.
//!
//! This packet extends the target-context and control-plane boundary model for
//! the Terraform, Kubernetes, and incident-adjacent surfaces. It keeps each
//! surface explicit about the exact target context, renders desired, rendered,
//! plan, live, and provider-overlay state as separate truth modes rather than
//! one blended resource view, gates every mutating or boundary-raising action
//! behind a reviewed preview or handoff, and preserves vendor-console handoff
//! truth whenever Aureline is not the authoritative control plane.
//!
//! It reuses the [`EnvironmentContext`], [`FreshnessLabel`], [`ActionKind`],
//! [`ConnectorClass`], and [`ControlPlaneHandoff`] vocabulary from
//! [`crate::target_context_and_control_plane_boundary`] instead of inventing a
//! parallel infrastructure model.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::target_context_and_control_plane_boundary::{
    ActionKind, ConnectorClass, ControlPlaneHandoff, EnvironmentCompleteness, EnvironmentContext,
    FreshnessLabel, InfraBoundaryFinding, InfraBoundaryFindingSeverity,
};

/// Schema version for cluster-context and live-resource packets.
pub const CLUSTER_LIVE_RESOURCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind discriminator for [`ClusterLiveResourcePacket`].
pub const CLUSTER_LIVE_RESOURCE_PACKET_RECORD_KIND: &str =
    "infra_cluster_context_and_live_resource_packet";

/// JSON Schema reference for packet interchange.
pub const CLUSTER_LIVE_RESOURCE_SCHEMA_REF: &str =
    "schemas/infra/cluster-context-and-live-resource.schema.json";

/// Reviewer-facing documentation reference.
pub const CLUSTER_LIVE_RESOURCE_DOC_REF: &str = "docs/infra/cluster-context-and-live-resource.md";

/// Fixture corpus directory for context-strip and truth-mode drills.
pub const CLUSTER_LIVE_RESOURCE_FIXTURE_DIR: &str =
    "fixtures/infra/cluster-context-and-live-resource";

/// Provisioning tool family that owns the surfaces in this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpsToolKind {
    /// Terraform or other declarative infrastructure provisioning.
    Terraform,
    /// Kubernetes cluster and workload management.
    Kubernetes,
    /// Incident workspace or runbook adjacent to the above.
    IncidentAdjacent,
}

/// Infrastructure-adjacent surface that renders a context strip and views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpsSurface {
    /// Terraform plan or apply review.
    TerraformPlanReview,
    /// Kubernetes resource browser or manifest view.
    KubernetesResourceView,
    /// Live cluster resource inspection.
    ClusterLiveResource,
    /// Incident workspace runbook step.
    IncidentRunbookStep,
    /// AI action or explanation sheet.
    AiActionSheet,
    /// Machine-readable CLI JSON.
    Cli,
    /// Browser or provider console handoff.
    BrowserConsoleHandoff,
    /// Support bundle or runbook export.
    SupportRunbookExport,
}

impl OpsSurface {
    /// True when the surface must render all five truth modes as separate views.
    const fn requires_all_truth_modes(self) -> bool {
        matches!(
            self,
            Self::TerraformPlanReview
                | Self::KubernetesResourceView
                | Self::ClusterLiveResource
                | Self::IncidentRunbookStep
                | Self::SupportRunbookExport
        )
    }
}

/// Distinct truth mode rendered as its own view, never blended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruthMode {
    /// Repo-authored desired state.
    Desired,
    /// Generated state derived from desired inputs.
    Rendered,
    /// Plan, diff, dry-run, or validation result.
    Plan,
    /// Live observation from a connector.
    Live,
    /// Provider-owned overlay or console-only context.
    ProviderOverlay,
}

impl TruthMode {
    /// True when a view in this mode may be marked mutation-capable.
    const fn mutation_capable_allowed(self) -> bool {
        matches!(self, Self::Live)
    }
}

/// Freshness labels that block live mutation when set on a live view.
const NON_MUTABLE_FRESHNESS: &[FreshnessLabel] = &[
    FreshnessLabel::Stale,
    FreshnessLabel::Offline,
    FreshnessLabel::PermissionLimited,
    FreshnessLabel::Unavailable,
    FreshnessLabel::CachedWithinFloor,
];

/// Context strip naming the exact target a surface is acting against.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClusterContextStrip {
    /// Stable strip id.
    pub strip_id: String,
    /// Referenced environment context.
    pub context_ref: String,
    /// Provider family shown on the strip.
    pub provider: String,
    /// Account or subscription reference.
    pub account_subscription: String,
    /// Project reference when applicable.
    pub project: Option<String>,
    /// Cluster reference when applicable.
    pub cluster: Option<String>,
    /// Namespace reference when applicable.
    pub namespace: Option<String>,
    /// Region or zone reference when applicable.
    pub region: Option<String>,
    /// Tenant reference when applicable.
    pub tenant: Option<String>,
    /// Execution origin shown on the strip.
    pub execution_origin: String,
    /// Credential handle class, never raw credential material.
    pub credential_class: String,
}

/// One truth-mode view rendered with explicit freshness and source labels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TruthModeView {
    /// Truth mode this view renders.
    pub truth_mode: TruthMode,
    /// Human-facing source label, such as "repo manifest" or "kube live".
    pub source_label: String,
    /// Stable source ref backing the view.
    pub source_ref: String,
    /// Freshness label for the view.
    pub freshness: FreshnessLabel,
    /// Observation timestamp when the view is plan or live derived.
    pub observed_at: Option<String>,
    /// True when this view is collapsed with another truth mode.
    pub blended_with_other_modes: bool,
    /// True when this view can initiate mutation against the target.
    pub mutation_capable: bool,
}

/// Projection of one ops-adjacent surface onto the shared packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpsSurfaceProjection {
    /// Surface being projected.
    pub surface: OpsSurface,
    /// Provisioning tool family that owns the surface.
    pub tool_kind: OpsToolKind,
    /// Context strip rendered by the surface.
    pub context_strip: ClusterContextStrip,
    /// Separate truth-mode views rendered by the surface.
    pub truth_mode_views: Vec<TruthModeView>,
    /// True when the surface preserves the in-product truth-mode vocabulary.
    pub preserves_truth_vocabulary: bool,
    /// True when this surface consumes the same packet.
    pub uses_shared_packet: bool,
}

/// Gate proving a mutating or boundary-raising action was previewed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutatingActionGate {
    /// Stable gate id.
    pub gate_id: String,
    /// Surface that initiated the action.
    pub surface: OpsSurface,
    /// Provisioning tool family for the action.
    pub tool_kind: OpsToolKind,
    /// Action kind under review.
    pub action_kind: ActionKind,
    /// Target context shown before action.
    pub context_ref: String,
    /// True when the action requires a reviewed preview or handoff.
    pub requires_reviewed_preview: bool,
    /// Preview, plan, or dry-run ref shown before execution.
    pub preview_ref: Option<String>,
    /// Exact-target preview ref shown before execution.
    pub target_preview_ref: Option<String>,
    /// Source-of-truth posture shown before execution or handoff.
    pub source_of_truth_posture: String,
    /// Console-handoff ref when the action leaves Aureline authority.
    pub handoff_ref: Option<String>,
    /// True when a current approval admits the action.
    pub approved: bool,
}

/// Console-handoff truth preserved when Aureline is not authoritative.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsoleHandoffTruth {
    /// Surface that performed the handoff.
    pub surface: OpsSurface,
    /// Provisioning tool family for the handoff.
    pub tool_kind: OpsToolKind,
    /// True only when Aureline owns the authoritative control plane.
    pub aureline_is_authoritative: bool,
    /// Reused provider-console handoff packet.
    pub handoff: ControlPlaneHandoff,
}

/// Cluster-context and live-resource qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClusterLiveResourcePacket {
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
    /// Surface projections that must agree on context and truth modes.
    pub surface_projections: Vec<OpsSurfaceProjection>,
    /// Mutating or boundary-raising action gates.
    pub action_gates: Vec<MutatingActionGate>,
    /// Provider-console handoff truth packets.
    pub console_handoffs: Vec<ConsoleHandoffTruth>,
    /// Export-safe support summary.
    pub support_summary: String,
}

impl ClusterLiveResourcePacket {
    /// Validate this packet against context-strip and truth-mode invariants.
    pub fn validate(&self) -> ClusterLiveResourceValidationReport {
        validate_packet(self)
    }
}

/// Validates one cluster-context and live-resource packet.
pub fn validate_packet(packet: &ClusterLiveResourcePacket) -> ClusterLiveResourceValidationReport {
    let mut findings = Vec::new();
    let mut truth_modes = BTreeSet::new();
    let mut surfaces = BTreeSet::new();
    let mut tool_kinds = BTreeSet::new();
    let context_id = packet.environment_context.context_id.as_str();

    if packet.record_kind != CLUSTER_LIVE_RESOURCE_PACKET_RECORD_KIND {
        findings.push(error(
            "record_kind",
            "Packet record_kind is not the cluster live-resource discriminator.",
        ));
    }
    if packet.schema_version != CLUSTER_LIVE_RESOURCE_SCHEMA_VERSION {
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

    for projection in &packet.surface_projections {
        surfaces.insert(projection.surface);
        tool_kinds.insert(projection.tool_kind);

        if !projection.uses_shared_packet {
            findings.push(error(
                "surface_packet",
                "Surface projection does not consume the shared packet.",
            ));
        }

        validate_strip(
            &projection.context_strip,
            &packet.environment_context,
            &mut findings,
        );

        let mut projection_modes = BTreeSet::new();
        for view in &projection.truth_mode_views {
            truth_modes.insert(view.truth_mode);
            if !projection_modes.insert(view.truth_mode) {
                findings.push(error(
                    "duplicate_truth_mode",
                    "Surface renders the same truth mode more than once.",
                ));
            }
            if view.blended_with_other_modes {
                findings.push(error(
                    "blended_truth_mode",
                    "Truth-mode view collapses live and authored state into one blended view.",
                ));
            }
            if view.mutation_capable && !view.truth_mode.mutation_capable_allowed() {
                findings.push(error(
                    "mutation_mode",
                    "Non-live truth-mode view is marked mutation-capable.",
                ));
            }
            if view.mutation_capable && NON_MUTABLE_FRESHNESS.contains(&view.freshness) {
                findings.push(error(
                    "mutation_freshness",
                    "Mutation-capable view is not backed by live freshness.",
                ));
            }
            if view.source_label.trim().is_empty() || view.source_ref.trim().is_empty() {
                findings.push(error(
                    "view_source",
                    "Truth-mode view is missing a source label or source ref.",
                ));
            }
        }

        if projection.surface.requires_all_truth_modes() {
            for required in REQUIRED_TRUTH_MODES {
                if !projection_modes.contains(&required) {
                    findings.push(error(
                        "truth_mode_coverage",
                        "Surface does not render all five truth modes as separate views.",
                    ));
                }
            }
        }

        if matches!(projection.surface, OpsSurface::SupportRunbookExport)
            && !projection.preserves_truth_vocabulary
        {
            findings.push(error(
                "support_vocabulary",
                "Support export does not preserve the in-product truth-mode vocabulary.",
            ));
        }
    }

    for required in [
        OpsToolKind::Terraform,
        OpsToolKind::Kubernetes,
        OpsToolKind::IncidentAdjacent,
    ] {
        if !tool_kinds.contains(&required) {
            findings.push(error(
                "tool_coverage",
                "Packet is missing a required Terraform, Kubernetes, or incident-adjacent surface.",
            ));
        }
    }
    if !surfaces.contains(&OpsSurface::SupportRunbookExport) {
        findings.push(error(
            "support_surface",
            "Packet is missing the support runbook export surface.",
        ));
    }
    if !surfaces.contains(&OpsSurface::BrowserConsoleHandoff) {
        findings.push(error(
            "handoff_surface",
            "Packet is missing the browser console handoff surface.",
        ));
    }

    for gate in &packet.action_gates {
        if gate.context_ref != context_id {
            findings.push(error(
                "gate_target",
                "Action gate points at a different target context.",
            ));
        }
        if gate.action_kind.raises_boundary() {
            if !gate.requires_reviewed_preview {
                findings.push(error(
                    "gate_preview_required",
                    "Boundary-raising action gate does not require a reviewed preview.",
                ));
            }
            if gate.target_preview_ref.is_none() {
                findings.push(error(
                    "gate_target_preview",
                    "Boundary-raising action gate does not preview the exact target.",
                ));
            }
            if gate.source_of_truth_posture.trim().is_empty() {
                findings.push(error(
                    "gate_sot_posture",
                    "Boundary-raising action gate does not show the source-of-truth posture.",
                ));
            }
            if gate.preview_ref.is_none() && gate.handoff_ref.is_none() {
                findings.push(error(
                    "gate_preview_or_handoff",
                    "Boundary-raising action gate lacks both a preview and a handoff ref.",
                ));
            }
            if matches!(gate.action_kind, ActionKind::BrowserConsoleLaunch)
                && gate.handoff_ref.is_none()
            {
                findings.push(error(
                    "gate_console_handoff",
                    "Console-launch gate does not carry an explicit handoff ref.",
                ));
            }
            if packet.environment_context.high_risk
                && matches!(gate.action_kind, ActionKind::Mutate)
                && gate.approved
                && gate.preview_ref.is_none()
            {
                findings.push(error(
                    "gate_high_risk_preview",
                    "Approved high-risk mutation lacks a reviewed preview ref.",
                ));
            }
        }
    }

    for handoff in &packet.console_handoffs {
        surfaces.insert(handoff.surface);
        tool_kinds.insert(handoff.tool_kind);
        if handoff.handoff.target_context_ref != context_id {
            findings.push(error(
                "handoff_target",
                "Console handoff points at a different target context.",
            ));
        }
        if !matches!(
            handoff.handoff.connector_class,
            ConnectorClass::ProviderConsoleOverlay
        ) {
            findings.push(error(
                "handoff_connector",
                "Console handoff does not use provider/console overlay class.",
            ));
        }
        if !handoff.handoff.explicit_handoff_destination || !handoff.handoff.not_substitute_truth {
            findings.push(error(
                "handoff_truth",
                "Console handoff is not explicit or is treated as substitute truth.",
            ));
        }
        if handoff.aureline_is_authoritative {
            findings.push(error(
                "handoff_authority",
                "Console handoff claims Aureline as the authoritative control plane.",
            ));
        }
    }

    let passed = findings
        .iter()
        .all(|finding| finding.severity != InfraBoundaryFindingSeverity::Error);
    ClusterLiveResourceValidationReport {
        record_kind: "infra_cluster_context_and_live_resource_validation_report".to_string(),
        schema_version: CLUSTER_LIVE_RESOURCE_SCHEMA_VERSION,
        packet_id: packet.packet_id.clone(),
        passed,
        truth_modes,
        surfaces,
        tool_kinds,
        findings,
    }
}

/// Required truth modes for resource-rendering surfaces.
const REQUIRED_TRUTH_MODES: [TruthMode; 5] = [
    TruthMode::Desired,
    TruthMode::Rendered,
    TruthMode::Plan,
    TruthMode::Live,
    TruthMode::ProviderOverlay,
];

fn validate_strip(
    strip: &ClusterContextStrip,
    context: &EnvironmentContext,
    findings: &mut Vec<InfraBoundaryFinding>,
) {
    if strip.context_ref != context.context_id {
        findings.push(error(
            "strip_context",
            "Context strip points at a different environment context.",
        ));
    }
    let matches_identity = strip.provider == context.provider
        && strip.account_subscription == context.account_subscription_project
        && strip.cluster.as_deref() == context.cluster.as_deref()
        && strip.namespace.as_deref() == context.namespace.as_deref()
        && strip.region.as_deref() == context.region_zone.as_deref()
        && strip.tenant.as_deref() == context.tenant.as_deref();
    if !matches_identity {
        findings.push(error(
            "strip_identity",
            "Context strip does not match the environment-context target identity.",
        ));
    }
    if strip.execution_origin.trim().is_empty() {
        findings.push(error(
            "strip_execution_origin",
            "Context strip is missing the execution origin.",
        ));
    }
    if strip.credential_class != context.credential_handle_class {
        findings.push(error(
            "strip_credential_class",
            "Context strip credential class does not match the environment context.",
        ));
    }
}

/// Validation report emitted for a cluster-context and live-resource packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClusterLiveResourceValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Packet id validated.
    pub packet_id: String,
    /// True when no error-severity finding was emitted.
    pub passed: bool,
    /// Truth modes covered by surface views.
    pub truth_modes: BTreeSet<TruthMode>,
    /// Surfaces covered by the packet.
    pub surfaces: BTreeSet<OpsSurface>,
    /// Tool families covered by the packet.
    pub tool_kinds: BTreeSet<OpsToolKind>,
    /// Findings emitted during validation.
    pub findings: Vec<InfraBoundaryFinding>,
}

fn error(check_id: &str, message: &str) -> InfraBoundaryFinding {
    InfraBoundaryFinding {
        severity: InfraBoundaryFindingSeverity::Error,
        check_id: check_id.to_string(),
        message: message.to_string(),
    }
}

#[cfg(test)]
mod tests;
