//! AI provider/model, prompt-pack, tool-schema, and local-model rollout packets.
//!
//! This module owns the metadata-only rollout publication packet that stable AI
//! routes consume when AI behavior changes without a desktop binary release.
//! The packet keeps provider/model enablement, prompt packs, tool-schema packs,
//! local-model packs, and feature-level AI rollout objects independently
//! promotable and independently revocable. It also preserves the mirror/offline
//! publication facts needed by support, compliance, and air-gapped deployments.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`AiRolloutPublicationPacket`].
pub const AI_ROLLOUT_PACKET_RECORD_KIND: &str = "ai_rollout_publication_packet";

/// Schema version for AI rollout publication packets.
pub const AI_ROLLOUT_PACKET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the rollout packet boundary schema.
pub const AI_ROLLOUT_PACKET_SCHEMA_REF: &str = "schemas/ai/ai-rollout-packet.schema.json";

/// Repo-relative path of the rollout governance document.
pub const AI_ROLLOUT_GOVERNANCE_DOC_REF: &str = "docs/ai/rollout-and-rollback-governance.md";

/// Repo-relative path of the checked AI rollout publication packet.
pub const AI_ROLLOUT_PUBLICATION_ARTIFACT_REF: &str =
    "artifacts/ai/m4/provider-model-prompt-tool-rollout/rollout_packet.json";

/// Repo-relative path of the checked AI rollout support export.
pub const AI_ROLLOUT_SUPPORT_EXPORT_REF: &str =
    "artifacts/ai/m4/provider-model-prompt-tool-rollout/support_export.json";

/// Repo-relative path of the checked AI rollout Markdown summary.
pub const AI_ROLLOUT_SUMMARY_REF: &str =
    "artifacts/ai/m4/provider-model-prompt-tool-rollout/summary.md";

/// Repo-relative path of the local-model pack publication manifest.
pub const LOCAL_MODEL_PACK_PUBLICATION_MANIFEST_REF: &str =
    "artifacts/ai/m4/local-model-pack-publication/manifest.json";

/// Ring vocabulary for AI-pack promotion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRolloutRingClass {
    /// Internal dogfood and canary cohort.
    Canary,
    /// Named pilot cohort with close owner review.
    Pilot,
    /// Broad rollout before stable/LTS admission.
    Broad,
    /// Long-term stable lane admission.
    Lts,
}

impl AiRolloutRingClass {
    /// Ring set a stable AI route must show before admission.
    pub const fn required_for_stable() -> [Self; 4] {
        [Self::Canary, Self::Pilot, Self::Broad, Self::Lts]
    }
}

/// Kind of AI rollout object carried by the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRolloutObjectKind {
    /// Provider or model enablement entry.
    ProviderModelEnablement,
    /// Prompt-pack version entry.
    PromptPack,
    /// Tool-schema pack entry.
    ToolSchemaPack,
    /// Local-model pack entry.
    LocalModelPack,
    /// Feature-level AI rollout entry.
    FeatureAiRollout,
}

/// Lifecycle state for an AI rollout object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRolloutStateClass {
    /// Drafted but not admitted to a ring.
    Draft,
    /// Currently in canary.
    Canary,
    /// Currently in pilot.
    Pilot,
    /// Currently in broad rollout.
    Broad,
    /// Stable/LTS admitted.
    Stable,
    /// Deprecated but still admitted with a replacement.
    Deprecated,
    /// Withdrawn and unavailable for new routes.
    Withdrawn,
    /// Policy-disabled.
    Disabled,
}

/// Fallback class used when a rollout object is withdrawn or disabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiFallbackRouteClass {
    /// Degrade to a verified local model route.
    LocalModel,
    /// Degrade to a user- or organization-keyed provider route.
    ByokProvider,
    /// Degrade to a non-AI manual workflow.
    ManualWorkflow,
}

/// Execution route class for a stable AI route row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRouteOriginClass {
    /// The route runs on a local model pack.
    LocalModel,
    /// The route uses a user- or organization-keyed provider.
    ByokProvider,
    /// The route uses a managed hosted provider.
    ManagedProvider,
    /// The route uses an enterprise gateway.
    EnterpriseGateway,
}

/// Revocation state carried by mirror/offline publication metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiPackRevocationStateClass {
    /// The pack is approved for use.
    Approved,
    /// The pack is revoked and must downgrade.
    Revoked,
    /// The pack is denied by policy.
    DeniedByPolicy,
}

/// Current fallback contract for an AI rollout object or stable route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiFallbackContract {
    /// Stable fallback contract identifier.
    pub fallback_contract_ref: String,
    /// Route class users are explicitly offered.
    pub fallback_route_class: AiFallbackRouteClass,
    /// Surface or command that exposes the fallback.
    pub fallback_surface_ref: String,
    /// Short support-safe reason label.
    pub user_visible_reason: String,
    /// True when editor core remains available outside the AI event.
    pub keeps_core_available: bool,
}

/// Governed object that can promote or roll back independently of the binary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRolloutObject {
    /// Stable object identifier.
    pub rollout_object_id: String,
    /// Object kind.
    pub object_kind: AiRolloutObjectKind,
    /// Owning team or council ref.
    pub owner_ref: String,
    /// Promotion artifact ref for this object.
    pub promotion_artifact_ref: String,
    /// Current version, digest, or route-policy ref.
    pub current_version_ref: String,
    /// Compatibility range carried by the object.
    pub compatibility_range_ref: String,
    /// Graduation packet backing the stable claim.
    pub graduation_packet_ref: String,
    /// Evidence packet or review ref backing the promotion.
    pub evidence_ref: String,
    /// Independent rollback, deny, or kill-switch lever.
    pub rollback_or_deny_lever_ref: String,
    /// Current lifecycle state.
    pub rollout_state: AiRolloutStateClass,
    /// Rings completed by this object.
    pub rings_completed: Vec<AiRolloutRingClass>,
    /// True when the object can be mirrored/offline-published.
    pub mirrorable: bool,
    /// Mirror/offline revocation state.
    pub revocation_state: AiPackRevocationStateClass,
    /// Current fallback behavior when the object is withdrawn.
    pub fallback_contract: AiFallbackContract,
}

/// Stable AI route row consumed by UI, CLI, docs/help, support, and release packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableAiRouteRow {
    /// Stable route identifier.
    pub route_id: String,
    /// User-visible surface or workflow id.
    pub surface_id: String,
    /// Route origin class.
    pub route_origin_class: AiRouteOriginClass,
    /// Provider registry entry ref.
    pub provider_entry_ref: String,
    /// Provider display label.
    pub provider_label: String,
    /// Model registry entry ref.
    pub model_entry_ref: String,
    /// Model display label.
    pub model_label: String,
    /// Prompt-pack version ref.
    pub prompt_pack_version_ref: String,
    /// Compatible tool-schema range ref.
    pub tool_schema_pack_range_ref: String,
    /// Local-model pack provenance ref, required for local-model routes.
    pub local_model_pack_provenance_ref: Option<String>,
    /// Routing-policy version ref.
    pub routing_policy_version_ref: String,
    /// Rollout object refs that jointly admit this route.
    pub rollout_object_refs: Vec<String>,
    /// Independent rollback or deny levers visible for this route.
    pub independent_rollback_refs: Vec<String>,
    /// Current graduation packet ref.
    pub graduation_packet_ref: String,
    /// Current fallback contract.
    pub fallback_contract: AiFallbackContract,
    /// Mirror publication row ref.
    pub mirror_publication_ref: String,
    /// Support export ref that can reconstruct the route.
    pub support_export_ref: String,
}

/// Downgrade receipt emitted when a rollout object is withdrawn or disabled.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiDowngradeReceipt {
    /// Stable receipt identifier.
    pub receipt_id: String,
    /// Withdrawn rollout object ref.
    pub withdrawn_object_ref: String,
    /// Affected route refs.
    pub affected_route_refs: Vec<String>,
    /// Support-safe cause label.
    pub cause: String,
    /// Explicit fallback route class.
    pub fallback_route_class: AiFallbackRouteClass,
    /// Fallback contract ref used by affected routes.
    pub fallback_contract_ref: String,
    /// Must be false for AI-pack withdrawal events.
    pub general_product_outage: bool,
    /// User-visible downgrade label.
    pub user_visible_label: String,
}

/// Mirror and offline publication facts for prompt/tool/local-model packs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiMirrorPublication {
    /// Stable publication ref.
    pub publication_ref: String,
    /// Approved prompt-pack refs available through the mirror.
    pub approved_prompt_pack_refs: Vec<String>,
    /// Approved tool-schema pack refs available through the mirror.
    pub approved_tool_schema_pack_refs: Vec<String>,
    /// Approved local-model pack refs available through the mirror.
    pub approved_local_model_pack_refs: Vec<String>,
    /// Provenance manifest ref.
    pub provenance_manifest_ref: String,
    /// Compatibility manifest ref.
    pub compatibility_manifest_ref: String,
    /// Revocation manifest ref.
    pub revocation_manifest_ref: String,
    /// Downgrade manifest ref.
    pub downgrade_manifest_ref: String,
    /// True if vendor-network access is needed to verify publication.
    pub vendor_network_required: bool,
    /// Offline drill refs proving publication without vendor network.
    pub offline_drill_refs: Vec<String>,
    /// Air-gapped profile refs that can consume this publication.
    pub air_gapped_profile_refs: Vec<String>,
}

/// Export-safe publication packet for stable AI-pack rollout truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRolloutPublicationPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Policy epoch ref used by the packet.
    pub policy_epoch_ref: String,
    /// Routing-policy version ref used by all stable routes.
    pub routing_policy_version_ref: String,
    /// Stable/LTS ring ref.
    pub stable_ring_ref: String,
    /// Independently promotable rollout objects.
    pub rollout_objects: Vec<AiRolloutObject>,
    /// Stable route rows consumed by product/support surfaces.
    pub stable_routes: Vec<StableAiRouteRow>,
    /// Downgrade receipts proving independent rollback behavior.
    pub downgrade_receipts: Vec<AiDowngradeReceipt>,
    /// Mirror/offline publication metadata.
    pub mirror_publication: AiMirrorPublication,
    /// Support export refs that consume this packet.
    pub support_export_refs: Vec<String>,
    /// Source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Redaction class token.
    pub redaction_class_token: String,
    /// UTC mint timestamp.
    pub minted_at: String,
}

impl AiRolloutPublicationPacket {
    /// Parses the checked-in rollout publication artifact.
    ///
    /// # Errors
    ///
    /// Returns a JSON parse error if the checked-in packet is malformed.
    pub fn current() -> Result<Self, serde_json::Error> {
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/ai/m4/provider-model-prompt-tool-rollout/rollout_packet.json"
        )))
    }

    /// Returns all validation findings for this publication packet.
    pub fn validate(&self) -> Vec<AiRolloutPacketViolation> {
        let mut findings = Vec::new();
        validate_header(self, &mut findings);
        validate_source_refs(self, &mut findings);

        let objects = self
            .rollout_objects
            .iter()
            .map(|object| (object.rollout_object_id.as_str(), object))
            .collect::<BTreeMap<_, _>>();
        let route_ids = self
            .stable_routes
            .iter()
            .map(|route| route.route_id.as_str())
            .collect::<BTreeSet<_>>();

        validate_rollout_objects(&objects, &mut findings);
        validate_stable_routes(self, &objects, &mut findings);
        validate_downgrade_receipts(self, &objects, &route_ids, &mut findings);
        validate_mirror_publication(self, &objects, &mut findings);

        findings.sort();
        findings.dedup();
        findings
    }

    /// Returns true when the packet has no validation findings.
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }
}

/// Validation findings emitted by [`AiRolloutPublicationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AiRolloutPacketViolation {
    /// The record kind or schema version is wrong.
    HeaderInvalid,
    /// Required source contract refs are missing.
    SourceContractMissing,
    /// No stable route rows are present.
    StableRouteMissing,
    /// A route is missing provider/model/prompt/tool/routing/graduation identity.
    RouteIdentityIncomplete,
    /// A local-model route lacks local-model pack provenance.
    LocalModelProvenanceMissing,
    /// A route references an unknown rollout object.
    RouteReferencesUnknownRolloutObject,
    /// A route does not expose independent rollback or deny levers.
    RouteRollbackLeverMissing,
    /// A stable route lacks one required object kind.
    RouteRequiredObjectKindMissing,
    /// A rollout object lacks promotion, evidence, compatibility, or rollback metadata.
    RolloutObjectMetadataIncomplete,
    /// A stable rollout object did not complete canary, pilot, broad, and LTS rings.
    StableObjectRingIncomplete,
    /// A fallback contract is incomplete or would block the local core.
    FallbackContractIncomplete,
    /// Withdrawal or disablement lacks a downgrade receipt.
    DowngradeReceiptMissing,
    /// A downgrade receipt references an unknown route or object.
    DowngradeReceiptReferenceInvalid,
    /// A downgrade receipt turns an AI-pack withdrawal into a product outage.
    DowngradeTreatsAiAsProductOutage,
    /// Mirror/offline metadata is incomplete.
    MirrorPublicationIncomplete,
    /// Mirror/offline publication still depends on vendor network access.
    MirrorRequiresVendorNetwork,
}

impl AiRolloutPacketViolation {
    /// Stable string token used by tests and support projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeaderInvalid => "header_invalid",
            Self::SourceContractMissing => "source_contract_missing",
            Self::StableRouteMissing => "stable_route_missing",
            Self::RouteIdentityIncomplete => "route_identity_incomplete",
            Self::LocalModelProvenanceMissing => "local_model_provenance_missing",
            Self::RouteReferencesUnknownRolloutObject => "route_references_unknown_rollout_object",
            Self::RouteRollbackLeverMissing => "route_rollback_lever_missing",
            Self::RouteRequiredObjectKindMissing => "route_required_object_kind_missing",
            Self::RolloutObjectMetadataIncomplete => "rollout_object_metadata_incomplete",
            Self::StableObjectRingIncomplete => "stable_object_ring_incomplete",
            Self::FallbackContractIncomplete => "fallback_contract_incomplete",
            Self::DowngradeReceiptMissing => "downgrade_receipt_missing",
            Self::DowngradeReceiptReferenceInvalid => "downgrade_receipt_reference_invalid",
            Self::DowngradeTreatsAiAsProductOutage => "downgrade_treats_ai_as_product_outage",
            Self::MirrorPublicationIncomplete => "mirror_publication_incomplete",
            Self::MirrorRequiresVendorNetwork => "mirror_requires_vendor_network",
        }
    }
}

impl fmt::Display for AiRolloutPacketViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Error for AiRolloutPacketViolation {}

fn validate_header(
    packet: &AiRolloutPublicationPacket,
    findings: &mut Vec<AiRolloutPacketViolation>,
) {
    if packet.record_kind != AI_ROLLOUT_PACKET_RECORD_KIND
        || packet.schema_version != AI_ROLLOUT_PACKET_SCHEMA_VERSION
        || packet.packet_id.is_empty()
        || packet.policy_epoch_ref.is_empty()
        || packet.routing_policy_version_ref.is_empty()
        || packet.stable_ring_ref.is_empty()
    {
        findings.push(AiRolloutPacketViolation::HeaderInvalid);
    }
}

fn validate_source_refs(
    packet: &AiRolloutPublicationPacket,
    findings: &mut Vec<AiRolloutPacketViolation>,
) {
    for required in [AI_ROLLOUT_PACKET_SCHEMA_REF, AI_ROLLOUT_GOVERNANCE_DOC_REF] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|contract_ref| contract_ref == required)
        {
            findings.push(AiRolloutPacketViolation::SourceContractMissing);
        }
    }
}

fn validate_rollout_objects(
    objects: &BTreeMap<&str, &AiRolloutObject>,
    findings: &mut Vec<AiRolloutPacketViolation>,
) {
    let required_rings = AiRolloutRingClass::required_for_stable()
        .into_iter()
        .collect::<BTreeSet<_>>();

    for object in objects.values() {
        if object.owner_ref.is_empty()
            || object.promotion_artifact_ref.is_empty()
            || object.current_version_ref.is_empty()
            || object.compatibility_range_ref.is_empty()
            || object.graduation_packet_ref.is_empty()
            || object.evidence_ref.is_empty()
            || object.rollback_or_deny_lever_ref.is_empty()
            || !object.mirrorable
        {
            findings.push(AiRolloutPacketViolation::RolloutObjectMetadataIncomplete);
        }
        validate_fallback(&object.fallback_contract, findings);
        if object.rollout_state == AiRolloutStateClass::Stable {
            let completed = object
                .rings_completed
                .iter()
                .copied()
                .collect::<BTreeSet<_>>();
            if !required_rings.is_subset(&completed) {
                findings.push(AiRolloutPacketViolation::StableObjectRingIncomplete);
            }
        }
    }
}

fn validate_stable_routes(
    packet: &AiRolloutPublicationPacket,
    objects: &BTreeMap<&str, &AiRolloutObject>,
    findings: &mut Vec<AiRolloutPacketViolation>,
) {
    if packet.stable_routes.is_empty() {
        findings.push(AiRolloutPacketViolation::StableRouteMissing);
        return;
    }

    for route in &packet.stable_routes {
        if route.provider_entry_ref.is_empty()
            || route.provider_label.is_empty()
            || route.model_entry_ref.is_empty()
            || route.model_label.is_empty()
            || route.prompt_pack_version_ref.is_empty()
            || route.tool_schema_pack_range_ref.is_empty()
            || route.routing_policy_version_ref.is_empty()
            || route.graduation_packet_ref.is_empty()
            || route.mirror_publication_ref.is_empty()
            || route.support_export_ref.is_empty()
        {
            findings.push(AiRolloutPacketViolation::RouteIdentityIncomplete);
        }
        if route.route_origin_class == AiRouteOriginClass::LocalModel
            && route
                .local_model_pack_provenance_ref
                .as_deref()
                .unwrap_or_default()
                .is_empty()
        {
            findings.push(AiRolloutPacketViolation::LocalModelProvenanceMissing);
        }
        if route.independent_rollback_refs.is_empty()
            || route.independent_rollback_refs.iter().any(String::is_empty)
        {
            findings.push(AiRolloutPacketViolation::RouteRollbackLeverMissing);
        }
        validate_fallback(&route.fallback_contract, findings);

        let mut object_kinds = BTreeSet::new();
        for object_ref in &route.rollout_object_refs {
            match objects.get(object_ref.as_str()) {
                Some(object) => {
                    object_kinds.insert(object.object_kind);
                }
                None => {
                    findings.push(AiRolloutPacketViolation::RouteReferencesUnknownRolloutObject)
                }
            }
        }

        for required in [
            AiRolloutObjectKind::ProviderModelEnablement,
            AiRolloutObjectKind::PromptPack,
            AiRolloutObjectKind::ToolSchemaPack,
            AiRolloutObjectKind::FeatureAiRollout,
        ] {
            if !object_kinds.contains(&required) {
                findings.push(AiRolloutPacketViolation::RouteRequiredObjectKindMissing);
            }
        }
        if route.route_origin_class == AiRouteOriginClass::LocalModel
            && !object_kinds.contains(&AiRolloutObjectKind::LocalModelPack)
        {
            findings.push(AiRolloutPacketViolation::RouteRequiredObjectKindMissing);
        }
    }
}

fn validate_downgrade_receipts(
    packet: &AiRolloutPublicationPacket,
    objects: &BTreeMap<&str, &AiRolloutObject>,
    route_ids: &BTreeSet<&str>,
    findings: &mut Vec<AiRolloutPacketViolation>,
) {
    let receipt_object_refs = packet
        .downgrade_receipts
        .iter()
        .map(|receipt| receipt.withdrawn_object_ref.as_str())
        .collect::<BTreeSet<_>>();

    for object in objects.values() {
        if matches!(
            object.rollout_state,
            AiRolloutStateClass::Withdrawn | AiRolloutStateClass::Disabled
        ) && !receipt_object_refs.contains(object.rollout_object_id.as_str())
        {
            findings.push(AiRolloutPacketViolation::DowngradeReceiptMissing);
        }
    }

    for receipt in &packet.downgrade_receipts {
        if !objects.contains_key(receipt.withdrawn_object_ref.as_str())
            || receipt.receipt_id.is_empty()
            || receipt.affected_route_refs.is_empty()
            || receipt.cause.is_empty()
            || receipt.fallback_contract_ref.is_empty()
            || receipt.user_visible_label.is_empty()
            || receipt
                .affected_route_refs
                .iter()
                .any(|route_ref| !route_ids.contains(route_ref.as_str()))
        {
            findings.push(AiRolloutPacketViolation::DowngradeReceiptReferenceInvalid);
        }
        if receipt.general_product_outage {
            findings.push(AiRolloutPacketViolation::DowngradeTreatsAiAsProductOutage);
        }
    }
}

fn validate_mirror_publication(
    packet: &AiRolloutPublicationPacket,
    objects: &BTreeMap<&str, &AiRolloutObject>,
    findings: &mut Vec<AiRolloutPacketViolation>,
) {
    let mirror = &packet.mirror_publication;
    if mirror.publication_ref.is_empty()
        || mirror.approved_prompt_pack_refs.is_empty()
        || mirror.approved_tool_schema_pack_refs.is_empty()
        || mirror.approved_local_model_pack_refs.is_empty()
        || mirror.provenance_manifest_ref.is_empty()
        || mirror.compatibility_manifest_ref.is_empty()
        || mirror.revocation_manifest_ref.is_empty()
        || mirror.downgrade_manifest_ref.is_empty()
        || mirror.offline_drill_refs.is_empty()
        || mirror.air_gapped_profile_refs.is_empty()
    {
        findings.push(AiRolloutPacketViolation::MirrorPublicationIncomplete);
    }
    if mirror.vendor_network_required {
        findings.push(AiRolloutPacketViolation::MirrorRequiresVendorNetwork);
    }

    let local_pack_objects = objects
        .values()
        .filter(|object| object.object_kind == AiRolloutObjectKind::LocalModelPack)
        .map(|object| object.rollout_object_id.as_str())
        .collect::<BTreeSet<_>>();
    if local_pack_objects.is_empty()
        || !mirror
            .approved_local_model_pack_refs
            .iter()
            .any(|pack_ref| local_pack_objects.contains(pack_ref.as_str()))
    {
        findings.push(AiRolloutPacketViolation::MirrorPublicationIncomplete);
    }
}

fn validate_fallback(fallback: &AiFallbackContract, findings: &mut Vec<AiRolloutPacketViolation>) {
    if fallback.fallback_contract_ref.is_empty()
        || fallback.fallback_surface_ref.is_empty()
        || fallback.user_visible_reason.is_empty()
        || !fallback.keeps_core_available
    {
        findings.push(AiRolloutPacketViolation::FallbackContractIncomplete);
    }
}

#[cfg(test)]
mod tests;
