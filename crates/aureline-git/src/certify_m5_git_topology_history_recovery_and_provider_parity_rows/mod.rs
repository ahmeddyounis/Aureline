//! Certification register for the claimed M5 Git and source-acquisition rows.
//!
//! This module is the claim-hardening capstone over the M5 Git depth lane. The
//! earlier topology, worktree-scope, history-surgery, and stash/reflog/checkpoint
//! rows only stay green if every claimed M5 Git or source-acquisition surface can
//! prove, under current evidence, that it is honest about repository topology,
//! that it scopes operations to the right worktree or root, that any history
//! rewrite is previewed and recoverable, and that local Git truth stays
//! authoritative when a provider overlay is degraded or absent.
//!
//! Each [`M5GitCertificationRow`] binds one claimed surface to the four
//! certification dimensions it must prove ([`CertificationDimension`]), records
//! per-dimension evidence freshness and proof state, and carries a derived
//! [`CertificationVerdict`]. The verdict is never declared independently of the
//! evidence: [`M5GitCertificationRow::derive_verdict`] folds the dimensions
//! fail-closed, and validation rejects any packet whose declared verdict does not
//! match its evidence. A stale or unrun dimension narrows the row to
//! [`CertificationVerdict::RetestPending`], an honestly partial dimension narrows
//! it to [`CertificationVerdict::Limited`], and a failed or missing dimension
//! narrows it to [`CertificationVerdict::Unsupported`]. This is the downgrade
//! automation the spec requires: claim truth is not a manual flag.
//!
//! The [`CertificationParityAudit`] proves every consumer surface — product,
//! docs/help, CLI, support export, evaluation packs, claim-publication manifests,
//! and release/public-truth — reflects the same row verdicts, so no surface can
//! advertise wider than the current machine-readable row. The packet references
//! the upstream topology, history-surgery, stash-recovery, topology-action, and
//! frozen-matrix contracts by id rather than redefining them, so all surfaces read
//! one certification register.
//!
//! Certification truth is never reduced to a badge: the rows control whether a
//! claim may be published. Raw paths, raw object bytes, raw branch names, raw
//! patch/reflog/stash bodies, raw provider payloads, and credentials stay outside
//! the support boundary.
//!
//! The boundary schema is
//! [`schemas/git/certify-m5-git-topology-history-recovery-and-provider-parity-rows.schema.json`](../../../../schemas/git/certify-m5-git-topology-history-recovery-and-provider-parity-rows.schema.json).
//! The contract doc is
//! [`docs/git/m5/certify_m5_git_topology_history_recovery_and_provider_parity_rows.md`](../../../../docs/git/m5/certify_m5_git_topology_history_recovery_and_provider_parity_rows.md).
//! The protected fixture directory is
//! [`fixtures/git/m5/certification-corpus/`](../../../../fixtures/git/m5/certification-corpus/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5GitCertificationPacket`].
pub const M5_GIT_CERTIFICATION_RECORD_KIND: &str =
    "certify_m5_git_topology_history_recovery_and_provider_parity_rows";

/// Schema version for M5 Git certification records.
pub const M5_GIT_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_GIT_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/git/certify-m5-git-topology-history-recovery-and-provider-parity-rows.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_GIT_CERTIFICATION_DOC_REF: &str =
    "docs/git/m5/certify_m5_git_topology_history_recovery_and_provider_parity_rows.md";

/// Repo-relative path of the frozen M5 topology/history-surgery matrix contract.
pub const M5_GIT_CERTIFICATION_MATRIX_CONTRACT_REF: &str =
    "schemas/git/freeze-the-m5-repository-topology-worktree-scope-history-surgery-and-checkpoint-recovery-matrix.schema.json";

/// Repo-relative path of the repository-topology contract.
pub const M5_GIT_CERTIFICATION_TOPOLOGY_CONTRACT_REF: &str = "schemas/git/topology.schema.json";

/// Repo-relative path of the topology-action review contract.
pub const M5_GIT_CERTIFICATION_TOPOLOGY_ACTION_CONTRACT_REF: &str =
    "schemas/git/topology_action_review.schema.json";

/// Repo-relative path of the history-surgery review contract.
pub const M5_GIT_CERTIFICATION_HISTORY_SURGERY_CONTRACT_REF: &str =
    "schemas/git/history-surgery-review.schema.json";

/// Repo-relative path of the stash/reflog/checkpoint recovery contract.
pub const M5_GIT_CERTIFICATION_STASH_RECOVERY_CONTRACT_REF: &str =
    "schemas/git/stash-recovery.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_GIT_CERTIFICATION_FIXTURE_DIR: &str = "fixtures/git/m5/certification-corpus";

/// Repo-relative path of the checked support-export artifact.
pub const M5_GIT_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/git/m5/certify_m5_git_topology_history_recovery_and_provider_parity_rows/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_GIT_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/git/m5/certify_m5_git_topology_history_recovery_and_provider_parity_rows.md";

/// A claimed M5 Git or source-acquisition surface that must be certified.
///
/// These are the rows that downstream product, docs/help, CLI, support,
/// evaluation, and release/public-truth surfaces advertise; certification
/// decides whether each may keep its published claim or must narrow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GitClaimRow {
    /// Cloning, opening, and initializing/hydrating a repository's topology.
    SourceAcquisitionAndTopologyInitialization,
    /// Honest topology truth (sparse/partial/shallow/submodule/nested/LFS).
    RepositoryTopologyHonesty,
    /// Scoping operations to the correct worktree or root.
    WorktreeAndRootScoping,
    /// Topology honesty propagated into search, AI context, and review overlays.
    TopologyAwareSearchAiReviewParity,
    /// History rewrite preview and reachable recovery.
    HistorySurgeryPreviewAndRecovery,
    /// Stash, reflog, and checkpoint restore depth.
    StashReflogCheckpointRecovery,
    /// Conflict-resolution session continuity across reopen and restart.
    ConflictResolutionContinuity,
    /// Publish / ref-update with provider-degraded local continuity.
    PublishAndProviderParity,
}

impl M5GitClaimRow {
    /// Every claimed row, in canonical declaration order.
    pub const ALL: [Self; 8] = [
        Self::SourceAcquisitionAndTopologyInitialization,
        Self::RepositoryTopologyHonesty,
        Self::WorktreeAndRootScoping,
        Self::TopologyAwareSearchAiReviewParity,
        Self::HistorySurgeryPreviewAndRecovery,
        Self::StashReflogCheckpointRecovery,
        Self::ConflictResolutionContinuity,
        Self::PublishAndProviderParity,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceAcquisitionAndTopologyInitialization => {
                "source_acquisition_and_topology_initialization"
            }
            Self::RepositoryTopologyHonesty => "repository_topology_honesty",
            Self::WorktreeAndRootScoping => "worktree_and_root_scoping",
            Self::TopologyAwareSearchAiReviewParity => "topology_aware_search_ai_review_parity",
            Self::HistorySurgeryPreviewAndRecovery => "history_surgery_preview_and_recovery",
            Self::StashReflogCheckpointRecovery => "stash_reflog_checkpoint_recovery",
            Self::ConflictResolutionContinuity => "conflict_resolution_continuity",
            Self::PublishAndProviderParity => "publish_and_provider_parity",
        }
    }

    /// Whether this row performs a history rewrite (and so the
    /// [`CertificationDimension::HistorySurgeryPreviewRecovery`] dimension is
    /// applicable rather than not-applicable).
    pub const fn rewrites_history(self) -> bool {
        matches!(
            self,
            Self::HistorySurgeryPreviewAndRecovery
                | Self::StashReflogCheckpointRecovery
                | Self::ConflictResolutionContinuity
                | Self::PublishAndProviderParity
        )
    }
}

/// One of the four dimensions every claimed row must prove to stay certified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDimension {
    /// Current topology is reported honestly, never overclaiming coverage.
    TopologyHonesty,
    /// Operations are scoped to the correct worktree or root.
    WorktreeRootScoping,
    /// History rewrites are previewed and remain recoverable.
    HistorySurgeryPreviewRecovery,
    /// Local Git truth stays authoritative under a degraded/absent provider.
    LocalProviderParity,
}

impl CertificationDimension {
    /// Every dimension, in canonical declaration order.
    pub const ALL: [Self; 4] = [
        Self::TopologyHonesty,
        Self::WorktreeRootScoping,
        Self::HistorySurgeryPreviewRecovery,
        Self::LocalProviderParity,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyHonesty => "topology_honesty",
            Self::WorktreeRootScoping => "worktree_root_scoping",
            Self::HistorySurgeryPreviewRecovery => "history_surgery_preview_recovery",
            Self::LocalProviderParity => "local_provider_parity",
        }
    }
}

/// Freshness of the evidence backing a single certification dimension.
///
/// Freshness is distinct from the proof state: evidence can be conclusively
/// [`DimensionProofState::Proven`] yet [`EvidenceFreshness::Stale`], which still
/// narrows the row because the proof must be re-run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// Evidence is fresh within the review SLO window.
    Current,
    /// Evidence exists but is past the freshness window and must be re-run.
    Stale,
    /// No evidence is captured for this dimension.
    Missing,
}

impl EvidenceFreshness {
    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }
}

/// Proof state asserted by the evidence backing a certification dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DimensionProofState {
    /// Evidence shows the dimension fully holds.
    Proven,
    /// Evidence shows the dimension holds only partially and narrows the claim.
    Narrowed,
    /// Evidence shows the dimension does not hold (an overclaim or gap).
    Failed,
    /// The proof has not been run for the current corpus.
    NotRun,
}

impl DimensionProofState {
    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proven => "proven",
            Self::Narrowed => "narrowed",
            Self::Failed => "failed",
            Self::NotRun => "not_run",
        }
    }
}

/// Certification verdict for a row, derived fail-closed from its dimensions.
///
/// The variants are ordered by severity: [`Self::Certified`] is the only verdict
/// that permits the full published claim; the rest narrow it. The ordering is
/// used by [`Self::worse_of`] so the worst dimension contribution wins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationVerdict {
    /// Every applicable dimension is current and proven; the claim stands.
    Certified,
    /// An applicable dimension is honestly partial; the claim is narrowed.
    Limited,
    /// An applicable dimension is stale or unrun; the claim awaits a retest.
    RetestPending,
    /// An applicable dimension failed or its evidence is missing; unsupported.
    Unsupported,
}

impl CertificationVerdict {
    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Limited => "limited",
            Self::RetestPending => "retest_pending",
            Self::Unsupported => "unsupported",
        }
    }

    /// Severity rank used to fold dimension contributions (worst wins).
    const fn severity(self) -> u8 {
        match self {
            Self::Certified => 0,
            Self::Limited => 1,
            Self::RetestPending => 2,
            Self::Unsupported => 3,
        }
    }

    /// Returns the more severe (narrower) of two verdicts.
    pub fn worse_of(self, other: Self) -> Self {
        if other.severity() > self.severity() {
            other
        } else {
            self
        }
    }

    /// Whether this verdict permits the full published claim to stand.
    pub const fn permits_full_claim(self) -> bool {
        matches!(self, Self::Certified)
    }

    /// Whether this verdict narrows the published claim.
    pub const fn is_narrowed(self) -> bool {
        !self.permits_full_claim()
    }
}

/// Consumer surface that must reflect the same row verdicts as the register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationConsumerSurface {
    /// In-product Git/source-acquisition surfaces.
    Product,
    /// Docs and in-app help content.
    DocsHelp,
    /// CLI / headless replay or JSON output.
    Cli,
    /// Redaction-safe support / export packets.
    SupportExport,
    /// Evaluation / pilot packs.
    EvaluationPack,
    /// Claim-publication manifests.
    ClaimPublicationManifest,
    /// Release notes and public-truth marketing surfaces.
    ReleasePublicTruth,
}

impl CertificationConsumerSurface {
    /// Every surface, in canonical declaration order.
    pub const ALL: [Self; 7] = [
        Self::Product,
        Self::DocsHelp,
        Self::Cli,
        Self::SupportExport,
        Self::EvaluationPack,
        Self::ClaimPublicationManifest,
        Self::ReleasePublicTruth,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Product => "product",
            Self::DocsHelp => "docs_help",
            Self::Cli => "cli",
            Self::SupportExport => "support_export",
            Self::EvaluationPack => "evaluation_pack",
            Self::ClaimPublicationManifest => "claim_publication_manifest",
            Self::ReleasePublicTruth => "release_public_truth",
        }
    }
}

/// One dimension's qualification within a certification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionQualification {
    /// Dimension this entry qualifies.
    pub dimension: CertificationDimension,
    /// Whether this dimension applies to the row (skipped if not).
    pub applicable: bool,
    /// Freshness of the backing evidence.
    pub freshness: EvidenceFreshness,
    /// Proof state asserted by the backing evidence.
    pub proof_state: DimensionProofState,
    /// Repo-relative refs to the artifacts/fixtures proving this dimension.
    pub evidence_refs: Vec<String>,
    /// Human-readable summary of what the evidence shows.
    pub summary: String,
}

impl DimensionQualification {
    /// Whether this dimension is currently satisfied (or not applicable).
    pub fn is_satisfied(&self) -> bool {
        !self.applicable
            || (self.freshness == EvidenceFreshness::Current
                && self.proof_state == DimensionProofState::Proven)
    }

    /// Verdict this dimension contributes when applicable.
    ///
    /// This is the heart of the downgrade automation: failed/missing evidence
    /// drives [`CertificationVerdict::Unsupported`], stale/unrun evidence drives
    /// [`CertificationVerdict::RetestPending`], honestly partial evidence drives
    /// [`CertificationVerdict::Limited`], and only current-and-proven evidence
    /// keeps [`CertificationVerdict::Certified`].
    pub fn verdict_contribution(&self) -> Option<CertificationVerdict> {
        if !self.applicable {
            return None;
        }
        let verdict = match (self.proof_state, self.freshness) {
            (DimensionProofState::Failed, _) => CertificationVerdict::Unsupported,
            (_, EvidenceFreshness::Missing) => CertificationVerdict::Unsupported,
            (DimensionProofState::NotRun, _) => CertificationVerdict::RetestPending,
            (_, EvidenceFreshness::Stale) => CertificationVerdict::RetestPending,
            (DimensionProofState::Narrowed, EvidenceFreshness::Current) => {
                CertificationVerdict::Limited
            }
            (DimensionProofState::Proven, EvidenceFreshness::Current) => {
                CertificationVerdict::Certified
            }
        };
        Some(verdict)
    }
}

/// One claimed M5 Git row in the certification register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GitCertificationRow {
    /// Claimed surface governed by this row.
    pub claim_row: M5GitClaimRow,
    /// Human-readable row label.
    pub row_label: String,
    /// The published claim this row certifies (or narrows).
    pub published_claim: String,
    /// Per-dimension qualifications (one entry per [`CertificationDimension`]).
    pub dimensions: Vec<DimensionQualification>,
    /// Declared verdict; must equal [`Self::derive_verdict`].
    pub verdict: CertificationVerdict,
    /// Reason the claim is narrowed; required when the verdict is not certified.
    pub narrowing_reason: Option<String>,
    /// Surfaces that must reflect this row's verdict.
    pub consumer_surfaces: Vec<CertificationConsumerSurface>,
}

impl M5GitCertificationRow {
    /// Derives the fail-closed verdict from this row's dimensions.
    ///
    /// Folds the contribution of every applicable dimension, keeping the worst.
    /// A row with no applicable dimension is [`CertificationVerdict::Unsupported`]
    /// because it certifies nothing.
    pub fn derive_verdict(&self) -> CertificationVerdict {
        let mut verdict: Option<CertificationVerdict> = None;
        for dimension in &self.dimensions {
            if let Some(contribution) = dimension.verdict_contribution() {
                verdict = Some(match verdict {
                    Some(current) => current.worse_of(contribution),
                    None => contribution,
                });
            }
        }
        verdict.unwrap_or(CertificationVerdict::Unsupported)
    }

    /// Whether the declared verdict matches the derived (evidence) verdict.
    pub fn verdict_matches_evidence(&self) -> bool {
        self.verdict == self.derive_verdict()
    }

    /// Whether every required dimension is present exactly once.
    fn has_all_dimensions(&self) -> bool {
        let present: BTreeSet<CertificationDimension> = self
            .dimensions
            .iter()
            .map(|entry| entry.dimension)
            .collect();
        present.len() == self.dimensions.len()
            && CertificationDimension::ALL
                .iter()
                .all(|dimension| present.contains(dimension))
    }
}

/// Parity audit proving every surface reflects the same row verdicts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationParityAudit {
    /// Product surfaces reflect the row verdicts.
    pub product_reflects_row_verdicts: bool,
    /// Docs/help reflect the row verdicts.
    pub docs_help_reflects_row_verdicts: bool,
    /// CLI/headless reflects the row verdicts.
    pub cli_reflects_row_verdicts: bool,
    /// Support export reflects the row verdicts.
    pub support_export_reflects_row_verdicts: bool,
    /// Evaluation packs reflect the row verdicts.
    pub evaluation_packs_reflect_row_verdicts: bool,
    /// Claim-publication manifests reflect the row verdicts.
    pub claim_publication_manifests_reflect_row_verdicts: bool,
    /// Release/public-truth surfaces reflect the row verdicts.
    pub release_public_truth_reflects_row_verdicts: bool,
    /// No surface advertises wider than the current machine-readable row.
    pub no_surface_claims_wider_than_row: bool,
    /// Local Git truth is authoritative over any provider overlay.
    pub local_truth_authoritative_over_provider: bool,
}

impl CertificationParityAudit {
    /// Whether every parity invariant holds.
    pub fn is_complete(&self) -> bool {
        self.product_reflects_row_verdicts
            && self.docs_help_reflects_row_verdicts
            && self.cli_reflects_row_verdicts
            && self.support_export_reflects_row_verdicts
            && self.evaluation_packs_reflect_row_verdicts
            && self.claim_publication_manifests_reflect_row_verdicts
            && self.release_public_truth_reflects_row_verdicts
            && self.no_surface_claims_wider_than_row
            && self.local_truth_authoritative_over_provider
    }
}

/// Downgrade-automation contract bound to the register's derivation semantics.
///
/// The narrowing targets must match the fail-closed mapping implemented by
/// [`DimensionQualification::verdict_contribution`], so the declared automation
/// can never drift from the behavior the register actually enforces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradeAutomation {
    /// Stale evidence automatically narrows the affected row.
    pub auto_narrow_on_stale: bool,
    /// Verdict a stale or unrun dimension narrows to (must be retest-pending).
    pub stale_or_unrun_narrows_to: CertificationVerdict,
    /// Verdict an honestly partial dimension narrows to (must be limited).
    pub partial_narrows_to: CertificationVerdict,
    /// Verdict a failed or missing dimension narrows to (must be unsupported).
    pub failure_or_missing_narrows_to: CertificationVerdict,
    /// Narrowing propagates into docs/help.
    pub propagates_to_docs_help: bool,
    /// Narrowing propagates into support packets.
    pub propagates_to_support_packets: bool,
    /// Narrowing propagates into evaluation packs.
    pub propagates_to_evaluation_packs: bool,
    /// Narrowing propagates into claim-publication manifests.
    pub propagates_to_claim_publication_manifests: bool,
    /// Release/public-truth surfaces stop overclaiming when a row slips.
    pub release_surface_stops_overclaiming_on_slip: bool,
}

impl DowngradeAutomation {
    /// Whether the automation matches the register's fail-closed semantics.
    pub fn is_consistent(&self) -> bool {
        self.auto_narrow_on_stale
            && self.stale_or_unrun_narrows_to == CertificationVerdict::RetestPending
            && self.partial_narrows_to == CertificationVerdict::Limited
            && self.failure_or_missing_narrows_to == CertificationVerdict::Unsupported
            && self.propagates_to_docs_help
            && self.propagates_to_support_packets
            && self.propagates_to_evaluation_packs
            && self.propagates_to_claim_publication_manifests
            && self.release_surface_stops_overclaiming_on_slip
    }
}

/// Governance review block proving the register controls claims, not badges.
///
/// These are enforcement statements about the mechanism, not assertions about
/// any single row's current state: they hold for the canonical packet and for a
/// degraded packet in which one row has narrowed. The per-row truth lives in the
/// rows and their derived verdicts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationGovernanceReview {
    /// Every claimed row is required to carry current topology and recovery
    /// evidence before it may be certified.
    pub requires_current_topology_and_recovery_evidence_per_row: bool,
    /// Stale or underqualified rows narrow automatically.
    pub stale_or_underqualified_rows_narrow_automatically: bool,
    /// Release/public-truth surfaces stop overclaiming when parity slips.
    pub release_surfaces_stop_overclaiming_on_slip: bool,
    /// Claim truth is not left manual.
    pub claim_truth_is_not_manual: bool,
    /// The register fails closed to retest/limited/unsupported.
    pub fails_closed_to_retest_limited_or_unsupported: bool,
    /// Provider-degraded local continuity is required to certify a row.
    pub provider_degraded_local_continuity_required: bool,
    /// One certification register is shared across all surfaces.
    pub one_certification_register_across_surfaces: bool,
    /// No claim is broadened beyond what the proof packet sustains.
    pub no_claim_broadened_beyond_proof_packet: bool,
}

impl CertificationGovernanceReview {
    /// Whether every governance invariant holds.
    pub fn is_complete(&self) -> bool {
        self.requires_current_topology_and_recovery_evidence_per_row
            && self.stale_or_underqualified_rows_narrow_automatically
            && self.release_surfaces_stop_overclaiming_on_slip
            && self.claim_truth_is_not_manual
            && self.fails_closed_to_retest_limited_or_unsupported
            && self.provider_degraded_local_continuity_required
            && self.one_certification_register_across_surfaces
            && self.no_claim_broadened_beyond_proof_packet
    }
}

/// Freshness posture for the certification register as a whole.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationFreshnessPosture {
    /// Review SLO in hours.
    pub review_slo_hours: u32,
    /// RFC 3339 timestamp of the last review.
    pub last_reviewed_at: String,
    /// True when stale evidence automatically narrows claims.
    pub auto_narrow_on_stale: bool,
    /// True while the evidence validity window is open.
    pub evidence_window_open: bool,
}

impl CertificationFreshnessPosture {
    /// Whether the posture block is complete and active.
    pub fn is_complete(&self) -> bool {
        self.review_slo_hours > 0
            && !self.last_reviewed_at.trim().is_empty()
            && self.auto_narrow_on_stale
            && self.evidence_window_open
    }
}

/// Constructor input for [`M5GitCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5GitCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certification rows.
    pub rows: Vec<M5GitCertificationRow>,
    /// Parity audit block.
    pub parity_audit: CertificationParityAudit,
    /// Downgrade automation block.
    pub downgrade_automation: DowngradeAutomation,
    /// Governance review block.
    pub governance_review: CertificationGovernanceReview,
    /// Freshness posture block.
    pub freshness_posture: CertificationFreshnessPosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 Git certification register packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GitCertificationPacket {
    /// Record kind; must equal [`M5_GIT_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_GIT_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certification rows.
    pub rows: Vec<M5GitCertificationRow>,
    /// Parity audit block.
    pub parity_audit: CertificationParityAudit,
    /// Downgrade automation block.
    pub downgrade_automation: DowngradeAutomation,
    /// Governance review block.
    pub governance_review: CertificationGovernanceReview,
    /// Freshness posture block.
    pub freshness_posture: CertificationFreshnessPosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5GitCertificationPacket {
    /// Builds a certification packet from frozen input.
    pub fn new(input: M5GitCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_GIT_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_GIT_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            rows: input.rows,
            parity_audit: input.parity_audit,
            downgrade_automation: input.downgrade_automation,
            governance_review: input.governance_review,
            freshness_posture: input.freshness_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the register invariants.
    ///
    /// Returns every violation found rather than stopping at the first, so a
    /// regenerator or CI gate can report the full set at once.
    pub fn validate(&self) -> Vec<M5GitCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_GIT_CERTIFICATION_RECORD_KIND {
            violations.push(M5GitCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_GIT_CERTIFICATION_SCHEMA_VERSION {
            violations.push(M5GitCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5GitCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.parity_audit.is_complete() {
            violations.push(M5GitCertificationViolation::ParityAuditIncomplete);
        }
        if !self.downgrade_automation.is_consistent() {
            violations.push(M5GitCertificationViolation::DowngradeAutomationInconsistent);
        }
        if !self.governance_review.is_complete() {
            violations.push(M5GitCertificationViolation::GovernanceReviewIncomplete);
        }
        if !self.freshness_posture.is_complete() {
            violations.push(M5GitCertificationViolation::FreshnessPostureIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 git certification packet serializes"),
        ) {
            violations.push(M5GitCertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Rows whose claim currently stands certified.
    pub fn certified_rows(&self) -> Vec<&M5GitCertificationRow> {
        self.rows
            .iter()
            .filter(|row| row.verdict.permits_full_claim())
            .collect()
    }

    /// Rows whose claim is currently narrowed.
    pub fn narrowed_rows(&self) -> Vec<&M5GitCertificationRow> {
        self.rows
            .iter()
            .filter(|row| row.verdict.is_narrowed())
            .collect()
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 git certification packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Git Certification Register\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Review SLO: {} hours (last reviewed: {}, window open: {})\n",
            self.freshness_posture.review_slo_hours,
            self.freshness_posture.last_reviewed_at,
            self.freshness_posture.evidence_window_open,
        ));
        out.push_str(&format!(
            "- Rows: {} total / {} certified / {} narrowed\n",
            self.rows.len(),
            self.certified_rows().len(),
            self.narrowed_rows().len(),
        ));

        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: verdict `{}`",
                row.claim_row.as_str(),
                row.verdict.as_str(),
            ));
            if let Some(reason) = &row.narrowing_reason {
                out.push_str(&format!(" — {reason}"));
            }
            out.push('\n');
            for dimension in &row.dimensions {
                if !dimension.applicable {
                    continue;
                }
                out.push_str(&format!(
                    "  - {}: freshness `{}`, proof `{}`\n",
                    dimension.dimension.as_str(),
                    dimension.freshness.as_str(),
                    dimension.proof_state.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum M5GitCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5GitCertificationViolation>),
}

impl fmt::Display for M5GitCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 git certification export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 git certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5GitCertificationArtifactError {}

/// Validation failures emitted by [`M5GitCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5GitCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required claim row is missing from the register.
    RequiredClaimRowMissing,
    /// A claim row is listed more than once.
    DuplicateClaimRow,
    /// A row does not carry exactly one entry per certification dimension.
    RowMissingDimensions,
    /// A row has no applicable dimension and so certifies nothing.
    RowHasNoApplicableDimension,
    /// A dimension entry is incomplete (empty summary).
    DimensionEntryIncomplete,
    /// A proven or narrowed dimension carries no evidence refs.
    ProvenDimensionMissingEvidence,
    /// A row's declared verdict does not match its evidence.
    VerdictDoesNotMatchEvidence,
    /// A narrowed row does not name a narrowing reason.
    NarrowedRowMissingReason,
    /// A certified row redundantly names a narrowing reason.
    CertifiedRowHasNarrowingReason,
    /// A row does not list any consumer surface.
    RowMissingConsumerSurfaces,
    /// History rewrite applicability does not match the row's nature.
    HistoryDimensionApplicabilityMismatch,
    /// Parity audit does not satisfy required invariants.
    ParityAuditIncomplete,
    /// Downgrade automation is inconsistent with the register semantics.
    DowngradeAutomationInconsistent,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Freshness posture block is incomplete.
    FreshnessPostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5GitCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredClaimRowMissing => "required_claim_row_missing",
            Self::DuplicateClaimRow => "duplicate_claim_row",
            Self::RowMissingDimensions => "row_missing_dimensions",
            Self::RowHasNoApplicableDimension => "row_has_no_applicable_dimension",
            Self::DimensionEntryIncomplete => "dimension_entry_incomplete",
            Self::ProvenDimensionMissingEvidence => "proven_dimension_missing_evidence",
            Self::VerdictDoesNotMatchEvidence => "verdict_does_not_match_evidence",
            Self::NarrowedRowMissingReason => "narrowed_row_missing_reason",
            Self::CertifiedRowHasNarrowingReason => "certified_row_has_narrowing_reason",
            Self::RowMissingConsumerSurfaces => "row_missing_consumer_surfaces",
            Self::HistoryDimensionApplicabilityMismatch => {
                "history_dimension_applicability_mismatch"
            }
            Self::ParityAuditIncomplete => "parity_audit_incomplete",
            Self::DowngradeAutomationInconsistent => "downgrade_automation_inconsistent",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::FreshnessPostureIncomplete => "freshness_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable certification export.
///
/// # Errors
///
/// Returns [`M5GitCertificationArtifactError`] when the checked-in export fails
/// to parse or violates the certification contract.
pub fn current_m5_git_certification_export(
) -> Result<M5GitCertificationPacket, M5GitCertificationArtifactError> {
    let packet: M5GitCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/git/m5/certify_m5_git_topology_history_recovery_and_provider_parity_rows/support_export.json"
    )))
    .map_err(M5GitCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5GitCertificationArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5GitCertificationPacket,
    violations: &mut Vec<M5GitCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_GIT_CERTIFICATION_SCHEMA_REF,
        M5_GIT_CERTIFICATION_DOC_REF,
        M5_GIT_CERTIFICATION_MATRIX_CONTRACT_REF,
        M5_GIT_CERTIFICATION_TOPOLOGY_CONTRACT_REF,
        M5_GIT_CERTIFICATION_TOPOLOGY_ACTION_CONTRACT_REF,
        M5_GIT_CERTIFICATION_HISTORY_SURGERY_CONTRACT_REF,
        M5_GIT_CERTIFICATION_STASH_RECOVERY_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5GitCertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &M5GitCertificationPacket,
    violations: &mut Vec<M5GitCertificationViolation>,
) {
    let mut seen: BTreeSet<M5GitClaimRow> = BTreeSet::new();
    for row in &packet.rows {
        if !seen.insert(row.claim_row) {
            violations.push(M5GitCertificationViolation::DuplicateClaimRow);
        }
        validate_single_row(row, violations);
    }
    for required in M5GitClaimRow::ALL {
        if !seen.contains(&required) {
            violations.push(M5GitCertificationViolation::RequiredClaimRowMissing);
            return;
        }
    }
}

fn validate_single_row(
    row: &M5GitCertificationRow,
    violations: &mut Vec<M5GitCertificationViolation>,
) {
    if !row.has_all_dimensions() {
        violations.push(M5GitCertificationViolation::RowMissingDimensions);
    }
    if row.consumer_surfaces.is_empty() {
        violations.push(M5GitCertificationViolation::RowMissingConsumerSurfaces);
    }

    let mut any_applicable = false;
    for dimension in &row.dimensions {
        if dimension.applicable {
            any_applicable = true;
        }
        if dimension.summary.trim().is_empty() {
            violations.push(M5GitCertificationViolation::DimensionEntryIncomplete);
        }
        let asserts_proof = matches!(
            dimension.proof_state,
            DimensionProofState::Proven | DimensionProofState::Narrowed
        );
        if dimension.applicable && asserts_proof && dimension.evidence_refs.is_empty() {
            violations.push(M5GitCertificationViolation::ProvenDimensionMissingEvidence);
        }
        if dimension.dimension == CertificationDimension::HistorySurgeryPreviewRecovery
            && dimension.applicable != row.claim_row.rewrites_history()
        {
            violations.push(M5GitCertificationViolation::HistoryDimensionApplicabilityMismatch);
        }
    }
    if !any_applicable {
        violations.push(M5GitCertificationViolation::RowHasNoApplicableDimension);
    }

    if !row.verdict_matches_evidence() {
        violations.push(M5GitCertificationViolation::VerdictDoesNotMatchEvidence);
    }
    match (row.verdict.is_narrowed(), &row.narrowing_reason) {
        (true, None) => violations.push(M5GitCertificationViolation::NarrowedRowMissingReason),
        (true, Some(reason)) if reason.trim().is_empty() => {
            violations.push(M5GitCertificationViolation::NarrowedRowMissingReason);
        }
        (false, Some(_)) => {
            violations.push(M5GitCertificationViolation::CertifiedRowHasNarrowingReason);
        }
        _ => {}
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => {
            let lower = text.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
