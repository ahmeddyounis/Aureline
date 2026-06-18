//! Release-grade certification that built-in package-manager mutation,
//! registry-auth continuity, lockfile-safe review, and package-state truth hold
//! on every claimed ecosystem and every deployment profile, with automatic
//! downgrade when evidence goes stale or cross-surface parity breaks.
//!
//! Where [`crate::ecosystem_qualification_certification`] certifies the broad
//! qualification lanes per ecosystem, this module certifies the **package
//! mutation** claim specifically, and it widens the matrix by the dimension that
//! decides whether a mutation can be trusted in the field: the
//! [`DeploymentProfile`] — a direct primary registry, a registry mirror, or an
//! offline snapshot. The matrix holds one [`MutationCertificationRow`] for every
//! (ecosystem, deployment-profile) cell, so a claim proven against a direct
//! registry never silently extends to the mirror or offline rows it was never
//! tested on.
//!
//! Each row certifies the four mutation-proof dimensions the lane must keep
//! honest — [`ProofDimension::PackageStateTruth`],
//! [`ProofDimension::RegistryAuthContinuity`],
//! [`ProofDimension::LockfileSafeReview`], and
//! [`ProofDimension::CrossSurfaceParity`] — each carrying a
//! [`DimensionProofState`] of `proven`, `degraded`, `stale`, or `unproven`.
//!
//! The model is a publication gate, not a label store. The claim a row may
//! *publish* is derived deterministically and fails closed: it is the weakest of
//! the declared claim, the [`EvidenceFreshness`] ceiling, and every dimension
//! proof's ceiling. A stale dimension or stale freshness narrows to
//! [`MutationClaimClass::RetestPending`]; a degraded (mirror/offline-only)
//! dimension narrows to [`MutationClaimClass::Limited`]; an unproven dimension or
//! expired evidence withholds the claim as [`MutationClaimClass::Unsupported`].
//! Because [`MutationCertificationRow::published_claim`] and
//! [`MutationCertificationRow::narrowing_action`] are validated against the
//! recomputed gate decision, release/public-truth surfaces can prove
//! underqualified rows narrow automatically instead of overclaiming.
//!
//! Cross-surface parity is made mechanical. Every row carries a
//! [`SurfaceParityCell`] for each [`ParitySurface`] — product, CLI, docs/help,
//! and support/export — and the recorded [`ProofDimension::CrossSurfaceParity`]
//! state must equal the state recomputed from those cells: any divergent surface
//! forces `unproven`, any absent surface caps at `degraded`. So a row cannot read
//! green while product, CLI, docs/help, and support packets disagree.
//!
//! The packet binds to the frozen matrix at
//! [`crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix`]
//! through [`PackageMutationCertification::matrix_ref`], so every claimed surface
//! references the one shared package-state vocabulary rather than ecosystem-local
//! folklore.
//!
//! The packet is checked in at
//! `artifacts/deps/m5/package-mutation-certification.json` and embedded here, so
//! this typed consumer and any CI gate agree on every row without a cargo build
//! in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no credential bodies, raw provider payloads, registry tokens, or
//! private registry URLs.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix::M5_PACKAGE_STATE_MATRIX_PATH;

/// Supported package-mutation certification packet schema version.
pub const PACKAGE_MUTATION_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const PACKAGE_MUTATION_CERTIFICATION_RECORD_KIND: &str = "package_mutation_certification";

/// Repo-relative path to the checked-in packet.
pub const PACKAGE_MUTATION_CERTIFICATION_PATH: &str =
    "artifacts/deps/m5/package-mutation-certification.json";

/// Embedded checked-in packet JSON.
pub const PACKAGE_MUTATION_CERTIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/deps/m5/package-mutation-certification.json"
));

/// A marketed ecosystem the package-mutation matrix certifies.
///
/// The tokens match [`crate::ecosystem_qualification_certification::ClaimedEcosystem`]
/// so the two certification packets agree on ecosystem identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedEcosystem {
    /// Rust Cargo workspace and crate manifests.
    Cargo,
    /// Node package manifests using pnpm workspace semantics.
    NodePnpm,
    /// Python pip / project manifests.
    PythonPip,
}

impl CertifiedEcosystem {
    /// Every certified ecosystem, in declaration order.
    pub const ALL: [Self; 3] = [Self::Cargo, Self::NodePnpm, Self::PythonPip];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cargo => "cargo",
            Self::NodePnpm => "node_pnpm",
            Self::PythonPip => "python_pip",
        }
    }
}

/// The deployment profile a mutation claim is certified against.
///
/// A mutation that is trustworthy against a direct primary registry is not
/// automatically trustworthy through a mirror or from an offline snapshot, so the
/// matrix certifies each profile as its own row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfile {
    /// The ecosystem's primary registry reached directly and online.
    DirectRegistry,
    /// An enterprise or organization registry mirror.
    RegistryMirror,
    /// An offline / cache-only snapshot with no live registry reach.
    OfflineSnapshot,
}

impl DeploymentProfile {
    /// Every deployment profile, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::DirectRegistry,
        Self::RegistryMirror,
        Self::OfflineSnapshot,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectRegistry => "direct_registry",
            Self::RegistryMirror => "registry_mirror",
            Self::OfflineSnapshot => "offline_snapshot",
        }
    }
}

/// A mutation-proof dimension certified on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofDimension {
    /// Current package-state truth: requested/resolved identity, relation,
    /// advisory/license overlays, and resolution environment.
    PackageStateTruth,
    /// Registry-auth continuity: a credential reaches the registry or mirror and
    /// degradation states stay distinct from a generic failure.
    RegistryAuthContinuity,
    /// Lockfile-safe review: the pre-apply review, lockfile diff class, and
    /// rollback checkpoint hold before any mutation commits.
    LockfileSafeReview,
    /// Cross-surface parity: product, CLI, docs/help, and support packets express
    /// the same mutation truth.
    CrossSurfaceParity,
}

impl ProofDimension {
    /// Every proof dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PackageStateTruth,
        Self::RegistryAuthContinuity,
        Self::LockfileSafeReview,
        Self::CrossSurfaceParity,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageStateTruth => "package_state_truth",
            Self::RegistryAuthContinuity => "registry_auth_continuity",
            Self::LockfileSafeReview => "lockfile_safe_review",
            Self::CrossSurfaceParity => "cross_surface_parity",
        }
    }
}

/// The proof state of one dimension on one row.
///
/// Ordered low-to-high by [`DimensionProofState::rank`]: an `unproven` dimension
/// carries no claim and a `proven` dimension carries a full, current claim. Each
/// state implies the strongest [`MutationClaimClass`] a row may publish for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DimensionProofState {
    /// Current evidence proves the dimension fully.
    Proven,
    /// Proven only under degraded (mirror/offline-only) conditions.
    Degraded,
    /// Evidence exists but is past its freshness SLO; must be re-tested.
    Stale,
    /// No proof, or the dimension's parity/continuity is broken.
    Unproven,
}

impl DimensionProofState {
    /// Every dimension proof state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Proven, Self::Degraded, Self::Stale, Self::Unproven];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proven => "proven",
            Self::Degraded => "degraded",
            Self::Stale => "stale",
            Self::Unproven => "unproven",
        }
    }

    /// Monotonic rank; higher means stronger proof.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Unproven => 0,
            Self::Stale => 1,
            Self::Degraded => 2,
            Self::Proven => 3,
        }
    }

    /// The strongest claim a row may publish for a dimension in this state.
    pub const fn claim_ceiling(self) -> MutationClaimClass {
        match self {
            Self::Proven => MutationClaimClass::Certified,
            Self::Degraded => MutationClaimClass::Limited,
            Self::Stale => MutationClaimClass::RetestPending,
            Self::Unproven => MutationClaimClass::Unsupported,
        }
    }
}

/// The claim class published for a certification row.
///
/// Ordered low-to-high by [`MutationClaimClass::rank`]. `Certified` is a full,
/// current, cross-surface-consistent mutation claim; the three fail-closed
/// classes narrow it without ever overclaiming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationClaimClass {
    /// Full, current, evidence-backed package-mutation claim.
    Certified,
    /// A scoped claim that holds only under degraded (mirror/offline) conditions.
    Limited,
    /// No current claim; evidence is stale and must be re-tested.
    RetestPending,
    /// No claim; the row is unsupported or its parity/continuity is broken.
    Unsupported,
}

impl MutationClaimClass {
    /// Every claim class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Certified,
        Self::Limited,
        Self::RetestPending,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Limited => "limited",
            Self::RetestPending => "retest_pending",
            Self::Unsupported => "unsupported",
        }
    }

    /// Monotonic rank; higher means a stronger claim.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Unsupported => 0,
            Self::RetestPending => 1,
            Self::Limited => 2,
            Self::Certified => 3,
        }
    }

    /// The weaker (lower-rank) of two claim classes.
    pub const fn min(self, other: Self) -> Self {
        if other.rank() < self.rank() {
            other
        } else {
            self
        }
    }
}

/// Freshness of a row's certification evidence relative to its freshness SLO.
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

    /// The strongest claim this freshness alone permits a row to publish.
    ///
    /// Only `current` evidence may publish a full certification; `stale` or
    /// `unknown` freshness narrows to retest-pending, and `expired` evidence
    /// withholds the claim entirely.
    pub const fn claim_ceiling(self) -> MutationClaimClass {
        match self {
            Self::Current => MutationClaimClass::Certified,
            Self::Stale | Self::Unknown => MutationClaimClass::RetestPending,
            Self::Expired => MutationClaimClass::Unsupported,
        }
    }
}

/// The action the publication gate takes on a row relative to a full
/// certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimNarrowing {
    /// No narrowing; the row publishes a full certification.
    None,
    /// Narrow the published claim to a limited (degraded-conditions) claim.
    NarrowToLimited,
    /// Narrow the published claim to retest-pending (no current claim).
    NarrowToRetestPending,
    /// Withhold the row from publication as unsupported.
    WithholdAsUnsupported,
}

impl ClaimNarrowing {
    /// Every narrowing action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::None,
        Self::NarrowToLimited,
        Self::NarrowToRetestPending,
        Self::WithholdAsUnsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::NarrowToLimited => "narrow_to_limited",
            Self::NarrowToRetestPending => "narrow_to_retest_pending",
            Self::WithholdAsUnsupported => "withhold_as_unsupported",
        }
    }

    /// The narrowing action implied by a published claim.
    pub const fn for_published(claim: MutationClaimClass) -> Self {
        match claim {
            MutationClaimClass::Certified => Self::None,
            MutationClaimClass::Limited => Self::NarrowToLimited,
            MutationClaimClass::RetestPending => Self::NarrowToRetestPending,
            MutationClaimClass::Unsupported => Self::WithholdAsUnsupported,
        }
    }
}

/// A surface the cross-surface parity dimension is checked across.
///
/// These are exactly the surfaces the track invariant names: a claim must stay
/// consistent across product, CLI, docs/help, and support packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParitySurface {
    /// The desktop product surface.
    Product,
    /// The CLI / headless surface.
    Cli,
    /// The docs / Help/About surface.
    DocsHelp,
    /// The partner / support-export packet surface.
    SupportExport,
}

impl ParitySurface {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Product,
        Self::Cli,
        Self::DocsHelp,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Product => "product",
            Self::Cli => "cli",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
        }
    }
}

/// The parity state of one surface on one row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityState {
    /// The surface expresses the same mutation truth as the row.
    Consistent,
    /// The surface expresses a conflicting claim; parity is broken.
    Divergent,
    /// The surface does not yet carry the row; parity coverage is incomplete.
    Absent,
}

impl ParityState {
    /// Every parity state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Consistent, Self::Divergent, Self::Absent];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consistent => "consistent",
            Self::Divergent => "divergent",
            Self::Absent => "absent",
        }
    }
}

/// One certified proof dimension on a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionProof {
    /// The dimension this proof certifies.
    pub dimension: ProofDimension,
    /// The dimension's proof state.
    pub state: DimensionProofState,
    /// Ref to the proof artifact backing this dimension.
    pub evidence_ref: String,
}

/// One surface's parity state on a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceParityCell {
    /// The surface this cell describes.
    pub surface: ParitySurface,
    /// The surface's parity state.
    pub state: ParityState,
}

/// One certification row for an (ecosystem, deployment-profile) cell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutationCertificationRow {
    /// Stable row id.
    pub row_id: String,
    /// Ecosystem this row certifies.
    pub ecosystem: CertifiedEcosystem,
    /// Deployment profile this row certifies.
    pub deployment_profile: DeploymentProfile,
    /// Claim the lane asserts before the publication gate.
    pub declared_claim: MutationClaimClass,
    /// Claim actually published after the gate narrows the row.
    ///
    /// Must equal [`MutationCertificationRow::effective_claim`]; validation
    /// rejects a row that publishes beyond what its evidence supports.
    pub published_claim: MutationClaimClass,
    /// Freshness of the row's certification evidence.
    pub evidence_freshness: EvidenceFreshness,
    /// Action the gate takes on this row; must equal the recomputed narrowing.
    pub narrowing_action: ClaimNarrowing,
    /// One proof per [`ProofDimension`]; complete and without duplicates.
    pub dimension_proofs: Vec<DimensionProof>,
    /// One parity cell per [`ParitySurface`]; complete and without duplicates.
    pub surface_parity: Vec<SurfaceParityCell>,
    /// Ref to the row's own qualification proof packet.
    pub qualification_packet_ref: String,
    /// Ref to the row's own proof corpus.
    pub corpus_ref: String,
    /// Source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl MutationCertificationRow {
    /// Returns the proof for a dimension, if present.
    pub fn dimension_proof(&self, dimension: ProofDimension) -> Option<&DimensionProof> {
        self.dimension_proofs
            .iter()
            .find(|p| p.dimension == dimension)
    }

    /// Returns the parity cell for a surface, if present.
    pub fn parity_cell(&self, surface: ParitySurface) -> Option<&SurfaceParityCell> {
        self.surface_parity.iter().find(|c| c.surface == surface)
    }

    /// The cross-surface parity state recomputed from the surface cells.
    ///
    /// Any divergent surface forces `unproven`; otherwise any absent surface caps
    /// the dimension at `degraded`; a fully consistent set is `proven`.
    pub fn recomputed_parity_state(&self) -> DimensionProofState {
        if self
            .surface_parity
            .iter()
            .any(|c| c.state == ParityState::Divergent)
        {
            DimensionProofState::Unproven
        } else if self
            .surface_parity
            .iter()
            .any(|c| c.state == ParityState::Absent)
        {
            DimensionProofState::Degraded
        } else {
            DimensionProofState::Proven
        }
    }

    /// Whether the recorded cross-surface parity dimension matches the state
    /// recomputed from the surface cells.
    pub fn parity_consistent(&self) -> bool {
        match self.dimension_proof(ProofDimension::CrossSurfaceParity) {
            Some(proof) => proof.state == self.recomputed_parity_state(),
            None => false,
        }
    }

    /// Whether the row carries exactly one proof per dimension.
    pub fn has_complete_dimensions(&self) -> bool {
        ProofDimension::ALL
            .iter()
            .all(|d| self.dimension_proof(*d).is_some())
            && self.dimension_proofs.len() == ProofDimension::ALL.len()
    }

    /// Whether the row carries exactly one parity cell per surface.
    pub fn has_complete_surfaces(&self) -> bool {
        ParitySurface::ALL
            .iter()
            .all(|s| self.parity_cell(*s).is_some())
            && self.surface_parity.len() == ParitySurface::ALL.len()
    }

    /// The claim the publication gate permits this row to publish.
    ///
    /// Starts from [`MutationCertificationRow::declared_claim`] and lowers it to
    /// the weakest ceiling implied by the evidence freshness and every dimension
    /// proof — so a stale, degraded, or unproven row can never publish a full
    /// certification.
    pub fn effective_claim(&self) -> MutationClaimClass {
        let mut ceiling = self
            .declared_claim
            .min(self.evidence_freshness.claim_ceiling());
        for proof in &self.dimension_proofs {
            ceiling = ceiling.min(proof.state.claim_ceiling());
        }
        ceiling
    }

    /// The narrowing action the gate must record for this row.
    pub fn required_narrowing(&self) -> ClaimNarrowing {
        ClaimNarrowing::for_published(self.effective_claim())
    }

    /// Whether the row may publish a full certification.
    pub fn is_promotable(&self) -> bool {
        self.effective_claim() == MutationClaimClass::Certified
    }

    /// Whether any surface diverges from the row's mutation truth.
    pub fn has_parity_break(&self) -> bool {
        self.surface_parity
            .iter()
            .any(|c| c.state == ParityState::Divergent)
    }

    /// Whether every dimension on the row is fully proven.
    pub fn is_fully_proven(&self) -> bool {
        self.dimension_proofs
            .iter()
            .all(|p| p.state == DimensionProofState::Proven)
    }

    /// The dimensions whose proof keeps the row below a full certification.
    pub fn limiting_dimensions(&self) -> Vec<ProofDimension> {
        self.dimension_proofs
            .iter()
            .filter(|p| p.state != DimensionProofState::Proven)
            .map(|p| p.dimension)
            .collect()
    }

    /// Whether the stored published claim and narrowing action agree with the
    /// recomputed gate decision and the parity dimension is consistent.
    pub fn gate_consistent(&self) -> bool {
        self.published_claim == self.effective_claim()
            && self.narrowing_action == self.required_narrowing()
            && self.parity_consistent()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageMutationCertificationSummary {
    /// Total rows.
    pub total_rows: usize,
    /// Number of certified ecosystems.
    pub ecosystem_count: usize,
    /// Number of deployment profiles.
    pub deployment_profile_count: usize,
    /// Number of proof dimensions.
    pub dimension_count: usize,
    /// Rows published as certified.
    pub certified_rows: usize,
    /// Rows published as limited.
    pub limited_rows: usize,
    /// Rows published as retest-pending.
    pub retest_pending_rows: usize,
    /// Rows published as unsupported.
    pub unsupported_rows: usize,
    /// Rows that may publish a full certification.
    pub promotable_rows: usize,
    /// Rows the gate narrowed in any way.
    pub narrowed_rows: usize,
    /// Rows the gate withheld from publication.
    pub withheld_rows: usize,
    /// Rows with current evidence freshness.
    pub current_freshness_rows: usize,
    /// Rows whose every dimension is fully proven.
    pub fully_proven_rows: usize,
    /// Rows carrying at least one divergent surface.
    pub rows_with_parity_break: usize,
}

/// A redaction-safe export of one dimension's proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionStateExport {
    /// Dimension token.
    pub dimension: String,
    /// Proof-state token.
    pub state: String,
}

/// A redaction-safe export of one surface's parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceParityExport {
    /// Surface token.
    pub surface: String,
    /// Parity-state token.
    pub state: String,
}

/// A redaction-safe export row projected from a certification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageMutationCertificationExportRow {
    /// Row id.
    pub row_id: String,
    /// Ecosystem token.
    pub ecosystem: String,
    /// Deployment-profile token.
    pub deployment_profile: String,
    /// Declared-claim token.
    pub declared_claim: String,
    /// Published-claim token.
    pub published_claim: String,
    /// Evidence-freshness token.
    pub evidence_freshness: String,
    /// Narrowing-action token.
    pub narrowing_action: String,
    /// Per-dimension proof states.
    pub dimension_states: Vec<DimensionStateExport>,
    /// Per-surface parity states.
    pub surface_parity: Vec<SurfaceParityExport>,
    /// Dimension tokens that keep the row below a full certification.
    pub limiting_dimensions: Vec<String>,
    /// Whether any surface diverges from the row's mutation truth.
    pub parity_break: bool,
    /// Whether the row publishes a full certification.
    pub publication_ready: bool,
    /// Qualification packet ref.
    pub qualification_packet_ref: String,
    /// Proof corpus ref.
    pub corpus_ref: String,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageMutationCertificationExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Ref to the frozen matrix the packet binds to.
    pub matrix_ref: String,
    /// Projected rows.
    pub rows: Vec<PackageMutationCertificationExportRow>,
    /// Whether every row's published claim and narrowing agree with the gate.
    pub all_rows_gate_consistent: bool,
    /// Rows that may publish a full certification.
    pub promotable_count: usize,
    /// Rows the gate narrowed in any way.
    pub narrowed_count: usize,
    /// Rows the gate withheld from publication.
    pub withheld_count: usize,
    /// Rows carrying at least one divergent surface.
    pub parity_break_count: usize,
}

/// The typed package-mutation certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageMutationCertification {
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
    /// Ref to the frozen matrix this packet binds to; must equal
    /// [`M5_PACKAGE_STATE_MATRIX_PATH`].
    pub matrix_ref: String,
    /// Certified ecosystems; one row per ecosystem and deployment profile.
    pub certified_ecosystems: Vec<CertifiedEcosystem>,
    /// Closed deployment-profile vocabulary.
    pub deployment_profiles: Vec<DeploymentProfile>,
    /// Closed proof-dimension vocabulary.
    pub proof_dimensions: Vec<ProofDimension>,
    /// Closed dimension-proof-state vocabulary.
    pub dimension_proof_states: Vec<DimensionProofState>,
    /// Closed claim-class vocabulary.
    pub claim_classes: Vec<MutationClaimClass>,
    /// Closed evidence-freshness vocabulary.
    pub evidence_freshness_classes: Vec<EvidenceFreshness>,
    /// Closed narrowing-action vocabulary.
    pub narrowing_actions: Vec<ClaimNarrowing>,
    /// Closed parity-surface vocabulary.
    pub parity_surfaces: Vec<ParitySurface>,
    /// Closed parity-state vocabulary.
    pub parity_states: Vec<ParityState>,
    /// Certification rows, one per (ecosystem, deployment-profile) cell.
    #[serde(default)]
    pub rows: Vec<MutationCertificationRow>,
    /// Summary counts.
    pub summary: PackageMutationCertificationSummary,
}

impl PackageMutationCertification {
    /// Returns the row for an (ecosystem, deployment-profile) cell.
    pub fn row(
        &self,
        ecosystem: CertifiedEcosystem,
        profile: DeploymentProfile,
    ) -> Option<&MutationCertificationRow> {
        self.rows
            .iter()
            .find(|r| r.ecosystem == ecosystem && r.deployment_profile == profile)
    }

    /// Rows that may publish a full certification.
    pub fn promotable_rows(&self) -> impl Iterator<Item = &MutationCertificationRow> {
        self.rows.iter().filter(|r| r.is_promotable())
    }

    /// Rows the gate narrowed in any way.
    pub fn narrowed_rows(&self) -> impl Iterator<Item = &MutationCertificationRow> {
        self.rows
            .iter()
            .filter(|r| r.required_narrowing() != ClaimNarrowing::None)
    }

    /// Rows the gate withheld from publication.
    pub fn withheld_rows(&self) -> impl Iterator<Item = &MutationCertificationRow> {
        self.rows
            .iter()
            .filter(|r| r.required_narrowing() == ClaimNarrowing::WithholdAsUnsupported)
    }

    /// Rows carrying at least one divergent surface.
    pub fn parity_break_rows(&self) -> impl Iterator<Item = &MutationCertificationRow> {
        self.rows.iter().filter(|r| r.has_parity_break())
    }

    /// Whether every row's stored published claim and narrowing action agree with
    /// the recomputed gate decision.
    pub fn all_rows_gate_consistent(&self) -> bool {
        self.rows.iter().all(|r| r.gate_consistent())
    }

    /// Whether every row's cross-surface parity dimension matches the state
    /// recomputed from its surface cells.
    pub fn all_rows_parity_consistent(&self) -> bool {
        self.rows.iter().all(|r| r.parity_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> PackageMutationCertificationSummary {
        let count_published = |claim: MutationClaimClass| {
            self.rows
                .iter()
                .filter(|r| r.published_claim == claim)
                .count()
        };
        PackageMutationCertificationSummary {
            total_rows: self.rows.len(),
            ecosystem_count: self.certified_ecosystems.len(),
            deployment_profile_count: self.deployment_profiles.len(),
            dimension_count: self.proof_dimensions.len(),
            certified_rows: count_published(MutationClaimClass::Certified),
            limited_rows: count_published(MutationClaimClass::Limited),
            retest_pending_rows: count_published(MutationClaimClass::RetestPending),
            unsupported_rows: count_published(MutationClaimClass::Unsupported),
            promotable_rows: self.promotable_rows().count(),
            narrowed_rows: self.narrowed_rows().count(),
            withheld_rows: self.withheld_rows().count(),
            current_freshness_rows: self
                .rows
                .iter()
                .filter(|r| r.evidence_freshness.is_current())
                .count(),
            fully_proven_rows: self.rows.iter().filter(|r| r.is_fully_proven()).count(),
            rows_with_parity_break: self.parity_break_rows().count(),
        }
    }

    /// Produces an export projection that downstream surfaces — Help/About,
    /// docs/help, support exports, and release/public-truth packets — render
    /// instead of restating certification status by hand.
    pub fn export_projection(&self) -> PackageMutationCertificationExportProjection {
        let rows = self
            .rows
            .iter()
            .map(|row| PackageMutationCertificationExportRow {
                row_id: row.row_id.clone(),
                ecosystem: row.ecosystem.as_str().to_owned(),
                deployment_profile: row.deployment_profile.as_str().to_owned(),
                declared_claim: row.declared_claim.as_str().to_owned(),
                published_claim: row.published_claim.as_str().to_owned(),
                evidence_freshness: row.evidence_freshness.as_str().to_owned(),
                narrowing_action: row.narrowing_action.as_str().to_owned(),
                dimension_states: row
                    .dimension_proofs
                    .iter()
                    .map(|p| DimensionStateExport {
                        dimension: p.dimension.as_str().to_owned(),
                        state: p.state.as_str().to_owned(),
                    })
                    .collect(),
                surface_parity: row
                    .surface_parity
                    .iter()
                    .map(|c| SurfaceParityExport {
                        surface: c.surface.as_str().to_owned(),
                        state: c.state.as_str().to_owned(),
                    })
                    .collect(),
                limiting_dimensions: row
                    .limiting_dimensions()
                    .iter()
                    .map(|d| d.as_str().to_owned())
                    .collect(),
                parity_break: row.has_parity_break(),
                publication_ready: row.is_promotable(),
                qualification_packet_ref: row.qualification_packet_ref.clone(),
                corpus_ref: row.corpus_ref.clone(),
                summary: format!(
                    "{} / {}: declared {}, published {} ({}), freshness {}",
                    row.ecosystem.as_str(),
                    row.deployment_profile.as_str(),
                    row.declared_claim.as_str(),
                    row.published_claim.as_str(),
                    row.narrowing_action.as_str(),
                    row.evidence_freshness.as_str()
                ),
            })
            .collect();
        PackageMutationCertificationExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            matrix_ref: self.matrix_ref.clone(),
            rows,
            all_rows_gate_consistent: self.all_rows_gate_consistent(),
            promotable_count: self.promotable_rows().count(),
            narrowed_count: self.narrowed_rows().count(),
            withheld_count: self.withheld_rows().count(),
            parity_break_count: self.parity_break_rows().count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<PackageMutationCertificationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<CertifiedEcosystem> =
            self.certified_ecosystems.iter().copied().collect();

        let mut seen_rows = BTreeSet::new();
        let mut seen_cells = BTreeSet::new();
        for row in &self.rows {
            if !seen_rows.insert(row.row_id.clone()) {
                violations.push(PackageMutationCertificationViolation::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            if !seen_cells.insert((row.ecosystem, row.deployment_profile)) {
                violations.push(PackageMutationCertificationViolation::DuplicateMatrixCell {
                    ecosystem: row.ecosystem.as_str(),
                    profile: row.deployment_profile.as_str(),
                });
            }
            if !claimed.contains(&row.ecosystem) {
                violations.push(
                    PackageMutationCertificationViolation::UnclaimedEcosystemRow {
                        row_id: row.row_id.clone(),
                        ecosystem: row.ecosystem.as_str(),
                    },
                );
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed (ecosystem, profile) cell must carry its own row, so a
        // certified direct-registry row never extends to its mirror or offline
        // siblings.
        for &ecosystem in &self.certified_ecosystems {
            for &profile in &self.deployment_profiles {
                if !seen_cells.contains(&(ecosystem, profile)) {
                    violations.push(PackageMutationCertificationViolation::MissingMatrixCell {
                        ecosystem: ecosystem.as_str(),
                        profile: profile.as_str(),
                    });
                }
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(PackageMutationCertificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<PackageMutationCertificationViolation>) {
        if self.schema_version != PACKAGE_MUTATION_CERTIFICATION_SCHEMA_VERSION {
            violations.push(
                PackageMutationCertificationViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != PACKAGE_MUTATION_CERTIFICATION_RECORD_KIND {
            violations.push(
                PackageMutationCertificationViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        if self.matrix_ref != M5_PACKAGE_STATE_MATRIX_PATH {
            violations.push(PackageMutationCertificationViolation::MatrixRefMismatch {
                actual: self.matrix_ref.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(PackageMutationCertificationViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "certified_ecosystems",
                self.certified_ecosystems == CertifiedEcosystem::ALL.to_vec(),
            ),
            (
                "deployment_profiles",
                self.deployment_profiles == DeploymentProfile::ALL.to_vec(),
            ),
            (
                "proof_dimensions",
                self.proof_dimensions == ProofDimension::ALL.to_vec(),
            ),
            (
                "dimension_proof_states",
                self.dimension_proof_states == DimensionProofState::ALL.to_vec(),
            ),
            (
                "claim_classes",
                self.claim_classes == MutationClaimClass::ALL.to_vec(),
            ),
            (
                "evidence_freshness_classes",
                self.evidence_freshness_classes == EvidenceFreshness::ALL.to_vec(),
            ),
            (
                "narrowing_actions",
                self.narrowing_actions == ClaimNarrowing::ALL.to_vec(),
            ),
            (
                "parity_surfaces",
                self.parity_surfaces == ParitySurface::ALL.to_vec(),
            ),
            (
                "parity_states",
                self.parity_states == ParityState::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(
                    PackageMutationCertificationViolation::ClosedVocabularyMismatch { field },
                );
            }
        }
    }

    fn validate_row(
        &self,
        row: &MutationCertificationRow,
        violations: &mut Vec<PackageMutationCertificationViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("qualification_packet_ref", &row.qualification_packet_ref),
            ("corpus_ref", &row.corpus_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(PackageMutationCertificationViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }

        // Exactly one proof per dimension, no duplicates, each with evidence.
        let mut seen_dims = BTreeSet::new();
        for proof in &row.dimension_proofs {
            if !seen_dims.insert(proof.dimension) {
                violations.push(
                    PackageMutationCertificationViolation::DuplicateDimensionProof {
                        row_id: row.row_id.clone(),
                        dimension: proof.dimension.as_str(),
                    },
                );
            }
            if proof.evidence_ref.trim().is_empty() {
                violations.push(PackageMutationCertificationViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: "dimension_proof.evidence_ref",
                });
            }
        }
        if !row.has_complete_dimensions() {
            violations.push(
                PackageMutationCertificationViolation::IncompleteDimensionProofs {
                    row_id: row.row_id.clone(),
                },
            );
        }

        // Exactly one parity cell per surface, no duplicates.
        let mut seen_surfaces = BTreeSet::new();
        for cell in &row.surface_parity {
            if !seen_surfaces.insert(cell.surface) {
                violations.push(
                    PackageMutationCertificationViolation::DuplicateSurfaceParity {
                        row_id: row.row_id.clone(),
                        surface: cell.surface.as_str(),
                    },
                );
            }
        }
        if !row.has_complete_surfaces() {
            violations.push(
                PackageMutationCertificationViolation::IncompleteSurfaceParity {
                    row_id: row.row_id.clone(),
                },
            );
        }

        // The recorded cross-surface parity dimension must equal the state
        // recomputed from the surface cells, so a row cannot read consistent
        // while a surface diverges.
        if row.has_complete_dimensions() && row.has_complete_surfaces() && !row.parity_consistent()
        {
            let recorded = row
                .dimension_proof(ProofDimension::CrossSurfaceParity)
                .map(|p| p.state.as_str())
                .unwrap_or("<missing>");
            violations.push(PackageMutationCertificationViolation::ParityStateMismatch {
                row_id: row.row_id.clone(),
                recorded,
                computed: row.recomputed_parity_state().as_str(),
            });
        }

        // The published claim must equal the gate's recomputed decision.
        let effective = row.effective_claim();
        if row.published_claim != effective {
            violations.push(
                PackageMutationCertificationViolation::OverstatedPublishedClaim {
                    row_id: row.row_id.clone(),
                    published: row.published_claim.as_str(),
                    computed: effective.as_str(),
                },
            );
        }

        // The recorded narrowing action must match the published claim.
        let required = row.required_narrowing();
        if row.narrowing_action != required {
            violations.push(
                PackageMutationCertificationViolation::NarrowingActionMismatch {
                    row_id: row.row_id.clone(),
                    declared: row.narrowing_action.as_str(),
                    required: required.as_str(),
                },
            );
        }

        // A promotable row must be genuinely clean: current freshness, every
        // dimension proven, and every surface consistent.
        if row.is_promotable()
            && (!row.evidence_freshness.is_current()
                || !row.is_fully_proven()
                || row
                    .surface_parity
                    .iter()
                    .any(|c| c.state != ParityState::Consistent))
        {
            violations.push(PackageMutationCertificationViolation::PromotedRowNotClean {
                row_id: row.row_id.clone(),
            });
        }
    }
}

/// A validation violation for the package-mutation certification packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageMutationCertificationViolation {
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
    /// The packet does not bind to the canonical frozen matrix path.
    MatrixRefMismatch {
        /// Matrix ref found in the packet.
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
    /// A row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// An (ecosystem, profile) cell carries more than one row.
    DuplicateMatrixCell {
        /// Ecosystem token.
        ecosystem: &'static str,
        /// Deployment-profile token.
        profile: &'static str,
    },
    /// A claimed (ecosystem, profile) cell has no row.
    MissingMatrixCell {
        /// Ecosystem token.
        ecosystem: &'static str,
        /// Deployment-profile token.
        profile: &'static str,
    },
    /// A row covers an ecosystem the matrix does not claim.
    UnclaimedEcosystemRow {
        /// Row id.
        row_id: String,
        /// Ecosystem token.
        ecosystem: &'static str,
    },
    /// A row does not carry exactly one proof per dimension.
    IncompleteDimensionProofs {
        /// Row id.
        row_id: String,
    },
    /// A row lists a dimension proof more than once.
    DuplicateDimensionProof {
        /// Row id.
        row_id: String,
        /// Dimension token.
        dimension: &'static str,
    },
    /// A row does not carry exactly one parity cell per surface.
    IncompleteSurfaceParity {
        /// Row id.
        row_id: String,
    },
    /// A row lists a surface parity cell more than once.
    DuplicateSurfaceParity {
        /// Row id.
        row_id: String,
        /// Surface token.
        surface: &'static str,
    },
    /// The recorded parity dimension disagrees with the recomputed parity state.
    ParityStateMismatch {
        /// Row id.
        row_id: String,
        /// Recorded parity-dimension token.
        recorded: &'static str,
        /// Recomputed parity-state token.
        computed: &'static str,
    },
    /// A row publishes a claim beyond what its evidence supports.
    OverstatedPublishedClaim {
        /// Row id.
        row_id: String,
        /// Published-claim token.
        published: &'static str,
        /// Computed effective-claim token.
        computed: &'static str,
    },
    /// A row's narrowing action disagrees with its published claim.
    NarrowingActionMismatch {
        /// Row id.
        row_id: String,
        /// Declared narrowing token.
        declared: &'static str,
        /// Required narrowing token.
        required: &'static str,
    },
    /// A promotable row still carries a degraded dimension, divergent/absent
    /// surface, or non-current freshness.
    PromotedRowNotClean {
        /// Row id.
        row_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for PackageMutationCertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::MatrixRefMismatch { actual } => {
                write!(
                    f,
                    "packet matrix_ref {actual} is not the frozen matrix path"
                )
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id } => {
                write!(f, "duplicate row id {row_id}")
            }
            Self::DuplicateMatrixCell { ecosystem, profile } => {
                write!(f, "duplicate matrix cell for {ecosystem}/{profile}")
            }
            Self::MissingMatrixCell { ecosystem, profile } => {
                write!(f, "missing matrix cell for {ecosystem}/{profile}")
            }
            Self::UnclaimedEcosystemRow { row_id, ecosystem } => {
                write!(f, "row {row_id} covers unclaimed ecosystem {ecosystem}")
            }
            Self::IncompleteDimensionProofs { row_id } => {
                write!(f, "row {row_id} does not carry one proof per dimension")
            }
            Self::DuplicateDimensionProof { row_id, dimension } => {
                write!(f, "row {row_id} repeats dimension proof {dimension}")
            }
            Self::IncompleteSurfaceParity { row_id } => {
                write!(f, "row {row_id} does not carry one parity cell per surface")
            }
            Self::DuplicateSurfaceParity { row_id, surface } => {
                write!(f, "row {row_id} repeats surface parity {surface}")
            }
            Self::ParityStateMismatch {
                row_id,
                recorded,
                computed,
            } => {
                write!(
                    f,
                    "row {row_id} records parity {recorded} but the surfaces compute {computed}"
                )
            }
            Self::OverstatedPublishedClaim {
                row_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "row {row_id} publishes claim {published} but the gate computes {computed}"
                )
            }
            Self::NarrowingActionMismatch {
                row_id,
                declared,
                required,
            } => {
                write!(
                    f,
                    "row {row_id} records narrowing {declared} but the gate requires {required}"
                )
            }
            Self::PromotedRowNotClean { row_id } => {
                write!(
                    f,
                    "row {row_id} is promotable but carries a non-proven dimension, non-consistent surface, or non-current freshness"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for PackageMutationCertificationViolation {}

/// Loads the embedded package-mutation certification packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`PackageMutationCertification`].
pub fn current_package_mutation_certification(
) -> Result<PackageMutationCertification, serde_json::Error> {
    serde_json::from_str(PACKAGE_MUTATION_CERTIFICATION_JSON)
}

#[cfg(test)]
mod tests;
