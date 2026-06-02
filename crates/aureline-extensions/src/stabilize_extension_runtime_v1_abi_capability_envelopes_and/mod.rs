//! Stabilize the extension runtime v1 ABI, capability envelopes, and host
//! isolation for the stable line.
//!
//! The runtime v1 *beta* admission contract (see [`crate::runtime`]) owns the
//! per-session admission decision: did the host negotiate the declared capability
//! worlds, is the publisher identity attributable, is the lifecycle runnable. This
//! module owns the layer above it — the **stable, published runtime ABI truth** a
//! claimed stable ecosystem row carries, and the **stability qualification** that
//! truth is allowed to claim.
//!
//! The central rule mirrors the rest of the stable line: a **stable** runtime
//! claim may never be implied from a catalog row alone. A row that renders a
//! `stable` runtime badge must pin the published ABI contract version, resolve to
//! an enforced [`sandbox profile`](SandboxProfileBinding) (not a fail-closed
//! downgrade), carry a capability envelope that never widens
//! (`granted ⊆ negotiated ⊆ declared`), keep its publisher trust tier out of
//! quarantine, stay on a runnable lifecycle, keep every active contribution
//! nominal, hold a bounded activation cost, and be fully attributed. When any of
//! those fails, the visible tier is **automatically narrowed below Stable**
//! (to `beta` or `preview`, or `withdrawn` when the host cannot enforce a sandbox
//! at all) rather than left asserting a runtime readiness the host cannot back.
//!
//! Two security guardrails are encoded so they cannot be papered over:
//!
//! - **No silent ambient widening.** A claimed sandboxed runtime class
//!   ([`SANDBOXED_RUNTIME_CLASSES`]) whose binding sets
//!   `widens_to_ambient_full_user` is rejected at construction — a sandboxed
//!   path can never quietly resolve to ambient full-user execution.
//! - **Fail-closed downgrade is visible.** When a platform/backend cannot enforce
//!   the published profile, the enforcement state is `fail_closed_downgraded`
//!   (still sandboxed, but a narrower profile) or `unenforceable_refused` (no safe
//!   profile, the row is withdrawn). Both narrow the claim below Stable and raise
//!   a [`DowngradedHostBanner`].
//!
//! The published runtime-class vocabulary ([`RUNTIME_CLASSES`]) — `passive_package`,
//! `wasm_capability_sandbox`, `declarative_host_rendered_view`, `external_host`,
//! `compatibility_bridge`, `remote_side_component` — is carried on every record
//! and on every [`ActiveContributionInspectorEntry`], so install review, the
//! runtime inspector, the quarantine flow, diagnostics, and support exports name
//! the real execution model instead of collapsing everything into a generic
//! extension badge.
//!
//! The companion schema lives at
//! [`schemas/extensions/stable_runtime_abi.schema.json`](../../../../schemas/extensions/stable_runtime_abi.schema.json).
//! Canonical fixtures live under
//! `fixtures/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every stable runtime-ABI record.
pub const STABLE_RUNTIME_ABI_SCHEMA_VERSION: u32 = 1;

/// The published, stable runtime ABI contract version. A `stable` runtime claim
/// must pin exactly this version; any other version narrows below Stable.
pub const STABLE_RUNTIME_ABI_PUBLISHED_VERSION: u32 = 1;

/// Canonical schema path the packet cites.
pub const STABLE_RUNTIME_ABI_SCHEMA_REF: &str = "schemas/extensions/stable_runtime_abi.schema.json";

/// Record-kind tag for [`StableRuntimeAbiPacket`].
pub const STABLE_RUNTIME_ABI_PACKET_RECORD_KIND: &str = "stable_runtime_abi_packet";

/// Record-kind tag for [`RuntimeAbiIdentity`].
pub const RUNTIME_ABI_IDENTITY_RECORD_KIND: &str = "stable_runtime_abi_identity";

/// Record-kind tag for [`RuntimeClassDeclaration`].
pub const RUNTIME_CLASS_DECLARATION_RECORD_KIND: &str = "stable_runtime_class_declaration";

/// Record-kind tag for [`SandboxProfileBinding`].
pub const SANDBOX_PROFILE_BINDING_RECORD_KIND: &str = "stable_sandbox_profile_binding";

/// Record-kind tag for [`CapabilityEnvelope`].
pub const CAPABILITY_ENVELOPE_RECORD_KIND: &str = "stable_capability_envelope";

/// Record-kind tag for [`HostIsolationPosture`].
pub const HOST_ISOLATION_POSTURE_RECORD_KIND: &str = "stable_host_isolation_posture";

/// Record-kind tag for [`ActivationBudget`].
pub const ACTIVATION_BUDGET_RECORD_KIND: &str = "stable_runtime_activation_budget";

/// Record-kind tag for [`ActiveContributionInspectorEntry`].
pub const ACTIVE_CONTRIBUTION_INSPECTOR_ENTRY_RECORD_KIND: &str =
    "stable_active_contribution_inspector_entry";

/// Record-kind tag for [`RuntimeAbiQualificationClaim`].
pub const RUNTIME_ABI_QUALIFICATION_CLAIM_RECORD_KIND: &str =
    "stable_runtime_abi_qualification_claim";

/// Record-kind tag for [`DowngradedHostBanner`].
pub const DOWNGRADED_HOST_BANNER_RECORD_KIND: &str = "stable_downgraded_host_banner";

/// Record-kind tag for [`StableRuntimeAbiInspection`].
pub const STABLE_RUNTIME_ABI_INSPECTION_RECORD_KIND: &str = "stable_runtime_abi_inspection";

/// Record-kind tag for [`StableRuntimeAbiSupportExport`].
pub const STABLE_RUNTIME_ABI_SUPPORT_EXPORT_RECORD_KIND: &str = "stable_runtime_abi_support_export";

/// Published, controlled runtime-class vocabulary exposed anywhere a contributed
/// command, panel, provider, or background lane runs, fails, or downgrades.
pub const RUNTIME_CLASSES: &[&str] = &[
    "passive_package",
    "wasm_capability_sandbox",
    "declarative_host_rendered_view",
    "external_host",
    "compatibility_bridge",
    "remote_side_component",
];

/// Runtime classes that execute extension-authored code under a sandbox and so
/// must publish *and* enforce a sandbox profile and never widen to ambient
/// full-user execution.
pub const SANDBOXED_RUNTIME_CLASSES: &[&str] = &[
    "wasm_capability_sandbox",
    "external_host",
    "compatibility_bridge",
    "remote_side_component",
];

/// Runtime classes that run no extension-authored code locally; no sandbox
/// profile is required and they can never widen to ambient execution.
pub const NON_EXECUTING_RUNTIME_CLASSES: &[&str] =
    &["passive_package", "declarative_host_rendered_view"];

/// Closed execution-locus vocabulary describing where a contribution runs.
pub const EXECUTION_LOCUS_CLASSES: &[&str] = &[
    "editor_in_process_isolated",
    "dedicated_subprocess",
    "helper_binary",
    "remote_agent",
    "bridged_foreign_runtime",
    "host_rendered_no_extension_code",
    "passive_no_execution",
];

/// Closed backend-classification vocabulary naming the enforcement backend.
pub const BACKEND_CLASSIFICATION_CLASSES: &[&str] = &[
    "wasm_component_model",
    "wasm_core_module",
    "os_process_sandbox",
    "seatbelt_sandbox_profile",
    "landlock_seccomp_profile",
    "app_container_profile",
    "remote_enforced_envelope",
    "bridge_translated_profile",
    "none_passive",
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

/// Lifecycle states a stable runtime claim may keep.
pub const RUNNABLE_LIFECYCLE_STATES: &[&str] =
    &["installed", "pending_activation", "active", "recovered"];

/// Closed contribution-kind vocabulary.
pub const CONTRIBUTION_KIND_CLASSES: &[&str] =
    &["command", "panel", "provider", "background_lane", "view"];

/// Closed contribution-host-state vocabulary, inspectable even when a
/// contribution is quarantined, bridged, or running on a narrower profile.
pub const CONTRIBUTION_HOST_STATE_CLASSES: &[&str] = &[
    "running_nominal",
    "bridged",
    "downgraded_narrower_profile",
    "quarantined",
    "failed",
];

/// Closed isolation-boundary vocabulary.
pub const ISOLATION_BOUNDARY_CLASSES: &[&str] = &[
    "in_process_capability_world",
    "process_isolated",
    "remote_envelope",
    "bridge_translated",
    "passive_none",
];

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

/// Tiers that count as a *stable* runtime claim.
pub const STABLE_TIERS: &[&str] = &["stable"];

/// Closed claim-basis vocabulary. `catalog_asserted_only` may never back stable.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["enforcement_backed", "catalog_asserted_only"];

/// Closed support-claim vocabulary a tier may imply.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_runtime_ready_claim",
    "beta_runtime_partial_claim",
    "preview_runtime_experimental_claim",
    "withdrawn_no_runtime_claim",
];

/// Closed set of reasons that narrow a stable runtime claim below Stable.
pub const RUNTIME_ABI_DOWNGRADE_REASONS: &[&str] = &[
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
];

/// Reasons that narrow all the way to `withdrawn`.
const WITHDRAWN_CLASS_REASONS: &[&str] = &[
    "sandbox_unenforceable",
    "lifecycle_not_runnable",
    "activation_cost_unbounded",
];

/// Reasons that narrow to `preview` (any structural/security shortfall).
const PREVIEW_CLASS_REASONS: &[&str] = &[
    "abi_version_mismatch",
    "catalog_only_trust_not_enforcement_backed",
    "trust_tier_quarantined",
    "contribution_quarantined",
    "attribution_incomplete",
];

/// Reasons whose only effect is a safe narrowing to `beta`.
const BETA_CLASS_REASONS: &[&str] = &[
    "sandbox_fail_closed_downgraded",
    "contribution_host_downgraded",
    "activation_cost_over_budget",
];

/// Closed set of consumer surfaces that ingest this packet.
pub const STABLE_RUNTIME_ABI_CONSUMER_SURFACES: &[&str] = &[
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

/// Input describing a stable runtime-ABI packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableRuntimeAbiInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity input.
    pub identity: RuntimeAbiIdentityInput,
    /// Runtime-class declaration input.
    pub runtime_class_declaration: RuntimeClassDeclarationInput,
    /// Sandbox-profile binding input.
    pub sandbox_profile: SandboxProfileBindingInput,
    /// Capability-envelope input.
    pub capability_envelope: CapabilityEnvelopeInput,
    /// Host-isolation posture input.
    pub host_isolation: HostIsolationPostureInput,
    /// Activation-budget input.
    pub activation_budget: ActivationBudgetInput,
    /// Active-contribution inspector entries.
    #[serde(default)]
    pub contributions: Vec<ActiveContributionInspectorEntryInput>,
    /// Stability qualification claim input.
    pub claim: RuntimeAbiQualificationClaimInput,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input for [`RuntimeAbiIdentity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAbiIdentityInput {
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

/// Input for [`RuntimeClassDeclaration`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeClassDeclarationInput {
    /// Published runtime class.
    pub runtime_class: String,
    /// Execution locus for the runtime class.
    pub execution_locus_class: String,
}

/// Input for [`SandboxProfileBinding`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxProfileBindingInput {
    /// Stable, published sandbox profile id.
    pub sandbox_profile_id: String,
    /// Backend classification enforcing the profile.
    pub backend_classification_class: String,
    /// Enforcement state for the published profile.
    pub enforcement_state_class: String,
    /// Public label naming the platform backend (e.g. "macOS seatbelt").
    pub platform_backend_label: String,
    /// MUST be false on a sandboxed runtime class; a claimed sandbox can never
    /// silently widen to ambient full-user execution.
    pub widens_to_ambient_full_user: bool,
}

/// Input for [`CapabilityEnvelope`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityEnvelopeInput {
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

/// Input for [`HostIsolationPosture`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostIsolationPostureInput {
    /// Isolation boundary class.
    pub isolation_boundary_class: String,
    /// Restart posture class.
    pub restart_posture_class: String,
    /// Restart attempts recorded so far in the current host session.
    pub restart_attempt_count: u32,
}

/// Input for [`ActivationBudget`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivationBudgetInput {
    /// Activation-budget class.
    pub budget_class: String,
    /// Opaque ref to the declared activation-cost budget.
    pub declared_cost_ref: String,
    /// Opaque ref to the observed activation-cost evidence.
    pub observed_cost_ref: String,
}

/// Input for one [`ActiveContributionInspectorEntry`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveContributionInspectorEntryInput {
    /// Stable contribution id.
    pub contribution_id: String,
    /// Contribution kind.
    pub contribution_kind_class: String,
    /// Source package the contribution belongs to.
    pub source_package_ref: String,
    /// Runtime class the contribution runs under.
    pub runtime_class: String,
    /// Execution locus for the contribution.
    pub execution_locus_class: String,
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

/// Input for [`RuntimeAbiQualificationClaim`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAbiQualificationClaimInput {
    /// Runtime tier claimed by the row.
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
pub struct RuntimeAbiIdentity {
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

impl RuntimeAbiIdentity {
    /// Returns true when the row pins the published stable ABI version.
    pub fn abi_version_current(&self) -> bool {
        self.abi_contract_version == STABLE_RUNTIME_ABI_PUBLISHED_VERSION
    }

    /// Returns true when the lifecycle is runnable.
    pub fn lifecycle_runnable(&self) -> bool {
        RUNNABLE_LIFECYCLE_STATES.contains(&self.lifecycle_state_class.as_str())
    }
}

/// Published runtime-class declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeClassDeclaration {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Published runtime class.
    pub runtime_class: String,
    /// Execution locus for the runtime class.
    pub execution_locus_class: String,
}

impl RuntimeClassDeclaration {
    /// Returns true when the runtime class executes extension-authored code under
    /// a sandbox.
    pub fn is_sandboxed(&self) -> bool {
        SANDBOXED_RUNTIME_CLASSES.contains(&self.runtime_class.as_str())
    }
}

/// Stable sandbox-profile binding plus its fail-closed enforcement state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxProfileBinding {
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
    /// Always false; a claimed sandbox never widens to ambient full-user.
    pub widens_to_ambient_full_user: bool,
}

impl SandboxProfileBinding {
    /// Returns true when the published profile is enforced as published.
    pub fn enforced_as_published(&self) -> bool {
        self.enforcement_state_class == "enforced_as_published"
    }
}

/// Capability envelope that never widens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityEnvelope {
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

impl CapabilityEnvelope {
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

/// Host-isolation posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostIsolationPosture {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Isolation boundary class.
    pub isolation_boundary_class: String,
    /// Restart posture class.
    pub restart_posture_class: String,
    /// Restart attempts recorded so far in the current host session.
    pub restart_attempt_count: u32,
}

/// Activation-budget instrumentation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivationBudget {
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

/// One active-contribution inspector entry. Always carries source package,
/// runtime class, execution locus, trust tier, used permissions, and the
/// last-known-good host — even when quarantined, bridged, or downgraded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveContributionInspectorEntry {
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
    /// Runtime class the contribution runs under.
    pub runtime_class: String,
    /// Execution locus for the contribution.
    pub execution_locus_class: String,
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

impl ActiveContributionInspectorEntry {
    /// Returns true when the entry is fully attributed for the inspector.
    pub fn is_attributed(&self) -> bool {
        !self.source_package_ref.trim().is_empty()
            && !self.last_known_good_host_ref.trim().is_empty()
            && !self.contribution_id.trim().is_empty()
    }
}

/// Stability qualification claim after the posture is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAbiQualificationClaim {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Runtime tier claimed by the row.
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
pub struct DowngradedHostBanner {
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

/// Compact inspection row for CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableRuntimeAbiInspection {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet inspected by this row.
    pub packet_id_ref: String,
    /// Runtime class.
    pub runtime_class: String,
    /// Execution locus.
    pub execution_locus_class: String,
    /// Effective runtime tier.
    pub effective_tier: String,
    /// True when the claim is a stable runtime claim.
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
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// True when a downgraded-host banner is required.
    pub downgraded_host_banner_required: bool,
    /// True when every active contribution is fully attributed.
    pub attribution_complete: bool,
    /// Number of active contributions.
    pub active_contribution_count: usize,
    /// Number of quarantined/failed contributions.
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

/// Stable runtime-ABI packet consumed by install review, the runtime inspector,
/// the quarantine flow, diagnostics, support export, docs/help, and release
/// packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableRuntimeAbiPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity.
    pub identity: RuntimeAbiIdentity,
    /// Runtime-class declaration.
    pub runtime_class_declaration: RuntimeClassDeclaration,
    /// Sandbox-profile binding.
    pub sandbox_profile: SandboxProfileBinding,
    /// Capability envelope.
    pub capability_envelope: CapabilityEnvelope,
    /// Host-isolation posture.
    pub host_isolation: HostIsolationPosture,
    /// Activation-budget instrumentation.
    pub activation_budget: ActivationBudget,
    /// Active-contribution inspector entries.
    pub contributions: Vec<ActiveContributionInspectorEntry>,
    /// Stability qualification claim after the posture is applied.
    pub claim: RuntimeAbiQualificationClaim,
    /// Downgraded-host banner requirement.
    pub downgraded_host_banner: DowngradedHostBanner,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the packet cites.
    pub source_schema_refs: Vec<String>,
    /// False so a sandboxed contribution can never silently widen to ambient.
    pub allows_ambient_full_user_widening: bool,
    /// False so catalog-only trust cannot back a stable runtime claim.
    pub allows_catalog_only_trust: bool,
    /// False so an unbounded activation cost cannot ride the stable line.
    pub allows_unbounded_activation_cost: bool,
    /// False so a host downgrade can never be silent.
    pub allows_silent_host_downgrade: bool,
    /// Inspection row.
    pub inspection: StableRuntimeAbiInspection,
}

impl StableRuntimeAbiPacket {
    /// Builds a stable runtime-ABI packet from input, applying the runtime
    /// posture to the claimed tier so any required downgrade below Stable is
    /// automatic.
    ///
    /// # Errors
    ///
    /// Returns [`StableRuntimeAbiValidationError`] when the input violates an
    /// identity, runtime-class, sandbox, capability-envelope, contribution, or
    /// claim invariant — including the hard security guardrails (ambient
    /// widening, capability envelope widening).
    pub fn from_input(
        input: StableRuntimeAbiInput,
    ) -> Result<Self, StableRuntimeAbiValidationError> {
        validate_input(&input)?;

        let identity = identity_record(&input.identity);
        let runtime_class_declaration = runtime_class_record(&input.runtime_class_declaration);
        let sandbox_profile = sandbox_record(&input.sandbox_profile);
        let capability_envelope = envelope_record(&input.capability_envelope);
        let host_isolation = host_isolation_record(&input.host_isolation);
        let activation_budget = activation_budget_record(&input.activation_budget);
        let contributions: Vec<ActiveContributionInspectorEntry> = input
            .contributions
            .iter()
            .map(contribution_record)
            .collect();
        let claim = claim_record(
            &input.claim,
            &identity,
            &sandbox_profile,
            &activation_budget,
            &contributions,
        );
        let downgraded_host_banner = banner_record(&sandbox_profile, &identity, &contributions);
        let inspection = inspection_record(
            &input.packet_id,
            &runtime_class_declaration,
            &identity,
            &sandbox_profile,
            &capability_envelope,
            &activation_budget,
            &contributions,
            &claim,
            &downgraded_host_banner,
        );

        let packet = Self {
            record_kind: STABLE_RUNTIME_ABI_PACKET_RECORD_KIND.to_string(),
            schema_version: STABLE_RUNTIME_ABI_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            identity,
            runtime_class_declaration,
            sandbox_profile,
            capability_envelope,
            host_isolation,
            activation_budget,
            contributions,
            claim,
            downgraded_host_banner,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![STABLE_RUNTIME_ABI_SCHEMA_REF.to_string()],
            allows_ambient_full_user_widening: false,
            allows_catalog_only_trust: false,
            allows_unbounded_activation_cost: false,
            allows_silent_host_downgrade: false,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the stable runtime-ABI invariants.
    ///
    /// # Errors
    ///
    /// Returns [`StableRuntimeAbiValidationError`] when an invariant is violated.
    pub fn validate(&self) -> Result<(), StableRuntimeAbiValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            STABLE_RUNTIME_ABI_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            STABLE_RUNTIME_ABI_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_identity(&self.identity)?;
        validate_runtime_class(&self.runtime_class_declaration)?;
        validate_sandbox(&self.sandbox_profile, &self.runtime_class_declaration)?;
        validate_envelope(&self.capability_envelope)?;
        validate_host_isolation(&self.host_isolation)?;
        validate_activation_budget(&self.activation_budget)?;
        for entry in &self.contributions {
            validate_contribution(entry)?;
        }
        validate_claim(&self.claim)?;
        validate_banner(&self.downgraded_host_banner)?;

        for surface in &self.consumer_surfaces {
            ensure_token(
                STABLE_RUNTIME_ABI_CONSUMER_SURFACES,
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
            .any(|r| r == STABLE_RUNTIME_ABI_SCHEMA_REF)
        {
            return Err(err("packet must cite its source schema"));
        }

        // No ambient widening, catalog-only trust, unbounded cost, or silent
        // host downgrade may ride a published stable runtime row.
        if self.allows_ambient_full_user_widening
            || self.allows_catalog_only_trust
            || self.allows_unbounded_activation_cost
            || self.allows_silent_host_downgrade
        {
            return Err(err(
                "a stable runtime packet must not allow ambient widening, catalog-only trust, unbounded activation cost, or silent host downgrade",
            ));
        }

        // Stable-claim binding: a stable effective tier must pin the published
        // ABI, be enforcement-backed, enforce its sandbox as published, keep the
        // trust tier out of quarantine, stay runnable, keep every contribution
        // nominal, hold a bounded activation cost, and be fully attributed.
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
            if !self.sandbox_profile.enforced_as_published() {
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
            if self.claim.downgraded {
                return Err(err(
                    "a stable effective tier must not also be marked downgraded",
                ));
            }
        }

        // Downgrade truth: a downgraded claim carries at least one reason and
        // never keeps a stable effective tier.
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
            &self.sandbox_profile,
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
        let banner_required =
            host_is_downgraded(&self.sandbox_profile, &self.identity, &self.contributions);
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

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableRuntimeAbiProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Extension identity.
    pub extension_identity_ref: String,
    /// Runtime class.
    pub runtime_class: String,
    /// Execution locus.
    pub execution_locus_class: String,
    /// Claimed tier.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim is a stable runtime claim.
    pub stable_claim: bool,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// Downgrade reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-host banner is required.
    pub downgraded_host_banner_required: bool,
    /// Number of active contributions.
    pub active_contribution_count: usize,
}

impl From<StableRuntimeAbiPacket> for StableRuntimeAbiProjection {
    fn from(packet: StableRuntimeAbiPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            extension_identity_ref: packet.identity.extension_identity_ref,
            runtime_class: packet.runtime_class_declaration.runtime_class,
            execution_locus_class: packet.runtime_class_declaration.execution_locus_class,
            claimed_tier: packet.claim.claimed_tier,
            effective_tier: packet.claim.effective_tier,
            support_claim_class: packet.claim.support_claim_class,
            stable_claim: packet.inspection.stable_claim,
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
/// Returns [`StableRuntimeAbiError`] when the payload fails to parse or violates
/// the stable runtime-ABI invariants.
pub fn project_stable_runtime_abi(
    payload: &str,
) -> Result<StableRuntimeAbiProjection, StableRuntimeAbiError> {
    let packet: StableRuntimeAbiPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(StableRuntimeAbiProjection::from(packet))
}

// ---------------------------------------------------------------------------
// Support export projection
// ---------------------------------------------------------------------------

/// Metadata-safe support/partner export row that quotes the same closed tokens
/// as the packet without leaking raw manifest, profile, or runtime-payload bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableRuntimeAbiSupportExport {
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
    /// Runtime class.
    pub runtime_class: String,
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
    /// Number of quarantined/failed contributions.
    pub quarantined_contribution_count: usize,
    /// Number of contributions on a narrower-than-published profile.
    pub downgraded_contribution_count: usize,
    /// Activation-budget class.
    pub activation_budget_class: String,
    /// Export-safe summary suitable for support/partner consumers.
    pub export_safe_summary: String,
}

/// Projects a packet into the metadata-safe support/partner export row.
pub fn project_stable_runtime_abi_support_export(
    packet: &StableRuntimeAbiPacket,
) -> StableRuntimeAbiSupportExport {
    let blocks_activation = packet.claim.effective_tier == "withdrawn";
    let export_safe_summary = format!(
        "{} Runtime class={} locus={}. Sandbox profile={} backend={} enforcement={}. Trust={} lifecycle={}. Tier claimed={} effective={} (downgraded={}). Banner required={}. Contributions: active={} quarantined={} downgraded={}. Activation budget={}.",
        packet.claim.summary_label,
        packet.runtime_class_declaration.runtime_class,
        packet.runtime_class_declaration.execution_locus_class,
        packet.sandbox_profile.sandbox_profile_id,
        packet.sandbox_profile.backend_classification_class,
        packet.sandbox_profile.enforcement_state_class,
        packet.identity.publisher_trust_tier_class,
        packet.identity.lifecycle_state_class,
        packet.claim.claimed_tier,
        packet.claim.effective_tier,
        packet.claim.downgraded,
        packet.downgraded_host_banner.must_display,
        packet.inspection.active_contribution_count,
        packet.inspection.quarantined_contribution_count,
        packet.inspection.downgraded_contribution_count,
        packet.activation_budget.budget_class,
    );

    StableRuntimeAbiSupportExport {
        record_kind: STABLE_RUNTIME_ABI_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: STABLE_RUNTIME_ABI_SCHEMA_VERSION,
        export_id: format!("stable_runtime_abi_support_export:{}", packet.packet_id),
        packet_ref: packet.packet_id.clone(),
        extension_identity_ref: packet.identity.extension_identity_ref.clone(),
        extension_version: packet.identity.extension_version.clone(),
        source_package_ref: packet.identity.source_package_ref.clone(),
        runtime_class: packet.runtime_class_declaration.runtime_class.clone(),
        execution_locus_class: packet
            .runtime_class_declaration
            .execution_locus_class
            .clone(),
        backend_classification_class: packet.sandbox_profile.backend_classification_class.clone(),
        sandbox_profile_id: packet.sandbox_profile.sandbox_profile_id.clone(),
        sandbox_enforcement_state_class: packet.sandbox_profile.enforcement_state_class.clone(),
        trust_tier_class: packet.identity.publisher_trust_tier_class.clone(),
        lifecycle_state_class: packet.identity.lifecycle_state_class.clone(),
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

/// Error enum for stable runtime-ABI operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableRuntimeAbiError {
    /// Validation failed.
    Validation(StableRuntimeAbiValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for StableRuntimeAbiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for StableRuntimeAbiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for stable runtime-ABI packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableRuntimeAbiValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for StableRuntimeAbiValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StableRuntimeAbiValidationError {}

impl StableRuntimeAbiValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<serde_json::Error> for StableRuntimeAbiError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(StableRuntimeAbiValidationError {
            message: err.to_string(),
        })
    }
}

impl From<StableRuntimeAbiValidationError> for StableRuntimeAbiError {
    fn from(err: StableRuntimeAbiValidationError) -> Self {
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

/// Applies the runtime posture to a claimed tier, narrowing automatically below
/// Stable when the host can no longer back it.
fn derive_effective_tier(
    claimed_tier: &str,
    claim_basis: &str,
    identity: &RuntimeAbiIdentity,
    sandbox: &SandboxProfileBinding,
    activation_budget: &ActivationBudget,
    contributions: &[ActiveContributionInspectorEntry],
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
        "stable" => "stable_runtime_ready_claim",
        "beta" => "beta_runtime_partial_claim",
        "preview" => "preview_runtime_experimental_claim",
        "withdrawn" => "withdrawn_no_runtime_claim",
        _ => "preview_runtime_experimental_claim",
    }
    .to_string()
}

/// Returns true when the host posture is downgraded and a banner must be shown.
fn host_is_downgraded(
    sandbox: &SandboxProfileBinding,
    identity: &RuntimeAbiIdentity,
    contributions: &[ActiveContributionInspectorEntry],
) -> bool {
    matches!(
        sandbox.enforcement_state_class.as_str(),
        "fail_closed_downgraded" | "unenforceable_refused"
    ) || identity.publisher_trust_tier_class == "quarantined"
        || contributions.iter().any(|c| {
            matches!(
                c.host_state_class.as_str(),
                "quarantined" | "failed" | "downgraded_narrower_profile"
            )
        })
}

/// Picks the most-severe banner reason for a downgraded host posture.
fn banner_reason_for(
    sandbox: &SandboxProfileBinding,
    identity: &RuntimeAbiIdentity,
    contributions: &[ActiveContributionInspectorEntry],
) -> Option<String> {
    if sandbox.enforcement_state_class == "unenforceable_refused" {
        return Some("sandbox_unenforceable".to_string());
    }
    if sandbox.enforcement_state_class == "fail_closed_downgraded" {
        return Some("sandbox_fail_closed_downgraded".to_string());
    }
    if identity.publisher_trust_tier_class == "quarantined" {
        return Some("trust_tier_quarantined".to_string());
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

fn identity_record(input: &RuntimeAbiIdentityInput) -> RuntimeAbiIdentity {
    RuntimeAbiIdentity {
        record_kind: RUNTIME_ABI_IDENTITY_RECORD_KIND.to_string(),
        schema_version: STABLE_RUNTIME_ABI_SCHEMA_VERSION,
        runtime_contract_ref: input.runtime_contract_ref.clone(),
        extension_identity_ref: input.extension_identity_ref.clone(),
        extension_version: input.extension_version.clone(),
        source_package_ref: input.source_package_ref.clone(),
        abi_contract_version: input.abi_contract_version,
        publisher_trust_tier_class: input.publisher_trust_tier_class.clone(),
        lifecycle_state_class: input.lifecycle_state_class.clone(),
    }
}

fn runtime_class_record(input: &RuntimeClassDeclarationInput) -> RuntimeClassDeclaration {
    RuntimeClassDeclaration {
        record_kind: RUNTIME_CLASS_DECLARATION_RECORD_KIND.to_string(),
        schema_version: STABLE_RUNTIME_ABI_SCHEMA_VERSION,
        runtime_class: input.runtime_class.clone(),
        execution_locus_class: input.execution_locus_class.clone(),
    }
}

fn sandbox_record(input: &SandboxProfileBindingInput) -> SandboxProfileBinding {
    SandboxProfileBinding {
        record_kind: SANDBOX_PROFILE_BINDING_RECORD_KIND.to_string(),
        schema_version: STABLE_RUNTIME_ABI_SCHEMA_VERSION,
        sandbox_profile_id: input.sandbox_profile_id.clone(),
        backend_classification_class: input.backend_classification_class.clone(),
        enforcement_state_class: input.enforcement_state_class.clone(),
        platform_backend_label: input.platform_backend_label.clone(),
        widens_to_ambient_full_user: input.widens_to_ambient_full_user,
    }
}

fn envelope_record(input: &CapabilityEnvelopeInput) -> CapabilityEnvelope {
    CapabilityEnvelope {
        record_kind: CAPABILITY_ENVELOPE_RECORD_KIND.to_string(),
        schema_version: STABLE_RUNTIME_ABI_SCHEMA_VERSION,
        declared_capability_refs: input.declared_capability_refs.clone(),
        negotiated_capability_refs: input.negotiated_capability_refs.clone(),
        granted_capability_refs: input.granted_capability_refs.clone(),
        narrowing_reasons_recorded: input.narrowing_reasons_recorded,
    }
}

fn host_isolation_record(input: &HostIsolationPostureInput) -> HostIsolationPosture {
    HostIsolationPosture {
        record_kind: HOST_ISOLATION_POSTURE_RECORD_KIND.to_string(),
        schema_version: STABLE_RUNTIME_ABI_SCHEMA_VERSION,
        isolation_boundary_class: input.isolation_boundary_class.clone(),
        restart_posture_class: input.restart_posture_class.clone(),
        restart_attempt_count: input.restart_attempt_count,
    }
}

fn activation_budget_record(input: &ActivationBudgetInput) -> ActivationBudget {
    ActivationBudget {
        record_kind: ACTIVATION_BUDGET_RECORD_KIND.to_string(),
        schema_version: STABLE_RUNTIME_ABI_SCHEMA_VERSION,
        budget_class: input.budget_class.clone(),
        declared_cost_ref: input.declared_cost_ref.clone(),
        observed_cost_ref: input.observed_cost_ref.clone(),
    }
}

fn contribution_record(
    input: &ActiveContributionInspectorEntryInput,
) -> ActiveContributionInspectorEntry {
    ActiveContributionInspectorEntry {
        record_kind: ACTIVE_CONTRIBUTION_INSPECTOR_ENTRY_RECORD_KIND.to_string(),
        schema_version: STABLE_RUNTIME_ABI_SCHEMA_VERSION,
        contribution_id: input.contribution_id.clone(),
        contribution_kind_class: input.contribution_kind_class.clone(),
        source_package_ref: input.source_package_ref.clone(),
        runtime_class: input.runtime_class.clone(),
        execution_locus_class: input.execution_locus_class.clone(),
        trust_tier_class: input.trust_tier_class.clone(),
        used_permission_refs: input.used_permission_refs.clone(),
        last_known_good_host_ref: input.last_known_good_host_ref.clone(),
        host_state_class: input.host_state_class.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn claim_record(
    input: &RuntimeAbiQualificationClaimInput,
    identity: &RuntimeAbiIdentity,
    sandbox: &SandboxProfileBinding,
    activation_budget: &ActivationBudget,
    contributions: &[ActiveContributionInspectorEntry],
) -> RuntimeAbiQualificationClaim {
    let attribution_complete = !identity.source_package_ref.trim().is_empty()
        && contributions.iter().all(|c| c.is_attributed());
    let derived = derive_effective_tier(
        &input.claimed_tier,
        &input.claim_basis_class,
        identity,
        sandbox,
        activation_budget,
        contributions,
        attribution_complete,
    );
    RuntimeAbiQualificationClaim {
        record_kind: RUNTIME_ABI_QUALIFICATION_CLAIM_RECORD_KIND.to_string(),
        schema_version: STABLE_RUNTIME_ABI_SCHEMA_VERSION,
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
    sandbox: &SandboxProfileBinding,
    identity: &RuntimeAbiIdentity,
    contributions: &[ActiveContributionInspectorEntry],
) -> DowngradedHostBanner {
    let must_display = host_is_downgraded(sandbox, identity, contributions);
    let banner_reason_class = if must_display {
        banner_reason_for(sandbox, identity, contributions)
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
            "Host running on a narrower-than-published profile ({}).",
            banner_reason_class.clone().unwrap_or_default()
        )
    } else {
        "Host enforcing the published runtime profile.".to_string()
    };
    DowngradedHostBanner {
        record_kind: DOWNGRADED_HOST_BANNER_RECORD_KIND.to_string(),
        schema_version: STABLE_RUNTIME_ABI_SCHEMA_VERSION,
        must_display,
        banner_reason_class,
        last_known_good_host_ref,
        summary_label,
    }
}

#[allow(clippy::too_many_arguments)]
fn inspection_record(
    packet_id: &str,
    runtime_class: &RuntimeClassDeclaration,
    identity: &RuntimeAbiIdentity,
    sandbox: &SandboxProfileBinding,
    envelope: &CapabilityEnvelope,
    activation_budget: &ActivationBudget,
    contributions: &[ActiveContributionInspectorEntry],
    claim: &RuntimeAbiQualificationClaim,
    banner: &DowngradedHostBanner,
) -> StableRuntimeAbiInspection {
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

    StableRuntimeAbiInspection {
        record_kind: STABLE_RUNTIME_ABI_INSPECTION_RECORD_KIND.to_string(),
        schema_version: STABLE_RUNTIME_ABI_SCHEMA_VERSION,
        packet_id_ref: packet_id.to_string(),
        runtime_class: runtime_class.runtime_class.clone(),
        execution_locus_class: runtime_class.execution_locus_class.clone(),
        effective_tier: claim.effective_tier.clone(),
        stable_claim,
        abi_version_current: identity.abi_version_current(),
        sandbox_enforced_as_published: sandbox.enforced_as_published(),
        widens_to_ambient_full_user: sandbox.widens_to_ambient_full_user,
        capability_envelope_well_formed: envelope.is_well_formed(),
        trust_tier_class: identity.publisher_trust_tier_class.clone(),
        lifecycle_runnable: identity.lifecycle_runnable(),
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

fn validate_input(input: &StableRuntimeAbiInput) -> Result<(), StableRuntimeAbiValidationError> {
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

    let rc = &input.runtime_class_declaration;
    ensure_token(RUNTIME_CLASSES, &rc.runtime_class, "runtime_class")?;
    ensure_token(
        EXECUTION_LOCUS_CLASSES,
        &rc.execution_locus_class,
        "execution_locus_class",
    )?;
    if !runtime_class_supports_locus(&rc.runtime_class, &rc.execution_locus_class) {
        return Err(err(format!(
            "execution_locus_class {} is not reserved for runtime_class {}",
            rc.execution_locus_class, rc.runtime_class
        )));
    }

    let sb = &input.sandbox_profile;
    ensure_nonempty(&sb.sandbox_profile_id, "sandbox_profile.sandbox_profile_id")?;
    ensure_token(
        BACKEND_CLASSIFICATION_CLASSES,
        &sb.backend_classification_class,
        "sandbox_profile.backend_classification_class",
    )?;
    ensure_token(
        SANDBOX_ENFORCEMENT_STATES,
        &sb.enforcement_state_class,
        "sandbox_profile.enforcement_state_class",
    )?;
    ensure_nonempty(
        &sb.platform_backend_label,
        "sandbox_profile.platform_backend_label",
    )?;
    if !backend_supports_runtime_class(&sb.backend_classification_class, &rc.runtime_class) {
        return Err(err(format!(
            "backend_classification_class {} is not reserved for runtime_class {}",
            sb.backend_classification_class, rc.runtime_class
        )));
    }
    // Hard security guardrail: a claimed sandboxed runtime class can never widen
    // to ambient full-user execution.
    if SANDBOXED_RUNTIME_CLASSES.contains(&rc.runtime_class.as_str())
        && sb.widens_to_ambient_full_user
    {
        return Err(err(
            "a sandboxed runtime class must not widen to ambient full-user execution",
        ));
    }
    // A non-executing class runs no code, so it can never widen and never needs a
    // fail-closed downgrade.
    if NON_EXECUTING_RUNTIME_CLASSES.contains(&rc.runtime_class.as_str()) {
        if sb.widens_to_ambient_full_user {
            return Err(err(
                "a non-executing runtime class must not widen to ambient full-user execution",
            ));
        }
        if sb.enforcement_state_class != "enforced_as_published" {
            return Err(err(
                "a non-executing runtime class has no profile to fail-close; enforcement must be enforced_as_published",
            ));
        }
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

    let hi = &input.host_isolation;
    ensure_token(
        ISOLATION_BOUNDARY_CLASSES,
        &hi.isolation_boundary_class,
        "host_isolation.isolation_boundary_class",
    )?;
    ensure_token(
        RESTART_POSTURE_CLASSES,
        &hi.restart_posture_class,
        "host_isolation.restart_posture_class",
    )?;

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
            RUNTIME_CLASSES,
            &c.runtime_class,
            "contribution.runtime_class",
        )?;
        ensure_token(
            EXECUTION_LOCUS_CLASSES,
            &c.execution_locus_class,
            "contribution.execution_locus_class",
        )?;
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
            STABLE_RUNTIME_ABI_CONSUMER_SURFACES,
            surface,
            "consumer_surface",
        )?;
    }
    if input.consumer_surfaces.is_empty() {
        return Err(err("input must bind at least one consumer surface"));
    }

    Ok(())
}

fn validate_identity(identity: &RuntimeAbiIdentity) -> Result<(), StableRuntimeAbiValidationError> {
    ensure_eq(
        identity.record_kind.as_str(),
        RUNTIME_ABI_IDENTITY_RECORD_KIND,
        "identity record_kind",
    )?;
    ensure_eq_u32(
        identity.schema_version,
        STABLE_RUNTIME_ABI_SCHEMA_VERSION,
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

fn validate_runtime_class(
    rc: &RuntimeClassDeclaration,
) -> Result<(), StableRuntimeAbiValidationError> {
    ensure_eq(
        rc.record_kind.as_str(),
        RUNTIME_CLASS_DECLARATION_RECORD_KIND,
        "runtime_class record_kind",
    )?;
    ensure_token(RUNTIME_CLASSES, &rc.runtime_class, "runtime_class")?;
    ensure_token(
        EXECUTION_LOCUS_CLASSES,
        &rc.execution_locus_class,
        "execution_locus_class",
    )?;
    if !runtime_class_supports_locus(&rc.runtime_class, &rc.execution_locus_class) {
        return Err(err(
            "execution_locus_class is not reserved for runtime_class",
        ));
    }
    Ok(())
}

fn validate_sandbox(
    sb: &SandboxProfileBinding,
    rc: &RuntimeClassDeclaration,
) -> Result<(), StableRuntimeAbiValidationError> {
    ensure_eq(
        sb.record_kind.as_str(),
        SANDBOX_PROFILE_BINDING_RECORD_KIND,
        "sandbox record_kind",
    )?;
    ensure_token(
        BACKEND_CLASSIFICATION_CLASSES,
        &sb.backend_classification_class,
        "sandbox backend_classification_class",
    )?;
    ensure_token(
        SANDBOX_ENFORCEMENT_STATES,
        &sb.enforcement_state_class,
        "sandbox enforcement_state_class",
    )?;
    if !backend_supports_runtime_class(&sb.backend_classification_class, &rc.runtime_class) {
        return Err(err(
            "backend_classification_class is not reserved for runtime_class",
        ));
    }
    if rc.is_sandboxed() && sb.widens_to_ambient_full_user {
        return Err(err(
            "a sandboxed runtime class must not widen to ambient full-user execution",
        ));
    }
    Ok(())
}

fn validate_envelope(env: &CapabilityEnvelope) -> Result<(), StableRuntimeAbiValidationError> {
    ensure_eq(
        env.record_kind.as_str(),
        CAPABILITY_ENVELOPE_RECORD_KIND,
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

fn validate_host_isolation(
    hi: &HostIsolationPosture,
) -> Result<(), StableRuntimeAbiValidationError> {
    ensure_eq(
        hi.record_kind.as_str(),
        HOST_ISOLATION_POSTURE_RECORD_KIND,
        "host_isolation record_kind",
    )?;
    ensure_token(
        ISOLATION_BOUNDARY_CLASSES,
        &hi.isolation_boundary_class,
        "host_isolation isolation_boundary_class",
    )?;
    ensure_token(
        RESTART_POSTURE_CLASSES,
        &hi.restart_posture_class,
        "host_isolation restart_posture_class",
    )?;
    Ok(())
}

fn validate_activation_budget(
    ab: &ActivationBudget,
) -> Result<(), StableRuntimeAbiValidationError> {
    ensure_eq(
        ab.record_kind.as_str(),
        ACTIVATION_BUDGET_RECORD_KIND,
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
    c: &ActiveContributionInspectorEntry,
) -> Result<(), StableRuntimeAbiValidationError> {
    ensure_eq(
        c.record_kind.as_str(),
        ACTIVE_CONTRIBUTION_INSPECTOR_ENTRY_RECORD_KIND,
        "contribution record_kind",
    )?;
    ensure_token(
        CONTRIBUTION_KIND_CLASSES,
        &c.contribution_kind_class,
        "contribution contribution_kind_class",
    )?;
    ensure_token(
        RUNTIME_CLASSES,
        &c.runtime_class,
        "contribution runtime_class",
    )?;
    ensure_token(
        EXECUTION_LOCUS_CLASSES,
        &c.execution_locus_class,
        "contribution execution_locus_class",
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
    // The inspector must stay attributable even when quarantined/bridged/downgraded.
    if !c.is_attributed() {
        return Err(err(
            "contribution inspector entry must keep source package, id, and last-known-good host",
        ));
    }
    Ok(())
}

fn validate_claim(
    claim: &RuntimeAbiQualificationClaim,
) -> Result<(), StableRuntimeAbiValidationError> {
    ensure_eq(
        claim.record_kind.as_str(),
        RUNTIME_ABI_QUALIFICATION_CLAIM_RECORD_KIND,
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
            RUNTIME_ABI_DOWNGRADE_REASONS,
            reason,
            "claim downgrade_reason",
        )?;
    }
    Ok(())
}

fn validate_banner(banner: &DowngradedHostBanner) -> Result<(), StableRuntimeAbiValidationError> {
    ensure_eq(
        banner.record_kind.as_str(),
        DOWNGRADED_HOST_BANNER_RECORD_KIND,
        "banner record_kind",
    )?;
    if let Some(reason) = &banner.banner_reason_class {
        ensure_token(
            RUNTIME_ABI_DOWNGRADE_REASONS,
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
    inspection: &StableRuntimeAbiInspection,
    packet: &StableRuntimeAbiPacket,
) -> Result<(), StableRuntimeAbiValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        STABLE_RUNTIME_ABI_INSPECTION_RECORD_KIND,
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
    if inspection.widens_to_ambient_full_user != packet.sandbox_profile.widens_to_ambient_full_user
    {
        return Err(err(
            "inspection widens_to_ambient_full_user is inconsistent",
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

fn runtime_class_supports_locus(runtime_class: &str, locus: &str) -> bool {
    matches!(
        (runtime_class, locus),
        ("passive_package", "passive_no_execution")
            | (
                "declarative_host_rendered_view",
                "host_rendered_no_extension_code"
            )
            | (
                "wasm_capability_sandbox",
                "editor_in_process_isolated" | "dedicated_subprocess"
            )
            | ("external_host", "dedicated_subprocess" | "helper_binary")
            | ("compatibility_bridge", "bridged_foreign_runtime")
            | ("remote_side_component", "remote_agent")
    )
}

fn backend_supports_runtime_class(backend: &str, runtime_class: &str) -> bool {
    matches!(
        (runtime_class, backend),
        ("passive_package", "none_passive")
            | ("declarative_host_rendered_view", "none_passive")
            | (
                "wasm_capability_sandbox",
                "wasm_component_model" | "wasm_core_module"
            )
            | (
                "external_host",
                "os_process_sandbox"
                    | "seatbelt_sandbox_profile"
                    | "landlock_seccomp_profile"
                    | "app_container_profile"
            )
            | ("compatibility_bridge", "bridge_translated_profile")
            | ("remote_side_component", "remote_enforced_envelope")
    )
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn err(message: impl Into<String>) -> StableRuntimeAbiValidationError {
    StableRuntimeAbiValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(left: T, right: T, field: &str) -> Result<(), StableRuntimeAbiValidationError>
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
) -> Result<(), StableRuntimeAbiValidationError> {
    if left != right {
        return Err(err(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), StableRuntimeAbiValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), StableRuntimeAbiValidationError> {
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
