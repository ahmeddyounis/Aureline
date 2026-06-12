//! Plan, dry-run, admission, and policy-check viewer packets.
//!
//! This module freezes the M5 viewer contract for planned and validated
//! infrastructure outputs. It keeps plan, diff, dry-run, admission, and
//! policy-check results labeled as [`TruthLayer::PlannedValidated`], requires
//! explicit tool identity/version plus target context on every viewer, blocks
//! hidden live-authority inheritance from file inspection, and preserves
//! review, incident, and support-export joins for later handoff or repair
//! workflows.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{
    source_intelligence_and_resource_relationships::{
        InfrastructureFamily, TargetContextField, TruthLayer,
        SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND,
    },
    target_context_and_control_plane_boundary::{
        ActionKind, ActionPosture, EnvironmentCompleteness, EnvironmentContext,
        InfraBoundaryFinding, InfraBoundaryFindingSeverity,
        CONTROL_PLANE_BOUNDARY_PACKET_RECORD_KIND,
    },
};

/// Schema version for plan and validation viewer packets.
pub const PLAN_AND_VALIDATION_VIEWER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind discriminator for [`PlanAndValidationViewerPacket`].
pub const PLAN_AND_VALIDATION_VIEWER_PACKET_RECORD_KIND: &str =
    "infra_plan_and_validation_viewer_packet";

/// JSON Schema reference for packet interchange.
pub const PLAN_AND_VALIDATION_VIEWER_SCHEMA_REF: &str =
    "schemas/infra/plan-and-validation-viewers.schema.json";

/// Reviewer-facing documentation reference.
pub const PLAN_AND_VALIDATION_VIEWER_DOC_REF: &str = "docs/infra/plan-and-validation-viewers.md";

/// Fixture corpus directory for plan and validation viewer drills.
pub const PLAN_AND_VALIDATION_VIEWER_FIXTURE_DIR: &str =
    "fixtures/infra/plan-and-validation-viewers";

/// Distinct planned or validated viewer class claimed by the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanValidationViewerKind {
    /// A plan summary or rendered change plan.
    Plan,
    /// A change diff viewer with planned target effects.
    Diff,
    /// A dry-run or simulation result.
    DryRun,
    /// An admission or preflight decision result.
    Admission,
    /// A policy or validation check result.
    PolicyCheck,
}

/// Stable outcome posture carried by one viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanValidationViewerResult {
    /// The result is ready for explicit review before follow-up.
    ReadyForReview,
    /// The validation or check passed.
    ValidationPassed,
    /// The result found changes or drift that require review.
    ChangesDetected,
    /// The result blocked or denied the requested action.
    Blocked,
}

/// Tool identity preserved for one plan or validation viewer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanValidationToolIdentity {
    /// Stable tool or engine name.
    pub tool_name: String,
    /// Tool or engine version captured with the result.
    pub tool_version: String,
    /// Invocation or adapter identity used to produce the result.
    pub invocation_identity: String,
}

/// One plan, diff, dry-run, admission, or policy-check viewer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanValidationViewerRecord {
    /// Stable viewer id.
    pub viewer_id: String,
    /// Viewer class represented by the record.
    pub viewer_kind: PlanValidationViewerKind,
    /// Infrastructure family the viewer applies to.
    pub family: InfrastructureFamily,
    /// Truth layer rendered by the viewer.
    pub truth_layer: TruthLayer,
    /// Outcome posture carried by the viewer.
    pub result: PlanValidationViewerResult,
    /// Shared environment-context ref backing the viewer.
    pub context_ref: String,
    /// Visible target-context fields preserved in the viewer.
    pub visible_target_fields: Vec<TargetContextField>,
    /// Tool identity and version preserved with the result.
    pub tool_identity: PlanValidationToolIdentity,
    /// Capture timestamp for the result.
    pub captured_at: String,
    /// Stable input or source ref used to produce the viewer.
    pub input_ref: String,
    /// Stable target object or target-scope ref described by the viewer.
    pub target_ref: String,
    /// True when follow-up can lead to mutation.
    pub can_lead_to_mutate: bool,
    /// True when explicit review is required before any mutate follow-up.
    pub review_before_apply_required: bool,
    /// Effective authority posture of the viewer itself.
    pub authority_posture: ActionPosture,
    /// True when the viewer was derived from repo-owned or static inputs.
    pub source_is_static_repo_material: bool,
    /// True when the viewer silently inherited live authority from inspection.
    pub inherits_live_authority_from_viewer: bool,
    /// Optional explicit handoff ref when the viewer ends at a provider boundary.
    pub handoff_ref: Option<String>,
    /// Export-safe viewer summary.
    pub support_summary: String,
}

/// Explicit source of authority for a follow-up gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewerAuthoritySourceClass {
    /// Follow-up requires a reviewed preview plus step-up inside Aureline.
    ReviewedPreviewStepUp,
    /// Follow-up leaves Aureline through an attributable provider handoff.
    ExplicitProviderHandoff,
}

/// Review gate for any follow-up that can lead to mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerFollowUpGate {
    /// Stable gate id.
    pub gate_id: String,
    /// Viewer ref that the gate is attached to.
    pub viewer_ref: String,
    /// Follow-up action class under review.
    pub action_kind: ActionKind,
    /// Shared environment-context ref backing the gate.
    pub context_ref: String,
    /// True when approval is required before follow-up.
    pub approval_required: bool,
    /// True when the viewer was explicitly reviewed as the preview surface.
    pub preview_reviewed: bool,
    /// Explicit source of authority for the follow-up.
    pub authority_source_class: ViewerAuthoritySourceClass,
    /// Stable ref for the approval, ticket, or handoff packet.
    pub authority_source_ref: String,
    /// Explicit handoff ref when the follow-up leaves Aureline authority.
    pub handoff_ref: Option<String>,
    /// True when the gate currently admits the follow-up.
    pub approved: bool,
    /// Export-safe gate summary.
    pub support_summary: String,
}

/// Consumer surface that must preserve viewer attribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewerConsumerSurface {
    /// Review sheet or approval workflow.
    Review,
    /// Incident timeline or repair workflow.
    Incident,
    /// Support export and evidence handoff surfaces.
    SupportExport,
}

/// Join proving a consumer preserves attribution for viewer outcomes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerConsumerJoin {
    /// Stable join id.
    pub join_id: String,
    /// Consumer surface covered by the join.
    pub surface: ViewerConsumerSurface,
    /// Packet id consumed by the surface.
    pub source_packet_ref: String,
    /// Viewer refs preserved by the surface.
    pub viewer_refs: Vec<String>,
    /// Gate refs preserved by the surface.
    pub gate_refs: Vec<String>,
    /// True when the surface reads the shared packet directly.
    pub uses_shared_packet: bool,
    /// True when tool identity and version remain visible.
    pub preserves_tool_identity: bool,
    /// True when exact target context remains visible.
    pub preserves_target_context: bool,
    /// True when the viewer timestamp remains visible.
    pub preserves_timestamp: bool,
    /// True when handoff or later-repair attribution remains visible.
    pub preserves_handoff_and_repair_attribution: bool,
    /// True when the join remains export-safe.
    pub export_safe_only: bool,
    /// True when the consumer minted hidden live authority.
    pub hidden_live_authority_created: bool,
    /// Export-safe join summary.
    pub support_summary: String,
}

/// Canonical packet for plan and validation viewers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanAndValidationViewerPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp for the packet.
    pub captured_at: String,
    /// Source-intelligence packet family that viewer refs build on.
    pub source_intelligence_packet_ref: String,
    /// Boundary packet family that target context and action safety build on.
    pub boundary_packet_ref: String,
    /// Shared environment contexts used by the viewers.
    pub environment_contexts: Vec<EnvironmentContext>,
    /// Concrete viewer records.
    pub viewer_records: Vec<PlanValidationViewerRecord>,
    /// Explicit review gates for mutate-adjacent follow-up.
    pub follow_up_gates: Vec<ViewerFollowUpGate>,
    /// Review, incident, and export joins that preserve attribution.
    pub consumer_joins: Vec<ViewerConsumerJoin>,
    /// Export-safe packet summary.
    pub support_summary: String,
}

impl PlanAndValidationViewerPacket {
    /// Validates the packet against the canonical viewer invariants.
    pub fn validate(&self) -> PlanAndValidationViewerValidationReport {
        validate_plan_and_validation_viewer_packet(self)
    }

    /// Resolves one viewer by stable id.
    pub fn viewer(&self, viewer_id: &str) -> Option<&PlanValidationViewerRecord> {
        self.viewer_records
            .iter()
            .find(|viewer| viewer.viewer_id == viewer_id)
    }

    /// Resolves one follow-up gate by stable id.
    pub fn gate(&self, gate_id: &str) -> Option<&ViewerFollowUpGate> {
        self.follow_up_gates
            .iter()
            .find(|gate| gate.gate_id == gate_id)
    }

    /// Resolves one consumer join by surface.
    pub fn consumer_join(&self, surface: ViewerConsumerSurface) -> Option<&ViewerConsumerJoin> {
        self.consumer_joins
            .iter()
            .find(|join| join.surface == surface)
    }
}

/// Validation report emitted for a viewer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanAndValidationViewerValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Packet id validated.
    pub packet_id: String,
    /// Whether no error-severity checks failed.
    pub passed: bool,
    /// Viewer kinds covered by the packet.
    pub viewer_kinds: BTreeSet<PlanValidationViewerKind>,
    /// Infrastructure families covered by the packet.
    pub families: BTreeSet<InfrastructureFamily>,
    /// Consumer surfaces covered by the packet.
    pub consumer_surfaces: BTreeSet<ViewerConsumerSurface>,
    /// Validation findings emitted by the packet.
    pub findings: Vec<InfraBoundaryFinding>,
}

/// Validates one plan and validation viewer packet.
pub fn validate_plan_and_validation_viewer_packet(
    packet: &PlanAndValidationViewerPacket,
) -> PlanAndValidationViewerValidationReport {
    let mut findings = Vec::new();
    let mut viewer_kinds = BTreeSet::new();
    let mut families = BTreeSet::new();
    let mut consumer_surfaces = BTreeSet::new();
    let mut context_ids = BTreeSet::new();
    let mut viewer_ids = BTreeSet::new();
    let mut gate_ids = BTreeSet::new();
    let mut join_ids = BTreeSet::new();

    if packet.record_kind != PLAN_AND_VALIDATION_VIEWER_PACKET_RECORD_KIND {
        findings.push(error(
            "record_kind",
            "Packet record_kind is not the plan-and-validation viewer discriminator.",
        ));
    }
    if packet.schema_version != PLAN_AND_VALIDATION_VIEWER_SCHEMA_VERSION {
        findings.push(error(
            "schema_version",
            "Packet schema_version does not match this crate.",
        ));
    }
    if packet.source_intelligence_packet_ref != SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND {
        findings.push(error(
            "source_intelligence_packet_ref",
            "Viewer packet must cite the canonical infrastructure object packet record kind.",
        ));
    }
    if packet.boundary_packet_ref != CONTROL_PLANE_BOUNDARY_PACKET_RECORD_KIND {
        findings.push(error(
            "boundary_packet_ref",
            "Viewer packet must cite the canonical boundary packet record kind.",
        ));
    }
    if packet.support_summary.trim().is_empty() {
        findings.push(error(
            "support_summary",
            "Viewer packet is missing a support summary.",
        ));
    }

    for context in &packet.environment_contexts {
        if !context_ids.insert(context.context_id.as_str()) {
            findings.push(error(
                "duplicate_context",
                "Viewer packet repeats the same environment context.",
            ));
        }
        if !context.ambient_context_prohibited {
            findings.push(error(
                "ambient_context",
                "Viewer packet environment context allows ambient inheritance.",
            ));
        }
        if context.completeness == EnvironmentCompleteness::Incomplete {
            findings.push(error(
                "context_completeness",
                "Viewer packet environment context is incomplete.",
            ));
        }
    }

    for viewer in &packet.viewer_records {
        viewer_kinds.insert(viewer.viewer_kind);
        families.insert(viewer.family);

        if !viewer_ids.insert(viewer.viewer_id.as_str()) {
            findings.push(error(
                "duplicate_viewer",
                "Viewer packet repeats the same viewer id.",
            ));
        }
        if !context_ids.contains(viewer.context_ref.as_str()) {
            findings.push(error(
                "missing_context_ref",
                "Viewer record references an unknown environment context.",
            ));
        }
        if viewer.truth_layer != TruthLayer::PlannedValidated {
            findings.push(error(
                "truth_layer",
                "Plan and validation viewers must render the planned/validated truth layer.",
            ));
        }
        if viewer.tool_identity.tool_name.trim().is_empty()
            || viewer.tool_identity.tool_version.trim().is_empty()
            || viewer.tool_identity.invocation_identity.trim().is_empty()
        {
            findings.push(error(
                "tool_identity",
                "Viewer record is missing tool identity, tool version, or invocation identity.",
            ));
        }
        if viewer.input_ref.trim().is_empty()
            || viewer.target_ref.trim().is_empty()
            || viewer.support_summary.trim().is_empty()
        {
            findings.push(error(
                "viewer_shape",
                "Viewer record is missing input, target, or support summary data.",
            ));
        }
        if viewer.visible_target_fields.is_empty() {
            findings.push(error(
                "target_fields",
                "Viewer record is missing visible target-context fields.",
            ));
        }
        validate_visible_target_fields(viewer, packet, &mut findings);
        if matches!(
            viewer.authority_posture,
            ActionPosture::StepUpRequired
                | ActionPosture::WriteApproved
                | ActionPosture::Blocked
                | ActionPosture::NotClaimed
        ) {
            findings.push(error(
                "viewer_authority_posture",
                "Viewer records may not imply direct live mutation authority.",
            ));
        }
        if viewer.inherits_live_authority_from_viewer {
            findings.push(error(
                "hidden_live_authority",
                "Plan and validation viewers may not silently inherit live authority.",
            ));
        }
        if viewer.can_lead_to_mutate && !viewer.review_before_apply_required {
            findings.push(error(
                "review_before_apply",
                "Mutate-adjacent viewers must require explicit review before apply.",
            ));
        }
        if viewer.handoff_ref.is_some() && viewer.authority_posture != ActionPosture::HandoffOnly {
            findings.push(error(
                "viewer_handoff_posture",
                "Viewer handoff refs require handoff-only authority posture.",
            ));
        }
        if viewer.source_is_static_repo_material
            && viewer.authority_posture == ActionPosture::WriteApproved
        {
            findings.push(error(
                "static_source_authority",
                "Static source material must never appear write-approved.",
            ));
        }
        if viewer.can_lead_to_mutate
            && !packet
                .follow_up_gates
                .iter()
                .any(|gate| gate.viewer_ref == viewer.viewer_id)
        {
            findings.push(error(
                "missing_follow_up_gate",
                "Mutate-adjacent viewers must have an explicit follow-up gate.",
            ));
        }
    }

    for required in REQUIRED_VIEWER_KINDS {
        if !viewer_kinds.contains(&required) {
            findings.push(error(
                "viewer_kind_coverage",
                "Viewer packet is missing a required viewer kind.",
            ));
        }
    }

    for gate in &packet.follow_up_gates {
        if !gate_ids.insert(gate.gate_id.as_str()) {
            findings.push(error(
                "duplicate_gate",
                "Viewer packet repeats the same follow-up gate id.",
            ));
        }
        let Some(viewer) = packet.viewer(&gate.viewer_ref) else {
            findings.push(error(
                "gate_viewer_ref",
                "Follow-up gate references an unknown viewer.",
            ));
            continue;
        };
        if !context_ids.contains(gate.context_ref.as_str()) {
            findings.push(error(
                "gate_context_ref",
                "Follow-up gate references an unknown environment context.",
            ));
        }
        if gate.context_ref != viewer.context_ref {
            findings.push(error(
                "gate_context_match",
                "Follow-up gate must reuse the viewer target context.",
            ));
        }
        if !viewer.can_lead_to_mutate {
            findings.push(error(
                "gate_non_mutating_viewer",
                "Only mutate-adjacent viewers may carry a follow-up gate.",
            ));
        }
        if !gate.approval_required {
            findings.push(error(
                "gate_approval_required",
                "Follow-up gates must require explicit approval.",
            ));
        }
        if !gate.preview_reviewed {
            findings.push(error(
                "gate_preview_reviewed",
                "Follow-up gates must prove the viewer was explicitly reviewed.",
            ));
        }
        if gate.authority_source_ref.trim().is_empty() || gate.support_summary.trim().is_empty() {
            findings.push(error(
                "gate_shape",
                "Follow-up gate is missing its authority source ref or support summary.",
            ));
        }
        match gate.authority_source_class {
            ViewerAuthoritySourceClass::ReviewedPreviewStepUp => {
                if gate.handoff_ref.is_some() {
                    findings.push(error(
                        "gate_unexpected_handoff",
                        "Step-up follow-up gates may not also claim a provider handoff.",
                    ));
                }
            }
            ViewerAuthoritySourceClass::ExplicitProviderHandoff => {
                if gate.handoff_ref.is_none() {
                    findings.push(error(
                        "gate_missing_handoff",
                        "Provider-handoff follow-up gates must cite a handoff ref.",
                    ));
                }
            }
        }
        if !matches!(
            gate.action_kind,
            ActionKind::Mutate | ActionKind::BrowserConsoleLaunch
        ) {
            findings.push(error(
                "gate_action_kind",
                "Plan and validation follow-up gates must stay mutate- or handoff-scoped.",
            ));
        }
    }

    for join in &packet.consumer_joins {
        consumer_surfaces.insert(join.surface);

        if !join_ids.insert(join.join_id.as_str()) {
            findings.push(error(
                "duplicate_join",
                "Viewer packet repeats the same consumer join id.",
            ));
        }
        if join.source_packet_ref != packet.packet_id {
            findings.push(error(
                "join_packet_ref",
                "Consumer join must point back to the packet id it preserves.",
            ));
        }
        if !join.uses_shared_packet {
            findings.push(error(
                "join_shared_packet",
                "Consumer join does not use the shared viewer packet.",
            ));
        }
        if !join.preserves_tool_identity
            || !join.preserves_target_context
            || !join.preserves_timestamp
            || !join.preserves_handoff_and_repair_attribution
        {
            findings.push(error(
                "join_attribution",
                "Consumer join dropped tool identity, target context, timestamp, or follow-up attribution.",
            ));
        }
        if !join.export_safe_only {
            findings.push(error(
                "join_export_safety",
                "Consumer join must remain export-safe.",
            ));
        }
        if join.hidden_live_authority_created {
            findings.push(error(
                "join_hidden_live_authority",
                "Consumer join created hidden live authority.",
            ));
        }
        if join.viewer_refs.is_empty() || join.gate_refs.is_empty() {
            findings.push(error(
                "join_shape",
                "Consumer join must preserve at least one viewer ref and one gate ref.",
            ));
        }
        for viewer_ref in &join.viewer_refs {
            if packet.viewer(viewer_ref).is_none() {
                findings.push(error(
                    "join_viewer_ref",
                    "Consumer join references an unknown viewer.",
                ));
            }
        }
        for gate_ref in &join.gate_refs {
            if packet.gate(gate_ref).is_none() {
                findings.push(error(
                    "join_gate_ref",
                    "Consumer join references an unknown follow-up gate.",
                ));
            }
        }
        if join.viewer_refs.len() != packet.viewer_records.len()
            || join.gate_refs.len() != packet.follow_up_gates.len()
        {
            findings.push(error(
                "join_coverage",
                "Consumer join must preserve every viewer and follow-up gate in the packet.",
            ));
        }
    }

    for required in REQUIRED_CONSUMER_SURFACES {
        if !consumer_surfaces.contains(&required) {
            findings.push(error(
                "consumer_surface_coverage",
                "Viewer packet is missing a required consumer join surface.",
            ));
        }
    }

    let passed = findings
        .iter()
        .all(|finding| finding.severity != InfraBoundaryFindingSeverity::Error);

    PlanAndValidationViewerValidationReport {
        record_kind: "infra_plan_and_validation_viewer_validation_report".to_string(),
        schema_version: PLAN_AND_VALIDATION_VIEWER_SCHEMA_VERSION,
        packet_id: packet.packet_id.clone(),
        passed,
        viewer_kinds,
        families,
        consumer_surfaces,
        findings,
    }
}

fn validate_visible_target_fields(
    viewer: &PlanValidationViewerRecord,
    packet: &PlanAndValidationViewerPacket,
    findings: &mut Vec<InfraBoundaryFinding>,
) {
    let required_base = [
        TargetContextField::Provider,
        TargetContextField::AccountSubscriptionProject,
        TargetContextField::ExecutionOrigin,
        TargetContextField::ToolIdentity,
        TargetContextField::FreshnessTimestamp,
    ];
    let fields = viewer
        .visible_target_fields
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    for required in required_base {
        if !fields.contains(&required) {
            findings.push(error(
                "target_field_minimums",
                "Viewer record omitted one of the required target-context fields.",
            ));
        }
    }

    let Some(context) = packet
        .environment_contexts
        .iter()
        .find(|context| context.context_id == viewer.context_ref)
    else {
        return;
    };

    if context.cluster.is_some() && !fields.contains(&TargetContextField::EnvironmentSelector) {
        findings.push(error(
            "target_field_cluster",
            "Viewer record omitted the environment selector for a cluster-scoped context.",
        ));
    }
    if context.namespace.is_some() && !fields.contains(&TargetContextField::NamespaceOrScope) {
        findings.push(error(
            "target_field_namespace",
            "Viewer record omitted namespace or scope for a scoped context.",
        ));
    }
    if context.region_zone.is_some() && !fields.contains(&TargetContextField::RegionZone) {
        findings.push(error(
            "target_field_region",
            "Viewer record omitted region or zone for a regionalized context.",
        ));
    }
    if context.tenant.is_some() && !fields.contains(&TargetContextField::Tenant) {
        findings.push(error(
            "target_field_tenant",
            "Viewer record omitted tenant for a tenant-scoped context.",
        ));
    }
    if !fields.contains(&TargetContextField::WorkspaceRevision) {
        findings.push(error(
            "target_field_workspace",
            "Viewer record omitted workspace revision from the visible target context.",
        ));
    }
}

fn error(check_id: &str, message: &str) -> InfraBoundaryFinding {
    InfraBoundaryFinding {
        severity: InfraBoundaryFindingSeverity::Error,
        check_id: check_id.to_string(),
        message: message.to_string(),
    }
}

const REQUIRED_VIEWER_KINDS: [PlanValidationViewerKind; 5] = [
    PlanValidationViewerKind::Plan,
    PlanValidationViewerKind::Diff,
    PlanValidationViewerKind::DryRun,
    PlanValidationViewerKind::Admission,
    PlanValidationViewerKind::PolicyCheck,
];

const REQUIRED_CONSUMER_SURFACES: [ViewerConsumerSurface; 3] = [
    ViewerConsumerSurface::Review,
    ViewerConsumerSurface::Incident,
    ViewerConsumerSurface::SupportExport,
];

/// Returns a deterministic qualified viewer packet for tests and fixtures.
pub fn seeded_plan_and_validation_viewer_packet() -> PlanAndValidationViewerPacket {
    use ActionPosture::{DryRunOnly, HandoffOnly, InspectOnly};

    fn context(
        context_id: &str,
        provider: &str,
        account: &str,
        cluster: Option<&str>,
        namespace: Option<&str>,
        region: Option<&str>,
        tenant: Option<&str>,
        execution_profile: &str,
        toolchain: &str,
        credential_class: &str,
        write_scope: &str,
        observed_at: &str,
    ) -> EnvironmentContext {
        EnvironmentContext {
            context_id: context_id.to_string(),
            provider: provider.to_string(),
            account_subscription_project: account.to_string(),
            cluster: cluster.map(str::to_string),
            namespace: namespace.map(str::to_string),
            region_zone: region.map(str::to_string),
            tenant: tenant.map(str::to_string),
            workspace_root: "workspace://checkout".to_string(),
            branch_worktree_or_commit: "refs/heads/main@3d3ea61".to_string(),
            execution_context_profile: execution_profile.to_string(),
            toolchain_cli_identity: toolchain.to_string(),
            credential_handle_class: credential_class.to_string(),
            issuance_source: "workspace-secret-broker".to_string(),
            expiry: Some("2026-06-12T22:00:00Z".to_string()),
            write_scope: write_scope.to_string(),
            observed_at: observed_at.to_string(),
            completeness: EnvironmentCompleteness::Complete,
            ambient_context_prohibited: true,
            high_risk: matches!(cluster, Some("cluster:payments-prod-eu1")),
        }
    }

    fn tool(
        tool_name: &str,
        tool_version: &str,
        invocation_identity: &str,
    ) -> PlanValidationToolIdentity {
        PlanValidationToolIdentity {
            tool_name: tool_name.to_string(),
            tool_version: tool_version.to_string(),
            invocation_identity: invocation_identity.to_string(),
        }
    }

    fn viewer(
        viewer_id: &str,
        viewer_kind: PlanValidationViewerKind,
        family: InfrastructureFamily,
        result: PlanValidationViewerResult,
        context_ref: &str,
        visible_target_fields: &[TargetContextField],
        tool_identity: PlanValidationToolIdentity,
        captured_at: &str,
        input_ref: &str,
        target_ref: &str,
        authority_posture: ActionPosture,
        handoff_ref: Option<&str>,
        support_summary: &str,
    ) -> PlanValidationViewerRecord {
        PlanValidationViewerRecord {
            viewer_id: viewer_id.to_string(),
            viewer_kind,
            family,
            truth_layer: TruthLayer::PlannedValidated,
            result,
            context_ref: context_ref.to_string(),
            visible_target_fields: visible_target_fields.to_vec(),
            tool_identity,
            captured_at: captured_at.to_string(),
            input_ref: input_ref.to_string(),
            target_ref: target_ref.to_string(),
            can_lead_to_mutate: true,
            review_before_apply_required: true,
            authority_posture,
            source_is_static_repo_material: true,
            inherits_live_authority_from_viewer: false,
            handoff_ref: handoff_ref.map(str::to_string),
            support_summary: support_summary.to_string(),
        }
    }

    fn gate(
        gate_id: &str,
        viewer_ref: &str,
        action_kind: ActionKind,
        context_ref: &str,
        authority_source_class: ViewerAuthoritySourceClass,
        authority_source_ref: &str,
        handoff_ref: Option<&str>,
        approved: bool,
        support_summary: &str,
    ) -> ViewerFollowUpGate {
        ViewerFollowUpGate {
            gate_id: gate_id.to_string(),
            viewer_ref: viewer_ref.to_string(),
            action_kind,
            context_ref: context_ref.to_string(),
            approval_required: true,
            preview_reviewed: true,
            authority_source_class,
            authority_source_ref: authority_source_ref.to_string(),
            handoff_ref: handoff_ref.map(str::to_string),
            approved,
            support_summary: support_summary.to_string(),
        }
    }

    fn join(
        join_id: &str,
        surface: ViewerConsumerSurface,
        source_packet_ref: &str,
        viewer_refs: &[&str],
        gate_refs: &[&str],
        support_summary: &str,
    ) -> ViewerConsumerJoin {
        ViewerConsumerJoin {
            join_id: join_id.to_string(),
            surface,
            source_packet_ref: source_packet_ref.to_string(),
            viewer_refs: viewer_refs
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            gate_refs: gate_refs.iter().map(|value| (*value).to_string()).collect(),
            uses_shared_packet: true,
            preserves_tool_identity: true,
            preserves_target_context: true,
            preserves_timestamp: true,
            preserves_handoff_and_repair_attribution: true,
            export_safe_only: true,
            hidden_live_authority_created: false,
            support_summary: support_summary.to_string(),
        }
    }

    let packet_id = "infra-viewers:checkout:2026-06-12".to_string();
    let shared_fields = vec![
        TargetContextField::Provider,
        TargetContextField::AccountSubscriptionProject,
        TargetContextField::EnvironmentSelector,
        TargetContextField::NamespaceOrScope,
        TargetContextField::RegionZone,
        TargetContextField::Tenant,
        TargetContextField::WorkspaceRevision,
        TargetContextField::ExecutionOrigin,
        TargetContextField::ToolIdentity,
        TargetContextField::FreshnessTimestamp,
    ];

    let viewer_ids = [
        "viewer:terraform:plan",
        "viewer:kubernetes:diff",
        "viewer:kubernetes:dry_run",
        "viewer:policy:admission",
        "viewer:ci:policy_check",
    ];
    let gate_ids = [
        "gate:terraform:apply",
        "gate:kubernetes:diff_apply",
        "gate:kubernetes:dry_run_apply",
        "gate:policy:admission_handoff",
        "gate:ci:policy_apply",
    ];

    PlanAndValidationViewerPacket {
        record_kind: PLAN_AND_VALIDATION_VIEWER_PACKET_RECORD_KIND.to_string(),
        schema_version: PLAN_AND_VALIDATION_VIEWER_SCHEMA_VERSION,
        packet_id: packet_id.clone(),
        captured_at: "2026-06-12T20:10:00Z".to_string(),
        source_intelligence_packet_ref: SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND.to_string(),
        boundary_packet_ref: CONTROL_PLANE_BOUNDARY_PACKET_RECORD_KIND.to_string(),
        environment_contexts: vec![
            context(
                "ctx:terraform:payments-prod",
                "aws",
                "account:payments-prod",
                Some("service:checkout"),
                Some("scope:runtime"),
                Some("eu-west-1"),
                Some("tenant:payments"),
                "exec.local.terraform",
                "terraform:v1.9.5",
                "delegated_read_only",
                "preview_then_ticketed_apply",
                "2026-06-12T20:02:00Z",
            ),
            context(
                "ctx:kubernetes:payments-prod",
                "kubernetes",
                "project:payments-prod",
                Some("cluster:payments-prod-eu1"),
                Some("namespace:checkout"),
                Some("eu-west-1"),
                Some("tenant:payments"),
                "exec.remote.ops",
                "kubectl:v1.31.0",
                "managed_connector_handle",
                "namespace_bound_pending_approval",
                "2026-06-12T20:03:00Z",
            ),
            context(
                "ctx:policy:payments-prod",
                "kubernetes",
                "project:payments-prod",
                Some("cluster:payments-prod-eu1"),
                Some("namespace:checkout"),
                Some("eu-west-1"),
                Some("tenant:payments"),
                "exec.remote.policy",
                "kyverno-cli:v1.14.1",
                "managed_connector_handle",
                "handoff_only",
                "2026-06-12T20:04:00Z",
            ),
            context(
                "ctx:ci:release",
                "github_actions",
                "org:payments",
                Some("workflow:release"),
                Some("env:prod"),
                Some("eu-west-1"),
                Some("tenant:payments"),
                "exec.ci.release",
                "conftest:v0.60.0",
                "ci_attestation_handle",
                "preview_then_ticketed_apply",
                "2026-06-12T20:05:00Z",
            ),
        ],
        viewer_records: vec![
            viewer(
                "viewer:terraform:plan",
                PlanValidationViewerKind::Plan,
                InfrastructureFamily::TerraformHcl,
                PlanValidationViewerResult::ReadyForReview,
                "ctx:terraform:payments-prod",
                &shared_fields,
                tool(
                    "terraform",
                    "1.9.5",
                    "terraform plan -out checkout.tfplan",
                ),
                "2026-06-12T20:02:30Z",
                "artifact://terraform/checkout.tfplan",
                "obj:tf:planned",
                DryRunOnly,
                None,
                "Terraform plan keeps target scope, tool version, and review-first apply posture explicit.",
            ),
            viewer(
                "viewer:kubernetes:diff",
                PlanValidationViewerKind::Diff,
                InfrastructureFamily::KubernetesHelm,
                PlanValidationViewerResult::ChangesDetected,
                "ctx:kubernetes:payments-prod",
                &shared_fields,
                tool("helm-diff", "3.9.11", "helm diff upgrade checkout"),
                "2026-06-12T20:03:30Z",
                "artifact://helm/diff/checkout",
                "obj:k8s:planned",
                DryRunOnly,
                None,
                "Helm diff remains planned truth with explicit cluster and namespace scope.",
            ),
            viewer(
                "viewer:kubernetes:dry_run",
                PlanValidationViewerKind::DryRun,
                InfrastructureFamily::KubernetesHelm,
                PlanValidationViewerResult::ReadyForReview,
                "ctx:kubernetes:payments-prod",
                &shared_fields,
                tool(
                    "kubectl",
                    "1.31.0",
                    "kubectl apply --server-side --dry-run=server",
                ),
                "2026-06-12T20:03:45Z",
                "artifact://kubectl/dry-run/checkout",
                "obj:k8s:planned",
                DryRunOnly,
                None,
                "Server-side dry run stays review-first and does not grant live authority.",
            ),
            viewer(
                "viewer:policy:admission",
                PlanValidationViewerKind::Admission,
                InfrastructureFamily::PolicyManifest,
                PlanValidationViewerResult::Blocked,
                "ctx:policy:payments-prod",
                &shared_fields,
                tool(
                    "kyverno-admission",
                    "1.14.1",
                    "kyverno apply checkout-admission",
                ),
                "2026-06-12T20:04:10Z",
                "artifact://policy/admission/checkout",
                "obj:policy:planned",
                HandoffOnly,
                Some("handoff:policy-console:checkout"),
                "Admission result stays attributable even when the repair path hands off to the policy console.",
            ),
            viewer(
                "viewer:ci:policy_check",
                PlanValidationViewerKind::PolicyCheck,
                InfrastructureFamily::CiEnvironment,
                PlanValidationViewerResult::ValidationPassed,
                "ctx:ci:release",
                &shared_fields,
                tool(
                    "conftest",
                    "0.60.0",
                    "conftest test .github/workflows/release.yml",
                ),
                "2026-06-12T20:05:20Z",
                "artifact://ci/policy-check/release",
                "obj:ci:planned",
                InspectOnly,
                None,
                "CI policy-check output keeps source revision, target environment, and tool identity visible before rollout.",
            ),
        ],
        follow_up_gates: vec![
            gate(
                "gate:terraform:apply",
                "viewer:terraform:plan",
                ActionKind::Mutate,
                "ctx:terraform:payments-prod",
                ViewerAuthoritySourceClass::ReviewedPreviewStepUp,
                "approval:terraform-change:checkout",
                None,
                false,
                "Terraform apply remains blocked pending explicit review and approval.",
            ),
            gate(
                "gate:kubernetes:diff_apply",
                "viewer:kubernetes:diff",
                ActionKind::Mutate,
                "ctx:kubernetes:payments-prod",
                ViewerAuthoritySourceClass::ReviewedPreviewStepUp,
                "approval:kubernetes-diff:checkout",
                None,
                false,
                "Helm diff follow-up requires the reviewed diff plus explicit approval.",
            ),
            gate(
                "gate:kubernetes:dry_run_apply",
                "viewer:kubernetes:dry_run",
                ActionKind::Mutate,
                "ctx:kubernetes:payments-prod",
                ViewerAuthoritySourceClass::ReviewedPreviewStepUp,
                "approval:kubernetes-dry-run:checkout",
                None,
                false,
                "Dry-run follow-up stays bound to the same cluster and namespace context.",
            ),
            gate(
                "gate:policy:admission_handoff",
                "viewer:policy:admission",
                ActionKind::BrowserConsoleLaunch,
                "ctx:policy:payments-prod",
                ViewerAuthoritySourceClass::ExplicitProviderHandoff,
                "ticket:policy-console:checkout",
                Some("handoff:policy-console:checkout"),
                false,
                "Admission repair leaves Aureline through an explicit, attributable policy-console handoff.",
            ),
            gate(
                "gate:ci:policy_apply",
                "viewer:ci:policy_check",
                ActionKind::Mutate,
                "ctx:ci:release",
                ViewerAuthoritySourceClass::ReviewedPreviewStepUp,
                "approval:ci-release-policy:checkout",
                None,
                false,
                "Release policy gates keep rollout approval and preview linked to the policy-check viewer.",
            ),
        ],
        consumer_joins: vec![
            join(
                "join:review",
                ViewerConsumerSurface::Review,
                &packet_id,
                &viewer_ids,
                &gate_ids,
                "Review workflows preserve every plan and validation viewer plus its follow-up gate.",
            ),
            join(
                "join:incident",
                ViewerConsumerSurface::Incident,
                &packet_id,
                &viewer_ids,
                &gate_ids,
                "Incident repair flows preserve every viewer result, gate, and handoff breadcrumb for later investigation.",
            ),
            join(
                "join:support_export",
                ViewerConsumerSurface::SupportExport,
                &packet_id,
                &viewer_ids,
                &gate_ids,
                "Support export preserves metadata-only plan and validation attribution without embedding raw provider payloads.",
            ),
        ],
        support_summary:
            "Canonical plan and validation viewer packet keeps tool identity, target context, review-before-apply posture, and export-safe attribution explicit.".to_string(),
    }
}
