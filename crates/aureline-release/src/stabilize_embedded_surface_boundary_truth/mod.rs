//! Typed embedded-surface boundary truth register hardened against the claim
//! manifest, owner/origin chrome, native-approval boundary, and system-browser
//! auth integrity audits.
//!
//! Where the [`stable_claim_manifest`](crate::stable_claim_manifest) decides the
//! single canonical lifecycle label each *subject* publishes, this module answers
//! the question: **for every embedded surface family — docs/help panes, extension
//! webviews, marketplace/account pages, service dashboards, and auth confirmation
//! surfaces — does that surface carry owner/origin chrome, service-boundary truth,
//! a native-approval fence that keeps high-risk actions product-owned, and a
//! system-browser auth default with an honest browser fallback?**
//!
//! Each [`TruthRow`] is one `(surface, public claim)` binding. It:
//!
//! - names the surface it governs and whether it is release-blocking;
//! - pins the proof packet with its packet-freshness SLO;
//! - names the claim manifest entry whose public claim it backs and the canonical
//!   lifecycle label that is a hard ceiling;
//! - records the boundary state earned ([`BoundaryState`]), the truth state
//!   ([`TruthState`]), active gap reasons ([`GapReason`]), and the effectively
//!   published label;
//! - for docs/help rows, carries a [`SourceTruthSnapshot`] proving source class,
//!   version match, and freshness are disclosed;
//! - for all rows, carries a [`BrowserFallbackSnapshot`] proving open-in-browser
//!   fallback posture and object-identity preservation;
//! - for all rows, carries a [`NativeApprovalSnapshot`] proving high-risk
//!   approvals, destructive confirmations, trust elevation, update verification,
//!   and AI apply review remain host-owned;
//! - for auth rows, carries an [`AuthHandoffSnapshot`] proving system-browser
//!   default, return-path labeling, exact-target preservation, and exception
//!   disclosure.
//!
//! The register is checked in at
//! `artifacts/release/stabilize_embedded_surface_boundary_truth.json` and
//! embedded here.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported embedded-surface boundary truth schema version.
pub const EMBEDDED_SURFACE_BOUNDARY_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const EMBEDDED_SURFACE_BOUNDARY_TRUTH_RECORD_KIND: &str = "embedded_surface_boundary_truth";

/// Repo-relative path to the checked-in register.
pub const EMBEDDED_SURFACE_BOUNDARY_TRUTH_PATH: &str =
    "artifacts/release/stabilize_embedded_surface_boundary_truth.json";

/// Embedded checked-in register JSON.
pub const EMBEDDED_SURFACE_BOUNDARY_TRUTH_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/stabilize_embedded_surface_boundary_truth.json"
));

/// The embedded surface family a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKind {
    /// In-product docs and help panes.
    EmbeddedDocsHelp,
    /// Extension-hosted webview-like surfaces.
    ExtensionHostedSurface,
    /// Marketplace, account, billing, or org pages.
    EmbeddedMarketplaceOrAccount,
    /// Service dashboards and provider-owned hosted panes.
    EmbeddedServiceDashboard,
    /// Auth confirmation and handoff surfaces.
    EmbeddedAuthConfirmation,
}

impl SurfaceKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EmbeddedDocsHelp,
        Self::ExtensionHostedSurface,
        Self::EmbeddedMarketplaceOrAccount,
        Self::EmbeddedServiceDashboard,
        Self::EmbeddedAuthConfirmation,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmbeddedDocsHelp => "embedded_docs_help",
            Self::ExtensionHostedSurface => "extension_hosted_surface",
            Self::EmbeddedMarketplaceOrAccount => "embedded_marketplace_or_account",
            Self::EmbeddedServiceDashboard => "embedded_service_dashboard",
            Self::EmbeddedAuthConfirmation => "embedded_auth_confirmation",
        }
    }
}

/// Boundary state vocabulary shared across embedded surface families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryState {
    /// Live, verified, current boundary.
    LiveVerified,
    /// Content is shown from a stale snapshot.
    StaleSnapshot,
    /// Policy blocks the embedded surface.
    PolicyBlocked,
    /// Certificate or trust-store validation failed.
    CertificateFailed,
    /// Cross-origin limitations apply.
    CrossOriginLimited,
    /// Offline or mirror-only snapshot.
    OfflineSnapshot,
    /// External open only; embedded body is withheld.
    ExternalOpenOnly,
}

impl BoundaryState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LiveVerified,
        Self::StaleSnapshot,
        Self::PolicyBlocked,
        Self::CertificateFailed,
        Self::CrossOriginLimited,
        Self::OfflineSnapshot,
        Self::ExternalOpenOnly,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveVerified => "live_verified",
            Self::StaleSnapshot => "stale_snapshot",
            Self::PolicyBlocked => "policy_blocked",
            Self::CertificateFailed => "certificate_failed",
            Self::CrossOriginLimited => "cross_origin_limited",
            Self::OfflineSnapshot => "offline_snapshot",
            Self::ExternalOpenOnly => "external_open_only",
        }
    }

    /// Whether the state lets a surface carry a stable public claim.
    pub const fn supports_stable(self) -> bool {
        matches!(self, Self::LiveVerified)
    }
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
    /// The surface's evidence is incomplete.
    EvidenceIncomplete,
    /// A waiver the surface relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
    /// The surface's published claim does not match the current claim manifest.
    ClaimManifestMismatch,
    /// Owner or origin chrome is missing on an embedded surface.
    OwnerOriginChromeMissing,
    /// A high-risk approval boundary leaked to an embedded surface.
    NativeApprovalBoundaryLeaked,
    /// System-browser auth default is missing on an identity or risky-web row.
    SystemBrowserDefaultMissing,
    /// Browser fallback or open-in-browser path is unavailable.
    BrowserFallbackUnavailable,
}

impl GapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ClaimLabelNarrowed,
        Self::ProofPacketFreshnessBreached,
        Self::ProofPacketMissing,
        Self::EvidenceIncomplete,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
        Self::ClaimManifestMismatch,
        Self::OwnerOriginChromeMissing,
        Self::NativeApprovalBoundaryLeaked,
        Self::SystemBrowserDefaultMissing,
        Self::BrowserFallbackUnavailable,
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
            Self::OwnerOriginChromeMissing => "owner_origin_chrome_missing",
            Self::NativeApprovalBoundaryLeaked => "native_approval_boundary_leaked",
            Self::SystemBrowserDefaultMissing => "system_browser_default_missing",
            Self::BrowserFallbackUnavailable => "browser_fallback_unavailable",
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

/// Source-truth snapshot for docs/help rows proving source class, version match,
/// and freshness disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceTruthSnapshot {
    /// Source class token (e.g. project_docs, mirrored_official_docs).
    pub source_class: String,
    /// Version match state token.
    pub version_match_state: String,
    /// Freshness class token.
    pub freshness_class: String,
    /// Human-readable snapshot age or freshness label.
    pub snapshot_age_label: String,
}

/// Browser-fallback snapshot proving open-in-browser posture and object-identity
/// preservation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BrowserFallbackSnapshot {
    /// Fallback posture class token.
    pub posture_class: String,
    /// Fallback target class token.
    pub fallback_target_class: String,
    /// Whether an open-in-browser action is available.
    pub open_in_browser_available: bool,
    /// Whether the fallback preserves exact object identity.
    pub preserves_object_identity: bool,
}

/// Native-approval snapshot proving high-risk actions remain host-owned.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeApprovalSnapshot {
    /// High-risk approval sheets remain product-owned.
    pub high_risk_approval_host_owned: bool,
    /// Destructive confirmations remain product-owned.
    pub destructive_confirmation_host_owned: bool,
    /// Workspace trust elevation remains product-owned.
    pub trust_elevation_host_owned: bool,
    /// Update verification remains product-owned.
    pub update_verification_host_owned: bool,
    /// AI apply review remains product-owned.
    pub ai_apply_review_host_owned: bool,
}

/// Auth-handoff snapshot for auth confirmation rows proving system-browser
/// default, return-path labeling, and exception disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthHandoffSnapshot {
    /// Auth defaults to system browser.
    pub system_browser_default: bool,
    /// Return path carries exact target labeling.
    pub return_path_labeled: bool,
    /// Exact target object identity is preserved across handoff.
    pub exact_target_preserved: bool,
    /// Any embedded exception is visibly disclosed.
    pub exception_disclosed: bool,
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

/// One embedded-surface boundary truth row: a `(surface, public claim)` binding
/// bound to its proof packet, boundary state, native-approval fence, and
/// browser-fallback posture.
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
    /// The canonical lifecycle label the public claim publishes. The ceiling.
    pub claim_label: StableClaimLevel,
    /// Truth state earned for the surface.
    pub truth_state: TruthState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Boundary state earned for the surface.
    pub boundary_state: BoundaryState,
    /// For docs/help rows, the source-truth snapshot.
    #[serde(default)]
    pub source_truth: Option<SourceTruthSnapshot>,
    /// Browser-fallback posture snapshot.
    pub browser_fallback: BrowserFallbackSnapshot,
    /// Native-approval boundary snapshot.
    pub native_approval: NativeApprovalSnapshot,
    /// For auth rows, the auth-handoff snapshot.
    #[serde(default)]
    pub auth_handoff: Option<AuthHandoffSnapshot>,
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

/// The recorded publication verdict for the boundary truth register.
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
    /// Docs/help surfaces.
    pub docs_help_entries: usize,
    /// Extension-hosted surfaces.
    pub extension_entries: usize,
    /// Marketplace/account surfaces.
    pub marketplace_entries: usize,
    /// Service-dashboard surfaces.
    pub service_dashboard_entries: usize,
    /// Auth-confirmation surfaces.
    pub auth_confirmation_entries: usize,
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

/// The typed embedded-surface boundary truth register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmbeddedSurfaceBoundaryTruth {
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
    /// Ref to the embedded-boundary alpha snapshot this register consumes.
    pub embedded_boundary_alpha_ref: String,
    /// Ref to the boundary-fallback alpha packet this register consumes.
    pub browser_fallback_alpha_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed surface-kind vocabulary.
    pub surface_kinds: Vec<SurfaceKind>,
    /// Closed boundary-state vocabulary.
    pub boundary_states: Vec<BoundaryState>,
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

impl EmbeddedSurfaceBoundaryTruth {
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
            docs_help_entries: kind(SurfaceKind::EmbeddedDocsHelp),
            extension_entries: kind(SurfaceKind::ExtensionHostedSurface),
            marketplace_entries: kind(SurfaceKind::EmbeddedMarketplaceOrAccount),
            service_dashboard_entries: kind(SurfaceKind::EmbeddedServiceDashboard),
            auth_confirmation_entries: kind(SurfaceKind::EmbeddedAuthConfirmation),
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

    /// Produces an export-safe projection of the register.
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
                    boundary_state: row.boundary_state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<EmbeddedSurfaceBoundaryTruthViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(
                    EmbeddedSurfaceBoundaryTruthViolation::DuplicateEntryId {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(EmbeddedSurfaceBoundaryTruthViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(EmbeddedSurfaceBoundaryTruthViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(
        &self,
        violations: &mut Vec<EmbeddedSurfaceBoundaryTruthViolation>,
    ) {
        if self.schema_version != EMBEDDED_SURFACE_BOUNDARY_TRUTH_SCHEMA_VERSION {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != EMBEDDED_SURFACE_BOUNDARY_TRUTH_RECORD_KIND {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            (
                "embedded_boundary_alpha_ref",
                &self.embedded_boundary_alpha_ref,
            ),
            (
                "browser_fallback_alpha_ref",
                &self.browser_fallback_alpha_ref,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(EmbeddedSurfaceBoundaryTruthViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::ClosedVocabularyMismatch {
                    field: "lifecycle_labels",
                },
            );
        }
        if self.surface_kinds != SurfaceKind::ALL.to_vec() {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::ClosedVocabularyMismatch {
                    field: "surface_kinds",
                },
            );
        }
        if self.boundary_states != BoundaryState::ALL.to_vec() {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::ClosedVocabularyMismatch {
                    field: "boundary_states",
                },
            );
        }
        if self.truth_states != TruthState::ALL.to_vec() {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::ClosedVocabularyMismatch {
                    field: "truth_states",
                },
            );
        }
        if self.gap_reasons != GapReason::ALL.to_vec() {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::ClosedVocabularyMismatch {
                    field: "gap_reasons",
                },
            );
        }
        if self.truth_actions != TruthAction::ALL.to_vec() {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::ClosedVocabularyMismatch {
                    field: "truth_actions",
                },
            );
        }
        if self.release_blocking_surface_refs.is_empty() {
            violations.push(EmbeddedSurfaceBoundaryTruthViolation::EmptyField {
                entry_id: "<register>".to_owned(),
                field_name: "release_blocking_surface_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.cutline_level",
                },
            );
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.above_cutline_levels",
                },
            );
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.below_cutline_levels",
                },
            );
        }
        if cutline.description.trim().is_empty() {
            violations.push(EmbeddedSurfaceBoundaryTruthViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(
        &self,
        violations: &mut Vec<EmbeddedSurfaceBoundaryTruthViolation>,
    ) {
        if self.truth_rules.is_empty() {
            violations.push(EmbeddedSurfaceBoundaryTruthViolation::NoTruthRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.truth_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(
                    EmbeddedSurfaceBoundaryTruthViolation::DuplicateRuleId {
                        rule_id: rule.rule_id.clone(),
                    },
                );
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(EmbeddedSurfaceBoundaryTruthViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(
                    EmbeddedSurfaceBoundaryTruthViolation::RuleWithoutLabels {
                        rule_id: rule.rule_id.clone(),
                    },
                );
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in GapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(
                    EmbeddedSurfaceBoundaryTruthViolation::GapReasonWithoutRule {
                        reason,
                    },
                );
            }
        }
    }

    fn validate_row(
        &self,
        row: &TruthRow,
        violations: &mut Vec<EmbeddedSurfaceBoundaryTruthViolation>,
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
                violations.push(EmbeddedSurfaceBoundaryTruthViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // Ceiling: no surface may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::PublishedWiderThanClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                },
            );
        }

        // Freshness SLO consistency.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(EmbeddedSurfaceBoundaryTruthViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::FreshnessSloInconsistent {
                    entry_id: row.entry_id.clone(),
                },
            );
        }

        // Embedded-docs/help rows must carry a source_truth snapshot.
        if row.surface_kind == SurfaceKind::EmbeddedDocsHelp && row.source_truth.is_none() {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::DocsHelpWithoutSourceTruth {
                    entry_id: row.entry_id.clone(),
                },
            );
        }

        // Auth-confirmation rows must carry an auth_handoff snapshot.
        if row.surface_kind == SurfaceKind::EmbeddedAuthConfirmation
            && row.auth_handoff.is_none()
        {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::AuthWithoutHandoffSnapshot {
                    entry_id: row.entry_id.clone(),
                },
            );
        }

        // All rows must keep high-risk approval host-owned.
        if !row.native_approval.high_risk_approval_host_owned {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::NativeApprovalLeaked {
                    entry_id: row.entry_id.clone(),
                    field: "high_risk_approval_host_owned",
                },
            );
        }
        if !row.native_approval.destructive_confirmation_host_owned {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::NativeApprovalLeaked {
                    entry_id: row.entry_id.clone(),
                    field: "destructive_confirmation_host_owned",
                },
            );
        }
        if !row.native_approval.trust_elevation_host_owned {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::NativeApprovalLeaked {
                    entry_id: row.entry_id.clone(),
                    field: "trust_elevation_host_owned",
                },
            );
        }
        if !row.native_approval.update_verification_host_owned {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::NativeApprovalLeaked {
                    entry_id: row.entry_id.clone(),
                    field: "update_verification_host_owned",
                },
            );
        }
        if !row.native_approval.ai_apply_review_host_owned {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::NativeApprovalLeaked {
                    entry_id: row.entry_id.clone(),
                    field: "ai_apply_review_host_owned",
                },
            );
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A held row must publish exactly its claim, carry no active gap
            // reason, ride a captured packet within SLO, and be owner-signed.
            if row.published_label != row.claim_label {
                violations.push(
                    EmbeddedSurfaceBoundaryTruthViolation::HeldLabelNotEqualClaimed {
                        entry_id: row.entry_id.clone(),
                        claimed: row.claim_label,
                        published: row.published_label,
                    },
                );
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(
                    EmbeddedSurfaceBoundaryTruthViolation::HeldWithActiveGap {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if !row.proof_packet.has_capture() {
                violations.push(
                    EmbeddedSurfaceBoundaryTruthViolation::HeldWithoutFreshPacket {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if !slo_state.is_within_slo() {
                violations.push(
                    EmbeddedSurfaceBoundaryTruthViolation::HeldOnStalePacket {
                        entry_id: row.entry_id.clone(),
                        slo_state,
                    },
                );
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(
                    EmbeddedSurfaceBoundaryTruthViolation::HeldWithoutSignoff {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        } else {
            // A narrowing state must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(
                    EmbeddedSurfaceBoundaryTruthViolation::PublishedLabelNotNarrowed {
                        entry_id: row.entry_id.clone(),
                        state: row.truth_state,
                        published: row.published_label,
                    },
                );
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(
                    EmbeddedSurfaceBoundaryTruthViolation::NarrowingWithoutReason {
                        entry_id: row.entry_id.clone(),
                        state: row.truth_state,
                    },
                );
            }
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(GapReason::ProofPacketFreshnessBreached)
            {
                violations.push(
                    EmbeddedSurfaceBoundaryTruthViolation::BreachedPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(GapReason::ProofPacketMissing)
            {
                violations.push(
                    EmbeddedSurfaceBoundaryTruthViolation::MissingPacketWithoutReason {
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
        violations: &mut Vec<EmbeddedSurfaceBoundaryTruthViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<EmbeddedSurfaceBoundaryTruthViolation>,
                               expected: GapReason| {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::StateReasonIncoherent {
                    entry_id: row.entry_id.clone(),
                    state: row.truth_state,
                    expected_reason: expected,
                },
            );
        };

        match row.truth_state {
            TruthState::NarrowedUnbacked => {
                const ALLOWED: [GapReason; 5] = [
                    GapReason::EvidenceIncomplete,
                    GapReason::OwnerSignoffMissing,
                    GapReason::ClaimLabelNarrowed,
                    GapReason::OwnerOriginChromeMissing,
                    GapReason::BrowserFallbackUnavailable,
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
                        EmbeddedSurfaceBoundaryTruthViolation::WaiverStateWithoutWaiver {
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
                        EmbeddedSurfaceBoundaryTruthViolation::WaiverStateWithoutWaiver {
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
        violations: &mut Vec<EmbeddedSurfaceBoundaryTruthViolation>,
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
                    EmbeddedSurfaceBoundaryTruthViolation::ReleaseBlockingSurfaceUncovered {
                        surface_ref: declared.clone(),
                    },
                );
            }
        }
    }

    fn validate_publication(
        &self,
        violations: &mut Vec<EmbeddedSurfaceBoundaryTruthViolation>,
    ) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(EmbeddedSurfaceBoundaryTruthViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(EmbeddedSurfaceBoundaryTruthViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_entry_ids != self.computed_blocking_entry_ids() {
            violations.push(
                EmbeddedSurfaceBoundaryTruthViolation::PublicationBlockingSetMismatch {
                    field: "blocking_entry_ids",
                },
            );
        }
    }
}

/// Export-safe projection of one truth row.
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
    /// Boundary state earned for the surface.
    pub boundary_state: BoundaryState,
    /// Freshness-SLO state of the proof packet.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
}

/// Export-safe projection of the boundary truth register.
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
pub enum EmbeddedSurfaceBoundaryTruthViolation {
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
    /// A docs/help row lacks a source_truth snapshot.
    DocsHelpWithoutSourceTruth {
        /// Entry id.
        entry_id: String,
    },
    /// An auth row lacks an auth_handoff snapshot.
    AuthWithoutHandoffSnapshot {
        /// Entry id.
        entry_id: String,
    },
    /// A native approval boundary leaked to an embedded surface.
    NativeApprovalLeaked {
        /// Entry id.
        entry_id: String,
        /// Field that leaked.
        field: &'static str,
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

impl Error for EmbeddedSurfaceBoundaryTruthViolation {}

impl fmt::Display for EmbeddedSurfaceBoundaryTruthViolation {
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
            Self::DocsHelpWithoutSourceTruth { entry_id } => {
                write!(
                    f,
                    "'{entry_id}' is a docs/help row without a source_truth snapshot"
                )
            }
            Self::AuthWithoutHandoffSnapshot { entry_id } => {
                write!(
                    f,
                    "'{entry_id}' is an auth row without an auth_handoff snapshot"
                )
            }
            Self::NativeApprovalLeaked { entry_id, field } => {
                write!(
                    f,
                    "'{entry_id}' native approval leaked: {field} is not host-owned"
                )
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
pub fn current_embedded_surface_boundary_truth(
) -> Result<EmbeddedSurfaceBoundaryTruth, Box<dyn Error>> {
    let parsed: EmbeddedSurfaceBoundaryTruth =
        serde_json::from_str(EMBEDDED_SURFACE_BOUNDARY_TRUTH_JSON)?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_register_parses_and_validates() {
        let reg = current_embedded_surface_boundary_truth()
            .expect("checked-in embedded surface boundary truth register parses into the model");
        assert_eq!(reg.schema_version, EMBEDDED_SURFACE_BOUNDARY_TRUTH_SCHEMA_VERSION);
        assert_eq!(reg.record_kind, EMBEDDED_SURFACE_BOUNDARY_TRUTH_RECORD_KIND);
        let violations = reg.validate();
        assert!(
            violations.is_empty(),
            "checked-in register must validate cleanly: {violations:#?}"
        );
    }

    #[test]
    fn covers_every_surface_kind() {
        let reg = current_embedded_surface_boundary_truth().unwrap();
        for kind in SurfaceKind::ALL {
            assert!(
                !reg.rows_for_kind(kind).is_empty(),
                "surface kind {} must have at least one row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn covers_every_boundary_state() {
        let reg = current_embedded_surface_boundary_truth().unwrap();
        let covered: Vec<BoundaryState> = reg
            .rows
            .iter()
            .map(|row| row.boundary_state)
            .collect();
        for state in BoundaryState::ALL {
            assert!(
                covered.contains(&state),
                "boundary state {} must appear on at least one row",
                state.as_str()
            );
        }
    }

    #[test]
    fn covers_every_declared_release_blocking_surface() {
        let reg = current_embedded_surface_boundary_truth().unwrap();
        assert!(!reg.release_blocking_surface_refs.is_empty());
        let covered: Vec<&str> = reg
            .release_blocking_rows()
            .into_iter()
            .map(|row| row.surface_ref.as_str())
            .collect();
        for declared in &reg.release_blocking_surface_refs {
            assert!(
                covered.contains(&declared.as_str()),
                "{declared} has no covering release-blocking row"
            );
        }
    }
}
