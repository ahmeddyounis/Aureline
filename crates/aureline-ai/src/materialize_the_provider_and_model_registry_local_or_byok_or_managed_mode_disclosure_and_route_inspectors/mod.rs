//! Materialized provider/model route disclosure with local, BYOK, or managed
//! mode labels and per-route inspectors.
//!
//! This module materializes the provider/model registry into one export-safe
//! disclosure packet whose unit of truth is a [`RouteInspectorRow`]: a single
//! claimed AI route that binds its provider, model, execution mode, locality,
//! region, retention, cost disclosure, tool side-effect class, and automation
//! authority into one inspectable record. Every claimed route declares exactly
//! one [`ExecutionModeClass`] — local, BYOK, or managed — and that label must
//! agree with the route's [`RouteLocalityClass`] so the headline mode badge can
//! never disagree with where bytes actually run.
//!
//! The packet is the canonical route-inspector source for shell, docs, support
//! export, and release tooling. It refuses to let a route hide cost, provider,
//! region, retention, or automation authority behind generic labels: a claimed
//! managed or BYOK route with an unverified region, retention, or cost posture
//! fails validation, a local route that is not fully on-device fails
//! validation, and a route whose tool side-effect mutates without human apply
//! authority fails validation. It reuses the qualification-class and
//! downgrade-trigger vocabularies frozen by the M5 AI workflow matrix lane and
//! references the provider/model registry schema by path so no inspector row
//! may stay greener than its evidence. Raw prompt bodies, raw diffs, raw
//! provider payloads, credentials, raw endpoint URLs, exact token counts, and
//! exact cost amounts stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ai/materialize-the-provider-and-model-registry-local-or-byok-or-managed-mode-disclosure-and-route-inspectors.schema.json`](../../../../schemas/ai/materialize-the-provider-and-model-registry-local-or-byok-or-managed-mode-disclosure-and-route-inspectors.schema.json).
//! The contract doc is
//! [`docs/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors.md`](../../../../docs/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors/`](../../../../fixtures/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass, M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`ProviderRouteDisclosurePacket`].
pub const PROVIDER_ROUTE_DISCLOSURE_RECORD_KIND: &str =
    "materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors";

/// Schema version for provider route disclosure records.
pub const PROVIDER_ROUTE_DISCLOSURE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PROVIDER_ROUTE_DISCLOSURE_SCHEMA_REF: &str =
    "schemas/ai/materialize-the-provider-and-model-registry-local-or-byok-or-managed-mode-disclosure-and-route-inspectors.schema.json";

/// Repo-relative path of the route disclosure contract doc.
pub const PROVIDER_ROUTE_DISCLOSURE_DOC_REF: &str =
    "docs/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors.md";

/// Repo-relative path of the provider/model registry schema this materializes.
pub const PROVIDER_MODEL_REGISTRY_SCHEMA_REF: &str =
    "schemas/ai/provider_model_registry.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const PROVIDER_ROUTE_DISCLOSURE_FIXTURE_DIR: &str =
    "fixtures/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors";

/// Repo-relative path of the checked support-export artifact.
pub const PROVIDER_ROUTE_DISCLOSURE_ARTIFACT_REF: &str =
    "artifacts/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const PROVIDER_ROUTE_DISCLOSURE_SUMMARY_REF: &str =
    "artifacts/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors.md";

/// The headline execution mode disclosed for one claimed route.
///
/// Every claimed route is exactly one of these three; the badge a user sees
/// derives from this class and must agree with the route's
/// [`RouteLocalityClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionModeClass {
    /// Inference and bytes stay on the local device.
    Local,
    /// A user-held key calls a vendor or self-hosted endpoint directly.
    Byok,
    /// Calls are brokered through a first-party or enterprise managed path.
    Managed,
}

impl ExecutionModeClass {
    /// Every disclosed mode, in declaration order.
    pub const ALL: [Self; 3] = [Self::Local, Self::Byok, Self::Managed];

    /// Stable token recorded in the disclosure.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Byok => "byok",
            Self::Managed => "managed",
        }
    }
}

/// Where a route's inference actually runs.
///
/// Each locality belongs to exactly one [`ExecutionModeClass`]; the disclosure
/// rejects any route whose declared mode disagrees with its locality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteLocalityClass {
    /// Runs inside the Aureline process on the local device.
    OnDeviceInProcess,
    /// Runs in a local companion service or sandboxed process.
    OnDeviceLocalService,
    /// A user key calls a vendor endpoint directly.
    ByokDirectVendor,
    /// A user key calls a self-hosted endpoint directly.
    ByokSelfHosted,
    /// Calls are brokered through an enterprise gateway.
    ManagedEnterpriseGateway,
    /// Calls land on a first-party managed hosted path.
    ManagedVendorHosted,
}

impl RouteLocalityClass {
    /// Stable token recorded in the disclosure.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnDeviceInProcess => "on_device_in_process",
            Self::OnDeviceLocalService => "on_device_local_service",
            Self::ByokDirectVendor => "byok_direct_vendor",
            Self::ByokSelfHosted => "byok_self_hosted",
            Self::ManagedEnterpriseGateway => "managed_enterprise_gateway",
            Self::ManagedVendorHosted => "managed_vendor_hosted",
        }
    }

    /// The execution mode this locality belongs to.
    pub const fn execution_mode(self) -> ExecutionModeClass {
        match self {
            Self::OnDeviceInProcess | Self::OnDeviceLocalService => ExecutionModeClass::Local,
            Self::ByokDirectVendor | Self::ByokSelfHosted => ExecutionModeClass::Byok,
            Self::ManagedEnterpriseGateway | Self::ManagedVendorHosted => {
                ExecutionModeClass::Managed
            }
        }
    }
}

/// Region posture disclosed for a route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteRegionClass {
    /// Bytes never leave the local device.
    OnDeviceOnly,
    /// Bytes are pinned to one named region.
    SingleRegionPinned,
    /// Bytes are pinned to a named region set.
    MultiRegionPinned,
    /// The provider chooses the region by default.
    VendorDefaultUnpinned,
    /// Region posture is unverified.
    UnknownUnverified,
    /// Region posture is policy-blocked.
    PolicyBlocked,
}

impl RouteRegionClass {
    /// Stable token recorded in the disclosure.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnDeviceOnly => "on_device_only",
            Self::SingleRegionPinned => "single_region_pinned",
            Self::MultiRegionPinned => "multi_region_pinned",
            Self::VendorDefaultUnpinned => "vendor_default_unpinned",
            Self::UnknownUnverified => "unknown_unverified",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// Whether the region posture is concretely disclosed.
    pub const fn is_disclosed(self) -> bool {
        !matches!(self, Self::UnknownUnverified)
    }

    /// Whether the posture is the fully-local on-device posture.
    pub const fn is_local_only(self) -> bool {
        matches!(self, Self::OnDeviceOnly)
    }
}

/// Retention posture disclosed for a route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteRetentionClass {
    /// Local-only route with no provider retention.
    NoRetentionLocalOnly,
    /// Provider promises request/response body discard.
    NoRetentionPromised,
    /// Provider retains bodies for bounded operator access only.
    BoundedRetentionOperatorOnly,
    /// Provider offers bounded retention with user export.
    BoundedRetentionWithExport,
    /// Provider retains without training use.
    UnboundedRetentionNotTrained,
    /// Provider may train on retained content.
    UnboundedRetentionUsedForTraining,
    /// Retention posture is unverified.
    UnknownUnverified,
    /// Retention posture is policy-blocked.
    PolicyBlocked,
}

impl RouteRetentionClass {
    /// Stable token recorded in the disclosure.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRetentionLocalOnly => "no_retention_local_only",
            Self::NoRetentionPromised => "no_retention_promised",
            Self::BoundedRetentionOperatorOnly => "bounded_retention_operator_only",
            Self::BoundedRetentionWithExport => "bounded_retention_with_export",
            Self::UnboundedRetentionNotTrained => "unbounded_retention_not_trained",
            Self::UnboundedRetentionUsedForTraining => "unbounded_retention_used_for_training",
            Self::UnknownUnverified => "unknown_unverified",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// Whether the retention posture is concretely disclosed.
    pub const fn is_disclosed(self) -> bool {
        !matches!(self, Self::UnknownUnverified)
    }

    /// Whether the posture is the fully-local no-retention posture.
    pub const fn is_local_only(self) -> bool {
        matches!(self, Self::NoRetentionLocalOnly)
    }
}

/// Cost disclosure posture for a route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteCostDisclosureClass {
    /// Local compute with no metered provider cost.
    NoCostLocalCompute,
    /// Metered per-token cost is disclosed to the user.
    MeteredPerTokenDisclosed,
    /// A flat-rate cost is disclosed to the user.
    FlatRateDisclosed,
    /// Cost is capped by a disclosed budget.
    BudgetCappedDisclosed,
    /// Cost posture is unverified.
    UnknownUnverified,
}

impl RouteCostDisclosureClass {
    /// Stable token recorded in the disclosure.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoCostLocalCompute => "no_cost_local_compute",
            Self::MeteredPerTokenDisclosed => "metered_per_token_disclosed",
            Self::FlatRateDisclosed => "flat_rate_disclosed",
            Self::BudgetCappedDisclosed => "budget_capped_disclosed",
            Self::UnknownUnverified => "unknown_unverified",
        }
    }

    /// Whether the cost posture is concretely disclosed.
    pub const fn is_disclosed(self) -> bool {
        !matches!(self, Self::UnknownUnverified)
    }

    /// Whether the posture is the fully-local no-cost posture.
    pub const fn is_local_free(self) -> bool {
        matches!(self, Self::NoCostLocalCompute)
    }
}

/// Side-effect class of the tools a route may invoke.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteToolSideEffectClass {
    /// The route reads or inspects only; no mutation.
    InspectOnly,
    /// The route may make locally reversible edits.
    LocalReversibleEdit,
    /// The route may make locally destructive edits.
    LocalDestructiveEdit,
    /// The route may post reversible external comments.
    ExternalReversibleComment,
    /// The route may publish irreversibly to an external surface.
    ExternalIrreversiblePublish,
    /// The route may mutate policy or trust state.
    PolicyOrTrustMutation,
}

impl RouteToolSideEffectClass {
    /// Stable token recorded in the disclosure.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOnly => "inspect_only",
            Self::LocalReversibleEdit => "local_reversible_edit",
            Self::LocalDestructiveEdit => "local_destructive_edit",
            Self::ExternalReversibleComment => "external_reversible_comment",
            Self::ExternalIrreversiblePublish => "external_irreversible_publish",
            Self::PolicyOrTrustMutation => "policy_or_trust_mutation",
        }
    }

    /// Whether the route can change state outside read-only inspection.
    pub const fn is_mutating(self) -> bool {
        !matches!(self, Self::InspectOnly)
    }
}

/// Automation authority a route is permitted to exercise.
///
/// There is no fully autonomous self-apply variant; every mutating authority
/// requires a human in the loop, so the disclosure fails closed by design.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteAutomationAuthorityClass {
    /// The route may read and propose but never apply.
    ReadOnlyNoApply,
    /// The route may preview only; a human applies.
    PreviewOnlyHumanApply,
    /// The route may apply a scoped change after human approval.
    ScopedApplyHumanApproved,
    /// A signed recipe may run after human approval.
    SignedRecipeHumanApproved,
    /// A background agent works on a side branch with human-gated merge.
    BackgroundAgentHumanGatedMerge,
}

impl RouteAutomationAuthorityClass {
    /// Stable token recorded in the disclosure.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyNoApply => "read_only_no_apply",
            Self::PreviewOnlyHumanApply => "preview_only_human_apply",
            Self::ScopedApplyHumanApproved => "scoped_apply_human_approved",
            Self::SignedRecipeHumanApproved => "signed_recipe_human_approved",
            Self::BackgroundAgentHumanGatedMerge => "background_agent_human_gated_merge",
        }
    }

    /// Whether this authority allows any mutating apply.
    pub const fn permits_mutation(self) -> bool {
        !matches!(self, Self::ReadOnlyNoApply)
    }
}

/// One downgrade rule that narrows a route's claim when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteDowngradeRule {
    /// Trigger that fires this rule.
    pub trigger: M5AiWorkflowDowngradeTrigger,
    /// Qualification the route narrows to when the trigger fires.
    pub narrowed_to: M5AiWorkflowQualificationClass,
    /// Whether tooling enforces this rule automatically.
    pub auto_enforced: bool,
    /// Review-safe rationale for the narrowing.
    pub rationale: String,
}

/// One materialized route inspector row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteInspectorRow {
    /// Stable route id.
    pub route_id: String,
    /// Provider identity token (no raw endpoint URL).
    pub provider_id: String,
    /// Model identity token.
    pub model_id: String,
    /// Qualification class claimed for this route.
    pub claimed_qualification: M5AiWorkflowQualificationClass,
    /// Headline execution mode badge.
    pub execution_mode: ExecutionModeClass,
    /// Where the route actually runs.
    pub locality: RouteLocalityClass,
    /// Region posture.
    pub region: RouteRegionClass,
    /// Retention posture.
    pub retention: RouteRetentionClass,
    /// Cost disclosure posture.
    pub cost_disclosure: RouteCostDisclosureClass,
    /// Tool side-effect class for the route.
    pub tool_side_effect: RouteToolSideEffectClass,
    /// Automation authority the route may exercise.
    pub automation_authority: RouteAutomationAuthorityClass,
    /// Review-safe mode disclosure label shown to users.
    pub mode_disclosure_label: String,
    /// Downgrade rules that narrow the claim.
    pub downgrade_rules: Vec<RouteDowngradeRule>,
    /// Required evidence packet refs backing the claim.
    pub evidence_packet_refs: Vec<String>,
}

impl RouteInspectorRow {
    /// Whether this route carries a publicly claimed qualification.
    ///
    /// Stable, Beta, and Preview are claimed lanes; Experimental, Held, and
    /// Unavailable are not.
    pub fn is_claimed(&self) -> bool {
        matches!(
            self.claimed_qualification,
            M5AiWorkflowQualificationClass::Stable
                | M5AiWorkflowQualificationClass::Beta
                | M5AiWorkflowQualificationClass::Preview
        )
    }

    /// Qualification this route narrows to when `trigger` fires.
    ///
    /// Returns the claimed qualification unchanged when no rule matches; this is
    /// the deterministic downgrade automation consumers and release tooling
    /// project instead of re-deriving narrowing locally.
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

    /// Renders a deterministic, review-safe inspector card for this route.
    pub fn render_inspector(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("### Route `{}`\n", self.route_id));
        out.push_str(&format!(
            "- Mode: `{}` ({})\n",
            self.execution_mode.as_str(),
            self.mode_disclosure_label
        ));
        out.push_str(&format!("- Provider: `{}`\n", self.provider_id));
        out.push_str(&format!("- Model: `{}`\n", self.model_id));
        out.push_str(&format!(
            "- Qualification: `{}`\n",
            self.claimed_qualification.as_str()
        ));
        out.push_str(&format!("- Locality: `{}`\n", self.locality.as_str()));
        out.push_str(&format!("- Region: `{}`\n", self.region.as_str()));
        out.push_str(&format!("- Retention: `{}`\n", self.retention.as_str()));
        out.push_str(&format!("- Cost: `{}`\n", self.cost_disclosure.as_str()));
        out.push_str(&format!(
            "- Tool side-effect: `{}`\n",
            self.tool_side_effect.as_str()
        ));
        out.push_str(&format!(
            "- Automation authority: `{}`\n",
            self.automation_authority.as_str()
        ));
        out
    }
}

/// Proof freshness block for the disclosure packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRouteDisclosureProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed routes.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ProviderRouteDisclosurePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderRouteDisclosurePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable disclosure label.
    pub disclosure_label: String,
    /// Materialized route inspector rows.
    pub routes: Vec<RouteInspectorRow>,
    /// Proof freshness block.
    pub proof_freshness: ProviderRouteDisclosureProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe provider route disclosure packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRouteDisclosurePacket {
    /// Record kind; must equal [`PROVIDER_ROUTE_DISCLOSURE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PROVIDER_ROUTE_DISCLOSURE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable disclosure label.
    pub disclosure_label: String,
    /// Materialized route inspector rows.
    pub routes: Vec<RouteInspectorRow>,
    /// Proof freshness block.
    pub proof_freshness: ProviderRouteDisclosureProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ProviderRouteDisclosurePacket {
    /// Builds a provider route disclosure packet from stable-lane input.
    pub fn new(input: ProviderRouteDisclosurePacketInput) -> Self {
        Self {
            record_kind: PROVIDER_ROUTE_DISCLOSURE_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_ROUTE_DISCLOSURE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            disclosure_label: input.disclosure_label,
            routes: input.routes,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the provider route disclosure invariants.
    pub fn validate(&self) -> Vec<ProviderRouteDisclosureViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PROVIDER_ROUTE_DISCLOSURE_RECORD_KIND {
            violations.push(ProviderRouteDisclosureViolation::WrongRecordKind);
        }
        if self.schema_version != PROVIDER_ROUTE_DISCLOSURE_SCHEMA_VERSION {
            violations.push(ProviderRouteDisclosureViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.disclosure_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ProviderRouteDisclosureViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_routes_present(self, &mut violations);
        for route in &self.routes {
            validate_route(route, &mut violations);
        }
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("provider route disclosure packet serializes"),
        ) {
            violations.push(ProviderRouteDisclosureViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Count of routes disclosed as local.
    pub fn local_route_count(&self) -> usize {
        self.route_count_for(ExecutionModeClass::Local)
    }

    /// Count of routes disclosed as BYOK.
    pub fn byok_route_count(&self) -> usize {
        self.route_count_for(ExecutionModeClass::Byok)
    }

    /// Count of routes disclosed as managed.
    pub fn managed_route_count(&self) -> usize {
        self.route_count_for(ExecutionModeClass::Managed)
    }

    fn route_count_for(&self, mode: ExecutionModeClass) -> usize {
        self.routes
            .iter()
            .filter(|route| route.execution_mode == mode)
            .count()
    }

    /// Returns the inspector row for `route_id`, if present.
    pub fn route(&self, route_id: &str) -> Option<&RouteInspectorRow> {
        self.routes.iter().find(|route| route.route_id == route_id)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("provider route disclosure packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Provider Route Disclosure And Inspectors\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.disclosure_label));
        out.push_str(&format!(
            "- Routes: {} ({} local, {} BYOK, {} managed)\n",
            self.routes.len(),
            self.local_route_count(),
            self.byok_route_count(),
            self.managed_route_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Route inspectors\n\n");
        for route in &self.routes {
            out.push_str(&route.render_inspector());
            out.push('\n');
        }
        out
    }
}

/// Errors emitted when reading the checked-in provider route disclosure export.
#[derive(Debug)]
pub enum ProviderRouteDisclosureArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ProviderRouteDisclosureViolation>),
}

impl fmt::Display for ProviderRouteDisclosureArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "provider route disclosure export parse failed: {error}"
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
                    "provider route disclosure export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ProviderRouteDisclosureArtifactError {}

/// Validation failures emitted by [`ProviderRouteDisclosurePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderRouteDisclosureViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The packet carries no routes.
    NoRoutes,
    /// A route id appears more than once.
    DuplicateRoute,
    /// A route row is missing a required identity or label field.
    RouteRowIncomplete,
    /// A route's declared mode disagrees with its locality.
    ModeLocalityMismatch,
    /// A local-mode route is not fully on-device for region, retention, or cost.
    LocalModeNotFullyLocal,
    /// A claimed route hides region, retention, or cost behind an unverified posture.
    UndisclosedTrustPosture,
    /// A route's tools mutate without any human apply authority.
    SideEffectWithoutApplyAuthority,
    /// A claimed route is missing required evidence packet refs.
    ClaimedRouteMissingEvidence,
    /// A route has no downgrade rules.
    DowngradeRulesMissing,
    /// A route's downgrade rules omit the proof-stale trigger.
    DowngradeRuleMissingProofStale,
    /// A downgrade rule does not narrow below the claimed qualification.
    DowngradeRuleNotNarrowing,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ProviderRouteDisclosureViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NoRoutes => "no_routes",
            Self::DuplicateRoute => "duplicate_route",
            Self::RouteRowIncomplete => "route_row_incomplete",
            Self::ModeLocalityMismatch => "mode_locality_mismatch",
            Self::LocalModeNotFullyLocal => "local_mode_not_fully_local",
            Self::UndisclosedTrustPosture => "undisclosed_trust_posture",
            Self::SideEffectWithoutApplyAuthority => "side_effect_without_apply_authority",
            Self::ClaimedRouteMissingEvidence => "claimed_route_missing_evidence",
            Self::DowngradeRulesMissing => "downgrade_rules_missing",
            Self::DowngradeRuleMissingProofStale => "downgrade_rule_missing_proof_stale",
            Self::DowngradeRuleNotNarrowing => "downgrade_rule_not_narrowing",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in provider route disclosure export.
pub fn current_provider_route_disclosure_export(
) -> Result<ProviderRouteDisclosurePacket, ProviderRouteDisclosureArtifactError> {
    let packet: ProviderRouteDisclosurePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors/support_export.json"
    )))
    .map_err(ProviderRouteDisclosureArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ProviderRouteDisclosureArtifactError::Validation(violations))
    }
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

fn validate_source_contracts(
    packet: &ProviderRouteDisclosurePacket,
    violations: &mut Vec<ProviderRouteDisclosureViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PROVIDER_ROUTE_DISCLOSURE_SCHEMA_REF,
        PROVIDER_ROUTE_DISCLOSURE_DOC_REF,
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF,
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ProviderRouteDisclosureViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_routes_present(
    packet: &ProviderRouteDisclosurePacket,
    violations: &mut Vec<ProviderRouteDisclosureViolation>,
) {
    if packet.routes.is_empty() {
        violations.push(ProviderRouteDisclosureViolation::NoRoutes);
        return;
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for route in &packet.routes {
        if !seen.insert(route.route_id.as_str()) {
            violations.push(ProviderRouteDisclosureViolation::DuplicateRoute);
        }
    }
}

fn validate_route(
    route: &RouteInspectorRow,
    violations: &mut Vec<ProviderRouteDisclosureViolation>,
) {
    if route.route_id.trim().is_empty()
        || route.provider_id.trim().is_empty()
        || route.model_id.trim().is_empty()
        || route.mode_disclosure_label.trim().is_empty()
    {
        violations.push(ProviderRouteDisclosureViolation::RouteRowIncomplete);
    }

    if route.locality.execution_mode() != route.execution_mode {
        violations.push(ProviderRouteDisclosureViolation::ModeLocalityMismatch);
    }

    if route.execution_mode == ExecutionModeClass::Local
        && (!route.region.is_local_only()
            || !route.retention.is_local_only()
            || !route.cost_disclosure.is_local_free())
    {
        violations.push(ProviderRouteDisclosureViolation::LocalModeNotFullyLocal);
    }

    if route.is_claimed()
        && (!route.region.is_disclosed()
            || !route.retention.is_disclosed()
            || !route.cost_disclosure.is_disclosed())
    {
        violations.push(ProviderRouteDisclosureViolation::UndisclosedTrustPosture);
    }

    if route.tool_side_effect.is_mutating() && !route.automation_authority.permits_mutation() {
        violations.push(ProviderRouteDisclosureViolation::SideEffectWithoutApplyAuthority);
    }

    if route.is_claimed() && route.evidence_packet_refs.is_empty() {
        violations.push(ProviderRouteDisclosureViolation::ClaimedRouteMissingEvidence);
    }

    validate_downgrade_rules(route, violations);
}

fn validate_downgrade_rules(
    route: &RouteInspectorRow,
    violations: &mut Vec<ProviderRouteDisclosureViolation>,
) {
    if route.downgrade_rules.is_empty() {
        violations.push(ProviderRouteDisclosureViolation::DowngradeRulesMissing);
        return;
    }

    if !route
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProofStale)
    {
        violations.push(ProviderRouteDisclosureViolation::DowngradeRuleMissingProofStale);
    }

    let claimed_rank = qualification_rank(route.claimed_qualification);
    for rule in &route.downgrade_rules {
        if qualification_rank(rule.narrowed_to) >= claimed_rank {
            violations.push(ProviderRouteDisclosureViolation::DowngradeRuleNotNarrowing);
            break;
        }
    }
}

fn validate_proof_freshness(
    packet: &ProviderRouteDisclosurePacket,
    violations: &mut Vec<ProviderRouteDisclosureViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(ProviderRouteDisclosureViolation::ProofFreshnessIncomplete);
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
