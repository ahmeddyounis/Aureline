//! AI composer, context-inspector, and provider-routing primitives.
//!
//! This crate owns inspectable AI records consumed by shell, diagnostics,
//! support export, and evidence surfaces. The composer lane exposes one
//! [`composer::ComposerDraft`] object plus typed mention, attachment,
//! slash-command, and route-placeholder vocabularies. The routing lane exposes
//! one [`routing::AiRoutingPacket`] object for provider/model identity, quota
//! explainability, latency/cost envelopes, and visible route-change lineage on
//! claimed hosted-model paths.
//!
//! These records carry no credential bodies, raw provider payloads, raw
//! endpoint URLs, exact token counts, or exact cost amounts. Consumers project
//! the typed packets directly and never re-derive authority, lifecycle state,
//! provider identity, quota state, or route explanations locally.
//!
//! The frozen cross-tool contracts the seed projects against are
//! [`/docs/ai/prompt_composer_contract.md`](../../../docs/ai/prompt_composer_contract.md)
//! and
//! [`/docs/ai/context_assembly_contract.md`](../../../docs/ai/context_assembly_contract.md).
//! The routing alpha entry point is
//! [`/docs/ai/routing_cost_alpha.md`](../../../docs/ai/routing_cost_alpha.md).
//! The records cover bounded, honest subsets of the frozen vocabularies and
//! grow additively without forking truth.

#![doc(html_root_url = "https://docs.rs/aureline-ai/0.0.0")]

pub mod composer;
pub mod routing;

pub use composer::{
    AttachmentKind, AttachmentStatusClass, BlockReason, ComposerAttachment, ComposerDraft,
    ComposerDraftState, ComposerMention, ComposerSlashCommandInvocation, DispatchTargetClass,
    MentionKind, MentionResolutionState, PrototypeLabel, ProviderClass, RoutePathClass,
    RoutePlaceholder, SelectionReasonClass, SlashCommandResolutionState, SourceClass, TrustPosture,
    ValidationOutcome, COMPOSER_DRAFT_RECORD_KIND, COMPOSER_DRAFT_SCHEMA_VERSION,
};
pub use routing::{
    AiRouteCandidate, AiRouteProviderClass, AiRoutingPacket, AiRoutingSupportPacket,
    AiRoutingSupportRouteChangeRow, AiRoutingSurfaceRow, AiRoutingViolation, CostEnvelopeClass,
    CostVisibilityClass, DeploymentProfileClass, ExecutionLocusClass, ExhaustionStateClass,
    LatencyCostEnvelope, LatencyEnvelopeClass, PolicyTrustState, QuotaFamilyClass, QuotaInspector,
    QuotaScopeClass, QuotaStateClass, RetentionStanceClass, RouteChangeCauseClass,
    RouteChangeLineage, RouteOriginClass, RouteSelectionOverrideReasonClass,
    RouteSelectionReasonClass, RoutingPolicyContext, RoutingRunStateClass, SelectedOutcomeClass,
    TokenCeilingClass, ToolCallCeilingClass, WallTimeCeilingClass, AI_ROUTING_PACKET_RECORD_KIND,
    AI_ROUTING_SCHEMA_VERSION, AI_ROUTING_SUPPORT_PACKET_RECORD_KIND,
};
