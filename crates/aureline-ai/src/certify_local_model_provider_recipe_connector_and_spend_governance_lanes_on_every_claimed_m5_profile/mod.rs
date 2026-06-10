//! Certification of the local-model, provider, recipe, connector, and
//! spend-governance lanes on every claimed M5 profile.
//!
//! This module locks the canonical M5 service-governance certification into one
//! export-safe packet. Each [`LaneCertification`] binds one governed lane —
//! local-model packs, provider routing, recipe automation, external connectors,
//! and spend governance — to its claimed qualification, the per-profile coverage
//! that says where the lane is claimed and how it discloses provider, model,
//! locality, region, retention, cost owner, tool side-effect class, and
//! automation authority, a disclosure scorecard scored against fixed
//! thresholds, and a closed set of downgrade rules that narrow the claim instead
//! of hiding the lane.
//!
//! The certification is the single source of truth for whether each governed M5
//! lane may keep its public claim on a given profile — the channel, profile, and
//! provider families (local-only, BYOK-direct, managed-hosted, offline-mirror,
//! and hybrid-managed) qualified in this batch. It reuses the qualification and
//! downgrade vocabularies frozen by the M5 AI workflow matrix lane, the
//! routing-policy execution-mode vocabulary, the tool-gateway side-effect
//! vocabulary, and the recipe-pack automation-authority vocabulary rather than
//! inventing parallel terms, and it binds every certified lane to its canonical
//! source schema by requiring that schema's ref in `source_contract_refs` so no
//! surface may stay greener than this packet. The packet refuses to claim a lane
//! on a profile while hiding provider, model, region, retention, or cost,
//! refuses an execution mode that disagrees with the profile family, refuses to
//! widen a managed claim beyond a qualified family, refuses an automation
//! authority too weak for the side effect it grants, and narrows rather than
//! hides on stale proof.
//!
//! Raw prompt bodies, raw diffs, raw provider payloads, credentials, exact token
//! counts, exact cost amounts, and raw endpoint URLs stay outside the support
//! boundary; the packet carries only typed disclosure booleans and class tokens.
//!
//! The boundary schema is
//! [`schemas/ai/certify-local-model-provider-recipe-connector-and-spend-governance-lanes-on-every-claimed-m5-profile.schema.json`](../../../../schemas/ai/certify-local-model-provider-recipe-connector-and-spend-governance-lanes-on-every-claimed-m5-profile.schema.json).
//! The contract doc is
//! [`docs/automation/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile.md`](../../../../docs/automation/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile/`](../../../../fixtures/ai/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation::USER_AUTOMATION_SCHEMA_REF;
use crate::add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents::AGENT_BUDGET_SCHEMA_REF;
use crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass,
    M5_AI_WORKFLOW_MATRIX_ARTIFACT_REF, M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};
use crate::implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes::USAGE_REPORTING_SCHEMA_REF;
use crate::implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support::LOCAL_MODEL_PACK_SCHEMA_REF;
use crate::implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains::{
    RoutePolicyModeClass, ROUTING_POLICY_SCHEMA_REF,
};
use crate::implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay::{
    AutomationAuthorityClass, RECIPE_PACK_SCHEMA_REF,
};
use crate::materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors::PROVIDER_ROUTE_DISCLOSURE_SCHEMA_REF;
use crate::ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure::CONNECTOR_MANIFEST_SCHEMA_REF;
use crate::tool_gateway::{ToolSideEffectClass, TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF};

/// Stable record-kind tag carried by [`M5LaneCertificationPacket`].
pub const M5_LANE_CERTIFICATION_RECORD_KIND: &str =
    "certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile";

/// Schema version for M5 lane certification records.
pub const M5_LANE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_LANE_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/ai/certify-local-model-provider-recipe-connector-and-spend-governance-lanes-on-every-claimed-m5-profile.schema.json";

/// Repo-relative path of the certification contract doc.
pub const M5_LANE_CERTIFICATION_DOC_REF: &str =
    "docs/automation/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_LANE_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/ai/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile";

/// Repo-relative path of the checked support-export artifact.
pub const M5_LANE_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/ai/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_LANE_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/ai/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile.md";

/// One governed lane certified by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedLane {
    /// Local-model pack install, provenance, hardware fit, and mirror support.
    LocalModelPack,
    /// Provider/model registry, route disclosure, graduation, and routing
    /// policy.
    ProviderRouting,
    /// Signed/shared recipe packs and recorded-macro user automation.
    RecipeAutomation,
    /// External-tool gateway connectors and side-effect disclosure.
    ExternalConnector,
    /// Spend ledgers, quota ceilings, and customer-visible usage reporting.
    SpendGovernance,
}

impl CertifiedLane {
    /// Every certified lane, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalModelPack,
        Self::ProviderRouting,
        Self::RecipeAutomation,
        Self::ExternalConnector,
        Self::SpendGovernance,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalModelPack => "local_model_pack",
            Self::ProviderRouting => "provider_routing",
            Self::RecipeAutomation => "recipe_automation",
            Self::ExternalConnector => "external_connector",
            Self::SpendGovernance => "spend_governance",
        }
    }

    /// Canonical source schemas this lane's claim must stay aligned with.
    ///
    /// The certification binds every lane to the schema of every first-consumer
    /// packet it certifies and requires each ref in `source_contract_refs`, so a
    /// lane can never claim more than its canonical source schemas admit.
    pub const fn canonical_schema_refs(self) -> &'static [&'static str] {
        match self {
            Self::LocalModelPack => &[LOCAL_MODEL_PACK_SCHEMA_REF],
            Self::ProviderRouting => &[
                PROVIDER_ROUTE_DISCLOSURE_SCHEMA_REF,
                ROUTING_POLICY_SCHEMA_REF,
            ],
            Self::RecipeAutomation => &[RECIPE_PACK_SCHEMA_REF, USER_AUTOMATION_SCHEMA_REF],
            Self::ExternalConnector => &[
                CONNECTOR_MANIFEST_SCHEMA_REF,
                TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF,
            ],
            Self::SpendGovernance => &[AGENT_BUDGET_SCHEMA_REF, USAGE_REPORTING_SCHEMA_REF],
        }
    }

    /// Primary canonical source schema for display and indexing.
    pub const fn primary_schema_ref(self) -> &'static str {
        self.canonical_schema_refs()[0]
    }
}

/// One claimed M5 profile — a channel, profile, or provider family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaneProfile {
    /// On-device only; no request leaves the machine.
    LocalOnly,
    /// Vendor reached through the user's own credential.
    ByokDirect,
    /// First-party managed endpoint.
    ManagedHosted,
    /// On-device execution from a mirrored or offline pack channel.
    OfflineMirror,
    /// Managed control plane with a local or BYOK execution leg.
    HybridManaged,
}

impl M5LaneProfile {
    /// Every claimed profile, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOnly,
        Self::ByokDirect,
        Self::ManagedHosted,
        Self::OfflineMirror,
        Self::HybridManaged,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ByokDirect => "byok_direct",
            Self::ManagedHosted => "managed_hosted",
            Self::OfflineMirror => "offline_mirror",
            Self::HybridManaged => "hybrid_managed",
        }
    }

    /// Execution mode a claimed route on this profile must resolve to.
    ///
    /// Offline-mirror runs on-device with no egress, so it resolves to the same
    /// local execution mode as local-only; the profiles still differ by channel.
    pub const fn expected_mode(self) -> RoutePolicyModeClass {
        match self {
            Self::LocalOnly | Self::OfflineMirror => RoutePolicyModeClass::Local,
            Self::ByokDirect => RoutePolicyModeClass::Byok,
            Self::ManagedHosted | Self::HybridManaged => RoutePolicyModeClass::Managed,
        }
    }

    /// Whether this profile is a managed-service family.
    ///
    /// Managed-service claims may not be widened beyond the qualified families,
    /// so a claimed lane on a managed family must declare it stays within them.
    pub const fn is_managed_family(self) -> bool {
        matches!(self, Self::ManagedHosted | Self::HybridManaged)
    }
}

/// A disclosure dimension scored on every lane's certification scorecard.
///
/// The dimensions are exactly the invariant axes that must remain explicit and
/// exportable on every claimed row: none may hide behind generic AI language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneDisclosureDimension {
    /// Provider identity is disclosed, not generic.
    ProviderIdentity,
    /// Model and route identity are disclosed.
    ModelRoute,
    /// Execution locality agrees with where bytes run.
    ExecutionLocality,
    /// Region or residency is disclosed.
    RegionResidency,
    /// Retention posture is disclosed.
    RetentionPosture,
    /// Cost and the charge owner are disclosed, not hidden.
    CostBudgetOwner,
    /// Tool side-effect class is disclosed for every effect.
    ToolSideEffect,
    /// Automation authority is disclosed and bounded.
    AutomationAuthority,
}

impl LaneDisclosureDimension {
    /// Every required disclosure dimension, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ProviderIdentity,
        Self::ModelRoute,
        Self::ExecutionLocality,
        Self::RegionResidency,
        Self::RetentionPosture,
        Self::CostBudgetOwner,
        Self::ToolSideEffect,
        Self::AutomationAuthority,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderIdentity => "provider_identity",
            Self::ModelRoute => "model_route",
            Self::ExecutionLocality => "execution_locality",
            Self::RegionResidency => "region_residency",
            Self::RetentionPosture => "retention_posture",
            Self::CostBudgetOwner => "cost_budget_owner",
            Self::ToolSideEffect => "tool_side_effect",
            Self::AutomationAuthority => "automation_authority",
        }
    }
}

/// Pass/warn/fail status earned by one disclosure dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisclosureStatus {
    /// Score meets or exceeds the threshold with margin.
    Pass,
    /// Score meets the threshold but sits at the borderline.
    Warn,
    /// Score is below the threshold.
    Fail,
}

impl DisclosureStatus {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Warn => "warn",
            Self::Fail => "fail",
        }
    }

    /// Whether the recorded status is consistent with its score and threshold.
    ///
    /// `Pass` and `Warn` both require the score to meet the threshold; `Fail`
    /// requires it to fall short.
    pub const fn is_consistent(self, score: u8, threshold: u8) -> bool {
        match self {
            Self::Pass | Self::Warn => score >= threshold,
            Self::Fail => score < threshold,
        }
    }
}

/// One scored row of a lane's disclosure scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneDisclosureRow {
    /// Disclosure dimension being scored.
    pub dimension: LaneDisclosureDimension,
    /// Achieved score, on a 0..=100 scale.
    pub score: u8,
    /// Minimum score required for this dimension.
    pub threshold: u8,
    /// Recorded pass/warn/fail status.
    pub status: DisclosureStatus,
}

/// How one lane is claimed and disclosed on one M5 profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileCoverageRow {
    /// Profile this row covers.
    pub profile: M5LaneProfile,
    /// Whether the lane carries a public claim on this profile.
    pub claimed_on_profile: bool,
    /// Qualification the lane holds on this profile.
    pub profile_qualification: M5AiWorkflowQualificationClass,
    /// Execution mode a route on this profile resolves to.
    pub execution_mode: RoutePolicyModeClass,
    /// Provider identity is disclosed on this profile.
    pub provider_disclosed: bool,
    /// Model and route identity are disclosed on this profile.
    pub model_route_disclosed: bool,
    /// Region or residency is disclosed on this profile.
    pub region_disclosed: bool,
    /// Retention posture is disclosed on this profile.
    pub retention_disclosed: bool,
    /// Cost and charge owner are disclosed on this profile.
    pub cost_owner_disclosed: bool,
    /// Highest tool side-effect class this lane can produce on this profile.
    pub side_effect_class: ToolSideEffectClass,
    /// Automation authority granted to this lane on this profile.
    pub automation_authority: AutomationAuthorityClass,
    /// Managed claim stays within a qualified managed family.
    pub managed_claim_within_qualified_family: bool,
}

/// One downgrade rule that narrows a lane's claim when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneDowngradeRule {
    /// Trigger that fires this rule.
    pub trigger: M5AiWorkflowDowngradeTrigger,
    /// Qualification the lane narrows to when the trigger fires.
    pub narrowed_to: M5AiWorkflowQualificationClass,
    /// Whether tooling enforces this rule automatically.
    pub auto_enforced: bool,
    /// Review-safe rationale for the narrowing.
    pub rationale: String,
}

/// Full certification for one governed M5 lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneCertification {
    /// Governed lane being certified.
    pub lane: CertifiedLane,
    /// Headline qualification class claimed for this lane.
    pub claimed_qualification: M5AiWorkflowQualificationClass,
    /// Review-safe scope summary.
    pub scope_summary: String,
    /// Per-profile coverage rows.
    pub profile_coverage: Vec<ProfileCoverageRow>,
    /// Disclosure scorecard rows.
    pub disclosure_scorecard: Vec<LaneDisclosureRow>,
    /// Downgrade rules that narrow the claim.
    pub downgrade_rules: Vec<LaneDowngradeRule>,
    /// Required evidence packet refs backing the claim.
    pub evidence_packet_refs: Vec<String>,
}

impl LaneCertification {
    /// Qualification this lane narrows to when `trigger` fires.
    ///
    /// Returns the claimed qualification unchanged when no rule matches the
    /// trigger; this is the deterministic downgrade automation consumers and
    /// release tooling project instead of re-deriving narrowing locally.
    pub fn narrowed_qualification(
        &self,
        trigger: M5AiWorkflowDowngradeTrigger,
    ) -> M5AiWorkflowQualificationClass {
        self.downgrade_rules
            .iter()
            .find(|rule| rule.trigger == trigger)
            .map(|rule| rule.narrowed_to)
            .unwrap_or(self.claimed_qualification)
    }

    /// Whether this lane carries a publicly claimed qualification.
    ///
    /// Stable, Beta, and Preview are claimed lanes; Experimental, Held, and
    /// Unavailable are not.
    pub fn is_claimed(&self) -> bool {
        is_claimed_qualification(self.claimed_qualification)
    }
}

/// Proof freshness block for the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaneCertificationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed lanes.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5LaneCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LaneCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Per-lane certifications.
    pub lane_certifications: Vec<LaneCertification>,
    /// Proof freshness block.
    pub proof_freshness: M5LaneCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 lane certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaneCertificationPacket {
    /// Record kind; must equal [`M5_LANE_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_LANE_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Per-lane certifications.
    pub lane_certifications: Vec<LaneCertification>,
    /// Proof freshness block.
    pub proof_freshness: M5LaneCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5LaneCertificationPacket {
    /// Builds an M5 lane certification packet from stable-lane input.
    pub fn new(input: M5LaneCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_LANE_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_LANE_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            lane_certifications: input.lane_certifications,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 lane certification invariants.
    pub fn validate(&self) -> Vec<M5LaneCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_LANE_CERTIFICATION_RECORD_KIND {
            violations.push(M5LaneCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_LANE_CERTIFICATION_SCHEMA_VERSION {
            violations.push(M5LaneCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5LaneCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_lanes_present(self, &mut violations);
        for cert in &self.lane_certifications {
            validate_lane_certification(cert, &mut violations);
        }
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 lane certification packet serializes"),
        ) {
            violations.push(M5LaneCertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Count of lanes whose headline qualification is Stable.
    pub fn stable_lane_count(&self) -> usize {
        self.lane_certifications
            .iter()
            .filter(|cert| cert.claimed_qualification.is_stable())
            .count()
    }

    /// Count of (lane, profile) pairs that carry a public claim.
    pub fn claimed_profile_count(&self) -> usize {
        self.lane_certifications
            .iter()
            .flat_map(|cert| cert.profile_coverage.iter())
            .filter(|row| row.claimed_on_profile)
            .count()
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 lane certification packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Lane Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Lanes: {} ({} stable)\n",
            self.lane_certifications.len(),
            self.stable_lane_count()
        ));
        out.push_str(&format!(
            "- Claimed (lane, profile) pairs: {}\n",
            self.claimed_profile_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Lanes\n\n");
        for cert in &self.lane_certifications {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                cert.lane.as_str(),
                cert.claimed_qualification.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", cert.scope_summary));
            out.push_str(&format!(
                "  - Canonical schemas: {}\n",
                cert.lane
                    .canonical_schema_refs()
                    .iter()
                    .map(|r| format!("`{r}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str("  - Profiles:\n");
            for row in &cert.profile_coverage {
                out.push_str(&format!(
                    "    - `{}`: {} ({})\n",
                    row.profile.as_str(),
                    if row.claimed_on_profile {
                        row.profile_qualification.as_str()
                    } else {
                        "not claimed"
                    },
                    row.execution_mode.as_str()
                ));
            }
            out.push_str(&format!(
                "  - Disclosure dimensions: {} | Downgrade rules: {}\n",
                cert.disclosure_scorecard.len(),
                cert.downgrade_rules.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 lane certification export.
#[derive(Debug)]
pub enum M5LaneCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5LaneCertificationViolation>),
}

impl fmt::Display for M5LaneCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 lane certification export parse failed: {error}"
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
                    "m5 lane certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5LaneCertificationArtifactError {}

/// Validation failures emitted by [`M5LaneCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5LaneCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A lane's canonical source schema is not bound in source contracts.
    LaneCanonicalSchemaUnbound,
    /// A required lane is missing from the certification.
    RequiredLaneMissing,
    /// A lane appears more than once.
    DuplicateLane,
    /// A lane certification row is incomplete.
    LaneRowIncomplete,
    /// A disclosure scorecard does not cover every required dimension.
    DisclosureDimensionMissing,
    /// A disclosure score or threshold is out of the 0..=100 range.
    DisclosureScoreOutOfRange,
    /// A disclosure status is inconsistent with its score and threshold.
    DisclosureStatusInconsistent,
    /// A Stable-claimed lane has a disclosure dimension that is not passing.
    StableLaneDisclosureNotPassing,
    /// A Beta-claimed lane has a failing disclosure dimension.
    BetaLaneDisclosureFailing,
    /// A lane does not cover every required profile.
    RequiredProfileMissing,
    /// A profile appears more than once in a lane's coverage.
    DuplicateProfile,
    /// A profile coverage row's claim flag disagrees with its qualification.
    ProfileClaimFlagInconsistent,
    /// A profile coverage row claims more than the lane's headline claim.
    ProfileQualificationExceedsClaim,
    /// A claimed profile row hides provider, model, region, retention, or cost.
    ClaimedProfileMissingDisclosure,
    /// A claimed profile row's execution mode disagrees with the profile family.
    LocalityProfileMismatch,
    /// A claimed managed-family row does not stay within a qualified family.
    ManagedClaimWidened,
    /// A claimed profile row's automation authority is too weak for its effect.
    AutomationAuthorityInsufficient,
    /// A claimed lane is missing required evidence packet refs.
    ClaimedLaneMissingEvidence,
    /// A lane has no downgrade rules.
    DowngradeRulesMissing,
    /// A lane's downgrade rules omit the proof-stale trigger.
    DowngradeRuleMissingProofStale,
    /// A downgrade rule does not narrow below the claimed qualification.
    DowngradeRuleNotNarrowing,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5LaneCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::LaneCanonicalSchemaUnbound => "lane_canonical_schema_unbound",
            Self::RequiredLaneMissing => "required_lane_missing",
            Self::DuplicateLane => "duplicate_lane",
            Self::LaneRowIncomplete => "lane_row_incomplete",
            Self::DisclosureDimensionMissing => "disclosure_dimension_missing",
            Self::DisclosureScoreOutOfRange => "disclosure_score_out_of_range",
            Self::DisclosureStatusInconsistent => "disclosure_status_inconsistent",
            Self::StableLaneDisclosureNotPassing => "stable_lane_disclosure_not_passing",
            Self::BetaLaneDisclosureFailing => "beta_lane_disclosure_failing",
            Self::RequiredProfileMissing => "required_profile_missing",
            Self::DuplicateProfile => "duplicate_profile",
            Self::ProfileClaimFlagInconsistent => "profile_claim_flag_inconsistent",
            Self::ProfileQualificationExceedsClaim => "profile_qualification_exceeds_claim",
            Self::ClaimedProfileMissingDisclosure => "claimed_profile_missing_disclosure",
            Self::LocalityProfileMismatch => "locality_profile_mismatch",
            Self::ManagedClaimWidened => "managed_claim_widened",
            Self::AutomationAuthorityInsufficient => "automation_authority_insufficient",
            Self::ClaimedLaneMissingEvidence => "claimed_lane_missing_evidence",
            Self::DowngradeRulesMissing => "downgrade_rules_missing",
            Self::DowngradeRuleMissingProofStale => "downgrade_rule_missing_proof_stale",
            Self::DowngradeRuleNotNarrowing => "downgrade_rule_not_narrowing",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in M5 lane certification export.
pub fn current_m5_lane_certification_export(
) -> Result<M5LaneCertificationPacket, M5LaneCertificationArtifactError> {
    let packet: M5LaneCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile/support_export.json"
    )))
    .map_err(M5LaneCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5LaneCertificationArtifactError::Validation(violations))
    }
}

/// Whether a qualification class is a publicly claimed lane.
fn is_claimed_qualification(class: M5AiWorkflowQualificationClass) -> bool {
    matches!(
        class,
        M5AiWorkflowQualificationClass::Stable
            | M5AiWorkflowQualificationClass::Beta
            | M5AiWorkflowQualificationClass::Preview
    )
}

/// Ordinal rank used to compare qualification severity for downgrade rules.
///
/// Higher means a stronger public claim, so a downgrade must move to a strictly
/// lower rank.
fn qualification_rank(class: M5AiWorkflowQualificationClass) -> u8 {
    match class {
        M5AiWorkflowQualificationClass::Unavailable => 0,
        M5AiWorkflowQualificationClass::Held => 1,
        M5AiWorkflowQualificationClass::Experimental => 2,
        M5AiWorkflowQualificationClass::Preview => 3,
        M5AiWorkflowQualificationClass::Beta => 4,
        M5AiWorkflowQualificationClass::Stable => 5,
    }
}

/// Minimum automation-authority rank required to grant a side-effect class.
fn required_authority_rank(side_effect: ToolSideEffectClass) -> u8 {
    match side_effect {
        ToolSideEffectClass::InspectOnly => 0,
        ToolSideEffectClass::LocalReversibleEdit | ToolSideEffectClass::AutomationAdmissionOnly => {
            1
        }
        ToolSideEffectClass::LocalDestructiveEdit
        | ToolSideEffectClass::PrivilegedInspectionAttach => 2,
        ToolSideEffectClass::ExternalReversibleComment => 3,
        ToolSideEffectClass::ExternalIrreversiblePublish
        | ToolSideEffectClass::CredentialHandleProjection
        | ToolSideEffectClass::PolicyOrTrustMutation
        | ToolSideEffectClass::CapabilityWidening => 4,
    }
}

/// Rank of an automation authority; higher grants more side effects.
fn authority_rank(authority: AutomationAuthorityClass) -> u8 {
    match authority {
        AutomationAuthorityClass::InspectOnlyNoAuthority => 0,
        AutomationAuthorityClass::LocalReversibleOnly => 1,
        AutomationAuthorityClass::LocalWithApproval => 2,
        AutomationAuthorityClass::ExternalReversibleWithApproval => 3,
        AutomationAuthorityClass::ExternalIrreversibleAdminGated
        | AutomationAuthorityClass::ManagedOnlyTemplateAuthority => 4,
    }
}

fn validate_source_contracts(
    packet: &M5LaneCertificationPacket,
    violations: &mut Vec<M5LaneCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_LANE_CERTIFICATION_SCHEMA_REF,
        M5_LANE_CERTIFICATION_DOC_REF,
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
        M5_AI_WORKFLOW_MATRIX_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5LaneCertificationViolation::MissingSourceContracts);
            break;
        }
    }

    for lane in CertifiedLane::ALL {
        if packet
            .lane_certifications
            .iter()
            .any(|cert| cert.lane == lane)
            && !lane
                .canonical_schema_refs()
                .iter()
                .all(|schema_ref| refs.contains(schema_ref))
        {
            violations.push(M5LaneCertificationViolation::LaneCanonicalSchemaUnbound);
            break;
        }
    }
}

fn validate_lanes_present(
    packet: &M5LaneCertificationPacket,
    violations: &mut Vec<M5LaneCertificationViolation>,
) {
    let mut seen: BTreeSet<CertifiedLane> = BTreeSet::new();
    for cert in &packet.lane_certifications {
        if !seen.insert(cert.lane) {
            violations.push(M5LaneCertificationViolation::DuplicateLane);
        }
    }
    for required in CertifiedLane::ALL {
        if !seen.contains(&required) {
            violations.push(M5LaneCertificationViolation::RequiredLaneMissing);
            return;
        }
    }
}

fn validate_lane_certification(
    cert: &LaneCertification,
    violations: &mut Vec<M5LaneCertificationViolation>,
) {
    if cert.scope_summary.trim().is_empty() {
        violations.push(M5LaneCertificationViolation::LaneRowIncomplete);
    }

    validate_scorecard(cert, violations);
    validate_profile_coverage(cert, violations);
    validate_downgrade_rules(cert, violations);

    if cert.is_claimed() && cert.evidence_packet_refs.is_empty() {
        violations.push(M5LaneCertificationViolation::ClaimedLaneMissingEvidence);
    }
}

fn validate_scorecard(
    cert: &LaneCertification,
    violations: &mut Vec<M5LaneCertificationViolation>,
) {
    let covered: BTreeSet<LaneDisclosureDimension> = cert
        .disclosure_scorecard
        .iter()
        .map(|row| row.dimension)
        .collect();
    for required in LaneDisclosureDimension::ALL {
        if !covered.contains(&required) {
            violations.push(M5LaneCertificationViolation::DisclosureDimensionMissing);
            break;
        }
    }

    let is_stable = cert.claimed_qualification.is_stable();
    let is_beta = matches!(
        cert.claimed_qualification,
        M5AiWorkflowQualificationClass::Beta
    );
    for row in &cert.disclosure_scorecard {
        if row.score > 100 || row.threshold > 100 {
            violations.push(M5LaneCertificationViolation::DisclosureScoreOutOfRange);
        }
        if !row.status.is_consistent(row.score, row.threshold) {
            violations.push(M5LaneCertificationViolation::DisclosureStatusInconsistent);
        }
        if is_stable && row.status != DisclosureStatus::Pass {
            violations.push(M5LaneCertificationViolation::StableLaneDisclosureNotPassing);
        }
        if is_beta && row.status == DisclosureStatus::Fail {
            violations.push(M5LaneCertificationViolation::BetaLaneDisclosureFailing);
        }
    }
}

fn validate_profile_coverage(
    cert: &LaneCertification,
    violations: &mut Vec<M5LaneCertificationViolation>,
) {
    let mut seen: BTreeSet<M5LaneProfile> = BTreeSet::new();
    let mut duplicate = false;
    for row in &cert.profile_coverage {
        if !seen.insert(row.profile) {
            duplicate = true;
        }
    }
    if duplicate {
        violations.push(M5LaneCertificationViolation::DuplicateProfile);
    }
    for required in M5LaneProfile::ALL {
        if !seen.contains(&required) {
            violations.push(M5LaneCertificationViolation::RequiredProfileMissing);
            break;
        }
    }

    let claimed_rank = qualification_rank(cert.claimed_qualification);
    for row in &cert.profile_coverage {
        if row.claimed_on_profile != is_claimed_qualification(row.profile_qualification) {
            violations.push(M5LaneCertificationViolation::ProfileClaimFlagInconsistent);
        }
        if qualification_rank(row.profile_qualification) > claimed_rank {
            violations.push(M5LaneCertificationViolation::ProfileQualificationExceedsClaim);
        }

        if !row.claimed_on_profile {
            continue;
        }

        if !(row.provider_disclosed
            && row.model_route_disclosed
            && row.region_disclosed
            && row.retention_disclosed
            && row.cost_owner_disclosed)
        {
            violations.push(M5LaneCertificationViolation::ClaimedProfileMissingDisclosure);
        }
        if row.execution_mode != row.profile.expected_mode() {
            violations.push(M5LaneCertificationViolation::LocalityProfileMismatch);
        }
        if row.profile.is_managed_family() && !row.managed_claim_within_qualified_family {
            violations.push(M5LaneCertificationViolation::ManagedClaimWidened);
        }
        if authority_rank(row.automation_authority) < required_authority_rank(row.side_effect_class)
        {
            violations.push(M5LaneCertificationViolation::AutomationAuthorityInsufficient);
        }
    }
}

fn validate_downgrade_rules(
    cert: &LaneCertification,
    violations: &mut Vec<M5LaneCertificationViolation>,
) {
    if cert.downgrade_rules.is_empty() {
        violations.push(M5LaneCertificationViolation::DowngradeRulesMissing);
        return;
    }

    if !cert
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProofStale)
    {
        violations.push(M5LaneCertificationViolation::DowngradeRuleMissingProofStale);
    }

    let claimed_rank = qualification_rank(cert.claimed_qualification);
    for rule in &cert.downgrade_rules {
        if qualification_rank(rule.narrowed_to) >= claimed_rank {
            violations.push(M5LaneCertificationViolation::DowngradeRuleNotNarrowing);
            break;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5LaneCertificationPacket,
    violations: &mut Vec<M5LaneCertificationViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5LaneCertificationViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
