//! Target-discovery confidence cards and boundary export projections.
//!
//! This module projects the canonical [`crate::execution_context::ExecutionContext`]
//! into cards that launch surfaces, review packets, and support exports can
//! render without re-deriving target truth. The resolver remains the owner of
//! target identity, precedence, confidence, prebuild, and helper-version
//! posture; this module only turns that record into the small inspection
//! surfaces required before a run-capable action proceeds.

use serde::{Deserialize, Serialize};

use crate::execution_context::{
    ConfidenceLevel, ExecutionContext, ExecutionContextExplanation, ExecutionRouteOrigin,
    MixedVersionDriftState, PrebuildReuseState, ReachabilityState, ResolverInputDecision,
    ResolverInputField, SurfaceClass, TargetClass, TargetConfidenceReason,
};
use crate::provenance::{
    dedupe_context_provenance, ExecutionEventProvenance, ExecutionProvenanceEvent,
    ExecutionProvenanceEventClass,
};
use crate::TrustState;

/// Schema version for target-confidence card, export, and review packets.
pub const TARGET_CONFIDENCE_ALPHA_SCHEMA_VERSION: u32 = 1;
/// Stable record-kind tag for one target-confidence card.
pub const TARGET_CONFIDENCE_CARD_RECORD_KIND: &str = "target_confidence_card_record";
/// Stable record-kind tag for support-export target confidence packets.
pub const TARGET_CONFIDENCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "target_confidence_support_export_record";
/// Stable record-kind tag for review packets that quote target confidence.
pub const TARGET_CONFIDENCE_REVIEW_PACKET_RECORD_KIND: &str =
    "target_confidence_review_packet_record";

/// Launch lane class used by target-confidence cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetConfidenceLaneClass {
    /// Work runs on the local desktop host.
    Local,
    /// Work crosses a helper, remote, container, sandbox, or managed boundary.
    HelperBacked,
}

impl TargetConfidenceLaneClass {
    /// Stable token recorded in cards, support exports, and review packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::HelperBacked => "helper_backed",
        }
    }
}

/// Frozen target-discovery confidence class quoted by cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetDiscoveryConfidenceClass {
    /// Declarative target metadata names the selected target exactly.
    CanonicalDeclared,
    /// A materialized instance round-trips to a canonical target identity.
    CanonicalMaterialised,
    /// Probes matched the declared capability and target envelope.
    ProbedConsistent,
    /// Probes or helper metadata exposed drift or unchecked capability truth.
    ProbedDivergent,
    /// Ambient signals were enough to continue, but not enough for authority.
    InferredFromAmbient,
    /// Multiple plausible targets require explicit user selection.
    UnresolvedRequiresUser,
    /// The resolver was unavailable, so no target may be launched.
    ResolverUnavailable,
}

impl TargetDiscoveryConfidenceClass {
    /// Stable token recorded in cards, support exports, and review packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalDeclared => "canonical_declared",
            Self::CanonicalMaterialised => "canonical_materialised",
            Self::ProbedConsistent => "probed_consistent",
            Self::ProbedDivergent => "probed_divergent",
            Self::InferredFromAmbient => "inferred_from_ambient",
            Self::UnresolvedRequiresUser => "unresolved_requires_user",
            Self::ResolverUnavailable => "resolver_unavailable",
        }
    }
}

/// Frozen host-boundary cue class quoted by target-confidence cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostBoundaryCueClass {
    /// Fact came from the local host kernel that renders the surface.
    LocalHostBoundary,
    /// Fact came from a user-mode sandbox on the same kernel.
    UserModeSandboxBoundary,
    /// Fact crossed a container kernel namespace.
    ContainerKernelBoundary,
    /// Fact crossed a declared devcontainer boundary.
    DevcontainerBoundary,
    /// Fact was produced on an SSH remote.
    RemoteSshBoundary,
    /// Fact came from a remote agent or helper attach service.
    RemoteAgentBoundary,
    /// Fact came from a managed workspace runtime.
    ManagedWorkspaceBoundary,
    /// Fact came from a notebook kernel runtime.
    NotebookKernelBoundary,
    /// Fact came from an AI sandbox or tool-call runtime.
    AiSandboxBoundary,
    /// Fact returned from a browser handoff.
    BrowserHandoffReturnBoundary,
    /// Fact came from a compatibility-bridged host.
    BridgedHostBoundary,
}

impl HostBoundaryCueClass {
    /// Stable token recorded in cards, support exports, and review packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHostBoundary => "local_host_boundary",
            Self::UserModeSandboxBoundary => "user_mode_sandbox_boundary",
            Self::ContainerKernelBoundary => "container_kernel_boundary",
            Self::DevcontainerBoundary => "devcontainer_boundary",
            Self::RemoteSshBoundary => "remote_ssh_boundary",
            Self::RemoteAgentBoundary => "remote_agent_boundary",
            Self::ManagedWorkspaceBoundary => "managed_workspace_boundary",
            Self::NotebookKernelBoundary => "notebook_kernel_boundary",
            Self::AiSandboxBoundary => "ai_sandbox_boundary",
            Self::BrowserHandoffReturnBoundary => "browser_handoff_return_boundary",
            Self::BridgedHostBoundary => "bridged_host_boundary",
        }
    }

    /// Short label safe for launch cards and export summaries.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalHostBoundary => "Local host",
            Self::UserModeSandboxBoundary => "User-mode sandbox",
            Self::ContainerKernelBoundary => "Container",
            Self::DevcontainerBoundary => "Devcontainer",
            Self::RemoteSshBoundary => "SSH remote",
            Self::RemoteAgentBoundary => "Remote helper",
            Self::ManagedWorkspaceBoundary => "Managed workspace",
            Self::NotebookKernelBoundary => "Notebook kernel",
            Self::AiSandboxBoundary => "AI sandbox",
            Self::BrowserHandoffReturnBoundary => "Browser handoff",
            Self::BridgedHostBoundary => "Bridged host",
        }
    }
}

/// One explanation row rendered inside a target-confidence card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetConfidenceExplanationRow {
    /// Dotted path from the source execution-context explanation.
    pub field_path: String,
    /// Resolver effect token.
    pub effect_token: String,
    /// Resolver reason-code token.
    pub reason_code_token: String,
    /// Resolver source token.
    pub source_token: String,
    /// Token form of the resolved state.
    pub resolved_value_token: String,
    /// Export-safe summary for UI, CLI, review, and support surfaces.
    pub summary: String,
}

/// Card shown before a launchable surface dispatches work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetConfidenceCard {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable card id.
    pub card_id: String,
    /// Local or helper-backed lane class.
    pub lane: TargetConfidenceLaneClass,
    /// Stable lane token.
    pub lane_token: String,
    /// Workspace id from the source context.
    pub workspace_id: String,
    /// Command id from the source context.
    pub command_id: String,
    /// Canonical execution-context id.
    pub execution_context_ref: String,
    /// Resolver provenance record id.
    pub provenance_record_ref: String,
    /// Invoking surface class.
    pub surface: SurfaceClass,
    /// Stable invoking-surface token.
    pub surface_token: String,
    /// Resolved target class.
    pub target_class: TargetClass,
    /// Stable target-class token.
    pub target_class_token: String,
    /// Canonical target id.
    pub target_id: String,
    /// Target reachability state.
    pub reachability_state: ReachabilityState,
    /// Stable reachability token.
    pub reachability_state_token: String,
    /// Target-discovery confidence class for the selected target.
    pub discovery_confidence_class: TargetDiscoveryConfidenceClass,
    /// Stable discovery-confidence token.
    pub discovery_confidence_token: String,
    /// Coarse resolver confidence level.
    pub target_confidence_level: ConfidenceLevel,
    /// Stable resolver confidence token.
    pub target_confidence_level_token: String,
    /// Structured resolver confidence reasons.
    pub target_confidence_reason_tokens: Vec<String>,
    /// Divergence or inference reasons required for non-authoritative claims.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub divergence_or_inference_reason_tokens: Vec<String>,
    /// Host-boundary cue class.
    pub host_boundary_cue: HostBoundaryCueClass,
    /// Stable host-boundary cue token.
    pub host_boundary_cue_token: String,
    /// Short host-boundary label.
    pub host_boundary_label: String,
    /// Ordered cue stack, outermost-to-innermost.
    pub host_boundary_cue_stack_tokens: Vec<String>,
    /// True when surfaces must display a boundary cue before dispatch.
    pub host_boundary_visible: bool,
    /// Route-origin label for the selected action route.
    pub route_origin: ExecutionRouteOrigin,
    /// Stable route-origin token.
    pub route_origin_token: String,
    /// Short route-origin label.
    pub route_origin_label: String,
    /// Transport label for the selected route.
    pub route_transport_label: String,
    /// Tunnel session ref when the action traverses a tunnel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tunnel_session_ref: Option<String>,
    /// Target identity ref preserved for route reconstruction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_target_identity_ref: Option<String>,
    /// Source token that won target selection precedence.
    pub selected_by_source_token: String,
    /// Resolver explanation rows that justify the selected target.
    pub chosen_because: Vec<TargetConfidenceExplanationRow>,
    /// Action ref for the shared execution-context inspector.
    pub inspect_action_ref: String,
    /// Action ref for changing target selection before dispatch.
    pub change_target_action_ref: String,
    /// True because raw paths, command lines, environment bodies, and secrets are excluded.
    pub redaction_safe: bool,
}

impl TargetConfidenceCard {
    /// Builds a target-confidence card from the canonical execution context.
    pub fn from_context(context: &ExecutionContext) -> Self {
        let lane = lane_for_target(context.target_identity.target_class);
        let host_boundary_cue = host_boundary_cue_for_target(context.target_identity.target_class);
        let route_origin = route_origin_for_context(context);
        let discovery_confidence_class = discovery_confidence_class(context);
        let target_decision = context
            .provenance
            .input_decisions
            .iter()
            .find(|decision| decision.field == ResolverInputField::TargetClass);
        let selected_by_source_token = target_decision
            .map(|decision| decision.winning_source.as_str().to_owned())
            .unwrap_or_else(|| "resolver_unavailable".to_owned());
        let host_boundary_cue_stack_tokens = host_boundary_stack_tokens(host_boundary_cue);
        let target_confidence_reason_tokens = context
            .target_confidence
            .reasons
            .iter()
            .map(|reason| reason.as_str().to_owned())
            .collect::<Vec<_>>();
        let divergence_or_inference_reason_tokens =
            divergence_or_inference_reason_tokens(context, discovery_confidence_class);
        Self {
            record_kind: TARGET_CONFIDENCE_CARD_RECORD_KIND.to_owned(),
            schema_version: TARGET_CONFIDENCE_ALPHA_SCHEMA_VERSION,
            card_id: format!(
                "target-confidence-card:{}",
                stable_token(&context.execution_context_id)
            ),
            lane,
            lane_token: lane.as_str().to_owned(),
            workspace_id: context.invocation_subject.workspace_id.clone(),
            command_id: context.invocation_subject.command_id.clone(),
            execution_context_ref: context.execution_context_id.clone(),
            provenance_record_ref: context.provenance.provenance_record_id.clone(),
            surface: context.invocation_subject.surface,
            surface_token: context.invocation_subject.surface.as_str().to_owned(),
            target_class: context.target_identity.target_class,
            target_class_token: context.target_identity.target_class.as_str().to_owned(),
            target_id: context.target_identity.canonical_target_id.clone(),
            reachability_state: context.target_identity.reachability_state,
            reachability_state_token: context
                .target_identity
                .reachability_state
                .as_str()
                .to_owned(),
            discovery_confidence_class,
            discovery_confidence_token: discovery_confidence_class.as_str().to_owned(),
            target_confidence_level: context.target_confidence.level,
            target_confidence_level_token: context.target_confidence.level.as_str().to_owned(),
            target_confidence_reason_tokens,
            divergence_or_inference_reason_tokens,
            host_boundary_cue,
            host_boundary_cue_token: host_boundary_cue.as_str().to_owned(),
            host_boundary_label: host_boundary_cue.label().to_owned(),
            host_boundary_cue_stack_tokens,
            host_boundary_visible: context.target_identity.local_vs_managed_boundary_visible
                || host_boundary_cue == HostBoundaryCueClass::LocalHostBoundary,
            route_origin_token: route_origin.route_class_token.clone(),
            route_origin_label: route_origin.route_label.clone(),
            route_transport_label: route_origin.transport_label.clone(),
            tunnel_session_ref: route_origin.tunnel_session_ref.clone(),
            route_target_identity_ref: route_origin.target_identity_ref.clone(),
            route_origin,
            selected_by_source_token,
            chosen_because: card_explanation_rows(context, target_decision),
            inspect_action_ref: format!(
                "action:execution-context:inspect:{}",
                context.execution_context_id
            ),
            change_target_action_ref: format!(
                "action:execution-context:switch-target:{}:{}",
                stable_token(&context.invocation_subject.workspace_id),
                context.invocation_subject.surface.as_str()
            ),
            redaction_safe: true,
        }
    }

    /// Returns one deterministic support/export line for this card.
    pub fn summary_line(&self) -> String {
        format!(
            "lane={}; context={}; target={}({}); discovery={}; confidence={}; boundary={}; route={}; inspect={}; change={}",
            self.lane_token,
            self.execution_context_ref,
            self.target_id,
            self.target_class_token,
            self.discovery_confidence_token,
            self.target_confidence_level_token,
            self.host_boundary_cue_token,
            self.route_origin_token,
            self.inspect_action_ref,
            self.change_target_action_ref,
        )
    }
}

/// Host-boundary row included in exports and review packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetHostBoundaryRow {
    /// Card that owns this boundary row.
    pub card_ref: String,
    /// Local or helper-backed lane token.
    pub lane_token: String,
    /// Canonical execution-context id.
    pub execution_context_ref: String,
    /// Canonical target id.
    pub target_id: String,
    /// Stable target-class token.
    pub target_class_token: String,
    /// Stable host-boundary cue token.
    pub host_boundary_cue_token: String,
    /// Short host-boundary label.
    pub host_boundary_label: String,
    /// Ordered cue stack, outermost-to-innermost.
    pub host_boundary_cue_stack_tokens: Vec<String>,
    /// Stable route-origin token.
    pub route_origin_token: String,
    /// Short route-origin label.
    pub route_origin_label: String,
    /// Transport label for the selected route.
    pub route_transport_label: String,
}

impl TargetHostBoundaryRow {
    /// Builds a boundary row from a target-confidence card.
    pub fn from_card(card: &TargetConfidenceCard) -> Self {
        Self {
            card_ref: card.card_id.clone(),
            lane_token: card.lane_token.clone(),
            execution_context_ref: card.execution_context_ref.clone(),
            target_id: card.target_id.clone(),
            target_class_token: card.target_class_token.clone(),
            host_boundary_cue_token: card.host_boundary_cue_token.clone(),
            host_boundary_label: card.host_boundary_label.clone(),
            host_boundary_cue_stack_tokens: card.host_boundary_cue_stack_tokens.clone(),
            route_origin_token: card.route_origin_token.clone(),
            route_origin_label: card.route_origin_label.clone(),
            route_transport_label: card.route_transport_label.clone(),
        }
    }
}

/// Support-export projection for target confidence and host-boundary truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetConfidenceSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Workspace id shared by the exported contexts.
    pub workspace_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Target-confidence cards included in the export.
    pub cards: Vec<TargetConfidenceCard>,
    /// Boundary rows copied out for support workflows.
    pub host_boundaries: Vec<TargetHostBoundaryRow>,
    /// Redaction-safe execution-context provenance for every card.
    pub context_provenance: Vec<ExecutionEventProvenance>,
    /// Support-export provenance events carrying the same context objects.
    pub context_provenance_events: Vec<ExecutionProvenanceEvent>,
    /// True because raw paths, command lines, environment bodies, and secrets are excluded.
    pub redaction_safe: bool,
}

impl TargetConfidenceSupportExport {
    /// Builds a support-export packet from canonical execution contexts.
    pub fn from_contexts<'a>(
        support_export_id: impl Into<String>,
        generated_at: impl Into<String>,
        contexts: impl IntoIterator<Item = &'a ExecutionContext>,
    ) -> Self {
        let support_export_id = support_export_id.into();
        let generated_at = generated_at.into();
        let contexts = contexts.into_iter().collect::<Vec<_>>();
        let cards = contexts
            .iter()
            .map(|context| TargetConfidenceCard::from_context(context))
            .collect::<Vec<_>>();
        let host_boundaries = cards
            .iter()
            .map(TargetHostBoundaryRow::from_card)
            .collect::<Vec<_>>();
        let context_provenance = dedupe_context_provenance(
            contexts
                .iter()
                .map(|context| ExecutionEventProvenance::from_context(context)),
        );
        let context_provenance_events = context_provenance
            .iter()
            .map(|provenance| {
                ExecutionProvenanceEvent::new(
                    format!(
                        "execution-provenance-event:target-confidence-export:{}:{}",
                        stable_token(&support_export_id),
                        provenance.context_provenance_id
                    ),
                    ExecutionProvenanceEventClass::SupportExport,
                    support_export_id.clone(),
                    generated_at.clone(),
                    provenance.clone(),
                )
            })
            .collect();
        let workspace_id = cards
            .first()
            .map(|card| card.workspace_id.clone())
            .unwrap_or_default();
        Self {
            record_kind: TARGET_CONFIDENCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TARGET_CONFIDENCE_ALPHA_SCHEMA_VERSION,
            support_export_id,
            workspace_id,
            generated_at,
            cards,
            host_boundaries,
            context_provenance,
            context_provenance_events,
            redaction_safe: true,
        }
    }

    /// Renders stable plaintext lines for CLI or support review.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!(
            "Target confidence support export: {}\n",
            self.support_export_id
        );
        for card in &self.cards {
            out.push_str(&card.summary_line());
            out.push('\n');
        }
        out
    }
}

/// One review row produced before dispatch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetConfidenceReviewRow {
    /// Card that owns this review row.
    pub card_ref: String,
    /// Local or helper-backed lane token.
    pub lane_token: String,
    /// Canonical execution-context id.
    pub execution_context_ref: String,
    /// Stable discovery-confidence token.
    pub discovery_confidence_token: String,
    /// Stable resolver confidence token.
    pub target_confidence_level_token: String,
    /// Stable host-boundary cue token.
    pub host_boundary_cue_token: String,
    /// Short host-boundary label.
    pub host_boundary_label: String,
    /// True when this row should be reviewed before dispatch.
    pub review_before_dispatch: bool,
    /// Action ref for the shared execution-context inspector.
    pub inspect_action_ref: String,
    /// Action ref for changing target selection before dispatch.
    pub change_target_action_ref: String,
    /// Export-safe reason for the review posture.
    pub review_summary: String,
}

impl TargetConfidenceReviewRow {
    /// Builds a review row from a target-confidence card.
    pub fn from_card(card: &TargetConfidenceCard) -> Self {
        let review_before_dispatch = card.lane == TargetConfidenceLaneClass::HelperBacked
            || card.target_confidence_level != ConfidenceLevel::High
            || !card.divergence_or_inference_reason_tokens.is_empty();
        let review_summary = if review_before_dispatch {
            format!(
                "Review target {} before dispatch because discovery={}, confidence={}, boundary={}.",
                card.target_id,
                card.discovery_confidence_token,
                card.target_confidence_level_token,
                card.host_boundary_cue_token
            )
        } else {
            format!(
                "Target {} is local with discovery={} and confidence={}.",
                card.target_id, card.discovery_confidence_token, card.target_confidence_level_token
            )
        };
        Self {
            card_ref: card.card_id.clone(),
            lane_token: card.lane_token.clone(),
            execution_context_ref: card.execution_context_ref.clone(),
            discovery_confidence_token: card.discovery_confidence_token.clone(),
            target_confidence_level_token: card.target_confidence_level_token.clone(),
            host_boundary_cue_token: card.host_boundary_cue_token.clone(),
            host_boundary_label: card.host_boundary_label.clone(),
            review_before_dispatch,
            inspect_action_ref: card.inspect_action_ref.clone(),
            change_target_action_ref: card.change_target_action_ref.clone(),
            review_summary,
        }
    }
}

/// Review packet preserving target confidence and boundary labels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetConfidenceReviewPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable review-packet id.
    pub review_packet_id: String,
    /// Workspace id shared by the reviewed contexts.
    pub workspace_id: String,
    /// Packet creation timestamp.
    pub created_at: String,
    /// Target-confidence cards included in the packet.
    pub cards: Vec<TargetConfidenceCard>,
    /// Boundary rows copied out for review workflows.
    pub host_boundaries: Vec<TargetHostBoundaryRow>,
    /// Review rows preserving inspect/change actions.
    pub review_rows: Vec<TargetConfidenceReviewRow>,
    /// True when any row needs review before dispatch.
    pub review_required_before_dispatch: bool,
    /// True because raw paths, command lines, environment bodies, and secrets are excluded.
    pub redaction_safe: bool,
}

impl TargetConfidenceReviewPacket {
    /// Builds a review packet from canonical execution contexts.
    pub fn from_contexts<'a>(
        review_packet_id: impl Into<String>,
        created_at: impl Into<String>,
        contexts: impl IntoIterator<Item = &'a ExecutionContext>,
    ) -> Self {
        let cards = contexts
            .into_iter()
            .map(TargetConfidenceCard::from_context)
            .collect::<Vec<_>>();
        Self::from_cards(review_packet_id, created_at, cards)
    }

    /// Builds a review packet from precomputed target-confidence cards.
    pub fn from_cards(
        review_packet_id: impl Into<String>,
        created_at: impl Into<String>,
        cards: Vec<TargetConfidenceCard>,
    ) -> Self {
        let host_boundaries = cards
            .iter()
            .map(TargetHostBoundaryRow::from_card)
            .collect::<Vec<_>>();
        let review_rows = cards
            .iter()
            .map(TargetConfidenceReviewRow::from_card)
            .collect::<Vec<_>>();
        let review_required_before_dispatch =
            review_rows.iter().any(|row| row.review_before_dispatch);
        let workspace_id = cards
            .first()
            .map(|card| card.workspace_id.clone())
            .unwrap_or_default();
        Self {
            record_kind: TARGET_CONFIDENCE_REVIEW_PACKET_RECORD_KIND.to_owned(),
            schema_version: TARGET_CONFIDENCE_ALPHA_SCHEMA_VERSION,
            review_packet_id: review_packet_id.into(),
            workspace_id,
            created_at: created_at.into(),
            cards,
            host_boundaries,
            review_rows,
            review_required_before_dispatch,
            redaction_safe: true,
        }
    }

    /// Renders stable plaintext lines for CLI or support review.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!(
            "Target confidence review packet: {}\n",
            self.review_packet_id
        );
        for row in &self.review_rows {
            out.push_str(&format!(
                "lane={}; context={}; discovery={}; confidence={}; boundary={}; review={}; inspect={}; change={}\n",
                row.lane_token,
                row.execution_context_ref,
                row.discovery_confidence_token,
                row.target_confidence_level_token,
                row.host_boundary_cue_token,
                row.review_before_dispatch,
                row.inspect_action_ref,
                row.change_target_action_ref,
            ));
        }
        out
    }
}

fn lane_for_target(target_class: TargetClass) -> TargetConfidenceLaneClass {
    if target_class == TargetClass::LocalHost {
        TargetConfidenceLaneClass::Local
    } else {
        TargetConfidenceLaneClass::HelperBacked
    }
}

fn host_boundary_cue_for_target(target_class: TargetClass) -> HostBoundaryCueClass {
    match target_class {
        TargetClass::LocalHost => HostBoundaryCueClass::LocalHostBoundary,
        TargetClass::SshRemote => HostBoundaryCueClass::RemoteSshBoundary,
        TargetClass::ContainerLocal => HostBoundaryCueClass::ContainerKernelBoundary,
        TargetClass::Devcontainer => HostBoundaryCueClass::DevcontainerBoundary,
        TargetClass::RemoteWorkspaceVm => HostBoundaryCueClass::RemoteAgentBoundary,
        TargetClass::PrebuildRuntime | TargetClass::ManagedWorkspace => {
            HostBoundaryCueClass::ManagedWorkspaceBoundary
        }
        TargetClass::NotebookKernelLocal | TargetClass::NotebookKernelRemote => {
            HostBoundaryCueClass::NotebookKernelBoundary
        }
        TargetClass::AiSandbox => HostBoundaryCueClass::AiSandboxBoundary,
    }
}

fn host_boundary_stack_tokens(cue: HostBoundaryCueClass) -> Vec<String> {
    if cue == HostBoundaryCueClass::LocalHostBoundary {
        vec![cue.as_str().to_owned()]
    } else {
        vec![
            HostBoundaryCueClass::LocalHostBoundary.as_str().to_owned(),
            cue.as_str().to_owned(),
        ]
    }
}

fn route_origin_for_context(context: &ExecutionContext) -> ExecutionRouteOrigin {
    context.route_origin.clone().unwrap_or_else(|| {
        ExecutionRouteOrigin::for_target(
            context.target_identity.target_class,
            context.target_identity.canonical_target_id.clone(),
        )
    })
}

fn discovery_confidence_class(context: &ExecutionContext) -> TargetDiscoveryConfidenceClass {
    if context.target_confidence.reasons.iter().any(|reason| {
        matches!(
            reason,
            TargetConfidenceReason::ResolverFallbackTarget
                | TargetConfidenceReason::TrustPending
                | TargetConfidenceReason::PolicyBlockedReachability
        )
    }) {
        return TargetDiscoveryConfidenceClass::InferredFromAmbient;
    }

    if context.target_confidence.reasons.iter().any(|reason| {
        matches!(
            reason,
            TargetConfidenceReason::CapsuleDrift
                | TargetConfidenceReason::MixedVersionUnchecked
                | TargetConfidenceReason::ConflictingTargetSources
        )
    }) {
        return TargetDiscoveryConfidenceClass::ProbedDivergent;
    }

    if context.target_identity.target_class == TargetClass::PrebuildRuntime
        && context.prebuild_metadata.reuse_state == PrebuildReuseState::Reused
    {
        return TargetDiscoveryConfidenceClass::CanonicalDeclared;
    }

    if context.target_identity.target_class == TargetClass::LocalHost {
        TargetDiscoveryConfidenceClass::CanonicalMaterialised
    } else {
        TargetDiscoveryConfidenceClass::ProbedConsistent
    }
}

fn divergence_or_inference_reason_tokens(
    context: &ExecutionContext,
    discovery_confidence_class: TargetDiscoveryConfidenceClass,
) -> Vec<String> {
    let requires_reasons = matches!(
        discovery_confidence_class,
        TargetDiscoveryConfidenceClass::ProbedDivergent
            | TargetDiscoveryConfidenceClass::InferredFromAmbient
            | TargetDiscoveryConfidenceClass::UnresolvedRequiresUser
            | TargetDiscoveryConfidenceClass::ResolverUnavailable
    );
    if !requires_reasons {
        return Vec::new();
    }

    let mut tokens = Vec::new();
    for reason in &context.target_confidence.reasons {
        if matches!(
            reason,
            TargetConfidenceReason::ResolverFallbackTarget
                | TargetConfidenceReason::ConflictingTargetSources
                | TargetConfidenceReason::TrustPending
                | TargetConfidenceReason::TrustRestricted
                | TargetConfidenceReason::PolicyBlockedReachability
                | TargetConfidenceReason::CapsuleDrift
                | TargetConfidenceReason::MixedVersionUnchecked
                | TargetConfidenceReason::RemoteOrManagedBoundary
        ) {
            push_unique_token(&mut tokens, reason.as_str());
        }
    }
    if context.mixed_version_drift.state == MixedVersionDriftState::NotNegotiated {
        push_unique_token(&mut tokens, context.mixed_version_drift.reason.as_str());
    }
    if context.policy_and_trust.trust_state != TrustState::Trusted {
        push_unique_token(&mut tokens, context.policy_and_trust.trust_state.as_str());
    }
    tokens
}

fn card_explanation_rows(
    context: &ExecutionContext,
    target_decision: Option<&ResolverInputDecision>,
) -> Vec<TargetConfidenceExplanationRow> {
    let mut rows = Vec::new();
    if let Some(decision) = target_decision {
        rows.push(target_decision_row(decision));
    }

    for explanation in &context.explanations {
        if is_card_explanation(explanation) {
            rows.push(explanation_row(explanation));
        }
    }
    rows
}

fn target_decision_row(decision: &ResolverInputDecision) -> TargetConfidenceExplanationRow {
    let summary = if decision.conflicting_sources.is_empty() {
        format!(
            "Target class resolved to {} from {}.",
            decision.resolved_value_token,
            decision.winning_source.as_str()
        )
    } else {
        let conflicts = decision
            .conflicting_sources
            .iter()
            .map(|source| source.as_str())
            .collect::<Vec<_>>()
            .join("|");
        format!(
            "Target class resolved to {} from {}; lower-precedence sources differed: {}.",
            decision.resolved_value_token,
            decision.winning_source.as_str(),
            conflicts
        )
    };
    TargetConfidenceExplanationRow {
        field_path: "resolver_input.target_class".to_owned(),
        effect_token: "selected_by_precedence".to_owned(),
        reason_code_token: reason_code_token_for_source(decision.winning_source.as_str())
            .to_owned(),
        source_token: decision.winning_source.as_str().to_owned(),
        resolved_value_token: decision.resolved_value_token.clone(),
        summary,
    }
}

fn explanation_row(explanation: &ExecutionContextExplanation) -> TargetConfidenceExplanationRow {
    TargetConfidenceExplanationRow {
        field_path: explanation.field_path.clone(),
        effect_token: explanation.effect.as_str().to_owned(),
        reason_code_token: explanation.reason_code.as_str().to_owned(),
        source_token: explanation.source.as_str().to_owned(),
        resolved_value_token: explanation.resolved_value_token.clone(),
        summary: explanation_summary(explanation),
    }
}

fn explanation_summary(explanation: &ExecutionContextExplanation) -> String {
    match explanation.field_path.as_str() {
        "target_identity.local_vs_managed_boundary_visible" => format!(
            "Host boundary visibility resolved to {}.",
            explanation.resolved_value_token
        ),
        "prebuild_metadata.reuse_state" => format!(
            "Prebuild reuse state resolved to {}.",
            explanation.resolved_value_token
        ),
        "mixed_version_drift.state" => format!(
            "Helper/client version posture resolved to {}.",
            explanation.resolved_value_token
        ),
        "policy_and_trust.trust_state" => {
            format!(
                "Trust state resolved to {}.",
                explanation.resolved_value_token
            )
        }
        "policy_and_trust.policy_epoch" => {
            format!(
                "Policy posture resolved to {}.",
                explanation.resolved_value_token
            )
        }
        _ => format!(
            "{} resolved to {}.",
            explanation.field_path, explanation.resolved_value_token
        ),
    }
}

fn is_card_explanation(explanation: &ExecutionContextExplanation) -> bool {
    matches!(
        explanation.field_path.as_str(),
        "target_identity.local_vs_managed_boundary_visible"
            | "prebuild_metadata.reuse_state"
            | "mixed_version_drift.state"
            | "policy_and_trust.trust_state"
            | "policy_and_trust.policy_epoch"
    )
}

fn reason_code_token_for_source(source_token: &str) -> &'static str {
    match source_token {
        "explicit_override" => "explicit_override_won",
        "surface_requested" => "surface_request_won",
        "workspace_default" => "workspace_default_won",
        _ => "resolver_fallback_used",
    }
}

fn push_unique_token(tokens: &mut Vec<String>, token: &str) {
    if !tokens.iter().any(|known| known == token) {
        tokens.push(token.to_owned());
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
    use crate::execution_context::{
        CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest,
        ExecutionContextResolver, ExecutionContextResolverConfig, ExecutionRouteOrigin,
        IdentityMode, ScopeClass, ToolchainClass,
    };

    fn resolver() -> ExecutionContextResolver {
        ExecutionContextResolver::new(ExecutionContextResolverConfig {
            workspace_id: "workspace:target-confidence".to_owned(),
            profile_id: Some("profile:default".to_owned()),
            identity_mode: IdentityMode::AccountFreeLocal,
            policy_epoch: 7,
            workspace_default_target_class: TargetClass::LocalHost,
            workspace_default_working_directory: Some("/Users/example/private/project".to_owned()),
            workspace_default_scope_class: ScopeClass::CurrentRoot,
            local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
            environment_capsule_ref: EnvironmentCapsuleRef {
                capsule_id: "capsule:target-confidence".to_owned(),
                capsule_hash: "sha256:target-confidence".to_owned(),
                resolved_schema_version: "1".to_owned(),
                drift_state: CapsuleDriftState::InSync,
            },
            resolver_version: "target-confidence-test".to_owned(),
        })
    }

    fn local_and_helper_contexts() -> (ExecutionContext, ExecutionContext) {
        let mut resolver = resolver();
        let local = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run.local",
            TrustState::Trusted,
            "2026-05-13T19:40:00Z",
        ));
        let mut helper_request = ExecutionContextRequest::task_seed(
            "task.run.helper",
            TrustState::Restricted,
            "2026-05-13T19:41:00Z",
        );
        helper_request.requested_target_class = Some(TargetClass::ManagedWorkspace);
        helper_request.requested_toolchain_class = Some(ToolchainClass::BuildDriverRuntime);
        let helper = resolver.resolve(helper_request);
        (local, helper)
    }

    #[test]
    fn cards_display_local_and_helper_boundary_truth() {
        let (local, helper) = local_and_helper_contexts();
        let local_card = TargetConfidenceCard::from_context(&local);
        let helper_card = TargetConfidenceCard::from_context(&helper);

        assert_eq!(local_card.lane, TargetConfidenceLaneClass::Local);
        assert_eq!(
            local_card.discovery_confidence_class,
            TargetDiscoveryConfidenceClass::CanonicalMaterialised
        );
        assert_eq!(
            local_card.host_boundary_cue,
            HostBoundaryCueClass::LocalHostBoundary
        );
        assert_eq!(local_card.target_confidence_level, ConfidenceLevel::High);
        assert_eq!(
            local_card.selected_by_source_token,
            "surface_requested".to_owned()
        );
        assert!(local_card
            .inspect_action_ref
            .contains(&local.execution_context_id));
        assert!(local_card
            .chosen_because
            .iter()
            .any(|row| row.field_path == "resolver_input.target_class"));

        assert_eq!(helper_card.lane, TargetConfidenceLaneClass::HelperBacked);
        assert_eq!(
            helper_card.discovery_confidence_class,
            TargetDiscoveryConfidenceClass::ProbedDivergent
        );
        assert_eq!(
            helper_card.host_boundary_cue,
            HostBoundaryCueClass::ManagedWorkspaceBoundary
        );
        assert_eq!(helper_card.host_boundary_label, "Managed workspace");
        assert_eq!(helper_card.target_confidence_level, ConfidenceLevel::Medium);
        assert!(helper_card
            .target_confidence_reason_tokens
            .contains(&"remote_or_managed_boundary".to_owned()));
        assert!(helper_card
            .divergence_or_inference_reason_tokens
            .contains(&"mixed_version_unchecked".to_owned()));
        assert!(helper_card
            .divergence_or_inference_reason_tokens
            .contains(&"helper_boundary_not_negotiated".to_owned()));
        assert!(helper_card
            .chosen_because
            .iter()
            .any(|row| row.reason_code_token == "helper_boundary_not_negotiated"));
    }

    #[test]
    fn support_export_and_review_packet_preserve_host_boundary_labels() {
        let (local, helper) = local_and_helper_contexts();
        let export = TargetConfidenceSupportExport::from_contexts(
            "support-export:target-confidence",
            "2026-05-13T19:42:00Z",
            [&local, &helper],
        );

        assert_eq!(
            export.record_kind,
            TARGET_CONFIDENCE_SUPPORT_EXPORT_RECORD_KIND
        );
        assert_eq!(export.cards.len(), 2);
        assert_eq!(export.host_boundaries.len(), 2);
        assert!(export
            .host_boundaries
            .iter()
            .any(|row| row.host_boundary_cue_token == "local_host_boundary"));
        assert!(export
            .host_boundaries
            .iter()
            .any(|row| row.host_boundary_cue_token == "managed_workspace_boundary"));
        assert_eq!(export.context_provenance.len(), 2);
        let rendered = export.render_plaintext();
        assert!(rendered.contains("boundary=managed_workspace_boundary"));
        assert!(!rendered.contains("/Users/example/private/project"));

        let review = TargetConfidenceReviewPacket::from_contexts(
            "review:target-confidence",
            "2026-05-13T19:43:00Z",
            [&local, &helper],
        );
        assert_eq!(
            review.record_kind,
            TARGET_CONFIDENCE_REVIEW_PACKET_RECORD_KIND
        );
        assert!(review.review_required_before_dispatch);
        assert!(review
            .review_rows
            .iter()
            .any(|row| row.lane_token == "helper_backed"
                && row.host_boundary_cue_token == "managed_workspace_boundary"
                && row.review_before_dispatch));
        assert!(review.render_plaintext().contains("change=action:"));
    }

    #[test]
    fn tunneled_target_card_keeps_remote_boundary_and_tunnel_route() {
        let mut resolver = resolver();
        let mut request = ExecutionContextRequest::local_terminal_seed(
            "terminal.open.tunnel",
            TrustState::Trusted,
            "2026-05-13T19:44:00Z",
        );
        request.override_target_class = Some(TargetClass::SshRemote);
        request.override_working_directory = Some("/srv/code");
        let context =
            resolver
                .resolve(request)
                .with_route_origin(ExecutionRouteOrigin::tunnel_exposed(
                    "SSH tunnel",
                    "tunnel.session.target_confidence.0001",
                    "target.ssh_remote.target_confidence",
                ));

        let card = TargetConfidenceCard::from_context(&context);

        assert_eq!(card.target_class, TargetClass::SshRemote);
        assert_eq!(
            card.host_boundary_cue,
            HostBoundaryCueClass::RemoteSshBoundary
        );
        assert_eq!(card.route_origin_token, "tunnel_exposed_route");
        assert_eq!(card.route_origin_label, "Tunnel route");
        assert_eq!(card.route_transport_label, "SSH tunnel");
        assert_eq!(
            card.tunnel_session_ref.as_deref(),
            Some("tunnel.session.target_confidence.0001")
        );
        assert_eq!(
            card.host_boundary_cue_stack_tokens,
            vec!["local_host_boundary", "remote_ssh_boundary"]
        );
    }

    #[test]
    fn fixture_replays_local_and_helper_target_confidence_cards() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/runtime/target_confidence_alpha/local_and_helper_cards.json");
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
        let fixture: TargetConfidenceFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()));

        assert_eq!(fixture.record_kind, "target_confidence_alpha_case");
        assert_eq!(
            fixture.schema_version,
            TARGET_CONFIDENCE_ALPHA_SCHEMA_VERSION
        );

        let (local, helper) = local_and_helper_contexts();
        let export = TargetConfidenceSupportExport::from_contexts(
            &fixture.support_export_id,
            &fixture.generated_at,
            [&local, &helper],
        );
        let review = TargetConfidenceReviewPacket::from_contexts(
            &fixture.review_packet_id,
            &fixture.generated_at,
            [&local, &helper],
        );

        assert_eq!(export.cards.len(), fixture.expect.card_count);
        for expected in &fixture.expect.cards {
            let card = export
                .cards
                .iter()
                .find(|card| card.lane_token == expected.lane_token)
                .unwrap_or_else(|| panic!("missing card lane {}", expected.lane_token));
            assert_eq!(
                card.discovery_confidence_token,
                expected.discovery_confidence_token
            );
            assert_eq!(
                card.host_boundary_cue_token,
                expected.host_boundary_cue_token
            );
            assert_eq!(
                card.target_confidence_level_token,
                expected.target_confidence_level_token
            );
            for reason in &expected.required_reason_tokens {
                assert!(
                    card.target_confidence_reason_tokens.contains(reason)
                        || card.divergence_or_inference_reason_tokens.contains(reason),
                    "missing reason {reason} on {}",
                    expected.lane_token
                );
            }
        }
        assert_eq!(
            review.review_required_before_dispatch,
            fixture.expect.review_required_before_dispatch
        );
        assert!(export.redaction_safe);
        assert!(review.redaction_safe);
    }

    #[derive(Debug, Deserialize)]
    struct TargetConfidenceFixture {
        record_kind: String,
        schema_version: u32,
        generated_at: String,
        support_export_id: String,
        review_packet_id: String,
        expect: TargetConfidenceFixtureExpect,
    }

    #[derive(Debug, Deserialize)]
    struct TargetConfidenceFixtureExpect {
        card_count: usize,
        cards: Vec<TargetConfidenceFixtureCard>,
        review_required_before_dispatch: bool,
    }

    #[derive(Debug, Deserialize)]
    struct TargetConfidenceFixtureCard {
        lane_token: String,
        discovery_confidence_token: String,
        host_boundary_cue_token: String,
        target_confidence_level_token: String,
        #[serde(default)]
        required_reason_tokens: Vec<String>,
    }
}
