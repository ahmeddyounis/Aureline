//! Canonical M5 ecosystem install-governance matrix with a non-inheriting
//! promotion gate that auto-narrows any underqualified artifact family.
//!
//! Where the shared compatibility scorecard speaks for individual imported
//! extensions and bridges, this module freezes the canonical **M5 artifact-family
//! matrix**: first-party framework packs, docs packs, local-model packs, signed
//! recipe packs, template artifacts, bridge-backed packages, side-loaded packages,
//! and mirrored/private-registry variants. Each [`FamilyGovernanceRow`] names the
//! family's source class, runtime origin, declared support class, compatibility
//! label, permission-manifest state, activation-budget band, lifecycle state,
//! evidence freshness, and rollback posture, then publishes a support class that no
//! input can exceed.
//!
//! The model is a release-control gate, not a badge store. The support class a
//! family may *publish* is derived deterministically: a family whose runtime origin
//! is unverified, whose evidence is stale, whose permission delta is unreviewed,
//! whose activation budget is exhausted, whose compatibility is unsupported, whose
//! rollback is incomplete, or that is quarantined cannot publish full support, and
//! its [`PromotionDecision`] records whether the gate promoted it, narrowed it to
//! best-effort or community support, or failed promotion and withheld the claim.
//! Because [`FamilyGovernanceRow::published_support_class`],
//! [`FamilyGovernanceRow::promotion_decision`], and the recomputed
//! [`FamilyGovernanceRow::narrowing_reasons`] are all validated against the gate,
//! registry, Help/About, release, and support surfaces can prove that
//! underqualified families narrow automatically before publication and that no
//! family publishes beyond what its own provenance, freshness, permission,
//! activation, compatibility, and rollback states support.
//!
//! Governance stays family-specific and provenance-bound. The packet pins the
//! artifact-family vocabulary and requires exactly one row per claimed family, so a
//! signed first-party framework pack never lends its trust to a side-loaded package
//! or a mirrored-registry variant, and no family inherits trust simply because it
//! looks first-party. Every family must declare its own runtime origin,
//! permission class, activation cost, compatibility, and rollback posture.
//!
//! The packet is checked in at
//! `artifacts/ecosystem/m5/m5-ecosystem-install-governance-matrix.json` and embedded
//! here, so this typed consumer and any CI gate agree on every family without a
//! cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no credential bodies, raw provider payloads, signing secrets, or mirror
//! tokens.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5 ecosystem install-governance matrix schema version.
pub const M5_ECOSYSTEM_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_ECOSYSTEM_GOVERNANCE_RECORD_KIND: &str = "m5_ecosystem_install_governance_matrix";

/// Repo-relative path to the checked-in packet.
pub const M5_ECOSYSTEM_GOVERNANCE_PATH: &str =
    "artifacts/ecosystem/m5/m5-ecosystem-install-governance-matrix.json";

/// Embedded checked-in packet JSON.
pub const M5_ECOSYSTEM_GOVERNANCE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-ecosystem-install-governance-matrix.json"
));

/// A marketed M5 artifact family the matrix makes claims about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactFamily {
    /// First-party framework pack.
    FirstPartyFrameworkPack,
    /// Documentation pack.
    DocsPack,
    /// Local-model pack.
    LocalModelPack,
    /// Signed recipe pack.
    SignedRecipePack,
    /// Template artifact.
    TemplateArtifact,
    /// Bridge-backed package.
    BridgeBackedPackage,
    /// Side-loaded package.
    SideLoadedPackage,
    /// Mirrored/private-registry variant.
    MirroredRegistryVariant,
}

impl ArtifactFamily {
    /// Every artifact family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::FirstPartyFrameworkPack,
        Self::DocsPack,
        Self::LocalModelPack,
        Self::SignedRecipePack,
        Self::TemplateArtifact,
        Self::BridgeBackedPackage,
        Self::SideLoadedPackage,
        Self::MirroredRegistryVariant,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyFrameworkPack => "first_party_framework_pack",
            Self::DocsPack => "docs_pack",
            Self::LocalModelPack => "local_model_pack",
            Self::SignedRecipePack => "signed_recipe_pack",
            Self::TemplateArtifact => "template_artifact",
            Self::BridgeBackedPackage => "bridge_backed_package",
            Self::SideLoadedPackage => "side_loaded_package",
            Self::MirroredRegistryVariant => "mirrored_registry_variant",
        }
    }
}

/// Support class of a family's published ecosystem claim.
///
/// Ordered low-to-high by [`SupportClass::rank`]: an [`SupportClass::Unsupported`]
/// family carries no claim, and a [`SupportClass::FullySupported`] family carries a
/// full, current, evidence-backed claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Full, current, provenance-backed support.
    FullySupported,
    /// Partial coverage; published as best-effort depth only.
    BestEffortSupported,
    /// Community-maintained depth only; below the first-party bar.
    CommunitySupported,
    /// Not supported; carries no positive claim.
    Unsupported,
}

impl SupportClass {
    /// Every support class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullySupported,
        Self::BestEffortSupported,
        Self::CommunitySupported,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullySupported => "fully_supported",
            Self::BestEffortSupported => "best_effort_supported",
            Self::CommunitySupported => "community_supported",
            Self::Unsupported => "unsupported",
        }
    }

    /// Monotonic rank; higher means a stronger claim.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Unsupported => 0,
            Self::CommunitySupported => 1,
            Self::BestEffortSupported => 2,
            Self::FullySupported => 3,
        }
    }

    /// The weaker (lower-rank) of two support classes.
    pub const fn min(self, other: Self) -> Self {
        if other.rank() < self.rank() {
            other
        } else {
            self
        }
    }
}

/// Runtime origin of an artifact family.
///
/// The origin caps the support class a family may publish, so a side-loaded or
/// community package can never inherit first-party trust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeOrigin {
    /// First-party signed build.
    FirstPartySigned,
    /// Partner signed build.
    PartnerSigned,
    /// Community signed build.
    CommunitySigned,
    /// Bridge-backed runtime adapter.
    BridgeRuntime,
    /// Local-model runtime.
    LocalModelRuntime,
    /// Unsigned, side-loaded artifact.
    UnsignedSideLoaded,
}

impl RuntimeOrigin {
    /// Every runtime origin, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FirstPartySigned,
        Self::PartnerSigned,
        Self::CommunitySigned,
        Self::BridgeRuntime,
        Self::LocalModelRuntime,
        Self::UnsignedSideLoaded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartySigned => "first_party_signed",
            Self::PartnerSigned => "partner_signed",
            Self::CommunitySigned => "community_signed",
            Self::BridgeRuntime => "bridge_runtime",
            Self::LocalModelRuntime => "local_model_runtime",
            Self::UnsignedSideLoaded => "unsigned_side_loaded",
        }
    }

    /// Highest support class this runtime origin permits a family to publish.
    pub const fn support_ceiling(self) -> SupportClass {
        match self {
            Self::FirstPartySigned | Self::PartnerSigned => SupportClass::FullySupported,
            Self::BridgeRuntime | Self::LocalModelRuntime => SupportClass::BestEffortSupported,
            Self::CommunitySigned | Self::UnsignedSideLoaded => SupportClass::CommunitySupported,
        }
    }

    /// Whether this origin raises the [`NarrowingReason::ProvenanceUnverified`]
    /// trigger.
    pub const fn is_provenance_unverified_trigger(self) -> bool {
        matches!(self, Self::UnsignedSideLoaded)
    }
}

/// Compatibility label of a family against the install target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityLabel {
    /// Compatible with the target.
    Compatible,
    /// Compatible but degraded; depth narrows to best-effort.
    DegradedCompatible,
    /// Requires a compatibility bridge; depth narrows to best-effort.
    CompatibilityBridgeRequired,
    /// Unsupported on the target; carries no claim.
    UnsupportedOnTarget,
}

impl CompatibilityLabel {
    /// Every compatibility label, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Compatible,
        Self::DegradedCompatible,
        Self::CompatibilityBridgeRequired,
        Self::UnsupportedOnTarget,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::DegradedCompatible => "degraded_compatible",
            Self::CompatibilityBridgeRequired => "compatibility_bridge_required",
            Self::UnsupportedOnTarget => "unsupported_on_target",
        }
    }

    /// Highest support class this compatibility label permits a family to publish.
    pub const fn support_ceiling(self) -> SupportClass {
        match self {
            Self::Compatible => SupportClass::FullySupported,
            Self::DegradedCompatible | Self::CompatibilityBridgeRequired => {
                SupportClass::BestEffortSupported
            }
            Self::UnsupportedOnTarget => SupportClass::Unsupported,
        }
    }

    /// Whether this label raises the [`NarrowingReason::CompatibilityUnsupported`]
    /// trigger.
    pub const fn is_unsupported_trigger(self) -> bool {
        matches!(self, Self::UnsupportedOnTarget)
    }
}

/// State of a family's permission manifest relative to the prior install.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionManifestState {
    /// No permission delta.
    Unchanged,
    /// Permissions were reduced.
    Reduced,
    /// Permissions were added and disclosed for review.
    AdditiveDisclosed,
    /// Permissions were expanded without review.
    ExpandedUnreviewed,
    /// The family carries no permission manifest.
    NotApplicable,
}

impl PermissionManifestState {
    /// Every permission-manifest state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Unchanged,
        Self::Reduced,
        Self::AdditiveDisclosed,
        Self::ExpandedUnreviewed,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::Reduced => "reduced",
            Self::AdditiveDisclosed => "additive_disclosed",
            Self::ExpandedUnreviewed => "expanded_unreviewed",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Highest support class this permission state permits a family to publish.
    pub const fn support_ceiling(self) -> SupportClass {
        match self {
            Self::Unchanged | Self::Reduced | Self::NotApplicable => SupportClass::FullySupported,
            Self::AdditiveDisclosed => SupportClass::BestEffortSupported,
            Self::ExpandedUnreviewed => SupportClass::CommunitySupported,
        }
    }

    /// Whether this state raises the
    /// [`NarrowingReason::PermissionExpansionUnreviewed`] trigger.
    pub const fn is_expansion_unreviewed_trigger(self) -> bool {
        matches!(self, Self::ExpandedUnreviewed)
    }
}

/// Activation-budget band for a family's managed-workspace runtime cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivationBudgetBand {
    /// Comfortably under budget.
    HealthyUnderBudget,
    /// Approaching the budget ceiling; depth narrows to best-effort.
    ApproachingCeiling,
    /// Over budget; depth narrows to community support.
    OverBudget,
    /// Budget cannot be established; depth narrows to best-effort.
    BudgetUnknown,
    /// No activation budget applies (for example, a local host).
    NotApplicable,
}

impl ActivationBudgetBand {
    /// Every activation-budget band, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HealthyUnderBudget,
        Self::ApproachingCeiling,
        Self::OverBudget,
        Self::BudgetUnknown,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HealthyUnderBudget => "healthy_under_budget",
            Self::ApproachingCeiling => "approaching_ceiling",
            Self::OverBudget => "over_budget",
            Self::BudgetUnknown => "budget_unknown",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Highest support class this activation band permits a family to publish.
    pub const fn support_ceiling(self) -> SupportClass {
        match self {
            Self::HealthyUnderBudget | Self::NotApplicable => SupportClass::FullySupported,
            Self::ApproachingCeiling | Self::BudgetUnknown => SupportClass::BestEffortSupported,
            Self::OverBudget => SupportClass::CommunitySupported,
        }
    }

    /// Whether this band raises the [`NarrowingReason::ActivationBudgetExceeded`]
    /// trigger.
    pub const fn is_exceeded_trigger(self) -> bool {
        matches!(self, Self::OverBudget)
    }
}

/// Lifecycle state of an installed or installable family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleState {
    /// Discoverable and installable.
    Available,
    /// Installed and active.
    Installed,
    /// Installed with an update available.
    UpdateAvailable,
    /// Installed but disabled by the user.
    Disabled,
    /// Quarantined pending review; carries no claim.
    Quarantined,
    /// Rolled back from a failed install/update; carries no claim.
    RolledBack,
    /// Retired from the catalog; community depth only.
    Retired,
}

impl LifecycleState {
    /// Every lifecycle state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Available,
        Self::Installed,
        Self::UpdateAvailable,
        Self::Disabled,
        Self::Quarantined,
        Self::RolledBack,
        Self::Retired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Installed => "installed",
            Self::UpdateAvailable => "update_available",
            Self::Disabled => "disabled",
            Self::Quarantined => "quarantined",
            Self::RolledBack => "rolled_back",
            Self::Retired => "retired",
        }
    }

    /// Highest support class this lifecycle state permits a family to publish.
    pub const fn support_ceiling(self) -> SupportClass {
        match self {
            Self::Available | Self::Installed | Self::UpdateAvailable | Self::Disabled => {
                SupportClass::FullySupported
            }
            Self::Retired => SupportClass::CommunitySupported,
            Self::Quarantined | Self::RolledBack => SupportClass::Unsupported,
        }
    }

    /// Whether this state raises the [`NarrowingReason::Quarantined`] trigger.
    ///
    /// Both an active quarantine and a rolled-back install withhold a live claim.
    pub const fn is_quarantined_trigger(self) -> bool {
        matches!(self, Self::Quarantined | Self::RolledBack)
    }
}

/// Freshness of a family's qualifying evidence relative to its freshness SLO.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// Evidence is current within its freshness SLO.
    Current,
    /// Evidence is present but past its freshness SLO.
    Stale,
    /// Evidence has expired and no longer backs a live claim.
    Expired,
    /// Evidence freshness cannot be established.
    Unknown,
}

impl EvidenceFreshness {
    /// Every freshness class, in declaration order.
    pub const ALL: [Self; 4] = [Self::Current, Self::Stale, Self::Expired, Self::Unknown];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::Unknown => "unknown",
        }
    }

    /// Whether the evidence is current within its freshness SLO.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }

    /// Highest support class this freshness alone permits a family to publish.
    pub const fn support_ceiling(self) -> SupportClass {
        match self {
            Self::Current => SupportClass::FullySupported,
            Self::Stale | Self::Unknown => SupportClass::BestEffortSupported,
            Self::Expired => SupportClass::CommunitySupported,
        }
    }

    /// Whether this freshness raises the [`NarrowingReason::EvidenceStale`] trigger.
    ///
    /// Stale and expired evidence both raise the trigger; `unknown` lowers the
    /// ceiling but is treated as a soft state, not a headline trigger.
    pub const fn is_stale_trigger(self) -> bool {
        matches!(self, Self::Stale | Self::Expired)
    }
}

/// Rollback posture for a family's install or update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackPosture {
    /// Exact reversal verified.
    ReversibleVerified,
    /// Reversal path declared but not verified; depth narrows to best-effort.
    ReversibleUnverified,
    /// Only a compensating reversal is available; depth narrows to best-effort.
    CompensatingOnly,
    /// No reversal is possible; depth narrows to community support.
    Irreversible,
    /// No rollback applies.
    NotApplicable,
}

impl RollbackPosture {
    /// Every rollback posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReversibleVerified,
        Self::ReversibleUnverified,
        Self::CompensatingOnly,
        Self::Irreversible,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReversibleVerified => "reversible_verified",
            Self::ReversibleUnverified => "reversible_unverified",
            Self::CompensatingOnly => "compensating_only",
            Self::Irreversible => "irreversible",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Highest support class this rollback posture permits a family to publish.
    pub const fn support_ceiling(self) -> SupportClass {
        match self {
            Self::ReversibleVerified | Self::NotApplicable => SupportClass::FullySupported,
            Self::ReversibleUnverified | Self::CompensatingOnly => {
                SupportClass::BestEffortSupported
            }
            Self::Irreversible => SupportClass::CommunitySupported,
        }
    }

    /// Whether this posture raises the [`NarrowingReason::RollbackIncomplete`]
    /// trigger.
    ///
    /// An unverified reversal and an irreversible install both raise the trigger;
    /// a compensating-only reversal lowers the ceiling but is a disclosed, accepted
    /// posture rather than a headline trigger.
    pub const fn is_incomplete_trigger(self) -> bool {
        matches!(self, Self::ReversibleUnverified | Self::Irreversible)
    }
}

/// A headline reason the governance gate narrows a family.
///
/// These are the canonical release-control triggers: unverified provenance, stale
/// evidence, an unreviewed permission expansion, an exceeded activation budget,
/// an unsupported compatibility target, an incomplete rollback, and a quarantine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// The runtime origin is unsigned/side-loaded.
    ProvenanceUnverified,
    /// The qualifying evidence is stale or expired.
    EvidenceStale,
    /// The permission manifest was expanded without review.
    PermissionExpansionUnreviewed,
    /// The activation budget is exceeded.
    ActivationBudgetExceeded,
    /// The family is unsupported on the install target.
    CompatibilityUnsupported,
    /// The rollback path is unverified or irreversible.
    RollbackIncomplete,
    /// The family is quarantined or rolled back.
    Quarantined,
}

impl NarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ProvenanceUnverified,
        Self::EvidenceStale,
        Self::PermissionExpansionUnreviewed,
        Self::ActivationBudgetExceeded,
        Self::CompatibilityUnsupported,
        Self::RollbackIncomplete,
        Self::Quarantined,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvenanceUnverified => "provenance_unverified",
            Self::EvidenceStale => "evidence_stale",
            Self::PermissionExpansionUnreviewed => "permission_expansion_unreviewed",
            Self::ActivationBudgetExceeded => "activation_budget_exceeded",
            Self::CompatibilityUnsupported => "compatibility_unsupported",
            Self::RollbackIncomplete => "rollback_incomplete",
            Self::Quarantined => "quarantined",
        }
    }
}

/// The action the governance gate takes on a family relative to full support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionDecision {
    /// No narrowing; the family publishes full support.
    Promote,
    /// Narrow the published claim to best-effort support.
    NarrowToBestEffort,
    /// Narrow the published claim to community support.
    NarrowToCommunity,
    /// Fail promotion and withhold the claim entirely.
    FailPromotion,
}

impl PromotionDecision {
    /// Every promotion decision, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Promote,
        Self::NarrowToBestEffort,
        Self::NarrowToCommunity,
        Self::FailPromotion,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Promote => "promote",
            Self::NarrowToBestEffort => "narrow_to_best_effort",
            Self::NarrowToCommunity => "narrow_to_community",
            Self::FailPromotion => "fail_promotion",
        }
    }

    /// The decision implied by a published support class.
    pub const fn for_published(support: SupportClass) -> Self {
        match support {
            SupportClass::FullySupported => Self::Promote,
            SupportClass::BestEffortSupported => Self::NarrowToBestEffort,
            SupportClass::CommunitySupported => Self::NarrowToCommunity,
            SupportClass::Unsupported => Self::FailPromotion,
        }
    }

    /// Whether the gate narrowed or withheld the family.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::Promote)
    }
}

/// One governance row for a marketed M5 artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyGovernanceRow {
    /// Stable family-governance id.
    pub family_id: String,
    /// Marketed M5 artifact family this row governs.
    pub artifact_family: ArtifactFamily,
    /// Runtime origin of the family.
    pub runtime_origin: RuntimeOrigin,
    /// Support class the family's own evidence claims, before the gate.
    pub declared_support_class: SupportClass,
    /// Support class actually published after the gate narrows the family.
    ///
    /// Must equal [`FamilyGovernanceRow::effective_support_class`].
    pub published_support_class: SupportClass,
    /// Compatibility label against the install target.
    pub compatibility_label: CompatibilityLabel,
    /// Permission-manifest state relative to the prior install.
    pub permission_manifest_state: PermissionManifestState,
    /// Activation-budget band.
    pub activation_budget_band: ActivationBudgetBand,
    /// Lifecycle state.
    pub lifecycle_state: LifecycleState,
    /// Freshness of the family's qualifying evidence.
    pub evidence_freshness: EvidenceFreshness,
    /// Rollback posture for the family's install/update.
    pub rollback_posture: RollbackPosture,
    /// Decision the gate takes; must equal the recomputed decision.
    pub promotion_decision: PromotionDecision,
    /// Headline narrowing reasons; must equal the recomputed set.
    #[serde(default)]
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Ref to the family's provenance/signature record.
    pub provenance_ref: String,
    /// Ref to the family's permission manifest.
    pub permission_manifest_ref: String,
    /// Ref to the family's activation-budget record.
    pub activation_budget_ref: String,
    /// Ref to the family's compatibility/downgrade story.
    pub compatibility_ref: String,
    /// Ref to the family's durable rollback path.
    pub rollback_ref: String,
    /// Ref binding this row into registry, Help/About, support, and release surfaces.
    pub support_export_ref: String,
    /// Additional source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl FamilyGovernanceRow {
    /// The support class the family's own evidence assessed, before environmental
    /// narrowing.
    pub fn capability_floor(&self) -> SupportClass {
        self.declared_support_class
    }

    /// The support class the gate permits this family to publish.
    ///
    /// Lowers the capability floor to the weakest ceiling implied by the runtime
    /// origin, evidence freshness, permission state, activation budget,
    /// compatibility label, rollback posture, and lifecycle state, so a family with
    /// unverified provenance, stale evidence, an unreviewed permission expansion, an
    /// exceeded activation budget, an unsupported target, an incomplete rollback, or
    /// an active quarantine can never publish full support.
    pub fn effective_support_class(&self) -> SupportClass {
        self.capability_floor()
            .min(self.runtime_origin.support_ceiling())
            .min(self.evidence_freshness.support_ceiling())
            .min(self.permission_manifest_state.support_ceiling())
            .min(self.activation_budget_band.support_ceiling())
            .min(self.compatibility_label.support_ceiling())
            .min(self.rollback_posture.support_ceiling())
            .min(self.lifecycle_state.support_ceiling())
    }

    /// The headline narrowing reasons recomputed from the family's observed states.
    pub fn computed_narrowing_reasons(&self) -> Vec<NarrowingReason> {
        let mut reasons = Vec::new();
        if self.runtime_origin.is_provenance_unverified_trigger() {
            reasons.push(NarrowingReason::ProvenanceUnverified);
        }
        if self.evidence_freshness.is_stale_trigger() {
            reasons.push(NarrowingReason::EvidenceStale);
        }
        if self
            .permission_manifest_state
            .is_expansion_unreviewed_trigger()
        {
            reasons.push(NarrowingReason::PermissionExpansionUnreviewed);
        }
        if self.activation_budget_band.is_exceeded_trigger() {
            reasons.push(NarrowingReason::ActivationBudgetExceeded);
        }
        if self.compatibility_label.is_unsupported_trigger() {
            reasons.push(NarrowingReason::CompatibilityUnsupported);
        }
        if self.rollback_posture.is_incomplete_trigger() {
            reasons.push(NarrowingReason::RollbackIncomplete);
        }
        if self.lifecycle_state.is_quarantined_trigger() {
            reasons.push(NarrowingReason::Quarantined);
        }
        reasons
    }

    /// The decision the gate must record for this family.
    pub fn required_decision(&self) -> PromotionDecision {
        PromotionDecision::for_published(self.effective_support_class())
    }

    /// Whether the family may publish full support.
    pub fn is_promotable(&self) -> bool {
        self.effective_support_class() == SupportClass::FullySupported
    }

    /// Whether the family carries its own non-empty provenance, permission,
    /// activation, compatibility, rollback, and support-export refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.provenance_ref.trim().is_empty()
            && !self.permission_manifest_ref.trim().is_empty()
            && !self.activation_budget_ref.trim().is_empty()
            && !self.compatibility_ref.trim().is_empty()
            && !self.rollback_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
    }

    /// Whether the stored published support class, decision, and narrowing reasons
    /// all agree with the recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_support_class == self.effective_support_class()
            && self.promotion_decision == self.required_decision()
            && self.narrowing_reasons == self.computed_narrowing_reasons()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5EcosystemGovernanceSummary {
    /// Total family rows.
    pub total_families: usize,
    /// Number of marketed families claimed.
    pub family_count: usize,
    /// Families published as fully supported.
    pub fully_supported_families: usize,
    /// Families published as best-effort supported.
    pub best_effort_families: usize,
    /// Families published as community supported.
    pub community_supported_families: usize,
    /// Families published as unsupported.
    pub unsupported_families: usize,
    /// Families that may publish full support.
    pub promotable_families: usize,
    /// Families the gate narrowed or withheld in any way.
    pub narrowed_families: usize,
    /// Families the gate failed promotion on.
    pub failed_promotion_families: usize,
    /// Families with current evidence freshness.
    pub current_freshness_families: usize,
    /// Families over their activation budget.
    pub over_budget_families: usize,
    /// Families with unverified provenance.
    pub provenance_unverified_families: usize,
    /// Families that are quarantined or rolled back.
    pub quarantined_families: usize,
    /// Families carrying at least one narrowing reason.
    pub families_with_narrowing_reasons: usize,
}

/// A redaction-safe export row projected from a family governance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EcosystemGovernanceExportRow {
    /// Family-governance id.
    pub family_id: String,
    /// Artifact-family token.
    pub artifact_family: String,
    /// Runtime-origin token.
    pub runtime_origin: String,
    /// Declared support-class token.
    pub declared_support_class: String,
    /// Published support-class token.
    pub published_support_class: String,
    /// Compatibility-label token.
    pub compatibility_label: String,
    /// Permission-manifest-state token.
    pub permission_manifest_state: String,
    /// Activation-budget-band token.
    pub activation_budget_band: String,
    /// Lifecycle-state token.
    pub lifecycle_state: String,
    /// Evidence-freshness token.
    pub evidence_freshness: String,
    /// Rollback-posture token.
    pub rollback_posture: String,
    /// Promotion-decision token.
    pub promotion_decision: String,
    /// Narrowing-reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Provenance ref.
    pub provenance_ref: String,
    /// Whether the family publishes full support.
    pub publication_ready: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EcosystemGovernanceExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub families: Vec<M5EcosystemGovernanceExportRow>,
    /// Whether every family's published claim and decision agree with the gate.
    pub all_families_gate_consistent: bool,
    /// Families that may publish full support.
    pub promotable_count: usize,
    /// Families the gate narrowed or withheld.
    pub narrowed_count: usize,
    /// Families the gate failed promotion on.
    pub failed_promotion_count: usize,
}

/// The typed M5 ecosystem install-governance matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5EcosystemGovernanceMatrix {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Marketed families the packet claims; one row per family.
    pub artifact_families: Vec<ArtifactFamily>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed runtime-origin vocabulary.
    pub runtime_origins: Vec<RuntimeOrigin>,
    /// Closed compatibility-label vocabulary.
    pub compatibility_labels: Vec<CompatibilityLabel>,
    /// Closed permission-manifest-state vocabulary.
    pub permission_manifest_states: Vec<PermissionManifestState>,
    /// Closed activation-budget-band vocabulary.
    pub activation_budget_bands: Vec<ActivationBudgetBand>,
    /// Closed lifecycle-state vocabulary.
    pub lifecycle_states: Vec<LifecycleState>,
    /// Closed evidence-freshness vocabulary.
    pub evidence_freshness_classes: Vec<EvidenceFreshness>,
    /// Closed rollback-posture vocabulary.
    pub rollback_postures: Vec<RollbackPosture>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed promotion-decision vocabulary.
    pub promotion_decisions: Vec<PromotionDecision>,
    /// Governance rows, one per marketed family.
    #[serde(default)]
    pub families: Vec<FamilyGovernanceRow>,
    /// Summary counts.
    pub summary: M5EcosystemGovernanceSummary,
}

impl M5EcosystemGovernanceMatrix {
    /// Returns the row for a marketed family.
    pub fn family(&self, family: ArtifactFamily) -> Option<&FamilyGovernanceRow> {
        self.families.iter().find(|f| f.artifact_family == family)
    }

    /// Families that may publish full support.
    pub fn promotable_families(&self) -> impl Iterator<Item = &FamilyGovernanceRow> {
        self.families.iter().filter(|f| f.is_promotable())
    }

    /// Families the gate narrowed or withheld in any way.
    pub fn narrowed_families(&self) -> impl Iterator<Item = &FamilyGovernanceRow> {
        self.families
            .iter()
            .filter(|f| f.required_decision().is_narrowed())
    }

    /// Families the gate failed promotion on.
    pub fn failed_promotion_families(&self) -> impl Iterator<Item = &FamilyGovernanceRow> {
        self.families
            .iter()
            .filter(|f| f.required_decision() == PromotionDecision::FailPromotion)
    }

    /// Whether every family's stored published claim, decision, and reasons agree
    /// with the recomputed gate decision.
    pub fn all_families_gate_consistent(&self) -> bool {
        self.families.iter().all(|f| f.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5EcosystemGovernanceSummary {
        let count_published = |support: SupportClass| {
            self.families
                .iter()
                .filter(|f| f.published_support_class == support)
                .count()
        };
        M5EcosystemGovernanceSummary {
            total_families: self.families.len(),
            family_count: self.artifact_families.len(),
            fully_supported_families: count_published(SupportClass::FullySupported),
            best_effort_families: count_published(SupportClass::BestEffortSupported),
            community_supported_families: count_published(SupportClass::CommunitySupported),
            unsupported_families: count_published(SupportClass::Unsupported),
            promotable_families: self.promotable_families().count(),
            narrowed_families: self.narrowed_families().count(),
            failed_promotion_families: self.failed_promotion_families().count(),
            current_freshness_families: self
                .families
                .iter()
                .filter(|f| f.evidence_freshness.is_current())
                .count(),
            over_budget_families: self
                .families
                .iter()
                .filter(|f| f.activation_budget_band.is_exceeded_trigger())
                .count(),
            provenance_unverified_families: self
                .families
                .iter()
                .filter(|f| f.runtime_origin.is_provenance_unverified_trigger())
                .count(),
            quarantined_families: self
                .families
                .iter()
                .filter(|f| f.lifecycle_state.is_quarantined_trigger())
                .count(),
            families_with_narrowing_reasons: self
                .families
                .iter()
                .filter(|f| !f.narrowing_reasons.is_empty())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — the marketplace and
    /// registry, Help/About, docs/migration, support exports, and
    /// release/public-truth packets — render instead of restating M5 ecosystem
    /// install/lifecycle/activation status text by hand.
    pub fn export_projection(&self) -> M5EcosystemGovernanceExportProjection {
        let families = self
            .families
            .iter()
            .map(|f| M5EcosystemGovernanceExportRow {
                family_id: f.family_id.clone(),
                artifact_family: f.artifact_family.as_str().to_owned(),
                runtime_origin: f.runtime_origin.as_str().to_owned(),
                declared_support_class: f.declared_support_class.as_str().to_owned(),
                published_support_class: f.published_support_class.as_str().to_owned(),
                compatibility_label: f.compatibility_label.as_str().to_owned(),
                permission_manifest_state: f.permission_manifest_state.as_str().to_owned(),
                activation_budget_band: f.activation_budget_band.as_str().to_owned(),
                lifecycle_state: f.lifecycle_state.as_str().to_owned(),
                evidence_freshness: f.evidence_freshness.as_str().to_owned(),
                rollback_posture: f.rollback_posture.as_str().to_owned(),
                promotion_decision: f.promotion_decision.as_str().to_owned(),
                narrowing_reasons: f
                    .narrowing_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                provenance_ref: f.provenance_ref.clone(),
                publication_ready: f.is_promotable(),
                summary: format!(
                    "{}: origin {}, declared {}, published {} ({}), compatibility {}, permission {}, budget {}, lifecycle {}, rollback {}",
                    f.artifact_family.as_str(),
                    f.runtime_origin.as_str(),
                    f.declared_support_class.as_str(),
                    f.published_support_class.as_str(),
                    f.promotion_decision.as_str(),
                    f.compatibility_label.as_str(),
                    f.permission_manifest_state.as_str(),
                    f.activation_budget_band.as_str(),
                    f.lifecycle_state.as_str(),
                    f.rollback_posture.as_str()
                ),
            })
            .collect();
        M5EcosystemGovernanceExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            families,
            all_families_gate_consistent: self.all_families_gate_consistent(),
            promotable_count: self.promotable_families().count(),
            narrowed_count: self.narrowed_families().count(),
            failed_promotion_count: self.failed_promotion_families().count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5EcosystemGovernanceViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<ArtifactFamily> = self.artifact_families.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.families {
            if !seen_ids.insert(row.family_id.clone()) {
                violations.push(M5EcosystemGovernanceViolation::DuplicateFamilyId {
                    family_id: row.family_id.clone(),
                });
            }
            if !seen_families.insert(row.artifact_family) {
                violations.push(M5EcosystemGovernanceViolation::DuplicateFamilyRow {
                    family: row.artifact_family.as_str(),
                });
            }
            if !claimed.contains(&row.artifact_family) {
                violations.push(M5EcosystemGovernanceViolation::UnclaimedFamilyRow {
                    family_id: row.family_id.clone(),
                    family: row.artifact_family.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed family must carry its own row, so a family never inherits
        // trust from an adjacent certified one.
        for &family in &self.artifact_families {
            if !seen_families.contains(&family) {
                violations.push(M5EcosystemGovernanceViolation::MissingFamilyRow {
                    family: family.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5EcosystemGovernanceViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5EcosystemGovernanceViolation>) {
        if self.schema_version != M5_ECOSYSTEM_GOVERNANCE_SCHEMA_VERSION {
            violations.push(M5EcosystemGovernanceViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_ECOSYSTEM_GOVERNANCE_RECORD_KIND {
            violations.push(M5EcosystemGovernanceViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5EcosystemGovernanceViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "artifact_families",
                self.artifact_families == ArtifactFamily::ALL.to_vec(),
            ),
            (
                "support_classes",
                self.support_classes == SupportClass::ALL.to_vec(),
            ),
            (
                "runtime_origins",
                self.runtime_origins == RuntimeOrigin::ALL.to_vec(),
            ),
            (
                "compatibility_labels",
                self.compatibility_labels == CompatibilityLabel::ALL.to_vec(),
            ),
            (
                "permission_manifest_states",
                self.permission_manifest_states == PermissionManifestState::ALL.to_vec(),
            ),
            (
                "activation_budget_bands",
                self.activation_budget_bands == ActivationBudgetBand::ALL.to_vec(),
            ),
            (
                "lifecycle_states",
                self.lifecycle_states == LifecycleState::ALL.to_vec(),
            ),
            (
                "evidence_freshness_classes",
                self.evidence_freshness_classes == EvidenceFreshness::ALL.to_vec(),
            ),
            (
                "rollback_postures",
                self.rollback_postures == RollbackPosture::ALL.to_vec(),
            ),
            (
                "narrowing_reasons",
                self.narrowing_reasons == NarrowingReason::ALL.to_vec(),
            ),
            (
                "promotion_decisions",
                self.promotion_decisions == PromotionDecision::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5EcosystemGovernanceViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(
        &self,
        row: &FamilyGovernanceRow,
        violations: &mut Vec<M5EcosystemGovernanceViolation>,
    ) {
        for (field, value) in [
            ("family_id", &row.family_id),
            ("provenance_ref", &row.provenance_ref),
            ("permission_manifest_ref", &row.permission_manifest_ref),
            ("activation_budget_ref", &row.activation_budget_ref),
            ("compatibility_ref", &row.compatibility_ref),
            ("rollback_ref", &row.rollback_ref),
            ("support_export_ref", &row.support_export_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5EcosystemGovernanceViolation::EmptyField {
                    id: row.family_id.clone(),
                    field_name: field,
                });
            }
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.narrowing_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5EcosystemGovernanceViolation::DuplicateNarrowingReason {
                    family_id: row.family_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // The published support class must equal the gate's recomputed decision, so
        // a family can never publish beyond what its provenance, freshness,
        // permission, activation, compatibility, rollback, and lifecycle states
        // support.
        let effective = row.effective_support_class();
        if row.published_support_class != effective {
            violations.push(
                M5EcosystemGovernanceViolation::OverstatedPublishedSupportClass {
                    family_id: row.family_id.clone(),
                    published: row.published_support_class.as_str(),
                    computed: effective.as_str(),
                },
            );
        }

        // The recorded decision must match the published support class, so release
        // tooling proves underqualified families narrow automatically.
        let required = row.required_decision();
        if row.promotion_decision != required {
            violations.push(M5EcosystemGovernanceViolation::DecisionMismatch {
                family_id: row.family_id.clone(),
                declared: row.promotion_decision.as_str(),
                required: required.as_str(),
            });
        }

        // The recorded narrowing reasons must equal the reasons recomputed from the
        // observed states, so a narrowing can never be asserted or hidden by hand.
        let computed = row.computed_narrowing_reasons();
        if row.narrowing_reasons != computed {
            violations.push(M5EcosystemGovernanceViolation::NarrowingReasonsMismatch {
                family_id: row.family_id.clone(),
            });
        }

        // A promotable family must be genuinely clean: a fully-supporting runtime
        // origin, current freshness, a non-narrowing permission/activation/
        // compatibility/rollback/lifecycle state, an all-supported capability floor,
        // and no narrowing reason. This is the non-inheritance guardrail.
        if row.is_promotable()
            && (row.runtime_origin.support_ceiling() != SupportClass::FullySupported
                || !row.evidence_freshness.is_current()
                || row.permission_manifest_state.support_ceiling() != SupportClass::FullySupported
                || row.activation_budget_band.support_ceiling() != SupportClass::FullySupported
                || row.compatibility_label.support_ceiling() != SupportClass::FullySupported
                || row.rollback_posture.support_ceiling() != SupportClass::FullySupported
                || row.lifecycle_state.support_ceiling() != SupportClass::FullySupported
                || row.capability_floor() != SupportClass::FullySupported
                || !row.narrowing_reasons.is_empty())
        {
            violations.push(M5EcosystemGovernanceViolation::PromotedFamilyNotClean {
                family_id: row.family_id.clone(),
            });
        }
    }
}

/// A validation violation for the M5 ecosystem governance packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5EcosystemGovernanceViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A family-governance id appears more than once.
    DuplicateFamilyId {
        /// Duplicate family id.
        family_id: String,
    },
    /// A marketed family carries more than one row.
    DuplicateFamilyRow {
        /// Family token.
        family: &'static str,
    },
    /// A claimed marketed family has no row.
    MissingFamilyRow {
        /// Family token.
        family: &'static str,
    },
    /// A row covers a family the packet does not claim.
    UnclaimedFamilyRow {
        /// Row id.
        family_id: String,
        /// Family token.
        family: &'static str,
    },
    /// A row lists a narrowing reason more than once.
    DuplicateNarrowingReason {
        /// Row id.
        family_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A family publishes a support class beyond what its evidence supports.
    OverstatedPublishedSupportClass {
        /// Row id.
        family_id: String,
        /// Published support-class token.
        published: &'static str,
        /// Computed effective support-class token.
        computed: &'static str,
    },
    /// A family's decision disagrees with its published support class.
    DecisionMismatch {
        /// Row id.
        family_id: String,
        /// Declared decision token.
        declared: &'static str,
        /// Required decision token.
        required: &'static str,
    },
    /// A family's narrowing reasons disagree with the recomputed reasons.
    NarrowingReasonsMismatch {
        /// Row id.
        family_id: String,
    },
    /// A promotable family still carries a narrowing reason or a non-clean state.
    PromotedFamilyNotClean {
        /// Row id.
        family_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5EcosystemGovernanceViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateFamilyId { family_id } => {
                write!(f, "duplicate family id {family_id}")
            }
            Self::DuplicateFamilyRow { family } => {
                write!(f, "duplicate row for family {family}")
            }
            Self::MissingFamilyRow { family } => {
                write!(f, "missing row for claimed family {family}")
            }
            Self::UnclaimedFamilyRow { family_id, family } => {
                write!(f, "row {family_id} covers unclaimed family {family}")
            }
            Self::DuplicateNarrowingReason { family_id, reason } => {
                write!(f, "row {family_id} repeats narrowing reason {reason}")
            }
            Self::OverstatedPublishedSupportClass {
                family_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "row {family_id} publishes support class {published} but the gate computes {computed}"
                )
            }
            Self::DecisionMismatch {
                family_id,
                declared,
                required,
            } => {
                write!(
                    f,
                    "row {family_id} records decision {declared} but the gate requires {required}"
                )
            }
            Self::NarrowingReasonsMismatch { family_id } => {
                write!(
                    f,
                    "row {family_id} narrowing reasons disagree with the gate"
                )
            }
            Self::PromotedFamilyNotClean { family_id } => {
                write!(
                    f,
                    "row {family_id} is promotable but carries a narrowing reason or non-clean state"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for M5EcosystemGovernanceViolation {}

/// Loads the embedded M5 ecosystem governance matrix packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5EcosystemGovernanceMatrix`].
pub fn current_m5_ecosystem_governance_matrix(
) -> Result<M5EcosystemGovernanceMatrix, serde_json::Error> {
    serde_json::from_str(M5_ECOSYSTEM_GOVERNANCE_JSON)
}

#[cfg(test)]
mod tests;
