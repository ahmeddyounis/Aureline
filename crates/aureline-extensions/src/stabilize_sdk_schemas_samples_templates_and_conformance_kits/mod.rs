//! Stabilize the SDK schemas, sample extensions, project templates, and
//! conformance kits for the stable extension-author lane.
//!
//! The SDK v1 starter pack (see [`crate::sdk_v1`]) owns the first inspectable
//! *beta* author lane — the published typed API surfaces, the manifest-authoring
//! guides, and the canonical sample pack. This module owns the layer above it —
//! the **stable, conformance-backed author-lane truth** a claimed stable
//! extension-author surface carries, and the **stability qualification** that
//! truth is allowed to claim.
//!
//! A stable author lane must bind, machine-readably:
//!
//! - the **kit artifacts** the lane ships — the published **SDK schemas**, the
//!   canonical **sample extensions** (wasm and external-host), the scaffolding
//!   **project templates**, and the **conformance kit** — each with an artifact
//!   kind, a host class, a published-version ref, a conformance state, and
//!   whether it pins the published schema version,
//! - the **aggregate conformance summary** across those artifacts (so the lane
//!   never claims stable while a sample, template, or kit is nonconformant),
//! - the **activation-budget** instrumentation for the lane's worst-case sample
//!   (so an unbounded activation cost can never ride a stable claim), and
//! - the **publisher-continuity** binding that keeps the author-lane provenance
//!   current.
//!
//! The central rule mirrors the rest of the stable line: a **stable** author-lane
//! claim may never be implied from a catalog row alone. A row that renders a
//! `stable` author-lane badge must pin the published SDK version, be
//! conformance-backed (not catalog-asserted), keep its publisher trust tier out
//! of quarantine, stay on an installable lifecycle, ship every required artifact
//! kind, pin every artifact to the published schema version, keep every artifact
//! conformant, keep its activation cost bounded and within budget, keep its
//! publisher continuity current, and be fully attributed. When any of those
//! fails, the visible tier is **automatically narrowed below Stable** (to `beta`
//! or `preview`, or `withdrawn` when the lane cannot be trusted to author against
//! at all) rather than left asserting an author-lane readiness the conformance kit
//! cannot back.
//!
//! Three guardrails are encoded so they cannot be papered over:
//!
//! - **No ambient extension privilege.** A sample or template that scaffolds an
//!   unbounded permission set (`declares_bounded_permissions == false`) widens
//!   author-template authority implicitly; the lane narrows below Stable and a
//!   lane-review banner is raised.
//! - **No catalog-only trust.** A `catalog_asserted_only` claim basis can never
//!   back a stable author-lane claim; it narrows below Stable.
//! - **No unbounded activation cost.** An `unbounded` activation budget withdraws
//!   the lane outright; an `over_budget` budget narrows to `beta`.
//!
//! The companion schema lives at
//! [`schemas/extensions/stable_sdk_author_lane.schema.json`](../../../../schemas/extensions/stable_sdk_author_lane.schema.json).
//! Canonical fixtures live under
//! `fixtures/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every stable SDK author-lane record.
pub const STABLE_SDK_AUTHOR_LANE_SCHEMA_VERSION: u32 = 1;

/// The published, stable SDK major version. A `stable` author-lane claim must pin
/// exactly this version; any other version narrows below Stable.
pub const STABLE_SDK_AUTHOR_LANE_PUBLISHED_SDK_VERSION: u32 = 1;

/// The published, stable schema version each kit artifact must pin to keep a
/// stable claim.
pub const STABLE_SDK_AUTHOR_LANE_PUBLISHED_ARTIFACT_VERSION: u32 = 1;

/// Canonical schema path the packet cites.
pub const STABLE_SDK_AUTHOR_LANE_SCHEMA_REF: &str =
    "schemas/extensions/stable_sdk_author_lane.schema.json";

/// Record-kind tag for [`StableSdkAuthorLanePacket`].
pub const STABLE_SDK_AUTHOR_LANE_PACKET_RECORD_KIND: &str = "stable_sdk_author_lane_packet";

/// Record-kind tag for [`SdkAuthorLaneIdentity`].
pub const SDK_AUTHOR_LANE_IDENTITY_RECORD_KIND: &str = "stable_sdk_author_lane_identity";

/// Record-kind tag for [`SdkKitArtifact`].
pub const SDK_KIT_ARTIFACT_RECORD_KIND: &str = "stable_sdk_kit_artifact";

/// Record-kind tag for [`SdkConformanceSummary`].
pub const SDK_CONFORMANCE_SUMMARY_RECORD_KIND: &str = "stable_sdk_conformance_summary";

/// Record-kind tag for [`SdkActivationBudget`].
pub const SDK_ACTIVATION_BUDGET_RECORD_KIND: &str = "stable_sdk_activation_budget";

/// Record-kind tag for [`SdkPublisherContinuity`].
pub const SDK_PUBLISHER_CONTINUITY_RECORD_KIND: &str = "stable_sdk_publisher_continuity";

/// Record-kind tag for [`SdkAuthorLaneQualificationClaim`].
pub const SDK_AUTHOR_LANE_QUALIFICATION_CLAIM_RECORD_KIND: &str =
    "stable_sdk_author_lane_qualification_claim";

/// Record-kind tag for [`DowngradedLaneBanner`].
pub const DOWNGRADED_LANE_BANNER_RECORD_KIND: &str = "stable_sdk_downgraded_lane_banner";

/// Record-kind tag for [`StableSdkAuthorLaneInspection`].
pub const STABLE_SDK_AUTHOR_LANE_INSPECTION_RECORD_KIND: &str =
    "stable_sdk_author_lane_inspection";

/// Record-kind tag for [`StableSdkAuthorLaneSupportExport`].
pub const STABLE_SDK_AUTHOR_LANE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stable_sdk_author_lane_support_export";

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

/// Lifecycle states a stable author-lane claim may keep (installable / runnable).
pub const INSTALLABLE_LIFECYCLE_STATES: &[&str] =
    &["installed", "pending_activation", "active", "recovered"];

/// Closed kit-artifact-kind vocabulary. A stable lane must ship every kind.
pub const ARTIFACT_KIND_CLASSES: &[&str] =
    &["sdk_schema", "sample_extension", "project_template", "conformance_kit"];

/// The artifact kinds a stable author lane must include at least one of.
pub const REQUIRED_ARTIFACT_KINDS: &[&str] =
    &["sdk_schema", "sample_extension", "project_template", "conformance_kit"];

/// Closed host-class vocabulary carried on each artifact.
pub const ARTIFACT_HOST_CLASSES: &[&str] =
    &["wasm", "external_host", "webview", "headless", "not_applicable"];

/// Closed conformance-state vocabulary. `conformant` is the only state a stable
/// claim may keep.
pub const CONFORMANCE_STATE_CLASSES: &[&str] =
    &["conformant", "nonconformant", "not_run", "waived"];

/// Closed activation-budget vocabulary. `within_budget` is the only state a stable
/// claim may keep.
pub const ACTIVATION_BUDGET_CLASSES: &[&str] =
    &["within_budget", "over_budget", "unbounded", "not_measured"];

/// Closed publisher-continuity vocabulary. `current` is the only state a stable
/// claim may keep.
pub const PUBLISHER_CONTINUITY_CLASSES: &[&str] = &["current", "stale", "missing", "revoked"];

/// Closed set of switch-readiness stability tiers.
pub const STABILITY_TIERS: &[&str] = &["stable", "beta", "preview", "withdrawn"];

/// Tiers that count as a *stable* author-lane claim.
pub const STABLE_TIERS: &[&str] = &["stable"];

/// Closed claim-basis vocabulary. `catalog_asserted_only` may never back stable.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["conformance_backed", "catalog_asserted_only"];

/// Closed support-claim vocabulary a tier may imply.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_author_lane_ready_claim",
    "beta_author_lane_partial_claim",
    "preview_author_lane_experimental_claim",
    "withdrawn_no_author_lane_claim",
];

/// Closed set of reasons that narrow a stable author-lane claim below Stable.
pub const SDK_AUTHOR_LANE_DOWNGRADE_REASONS: &[&str] = &[
    "sdk_version_not_published",
    "catalog_only_trust_not_conformance_backed",
    "trust_tier_quarantined",
    "lifecycle_not_installable",
    "missing_required_artifact_kind",
    "artifact_below_published_version",
    "artifact_above_published_version",
    "artifact_nonconformant",
    "artifact_conformance_not_run",
    "ambient_template_privilege",
    "activation_cost_unbounded",
    "activation_cost_over_budget",
    "activation_cost_not_measured",
    "publisher_continuity_revoked",
    "publisher_continuity_missing",
    "publisher_continuity_stale",
    "attribution_incomplete",
];

/// Reasons that narrow all the way to `withdrawn` (the lane cannot be trusted to
/// author against at all).
const WITHDRAWN_CLASS_REASONS: &[&str] = &[
    "lifecycle_not_installable",
    "missing_required_artifact_kind",
    "artifact_below_published_version",
    "artifact_nonconformant",
    "ambient_template_privilege",
    "activation_cost_unbounded",
    "publisher_continuity_revoked",
];

/// Reasons that narrow to `preview` (a structural/trust shortfall).
const PREVIEW_CLASS_REASONS: &[&str] = &[
    "sdk_version_not_published",
    "catalog_only_trust_not_conformance_backed",
    "trust_tier_quarantined",
    "artifact_conformance_not_run",
    "activation_cost_not_measured",
    "publisher_continuity_missing",
    "attribution_incomplete",
];

/// Reasons whose only effect is a safe narrowing to `beta`.
const BETA_CLASS_REASONS: &[&str] = &[
    "artifact_above_published_version",
    "activation_cost_over_budget",
    "publisher_continuity_stale",
];

/// Closed set of consumer surfaces that ingest this packet.
pub const STABLE_SDK_AUTHOR_LANE_CONSUMER_SURFACES: &[&str] = &[
    "sdk_docs_surface",
    "author_onboarding",
    "publication_review",
    "conformance_dashboard",
    "diagnostics",
    "support_export",
    "docs_help_surface",
    "release_packet",
    "cli_inspector",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a stable SDK author-lane packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableSdkAuthorLaneInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity input.
    pub identity: SdkAuthorLaneIdentityInput,
    /// Kit artifacts (SDK schemas, samples, templates, conformance kit).
    #[serde(default)]
    pub artifacts: Vec<SdkKitArtifactInput>,
    /// Activation-budget input for the lane's worst-case sample.
    pub activation_budget: SdkActivationBudgetInput,
    /// Publisher-continuity input.
    pub publisher_continuity: SdkPublisherContinuityInput,
    /// Stability qualification claim input.
    pub claim: SdkAuthorLaneQualificationClaimInput,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input for [`SdkAuthorLaneIdentity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkAuthorLaneIdentityInput {
    /// Ref to the SDK starter-pack record this lane stabilizes.
    pub starter_pack_ref: String,
    /// Opaque author-lane identity ref.
    pub lane_identity_ref: String,
    /// Published SDK version this lane pins.
    pub sdk_version: u32,
    /// Ref to the source package the kit came from.
    pub source_package_ref: String,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

/// Input for one [`SdkKitArtifact`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkKitArtifactInput {
    /// Stable artifact id.
    pub artifact_id: String,
    /// Artifact kind: schema, sample, template, or conformance kit.
    pub artifact_kind_class: String,
    /// Host class the artifact targets.
    pub host_class: String,
    /// Opaque ref to the published version of this artifact.
    pub published_version_ref: String,
    /// Schema version the artifact pins. A stable claim requires the published
    /// artifact version.
    pub artifact_schema_version: u32,
    /// Conformance state for the artifact.
    pub conformance_state_class: String,
    /// Ref to the conformance report backing the state.
    pub conformance_report_ref: String,
    /// Whether the artifact scaffolds a bounded permission set. A sample or
    /// template with this false scaffolds ambient privilege.
    pub declares_bounded_permissions: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`SdkActivationBudget`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkActivationBudgetInput {
    /// Activation-budget posture for the worst-case sample.
    pub budget_class: String,
    /// Ref to the measured activation cost.
    pub measured_cost_ref: String,
    /// Ref to the declared activation-budget ceiling.
    pub budget_ceiling_ref: String,
    /// Number of samples whose activation cost was measured.
    pub measured_sample_count: usize,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`SdkPublisherContinuity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkPublisherContinuityInput {
    /// Continuity state for the lane publisher.
    pub continuity_state_class: String,
    /// Opaque ref to the publisher-continuity packet, when one is bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuity_packet_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`SdkAuthorLaneQualificationClaim`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkAuthorLaneQualificationClaimInput {
    /// Author-lane tier claimed by the row.
    pub claimed_tier: String,
    /// Claim basis: conformance-backed vs catalog-asserted only.
    pub claim_basis_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// Identity shared across every surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkAuthorLaneIdentity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ref to the SDK starter-pack record this lane stabilizes.
    pub starter_pack_ref: String,
    /// Opaque author-lane identity ref.
    pub lane_identity_ref: String,
    /// Published SDK version this lane pins.
    pub sdk_version: u32,
    /// Source package the kit came from.
    pub source_package_ref: String,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

impl SdkAuthorLaneIdentity {
    /// Returns true when the lane pins the published stable SDK version.
    pub fn sdk_version_current(&self) -> bool {
        self.sdk_version == STABLE_SDK_AUTHOR_LANE_PUBLISHED_SDK_VERSION
    }

    /// Returns true when the lifecycle is installable.
    pub fn lifecycle_installable(&self) -> bool {
        INSTALLABLE_LIFECYCLE_STATES.contains(&self.lifecycle_state_class.as_str())
    }
}

/// One kit artifact: an SDK schema, a sample extension, a project template, or
/// the conformance kit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkKitArtifact {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable artifact id.
    pub artifact_id: String,
    /// Artifact kind.
    pub artifact_kind_class: String,
    /// Host class the artifact targets.
    pub host_class: String,
    /// Opaque ref to the published version of this artifact.
    pub published_version_ref: String,
    /// Schema version the artifact pins.
    pub artifact_schema_version: u32,
    /// Conformance state for the artifact.
    pub conformance_state_class: String,
    /// Ref to the conformance report backing the state.
    pub conformance_report_ref: String,
    /// Whether the artifact scaffolds a bounded permission set.
    pub declares_bounded_permissions: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl SdkKitArtifact {
    /// Returns true when the artifact pins the published schema version.
    pub fn version_current(&self) -> bool {
        self.artifact_schema_version == STABLE_SDK_AUTHOR_LANE_PUBLISHED_ARTIFACT_VERSION
    }

    /// Returns true when the artifact is conformant.
    pub fn conformant(&self) -> bool {
        self.conformance_state_class == "conformant"
    }

    /// Returns true when this artifact is a sample or a template (a scaffold that
    /// could carry ambient privilege).
    pub fn is_scaffold(&self) -> bool {
        matches!(
            self.artifact_kind_class.as_str(),
            "sample_extension" | "project_template"
        )
    }

    /// Returns true when a scaffold artifact widens template authority implicitly
    /// by not declaring a bounded permission set.
    pub fn widens_template_authority(&self) -> bool {
        self.is_scaffold() && !self.declares_bounded_permissions
    }
}

/// Aggregate conformance summary across the kit artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkConformanceSummary {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Total number of kit artifacts.
    pub artifact_count: usize,
    /// Number of conformant artifacts.
    pub conformant_count: usize,
    /// Number of nonconformant artifacts.
    pub nonconformant_count: usize,
    /// Number of artifacts whose conformance was not run.
    pub not_run_count: usize,
    /// Number of artifacts with a waived conformance state.
    pub waived_count: usize,
    /// Artifact kinds present in the lane (sorted, deduped).
    pub present_kinds: Vec<String>,
    /// Required artifact kinds that are missing from the lane.
    pub missing_required_kinds: Vec<String>,
    /// True when every artifact is conformant.
    pub all_conformant: bool,
    /// True when every required artifact kind is present.
    pub all_required_kinds_present: bool,
}

impl SdkConformanceSummary {
    /// Returns true when the lane is fully conformant and complete.
    pub fn lane_conformant(&self) -> bool {
        self.all_conformant && self.all_required_kinds_present
    }
}

/// Activation-budget instrumentation for the lane's worst-case sample.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkActivationBudget {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Activation-budget posture.
    pub budget_class: String,
    /// Ref to the measured activation cost.
    pub measured_cost_ref: String,
    /// Ref to the declared activation-budget ceiling.
    pub budget_ceiling_ref: String,
    /// Number of samples whose activation cost was measured.
    pub measured_sample_count: usize,
    /// Reviewable summary.
    pub summary_label: String,
}

impl SdkActivationBudget {
    /// Returns true when the activation cost is bounded and within budget.
    pub fn within_budget(&self) -> bool {
        self.budget_class == "within_budget"
    }

    /// Returns true when the activation cost is unbounded.
    pub fn unbounded(&self) -> bool {
        self.budget_class == "unbounded"
    }
}

/// Publisher-continuity binding for the author lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkPublisherContinuity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Continuity state for the lane publisher.
    pub continuity_state_class: String,
    /// Opaque ref to the publisher-continuity packet, when one is bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuity_packet_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

impl SdkPublisherContinuity {
    /// Returns true when the continuity is current.
    pub fn current(&self) -> bool {
        self.continuity_state_class == "current"
    }
}

/// Stability qualification claim after the posture is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkAuthorLaneQualificationClaim {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Author-lane tier claimed by the row.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim the effective tier is allowed to imply.
    pub support_claim_class: String,
    /// Claim basis: conformance-backed vs catalog-asserted only.
    pub claim_basis_class: String,
    /// True when the claimed tier was narrowed below Stable.
    pub downgraded: bool,
    /// Reasons that narrowed the claim.
    pub downgrade_reasons: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Downgraded-lane banner requirement. Raised whenever a reviewer must see an
/// author-lane shortfall before relying on the lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradedLaneBanner {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// True when a downgraded-lane banner must be displayed.
    pub must_display: bool,
    /// Most-severe applicable banner reason, drawn from the downgrade vocabulary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_reason_class: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Compact inspection row for CLI/headless and dashboard surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableSdkAuthorLaneInspection {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet inspected by this row.
    pub packet_id_ref: String,
    /// Effective author-lane tier.
    pub effective_tier: String,
    /// True when the claim is a stable author-lane claim.
    pub stable_claim: bool,
    /// True when the lane pins the published SDK version.
    pub sdk_version_current: bool,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// True when the lifecycle is installable.
    pub lifecycle_installable: bool,
    /// True when every artifact is conformant and every required kind is present.
    pub lane_conformant: bool,
    /// Activation-budget posture.
    pub activation_budget_class: String,
    /// True when the activation cost is bounded and within budget.
    pub activation_within_budget: bool,
    /// Publisher-continuity state.
    pub publisher_continuity_class: String,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// True when a downgraded-lane banner is required.
    pub downgraded_lane_banner_required: bool,
    /// True when identity and every artifact are fully attributed.
    pub attribution_complete: bool,
    /// Always false; surfaced so a reviewer can see ambient privilege is forbidden.
    pub ambient_template_privilege_present: bool,
    /// Number of kit artifacts.
    pub artifact_count: usize,
    /// Number of conformant artifacts.
    pub conformant_artifact_count: usize,
    /// Number of nonconformant artifacts.
    pub nonconformant_artifact_count: usize,
    /// Number of artifacts not pinned to the published schema version.
    pub stale_version_artifact_count: usize,
    /// Number of required artifact kinds missing from the lane.
    pub missing_required_kind_count: usize,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Stable SDK author-lane packet consumed by the SDK docs surface, author
/// onboarding, publication review, the conformance dashboard, diagnostics,
/// support export, docs/help, and release packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableSdkAuthorLanePacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity.
    pub identity: SdkAuthorLaneIdentity,
    /// Kit artifacts.
    pub artifacts: Vec<SdkKitArtifact>,
    /// Aggregate conformance summary.
    pub conformance_summary: SdkConformanceSummary,
    /// Activation-budget instrumentation.
    pub activation_budget: SdkActivationBudget,
    /// Publisher-continuity binding.
    pub publisher_continuity: SdkPublisherContinuity,
    /// Stability qualification claim after the posture is applied.
    pub claim: SdkAuthorLaneQualificationClaim,
    /// Downgraded-lane banner requirement.
    pub downgraded_lane_banner: DowngradedLaneBanner,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the packet cites.
    pub source_schema_refs: Vec<String>,
    /// False so a sample/template can never scaffold ambient privilege.
    pub allows_ambient_template_privilege: bool,
    /// False so catalog-only trust cannot back a stable author-lane claim.
    pub allows_catalog_only_trust: bool,
    /// False so an unbounded activation cost can never ride a stable lane.
    pub allows_unbounded_activation_cost: bool,
    /// False so a nonconformant artifact can never ride a stable lane.
    pub allows_nonconformant_stable_claim: bool,
    /// Inspection row.
    pub inspection: StableSdkAuthorLaneInspection,
}

impl StableSdkAuthorLanePacket {
    /// Builds a stable SDK author-lane packet from input, deriving the aggregate
    /// conformance summary and applying the lane posture to the claimed tier so any
    /// required downgrade below Stable is automatic.
    ///
    /// # Errors
    ///
    /// Returns [`StableSdkAuthorLaneValidationError`] when the input violates an
    /// identity, artifact, conformance, budget, continuity, or claim invariant.
    pub fn from_input(
        input: StableSdkAuthorLaneInput,
    ) -> Result<Self, StableSdkAuthorLaneValidationError> {
        validate_input(&input)?;

        let identity = identity_record(&input.identity);
        let artifacts: Vec<SdkKitArtifact> =
            input.artifacts.iter().map(artifact_record).collect();
        let conformance_summary = summarize_conformance(&artifacts);
        let activation_budget = activation_budget_record(&input.activation_budget);
        let publisher_continuity = publisher_continuity_record(&input.publisher_continuity);
        let attribution_complete = attribution_is_complete(&identity, &artifacts);
        let claim = claim_record(
            &input.claim,
            &identity,
            &artifacts,
            &conformance_summary,
            &activation_budget,
            &publisher_continuity,
            attribution_complete,
        );
        let downgraded_lane_banner = banner_record(
            &identity,
            &artifacts,
            &conformance_summary,
            &activation_budget,
            &publisher_continuity,
        );
        let inspection = inspection_record(
            &input.packet_id,
            &identity,
            &artifacts,
            &conformance_summary,
            &activation_budget,
            &publisher_continuity,
            &claim,
            &downgraded_lane_banner,
            attribution_complete,
        );

        let packet = Self {
            record_kind: STABLE_SDK_AUTHOR_LANE_PACKET_RECORD_KIND.to_string(),
            schema_version: STABLE_SDK_AUTHOR_LANE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            identity,
            artifacts,
            conformance_summary,
            activation_budget,
            publisher_continuity,
            claim,
            downgraded_lane_banner,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![STABLE_SDK_AUTHOR_LANE_SCHEMA_REF.to_string()],
            allows_ambient_template_privilege: false,
            allows_catalog_only_trust: false,
            allows_unbounded_activation_cost: false,
            allows_nonconformant_stable_claim: false,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the stable SDK author-lane invariants.
    ///
    /// # Errors
    ///
    /// Returns [`StableSdkAuthorLaneValidationError`] when an invariant is violated.
    pub fn validate(&self) -> Result<(), StableSdkAuthorLaneValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            STABLE_SDK_AUTHOR_LANE_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            STABLE_SDK_AUTHOR_LANE_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_identity(&self.identity)?;
        if self.artifacts.is_empty() {
            return Err(err("packet must carry at least one kit artifact"));
        }
        let mut artifact_ids = BTreeSet::new();
        for artifact in &self.artifacts {
            validate_artifact(artifact)?;
            if !artifact_ids.insert(artifact.artifact_id.as_str()) {
                return Err(err(format!("duplicate artifact_id: {}", artifact.artifact_id)));
            }
        }
        validate_conformance_summary(&self.conformance_summary)?;
        validate_activation_budget(&self.activation_budget)?;
        validate_publisher_continuity(&self.publisher_continuity)?;
        validate_claim(&self.claim)?;
        validate_banner(&self.downgraded_lane_banner)?;

        for surface in &self.consumer_surfaces {
            ensure_token(
                STABLE_SDK_AUTHOR_LANE_CONSUMER_SURFACES,
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
            .any(|r| r == STABLE_SDK_AUTHOR_LANE_SCHEMA_REF)
        {
            return Err(err("packet must cite its source schema"));
        }

        // No ambient template privilege, catalog-only trust, unbounded activation
        // cost, or nonconformant stable claim may ride a published stable lane row.
        if self.allows_ambient_template_privilege
            || self.allows_catalog_only_trust
            || self.allows_unbounded_activation_cost
            || self.allows_nonconformant_stable_claim
        {
            return Err(err(
                "a stable author-lane packet must not allow ambient template privilege, catalog-only trust, unbounded activation cost, or a nonconformant stable claim",
            ));
        }

        // The conformance summary is re-derived from the artifacts so a stored
        // packet cannot hide a nonconformant or missing artifact kind.
        let derived_summary = summarize_conformance(&self.artifacts);
        if derived_summary != self.conformance_summary {
            return Err(err(
                "stored conformance summary does not match the artifact-derived truth",
            ));
        }

        // Stable-claim binding.
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            if !self.identity.sdk_version_current() {
                return Err(err(
                    "stable effective tier must pin the published SDK version",
                ));
            }
            if self.claim.claim_basis_class != "conformance_backed" {
                return Err(err(
                    "stable effective tier must be conformance-backed, not catalog-asserted",
                ));
            }
            if self.identity.publisher_trust_tier_class == "quarantined" {
                return Err(err(
                    "stable effective tier must not carry a quarantined trust tier",
                ));
            }
            if !self.identity.lifecycle_installable() {
                return Err(err("stable effective tier must stay on an installable lifecycle"));
            }
            if !self.conformance_summary.all_required_kinds_present {
                return Err(err(
                    "stable effective tier must ship every required artifact kind",
                ));
            }
            if !self.conformance_summary.all_conformant {
                return Err(err(
                    "stable effective tier must keep every artifact conformant",
                ));
            }
            if self.artifacts.iter().any(|a| !a.version_current()) {
                return Err(err(
                    "stable effective tier must pin every artifact to the published schema version",
                ));
            }
            if self.artifacts.iter().any(|a| a.widens_template_authority()) {
                return Err(err(
                    "stable effective tier must not scaffold ambient template privilege",
                ));
            }
            if !self.activation_budget.within_budget() {
                return Err(err(
                    "stable effective tier must keep its activation cost bounded and within budget",
                ));
            }
            if !self.publisher_continuity.current() {
                return Err(err(
                    "stable effective tier must keep its publisher continuity current",
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
        let derived = derive_effective_tier(
            &self.claim.claimed_tier,
            &self.claim.claim_basis_class,
            &self.identity,
            &self.artifacts,
            &self.conformance_summary,
            &self.activation_budget,
            &self.publisher_continuity,
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
        let expected: BTreeSet<&str> =
            derived.downgrade_reasons.iter().map(String::as_str).collect();
        if stored != expected {
            return Err(err(
                "stored downgrade reasons do not match the posture-derived reasons",
            ));
        }

        // Banner truth.
        let banner_required = lane_requires_warning(
            &self.identity,
            &self.artifacts,
            &self.conformance_summary,
            &self.activation_budget,
            &self.publisher_continuity,
        );
        if self.downgraded_lane_banner.must_display != banner_required {
            return Err(err(
                "downgraded-lane banner must_display does not match the lane posture",
            ));
        }

        validate_inspection(&self.inspection, self)?;
        Ok(())
    }

    /// Returns true when no stable claim is implied from catalog-only trust.
    pub fn no_catalog_only_stable_claim(&self) -> bool {
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            return self.claim.claim_basis_class == "conformance_backed";
        }
        true
    }

    /// Returns true when identity and every artifact are fully attributed.
    pub fn attribution_complete(&self) -> bool {
        attribution_is_complete(&self.identity, &self.artifacts)
    }
}

// ---------------------------------------------------------------------------
// Projection
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and dashboard surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableSdkAuthorLaneProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Author-lane identity.
    pub lane_identity_ref: String,
    /// Claimed tier.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim is a stable author-lane claim.
    pub stable_claim: bool,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// Downgrade reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-lane banner is required.
    pub downgraded_lane_banner_required: bool,
    /// Number of kit artifacts.
    pub artifact_count: usize,
    /// Number of conformant artifacts.
    pub conformant_artifact_count: usize,
    /// True when every artifact is conformant and every required kind is present.
    pub lane_conformant: bool,
}

impl From<StableSdkAuthorLanePacket> for StableSdkAuthorLaneProjection {
    fn from(packet: StableSdkAuthorLanePacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            lane_identity_ref: packet.identity.lane_identity_ref,
            claimed_tier: packet.claim.claimed_tier,
            effective_tier: packet.claim.effective_tier,
            support_claim_class: packet.claim.support_claim_class,
            stable_claim: packet.inspection.stable_claim,
            downgraded: packet.claim.downgraded,
            downgrade_reasons: packet.claim.downgrade_reasons,
            downgraded_lane_banner_required: packet.downgraded_lane_banner.must_display,
            artifact_count: packet.inspection.artifact_count,
            conformant_artifact_count: packet.inspection.conformant_artifact_count,
            lane_conformant: packet.inspection.lane_conformant,
        }
    }
}

/// Parses and validates a materialized packet, returning the compact projection.
///
/// # Errors
///
/// Returns [`StableSdkAuthorLaneError`] when the payload fails to parse or violates
/// the stable SDK author-lane invariants.
pub fn project_stable_sdk_author_lane(
    payload: &str,
) -> Result<StableSdkAuthorLaneProjection, StableSdkAuthorLaneError> {
    let packet: StableSdkAuthorLanePacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(StableSdkAuthorLaneProjection::from(packet))
}

// ---------------------------------------------------------------------------
// Support export projection
// ---------------------------------------------------------------------------

/// Metadata-safe support/partner export row that quotes the same closed tokens as
/// the packet without leaking raw schema, sample, template, or version bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableSdkAuthorLaneSupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub export_id: String,
    /// Ref to the packet this export quotes.
    pub packet_ref: String,
    /// Author-lane identity.
    pub lane_identity_ref: String,
    /// Source package.
    pub source_package_ref: String,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// Lifecycle state.
    pub lifecycle_state_class: String,
    /// Activation-budget posture.
    pub activation_budget_class: String,
    /// Publisher-continuity state.
    pub publisher_continuity_class: String,
    /// Effective tier.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim was narrowed below Stable.
    pub downgraded: bool,
    /// Narrowing reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-lane banner is required.
    pub downgraded_lane_banner_required: bool,
    /// True when the effective tier blocks authoring against the lane (withdrawn).
    pub blocks_authoring: bool,
    /// True when a sample or template scaffolds ambient privilege.
    pub ambient_template_privilege_present: bool,
    /// Number of kit artifacts.
    pub artifact_count: usize,
    /// Number of conformant artifacts.
    pub conformant_artifact_count: usize,
    /// Number of nonconformant artifacts.
    pub nonconformant_artifact_count: usize,
    /// Number of required artifact kinds missing from the lane.
    pub missing_required_kind_count: usize,
    /// Export-safe summary suitable for support/partner consumers.
    pub export_safe_summary: String,
}

/// Projects a packet into the metadata-safe support/partner export row.
pub fn project_stable_sdk_author_lane_support_export(
    packet: &StableSdkAuthorLanePacket,
) -> StableSdkAuthorLaneSupportExport {
    let blocks_authoring = packet.claim.effective_tier == "withdrawn";
    let ambient = packet.artifacts.iter().any(|a| a.widens_template_authority());
    let export_safe_summary = format!(
        "{} Trust={} lifecycle={}. Conformance: artifacts={} conformant={} nonconformant={} missing_kinds={}. Activation={}. Continuity={}. Tier claimed={} effective={} (downgraded={}). Banner required={}. Ambient template privilege={}.",
        packet.claim.summary_label,
        packet.identity.publisher_trust_tier_class,
        packet.identity.lifecycle_state_class,
        packet.conformance_summary.artifact_count,
        packet.conformance_summary.conformant_count,
        packet.conformance_summary.nonconformant_count,
        packet.conformance_summary.missing_required_kinds.len(),
        packet.activation_budget.budget_class,
        packet.publisher_continuity.continuity_state_class,
        packet.claim.claimed_tier,
        packet.claim.effective_tier,
        packet.claim.downgraded,
        packet.downgraded_lane_banner.must_display,
        ambient,
    );

    StableSdkAuthorLaneSupportExport {
        record_kind: STABLE_SDK_AUTHOR_LANE_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_AUTHOR_LANE_SCHEMA_VERSION,
        export_id: format!("stable_sdk_author_lane_support_export:{}", packet.packet_id),
        packet_ref: packet.packet_id.clone(),
        lane_identity_ref: packet.identity.lane_identity_ref.clone(),
        source_package_ref: packet.identity.source_package_ref.clone(),
        trust_tier_class: packet.identity.publisher_trust_tier_class.clone(),
        lifecycle_state_class: packet.identity.lifecycle_state_class.clone(),
        activation_budget_class: packet.activation_budget.budget_class.clone(),
        publisher_continuity_class: packet.publisher_continuity.continuity_state_class.clone(),
        effective_tier: packet.claim.effective_tier.clone(),
        support_claim_class: packet.claim.support_claim_class.clone(),
        downgraded: packet.claim.downgraded,
        downgrade_reasons: packet.claim.downgrade_reasons.clone(),
        downgraded_lane_banner_required: packet.downgraded_lane_banner.must_display,
        blocks_authoring,
        ambient_template_privilege_present: ambient,
        artifact_count: packet.conformance_summary.artifact_count,
        conformant_artifact_count: packet.conformance_summary.conformant_count,
        nonconformant_artifact_count: packet.conformance_summary.nonconformant_count,
        missing_required_kind_count: packet.conformance_summary.missing_required_kinds.len(),
        export_safe_summary,
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for stable SDK author-lane operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableSdkAuthorLaneError {
    /// Validation failed.
    Validation(StableSdkAuthorLaneValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for StableSdkAuthorLaneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for StableSdkAuthorLaneError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for stable SDK author-lane packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableSdkAuthorLaneValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for StableSdkAuthorLaneValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StableSdkAuthorLaneValidationError {}

impl StableSdkAuthorLaneValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<serde_json::Error> for StableSdkAuthorLaneError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(StableSdkAuthorLaneValidationError {
            message: err.to_string(),
        })
    }
}

impl From<StableSdkAuthorLaneValidationError> for StableSdkAuthorLaneError {
    fn from(err: StableSdkAuthorLaneValidationError) -> Self {
        Self::Validation(err)
    }
}

// ---------------------------------------------------------------------------
// Conformance aggregation
// ---------------------------------------------------------------------------

/// Summarizes the conformance posture across the kit artifacts. The summary is
/// derived purely from the artifacts so it cannot drift from its evidence.
fn summarize_conformance(artifacts: &[SdkKitArtifact]) -> SdkConformanceSummary {
    let conformant_count = artifacts.iter().filter(|a| a.conformant()).count();
    let nonconformant_count = artifacts
        .iter()
        .filter(|a| a.conformance_state_class == "nonconformant")
        .count();
    let not_run_count = artifacts
        .iter()
        .filter(|a| a.conformance_state_class == "not_run")
        .count();
    let waived_count = artifacts
        .iter()
        .filter(|a| a.conformance_state_class == "waived")
        .count();

    let present: BTreeSet<String> = artifacts
        .iter()
        .map(|a| a.artifact_kind_class.clone())
        .collect();
    let present_kinds: Vec<String> = present.iter().cloned().collect();
    let missing_required_kinds: Vec<String> = REQUIRED_ARTIFACT_KINDS
        .iter()
        .filter(|k| !present.contains(**k))
        .map(|k| k.to_string())
        .collect();

    let all_conformant = !artifacts.is_empty() && conformant_count == artifacts.len();
    let all_required_kinds_present = missing_required_kinds.is_empty();

    SdkConformanceSummary {
        record_kind: SDK_CONFORMANCE_SUMMARY_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_AUTHOR_LANE_SCHEMA_VERSION,
        artifact_count: artifacts.len(),
        conformant_count,
        nonconformant_count,
        not_run_count,
        waived_count,
        present_kinds,
        missing_required_kinds,
        all_conformant,
        all_required_kinds_present,
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

/// Applies the lane posture to a claimed tier, narrowing automatically below
/// Stable when the conformance kit can no longer back it.
#[allow(clippy::too_many_arguments)]
fn derive_effective_tier(
    claimed_tier: &str,
    claim_basis: &str,
    identity: &SdkAuthorLaneIdentity,
    artifacts: &[SdkKitArtifact],
    summary: &SdkConformanceSummary,
    activation: &SdkActivationBudget,
    continuity: &SdkPublisherContinuity,
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

    if !identity.sdk_version_current() {
        reasons.push("sdk_version_not_published".to_string());
    }
    if claim_basis != "conformance_backed" {
        reasons.push("catalog_only_trust_not_conformance_backed".to_string());
    }
    if identity.publisher_trust_tier_class == "quarantined" {
        reasons.push("trust_tier_quarantined".to_string());
    }
    if !identity.lifecycle_installable() {
        reasons.push("lifecycle_not_installable".to_string());
    }
    if !summary.all_required_kinds_present {
        reasons.push("missing_required_artifact_kind".to_string());
    }
    if artifacts
        .iter()
        .any(|a| a.artifact_schema_version < STABLE_SDK_AUTHOR_LANE_PUBLISHED_ARTIFACT_VERSION)
    {
        reasons.push("artifact_below_published_version".to_string());
    }
    if artifacts
        .iter()
        .any(|a| a.artifact_schema_version > STABLE_SDK_AUTHOR_LANE_PUBLISHED_ARTIFACT_VERSION)
    {
        reasons.push("artifact_above_published_version".to_string());
    }
    if summary.nonconformant_count > 0 {
        reasons.push("artifact_nonconformant".to_string());
    }
    if summary.not_run_count > 0 {
        reasons.push("artifact_conformance_not_run".to_string());
    }
    if artifacts.iter().any(|a| a.widens_template_authority()) {
        reasons.push("ambient_template_privilege".to_string());
    }
    match activation.budget_class.as_str() {
        "unbounded" => reasons.push("activation_cost_unbounded".to_string()),
        "over_budget" => reasons.push("activation_cost_over_budget".to_string()),
        "not_measured" => reasons.push("activation_cost_not_measured".to_string()),
        _ => {}
    }
    match continuity.continuity_state_class.as_str() {
        "revoked" => reasons.push("publisher_continuity_revoked".to_string()),
        "missing" => reasons.push("publisher_continuity_missing".to_string()),
        "stale" => reasons.push("publisher_continuity_stale".to_string()),
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
        "stable" => "stable_author_lane_ready_claim",
        "beta" => "beta_author_lane_partial_claim",
        "preview" => "preview_author_lane_experimental_claim",
        "withdrawn" => "withdrawn_no_author_lane_claim",
        _ => "preview_author_lane_experimental_claim",
    }
    .to_string()
}

/// Returns true when identity and every artifact are fully attributed.
fn attribution_is_complete(
    identity: &SdkAuthorLaneIdentity,
    artifacts: &[SdkKitArtifact],
) -> bool {
    !identity.source_package_ref.trim().is_empty()
        && !identity.lane_identity_ref.trim().is_empty()
        && !identity.starter_pack_ref.trim().is_empty()
        && artifacts.iter().all(|a| {
            !a.artifact_id.trim().is_empty()
                && !a.published_version_ref.trim().is_empty()
                && !a.conformance_report_ref.trim().is_empty()
        })
}

/// Returns true when the lane posture requires a pre-trust warning banner.
fn lane_requires_warning(
    identity: &SdkAuthorLaneIdentity,
    artifacts: &[SdkKitArtifact],
    summary: &SdkConformanceSummary,
    activation: &SdkActivationBudget,
    continuity: &SdkPublisherContinuity,
) -> bool {
    identity.publisher_trust_tier_class == "quarantined"
        || !identity.lifecycle_installable()
        || !summary.all_required_kinds_present
        || summary.nonconformant_count > 0
        || artifacts
            .iter()
            .any(|a| a.artifact_schema_version < STABLE_SDK_AUTHOR_LANE_PUBLISHED_ARTIFACT_VERSION)
        || artifacts.iter().any(|a| a.widens_template_authority())
        || activation.unbounded()
        || matches!(continuity.continuity_state_class.as_str(), "revoked" | "missing")
}

/// Picks the most-severe banner reason for a lane that requires a warning.
fn banner_reason_for(
    identity: &SdkAuthorLaneIdentity,
    artifacts: &[SdkKitArtifact],
    summary: &SdkConformanceSummary,
    activation: &SdkActivationBudget,
    continuity: &SdkPublisherContinuity,
) -> Option<String> {
    if artifacts.iter().any(|a| a.widens_template_authority()) {
        return Some("ambient_template_privilege".to_string());
    }
    if summary.nonconformant_count > 0 {
        return Some("artifact_nonconformant".to_string());
    }
    if !summary.all_required_kinds_present {
        return Some("missing_required_artifact_kind".to_string());
    }
    if artifacts
        .iter()
        .any(|a| a.artifact_schema_version < STABLE_SDK_AUTHOR_LANE_PUBLISHED_ARTIFACT_VERSION)
    {
        return Some("artifact_below_published_version".to_string());
    }
    if activation.unbounded() {
        return Some("activation_cost_unbounded".to_string());
    }
    if continuity.continuity_state_class == "revoked" {
        return Some("publisher_continuity_revoked".to_string());
    }
    if continuity.continuity_state_class == "missing" {
        return Some("publisher_continuity_missing".to_string());
    }
    if !identity.lifecycle_installable() {
        return Some("lifecycle_not_installable".to_string());
    }
    if identity.publisher_trust_tier_class == "quarantined" {
        return Some("trust_tier_quarantined".to_string());
    }
    None
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn identity_record(input: &SdkAuthorLaneIdentityInput) -> SdkAuthorLaneIdentity {
    SdkAuthorLaneIdentity {
        record_kind: SDK_AUTHOR_LANE_IDENTITY_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_AUTHOR_LANE_SCHEMA_VERSION,
        starter_pack_ref: input.starter_pack_ref.clone(),
        lane_identity_ref: input.lane_identity_ref.clone(),
        sdk_version: input.sdk_version,
        source_package_ref: input.source_package_ref.clone(),
        publisher_trust_tier_class: input.publisher_trust_tier_class.clone(),
        lifecycle_state_class: input.lifecycle_state_class.clone(),
    }
}

fn artifact_record(input: &SdkKitArtifactInput) -> SdkKitArtifact {
    SdkKitArtifact {
        record_kind: SDK_KIT_ARTIFACT_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_AUTHOR_LANE_SCHEMA_VERSION,
        artifact_id: input.artifact_id.clone(),
        artifact_kind_class: input.artifact_kind_class.clone(),
        host_class: input.host_class.clone(),
        published_version_ref: input.published_version_ref.clone(),
        artifact_schema_version: input.artifact_schema_version,
        conformance_state_class: input.conformance_state_class.clone(),
        conformance_report_ref: input.conformance_report_ref.clone(),
        declares_bounded_permissions: input.declares_bounded_permissions,
        summary_label: input.summary_label.clone(),
    }
}

fn activation_budget_record(input: &SdkActivationBudgetInput) -> SdkActivationBudget {
    SdkActivationBudget {
        record_kind: SDK_ACTIVATION_BUDGET_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_AUTHOR_LANE_SCHEMA_VERSION,
        budget_class: input.budget_class.clone(),
        measured_cost_ref: input.measured_cost_ref.clone(),
        budget_ceiling_ref: input.budget_ceiling_ref.clone(),
        measured_sample_count: input.measured_sample_count,
        summary_label: input.summary_label.clone(),
    }
}

fn publisher_continuity_record(input: &SdkPublisherContinuityInput) -> SdkPublisherContinuity {
    SdkPublisherContinuity {
        record_kind: SDK_PUBLISHER_CONTINUITY_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_AUTHOR_LANE_SCHEMA_VERSION,
        continuity_state_class: input.continuity_state_class.clone(),
        continuity_packet_ref: input.continuity_packet_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

#[allow(clippy::too_many_arguments)]
fn claim_record(
    input: &SdkAuthorLaneQualificationClaimInput,
    identity: &SdkAuthorLaneIdentity,
    artifacts: &[SdkKitArtifact],
    summary: &SdkConformanceSummary,
    activation: &SdkActivationBudget,
    continuity: &SdkPublisherContinuity,
    attribution_complete: bool,
) -> SdkAuthorLaneQualificationClaim {
    let derived = derive_effective_tier(
        &input.claimed_tier,
        &input.claim_basis_class,
        identity,
        artifacts,
        summary,
        activation,
        continuity,
        attribution_complete,
    );
    SdkAuthorLaneQualificationClaim {
        record_kind: SDK_AUTHOR_LANE_QUALIFICATION_CLAIM_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_AUTHOR_LANE_SCHEMA_VERSION,
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
    identity: &SdkAuthorLaneIdentity,
    artifacts: &[SdkKitArtifact],
    summary: &SdkConformanceSummary,
    activation: &SdkActivationBudget,
    continuity: &SdkPublisherContinuity,
) -> DowngradedLaneBanner {
    let must_display = lane_requires_warning(identity, artifacts, summary, activation, continuity);
    let banner_reason_class = if must_display {
        banner_reason_for(identity, artifacts, summary, activation, continuity)
    } else {
        None
    };
    let summary_label = if must_display {
        format!(
            "Author lane requires review before relying on it ({}).",
            banner_reason_class.clone().unwrap_or_default()
        )
    } else {
        "Author lane stabilized: all artifacts conformant, activation bounded, continuity current."
            .to_string()
    };
    DowngradedLaneBanner {
        record_kind: DOWNGRADED_LANE_BANNER_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_AUTHOR_LANE_SCHEMA_VERSION,
        must_display,
        banner_reason_class,
        summary_label,
    }
}

#[allow(clippy::too_many_arguments)]
fn inspection_record(
    packet_id: &str,
    identity: &SdkAuthorLaneIdentity,
    artifacts: &[SdkKitArtifact],
    summary: &SdkConformanceSummary,
    activation: &SdkActivationBudget,
    continuity: &SdkPublisherContinuity,
    claim: &SdkAuthorLaneQualificationClaim,
    banner: &DowngradedLaneBanner,
    attribution_complete: bool,
) -> StableSdkAuthorLaneInspection {
    let stable_claim = STABLE_TIERS.contains(&claim.effective_tier.as_str());
    let stale_version_artifact_count =
        artifacts.iter().filter(|a| !a.version_current()).count();
    let ambient = artifacts.iter().any(|a| a.widens_template_authority());

    StableSdkAuthorLaneInspection {
        record_kind: STABLE_SDK_AUTHOR_LANE_INSPECTION_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_AUTHOR_LANE_SCHEMA_VERSION,
        packet_id_ref: packet_id.to_string(),
        effective_tier: claim.effective_tier.clone(),
        stable_claim,
        sdk_version_current: identity.sdk_version_current(),
        trust_tier_class: identity.publisher_trust_tier_class.clone(),
        lifecycle_installable: identity.lifecycle_installable(),
        lane_conformant: summary.lane_conformant(),
        activation_budget_class: activation.budget_class.clone(),
        activation_within_budget: activation.within_budget(),
        publisher_continuity_class: continuity.continuity_state_class.clone(),
        downgraded: claim.downgraded,
        downgraded_lane_banner_required: banner.must_display,
        attribution_complete,
        ambient_template_privilege_present: ambient,
        artifact_count: summary.artifact_count,
        conformant_artifact_count: summary.conformant_count,
        nonconformant_artifact_count: summary.nonconformant_count,
        stale_version_artifact_count,
        missing_required_kind_count: summary.missing_required_kinds.len(),
        summary_label: claim.summary_label.clone(),
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &StableSdkAuthorLaneInput,
) -> Result<(), StableSdkAuthorLaneValidationError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    let id = &input.identity;
    ensure_nonempty(&id.starter_pack_ref, "identity.starter_pack_ref")?;
    if !id.starter_pack_ref.starts_with("sdk_v1_starter_pack:") {
        return Err(err(
            "identity.starter_pack_ref must start with 'sdk_v1_starter_pack:'",
        ));
    }
    ensure_nonempty(&id.lane_identity_ref, "identity.lane_identity_ref")?;
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

    if input.artifacts.is_empty() {
        return Err(err("input must carry at least one kit artifact"));
    }
    let mut artifact_ids = BTreeSet::new();
    for a in &input.artifacts {
        ensure_nonempty(&a.artifact_id, "artifact.artifact_id")?;
        if !artifact_ids.insert(&a.artifact_id) {
            return Err(err(format!("duplicate artifact_id: {}", a.artifact_id)));
        }
        ensure_token(ARTIFACT_KIND_CLASSES, &a.artifact_kind_class, "artifact.artifact_kind_class")?;
        ensure_token(ARTIFACT_HOST_CLASSES, &a.host_class, "artifact.host_class")?;
        ensure_nonempty(&a.published_version_ref, "artifact.published_version_ref")?;
        ensure_token(
            CONFORMANCE_STATE_CLASSES,
            &a.conformance_state_class,
            "artifact.conformance_state_class",
        )?;
        ensure_nonempty(&a.conformance_report_ref, "artifact.conformance_report_ref")?;
        // A non-scaffold artifact (schema or conformance kit) does not scaffold a
        // permission set, so it must declare its permission set bounded.
        if !a.declares_bounded_permissions
            && !matches!(
                a.artifact_kind_class.as_str(),
                "sample_extension" | "project_template"
            )
        {
            return Err(err(
                "only a sample or template may declare an unbounded scaffold permission set",
            ));
        }
    }

    let act = &input.activation_budget;
    ensure_token(ACTIVATION_BUDGET_CLASSES, &act.budget_class, "activation_budget.budget_class")?;
    ensure_nonempty(&act.measured_cost_ref, "activation_budget.measured_cost_ref")?;
    ensure_nonempty(&act.budget_ceiling_ref, "activation_budget.budget_ceiling_ref")?;

    let cont = &input.publisher_continuity;
    ensure_token(
        PUBLISHER_CONTINUITY_CLASSES,
        &cont.continuity_state_class,
        "publisher_continuity.continuity_state_class",
    )?;
    // A current continuity must bind a continuity packet ref.
    if cont.continuity_state_class == "current"
        && cont
            .continuity_packet_ref
            .as_ref()
            .map(|r| r.trim().is_empty())
            .unwrap_or(true)
    {
        return Err(err(
            "a current publisher continuity must bind a continuity_packet_ref",
        ));
    }

    let claim = &input.claim;
    ensure_token(STABILITY_TIERS, &claim.claimed_tier, "claim.claimed_tier")?;
    ensure_token(CLAIM_BASIS_CLASSES, &claim.claim_basis_class, "claim.claim_basis_class")?;

    for surface in &input.consumer_surfaces {
        ensure_token(STABLE_SDK_AUTHOR_LANE_CONSUMER_SURFACES, surface, "consumer_surface")?;
    }
    if input.consumer_surfaces.is_empty() {
        return Err(err("input must bind at least one consumer surface"));
    }

    Ok(())
}

fn validate_identity(
    identity: &SdkAuthorLaneIdentity,
) -> Result<(), StableSdkAuthorLaneValidationError> {
    ensure_eq(
        identity.record_kind.as_str(),
        SDK_AUTHOR_LANE_IDENTITY_RECORD_KIND,
        "identity record_kind",
    )?;
    ensure_eq_u32(
        identity.schema_version,
        STABLE_SDK_AUTHOR_LANE_SCHEMA_VERSION,
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

fn validate_artifact(
    artifact: &SdkKitArtifact,
) -> Result<(), StableSdkAuthorLaneValidationError> {
    ensure_eq(
        artifact.record_kind.as_str(),
        SDK_KIT_ARTIFACT_RECORD_KIND,
        "artifact record_kind",
    )?;
    ensure_eq_u32(
        artifact.schema_version,
        STABLE_SDK_AUTHOR_LANE_SCHEMA_VERSION,
        "artifact schema_version",
    )?;
    ensure_nonempty(&artifact.artifact_id, "artifact artifact_id")?;
    ensure_token(
        ARTIFACT_KIND_CLASSES,
        &artifact.artifact_kind_class,
        "artifact artifact_kind_class",
    )?;
    ensure_token(ARTIFACT_HOST_CLASSES, &artifact.host_class, "artifact host_class")?;
    ensure_token(
        CONFORMANCE_STATE_CLASSES,
        &artifact.conformance_state_class,
        "artifact conformance_state_class",
    )?;
    ensure_nonempty(&artifact.published_version_ref, "artifact published_version_ref")?;
    ensure_nonempty(&artifact.conformance_report_ref, "artifact conformance_report_ref")?;
    Ok(())
}

fn validate_conformance_summary(
    summary: &SdkConformanceSummary,
) -> Result<(), StableSdkAuthorLaneValidationError> {
    ensure_eq(
        summary.record_kind.as_str(),
        SDK_CONFORMANCE_SUMMARY_RECORD_KIND,
        "conformance_summary record_kind",
    )?;
    for kind in &summary.present_kinds {
        ensure_token(ARTIFACT_KIND_CLASSES, kind, "conformance_summary.present_kinds")?;
    }
    for kind in &summary.missing_required_kinds {
        ensure_token(ARTIFACT_KIND_CLASSES, kind, "conformance_summary.missing_required_kinds")?;
    }
    Ok(())
}

fn validate_activation_budget(
    activation: &SdkActivationBudget,
) -> Result<(), StableSdkAuthorLaneValidationError> {
    ensure_eq(
        activation.record_kind.as_str(),
        SDK_ACTIVATION_BUDGET_RECORD_KIND,
        "activation_budget record_kind",
    )?;
    ensure_token(
        ACTIVATION_BUDGET_CLASSES,
        &activation.budget_class,
        "activation_budget budget_class",
    )?;
    ensure_nonempty(&activation.measured_cost_ref, "activation_budget measured_cost_ref")?;
    ensure_nonempty(&activation.budget_ceiling_ref, "activation_budget budget_ceiling_ref")?;
    Ok(())
}

fn validate_publisher_continuity(
    continuity: &SdkPublisherContinuity,
) -> Result<(), StableSdkAuthorLaneValidationError> {
    ensure_eq(
        continuity.record_kind.as_str(),
        SDK_PUBLISHER_CONTINUITY_RECORD_KIND,
        "publisher_continuity record_kind",
    )?;
    ensure_token(
        PUBLISHER_CONTINUITY_CLASSES,
        &continuity.continuity_state_class,
        "publisher_continuity continuity_state_class",
    )?;
    Ok(())
}

fn validate_claim(
    claim: &SdkAuthorLaneQualificationClaim,
) -> Result<(), StableSdkAuthorLaneValidationError> {
    ensure_eq(
        claim.record_kind.as_str(),
        SDK_AUTHOR_LANE_QUALIFICATION_CLAIM_RECORD_KIND,
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
        ensure_token(SDK_AUTHOR_LANE_DOWNGRADE_REASONS, reason, "claim downgrade_reason")?;
    }
    Ok(())
}

fn validate_banner(
    banner: &DowngradedLaneBanner,
) -> Result<(), StableSdkAuthorLaneValidationError> {
    ensure_eq(
        banner.record_kind.as_str(),
        DOWNGRADED_LANE_BANNER_RECORD_KIND,
        "banner record_kind",
    )?;
    if let Some(reason) = &banner.banner_reason_class {
        ensure_token(SDK_AUTHOR_LANE_DOWNGRADE_REASONS, reason, "banner banner_reason_class")?;
        if !banner.must_display {
            return Err(err("banner_reason_class is set but must_display is false"));
        }
    } else if banner.must_display {
        return Err(err("must_display is true but no banner_reason_class is set"));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &StableSdkAuthorLaneInspection,
    packet: &StableSdkAuthorLanePacket,
) -> Result<(), StableSdkAuthorLaneValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        STABLE_SDK_AUTHOR_LANE_INSPECTION_RECORD_KIND,
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
    if inspection.downgraded_lane_banner_required != packet.downgraded_lane_banner.must_display {
        return Err(err("inspection downgraded_lane_banner_required is inconsistent"));
    }
    if inspection.attribution_complete != packet.attribution_complete() {
        return Err(err("inspection attribution_complete is inconsistent"));
    }
    if inspection.ambient_template_privilege_present
        != packet.artifacts.iter().any(|a| a.widens_template_authority())
    {
        return Err(err("inspection ambient_template_privilege_present is inconsistent"));
    }
    if inspection.artifact_count != packet.artifacts.len() {
        return Err(err("inspection artifact_count is inconsistent"));
    }
    if inspection.conformant_artifact_count != packet.conformance_summary.conformant_count {
        return Err(err("inspection conformant_artifact_count is inconsistent"));
    }
    if inspection.missing_required_kind_count != packet.conformance_summary.missing_required_kinds.len()
    {
        return Err(err("inspection missing_required_kind_count is inconsistent"));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn err(message: impl Into<String>) -> StableSdkAuthorLaneValidationError {
    StableSdkAuthorLaneValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(left: T, right: T, field: &str) -> Result<(), StableSdkAuthorLaneValidationError>
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
) -> Result<(), StableSdkAuthorLaneValidationError> {
    if left != right {
        return Err(err(format!("{field} mismatch: expected {right}, got {left}")));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), StableSdkAuthorLaneValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), StableSdkAuthorLaneValidationError> {
    if !tokens.contains(&value) {
        return Err(err(format!("{field} must be one of {tokens:?}, got {value}")));
    }
    Ok(())
}
