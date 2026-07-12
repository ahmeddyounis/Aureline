//! Implemented M5 AI-message-card and evidence-timeline primitives.
//!
//! The frozen [editor-inline component matrix][matrix] names the reusable editor / review / AI inline
//! UI components and locks their controlled vocabulary. This module is the fourth and final implement
//! lane over that matrix (after the [editor-tab / gutter lane][tabgutter], the
//! [diagnostic-decoration / code-action-chip lane][diagchip], and the
//! [diff-view / review-thread lane][diffreview]): it turns the two inline *AI-evidence* components — the
//! **AI message card** and the **evidence timeline** — into resolvers that produce export-safe, honest
//! projections, so a user can read what an AI message means (its lifecycle state, source context,
//! confidence / uncertainty class, route / provider locality, spend / cost posture, and available safe
//! actions) and what an evidence trail records (timestamp, evidence kind, tool / validation lineage,
//! related run / change / resource, and open / replay / export actions) *without* that truth collapsing
//! into one generic completed chat bubble, hiding approval state, drifting an evidence pointer, or being
//! flattened into an opaque log across the editor, review, notebook, AI, support, and export consumers.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render AI message cards with source context, confidence or uncertainty class, route / provider or
//!   locality, spend / cost posture where claimed, and available safe actions.**
//!   [`resolve_ai_message_card`] refuses to read as a clean card when the message identity is unstated,
//!   the lifecycle state is unresolved or encoded generically as one completed message, the approval
//!   state is hidden, the confidence is unstated, the source context is unresolved or undisclosed, the
//!   route / provider locality is unresolved or implicit, the spend posture is unresolved or undisclosed,
//!   no safe actions are offered, or no command-backed detail path is reachable; it degrades instead.
//! * **Render evidence timelines with timestamp, evidence kind, tool or validation lineage, related
//!   run / change / resource, and open / replay / export actions that remain truthful under redaction.**
//!   [`resolve_evidence_timeline`] degrades when the entry identity is unstated, the timestamp is
//!   missing, the evidence kind is unresolved, the lineage is unresolved or unstated, no related resource
//!   is named, the disclosure state is unresolved, a redacted / partial timeline hides that it is
//!   incomplete, the trail is an opaque log rather than an inspectable structure, no open / replay /
//!   export action is offered, or no command-backed detail path is reachable.
//! * **Keep draft, streaming, review-required, blocked-by-policy, applied, reverted, failed, and
//!   stale-evidence states explicit instead of implying one generic completed message.** The packet
//!   proves, by resolved examples, that the same message and evidence vocabulary holds across surfaces,
//!   that a user can inspect source context, approval state, and supporting evidence before trusting an
//!   AI output, and that timeline / export consumers preserve lineage and redaction truth rather than
//!   flattening AI history into unstructured logs.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5EditorInlineDisposition`] inline-disposition vocabulary, the [`M5AiConfidence`] AI-confidence
//! vocabulary, and the [`M5EvidenceDisclosure`] evidence-disclosure vocabulary — so editor, review,
//! notebook, AI, support, and export surfaces can never fork their own confidence or evidence wording.
//! Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_editor_inline_component_matrix
//! [tabgutter]: crate::m5_editor_tab_and_gutter_state_and_marker_layering
//! [diagchip]: crate::m5_diagnostic_decoration_and_code_action_chip_state_and_fix_posture
//! [diffreview]: crate::m5_diff_view_and_review_thread_anchor_durability_and_review_state

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_ai_evidence_controls, seeded_m5_ai_evidence_controls_ai_ui_beta_narrowed,
    seeded_m5_ai_evidence_controls_support_export_preview_narrowed,
    M5_AI_EVIDENCE_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_editor_inline_component_matrix::{
    M5AiConfidence, M5EditorInlineAccessibilityRoute, M5EditorInlineComponentFamily,
    M5EditorInlineConsumerSurface, M5EditorInlineDeploymentLine, M5EditorInlineDisposition,
    M5EditorInlineDowngradeTrigger, M5EditorInlineQualificationClass, M5EditorInlineRequiredLabel,
    M5EvidenceDisclosure, M5_AI_MESSAGE_CARD_SCHEMA_REF, M5_EDITOR_INLINE_COMPONENT_DOC_REF,
    M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF, M5_EVIDENCE_TIMELINE_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5AiEvidenceControlsPacket`].
pub const M5_AI_EVIDENCE_CONTROLS_RECORD_KIND: &str =
    "implement_m5_ai_message_card_and_evidence_timeline_controls";

/// Schema version for M5 AI-message-card / evidence-timeline controls records.
pub const M5_AI_EVIDENCE_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_AI_EVIDENCE_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-ai-message-card-evidence-timeline-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_AI_EVIDENCE_CONTROLS_DOC_REF: &str =
    "docs/editor/m5_ai_message_card_and_evidence_timeline_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AI_EVIDENCE_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-ai-message-card-evidence-timeline-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_AI_EVIDENCE_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-ai-message-card-evidence-timeline-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_AI_EVIDENCE_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-ai-message-card-evidence-timeline-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_AI_EVIDENCE_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-ai-message-card-evidence-timeline-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface taxonomy
/// so no lane invents a parallel surface set.
pub type M5AiEvidenceConsumerSurface = M5EditorInlineConsumerSurface;

/// Controlled AI-message lifecycle state a card names, so a draft, streaming, review-required,
/// blocked-by-policy, applied, reverted, failed, or stale-evidence message is never collapsed into one
/// generic completed message. Minted by this lane to carry the exact state list the AI-message-card
/// acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiMessageState {
    /// An unsent draft message.
    Draft,
    /// A message still streaming in.
    Streaming,
    /// A message whose action requires review before it takes effect.
    ReviewRequired,
    /// A message whose action is blocked by policy.
    BlockedByPolicy,
    /// A message whose action has been applied.
    Applied,
    /// A message whose action has been reverted.
    Reverted,
    /// A message whose action failed.
    Failed,
    /// A message whose supporting evidence has gone stale.
    StaleEvidence,
    /// The message state cannot currently be resolved.
    StateUnknown,
}

impl M5AiMessageState {
    /// Every AI-message state, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Draft,
        Self::Streaming,
        Self::ReviewRequired,
        Self::BlockedByPolicy,
        Self::Applied,
        Self::Reverted,
        Self::Failed,
        Self::StaleEvidence,
        Self::StateUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Streaming => "streaming",
            Self::ReviewRequired => "review_required",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::Applied => "applied",
            Self::Reverted => "reverted",
            Self::Failed => "failed",
            Self::StaleEvidence => "stale_evidence",
            Self::StateUnknown => "state_unknown",
        }
    }

    /// Whether this state requires approval and must never read as already applied without disclosure.
    pub const fn needs_approval(self) -> bool {
        matches!(self, Self::ReviewRequired | Self::BlockedByPolicy)
    }

    /// Whether this state names stale supporting evidence.
    pub const fn is_stale_evidence(self) -> bool {
        matches!(self, Self::StaleEvidence)
    }

    /// Whether the message state is known (not the unknown sentinel).
    pub const fn is_known(self) -> bool {
        !matches!(self, Self::StateUnknown)
    }
}

/// Controlled AI source-context class a card names, so a message grounded in a model prior, retrieved
/// externally, or with no source cited is never mistaken for a workspace-grounded answer. Minted by this
/// lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiSourceContext {
    /// Grounded in the current workspace.
    GroundedInWorkspace,
    /// Grounded in project / product documentation.
    GroundedInDocs,
    /// Produced from the model prior only, with no external grounding.
    ModelPriorOnly,
    /// Retrieved from an external source.
    RetrievedExternal,
    /// No source is cited.
    NoSourceCited,
    /// The source context cannot currently be resolved.
    SourceUnresolved,
}

impl M5AiSourceContext {
    /// Every source context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::GroundedInWorkspace,
        Self::GroundedInDocs,
        Self::ModelPriorOnly,
        Self::RetrievedExternal,
        Self::NoSourceCited,
        Self::SourceUnresolved,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GroundedInWorkspace => "grounded_in_workspace",
            Self::GroundedInDocs => "grounded_in_docs",
            Self::ModelPriorOnly => "model_prior_only",
            Self::RetrievedExternal => "retrieved_external",
            Self::NoSourceCited => "no_source_cited",
            Self::SourceUnresolved => "source_unresolved",
        }
    }

    /// Whether this context must disclose that the message is not grounded in the local workspace.
    pub const fn needs_disclosure(self) -> bool {
        matches!(
            self,
            Self::ModelPriorOnly | Self::RetrievedExternal | Self::NoSourceCited
        )
    }

    /// Whether the source context is resolved (not the unresolved sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::SourceUnresolved)
    }
}

/// Controlled AI route / provider locality a card names, so the local-versus-hosted-provider distinction
/// stays explicit and desktop, browser handoff, and exported AI packets never drift on where an answer
/// was produced. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiRouteLocality {
    /// Produced by a locally hosted model.
    LocalModel,
    /// Produced by a hosted provider.
    HostedProvider,
    /// Served from a locally mirrored cache.
    MirroredCache,
    /// Produced by a bring-your-own-key hosted provider.
    ByoKeyProvider,
    /// Produced offline from a replayed transcript.
    OfflineReplay,
    /// The route / provider locality cannot currently be resolved.
    LocalityUnresolved,
}

impl M5AiRouteLocality {
    /// Every route locality, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalModel,
        Self::HostedProvider,
        Self::MirroredCache,
        Self::ByoKeyProvider,
        Self::OfflineReplay,
        Self::LocalityUnresolved,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalModel => "local_model",
            Self::HostedProvider => "hosted_provider",
            Self::MirroredCache => "mirrored_cache",
            Self::ByoKeyProvider => "byo_key_provider",
            Self::OfflineReplay => "offline_replay",
            Self::LocalityUnresolved => "locality_unresolved",
        }
    }

    /// Whether the message was produced by a hosted provider.
    pub const fn is_hosted(self) -> bool {
        matches!(self, Self::HostedProvider | Self::ByoKeyProvider)
    }

    /// Whether the route locality is resolved (not the unresolved sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::LocalityUnresolved)
    }
}

/// Controlled AI spend / cost posture a card names where claimed, so a metered or over-budget message is
/// never presented as free. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiSpendPosture {
    /// No metered cost.
    NoCost,
    /// Metered against local compute.
    MeteredLocal,
    /// Metered against a hosted provider.
    MeteredHosted,
    /// Metered but capped by a spend budget.
    BudgetCapped,
    /// Over the spend budget.
    OverBudget,
    /// The spend posture cannot currently be resolved.
    SpendUnresolved,
}

impl M5AiSpendPosture {
    /// Every spend posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoCost,
        Self::MeteredLocal,
        Self::MeteredHosted,
        Self::BudgetCapped,
        Self::OverBudget,
        Self::SpendUnresolved,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoCost => "no_cost",
            Self::MeteredLocal => "metered_local",
            Self::MeteredHosted => "metered_hosted",
            Self::BudgetCapped => "budget_capped",
            Self::OverBudget => "over_budget",
            Self::SpendUnresolved => "spend_unresolved",
        }
    }

    /// Whether this posture must disclose a metered / capped / over-budget cost rather than read as free.
    pub const fn needs_disclosure(self) -> bool {
        matches!(
            self,
            Self::MeteredHosted | Self::BudgetCapped | Self::OverBudget
        )
    }

    /// Whether the spend posture is resolved (not the unresolved sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::SpendUnresolved)
    }
}

/// Controlled evidence kind an evidence-timeline entry names, so a tool invocation, validation run,
/// retrieval, user edit, or external reference is never flattened into a generic log line. Minted by
/// this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceKind {
    /// A tool invocation.
    ToolInvocation,
    /// A validation / test run.
    ValidationRun,
    /// A retrieval / search.
    Retrieval,
    /// A user edit.
    UserEdit,
    /// An external reference.
    ExternalReference,
    /// The evidence kind cannot currently be resolved.
    KindUnresolved,
}

impl M5EvidenceKind {
    /// Every evidence kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ToolInvocation,
        Self::ValidationRun,
        Self::Retrieval,
        Self::UserEdit,
        Self::ExternalReference,
        Self::KindUnresolved,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ToolInvocation => "tool_invocation",
            Self::ValidationRun => "validation_run",
            Self::Retrieval => "retrieval",
            Self::UserEdit => "user_edit",
            Self::ExternalReference => "external_reference",
            Self::KindUnresolved => "kind_unresolved",
        }
    }

    /// Whether the evidence kind is resolved (not the unresolved sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::KindUnresolved)
    }
}

/// Controlled evidence lineage class an evidence-timeline entry names, so a user can see the tool /
/// validation / run / change / resource lineage behind a piece of evidence rather than an opaque log.
/// Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceLineageClass {
    /// Lineage back to the tool that produced it.
    ToolLineage,
    /// Lineage back to the validation that produced it.
    ValidationLineage,
    /// Lineage back to the run it belongs to.
    RunLineage,
    /// Lineage back to the change it belongs to.
    ChangeLineage,
    /// Lineage back to the resource it belongs to.
    ResourceLineage,
    /// The lineage cannot currently be resolved.
    LineageUnresolved,
}

impl M5EvidenceLineageClass {
    /// Every lineage class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ToolLineage,
        Self::ValidationLineage,
        Self::RunLineage,
        Self::ChangeLineage,
        Self::ResourceLineage,
        Self::LineageUnresolved,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ToolLineage => "tool_lineage",
            Self::ValidationLineage => "validation_lineage",
            Self::RunLineage => "run_lineage",
            Self::ChangeLineage => "change_lineage",
            Self::ResourceLineage => "resource_lineage",
            Self::LineageUnresolved => "lineage_unresolved",
        }
    }

    /// Whether the lineage class is resolved (not the unresolved sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::LineageUnresolved)
    }
}

/// One mandatory rendered part an AI message card or evidence timeline must be able to show, so no
/// state, source-context, confidence, route-locality, spend, evidence-kind, lineage, or redaction fact
/// is left implicit behind compact chrome, a tooltip, or an opaque log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiEvidenceAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed inline disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The AI source context (card).
    SourceContext,
    /// The AI confidence / uncertainty class (card).
    Confidence,
    /// The AI route / provider locality (card).
    RouteLocality,
    /// The AI spend / cost posture (card).
    SpendPosture,
    /// The available safe actions (card).
    SafeActions,
    /// The evidence kind (timeline).
    EvidenceKind,
    /// The evidence lineage class (timeline).
    LineageClass,
    /// The redaction / disclosure state (timeline).
    RedactionDisclosure,
    /// The command-backed path to trace the message or evidence (both components).
    StateCommand,
}

impl M5AiEvidenceAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::SourceContext,
        Self::Confidence,
        Self::RouteLocality,
        Self::SpendPosture,
        Self::SafeActions,
        Self::EvidenceKind,
        Self::LineageClass,
        Self::RedactionDisclosure,
        Self::StateCommand,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::SourceContext => "source_context",
            Self::Confidence => "confidence",
            Self::RouteLocality => "route_locality",
            Self::SpendPosture => "spend_posture",
            Self::SafeActions => "safe_actions",
            Self::EvidenceKind => "evidence_kind",
            Self::LineageClass => "lineage_class",
            Self::RedactionDisclosure => "redaction_disclosure",
            Self::StateCommand => "state_command",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route to trust an AI message
/// or trace an evidence trail behind a degraded component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiEvidenceNextAction {
    /// Open the command-backed component detail.
    OpenComponentDetail,
    /// Inspect the source context and approval state before trusting the message.
    InspectSourceAndApproval,
    /// Review the controlled message state.
    ReviewMessageState,
    /// Inspect the evidence lineage behind the timeline.
    InspectEvidenceLineage,
    /// Preserve the redaction truth of the timeline.
    PreserveRedactionTruth,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5AiEvidenceNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenComponentDetail,
        Self::InspectSourceAndApproval,
        Self::ReviewMessageState,
        Self::InspectEvidenceLineage,
        Self::PreserveRedactionTruth,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenComponentDetail => "open_component_detail",
            Self::InspectSourceAndApproval => "inspect_source_and_approval",
            Self::ReviewMessageState => "review_message_state",
            Self::InspectEvidenceLineage => "inspect_evidence_lineage",
            Self::PreserveRedactionTruth => "preserve_redaction_truth",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiEvidenceExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The inline dispositions carried.
    Dispositions,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The AI message state named by the card.
    MessageState,
    /// The AI source context named by the card.
    SourceContext,
    /// The AI route / provider locality named by the card.
    RouteLocality,
    /// The AI spend posture named by the card.
    SpendPosture,
    /// The evidence kind named by the timeline.
    EvidenceKind,
    /// The evidence lineage class named by the timeline.
    LineageClass,
    /// The accountable owner role.
    OwnerRole,
}

impl M5AiEvidenceExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::MessageState,
        Self::SourceContext,
        Self::RouteLocality,
        Self::SpendPosture,
        Self::EvidenceKind,
        Self::LineageClass,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::Dispositions => "dispositions",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::MessageState => "message_state",
            Self::SourceContext => "source_context",
            Self::RouteLocality => "route_locality",
            Self::SpendPosture => "spend_posture",
            Self::EvidenceKind => "evidence_kind",
            Self::LineageClass => "lineage_class",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason an AI message card degraded below a clean, legible state. The degrade-first ladder returns one
/// of these instead of ever letting an ambiguous card read as a clean, trustworthy message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiMessageCardDegradeReason {
    /// The message identity / label is unstated.
    AiIdentityUnstated,
    /// The message state cannot currently be resolved.
    MessageStateUnresolved,
    /// The message state is encoded generically as one completed message rather than named.
    MessageStateEncodedGenerically,
    /// A review-required / blocked message hides its approval state and reads as applied.
    ApprovalStateHidden,
    /// The confidence / uncertainty class is unstated.
    ConfidenceUnstated,
    /// The source context cannot currently be resolved.
    SourceContextUnresolved,
    /// A non-workspace source context is not disclosed.
    SourceContextNotDisclosed,
    /// The route / provider locality cannot currently be resolved.
    RouteLocalityUnresolved,
    /// The local-versus-hosted-provider distinction is left implicit.
    RouteLocalityImplicit,
    /// The spend posture cannot currently be resolved.
    SpendPostureUnresolved,
    /// A metered / capped / over-budget spend is not disclosed.
    SpendPostureNotDisclosed,
    /// No safe actions are offered.
    SafeActionsMissing,
    /// No command-backed path to trace the message is reachable.
    CardDetailPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5AiMessageCardDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::AiIdentityUnstated,
        Self::MessageStateUnresolved,
        Self::MessageStateEncodedGenerically,
        Self::ApprovalStateHidden,
        Self::ConfidenceUnstated,
        Self::SourceContextUnresolved,
        Self::SourceContextNotDisclosed,
        Self::RouteLocalityUnresolved,
        Self::RouteLocalityImplicit,
        Self::SpendPostureUnresolved,
        Self::SpendPostureNotDisclosed,
        Self::SafeActionsMissing,
        Self::CardDetailPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiIdentityUnstated => "ai_identity_unstated",
            Self::MessageStateUnresolved => "message_state_unresolved",
            Self::MessageStateEncodedGenerically => "message_state_encoded_generically",
            Self::ApprovalStateHidden => "approval_state_hidden",
            Self::ConfidenceUnstated => "confidence_unstated",
            Self::SourceContextUnresolved => "source_context_unresolved",
            Self::SourceContextNotDisclosed => "source_context_not_disclosed",
            Self::RouteLocalityUnresolved => "route_locality_unresolved",
            Self::RouteLocalityImplicit => "route_locality_implicit",
            Self::SpendPostureUnresolved => "spend_posture_unresolved",
            Self::SpendPostureNotDisclosed => "spend_posture_not_disclosed",
            Self::SafeActionsMissing => "safe_actions_missing",
            Self::CardDetailPathMissing => "card_detail_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5AiEvidenceNextAction {
        match self {
            Self::AiIdentityUnstated
            | Self::MessageStateUnresolved
            | Self::MessageStateEncodedGenerically => M5AiEvidenceNextAction::ReviewMessageState,
            Self::ApprovalStateHidden
            | Self::ConfidenceUnstated
            | Self::SourceContextUnresolved
            | Self::SourceContextNotDisclosed
            | Self::RouteLocalityUnresolved
            | Self::RouteLocalityImplicit
            | Self::SpendPostureUnresolved
            | Self::SpendPostureNotDisclosed => M5AiEvidenceNextAction::InspectSourceAndApproval,
            Self::SafeActionsMissing | Self::CardDetailPathMissing | Self::ProofStale => {
                M5AiEvidenceNextAction::OpenComponentDetail
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EditorInlineDowngradeTrigger {
        match self {
            Self::ConfidenceUnstated => M5EditorInlineDowngradeTrigger::AiConfidenceUnstated,
            Self::ProofStale => M5EditorInlineDowngradeTrigger::ProofStale,
            _ => M5EditorInlineDowngradeTrigger::GenericChromeWordingUsed,
        }
    }
}

/// Reason an evidence timeline degraded below a clean, legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceTimelineDegradeReason {
    /// The evidence entry identity / label is unstated.
    EvidenceIdentityUnstated,
    /// The evidence timestamp is missing.
    TimestampMissing,
    /// The evidence kind cannot currently be resolved.
    EvidenceKindUnresolved,
    /// The evidence lineage cannot currently be resolved.
    LineageUnresolved,
    /// The tool / validation lineage is not stated.
    LineageNotStated,
    /// No related run / change / resource is named.
    RelatedResourceMissing,
    /// The disclosure state cannot currently be resolved.
    DisclosureUnresolved,
    /// A redacted / partial timeline is not disclosed as incomplete.
    RedactionOrPartialNotDisclosed,
    /// The evidence trail is an opaque log rather than an inspectable structure.
    OpaqueLogNotInspectable,
    /// No open / replay / export action is offered.
    ReplayExportActionsMissing,
    /// No command-backed path to trace the timeline is reachable.
    TimelineDetailPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5EvidenceTimelineDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::EvidenceIdentityUnstated,
        Self::TimestampMissing,
        Self::EvidenceKindUnresolved,
        Self::LineageUnresolved,
        Self::LineageNotStated,
        Self::RelatedResourceMissing,
        Self::DisclosureUnresolved,
        Self::RedactionOrPartialNotDisclosed,
        Self::OpaqueLogNotInspectable,
        Self::ReplayExportActionsMissing,
        Self::TimelineDetailPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceIdentityUnstated => "evidence_identity_unstated",
            Self::TimestampMissing => "timestamp_missing",
            Self::EvidenceKindUnresolved => "evidence_kind_unresolved",
            Self::LineageUnresolved => "lineage_unresolved",
            Self::LineageNotStated => "lineage_not_stated",
            Self::RelatedResourceMissing => "related_resource_missing",
            Self::DisclosureUnresolved => "disclosure_unresolved",
            Self::RedactionOrPartialNotDisclosed => "redaction_or_partial_not_disclosed",
            Self::OpaqueLogNotInspectable => "opaque_log_not_inspectable",
            Self::ReplayExportActionsMissing => "replay_export_actions_missing",
            Self::TimelineDetailPathMissing => "timeline_detail_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5AiEvidenceNextAction {
        match self {
            Self::EvidenceIdentityUnstated
            | Self::TimestampMissing
            | Self::EvidenceKindUnresolved
            | Self::LineageUnresolved
            | Self::LineageNotStated
            | Self::RelatedResourceMissing => M5AiEvidenceNextAction::InspectEvidenceLineage,
            Self::DisclosureUnresolved
            | Self::RedactionOrPartialNotDisclosed
            | Self::OpaqueLogNotInspectable => M5AiEvidenceNextAction::PreserveRedactionTruth,
            Self::ReplayExportActionsMissing
            | Self::TimelineDetailPathMissing
            | Self::ProofStale => M5AiEvidenceNextAction::OpenComponentDetail,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EditorInlineDowngradeTrigger {
        match self {
            Self::OpaqueLogNotInspectable => {
                M5EditorInlineDowngradeTrigger::EvidenceTimelineOpaqueLog
            }
            Self::LineageNotStated | Self::RedactionOrPartialNotDisclosed => {
                M5EditorInlineDowngradeTrigger::EvidencePointerDriftedSilently
            }
            Self::ProofStale => M5EditorInlineDowngradeTrigger::ProofStale,
            _ => M5EditorInlineDowngradeTrigger::GenericChromeWordingUsed,
        }
    }
}

/// True when an evidence disclosure state is redacted or only partially loaded, so it must disclose that
/// the trail is not complete.
fn evidence_is_redacted_or_partial(disclosure: M5EvidenceDisclosure) -> bool {
    matches!(
        disclosure,
        M5EvidenceDisclosure::RedactedExportSafe | M5EvidenceDisclosure::PartiallyLoaded
    )
}

/// True when an evidence disclosure state is resolved (not the unknown sentinel).
fn evidence_disclosure_is_resolved(disclosure: M5EvidenceDisclosure) -> bool {
    !matches!(disclosure, M5EvidenceDisclosure::DisclosureUnknown)
}

/// Input to [`resolve_ai_message_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AiMessageCardResolutionInput {
    /// Stable identity of the AI-message-card instance.
    pub card_id: String,
    /// The message label / identity shown; empty means unstated.
    pub message_label: String,
    /// The controlled AI-message lifecycle state.
    pub message_state: M5AiMessageState,
    /// True when the state is named, never collapsed into one generic completed message.
    pub state_stated: bool,
    /// True when a review-required / blocked message discloses its approval state, never reading applied.
    pub approval_state_disclosed: bool,
    /// The AI confidence / uncertainty class.
    pub confidence: M5AiConfidence,
    /// True when the confidence / uncertainty class is stated, never left implicit.
    pub confidence_stated: bool,
    /// The AI source context.
    pub source_context: M5AiSourceContext,
    /// True when a non-workspace source context is disclosed, never presented as workspace-grounded.
    pub source_disclosed: bool,
    /// The AI route / provider locality.
    pub route_locality: M5AiRouteLocality,
    /// True when the local-versus-hosted-provider distinction is explicit.
    pub route_distinction_explicit: bool,
    /// The AI spend / cost posture.
    pub spend_posture: M5AiSpendPosture,
    /// True when a metered / capped / over-budget spend is disclosed, never presented as free.
    pub spend_disclosed: bool,
    /// True when at least one safe action is offered.
    pub safe_actions_available: bool,
    /// True when a command-backed entrypoint to trace the message is reachable, never menu-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe AI-message-card projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAiMessageCard {
    /// Stable identity of the AI-message-card instance.
    pub card_id: String,
    /// The message label / identity named by the card.
    pub message_label: String,
    /// The message-state token named by the card.
    pub message_state: String,
    /// Whether the message state is stated (name, never one generic completed message).
    pub state_stated: bool,
    /// Whether the message requires approval and must never read as applied.
    pub needs_approval: bool,
    /// Whether the approval state is disclosed.
    pub approval_state_disclosed: bool,
    /// The confidence token named by the card.
    pub confidence: String,
    /// Whether the confidence / uncertainty class is stated.
    pub confidence_stated: bool,
    /// The source-context token named by the card.
    pub source_context: String,
    /// Whether the source context must disclose that the message is not workspace-grounded.
    pub source_needs_disclosure: bool,
    /// Whether a non-workspace source context is disclosed.
    pub source_disclosed: bool,
    /// The route-locality token named by the card.
    pub route_locality: String,
    /// Whether the message was produced by a hosted provider.
    pub route_is_hosted: bool,
    /// Whether the local-versus-hosted-provider distinction is explicit.
    pub route_distinction_explicit: bool,
    /// The spend-posture token named by the card.
    pub spend_posture: String,
    /// Whether the spend posture must disclose a metered / capped / over-budget cost.
    pub spend_needs_disclosure: bool,
    /// Whether a metered / capped / over-budget spend is disclosed.
    pub spend_disclosed: bool,
    /// Whether at least one safe action is offered.
    pub safe_actions_available: bool,
    /// Whether a command-backed entrypoint to trace the message is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the card could not read as a clean, legible state.
    pub degrade_reason: Option<M5AiMessageCardDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5AiEvidenceNextAction,
    /// Whether the card is legible at a glance (clean card naming every fact).
    pub card_legible_at_a_glance: bool,
}

impl M5ResolvedAiMessageCard {
    /// Whether this card reads as a clean, legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_evidence_timeline`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EvidenceTimelineResolutionInput {
    /// Stable identity of the evidence-timeline instance.
    pub timeline_id: String,
    /// The evidence entry label / identity shown; empty means unstated.
    pub entry_label: String,
    /// True when the evidence entry names a timestamp.
    pub has_timestamp: bool,
    /// The evidence kind.
    pub evidence_kind: M5EvidenceKind,
    /// The evidence lineage class.
    pub lineage_class: M5EvidenceLineageClass,
    /// True when the tool / validation lineage is stated, never left implicit.
    pub lineage_stated: bool,
    /// True when a related run / change / resource is named.
    pub related_ref_present: bool,
    /// The evidence disclosure state.
    pub disclosure: M5EvidenceDisclosure,
    /// True when a redacted / partial timeline discloses it is incomplete, never reading as complete.
    pub redaction_disclosed: bool,
    /// True when the evidence trail is an inspectable structure, never an opaque log.
    pub structured_not_opaque: bool,
    /// True when at least one open / replay / export action is offered.
    pub replay_export_actions_available: bool,
    /// True when a command-backed entrypoint to trace the timeline is reachable, never menu-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe evidence-timeline projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedEvidenceTimeline {
    /// Stable identity of the evidence-timeline instance.
    pub timeline_id: String,
    /// The evidence entry label / identity named by the timeline.
    pub entry_label: String,
    /// Whether the evidence entry names a timestamp.
    pub has_timestamp: bool,
    /// The evidence-kind token named by the timeline.
    pub evidence_kind: String,
    /// The lineage-class token named by the timeline.
    pub lineage_class: String,
    /// Whether the tool / validation lineage is stated.
    pub lineage_stated: bool,
    /// Whether a related run / change / resource is named.
    pub related_ref_present: bool,
    /// The disclosure token named by the timeline.
    pub disclosure: String,
    /// Whether the disclosure state is redacted or only partially loaded.
    pub is_redacted_or_partial: bool,
    /// Whether a redacted / partial timeline discloses it is incomplete.
    pub redaction_disclosed: bool,
    /// Whether the evidence trail is an inspectable structure.
    pub structured_not_opaque: bool,
    /// Whether at least one open / replay / export action is offered.
    pub replay_export_actions_available: bool,
    /// Whether a command-backed entrypoint to trace the timeline is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the timeline could not read as a clean, legible state.
    pub degrade_reason: Option<M5EvidenceTimelineDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5AiEvidenceNextAction,
    /// Whether the timeline is legible at a glance (clean timeline naming every fact).
    pub timeline_legible_at_a_glance: bool,
}

impl M5ResolvedEvidenceTimeline {
    /// Whether this timeline reads as a clean, legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5AiEvidenceResolutionError {
    /// The card id was empty.
    EmptyCardId,
    /// The timeline id was empty.
    EmptyTimelineId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5AiEvidenceResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCardId => "empty_card_id",
            Self::EmptyTimelineId => "empty_timeline_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5AiEvidenceResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 ai-message-card / evidence-timeline resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AiEvidenceResolutionError {}

/// Resolves an AI message card so a user can inspect source context, approval state, confidence, route /
/// provider locality, spend posture, and available safe actions before treating an AI output as ready to
/// trust or apply: the card names its lifecycle state (never one generic completed message), discloses
/// approval state (never reading review-required or blocked as applied), states its confidence, discloses
/// a non-workspace source, keeps the local-versus-hosted-provider distinction explicit, discloses a
/// metered spend, offers safe actions, and always offers a command-backed detail entrypoint.
pub fn resolve_ai_message_card(
    input: M5AiMessageCardResolutionInput,
) -> Result<M5ResolvedAiMessageCard, M5AiEvidenceResolutionError> {
    if input.card_id.trim().is_empty() {
        return Err(M5AiEvidenceResolutionError::EmptyCardId);
    }
    if string_is_forbidden(&input.card_id) || string_is_forbidden(&input.message_label) {
        return Err(M5AiEvidenceResolutionError::ForbiddenMaterial);
    }

    let needs_approval = input.message_state.needs_approval();
    let source_needs_disclosure = input.source_context.needs_disclosure();
    let route_is_hosted = input.route_locality.is_hosted();
    let spend_needs_disclosure = input.spend_posture.needs_disclosure();

    let degrade_reason = if input.message_label.trim().is_empty() {
        Some(M5AiMessageCardDegradeReason::AiIdentityUnstated)
    } else if !input.message_state.is_known() {
        Some(M5AiMessageCardDegradeReason::MessageStateUnresolved)
    } else if !input.state_stated {
        Some(M5AiMessageCardDegradeReason::MessageStateEncodedGenerically)
    } else if needs_approval && !input.approval_state_disclosed {
        Some(M5AiMessageCardDegradeReason::ApprovalStateHidden)
    } else if !input.confidence_stated {
        Some(M5AiMessageCardDegradeReason::ConfidenceUnstated)
    } else if !input.source_context.is_resolved() {
        Some(M5AiMessageCardDegradeReason::SourceContextUnresolved)
    } else if source_needs_disclosure && !input.source_disclosed {
        Some(M5AiMessageCardDegradeReason::SourceContextNotDisclosed)
    } else if !input.route_locality.is_resolved() {
        Some(M5AiMessageCardDegradeReason::RouteLocalityUnresolved)
    } else if !input.route_distinction_explicit {
        Some(M5AiMessageCardDegradeReason::RouteLocalityImplicit)
    } else if !input.spend_posture.is_resolved() {
        Some(M5AiMessageCardDegradeReason::SpendPostureUnresolved)
    } else if spend_needs_disclosure && !input.spend_disclosed {
        Some(M5AiMessageCardDegradeReason::SpendPostureNotDisclosed)
    } else if !input.safe_actions_available {
        Some(M5AiMessageCardDegradeReason::SafeActionsMissing)
    } else if !input.detail_command_available {
        Some(M5AiMessageCardDegradeReason::CardDetailPathMissing)
    } else if !input.proof_fresh {
        Some(M5AiMessageCardDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5AiEvidenceNextAction::InspectSourceAndApproval,
    };

    Ok(M5ResolvedAiMessageCard {
        card_id: input.card_id,
        message_label: input.message_label,
        message_state: input.message_state.as_str().to_owned(),
        state_stated: input.state_stated,
        needs_approval,
        approval_state_disclosed: input.approval_state_disclosed,
        confidence: input.confidence.as_str().to_owned(),
        confidence_stated: input.confidence_stated,
        source_context: input.source_context.as_str().to_owned(),
        source_needs_disclosure,
        source_disclosed: input.source_disclosed,
        route_locality: input.route_locality.as_str().to_owned(),
        route_is_hosted,
        route_distinction_explicit: input.route_distinction_explicit,
        spend_posture: input.spend_posture.as_str().to_owned(),
        spend_needs_disclosure,
        spend_disclosed: input.spend_disclosed,
        safe_actions_available: input.safe_actions_available,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        card_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// Resolves an evidence timeline so a user can read a timestamp, evidence kind, tool / validation
/// lineage, related run / change / resource, and open / replay / export actions that remain truthful
/// under redaction: the timeline names a timestamp, a resolved evidence kind, a stated lineage, a related
/// resource, discloses that a redacted / partial trail is incomplete, keeps an inspectable structure
/// (never an opaque log), offers replay / export actions, and always offers a command-backed detail
/// entrypoint.
pub fn resolve_evidence_timeline(
    input: M5EvidenceTimelineResolutionInput,
) -> Result<M5ResolvedEvidenceTimeline, M5AiEvidenceResolutionError> {
    if input.timeline_id.trim().is_empty() {
        return Err(M5AiEvidenceResolutionError::EmptyTimelineId);
    }
    if string_is_forbidden(&input.timeline_id) || string_is_forbidden(&input.entry_label) {
        return Err(M5AiEvidenceResolutionError::ForbiddenMaterial);
    }

    let is_redacted_or_partial = evidence_is_redacted_or_partial(input.disclosure);

    let degrade_reason = if input.entry_label.trim().is_empty() {
        Some(M5EvidenceTimelineDegradeReason::EvidenceIdentityUnstated)
    } else if !input.has_timestamp {
        Some(M5EvidenceTimelineDegradeReason::TimestampMissing)
    } else if !input.evidence_kind.is_resolved() {
        Some(M5EvidenceTimelineDegradeReason::EvidenceKindUnresolved)
    } else if !input.lineage_class.is_resolved() {
        Some(M5EvidenceTimelineDegradeReason::LineageUnresolved)
    } else if !input.lineage_stated {
        Some(M5EvidenceTimelineDegradeReason::LineageNotStated)
    } else if !input.related_ref_present {
        Some(M5EvidenceTimelineDegradeReason::RelatedResourceMissing)
    } else if !evidence_disclosure_is_resolved(input.disclosure) {
        Some(M5EvidenceTimelineDegradeReason::DisclosureUnresolved)
    } else if is_redacted_or_partial && !input.redaction_disclosed {
        Some(M5EvidenceTimelineDegradeReason::RedactionOrPartialNotDisclosed)
    } else if !input.structured_not_opaque {
        Some(M5EvidenceTimelineDegradeReason::OpaqueLogNotInspectable)
    } else if !input.replay_export_actions_available {
        Some(M5EvidenceTimelineDegradeReason::ReplayExportActionsMissing)
    } else if !input.detail_command_available {
        Some(M5EvidenceTimelineDegradeReason::TimelineDetailPathMissing)
    } else if !input.proof_fresh {
        Some(M5EvidenceTimelineDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5AiEvidenceNextAction::InspectEvidenceLineage,
    };

    Ok(M5ResolvedEvidenceTimeline {
        timeline_id: input.timeline_id,
        entry_label: input.entry_label,
        has_timestamp: input.has_timestamp,
        evidence_kind: input.evidence_kind.as_str().to_owned(),
        lineage_class: input.lineage_class.as_str().to_owned(),
        lineage_stated: input.lineage_stated,
        related_ref_present: input.related_ref_present,
        disclosure: input.disclosure.as_str().to_owned(),
        is_redacted_or_partial,
        redaction_disclosed: input.redaction_disclosed,
        structured_not_opaque: input.structured_not_opaque,
        replay_export_actions_available: input.replay_export_actions_available,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        timeline_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved AI-message-card and evidence-timeline
/// examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiEvidenceControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5AiEvidenceConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5EditorInlineQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5EditorInlineDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5EditorInlineRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5EditorInlineAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5AiEvidenceAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5AiEvidenceExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5EditorInlineDowngradeTrigger>,
    /// Resolved AI-message-card examples.
    pub card_examples: Vec<M5ResolvedAiMessageCard>,
    /// Resolved evidence-timeline examples.
    pub evidence_examples: Vec<M5ResolvedEvidenceTimeline>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: an AI message state or source context is never silently generic.
    pub ai_message_state_or_source_context_silently_generic: bool,
    /// Hard invariant: an AI route or spend posture never silently drifts.
    pub ai_route_or_spend_posture_silently_drifts: bool,
    /// Hard invariant: an evidence timeline is never hidden in an opaque log.
    pub evidence_timeline_hidden_in_opaque_log: bool,
    /// Hard invariant: an evidence lineage or redaction truth never silently drifts.
    pub evidence_lineage_or_redaction_truth_silently_drifts: bool,
}

impl M5AiEvidenceControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5AiEvidenceAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5AiEvidenceAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5AiEvidenceExportField> =
            self.export_fields.iter().copied().collect();
        M5AiEvidenceExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.ai_message_state_or_source_context_silently_generic
            && !self.ai_route_or_spend_posture_silently_drifts
            && !self.evidence_timeline_hidden_in_opaque_log
            && !self.evidence_lineage_or_redaction_truth_silently_drifts
    }

    /// True when every resolved example on this row is honest: no clean card collapses its state to a
    /// generic completed message, hides approval, leaves confidence implicit, presents a non-workspace
    /// source as grounded, leaves the route locality implicit, hides a metered spend, omits safe actions,
    /// or lacks a trace path; and no clean evidence omits a timestamp / lineage / related resource, hides
    /// a redacted / partial trail, reads as an opaque log, omits replay / export actions, or lacks a
    /// trace path.
    fn examples_are_honest(&self) -> bool {
        self.card_examples
            .iter()
            .all(|ex| !ex.is_clean() || card_is_honest(ex))
            && self
                .evidence_examples
                .iter()
                .all(|ex| !ex.is_clean() || evidence_is_honest(ex))
    }
}

/// True when a clean AI message card keeps every guardrail: state stated, approval disclosed, confidence
/// stated, non-workspace source disclosed, explicit route locality, metered spend disclosed, safe
/// actions offered, and a reachable trace.
fn card_is_honest(ex: &M5ResolvedAiMessageCard) -> bool {
    ex.state_stated
        && (ex.approval_state_disclosed || !ex.needs_approval)
        && ex.confidence_stated
        && (ex.source_disclosed || !ex.source_needs_disclosure)
        && ex.route_distinction_explicit
        && (ex.spend_disclosed || !ex.spend_needs_disclosure)
        && ex.safe_actions_available
        && ex.detail_command_available
}

/// True when a clean evidence timeline keeps every guardrail: timestamp present, lineage stated, related
/// resource named, redacted / partial trail disclosed, an inspectable (non-opaque) structure, replay /
/// export actions offered, and a reachable trace.
fn evidence_is_honest(ex: &M5ResolvedEvidenceTimeline) -> bool {
    ex.has_timestamp
        && ex.lineage_stated
        && ex.related_ref_present
        && (ex.redaction_disclosed || !ex.is_redacted_or_partial)
        && ex.structured_not_opaque
        && ex.replay_export_actions_available
        && ex.detail_command_available
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiEvidenceVocabularySet {
    /// Inline-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// AI-confidence tokens (bound from the frozen matrix).
    pub ai_confidences: Vec<String>,
    /// Evidence-disclosure tokens (bound from the frozen matrix).
    pub evidence_disclosures: Vec<String>,
    /// AI-message-state tokens (minted by this lane).
    pub ai_message_states: Vec<String>,
    /// AI source-context tokens (minted by this lane).
    pub ai_source_contexts: Vec<String>,
    /// AI route-locality tokens (minted by this lane).
    pub ai_route_localities: Vec<String>,
    /// AI spend-posture tokens (minted by this lane).
    pub ai_spend_postures: Vec<String>,
    /// Evidence-kind tokens (minted by this lane).
    pub evidence_kinds: Vec<String>,
    /// Evidence lineage-class tokens (minted by this lane).
    pub evidence_lineage_classes: Vec<String>,
    /// AI-message-card degrade-reason tokens.
    pub ai_message_card_degrade_reasons: Vec<String>,
    /// Evidence-timeline degrade-reason tokens.
    pub evidence_timeline_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5AiEvidenceVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5EditorInlineDisposition::ALL, |v| v.as_str()),
            ai_confidences: tokens(&M5AiConfidence::ALL, |v| v.as_str()),
            evidence_disclosures: tokens(&M5EvidenceDisclosure::ALL, |v| v.as_str()),
            ai_message_states: tokens(&M5AiMessageState::ALL, |v| v.as_str()),
            ai_source_contexts: tokens(&M5AiSourceContext::ALL, |v| v.as_str()),
            ai_route_localities: tokens(&M5AiRouteLocality::ALL, |v| v.as_str()),
            ai_spend_postures: tokens(&M5AiSpendPosture::ALL, |v| v.as_str()),
            evidence_kinds: tokens(&M5EvidenceKind::ALL, |v| v.as_str()),
            evidence_lineage_classes: tokens(&M5EvidenceLineageClass::ALL, |v| v.as_str()),
            ai_message_card_degrade_reasons: tokens(&M5AiMessageCardDegradeReason::ALL, |v| {
                v.as_str()
            }),
            evidence_timeline_degrade_reasons: tokens(&M5EvidenceTimelineDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5AiEvidenceAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5AiEvidenceNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5AiEvidenceExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5EditorInlineConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5AiEvidenceGovernanceReview {
    /// The card names its state, source context, confidence, route, and spend with one vocabulary.
    pub card_names_state_source_confidence_route_and_spend: bool,
    /// Draft / streaming / review-required / blocked / applied / reverted / failed / stale states stay
    /// explicit rather than one generic completed message.
    pub message_states_stay_explicit: bool,
    /// Approval state is always inspectable before an AI output is trusted or applied.
    pub approval_state_always_inspectable: bool,
    /// The local-versus-hosted-provider distinction and spend posture stay explicit.
    pub route_locality_and_spend_stay_explicit: bool,
    /// The evidence timeline names a timestamp, evidence kind, lineage, and related resource.
    pub evidence_names_timestamp_kind_lineage_and_resource: bool,
    /// Evidence timelines keep an inspectable structure rather than an opaque log.
    pub evidence_keeps_inspectable_structure: bool,
    /// Redaction and partial-load truth are preserved rather than reading as complete.
    pub redaction_and_partial_truth_preserved: bool,
    /// Evidence pointers never silently drift.
    pub evidence_pointers_never_silently_drift: bool,
    /// The same message and evidence grammar holds across desktop, browser handoff, and export.
    pub message_and_evidence_grammar_holds_across_surfaces: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiEvidenceConsumerProjection {
    /// AI surfaces consume the shared message and evidence vocabulary.
    pub ai_surfaces_consume_message_and_evidence_vocabulary: bool,
    /// Editor and review surfaces consume the shared message-state and evidence vocabulary.
    pub editor_and_review_surfaces_consume_shared_vocabulary: bool,
    /// Notebook surfaces consume the shared evidence-lineage vocabulary.
    pub notebook_surfaces_consume_evidence_lineage_vocabulary: bool,
    /// Browser handoff and export preserve source context, spend, lineage, and redaction truth.
    pub browser_handoff_and_export_preserve_source_and_lineage: bool,
    /// Message and evidence facts trace back to one canonical component contract.
    pub facts_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical editor-inline source.
    pub support_export_reads_single_editor_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiEvidenceProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiEvidenceReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5AiEvidenceControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AiEvidenceControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5AiEvidenceControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiEvidenceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiEvidenceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiEvidenceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiEvidenceProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiEvidenceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 AI-message-card / evidence-timeline controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiEvidenceControlsPacket {
    /// Record kind; must equal [`M5_AI_EVIDENCE_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AI_EVIDENCE_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5AiEvidenceControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiEvidenceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiEvidenceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiEvidenceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiEvidenceProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiEvidenceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AiEvidenceControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5AiEvidenceControlsPacketInput) -> Self {
        Self {
            record_kind: M5_AI_EVIDENCE_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_AI_EVIDENCE_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            controls_label: input.controls_label,
            controls_rows: input.controls_rows,
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

    /// Validates the controls-packet invariants.
    pub fn validate(&self) -> Vec<M5AiEvidenceControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_AI_EVIDENCE_CONTROLS_RECORD_KIND {
            violations.push(M5AiEvidenceControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AI_EVIDENCE_CONTROLS_SCHEMA_VERSION {
            violations.push(M5AiEvidenceControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AiEvidenceControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5AiEvidenceControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 ai-message-card / evidence-timeline controls packet serializes"),
        ) {
            violations.push(M5AiEvidenceControlsViolation::RawMaterialInExport);
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
            .expect("m5 ai-message-card / evidence-timeline controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,card_examples,evidence_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .card_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.evidence_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.card_examples.len(),
                row.evidence_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 AI-Message-Card and Evidence-Timeline Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- AI message states: {}\n",
            self.vocabulary_set.ai_message_states.join(", ")
        ));
        out.push_str(&format!(
            "- Evidence kinds: {}\n",
            self.vocabulary_set.evidence_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.controls_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Card examples: {} / evidence examples: {}\n",
                row.card_examples.len(),
                row.evidence_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5AiEvidenceControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AiEvidenceControlsViolation>),
}

impl fmt::Display for M5AiEvidenceControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 ai-message-card / evidence-timeline controls export parse failed: {error}"
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
                    "m5 ai-message-card / evidence-timeline controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AiEvidenceControlsArtifactError {}

/// Validation failures emitted by [`M5AiEvidenceControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AiEvidenceControlsViolation {
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
    /// The controls packet declares no rows.
    NoControlsRows,
    /// A controls row is incomplete.
    ControlsRowIncomplete,
    /// A controls row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row does not point at both component schemas.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (generic state, hidden approval, undisclosed
    /// source / spend, opaque log, hidden redaction, or missing trace).
    DishonestExample,
    /// A controls row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// The shared message and evidence vocabulary is not proven: clean cards do not span the shared
    /// message-state and route grammar, clean evidence does not span evidence kinds, or no generic-state
    /// example degrades.
    MessageAndEvidenceVocabularyNotProven,
    /// Source, approval, and evidence inspectability is not proven: no clean card discloses a source, no
    /// approval-hidden example degrades, or a clean card and evidence do not both offer a detail path.
    SourceApprovalAndEvidenceInspectableNotProven,
    /// Lineage and redaction truth is not proven: clean evidence does not span lineage classes, no
    /// opaque-log example degrades, or no redaction-hidden example degrades.
    LineageAndRedactionTruthNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5AiEvidenceControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoControlsRows => "no_controls_rows",
            Self::ControlsRowIncomplete => "controls_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::MessageAndEvidenceVocabularyNotProven => {
                "message_and_evidence_vocabulary_not_proven"
            }
            Self::SourceApprovalAndEvidenceInspectableNotProven => {
                "source_approval_and_evidence_inspectable_not_proven"
            }
            Self::LineageAndRedactionTruthNotProven => "lineage_and_redaction_truth_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_ai_evidence_controls_export(
) -> Result<M5AiEvidenceControlsPacket, M5AiEvidenceControlsArtifactError> {
    let packet: M5AiEvidenceControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-ai-message-card-evidence-timeline-controls-proof/support_export.json"
    )))
    .map_err(M5AiEvidenceControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AiEvidenceControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5AiEvidenceControlsPacket,
    violations: &mut Vec<M5AiEvidenceControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_AI_EVIDENCE_CONTROLS_SCHEMA_REF,
        M5_AI_EVIDENCE_CONTROLS_DOC_REF,
        M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF,
        M5_EDITOR_INLINE_COMPONENT_DOC_REF,
        M5_AI_MESSAGE_CARD_SCHEMA_REF,
        M5_EVIDENCE_TIMELINE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AiEvidenceControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5AiEvidenceControlsPacket,
    violations: &mut Vec<M5AiEvidenceControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5AiEvidenceControlsViolation::NoControlsRows);
        return;
    }
    for row in &packet.controls_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5AiEvidenceControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5AiEvidenceControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5AiEvidenceControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_AI_MESSAGE_CARD_SCHEMA_REF)
            || !refs.contains(M5_EVIDENCE_TIMELINE_SCHEMA_REF)
        {
            violations.push(M5AiEvidenceControlsViolation::ComponentSchemaRefMissing);
        }
        if row.card_examples.is_empty() || row.evidence_examples.is_empty() {
            violations.push(M5AiEvidenceControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5AiEvidenceControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5AiEvidenceControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5AiEvidenceControlsPacket,
    violations: &mut Vec<M5AiEvidenceControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.card_names_state_source_confidence_route_and_spend,
        review.message_states_stay_explicit,
        review.approval_state_always_inspectable,
        review.route_locality_and_spend_stay_explicit,
        review.evidence_names_timestamp_kind_lineage_and_resource,
        review.evidence_keeps_inspectable_structure,
        review.redaction_and_partial_truth_preserved,
        review.evidence_pointers_never_silently_drift,
        review.message_and_evidence_grammar_holds_across_surfaces,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5AiEvidenceControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AiEvidenceControlsPacket,
    violations: &mut Vec<M5AiEvidenceControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.ai_surfaces_consume_message_and_evidence_vocabulary,
        projection.editor_and_review_surfaces_consume_shared_vocabulary,
        projection.notebook_surfaces_consume_evidence_lineage_vocabulary,
        projection.browser_handoff_and_export_preserve_source_and_lineage,
        projection.facts_trace_to_single_component_contract,
        projection.support_export_reads_single_editor_source,
    ] {
        if !ok {
            violations.push(M5AiEvidenceControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AiEvidenceControlsPacket,
    violations: &mut Vec<M5AiEvidenceControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AiEvidenceControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AiEvidenceControlsPacket,
    violations: &mut Vec<M5AiEvidenceControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5AiEvidenceControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5AiEvidenceControlsPacket,
    violations: &mut Vec<M5AiEvidenceControlsViolation>,
) {
    let cards = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.card_examples.iter())
    };
    let evidence = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.evidence_examples.iter())
    };

    // AC1: AI surfaces across claimed M5 lanes expose the same message and evidence vocabulary rather
    // than per-feature chat chrome. Clean cards cover at least two distinct message states and span
    // local-model and hosted-provider route localities, clean evidence covers at least two distinct
    // evidence kinds, a generically-encoded state example degrades, and no clean card is generic.
    let clean_message_states: BTreeSet<String> = cards()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.message_state.clone())
        .collect();
    let clean_route_localities: BTreeSet<String> = cards()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.route_locality.clone())
        .collect();
    let clean_evidence_kinds: BTreeSet<String> = evidence()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.evidence_kind.clone())
        .collect();
    let generic_state_degrades = cards().any(|ex| {
        ex.degrade_reason == Some(M5AiMessageCardDegradeReason::MessageStateEncodedGenerically)
    });
    let spans_local_and_hosted = clean_route_localities.contains("local_model")
        && clean_route_localities.contains("hosted_provider");
    let no_clean_generic = cards().all(|ex| !ex.is_clean() || ex.state_stated);
    if !(clean_message_states.len() >= 2
        && spans_local_and_hosted
        && clean_evidence_kinds.len() >= 2
        && generic_state_degrades
        && no_clean_generic)
    {
        violations.push(M5AiEvidenceControlsViolation::MessageAndEvidenceVocabularyNotProven);
    }

    // AC2: users can inspect source context, approval state, and supporting evidence before treating an
    // AI output as ready to trust or apply. At least one clean card discloses a non-workspace source, an
    // approval-hidden example degrades, no clean card hides approval, and a clean card and clean evidence
    // both offer a command-backed detail entrypoint.
    let clean_source_disclosed =
        cards().any(|ex| ex.is_clean() && ex.source_needs_disclosure && ex.source_disclosed);
    let approval_hidden_degrades = cards()
        .any(|ex| ex.degrade_reason == Some(M5AiMessageCardDegradeReason::ApprovalStateHidden));
    let no_clean_approval_hidden =
        cards().all(|ex| !ex.is_clean() || ex.approval_state_disclosed || !ex.needs_approval);
    let traceable_card = cards().any(|ex| ex.is_clean() && ex.detail_command_available);
    let traceable_evidence = evidence().any(|ex| ex.is_clean() && ex.detail_command_available);
    if !(clean_source_disclosed
        && approval_hidden_degrades
        && no_clean_approval_hidden
        && traceable_card
        && traceable_evidence)
    {
        violations
            .push(M5AiEvidenceControlsViolation::SourceApprovalAndEvidenceInspectableNotProven);
    }

    // AC3: timeline and export consumers preserve lineage and redaction truth instead of flattening AI
    // history into unstructured logs. Clean evidence covers at least two distinct lineage classes, an
    // opaque-log example degrades, a redaction-hidden example degrades, no clean evidence is opaque, and
    // no clean evidence hides a redacted / partial trail.
    let clean_lineage_classes: BTreeSet<String> = evidence()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.lineage_class.clone())
        .collect();
    let opaque_degrades = evidence().any(|ex| {
        ex.degrade_reason == Some(M5EvidenceTimelineDegradeReason::OpaqueLogNotInspectable)
    });
    let redaction_degrades = evidence().any(|ex| {
        ex.degrade_reason == Some(M5EvidenceTimelineDegradeReason::RedactionOrPartialNotDisclosed)
    });
    let no_clean_opaque = evidence().all(|ex| !ex.is_clean() || ex.structured_not_opaque);
    let no_clean_redaction_hidden =
        evidence().all(|ex| !ex.is_clean() || ex.redaction_disclosed || !ex.is_redacted_or_partial);
    if !(clean_lineage_classes.len() >= 2
        && opaque_degrades
        && redaction_degrades
        && no_clean_opaque
        && no_clean_redaction_hidden)
    {
        violations.push(M5AiEvidenceControlsViolation::LineageAndRedactionTruthNotProven);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
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

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The two component families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5EditorInlineComponentFamily; 2] = [
    M5EditorInlineComponentFamily::AiMessageCard,
    M5EditorInlineComponentFamily::EvidenceTimeline,
];
