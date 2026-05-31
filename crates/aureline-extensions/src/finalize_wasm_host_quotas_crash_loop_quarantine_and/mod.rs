//! Finalize Wasm host quotas, crash-loop quarantine, and restart-budget
//! governance for the stable line.
//!
//! The extension host *supervision* contract (see [`crate::supervision`]) owns
//! the per-evaluation beta decision: which axis is under pressure, whether to
//! throttle/disable/quarantine, and how the response surfaces. This module owns
//! the layer above it — the **stable, published governance truth** a claimed
//! stable ecosystem row carries for its Wasm host quotas, its crash-loop
//! quarantine posture, and its restart-budget governance, plus the **stability
//! qualification** that truth is allowed to claim.
//!
//! The central rule mirrors the rest of the stable line: a **stable** governance
//! claim may never be implied from a catalog row alone. A row that renders a
//! `stable` governance badge must pin the published governance contract version,
//! be enforcement-backed, hold every Wasm host quota axis bounded and enforced as
//! published, keep its crash-loop window clear of a disable/quarantine trip, keep
//! its restart budget bounded and not exhausted, keep its quarantine posture
//! nominal, keep its publisher trust tier out of quarantine, stay on a runnable
//! lifecycle, keep every active contribution nominal, and be fully attributed.
//! When any of those fails, the visible tier is **automatically narrowed below
//! Stable** (to `beta` or `preview`, or `withdrawn` when a quota cannot be
//! bounded, a restart posture is unbounded, a crash loop has tripped quarantine,
//! or the lifecycle is not runnable) rather than left asserting a governance
//! readiness the host cannot back.
//!
//! Two security guardrails are encoded so they cannot be papered over:
//!
//! - **No unbounded quota.** A Wasm host quota axis whose binding is `bounded ==
//!   false` (or whose enforcement state is `unenforceable_refused`) can never ride
//!   the stable line — the claim is narrowed to `withdrawn` and a banner is raised.
//! - **No unbounded restart.** A `restart_budget_unbounded_refused` restart
//!   posture is withdrawn; a stable claim must use a bounded posture and keep its
//!   attempts under budget.
//!
//! The quarantine posture, the crash-loop trip state, and the downgraded-host
//! banner are **re-derived from the governance posture at validation time**, so a
//! stored packet can never drift from its evidence. The published runtime-class
//! vocabulary ([`RUNTIME_CLASSES`]) is carried on the record and on every
//! [`GovernedContributionEntry`] so install review, the runtime inspector, the
//! quarantine flow, diagnostics, and support exports name the real execution model
//! instead of collapsing everything into a generic extension badge.
//!
//! The companion schema lives at
//! [`schemas/extensions/stable_wasm_host_governance.schema.json`](../../../../schemas/extensions/stable_wasm_host_governance.schema.json).
//! Canonical fixtures live under
//! `fixtures/extensions/m4/finalize-wasm-host-quotas-crash-loop-quarantine-and/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every stable Wasm-host-governance record.
pub const STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// The published, stable governance contract version. A `stable` governance
/// claim must pin exactly this version; any other version narrows below Stable.
pub const STABLE_GOVERNANCE_PUBLISHED_VERSION: u32 = 1;

/// Canonical schema path the packet cites.
pub const STABLE_WASM_HOST_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/extensions/stable_wasm_host_governance.schema.json";

/// Record-kind tag for [`StableWasmHostGovernancePacket`].
pub const STABLE_WASM_HOST_GOVERNANCE_PACKET_RECORD_KIND: &str =
    "stable_wasm_host_governance_packet";

/// Record-kind tag for [`GovernanceIdentity`].
pub const GOVERNANCE_IDENTITY_RECORD_KIND: &str = "stable_wasm_host_governance_identity";

/// Record-kind tag for [`GovernanceRuntimeClassDeclaration`].
pub const GOVERNANCE_RUNTIME_CLASS_DECLARATION_RECORD_KIND: &str =
    "stable_wasm_host_governance_runtime_class_declaration";

/// Record-kind tag for [`HostQuotaAxis`].
pub const HOST_QUOTA_AXIS_RECORD_KIND: &str = "stable_wasm_host_quota_axis";

/// Record-kind tag for [`CrashLoopGovernance`].
pub const CRASH_LOOP_GOVERNANCE_RECORD_KIND: &str = "stable_crash_loop_governance";

/// Record-kind tag for [`RestartBudgetGovernance`].
pub const RESTART_BUDGET_GOVERNANCE_RECORD_KIND: &str = "stable_restart_budget_governance";

/// Record-kind tag for [`QuarantinePosture`].
pub const QUARANTINE_POSTURE_RECORD_KIND: &str = "stable_quarantine_posture";

/// Record-kind tag for [`GovernedContributionEntry`].
pub const GOVERNED_CONTRIBUTION_ENTRY_RECORD_KIND: &str = "stable_governed_contribution_entry";

/// Record-kind tag for [`GovernanceQualificationClaim`].
pub const GOVERNANCE_QUALIFICATION_CLAIM_RECORD_KIND: &str =
    "stable_wasm_host_governance_qualification_claim";

/// Record-kind tag for [`GovernanceDowngradedHostBanner`].
pub const GOVERNANCE_DOWNGRADED_HOST_BANNER_RECORD_KIND: &str =
    "stable_wasm_host_governance_downgraded_host_banner";

/// Record-kind tag for [`StableWasmHostGovernanceInspection`].
pub const STABLE_WASM_HOST_GOVERNANCE_INSPECTION_RECORD_KIND: &str =
    "stable_wasm_host_governance_inspection";

/// Record-kind tag for [`StableWasmHostGovernanceSupportExport`].
pub const STABLE_WASM_HOST_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stable_wasm_host_governance_support_export";

/// Published, controlled runtime-class vocabulary, shared with the stable
/// runtime-ABI lane so support and review surfaces carry one vocabulary.
pub const RUNTIME_CLASSES: &[&str] = &[
    "passive_package",
    "wasm_capability_sandbox",
    "declarative_host_rendered_view",
    "external_host",
    "compatibility_bridge",
    "remote_side_component",
];

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

/// Closed Wasm-host quota-axis vocabulary. Every executing host publishes the
/// bounds it enforces on these axes.
pub const QUOTA_AXIS_CLASSES: &[&str] = &[
    "linear_memory",
    "execution_fuel",
    "table_elements",
    "instance_count",
    "epoch_deadline",
    "host_call_egress",
];

/// Closed quota-enforcement-state vocabulary. `enforced_as_published` is the only
/// state a stable claim may keep on every axis; the others are the fail-closed
/// downgrade.
pub const QUOTA_ENFORCEMENT_STATES: &[&str] = &[
    "enforced_as_published",
    "fail_closed_downgraded",
    "unenforceable_refused",
];

/// Closed quota-pressure vocabulary describing observed peak vs. the published
/// bound.
pub const QUOTA_PRESSURE_CLASSES: &[&str] = &[
    "nominal",
    "approaching_limit",
    "soft_breach",
    "hard_breach",
    "not_applicable",
];

/// Closed crash-loop-state vocabulary mirrored from the quarantine-rule lane.
pub const CRASH_LOOP_STATE_CLASSES: &[&str] = &[
    "nominal",
    "window_open",
    "disable_tripped",
    "quarantine_tripped",
];

/// Closed restart-posture vocabulary. `restart_budget_unbounded_refused` may never
/// be stable.
pub const RESTART_POSTURE_CLASSES: &[&str] = &[
    "no_restart_attempted",
    "one_warm_restart_under_budget",
    "exponential_backoff_bounded",
    "restart_budget_unbounded_refused",
];

/// Closed quarantine-state vocabulary derived from the governance posture.
pub const QUARANTINE_STATE_CLASSES: &[&str] = &[
    "none_nominal",
    "throttled",
    "disabled_until_next_session",
    "quarantined",
];

/// Closed recovery-precondition vocabulary describing the path back to nominal.
pub const RECOVERY_PRECONDITION_CLASSES: &[&str] = &[
    "none_not_recovering",
    "resource_returned_to_nominal",
    "crash_loop_window_cleared",
    "admin_cleared_quarantine",
    "user_explicit_reenable",
    "next_session_cold_start",
];

/// Closed visibility-surface vocabulary. A non-nominal quarantine posture must
/// surface on install_review or the runtime inspector, never nowhere.
pub const VISIBILITY_SURFACE_CLASSES: &[&str] = &[
    "not_visible_nominal_row",
    "runtime_status_pill_only",
    "install_review_and_inspector",
    "install_review_and_pill",
    "inspector_and_pill",
    "install_review_inspector_and_pill",
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

/// Lifecycle states a stable governance claim may keep.
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

/// Closed set of switch-readiness stability tiers.
pub const STABILITY_TIERS: &[&str] = &["stable", "beta", "preview", "withdrawn"];

/// Tiers that count as a *stable* governance claim.
pub const STABLE_TIERS: &[&str] = &["stable"];

/// Closed claim-basis vocabulary. `catalog_asserted_only` may never back stable.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["enforcement_backed", "catalog_asserted_only"];

/// Closed support-claim vocabulary a tier may imply.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_governance_ready_claim",
    "beta_governance_partial_claim",
    "preview_governance_experimental_claim",
    "withdrawn_no_governance_claim",
];

/// Closed set of reasons that narrow a stable governance claim below Stable.
pub const GOVERNANCE_DOWNGRADE_REASONS: &[&str] = &[
    "governance_version_mismatch",
    "catalog_only_trust_not_enforcement_backed",
    "quota_enforcement_unverified",
    "quota_axis_soft_breached",
    "quota_axis_hard_breached",
    "quota_unbounded_refused",
    "crash_loop_window_breached",
    "crash_loop_quarantine_active",
    "restart_budget_exhausted",
    "restart_posture_unbounded",
    "contribution_not_nominal",
    "trust_tier_quarantined",
    "lifecycle_not_runnable",
    "attribution_incomplete",
];

/// Reasons that narrow all the way to `withdrawn`.
const WITHDRAWN_CLASS_REASONS: &[&str] = &[
    "quota_unbounded_refused",
    "crash_loop_quarantine_active",
    "restart_posture_unbounded",
    "lifecycle_not_runnable",
];

/// Reasons that narrow to `preview` (a structural/governance shortfall that
/// blocks activation until cleared).
const PREVIEW_CLASS_REASONS: &[&str] = &[
    "governance_version_mismatch",
    "catalog_only_trust_not_enforcement_backed",
    "quota_axis_hard_breached",
    "crash_loop_window_breached",
    "restart_budget_exhausted",
    "contribution_not_nominal",
    "trust_tier_quarantined",
    "attribution_incomplete",
];

/// Reasons whose only effect is a safe narrowing to `beta`.
const BETA_CLASS_REASONS: &[&str] = &["quota_enforcement_unverified", "quota_axis_soft_breached"];

/// Closed set of consumer surfaces that ingest this packet.
pub const STABLE_WASM_HOST_GOVERNANCE_CONSUMER_SURFACES: &[&str] = &[
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

/// Input describing a stable Wasm-host-governance packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableWasmHostGovernanceInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity input.
    pub identity: GovernanceIdentityInput,
    /// Runtime-class declaration input.
    pub runtime_class_declaration: GovernanceRuntimeClassDeclarationInput,
    /// Wasm host quota axes.
    pub quota_axes: Vec<HostQuotaAxisInput>,
    /// Crash-loop governance input.
    pub crash_loop: CrashLoopGovernanceInput,
    /// Restart-budget governance input.
    pub restart_budget: RestartBudgetGovernanceInput,
    /// Quarantine-posture input (recovery, trigger rule, visibility).
    pub quarantine_posture: QuarantinePostureInput,
    /// Active-contribution inspector entries.
    #[serde(default)]
    pub contributions: Vec<GovernedContributionEntryInput>,
    /// Stability qualification claim input.
    pub claim: GovernanceQualificationClaimInput,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input for [`GovernanceIdentity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceIdentityInput {
    /// Ref to the runtime v1 beta admission contract this packet governs.
    pub runtime_contract_ref: String,
    /// Opaque extension identity ref.
    pub extension_identity_ref: String,
    /// Extension version declared by the manifest baseline.
    pub extension_version: String,
    /// Ref to the source package the contributions came from.
    pub source_package_ref: String,
    /// Governance contract version this row pins.
    pub governance_contract_version: u32,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

/// Input for [`GovernanceRuntimeClassDeclaration`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceRuntimeClassDeclarationInput {
    /// Published runtime class.
    pub runtime_class: String,
    /// Execution locus for the runtime class.
    pub execution_locus_class: String,
}

/// Input for one [`HostQuotaAxis`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostQuotaAxisInput {
    /// Quota axis.
    pub axis_class: String,
    /// Opaque ref to the declared bound for the axis.
    pub declared_limit_ref: String,
    /// Opaque ref to the observed peak evidence for the axis.
    pub observed_peak_ref: String,
    /// Enforcement state for the published bound.
    pub enforcement_state_class: String,
    /// Pressure class of the observed peak against the bound.
    pub pressure_class: String,
    /// Whether the axis is bounded. `false` is an unbounded quota and is refused
    /// from the stable line.
    pub bounded: bool,
}

/// Input for [`CrashLoopGovernance`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopGovernanceInput {
    /// Opaque ref to the crash-loop window definition.
    pub window_ref: String,
    /// Distinct failures observed in the current window.
    pub distinct_failures: u32,
    /// Distinct-failure count that trips a disable.
    pub disable_threshold: u32,
    /// Distinct-failure count that trips a quarantine.
    pub quarantine_threshold: u32,
    /// Current crash-loop state.
    pub state_class: String,
    /// Trigger-rule ref cited when the crash loop disables or quarantines.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_rule_ref: Option<String>,
}

/// Input for [`RestartBudgetGovernance`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartBudgetGovernanceInput {
    /// Restart posture.
    pub restart_posture_class: String,
    /// Restart attempts used in the current host session.
    pub attempts_used: u32,
    /// Restart attempts remaining under budget.
    pub attempts_remaining: u32,
}

/// Input for [`QuarantinePosture`]. The quarantine state itself is derived from
/// the governance posture; the host supplies the recovery path, trigger-rule ref,
/// and visibility surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuarantinePostureInput {
    /// Recovery precondition class describing the path back to nominal.
    pub recovery_precondition_class: String,
    /// Trigger-rule ref cited when the host is disabled or quarantined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_rule_ref: Option<String>,
    /// Visibility surface for a non-nominal posture.
    pub visibility_surface_class: String,
}

/// Input for one [`GovernedContributionEntry`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernedContributionEntryInput {
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

/// Input for [`GovernanceQualificationClaim`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceQualificationClaimInput {
    /// Governance tier claimed by the row.
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
pub struct GovernanceIdentity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ref to the runtime v1 beta admission contract this packet governs.
    pub runtime_contract_ref: String,
    /// Opaque extension identity ref.
    pub extension_identity_ref: String,
    /// Extension version.
    pub extension_version: String,
    /// Source package the contributions came from.
    pub source_package_ref: String,
    /// Governance contract version this row pins.
    pub governance_contract_version: u32,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

impl GovernanceIdentity {
    /// Returns true when the row pins the published stable governance version.
    pub fn governance_version_current(&self) -> bool {
        self.governance_contract_version == STABLE_GOVERNANCE_PUBLISHED_VERSION
    }

    /// Returns true when the lifecycle is runnable.
    pub fn lifecycle_runnable(&self) -> bool {
        RUNNABLE_LIFECYCLE_STATES.contains(&self.lifecycle_state_class.as_str())
    }
}

/// Published runtime-class declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceRuntimeClassDeclaration {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Published runtime class.
    pub runtime_class: String,
    /// Execution locus for the runtime class.
    pub execution_locus_class: String,
}

/// One Wasm host quota axis binding plus its observed pressure and enforcement
/// state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostQuotaAxis {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Quota axis.
    pub axis_class: String,
    /// Opaque ref to the declared bound for the axis.
    pub declared_limit_ref: String,
    /// Opaque ref to the observed peak evidence for the axis.
    pub observed_peak_ref: String,
    /// Enforcement state for the published bound.
    pub enforcement_state_class: String,
    /// Pressure class of the observed peak against the bound.
    pub pressure_class: String,
    /// Whether the axis is bounded.
    pub bounded: bool,
}

impl HostQuotaAxis {
    /// Returns true when the axis is bounded and enforced as published.
    pub fn enforced_as_published(&self) -> bool {
        self.bounded && self.enforcement_state_class == "enforced_as_published"
    }

    /// Returns true when the axis is unbounded or its enforcement cannot be
    /// attested at all.
    pub fn is_unbounded(&self) -> bool {
        !self.bounded || self.enforcement_state_class == "unenforceable_refused"
    }
}

/// Crash-loop governance window plus its trip thresholds and current state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopGovernance {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque ref to the crash-loop window definition.
    pub window_ref: String,
    /// Distinct failures observed in the current window.
    pub distinct_failures: u32,
    /// Distinct-failure count that trips a disable.
    pub disable_threshold: u32,
    /// Distinct-failure count that trips a quarantine.
    pub quarantine_threshold: u32,
    /// Current crash-loop state.
    pub state_class: String,
    /// Trigger-rule ref cited when the crash loop disables or quarantines.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_rule_ref: Option<String>,
}

impl CrashLoopGovernance {
    /// Returns true when the crash-loop window has tripped a disable.
    pub fn disable_tripped(&self) -> bool {
        matches!(self.state_class.as_str(), "disable_tripped" | "quarantine_tripped")
    }

    /// Returns true when the crash-loop window has tripped a quarantine.
    pub fn quarantine_tripped(&self) -> bool {
        self.state_class == "quarantine_tripped"
    }
}

/// Restart-budget governance snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartBudgetGovernance {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Restart posture.
    pub restart_posture_class: String,
    /// Restart attempts used in the current host session.
    pub attempts_used: u32,
    /// Restart attempts remaining under budget.
    pub attempts_remaining: u32,
}

impl RestartBudgetGovernance {
    /// Returns true when the restart posture is unbounded and refused.
    pub fn posture_unbounded(&self) -> bool {
        self.restart_posture_class == "restart_budget_unbounded_refused"
    }

    /// Returns true when the restart budget is exhausted (attempts used with none
    /// remaining).
    pub fn exhausted(&self) -> bool {
        self.attempts_remaining == 0 && self.attempts_used > 0
    }
}

/// Quarantine posture derived from the governance posture plus the host-supplied
/// recovery path, trigger rule, and visibility surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuarantinePosture {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Derived quarantine state.
    pub quarantine_state_class: String,
    /// Recovery precondition class describing the path back to nominal.
    pub recovery_precondition_class: String,
    /// Trigger-rule ref cited when the host is disabled or quarantined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_rule_ref: Option<String>,
    /// Visibility surface for a non-nominal posture.
    pub visibility_surface_class: String,
    /// True when activation is blocked by the posture.
    pub blocks_activation: bool,
}

/// One active-contribution inspector entry. Always carries source package,
/// runtime class, execution locus, trust tier, used permissions, and the
/// last-known-good host — even when quarantined, bridged, or downgraded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernedContributionEntry {
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

impl GovernedContributionEntry {
    /// Returns true when the entry is fully attributed for the inspector.
    pub fn is_attributed(&self) -> bool {
        !self.source_package_ref.trim().is_empty()
            && !self.last_known_good_host_ref.trim().is_empty()
            && !self.contribution_id.trim().is_empty()
    }

    /// Returns true when the contribution is running nominally.
    pub fn is_nominal(&self) -> bool {
        self.host_state_class == "running_nominal"
    }
}

/// Stability qualification claim after the governance posture is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceQualificationClaim {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Governance tier claimed by the row.
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
pub struct GovernanceDowngradedHostBanner {
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
pub struct StableWasmHostGovernanceInspection {
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
    /// Effective governance tier.
    pub effective_tier: String,
    /// True when the claim is a stable governance claim.
    pub stable_claim: bool,
    /// True when the row pins the published governance version.
    pub governance_version_current: bool,
    /// True when every quota axis is bounded and enforced as published.
    pub all_quotas_enforced_as_published: bool,
    /// Always false; surfaced so a reviewer can see an unbounded quota is forbidden
    /// on the stable line.
    pub allows_unbounded_quota: bool,
    /// Number of quota axes.
    pub quota_axis_count: usize,
    /// Number of quota axes at soft/hard breach pressure.
    pub breached_quota_axis_count: usize,
    /// Crash-loop state.
    pub crash_loop_state_class: String,
    /// Distinct crash-loop failures observed in the window.
    pub crash_loop_distinct_failures: u32,
    /// Restart posture.
    pub restart_posture_class: String,
    /// Restart attempts remaining.
    pub restart_attempts_remaining: u32,
    /// True when the restart budget is exhausted.
    pub restart_budget_exhausted: bool,
    /// Derived quarantine state.
    pub quarantine_state_class: String,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// True when the lifecycle is runnable.
    pub lifecycle_runnable: bool,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// True when a downgraded-host banner is required.
    pub downgraded_host_banner_required: bool,
    /// True when activation is blocked.
    pub blocks_activation: bool,
    /// True when every active contribution is fully attributed.
    pub attribution_complete: bool,
    /// Number of active contributions.
    pub active_contribution_count: usize,
    /// Number of quarantined/failed contributions.
    pub quarantined_contribution_count: usize,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Stable Wasm-host-governance packet consumed by install review, the runtime
/// inspector, the quarantine flow, diagnostics, support export, docs/help, and
/// release packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableWasmHostGovernancePacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity.
    pub identity: GovernanceIdentity,
    /// Runtime-class declaration.
    pub runtime_class_declaration: GovernanceRuntimeClassDeclaration,
    /// Wasm host quota axes.
    pub quota_axes: Vec<HostQuotaAxis>,
    /// Crash-loop governance.
    pub crash_loop: CrashLoopGovernance,
    /// Restart-budget governance.
    pub restart_budget: RestartBudgetGovernance,
    /// Quarantine posture.
    pub quarantine_posture: QuarantinePosture,
    /// Active-contribution inspector entries.
    pub contributions: Vec<GovernedContributionEntry>,
    /// Stability qualification claim after the posture is applied.
    pub claim: GovernanceQualificationClaim,
    /// Downgraded-host banner requirement.
    pub downgraded_host_banner: GovernanceDowngradedHostBanner,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the packet cites.
    pub source_schema_refs: Vec<String>,
    /// False so an unbounded Wasm host quota can never ride the stable line.
    pub allows_unbounded_quota: bool,
    /// False so an unbounded restart posture can never ride the stable line.
    pub allows_unbounded_restart: bool,
    /// False so catalog-only trust cannot back a stable governance claim.
    pub allows_catalog_only_trust: bool,
    /// False so a crash-loop disable/quarantine can never be silent.
    pub allows_silent_quarantine: bool,
    /// Inspection row.
    pub inspection: StableWasmHostGovernanceInspection,
}

impl StableWasmHostGovernancePacket {
    /// Builds a stable Wasm-host-governance packet from input, applying the
    /// governance posture to the claimed tier so any required downgrade below
    /// Stable is automatic.
    ///
    /// # Errors
    ///
    /// Returns [`StableWasmHostGovernanceValidationError`] when the input violates
    /// an identity, runtime-class, quota, crash-loop, restart, quarantine,
    /// contribution, or claim invariant.
    pub fn from_input(
        input: StableWasmHostGovernanceInput,
    ) -> Result<Self, StableWasmHostGovernanceValidationError> {
        validate_input(&input)?;

        let identity = identity_record(&input.identity);
        let runtime_class_declaration = runtime_class_record(&input.runtime_class_declaration);
        let quota_axes: Vec<HostQuotaAxis> = input.quota_axes.iter().map(quota_axis_record).collect();
        let crash_loop = crash_loop_record(&input.crash_loop);
        let restart_budget = restart_budget_record(&input.restart_budget);
        let contributions: Vec<GovernedContributionEntry> =
            input.contributions.iter().map(contribution_record).collect();

        let attribution_complete = !identity.source_package_ref.trim().is_empty()
            && contributions.iter().all(|c| c.is_attributed());
        let reasons = derive_downgrade_reasons(
            &input.claim.claimed_tier,
            &input.claim.claim_basis_class,
            &identity,
            &quota_axes,
            &crash_loop,
            &restart_budget,
            &contributions,
            attribution_complete,
        );
        let quarantine_state = derive_quarantine_state(&reasons);
        let blocks_activation = matches!(
            quarantine_state,
            "disabled_until_next_session" | "quarantined"
        );
        let quarantine_posture = quarantine_record(
            &input.quarantine_posture,
            quarantine_state,
            blocks_activation,
        );
        let claim = claim_record(&input.claim, &reasons);
        let downgraded_host_banner = banner_record(
            &identity,
            &quota_axes,
            &crash_loop,
            &restart_budget,
            &contributions,
        );
        let inspection = inspection_record(
            &input.packet_id,
            &runtime_class_declaration,
            &identity,
            &quota_axes,
            &crash_loop,
            &restart_budget,
            &quarantine_posture,
            &contributions,
            &claim,
            &downgraded_host_banner,
            attribution_complete,
        );

        let packet = Self {
            record_kind: STABLE_WASM_HOST_GOVERNANCE_PACKET_RECORD_KIND.to_string(),
            schema_version: STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            identity,
            runtime_class_declaration,
            quota_axes,
            crash_loop,
            restart_budget,
            quarantine_posture,
            contributions,
            claim,
            downgraded_host_banner,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![STABLE_WASM_HOST_GOVERNANCE_SCHEMA_REF.to_string()],
            allows_unbounded_quota: false,
            allows_unbounded_restart: false,
            allows_catalog_only_trust: false,
            allows_silent_quarantine: false,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the stable Wasm-host-governance invariants.
    ///
    /// # Errors
    ///
    /// Returns [`StableWasmHostGovernanceValidationError`] when an invariant is
    /// violated.
    pub fn validate(&self) -> Result<(), StableWasmHostGovernanceValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            STABLE_WASM_HOST_GOVERNANCE_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_identity(&self.identity)?;
        validate_runtime_class(&self.runtime_class_declaration)?;
        validate_quota_axes(&self.quota_axes)?;
        validate_crash_loop(&self.crash_loop)?;
        validate_restart_budget(&self.restart_budget)?;
        validate_quarantine_posture(&self.quarantine_posture)?;
        for entry in &self.contributions {
            validate_contribution(entry)?;
        }
        validate_claim(&self.claim)?;
        validate_banner(&self.downgraded_host_banner)?;

        for surface in &self.consumer_surfaces {
            ensure_token(
                STABLE_WASM_HOST_GOVERNANCE_CONSUMER_SURFACES,
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
            .any(|r| r == STABLE_WASM_HOST_GOVERNANCE_SCHEMA_REF)
        {
            return Err(err("packet must cite its source schema"));
        }

        // No unbounded quota, unbounded restart, catalog-only trust, or silent
        // quarantine may ride a published stable governance row.
        if self.allows_unbounded_quota
            || self.allows_unbounded_restart
            || self.allows_catalog_only_trust
            || self.allows_silent_quarantine
        {
            return Err(err(
                "a stable governance packet must not allow unbounded quota, unbounded restart, catalog-only trust, or silent quarantine",
            ));
        }

        let attribution_complete = self.attribution_complete();

        // Stable-claim binding: a stable effective tier must pin the published
        // governance contract, be enforcement-backed, hold every quota axis
        // bounded and enforced as published with no breach, keep the crash-loop
        // window clear, keep the restart budget bounded and not exhausted, keep
        // the quarantine posture nominal, keep the trust tier out of quarantine,
        // stay runnable, keep every contribution nominal, and be fully attributed.
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            if !self.identity.governance_version_current() {
                return Err(err(
                    "stable effective tier must pin the published governance contract version",
                ));
            }
            if self.claim.claim_basis_class != "enforcement_backed" {
                return Err(err(
                    "stable effective tier must be enforcement-backed, not catalog-asserted",
                ));
            }
            if self.quota_axes.iter().any(|a| !a.enforced_as_published()) {
                return Err(err(
                    "stable effective tier must hold every quota axis bounded and enforced as published",
                ));
            }
            if self
                .quota_axes
                .iter()
                .any(|a| matches!(a.pressure_class.as_str(), "soft_breach" | "hard_breach"))
            {
                return Err(err(
                    "stable effective tier must not carry a breached quota axis",
                ));
            }
            if self.crash_loop.state_class != "nominal" {
                return Err(err(
                    "stable effective tier must keep the crash-loop window nominal",
                ));
            }
            if self.restart_budget.posture_unbounded() || self.restart_budget.exhausted() {
                return Err(err(
                    "stable effective tier must keep a bounded, non-exhausted restart budget",
                ));
            }
            if self.quarantine_posture.quarantine_state_class != "none_nominal" {
                return Err(err(
                    "stable effective tier must keep the quarantine posture nominal",
                ));
            }
            if self.identity.publisher_trust_tier_class == "quarantined" {
                return Err(err(
                    "stable effective tier must not carry a quarantined trust tier",
                ));
            }
            if !self.identity.lifecycle_runnable() {
                return Err(err("stable effective tier must stay on a runnable lifecycle"));
            }
            if self.contributions.iter().any(|c| !c.is_nominal()) {
                return Err(err(
                    "stable effective tier must not carry a non-nominal contribution",
                ));
            }
            if !attribution_complete {
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
        let reasons = derive_downgrade_reasons(
            &self.claim.claimed_tier,
            &self.claim.claim_basis_class,
            &self.identity,
            &self.quota_axes,
            &self.crash_loop,
            &self.restart_budget,
            &self.contributions,
            attribution_complete,
        );
        let derived = derive_effective_tier(&self.claim.claimed_tier, &reasons);
        if derived.effective_tier != self.claim.effective_tier {
            return Err(err(
                "stored effective tier does not match the posture-derived tier",
            ));
        }
        if derived.support_claim != self.claim.support_claim_class {
            return Err(err(
                "stored support claim does not match the posture-derived support claim",
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
        let expected: BTreeSet<&str> = derived.downgrade_reasons.iter().map(String::as_str).collect();
        if stored != expected {
            return Err(err(
                "stored downgrade reasons do not match the posture-derived reasons",
            ));
        }

        // Quarantine-state truth: the stored quarantine state must match the
        // posture-derived state, and it must surface when non-nominal.
        let derived_state = derive_quarantine_state(&reasons);
        if self.quarantine_posture.quarantine_state_class != derived_state {
            return Err(err(
                "stored quarantine state does not match the posture-derived state",
            ));
        }
        let derived_blocks = matches!(derived_state, "disabled_until_next_session" | "quarantined");
        if self.quarantine_posture.blocks_activation != derived_blocks {
            return Err(err(
                "stored quarantine blocks_activation does not match the posture-derived state",
            ));
        }

        // Banner truth: a banner must be raised exactly when the host posture is
        // downgraded, and never silently suppressed.
        let banner_required = host_is_downgraded(
            &self.identity,
            &self.quota_axes,
            &self.crash_loop,
            &self.restart_budget,
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

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableWasmHostGovernanceProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Extension identity.
    pub extension_identity_ref: String,
    /// Runtime class.
    pub runtime_class: String,
    /// Claimed tier.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim is a stable governance claim.
    pub stable_claim: bool,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// Downgrade reasons.
    pub downgrade_reasons: Vec<String>,
    /// Derived quarantine state.
    pub quarantine_state_class: String,
    /// True when a downgraded-host banner is required.
    pub downgraded_host_banner_required: bool,
    /// True when activation is blocked.
    pub blocks_activation: bool,
    /// Number of active contributions.
    pub active_contribution_count: usize,
}

impl From<StableWasmHostGovernancePacket> for StableWasmHostGovernanceProjection {
    fn from(packet: StableWasmHostGovernancePacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            extension_identity_ref: packet.identity.extension_identity_ref,
            runtime_class: packet.runtime_class_declaration.runtime_class,
            claimed_tier: packet.claim.claimed_tier,
            effective_tier: packet.claim.effective_tier,
            support_claim_class: packet.claim.support_claim_class,
            stable_claim: packet.inspection.stable_claim,
            downgraded: packet.claim.downgraded,
            downgrade_reasons: packet.claim.downgrade_reasons,
            quarantine_state_class: packet.quarantine_posture.quarantine_state_class,
            downgraded_host_banner_required: packet.downgraded_host_banner.must_display,
            blocks_activation: packet.quarantine_posture.blocks_activation,
            active_contribution_count: packet.contributions.len(),
        }
    }
}

/// Parses and validates a materialized packet, returning the compact projection.
///
/// # Errors
///
/// Returns [`StableWasmHostGovernanceError`] when the payload fails to parse or
/// violates the stable Wasm-host-governance invariants.
pub fn project_stable_wasm_host_governance(
    payload: &str,
) -> Result<StableWasmHostGovernanceProjection, StableWasmHostGovernanceError> {
    let packet: StableWasmHostGovernancePacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(StableWasmHostGovernanceProjection::from(packet))
}

// ---------------------------------------------------------------------------
// Support export projection
// ---------------------------------------------------------------------------

/// Metadata-safe support/partner export row that quotes the same closed tokens as
/// the packet without leaking raw manifest, profile, or runtime-payload bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableWasmHostGovernanceSupportExport {
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
    /// Derived quarantine state.
    pub quarantine_state_class: String,
    /// Recovery precondition class.
    pub recovery_precondition_class: String,
    /// Trigger-rule ref cited for a disable/quarantine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_rule_ref: Option<String>,
    /// True when a downgraded-host banner is required.
    pub downgraded_host_banner_required: bool,
    /// True when activation is blocked.
    pub blocks_activation: bool,
    /// Number of quota axes.
    pub quota_axis_count: usize,
    /// Number of breached quota axes.
    pub breached_quota_axis_count: usize,
    /// Crash-loop state.
    pub crash_loop_state_class: String,
    /// Crash-loop distinct failures.
    pub crash_loop_distinct_failures: u32,
    /// Restart posture.
    pub restart_posture_class: String,
    /// Restart attempts remaining.
    pub restart_attempts_remaining: u32,
    /// Number of active contributions.
    pub active_contribution_count: usize,
    /// Number of quarantined/failed contributions.
    pub quarantined_contribution_count: usize,
    /// Export-safe summary suitable for support/partner consumers.
    pub export_safe_summary: String,
}

/// Projects a packet into the metadata-safe support/partner export row.
pub fn project_stable_wasm_host_governance_support_export(
    packet: &StableWasmHostGovernancePacket,
) -> StableWasmHostGovernanceSupportExport {
    let export_safe_summary = format!(
        "{} Runtime class={}. Quotas: {} axes, {} breached, all_enforced={}. Crash loop={} failures={}/{}. Restart posture={} remaining={} exhausted={}. Quarantine={} (blocks_activation={}). Trust={} lifecycle={}. Tier claimed={} effective={} (downgraded={}). Banner required={}.",
        packet.claim.summary_label,
        packet.runtime_class_declaration.runtime_class,
        packet.inspection.quota_axis_count,
        packet.inspection.breached_quota_axis_count,
        packet.inspection.all_quotas_enforced_as_published,
        packet.crash_loop.state_class,
        packet.crash_loop.distinct_failures,
        packet.crash_loop.quarantine_threshold,
        packet.restart_budget.restart_posture_class,
        packet.restart_budget.attempts_remaining,
        packet.restart_budget.exhausted(),
        packet.quarantine_posture.quarantine_state_class,
        packet.quarantine_posture.blocks_activation,
        packet.identity.publisher_trust_tier_class,
        packet.identity.lifecycle_state_class,
        packet.claim.claimed_tier,
        packet.claim.effective_tier,
        packet.claim.downgraded,
        packet.downgraded_host_banner.must_display,
    );

    StableWasmHostGovernanceSupportExport {
        record_kind: STABLE_WASM_HOST_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION,
        export_id: format!("stable_wasm_host_governance_support_export:{}", packet.packet_id),
        packet_ref: packet.packet_id.clone(),
        extension_identity_ref: packet.identity.extension_identity_ref.clone(),
        extension_version: packet.identity.extension_version.clone(),
        source_package_ref: packet.identity.source_package_ref.clone(),
        runtime_class: packet.runtime_class_declaration.runtime_class.clone(),
        trust_tier_class: packet.identity.publisher_trust_tier_class.clone(),
        lifecycle_state_class: packet.identity.lifecycle_state_class.clone(),
        effective_tier: packet.claim.effective_tier.clone(),
        support_claim_class: packet.claim.support_claim_class.clone(),
        downgraded: packet.claim.downgraded,
        downgrade_reasons: packet.claim.downgrade_reasons.clone(),
        quarantine_state_class: packet.quarantine_posture.quarantine_state_class.clone(),
        recovery_precondition_class: packet.quarantine_posture.recovery_precondition_class.clone(),
        trigger_rule_ref: packet.quarantine_posture.trigger_rule_ref.clone(),
        downgraded_host_banner_required: packet.downgraded_host_banner.must_display,
        blocks_activation: packet.quarantine_posture.blocks_activation,
        quota_axis_count: packet.inspection.quota_axis_count,
        breached_quota_axis_count: packet.inspection.breached_quota_axis_count,
        crash_loop_state_class: packet.crash_loop.state_class.clone(),
        crash_loop_distinct_failures: packet.crash_loop.distinct_failures,
        restart_posture_class: packet.restart_budget.restart_posture_class.clone(),
        restart_attempts_remaining: packet.restart_budget.attempts_remaining,
        active_contribution_count: packet.inspection.active_contribution_count,
        quarantined_contribution_count: packet.inspection.quarantined_contribution_count,
        export_safe_summary,
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for stable Wasm-host-governance operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableWasmHostGovernanceError {
    /// Validation failed.
    Validation(StableWasmHostGovernanceValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for StableWasmHostGovernanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for StableWasmHostGovernanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for stable Wasm-host-governance packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableWasmHostGovernanceValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for StableWasmHostGovernanceValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StableWasmHostGovernanceValidationError {}

impl StableWasmHostGovernanceValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<serde_json::Error> for StableWasmHostGovernanceError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(StableWasmHostGovernanceValidationError {
            message: err.to_string(),
        })
    }
}

impl From<StableWasmHostGovernanceValidationError> for StableWasmHostGovernanceError {
    fn from(err: StableWasmHostGovernanceValidationError) -> Self {
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

/// Collects the typed narrowing reasons for a claimed tier given the governance
/// posture. Returns an empty set for an already-honest non-stable claim.
#[allow(clippy::too_many_arguments)]
fn derive_downgrade_reasons(
    claimed_tier: &str,
    claim_basis: &str,
    identity: &GovernanceIdentity,
    quota_axes: &[HostQuotaAxis],
    crash_loop: &CrashLoopGovernance,
    restart_budget: &RestartBudgetGovernance,
    contributions: &[GovernedContributionEntry],
    attribution_complete: bool,
) -> Vec<String> {
    // Non-stable claims are already honest; they carry no narrowing reasons.
    if !STABLE_TIERS.contains(&claimed_tier) {
        return Vec::new();
    }

    let mut reasons: Vec<String> = Vec::new();

    if !identity.governance_version_current() {
        reasons.push("governance_version_mismatch".to_string());
    }
    if claim_basis != "enforcement_backed" {
        reasons.push("catalog_only_trust_not_enforcement_backed".to_string());
    }
    if quota_axes.iter().any(HostQuotaAxis::is_unbounded) {
        reasons.push("quota_unbounded_refused".to_string());
    }
    if quota_axes
        .iter()
        .any(|a| !a.is_unbounded() && a.enforcement_state_class == "fail_closed_downgraded")
    {
        reasons.push("quota_enforcement_unverified".to_string());
    }
    if quota_axes.iter().any(|a| a.pressure_class == "soft_breach") {
        reasons.push("quota_axis_soft_breached".to_string());
    }
    if quota_axes.iter().any(|a| a.pressure_class == "hard_breach") {
        reasons.push("quota_axis_hard_breached".to_string());
    }
    if crash_loop.quarantine_tripped() {
        reasons.push("crash_loop_quarantine_active".to_string());
    } else if matches!(crash_loop.state_class.as_str(), "window_open" | "disable_tripped") {
        reasons.push("crash_loop_window_breached".to_string());
    }
    if restart_budget.posture_unbounded() {
        reasons.push("restart_posture_unbounded".to_string());
    }
    if restart_budget.exhausted() {
        reasons.push("restart_budget_exhausted".to_string());
    }
    if contributions.iter().any(|c| !c.is_nominal()) {
        reasons.push("contribution_not_nominal".to_string());
    }
    if identity.publisher_trust_tier_class == "quarantined" {
        reasons.push("trust_tier_quarantined".to_string());
    }
    if !identity.lifecycle_runnable() {
        reasons.push("lifecycle_not_runnable".to_string());
    }
    if !attribution_complete {
        reasons.push("attribution_incomplete".to_string());
    }

    reasons.sort();
    reasons.dedup();
    reasons
}

/// Applies the collected narrowing reasons to a claimed tier.
fn derive_effective_tier(claimed_tier: &str, reasons: &[String]) -> DerivedTier {
    if !STABLE_TIERS.contains(&claimed_tier) {
        return DerivedTier {
            effective_tier: claimed_tier.to_string(),
            support_claim: support_claim_for(claimed_tier),
            downgraded: false,
            downgrade_reasons: Vec::new(),
        };
    }

    if reasons.is_empty() {
        DerivedTier {
            effective_tier: claimed_tier.to_string(),
            support_claim: support_claim_for(claimed_tier),
            downgraded: false,
            downgrade_reasons: Vec::new(),
        }
    } else {
        let effective = narrow_tier_for(reasons);
        DerivedTier {
            effective_tier: effective.to_string(),
            support_claim: support_claim_for(effective),
            downgraded: true,
            downgrade_reasons: reasons.to_vec(),
        }
    }
}

/// Picks the effective tier given the active narrowing reasons.
fn narrow_tier_for(reasons: &[String]) -> &'static str {
    if reasons.iter().any(|r| WITHDRAWN_CLASS_REASONS.contains(&r.as_str())) {
        "withdrawn"
    } else if reasons.iter().any(|r| PREVIEW_CLASS_REASONS.contains(&r.as_str())) {
        "preview"
    } else {
        debug_assert!(reasons.iter().all(|r| BETA_CLASS_REASONS.contains(&r.as_str())));
        "beta"
    }
}

/// Maps an effective tier to the support claim it may imply.
fn support_claim_for(tier: &str) -> String {
    match tier {
        "stable" => "stable_governance_ready_claim",
        "beta" => "beta_governance_partial_claim",
        "preview" => "preview_governance_experimental_claim",
        "withdrawn" => "withdrawn_no_governance_claim",
        _ => "preview_governance_experimental_claim",
    }
    .to_string()
}

/// Derives the quarantine state from the collected narrowing reasons.
fn derive_quarantine_state(reasons: &[String]) -> &'static str {
    if reasons.iter().any(|r| WITHDRAWN_CLASS_REASONS.contains(&r.as_str())) {
        "quarantined"
    } else if reasons.iter().any(|r| PREVIEW_CLASS_REASONS.contains(&r.as_str())) {
        "disabled_until_next_session"
    } else if reasons.iter().any(|r| BETA_CLASS_REASONS.contains(&r.as_str())) {
        "throttled"
    } else {
        "none_nominal"
    }
}

/// Returns true when the host posture is physically downgraded and a banner must
/// be shown. Delegates to [`banner_reason_for`] so the must-display flag and the
/// banner reason can never disagree. A claim-basis problem alone (version
/// mismatch, catalog-only trust, attribution gap) disables the row but does not
/// raise a *downgraded-host* banner, since the host itself is still nominal.
fn host_is_downgraded(
    identity: &GovernanceIdentity,
    quota_axes: &[HostQuotaAxis],
    crash_loop: &CrashLoopGovernance,
    restart_budget: &RestartBudgetGovernance,
    contributions: &[GovernedContributionEntry],
) -> bool {
    banner_reason_for(identity, quota_axes, crash_loop, restart_budget, contributions).is_some()
}

/// Picks the most-severe banner reason for a downgraded host posture.
fn banner_reason_for(
    identity: &GovernanceIdentity,
    quota_axes: &[HostQuotaAxis],
    crash_loop: &CrashLoopGovernance,
    restart_budget: &RestartBudgetGovernance,
    contributions: &[GovernedContributionEntry],
) -> Option<String> {
    if quota_axes.iter().any(HostQuotaAxis::is_unbounded) {
        return Some("quota_unbounded_refused".to_string());
    }
    if crash_loop.quarantine_tripped() {
        return Some("crash_loop_quarantine_active".to_string());
    }
    if restart_budget.posture_unbounded() {
        return Some("restart_posture_unbounded".to_string());
    }
    if !identity.lifecycle_runnable() {
        return Some("lifecycle_not_runnable".to_string());
    }
    if quota_axes.iter().any(|a| a.pressure_class == "hard_breach") {
        return Some("quota_axis_hard_breached".to_string());
    }
    if matches!(crash_loop.state_class.as_str(), "window_open" | "disable_tripped") {
        return Some("crash_loop_window_breached".to_string());
    }
    if restart_budget.exhausted() {
        return Some("restart_budget_exhausted".to_string());
    }
    if identity.publisher_trust_tier_class == "quarantined" {
        return Some("trust_tier_quarantined".to_string());
    }
    if contributions.iter().any(|c| !c.is_nominal()) {
        return Some("contribution_not_nominal".to_string());
    }
    if quota_axes
        .iter()
        .any(|a| a.enforcement_state_class == "fail_closed_downgraded")
    {
        return Some("quota_enforcement_unverified".to_string());
    }
    if quota_axes.iter().any(|a| a.pressure_class == "soft_breach") {
        return Some("quota_axis_soft_breached".to_string());
    }
    None
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn identity_record(input: &GovernanceIdentityInput) -> GovernanceIdentity {
    GovernanceIdentity {
        record_kind: GOVERNANCE_IDENTITY_RECORD_KIND.to_string(),
        schema_version: STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION,
        runtime_contract_ref: input.runtime_contract_ref.clone(),
        extension_identity_ref: input.extension_identity_ref.clone(),
        extension_version: input.extension_version.clone(),
        source_package_ref: input.source_package_ref.clone(),
        governance_contract_version: input.governance_contract_version,
        publisher_trust_tier_class: input.publisher_trust_tier_class.clone(),
        lifecycle_state_class: input.lifecycle_state_class.clone(),
    }
}

fn runtime_class_record(
    input: &GovernanceRuntimeClassDeclarationInput,
) -> GovernanceRuntimeClassDeclaration {
    GovernanceRuntimeClassDeclaration {
        record_kind: GOVERNANCE_RUNTIME_CLASS_DECLARATION_RECORD_KIND.to_string(),
        schema_version: STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION,
        runtime_class: input.runtime_class.clone(),
        execution_locus_class: input.execution_locus_class.clone(),
    }
}

fn quota_axis_record(input: &HostQuotaAxisInput) -> HostQuotaAxis {
    HostQuotaAxis {
        record_kind: HOST_QUOTA_AXIS_RECORD_KIND.to_string(),
        schema_version: STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION,
        axis_class: input.axis_class.clone(),
        declared_limit_ref: input.declared_limit_ref.clone(),
        observed_peak_ref: input.observed_peak_ref.clone(),
        enforcement_state_class: input.enforcement_state_class.clone(),
        pressure_class: input.pressure_class.clone(),
        bounded: input.bounded,
    }
}

fn crash_loop_record(input: &CrashLoopGovernanceInput) -> CrashLoopGovernance {
    CrashLoopGovernance {
        record_kind: CRASH_LOOP_GOVERNANCE_RECORD_KIND.to_string(),
        schema_version: STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION,
        window_ref: input.window_ref.clone(),
        distinct_failures: input.distinct_failures,
        disable_threshold: input.disable_threshold,
        quarantine_threshold: input.quarantine_threshold,
        state_class: input.state_class.clone(),
        trigger_rule_ref: input.trigger_rule_ref.clone(),
    }
}

fn restart_budget_record(input: &RestartBudgetGovernanceInput) -> RestartBudgetGovernance {
    RestartBudgetGovernance {
        record_kind: RESTART_BUDGET_GOVERNANCE_RECORD_KIND.to_string(),
        schema_version: STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION,
        restart_posture_class: input.restart_posture_class.clone(),
        attempts_used: input.attempts_used,
        attempts_remaining: input.attempts_remaining,
    }
}

fn quarantine_record(
    input: &QuarantinePostureInput,
    quarantine_state: &str,
    blocks_activation: bool,
) -> QuarantinePosture {
    QuarantinePosture {
        record_kind: QUARANTINE_POSTURE_RECORD_KIND.to_string(),
        schema_version: STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION,
        quarantine_state_class: quarantine_state.to_string(),
        recovery_precondition_class: input.recovery_precondition_class.clone(),
        trigger_rule_ref: input.trigger_rule_ref.clone(),
        visibility_surface_class: input.visibility_surface_class.clone(),
        blocks_activation,
    }
}

fn contribution_record(input: &GovernedContributionEntryInput) -> GovernedContributionEntry {
    GovernedContributionEntry {
        record_kind: GOVERNED_CONTRIBUTION_ENTRY_RECORD_KIND.to_string(),
        schema_version: STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION,
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
    input: &GovernanceQualificationClaimInput,
    reasons: &[String],
) -> GovernanceQualificationClaim {
    let derived = derive_effective_tier(&input.claimed_tier, reasons);
    GovernanceQualificationClaim {
        record_kind: GOVERNANCE_QUALIFICATION_CLAIM_RECORD_KIND.to_string(),
        schema_version: STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION,
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
    identity: &GovernanceIdentity,
    quota_axes: &[HostQuotaAxis],
    crash_loop: &CrashLoopGovernance,
    restart_budget: &RestartBudgetGovernance,
    contributions: &[GovernedContributionEntry],
) -> GovernanceDowngradedHostBanner {
    let must_display =
        host_is_downgraded(identity, quota_axes, crash_loop, restart_budget, contributions);
    let banner_reason_class = if must_display {
        banner_reason_for(identity, quota_axes, crash_loop, restart_budget, contributions)
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
            "Wasm host running under a narrower-than-published governance posture ({}).",
            banner_reason_class.clone().unwrap_or_default()
        )
    } else {
        "Wasm host enforcing the published quota, crash-loop, and restart-budget governance."
            .to_string()
    };
    GovernanceDowngradedHostBanner {
        record_kind: GOVERNANCE_DOWNGRADED_HOST_BANNER_RECORD_KIND.to_string(),
        schema_version: STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION,
        must_display,
        banner_reason_class,
        last_known_good_host_ref,
        summary_label,
    }
}

#[allow(clippy::too_many_arguments)]
fn inspection_record(
    packet_id: &str,
    runtime_class: &GovernanceRuntimeClassDeclaration,
    identity: &GovernanceIdentity,
    quota_axes: &[HostQuotaAxis],
    crash_loop: &CrashLoopGovernance,
    restart_budget: &RestartBudgetGovernance,
    quarantine_posture: &QuarantinePosture,
    contributions: &[GovernedContributionEntry],
    claim: &GovernanceQualificationClaim,
    banner: &GovernanceDowngradedHostBanner,
    attribution_complete: bool,
) -> StableWasmHostGovernanceInspection {
    let stable_claim = STABLE_TIERS.contains(&claim.effective_tier.as_str());
    let all_quotas_enforced_as_published =
        quota_axes.iter().all(HostQuotaAxis::enforced_as_published);
    let breached_quota_axis_count = quota_axes
        .iter()
        .filter(|a| matches!(a.pressure_class.as_str(), "soft_breach" | "hard_breach"))
        .count();
    let quarantined_contribution_count = contributions
        .iter()
        .filter(|c| matches!(c.host_state_class.as_str(), "quarantined" | "failed"))
        .count();

    StableWasmHostGovernanceInspection {
        record_kind: STABLE_WASM_HOST_GOVERNANCE_INSPECTION_RECORD_KIND.to_string(),
        schema_version: STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION,
        packet_id_ref: packet_id.to_string(),
        runtime_class: runtime_class.runtime_class.clone(),
        execution_locus_class: runtime_class.execution_locus_class.clone(),
        effective_tier: claim.effective_tier.clone(),
        stable_claim,
        governance_version_current: identity.governance_version_current(),
        all_quotas_enforced_as_published,
        allows_unbounded_quota: false,
        quota_axis_count: quota_axes.len(),
        breached_quota_axis_count,
        crash_loop_state_class: crash_loop.state_class.clone(),
        crash_loop_distinct_failures: crash_loop.distinct_failures,
        restart_posture_class: restart_budget.restart_posture_class.clone(),
        restart_attempts_remaining: restart_budget.attempts_remaining,
        restart_budget_exhausted: restart_budget.exhausted(),
        quarantine_state_class: quarantine_posture.quarantine_state_class.clone(),
        trust_tier_class: identity.publisher_trust_tier_class.clone(),
        lifecycle_runnable: identity.lifecycle_runnable(),
        downgraded: claim.downgraded,
        downgraded_host_banner_required: banner.must_display,
        blocks_activation: quarantine_posture.blocks_activation,
        attribution_complete,
        active_contribution_count: contributions.len(),
        quarantined_contribution_count,
        summary_label: claim.summary_label.clone(),
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &StableWasmHostGovernanceInput,
) -> Result<(), StableWasmHostGovernanceValidationError> {
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
    ensure_nonempty(&id.extension_identity_ref, "identity.extension_identity_ref")?;
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

    if input.quota_axes.is_empty() {
        return Err(err("packet must declare at least one Wasm host quota axis"));
    }
    let mut axis_classes = BTreeSet::new();
    for axis in &input.quota_axes {
        ensure_token(QUOTA_AXIS_CLASSES, &axis.axis_class, "quota_axis.axis_class")?;
        if !axis_classes.insert(&axis.axis_class) {
            return Err(err(format!("duplicate quota axis: {}", axis.axis_class)));
        }
        ensure_nonempty(&axis.declared_limit_ref, "quota_axis.declared_limit_ref")?;
        ensure_nonempty(&axis.observed_peak_ref, "quota_axis.observed_peak_ref")?;
        ensure_token(
            QUOTA_ENFORCEMENT_STATES,
            &axis.enforcement_state_class,
            "quota_axis.enforcement_state_class",
        )?;
        ensure_token(QUOTA_PRESSURE_CLASSES, &axis.pressure_class, "quota_axis.pressure_class")?;
    }

    let cl = &input.crash_loop;
    ensure_nonempty(&cl.window_ref, "crash_loop.window_ref")?;
    ensure_token(CRASH_LOOP_STATE_CLASSES, &cl.state_class, "crash_loop.state_class")?;
    if cl.disable_threshold == 0 || cl.quarantine_threshold == 0 {
        return Err(err("crash-loop thresholds must be at least 1"));
    }
    if cl.disable_threshold > cl.quarantine_threshold {
        return Err(err(
            "crash_loop.disable_threshold must be <= crash_loop.quarantine_threshold",
        ));
    }
    crash_loop_state_consistent(cl)?;
    if matches!(cl.state_class.as_str(), "disable_tripped" | "quarantine_tripped") {
        match cl.trigger_rule_ref.as_deref() {
            Some(t) if t.starts_with("quarantine_rule:") => {}
            Some(_) => {
                return Err(err("crash_loop.trigger_rule_ref must start with 'quarantine_rule:'"))
            }
            None => {
                return Err(err(
                    "a disabled/quarantined crash-loop state must cite a trigger_rule_ref",
                ))
            }
        }
    }

    let rb = &input.restart_budget;
    ensure_token(
        RESTART_POSTURE_CLASSES,
        &rb.restart_posture_class,
        "restart_budget.restart_posture_class",
    )?;

    let qp = &input.quarantine_posture;
    ensure_token(
        RECOVERY_PRECONDITION_CLASSES,
        &qp.recovery_precondition_class,
        "quarantine_posture.recovery_precondition_class",
    )?;
    ensure_token(
        VISIBILITY_SURFACE_CLASSES,
        &qp.visibility_surface_class,
        "quarantine_posture.visibility_surface_class",
    )?;
    if let Some(t) = qp.trigger_rule_ref.as_deref() {
        if !t.starts_with("quarantine_rule:") {
            return Err(err(
                "quarantine_posture.trigger_rule_ref must start with 'quarantine_rule:'",
            ));
        }
    }

    let mut contribution_ids = BTreeSet::new();
    for c in &input.contributions {
        ensure_nonempty(&c.contribution_id, "contribution.contribution_id")?;
        if !contribution_ids.insert(&c.contribution_id) {
            return Err(err(format!("duplicate contribution_id: {}", c.contribution_id)));
        }
        ensure_token(
            CONTRIBUTION_KIND_CLASSES,
            &c.contribution_kind_class,
            "contribution.contribution_kind_class",
        )?;
        ensure_nonempty(&c.source_package_ref, "contribution.source_package_ref")?;
        ensure_token(RUNTIME_CLASSES, &c.runtime_class, "contribution.runtime_class")?;
        ensure_token(
            EXECUTION_LOCUS_CLASSES,
            &c.execution_locus_class,
            "contribution.execution_locus_class",
        )?;
        ensure_token(TRUST_TIER_CLASSES, &c.trust_tier_class, "contribution.trust_tier_class")?;
        ensure_nonempty(&c.last_known_good_host_ref, "contribution.last_known_good_host_ref")?;
        ensure_token(
            CONTRIBUTION_HOST_STATE_CLASSES,
            &c.host_state_class,
            "contribution.host_state_class",
        )?;
    }

    let claim = &input.claim;
    ensure_token(STABILITY_TIERS, &claim.claimed_tier, "claim.claimed_tier")?;
    ensure_token(CLAIM_BASIS_CLASSES, &claim.claim_basis_class, "claim.claim_basis_class")?;

    for surface in &input.consumer_surfaces {
        ensure_token(STABLE_WASM_HOST_GOVERNANCE_CONSUMER_SURFACES, surface, "consumer_surface")?;
    }
    if input.consumer_surfaces.is_empty() {
        return Err(err("input must bind at least one consumer surface"));
    }

    Ok(())
}

fn validate_identity(
    identity: &GovernanceIdentity,
) -> Result<(), StableWasmHostGovernanceValidationError> {
    ensure_eq(
        identity.record_kind.as_str(),
        GOVERNANCE_IDENTITY_RECORD_KIND,
        "identity record_kind",
    )?;
    ensure_eq_u32(
        identity.schema_version,
        STABLE_WASM_HOST_GOVERNANCE_SCHEMA_VERSION,
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
    rc: &GovernanceRuntimeClassDeclaration,
) -> Result<(), StableWasmHostGovernanceValidationError> {
    ensure_eq(
        rc.record_kind.as_str(),
        GOVERNANCE_RUNTIME_CLASS_DECLARATION_RECORD_KIND,
        "runtime_class record_kind",
    )?;
    ensure_token(RUNTIME_CLASSES, &rc.runtime_class, "runtime_class")?;
    ensure_token(
        EXECUTION_LOCUS_CLASSES,
        &rc.execution_locus_class,
        "execution_locus_class",
    )?;
    if !runtime_class_supports_locus(&rc.runtime_class, &rc.execution_locus_class) {
        return Err(err("execution_locus_class is not reserved for runtime_class"));
    }
    Ok(())
}

fn validate_quota_axes(
    axes: &[HostQuotaAxis],
) -> Result<(), StableWasmHostGovernanceValidationError> {
    if axes.is_empty() {
        return Err(err("packet must declare at least one Wasm host quota axis"));
    }
    let mut axis_classes = BTreeSet::new();
    for axis in axes {
        ensure_eq(
            axis.record_kind.as_str(),
            HOST_QUOTA_AXIS_RECORD_KIND,
            "quota_axis record_kind",
        )?;
        ensure_token(QUOTA_AXIS_CLASSES, &axis.axis_class, "quota_axis axis_class")?;
        if !axis_classes.insert(axis.axis_class.as_str()) {
            return Err(err(format!("duplicate quota axis: {}", axis.axis_class)));
        }
        ensure_token(
            QUOTA_ENFORCEMENT_STATES,
            &axis.enforcement_state_class,
            "quota_axis enforcement_state_class",
        )?;
        ensure_token(QUOTA_PRESSURE_CLASSES, &axis.pressure_class, "quota_axis pressure_class")?;
    }
    Ok(())
}

fn validate_crash_loop(
    cl: &CrashLoopGovernance,
) -> Result<(), StableWasmHostGovernanceValidationError> {
    ensure_eq(
        cl.record_kind.as_str(),
        CRASH_LOOP_GOVERNANCE_RECORD_KIND,
        "crash_loop record_kind",
    )?;
    ensure_token(CRASH_LOOP_STATE_CLASSES, &cl.state_class, "crash_loop state_class")?;
    if cl.disable_threshold == 0 || cl.quarantine_threshold == 0 {
        return Err(err("crash-loop thresholds must be at least 1"));
    }
    if cl.disable_threshold > cl.quarantine_threshold {
        return Err(err(
            "crash_loop disable_threshold must be <= quarantine_threshold",
        ));
    }
    crash_loop_state_consistent(cl)?;
    Ok(())
}

fn crash_loop_state_consistent(
    cl: &impl CrashLoopGovernanceInputLike,
) -> Result<(), StableWasmHostGovernanceValidationError> {
    match cl.state_class() {
        "nominal" => {
            if cl.distinct_failures() >= cl.disable_threshold() {
                return Err(err(
                    "crash_loop state nominal requires distinct_failures < disable_threshold",
                ));
            }
        }
        "window_open" => {
            if cl.distinct_failures() == 0 || cl.distinct_failures() >= cl.disable_threshold() {
                return Err(err(
                    "crash_loop state window_open requires 0 < distinct_failures < disable_threshold",
                ));
            }
        }
        "disable_tripped" => {
            if cl.distinct_failures() < cl.disable_threshold()
                || cl.distinct_failures() >= cl.quarantine_threshold()
            {
                return Err(err(
                    "crash_loop state disable_tripped requires disable_threshold <= distinct_failures < quarantine_threshold",
                ));
            }
        }
        "quarantine_tripped" => {
            if cl.distinct_failures() < cl.quarantine_threshold() {
                return Err(err(
                    "crash_loop state quarantine_tripped requires distinct_failures >= quarantine_threshold",
                ));
            }
        }
        _ => {}
    }
    Ok(())
}

/// Minimal accessor trait so [`crash_loop_state_consistent`] can validate both the
/// input and the record without duplicating logic.
trait CrashLoopGovernanceInputLike {
    fn distinct_failures(&self) -> u32;
    fn disable_threshold(&self) -> u32;
    fn quarantine_threshold(&self) -> u32;
    fn state_class(&self) -> &str;
}

impl CrashLoopGovernanceInputLike for CrashLoopGovernanceInput {
    fn distinct_failures(&self) -> u32 {
        self.distinct_failures
    }
    fn disable_threshold(&self) -> u32 {
        self.disable_threshold
    }
    fn quarantine_threshold(&self) -> u32 {
        self.quarantine_threshold
    }
    fn state_class(&self) -> &str {
        &self.state_class
    }
}

impl CrashLoopGovernanceInputLike for CrashLoopGovernance {
    fn distinct_failures(&self) -> u32 {
        self.distinct_failures
    }
    fn disable_threshold(&self) -> u32 {
        self.disable_threshold
    }
    fn quarantine_threshold(&self) -> u32 {
        self.quarantine_threshold
    }
    fn state_class(&self) -> &str {
        &self.state_class
    }
}

fn validate_restart_budget(
    rb: &RestartBudgetGovernance,
) -> Result<(), StableWasmHostGovernanceValidationError> {
    ensure_eq(
        rb.record_kind.as_str(),
        RESTART_BUDGET_GOVERNANCE_RECORD_KIND,
        "restart_budget record_kind",
    )?;
    ensure_token(
        RESTART_POSTURE_CLASSES,
        &rb.restart_posture_class,
        "restart_budget restart_posture_class",
    )?;
    Ok(())
}

fn validate_quarantine_posture(
    qp: &QuarantinePosture,
) -> Result<(), StableWasmHostGovernanceValidationError> {
    ensure_eq(
        qp.record_kind.as_str(),
        QUARANTINE_POSTURE_RECORD_KIND,
        "quarantine_posture record_kind",
    )?;
    ensure_token(
        QUARANTINE_STATE_CLASSES,
        &qp.quarantine_state_class,
        "quarantine_posture quarantine_state_class",
    )?;
    ensure_token(
        RECOVERY_PRECONDITION_CLASSES,
        &qp.recovery_precondition_class,
        "quarantine_posture recovery_precondition_class",
    )?;
    ensure_token(
        VISIBILITY_SURFACE_CLASSES,
        &qp.visibility_surface_class,
        "quarantine_posture visibility_surface_class",
    )?;
    let nominal = qp.quarantine_state_class == "none_nominal";
    // A nominal posture is not recovering and is not visible as a non-nominal row.
    if nominal && qp.recovery_precondition_class != "none_not_recovering" {
        return Err(err(
            "a nominal quarantine posture must keep recovery_precondition_class none_not_recovering",
        ));
    }
    if nominal && qp.visibility_surface_class != "not_visible_nominal_row" {
        return Err(err(
            "a nominal quarantine posture must keep visibility_surface_class not_visible_nominal_row",
        ));
    }
    // A non-nominal posture must surface on at least one user-facing surface and
    // never silently sit on a nominal row.
    if !nominal && qp.visibility_surface_class == "not_visible_nominal_row" {
        return Err(err(
            "a non-nominal quarantine posture must surface on a user-facing surface",
        ));
    }
    // A quarantined/disabled posture must cite a trigger rule.
    if matches!(
        qp.quarantine_state_class.as_str(),
        "disabled_until_next_session" | "quarantined"
    ) && qp.trigger_rule_ref.is_none()
    {
        return Err(err(
            "a disabled/quarantined posture must cite a trigger_rule_ref",
        ));
    }
    if let Some(t) = qp.trigger_rule_ref.as_deref() {
        if !t.starts_with("quarantine_rule:") {
            return Err(err(
                "quarantine_posture trigger_rule_ref must start with 'quarantine_rule:'",
            ));
        }
    }
    Ok(())
}

fn validate_contribution(
    c: &GovernedContributionEntry,
) -> Result<(), StableWasmHostGovernanceValidationError> {
    ensure_eq(
        c.record_kind.as_str(),
        GOVERNED_CONTRIBUTION_ENTRY_RECORD_KIND,
        "contribution record_kind",
    )?;
    ensure_token(
        CONTRIBUTION_KIND_CLASSES,
        &c.contribution_kind_class,
        "contribution contribution_kind_class",
    )?;
    ensure_token(RUNTIME_CLASSES, &c.runtime_class, "contribution runtime_class")?;
    ensure_token(
        EXECUTION_LOCUS_CLASSES,
        &c.execution_locus_class,
        "contribution execution_locus_class",
    )?;
    ensure_token(TRUST_TIER_CLASSES, &c.trust_tier_class, "contribution trust_tier_class")?;
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
    claim: &GovernanceQualificationClaim,
) -> Result<(), StableWasmHostGovernanceValidationError> {
    ensure_eq(
        claim.record_kind.as_str(),
        GOVERNANCE_QUALIFICATION_CLAIM_RECORD_KIND,
        "claim record_kind",
    )?;
    ensure_token(STABILITY_TIERS, &claim.claimed_tier, "claim claimed_tier")?;
    ensure_token(STABILITY_TIERS, &claim.effective_tier, "claim effective_tier")?;
    ensure_token(
        SUPPORT_CLAIM_CLASSES,
        &claim.support_claim_class,
        "claim support_claim_class",
    )?;
    ensure_token(CLAIM_BASIS_CLASSES, &claim.claim_basis_class, "claim claim_basis_class")?;
    for reason in &claim.downgrade_reasons {
        ensure_token(GOVERNANCE_DOWNGRADE_REASONS, reason, "claim downgrade_reason")?;
    }
    Ok(())
}

fn validate_banner(
    banner: &GovernanceDowngradedHostBanner,
) -> Result<(), StableWasmHostGovernanceValidationError> {
    ensure_eq(
        banner.record_kind.as_str(),
        GOVERNANCE_DOWNGRADED_HOST_BANNER_RECORD_KIND,
        "banner record_kind",
    )?;
    if let Some(reason) = &banner.banner_reason_class {
        ensure_token(GOVERNANCE_DOWNGRADE_REASONS, reason, "banner banner_reason_class")?;
        if !banner.must_display {
            return Err(err("banner_reason_class is set but must_display is false"));
        }
    } else if banner.must_display {
        return Err(err("must_display is true but no banner_reason_class is set"));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &StableWasmHostGovernanceInspection,
    packet: &StableWasmHostGovernancePacket,
) -> Result<(), StableWasmHostGovernanceValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        STABLE_WASM_HOST_GOVERNANCE_INSPECTION_RECORD_KIND,
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
        return Err(err("inspection active_contribution_count must match contributions"));
    }
    if inspection.quota_axis_count != packet.quota_axes.len() {
        return Err(err("inspection quota_axis_count must match quota_axes"));
    }
    if inspection.downgraded != packet.claim.downgraded {
        return Err(err("inspection downgraded is inconsistent"));
    }
    if inspection.downgraded_host_banner_required != packet.downgraded_host_banner.must_display {
        return Err(err("inspection downgraded_host_banner_required is inconsistent"));
    }
    if inspection.blocks_activation != packet.quarantine_posture.blocks_activation {
        return Err(err("inspection blocks_activation is inconsistent"));
    }
    if inspection.quarantine_state_class != packet.quarantine_posture.quarantine_state_class {
        return Err(err("inspection quarantine_state_class is inconsistent"));
    }
    if inspection.attribution_complete != packet.attribution_complete() {
        return Err(err("inspection attribution_complete is inconsistent"));
    }
    if inspection.allows_unbounded_quota {
        return Err(err("inspection allows_unbounded_quota must be false"));
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
            | ("declarative_host_rendered_view", "host_rendered_no_extension_code")
            | (
                "wasm_capability_sandbox",
                "editor_in_process_isolated" | "dedicated_subprocess"
            )
            | ("external_host", "dedicated_subprocess" | "helper_binary")
            | ("compatibility_bridge", "bridged_foreign_runtime")
            | ("remote_side_component", "remote_agent")
    )
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn err(message: impl Into<String>) -> StableWasmHostGovernanceValidationError {
    StableWasmHostGovernanceValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(left: T, right: T, field: &str) -> Result<(), StableWasmHostGovernanceValidationError>
where
    T: PartialEq + fmt::Display,
{
    if left != right {
        return Err(err(format!("{field} mismatch: expected {right}, got {left}")));
    }
    Ok(())
}

fn ensure_eq_u32(
    left: u32,
    right: u32,
    field: &str,
) -> Result<(), StableWasmHostGovernanceValidationError> {
    if left != right {
        return Err(err(format!("{field} mismatch: expected {right}, got {left}")));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), StableWasmHostGovernanceValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), StableWasmHostGovernanceValidationError> {
    if !tokens.contains(&value) {
        return Err(err(format!("{field} must be one of {tokens:?}, got {value}")));
    }
    Ok(())
}
