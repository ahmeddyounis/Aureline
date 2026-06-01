//! Stabilized client-origin, target-context, route class, capability-route
//! inspection, and expiry ownership disclosure packet.
//!
//! This module stabilizes the route-origin and capability-disclosure lane into
//! one export-safe packet that binds — for a single provider or remote flow on a
//! claimed stable row — the five facts a route-explicit lane must surface no
//! matter where the action is reached from:
//!
//! - the **capability-route inspector** — a single, attributable object naming
//!   route class, target identity, capability boundary, approval scope, expiry,
//!   and revalidation triggers — reachable from previews, approvals, deep links,
//!   browser handoffs, tunnels, provider actions, AI-tool runs, and support
//!   exports without requiring a debug or admin toggle;
//! - the **client-origin and target-context disclosure** — the named origin class
//!   (local client, browser companion, CLI/headless, deep link, AI tool) and
//!   target context (local workspace, remote workspace, managed provider, external
//!   provider, tunnel-exposed endpoint) every route decision is bound to, so a
//!   replay or audit can reconstruct the authority chain that was live at the
//!   moment of apply;
//! - the **route class and capability boundary** — the typed route class
//!   (local, browser handoff, managed, enterprise gateway, tunnel, provider
//!   action), the capability boundary the route crosses, and the spend/egress
//!   posture the route implies, surfaced identically in preview, approval, and
//!   support export;
//! - the **expiry and ownership disclosure** — who owns the approval, when it
//!   expires, and what revalidation trigger class forces visible reapproval rather
//!   than silent replay, enforced on every route and approval drift; and
//! - the **cross-surface inspector parity** — the same capability-route inspector
//!   object is reachable from every claimed stable surface (menu, keybinding,
//!   palette, CLI/headless, AI tool, voice, recipe, deep link, browser companion)
//!   and any surface still lacking stable qualification is automatically narrowed
//!   below Stable in product copy and release packets.
//!
//! It does not re-derive the descriptor, registry, invocation, authority,
//! high-risk hardening, or command-parity models. The
//! [`crate::stabilize_command_contract::CommandContractStabilizationPacket`],
//! [`crate::harden_high_risk_command::HighRiskCommandHardeningPacket`], and
//! [`crate::finalize_command_parity::CommandParityFinalizationPacket`] own those
//! contracts. This packet references them by stable schema ref and adds the
//! route-origin and capability-disclosure invariants the stable line needs so
//! UI, CLI, AI, support export, and deep-link surfaces all read the *same*
//! route truth.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw provider URLs, raw endpoint hostnames,
//! raw credentials, exact token counts, exact cost amounts, and billing-account
//! ids stay outside the support boundary.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stabilize_command_contract::{
    CommandContractEvidenceExport, CommandSurfaceClass, StableContractRefs,
    SurfaceQualificationClass,
};

/// Stable record-kind tag carried by [`ClientOriginRouteClassPacket`].
pub const STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_RECORD_KIND: &str =
    "client_origin_route_class_stabilization_packet";

/// Schema version for client-origin / route-class stabilization records.
pub const STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_SCHEMA_REF: &str =
    "schemas/commands/stabilize_client_origin_route_class.schema.json";

/// Repo-relative path of the stabilization doc.
pub const STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_DOC_REF: &str =
    "docs/commands/m4/stabilize_client_origin_route_class.md";

/// Repo-relative path of the frozen command-descriptor contract.
pub const STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_DESCRIPTOR_CONTRACT_REF: &str =
    "docs/commands/command_descriptor_contract.md";

/// Repo-relative path of the frozen invocation-result and parity contract.
pub const STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_PARITY_CONTRACT_REF: &str =
    "docs/commands/invocation_result_and_parity_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_FIXTURE_DIR: &str =
    "fixtures/commands/m4/stabilize_client_origin_route_class";

/// Repo-relative path of the checked-in support export artifact.
pub const STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_ARTIFACT_REF: &str =
    "artifacts/commands/m4/stabilize_client_origin_route_class/support_export.json";

/// Repo-relative path of the checked-in Markdown summary.
pub const STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_SUMMARY_REF: &str =
    "artifacts/commands/m4/stabilize_client_origin_route_class/summary.md";

/// Origin class naming where a command or action request was initiated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientOriginClass {
    /// Initiated from an interactive UI surface in the local Aureline process.
    LocalUi,
    /// Initiated from the CLI or a headless/scripted surface.
    CliHeadless,
    /// Initiated via a deep link (protocol handler, URL scheme, share link).
    DeepLink,
    /// Initiated from the browser companion extension.
    BrowserCompanion,
    /// Initiated by an AI tool-call within a running model session.
    AiToolCall,
    /// Initiated via a recipe or macro automation step.
    RecipeAutomation,
    /// Initiated through a tunnel-exposed endpoint (SSH tunnel, dev tunnel).
    TunnelEndpoint,
    /// Initiated from a provider action callback.
    ProviderAction,
}

impl ClientOriginClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUi => "local_ui",
            Self::CliHeadless => "cli_headless",
            Self::DeepLink => "deep_link",
            Self::BrowserCompanion => "browser_companion",
            Self::AiToolCall => "ai_tool_call",
            Self::RecipeAutomation => "recipe_automation",
            Self::TunnelEndpoint => "tunnel_endpoint",
            Self::ProviderAction => "provider_action",
        }
    }

    /// Client origin classes the inspector must cover.
    pub const fn required_coverage() -> [Self; 8] {
        [
            Self::LocalUi,
            Self::CliHeadless,
            Self::DeepLink,
            Self::BrowserCompanion,
            Self::AiToolCall,
            Self::RecipeAutomation,
            Self::TunnelEndpoint,
            Self::ProviderAction,
        ]
    }
}

/// Target context naming where the action's effects land.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetContextClass {
    /// Effects land in the local workspace on the current device.
    LocalWorkspace,
    /// Effects land in a remote workspace reached over an authenticated session.
    RemoteWorkspace,
    /// Effects land in a managed provider service (enterprise gateway, managed API).
    ManagedProvider,
    /// Effects land in an external provider (user-connected BYOK or self-hosted).
    ExternalProvider,
    /// Effects land via a tunnel-exposed endpoint.
    TunnelExposedEndpoint,
    /// Effects land in a browser tab brokered through the companion.
    BrowserHandoff,
}

impl TargetContextClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::RemoteWorkspace => "remote_workspace",
            Self::ManagedProvider => "managed_provider",
            Self::ExternalProvider => "external_provider",
            Self::TunnelExposedEndpoint => "tunnel_exposed_endpoint",
            Self::BrowserHandoff => "browser_handoff",
        }
    }
}

/// Coarse route class an action uses to reach its target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionRouteClass {
    /// Action stays on the local device without leaving the process.
    Local,
    /// Action is handed off to a browser tab via the companion.
    BrowserHandoff,
    /// Action reaches a managed enterprise provider.
    Managed,
    /// Action is brokered through an enterprise gateway.
    EnterpriseGateway,
    /// Action reaches an external provider over a tunnel.
    TunnelForwarded,
    /// Action is dispatched by a provider action callback.
    ProviderActionCallback,
    /// Action uses a BYOK user/org-held credential.
    Byok,
}

impl ActionRouteClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::BrowserHandoff => "browser_handoff",
            Self::Managed => "managed",
            Self::EnterpriseGateway => "enterprise_gateway",
            Self::TunnelForwarded => "tunnel_forwarded",
            Self::ProviderActionCallback => "provider_action_callback",
            Self::Byok => "byok",
        }
    }

    /// True when the route class crosses a managed-policy boundary.
    pub const fn is_managed_boundary(self) -> bool {
        matches!(self, Self::Managed | Self::EnterpriseGateway)
    }
}

/// Capability boundary the route crosses or stays within.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityBoundaryClass {
    /// Action stays within the local device capability boundary.
    LocalDevice,
    /// Action crosses a user-account boundary.
    UserAccount,
    /// Action crosses an organisation or enterprise boundary.
    OrgEnterprise,
    /// Action crosses a managed-policy capability boundary.
    ManagedPolicy,
    /// Action crosses an external provider boundary (leaves the enterprise).
    ExternalProvider,
    /// Action crosses a browser-companion capability boundary.
    BrowserCompanion,
    /// Action crosses a tunnel capability boundary.
    TunnelBoundary,
}

impl CapabilityBoundaryClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDevice => "local_device",
            Self::UserAccount => "user_account",
            Self::OrgEnterprise => "org_enterprise",
            Self::ManagedPolicy => "managed_policy",
            Self::ExternalProvider => "external_provider",
            Self::BrowserCompanion => "browser_companion",
            Self::TunnelBoundary => "tunnel_boundary",
        }
    }

    /// True when the boundary implies a policy-governed capability scope.
    pub const fn is_policy_governed(self) -> bool {
        matches!(self, Self::ManagedPolicy | Self::OrgEnterprise)
    }
}

/// Condition that forces visible reapproval rather than silent replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevalidationTriggerClass {
    /// The approval epoch changed since the last grant.
    ApprovalEpochDrift,
    /// The route or provider changed since the last grant.
    RouteOrProviderDrift,
    /// The target identity changed since the last grant.
    TargetIdentityDrift,
    /// The capability boundary changed since the last grant.
    CapabilityBoundaryDrift,
    /// The policy epoch advanced since the last grant.
    PolicyEpochAdvanced,
    /// The approval expiry passed.
    ApprovalExpired,
    /// An explicit revalidation was requested by the user or admin.
    ExplicitRevalidationRequested,
}

impl RevalidationTriggerClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApprovalEpochDrift => "approval_epoch_drift",
            Self::RouteOrProviderDrift => "route_or_provider_drift",
            Self::TargetIdentityDrift => "target_identity_drift",
            Self::CapabilityBoundaryDrift => "capability_boundary_drift",
            Self::PolicyEpochAdvanced => "policy_epoch_advanced",
            Self::ApprovalExpired => "approval_expired",
            Self::ExplicitRevalidationRequested => "explicit_revalidation_requested",
        }
    }

    /// Revalidation triggers a stable approval scope must enumerate.
    pub const fn required_coverage() -> [Self; 7] {
        [
            Self::ApprovalEpochDrift,
            Self::RouteOrProviderDrift,
            Self::TargetIdentityDrift,
            Self::CapabilityBoundaryDrift,
            Self::PolicyEpochAdvanced,
            Self::ApprovalExpired,
            Self::ExplicitRevalidationRequested,
        ]
    }
}

/// The capability-route inspector object surfaced at every claimed stable entry
/// point (preview, approval, deep link, browser handoff, tunnel, provider action,
/// AI-tool run, support export).
///
/// All fields carry opaque refs, state tokens, and coarse classes only. Raw
/// endpoint URLs, raw credentials, and exact token/cost figures stay outside the
/// export boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRouteInspector {
    /// Stable inspector id (opaque, never a URL or credential).
    pub inspector_id: String,
    /// Client origin class naming where the request was initiated.
    pub client_origin: ClientOriginClass,
    /// Target context class naming where effects land.
    pub target_context: TargetContextClass,
    /// Coarse route class the action will use.
    pub route_class: ActionRouteClass,
    /// Capability boundary the route crosses.
    pub capability_boundary: CapabilityBoundaryClass,
    /// Stable target identity ref (opaque, never raw hostname or credential).
    pub target_identity_ref: String,
    /// Approval scope ref governing this route.
    pub approval_scope_ref: String,
    /// Policy epoch ref the approval was evaluated under.
    pub policy_epoch_ref: String,
    /// Opaque expiry token (never a raw timestamp carrying user data).
    pub expiry_token: String,
    /// Who owns the approval scope (display-safe label, not PII).
    pub approval_owner_label: String,
    /// Revalidation triggers that force visible reapproval on drift.
    pub revalidation_triggers: Vec<RevalidationTriggerClass>,
    /// True when route or approval drift forces visible reapproval rather than
    /// silent replay.
    pub drift_forces_reapproval: bool,
    /// True when the inspector is reachable without a debug or admin toggle.
    pub reachable_without_debug_toggle: bool,
    /// Spend/egress posture token for the route (e.g. `entitlement_band`, `metered`).
    pub spend_posture_token: String,
    /// Ref to the approval disclosure shown in preview and approval UI.
    pub approval_disclosure_ref: String,
}

impl CapabilityRouteInspector {
    fn guards_hold(&self) -> bool {
        !self.inspector_id.trim().is_empty()
            && !self.target_identity_ref.trim().is_empty()
            && !self.approval_scope_ref.trim().is_empty()
            && !self.policy_epoch_ref.trim().is_empty()
            && !self.expiry_token.trim().is_empty()
            && !self.approval_owner_label.trim().is_empty()
            && !self.spend_posture_token.trim().is_empty()
            && !self.approval_disclosure_ref.trim().is_empty()
            && self.drift_forces_reapproval
            && self.reachable_without_debug_toggle
    }

    fn covers_all_revalidation_triggers(&self) -> bool {
        RevalidationTriggerClass::required_coverage()
            .iter()
            .all(|required| self.revalidation_triggers.iter().any(|t| t == required))
    }
}

/// Approval scope and ownership disclosure bound to a capability-route record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalScopeRecord {
    /// Stable approval scope ref.
    pub scope_ref: String,
    /// Display-safe owner label (not PII).
    pub owner_label: String,
    /// Opaque expiry token.
    pub expiry_token: String,
    /// True when expiry is enforced and an expired approval forces reapproval.
    pub expiry_enforced: bool,
    /// True when the scope is disclosed in the preview and approval UI.
    pub disclosed_in_preview: bool,
    /// True when the scope is disclosed in the support export.
    pub disclosed_in_support_export: bool,
}

impl ApprovalScopeRecord {
    fn guards_hold(&self) -> bool {
        !self.scope_ref.trim().is_empty()
            && !self.owner_label.trim().is_empty()
            && !self.expiry_token.trim().is_empty()
            && self.expiry_enforced
            && self.disclosed_in_preview
            && self.disclosed_in_support_export
    }
}

/// One inspector surface row naming how the capability-route inspector is
/// reachable from a specific entry point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorSurfaceRow {
    /// Command invocation surface this row covers.
    pub surface_class: CommandSurfaceClass,
    /// True when the inspector is reachable from this surface.
    pub inspector_reachable: bool,
    /// True when route class is disclosed on this surface.
    pub discloses_route_class: bool,
    /// True when target identity is disclosed on this surface.
    pub discloses_target_identity: bool,
    /// True when capability boundary is disclosed on this surface.
    pub discloses_capability_boundary: bool,
    /// True when approval scope and expiry are disclosed on this surface.
    pub discloses_approval_scope_and_expiry: bool,
    /// True when revalidation triggers are disclosed on this surface.
    pub discloses_revalidation_triggers: bool,
    /// True when this surface enforces the same policy checks as all others.
    pub policy_checked: bool,
    /// True when this surface never widens the capability boundary.
    pub no_capability_widening: bool,
    /// Stable-qualification posture for this surface.
    pub qualification: SurfaceQualificationClass,
    /// True when this surface claims the Stable lane.
    pub claimed_stable: bool,
}

impl InspectorSurfaceRow {
    fn preserves_full_disclosure(&self) -> bool {
        self.inspector_reachable
            && self.discloses_route_class
            && self.discloses_target_identity
            && self.discloses_capability_boundary
            && self.discloses_approval_scope_and_expiry
            && self.discloses_revalidation_triggers
            && self.policy_checked
            && self.no_capability_widening
    }
}

/// Constructor input for [`ClientOriginRouteClassPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientOriginRouteClassPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Command or action family id this packet covers.
    pub command_family_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Policy epoch ref this row was evaluated under.
    pub policy_epoch_ref: String,
    /// The single descriptor registry and result schema every surface projects.
    pub contract_refs: StableContractRefs,
    /// The capability-route inspector for this flow.
    pub capability_route_inspector: CapabilityRouteInspector,
    /// The approval scope and ownership record.
    pub approval_scope: ApprovalScopeRecord,
    /// Cross-surface inspector parity rows.
    pub surface_rows: Vec<InspectorSurfaceRow>,
    /// Exportable evidence lineage.
    pub evidence_export: CommandContractEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe client-origin, target-context, and route-class stabilization
/// record.
///
/// Binds the capability-route inspector, approval scope, expiry ownership
/// disclosure, and cross-surface parity for one provider or remote flow on a
/// claimed stable row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientOriginRouteClassPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Command or action family id this packet covers.
    pub command_family_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Policy epoch ref this row was evaluated under.
    pub policy_epoch_ref: String,
    /// The single descriptor registry and result schema every surface projects.
    pub contract_refs: StableContractRefs,
    /// The capability-route inspector for this flow.
    pub capability_route_inspector: CapabilityRouteInspector,
    /// The approval scope and ownership record.
    pub approval_scope: ApprovalScopeRecord,
    /// Cross-surface inspector parity rows.
    pub surface_rows: Vec<InspectorSurfaceRow>,
    /// Exportable evidence lineage.
    pub evidence_export: CommandContractEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ClientOriginRouteClassPacket {
    /// Builds a client-origin / route-class stabilization packet from
    /// canonical rows.
    pub fn new(input: ClientOriginRouteClassPacketInput) -> Self {
        Self {
            record_kind: STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_RECORD_KIND.to_owned(),
            schema_version: STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            command_family_id: input.command_family_id,
            display_label: input.display_label,
            claimed_stable: input.claimed_stable,
            policy_epoch_ref: input.policy_epoch_ref,
            contract_refs: input.contract_refs,
            capability_route_inspector: input.capability_route_inspector,
            approval_scope: input.approval_scope,
            surface_rows: input.surface_rows,
            evidence_export: input.evidence_export,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the packet's stable-line invariants.
    ///
    /// Returns every [`ClientOriginRouteClassViolation`] found; an empty vec
    /// means the packet is conformant.
    pub fn validate(&self) -> Vec<ClientOriginRouteClassViolation> {
        let mut violations = Vec::new();
        if self.record_kind != STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_RECORD_KIND {
            violations.push(ClientOriginRouteClassViolation::WrongRecordKind);
        }
        if self.schema_version != STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_SCHEMA_VERSION {
            violations.push(ClientOriginRouteClassViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.command_family_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ClientOriginRouteClassViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_contract_refs(self, &mut violations);
        validate_capability_route_inspector(self, &mut violations);
        validate_approval_scope(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_evidence_export(self, &mut violations);
        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("client-origin route-class packet serializes"),
        ) {
            violations.push(ClientOriginRouteClassViolation::RawMaterialInExport);
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
            .expect("client-origin route-class packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed_surfaces = self
            .surface_rows
            .iter()
            .filter(|row| !row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# Client-Origin, Target-Context, and Route-Class Stabilization\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Command family: `{}`\n",
            self.command_family_id
        ));
        out.push_str(&format!(
            "- Evidence id: `{}`\n",
            self.evidence_export.evidence_id
        ));
        out.push_str(&format!("- Claimed stable: {}\n", self.claimed_stable));
        out.push_str(&format!(
            "- Client origin: `{}`\n",
            self.capability_route_inspector.client_origin.as_str()
        ));
        out.push_str(&format!(
            "- Target context: `{}`\n",
            self.capability_route_inspector.target_context.as_str()
        ));
        out.push_str(&format!(
            "- Route class: `{}`\n",
            self.capability_route_inspector.route_class.as_str()
        ));
        out.push_str(&format!(
            "- Capability boundary: `{}`\n",
            self.capability_route_inspector.capability_boundary.as_str()
        ));
        out.push_str(&format!(
            "- Drift forces reapproval: {}\n",
            self.capability_route_inspector.drift_forces_reapproval
        ));
        out.push_str(&format!(
            "- Revalidation triggers: {}\n",
            self.capability_route_inspector.revalidation_triggers.len()
        ));
        out.push_str(&format!(
            "- Approval scope expiry enforced: {}\n",
            self.approval_scope.expiry_enforced
        ));
        out.push_str(&format!(
            "- Inspector surfaces: {} ({} narrowed below Stable)\n",
            self.surface_rows.len(),
            narrowed_surfaces
        ));
        out
    }
}

/// Errors emitted when reading the checked-in support export.
#[derive(Debug)]
pub enum ClientOriginRouteClassArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ClientOriginRouteClassViolation>),
}

impl fmt::Display for ClientOriginRouteClassArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "client-origin route-class export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "client-origin route-class export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ClientOriginRouteClassArtifactError {}

/// Validation failures emitted by [`ClientOriginRouteClassPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientOriginRouteClassViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The bound canonical contract refs drifted from the single registry/schema.
    ContractRefsNotCanonical,
    /// The capability-route inspector is missing required fields or guards.
    InspectorGuardsBroken,
    /// The inspector does not cover all required revalidation triggers.
    RevalidationTriggerCoverageMissing,
    /// The approval scope is missing required fields or guards.
    ApprovalScopeGuardsBroken,
    /// Cross-surface coverage is incomplete (missing a required surface class).
    InspectorSurfaceCoverageMissing,
    /// A stable, reachable surface dropped required disclosure or policy parity.
    InspectorSurfaceParityBroken,
    /// A surface narrowed below Stable still claims the Stable lane.
    UnqualifiedSurfaceClaimsStable,
    /// Evidence export refs are missing.
    EvidenceExportRefsMissing,
    /// The packet carries raw material outside the export boundary.
    RawMaterialInExport,
}

impl ClientOriginRouteClassViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ContractRefsNotCanonical => "contract_refs_not_canonical",
            Self::InspectorGuardsBroken => "inspector_guards_broken",
            Self::RevalidationTriggerCoverageMissing => "revalidation_trigger_coverage_missing",
            Self::ApprovalScopeGuardsBroken => "approval_scope_guards_broken",
            Self::InspectorSurfaceCoverageMissing => "inspector_surface_coverage_missing",
            Self::InspectorSurfaceParityBroken => "inspector_surface_parity_broken",
            Self::UnqualifiedSurfaceClaimsStable => "unqualified_surface_claims_stable",
            Self::EvidenceExportRefsMissing => "evidence_export_refs_missing",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Returns the checked-in client-origin / route-class support export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or
/// validate.
pub fn current_client_origin_route_class_export(
) -> Result<ClientOriginRouteClassPacket, ClientOriginRouteClassArtifactError> {
    let packet: ClientOriginRouteClassPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/commands/m4/stabilize_client_origin_route_class/support_export.json"
    )))
    .map_err(ClientOriginRouteClassArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ClientOriginRouteClassArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &ClientOriginRouteClassPacket,
    violations: &mut Vec<ClientOriginRouteClassViolation>,
) {
    for required in [
        STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_DOC_REF,
        STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_SCHEMA_REF,
        STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_DESCRIPTOR_CONTRACT_REF,
        STABILIZE_CLIENT_ORIGIN_ROUTE_CLASS_PARITY_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(ClientOriginRouteClassViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_contract_refs(
    packet: &ClientOriginRouteClassPacket,
    violations: &mut Vec<ClientOriginRouteClassViolation>,
) {
    if packet.contract_refs != StableContractRefs::canonical() {
        violations.push(ClientOriginRouteClassViolation::ContractRefsNotCanonical);
    }
}

fn validate_capability_route_inspector(
    packet: &ClientOriginRouteClassPacket,
    violations: &mut Vec<ClientOriginRouteClassViolation>,
) {
    let inspector = &packet.capability_route_inspector;
    if !inspector.guards_hold() {
        violations.push(ClientOriginRouteClassViolation::InspectorGuardsBroken);
    }
    if !inspector.covers_all_revalidation_triggers() {
        violations.push(ClientOriginRouteClassViolation::RevalidationTriggerCoverageMissing);
    }
}

fn validate_approval_scope(
    packet: &ClientOriginRouteClassPacket,
    violations: &mut Vec<ClientOriginRouteClassViolation>,
) {
    if !packet.approval_scope.guards_hold() {
        violations.push(ClientOriginRouteClassViolation::ApprovalScopeGuardsBroken);
    }
}

fn validate_surface_rows(
    packet: &ClientOriginRouteClassPacket,
    violations: &mut Vec<ClientOriginRouteClassViolation>,
) {
    for required in CommandSurfaceClass::required_coverage() {
        if !packet
            .surface_rows
            .iter()
            .any(|row| row.surface_class == required)
        {
            violations.push(ClientOriginRouteClassViolation::InspectorSurfaceCoverageMissing);
            break;
        }
    }
    for row in &packet.surface_rows {
        // A surface narrowed below Stable may not claim the Stable lane.
        if row.claimed_stable && !row.qualification.is_stable() {
            violations.push(ClientOriginRouteClassViolation::UnqualifiedSurfaceClaimsStable);
            break;
        }
        // A stable, reachable surface must preserve full disclosure and policy parity.
        if row.qualification.is_stable() && row.inspector_reachable && !row.preserves_full_disclosure() {
            violations.push(ClientOriginRouteClassViolation::InspectorSurfaceParityBroken);
            break;
        }
    }
}

fn validate_evidence_export(
    packet: &ClientOriginRouteClassPacket,
    violations: &mut Vec<ClientOriginRouteClassViolation>,
) {
    let export = &packet.evidence_export;
    if export.evidence_id.trim().is_empty()
        || export.json_export_ref.trim().is_empty()
        || export.markdown_summary_ref.trim().is_empty()
        || export.admin_inspector_ref.trim().is_empty()
        || export.support_export_ref.trim().is_empty()
    {
        violations.push(ClientOriginRouteClassViolation::EvidenceExportRefsMissing);
    }
}

fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_material(text),
        serde_json::Value::Array(values) => values.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

fn contains_forbidden_material(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("bearer ")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("private_key")
        || lower.contains("signing_key")
        || lower.contains("raw_prompt")
        || lower.contains("raw_diff")
        || lower.contains("raw_body")
        || lower.contains("billing-account")
}

#[cfg(test)]
mod tests;
