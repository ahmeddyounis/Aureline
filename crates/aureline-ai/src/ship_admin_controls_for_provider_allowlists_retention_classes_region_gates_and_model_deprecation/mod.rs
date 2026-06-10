//! Admin controls for provider allowlists, retention classes, region gates, and
//! model deprecation.
//!
//! This module carries operator-set governance over AI providers and models in
//! one export-safe truth packet whose unit of truth is an [`AdminControlRow`]: a
//! single admin control bound to the provider (and, for a model-deprecation
//! control, the model) it governs, the typed [`AdminControlDirective`] that says
//! what the control does, the scope and state it is enforced under, the admin
//! authority that set it, and how it narrows when proof goes stale or a provider
//! goes away. The packet is the canonical admin-control source for shell, docs,
//! support export, and release tooling; consumers project it instead of
//! re-deriving allowlist, retention, region, or deprecation posture by hand.
//!
//! Four control families share one row shape. A *provider allowlist* control
//! allows, conditionally allows, denies, or holds a provider under a named
//! execution mode. A *retention-class floor* control sets the minimum retention
//! posture a route may carry and optionally denies any route below it. A *region
//! gate* control names the region posture (and the concrete region tags) a route
//! may run in and optionally denies any route outside it. A *model deprecation*
//! control moves a named model through its lifecycle — generally available,
//! deprecation announced, sunset scheduled, blocked for new sessions, or retired
//! — and names the replacement and migration path so no user is stranded.
//!
//! The packet refuses to present a control greener than its posture can back, and
//! never hides cost, provider, region, retention, or automation authority behind
//! generic AI language. Every control names its provider explicitly; a retention
//! floor names a concretely disclosed retention class; a region gate names a
//! disclosed region posture and, when pinned, the concrete region tags; a model
//! deprecation that has begun names a migration path. A control that *denies* —
//! a denied provider, a retention floor that rejects routes below it, a region
//! gate that rejects routes outside it, or a deprecation that blocks or retires a
//! model — is governed like a release-bearing surface: it carries a real admin
//! gate, is audited, and is actually enforced (or explicitly monitor-only) rather
//! than sitting in a silent draft while claiming a public qualification.
//!
//! A control blocked by a higher-tier policy, or a provider allowlist still
//! pending review, narrows its claim instead of staying behind a Stable, Beta, or
//! Preview label. Every row carries a closed set of downgrade rules — including
//! the proof-stale and provider-unavailable triggers — that narrow the claim
//! instead of hiding the control, reusing the qualification, downgrade-trigger,
//! and rollback-posture vocabularies frozen by the M5 AI workflow matrix lane, the
//! execution-mode / region / retention vocabularies frozen by the provider/model
//! registry lane, and the approval vocabulary frozen by the tool-gateway baseline,
//! so no admin-control row may stay greener than its evidence.
//!
//! Provider endpoints, raw credential bodies, raw API keys, OAuth tokens, and raw
//! provider payloads stay outside the support boundary; the packet carries opaque
//! ids, classes, region tags, content addresses, and review-safe labels only.
//!
//! The boundary schema is
//! [`schemas/ai/ship-admin-controls-for-provider-allowlists-retention-classes-region-gates-and-model-deprecation.schema.json`](../../../../schemas/ai/ship-admin-controls-for-provider-allowlists-retention-classes-region-gates-and-model-deprecation.schema.json).
//! The contract doc is
//! [`docs/automation/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation.md`](../../../../docs/automation/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation/`](../../../../fixtures/ai/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass, M5AiWorkflowRollbackPosture,
    M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};
use crate::materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors::{
    ExecutionModeClass, RouteRegionClass, RouteRetentionClass, PROVIDER_MODEL_REGISTRY_SCHEMA_REF,
    PROVIDER_ROUTE_DISCLOSURE_SCHEMA_REF,
};
use crate::tool_gateway::{ToolApprovalPostureClass, TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF};

/// Stable record-kind tag carried by [`AdminControlPacket`].
pub const ADMIN_CONTROLS_RECORD_KIND: &str =
    "ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation";

/// Schema version for admin-control records.
pub const ADMIN_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const ADMIN_CONTROLS_SCHEMA_REF: &str =
    "schemas/ai/ship-admin-controls-for-provider-allowlists-retention-classes-region-gates-and-model-deprecation.schema.json";

/// Repo-relative path of the admin-control contract doc.
pub const ADMIN_CONTROLS_DOC_REF: &str =
    "docs/automation/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation.md";

/// Repo-relative path of the protected fixture directory.
pub const ADMIN_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ai/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation";

/// Repo-relative path of the checked support-export artifact.
pub const ADMIN_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/ai/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const ADMIN_CONTROLS_SUMMARY_REF: &str =
    "artifacts/ai/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation.md";

/// Control family an [`AdminControlRow`] belongs to.
///
/// The family is derived from the row's [`AdminControlDirective`]; it is the
/// top-level axis support and release surfaces filter on without descending into
/// the typed directive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminControlFamilyClass {
    /// Allows, conditionally allows, denies, or holds a provider.
    ProviderAllowlist,
    /// Sets the minimum retention posture a route may carry.
    RetentionClassFloor,
    /// Names the region posture a route may run in.
    RegionGate,
    /// Moves a named model through its deprecation lifecycle.
    ModelDeprecation,
}

impl AdminControlFamilyClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAllowlist => "provider_allowlist",
            Self::RetentionClassFloor => "retention_class_floor",
            Self::RegionGate => "region_gate",
            Self::ModelDeprecation => "model_deprecation",
        }
    }

    /// Whether this family governs a single named model rather than a provider
    /// as a whole.
    pub const fn requires_model_target(self) -> bool {
        matches!(self, Self::ModelDeprecation)
    }
}

/// Decision an admin recorded for a provider allowlist control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminAllowlistDecisionClass {
    /// The provider is allowed under the governed execution mode.
    ProviderAllowed,
    /// The provider is allowed only under named conditions.
    ProviderAllowedWithConditions,
    /// The provider is denied by admin policy.
    ProviderDeniedByPolicy,
    /// The provider is recorded but awaiting admin review.
    ProviderPendingReview,
}

impl AdminAllowlistDecisionClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAllowed => "provider_allowed",
            Self::ProviderAllowedWithConditions => "provider_allowed_with_conditions",
            Self::ProviderDeniedByPolicy => "provider_denied_by_policy",
            Self::ProviderPendingReview => "provider_pending_review",
        }
    }

    /// Whether the provider may be dispatched to under this decision.
    pub const fn is_allowed(self) -> bool {
        matches!(
            self,
            Self::ProviderAllowed | Self::ProviderAllowedWithConditions
        )
    }

    /// Whether the decision denies the provider.
    pub const fn is_denied(self) -> bool {
        matches!(self, Self::ProviderDeniedByPolicy)
    }

    /// Whether the decision is still pending admin review.
    pub const fn is_pending(self) -> bool {
        matches!(self, Self::ProviderPendingReview)
    }
}

/// Lifecycle stage of a model under a deprecation control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelLifecycleStageClass {
    /// The model is generally available.
    GenerallyAvailable,
    /// Deprecation is announced; the model still serves.
    DeprecationAnnounced,
    /// Deprecation with a scheduled sunset date.
    DeprecatedSunsetScheduled,
    /// New sessions are blocked; existing sessions may drain.
    BlockedNewSessions,
    /// The model is retired and removed.
    RetiredRemoved,
}

impl ModelLifecycleStageClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GenerallyAvailable => "generally_available",
            Self::DeprecationAnnounced => "deprecation_announced",
            Self::DeprecatedSunsetScheduled => "deprecated_sunset_scheduled",
            Self::BlockedNewSessions => "blocked_new_sessions",
            Self::RetiredRemoved => "retired_removed",
        }
    }

    /// Whether the model has entered any deprecation stage.
    pub const fn is_deprecating(self) -> bool {
        !matches!(self, Self::GenerallyAvailable)
    }

    /// Whether the stage blocks new use of the model.
    pub const fn blocks_new_use(self) -> bool {
        matches!(self, Self::BlockedNewSessions | Self::RetiredRemoved)
    }

    /// Whether the stage requires a concrete sunset date.
    pub const fn requires_sunset_date(self) -> bool {
        matches!(self, Self::DeprecatedSunsetScheduled)
    }

    /// Whether the stage requires a migration path so users are not stranded.
    pub const fn requires_migration_path(self) -> bool {
        self.is_deprecating()
    }
}

/// Provider-allowlist directive payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderAllowlistDirective {
    /// Admin decision for the provider under the governed execution mode.
    pub decision: AdminAllowlistDecisionClass,
    /// Review-safe label describing the decision and any conditions.
    pub decision_label: String,
}

/// Retention-class-floor directive payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionFloorDirective {
    /// Minimum retention posture a governed route may carry.
    pub required_floor: RouteRetentionClass,
    /// Whether the control denies any route below the floor.
    pub denies_below_floor: bool,
    /// Review-safe label describing the floor.
    pub floor_label: String,
}

/// Region-gate directive payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionGateDirective {
    /// Region posture a governed route may run in.
    pub allowed_region_posture: RouteRegionClass,
    /// Concrete, review-safe region tags the gate pins to (e.g. `eu`, `us-gov`);
    /// non-empty exactly when the posture is region-pinned.
    #[serde(default)]
    pub allowed_region_tags: Vec<String>,
    /// Whether the control denies any route outside the gate.
    pub denies_outside_gate: bool,
    /// Review-safe label describing the gate.
    pub gate_label: String,
}

impl RegionGateDirective {
    /// Whether the gate's posture pins to one or more named regions.
    pub const fn is_region_pinned(&self) -> bool {
        matches!(
            self.allowed_region_posture,
            RouteRegionClass::SingleRegionPinned | RouteRegionClass::MultiRegionPinned
        )
    }
}

/// Model-deprecation directive payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelDeprecationDirective {
    /// Lifecycle stage of the governed model.
    pub lifecycle_stage: ModelLifecycleStageClass,
    /// RFC 3339 sunset date; non-empty when the stage requires one.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sunset_date: String,
    /// Opaque ref to the replacement model, when one is named.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub replacement_model_ref: String,
    /// Repo-relative migration runbook or doc ref; non-empty once deprecation has
    /// begun so users are not stranded.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub migration_path_ref: String,
    /// Review-safe label describing the lifecycle stage.
    pub stage_label: String,
}

/// Typed directive that says what an [`AdminControlRow`] does.
///
/// The directive's `kind` tag is the control's family. Each variant carries only
/// the fields its family needs; consumers match the directive to project the
/// exact posture rather than re-deriving it from free text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AdminControlDirective {
    /// Allows, conditionally allows, denies, or holds a provider.
    ProviderAllowlist(ProviderAllowlistDirective),
    /// Sets the minimum retention posture a route may carry.
    RetentionFloor(RetentionFloorDirective),
    /// Names the region posture a route may run in.
    RegionGate(RegionGateDirective),
    /// Moves a named model through its deprecation lifecycle.
    ModelDeprecation(ModelDeprecationDirective),
}

impl AdminControlDirective {
    /// The control family this directive belongs to.
    pub const fn family(&self) -> AdminControlFamilyClass {
        match self {
            Self::ProviderAllowlist(_) => AdminControlFamilyClass::ProviderAllowlist,
            Self::RetentionFloor(_) => AdminControlFamilyClass::RetentionClassFloor,
            Self::RegionGate(_) => AdminControlFamilyClass::RegionGate,
            Self::ModelDeprecation(_) => AdminControlFamilyClass::ModelDeprecation,
        }
    }

    /// Whether this directive denies, blocks, or retires what it governs.
    ///
    /// A denial control is held to the release-bearing bar: it must carry a real
    /// admin gate, be audited, and actually enforce.
    pub fn is_denial(&self) -> bool {
        match self {
            Self::ProviderAllowlist(directive) => directive.decision.is_denied(),
            Self::RetentionFloor(directive) => directive.denies_below_floor,
            Self::RegionGate(directive) => directive.denies_outside_gate,
            Self::ModelDeprecation(directive) => directive.lifecycle_stage.blocks_new_use(),
        }
    }

    /// Whether this directive is still awaiting admin review.
    pub fn is_pending(&self) -> bool {
        match self {
            Self::ProviderAllowlist(directive) => directive.decision.is_pending(),
            _ => false,
        }
    }

    /// Deterministic, review-safe one-line summary of the directive.
    pub fn summary_line(&self) -> String {
        match self {
            Self::ProviderAllowlist(directive) => {
                format!("allowlist `{}`", directive.decision.as_str())
            }
            Self::RetentionFloor(directive) => format!(
                "retention floor `{}` (denies below: {})",
                directive.required_floor.as_str(),
                directive.denies_below_floor
            ),
            Self::RegionGate(directive) => format!(
                "region gate `{}` (denies outside: {})",
                directive.allowed_region_posture.as_str(),
                directive.denies_outside_gate
            ),
            Self::ModelDeprecation(directive) => {
                format!("model lifecycle `{}`", directive.lifecycle_stage.as_str())
            }
        }
    }
}

/// Scope an admin control is enforced against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminEnforcementScopeClass {
    /// Enforced across the whole organisation.
    OrganisationWide,
    /// Enforced for one tenant.
    TenantScoped,
    /// Enforced for one workspace.
    WorkspaceScoped,
    /// Enforced for one named deployment profile.
    DeploymentProfileScoped,
}

impl AdminEnforcementScopeClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrganisationWide => "organisation_wide",
            Self::TenantScoped => "tenant_scoped",
            Self::WorkspaceScoped => "workspace_scoped",
            Self::DeploymentProfileScoped => "deployment_profile_scoped",
        }
    }
}

/// Effective enforcement state of an admin control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminControlStateClass {
    /// Actively enforced.
    EnforcedActive,
    /// Active but report-only; it observes without blocking.
    EnforcedMonitorOnly,
    /// Scheduled to activate but not yet enforcing.
    StagedPendingActivation,
    /// Was enforced, then rolled back.
    RolledBack,
    /// Replaced by a newer control.
    SupersededByNewerControl,
    /// Authored but not yet enforced.
    DraftNotEnforced,
    /// Overridden by a higher-tier policy.
    BlockedByHigherPolicy,
}

impl AdminControlStateClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnforcedActive => "enforced_active",
            Self::EnforcedMonitorOnly => "enforced_monitor_only",
            Self::StagedPendingActivation => "staged_pending_activation",
            Self::RolledBack => "rolled_back",
            Self::SupersededByNewerControl => "superseded_by_newer_control",
            Self::DraftNotEnforced => "draft_not_enforced",
            Self::BlockedByHigherPolicy => "blocked_by_higher_policy",
        }
    }

    /// Whether the control is live — actively enforcing or actively monitoring.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::EnforcedActive | Self::EnforcedMonitorOnly)
    }

    /// Whether the control actively blocks dispatch.
    pub const fn is_enforcing(self) -> bool {
        matches!(self, Self::EnforcedActive)
    }

    /// Whether the control is overridden by a higher-tier policy.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::BlockedByHigherPolicy)
    }
}

/// One downgrade rule that narrows a control's claim when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminControlDowngradeRule {
    /// Trigger that fires this rule.
    pub trigger: M5AiWorkflowDowngradeTrigger,
    /// Qualification the control narrows to when the trigger fires.
    pub narrowed_to: M5AiWorkflowQualificationClass,
    /// Whether tooling enforces this rule automatically.
    pub auto_enforced: bool,
    /// Review-safe rationale for the narrowing.
    pub rationale: String,
}

/// One admin control governing a provider (and, for deprecation, a model).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminControlRow {
    /// Stable control id.
    pub control_id: String,
    /// Human-readable control label.
    pub control_label: String,
    /// Control family label.
    pub control_family_label: String,
    /// Opaque id of the provider this control governs.
    pub target_provider_id: String,
    /// Human-readable provider label.
    pub target_provider_label: String,
    /// Opaque id of the model this control governs; required for a model
    /// deprecation control and empty otherwise.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub target_model_id: String,
    /// Execution mode the control governs the provider under.
    pub governed_execution_mode: ExecutionModeClass,
    /// Typed directive that says what the control does.
    pub directive: AdminControlDirective,
    /// Scope the control is enforced against.
    pub enforcement_scope: AdminEnforcementScopeClass,
    /// Effective enforcement state.
    pub enforcement_state: AdminControlStateClass,
    /// Admin authority gate required to set or change the control.
    pub admin_authority: ToolApprovalPostureClass,
    /// Opaque ref to the admin identity that set the control.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub admin_identity_ref: String,
    /// Whether changes to the control are durably audited.
    pub audited: bool,
    /// Qualification class claimed for this control.
    pub claimed_qualification: M5AiWorkflowQualificationClass,
    /// Downgrade rules that narrow the claim.
    pub downgrade_rules: Vec<AdminControlDowngradeRule>,
    /// Rollback posture for a control change.
    pub rollback_posture: M5AiWorkflowRollbackPosture,
    /// True when the rollback path has been drilled and verified.
    pub rollback_verified: bool,
    /// Required evidence packet refs backing the claim.
    pub evidence_packet_refs: Vec<String>,
    /// Review-safe explanation of the control posture.
    pub explanation_label: String,
}

impl AdminControlRow {
    /// The control family, derived from the directive.
    pub const fn family(&self) -> AdminControlFamilyClass {
        self.directive.family()
    }

    /// Whether this control carries a publicly claimed qualification.
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

    /// Whether the control denies, blocks, or retires what it governs.
    pub fn is_denial(&self) -> bool {
        self.directive.is_denial()
    }

    /// Whether the control carries a real admin gate.
    pub fn has_admin_gate(&self) -> bool {
        self.admin_authority.requires_approval_gate() || self.admin_authority.denies_dispatch()
    }

    /// Qualification this control narrows to when `trigger` fires.
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

    /// Renders a deterministic, review-safe inspector card for this control.
    pub fn render_inspector(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("### Control `{}`\n", self.control_id));
        out.push_str(&format!("- Label: `{}`\n", self.control_label));
        out.push_str(&format!("- Family: `{}`\n", self.family().as_str()));
        out.push_str(&format!(
            "- Qualification: `{}`\n",
            self.claimed_qualification.as_str()
        ));
        let model = if self.target_model_id.is_empty() {
            String::from("(provider-wide)")
        } else {
            format!("model `{}`", self.target_model_id)
        };
        out.push_str(&format!(
            "- Target: provider `{}` / {} / mode `{}`\n",
            self.target_provider_id,
            model,
            self.governed_execution_mode.as_str()
        ));
        out.push_str(&format!("- Directive: {}\n", self.directive.summary_line()));
        out.push_str(&format!(
            "- Enforcement: `{}` / scope `{}` / authority `{}`\n",
            self.enforcement_state.as_str(),
            self.enforcement_scope.as_str(),
            self.admin_authority.as_str()
        ));
        out.push_str(&format!(
            "- Rollback: `{}` (verified: {})\n",
            self.rollback_posture.as_str(),
            self.rollback_verified
        ));
        out
    }
}

/// Proof freshness block for the admin-control packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminControlsProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed controls.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`AdminControlPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminControlPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Admin-control rows.
    pub controls: Vec<AdminControlRow>,
    /// Proof freshness block.
    pub proof_freshness: AdminControlsProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe admin-control packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminControlPacket {
    /// Record kind; must equal [`ADMIN_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`ADMIN_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Admin-control rows.
    pub controls: Vec<AdminControlRow>,
    /// Proof freshness block.
    pub proof_freshness: AdminControlsProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AdminControlPacket {
    /// Builds an admin-control packet from stable-lane input.
    pub fn new(input: AdminControlPacketInput) -> Self {
        Self {
            record_kind: ADMIN_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: ADMIN_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            catalogue_label: input.catalogue_label,
            controls: input.controls,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the admin-control invariants.
    pub fn validate(&self) -> Vec<AdminControlViolation> {
        let mut violations = Vec::new();

        if self.record_kind != ADMIN_CONTROLS_RECORD_KIND {
            violations.push(AdminControlViolation::WrongRecordKind);
        }
        if self.schema_version != ADMIN_CONTROLS_SCHEMA_VERSION {
            violations.push(AdminControlViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.catalogue_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AdminControlViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_controls_present(self, &mut violations);
        for control in &self.controls {
            validate_control(control, &mut violations);
        }
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_admin_material(
            &serde_json::to_value(self).expect("admin control packet serializes"),
        ) {
            violations.push(AdminControlViolation::RawProviderMaterialInExport);
        }

        violations
    }

    /// Count of controls carrying a publicly claimed qualification.
    pub fn claimed_control_count(&self) -> usize {
        self.controls.iter().filter(|c| c.is_claimed()).count()
    }

    /// Count of controls that deny, block, or retire what they govern.
    pub fn denial_control_count(&self) -> usize {
        self.controls.iter().filter(|c| c.is_denial()).count()
    }

    /// Count of controls in the given family.
    pub fn family_count(&self, family: AdminControlFamilyClass) -> usize {
        self.controls
            .iter()
            .filter(|c| c.family() == family)
            .count()
    }

    /// Returns the control row for `control_id`, if present.
    pub fn control(&self, control_id: &str) -> Option<&AdminControlRow> {
        self.controls.iter().find(|c| c.control_id == control_id)
    }

    /// Whether dispatch to `provider_id` is denied by a live admin allowlist
    /// control under any governed mode.
    ///
    /// This is the consuming projection routing and shell surfaces read instead
    /// of re-deriving admin allowlist posture; it considers only controls that
    /// are actively enforcing.
    pub fn is_provider_admin_blocked(&self, provider_id: &str) -> bool {
        self.controls.iter().any(|control| {
            control.target_provider_id == provider_id
                && control.enforcement_state.is_enforcing()
                && matches!(
                    &control.directive,
                    AdminControlDirective::ProviderAllowlist(directive)
                        if directive.decision.is_denied()
                )
        })
    }

    /// Live controls that actively enforce or monitor.
    pub fn live_controls(&self) -> impl Iterator<Item = &AdminControlRow> {
        self.controls
            .iter()
            .filter(|c| c.enforcement_state.is_live())
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("admin control packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# Admin Controls: Provider Allowlists, Retention, Region Gates, Model Deprecation\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalogue_label));
        out.push_str(&format!(
            "- Controls: {} ({} claimed, {} denial)\n",
            self.controls.len(),
            self.claimed_control_count(),
            self.denial_control_count()
        ));
        out.push_str(&format!(
            "- Families: {} allowlist, {} retention floor, {} region gate, {} model deprecation\n",
            self.family_count(AdminControlFamilyClass::ProviderAllowlist),
            self.family_count(AdminControlFamilyClass::RetentionClassFloor),
            self.family_count(AdminControlFamilyClass::RegionGate),
            self.family_count(AdminControlFamilyClass::ModelDeprecation)
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Control inspectors\n\n");
        for control in &self.controls {
            out.push_str(&control.render_inspector());
            out.push('\n');
        }
        out
    }
}

/// Errors emitted when reading the checked-in admin-control export.
#[derive(Debug)]
pub enum AdminControlArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AdminControlViolation>),
}

impl fmt::Display for AdminControlArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "admin control export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "admin control export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AdminControlArtifactError {}

/// Validation failures emitted by [`AdminControlPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdminControlViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The packet carries no controls.
    NoControls,
    /// A control id appears more than once.
    DuplicateControl,
    /// A control row is missing a required identity or label field.
    ControlRowIncomplete,
    /// A control is missing its governed provider id or label.
    ControlMissingProviderTarget,
    /// A model-deprecation control is missing its governed model id.
    ModelDeprecationMissingModelTarget,
    /// A non-deprecation control carries a model id it does not govern.
    NonDeprecationCarriesModelTarget,
    /// A directive payload is missing its review-safe label.
    DirectiveLabelMissing,
    /// A retention floor names an unverified retention posture.
    RetentionFloorNotDisclosed,
    /// A region gate names an unverified region posture.
    RegionGateNotDisclosed,
    /// A region-pinned gate does not name any concrete region tag.
    RegionGatePinnedWithoutTags,
    /// An unpinned region gate names concrete region tags.
    RegionGateUnpinnedWithTags,
    /// A scheduled-sunset deprecation does not name a sunset date.
    DeprecationMissingSunsetDate,
    /// A control not scheduling a sunset still carries a sunset date.
    DeprecationUnexpectedSunsetDate,
    /// A deprecation that has begun does not name a migration path.
    DeprecationMissingMigrationPath,
    /// A denial control carries no real admin gate.
    DenialControlWithoutAdminGate,
    /// A denial control is not audited.
    DenialControlNotAudited,
    /// A claimed denial control is not actually enforced or monitored.
    ClaimedDenialNotEnforced,
    /// A control blocked by higher policy still claims a public qualification.
    BlockedControlClaimsQualification,
    /// A provider allowlist still pending review claims Stable.
    PendingAllowlistClaimsStable,
    /// A claimed control is missing required evidence packet refs.
    ClaimedControlMissingEvidence,
    /// A claimed control's reversible rollback path is not verified.
    ClaimedRollbackUnverified,
    /// A control has no downgrade rules.
    DowngradeRulesMissing,
    /// A control's downgrade rules omit the proof-stale trigger.
    DowngradeRuleMissingProofStale,
    /// A control's downgrade rules omit the provider-unavailable trigger.
    DowngradeRuleMissingProviderUnavailable,
    /// A downgrade rule does not narrow below the claimed qualification.
    DowngradeRuleNotNarrowing,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw provider material.
    RawProviderMaterialInExport,
}

impl AdminControlViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NoControls => "no_controls",
            Self::DuplicateControl => "duplicate_control",
            Self::ControlRowIncomplete => "control_row_incomplete",
            Self::ControlMissingProviderTarget => "control_missing_provider_target",
            Self::ModelDeprecationMissingModelTarget => "model_deprecation_missing_model_target",
            Self::NonDeprecationCarriesModelTarget => "non_deprecation_carries_model_target",
            Self::DirectiveLabelMissing => "directive_label_missing",
            Self::RetentionFloorNotDisclosed => "retention_floor_not_disclosed",
            Self::RegionGateNotDisclosed => "region_gate_not_disclosed",
            Self::RegionGatePinnedWithoutTags => "region_gate_pinned_without_tags",
            Self::RegionGateUnpinnedWithTags => "region_gate_unpinned_with_tags",
            Self::DeprecationMissingSunsetDate => "deprecation_missing_sunset_date",
            Self::DeprecationUnexpectedSunsetDate => "deprecation_unexpected_sunset_date",
            Self::DeprecationMissingMigrationPath => "deprecation_missing_migration_path",
            Self::DenialControlWithoutAdminGate => "denial_control_without_admin_gate",
            Self::DenialControlNotAudited => "denial_control_not_audited",
            Self::ClaimedDenialNotEnforced => "claimed_denial_not_enforced",
            Self::BlockedControlClaimsQualification => "blocked_control_claims_qualification",
            Self::PendingAllowlistClaimsStable => "pending_allowlist_claims_stable",
            Self::ClaimedControlMissingEvidence => "claimed_control_missing_evidence",
            Self::ClaimedRollbackUnverified => "claimed_rollback_unverified",
            Self::DowngradeRulesMissing => "downgrade_rules_missing",
            Self::DowngradeRuleMissingProofStale => "downgrade_rule_missing_proof_stale",
            Self::DowngradeRuleMissingProviderUnavailable => {
                "downgrade_rule_missing_provider_unavailable"
            }
            Self::DowngradeRuleNotNarrowing => "downgrade_rule_not_narrowing",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawProviderMaterialInExport => "raw_provider_material_in_export",
        }
    }
}

/// Reads and validates the checked-in admin-control export.
pub fn current_admin_controls_export() -> Result<AdminControlPacket, AdminControlArtifactError> {
    let packet: AdminControlPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation/support_export.json"
    )))
    .map_err(AdminControlArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AdminControlArtifactError::Validation(violations))
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
    packet: &AdminControlPacket,
    violations: &mut Vec<AdminControlViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        ADMIN_CONTROLS_SCHEMA_REF,
        ADMIN_CONTROLS_DOC_REF,
        PROVIDER_ROUTE_DISCLOSURE_SCHEMA_REF,
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF,
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
        TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(AdminControlViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_present(
    packet: &AdminControlPacket,
    violations: &mut Vec<AdminControlViolation>,
) {
    if packet.controls.is_empty() {
        violations.push(AdminControlViolation::NoControls);
        return;
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for control in &packet.controls {
        if !seen.insert(control.control_id.as_str()) {
            violations.push(AdminControlViolation::DuplicateControl);
        }
    }
}

fn validate_control(control: &AdminControlRow, violations: &mut Vec<AdminControlViolation>) {
    if control.control_id.trim().is_empty()
        || control.control_label.trim().is_empty()
        || control.control_family_label.trim().is_empty()
        || control.explanation_label.trim().is_empty()
    {
        violations.push(AdminControlViolation::ControlRowIncomplete);
    }

    if control.target_provider_id.trim().is_empty()
        || control.target_provider_label.trim().is_empty()
    {
        violations.push(AdminControlViolation::ControlMissingProviderTarget);
    }

    // A model-deprecation control names its model; the other families govern a
    // provider as a whole and carry no model id.
    if control.family().requires_model_target() {
        if control.target_model_id.trim().is_empty() {
            violations.push(AdminControlViolation::ModelDeprecationMissingModelTarget);
        }
    } else if !control.target_model_id.trim().is_empty() {
        violations.push(AdminControlViolation::NonDeprecationCarriesModelTarget);
    }

    validate_directive(control, violations);
    validate_denial_governance(control, violations);
    validate_claim_state(control, violations);
    validate_downgrade_rules(control, violations);
}

fn validate_directive(control: &AdminControlRow, violations: &mut Vec<AdminControlViolation>) {
    match &control.directive {
        AdminControlDirective::ProviderAllowlist(directive) => {
            if directive.decision_label.trim().is_empty() {
                violations.push(AdminControlViolation::DirectiveLabelMissing);
            }
        }
        AdminControlDirective::RetentionFloor(directive) => {
            if directive.floor_label.trim().is_empty() {
                violations.push(AdminControlViolation::DirectiveLabelMissing);
            }
            // A retention floor must name a concretely disclosed posture — an
            // unverified or policy-blocked posture is not a floor a route can be
            // held to.
            if !directive.required_floor.is_disclosed()
                || directive.required_floor == RouteRetentionClass::PolicyBlocked
            {
                violations.push(AdminControlViolation::RetentionFloorNotDisclosed);
            }
        }
        AdminControlDirective::RegionGate(directive) => {
            if directive.gate_label.trim().is_empty() {
                violations.push(AdminControlViolation::DirectiveLabelMissing);
            }
            if !directive.allowed_region_posture.is_disclosed()
                || directive.allowed_region_posture == RouteRegionClass::PolicyBlocked
            {
                violations.push(AdminControlViolation::RegionGateNotDisclosed);
            }
            // A pinned gate names its regions; an unpinned posture names none.
            if directive.is_region_pinned() && directive.allowed_region_tags.is_empty() {
                violations.push(AdminControlViolation::RegionGatePinnedWithoutTags);
            }
            if !directive.is_region_pinned() && !directive.allowed_region_tags.is_empty() {
                violations.push(AdminControlViolation::RegionGateUnpinnedWithTags);
            }
        }
        AdminControlDirective::ModelDeprecation(directive) => {
            if directive.stage_label.trim().is_empty() {
                violations.push(AdminControlViolation::DirectiveLabelMissing);
            }
            let has_sunset = !directive.sunset_date.trim().is_empty();
            if directive.lifecycle_stage.requires_sunset_date() && !has_sunset {
                violations.push(AdminControlViolation::DeprecationMissingSunsetDate);
            }
            if !directive.lifecycle_stage.requires_sunset_date() && has_sunset {
                violations.push(AdminControlViolation::DeprecationUnexpectedSunsetDate);
            }
            // A deprecation that has begun names a migration path so no user is
            // stranded — a replacement model or a migration runbook.
            if directive.lifecycle_stage.requires_migration_path()
                && directive.migration_path_ref.trim().is_empty()
                && directive.replacement_model_ref.trim().is_empty()
            {
                violations.push(AdminControlViolation::DeprecationMissingMigrationPath);
            }
        }
    }
}

fn validate_denial_governance(
    control: &AdminControlRow,
    violations: &mut Vec<AdminControlViolation>,
) {
    if !control.is_denial() {
        return;
    }

    // A denial control is governed like a release-bearing surface: it carries a
    // real admin gate and is audited.
    if !control.has_admin_gate() {
        violations.push(AdminControlViolation::DenialControlWithoutAdminGate);
    }
    if !control.audited {
        violations.push(AdminControlViolation::DenialControlNotAudited);
    }

    // A claimed denial control may not sit in a silent draft or staged state
    // while presenting a public qualification — it must actually be live.
    if control.is_claimed() && !control.enforcement_state.is_live() {
        violations.push(AdminControlViolation::ClaimedDenialNotEnforced);
    }
}

fn validate_claim_state(control: &AdminControlRow, violations: &mut Vec<AdminControlViolation>) {
    // A control overridden by a higher-tier policy narrows its claim instead of
    // keeping a public qualification.
    if control.is_claimed() && control.enforcement_state.is_blocked() {
        violations.push(AdminControlViolation::BlockedControlClaimsQualification);
    }

    // A provider allowlist still pending admin review may not claim Stable.
    if control.directive.is_pending()
        && control.claimed_qualification == M5AiWorkflowQualificationClass::Stable
    {
        violations.push(AdminControlViolation::PendingAllowlistClaimsStable);
    }

    if control.is_claimed() && control.evidence_packet_refs.is_empty() {
        violations.push(AdminControlViolation::ClaimedControlMissingEvidence);
    }

    // A claimed control whose change can be reversed must have drilled that
    // reversal; a non-applicable posture carries no reversal.
    if control.is_claimed()
        && control.rollback_posture != M5AiWorkflowRollbackPosture::NotApplicable
        && !control.rollback_verified
    {
        violations.push(AdminControlViolation::ClaimedRollbackUnverified);
    }
}

fn validate_downgrade_rules(
    control: &AdminControlRow,
    violations: &mut Vec<AdminControlViolation>,
) {
    if control.downgrade_rules.is_empty() {
        violations.push(AdminControlViolation::DowngradeRulesMissing);
        return;
    }

    if !control
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProofStale)
    {
        violations.push(AdminControlViolation::DowngradeRuleMissingProofStale);
    }

    // Provider outages and quota exhaustion narrow through the
    // provider-unavailable trigger, so every row must carry it.
    if !control
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProviderUnavailable)
    {
        violations.push(AdminControlViolation::DowngradeRuleMissingProviderUnavailable);
    }

    let claimed_rank = qualification_rank(control.claimed_qualification);
    for rule in &control.downgrade_rules {
        if qualification_rank(rule.narrowed_to) >= claimed_rank {
            violations.push(AdminControlViolation::DowngradeRuleNotNarrowing);
            break;
        }
    }
}

fn validate_proof_freshness(
    packet: &AdminControlPacket,
    violations: &mut Vec<AdminControlViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(AdminControlViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// Admin controls are declarative: the support boundary carries opaque ids,
/// classes, region tags, content addresses, and review-safe labels only, never
/// raw provider endpoints, credential bodies, raw API keys, OAuth tokens, or raw
/// provider payloads.
fn json_contains_forbidden_admin_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_admin_material(text),
        serde_json::Value::Array(values) => {
            values.iter().any(json_contains_forbidden_admin_material)
        }
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_admin_material),
        _ => false,
    }
}

fn contains_forbidden_admin_material(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("api_key=")
        || lower.contains("api-key=")
        || lower.contains("raw_api_key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
}
