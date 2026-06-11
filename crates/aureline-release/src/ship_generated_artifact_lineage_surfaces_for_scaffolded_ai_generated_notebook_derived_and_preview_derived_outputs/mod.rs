//! Typed generated-artifact lineage register, provenance disclosure, and downgrade automation.
//!
//! This module freezes the canonical control surface for the lineage of
//! generated outputs. Where the depth-claim manifest speaks for the *depth
//! claim* each feature family publishes and the train scorecard register speaks
//! for the *per-feature scorecard*, this register speaks for the *lineage
//! surface* every generated-artifact family exposes — scaffolded, AI-generated,
//! notebook-derived, and preview-derived outputs. Each [`LineageSurface`] binds
//! one generated-artifact family to:
//!
//! - the stable claim it backs ([`LineageSurface::claim_ref`],
//!   [`LineageSurface::claim_label`]),
//! - a lineage scorecard ([`LineageSurface::scorecard`]) of one [`LineageCell`]
//!   per [`LineageDimension`], so provenance, inputs, generator identity,
//!   transform, reproducibility, and disclosure are each an explicit, inspectable
//!   grade,
//! - the artifact provenance it discloses ([`LineageSurface::lineage`]): the
//!   generator, the provider/host, the [`TrustTier`], the inputs, and whether the
//!   artifact is labeled to the user as generated,
//! - an owner manifest ([`LineageSurface::owner_signoff`]) recording who signed
//!   the claim,
//! - an explicit rollback/downgrade automation record
//!   ([`LineageSurface::downgrade_automation`]) binding the surface to a verified
//!   rollback plan and the trigger and floor it narrows to,
//! - the overall lineage state earned ([`LineageState`]), the active narrowing
//!   reasons ([`NarrowingReason`]), and the effective label after narrowing
//!   ([`LineageSurface::published_label`]),
//! - a [`ProofPacket`] (reused from the stable claim manifest) and its freshness
//!   SLO, plus an optional waiver.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between a surface that may publish a Stable claim and one that must narrow
//! below it. The [`LineageStopRule`] set names the closed conditions that gate
//! promotion — one per [`NarrowingReason`] — and
//! [`GeneratedArtifactLineageRegister::promotion`] records the proceed/hold
//! verdict.
//!
//! The register is checked in at
//! `artifacts/release/m5/ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_generated_notebook_derived_and_preview_derived_outputs.json`
//! and embedded here, so this typed consumer and the CI gate agree on every
//! lineage surface without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw generated bodies, raw logs, signatures, or credential material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

/// Supported register schema version.
pub const GENERATED_ARTIFACT_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const GENERATED_ARTIFACT_LINEAGE_RECORD_KIND: &str =
    "ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_generated_notebook_derived_and_preview_derived_outputs";

/// Repo-relative path to the checked-in register.
pub const GENERATED_ARTIFACT_LINEAGE_PATH: &str =
    "artifacts/release/m5/ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_generated_notebook_derived_and_preview_derived_outputs.json";

/// Embedded checked-in register JSON.
pub const GENERATED_ARTIFACT_LINEAGE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_generated_notebook_derived_and_preview_derived_outputs.json"
));

/// Generated-artifact family a lineage surface governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorKind {
    /// Files emitted by project scaffolding and code generation.
    Scaffolded,
    /// AI-generated edits, completions, and composed artifacts.
    AiGenerated,
    /// Artifacts exported or derived from notebook cells and outputs.
    NotebookDerived,
    /// Artifacts published from the preview/designer surface.
    PreviewDerived,
}

impl GeneratorKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Scaffolded,
        Self::AiGenerated,
        Self::NotebookDerived,
        Self::PreviewDerived,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scaffolded => "scaffolded",
            Self::AiGenerated => "ai_generated",
            Self::NotebookDerived => "notebook_derived",
            Self::PreviewDerived => "preview_derived",
        }
    }
}

/// One dimension of the lineage scorecard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageDimension {
    /// The generator that produced the artifact is identified.
    Provenance,
    /// The inputs that fed the generation are recorded.
    Inputs,
    /// The generator's provider/host/trust identity is disclosed.
    GeneratorIdentity,
    /// The transformation chain from inputs to artifact is recorded.
    Transform,
    /// The generation is reproducible or its non-reproducibility is recorded.
    Reproducibility,
    /// The artifact is labeled to the user as generated.
    Disclosure,
}

impl LineageDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Provenance,
        Self::Inputs,
        Self::GeneratorIdentity,
        Self::Transform,
        Self::Reproducibility,
        Self::Disclosure,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Provenance => "provenance",
            Self::Inputs => "inputs",
            Self::GeneratorIdentity => "generator_identity",
            Self::Transform => "transform",
            Self::Reproducibility => "reproducibility",
            Self::Disclosure => "disclosure",
        }
    }

    /// The narrowing reason a non-passing, non-waived cell must name, given the
    /// cell's [`DimensionGrade`].
    pub const fn reason_for_grade(self, grade: DimensionGrade) -> Option<NarrowingReason> {
        match grade {
            DimensionGrade::Missing => Some(NarrowingReason::LineageDimensionMissing),
            DimensionGrade::Fail | DimensionGrade::Partial => {
                Some(NarrowingReason::LineageDimensionFailed)
            }
            DimensionGrade::Pass | DimensionGrade::Waived => None,
        }
    }
}

/// The grade earned on one lineage dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DimensionGrade {
    /// The dimension fully passes.
    Pass,
    /// The dimension partially passes; remediation is required.
    Partial,
    /// The dimension fails.
    Fail,
    /// Held provisionally under an active, unexpired waiver.
    Waived,
    /// The dimension has no lineage evidence at all.
    Missing,
}

impl DimensionGrade {
    /// Every grade, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Pass,
        Self::Partial,
        Self::Fail,
        Self::Waived,
        Self::Missing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Partial => "partial",
            Self::Fail => "fail",
            Self::Waived => "waived",
            Self::Missing => "missing",
        }
    }

    /// Whether a cell in this grade lets the surface hold its claim.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Pass | Self::Waived)
    }
}

/// Trust tier of the generator/provider behind a generated artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustTier {
    /// First-party Aureline generator.
    FirstParty,
    /// A verified third-party provider/host.
    VerifiedThirdParty,
    /// A community-sourced generator or template.
    Community,
    /// An unverified or untrusted generator.
    Untrusted,
}

impl TrustTier {
    /// Every tier, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FirstParty,
        Self::VerifiedThirdParty,
        Self::Community,
        Self::Untrusted,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::VerifiedThirdParty => "verified_third_party",
            Self::Community => "community",
            Self::Untrusted => "untrusted",
        }
    }
}

/// Overall lineage state a surface earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageState {
    /// Every dimension passes, the artifact is disclosed as generated, the owner
    /// manifest is signed, and rollback/downgrade automation is defined and
    /// verified.
    Traced,
    /// One or more lineage dimensions failed, are partial, or are missing.
    LineageRegressed,
    /// The proof packet has gone stale.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers a
    /// recorded gap.
    OnWaiver,
    /// Rollback/downgrade automation is undefined or its rollback plan is
    /// unverified.
    RollbackUndefined,
    /// The owner manifest is unsigned.
    OwnerUnsigned,
}

impl LineageState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Traced,
        Self::LineageRegressed,
        Self::Stale,
        Self::OnWaiver,
        Self::RollbackUndefined,
        Self::OwnerUnsigned,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Traced => "traced",
            Self::LineageRegressed => "lineage_regressed",
            Self::Stale => "stale",
            Self::OnWaiver => "on_waiver",
            Self::RollbackUndefined => "rollback_undefined",
            Self::OwnerUnsigned => "owner_unsigned",
        }
    }

    /// Whether the state lets a surface carry the claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Traced | Self::OnWaiver)
    }

    /// Whether the state forces the surface below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a lineage surface claim narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// A lineage dimension failed or is only partial.
    LineageDimensionFailed,
    /// A lineage dimension is missing.
    LineageDimensionMissing,
    /// The proof packet is missing.
    ProofPacketMissing,
    /// The proof packet is stale.
    ProofPacketStale,
    /// The owner manifest is unsigned.
    OwnerManifestUnsigned,
    /// The rollback plan is unverified.
    RollbackPlanUnverified,
    /// The downgrade automation is undefined.
    DowngradeAutomationUndefined,
    /// A waiver the surface relied on has expired.
    WaiverExpired,
}

impl NarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::LineageDimensionFailed,
        Self::LineageDimensionMissing,
        Self::ProofPacketMissing,
        Self::ProofPacketStale,
        Self::OwnerManifestUnsigned,
        Self::RollbackPlanUnverified,
        Self::DowngradeAutomationUndefined,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LineageDimensionFailed => "lineage_dimension_failed",
            Self::LineageDimensionMissing => "lineage_dimension_missing",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::OwnerManifestUnsigned => "owner_manifest_unsigned",
            Self::RollbackPlanUnverified => "rollback_plan_unverified",
            Self::DowngradeAutomationUndefined => "downgrade_automation_undefined",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Whether this reason marks a rollback/downgrade automation gap.
    pub const fn is_rollback_gap(self) -> bool {
        matches!(
            self,
            Self::RollbackPlanUnverified | Self::DowngradeAutomationUndefined
        )
    }

    /// Whether this reason marks a lineage-dimension gap.
    pub const fn is_dimension_gap(self) -> bool {
        matches!(
            self,
            Self::LineageDimensionFailed | Self::LineageDimensionMissing
        )
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopAction {
    /// Hold promotion until the condition clears.
    HoldPromotion,
    /// Narrow the claim below the cutline.
    NarrowLabel,
    /// Remediate the failing or missing lineage dimension.
    RemediateLineage,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Obtain the required owner-manifest sign-off.
    RequestOwnerSignoff,
    /// Verify the rollback plan.
    VerifyRollbackPlan,
    /// Define the downgrade automation.
    DefineDowngradeAutomation,
    /// Renew the expired waiver.
    RenewWaiver,
}

impl StopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::HoldPromotion,
        Self::NarrowLabel,
        Self::RemediateLineage,
        Self::RefreshProofPacket,
        Self::RequestOwnerSignoff,
        Self::VerifyRollbackPlan,
        Self::DefineDowngradeAutomation,
        Self::RenewWaiver,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowLabel => "narrow_label",
            Self::RemediateLineage => "remediate_lineage",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::VerifyRollbackPlan => "verify_rollback_plan",
            Self::DefineDowngradeAutomation => "define_downgrade_automation",
            Self::RenewWaiver => "renew_waiver",
        }
    }
}

/// What triggers a surface's automated rollback/downgrade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeTrigger {
    /// Fires when the proof packet goes stale.
    ProofStale,
    /// Fires when the lineage scorecard regresses.
    LineageRegressed,
    /// Fires when owner sign-off is revoked.
    OwnerRevoked,
    /// Operator-driven manual downgrade.
    Manual,
}

impl DowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProofStale,
        Self::LineageRegressed,
        Self::OwnerRevoked,
        Self::Manual,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::LineageRegressed => "lineage_regressed",
            Self::OwnerRevoked => "owner_revoked",
            Self::Manual => "manual",
        }
    }
}

/// The defined/verified state of a surface's rollback/downgrade automation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationState {
    /// The automation is defined and its rollback plan is verified.
    Defined,
    /// The automation is defined but its rollback plan is unverified.
    Unverified,
    /// The automation is undefined.
    Undefined,
}

impl AutomationState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Defined, Self::Unverified, Self::Undefined];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Defined => "defined",
            Self::Unverified => "unverified",
            Self::Undefined => "undefined",
        }
    }

    /// Whether the automation is defined and verified, letting a surface hold a
    /// Stable claim.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Defined)
    }
}

/// One cell of the lineage scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LineageCell {
    /// The lineage dimension this cell speaks for.
    pub dimension: LineageDimension,
    /// The grade earned for the dimension.
    pub grade: DimensionGrade,
    /// Ref to the dimension's evidence. Empty only on a missing cell.
    pub evidence_ref: String,
}

/// The disclosed provenance of a generated artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LineageProvenance {
    /// Stable ref to the generator that produced the artifact.
    pub generator_ref: String,
    /// Stable ref to the provider/host behind the generator.
    pub provider_ref: String,
    /// Trust tier of the generator/provider.
    pub trust_tier: TrustTier,
    /// Refs to the inputs that fed the generation.
    #[serde(default)]
    pub input_refs: Vec<String>,
    /// Whether the artifact is labeled to the user as generated.
    pub generated_labeled: bool,
}

/// A surface's rollback/downgrade automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DowngradeAutomation {
    /// Stable ref to the automation definition.
    pub automation_ref: String,
    /// Ref to the rollback plan the automation drives.
    pub rollback_plan_ref: String,
    /// What triggers the automated downgrade.
    pub trigger: DowngradeTrigger,
    /// The lifecycle label the automation narrows the surface to.
    pub target_floor: StableClaimLevel,
    /// The defined/verified state of the automation.
    pub state: AutomationState,
    /// Whether the rollback plan has been verified end-to-end.
    pub rollback_verified: bool,
}

/// One lineage stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LineageStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched surface fires this rule.
    pub trigger_reason: NarrowingReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: StopAction,
    /// Whether firing this rule blocks promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One generated-artifact lineage surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LineageSurface {
    /// Stable surface id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The generated-artifact family this surface governs.
    pub generator_kind: GeneratorKind,
    /// The surface ref this entry speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the surface.
    pub surface_summary: String,
    /// Whether the surface is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose claim this surface backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the claim publishes.
    pub claim_label: StableClaimLevel,
    /// Overall lineage state earned for the surface.
    pub lineage_state: LineageState,
    /// The lineage scorecard: one cell per [`LineageDimension`].
    pub scorecard: Vec<LineageCell>,
    /// The disclosed provenance of the generated artifact.
    pub lineage: LineageProvenance,
    /// The rollback/downgrade automation backing the surface.
    pub downgrade_automation: DowngradeAutomation,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner manifest sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the surface below its claim label.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// The lifecycle label the surface effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this surface's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the surface carries this posture.
    pub rationale: String,
}

impl LineageSurface {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the surface's state lets it carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.lineage_state.holds_label()
    }

    /// True when a narrowing reason is active on the surface.
    pub fn has_active_reason(&self, reason: NarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }

    /// Returns the cell registered for `dimension`, if any.
    pub fn cell(&self, dimension: LineageDimension) -> Option<&LineageCell> {
        self.scorecard
            .iter()
            .find(|cell| cell.dimension == dimension)
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneratedArtifactLineageSummary {
    /// Total number of lineage surfaces.
    pub total_entries: usize,
    /// Distinct claims covered.
    pub total_claims: usize,
    /// Surfaces publishing a label at or above the cutline.
    pub entries_traced: usize,
    /// Surfaces narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Surfaces holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Surfaces carrying a lineage-dimension gap (failed or missing dimension).
    pub entries_with_dimension_gap: usize,
    /// Surfaces carrying an owner-manifest-unsigned reason.
    pub entries_with_owner_gap: usize,
    /// Surfaces carrying a rollback/downgrade automation gap.
    pub entries_with_rollback_gap: usize,
    /// Surfaces whose artifact is not labeled as generated.
    pub entries_unlabeled: usize,
    /// Total release-blocking surfaces.
    pub release_blocking_total: usize,
    /// Release-blocking surfaces publishing a label at or above the cutline.
    pub release_blocking_traced: usize,
    /// Release-blocking surfaces narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Scaffolded surfaces.
    pub scaffolded_entries: usize,
    /// AI-generated surfaces.
    pub ai_generated_entries: usize,
    /// Notebook-derived surfaces.
    pub notebook_derived_entries: usize,
    /// Preview-derived surfaces.
    pub preview_derived_entries: usize,
    /// First-party-trust surfaces.
    pub first_party_entries: usize,
    /// Verified-third-party-trust surfaces.
    pub verified_third_party_entries: usize,
    /// Community-trust surfaces.
    pub community_entries: usize,
    /// Untrusted surfaces.
    pub untrusted_entries: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active narrowing reasons across all surfaces.
    pub total_active_narrowing_reasons: usize,
    /// Total lineage cells across all surfaces.
    pub total_lineage_cells: usize,
    /// Cells graded `pass`.
    pub cells_pass: usize,
    /// Cells graded `partial`.
    pub cells_partial: usize,
    /// Cells graded `fail`.
    pub cells_fail: usize,
    /// Cells graded `waived`.
    pub cells_waived: usize,
    /// Cells graded `missing`.
    pub cells_missing: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageExportRow {
    /// Stable surface id.
    pub entry_id: String,
    /// The generated-artifact family this surface governs.
    pub generator_kind: GeneratorKind,
    /// The surface ref this entry speaks about.
    pub surface_ref: String,
    /// Whether the surface is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose claim this surface backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the surface publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Overall lineage state earned.
    pub lineage_state: LineageState,
    /// Trust tier of the generator/provider.
    pub trust_tier: TrustTier,
    /// Whether the artifact is labeled to the user as generated.
    pub generated_labeled: bool,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Rollback/downgrade automation state.
    pub automation_state: AutomationState,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageExportProjection {
    /// Register identifier.
    pub manifest_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<LineageExportRow>,
}

/// The typed generated-artifact lineage register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneratedArtifactLineageRegister {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register identifier.
    pub manifest_id: String,
    /// Lifecycle status of this register artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this register ingests.
    pub claim_manifest_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed generator-kind vocabulary.
    pub generator_kinds: Vec<GeneratorKind>,
    /// Closed lineage-dimension vocabulary.
    pub lineage_dimensions: Vec<LineageDimension>,
    /// Closed dimension-grade vocabulary.
    pub dimension_grades: Vec<DimensionGrade>,
    /// Closed lineage-state vocabulary.
    pub lineage_states: Vec<LineageState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<StopAction>,
    /// Closed downgrade-trigger vocabulary.
    pub downgrade_triggers: Vec<DowngradeTrigger>,
    /// Closed automation-state vocabulary.
    pub automation_states: Vec<AutomationState>,
    /// Closed trust-tier vocabulary.
    pub trust_tiers: Vec<TrustTier>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this register must cover.
    pub release_blocking_surface_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<LineageStopRule>,
    /// Lineage surfaces.
    pub rows: Vec<LineageSurface>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: GeneratedArtifactLineageSummary,
}

impl GeneratedArtifactLineageRegister {
    /// Returns the surface registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&LineageSurface> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the surfaces publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&LineageSurface> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the surfaces narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&LineageSurface> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking surfaces.
    pub fn release_blocking_rows(&self) -> Vec<&LineageSurface> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the surfaces for one generator kind.
    pub fn rows_for_kind(&self, kind: GeneratorKind) -> Vec<&LineageSurface> {
        self.rows
            .iter()
            .filter(|row| row.generator_kind == kind)
            .collect()
    }

    /// Distinct claims (by claim ref) the register covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched surface carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &LineageStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the surfaces and stop rules.
    pub fn computed_promotion_decision(&self) -> PromotionDecision {
        if self
            .stop_rules
            .iter()
            .any(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Stop-rule ids that block promotion and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Surface ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only surfaces whose claim is at or above the cutline count: a surface
    /// whose claim is already canonically narrowed is not a *promotion* blocker,
    /// it merely inherits the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<NarrowingReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row.claim_holds_stable()
                && row
                    .active_narrowing_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the surfaces and stop rules.
    pub fn computed_summary(&self) -> GeneratedArtifactLineageSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: GeneratorKind| self.rows_for_kind(kind).len();
        let trust = |tier: TrustTier| {
            self.rows
                .iter()
                .filter(|row| row.lineage.trust_tier == tier)
                .count()
        };
        let with_predicate = |pred: fn(NarrowingReason) -> bool| {
            self.rows
                .iter()
                .filter(|row| row.active_narrowing_reasons.iter().any(|r| pred(*r)))
                .count()
        };
        let with_reason = |reason: NarrowingReason| {
            self.rows
                .iter()
                .filter(|row| row.has_active_reason(reason))
                .count()
        };
        let cell_grade = |grade: DimensionGrade| {
            self.rows
                .iter()
                .flat_map(|row| row.scorecard.iter())
                .filter(|cell| cell.grade == grade)
                .count()
        };
        let release_blocking: Vec<&LineageSurface> = self.release_blocking_rows();
        GeneratedArtifactLineageSummary {
            total_entries: self.rows.len(),
            total_claims: self.claims().len(),
            entries_traced: self
                .rows
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            entries_narrowed: self
                .rows
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            entries_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.lineage_state == LineageState::OnWaiver)
                .count(),
            entries_with_dimension_gap: with_predicate(NarrowingReason::is_dimension_gap),
            entries_with_owner_gap: with_reason(NarrowingReason::OwnerManifestUnsigned),
            entries_with_rollback_gap: with_predicate(NarrowingReason::is_rollback_gap),
            entries_unlabeled: self
                .rows
                .iter()
                .filter(|row| !row.lineage.generated_labeled)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_traced: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            scaffolded_entries: kind(GeneratorKind::Scaffolded),
            ai_generated_entries: kind(GeneratorKind::AiGenerated),
            notebook_derived_entries: kind(GeneratorKind::NotebookDerived),
            preview_derived_entries: kind(GeneratorKind::PreviewDerived),
            first_party_entries: trust(TrustTier::FirstParty),
            verified_third_party_entries: trust(TrustTier::VerifiedThirdParty),
            community_entries: trust(TrustTier::Community),
            untrusted_entries: trust(TrustTier::Untrusted),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_narrowing_reasons: self
                .rows
                .iter()
                .map(|row| row.active_narrowing_reasons.len())
                .sum(),
            total_lineage_cells: self.rows.iter().map(|row| row.scorecard.len()).sum(),
            cells_pass: cell_grade(DimensionGrade::Pass),
            cells_partial: cell_grade(DimensionGrade::Partial),
            cells_fail: cell_grade(DimensionGrade::Fail),
            cells_waived: cell_grade(DimensionGrade::Waived),
            cells_missing: cell_grade(DimensionGrade::Missing),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection that downstream surfaces
    /// render instead of cloning status text.
    pub fn support_export_projection(&self) -> LineageExportProjection {
        LineageExportProjection {
            manifest_id: self.manifest_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| LineageExportRow {
                    entry_id: row.entry_id.clone(),
                    generator_kind: row.generator_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    lineage_state: row.lineage_state,
                    trust_tier: row.lineage.trust_tier,
                    generated_labeled: row.lineage.generated_labeled,
                    slo_state: row.proof_packet.slo_state,
                    automation_state: row.downgrade_automation.state,
                    active_narrowing_reasons: row.active_narrowing_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<LineageRegisterViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(LineageRegisterViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(LineageRegisterViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(LineageRegisterViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<LineageRegisterViolation>) {
        if self.schema_version != GENERATED_ARTIFACT_LINEAGE_SCHEMA_VERSION {
            violations.push(LineageRegisterViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != GENERATED_ARTIFACT_LINEAGE_RECORD_KIND {
            violations.push(LineageRegisterViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("manifest_id", &self.manifest_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(LineageRegisterViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(LineageRegisterViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.generator_kinds != GeneratorKind::ALL.to_vec() {
            violations.push(LineageRegisterViolation::ClosedVocabularyMismatch {
                field: "generator_kinds",
            });
        }
        if self.lineage_dimensions != LineageDimension::ALL.to_vec() {
            violations.push(LineageRegisterViolation::ClosedVocabularyMismatch {
                field: "lineage_dimensions",
            });
        }
        if self.dimension_grades != DimensionGrade::ALL.to_vec() {
            violations.push(LineageRegisterViolation::ClosedVocabularyMismatch {
                field: "dimension_grades",
            });
        }
        if self.lineage_states != LineageState::ALL.to_vec() {
            violations.push(LineageRegisterViolation::ClosedVocabularyMismatch {
                field: "lineage_states",
            });
        }
        if self.narrowing_reasons != NarrowingReason::ALL.to_vec() {
            violations.push(LineageRegisterViolation::ClosedVocabularyMismatch {
                field: "narrowing_reasons",
            });
        }
        if self.stop_rule_actions != StopAction::ALL.to_vec() {
            violations.push(LineageRegisterViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }
        if self.downgrade_triggers != DowngradeTrigger::ALL.to_vec() {
            violations.push(LineageRegisterViolation::ClosedVocabularyMismatch {
                field: "downgrade_triggers",
            });
        }
        if self.automation_states != AutomationState::ALL.to_vec() {
            violations.push(LineageRegisterViolation::ClosedVocabularyMismatch {
                field: "automation_states",
            });
        }
        if self.trust_tiers != TrustTier::ALL.to_vec() {
            violations.push(LineageRegisterViolation::ClosedVocabularyMismatch {
                field: "trust_tiers",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(LineageRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(LineageRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(LineageRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(LineageRegisterViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<LineageRegisterViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(LineageRegisterViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(LineageRegisterViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(LineageRegisterViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(LineageRegisterViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in NarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(LineageRegisterViolation::NarrowingReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(&self, row: &LineageSurface, violations: &mut Vec<LineageRegisterViolation>) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("surface_ref", &row.surface_ref),
            ("surface_summary", &row.surface_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            ("proof_packet.packet_id", &row.proof_packet.packet_id),
            ("proof_packet.packet_ref", &row.proof_packet.packet_ref),
            (
                "proof_packet.proof_index_ref",
                &row.proof_packet.proof_index_ref,
            ),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &row.proof_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
            ("lineage.generator_ref", &row.lineage.generator_ref),
            ("lineage.provider_ref", &row.lineage.provider_ref),
            (
                "downgrade_automation.automation_ref",
                &row.downgrade_automation.automation_ref,
            ),
            (
                "downgrade_automation.rollback_plan_ref",
                &row.downgrade_automation.rollback_plan_ref,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(LineageRegisterViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_scorecard(row, violations);
        self.validate_automation(row, violations);

        // The ceiling: no surface may carry a label wider than the claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(LineageRegisterViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be positive and the warn window may not
        // exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(LineageRegisterViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(LineageRegisterViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A claim whose canonical label is below the cutline forces the surface
        // to inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(LineageRegisterViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(LineageRegisterViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.lineage_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed surface publishes exactly the claim's canonical label,
            // carries no active reason, rides a captured within-SLO packet, is
            // labeled as generated, is owner-signed, and rides defined-and-verified
            // downgrade automation.
            if row.published_label != row.claim_label {
                violations.push(LineageRegisterViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_narrowing_reasons.is_empty() {
                violations.push(LineageRegisterViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.lineage.generated_labeled {
                violations.push(LineageRegisterViolation::HeldWithoutDisclosure {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(LineageRegisterViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(LineageRegisterViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(LineageRegisterViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.downgrade_automation.state.holds()
                || !row.downgrade_automation.rollback_verified
            {
                violations.push(LineageRegisterViolation::HeldWithoutDowngradeAutomation {
                    entry_id: row.entry_id.clone(),
                    state: row.downgrade_automation.state,
                });
            }
        } else {
            // A narrowing state must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(LineageRegisterViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.lineage_state,
                    published: row.published_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(LineageRegisterViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.lineage_state,
                });
            }
            // A narrowing surface whose packet is breached or missing must name
            // the matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(NarrowingReason::ProofPacketStale)
            {
                violations.push(LineageRegisterViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(NarrowingReason::ProofPacketMissing)
            {
                violations.push(LineageRegisterViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_scorecard(
        &self,
        row: &LineageSurface,
        violations: &mut Vec<LineageRegisterViolation>,
    ) {
        let mut seen: BTreeSet<LineageDimension> = BTreeSet::new();
        for cell in &row.scorecard {
            if !seen.insert(cell.dimension) {
                violations.push(LineageRegisterViolation::DuplicateDimension {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A missing cell carries no evidence ref; every other grade must.
            if cell.grade != DimensionGrade::Missing && cell.evidence_ref.trim().is_empty() {
                violations.push(LineageRegisterViolation::CellEvidenceMissing {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A waived cell only holds under an unexpired waiver.
            if cell.grade == DimensionGrade::Waived && row.waiver.is_none() {
                violations.push(LineageRegisterViolation::WaivedCellWithoutWaiver {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A non-passing, non-waived cell must name its narrowing reason.
            if !cell.grade.holds() {
                if let Some(reason) = cell.dimension.reason_for_grade(cell.grade) {
                    if !row.has_active_reason(reason) {
                        violations.push(LineageRegisterViolation::CellReasonNotActive {
                            entry_id: row.entry_id.clone(),
                            dimension: cell.dimension,
                            reason,
                        });
                    }
                }
            }
        }
        // The scorecard must carry exactly one cell per dimension.
        for dimension in LineageDimension::ALL {
            if !seen.contains(&dimension) {
                violations.push(LineageRegisterViolation::LineageIncompleteCoverage {
                    entry_id: row.entry_id.clone(),
                    dimension,
                });
            }
        }
    }

    fn validate_automation(
        &self,
        row: &LineageSurface,
        violations: &mut Vec<LineageRegisterViolation>,
    ) {
        let automation = &row.downgrade_automation;
        // A downgrade narrows the claim, so its floor must be below the cutline.
        if automation.target_floor.is_at_or_above_cutline() {
            violations.push(LineageRegisterViolation::DowngradeFloorNotBelowCutline {
                entry_id: row.entry_id.clone(),
                floor: automation.target_floor,
            });
        }
        // An undefined automation must name the undefined reason.
        if automation.state == AutomationState::Undefined
            && !row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
        {
            violations.push(LineageRegisterViolation::AutomationStateWithoutReason {
                entry_id: row.entry_id.clone(),
                state: automation.state,
            });
        }
        // An unverified rollback plan must name a rollback or downgrade reason.
        if !automation.rollback_verified
            && !row.has_active_reason(NarrowingReason::RollbackPlanUnverified)
            && !row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
        {
            violations.push(LineageRegisterViolation::RollbackUnverifiedWithoutReason {
                entry_id: row.entry_id.clone(),
            });
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &LineageSurface,
        violations: &mut Vec<LineageRegisterViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<LineageRegisterViolation>,
                               expected: NarrowingReason| {
            violations.push(LineageRegisterViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.lineage_state,
                expected_reason: expected,
            });
        };

        match row.lineage_state {
            LineageState::LineageRegressed => {
                if !row.has_active_reason(NarrowingReason::LineageDimensionFailed)
                    && !row.has_active_reason(NarrowingReason::LineageDimensionMissing)
                {
                    push_incoherent(violations, NarrowingReason::LineageDimensionFailed);
                }
            }
            LineageState::Stale => {
                if !row.has_active_reason(NarrowingReason::ProofPacketStale) {
                    push_incoherent(violations, NarrowingReason::ProofPacketStale);
                }
            }
            LineageState::RollbackUndefined => {
                if !row.has_active_reason(NarrowingReason::RollbackPlanUnverified)
                    && !row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
                {
                    push_incoherent(violations, NarrowingReason::DowngradeAutomationUndefined);
                }
            }
            LineageState::OwnerUnsigned => {
                if !row.has_active_reason(NarrowingReason::OwnerManifestUnsigned) {
                    push_incoherent(violations, NarrowingReason::OwnerManifestUnsigned);
                }
            }
            LineageState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(LineageRegisterViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.lineage_state,
                    });
                }
            }
            LineageState::Traced => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<LineageRegisterViolation>) {
        let covered: BTreeSet<String> = self
            .rows
            .iter()
            .map(|row| row.surface_ref.clone())
            .collect();
        for declared in &self.release_blocking_surface_refs {
            if !covered.contains(declared) {
                violations.push(LineageRegisterViolation::ReleaseBlockingSurfaceUncovered {
                    surface_ref: declared.clone(),
                });
            }
        }
        for row in &self.rows {
            if row.release_blocking
                && !self
                    .release_blocking_surface_refs
                    .contains(&row.surface_ref)
            {
                violations.push(LineageRegisterViolation::ReleaseBlockingRowNotDeclared {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<LineageRegisterViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(LineageRegisterViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(LineageRegisterViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(LineageRegisterViolation::PromotionDecisionInconsistent {
                declared: self.promotion.decision,
                computed,
            });
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(LineageRegisterViolation::PromotionBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(LineageRegisterViolation::PromotionBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the generated-artifact lineage register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineageRegisterViolation {
    /// The register carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the register.
        actual: u32,
    },
    /// The register carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the register.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The register has no surfaces.
    EmptyRegister,
    /// The register has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Surface or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A surface id appears more than once.
    DuplicateEntryId {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// A stop-rule id appears more than once.
    DuplicateStopRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A stop rule names no labels to watch.
    StopRuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A narrowing reason has no stop rule watching for it.
    NarrowingReasonWithoutStopRule {
        /// Uncovered reason.
        reason: NarrowingReason,
    },
    /// A surface has two cells for one dimension.
    DuplicateDimension {
        /// Surface id.
        entry_id: String,
        /// Duplicated dimension.
        dimension: LineageDimension,
    },
    /// A surface is missing a dimension cell.
    LineageIncompleteCoverage {
        /// Surface id.
        entry_id: String,
        /// Uncovered dimension.
        dimension: LineageDimension,
    },
    /// A non-missing cell has no evidence ref.
    CellEvidenceMissing {
        /// Surface id.
        entry_id: String,
        /// Dimension.
        dimension: LineageDimension,
    },
    /// A waived cell is carried without a waiver.
    WaivedCellWithoutWaiver {
        /// Surface id.
        entry_id: String,
        /// Dimension.
        dimension: LineageDimension,
    },
    /// A non-passing cell does not name its narrowing reason.
    CellReasonNotActive {
        /// Surface id.
        entry_id: String,
        /// Dimension.
        dimension: LineageDimension,
        /// The reason the cell requires.
        reason: NarrowingReason,
    },
    /// The published label is wider than the backed claim's canonical label.
    PublishedWiderThanClaim {
        /// Surface id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A surface holds a label while the claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Surface id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
    },
    /// A narrowing state carries no active reason.
    NarrowingWithoutReason {
        /// Surface id.
        entry_id: String,
        /// Lineage state.
        state: LineageState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Surface id.
        entry_id: String,
        /// Lineage state.
        state: LineageState,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held surface carries a published label different from the claim.
    HeldLabelNotEqualClaim {
        /// Surface id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held surface has active narrowing reasons.
    HeldWithActiveGap {
        /// Surface id.
        entry_id: String,
    },
    /// A held surface does not label its artifact as generated.
    HeldWithoutDisclosure {
        /// Surface id.
        entry_id: String,
    },
    /// A held surface has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Surface id.
        entry_id: String,
    },
    /// A held surface rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Surface id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held surface lacks owner-manifest sign-off.
    HeldWithoutSignoff {
        /// Surface id.
        entry_id: String,
    },
    /// A held surface lacks defined-and-verified downgrade automation.
    HeldWithoutDowngradeAutomation {
        /// Surface id.
        entry_id: String,
        /// Automation state.
        state: AutomationState,
    },
    /// The downgrade automation floor is not below the cutline.
    DowngradeFloorNotBelowCutline {
        /// Surface id.
        entry_id: String,
        /// Declared floor.
        floor: StableClaimLevel,
    },
    /// An undefined automation does not name the undefined reason.
    AutomationStateWithoutReason {
        /// Surface id.
        entry_id: String,
        /// Automation state.
        state: AutomationState,
    },
    /// An unverified rollback plan does not name a rollback/downgrade reason.
    RollbackUnverifiedWithoutReason {
        /// Surface id.
        entry_id: String,
    },
    /// A narrowing surface with a breached proof packet does not name the stale reason.
    BreachedPacketWithoutReason {
        /// Surface id.
        entry_id: String,
    },
    /// A narrowing surface with a missing proof packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Surface id.
        entry_id: String,
    },
    /// A lineage state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Surface id.
        entry_id: String,
        /// Lineage state.
        state: LineageState,
        /// Reason the state requires.
        expected_reason: NarrowingReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Surface id.
        entry_id: String,
        /// Lineage state.
        state: LineageState,
    },
    /// A release-blocking surface ref has no covering surface.
    ReleaseBlockingSurfaceUncovered {
        /// Surface ref.
        surface_ref: String,
    },
    /// A release-blocking surface is not declared in the release-blocking list.
    ReleaseBlockingRowNotDeclared {
        /// Surface id.
        entry_id: String,
    },
    /// The declared promotion decision disagrees with the computed one.
    PromotionDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// The declared promotion blocking set disagrees with the computed one.
    PromotionBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The summary counts disagree with the surfaces.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Surface id.
        entry_id: String,
    },
}

impl fmt::Display for LineageRegisterViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported register schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported register record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "register {field} is not the canonical value")
            }
            Self::EmptyRegister => write!(f, "register has no surfaces"),
            Self::NoStopRules => write!(f, "register has no stop rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id {entry_id}")
            }
            Self::DuplicateStopRuleId { rule_id } => {
                write!(f, "duplicate stop rule id {rule_id}")
            }
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::NarrowingReasonWithoutStopRule { reason } => write!(
                f,
                "narrowing reason {} has no stop rule watching for it",
                reason.as_str()
            ),
            Self::DuplicateDimension {
                entry_id,
                dimension,
            } => write!(
                f,
                "surface {entry_id} has duplicate dimension {}",
                dimension.as_str()
            ),
            Self::LineageIncompleteCoverage {
                entry_id,
                dimension,
            } => write!(
                f,
                "surface {entry_id} is missing dimension {}",
                dimension.as_str()
            ),
            Self::CellEvidenceMissing {
                entry_id,
                dimension,
            } => write!(
                f,
                "surface {entry_id} dimension {} has no evidence ref",
                dimension.as_str()
            ),
            Self::WaivedCellWithoutWaiver {
                entry_id,
                dimension,
            } => write!(
                f,
                "surface {entry_id} dimension {} is waived without a waiver",
                dimension.as_str()
            ),
            Self::CellReasonNotActive {
                entry_id,
                dimension,
                reason,
            } => write!(
                f,
                "surface {entry_id} dimension {} requires active reason {}",
                dimension.as_str(),
                reason.as_str()
            ),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "surface {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "surface {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "surface {entry_id} state {state:?} narrows without active reason"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "surface {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "surface {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "surface {entry_id} holds stable with active gap")
            }
            Self::HeldWithoutDisclosure { entry_id } => {
                write!(
                    f,
                    "surface {entry_id} holds stable without labeling its artifact as generated"
                )
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "surface {entry_id} holds stable without fresh packet")
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => {
                write!(
                    f,
                    "surface {entry_id} holds stable on stale packet {slo_state:?}"
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "surface {entry_id} holds stable without owner signoff")
            }
            Self::HeldWithoutDowngradeAutomation { entry_id, state } => write!(
                f,
                "surface {entry_id} holds stable without defined+verified downgrade automation ({state:?})"
            ),
            Self::DowngradeFloorNotBelowCutline { entry_id, floor } => write!(
                f,
                "surface {entry_id} downgrade floor {floor:?} is not below the cutline"
            ),
            Self::AutomationStateWithoutReason { entry_id, state } => write!(
                f,
                "surface {entry_id} automation state {state:?} names no narrowing reason"
            ),
            Self::RollbackUnverifiedWithoutReason { entry_id } => write!(
                f,
                "surface {entry_id} has an unverified rollback plan without a reason"
            ),
            Self::BreachedPacketWithoutReason { entry_id } => write!(
                f,
                "surface {entry_id} breached packet without proof_packet_stale reason"
            ),
            Self::MissingPacketWithoutReason { entry_id } => write!(
                f,
                "surface {entry_id} missing packet without proof_packet_missing reason"
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "surface {entry_id} state {state:?} requires reason {expected_reason:?}"
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(f, "surface {entry_id} state {state:?} names no waiver")
            }
            Self::ReleaseBlockingSurfaceUncovered { surface_ref } => {
                write!(
                    f,
                    "release-blocking surface {surface_ref} has no covering surface"
                )
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => write!(
                f,
                "release-blocking surface {entry_id} is not declared in release_blocking_surface_refs"
            ),
            Self::PromotionDecisionInconsistent { declared, computed } => {
                write!(f, "promotion {declared:?} disagrees with computed {computed:?}")
            }
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with surfaces"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "surface {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for LineageRegisterViolation {}

/// Loads the embedded generated-artifact lineage register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`GeneratedArtifactLineageRegister`].
pub fn current_generated_artifact_lineage_register(
) -> Result<GeneratedArtifactLineageRegister, serde_json::Error> {
    serde_json::from_str(GENERATED_ARTIFACT_LINEAGE_JSON)
}

#[cfg(test)]
mod tests;
