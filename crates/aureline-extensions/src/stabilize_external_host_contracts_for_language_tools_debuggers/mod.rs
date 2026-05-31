//! Stabilize the external-host contract for language tools, debuggers, CLIs,
//! and database / infrastructure adapters on the stable line.
//!
//! The runtime v1 *beta* admission contract ([`crate::runtime`]) owns the
//! per-session admission decision, and the stable runtime-ABI lane
//! ([`crate::stabilize_extension_runtime_v1_abi_capability_envelopes_and`])
//! owns the published, stable runtime truth a claimed stable row carries. This
//! module owns the layer those two leave open for **external hosts** — the
//! out-of-process language servers, debug adapters, CLI helpers, and
//! database / infrastructure adapters an extension drives through a separately
//! supervised host process or a remote / managed endpoint.
//!
//! Two things make external hosts different from a Wasm capability sandbox, and
//! this module makes both first-class instead of adapter-local strings:
//!
//! - **A typed data-plane contract for database / infrastructure adapters.**
//!   When the external host is a [`database_adapter`](EXTERNAL_HOST_KIND_CLASSES)
//!   or an [`infra_adapter`](EXTERNAL_HOST_KIND_CLASSES), the row carries an
//!   [`ExternalHostDataPlaneContract`] whose connection / target class,
//!   auth-source mode, read-only-versus-write-capable posture,
//!   local / tunneled / remote / managed origin, result / export safety, and
//!   control-plane-boundary truth are typed fields. An adapter that cannot name
//!   that contract cannot be constructed; a non-adapter host that tries to carry
//!   one is rejected.
//! - **Reconnect / replay honesty.** Every external host carries an
//!   [`ExternalHostReconnectReplaySafety`] record. After a host restart or a
//!   reconnect, an external host must never silently re-run a query, an
//!   apply-capable action, or a control-plane mutation: a reattach whose pending
//!   work has possible side effects must require an explicit replay or review
//!   path. A contract that claims a stateless safe-resume while side effects are
//!   pending, or that admits silently re-running side effects, is rejected at
//!   construction.
//!
//! The central rule mirrors the rest of the stable line: a **stable** external-host
//! claim is derived, never asserted. A row may render `stable` only when it pins
//! the published ABI contract version, is enforcement-backed (not catalog-asserted),
//! enforces its sandbox profile as published, keeps its publisher trust tier out
//! of quarantine, stays on a runnable lifecycle, keeps every active contribution
//! nominal, holds a bounded activation cost, is fully attributed, keeps its
//! connection state honest, and — for an adapter — sources auth from a managed
//! broker (never ambient), bounds its result / export, and never exposes an
//! unguarded mutating control plane. When any of those fails the visible tier is
//! **automatically narrowed below Stable** (`beta`, `preview`, or `withdrawn`)
//! with machine-readable reasons rather than left asserting a readiness the host
//! cannot back.
//!
//! The companion schema lives at
//! [`schemas/extensions/stable_external_host_contract.schema.json`](../../../../schemas/extensions/stable_external_host_contract.schema.json).
//! Canonical fixtures live under
//! `fixtures/extensions/m4/stabilize-external-host-contracts-for-language-tools-debuggers/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every stable external-host contract record.
pub const STABLE_EXTERNAL_HOST_SCHEMA_VERSION: u32 = 1;

/// The published, stable ABI contract version an external-host stable claim must
/// pin; any other version narrows below Stable.
pub const STABLE_EXTERNAL_HOST_PUBLISHED_ABI_VERSION: u32 = 1;

/// Canonical schema path the packet cites.
pub const STABLE_EXTERNAL_HOST_SCHEMA_REF: &str =
    "schemas/extensions/stable_external_host_contract.schema.json";

/// Record-kind tag for [`StableExternalHostContractPacket`].
pub const STABLE_EXTERNAL_HOST_PACKET_RECORD_KIND: &str = "stable_external_host_contract_packet";

/// Record-kind tag for [`ExternalHostContractIdentity`].
pub const EXTERNAL_HOST_IDENTITY_RECORD_KIND: &str = "stable_external_host_identity";

/// Record-kind tag for [`ExternalHostKindDeclaration`].
pub const EXTERNAL_HOST_KIND_DECLARATION_RECORD_KIND: &str =
    "stable_external_host_kind_declaration";

/// Record-kind tag for [`ExternalHostSandboxBinding`].
pub const EXTERNAL_HOST_SANDBOX_BINDING_RECORD_KIND: &str = "stable_external_host_sandbox_binding";

/// Record-kind tag for [`ExternalHostCapabilityEnvelope`].
pub const EXTERNAL_HOST_CAPABILITY_ENVELOPE_RECORD_KIND: &str =
    "stable_external_host_capability_envelope";

/// Record-kind tag for [`ExternalHostDataPlaneContract`].
pub const EXTERNAL_HOST_DATA_PLANE_CONTRACT_RECORD_KIND: &str =
    "stable_external_host_data_plane_contract";

/// Record-kind tag for [`ExternalHostReconnectReplaySafety`].
pub const EXTERNAL_HOST_RECONNECT_REPLAY_SAFETY_RECORD_KIND: &str =
    "stable_external_host_reconnect_replay_safety";

/// Record-kind tag for [`ExternalHostActivationBudget`].
pub const EXTERNAL_HOST_ACTIVATION_BUDGET_RECORD_KIND: &str =
    "stable_external_host_activation_budget";

/// Record-kind tag for [`ExternalHostContributionEntry`].
pub const EXTERNAL_HOST_CONTRIBUTION_ENTRY_RECORD_KIND: &str =
    "stable_external_host_contribution_entry";

/// Record-kind tag for [`ExternalHostQualificationClaim`].
pub const EXTERNAL_HOST_QUALIFICATION_CLAIM_RECORD_KIND: &str =
    "stable_external_host_qualification_claim";

/// Record-kind tag for [`ExternalHostDowngradedBanner`].
pub const EXTERNAL_HOST_DOWNGRADED_BANNER_RECORD_KIND: &str =
    "stable_external_host_downgraded_banner";

/// Record-kind tag for [`ExternalHostContractInspection`].
pub const EXTERNAL_HOST_INSPECTION_RECORD_KIND: &str = "stable_external_host_contract_inspection";

/// Record-kind tag for [`ExternalHostContractSupportExport`].
pub const EXTERNAL_HOST_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stable_external_host_contract_support_export";

/// Published, controlled external-host kind vocabulary.
pub const EXTERNAL_HOST_KIND_CLASSES: &[&str] = &[
    "language_tool",
    "debug_adapter",
    "cli_tool",
    "database_adapter",
    "infra_adapter",
];

/// External-host kinds whose row must carry a typed data-plane contract.
pub const DATA_PLANE_HOST_KINDS: &[&str] = &["database_adapter", "infra_adapter"];

/// Published host-protocol vocabulary.
pub const HOST_PROTOCOL_CLASSES: &[&str] = &[
    "language_server_protocol",
    "debug_adapter_protocol",
    "cli_invocation",
    "database_wire_protocol",
    "infra_control_api",
    "custom_host_protocol",
];

/// Closed execution-locus vocabulary for external hosts. External hosts never run
/// in-process; they are subprocesses, helper binaries, or remote / managed agents.
pub const EXTERNAL_HOST_EXECUTION_LOCUS_CLASSES: &[&str] = &[
    "dedicated_subprocess",
    "helper_binary",
    "remote_agent",
    "managed_service_endpoint",
];

/// Closed backend-classification vocabulary naming the enforcement backend.
pub const EXTERNAL_HOST_BACKEND_CLASSES: &[&str] = &[
    "os_process_sandbox",
    "seatbelt_sandbox_profile",
    "landlock_seccomp_profile",
    "app_container_profile",
    "remote_enforced_envelope",
];

/// Closed sandbox-enforcement-state vocabulary. `enforced_as_published` is the
/// only state a stable claim may keep; the others are the fail-closed downgrade.
pub const SANDBOX_ENFORCEMENT_STATES: &[&str] = &[
    "enforced_as_published",
    "fail_closed_downgraded",
    "unenforceable_refused",
];

/// Closed publisher-trust-tier vocabulary.
pub const TRUST_TIER_CLASSES: &[&str] = &[
    "verified_publisher",
    "known_publisher",
    "community_unverified",
    "quarantined",
];

/// Closed lifecycle-state vocabulary mirrored from the extension lifecycle lane.
pub const LIFECYCLE_STATE_CLASSES: &[&str] = &[
    "installed",
    "pending_activation",
    "active",
    "recovered",
    "degraded",
    "disabled",
    "quarantined",
    "removed",
    "publisher_blocked",
];

/// Lifecycle states a stable external-host claim may keep.
pub const RUNNABLE_LIFECYCLE_STATES: &[&str] =
    &["installed", "pending_activation", "active", "recovered"];

/// Closed contribution-kind vocabulary for external-host contributions.
pub const CONTRIBUTION_KIND_CLASSES: &[&str] = &[
    "language_feature",
    "debug_session",
    "command",
    "data_query",
    "control_action",
];

/// Closed contribution-host-state vocabulary, inspectable even when a contribution
/// is quarantined, bridged, or running on a narrower profile.
pub const CONTRIBUTION_HOST_STATE_CLASSES: &[&str] = &[
    "running_nominal",
    "downgraded_narrower_profile",
    "quarantined",
    "failed",
];

/// Closed connection / target class vocabulary for a data-plane adapter.
pub const CONNECTION_TARGET_CLASSES: &[&str] = &[
    "relational_database",
    "document_database",
    "key_value_store",
    "message_broker",
    "object_store",
    "container_runtime",
    "orchestrator_control_plane",
    "cloud_resource_manager",
    "secrets_manager",
    "generic_endpoint",
];

/// Closed auth-source-mode vocabulary. `ambient_environment` may never back a
/// stable adapter claim.
pub const AUTH_SOURCE_MODE_CLASSES: &[&str] = &[
    "workspace_credential_broker",
    "host_managed_keychain",
    "external_secret_reference",
    "ephemeral_session_token",
    "interactive_user_prompt",
    "ambient_environment",
];

/// Closed read-only-versus-write-capable posture vocabulary.
pub const WRITE_POSTURE_CLASSES: &[&str] =
    &["read_only", "write_capable", "control_plane_mutating"];

/// Closed adapter-origin vocabulary describing where the target system lives.
pub const ADAPTER_ORIGIN_CLASSES: &[&str] = &["local", "tunneled", "remote", "managed"];

/// Closed result / export safety vocabulary. `unbounded_unsafe` may never be stable.
pub const RESULT_EXPORT_SAFETY_CLASSES: &[&str] = &[
    "bounded_redacted",
    "bounded_full",
    "streamed_capped",
    "unbounded_unsafe",
];

/// Closed control-plane-boundary vocabulary. `unguarded_mutating` may never be stable.
pub const CONTROL_PLANE_BOUNDARY_CLASSES: &[&str] = &[
    "no_control_plane",
    "read_only_observability",
    "gated_apply_with_review",
    "unguarded_mutating",
];

/// Closed connection-state vocabulary; the row stays honest about its connection.
pub const CONNECTION_STATE_CLASSES: &[&str] = &[
    "connected_nominal",
    "disconnected_clean",
    "reconnect_pending",
    "disconnected_dirty",
    "quarantined",
];

/// Closed reattach-policy vocabulary. A reattach with possible side effects may
/// never claim `stateless_safe_resume`.
pub const REATTACH_POLICY_CLASSES: &[&str] = &[
    "stateless_safe_resume",
    "explicit_replay_required",
    "explicit_review_required",
    "blocked_pending_operator",
];

/// Closed pending-reattach side-effect vocabulary. `apply_capable` and
/// `control_plane_mutation` are side-effecting; the others are safe.
pub const PENDING_SIDE_EFFECT_CLASSES: &[&str] = &[
    "none",
    "query_results_only",
    "apply_capable",
    "control_plane_mutation",
];

/// Pending side-effect classes that make a reattach unsafe to resume silently.
pub const SIDE_EFFECTING_PENDING_CLASSES: &[&str] = &["apply_capable", "control_plane_mutation"];

/// Closed restart-posture vocabulary.
pub const RESTART_POSTURE_CLASSES: &[&str] = &[
    "no_restart_attempted",
    "one_warm_restart_under_budget",
    "exponential_backoff_bounded",
];

/// Closed activation-budget vocabulary. `unbounded_refused` may never be stable.
pub const ACTIVATION_BUDGET_CLASSES: &[&str] = &[
    "within_budget",
    "approaching_budget",
    "over_budget_throttled",
    "unbounded_refused",
];

/// Closed set of switch-readiness stability tiers.
pub const STABILITY_TIERS: &[&str] = &["stable", "beta", "preview", "withdrawn"];

/// Tiers that count as a *stable* external-host claim.
pub const STABLE_TIERS: &[&str] = &["stable"];

/// Closed claim-basis vocabulary. `catalog_asserted_only` may never back stable.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["enforcement_backed", "catalog_asserted_only"];

/// Closed support-claim vocabulary a tier may imply.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_external_host_ready_claim",
    "beta_external_host_partial_claim",
    "preview_external_host_experimental_claim",
    "withdrawn_no_external_host_claim",
];

/// Closed set of reasons that narrow a stable external-host claim below Stable.
pub const EXTERNAL_HOST_DOWNGRADE_REASONS: &[&str] = &[
    "abi_version_mismatch",
    "catalog_only_trust_not_enforcement_backed",
    "sandbox_fail_closed_downgraded",
    "sandbox_unenforceable",
    "trust_tier_quarantined",
    "lifecycle_not_runnable",
    "contribution_quarantined",
    "contribution_host_downgraded",
    "activation_cost_over_budget",
    "activation_cost_unbounded",
    "attribution_incomplete",
    "ambient_auth_source",
    "unbounded_result_export",
    "unguarded_control_plane_mutation",
    "connection_state_dirty",
    "reattach_review_pending",
];

/// Reasons that narrow all the way to `withdrawn`.
const WITHDRAWN_CLASS_REASONS: &[&str] = &[
    "sandbox_unenforceable",
    "lifecycle_not_runnable",
    "activation_cost_unbounded",
    "unbounded_result_export",
    "unguarded_control_plane_mutation",
];

/// Reasons that narrow to `preview` (any structural / security shortfall).
const PREVIEW_CLASS_REASONS: &[&str] = &[
    "abi_version_mismatch",
    "catalog_only_trust_not_enforcement_backed",
    "trust_tier_quarantined",
    "contribution_quarantined",
    "attribution_incomplete",
    "ambient_auth_source",
    "connection_state_dirty",
    "reattach_review_pending",
];

/// Reasons whose only effect is a safe narrowing to `beta`.
const BETA_CLASS_REASONS: &[&str] = &[
    "sandbox_fail_closed_downgraded",
    "contribution_host_downgraded",
    "activation_cost_over_budget",
];

/// Closed set of consumer surfaces that ingest this packet.
pub const STABLE_EXTERNAL_HOST_CONSUMER_SURFACES: &[&str] = &[
    "install_review",
    "runtime_inspector",
    "quarantine_flow",
    "diagnostics",
    "support_export",
    "docs_help_surface",
    "release_packet",
    "cli_inspector",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a stable external-host contract packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableExternalHostContractInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity input.
    pub identity: ExternalHostContractIdentityInput,
    /// External-host kind declaration input.
    pub host_kind_declaration: ExternalHostKindDeclarationInput,
    /// Sandbox-binding input.
    pub sandbox_binding: ExternalHostSandboxBindingInput,
    /// Capability-envelope input.
    pub capability_envelope: ExternalHostCapabilityEnvelopeInput,
    /// Typed data-plane contract; required for database / infra adapters and
    /// forbidden for other host kinds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_plane_contract: Option<ExternalHostDataPlaneContractInput>,
    /// Reconnect / replay safety input.
    pub reconnect_replay_safety: ExternalHostReconnectReplaySafetyInput,
    /// Activation-budget input.
    pub activation_budget: ExternalHostActivationBudgetInput,
    /// Active-contribution inspector entries.
    #[serde(default)]
    pub contributions: Vec<ExternalHostContributionEntryInput>,
    /// Stability qualification claim input.
    pub claim: ExternalHostQualificationClaimInput,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support / export surfaces.
    pub summary_label: String,
}

/// Input for [`ExternalHostContractIdentity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostContractIdentityInput {
    /// Ref to the runtime v1 beta admission contract this packet stabilizes.
    pub runtime_contract_ref: String,
    /// Opaque extension identity ref.
    pub extension_identity_ref: String,
    /// Extension version declared by the manifest baseline.
    pub extension_version: String,
    /// Ref to the source package the contributions came from.
    pub source_package_ref: String,
    /// ABI contract version this row pins.
    pub abi_contract_version: u32,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

/// Input for [`ExternalHostKindDeclaration`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostKindDeclarationInput {
    /// Published external-host kind.
    pub host_kind_class: String,
    /// Execution locus for the external host.
    pub execution_locus_class: String,
    /// Host protocol the extension drives.
    pub protocol_class: String,
}

/// Input for [`ExternalHostSandboxBinding`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostSandboxBindingInput {
    /// Stable, published sandbox profile id.
    pub sandbox_profile_id: String,
    /// Backend classification enforcing the profile.
    pub backend_classification_class: String,
    /// Enforcement state for the published profile.
    pub enforcement_state_class: String,
    /// Public label naming the platform backend.
    pub platform_backend_label: String,
    /// MUST be false; an external host can never widen to ambient full-user
    /// execution.
    pub widens_to_ambient_full_user: bool,
}

/// Input for [`ExternalHostCapabilityEnvelope`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostCapabilityEnvelopeInput {
    /// Opaque refs to the capabilities the manifest declared.
    #[serde(default)]
    pub declared_capability_refs: Vec<String>,
    /// Opaque refs to the capabilities the host negotiated (⊆ declared).
    #[serde(default)]
    pub negotiated_capability_refs: Vec<String>,
    /// Opaque refs to the capabilities the runtime granted (⊆ negotiated).
    #[serde(default)]
    pub granted_capability_refs: Vec<String>,
    /// Whether typed narrowing reasons were recorded for dropped capabilities.
    pub narrowing_reasons_recorded: bool,
}

/// Input for [`ExternalHostDataPlaneContract`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostDataPlaneContractInput {
    /// Connection / target class.
    pub connection_target_class: String,
    /// Auth-source mode.
    pub auth_source_mode_class: String,
    /// Read-only-versus-write-capable posture.
    pub write_posture_class: String,
    /// Local / tunneled / remote / managed origin.
    pub origin_class: String,
    /// Result / export safety.
    pub result_export_safety_class: String,
    /// Control-plane-boundary truth.
    pub control_plane_boundary_class: String,
    /// Opaque ref to the target system descriptor (host:port, cluster, account…).
    pub target_descriptor_ref: String,
}

/// Input for [`ExternalHostReconnectReplaySafety`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostReconnectReplaySafetyInput {
    /// Current connection state.
    pub connection_state_class: String,
    /// Restart posture for the host process.
    pub restart_posture_class: String,
    /// Whether the host is currently quarantined.
    pub quarantine_active: bool,
    /// Side-effect class of work that would be replayed on reattach.
    pub pending_reattach_side_effect_class: String,
    /// Reattach policy applied when the host reconnects.
    pub reattach_policy_class: String,
    /// MUST be false; an external host may never silently re-run a query,
    /// apply-capable action, or control-plane mutation after restart.
    pub silently_reruns_side_effects: bool,
}

/// Input for [`ExternalHostActivationBudget`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostActivationBudgetInput {
    /// Activation-budget class.
    pub budget_class: String,
    /// Opaque ref to the declared activation-cost budget.
    pub declared_cost_ref: String,
    /// Opaque ref to the observed activation-cost evidence.
    pub observed_cost_ref: String,
}

/// Input for one [`ExternalHostContributionEntry`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostContributionEntryInput {
    /// Stable contribution id.
    pub contribution_id: String,
    /// Contribution kind.
    pub contribution_kind_class: String,
    /// Source package the contribution belongs to.
    pub source_package_ref: String,
    /// Trust tier rendered on the inspector row.
    pub trust_tier_class: String,
    /// Opaque refs to the permissions the contribution actually used.
    #[serde(default)]
    pub used_permission_refs: Vec<String>,
    /// Ref to the last-known-good host for the contribution.
    pub last_known_good_host_ref: String,
    /// Current host state for the contribution.
    pub host_state_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`ExternalHostQualificationClaim`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostQualificationClaimInput {
    /// External-host tier claimed by the row.
    pub claimed_tier: String,
    /// Claim basis: enforcement-backed vs catalog-asserted only.
    pub claim_basis_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// Identity shared across every surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostContractIdentity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ref to the runtime v1 beta admission contract this packet stabilizes.
    pub runtime_contract_ref: String,
    /// Opaque extension identity ref.
    pub extension_identity_ref: String,
    /// Extension version.
    pub extension_version: String,
    /// Source package the contributions came from.
    pub source_package_ref: String,
    /// ABI contract version this row pins.
    pub abi_contract_version: u32,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

impl ExternalHostContractIdentity {
    /// Returns true when the row pins the published stable ABI version.
    pub fn abi_version_current(&self) -> bool {
        self.abi_contract_version == STABLE_EXTERNAL_HOST_PUBLISHED_ABI_VERSION
    }

    /// Returns true when the lifecycle is runnable.
    pub fn lifecycle_runnable(&self) -> bool {
        RUNNABLE_LIFECYCLE_STATES.contains(&self.lifecycle_state_class.as_str())
    }
}

/// Published external-host kind declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostKindDeclaration {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Published external-host kind.
    pub host_kind_class: String,
    /// Execution locus for the external host.
    pub execution_locus_class: String,
    /// Host protocol the extension drives.
    pub protocol_class: String,
}

impl ExternalHostKindDeclaration {
    /// Returns true when this host kind must carry a typed data-plane contract.
    pub fn requires_data_plane_contract(&self) -> bool {
        DATA_PLANE_HOST_KINDS.contains(&self.host_kind_class.as_str())
    }
}

/// Stable sandbox binding plus its fail-closed enforcement state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostSandboxBinding {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable, published sandbox profile id.
    pub sandbox_profile_id: String,
    /// Backend classification enforcing the profile.
    pub backend_classification_class: String,
    /// Enforcement state for the published profile.
    pub enforcement_state_class: String,
    /// Public label naming the platform backend.
    pub platform_backend_label: String,
    /// Always false; an external host never widens to ambient full-user.
    pub widens_to_ambient_full_user: bool,
}

impl ExternalHostSandboxBinding {
    /// Returns true when the published profile is enforced as published.
    pub fn enforced_as_published(&self) -> bool {
        self.enforcement_state_class == "enforced_as_published"
    }
}

/// Capability envelope that never widens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostCapabilityEnvelope {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Capabilities the manifest declared.
    pub declared_capability_refs: Vec<String>,
    /// Capabilities the host negotiated (⊆ declared).
    pub negotiated_capability_refs: Vec<String>,
    /// Capabilities the runtime granted (⊆ negotiated).
    pub granted_capability_refs: Vec<String>,
    /// Whether typed narrowing reasons were recorded for dropped capabilities.
    pub narrowing_reasons_recorded: bool,
}

impl ExternalHostCapabilityEnvelope {
    /// Returns true when granted ⊆ negotiated ⊆ declared.
    pub fn is_well_formed(&self) -> bool {
        is_subset(
            &self.negotiated_capability_refs,
            &self.declared_capability_refs,
        ) && is_subset(
            &self.granted_capability_refs,
            &self.negotiated_capability_refs,
        )
    }

    /// Returns true when capabilities were dropped without recording reasons.
    pub fn narrowing_diff_inconsistent(&self) -> bool {
        let narrowed = self.granted_capability_refs.len() < self.declared_capability_refs.len();
        narrowed && !self.narrowing_reasons_recorded
    }
}

/// Typed data-plane contract for a database / infrastructure adapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostDataPlaneContract {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Connection / target class.
    pub connection_target_class: String,
    /// Auth-source mode.
    pub auth_source_mode_class: String,
    /// Read-only-versus-write-capable posture.
    pub write_posture_class: String,
    /// Local / tunneled / remote / managed origin.
    pub origin_class: String,
    /// Result / export safety.
    pub result_export_safety_class: String,
    /// Control-plane-boundary truth.
    pub control_plane_boundary_class: String,
    /// Opaque ref to the target system descriptor.
    pub target_descriptor_ref: String,
}

impl ExternalHostDataPlaneContract {
    /// Returns true when the auth source is ambient (never stable).
    pub fn uses_ambient_auth(&self) -> bool {
        self.auth_source_mode_class == "ambient_environment"
    }

    /// Returns true when the result / export is unbounded (never stable).
    pub fn unbounded_export(&self) -> bool {
        self.result_export_safety_class == "unbounded_unsafe"
    }

    /// Returns true when the control plane is mutating but unguarded (never stable).
    pub fn unguarded_control_plane(&self) -> bool {
        self.control_plane_boundary_class == "unguarded_mutating"
    }
}

/// Reconnect / replay safety record. Keeps connection state honest and proves a
/// reattach with possible side effects requires an explicit replay or review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostReconnectReplaySafety {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Current connection state.
    pub connection_state_class: String,
    /// Restart posture for the host process.
    pub restart_posture_class: String,
    /// Whether the host is currently quarantined.
    pub quarantine_active: bool,
    /// Side-effect class of work that would be replayed on reattach.
    pub pending_reattach_side_effect_class: String,
    /// Reattach policy applied when the host reconnects.
    pub reattach_policy_class: String,
    /// Always false; an external host may never silently re-run side effects.
    pub silently_reruns_side_effects: bool,
}

impl ExternalHostReconnectReplaySafety {
    /// Returns true when the pending reattach work has possible side effects.
    pub fn pending_side_effects_possible(&self) -> bool {
        SIDE_EFFECTING_PENDING_CLASSES.contains(&self.pending_reattach_side_effect_class.as_str())
    }

    /// Returns true when the connection state is dirty or quarantined.
    pub fn connection_dirty(&self) -> bool {
        self.quarantine_active
            || matches!(
                self.connection_state_class.as_str(),
                "disconnected_dirty" | "quarantined"
            )
    }

    /// Returns true when a reattach requires an explicit operator replay / review
    /// because pending work has possible side effects.
    pub fn reattach_review_pending(&self) -> bool {
        self.pending_side_effects_possible()
            && matches!(
                self.reattach_policy_class.as_str(),
                "explicit_replay_required"
                    | "explicit_review_required"
                    | "blocked_pending_operator"
            )
    }
}

/// Activation-budget instrumentation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostActivationBudget {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Activation-budget class.
    pub budget_class: String,
    /// Opaque ref to the declared activation-cost budget.
    pub declared_cost_ref: String,
    /// Opaque ref to the observed activation-cost evidence.
    pub observed_cost_ref: String,
}

/// One active-contribution inspector entry. Always carries source package, trust
/// tier, used permissions, and the last-known-good host — even when quarantined,
/// bridged, or downgraded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostContributionEntry {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable contribution id.
    pub contribution_id: String,
    /// Contribution kind.
    pub contribution_kind_class: String,
    /// Source package the contribution belongs to.
    pub source_package_ref: String,
    /// Trust tier rendered on the inspector row.
    pub trust_tier_class: String,
    /// Opaque refs to the permissions the contribution actually used.
    pub used_permission_refs: Vec<String>,
    /// Ref to the last-known-good host for the contribution.
    pub last_known_good_host_ref: String,
    /// Current host state for the contribution.
    pub host_state_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl ExternalHostContributionEntry {
    /// Returns true when the entry is fully attributed for the inspector.
    pub fn is_attributed(&self) -> bool {
        !self.source_package_ref.trim().is_empty()
            && !self.last_known_good_host_ref.trim().is_empty()
            && !self.contribution_id.trim().is_empty()
    }
}

/// Stability qualification claim after the posture is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostQualificationClaim {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// External-host tier claimed by the row.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim the effective tier is allowed to imply.
    pub support_claim_class: String,
    /// Claim basis: enforcement-backed vs catalog-asserted only.
    pub claim_basis_class: String,
    /// True when the claimed tier was narrowed below Stable.
    pub downgraded: bool,
    /// Reasons that narrowed the claim.
    pub downgrade_reasons: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Downgraded-host banner requirement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostDowngradedBanner {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// True when a downgraded-host banner must be displayed.
    pub must_display: bool,
    /// Most-severe applicable banner reason, drawn from the downgrade vocabulary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_reason_class: Option<String>,
    /// Last-known-good host the banner points at, when a banner is shown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good_host_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Compact inspection row for CLI / headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostContractInspection {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet inspected by this row.
    pub packet_id_ref: String,
    /// External-host kind.
    pub host_kind_class: String,
    /// Host protocol.
    pub protocol_class: String,
    /// Execution locus.
    pub execution_locus_class: String,
    /// Effective external-host tier.
    pub effective_tier: String,
    /// True when the claim is a stable external-host claim.
    pub stable_claim: bool,
    /// True when the row pins the published ABI version.
    pub abi_version_current: bool,
    /// True when the sandbox is enforced as published.
    pub sandbox_enforced_as_published: bool,
    /// Always false; surfaced so a reviewer can see ambient widening is forbidden.
    pub widens_to_ambient_full_user: bool,
    /// True when granted ⊆ negotiated ⊆ declared.
    pub capability_envelope_well_formed: bool,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// True when the lifecycle is runnable.
    pub lifecycle_runnable: bool,
    /// True when a typed data-plane contract is present.
    pub data_plane_contract_present: bool,
    /// Write posture, when a data-plane contract is present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_posture_class: Option<String>,
    /// Auth-source mode, when a data-plane contract is present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_source_mode_class: Option<String>,
    /// Control-plane boundary, when a data-plane contract is present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_plane_boundary_class: Option<String>,
    /// Adapter origin, when a data-plane contract is present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_class: Option<String>,
    /// Result / export safety, when a data-plane contract is present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_export_safety_class: Option<String>,
    /// Connection state.
    pub connection_state_class: String,
    /// Reattach policy.
    pub reattach_policy_class: String,
    /// Pending reattach side-effect class.
    pub pending_reattach_side_effect_class: String,
    /// Always false; surfaced so a reviewer can see silent reruns are forbidden.
    pub silently_reruns_side_effects: bool,
    /// True when a reattach is pending an explicit replay / review.
    pub reattach_review_pending: bool,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// True when a downgraded-host banner is required.
    pub downgraded_host_banner_required: bool,
    /// True when every active contribution is fully attributed.
    pub attribution_complete: bool,
    /// Number of active contributions.
    pub active_contribution_count: usize,
    /// Number of quarantined / failed contributions.
    pub quarantined_contribution_count: usize,
    /// Number of contributions on a narrower-than-published profile.
    pub downgraded_contribution_count: usize,
    /// Number of declared capabilities.
    pub declared_capability_count: usize,
    /// Number of negotiated capabilities.
    pub negotiated_capability_count: usize,
    /// Number of granted capabilities.
    pub granted_capability_count: usize,
    /// Activation-budget class.
    pub activation_budget_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Stable external-host contract packet consumed by install review, the runtime
/// inspector, the quarantine flow, diagnostics, support export, docs / help, and
/// release packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableExternalHostContractPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity.
    pub identity: ExternalHostContractIdentity,
    /// External-host kind declaration.
    pub host_kind_declaration: ExternalHostKindDeclaration,
    /// Sandbox binding.
    pub sandbox_binding: ExternalHostSandboxBinding,
    /// Capability envelope.
    pub capability_envelope: ExternalHostCapabilityEnvelope,
    /// Typed data-plane contract (present for database / infra adapters).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_plane_contract: Option<ExternalHostDataPlaneContract>,
    /// Reconnect / replay safety.
    pub reconnect_replay_safety: ExternalHostReconnectReplaySafety,
    /// Activation-budget instrumentation.
    pub activation_budget: ExternalHostActivationBudget,
    /// Active-contribution inspector entries.
    pub contributions: Vec<ExternalHostContributionEntry>,
    /// Stability qualification claim after the posture is applied.
    pub claim: ExternalHostQualificationClaim,
    /// Downgraded-host banner requirement.
    pub downgraded_host_banner: ExternalHostDowngradedBanner,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the packet cites.
    pub source_schema_refs: Vec<String>,
    /// False so an external host can never silently widen to ambient.
    pub allows_ambient_full_user_widening: bool,
    /// False so catalog-only trust cannot back a stable external-host claim.
    pub allows_catalog_only_trust: bool,
    /// False so an unbounded activation cost cannot ride the stable line.
    pub allows_unbounded_activation_cost: bool,
    /// False so a host downgrade can never be silent.
    pub allows_silent_host_downgrade: bool,
    /// False so a reattach can never silently re-run side effects.
    pub allows_silent_side_effect_replay: bool,
    /// Inspection row.
    pub inspection: ExternalHostContractInspection,
}

impl StableExternalHostContractPacket {
    /// Builds a stable external-host contract packet from input, applying the
    /// posture to the claimed tier so any required downgrade below Stable is
    /// automatic.
    ///
    /// # Errors
    ///
    /// Returns [`StableExternalHostValidationError`] when the input violates an
    /// identity, host-kind, sandbox, capability-envelope, data-plane,
    /// reconnect / replay, contribution, or claim invariant — including the hard
    /// security guardrails (ambient widening, capability envelope widening,
    /// silent side-effect replay, a side-effecting reattach claiming a stateless
    /// safe resume, a data-plane host kind missing its typed contract).
    pub fn from_input(
        input: StableExternalHostContractInput,
    ) -> Result<Self, StableExternalHostValidationError> {
        validate_input(&input)?;

        let identity = identity_record(&input.identity);
        let host_kind_declaration = host_kind_record(&input.host_kind_declaration);
        let sandbox_binding = sandbox_record(&input.sandbox_binding);
        let capability_envelope = envelope_record(&input.capability_envelope);
        let data_plane_contract = input.data_plane_contract.as_ref().map(data_plane_record);
        let reconnect_replay_safety = reconnect_record(&input.reconnect_replay_safety);
        let activation_budget = activation_budget_record(&input.activation_budget);
        let contributions: Vec<ExternalHostContributionEntry> = input
            .contributions
            .iter()
            .map(contribution_record)
            .collect();
        let claim = claim_record(
            &input.claim,
            &identity,
            &sandbox_binding,
            data_plane_contract.as_ref(),
            &reconnect_replay_safety,
            &activation_budget,
            &contributions,
        );
        let downgraded_host_banner = banner_record(
            &sandbox_binding,
            &identity,
            data_plane_contract.as_ref(),
            &reconnect_replay_safety,
            &contributions,
        );
        let inspection = inspection_record(
            &input.packet_id,
            &host_kind_declaration,
            &identity,
            &sandbox_binding,
            &capability_envelope,
            data_plane_contract.as_ref(),
            &reconnect_replay_safety,
            &activation_budget,
            &contributions,
            &claim,
            &downgraded_host_banner,
        );

        let packet = Self {
            record_kind: STABLE_EXTERNAL_HOST_PACKET_RECORD_KIND.to_string(),
            schema_version: STABLE_EXTERNAL_HOST_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            identity,
            host_kind_declaration,
            sandbox_binding,
            capability_envelope,
            data_plane_contract,
            reconnect_replay_safety,
            activation_budget,
            contributions,
            claim,
            downgraded_host_banner,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![STABLE_EXTERNAL_HOST_SCHEMA_REF.to_string()],
            allows_ambient_full_user_widening: false,
            allows_catalog_only_trust: false,
            allows_unbounded_activation_cost: false,
            allows_silent_host_downgrade: false,
            allows_silent_side_effect_replay: false,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the stable external-host contract invariants.
    ///
    /// # Errors
    ///
    /// Returns [`StableExternalHostValidationError`] when an invariant is violated.
    pub fn validate(&self) -> Result<(), StableExternalHostValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            STABLE_EXTERNAL_HOST_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            STABLE_EXTERNAL_HOST_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_identity(&self.identity)?;
        validate_host_kind(&self.host_kind_declaration)?;
        validate_sandbox(&self.sandbox_binding)?;
        validate_envelope(&self.capability_envelope)?;
        validate_data_plane_presence(
            &self.host_kind_declaration,
            self.data_plane_contract.as_ref(),
        )?;
        if let Some(dp) = &self.data_plane_contract {
            validate_data_plane(dp)?;
        }
        validate_reconnect(&self.reconnect_replay_safety)?;
        validate_activation_budget(&self.activation_budget)?;
        for entry in &self.contributions {
            validate_contribution(entry)?;
        }
        validate_claim(&self.claim)?;
        validate_banner(&self.downgraded_host_banner)?;

        for surface in &self.consumer_surfaces {
            ensure_token(
                STABLE_EXTERNAL_HOST_CONSUMER_SURFACES,
                surface,
                "consumer_surface",
            )?;
        }
        if self.consumer_surfaces.is_empty() {
            return Err(err("packet must bind at least one consumer surface"));
        }
        if !self
            .source_schema_refs
            .iter()
            .any(|r| r == STABLE_EXTERNAL_HOST_SCHEMA_REF)
        {
            return Err(err("packet must cite its source schema"));
        }

        // No ambient widening, catalog-only trust, unbounded cost, silent host
        // downgrade, or silent side-effect replay may ride a stable row.
        if self.allows_ambient_full_user_widening
            || self.allows_catalog_only_trust
            || self.allows_unbounded_activation_cost
            || self.allows_silent_host_downgrade
            || self.allows_silent_side_effect_replay
        {
            return Err(err(
                "a stable external-host packet must not allow ambient widening, catalog-only trust, unbounded activation cost, silent host downgrade, or silent side-effect replay",
            ));
        }

        // Hard reconnect / replay guardrails, re-checked on a parsed packet.
        if self.reconnect_replay_safety.silently_reruns_side_effects {
            return Err(err(
                "an external host must never silently re-run a query, apply-capable action, or control-plane mutation after restart",
            ));
        }
        if self.reconnect_replay_safety.pending_side_effects_possible()
            && self.reconnect_replay_safety.reattach_policy_class == "stateless_safe_resume"
        {
            return Err(err(
                "a reattach with possible side effects must require an explicit replay or review path, not a stateless safe resume",
            ));
        }

        // Stable-claim binding: a stable effective tier must pin the published ABI,
        // be enforcement-backed, enforce its sandbox as published, keep the trust
        // tier out of quarantine, stay runnable, keep every contribution nominal,
        // hold a bounded activation cost, be fully attributed, keep its connection
        // honest, have no pending side-effecting reattach, and — for an adapter —
        // source auth from a managed broker, bound its export, and never expose an
        // unguarded mutating control plane.
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            if !self.identity.abi_version_current() {
                return Err(err(
                    "stable effective tier must pin the published ABI contract version",
                ));
            }
            if self.claim.claim_basis_class != "enforcement_backed" {
                return Err(err(
                    "stable effective tier must be enforcement-backed, not catalog-asserted",
                ));
            }
            if !self.sandbox_binding.enforced_as_published() {
                return Err(err(
                    "stable effective tier must enforce its sandbox profile as published",
                ));
            }
            if self.identity.publisher_trust_tier_class == "quarantined" {
                return Err(err(
                    "stable effective tier must not carry a quarantined trust tier",
                ));
            }
            if !self.identity.lifecycle_runnable() {
                return Err(err(
                    "stable effective tier must stay on a runnable lifecycle",
                ));
            }
            if self
                .contributions
                .iter()
                .any(|c| matches!(c.host_state_class.as_str(), "quarantined" | "failed"))
            {
                return Err(err(
                    "stable effective tier must not carry a quarantined or failed contribution",
                ));
            }
            if self
                .contributions
                .iter()
                .any(|c| c.host_state_class == "downgraded_narrower_profile")
            {
                return Err(err(
                    "stable effective tier must not carry a downgraded contribution",
                ));
            }
            if !matches!(
                self.activation_budget.budget_class.as_str(),
                "within_budget" | "approaching_budget"
            ) {
                return Err(err(
                    "stable effective tier must hold a bounded activation cost",
                ));
            }
            if !self.attribution_complete() {
                return Err(err("stable effective tier must be fully attributed"));
            }
            if self.reconnect_replay_safety.connection_dirty() {
                return Err(err(
                    "stable effective tier must keep an honest, non-dirty connection state",
                ));
            }
            if self.reconnect_replay_safety.reattach_review_pending() {
                return Err(err(
                    "stable effective tier must not have a side-effecting reattach pending review",
                ));
            }
            if let Some(dp) = &self.data_plane_contract {
                if dp.uses_ambient_auth() {
                    return Err(err(
                        "stable effective tier must not source adapter auth from the ambient environment",
                    ));
                }
                if dp.unbounded_export() {
                    return Err(err("stable effective tier must bound its result / export"));
                }
                if dp.unguarded_control_plane() {
                    return Err(err(
                        "stable effective tier must not expose an unguarded mutating control plane",
                    ));
                }
            }
            if self.claim.downgraded {
                return Err(err(
                    "a stable effective tier must not also be marked downgraded",
                ));
            }
        }

        // Downgrade truth: a downgraded claim carries at least one reason and never
        // keeps a stable effective tier.
        if self.claim.downgraded {
            if self.claim.downgrade_reasons.is_empty() {
                return Err(err("a downgraded claim must carry at least one reason"));
            }
            if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
                return Err(err("a downgraded claim must not keep a stable tier"));
            }
        }

        // Re-derive the effective tier and downgrade verdict so the stored claim
        // cannot drift from the posture truth.
        let derived = derive_effective_tier(
            &self.claim.claimed_tier,
            &self.claim.claim_basis_class,
            &self.identity,
            &self.sandbox_binding,
            self.data_plane_contract.as_ref(),
            &self.reconnect_replay_safety,
            &self.activation_budget,
            &self.contributions,
            self.attribution_complete(),
        );
        if derived.effective_tier != self.claim.effective_tier {
            return Err(err(
                "stored effective tier does not match the posture-derived tier",
            ));
        }
        if derived.downgraded != self.claim.downgraded {
            return Err(err(
                "stored downgrade flag does not match the posture-derived verdict",
            ));
        }
        let stored: BTreeSet<&str> = self
            .claim
            .downgrade_reasons
            .iter()
            .map(String::as_str)
            .collect();
        let expected: BTreeSet<&str> = derived
            .downgrade_reasons
            .iter()
            .map(String::as_str)
            .collect();
        if stored != expected {
            return Err(err(
                "stored downgrade reasons do not match the posture-derived reasons",
            ));
        }

        // Banner truth: a banner must be raised exactly when the posture is
        // downgraded, and never silently suppressed.
        let banner_required = host_is_downgraded(
            &self.sandbox_binding,
            &self.identity,
            self.data_plane_contract.as_ref(),
            &self.reconnect_replay_safety,
            &self.contributions,
        );
        if self.downgraded_host_banner.must_display != banner_required {
            return Err(err(
                "downgraded-host banner must_display does not match the host posture",
            ));
        }

        validate_inspection(&self.inspection, self)?;
        Ok(())
    }

    /// Returns true when no stable claim is implied from catalog-only trust.
    pub fn no_catalog_only_stable_claim(&self) -> bool {
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            return self.claim.claim_basis_class == "enforcement_backed";
        }
        true
    }

    /// Returns true when every active contribution is fully attributed.
    pub fn attribution_complete(&self) -> bool {
        !self.identity.source_package_ref.trim().is_empty()
            && self.contributions.iter().all(|c| c.is_attributed())
    }
}

// ---------------------------------------------------------------------------
// Projection
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI / headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableExternalHostContractProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Extension identity.
    pub extension_identity_ref: String,
    /// External-host kind.
    pub host_kind_class: String,
    /// Host protocol.
    pub protocol_class: String,
    /// Execution locus.
    pub execution_locus_class: String,
    /// Claimed tier.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim is a stable external-host claim.
    pub stable_claim: bool,
    /// True when a typed data-plane contract is present.
    pub data_plane_contract_present: bool,
    /// Connection state.
    pub connection_state_class: String,
    /// Reattach policy.
    pub reattach_policy_class: String,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// Downgrade reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-host banner is required.
    pub downgraded_host_banner_required: bool,
    /// Number of active contributions.
    pub active_contribution_count: usize,
}

impl From<StableExternalHostContractPacket> for StableExternalHostContractProjection {
    fn from(packet: StableExternalHostContractPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            extension_identity_ref: packet.identity.extension_identity_ref,
            host_kind_class: packet.host_kind_declaration.host_kind_class,
            protocol_class: packet.host_kind_declaration.protocol_class,
            execution_locus_class: packet.host_kind_declaration.execution_locus_class,
            claimed_tier: packet.claim.claimed_tier,
            effective_tier: packet.claim.effective_tier,
            support_claim_class: packet.claim.support_claim_class,
            stable_claim: packet.inspection.stable_claim,
            data_plane_contract_present: packet.data_plane_contract.is_some(),
            connection_state_class: packet.reconnect_replay_safety.connection_state_class,
            reattach_policy_class: packet.reconnect_replay_safety.reattach_policy_class,
            downgraded: packet.claim.downgraded,
            downgrade_reasons: packet.claim.downgrade_reasons,
            downgraded_host_banner_required: packet.downgraded_host_banner.must_display,
            active_contribution_count: packet.contributions.len(),
        }
    }
}

/// Parses and validates a materialized packet, returning the compact projection.
///
/// # Errors
///
/// Returns [`StableExternalHostError`] when the payload fails to parse or violates
/// the stable external-host contract invariants.
pub fn project_stable_external_host_contract(
    payload: &str,
) -> Result<StableExternalHostContractProjection, StableExternalHostError> {
    let packet: StableExternalHostContractPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(StableExternalHostContractProjection::from(packet))
}

// ---------------------------------------------------------------------------
// Support export projection
// ---------------------------------------------------------------------------

/// Metadata-safe support / partner export row that quotes the same closed tokens as
/// the packet without leaking raw manifest, profile, target, or payload bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHostContractSupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub export_id: String,
    /// Ref to the packet this export quotes.
    pub packet_ref: String,
    /// Extension identity.
    pub extension_identity_ref: String,
    /// Extension version.
    pub extension_version: String,
    /// Source package.
    pub source_package_ref: String,
    /// External-host kind.
    pub host_kind_class: String,
    /// Host protocol.
    pub protocol_class: String,
    /// Execution locus.
    pub execution_locus_class: String,
    /// Backend classification.
    pub backend_classification_class: String,
    /// Sandbox profile id.
    pub sandbox_profile_id: String,
    /// Sandbox enforcement state.
    pub sandbox_enforcement_state_class: String,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// Lifecycle state.
    pub lifecycle_state_class: String,
    /// True when a typed data-plane contract is present.
    pub data_plane_contract_present: bool,
    /// Connection / target class, when an adapter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection_target_class: Option<String>,
    /// Auth-source mode, when an adapter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_source_mode_class: Option<String>,
    /// Write posture, when an adapter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_posture_class: Option<String>,
    /// Adapter origin, when an adapter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_class: Option<String>,
    /// Result / export safety, when an adapter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_export_safety_class: Option<String>,
    /// Control-plane boundary, when an adapter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_plane_boundary_class: Option<String>,
    /// Connection state.
    pub connection_state_class: String,
    /// Reattach policy.
    pub reattach_policy_class: String,
    /// Pending reattach side-effect class.
    pub pending_reattach_side_effect_class: String,
    /// True when a reattach is pending an explicit replay / review.
    pub reattach_review_pending: bool,
    /// Effective tier.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim was narrowed below Stable.
    pub downgraded: bool,
    /// Narrowing reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-host banner is required.
    pub downgraded_host_banner_required: bool,
    /// True when the effective tier blocks activation (withdrawn).
    pub blocks_activation: bool,
    /// Number of active contributions.
    pub active_contribution_count: usize,
    /// Number of quarantined / failed contributions.
    pub quarantined_contribution_count: usize,
    /// Number of contributions on a narrower-than-published profile.
    pub downgraded_contribution_count: usize,
    /// Activation-budget class.
    pub activation_budget_class: String,
    /// Export-safe summary suitable for support / partner consumers.
    pub export_safe_summary: String,
}

/// Projects a packet into the metadata-safe support / partner export row.
pub fn project_stable_external_host_contract_support_export(
    packet: &StableExternalHostContractPacket,
) -> ExternalHostContractSupportExport {
    let blocks_activation = packet.claim.effective_tier == "withdrawn";
    let dp = packet.data_plane_contract.as_ref();
    let export_safe_summary = format!(
        "{} Host kind={} protocol={} locus={}. Sandbox profile={} backend={} enforcement={}. Trust={} lifecycle={}. Data-plane={} (write={}, auth={}, control_plane={}, export={}). Connection={} reattach={} pending={} (review_pending={}). Tier claimed={} effective={} (downgraded={}). Banner required={}. Contributions: active={} quarantined={} downgraded={}. Activation budget={}.",
        packet.claim.summary_label,
        packet.host_kind_declaration.host_kind_class,
        packet.host_kind_declaration.protocol_class,
        packet.host_kind_declaration.execution_locus_class,
        packet.sandbox_binding.sandbox_profile_id,
        packet.sandbox_binding.backend_classification_class,
        packet.sandbox_binding.enforcement_state_class,
        packet.identity.publisher_trust_tier_class,
        packet.identity.lifecycle_state_class,
        dp.is_some(),
        dp.map(|d| d.write_posture_class.as_str()).unwrap_or("n/a"),
        dp.map(|d| d.auth_source_mode_class.as_str()).unwrap_or("n/a"),
        dp.map(|d| d.control_plane_boundary_class.as_str())
            .unwrap_or("n/a"),
        dp.map(|d| d.result_export_safety_class.as_str())
            .unwrap_or("n/a"),
        packet.reconnect_replay_safety.connection_state_class,
        packet.reconnect_replay_safety.reattach_policy_class,
        packet.reconnect_replay_safety.pending_reattach_side_effect_class,
        packet.reconnect_replay_safety.reattach_review_pending(),
        packet.claim.claimed_tier,
        packet.claim.effective_tier,
        packet.claim.downgraded,
        packet.downgraded_host_banner.must_display,
        packet.inspection.active_contribution_count,
        packet.inspection.quarantined_contribution_count,
        packet.inspection.downgraded_contribution_count,
        packet.activation_budget.budget_class,
    );

    ExternalHostContractSupportExport {
        record_kind: EXTERNAL_HOST_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: STABLE_EXTERNAL_HOST_SCHEMA_VERSION,
        export_id: format!("stable_external_host_support_export:{}", packet.packet_id),
        packet_ref: packet.packet_id.clone(),
        extension_identity_ref: packet.identity.extension_identity_ref.clone(),
        extension_version: packet.identity.extension_version.clone(),
        source_package_ref: packet.identity.source_package_ref.clone(),
        host_kind_class: packet.host_kind_declaration.host_kind_class.clone(),
        protocol_class: packet.host_kind_declaration.protocol_class.clone(),
        execution_locus_class: packet.host_kind_declaration.execution_locus_class.clone(),
        backend_classification_class: packet.sandbox_binding.backend_classification_class.clone(),
        sandbox_profile_id: packet.sandbox_binding.sandbox_profile_id.clone(),
        sandbox_enforcement_state_class: packet.sandbox_binding.enforcement_state_class.clone(),
        trust_tier_class: packet.identity.publisher_trust_tier_class.clone(),
        lifecycle_state_class: packet.identity.lifecycle_state_class.clone(),
        data_plane_contract_present: dp.is_some(),
        connection_target_class: dp.map(|d| d.connection_target_class.clone()),
        auth_source_mode_class: dp.map(|d| d.auth_source_mode_class.clone()),
        write_posture_class: dp.map(|d| d.write_posture_class.clone()),
        origin_class: dp.map(|d| d.origin_class.clone()),
        result_export_safety_class: dp.map(|d| d.result_export_safety_class.clone()),
        control_plane_boundary_class: dp.map(|d| d.control_plane_boundary_class.clone()),
        connection_state_class: packet
            .reconnect_replay_safety
            .connection_state_class
            .clone(),
        reattach_policy_class: packet.reconnect_replay_safety.reattach_policy_class.clone(),
        pending_reattach_side_effect_class: packet
            .reconnect_replay_safety
            .pending_reattach_side_effect_class
            .clone(),
        reattach_review_pending: packet.reconnect_replay_safety.reattach_review_pending(),
        effective_tier: packet.claim.effective_tier.clone(),
        support_claim_class: packet.claim.support_claim_class.clone(),
        downgraded: packet.claim.downgraded,
        downgrade_reasons: packet.claim.downgrade_reasons.clone(),
        downgraded_host_banner_required: packet.downgraded_host_banner.must_display,
        blocks_activation,
        active_contribution_count: packet.inspection.active_contribution_count,
        quarantined_contribution_count: packet.inspection.quarantined_contribution_count,
        downgraded_contribution_count: packet.inspection.downgraded_contribution_count,
        activation_budget_class: packet.activation_budget.budget_class.clone(),
        export_safe_summary,
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for stable external-host contract operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableExternalHostError {
    /// Validation failed.
    Validation(StableExternalHostValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for StableExternalHostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for StableExternalHostError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for stable external-host contract packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableExternalHostValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for StableExternalHostValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StableExternalHostValidationError {}

impl StableExternalHostValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<serde_json::Error> for StableExternalHostError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(StableExternalHostValidationError {
            message: err.to_string(),
        })
    }
}

impl From<StableExternalHostValidationError> for StableExternalHostError {
    fn from(err: StableExternalHostValidationError) -> Self {
        Self::Validation(err)
    }
}

// ---------------------------------------------------------------------------
// Effective-tier derivation (the automatic narrowing)
// ---------------------------------------------------------------------------

struct DerivedTier {
    effective_tier: String,
    support_claim: String,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
}

/// Applies the external-host posture to a claimed tier, narrowing automatically
/// below Stable when the host can no longer back it.
#[allow(clippy::too_many_arguments)]
fn derive_effective_tier(
    claimed_tier: &str,
    claim_basis: &str,
    identity: &ExternalHostContractIdentity,
    sandbox: &ExternalHostSandboxBinding,
    data_plane: Option<&ExternalHostDataPlaneContract>,
    reconnect: &ExternalHostReconnectReplaySafety,
    activation_budget: &ExternalHostActivationBudget,
    contributions: &[ExternalHostContributionEntry],
    attribution_complete: bool,
) -> DerivedTier {
    // Non-stable claims are already honest; they pass through unchanged.
    if !STABLE_TIERS.contains(&claimed_tier) {
        return DerivedTier {
            effective_tier: claimed_tier.to_string(),
            support_claim: support_claim_for(claimed_tier),
            downgraded: false,
            downgrade_reasons: Vec::new(),
        };
    }

    let mut reasons: Vec<String> = Vec::new();

    if !identity.abi_version_current() {
        reasons.push("abi_version_mismatch".to_string());
    }
    if claim_basis != "enforcement_backed" {
        reasons.push("catalog_only_trust_not_enforcement_backed".to_string());
    }
    match sandbox.enforcement_state_class.as_str() {
        "fail_closed_downgraded" => reasons.push("sandbox_fail_closed_downgraded".to_string()),
        "unenforceable_refused" => reasons.push("sandbox_unenforceable".to_string()),
        _ => {}
    }
    if identity.publisher_trust_tier_class == "quarantined" {
        reasons.push("trust_tier_quarantined".to_string());
    }
    if !identity.lifecycle_runnable() {
        reasons.push("lifecycle_not_runnable".to_string());
    }
    if contributions
        .iter()
        .any(|c| matches!(c.host_state_class.as_str(), "quarantined" | "failed"))
    {
        reasons.push("contribution_quarantined".to_string());
    }
    if contributions
        .iter()
        .any(|c| c.host_state_class == "downgraded_narrower_profile")
    {
        reasons.push("contribution_host_downgraded".to_string());
    }
    match activation_budget.budget_class.as_str() {
        "over_budget_throttled" => reasons.push("activation_cost_over_budget".to_string()),
        "unbounded_refused" => reasons.push("activation_cost_unbounded".to_string()),
        _ => {}
    }
    if !attribution_complete {
        reasons.push("attribution_incomplete".to_string());
    }
    if let Some(dp) = data_plane {
        if dp.uses_ambient_auth() {
            reasons.push("ambient_auth_source".to_string());
        }
        if dp.unbounded_export() {
            reasons.push("unbounded_result_export".to_string());
        }
        if dp.unguarded_control_plane() {
            reasons.push("unguarded_control_plane_mutation".to_string());
        }
    }
    if reconnect.connection_dirty() {
        reasons.push("connection_state_dirty".to_string());
    }
    if reconnect.reattach_review_pending() {
        reasons.push("reattach_review_pending".to_string());
    }

    reasons.sort();
    reasons.dedup();

    if reasons.is_empty() {
        DerivedTier {
            effective_tier: claimed_tier.to_string(),
            support_claim: support_claim_for(claimed_tier),
            downgraded: false,
            downgrade_reasons: Vec::new(),
        }
    } else {
        let effective = narrow_tier_for(&reasons);
        DerivedTier {
            effective_tier: effective.to_string(),
            support_claim: support_claim_for(effective),
            downgraded: true,
            downgrade_reasons: reasons,
        }
    }
}

/// Picks the effective tier given the active narrowing reasons.
fn narrow_tier_for(reasons: &[String]) -> &'static str {
    if reasons
        .iter()
        .any(|r| WITHDRAWN_CLASS_REASONS.contains(&r.as_str()))
    {
        "withdrawn"
    } else if reasons
        .iter()
        .any(|r| PREVIEW_CLASS_REASONS.contains(&r.as_str()))
    {
        "preview"
    } else {
        debug_assert!(reasons
            .iter()
            .all(|r| BETA_CLASS_REASONS.contains(&r.as_str())));
        "beta"
    }
}

/// Maps an effective tier to the support claim it may imply.
fn support_claim_for(tier: &str) -> String {
    match tier {
        "stable" => "stable_external_host_ready_claim",
        "beta" => "beta_external_host_partial_claim",
        "preview" => "preview_external_host_experimental_claim",
        "withdrawn" => "withdrawn_no_external_host_claim",
        _ => "preview_external_host_experimental_claim",
    }
    .to_string()
}

/// Returns true when the host posture is downgraded and a banner must be shown.
fn host_is_downgraded(
    sandbox: &ExternalHostSandboxBinding,
    identity: &ExternalHostContractIdentity,
    data_plane: Option<&ExternalHostDataPlaneContract>,
    reconnect: &ExternalHostReconnectReplaySafety,
    contributions: &[ExternalHostContributionEntry],
) -> bool {
    matches!(
        sandbox.enforcement_state_class.as_str(),
        "fail_closed_downgraded" | "unenforceable_refused"
    ) || identity.publisher_trust_tier_class == "quarantined"
        || reconnect.connection_dirty()
        || data_plane.is_some_and(|dp| {
            dp.uses_ambient_auth() || dp.unbounded_export() || dp.unguarded_control_plane()
        })
        || contributions.iter().any(|c| {
            matches!(
                c.host_state_class.as_str(),
                "quarantined" | "failed" | "downgraded_narrower_profile"
            )
        })
}

/// Picks the most-severe banner reason for a downgraded host posture.
fn banner_reason_for(
    sandbox: &ExternalHostSandboxBinding,
    identity: &ExternalHostContractIdentity,
    data_plane: Option<&ExternalHostDataPlaneContract>,
    reconnect: &ExternalHostReconnectReplaySafety,
    contributions: &[ExternalHostContributionEntry],
) -> Option<String> {
    if sandbox.enforcement_state_class == "unenforceable_refused" {
        return Some("sandbox_unenforceable".to_string());
    }
    if let Some(dp) = data_plane {
        if dp.unguarded_control_plane() {
            return Some("unguarded_control_plane_mutation".to_string());
        }
        if dp.unbounded_export() {
            return Some("unbounded_result_export".to_string());
        }
    }
    if sandbox.enforcement_state_class == "fail_closed_downgraded" {
        return Some("sandbox_fail_closed_downgraded".to_string());
    }
    if identity.publisher_trust_tier_class == "quarantined" {
        return Some("trust_tier_quarantined".to_string());
    }
    if reconnect.connection_dirty() {
        return Some("connection_state_dirty".to_string());
    }
    if data_plane.is_some_and(|dp| dp.uses_ambient_auth()) {
        return Some("ambient_auth_source".to_string());
    }
    if contributions
        .iter()
        .any(|c| matches!(c.host_state_class.as_str(), "quarantined" | "failed"))
    {
        return Some("contribution_quarantined".to_string());
    }
    if contributions
        .iter()
        .any(|c| c.host_state_class == "downgraded_narrower_profile")
    {
        return Some("contribution_host_downgraded".to_string());
    }
    None
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn identity_record(input: &ExternalHostContractIdentityInput) -> ExternalHostContractIdentity {
    ExternalHostContractIdentity {
        record_kind: EXTERNAL_HOST_IDENTITY_RECORD_KIND.to_string(),
        schema_version: STABLE_EXTERNAL_HOST_SCHEMA_VERSION,
        runtime_contract_ref: input.runtime_contract_ref.clone(),
        extension_identity_ref: input.extension_identity_ref.clone(),
        extension_version: input.extension_version.clone(),
        source_package_ref: input.source_package_ref.clone(),
        abi_contract_version: input.abi_contract_version,
        publisher_trust_tier_class: input.publisher_trust_tier_class.clone(),
        lifecycle_state_class: input.lifecycle_state_class.clone(),
    }
}

fn host_kind_record(input: &ExternalHostKindDeclarationInput) -> ExternalHostKindDeclaration {
    ExternalHostKindDeclaration {
        record_kind: EXTERNAL_HOST_KIND_DECLARATION_RECORD_KIND.to_string(),
        schema_version: STABLE_EXTERNAL_HOST_SCHEMA_VERSION,
        host_kind_class: input.host_kind_class.clone(),
        execution_locus_class: input.execution_locus_class.clone(),
        protocol_class: input.protocol_class.clone(),
    }
}

fn sandbox_record(input: &ExternalHostSandboxBindingInput) -> ExternalHostSandboxBinding {
    ExternalHostSandboxBinding {
        record_kind: EXTERNAL_HOST_SANDBOX_BINDING_RECORD_KIND.to_string(),
        schema_version: STABLE_EXTERNAL_HOST_SCHEMA_VERSION,
        sandbox_profile_id: input.sandbox_profile_id.clone(),
        backend_classification_class: input.backend_classification_class.clone(),
        enforcement_state_class: input.enforcement_state_class.clone(),
        platform_backend_label: input.platform_backend_label.clone(),
        widens_to_ambient_full_user: input.widens_to_ambient_full_user,
    }
}

fn envelope_record(input: &ExternalHostCapabilityEnvelopeInput) -> ExternalHostCapabilityEnvelope {
    ExternalHostCapabilityEnvelope {
        record_kind: EXTERNAL_HOST_CAPABILITY_ENVELOPE_RECORD_KIND.to_string(),
        schema_version: STABLE_EXTERNAL_HOST_SCHEMA_VERSION,
        declared_capability_refs: input.declared_capability_refs.clone(),
        negotiated_capability_refs: input.negotiated_capability_refs.clone(),
        granted_capability_refs: input.granted_capability_refs.clone(),
        narrowing_reasons_recorded: input.narrowing_reasons_recorded,
    }
}

fn data_plane_record(input: &ExternalHostDataPlaneContractInput) -> ExternalHostDataPlaneContract {
    ExternalHostDataPlaneContract {
        record_kind: EXTERNAL_HOST_DATA_PLANE_CONTRACT_RECORD_KIND.to_string(),
        schema_version: STABLE_EXTERNAL_HOST_SCHEMA_VERSION,
        connection_target_class: input.connection_target_class.clone(),
        auth_source_mode_class: input.auth_source_mode_class.clone(),
        write_posture_class: input.write_posture_class.clone(),
        origin_class: input.origin_class.clone(),
        result_export_safety_class: input.result_export_safety_class.clone(),
        control_plane_boundary_class: input.control_plane_boundary_class.clone(),
        target_descriptor_ref: input.target_descriptor_ref.clone(),
    }
}

fn reconnect_record(
    input: &ExternalHostReconnectReplaySafetyInput,
) -> ExternalHostReconnectReplaySafety {
    ExternalHostReconnectReplaySafety {
        record_kind: EXTERNAL_HOST_RECONNECT_REPLAY_SAFETY_RECORD_KIND.to_string(),
        schema_version: STABLE_EXTERNAL_HOST_SCHEMA_VERSION,
        connection_state_class: input.connection_state_class.clone(),
        restart_posture_class: input.restart_posture_class.clone(),
        quarantine_active: input.quarantine_active,
        pending_reattach_side_effect_class: input.pending_reattach_side_effect_class.clone(),
        reattach_policy_class: input.reattach_policy_class.clone(),
        silently_reruns_side_effects: input.silently_reruns_side_effects,
    }
}

fn activation_budget_record(
    input: &ExternalHostActivationBudgetInput,
) -> ExternalHostActivationBudget {
    ExternalHostActivationBudget {
        record_kind: EXTERNAL_HOST_ACTIVATION_BUDGET_RECORD_KIND.to_string(),
        schema_version: STABLE_EXTERNAL_HOST_SCHEMA_VERSION,
        budget_class: input.budget_class.clone(),
        declared_cost_ref: input.declared_cost_ref.clone(),
        observed_cost_ref: input.observed_cost_ref.clone(),
    }
}

fn contribution_record(
    input: &ExternalHostContributionEntryInput,
) -> ExternalHostContributionEntry {
    ExternalHostContributionEntry {
        record_kind: EXTERNAL_HOST_CONTRIBUTION_ENTRY_RECORD_KIND.to_string(),
        schema_version: STABLE_EXTERNAL_HOST_SCHEMA_VERSION,
        contribution_id: input.contribution_id.clone(),
        contribution_kind_class: input.contribution_kind_class.clone(),
        source_package_ref: input.source_package_ref.clone(),
        trust_tier_class: input.trust_tier_class.clone(),
        used_permission_refs: input.used_permission_refs.clone(),
        last_known_good_host_ref: input.last_known_good_host_ref.clone(),
        host_state_class: input.host_state_class.clone(),
        summary_label: input.summary_label.clone(),
    }
}

#[allow(clippy::too_many_arguments)]
fn claim_record(
    input: &ExternalHostQualificationClaimInput,
    identity: &ExternalHostContractIdentity,
    sandbox: &ExternalHostSandboxBinding,
    data_plane: Option<&ExternalHostDataPlaneContract>,
    reconnect: &ExternalHostReconnectReplaySafety,
    activation_budget: &ExternalHostActivationBudget,
    contributions: &[ExternalHostContributionEntry],
) -> ExternalHostQualificationClaim {
    let attribution_complete = !identity.source_package_ref.trim().is_empty()
        && contributions.iter().all(|c| c.is_attributed());
    let derived = derive_effective_tier(
        &input.claimed_tier,
        &input.claim_basis_class,
        identity,
        sandbox,
        data_plane,
        reconnect,
        activation_budget,
        contributions,
        attribution_complete,
    );
    ExternalHostQualificationClaim {
        record_kind: EXTERNAL_HOST_QUALIFICATION_CLAIM_RECORD_KIND.to_string(),
        schema_version: STABLE_EXTERNAL_HOST_SCHEMA_VERSION,
        claimed_tier: input.claimed_tier.clone(),
        effective_tier: derived.effective_tier,
        support_claim_class: derived.support_claim,
        claim_basis_class: input.claim_basis_class.clone(),
        downgraded: derived.downgraded,
        downgrade_reasons: derived.downgrade_reasons,
        summary_label: input.summary_label.clone(),
    }
}

fn banner_record(
    sandbox: &ExternalHostSandboxBinding,
    identity: &ExternalHostContractIdentity,
    data_plane: Option<&ExternalHostDataPlaneContract>,
    reconnect: &ExternalHostReconnectReplaySafety,
    contributions: &[ExternalHostContributionEntry],
) -> ExternalHostDowngradedBanner {
    let must_display = host_is_downgraded(sandbox, identity, data_plane, reconnect, contributions);
    let banner_reason_class = if must_display {
        banner_reason_for(sandbox, identity, data_plane, reconnect, contributions)
    } else {
        None
    };
    let last_known_good_host_ref = if must_display {
        contributions
            .iter()
            .find(|c| !c.last_known_good_host_ref.trim().is_empty())
            .map(|c| c.last_known_good_host_ref.clone())
    } else {
        None
    };
    let summary_label = if must_display {
        format!(
            "External host running on a narrower-than-published posture ({}).",
            banner_reason_class.clone().unwrap_or_default()
        )
    } else {
        "External host enforcing the published contract.".to_string()
    };
    ExternalHostDowngradedBanner {
        record_kind: EXTERNAL_HOST_DOWNGRADED_BANNER_RECORD_KIND.to_string(),
        schema_version: STABLE_EXTERNAL_HOST_SCHEMA_VERSION,
        must_display,
        banner_reason_class,
        last_known_good_host_ref,
        summary_label,
    }
}

#[allow(clippy::too_many_arguments)]
fn inspection_record(
    packet_id: &str,
    host_kind: &ExternalHostKindDeclaration,
    identity: &ExternalHostContractIdentity,
    sandbox: &ExternalHostSandboxBinding,
    envelope: &ExternalHostCapabilityEnvelope,
    data_plane: Option<&ExternalHostDataPlaneContract>,
    reconnect: &ExternalHostReconnectReplaySafety,
    activation_budget: &ExternalHostActivationBudget,
    contributions: &[ExternalHostContributionEntry],
    claim: &ExternalHostQualificationClaim,
    banner: &ExternalHostDowngradedBanner,
) -> ExternalHostContractInspection {
    let stable_claim = STABLE_TIERS.contains(&claim.effective_tier.as_str());
    let attribution_complete = !identity.source_package_ref.trim().is_empty()
        && contributions.iter().all(|c| c.is_attributed());
    let quarantined_contribution_count = contributions
        .iter()
        .filter(|c| matches!(c.host_state_class.as_str(), "quarantined" | "failed"))
        .count();
    let downgraded_contribution_count = contributions
        .iter()
        .filter(|c| c.host_state_class == "downgraded_narrower_profile")
        .count();

    ExternalHostContractInspection {
        record_kind: EXTERNAL_HOST_INSPECTION_RECORD_KIND.to_string(),
        schema_version: STABLE_EXTERNAL_HOST_SCHEMA_VERSION,
        packet_id_ref: packet_id.to_string(),
        host_kind_class: host_kind.host_kind_class.clone(),
        protocol_class: host_kind.protocol_class.clone(),
        execution_locus_class: host_kind.execution_locus_class.clone(),
        effective_tier: claim.effective_tier.clone(),
        stable_claim,
        abi_version_current: identity.abi_version_current(),
        sandbox_enforced_as_published: sandbox.enforced_as_published(),
        widens_to_ambient_full_user: sandbox.widens_to_ambient_full_user,
        capability_envelope_well_formed: envelope.is_well_formed(),
        trust_tier_class: identity.publisher_trust_tier_class.clone(),
        lifecycle_runnable: identity.lifecycle_runnable(),
        data_plane_contract_present: data_plane.is_some(),
        write_posture_class: data_plane.map(|d| d.write_posture_class.clone()),
        auth_source_mode_class: data_plane.map(|d| d.auth_source_mode_class.clone()),
        control_plane_boundary_class: data_plane.map(|d| d.control_plane_boundary_class.clone()),
        origin_class: data_plane.map(|d| d.origin_class.clone()),
        result_export_safety_class: data_plane.map(|d| d.result_export_safety_class.clone()),
        connection_state_class: reconnect.connection_state_class.clone(),
        reattach_policy_class: reconnect.reattach_policy_class.clone(),
        pending_reattach_side_effect_class: reconnect.pending_reattach_side_effect_class.clone(),
        silently_reruns_side_effects: reconnect.silently_reruns_side_effects,
        reattach_review_pending: reconnect.reattach_review_pending(),
        downgraded: claim.downgraded,
        downgraded_host_banner_required: banner.must_display,
        attribution_complete,
        active_contribution_count: contributions.len(),
        quarantined_contribution_count,
        downgraded_contribution_count,
        declared_capability_count: envelope.declared_capability_refs.len(),
        negotiated_capability_count: envelope.negotiated_capability_refs.len(),
        granted_capability_count: envelope.granted_capability_refs.len(),
        activation_budget_class: activation_budget.budget_class.clone(),
        summary_label: claim.summary_label.clone(),
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &StableExternalHostContractInput,
) -> Result<(), StableExternalHostValidationError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    let id = &input.identity;
    ensure_nonempty(&id.runtime_contract_ref, "identity.runtime_contract_ref")?;
    if !id.runtime_contract_ref.starts_with("runtime_v1_beta:") {
        return Err(err(
            "identity.runtime_contract_ref must start with 'runtime_v1_beta:'",
        ));
    }
    ensure_nonempty(
        &id.extension_identity_ref,
        "identity.extension_identity_ref",
    )?;
    ensure_nonempty(&id.extension_version, "identity.extension_version")?;
    ensure_nonempty(&id.source_package_ref, "identity.source_package_ref")?;
    ensure_token(
        TRUST_TIER_CLASSES,
        &id.publisher_trust_tier_class,
        "identity.publisher_trust_tier_class",
    )?;
    ensure_token(
        LIFECYCLE_STATE_CLASSES,
        &id.lifecycle_state_class,
        "identity.lifecycle_state_class",
    )?;

    let hk = &input.host_kind_declaration;
    ensure_token(
        EXTERNAL_HOST_KIND_CLASSES,
        &hk.host_kind_class,
        "host_kind_class",
    )?;
    ensure_token(
        EXTERNAL_HOST_EXECUTION_LOCUS_CLASSES,
        &hk.execution_locus_class,
        "execution_locus_class",
    )?;
    ensure_token(HOST_PROTOCOL_CLASSES, &hk.protocol_class, "protocol_class")?;
    if !host_kind_supports_locus(&hk.host_kind_class, &hk.execution_locus_class) {
        return Err(err(format!(
            "execution_locus_class {} is not reserved for host_kind_class {}",
            hk.execution_locus_class, hk.host_kind_class
        )));
    }
    if !host_kind_supports_protocol(&hk.host_kind_class, &hk.protocol_class) {
        return Err(err(format!(
            "protocol_class {} is not reserved for host_kind_class {}",
            hk.protocol_class, hk.host_kind_class
        )));
    }

    let sb = &input.sandbox_binding;
    ensure_nonempty(&sb.sandbox_profile_id, "sandbox_binding.sandbox_profile_id")?;
    ensure_token(
        EXTERNAL_HOST_BACKEND_CLASSES,
        &sb.backend_classification_class,
        "sandbox_binding.backend_classification_class",
    )?;
    ensure_token(
        SANDBOX_ENFORCEMENT_STATES,
        &sb.enforcement_state_class,
        "sandbox_binding.enforcement_state_class",
    )?;
    ensure_nonempty(
        &sb.platform_backend_label,
        "sandbox_binding.platform_backend_label",
    )?;
    if !locus_supports_backend(&hk.execution_locus_class, &sb.backend_classification_class) {
        return Err(err(format!(
            "backend_classification_class {} is not reserved for execution_locus_class {}",
            sb.backend_classification_class, hk.execution_locus_class
        )));
    }
    // Hard security guardrail: an external host can never widen to ambient
    // full-user execution.
    if sb.widens_to_ambient_full_user {
        return Err(err(
            "an external host must not widen to ambient full-user execution",
        ));
    }

    let env = &input.capability_envelope;
    if !is_subset(
        &env.negotiated_capability_refs,
        &env.declared_capability_refs,
    ) {
        return Err(err(
            "negotiated_capability_refs must be a subset of declared_capability_refs",
        ));
    }
    if !is_subset(
        &env.granted_capability_refs,
        &env.negotiated_capability_refs,
    ) {
        return Err(err(
            "granted_capability_refs must be a subset of negotiated_capability_refs",
        ));
    }

    // A data-plane host kind must carry a typed data-plane contract; any other
    // host kind must not.
    let requires_dp = DATA_PLANE_HOST_KINDS.contains(&hk.host_kind_class.as_str());
    match (&input.data_plane_contract, requires_dp) {
        (Some(_), false) => {
            return Err(err(
                "only database / infra adapters may carry a data-plane contract",
            ));
        }
        (None, true) => {
            return Err(err(
                "a database / infra adapter must carry a typed data-plane contract",
            ));
        }
        _ => {}
    }
    if let Some(dp) = &input.data_plane_contract {
        ensure_token(
            CONNECTION_TARGET_CLASSES,
            &dp.connection_target_class,
            "data_plane_contract.connection_target_class",
        )?;
        ensure_token(
            AUTH_SOURCE_MODE_CLASSES,
            &dp.auth_source_mode_class,
            "data_plane_contract.auth_source_mode_class",
        )?;
        ensure_token(
            WRITE_POSTURE_CLASSES,
            &dp.write_posture_class,
            "data_plane_contract.write_posture_class",
        )?;
        ensure_token(
            ADAPTER_ORIGIN_CLASSES,
            &dp.origin_class,
            "data_plane_contract.origin_class",
        )?;
        ensure_token(
            RESULT_EXPORT_SAFETY_CLASSES,
            &dp.result_export_safety_class,
            "data_plane_contract.result_export_safety_class",
        )?;
        ensure_token(
            CONTROL_PLANE_BOUNDARY_CLASSES,
            &dp.control_plane_boundary_class,
            "data_plane_contract.control_plane_boundary_class",
        )?;
        ensure_nonempty(
            &dp.target_descriptor_ref,
            "data_plane_contract.target_descriptor_ref",
        )?;
        // A read-only adapter cannot host a mutating control plane: that would be a
        // self-contradicting contract, not just a narrowing.
        if dp.write_posture_class == "read_only"
            && matches!(
                dp.control_plane_boundary_class.as_str(),
                "gated_apply_with_review" | "unguarded_mutating"
            )
        {
            return Err(err(
                "a read_only adapter must not declare a mutating control-plane boundary",
            ));
        }
    }

    let rc = &input.reconnect_replay_safety;
    ensure_token(
        CONNECTION_STATE_CLASSES,
        &rc.connection_state_class,
        "reconnect_replay_safety.connection_state_class",
    )?;
    ensure_token(
        RESTART_POSTURE_CLASSES,
        &rc.restart_posture_class,
        "reconnect_replay_safety.restart_posture_class",
    )?;
    ensure_token(
        PENDING_SIDE_EFFECT_CLASSES,
        &rc.pending_reattach_side_effect_class,
        "reconnect_replay_safety.pending_reattach_side_effect_class",
    )?;
    ensure_token(
        REATTACH_POLICY_CLASSES,
        &rc.reattach_policy_class,
        "reconnect_replay_safety.reattach_policy_class",
    )?;
    // Hard guardrail: never silently re-run side effects after a restart.
    if rc.silently_reruns_side_effects {
        return Err(err(
            "an external host must never silently re-run a query, apply-capable action, or control-plane mutation after restart",
        ));
    }
    // Hard guardrail: a reattach with possible side effects must require an
    // explicit replay or review path, not a stateless safe resume.
    if SIDE_EFFECTING_PENDING_CLASSES.contains(&rc.pending_reattach_side_effect_class.as_str())
        && rc.reattach_policy_class == "stateless_safe_resume"
    {
        return Err(err(
            "a reattach with possible side effects must require an explicit replay or review path, not a stateless safe resume",
        ));
    }

    let ab = &input.activation_budget;
    ensure_token(
        ACTIVATION_BUDGET_CLASSES,
        &ab.budget_class,
        "activation_budget.budget_class",
    )?;
    ensure_nonempty(&ab.declared_cost_ref, "activation_budget.declared_cost_ref")?;
    ensure_nonempty(&ab.observed_cost_ref, "activation_budget.observed_cost_ref")?;

    let mut contribution_ids = BTreeSet::new();
    for c in &input.contributions {
        ensure_nonempty(&c.contribution_id, "contribution.contribution_id")?;
        if !contribution_ids.insert(&c.contribution_id) {
            return Err(err(format!(
                "duplicate contribution_id: {}",
                c.contribution_id
            )));
        }
        ensure_token(
            CONTRIBUTION_KIND_CLASSES,
            &c.contribution_kind_class,
            "contribution.contribution_kind_class",
        )?;
        ensure_nonempty(&c.source_package_ref, "contribution.source_package_ref")?;
        ensure_token(
            TRUST_TIER_CLASSES,
            &c.trust_tier_class,
            "contribution.trust_tier_class",
        )?;
        ensure_nonempty(
            &c.last_known_good_host_ref,
            "contribution.last_known_good_host_ref",
        )?;
        ensure_token(
            CONTRIBUTION_HOST_STATE_CLASSES,
            &c.host_state_class,
            "contribution.host_state_class",
        )?;
    }

    let claim = &input.claim;
    ensure_token(STABILITY_TIERS, &claim.claimed_tier, "claim.claimed_tier")?;
    ensure_token(
        CLAIM_BASIS_CLASSES,
        &claim.claim_basis_class,
        "claim.claim_basis_class",
    )?;

    for surface in &input.consumer_surfaces {
        ensure_token(
            STABLE_EXTERNAL_HOST_CONSUMER_SURFACES,
            surface,
            "consumer_surface",
        )?;
    }
    if input.consumer_surfaces.is_empty() {
        return Err(err("input must bind at least one consumer surface"));
    }

    Ok(())
}

fn validate_identity(
    identity: &ExternalHostContractIdentity,
) -> Result<(), StableExternalHostValidationError> {
    ensure_eq(
        identity.record_kind.as_str(),
        EXTERNAL_HOST_IDENTITY_RECORD_KIND,
        "identity record_kind",
    )?;
    ensure_eq_u32(
        identity.schema_version,
        STABLE_EXTERNAL_HOST_SCHEMA_VERSION,
        "identity schema_version",
    )?;
    ensure_token(
        TRUST_TIER_CLASSES,
        &identity.publisher_trust_tier_class,
        "identity publisher_trust_tier_class",
    )?;
    ensure_token(
        LIFECYCLE_STATE_CLASSES,
        &identity.lifecycle_state_class,
        "identity lifecycle_state_class",
    )?;
    Ok(())
}

fn validate_host_kind(
    hk: &ExternalHostKindDeclaration,
) -> Result<(), StableExternalHostValidationError> {
    ensure_eq(
        hk.record_kind.as_str(),
        EXTERNAL_HOST_KIND_DECLARATION_RECORD_KIND,
        "host_kind record_kind",
    )?;
    ensure_token(
        EXTERNAL_HOST_KIND_CLASSES,
        &hk.host_kind_class,
        "host_kind_class",
    )?;
    ensure_token(
        EXTERNAL_HOST_EXECUTION_LOCUS_CLASSES,
        &hk.execution_locus_class,
        "execution_locus_class",
    )?;
    ensure_token(HOST_PROTOCOL_CLASSES, &hk.protocol_class, "protocol_class")?;
    if !host_kind_supports_locus(&hk.host_kind_class, &hk.execution_locus_class) {
        return Err(err(
            "execution_locus_class is not reserved for host_kind_class",
        ));
    }
    if !host_kind_supports_protocol(&hk.host_kind_class, &hk.protocol_class) {
        return Err(err("protocol_class is not reserved for host_kind_class"));
    }
    Ok(())
}

fn validate_sandbox(
    sb: &ExternalHostSandboxBinding,
) -> Result<(), StableExternalHostValidationError> {
    ensure_eq(
        sb.record_kind.as_str(),
        EXTERNAL_HOST_SANDBOX_BINDING_RECORD_KIND,
        "sandbox record_kind",
    )?;
    ensure_token(
        EXTERNAL_HOST_BACKEND_CLASSES,
        &sb.backend_classification_class,
        "sandbox backend_classification_class",
    )?;
    ensure_token(
        SANDBOX_ENFORCEMENT_STATES,
        &sb.enforcement_state_class,
        "sandbox enforcement_state_class",
    )?;
    if sb.widens_to_ambient_full_user {
        return Err(err(
            "an external host must not widen to ambient full-user execution",
        ));
    }
    Ok(())
}

fn validate_envelope(
    env: &ExternalHostCapabilityEnvelope,
) -> Result<(), StableExternalHostValidationError> {
    ensure_eq(
        env.record_kind.as_str(),
        EXTERNAL_HOST_CAPABILITY_ENVELOPE_RECORD_KIND,
        "envelope record_kind",
    )?;
    if !env.is_well_formed() {
        return Err(err(
            "capability envelope must keep granted ⊆ negotiated ⊆ declared",
        ));
    }
    if env.narrowing_diff_inconsistent() {
        return Err(err(
            "capabilities were narrowed but narrowing_reasons_recorded is false",
        ));
    }
    Ok(())
}

fn validate_data_plane_presence(
    hk: &ExternalHostKindDeclaration,
    dp: Option<&ExternalHostDataPlaneContract>,
) -> Result<(), StableExternalHostValidationError> {
    match (dp, hk.requires_data_plane_contract()) {
        (Some(_), false) => Err(err(
            "only database / infra adapters may carry a data-plane contract",
        )),
        (None, true) => Err(err(
            "a database / infra adapter must carry a typed data-plane contract",
        )),
        _ => Ok(()),
    }
}

fn validate_data_plane(
    dp: &ExternalHostDataPlaneContract,
) -> Result<(), StableExternalHostValidationError> {
    ensure_eq(
        dp.record_kind.as_str(),
        EXTERNAL_HOST_DATA_PLANE_CONTRACT_RECORD_KIND,
        "data_plane record_kind",
    )?;
    ensure_token(
        CONNECTION_TARGET_CLASSES,
        &dp.connection_target_class,
        "data_plane connection_target_class",
    )?;
    ensure_token(
        AUTH_SOURCE_MODE_CLASSES,
        &dp.auth_source_mode_class,
        "data_plane auth_source_mode_class",
    )?;
    ensure_token(
        WRITE_POSTURE_CLASSES,
        &dp.write_posture_class,
        "data_plane write_posture_class",
    )?;
    ensure_token(
        ADAPTER_ORIGIN_CLASSES,
        &dp.origin_class,
        "data_plane origin_class",
    )?;
    ensure_token(
        RESULT_EXPORT_SAFETY_CLASSES,
        &dp.result_export_safety_class,
        "data_plane result_export_safety_class",
    )?;
    ensure_token(
        CONTROL_PLANE_BOUNDARY_CLASSES,
        &dp.control_plane_boundary_class,
        "data_plane control_plane_boundary_class",
    )?;
    ensure_nonempty(
        &dp.target_descriptor_ref,
        "data_plane target_descriptor_ref",
    )?;
    if dp.write_posture_class == "read_only"
        && matches!(
            dp.control_plane_boundary_class.as_str(),
            "gated_apply_with_review" | "unguarded_mutating"
        )
    {
        return Err(err(
            "a read_only adapter must not declare a mutating control-plane boundary",
        ));
    }
    Ok(())
}

fn validate_reconnect(
    rc: &ExternalHostReconnectReplaySafety,
) -> Result<(), StableExternalHostValidationError> {
    ensure_eq(
        rc.record_kind.as_str(),
        EXTERNAL_HOST_RECONNECT_REPLAY_SAFETY_RECORD_KIND,
        "reconnect record_kind",
    )?;
    ensure_token(
        CONNECTION_STATE_CLASSES,
        &rc.connection_state_class,
        "reconnect connection_state_class",
    )?;
    ensure_token(
        RESTART_POSTURE_CLASSES,
        &rc.restart_posture_class,
        "reconnect restart_posture_class",
    )?;
    ensure_token(
        PENDING_SIDE_EFFECT_CLASSES,
        &rc.pending_reattach_side_effect_class,
        "reconnect pending_reattach_side_effect_class",
    )?;
    ensure_token(
        REATTACH_POLICY_CLASSES,
        &rc.reattach_policy_class,
        "reconnect reattach_policy_class",
    )?;
    Ok(())
}

fn validate_activation_budget(
    ab: &ExternalHostActivationBudget,
) -> Result<(), StableExternalHostValidationError> {
    ensure_eq(
        ab.record_kind.as_str(),
        EXTERNAL_HOST_ACTIVATION_BUDGET_RECORD_KIND,
        "activation_budget record_kind",
    )?;
    ensure_token(
        ACTIVATION_BUDGET_CLASSES,
        &ab.budget_class,
        "activation_budget budget_class",
    )?;
    Ok(())
}

fn validate_contribution(
    c: &ExternalHostContributionEntry,
) -> Result<(), StableExternalHostValidationError> {
    ensure_eq(
        c.record_kind.as_str(),
        EXTERNAL_HOST_CONTRIBUTION_ENTRY_RECORD_KIND,
        "contribution record_kind",
    )?;
    ensure_token(
        CONTRIBUTION_KIND_CLASSES,
        &c.contribution_kind_class,
        "contribution contribution_kind_class",
    )?;
    ensure_token(
        TRUST_TIER_CLASSES,
        &c.trust_tier_class,
        "contribution trust_tier_class",
    )?;
    ensure_token(
        CONTRIBUTION_HOST_STATE_CLASSES,
        &c.host_state_class,
        "contribution host_state_class",
    )?;
    // The inspector must stay attributable even when quarantined / failed / downgraded.
    if !c.is_attributed() {
        return Err(err(
            "contribution inspector entry must keep source package, id, and last-known-good host",
        ));
    }
    Ok(())
}

fn validate_claim(
    claim: &ExternalHostQualificationClaim,
) -> Result<(), StableExternalHostValidationError> {
    ensure_eq(
        claim.record_kind.as_str(),
        EXTERNAL_HOST_QUALIFICATION_CLAIM_RECORD_KIND,
        "claim record_kind",
    )?;
    ensure_token(STABILITY_TIERS, &claim.claimed_tier, "claim claimed_tier")?;
    ensure_token(
        STABILITY_TIERS,
        &claim.effective_tier,
        "claim effective_tier",
    )?;
    ensure_token(
        SUPPORT_CLAIM_CLASSES,
        &claim.support_claim_class,
        "claim support_claim_class",
    )?;
    ensure_token(
        CLAIM_BASIS_CLASSES,
        &claim.claim_basis_class,
        "claim claim_basis_class",
    )?;
    for reason in &claim.downgrade_reasons {
        ensure_token(
            EXTERNAL_HOST_DOWNGRADE_REASONS,
            reason,
            "claim downgrade_reason",
        )?;
    }
    Ok(())
}

fn validate_banner(
    banner: &ExternalHostDowngradedBanner,
) -> Result<(), StableExternalHostValidationError> {
    ensure_eq(
        banner.record_kind.as_str(),
        EXTERNAL_HOST_DOWNGRADED_BANNER_RECORD_KIND,
        "banner record_kind",
    )?;
    if let Some(reason) = &banner.banner_reason_class {
        ensure_token(
            EXTERNAL_HOST_DOWNGRADE_REASONS,
            reason,
            "banner banner_reason_class",
        )?;
        if !banner.must_display {
            return Err(err("banner_reason_class is set but must_display is false"));
        }
    } else if banner.must_display {
        return Err(err(
            "must_display is true but no banner_reason_class is set",
        ));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &ExternalHostContractInspection,
    packet: &StableExternalHostContractPacket,
) -> Result<(), StableExternalHostValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        EXTERNAL_HOST_INSPECTION_RECORD_KIND,
        "inspection record_kind",
    )?;
    ensure_eq(
        inspection.packet_id_ref.as_str(),
        packet.packet_id.as_str(),
        "inspection packet_id_ref",
    )?;
    ensure_eq(
        inspection.effective_tier.as_str(),
        packet.claim.effective_tier.as_str(),
        "inspection effective_tier",
    )?;
    if inspection.active_contribution_count != packet.contributions.len() {
        return Err(err(
            "inspection active_contribution_count must match contributions",
        ));
    }
    if inspection.downgraded != packet.claim.downgraded {
        return Err(err("inspection downgraded is inconsistent"));
    }
    if inspection.downgraded_host_banner_required != packet.downgraded_host_banner.must_display {
        return Err(err(
            "inspection downgraded_host_banner_required is inconsistent",
        ));
    }
    if inspection.attribution_complete != packet.attribution_complete() {
        return Err(err("inspection attribution_complete is inconsistent"));
    }
    if inspection.widens_to_ambient_full_user != packet.sandbox_binding.widens_to_ambient_full_user
    {
        return Err(err(
            "inspection widens_to_ambient_full_user is inconsistent",
        ));
    }
    if inspection.silently_reruns_side_effects
        != packet.reconnect_replay_safety.silently_reruns_side_effects
    {
        return Err(err(
            "inspection silently_reruns_side_effects is inconsistent",
        ));
    }
    if inspection.reattach_review_pending
        != packet.reconnect_replay_safety.reattach_review_pending()
    {
        return Err(err("inspection reattach_review_pending is inconsistent"));
    }
    if inspection.data_plane_contract_present != packet.data_plane_contract.is_some() {
        return Err(err(
            "inspection data_plane_contract_present is inconsistent",
        ));
    }
    if inspection.capability_envelope_well_formed != packet.capability_envelope.is_well_formed() {
        return Err(err(
            "inspection capability_envelope_well_formed is inconsistent",
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Mapping tables
// ---------------------------------------------------------------------------

fn host_kind_supports_locus(host_kind: &str, locus: &str) -> bool {
    match host_kind {
        "language_tool" | "debug_adapter" => {
            matches!(locus, "dedicated_subprocess" | "helper_binary")
        }
        "cli_tool" => matches!(locus, "helper_binary" | "dedicated_subprocess"),
        "database_adapter" | "infra_adapter" => matches!(
            locus,
            "dedicated_subprocess" | "remote_agent" | "managed_service_endpoint"
        ),
        _ => false,
    }
}

fn host_kind_supports_protocol(host_kind: &str, protocol: &str) -> bool {
    match host_kind {
        "language_tool" => matches!(
            protocol,
            "language_server_protocol" | "custom_host_protocol"
        ),
        "debug_adapter" => matches!(protocol, "debug_adapter_protocol" | "custom_host_protocol"),
        "cli_tool" => matches!(protocol, "cli_invocation" | "custom_host_protocol"),
        "database_adapter" => matches!(protocol, "database_wire_protocol" | "custom_host_protocol"),
        "infra_adapter" => matches!(protocol, "infra_control_api" | "custom_host_protocol"),
        _ => false,
    }
}

fn locus_supports_backend(locus: &str, backend: &str) -> bool {
    match locus {
        "dedicated_subprocess" | "helper_binary" => matches!(
            backend,
            "os_process_sandbox"
                | "seatbelt_sandbox_profile"
                | "landlock_seccomp_profile"
                | "app_container_profile"
        ),
        "remote_agent" | "managed_service_endpoint" => backend == "remote_enforced_envelope",
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn err(message: impl Into<String>) -> StableExternalHostValidationError {
    StableExternalHostValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(left: T, right: T, field: &str) -> Result<(), StableExternalHostValidationError>
where
    T: PartialEq + fmt::Display,
{
    if left != right {
        return Err(err(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_eq_u32(
    left: u32,
    right: u32,
    field: &str,
) -> Result<(), StableExternalHostValidationError> {
    if left != right {
        return Err(err(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), StableExternalHostValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), StableExternalHostValidationError> {
    if !tokens.contains(&value) {
        return Err(err(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
    }
    Ok(())
}

fn is_subset(subset: &[String], superset: &[String]) -> bool {
    subset.iter().all(|item| superset.contains(item))
}
