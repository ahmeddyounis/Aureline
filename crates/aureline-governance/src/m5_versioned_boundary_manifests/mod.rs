//! Typed register of versioned, per-family boundary manifests.
//!
//! The sibling [`m5_boundary_and_upstream_durability`](crate::m5_boundary_and_upstream_durability)
//! matrix freezes the standing durability posture of every asset lane: where the
//! open/local core ends, which compliance controls hold, who holds emergency
//! authority, and whether critical upstreams are owned. It answers *is the lane
//! durable right now?* — but it is one matrix, not bound to a release train, and it
//! does not, per claimed feature family, publish the inspectable boundary claim a
//! user, admin, or procurement reviewer reads off the product.
//!
//! This module is that publication layer. For every claimed M5 family it records
//! one **versioned** [`BoundaryManifest`] that states, in one copy-safe record:
//!
//! - which capabilities stay open/local and which may be productized, expressed as
//!   a per-[`AssetLane`](crate::m5_boundary_and_upstream_durability::AssetLane)
//!   [`LaneDisposition`] bound to the lane's boundary posture and support class;
//! - the [`Guardrail`] set that preserves the claim — the local core stays useful,
//!   new managed value never silently redefines it, the open-core claim carries
//!   per-lane detail rather than vague slogans, residual dependencies are disclosed,
//!   and the manifest is linked from release evidence;
//! - the [`ResidualDependency`] disclosure — every residual proprietary or hosted
//!   dependency the family still rests on, and whether it is disclosed on the
//!   user/admin truth surfaces or omitted;
//! - the [`ReleaseLink`] binding the manifest to a release train, so the manifest's
//!   declared label can be held in **parity** with the release evidence — a manifest
//!   may never publish a label greener than the train backing it.
//!
//! A manifest is [`ManifestState::Published`] only when its release link is present
//! and fresh, its declared label is in parity with the train, every residual
//! dependency is disclosed, every guardrail holds, the proof is fresh, and the owner
//! signed. Otherwise it narrows on the *specific* axis that thinned out — a release
//! link gap, a parity break, an undisclosed dependency, an unsatisfied guardrail, or
//! stale proof — never collapsing to one global flag. A narrowed manifest drops its
//! [`BoundaryManifest::effective_label`] below the launch cutline and may never
//! publish an effective label wider than the one it declares.
//!
//! The [`GuardrailRule`] set names the closed conditions that gate publication. An
//! *inherited* narrowing — a family whose declared label already sits below the
//! cutline, or a gap held by an unexpired waiver — is gated upstream and does not
//! itself hold promotion; a *manifest-layer* failure on a family whose declared
//! label is still at or above the cutline holds promotion through a stop rule,
//! recorded in [`BoundaryManifestRegister::publication`]. The cross-family
//! [`ReleaseLinkParity`] block summarizes release-link parity over the whole train.
//!
//! The register is checked in at
//! `artifacts/governance/m5-versioned-boundary-manifests.json` and embedded here, so
//! this typed consumer and the CI gate agree on every manifest without a cargo build
//! in CI. The model is metadata-only: every field is a typed state, a boolean flag,
//! a small count, a version string, or an opaque ref. It carries no credential
//! bodies, raw provider payloads, signatures, or attestation material. Date
//! arithmetic (recomputing proof freshness, link freshness, and waiver expiry against
//! an `as_of` date) and cross-artifact joins against the durability matrix live in
//! the CI gate and the integration test; this model enforces the invariants that hold
//! regardless of the clock: the parity ceiling, the no-widening ceiling, guardrail and
//! disclosure completeness, narrowing consistency, reason/state coherence, summary
//! agreement, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_boundary_and_upstream_durability::{
    AssetLane, BoundaryPosture, FreshnessSloState, LifecycleLabel, OwnerSignoff, ProofPacket,
    SupportClass, Waiver,
};

/// Supported register schema version.
pub const M5_VERSIONED_BOUNDARY_MANIFESTS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_VERSIONED_BOUNDARY_MANIFESTS_RECORD_KIND: &str =
    "m5_versioned_boundary_manifest_register";

/// Repo-relative path to the checked-in register.
pub const M5_VERSIONED_BOUNDARY_MANIFESTS_PATH: &str =
    "artifacts/governance/m5-versioned-boundary-manifests.json";

/// Embedded checked-in register JSON.
pub const M5_VERSIONED_BOUNDARY_MANIFESTS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/m5-versioned-boundary-manifests.json"
));

/// A claimed M5 feature family a manifest governs.
///
/// This reuses the train-wide family vocabulary the qualification matrix and claim
/// manifests already publish, so a manifest joins to its release evidence by family
/// rather than minting a manifest-local synonym set. Every family must be covered by
/// exactly one manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5Family {
    /// Notebook and data-rich notebook depth surfaces.
    Notebook,
    /// Data-heavy surfaces (result grids, variable explorers).
    DataRich,
    /// AI-adjacent surfaces and language intelligence.
    AiAdjacent,
    /// Core framework and platform foundations.
    Framework,
    /// Review and diff surfaces.
    Review,
    /// Browser/mobile companion surfaces.
    Companion,
    /// Managed-depth and infrastructure surfaces.
    ManagedDepth,
}

impl M5Family {
    /// Every family, in declaration order. Every family must have a manifest.
    pub const ALL: [Self; 7] = [
        Self::Notebook,
        Self::DataRich,
        Self::AiAdjacent,
        Self::Framework,
        Self::Review,
        Self::Companion,
        Self::ManagedDepth,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::DataRich => "data_rich",
            Self::AiAdjacent => "ai_adjacent",
            Self::Framework => "framework",
            Self::Review => "review",
            Self::Companion => "companion",
            Self::ManagedDepth => "managed_depth",
        }
    }
}

/// How a single asset lane is dispositioned within a family's manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneDisposition {
    /// The capability stays open and locally useful.
    OpenLocalRetained,
    /// An open/local core with an optional productized managed add-on.
    ProductizableAddOn,
    /// Delivered only as a managed/hosted service, not part of the local core.
    ManagedOnly,
    /// A restricted (brand or source-available) asset.
    RestrictedAsset,
}

impl LaneDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OpenLocalRetained,
        Self::ProductizableAddOn,
        Self::ManagedOnly,
        Self::RestrictedAsset,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenLocalRetained => "open_local_retained",
            Self::ProductizableAddOn => "productizable_add_on",
            Self::ManagedOnly => "managed_only",
            Self::RestrictedAsset => "restricted_asset",
        }
    }

    /// True when this disposition is consistent with the lane's boundary posture.
    pub fn is_consistent_with(self, posture: BoundaryPosture) -> bool {
        match self {
            Self::OpenLocalRetained => posture.is_open_baseline(),
            Self::ProductizableAddOn => posture == BoundaryPosture::OpenLocalWithManagedOptional,
            Self::ManagedOnly => posture == BoundaryPosture::ManagedService,
            Self::RestrictedAsset => matches!(
                posture,
                BoundaryPosture::SourceAvailableRestricted | BoundaryPosture::RestrictedBrand
            ),
        }
    }
}

/// A guardrail that preserves a manifest's boundary claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardrailKind {
    /// The local core remains materially useful without any managed value.
    LocalCoreRemainsUseful,
    /// New managed value may not silently redefine local-core usefulness.
    NoSilentLocalRedefinition,
    /// The open-core claim carries per-lane asset detail, not vague slogans.
    AssetLaneDetailPublished,
    /// Residual proprietary/hosted dependencies are disclosed on truth surfaces.
    ResidualDependencyDisclosed,
    /// The manifest is linked from release evidence.
    ReleaseLinkPublished,
}

impl GuardrailKind {
    /// Every guardrail kind, in declaration order. Every manifest must declare each.
    pub const ALL: [Self; 5] = [
        Self::LocalCoreRemainsUseful,
        Self::NoSilentLocalRedefinition,
        Self::AssetLaneDetailPublished,
        Self::ResidualDependencyDisclosed,
        Self::ReleaseLinkPublished,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCoreRemainsUseful => "local_core_remains_useful",
            Self::NoSilentLocalRedefinition => "no_silent_local_redefinition",
            Self::AssetLaneDetailPublished => "asset_lane_detail_published",
            Self::ResidualDependencyDisclosed => "residual_dependency_disclosed",
            Self::ReleaseLinkPublished => "release_link_published",
        }
    }
}

/// Satisfaction state of a guardrail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardrailState {
    /// The guardrail holds.
    Satisfied,
    /// The guardrail applies but is not satisfied.
    Unsatisfied,
    /// The guardrail does not apply to this family.
    NotApplicable,
}

impl GuardrailState {
    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Unsatisfied => "unsatisfied",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// The class of a residual dependency a family still rests on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyClass {
    /// A proprietary (non-open) software component.
    ProprietaryComponent,
    /// A hosted service the family calls into.
    HostedService,
    /// A managed/hosted model provider.
    ManagedModelProvider,
    /// A trademark or brand asset.
    TrademarkBrandAsset,
}

impl DependencyClass {
    /// Every dependency class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProprietaryComponent,
        Self::HostedService,
        Self::ManagedModelProvider,
        Self::TrademarkBrandAsset,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProprietaryComponent => "proprietary_component",
            Self::HostedService => "hosted_service",
            Self::ManagedModelProvider => "managed_model_provider",
            Self::TrademarkBrandAsset => "trademark_brand_asset",
        }
    }
}

/// The state of a manifest's link to its release train.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseLinkState {
    /// The link is present, fresh, and in parity with the train.
    Linked,
    /// No link to a release train.
    Missing,
    /// The link exists but its evidence has aged past its freshness window.
    Stale,
    /// The manifest declares a label greener than the train's evidence supports.
    ParityBroken,
}

impl ReleaseLinkState {
    /// Every link state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Linked, Self::Missing, Self::Stale, Self::ParityBroken];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Linked => "linked",
            Self::Missing => "missing",
            Self::Stale => "stale",
            Self::ParityBroken => "parity_broken",
        }
    }
}

/// The state a manifest earns after narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestState {
    /// Release link, parity, disclosure, guardrails, and proof all hold.
    Published,
    /// The release link is missing or stale.
    NarrowedReleaseLink,
    /// The declared label is greener than the linked release evidence.
    NarrowedParity,
    /// A residual proprietary/hosted dependency is undisclosed.
    NarrowedDisclosure,
    /// A guardrail preserving the claim is unsatisfied.
    NarrowedGuardrail,
    /// The proof packet, sign-off, or waiver thinned out.
    NarrowedStale,
    /// The family is withdrawn.
    Withdrawn,
}

impl ManifestState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Published,
        Self::NarrowedReleaseLink,
        Self::NarrowedParity,
        Self::NarrowedDisclosure,
        Self::NarrowedGuardrail,
        Self::NarrowedStale,
        Self::Withdrawn,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::NarrowedReleaseLink => "narrowed_release_link",
            Self::NarrowedParity => "narrowed_parity",
            Self::NarrowedDisclosure => "narrowed_disclosure",
            Self::NarrowedGuardrail => "narrowed_guardrail",
            Self::NarrowedStale => "narrowed_stale",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when the state is a narrowed state (not published, not withdrawn).
    pub fn is_narrowed(self) -> bool {
        !matches!(self, Self::Published | Self::Withdrawn)
    }
}

/// A reason a manifest narrowed. Closed vocabulary; every reason is watched by a
/// [`GuardrailRule`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestReason {
    /// No link to a release train.
    ReleaseLinkMissing,
    /// The release-train link evidence is stale.
    ReleaseLinkStale,
    /// The declared label is greener than the linked release evidence.
    ReleaseParityBroken,
    /// A residual proprietary/hosted dependency is undisclosed.
    UndisclosedResidualDependency,
    /// A guardrail preserving the claim is unsatisfied.
    GuardrailUnsatisfied,
    /// The proof packet aged past its freshness SLO.
    ManifestProofStale,
    /// No proof packet is captured.
    ManifestProofMissing,
    /// The owner sign-off is missing.
    OwnerSignoffMissing,
    /// The waiver relied on has expired.
    WaiverExpired,
}

impl ManifestReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ReleaseLinkMissing,
        Self::ReleaseLinkStale,
        Self::ReleaseParityBroken,
        Self::UndisclosedResidualDependency,
        Self::GuardrailUnsatisfied,
        Self::ManifestProofStale,
        Self::ManifestProofMissing,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseLinkMissing => "release_link_missing",
            Self::ReleaseLinkStale => "release_link_stale",
            Self::ReleaseParityBroken => "release_parity_broken",
            Self::UndisclosedResidualDependency => "undisclosed_residual_dependency",
            Self::GuardrailUnsatisfied => "guardrail_unsatisfied",
            Self::ManifestProofStale => "manifest_proof_stale",
            Self::ManifestProofMissing => "manifest_proof_missing",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Precedence: lower is worse and wins when several reasons are active.
    const fn precedence(self) -> u8 {
        match self.state_group() {
            ManifestState::NarrowedParity => 0,
            ManifestState::NarrowedReleaseLink => 1,
            ManifestState::NarrowedDisclosure => 2,
            ManifestState::NarrowedGuardrail => 3,
            _ => 4,
        }
    }

    /// The narrowing state this reason maps to.
    pub const fn state_group(self) -> ManifestState {
        match self {
            Self::ReleaseParityBroken => ManifestState::NarrowedParity,
            Self::ReleaseLinkMissing | Self::ReleaseLinkStale => ManifestState::NarrowedReleaseLink,
            Self::UndisclosedResidualDependency => ManifestState::NarrowedDisclosure,
            Self::GuardrailUnsatisfied => ManifestState::NarrowedGuardrail,
            Self::ManifestProofStale
            | Self::ManifestProofMissing
            | Self::OwnerSignoffMissing
            | Self::WaiverExpired => ManifestState::NarrowedStale,
        }
    }
}

/// An action a [`GuardrailRule`] recommends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestAction {
    /// Hold publication until the gap clears.
    HoldPublication,
    /// Link the manifest to release evidence.
    LinkReleaseEvidence,
    /// Refresh the stale release-train link.
    RefreshReleaseLink,
    /// Realign the declared label to the release evidence.
    RealignClaimToReleaseEvidence,
    /// Disclose the residual proprietary/hosted dependency.
    DiscloseResidualDependency,
    /// Satisfy the unsatisfied guardrail.
    SatisfyGuardrail,
    /// Refresh the manifest proof packet.
    RefreshManifestProof,
    /// Request the owner sign-off.
    RequestOwnerSignoff,
}

impl ManifestAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::HoldPublication,
        Self::LinkReleaseEvidence,
        Self::RefreshReleaseLink,
        Self::RealignClaimToReleaseEvidence,
        Self::DiscloseResidualDependency,
        Self::SatisfyGuardrail,
        Self::RefreshManifestProof,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::LinkReleaseEvidence => "link_release_evidence",
            Self::RefreshReleaseLink => "refresh_release_link",
            Self::RealignClaimToReleaseEvidence => "realign_claim_to_release_evidence",
            Self::DiscloseResidualDependency => "disclose_residual_dependency",
            Self::SatisfyGuardrail => "satisfy_guardrail",
            Self::RefreshManifestProof => "refresh_manifest_proof",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// Publication decision recorded by the register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationDecision {
    /// No manifest-layer stop rule fires; publication may proceed.
    Proceed,
    /// A manifest-layer stop rule fires; hold publication.
    Hold,
}

impl PublicationDecision {
    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::Hold => "hold",
        }
    }
}

/// The support class consistent with a boundary posture.
///
/// This mirrors the durability matrix's posture→support mapping so a manifest's
/// lane entry cannot publish a support class its posture does not earn.
const fn posture_support_class(posture: BoundaryPosture) -> SupportClass {
    match posture {
        BoundaryPosture::OpenLocalCore => SupportClass::OpenLocal,
        BoundaryPosture::OpenLocalWithManagedOptional => SupportClass::MixedOpenManaged,
        BoundaryPosture::SourceAvailableRestricted => SupportClass::Restricted,
        BoundaryPosture::ManagedService => SupportClass::Managed,
        BoundaryPosture::RestrictedBrand => SupportClass::Restricted,
    }
}

/// A manifest's binding to its release train (the release-link parity axis).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseLink {
    /// Stable release-train id.
    pub train_id: String,
    /// Reference to the release-train artifact.
    pub train_ref: String,
    /// The lifecycle label the release evidence supports for this family.
    pub train_label: LifecycleLabel,
    /// Reference to the canonical M5 evidence index entry.
    pub evidence_index_ref: String,
    /// Link state.
    pub link_state: ReleaseLinkState,
    /// Freshness state of the link evidence.
    pub slo_state: FreshnessSloState,
    /// Date the link was last reconciled (`null` when missing).
    pub linked_at: Option<String>,
}

impl ReleaseLink {
    /// True when the link is present (not missing).
    pub fn is_present(&self) -> bool {
        self.link_state != ReleaseLinkState::Missing
    }
}

/// One residual proprietary or hosted dependency a family still rests on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResidualDependency {
    /// Stable dependency id.
    pub dependency_id: String,
    /// Reference to the dependency (register entry, SBOM node, or service).
    pub dependency_ref: String,
    /// Dependency class.
    pub dependency_class: DependencyClass,
    /// The asset lane the dependency affects.
    pub affected_lane: AssetLane,
    /// One-line summary of the residual dependency.
    pub summary: String,
    /// True when the dependency is disclosed on the user/admin truth surfaces.
    pub disclosed: bool,
    /// Surfaces the disclosure is published on (Help/About, docs, procurement).
    pub disclosure_surface_refs: Vec<String>,
    /// True when a fork/replace path exists.
    pub replaceable: bool,
    /// Reference to the replacement/contingency plan.
    pub replacement_plan_ref: String,
}

/// One guardrail preserving a manifest's claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Guardrail {
    /// The guardrail kind.
    pub kind: GuardrailKind,
    /// Reference to the policy/check that governs the guardrail.
    pub guardrail_ref: String,
    /// Owning team or role.
    pub owner_ref: String,
    /// Satisfaction state.
    pub state: GuardrailState,
    /// Reviewable description.
    pub description: String,
}

/// One asset-lane entry within a manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestLaneEntry {
    /// The asset lane.
    pub asset_lane: AssetLane,
    /// How the lane is dispositioned for this family.
    pub disposition: LaneDisposition,
    /// Boundary posture.
    pub boundary_posture: BoundaryPosture,
    /// Support class published.
    pub support_class: SupportClass,
    /// True when this lane's ordinary local usefulness must remain open.
    pub must_remain_open: bool,
    /// One-line capability summary.
    pub capability_summary: String,
    /// Reference to the durability-matrix row that grounds this lane.
    pub durability_row_ref: String,
}

/// One versioned, per-family boundary manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryManifest {
    /// Stable manifest id.
    pub manifest_id: String,
    /// The M5 family this manifest governs.
    pub family: M5Family,
    /// Human-readable title.
    pub title: String,
    /// The manifest version (immutable once published).
    pub manifest_version: String,
    /// The prior manifest version, if any.
    pub prior_version: Option<String>,
    /// Date this manifest version was cut.
    pub as_of: String,
    /// Reference to the governed subject.
    pub subject_ref: String,
    /// One-line subject summary.
    pub subject_summary: String,
    /// True when this family is part of the release-blocking set.
    pub release_blocking: bool,
    /// The lifecycle/support label this manifest declares.
    pub declared_label: LifecycleLabel,
    /// Support class published for the family as a whole.
    pub support_class: SupportClass,
    /// Binding to the release train.
    pub release_link: ReleaseLink,
    /// Per-asset-lane disposition.
    pub lane_entries: Vec<ManifestLaneEntry>,
    /// Guardrails preserving the claim.
    pub guardrails: Vec<Guardrail>,
    /// Residual proprietary/hosted dependency disclosure.
    pub residual_dependencies: Vec<ResidualDependency>,
    /// Proof packet grounding the manifest.
    pub proof_packet: ProofPacket,
    /// Optional waiver holding a gap provisionally.
    pub waiver: Option<Waiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// State earned after narrowing.
    pub manifest_state: ManifestState,
    /// Active narrowing reasons.
    pub active_reasons: Vec<ManifestReason>,
    /// The label the manifest effectively publishes after narrowing.
    pub effective_label: LifecycleLabel,
    /// Surfaces that reuse this manifest (Help/About, docs, support/eval packets).
    pub surfaces: Vec<String>,
    /// Reviewable reason the manifest carries its state.
    pub rationale: String,
}

impl BoundaryManifest {
    /// True when the manifest is held by an unexpired waiver.
    pub fn is_waived(&self) -> bool {
        self.waiver.is_some() && !self.has_active_reason(ManifestReason::WaiverExpired)
    }

    /// True when the manifest carries the given active reason.
    pub fn has_active_reason(&self, reason: ManifestReason) -> bool {
        self.active_reasons.contains(&reason)
    }

    /// True when the manifest holds a published state.
    pub fn is_published(&self) -> bool {
        self.manifest_state == ManifestState::Published
    }

    /// True when the family declares a label at or above the cutline.
    pub fn declares_at_or_above_cutline(&self) -> bool {
        self.declared_label.is_at_or_above_cutline()
    }

    /// True when the declared label is greener than the release-train evidence.
    pub fn over_claims_release_evidence(&self) -> bool {
        self.declared_label.rank() > self.release_link.train_label.rank()
    }

    /// True when any residual dependency is undisclosed.
    pub fn has_undisclosed_dependency(&self) -> bool {
        self.residual_dependencies.iter().any(|d| !d.disclosed)
    }

    /// True when any guardrail is unsatisfied.
    pub fn has_unsatisfied_guardrail(&self) -> bool {
        self.guardrails
            .iter()
            .any(|g| g.state == GuardrailState::Unsatisfied)
    }

    /// The state implied by the active reasons and the declared label.
    pub fn computed_state(&self) -> ManifestState {
        if self.declared_label == LifecycleLabel::Withdrawn {
            return ManifestState::Withdrawn;
        }
        match self
            .active_reasons
            .iter()
            .min_by_key(|reason| reason.precedence())
        {
            None => ManifestState::Published,
            Some(reason) => reason.state_group(),
        }
    }

    /// The effective label implied by the state and the declared label.
    pub fn computed_effective_label(&self) -> LifecycleLabel {
        match self.computed_state() {
            ManifestState::Published => self.declared_label,
            ManifestState::Withdrawn => LifecycleLabel::Withdrawn,
            _ => {
                // Narrowing drops the family below the cutline: take the
                // less-supported of the declared label and beta.
                if self.declared_label.rank() <= LifecycleLabel::Beta.rank() {
                    self.declared_label
                } else {
                    LifecycleLabel::Beta
                }
            }
        }
    }

    /// True when the manifest may hold promotion: a release-blocking family,
    /// narrowed by a manifest-layer gap, declaring a label at or above the cutline,
    /// and not held by an unexpired waiver.
    fn holds_promotion(&self) -> bool {
        self.release_blocking
            && self.manifest_state.is_narrowed()
            && self.declares_at_or_above_cutline()
            && !self.is_waived()
    }

    /// True when the manifest is in release-link parity: linked, fresh, and not
    /// over-claiming the train evidence.
    pub fn is_in_release_parity(&self) -> bool {
        self.release_link.link_state == ReleaseLinkState::Linked
            && !self.over_claims_release_evidence()
    }
}

/// A closed stop-rule that gates publication on a narrowing reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuardrailRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The reason that triggers the rule.
    pub trigger_reason: ManifestReason,
    /// Declared labels the rule applies to.
    pub applies_to_labels: Vec<LifecycleLabel>,
    /// Default recommended action.
    pub default_action: ManifestAction,
    /// True when the rule holds publication.
    pub blocks_publication: bool,
    /// Reviewable rationale.
    pub rationale: String,
}

/// The launch cutline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestCutline {
    /// The cutline level (`stable`).
    pub cutline_level: LifecycleLabel,
    /// Labels at or above the cutline.
    pub above_cutline_levels: Vec<LifecycleLabel>,
    /// Labels below the cutline.
    pub below_cutline_levels: Vec<LifecycleLabel>,
    /// Description.
    pub description: String,
}

/// Canonical source registers this manifest register binds together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceContractRefs {
    /// Open/local-boundary and upstream-durability matrix.
    pub durability_matrix_ref: String,
    /// Open-versus-paid boundary audit (release lane).
    pub open_paid_boundary_audit_ref: String,
    /// Canonical M5 evidence index.
    pub m5_evidence_index_ref: String,
    /// M5 claim-publication manifest register.
    pub claim_manifest_ref: String,
    /// Release-train / publish-target index.
    pub release_train_index_ref: String,
    /// Support/export packet register.
    pub support_export_ref: String,
}

/// Publication verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    /// Stable publication-gate id.
    pub publication_gate: String,
    /// Proceed/hold decision.
    pub decision: PublicationDecision,
    /// Firing rule ids.
    pub blocking_rule_ids: Vec<String>,
    /// Offending manifest ids.
    pub blocking_manifest_ids: Vec<String>,
    /// Reviewable rationale.
    pub rationale: String,
}

/// Cross-family release-link parity summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseLinkParity {
    /// Stable parity-gate id.
    pub parity_gate: String,
    /// Total families.
    pub families_total: usize,
    /// Families whose link is present, fresh, and in parity.
    pub families_in_parity: usize,
    /// Families whose link is missing or stale.
    pub families_link_broken: usize,
    /// Families over-claiming the release evidence.
    pub families_parity_broken: usize,
    /// True when every family carries a present release link.
    pub all_families_linked: bool,
    /// Reviewable rationale.
    pub rationale: String,
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestSummary {
    /// Total manifests.
    pub total_manifests: usize,
    /// Published manifests.
    pub manifests_published: usize,
    /// Narrowed manifests.
    pub manifests_narrowed: usize,
    /// Manifests in the `published` state.
    pub state_published: usize,
    /// Manifests in the `narrowed_release_link` state.
    pub state_narrowed_release_link: usize,
    /// Manifests in the `narrowed_parity` state.
    pub state_narrowed_parity: usize,
    /// Manifests in the `narrowed_disclosure` state.
    pub state_narrowed_disclosure: usize,
    /// Manifests in the `narrowed_guardrail` state.
    pub state_narrowed_guardrail: usize,
    /// Manifests in the `narrowed_stale` state.
    pub state_narrowed_stale: usize,
    /// Manifests in the `withdrawn` state.
    pub state_withdrawn: usize,
    /// Release-blocking manifests.
    pub release_blocking_total: usize,
    /// Release-blocking manifests that are narrowed.
    pub release_blocking_narrowed: usize,
    /// Manifests held by an active waiver.
    pub manifests_on_active_waiver: usize,
    /// Total residual dependencies disclosed.
    pub total_residual_dependencies: usize,
    /// Residual dependencies that are disclosed.
    pub residual_dependencies_disclosed: usize,
    /// Residual dependencies that are undisclosed.
    pub residual_dependencies_undisclosed: usize,
    /// Total guardrails.
    pub total_guardrails: usize,
    /// Unsatisfied guardrails.
    pub guardrails_unsatisfied: usize,
    /// Manifests carrying a present release link.
    pub manifests_linked: usize,
    /// Total active narrowing reasons.
    pub total_active_reasons: usize,
    /// Distinct rules firing.
    pub rules_firing: usize,
}

/// The typed register of versioned, per-family boundary manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryManifestRegister {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register id.
    pub register_id: String,
    /// Lifecycle status of this artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// Date the register was last reconciled.
    pub as_of: String,
    /// Canonical source registers.
    pub source_contract_refs: SourceContractRefs,
    /// Launch cutline.
    pub manifest_cutline: ManifestCutline,
    /// Closed family vocabulary.
    pub families: Vec<M5Family>,
    /// Closed asset-lane vocabulary.
    pub asset_lanes: Vec<AssetLane>,
    /// Closed boundary-posture vocabulary.
    pub boundary_postures: Vec<BoundaryPosture>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed lane-disposition vocabulary.
    pub lane_dispositions: Vec<LaneDisposition>,
    /// Closed guardrail-kind vocabulary.
    pub guardrail_kinds: Vec<GuardrailKind>,
    /// Closed dependency-class vocabulary.
    pub dependency_classes: Vec<DependencyClass>,
    /// Closed manifest-state vocabulary.
    pub manifest_states: Vec<ManifestState>,
    /// Closed manifest-reason vocabulary.
    pub manifest_reasons: Vec<ManifestReason>,
    /// Closed manifest-action vocabulary.
    pub manifest_actions: Vec<ManifestAction>,
    /// Stop rules.
    pub rules: Vec<GuardrailRule>,
    /// Per-family manifests.
    pub manifests: Vec<BoundaryManifest>,
    /// Cross-family release-link parity summary.
    pub release_link_parity: ReleaseLinkParity,
    /// Publication verdict.
    pub publication: Publication,
    /// Summary counts.
    pub summary: ManifestSummary,
}

impl BoundaryManifestRegister {
    /// Returns the manifest with the given id.
    pub fn manifest(&self, manifest_id: &str) -> Option<&BoundaryManifest> {
        self.manifests.iter().find(|m| m.manifest_id == manifest_id)
    }

    /// Returns the manifest for a family.
    pub fn manifest_for_family(&self, family: M5Family) -> Option<&BoundaryManifest> {
        self.manifests.iter().find(|m| m.family == family)
    }

    /// Returns the published manifests.
    pub fn manifests_published(&self) -> Vec<&BoundaryManifest> {
        self.manifests.iter().filter(|m| m.is_published()).collect()
    }

    /// Returns the narrowed manifests.
    pub fn manifests_narrowed(&self) -> Vec<&BoundaryManifest> {
        self.manifests
            .iter()
            .filter(|m| m.manifest_state.is_narrowed())
            .collect()
    }

    /// Returns the rule with the given trigger reason, if any.
    fn rule_for(&self, reason: ManifestReason) -> Option<&GuardrailRule> {
        self.rules.iter().find(|rule| rule.trigger_reason == reason)
    }

    /// Recomputes the firing rule ids: a blocking rule fires when a
    /// promotion-holding manifest carries its trigger reason at an applicable label.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for rule in &self.rules {
            if !rule.blocks_publication {
                continue;
            }
            let fires = self.manifests.iter().any(|m| {
                m.holds_promotion()
                    && m.has_active_reason(rule.trigger_reason)
                    && rule.applies_to_labels.contains(&m.declared_label)
            });
            if fires {
                ids.insert(rule.rule_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the offending manifest ids: promotion-holding manifests carrying a
    /// reason watched by a firing blocking rule.
    pub fn computed_blocking_manifest_ids(&self) -> Vec<String> {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for m in &self.manifests {
            if !m.holds_promotion() {
                continue;
            }
            let blocked = m.active_reasons.iter().any(|reason| {
                self.rule_for(*reason).is_some_and(|rule| {
                    rule.blocks_publication && rule.applies_to_labels.contains(&m.declared_label)
                })
            });
            if blocked {
                ids.insert(m.manifest_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the publication decision.
    pub fn computed_decision(&self) -> PublicationDecision {
        if self.computed_blocking_manifest_ids().is_empty() {
            PublicationDecision::Proceed
        } else {
            PublicationDecision::Hold
        }
    }

    /// Recomputes the cross-family release-link parity summary.
    pub fn computed_release_link_parity(&self) -> ReleaseLinkParity {
        let link_broken = self
            .manifests
            .iter()
            .filter(|m| {
                matches!(
                    m.release_link.link_state,
                    ReleaseLinkState::Missing | ReleaseLinkState::Stale
                )
            })
            .count();
        let parity_broken = self
            .manifests
            .iter()
            .filter(|m| {
                m.release_link.link_state == ReleaseLinkState::ParityBroken
                    || m.over_claims_release_evidence()
            })
            .count();
        ReleaseLinkParity {
            parity_gate: self.release_link_parity.parity_gate.clone(),
            families_total: self.manifests.len(),
            families_in_parity: self
                .manifests
                .iter()
                .filter(|m| m.is_in_release_parity())
                .count(),
            families_link_broken: link_broken,
            families_parity_broken: parity_broken,
            all_families_linked: self.manifests.iter().all(|m| m.release_link.is_present()),
            rationale: self.release_link_parity.rationale.clone(),
        }
    }

    /// Recomputes the summary block from the manifests.
    pub fn computed_summary(&self) -> ManifestSummary {
        let count_state = |state: ManifestState| {
            self.manifests
                .iter()
                .filter(|m| m.manifest_state == state)
                .count()
        };
        let deps: Vec<&ResidualDependency> = self
            .manifests
            .iter()
            .flat_map(|m| m.residual_dependencies.iter())
            .collect();
        let guardrails: Vec<&Guardrail> = self
            .manifests
            .iter()
            .flat_map(|m| m.guardrails.iter())
            .collect();
        ManifestSummary {
            total_manifests: self.manifests.len(),
            manifests_published: self.manifests_published().len(),
            manifests_narrowed: self.manifests_narrowed().len(),
            state_published: count_state(ManifestState::Published),
            state_narrowed_release_link: count_state(ManifestState::NarrowedReleaseLink),
            state_narrowed_parity: count_state(ManifestState::NarrowedParity),
            state_narrowed_disclosure: count_state(ManifestState::NarrowedDisclosure),
            state_narrowed_guardrail: count_state(ManifestState::NarrowedGuardrail),
            state_narrowed_stale: count_state(ManifestState::NarrowedStale),
            state_withdrawn: count_state(ManifestState::Withdrawn),
            release_blocking_total: self.manifests.iter().filter(|m| m.release_blocking).count(),
            release_blocking_narrowed: self
                .manifests
                .iter()
                .filter(|m| m.release_blocking && m.manifest_state.is_narrowed())
                .count(),
            manifests_on_active_waiver: self.manifests.iter().filter(|m| m.is_waived()).count(),
            total_residual_dependencies: deps.len(),
            residual_dependencies_disclosed: deps.iter().filter(|d| d.disclosed).count(),
            residual_dependencies_undisclosed: deps.iter().filter(|d| !d.disclosed).count(),
            total_guardrails: guardrails.len(),
            guardrails_unsatisfied: guardrails
                .iter()
                .filter(|g| g.state == GuardrailState::Unsatisfied)
                .count(),
            manifests_linked: self
                .manifests
                .iter()
                .filter(|m| m.release_link.is_present())
                .count(),
            total_active_reasons: self.manifests.iter().map(|m| m.active_reasons.len()).sum(),
            rules_firing: self.computed_blocking_rule_ids().len(),
        }
    }

    /// A copy-safe projection for reuse by Help/About, docs publication, support
    /// exports, and evaluation packets. It carries only the family, version, declared
    /// and effective labels, state, parity, active reasons, and surfaces — never the
    /// detailed release-link, dependency, and proof internals.
    pub fn reuse_projection(&self) -> Vec<ManifestReuseRow> {
        self.manifests
            .iter()
            .map(|m| ManifestReuseRow {
                manifest_id: m.manifest_id.clone(),
                family: m.family,
                manifest_version: m.manifest_version.clone(),
                declared_label: m.declared_label,
                effective_label: m.effective_label,
                support_class: m.support_class,
                manifest_state: m.manifest_state,
                release_blocking: m.release_blocking,
                in_release_parity: m.is_in_release_parity(),
                undisclosed_dependencies: m
                    .residual_dependencies
                    .iter()
                    .filter(|d| !d.disclosed)
                    .count(),
                active_reasons: m.active_reasons.clone(),
                surfaces: m.surfaces.clone(),
            })
            .collect()
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<RegisterViolation> {
        let mut v = Vec::new();

        if self.schema_version != M5_VERSIONED_BOUNDARY_MANIFESTS_SCHEMA_VERSION {
            v.push(RegisterViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_VERSIONED_BOUNDARY_MANIFESTS_RECORD_KIND {
            v.push(RegisterViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }

        self.validate_vocabularies(&mut v);

        if self.manifests.is_empty() {
            v.push(RegisterViolation::EmptyRegister);
        }

        // Every family must be covered by exactly one manifest.
        for family in M5Family::ALL {
            let count = self.manifests.iter().filter(|m| m.family == family).count();
            if count == 0 {
                v.push(RegisterViolation::FamilyUncovered { family });
            } else if count > 1 {
                v.push(RegisterViolation::FamilyDuplicated { family });
            }
        }

        // Every reason must have a stop rule.
        for reason in ManifestReason::ALL {
            if self.rule_for(reason).is_none() {
                v.push(RegisterViolation::ReasonUncoveredByRule { reason });
            }
        }

        let mut seen = BTreeSet::new();
        for m in &self.manifests {
            self.validate_manifest(m, &mut seen, &mut v);
        }

        // Verdict, parity, and summary coherence.
        if self.publication.decision != self.computed_decision() {
            v.push(RegisterViolation::PublicationDecisionInconsistent);
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            v.push(RegisterViolation::PublicationBlockingRulesMismatch);
        }
        if self.publication.blocking_manifest_ids != self.computed_blocking_manifest_ids() {
            v.push(RegisterViolation::PublicationBlockingManifestsMismatch);
        }
        if self.release_link_parity != self.computed_release_link_parity() {
            v.push(RegisterViolation::ReleaseLinkParityMismatch);
        }
        if self.summary != self.computed_summary() {
            v.push(RegisterViolation::SummaryMismatch);
        }

        v
    }

    fn validate_vocabularies(&self, v: &mut Vec<RegisterViolation>) {
        if self.families != M5Family::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch { field: "families" });
        }
        if self.asset_lanes != AssetLane::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "asset_lanes",
            });
        }
        if self.boundary_postures != BoundaryPosture::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "boundary_postures",
            });
        }
        if self.support_classes != SupportClass::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "support_classes",
            });
        }
        if self.lane_dispositions != LaneDisposition::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "lane_dispositions",
            });
        }
        if self.guardrail_kinds != GuardrailKind::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "guardrail_kinds",
            });
        }
        if self.dependency_classes != DependencyClass::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "dependency_classes",
            });
        }
        if self.manifest_states != ManifestState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "manifest_states",
            });
        }
        if self.manifest_reasons != ManifestReason::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "manifest_reasons",
            });
        }
        if self.manifest_actions != ManifestAction::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "manifest_actions",
            });
        }
        if self.manifest_cutline.cutline_level != LifecycleLabel::Stable {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "manifest_cutline",
            });
        }
    }

    fn validate_manifest(
        &self,
        m: &BoundaryManifest,
        seen: &mut BTreeSet<String>,
        v: &mut Vec<RegisterViolation>,
    ) {
        for (field, value) in [
            ("manifest_id", &m.manifest_id),
            ("title", &m.title),
            ("manifest_version", &m.manifest_version),
            ("subject_ref", &m.subject_ref),
            ("subject_summary", &m.subject_summary),
            ("rationale", &m.rationale),
        ] {
            if value.trim().is_empty() {
                v.push(RegisterViolation::EmptyField {
                    manifest_id: m.manifest_id.clone(),
                    field_name: field,
                });
            }
        }
        if !seen.insert(m.manifest_id.clone()) {
            v.push(RegisterViolation::DuplicateManifestId {
                manifest_id: m.manifest_id.clone(),
            });
        }
        if m.lane_entries.is_empty() {
            v.push(RegisterViolation::ManifestMissingLaneEntries {
                manifest_id: m.manifest_id.clone(),
            });
        }
        if m.surfaces.is_empty() {
            v.push(RegisterViolation::ManifestMissingSurfaces {
                manifest_id: m.manifest_id.clone(),
            });
        }

        self.validate_guardrails(m, v);
        self.validate_lane_entries(m, v);
        self.validate_reason_evidence(m, v);
        self.validate_state_and_label(m, v);
    }

    fn validate_guardrails(&self, m: &BoundaryManifest, v: &mut Vec<RegisterViolation>) {
        // Every guardrail kind must be declared exactly once: a manifest may never
        // omit the asset-lane-detail or residual-dependency guardrails and so cannot
        // publish vague "open core" copy.
        for kind in GuardrailKind::ALL {
            let count = m.guardrails.iter().filter(|g| g.kind == kind).count();
            if count != 1 {
                v.push(RegisterViolation::GuardrailKindNotDeclaredOnce {
                    manifest_id: m.manifest_id.clone(),
                    kind,
                });
            }
        }
    }

    fn validate_lane_entries(&self, m: &BoundaryManifest, v: &mut Vec<RegisterViolation>) {
        let mut lanes = BTreeSet::new();
        for entry in &m.lane_entries {
            if !lanes.insert(entry.asset_lane) {
                v.push(RegisterViolation::DuplicateLaneEntry {
                    manifest_id: m.manifest_id.clone(),
                    lane: entry.asset_lane,
                });
            }
            if entry.support_class != posture_support_class(entry.boundary_posture) {
                v.push(RegisterViolation::LaneSupportClassPostureMismatch {
                    manifest_id: m.manifest_id.clone(),
                    lane: entry.asset_lane,
                });
            }
            if !entry.disposition.is_consistent_with(entry.boundary_posture) {
                v.push(RegisterViolation::LaneDispositionPostureMismatch {
                    manifest_id: m.manifest_id.clone(),
                    lane: entry.asset_lane,
                });
            }
            if entry.must_remain_open && !entry.boundary_posture.is_open_baseline() {
                v.push(RegisterViolation::MustRemainOpenLaneBlurred {
                    manifest_id: m.manifest_id.clone(),
                    lane: entry.asset_lane,
                });
            }
            if entry.durability_row_ref.trim().is_empty() {
                v.push(RegisterViolation::LaneMissingDurabilityRef {
                    manifest_id: m.manifest_id.clone(),
                    lane: entry.asset_lane,
                });
            }
        }
        // Every residual dependency must affect a lane the manifest actually lists.
        for dep in &m.residual_dependencies {
            if !lanes.contains(&dep.affected_lane) {
                v.push(RegisterViolation::ResidualDependencyLaneUnknown {
                    manifest_id: m.manifest_id.clone(),
                    dependency_id: dep.dependency_id.clone(),
                });
            }
            if dep.disclosed && dep.disclosure_surface_refs.is_empty() {
                v.push(RegisterViolation::DisclosedDependencyMissingSurface {
                    manifest_id: m.manifest_id.clone(),
                    dependency_id: dep.dependency_id.clone(),
                });
            }
        }
    }

    /// Every active reason must be justified by the manifest's own fields, and every
    /// structural gap must surface its reason.
    fn validate_reason_evidence(&self, m: &BoundaryManifest, v: &mut Vec<RegisterViolation>) {
        let link_missing = m.release_link.link_state == ReleaseLinkState::Missing;
        let link_stale = m.release_link.link_state == ReleaseLinkState::Stale;
        let parity_broken = m.release_link.link_state == ReleaseLinkState::ParityBroken
            || m.over_claims_release_evidence();
        let undisclosed = m.has_undisclosed_dependency();
        let guardrail_gap = m.has_unsatisfied_guardrail();
        let proof_stale = m.proof_packet.slo_state == FreshnessSloState::Breached;
        let proof_missing = m.proof_packet.slo_state == FreshnessSloState::Missing;
        let signoff_missing = !m.owner_signoff.signed_off;

        // reason present ⇒ justified
        for reason in &m.active_reasons {
            let justified = match reason {
                ManifestReason::ReleaseLinkMissing => link_missing,
                ManifestReason::ReleaseLinkStale => link_stale,
                ManifestReason::ReleaseParityBroken => parity_broken,
                ManifestReason::UndisclosedResidualDependency => undisclosed,
                ManifestReason::GuardrailUnsatisfied => guardrail_gap,
                ManifestReason::ManifestProofStale => proof_stale,
                ManifestReason::ManifestProofMissing => proof_missing,
                ManifestReason::OwnerSignoffMissing => signoff_missing,
                ManifestReason::WaiverExpired => m.waiver.is_some(),
            };
            if !justified {
                v.push(RegisterViolation::ReasonNotJustified {
                    manifest_id: m.manifest_id.clone(),
                    reason: *reason,
                });
            }
        }

        // structural gap ⇒ reason present (so a gap can never hide).
        let require = |present: bool, reason: ManifestReason, v: &mut Vec<RegisterViolation>| {
            if present && !m.has_active_reason(reason) {
                v.push(RegisterViolation::GapWithoutReason {
                    manifest_id: m.manifest_id.clone(),
                    reason,
                });
            }
        };
        require(link_missing, ManifestReason::ReleaseLinkMissing, v);
        require(link_stale, ManifestReason::ReleaseLinkStale, v);
        require(parity_broken, ManifestReason::ReleaseParityBroken, v);
        require(
            undisclosed,
            ManifestReason::UndisclosedResidualDependency,
            v,
        );
        require(guardrail_gap, ManifestReason::GuardrailUnsatisfied, v);
        require(proof_stale, ManifestReason::ManifestProofStale, v);
        require(proof_missing, ManifestReason::ManifestProofMissing, v);
        require(signoff_missing, ManifestReason::OwnerSignoffMissing, v);
    }

    fn validate_state_and_label(&self, m: &BoundaryManifest, v: &mut Vec<RegisterViolation>) {
        // published ⇒ no reasons; narrowed ⇒ at least one reason.
        if m.is_published() && !m.active_reasons.is_empty() {
            v.push(RegisterViolation::PublishedWithActiveReason {
                manifest_id: m.manifest_id.clone(),
            });
        }
        if m.manifest_state.is_narrowed() && m.active_reasons.is_empty() {
            v.push(RegisterViolation::NarrowedWithoutReason {
                manifest_id: m.manifest_id.clone(),
            });
        }
        // state must equal the state implied by the reasons.
        if m.manifest_state != m.computed_state() {
            v.push(RegisterViolation::StateReasonMismatch {
                manifest_id: m.manifest_id.clone(),
                declared: m.manifest_state,
                computed: m.computed_state(),
            });
        }
        // never widen: effective may not rank above declared.
        if m.effective_label.rank() > m.declared_label.rank() {
            v.push(RegisterViolation::EffectiveLabelExceedsDeclared {
                manifest_id: m.manifest_id.clone(),
            });
        }
        // parity ceiling: a published manifest may never declare greener than its
        // release evidence.
        if m.is_published() && m.over_claims_release_evidence() {
            v.push(RegisterViolation::PublishedOverClaimsReleaseEvidence {
                manifest_id: m.manifest_id.clone(),
            });
        }
        // effective must equal the computed effective label.
        if m.effective_label != m.computed_effective_label() {
            v.push(RegisterViolation::EffectiveLabelMismatch {
                manifest_id: m.manifest_id.clone(),
            });
        }
        // a narrowed manifest must drop below the cutline.
        if m.manifest_state.is_narrowed() && m.effective_label.is_at_or_above_cutline() {
            v.push(RegisterViolation::NarrowedAboveCutline {
                manifest_id: m.manifest_id.clone(),
            });
        }
    }
}

/// A copy-safe reuse projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestReuseRow {
    /// Manifest id.
    pub manifest_id: String,
    /// Family.
    pub family: M5Family,
    /// Manifest version.
    pub manifest_version: String,
    /// Declared label.
    pub declared_label: LifecycleLabel,
    /// Effective label after narrowing.
    pub effective_label: LifecycleLabel,
    /// Support class.
    pub support_class: SupportClass,
    /// Manifest state.
    pub manifest_state: ManifestState,
    /// Release-blocking flag.
    pub release_blocking: bool,
    /// True when the manifest is in release-link parity.
    pub in_release_parity: bool,
    /// Count of undisclosed residual dependencies.
    pub undisclosed_dependencies: usize,
    /// Active narrowing reasons.
    pub active_reasons: Vec<ManifestReason>,
    /// Reuse surfaces.
    pub surfaces: Vec<String>,
}

/// A validation violation for the versioned-boundary-manifest register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterViolation {
    /// Unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found.
        actual: u32,
    },
    /// Unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The register has no manifests.
    EmptyRegister,
    /// A family has no manifest.
    FamilyUncovered {
        /// Uncovered family.
        family: M5Family,
    },
    /// A family has more than one manifest.
    FamilyDuplicated {
        /// Duplicated family.
        family: M5Family,
    },
    /// A narrowing reason has no stop rule.
    ReasonUncoveredByRule {
        /// Uncovered reason.
        reason: ManifestReason,
    },
    /// A manifest id appears more than once.
    DuplicateManifestId {
        /// Duplicate id.
        manifest_id: String,
    },
    /// A required field is empty.
    EmptyField {
        /// Manifest id.
        manifest_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A manifest lists no asset-lane entries.
    ManifestMissingLaneEntries {
        /// Manifest id.
        manifest_id: String,
    },
    /// A manifest lists no reuse surfaces.
    ManifestMissingSurfaces {
        /// Manifest id.
        manifest_id: String,
    },
    /// A guardrail kind is not declared exactly once.
    GuardrailKindNotDeclaredOnce {
        /// Manifest id.
        manifest_id: String,
        /// Offending kind.
        kind: GuardrailKind,
    },
    /// A lane appears in more than one entry.
    DuplicateLaneEntry {
        /// Manifest id.
        manifest_id: String,
        /// Duplicated lane.
        lane: AssetLane,
    },
    /// A lane entry's support class disagrees with its boundary posture.
    LaneSupportClassPostureMismatch {
        /// Manifest id.
        manifest_id: String,
        /// Offending lane.
        lane: AssetLane,
    },
    /// A lane entry's disposition disagrees with its boundary posture.
    LaneDispositionPostureMismatch {
        /// Manifest id.
        manifest_id: String,
        /// Offending lane.
        lane: AssetLane,
    },
    /// A must-remain-open lane carries a non-open-baseline posture.
    MustRemainOpenLaneBlurred {
        /// Manifest id.
        manifest_id: String,
        /// Offending lane.
        lane: AssetLane,
    },
    /// A lane entry has no durability-matrix row reference.
    LaneMissingDurabilityRef {
        /// Manifest id.
        manifest_id: String,
        /// Offending lane.
        lane: AssetLane,
    },
    /// A residual dependency affects a lane the manifest does not list.
    ResidualDependencyLaneUnknown {
        /// Manifest id.
        manifest_id: String,
        /// Dependency id.
        dependency_id: String,
    },
    /// A disclosed dependency lists no disclosure surface.
    DisclosedDependencyMissingSurface {
        /// Manifest id.
        manifest_id: String,
        /// Dependency id.
        dependency_id: String,
    },
    /// An active reason is not justified by the manifest's fields.
    ReasonNotJustified {
        /// Manifest id.
        manifest_id: String,
        /// Offending reason.
        reason: ManifestReason,
    },
    /// A structural gap is present but its reason is not active.
    GapWithoutReason {
        /// Manifest id.
        manifest_id: String,
        /// Missing reason.
        reason: ManifestReason,
    },
    /// A published manifest carries an active reason.
    PublishedWithActiveReason {
        /// Manifest id.
        manifest_id: String,
    },
    /// A narrowed manifest carries no reason.
    NarrowedWithoutReason {
        /// Manifest id.
        manifest_id: String,
    },
    /// The manifest state disagrees with the active reasons.
    StateReasonMismatch {
        /// Manifest id.
        manifest_id: String,
        /// Declared state.
        declared: ManifestState,
        /// Computed state.
        computed: ManifestState,
    },
    /// The effective label ranks above the declared label.
    EffectiveLabelExceedsDeclared {
        /// Manifest id.
        manifest_id: String,
    },
    /// A published manifest declares a label greener than its release evidence.
    PublishedOverClaimsReleaseEvidence {
        /// Manifest id.
        manifest_id: String,
    },
    /// The effective label disagrees with the computed effective label.
    EffectiveLabelMismatch {
        /// Manifest id.
        manifest_id: String,
    },
    /// A narrowed manifest did not drop below the cutline.
    NarrowedAboveCutline {
        /// Manifest id.
        manifest_id: String,
    },
    /// The publication decision disagrees with the firing rules.
    PublicationDecisionInconsistent,
    /// The recorded blocking rule ids disagree with the computed set.
    PublicationBlockingRulesMismatch,
    /// The recorded blocking manifest ids disagree with the computed set.
    PublicationBlockingManifestsMismatch,
    /// The recorded release-link parity disagrees with the computed summary.
    ReleaseLinkParityMismatch,
    /// The summary counts disagree with the manifests.
    SummaryMismatch,
}

impl fmt::Display for RegisterViolation {
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
            Self::EmptyRegister => write!(f, "register has no manifests"),
            Self::FamilyUncovered { family } => {
                write!(f, "family {} has no manifest", family.as_str())
            }
            Self::FamilyDuplicated { family } => {
                write!(f, "family {} has more than one manifest", family.as_str())
            }
            Self::ReasonUncoveredByRule { reason } => {
                write!(f, "reason {} has no stop rule", reason.as_str())
            }
            Self::DuplicateManifestId { manifest_id } => {
                write!(f, "duplicate manifest id {manifest_id}")
            }
            Self::EmptyField {
                manifest_id,
                field_name,
            } => write!(f, "manifest {manifest_id} has empty field {field_name}"),
            Self::ManifestMissingLaneEntries { manifest_id } => {
                write!(f, "manifest {manifest_id} lists no asset-lane entries")
            }
            Self::ManifestMissingSurfaces { manifest_id } => {
                write!(f, "manifest {manifest_id} lists no reuse surfaces")
            }
            Self::GuardrailKindNotDeclaredOnce { manifest_id, kind } => write!(
                f,
                "manifest {manifest_id} does not declare guardrail {} exactly once",
                kind.as_str()
            ),
            Self::DuplicateLaneEntry { manifest_id, lane } => write!(
                f,
                "manifest {manifest_id} lists lane {} more than once",
                lane.as_str()
            ),
            Self::LaneSupportClassPostureMismatch { manifest_id, lane } => write!(
                f,
                "manifest {manifest_id} lane {} support class disagrees with its posture",
                lane.as_str()
            ),
            Self::LaneDispositionPostureMismatch { manifest_id, lane } => write!(
                f,
                "manifest {manifest_id} lane {} disposition disagrees with its posture",
                lane.as_str()
            ),
            Self::MustRemainOpenLaneBlurred { manifest_id, lane } => write!(
                f,
                "manifest {manifest_id} must-remain-open lane {} carries a non-open-baseline posture",
                lane.as_str()
            ),
            Self::LaneMissingDurabilityRef { manifest_id, lane } => write!(
                f,
                "manifest {manifest_id} lane {} has no durability-matrix row reference",
                lane.as_str()
            ),
            Self::ResidualDependencyLaneUnknown {
                manifest_id,
                dependency_id,
            } => write!(
                f,
                "manifest {manifest_id} residual dependency {dependency_id} affects a lane it does not list"
            ),
            Self::DisclosedDependencyMissingSurface {
                manifest_id,
                dependency_id,
            } => write!(
                f,
                "manifest {manifest_id} discloses dependency {dependency_id} but lists no disclosure surface"
            ),
            Self::ReasonNotJustified {
                manifest_id,
                reason,
            } => write!(
                f,
                "manifest {manifest_id} names reason {} which its fields do not justify",
                reason.as_str()
            ),
            Self::GapWithoutReason {
                manifest_id,
                reason,
            } => write!(
                f,
                "manifest {manifest_id} has a structural gap but does not name reason {}",
                reason.as_str()
            ),
            Self::PublishedWithActiveReason { manifest_id } => {
                write!(f, "published manifest {manifest_id} carries an active narrowing reason")
            }
            Self::NarrowedWithoutReason { manifest_id } => {
                write!(f, "narrowed manifest {manifest_id} names no reason")
            }
            Self::StateReasonMismatch {
                manifest_id,
                declared,
                computed,
            } => write!(
                f,
                "manifest {manifest_id} records state {} but its reasons imply {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::EffectiveLabelExceedsDeclared { manifest_id } => {
                write!(f, "manifest {manifest_id} effective label is wider than its declared label")
            }
            Self::PublishedOverClaimsReleaseEvidence { manifest_id } => write!(
                f,
                "published manifest {manifest_id} declares a label greener than its release evidence"
            ),
            Self::EffectiveLabelMismatch { manifest_id } => {
                write!(f, "manifest {manifest_id} effective label disagrees with its state")
            }
            Self::NarrowedAboveCutline { manifest_id } => {
                write!(f, "narrowed manifest {manifest_id} did not drop below the cutline")
            }
            Self::PublicationDecisionInconsistent => {
                write!(f, "publication decision disagrees with the firing rules")
            }
            Self::PublicationBlockingRulesMismatch => {
                write!(f, "publication blocking_rule_ids disagree with the computed set")
            }
            Self::PublicationBlockingManifestsMismatch => {
                write!(f, "publication blocking_manifest_ids disagree with the computed set")
            }
            Self::ReleaseLinkParityMismatch => {
                write!(f, "release_link_parity disagrees with the computed summary")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with the manifests"),
        }
    }
}

impl Error for RegisterViolation {}

/// Loads the embedded versioned-boundary-manifest register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`BoundaryManifestRegister`] — including when a manifest carries a token outside
/// any closed vocabulary.
pub fn current_m5_versioned_boundary_manifests(
) -> Result<BoundaryManifestRegister, serde_json::Error> {
    serde_json::from_str(M5_VERSIONED_BOUNDARY_MANIFESTS_JSON)
}

#[cfg(test)]
mod tests;
