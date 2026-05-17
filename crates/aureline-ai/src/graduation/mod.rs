//! AI beta graduation state and support-export gates.
//!
//! This module reads the checked-in graduation state for claimed beta AI
//! surfaces and evaluates it against the provider/model registry. The state is
//! intentionally metadata-only: packets carry opaque refs, coarse envelope
//! classes, owner refs, eval-set refs, threshold refs, cost-profile refs, and
//! kill-switch refs, while raw prompts, transcripts, provider payloads,
//! credentials, and exact spend values stay outside the boundary.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::registry::{ProviderModelRegistryPacket, RegistryRouteCandidate};

/// Stable record-kind tag carried by [`AiGraduationState`].
pub const AI_GRADUATION_STATE_RECORD_KIND: &str = "ai_beta_graduation_state";

/// Stable record-kind tag carried by [`AiGraduationPacket`].
pub const AI_GRADUATION_PACKET_RECORD_KIND: &str = "graduation_packet_record";

/// Schema version for the beta graduation-state projection.
pub const AI_GRADUATION_STATE_SCHEMA_VERSION: u32 = 1;

/// Required eval evidence kinds for a beta AI graduation packet.
pub const REQUIRED_BETA_EVIDENCE_KINDS: &[&str] = &[
    "protected_eval_corpus_passed",
    "red_team_corpus_passed",
    "latency_envelope_measured",
    "cost_envelope_measured",
];

/// Consumer projection class that must read the same graduation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiGraduationConsumerSurfaceClass {
    /// Documentation or embedded help projection.
    Docs,
    /// Release, claim-manifest, or assurance-center projection.
    Release,
    /// Support-export projection.
    SupportExport,
    /// CLI or headless inspection projection.
    Cli,
}

impl AiGraduationConsumerSurfaceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Docs => "docs",
            Self::Release => "release",
            Self::SupportExport => "support_export",
            Self::Cli => "cli",
        }
    }
}

/// Promotion-gate state derived from the packet and registry row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiGraduationGateState {
    /// The registry row has a current packet and can keep its claimed posture.
    Promotable,
    /// The registry row remains visible but is downgraded until evidence recovers.
    Downgraded,
    /// The registry row cannot admit new AI dispatch.
    Blocked,
}

impl AiGraduationGateState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Promotable => "promotable",
            Self::Downgraded => "downgraded",
            Self::Blocked => "blocked",
        }
    }
}

/// Effective support class projected after graduation checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiGraduationSupportClass {
    /// Evidence is current and the row can preserve its claim.
    Supported,
    /// Evidence is missing or stale.
    EvidenceStale,
    /// Evidence exists but must be re-run or corrected before widening.
    RetestPending,
    /// No route or packet can support the row.
    Unsupported,
}

impl AiGraduationSupportClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::EvidenceStale => "evidence_stale",
            Self::RetestPending => "retest_pending",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Packet freshness class projected to support and docs readers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiGraduationFreshnessClass {
    /// A matching packet is present and unexpired.
    Current,
    /// A matching packet is present but expired.
    Stale,
    /// No matching packet is present.
    Missing,
}

impl AiGraduationFreshnessClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }
}

/// Projection registration proving a consumer reads one graduation state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiGraduationConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface_class: AiGraduationConsumerSurfaceClass,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Graduation state ref consumed by the projection.
    pub graduation_state_ref: String,
    /// Absolute timestamp the projection was generated.
    pub rendered_at: String,
}

/// Policy context carried by a graduation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiGraduationPolicyContext {
    /// Policy epoch that admitted the packet.
    pub policy_epoch: String,
    /// Workspace trust state at packet mint time.
    pub trust_state: String,
    /// Deployment profile class at packet mint time.
    pub deployment_profile_class: String,
    /// Execution-context id when the packet was minted.
    #[serde(default)]
    pub execution_context_id: String,
}

/// Eval evidence row carried by an AI graduation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiGraduationEvidenceEntry {
    /// Evidence kind such as protected eval, red team, latency, or cost.
    pub eval_evidence_kind: String,
    /// Verification posture for this evidence row.
    pub eval_posture_class: String,
    /// Opaque ref to the evaluation packet.
    pub evaluation_packet_ref: String,
    /// Capability covered by the evidence row.
    pub covers_capability_class: String,
    /// Coarse result summary class.
    #[serde(default)]
    pub result_summary_class: String,
    /// Review-safe summary.
    #[serde(default)]
    pub notes_summary: String,
}

/// Rollback plan carried by an AI graduation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiGraduationRollbackPlan {
    /// Rollback plan kind.
    pub rollback_plan_kind: String,
    /// Provider entry restored by rollback, when applicable.
    #[serde(default)]
    pub rollback_to_provider_entry_ref: String,
    /// Model entry restored by rollback, when applicable.
    #[serde(default)]
    pub rollback_to_model_entry_ref: String,
    /// Prompt pack restored by rollback, when applicable.
    #[serde(default)]
    pub rollback_to_prompt_pack_manifest_ref: String,
    /// Tool pack restored by rollback, when applicable.
    #[serde(default)]
    pub rollback_to_tool_pack_manifest_ref: String,
    /// Rollout state restored by rollback, when applicable.
    #[serde(default)]
    pub rollback_to_rollout_state_class: String,
    /// Review-safe trigger summary.
    #[serde(default)]
    pub rollback_trigger_summary: String,
    /// Role that owns the rollback decision.
    #[serde(default)]
    pub rollback_owner_role: String,
}

/// Current graduation packet for one claimed AI surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiGraduationPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Graduation-packet schema version.
    pub graduation_packet_schema_version: u32,
    /// Stable graduation packet id.
    pub graduation_packet_id: String,
    /// Stable workflow or surface id the packet gates.
    pub workflow_or_surface_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Promotion kind that minted this packet.
    pub promotion_kind: String,
    /// Target rollout state class.
    pub target_rollout_state_class: String,
    /// Prior rollout state class.
    #[serde(default)]
    pub prior_rollout_state_class: String,
    /// Provider entry this packet admits.
    pub provider_entry_ref: String,
    /// Model entry this packet admits.
    pub model_entry_ref: String,
    /// Prompt pack manifest this packet admits.
    pub prompt_pack_manifest_ref: String,
    /// Tool pack manifest this packet admits.
    pub tool_pack_manifest_ref: String,
    /// Opaque eval set ref used by this packet.
    pub eval_set_ref: String,
    /// Opaque eval thresholds ref used by this packet.
    pub eval_thresholds_ref: String,
    /// Coarse cost profile ref used by this packet.
    pub cost_profile_ref: String,
    /// Kill-switch ref that can revoke or narrow the row.
    pub kill_switch_ref: String,
    /// Owner ref for the packet and its renewal.
    pub owner_ref: String,
    /// Eval, red-team, latency, and cost evidence entries.
    #[serde(default)]
    pub eval_evidence_entries: Vec<AiGraduationEvidenceEntry>,
    /// Coarse latency envelope admitted by the packet.
    pub latency_envelope_class: String,
    /// Coarse cost envelope admitted by the packet.
    pub cost_envelope_class: String,
    /// Fallback posture when the selected route is unavailable.
    pub fallback_posture_class: String,
    /// Fallback provider entry, when the fallback pins one.
    #[serde(default)]
    pub fallback_target_provider_entry_ref: String,
    /// Fallback model entry, when the fallback pins one.
    #[serde(default)]
    pub fallback_target_model_entry_ref: String,
    /// Rollback plan for evidence or policy regressions.
    pub rollback_plan: AiGraduationRollbackPlan,
    /// Approval-ticket ref that admitted this packet.
    #[serde(default)]
    pub originating_approval_ticket_ref: String,
    /// Packet refs superseded by this packet.
    #[serde(default)]
    pub supersedes_graduation_packet_refs: Vec<String>,
    /// Assurance claim refs linked by the packet.
    #[serde(default)]
    pub linked_assurance_claim_refs: Vec<String>,
    /// Policy context that admitted the packet.
    pub policy_context: AiGraduationPolicyContext,
    /// Redaction class for packet projection.
    pub redaction_class: String,
    /// Review-safe packet summary.
    #[serde(default)]
    pub notes_summary: String,
    /// Timestamp the packet was minted.
    pub minted_at: String,
    /// Timestamp after which the packet is stale.
    pub expires_at: String,
}

impl AiGraduationPacket {
    /// Returns true when the packet has expired as of the supplied timestamp.
    pub fn is_stale_as_of(&self, as_of: &str) -> bool {
        !self.expires_at.trim().is_empty() && self.expires_at.as_str() <= as_of
    }

    /// Returns true when every required beta evidence kind is present.
    pub fn has_required_beta_evidence(&self) -> bool {
        let present = self
            .eval_evidence_entries
            .iter()
            .map(|entry| entry.eval_evidence_kind.as_str())
            .collect::<BTreeSet<_>>();
        REQUIRED_BETA_EVIDENCE_KINDS
            .iter()
            .all(|required| present.contains(required))
    }

    fn required_metadata_missing(&self) -> Vec<AiGraduationViolation> {
        let mut violations = Vec::new();
        if self.owner_ref.trim().is_empty() {
            violations.push(AiGraduationViolation::PacketMissingOwner);
        }
        if self.eval_set_ref.trim().is_empty() {
            violations.push(AiGraduationViolation::PacketMissingEvalSet);
        }
        if self.eval_thresholds_ref.trim().is_empty() {
            violations.push(AiGraduationViolation::PacketMissingThresholds);
        }
        if self.cost_profile_ref.trim().is_empty() {
            violations.push(AiGraduationViolation::PacketMissingCostProfile);
        }
        if self.kill_switch_ref.trim().is_empty() {
            violations.push(AiGraduationViolation::PacketMissingKillSwitch);
        }
        if self.fallback_posture_class.trim().is_empty() {
            violations.push(AiGraduationViolation::PacketMissingFallback);
        }
        if self.rollback_plan.rollback_plan_kind.trim().is_empty() {
            violations.push(AiGraduationViolation::PacketMissingRollbackPlan);
        }
        if self.originating_approval_ticket_ref.trim().is_empty() {
            violations.push(AiGraduationViolation::PacketMissingApprovalTicket);
        }
        if !self.has_required_beta_evidence() {
            violations.push(AiGraduationViolation::PacketMissingRequiredEvidence);
        }
        violations
    }
}

/// Canonical graduation state for claimed beta AI surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiGraduationState {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable state id consumed by release, docs, and support projections.
    pub graduation_state_id: String,
    /// Provider/model registry state this graduation state validates.
    pub registry_state_ref: String,
    /// Eval thresholds artifact ref read by every packet in this state.
    pub eval_thresholds_ref: String,
    /// Timestamp used for packet freshness decisions.
    pub as_of: String,
    /// Checked-in packet artifact refs.
    #[serde(default)]
    pub packet_refs: Vec<String>,
    /// Current packet records.
    #[serde(default)]
    pub packets: Vec<AiGraduationPacket>,
    /// Consumer projections that must read this same state.
    #[serde(default)]
    pub consumer_projections: Vec<AiGraduationConsumerProjection>,
    /// Source contracts consumed by this state.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

impl AiGraduationState {
    /// Validates the graduation state against the provider/model registry.
    pub fn validate(&self, registry: &ProviderModelRegistryPacket) -> Vec<AiGraduationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != AI_GRADUATION_STATE_RECORD_KIND {
            violations.push(AiGraduationViolation::WrongRecordKind);
        }
        if self.schema_version != AI_GRADUATION_STATE_SCHEMA_VERSION {
            violations.push(AiGraduationViolation::WrongSchemaVersion);
        }
        if self.graduation_state_id.trim().is_empty() {
            violations.push(AiGraduationViolation::MissingGraduationStateId);
        }
        if self.as_of.trim().is_empty() {
            violations.push(AiGraduationViolation::MissingAsOf);
        }
        if self.eval_thresholds_ref.trim().is_empty() {
            violations.push(AiGraduationViolation::MissingEvalThresholdsRef);
        }
        if self.source_contract_refs.is_empty() {
            violations.push(AiGraduationViolation::MissingSourceContractRefs);
        }
        if self.registry_state_ref != registry.registry_id {
            violations.push(AiGraduationViolation::RegistryStateMismatch);
        }
        if self.consumer_projection_missing(AiGraduationConsumerSurfaceClass::Docs)
            || self.consumer_projection_missing(AiGraduationConsumerSurfaceClass::Release)
            || self.consumer_projection_missing(AiGraduationConsumerSurfaceClass::SupportExport)
        {
            violations.push(AiGraduationViolation::ConsumerProjectionMissing);
        }
        if self.consumer_projection_drifted() {
            violations.push(AiGraduationViolation::ConsumerProjectionDrift);
        }

        let mut packet_ids = BTreeSet::new();
        for packet in &self.packets {
            if !packet_ids.insert(packet.graduation_packet_id.as_str()) {
                violations.push(AiGraduationViolation::DuplicatePacket);
            }
            violations.extend(self.validate_packet_envelope(packet, registry));
        }

        for surface in &registry.claimed_surfaces {
            violations.extend(self.surface_violations(registry, &surface.surface_id));
        }

        if self.contains_forbidden_boundary_material() {
            violations.push(AiGraduationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Builds support-export summaries for every claimed registry surface.
    pub fn support_summaries_for_registry(
        &self,
        registry: &ProviderModelRegistryPacket,
    ) -> Vec<AiGraduationSurfaceSupportSummary> {
        registry
            .claimed_surfaces
            .iter()
            .map(|surface| self.surface_support_summary(registry, &surface.surface_id))
            .collect()
    }

    /// Returns the derived graduation status for one registry surface.
    pub fn surface_status(
        &self,
        registry: &ProviderModelRegistryPacket,
        surface_id: &str,
    ) -> AiGraduationSurfaceStatus {
        let display_label = registry
            .claimed_surfaces
            .iter()
            .find(|surface| surface.surface_id == surface_id)
            .map(|surface| surface.display_label.clone())
            .unwrap_or_else(|| surface_id.to_owned());
        let resolution = registry.resolve_route_for_surface(surface_id);
        let Some(selected) = resolution.selected_candidate.as_ref() else {
            return AiGraduationSurfaceStatus {
                surface_id: surface_id.to_owned(),
                display_label,
                selected_provider_entry_ref: None,
                selected_model_entry_ref: None,
                graduation_packet_ref: None,
                owner_ref: None,
                eval_set_ref: None,
                eval_thresholds_ref: None,
                cost_profile_ref: None,
                kill_switch_ref: None,
                gate_state: AiGraduationGateState::Blocked,
                effective_support_class: AiGraduationSupportClass::Unsupported,
                freshness_class: AiGraduationFreshnessClass::Missing,
                downgrade_reason_tokens: vec![AiGraduationViolation::NoEligibleRouteForSurface
                    .as_str()
                    .to_owned()],
            };
        };

        let expected_packet_ref = registry
            .provider_entries
            .iter()
            .find(|provider| provider.provider_entry_id == selected.provider_entry_ref)
            .map(|provider| provider.graduation_packet_ref.clone());
        let packet = self.packet_for_surface(surface_id);
        let Some(packet) = packet else {
            return AiGraduationSurfaceStatus {
                surface_id: surface_id.to_owned(),
                display_label,
                selected_provider_entry_ref: Some(selected.provider_entry_ref.clone()),
                selected_model_entry_ref: Some(selected.model_entry_ref.clone()),
                graduation_packet_ref: expected_packet_ref,
                owner_ref: None,
                eval_set_ref: None,
                eval_thresholds_ref: None,
                cost_profile_ref: None,
                kill_switch_ref: None,
                gate_state: AiGraduationGateState::Downgraded,
                effective_support_class: AiGraduationSupportClass::EvidenceStale,
                freshness_class: AiGraduationFreshnessClass::Missing,
                downgrade_reason_tokens: vec![AiGraduationViolation::SurfaceMissingPacket
                    .as_str()
                    .to_owned()],
            };
        };

        let mut downgrade_reason_tokens = self
            .packet_surface_violations(packet, selected, expected_packet_ref.as_deref(), registry)
            .into_iter()
            .map(|violation| violation.as_str().to_owned())
            .collect::<Vec<_>>();
        if packet.is_stale_as_of(&self.as_of) {
            downgrade_reason_tokens.push(
                AiGraduationViolation::SurfacePacketStale
                    .as_str()
                    .to_owned(),
            );
        }

        let freshness_class = if packet.is_stale_as_of(&self.as_of) {
            AiGraduationFreshnessClass::Stale
        } else {
            AiGraduationFreshnessClass::Current
        };
        let (gate_state, effective_support_class) = if downgrade_reason_tokens.is_empty() {
            (
                AiGraduationGateState::Promotable,
                AiGraduationSupportClass::Supported,
            )
        } else if downgrade_reason_tokens
            .iter()
            .any(|token| token == AiGraduationViolation::SurfacePacketStale.as_str())
        {
            (
                AiGraduationGateState::Downgraded,
                AiGraduationSupportClass::EvidenceStale,
            )
        } else {
            (
                AiGraduationGateState::Downgraded,
                AiGraduationSupportClass::RetestPending,
            )
        };

        AiGraduationSurfaceStatus {
            surface_id: surface_id.to_owned(),
            display_label,
            selected_provider_entry_ref: Some(selected.provider_entry_ref.clone()),
            selected_model_entry_ref: Some(selected.model_entry_ref.clone()),
            graduation_packet_ref: Some(packet.graduation_packet_id.clone()),
            owner_ref: Some(packet.owner_ref.clone()),
            eval_set_ref: Some(packet.eval_set_ref.clone()),
            eval_thresholds_ref: Some(packet.eval_thresholds_ref.clone()),
            cost_profile_ref: Some(packet.cost_profile_ref.clone()),
            kill_switch_ref: Some(packet.kill_switch_ref.clone()),
            gate_state,
            effective_support_class,
            freshness_class,
            downgrade_reason_tokens,
        }
    }

    fn surface_support_summary(
        &self,
        registry: &ProviderModelRegistryPacket,
        surface_id: &str,
    ) -> AiGraduationSurfaceSupportSummary {
        let status = self.surface_status(registry, surface_id);
        AiGraduationSurfaceSupportSummary {
            surface_id: status.surface_id,
            display_label: status.display_label,
            graduation_state_ref: self.graduation_state_id.clone(),
            selected_provider_entry_ref: status.selected_provider_entry_ref,
            selected_model_entry_ref: status.selected_model_entry_ref,
            graduation_packet_ref: status.graduation_packet_ref,
            packet_freshness_token: status.freshness_class.as_str().to_owned(),
            promotion_gate_token: status.gate_state.as_str().to_owned(),
            effective_support_class_token: status.effective_support_class.as_str().to_owned(),
            owner_ref: status.owner_ref,
            eval_set_ref: status.eval_set_ref,
            eval_thresholds_ref: status.eval_thresholds_ref,
            cost_profile_ref: status.cost_profile_ref,
            kill_switch_ref: status.kill_switch_ref,
            downgrade_reason_tokens: status.downgrade_reason_tokens,
        }
    }

    fn validate_packet_envelope(
        &self,
        packet: &AiGraduationPacket,
        registry: &ProviderModelRegistryPacket,
    ) -> Vec<AiGraduationViolation> {
        let mut violations = Vec::new();
        if packet.record_kind != AI_GRADUATION_PACKET_RECORD_KIND {
            violations.push(AiGraduationViolation::PacketWrongEnvelope);
        }
        if packet.graduation_packet_schema_version != AI_GRADUATION_STATE_SCHEMA_VERSION {
            violations.push(AiGraduationViolation::PacketWrongEnvelope);
        }
        if packet.policy_context.policy_epoch != registry.policy_context.policy_epoch_ref {
            violations.push(AiGraduationViolation::PacketPolicyEpochMismatch);
        }
        if packet.eval_thresholds_ref != self.eval_thresholds_ref {
            violations.push(AiGraduationViolation::PacketThresholdsRefMismatch);
        }
        violations.extend(packet.required_metadata_missing());
        violations
    }

    fn surface_violations(
        &self,
        registry: &ProviderModelRegistryPacket,
        surface_id: &str,
    ) -> Vec<AiGraduationViolation> {
        let resolution = registry.resolve_route_for_surface(surface_id);
        let Some(selected) = resolution.selected_candidate.as_ref() else {
            return vec![AiGraduationViolation::NoEligibleRouteForSurface];
        };
        let expected_packet_ref = registry
            .provider_entries
            .iter()
            .find(|provider| provider.provider_entry_id == selected.provider_entry_ref)
            .map(|provider| provider.graduation_packet_ref.as_str());
        let Some(packet) = self.packet_for_surface(surface_id) else {
            return vec![AiGraduationViolation::SurfaceMissingPacket];
        };

        let mut violations =
            self.packet_surface_violations(packet, selected, expected_packet_ref, registry);
        if packet.is_stale_as_of(&self.as_of) {
            violations.push(AiGraduationViolation::SurfacePacketStale);
        }
        violations
    }

    fn packet_surface_violations(
        &self,
        packet: &AiGraduationPacket,
        selected: &RegistryRouteCandidate,
        expected_packet_ref: Option<&str>,
        registry: &ProviderModelRegistryPacket,
    ) -> Vec<AiGraduationViolation> {
        let mut violations = Vec::new();
        if expected_packet_ref != Some(packet.graduation_packet_id.as_str()) {
            violations.push(AiGraduationViolation::PacketIdDoesNotMatchRegistryRef);
        }
        if packet.provider_entry_ref != selected.provider_entry_ref {
            violations.push(AiGraduationViolation::PacketProviderMismatch);
        }
        if packet.model_entry_ref != selected.model_entry_ref {
            violations.push(AiGraduationViolation::PacketModelMismatch);
        }
        if packet.policy_context.policy_epoch != registry.policy_context.policy_epoch_ref {
            violations.push(AiGraduationViolation::PacketPolicyEpochMismatch);
        }
        if packet.eval_thresholds_ref != self.eval_thresholds_ref {
            violations.push(AiGraduationViolation::PacketThresholdsRefMismatch);
        }
        violations.extend(packet.required_metadata_missing());
        violations
    }

    fn packet_for_surface(&self, surface_id: &str) -> Option<&AiGraduationPacket> {
        self.packets
            .iter()
            .find(|packet| packet.workflow_or_surface_id == surface_id)
    }

    fn consumer_projection_missing(&self, class: AiGraduationConsumerSurfaceClass) -> bool {
        !self
            .consumer_projections
            .iter()
            .any(|projection| projection.consumer_surface_class == class)
    }

    fn consumer_projection_drifted(&self) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.graduation_state_ref != self.graduation_state_id
                || projection.projection_ref.trim().is_empty()
        })
    }

    fn contains_forbidden_boundary_material(&self) -> bool {
        serde_json::to_value(self)
            .ok()
            .is_some_and(|value| json_contains_forbidden_boundary_material(&value))
    }
}

/// Derived graduation status for one claimed AI surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiGraduationSurfaceStatus {
    /// Surface id.
    pub surface_id: String,
    /// Display label safe for support export.
    pub display_label: String,
    /// Selected provider entry, when route resolution admitted one.
    pub selected_provider_entry_ref: Option<String>,
    /// Selected model entry, when route resolution admitted one.
    pub selected_model_entry_ref: Option<String>,
    /// Graduation packet ref backing the row, when present.
    pub graduation_packet_ref: Option<String>,
    /// Packet owner ref, when present.
    pub owner_ref: Option<String>,
    /// Eval set ref, when present.
    pub eval_set_ref: Option<String>,
    /// Eval thresholds ref, when present.
    pub eval_thresholds_ref: Option<String>,
    /// Cost profile ref, when present.
    pub cost_profile_ref: Option<String>,
    /// Kill-switch ref, when present.
    pub kill_switch_ref: Option<String>,
    /// Derived promotion-gate state.
    pub gate_state: AiGraduationGateState,
    /// Derived support class.
    pub effective_support_class: AiGraduationSupportClass,
    /// Derived packet freshness class.
    pub freshness_class: AiGraduationFreshnessClass,
    /// Stable downgrade reason tokens.
    pub downgrade_reason_tokens: Vec<String>,
}

/// Support-export row for one claimed AI surface's graduation state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiGraduationSurfaceSupportSummary {
    /// Surface id.
    pub surface_id: String,
    /// Surface display label.
    pub display_label: String,
    /// Graduation state ref used by this summary.
    pub graduation_state_ref: String,
    /// Selected provider entry, when route resolution admitted one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_provider_entry_ref: Option<String>,
    /// Selected model entry, when route resolution admitted one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_model_entry_ref: Option<String>,
    /// Graduation packet ref backing the row, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graduation_packet_ref: Option<String>,
    /// Packet freshness token.
    pub packet_freshness_token: String,
    /// Promotion gate token.
    pub promotion_gate_token: String,
    /// Effective support-class token.
    pub effective_support_class_token: String,
    /// Packet owner ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_ref: Option<String>,
    /// Eval set ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eval_set_ref: Option<String>,
    /// Eval threshold ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eval_thresholds_ref: Option<String>,
    /// Cost profile ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_profile_ref: Option<String>,
    /// Kill switch ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kill_switch_ref: Option<String>,
    /// Stable downgrade reasons, empty when the row is promotable.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub downgrade_reason_tokens: Vec<String>,
}

/// Validation failures emitted by [`AiGraduationState::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiGraduationViolation {
    /// State record kind is wrong.
    WrongRecordKind,
    /// State schema version is wrong.
    WrongSchemaVersion,
    /// Graduation state id is missing.
    MissingGraduationStateId,
    /// Freshness timestamp is missing.
    MissingAsOf,
    /// Eval thresholds ref is missing.
    MissingEvalThresholdsRef,
    /// Source contract refs are missing.
    MissingSourceContractRefs,
    /// Graduation state points at a different provider registry.
    RegistryStateMismatch,
    /// Required consumer projection is absent.
    ConsumerProjectionMissing,
    /// Consumer projections do not read the same graduation state.
    ConsumerProjectionDrift,
    /// Two packets share the same packet id.
    DuplicatePacket,
    /// Packet envelope is wrong.
    PacketWrongEnvelope,
    /// Packet owner is missing.
    PacketMissingOwner,
    /// Packet eval set is missing.
    PacketMissingEvalSet,
    /// Packet thresholds are missing.
    PacketMissingThresholds,
    /// Packet cost profile is missing.
    PacketMissingCostProfile,
    /// Packet kill switch is missing.
    PacketMissingKillSwitch,
    /// Packet fallback posture is missing.
    PacketMissingFallback,
    /// Packet rollback plan is missing.
    PacketMissingRollbackPlan,
    /// Packet admitting approval ticket is missing.
    PacketMissingApprovalTicket,
    /// Packet lacks required beta evidence kinds.
    PacketMissingRequiredEvidence,
    /// Packet policy epoch does not match registry policy epoch.
    PacketPolicyEpochMismatch,
    /// Packet thresholds ref does not match the state thresholds ref.
    PacketThresholdsRefMismatch,
    /// Claimed surface has no admitted route.
    NoEligibleRouteForSurface,
    /// Claimed surface lacks a current packet.
    SurfaceMissingPacket,
    /// Claimed surface packet is expired.
    SurfacePacketStale,
    /// Packet provider does not match the selected registry provider.
    PacketProviderMismatch,
    /// Packet model does not match the selected registry model.
    PacketModelMismatch,
    /// Packet id does not match the registry graduation packet ref.
    PacketIdDoesNotMatchRegistryRef,
    /// Exportable fields contain raw boundary material.
    RawBoundaryMaterialInExport,
}

impl AiGraduationViolation {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "graduation_state_wrong_record_kind",
            Self::WrongSchemaVersion => "graduation_state_wrong_schema_version",
            Self::MissingGraduationStateId => "graduation_state_missing_id",
            Self::MissingAsOf => "graduation_state_missing_as_of",
            Self::MissingEvalThresholdsRef => "graduation_state_missing_eval_thresholds_ref",
            Self::MissingSourceContractRefs => "graduation_state_missing_source_contract_refs",
            Self::RegistryStateMismatch => "graduation_state_registry_state_mismatch",
            Self::ConsumerProjectionMissing => "graduation_state_consumer_projection_missing",
            Self::ConsumerProjectionDrift => "graduation_state_consumer_projection_drift",
            Self::DuplicatePacket => "graduation_state_duplicate_packet",
            Self::PacketWrongEnvelope => "graduation_packet_wrong_envelope",
            Self::PacketMissingOwner => "graduation_packet_missing_owner",
            Self::PacketMissingEvalSet => "graduation_packet_missing_eval_set",
            Self::PacketMissingThresholds => "graduation_packet_missing_thresholds",
            Self::PacketMissingCostProfile => "graduation_packet_missing_cost_profile",
            Self::PacketMissingKillSwitch => "graduation_packet_missing_kill_switch",
            Self::PacketMissingFallback => "graduation_packet_missing_fallback",
            Self::PacketMissingRollbackPlan => "graduation_packet_missing_rollback_plan",
            Self::PacketMissingApprovalTicket => "graduation_packet_missing_approval_ticket",
            Self::PacketMissingRequiredEvidence => "graduation_packet_missing_required_evidence",
            Self::PacketPolicyEpochMismatch => "graduation_packet_policy_epoch_mismatch",
            Self::PacketThresholdsRefMismatch => "graduation_packet_thresholds_ref_mismatch",
            Self::NoEligibleRouteForSurface => "graduation_surface_no_eligible_route",
            Self::SurfaceMissingPacket => "graduation_surface_missing_packet",
            Self::SurfacePacketStale => "graduation_surface_packet_stale",
            Self::PacketProviderMismatch => "graduation_packet_provider_mismatch",
            Self::PacketModelMismatch => "graduation_packet_model_mismatch",
            Self::PacketIdDoesNotMatchRegistryRef => {
                "graduation_packet_id_does_not_match_registry_ref"
            }
            Self::RawBoundaryMaterialInExport => "graduation_raw_boundary_material_in_export",
        }
    }
}

fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => {
            text.contains("://")
                || text.contains("api_key")
                || text.contains("oauth_token")
                || text.contains("Bearer ")
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Returns the checked-in AI beta graduation state artifact.
///
/// # Errors
///
/// Returns a JSON parse error if the checked-in state or packet artifact is
/// malformed.
pub fn current_beta_graduation_state() -> Result<AiGraduationState, serde_json::Error> {
    let mut state: AiGraduationState = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m3/graduation_packets/graduation_state.json"
    )))?;
    if state.packets.is_empty() {
        state.packets = current_beta_graduation_packet_artifacts()?
            .into_values()
            .collect();
    }
    Ok(state)
}

/// Returns checked-in standalone packet artifacts keyed by packet id.
///
/// # Errors
///
/// Returns a JSON parse error if any checked-in packet artifact is malformed.
pub fn current_beta_graduation_packet_artifacts(
) -> Result<BTreeMap<String, AiGraduationPacket>, serde_json::Error> {
    let packets = [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/ai/m3/graduation_packets/inline_chat_local_first_beta.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/ai/m3/graduation_packets/review_chat_cheapest_beta.json"
        )),
    ];
    let mut parsed = BTreeMap::new();
    for packet in packets {
        let packet: AiGraduationPacket = serde_json::from_str(packet)?;
        parsed.insert(packet.graduation_packet_id.clone(), packet);
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests;
