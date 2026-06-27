//! Frozen design-system contract matrix for the claimed M5 surfaces.
//!
//! Where the beta contract ([`crate::seeded_component_state_registry`] and friends) hardens
//! component-state, cue-family, screenshot-diff, and token-conformance truth, this module
//! freezes the *canonical object model* the design system ships: the governed inventory of
//! foundations/tokens, component contracts, reference layouts, state-semantic families, demo
//! fixtures, and proof packets, plus the claimed-surface coverage gate that keeps later M5
//! families from claiming shell parity without checked-in design-system evidence.
//!
//! The matrix has two coordinated halves. The **inventory** ([`M5DesignSystemContractObject`])
//! names every governed design-system object, its accountable owner, its first consumer, the
//! canonical artifact that materializes it, the release packet that keeps it current, and the
//! proof lane that blocks drift. The **coverage gate** ([`M5SurfaceContractCoverage`]) maps
//! each claimed M5 surface to the contract objects it must point at, resolves a
//! green/yellow/red traffic-light status (conformant / retest-pending / uncovered) from the
//! inventory, and resolves a release-gate decision the release/public-truth automation reads:
//! a surface that lacks a mapped contract object is *blocked* from Stable promotion and named
//! (never left invisible), while a surface whose mapped object has stale design-system proof
//! *auto-narrows* below Stable before promotion. Blocking gaps can be accepted under an
//! active, disclosed waiver scoped to a single object, which ships the surface at the waived
//! claim while its true status stays red.
//!
//! The matrix packet is the single M5 source of design-system contract truth: shell, help,
//! onboarding, presentation, the extension SDK, release-center, QA, support exports, and the
//! stable-claim matrix consume the same rows rather than re-describing component, token,
//! layout, or state behavior in local docs. The compact [`M5DesignSystemDashboard`]
//! projection is the published green/yellow/red scoreboard. Raw provider payloads,
//! credentials, secret material, and untranslated free-text prose stay outside the support
//! boundary.
//!
//! The three governed canonical-artifact shapes the inventory references — foundations/tokens
//! ([`M5FoundationsArtifact`]), component contracts ([`M5ComponentContractArtifact`]), and
//! reference layouts ([`M5ReferenceLayoutArtifact`]) — are first-class machine-readable
//! records here, so the schemas, gallery fixtures, and matrix all derive from one set of
//! types.
//!
//! - Matrix schema:
//!   [`schemas/design-system/m5-design-system-contract-matrix.schema.json`](../../../../../schemas/design-system/m5-design-system-contract-matrix.schema.json)
//! - Dashboard schema:
//!   [`schemas/design-system/m5-design-system-dashboard.schema.json`](../../../../../schemas/design-system/m5-design-system-dashboard.schema.json)
//! - Foundations schema:
//!   [`schemas/design-system/m5-foundations.schema.json`](../../../../../schemas/design-system/m5-foundations.schema.json)
//! - Component-contract schema:
//!   [`schemas/design-system/m5-component-contract.schema.json`](../../../../../schemas/design-system/m5-component-contract.schema.json)
//! - Reference-layout schema:
//!   [`schemas/design-system/m5-reference-layout.schema.json`](../../../../../schemas/design-system/m5-reference-layout.schema.json)
//! - Contract doc:
//!   [`docs/design-system/m5-design-system-contract-matrix.md`](../../../../../docs/design-system/m5-design-system-contract-matrix.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_component_contract_gallery, seeded_m5_design_system_contract_matrix,
    seeded_m5_design_system_contract_matrix_missing_object,
    seeded_m5_design_system_contract_matrix_stale_proof_retest_pending,
    seeded_m5_design_system_contract_matrix_waived_narrowed, seeded_m5_foundations_artifact,
    seeded_m5_reference_layout_artifact, M5_DESIGN_SYSTEM_CONTRACT_MATRIX_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_ui::density::DensityClass;
use aureline_ui::themes::AccessibilityPostureClass;
use aureline_ui::tokens::ThemeClass;

// The design-system beta contract owns the canonical state classes, cue families, and
// launch-surface classes the matrix reuses, so the contract objects map to the same
// vocabulary the rest of the crate publishes.
use crate::{CanonicalStateClass, CueFamilyClass, LaunchSurfaceClass};

/// Stable record-kind tag carried by [`M5DesignSystemContractMatrix`].
pub const M5_DESIGN_SYSTEM_CONTRACT_MATRIX_RECORD_KIND: &str = "m5_design_system_contract_matrix";

/// Schema version for M5 design-system contract-matrix packets.
pub const M5_DESIGN_SYSTEM_CONTRACT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`M5DesignSystemDashboard`].
pub const M5_DESIGN_SYSTEM_DASHBOARD_RECORD_KIND: &str = "m5_design_system_dashboard";

/// Schema version for M5 design-system contract dashboards.
pub const M5_DESIGN_SYSTEM_DASHBOARD_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`M5FoundationsArtifact`].
pub const M5_FOUNDATIONS_ARTIFACT_RECORD_KIND: &str = "m5_design_system_foundations";

/// Stable record-kind tag carried by [`M5ComponentContractArtifact`].
pub const M5_COMPONENT_CONTRACT_ARTIFACT_RECORD_KIND: &str = "m5_design_system_component_contract";

/// Stable record-kind tag carried by [`M5ReferenceLayoutArtifact`].
pub const M5_REFERENCE_LAYOUT_ARTIFACT_RECORD_KIND: &str = "m5_design_system_reference_layout";

/// Schema version shared by the canonical-artifact records.
pub const M5_DESIGN_SYSTEM_ARTIFACT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the matrix boundary schema.
pub const M5_DESIGN_SYSTEM_CONTRACT_MATRIX_SCHEMA_REF: &str =
    "schemas/design-system/m5-design-system-contract-matrix.schema.json";

/// Repo-relative path of the dashboard boundary schema.
pub const M5_DESIGN_SYSTEM_DASHBOARD_SCHEMA_REF: &str =
    "schemas/design-system/m5-design-system-dashboard.schema.json";

/// Repo-relative path of the foundations canonical-artifact schema.
pub const M5_FOUNDATIONS_SCHEMA_REF: &str = "schemas/design-system/m5-foundations.schema.json";

/// Repo-relative path of the component-contract canonical-artifact schema.
pub const M5_COMPONENT_CONTRACT_SCHEMA_REF: &str =
    "schemas/design-system/m5-component-contract.schema.json";

/// Repo-relative path of the reference-layout canonical-artifact schema.
pub const M5_REFERENCE_LAYOUT_SCHEMA_REF: &str =
    "schemas/design-system/m5-reference-layout.schema.json";

/// Repo-relative path of the matrix contract doc.
pub const M5_DESIGN_SYSTEM_CONTRACT_DOC_REF: &str =
    "docs/design-system/m5-design-system-contract-matrix.md";

/// Repo-relative path of the human-readable governance matrix.
pub const M5_DESIGN_SYSTEM_GOVERNANCE_REF: &str =
    "artifacts/design-system/m5-design-system-contract-governance.md";

/// Repo-relative path of the release-grade matrix support export — the proof lane that blocks
/// drift for every governed object.
pub const M5_DESIGN_SYSTEM_PROOF_REF: &str =
    "artifacts/release/m5-design-system-proof/support_export.json";

/// Repo-relative path of the component-gallery demo-fixture directory.
pub const M5_COMPONENT_GALLERY_DIR: &str = "fixtures/ui/m5-component-gallery/";

/// Prefix every governed message id carries so consumers can route them by lane.
pub const M5_CONTRACT_MESSAGE_ID_PREFIX: &str = "design_system_contract.";

/// One governed kind of design-system contract object the matrix publishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContractObjectKind {
    /// Canonical foundations: token packages, themes, density, and motion postures.
    Foundation,
    /// A launch-critical component contract: anatomy, states, keyboard/accessibility behavior,
    /// token dependencies, and extension guidance.
    ComponentContract,
    /// A reference layout: shell slots and placeholder behavior.
    ReferenceLayout,
    /// A state-semantic family: the canonical state classes and their cue requirements.
    StateSemanticFamily,
    /// A demo fixture: the checked-in component-gallery example a surface renders from.
    DemoFixture,
    /// A proof packet: the visual/a11y/token evidence generated from the contract.
    ProofPacket,
}

impl M5ContractObjectKind {
    /// Every governed object kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Foundation,
        Self::ComponentContract,
        Self::ReferenceLayout,
        Self::StateSemanticFamily,
        Self::DemoFixture,
        Self::ProofPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Foundation => "foundation",
            Self::ComponentContract => "component_contract",
            Self::ReferenceLayout => "reference_layout",
            Self::StateSemanticFamily => "state_semantic_family",
            Self::DemoFixture => "demo_fixture",
            Self::ProofPacket => "proof_packet",
        }
    }

    /// Repo-relative canonical schema that governs this object kind's artifact shape.
    pub const fn canonical_schema_ref(self) -> &'static str {
        match self {
            Self::Foundation => M5_FOUNDATIONS_SCHEMA_REF,
            Self::ComponentContract => M5_COMPONENT_CONTRACT_SCHEMA_REF,
            Self::ReferenceLayout => M5_REFERENCE_LAYOUT_SCHEMA_REF,
            // State families, demo fixtures, and proof packets are governed by the matrix
            // packet schema itself rather than a dedicated artifact schema.
            Self::StateSemanticFamily | Self::DemoFixture | Self::ProofPacket => {
                M5_DESIGN_SYSTEM_CONTRACT_MATRIX_SCHEMA_REF
            }
        }
    }
}

/// A surface or system that consumes the design-system contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DesignSystemConsumer {
    /// The shell renders from the contract.
    Shell,
    /// Help and About document the contract.
    Help,
    /// Onboarding reflects the contract.
    Onboarding,
    /// Presentation/classroom mode reflects the contract.
    Presentation,
    /// The extension SDK consumes the contract as extension-UI guidance.
    ExtensionSdk,
    /// Release center consumes the contract.
    ReleaseCenter,
    /// QA gates on the contract.
    Qa,
    /// Support export consumes the contract.
    SupportExport,
    /// The stable-claim matrix reads the contract.
    StableClaimMatrix,
}

impl M5DesignSystemConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Shell,
        Self::Help,
        Self::Onboarding,
        Self::Presentation,
        Self::ExtensionSdk,
        Self::ReleaseCenter,
        Self::Qa,
        Self::SupportExport,
        Self::StableClaimMatrix,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Help => "help",
            Self::Onboarding => "onboarding",
            Self::Presentation => "presentation",
            Self::ExtensionSdk => "extension_sdk",
            Self::ReleaseCenter => "release_center",
            Self::Qa => "qa",
            Self::SupportExport => "support_export",
            Self::StableClaimMatrix => "stable_claim_matrix",
        }
    }
}

/// Freshness of a governed object's design-system proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContractProofFreshness {
    /// The proof is within its freshness SLO.
    Current,
    /// The proof exists but has fallen outside its freshness SLO.
    Stale,
    /// No current proof row exists for this object.
    Missing,
}

impl M5ContractProofFreshness {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Current, Self::Stale, Self::Missing];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }

    /// True when the proof is within its freshness SLO.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// Green/yellow/red coverage status for a claimed surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageStatus {
    /// Every required contract object is mapped and its proof is current.
    Conformant,
    /// A required contract object's design-system proof is stale; the surface must be retested.
    RetestPending,
    /// A required contract object is unmapped or its proof is missing; the surface is uncovered.
    Uncovered,
}

impl M5CoverageStatus {
    /// Every status, in declaration order.
    pub const ALL: [Self; 3] = [Self::Conformant, Self::RetestPending, Self::Uncovered];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Conformant => "conformant",
            Self::RetestPending => "retest_pending",
            Self::Uncovered => "uncovered",
        }
    }

    /// The traffic-light signal this status maps to.
    pub const fn signal(self) -> M5CoverageSignal {
        match self {
            Self::Conformant => M5CoverageSignal::Green,
            Self::RetestPending => M5CoverageSignal::Yellow,
            Self::Uncovered => M5CoverageSignal::Red,
        }
    }
}

/// Traffic-light signal for the published dashboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageSignal {
    /// Conformant.
    Green,
    /// Retest-pending.
    Yellow,
    /// Uncovered.
    Red,
}

impl M5CoverageSignal {
    /// Every signal, in declaration order.
    pub const ALL: [Self; 3] = [Self::Green, Self::Yellow, Self::Red];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }
}

/// Release-gate decision the release/public-truth automation reads for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageGateDecision {
    /// The surface may promote to Stable at its full claim.
    CertifiedPromote,
    /// The surface auto-narrows to a disclosed reduced claim before promotion.
    AutoNarrowed,
    /// The surface is blocked from Stable promotion by an unmapped object or missing proof.
    Blocked,
}

impl M5CoverageGateDecision {
    /// Every decision, in declaration order.
    pub const ALL: [Self; 3] = [Self::CertifiedPromote, Self::AutoNarrowed, Self::Blocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedPromote => "certified_promote",
            Self::AutoNarrowed => "auto_narrowed",
            Self::Blocked => "blocked",
        }
    }

    /// True when the decision blocks Stable promotion.
    pub const fn blocks(self) -> bool {
        matches!(self, Self::Blocked)
    }
}

/// Public claim class a claimed surface holds for its design-system contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DesignSystemClaimClass {
    /// Full Stable claim.
    Stable,
    /// Narrowed Beta claim.
    Beta,
    /// Narrowed Preview claim.
    Preview,
    /// Narrowed Experimental claim.
    Experimental,
    /// The claim is held (blocked) until the gap is resolved.
    Held,
    /// The contract is unavailable on the surface.
    Unavailable,
}

impl M5DesignSystemClaimClass {
    /// Every claim class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Experimental,
        Self::Held,
        Self::Unavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Held => "held",
            Self::Unavailable => "unavailable",
        }
    }

    /// True when the surface keeps a full Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Restrictiveness rank (Stable least, Unavailable most).
    const fn rank(self) -> u8 {
        match self {
            Self::Stable => 0,
            Self::Beta => 1,
            Self::Preview => 2,
            Self::Experimental => 3,
            Self::Held => 4,
            Self::Unavailable => 5,
        }
    }
}

/// The more restrictive of two claim classes.
fn more_restrictive(
    a: M5DesignSystemClaimClass,
    b: M5DesignSystemClaimClass,
) -> M5DesignSystemClaimClass {
    if a.rank() >= b.rank() {
        a
    } else {
        b
    }
}

/// One kind of contract-coverage gap a surface can carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContractGapKind {
    /// The required contract object is not published in the inventory.
    UnmappedObject,
    /// The required contract object exists but carries no current proof.
    MissingProof,
    /// The required contract object exists but its proof has gone stale.
    StaleProof,
}

impl M5ContractGapKind {
    /// Every gap kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::UnmappedObject, Self::MissingProof, Self::StaleProof];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnmappedObject => "unmapped_object",
            Self::MissingProof => "missing_proof",
            Self::StaleProof => "stale_proof",
        }
    }

    /// True when the gap blocks Stable promotion without a waiver.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::UnmappedObject | Self::MissingProof)
    }
}

/// One governed design-system contract object in the published inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesignSystemContractObject {
    /// Stable object id, unique within the matrix.
    pub object_id: String,
    /// The governed object kind.
    pub object_kind: M5ContractObjectKind,
    /// Human-readable object name.
    pub display_name: String,
    /// Owner role accountable for keeping the object current.
    pub owner_role: String,
    /// The first consumer that reads this object.
    pub first_consumer: M5DesignSystemConsumer,
    /// Repo-relative canonical artifact that materializes this object.
    pub canonical_artifact_ref: String,
    /// Repo-relative schema that governs the canonical artifact's shape.
    pub schema_ref: String,
    /// Repo-relative release packet that keeps this object current.
    pub release_packet_ref: String,
    /// Repo-relative proof lane that blocks drift for this object.
    pub proof_lane_ref: String,
    /// Repo-relative extension-SDK guidance ref that extenders read for this object.
    pub extension_guidance_ref: String,
    /// Freshness of this object's design-system proof.
    pub proof_freshness: M5ContractProofFreshness,
    /// Stable message id; prefixed [`M5_CONTRACT_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
}

impl M5DesignSystemContractObject {
    /// True when this object's proof is within its freshness SLO.
    pub fn is_current(&self) -> bool {
        self.proof_freshness.is_current()
    }
}

/// One required contract object a claimed surface must map.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RequiredContractObject {
    /// The object id the surface points at.
    pub object_id: String,
    /// The governed object kind the surface needs.
    pub object_kind: M5ContractObjectKind,
}

/// One contract-coverage gap on a claimed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContractGap {
    /// Surface this gap applies to.
    pub surface_id: String,
    /// The required object id the gap concerns.
    pub object_id: String,
    /// The governed object kind the surface needed.
    pub object_kind: M5ContractObjectKind,
    /// The kind of gap (unmapped object, missing proof, or stale proof).
    pub gap_kind: M5ContractGapKind,
    /// Whether this gap was accepted under an active waiver.
    pub waived: bool,
    /// Stable message id; prefixed [`M5_CONTRACT_MESSAGE_ID_PREFIX`].
    pub cause_message_id: String,
}

/// One active waiver accepting a disclosed reduced claim for a single contract object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoverageWaiver {
    /// Stable waiver id.
    pub waiver_id: String,
    /// The object id the waiver scopes; the waiver only covers this object's gap.
    pub object_id: String,
    /// Stable message id naming the waiver reason; prefixed [`M5_CONTRACT_MESSAGE_ID_PREFIX`].
    pub reason_message_id: String,
    /// Owner role accountable for the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry timestamp of the waiver.
    pub expires_at: String,
    /// The disclosed reduced claim accepted under this waiver.
    pub narrowed_to: M5DesignSystemClaimClass,
}

/// Derived coverage fields for a surface, computed from its required objects, the inventory,
/// and its waivers.
struct DerivedCoverage {
    status: M5CoverageStatus,
    signal: M5CoverageSignal,
    effective_class: M5DesignSystemClaimClass,
    gate_decision: M5CoverageGateDecision,
    gaps: Vec<M5ContractGap>,
}

/// Computes the derived coverage fields for a surface from its claim, required objects, the
/// inventory, and its waivers. This is the single source of truth the seed reconciles to and
/// the validator recomputes against, so the stored derived fields can never silently drift.
fn derive_coverage(
    surface_id: &str,
    claimed: M5DesignSystemClaimClass,
    required: &[M5RequiredContractObject],
    inventory: &[M5DesignSystemContractObject],
    waivers: &[M5CoverageWaiver],
) -> DerivedCoverage {
    let waived = |object_id: &str| waivers.iter().any(|w| w.object_id == object_id);

    let mut gaps: Vec<M5ContractGap> = required
        .iter()
        .filter_map(|req| {
            let gap_kind = match inventory.iter().find(|o| o.object_id == req.object_id) {
                None => Some(M5ContractGapKind::UnmappedObject),
                Some(object) => match object.proof_freshness {
                    M5ContractProofFreshness::Current => None,
                    M5ContractProofFreshness::Missing => Some(M5ContractGapKind::MissingProof),
                    M5ContractProofFreshness::Stale => Some(M5ContractGapKind::StaleProof),
                },
            }?;
            Some(M5ContractGap {
                surface_id: surface_id.to_owned(),
                object_id: req.object_id.clone(),
                object_kind: req.object_kind,
                gap_kind,
                waived: waived(&req.object_id),
                cause_message_id: format!(
                    "{}{}.{}.gap",
                    M5_CONTRACT_MESSAGE_ID_PREFIX, surface_id, req.object_id
                ),
            })
        })
        .collect();
    gaps.sort_by(|a, b| a.object_id.cmp(&b.object_id));

    let any_blocking = gaps.iter().any(|g| g.gap_kind.is_blocking());
    let any_narrowing = gaps.iter().any(|g| !g.gap_kind.is_blocking());

    // The status reflects true coverage, independent of waivers, so the dashboard never hides
    // a real gap behind a waiver.
    let status = if any_blocking {
        M5CoverageStatus::Uncovered
    } else if any_narrowing {
        M5CoverageStatus::RetestPending
    } else {
        M5CoverageStatus::Conformant
    };

    let unwaived_blocking = gaps.iter().any(|g| g.gap_kind.is_blocking() && !g.waived);
    let any_gap = !gaps.is_empty();

    let gate_decision = if unwaived_blocking {
        M5CoverageGateDecision::Blocked
    } else if any_gap {
        M5CoverageGateDecision::AutoNarrowed
    } else {
        M5CoverageGateDecision::CertifiedPromote
    };

    let effective_class = match gate_decision {
        M5CoverageGateDecision::CertifiedPromote => claimed,
        M5CoverageGateDecision::Blocked => M5DesignSystemClaimClass::Held,
        M5CoverageGateDecision::AutoNarrowed => {
            // Floor at Beta for any unwaived gap, then apply the most restrictive accepted
            // claim across active waivers.
            let mut effective = M5DesignSystemClaimClass::Stable;
            if gaps.iter().any(|g| !g.waived) {
                effective = more_restrictive(effective, M5DesignSystemClaimClass::Beta);
            }
            for waiver in waivers {
                effective = more_restrictive(effective, waiver.narrowed_to);
            }
            effective
        }
    };

    DerivedCoverage {
        status,
        signal: status.signal(),
        effective_class,
        gate_decision,
        gaps,
    }
}

/// One claimed M5 surface's contract-coverage row: the contract objects it must map, its
/// traffic-light status, release-gate decision, active waivers, and exact contract gaps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceContractCoverage {
    /// Stable surface id, unique within the matrix.
    pub surface_id: String,
    /// Launch-critical surface class (beta-contract owned).
    pub surface_class: LaunchSurfaceClass,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Owner role accountable for keeping this surface's coverage current.
    pub owner_role: String,
    /// Public claim the surface wants to keep (always Stable for claimed M5 surfaces).
    pub claimed_class: M5DesignSystemClaimClass,
    /// Effective claim after the coverage gate applies.
    pub effective_class: M5DesignSystemClaimClass,
    /// Green/yellow/red coverage status.
    pub coverage_status: M5CoverageStatus,
    /// Traffic-light signal (mirrors [`Self::coverage_status`]).
    pub signal: M5CoverageSignal,
    /// The contract objects this surface must map.
    pub required_objects: Vec<M5RequiredContractObject>,
    /// Active waivers accepting a disclosed reduced claim for one object each.
    pub waivers: Vec<M5CoverageWaiver>,
    /// Release-gate decision the release/public-truth automation reads.
    pub gate_decision: M5CoverageGateDecision,
    /// Exact contract gaps for this surface.
    pub gaps: Vec<M5ContractGap>,
    /// Consumer surfaces that project this row's coverage.
    pub consumer_surfaces: Vec<M5DesignSystemConsumer>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Stable message id for the status; prefixed [`M5_CONTRACT_MESSAGE_ID_PREFIX`].
    pub status_message_id: String,
    /// Stable message id for the gate; prefixed [`M5_CONTRACT_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

impl M5SurfaceContractCoverage {
    /// Recomputes the derived status, signal, effective claim, gate, and gaps from the
    /// required objects, the inventory, and the waivers. The seed calls this after authoring
    /// or mutating a row so the derived blocks never need hand-maintenance.
    pub fn recompute_derived(&mut self, inventory: &[M5DesignSystemContractObject]) {
        let derived = derive_coverage(
            &self.surface_id,
            self.claimed_class,
            &self.required_objects,
            inventory,
            &self.waivers,
        );
        self.coverage_status = derived.status;
        self.signal = derived.signal;
        self.effective_class = derived.effective_class;
        self.gate_decision = derived.gate_decision;
        self.gaps = derived.gaps;
    }

    /// True when the surface is blocked from Stable promotion.
    pub fn is_blocked(&self) -> bool {
        self.gate_decision.blocks()
    }

    /// True when the surface auto-narrowed below its claim.
    pub fn is_auto_narrowed(&self) -> bool {
        matches!(self.gate_decision, M5CoverageGateDecision::AutoNarrowed)
    }

    /// True when the surface is fully conformant for Stable promotion.
    pub fn is_conformant(&self) -> bool {
        matches!(self.gate_decision, M5CoverageGateDecision::CertifiedPromote)
    }
}

/// Compact green/yellow/red contract dashboard — the published scoreboard projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesignSystemDashboard {
    /// Record kind; must equal [`M5_DESIGN_SYSTEM_DASHBOARD_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DESIGN_SYSTEM_DASHBOARD_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Cross-ref to the matrix packet this dashboard projects.
    pub matrix_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// Total governed contract objects in the inventory.
    pub total_objects: u32,
    /// Governed-object counts per kind (sorted by kind token).
    pub objects_by_kind: Vec<M5ObjectKindCount>,
    /// Object ids whose proof is stale or missing (sorted).
    pub stale_object_ids: Vec<String>,
    /// Total claimed surfaces.
    pub total_surfaces: u32,
    /// Green (conformant) count.
    pub green_count: u32,
    /// Yellow (retest-pending) count.
    pub yellow_count: u32,
    /// Red (uncovered) count.
    pub red_count: u32,
    /// Conformant surface ids (sorted).
    pub conformant_surface_ids: Vec<String>,
    /// Retest-pending surface ids (sorted).
    pub retest_pending_surface_ids: Vec<String>,
    /// Uncovered surface ids (sorted).
    pub uncovered_surface_ids: Vec<String>,
    /// Surface ids that auto-narrowed below their claim (sorted).
    pub auto_narrowed_surface_ids: Vec<String>,
    /// Surface ids blocked from Stable promotion (sorted).
    pub blocked_surface_ids: Vec<String>,
    /// Surface ids that carry at least one active waiver (sorted).
    pub waived_surface_ids: Vec<String>,
    /// Active waiver ids (sorted).
    pub active_waiver_ids: Vec<String>,
    /// True when at least one surface is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
    /// Exact contract gaps across all surfaces.
    pub contract_gaps: Vec<M5ContractGap>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Stable message id; prefixed [`M5_CONTRACT_MESSAGE_ID_PREFIX`].
    pub dashboard_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

/// One governed-object kind's count in the inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ObjectKindCount {
    /// The governed object kind.
    pub object_kind: M5ContractObjectKind,
    /// Number of inventory objects of this kind.
    pub count: u32,
}

/// Self-describing controlled-vocabulary set for the contract-shaped tokens this lane mints,
/// plus the beta-contract and foundations tokens these rows reuse, so the packet resolves
/// every token it carries on its own.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesignSystemContractVocabularySet {
    /// Contract-object-kind tokens.
    pub object_kinds: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
    /// Proof-freshness-state tokens.
    pub proof_freshness_states: Vec<String>,
    /// Coverage-status tokens.
    pub coverage_statuses: Vec<String>,
    /// Coverage-signal tokens.
    pub coverage_signals: Vec<String>,
    /// Coverage-gate-decision tokens.
    pub coverage_gate_decisions: Vec<String>,
    /// Claim-class tokens.
    pub claim_classes: Vec<String>,
    /// Gap-kind tokens.
    pub gap_kinds: Vec<String>,
    /// State-semantic-family tokens (beta-contract owned).
    pub state_semantic_families: Vec<String>,
    /// Launch-surface-class tokens (beta-contract owned).
    pub launch_surface_classes: Vec<String>,
    /// Cue-family tokens (beta-contract owned).
    pub cue_families: Vec<String>,
}

impl M5DesignSystemContractVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL`/`required` arrays.
    pub fn canonical() -> Self {
        Self {
            object_kinds: M5ContractObjectKind::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            consumers: M5DesignSystemConsumer::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            proof_freshness_states: M5ContractProofFreshness::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            coverage_statuses: M5CoverageStatus::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            coverage_signals: M5CoverageSignal::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            coverage_gate_decisions: M5CoverageGateDecision::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            claim_classes: M5DesignSystemClaimClass::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            gap_kinds: M5ContractGapKind::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            state_semantic_families: CanonicalStateClass::required()
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            launch_surface_classes: LaunchSurfaceClass::required()
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            cue_families: CueFamilyClass::required()
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Design-system contract conformance review. Every flag is a hard invariant; all must hold
/// for the packet to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesignSystemConformanceReview {
    /// Every governed object names its owner and first consumer.
    pub every_object_named_with_owner_and_first_consumer: bool,
    /// The foundations/tokens object is published.
    pub foundations_object_published: bool,
    /// The component-contract objects are published.
    pub component_contracts_object_published: bool,
    /// The reference-layout objects are published.
    pub reference_layouts_object_published: bool,
    /// The state-semantic-family object is published.
    pub state_semantic_families_object_published: bool,
    /// The demo-fixture object is published.
    pub demo_fixtures_object_published: bool,
    /// The proof-packet object is published.
    pub proof_packets_object_published: bool,
    /// Every object binds a canonical artifact and proof lane.
    pub every_object_binds_canonical_artifact_and_proof_lane: bool,
    /// Every object binds a release packet that keeps it current.
    pub every_object_binds_release_packet: bool,
    /// Launch-critical components declare anatomy, states, keyboard/a11y, token deps, and
    /// extension guidance.
    pub component_contracts_declare_anatomy_states_keyboard_a11y_tokens_extension: bool,
    /// Every claimed surface maps to required contract objects.
    pub every_claimed_surface_maps_required_objects: bool,
    /// An unmapped contract object blocks Stable promotion.
    pub unmapped_object_blocks_stable_promotion: bool,
    /// Stale or missing proof auto-narrows the surface before Stable promotion.
    pub stale_or_missing_proof_auto_narrows_before_stable: bool,
    /// Active waivers are disclosed with scope, owner, and expiry.
    pub waivers_disclosed_with_scope_owner_and_expiry: bool,
    /// Exact contract gaps are named per object.
    pub exact_contract_gaps_named: bool,
    /// The dashboard traffic-light counts match the rows.
    pub dashboard_traffic_light_matches_rows: bool,
    /// The machine-readable contract is shared, not restated in local docs.
    pub machine_readable_contract_shared_not_restated: bool,
    /// Support/export carries no raw boundary material.
    pub support_export_carries_no_raw_boundary_material: bool,
}

impl M5DesignSystemConformanceReview {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.every_object_named_with_owner_and_first_consumer
            && self.foundations_object_published
            && self.component_contracts_object_published
            && self.reference_layouts_object_published
            && self.state_semantic_families_object_published
            && self.demo_fixtures_object_published
            && self.proof_packets_object_published
            && self.every_object_binds_canonical_artifact_and_proof_lane
            && self.every_object_binds_release_packet
            && self.component_contracts_declare_anatomy_states_keyboard_a11y_tokens_extension
            && self.every_claimed_surface_maps_required_objects
            && self.unmapped_object_blocks_stable_promotion
            && self.stale_or_missing_proof_auto_narrows_before_stable
            && self.waivers_disclosed_with_scope_owner_and_expiry
            && self.exact_contract_gaps_named
            && self.dashboard_traffic_light_matches_rows
            && self.machine_readable_contract_shared_not_restated
            && self.support_export_carries_no_raw_boundary_material
    }
}

/// Consumer projection block: who reads the contract matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesignSystemConsumerProjection {
    /// The shell consumes the contract.
    pub shell_consumes_contract: bool,
    /// Help documents the contract.
    pub help_documents_contract: bool,
    /// Onboarding reflects the contract.
    pub onboarding_reflects_contract: bool,
    /// Presentation reflects the contract.
    pub presentation_reflects_contract: bool,
    /// The extension SDK consumes the contract.
    pub extension_sdk_consumes_contract: bool,
    /// Release center consumes the contract.
    pub release_center_consumes_contract: bool,
    /// QA gates on the contract.
    pub qa_gates_on_contract: bool,
    /// Support export consumes the contract.
    pub support_export_consumes_contract: bool,
    /// The stable-claim matrix reads the contract.
    pub stable_claim_matrix_reads_contract: bool,
}

impl M5DesignSystemConsumerProjection {
    /// True when every projection holds.
    pub fn all_hold(&self) -> bool {
        self.shell_consumes_contract
            && self.help_documents_contract
            && self.onboarding_reflects_contract
            && self.presentation_reflects_contract
            && self.extension_sdk_consumes_contract
            && self.release_center_consumes_contract
            && self.qa_gates_on_contract
            && self.support_export_consumes_contract
            && self.stable_claim_matrix_reads_contract
    }
}

/// Packet-level release gate aggregating the per-surface coverage gates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesignSystemReleaseGate {
    /// True when at least one surface is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
    /// Sorted surface ids blocked from Stable promotion.
    pub blocked_surface_ids: Vec<String>,
    /// Sorted surface ids that auto-narrowed below their claim.
    pub auto_narrowed_surface_ids: Vec<String>,
    /// Sorted surface ids fully conformant for Stable promotion.
    pub conformant_surface_ids: Vec<String>,
    /// Sorted surface ids carrying at least one active waiver.
    pub waived_surface_ids: Vec<String>,
    /// Stable message id; prefixed [`M5_CONTRACT_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

/// Proof-freshness policy for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContractProofFreshnessPolicy {
    /// Freshness SLO in hours; a proof older than this auto-narrows the surfaces that map it.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// Whether stale proof auto-narrows the mapping surfaces.
    pub auto_narrow_on_stale: bool,
}

/// Release and mirror/offline parity posture for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesignSystemReleasePosture {
    /// Repo-relative release packet ref.
    pub release_packet_ref: String,
    /// Repo-relative mirror/offline packet ref.
    pub mirror_offline_packet_ref: String,
    /// Whether support-export parity is required.
    pub support_export_parity_required: bool,
    /// Whether mirror/offline parity is required.
    pub mirror_offline_parity_required: bool,
    /// Whether Stable promotion blocks without a mapped contract object.
    pub stable_promotion_blocks_without_mapped_object: bool,
}

/// Constructor input for [`M5DesignSystemContractMatrix::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DesignSystemContractMatrixInput {
    /// Stable matrix id.
    pub matrix_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The governed contract-object inventory.
    pub contract_objects: Vec<M5DesignSystemContractObject>,
    /// Per-surface coverage rows.
    pub surfaces: Vec<M5SurfaceContractCoverage>,
    /// Controlled-vocabulary set.
    pub vocabulary_set: M5DesignSystemContractVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5DesignSystemConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DesignSystemConsumerProjection,
    /// Packet-level release gate.
    pub release_gate: M5DesignSystemReleaseGate,
    /// Proof freshness policy.
    pub proof_freshness: M5ContractProofFreshnessPolicy,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5DesignSystemReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 design-system contract matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesignSystemContractMatrix {
    /// Record kind; must equal [`M5_DESIGN_SYSTEM_CONTRACT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DESIGN_SYSTEM_CONTRACT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable matrix id.
    pub matrix_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The governed contract-object inventory.
    pub contract_objects: Vec<M5DesignSystemContractObject>,
    /// Per-surface coverage rows.
    pub surfaces: Vec<M5SurfaceContractCoverage>,
    /// Controlled-vocabulary set.
    pub vocabulary_set: M5DesignSystemContractVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5DesignSystemConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DesignSystemConsumerProjection,
    /// Packet-level release gate.
    pub release_gate: M5DesignSystemReleaseGate,
    /// Proof freshness policy.
    pub proof_freshness: M5ContractProofFreshnessPolicy,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5DesignSystemReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DesignSystemContractMatrix {
    /// Builds a contract matrix from seed input.
    pub fn new(input: M5DesignSystemContractMatrixInput) -> Self {
        Self {
            record_kind: M5_DESIGN_SYSTEM_CONTRACT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_DESIGN_SYSTEM_CONTRACT_MATRIX_SCHEMA_VERSION,
            matrix_id: input.matrix_id,
            report_label: input.report_label,
            contract_objects: input.contract_objects,
            surfaces: input.surfaces,
            vocabulary_set: input.vocabulary_set,
            conformance_review: input.conformance_review,
            consumer_projection: input.consumer_projection,
            release_gate: input.release_gate,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release/public-truth automation must hold Stable promotion because at
    /// least one claimed surface is blocked.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.release_gate.blocks_stable_promotion
    }

    /// Surface ids currently blocked from Stable promotion.
    pub fn blocked_surface_ids(&self) -> Vec<&str> {
        self.surfaces
            .iter()
            .filter(|s| s.is_blocked())
            .map(|s| s.surface_id.as_str())
            .collect()
    }

    /// Finds a governed contract object by id.
    pub fn object(&self, object_id: &str) -> Option<&M5DesignSystemContractObject> {
        self.contract_objects
            .iter()
            .find(|o| o.object_id == object_id)
    }

    /// Builds the compact green/yellow/red dashboard projection from the rows and inventory.
    pub fn dashboard(&self) -> M5DesignSystemDashboard {
        let by_status = |status: M5CoverageStatus| -> Vec<String> {
            let mut ids: Vec<String> = self
                .surfaces
                .iter()
                .filter(|s| s.coverage_status == status)
                .map(|s| s.surface_id.clone())
                .collect();
            ids.sort();
            ids
        };
        let by_predicate = |predicate: &dyn Fn(&M5SurfaceContractCoverage) -> bool| -> Vec<String> {
            let mut ids: Vec<String> = self
                .surfaces
                .iter()
                .filter(|s| predicate(s))
                .map(|s| s.surface_id.clone())
                .collect();
            ids.sort();
            ids
        };

        let mut active_waiver_ids: Vec<String> = self
            .surfaces
            .iter()
            .flat_map(|s| s.waivers.iter().map(|w| w.waiver_id.clone()))
            .collect();
        active_waiver_ids.sort();

        let mut contract_gaps: Vec<M5ContractGap> = self
            .surfaces
            .iter()
            .flat_map(|s| s.gaps.iter().cloned())
            .collect();
        contract_gaps.sort_by(|a, b| {
            a.surface_id
                .cmp(&b.surface_id)
                .then(a.object_id.cmp(&b.object_id))
        });

        let mut objects_by_kind: Vec<M5ObjectKindCount> = M5ContractObjectKind::ALL
            .iter()
            .map(|&kind| M5ObjectKindCount {
                object_kind: kind,
                count: self
                    .contract_objects
                    .iter()
                    .filter(|o| o.object_kind == kind)
                    .count() as u32,
            })
            .collect();
        objects_by_kind.sort_by(|a, b| a.object_kind.as_str().cmp(b.object_kind.as_str()));

        let mut stale_object_ids: Vec<String> = self
            .contract_objects
            .iter()
            .filter(|o| !o.is_current())
            .map(|o| o.object_id.clone())
            .collect();
        stale_object_ids.sort();

        let count_signal = |signal: M5CoverageSignal| -> u32 {
            self.surfaces.iter().filter(|s| s.signal == signal).count() as u32
        };

        let blocked_surface_ids = by_predicate(&|s| s.is_blocked());

        M5DesignSystemDashboard {
            record_kind: M5_DESIGN_SYSTEM_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_DESIGN_SYSTEM_DASHBOARD_SCHEMA_VERSION,
            matrix_id: self.matrix_id.clone(),
            report_label: self.report_label.clone(),
            total_objects: self.contract_objects.len() as u32,
            objects_by_kind,
            stale_object_ids,
            total_surfaces: self.surfaces.len() as u32,
            green_count: count_signal(M5CoverageSignal::Green),
            yellow_count: count_signal(M5CoverageSignal::Yellow),
            red_count: count_signal(M5CoverageSignal::Red),
            conformant_surface_ids: by_status(M5CoverageStatus::Conformant),
            retest_pending_surface_ids: by_status(M5CoverageStatus::RetestPending),
            uncovered_surface_ids: by_status(M5CoverageStatus::Uncovered),
            auto_narrowed_surface_ids: by_predicate(&|s| s.is_auto_narrowed()),
            blocked_surface_ids: blocked_surface_ids.clone(),
            waived_surface_ids: by_predicate(&|s| !s.waivers.is_empty()),
            active_waiver_ids,
            blocks_stable_promotion: !blocked_surface_ids.is_empty(),
            contract_gaps,
            source_contract_refs: self.source_contract_refs.clone(),
            dashboard_message_id: format!("{}dashboard", M5_CONTRACT_MESSAGE_ID_PREFIX),
            minted_at: self.minted_at.clone(),
        }
    }

    /// Validates the contract-matrix invariants.
    pub fn validate(&self) -> Vec<M5ContractMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DESIGN_SYSTEM_CONTRACT_MATRIX_RECORD_KIND {
            violations.push(M5ContractMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DESIGN_SYSTEM_CONTRACT_MATRIX_SCHEMA_VERSION {
            violations.push(M5ContractMatrixViolation::WrongSchemaVersion);
        }
        if self.matrix_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ContractMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_inventory(self, &mut violations);
        validate_surfaces(self, &mut violations);
        validate_release_gate_aggregate(self, &mut violations);
        validate_dashboard(self, &mut violations);
        validate_conformance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 contract matrix serializes"),
        ) {
            violations.push(M5ContractMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 contract matrix serializes")
    }

    /// Deterministic export-safe JSON for the dashboard projection.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn dashboard_json(&self) -> String {
        serde_json::to_string_pretty(&self.dashboard()).expect("m5 contract dashboard serializes")
    }

    /// Deterministic Markdown proof for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let dashboard = self.dashboard();
        let mut out = String::new();
        out.push_str("# M5 Design-System Contract Matrix\n\n");
        out.push_str(&format!("- Matrix: `{}`\n", self.matrix_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!(
            "- Governed objects: {} ({} foundation, {} component, {} layout, {} state, {} fixture, {} proof)\n",
            dashboard.total_objects,
            self.objects_of_kind(M5ContractObjectKind::Foundation),
            self.objects_of_kind(M5ContractObjectKind::ComponentContract),
            self.objects_of_kind(M5ContractObjectKind::ReferenceLayout),
            self.objects_of_kind(M5ContractObjectKind::StateSemanticFamily),
            self.objects_of_kind(M5ContractObjectKind::DemoFixture),
            self.objects_of_kind(M5ContractObjectKind::ProofPacket),
        ));
        out.push_str(&format!(
            "- Surfaces: {} ({} green, {} yellow, {} red)\n",
            dashboard.total_surfaces,
            dashboard.green_count,
            dashboard.yellow_count,
            dashboard.red_count
        ));
        out.push_str(&format!(
            "- Release gate: {} ({} blocked, {} auto-narrowed, {} conformant)\n",
            if self.release_gate.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            },
            self.release_gate.blocked_surface_ids.len(),
            self.release_gate.auto_narrowed_surface_ids.len(),
            self.release_gate.conformant_surface_ids.len()
        ));
        out.push_str(&format!(
            "- Active waivers: {}\n",
            dashboard.active_waiver_ids.len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Governed contract objects\n\n");
        for object in &self.contract_objects {
            out.push_str(&format!(
                "- **{}** (`{}`, {}): owner {}, first consumer `{}`, proof `{}`\n",
                object.object_id,
                object.display_name,
                object.object_kind.as_str(),
                object.owner_role,
                object.first_consumer.as_str(),
                object.proof_freshness.as_str(),
            ));
            out.push_str(&format!(
                "  - Canonical artifact: `{}`\n",
                object.canonical_artifact_ref
            ));
            out.push_str(&format!("  - Proof lane: `{}`\n", object.proof_lane_ref));
            out.push_str(&format!(
                "  - Release packet: `{}`\n",
                object.release_packet_ref
            ));
        }

        out.push_str("\n## Claimed surfaces\n\n");
        for surface in &self.surfaces {
            out.push_str(&format!(
                "- **{}** (`{}`): `{}` ({}), claim `{}` → `{}`, gate `{}`\n",
                surface.surface_id,
                surface.surface_class.as_str(),
                surface.coverage_status.as_str(),
                surface.signal.as_str(),
                surface.claimed_class.as_str(),
                surface.effective_class.as_str(),
                surface.gate_decision.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", surface.owner_role));
            for required in &surface.required_objects {
                out.push_str(&format!(
                    "  - requires `{}` ({})\n",
                    required.object_id,
                    required.object_kind.as_str()
                ));
            }
            for gap in &surface.gaps {
                out.push_str(&format!(
                    "  - gap on `{}` ({}): `{}`{}\n",
                    gap.object_id,
                    gap.object_kind.as_str(),
                    gap.gap_kind.as_str(),
                    if gap.waived { " (waived)" } else { "" }
                ));
            }
            for waiver in &surface.waivers {
                out.push_str(&format!(
                    "  - waiver `{}` on `{}` → `{}` (owner {}, expires {})\n",
                    waiver.waiver_id,
                    waiver.object_id,
                    waiver.narrowed_to.as_str(),
                    waiver.owner_role,
                    waiver.expires_at
                ));
            }
        }
        out
    }

    /// Counts the inventory objects of a given kind.
    fn objects_of_kind(&self, kind: M5ContractObjectKind) -> usize {
        self.contract_objects
            .iter()
            .filter(|o| o.object_kind == kind)
            .count()
    }
}

/// Errors emitted when reading the checked-in matrix export.
#[derive(Debug)]
pub enum M5ContractMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ContractMatrixViolation>),
}

impl fmt::Display for M5ContractMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "m5 contract matrix parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                write!(
                    formatter,
                    "m5 contract matrix failed validation: {}",
                    tokens.join(",")
                )
            }
        }
    }
}

impl Error for M5ContractMatrixArtifactError {}

/// Validation failures emitted by [`M5DesignSystemContractMatrix::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ContractMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A governed object kind has no inventory object.
    RequiredObjectKindMissing,
    /// Two inventory objects share an id.
    DuplicateObjectId,
    /// A governed object is incomplete.
    ObjectIncomplete,
    /// A governed object's schema ref is wrong for its kind.
    ObjectSchemaMismatch,
    /// Two coverage rows share a surface id.
    DuplicateSurfaceId,
    /// A coverage row is incomplete.
    CoverageRowIncomplete,
    /// A coverage row does not claim Stable.
    RowDoesNotClaimStable,
    /// A coverage row maps no required objects.
    RowMapsNoRequiredObjects,
    /// A coverage row's derived status / signal / gate / effective claim disagrees with the
    /// inventory and its waivers.
    DerivedRowInconsistent,
    /// A coverage row's gaps disagree with the inventory.
    GapsInconsistent,
    /// A waiver is incomplete.
    WaiverIncomplete,
    /// A waiver scopes an object the row does not require.
    WaiverObjectUnknown,
    /// A coverage row has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A message id is missing the governed prefix.
    MessageIdPrefixMissing,
    /// The packet-level release gate disagrees with the per-surface gates.
    ReleaseGateAggregateInconsistent,
    /// The dashboard projection disagrees with the rows or inventory.
    DashboardInconsistent,
    /// Conformance review does not satisfy required invariants.
    ConformanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness policy is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ContractMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredObjectKindMissing => "required_object_kind_missing",
            Self::DuplicateObjectId => "duplicate_object_id",
            Self::ObjectIncomplete => "object_incomplete",
            Self::ObjectSchemaMismatch => "object_schema_mismatch",
            Self::DuplicateSurfaceId => "duplicate_surface_id",
            Self::CoverageRowIncomplete => "coverage_row_incomplete",
            Self::RowDoesNotClaimStable => "row_does_not_claim_stable",
            Self::RowMapsNoRequiredObjects => "row_maps_no_required_objects",
            Self::DerivedRowInconsistent => "derived_row_inconsistent",
            Self::GapsInconsistent => "gaps_inconsistent",
            Self::WaiverIncomplete => "waiver_incomplete",
            Self::WaiverObjectUnknown => "waiver_object_unknown",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::MessageIdPrefixMissing => "message_id_prefix_missing",
            Self::ReleaseGateAggregateInconsistent => "release_gate_aggregate_inconsistent",
            Self::DashboardInconsistent => "dashboard_inconsistent",
            Self::ConformanceReviewIncomplete => "conformance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in matrix support export.
pub fn current_stable_m5_design_system_contract_matrix(
) -> Result<M5DesignSystemContractMatrix, M5ContractMatrixArtifactError> {
    let matrix: M5DesignSystemContractMatrix = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-design-system-proof/support_export.json"
    )))
    .map_err(M5ContractMatrixArtifactError::SupportExport)?;
    let violations = matrix.validate();
    if violations.is_empty() {
        Ok(matrix)
    } else {
        Err(M5ContractMatrixArtifactError::Validation(violations))
    }
}

/// Reads the checked-in dashboard projection.
pub fn current_stable_m5_design_system_dashboard(
) -> Result<M5DesignSystemDashboard, M5ContractMatrixArtifactError> {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/design-system/m5-design-system-dashboard.json"
    )))
    .map_err(M5ContractMatrixArtifactError::SupportExport)
}

fn validate_source_contracts(
    matrix: &M5DesignSystemContractMatrix,
    violations: &mut Vec<M5ContractMatrixViolation>,
) {
    let refs: BTreeSet<&str> = matrix
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DESIGN_SYSTEM_CONTRACT_MATRIX_SCHEMA_REF,
        M5_DESIGN_SYSTEM_DASHBOARD_SCHEMA_REF,
        M5_FOUNDATIONS_SCHEMA_REF,
        M5_COMPONENT_CONTRACT_SCHEMA_REF,
        M5_REFERENCE_LAYOUT_SCHEMA_REF,
        M5_DESIGN_SYSTEM_CONTRACT_DOC_REF,
        M5_DESIGN_SYSTEM_GOVERNANCE_REF,
        M5_DESIGN_SYSTEM_PROOF_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ContractMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    matrix: &M5DesignSystemContractMatrix,
    violations: &mut Vec<M5ContractMatrixViolation>,
) {
    if !matrix.vocabulary_set.matches_canonical() {
        violations.push(M5ContractMatrixViolation::VocabularySetDrift);
    }
}

fn validate_inventory(
    matrix: &M5DesignSystemContractMatrix,
    violations: &mut Vec<M5ContractMatrixViolation>,
) {
    let present: BTreeSet<M5ContractObjectKind> = matrix
        .contract_objects
        .iter()
        .map(|o| o.object_kind)
        .collect();
    for required in M5ContractObjectKind::ALL {
        if !present.contains(&required) {
            violations.push(M5ContractMatrixViolation::RequiredObjectKindMissing);
            break;
        }
    }

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for object in &matrix.contract_objects {
        if !seen_ids.insert(object.object_id.as_str()) {
            violations.push(M5ContractMatrixViolation::DuplicateObjectId);
        }
        if object.object_id.trim().is_empty()
            || object.display_name.trim().is_empty()
            || object.owner_role.trim().is_empty()
            || object.canonical_artifact_ref.trim().is_empty()
            || object.release_packet_ref.trim().is_empty()
            || object.proof_lane_ref.trim().is_empty()
            || object.extension_guidance_ref.trim().is_empty()
        {
            violations.push(M5ContractMatrixViolation::ObjectIncomplete);
        }
        if object.schema_ref != object.object_kind.canonical_schema_ref() {
            violations.push(M5ContractMatrixViolation::ObjectSchemaMismatch);
        }
        if !object
            .summary_message_id
            .starts_with(M5_CONTRACT_MESSAGE_ID_PREFIX)
        {
            violations.push(M5ContractMatrixViolation::MessageIdPrefixMissing);
        }
    }
}

fn validate_surfaces(
    matrix: &M5DesignSystemContractMatrix,
    violations: &mut Vec<M5ContractMatrixViolation>,
) {
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for surface in &matrix.surfaces {
        if !seen_ids.insert(surface.surface_id.as_str()) {
            violations.push(M5ContractMatrixViolation::DuplicateSurfaceId);
        }
        if surface.surface_id.trim().is_empty()
            || surface.surface_label.trim().is_empty()
            || surface.owner_role.trim().is_empty()
            || surface.source_contract_refs.is_empty()
        {
            violations.push(M5ContractMatrixViolation::CoverageRowIncomplete);
        }
        if !surface.claimed_class.is_stable() {
            violations.push(M5ContractMatrixViolation::RowDoesNotClaimStable);
        }
        if surface.required_objects.is_empty() {
            violations.push(M5ContractMatrixViolation::RowMapsNoRequiredObjects);
        }
        if surface.consumer_surfaces.is_empty() {
            violations.push(M5ContractMatrixViolation::ConsumerSurfacesMissing);
        }
        if !surface
            .status_message_id
            .starts_with(M5_CONTRACT_MESSAGE_ID_PREFIX)
            || !surface
                .gate_message_id
                .starts_with(M5_CONTRACT_MESSAGE_ID_PREFIX)
        {
            violations.push(M5ContractMatrixViolation::MessageIdPrefixMissing);
        }

        validate_surface_waivers(surface, violations);
        validate_surface_derived(matrix, surface, violations);
    }
}

fn validate_surface_waivers(
    surface: &M5SurfaceContractCoverage,
    violations: &mut Vec<M5ContractMatrixViolation>,
) {
    for waiver in &surface.waivers {
        if waiver.waiver_id.trim().is_empty()
            || waiver.reason_message_id.trim().is_empty()
            || waiver.owner_role.trim().is_empty()
            || waiver.expires_at.trim().is_empty()
        {
            violations.push(M5ContractMatrixViolation::WaiverIncomplete);
        }
        if !waiver
            .reason_message_id
            .starts_with(M5_CONTRACT_MESSAGE_ID_PREFIX)
        {
            violations.push(M5ContractMatrixViolation::MessageIdPrefixMissing);
        }
        // A waiver must accept a genuinely reduced claim, never re-grant Stable.
        if waiver.narrowed_to.is_stable() {
            violations.push(M5ContractMatrixViolation::WaiverIncomplete);
        }
        if !surface
            .required_objects
            .iter()
            .any(|r| r.object_id == waiver.object_id)
        {
            violations.push(M5ContractMatrixViolation::WaiverObjectUnknown);
        }
    }
}

fn validate_surface_derived(
    matrix: &M5DesignSystemContractMatrix,
    surface: &M5SurfaceContractCoverage,
    violations: &mut Vec<M5ContractMatrixViolation>,
) {
    let derived = derive_coverage(
        &surface.surface_id,
        surface.claimed_class,
        &surface.required_objects,
        &matrix.contract_objects,
        &surface.waivers,
    );
    if surface.coverage_status != derived.status
        || surface.signal != derived.signal
        || surface.signal != surface.coverage_status.signal()
        || surface.effective_class != derived.effective_class
        || surface.gate_decision != derived.gate_decision
    {
        violations.push(M5ContractMatrixViolation::DerivedRowInconsistent);
    }
    if surface.gaps != derived.gaps {
        violations.push(M5ContractMatrixViolation::GapsInconsistent);
    }
}

fn validate_release_gate_aggregate(
    matrix: &M5DesignSystemContractMatrix,
    violations: &mut Vec<M5ContractMatrixViolation>,
) {
    let collect = |predicate: &dyn Fn(&M5SurfaceContractCoverage) -> bool| -> Vec<String> {
        let mut ids: Vec<String> = matrix
            .surfaces
            .iter()
            .filter(|s| predicate(s))
            .map(|s| s.surface_id.clone())
            .collect();
        ids.sort();
        ids
    };
    let blocked = collect(&|s| s.is_blocked());
    let auto_narrowed = collect(&|s| s.is_auto_narrowed());
    let conformant = collect(&|s| s.is_conformant());
    let waived = collect(&|s| !s.waivers.is_empty());
    let blocks_expected = !blocked.is_empty();

    let gate = &matrix.release_gate;
    if gate.blocks_stable_promotion != blocks_expected
        || gate.blocked_surface_ids != blocked
        || gate.auto_narrowed_surface_ids != auto_narrowed
        || gate.conformant_surface_ids != conformant
        || gate.waived_surface_ids != waived
        || !gate
            .gate_message_id
            .starts_with(M5_CONTRACT_MESSAGE_ID_PREFIX)
    {
        violations.push(M5ContractMatrixViolation::ReleaseGateAggregateInconsistent);
    }
}

fn validate_dashboard(
    matrix: &M5DesignSystemContractMatrix,
    violations: &mut Vec<M5ContractMatrixViolation>,
) {
    // The dashboard is a pure projection; recompute and check its internal accounting holds.
    let dashboard = matrix.dashboard();
    let signal_count = dashboard.green_count + dashboard.yellow_count + dashboard.red_count;
    let status_count = dashboard.conformant_surface_ids.len()
        + dashboard.retest_pending_surface_ids.len()
        + dashboard.uncovered_surface_ids.len();
    let object_kind_count: u32 = dashboard.objects_by_kind.iter().map(|c| c.count).sum();
    if dashboard.total_surfaces != matrix.surfaces.len() as u32
        || dashboard.total_objects != matrix.contract_objects.len() as u32
        || object_kind_count != dashboard.total_objects
        || signal_count != dashboard.total_surfaces
        || status_count as u32 != dashboard.total_surfaces
        || dashboard.green_count != dashboard.conformant_surface_ids.len() as u32
        || dashboard.yellow_count != dashboard.retest_pending_surface_ids.len() as u32
        || dashboard.red_count != dashboard.uncovered_surface_ids.len() as u32
        || dashboard.blocks_stable_promotion != matrix.release_gate.blocks_stable_promotion
        || dashboard.blocked_surface_ids != matrix.release_gate.blocked_surface_ids
        || !dashboard
            .dashboard_message_id
            .starts_with(M5_CONTRACT_MESSAGE_ID_PREFIX)
    {
        violations.push(M5ContractMatrixViolation::DashboardInconsistent);
    }
}

fn validate_conformance_review(
    matrix: &M5DesignSystemContractMatrix,
    violations: &mut Vec<M5ContractMatrixViolation>,
) {
    if !matrix.conformance_review.all_hold() {
        violations.push(M5ContractMatrixViolation::ConformanceReviewIncomplete);
    }
}

fn validate_consumer_projection(
    matrix: &M5DesignSystemContractMatrix,
    violations: &mut Vec<M5ContractMatrixViolation>,
) {
    if !matrix.consumer_projection.all_hold() {
        violations.push(M5ContractMatrixViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_proof_freshness(
    matrix: &M5DesignSystemContractMatrix,
    violations: &mut Vec<M5ContractMatrixViolation>,
) {
    let freshness = &matrix.proof_freshness;
    if freshness.proof_freshness_slo_hours == 0
        || freshness.last_proof_refresh.trim().is_empty()
        || !freshness.auto_narrow_on_stale
    {
        violations.push(M5ContractMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    matrix: &M5DesignSystemContractMatrix,
    violations: &mut Vec<M5ContractMatrixViolation>,
) {
    let posture = &matrix.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
        || !posture.stable_promotion_blocks_without_mapped_object
    {
        violations.push(M5ContractMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Returns true when the JSON tree carries any forbidden raw-boundary material (credential
/// bodies, raw provider payloads). Matrix packets are metadata-only by construction; this is
/// a defense-in-depth scan over the serialized export.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    const FORBIDDEN_KEYS: [&str; 6] = [
        "api_key",
        "authorization",
        "password",
        "secret",
        "access_token",
        "raw_payload",
    ];
    match value {
        serde_json::Value::Object(map) => {
            for (key, child) in map {
                if FORBIDDEN_KEYS.contains(&key.to_lowercase().as_str()) {
                    return true;
                }
                if json_contains_forbidden_boundary_material(child) {
                    return true;
                }
            }
            false
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Canonical artifact records the inventory references.
// ---------------------------------------------------------------------------

/// One token family inside the foundations artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TokenFamily {
    /// Stable token-family id.
    pub family_id: String,
    /// Human-readable family name.
    pub display_name: String,
    /// Semantic token references that belong to this family.
    pub semantic_token_refs: Vec<String>,
}

/// Canonical foundations/tokens artifact — the shape governed by
/// [`M5_FOUNDATIONS_SCHEMA_REF`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FoundationsArtifact {
    /// Record kind; must equal [`M5_FOUNDATIONS_ARTIFACT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DESIGN_SYSTEM_ARTIFACT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable foundations id.
    pub foundations_id: String,
    /// Owner role accountable for the foundations.
    pub owner_role: String,
    /// Governed token families.
    pub token_families: Vec<M5TokenFamily>,
    /// Theme-class tokens the foundations span.
    pub theme_classes: Vec<String>,
    /// Density-class tokens the foundations span.
    pub density_classes: Vec<String>,
    /// Motion-posture tokens the foundations span.
    pub motion_postures: Vec<String>,
    /// Repo-relative proof lane that blocks drift.
    pub proof_lane_ref: String,
    /// Stable summary message id; prefixed [`M5_CONTRACT_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
}

impl M5FoundationsArtifact {
    /// Validates the foundations artifact, returning stable issue tokens.
    pub fn validate(&self) -> Vec<&'static str> {
        let mut issues = Vec::new();
        if self.record_kind != M5_FOUNDATIONS_ARTIFACT_RECORD_KIND {
            issues.push("wrong_record_kind");
        }
        if self.schema_version != M5_DESIGN_SYSTEM_ARTIFACT_SCHEMA_VERSION {
            issues.push("wrong_schema_version");
        }
        if self.foundations_id.trim().is_empty() || self.owner_role.trim().is_empty() {
            issues.push("missing_identity");
        }
        if self.token_families.is_empty()
            || self
                .token_families
                .iter()
                .any(|f| f.semantic_token_refs.is_empty())
        {
            issues.push("token_families_incomplete");
        }
        if self.theme_classes.is_empty()
            || self.density_classes.is_empty()
            || self.motion_postures.is_empty()
        {
            issues.push("foundation_vocabulary_incomplete");
        }
        if self.proof_lane_ref.trim().is_empty() {
            issues.push("missing_proof_lane");
        }
        if !self
            .summary_message_id
            .starts_with(M5_CONTRACT_MESSAGE_ID_PREFIX)
        {
            issues.push("message_id_prefix_missing");
        }
        issues
    }
}

/// One anatomy part of a component contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AnatomyPart {
    /// Stable part id.
    pub part_id: String,
    /// Accessibility/semantic role of the part.
    pub role: String,
}

/// One keyboard binding declared by a component contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5KeyBinding {
    /// Key chord (e.g. `Enter`, `ArrowDown`).
    pub keys: String,
    /// Action the chord triggers.
    pub action: String,
}

/// Accessibility contract block for a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComponentAccessibility {
    /// Accessibility role of the component root.
    pub role: String,
    /// Screen-reader label rule.
    pub screen_reader_label_rule: String,
    /// Focus-order rule.
    pub focus_order_rule: String,
}

/// Canonical component-contract artifact — the shape governed by
/// [`M5_COMPONENT_CONTRACT_SCHEMA_REF`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComponentContractArtifact {
    /// Record kind; must equal [`M5_COMPONENT_CONTRACT_ARTIFACT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DESIGN_SYSTEM_ARTIFACT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable component id.
    pub component_id: String,
    /// Human-readable component name.
    pub display_name: String,
    /// Launch-critical surface class this component anchors.
    pub surface_class: LaunchSurfaceClass,
    /// Owner role accountable for the component contract.
    pub owner_role: String,
    /// Component anatomy parts.
    pub anatomy: Vec<M5AnatomyPart>,
    /// State-semantic families the component declares.
    pub states: Vec<CanonicalStateClass>,
    /// Keyboard model.
    pub keyboard_model: Vec<M5KeyBinding>,
    /// Accessibility contract.
    pub accessibility: M5ComponentAccessibility,
    /// Semantic token dependencies.
    pub token_dependencies: Vec<String>,
    /// Repo-relative extension-SDK guidance ref.
    pub extension_guidance_ref: String,
    /// Stable summary message id; prefixed [`M5_CONTRACT_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
}

impl M5ComponentContractArtifact {
    /// Validates the component-contract artifact, returning stable issue tokens.
    pub fn validate(&self) -> Vec<&'static str> {
        let mut issues = Vec::new();
        if self.record_kind != M5_COMPONENT_CONTRACT_ARTIFACT_RECORD_KIND {
            issues.push("wrong_record_kind");
        }
        if self.schema_version != M5_DESIGN_SYSTEM_ARTIFACT_SCHEMA_VERSION {
            issues.push("wrong_schema_version");
        }
        if self.component_id.trim().is_empty()
            || self.display_name.trim().is_empty()
            || self.owner_role.trim().is_empty()
        {
            issues.push("missing_identity");
        }
        // A launch-critical component declares anatomy, states, keyboard, accessibility, and
        // token dependencies — the full contract, not a partial sketch.
        if self.anatomy.is_empty() {
            issues.push("missing_anatomy");
        }
        if self.states.is_empty() {
            issues.push("missing_states");
        }
        if self.keyboard_model.is_empty() {
            issues.push("missing_keyboard_model");
        }
        if self.accessibility.role.trim().is_empty()
            || self
                .accessibility
                .screen_reader_label_rule
                .trim()
                .is_empty()
            || self.accessibility.focus_order_rule.trim().is_empty()
        {
            issues.push("missing_accessibility");
        }
        if self.token_dependencies.is_empty() {
            issues.push("missing_token_dependencies");
        }
        if self.extension_guidance_ref.trim().is_empty() {
            issues.push("missing_extension_guidance");
        }
        if !self
            .summary_message_id
            .starts_with(M5_CONTRACT_MESSAGE_ID_PREFIX)
        {
            issues.push("message_id_prefix_missing");
        }
        issues
    }
}

/// One shell slot inside the reference-layout artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellSlot {
    /// Stable slot id.
    pub slot_id: String,
    /// Accessibility/landmark role of the slot.
    pub role: String,
    /// Placeholder behavior when the slot is empty or loading.
    pub placeholder_behavior: String,
}

/// Placeholder policy for a reference layout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PlaceholderPolicy {
    /// Rule for an empty slot.
    pub empty_slot_rule: String,
    /// Rule for a loading slot.
    pub loading_slot_rule: String,
}

/// Canonical reference-layout artifact — the shape governed by
/// [`M5_REFERENCE_LAYOUT_SCHEMA_REF`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReferenceLayoutArtifact {
    /// Record kind; must equal [`M5_REFERENCE_LAYOUT_ARTIFACT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DESIGN_SYSTEM_ARTIFACT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable layout id.
    pub layout_id: String,
    /// Human-readable layout name.
    pub display_name: String,
    /// Owner role accountable for the reference layout.
    pub owner_role: String,
    /// Shell slots the layout governs.
    pub shell_slots: Vec<M5ShellSlot>,
    /// Placeholder policy across slots.
    pub placeholder_policy: M5PlaceholderPolicy,
    /// Stable summary message id; prefixed [`M5_CONTRACT_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
}

impl M5ReferenceLayoutArtifact {
    /// Validates the reference-layout artifact, returning stable issue tokens.
    pub fn validate(&self) -> Vec<&'static str> {
        let mut issues = Vec::new();
        if self.record_kind != M5_REFERENCE_LAYOUT_ARTIFACT_RECORD_KIND {
            issues.push("wrong_record_kind");
        }
        if self.schema_version != M5_DESIGN_SYSTEM_ARTIFACT_SCHEMA_VERSION {
            issues.push("wrong_schema_version");
        }
        if self.layout_id.trim().is_empty()
            || self.display_name.trim().is_empty()
            || self.owner_role.trim().is_empty()
        {
            issues.push("missing_identity");
        }
        if self.shell_slots.is_empty()
            || self
                .shell_slots
                .iter()
                .any(|s| s.placeholder_behavior.trim().is_empty())
        {
            issues.push("shell_slots_incomplete");
        }
        if self.placeholder_policy.empty_slot_rule.trim().is_empty()
            || self.placeholder_policy.loading_slot_rule.trim().is_empty()
        {
            issues.push("placeholder_policy_incomplete");
        }
        if !self
            .summary_message_id
            .starts_with(M5_CONTRACT_MESSAGE_ID_PREFIX)
        {
            issues.push("message_id_prefix_missing");
        }
        issues
    }
}

/// Helper exposing the canonical foundation vocabulary tokens the seed reuses.
pub(crate) fn theme_class_tokens() -> Vec<String> {
    [
        ThemeClass::DarkReference,
        ThemeClass::LightParity,
        ThemeClass::HighContrastDark,
        ThemeClass::HighContrastLight,
    ]
    .iter()
    .map(|v| v.token().to_owned())
    .collect()
}

/// Helper exposing the canonical density vocabulary tokens the seed reuses.
pub(crate) fn density_class_tokens() -> Vec<String> {
    [
        DensityClass::Compact,
        DensityClass::Standard,
        DensityClass::Comfortable,
    ]
    .iter()
    .map(|v| v.token().to_owned())
    .collect()
}

/// Helper exposing the canonical motion-posture vocabulary tokens the seed reuses.
pub(crate) fn motion_posture_tokens() -> Vec<String> {
    [
        AccessibilityPostureClass::MotionStandard,
        AccessibilityPostureClass::MotionReduced,
        AccessibilityPostureClass::MotionLowMotion,
        AccessibilityPostureClass::MotionPowerSaver,
        AccessibilityPostureClass::MotionCriticalHotPath,
    ]
    .iter()
    .map(|v| v.token().to_owned())
    .collect()
}
