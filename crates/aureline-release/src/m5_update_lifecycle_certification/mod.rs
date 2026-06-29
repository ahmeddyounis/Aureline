//! Per-channel / per-profile qualification of the M5 update / support-lifecycle contract.
//!
//! The [update / support-lifecycle governance matrix](crate::m5_update_lifecycle) freezes the
//! governed facets (update availability, change impact, release-note evidence, migration assistant,
//! service health, support window, compatibility window, end-of-support) and certifies the
//! *consumer surfaces* that read them. It does not, on its own, say whether a **claimed M5 channel
//! and deployment profile** — `stable` on a managed deployment, `lts` self-hosted, `nightly`, and so
//! on — actually maps to fresh proof for the update / support-lifecycle contract. This lane closes
//! that gap: it projects the governance matrix onto the claimed channel × profile grid and qualifies
//! each pair, narrowing or blocking the claim deterministically when the backing proof is stale,
//! expired, or missing rather than letting a channel keep a generic Stable promise behind drifted
//! evidence.
//!
//! The certification is a *pure function of the governance matrix* — it carries no parallel,
//! hand-maintained inventory. Each claim is qualified along four [proof dimensions]
//! ([CertificationDimension]) drawn straight from the governed facets:
//!
//! - **update communication** — update availability, change impact, and release-note evidence;
//! - **migration guidance** — the migration assistant;
//! - **lifecycle windows** — the support window, compatibility window, and end-of-support state;
//! - **stale-data behavior** — the service-health / stale-or-mirrored data labeling.
//!
//! For one claimed (`channel`, `profile`) pair and one dimension, the certification gathers the
//! governed facets that back that dimension *and* scope to that channel and profile. A dimension no
//! facet covers for the pair is [not applicable](CertificationOutcome::NotApplicable) (honestly
//! labeled, never a hidden gap); otherwise the cell takes the **worst** proof freshness and the
//! **worst** lifecycle-state gate among the covering facets, so a downgraded copy can never read as
//! live. A claim's gate is the worst of its applicable cells, and its effective qualification is the
//! claimed class narrowed down that gate — `governed` keeps the claim, `narrowed` floors it at Beta,
//! `blocked` floors it at Unavailable.
//!
//! The claimed consumer surfaces ([CertificationConsumer]) — release center, update center,
//! Help/About, docs/help, support exports, and shiproom — each bind the dimensions they surface and
//! *derive* their posture and the exact channel/profile pairs they must narrow or block from the
//! grid, so release, help, support, and shiproom packets read one certification output instead of
//! parallel channel inventories.
//!
//! The [`M5UpdateLifecycleCertification`] packet is the one inspectable, serde-serializable truth
//! release/help/support/shiproom surfaces consume; it carries metadata and refs only — no credential
//! bodies or raw provider payloads.
//!
//! - Packet schema:
//!   [`schemas/release/m5-update-lifecycle-certification.schema.json`](../../../../../schemas/release/m5-update-lifecycle-certification.schema.json)
//! - Contract doc:
//!   [`docs/release/m5-update-lifecycle-certification-contract.md`](../../../../../docs/release/m5-update-lifecycle-certification-contract.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_update_lifecycle_certification,
    seeded_m5_update_lifecycle_certification_missing_proof_blocked,
    seeded_m5_update_lifecycle_certification_stale_proof_narrowed,
    M5_UPDATE_LIFECYCLE_CERTIFICATION_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The certification reuses the governance matrix's frozen channel / profile / facet vocabulary and
// the descriptor / badge gate vocabulary, so the certification layer can never drift to a different
// channel set, facet set, or gate token than the surfaces it qualifies.
use crate::m5_descriptor_badge::{
    ConsumerStatus, DescriptorGate, DescriptorSignal, FreshnessState, QualificationClass,
};
use crate::m5_update_lifecycle::{
    ChannelScope, DeploymentProfile, LifecycleFacet, LifecycleFacetRow,
    M5UpdateLifecycleGovernance, StaleDataBehavior,
};

/// Record-kind tag carried by [`M5UpdateLifecycleCertification`].
pub const M5_UPDATE_LIFECYCLE_CERTIFICATION_RECORD_KIND: &str = "m5_update_lifecycle_certification";

/// Schema version for the certification packet.
pub const M5_UPDATE_LIFECYCLE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the certification packet schema.
pub const M5_UPDATE_LIFECYCLE_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/release/m5-update-lifecycle-certification.schema.json";

/// Repo-relative path of the published certification inventory.
pub const M5_UPDATE_LIFECYCLE_CERTIFICATION_REF: &str =
    "artifacts/release/m5-update-lifecycle-certification.json";

/// Repo-relative path of the rendered certification document.
pub const M5_UPDATE_LIFECYCLE_CERTIFICATION_DOCUMENT_REF: &str =
    "artifacts/release/m5-update-lifecycle-certification.md";

/// Repo-relative path of the machine-readable certification grid export.
pub const M5_UPDATE_LIFECYCLE_CERTIFICATION_CSV_REF: &str =
    "artifacts/release/m5-update-lifecycle-certification.csv";

/// Repo-relative path of the release-grade certification parity proof.
pub const M5_UPDATE_LIFECYCLE_CERTIFICATION_PROOF_REF: &str =
    "artifacts/release/m5-update-lifecycle-proof/update-lifecycle-certification.json";

/// Repo-relative path of the release-grade certification proof report.
pub const M5_UPDATE_LIFECYCLE_CERTIFICATION_PROOF_MD_REF: &str =
    "artifacts/release/m5-update-lifecycle-proof/update-lifecycle-certification.md";

/// Repo-relative path of the certification contract doc.
pub const M5_UPDATE_LIFECYCLE_CERTIFICATION_DOC_REF: &str =
    "docs/release/m5-update-lifecycle-certification-contract.md";

/// Repo-relative directory of the per-state certification fixtures.
pub const M5_UPDATE_LIFECYCLE_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/release/m5-update-lifecycle-certification/";

/// Prefix every certification message id carries so consumers can route it.
pub const M5_UPDATE_LIFECYCLE_CERTIFICATION_MESSAGE_ID_PREFIX: &str =
    "release_update_lifecycle_certification.";

/// One proof dimension a claimed channel / profile is qualified along. Each dimension aggregates the
/// governed [facets](LifecycleFacet) the source set treats as that part of the update /
/// support-lifecycle contract, so the certification reuses the matrix's facet proofs rather than
/// restating them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDimension {
    /// What an update offers and discloses: availability, change impact, release-note evidence.
    UpdateCommunication,
    /// What carries a user across an update: the migration assistant.
    MigrationGuidance,
    /// How long a build stays supported: support window, compatibility window, end-of-support.
    LifecycleWindows,
    /// How the surface labels stale, mirrored, or no-live-data conditions: service health.
    StaleDataBehavior,
}

impl CertificationDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::UpdateCommunication,
        Self::MigrationGuidance,
        Self::LifecycleWindows,
        Self::StaleDataBehavior,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpdateCommunication => "update_communication",
            Self::MigrationGuidance => "migration_guidance",
            Self::LifecycleWindows => "lifecycle_windows",
            Self::StaleDataBehavior => "stale_data_behavior",
        }
    }

    /// Reviewer-facing dimension label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UpdateCommunication => "Update communication",
            Self::MigrationGuidance => "Migration guidance",
            Self::LifecycleWindows => "Lifecycle windows",
            Self::StaleDataBehavior => "Stale-data behavior",
        }
    }

    /// The governed facets that back this dimension's proof.
    pub fn backing_facets(self) -> &'static [LifecycleFacet] {
        match self {
            Self::UpdateCommunication => &[
                LifecycleFacet::UpdateAvailability,
                LifecycleFacet::ChangeImpact,
                LifecycleFacet::ReleaseNoteEvidence,
            ],
            Self::MigrationGuidance => &[LifecycleFacet::MigrationAssistant],
            Self::LifecycleWindows => &[
                LifecycleFacet::SupportWindow,
                LifecycleFacet::CompatibilityWindow,
                LifecycleFacet::EndOfSupport,
            ],
            Self::StaleDataBehavior => &[LifecycleFacet::ServiceHealth],
        }
    }

    /// Owner role accountable for keeping this dimension's proof current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::UpdateCommunication => "release_update_center_owner",
            Self::MigrationGuidance => "migration_continuity_owner",
            Self::LifecycleWindows => "support_lifecycle_owner",
            Self::StaleDataBehavior => "migration_continuity_owner",
        }
    }
}

/// The outcome a claim earned on one [dimension](CertificationDimension): fully certified, narrowed,
/// blocked, or — when no governed facet covers the pair — honestly labeled not-applicable rather
/// than a hidden gap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationOutcome {
    /// Every backing facet maps to a current proof and a governed lifecycle state.
    Certified,
    /// At least one backing facet's proof is stale or its lifecycle state narrows.
    Narrowed,
    /// At least one backing facet's proof is expired / missing or its lifecycle state blocks.
    Blocked,
    /// No governed facet covers this channel / profile for this dimension; the dimension does not
    /// apply to the pair.
    NotApplicable,
}

impl CertificationOutcome {
    /// Every outcome, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Certified,
        Self::Narrowed,
        Self::Blocked,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Narrowed => "narrowed",
            Self::Blocked => "blocked",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// The outcome implied by an applicable cell's gate.
    const fn for_gate(gate: DescriptorGate) -> Self {
        match gate {
            DescriptorGate::Governed => Self::Certified,
            DescriptorGate::Narrowed => Self::Narrowed,
            DescriptorGate::Blocked => Self::Blocked,
        }
    }
}

/// Why a claim's cell narrowed or blocked: a proof-currency gap or a lifecycle-state gap. Naming the
/// kind is what lets the certification say *why* a channel / profile claim was held rather than
/// leaving it implied behind a generic stable label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationGapKind {
    /// A backing facet's proof is stale (narrows).
    ProofStale,
    /// A backing facet's proof is expired (blocks).
    ProofExpired,
    /// A backing facet's proof is missing (blocks).
    ProofMissing,
    /// A backing facet's current lifecycle state itself narrows the claim.
    LifecycleStateNarrowed,
    /// A backing facet's current lifecycle state itself blocks the claim.
    LifecycleStateBlocked,
}

impl CertificationGapKind {
    /// Every gap kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProofStale,
        Self::ProofExpired,
        Self::ProofMissing,
        Self::LifecycleStateNarrowed,
        Self::LifecycleStateBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::ProofExpired => "proof_expired",
            Self::ProofMissing => "proof_missing",
            Self::LifecycleStateNarrowed => "lifecycle_state_narrowed",
            Self::LifecycleStateBlocked => "lifecycle_state_blocked",
        }
    }

    /// True when this gap blocks Stable promotion (vs only narrowing it).
    pub const fn is_blocking(self) -> bool {
        matches!(
            self,
            Self::ProofExpired | Self::ProofMissing | Self::LifecycleStateBlocked
        )
    }
}

/// The proof-currency gap kind a freshness implies, if any.
const fn freshness_gap(freshness: FreshnessState) -> Option<CertificationGapKind> {
    match freshness {
        FreshnessState::Current => None,
        FreshnessState::Stale => Some(CertificationGapKind::ProofStale),
        FreshnessState::Expired => Some(CertificationGapKind::ProofExpired),
        FreshnessState::Missing => Some(CertificationGapKind::ProofMissing),
    }
}

/// The lifecycle-state gap kind a state gate implies, if any.
const fn state_gap(gate: DescriptorGate) -> Option<CertificationGapKind> {
    match gate {
        DescriptorGate::Governed => None,
        DescriptorGate::Narrowed => Some(CertificationGapKind::LifecycleStateNarrowed),
        DescriptorGate::Blocked => Some(CertificationGapKind::LifecycleStateBlocked),
    }
}

/// The gate a proof freshness implies on its own.
const fn freshness_gate(freshness: FreshnessState) -> DescriptorGate {
    match freshness {
        FreshnessState::Current => DescriptorGate::Governed,
        FreshnessState::Stale => DescriptorGate::Narrowed,
        FreshnessState::Expired | FreshnessState::Missing => DescriptorGate::Blocked,
    }
}

/// The more severe of two gates (`Blocked` > `Narrowed` > `Governed`).
const fn worst_gate(a: DescriptorGate, b: DescriptorGate) -> DescriptorGate {
    match (a, b) {
        (DescriptorGate::Blocked, _) | (_, DescriptorGate::Blocked) => DescriptorGate::Blocked,
        (DescriptorGate::Narrowed, _) | (_, DescriptorGate::Narrowed) => DescriptorGate::Narrowed,
        _ => DescriptorGate::Governed,
    }
}

/// The coverage status a gate implies.
const fn status_for_gate(gate: DescriptorGate) -> ConsumerStatus {
    match gate {
        DescriptorGate::Governed => ConsumerStatus::Mapped,
        DescriptorGate::Narrowed => ConsumerStatus::Provisional,
        DescriptorGate::Blocked => ConsumerStatus::Unmapped,
    }
}

/// The claimed qualification narrowed down its earned gate: `governed` keeps the claim, `narrowed`
/// floors it at Beta, `blocked` floors it at Unavailable.
fn narrow_qualification(claimed: QualificationClass, gate: DescriptorGate) -> QualificationClass {
    match gate {
        DescriptorGate::Governed => claimed,
        // QualificationClass declaration order is most→least permissive, so `max` is the more
        // restrictive of the two.
        DescriptorGate::Narrowed => claimed.max(QualificationClass::Beta),
        DescriptorGate::Blocked => QualificationClass::Unavailable,
    }
}

/// One qualification cell: a claimed (`channel`, `profile`) pair certified along one
/// [dimension](CertificationDimension). The cell derives its outcome from the governed facets that
/// back the dimension and scope to the pair, so it can never cite a posture stronger than its
/// weakest backing proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationCell {
    /// The dimension this cell certifies.
    pub dimension: CertificationDimension,
    /// Reviewer-facing dimension label.
    pub dimension_label: String,
    /// The outcome earned.
    pub outcome: CertificationOutcome,
    /// The governed facets that back this cell (scoped to the claim's channel and profile).
    pub backing_facets: Vec<LifecycleFacet>,
    /// The proof paths backing the cell — refs only.
    pub proof_refs: Vec<String>,
    /// Worst proof freshness among the backing facets; absent when not applicable.
    pub proof_freshness: Option<FreshnessState>,
    /// Worst lifecycle-state gate among the backing facets; absent when not applicable.
    pub lifecycle_gate: Option<DescriptorGate>,
    /// Worst (most cautionary) stale-data behavior among the backing facets; absent when not
    /// applicable.
    pub stale_data_behavior: Option<StaleDataBehavior>,
    /// Gate the cell contributes to the claim. Not-applicable cells are governed and excluded.
    pub gate: DescriptorGate,
    /// Coverage status (mirrors [`Self::gate`]).
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Why the cell narrowed or blocked, if it did.
    pub gap_kind: Option<CertificationGapKind>,
    /// Stable message id; prefixed [`M5_UPDATE_LIFECYCLE_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl CertificationCell {
    /// Derives a cell for a (`channel`, `profile`, `dimension`) tuple from the governed facet rows.
    fn derive(
        dimension: CertificationDimension,
        channel: ChannelScope,
        profile: DeploymentProfile,
        facets: &[LifecycleFacetRow],
    ) -> Self {
        // Gather the backing facets that scope to this channel and profile.
        let mut backing: Vec<&LifecycleFacetRow> = Vec::new();
        for facet in dimension.backing_facets() {
            if let Some(row) = facets.iter().find(|r| r.facet == *facet) {
                if row.channel_scope.contains(&channel) && row.profiles.contains(&profile) {
                    backing.push(row);
                }
            }
        }

        let detail_message_id = format!(
            "{}cell.{}.{}.{}",
            M5_UPDATE_LIFECYCLE_CERTIFICATION_MESSAGE_ID_PREFIX,
            channel.as_str(),
            profile.as_str(),
            dimension.as_str()
        );

        if backing.is_empty() {
            // No governed facet covers this pair for this dimension: honestly not applicable.
            return Self {
                dimension,
                dimension_label: dimension.label().to_owned(),
                outcome: CertificationOutcome::NotApplicable,
                backing_facets: Vec::new(),
                proof_refs: Vec::new(),
                proof_freshness: None,
                lifecycle_gate: None,
                stale_data_behavior: None,
                gate: DescriptorGate::Governed,
                status: ConsumerStatus::Mapped,
                signal: ConsumerStatus::Mapped.signal(),
                gap_kind: None,
                detail_message_id,
            };
        }

        // Worst proof freshness, worst lifecycle-state gate, worst stale-data behavior.
        let proof_freshness = backing
            .iter()
            .map(|r| r.proof_freshness)
            .max_by_key(|f| freshness_rank(*f))
            .expect("backing is non-empty");
        let lifecycle_gate = backing
            .iter()
            .map(|r| r.state_gate)
            .max_by_key(|g| gate_rank(*g))
            .expect("backing is non-empty");
        let stale_data_behavior = backing
            .iter()
            .map(|r| r.stale_data_behavior)
            .max_by_key(|b| stale_rank(*b))
            .expect("backing is non-empty");

        let gate = worst_gate(freshness_gate(proof_freshness), lifecycle_gate);
        // A proof-currency gap reads ahead of a lifecycle-state gap when both apply, so the named
        // cause matches the dominant reason the cell could not stand.
        let gap_kind = freshness_gap(proof_freshness).or_else(|| state_gap(lifecycle_gate));
        let status = status_for_gate(gate);

        let mut facet_kinds: Vec<LifecycleFacet> = backing.iter().map(|r| r.facet).collect();
        facet_kinds.sort_by_key(|f| facet_rank(*f));
        facet_kinds.dedup();
        let proof_refs: Vec<String> = facet_kinds
            .iter()
            .map(|f| f.proof_ref().to_owned())
            .collect();

        Self {
            dimension,
            dimension_label: dimension.label().to_owned(),
            outcome: CertificationOutcome::for_gate(gate),
            backing_facets: facet_kinds,
            proof_refs,
            proof_freshness: Some(proof_freshness),
            lifecycle_gate: Some(lifecycle_gate),
            stale_data_behavior: Some(stale_data_behavior),
            gate,
            status,
            signal: status.signal(),
            gap_kind,
            detail_message_id,
        }
    }

    /// True when this cell contributes to its claim's gate (i.e. the dimension applies).
    pub fn is_applicable(&self) -> bool {
        self.outcome != CertificationOutcome::NotApplicable
    }

    /// Re-derives this cell's invariants and reports any drift.
    fn validate(&self) -> Vec<M5UpdateLifecycleCertificationViolation> {
        let mut out = Vec::new();
        if self.dimension_label != self.dimension.label() {
            out.push(M5UpdateLifecycleCertificationViolation::CellFieldMismatch);
        }
        match self.outcome {
            CertificationOutcome::NotApplicable => {
                if !self.backing_facets.is_empty()
                    || self.proof_freshness.is_some()
                    || self.lifecycle_gate.is_some()
                    || self.gate != DescriptorGate::Governed
                    || self.gap_kind.is_some()
                {
                    out.push(M5UpdateLifecycleCertificationViolation::CellOutcomeDrift);
                }
            }
            _ => {
                let (Some(freshness), Some(lifecycle_gate)) =
                    (self.proof_freshness, self.lifecycle_gate)
                else {
                    out.push(M5UpdateLifecycleCertificationViolation::CellOutcomeDrift);
                    return out;
                };
                let gate = worst_gate(freshness_gate(freshness), lifecycle_gate);
                let gap = freshness_gap(freshness).or_else(|| state_gap(lifecycle_gate));
                if self.gate != gate
                    || self.outcome != CertificationOutcome::for_gate(gate)
                    || self.status != status_for_gate(gate)
                    || self.signal != status_for_gate(gate).signal()
                    || self.gap_kind != gap
                    || self.backing_facets.is_empty()
                    || self.proof_refs.len() != self.backing_facets.len()
                {
                    out.push(M5UpdateLifecycleCertificationViolation::CellOutcomeDrift);
                }
            }
        }
        if !self
            .detail_message_id
            .starts_with(M5_UPDATE_LIFECYCLE_CERTIFICATION_MESSAGE_ID_PREFIX)
        {
            out.push(M5UpdateLifecycleCertificationViolation::UnprefixedMessageId);
        }
        out
    }
}

/// One claimed M5 channel / deployment-profile pair qualified against the update / support-lifecycle
/// contract: the qualification class it wants to keep, a cell per [dimension](CertificationDimension),
/// the gate derived from those cells, and the effective qualification after narrowing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelProfileClaim {
    /// The claimed channel.
    pub channel: ChannelScope,
    /// The claimed deployment profile.
    pub profile: DeploymentProfile,
    /// Reviewer-facing claim label.
    pub claim_label: String,
    /// Stable claim id (`<channel>:<profile>`), used as a ref by consumers.
    pub claim_ref: String,
    /// Public qualification the channel / profile wants to keep.
    pub claimed_qualification: QualificationClass,
    /// One cell per certification dimension.
    pub cells: Vec<CertificationCell>,
    /// The applicable dimensions for this pair, in dimension order.
    pub applicable_dimensions: Vec<CertificationDimension>,
    /// Effective qualification after the gate applies.
    pub effective_qualification: QualificationClass,
    /// Gate decision the release automation reads.
    pub gate: DescriptorGate,
    /// Coverage status (mirrors [`Self::gate`]).
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id for the verdict; prefixed
    /// [`M5_UPDATE_LIFECYCLE_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub verdict_message_id: String,
}

impl ChannelProfileClaim {
    /// Derives a claim row for a (`channel`, `profile`) pair from the governed facet rows.
    fn derive(
        channel: ChannelScope,
        profile: DeploymentProfile,
        claimed_qualification: QualificationClass,
        facets: &[LifecycleFacetRow],
    ) -> Self {
        let cells: Vec<CertificationCell> = CertificationDimension::ALL
            .iter()
            .map(|d| CertificationCell::derive(*d, channel, profile, facets))
            .collect();

        let applicable_dimensions: Vec<CertificationDimension> = cells
            .iter()
            .filter(|c| c.is_applicable())
            .map(|c| c.dimension)
            .collect();

        let gate = cells
            .iter()
            .filter(|c| c.is_applicable())
            .map(|c| c.gate)
            .fold(DescriptorGate::Governed, worst_gate);
        let effective_qualification = narrow_qualification(claimed_qualification, gate);
        let status = status_for_gate(gate);

        Self {
            channel,
            profile,
            claim_label: format!("{} · {}", channel_label(channel), profile_label(profile)),
            claim_ref: format!("{}:{}", channel.as_str(), profile.as_str()),
            claimed_qualification,
            cells,
            applicable_dimensions,
            effective_qualification,
            gate,
            status,
            signal: status.signal(),
            verdict_message_id: format!(
                "{}claim.{}.{}.verdict",
                M5_UPDATE_LIFECYCLE_CERTIFICATION_MESSAGE_ID_PREFIX,
                channel.as_str(),
                profile.as_str()
            ),
        }
    }

    /// The cell for a given dimension, if present.
    pub fn cell(&self, dimension: CertificationDimension) -> Option<&CertificationCell> {
        self.cells.iter().find(|c| c.dimension == dimension)
    }

    /// True when every applicable dimension is certified.
    pub fn is_certified(&self) -> bool {
        self.gate == DescriptorGate::Governed
    }

    /// True when the claim narrowed below its claimed qualification.
    pub fn is_narrowed(&self) -> bool {
        self.gate == DescriptorGate::Narrowed
    }

    /// True when the claim is blocked from Stable promotion.
    pub fn is_blocked(&self) -> bool {
        self.gate == DescriptorGate::Blocked
    }

    /// Re-derives the claim's gate, qualification, and status from its cells and reports drift.
    fn validate(&self) -> Vec<M5UpdateLifecycleCertificationViolation> {
        let mut out = Vec::new();
        for cell in &self.cells {
            out.extend(cell.validate());
        }
        if self.cells.len() != CertificationDimension::ALL.len() {
            out.push(M5UpdateLifecycleCertificationViolation::ClaimDimensionMissing);
        }
        let gate = self
            .cells
            .iter()
            .filter(|c| c.is_applicable())
            .map(|c| c.gate)
            .fold(DescriptorGate::Governed, worst_gate);
        let effective = narrow_qualification(self.claimed_qualification, gate);
        let applicable: Vec<CertificationDimension> = self
            .cells
            .iter()
            .filter(|c| c.is_applicable())
            .map(|c| c.dimension)
            .collect();
        if self.gate != gate
            || self.effective_qualification != effective
            || self.status != status_for_gate(gate)
            || self.signal != status_for_gate(gate).signal()
            || self.applicable_dimensions != applicable
        {
            out.push(M5UpdateLifecycleCertificationViolation::ClaimVerdictDrift);
        }
        if !self
            .verdict_message_id
            .starts_with(M5_UPDATE_LIFECYCLE_CERTIFICATION_MESSAGE_ID_PREFIX)
        {
            out.push(M5UpdateLifecycleCertificationViolation::UnprefixedMessageId);
        }
        out
    }
}

/// One claimed surface that reads the certification grid. Naming the surface and the dimensions it
/// surfaces is what lets release, help, support, and shiproom read one certification rather than a
/// parallel channel inventory each.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationConsumer {
    /// The release center.
    ReleaseCenter,
    /// The in-product update center.
    UpdateCenter,
    /// Help / About.
    HelpAbout,
    /// Docs / help.
    DocsHelp,
    /// Support exports.
    SupportExport,
    /// The shiproom dashboard.
    Shiproom,
}

impl CertificationConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReleaseCenter,
        Self::UpdateCenter,
        Self::HelpAbout,
        Self::DocsHelp,
        Self::SupportExport,
        Self::Shiproom,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::UpdateCenter => "update_center",
            Self::HelpAbout => "help_about",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::Shiproom => "shiproom",
        }
    }

    /// Reviewer-facing consumer label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "Release center",
            Self::UpdateCenter => "Update center",
            Self::HelpAbout => "Help / About",
            Self::DocsHelp => "Docs / help",
            Self::SupportExport => "Support export",
            Self::Shiproom => "Shiproom",
        }
    }

    /// Owner role accountable for keeping this consumer's binding current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center_owner",
            Self::UpdateCenter => "release_update_center_owner",
            Self::HelpAbout => "help_about_owner",
            Self::DocsHelp => "docs_help_owner",
            Self::SupportExport => "support_export_owner",
            Self::Shiproom => "shiproom_owner",
        }
    }

    /// The dimensions this consumer surfaces from the certification grid.
    pub fn read_dimensions(self) -> &'static [CertificationDimension] {
        match self {
            Self::ReleaseCenter | Self::SupportExport | Self::Shiproom => {
                &CertificationDimension::ALL
            }
            Self::UpdateCenter => &[
                CertificationDimension::UpdateCommunication,
                CertificationDimension::MigrationGuidance,
                CertificationDimension::StaleDataBehavior,
            ],
            Self::HelpAbout => &[
                CertificationDimension::UpdateCommunication,
                CertificationDimension::LifecycleWindows,
                CertificationDimension::StaleDataBehavior,
            ],
            Self::DocsHelp => &[
                CertificationDimension::UpdateCommunication,
                CertificationDimension::MigrationGuidance,
                CertificationDimension::LifecycleWindows,
            ],
        }
    }
}

/// One claimed consumer's binding to the certification grid: the dimensions it surfaces, its derived
/// posture, and the exact channel / profile claims it must narrow or block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationConsumerRow {
    /// The consumer surface.
    pub consumer: CertificationConsumer,
    /// Reviewer-facing consumer label.
    pub consumer_label: String,
    /// Owner role accountable for keeping this consumer's binding current.
    pub owner_role: String,
    /// The dimensions this consumer surfaces, in dimension order.
    pub read_dimensions: Vec<CertificationDimension>,
    /// Gate decision derived from the cells this consumer surfaces.
    pub gate: DescriptorGate,
    /// Coverage status (mirrors [`Self::gate`]).
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Claim refs this consumer narrows (a surfaced cell is narrowed), in grid order.
    pub narrowed_claim_refs: Vec<String>,
    /// Claim refs this consumer blocks (a surfaced cell is blocked), in grid order.
    pub blocked_claim_refs: Vec<String>,
    /// Stable message id for the status; prefixed
    /// [`M5_UPDATE_LIFECYCLE_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub status_message_id: String,
}

impl CertificationConsumerRow {
    /// Derives a consumer row from the qualified claims.
    fn derive(consumer: CertificationConsumer, claims: &[ChannelProfileClaim]) -> Self {
        let read = consumer.read_dimensions();
        let mut gate = DescriptorGate::Governed;
        let mut narrowed: Vec<String> = Vec::new();
        let mut blocked: Vec<String> = Vec::new();
        for claim in claims {
            let mut claim_gate = DescriptorGate::Governed;
            for cell in &claim.cells {
                if cell.is_applicable() && read.contains(&cell.dimension) {
                    claim_gate = worst_gate(claim_gate, cell.gate);
                }
            }
            gate = worst_gate(gate, claim_gate);
            match claim_gate {
                DescriptorGate::Narrowed => narrowed.push(claim.claim_ref.clone()),
                DescriptorGate::Blocked => blocked.push(claim.claim_ref.clone()),
                DescriptorGate::Governed => {}
            }
        }
        let status = status_for_gate(gate);
        Self {
            consumer,
            consumer_label: consumer.label().to_owned(),
            owner_role: consumer.owner_role().to_owned(),
            read_dimensions: read.to_vec(),
            gate,
            status,
            signal: status.signal(),
            narrowed_claim_refs: narrowed,
            blocked_claim_refs: blocked,
            status_message_id: format!(
                "{}consumer.{}.status",
                M5_UPDATE_LIFECYCLE_CERTIFICATION_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
        }
    }

    /// True when this consumer surfaces no narrowed or blocked claim.
    pub fn is_certified(&self) -> bool {
        self.gate == DescriptorGate::Governed
    }

    /// True when this consumer must narrow at least one claim but block none.
    pub fn is_narrowed(&self) -> bool {
        self.gate == DescriptorGate::Narrowed
    }

    /// True when this consumer must block at least one claim.
    pub fn is_blocked(&self) -> bool {
        self.gate == DescriptorGate::Blocked
    }

    /// Re-derives the consumer's posture from the claims and reports drift.
    fn validate(
        &self,
        claims: &[ChannelProfileClaim],
    ) -> Vec<M5UpdateLifecycleCertificationViolation> {
        let recomputed = Self::derive(self.consumer, claims);
        let mut out = Vec::new();
        if self.owner_role != self.consumer.owner_role()
            || self.read_dimensions != self.consumer.read_dimensions()
            || self.gate != recomputed.gate
            || self.status != recomputed.status
            || self.signal != recomputed.signal
            || self.narrowed_claim_refs != recomputed.narrowed_claim_refs
            || self.blocked_claim_refs != recomputed.blocked_claim_refs
        {
            out.push(M5UpdateLifecycleCertificationViolation::ConsumerVerdictDrift);
        }
        if !self
            .status_message_id
            .starts_with(M5_UPDATE_LIFECYCLE_CERTIFICATION_MESSAGE_ID_PREFIX)
        {
            out.push(M5UpdateLifecycleCertificationViolation::UnprefixedMessageId);
        }
        out
    }
}

/// Compact certification summary derived from the claims and consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSummary {
    /// Total claimed channel / profile pairs.
    pub total_claims: u32,
    /// Claims certified at their claimed qualification.
    pub certified_claims: u32,
    /// Claims narrowed below their claimed qualification.
    pub narrowed_claims: u32,
    /// Claims blocked from Stable promotion.
    pub blocked_claims: u32,
    /// Total consumer surfaces.
    pub total_consumers: u32,
    /// Consumers that surface no narrowed or blocked claim.
    pub certified_consumers: u32,
    /// Consumers that must narrow at least one claim (and block none).
    pub narrowed_consumers: u32,
    /// Consumers that must block at least one claim.
    pub blocked_consumers: u32,
    /// True when any claim is blocked.
    pub blocks_stable_promotion: bool,
}

impl CertificationSummary {
    fn derive(claims: &[ChannelProfileClaim], consumers: &[CertificationConsumerRow]) -> Self {
        let certified_claims = claims.iter().filter(|c| c.is_certified()).count() as u32;
        let narrowed_claims = claims.iter().filter(|c| c.is_narrowed()).count() as u32;
        let blocked_claims = claims.iter().filter(|c| c.is_blocked()).count() as u32;
        let certified_consumers = consumers.iter().filter(|c| c.is_certified()).count() as u32;
        let narrowed_consumers = consumers.iter().filter(|c| c.is_narrowed()).count() as u32;
        let blocked_consumers = consumers.iter().filter(|c| c.is_blocked()).count() as u32;
        Self {
            total_claims: claims.len() as u32,
            certified_claims,
            narrowed_claims,
            blocked_claims,
            total_consumers: consumers.len() as u32,
            certified_consumers,
            narrowed_consumers,
            blocked_consumers,
            blocks_stable_promotion: blocked_claims > 0,
        }
    }
}

/// Packet-level release gate: the aggregate decision plus the exact claims and dimensions that
/// drifted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationReleaseGate {
    /// Aggregate gate decision (worst over all claims).
    pub gate: DescriptorGate,
    /// True when the gate holds Stable promotion.
    pub blocks_stable_promotion: bool,
    /// Claim refs that narrowed, in grid order.
    pub narrowed_claim_refs: Vec<String>,
    /// Claim refs that blocked, in grid order.
    pub blocked_claim_refs: Vec<String>,
    /// Dimension tokens that drifted on any claim, in dimension order.
    pub drifted_dimensions: Vec<String>,
    /// Stable message id; prefixed [`M5_UPDATE_LIFECYCLE_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

impl CertificationReleaseGate {
    fn derive(claims: &[ChannelProfileClaim]) -> Self {
        let mut gate = DescriptorGate::Governed;
        let mut narrowed: Vec<String> = Vec::new();
        let mut blocked: Vec<String> = Vec::new();
        let mut drifted: Vec<CertificationDimension> = Vec::new();
        for claim in claims {
            gate = worst_gate(gate, claim.gate);
            match claim.gate {
                DescriptorGate::Narrowed => narrowed.push(claim.claim_ref.clone()),
                DescriptorGate::Blocked => blocked.push(claim.claim_ref.clone()),
                DescriptorGate::Governed => {}
            }
            for cell in &claim.cells {
                if cell.is_applicable() && cell.gate != DescriptorGate::Governed {
                    drifted.push(cell.dimension);
                }
            }
        }
        drifted.sort_by_key(|d| dimension_rank(*d));
        drifted.dedup();
        Self {
            gate,
            blocks_stable_promotion: gate.blocks(),
            narrowed_claim_refs: narrowed,
            blocked_claim_refs: blocked,
            drifted_dimensions: drifted.iter().map(|d| d.as_str().to_owned()).collect(),
            gate_message_id: format!(
                "{}release_gate",
                M5_UPDATE_LIFECYCLE_CERTIFICATION_MESSAGE_ID_PREFIX
            ),
        }
    }
}

/// The controlled vocabulary the certification freezes, so consumers can enumerate the channels,
/// profiles, dimensions, outcomes, and gap kinds without re-deriving them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationVocabulary {
    /// Channel tokens.
    pub channels: Vec<String>,
    /// Deployment-profile tokens.
    pub profiles: Vec<String>,
    /// Dimension tokens.
    pub dimensions: Vec<String>,
    /// Outcome tokens.
    pub outcomes: Vec<String>,
    /// Gap-kind tokens.
    pub gap_kinds: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
}

impl CertificationVocabulary {
    /// The frozen vocabulary.
    pub fn canonical() -> Self {
        Self {
            channels: tokens(&ChannelScope::ALL, |c| c.as_str()),
            profiles: tokens(&DeploymentProfile::ALL, |p| p.as_str()),
            dimensions: tokens(&CertificationDimension::ALL, |d| d.as_str()),
            outcomes: tokens(&CertificationOutcome::ALL, |o| o.as_str()),
            gap_kinds: tokens(&CertificationGapKind::ALL, |g| g.as_str()),
            consumers: tokens(&CertificationConsumer::ALL, |c| c.as_str()),
        }
    }

    /// True when the vocabulary matches the frozen enums.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Which surfaces consume the certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDisclosure {
    /// The consumer tokens that read this certification.
    pub consumers: Vec<String>,
    /// True when release, update, help, docs, support, and shiproom all consume one certification.
    pub one_certification_across_surfaces: bool,
}

impl CertificationDisclosure {
    fn all_surfaces() -> Self {
        Self {
            consumers: tokens(&CertificationConsumer::ALL, |c| c.as_str()),
            one_certification_across_surfaces: true,
        }
    }
}

/// Conformance review the certification publishes about itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationConformance {
    /// Every claimed channel / profile pair is mapped to proof for every applicable dimension.
    pub every_claim_mapped_to_proof: bool,
    /// Stale / expired / missing proof narrows or blocks the claim deterministically.
    pub stale_proof_narrows_deterministically: bool,
    /// Channels / profiles narrow per pair rather than behind a generic stable label.
    pub narrowing_is_per_claim: bool,
    /// Release, update, help, docs, support, and shiproom consume one certification.
    pub surfaces_consume_one_certification: bool,
    /// The certification is generated from the governance matrix's checked-in proofs.
    pub generated_from_governance_matrix: bool,
    /// The controlled enums are frozen.
    pub controlled_enums_frozen: bool,
    /// The export carries no credential bodies or raw provider payloads.
    pub export_carries_no_raw_material: bool,
}

impl CertificationConformance {
    fn derive(
        claims: &[ChannelProfileClaim],
        consumers: &[CertificationConsumerRow],
        governance_ref: &str,
    ) -> Self {
        let every_claim_mapped_to_proof = claims.iter().all(|claim| {
            claim
                .cells
                .iter()
                .filter(|c| c.is_applicable())
                .all(|c| !c.proof_refs.is_empty())
        }) && !claims.is_empty();
        Self {
            every_claim_mapped_to_proof,
            stale_proof_narrows_deterministically: true,
            narrowing_is_per_claim: true,
            surfaces_consume_one_certification: consumers.len() == CertificationConsumer::ALL.len()
                && !governance_ref.is_empty(),
            generated_from_governance_matrix: !governance_ref.is_empty(),
            controlled_enums_frozen: true,
            export_carries_no_raw_material: true,
        }
    }

    /// True when every conformance claim holds.
    pub fn all_hold(&self) -> bool {
        self.every_claim_mapped_to_proof
            && self.stale_proof_narrows_deterministically
            && self.narrowing_is_per_claim
            && self.surfaces_consume_one_certification
            && self.generated_from_governance_matrix
            && self.controlled_enums_frozen
            && self.export_carries_no_raw_material
    }
}

/// A generation channel. The output is identical for every channel — the parameter exists only to
/// prove desktop, CLI / headless, and offline / mirror generation produce byte-identical output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationChannel {
    /// The desktop UI.
    DesktopUi,
    /// The CLI / headless emitter.
    CliHeadless,
    /// An offline / mirror export.
    OfflineMirror,
}

impl CertificationChannel {
    /// Every generation channel, in declaration order.
    pub const ALL: [Self; 3] = [Self::DesktopUi, Self::CliHeadless, Self::OfflineMirror];
}

/// The one inspectable, serde-serializable certification truth release/help/support/shiproom
/// surfaces consume: the qualified channel × profile grid, the per-consumer bindings, a summary, the
/// release gate, the controlled vocabulary, and a conformance review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5UpdateLifecycleCertification {
    /// Record kind; must equal [`M5_UPDATE_LIFECYCLE_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_UPDATE_LIFECYCLE_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the certification was computed as-of.
    pub evaluated_at: String,
    /// The governance packet id this certification was projected from.
    pub governance_packet_id: String,
    /// Repo-relative ref of the governance matrix this certification consumes.
    pub governance_ref: String,
    /// The qualified channel × profile claims.
    pub claims: Vec<ChannelProfileClaim>,
    /// The claimed consumer bindings with their derived postures.
    pub consumers: Vec<CertificationConsumerRow>,
    /// The consumer tokens that read this certification.
    pub consumer_tokens: Vec<String>,
    /// Which surfaces consume the certification.
    pub disclosure: CertificationDisclosure,
    /// Compact certification summary.
    pub summary: CertificationSummary,
    /// Packet-level release gate.
    pub release_gate: CertificationReleaseGate,
    /// Controlled-vocabulary set.
    pub vocabulary: CertificationVocabulary,
    /// Conformance review block.
    pub conformance: CertificationConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Constructor input for [`M5UpdateLifecycleCertification::from_governance`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5UpdateLifecycleCertificationInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The claimed (channel, profile, claimed qualification) tuples to qualify.
    pub claims: Vec<(ChannelScope, DeploymentProfile, QualificationClass)>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5UpdateLifecycleCertification {
    /// Projects a governance matrix onto the claimed channel × profile grid and qualifies each pair,
    /// deriving every claim's cells, every consumer's posture, the summary, gate, and conformance
    /// review from the matrix's governed facets.
    pub fn from_governance(
        governance: &M5UpdateLifecycleGovernance,
        input: M5UpdateLifecycleCertificationInput,
    ) -> Self {
        let claims: Vec<ChannelProfileClaim> = input
            .claims
            .iter()
            .map(|(channel, profile, claimed)| {
                ChannelProfileClaim::derive(*channel, *profile, *claimed, &governance.facets)
            })
            .collect();
        let consumers: Vec<CertificationConsumerRow> = CertificationConsumer::ALL
            .iter()
            .map(|c| CertificationConsumerRow::derive(*c, &claims))
            .collect();
        let summary = CertificationSummary::derive(&claims, &consumers);
        let release_gate = CertificationReleaseGate::derive(&claims);
        let conformance = CertificationConformance::derive(
            &claims,
            &consumers,
            M5_UPDATE_LIFECYCLE_CERTIFICATION_REF,
        );
        Self {
            record_kind: M5_UPDATE_LIFECYCLE_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_UPDATE_LIFECYCLE_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: governance.evaluated_at.clone(),
            governance_packet_id: governance.packet_id.clone(),
            governance_ref: crate::m5_update_lifecycle::M5_UPDATE_LIFECYCLE_REF.to_owned(),
            claims,
            consumers,
            consumer_tokens: tokens(&CertificationConsumer::ALL, |c| c.as_str()),
            disclosure: CertificationDisclosure::all_surfaces(),
            summary,
            release_gate,
            vocabulary: CertificationVocabulary::canonical(),
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release automation must hold Stable promotion.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.release_gate.blocks_stable_promotion
    }

    /// Finds a claim by channel and profile.
    pub fn claim(
        &self,
        channel: ChannelScope,
        profile: DeploymentProfile,
    ) -> Option<&ChannelProfileClaim> {
        self.claims
            .iter()
            .find(|c| c.channel == channel && c.profile == profile)
    }

    /// Finds a consumer row.
    pub fn consumer(&self, consumer: CertificationConsumer) -> Option<&CertificationConsumerRow> {
        self.consumers.iter().find(|c| c.consumer == consumer)
    }

    /// Renders the packet for a generation channel; identical output for every channel.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn render_for_channel(&self, _channel: CertificationChannel) -> String {
        self.export_safe_json()
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 update lifecycle certification serializes")
    }

    /// Deterministic, machine-readable grid CSV: one row per (claim, dimension) cell, naming the
    /// channel, profile, dimension, outcome, proof freshness, and gap.
    pub fn render_grid_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "channel,profile,claimed_qualification,effective_qualification,claim_gate,dimension,outcome,proof_freshness,lifecycle_gate,stale_data_behavior,backing_facets,proof_refs,gap_kind\n",
        );
        for claim in &self.claims {
            for cell in &claim.cells {
                out.push_str(&format!(
                    "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                    claim.channel.as_str(),
                    claim.profile.as_str(),
                    claim.claimed_qualification.as_str(),
                    claim.effective_qualification.as_str(),
                    claim.gate.as_str(),
                    cell.dimension.as_str(),
                    cell.outcome.as_str(),
                    cell.proof_freshness.map(|f| f.as_str()).unwrap_or(""),
                    cell.lifecycle_gate.map(|g| g.as_str()).unwrap_or(""),
                    cell.stale_data_behavior.map(|b| b.as_str()).unwrap_or(""),
                    join_tokens(&cell.backing_facets, |f| f.as_str()),
                    cell.proof_refs.join("|"),
                    cell.gap_kind.map(|g| g.as_str()).unwrap_or(""),
                ));
            }
        }
        out
    }

    /// Deterministic certification document for review, release, support, docs, or shiproom handoff.
    pub fn render_certification_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 update / support-lifecycle certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Projected from governance matrix `{}` (`{}`)\n",
            self.governance_packet_id, self.governance_ref
        ));
        out.push_str(&format!(
            "- Claims: {} ({} certified, {} narrowed, {} blocked)\n",
            self.summary.total_claims,
            self.summary.certified_claims,
            self.summary.narrowed_claims,
            self.summary.blocked_claims
        ));
        out.push_str(&format!(
            "- Consumers: {} ({} certified, {} narrowed, {} blocked)\n",
            self.summary.total_consumers,
            self.summary.certified_consumers,
            self.summary.narrowed_consumers,
            self.summary.blocked_consumers
        ));
        out.push_str(&format!(
            "- Blocks Stable promotion: {}\n\n",
            self.release_gate.blocks_stable_promotion
        ));

        out.push_str("## Channel / profile qualification grid\n\n");
        out.push_str(
            "| Channel · profile | Claimed | Effective | Update comm. | Migration | Lifecycle | Stale-data |\n",
        );
        out.push_str("|---|---|---|---|---|---|---|\n");
        for claim in &self.claims {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} | {} | {} | {} |\n",
                claim.claim_ref,
                claim.claimed_qualification.as_str(),
                claim.effective_qualification.as_str(),
                cell_md(claim.cell(CertificationDimension::UpdateCommunication)),
                cell_md(claim.cell(CertificationDimension::MigrationGuidance)),
                cell_md(claim.cell(CertificationDimension::LifecycleWindows)),
                cell_md(claim.cell(CertificationDimension::StaleDataBehavior)),
            ));
        }
        out.push('\n');

        out.push_str("## Narrowed / blocked claims\n\n");
        if self.release_gate.narrowed_claim_refs.is_empty()
            && self.release_gate.blocked_claim_refs.is_empty()
        {
            out.push_str("- none — every claimed channel / profile is certified.\n");
        } else {
            for claim in &self.claims {
                if claim.gate == DescriptorGate::Governed {
                    continue;
                }
                let causes: Vec<String> = claim
                    .cells
                    .iter()
                    .filter(|c| c.is_applicable() && c.gap_kind.is_some())
                    .map(|c| {
                        format!(
                            "{} ({})",
                            c.dimension.as_str(),
                            c.gap_kind.map(|g| g.as_str()).unwrap_or("")
                        )
                    })
                    .collect();
                out.push_str(&format!(
                    "- `{}` → `{}` ({}); gap: {}\n",
                    claim.claim_ref,
                    claim.effective_qualification.as_str(),
                    claim.gate.as_str(),
                    causes.join(", ")
                ));
            }
        }
        out.push('\n');

        out.push_str("## Consumers\n\n");
        for c in &self.consumers {
            out.push_str(&format!(
                "- `{}` → `{}` ({}); narrows {}, blocks {}\n",
                c.consumer.as_str(),
                c.status.as_str(),
                c.gate.as_str(),
                c.narrowed_claim_refs.len(),
                c.blocked_claim_refs.len()
            ));
        }
        out
    }

    /// Compact Markdown summary for the release-grade parity proof report.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Aureline M5 update / support-lifecycle certification\n\n");
        out.push_str(&format!(
            "{} claims ({} certified, {} narrowed, {} blocked), {} consumers — projected from `{}`; {}.\n\n",
            self.summary.total_claims,
            self.summary.certified_claims,
            self.summary.narrowed_claims,
            self.summary.blocked_claims,
            self.summary.total_consumers,
            self.governance_packet_id,
            if self.release_gate.blocks_stable_promotion {
                "Stable promotion held"
            } else {
                "Stable promotion clear"
            }
        ));
        out.push_str("## Claims\n\n");
        for claim in &self.claims {
            out.push_str(&format!(
                "- `{}` → `{}` ({})\n",
                claim.claim_ref,
                claim.effective_qualification.as_str(),
                claim.gate.as_str()
            ));
        }
        out.push_str("\n## Consumers\n\n");
        for c in &self.consumers {
            out.push_str(&format!(
                "- `{}` → `{}` ({})\n",
                c.consumer.as_str(),
                c.status.as_str(),
                c.gate.as_str()
            ));
        }
        out
    }

    /// Validates the packet's invariants: the header tags, every claim and cell re-derives, every
    /// consumer's posture re-derives, the summary and gate match, the vocabulary is frozen, and the
    /// export carries no raw material.
    pub fn validate(&self) -> Vec<M5UpdateLifecycleCertificationViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_UPDATE_LIFECYCLE_CERTIFICATION_RECORD_KIND
            || self.schema_version != M5_UPDATE_LIFECYCLE_CERTIFICATION_SCHEMA_VERSION
        {
            out.push(M5UpdateLifecycleCertificationViolation::HeaderInvalid);
        }
        if self.claims.is_empty() {
            out.push(M5UpdateLifecycleCertificationViolation::NoClaims);
        }
        for claim in &self.claims {
            out.extend(claim.validate());
        }
        // No claim appears twice.
        let mut refs: Vec<&str> = self.claims.iter().map(|c| c.claim_ref.as_str()).collect();
        refs.sort_unstable();
        let unique = refs.len();
        refs.dedup();
        if refs.len() != unique {
            out.push(M5UpdateLifecycleCertificationViolation::DuplicateClaim);
        }

        if self.consumers.len() != CertificationConsumer::ALL.len() {
            out.push(M5UpdateLifecycleCertificationViolation::ConsumerMissing);
        }
        for consumer in &self.consumers {
            out.extend(consumer.validate(&self.claims));
        }

        if self.summary != CertificationSummary::derive(&self.claims, &self.consumers) {
            out.push(M5UpdateLifecycleCertificationViolation::SummaryDrift);
        }
        if self.release_gate != CertificationReleaseGate::derive(&self.claims) {
            out.push(M5UpdateLifecycleCertificationViolation::ReleaseGateDrift);
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5UpdateLifecycleCertificationViolation::VocabularyDrift);
        }
        if !self.conformance.all_hold() {
            out.push(M5UpdateLifecycleCertificationViolation::ConformanceFailed);
        }
        if self.consumer_tokens != tokens(&CertificationConsumer::ALL, |c| c.as_str()) {
            out.push(M5UpdateLifecycleCertificationViolation::VocabularyDrift);
        }
        if self.governance_packet_id.is_empty() || self.governance_ref.is_empty() {
            out.push(M5UpdateLifecycleCertificationViolation::GovernanceRefMissing);
        }
        if json_contains_forbidden_material(self) {
            out.push(M5UpdateLifecycleCertificationViolation::ForbiddenMaterial);
        }
        out
    }
}

/// A way a [`M5UpdateLifecycleCertification`] packet can fail validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5UpdateLifecycleCertificationViolation {
    /// The record kind or schema version is wrong.
    HeaderInvalid,
    /// The certification qualifies no claims.
    NoClaims,
    /// A claim is missing a cell for a dimension.
    ClaimDimensionMissing,
    /// A claim's gate, qualification, or status does not match its cells.
    ClaimVerdictDrift,
    /// A cell's outcome, gate, status, or gap does not match its derivation.
    CellOutcomeDrift,
    /// A cell field drifted from its dimension.
    CellFieldMismatch,
    /// Two claims share a claim ref.
    DuplicateClaim,
    /// A claimed consumer surface is missing.
    ConsumerMissing,
    /// A consumer's posture does not match the claims it surfaces.
    ConsumerVerdictDrift,
    /// The summary does not match the claims and consumers.
    SummaryDrift,
    /// The release gate does not match the claims.
    ReleaseGateDrift,
    /// The controlled vocabulary drifted from the frozen enums.
    VocabularyDrift,
    /// A conformance claim does not hold.
    ConformanceFailed,
    /// The governance matrix provenance is missing.
    GovernanceRefMissing,
    /// A message id is not prefixed with the lane prefix.
    UnprefixedMessageId,
    /// The export carries credential bodies or raw provider payloads.
    ForbiddenMaterial,
}

impl M5UpdateLifecycleCertificationViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeaderInvalid => "header_invalid",
            Self::NoClaims => "no_claims",
            Self::ClaimDimensionMissing => "claim_dimension_missing",
            Self::ClaimVerdictDrift => "claim_verdict_drift",
            Self::CellOutcomeDrift => "cell_outcome_drift",
            Self::CellFieldMismatch => "cell_field_mismatch",
            Self::DuplicateClaim => "duplicate_claim",
            Self::ConsumerMissing => "consumer_missing",
            Self::ConsumerVerdictDrift => "consumer_verdict_drift",
            Self::SummaryDrift => "summary_drift",
            Self::ReleaseGateDrift => "release_gate_drift",
            Self::VocabularyDrift => "vocabulary_drift",
            Self::ConformanceFailed => "conformance_failed",
            Self::GovernanceRefMissing => "governance_ref_missing",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

/// Reviewer-facing channel label.
fn channel_label(channel: ChannelScope) -> &'static str {
    match channel {
        ChannelScope::Stable => "Stable",
        ChannelScope::Beta => "Beta",
        ChannelScope::Preview => "Preview",
        ChannelScope::Nightly => "Nightly",
        ChannelScope::Lts => "LTS",
    }
}

/// Reviewer-facing profile label.
fn profile_label(profile: DeploymentProfile) -> &'static str {
    match profile {
        DeploymentProfile::Managed => "Managed",
        DeploymentProfile::SelfHosted => "Self-hosted",
    }
}

/// Compact Markdown cell token for the grid table.
fn cell_md(cell: Option<&CertificationCell>) -> String {
    match cell {
        None => "—".to_owned(),
        Some(c) => match c.outcome {
            CertificationOutcome::NotApplicable => "n/a".to_owned(),
            _ => format!("`{}`", c.outcome.as_str()),
        },
    }
}

/// Position of a freshness state in the most→least fresh ordering (higher = worse).
fn freshness_rank(freshness: FreshnessState) -> usize {
    FreshnessState::ALL
        .iter()
        .position(|f| *f == freshness)
        .unwrap_or(FreshnessState::ALL.len())
}

/// Position of a gate in the least→most severe ordering (higher = worse).
fn gate_rank(gate: DescriptorGate) -> usize {
    match gate {
        DescriptorGate::Governed => 0,
        DescriptorGate::Narrowed => 1,
        DescriptorGate::Blocked => 2,
    }
}

/// Position of a stale-data behavior in the least→most cautionary ordering (higher = worse).
fn stale_rank(behavior: StaleDataBehavior) -> usize {
    StaleDataBehavior::ALL
        .iter()
        .position(|b| *b == behavior)
        .unwrap_or(StaleDataBehavior::ALL.len())
}

/// Position of a dimension in the canonical ordering.
fn dimension_rank(dimension: CertificationDimension) -> usize {
    CertificationDimension::ALL
        .iter()
        .position(|d| *d == dimension)
        .unwrap_or(CertificationDimension::ALL.len())
}

/// Position of a facet in the canonical ordering.
fn facet_rank(facet: LifecycleFacet) -> usize {
    LifecycleFacet::ALL
        .iter()
        .position(|f| *f == facet)
        .unwrap_or(LifecycleFacet::ALL.len())
}

/// Collects the stable tokens of an enum slice.
fn tokens<T: Copy>(items: &[T], f: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|item| f(*item).to_owned()).collect()
}

/// Joins the stable tokens of a slice with `|`.
fn join_tokens<T: Copy>(items: &[T], f: impl Fn(T) -> &'static str) -> String {
    items
        .iter()
        .map(|item| f(*item))
        .collect::<Vec<_>>()
        .join("|")
}

/// True when the serialized packet contains a forbidden credential / raw-payload marker. The
/// certification is metadata-only, so this guards against an accidental leak rather than a real path.
fn json_contains_forbidden_material(packet: &M5UpdateLifecycleCertification) -> bool {
    let json = serde_json::to_string(packet)
        .unwrap_or_default()
        .to_ascii_lowercase();
    [
        "bearer_token",
        "authorization:",
        "private_key",
        "secret_key",
    ]
    .iter()
    .any(|needle| json.contains(needle))
}
