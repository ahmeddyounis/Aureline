//! Typed claim-narrowing automation for optional surfaces that lack a stable
//! qualification packet.
//!
//! The [`stable_claim_manifest`](crate::stable_claim_manifest) assigns each
//! *subject* its single canonical lifecycle label; the
//! [`stable_qualification_matrix`](crate::stable_qualification_matrix) decides whether
//! each per-lane *qualification row* holds its claimed level; the
//! [`stable_proof_index`](crate::stable_proof_index) decides whether each launch-blocking
//! *requirement* is proven. All three speak for surfaces that are *meant* to ship at the
//! stable cutline. None of them answer the question this module answers: **for each
//! optional surface — an opt-in capability, an optional integration, a secondary platform,
//! or a shipped-but-experimental preview — is there a fresh, complete, owner-signed stable
//! qualification packet behind it, or is the surface riding on optimism and an adjacent
//! green row?**
//!
//! This module is the **optional-surface qualification register**. Its governing rule is
//! the inverse of the failure mode the release guardrails warn about: an optional surface
//! does **not** inherit Stable from a neighbouring claim. The default for an optional
//! surface with no stable qualification packet is *narrowed*, and the only way back above
//! the cutline is to author and capture a packet. Each [`FinalizeOptionalSurface`] is therefore one
//! `(optional surface, public claim)` binding whose qualification packet is an
//! [`Option<ProofPacket>`]: when it is `None`, the surface **lacks a stable qualification
//! packet** and is structurally required to narrow below the cutline; when it is `Some`, the
//! packet's freshness SLO, capture, and evidence decide whether the surface may render at the
//! public claim's label.
//!
//! Each [`FinalizeOptionalSurface`]:
//!
//! - names the optional surface it governs ([`FinalizeOptionalSurface::surface_kind`],
//!   [`FinalizeOptionalSurface::surface_ref`], [`FinalizeOptionalSurface::surface_summary`]) and whether it
//!   is part of the release-relevant surface set ([`FinalizeOptionalSurface::release_relevant`]);
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose public
//!   claim it backs ([`FinalizeOptionalSurface::claim_ref`]) and the canonical lifecycle label that
//!   entry publishes ([`FinalizeOptionalSurface::claim_label`]). That label is a hard **ceiling**: a
//!   surface may render at the claim's label or narrow below it, but it may never display a
//!   maturity wider than the public claim it backs;
//! - carries its qualification packet as an [`Option<ProofPacket>`] with a packet-freshness
//!   SLO — `None` is the canonical "no stable qualification packet" state;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-surface labels, so
//!   docs, Help/About, the release center, and support exports ingest one label per surface
//!   instead of cloning their own;
//! - records the surface state earned ([`FinalizeSurfaceState`]), the active narrow reasons
//!   ([`FinalizeNarrowReason`]), and the label it *effectively* displays after narrowing
//!   ([`FinalizeOptionalSurface::displayed_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary between a
//! surface that renders qualified for a Stable public claim and one narrowed below it. A
//! surface that is not qualified — because it has no qualification packet, because its packet
//! breached its freshness SLO, because its surface evidence or capability is incomplete,
//! because a waiver it relied on expired, or because the public claim it backs is itself
//! below the cutline — is structurally required to drop below the cutline rather than inherit
//! an adjacent qualified surface. The [`FinalizeSurfaceStopRule`] set names the closed conditions
//! that gate promotion, and [`FinalizeQualificationPacketsForOptionalSurfacesAndEnforce::publication`] records the
//! proceed/hold verdict.
//!
//! The register is checked in at `artifacts/release/finalize_qualification_packets_for_optional_surfaces_and_enforce.json` and
//! embedded here, so this typed consumer and the CI gate agree on every surface without a
//! cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It carries no
//! raw artifacts, raw logs, signatures, or credential material. Two classes of check live
//! outside this model because they need more than the register sees: date arithmetic
//! (recomputing the packet-freshness state and waiver expiry against an `as_of` date) and the
//! claim-ceiling cross-check (whether each surface's `claim_label` still equals the label the
//! stable claim manifest publishes for the entry named by `claim_ref`). Those live in the CI
//! gate. This model enforces the structural and logical invariants that hold regardless of
//! the clock and the neighbouring artifact — the ceiling/no-widening rule, the
//! absent-packet-narrows rule, narrowing consistency, packet/state coherence, owner sign-off
//! on qualified surfaces, surface-kind and release-set coverage, publication-rule wiring, and
//! the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported optional-surface-qualification schema version.
pub const FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_RECORD_KIND: &str = "finalize_qualification_packets_for_optional_surfaces_and_enforce";

/// Repo-relative path to the checked-in register.
pub const FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_PATH: &str =
    "artifacts/release/finalize_qualification_packets_for_optional_surfaces_and_enforce.json";

/// Embedded checked-in register JSON.
pub const FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/finalize_qualification_packets_for_optional_surfaces_and_enforce.json"
));

/// The class of optional surface a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizeOptionalSurfaceKind {
    /// An opt-in capability behind a feature toggle.
    OptInCapability,
    /// An optional third-party or external integration.
    OptionalIntegration,
    /// A secondary platform, deployment, or runtime target.
    SecondaryPlatform,
    /// A shipped-but-experimental preview surface.
    ExperimentalPreview,
}

impl FinalizeOptionalSurfaceKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OptInCapability,
        Self::OptionalIntegration,
        Self::SecondaryPlatform,
        Self::ExperimentalPreview,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OptInCapability => "opt_in_capability",
            Self::OptionalIntegration => "optional_integration",
            Self::SecondaryPlatform => "secondary_platform",
            Self::ExperimentalPreview => "experimental_preview",
        }
    }
}

/// Deployment target for per-row optional-surface qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentTarget {
    /// Desktop local installation.
    DesktopLocal,
    /// Remote or helper-connected environment.
    RemoteHelper,
    /// Vendor-managed hosted environment.
    Managed,
    /// Customer self-hosted environment.
    SelfHosted,
    /// Offline / air-gapped deployment.
    AirGapped,
}

impl DeploymentTarget {
    /// Every deployment target, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DesktopLocal,
        Self::RemoteHelper,
        Self::Managed,
        Self::SelfHosted,
        Self::AirGapped,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopLocal => "desktop_local",
            Self::RemoteHelper => "remote_helper",
            Self::Managed => "managed",
            Self::SelfHosted => "self_hosted",
            Self::AirGapped => "air_gapped",
        }
    }
}

/// Deployment-specific access mode for an optional surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentAccessMode {
    /// Full stable access.
    Stable,
    /// Preview access (below the stable cutline).
    Preview,
    /// Inspect-only access.
    InspectOnly,
    /// Handoff-only access.
    HandoffOnly,
    /// Hidden from users on this deployment target.
    Hidden,
    /// Client or profile-limited access.
    ClientProfileLimited,
}

impl DeploymentAccessMode {
    /// Every access mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stable,
        Self::Preview,
        Self::InspectOnly,
        Self::HandoffOnly,
        Self::Hidden,
        Self::ClientProfileLimited,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Preview => "preview",
            Self::InspectOnly => "inspect_only",
            Self::HandoffOnly => "handoff_only",
            Self::Hidden => "hidden",
            Self::ClientProfileLimited => "client_profile_limited",
        }
    }

    /// Whether this access mode renders at or above the stable cutline.
    pub const fn renders_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Per-deployment-target qualification record for an optional surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeploymentQualification {
    /// The deployment target this row records.
    pub deployment_target: DeploymentTarget,
    /// The access mode the surface carries on this deployment target.
    pub access_mode: DeploymentAccessMode,
    /// Surface state earned on this deployment target.
    pub surface_state: FinalizeSurfaceState,
    /// Active narrow reasons on this deployment target.
    #[serde(default)]
    pub active_narrow_reasons: Vec<FinalizeNarrowReason>,
    /// Reviewable reason this deployment target carries this posture.
    pub rationale: String,
}

/// Surface state a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizeSurfaceState {
    /// The surface is qualified: a captured, within-SLO qualification packet, complete
    /// evidence, and an owner sign-off back the public claim at its full canonical
    /// lifecycle label.
    QualifiedStable,
    /// The surface renders the claim's full label only because an active, unexpired waiver
    /// covers a recorded gap.
    QualifiedOnWaiver,
    /// The surface has **no** stable qualification packet at all; it is narrowed and may
    /// never inherit an adjacent qualified surface.
    NarrowedNoPacket,
    /// The surface carries a packet but its surface evidence is incomplete or a surface
    /// capability is absent; the label must narrow.
    NarrowedIncomplete,
    /// The surface's qualification packet breached its freshness SLO; the label must narrow.
    NarrowedStale,
    /// The public claim this surface backs is itself below the cutline, so the surface
    /// inherits that ceiling and narrows.
    NarrowedClaimNarrowed,
    /// The surface relied on a waiver that has expired; the label must narrow.
    NarrowedWaiverExpired,
}

impl FinalizeSurfaceState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::QualifiedStable,
        Self::QualifiedOnWaiver,
        Self::NarrowedNoPacket,
        Self::NarrowedIncomplete,
        Self::NarrowedStale,
        Self::NarrowedClaimNarrowed,
        Self::NarrowedWaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualifiedStable => "qualified_stable",
            Self::QualifiedOnWaiver => "qualified_on_waiver",
            Self::NarrowedNoPacket => "narrowed_no_packet",
            Self::NarrowedIncomplete => "narrowed_incomplete",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedClaimNarrowed => "narrowed_claim_narrowed",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
        }
    }

    /// Whether the state lets a surface render the public claim at its label.
    pub const fn renders_qualified(self) -> bool {
        matches!(self, Self::QualifiedStable | Self::QualifiedOnWaiver)
    }

    /// Whether the state forces the surface below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.renders_qualified()
    }
}

/// Closed reason a surface narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizeNarrowReason {
    /// The public claim this surface backs is itself below the cutline.
    ClaimLabelNarrowed,
    /// The surface has no stable qualification packet at all.
    QualificationPacketAbsent,
    /// The surface names a capability the build does not yet implement.
    SurfaceCapabilityAbsent,
    /// The surface's qualification evidence is incomplete.
    SurfaceEvidenceIncomplete,
    /// The qualification packet breached its freshness SLO.
    QualificationPacketBreached,
    /// A waiver the surface relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl FinalizeNarrowReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ClaimLabelNarrowed,
        Self::QualificationPacketAbsent,
        Self::SurfaceCapabilityAbsent,
        Self::SurfaceEvidenceIncomplete,
        Self::QualificationPacketBreached,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::QualificationPacketAbsent => "qualification_packet_absent",
            Self::SurfaceCapabilityAbsent => "surface_capability_absent",
            Self::SurfaceEvidenceIncomplete => "surface_evidence_incomplete",
            Self::QualificationPacketBreached => "qualification_packet_breached",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizeNarrowAction {
    /// Hold stable promotion until the condition clears.
    HoldPromotion,
    /// Narrow the surface's displayed lifecycle label below the cutline.
    NarrowSurfaceLabel,
    /// Author and capture a stable qualification packet for the surface.
    AuthorQualificationPacket,
    /// Refresh the qualification packet so it re-enters its SLO.
    RefreshQualificationPacket,
    /// Recapture the surface-level qualification evidence.
    RecaptureSurfaceEvidence,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl FinalizeNarrowAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HoldPromotion,
        Self::NarrowSurfaceLabel,
        Self::AuthorQualificationPacket,
        Self::RefreshQualificationPacket,
        Self::RecaptureSurfaceEvidence,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowSurfaceLabel => "narrow_surface_label",
            Self::AuthorQualificationPacket => "author_qualification_packet",
            Self::RefreshQualificationPacket => "refresh_qualification_packet",
            Self::RecaptureSurfaceEvidence => "recapture_surface_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One surface stop rule: a closed condition that narrows a surface label and may gate
/// stable promotion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizeSurfaceStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrow reason whose presence on a watched surface fires this rule.
    pub trigger_reason: FinalizeNarrowReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: FinalizeNarrowAction,
    /// Whether firing this rule blocks stable promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One optional surface: a `(surface, public claim)` binding bound to its optional
/// qualification packet, canonical ceiling label, and packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizeOptionalSurface {
    /// Stable surface id.
    pub surface_id: String,
    /// Human-readable title.
    pub title: String,
    /// The class of optional surface this row governs.
    pub surface_kind: FinalizeOptionalSurfaceKind,
    /// The surface id/ref this row governs.
    pub surface_ref: String,
    /// Reviewable one-line statement of the surface.
    pub surface_summary: String,
    /// Whether the surface is part of the release-relevant surface set.
    pub release_relevant: bool,
    /// The stable-claim-manifest entry id whose public claim this surface backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: a surface may
    /// never render a label wider than this.
    pub claim_label: StableClaimLevel,
    /// Surface state earned for the surface.
    pub surface_state: FinalizeSurfaceState,
    /// Ref to the source the surface is defined or described by.
    pub source_ref: String,
    /// The qualification packet and its SLO, or `None` when the surface has no stable
    /// qualification packet at all.
    #[serde(default)]
    pub qualification_packet: Option<ProofPacket>,
    /// Waiver authorizing a provisional qualified surface, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrow reasons narrowing the surface.
    #[serde(default)]
    pub active_narrow_reasons: Vec<FinalizeNarrowReason>,
    /// The lifecycle label the surface effectively displays after narrowing.
    pub displayed_label: StableClaimLevel,
    /// Per-deployment-target qualification records for this surface.
    #[serde(default)]
    pub deployment_qualifications: Vec<DeploymentQualification>,
    /// Publication destinations that render this surface's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the surface carries this posture.
    pub rationale: String,
}

impl FinalizeOptionalSurface {
    /// True when the displayed label is at or above the cutline.
    pub fn renders_stable(&self) -> bool {
        self.displayed_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the surface's state lets it render its claimed label.
    pub fn renders_qualified(&self) -> bool {
        self.surface_state.renders_qualified()
    }

    /// True when the surface carries a stable qualification packet.
    pub fn has_packet(&self) -> bool {
        self.qualification_packet.is_some()
    }

    /// True when a narrow reason is active on the surface.
    pub fn has_active_reason(&self, reason: FinalizeNarrowReason) -> bool {
        self.active_narrow_reasons.contains(&reason)
    }

    /// The packet's freshness-SLO state, or `None` when the surface has no packet.
    pub fn slo_state(&self) -> Option<FreshnessSloState> {
        self.qualification_packet.as_ref().map(|p| p.slo_state)
    }
}

/// The recorded publication verdict for the optional-surface register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizeSurfacePublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Stop-rule ids that block promotion, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Surface ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_surface_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizeQualificationSummary {
    /// Total number of surfaces.
    pub total_surfaces: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Surfaces rendering a label at or above the cutline.
    pub surfaces_qualified_stable: usize,
    /// Surfaces narrowed below the cutline.
    pub surfaces_narrowed_below_cutline: usize,
    /// Surfaces rendering qualified via an active waiver.
    pub surfaces_on_active_waiver: usize,
    /// Surfaces carrying a stable qualification packet.
    pub surfaces_with_packet: usize,
    /// Surfaces lacking a stable qualification packet entirely.
    pub surfaces_without_packet: usize,
    /// Total release-relevant surfaces.
    pub release_relevant_total: usize,
    /// Release-relevant surfaces rendering a label at or above the cutline.
    pub release_relevant_qualified: usize,
    /// Release-relevant surfaces narrowed below the cutline.
    pub release_relevant_narrowed: usize,
    /// Opt-in-capability surfaces.
    pub opt_in_capability_surfaces: usize,
    /// Optional-integration surfaces.
    pub optional_integration_surfaces: usize,
    /// Secondary-platform surfaces.
    pub secondary_platform_surfaces: usize,
    /// Experimental-preview surfaces.
    pub experimental_preview_surfaces: usize,
    /// Packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Total active narrow reasons across all surfaces.
    pub total_active_narrow_reasons: usize,
    /// Number of stop rules currently firing.
    pub stop_rules_firing: usize,
    /// Deployment rows that render stable.
    pub deployment_rows_stable: usize,
    /// Deployment rows that render below stable.
    pub deployment_rows_narrowed: usize,
    /// Release-relevant surfaces with complete deployment coverage.
    pub release_relevant_complete_coverage: usize,
}

/// The typed optional-surface qualification register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizeQualificationPacketsForOptionalSurfacesAndEnforce {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register identifier.
    pub register_id: String,
    /// Lifecycle status of this register artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this register ingests as its public-claim source and
    /// ceiling.
    pub claim_manifest_ref: String,
    /// Ref to the freshness-SLO register every qualification packet rides.
    pub freshness_slo_register_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed surface-kind vocabulary.
    pub surface_kinds: Vec<FinalizeOptionalSurfaceKind>,
    /// Closed surface-state vocabulary.
    pub surface_states: Vec<FinalizeSurfaceState>,
    /// Closed narrow-reason vocabulary.
    pub narrow_reasons: Vec<FinalizeNarrowReason>,
    /// Closed narrow-action vocabulary.
    pub narrow_actions: Vec<FinalizeNarrowAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-relevant surface refs this register must cover.
    pub release_relevant_surface_refs: Vec<String>,
    /// Surface stop rules.
    pub stop_rules: Vec<FinalizeSurfaceStopRule>,
    /// Optional surfaces.
    pub surfaces: Vec<FinalizeOptionalSurface>,
    /// Recorded publication verdict.
    pub publication: FinalizeSurfacePublicationRecord,
    /// Summary counts.
    pub summary: FinalizeQualificationSummary,
}

impl FinalizeQualificationPacketsForOptionalSurfacesAndEnforce {
    /// Returns the surface registered for `surface_id`.
    pub fn surface(&self, surface_id: &str) -> Option<&FinalizeOptionalSurface> {
        self.surfaces
            .iter()
            .find(|surface| surface.surface_id == surface_id)
    }

    /// Returns the surfaces rendering a label at or above the cutline.
    pub fn surfaces_qualified_stable(&self) -> Vec<&FinalizeOptionalSurface> {
        self.surfaces
            .iter()
            .filter(|surface| surface.renders_stable())
            .collect()
    }

    /// Returns the surfaces narrowed below the cutline.
    pub fn surfaces_narrowed(&self) -> Vec<&FinalizeOptionalSurface> {
        self.surfaces
            .iter()
            .filter(|surface| !surface.renders_stable())
            .collect()
    }

    /// Returns the surfaces lacking a stable qualification packet.
    pub fn surfaces_without_packet(&self) -> Vec<&FinalizeOptionalSurface> {
        self.surfaces
            .iter()
            .filter(|surface| !surface.has_packet())
            .collect()
    }

    /// Returns the release-relevant surfaces.
    pub fn release_relevant_surfaces(&self) -> Vec<&FinalizeOptionalSurface> {
        self.surfaces
            .iter()
            .filter(|surface| surface.release_relevant)
            .collect()
    }

    /// Returns the surfaces of one kind.
    pub fn surfaces_for_kind(&self, kind: FinalizeOptionalSurfaceKind) -> Vec<&FinalizeOptionalSurface> {
        self.surfaces
            .iter()
            .filter(|surface| surface.surface_kind == kind)
            .collect()
    }

    /// Distinct public claims (by claim ref) the register covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for surface in &self.surfaces {
            set.insert(surface.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched surface carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &FinalizeSurfaceStopRule) -> bool {
        self.surfaces.iter().any(|surface| {
            rule.applies_to_labels.contains(&surface.claim_label)
                && surface.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the surfaces and stop rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
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

    /// Rule ids that block promotion and are currently firing, sorted.
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
    /// Only surfaces whose public claim is at or above the cutline count: a surface whose
    /// claim is already canonically narrowed is not a *register* blocker, it merely inherits
    /// the upstream ceiling.
    pub fn computed_blocking_surface_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<FinalizeNarrowReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for surface in &self.surfaces {
            if surface.claim_holds_stable()
                && surface
                    .active_narrow_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(surface.surface_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the surfaces and stop rules.
    pub fn computed_summary(&self) -> FinalizeQualificationSummary {
        let packets = |state: FreshnessSloState| {
            self.surfaces
                .iter()
                .filter(|surface| surface.slo_state() == Some(state))
                .count()
        };
        let kind = |kind: FinalizeOptionalSurfaceKind| self.surfaces_for_kind(kind).len();
        let release_relevant: Vec<&FinalizeOptionalSurface> = self
            .surfaces
            .iter()
            .filter(|surface| surface.release_relevant)
            .collect();
        FinalizeQualificationSummary {
            total_surfaces: self.surfaces.len(),
            total_claims: self.claims().len(),
            surfaces_qualified_stable: self
                .surfaces
                .iter()
                .filter(|surface| surface.renders_stable())
                .count(),
            surfaces_narrowed_below_cutline: self
                .surfaces
                .iter()
                .filter(|surface| !surface.renders_stable())
                .count(),
            surfaces_on_active_waiver: self
                .surfaces
                .iter()
                .filter(|surface| surface.surface_state == FinalizeSurfaceState::QualifiedOnWaiver)
                .count(),
            surfaces_with_packet: self
                .surfaces
                .iter()
                .filter(|surface| surface.has_packet())
                .count(),
            surfaces_without_packet: self
                .surfaces
                .iter()
                .filter(|surface| !surface.has_packet())
                .count(),
            release_relevant_total: release_relevant.len(),
            release_relevant_qualified: release_relevant
                .iter()
                .filter(|surface| surface.renders_stable())
                .count(),
            release_relevant_narrowed: release_relevant
                .iter()
                .filter(|surface| !surface.renders_stable())
                .count(),
            opt_in_capability_surfaces: kind(FinalizeOptionalSurfaceKind::OptInCapability),
            optional_integration_surfaces: kind(FinalizeOptionalSurfaceKind::OptionalIntegration),
            secondary_platform_surfaces: kind(FinalizeOptionalSurfaceKind::SecondaryPlatform),
            experimental_preview_surfaces: kind(FinalizeOptionalSurfaceKind::ExperimentalPreview),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            total_active_narrow_reasons: self
                .surfaces
                .iter()
                .map(|surface| surface.active_narrow_reasons.len())
                .sum(),
            stop_rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
            deployment_rows_stable: self
                .surfaces
                .iter()
                .flat_map(|surface| &surface.deployment_qualifications)
                .filter(|dq| dq.access_mode.renders_stable())
                .count(),
            deployment_rows_narrowed: self
                .surfaces
                .iter()
                .flat_map(|surface| &surface.deployment_qualifications)
                .filter(|dq| !dq.access_mode.renders_stable())
                .count(),
            release_relevant_complete_coverage: self
                .surfaces
                .iter()
                .filter(|surface| {
                    surface.release_relevant
                        && surface.deployment_qualifications.len() == DeploymentTarget::ALL.len()
                })
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the register that downstream
    /// surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> FinalizeSurfaceExportProjection {
        FinalizeSurfaceExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            surfaces: self
                .surfaces
                .iter()
                .map(|surface| FinalizeSurfaceExportRow {
                    surface_id: surface.surface_id.clone(),
                    surface_kind: surface.surface_kind,
                    surface_ref: surface.surface_ref.clone(),
                    release_relevant: surface.release_relevant,
                    claim_ref: surface.claim_ref.clone(),
                    claim_label: surface.claim_label,
                    displayed_label: surface.displayed_label,
                    renders_stable: surface.renders_stable(),
                    surface_state: surface.surface_state,
                    has_packet: surface.has_packet(),
                    slo_state: surface.slo_state(),
                    active_narrow_reasons: surface.active_narrow_reasons.clone(),
                    deployment_qualifications: surface.deployment_qualifications.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<FinalizeQualificationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for surface in &self.surfaces {
            if !seen.insert(surface.surface_id.clone()) {
                violations.push(FinalizeQualificationViolation::DuplicateSurfaceId {
                    surface_id: surface.surface_id.clone(),
                });
            }
            self.validate_surface(surface, &mut violations);
        }
        if self.surfaces.is_empty() {
            violations.push(FinalizeQualificationViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(FinalizeQualificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<FinalizeQualificationViolation>) {
        if self.schema_version != FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_SCHEMA_VERSION {
            violations.push(
                FinalizeQualificationViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_RECORD_KIND {
            violations.push(
                FinalizeQualificationViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            (
                "freshness_slo_register_ref",
                &self.freshness_slo_register_ref,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(FinalizeQualificationViolation::EmptyField {
                    surface_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(
                FinalizeQualificationViolation::ClosedVocabularyMismatch {
                    field: "lifecycle_labels",
                },
            );
        }
        if self.surface_kinds != FinalizeOptionalSurfaceKind::ALL.to_vec() {
            violations.push(
                FinalizeQualificationViolation::ClosedVocabularyMismatch {
                    field: "surface_kinds",
                },
            );
        }
        if self.surface_states != FinalizeSurfaceState::ALL.to_vec() {
            violations.push(
                FinalizeQualificationViolation::ClosedVocabularyMismatch {
                    field: "surface_states",
                },
            );
        }
        if self.narrow_reasons != FinalizeNarrowReason::ALL.to_vec() {
            violations.push(
                FinalizeQualificationViolation::ClosedVocabularyMismatch {
                    field: "narrow_reasons",
                },
            );
        }
        if self.narrow_actions != FinalizeNarrowAction::ALL.to_vec() {
            violations.push(
                FinalizeQualificationViolation::ClosedVocabularyMismatch {
                    field: "narrow_actions",
                },
            );
        }
        if self.release_relevant_surface_refs.is_empty() {
            violations.push(FinalizeQualificationViolation::EmptyField {
                surface_id: "<register>".to_owned(),
                field_name: "release_relevant_surface_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(
                FinalizeQualificationViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.cutline_level",
                },
            );
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(
                FinalizeQualificationViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.above_cutline_levels",
                },
            );
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(
                FinalizeQualificationViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.below_cutline_levels",
                },
            );
        }
        if cutline.description.trim().is_empty() {
            violations.push(FinalizeQualificationViolation::EmptyField {
                surface_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<FinalizeQualificationViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(FinalizeQualificationViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(FinalizeQualificationViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(FinalizeQualificationViolation::EmptyField {
                        surface_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(FinalizeQualificationViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        // Every narrow reason must have a rule, so a reason cannot fire without a
        // corresponding promotion gate.
        for reason in FinalizeNarrowReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(FinalizeQualificationViolation::ReasonWithoutRule { reason });
            }
        }
    }

    fn validate_surface(
        &self,
        surface: &FinalizeOptionalSurface,
        violations: &mut Vec<FinalizeQualificationViolation>,
    ) {
        for (field, value) in [
            ("surface_id", &surface.surface_id),
            ("title", &surface.title),
            ("surface_ref", &surface.surface_ref),
            ("surface_summary", &surface.surface_summary),
            ("claim_ref", &surface.claim_ref),
            ("source_ref", &surface.source_ref),
            ("rationale", &surface.rationale),
            ("owner_signoff.owner_ref", &surface.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(FinalizeQualificationViolation::EmptyField {
                    surface_id: surface.surface_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no surface may render a label wider than the public claim's canonical
        // label.
        if surface.displayed_label.rank() > surface.claim_label.rank() {
            violations.push(
                FinalizeQualificationViolation::DisplayedWiderThanClaim {
                    surface_id: surface.surface_id.clone(),
                    claim: surface.claim_label,
                    displayed: surface.displayed_label,
                },
            );
        }

        self.validate_packet(surface, violations);

        // A public claim whose canonical label is below the cutline forces the surface to
        // inherit that ceiling and narrow.
        if !surface.claim_holds_stable() {
            if surface.renders_qualified() {
                violations.push(
                    FinalizeQualificationViolation::QualifiedOnNarrowedClaim {
                        surface_id: surface.surface_id.clone(),
                        claim: surface.claim_label,
                    },
                );
            }
            if !surface.has_active_reason(FinalizeNarrowReason::ClaimLabelNarrowed) {
                violations.push(
                    FinalizeQualificationViolation::ClaimNarrowedWithoutReason {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        // The absent-packet rule: a surface with no qualification packet must be narrowed,
        // must name qualification_packet_absent, and must never render qualified — it can
        // never inherit an adjacent qualified surface.
        if !surface.has_packet() {
            if surface.renders_qualified() {
                violations.push(
                    FinalizeQualificationViolation::QualifiedWithoutPacket {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if !surface.has_active_reason(FinalizeNarrowReason::QualificationPacketAbsent) {
                violations.push(
                    FinalizeQualificationViolation::AbsentPacketWithoutReason {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        } else if surface.has_active_reason(FinalizeNarrowReason::QualificationPacketAbsent) {
            // A surface that *does* carry a packet may not claim the packet is absent.
            violations.push(
                FinalizeQualificationViolation::PacketPresentButReasonAbsent {
                    surface_id: surface.surface_id.clone(),
                },
            );
        }

        let slo_state = surface.slo_state();

        if surface.renders_qualified() {
            // A qualified surface renders exactly the public claim's canonical label, carries
            // no active narrow reason, rides a captured within-SLO packet, and is owner-signed.
            if surface.displayed_label != surface.claim_label {
                violations.push(
                    FinalizeQualificationViolation::QualifiedLabelNotEqualClaim {
                        surface_id: surface.surface_id.clone(),
                        claim: surface.claim_label,
                        displayed: surface.displayed_label,
                    },
                );
            }
            if !surface.active_narrow_reasons.is_empty() {
                violations.push(
                    FinalizeQualificationViolation::QualifiedWithActiveReason {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            match &surface.qualification_packet {
                None => {
                    // Already flagged by the absent-packet rule above.
                }
                Some(packet) => {
                    if !packet.has_capture() {
                        violations.push(
                            FinalizeQualificationViolation::QualifiedWithoutFreshPacket {
                                surface_id: surface.surface_id.clone(),
                            },
                        );
                    }
                    if !packet.slo_state.is_within_slo() {
                        violations.push(
                            FinalizeQualificationViolation::QualifiedOnStalePacket {
                                surface_id: surface.surface_id.clone(),
                                slo_state: packet.slo_state,
                            },
                        );
                    }
                }
            }
            if !(surface.owner_signoff.signed_off && surface.owner_signoff.signed_at.is_some()) {
                violations.push(
                    FinalizeQualificationViolation::QualifiedWithoutSignoff {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        } else {
            // A narrowing state must drop the displayed label below the cutline and name at
            // least one active reason.
            if surface.renders_stable() {
                violations.push(
                    FinalizeQualificationViolation::DisplayedLabelNotNarrowed {
                        surface_id: surface.surface_id.clone(),
                        state: surface.surface_state,
                        displayed: surface.displayed_label,
                    },
                );
            }
            if surface.active_narrow_reasons.is_empty() {
                violations.push(
                    FinalizeQualificationViolation::NarrowingWithoutReason {
                        surface_id: surface.surface_id.clone(),
                        state: surface.surface_state,
                    },
                );
            }
            // A narrowing surface whose packet is breached must name the matching freshness
            // reason, so the freshness automation stays honest.
            if slo_state == Some(FreshnessSloState::Breached)
                && !surface.has_active_reason(FinalizeNarrowReason::QualificationPacketBreached)
            {
                violations.push(
                    FinalizeQualificationViolation::BreachedPacketWithoutReason {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        // Deployment qualification coherence: a narrowed surface or a surface without a
        // packet must not have any deployment row rendering stable.
        for dq in &surface.deployment_qualifications {
            if dq.access_mode == DeploymentAccessMode::Stable {
                if !surface.has_packet() {
                    violations.push(
                        FinalizeQualificationViolation::DeploymentStableWithoutPacket {
                            surface_id: surface.surface_id.clone(),
                            deployment_target: dq.deployment_target,
                        },
                    );
                }
                if !surface.renders_stable() {
                    violations.push(
                        FinalizeQualificationViolation::DeploymentStableOnNarrowedSurface {
                            surface_id: surface.surface_id.clone(),
                            deployment_target: dq.deployment_target,
                        },
                    );
                }
            }
        }

        self.validate_state_reason_coherence(surface, violations);
    }

    fn validate_packet(
        &self,
        surface: &FinalizeOptionalSurface,
        violations: &mut Vec<FinalizeQualificationViolation>,
    ) {
        let Some(packet) = &surface.qualification_packet else {
            return;
        };
        for (field, value) in [
            ("qualification_packet.packet_id", &packet.packet_id),
            ("qualification_packet.packet_ref", &packet.packet_ref),
            (
                "qualification_packet.proof_index_ref",
                &packet.proof_index_ref,
            ),
            (
                "qualification_packet.freshness_slo.slo_register_ref",
                &packet.freshness_slo.slo_register_ref,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(FinalizeQualificationViolation::EmptyField {
                    surface_id: surface.surface_id.clone(),
                    field_name: field,
                });
            }
        }
        // The freshness SLO target must be a positive number of days and the warn window may
        // not exceed it.
        if packet.freshness_slo.target_max_age_days == 0 {
            violations.push(FinalizeQualificationViolation::EmptyField {
                surface_id: surface.surface_id.clone(),
                field_name: "qualification_packet.freshness_slo.target_max_age_days",
            });
        }
        if !packet.freshness_slo.window_is_consistent() {
            violations.push(
                FinalizeQualificationViolation::FreshnessSloInconsistent {
                    surface_id: surface.surface_id.clone(),
                },
            );
        }
        // A present packet must be a real captured packet within the SLO window — `missing`
        // (no capture) is expressed by the absence of the whole packet, not by a degenerate
        // packet block.
        if packet.slo_state == FreshnessSloState::Missing || !packet.has_capture() {
            violations.push(
                FinalizeQualificationViolation::PacketPresentWithoutCapture {
                    surface_id: surface.surface_id.clone(),
                },
            );
        }
    }

    fn validate_state_reason_coherence(
        &self,
        surface: &FinalizeOptionalSurface,
        violations: &mut Vec<FinalizeQualificationViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<FinalizeQualificationViolation>,
                               expected: FinalizeNarrowReason| {
            violations.push(
                FinalizeQualificationViolation::StateReasonIncoherent {
                    surface_id: surface.surface_id.clone(),
                    state: surface.surface_state,
                    expected_reason: expected,
                },
            );
        };

        match surface.surface_state {
            FinalizeSurfaceState::NarrowedNoPacket => {
                if !surface.has_active_reason(FinalizeNarrowReason::QualificationPacketAbsent) {
                    push_incoherent(violations, FinalizeNarrowReason::QualificationPacketAbsent);
                }
                if surface.has_packet() {
                    violations.push(
                        FinalizeQualificationViolation::NoPacketStateWithPacket {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
            }
            FinalizeSurfaceState::NarrowedIncomplete => {
                const ALLOWED: [FinalizeNarrowReason; 3] = [
                    FinalizeNarrowReason::SurfaceCapabilityAbsent,
                    FinalizeNarrowReason::SurfaceEvidenceIncomplete,
                    FinalizeNarrowReason::OwnerSignoffMissing,
                ];
                if !ALLOWED.iter().any(|r| surface.has_active_reason(*r)) {
                    push_incoherent(violations, FinalizeNarrowReason::SurfaceEvidenceIncomplete);
                }
            }
            FinalizeSurfaceState::NarrowedStale => {
                if !surface.has_active_reason(FinalizeNarrowReason::QualificationPacketBreached) {
                    push_incoherent(violations, FinalizeNarrowReason::QualificationPacketBreached);
                }
            }
            FinalizeSurfaceState::NarrowedClaimNarrowed => {
                if !surface.has_active_reason(FinalizeNarrowReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, FinalizeNarrowReason::ClaimLabelNarrowed);
                }
            }
            FinalizeSurfaceState::NarrowedWaiverExpired => {
                if !surface.has_active_reason(FinalizeNarrowReason::WaiverExpired) {
                    push_incoherent(violations, FinalizeNarrowReason::WaiverExpired);
                }
                if surface.waiver.is_none() {
                    violations.push(
                        FinalizeQualificationViolation::WaiverStateWithoutWaiver {
                            surface_id: surface.surface_id.clone(),
                            state: surface.surface_state,
                        },
                    );
                }
            }
            FinalizeSurfaceState::QualifiedOnWaiver => {
                if surface
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(
                        FinalizeQualificationViolation::WaiverStateWithoutWaiver {
                            surface_id: surface.surface_id.clone(),
                            state: surface.surface_state,
                        },
                    );
                }
            }
            FinalizeSurfaceState::QualifiedStable => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<FinalizeQualificationViolation>) {
        // Each surface ref appears at most once: a surface has one canonical register row.
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for surface in &self.surfaces {
            if !seen.insert(surface.surface_ref.as_str()) {
                violations.push(FinalizeQualificationViolation::DuplicateSurfaceRef {
                    surface_ref: surface.surface_ref.clone(),
                });
            }
        }

        // The release set must cover every declared release-relevant surface with exactly one
        // release-relevant row, and every release-relevant row must be declared, so a surface
        // cannot quietly drop out of the register.
        let declared: BTreeSet<&str> = self
            .release_relevant_surface_refs
            .iter()
            .map(String::as_str)
            .collect();
        let covered: BTreeSet<&str> = self
            .surfaces
            .iter()
            .filter(|surface| surface.release_relevant)
            .map(|surface| surface.surface_ref.as_str())
            .collect();
        for declared_ref in &declared {
            if !covered.contains(declared_ref) {
                violations.push(
                    FinalizeQualificationViolation::ReleaseRelevantRefWithoutSurface {
                        surface_ref: (*declared_ref).to_owned(),
                    },
                );
            }
        }
        for surface in &self.surfaces {
            if surface.release_relevant && !declared.contains(surface.surface_ref.as_str()) {
                violations.push(
                    FinalizeQualificationViolation::ReleaseRelevantSurfaceNotInSet {
                        surface_id: surface.surface_id.clone(),
                        surface_ref: surface.surface_ref.clone(),
                    },
                );
            }
        }

        // The register must cover all four surface kinds, so the release line cannot govern
        // some optional surfaces and silently leave a whole class ungoverned.
        for kind in FinalizeOptionalSurfaceKind::ALL {
            if self.surfaces_for_kind(kind).is_empty() {
                violations.push(FinalizeQualificationViolation::SurfaceKindAbsent { kind });
            }
        }

        // Every release-relevant surface must have deployment qualifications for all targets.
        for surface in &self.surfaces {
            if surface.release_relevant {
                let present: BTreeSet<DeploymentTarget> = surface
                    .deployment_qualifications
                    .iter()
                    .map(|dq| dq.deployment_target)
                    .collect();
                let missing: Vec<DeploymentTarget> = DeploymentTarget::ALL
                    .iter()
                    .filter(|t| !present.contains(t))
                    .copied()
                    .collect();
                if !missing.is_empty() {
                    violations.push(
                        FinalizeQualificationViolation::DeploymentCoverageIncomplete {
                            surface_id: surface.surface_id.clone(),
                            missing,
                        },
                    );
                }
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<FinalizeQualificationViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(FinalizeQualificationViolation::EmptyField {
                surface_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(FinalizeQualificationViolation::EmptyField {
                surface_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                FinalizeQualificationViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                FinalizeQualificationViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_surface_ids != self.computed_blocking_surface_ids() {
            violations.push(
                FinalizeQualificationViolation::PublicationBlockingSetMismatch {
                    field: "blocking_surface_ids",
                },
            );
        }
    }
}

/// A redaction-safe export row projected from the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSurfaceExportRow {
    /// Stable surface id.
    pub surface_id: String,
    /// Surface kind.
    pub surface_kind: FinalizeOptionalSurfaceKind,
    /// Surface ref.
    pub surface_ref: String,
    /// Whether the surface is part of the release-relevant set.
    pub release_relevant: bool,
    /// The public-claim entry ref the surface backs.
    pub claim_ref: String,
    /// The public claim's canonical ceiling label.
    pub claim_label: StableClaimLevel,
    /// Lifecycle label the surface displays.
    pub displayed_label: StableClaimLevel,
    /// Whether the surface renders a label at or above the cutline.
    pub renders_stable: bool,
    /// Surface state.
    pub surface_state: FinalizeSurfaceState,
    /// Whether the surface carries a stable qualification packet.
    pub has_packet: bool,
    /// Qualification-packet SLO state, or null when the surface has no packet.
    pub slo_state: Option<FreshnessSloState>,
    /// Active narrow reasons.
    pub active_narrow_reasons: Vec<FinalizeNarrowReason>,
    /// Per-deployment-target qualification records.
    pub deployment_qualifications: Vec<DeploymentQualification>,
}

/// A redaction-safe export projection of the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSurfaceExportProjection {
    /// Register id this projection was produced from.
    pub register_id: String,
    /// Register as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Projected surfaces.
    pub surfaces: Vec<FinalizeSurfaceExportRow>,
}

/// A validation violation for the optional-surface qualification register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinalizeQualificationViolation {
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
        /// Surface, rule, or section id.
        surface_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A surface id appears more than once.
    DuplicateSurfaceId {
        /// Duplicate surface id.
        surface_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A stop rule names no labels to watch.
    RuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A narrow reason has no rule watching for it.
    ReasonWithoutRule {
        /// Uncovered reason.
        reason: FinalizeNarrowReason,
    },
    /// A displayed label is wider than the public claim's canonical label.
    DisplayedWiderThanClaim {
        /// Surface id.
        surface_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
        /// Displayed label.
        displayed: StableClaimLevel,
    },
    /// A freshness SLO's warn window exceeds its target age.
    FreshnessSloInconsistent {
        /// Surface id.
        surface_id: String,
    },
    /// A present packet has no capture or carries the degenerate `missing` SLO state.
    PacketPresentWithoutCapture {
        /// Surface id.
        surface_id: String,
    },
    /// A surface renders qualified while it carries no stable qualification packet.
    QualifiedWithoutPacket {
        /// Surface id.
        surface_id: String,
    },
    /// A surface with no packet does not name the absent-packet reason.
    AbsentPacketWithoutReason {
        /// Surface id.
        surface_id: String,
    },
    /// A surface that carries a packet names the absent-packet reason.
    PacketPresentButReasonAbsent {
        /// Surface id.
        surface_id: String,
    },
    /// A `narrowed_no_packet` state carries a packet.
    NoPacketStateWithPacket {
        /// Surface id.
        surface_id: String,
    },
    /// A surface renders qualified while the public claim's canonical label is narrowed.
    QualifiedOnNarrowedClaim {
        /// Surface id.
        surface_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
    },
    /// A surface whose claim is narrowed does not carry the claim-narrowed reason.
    ClaimNarrowedWithoutReason {
        /// Surface id.
        surface_id: String,
    },
    /// A narrowing state did not drop the displayed label below the cutline.
    DisplayedLabelNotNarrowed {
        /// Surface id.
        surface_id: String,
        /// Surface state.
        state: FinalizeSurfaceState,
        /// Displayed label.
        displayed: StableClaimLevel,
    },
    /// A narrowing state carries no active narrow reason.
    NarrowingWithoutReason {
        /// Surface id.
        surface_id: String,
        /// Surface state.
        state: FinalizeSurfaceState,
    },
    /// A qualified surface's displayed label is not equal to its claim ceiling label.
    QualifiedLabelNotEqualClaim {
        /// Surface id.
        surface_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
        /// Displayed label.
        displayed: StableClaimLevel,
    },
    /// A qualified surface carries an active narrow reason.
    QualifiedWithActiveReason {
        /// Surface id.
        surface_id: String,
    },
    /// A qualified surface rides a qualification packet with no capture or evidence.
    QualifiedWithoutFreshPacket {
        /// Surface id.
        surface_id: String,
    },
    /// A qualified surface rides a qualification packet outside its SLO.
    QualifiedOnStalePacket {
        /// Surface id.
        surface_id: String,
        /// The packet's freshness-SLO state.
        slo_state: FreshnessSloState,
    },
    /// A qualified surface has no owner sign-off.
    QualifiedWithoutSignoff {
        /// Surface id.
        surface_id: String,
    },
    /// A narrowing surface with a breached packet does not name the breach reason.
    BreachedPacketWithoutReason {
        /// Surface id.
        surface_id: String,
    },
    /// A surface state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Surface id.
        surface_id: String,
        /// Surface state.
        state: FinalizeSurfaceState,
        /// Reason the state requires.
        expected_reason: FinalizeNarrowReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Surface id.
        surface_id: String,
        /// Surface state.
        state: FinalizeSurfaceState,
    },
    /// A surface ref appears on more than one row.
    DuplicateSurfaceRef {
        /// Duplicate surface ref.
        surface_ref: String,
    },
    /// A declared release-relevant surface ref has no covering row.
    ReleaseRelevantRefWithoutSurface {
        /// Uncovered surface ref.
        surface_ref: String,
    },
    /// A release-relevant row's surface ref is not in the declared set.
    ReleaseRelevantSurfaceNotInSet {
        /// Surface id.
        surface_id: String,
        /// The row's surface ref.
        surface_ref: String,
    },
    /// A surface kind is not covered by any row.
    SurfaceKindAbsent {
        /// The uncovered surface kind.
        kind: FinalizeOptionalSurfaceKind,
    },
    /// The declared publication decision disagrees with the computed one.
    PublicationDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// The declared publication blocking set disagrees with the computed one.
    PublicationBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A release-relevant surface does not have deployment qualifications for all targets.
    DeploymentCoverageIncomplete {
        /// Surface id.
        surface_id: String,
        /// Missing deployment targets.
        missing: Vec<DeploymentTarget>,
    },
    /// A deployment row renders stable while the surface is narrowed below the cutline.
    DeploymentStableOnNarrowedSurface {
        /// Surface id.
        surface_id: String,
        /// Deployment target.
        deployment_target: DeploymentTarget,
    },
    /// A deployment row renders stable while the surface has no qualification packet.
    DeploymentStableWithoutPacket {
        /// Surface id.
        surface_id: String,
        /// Deployment target.
        deployment_target: DeploymentTarget,
    },
    /// The summary counts disagree with the surfaces.
    SummaryMismatch,
}

impl fmt::Display for FinalizeQualificationViolation {
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
                surface_id,
                field_name,
            } => write!(f, "{surface_id} has empty field {field_name}"),
            Self::DuplicateSurfaceId { surface_id } => {
                write!(f, "duplicate surface id {surface_id}")
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate stop rule id {rule_id}")
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::ReasonWithoutRule { reason } => {
                write!(f, "narrow reason {} has no rule watching for it", reason.as_str())
            }
            Self::DisplayedWiderThanClaim {
                surface_id,
                claim,
                displayed,
            } => write!(
                f,
                "surface {surface_id} displayed label {} is wider than the claim ceiling {}",
                displayed.as_str(),
                claim.as_str()
            ),
            Self::FreshnessSloInconsistent { surface_id } => write!(
                f,
                "surface {surface_id} freshness SLO warn window exceeds its target age"
            ),
            Self::PacketPresentWithoutCapture { surface_id } => write!(
                f,
                "surface {surface_id} carries a packet block with no capture; express an absent packet by omitting the packet entirely"
            ),
            Self::QualifiedWithoutPacket { surface_id } => write!(
                f,
                "surface {surface_id} renders qualified while it has no stable qualification packet"
            ),
            Self::AbsentPacketWithoutReason { surface_id } => write!(
                f,
                "surface {surface_id} has no qualification packet but does not name qualification_packet_absent"
            ),
            Self::PacketPresentButReasonAbsent { surface_id } => write!(
                f,
                "surface {surface_id} carries a packet but names qualification_packet_absent"
            ),
            Self::NoPacketStateWithPacket { surface_id } => write!(
                f,
                "surface {surface_id} is narrowed_no_packet but carries a qualification packet"
            ),
            Self::QualifiedOnNarrowedClaim { surface_id, claim } => write!(
                f,
                "surface {surface_id} renders qualified while the public claim label {} is below the cutline",
                claim.as_str()
            ),
            Self::ClaimNarrowedWithoutReason { surface_id } => write!(
                f,
                "surface {surface_id} backs a claim that is narrowed but does not name claim_label_narrowed"
            ),
            Self::DisplayedLabelNotNarrowed {
                surface_id,
                state,
                displayed,
            } => write!(
                f,
                "surface {surface_id} state {} must narrow below the cutline but displays {}",
                state.as_str(),
                displayed.as_str()
            ),
            Self::NarrowingWithoutReason { surface_id, state } => write!(
                f,
                "surface {surface_id} state {} narrows without naming an active narrow reason",
                state.as_str()
            ),
            Self::QualifiedLabelNotEqualClaim {
                surface_id,
                claim,
                displayed,
            } => write!(
                f,
                "surface {surface_id} displays {} but its public claim label is {}",
                displayed.as_str(),
                claim.as_str()
            ),
            Self::QualifiedWithActiveReason { surface_id } => write!(
                f,
                "surface {surface_id} renders qualified while a narrow reason is active"
            ),
            Self::QualifiedWithoutFreshPacket { surface_id } => write!(
                f,
                "surface {surface_id} renders qualified with no captured, evidence-backed packet"
            ),
            Self::QualifiedOnStalePacket {
                surface_id,
                slo_state,
            } => write!(
                f,
                "surface {surface_id} renders qualified while its packet is {} (outside its freshness SLO)",
                slo_state.as_str()
            ),
            Self::QualifiedWithoutSignoff { surface_id } => {
                write!(f, "surface {surface_id} renders qualified without owner sign-off")
            }
            Self::BreachedPacketWithoutReason { surface_id } => write!(
                f,
                "surface {surface_id} has a breached packet but does not name qualification_packet_breached"
            ),
            Self::StateReasonIncoherent {
                surface_id,
                state,
                expected_reason,
            } => write!(
                f,
                "surface {surface_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { surface_id, state } => write!(
                f,
                "surface {surface_id} state {} names no waiver",
                state.as_str()
            ),
            Self::DuplicateSurfaceRef { surface_ref } => {
                write!(f, "duplicate surface ref {surface_ref}")
            }
            Self::ReleaseRelevantRefWithoutSurface { surface_ref } => write!(
                f,
                "declared release-relevant surface {surface_ref} has no covering row"
            ),
            Self::ReleaseRelevantSurfaceNotInSet {
                surface_id,
                surface_ref,
            } => write!(
                f,
                "surface {surface_id} is release-relevant but its surface {surface_ref} is not in release_relevant_surface_refs"
            ),
            Self::SurfaceKindAbsent { kind } => write!(
                f,
                "surface kind {} is not covered by any register row",
                kind.as_str()
            ),
            Self::PublicationDecisionInconsistent { declared, computed } => write!(
                f,
                "publication decision {} disagrees with computed {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication {field} disagrees with the firing rules")
            }
            Self::DeploymentCoverageIncomplete { surface_id, missing } => {
                let missing_str: Vec<&str> = missing.iter().map(|d| d.as_str()).collect();
                write!(f, "surface {surface_id} missing deployment qualifications for {}", missing_str.join(", "))
            }
            Self::DeploymentStableOnNarrowedSurface { surface_id, deployment_target } => {
                write!(f, "surface {surface_id} deployment row {} renders stable while the surface is narrowed", deployment_target.as_str())
            }
            Self::DeploymentStableWithoutPacket { surface_id, deployment_target } => {
                write!(f, "surface {surface_id} deployment row {} renders stable while the surface has no packet", deployment_target.as_str())
            }
            Self::SummaryMismatch => {
                write!(f, "register summary counts disagree with the surfaces")
            }
        }
    }
}

impl Error for FinalizeQualificationViolation {}

/// Loads the embedded optional-surface qualification register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`FinalizeQualificationPacketsForOptionalSurfacesAndEnforce`] — including when a surface carries a lifecycle label,
/// surface kind, surface state, freshness-SLO state, narrow reason, or narrow action outside
/// the closed vocabularies.
pub fn current_finalize_qualification_packets_for_optional_surfaces_and_enforce(
) -> Result<FinalizeQualificationPacketsForOptionalSurfacesAndEnforce, serde_json::Error> {
    serde_json::from_str(FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn register() -> FinalizeQualificationPacketsForOptionalSurfacesAndEnforce {
        current_finalize_qualification_packets_for_optional_surfaces_and_enforce().expect("register parses")
    }

    #[test]
    fn embedded_register_parses_and_validates() {
        let register = register();
        assert_eq!(
            register.schema_version,
            FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_SCHEMA_VERSION
        );
        assert_eq!(
            register.record_kind,
            FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_RECORD_KIND
        );
        assert_eq!(register.validate(), Vec::new());
        assert!(!register.surfaces.is_empty());
    }

    #[test]
    fn every_surface_kind_is_covered() {
        let register = register();
        for kind in FinalizeOptionalSurfaceKind::ALL {
            assert!(
                !register.surfaces_for_kind(kind).is_empty(),
                "surface kind {} must have at least one register row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn every_release_relevant_surface_is_covered() {
        let register = register();
        let covered: BTreeSet<&str> = register
            .surfaces
            .iter()
            .filter(|surface| surface.release_relevant)
            .map(|surface| surface.surface_ref.as_str())
            .collect();
        assert!(!register.release_relevant_surface_refs.is_empty());
        for declared in &register.release_relevant_surface_refs {
            assert!(
                covered.contains(declared.as_str()),
                "{declared} has no covering release-relevant row"
            );
        }
    }

    #[test]
    fn register_exercises_qualified_and_narrowed_surfaces() {
        let register = register();
        assert!(
            !register.surfaces_qualified_stable().is_empty(),
            "register must show at least one qualified-stable surface"
        );
        assert!(
            !register.surfaces_narrowed().is_empty(),
            "register must show at least one narrowed surface"
        );
    }

    #[test]
    fn register_exercises_a_surface_lacking_a_packet() {
        let register = register();
        let absent = register
            .surfaces
            .iter()
            .find(|surface| !surface.has_packet())
            .expect("register must exercise a surface lacking a qualification packet");
        assert_eq!(absent.surface_state, FinalizeSurfaceState::NarrowedNoPacket);
        assert!(!absent.renders_stable());
        assert!(absent.has_active_reason(FinalizeNarrowReason::QualificationPacketAbsent));
    }

    #[test]
    fn an_absent_packet_can_never_render_stable() {
        let register = register();
        for surface in &register.surfaces {
            if !surface.has_packet() {
                assert!(
                    !surface.renders_stable(),
                    "{} lacks a packet but renders at or above the cutline",
                    surface.surface_id
                );
            }
        }
    }

    #[test]
    fn summary_counts_match_surfaces() {
        let register = register();
        assert_eq!(register.summary, register.computed_summary());
        assert_eq!(
            register.summary.surfaces_qualified_stable
                + register.summary.surfaces_narrowed_below_cutline,
            register.surfaces.len()
        );
        assert_eq!(
            register.summary.surfaces_with_packet + register.summary.surfaces_without_packet,
            register.surfaces.len()
        );
        assert_eq!(
            register.summary.opt_in_capability_surfaces
                + register.summary.optional_integration_surfaces
                + register.summary.secondary_platform_surfaces
                + register.summary.experimental_preview_surfaces,
            register.surfaces.len()
        );
        assert_eq!(
            register.summary.packets_current
                + register.summary.packets_due_for_refresh
                + register.summary.packets_breached,
            register.summary.surfaces_with_packet
        );
    }

    #[test]
    fn publication_holds_when_a_blocking_rule_fires() {
        let register = register();
        assert_eq!(register.publication.decision, PromotionDecision::Hold);
        assert_eq!(
            register.publication.decision,
            register.computed_publication_decision()
        );
        assert!(!register.publication.blocking_rule_ids.is_empty());
        assert!(!register.publication.blocking_surface_ids.is_empty());
    }

    #[test]
    fn every_narrow_reason_has_a_rule() {
        let register = register();
        let covered: BTreeSet<FinalizeNarrowReason> = register
            .stop_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in FinalizeNarrowReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn no_surface_renders_wider_than_its_claim_ceiling() {
        let register = register();
        for surface in &register.surfaces {
            assert!(
                surface.displayed_label.rank() <= surface.claim_label.rank(),
                "{} renders wider than its ceiling",
                surface.surface_id
            );
        }
    }

    #[test]
    fn validate_flags_a_surface_wider_than_ceiling() {
        let mut register = register();
        let surface = register
            .surfaces
            .iter_mut()
            .find(|surface| {
                !surface.renders_stable() && surface.claim_label == StableClaimLevel::Beta
            })
            .expect("a narrowed surface under a beta ceiling exists");
        surface.displayed_label = StableClaimLevel::Stable;
        let surface_id = surface.surface_id.clone();
        register.summary = register.computed_summary();
        assert!(register.validate().iter().any(|v| matches!(
            v,
            FinalizeQualificationViolation::DisplayedWiderThanClaim { surface_id: id, .. } if *id == surface_id
        )));
    }

    #[test]
    fn validate_flags_an_absent_packet_rendered_qualified() {
        let mut register = register();
        let surface = register
            .surfaces
            .iter_mut()
            .find(|surface| !surface.has_packet())
            .expect("a surface lacking a packet exists");
        surface.surface_state = FinalizeSurfaceState::QualifiedStable;
        surface.displayed_label = surface.claim_label;
        surface.active_narrow_reasons.clear();
        register.summary = register.computed_summary();
        register.publication.decision = register.computed_publication_decision();
        register.publication.blocking_rule_ids = register.computed_blocking_rule_ids();
        register.publication.blocking_surface_ids = register.computed_blocking_surface_ids();
        assert!(register.validate().iter().any(|v| matches!(
            v,
            FinalizeQualificationViolation::QualifiedWithoutPacket { .. }
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut register = register();
        let surface = register
            .surfaces
            .iter_mut()
            .find(|surface| surface.surface_state == FinalizeSurfaceState::NarrowedStale)
            .expect("a narrowed-stale surface exists");
        surface.displayed_label = surface.claim_label;
        register.summary = register.computed_summary();
        register.publication.decision = register.computed_publication_decision();
        register.publication.blocking_rule_ids = register.computed_blocking_rule_ids();
        register.publication.blocking_surface_ids = register.computed_blocking_surface_ids();
        assert!(register.validate().iter().any(|v| matches!(
            v,
            FinalizeQualificationViolation::DisplayedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut register = register();
        register.publication.decision = PromotionDecision::Proceed;
        assert!(register.validate().iter().any(|v| matches!(
            v,
            FinalizeQualificationViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_qualified_surface_without_signoff() {
        let mut register = register();
        let surface = register
            .surfaces
            .iter_mut()
            .find(|surface| surface.renders_qualified())
            .expect("a qualified surface exists");
        surface.owner_signoff.signed_off = false;
        surface.owner_signoff.signed_at = None;
        let surface_id = surface.surface_id.clone();
        register.summary = register.computed_summary();
        assert!(register.validate().contains(
            &FinalizeQualificationViolation::QualifiedWithoutSignoff { surface_id }
        ));
    }

    #[test]
    fn export_projection_mirrors_surfaces() {
        let register = register();
        let projection = register.support_export_projection();
        assert_eq!(projection.surfaces.len(), register.surfaces.len());
        assert_eq!(
            projection.publication_decision,
            register.publication.decision
        );
        for (surface, projected) in register.surfaces.iter().zip(&projection.surfaces) {
            assert_eq!(surface.surface_id, projected.surface_id);
            assert_eq!(surface.surface_ref, projected.surface_ref);
            assert_eq!(surface.renders_stable(), projected.renders_stable);
            assert_eq!(surface.displayed_label, projected.displayed_label);
            assert_eq!(surface.has_packet(), projected.has_packet);
            assert_eq!(surface.slo_state(), projected.slo_state);
        }
    }
}
