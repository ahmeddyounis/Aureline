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
//! the cutline is to author and capture a packet. Each [`OptionalSurface`] is therefore one
//! `(optional surface, public claim)` binding whose qualification packet is an
//! [`Option<ProofPacket>`]: when it is `None`, the surface **lacks a stable qualification
//! packet** and is structurally required to narrow below the cutline; when it is `Some`, the
//! packet's freshness SLO, capture, and evidence decide whether the surface may render at the
//! public claim's label.
//!
//! Each [`OptionalSurface`]:
//!
//! - names the optional surface it governs ([`OptionalSurface::surface_kind`],
//!   [`OptionalSurface::surface_ref`], [`OptionalSurface::surface_summary`]) and whether it
//!   is part of the release-relevant surface set ([`OptionalSurface::release_relevant`]);
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose public
//!   claim it backs ([`OptionalSurface::claim_ref`]) and the canonical lifecycle label that
//!   entry publishes ([`OptionalSurface::claim_label`]). That label is a hard **ceiling**: a
//!   surface may render at the claim's label or narrow below it, but it may never display a
//!   maturity wider than the public claim it backs;
//! - carries its qualification packet as an [`Option<ProofPacket>`] with a packet-freshness
//!   SLO — `None` is the canonical "no stable qualification packet" state;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-surface labels, so
//!   docs, Help/About, the release center, and support exports ingest one label per surface
//!   instead of cloning their own;
//! - records the surface state earned ([`SurfaceState`]), the active narrow reasons
//!   ([`NarrowReason`]), and the label it *effectively* displays after narrowing
//!   ([`OptionalSurface::displayed_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary between a
//! surface that renders qualified for a Stable public claim and one narrowed below it. A
//! surface that is not qualified — because it has no qualification packet, because its packet
//! breached its freshness SLO, because its surface evidence or capability is incomplete,
//! because a waiver it relied on expired, or because the public claim it backs is itself
//! below the cutline — is structurally required to drop below the cutline rather than inherit
//! an adjacent qualified surface. The [`SurfaceStopRule`] set names the closed conditions
//! that gate promotion, and [`OptionalSurfaceQualification::publication`] records the
//! proceed/hold verdict.
//!
//! The register is checked in at `artifacts/release/optional_surface_qualification.json` and
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
pub const OPTIONAL_SURFACE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const OPTIONAL_SURFACE_QUALIFICATION_RECORD_KIND: &str = "optional_surface_qualification";

/// Repo-relative path to the checked-in register.
pub const OPTIONAL_SURFACE_QUALIFICATION_PATH: &str =
    "artifacts/release/optional_surface_qualification.json";

/// Embedded checked-in register JSON.
pub const OPTIONAL_SURFACE_QUALIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/optional_surface_qualification.json"
));

/// The class of optional surface a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OptionalSurfaceKind {
    /// An opt-in capability behind a feature toggle.
    OptInCapability,
    /// An optional third-party or external integration.
    OptionalIntegration,
    /// A secondary platform, deployment, or runtime target.
    SecondaryPlatform,
    /// A shipped-but-experimental preview surface.
    ExperimentalPreview,
}

impl OptionalSurfaceKind {
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

/// Surface state a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceState {
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

impl SurfaceState {
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
pub enum NarrowReason {
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

impl NarrowReason {
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
pub enum NarrowAction {
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

impl NarrowAction {
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
pub struct SurfaceStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrow reason whose presence on a watched surface fires this rule.
    pub trigger_reason: NarrowReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: NarrowAction,
    /// Whether firing this rule blocks stable promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One optional surface: a `(surface, public claim)` binding bound to its optional
/// qualification packet, canonical ceiling label, and packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionalSurface {
    /// Stable surface id.
    pub surface_id: String,
    /// Human-readable title.
    pub title: String,
    /// The class of optional surface this row governs.
    pub surface_kind: OptionalSurfaceKind,
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
    pub surface_state: SurfaceState,
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
    pub active_narrow_reasons: Vec<NarrowReason>,
    /// The lifecycle label the surface effectively displays after narrowing.
    pub displayed_label: StableClaimLevel,
    /// Publication destinations that render this surface's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the surface carries this posture.
    pub rationale: String,
}

impl OptionalSurface {
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
    pub fn has_active_reason(&self, reason: NarrowReason) -> bool {
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
pub struct SurfacePublicationRecord {
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
pub struct OptionalSurfaceQualificationSummary {
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
}

/// The typed optional-surface qualification register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionalSurfaceQualification {
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
    pub surface_kinds: Vec<OptionalSurfaceKind>,
    /// Closed surface-state vocabulary.
    pub surface_states: Vec<SurfaceState>,
    /// Closed narrow-reason vocabulary.
    pub narrow_reasons: Vec<NarrowReason>,
    /// Closed narrow-action vocabulary.
    pub narrow_actions: Vec<NarrowAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-relevant surface refs this register must cover.
    pub release_relevant_surface_refs: Vec<String>,
    /// Surface stop rules.
    pub stop_rules: Vec<SurfaceStopRule>,
    /// Optional surfaces.
    pub surfaces: Vec<OptionalSurface>,
    /// Recorded publication verdict.
    pub publication: SurfacePublicationRecord,
    /// Summary counts.
    pub summary: OptionalSurfaceQualificationSummary,
}

impl OptionalSurfaceQualification {
    /// Returns the surface registered for `surface_id`.
    pub fn surface(&self, surface_id: &str) -> Option<&OptionalSurface> {
        self.surfaces
            .iter()
            .find(|surface| surface.surface_id == surface_id)
    }

    /// Returns the surfaces rendering a label at or above the cutline.
    pub fn surfaces_qualified_stable(&self) -> Vec<&OptionalSurface> {
        self.surfaces
            .iter()
            .filter(|surface| surface.renders_stable())
            .collect()
    }

    /// Returns the surfaces narrowed below the cutline.
    pub fn surfaces_narrowed(&self) -> Vec<&OptionalSurface> {
        self.surfaces
            .iter()
            .filter(|surface| !surface.renders_stable())
            .collect()
    }

    /// Returns the surfaces lacking a stable qualification packet.
    pub fn surfaces_without_packet(&self) -> Vec<&OptionalSurface> {
        self.surfaces
            .iter()
            .filter(|surface| !surface.has_packet())
            .collect()
    }

    /// Returns the release-relevant surfaces.
    pub fn release_relevant_surfaces(&self) -> Vec<&OptionalSurface> {
        self.surfaces
            .iter()
            .filter(|surface| surface.release_relevant)
            .collect()
    }

    /// Returns the surfaces of one kind.
    pub fn surfaces_for_kind(&self, kind: OptionalSurfaceKind) -> Vec<&OptionalSurface> {
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
    pub fn stop_rule_fires(&self, rule: &SurfaceStopRule) -> bool {
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
        let blocking_triggers: BTreeSet<NarrowReason> = self
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
    pub fn computed_summary(&self) -> OptionalSurfaceQualificationSummary {
        let packets = |state: FreshnessSloState| {
            self.surfaces
                .iter()
                .filter(|surface| surface.slo_state() == Some(state))
                .count()
        };
        let kind = |kind: OptionalSurfaceKind| self.surfaces_for_kind(kind).len();
        let release_relevant: Vec<&OptionalSurface> = self
            .surfaces
            .iter()
            .filter(|surface| surface.release_relevant)
            .collect();
        OptionalSurfaceQualificationSummary {
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
                .filter(|surface| surface.surface_state == SurfaceState::QualifiedOnWaiver)
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
            opt_in_capability_surfaces: kind(OptionalSurfaceKind::OptInCapability),
            optional_integration_surfaces: kind(OptionalSurfaceKind::OptionalIntegration),
            secondary_platform_surfaces: kind(OptionalSurfaceKind::SecondaryPlatform),
            experimental_preview_surfaces: kind(OptionalSurfaceKind::ExperimentalPreview),
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
        }
    }

    /// Produces an export/Help-About-safe projection of the register that downstream
    /// surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> SurfaceExportProjection {
        SurfaceExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            surfaces: self
                .surfaces
                .iter()
                .map(|surface| SurfaceExportRow {
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
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<OptionalSurfaceQualificationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for surface in &self.surfaces {
            if !seen.insert(surface.surface_id.clone()) {
                violations.push(OptionalSurfaceQualificationViolation::DuplicateSurfaceId {
                    surface_id: surface.surface_id.clone(),
                });
            }
            self.validate_surface(surface, &mut violations);
        }
        if self.surfaces.is_empty() {
            violations.push(OptionalSurfaceQualificationViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(OptionalSurfaceQualificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<OptionalSurfaceQualificationViolation>) {
        if self.schema_version != OPTIONAL_SURFACE_QUALIFICATION_SCHEMA_VERSION {
            violations.push(
                OptionalSurfaceQualificationViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != OPTIONAL_SURFACE_QUALIFICATION_RECORD_KIND {
            violations.push(
                OptionalSurfaceQualificationViolation::UnsupportedRecordKind {
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
                violations.push(OptionalSurfaceQualificationViolation::EmptyField {
                    surface_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(
                OptionalSurfaceQualificationViolation::ClosedVocabularyMismatch {
                    field: "lifecycle_labels",
                },
            );
        }
        if self.surface_kinds != OptionalSurfaceKind::ALL.to_vec() {
            violations.push(
                OptionalSurfaceQualificationViolation::ClosedVocabularyMismatch {
                    field: "surface_kinds",
                },
            );
        }
        if self.surface_states != SurfaceState::ALL.to_vec() {
            violations.push(
                OptionalSurfaceQualificationViolation::ClosedVocabularyMismatch {
                    field: "surface_states",
                },
            );
        }
        if self.narrow_reasons != NarrowReason::ALL.to_vec() {
            violations.push(
                OptionalSurfaceQualificationViolation::ClosedVocabularyMismatch {
                    field: "narrow_reasons",
                },
            );
        }
        if self.narrow_actions != NarrowAction::ALL.to_vec() {
            violations.push(
                OptionalSurfaceQualificationViolation::ClosedVocabularyMismatch {
                    field: "narrow_actions",
                },
            );
        }
        if self.release_relevant_surface_refs.is_empty() {
            violations.push(OptionalSurfaceQualificationViolation::EmptyField {
                surface_id: "<register>".to_owned(),
                field_name: "release_relevant_surface_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(
                OptionalSurfaceQualificationViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.cutline_level",
                },
            );
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(
                OptionalSurfaceQualificationViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.above_cutline_levels",
                },
            );
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(
                OptionalSurfaceQualificationViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.below_cutline_levels",
                },
            );
        }
        if cutline.description.trim().is_empty() {
            violations.push(OptionalSurfaceQualificationViolation::EmptyField {
                surface_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<OptionalSurfaceQualificationViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(OptionalSurfaceQualificationViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(OptionalSurfaceQualificationViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(OptionalSurfaceQualificationViolation::EmptyField {
                        surface_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(OptionalSurfaceQualificationViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        // Every narrow reason must have a rule, so a reason cannot fire without a
        // corresponding promotion gate.
        for reason in NarrowReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(OptionalSurfaceQualificationViolation::ReasonWithoutRule { reason });
            }
        }
    }

    fn validate_surface(
        &self,
        surface: &OptionalSurface,
        violations: &mut Vec<OptionalSurfaceQualificationViolation>,
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
                violations.push(OptionalSurfaceQualificationViolation::EmptyField {
                    surface_id: surface.surface_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no surface may render a label wider than the public claim's canonical
        // label.
        if surface.displayed_label.rank() > surface.claim_label.rank() {
            violations.push(
                OptionalSurfaceQualificationViolation::DisplayedWiderThanClaim {
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
                    OptionalSurfaceQualificationViolation::QualifiedOnNarrowedClaim {
                        surface_id: surface.surface_id.clone(),
                        claim: surface.claim_label,
                    },
                );
            }
            if !surface.has_active_reason(NarrowReason::ClaimLabelNarrowed) {
                violations.push(
                    OptionalSurfaceQualificationViolation::ClaimNarrowedWithoutReason {
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
                    OptionalSurfaceQualificationViolation::QualifiedWithoutPacket {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if !surface.has_active_reason(NarrowReason::QualificationPacketAbsent) {
                violations.push(
                    OptionalSurfaceQualificationViolation::AbsentPacketWithoutReason {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        } else if surface.has_active_reason(NarrowReason::QualificationPacketAbsent) {
            // A surface that *does* carry a packet may not claim the packet is absent.
            violations.push(
                OptionalSurfaceQualificationViolation::PacketPresentButReasonAbsent {
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
                    OptionalSurfaceQualificationViolation::QualifiedLabelNotEqualClaim {
                        surface_id: surface.surface_id.clone(),
                        claim: surface.claim_label,
                        displayed: surface.displayed_label,
                    },
                );
            }
            if !surface.active_narrow_reasons.is_empty() {
                violations.push(
                    OptionalSurfaceQualificationViolation::QualifiedWithActiveReason {
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
                            OptionalSurfaceQualificationViolation::QualifiedWithoutFreshPacket {
                                surface_id: surface.surface_id.clone(),
                            },
                        );
                    }
                    if !packet.slo_state.is_within_slo() {
                        violations.push(
                            OptionalSurfaceQualificationViolation::QualifiedOnStalePacket {
                                surface_id: surface.surface_id.clone(),
                                slo_state: packet.slo_state,
                            },
                        );
                    }
                }
            }
            if !(surface.owner_signoff.signed_off && surface.owner_signoff.signed_at.is_some()) {
                violations.push(
                    OptionalSurfaceQualificationViolation::QualifiedWithoutSignoff {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        } else {
            // A narrowing state must drop the displayed label below the cutline and name at
            // least one active reason.
            if surface.renders_stable() {
                violations.push(
                    OptionalSurfaceQualificationViolation::DisplayedLabelNotNarrowed {
                        surface_id: surface.surface_id.clone(),
                        state: surface.surface_state,
                        displayed: surface.displayed_label,
                    },
                );
            }
            if surface.active_narrow_reasons.is_empty() {
                violations.push(
                    OptionalSurfaceQualificationViolation::NarrowingWithoutReason {
                        surface_id: surface.surface_id.clone(),
                        state: surface.surface_state,
                    },
                );
            }
            // A narrowing surface whose packet is breached must name the matching freshness
            // reason, so the freshness automation stays honest.
            if slo_state == Some(FreshnessSloState::Breached)
                && !surface.has_active_reason(NarrowReason::QualificationPacketBreached)
            {
                violations.push(
                    OptionalSurfaceQualificationViolation::BreachedPacketWithoutReason {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        self.validate_state_reason_coherence(surface, violations);
    }

    fn validate_packet(
        &self,
        surface: &OptionalSurface,
        violations: &mut Vec<OptionalSurfaceQualificationViolation>,
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
                violations.push(OptionalSurfaceQualificationViolation::EmptyField {
                    surface_id: surface.surface_id.clone(),
                    field_name: field,
                });
            }
        }
        // The freshness SLO target must be a positive number of days and the warn window may
        // not exceed it.
        if packet.freshness_slo.target_max_age_days == 0 {
            violations.push(OptionalSurfaceQualificationViolation::EmptyField {
                surface_id: surface.surface_id.clone(),
                field_name: "qualification_packet.freshness_slo.target_max_age_days",
            });
        }
        if !packet.freshness_slo.window_is_consistent() {
            violations.push(
                OptionalSurfaceQualificationViolation::FreshnessSloInconsistent {
                    surface_id: surface.surface_id.clone(),
                },
            );
        }
        // A present packet must be a real captured packet within the SLO window — `missing`
        // (no capture) is expressed by the absence of the whole packet, not by a degenerate
        // packet block.
        if packet.slo_state == FreshnessSloState::Missing || !packet.has_capture() {
            violations.push(
                OptionalSurfaceQualificationViolation::PacketPresentWithoutCapture {
                    surface_id: surface.surface_id.clone(),
                },
            );
        }
    }

    fn validate_state_reason_coherence(
        &self,
        surface: &OptionalSurface,
        violations: &mut Vec<OptionalSurfaceQualificationViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<OptionalSurfaceQualificationViolation>,
                               expected: NarrowReason| {
            violations.push(
                OptionalSurfaceQualificationViolation::StateReasonIncoherent {
                    surface_id: surface.surface_id.clone(),
                    state: surface.surface_state,
                    expected_reason: expected,
                },
            );
        };

        match surface.surface_state {
            SurfaceState::NarrowedNoPacket => {
                if !surface.has_active_reason(NarrowReason::QualificationPacketAbsent) {
                    push_incoherent(violations, NarrowReason::QualificationPacketAbsent);
                }
                if surface.has_packet() {
                    violations.push(
                        OptionalSurfaceQualificationViolation::NoPacketStateWithPacket {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
            }
            SurfaceState::NarrowedIncomplete => {
                const ALLOWED: [NarrowReason; 3] = [
                    NarrowReason::SurfaceCapabilityAbsent,
                    NarrowReason::SurfaceEvidenceIncomplete,
                    NarrowReason::OwnerSignoffMissing,
                ];
                if !ALLOWED.iter().any(|r| surface.has_active_reason(*r)) {
                    push_incoherent(violations, NarrowReason::SurfaceEvidenceIncomplete);
                }
            }
            SurfaceState::NarrowedStale => {
                if !surface.has_active_reason(NarrowReason::QualificationPacketBreached) {
                    push_incoherent(violations, NarrowReason::QualificationPacketBreached);
                }
            }
            SurfaceState::NarrowedClaimNarrowed => {
                if !surface.has_active_reason(NarrowReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, NarrowReason::ClaimLabelNarrowed);
                }
            }
            SurfaceState::NarrowedWaiverExpired => {
                if !surface.has_active_reason(NarrowReason::WaiverExpired) {
                    push_incoherent(violations, NarrowReason::WaiverExpired);
                }
                if surface.waiver.is_none() {
                    violations.push(
                        OptionalSurfaceQualificationViolation::WaiverStateWithoutWaiver {
                            surface_id: surface.surface_id.clone(),
                            state: surface.surface_state,
                        },
                    );
                }
            }
            SurfaceState::QualifiedOnWaiver => {
                if surface
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(
                        OptionalSurfaceQualificationViolation::WaiverStateWithoutWaiver {
                            surface_id: surface.surface_id.clone(),
                            state: surface.surface_state,
                        },
                    );
                }
            }
            SurfaceState::QualifiedStable => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<OptionalSurfaceQualificationViolation>) {
        // Each surface ref appears at most once: a surface has one canonical register row.
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for surface in &self.surfaces {
            if !seen.insert(surface.surface_ref.as_str()) {
                violations.push(OptionalSurfaceQualificationViolation::DuplicateSurfaceRef {
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
                    OptionalSurfaceQualificationViolation::ReleaseRelevantRefWithoutSurface {
                        surface_ref: (*declared_ref).to_owned(),
                    },
                );
            }
        }
        for surface in &self.surfaces {
            if surface.release_relevant && !declared.contains(surface.surface_ref.as_str()) {
                violations.push(
                    OptionalSurfaceQualificationViolation::ReleaseRelevantSurfaceNotInSet {
                        surface_id: surface.surface_id.clone(),
                        surface_ref: surface.surface_ref.clone(),
                    },
                );
            }
        }

        // The register must cover all four surface kinds, so the release line cannot govern
        // some optional surfaces and silently leave a whole class ungoverned.
        for kind in OptionalSurfaceKind::ALL {
            if self.surfaces_for_kind(kind).is_empty() {
                violations.push(OptionalSurfaceQualificationViolation::SurfaceKindAbsent { kind });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<OptionalSurfaceQualificationViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(OptionalSurfaceQualificationViolation::EmptyField {
                surface_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(OptionalSurfaceQualificationViolation::EmptyField {
                surface_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                OptionalSurfaceQualificationViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                OptionalSurfaceQualificationViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_surface_ids != self.computed_blocking_surface_ids() {
            violations.push(
                OptionalSurfaceQualificationViolation::PublicationBlockingSetMismatch {
                    field: "blocking_surface_ids",
                },
            );
        }
    }
}

/// A redaction-safe export row projected from the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceExportRow {
    /// Stable surface id.
    pub surface_id: String,
    /// Surface kind.
    pub surface_kind: OptionalSurfaceKind,
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
    pub surface_state: SurfaceState,
    /// Whether the surface carries a stable qualification packet.
    pub has_packet: bool,
    /// Qualification-packet SLO state, or null when the surface has no packet.
    pub slo_state: Option<FreshnessSloState>,
    /// Active narrow reasons.
    pub active_narrow_reasons: Vec<NarrowReason>,
}

/// A redaction-safe export projection of the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceExportProjection {
    /// Register id this projection was produced from.
    pub register_id: String,
    /// Register as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Projected surfaces.
    pub surfaces: Vec<SurfaceExportRow>,
}

/// A validation violation for the optional-surface qualification register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionalSurfaceQualificationViolation {
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
        reason: NarrowReason,
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
        state: SurfaceState,
        /// Displayed label.
        displayed: StableClaimLevel,
    },
    /// A narrowing state carries no active narrow reason.
    NarrowingWithoutReason {
        /// Surface id.
        surface_id: String,
        /// Surface state.
        state: SurfaceState,
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
        state: SurfaceState,
        /// Reason the state requires.
        expected_reason: NarrowReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Surface id.
        surface_id: String,
        /// Surface state.
        state: SurfaceState,
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
        kind: OptionalSurfaceKind,
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
    /// The summary counts disagree with the surfaces.
    SummaryMismatch,
}

impl fmt::Display for OptionalSurfaceQualificationViolation {
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
            Self::SummaryMismatch => {
                write!(f, "register summary counts disagree with the surfaces")
            }
        }
    }
}

impl Error for OptionalSurfaceQualificationViolation {}

/// Loads the embedded optional-surface qualification register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`OptionalSurfaceQualification`] — including when a surface carries a lifecycle label,
/// surface kind, surface state, freshness-SLO state, narrow reason, or narrow action outside
/// the closed vocabularies.
pub fn current_optional_surface_qualification(
) -> Result<OptionalSurfaceQualification, serde_json::Error> {
    serde_json::from_str(OPTIONAL_SURFACE_QUALIFICATION_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn register() -> OptionalSurfaceQualification {
        current_optional_surface_qualification().expect("register parses")
    }

    #[test]
    fn embedded_register_parses_and_validates() {
        let register = register();
        assert_eq!(
            register.schema_version,
            OPTIONAL_SURFACE_QUALIFICATION_SCHEMA_VERSION
        );
        assert_eq!(
            register.record_kind,
            OPTIONAL_SURFACE_QUALIFICATION_RECORD_KIND
        );
        assert_eq!(register.validate(), Vec::new());
        assert!(!register.surfaces.is_empty());
    }

    #[test]
    fn every_surface_kind_is_covered() {
        let register = register();
        for kind in OptionalSurfaceKind::ALL {
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
        assert_eq!(absent.surface_state, SurfaceState::NarrowedNoPacket);
        assert!(!absent.renders_stable());
        assert!(absent.has_active_reason(NarrowReason::QualificationPacketAbsent));
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
        let covered: BTreeSet<NarrowReason> = register
            .stop_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in NarrowReason::ALL {
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
            OptionalSurfaceQualificationViolation::DisplayedWiderThanClaim { surface_id: id, .. } if *id == surface_id
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
        surface.surface_state = SurfaceState::QualifiedStable;
        surface.displayed_label = surface.claim_label;
        surface.active_narrow_reasons.clear();
        register.summary = register.computed_summary();
        register.publication.decision = register.computed_publication_decision();
        register.publication.blocking_rule_ids = register.computed_blocking_rule_ids();
        register.publication.blocking_surface_ids = register.computed_blocking_surface_ids();
        assert!(register.validate().iter().any(|v| matches!(
            v,
            OptionalSurfaceQualificationViolation::QualifiedWithoutPacket { .. }
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut register = register();
        let surface = register
            .surfaces
            .iter_mut()
            .find(|surface| surface.surface_state == SurfaceState::NarrowedStale)
            .expect("a narrowed-stale surface exists");
        surface.displayed_label = surface.claim_label;
        register.summary = register.computed_summary();
        register.publication.decision = register.computed_publication_decision();
        register.publication.blocking_rule_ids = register.computed_blocking_rule_ids();
        register.publication.blocking_surface_ids = register.computed_blocking_surface_ids();
        assert!(register.validate().iter().any(|v| matches!(
            v,
            OptionalSurfaceQualificationViolation::DisplayedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut register = register();
        register.publication.decision = PromotionDecision::Proceed;
        assert!(register.validate().iter().any(|v| matches!(
            v,
            OptionalSurfaceQualificationViolation::PublicationDecisionInconsistent { .. }
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
            &OptionalSurfaceQualificationViolation::QualifiedWithoutSignoff { surface_id }
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
