//! Help/About, release-center, diagnostics, and procurement commercial-boundary
//! cards with open-versus-paid truth and residual-dependency disclosure.
//!
//! This module is the canonical commercial-boundary-card object. Where the
//! sibling [`crate::m5_commercial_control_plane`] freezes the per-lane fail
//! posture and the managed-state vocabulary, and
//! [`crate::m5_offboarding_cards`] freezes the humane offboarding surface, this
//! module freezes the *commercial-boundary surface* a user, admin, or
//! procurement reviewer sees on Help/About, the release center, diagnostics, and
//! in a procurement/support packet: which capabilities are local and open-source,
//! which are optional managed/paid lanes, what residual dependencies remain
//! vendor-hosted, which deployment profiles a boundary statement actually holds
//! in, and what procurement/support evidence is available.
//!
//! The object freezes one [`BoundaryCard`] for the local open core plus one per
//! managed [`ServiceFamily`], and one [`BoundarySurfaceBinding`] per consumer
//! surface. It reuses the closed vocabularies the control-plane matrix already
//! froze — [`ServiceFamily`], [`MarketedClaim`], [`ExportGuarantee`],
//! [`ScopeOwner`], and [`PostureOrigin`] — rather than minting a parallel synonym
//! set. The new tokens are only the commercial-boundary vocabulary the matrix did
//! not carry: the open-versus-paid boundary class, the residual-dependency class
//! and its deployment-profile honesty, the deployment-profile qualifier, the
//! procurement/support packet kind, the boundary-evidence status, and the
//! boundary-action kind.
//!
//! Five invariants keep the cards honest. First, **the local core is never
//! blocked**: every card — local or managed — carries a non-empty
//! [`BoundaryCard::local_safe_baseline`], so a stale or unreachable metering or
//! rating path narrows only an optional managed action, never local editing,
//! search, Git, or already-authorized local automation. Second, **export,
//! support, and local continuation outrank upsell**: a card's actions are ranked
//! and no learn-about-paid action may rank above an export, procurement, or
//! continue-local action. Third, **no open boundary is overstated**: every
//! residual dependency declares whether it [`ResidualDependency::remains_vendor_hosted`]
//! and whether it is [`ResidualDependency::eliminated_under_self_host`], and every
//! card names the [`DeploymentProfile`]s its boundary statement holds in, so the
//! surface never implies a stronger self-hosted or open boundary than the running
//! lane supports. Fourth, **no number crosses the boundary bare**: boundary cards
//! disclose the open-versus-paid posture, not spend, and defer every quota/spend
//! figure to the usage, forecast, and chargeback surfaces; each card carries an
//! as-of time so a future bound figure is never shown without one. Fifth, **the
//! marketed claim narrows from the evidence status**: a card's effective claim is
//! recomputed from its declared claim capped by its
//! [`BoundaryEvidenceStatus`], so missing, stale, or downgraded boundary evidence
//! narrows the marketed/support claim automatically rather than leaving it an
//! optimistic constant; the stored value must equal that recomputation or
//! validation fails.
//!
//! [`BoundaryCardSet::cross_check_against_control_plane`] confirms each managed
//! card agrees with the control-plane lane for its service family on the declared
//! claim, the export guarantee, and a non-empty local-safe baseline, so the cards
//! project the matrix rather than a parallel spreadsheet. Procurement and support
//! packets bind the *same* [`ProcurementSupportEvidence`] object, so a buyer and a
//! support engineer read one vocabulary and one object model.
//!
//! [`canonical_commercial_boundary_card_set`] builds the frozen set and
//! [`current_stable_commercial_boundary_card_set`] reads and validates the
//! checked-in packet at
//! [`artifacts/service/m5-commercial-boundary-cards.json`](../../../../artifacts/service/m5-commercial-boundary-cards.json),
//! so Help/About, the release center, diagnostics, the procurement/support
//! packet, and claim/public-truth automation all ingest one packet rather than
//! cloning status text. The boundary schema is
//! [`schemas/service/m5-commercial-boundary-cards.schema.json`](../../../../schemas/service/m5-commercial-boundary-cards.schema.json).

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_commercial_control_plane::{
    canonical_source_refs, canonical_stable_commercial_control_plane_matrix, ExportGuarantee,
    MarketedClaim, PostureOrigin, ServiceFamily,
};

#[cfg(test)]
mod tests;

/// Supported schema version for the commercial-boundary-card set.
pub const COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the card-set packet.
pub const CARD_SET_RECORD_KIND: &str = "m5_commercial_boundary_card_set";

/// Stable record-kind tag for a single boundary card.
pub const CARD_RECORD_KIND: &str = "m5_commercial_boundary_card";

/// Stable record-kind tag for a residual-dependency disclosure.
pub const RESIDUAL_DEPENDENCY_RECORD_KIND: &str = "m5_commercial_boundary_residual_dependency";

/// Stable record-kind tag for a deployment-profile qualifier.
pub const PROFILE_QUALIFIER_RECORD_KIND: &str = "m5_commercial_boundary_profile_qualifier";

/// Stable record-kind tag for a procurement/support evidence block.
pub const PROCUREMENT_EVIDENCE_RECORD_KIND: &str = "m5_commercial_boundary_procurement_evidence";

/// Stable record-kind tag for a boundary card action.
pub const CARD_ACTION_RECORD_KIND: &str = "m5_commercial_boundary_card_action";

/// Stable record-kind tag for a surface binding.
pub const SURFACE_BINDING_RECORD_KIND: &str = "m5_commercial_boundary_surface_binding";

/// Stable record-kind tag for the card-set inspection block.
pub const INSPECTION_RECORD_KIND: &str = "m5_commercial_boundary_card_inspection";

/// Repo-relative path to the boundary schema.
pub const COMMERCIAL_BOUNDARY_CARDS_SCHEMA_REF: &str =
    "schemas/service/m5-commercial-boundary-cards.schema.json";

/// Repo-relative path to the reviewer contract.
pub const COMMERCIAL_BOUNDARY_CARDS_DOC_REF: &str =
    "docs/m5/ship-help-about-release-center-diagnostics-commercial-boundary-cards-with-open-versus-paid-truth-residual-dependency-disclosure-and-procurement-support-packet-parity.md";

/// Repo-relative path to the checked-in card-set packet.
pub const COMMERCIAL_BOUNDARY_CARDS_ARTIFACT_PATH: &str =
    "artifacts/service/m5-commercial-boundary-cards.json";

/// The open-versus-paid class a commercial-boundary card declares.
///
/// A card is either the local open-source core — fully usable and free, with no
/// managed dependency — or an optional managed/paid lane whose local-safe
/// baseline always continues when the lane is off. The two classes never blur:
/// the local-open card makes no managed claim, and a managed card always names a
/// non-empty local-safe baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryClass {
    /// The local, open-source core — free, no managed dependency.
    LocalOpenSource,
    /// An optional managed/paid lane; the local-safe baseline continues without it.
    ManagedPaidOptional,
}

impl BoundaryClass {
    /// The marketed claim a card of this class declares before any narrowing.
    ///
    /// The local open core makes only the local-safe claim; a managed lane
    /// declares the full managed claim and narrows from there.
    pub const fn declared_claim(self) -> MarketedClaim {
        match self {
            Self::LocalOpenSource => MarketedClaim::LocalSafeOnly,
            Self::ManagedPaidOptional => MarketedClaim::ManagedFull,
        }
    }

    /// True when this class names an optional managed/paid lane.
    pub const fn is_managed_paid(self) -> bool {
        matches!(self, Self::ManagedPaidOptional)
    }
}

/// Closed residual-dependency class vocabulary, re-exported from the
/// residual-dependency ledger
/// (`artifacts/governance/residual_dependencies.yaml`).
///
/// A residual dependency is a vendor-hosted or external dependency that a
/// managed lane carries; disclosing it honestly is what keeps the open boundary
/// from being overstated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyClass {
    /// Managed sign-in / identity.
    SignIn,
    /// A package registry.
    PackageRegistry,
    /// A remote mirror.
    RemoteMirror,
    /// A remote agent / workspace control plane.
    RemoteAgent,
    /// A symbol service.
    SymbolService,
    /// An AI provider.
    AiProvider,
    /// A signed policy bundle.
    PolicyBundle,
    /// A docs pack.
    DocsPack,
    /// A browser handoff bridge.
    BrowserHandoff,
    /// A companion notification channel.
    CompanionNotificationChannel,
    /// Reachability of the hosted control plane.
    HostedControlPlaneReachability,
}

/// Closed deployment-profile vocabulary, re-exported from the deployment-profile
/// register (`artifacts/governance/deployment_profiles.yaml`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfile {
    /// A single user running fully locally.
    IndividualLocal,
    /// A customer-operated, self-hosted control plane and storage.
    SelfHosted,
    /// An enterprise online deployment against the vendor control plane.
    EnterpriseOnline,
    /// An air-gapped deployment served by signed mirrors and offline bundles.
    AirGapped,
    /// The vendor-managed cloud.
    ManagedCloud,
}

impl DeploymentProfile {
    /// Every deployment profile, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::IndividualLocal,
        Self::SelfHosted,
        Self::EnterpriseOnline,
        Self::AirGapped,
        Self::ManagedCloud,
    ];
}

/// The freshness of the evidence backing a boundary card's marketed claim.
///
/// Boundary evidence is the claim-manifest and deployment-profile-truth proof a
/// card rides; when it goes stale, missing, or is downgraded, the card narrows
/// its marketed claim automatically rather than keeping an optimistic constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryEvidenceStatus {
    /// The backing evidence is current within its freshness window.
    Current,
    /// The backing evidence is stale; the managed claim narrows.
    Stale,
    /// The backing evidence is missing; the claim drops to the local-safe baseline.
    Missing,
    /// The backing claim was downgraded; it drops to the local-safe baseline.
    Downgraded,
}

impl BoundaryEvidenceStatus {
    /// Every evidence status, in declaration order.
    pub const ALL: [Self; 4] = [Self::Current, Self::Stale, Self::Missing, Self::Downgraded];

    /// The marketed-claim cap this status imposes on a card.
    ///
    /// Current imposes no cap; stale narrows to the reduced managed claim;
    /// missing and downgraded drop to the local-safe baseline.
    pub const fn claim_cap(self) -> MarketedClaim {
        match self {
            Self::Current => MarketedClaim::ManagedFull,
            Self::Stale => MarketedClaim::ManagedNarrowed,
            Self::Missing | Self::Downgraded => MarketedClaim::LocalSafeOnly,
        }
    }

    /// True when this status leaves the full managed claim intact.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// The kind of procurement/support evidence a boundary card links to.
///
/// Procurement and support packets reuse this same closed vocabulary, so a buyer
/// and a support engineer read one object model rather than parallel artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcurementPacketKind {
    /// The open-source license and component manifest.
    OpenSourceLicenseManifest,
    /// The residual-dependency disclosure packet.
    ResidualDependencyDisclosure,
    /// The deployment-profile truth packet.
    DeploymentProfileTruthPacket,
    /// The usage and forecast export.
    UsageAndForecastExport,
    /// The chargeback-scope export.
    ChargebackExport,
    /// The entitlement summary.
    EntitlementSummary,
    /// A support bundle.
    SupportBundle,
    /// The offboarding export.
    OffboardingExport,
}

/// The kind of action a commercial-boundary card offers.
///
/// Export, procurement, and local continuation always outrank upsell; a
/// [`LearnAboutPaid`](Self::LearnAboutPaid) action may never rank above an
/// [`ExportEvidence`](Self::ExportEvidence), [`ViewProcurementPacket`](Self::ViewProcurementPacket),
/// or [`ContinueLocal`](Self::ContinueLocal) action on the same card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryActionKind {
    /// Export the open-source/license and residual-dependency evidence now.
    ExportEvidence,
    /// Keep using the local open core; it is unaffected.
    ContinueLocal,
    /// View or assemble the procurement/support evidence packet.
    ViewProcurementPacket,
    /// Review the residual vendor-hosted dependencies.
    ViewResidualDependencies,
    /// Review which deployment profiles the boundary holds in.
    ViewDeploymentProfileTruth,
    /// Learn about (or buy) the optional managed/paid lane.
    LearnAboutPaid,
}

impl BoundaryActionKind {
    /// True when this action is an export, procurement, or local-continuation
    /// action that must never be outranked by an upsell prompt.
    pub const fn is_protected_priority(self) -> bool {
        matches!(
            self,
            Self::ExportEvidence | Self::ContinueLocal | Self::ViewProcurementPacket
        )
    }

    /// True when this action is an upsell / learn-about-paid prompt.
    pub const fn is_upsell_prompt(self) -> bool {
        matches!(self, Self::LearnAboutPaid)
    }
}

/// How a boundary card discloses any spend or quota figure.
///
/// Boundary cards disclose the open-versus-paid posture, not spend; every figure
/// is deferred to the usage, forecast, and chargeback surfaces, where it is bound
/// to its unit, as-of time, and scope owner. A card therefore shows no bare
/// number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CostFigureDisclosure {
    /// No spend or quota number is shown; numbers live on the metering surfaces.
    DeferredToMeteringSurfaces,
    /// A figure is shown bound to its unit, as-of time, and scope owner.
    BoundToUnitAsOfScope,
}

impl CostFigureDisclosure {
    /// True when a bound number is shown (never a bare one).
    pub const fn shows_number(self) -> bool {
        matches!(self, Self::BoundToUnitAsOfScope)
    }
}

/// The closed set of surfaces that project the commercial-boundary cards.
///
/// This extends the control-plane consumer surfaces with the two surfaces this
/// feature names that the matrix did not carry — the release center and the
/// procurement packet — while keeping the names aligned where they overlap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundarySurface {
    /// The Help/About truth surface.
    HelpAbout,
    /// The release center.
    ReleaseCenter,
    /// Diagnostics and service-health surfaces.
    Diagnostics,
    /// The procurement evidence packet.
    ProcurementPacket,
    /// The support and admin export packet.
    SupportAdminPacket,
    /// Claim and public-truth narrowing automation.
    ClaimPublicTruthAutomation,
}

impl BoundarySurface {
    /// Every surface the cards must reach.
    pub const ALL: [Self; 6] = [
        Self::HelpAbout,
        Self::ReleaseCenter,
        Self::Diagnostics,
        Self::ProcurementPacket,
        Self::SupportAdminPacket,
        Self::ClaimPublicTruthAutomation,
    ];
}

/// One residual vendor-hosted or external dependency a managed card discloses.
///
/// The disclosure is honest about the open boundary: it names whether the
/// dependency [`Self::remains_vendor_hosted`] in the vendor-managed profiles and
/// whether self-hosting [`Self::eliminated_under_self_host`] removes it, so the
/// card never implies a stronger self-hosted boundary than the lane supports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidualDependency {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// The residual-dependency class.
    pub dependency_class: DependencyClass,
    /// True when this dependency stays vendor-hosted in the managed/online profiles.
    pub remains_vendor_hosted: bool,
    /// True when self-hosting or a customer-operated mirror removes this dependency.
    pub eliminated_under_self_host: bool,
    /// Reviewable disclosure naming what remains vendor-hosted and how to localize it.
    pub disclosure: String,
}

impl ResidualDependency {
    /// Builds a residual-dependency disclosure.
    pub fn new(
        dependency_class: DependencyClass,
        remains_vendor_hosted: bool,
        eliminated_under_self_host: bool,
        disclosure: &str,
    ) -> Self {
        Self {
            record_kind: RESIDUAL_DEPENDENCY_RECORD_KIND.to_owned(),
            schema_version: COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION,
            dependency_class,
            remains_vendor_hosted,
            eliminated_under_self_host,
            disclosure: disclosure.to_owned(),
        }
    }
}

/// Which deployment profiles a card's boundary statement holds in, and what it
/// narrows to elsewhere.
///
/// Both lists draw from the frozen [`DeploymentProfile`] vocabulary, and at least
/// one profile must be named, so a card never implies its open/paid boundary
/// holds universally when it does not.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentProfileQualifier {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// The deployment profiles in which the card's boundary statement is accurate.
    pub holds_in_profiles: Vec<DeploymentProfile>,
    /// The deployment profiles in which the managed lane is not offered (boundary narrows).
    pub not_offered_in_profiles: Vec<DeploymentProfile>,
    /// Reviewable note describing the per-profile qualification.
    pub qualifier_note: String,
}

/// The procurement/support evidence a boundary card links to.
///
/// Procurement and support surfaces bind this same object, so the buyer-facing
/// and support-facing packets share one vocabulary and one object model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcurementSupportEvidence {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// The packet kinds this card makes available; never empty.
    pub packet_kinds: Vec<ProcurementPacketKind>,
    /// The bounded export guarantee for the card's evidence.
    pub export_guarantee: ExportGuarantee,
    /// Reviewable reference to the support/admin packet surface (never a raw URL).
    pub support_admin_packet_ref: String,
    /// Reviewable summary of what the procurement/support evidence proves.
    pub summary: String,
}

/// One ranked user-visible action a boundary card offers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryAction {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// The action kind.
    pub kind: BoundaryActionKind,
    /// Render rank; lower is higher priority (rendered first).
    pub rank: u32,
    /// The reviewable label the surface renders.
    pub label: String,
}

impl BoundaryAction {
    /// Builds a ranked action with the given kind and label.
    pub fn new(kind: BoundaryActionKind, rank: u32, label: &str) -> Self {
        Self {
            record_kind: CARD_ACTION_RECORD_KIND.to_owned(),
            schema_version: COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION,
            kind,
            rank,
            label: label.to_owned(),
        }
    }
}

/// One frozen commercial-boundary card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryCard {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable card identifier.
    pub card_id: String,
    /// Reviewable card title.
    pub title: String,
    /// Reviewable card summary.
    pub summary: String,
    /// The open-versus-paid class this card declares.
    pub boundary_class: BoundaryClass,
    /// The managed service family this card maps to; absent for the local open core.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_family: Option<ServiceFamily>,
    /// The reviewable open-versus-paid statement.
    pub open_paid_statement: String,
    /// The posture origin a narrowing is cited back to.
    pub posture_origin: PostureOrigin,
    /// The residual vendor-hosted dependencies; empty only for the local open core.
    pub residual_dependencies: Vec<ResidualDependency>,
    /// Which deployment profiles the boundary holds in.
    pub deployment_profile_qualifier: DeploymentProfileQualifier,
    /// The procurement/support evidence available for this card.
    pub procurement_support_evidence: ProcurementSupportEvidence,
    /// Non-empty local-safe baseline that always continues; the local core is never blocked.
    pub local_safe_baseline: Vec<String>,
    /// How any spend/quota figure is disclosed; deferred or bound, never bare.
    pub cost_figure_disclosure: CostFigureDisclosure,
    /// Last evidence as-of time; present even when no number is shown.
    pub as_of: String,
    /// The freshness of the evidence backing the marketed claim.
    pub evidence_status: BoundaryEvidenceStatus,
    /// The ranked actions; export, procurement, and local continuation outrank upsell.
    pub actions: Vec<BoundaryAction>,
    /// The marketed claim the card declares before narrowing.
    pub declared_marketed_claim: MarketedClaim,
    /// The marketed claim after the evidence-status cap is applied.
    pub effective_marketed_claim: MarketedClaim,
    /// Short recovery cue. Present (non-null) when the card is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_cue: Option<String>,
}

impl BoundaryCard {
    /// Returns the card's export action, the action that must never be buried.
    pub fn export_action(&self) -> Option<&BoundaryAction> {
        self.actions
            .iter()
            .find(|a| a.kind == BoundaryActionKind::ExportEvidence)
    }

    /// True when no upsell action ranks above an export, procurement, or
    /// continue-local action.
    pub fn upsell_never_outranks_truth(&self) -> bool {
        let protected_max = self
            .actions
            .iter()
            .filter(|a| a.kind.is_protected_priority())
            .map(|a| a.rank)
            .max();
        let Some(protected_max) = protected_max else {
            return false;
        };
        self.actions
            .iter()
            .filter(|a| a.kind.is_upsell_prompt())
            .all(|u| u.rank > protected_max)
    }

    /// True when this card's boundary statement holds in `profile`.
    pub fn holds_in(&self, profile: DeploymentProfile) -> bool {
        self.deployment_profile_qualifier
            .holds_in_profiles
            .contains(&profile)
    }
}

/// One surface bound to the boundary cards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundarySurfaceBinding {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable binding identifier.
    pub binding_id: String,
    /// The surface that projects the cards.
    pub surface: BoundarySurface,
    /// The card ids this surface resolves through.
    pub bound_card_ids: Vec<String>,
    /// Always true: the surface projects the effective claim, never a stronger one.
    pub projects_effective_claim: bool,
    /// Always true: the surface renders the local-safe baseline.
    pub renders_local_safe_baseline: bool,
    /// Always true: the surface discloses the residual dependencies.
    pub discloses_residual_dependencies: bool,
    /// Always true: the surface names the deployment-profile qualifier.
    pub names_deployment_profile_qualifier: bool,
    /// Always true: the surface keeps export/procurement above any upsell prompt.
    pub surfaces_evidence_before_upsell: bool,
    /// Reviewable summary of what the surface renders.
    pub summary: String,
}

/// Compact inspection block recomputed from the card set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryCardInspection {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Number of cards.
    pub card_count: usize,
    /// Number of surface bindings.
    pub surface_binding_count: usize,
    /// Number of local-open-core cards.
    pub local_open_card_count: usize,
    /// Number of managed/paid cards.
    pub managed_paid_card_count: usize,
    /// Number of distinct service families covered by managed cards.
    pub service_families_covered: usize,
    /// True when every managed service family carries a card.
    pub managed_lane_coverage_complete: bool,
    /// True when all six surfaces are bound.
    pub surface_coverage_complete: bool,
    /// True when every card keeps a non-empty local-safe baseline.
    pub all_cards_local_safe_backed: bool,
    /// True when every managed card discloses at least one residual dependency.
    pub all_managed_cards_disclose_residual: bool,
    /// True when every card names at least one deployment profile it holds in.
    pub all_cards_qualify_deployment_profile: bool,
    /// True when every card links non-empty procurement/support evidence.
    pub all_cards_link_procurement_evidence: bool,
    /// True when no card shows a bare number: every figure is deferred or bound, with an as-of time.
    pub value_never_bare: bool,
    /// True when no card lets an upsell prompt outrank export/procurement/local continuation.
    pub upsell_never_outranks_truth: bool,
    /// Number of cards still backing the full managed claim.
    pub effective_full_card_count: usize,
    /// Number of cards narrowed to a reduced managed claim.
    pub narrowed_card_count: usize,
    /// Number of cards at the local-safe-only claim.
    pub local_safe_only_card_count: usize,
}

/// The frozen commercial-boundary-card set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryCardSet {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable set identifier.
    pub set_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Integer revision for the set content.
    pub set_revision: u32,
    /// Reviewable set title.
    pub title: String,
    /// Reviewable set summary.
    pub summary: String,
    /// Source schema and contract refs the set cites.
    pub source_refs: Vec<String>,
    /// The boundary cards.
    pub cards: Vec<BoundaryCard>,
    /// The surface bindings.
    pub surface_bindings: Vec<BoundarySurfaceBinding>,
    /// The recomputed inspection block.
    pub inspection: BoundaryCardInspection,
}

impl BoundaryCardSet {
    /// Returns the card for `service_family`, when one is frozen.
    pub fn card_for_family(&self, family: ServiceFamily) -> Option<&BoundaryCard> {
        self.cards.iter().find(|c| c.service_family == Some(family))
    }

    /// Returns the local-open-core card, when one is frozen.
    pub fn local_open_card(&self) -> Option<&BoundaryCard> {
        self.cards
            .iter()
            .find(|c| c.boundary_class == BoundaryClass::LocalOpenSource)
    }

    /// Applies an evidence status to every managed card, narrowing its claim.
    ///
    /// Every [`BoundaryClass::ManagedPaidOptional`] card has its effective
    /// marketed claim recomputed from `status`'s
    /// [`BoundaryEvidenceStatus::claim_cap`], its recovery cue set, and the
    /// inspection recomputed. The local-safe baseline is never removed, so the
    /// local core stays available, and the local-open card never narrows.
    pub fn apply_evidence_status(&mut self, status: BoundaryEvidenceStatus) {
        for card in &mut self.cards {
            if !card.boundary_class.is_managed_paid() {
                continue;
            }
            card.evidence_status = status;
            let effective = weaker_claim(card.declared_marketed_claim, status.claim_cap());
            card.effective_marketed_claim = effective;
            card.recovery_cue = if effective == card.declared_marketed_claim {
                None
            } else {
                Some(evidence_recovery_cue(status))
            };
        }
        self.inspection = BoundaryCardInspection::derive(&self.cards, &self.surface_bindings);
    }

    /// Serializes the set as pretty JSON safe for the checked-in artifact and exports.
    ///
    /// # Panics
    ///
    /// Panics only if the set cannot be serialized, which a validated set never is.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("boundary card set serializes to JSON")
    }

    /// Validates the set and recomputes every derived value.
    ///
    /// Returns an empty vector when the set is internally consistent. Otherwise
    /// returns one [`BoundaryCardViolation`] per failed invariant: a wrong record
    /// kind or schema version, a missing identifier, a duplicate card, an
    /// incomplete managed-lane set, an empty local-safe baseline, a managed card
    /// that hides its residual dependencies, an unqualified deployment profile,
    /// empty procurement evidence, a bare number, a buried export/procurement
    /// action, an effective claim that does not match the evidence cap, a missing
    /// recovery cue on a narrowed card, an unbound surface, or a stale inspection
    /// block.
    pub fn validate(&self) -> Vec<BoundaryCardViolation> {
        let mut violations = Vec::new();
        let mut push = |field: &str, message: &str| {
            violations.push(BoundaryCardViolation {
                field: field.to_owned(),
                message: message.to_owned(),
            });
        };

        if self.record_kind != CARD_SET_RECORD_KIND {
            push("record_kind", "set record_kind is wrong");
        }
        if self.schema_version != COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION {
            push("schema_version", "set schema_version is wrong");
        }
        if self.set_id.trim().is_empty() {
            push("set_id", "set_id must be non-empty");
        }
        if self.generated_at.trim().is_empty() {
            push("generated_at", "generated_at must be non-empty");
        }
        if self.title.trim().is_empty() {
            push("title", "title must be non-empty");
        }
        if self.summary.trim().is_empty() {
            push("summary", "summary must be non-empty");
        }
        if self.set_revision == 0 {
            push("set_revision", "set_revision must be at least 1");
        }
        if !self
            .source_refs
            .iter()
            .any(|entry| entry == COMMERCIAL_BOUNDARY_CARDS_SCHEMA_REF)
        {
            push("source_refs", "set must cite its boundary schema");
        }
        if self.cards.is_empty() {
            push("cards", "set must contain at least one card");
        }

        let mut card_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut local_open_count = 0usize;
        for card in &self.cards {
            self.validate_card(card, &mut push);
            if !card_ids.insert(card.card_id.as_str()) {
                push("cards", "card_id values must be unique");
            }
            match card.service_family {
                Some(family) => {
                    if !seen_families.insert(family) {
                        push("cards", "each service family must carry at most one card");
                    }
                }
                None => local_open_count += 1,
            }
        }

        // Exactly one local-open-core card anchors the open side.
        if local_open_count != 1 {
            push(
                "cards",
                "the set must carry exactly one local-open-core card",
            );
        }
        // Every managed service family carries a boundary card.
        for family in ServiceFamily::ALL {
            if !self.cards.iter().any(|c| c.service_family == Some(family)) {
                push("cards", "every managed service family must carry a card");
                break;
            }
        }

        self.validate_surface_bindings(&mut push);

        let derived = BoundaryCardInspection::derive(&self.cards, &self.surface_bindings);
        if derived != self.inspection {
            push(
                "inspection",
                "stored inspection block does not match the recomputed set",
            );
        }

        violations
    }

    fn validate_card(&self, card: &BoundaryCard, push: &mut impl FnMut(&str, &str)) {
        if card.record_kind != CARD_RECORD_KIND {
            push("card.record_kind", "card record_kind is wrong");
        }
        if card.schema_version != COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION {
            push("card.schema_version", "card schema_version is wrong");
        }
        for (field, value) in [
            ("card.card_id", &card.card_id),
            ("card.title", &card.title),
            ("card.summary", &card.summary),
            ("card.open_paid_statement", &card.open_paid_statement),
            ("card.as_of", &card.as_of),
        ] {
            if value.trim().is_empty() {
                push(field, "value must be non-empty");
            }
        }

        // The local core is never blocked: every card keeps a non-empty baseline.
        if card.local_safe_baseline.is_empty()
            || card.local_safe_baseline.iter().any(|s| s.trim().is_empty())
        {
            push(
                "card.local_safe_baseline",
                "every card must keep a non-empty local-safe baseline",
            );
        }

        // The boundary class and service family stay consistent.
        match card.boundary_class {
            BoundaryClass::LocalOpenSource => {
                if card.service_family.is_some() {
                    push(
                        "card.service_family",
                        "the local-open-core card must not bind a managed service family",
                    );
                }
                // The open core carries no residual vendor-hosted dependency.
                if !card.residual_dependencies.is_empty() {
                    push(
                        "card.residual_dependencies",
                        "the local-open-core card must not declare a residual vendor dependency",
                    );
                }
            }
            BoundaryClass::ManagedPaidOptional => {
                if card.service_family.is_none() {
                    push(
                        "card.service_family",
                        "a managed/paid card must bind a managed service family",
                    );
                }
                // A managed card discloses at least one residual dependency honestly.
                if card.residual_dependencies.is_empty() {
                    push(
                        "card.residual_dependencies",
                        "a managed/paid card must disclose at least one residual dependency",
                    );
                }
            }
        }

        for dep in &card.residual_dependencies {
            self.validate_residual_dependency(dep, push);
        }
        self.validate_profile_qualifier(card, push);
        self.validate_procurement_evidence(card, push);
        self.validate_actions(card, push);
        self.validate_claim(card, push);

        // No number crosses the boundary bare: every card carries an as-of time.
        if card.as_of.trim().is_empty() {
            push(
                "card.as_of",
                "every card must carry an as-of time so a number is never shown without one",
            );
        }
    }

    fn validate_residual_dependency(
        &self,
        dep: &ResidualDependency,
        push: &mut impl FnMut(&str, &str),
    ) {
        if dep.record_kind != RESIDUAL_DEPENDENCY_RECORD_KIND {
            push(
                "card.residual_dependency.record_kind",
                "residual-dependency record_kind is wrong",
            );
        }
        if dep.schema_version != COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION {
            push(
                "card.residual_dependency.schema_version",
                "residual-dependency schema_version is wrong",
            );
        }
        if dep.disclosure.trim().is_empty() {
            push(
                "card.residual_dependency.disclosure",
                "a residual dependency must carry a reviewable disclosure",
            );
        }
        // The open boundary is never overstated: a dependency that is neither
        // vendor-hosted nor eliminable would be an empty disclosure.
        if !dep.remains_vendor_hosted && !dep.eliminated_under_self_host {
            push(
                "card.residual_dependency",
                "a residual dependency must state whether it stays vendor-hosted or is eliminated under self-host",
            );
        }
    }

    fn validate_profile_qualifier(&self, card: &BoundaryCard, push: &mut impl FnMut(&str, &str)) {
        let qualifier = &card.deployment_profile_qualifier;
        if qualifier.record_kind != PROFILE_QUALIFIER_RECORD_KIND {
            push(
                "card.deployment_profile_qualifier.record_kind",
                "profile-qualifier record_kind is wrong",
            );
        }
        if qualifier.schema_version != COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION {
            push(
                "card.deployment_profile_qualifier.schema_version",
                "profile-qualifier schema_version is wrong",
            );
        }
        // No open boundary is overstated: a card names the profiles it holds in.
        if qualifier.holds_in_profiles.is_empty() {
            push(
                "card.deployment_profile_qualifier.holds_in_profiles",
                "a card must name at least one deployment profile its boundary holds in",
            );
        }
        let mut seen = BTreeSet::new();
        for profile in &qualifier.holds_in_profiles {
            if !seen.insert(*profile) {
                push(
                    "card.deployment_profile_qualifier.holds_in_profiles",
                    "deployment profiles must be distinct",
                );
            }
        }
        // A profile is never both held-in and not-offered.
        for profile in &qualifier.not_offered_in_profiles {
            if qualifier.holds_in_profiles.contains(profile) {
                push(
                    "card.deployment_profile_qualifier.not_offered_in_profiles",
                    "a profile cannot be both held-in and not-offered",
                );
            }
        }
        if qualifier.qualifier_note.trim().is_empty() {
            push(
                "card.deployment_profile_qualifier.qualifier_note",
                "the profile qualifier must carry a note",
            );
        }
    }

    fn validate_procurement_evidence(
        &self,
        card: &BoundaryCard,
        push: &mut impl FnMut(&str, &str),
    ) {
        let evidence = &card.procurement_support_evidence;
        if evidence.record_kind != PROCUREMENT_EVIDENCE_RECORD_KIND {
            push(
                "card.procurement_support_evidence.record_kind",
                "procurement-evidence record_kind is wrong",
            );
        }
        if evidence.schema_version != COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION {
            push(
                "card.procurement_support_evidence.schema_version",
                "procurement-evidence schema_version is wrong",
            );
        }
        if evidence.packet_kinds.is_empty() {
            push(
                "card.procurement_support_evidence.packet_kinds",
                "a card must link at least one procurement/support packet kind",
            );
        }
        let mut seen = BTreeSet::new();
        for kind in &evidence.packet_kinds {
            if !seen.insert(*kind) {
                push(
                    "card.procurement_support_evidence.packet_kinds",
                    "packet kinds must be distinct",
                );
            }
        }
        if evidence.support_admin_packet_ref.trim().is_empty() {
            push(
                "card.procurement_support_evidence.support_admin_packet_ref",
                "the procurement/support evidence must name a packet ref",
            );
        }
        if evidence.summary.trim().is_empty() {
            push(
                "card.procurement_support_evidence.summary",
                "the procurement/support evidence must carry a summary",
            );
        }
    }

    fn validate_actions(&self, card: &BoundaryCard, push: &mut impl FnMut(&str, &str)) {
        let mut ranks = BTreeSet::new();
        let mut kinds = BTreeSet::new();
        for action in &card.actions {
            if action.record_kind != CARD_ACTION_RECORD_KIND {
                push("card.action.record_kind", "action record_kind is wrong");
            }
            if action.schema_version != COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION {
                push(
                    "card.action.schema_version",
                    "action schema_version is wrong",
                );
            }
            if action.label.trim().is_empty() {
                push("card.action.label", "action label must be non-empty");
            }
            if !ranks.insert(action.rank) {
                push("card.actions", "action ranks must be unique within a card");
            }
            kinds.insert(action.kind);
        }
        // Export, procurement, and local continuation are always offered.
        if !kinds.contains(&BoundaryActionKind::ExportEvidence) {
            push("card.actions", "every card must offer an export action");
        }
        if !kinds.contains(&BoundaryActionKind::ContinueLocal) {
            push(
                "card.actions",
                "every card must offer a local-continuation action",
            );
        }
        if !kinds.contains(&BoundaryActionKind::ViewProcurementPacket) {
            push(
                "card.actions",
                "every card must offer a procurement/support packet action",
            );
        }
        // Upsell never outranks export, procurement, or local continuation.
        if !card.upsell_never_outranks_truth() {
            push(
                "card.actions",
                "export, procurement, and local continuation must outrank any upsell prompt",
            );
        }
        // Only managed cards carry an upsell prompt; the open core never upsells itself.
        let has_upsell = kinds.contains(&BoundaryActionKind::LearnAboutPaid);
        if has_upsell && card.boundary_class == BoundaryClass::LocalOpenSource {
            push(
                "card.actions",
                "the local-open-core card must not carry an upsell prompt",
            );
        }
    }

    fn validate_claim(&self, card: &BoundaryCard, push: &mut impl FnMut(&str, &str)) {
        // The declared claim matches the boundary class.
        if card.declared_marketed_claim != card.boundary_class.declared_claim() {
            push(
                "card.declared_marketed_claim",
                "the declared claim must match the boundary class",
            );
        }
        // The effective claim is recomputed from the declared claim and the evidence cap.
        let expected = weaker_claim(
            card.declared_marketed_claim,
            card.evidence_status.claim_cap(),
        );
        if card.effective_marketed_claim != expected {
            push(
                "card.effective_marketed_claim",
                "the effective claim must equal the declared claim capped by the evidence status",
            );
        }
        // A narrowed card carries a recovery cue; a full (unnarrowed) card must not.
        let narrowed = expected != card.declared_marketed_claim;
        match (&card.recovery_cue, narrowed) {
            (None, true) => push(
                "card.recovery_cue",
                "a narrowed card must carry a recovery cue",
            ),
            (Some(cue), false) => {
                if !cue.trim().is_empty() {
                    push(
                        "card.recovery_cue",
                        "an unnarrowed card must not carry a recovery cue",
                    );
                }
            }
            (Some(cue), true) if cue.trim().is_empty() => {
                push("card.recovery_cue", "recovery cue must be non-empty");
            }
            _ => {}
        }
    }

    fn validate_surface_bindings(&self, push: &mut impl FnMut(&str, &str)) {
        let card_ids: BTreeSet<&str> = self.cards.iter().map(|c| c.card_id.as_str()).collect();
        let mut binding_ids = BTreeSet::new();
        for binding in &self.surface_bindings {
            if binding.record_kind != SURFACE_BINDING_RECORD_KIND {
                push(
                    "surface_binding.record_kind",
                    "binding record_kind is wrong",
                );
            }
            if binding.schema_version != COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION {
                push(
                    "surface_binding.schema_version",
                    "binding schema_version is wrong",
                );
            }
            if binding.binding_id.trim().is_empty() {
                push("surface_binding.binding_id", "binding_id must be non-empty");
            }
            if !binding_ids.insert(binding.binding_id.as_str()) {
                push("surface_bindings", "binding_id values must be unique");
            }
            if binding.summary.trim().is_empty() {
                push(
                    "surface_binding.summary",
                    "binding summary must be non-empty",
                );
            }
            for (flag, field) in [
                (binding.projects_effective_claim, "projects_effective_claim"),
                (
                    binding.renders_local_safe_baseline,
                    "renders_local_safe_baseline",
                ),
                (
                    binding.discloses_residual_dependencies,
                    "discloses_residual_dependencies",
                ),
                (
                    binding.names_deployment_profile_qualifier,
                    "names_deployment_profile_qualifier",
                ),
                (
                    binding.surfaces_evidence_before_upsell,
                    "surfaces_evidence_before_upsell",
                ),
            ] {
                if !flag {
                    push("surface_binding", &format!("a surface must assert {field}"));
                }
            }
            if binding.bound_card_ids.is_empty() {
                push(
                    "surface_binding.bound_card_ids",
                    "a binding must resolve through at least one card",
                );
            }
            for card_ref in &binding.bound_card_ids {
                if !card_ids.contains(card_ref.as_str()) {
                    push(
                        "surface_binding.bound_card_ids",
                        "binding card ref must resolve to a card",
                    );
                }
            }
        }
        // Every surface must be bound.
        for surface in BoundarySurface::ALL {
            if !self.surface_bindings.iter().any(|b| b.surface == surface) {
                push(
                    "surface_bindings",
                    "Help/About, release center, diagnostics, procurement, support/admin, and claim automation must all bind",
                );
                break;
            }
        }
    }

    /// Cross-checks every managed card against its control-plane lane row.
    ///
    /// Confirms each [`BoundaryClass::ManagedPaidOptional`] card agrees with the
    /// commercial-control-plane lane for its [`ServiceFamily`] on the declared
    /// marketed claim, the export guarantee, and a non-empty local-safe baseline,
    /// so a boundary card projects the matrix rather than inventing a parallel
    /// boundary. Returns an empty vector when every managed card matches its lane.
    pub fn cross_check_against_control_plane(&self) -> Vec<BoundaryCardViolation> {
        let matrix = canonical_stable_commercial_control_plane_matrix();
        let mut violations = Vec::new();
        for card in &self.cards {
            let Some(family) = card.service_family else {
                continue;
            };
            let Some(lane) = matrix.lanes.iter().find(|l| l.service_family == family) else {
                violations.push(BoundaryCardViolation {
                    field: "card.service_family".to_owned(),
                    message: format!(
                        "card {} maps to service family {family:?} with no control-plane lane",
                        card.card_id
                    ),
                });
                continue;
            };
            let mut mismatch = |field: &str| {
                violations.push(BoundaryCardViolation {
                    field: field.to_owned(),
                    message: format!(
                        "card {} drifted from control-plane lane {}",
                        card.card_id, lane.lane_id
                    ),
                });
            };
            if card.declared_marketed_claim != lane.declared_marketed_claim {
                mismatch("card.declared_marketed_claim");
            }
            if card.procurement_support_evidence.export_guarantee != lane.export_guarantee {
                mismatch("card.procurement_support_evidence.export_guarantee");
            }
            if lane.local_safe_baseline.is_empty() || card.local_safe_baseline.is_empty() {
                mismatch("card.local_safe_baseline");
            }
        }
        violations
    }
}

impl BoundaryCardInspection {
    fn derive(cards: &[BoundaryCard], surface_bindings: &[BoundarySurfaceBinding]) -> Self {
        let families: BTreeSet<ServiceFamily> =
            cards.iter().filter_map(|c| c.service_family).collect();
        let surfaces: BTreeSet<BoundarySurface> =
            surface_bindings.iter().map(|b| b.surface).collect();

        let local_open_card_count = cards
            .iter()
            .filter(|c| c.boundary_class == BoundaryClass::LocalOpenSource)
            .count();
        let managed_paid_card_count = cards
            .iter()
            .filter(|c| c.boundary_class == BoundaryClass::ManagedPaidOptional)
            .count();

        let effective_full_card_count = cards
            .iter()
            .filter(|c| c.effective_marketed_claim == MarketedClaim::ManagedFull)
            .count();
        let narrowed_card_count = cards
            .iter()
            .filter(|c| c.effective_marketed_claim != c.declared_marketed_claim)
            .count();
        let local_safe_only_card_count = cards
            .iter()
            .filter(|c| c.effective_marketed_claim == MarketedClaim::LocalSafeOnly)
            .count();

        Self {
            record_kind: INSPECTION_RECORD_KIND.to_owned(),
            schema_version: COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION,
            card_count: cards.len(),
            surface_binding_count: surface_bindings.len(),
            local_open_card_count,
            managed_paid_card_count,
            service_families_covered: families.len(),
            managed_lane_coverage_complete: families.len() == ServiceFamily::ALL.len(),
            surface_coverage_complete: surfaces.len() == BoundarySurface::ALL.len(),
            all_cards_local_safe_backed: cards.iter().all(|c| !c.local_safe_baseline.is_empty()),
            all_managed_cards_disclose_residual: cards
                .iter()
                .filter(|c| c.boundary_class.is_managed_paid())
                .all(|c| !c.residual_dependencies.is_empty()),
            all_cards_qualify_deployment_profile: cards
                .iter()
                .all(|c| !c.deployment_profile_qualifier.holds_in_profiles.is_empty()),
            all_cards_link_procurement_evidence: cards
                .iter()
                .all(|c| !c.procurement_support_evidence.packet_kinds.is_empty()),
            value_never_bare: cards.iter().all(|c| !c.as_of.trim().is_empty()),
            upsell_never_outranks_truth: cards.iter().all(|c| c.upsell_never_outranks_truth()),
            effective_full_card_count,
            narrowed_card_count,
            local_safe_only_card_count,
        }
    }
}

/// One failed card-set invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryCardViolation {
    /// The field path that failed.
    pub field: String,
    /// A short reviewable message.
    pub message: String,
}

impl fmt::Display for BoundaryCardViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

/// Error returned when the checked-in set cannot be read or validated.
#[derive(Debug)]
pub enum BoundaryCardError {
    /// The checked-in JSON failed to parse.
    Parse(serde_json::Error),
    /// The checked-in set failed validation.
    Validation(Vec<BoundaryCardViolation>),
}

impl fmt::Display for BoundaryCardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "commercial boundary card set parse error: {err}"),
            Self::Validation(violations) => write!(
                f,
                "commercial boundary card set failed validation: {} violation(s)",
                violations.len()
            ),
        }
    }
}

impl std::error::Error for BoundaryCardError {}

/// Returns the weaker (more-narrowed) of two marketed claims.
fn weaker_claim(a: MarketedClaim, b: MarketedClaim) -> MarketedClaim {
    fn rank(claim: MarketedClaim) -> u8 {
        match claim {
            MarketedClaim::LocalSafeOnly => 0,
            MarketedClaim::ManagedNarrowed => 1,
            MarketedClaim::ManagedFull => 2,
        }
    }
    if rank(a) <= rank(b) {
        a
    } else {
        b
    }
}

/// The recovery cue shown when boundary evidence narrows a managed card.
fn evidence_recovery_cue(status: BoundaryEvidenceStatus) -> String {
    match status {
        BoundaryEvidenceStatus::Current => {
            "Boundary evidence is current; no recovery needed."
        }
        BoundaryEvidenceStatus::Stale => {
            "The boundary evidence is stale and labeled; the managed claim narrows until it refreshes. Local work continues now."
        }
        BoundaryEvidenceStatus::Missing => {
            "The boundary evidence is missing; the surface claims only the local-safe baseline until it is restored. Local work continues now."
        }
        BoundaryEvidenceStatus::Downgraded => {
            "The backing claim was downgraded; the surface claims only the local-safe baseline. Local work continues now."
        }
    }
    .to_owned()
}

/// Reads and validates the checked-in stable commercial-boundary-card set.
///
/// This is the canonical reader: Help/About, the release center, diagnostics, the
/// procurement/support packet, and claim/public-truth automation call it to ingest
/// the cards rather than cloning status text.
///
/// # Errors
///
/// Returns [`BoundaryCardError`] when the checked-in packet fails to parse or
/// fails validation.
pub fn current_stable_commercial_boundary_card_set() -> Result<BoundaryCardSet, BoundaryCardError> {
    let set: BoundaryCardSet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/service/m5-commercial-boundary-cards.json"
    )))
    .map_err(BoundaryCardError::Parse)?;
    let violations = set.validate();
    if violations.is_empty() {
        Ok(set)
    } else {
        Err(BoundaryCardError::Validation(violations))
    }
}

/// Source refs every boundary-card export carries.
fn card_source_refs() -> Vec<String> {
    let mut refs = vec![
        COMMERCIAL_BOUNDARY_CARDS_SCHEMA_REF.to_owned(),
        COMMERCIAL_BOUNDARY_CARDS_DOC_REF.to_owned(),
        "artifacts/governance/residual_dependencies.yaml".to_owned(),
        "artifacts/governance/deployment_profiles.yaml".to_owned(),
        "artifacts/service/residual_dependency_cost_notes.yaml".to_owned(),
    ];
    // Reuse the control-plane refs so the cards cite the same frozen vocabulary.
    refs.extend(canonical_source_refs());
    refs
}

/// Deterministic as-of time for the checked-in cards.
pub const STABLE_AS_OF: &str = "2026-06-15T00:00:00Z";

/// Stable identifier for the checked-in set.
pub const STABLE_SET_ID: &str = "commercial-boundary-cards:stable:0001";

/// Stable title for the checked-in set.
pub const STABLE_SET_TITLE: &str =
    "Help/About, release-center, diagnostics, and procurement commercial-boundary cards with open-versus-paid truth, residual-dependency disclosure, and procurement/support packet parity";

/// Deterministic timestamp for the checked-in set.
pub const STABLE_SET_GENERATED_AT: &str = "2026-06-15T00:00:00Z";

/// Revision for the checked-in set.
pub const STABLE_SET_REVISION: u32 = 1;

/// The fixed, per-card data a card is built from.
struct CardDef {
    card_id: &'static str,
    title: &'static str,
    summary: &'static str,
    boundary_class: BoundaryClass,
    service_family: Option<ServiceFamily>,
    open_paid_statement: &'static str,
    posture_origin: PostureOrigin,
    residual_dependencies: Vec<ResidualDependency>,
    holds_in_profiles: &'static [DeploymentProfile],
    not_offered_in_profiles: &'static [DeploymentProfile],
    qualifier_note: &'static str,
    packet_kinds: &'static [ProcurementPacketKind],
    export_guarantee: ExportGuarantee,
    support_admin_packet_ref: &'static str,
    evidence_summary: &'static str,
    local_safe_baseline: &'static [&'static str],
    has_upsell: bool,
}

fn build_actions(has_upsell: bool) -> Vec<BoundaryAction> {
    let mut actions = vec![
        BoundaryAction::new(
            BoundaryActionKind::ExportEvidence,
            1,
            "Export the open-source license and residual-dependency evidence now (CSV and JSON where offered).",
        ),
        BoundaryAction::new(
            BoundaryActionKind::ContinueLocal,
            2,
            "Keep editing, searching, and using Git locally; the open core is unaffected.",
        ),
        BoundaryAction::new(
            BoundaryActionKind::ViewProcurementPacket,
            3,
            "View or assemble the procurement and support evidence packet.",
        ),
        BoundaryAction::new(
            BoundaryActionKind::ViewResidualDependencies,
            4,
            "Review the residual vendor-hosted dependencies and how to localize them.",
        ),
        BoundaryAction::new(
            BoundaryActionKind::ViewDeploymentProfileTruth,
            5,
            "Review which deployment profiles this boundary holds in.",
        ),
    ];
    if has_upsell {
        actions.push(BoundaryAction::new(
            BoundaryActionKind::LearnAboutPaid,
            6,
            "Learn about the optional managed lane; this never outranks export, procurement, or local continuation.",
        ));
    }
    actions
}

fn build_card(def: CardDef) -> BoundaryCard {
    let qualifier = DeploymentProfileQualifier {
        record_kind: PROFILE_QUALIFIER_RECORD_KIND.to_owned(),
        schema_version: COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION,
        holds_in_profiles: def.holds_in_profiles.to_vec(),
        not_offered_in_profiles: def.not_offered_in_profiles.to_vec(),
        qualifier_note: def.qualifier_note.to_owned(),
    };

    let evidence = ProcurementSupportEvidence {
        record_kind: PROCUREMENT_EVIDENCE_RECORD_KIND.to_owned(),
        schema_version: COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION,
        packet_kinds: def.packet_kinds.to_vec(),
        export_guarantee: def.export_guarantee,
        support_admin_packet_ref: def.support_admin_packet_ref.to_owned(),
        summary: def.evidence_summary.to_owned(),
    };

    let declared = def.boundary_class.declared_claim();
    // The canonical set ships with current evidence, so no card is narrowed.
    let evidence_status = BoundaryEvidenceStatus::Current;
    let effective = weaker_claim(declared, evidence_status.claim_cap());

    BoundaryCard {
        record_kind: CARD_RECORD_KIND.to_owned(),
        schema_version: COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION,
        card_id: def.card_id.to_owned(),
        title: def.title.to_owned(),
        summary: def.summary.to_owned(),
        boundary_class: def.boundary_class,
        service_family: def.service_family,
        open_paid_statement: def.open_paid_statement.to_owned(),
        posture_origin: def.posture_origin,
        residual_dependencies: def.residual_dependencies,
        deployment_profile_qualifier: qualifier,
        procurement_support_evidence: evidence,
        local_safe_baseline: def
            .local_safe_baseline
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        cost_figure_disclosure: CostFigureDisclosure::DeferredToMeteringSurfaces,
        as_of: STABLE_AS_OF.to_owned(),
        evidence_status,
        actions: build_actions(def.has_upsell),
        declared_marketed_claim: declared,
        effective_marketed_claim: effective,
        recovery_cue: None,
    }
}

fn binding(
    binding_id: &str,
    surface: BoundarySurface,
    bound_card_ids: &[&str],
    summary: &str,
) -> BoundarySurfaceBinding {
    BoundarySurfaceBinding {
        record_kind: SURFACE_BINDING_RECORD_KIND.to_owned(),
        schema_version: COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION,
        binding_id: binding_id.to_owned(),
        surface,
        bound_card_ids: bound_card_ids.iter().map(|s| (*s).to_owned()).collect(),
        projects_effective_claim: true,
        renders_local_safe_baseline: true,
        discloses_residual_dependencies: true,
        names_deployment_profile_qualifier: true,
        surfaces_evidence_before_upsell: true,
        summary: summary.to_owned(),
    }
}

/// Builds the checked-in set with the stable identity constants.
///
/// The checked-in artifact, the conformance dump, and the round-trip test all
/// build through this function so they agree on every field.
pub fn canonical_stable_commercial_boundary_card_set() -> BoundaryCardSet {
    canonical_commercial_boundary_card_set(
        STABLE_SET_ID.to_owned(),
        STABLE_SET_TITLE.to_owned(),
        STABLE_SET_GENERATED_AT.to_owned(),
        STABLE_SET_REVISION,
    )
}

/// Builds the canonical, frozen commercial-boundary-card set.
///
/// The set freezes one local-open-core card plus one card per managed
/// [`ServiceFamily`], and one binding per [`BoundarySurface`]. Each card states
/// its open-versus-paid class, its residual vendor-hosted dependencies (empty only
/// for the local open core), the deployment profiles its boundary holds in, the
/// procurement/support evidence available, and a non-empty local-safe baseline,
/// keeps export and local continuation above any upsell, and narrows its marketed
/// claim from the boundary evidence status.
pub fn canonical_commercial_boundary_card_set(
    set_id: String,
    title: String,
    generated_at: String,
    set_revision: u32,
) -> BoundaryCardSet {
    let defs = vec![
        CardDef {
            card_id: "commercial_boundary.local_open_core",
            title: "Local open-source core",
            summary: "Editing, search, navigation, local Git, and already-authorized local automation are local, open-source, and free. They never depend on a managed account, a paid plan, or a network round trip.",
            boundary_class: BoundaryClass::LocalOpenSource,
            service_family: None,
            open_paid_statement: "Open and local: the editor core, search, Git, and local automation are open-source and run with no managed dependency and no payment.",
            posture_origin: PostureOrigin::LocalOnlyNoManagedAccount,
            residual_dependencies: vec![],
            holds_in_profiles: &[
                DeploymentProfile::IndividualLocal,
                DeploymentProfile::SelfHosted,
                DeploymentProfile::EnterpriseOnline,
                DeploymentProfile::AirGapped,
                DeploymentProfile::ManagedCloud,
            ],
            not_offered_in_profiles: &[],
            qualifier_note: "The local open core is available in every deployment profile, including fully offline and air-gapped, with no residual vendor dependency.",
            packet_kinds: &[
                ProcurementPacketKind::OpenSourceLicenseManifest,
                ProcurementPacketKind::DeploymentProfileTruthPacket,
            ],
            export_guarantee: ExportGuarantee::ParityWithCsvAndJson,
            support_admin_packet_ref: "support.packet.open_source_license_and_profile_truth",
            evidence_summary: "The open-source license and component manifest plus the deployment-profile truth packet prove the local core is open and self-contained.",
            local_safe_baseline: &[
                "Local editing, search, navigation, and Git run with no managed dependency.",
                "Already-authorized local automation and local AI (BYOK) continue offline.",
            ],
            has_upsell: false,
        },
        CardDef {
            card_id: "commercial_boundary.ai_gateway",
            title: "Managed AI gateway (optional, paid)",
            summary: "The managed AI broker is an optional paid lane. Bring-your-own-key and local AI providers keep running without it, and non-AI editing is unaffected.",
            boundary_class: BoundaryClass::ManagedPaidOptional,
            service_family: Some(ServiceFamily::AiGatewayFamily),
            open_paid_statement: "Paid and optional: managed-broker inference is metered and billed; direct, bring-your-own-key, and local AI routes are the open alternative.",
            posture_origin: PostureOrigin::Plan,
            residual_dependencies: vec![
                ResidualDependency::new(
                    DependencyClass::AiProvider,
                    true,
                    true,
                    "Managed inference routes to a vendor-hosted AI provider; switching to bring-your-own-key or a local provider removes that dependency.",
                ),
                ResidualDependency::new(
                    DependencyClass::HostedControlPlaneReachability,
                    true,
                    true,
                    "The managed broker requires reachability to the hosted control plane; a self-hosted or BYOK route does not.",
                ),
            ],
            holds_in_profiles: &[
                DeploymentProfile::EnterpriseOnline,
                DeploymentProfile::ManagedCloud,
            ],
            not_offered_in_profiles: &[DeploymentProfile::AirGapped],
            qualifier_note: "The managed broker is offered online; air-gapped and self-hosted deployments use bring-your-own-key or local providers instead of the vendor broker.",
            packet_kinds: &[
                ProcurementPacketKind::UsageAndForecastExport,
                ProcurementPacketKind::ChargebackExport,
                ProcurementPacketKind::ResidualDependencyDisclosure,
                ProcurementPacketKind::EntitlementSummary,
            ],
            export_guarantee: ExportGuarantee::ParityWithCsvAndJson,
            support_admin_packet_ref: "support.packet.ai_gateway_usage_and_residual",
            evidence_summary: "AI token usage, forecast, and chargeback exports plus the residual-dependency disclosure show the paid boundary and what continues locally.",
            local_safe_baseline: &[
                "Direct and bring-your-own-key AI routes keep running.",
                "Local editing, search, and Git are unaffected when the managed broker is off.",
            ],
            has_upsell: true,
        },
        CardDef {
            card_id: "commercial_boundary.settings_sync",
            title: "Managed settings sync (optional, paid)",
            summary: "Cross-device settings sync is an optional paid lane. Local settings and files stay authoritative when sync is off.",
            boundary_class: BoundaryClass::ManagedPaidOptional,
            service_family: Some(ServiceFamily::SyncFamily),
            open_paid_statement: "Paid and optional: managed settings sync replicates settings across devices; local settings and files are the open, authoritative source either way.",
            posture_origin: PostureOrigin::Plan,
            residual_dependencies: vec![
                ResidualDependency::new(
                    DependencyClass::HostedControlPlaneReachability,
                    true,
                    true,
                    "Sync replicates through the hosted control plane; a self-hosted control plane keeps the data customer-operated.",
                ),
                ResidualDependency::new(
                    DependencyClass::SignIn,
                    true,
                    false,
                    "Sync requires a managed sign-in to scope the synced settings; local settings need no sign-in.",
                ),
            ],
            holds_in_profiles: &[
                DeploymentProfile::SelfHosted,
                DeploymentProfile::EnterpriseOnline,
                DeploymentProfile::ManagedCloud,
            ],
            not_offered_in_profiles: &[
                DeploymentProfile::IndividualLocal,
                DeploymentProfile::AirGapped,
            ],
            qualifier_note: "Sync is offered for signed-in deployments; individual-local and air-gapped installs keep local settings authoritative with no sync.",
            packet_kinds: &[
                ProcurementPacketKind::UsageAndForecastExport,
                ProcurementPacketKind::ChargebackExport,
                ProcurementPacketKind::ResidualDependencyDisclosure,
            ],
            export_guarantee: ExportGuarantee::ParityWithJsonOnly,
            support_admin_packet_ref: "support.packet.settings_sync_usage_and_residual",
            evidence_summary: "Stored-bytes usage and chargeback exports plus the residual-dependency disclosure show the sync boundary and the local-authoritative fallback.",
            local_safe_baseline: &[
                "Local settings and files stay authoritative on device.",
                "Editing continues offline; sync resumes when the lane clears.",
            ],
            has_upsell: true,
        },
        CardDef {
            card_id: "commercial_boundary.companion_relay",
            title: "Companion relay (optional, paid)",
            summary: "The managed collaboration relay is an optional paid lane. Local incident notes and offline packets continue without it.",
            boundary_class: BoundaryClass::ManagedPaidOptional,
            service_family: Some(ServiceFamily::CollaborationRelayFamily),
            open_paid_statement: "Paid and optional: the managed relay carries live collaboration and companion-follow sessions; local notes, patches, and offline packets are the open alternative.",
            posture_origin: PostureOrigin::Plan,
            residual_dependencies: vec![
                ResidualDependency::new(
                    DependencyClass::HostedControlPlaneReachability,
                    true,
                    true,
                    "Live relay transport needs the hosted control plane; a self-hosted relay keeps transport customer-operated.",
                ),
                ResidualDependency::new(
                    DependencyClass::CompanionNotificationChannel,
                    true,
                    false,
                    "Companion notifications route through a vendor channel; desktop handoff resumes the same local context without it.",
                ),
            ],
            holds_in_profiles: &[
                DeploymentProfile::SelfHosted,
                DeploymentProfile::EnterpriseOnline,
                DeploymentProfile::ManagedCloud,
            ],
            not_offered_in_profiles: &[
                DeploymentProfile::IndividualLocal,
                DeploymentProfile::AirGapped,
            ],
            qualifier_note: "Live relay is offered for connected deployments; individual-local and air-gapped installs keep local collaboration artifacts with no relay.",
            packet_kinds: &[
                ProcurementPacketKind::UsageAndForecastExport,
                ProcurementPacketKind::ChargebackExport,
                ProcurementPacketKind::ResidualDependencyDisclosure,
                ProcurementPacketKind::SupportBundle,
            ],
            export_guarantee: ExportGuarantee::ParityWithCsvAndJson,
            support_admin_packet_ref: "support.packet.companion_relay_usage_and_residual",
            evidence_summary: "Participant-minute usage and chargeback exports plus the residual-dependency disclosure show the relay boundary and the local-collaboration fallback.",
            local_safe_baseline: &[
                "Local incident notes and offline packets continue.",
                "Desktop handoff resumes the exact local context.",
            ],
            has_upsell: true,
        },
        CardDef {
            card_id: "commercial_boundary.registry_mirror",
            title: "Managed registry and mirror (optional, paid)",
            summary: "The managed registry and mirror is an optional paid lane. Installed extensions and local or sideloaded packages keep running without it.",
            boundary_class: BoundaryClass::ManagedPaidOptional,
            service_family: Some(ServiceFamily::RegistryOrMirrorMetadataFamily),
            open_paid_statement: "Paid and optional: the managed registry and mirror serve discovery and install metadata; a customer-operated mirror or offline bundle is the open alternative.",
            posture_origin: PostureOrigin::Plan,
            residual_dependencies: vec![
                ResidualDependency::new(
                    DependencyClass::PackageRegistry,
                    true,
                    true,
                    "New installs read the managed package registry; a customer-operated mirror or signed offline bundle removes that dependency.",
                ),
                ResidualDependency::new(
                    DependencyClass::RemoteMirror,
                    true,
                    true,
                    "Catalog refresh reads the managed mirror; an air-gapped deployment imports a signed mirror snapshot instead.",
                ),
            ],
            holds_in_profiles: &[
                DeploymentProfile::SelfHosted,
                DeploymentProfile::EnterpriseOnline,
                DeploymentProfile::AirGapped,
                DeploymentProfile::ManagedCloud,
            ],
            not_offered_in_profiles: &[],
            qualifier_note: "Discovery and install are offered everywhere; air-gapped and self-hosted deployments resolve against a customer-operated mirror or signed offline bundle rather than the vendor registry.",
            packet_kinds: &[
                ProcurementPacketKind::UsageAndForecastExport,
                ProcurementPacketKind::ChargebackExport,
                ProcurementPacketKind::ResidualDependencyDisclosure,
            ],
            export_guarantee: ExportGuarantee::ParityWithCsvAndJson,
            support_admin_packet_ref: "support.packet.registry_mirror_usage_and_residual",
            evidence_summary: "Download-count usage and chargeback exports plus the residual-dependency disclosure show the registry boundary and the mirror/offline fallback.",
            local_safe_baseline: &[
                "Installed extensions keep running.",
                "Local and sideloaded packages are unaffected.",
            ],
            has_upsell: true,
        },
        CardDef {
            card_id: "commercial_boundary.support_ingest",
            title: "Managed support ingest (optional, paid)",
            summary: "Managed support-bundle ingest is an optional paid lane. Local support bundles still generate and export without it.",
            boundary_class: BoundaryClass::ManagedPaidOptional,
            service_family: Some(ServiceFamily::TelemetryOrSupportIngestFamily),
            open_paid_statement: "Paid and optional: managed support ingest uploads bundles to the vendor sink; local support-bundle export is the open alternative and always available.",
            posture_origin: PostureOrigin::Plan,
            residual_dependencies: vec![
                ResidualDependency::new(
                    DependencyClass::HostedControlPlaneReachability,
                    true,
                    false,
                    "Upload needs the hosted ingest sink; local export of the same bundle never does.",
                ),
                ResidualDependency::new(
                    DependencyClass::PolicyBundle,
                    true,
                    true,
                    "Upload enforces a managed redaction/policy bundle; a self-hosted policy bundle keeps that enforcement customer-operated.",
                ),
            ],
            holds_in_profiles: &[
                DeploymentProfile::SelfHosted,
                DeploymentProfile::EnterpriseOnline,
                DeploymentProfile::ManagedCloud,
            ],
            not_offered_in_profiles: &[DeploymentProfile::AirGapped],
            qualifier_note: "Managed upload is offered for connected deployments; air-gapped installs export bundles locally and deliver them through an approved channel.",
            packet_kinds: &[
                ProcurementPacketKind::UsageAndForecastExport,
                ProcurementPacketKind::ResidualDependencyDisclosure,
                ProcurementPacketKind::SupportBundle,
            ],
            export_guarantee: ExportGuarantee::ParityWithCsvAndJson,
            support_admin_packet_ref: "support.packet.support_ingest_usage_and_residual",
            evidence_summary: "Support-bundle-count usage and the residual-dependency disclosure show the ingest boundary and the always-available local-export fallback.",
            local_safe_baseline: &[
                "Local support bundles still generate on device.",
                "Offline evidence capture continues.",
            ],
            has_upsell: true,
        },
        CardDef {
            card_id: "commercial_boundary.managed_workspace",
            title: "Managed workspace (optional, paid)",
            summary: "The remote managed workspace is an optional paid lane. Local checkout, editing, tasks, and Git continue without it.",
            boundary_class: BoundaryClass::ManagedPaidOptional,
            service_family: Some(ServiceFamily::RemoteWorkspaceControlPlaneFamily),
            open_paid_statement: "Paid and optional: the remote workspace runs sessions on the managed control plane; local checkout, tasks, and Git are the open alternative.",
            posture_origin: PostureOrigin::Plan,
            residual_dependencies: vec![
                ResidualDependency::new(
                    DependencyClass::RemoteAgent,
                    true,
                    true,
                    "Remote sessions run on a vendor-hosted agent; a self-hosted control plane runs the agent in the customer environment.",
                ),
                ResidualDependency::new(
                    DependencyClass::HostedControlPlaneReachability,
                    true,
                    true,
                    "Attaching a remote workspace needs the hosted control plane; local checkout never does.",
                ),
            ],
            holds_in_profiles: &[
                DeploymentProfile::SelfHosted,
                DeploymentProfile::EnterpriseOnline,
                DeploymentProfile::ManagedCloud,
            ],
            not_offered_in_profiles: &[
                DeploymentProfile::IndividualLocal,
                DeploymentProfile::AirGapped,
            ],
            qualifier_note: "Remote workspaces are offered for connected deployments; individual-local and air-gapped installs work entirely from the local checkout.",
            packet_kinds: &[
                ProcurementPacketKind::UsageAndForecastExport,
                ProcurementPacketKind::ChargebackExport,
                ProcurementPacketKind::ResidualDependencyDisclosure,
            ],
            export_guarantee: ExportGuarantee::ParityWithJsonOnly,
            support_admin_packet_ref: "support.packet.managed_workspace_usage_and_residual",
            evidence_summary: "Workspace-hour usage and chargeback exports plus the residual-dependency disclosure show the remote-workspace boundary and the local-checkout fallback.",
            local_safe_baseline: &[
                "Local checkout and editing continue.",
                "Local tasks and Git are unaffected when the remote workspace narrows.",
            ],
            has_upsell: true,
        },
    ];

    let cards: Vec<BoundaryCard> = defs.into_iter().map(build_card).collect();

    let all_card_ids: Vec<&str> = cards.iter().map(|c| c.card_id.as_str()).collect();
    // Procurement and support packets bind the managed cards plus the open-core
    // card so the open-versus-paid boundary reads as one object model.
    let procurement_card_ids: Vec<&str> = all_card_ids.clone();

    let surface_bindings = vec![
        binding(
            "commercial_boundary_surface.help_about",
            BoundarySurface::HelpAbout,
            &all_card_ids,
            "Help/About states which capabilities are local and open-source, which are optional managed/paid lanes, what residual dependencies remain vendor-hosted, and which deployment profiles each boundary holds in.",
        ),
        binding(
            "commercial_boundary_surface.release_center",
            BoundarySurface::ReleaseCenter,
            &all_card_ids,
            "The release center shows the open-versus-paid boundary, residual-dependency disclosure, and deployment-profile qualifiers alongside each release, projecting the effective claim without overstating the open boundary.",
        ),
        binding(
            "commercial_boundary_surface.diagnostics",
            BoundarySurface::Diagnostics,
            &all_card_ids,
            "Diagnostics and service-health surfaces project each card's effective claim and local-safe baseline, and never block the local core when a managed lane's evidence is stale or missing.",
        ),
        binding(
            "commercial_boundary_surface.procurement_packet",
            BoundarySurface::ProcurementPacket,
            &procurement_card_ids,
            "The procurement packet reuses the same evidence object — open-source license manifest, residual-dependency disclosure, usage/forecast and chargeback exports, and deployment-profile truth — so a buyer reads one vocabulary.",
        ),
        binding(
            "commercial_boundary_surface.support_admin_packet",
            BoundarySurface::SupportAdminPacket,
            &procurement_card_ids,
            "The support/admin packet binds the same procurement/support evidence object, keeping export and support truth above any upsell prompt.",
        ),
        binding(
            "commercial_boundary_surface.claim_public_truth",
            BoundarySurface::ClaimPublicTruthAutomation,
            &all_card_ids,
            "Claim and public-truth automation narrows a marketed/support claim to each card's effective claim when the boundary evidence is stale, missing, or downgraded.",
        ),
    ];

    let inspection = BoundaryCardInspection::derive(&cards, &surface_bindings);

    let summary =
        "Frozen Help/About, release-center, diagnostics, and procurement commercial-boundary \
        cards for the local open core and the managed lanes. Each card states the open-versus-paid \
        boundary, discloses residual vendor-hosted dependencies, names the deployment profiles its \
        boundary holds in, links procurement/support evidence at export parity, keeps a non-empty \
        local-safe baseline above any upsell, and narrows the marketed claim from the boundary \
        evidence status — never blocking the local core or overstating the open boundary."
            .to_owned();

    BoundaryCardSet {
        record_kind: CARD_SET_RECORD_KIND.to_owned(),
        schema_version: COMMERCIAL_BOUNDARY_CARDS_SCHEMA_VERSION,
        set_id,
        generated_at,
        set_revision,
        title,
        summary,
        source_refs: card_source_refs(),
        cards,
        surface_bindings,
        inspection,
    }
}
