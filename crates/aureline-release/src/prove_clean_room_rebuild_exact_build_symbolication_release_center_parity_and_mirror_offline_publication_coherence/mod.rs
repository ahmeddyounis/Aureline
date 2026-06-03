//! Proves clean-room rebuild verification, exact-build symbolication,
//! release-center parity, and mirror/offline publication coherence.
//!
//! This module is the typed consumer for the clean-room rebuild proof artifact.
//! It records, for every marketed package-channel, symbolication surface, and
//! parity surface, whether the row still holds its published lifecycle label or
//! has narrowed because rebuild verification, exact-build symbol linkage, or a
//! mirror/offline packet fell out of policy.
//!
//! Each [`ChannelFamilyRow`] binds one family surface to the public claim it
//! backs, the [`ProofPacket`] that grounds it, the owner sign-off that governs
//! it, and an optional [`QualificationWaiver`] that can provisionally hold the
//! claim while remaining evidence is completed.
//!
//! The checked-in JSON lives at
//! `artifacts/release/m4/clean-room-rebuild-proof.json` and is embedded here so
//! typed consumers, tests, and CI all agree on the same canonical record.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::ProofPacket;
use crate::stable_claim_matrix::{OwnerSignoff, QualificationWaiver};

const CANONICAL_LIFECYCLE_LABELS: [&str; 5] = ["lts", "stable", "beta", "preview", "withdrawn"];

/// Supported artifact schema version.
pub const CLEAN_ROOM_REBUILD_PROOF_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the artifact.
pub const CLEAN_ROOM_REBUILD_PROOF_RECORD_KIND: &str = "clean_room_rebuild_proof";

/// Repo-relative path to the checked-in artifact.
pub const CLEAN_ROOM_REBUILD_PROOF_PATH: &str =
    "artifacts/release/m4/clean-room-rebuild-proof.json";

/// Embedded checked-in artifact JSON.
pub const CLEAN_ROOM_REBUILD_PROOF_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m4/clean-room-rebuild-proof.json"
));

/// Category of marketed row covered by this proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelFamilyCategory {
    /// Package and channel publication rows.
    PackageChannel,
    /// Exact-build symbolication rows.
    Symbolication,
    /// User-facing truth and parity surfaces.
    ParitySurface,
}

impl ChannelFamilyCategory {
    /// Every category, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::PackageChannel,
        Self::Symbolication,
        Self::ParitySurface,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageChannel => "package_channel",
            Self::Symbolication => "symbolication",
            Self::ParitySurface => "parity_surface",
        }
    }
}

/// Family kind governed by this proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelFamilyKind {
    /// Stable desktop package row.
    StableDesktop,
    /// Stable CLI package row.
    StableCli,
    /// Stable remote-agent package row.
    StableRemoteAgent,
    /// Preview desktop package row.
    PreviewDesktop,
    /// Portable package row.
    Portable,
    /// Managed-install package row.
    ManagedInstall,
    /// Mirror/offline package row.
    MirrorOffline,
    /// Stable crash-symbol row.
    StableCrashSymbols,
    /// Stable source-map row.
    StableSourceMaps,
    /// Preview crash-symbol row.
    PreviewCrashSymbols,
    /// Release-center parity row.
    ReleaseCenter,
    /// About/Help parity row.
    AboutHelp,
    /// Rollback-metadata parity row.
    RollbackMetadata,
    /// Advisory-publication parity row.
    AdvisoryPublication,
    /// Mirror/offline publication-pack parity row.
    MirrorOfflinePack,
}

impl ChannelFamilyKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 15] = [
        Self::StableDesktop,
        Self::StableCli,
        Self::StableRemoteAgent,
        Self::PreviewDesktop,
        Self::Portable,
        Self::ManagedInstall,
        Self::MirrorOffline,
        Self::StableCrashSymbols,
        Self::StableSourceMaps,
        Self::PreviewCrashSymbols,
        Self::ReleaseCenter,
        Self::AboutHelp,
        Self::RollbackMetadata,
        Self::AdvisoryPublication,
        Self::MirrorOfflinePack,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableDesktop => "stable_desktop",
            Self::StableCli => "stable_cli",
            Self::StableRemoteAgent => "stable_remote_agent",
            Self::PreviewDesktop => "preview_desktop",
            Self::Portable => "portable",
            Self::ManagedInstall => "managed_install",
            Self::MirrorOffline => "mirror_offline",
            Self::StableCrashSymbols => "stable_crash_symbols",
            Self::StableSourceMaps => "stable_source_maps",
            Self::PreviewCrashSymbols => "preview_crash_symbols",
            Self::ReleaseCenter => "release_center",
            Self::AboutHelp => "about_help",
            Self::RollbackMetadata => "rollback_metadata",
            Self::AdvisoryPublication => "advisory_publication",
            Self::MirrorOfflinePack => "mirror_offline_pack",
        }
    }
}

/// Rebuild posture earned by a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RebuildState {
    /// The clean-room rebuild is fully verified.
    Verified,
    /// The rebuild lane has only completed a rehearsal.
    Rehearsal,
    /// The recorded rebuild evidence is stale.
    Stale,
    /// The rebuild evidence is missing.
    Missing,
    /// Rebuild posture does not apply to this row.
    NotApplicable,
}

impl RebuildState {
    /// Every rebuild state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Verified,
        Self::Rehearsal,
        Self::Stale,
        Self::Missing,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Rehearsal => "rehearsal",
            Self::Stale => "stale",
            Self::Missing => "missing",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Symbolication posture earned by a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolicationState {
    /// Exact-build symbols are linked.
    Linked,
    /// Exact-build symbols are present but not linked.
    Unlinked,
    /// Symbolication evidence is missing.
    Missing,
    /// Symbolication posture does not apply to this row.
    NotApplicable,
}

impl SymbolicationState {
    /// Every symbolication state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Linked,
        Self::Unlinked,
        Self::Missing,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Linked => "linked",
            Self::Unlinked => "unlinked",
            Self::Missing => "missing",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Claim-bearing state earned by a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelFamilyState {
    /// The row is current and holds its claim label.
    Current,
    /// The row holds its claim label on an active waiver.
    OnWaiver,
    /// The row narrowed because its packet is stale.
    NarrowedStale,
    /// The row narrowed because its packet is missing.
    NarrowedMissing,
    /// The row narrowed because required evidence is incomplete.
    NarrowedUnbacked,
    /// The row narrowed because a relied-on waiver expired.
    NarrowedWaiverExpired,
}

impl ChannelFamilyState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Current,
        Self::OnWaiver,
        Self::NarrowedStale,
        Self::NarrowedMissing,
        Self::NarrowedUnbacked,
        Self::NarrowedWaiverExpired,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::OnWaiver => "on_waiver",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedMissing => "narrowed_missing",
            Self::NarrowedUnbacked => "narrowed_unbacked",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
        }
    }

    /// True when the state allows the row to keep its claim label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Current | Self::OnWaiver)
    }

    /// True when the state forces the row below its claim label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a row narrows or a rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelFamilyGapReason {
    /// The backing claim ceiling was already narrowed.
    ClaimLabelNarrowed,
    /// No packet was captured.
    PacketMissing,
    /// The packet breached its freshness SLO.
    PacketFreshnessBreached,
    /// Required evidence is incomplete.
    EvidenceIncomplete,
    /// The clean-room rebuild is not yet verified.
    RebuildNotVerified,
    /// Exact-build symbolication is not linked.
    SymbolicationUnlinked,
    /// A parity surface no longer matches the canonical release truth.
    ParityMismatch,
    /// A waiver used to hold the claim expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl ChannelFamilyGapReason {
    /// Every gap reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ClaimLabelNarrowed,
        Self::PacketMissing,
        Self::PacketFreshnessBreached,
        Self::EvidenceIncomplete,
        Self::RebuildNotVerified,
        Self::SymbolicationUnlinked,
        Self::ParityMismatch,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::PacketMissing => "packet_missing",
            Self::PacketFreshnessBreached => "packet_freshness_breached",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::RebuildNotVerified => "rebuild_not_verified",
            Self::SymbolicationUnlinked => "symbolication_unlinked",
            Self::ParityMismatch => "parity_mismatch",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelFamilyAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the public claim.
    NarrowClaim,
    /// Refresh the packet.
    RefreshPacket,
    /// Recapture the supporting evidence.
    RecaptureEvidence,
    /// Request owner sign-off.
    RequestOwnerSignoff,
}

impl ChannelFamilyAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HoldPublication,
        Self::NarrowClaim,
        Self::RefreshPacket,
        Self::RecaptureEvidence,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowClaim => "narrow_claim",
            Self::RefreshPacket => "refresh_packet",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// Publication verdict for the proof lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationDecision {
    /// Publication may proceed.
    Proceed,
    /// Publication must hold.
    Hold,
}

impl PublicationDecision {
    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::Hold => "hold",
        }
    }
}

/// One family row in the clean-room rebuild proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChannelFamilyRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// Category the row belongs to.
    pub category: ChannelFamilyCategory,
    /// Specific family kind the row governs.
    pub family_kind: ChannelFamilyKind,
    /// Ref to the governed subject.
    pub subject_ref: String,
    /// Human-readable summary of the governed subject.
    pub subject_summary: String,
    /// Whether this row is part of the blocking release set.
    pub release_blocking: bool,
    /// Ref to the public claim this row backs.
    pub claim_ref: String,
    /// Canonical lifecycle label published by the backing claim.
    pub claim_label: String,
    /// State earned by the row.
    pub family_state: ChannelFamilyState,
    /// Rebuild posture earned by the row.
    pub rebuild_state: RebuildState,
    /// Symbolication posture earned by the row.
    pub symbolication_state: SymbolicationState,
    /// Packet grounding the row.
    pub proof_packet: ProofPacket,
    /// Owner sign-off carried by the row.
    pub owner_signoff: OwnerSignoff,
    /// Waiver authorizing a provisional hold, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<ChannelFamilyGapReason>,
    /// Effective lifecycle label after narrowing.
    pub effective_label: String,
    /// Publication surfaces that render this row.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable explanation for the row posture.
    pub rationale: String,
}

impl ChannelFamilyRow {
    /// True when the row still holds its claim label.
    pub fn holds_claim(&self) -> bool {
        self.family_state.holds_label() && self.effective_label == self.claim_label
    }

    /// True when `reason` is active on the row.
    pub fn has_active_reason(&self, reason: ChannelFamilyGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// One rule that narrows a row and may gate publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChannelFamilyRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason that fires this rule.
    pub trigger_reason: ChannelFamilyGapReason,
    /// Row states this rule watches.
    pub applies_to_states: Vec<ChannelFamilyState>,
    /// Default action prescribed when the rule fires.
    pub default_action: ChannelFamilyAction,
    /// Whether the rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable rationale for the rule.
    pub rationale: String,
}

/// The recorded publication verdict for this proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationDecisionRecord {
    /// Gate governed by this verdict.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PublicationDecision,
    /// Blocking rule ids, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Blocking row ids, sorted.
    #[serde(default)]
    pub blocking_row_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the proof artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CleanRoomRebuildProofSummary {
    /// Total number of rows.
    pub total_rows: usize,
    /// Number of rows holding their claim.
    pub rows_holding_claim: usize,
    /// Number of rows narrowed below their claim.
    pub rows_narrowed: usize,
    /// Number of rows currently held on waiver.
    pub rows_on_active_waiver: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of rules currently firing.
    pub rules_firing: usize,
}

/// The typed clean-room rebuild proof artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CleanRoomRebuildProof {
    /// Artifact schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable artifact identifier.
    pub artifact_id: String,
    /// Lifecycle status of the artifact snapshot.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date the artifact is current as of.
    pub as_of: String,
    /// Ref to the exact build identity this proof covers.
    pub build_identity_ref: String,
    /// Ref to the stable claim manifest this proof ingests.
    pub claim_manifest_ref: String,
    /// Ref to the stable proof index this proof ingests.
    pub stable_proof_index_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<String>,
    /// Closed family-category vocabulary.
    pub family_categories: Vec<ChannelFamilyCategory>,
    /// Closed family-kind vocabulary.
    pub family_kinds: Vec<ChannelFamilyKind>,
    /// Closed family-state vocabulary.
    pub family_states: Vec<ChannelFamilyState>,
    /// Closed rebuild-state vocabulary.
    pub rebuild_states: Vec<RebuildState>,
    /// Closed symbolication-state vocabulary.
    pub symbolication_states: Vec<SymbolicationState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<ChannelFamilyGapReason>,
    /// Closed family-action vocabulary.
    pub family_actions: Vec<ChannelFamilyAction>,
    /// Family rows governed by this proof.
    pub rows: Vec<ChannelFamilyRow>,
    /// Rules watched by this proof.
    pub rules: Vec<ChannelFamilyRule>,
    /// Recorded publication verdict.
    pub publication: PublicationDecisionRecord,
    /// Summary counts.
    pub summary: CleanRoomRebuildProofSummary,
}

impl CleanRoomRebuildProof {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&ChannelFamilyRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns rows still holding their claim label.
    pub fn rows_holding_claim(&self) -> Vec<&ChannelFamilyRow> {
        self.rows.iter().filter(|row| row.holds_claim()).collect()
    }

    /// Returns rows narrowed below their claim label.
    pub fn rows_narrowed(&self) -> Vec<&ChannelFamilyRow> {
        self.rows.iter().filter(|row| !row.holds_claim()).collect()
    }

    /// True when `rule` fires for any row in its watch set.
    pub fn rule_fires(&self, rule: &ChannelFamilyRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_states.contains(&row.family_state)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and rules.
    pub fn computed_publication_decision(&self) -> PublicationDecision {
        if self
            .rules
            .iter()
            .any(|rule| rule.blocks_publication && self.rule_fires(rule))
        {
            PublicationDecision::Hold
        } else {
            PublicationDecision::Proceed
        }
    }

    /// Returns blocking rule ids that are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Returns blocking row ids for currently firing blocking rules, sorted and unique.
    pub fn computed_blocking_row_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<ChannelFamilyGapReason> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids = BTreeSet::new();
        for row in &self.rows {
            if row
                .active_gap_reasons
                .iter()
                .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary counts from rows and rules.
    pub fn computed_summary(&self) -> CleanRoomRebuildProofSummary {
        CleanRoomRebuildProofSummary {
            total_rows: self.rows.len(),
            rows_holding_claim: self.rows.iter().filter(|row| row.holds_claim()).count(),
            rows_narrowed: self.rows.iter().filter(|row| !row.holds_claim()).count(),
            rows_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.family_state == ChannelFamilyState::OnWaiver)
                .count(),
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            rules_firing: self
                .rules
                .iter()
                .filter(|rule| self.rule_fires(rule))
                .count(),
        }
    }

    /// Produces a redaction-safe export projection of the proof artifact.
    pub fn support_export_projection(&self) -> CleanRoomRebuildProofExportProjection {
        CleanRoomRebuildProofExportProjection {
            artifact_id: self.artifact_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| ChannelFamilyExportRow {
                    entry_id: row.entry_id.clone(),
                    category: row.category,
                    family_kind: row.family_kind,
                    claim_label: row.claim_label.clone(),
                    effective_label: row.effective_label.clone(),
                    holds_claim: row.holds_claim(),
                    family_state: row.family_state,
                    rebuild_state: row.rebuild_state,
                    symbolication_state: row.symbolication_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the artifact, returning every violation found.
    pub fn validate(&self) -> Vec<CleanRoomRebuildProofViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(CleanRoomRebuildProofViolation::DuplicateRowId {
                    row_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(CleanRoomRebuildProofViolation::EmptyRows);
        }

        let present_kinds: BTreeSet<ChannelFamilyKind> =
            self.rows.iter().map(|row| row.family_kind).collect();
        for kind in ChannelFamilyKind::ALL {
            if !present_kinds.contains(&kind) {
                violations.push(CleanRoomRebuildProofViolation::FamilyKindMissing { kind });
            }
        }

        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(CleanRoomRebuildProofViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<CleanRoomRebuildProofViolation>) {
        if self.schema_version != CLEAN_ROOM_REBUILD_PROOF_SCHEMA_VERSION {
            violations.push(CleanRoomRebuildProofViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != CLEAN_ROOM_REBUILD_PROOF_RECORD_KIND {
            violations.push(CleanRoomRebuildProofViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("artifact_id", &self.artifact_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("build_identity_ref", &self.build_identity_ref),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("stable_proof_index_ref", &self.stable_proof_index_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(CleanRoomRebuildProofViolation::EmptyField {
                    id: "<artifact>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != canonical_lifecycle_labels() {
            violations.push(CleanRoomRebuildProofViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.family_categories != ChannelFamilyCategory::ALL.to_vec() {
            violations.push(CleanRoomRebuildProofViolation::ClosedVocabularyMismatch {
                field: "family_categories",
            });
        }
        if self.family_kinds != ChannelFamilyKind::ALL.to_vec() {
            violations.push(CleanRoomRebuildProofViolation::ClosedVocabularyMismatch {
                field: "family_kinds",
            });
        }
        if self.family_states != ChannelFamilyState::ALL.to_vec() {
            violations.push(CleanRoomRebuildProofViolation::ClosedVocabularyMismatch {
                field: "family_states",
            });
        }
        if self.rebuild_states != RebuildState::ALL.to_vec() {
            violations.push(CleanRoomRebuildProofViolation::ClosedVocabularyMismatch {
                field: "rebuild_states",
            });
        }
        if self.symbolication_states != SymbolicationState::ALL.to_vec() {
            violations.push(CleanRoomRebuildProofViolation::ClosedVocabularyMismatch {
                field: "symbolication_states",
            });
        }
        if self.gap_reasons != ChannelFamilyGapReason::ALL.to_vec() {
            violations.push(CleanRoomRebuildProofViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.family_actions != ChannelFamilyAction::ALL.to_vec() {
            violations.push(CleanRoomRebuildProofViolation::ClosedVocabularyMismatch {
                field: "family_actions",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<CleanRoomRebuildProofViolation>) {
        if self.rules.is_empty() {
            violations.push(CleanRoomRebuildProofViolation::NoRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(CleanRoomRebuildProofViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(CleanRoomRebuildProofViolation::EmptyField {
                        id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_states.is_empty() {
                violations.push(CleanRoomRebuildProofViolation::RuleWithoutStates {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in ChannelFamilyGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(CleanRoomRebuildProofViolation::GapReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &ChannelFamilyRow,
        violations: &mut Vec<CleanRoomRebuildProofViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("subject_ref", &row.subject_ref),
            ("subject_summary", &row.subject_summary),
            ("rationale", &row.rationale),
            ("claim_ref", &row.claim_ref),
            ("claim_label", &row.claim_label),
            ("effective_label", &row.effective_label),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(CleanRoomRebuildProofViolation::EmptyField {
                    id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        if row.family_state.forces_narrowing() {
            if row.effective_label == row.claim_label {
                violations.push(CleanRoomRebuildProofViolation::EffectiveLabelNotNarrowed {
                    row_id: row.entry_id.clone(),
                    state: row.family_state,
                    claim_label: row.claim_label.clone(),
                    effective_label: row.effective_label.clone(),
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(CleanRoomRebuildProofViolation::NarrowingWithoutReason {
                    row_id: row.entry_id.clone(),
                    state: row.family_state,
                });
            }
        }

        if row.family_state.holds_label() {
            if !row.active_gap_reasons.is_empty() {
                violations.push(CleanRoomRebuildProofViolation::HeldRowWithActiveGap {
                    row_id: row.entry_id.clone(),
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(CleanRoomRebuildProofViolation::HeldRowWithoutSignoff {
                    row_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_row_state_reason_coherence(row, violations);
    }

    fn validate_row_state_reason_coherence(
        &self,
        row: &ChannelFamilyRow,
        violations: &mut Vec<CleanRoomRebuildProofViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<CleanRoomRebuildProofViolation>,
                               expected: ChannelFamilyGapReason| {
            violations.push(CleanRoomRebuildProofViolation::StateReasonIncoherent {
                row_id: row.entry_id.clone(),
                state: row.family_state,
                expected_reason: expected,
            });
        };

        match row.family_state {
            ChannelFamilyState::NarrowedStale => {
                if !row.has_active_reason(ChannelFamilyGapReason::PacketFreshnessBreached) {
                    push_incoherent(violations, ChannelFamilyGapReason::PacketFreshnessBreached);
                }
            }
            ChannelFamilyState::NarrowedMissing => {
                if !row.has_active_reason(ChannelFamilyGapReason::PacketMissing) {
                    push_incoherent(violations, ChannelFamilyGapReason::PacketMissing);
                }
            }
            ChannelFamilyState::NarrowedUnbacked => {
                const ALLOWED: [ChannelFamilyGapReason; 6] = [
                    ChannelFamilyGapReason::EvidenceIncomplete,
                    ChannelFamilyGapReason::RebuildNotVerified,
                    ChannelFamilyGapReason::SymbolicationUnlinked,
                    ChannelFamilyGapReason::ParityMismatch,
                    ChannelFamilyGapReason::OwnerSignoffMissing,
                    ChannelFamilyGapReason::ClaimLabelNarrowed,
                ];
                if !ALLOWED.iter().any(|reason| row.has_active_reason(*reason)) {
                    push_incoherent(violations, ChannelFamilyGapReason::EvidenceIncomplete);
                }
            }
            ChannelFamilyState::NarrowedWaiverExpired => {
                if !row.has_active_reason(ChannelFamilyGapReason::WaiverExpired) {
                    push_incoherent(violations, ChannelFamilyGapReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(CleanRoomRebuildProofViolation::WaiverStateWithoutWaiver {
                        row_id: row.entry_id.clone(),
                        state: row.family_state,
                    });
                }
            }
            ChannelFamilyState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|waiver| {
                        waiver.waiver_ref.trim().is_empty() || waiver.expires_at.trim().is_empty()
                    })
                    .unwrap_or(true)
                {
                    violations.push(CleanRoomRebuildProofViolation::WaiverStateWithoutWaiver {
                        row_id: row.entry_id.clone(),
                        state: row.family_state,
                    });
                }
            }
            ChannelFamilyState::Current => {}
        }
    }

    fn validate_publication(&self, violations: &mut Vec<CleanRoomRebuildProofViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(CleanRoomRebuildProofViolation::EmptyField {
                id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(CleanRoomRebuildProofViolation::EmptyField {
                id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                CleanRoomRebuildProofViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                CleanRoomRebuildProofViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_row_ids != self.computed_blocking_row_ids() {
            violations.push(
                CleanRoomRebuildProofViolation::PublicationBlockingSetMismatch {
                    field: "blocking_row_ids",
                },
            );
        }
    }
}

/// A redaction-safe export row projected from a family row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelFamilyExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// Row category.
    pub category: ChannelFamilyCategory,
    /// Family kind.
    pub family_kind: ChannelFamilyKind,
    /// Canonical lifecycle label published by the claim.
    pub claim_label: String,
    /// Effective lifecycle label after narrowing.
    pub effective_label: String,
    /// True when the row still holds its claim.
    pub holds_claim: bool,
    /// State earned by the row.
    pub family_state: ChannelFamilyState,
    /// Rebuild posture earned by the row.
    pub rebuild_state: RebuildState,
    /// Symbolication posture earned by the row.
    pub symbolication_state: SymbolicationState,
    /// Active gap reasons on the row.
    pub active_gap_reasons: Vec<ChannelFamilyGapReason>,
}

/// A redaction-safe export projection of the proof artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanRoomRebuildProofExportProjection {
    /// Artifact id this projection was produced from.
    pub artifact_id: String,
    /// Artifact as-of date.
    pub as_of: String,
    /// Publication decision.
    pub publication_decision: PublicationDecision,
    /// Projected rows.
    pub rows: Vec<ChannelFamilyExportRow>,
}

/// A validation violation for the clean-room rebuild proof artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CleanRoomRebuildProofViolation {
    /// The artifact carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the artifact.
        actual: u32,
    },
    /// The artifact carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the artifact.
        actual: String,
    },
    /// A closed vocabulary is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The artifact has no rows.
    EmptyRows,
    /// The artifact has no rules.
    NoRules,
    /// A required family kind is missing from the rows.
    FamilyKindMissing {
        /// Missing kind.
        kind: ChannelFamilyKind,
    },
    /// A required field is empty.
    EmptyField {
        /// Row, rule, or section id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A rule names no states to watch.
    RuleWithoutStates {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no rule watching for it.
    GapReasonWithoutRule {
        /// Uncovered reason.
        reason: ChannelFamilyGapReason,
    },
    /// A narrowing state did not narrow its effective label.
    EffectiveLabelNotNarrowed {
        /// Row id.
        row_id: String,
        /// Row state.
        state: ChannelFamilyState,
        /// Claim label.
        claim_label: String,
        /// Effective label.
        effective_label: String,
    },
    /// A narrowing row carries no active reason.
    NarrowingWithoutReason {
        /// Row id.
        row_id: String,
        /// Row state.
        state: ChannelFamilyState,
    },
    /// A held row carries an active gap reason.
    HeldRowWithActiveGap {
        /// Row id.
        row_id: String,
    },
    /// A held row lacks owner sign-off.
    HeldRowWithoutSignoff {
        /// Row id.
        row_id: String,
    },
    /// A row state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        row_id: String,
        /// Row state.
        state: ChannelFamilyState,
        /// Reason the state requires.
        expected_reason: ChannelFamilyGapReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        row_id: String,
        /// Row state.
        state: ChannelFamilyState,
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
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for CleanRoomRebuildProofViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported artifact schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported artifact record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "artifact {field} is not the canonical value")
            }
            Self::EmptyRows => write!(f, "artifact has no channel family rows"),
            Self::NoRules => write!(f, "artifact has no rules"),
            Self::FamilyKindMissing { kind } => {
                write!(f, "missing row for family kind {}", kind.as_str())
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id } => {
                write!(f, "duplicate row id {row_id}")
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate rule id {rule_id}")
            }
            Self::RuleWithoutStates { rule_id } => {
                write!(f, "rule {rule_id} watches no states")
            }
            Self::GapReasonWithoutRule { reason } => write!(
                f,
                "gap reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::EffectiveLabelNotNarrowed {
                row_id,
                state,
                claim_label,
                effective_label,
            } => write!(
                f,
                "row {row_id} state {} must narrow below claim label {claim_label} but effective is {effective_label}",
                state.as_str()
            ),
            Self::NarrowingWithoutReason { row_id, state } => write!(
                f,
                "row {row_id} state {} narrows without naming an active gap reason",
                state.as_str()
            ),
            Self::HeldRowWithActiveGap { row_id } => {
                write!(f, "row {row_id} holds claim while a gap reason is active")
            }
            Self::HeldRowWithoutSignoff { row_id } => {
                write!(f, "row {row_id} holds claim without owner sign-off")
            }
            Self::StateReasonIncoherent {
                row_id,
                state,
                expected_reason,
            } => write!(
                f,
                "row {row_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { row_id, state } => write!(
                f,
                "row {row_id} state {} names no waiver",
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
                "publication {field} disagrees with the firing rules"
            ),
            Self::SummaryMismatch => write!(f, "artifact summary counts disagree with the rows"),
        }
    }
}

impl Error for CleanRoomRebuildProofViolation {}

/// Loads the embedded clean-room rebuild proof artifact.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in artifact no longer matches
/// [`CleanRoomRebuildProof`].
pub fn current_clean_room_rebuild_proof() -> Result<CleanRoomRebuildProof, serde_json::Error> {
    serde_json::from_str(CLEAN_ROOM_REBUILD_PROOF_JSON)
}

fn canonical_lifecycle_labels() -> Vec<String> {
    CANONICAL_LIFECYCLE_LABELS
        .iter()
        .map(|label| (*label).to_owned())
        .collect()
}
