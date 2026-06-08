//! Typed v1.0 support-class ledger, certified-archetype manifest, and downgrade
//! automation.
//!
//! Where the [`stable_claim_matrix`](crate::stable_claim_matrix) decides which
//! surfaces may publish as *Stable*, this module decides which *support class*
//! each subject publishes as for the v1.0 release train, and narrows that class
//! automatically when its backing thins out. It is the publication layer that
//! ingests the stable claim matrix instead of cloning its status text.
//!
//! Each [`SupportClassEntry`] binds one subject to:
//!
//! - the support class it is put forward as ([`SupportClassEntry::claimed_class`],
//!   always a positive class),
//! - the backing stable-claim id it depends on
//!   ([`SupportClassEntry::backing_stable_claim_ref`]) and, for a Certified
//!   claim, the certified-archetype manifest entry that defends it
//!   ([`SupportClassEntry::certified_archetype_ref`]),
//! - qualification evidence with a freshness window, an optional waiver, and an
//!   owner sign-off ([`SupportEvidence`], [`LedgerWaiver`], [`LedgerOwnerSignoff`]),
//! - the ledger state earned ([`LedgerState`]), the active downgrade reasons
//!   ([`DowngradeReason`]), and
//! - the class it *effectively* publishes after narrowing
//!   ([`SupportClassEntry::effective_class`]).
//!
//! The [`CertifiedCutline`] fixes that *Certified* is reserved for subjects that
//! reference a fresh, owner-signed [`CertifiedArchetype`] whose status is
//! [`CertificationStatus::Certified`]. The [`DowngradeRule`] set names the closed
//! conditions that gate publication, and [`SupportClassLedger::publication`]
//! records the resulting proceed/hold verdict.
//!
//! The ledger is checked in at `artifacts/release/support_class_ledger.json` and
//! embedded here, so this typed consumer and the CI gate agree on every entry
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material. Two
//! checks live outside this model because they need more than the ledger sees:
//! date arithmetic (waiver expiry and evidence staleness against an `as_of`
//! date) and the cross-artifact backing-claim check (whether each
//! `backing_stable_claim_ref` still holds in the stable claim matrix). Both live
//! in the CI gate. This model enforces the structural and logical invariants
//! that hold regardless of the clock and the neighbouring artifact — narrowing
//! consistency, the no-widening rule, the certified-archetype linkage, owner
//! sign-off on published claims, downgrade-rule wiring, and the publication
//! verdict.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported ledger schema version.
pub const SUPPORT_CLASS_LEDGER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the ledger.
pub const SUPPORT_CLASS_LEDGER_RECORD_KIND: &str = "support_class_ledger";

/// Repo-relative path to the checked-in ledger.
pub const SUPPORT_CLASS_LEDGER_PATH: &str = "artifacts/release/support_class_ledger.json";

/// Embedded checked-in ledger JSON.
pub const SUPPORT_CLASS_LEDGER_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/support_class_ledger.json"
));

/// Public support class a subject is put forward as or effectively publishes.
///
/// The first four are positive classes, ranked strongest to weakest; the last
/// four are the distinct refusal classes, each terminal (rank `0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Certified for a named, fresh certified-archetype scope envelope.
    Certified,
    /// Backed by a live supported compatibility row.
    Supported,
    /// Backed by community evidence or partner attestation.
    Community,
    /// Runtime observation only; no support window claimed.
    Experimental,
    /// Certified or supported under a different scope envelope; this surface is
    /// outside it.
    NotCertifiedInThisMode,
    /// A required provider, workspace, or mode is not configured.
    NotConfigured,
    /// An admin or tenant policy has disabled the capability.
    DisabledByPolicy,
    /// Not supported on this surface and not under any other scope envelope.
    NotSupported,
}

impl SupportClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Certified,
        Self::Supported,
        Self::Community,
        Self::Experimental,
        Self::NotCertifiedInThisMode,
        Self::NotConfigured,
        Self::DisabledByPolicy,
        Self::NotSupported,
    ];

    /// Positive classes, strongest first.
    pub const POSITIVE: [Self; 4] = [
        Self::Certified,
        Self::Supported,
        Self::Community,
        Self::Experimental,
    ];

    /// Refusal classes, in declaration order.
    pub const REFUSAL: [Self; 4] = [
        Self::NotCertifiedInThisMode,
        Self::NotConfigured,
        Self::DisabledByPolicy,
        Self::NotSupported,
    ];

    /// Stable token recorded in the ledger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Supported => "supported",
            Self::Community => "community",
            Self::Experimental => "experimental",
            Self::NotCertifiedInThisMode => "not_certified_in_this_mode",
            Self::NotConfigured => "not_configured",
            Self::DisabledByPolicy => "disabled_by_policy",
            Self::NotSupported => "not_supported",
        }
    }

    /// Strength rank; a stronger positive claim ranks higher. Every refusal
    /// class is terminal at rank `0`.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Certified => 4,
            Self::Supported => 3,
            Self::Community => 2,
            Self::Experimental => 1,
            Self::NotCertifiedInThisMode
            | Self::NotConfigured
            | Self::DisabledByPolicy
            | Self::NotSupported => 0,
        }
    }

    /// True when this is a positive (non-refusal) support class.
    pub const fn is_positive(self) -> bool {
        self.rank() > 0
    }

    /// True when this is a refusal class.
    pub const fn is_refusal(self) -> bool {
        !self.is_positive()
    }
}

/// Ledger state earned by an entry for its claimed class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LedgerState {
    /// Full, current backing with owner sign-off; publishes the claimed class.
    Published,
    /// Publishes the claimed class only because an active, unexpired waiver
    /// covers a recorded gap.
    ProvisionalOnWaiver,
    /// Required backing is absent; the class must narrow.
    NarrowedUnqualified,
    /// Backing evidence exists but its freshness window expired; the class must
    /// narrow.
    NarrowedStale,
    /// The entry relied on a waiver that has expired; the class must narrow.
    NarrowedWaiverExpired,
}

impl LedgerState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Published,
        Self::ProvisionalOnWaiver,
        Self::NarrowedUnqualified,
        Self::NarrowedStale,
        Self::NarrowedWaiverExpired,
    ];

    /// Stable token recorded in the ledger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::ProvisionalOnWaiver => "provisional_on_waiver",
            Self::NarrowedUnqualified => "narrowed_unqualified",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
        }
    }

    /// Whether the state lets an entry publish its claimed class.
    pub const fn holds_claim(self) -> bool {
        matches!(self, Self::Published | Self::ProvisionalOnWaiver)
    }

    /// Whether the state forces the entry to narrow below its claimed class.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_claim()
    }
}

/// Closed reason a support class narrows or a downgrade rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeReason {
    /// A Certified claim references no certified-archetype manifest entry.
    CertifiedArchetypeUnmanifested,
    /// The referenced certified-archetype report is no longer fresh.
    CertifiedArchetypeEvidenceStale,
    /// The referenced certified-archetype manifest entry is decertified.
    CertifiedArchetypeDecertified,
    /// Required support evidence is absent.
    SupportEvidenceMissing,
    /// Support evidence exists but is no longer fresh.
    SupportEvidenceStale,
    /// The backing stable claim narrowed below the stable cutline.
    BackingStableClaimNarrowed,
    /// A waiver the class relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl DowngradeReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CertifiedArchetypeUnmanifested,
        Self::CertifiedArchetypeEvidenceStale,
        Self::CertifiedArchetypeDecertified,
        Self::SupportEvidenceMissing,
        Self::SupportEvidenceStale,
        Self::BackingStableClaimNarrowed,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the ledger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedArchetypeUnmanifested => "certified_archetype_unmanifested",
            Self::CertifiedArchetypeEvidenceStale => "certified_archetype_evidence_stale",
            Self::CertifiedArchetypeDecertified => "certified_archetype_decertified",
            Self::SupportEvidenceMissing => "support_evidence_missing",
            Self::SupportEvidenceStale => "support_evidence_stale",
            Self::BackingStableClaimNarrowed => "backing_stable_claim_narrowed",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a downgrade rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAction {
    /// Automatically narrow the published support class below the claimed class.
    NarrowPublishedClass,
    /// Refresh the certified-archetype report.
    RefreshCertifiedArchetype,
    /// Hold v1.0 publication until the condition clears.
    HoldPublication,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
    /// Route users to a supported scope envelope.
    RouteToSupportedMode,
}

impl DowngradeAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NarrowPublishedClass,
        Self::RefreshCertifiedArchetype,
        Self::HoldPublication,
        Self::RequestOwnerSignoff,
        Self::RouteToSupportedMode,
    ];

    /// Stable token recorded in the ledger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NarrowPublishedClass => "narrow_published_class",
            Self::RefreshCertifiedArchetype => "refresh_certified_archetype",
            Self::HoldPublication => "hold_publication",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::RouteToSupportedMode => "route_to_supported_mode",
        }
    }
}

/// Required evidence path for a support class. Mirrors the closed vocabulary in
/// `artifacts/release/support_class_rows.yaml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidencePathClass {
    /// A certified-archetype report.
    CertifiedArchetypeReport,
    /// A supported compatibility row.
    CompatibilityReportSupported,
    /// Community evidence or a partner attestation.
    CommunityEvidenceOrPartnerAttestation,
    /// Experimental runtime observation only.
    ExperimentalRuntimeObservationOnly,
    /// No evidence required (a refusal-state entry).
    NoEvidenceRequiredRefusalState,
}

impl EvidencePathClass {
    /// Every path class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CertifiedArchetypeReport,
        Self::CompatibilityReportSupported,
        Self::CommunityEvidenceOrPartnerAttestation,
        Self::ExperimentalRuntimeObservationOnly,
        Self::NoEvidenceRequiredRefusalState,
    ];

    /// Stable token recorded in the ledger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedArchetypeReport => "certified_archetype_report",
            Self::CompatibilityReportSupported => "compatibility_report_supported",
            Self::CommunityEvidenceOrPartnerAttestation => {
                "community_evidence_or_partner_attestation"
            }
            Self::ExperimentalRuntimeObservationOnly => "experimental_runtime_observation_only",
            Self::NoEvidenceRequiredRefusalState => "no_evidence_required_refusal_state",
        }
    }
}

/// Certification status of a certified-archetype manifest entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationStatus {
    /// The archetype is certified.
    Certified,
    /// Certification has been withdrawn.
    Decertified,
}

impl CertificationStatus {
    /// Stable token recorded in the ledger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Decertified => "decertified",
        }
    }
}

/// Publication verdict for the v1.0 support line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationDecision {
    /// The support line may publish.
    Proceed,
    /// Publication is blocked by one or more firing downgrade rules.
    Hold,
}

impl PublicationDecision {
    /// Stable token recorded in the ledger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::Hold => "hold",
        }
    }
}

/// The boundary that fixes what *Certified* requires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedCutline {
    /// The class that requires a certified-archetype manifest entry. Always
    /// `certified`.
    pub cutline_class: SupportClass,
    /// The positive classes, strongest first.
    pub positive_classes: Vec<SupportClass>,
    /// The refusal classes a narrowed entry may drop to.
    pub refusal_classes: Vec<SupportClass>,
    /// Reviewable description of the cutline.
    pub description: String,
}

/// Certification evidence for a certified-archetype manifest entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArchetypeCertification {
    /// The certified-archetype report ref.
    pub report_ref: String,
    /// UTC date the report was captured, or null when none exists yet.
    #[serde(default)]
    pub captured_at: Option<String>,
    /// Days the report stays claim-bearing after capture.
    pub freshness_window_days: u32,
    /// Owning team or role.
    pub owner_ref: String,
    /// Whether the owner has signed off the certification.
    pub signed_off: bool,
    /// UTC date of sign-off, or null when not signed off.
    #[serde(default)]
    pub signed_at: Option<String>,
}

/// One certified-archetype manifest entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedArchetype {
    /// Stable archetype id.
    pub archetype_id: String,
    /// Human-readable title.
    pub title: String,
    /// Client class the archetype is certified for.
    pub client_class: String,
    /// OS family the archetype is certified for.
    pub os_family: String,
    /// Deployment mode the archetype is certified for.
    pub deployment_mode: String,
    /// Local-vs-remote mode the archetype is certified for.
    pub locality_mode: String,
    /// Whether the archetype is certified or decertified.
    pub certification_status: CertificationStatus,
    /// Certification evidence.
    pub certification: ArchetypeCertification,
    /// Reviewable scope envelope summary.
    pub scope_summary: String,
    /// Reviewable reason the archetype carries this status.
    pub rationale: String,
}

impl CertifiedArchetype {
    /// True when the archetype is certified.
    pub fn is_certified(&self) -> bool {
        self.certification_status == CertificationStatus::Certified
    }
}

/// One downgrade rule: a closed condition that narrows a published class and may
/// gate publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DowngradeRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The downgrade reason whose presence on a claimed entry fires this rule.
    pub trigger_reason: DowngradeReason,
    /// Claimed classes this rule watches.
    pub applies_to_classes: Vec<SupportClass>,
    /// Default action prescribed when the rule fires.
    pub default_action: DowngradeAction,
    /// Whether firing this rule blocks v1.0 publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// Qualification evidence and freshness window for a ledger entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportEvidence {
    /// Evidence refs backing the class. Empty only on narrowed entries.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// The stable proof-index entry this class is registered under.
    pub proof_index_ref: String,
    /// The evidence path the class requires.
    pub required_evidence_path: EvidencePathClass,
    /// UTC date the evidence was captured, or null when none exists yet.
    #[serde(default)]
    pub captured_at: Option<String>,
    /// Days the evidence stays claim-bearing after capture.
    pub freshness_window_days: u32,
}

/// An active or expired waiver that authorized a provisional class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LedgerWaiver {
    /// Stable waiver ref.
    pub waiver_ref: String,
    /// UTC date the waiver expires.
    pub expires_at: String,
    /// Reviewable reason the waiver was granted.
    pub reason: String,
}

/// Owner sign-off on a ledger entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LedgerOwnerSignoff {
    /// Owning team or role.
    pub owner_ref: String,
    /// Whether the owner has signed off the support claim.
    pub signed_off: bool,
    /// UTC date of sign-off, or null when not signed off.
    #[serde(default)]
    pub signed_at: Option<String>,
}

/// One support-class ledger entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportClassEntry {
    /// Stable entry id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// Subject family the entry speaks for.
    pub subject_family: String,
    /// The support class the entry is put forward as. Always positive.
    pub claimed_class: SupportClass,
    /// Ledger state earned for the claimed class.
    pub ledger_state: LedgerState,
    /// Qualification evidence and freshness window.
    pub evidence: SupportEvidence,
    /// The certified-archetype manifest entry this class references. Required
    /// when the claimed class is `certified`.
    #[serde(default)]
    pub certified_archetype_ref: Option<String>,
    /// The stable-claim id this support class depends on.
    #[serde(default)]
    pub backing_stable_claim_ref: Option<String>,
    /// Waiver authorizing a provisional class, when present.
    #[serde(default)]
    pub waiver: Option<LedgerWaiver>,
    /// Owner sign-off.
    pub owner_signoff: LedgerOwnerSignoff,
    /// Active downgrade reasons narrowing the entry.
    #[serde(default)]
    pub active_downgrade_reasons: Vec<DowngradeReason>,
    /// The class the entry effectively publishes after narrowing.
    pub effective_class: SupportClass,
    /// Publication destinations that render this entry.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the entry carries this posture.
    pub rationale: String,
}

impl SupportClassEntry {
    /// True when the entry publishes its claimed class.
    pub fn holds_claim(&self) -> bool {
        self.ledger_state.holds_claim()
    }

    /// True when the entry effectively publishes the Certified class.
    pub fn publishes_certified(&self) -> bool {
        self.effective_class == SupportClass::Certified
    }

    /// True when a downgrade reason is active on the entry.
    pub fn has_active_reason(&self, reason: DowngradeReason) -> bool {
        self.active_downgrade_reasons.contains(&reason)
    }
}

/// The recorded publication verdict for the v1.0 support line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationDecisionRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PublicationDecision,
    /// Downgrade-rule ids that block publication, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Entry ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_entry_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the ledger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportClassLedgerSummary {
    /// Total number of entries.
    pub total_entries: usize,
    /// Entries publishing their claimed class.
    pub entries_published_as_claimed: usize,
    /// Entries narrowed below their claimed class.
    pub entries_narrowed: usize,
    /// Entries publishing a class via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Entries effectively publishing the Certified class.
    pub certified_entries: usize,
    /// Total active downgrade reasons across all entries.
    pub total_active_downgrade_reasons: usize,
    /// Number of downgrade rules currently firing.
    pub downgrade_rules_firing: usize,
    /// Total certified-archetype manifest entries.
    pub certified_archetypes_total: usize,
    /// Certified-archetype manifest entries that are decertified.
    pub certified_archetypes_decertified: usize,
}

/// The typed v1.0 support-class ledger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportClassLedger {
    /// Ledger schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable ledger identifier.
    pub ledger_id: String,
    /// Lifecycle status of this ledger artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// The release train this ledger publishes.
    pub release_train: String,
    /// Ref to the stable claim matrix this ledger ingests.
    pub claim_matrix_ref: String,
    /// Ref to the support-class taxonomy this ledger publishes against.
    pub taxonomy_ref: String,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed ledger-state vocabulary.
    pub ledger_states: Vec<LedgerState>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Closed downgrade-action vocabulary.
    pub downgrade_actions: Vec<DowngradeAction>,
    /// Closed evidence-path vocabulary.
    pub evidence_path_classes: Vec<EvidencePathClass>,
    /// The certified cutline.
    pub certified_cutline: CertifiedCutline,
    /// The certified-archetype manifest.
    pub certified_archetypes: Vec<CertifiedArchetype>,
    /// Downgrade rules.
    pub downgrade_rules: Vec<DowngradeRule>,
    /// Support-class ledger entries.
    pub entries: Vec<SupportClassEntry>,
    /// Recorded publication verdict.
    pub publication: PublicationDecisionRecord,
    /// Summary counts.
    pub summary: SupportClassLedgerSummary,
}

impl SupportClassLedger {
    /// Returns the entry registered for `entry_id`.
    pub fn entry(&self, entry_id: &str) -> Option<&SupportClassEntry> {
        self.entries.iter().find(|entry| entry.entry_id == entry_id)
    }

    /// Returns the certified-archetype manifest entry for `archetype_id`.
    pub fn archetype(&self, archetype_id: &str) -> Option<&CertifiedArchetype> {
        self.certified_archetypes
            .iter()
            .find(|archetype| archetype.archetype_id == archetype_id)
    }

    /// Returns the entries publishing their claimed class.
    pub fn entries_holding(&self) -> Vec<&SupportClassEntry> {
        self.entries.iter().filter(|e| e.holds_claim()).collect()
    }

    /// Returns the entries narrowed below their claimed class.
    pub fn entries_narrowed(&self) -> Vec<&SupportClassEntry> {
        self.entries.iter().filter(|e| !e.holds_claim()).collect()
    }

    /// True when `rule` fires: a claimed entry in its watch set carries its
    /// trigger reason.
    pub fn downgrade_rule_fires(&self, rule: &DowngradeRule) -> bool {
        self.entries.iter().any(|entry| {
            rule.applies_to_classes.contains(&entry.claimed_class)
                && entry.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the entries and downgrade rules.
    pub fn computed_publication_decision(&self) -> PublicationDecision {
        if self
            .downgrade_rules
            .iter()
            .any(|rule| rule.blocks_publication && self.downgrade_rule_fires(rule))
        {
            PublicationDecision::Hold
        } else {
            PublicationDecision::Proceed
        }
    }

    /// Downgrade-rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .downgrade_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.downgrade_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Entry ids that trigger a blocking, firing downgrade rule, sorted and
    /// unique.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<DowngradeReason> = self
            .downgrade_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.downgrade_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for entry in &self.entries {
            if entry.claimed_class.is_positive()
                && entry
                    .active_downgrade_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(entry.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the entries, rules, and manifest.
    pub fn computed_summary(&self) -> SupportClassLedgerSummary {
        SupportClassLedgerSummary {
            total_entries: self.entries.len(),
            entries_published_as_claimed: self.entries.iter().filter(|e| e.holds_claim()).count(),
            entries_narrowed: self.entries.iter().filter(|e| !e.holds_claim()).count(),
            entries_on_active_waiver: self
                .entries
                .iter()
                .filter(|e| e.ledger_state == LedgerState::ProvisionalOnWaiver)
                .count(),
            certified_entries: self
                .entries
                .iter()
                .filter(|e| e.publishes_certified())
                .count(),
            total_active_downgrade_reasons: self
                .entries
                .iter()
                .map(|e| e.active_downgrade_reasons.len())
                .sum(),
            downgrade_rules_firing: self
                .downgrade_rules
                .iter()
                .filter(|rule| self.downgrade_rule_fires(rule))
                .count(),
            certified_archetypes_total: self.certified_archetypes.len(),
            certified_archetypes_decertified: self
                .certified_archetypes
                .iter()
                .filter(|a| !a.is_certified())
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the ledger that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> SupportClassExportProjection {
        SupportClassExportProjection {
            ledger_id: self.ledger_id.clone(),
            release_train: self.release_train.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            entries: self
                .entries
                .iter()
                .map(|entry| SupportClassExportRow {
                    entry_id: entry.entry_id.clone(),
                    subject_family: entry.subject_family.clone(),
                    claimed_class: entry.claimed_class,
                    effective_class: entry.effective_class,
                    holds_claim: entry.holds_claim(),
                    ledger_state: entry.ledger_state,
                    active_downgrade_reasons: entry.active_downgrade_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the ledger, returning every violation found.
    pub fn validate(&self) -> Vec<SupportClassLedgerViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_archetypes(&mut violations);
        self.validate_rules(&mut violations);

        let archetypes: BTreeMap<&str, &CertifiedArchetype> = self
            .certified_archetypes
            .iter()
            .map(|a| (a.archetype_id.as_str(), a))
            .collect();

        let mut seen = BTreeSet::new();
        for entry in &self.entries {
            if !seen.insert(entry.entry_id.clone()) {
                violations.push(SupportClassLedgerViolation::DuplicateEntryId {
                    entry_id: entry.entry_id.clone(),
                });
            }
            self.validate_entry(entry, &archetypes, &mut violations);
        }
        if self.entries.is_empty() {
            violations.push(SupportClassLedgerViolation::EmptyLedger);
        }

        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(SupportClassLedgerViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<SupportClassLedgerViolation>) {
        if self.schema_version != SUPPORT_CLASS_LEDGER_SCHEMA_VERSION {
            violations.push(SupportClassLedgerViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != SUPPORT_CLASS_LEDGER_RECORD_KIND {
            violations.push(SupportClassLedgerViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("ledger_id", &self.ledger_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("release_train", &self.release_train),
            ("claim_matrix_ref", &self.claim_matrix_ref),
            ("taxonomy_ref", &self.taxonomy_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(SupportClassLedgerViolation::EmptyField {
                    entry_id: "<ledger>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.support_classes != SupportClass::ALL.to_vec() {
            violations.push(SupportClassLedgerViolation::ClosedVocabularyMismatch {
                field: "support_classes",
            });
        }
        if self.ledger_states != LedgerState::ALL.to_vec() {
            violations.push(SupportClassLedgerViolation::ClosedVocabularyMismatch {
                field: "ledger_states",
            });
        }
        if self.downgrade_reasons != DowngradeReason::ALL.to_vec() {
            violations.push(SupportClassLedgerViolation::ClosedVocabularyMismatch {
                field: "downgrade_reasons",
            });
        }
        if self.downgrade_actions != DowngradeAction::ALL.to_vec() {
            violations.push(SupportClassLedgerViolation::ClosedVocabularyMismatch {
                field: "downgrade_actions",
            });
        }
        if self.evidence_path_classes != EvidencePathClass::ALL.to_vec() {
            violations.push(SupportClassLedgerViolation::ClosedVocabularyMismatch {
                field: "evidence_path_classes",
            });
        }

        let cutline = &self.certified_cutline;
        if cutline.cutline_class != SupportClass::Certified {
            violations.push(SupportClassLedgerViolation::ClosedVocabularyMismatch {
                field: "certified_cutline.cutline_class",
            });
        }
        if cutline.positive_classes != SupportClass::POSITIVE.to_vec() {
            violations.push(SupportClassLedgerViolation::ClosedVocabularyMismatch {
                field: "certified_cutline.positive_classes",
            });
        }
        if cutline.refusal_classes != SupportClass::REFUSAL.to_vec() {
            violations.push(SupportClassLedgerViolation::ClosedVocabularyMismatch {
                field: "certified_cutline.refusal_classes",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(SupportClassLedgerViolation::EmptyField {
                entry_id: "<certified_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_archetypes(&self, violations: &mut Vec<SupportClassLedgerViolation>) {
        if self.certified_archetypes.is_empty() {
            violations.push(SupportClassLedgerViolation::NoArchetypes);
        }
        let mut seen = BTreeSet::new();
        for archetype in &self.certified_archetypes {
            if !seen.insert(archetype.archetype_id.clone()) {
                violations.push(SupportClassLedgerViolation::DuplicateArchetypeId {
                    archetype_id: archetype.archetype_id.clone(),
                });
            }
            for (field, value) in [
                ("archetype_id", &archetype.archetype_id),
                ("title", &archetype.title),
                ("client_class", &archetype.client_class),
                ("os_family", &archetype.os_family),
                ("deployment_mode", &archetype.deployment_mode),
                ("locality_mode", &archetype.locality_mode),
                ("scope_summary", &archetype.scope_summary),
                ("rationale", &archetype.rationale),
                (
                    "certification.report_ref",
                    &archetype.certification.report_ref,
                ),
                (
                    "certification.owner_ref",
                    &archetype.certification.owner_ref,
                ),
            ] {
                if value.trim().is_empty() {
                    violations.push(SupportClassLedgerViolation::EmptyField {
                        entry_id: archetype.archetype_id.clone(),
                        field_name: field,
                    });
                }
            }
            if archetype.certification.freshness_window_days == 0 {
                violations.push(SupportClassLedgerViolation::EmptyField {
                    entry_id: archetype.archetype_id.clone(),
                    field_name: "certification.freshness_window_days",
                });
            }
            // A certified archetype must be captured and owner-signed.
            if archetype.is_certified()
                && !(archetype.certification.signed_off
                    && archetype.certification.signed_at.is_some()
                    && archetype.certification.captured_at.is_some())
            {
                violations.push(
                    SupportClassLedgerViolation::CertifiedArchetypeWithoutSignoff {
                        archetype_id: archetype.archetype_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_rules(&self, violations: &mut Vec<SupportClassLedgerViolation>) {
        if self.downgrade_rules.is_empty() {
            violations.push(SupportClassLedgerViolation::NoDowngradeRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.downgrade_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(SupportClassLedgerViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(SupportClassLedgerViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_classes.is_empty() {
                violations.push(SupportClassLedgerViolation::RuleWithoutClasses {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        // Every downgrade reason must have a rule, so a narrowing reason cannot
        // fire without a corresponding publication gate.
        for reason in DowngradeReason::ALL {
            if !covered.contains(&reason) {
                violations.push(SupportClassLedgerViolation::DowngradeReasonWithoutRule { reason });
            }
        }
    }

    fn validate_entry(
        &self,
        entry: &SupportClassEntry,
        archetypes: &BTreeMap<&str, &CertifiedArchetype>,
        violations: &mut Vec<SupportClassLedgerViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &entry.entry_id),
            ("title", &entry.title),
            ("subject_family", &entry.subject_family),
            ("rationale", &entry.rationale),
            ("evidence.proof_index_ref", &entry.evidence.proof_index_ref),
            ("owner_signoff.owner_ref", &entry.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(SupportClassLedgerViolation::EmptyField {
                    entry_id: entry.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // A ledger entry must put forward a positive support class.
        if !entry.claimed_class.is_positive() {
            violations.push(SupportClassLedgerViolation::ClaimedClassNotPositive {
                entry_id: entry.entry_id.clone(),
                claimed: entry.claimed_class,
            });
        }

        // No widening: the effective class may not be stronger than claimed.
        if entry.effective_class.rank() > entry.claimed_class.rank() {
            violations.push(SupportClassLedgerViolation::EffectiveWiderThanClaimed {
                entry_id: entry.entry_id.clone(),
                claimed: entry.claimed_class,
                effective: entry.effective_class,
            });
        }

        if entry.evidence.freshness_window_days == 0 {
            violations.push(SupportClassLedgerViolation::EmptyField {
                entry_id: entry.entry_id.clone(),
                field_name: "evidence.freshness_window_days",
            });
        }

        // A narrowing state must drop strictly below the claimed class and name
        // at least one active reason.
        if entry.ledger_state.forces_narrowing() {
            if entry.effective_class.rank() >= entry.claimed_class.rank() {
                violations.push(SupportClassLedgerViolation::EffectiveNotNarrowed {
                    entry_id: entry.entry_id.clone(),
                    state: entry.ledger_state,
                    effective: entry.effective_class,
                });
            }
            if entry.active_downgrade_reasons.is_empty() {
                violations.push(SupportClassLedgerViolation::NarrowingWithoutReason {
                    entry_id: entry.entry_id.clone(),
                    state: entry.ledger_state,
                });
            }
        }

        // A published class must be current, proof-backed, owner-signed, and
        // free of active downgrade reasons.
        if entry.holds_claim() {
            if entry.effective_class != entry.claimed_class {
                violations.push(SupportClassLedgerViolation::HeldClassNotEqualClaimed {
                    entry_id: entry.entry_id.clone(),
                    claimed: entry.claimed_class,
                    effective: entry.effective_class,
                });
            }
            if !entry.active_downgrade_reasons.is_empty() {
                violations.push(SupportClassLedgerViolation::HeldWithActiveDowngrade {
                    entry_id: entry.entry_id.clone(),
                });
            }
            if entry.evidence.evidence_refs.is_empty() {
                violations.push(SupportClassLedgerViolation::HeldWithoutEvidence {
                    entry_id: entry.entry_id.clone(),
                });
            }
            if !(entry.owner_signoff.signed_off && entry.owner_signoff.signed_at.is_some()) {
                violations.push(SupportClassLedgerViolation::HeldWithoutSignoff {
                    entry_id: entry.entry_id.clone(),
                });
            }
        }

        self.validate_certified_linkage(entry, archetypes, violations);
        self.validate_state_reason_coherence(entry, violations);
    }

    fn validate_certified_linkage(
        &self,
        entry: &SupportClassEntry,
        archetypes: &BTreeMap<&str, &CertifiedArchetype>,
        violations: &mut Vec<SupportClassLedgerViolation>,
    ) {
        if entry.claimed_class != SupportClass::Certified {
            return;
        }
        match entry.certified_archetype_ref.as_deref() {
            None => {
                violations.push(SupportClassLedgerViolation::CertifiedWithoutArchetypeRef {
                    entry_id: entry.entry_id.clone(),
                });
            }
            Some(archetype_ref) => match archetypes.get(archetype_ref) {
                None => {
                    violations.push(
                        SupportClassLedgerViolation::CertifiedArchetypeNotInManifest {
                            entry_id: entry.entry_id.clone(),
                            archetype_ref: archetype_ref.to_owned(),
                        },
                    );
                }
                Some(archetype) => {
                    // A Certified claim may only publish against a certified
                    // (not decertified) archetype.
                    if entry.publishes_certified() && !archetype.is_certified() {
                        violations.push(
                            SupportClassLedgerViolation::CertifiedOnDecertifiedArchetype {
                                entry_id: entry.entry_id.clone(),
                                archetype_ref: archetype_ref.to_owned(),
                            },
                        );
                    }
                }
            },
        }
    }

    fn validate_state_reason_coherence(
        &self,
        entry: &SupportClassEntry,
        violations: &mut Vec<SupportClassLedgerViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<SupportClassLedgerViolation>,
                               expected: DowngradeReason| {
            violations.push(SupportClassLedgerViolation::StateReasonIncoherent {
                entry_id: entry.entry_id.clone(),
                state: entry.ledger_state,
                expected_reason: expected,
            });
        };

        match entry.ledger_state {
            LedgerState::NarrowedUnqualified => {
                const ALLOWED: [DowngradeReason; 5] = [
                    DowngradeReason::SupportEvidenceMissing,
                    DowngradeReason::CertifiedArchetypeUnmanifested,
                    DowngradeReason::CertifiedArchetypeDecertified,
                    DowngradeReason::BackingStableClaimNarrowed,
                    DowngradeReason::OwnerSignoffMissing,
                ];
                if !ALLOWED.iter().any(|r| entry.has_active_reason(*r)) {
                    push_incoherent(violations, DowngradeReason::SupportEvidenceMissing);
                }
            }
            LedgerState::NarrowedStale => {
                if !(entry.has_active_reason(DowngradeReason::SupportEvidenceStale)
                    || entry.has_active_reason(DowngradeReason::CertifiedArchetypeEvidenceStale))
                {
                    push_incoherent(violations, DowngradeReason::SupportEvidenceStale);
                }
            }
            LedgerState::NarrowedWaiverExpired => {
                if !entry.has_active_reason(DowngradeReason::WaiverExpired) {
                    push_incoherent(violations, DowngradeReason::WaiverExpired);
                }
                if entry.waiver.is_none() {
                    violations.push(SupportClassLedgerViolation::WaiverStateWithoutWaiver {
                        entry_id: entry.entry_id.clone(),
                        state: entry.ledger_state,
                    });
                }
            }
            LedgerState::ProvisionalOnWaiver => {
                if entry
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(SupportClassLedgerViolation::WaiverStateWithoutWaiver {
                        entry_id: entry.entry_id.clone(),
                        state: entry.ledger_state,
                    });
                }
            }
            LedgerState::Published => {}
        }
    }

    fn validate_publication(&self, violations: &mut Vec<SupportClassLedgerViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(SupportClassLedgerViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(SupportClassLedgerViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                SupportClassLedgerViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                SupportClassLedgerViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_entry_ids != self.computed_blocking_entry_ids() {
            violations.push(
                SupportClassLedgerViolation::PublicationBlockingSetMismatch {
                    field: "blocking_entry_ids",
                },
            );
        }
    }
}

/// A redaction-safe export row projected from the ledger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportClassExportRow {
    /// Stable entry id.
    pub entry_id: String,
    /// Subject family.
    pub subject_family: String,
    /// Class the entry is put forward as.
    pub claimed_class: SupportClass,
    /// Class the entry effectively publishes.
    pub effective_class: SupportClass,
    /// Whether the entry publishes its claimed class.
    pub holds_claim: bool,
    /// Ledger state.
    pub ledger_state: LedgerState,
    /// Active downgrade reasons.
    pub active_downgrade_reasons: Vec<DowngradeReason>,
}

/// A redaction-safe export projection of the ledger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportClassExportProjection {
    /// Ledger id this projection was produced from.
    pub ledger_id: String,
    /// Release train this ledger publishes.
    pub release_train: String,
    /// Ledger as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PublicationDecision,
    /// Projected entries.
    pub entries: Vec<SupportClassExportRow>,
}

/// A validation violation for the support-class ledger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupportClassLedgerViolation {
    /// The ledger carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the ledger.
        actual: u32,
    },
    /// The ledger carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the ledger.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The ledger has no entries.
    EmptyLedger,
    /// The ledger has no certified-archetype manifest entries.
    NoArchetypes,
    /// The ledger has no downgrade rules.
    NoDowngradeRules,
    /// A required field is empty.
    EmptyField {
        /// Entry, archetype, rule, or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// An entry id appears more than once.
    DuplicateEntryId {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// An archetype id appears more than once.
    DuplicateArchetypeId {
        /// Duplicate archetype id.
        archetype_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A downgrade rule names no classes to watch.
    RuleWithoutClasses {
        /// Rule id.
        rule_id: String,
    },
    /// A downgrade reason has no rule watching for it.
    DowngradeReasonWithoutRule {
        /// Uncovered reason.
        reason: DowngradeReason,
    },
    /// A certified archetype lacks capture or owner sign-off.
    CertifiedArchetypeWithoutSignoff {
        /// Archetype id.
        archetype_id: String,
    },
    /// An entry asserts a refusal class as its claimed class.
    ClaimedClassNotPositive {
        /// Entry id.
        entry_id: String,
        /// Claimed class.
        claimed: SupportClass,
    },
    /// An effective class is stronger than the claimed class.
    EffectiveWiderThanClaimed {
        /// Entry id.
        entry_id: String,
        /// Claimed class.
        claimed: SupportClass,
        /// Effective class.
        effective: SupportClass,
    },
    /// A narrowing state did not drop the entry below its claimed class.
    EffectiveNotNarrowed {
        /// Entry id.
        entry_id: String,
        /// Ledger state.
        state: LedgerState,
        /// Effective class.
        effective: SupportClass,
    },
    /// A narrowing state carries no active downgrade reason.
    NarrowingWithoutReason {
        /// Entry id.
        entry_id: String,
        /// Ledger state.
        state: LedgerState,
    },
    /// A held entry's effective class is not equal to its claimed class.
    HeldClassNotEqualClaimed {
        /// Entry id.
        entry_id: String,
        /// Claimed class.
        claimed: SupportClass,
        /// Effective class.
        effective: SupportClass,
    },
    /// A held entry carries an active downgrade reason.
    HeldWithActiveDowngrade {
        /// Entry id.
        entry_id: String,
    },
    /// A held entry names no qualification evidence.
    HeldWithoutEvidence {
        /// Entry id.
        entry_id: String,
    },
    /// A held entry has no owner sign-off.
    HeldWithoutSignoff {
        /// Entry id.
        entry_id: String,
    },
    /// A Certified claim references no certified-archetype manifest entry.
    CertifiedWithoutArchetypeRef {
        /// Entry id.
        entry_id: String,
    },
    /// A Certified claim references an archetype absent from the manifest.
    CertifiedArchetypeNotInManifest {
        /// Entry id.
        entry_id: String,
        /// Referenced archetype id.
        archetype_ref: String,
    },
    /// A Certified claim publishes against a decertified archetype.
    CertifiedOnDecertifiedArchetype {
        /// Entry id.
        entry_id: String,
        /// Referenced archetype id.
        archetype_ref: String,
    },
    /// A ledger state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Entry id.
        entry_id: String,
        /// Ledger state.
        state: LedgerState,
        /// Reason the state requires.
        expected_reason: DowngradeReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Entry id.
        entry_id: String,
        /// Ledger state.
        state: LedgerState,
    },
    /// The declared publication decision disagrees with the computed one.
    PublicationDecisionInconsistent {
        /// Declared decision.
        declared: PublicationDecision,
        /// Computed decision.
        computed: PublicationDecision,
    },
    /// The declared publication blocking set disagrees with the computed one.
    PublicationBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The summary counts disagree with the entries.
    SummaryMismatch,
}

impl fmt::Display for SupportClassLedgerViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported ledger schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported ledger record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "ledger {field} is not the canonical value")
            }
            Self::EmptyLedger => write!(f, "ledger has no entries"),
            Self::NoArchetypes => write!(f, "ledger has no certified-archetype manifest entries"),
            Self::NoDowngradeRules => write!(f, "ledger has no downgrade rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => write!(f, "duplicate ledger entry id {entry_id}"),
            Self::DuplicateArchetypeId { archetype_id } => {
                write!(f, "duplicate archetype id {archetype_id}")
            }
            Self::DuplicateRuleId { rule_id } => write!(f, "duplicate downgrade rule id {rule_id}"),
            Self::RuleWithoutClasses { rule_id } => {
                write!(f, "downgrade rule {rule_id} watches no classes")
            }
            Self::DowngradeReasonWithoutRule { reason } => write!(
                f,
                "downgrade reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::CertifiedArchetypeWithoutSignoff { archetype_id } => write!(
                f,
                "certified archetype {archetype_id} lacks capture or owner sign-off"
            ),
            Self::ClaimedClassNotPositive { entry_id, claimed } => write!(
                f,
                "entry {entry_id} claims refusal class {} which is not a positive support class",
                claimed.as_str()
            ),
            Self::EffectiveWiderThanClaimed {
                entry_id,
                claimed,
                effective,
            } => write!(
                f,
                "entry {entry_id} effective class {} is wider than claimed class {}",
                effective.as_str(),
                claimed.as_str()
            ),
            Self::EffectiveNotNarrowed {
                entry_id,
                state,
                effective,
            } => write!(
                f,
                "entry {entry_id} state {} must narrow below the claimed class but publishes {}",
                state.as_str(),
                effective.as_str()
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "entry {entry_id} state {} narrows without naming an active downgrade reason",
                state.as_str()
            ),
            Self::HeldClassNotEqualClaimed {
                entry_id,
                claimed,
                effective,
            } => write!(
                f,
                "entry {entry_id} publishes its claim but effective class {} is not the claimed class {}",
                effective.as_str(),
                claimed.as_str()
            ),
            Self::HeldWithActiveDowngrade { entry_id } => write!(
                f,
                "entry {entry_id} publishes its claim while a downgrade reason is active"
            ),
            Self::HeldWithoutEvidence { entry_id } => {
                write!(f, "entry {entry_id} publishes its claim with no evidence")
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "entry {entry_id} publishes its claim without owner sign-off")
            }
            Self::CertifiedWithoutArchetypeRef { entry_id } => write!(
                f,
                "entry {entry_id} claims Certified without referencing a certified-archetype manifest entry"
            ),
            Self::CertifiedArchetypeNotInManifest {
                entry_id,
                archetype_ref,
            } => write!(
                f,
                "entry {entry_id} references archetype {archetype_ref} which is not in the manifest"
            ),
            Self::CertifiedOnDecertifiedArchetype {
                entry_id,
                archetype_ref,
            } => write!(
                f,
                "entry {entry_id} publishes Certified against decertified archetype {archetype_ref}"
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "entry {entry_id} state {} requires an active reason such as {}",
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
            Self::PublicationBlockingSetMismatch { field } => write!(
                f,
                "publication {field} disagrees with the firing downgrade rules"
            ),
            Self::SummaryMismatch => write!(f, "ledger summary counts disagree with the entries"),
        }
    }
}

impl Error for SupportClassLedgerViolation {}

/// Loads the embedded support-class ledger.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in ledger no longer matches
/// [`SupportClassLedger`] — including when an entry carries a support class,
/// ledger state, downgrade reason, action, or evidence path outside the closed
/// vocabularies.
pub fn current_support_class_ledger() -> Result<SupportClassLedger, serde_json::Error> {
    serde_json::from_str(SUPPORT_CLASS_LEDGER_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ledger() -> SupportClassLedger {
        current_support_class_ledger().expect("ledger parses")
    }

    #[test]
    fn embedded_ledger_parses_and_validates() {
        let ledger = ledger();
        assert_eq!(ledger.schema_version, SUPPORT_CLASS_LEDGER_SCHEMA_VERSION);
        assert_eq!(ledger.record_kind, SUPPORT_CLASS_LEDGER_RECORD_KIND);
        assert_eq!(ledger.validate(), Vec::new());
        assert!(!ledger.entries.is_empty());
    }

    #[test]
    fn cutline_partitions_classes() {
        for class in SupportClass::POSITIVE {
            assert!(class.is_positive(), "{}", class.as_str());
        }
        for class in SupportClass::REFUSAL {
            assert!(class.is_refusal(), "{}", class.as_str());
            assert_eq!(class.rank(), 0, "{}", class.as_str());
        }
    }

    #[test]
    fn ledger_exercises_published_entries_without_narrowing() {
        let ledger = ledger();
        assert!(
            !ledger.entries_holding().is_empty(),
            "ledger must publish at least one held support class"
        );
        assert!(
            ledger.entries_narrowed().is_empty(),
            "stable support evidence must not carry narrowed support classes"
        );
    }

    #[test]
    fn summary_counts_match_entries() {
        let ledger = ledger();
        assert_eq!(ledger.summary, ledger.computed_summary());
        assert_eq!(
            ledger.summary.entries_published_as_claimed + ledger.summary.entries_narrowed,
            ledger.entries.len()
        );
    }

    #[test]
    fn publication_proceeds_without_blocking_rules() {
        let ledger = ledger();
        assert_eq!(ledger.publication.decision, PublicationDecision::Proceed);
        assert_eq!(
            ledger.publication.decision,
            ledger.computed_publication_decision()
        );
        assert!(ledger.publication.blocking_rule_ids.is_empty());
        assert!(ledger.publication.blocking_entry_ids.is_empty());
    }

    #[test]
    fn every_downgrade_reason_has_a_rule() {
        let ledger = ledger();
        let covered: BTreeSet<DowngradeReason> = ledger
            .downgrade_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in DowngradeReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn validate_flags_a_held_entry_with_active_downgrade() {
        let mut ledger = ledger();
        let entry = ledger
            .entries
            .iter_mut()
            .find(|e| e.holds_claim())
            .expect("a held entry exists");
        entry
            .active_downgrade_reasons
            .push(DowngradeReason::BackingStableClaimNarrowed);
        let entry_id = entry.entry_id.clone();
        ledger.summary = ledger.computed_summary();
        assert!(ledger
            .validate()
            .contains(&SupportClassLedgerViolation::HeldWithActiveDowngrade { entry_id }));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut ledger = ledger();
        let entry = ledger
            .entries
            .iter_mut()
            .find(|e| e.holds_claim())
            .expect("a held entry exists");
        entry.ledger_state = LedgerState::NarrowedUnqualified;
        entry
            .active_downgrade_reasons
            .push(DowngradeReason::SupportEvidenceMissing);
        entry.effective_class = entry.claimed_class;
        ledger.summary = ledger.computed_summary();
        ledger.publication.decision = ledger.computed_publication_decision();
        ledger.publication.blocking_rule_ids = ledger.computed_blocking_rule_ids();
        ledger.publication.blocking_entry_ids = ledger.computed_blocking_entry_ids();
        assert!(ledger
            .validate()
            .iter()
            .any(|v| matches!(v, SupportClassLedgerViolation::EffectiveNotNarrowed { .. })));
    }

    #[test]
    fn validate_flags_certified_without_a_manifest_entry() {
        let mut ledger = ledger();
        let entry = ledger
            .entries
            .iter_mut()
            .find(|e| e.claimed_class == SupportClass::Certified && e.holds_claim())
            .expect("a held certified entry exists");
        entry.certified_archetype_ref = Some("certified_archetype:does_not_exist".to_owned());
        assert!(ledger.validate().iter().any(|v| matches!(
            v,
            SupportClassLedgerViolation::CertifiedArchetypeNotInManifest { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut ledger = ledger();
        ledger.publication.decision = PublicationDecision::Hold;
        assert!(ledger.validate().iter().any(|v| matches!(
            v,
            SupportClassLedgerViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn export_projection_mirrors_entries() {
        let ledger = ledger();
        let projection = ledger.support_export_projection();
        assert_eq!(projection.entries.len(), ledger.entries.len());
        assert_eq!(projection.publication_decision, ledger.publication.decision);
        for (entry, projected) in ledger.entries.iter().zip(&projection.entries) {
            assert_eq!(entry.entry_id, projected.entry_id);
            assert_eq!(entry.holds_claim(), projected.holds_claim);
            assert_eq!(entry.effective_class, projected.effective_class);
        }
    }
}
