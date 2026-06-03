//! Publish the signed M4 stable evidence pack plus the benchmark, compatibility,
//! and migration launch bundle.
//!
//! The [`stable_claim_manifest`](crate::stable_claim_manifest) decides the single
//! canonical lifecycle label each *subject* publishes; the
//! [`stable_proof_index`](crate::stable_proof_index) decides whether each
//! launch-blocking requirement is proven; the
//! [`stable_publication_pack`](crate::stable_publication_pack) governs the
//! outward-facing publications the release line ships about itself. None of them
//! answer the question this module answers: **for each evidence bundle that must be
//! signed and published as part of the M4 stable evidence pack — benchmarks,
//! compatibility, migration, accessibility, docs/help truth, security response,
//! release artifact graph, qualification, release promotion, and critical dependency
//! — is the bundle actually signed or attested, grounded in a current proof packet,
//! and narrowed below the cutline the moment its backing thins out?** This module is
//! the **signed M4 stable evidence pack**. For every bundle it records one row that
//! binds the bundle to the [`stable_claim_manifest`](crate::stable_claim_manifest)
//! entry whose lifecycle label it backs, the upstream evidence artifact it
//! references, the attestation or signature that governs it, and the waiver (if any)
//! holding it provisionally.
//!
//! Each [`EvidenceBundleRow`] is one `(evidence bundle, public claim)` binding. It:
//!
//! - names the bundle kind it governs ([`EvidenceBundleRow::bundle_kind`],
//!   [`EvidenceBundleRow::subject_ref`], [`EvidenceBundleRow::subject_summary`])
//!   and whether that bundle is part of the release-blocking set
//!   ([`EvidenceBundleRow::release_blocking`]);
//! - pins the upstream evidence artifact ref
//!   ([`EvidenceBundleRow::upstream_artifact_ref`]) so the signed pack is always
//!   traceable to the exact build it speaks for;
//! - pins the proof packet ([`ProofPacket`]) with its packet-freshness SLO;
//! - carries an attestation ref ([`EvidenceBundleRow::attestation_ref`]) and a
//!   signature ref ([`EvidenceBundleRow::signature_ref`]) so the bundle's signing
//!   status is observable;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose
//!   public claim it backs ([`EvidenceBundleRow::claim_ref`]) and the canonical
//!   lifecycle label that entry publishes ([`EvidenceBundleRow::claim_label`]).
//!   That label is a hard **ceiling**: a bundle may carry the claim's label or
//!   narrow below it, but it may never assert a public claim wider than the public
//!   claim it backs;
//! - records the bundle state earned ([`BundleState`]), the active gap reasons
//!   ([`BundleGapReason`]), and the label it *effectively* publishes after
//!   narrowing ([`EvidenceBundleRow::effective_label`]);
//! - carries an owner sign-off and optional waiver so a provisionally held bundle
//!   can still block promotion if the waiver expires or sign-off is missing.
//!
//! The [`BundleRule`] set names the closed conditions that gate publication, and
//! [`SignedM4StableEvidencePack::publication`] records the resulting proceed/hold
//! verdict.
//!
//! The pack is checked in at
//! `artifacts/release/publish_the_signed_m4_stable_evidence_pack_plus.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material. Date
//! arithmetic (packet-freshness recomputation and waiver expiry against an
//! `as_of` date) lives in the CI gate; this model enforces the structural and
//! logical invariants that hold regardless of the clock — narrowing consistency,
//! the no-widening rule, packet/state coherence, owner sign-off on current rows,
//! rule wiring, and the publication verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported schema version.
pub const SIGNED_M4_STABLE_EVIDENCE_PACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the pack.
pub const SIGNED_M4_STABLE_EVIDENCE_PACK_RECORD_KIND: &str = "signed_m4_stable_evidence_pack";

/// Repo-relative path to the checked-in pack.
pub const SIGNED_M4_STABLE_EVIDENCE_PACK_PATH: &str =
    "artifacts/release/publish_the_signed_m4_stable_evidence_pack_plus.json";

/// Embedded checked-in pack JSON.
pub const SIGNED_M4_STABLE_EVIDENCE_PACK_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/publish_the_signed_m4_stable_evidence_pack_plus.json"
));

/// The kind of evidence bundle a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceBundleKind {
    /// Benchmark launch bundle: public benchmark publication, hot-path performance
    /// budgets, and benchmark-lab traces.
    Benchmark,
    /// Compatibility launch bundle: compatibility reports, deprecation packets,
    /// schema version windows, and skew registers.
    Compatibility,
    /// Migration launch bundle: migration guides, known-limits publications,
    /// end-of-support notices, and rollback sequences.
    Migration,
    /// Accessibility signoff bundle: IME, grapheme, bidi, Unicode, high-contrast,
    /// zoom, density, pseudolocalization, RTL, and desktop platform conformance.
    Accessibility,
    /// Docs/help truth bundle: docs browser truth, pack truth, Help/About,
    /// service health, and semantic recall boundaries.
    DocsHelp,
    /// Security response bundle: advisory, CVE, GHSA publication, emergency
    /// disable, and mirror/offline drills.
    Security,
    /// Release artifact graph bundle: one-build identity, provenance, SBOM,
    /// notices, attestation, and mirror parity.
    ArtifactGraph,
    /// Qualification bundle: optional-surface qualification packets and enforced
    /// narrower-than-stable labeling.
    Qualification,
    /// Release promotion bundle: release-center promotion evidence, canary/pilot
    /// controls, and ring progression.
    ReleasePromotion,
    /// Critical dependency bundle: dependency register, fork/replace log,
    /// third-party import manifest, and REUSE/SPDX/notice coverage.
    Dependency,
}

impl EvidenceBundleKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::Benchmark,
        Self::Compatibility,
        Self::Migration,
        Self::Accessibility,
        Self::DocsHelp,
        Self::Security,
        Self::ArtifactGraph,
        Self::Qualification,
        Self::ReleasePromotion,
        Self::Dependency,
    ];

    /// Stable token recorded in the pack.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Benchmark => "benchmark",
            Self::Compatibility => "compatibility",
            Self::Migration => "migration",
            Self::Accessibility => "accessibility",
            Self::DocsHelp => "docs_help",
            Self::Security => "security",
            Self::ArtifactGraph => "artifact_graph",
            Self::Qualification => "qualification",
            Self::ReleasePromotion => "release_promotion",
            Self::Dependency => "dependency",
        }
    }
}

/// Bundle state a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleState {
    /// The bundle is signed and current: a captured, within-SLO proof packet and
    /// valid attestation or signature back the public claim at its full canonical
    /// lifecycle label.
    SignedCurrent,
    /// The bundle carries the claim's full label only because an active, unexpired
    /// waiver covers a recorded residual gap.
    SignedOnWaiver,
    /// The bundle is unsigned or unattested; the label must narrow.
    UnsignedUnattested,
    /// The proof packet breached its freshness SLO (or is missing); the label
    /// must narrow.
    NarrowedStale,
    /// The public claim this bundle backs is itself below the cutline, so the
    /// bundle inherits that ceiling and narrows.
    NarrowedClaimNarrowed,
    /// The bundle relied on a waiver that has expired; the label must narrow.
    NarrowedWaiverExpired,
    /// The required owner sign-off is missing.
    NarrowedOwnerSignoffMissing,
}

impl BundleState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SignedCurrent,
        Self::SignedOnWaiver,
        Self::UnsignedUnattested,
        Self::NarrowedStale,
        Self::NarrowedClaimNarrowed,
        Self::NarrowedWaiverExpired,
        Self::NarrowedOwnerSignoffMissing,
    ];

    /// Stable token recorded in the pack.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedCurrent => "signed_current",
            Self::SignedOnWaiver => "signed_on_waiver",
            Self::UnsignedUnattested => "unsigned_unattested",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedClaimNarrowed => "narrowed_claim_narrowed",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
            Self::NarrowedOwnerSignoffMissing => "narrowed_owner_signoff_missing",
        }
    }

    /// Whether the state lets a bundle carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::SignedCurrent | Self::SignedOnWaiver)
    }

    /// Whether the state forces the bundle below the public claim label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a bundle narrows or a bundle rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleGapReason {
    /// The backing public claim narrowed below the cutline.
    ClaimLabelNarrowed,
    /// No proof packet has been captured for the bundle.
    PacketMissing,
    /// The proof packet breached its freshness SLO.
    PacketFreshnessBreached,
    /// Required evidence is incomplete.
    EvidenceIncomplete,
    /// The bundle is missing its attestation or signature.
    AttestationMissing,
    /// The bundle's signature is invalid or does not match the build.
    SignatureInvalid,
    /// A waiver the bundle relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl BundleGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ClaimLabelNarrowed,
        Self::PacketMissing,
        Self::PacketFreshnessBreached,
        Self::EvidenceIncomplete,
        Self::AttestationMissing,
        Self::SignatureInvalid,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the pack.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::PacketMissing => "packet_missing",
            Self::PacketFreshnessBreached => "packet_freshness_breached",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::AttestationMissing => "attestation_missing",
            Self::SignatureInvalid => "signature_invalid",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a bundle rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the bundle's published lifecycle label below the cutline.
    NarrowBundleLabel,
    /// Refresh the proof packet so it re-enters its freshness SLO.
    RefreshPacket,
    /// Recapture the bundle evidence.
    RecaptureEvidence,
    /// Request or re-verify the bundle attestation or signature.
    RequestAttestation,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl BundleAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HoldPublication,
        Self::NarrowBundleLabel,
        Self::RefreshPacket,
        Self::RecaptureEvidence,
        Self::RequestAttestation,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the pack.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowBundleLabel => "narrow_bundle_label",
            Self::RefreshPacket => "refresh_packet",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::RequestAttestation => "request_attestation",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One bundle rule: a closed condition that narrows a bundle label and may gate
/// publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BundleRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: BundleGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: BundleAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One evidence bundle row: a `(bundle, public claim)` binding bound to its
/// upstream artifact, attestation, proof packet, and canonical ceiling label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceBundleRow {
    /// Stable bundle-row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The evidence bundle kind this row governs.
    pub bundle_kind: EvidenceBundleKind,
    /// Ref to the upstream evidence artifact this bundle aggregates.
    pub upstream_artifact_ref: String,
    /// Ref to the subject (surface, requirement, or publication) this bundle
    /// speaks for.
    pub subject_ref: String,
    /// Human-readable summary of the subject.
    pub subject_summary: String,
    /// Whether the bundle is part of the release-blocking bundle set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this bundle backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: a
    /// bundle may never carry a label wider than this.
    pub claim_label: StableClaimLevel,
    /// Bundle state earned for the public claim it backs.
    pub bundle_state: BundleState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Ref to the attestation that governs this bundle, when present.
    #[serde(default)]
    pub attestation_ref: Option<String>,
    /// Ref to the signature that covers this bundle, when present.
    #[serde(default)]
    pub signature_ref: Option<String>,
    /// Waiver authorizing a provisional bundle, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the bundle.
    #[serde(default)]
    pub active_gap_reasons: Vec<BundleGapReason>,
    /// The lifecycle label the bundle effectively publishes after narrowing.
    pub effective_label: StableClaimLevel,
    /// Publication destinations that render this bundle.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the bundle carries this posture.
    pub rationale: String,
}

impl EvidenceBundleRow {
    /// True when the bundle's effective label backs a Stable public claim.
    pub fn publishes_stable(&self) -> bool {
        self.effective_label.is_at_or_above_cutline()
    }

    /// True when the bundle state lets it carry the public claim at its label.
    pub fn holds_label(&self) -> bool {
        self.bundle_state.holds_label()
    }

    /// True when a gap reason is active on the bundle.
    pub fn has_active_reason(&self, reason: BundleGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// Publication verdict for the signed evidence pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationRecord {
    /// The publication gate this verdict answers.
    pub publication_gate: String,
    /// Proceed/hold decision.
    pub decision: PromotionDecision,
    /// Rule ids that are blocking publication.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Entry ids that are blocking publication.
    #[serde(default)]
    pub blocking_entry_ids: Vec<String>,
    /// Reviewable reason for the verdict.
    pub rationale: String,
}

/// Summary counts for the signed evidence pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Summary {
    /// Total number of bundle rows.
    pub total_bundles: usize,
    /// Bundles that are signed and current.
    pub bundles_signed_current: usize,
    /// Bundles that are signed on waiver.
    pub bundles_signed_on_waiver: usize,
    /// Bundles narrowed below the cutline.
    pub bundles_narrowed: usize,
    /// Release-blocking bundles.
    pub bundles_release_blocking: usize,
    /// Release-blocking bundles that are narrowed.
    pub bundles_release_blocking_narrowed: usize,
    /// Bundles on active waiver.
    pub bundles_on_waiver: usize,
    /// Benchmark bundle rows.
    pub benchmark_bundles: usize,
    /// Compatibility bundle rows.
    pub compatibility_bundles: usize,
    /// Migration bundle rows.
    pub migration_bundles: usize,
    /// Accessibility bundle rows.
    pub accessibility_bundles: usize,
    /// Docs/help bundle rows.
    pub docs_help_bundles: usize,
    /// Security bundle rows.
    pub security_bundles: usize,
    /// Artifact graph bundle rows.
    pub artifact_graph_bundles: usize,
    /// Qualification bundle rows.
    pub qualification_bundles: usize,
    /// Release promotion bundle rows.
    pub release_promotion_bundles: usize,
    /// Dependency bundle rows.
    pub dependency_bundles: usize,
    /// Packets currently within SLO.
    pub packets_current: usize,
    /// Packets due for refresh.
    pub packets_due_for_refresh: usize,
    /// Packets that breached SLO.
    pub packets_breached: usize,
    /// Packets missing.
    pub packets_missing: usize,
    /// Number of rules currently firing.
    pub rules_firing: usize,
}

/// One export row for support/procurement surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BundleExportRow {
    /// Bundle row id.
    pub entry_id: String,
    /// Subject ref.
    pub subject_ref: String,
    /// Whether the bundle publishes stable.
    pub publishes_stable: bool,
    /// Effective label.
    pub published_label: StableClaimLevel,
    /// SLO state from the proof packet.
    pub slo_state: String,
    /// Attestation ref, if any.
    #[serde(default)]
    pub attestation_ref: Option<String>,
    /// Signature ref, if any.
    #[serde(default)]
    pub signature_ref: Option<String>,
}

/// Export projection for support and procurement consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BundleExportProjection {
    /// Projection rows.
    pub rows: Vec<BundleExportRow>,
    /// Publication decision.
    pub publication_decision: PromotionDecision,
    /// Pack id.
    pub pack_id: String,
    /// As-of date.
    pub as_of: String,
}

/// The signed M4 stable evidence pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedM4StableEvidencePack {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Stable pack id.
    pub pack_id: String,
    /// Status.
    pub status: String,
    /// Overview page ref.
    pub overview_page: String,
    /// As-of date.
    pub as_of: String,
    /// Ref to the stable claim manifest.
    pub claim_manifest_ref: String,
    /// Closed vocabulary of lifecycle labels.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed vocabulary of bundle kinds.
    pub bundle_kinds: Vec<EvidenceBundleKind>,
    /// Closed vocabulary of bundle states.
    pub bundle_states: Vec<BundleState>,
    /// Closed vocabulary of gap reasons.
    pub gap_reasons: Vec<BundleGapReason>,
    /// Closed vocabulary of bundle actions.
    pub bundle_actions: Vec<BundleAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// Release-blocking bundle refs.
    pub release_blocking_bundle_refs: Vec<String>,
    /// Bundle rows.
    pub bundles: Vec<EvidenceBundleRow>,
    /// Bundle rules.
    pub bundle_rules: Vec<BundleRule>,
    /// Publication verdict.
    pub publication: PublicationRecord,
    /// Summary counts.
    pub summary: Summary,
}

impl SignedM4StableEvidencePack {
    /// Bundles whose effective label is at or above the stable cutline.
    pub fn rows_published_stable(&self) -> Vec<&EvidenceBundleRow> {
        self.bundles
            .iter()
            .filter(|r| r.publishes_stable())
            .collect()
    }

    /// Bundles whose effective label is below the stable cutline.
    pub fn rows_narrowed(&self) -> Vec<&EvidenceBundleRow> {
        self.bundles
            .iter()
            .filter(|r| !r.publishes_stable())
            .collect()
    }

    /// Compute the summary from the current rows.
    pub fn computed_summary(&self) -> Summary {
        Summary {
            total_bundles: self.bundles.len(),
            bundles_signed_current: self
                .bundles
                .iter()
                .filter(|r| r.bundle_state == BundleState::SignedCurrent)
                .count(),
            bundles_signed_on_waiver: self
                .bundles
                .iter()
                .filter(|r| r.bundle_state == BundleState::SignedOnWaiver)
                .count(),
            bundles_narrowed: self
                .bundles
                .iter()
                .filter(|r| r.bundle_state.forces_narrowing())
                .count(),
            bundles_release_blocking: self.bundles.iter().filter(|r| r.release_blocking).count(),
            bundles_release_blocking_narrowed: self
                .bundles
                .iter()
                .filter(|r| r.release_blocking && !r.publishes_stable())
                .count(),
            bundles_on_waiver: self.bundles.iter().filter(|r| r.waiver.is_some()).count(),
            benchmark_bundles: self
                .bundles
                .iter()
                .filter(|r| r.bundle_kind == EvidenceBundleKind::Benchmark)
                .count(),
            compatibility_bundles: self
                .bundles
                .iter()
                .filter(|r| r.bundle_kind == EvidenceBundleKind::Compatibility)
                .count(),
            migration_bundles: self
                .bundles
                .iter()
                .filter(|r| r.bundle_kind == EvidenceBundleKind::Migration)
                .count(),
            accessibility_bundles: self
                .bundles
                .iter()
                .filter(|r| r.bundle_kind == EvidenceBundleKind::Accessibility)
                .count(),
            docs_help_bundles: self
                .bundles
                .iter()
                .filter(|r| r.bundle_kind == EvidenceBundleKind::DocsHelp)
                .count(),
            security_bundles: self
                .bundles
                .iter()
                .filter(|r| r.bundle_kind == EvidenceBundleKind::Security)
                .count(),
            artifact_graph_bundles: self
                .bundles
                .iter()
                .filter(|r| r.bundle_kind == EvidenceBundleKind::ArtifactGraph)
                .count(),
            qualification_bundles: self
                .bundles
                .iter()
                .filter(|r| r.bundle_kind == EvidenceBundleKind::Qualification)
                .count(),
            release_promotion_bundles: self
                .bundles
                .iter()
                .filter(|r| r.bundle_kind == EvidenceBundleKind::ReleasePromotion)
                .count(),
            dependency_bundles: self
                .bundles
                .iter()
                .filter(|r| r.bundle_kind == EvidenceBundleKind::Dependency)
                .count(),
            packets_current: self
                .bundles
                .iter()
                .filter(|r| r.proof_packet.slo_state == FreshnessSloState::Current)
                .count(),
            packets_due_for_refresh: self
                .bundles
                .iter()
                .filter(|r| r.proof_packet.slo_state == FreshnessSloState::DueForRefresh)
                .count(),
            packets_breached: self
                .bundles
                .iter()
                .filter(|r| r.proof_packet.slo_state == FreshnessSloState::Breached)
                .count(),
            packets_missing: self
                .bundles
                .iter()
                .filter(|r| r.proof_packet.slo_state == FreshnessSloState::Missing)
                .count(),
            rules_firing: self.computed_blocking_rule_ids().len(),
        }
    }

    /// Compute the publication decision from current rows.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        let blocking = self.computed_blocking_rule_ids();
        if blocking.is_empty() {
            PromotionDecision::Proceed
        } else {
            PromotionDecision::Hold
        }
    }

    /// Compute the blocking rule ids from firing rules.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let active_reasons: BTreeSet<BundleGapReason> = self
            .bundles
            .iter()
            .flat_map(|r| &r.active_gap_reasons)
            .copied()
            .collect();
        self.bundle_rules
            .iter()
            .filter(|rule| rule.blocks_publication && active_reasons.contains(&rule.trigger_reason))
            .map(|rule| rule.rule_id.clone())
            .collect()
    }

    /// Compute the blocking entry ids from rows touched by firing rules.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_reasons: BTreeSet<BundleGapReason> = self
            .bundle_rules
            .iter()
            .filter(|rule| rule.blocks_publication)
            .map(|rule| rule.trigger_reason)
            .collect();
        self.bundles
            .iter()
            .filter(|r| {
                r.release_blocking
                    && r.active_gap_reasons
                        .iter()
                        .any(|reason| blocking_reasons.contains(reason))
            })
            .map(|r| r.entry_id.clone())
            .collect()
    }

    /// Build a support-export projection.
    pub fn support_export_projection(&self) -> BundleExportProjection {
        BundleExportProjection {
            rows: self
                .bundles
                .iter()
                .map(|r| BundleExportRow {
                    entry_id: r.entry_id.clone(),
                    subject_ref: r.subject_ref.clone(),
                    publishes_stable: r.publishes_stable(),
                    published_label: r.effective_label,
                    slo_state: r.proof_packet.slo_state.as_str().to_owned(),
                    attestation_ref: r.attestation_ref.clone(),
                    signature_ref: r.signature_ref.clone(),
                })
                .collect(),
            publication_decision: self.publication.decision,
            pack_id: self.pack_id.clone(),
            as_of: self.as_of.clone(),
        }
    }

    /// Validate structural and logical invariants.
    pub fn validate(&self) -> Vec<SignedM4StableEvidencePackViolation> {
        let mut violations = Vec::new();

        if self.schema_version != SIGNED_M4_STABLE_EVIDENCE_PACK_SCHEMA_VERSION {
            violations.push(SignedM4StableEvidencePackViolation::SchemaVersionMismatch {
                expected: SIGNED_M4_STABLE_EVIDENCE_PACK_SCHEMA_VERSION,
                found: self.schema_version,
            });
        }
        if self.record_kind != SIGNED_M4_STABLE_EVIDENCE_PACK_RECORD_KIND {
            violations.push(SignedM4StableEvidencePackViolation::RecordKindMismatch {
                expected: SIGNED_M4_STABLE_EVIDENCE_PACK_RECORD_KIND.to_owned(),
                found: self.record_kind.clone(),
            });
        }

        let kind_set: BTreeSet<EvidenceBundleKind> = self.bundle_kinds.iter().copied().collect();
        for kind in EvidenceBundleKind::ALL {
            if !kind_set.contains(&kind) {
                violations.push(SignedM4StableEvidencePackViolation::VocabIncomplete {
                    vocab: "bundle_kinds".to_owned(),
                    missing: kind.as_str().to_owned(),
                });
            }
        }

        let state_set: BTreeSet<BundleState> = self.bundle_states.iter().copied().collect();
        for state in BundleState::ALL {
            if !state_set.contains(&state) {
                violations.push(SignedM4StableEvidencePackViolation::VocabIncomplete {
                    vocab: "bundle_states".to_owned(),
                    missing: state.as_str().to_owned(),
                });
            }
        }

        let reason_set: BTreeSet<BundleGapReason> = self.gap_reasons.iter().copied().collect();
        for reason in BundleGapReason::ALL {
            if !reason_set.contains(&reason) {
                violations.push(SignedM4StableEvidencePackViolation::VocabIncomplete {
                    vocab: "gap_reasons".to_owned(),
                    missing: reason.as_str().to_owned(),
                });
            }
        }

        let action_set: BTreeSet<BundleAction> = self.bundle_actions.iter().copied().collect();
        for action in BundleAction::ALL {
            if !action_set.contains(&action) {
                violations.push(SignedM4StableEvidencePackViolation::VocabIncomplete {
                    vocab: "bundle_actions".to_owned(),
                    missing: action.as_str().to_owned(),
                });
            }
        }

        for bundle in &self.bundles {
            self.validate_bundle(bundle, &mut violations);
        }

        let rule_reasons: BTreeSet<BundleGapReason> =
            self.bundle_rules.iter().map(|r| r.trigger_reason).collect();
        for reason in BundleGapReason::ALL {
            if !rule_reasons.contains(&reason) {
                violations.push(SignedM4StableEvidencePackViolation::VocabIncomplete {
                    vocab: "bundle_rules".to_owned(),
                    missing: reason.as_str().to_owned(),
                });
            }
        }

        self.validate_publication(&mut violations);

        let computed = self.computed_summary();
        if self.summary != computed {
            violations.push(SignedM4StableEvidencePackViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_bundle(
        &self,
        bundle: &EvidenceBundleRow,
        violations: &mut Vec<SignedM4StableEvidencePackViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &bundle.entry_id),
            ("title", &bundle.title),
            ("upstream_artifact_ref", &bundle.upstream_artifact_ref),
            ("subject_ref", &bundle.subject_ref),
            ("subject_summary", &bundle.subject_summary),
            ("claim_ref", &bundle.claim_ref),
            ("rationale", &bundle.rationale),
            ("owner_signoff.owner_ref", &bundle.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(SignedM4StableEvidencePackViolation::EmptyField {
                    id: bundle.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        if !self.bundle_kinds.contains(&bundle.bundle_kind) {
            violations.push(SignedM4StableEvidencePackViolation::UnknownVocab {
                id: bundle.entry_id.clone(),
                vocab: "bundle_kind".to_owned(),
                value: format!("{:?}", bundle.bundle_kind),
            });
        }

        // No widening: effective_label may not exceed claim_label.
        if bundle.effective_label.rank() > bundle.claim_label.rank() {
            violations.push(
                SignedM4StableEvidencePackViolation::PublishedWiderThanClaim {
                    entry_id: bundle.entry_id.clone(),
                    claim_label: bundle.claim_label,
                    published_label: bundle.effective_label,
                },
            );
        }

        // A signed state must have owner sign-off and no active gap reasons.
        if bundle.bundle_state.holds_label() {
            if !bundle.active_gap_reasons.is_empty() {
                violations.push(
                    SignedM4StableEvidencePackViolation::HeldBundleWithActiveGap {
                        entry_id: bundle.entry_id.clone(),
                    },
                );
            }
            if !(bundle.owner_signoff.signed_off && bundle.owner_signoff.signed_at.is_some()) {
                violations.push(
                    SignedM4StableEvidencePackViolation::HeldBundleWithoutSignoff {
                        entry_id: bundle.entry_id.clone(),
                    },
                );
            }
        }

        // A narrowing state must drop the effective label below the claim and name
        // at least one active reason.
        if bundle.bundle_state.forces_narrowing() {
            if bundle.effective_label.rank() >= bundle.claim_label.rank() {
                violations.push(
                    SignedM4StableEvidencePackViolation::PublishedLabelNotNarrowed {
                        entry_id: bundle.entry_id.clone(),
                        state: bundle.bundle_state,
                        claim_label: bundle.claim_label,
                        published_label: bundle.effective_label,
                    },
                );
            }
            if bundle.active_gap_reasons.is_empty() {
                violations.push(
                    SignedM4StableEvidencePackViolation::NarrowingWithoutReason {
                        entry_id: bundle.entry_id.clone(),
                        state: bundle.bundle_state,
                    },
                );
            }
        }

        self.validate_bundle_state_reason_coherence(bundle, violations);
    }

    fn validate_bundle_state_reason_coherence(
        &self,
        bundle: &EvidenceBundleRow,
        violations: &mut Vec<SignedM4StableEvidencePackViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<SignedM4StableEvidencePackViolation>,
                               expected: BundleGapReason| {
            violations.push(SignedM4StableEvidencePackViolation::StateReasonIncoherent {
                entry_id: bundle.entry_id.clone(),
                state: bundle.bundle_state,
                expected_reason: expected,
            });
        };

        match bundle.bundle_state {
            BundleState::UnsignedUnattested => {
                if !bundle.has_active_reason(BundleGapReason::AttestationMissing)
                    && !bundle.has_active_reason(BundleGapReason::SignatureInvalid)
                {
                    push_incoherent(violations, BundleGapReason::AttestationMissing);
                }
            }
            BundleState::NarrowedStale => {
                if !bundle.has_active_reason(BundleGapReason::PacketFreshnessBreached) {
                    push_incoherent(violations, BundleGapReason::PacketFreshnessBreached);
                }
            }
            BundleState::NarrowedClaimNarrowed => {
                if !bundle.has_active_reason(BundleGapReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, BundleGapReason::ClaimLabelNarrowed);
                }
            }
            BundleState::NarrowedWaiverExpired => {
                if !bundle.has_active_reason(BundleGapReason::WaiverExpired) {
                    push_incoherent(violations, BundleGapReason::WaiverExpired);
                }
                if bundle.waiver.is_none() {
                    violations.push(
                        SignedM4StableEvidencePackViolation::WaiverStateWithoutWaiver {
                            entry_id: bundle.entry_id.clone(),
                            state: bundle.bundle_state,
                        },
                    );
                }
            }
            BundleState::NarrowedOwnerSignoffMissing => {
                if !bundle.has_active_reason(BundleGapReason::OwnerSignoffMissing) {
                    push_incoherent(violations, BundleGapReason::OwnerSignoffMissing);
                }
            }
            BundleState::SignedCurrent | BundleState::SignedOnWaiver => {}
        }
    }

    fn validate_publication(&self, violations: &mut Vec<SignedM4StableEvidencePackViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(SignedM4StableEvidencePackViolation::EmptyField {
                id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(SignedM4StableEvidencePackViolation::EmptyField {
                id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                SignedM4StableEvidencePackViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                SignedM4StableEvidencePackViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids".to_owned(),
                },
            );
        }
        if self.publication.blocking_entry_ids != self.computed_blocking_entry_ids() {
            violations.push(
                SignedM4StableEvidencePackViolation::PublicationBlockingSetMismatch {
                    field: "blocking_entry_ids".to_owned(),
                },
            );
        }
    }
}

/// Validation violation for the signed evidence pack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignedM4StableEvidencePackViolation {
    /// Schema version does not match the crate constant.
    SchemaVersionMismatch { expected: u32, found: u32 },
    /// Record kind does not match the crate constant.
    RecordKindMismatch { expected: String, found: String },
    /// A closed vocabulary is missing a required member.
    VocabIncomplete { vocab: String, missing: String },
    /// A row uses a value outside the closed vocabularies.
    UnknownVocab {
        id: String,
        vocab: String,
        value: String,
    },
    /// A required string field is empty.
    EmptyField {
        id: String,
        field_name: &'static str,
    },
    /// A bundle's effective label is wider than its claim ceiling.
    PublishedWiderThanClaim {
        entry_id: String,
        claim_label: StableClaimLevel,
        published_label: StableClaimLevel,
    },
    /// A bundle is held while an active gap reason is present.
    HeldBundleWithActiveGap { entry_id: String },
    /// A held bundle lacks owner sign-off.
    HeldBundleWithoutSignoff { entry_id: String },
    /// A narrowing state did not drop the effective label below the claim.
    PublishedLabelNotNarrowed {
        entry_id: String,
        state: BundleState,
        claim_label: StableClaimLevel,
        published_label: StableClaimLevel,
    },
    /// A narrowing state has no active reason.
    NarrowingWithoutReason {
        entry_id: String,
        state: BundleState,
    },
    /// A state/reason pair is incoherent.
    StateReasonIncoherent {
        entry_id: String,
        state: BundleState,
        expected_reason: BundleGapReason,
    },
    /// A waiver state lacks a waiver.
    WaiverStateWithoutWaiver {
        entry_id: String,
        state: BundleState,
    },
    /// The declared publication decision does not match the computed one.
    PublicationDecisionInconsistent {
        declared: PromotionDecision,
        computed: PromotionDecision,
    },
    /// A publication blocking set does not match the computed one.
    PublicationBlockingSetMismatch { field: String },
    /// Summary counts do not match the computed values.
    SummaryMismatch,
}

impl fmt::Display for SignedM4StableEvidencePackViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersionMismatch { expected, found } => write!(
                f,
                "schema version mismatch: expected {expected}, found {found}"
            ),
            Self::RecordKindMismatch { expected, found } => write!(
                f,
                "record kind mismatch: expected {expected}, found {found}"
            ),
            Self::VocabIncomplete { vocab, missing } => {
                write!(f, "vocabulary {vocab} is missing member {missing}")
            }
            Self::UnknownVocab { id, vocab, value } => {
                write!(f, "row {id} uses unknown {vocab} value {value}")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "row {id} has empty field {field_name}")
            }
            Self::PublishedWiderThanClaim {
                entry_id,
                claim_label,
                published_label,
            } => write!(
                f,
                "row {entry_id} publishes {published_label:?} wider than claim {claim_label:?}"
            ),
            Self::HeldBundleWithActiveGap { entry_id } => write!(
                f,
                "row {entry_id} is signed while an active gap reason is present"
            ),
            Self::HeldBundleWithoutSignoff { entry_id } => {
                write!(f, "row {entry_id} is signed without owner sign-off")
            }
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                claim_label,
                published_label,
            } => write!(
                f,
                "row {entry_id} state {} must narrow below claim {claim_label:?} but effective is {published_label:?}",
                state.as_str()
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "row {entry_id} state {} has no active gap reason",
                state.as_str()
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "row {entry_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => write!(
                f,
                "row {entry_id} state {} names no waiver",
                state.as_str()
            ),
            Self::PublicationDecisionInconsistent { declared, computed } => write!(
                f,
                "publication decision {declared:?} disagrees with computed {computed:?}"
            ),
            Self::PublicationBlockingSetMismatch { field } => write!(
                f,
                "publication {field} disagrees with the firing bundle rules"
            ),
            Self::SummaryMismatch => write!(f, "pack summary counts disagree with the bundles"),
        }
    }
}

impl Error for SignedM4StableEvidencePackViolation {}

/// Loads the embedded signed M4 stable evidence pack.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in artifact no longer matches
/// [`SignedM4StableEvidencePack`].
pub fn current_signed_m4_stable_evidence_pack(
) -> Result<SignedM4StableEvidencePack, serde_json::Error> {
    serde_json::from_str(SIGNED_M4_STABLE_EVIDENCE_PACK_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pack() -> SignedM4StableEvidencePack {
        current_signed_m4_stable_evidence_pack().expect("pack parses")
    }

    #[test]
    fn embedded_pack_parses_and_validates() {
        let pack = pack();
        assert_eq!(
            pack.schema_version,
            SIGNED_M4_STABLE_EVIDENCE_PACK_SCHEMA_VERSION
        );
        assert_eq!(pack.record_kind, SIGNED_M4_STABLE_EVIDENCE_PACK_RECORD_KIND);
        assert_eq!(pack.validate(), Vec::new());
        assert!(!pack.bundles.is_empty());
    }

    #[test]
    fn pack_exercises_signed_and_narrowed_rows() {
        let pack = pack();
        assert!(
            !pack.rows_published_stable().is_empty(),
            "pack must show at least one published-stable row"
        );
        assert!(
            !pack.rows_narrowed().is_empty(),
            "pack must show at least one narrowed row"
        );
    }

    #[test]
    fn pack_exercises_unsigned_and_signature_invalid_rows() {
        let pack = pack();
        let unsigned = pack.bundles.iter().find(|r| {
            r.bundle_state == BundleState::UnsignedUnattested
                && r.has_active_reason(BundleGapReason::AttestationMissing)
        });
        assert!(
            unsigned.is_some(),
            "pack must contain at least one unsigned/unattested bundle"
        );
    }

    #[test]
    fn summary_counts_match_rows() {
        let pack = pack();
        assert_eq!(pack.summary, pack.computed_summary());
        assert_eq!(
            pack.summary.bundles_signed_current
                + pack.summary.bundles_signed_on_waiver
                + pack.summary.bundles_narrowed,
            pack.bundles.len()
        );
        assert_eq!(
            pack.summary.benchmark_bundles
                + pack.summary.compatibility_bundles
                + pack.summary.migration_bundles
                + pack.summary.accessibility_bundles
                + pack.summary.docs_help_bundles
                + pack.summary.security_bundles
                + pack.summary.artifact_graph_bundles
                + pack.summary.qualification_bundles
                + pack.summary.release_promotion_bundles
                + pack.summary.dependency_bundles,
            pack.bundles.len()
        );
        assert_eq!(
            pack.summary.packets_current
                + pack.summary.packets_due_for_refresh
                + pack.summary.packets_breached
                + pack.summary.packets_missing,
            pack.bundles.len()
        );
    }

    #[test]
    fn publication_holds_when_a_blocking_rule_fires() {
        let pack = pack();
        assert_eq!(pack.publication.decision, PromotionDecision::Hold);
        assert_eq!(
            pack.publication.decision,
            pack.computed_publication_decision()
        );
        assert!(!pack.publication.blocking_rule_ids.is_empty());
        assert!(!pack.publication.blocking_entry_ids.is_empty());
    }

    #[test]
    fn every_gap_reason_has_a_rule() {
        let pack = pack();
        let covered: BTreeSet<BundleGapReason> = pack
            .bundle_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in BundleGapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn no_row_publishes_wider_than_its_claim_ceiling() {
        let pack = pack();
        for row in &pack.bundles {
            assert!(
                row.effective_label.rank() <= row.claim_label.rank(),
                "{} publishes wider than its ceiling",
                row.entry_id
            );
        }
    }

    #[test]
    fn validate_flags_a_bundle_wider_than_ceiling() {
        let mut pack = pack();
        let row = pack
            .bundles
            .iter_mut()
            .find(|r| !r.publishes_stable() && r.claim_label == StableClaimLevel::Beta)
            .expect("a narrowed row under a beta ceiling exists");
        row.effective_label = StableClaimLevel::Stable;
        let entry_id = row.entry_id.clone();
        pack.summary = pack.computed_summary();
        assert!(pack.validate().iter().any(|v| matches!(
            v,
            SignedM4StableEvidencePackViolation::PublishedWiderThanClaim { entry_id: id, .. } if *id == entry_id
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut pack = pack();
        let row = pack
            .bundles
            .iter_mut()
            .find(|r| r.bundle_state == BundleState::NarrowedStale)
            .expect("a narrowed-stale row exists");
        row.effective_label = row.claim_label;
        pack.summary = pack.computed_summary();
        pack.publication.decision = pack.computed_publication_decision();
        pack.publication.blocking_rule_ids = pack.computed_blocking_rule_ids();
        pack.publication.blocking_entry_ids = pack.computed_blocking_entry_ids();
        assert!(pack.validate().iter().any(|v| matches!(
            v,
            SignedM4StableEvidencePackViolation::PublishedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_unsigned_bundle_without_attestation_reason() {
        let mut pack = pack();
        let row = pack
            .bundles
            .iter_mut()
            .find(|r| r.bundle_state == BundleState::UnsignedUnattested)
            .expect("an unsigned row exists");
        row.active_gap_reasons
            .retain(|r| *r != BundleGapReason::AttestationMissing);
        pack.summary = pack.computed_summary();
        assert!(pack.validate().iter().any(|v| matches!(
            v,
            SignedM4StableEvidencePackViolation::StateReasonIncoherent { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut pack = pack();
        pack.publication.decision = PromotionDecision::Proceed;
        assert!(pack.validate().iter().any(|v| matches!(
            v,
            SignedM4StableEvidencePackViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_backed_row_without_signoff() {
        let mut pack = pack();
        let row = pack
            .bundles
            .iter_mut()
            .find(|r| r.holds_label())
            .expect("a held row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        let entry_id = row.entry_id.clone();
        pack.summary = pack.computed_summary();
        assert!(pack
            .validate()
            .contains(&SignedM4StableEvidencePackViolation::HeldBundleWithoutSignoff { entry_id }));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let pack = pack();
        let projection = pack.support_export_projection();
        assert_eq!(projection.rows.len(), pack.bundles.len());
        assert_eq!(projection.publication_decision, pack.publication.decision);
        for (row, projected) in pack.bundles.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, projected.entry_id);
            assert_eq!(row.subject_ref, projected.subject_ref);
            assert_eq!(row.publishes_stable(), projected.publishes_stable);
            assert_eq!(row.effective_label, projected.published_label);
            assert_eq!(row.proof_packet.slo_state.as_str(), projected.slo_state);
        }
    }
}
