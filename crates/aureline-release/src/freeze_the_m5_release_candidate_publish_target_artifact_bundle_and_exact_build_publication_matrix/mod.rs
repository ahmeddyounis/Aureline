//! Typed M5 release-candidate, publish-target, artifact-bundle, and exact-build
//! publication matrix.
//!
//! This module freezes the canonical publication matrix that maps every new M5
//! artifact family to the release-control truth needed to publish it as one
//! inspectable artifact graph rather than opaque CI output. Each
//! [`M5PublicationMatrixRow`] binds one M5 artifact family
//! ([`M5ArtifactFamilyKind`]) to:
//!
//! - the release candidate it ships under ([`M5PublicationMatrixRow::release_candidate_ref`])
//!   and the scoped publish target class it publishes to
//!   ([`crate::release_center_model::PublishTargetClass`]),
//! - its exact-build identity ([`ExactBuildIdentity`]): the one-build identity
//!   and provenance refs, signature state, attestation availability, SBOM scope,
//!   symbol/source-map availability, mirror freshness, rollback target, and
//!   evidence completeness,
//! - its rollback/revocation posture ([`RollbackRevocationPosture`]) and its
//!   mirror/offline publication expectation ([`MirrorOfflineExpectation`]),
//! - the required evidence and its freshness SLO ([`ProofPacket`]) and owner
//!   sign-off ([`OwnerSignoff`]),
//! - the public claim it backs, the active gap reasons
//!   ([`M5PublicationGapReason`]) narrowing it, and the effective label it
//!   carries after narrowing ([`M5PublicationMatrixRow::published_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the
//! boundary between an artifact family that may publish as Stable and one that
//! must narrow below it. The [`M5PublicationStopRule`] set names the closed
//! conditions that gate publication when exact-build linkage breaks, signatures
//! or attestations are missing, the SBOM or symbols are incomplete, a mirror is
//! stale, a rollback target is missing, evidence is incomplete, a proof packet
//! aged out, a waiver expired, or owner sign-off is missing.
//! [`M5PublicationMatrix::publication`] records the resulting proceed/hold
//! verdict, computed only from rows whose public claim is still at or above the
//! cutline — a family whose claim is already narrowed inherits that ceiling
//! rather than blocking the whole train.
//!
//! Build success is never treated as publication readiness: a row only holds its
//! claimed label when its exact-build linkage is intact, its proof packet is
//! within its freshness SLO, and it is owner-signed. Any row whose exact-build
//! linkage is stale or broken, or whose evidence is missing or stale, narrows
//! below the cutline before promotion.
//!
//! The matrix is checked in at
//! `artifacts/release/m5/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, SBOM bodies, or credential
//! material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::release_center_model::{
    BlastRadiusClass, PublishTargetClass, RollbackOrRevocationKind, SignatureStateClass,
};
use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

/// Supported matrix schema version.
pub const FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the matrix.
pub const FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_RECORD_KIND: &str =
    "freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix";

/// Repo-relative path to the checked-in matrix.
pub const FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_PATH: &str =
    "artifacts/release/m5/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix.json";

/// Embedded checked-in matrix JSON.
pub const FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_JSON: &str =
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix.json"
    ));

/// The new M5 artifact families this matrix governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ArtifactFamilyKind {
    /// Notebook packs and notebook-derived outputs.
    NotebookPack,
    /// Request/data assets (saved requests, datasets, fixtures).
    RequestDataAsset,
    /// Profiler and replay artifacts (traces, recordings).
    ProfilerReplayArtifact,
    /// Framework and template packs.
    FrameworkTemplatePack,
    /// Documentation packs.
    DocsPack,
    /// Model packs (local model bundles and metadata).
    ModelPack,
    /// Companion and offboarding packets.
    CompanionOffboardingPacket,
    /// Managed outputs produced by managed/tenant-scoped lanes.
    ManagedOutput,
}

impl M5ArtifactFamilyKind {
    /// Every family kind, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NotebookPack,
        Self::RequestDataAsset,
        Self::ProfilerReplayArtifact,
        Self::FrameworkTemplatePack,
        Self::DocsPack,
        Self::ModelPack,
        Self::CompanionOffboardingPacket,
        Self::ManagedOutput,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookPack => "notebook_pack",
            Self::RequestDataAsset => "request_data_asset",
            Self::ProfilerReplayArtifact => "profiler_replay_artifact",
            Self::FrameworkTemplatePack => "framework_template_pack",
            Self::DocsPack => "docs_pack",
            Self::ModelPack => "model_pack",
            Self::CompanionOffboardingPacket => "companion_offboarding_packet",
            Self::ManagedOutput => "managed_output",
        }
    }
}

/// Attestation availability for an artifact family's exact build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttestationAvailability {
    /// A signed build attestation is published and verifiable.
    Attested,
    /// Attestation is expected but the release signature is still pending.
    PendingAttestation,
    /// No attestation is available.
    Unattested,
    /// Attestation does not apply to this family.
    NotApplicable,
}

impl AttestationAvailability {
    /// Every value, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Attested,
        Self::PendingAttestation,
        Self::Unattested,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attested => "attested",
            Self::PendingAttestation => "pending_attestation",
            Self::Unattested => "unattested",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether the value lets a row hold a stable claim.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Attested | Self::NotApplicable)
    }
}

/// SBOM scope for an artifact family's exact build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SbomScope {
    /// A full dependency-graph SBOM is published.
    FullGraph,
    /// A component-scoped SBOM (direct components only) is published.
    ComponentScoped,
    /// Only a partial SBOM exists.
    Partial,
    /// No SBOM is available.
    Missing,
    /// SBOM does not apply to this family.
    NotApplicable,
}

impl SbomScope {
    /// Every value, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullGraph,
        Self::ComponentScoped,
        Self::Partial,
        Self::Missing,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullGraph => "full_graph",
            Self::ComponentScoped => "component_scoped",
            Self::Partial => "partial",
            Self::Missing => "missing",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether the value lets a row hold a stable claim.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::FullGraph | Self::ComponentScoped | Self::NotApplicable)
    }
}

/// Symbol and source-map availability for an artifact family's exact build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolSourceMapAvailability {
    /// Symbols/source maps are published alongside the artifact.
    Published,
    /// Symbols/source maps are retained internally for symbolication.
    RetainedInternal,
    /// Symbols/source maps were stripped and are not recoverable.
    Stripped,
    /// Symbols/source maps are expected but missing.
    Missing,
    /// Symbols/source maps do not apply to this family.
    NotApplicable,
}

impl SymbolSourceMapAvailability {
    /// Every value, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Published,
        Self::RetainedInternal,
        Self::Stripped,
        Self::Missing,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::RetainedInternal => "retained_internal",
            Self::Stripped => "stripped",
            Self::Missing => "missing",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether the value lets a row hold a stable claim.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Published | Self::RetainedInternal | Self::NotApplicable)
    }
}

/// Mirror freshness for an artifact family's published copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorFreshness {
    /// The mirror copy matches the canonical artifact graph.
    Current,
    /// The mirror copy lags the canonical artifact graph.
    Stale,
    /// The artifact has not yet been mirrored.
    Unpublished,
    /// Mirroring does not apply to this family.
    NotApplicable,
}

impl MirrorFreshness {
    /// Every value, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Current,
        Self::Stale,
        Self::Unpublished,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Unpublished => "unpublished",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether the value lets a row hold a stable claim.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Current | Self::NotApplicable)
    }
}

/// Evidence completeness for an artifact family's release evidence set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceCompleteness {
    /// Every required evidence ref is present.
    Complete,
    /// Some required evidence refs are missing.
    Partial,
    /// No required evidence is present.
    Missing,
}

impl EvidenceCompleteness {
    /// Every value, in declaration order.
    pub const ALL: [Self; 3] = [Self::Complete, Self::Partial, Self::Missing];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Missing => "missing",
        }
    }

    /// Whether the value lets a row hold a stable claim.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Closed reason an M5 publication row narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationGapReason {
    /// The release signature is missing, unverified, pending, or revoked.
    SignatureMissing,
    /// A build attestation is missing or pending.
    AttestationMissing,
    /// The SBOM is partial or missing.
    SbomIncomplete,
    /// Symbols/source maps were stripped or are missing.
    SymbolsMissing,
    /// The mirror copy is stale or unpublished.
    MirrorStale,
    /// No rollback target (last-known-good) is recorded.
    RollbackTargetMissing,
    /// The exact-build identity does not link the published artifact to a rebuild.
    ExactBuildLinkageBroken,
    /// The required evidence set is incomplete.
    EvidenceIncomplete,
    /// The proof packet is missing.
    ProofPacketMissing,
    /// The proof packet is stale.
    ProofPacketStale,
    /// A waiver the row relied on has expired.
    WaiverExpired,
    /// Required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl M5PublicationGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::SignatureMissing,
        Self::AttestationMissing,
        Self::SbomIncomplete,
        Self::SymbolsMissing,
        Self::MirrorStale,
        Self::RollbackTargetMissing,
        Self::ExactBuildLinkageBroken,
        Self::EvidenceIncomplete,
        Self::ProofPacketMissing,
        Self::ProofPacketStale,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignatureMissing => "signature_missing",
            Self::AttestationMissing => "attestation_missing",
            Self::SbomIncomplete => "sbom_incomplete",
            Self::SymbolsMissing => "symbols_missing",
            Self::MirrorStale => "mirror_stale",
            Self::RollbackTargetMissing => "rollback_target_missing",
            Self::ExactBuildLinkageBroken => "exact_build_linkage_broken",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublicationAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the public claim below the cutline.
    NarrowClaim,
    /// Re-sign the release artifact.
    ReSignArtifact,
    /// Re-attest the build.
    ReAttest,
    /// Regenerate the SBOM.
    RegenerateSbom,
    /// Publish the symbols/source maps.
    PublishSymbols,
    /// Refresh the mirror copy.
    RefreshMirror,
    /// Record the rollback target (last-known-good).
    RecordRollbackTarget,
    /// Rebuild to restore exact-build linkage.
    RebuildExactBuild,
    /// Recapture the evidence set.
    RecaptureEvidence,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl M5PublicationAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::HoldPublication,
        Self::NarrowClaim,
        Self::ReSignArtifact,
        Self::ReAttest,
        Self::RegenerateSbom,
        Self::PublishSymbols,
        Self::RefreshMirror,
        Self::RecordRollbackTarget,
        Self::RebuildExactBuild,
        Self::RecaptureEvidence,
        Self::RefreshProofPacket,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowClaim => "narrow_claim",
            Self::ReSignArtifact => "re_sign_artifact",
            Self::ReAttest => "re_attest",
            Self::RegenerateSbom => "regenerate_sbom",
            Self::PublishSymbols => "publish_symbols",
            Self::RefreshMirror => "refresh_mirror",
            Self::RecordRollbackTarget => "record_rollback_target",
            Self::RebuildExactBuild => "rebuild_exact_build",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// The exact-build identity for one artifact family.
///
/// This is the frozen exact-build vocabulary: the one-build identity and
/// provenance refs, plus the signature state, attestation availability, SBOM
/// scope, symbol/source-map availability, mirror freshness, rollback target, and
/// evidence completeness that decide whether the family is rebuildable,
/// identifiable, symbolicated, and revocable as one system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExactBuildIdentity {
    /// Ref to the one-build identity record (reproducible build identity).
    pub build_identity_ref: String,
    /// Ref to the provenance/attestation chain.
    pub provenance_ref: String,
    /// Release signature state.
    pub signature_state: SignatureStateClass,
    /// Attestation availability.
    pub attestation_availability: AttestationAvailability,
    /// SBOM scope.
    pub sbom_scope: SbomScope,
    /// Symbol/source-map availability.
    pub symbol_availability: SymbolSourceMapAvailability,
    /// Mirror freshness.
    pub mirror_freshness: MirrorFreshness,
    /// Ref to the rollback target (last-known-good). Empty only when narrowed.
    pub rollback_target_ref: String,
    /// Evidence completeness.
    pub evidence_completeness: EvidenceCompleteness,
}

impl ExactBuildIdentity {
    /// True when every exact-build field is intact enough to hold a stable claim:
    /// a verified signature, available attestation, in-scope SBOM, available
    /// symbols, current mirror, a recorded rollback target, and complete evidence.
    pub fn linkage_intact(&self) -> bool {
        self.signature_state == SignatureStateClass::Verified
            && self.attestation_availability.holds_label()
            && self.sbom_scope.holds_label()
            && self.symbol_availability.holds_label()
            && self.mirror_freshness.holds_label()
            && self.evidence_completeness.holds_label()
            && !self.rollback_target_ref.trim().is_empty()
    }

    /// The gap reasons that a broken exact-build field requires the row to name.
    ///
    /// Returns one reason per field that fails to hold its label, so the matrix
    /// can prove a narrowed row names every reason that forced it below the
    /// cutline.
    pub fn required_gap_reasons(&self) -> Vec<M5PublicationGapReason> {
        let mut reasons = Vec::new();
        if self.signature_state != SignatureStateClass::Verified {
            reasons.push(M5PublicationGapReason::SignatureMissing);
        }
        if !self.attestation_availability.holds_label() {
            reasons.push(M5PublicationGapReason::AttestationMissing);
        }
        if !self.sbom_scope.holds_label() {
            reasons.push(M5PublicationGapReason::SbomIncomplete);
        }
        if !self.symbol_availability.holds_label() {
            reasons.push(M5PublicationGapReason::SymbolsMissing);
        }
        if !self.mirror_freshness.holds_label() {
            reasons.push(M5PublicationGapReason::MirrorStale);
        }
        if self.evidence_completeness != EvidenceCompleteness::Complete {
            reasons.push(M5PublicationGapReason::EvidenceIncomplete);
        }
        if self.rollback_target_ref.trim().is_empty() {
            reasons.push(M5PublicationGapReason::RollbackTargetMissing);
        }
        reasons
    }
}

/// The rollback/revocation posture for one artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RollbackRevocationPosture {
    /// The rollback/revocation kind the family supports as its primary recovery.
    pub kind: RollbackOrRevocationKind,
    /// The blast radius a rollback/revocation of this family touches.
    pub blast_radius: BlastRadiusClass,
    /// Whether the family can be revoked once published.
    pub revocable: bool,
    /// Ref to the rollback/revocation posture record.
    pub posture_ref: String,
    /// Reviewable one-line statement of the posture.
    pub summary: String,
}

/// The mirror/offline publication expectation for one artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MirrorOfflineExpectation {
    /// Whether the family is expected to be published to a mirror feed.
    pub mirror_publish_expected: bool,
    /// Whether the family can be verified offline (without contacting the origin).
    pub offline_verifiable: bool,
    /// Ref to the mirror/offline parity record.
    pub parity_ref: String,
    /// Reviewable one-line statement of the expectation.
    pub summary: String,
}

/// One M5 publication stop rule: a closed condition that gates publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PublicationStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: M5PublicationGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: M5PublicationAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One M5 publication-matrix row: one artifact family's release-control truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PublicationMatrixRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The artifact family this row governs.
    pub family_kind: M5ArtifactFamilyKind,
    /// The artifact family subject ref this row speaks about.
    pub artifact_ref: String,
    /// Reviewable one-line statement of the artifact family.
    pub artifact_summary: String,
    /// Whether the family is part of the release-blocking set.
    pub release_blocking: bool,
    /// The release candidate this family publishes under (release-candidate scope).
    pub release_candidate_ref: String,
    /// The stable-claim-manifest entry id whose public claim this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// The scoped publish target class this family publishes to.
    pub publish_target_class: PublishTargetClass,
    /// The exact-build identity for this family.
    pub exact_build: ExactBuildIdentity,
    /// The rollback/revocation posture for this family.
    pub rollback_revocation: RollbackRevocationPosture,
    /// The mirror/offline publication expectation for this family.
    pub mirror_offline: MirrorOfflineExpectation,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<M5PublicationGapReason>,
    /// The lifecycle label the family effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl M5PublicationMatrixRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the exact-build linkage lets the family carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.exact_build.linkage_intact()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: M5PublicationGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// Summary counts carried by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PublicationMatrixSummary {
    /// Total number of family rows.
    pub total_entries: usize,
    /// Distinct release candidates covered.
    pub total_release_candidates: usize,
    /// Rows publishing a label at or above the cutline.
    pub entries_backed: usize,
    /// Rows narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Rows holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing a label at or above the cutline.
    pub release_blocking_backed: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Notebook-pack rows.
    pub notebook_pack_entries: usize,
    /// Request/data-asset rows.
    pub request_data_asset_entries: usize,
    /// Profiler/replay rows.
    pub profiler_replay_entries: usize,
    /// Framework/template rows.
    pub framework_template_entries: usize,
    /// Docs-pack rows.
    pub docs_pack_entries: usize,
    /// Model-pack rows.
    pub model_pack_entries: usize,
    /// Companion/offboarding rows.
    pub companion_offboarding_entries: usize,
    /// Managed-output rows.
    pub managed_output_entries: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Rows whose signature state is `verified`.
    pub signatures_verified: usize,
    /// Rows whose attestation is `attested`.
    pub attestations_attested: usize,
    /// Rows whose SBOM scope is full-graph or component-scoped.
    pub sbom_in_scope: usize,
    /// Rows whose symbols are published or retained internally.
    pub symbols_available: usize,
    /// Rows whose mirror copy is current.
    pub mirror_current: usize,
    /// Rows that recorded a rollback target (last-known-good).
    pub rollback_targets_recorded: usize,
    /// Rows expected to publish to a mirror feed.
    pub mirror_publish_expected: usize,
    /// Rows that are offline-verifiable.
    pub offline_verifiable: usize,
    /// Rows whose family is revocable once published.
    pub revocable_entries: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The artifact family this row governs.
    pub family_kind: M5ArtifactFamilyKind,
    /// The artifact family subject ref.
    pub artifact_ref: String,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The release candidate this family publishes under.
    pub release_candidate_ref: String,
    /// The scoped publish target class.
    pub publish_target_class: PublishTargetClass,
    /// The stable-claim-manifest entry id whose public claim this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the row publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Signature state.
    pub signature_state: SignatureStateClass,
    /// Attestation availability.
    pub attestation_availability: AttestationAvailability,
    /// SBOM scope.
    pub sbom_scope: SbomScope,
    /// Symbol/source-map availability.
    pub symbol_availability: SymbolSourceMapAvailability,
    /// Mirror freshness.
    pub mirror_freshness: MirrorFreshness,
    /// Whether the family is revocable once published.
    pub revocable: bool,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<M5PublicationGapReason>,
}

/// Export projection for Help/About, release-center, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationExportProjection {
    /// Matrix identifier.
    pub matrix_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Publication decision.
    pub publication_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5PublicationExportRow>,
}

/// The typed M5 release-candidate/publish-target/artifact-bundle/exact-build
/// publication matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PublicationMatrix {
    /// Matrix schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable matrix identifier.
    pub matrix_id: String,
    /// Lifecycle status of this matrix artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this matrix ingests.
    pub claim_manifest_ref: String,
    /// Ref to the release artifact graph this matrix publishes from.
    pub artifact_graph_ref: String,
    /// Ref to the M5 feature-train matrix this matrix extends.
    pub feature_train_matrix_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed artifact-family-kind vocabulary.
    pub family_kinds: Vec<M5ArtifactFamilyKind>,
    /// Closed attestation-availability vocabulary.
    pub attestation_states: Vec<AttestationAvailability>,
    /// Closed SBOM-scope vocabulary.
    pub sbom_scopes: Vec<SbomScope>,
    /// Closed symbol/source-map-availability vocabulary.
    pub symbol_availabilities: Vec<SymbolSourceMapAvailability>,
    /// Closed mirror-freshness vocabulary.
    pub mirror_freshness_states: Vec<MirrorFreshness>,
    /// Closed evidence-completeness vocabulary.
    pub evidence_completeness_states: Vec<EvidenceCompleteness>,
    /// Closed freshness-SLO-state vocabulary.
    pub freshness_states: Vec<FreshnessSloState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<M5PublicationGapReason>,
    /// Closed stop-rule-action vocabulary.
    pub publication_actions: Vec<M5PublicationAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking artifact refs this matrix must cover.
    pub release_blocking_artifact_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<M5PublicationStopRule>,
    /// Family rows.
    pub rows: Vec<M5PublicationMatrixRow>,
    /// Recorded publication verdict.
    pub publication: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5PublicationMatrixSummary,
}

impl M5PublicationMatrix {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&M5PublicationMatrixRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_backed(&self) -> Vec<&M5PublicationMatrixRow> {
        self.rows.iter().filter(|row| row.publishes_stable()).collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&M5PublicationMatrixRow> {
        self.rows.iter().filter(|row| !row.publishes_stable()).collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&M5PublicationMatrixRow> {
        self.rows.iter().filter(|row| row.release_blocking).collect()
    }

    /// Returns the rows for one family kind.
    pub fn rows_for_kind(&self, kind: M5ArtifactFamilyKind) -> Vec<&M5PublicationMatrixRow> {
        self.rows.iter().filter(|row| row.family_kind == kind).collect()
    }

    /// Distinct release candidates (by ref) the matrix covers.
    pub fn release_candidates(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.release_candidate_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &M5PublicationStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and stop rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .stop_rules
            .iter()
            .any(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Stop-rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose
    /// claim is already canonically narrowed is not a *publication* blocker, it
    /// merely inherits the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<M5PublicationGapReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row.claim_holds_stable()
                && row
                    .active_gap_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows and stop rules.
    pub fn computed_summary(&self) -> M5PublicationMatrixSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: M5ArtifactFamilyKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&M5PublicationMatrixRow> = self.release_blocking_rows();
        M5PublicationMatrixSummary {
            total_entries: self.rows.len(),
            total_release_candidates: self.release_candidates().len(),
            entries_backed: self.rows.iter().filter(|row| row.publishes_stable()).count(),
            entries_narrowed: self.rows.iter().filter(|row| !row.publishes_stable()).count(),
            entries_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| {
                    row.waiver
                        .as_ref()
                        .map(|w| !w.waiver_ref.trim().is_empty())
                        .unwrap_or(false)
                })
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_backed: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            notebook_pack_entries: kind(M5ArtifactFamilyKind::NotebookPack),
            request_data_asset_entries: kind(M5ArtifactFamilyKind::RequestDataAsset),
            profiler_replay_entries: kind(M5ArtifactFamilyKind::ProfilerReplayArtifact),
            framework_template_entries: kind(M5ArtifactFamilyKind::FrameworkTemplatePack),
            docs_pack_entries: kind(M5ArtifactFamilyKind::DocsPack),
            model_pack_entries: kind(M5ArtifactFamilyKind::ModelPack),
            companion_offboarding_entries: kind(M5ArtifactFamilyKind::CompanionOffboardingPacket),
            managed_output_entries: kind(M5ArtifactFamilyKind::ManagedOutput),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            signatures_verified: self
                .rows
                .iter()
                .filter(|row| row.exact_build.signature_state == SignatureStateClass::Verified)
                .count(),
            attestations_attested: self
                .rows
                .iter()
                .filter(|row| {
                    row.exact_build.attestation_availability == AttestationAvailability::Attested
                })
                .count(),
            sbom_in_scope: self
                .rows
                .iter()
                .filter(|row| {
                    matches!(
                        row.exact_build.sbom_scope,
                        SbomScope::FullGraph | SbomScope::ComponentScoped
                    )
                })
                .count(),
            symbols_available: self
                .rows
                .iter()
                .filter(|row| {
                    matches!(
                        row.exact_build.symbol_availability,
                        SymbolSourceMapAvailability::Published
                            | SymbolSourceMapAvailability::RetainedInternal
                    )
                })
                .count(),
            mirror_current: self
                .rows
                .iter()
                .filter(|row| row.exact_build.mirror_freshness == MirrorFreshness::Current)
                .count(),
            rollback_targets_recorded: self
                .rows
                .iter()
                .filter(|row| !row.exact_build.rollback_target_ref.trim().is_empty())
                .count(),
            mirror_publish_expected: self
                .rows
                .iter()
                .filter(|row| row.mirror_offline.mirror_publish_expected)
                .count(),
            offline_verifiable: self
                .rows
                .iter()
                .filter(|row| row.mirror_offline.offline_verifiable)
                .count(),
            revocable_entries: self
                .rows
                .iter()
                .filter(|row| row.rollback_revocation.revocable)
                .count(),
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About/release-center-safe projection of the
    /// matrix that downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> M5PublicationExportProjection {
        M5PublicationExportProjection {
            matrix_id: self.matrix_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5PublicationExportRow {
                    entry_id: row.entry_id.clone(),
                    family_kind: row.family_kind,
                    artifact_ref: row.artifact_ref.clone(),
                    release_blocking: row.release_blocking,
                    release_candidate_ref: row.release_candidate_ref.clone(),
                    publish_target_class: row.publish_target_class,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    signature_state: row.exact_build.signature_state,
                    attestation_availability: row.exact_build.attestation_availability,
                    sbom_scope: row.exact_build.sbom_scope,
                    symbol_availability: row.exact_build.symbol_availability,
                    mirror_freshness: row.exact_build.mirror_freshness,
                    revocable: row.rollback_revocation.revocable,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the matrix, returning every violation found.
    pub fn validate(&self) -> Vec<M5PublicationMatrixViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(M5PublicationMatrixViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(M5PublicationMatrixViolation::EmptyMatrix);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5PublicationMatrixViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5PublicationMatrixViolation>) {
        if self.schema_version
            != FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_SCHEMA_VERSION
        {
            violations.push(M5PublicationMatrixViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind
            != FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_RECORD_KIND
        {
            violations.push(M5PublicationMatrixViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("matrix_id", &self.matrix_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("artifact_graph_ref", &self.artifact_graph_ref),
            ("feature_train_matrix_ref", &self.feature_train_matrix_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5PublicationMatrixViolation::EmptyField {
                    entry_id: "<matrix>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(M5PublicationMatrixViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.family_kinds != M5ArtifactFamilyKind::ALL.to_vec() {
            violations.push(M5PublicationMatrixViolation::ClosedVocabularyMismatch {
                field: "family_kinds",
            });
        }
        if self.attestation_states != AttestationAvailability::ALL.to_vec() {
            violations.push(M5PublicationMatrixViolation::ClosedVocabularyMismatch {
                field: "attestation_states",
            });
        }
        if self.sbom_scopes != SbomScope::ALL.to_vec() {
            violations.push(M5PublicationMatrixViolation::ClosedVocabularyMismatch {
                field: "sbom_scopes",
            });
        }
        if self.symbol_availabilities != SymbolSourceMapAvailability::ALL.to_vec() {
            violations.push(M5PublicationMatrixViolation::ClosedVocabularyMismatch {
                field: "symbol_availabilities",
            });
        }
        if self.mirror_freshness_states != MirrorFreshness::ALL.to_vec() {
            violations.push(M5PublicationMatrixViolation::ClosedVocabularyMismatch {
                field: "mirror_freshness_states",
            });
        }
        if self.evidence_completeness_states != EvidenceCompleteness::ALL.to_vec() {
            violations.push(M5PublicationMatrixViolation::ClosedVocabularyMismatch {
                field: "evidence_completeness_states",
            });
        }
        if self.freshness_states != FreshnessSloState::ALL.to_vec() {
            violations.push(M5PublicationMatrixViolation::ClosedVocabularyMismatch {
                field: "freshness_states",
            });
        }
        if self.gap_reasons != M5PublicationGapReason::ALL.to_vec() {
            violations.push(M5PublicationMatrixViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.publication_actions != M5PublicationAction::ALL.to_vec() {
            violations.push(M5PublicationMatrixViolation::ClosedVocabularyMismatch {
                field: "publication_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(M5PublicationMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5PublicationMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5PublicationMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5PublicationMatrixViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5PublicationMatrixViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5PublicationMatrixViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5PublicationMatrixViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5PublicationMatrixViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5PublicationMatrixViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in M5PublicationGapReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(M5PublicationMatrixViolation::GapReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &M5PublicationMatrixRow,
        violations: &mut Vec<M5PublicationMatrixViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("artifact_ref", &row.artifact_ref),
            ("artifact_summary", &row.artifact_summary),
            ("release_candidate_ref", &row.release_candidate_ref),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            ("exact_build.build_identity_ref", &row.exact_build.build_identity_ref),
            ("exact_build.provenance_ref", &row.exact_build.provenance_ref),
            ("rollback_revocation.posture_ref", &row.rollback_revocation.posture_ref),
            ("rollback_revocation.summary", &row.rollback_revocation.summary),
            ("mirror_offline.parity_ref", &row.mirror_offline.parity_ref),
            ("mirror_offline.summary", &row.mirror_offline.summary),
            ("proof_packet.packet_id", &row.proof_packet.packet_id),
            ("proof_packet.packet_ref", &row.proof_packet.packet_ref),
            ("proof_packet.proof_index_ref", &row.proof_packet.proof_index_ref),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &row.proof_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5PublicationMatrixViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no family may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(M5PublicationMatrixViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(M5PublicationMatrixViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(M5PublicationMatrixViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // Build success is never publication readiness: a row publishing stable
        // must have intact exact-build linkage.
        if row.publishes_stable() && !row.exact_build.linkage_intact() {
            violations.push(M5PublicationMatrixViolation::HeldWithBrokenExactBuild {
                entry_id: row.entry_id.clone(),
            });
        }

        // A public claim whose canonical label is below the cutline forces the
        // family to inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(M5PublicationMatrixViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(M5PublicationMatrixViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed family carries exactly the public claim's canonical label,
            // names no active gap reason, rides a captured within-SLO packet, is
            // owner-signed, and is revocable.
            if row.published_label != row.claim_label {
                violations.push(M5PublicationMatrixViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(M5PublicationMatrixViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(M5PublicationMatrixViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(M5PublicationMatrixViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(M5PublicationMatrixViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.rollback_revocation.revocable {
                violations.push(M5PublicationMatrixViolation::HeldWithoutRevocation {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            // A narrowing family must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(M5PublicationMatrixViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    published: row.published_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(M5PublicationMatrixViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            // A narrowing row whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(M5PublicationGapReason::ProofPacketStale)
            {
                violations.push(M5PublicationMatrixViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(M5PublicationGapReason::ProofPacketMissing)
            {
                violations.push(M5PublicationMatrixViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_exact_build_reason_coherence(row, violations);
    }

    fn validate_exact_build_reason_coherence(
        &self,
        row: &M5PublicationMatrixRow,
        violations: &mut Vec<M5PublicationMatrixViolation>,
    ) {
        // Every broken exact-build field must be named by an active gap reason so
        // the narrowing is fully explained.
        for expected in row.exact_build.required_gap_reasons() {
            if !row.has_active_reason(expected) {
                violations.push(M5PublicationMatrixViolation::ExactBuildReasonIncoherent {
                    entry_id: row.entry_id.clone(),
                    expected_reason: expected,
                });
            }
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5PublicationMatrixViolation>) {
        let covered: BTreeSet<String> = self
            .rows
            .iter()
            .map(|row| row.artifact_ref.clone())
            .collect();
        for declared in &self.release_blocking_artifact_refs {
            if !covered.contains(declared) {
                violations.push(
                    M5PublicationMatrixViolation::ReleaseBlockingArtifactUncovered {
                        artifact_ref: declared.clone(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking
                && !self.release_blocking_artifact_refs.contains(&row.artifact_ref)
            {
                violations.push(
                    M5PublicationMatrixViolation::ReleaseBlockingRowNotDeclared {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<M5PublicationMatrixViolation>) {
        if self.publication.promotion_gate.trim().is_empty() {
            violations.push(M5PublicationMatrixViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(M5PublicationMatrixViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(M5PublicationMatrixViolation::PublicationDecisionInconsistent {
                declared: self.publication.decision,
                computed,
            });
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(M5PublicationMatrixViolation::PublicationBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.publication.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(M5PublicationMatrixViolation::PublicationBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the M5 publication matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5PublicationMatrixViolation {
    /// The matrix carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the matrix.
        actual: u32,
    },
    /// The matrix carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the matrix.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The matrix has no rows.
    EmptyMatrix,
    /// The matrix has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Row or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateEntryId {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// A stop-rule id appears more than once.
    DuplicateStopRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A stop rule names no labels to watch.
    StopRuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no stop rule watching for it.
    GapReasonWithoutStopRule {
        /// Uncovered reason.
        reason: M5PublicationGapReason,
    },
    /// The published label is wider than the backed claim's canonical label.
    PublishedWiderThanClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A row holds a label while the public claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
    },
    /// A narrowing row carries no active gap reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing row did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A backed row carries a published label different from the claim.
    HeldLabelNotEqualClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A backed row has active gap reasons.
    HeldWithActiveGap {
        /// Row id.
        entry_id: String,
    },
    /// A backed row has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Row id.
        entry_id: String,
    },
    /// A backed row rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Row id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A backed row has broken or stale exact-build linkage.
    HeldWithBrokenExactBuild {
        /// Row id.
        entry_id: String,
    },
    /// A backed row lacks owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A backed row is not revocable once published.
    HeldWithoutRevocation {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing row with a breached packet does not name the stale reason.
    BreachedPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing row with a missing packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A broken exact-build field is not named by an active gap reason.
    ExactBuildReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Reason the broken field requires.
        expected_reason: M5PublicationGapReason,
    },
    /// A release-blocking artifact ref has no covering row.
    ReleaseBlockingArtifactUncovered {
        /// Artifact ref.
        artifact_ref: String,
    },
    /// A release-blocking row is not declared in the release-blocking list.
    ReleaseBlockingRowNotDeclared {
        /// Row id.
        entry_id: String,
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
    /// The summary counts disagree with the rows.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Row id.
        entry_id: String,
    },
}

impl fmt::Display for M5PublicationMatrixViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported matrix schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported matrix record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "matrix {field} is not the canonical value")
            }
            Self::EmptyMatrix => write!(f, "matrix has no rows"),
            Self::NoStopRules => write!(f, "matrix has no stop rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => write!(f, "duplicate entry id {entry_id}"),
            Self::DuplicateStopRuleId { rule_id } => {
                write!(f, "duplicate stop rule id {rule_id}")
            }
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutStopRule { reason } => write!(
                f,
                "gap reason {} has no stop rule watching for it",
                reason.as_str()
            ),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "row {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "row {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::NarrowingWithoutReason { entry_id } => {
                write!(f, "row {entry_id} narrows without an active reason")
            }
            Self::PublishedLabelNotNarrowed {
                entry_id,
                published,
            } => write!(
                f,
                "row {entry_id} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "row {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "row {entry_id} holds stable with an active gap")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "row {entry_id} holds stable without a fresh packet")
            }
            Self::HeldOnStalePacket { entry_id, slo_state } => {
                write!(f, "row {entry_id} holds stable on stale packet {slo_state:?}")
            }
            Self::HeldWithBrokenExactBuild { entry_id } => {
                write!(f, "row {entry_id} holds stable with broken exact-build linkage")
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "row {entry_id} holds stable without owner signoff")
            }
            Self::HeldWithoutRevocation { entry_id } => {
                write!(f, "row {entry_id} holds stable but is not revocable")
            }
            Self::BreachedPacketWithoutReason { entry_id } => write!(
                f,
                "row {entry_id} breached packet without proof_packet_stale reason"
            ),
            Self::MissingPacketWithoutReason { entry_id } => write!(
                f,
                "row {entry_id} missing packet without proof_packet_missing reason"
            ),
            Self::ExactBuildReasonIncoherent {
                entry_id,
                expected_reason,
            } => write!(
                f,
                "row {entry_id} broken exact-build field requires reason {expected_reason:?}"
            ),
            Self::ReleaseBlockingArtifactUncovered { artifact_ref } => {
                write!(f, "release-blocking artifact {artifact_ref} has no covering row")
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => write!(
                f,
                "release-blocking row {entry_id} is not declared in release_blocking_artifact_refs"
            ),
            Self::PublicationDecisionInconsistent { declared, computed } => {
                write!(f, "publication {declared:?} disagrees with computed {computed:?}")
            }
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with rows"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "row {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for M5PublicationMatrixViolation {}

/// Loads the embedded M5 publication matrix.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in matrix no longer matches
/// [`M5PublicationMatrix`].
pub fn current_m5_publication_matrix() -> Result<M5PublicationMatrix, serde_json::Error> {
    serde_json::from_str(
        FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_JSON,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix() -> M5PublicationMatrix {
        current_m5_publication_matrix().expect("matrix parses")
    }

    #[test]
    fn embedded_matrix_parses_and_validates() {
        let m = matrix();
        assert_eq!(
            m.schema_version,
            FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_SCHEMA_VERSION
        );
        assert_eq!(
            m.record_kind,
            FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_RECORD_KIND
        );
        let violations = m.validate();
        assert!(violations.is_empty(), "matrix must validate cleanly: {violations:#?}");
        assert!(!m.rows.is_empty());
    }

    #[test]
    fn covers_every_family_kind() {
        let m = matrix();
        for kind in M5ArtifactFamilyKind::ALL {
            assert!(
                !m.rows_for_kind(kind).is_empty(),
                "family kind {} must have at least one row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn covers_every_declared_release_blocking_artifact() {
        let m = matrix();
        assert!(!m.release_blocking_artifact_refs.is_empty());
        let covered: Vec<&str> = m
            .release_blocking_rows()
            .iter()
            .map(|row| row.artifact_ref.as_str())
            .collect();
        for declared in &m.release_blocking_artifact_refs {
            assert!(
                covered.contains(&declared.as_str()),
                "{declared} has no covering release-blocking row"
            );
        }
    }

    #[test]
    fn summary_counts_match_rows() {
        let m = matrix();
        assert_eq!(m.summary, m.computed_summary());
        assert_eq!(m.summary.entries_backed + m.summary.entries_narrowed, m.rows.len());
    }

    #[test]
    fn publication_decision_matches_computed() {
        let m = matrix();
        assert_eq!(m.publication.decision, m.computed_publication_decision());
        assert_eq!(m.publication.blocking_rule_ids, m.computed_blocking_rule_ids());
        assert_eq!(m.publication.blocking_claim_ids, m.computed_blocking_entry_ids());
    }

    #[test]
    fn every_gap_reason_has_a_stop_rule() {
        let m = matrix();
        let covered: BTreeSet<M5PublicationGapReason> =
            m.stop_rules.iter().map(|rule| rule.trigger_reason).collect();
        for reason in M5PublicationGapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn matrix_narrows_at_least_one_family() {
        let m = matrix();
        assert!(
            !m.rows_narrowed().is_empty(),
            "the matrix must narrow at least one family below the cutline"
        );
    }

    #[test]
    fn validate_flags_a_backed_row_with_active_gap() {
        let mut m = matrix();
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a backed row exists");
        row.active_gap_reasons.push(M5PublicationGapReason::ProofPacketMissing);
        m.summary = m.computed_summary();
        assert!(m
            .validate()
            .iter()
            .any(|v| matches!(v, M5PublicationMatrixViolation::HeldWithActiveGap { .. })));
    }

    #[test]
    fn validate_flags_a_backed_row_with_broken_exact_build() {
        let mut m = matrix();
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a backed row exists");
        // Break the exact-build linkage without narrowing the published label.
        row.exact_build.signature_state = SignatureStateClass::Missing;
        m.summary = m.computed_summary();
        m.publication.decision = m.computed_publication_decision();
        m.publication.blocking_rule_ids = m.computed_blocking_rule_ids();
        m.publication.blocking_claim_ids = m.computed_blocking_entry_ids();
        assert!(m.validate().iter().any(|v| matches!(
            v,
            M5PublicationMatrixViolation::HeldWithBrokenExactBuild { .. }
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_row_that_does_not_narrow() {
        let mut m = matrix();
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a backed row exists");
        // Strip the SBOM (breaks linkage) but keep the published label stable.
        row.exact_build.sbom_scope = SbomScope::Missing;
        row.active_gap_reasons.push(M5PublicationGapReason::SbomIncomplete);
        m.summary = m.computed_summary();
        m.publication.decision = m.computed_publication_decision();
        m.publication.blocking_rule_ids = m.computed_blocking_rule_ids();
        m.publication.blocking_claim_ids = m.computed_blocking_entry_ids();
        assert!(m.validate().iter().any(|v| matches!(
            v,
            M5PublicationMatrixViolation::PublishedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut m = matrix();
        // Force a blocking rule to fire on a still-stable claim.
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a backed row exists");
        row.active_gap_reasons.push(M5PublicationGapReason::ProofPacketMissing);
        m.publication.decision = PromotionDecision::Proceed;
        m.publication.blocking_rule_ids = m.computed_blocking_rule_ids();
        m.publication.blocking_claim_ids = m.computed_blocking_entry_ids();
        assert!(m.validate().iter().any(|v| matches!(
            v,
            M5PublicationMatrixViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_backed_claim_without_signoff() {
        let mut m = matrix();
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a backed row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        m.summary = m.computed_summary();
        assert!(m
            .validate()
            .iter()
            .any(|v| matches!(v, M5PublicationMatrixViolation::HeldWithoutSignoff { .. })));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let m = matrix();
        let projection = m.support_export_projection();
        assert_eq!(projection.rows.len(), m.rows.len());
        for (row, proj) in m.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, proj.entry_id);
            assert_eq!(row.publishes_stable(), proj.publishes_stable);
        }
    }
}
