//! Typed stable claim manifest, canonical lifecycle labels, and packet-freshness
//! SLO automation.
//!
//! Where the [`stable_claim_matrix`](crate::stable_claim_matrix) decides which
//! subjects may publish as *Stable*, the
//! [`stable_qualification_matrix`](crate::stable_qualification_matrix) grounds
//! those claims in per-lane qualification rows, and the
//! [`support_class_ledger`](crate::support_class_ledger) publishes the v1.0
//! support class, this module is the **publication manifest** that binds those
//! three sources into one canonical record per published subject. It is the
//! single place that:
//!
//! - assigns each subject exactly one canonical **lifecycle label**
//!   ([`ManifestEntry::published_label`]) — reusing the
//!   [`StableClaimLevel`] vocabulary rather than re-minting lifecycle labels — so
//!   docs, Help/About, shiproom dashboards, and support exports ingest one label
//!   instead of each cloning their own;
//! - names the backing stable-claim row, qualification rows, and support-class
//!   entry the label depends on, so the manifest *ingests* the stable line rather
//!   than restating it; and
//! - attaches a **packet-freshness SLO** ([`FreshnessSlo`]) to every entry's
//!   proof packet and records the SLO state ([`FreshnessSloState`]) earned, so a
//!   subject whose proof packet has breached its freshness SLO narrows below the
//!   launch cutline automatically before publication.
//!
//! Each [`ManifestEntry`] carries the label it is put forward as
//! ([`ManifestEntry::claimed_label`]), the manifest state earned
//! ([`ManifestState`]), the active narrowing reasons ([`NarrowingReason`]), and
//! the label it *effectively* publishes after narrowing
//! ([`ManifestEntry::published_label`]). The [`LaunchCutline`] fixes the boundary
//! between a published-Stable label and a label narrowed below Stable, reused from
//! the stable claim matrix. The [`PublicationRule`] set names the closed
//! conditions that gate publication, and [`StableClaimManifest::publication`]
//! records the resulting proceed/hold verdict.
//!
//! The manifest is checked in at `artifacts/release/stable_claim_manifest.json`
//! and embedded here, so this typed consumer and the CI gate agree on every entry
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material. Two
//! classes of check live outside this model because they need more than the
//! manifest sees: date arithmetic (recomputing the freshness-SLO state and waiver
//! expiry against an `as_of` date) and the cross-artifact backing checks (whether
//! each `backing_claim_ref`, `qualification_row_refs`, and `support_class_ref`
//! still holds in its neighbouring artifact). Both live in the CI gate. This model
//! enforces the structural and logical invariants that hold regardless of the
//! clock and the neighbouring artifacts — narrowing consistency, the no-widening
//! rule, freshness-SLO/state coherence, owner sign-off on published labels,
//! publication-rule wiring, and the publication verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported manifest schema version.
pub const STABLE_CLAIM_MANIFEST_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the manifest.
pub const STABLE_CLAIM_MANIFEST_RECORD_KIND: &str = "stable_claim_manifest";

/// Repo-relative path to the checked-in manifest.
pub const STABLE_CLAIM_MANIFEST_PATH: &str = "artifacts/release/stable_claim_manifest.json";

/// Embedded checked-in manifest JSON.
pub const STABLE_CLAIM_MANIFEST_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/stable_claim_manifest.json"
));

/// The freshness-SLO state a proof packet earns against its target age.
///
/// `current` and `due_for_refresh` are both within the SLO; `breached` and
/// `missing` are outside it and force a published label to narrow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessSloState {
    /// Captured well within the freshness SLO.
    Current,
    /// Within the SLO but inside the warn window; refresh is due soon.
    DueForRefresh,
    /// Age exceeds the SLO target; the packet is stale.
    Breached,
    /// No proof packet has been captured.
    Missing,
}

impl FreshnessSloState {
    /// Every state, freshest to stalest.
    pub const ALL: [Self; 4] = [
        Self::Current,
        Self::DueForRefresh,
        Self::Breached,
        Self::Missing,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::DueForRefresh => "due_for_refresh",
            Self::Breached => "breached",
            Self::Missing => "missing",
        }
    }

    /// Freshness rank; a fresher state ranks higher. The CI gate uses this to
    /// fail a packet whose declared state is fresher than the clock allows.
    pub const fn freshness_rank(self) -> u8 {
        match self {
            Self::Current => 3,
            Self::DueForRefresh => 2,
            Self::Breached => 1,
            Self::Missing => 0,
        }
    }

    /// True when the packet is within its freshness SLO (current or due-soon).
    pub const fn is_within_slo(self) -> bool {
        matches!(self, Self::Current | Self::DueForRefresh)
    }

    /// True when the packet is outside its freshness SLO and forces narrowing.
    pub const fn forces_narrowing(self) -> bool {
        !self.is_within_slo()
    }
}

/// Manifest state earned by an entry for its claimed lifecycle label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestState {
    /// Full, current, owner-signed backing; publishes the claimed label.
    Published,
    /// Publishes the claimed label only because an active, unexpired waiver covers
    /// a recorded gap.
    ProvisionalOnWaiver,
    /// A backing claim, qualification lane, or support class is missing or
    /// narrowed; the label must narrow.
    NarrowedUnqualified,
    /// The proof packet breached its freshness SLO (or is missing); the label
    /// must narrow.
    NarrowedStale,
    /// The entry relied on a waiver that has expired; the label must narrow.
    NarrowedWaiverExpired,
}

impl ManifestState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Published,
        Self::ProvisionalOnWaiver,
        Self::NarrowedUnqualified,
        Self::NarrowedStale,
        Self::NarrowedWaiverExpired,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::ProvisionalOnWaiver => "provisional_on_waiver",
            Self::NarrowedUnqualified => "narrowed_unqualified",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
        }
    }

    /// Whether the state lets an entry publish its claimed label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Published | Self::ProvisionalOnWaiver)
    }

    /// Whether the state forces the entry below its claimed label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a lifecycle label narrows or a publication rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// The backing stable claim narrowed below the stable cutline.
    BackingClaimNarrowed,
    /// A backing qualification lane is narrowed below the cutline.
    QualificationIncomplete,
    /// The backing support class narrowed below the class it was put forward as.
    SupportClassThinned,
    /// The proof packet breached its freshness SLO.
    ProofPacketFreshnessBreached,
    /// No proof packet has been captured.
    ProofPacketMissing,
    /// A waiver the label relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl NarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::BackingClaimNarrowed,
        Self::QualificationIncomplete,
        Self::SupportClassThinned,
        Self::ProofPacketFreshnessBreached,
        Self::ProofPacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackingClaimNarrowed => "backing_claim_narrowed",
            Self::QualificationIncomplete => "qualification_incomplete",
            Self::SupportClassThinned => "support_class_thinned",
            Self::ProofPacketFreshnessBreached => "proof_packet_freshness_breached",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a publication rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the published lifecycle label below the cutline.
    NarrowLabel,
    /// Refresh the proof packet so it re-enters its freshness SLO.
    RefreshProofPacket,
    /// Re-validate the backing stable claim, qualification, or support class.
    RevalidateBackingClaim,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl PublicationAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HoldPublication,
        Self::NarrowLabel,
        Self::RefreshProofPacket,
        Self::RevalidateBackingClaim,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowLabel => "narrow_label",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RevalidateBackingClaim => "revalidate_backing_claim",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// The packet-freshness SLO for a proof packet: how long it stays claim-bearing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessSlo {
    /// The SLO target: the packet may be at most this many days old.
    pub target_max_age_days: u32,
    /// Days-remaining threshold at or below which the packet is `due_for_refresh`.
    pub warn_within_days: u32,
    /// Ref into the freshness-SLO register that defines this target.
    pub slo_register_ref: String,
}

impl FreshnessSlo {
    /// True when the warn window does not exceed the target age.
    pub const fn window_is_consistent(&self) -> bool {
        self.warn_within_days <= self.target_max_age_days
    }
}

/// The proof packet backing a manifest entry, with its freshness SLO and state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofPacket {
    /// Stable packet id.
    pub packet_id: String,
    /// Ref to the reviewer-facing proof packet.
    pub packet_ref: String,
    /// The stable proof-index row this packet is registered under.
    pub proof_index_ref: String,
    /// UTC date the packet was captured, or null when none exists yet.
    #[serde(default)]
    pub captured_at: Option<String>,
    /// The packet-freshness SLO.
    pub freshness_slo: FreshnessSlo,
    /// The freshness-SLO state earned.
    pub slo_state: FreshnessSloState,
    /// Evidence refs carried by the packet. Empty only on uncaptured packets.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ProofPacket {
    /// True when the packet has a capture date and at least one evidence ref.
    pub fn has_capture(&self) -> bool {
        self.captured_at.is_some() && !self.evidence_refs.is_empty()
    }
}

/// One publication rule: a closed condition that narrows a label and may gate
/// publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a claimed entry fires this rule.
    pub trigger_reason: NarrowingReason,
    /// Claimed labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: PublicationAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One stable claim manifest entry: a published subject bound to its sources, its
/// canonical lifecycle label, and its proof-packet freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestEntry {
    /// Stable entry id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// Subject family the entry speaks for.
    pub subject_family: String,
    /// The lifecycle label the entry is put forward as. Always at or above the
    /// cutline.
    pub claimed_label: StableClaimLevel,
    /// Manifest state earned for the claimed label.
    pub manifest_state: ManifestState,
    /// The stable-claim id this entry's label depends on.
    pub backing_claim_ref: String,
    /// The qualification-row ids this entry's label depends on.
    #[serde(default)]
    pub qualification_row_refs: Vec<String>,
    /// The support-class entry id this entry's label depends on.
    pub support_class_ref: String,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional label, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons narrowing the entry.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// The canonical lifecycle label the entry publishes after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this entry's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the entry carries this posture.
    pub rationale: String,
}

impl ManifestEntry {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the entry's state lets it hold its claimed label.
    pub fn holds_label(&self) -> bool {
        self.manifest_state.holds_label()
    }

    /// True when a narrowing reason is active on the entry.
    pub fn has_active_reason(&self, reason: NarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }
}

/// The recorded publication verdict for the stable claim manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestPublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Publication-rule ids that block publication, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Entry ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_entry_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StableClaimManifestSummary {
    /// Total number of entries.
    pub total_entries: usize,
    /// Entries publishing a label at or above the cutline.
    pub entries_published_stable: usize,
    /// Entries narrowed below the cutline.
    pub entries_narrowed_below_cutline: usize,
    /// Entries holding a label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active narrowing reasons across all entries.
    pub total_active_narrowing_reasons: usize,
    /// Number of publication rules currently firing.
    pub publication_rules_firing: usize,
}

/// The typed stable claim manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StableClaimManifest {
    /// Manifest schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable manifest identifier.
    pub manifest_id: String,
    /// Lifecycle status of this manifest artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim matrix this manifest ingests.
    pub claim_matrix_ref: String,
    /// Ref to the stable qualification matrix this manifest ingests.
    pub qualification_matrix_ref: String,
    /// Ref to the support-class ledger this manifest ingests.
    pub support_class_ledger_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed freshness-SLO-state vocabulary.
    pub freshness_slo_states: Vec<FreshnessSloState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed publication-action vocabulary.
    pub publication_actions: Vec<PublicationAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// Publication rules.
    pub publication_rules: Vec<PublicationRule>,
    /// Manifest entries.
    pub entries: Vec<ManifestEntry>,
    /// Recorded publication verdict.
    pub publication: ManifestPublicationRecord,
    /// Summary counts.
    pub summary: StableClaimManifestSummary,
}

impl StableClaimManifest {
    /// Returns the entry registered for `entry_id`.
    pub fn entry(&self, entry_id: &str) -> Option<&ManifestEntry> {
        self.entries.iter().find(|entry| entry.entry_id == entry_id)
    }

    /// Returns the entries publishing a label at or above the cutline.
    pub fn entries_published_stable(&self) -> Vec<&ManifestEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.publishes_stable())
            .collect()
    }

    /// Returns the entries narrowed below the cutline.
    pub fn entries_narrowed(&self) -> Vec<&ManifestEntry> {
        self.entries
            .iter()
            .filter(|entry| !entry.publishes_stable())
            .collect()
    }

    /// True when `rule` fires: a claimed entry in its watch set carries its
    /// trigger reason.
    pub fn publication_rule_fires(&self, rule: &PublicationRule) -> bool {
        self.entries.iter().any(|entry| {
            rule.applies_to_labels.contains(&entry.claimed_label)
                && entry.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the entries and publication rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .publication_rules
            .iter()
            .any(|rule| rule.blocks_publication && self.publication_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .publication_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.publication_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Entry ids that trigger a blocking, firing publication rule, sorted and
    /// unique.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<NarrowingReason> = self
            .publication_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.publication_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for entry in &self.entries {
            if entry.claimed_label.is_at_or_above_cutline()
                && entry
                    .active_narrowing_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(entry.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the entries and publication rules.
    pub fn computed_summary(&self) -> StableClaimManifestSummary {
        let packets = |state: FreshnessSloState| {
            self.entries
                .iter()
                .filter(|entry| entry.proof_packet.slo_state == state)
                .count()
        };
        StableClaimManifestSummary {
            total_entries: self.entries.len(),
            entries_published_stable: self
                .entries
                .iter()
                .filter(|entry| entry.publishes_stable())
                .count(),
            entries_narrowed_below_cutline: self
                .entries
                .iter()
                .filter(|entry| !entry.publishes_stable())
                .count(),
            entries_on_active_waiver: self
                .entries
                .iter()
                .filter(|entry| entry.manifest_state == ManifestState::ProvisionalOnWaiver)
                .count(),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_narrowing_reasons: self
                .entries
                .iter()
                .map(|entry| entry.active_narrowing_reasons.len())
                .sum(),
            publication_rules_firing: self
                .publication_rules
                .iter()
                .filter(|rule| self.publication_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the manifest that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> ManifestExportProjection {
        ManifestExportProjection {
            manifest_id: self.manifest_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            entries: self
                .entries
                .iter()
                .map(|entry| ManifestExportRow {
                    entry_id: entry.entry_id.clone(),
                    subject_family: entry.subject_family.clone(),
                    claimed_label: entry.claimed_label,
                    published_label: entry.published_label,
                    publishes_stable: entry.publishes_stable(),
                    manifest_state: entry.manifest_state,
                    slo_state: entry.proof_packet.slo_state,
                    active_narrowing_reasons: entry.active_narrowing_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the manifest, returning every violation found.
    pub fn validate(&self) -> Vec<StableClaimManifestViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for entry in &self.entries {
            if !seen.insert(entry.entry_id.clone()) {
                violations.push(StableClaimManifestViolation::DuplicateEntryId {
                    entry_id: entry.entry_id.clone(),
                });
            }
            self.validate_entry(entry, &mut violations);
        }
        if self.entries.is_empty() {
            violations.push(StableClaimManifestViolation::EmptyManifest);
        }

        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(StableClaimManifestViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<StableClaimManifestViolation>) {
        if self.schema_version != STABLE_CLAIM_MANIFEST_SCHEMA_VERSION {
            violations.push(StableClaimManifestViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != STABLE_CLAIM_MANIFEST_RECORD_KIND {
            violations.push(StableClaimManifestViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("manifest_id", &self.manifest_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_matrix_ref", &self.claim_matrix_ref),
            ("qualification_matrix_ref", &self.qualification_matrix_ref),
            ("support_class_ledger_ref", &self.support_class_ledger_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(StableClaimManifestViolation::EmptyField {
                    entry_id: "<manifest>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(StableClaimManifestViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.freshness_slo_states != FreshnessSloState::ALL.to_vec() {
            violations.push(StableClaimManifestViolation::ClosedVocabularyMismatch {
                field: "freshness_slo_states",
            });
        }
        if self.narrowing_reasons != NarrowingReason::ALL.to_vec() {
            violations.push(StableClaimManifestViolation::ClosedVocabularyMismatch {
                field: "narrowing_reasons",
            });
        }
        if self.publication_actions != PublicationAction::ALL.to_vec() {
            violations.push(StableClaimManifestViolation::ClosedVocabularyMismatch {
                field: "publication_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(StableClaimManifestViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(StableClaimManifestViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(StableClaimManifestViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(StableClaimManifestViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<StableClaimManifestViolation>) {
        if self.publication_rules.is_empty() {
            violations.push(StableClaimManifestViolation::NoPublicationRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.publication_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(StableClaimManifestViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(StableClaimManifestViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(StableClaimManifestViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        // Every narrowing reason must have a rule, so a narrowing reason cannot
        // fire without a corresponding publication gate.
        for reason in NarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(StableClaimManifestViolation::NarrowingReasonWithoutRule { reason });
            }
        }
    }

    fn validate_entry(
        &self,
        entry: &ManifestEntry,
        violations: &mut Vec<StableClaimManifestViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &entry.entry_id),
            ("title", &entry.title),
            ("subject_family", &entry.subject_family),
            ("backing_claim_ref", &entry.backing_claim_ref),
            ("support_class_ref", &entry.support_class_ref),
            ("rationale", &entry.rationale),
            ("proof_packet.packet_id", &entry.proof_packet.packet_id),
            ("proof_packet.packet_ref", &entry.proof_packet.packet_ref),
            (
                "proof_packet.proof_index_ref",
                &entry.proof_packet.proof_index_ref,
            ),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &entry.proof_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &entry.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(StableClaimManifestViolation::EmptyField {
                    entry_id: entry.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // A manifest entry must put forward a label at or above the cutline.
        if !entry.claimed_label.is_at_or_above_cutline() {
            violations.push(StableClaimManifestViolation::ClaimedLabelBelowCutline {
                entry_id: entry.entry_id.clone(),
                claimed: entry.claimed_label,
            });
        }

        // No widening: the published label may not be stronger than claimed.
        if entry.published_label.rank() > entry.claimed_label.rank() {
            violations.push(StableClaimManifestViolation::PublishedWiderThanClaimed {
                entry_id: entry.entry_id.clone(),
                claimed: entry.claimed_label,
                published: entry.published_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if entry.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(StableClaimManifestViolation::EmptyField {
                entry_id: entry.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !entry.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(StableClaimManifestViolation::FreshnessSloInconsistent {
                entry_id: entry.entry_id.clone(),
            });
        }

        let slo_state = entry.proof_packet.slo_state;

        if entry.holds_label() {
            // A published label publishes exactly its claim, carries no active
            // narrowing reason, rides a captured packet within its freshness SLO,
            // and is owner-signed.
            if entry.published_label != entry.claimed_label {
                violations.push(StableClaimManifestViolation::HeldLabelNotEqualClaimed {
                    entry_id: entry.entry_id.clone(),
                    claimed: entry.claimed_label,
                    published: entry.published_label,
                });
            }
            if !entry.active_narrowing_reasons.is_empty() {
                violations.push(StableClaimManifestViolation::HeldWithActiveNarrowing {
                    entry_id: entry.entry_id.clone(),
                });
            }
            if !entry.proof_packet.has_capture() {
                violations.push(StableClaimManifestViolation::HeldWithoutFreshPacket {
                    entry_id: entry.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(StableClaimManifestViolation::HeldOnStalePacket {
                    entry_id: entry.entry_id.clone(),
                    slo_state,
                });
            }
            if !(entry.owner_signoff.signed_off && entry.owner_signoff.signed_at.is_some()) {
                violations.push(StableClaimManifestViolation::HeldWithoutSignoff {
                    entry_id: entry.entry_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the published label below the cutline and
            // name at least one active reason.
            if entry.publishes_stable() {
                violations.push(StableClaimManifestViolation::PublishedLabelNotNarrowed {
                    entry_id: entry.entry_id.clone(),
                    state: entry.manifest_state,
                    published: entry.published_label,
                });
            }
            if entry.active_narrowing_reasons.is_empty() {
                violations.push(StableClaimManifestViolation::NarrowingWithoutReason {
                    entry_id: entry.entry_id.clone(),
                    state: entry.manifest_state,
                });
            }
            // A narrowing entry whose packet is breached or missing must name the
            // matching freshness reason, so the freshness automation stays honest.
            if slo_state == FreshnessSloState::Breached
                && !entry.has_active_reason(NarrowingReason::ProofPacketFreshnessBreached)
            {
                violations.push(StableClaimManifestViolation::BreachedPacketWithoutReason {
                    entry_id: entry.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !entry.has_active_reason(NarrowingReason::ProofPacketMissing)
            {
                violations.push(StableClaimManifestViolation::MissingPacketWithoutReason {
                    entry_id: entry.entry_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(entry, violations);
    }

    fn validate_state_reason_coherence(
        &self,
        entry: &ManifestEntry,
        violations: &mut Vec<StableClaimManifestViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<StableClaimManifestViolation>,
                               expected: NarrowingReason| {
            violations.push(StableClaimManifestViolation::StateReasonIncoherent {
                entry_id: entry.entry_id.clone(),
                state: entry.manifest_state,
                expected_reason: expected,
            });
        };

        match entry.manifest_state {
            ManifestState::NarrowedUnqualified => {
                const ALLOWED: [NarrowingReason; 4] = [
                    NarrowingReason::BackingClaimNarrowed,
                    NarrowingReason::QualificationIncomplete,
                    NarrowingReason::SupportClassThinned,
                    NarrowingReason::OwnerSignoffMissing,
                ];
                if !ALLOWED.iter().any(|r| entry.has_active_reason(*r)) {
                    push_incoherent(violations, NarrowingReason::BackingClaimNarrowed);
                }
            }
            ManifestState::NarrowedStale => {
                if !(entry.has_active_reason(NarrowingReason::ProofPacketFreshnessBreached)
                    || entry.has_active_reason(NarrowingReason::ProofPacketMissing))
                {
                    push_incoherent(violations, NarrowingReason::ProofPacketFreshnessBreached);
                }
            }
            ManifestState::NarrowedWaiverExpired => {
                if !entry.has_active_reason(NarrowingReason::WaiverExpired) {
                    push_incoherent(violations, NarrowingReason::WaiverExpired);
                }
                if entry.waiver.is_none() {
                    violations.push(StableClaimManifestViolation::WaiverStateWithoutWaiver {
                        entry_id: entry.entry_id.clone(),
                        state: entry.manifest_state,
                    });
                }
            }
            ManifestState::ProvisionalOnWaiver => {
                if entry
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(StableClaimManifestViolation::WaiverStateWithoutWaiver {
                        entry_id: entry.entry_id.clone(),
                        state: entry.manifest_state,
                    });
                }
            }
            ManifestState::Published => {}
        }
    }

    fn validate_publication(&self, violations: &mut Vec<StableClaimManifestViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(StableClaimManifestViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(StableClaimManifestViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                StableClaimManifestViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                StableClaimManifestViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_entry_ids != self.computed_blocking_entry_ids() {
            violations.push(
                StableClaimManifestViolation::PublicationBlockingSetMismatch {
                    field: "blocking_entry_ids",
                },
            );
        }
    }
}

/// A redaction-safe export row projected from the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestExportRow {
    /// Stable entry id.
    pub entry_id: String,
    /// Subject family.
    pub subject_family: String,
    /// Label the entry is put forward as.
    pub claimed_label: StableClaimLevel,
    /// Canonical lifecycle label the entry publishes.
    pub published_label: StableClaimLevel,
    /// Whether the entry publishes a label at or above the cutline.
    pub publishes_stable: bool,
    /// Manifest state.
    pub manifest_state: ManifestState,
    /// Proof-packet freshness-SLO state.
    pub slo_state: FreshnessSloState,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
}

/// A redaction-safe export projection of the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestExportProjection {
    /// Manifest id this projection was produced from.
    pub manifest_id: String,
    /// Manifest as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Projected entries.
    pub entries: Vec<ManifestExportRow>,
}

/// A validation violation for the stable claim manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableClaimManifestViolation {
    /// The manifest carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the manifest.
        actual: u32,
    },
    /// The manifest carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the manifest.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The manifest has no entries.
    EmptyManifest,
    /// The manifest has no publication rules.
    NoPublicationRules,
    /// A required field is empty.
    EmptyField {
        /// Entry, rule, or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// An entry id appears more than once.
    DuplicateEntryId {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A publication rule names no labels to watch.
    RuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A narrowing reason has no rule watching for it.
    NarrowingReasonWithoutRule {
        /// Uncovered reason.
        reason: NarrowingReason,
    },
    /// An entry asserts a label below the cutline.
    ClaimedLabelBelowCutline {
        /// Entry id.
        entry_id: String,
        /// Claimed label.
        claimed: StableClaimLevel,
    },
    /// A published label is stronger than the claimed label.
    PublishedWiderThanClaimed {
        /// Entry id.
        entry_id: String,
        /// Claimed label.
        claimed: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A freshness SLO's warn window exceeds its target age.
    FreshnessSloInconsistent {
        /// Entry id.
        entry_id: String,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Entry id.
        entry_id: String,
        /// Manifest state.
        state: ManifestState,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A narrowing state carries no active narrowing reason.
    NarrowingWithoutReason {
        /// Entry id.
        entry_id: String,
        /// Manifest state.
        state: ManifestState,
    },
    /// A held entry's published label is not equal to its claimed label.
    HeldLabelNotEqualClaimed {
        /// Entry id.
        entry_id: String,
        /// Claimed label.
        claimed: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A held entry carries an active narrowing reason.
    HeldWithActiveNarrowing {
        /// Entry id.
        entry_id: String,
    },
    /// A held entry rides a proof packet with no capture or evidence.
    HeldWithoutFreshPacket {
        /// Entry id.
        entry_id: String,
    },
    /// A held entry rides a proof packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Entry id.
        entry_id: String,
        /// The packet's freshness-SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held entry has no owner sign-off.
    HeldWithoutSignoff {
        /// Entry id.
        entry_id: String,
    },
    /// A narrowing entry with a breached packet does not name the breach reason.
    BreachedPacketWithoutReason {
        /// Entry id.
        entry_id: String,
    },
    /// A narrowing entry with a missing packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Entry id.
        entry_id: String,
    },
    /// A manifest state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Entry id.
        entry_id: String,
        /// Manifest state.
        state: ManifestState,
        /// Reason the state requires.
        expected_reason: NarrowingReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Entry id.
        entry_id: String,
        /// Manifest state.
        state: ManifestState,
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
    /// The summary counts disagree with the entries.
    SummaryMismatch,
}

impl fmt::Display for StableClaimManifestViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported manifest schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported manifest record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "manifest {field} is not the canonical value")
            }
            Self::EmptyManifest => write!(f, "manifest has no entries"),
            Self::NoPublicationRules => write!(f, "manifest has no publication rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate manifest entry id {entry_id}")
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate publication rule id {rule_id}")
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "publication rule {rule_id} watches no labels")
            }
            Self::NarrowingReasonWithoutRule { reason } => write!(
                f,
                "narrowing reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::ClaimedLabelBelowCutline { entry_id, claimed } => write!(
                f,
                "entry {entry_id} asserts label {} which is below the stable cutline",
                claimed.as_str()
            ),
            Self::PublishedWiderThanClaimed {
                entry_id,
                claimed,
                published,
            } => write!(
                f,
                "entry {entry_id} published label {} is wider than claimed label {}",
                published.as_str(),
                claimed.as_str()
            ),
            Self::FreshnessSloInconsistent { entry_id } => write!(
                f,
                "entry {entry_id} freshness SLO warn window exceeds its target age"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "entry {entry_id} state {} must narrow below the cutline but publishes {}",
                state.as_str(),
                published.as_str()
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "entry {entry_id} state {} narrows without naming an active narrowing reason",
                state.as_str()
            ),
            Self::HeldLabelNotEqualClaimed {
                entry_id,
                claimed,
                published,
            } => write!(
                f,
                "entry {entry_id} publishes {} but is put forward as {}",
                published.as_str(),
                claimed.as_str()
            ),
            Self::HeldWithActiveNarrowing { entry_id } => write!(
                f,
                "entry {entry_id} publishes its label while a narrowing reason is active"
            ),
            Self::HeldWithoutFreshPacket { entry_id } => write!(
                f,
                "entry {entry_id} publishes its label with no captured, evidence-backed proof packet"
            ),
            Self::HeldOnStalePacket { entry_id, slo_state } => write!(
                f,
                "entry {entry_id} publishes its label while its packet is {} (outside its freshness SLO)",
                slo_state.as_str()
            ),
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "entry {entry_id} publishes its label without owner sign-off")
            }
            Self::BreachedPacketWithoutReason { entry_id } => write!(
                f,
                "entry {entry_id} has a breached packet but does not name proof_packet_freshness_breached"
            ),
            Self::MissingPacketWithoutReason { entry_id } => write!(
                f,
                "entry {entry_id} has a missing packet but does not name proof_packet_missing"
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "entry {entry_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => write!(
                f,
                "entry {entry_id} state {} names no waiver",
                state.as_str()
            ),
            Self::PublicationDecisionInconsistent { declared, computed } => write!(
                f,
                "publication decision {} disagrees with computed {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication {field} disagrees with the firing rules")
            }
            Self::SummaryMismatch => write!(f, "manifest summary counts disagree with the entries"),
        }
    }
}

impl Error for StableClaimManifestViolation {}

/// Loads the embedded stable claim manifest.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in manifest no longer matches
/// [`StableClaimManifest`] — including when an entry carries a label, manifest
/// state, freshness-SLO state, narrowing reason, or publication action outside the
/// closed vocabularies.
pub fn current_stable_claim_manifest() -> Result<StableClaimManifest, serde_json::Error> {
    serde_json::from_str(STABLE_CLAIM_MANIFEST_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest() -> StableClaimManifest {
        current_stable_claim_manifest().expect("manifest parses")
    }

    #[test]
    fn embedded_manifest_parses_and_validates() {
        let manifest = manifest();
        assert_eq!(
            manifest.schema_version,
            STABLE_CLAIM_MANIFEST_SCHEMA_VERSION
        );
        assert_eq!(manifest.record_kind, STABLE_CLAIM_MANIFEST_RECORD_KIND);
        assert_eq!(manifest.validate(), Vec::new());
        assert!(!manifest.entries.is_empty());
    }

    #[test]
    fn manifest_exercises_published_and_narrowed_entries() {
        let manifest = manifest();
        assert!(
            !manifest.entries_published_stable().is_empty(),
            "manifest must show at least one published-stable label"
        );
        assert!(
            !manifest.entries_narrowed().is_empty(),
            "manifest must show at least one narrowed label"
        );
    }

    #[test]
    fn summary_counts_match_entries() {
        let manifest = manifest();
        assert_eq!(manifest.summary, manifest.computed_summary());
        assert_eq!(
            manifest.summary.entries_published_stable
                + manifest.summary.entries_narrowed_below_cutline,
            manifest.entries.len()
        );
        assert_eq!(
            manifest.summary.packets_current
                + manifest.summary.packets_due_for_refresh
                + manifest.summary.packets_breached
                + manifest.summary.packets_missing,
            manifest.entries.len()
        );
    }

    #[test]
    fn publication_holds_when_a_blocking_rule_fires() {
        let manifest = manifest();
        assert_eq!(manifest.publication.decision, PromotionDecision::Hold);
        assert_eq!(
            manifest.publication.decision,
            manifest.computed_publication_decision()
        );
        assert!(!manifest.publication.blocking_rule_ids.is_empty());
        assert!(!manifest.publication.blocking_entry_ids.is_empty());
    }

    #[test]
    fn every_narrowing_reason_has_a_rule() {
        let manifest = manifest();
        let covered: BTreeSet<NarrowingReason> = manifest
            .publication_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in NarrowingReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn at_least_one_label_narrows_on_a_breached_packet() {
        let manifest = manifest();
        assert!(
            manifest.entries.iter().any(|entry| {
                entry.proof_packet.slo_state == FreshnessSloState::Breached
                    && !entry.publishes_stable()
                    && entry.has_active_reason(NarrowingReason::ProofPacketFreshnessBreached)
            }),
            "the freshness-SLO automation must narrow at least one breached-packet label"
        );
    }

    #[test]
    fn validate_flags_a_breached_packet_on_a_published_entry() {
        let mut manifest = manifest();
        let entry = manifest
            .entries
            .iter_mut()
            .find(|entry| entry.holds_label())
            .expect("a held entry exists");
        entry.proof_packet.slo_state = FreshnessSloState::Breached;
        let entry_id = entry.entry_id.clone();
        manifest.summary = manifest.computed_summary();
        assert!(manifest
            .validate()
            .contains(&StableClaimManifestViolation::HeldOnStalePacket {
                entry_id,
                slo_state: FreshnessSloState::Breached,
            }));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut manifest = manifest();
        let entry = manifest
            .entries
            .iter_mut()
            .find(|entry| entry.manifest_state == ManifestState::NarrowedUnqualified)
            .expect("a narrowed entry exists");
        entry.published_label = entry.claimed_label;
        manifest.summary = manifest.computed_summary();
        manifest.publication.decision = manifest.computed_publication_decision();
        manifest.publication.blocking_rule_ids = manifest.computed_blocking_rule_ids();
        manifest.publication.blocking_entry_ids = manifest.computed_blocking_entry_ids();
        assert!(manifest.validate().iter().any(|violation| matches!(
            violation,
            StableClaimManifestViolation::PublishedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut manifest = manifest();
        manifest.publication.decision = PromotionDecision::Proceed;
        assert!(manifest.validate().iter().any(|violation| matches!(
            violation,
            StableClaimManifestViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_held_label_without_signoff() {
        let mut manifest = manifest();
        let entry = manifest
            .entries
            .iter_mut()
            .find(|entry| entry.holds_label())
            .expect("a held entry exists");
        entry.owner_signoff.signed_off = false;
        entry.owner_signoff.signed_at = None;
        let entry_id = entry.entry_id.clone();
        manifest.summary = manifest.computed_summary();
        assert!(manifest
            .validate()
            .contains(&StableClaimManifestViolation::HeldWithoutSignoff { entry_id }));
    }

    #[test]
    fn export_projection_mirrors_entries() {
        let manifest = manifest();
        let projection = manifest.support_export_projection();
        assert_eq!(projection.entries.len(), manifest.entries.len());
        assert_eq!(
            projection.publication_decision,
            manifest.publication.decision
        );
        for (entry, projected) in manifest.entries.iter().zip(&projection.entries) {
            assert_eq!(entry.entry_id, projected.entry_id);
            assert_eq!(entry.publishes_stable(), projected.publishes_stable);
            assert_eq!(entry.published_label, projected.published_label);
            assert_eq!(entry.proof_packet.slo_state, projected.slo_state);
        }
    }
}
