//! Surface-qualification packet certifying claimed M5 surfaces against the four governed
//! design-system lanes.
//!
//! Where the [contract matrix](crate::m5_design_system_contract) freezes the *object model* the
//! design system ships, and the four lanes — the
//! [foundation package](crate::m5_foundation_package), the
//! [component manifests](crate::m5_component_manifest), the
//! [reference layouts](crate::m5_reference_layout), and the
//! [evidence pack](crate::m5_evidence_pack) — each ship one slice of design-system truth, this
//! module is the **integrating qualification packet**. It ties each claimed M5 surface to the
//! *current* foundation tokens, component contracts, layout descriptor, and visual/a11y proof it
//! depends on, derives a green/yellow/red qualification verdict from those lanes, and auto-narrows
//! (or blocks) the surface's public claim when component proof is stale, layout parity is missing,
//! or token/state conformance fails — so a claim can never outrun the contract and proof that back
//! it.
//!
//! Each [`M5SurfaceQualification`] binds a workspace surface to the four lanes
//! ([`M5LaneBinding`]), names the component families it renders, and resolves:
//!
//! - a [`M5QualificationStatus`] (qualified / provisional / disqualified) reflecting the *true*
//!   conformance, independent of waivers, so the dashboard never hides a real gap;
//! - a [`M5QualificationGate`] the release/public-truth automation reads — a surface whose bound
//!   contract artifact is missing is *blocked* from Stable promotion (and named, never hidden),
//!   while a surface whose proof is stale or whose token/state conformance fails *auto-narrows*
//!   below Stable before promotion;
//! - an effective [`M5DesignSystemClaimClass`](crate::M5DesignSystemClaimClass) after the gate
//!   applies, floored at Beta for any unwaived narrowing gap and at the disclosed waived claim for
//!   any accepted blocking gap.
//!
//! The packet is the single M5 source of surface-qualification truth: Help/About, the release
//! center, shiproom, support exports, and the stable-claim matrix consume the same
//! [`M5SurfaceQualificationDashboard`] projection and [`M5QualificationReleaseGate`] rather than
//! maintaining parallel spreadsheets. Raw provider payloads, credentials, secret material, and
//! untranslated free-text prose stay outside the support boundary.
//!
//! - Packet schema:
//!   [`schemas/design-system/m5-surface-qualification.schema.json`](../../../../../schemas/design-system/m5-surface-qualification.schema.json)
//! - Dashboard schema:
//!   [`schemas/design-system/m5-surface-qualification-dashboard.schema.json`](../../../../../schemas/design-system/m5-surface-qualification-dashboard.schema.json)
//! - Contract doc:
//!   [`docs/design-system/m5-surface-qualification.md`](../../../../../docs/design-system/m5-surface-qualification.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_surface_qualification_packet,
    seeded_m5_surface_qualification_packet_missing_manifest_blocked,
    seeded_m5_surface_qualification_packet_stale_narrowed,
    seeded_m5_surface_qualification_packet_token_drift_narrowed,
    seeded_m5_surface_qualification_packet_waived_narrowed, M5_SURFACE_QUALIFICATION_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_component_manifest::{
    M5ComponentKind, M5ComponentManifestPackage, M5_COMPONENT_MANIFEST_PROOF_REF,
    M5_COMPONENT_MANIFEST_SCHEMA_REF,
};
use crate::m5_design_system_contract::M5DesignSystemClaimClass;
use crate::m5_evidence_pack::{
    M5EvidenceClaimGate, M5EvidencePack, M5_EVIDENCE_PACK_PROOF_REF, M5_EVIDENCE_PACK_SCHEMA_REF,
};
use crate::m5_foundation_package::{
    M5FoundationFamilyKind, M5FoundationPackage, M5SupportState, M5_FOUNDATION_PACKAGE_PROOF_REF,
    M5_FOUNDATION_PACKAGE_SCHEMA_REF,
};
use crate::m5_reference_layout::{
    M5ReferenceLayoutPackage, M5WorkspaceKind, M5_REFERENCE_LAYOUT_PROOF_REF,
    M5_REFERENCE_LAYOUT_SCHEMA_REF,
};
use crate::CanonicalStateClass;

/// Record-kind tag carried by [`M5SurfaceQualificationPacket`].
pub const M5_SURFACE_QUALIFICATION_RECORD_KIND: &str = "m5_design_system_surface_qualification";

/// Schema version for surface-qualification packets.
pub const M5_SURFACE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Record-kind tag carried by [`M5SurfaceQualificationDashboard`].
pub const M5_SURFACE_QUALIFICATION_DASHBOARD_RECORD_KIND: &str =
    "m5_design_system_surface_qualification_dashboard";

/// Schema version for surface-qualification dashboards.
pub const M5_SURFACE_QUALIFICATION_DASHBOARD_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the packet boundary schema.
pub const M5_SURFACE_QUALIFICATION_SCHEMA_REF: &str =
    "schemas/design-system/m5-surface-qualification.schema.json";

/// Repo-relative path of the dashboard boundary schema.
pub const M5_SURFACE_QUALIFICATION_DASHBOARD_SCHEMA_REF: &str =
    "schemas/design-system/m5-surface-qualification-dashboard.schema.json";

/// Repo-relative path of the qualification contract doc.
pub const M5_SURFACE_QUALIFICATION_DOC_REF: &str = "docs/design-system/m5-surface-qualification.md";

/// Repo-relative path of the release-grade qualification support export — the proof lane the
/// release center, shiproom, and support tooling read.
pub const M5_SURFACE_QUALIFICATION_PROOF_REF: &str =
    "artifacts/release/m5-design-system-proof/surface-qualification.json";

/// Repo-relative path of the published qualification dashboard projection.
pub const M5_SURFACE_QUALIFICATION_DASHBOARD_PROOF_REF: &str =
    "artifacts/design-system/m5-surface-qualification-dashboard.json";

/// Repo-relative path of the drill fixtures the headless emitter mints.
pub const M5_SURFACE_QUALIFICATION_FIXTURE_DIR: &str = "fixtures/ui/m5-surface-qualification/";

/// Prefix every governed message id in this lane carries so consumers can route them.
pub const M5_QUALIFICATION_MESSAGE_ID_PREFIX: &str = "design_system_qualification.";

/// One of the four governed design-system lanes a surface qualification draws proof from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QualificationLane {
    /// Canonical token / density / motion / contrast / state foundation package.
    Foundation,
    /// Component-contract manifest package.
    ComponentContract,
    /// Reference-layout descriptor package.
    ReferenceLayout,
    /// Visual / accessibility evidence pack.
    Evidence,
}

impl M5QualificationLane {
    /// Every lane, in declaration order. A surface binds one [`M5LaneBinding`] per lane.
    pub const ALL: [Self; 4] = [
        Self::Foundation,
        Self::ComponentContract,
        Self::ReferenceLayout,
        Self::Evidence,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Foundation => "foundation",
            Self::ComponentContract => "component_contract",
            Self::ReferenceLayout => "reference_layout",
            Self::Evidence => "evidence",
        }
    }

    /// Repo-relative schema that governs this lane's canonical artifact.
    pub const fn lane_schema_ref(self) -> &'static str {
        match self {
            Self::Foundation => M5_FOUNDATION_PACKAGE_SCHEMA_REF,
            Self::ComponentContract => M5_COMPONENT_MANIFEST_SCHEMA_REF,
            Self::ReferenceLayout => M5_REFERENCE_LAYOUT_SCHEMA_REF,
            Self::Evidence => M5_EVIDENCE_PACK_SCHEMA_REF,
        }
    }

    /// Repo-relative proof lane the qualification consumed for this lane.
    pub const fn lane_proof_ref(self) -> &'static str {
        match self {
            Self::Foundation => M5_FOUNDATION_PACKAGE_PROOF_REF,
            Self::ComponentContract => M5_COMPONENT_MANIFEST_PROOF_REF,
            Self::ReferenceLayout => M5_REFERENCE_LAYOUT_PROOF_REF,
            Self::Evidence => M5_EVIDENCE_PACK_PROOF_REF,
        }
    }
}

/// Per-lane conformance outcome for a surface binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LaneConformance {
    /// The lane proves the binding and is current/valid.
    Conformant,
    /// The lane's proof has fallen outside its freshness window; the surface must narrow.
    Stale,
    /// The lane is present but conformance (token/state resolution) fails; the surface narrows.
    Nonconformant,
    /// A required contract artifact or usable proof is absent; the surface is blocked.
    Missing,
}

impl M5LaneConformance {
    /// Every conformance state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Conformant,
        Self::Stale,
        Self::Nonconformant,
        Self::Missing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Conformant => "conformant",
            Self::Stale => "stale",
            Self::Nonconformant => "nonconformant",
            Self::Missing => "missing",
        }
    }

    /// True when the lane blocks Stable promotion (without a waiver).
    pub const fn blocks(self) -> bool {
        matches!(self, Self::Missing)
    }

    /// True when the lane narrows the claim rather than blocking or certifying it.
    pub const fn narrows(self) -> bool {
        matches!(self, Self::Stale | Self::Nonconformant)
    }
}

/// Green/yellow/red qualification status for a claimed surface, reflecting *true* conformance
/// independent of waivers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QualificationStatus {
    /// Every lane is conformant and current.
    Qualified,
    /// A lane is stale or non-conformant; the surface ships at a narrowed claim.
    Provisional,
    /// A required contract artifact or usable proof is absent; the surface is disqualified.
    Disqualified,
}

impl M5QualificationStatus {
    /// Every status, in declaration order.
    pub const ALL: [Self; 3] = [Self::Qualified, Self::Provisional, Self::Disqualified];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::Provisional => "provisional",
            Self::Disqualified => "disqualified",
        }
    }

    /// The traffic-light signal this status maps to.
    pub const fn signal(self) -> M5QualificationSignal {
        match self {
            Self::Qualified => M5QualificationSignal::Green,
            Self::Provisional => M5QualificationSignal::Yellow,
            Self::Disqualified => M5QualificationSignal::Red,
        }
    }
}

/// Traffic-light signal for the published dashboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QualificationSignal {
    /// Qualified.
    Green,
    /// Provisional.
    Yellow,
    /// Disqualified.
    Red,
}

impl M5QualificationSignal {
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
pub enum M5QualificationGate {
    /// The surface may promote to Stable at its full claim.
    CertifiedPromote,
    /// The surface auto-narrows to a disclosed reduced claim before promotion.
    AutoNarrowed,
    /// The surface is blocked from Stable promotion by a missing contract or proof.
    Blocked,
}

impl M5QualificationGate {
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

/// A surface or system that consumes the qualification output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QualificationConsumer {
    /// Help and About surface the qualification headline and narrowed/blocked surfaces.
    HelpAbout,
    /// Release center gates promotion on the qualification.
    ReleaseCenter,
    /// Shiproom watches the qualification dashboard for regressions.
    Shiproom,
    /// Support export ships the qualification packet.
    SupportExport,
    /// The stable-claim matrix reads the effective claim per surface.
    StableClaimMatrix,
}

impl M5QualificationConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HelpAbout,
        Self::ReleaseCenter,
        Self::Shiproom,
        Self::SupportExport,
        Self::StableClaimMatrix,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about",
            Self::ReleaseCenter => "release_center",
            Self::Shiproom => "shiproom",
            Self::SupportExport => "support_export",
            Self::StableClaimMatrix => "stable_claim_matrix",
        }
    }
}

/// One kind of qualification gap a surface can carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QualificationGapKind {
    /// A bound component manifest names a foundation token the package does not publish.
    FoundationTokenUnresolved,
    /// A canonical controlled state is not published by the foundation state family.
    FoundationStateUnpublished,
    /// A bound component family has no published manifest.
    ComponentManifestMissing,
    /// The surface's workspace has no published reference layout.
    ReferenceLayoutMissing,
    /// A bound component family's evidence proof is stale.
    EvidenceStale,
    /// A bound component family's evidence coverage is incomplete and blocks the claim.
    EvidenceBlocked,
}

impl M5QualificationGapKind {
    /// Every gap kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FoundationTokenUnresolved,
        Self::FoundationStateUnpublished,
        Self::ComponentManifestMissing,
        Self::ReferenceLayoutMissing,
        Self::EvidenceStale,
        Self::EvidenceBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FoundationTokenUnresolved => "foundation_token_unresolved",
            Self::FoundationStateUnpublished => "foundation_state_unpublished",
            Self::ComponentManifestMissing => "component_manifest_missing",
            Self::ReferenceLayoutMissing => "reference_layout_missing",
            Self::EvidenceStale => "evidence_stale",
            Self::EvidenceBlocked => "evidence_blocked",
        }
    }

    /// The lane this gap belongs to.
    pub const fn lane(self) -> M5QualificationLane {
        match self {
            Self::FoundationTokenUnresolved | Self::FoundationStateUnpublished => {
                M5QualificationLane::Foundation
            }
            Self::ComponentManifestMissing => M5QualificationLane::ComponentContract,
            Self::ReferenceLayoutMissing => M5QualificationLane::ReferenceLayout,
            Self::EvidenceStale | Self::EvidenceBlocked => M5QualificationLane::Evidence,
        }
    }

    /// True when this gap blocks Stable promotion without a waiver (a missing contract / proof).
    pub const fn is_blocking(self) -> bool {
        matches!(
            self,
            Self::ComponentManifestMissing | Self::ReferenceLayoutMissing | Self::EvidenceBlocked
        )
    }
}

/// One lane's proof binding for a surface: which lane, the artifact and version it was qualified
/// against, and the per-lane conformance outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LaneBinding {
    /// The governed lane.
    pub lane: M5QualificationLane,
    /// Repo-relative canonical artifact the surface was qualified against.
    pub source_ref: String,
    /// Repo-relative schema that governs the lane artifact.
    pub schema_ref: String,
    /// The lane artifact's version (semver / pack version).
    pub source_version: String,
    /// Per-lane conformance outcome.
    pub conformance: M5LaneConformance,
    /// Stable message id; prefixed [`M5_QUALIFICATION_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

/// One qualification gap on a claimed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5QualificationGap {
    /// Surface this gap applies to.
    pub surface_id: String,
    /// The lane the gap concerns.
    pub lane: M5QualificationLane,
    /// The kind of gap.
    pub gap_kind: M5QualificationGapKind,
    /// The component family / token / state / workspace the gap concerns.
    pub subject: String,
    /// Whether this gap was accepted under an active waiver.
    pub waived: bool,
    /// Stable message id; prefixed [`M5_QUALIFICATION_MESSAGE_ID_PREFIX`].
    pub cause_message_id: String,
}

/// One active waiver accepting a disclosed reduced claim for a single blocking gap.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5QualificationWaiver {
    /// Stable waiver id.
    pub waiver_id: String,
    /// The gap kind the waiver scopes.
    pub gap_kind: M5QualificationGapKind,
    /// The gap subject the waiver scopes; the waiver only covers this gap.
    pub subject: String,
    /// Stable message id naming the reason; prefixed [`M5_QUALIFICATION_MESSAGE_ID_PREFIX`].
    pub reason_message_id: String,
    /// Owner role accountable for the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry timestamp of the waiver.
    pub expires_at: String,
    /// The disclosed reduced claim accepted under this waiver.
    pub narrowed_to: M5DesignSystemClaimClass,
}

/// The four lane packets a surface qualification is derived from.
#[derive(Debug, Clone, Copy)]
pub struct M5QualificationLaneInputs<'a> {
    /// Foundation package.
    pub foundation: &'a M5FoundationPackage,
    /// Component-manifest package.
    pub manifests: &'a M5ComponentManifestPackage,
    /// Reference-layout package.
    pub layouts: &'a M5ReferenceLayoutPackage,
    /// Evidence pack (already re-evaluated to the qualification's evaluation date).
    pub evidence: &'a M5EvidencePack,
}

/// Restrictiveness rank of a claim class (Stable least, Unavailable most). Local copy so the lane
/// stays self-contained.
fn claim_rank(class: M5DesignSystemClaimClass) -> u8 {
    match class {
        M5DesignSystemClaimClass::Stable => 0,
        M5DesignSystemClaimClass::Beta => 1,
        M5DesignSystemClaimClass::Preview => 2,
        M5DesignSystemClaimClass::Experimental => 3,
        M5DesignSystemClaimClass::Held => 4,
        M5DesignSystemClaimClass::Unavailable => 5,
    }
}

/// The more restrictive of two claim classes.
fn more_restrictive(
    a: M5DesignSystemClaimClass,
    b: M5DesignSystemClaimClass,
) -> M5DesignSystemClaimClass {
    if claim_rank(a) >= claim_rank(b) {
        a
    } else {
        b
    }
}

/// Derived verdict fields computed purely from a surface's gaps and waivers.
struct DerivedVerdict {
    status: M5QualificationStatus,
    signal: M5QualificationSignal,
    gate: M5QualificationGate,
    effective_class: M5DesignSystemClaimClass,
}

/// Derives the verdict from the stored gaps and waivers. This is the single source the seed
/// reconciles to and the validator recomputes against, so the stored verdict can never drift.
fn derive_verdict(
    claimed: M5DesignSystemClaimClass,
    gaps: &[M5QualificationGap],
    waivers: &[M5QualificationWaiver],
) -> DerivedVerdict {
    let any_blocking = gaps.iter().any(|g| g.gap_kind.is_blocking());
    let any_narrowing = gaps.iter().any(|g| !g.gap_kind.is_blocking());

    // The status reflects true conformance, independent of waivers, so the dashboard never hides a
    // real gap behind a waiver.
    let status = if any_blocking {
        M5QualificationStatus::Disqualified
    } else if any_narrowing {
        M5QualificationStatus::Provisional
    } else {
        M5QualificationStatus::Qualified
    };

    let unwaived_blocking = gaps.iter().any(|g| g.gap_kind.is_blocking() && !g.waived);
    let any_gap = !gaps.is_empty();

    let gate = if unwaived_blocking {
        M5QualificationGate::Blocked
    } else if any_gap {
        M5QualificationGate::AutoNarrowed
    } else {
        M5QualificationGate::CertifiedPromote
    };

    let effective_class = match gate {
        M5QualificationGate::CertifiedPromote => claimed,
        M5QualificationGate::Blocked => M5DesignSystemClaimClass::Held,
        M5QualificationGate::AutoNarrowed => {
            // Floor at Beta for any unwaived gap, then apply the most restrictive accepted claim
            // across active waivers.
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

    DerivedVerdict {
        status,
        signal: status.signal(),
        gate,
        effective_class,
    }
}

/// The set of foundation token identifiers a bound manifest may resolve against — the
/// `entry_id`s of every still-resolving (supported or deprecated) entry.
fn resolvable_foundation_tokens(foundation: &M5FoundationPackage) -> BTreeSet<&str> {
    foundation
        .families
        .iter()
        .flat_map(|family| family.entries.iter())
        .filter(|entry| entry.support_state != M5SupportState::Unsupported)
        .map(|entry| entry.entry_id.as_str())
        .collect()
}

/// The set of controlled-state tokens the foundation component-state family publishes (the
/// canonical state tokens carried as each entry's `value_token`).
fn published_state_tokens(foundation: &M5FoundationPackage) -> BTreeSet<&str> {
    foundation
        .family(M5FoundationFamilyKind::ComponentState)
        .map(|family| {
            family
                .entries
                .iter()
                .filter(|entry| entry.support_state != M5SupportState::Unsupported)
                .map(|entry| entry.value_token.as_str())
                .collect()
        })
        .unwrap_or_default()
}

/// One claimed M5 surface's qualification row: the workspace it covers, the component families it
/// renders, its per-lane bindings, its verdict, active waivers, and exact gaps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceQualification {
    /// Stable surface id, unique within the packet.
    pub surface_id: String,
    /// The workspace family this surface covers.
    pub workspace_kind: M5WorkspaceKind,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Owner role accountable for keeping this surface's qualification current.
    pub owner_role: String,
    /// The component families the surface renders (and binds to manifest + evidence).
    pub bound_component_kinds: Vec<M5ComponentKind>,
    /// Public claim the surface wants to keep (always Stable for claimed M5 surfaces).
    pub claimed_class: M5DesignSystemClaimClass,
    /// Effective claim after the qualification gate applies.
    pub effective_class: M5DesignSystemClaimClass,
    /// Green/yellow/red qualification status.
    pub status: M5QualificationStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: M5QualificationSignal,
    /// Release-gate decision the release/public-truth automation reads.
    pub gate_decision: M5QualificationGate,
    /// One binding per governed lane.
    pub lane_bindings: Vec<M5LaneBinding>,
    /// Active waivers accepting a disclosed reduced claim for one blocking gap each.
    pub waivers: Vec<M5QualificationWaiver>,
    /// Exact qualification gaps for this surface.
    pub gaps: Vec<M5QualificationGap>,
    /// Consumer surfaces that project this row's qualification.
    pub consumer_surfaces: Vec<M5QualificationConsumer>,
    /// Stable message id for the status; prefixed [`M5_QUALIFICATION_MESSAGE_ID_PREFIX`].
    pub status_message_id: String,
    /// Stable message id for the gate; prefixed [`M5_QUALIFICATION_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

impl M5SurfaceQualification {
    /// Recomputes the per-lane bindings and gaps from the four lane packets, then recomputes the
    /// verdict. The seed calls this after authoring or mutating a row so the derived blocks never
    /// need hand-maintenance, and so the qualification is always generated from the same checked-in
    /// contract Aureline ships.
    pub fn recompute(&mut self, inputs: &M5QualificationLaneInputs) {
        let mut gaps = Vec::new();
        let waived = |kind: M5QualificationGapKind, subject: &str| -> bool {
            self.waivers
                .iter()
                .any(|w| w.gap_kind == kind && w.subject == subject)
        };
        let push_gap =
            |gaps: &mut Vec<M5QualificationGap>, kind: M5QualificationGapKind, subject: String| {
                let cause_message_id = format!(
                    "{}{}.{}.{}.gap",
                    M5_QUALIFICATION_MESSAGE_ID_PREFIX,
                    self.surface_id,
                    kind.as_str(),
                    subject
                );
                gaps.push(M5QualificationGap {
                    surface_id: self.surface_id.clone(),
                    lane: kind.lane(),
                    gap_kind: kind,
                    waived: waived(kind, &subject),
                    subject,
                    cause_message_id,
                });
            };

        // Foundation lane: every bound manifest's token dependencies resolve to a published
        // foundation entry, and every canonical controlled state is published.
        let foundation_tokens = resolvable_foundation_tokens(inputs.foundation);
        let state_tokens = published_state_tokens(inputs.foundation);
        for kind in &self.bound_component_kinds {
            if let Some(manifest) = inputs.manifests.manifest(*kind) {
                for dep in &manifest.token_dependencies {
                    if !foundation_tokens.contains(dep.as_str()) {
                        push_gap(
                            &mut gaps,
                            M5QualificationGapKind::FoundationTokenUnresolved,
                            dep.clone(),
                        );
                    }
                }
            }
        }
        for state in CanonicalStateClass::required() {
            if !state_tokens.contains(state.as_str()) {
                push_gap(
                    &mut gaps,
                    M5QualificationGapKind::FoundationStateUnpublished,
                    state.as_str().to_owned(),
                );
            }
        }

        // Component-contract lane: every bound family has a published manifest.
        for kind in &self.bound_component_kinds {
            if inputs.manifests.manifest(*kind).is_none() {
                push_gap(
                    &mut gaps,
                    M5QualificationGapKind::ComponentManifestMissing,
                    kind.as_str().to_owned(),
                );
            }
        }

        // Reference-layout lane: the workspace has a published layout.
        if inputs.layouts.layout(self.workspace_kind).is_none() {
            push_gap(
                &mut gaps,
                M5QualificationGapKind::ReferenceLayoutMissing,
                self.workspace_kind.as_str().to_owned(),
            );
        }

        // Evidence lane: every bound family's evidence is current and complete.
        for kind in &self.bound_component_kinds {
            match inputs.evidence.component(*kind).map(|c| c.claim_gate) {
                Some(M5EvidenceClaimGate::Certified) => {}
                Some(M5EvidenceClaimGate::Narrowed) => push_gap(
                    &mut gaps,
                    M5QualificationGapKind::EvidenceStale,
                    kind.as_str().to_owned(),
                ),
                Some(M5EvidenceClaimGate::Blocked) | None => push_gap(
                    &mut gaps,
                    M5QualificationGapKind::EvidenceBlocked,
                    kind.as_str().to_owned(),
                ),
            }
        }

        gaps.sort_by(|a, b| {
            a.gap_kind
                .as_str()
                .cmp(b.gap_kind.as_str())
                .then(a.subject.cmp(&b.subject))
        });
        self.gaps = gaps;

        self.lane_bindings = M5QualificationLane::ALL
            .iter()
            .map(|&lane| self.lane_binding(lane, inputs))
            .collect();

        let verdict = derive_verdict(self.claimed_class, &self.gaps, &self.waivers);
        self.status = verdict.status;
        self.signal = verdict.signal;
        self.gate_decision = verdict.gate;
        self.effective_class = verdict.effective_class;
    }

    /// Builds one lane binding, resolving its conformance from the gaps that fall on that lane.
    fn lane_binding(
        &self,
        lane: M5QualificationLane,
        inputs: &M5QualificationLaneInputs,
    ) -> M5LaneBinding {
        let lane_gaps: Vec<&M5QualificationGap> =
            self.gaps.iter().filter(|g| g.lane == lane).collect();
        let conformance = if lane_gaps.iter().any(|g| g.gap_kind.is_blocking()) {
            M5LaneConformance::Missing
        } else if lane_gaps
            .iter()
            .any(|g| g.gap_kind == M5QualificationGapKind::EvidenceStale)
        {
            M5LaneConformance::Stale
        } else if !lane_gaps.is_empty() {
            M5LaneConformance::Nonconformant
        } else {
            M5LaneConformance::Conformant
        };
        let source_version = match lane {
            M5QualificationLane::Foundation => inputs.foundation.package_version.clone(),
            M5QualificationLane::ComponentContract => inputs.manifests.package_version.clone(),
            M5QualificationLane::ReferenceLayout => inputs.layouts.package_version.clone(),
            M5QualificationLane::Evidence => inputs.evidence.pack_version.clone(),
        };
        M5LaneBinding {
            lane,
            source_ref: lane.lane_proof_ref().to_owned(),
            schema_ref: lane.lane_schema_ref().to_owned(),
            source_version,
            conformance,
            detail_message_id: format!(
                "{}{}.{}.binding",
                M5_QUALIFICATION_MESSAGE_ID_PREFIX,
                self.surface_id,
                lane.as_str()
            ),
        }
    }

    /// True when the surface is blocked from Stable promotion.
    pub fn is_blocked(&self) -> bool {
        self.gate_decision.blocks()
    }

    /// True when the surface auto-narrowed below its claim.
    pub fn is_auto_narrowed(&self) -> bool {
        matches!(self.gate_decision, M5QualificationGate::AutoNarrowed)
    }

    /// True when the surface is fully qualified for Stable promotion.
    pub fn is_qualified(&self) -> bool {
        matches!(self.gate_decision, M5QualificationGate::CertifiedPromote)
    }
}

/// One bound lane's source revision the packet was qualified against.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5QualificationLaneSource {
    /// The governed lane.
    pub lane: M5QualificationLane,
    /// Repo-relative canonical artifact ref.
    pub source_ref: String,
    /// Repo-relative schema ref.
    pub schema_ref: String,
    /// The lane artifact's id.
    pub source_id: String,
    /// The lane artifact's version.
    pub source_version: String,
}

/// Packet-level release gate aggregating the per-surface gates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5QualificationReleaseGate {
    /// True when at least one surface is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
    /// Sorted surface ids blocked from Stable promotion.
    pub blocked_surface_ids: Vec<String>,
    /// Sorted surface ids that auto-narrowed below their claim.
    pub auto_narrowed_surface_ids: Vec<String>,
    /// Sorted surface ids fully qualified for Stable promotion.
    pub qualified_surface_ids: Vec<String>,
    /// Sorted surface ids carrying at least one active waiver.
    pub waived_surface_ids: Vec<String>,
    /// Stable message id; prefixed [`M5_QUALIFICATION_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

/// Self-describing controlled-vocabulary set so the packet resolves every token it carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5QualificationVocabularySet {
    /// Lane tokens.
    pub lanes: Vec<String>,
    /// Lane-conformance tokens.
    pub lane_conformances: Vec<String>,
    /// Qualification-status tokens.
    pub qualification_statuses: Vec<String>,
    /// Signal tokens.
    pub signals: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
    /// Gap-kind tokens.
    pub gap_kinds: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
    /// Claim-class tokens (contract-matrix owned).
    pub claim_classes: Vec<String>,
    /// Workspace-kind tokens (reference-layout owned).
    pub workspace_kinds: Vec<String>,
    /// Component-kind tokens (component-manifest owned).
    pub component_kinds: Vec<String>,
}

impl M5QualificationVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            lanes: M5QualificationLane::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            lane_conformances: M5LaneConformance::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            qualification_statuses: M5QualificationStatus::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            signals: M5QualificationSignal::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            gate_decisions: M5QualificationGate::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            gap_kinds: M5QualificationGapKind::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            consumers: M5QualificationConsumer::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            claim_classes: M5DesignSystemClaimClass::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            workspace_kinds: M5WorkspaceKind::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            component_kinds: M5ComponentKind::ALL
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

/// Qualification conformance review. Every flag is a hard invariant; all must hold for the packet
/// to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5QualificationConformanceReview {
    /// Every claimed surface binds all four governed lanes.
    pub every_surface_binds_all_four_lanes: bool,
    /// Every surface names the component families it renders.
    pub every_surface_names_bound_component_families: bool,
    /// Token / state conformance is computed from the foundation package.
    pub token_state_conformance_computed_from_foundation: bool,
    /// A missing bound contract artifact blocks Stable promotion.
    pub missing_contract_blocks_stable_promotion: bool,
    /// Stale proof or failing conformance auto-narrows before Stable promotion.
    pub stale_or_failing_conformance_auto_narrows_before_stable: bool,
    /// Active waivers are disclosed with scope, owner, and expiry.
    pub waivers_disclosed_with_scope_owner_and_expiry: bool,
    /// Exact qualification gaps are named per surface.
    pub exact_gaps_named: bool,
    /// The dashboard traffic-light counts match the rows.
    pub dashboard_traffic_light_matches_rows: bool,
    /// The qualification is generated from the same checked-in lane contracts.
    pub generated_from_checked_in_lane_contracts: bool,
    /// Support export carries no raw boundary material.
    pub support_export_carries_no_raw_boundary_material: bool,
}

impl M5QualificationConformanceReview {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.every_surface_binds_all_four_lanes
            && self.every_surface_names_bound_component_families
            && self.token_state_conformance_computed_from_foundation
            && self.missing_contract_blocks_stable_promotion
            && self.stale_or_failing_conformance_auto_narrows_before_stable
            && self.waivers_disclosed_with_scope_owner_and_expiry
            && self.exact_gaps_named
            && self.dashboard_traffic_light_matches_rows
            && self.generated_from_checked_in_lane_contracts
            && self.support_export_carries_no_raw_boundary_material
    }
}

/// Consumer projection block: who reads the qualification output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5QualificationConsumerProjection {
    /// Help and About surface the qualification headline.
    pub help_about_surfaces_qualification: bool,
    /// Release center gates on the qualification.
    pub release_center_gates_on_qualification: bool,
    /// Shiproom watches the qualification dashboard.
    pub shiproom_watches_qualification_dashboard: bool,
    /// Support export ships the qualification packet.
    pub support_export_ships_qualification: bool,
    /// The stable-claim matrix reads the effective claim.
    pub stable_claim_matrix_reads_effective_claim: bool,
}

impl M5QualificationConsumerProjection {
    /// True when every projection holds.
    pub fn all_hold(&self) -> bool {
        self.help_about_surfaces_qualification
            && self.release_center_gates_on_qualification
            && self.shiproom_watches_qualification_dashboard
            && self.support_export_ships_qualification
            && self.stable_claim_matrix_reads_effective_claim
    }
}

/// Compact green/yellow/red qualification dashboard — the published scoreboard projection the
/// Help/About, shiproom, release, and support surfaces all read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceQualificationDashboard {
    /// Record kind; must equal [`M5_SURFACE_QUALIFICATION_DASHBOARD_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SURFACE_QUALIFICATION_DASHBOARD_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Cross-ref to the packet this dashboard projects.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the qualification was computed as-of.
    pub evaluated_at: String,
    /// Total claimed surfaces.
    pub total_surfaces: u32,
    /// Green (qualified) count.
    pub green_count: u32,
    /// Yellow (provisional) count.
    pub yellow_count: u32,
    /// Red (disqualified) count.
    pub red_count: u32,
    /// Qualified surface ids (sorted).
    pub qualified_surface_ids: Vec<String>,
    /// Provisional surface ids (sorted).
    pub provisional_surface_ids: Vec<String>,
    /// Disqualified surface ids (sorted).
    pub disqualified_surface_ids: Vec<String>,
    /// Surface ids that auto-narrowed below their claim (sorted).
    pub auto_narrowed_surface_ids: Vec<String>,
    /// Surface ids blocked from Stable promotion (sorted).
    pub blocked_surface_ids: Vec<String>,
    /// Surface ids carrying at least one active waiver (sorted).
    pub waived_surface_ids: Vec<String>,
    /// Active waiver ids (sorted).
    pub active_waiver_ids: Vec<String>,
    /// True when at least one surface is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
    /// Exact qualification gaps across all surfaces.
    pub qualification_gaps: Vec<M5QualificationGap>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Stable message id; prefixed [`M5_QUALIFICATION_MESSAGE_ID_PREFIX`].
    pub dashboard_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

/// Constructor input for [`M5SurfaceQualificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SurfaceQualificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the qualification was computed as-of.
    pub evaluated_at: String,
    /// The four bound lane source revisions.
    pub lane_sources: Vec<M5QualificationLaneSource>,
    /// Per-surface qualification rows.
    pub surfaces: Vec<M5SurfaceQualification>,
    /// Controlled-vocabulary set.
    pub vocabulary_set: M5QualificationVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5QualificationConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5QualificationConsumerProjection,
    /// Packet-level release gate.
    pub release_gate: M5QualificationReleaseGate,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 surface-qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceQualificationPacket {
    /// Record kind; must equal [`M5_SURFACE_QUALIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SURFACE_QUALIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the qualification was computed as-of.
    pub evaluated_at: String,
    /// The four bound lane source revisions.
    pub lane_sources: Vec<M5QualificationLaneSource>,
    /// Per-surface qualification rows.
    pub surfaces: Vec<M5SurfaceQualification>,
    /// Controlled-vocabulary set.
    pub vocabulary_set: M5QualificationVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5QualificationConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5QualificationConsumerProjection,
    /// Packet-level release gate.
    pub release_gate: M5QualificationReleaseGate,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SurfaceQualificationPacket {
    /// Builds a qualification packet from seed input.
    pub fn new(input: M5SurfaceQualificationPacketInput) -> Self {
        Self {
            record_kind: M5_SURFACE_QUALIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_SURFACE_QUALIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            lane_sources: input.lane_sources,
            surfaces: input.surfaces,
            vocabulary_set: input.vocabulary_set,
            conformance_review: input.conformance_review,
            consumer_projection: input.consumer_projection,
            release_gate: input.release_gate,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release/public-truth automation must hold Stable promotion because at least
    /// one claimed surface is blocked.
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

    /// Finds a surface qualification by id.
    pub fn surface(&self, surface_id: &str) -> Option<&M5SurfaceQualification> {
        self.surfaces.iter().find(|s| s.surface_id == surface_id)
    }

    /// Builds the compact green/yellow/red dashboard projection from the rows.
    pub fn dashboard(&self) -> M5SurfaceQualificationDashboard {
        let by_status = |status: M5QualificationStatus| -> Vec<String> {
            let mut ids: Vec<String> = self
                .surfaces
                .iter()
                .filter(|s| s.status == status)
                .map(|s| s.surface_id.clone())
                .collect();
            ids.sort();
            ids
        };
        let by_predicate = |predicate: &dyn Fn(&M5SurfaceQualification) -> bool| -> Vec<String> {
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

        let mut qualification_gaps: Vec<M5QualificationGap> = self
            .surfaces
            .iter()
            .flat_map(|s| s.gaps.iter().cloned())
            .collect();
        qualification_gaps.sort_by(|a, b| {
            a.surface_id
                .cmp(&b.surface_id)
                .then(a.gap_kind.as_str().cmp(b.gap_kind.as_str()))
                .then(a.subject.cmp(&b.subject))
        });

        let count_signal = |signal: M5QualificationSignal| -> u32 {
            self.surfaces.iter().filter(|s| s.signal == signal).count() as u32
        };

        let blocked_surface_ids = by_predicate(&|s| s.is_blocked());

        M5SurfaceQualificationDashboard {
            record_kind: M5_SURFACE_QUALIFICATION_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_SURFACE_QUALIFICATION_DASHBOARD_SCHEMA_VERSION,
            packet_id: self.packet_id.clone(),
            report_label: self.report_label.clone(),
            evaluated_at: self.evaluated_at.clone(),
            total_surfaces: self.surfaces.len() as u32,
            green_count: count_signal(M5QualificationSignal::Green),
            yellow_count: count_signal(M5QualificationSignal::Yellow),
            red_count: count_signal(M5QualificationSignal::Red),
            qualified_surface_ids: by_status(M5QualificationStatus::Qualified),
            provisional_surface_ids: by_status(M5QualificationStatus::Provisional),
            disqualified_surface_ids: by_status(M5QualificationStatus::Disqualified),
            auto_narrowed_surface_ids: by_predicate(&|s| s.is_auto_narrowed()),
            blocked_surface_ids: blocked_surface_ids.clone(),
            waived_surface_ids: by_predicate(&|s| !s.waivers.is_empty()),
            active_waiver_ids,
            blocks_stable_promotion: !blocked_surface_ids.is_empty(),
            qualification_gaps,
            source_contract_refs: self.source_contract_refs.clone(),
            dashboard_message_id: format!("{}dashboard", M5_QUALIFICATION_MESSAGE_ID_PREFIX),
            minted_at: self.minted_at.clone(),
        }
    }

    /// Validates the qualification packet invariants.
    pub fn validate(&self) -> Vec<M5QualificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SURFACE_QUALIFICATION_RECORD_KIND {
            violations.push(M5QualificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SURFACE_QUALIFICATION_SCHEMA_VERSION {
            violations.push(M5QualificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5QualificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_lane_sources(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surfaces(self, &mut violations);
        validate_release_gate_aggregate(self, &mut violations);
        validate_dashboard(self, &mut violations);
        validate_conformance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 surface qualification serializes"),
        ) {
            violations.push(M5QualificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 surface qualification serializes")
    }

    /// Deterministic export-safe JSON for the dashboard projection.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn dashboard_json(&self) -> String {
        serde_json::to_string_pretty(&self.dashboard())
            .expect("m5 surface qualification dashboard serializes")
    }

    /// Deterministic Markdown proof for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let dashboard = self.dashboard();
        let mut out = String::new();
        out.push_str("# M5 Surface-Qualification Packet\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Surfaces: {} ({} qualified, {} provisional, {} disqualified)\n",
            dashboard.total_surfaces,
            dashboard.green_count,
            dashboard.yellow_count,
            dashboard.red_count
        ));
        out.push_str(&format!(
            "- Release gate: {} ({} blocked, {} auto-narrowed, {} qualified)\n",
            if self.release_gate.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            },
            self.release_gate.blocked_surface_ids.len(),
            self.release_gate.auto_narrowed_surface_ids.len(),
            self.release_gate.qualified_surface_ids.len()
        ));
        out.push_str(&format!(
            "- Active waivers: {}\n",
            dashboard.active_waiver_ids.len()
        ));

        out.push_str("\n## Bound lane sources\n\n");
        for source in &self.lane_sources {
            out.push_str(&format!(
                "- **{}**: `{}` ({} `{}`)\n",
                source.lane.as_str(),
                source.source_id,
                source.source_version,
                source.source_ref
            ));
        }

        out.push_str("\n## Claimed surfaces\n\n");
        for surface in &self.surfaces {
            out.push_str(&format!(
                "- **{}** (`{}`): `{}` ({}), claim `{}` → `{}`, gate `{}`\n",
                surface.surface_id,
                surface.workspace_kind.as_str(),
                surface.status.as_str(),
                surface.signal.as_str(),
                surface.claimed_class.as_str(),
                surface.effective_class.as_str(),
                surface.gate_decision.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", surface.owner_role));
            let kinds: Vec<&str> = surface
                .bound_component_kinds
                .iter()
                .map(|k| k.as_str())
                .collect();
            out.push_str(&format!("  - Renders: {}\n", kinds.join(", ")));
            for binding in &surface.lane_bindings {
                out.push_str(&format!(
                    "  - lane `{}`: `{}` ({})\n",
                    binding.lane.as_str(),
                    binding.conformance.as_str(),
                    binding.source_version
                ));
            }
            for gap in &surface.gaps {
                out.push_str(&format!(
                    "  - gap `{}` on `{}`{}\n",
                    gap.gap_kind.as_str(),
                    gap.subject,
                    if gap.waived { " (waived)" } else { "" }
                ));
            }
            for waiver in &surface.waivers {
                out.push_str(&format!(
                    "  - waiver `{}` on `{}` → `{}` (owner {}, expires {})\n",
                    waiver.waiver_id,
                    waiver.subject,
                    waiver.narrowed_to.as_str(),
                    waiver.owner_role,
                    waiver.expires_at
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in qualification export.
#[derive(Debug)]
pub enum M5QualificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5QualificationViolation>),
}

impl fmt::Display for M5QualificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "m5 surface qualification parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                write!(
                    formatter,
                    "m5 surface qualification failed validation: {}",
                    tokens.join(",")
                )
            }
        }
    }
}

impl Error for M5QualificationArtifactError {}

/// Validation failures emitted by [`M5SurfaceQualificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5QualificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The lane-source block is incomplete (not all four lanes, or a field missing).
    LaneSourcesIncomplete,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// Two surfaces share an id.
    DuplicateSurfaceId,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface does not claim Stable.
    SurfaceDoesNotClaimStable,
    /// A surface binds no component families.
    SurfaceBindsNoComponents,
    /// A surface does not bind all four governed lanes exactly once.
    SurfaceLaneBindingsIncomplete,
    /// A lane binding's conformance disagrees with the surface's gaps.
    LaneBindingInconsistent,
    /// A surface's verdict disagrees with its gaps and waivers.
    DerivedVerdictInconsistent,
    /// A waiver is incomplete.
    WaiverIncomplete,
    /// A waiver scopes a gap the surface does not carry.
    WaiverGapUnknown,
    /// A surface has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A message id is missing the governed prefix.
    MessageIdPrefixMissing,
    /// The packet-level release gate disagrees with the per-surface gates.
    ReleaseGateAggregateInconsistent,
    /// The dashboard projection disagrees with the rows.
    DashboardInconsistent,
    /// Conformance review does not satisfy required invariants.
    ConformanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5QualificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::LaneSourcesIncomplete => "lane_sources_incomplete",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::DuplicateSurfaceId => "duplicate_surface_id",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::SurfaceDoesNotClaimStable => "surface_does_not_claim_stable",
            Self::SurfaceBindsNoComponents => "surface_binds_no_components",
            Self::SurfaceLaneBindingsIncomplete => "surface_lane_bindings_incomplete",
            Self::LaneBindingInconsistent => "lane_binding_inconsistent",
            Self::DerivedVerdictInconsistent => "derived_verdict_inconsistent",
            Self::WaiverIncomplete => "waiver_incomplete",
            Self::WaiverGapUnknown => "waiver_gap_unknown",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::MessageIdPrefixMissing => "message_id_prefix_missing",
            Self::ReleaseGateAggregateInconsistent => "release_gate_aggregate_inconsistent",
            Self::DashboardInconsistent => "dashboard_inconsistent",
            Self::ConformanceReviewIncomplete => "conformance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in qualification support export.
pub fn current_stable_m5_surface_qualification_packet(
) -> Result<M5SurfaceQualificationPacket, M5QualificationArtifactError> {
    let packet: M5SurfaceQualificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-design-system-proof/surface-qualification.json"
    )))
    .map_err(M5QualificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5QualificationArtifactError::Validation(violations))
    }
}

/// Reads the checked-in dashboard projection.
pub fn current_stable_m5_surface_qualification_dashboard(
) -> Result<M5SurfaceQualificationDashboard, M5QualificationArtifactError> {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/design-system/m5-surface-qualification-dashboard.json"
    )))
    .map_err(M5QualificationArtifactError::SupportExport)
}

fn validate_source_contracts(
    packet: &M5SurfaceQualificationPacket,
    violations: &mut Vec<M5QualificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SURFACE_QUALIFICATION_SCHEMA_REF,
        M5_SURFACE_QUALIFICATION_DASHBOARD_SCHEMA_REF,
        M5_FOUNDATION_PACKAGE_SCHEMA_REF,
        M5_COMPONENT_MANIFEST_SCHEMA_REF,
        M5_REFERENCE_LAYOUT_SCHEMA_REF,
        M5_EVIDENCE_PACK_SCHEMA_REF,
        M5_SURFACE_QUALIFICATION_DOC_REF,
        M5_SURFACE_QUALIFICATION_PROOF_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5QualificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_lane_sources(
    packet: &M5SurfaceQualificationPacket,
    violations: &mut Vec<M5QualificationViolation>,
) {
    let present: BTreeSet<M5QualificationLane> =
        packet.lane_sources.iter().map(|s| s.lane).collect();
    if present.len() != M5QualificationLane::ALL.len() {
        violations.push(M5QualificationViolation::LaneSourcesIncomplete);
        return;
    }
    for source in &packet.lane_sources {
        if source.source_ref.trim().is_empty()
            || source.schema_ref.trim().is_empty()
            || source.source_id.trim().is_empty()
            || source.source_version.trim().is_empty()
            || source.source_ref != source.lane.lane_proof_ref()
            || source.schema_ref != source.lane.lane_schema_ref()
        {
            violations.push(M5QualificationViolation::LaneSourcesIncomplete);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5SurfaceQualificationPacket,
    violations: &mut Vec<M5QualificationViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5QualificationViolation::VocabularySetDrift);
    }
}

fn validate_surfaces(
    packet: &M5SurfaceQualificationPacket,
    violations: &mut Vec<M5QualificationViolation>,
) {
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for surface in &packet.surfaces {
        if !seen_ids.insert(surface.surface_id.as_str()) {
            violations.push(M5QualificationViolation::DuplicateSurfaceId);
        }
        if surface.surface_id.trim().is_empty()
            || surface.surface_label.trim().is_empty()
            || surface.owner_role.trim().is_empty()
        {
            violations.push(M5QualificationViolation::SurfaceRowIncomplete);
        }
        if !surface.claimed_class.is_stable() {
            violations.push(M5QualificationViolation::SurfaceDoesNotClaimStable);
        }
        if surface.bound_component_kinds.is_empty() {
            violations.push(M5QualificationViolation::SurfaceBindsNoComponents);
        }
        if surface.consumer_surfaces.is_empty() {
            violations.push(M5QualificationViolation::ConsumerSurfacesMissing);
        }
        if !surface
            .status_message_id
            .starts_with(M5_QUALIFICATION_MESSAGE_ID_PREFIX)
            || !surface
                .gate_message_id
                .starts_with(M5_QUALIFICATION_MESSAGE_ID_PREFIX)
        {
            violations.push(M5QualificationViolation::MessageIdPrefixMissing);
        }

        validate_surface_lane_bindings(surface, violations);
        validate_surface_gaps(surface, violations);
        validate_surface_waivers(surface, violations);
        validate_surface_verdict(surface, violations);
    }
}

fn validate_surface_lane_bindings(
    surface: &M5SurfaceQualification,
    violations: &mut Vec<M5QualificationViolation>,
) {
    let bound: BTreeSet<M5QualificationLane> =
        surface.lane_bindings.iter().map(|b| b.lane).collect();
    if bound.len() != surface.lane_bindings.len() || bound.len() != M5QualificationLane::ALL.len() {
        violations.push(M5QualificationViolation::SurfaceLaneBindingsIncomplete);
        return;
    }
    for binding in &surface.lane_bindings {
        if binding.source_ref != binding.lane.lane_proof_ref()
            || binding.schema_ref != binding.lane.lane_schema_ref()
            || binding.source_version.trim().is_empty()
            || !binding
                .detail_message_id
                .starts_with(M5_QUALIFICATION_MESSAGE_ID_PREFIX)
        {
            violations.push(M5QualificationViolation::SurfaceLaneBindingsIncomplete);
        }
        // The recorded conformance must match what the surface's gaps imply for the lane.
        let lane_gaps: Vec<&M5QualificationGap> = surface
            .gaps
            .iter()
            .filter(|g| g.lane == binding.lane)
            .collect();
        let expected = if lane_gaps.iter().any(|g| g.gap_kind.is_blocking()) {
            M5LaneConformance::Missing
        } else if lane_gaps
            .iter()
            .any(|g| g.gap_kind == M5QualificationGapKind::EvidenceStale)
        {
            M5LaneConformance::Stale
        } else if !lane_gaps.is_empty() {
            M5LaneConformance::Nonconformant
        } else {
            M5LaneConformance::Conformant
        };
        if binding.conformance != expected {
            violations.push(M5QualificationViolation::LaneBindingInconsistent);
        }
    }
}

fn validate_surface_gaps(
    surface: &M5SurfaceQualification,
    violations: &mut Vec<M5QualificationViolation>,
) {
    for gap in &surface.gaps {
        if gap.surface_id != surface.surface_id
            || gap.lane != gap.gap_kind.lane()
            || gap.subject.trim().is_empty()
            || !gap
                .cause_message_id
                .starts_with(M5_QUALIFICATION_MESSAGE_ID_PREFIX)
        {
            violations.push(M5QualificationViolation::SurfaceRowIncomplete);
        }
        // The waived flag must agree with the active waivers.
        let waived = surface
            .waivers
            .iter()
            .any(|w| w.gap_kind == gap.gap_kind && w.subject == gap.subject);
        if waived != gap.waived {
            violations.push(M5QualificationViolation::DerivedVerdictInconsistent);
        }
    }
}

fn validate_surface_waivers(
    surface: &M5SurfaceQualification,
    violations: &mut Vec<M5QualificationViolation>,
) {
    for waiver in &surface.waivers {
        if waiver.waiver_id.trim().is_empty()
            || waiver.subject.trim().is_empty()
            || waiver.owner_role.trim().is_empty()
            || waiver.expires_at.trim().is_empty()
            || !waiver
                .reason_message_id
                .starts_with(M5_QUALIFICATION_MESSAGE_ID_PREFIX)
        {
            violations.push(M5QualificationViolation::WaiverIncomplete);
        }
        if !surface
            .gaps
            .iter()
            .any(|g| g.gap_kind == waiver.gap_kind && g.subject == waiver.subject)
        {
            violations.push(M5QualificationViolation::WaiverGapUnknown);
        }
    }
}

fn validate_surface_verdict(
    surface: &M5SurfaceQualification,
    violations: &mut Vec<M5QualificationViolation>,
) {
    let derived = derive_verdict(surface.claimed_class, &surface.gaps, &surface.waivers);
    if surface.status != derived.status
        || surface.signal != derived.signal
        || surface.gate_decision != derived.gate
        || surface.effective_class != derived.effective_class
    {
        violations.push(M5QualificationViolation::DerivedVerdictInconsistent);
    }
}

fn validate_release_gate_aggregate(
    packet: &M5SurfaceQualificationPacket,
    violations: &mut Vec<M5QualificationViolation>,
) {
    let collect = |predicate: &dyn Fn(&M5SurfaceQualification) -> bool| -> Vec<String> {
        let mut ids: Vec<String> = packet
            .surfaces
            .iter()
            .filter(|s| predicate(s))
            .map(|s| s.surface_id.clone())
            .collect();
        ids.sort();
        ids
    };
    let blocked = collect(&|s| s.is_blocked());
    let expected = M5QualificationReleaseGate {
        blocks_stable_promotion: !blocked.is_empty(),
        blocked_surface_ids: blocked,
        auto_narrowed_surface_ids: collect(&|s| s.is_auto_narrowed()),
        qualified_surface_ids: collect(&|s| s.is_qualified()),
        waived_surface_ids: collect(&|s| !s.waivers.is_empty()),
        gate_message_id: format!("{}release_gate", M5_QUALIFICATION_MESSAGE_ID_PREFIX),
    };
    if packet.release_gate != expected {
        violations.push(M5QualificationViolation::ReleaseGateAggregateInconsistent);
    }
}

fn validate_dashboard(
    packet: &M5SurfaceQualificationPacket,
    violations: &mut Vec<M5QualificationViolation>,
) {
    let dashboard = packet.dashboard();
    if dashboard.total_surfaces != packet.surfaces.len() as u32
        || dashboard.green_count + dashboard.yellow_count + dashboard.red_count
            != dashboard.total_surfaces
        || dashboard.blocks_stable_promotion != packet.blocks_stable_promotion()
    {
        violations.push(M5QualificationViolation::DashboardInconsistent);
    }
}

fn validate_conformance_review(
    packet: &M5SurfaceQualificationPacket,
    violations: &mut Vec<M5QualificationViolation>,
) {
    if !packet.conformance_review.all_hold() {
        violations.push(M5QualificationViolation::ConformanceReviewIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &M5SurfaceQualificationPacket,
    violations: &mut Vec<M5QualificationViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(M5QualificationViolation::ConsumerProjectionIncomplete);
    }
}

/// Scans a serialized packet for forbidden raw boundary material as defense in depth.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    const FORBIDDEN: [&str; 5] = [
        "api_key",
        "password",
        "authorization",
        "bearer ",
        "secret_key",
    ];
    match value {
        serde_json::Value::String(text) => {
            let lower = text.to_lowercase();
            FORBIDDEN.iter().any(|needle| lower.contains(needle))
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(map) => {
            map.keys().any(|key| {
                let lower = key.to_lowercase();
                FORBIDDEN.iter().any(|needle| lower.contains(needle))
            }) || map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
