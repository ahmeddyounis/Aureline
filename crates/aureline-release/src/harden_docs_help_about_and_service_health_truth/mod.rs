//! Typed docs, Help, About, service-health, and package/dependency safety truth
//! register hardened against the claim manifest, version-match, and freshness
//! audits.
//!
//! Where the [`stable_claim_manifest`](crate::stable_claim_manifest) decides the
//! single canonical lifecycle label each *subject* publishes, the
//! [`stable_proof_index`](crate::stable_proof_index) decides whether each
//! launch-blocking *requirement* is proven, and the
//! [`stable_publication_pack`](crate::stable_publication_pack) governs each
//! outward-facing publication, this module answers the question: **for each
//! user-facing truth surface — docs, Help, About, service-health, and
//! package/dependency safety — is that surface actually backed by a fresh proof
//! packet, aligned with the claim manifest and version-match expectations, with
//! required trust-class labels on every public destination, and with package-safety
//! disclosures complete before write?** This module is the **docs/help/About/
//! service-health truth register**. For every surface it records one row that
//! binds the surface to the [`stable_claim_manifest`](crate::stable_claim_manifest)
//! entry whose lifecycle label it backs, the proof packet that grounds it, the
//! service-contract state (for service-health rows), the About provenance card
//! (for About rows), the package-safety disclosure (for package-safety rows),
//! the waiver (if any) holding it provisionally, and the owner sign-off.
//!
//! Each [`TruthRow`] is one `(surface, public claim)` binding. It:
//!
//! - names the surface it governs ([`TruthRow::surface_kind`],
//!   [`TruthRow::surface_ref`], [`TruthRow::surface_summary`]) and whether that
//!   surface is part of the release-blocking truth set
//!   ([`TruthRow::release_blocking`]);
//! - pins the proof packet ([`ProofPacket`]) with its packet-freshness SLO;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry
//!   whose public claim it backs ([`TruthRow::claim_ref`]) and the canonical
//!   lifecycle label that entry publishes ([`TruthRow::claim_label`]). That
//!   label is a hard **ceiling**: a surface may carry the claim's label or narrow
//!   below it, but it may never assert a public claim wider than the public claim
//!   it backs;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-surface
//!   labels, so docs, Help/About, the release center, and support exports ingest
//!   one label per surface instead of cloning their own;
//! - records the truth state earned ([`TruthState`]), the active gap reasons
//!   ([`GapReason`]), and the label it *effectively* publishes after narrowing
//!   ([`TruthRow::published_label`]);
//! - for service-health rows, exposes the stable [`ServiceContractState`]
//!   vocabulary (`ready`, `degraded`, `local_only`, `stale`, `contract_mismatch`,
//!   `policy_blocked`, `unavailable`) so desktop UI, CLI/headless output, docs
//!   packs, cached notices, and support exports share one vocabulary;
//! - for About rows, carries the [`AboutProvenanceCard`] with version, channel,
//!   install mode, provenance/build state, copy-build-info action, and grouped
//!   destinations labeled by [`DestinationTrustClass`] rather than generic
//!   external-link chrome;
//! - for package-safety rows, carries the [`PackageSafetyDisclosure`] with
//!   manifest scope, registry/auth source, script or native-build risk, lockfile
//!   impact, license/advisory delta, validation tasks, and rollback path.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between a surface whose backing supports a Stable public claim and one narrowed
//! below it. A surface that is not backed — because its proof packet aged out or is
//! missing, because its evidence is incomplete, because its claim manifest
//! alignment failed, because its waiver expired, because its owner sign-off is
//! missing, or because the public claim it backs is itself below the cutline — is
//! structurally required to drop below the cutline rather than inherit an adjacent
//! backed surface. The [`TruthRule`] set names the closed conditions that gate
//! publication, and [`DocsHelpAboutServiceHealthTruth::publication`] records the
//! proceed/hold verdict.
//!
//! The register is checked in at
//! `artifacts/release/harden_docs_help_about_and_service_health_truth.json` and
//! embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material. Two
//! classes of check live outside this model because they need more than the
//! register sees: date arithmetic (recomputing the packet-freshness state and
//! waiver expiry against an `as_of` date) and the cross-artifact ceiling check
//! (whether each row's `claim_label` still equals the label the stable claim
//! manifest publishes for the entry named by `claim_ref`). Those live in the CI
//! gate. This model enforces the structural and logical invariants that hold
//! regardless of the clock and the neighbouring artifact — the ceiling/no-widening
//! rule, narrowing consistency, packet/state coherence, owner sign-off on backed
//! rows, surface-kind and release-line coverage, publication-rule wiring, and the
//! verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported docs/help/About/service-health truth schema version.
pub const DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_RECORD_KIND: &str =
    "docs_help_about_service_health_truth";

/// Repo-relative path to the checked-in register.
pub const DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_PATH: &str =
    "artifacts/release/harden_docs_help_about_and_service_health_truth.json";

/// Embedded checked-in register JSON.
pub const DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/harden_docs_help_about_and_service_health_truth.json"
));

/// The user-facing truth surface a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKind {
    /// Documentation truth surface (README, onboarding, module docs, support
    /// articles).
    Docs,
    /// In-product Help surface.
    Help,
    /// About / provenance summary card.
    About,
    /// Service-health descriptors and cached notices.
    ServiceHealth,
    /// Package and dependency safety disclosures.
    PackageSafety,
}

impl SurfaceKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Docs,
        Self::Help,
        Self::About,
        Self::ServiceHealth,
        Self::PackageSafety,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Docs => "docs",
            Self::Help => "help",
            Self::About => "about",
            Self::ServiceHealth => "service_health",
            Self::PackageSafety => "package_safety",
        }
    }
}

/// The service-contract state vocabulary shared across docs, Help, About, and
/// service-health publication surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceContractState {
    /// Fully operational; all backing claims, qualifications, and proof packets
    /// are current.
    Ready,
    /// Operational with reduced capability; some backing evidence is stale or on
    /// waiver.
    Degraded,
    /// Functioning without managed reachability; offline or degraded continuity.
    LocalOnly,
    /// Backing proof packet or claim manifest has breached freshness SLO.
    Stale,
    /// The surface's published claim does not match the current claim manifest.
    ContractMismatch,
    /// A policy block prevents the surface from publishing at its claimed label.
    PolicyBlocked,
    /// The surface is not reachable or not backed by any current proof packet.
    Unavailable,
}

impl ServiceContractState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Ready,
        Self::Degraded,
        Self::LocalOnly,
        Self::Stale,
        Self::ContractMismatch,
        Self::PolicyBlocked,
        Self::Unavailable,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Degraded => "degraded",
            Self::LocalOnly => "local_only",
            Self::Stale => "stale",
            Self::ContractMismatch => "contract_mismatch",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unavailable => "unavailable",
        }
    }

    /// Whether the state lets a surface carry a public claim at or above the
    /// stable cutline.
    pub const fn supports_stable(self) -> bool {
        matches!(self, Self::Ready | Self::Degraded)
    }
}

/// The trust class of a public destination linked from Help/About surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestinationTrustClass {
    /// Official Aureline destination managed by the core team.
    Official,
    /// Community-managed destination (forums, user groups, unofficial docs).
    Community,
    /// Mirrored destination that replicates official content.
    Mirrored,
    /// Self-hosted destination under operator control.
    SelfHosted,
    /// Vendor-owned destination outside the open-source project.
    VendorOwned,
}

impl DestinationTrustClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Official,
        Self::Community,
        Self::Mirrored,
        Self::SelfHosted,
        Self::VendorOwned,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Official => "official",
            Self::Community => "community",
            Self::Mirrored => "mirrored",
            Self::SelfHosted => "self_hosted",
            Self::VendorOwned => "vendor_owned",
        }
    }
}

/// One destination exposed on the About/help provenance summary card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HelpDestination {
    /// The trust class of this destination.
    pub trust_class: DestinationTrustClass,
    /// Human-readable label for the destination.
    pub label: String,
    /// URL or deep-link ref to the destination.
    pub destination_ref: String,
}

/// The About/help provenance summary card exposed on claimed stable rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AboutProvenanceCard {
    /// Build version string.
    pub version: String,
    /// Release channel (stable, beta, preview, etc.).
    pub channel: String,
    /// Install mode (system, user, portable, etc.).
    pub install_mode: String,
    /// Build provenance state (signed, reproducible, etc.).
    pub provenance_state: String,
    /// Action label for copying build-info to clipboard.
    pub copy_build_info_action: String,
    /// Grouped destinations labeled by destination trust class.
    pub destinations: Vec<HelpDestination>,
}

/// Package/dependency safety disclosure for install/update/remove/resolve flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageSafetyDisclosure {
    /// The manifest scope this disclosure governs (workspace, global, etc.).
    pub manifest_scope: String,
    /// Registry and auth source description.
    pub registry_auth_source: String,
    /// Whether the package runs install/build scripts that carry risk.
    pub script_risk: bool,
    /// Whether the package requires native compilation that carries risk.
    pub native_build_risk: bool,
    /// Impact on the lockfile (add, update, remove, resolve).
    pub lockfile_impact: String,
    /// License and advisory delta description.
    pub license_advisory_delta: String,
    /// Validation tasks required before write.
    pub validation_tasks: Vec<String>,
    /// Rollback path if the operation fails or is revoked.
    pub rollback_path: String,
}

/// Truth state a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruthState {
    /// The surface is backed: a captured, within-SLO proof packet backs the public
    /// claim at its full canonical lifecycle label, owner-signed.
    Current,
    /// The surface carries the claim's full label only because an active,
    /// unexpired waiver covers a recorded gap.
    CurrentOnWaiver,
    /// The proof packet or row evidence is incomplete, or owner sign-off is
    /// absent; the surface is not backed and the label must narrow.
    NarrowedUnbacked,
    /// The public claim this surface backs is itself below the cutline, so the
    /// surface inherits that ceiling and narrows.
    NarrowedClaimNarrowed,
    /// The proof packet breached its freshness SLO (or is missing); the surface
    /// is not backed and the label must narrow.
    NarrowedStale,
    /// The surface relied on a waiver that has expired; the label must narrow.
    NarrowedWaiverExpired,
    /// The surface's claim manifest alignment or version-match failed; the label
    /// must narrow.
    NarrowedContractMismatch,
}

impl TruthState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Current,
        Self::CurrentOnWaiver,
        Self::NarrowedUnbacked,
        Self::NarrowedClaimNarrowed,
        Self::NarrowedStale,
        Self::NarrowedWaiverExpired,
        Self::NarrowedContractMismatch,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::CurrentOnWaiver => "current_on_waiver",
            Self::NarrowedUnbacked => "narrowed_unbacked",
            Self::NarrowedClaimNarrowed => "narrowed_claim_narrowed",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
            Self::NarrowedContractMismatch => "narrowed_contract_mismatch",
        }
    }

    /// Whether the state lets a surface carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Current | Self::CurrentOnWaiver)
    }

    /// Whether the state forces the surface below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a surface narrows or a truth rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapReason {
    /// The public claim this surface backs is itself below the cutline.
    ClaimLabelNarrowed,
    /// The proof packet breached its freshness SLO.
    ProofPacketFreshnessBreached,
    /// No proof packet has been captured for the surface.
    ProofPacketMissing,
    /// The surface's evidence (docs-maintenance packet, stale-example findings,
    /// service-health descriptor) is incomplete.
    EvidenceIncomplete,
    /// A waiver the surface relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
    /// The surface's published claim does not match the current claim manifest.
    ClaimManifestMismatch,
}

impl GapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ClaimLabelNarrowed,
        Self::ProofPacketFreshnessBreached,
        Self::ProofPacketMissing,
        Self::EvidenceIncomplete,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
        Self::ClaimManifestMismatch,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::ProofPacketFreshnessBreached => "proof_packet_freshness_breached",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::ClaimManifestMismatch => "claim_manifest_mismatch",
        }
    }
}

/// Default action a truth rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruthAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the surface's published lifecycle label below the cutline.
    NarrowLabel,
    /// Refresh the proof packet so it re-enters its freshness SLO.
    RefreshProofPacket,
    /// Recapture the evidence the proof packet depends on.
    RecaptureEvidence,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
    /// Re-test claim manifest alignment and version-match.
    RetestClaimAlignment,
}

impl TruthAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HoldPublication,
        Self::NarrowLabel,
        Self::RefreshProofPacket,
        Self::RecaptureEvidence,
        Self::RequestOwnerSignoff,
        Self::RetestClaimAlignment,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowLabel => "narrow_label",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::RetestClaimAlignment => "retest_claim_alignment",
        }
    }
}

/// One truth rule: a closed condition that narrows a surface label and may gate
/// publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TruthRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: GapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: TruthAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One docs/help/About/service-health truth row: a `(surface, public claim)`
/// binding bound to its proof packet, canonical ceiling label, and
/// packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TruthRow {
    /// Stable truth-row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The surface this row governs.
    pub surface_kind: SurfaceKind,
    /// The surface id this row speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the surface.
    pub surface_summary: String,
    /// Whether the surface is part of the release-blocking truth set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this surface backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: a
    /// surface may never carry a label wider than this.
    pub claim_label: StableClaimLevel,
    /// Truth state earned for the surface.
    pub truth_state: TruthState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// For service-health rows, the service-contract state.
    #[serde(default)]
    pub service_contract_state: Option<ServiceContractState>,
    /// For About rows, the provenance summary card.
    #[serde(default)]
    pub about_card: Option<AboutProvenanceCard>,
    /// For package-safety rows, the safety disclosure.
    #[serde(default)]
    pub package_safety: Option<PackageSafetyDisclosure>,
    /// Waiver authorizing a provisional surface, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
    /// The lifecycle label the surface effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl TruthRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the surface carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.truth_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: GapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// The recorded publication verdict for the truth register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TruthPublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Truth-rule ids that block publication, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Truth-row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_entry_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TruthSummary {
    /// Total number of truth rows.
    pub total_entries: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows publishing a label at or above the cutline.
    pub entries_published_stable: usize,
    /// Rows narrowed below the cutline.
    pub entries_narrowed_below_cutline: usize,
    /// Rows holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing a label at or above the cutline.
    pub release_blocking_published_stable: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Docs surfaces.
    pub docs_entries: usize,
    /// Help surfaces.
    pub help_entries: usize,
    /// About surfaces.
    pub about_entries: usize,
    /// Service-health surfaces.
    pub service_health_entries: usize,
    /// Package-safety surfaces.
    pub package_safety_entries: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of truth rules currently firing.
    pub truth_rules_firing: usize,
}

/// The typed docs/help/About/service-health truth register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocsHelpAboutServiceHealthTruth {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register identifier.
    pub register_id: String,
    /// Lifecycle status of this register artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this register ingests as its public-claim
    /// source and ceiling.
    pub claim_manifest_ref: String,
    /// Ref to the docs-maintenance packet every docs row rides.
    pub docs_maintenance_packet_ref: String,
    /// Ref to the service-health descriptor register every service-health row
    /// rides.
    pub service_health_register_ref: String,
    /// Ref to the package-safety disclosure template every package-safety row
    /// rides.
    pub package_safety_template_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed surface-kind vocabulary.
    pub surface_kinds: Vec<SurfaceKind>,
    /// Closed service-contract-state vocabulary.
    pub service_contract_states: Vec<ServiceContractState>,
    /// Closed destination-trust-class vocabulary.
    pub destination_trust_classes: Vec<DestinationTrustClass>,
    /// Closed truth-state vocabulary.
    pub truth_states: Vec<TruthState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<GapReason>,
    /// Closed truth-action vocabulary.
    pub truth_actions: Vec<TruthAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this register must cover.
    pub release_blocking_surface_refs: Vec<String>,
    /// Truth rules.
    pub truth_rules: Vec<TruthRule>,
    /// Truth rows.
    pub rows: Vec<TruthRow>,
    /// Recorded publication verdict.
    pub publication: TruthPublicationRecord,
    /// Summary counts.
    pub summary: TruthSummary,
}

impl DocsHelpAboutServiceHealthTruth {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&TruthRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&TruthRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&TruthRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&TruthRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one surface kind.
    pub fn rows_for_kind(&self, kind: SurfaceKind) -> Vec<&TruthRow> {
        self.rows
            .iter()
            .filter(|row| row.surface_kind == kind)
            .collect()
    }

    /// Distinct public claims (by claim ref) the register covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn truth_rule_fires(&self, rule: &TruthRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and truth rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .truth_rules
            .iter()
            .any(|rule| rule.blocks_publication && self.truth_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .truth_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.truth_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Truth-row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose
    /// claim is already canonically narrowed is not a *publication* blocker, it
    /// merely inherits the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<GapReason> = self
            .truth_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.truth_rule_fires(rule))
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

    /// Recomputes the summary block from the rows and truth rules.
    pub fn computed_summary(&self) -> TruthSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: SurfaceKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&TruthRow> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect();
        TruthSummary {
            total_entries: self.rows.len(),
            total_claims: self.claims().len(),
            entries_published_stable: self
                .rows
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            entries_narrowed_below_cutline: self
                .rows
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            entries_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.truth_state == TruthState::CurrentOnWaiver)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_published_stable: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            docs_entries: kind(SurfaceKind::Docs),
            help_entries: kind(SurfaceKind::Help),
            about_entries: kind(SurfaceKind::About),
            service_health_entries: kind(SurfaceKind::ServiceHealth),
            package_safety_entries: kind(SurfaceKind::PackageSafety),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            truth_rules_firing: self
                .truth_rules
                .iter()
                .filter(|rule| self.truth_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the register that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> TruthExportProjection {
        TruthExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| TruthExportRow {
                    entry_id: row.entry_id.clone(),
                    surface_kind: row.surface_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    truth_state: row.truth_state,
                    slo_state: row.proof_packet.slo_state,
                    service_contract_state: row.service_contract_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<DocsHelpAboutServiceHealthTruthViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(DocsHelpAboutServiceHealthTruthViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(
        &self,
        violations: &mut Vec<DocsHelpAboutServiceHealthTruthViolation>,
    ) {
        if self.schema_version != DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_SCHEMA_VERSION {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_RECORD_KIND {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            (
                "docs_maintenance_packet_ref",
                &self.docs_maintenance_packet_ref,
            ),
            (
                "service_health_register_ref",
                &self.service_health_register_ref,
            ),
            (
                "package_safety_template_ref",
                &self.package_safety_template_ref,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(DocsHelpAboutServiceHealthTruthViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.surface_kinds != SurfaceKind::ALL.to_vec() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::ClosedVocabularyMismatch {
                field: "surface_kinds",
            });
        }
        if self.service_contract_states != ServiceContractState::ALL.to_vec() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::ClosedVocabularyMismatch {
                field: "service_contract_states",
            });
        }
        if self.destination_trust_classes != DestinationTrustClass::ALL.to_vec() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::ClosedVocabularyMismatch {
                field: "destination_trust_classes",
            });
        }
        if self.truth_states != TruthState::ALL.to_vec() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::ClosedVocabularyMismatch {
                field: "truth_states",
            });
        }
        if self.gap_reasons != GapReason::ALL.to_vec() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.truth_actions != TruthAction::ALL.to_vec() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::ClosedVocabularyMismatch {
                field: "truth_actions",
            });
        }
        if self.release_blocking_surface_refs.is_empty() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::EmptyField {
                entry_id: "<register>".to_owned(),
                field_name: "release_blocking_surface_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(
        &self,
        violations: &mut Vec<DocsHelpAboutServiceHealthTruthViolation>,
    ) {
        if self.truth_rules.is_empty() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::NoTruthRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.truth_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(DocsHelpAboutServiceHealthTruthViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(DocsHelpAboutServiceHealthTruthViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(DocsHelpAboutServiceHealthTruthViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        // Every gap reason must have a rule, so a gap reason cannot fire without a
        // corresponding publication gate.
        for reason in GapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(DocsHelpAboutServiceHealthTruthViolation::GapReasonWithoutRule {
                    reason,
                });
            }
        }
    }

    fn validate_row(
        &self,
        row: &TruthRow,
        violations: &mut Vec<DocsHelpAboutServiceHealthTruthViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("surface_ref", &row.surface_ref),
            ("surface_summary", &row.surface_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            ("proof_packet.packet_id", &row.proof_packet.packet_id),
            ("proof_packet.packet_ref", &row.proof_packet.packet_ref),
            (
                "proof_packet.proof_index_ref",
                &row.proof_packet.proof_index_ref,
            ),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &row.proof_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(DocsHelpAboutServiceHealthTruthViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no surface may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(
                DocsHelpAboutServiceHealthTruthViolation::PublishedWiderThanClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                },
            );
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(
                DocsHelpAboutServiceHealthTruthViolation::FreshnessSloInconsistent {
                    entry_id: row.entry_id.clone(),
                },
            );
        }

        // Service-health rows must carry a service_contract_state.
        if row.surface_kind == SurfaceKind::ServiceHealth && row.service_contract_state.is_none() {
            violations.push(
                DocsHelpAboutServiceHealthTruthViolation::ServiceHealthWithoutContractState {
                    entry_id: row.entry_id.clone(),
                },
            );
        }

        // About rows must carry an about_card.
        if row.surface_kind == SurfaceKind::About && row.about_card.is_none() {
            violations.push(
                DocsHelpAboutServiceHealthTruthViolation::AboutWithoutProvenanceCard {
                    entry_id: row.entry_id.clone(),
                },
            );
        }

        // Package-safety rows must carry a package_safety disclosure.
        if row.surface_kind == SurfaceKind::PackageSafety && row.package_safety.is_none() {
            violations.push(
                DocsHelpAboutServiceHealthTruthViolation::PackageSafetyWithoutDisclosure {
                    entry_id: row.entry_id.clone(),
                },
            );
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A published label publishes exactly its claim, carries no active
            // gap reason, rides a captured packet within its freshness SLO, and
            // is owner-signed.
            if row.published_label != row.claim_label {
                violations.push(
                    DocsHelpAboutServiceHealthTruthViolation::HeldLabelNotEqualClaimed {
                        entry_id: row.entry_id.clone(),
                        claimed: row.claim_label,
                        published: row.published_label,
                    },
                );
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(
                    DocsHelpAboutServiceHealthTruthViolation::HeldWithActiveGap {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if !row.proof_packet.has_capture() {
                violations.push(
                    DocsHelpAboutServiceHealthTruthViolation::HeldWithoutFreshPacket {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if !slo_state.is_within_slo() {
                violations.push(
                    DocsHelpAboutServiceHealthTruthViolation::HeldOnStalePacket {
                        entry_id: row.entry_id.clone(),
                        slo_state,
                    },
                );
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(
                    DocsHelpAboutServiceHealthTruthViolation::HeldWithoutSignoff {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        } else {
            // A narrowing state must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(
                    DocsHelpAboutServiceHealthTruthViolation::PublishedLabelNotNarrowed {
                        entry_id: row.entry_id.clone(),
                        state: row.truth_state,
                        published: row.published_label,
                    },
                );
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(
                    DocsHelpAboutServiceHealthTruthViolation::NarrowingWithoutReason {
                        entry_id: row.entry_id.clone(),
                        state: row.truth_state,
                    },
                );
            }
            // A narrowing entry whose packet is breached or missing must name the
            // matching freshness reason, so the freshness automation stays honest.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(GapReason::ProofPacketFreshnessBreached)
            {
                violations.push(
                    DocsHelpAboutServiceHealthTruthViolation::BreachedPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(GapReason::ProofPacketMissing)
            {
                violations.push(
                    DocsHelpAboutServiceHealthTruthViolation::MissingPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_state_reason_coherence(
        &self,
        row: &TruthRow,
        violations: &mut Vec<DocsHelpAboutServiceHealthTruthViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<DocsHelpAboutServiceHealthTruthViolation>,
                               expected: GapReason| {
            violations.push(
                DocsHelpAboutServiceHealthTruthViolation::StateReasonIncoherent {
                    entry_id: row.entry_id.clone(),
                    state: row.truth_state,
                    expected_reason: expected,
                },
            );
        };

        match row.truth_state {
            TruthState::NarrowedUnbacked => {
                const ALLOWED: [GapReason; 3] = [
                    GapReason::EvidenceIncomplete,
                    GapReason::OwnerSignoffMissing,
                    GapReason::ClaimLabelNarrowed,
                ];
                if !ALLOWED.iter().any(|r| row.has_active_reason(*r)) {
                    push_incoherent(violations, GapReason::EvidenceIncomplete);
                }
            }
            TruthState::NarrowedStale => {
                if !(row.has_active_reason(GapReason::ProofPacketFreshnessBreached)
                    || row.has_active_reason(GapReason::ProofPacketMissing))
                {
                    push_incoherent(violations, GapReason::ProofPacketFreshnessBreached);
                }
            }
            TruthState::NarrowedWaiverExpired => {
                if !row.has_active_reason(GapReason::WaiverExpired) {
                    push_incoherent(violations, GapReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(
                        DocsHelpAboutServiceHealthTruthViolation::WaiverStateWithoutWaiver {
                            entry_id: row.entry_id.clone(),
                            state: row.truth_state,
                        },
                    );
                }
            }
            TruthState::NarrowedContractMismatch => {
                if !row.has_active_reason(GapReason::ClaimManifestMismatch) {
                    push_incoherent(violations, GapReason::ClaimManifestMismatch);
                }
            }
            TruthState::NarrowedClaimNarrowed => {
                if !row.has_active_reason(GapReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, GapReason::ClaimLabelNarrowed);
                }
            }
            TruthState::CurrentOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(
                        DocsHelpAboutServiceHealthTruthViolation::WaiverStateWithoutWaiver {
                            entry_id: row.entry_id.clone(),
                            state: row.truth_state,
                        },
                    );
                }
            }
            TruthState::Current => {}
        }
    }

    fn validate_coverage(
        &self,
        violations: &mut Vec<DocsHelpAboutServiceHealthTruthViolation>,
    ) {
        let covered: BTreeSet<String> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .map(|row| row.surface_ref.clone())
            .collect();
        for declared in &self.release_blocking_surface_refs {
            if !covered.contains(declared) {
                violations.push(
                    DocsHelpAboutServiceHealthTruthViolation::ReleaseBlockingSurfaceUncovered {
                        surface_ref: declared.clone(),
                    },
                );
            }
        }
    }

    fn validate_publication(
        &self,
        violations: &mut Vec<DocsHelpAboutServiceHealthTruthViolation>,
    ) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(DocsHelpAboutServiceHealthTruthViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                DocsHelpAboutServiceHealthTruthViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                DocsHelpAboutServiceHealthTruthViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_entry_ids != self.computed_blocking_entry_ids() {
            violations.push(
                DocsHelpAboutServiceHealthTruthViolation::PublicationBlockingSetMismatch {
                    field: "blocking_entry_ids",
                },
            );
        }
    }
}

/// Export/Help-About-safe projection of one truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TruthExportRow {
    /// Stable truth-row id.
    pub entry_id: String,
    /// The surface this row governs.
    pub surface_kind: SurfaceKind,
    /// The surface id this row speaks about.
    pub surface_ref: String,
    /// Whether the surface is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this surface backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// The lifecycle label the surface effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the published label is at or above the cutline.
    pub publishes_stable: bool,
    /// Truth state earned for the surface.
    pub truth_state: TruthState,
    /// Freshness-SLO state of the proof packet.
    pub slo_state: FreshnessSloState,
    /// For service-health rows, the service-contract state.
    #[serde(default)]
    pub service_contract_state: Option<ServiceContractState>,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
}

/// Export/Help-About-safe projection of the truth register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TruthExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Publication decision.
    pub publication_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<TruthExportRow>,
}

/// Every violation the typed model can detect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocsHelpAboutServiceHealthTruthViolation {
    /// The schema version does not match the model.
    UnsupportedSchemaVersion {
        /// Schema version found in the artifact.
        actual: u32,
    },
    /// The record kind does not match the model.
    UnsupportedRecordKind {
        /// Record kind found in the artifact.
        actual: String,
    },
    /// A required string field is empty or whitespace-only.
    EmptyField {
        /// Entry or pack id where the field lives.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A closed vocabulary field does not match the model's expected set.
    ClosedVocabularyMismatch {
        /// Field name.
        field: &'static str,
    },
    /// A truth rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A truth row id appears more than once.
    DuplicateEntryId {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// No truth rules are defined.
    NoTruthRules,
    /// A truth rule watches no labels.
    RuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no rule watching for it.
    GapReasonWithoutRule {
        /// Uncovered gap reason.
        reason: GapReason,
    },
    /// The register contains no rows.
    EmptyRegister,
    /// A release-blocking surface ref has no covering row.
    ReleaseBlockingSurfaceUncovered {
        /// Uncovered surface ref.
        surface_ref: String,
    },
    /// A row's published label is wider than its claim's canonical label.
    PublishedWiderThanClaim {
        /// Entry id.
        entry_id: String,
        /// Claim label.
        claim: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// The freshness SLO warn window exceeds the target age.
    FreshnessSloInconsistent {
        /// Entry id.
        entry_id: String,
    },
    /// A service-health row lacks a service_contract_state.
    ServiceHealthWithoutContractState {
        /// Entry id.
        entry_id: String,
    },
    /// An About row lacks an about_card.
    AboutWithoutProvenanceCard {
        /// Entry id.
        entry_id: String,
    },
    /// A package-safety row lacks a package_safety disclosure.
    PackageSafetyWithoutDisclosure {
        /// Entry id.
        entry_id: String,
    },
    /// A backed row's published label does not equal its claimed label.
    HeldLabelNotEqualClaimed {
        /// Entry id.
        entry_id: String,
        /// Claimed label.
        claimed: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A backed row carries an active gap reason.
    HeldWithActiveGap {
        /// Entry id.
        entry_id: String,
    },
    /// A backed row has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Entry id.
        entry_id: String,
    },
    /// A backed row rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Entry id.
        entry_id: String,
        /// SLO state.
        slo_state: FreshnessSloState,
    },
    /// A backed row lacks owner sign-off.
    HeldWithoutSignoff {
        /// Entry id.
        entry_id: String,
    },
    /// A narrowing row still publishes a label at or above the cutline.
    PublishedLabelNotNarrowed {
        /// Entry id.
        entry_id: String,
        /// State.
        state: TruthState,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A narrowing row has no active gap reason.
    NarrowingWithoutReason {
        /// Entry id.
        entry_id: String,
        /// State.
        state: TruthState,
    },
    /// A breached-packet row does not name the freshness breach reason.
    BreachedPacketWithoutReason {
        /// Entry id.
        entry_id: String,
    },
    /// A missing-packet row does not name the missing-packet reason.
    MissingPacketWithoutReason {
        /// Entry id.
        entry_id: String,
    },
    /// A state and its active gap reasons are incoherent.
    StateReasonIncoherent {
        /// Entry id.
        entry_id: String,
        /// State.
        state: TruthState,
        /// Expected reason.
        expected_reason: GapReason,
    },
    /// A waiver-bearing state has no waiver.
    WaiverStateWithoutWaiver {
        /// Entry id.
        entry_id: String,
        /// State.
        state: TruthState,
    },
    /// The declared publication decision does not match the computed decision.
    PublicationDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// A publication blocking set does not match the computed set.
    PublicationBlockingSetMismatch {
        /// Field name.
        field: &'static str,
    },
    /// The summary block does not match the computed summary.
    SummaryMismatch,
}

impl Error for DocsHelpAboutServiceHealthTruthViolation {}

impl fmt::Display for DocsHelpAboutServiceHealthTruthViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported schema version: {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported record kind: {actual}")
            }
            Self::EmptyField {
                entry_id,
                field_name,
            } => {
                write!(f, "empty field '{field_name}' in '{entry_id}'")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "closed vocabulary mismatch: {field}")
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate rule id: {rule_id}")
            }
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id: {entry_id}")
            }
            Self::NoTruthRules => {
                write!(f, "no truth rules defined")
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "rule '{rule_id}' watches no labels")
            }
            Self::GapReasonWithoutRule { reason } => {
                write!(f, "gap reason '{}' has no rule", reason.as_str())
            }
            Self::EmptyRegister => {
                write!(f, "register contains no rows")
            }
            Self::ReleaseBlockingSurfaceUncovered { surface_ref } => {
                write!(f, "release-blocking surface '{surface_ref}' is uncovered")
            }
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => {
                write!(
                    f,
                    "'{entry_id}' published label '{published:?}' is wider than claim '{claim:?}'"
                )
            }
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "'{entry_id}' freshness SLO is inconsistent")
            }
            Self::ServiceHealthWithoutContractState { entry_id } => {
                write!(
                    f,
                    "'{entry_id}' is a service-health row without a service_contract_state"
                )
            }
            Self::AboutWithoutProvenanceCard { entry_id } => {
                write!(f, "'{entry_id}' is an About row without an about_card")
            }
            Self::PackageSafetyWithoutDisclosure { entry_id } => {
                write!(f, "'{entry_id}' is a package-safety row without a package_safety disclosure")
            }
            Self::HeldLabelNotEqualClaimed {
                entry_id,
                claimed,
                published,
            } => {
                write!(
                    f,
                    "'{entry_id}' held label '{published:?}' does not equal claimed '{claimed:?}'"
                )
            }
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "'{entry_id}' is held but carries an active gap reason")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "'{entry_id}' is held but has no fresh proof packet")
            }
            Self::HeldOnStalePacket { entry_id, slo_state } => {
                write!(
                    f,
                    "'{entry_id}' is held but rides a stale packet ({slo_state:?})"
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "'{entry_id}' is held but lacks owner sign-off")
            }
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => {
                write!(
                    f,
                    "'{entry_id}' state '{state:?}' forces narrowing but publishes '{published:?}'"
                )
            }
            Self::NarrowingWithoutReason { entry_id, state } => {
                write!(f, "'{entry_id}' state '{state:?}' narrows without an active reason")
            }
            Self::BreachedPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "'{entry_id}' has a breached packet but no freshness breach reason"
                )
            }
            Self::MissingPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "'{entry_id}' has a missing packet but no missing-packet reason"
                )
            }
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => {
                write!(
                    f,
                    "'{entry_id}' state '{state:?}' is incoherent (expected reason '{expected_reason:?}')"
                )
            }
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(
                    f,
                    "'{entry_id}' state '{state:?}' expects a waiver but none is present"
                )
            }
            Self::PublicationDecisionInconsistent { declared, computed } => {
                write!(
                    f,
                    "publication decision '{declared:?}' does not match computed '{computed:?}'"
                )
            }
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication blocking set mismatch: {field}")
            }
            Self::SummaryMismatch => {
                write!(f, "summary block does not match computed summary")
            }
        }
    }
}

/// Parse the embedded checked-in register JSON.
pub fn current_docs_help_about_service_health_truth(
) -> Result<DocsHelpAboutServiceHealthTruth, Box<dyn Error>> {
    let parsed: DocsHelpAboutServiceHealthTruth = serde_json::from_str(
        DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_JSON,
    )?;
    Ok(parsed)
}
