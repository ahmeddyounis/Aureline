//! Harden extension performance inspection, budget enforcement, and the
//! user-visible cost explanation for the stable ecosystem line — one
//! evidence-backed, automatically-narrowing performance-budget packet whose
//! stability qualification is derived, not asserted.
//!
//! The beta install-review, runtime, and marketplace-catalog lanes own per-row
//! admission and catalog truth. The stable runtime-ABI, manifest, lifecycle-flow,
//! catalog-truth, mirror-import, and bridge-certification lanes own the published
//! truth a claimed stable row carries on each of those axes. This module owns the
//! layer that makes an extension's **runtime performance cost** inspectable,
//! **budget-enforced**, and **explained to the user**:
//!
//! - the **performance inspection** — the worst-case budget axis, the measured
//!   p50 / p95 cost, the sample count, the benchmark-lab trace and corpus
//!   metadata it was drawn from, the measurement freshness, and whether the trace
//!   is attested rather than self-reported,
//! - the **budget enforcement** — the published p50 / p95 budget ceilings, the
//!   budget status against them (`within_budget` / `over_budget` / `unbounded` /
//!   `not_measured`), the enforcement mode (`enforced` / `advisory` /
//!   `unenforced`), and the threshold-adjustment posture (`unchanged` /
//!   `tightened` / `narrowed` / `relaxed`),
//! - the **waiver hook** — the waiver state, ref, and authority that backs any
//!   intentional threshold tightening / narrowing / relaxation so a budget can
//!   never be quietly moved,
//! - the **user-visible cost explanation** — the headline cost class
//!   (`negligible` / `light` / `moderate` / `heavy` / `unbounded`), the dominant
//!   cost factor, whether a user-readable explanation is actually attached, and
//!   the explanation ref,
//! - the **permission posture** (declared-vs-effective refs, no-widening flag) so
//!   no ambient privilege rides a performance claim,
//! - the **compatibility** label (scorecard ref, verified flag),
//! - the **install posture** (install scope and disclosure, revocation posture,
//!   mirrorability, rollback support), and
//! - the **stability qualification** after the posture is applied.
//!
//! The central rule mirrors the rest of the stable line: a **stable**
//! performance-budget claim may never be implied from a catalog row or an
//! adjacent green row. A row that renders a `stable` badge must pin the published
//! performance-budget profile version, be evidence-backed (not catalog-asserted),
//! keep its publisher trust tier out of quarantine, stay on a runnable lifecycle,
//! keep its cost bounded and within the published p50 / p95 budget, keep the
//! budget actually enforced, keep its benchmark measurement fresh and attested,
//! carry an active waiver for any relaxed threshold, explain its cost to the user,
//! never widen permissions, keep its compatibility verified and not parity-limited
//! / unsupported, disclose its install scope, keep a clean revocation posture,
//! stay mirrorable, and be fully attributed. When any of those fails, the visible
//! tier is **automatically narrowed below Stable** (`beta`, `preview`, or
//! `withdrawn`) with machine-readable reasons.
//!
//! Three guardrails are encoded so they cannot be papered over:
//!
//! - **No unbounded activation cost.** An `unbounded` budget status withdraws the
//!   row outright; an `over_budget` status narrows to `beta`.
//! - **No catalog-only trust.** A `catalog_asserted_only` claim basis can never
//!   back a stable performance claim; it narrows below Stable. An `unenforced`
//!   budget — a published number with no enforcement behind it — narrows to
//!   `preview`.
//! - **No ambient extension privilege.** A permission set widened beyond the
//!   declared manifest withdraws the row outright.
//!
//! The companion schema lives at
//! [`schemas/extensions/stable_performance_budget.schema.json`](../../../../schemas/extensions/stable_performance_budget.schema.json).
//! Canonical fixtures live under
//! `fixtures/extensions/m4/harden-extension-performance-inspection-budget-enforcement-and-user/`.

use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every stable performance-budget record.
pub const STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION: u32 = 1;

/// The published, stable performance-budget profile version. A `stable` claim must
/// pin exactly this version; any other version narrows below Stable.
pub const STABLE_PERFORMANCE_BUDGET_PUBLISHED_PROFILE_VERSION: u32 = 1;

/// Canonical schema path the packet cites.
pub const STABLE_PERFORMANCE_BUDGET_SCHEMA_REF: &str =
    "schemas/extensions/stable_performance_budget.schema.json";

/// Record-kind tag for [`StablePerformanceBudgetPacket`].
pub const STABLE_PERFORMANCE_BUDGET_PACKET_RECORD_KIND: &str = "stable_performance_budget_packet";

/// Record-kind tag for [`PerformanceBudgetIdentity`].
pub const PERFORMANCE_BUDGET_IDENTITY_RECORD_KIND: &str = "stable_performance_budget_identity";

/// Record-kind tag for [`PerformanceMeasurement`].
pub const PERFORMANCE_MEASUREMENT_RECORD_KIND: &str = "stable_performance_measurement";

/// Record-kind tag for [`PerformanceBudgetEnforcement`].
pub const PERFORMANCE_BUDGET_ENFORCEMENT_RECORD_KIND: &str =
    "stable_performance_budget_enforcement";

/// Record-kind tag for [`PerformanceBudgetWaiver`].
pub const PERFORMANCE_BUDGET_WAIVER_RECORD_KIND: &str = "stable_performance_budget_waiver";

/// Record-kind tag for [`PerformanceCostExplanation`].
pub const PERFORMANCE_COST_EXPLANATION_RECORD_KIND: &str = "stable_performance_cost_explanation";

/// Record-kind tag for [`PerformancePermissionPosture`].
pub const PERFORMANCE_PERMISSION_POSTURE_RECORD_KIND: &str =
    "stable_performance_permission_posture";

/// Record-kind tag for [`PerformanceCompatibility`].
pub const PERFORMANCE_COMPATIBILITY_RECORD_KIND: &str = "stable_performance_compatibility";

/// Record-kind tag for [`PerformanceInstallPosture`].
pub const PERFORMANCE_INSTALL_POSTURE_RECORD_KIND: &str = "stable_performance_install_posture";

/// Record-kind tag for [`PerformanceQualificationClaim`].
pub const PERFORMANCE_QUALIFICATION_CLAIM_RECORD_KIND: &str =
    "stable_performance_qualification_claim";

/// Record-kind tag for [`PerformanceDowngradedBanner`].
pub const PERFORMANCE_DOWNGRADED_BANNER_RECORD_KIND: &str = "stable_performance_downgraded_banner";

/// Record-kind tag for [`StablePerformanceBudgetInspection`].
pub const STABLE_PERFORMANCE_BUDGET_INSPECTION_RECORD_KIND: &str =
    "stable_performance_budget_inspection";

/// Record-kind tag for [`StablePerformanceBudgetSupportExport`].
pub const STABLE_PERFORMANCE_BUDGET_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stable_performance_budget_support_export";

/// Closed publisher-trust-tier vocabulary, shared with the rest of the stable line.
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

/// Lifecycle states a stable performance claim may keep (installable / runnable).
pub const RUNNABLE_LIFECYCLE_STATES: &[&str] =
    &["installed", "pending_activation", "active", "recovered"];

/// Closed budget-axis vocabulary — the performance dimension the worst-case cost is
/// measured on.
pub const BUDGET_AXIS_CLASSES: &[&str] = &[
    "activation_cold_start",
    "activation_warm_start",
    "main_thread_block",
    "background_cpu",
    "memory_footprint",
    "io_throughput",
    "host_rpc_latency",
    "render_frame",
];

/// Closed budget-status vocabulary. `within_budget` is the only status a stable
/// claim may keep.
pub const BUDGET_STATUS_CLASSES: &[&str] =
    &["within_budget", "over_budget", "unbounded", "not_measured"];

/// Closed measurement-freshness vocabulary. `fresh` is the only freshness a stable
/// claim may keep.
pub const MEASUREMENT_FRESHNESS_CLASSES: &[&str] = &["fresh", "stale", "expired", "not_measured"];

/// Closed enforcement-mode vocabulary. `enforced` is the only mode a stable claim
/// may keep.
pub const ENFORCEMENT_MODE_CLASSES: &[&str] = &["enforced", "advisory", "unenforced"];

/// Closed threshold-adjustment vocabulary. A `relaxed` threshold requires an active
/// waiver; a `tightened` / `narrowed` threshold requires a recorded waiver hook.
pub const THRESHOLD_ADJUSTMENT_CLASSES: &[&str] =
    &["unchanged", "tightened", "narrowed", "relaxed"];

/// Closed waiver-state vocabulary.
pub const WAIVER_STATE_CLASSES: &[&str] = &["none", "active", "expired", "revoked"];

/// Closed waiver-authority vocabulary — who signed the budget waiver.
pub const WAIVER_AUTHORITY_CLASSES: &[&str] = &["publisher", "admin", "release_engineering"];

/// Closed cost-class vocabulary — the user-visible headline cost. `unbounded` may
/// never ride a stable claim.
pub const COST_CLASSES: &[&str] = &["negligible", "light", "moderate", "heavy", "unbounded"];

/// Closed compatibility-label vocabulary.
pub const COMPATIBILITY_LABEL_CLASSES: &[&str] = &[
    "full_parity",
    "high_parity",
    "partial_parity",
    "limited_parity",
    "unsupported",
];

/// Closed install-scope vocabulary.
pub const INSTALL_SCOPE_CLASSES: &[&str] = &["user", "workspace", "machine", "portable"];

/// Closed revocation-posture vocabulary. `clean` is the only posture a stable claim
/// may keep.
pub const REVOCATION_POSTURE_CLASSES: &[&str] = &["clean", "advisory", "quarantined", "revoked"];

/// Closed mirrorability vocabulary. `not_mirrorable` narrows a stable claim.
pub const MIRRORABILITY_CLASSES: &[&str] = &["mirrorable", "mirror_pinned", "not_mirrorable"];

/// Closed set of stability tiers.
pub const STABILITY_TIERS: &[&str] = &["stable", "beta", "preview", "withdrawn"];

/// Tiers that count as a *stable* performance claim.
pub const STABLE_TIERS: &[&str] = &["stable"];

/// Closed claim-basis vocabulary. `catalog_asserted_only` may never back stable.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["evidence_backed", "catalog_asserted_only"];

/// Closed support-claim vocabulary a tier may imply.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_performance_budget_claim",
    "beta_performance_budget_partial_claim",
    "preview_performance_budget_experimental_claim",
    "withdrawn_no_performance_budget_claim",
];

/// Closed set of reasons that narrow a stable performance-budget claim below Stable.
pub const PERFORMANCE_BUDGET_DOWNGRADE_REASONS: &[&str] = &[
    "performance_budget_version_not_published",
    "catalog_only_trust_not_evidence_backed",
    "trust_tier_quarantined",
    "lifecycle_not_runnable",
    "budget_unbounded",
    "budget_over",
    "budget_not_measured",
    "measurement_stale",
    "measurement_expired",
    "measurement_not_measured",
    "enforcement_advisory",
    "enforcement_unenforced",
    "threshold_relaxed_without_waiver",
    "threshold_adjustment_missing_waiver",
    "waiver_expired",
    "waiver_revoked",
    "cost_unbounded",
    "cost_not_explained",
    "permission_widened",
    "compatibility_unsupported",
    "compatibility_parity_limited",
    "compatibility_not_verified",
    "install_scope_not_disclosed",
    "revocation_posture_quarantined",
    "revocation_posture_revoked",
    "revocation_posture_advisory",
    "not_mirrorable",
    "attribution_incomplete",
];

/// Reasons that narrow all the way to `withdrawn` (the cost cannot be trusted as
/// stable at all).
const WITHDRAWN_CLASS_REASONS: &[&str] = &[
    "lifecycle_not_runnable",
    "budget_unbounded",
    "cost_unbounded",
    "permission_widened",
    "compatibility_unsupported",
    "revocation_posture_quarantined",
    "revocation_posture_revoked",
];

/// Reasons that narrow to `preview` (a structural / disclosure / measurement
/// shortfall).
const PREVIEW_CLASS_REASONS: &[&str] = &[
    "performance_budget_version_not_published",
    "catalog_only_trust_not_evidence_backed",
    "trust_tier_quarantined",
    "budget_not_measured",
    "measurement_expired",
    "measurement_not_measured",
    "enforcement_unenforced",
    "threshold_relaxed_without_waiver",
    "threshold_adjustment_missing_waiver",
    "waiver_revoked",
    "cost_not_explained",
    "compatibility_not_verified",
    "install_scope_not_disclosed",
    "attribution_incomplete",
];

/// Reasons whose only effect is a safe narrowing to `beta`.
const BETA_CLASS_REASONS: &[&str] = &[
    "budget_over",
    "measurement_stale",
    "enforcement_advisory",
    "waiver_expired",
    "compatibility_parity_limited",
    "revocation_posture_advisory",
    "not_mirrorable",
];

/// Closed set of consumer surfaces that ingest this packet.
pub const STABLE_PERFORMANCE_BUDGET_CONSUMER_SURFACES: &[&str] = &[
    "marketplace_result_row",
    "marketplace_detail_page",
    "install_review",
    "extension_detail_view",
    "performance_inspector",
    "diagnostics",
    "support_export",
    "docs_help_surface",
    "release_packet",
    "cli_inspector",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a stable performance-budget packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StablePerformanceBudgetInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity input.
    pub identity: PerformanceBudgetIdentityInput,
    /// Performance-measurement input.
    pub measurement: PerformanceMeasurementInput,
    /// Budget-enforcement input.
    pub enforcement: PerformanceBudgetEnforcementInput,
    /// Waiver input.
    pub waiver: PerformanceBudgetWaiverInput,
    /// Cost-explanation input.
    pub cost_explanation: PerformanceCostExplanationInput,
    /// Permission-posture input.
    pub permission_posture: PerformancePermissionPostureInput,
    /// Compatibility input.
    pub compatibility: PerformanceCompatibilityInput,
    /// Install-posture input.
    pub install_posture: PerformanceInstallPostureInput,
    /// Stability qualification claim input.
    pub claim: PerformanceQualificationClaimInput,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input for [`PerformanceBudgetIdentity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceBudgetIdentityInput {
    /// Ref to the performance-profile descriptor this row stabilizes.
    pub performance_profile_ref: String,
    /// Opaque marketplace / catalog row identity ref.
    pub row_identity_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Published performance-budget profile version this row pins.
    pub performance_budget_version: u32,
    /// Publisher namespace the row asserts authority under.
    pub publisher_namespace: String,
    /// Ref to the pinned benchmark-evidence bundle.
    pub benchmark_evidence_ref: String,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

/// Input for [`PerformanceMeasurement`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceMeasurementInput {
    /// Worst-case budget axis the measurement covers.
    pub budget_axis_class: String,
    /// Measured p50 cost in the axis unit (ms or unit count).
    pub measured_p50: u32,
    /// Measured p95 cost in the axis unit.
    pub measured_p95: u32,
    /// Number of samples behind the measurement.
    pub sample_count: u32,
    /// Ref to the benchmark-lab corpus metadata the run was drawn from.
    pub corpus_metadata_ref: String,
    /// Ref to the benchmark-lab trace bundle.
    pub benchmark_trace_ref: String,
    /// Measurement freshness.
    pub measurement_freshness_class: String,
    /// Whether the benchmark trace is attested (not self-reported).
    pub trace_attested: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`PerformanceBudgetEnforcement`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceBudgetEnforcementInput {
    /// Budget status against the published p50 / p95 ceilings.
    pub budget_status_class: String,
    /// Published p50 budget ceiling in the axis unit.
    pub published_p50_budget: u32,
    /// Published p95 budget ceiling in the axis unit.
    pub published_p95_budget: u32,
    /// Enforcement mode.
    pub enforcement_mode_class: String,
    /// Threshold-adjustment posture.
    pub threshold_adjustment_class: String,
    /// Ref to the published budget profile.
    pub budget_profile_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`PerformanceBudgetWaiver`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceBudgetWaiverInput {
    /// Waiver state.
    pub waiver_state_class: String,
    /// Ref to the waiver record (empty when no waiver exists).
    pub waiver_ref: String,
    /// Waiver authority (set only when a waiver exists).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_authority_class: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`PerformanceCostExplanation`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceCostExplanationInput {
    /// Headline cost class.
    pub cost_class: String,
    /// Dominant cost factor (a budget axis).
    pub dominant_cost_factor_class: String,
    /// Whether a user-readable explanation is attached.
    pub cost_explained: bool,
    /// Ref to the user-visible explanation copy.
    pub explanation_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`PerformancePermissionPosture`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformancePermissionPostureInput {
    /// Ref to the declared permission set.
    pub declared_permission_ref: String,
    /// Ref to the effective permission set after resolution.
    pub effective_permission_ref: String,
    /// Whether authority was widened beyond the declared set.
    pub widened: bool,
    /// Whether re-consent is required before activation.
    pub reconsent_required: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`PerformanceCompatibility`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceCompatibilityInput {
    /// Compatibility label.
    pub compatibility_label_class: String,
    /// Ref to the compatibility scorecard.
    pub scorecard_ref: String,
    /// Whether compatibility was verified.
    pub compatibility_verified: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`PerformanceInstallPosture`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceInstallPostureInput {
    /// Install scope.
    pub install_scope_class: String,
    /// Whether the install scope is disclosed.
    pub install_scope_disclosed: bool,
    /// Revocation posture.
    pub revocation_posture_class: String,
    /// Mirrorability.
    pub mirrorability_class: String,
    /// Whether rollback is supported.
    pub rollback_supported: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`PerformanceQualificationClaim`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceQualificationClaimInput {
    /// Performance tier claimed by the row.
    pub claimed_tier: String,
    /// Claim basis: evidence-backed vs catalog-asserted only.
    pub claim_basis_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// Identity shared across every surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceBudgetIdentity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ref to the performance-profile descriptor this row stabilizes.
    pub performance_profile_ref: String,
    /// Opaque marketplace / catalog row identity ref.
    pub row_identity_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Published performance-budget profile version this row pins.
    pub performance_budget_version: u32,
    /// Publisher namespace the row asserts authority under.
    pub publisher_namespace: String,
    /// Ref to the pinned benchmark-evidence bundle.
    pub benchmark_evidence_ref: String,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

impl PerformanceBudgetIdentity {
    /// Returns true when the row pins the published performance-budget profile version.
    pub fn profile_version_current(&self) -> bool {
        self.performance_budget_version == STABLE_PERFORMANCE_BUDGET_PUBLISHED_PROFILE_VERSION
    }

    /// Returns true when the lifecycle is runnable.
    pub fn lifecycle_runnable(&self) -> bool {
        RUNNABLE_LIFECYCLE_STATES.contains(&self.lifecycle_state_class.as_str())
    }
}

/// Inspected performance measurement for the worst-case surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceMeasurement {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Worst-case budget axis the measurement covers.
    pub budget_axis_class: String,
    /// Measured p50 cost.
    pub measured_p50: u32,
    /// Measured p95 cost.
    pub measured_p95: u32,
    /// Number of samples behind the measurement.
    pub sample_count: u32,
    /// Ref to the benchmark-lab corpus metadata.
    pub corpus_metadata_ref: String,
    /// Ref to the benchmark-lab trace bundle.
    pub benchmark_trace_ref: String,
    /// Measurement freshness.
    pub measurement_freshness_class: String,
    /// Whether the benchmark trace is attested.
    pub trace_attested: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl PerformanceMeasurement {
    /// Returns true when the measurement is fresh.
    pub fn fresh(&self) -> bool {
        self.measurement_freshness_class == "fresh"
    }

    /// Returns true when the measurement is expired or was never taken.
    pub fn expired_or_missing(&self) -> bool {
        matches!(
            self.measurement_freshness_class.as_str(),
            "expired" | "not_measured"
        )
    }
}

/// Budget enforcement against the published p50 / p95 ceilings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceBudgetEnforcement {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Budget status against the published ceilings.
    pub budget_status_class: String,
    /// Published p50 budget ceiling.
    pub published_p50_budget: u32,
    /// Published p95 budget ceiling.
    pub published_p95_budget: u32,
    /// Enforcement mode.
    pub enforcement_mode_class: String,
    /// Threshold-adjustment posture.
    pub threshold_adjustment_class: String,
    /// Ref to the published budget profile.
    pub budget_profile_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl PerformanceBudgetEnforcement {
    /// Returns true when the cost is bounded and within the published budget.
    pub fn within_budget(&self) -> bool {
        self.budget_status_class == "within_budget"
    }

    /// Returns true when the cost is unbounded.
    pub fn unbounded(&self) -> bool {
        self.budget_status_class == "unbounded"
    }

    /// Returns true when the budget is actually enforced (not advisory / catalog-only).
    pub fn enforced(&self) -> bool {
        self.enforcement_mode_class == "enforced"
    }

    /// Returns true when the published threshold was relaxed (loosened).
    pub fn threshold_relaxed(&self) -> bool {
        self.threshold_adjustment_class == "relaxed"
    }

    /// Returns true when the published threshold was intentionally tightened / narrowed.
    pub fn threshold_tightened_or_narrowed(&self) -> bool {
        matches!(
            self.threshold_adjustment_class.as_str(),
            "tightened" | "narrowed"
        )
    }
}

/// Waiver hook backing any intentional threshold adjustment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceBudgetWaiver {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Waiver state.
    pub waiver_state_class: String,
    /// Ref to the waiver record (empty when no waiver exists).
    pub waiver_ref: String,
    /// Waiver authority (set only when a waiver exists).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_authority_class: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

impl PerformanceBudgetWaiver {
    /// Returns true when an active waiver is present.
    pub fn active(&self) -> bool {
        self.waiver_state_class == "active"
    }

    /// Returns true when a waiver hook is recorded (a non-empty ref exists).
    pub fn has_hook(&self) -> bool {
        !self.waiver_ref.trim().is_empty()
    }
}

/// User-visible cost explanation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceCostExplanation {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Headline cost class.
    pub cost_class: String,
    /// Dominant cost factor (a budget axis).
    pub dominant_cost_factor_class: String,
    /// Whether a user-readable explanation is attached.
    pub cost_explained: bool,
    /// Ref to the user-visible explanation copy.
    pub explanation_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl PerformanceCostExplanation {
    /// Returns true when the cost is unbounded.
    pub fn unbounded(&self) -> bool {
        self.cost_class == "unbounded"
    }
}

/// Permission posture for the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformancePermissionPosture {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ref to the declared permission set.
    pub declared_permission_ref: String,
    /// Ref to the effective permission set after resolution.
    pub effective_permission_ref: String,
    /// Whether authority was widened beyond the declared set.
    pub widened: bool,
    /// Whether re-consent is required before activation.
    pub reconsent_required: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Compatibility binding for the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceCompatibility {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Compatibility label.
    pub compatibility_label_class: String,
    /// Ref to the compatibility scorecard.
    pub scorecard_ref: String,
    /// Whether compatibility was verified.
    pub compatibility_verified: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl PerformanceCompatibility {
    /// Returns true when compatibility reports an unsupported parity.
    pub fn unsupported(&self) -> bool {
        self.compatibility_label_class == "unsupported"
    }

    /// Returns true when compatibility reports a parity-limited posture short of
    /// unsupported.
    pub fn parity_limited(&self) -> bool {
        matches!(
            self.compatibility_label_class.as_str(),
            "partial_parity" | "limited_parity"
        )
    }
}

/// Install / mirror / revocation posture for the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceInstallPosture {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Install scope.
    pub install_scope_class: String,
    /// Whether the install scope is disclosed.
    pub install_scope_disclosed: bool,
    /// Revocation posture.
    pub revocation_posture_class: String,
    /// Mirrorability.
    pub mirrorability_class: String,
    /// Whether rollback is supported.
    pub rollback_supported: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl PerformanceInstallPosture {
    /// Returns true when the revocation posture is clean.
    pub fn revocation_clean(&self) -> bool {
        self.revocation_posture_class == "clean"
    }

    /// Returns true when the row stays mirrorable.
    pub fn mirrorable(&self) -> bool {
        matches!(
            self.mirrorability_class.as_str(),
            "mirrorable" | "mirror_pinned"
        )
    }
}

/// Stability qualification claim after the posture is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceQualificationClaim {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Performance tier claimed by the row.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim the effective tier is allowed to imply.
    pub support_claim_class: String,
    /// Claim basis: evidence-backed vs catalog-asserted only.
    pub claim_basis_class: String,
    /// True when the claimed tier was narrowed below Stable.
    pub downgraded: bool,
    /// Reasons that narrowed the claim.
    pub downgrade_reasons: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Downgraded-row banner requirement. Raised whenever a reviewer must see a
/// performance or budget shortfall before relying on the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceDowngradedBanner {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// True when a downgraded-row banner must be displayed.
    pub must_display: bool,
    /// Most-severe applicable banner reason, drawn from the downgrade vocabulary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_reason_class: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Compact inspection row for CLI/headless and dashboard surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StablePerformanceBudgetInspection {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet inspected by this row.
    pub packet_id_ref: String,
    /// Effective performance tier.
    pub effective_tier: String,
    /// True when the claim is a stable performance claim.
    pub stable_claim: bool,
    /// True when the row pins the published profile version.
    pub profile_version_current: bool,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// True when the lifecycle is runnable.
    pub lifecycle_runnable: bool,
    /// Worst-case budget axis.
    pub budget_axis_class: String,
    /// Measured p50 cost.
    pub measured_p50: u32,
    /// Measured p95 cost.
    pub measured_p95: u32,
    /// Published p50 budget ceiling.
    pub published_p50_budget: u32,
    /// Published p95 budget ceiling.
    pub published_p95_budget: u32,
    /// Budget status.
    pub budget_status_class: String,
    /// True when the cost is bounded and within budget.
    pub within_budget: bool,
    /// True when the budget is actually enforced.
    pub budget_enforced: bool,
    /// Measurement freshness.
    pub measurement_freshness_class: String,
    /// True when the benchmark trace is attested.
    pub trace_attested: bool,
    /// Threshold-adjustment posture.
    pub threshold_adjustment_class: String,
    /// Waiver state.
    pub waiver_state_class: String,
    /// Headline cost class.
    pub cost_class: String,
    /// True when the cost is explained to the user.
    pub cost_explained: bool,
    /// True when permissions were not widened.
    pub permissions_not_widened: bool,
    /// Compatibility label.
    pub compatibility_label_class: String,
    /// True when compatibility was verified.
    pub compatibility_verified: bool,
    /// Revocation posture.
    pub revocation_posture_class: String,
    /// Mirrorability.
    pub mirrorability_class: String,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// True when a downgraded-row banner is required.
    pub downgraded_banner_required: bool,
    /// True when identity and every artifact are fully attributed.
    pub attribution_complete: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Stable performance-budget packet consumed by marketplace result / detail rows,
/// install review, the extension detail view, the performance inspector,
/// diagnostics, support export, docs/help, release packets, and the CLI inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StablePerformanceBudgetPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity.
    pub identity: PerformanceBudgetIdentity,
    /// Performance measurement.
    pub measurement: PerformanceMeasurement,
    /// Budget enforcement.
    pub enforcement: PerformanceBudgetEnforcement,
    /// Waiver hook.
    pub waiver: PerformanceBudgetWaiver,
    /// User-visible cost explanation.
    pub cost_explanation: PerformanceCostExplanation,
    /// Permission posture.
    pub permission_posture: PerformancePermissionPosture,
    /// Compatibility.
    pub compatibility: PerformanceCompatibility,
    /// Install posture.
    pub install_posture: PerformanceInstallPosture,
    /// Stability qualification claim after the posture is applied.
    pub claim: PerformanceQualificationClaim,
    /// Downgraded-row banner requirement.
    pub downgraded_banner: PerformanceDowngradedBanner,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the packet cites.
    pub source_schema_refs: Vec<String>,
    /// False so a catalog row can never imply a stable performance claim on its own.
    pub allows_catalog_only_trust: bool,
    /// False so a widened permission set can never ride a stable performance row.
    pub allows_ambient_extension_privilege: bool,
    /// False so an unbounded activation cost can never ride a stable performance row.
    pub allows_unbounded_activation_cost: bool,
    /// Inspection row.
    pub inspection: StablePerformanceBudgetInspection,
}

impl StablePerformanceBudgetPacket {
    /// Builds a stable performance-budget packet from input, applying the budget
    /// posture to the claimed tier so any required downgrade below Stable is
    /// automatic.
    ///
    /// # Errors
    ///
    /// Returns [`StablePerformanceBudgetValidationError`] when the input violates an
    /// identity, measurement, enforcement, waiver, cost, permission, compatibility,
    /// install, or claim invariant.
    pub fn from_input(
        input: StablePerformanceBudgetInput,
    ) -> Result<Self, StablePerformanceBudgetValidationError> {
        validate_input(&input)?;

        let identity = identity_record(&input.identity);
        let measurement = measurement_record(&input.measurement);
        let enforcement = enforcement_record(&input.enforcement);
        let waiver = waiver_record(&input.waiver);
        let cost_explanation = cost_explanation_record(&input.cost_explanation);
        let permission_posture = permission_posture_record(&input.permission_posture);
        let compatibility = compatibility_record(&input.compatibility);
        let install_posture = install_posture_record(&input.install_posture);
        let attribution_complete =
            attribution_is_complete(&identity, &measurement, &enforcement, &compatibility);

        let posture = BudgetPosture {
            identity: &identity,
            measurement: &measurement,
            enforcement: &enforcement,
            waiver: &waiver,
            cost_explanation: &cost_explanation,
            permission_posture: &permission_posture,
            compatibility: &compatibility,
            install_posture: &install_posture,
            attribution_complete,
        };

        let claim = claim_record(&input.claim, &posture);
        let downgraded_banner = banner_record(&posture);
        let inspection = inspection_record(&input.packet_id, &posture, &claim, &downgraded_banner);

        let packet = Self {
            record_kind: STABLE_PERFORMANCE_BUDGET_PACKET_RECORD_KIND.to_string(),
            schema_version: STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            identity,
            measurement,
            enforcement,
            waiver,
            cost_explanation,
            permission_posture,
            compatibility,
            install_posture,
            claim,
            downgraded_banner,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![STABLE_PERFORMANCE_BUDGET_SCHEMA_REF.to_string()],
            allows_catalog_only_trust: false,
            allows_ambient_extension_privilege: false,
            allows_unbounded_activation_cost: false,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the stable performance-budget invariants.
    ///
    /// # Errors
    ///
    /// Returns [`StablePerformanceBudgetValidationError`] when an invariant is violated.
    pub fn validate(&self) -> Result<(), StablePerformanceBudgetValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            STABLE_PERFORMANCE_BUDGET_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_identity(&self.identity)?;
        validate_measurement(&self.measurement)?;
        validate_enforcement(&self.enforcement)?;
        validate_waiver(&self.waiver)?;
        validate_cost_explanation(&self.cost_explanation)?;
        validate_permission_posture(&self.permission_posture)?;
        validate_compatibility(&self.compatibility)?;
        validate_install_posture(&self.install_posture)?;
        validate_claim(&self.claim)?;
        validate_banner(&self.downgraded_banner)?;
        validate_budget_numbers(&self.measurement, &self.enforcement)?;

        for surface in &self.consumer_surfaces {
            ensure_token(
                STABLE_PERFORMANCE_BUDGET_CONSUMER_SURFACES,
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
            .any(|r| r == STABLE_PERFORMANCE_BUDGET_SCHEMA_REF)
        {
            return Err(err("packet must cite its source schema"));
        }

        // No catalog-only trust, ambient extension privilege, or unbounded activation
        // cost may ride a published stable performance row.
        if self.allows_catalog_only_trust
            || self.allows_ambient_extension_privilege
            || self.allows_unbounded_activation_cost
        {
            return Err(err(
                "a stable performance-budget packet must not allow catalog-only trust, ambient extension privilege, or unbounded activation cost",
            ));
        }

        // Stable-claim binding.
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            if !self.identity.profile_version_current() {
                return Err(err(
                    "stable effective tier must pin the published performance-budget profile version",
                ));
            }
            if self.claim.claim_basis_class != "evidence_backed" {
                return Err(err(
                    "stable effective tier must be evidence-backed, not catalog-asserted",
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
            if !self.enforcement.within_budget() {
                return Err(err(
                    "stable effective tier must keep its cost bounded and within the published budget",
                ));
            }
            if !self.enforcement.enforced() {
                return Err(err(
                    "stable effective tier must keep its budget actually enforced",
                ));
            }
            if self.enforcement.threshold_relaxed() && !self.waiver.active() {
                return Err(err(
                    "stable effective tier must carry an active waiver for a relaxed threshold",
                ));
            }
            if self.enforcement.threshold_tightened_or_narrowed() && !self.waiver.has_hook() {
                return Err(err(
                    "stable effective tier must record a waiver hook for a tightened or narrowed threshold",
                ));
            }
            if matches!(
                self.waiver.waiver_state_class.as_str(),
                "expired" | "revoked"
            ) {
                return Err(err(
                    "stable effective tier must not carry an expired or revoked waiver",
                ));
            }
            if !self.measurement.fresh() {
                return Err(err(
                    "stable effective tier must keep a fresh benchmark measurement",
                ));
            }
            if !self.measurement.trace_attested {
                return Err(err(
                    "stable effective tier must keep an attested benchmark trace",
                ));
            }
            if self.cost_explanation.unbounded() {
                return Err(err(
                    "stable effective tier must not present an unbounded cost",
                ));
            }
            if !self.cost_explanation.cost_explained {
                return Err(err(
                    "stable effective tier must explain its cost to the user",
                ));
            }
            if self.permission_posture.widened {
                return Err(err("stable effective tier must not widen permissions"));
            }
            if self.compatibility.unsupported()
                || self.compatibility.parity_limited()
                || !self.compatibility.compatibility_verified
            {
                return Err(err(
                    "stable effective tier must carry verified, non-parity-limited compatibility",
                ));
            }
            if !self.install_posture.install_scope_disclosed {
                return Err(err("stable effective tier must disclose its install scope"));
            }
            if !self.install_posture.revocation_clean() {
                return Err(err(
                    "stable effective tier must keep a clean revocation posture",
                ));
            }
            if !self.install_posture.mirrorable() {
                return Err(err("stable effective tier must stay mirrorable"));
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

        // Downgrade truth.
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
        let posture = BudgetPosture {
            identity: &self.identity,
            measurement: &self.measurement,
            enforcement: &self.enforcement,
            waiver: &self.waiver,
            cost_explanation: &self.cost_explanation,
            permission_posture: &self.permission_posture,
            compatibility: &self.compatibility,
            install_posture: &self.install_posture,
            attribution_complete: self.attribution_complete(),
        };
        let derived = derive_effective_tier(
            &self.claim.claimed_tier,
            &self.claim.claim_basis_class,
            &posture,
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
        let mut stored = self.claim.downgrade_reasons.clone();
        stored.sort();
        let mut expected = derived.downgrade_reasons.clone();
        expected.sort();
        if stored != expected {
            return Err(err(
                "stored downgrade reasons do not match the posture-derived reasons",
            ));
        }

        // Banner truth.
        let banner_required = performance_requires_warning(&posture);
        if self.downgraded_banner.must_display != banner_required {
            return Err(err(
                "downgraded-row banner must_display does not match the performance posture",
            ));
        }

        validate_inspection(&self.inspection, self)?;
        Ok(())
    }

    /// Returns true when no stable claim is implied from catalog-only trust.
    pub fn no_catalog_only_stable_claim(&self) -> bool {
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            return self.claim.claim_basis_class == "evidence_backed";
        }
        true
    }

    /// Returns true when an unbounded cost is never left rendering stable.
    pub fn unbounded_cost_never_stable(&self) -> bool {
        if !self.enforcement.unbounded() && !self.cost_explanation.unbounded() {
            return true;
        }
        !STABLE_TIERS.contains(&self.claim.effective_tier.as_str())
    }

    /// Returns true when identity, the benchmark measurement, the budget profile, and
    /// compatibility are fully attributed.
    pub fn attribution_complete(&self) -> bool {
        attribution_is_complete(
            &self.identity,
            &self.measurement,
            &self.enforcement,
            &self.compatibility,
        )
    }
}

// ---------------------------------------------------------------------------
// Projection
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and dashboard surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StablePerformanceBudgetProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Marketplace / catalog row identity.
    pub row_identity_ref: String,
    /// Worst-case budget axis.
    pub budget_axis_class: String,
    /// Measured p50 cost.
    pub measured_p50: u32,
    /// Measured p95 cost.
    pub measured_p95: u32,
    /// Published p50 budget ceiling.
    pub published_p50_budget: u32,
    /// Published p95 budget ceiling.
    pub published_p95_budget: u32,
    /// Budget status.
    pub budget_status_class: String,
    /// Headline cost class.
    pub cost_class: String,
    /// Claimed tier.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim is a stable performance claim.
    pub stable_claim: bool,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// Downgrade reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-row banner is required.
    pub downgraded_banner_required: bool,
    /// Revocation posture.
    pub revocation_posture_class: String,
}

impl From<StablePerformanceBudgetPacket> for StablePerformanceBudgetProjection {
    fn from(packet: StablePerformanceBudgetPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            row_identity_ref: packet.identity.row_identity_ref,
            budget_axis_class: packet.measurement.budget_axis_class,
            measured_p50: packet.measurement.measured_p50,
            measured_p95: packet.measurement.measured_p95,
            published_p50_budget: packet.enforcement.published_p50_budget,
            published_p95_budget: packet.enforcement.published_p95_budget,
            budget_status_class: packet.enforcement.budget_status_class,
            cost_class: packet.cost_explanation.cost_class,
            claimed_tier: packet.claim.claimed_tier,
            effective_tier: packet.claim.effective_tier,
            support_claim_class: packet.claim.support_claim_class,
            stable_claim: packet.inspection.stable_claim,
            downgraded: packet.claim.downgraded,
            downgrade_reasons: packet.claim.downgrade_reasons,
            downgraded_banner_required: packet.downgraded_banner.must_display,
            revocation_posture_class: packet.install_posture.revocation_posture_class,
        }
    }
}

/// Parses and validates a materialized packet, returning the compact projection.
///
/// # Errors
///
/// Returns [`StablePerformanceBudgetError`] when the payload fails to parse or
/// violates the stable performance-budget invariants.
pub fn project_stable_performance_budget(
    payload: &str,
) -> Result<StablePerformanceBudgetProjection, StablePerformanceBudgetError> {
    let packet: StablePerformanceBudgetPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(StablePerformanceBudgetProjection::from(packet))
}

// ---------------------------------------------------------------------------
// Support export projection
// ---------------------------------------------------------------------------

/// Metadata-safe support / partner / mirror export row that quotes the same closed
/// tokens and numeric budgets as the packet without leaking raw artifact, trace, or
/// publisher-private bytes, and preserves the measured-vs-published cost so a
/// reviewer can see why a row is or is not within budget.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StablePerformanceBudgetSupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub export_id: String,
    /// Ref to the packet this export quotes.
    pub packet_ref: String,
    /// Marketplace / catalog row identity.
    pub row_identity_ref: String,
    /// Performance-profile descriptor ref.
    pub performance_profile_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Publisher namespace.
    pub publisher_namespace: String,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// Lifecycle state.
    pub lifecycle_state_class: String,
    /// Worst-case budget axis.
    pub budget_axis_class: String,
    /// Measured p50 cost.
    pub measured_p50: u32,
    /// Measured p95 cost.
    pub measured_p95: u32,
    /// Published p50 budget ceiling.
    pub published_p50_budget: u32,
    /// Published p95 budget ceiling.
    pub published_p95_budget: u32,
    /// Budget status.
    pub budget_status_class: String,
    /// Enforcement mode.
    pub enforcement_mode_class: String,
    /// Threshold-adjustment posture.
    pub threshold_adjustment_class: String,
    /// Waiver state.
    pub waiver_state_class: String,
    /// Measurement freshness.
    pub measurement_freshness_class: String,
    /// True when the benchmark trace is attested.
    pub trace_attested: bool,
    /// Headline cost class.
    pub cost_class: String,
    /// True when the cost is explained to the user.
    pub cost_explained: bool,
    /// Compatibility label.
    pub compatibility_label_class: String,
    /// Revocation posture.
    pub revocation_posture_class: String,
    /// Mirrorability.
    pub mirrorability_class: String,
    /// Effective tier.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim was narrowed below Stable.
    pub downgraded: bool,
    /// Narrowing reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-row banner is required.
    pub downgraded_banner_required: bool,
    /// True when the effective tier blocks the row as a stable performance claim (withdrawn).
    pub blocks_stable_performance: bool,
    /// Export-safe summary suitable for support / partner / mirror consumers.
    pub export_safe_summary: String,
}

/// Projects a packet into the metadata-safe support / partner / mirror export row.
pub fn project_stable_performance_budget_support_export(
    packet: &StablePerformanceBudgetPacket,
) -> StablePerformanceBudgetSupportExport {
    let blocks = packet.claim.effective_tier == "withdrawn";
    let export_safe_summary = format!(
        "{} Axis={} p50={}/{} p95={}/{} status={} enforcement={} threshold={} waiver={}. Measurement={} attested={}. Cost={} explained={}. Compatibility={}. Revocation={} mirrorability={}. Tier claimed={} effective={} (downgraded={}). Banner required={}.",
        packet.claim.summary_label,
        packet.measurement.budget_axis_class,
        packet.measurement.measured_p50,
        packet.enforcement.published_p50_budget,
        packet.measurement.measured_p95,
        packet.enforcement.published_p95_budget,
        packet.enforcement.budget_status_class,
        packet.enforcement.enforcement_mode_class,
        packet.enforcement.threshold_adjustment_class,
        packet.waiver.waiver_state_class,
        packet.measurement.measurement_freshness_class,
        packet.measurement.trace_attested,
        packet.cost_explanation.cost_class,
        packet.cost_explanation.cost_explained,
        packet.compatibility.compatibility_label_class,
        packet.install_posture.revocation_posture_class,
        packet.install_posture.mirrorability_class,
        packet.claim.claimed_tier,
        packet.claim.effective_tier,
        packet.claim.downgraded,
        packet.downgraded_banner.must_display,
    );

    StablePerformanceBudgetSupportExport {
        record_kind: STABLE_PERFORMANCE_BUDGET_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION,
        export_id: format!(
            "stable_performance_budget_support_export:{}",
            packet.packet_id
        ),
        packet_ref: packet.packet_id.clone(),
        row_identity_ref: packet.identity.row_identity_ref.clone(),
        performance_profile_ref: packet.identity.performance_profile_ref.clone(),
        extension_identity: packet.identity.extension_identity.clone(),
        extension_version: packet.identity.extension_version.clone(),
        publisher_namespace: packet.identity.publisher_namespace.clone(),
        trust_tier_class: packet.identity.publisher_trust_tier_class.clone(),
        lifecycle_state_class: packet.identity.lifecycle_state_class.clone(),
        budget_axis_class: packet.measurement.budget_axis_class.clone(),
        measured_p50: packet.measurement.measured_p50,
        measured_p95: packet.measurement.measured_p95,
        published_p50_budget: packet.enforcement.published_p50_budget,
        published_p95_budget: packet.enforcement.published_p95_budget,
        budget_status_class: packet.enforcement.budget_status_class.clone(),
        enforcement_mode_class: packet.enforcement.enforcement_mode_class.clone(),
        threshold_adjustment_class: packet.enforcement.threshold_adjustment_class.clone(),
        waiver_state_class: packet.waiver.waiver_state_class.clone(),
        measurement_freshness_class: packet.measurement.measurement_freshness_class.clone(),
        trace_attested: packet.measurement.trace_attested,
        cost_class: packet.cost_explanation.cost_class.clone(),
        cost_explained: packet.cost_explanation.cost_explained,
        compatibility_label_class: packet.compatibility.compatibility_label_class.clone(),
        revocation_posture_class: packet.install_posture.revocation_posture_class.clone(),
        mirrorability_class: packet.install_posture.mirrorability_class.clone(),
        effective_tier: packet.claim.effective_tier.clone(),
        support_claim_class: packet.claim.support_claim_class.clone(),
        downgraded: packet.claim.downgraded,
        downgrade_reasons: packet.claim.downgrade_reasons.clone(),
        downgraded_banner_required: packet.downgraded_banner.must_display,
        blocks_stable_performance: blocks,
        export_safe_summary,
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for stable performance-budget operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StablePerformanceBudgetError {
    /// Validation failed.
    Validation(StablePerformanceBudgetValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for StablePerformanceBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for StablePerformanceBudgetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for stable performance-budget packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StablePerformanceBudgetValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for StablePerformanceBudgetValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StablePerformanceBudgetValidationError {}

impl StablePerformanceBudgetValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<serde_json::Error> for StablePerformanceBudgetError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(StablePerformanceBudgetValidationError {
            message: err.to_string(),
        })
    }
}

impl From<StablePerformanceBudgetValidationError> for StablePerformanceBudgetError {
    fn from(err: StablePerformanceBudgetValidationError) -> Self {
        Self::Validation(err)
    }
}

// ---------------------------------------------------------------------------
// Effective-tier derivation (the automatic narrowing)
// ---------------------------------------------------------------------------

/// Bundle of derived records used to apply the budget posture.
struct BudgetPosture<'a> {
    identity: &'a PerformanceBudgetIdentity,
    measurement: &'a PerformanceMeasurement,
    enforcement: &'a PerformanceBudgetEnforcement,
    waiver: &'a PerformanceBudgetWaiver,
    cost_explanation: &'a PerformanceCostExplanation,
    permission_posture: &'a PerformancePermissionPosture,
    compatibility: &'a PerformanceCompatibility,
    install_posture: &'a PerformanceInstallPosture,
    attribution_complete: bool,
}

struct DerivedTier {
    effective_tier: String,
    support_claim: String,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
}

/// Collects the narrowing reasons triggered by the budget posture.
fn posture_reasons(posture: &BudgetPosture<'_>) -> Vec<String> {
    let mut reasons: Vec<String> = Vec::new();

    // Identity.
    if !posture.identity.profile_version_current() {
        reasons.push("performance_budget_version_not_published".to_string());
    }
    if posture.identity.publisher_trust_tier_class == "quarantined" {
        reasons.push("trust_tier_quarantined".to_string());
    }
    if !posture.identity.lifecycle_runnable() {
        reasons.push("lifecycle_not_runnable".to_string());
    }

    // Budget enforcement.
    match posture.enforcement.budget_status_class.as_str() {
        "unbounded" => reasons.push("budget_unbounded".to_string()),
        "over_budget" => reasons.push("budget_over".to_string()),
        "not_measured" => reasons.push("budget_not_measured".to_string()),
        _ => {}
    }
    match posture.enforcement.enforcement_mode_class.as_str() {
        "unenforced" => reasons.push("enforcement_unenforced".to_string()),
        "advisory" => reasons.push("enforcement_advisory".to_string()),
        _ => {}
    }
    if posture.enforcement.threshold_relaxed() && !posture.waiver.active() {
        reasons.push("threshold_relaxed_without_waiver".to_string());
    }
    if posture.enforcement.threshold_tightened_or_narrowed() && !posture.waiver.has_hook() {
        reasons.push("threshold_adjustment_missing_waiver".to_string());
    }

    // Measurement.
    match posture.measurement.measurement_freshness_class.as_str() {
        "stale" => reasons.push("measurement_stale".to_string()),
        "expired" => reasons.push("measurement_expired".to_string()),
        "not_measured" => reasons.push("measurement_not_measured".to_string()),
        _ => {}
    }

    // Waiver.
    match posture.waiver.waiver_state_class.as_str() {
        "expired" => reasons.push("waiver_expired".to_string()),
        "revoked" => reasons.push("waiver_revoked".to_string()),
        _ => {}
    }

    // Cost explanation.
    if posture.cost_explanation.unbounded() {
        reasons.push("cost_unbounded".to_string());
    }
    if !posture.cost_explanation.cost_explained {
        reasons.push("cost_not_explained".to_string());
    }

    // Permissions.
    if posture.permission_posture.widened {
        reasons.push("permission_widened".to_string());
    }

    // Compatibility.
    if posture.compatibility.unsupported() {
        reasons.push("compatibility_unsupported".to_string());
    } else if posture.compatibility.parity_limited() {
        reasons.push("compatibility_parity_limited".to_string());
    }
    if !posture.compatibility.compatibility_verified {
        reasons.push("compatibility_not_verified".to_string());
    }

    // Install posture.
    if !posture.install_posture.install_scope_disclosed {
        reasons.push("install_scope_not_disclosed".to_string());
    }
    match posture.install_posture.revocation_posture_class.as_str() {
        "quarantined" => reasons.push("revocation_posture_quarantined".to_string()),
        "revoked" => reasons.push("revocation_posture_revoked".to_string()),
        "advisory" => reasons.push("revocation_posture_advisory".to_string()),
        _ => {}
    }
    if !posture.install_posture.mirrorable() {
        reasons.push("not_mirrorable".to_string());
    }

    if !posture.attribution_complete {
        reasons.push("attribution_incomplete".to_string());
    }

    reasons.sort();
    reasons.dedup();
    reasons
}

/// Applies the budget posture to a claimed tier, narrowing automatically below Stable
/// when the evidence can no longer back it. The claim basis is folded in separately so
/// a `catalog_asserted_only` basis can never back a stable claim.
fn derive_effective_tier(
    claimed_tier: &str,
    claim_basis: &str,
    posture: &BudgetPosture<'_>,
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

    let mut reasons = posture_reasons(posture);
    if claim_basis != "evidence_backed" {
        reasons.push("catalog_only_trust_not_evidence_backed".to_string());
        reasons.sort();
        reasons.dedup();
    }

    if reasons.is_empty() {
        DerivedTier {
            effective_tier: claimed_tier.to_string(),
            support_claim: support_claim_for(claimed_tier),
            downgraded: false,
            downgrade_reasons: Vec::new(),
        }
    } else {
        let effective_tier = narrow_tier_for(&reasons);
        DerivedTier {
            effective_tier: effective_tier.to_string(),
            support_claim: support_claim_for(effective_tier),
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
        "stable" => "stable_performance_budget_claim",
        "beta" => "beta_performance_budget_partial_claim",
        "preview" => "preview_performance_budget_experimental_claim",
        "withdrawn" => "withdrawn_no_performance_budget_claim",
        _ => "preview_performance_budget_experimental_claim",
    }
    .to_string()
}

/// Returns true when identity, the benchmark measurement, the budget profile, and
/// compatibility are fully attributed.
fn attribution_is_complete(
    identity: &PerformanceBudgetIdentity,
    measurement: &PerformanceMeasurement,
    enforcement: &PerformanceBudgetEnforcement,
    compatibility: &PerformanceCompatibility,
) -> bool {
    !identity.performance_profile_ref.trim().is_empty()
        && !identity.row_identity_ref.trim().is_empty()
        && !identity.benchmark_evidence_ref.trim().is_empty()
        && !identity.publisher_namespace.trim().is_empty()
        && !measurement.benchmark_trace_ref.trim().is_empty()
        && !measurement.corpus_metadata_ref.trim().is_empty()
        && measurement.trace_attested
        && !enforcement.budget_profile_ref.trim().is_empty()
        && !compatibility.scorecard_ref.trim().is_empty()
}

/// Returns true when the budget posture requires a pre-trust warning banner.
fn performance_requires_warning(posture: &BudgetPosture<'_>) -> bool {
    posture.identity.publisher_trust_tier_class == "quarantined"
        || !posture.identity.lifecycle_runnable()
        || posture.enforcement.unbounded()
        || posture.cost_explanation.unbounded()
        || posture.permission_posture.widened
        || posture.compatibility.unsupported()
        || matches!(
            posture.install_posture.revocation_posture_class.as_str(),
            "quarantined" | "revoked"
        )
        || posture.waiver.waiver_state_class == "revoked"
        || (posture.enforcement.threshold_relaxed() && !posture.waiver.active())
}

/// Picks the most-severe banner reason for a row that requires a warning.
fn banner_reason_for(posture: &BudgetPosture<'_>) -> Option<String> {
    if posture.permission_posture.widened {
        return Some("permission_widened".to_string());
    }
    if posture.enforcement.unbounded() {
        return Some("budget_unbounded".to_string());
    }
    if posture.cost_explanation.unbounded() {
        return Some("cost_unbounded".to_string());
    }
    if posture.compatibility.unsupported() {
        return Some("compatibility_unsupported".to_string());
    }
    if posture.install_posture.revocation_posture_class == "revoked" {
        return Some("revocation_posture_revoked".to_string());
    }
    if posture.install_posture.revocation_posture_class == "quarantined" {
        return Some("revocation_posture_quarantined".to_string());
    }
    if !posture.identity.lifecycle_runnable() {
        return Some("lifecycle_not_runnable".to_string());
    }
    if posture.waiver.waiver_state_class == "revoked" {
        return Some("waiver_revoked".to_string());
    }
    if posture.enforcement.threshold_relaxed() && !posture.waiver.active() {
        return Some("threshold_relaxed_without_waiver".to_string());
    }
    if posture.identity.publisher_trust_tier_class == "quarantined" {
        return Some("trust_tier_quarantined".to_string());
    }
    None
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn identity_record(input: &PerformanceBudgetIdentityInput) -> PerformanceBudgetIdentity {
    PerformanceBudgetIdentity {
        record_kind: PERFORMANCE_BUDGET_IDENTITY_RECORD_KIND.to_string(),
        schema_version: STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION,
        performance_profile_ref: input.performance_profile_ref.clone(),
        row_identity_ref: input.row_identity_ref.clone(),
        extension_identity: input.extension_identity.clone(),
        extension_version: input.extension_version.clone(),
        package_id: input.package_id.clone(),
        performance_budget_version: input.performance_budget_version,
        publisher_namespace: input.publisher_namespace.clone(),
        benchmark_evidence_ref: input.benchmark_evidence_ref.clone(),
        publisher_trust_tier_class: input.publisher_trust_tier_class.clone(),
        lifecycle_state_class: input.lifecycle_state_class.clone(),
    }
}

fn measurement_record(input: &PerformanceMeasurementInput) -> PerformanceMeasurement {
    PerformanceMeasurement {
        record_kind: PERFORMANCE_MEASUREMENT_RECORD_KIND.to_string(),
        schema_version: STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION,
        budget_axis_class: input.budget_axis_class.clone(),
        measured_p50: input.measured_p50,
        measured_p95: input.measured_p95,
        sample_count: input.sample_count,
        corpus_metadata_ref: input.corpus_metadata_ref.clone(),
        benchmark_trace_ref: input.benchmark_trace_ref.clone(),
        measurement_freshness_class: input.measurement_freshness_class.clone(),
        trace_attested: input.trace_attested,
        summary_label: input.summary_label.clone(),
    }
}

fn enforcement_record(input: &PerformanceBudgetEnforcementInput) -> PerformanceBudgetEnforcement {
    PerformanceBudgetEnforcement {
        record_kind: PERFORMANCE_BUDGET_ENFORCEMENT_RECORD_KIND.to_string(),
        schema_version: STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION,
        budget_status_class: input.budget_status_class.clone(),
        published_p50_budget: input.published_p50_budget,
        published_p95_budget: input.published_p95_budget,
        enforcement_mode_class: input.enforcement_mode_class.clone(),
        threshold_adjustment_class: input.threshold_adjustment_class.clone(),
        budget_profile_ref: input.budget_profile_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn waiver_record(input: &PerformanceBudgetWaiverInput) -> PerformanceBudgetWaiver {
    PerformanceBudgetWaiver {
        record_kind: PERFORMANCE_BUDGET_WAIVER_RECORD_KIND.to_string(),
        schema_version: STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION,
        waiver_state_class: input.waiver_state_class.clone(),
        waiver_ref: input.waiver_ref.clone(),
        waiver_authority_class: input.waiver_authority_class.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn cost_explanation_record(input: &PerformanceCostExplanationInput) -> PerformanceCostExplanation {
    PerformanceCostExplanation {
        record_kind: PERFORMANCE_COST_EXPLANATION_RECORD_KIND.to_string(),
        schema_version: STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION,
        cost_class: input.cost_class.clone(),
        dominant_cost_factor_class: input.dominant_cost_factor_class.clone(),
        cost_explained: input.cost_explained,
        explanation_ref: input.explanation_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn permission_posture_record(
    input: &PerformancePermissionPostureInput,
) -> PerformancePermissionPosture {
    PerformancePermissionPosture {
        record_kind: PERFORMANCE_PERMISSION_POSTURE_RECORD_KIND.to_string(),
        schema_version: STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION,
        declared_permission_ref: input.declared_permission_ref.clone(),
        effective_permission_ref: input.effective_permission_ref.clone(),
        widened: input.widened,
        reconsent_required: input.reconsent_required,
        summary_label: input.summary_label.clone(),
    }
}

fn compatibility_record(input: &PerformanceCompatibilityInput) -> PerformanceCompatibility {
    PerformanceCompatibility {
        record_kind: PERFORMANCE_COMPATIBILITY_RECORD_KIND.to_string(),
        schema_version: STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION,
        compatibility_label_class: input.compatibility_label_class.clone(),
        scorecard_ref: input.scorecard_ref.clone(),
        compatibility_verified: input.compatibility_verified,
        summary_label: input.summary_label.clone(),
    }
}

fn install_posture_record(input: &PerformanceInstallPostureInput) -> PerformanceInstallPosture {
    PerformanceInstallPosture {
        record_kind: PERFORMANCE_INSTALL_POSTURE_RECORD_KIND.to_string(),
        schema_version: STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION,
        install_scope_class: input.install_scope_class.clone(),
        install_scope_disclosed: input.install_scope_disclosed,
        revocation_posture_class: input.revocation_posture_class.clone(),
        mirrorability_class: input.mirrorability_class.clone(),
        rollback_supported: input.rollback_supported,
        summary_label: input.summary_label.clone(),
    }
}

fn claim_record(
    input: &PerformanceQualificationClaimInput,
    posture: &BudgetPosture<'_>,
) -> PerformanceQualificationClaim {
    let derived = derive_effective_tier(&input.claimed_tier, &input.claim_basis_class, posture);
    PerformanceQualificationClaim {
        record_kind: PERFORMANCE_QUALIFICATION_CLAIM_RECORD_KIND.to_string(),
        schema_version: STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION,
        claimed_tier: input.claimed_tier.clone(),
        effective_tier: derived.effective_tier,
        support_claim_class: derived.support_claim,
        claim_basis_class: input.claim_basis_class.clone(),
        downgraded: derived.downgraded,
        downgrade_reasons: derived.downgrade_reasons,
        summary_label: input.summary_label.clone(),
    }
}

fn banner_record(posture: &BudgetPosture<'_>) -> PerformanceDowngradedBanner {
    let must_display = performance_requires_warning(posture);
    let banner_reason_class = if must_display {
        banner_reason_for(posture)
    } else {
        None
    };
    let summary_label = if must_display {
        format!(
            "Performance row requires review before install or enablement ({}).",
            banner_reason_class.clone().unwrap_or_default()
        )
    } else {
        "Performance budget hardened: cost within published budget, enforced, freshly measured, attested, and explained."
            .to_string()
    };
    PerformanceDowngradedBanner {
        record_kind: PERFORMANCE_DOWNGRADED_BANNER_RECORD_KIND.to_string(),
        schema_version: STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION,
        must_display,
        banner_reason_class,
        summary_label,
    }
}

fn inspection_record(
    packet_id: &str,
    posture: &BudgetPosture<'_>,
    claim: &PerformanceQualificationClaim,
    banner: &PerformanceDowngradedBanner,
) -> StablePerformanceBudgetInspection {
    let stable_claim = STABLE_TIERS.contains(&claim.effective_tier.as_str());

    StablePerformanceBudgetInspection {
        record_kind: STABLE_PERFORMANCE_BUDGET_INSPECTION_RECORD_KIND.to_string(),
        schema_version: STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION,
        packet_id_ref: packet_id.to_string(),
        effective_tier: claim.effective_tier.clone(),
        stable_claim,
        profile_version_current: posture.identity.profile_version_current(),
        trust_tier_class: posture.identity.publisher_trust_tier_class.clone(),
        lifecycle_runnable: posture.identity.lifecycle_runnable(),
        budget_axis_class: posture.measurement.budget_axis_class.clone(),
        measured_p50: posture.measurement.measured_p50,
        measured_p95: posture.measurement.measured_p95,
        published_p50_budget: posture.enforcement.published_p50_budget,
        published_p95_budget: posture.enforcement.published_p95_budget,
        budget_status_class: posture.enforcement.budget_status_class.clone(),
        within_budget: posture.enforcement.within_budget(),
        budget_enforced: posture.enforcement.enforced(),
        measurement_freshness_class: posture.measurement.measurement_freshness_class.clone(),
        trace_attested: posture.measurement.trace_attested,
        threshold_adjustment_class: posture.enforcement.threshold_adjustment_class.clone(),
        waiver_state_class: posture.waiver.waiver_state_class.clone(),
        cost_class: posture.cost_explanation.cost_class.clone(),
        cost_explained: posture.cost_explanation.cost_explained,
        permissions_not_widened: !posture.permission_posture.widened,
        compatibility_label_class: posture.compatibility.compatibility_label_class.clone(),
        compatibility_verified: posture.compatibility.compatibility_verified,
        revocation_posture_class: posture.install_posture.revocation_posture_class.clone(),
        mirrorability_class: posture.install_posture.mirrorability_class.clone(),
        downgraded: claim.downgraded,
        downgraded_banner_required: banner.must_display,
        attribution_complete: posture.attribution_complete,
        summary_label: claim.summary_label.clone(),
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &StablePerformanceBudgetInput,
) -> Result<(), StablePerformanceBudgetValidationError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    let id = &input.identity;
    ensure_nonempty(
        &id.performance_profile_ref,
        "identity.performance_profile_ref",
    )?;
    if !id
        .performance_profile_ref
        .starts_with("performance_profile:")
    {
        return Err(err(
            "identity.performance_profile_ref must start with 'performance_profile:'",
        ));
    }
    ensure_nonempty(&id.row_identity_ref, "identity.row_identity_ref")?;
    ensure_nonempty(&id.extension_identity, "identity.extension_identity")?;
    ensure_nonempty(&id.extension_version, "identity.extension_version")?;
    ensure_nonempty(&id.package_id, "identity.package_id")?;
    ensure_nonempty(&id.publisher_namespace, "identity.publisher_namespace")?;
    ensure_nonempty(
        &id.benchmark_evidence_ref,
        "identity.benchmark_evidence_ref",
    )?;
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

    let m = &input.measurement;
    ensure_token(
        BUDGET_AXIS_CLASSES,
        &m.budget_axis_class,
        "measurement.budget_axis_class",
    )?;
    ensure_token(
        MEASUREMENT_FRESHNESS_CLASSES,
        &m.measurement_freshness_class,
        "measurement.measurement_freshness_class",
    )?;
    ensure_nonempty(&m.corpus_metadata_ref, "measurement.corpus_metadata_ref")?;
    ensure_nonempty(&m.benchmark_trace_ref, "measurement.benchmark_trace_ref")?;

    let e = &input.enforcement;
    ensure_token(
        BUDGET_STATUS_CLASSES,
        &e.budget_status_class,
        "enforcement.budget_status_class",
    )?;
    ensure_token(
        ENFORCEMENT_MODE_CLASSES,
        &e.enforcement_mode_class,
        "enforcement.enforcement_mode_class",
    )?;
    ensure_token(
        THRESHOLD_ADJUSTMENT_CLASSES,
        &e.threshold_adjustment_class,
        "enforcement.threshold_adjustment_class",
    )?;
    ensure_nonempty(&e.budget_profile_ref, "enforcement.budget_profile_ref")?;

    let w = &input.waiver;
    ensure_token(
        WAIVER_STATE_CLASSES,
        &w.waiver_state_class,
        "waiver.waiver_state_class",
    )?;
    if w.waiver_state_class == "none" {
        if w.waiver_authority_class.is_some() {
            return Err(err(
                "waiver.waiver_authority_class must be unset when waiver_state is none",
            ));
        }
    } else {
        ensure_nonempty(&w.waiver_ref, "waiver.waiver_ref")?;
        match &w.waiver_authority_class {
            Some(authority) => ensure_token(
                WAIVER_AUTHORITY_CLASSES,
                authority,
                "waiver.waiver_authority_class",
            )?,
            None => {
                return Err(err(
                    "waiver.waiver_authority_class must be set when a waiver exists",
                ))
            }
        }
    }

    let c = &input.cost_explanation;
    ensure_token(COST_CLASSES, &c.cost_class, "cost_explanation.cost_class")?;
    ensure_token(
        BUDGET_AXIS_CLASSES,
        &c.dominant_cost_factor_class,
        "cost_explanation.dominant_cost_factor_class",
    )?;
    ensure_nonempty(&c.explanation_ref, "cost_explanation.explanation_ref")?;

    let perm = &input.permission_posture;
    ensure_nonempty(
        &perm.declared_permission_ref,
        "permission_posture.declared_permission_ref",
    )?;
    ensure_nonempty(
        &perm.effective_permission_ref,
        "permission_posture.effective_permission_ref",
    )?;

    let compat = &input.compatibility;
    ensure_token(
        COMPATIBILITY_LABEL_CLASSES,
        &compat.compatibility_label_class,
        "compatibility.compatibility_label_class",
    )?;
    ensure_nonempty(&compat.scorecard_ref, "compatibility.scorecard_ref")?;

    let inst = &input.install_posture;
    ensure_token(
        INSTALL_SCOPE_CLASSES,
        &inst.install_scope_class,
        "install_posture.install_scope_class",
    )?;
    ensure_token(
        REVOCATION_POSTURE_CLASSES,
        &inst.revocation_posture_class,
        "install_posture.revocation_posture_class",
    )?;
    ensure_token(
        MIRRORABILITY_CLASSES,
        &inst.mirrorability_class,
        "install_posture.mirrorability_class",
    )?;

    let claim = &input.claim;
    ensure_token(STABILITY_TIERS, &claim.claimed_tier, "claim.claimed_tier")?;
    ensure_token(
        CLAIM_BASIS_CLASSES,
        &claim.claim_basis_class,
        "claim.claim_basis_class",
    )?;

    for surface in &input.consumer_surfaces {
        ensure_token(
            STABLE_PERFORMANCE_BUDGET_CONSUMER_SURFACES,
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
    identity: &PerformanceBudgetIdentity,
) -> Result<(), StablePerformanceBudgetValidationError> {
    ensure_eq(
        identity.record_kind.as_str(),
        PERFORMANCE_BUDGET_IDENTITY_RECORD_KIND,
        "identity record_kind",
    )?;
    ensure_eq_u32(
        identity.schema_version,
        STABLE_PERFORMANCE_BUDGET_SCHEMA_VERSION,
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

fn validate_measurement(
    m: &PerformanceMeasurement,
) -> Result<(), StablePerformanceBudgetValidationError> {
    ensure_eq(
        m.record_kind.as_str(),
        PERFORMANCE_MEASUREMENT_RECORD_KIND,
        "measurement record_kind",
    )?;
    ensure_token(
        BUDGET_AXIS_CLASSES,
        &m.budget_axis_class,
        "measurement budget_axis_class",
    )?;
    ensure_token(
        MEASUREMENT_FRESHNESS_CLASSES,
        &m.measurement_freshness_class,
        "measurement measurement_freshness_class",
    )?;
    ensure_nonempty(&m.corpus_metadata_ref, "measurement corpus_metadata_ref")?;
    ensure_nonempty(&m.benchmark_trace_ref, "measurement benchmark_trace_ref")?;
    Ok(())
}

fn validate_enforcement(
    e: &PerformanceBudgetEnforcement,
) -> Result<(), StablePerformanceBudgetValidationError> {
    ensure_eq(
        e.record_kind.as_str(),
        PERFORMANCE_BUDGET_ENFORCEMENT_RECORD_KIND,
        "enforcement record_kind",
    )?;
    ensure_token(
        BUDGET_STATUS_CLASSES,
        &e.budget_status_class,
        "enforcement budget_status_class",
    )?;
    ensure_token(
        ENFORCEMENT_MODE_CLASSES,
        &e.enforcement_mode_class,
        "enforcement enforcement_mode_class",
    )?;
    ensure_token(
        THRESHOLD_ADJUSTMENT_CLASSES,
        &e.threshold_adjustment_class,
        "enforcement threshold_adjustment_class",
    )?;
    ensure_nonempty(&e.budget_profile_ref, "enforcement budget_profile_ref")?;
    Ok(())
}

fn validate_waiver(
    w: &PerformanceBudgetWaiver,
) -> Result<(), StablePerformanceBudgetValidationError> {
    ensure_eq(
        w.record_kind.as_str(),
        PERFORMANCE_BUDGET_WAIVER_RECORD_KIND,
        "waiver record_kind",
    )?;
    ensure_token(
        WAIVER_STATE_CLASSES,
        &w.waiver_state_class,
        "waiver waiver_state_class",
    )?;
    if w.waiver_state_class == "none" {
        if w.waiver_authority_class.is_some() {
            return Err(err(
                "waiver authority must be unset when waiver_state is none",
            ));
        }
    } else {
        ensure_nonempty(&w.waiver_ref, "waiver waiver_ref")?;
        match &w.waiver_authority_class {
            Some(authority) => ensure_token(
                WAIVER_AUTHORITY_CLASSES,
                authority,
                "waiver waiver_authority_class",
            )?,
            None => return Err(err("waiver authority must be set when a waiver exists")),
        }
    }
    Ok(())
}

fn validate_cost_explanation(
    c: &PerformanceCostExplanation,
) -> Result<(), StablePerformanceBudgetValidationError> {
    ensure_eq(
        c.record_kind.as_str(),
        PERFORMANCE_COST_EXPLANATION_RECORD_KIND,
        "cost_explanation record_kind",
    )?;
    ensure_token(COST_CLASSES, &c.cost_class, "cost_explanation cost_class")?;
    ensure_token(
        BUDGET_AXIS_CLASSES,
        &c.dominant_cost_factor_class,
        "cost_explanation dominant_cost_factor_class",
    )?;
    ensure_nonempty(&c.explanation_ref, "cost_explanation explanation_ref")?;
    Ok(())
}

fn validate_permission_posture(
    perm: &PerformancePermissionPosture,
) -> Result<(), StablePerformanceBudgetValidationError> {
    ensure_eq(
        perm.record_kind.as_str(),
        PERFORMANCE_PERMISSION_POSTURE_RECORD_KIND,
        "permission_posture record_kind",
    )?;
    ensure_nonempty(
        &perm.declared_permission_ref,
        "permission_posture declared_permission_ref",
    )?;
    ensure_nonempty(
        &perm.effective_permission_ref,
        "permission_posture effective_permission_ref",
    )?;
    Ok(())
}

fn validate_compatibility(
    compat: &PerformanceCompatibility,
) -> Result<(), StablePerformanceBudgetValidationError> {
    ensure_eq(
        compat.record_kind.as_str(),
        PERFORMANCE_COMPATIBILITY_RECORD_KIND,
        "compatibility record_kind",
    )?;
    ensure_token(
        COMPATIBILITY_LABEL_CLASSES,
        &compat.compatibility_label_class,
        "compatibility compatibility_label_class",
    )?;
    ensure_nonempty(&compat.scorecard_ref, "compatibility scorecard_ref")?;
    Ok(())
}

fn validate_install_posture(
    inst: &PerformanceInstallPosture,
) -> Result<(), StablePerformanceBudgetValidationError> {
    ensure_eq(
        inst.record_kind.as_str(),
        PERFORMANCE_INSTALL_POSTURE_RECORD_KIND,
        "install_posture record_kind",
    )?;
    ensure_token(
        INSTALL_SCOPE_CLASSES,
        &inst.install_scope_class,
        "install_posture install_scope_class",
    )?;
    ensure_token(
        REVOCATION_POSTURE_CLASSES,
        &inst.revocation_posture_class,
        "install_posture revocation_posture_class",
    )?;
    ensure_token(
        MIRRORABILITY_CLASSES,
        &inst.mirrorability_class,
        "install_posture mirrorability_class",
    )?;
    Ok(())
}

fn validate_claim(
    claim: &PerformanceQualificationClaim,
) -> Result<(), StablePerformanceBudgetValidationError> {
    ensure_eq(
        claim.record_kind.as_str(),
        PERFORMANCE_QUALIFICATION_CLAIM_RECORD_KIND,
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
            PERFORMANCE_BUDGET_DOWNGRADE_REASONS,
            reason,
            "claim downgrade_reason",
        )?;
    }
    Ok(())
}

fn validate_banner(
    banner: &PerformanceDowngradedBanner,
) -> Result<(), StablePerformanceBudgetValidationError> {
    ensure_eq(
        banner.record_kind.as_str(),
        PERFORMANCE_DOWNGRADED_BANNER_RECORD_KIND,
        "banner record_kind",
    )?;
    if let Some(reason) = &banner.banner_reason_class {
        ensure_token(
            PERFORMANCE_BUDGET_DOWNGRADE_REASONS,
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

/// Cross-checks the inspected measurement against the published budget so the
/// numeric p50 / p95 cost and the budget status cannot contradict each other.
fn validate_budget_numbers(
    m: &PerformanceMeasurement,
    e: &PerformanceBudgetEnforcement,
) -> Result<(), StablePerformanceBudgetValidationError> {
    if m.measured_p50 > m.measured_p95 {
        return Err(err("measured_p50 must not exceed measured_p95"));
    }
    if e.published_p50_budget > 0
        && e.published_p95_budget > 0
        && e.published_p50_budget > e.published_p95_budget
    {
        return Err(err(
            "published_p50_budget must not exceed published_p95_budget",
        ));
    }
    match e.budget_status_class.as_str() {
        "within_budget" => {
            if e.published_p50_budget == 0 || e.published_p95_budget == 0 {
                return Err(err(
                    "within_budget status must publish nonzero p50/p95 budget ceilings",
                ));
            }
            if m.measured_p50 > e.published_p50_budget || m.measured_p95 > e.published_p95_budget {
                return Err(err(
                    "within_budget status requires measured p50/p95 within the published budget",
                ));
            }
        }
        "over_budget" => {
            if e.published_p50_budget == 0 || e.published_p95_budget == 0 {
                return Err(err(
                    "over_budget status must publish nonzero p50/p95 budget ceilings",
                ));
            }
            if m.measured_p50 <= e.published_p50_budget && m.measured_p95 <= e.published_p95_budget
            {
                return Err(err(
                    "over_budget status requires measured p50 or p95 to exceed the published budget",
                ));
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_inspection(
    inspection: &StablePerformanceBudgetInspection,
    packet: &StablePerformanceBudgetPacket,
) -> Result<(), StablePerformanceBudgetValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        STABLE_PERFORMANCE_BUDGET_INSPECTION_RECORD_KIND,
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
    if inspection.downgraded != packet.claim.downgraded {
        return Err(err("inspection downgraded is inconsistent"));
    }
    if inspection.downgraded_banner_required != packet.downgraded_banner.must_display {
        return Err(err("inspection downgraded_banner_required is inconsistent"));
    }
    if inspection.attribution_complete != packet.attribution_complete() {
        return Err(err("inspection attribution_complete is inconsistent"));
    }
    if inspection.within_budget != packet.enforcement.within_budget() {
        return Err(err("inspection within_budget is inconsistent"));
    }
    if inspection.budget_enforced != packet.enforcement.enforced() {
        return Err(err("inspection budget_enforced is inconsistent"));
    }
    if inspection.permissions_not_widened == packet.permission_posture.widened {
        return Err(err("inspection permissions_not_widened is inconsistent"));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn err(message: impl Into<String>) -> StablePerformanceBudgetValidationError {
    StablePerformanceBudgetValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(
    left: T,
    right: T,
    field: &str,
) -> Result<(), StablePerformanceBudgetValidationError>
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
) -> Result<(), StablePerformanceBudgetValidationError> {
    if left != right {
        return Err(err(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), StablePerformanceBudgetValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), StablePerformanceBudgetValidationError> {
    if !tokens.contains(&value) {
        return Err(err(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
    }
    Ok(())
}
