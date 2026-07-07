//! Two reusable M5 AI routing primitives — the connector / tool-server detail row and
//! the local model pack card — so provider-neutral AI routing becomes inspectable on
//! first-class surfaces.
//!
//! Aureline's frozen AI-execution/replay component matrix
//! ([`crate::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix`])
//! names the connector detail row and the local model pack card as two governed
//! component families and freezes their controlled vocabulary — the connector
//! capabilities, the auth postures, the model pack states, the surface families, the
//! deployment lines, the consumer surfaces, the accessibility routes, the qualification
//! classes, and the downgrade triggers. This module *implements* those two contracts as
//! reusable primitives so a user can tell — from the row or the card alone — where a
//! tool or a model runs, what authority or hardware it depends on, and what disk cost,
//! offline locality, and bounded actions apply, before invocation and before mistaking
//! a generic `installed` state for a warm, verified, hardware-fit pack.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_connector_detail_row`] — takes one connector's canonical id,
//!    publisher / source, execution locus, declared capabilities, auth posture, and
//!    live reachability signals, and produces one [`M5ResolvedConnectorDetailRow`]
//!    carrying the derived connector readiness (warm versus cold versus unavailable
//!    versus policy-blocked), whether the connector is invocable, and whether it
//!    depends on an authority grant before invocation. It never masks the execution
//!    locus or auth posture, never presents a policy-blocked connector as ready, and
//!    never lets a side-effecting capability go undisclosed.
//! 2. [`resolve_local_model_pack_card`] — takes one model pack's identity, digest, size
//!    on disk, hardware expectations, pack state, provenance, and offline signals, and
//!    produces one [`M5ResolvedLocalModelPackCard`] carrying the derived hardware fit,
//!    the model pack readiness, the offline posture, and the bounded select / verify /
//!    remove actions. It never hides disk cost, hardware expectations, or offline
//!    locality behind a generic `installed` state.
//!
//! A single parity matrix — [`M5AiConnectorModelPrimitivePacket`] — binds one row per
//! claimed M5 routing consumer (AI settings, the model picker, the route inspector, the
//! evidence view, and the CLI / support export) to the shared connector and model
//! anatomy, the same execution loci, capabilities, auth postures, connector
//! readinesses, model pack states, model readinesses, hardware fits, offline postures,
//! bounded actions, export fields, and non-visual accessibility routes, so the
//! boundary / auth / locality vocabulary stays identical across settings, model
//! pickers, route inspectors, evidence views, and support / help exports.
//!
//! The connector capability ([`M5AiConnectorCapability`]), auth posture
//! ([`M5AiAuthPosture`]), model pack state ([`M5AiModelPackState`]), surface family
//! ([`M5AiSurfaceFamily`]), deployment line ([`M5AiDeploymentLine`]), consumer surface
//! ([`M5AiConsumerSurface`]), accessibility route ([`M5AiAccessibilityRoute`]),
//! qualification class ([`M5AiQualificationClass`]), and downgrade trigger
//! ([`M5AiExecutionDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the row
//! and the card themselves: their routing consumers, their anatomy parts, their
//! execution loci, their derived connector readiness, their derived model readiness,
//! their hardware fits, their offline postures, their bounded actions, and their export
//! fields. No M5 AI surface invents a second connector or model grammar.
//!
//! Raw URLs, raw tokens, credentials, private endpoints, and user text bodies stay
//! outside the support boundary; every canonical id, publisher, model identity, digest,
//! and hardware label is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ai/m5-ai-connector-detail-row-and-local-model-pack-card.schema.json`](../../../../schemas/ai/m5-ai-connector-detail-row-and-local-model-pack-card.schema.json)
//! and the contract doc is
//! [`docs/ai/m5/implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces.md`](../../../../docs/ai/m5/implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_ai_connector_model_primitive_evidence_view_beta_narrowed,
    seeded_m5_ai_connector_model_primitive_packet,
    seeded_m5_ai_connector_model_primitive_route_inspector_preview_narrowed,
    M5_AI_CONNECTOR_MODEL_PRIMITIVE_PACKET_ID,
};

// The connector capability, auth posture, model pack state, surface family, deployment
// line, consumer surface, accessibility route, qualification class, and downgrade
// triggers are frozen once, in the AI-execution/replay component matrix. These
// primitives reuse them verbatim so they never invent a parallel routing vocabulary.
pub use crate::freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix::{
    M5AiAccessibilityRoute, M5AiAuthPosture, M5AiConnectorCapability, M5AiConsumerSurface,
    M5AiDeploymentLine, M5AiExecutionDowngradeTrigger, M5AiModelPackState, M5AiQualificationClass,
    M5AiSurfaceFamily,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5AiConnectorModelPrimitivePacket`].
pub const M5_AI_CONNECTOR_MODEL_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces";

/// Schema version for M5 AI connector/local-model-primitive records.
pub const M5_AI_CONNECTOR_MODEL_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the connector-detail-row / local-model-pack-card schema.
pub const M5_AI_CONNECTOR_MODEL_SCHEMA_REF: &str =
    "schemas/ai/m5-ai-connector-detail-row-and-local-model-pack-card.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_AI_CONNECTOR_MODEL_DOC_REF: &str =
    "docs/ai/m5/implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces.md";

/// Repo-relative path of the frozen AI-execution/replay component matrix these
/// primitives narrow from.
pub const M5_AI_CONNECTOR_MODEL_COMPONENT_MATRIX_REF: &str =
    "schemas/ai/freeze-the-m5-ai-action-state-banner-connector-detail-row-local-model-pack-card-approval-sheet-tool-call-timeline-row-run-history-row-replay-review-and-agent-status-component-matrix.schema.json";

/// Repo-relative path of the external-tool-gateway / connector-manifest contract this
/// primitive binds its connector-capability and side-effect truth against.
pub const M5_AI_CONNECTOR_MODEL_GATEWAY_REF: &str =
    "schemas/ai/ship-the-external-tool-gateway-and-connector-manifests-with-capability-classes-and-side-effect-disclosure.schema.json";

/// Repo-relative path of the local-model-pack contract this primitive binds its
/// provenance / hardware-fit / offline truth against.
pub const M5_AI_CONNECTOR_MODEL_LOCAL_MODEL_REF: &str =
    "schemas/ai/implement-local-model-pack-install-provenance-hardware-fit-checks-and-offline-or-mirror-support.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_AI_CONNECTOR_MODEL_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AI_CONNECTOR_MODEL_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_AI_CONNECTOR_MODEL_CSV_REF: &str =
    "artifacts/ai/m5/implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_AI_CONNECTOR_MODEL_REPORT_REF: &str =
    "artifacts/ai/m5/implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces.md";

/// One claimed M5 routing consumer that renders the shared connector detail row and the
/// local model pack card. These are the consumers the acceptance criteria name — AI
/// settings, the model picker, the route inspector, the evidence view, and the CLI /
/// support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiConnectorModelConsumerSurface {
    /// The AI settings surface.
    AiSettings,
    /// The model picker.
    ModelPicker,
    /// The route inspector.
    RouteInspector,
    /// The evidence view.
    EvidenceView,
    /// The CLI inspect / support export.
    CliSupportExport,
}

impl M5AiConnectorModelConsumerSurface {
    /// Every claimed routing consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AiSettings,
        Self::ModelPicker,
        Self::RouteInspector,
        Self::EvidenceView,
        Self::CliSupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiSettings => "ai_settings",
            Self::ModelPicker => "model_picker",
            Self::RouteInspector => "route_inspector",
            Self::EvidenceView => "evidence_view",
            Self::CliSupportExport => "cli_support_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AiSettings => "AI Settings",
            Self::ModelPicker => "Model Picker",
            Self::RouteInspector => "Route Inspector",
            Self::EvidenceView => "Evidence View",
            Self::CliSupportExport => "CLI / Support Export",
        }
    }
}

/// Controlled execution locus — where a tool / connector actually runs, so a connector
/// detail row never leaves the acceptance-criterion "where a tool runs" implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiExecutionLocus {
    /// In-process, inside the Aureline host.
    InProcessLocal,
    /// A local subprocess on the same machine.
    LocalSubprocess,
    /// A local container on the same machine.
    LocalContainer,
    /// A remote managed service Aureline operates.
    RemoteManagedService,
    /// A third-party cloud service.
    ThirdPartyCloud,
    /// An on-prem bridge to a customer-operated service.
    OnPremBridge,
}

impl M5AiExecutionLocus {
    /// Every execution locus, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InProcessLocal,
        Self::LocalSubprocess,
        Self::LocalContainer,
        Self::RemoteManagedService,
        Self::ThirdPartyCloud,
        Self::OnPremBridge,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProcessLocal => "in_process_local",
            Self::LocalSubprocess => "local_subprocess",
            Self::LocalContainer => "local_container",
            Self::RemoteManagedService => "remote_managed_service",
            Self::ThirdPartyCloud => "third_party_cloud",
            Self::OnPremBridge => "on_prem_bridge",
        }
    }

    /// Review-safe phrase naming where the tool runs.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::InProcessLocal => "in-process on this machine",
            Self::LocalSubprocess => "a local subprocess on this machine",
            Self::LocalContainer => "a local container on this machine",
            Self::RemoteManagedService => "a remote managed service",
            Self::ThirdPartyCloud => "a third-party cloud service",
            Self::OnPremBridge => "an on-prem bridge to a customer service",
        }
    }

    /// True when the tool runs on the local machine.
    pub const fn is_local(self) -> bool {
        matches!(
            self,
            Self::InProcessLocal | Self::LocalSubprocess | Self::LocalContainer
        )
    }
}

/// Controlled connector anatomy part the shared detail row surfaces. The parts in
/// [`M5AiConnectorAnatomyPart::MANDATORY`] are required on every connector row so a user
/// can tell where the tool runs and what authority it needs before invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiConnectorAnatomyPart {
    /// The canonical connector id.
    CanonicalIdCue,
    /// The publisher / source.
    PublisherSourceCue,
    /// The execution locus: where the tool runs.
    ExecutionLocusCue,
    /// The declared capability list.
    CapabilityListCue,
    /// The auth posture.
    AuthPostureCue,
    /// The warm/cold/unavailable/policy-blocked readiness state.
    ReadinessStateCue,
    /// The side-effect disclosure.
    SideEffectDisclosureCue,
    /// The pre-invocation authority guard.
    InvocationGuardCue,
}

impl M5AiConnectorAnatomyPart {
    /// Every connector anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CanonicalIdCue,
        Self::PublisherSourceCue,
        Self::ExecutionLocusCue,
        Self::CapabilityListCue,
        Self::AuthPostureCue,
        Self::ReadinessStateCue,
        Self::SideEffectDisclosureCue,
        Self::InvocationGuardCue,
    ];

    /// The connector anatomy parts every row must render.
    pub const MANDATORY: [Self; 5] = [
        Self::CanonicalIdCue,
        Self::ExecutionLocusCue,
        Self::CapabilityListCue,
        Self::AuthPostureCue,
        Self::ReadinessStateCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalIdCue => "canonical_id_cue",
            Self::PublisherSourceCue => "publisher_source_cue",
            Self::ExecutionLocusCue => "execution_locus_cue",
            Self::CapabilityListCue => "capability_list_cue",
            Self::AuthPostureCue => "auth_posture_cue",
            Self::ReadinessStateCue => "readiness_state_cue",
            Self::SideEffectDisclosureCue => "side_effect_disclosure_cue",
            Self::InvocationGuardCue => "invocation_guard_cue",
        }
    }
}

/// Controlled local-model anatomy part the shared pack card surfaces. The parts in
/// [`M5AiModelPackAnatomyPart::MANDATORY`] are required on every card so disk cost,
/// hardware expectations, and offline locality are never hidden behind `installed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiModelPackAnatomyPart {
    /// The model identity.
    ModelIdentityCue,
    /// The content digest.
    DigestCue,
    /// The size on disk.
    DiskSizeCue,
    /// The hardware expectations.
    HardwareExpectationCue,
    /// The offline posture.
    OfflinePostureCue,
    /// The pack lifecycle state.
    PackStateCue,
    /// The provenance / verification state.
    ProvenanceCue,
    /// The bounded action row (select / verify / remove).
    ActionRowCue,
}

impl M5AiModelPackAnatomyPart {
    /// Every model-pack anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ModelIdentityCue,
        Self::DigestCue,
        Self::DiskSizeCue,
        Self::HardwareExpectationCue,
        Self::OfflinePostureCue,
        Self::PackStateCue,
        Self::ProvenanceCue,
        Self::ActionRowCue,
    ];

    /// The model-pack anatomy parts every card must render.
    pub const MANDATORY: [Self; 5] = [
        Self::ModelIdentityCue,
        Self::DigestCue,
        Self::DiskSizeCue,
        Self::HardwareExpectationCue,
        Self::OfflinePostureCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModelIdentityCue => "model_identity_cue",
            Self::DigestCue => "digest_cue",
            Self::DiskSizeCue => "disk_size_cue",
            Self::HardwareExpectationCue => "hardware_expectation_cue",
            Self::OfflinePostureCue => "offline_posture_cue",
            Self::PackStateCue => "pack_state_cue",
            Self::ProvenanceCue => "provenance_cue",
            Self::ActionRowCue => "action_row_cue",
        }
    }
}

/// The derived readiness of a connector — the resolver's verdict about whether a tool
/// is warm, cold, unavailable, or policy-blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiConnectorReadiness {
    /// Reachable and session-warmed.
    Warm,
    /// Reachable but cold (not yet warmed).
    Cold,
    /// Not reachable.
    Unavailable,
    /// Blocked by policy.
    PolicyBlocked,
}

impl M5AiConnectorReadiness {
    /// Every connector readiness, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Warm,
        Self::Cold,
        Self::Unavailable,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Warm => "warm",
            Self::Cold => "cold",
            Self::Unavailable => "unavailable",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// True when the connector can be invoked (reachable and not blocked).
    pub const fn is_invocable(self) -> bool {
        matches!(self, Self::Warm | Self::Cold)
    }

    /// True when the readiness needs operator attention before use.
    pub const fn needs_attention(self) -> bool {
        matches!(self, Self::Unavailable | Self::PolicyBlocked)
    }
}

/// The derived hardware fit of a local model pack against the available machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiModelHardwareFit {
    /// Fits comfortably within available memory.
    Fits,
    /// Fits only with memory pressure / swap.
    FitsWithSwap,
    /// Exceeds available memory.
    ExceedsMemory,
    /// Requires an accelerator that is not present.
    RequiresAccelerator,
}

impl M5AiModelHardwareFit {
    /// Every hardware fit, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Fits,
        Self::FitsWithSwap,
        Self::ExceedsMemory,
        Self::RequiresAccelerator,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fits => "fits",
            Self::FitsWithSwap => "fits_with_swap",
            Self::ExceedsMemory => "exceeds_memory",
            Self::RequiresAccelerator => "requires_accelerator",
        }
    }

    /// True when the fit blocks selection (exceeds memory or needs a missing
    /// accelerator).
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::ExceedsMemory | Self::RequiresAccelerator)
    }
}

/// The derived offline posture of a local model pack — how it depends on the network,
/// so offline locality is never hidden behind a generic `installed` state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiModelOfflinePosture {
    /// Runs fully offline; no network needed.
    RunsFullyOffline,
    /// Served from a local mirror.
    MirrorServed,
    /// Requires a network fetch before use.
    RequiresNetworkFetch,
    /// Locally cached and runs without network at invocation.
    LocalCached,
}

impl M5AiModelOfflinePosture {
    /// Every offline posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RunsFullyOffline,
        Self::MirrorServed,
        Self::RequiresNetworkFetch,
        Self::LocalCached,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunsFullyOffline => "runs_fully_offline",
            Self::MirrorServed => "mirror_served",
            Self::RequiresNetworkFetch => "requires_network_fetch",
            Self::LocalCached => "local_cached",
        }
    }

    /// True when the pack can be used with no network at invocation.
    pub const fn is_offline_capable(self) -> bool {
        matches!(
            self,
            Self::RunsFullyOffline | Self::MirrorServed | Self::LocalCached
        )
    }
}

/// The derived readiness of a local model pack — the resolver's verdict about whether
/// a pack is freely selectable or needs attention, never flattened to `installed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiModelPackReadiness {
    /// Installed, hardware-fit, and verified — freely selectable.
    ReadySelectable,
    /// Served from a mirror and selectable.
    MirroredReady,
    /// Available offline only, selectable within that locality.
    OfflineReady,
    /// An update is available; usable but not current.
    UpdatePending,
    /// Blocked by hardware fit.
    HardwareBlocked,
    /// Held pending provenance verification / quarantine review.
    VerificationHeld,
}

impl M5AiModelPackReadiness {
    /// Every model pack readiness, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReadySelectable,
        Self::MirroredReady,
        Self::OfflineReady,
        Self::UpdatePending,
        Self::HardwareBlocked,
        Self::VerificationHeld,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadySelectable => "ready_selectable",
            Self::MirroredReady => "mirrored_ready",
            Self::OfflineReady => "offline_ready",
            Self::UpdatePending => "update_pending",
            Self::HardwareBlocked => "hardware_blocked",
            Self::VerificationHeld => "verification_held",
        }
    }

    /// True when the pack can be selected.
    pub const fn is_selectable(self) -> bool {
        matches!(
            self,
            Self::ReadySelectable | Self::MirroredReady | Self::OfflineReady | Self::UpdatePending
        )
    }

    /// True when the pack needs attention before it can be used.
    pub const fn needs_attention(self) -> bool {
        matches!(self, Self::HardwareBlocked | Self::VerificationHeld)
    }
}

/// One bounded action a local model pack card offers, so a card never hides its
/// select / verify / remove affordances behind an opaque `installed` state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiModelPackAction {
    /// Select this pack for use.
    Select,
    /// Verify provenance / digest.
    Verify,
    /// Remove the pack from disk.
    Remove,
    /// Apply the available update.
    Update,
    /// Run the hardware-fit check.
    RunHardwareFitCheck,
}

impl M5AiModelPackAction {
    /// Every model pack action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Select,
        Self::Verify,
        Self::Remove,
        Self::Update,
        Self::RunHardwareFitCheck,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Select => "select",
            Self::Verify => "verify",
            Self::Remove => "remove",
            Self::Update => "update",
            Self::RunHardwareFitCheck => "run_hardware_fit_check",
        }
    }
}

/// A field the connector export carries so connector-row truth is reconstructable. The
/// fields in [`M5AiConnectorExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiConnectorExportField {
    /// The canonical connector id.
    CanonicalId,
    /// The publisher / source.
    PublisherSource,
    /// The execution locus.
    ExecutionLocus,
    /// The declared capabilities.
    Capabilities,
    /// The auth posture.
    AuthPosture,
    /// The derived connector readiness.
    ConnectorReadiness,
    /// Whether side effects are disclosed.
    SideEffectDisclosed,
    /// Whether the connector requires authority before invocation.
    RequiresAuthority,
}

impl M5AiConnectorExportField {
    /// Every connector export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CanonicalId,
        Self::PublisherSource,
        Self::ExecutionLocus,
        Self::Capabilities,
        Self::AuthPosture,
        Self::ConnectorReadiness,
        Self::SideEffectDisclosed,
        Self::RequiresAuthority,
    ];

    /// The connector export fields every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::CanonicalId,
        Self::ExecutionLocus,
        Self::Capabilities,
        Self::AuthPosture,
        Self::ConnectorReadiness,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalId => "canonical_id",
            Self::PublisherSource => "publisher_source",
            Self::ExecutionLocus => "execution_locus",
            Self::Capabilities => "capabilities",
            Self::AuthPosture => "auth_posture",
            Self::ConnectorReadiness => "connector_readiness",
            Self::SideEffectDisclosed => "side_effect_disclosed",
            Self::RequiresAuthority => "requires_authority",
        }
    }
}

/// A field the local-model export carries so pack-card truth is reconstructable. The
/// fields in [`M5AiModelPackExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiModelPackExportField {
    /// The model identity.
    ModelIdentity,
    /// The content digest.
    Digest,
    /// The size on disk in MB.
    DiskSizeMb,
    /// The hardware expectation label.
    HardwareExpectation,
    /// The derived hardware fit.
    HardwareFit,
    /// The derived offline posture.
    OfflinePosture,
    /// The derived model pack readiness.
    ModelPackReadiness,
    /// The bounded available actions.
    AvailableActions,
}

impl M5AiModelPackExportField {
    /// Every model-pack export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ModelIdentity,
        Self::Digest,
        Self::DiskSizeMb,
        Self::HardwareExpectation,
        Self::HardwareFit,
        Self::OfflinePosture,
        Self::ModelPackReadiness,
        Self::AvailableActions,
    ];

    /// The model-pack export fields every card must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::ModelIdentity,
        Self::Digest,
        Self::DiskSizeMb,
        Self::HardwareExpectation,
        Self::OfflinePosture,
        Self::ModelPackReadiness,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModelIdentity => "model_identity",
            Self::Digest => "digest",
            Self::DiskSizeMb => "disk_size_mb",
            Self::HardwareExpectation => "hardware_expectation",
            Self::HardwareFit => "hardware_fit",
            Self::OfflinePosture => "offline_posture",
            Self::ModelPackReadiness => "model_pack_readiness",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// True when a connector capability has side effects beyond a read-only query and so
/// must be disclosed before invocation.
pub const fn capability_is_side_effecting(capability: M5AiConnectorCapability) -> bool {
    !matches!(capability, M5AiConnectorCapability::ReadOnlyQuery)
}

// ---- connector resolver -------------------------------------------------

/// The full input to the connector-detail-row resolver for one connector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiConnectorRowResolutionInput {
    /// The opaque canonical connector id.
    pub canonical_id: String,
    /// The opaque publisher / source descriptor.
    pub publisher_source: String,
    /// Where the connector runs.
    pub execution_locus: M5AiExecutionLocus,
    /// The declared capability classes (must be non-empty).
    pub declared_capabilities: Vec<M5AiConnectorCapability>,
    /// How the connector authenticates.
    pub auth_posture: M5AiAuthPosture,
    /// True when policy blocks the connector.
    pub policy_blocked: bool,
    /// True when the connector is reachable.
    pub reachable: bool,
    /// True when the connector is session-warmed.
    pub session_warmed: bool,
    /// True when the connector's side effects are disclosed on the row.
    pub discloses_side_effects: bool,
}

/// The resolved connector-detail-row truth for one connector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedConnectorDetailRow {
    /// The opaque canonical connector id.
    pub canonical_id: String,
    /// The opaque publisher / source descriptor.
    pub publisher_source: String,
    /// Where the connector runs.
    pub execution_locus: M5AiExecutionLocus,
    /// The declared capability classes.
    pub declared_capabilities: Vec<M5AiConnectorCapability>,
    /// How the connector authenticates.
    pub auth_posture: M5AiAuthPosture,
    /// True when side effects are disclosed.
    pub discloses_side_effects: bool,
    /// The derived connector readiness.
    pub connector_readiness: M5AiConnectorReadiness,
    /// True when the connector can be invoked.
    pub is_invocable: bool,
    /// True when the readiness needs operator attention.
    pub needs_attention: bool,
    /// True when the connector runs locally.
    pub locus_is_local: bool,
    /// True when the connector depends on an authority grant before invocation.
    pub requires_authority_before_invocation: bool,
}

/// Errors returned by [`resolve_connector_detail_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AiConnectorRowResolutionError {
    /// The canonical id was empty.
    EmptyCanonicalId,
    /// The publisher / source was empty.
    EmptyPublisherSource,
    /// No capabilities were declared.
    EmptyCapabilities,
    /// A side-effecting capability was declared but side effects were not disclosed.
    SideEffectingCapabilityUndisclosed,
    /// A connector descriptor carried forbidden material.
    ForbiddenConnectorMaterial,
}

impl M5AiConnectorRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyCanonicalId => "empty_canonical_id",
            Self::EmptyPublisherSource => "empty_publisher_source",
            Self::EmptyCapabilities => "empty_capabilities",
            Self::SideEffectingCapabilityUndisclosed => "side_effecting_capability_undisclosed",
            Self::ForbiddenConnectorMaterial => "forbidden_connector_material",
        }
    }
}

impl fmt::Display for M5AiConnectorRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "ai connector row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AiConnectorRowResolutionError {}

/// Resolves one connector / tool-server detail row from its declared state.
///
/// The derived connector readiness is computed in a fixed blocking-first order: a
/// policy block wins first (a blocked connector never reads as ready), then an
/// unreachable connector reads as unavailable, then a session-warmed connector reads as
/// warm, and otherwise the connector reads as cold. A connector that declares any
/// side-effecting capability must disclose its side effects, and the row always carries
/// whether the connector depends on an authority grant before invocation — the reach,
/// locus, and auth posture are carried explicitly, never inferred away.
pub fn resolve_connector_detail_row(
    input: &M5AiConnectorRowResolutionInput,
) -> Result<M5ResolvedConnectorDetailRow, M5AiConnectorRowResolutionError> {
    if input.canonical_id.trim().is_empty() {
        return Err(M5AiConnectorRowResolutionError::EmptyCanonicalId);
    }
    if input.publisher_source.trim().is_empty() {
        return Err(M5AiConnectorRowResolutionError::EmptyPublisherSource);
    }
    if input.declared_capabilities.is_empty() {
        return Err(M5AiConnectorRowResolutionError::EmptyCapabilities);
    }
    if value_repr_is_forbidden(&input.canonical_id)
        || value_repr_is_forbidden(&input.publisher_source)
    {
        return Err(M5AiConnectorRowResolutionError::ForbiddenConnectorMaterial);
    }
    let has_side_effect = input
        .declared_capabilities
        .iter()
        .any(|cap| capability_is_side_effecting(*cap));
    if has_side_effect && !input.discloses_side_effects {
        return Err(M5AiConnectorRowResolutionError::SideEffectingCapabilityUndisclosed);
    }

    let connector_readiness =
        derive_connector_readiness(input.policy_blocked, input.reachable, input.session_warmed);
    // A connector depends on an authority grant before invocation when it can do
    // anything beyond a read-only query, or when it authenticates as anything other
    // than an unauthenticated connector.
    let requires_authority_before_invocation =
        has_side_effect || !matches!(input.auth_posture, M5AiAuthPosture::Unauthenticated);

    Ok(M5ResolvedConnectorDetailRow {
        canonical_id: input.canonical_id.clone(),
        publisher_source: input.publisher_source.clone(),
        execution_locus: input.execution_locus,
        declared_capabilities: input.declared_capabilities.clone(),
        auth_posture: input.auth_posture,
        discloses_side_effects: input.discloses_side_effects,
        connector_readiness,
        is_invocable: connector_readiness.is_invocable(),
        needs_attention: connector_readiness.needs_attention(),
        locus_is_local: input.execution_locus.is_local(),
        requires_authority_before_invocation,
    })
}

/// The fixed blocking-first connector-readiness ladder.
fn derive_connector_readiness(
    policy_blocked: bool,
    reachable: bool,
    session_warmed: bool,
) -> M5AiConnectorReadiness {
    if policy_blocked {
        M5AiConnectorReadiness::PolicyBlocked
    } else if !reachable {
        M5AiConnectorReadiness::Unavailable
    } else if session_warmed {
        M5AiConnectorReadiness::Warm
    } else {
        M5AiConnectorReadiness::Cold
    }
}

// ---- local model resolver -----------------------------------------------

/// The full input to the local-model-pack-card resolver for one pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiModelPackResolutionInput {
    /// The opaque model identity.
    pub model_identity: String,
    /// The opaque content digest.
    pub digest: String,
    /// The size on disk, in MB (must be non-zero — disk cost is never hidden).
    pub size_on_disk_mb: u64,
    /// The opaque hardware expectation label.
    pub hardware_expectation_label: String,
    /// The memory the pack requires, in MB.
    pub required_memory_mb: u64,
    /// The memory available on the machine, in MB.
    pub available_memory_mb: u64,
    /// True when the pack requires an accelerator.
    pub requires_accelerator: bool,
    /// True when an accelerator is present.
    pub accelerator_present: bool,
    /// The frozen pack lifecycle state.
    pub pack_state: M5AiModelPackState,
    /// True when the pack's provenance is verified.
    pub provenance_verified: bool,
    /// True when the pack requires a network fetch before use.
    pub requires_network_fetch: bool,
}

/// The resolved local-model-pack-card truth for one pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedLocalModelPackCard {
    /// The opaque model identity.
    pub model_identity: String,
    /// The opaque content digest.
    pub digest: String,
    /// The size on disk, in MB.
    pub size_on_disk_mb: u64,
    /// The opaque hardware expectation label.
    pub hardware_expectation_label: String,
    /// The frozen pack lifecycle state.
    pub pack_state: M5AiModelPackState,
    /// True when provenance is verified.
    pub provenance_verified: bool,
    /// The derived hardware fit.
    pub hardware_fit: M5AiModelHardwareFit,
    /// The derived offline posture.
    pub offline_posture: M5AiModelOfflinePosture,
    /// The derived model pack readiness.
    pub model_pack_readiness: M5AiModelPackReadiness,
    /// The bounded actions this card offers.
    pub available_actions: Vec<M5AiModelPackAction>,
    /// True when the pack can be selected.
    pub is_selectable: bool,
    /// True when the pack needs attention before use.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_local_model_pack_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AiModelPackResolutionError {
    /// The model identity was empty.
    EmptyModelIdentity,
    /// The digest was empty.
    EmptyDigest,
    /// The hardware expectation label was empty.
    EmptyHardwareExpectation,
    /// The size on disk was zero (disk cost must never be hidden).
    ZeroDiskSize,
    /// A model descriptor carried forbidden material.
    ForbiddenModelMaterial,
}

impl M5AiModelPackResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyModelIdentity => "empty_model_identity",
            Self::EmptyDigest => "empty_digest",
            Self::EmptyHardwareExpectation => "empty_hardware_expectation",
            Self::ZeroDiskSize => "zero_disk_size",
            Self::ForbiddenModelMaterial => "forbidden_model_material",
        }
    }
}

impl fmt::Display for M5AiModelPackResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "ai model pack resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AiModelPackResolutionError {}

/// Resolves one local model pack card from its declared state.
///
/// The derived hardware fit is computed from the required and available memory and the
/// accelerator signals; the model pack readiness is computed in a fixed blocking-first
/// order: quarantine or unverified provenance holds first, then a blocking hardware fit
/// blocks, then an available update reads as update-pending, then an offline-only pack
/// reads as offline-ready, then a mirrored pack reads as mirrored-ready, and otherwise
/// an installed pack reads as freely selectable. The offline posture and the bounded
/// select / verify / remove actions follow from those derivations, so disk cost,
/// hardware expectations, and offline locality are never hidden behind `installed`.
pub fn resolve_local_model_pack_card(
    input: &M5AiModelPackResolutionInput,
) -> Result<M5ResolvedLocalModelPackCard, M5AiModelPackResolutionError> {
    if input.model_identity.trim().is_empty() {
        return Err(M5AiModelPackResolutionError::EmptyModelIdentity);
    }
    if input.digest.trim().is_empty() {
        return Err(M5AiModelPackResolutionError::EmptyDigest);
    }
    if input.hardware_expectation_label.trim().is_empty() {
        return Err(M5AiModelPackResolutionError::EmptyHardwareExpectation);
    }
    if input.size_on_disk_mb == 0 {
        return Err(M5AiModelPackResolutionError::ZeroDiskSize);
    }
    if value_repr_is_forbidden(&input.model_identity)
        || value_repr_is_forbidden(&input.digest)
        || value_repr_is_forbidden(&input.hardware_expectation_label)
    {
        return Err(M5AiModelPackResolutionError::ForbiddenModelMaterial);
    }

    let hardware_fit = derive_hardware_fit(
        input.required_memory_mb,
        input.available_memory_mb,
        input.requires_accelerator,
        input.accelerator_present,
    );
    let model_pack_readiness =
        derive_model_readiness(input.pack_state, hardware_fit, input.provenance_verified);
    let offline_posture = derive_offline_posture(input.pack_state, input.requires_network_fetch);
    let available_actions = derive_available_actions(model_pack_readiness);

    Ok(M5ResolvedLocalModelPackCard {
        model_identity: input.model_identity.clone(),
        digest: input.digest.clone(),
        size_on_disk_mb: input.size_on_disk_mb,
        hardware_expectation_label: input.hardware_expectation_label.clone(),
        pack_state: input.pack_state,
        provenance_verified: input.provenance_verified,
        hardware_fit,
        offline_posture,
        model_pack_readiness,
        available_actions,
        is_selectable: model_pack_readiness.is_selectable(),
        needs_attention: model_pack_readiness.needs_attention(),
    })
}

/// Derives the hardware fit from the memory and accelerator signals.
fn derive_hardware_fit(
    required_memory_mb: u64,
    available_memory_mb: u64,
    requires_accelerator: bool,
    accelerator_present: bool,
) -> M5AiModelHardwareFit {
    if requires_accelerator && !accelerator_present {
        M5AiModelHardwareFit::RequiresAccelerator
    } else if required_memory_mb > available_memory_mb {
        M5AiModelHardwareFit::ExceedsMemory
    } else if required_memory_mb.saturating_mul(4) > available_memory_mb.saturating_mul(3) {
        // Required memory exceeds three-quarters of what is available — fits only under
        // memory pressure.
        M5AiModelHardwareFit::FitsWithSwap
    } else {
        M5AiModelHardwareFit::Fits
    }
}

/// The fixed blocking-first model-readiness ladder.
fn derive_model_readiness(
    pack_state: M5AiModelPackState,
    hardware_fit: M5AiModelHardwareFit,
    provenance_verified: bool,
) -> M5AiModelPackReadiness {
    if matches!(
        pack_state,
        M5AiModelPackState::Quarantined | M5AiModelPackState::ProvenanceUnverified
    ) || !provenance_verified
    {
        M5AiModelPackReadiness::VerificationHeld
    } else if matches!(pack_state, M5AiModelPackState::HardwareUnfit) || hardware_fit.is_blocking()
    {
        M5AiModelPackReadiness::HardwareBlocked
    } else if matches!(pack_state, M5AiModelPackState::UpdateAvailable) {
        M5AiModelPackReadiness::UpdatePending
    } else if matches!(pack_state, M5AiModelPackState::OfflineOnly) {
        M5AiModelPackReadiness::OfflineReady
    } else if matches!(pack_state, M5AiModelPackState::Mirrored) {
        M5AiModelPackReadiness::MirroredReady
    } else {
        M5AiModelPackReadiness::ReadySelectable
    }
}

/// Derives the offline posture from the pack state and network-fetch signal.
fn derive_offline_posture(
    pack_state: M5AiModelPackState,
    requires_network_fetch: bool,
) -> M5AiModelOfflinePosture {
    if matches!(pack_state, M5AiModelPackState::OfflineOnly) {
        M5AiModelOfflinePosture::RunsFullyOffline
    } else if matches!(pack_state, M5AiModelPackState::Mirrored) {
        M5AiModelOfflinePosture::MirrorServed
    } else if requires_network_fetch {
        M5AiModelOfflinePosture::RequiresNetworkFetch
    } else {
        M5AiModelOfflinePosture::LocalCached
    }
}

/// Derives the bounded action set from the model pack readiness.
fn derive_available_actions(readiness: M5AiModelPackReadiness) -> Vec<M5AiModelPackAction> {
    use M5AiModelPackAction as Action;
    match readiness {
        M5AiModelPackReadiness::VerificationHeld => vec![Action::Verify, Action::Remove],
        M5AiModelPackReadiness::HardwareBlocked => {
            vec![Action::RunHardwareFitCheck, Action::Remove]
        }
        M5AiModelPackReadiness::UpdatePending => {
            vec![
                Action::Select,
                Action::Update,
                Action::Verify,
                Action::Remove,
            ]
        }
        M5AiModelPackReadiness::ReadySelectable
        | M5AiModelPackReadiness::MirroredReady
        | M5AiModelPackReadiness::OfflineReady => {
            vec![Action::Select, Action::Verify, Action::Remove]
        }
    }
}

// ---- worked cases -------------------------------------------------------

/// One worked connector resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiConnectorRowResolutionCase {
    /// The resolver input.
    pub input: M5AiConnectorRowResolutionInput,
    /// The resolved truth. Must equal `resolve_connector_detail_row(&input)`.
    pub resolved: M5ResolvedConnectorDetailRow,
}

impl M5AiConnectorRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5AiConnectorRowResolutionInput) -> Self {
        let resolved = resolve_connector_detail_row(&input).expect("seed connector case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_connector_detail_row(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One worked local-model resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiModelPackResolutionCase {
    /// The resolver input.
    pub input: M5AiModelPackResolutionInput,
    /// The resolved truth. Must equal `resolve_local_model_pack_card(&input)`.
    pub resolved: M5ResolvedLocalModelPackCard,
}

impl M5AiModelPackResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5AiModelPackResolutionInput) -> Self {
        let resolved =
            resolve_local_model_pack_card(&input).expect("seed model pack case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_local_model_pack_card(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one routing consumer bound to the shared connector
/// and model anatomy, execution loci, capabilities, auth postures, connector
/// readinesses, model pack states, model readinesses, hardware fits, offline postures,
/// bounded actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiConnectorModelRow {
    /// Routing consumer family.
    pub consumer_surface: M5AiConnectorModelConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5AiQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 AI surface families that render / consume these components.
    pub surface_families: Vec<M5AiSurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5AiDeploymentLine>,
    /// Connector anatomy parts this row renders (must include the mandatory parts).
    pub connector_anatomy_parts: Vec<M5AiConnectorAnatomyPart>,
    /// Model-pack anatomy parts this card renders (must include the mandatory parts).
    pub model_anatomy_parts: Vec<M5AiModelPackAnatomyPart>,
    /// Execution loci this consumer distinguishes.
    pub execution_loci: Vec<M5AiExecutionLocus>,
    /// Connector capabilities this consumer names.
    pub connector_capabilities: Vec<M5AiConnectorCapability>,
    /// Auth postures this consumer distinguishes.
    pub auth_postures: Vec<M5AiAuthPosture>,
    /// Connector readinesses this consumer distinguishes.
    pub connector_readinesses: Vec<M5AiConnectorReadiness>,
    /// Model pack states this consumer distinguishes.
    pub model_pack_states: Vec<M5AiModelPackState>,
    /// Model pack readinesses this consumer distinguishes.
    pub model_pack_readinesses: Vec<M5AiModelPackReadiness>,
    /// Hardware fits this consumer distinguishes.
    pub hardware_fits: Vec<M5AiModelHardwareFit>,
    /// Offline postures this consumer distinguishes.
    pub offline_postures: Vec<M5AiModelOfflinePosture>,
    /// Bounded model pack actions this consumer offers.
    pub model_pack_actions: Vec<M5AiModelPackAction>,
    /// Connector export fields this row carries (must include the mandatory fields).
    pub connector_export_fields: Vec<M5AiConnectorExportField>,
    /// Model-pack export fields this card carries (must include the mandatory fields).
    pub model_export_fields: Vec<M5AiModelPackExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5AiAccessibilityRoute>,
    /// AI subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5AiConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5AiExecutionDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked connector resolutions proving the connector resolver on this consumer.
    pub connector_examples: Vec<M5AiConnectorRowResolutionCase>,
    /// Worked model-pack resolutions proving the model resolver on this consumer.
    pub model_examples: Vec<M5AiModelPackResolutionCase>,
    /// Hard invariant: this consumer never masks its execution locus or auth posture.
    /// MUST be `false`.
    pub masks_execution_locus_or_authority: bool,
    /// Hard invariant: this consumer never shows a blocked connector as ready. MUST be
    /// `false`.
    pub shows_blocked_connector_as_ready: bool,
    /// Hard invariant: this consumer never hides disk cost, hardware, or offline
    /// locality behind a generic `installed` state. MUST be `false`.
    pub hides_disk_hardware_or_offline_cost: bool,
    /// Hard invariant: this consumer never invents a parallel connector or model
    /// grammar. MUST be `false`.
    pub invents_parallel_connector_or_model_grammar: bool,
}

impl M5AiConnectorModelRow {
    /// True when the row declares every mandatory connector anatomy part.
    fn declares_mandatory_connector_anatomy(&self) -> bool {
        let present: BTreeSet<M5AiConnectorAnatomyPart> =
            self.connector_anatomy_parts.iter().copied().collect();
        M5AiConnectorAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory model-pack anatomy part.
    fn declares_mandatory_model_anatomy(&self) -> bool {
        let present: BTreeSet<M5AiModelPackAnatomyPart> =
            self.model_anatomy_parts.iter().copied().collect();
        M5AiModelPackAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory connector export field.
    fn declares_mandatory_connector_export(&self) -> bool {
        let present: BTreeSet<M5AiConnectorExportField> =
            self.connector_export_fields.iter().copied().collect();
        M5AiConnectorExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory model-pack export field.
    fn declares_mandatory_model_export(&self) -> bool {
        let present: BTreeSet<M5AiModelPackExportField> =
            self.model_export_fields.iter().copied().collect();
        M5AiModelPackExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_execution_locus_or_authority
            && !self.shows_blocked_connector_as_ready
            && !self.hides_disk_hardware_or_offline_cost
            && !self.invents_parallel_connector_or_model_grammar
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiConnectorModelVocabularySet {
    /// Routing-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Connector-anatomy-part tokens.
    pub connector_anatomy_parts: Vec<String>,
    /// Model-pack-anatomy-part tokens.
    pub model_anatomy_parts: Vec<String>,
    /// Execution-locus tokens.
    pub execution_loci: Vec<String>,
    /// Connector-readiness tokens.
    pub connector_readinesses: Vec<String>,
    /// Hardware-fit tokens.
    pub hardware_fits: Vec<String>,
    /// Offline-posture tokens.
    pub offline_postures: Vec<String>,
    /// Model-pack-readiness tokens.
    pub model_pack_readinesses: Vec<String>,
    /// Model-pack-action tokens.
    pub model_pack_actions: Vec<String>,
    /// Connector-export-field tokens.
    pub connector_export_fields: Vec<String>,
    /// Model-pack-export-field tokens.
    pub model_export_fields: Vec<String>,
    /// Connector-capability tokens (reused from the frozen matrix).
    pub connector_capabilities: Vec<String>,
    /// Auth-posture tokens (reused from the frozen matrix).
    pub auth_postures: Vec<String>,
    /// Model-pack-state tokens (reused from the frozen matrix).
    pub model_pack_states: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5AiConnectorModelVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5AiConnectorModelConsumerSurface::ALL, |v| v.as_str()),
            connector_anatomy_parts: tokens(&M5AiConnectorAnatomyPart::ALL, |v| v.as_str()),
            model_anatomy_parts: tokens(&M5AiModelPackAnatomyPart::ALL, |v| v.as_str()),
            execution_loci: tokens(&M5AiExecutionLocus::ALL, |v| v.as_str()),
            connector_readinesses: tokens(&M5AiConnectorReadiness::ALL, |v| v.as_str()),
            hardware_fits: tokens(&M5AiModelHardwareFit::ALL, |v| v.as_str()),
            offline_postures: tokens(&M5AiModelOfflinePosture::ALL, |v| v.as_str()),
            model_pack_readinesses: tokens(&M5AiModelPackReadiness::ALL, |v| v.as_str()),
            model_pack_actions: tokens(&M5AiModelPackAction::ALL, |v| v.as_str()),
            connector_export_fields: tokens(&M5AiConnectorExportField::ALL, |v| v.as_str()),
            model_export_fields: tokens(&M5AiModelPackExportField::ALL, |v| v.as_str()),
            connector_capabilities: tokens(&M5AiConnectorCapability::ALL, |v| v.as_str()),
            auth_postures: tokens(&M5AiAuthPosture::ALL, |v| v.as_str()),
            model_pack_states: tokens(&M5AiModelPackState::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5AiAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5AiConnectorModelGovernanceReview {
    /// One primitive pair carries connector and model truth on every consumer.
    pub one_primitive_carries_connector_and_model_truth: bool,
    /// The execution locus and authority are shown without a secondary inspector.
    pub execution_locus_and_authority_always_shown: bool,
    /// A policy-blocked or unreachable connector never reads as ready.
    pub connector_readiness_never_masks_blocked: bool,
    /// A side-effecting capability is always disclosed before invocation.
    pub side_effecting_capability_always_disclosed: bool,
    /// Disk cost, hardware expectations, and offline locality are always shown.
    pub disk_hardware_and_offline_always_shown: bool,
    /// A pack state is never flattened to a generic `installed` state.
    pub model_state_never_generic_installed: bool,
    /// The bounded actions reflect the derived readiness.
    pub bounded_actions_reflect_readiness: bool,
    /// The support / export packet reconstructs row and card truth.
    pub support_export_reconstructs_row_and_card_truth: bool,
    /// No consumer invents a second connector or model grammar.
    pub no_surface_invents_parallel_vocabulary: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Descriptors stay stable across UI, export, and support surfaces.
    pub descriptors_stable_across_ui_export_support: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiConnectorModelConsumerProjection {
    /// Settings, picker, inspector, evidence, and CLI/support consumers all consume the
    /// shared primitive pair.
    pub routing_surfaces_consume_shared_primitive: bool,
    /// The connector-readiness resolver reads a single canonical source.
    pub connector_readiness_reads_single_source: bool,
    /// The model-readiness resolver reads a single canonical source.
    pub model_readiness_reads_single_source: bool,
    /// The offline-posture derivation reads a single canonical source.
    pub offline_posture_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiConnectorModelProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiConnectorModelReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting AI audit.
    pub ai_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5AiConnectorModelPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AiConnectorModelPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Routing rows.
    pub rows: Vec<M5AiConnectorModelRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiConnectorModelVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiConnectorModelGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiConnectorModelConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiConnectorModelProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiConnectorModelReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 connector-detail-row / local-model-pack-card primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiConnectorModelPrimitivePacket {
    /// Record kind; must equal [`M5_AI_CONNECTOR_MODEL_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AI_CONNECTOR_MODEL_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Routing rows.
    pub rows: Vec<M5AiConnectorModelRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiConnectorModelVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiConnectorModelGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiConnectorModelConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiConnectorModelProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiConnectorModelReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AiConnectorModelPrimitivePacket {
    /// Builds an M5 connector/local-model-primitive packet from stable-lane input.
    pub fn new(input: M5AiConnectorModelPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_AI_CONNECTOR_MODEL_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_AI_CONNECTOR_MODEL_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
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

    /// Validates the M5 connector/local-model-primitive invariants.
    pub fn validate(&self) -> Vec<M5AiConnectorModelPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_AI_CONNECTOR_MODEL_PRIMITIVE_RECORD_KIND {
            violations.push(M5AiConnectorModelPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AI_CONNECTOR_MODEL_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5AiConnectorModelPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AiConnectorModelPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_connector_locus_and_authority(self, &mut violations);
        validate_connector_availability_coverage(self, &mut violations);
        validate_model_readiness_coverage(self, &mut violations);
        validate_offline_locality(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 ai connector/local-model primitive packet serializes"),
        ) {
            violations.push(M5AiConnectorModelPrimitiveViolation::RawMaterialInExport);
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
            .expect("m5 ai connector/local-model primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per routing consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,connector_anatomy,model_anatomy,execution_loci,connector_readinesses,model_readinesses,offline_postures,model_actions,connector_examples,model_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.connector_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.model_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.execution_loci, |v| v.as_str()),
                join_tokens(&row.connector_readinesses, |v| v.as_str()),
                join_tokens(&row.model_pack_readinesses, |v| v.as_str()),
                join_tokens(&row.offline_postures, |v| v.as_str()),
                join_tokens(&row.model_pack_actions, |v| v.as_str()),
                row.connector_examples.len(),
                row.model_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 AI Connector-Detail-Row and Local-Model-Pack-Card Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Routing consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Execution loci: {}\n",
            self.vocabulary_set.execution_loci.join(", ")
        ));
        out.push_str(&format!(
            "- Connector readinesses: {}\n",
            self.vocabulary_set.connector_readinesses.join(", ")
        ));
        out.push_str(&format!(
            "- Model pack readinesses: {}\n",
            self.vocabulary_set.model_pack_readinesses.join(", ")
        ));
        out.push_str(&format!(
            "- Offline postures: {}\n",
            self.vocabulary_set.offline_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Routing consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked connector rows: {}\n",
                row.connector_examples.len()
            ));
            for case in &row.connector_examples {
                out.push_str(&format!(
                    "    - `{}` at `{}` → `{}` (auth `{}`, authority-before-invocation `{}`)\n",
                    case.resolved.canonical_id,
                    case.resolved.execution_locus.as_str(),
                    case.resolved.connector_readiness.as_str(),
                    case.resolved.auth_posture.as_str(),
                    case.resolved.requires_authority_before_invocation
                ));
            }
            out.push_str(&format!(
                "  - Worked model cards: {}\n",
                row.model_examples.len()
            ));
            for case in &row.model_examples {
                out.push_str(&format!(
                    "    - `{}` ({} MB) → `{}` (fit `{}`, offline `{}`)\n",
                    case.resolved.model_identity,
                    case.resolved.size_on_disk_mb,
                    case.resolved.model_pack_readiness.as_str(),
                    case.resolved.hardware_fit.as_str(),
                    case.resolved.offline_posture.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 connector/local-model-primitive export.
#[derive(Debug)]
pub enum M5AiConnectorModelPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AiConnectorModelPrimitiveViolation>),
}

impl fmt::Display for M5AiConnectorModelPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 ai connector/local-model primitive export parse failed: {error}"
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
                    "m5 ai connector/local-model primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AiConnectorModelPrimitiveArtifactError {}

/// Validation failures emitted by [`M5AiConnectorModelPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AiConnectorModelPrimitiveViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required routing consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A routing row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory connector anatomy parts.
    MandatoryConnectorAnatomyMissing,
    /// A row omits one of the mandatory model-pack anatomy parts.
    MandatoryModelAnatomyMissing,
    /// A row omits one of the mandatory connector export fields.
    MandatoryConnectorExportMissing,
    /// A row omits one of the mandatory model-pack export fields.
    MandatoryModelExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked connector resolutions.
    ConnectorExampleMissing,
    /// A row declares no worked model-pack resolutions.
    ModelExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked connector resolution proves a locus disclosure with an
    /// authority-before-invocation requirement.
    ConnectorLocusAndAuthorityUnproven,
    /// No worked connector resolution proves both an invocable and a needs-attention
    /// connector.
    ConnectorAvailabilityCoverageUnproven,
    /// No worked model resolution proves both a selectable and a needs-attention pack.
    ModelReadinessCoverageUnproven,
    /// No worked model resolution proves an offline-capable pack with real disk cost.
    OfflineLocalityUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5AiConnectorModelPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryConnectorAnatomyMissing => "mandatory_connector_anatomy_missing",
            Self::MandatoryModelAnatomyMissing => "mandatory_model_anatomy_missing",
            Self::MandatoryConnectorExportMissing => "mandatory_connector_export_missing",
            Self::MandatoryModelExportMissing => "mandatory_model_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConnectorExampleMissing => "connector_example_missing",
            Self::ModelExampleMissing => "model_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ConnectorLocusAndAuthorityUnproven => "connector_locus_and_authority_unproven",
            Self::ConnectorAvailabilityCoverageUnproven => {
                "connector_availability_coverage_unproven"
            }
            Self::ModelReadinessCoverageUnproven => "model_readiness_coverage_unproven",
            Self::OfflineLocalityUnproven => "offline_locality_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 connector/local-model-primitive export.
pub fn current_stable_m5_ai_connector_model_primitive_export(
) -> Result<M5AiConnectorModelPrimitivePacket, M5AiConnectorModelPrimitiveArtifactError> {
    let packet: M5AiConnectorModelPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_ai_connector_detail_rows_and_local_model_pack_cards_across_claimed_m5_ai_routing_surfaces/support_export.json"
    )))
    .map_err(M5AiConnectorModelPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AiConnectorModelPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5AiConnectorModelPrimitivePacket,
    violations: &mut Vec<M5AiConnectorModelPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_AI_CONNECTOR_MODEL_SCHEMA_REF,
        M5_AI_CONNECTOR_MODEL_DOC_REF,
        M5_AI_CONNECTOR_MODEL_COMPONENT_MATRIX_REF,
        M5_AI_CONNECTOR_MODEL_GATEWAY_REF,
        M5_AI_CONNECTOR_MODEL_LOCAL_MODEL_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AiConnectorModelPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5AiConnectorModelPrimitivePacket,
    violations: &mut Vec<M5AiConnectorModelPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5AiConnectorModelPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5AiConnectorModelPrimitivePacket,
    violations: &mut Vec<M5AiConnectorModelPrimitiveViolation>,
) {
    let present: BTreeSet<M5AiConnectorModelConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5AiConnectorModelConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5AiConnectorModelPrimitiveViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.connector_anatomy_parts.is_empty()
            || row.model_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.execution_loci.is_empty()
            || row.connector_capabilities.is_empty()
            || row.auth_postures.is_empty()
            || row.connector_readinesses.is_empty()
            || row.model_pack_states.is_empty()
            || row.model_pack_readinesses.is_empty()
            || row.hardware_fits.is_empty()
            || row.offline_postures.is_empty()
            || row.model_pack_actions.is_empty()
        {
            violations.push(M5AiConnectorModelPrimitiveViolation::RowIncomplete);
        }
        if !row.declares_mandatory_connector_anatomy() {
            violations.push(M5AiConnectorModelPrimitiveViolation::MandatoryConnectorAnatomyMissing);
        }
        if !row.declares_mandatory_model_anatomy() {
            violations.push(M5AiConnectorModelPrimitiveViolation::MandatoryModelAnatomyMissing);
        }
        if !row.declares_mandatory_connector_export() {
            violations.push(M5AiConnectorModelPrimitiveViolation::MandatoryConnectorExportMissing);
        }
        if !row.declares_mandatory_model_export() {
            violations.push(M5AiConnectorModelPrimitiveViolation::MandatoryModelExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5AiAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5AiConnectorModelPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5AiConnectorModelPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5AiConnectorModelPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.connector_examples.is_empty() {
            violations.push(M5AiConnectorModelPrimitiveViolation::ConnectorExampleMissing);
        }
        if row.model_examples.is_empty() {
            violations.push(M5AiConnectorModelPrimitiveViolation::ModelExampleMissing);
        }
        if row
            .connector_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .model_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5AiConnectorModelPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5AiConnectorModelPrimitiveViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5AiConnectorModelPrimitiveViolation::RowInvariantViolated);
        }
    }
}

/// At least one worked connector resolution across the matrix must prove an invocable
/// connector that names its execution locus and requires an authority grant before
/// invocation — the acceptance-criterion example that a user can tell where a tool runs
/// and what authority it depends on before use.
fn validate_connector_locus_and_authority(
    packet: &M5AiConnectorModelPrimitivePacket,
    violations: &mut Vec<M5AiConnectorModelPrimitiveViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.connector_examples
            .iter()
            .any(|case| case.resolved.requires_authority_before_invocation)
    });
    if !proven {
        violations.push(M5AiConnectorModelPrimitiveViolation::ConnectorLocusAndAuthorityUnproven);
    }
}

/// At least one worked connector resolution must prove an invocable connector and at
/// least one must prove a needs-attention (unavailable or policy-blocked) connector —
/// the acceptance-criterion example that a blocked tool never reads as ready.
fn validate_connector_availability_coverage(
    packet: &M5AiConnectorModelPrimitivePacket,
    violations: &mut Vec<M5AiConnectorModelPrimitiveViolation>,
) {
    let has_invocable = packet.rows.iter().any(|row| {
        row.connector_examples
            .iter()
            .any(|case| case.resolved.is_invocable)
    });
    let has_attention = packet.rows.iter().any(|row| {
        row.connector_examples
            .iter()
            .any(|case| case.resolved.needs_attention)
    });
    if !(has_invocable && has_attention) {
        violations
            .push(M5AiConnectorModelPrimitiveViolation::ConnectorAvailabilityCoverageUnproven);
    }
}

/// At least one worked model resolution must prove a selectable pack and at least one
/// must prove a needs-attention (hardware-blocked or verification-held) pack — the
/// acceptance-criterion example that a hardware or provenance problem is never hidden
/// behind a generic `installed` state.
fn validate_model_readiness_coverage(
    packet: &M5AiConnectorModelPrimitivePacket,
    violations: &mut Vec<M5AiConnectorModelPrimitiveViolation>,
) {
    let has_selectable = packet.rows.iter().any(|row| {
        row.model_examples
            .iter()
            .any(|case| case.resolved.is_selectable)
    });
    let has_attention = packet.rows.iter().any(|row| {
        row.model_examples
            .iter()
            .any(|case| case.resolved.needs_attention)
    });
    if !(has_selectable && has_attention) {
        violations.push(M5AiConnectorModelPrimitiveViolation::ModelReadinessCoverageUnproven);
    }
}

/// At least one worked model resolution must prove an offline-capable pack that carries
/// a real (non-zero) disk cost — the acceptance-criterion example that offline locality
/// and disk cost are surfaced rather than hidden.
fn validate_offline_locality(
    packet: &M5AiConnectorModelPrimitivePacket,
    violations: &mut Vec<M5AiConnectorModelPrimitiveViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.model_examples.iter().any(|case| {
            case.resolved.offline_posture.is_offline_capable() && case.resolved.size_on_disk_mb > 0
        })
    });
    if !proven {
        violations.push(M5AiConnectorModelPrimitiveViolation::OfflineLocalityUnproven);
    }
}

fn validate_governance_review(
    packet: &M5AiConnectorModelPrimitivePacket,
    violations: &mut Vec<M5AiConnectorModelPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_connector_and_model_truth,
        review.execution_locus_and_authority_always_shown,
        review.connector_readiness_never_masks_blocked,
        review.side_effecting_capability_always_disclosed,
        review.disk_hardware_and_offline_always_shown,
        review.model_state_never_generic_installed,
        review.bounded_actions_reflect_readiness,
        review.support_export_reconstructs_row_and_card_truth,
        review.no_surface_invents_parallel_vocabulary,
        review.every_row_declares_accessibility_route,
        review.descriptors_stable_across_ui_export_support,
    ] {
        if !ok {
            violations.push(M5AiConnectorModelPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AiConnectorModelPrimitivePacket,
    violations: &mut Vec<M5AiConnectorModelPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.routing_surfaces_consume_shared_primitive,
        projection.connector_readiness_reads_single_source,
        projection.model_readiness_reads_single_source,
        projection.offline_posture_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5AiConnectorModelPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AiConnectorModelPrimitivePacket,
    violations: &mut Vec<M5AiConnectorModelPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AiConnectorModelPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AiConnectorModelPrimitivePacket,
    violations: &mut Vec<M5AiConnectorModelPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.ai_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5AiConnectorModelPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
