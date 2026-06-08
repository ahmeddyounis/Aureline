//! Typed stable boundary manifest across the local-OSS, self-hosted, managed, and
//! air-gapped value lines.
//!
//! Where the [`stable_claim_manifest`](crate::stable_claim_manifest) decides the
//! single canonical lifecycle label each subject publishes, that one label does
//! not survive contact with how the product is *deployed*: a subject that is
//! Stable when a hosted gateway is reachable can be unsupported when the same
//! workflow runs air-gapped. This module is the **boundary** layer on top of the
//! claim manifest. For every published subject it records, for each of the four
//! value lines, the lifecycle label that line can actually carry and why it
//! narrows when it cannot match the subject's canonical label.
//!
//! Each [`BoundaryRow`] is one `(subject, value line)` cell. It:
//!
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry it
//!   speaks for ([`BoundaryRow::manifest_entry_ref`]) and the canonical lifecycle
//!   label that entry publishes ([`BoundaryRow::manifest_label`]). That label is a
//!   hard **ceiling**: a value line may match it or narrow below it, but it may
//!   never publish wider than the subject's canonical label;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-line
//!   labels, so docs, Help/About, the release center, and support exports ingest
//!   one label per cell instead of cloning their own;
//! - carries a per-line proof packet with a packet-freshness SLO
//!   ([`ProofPacket`]), the boundary state earned ([`BoundaryState`]), the active
//!   narrowing reasons ([`NarrowingReason`]), and the label it *effectively*
//!   publishes after narrowing ([`BoundaryRow::published_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between a value line that publishes Stable and one narrowed below it. A line
//! that cannot support a subject — because the line lacks a capability the subject
//! needs, because its line-specific evidence is incomplete, because its proof
//! packet aged out, or because the subject's own canonical label is already below
//! the cutline — is structurally required to drop below the cutline rather than
//! inherit the subject's published label. The [`BoundaryRule`] set names the closed
//! conditions that gate boundary publication, and [`StableBoundaryManifest::publication`]
//! records the resulting proceed/hold verdict.
//!
//! The manifest is checked in at
//! `artifacts/release/stable_boundary_manifest.json` and embedded here, so this
//! typed consumer and the CI gate agree on every cell without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material. Two
//! classes of check live outside this model because they need more than the
//! manifest sees: date arithmetic (recomputing the packet-freshness state and
//! waiver expiry against an `as_of` date) and the cross-artifact ceiling check
//! (whether each row's `manifest_label` still equals the label the stable claim
//! manifest publishes for that entry). Both live in the CI gate. This model
//! enforces the structural and logical invariants that hold regardless of the
//! clock and the neighbouring artifact — the ceiling/no-widening rule, narrowing
//! consistency, packet/state coherence, owner sign-off on published cells,
//! per-subject value-line coverage, publication-rule wiring, and the verdict.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported manifest schema version.
pub const STABLE_BOUNDARY_MANIFEST_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the manifest.
pub const STABLE_BOUNDARY_MANIFEST_RECORD_KIND: &str = "stable_boundary_manifest";

/// Repo-relative path to the checked-in manifest.
pub const STABLE_BOUNDARY_MANIFEST_PATH: &str = "artifacts/release/stable_boundary_manifest.json";

/// Embedded checked-in manifest JSON.
pub const STABLE_BOUNDARY_MANIFEST_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/stable_boundary_manifest.json"
));

/// The deployment value line a boundary row speaks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueLine {
    /// Account-free local open-source mode: no hosted dependency.
    LocalOss,
    /// Customer-run control plane and mirrors.
    SelfHosted,
    /// Vendor-hosted convenience services (managed gateway, sync, marketplace).
    Managed,
    /// Offline / sovereign deployment with no outbound connectivity.
    AirGapped,
}

impl ValueLine {
    /// Every value line, least-managed to most-isolated.
    pub const ALL: [Self; 4] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
        }
    }
}

/// Boundary state a value line earned for a subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryState {
    /// The line carries the subject at its full canonical lifecycle label.
    Available,
    /// The line carries the subject only because an active, unexpired waiver
    /// covers a recorded line-specific gap.
    AvailableOnWaiver,
    /// The line cannot carry the subject: a capability is absent or the
    /// line-specific qualification is incomplete; the label must narrow.
    NarrowedUnsupported,
    /// The subject's canonical manifest label is itself below the cutline, so
    /// every value line inherits that ceiling and narrows.
    NarrowedByManifest,
    /// The line's proof packet breached its freshness SLO (or is missing); the
    /// label must narrow.
    NarrowedStale,
    /// The line relied on a waiver that has expired; the label must narrow.
    NarrowedWaiverExpired,
}

impl BoundaryState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Available,
        Self::AvailableOnWaiver,
        Self::NarrowedUnsupported,
        Self::NarrowedByManifest,
        Self::NarrowedStale,
        Self::NarrowedWaiverExpired,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::AvailableOnWaiver => "available_on_waiver",
            Self::NarrowedUnsupported => "narrowed_unsupported",
            Self::NarrowedByManifest => "narrowed_by_manifest",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
        }
    }

    /// Whether the state lets a value line carry the subject's claimed label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Available | Self::AvailableOnWaiver)
    }

    /// Whether the state forces the value line below its claimed label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a value-line label narrows or a boundary rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// The subject's canonical manifest label is itself below the cutline.
    ManifestLabelNarrowed,
    /// The value line lacks a capability the subject needs (e.g. no outbound
    /// provider routing in the air-gapped line).
    LineCapabilityAbsent,
    /// The line-specific qualification evidence is missing or narrowed.
    LineEvidenceIncomplete,
    /// The line's proof packet breached its freshness SLO.
    BoundaryPacketFreshnessBreached,
    /// No proof packet has been captured for the line.
    BoundaryPacketMissing,
    /// A waiver the line relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl NarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ManifestLabelNarrowed,
        Self::LineCapabilityAbsent,
        Self::LineEvidenceIncomplete,
        Self::BoundaryPacketFreshnessBreached,
        Self::BoundaryPacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManifestLabelNarrowed => "manifest_label_narrowed",
            Self::LineCapabilityAbsent => "line_capability_absent",
            Self::LineEvidenceIncomplete => "line_evidence_incomplete",
            Self::BoundaryPacketFreshnessBreached => "boundary_packet_freshness_breached",
            Self::BoundaryPacketMissing => "boundary_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a boundary rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryAction {
    /// Hold boundary publication until the condition clears.
    HoldPublication,
    /// Narrow the value line's published label below the cutline.
    NarrowLineLabel,
    /// Refresh the line's proof packet so it re-enters its freshness SLO.
    RefreshBoundaryPacket,
    /// Re-validate the value line's support for the subject.
    RevalidateLineSupport,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl BoundaryAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HoldPublication,
        Self::NarrowLineLabel,
        Self::RefreshBoundaryPacket,
        Self::RevalidateLineSupport,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowLineLabel => "narrow_line_label",
            Self::RefreshBoundaryPacket => "refresh_boundary_packet",
            Self::RevalidateLineSupport => "revalidate_line_support",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// Descriptive profile for one value line in the closed `value_lines` vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValueLineProfile {
    /// The value line this profile describes.
    pub value_line: ValueLine,
    /// Human-readable title.
    pub title: String,
    /// The line's outbound-connectivity posture (e.g. "no outbound network").
    pub connectivity_posture: String,
    /// Reviewable description of what the line guarantees.
    pub description: String,
}

/// One boundary rule: a closed condition that narrows a value-line label and may
/// gate boundary publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched row fires this rule.
    pub trigger_reason: NarrowingReason,
    /// Subject manifest labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: BoundaryAction,
    /// Whether firing this rule blocks boundary publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One stable boundary row: a `(subject, value line)` cell bound to its manifest
/// entry, the canonical ceiling label, and its line proof-packet freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryRow {
    /// Stable boundary id.
    pub boundary_id: String,
    /// Human-readable title.
    pub title: String,
    /// Subject family the row speaks for.
    pub subject_family: String,
    /// The value line this row records.
    pub value_line: ValueLine,
    /// The stable-claim-manifest entry id this row's subject maps to.
    pub manifest_entry_ref: String,
    /// The canonical lifecycle label the manifest publishes for the subject. The
    /// ceiling: a value line may never publish wider than this.
    pub manifest_label: StableClaimLevel,
    /// Boundary state earned by the value line.
    pub boundary_state: BoundaryState,
    /// Ref into the line-capability/profile record this line's support depends on.
    pub line_capability_ref: String,
    /// The line proof packet and its freshness SLO.
    pub boundary_packet: ProofPacket,
    /// Waiver authorizing a provisional line label, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons narrowing the row.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// The lifecycle label the value line effectively publishes after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl BoundaryRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the subject's canonical manifest label is at or above the cutline.
    pub fn manifest_holds_stable(&self) -> bool {
        self.manifest_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the value line carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.boundary_state.holds_label()
    }

    /// True when a narrowing reason is active on the row.
    pub fn has_active_reason(&self, reason: NarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }
}

/// Per-value-line rollup of how many subjects a line carries Stable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValueLineRollup {
    /// The value line summarized.
    pub value_line: ValueLine,
    /// Total boundary rows for the line.
    pub total: usize,
    /// Rows publishing a label at or above the cutline.
    pub published_stable: usize,
    /// Rows narrowed below the cutline.
    pub narrowed_below_cutline: usize,
}

/// The recorded publication verdict for the stable boundary manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryPublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Boundary-rule ids that block publication, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Boundary ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_boundary_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StableBoundaryManifestSummary {
    /// Total number of boundary rows.
    pub total_boundaries: usize,
    /// Distinct subjects covered.
    pub total_subjects: usize,
    /// Rows publishing a label at or above the cutline.
    pub boundaries_published_stable: usize,
    /// Rows narrowed below the cutline.
    pub boundaries_narrowed_below_cutline: usize,
    /// Rows holding a label via an active waiver.
    pub boundaries_on_active_waiver: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active narrowing reasons across all rows.
    pub total_active_narrowing_reasons: usize,
    /// Number of boundary rules currently firing.
    pub boundary_rules_firing: usize,
    /// Per-value-line rollups, one per value line in canonical order.
    pub line_rollups: Vec<ValueLineRollup>,
}

/// The typed stable boundary manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StableBoundaryManifest {
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
    /// Ref to the stable claim manifest this manifest ingests as its ceiling.
    pub claim_manifest_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed value-line vocabulary with profiles.
    pub value_lines: Vec<ValueLineProfile>,
    /// Closed boundary-state vocabulary.
    pub boundary_states: Vec<BoundaryState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed boundary-action vocabulary.
    pub boundary_actions: Vec<BoundaryAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// Boundary rules.
    pub boundary_rules: Vec<BoundaryRule>,
    /// Boundary rows.
    pub rows: Vec<BoundaryRow>,
    /// Recorded publication verdict.
    pub publication: BoundaryPublicationRecord,
    /// Summary counts.
    pub summary: StableBoundaryManifestSummary,
}

impl StableBoundaryManifest {
    /// Returns the row registered for `boundary_id`.
    pub fn row(&self, boundary_id: &str) -> Option<&BoundaryRow> {
        self.rows.iter().find(|row| row.boundary_id == boundary_id)
    }

    /// Returns the rows for one value line.
    pub fn rows_for_line(&self, line: ValueLine) -> Vec<&BoundaryRow> {
        self.rows
            .iter()
            .filter(|row| row.value_line == line)
            .collect()
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&BoundaryRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&BoundaryRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Distinct subjects (by manifest entry ref) the manifest covers.
    pub fn subjects(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.manifest_entry_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn boundary_rule_fires(&self, rule: &BoundaryRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.manifest_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and boundary rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .boundary_rules
            .iter()
            .any(|rule| rule.blocks_publication && self.boundary_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .boundary_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.boundary_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Boundary ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose subject ceiling is at or above the cutline count: a row
    /// whose subject is already canonically narrowed is not a *stable-boundary*
    /// blocker, it merely inherits the upstream ceiling.
    pub fn computed_blocking_boundary_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<NarrowingReason> = self
            .boundary_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.boundary_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row.manifest_holds_stable()
                && row
                    .active_narrowing_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.boundary_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the per-value-line rollups.
    pub fn computed_line_rollups(&self) -> Vec<ValueLineRollup> {
        ValueLine::ALL
            .iter()
            .map(|&line| {
                let rows = self.rows_for_line(line);
                let published_stable = rows.iter().filter(|row| row.publishes_stable()).count();
                ValueLineRollup {
                    value_line: line,
                    total: rows.len(),
                    published_stable,
                    narrowed_below_cutline: rows.len() - published_stable,
                }
            })
            .collect()
    }

    /// Recomputes the summary block from the rows and boundary rules.
    pub fn computed_summary(&self) -> StableBoundaryManifestSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.boundary_packet.slo_state == state)
                .count()
        };
        StableBoundaryManifestSummary {
            total_boundaries: self.rows.len(),
            total_subjects: self.subjects().len(),
            boundaries_published_stable: self
                .rows
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            boundaries_narrowed_below_cutline: self
                .rows
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            boundaries_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.boundary_state == BoundaryState::AvailableOnWaiver)
                .count(),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_narrowing_reasons: self
                .rows
                .iter()
                .map(|row| row.active_narrowing_reasons.len())
                .sum(),
            boundary_rules_firing: self
                .boundary_rules
                .iter()
                .filter(|rule| self.boundary_rule_fires(rule))
                .count(),
            line_rollups: self.computed_line_rollups(),
        }
    }

    /// Produces an export/Help-About-safe projection of the manifest that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> BoundaryExportProjection {
        BoundaryExportProjection {
            manifest_id: self.manifest_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            line_rollups: self.computed_line_rollups(),
            rows: self
                .rows
                .iter()
                .map(|row| BoundaryExportRow {
                    boundary_id: row.boundary_id.clone(),
                    subject_family: row.subject_family.clone(),
                    value_line: row.value_line,
                    manifest_label: row.manifest_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    boundary_state: row.boundary_state,
                    slo_state: row.boundary_packet.slo_state,
                    active_narrowing_reasons: row.active_narrowing_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the manifest, returning every violation found.
    pub fn validate(&self) -> Vec<StableBoundaryManifestViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.boundary_id.clone()) {
                violations.push(StableBoundaryManifestViolation::DuplicateBoundaryId {
                    boundary_id: row.boundary_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(StableBoundaryManifestViolation::EmptyManifest);
        }

        self.validate_subject_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(StableBoundaryManifestViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<StableBoundaryManifestViolation>) {
        if self.schema_version != STABLE_BOUNDARY_MANIFEST_SCHEMA_VERSION {
            violations.push(StableBoundaryManifestViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != STABLE_BOUNDARY_MANIFEST_RECORD_KIND {
            violations.push(StableBoundaryManifestViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("manifest_id", &self.manifest_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(StableBoundaryManifestViolation::EmptyField {
                    boundary_id: "<manifest>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(StableBoundaryManifestViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.boundary_states != BoundaryState::ALL.to_vec() {
            violations.push(StableBoundaryManifestViolation::ClosedVocabularyMismatch {
                field: "boundary_states",
            });
        }
        if self.narrowing_reasons != NarrowingReason::ALL.to_vec() {
            violations.push(StableBoundaryManifestViolation::ClosedVocabularyMismatch {
                field: "narrowing_reasons",
            });
        }
        if self.boundary_actions != BoundaryAction::ALL.to_vec() {
            violations.push(StableBoundaryManifestViolation::ClosedVocabularyMismatch {
                field: "boundary_actions",
            });
        }

        let lines: Vec<ValueLine> = self.value_lines.iter().map(|p| p.value_line).collect();
        if lines != ValueLine::ALL.to_vec() {
            violations.push(StableBoundaryManifestViolation::ClosedVocabularyMismatch {
                field: "value_lines",
            });
        }
        for profile in &self.value_lines {
            for (field, value) in [
                ("title", &profile.title),
                ("connectivity_posture", &profile.connectivity_posture),
                ("description", &profile.description),
            ] {
                if value.trim().is_empty() {
                    violations.push(StableBoundaryManifestViolation::EmptyField {
                        boundary_id: format!("<value_line:{}>", profile.value_line.as_str()),
                        field_name: field,
                    });
                }
            }
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(StableBoundaryManifestViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(StableBoundaryManifestViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(StableBoundaryManifestViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(StableBoundaryManifestViolation::EmptyField {
                boundary_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<StableBoundaryManifestViolation>) {
        if self.boundary_rules.is_empty() {
            violations.push(StableBoundaryManifestViolation::NoBoundaryRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.boundary_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(StableBoundaryManifestViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(StableBoundaryManifestViolation::EmptyField {
                        boundary_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(StableBoundaryManifestViolation::RuleWithoutLabels {
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
                    .push(StableBoundaryManifestViolation::NarrowingReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &BoundaryRow,
        violations: &mut Vec<StableBoundaryManifestViolation>,
    ) {
        for (field, value) in [
            ("boundary_id", &row.boundary_id),
            ("title", &row.title),
            ("subject_family", &row.subject_family),
            ("manifest_entry_ref", &row.manifest_entry_ref),
            ("line_capability_ref", &row.line_capability_ref),
            ("rationale", &row.rationale),
            ("boundary_packet.packet_id", &row.boundary_packet.packet_id),
            (
                "boundary_packet.packet_ref",
                &row.boundary_packet.packet_ref,
            ),
            (
                "boundary_packet.proof_index_ref",
                &row.boundary_packet.proof_index_ref,
            ),
            (
                "boundary_packet.freshness_slo.slo_register_ref",
                &row.boundary_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(StableBoundaryManifestViolation::EmptyField {
                    boundary_id: row.boundary_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no value line may publish wider than the subject's
        // canonical manifest label.
        if row.published_label.rank() > row.manifest_label.rank() {
            violations.push(
                StableBoundaryManifestViolation::PublishedWiderThanManifest {
                    boundary_id: row.boundary_id.clone(),
                    manifest: row.manifest_label,
                    published: row.published_label,
                },
            );
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.boundary_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(StableBoundaryManifestViolation::EmptyField {
                boundary_id: row.boundary_id.clone(),
                field_name: "boundary_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.boundary_packet.freshness_slo.window_is_consistent() {
            violations.push(StableBoundaryManifestViolation::FreshnessSloInconsistent {
                boundary_id: row.boundary_id.clone(),
            });
        }

        // A subject whose canonical manifest label is below the cutline forces
        // every value line to inherit that ceiling and narrow.
        if !row.manifest_holds_stable() {
            if row.holds_label() {
                violations.push(StableBoundaryManifestViolation::HeldOnNarrowedManifest {
                    boundary_id: row.boundary_id.clone(),
                    manifest: row.manifest_label,
                });
            }
            if !row.has_active_reason(NarrowingReason::ManifestLabelNarrowed) {
                violations.push(
                    StableBoundaryManifestViolation::ManifestNarrowedWithoutReason {
                        boundary_id: row.boundary_id.clone(),
                    },
                );
            }
        }

        let slo_state = row.boundary_packet.slo_state;

        if row.holds_label() {
            // A held value line publishes exactly the subject's canonical label,
            // carries no active narrowing reason, rides a captured within-SLO
            // packet, and is owner-signed.
            if row.published_label != row.manifest_label {
                violations.push(StableBoundaryManifestViolation::HeldLabelNotEqualManifest {
                    boundary_id: row.boundary_id.clone(),
                    manifest: row.manifest_label,
                    published: row.published_label,
                });
            }
            if !row.active_narrowing_reasons.is_empty() {
                violations.push(StableBoundaryManifestViolation::HeldWithActiveNarrowing {
                    boundary_id: row.boundary_id.clone(),
                });
            }
            if !row.boundary_packet.has_capture() {
                violations.push(StableBoundaryManifestViolation::HeldWithoutFreshPacket {
                    boundary_id: row.boundary_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(StableBoundaryManifestViolation::HeldOnStalePacket {
                    boundary_id: row.boundary_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(StableBoundaryManifestViolation::HeldWithoutSignoff {
                    boundary_id: row.boundary_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the published label below the cutline and
            // name at least one active reason.
            if row.publishes_stable() {
                violations.push(StableBoundaryManifestViolation::PublishedLabelNotNarrowed {
                    boundary_id: row.boundary_id.clone(),
                    state: row.boundary_state,
                    published: row.published_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(StableBoundaryManifestViolation::NarrowingWithoutReason {
                    boundary_id: row.boundary_id.clone(),
                    state: row.boundary_state,
                });
            }
            // A narrowing row whose packet is breached or missing must name the
            // matching freshness reason, so the freshness automation stays honest.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(NarrowingReason::BoundaryPacketFreshnessBreached)
            {
                violations.push(
                    StableBoundaryManifestViolation::BreachedPacketWithoutReason {
                        boundary_id: row.boundary_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(NarrowingReason::BoundaryPacketMissing)
            {
                violations.push(
                    StableBoundaryManifestViolation::MissingPacketWithoutReason {
                        boundary_id: row.boundary_id.clone(),
                    },
                );
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_state_reason_coherence(
        &self,
        row: &BoundaryRow,
        violations: &mut Vec<StableBoundaryManifestViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<StableBoundaryManifestViolation>,
                               expected: NarrowingReason| {
            violations.push(StableBoundaryManifestViolation::StateReasonIncoherent {
                boundary_id: row.boundary_id.clone(),
                state: row.boundary_state,
                expected_reason: expected,
            });
        };

        match row.boundary_state {
            BoundaryState::NarrowedUnsupported => {
                const ALLOWED: [NarrowingReason; 3] = [
                    NarrowingReason::LineCapabilityAbsent,
                    NarrowingReason::LineEvidenceIncomplete,
                    NarrowingReason::OwnerSignoffMissing,
                ];
                if !ALLOWED.iter().any(|r| row.has_active_reason(*r)) {
                    push_incoherent(violations, NarrowingReason::LineCapabilityAbsent);
                }
            }
            BoundaryState::NarrowedByManifest => {
                if !row.has_active_reason(NarrowingReason::ManifestLabelNarrowed) {
                    push_incoherent(violations, NarrowingReason::ManifestLabelNarrowed);
                }
            }
            BoundaryState::NarrowedStale => {
                if !(row.has_active_reason(NarrowingReason::BoundaryPacketFreshnessBreached)
                    || row.has_active_reason(NarrowingReason::BoundaryPacketMissing))
                {
                    push_incoherent(violations, NarrowingReason::BoundaryPacketFreshnessBreached);
                }
            }
            BoundaryState::NarrowedWaiverExpired => {
                if !row.has_active_reason(NarrowingReason::WaiverExpired) {
                    push_incoherent(violations, NarrowingReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(StableBoundaryManifestViolation::WaiverStateWithoutWaiver {
                        boundary_id: row.boundary_id.clone(),
                        state: row.boundary_state,
                    });
                }
            }
            BoundaryState::AvailableOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(StableBoundaryManifestViolation::WaiverStateWithoutWaiver {
                        boundary_id: row.boundary_id.clone(),
                        state: row.boundary_state,
                    });
                }
            }
            BoundaryState::Available => {}
        }
    }

    fn validate_subject_coverage(&self, violations: &mut Vec<StableBoundaryManifestViolation>) {
        // Each subject must carry exactly one row per value line: the manifest is
        // a full subject x value-line matrix, and a subject's canonical label must
        // be the same ceiling everywhere it appears.
        let mut by_subject: BTreeMap<String, Vec<&BoundaryRow>> = BTreeMap::new();
        for row in &self.rows {
            by_subject
                .entry(row.manifest_entry_ref.clone())
                .or_default()
                .push(row);
        }
        for (subject, rows) in &by_subject {
            let mut lines: BTreeSet<ValueLine> = BTreeSet::new();
            for row in rows {
                if !lines.insert(row.value_line) {
                    violations.push(StableBoundaryManifestViolation::DuplicateLineForSubject {
                        manifest_entry_ref: subject.clone(),
                        value_line: row.value_line,
                    });
                }
            }
            for line in ValueLine::ALL {
                if !lines.contains(&line) {
                    violations.push(StableBoundaryManifestViolation::SubjectMissingValueLine {
                        manifest_entry_ref: subject.clone(),
                        value_line: line,
                    });
                }
            }
            let labels: BTreeSet<StableClaimLevel> =
                rows.iter().map(|row| row.manifest_label).collect();
            if labels.len() > 1 {
                violations.push(
                    StableBoundaryManifestViolation::InconsistentManifestLabelForSubject {
                        manifest_entry_ref: subject.clone(),
                    },
                );
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<StableBoundaryManifestViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(StableBoundaryManifestViolation::EmptyField {
                boundary_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(StableBoundaryManifestViolation::EmptyField {
                boundary_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                StableBoundaryManifestViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                StableBoundaryManifestViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_boundary_ids != self.computed_blocking_boundary_ids() {
            violations.push(
                StableBoundaryManifestViolation::PublicationBlockingSetMismatch {
                    field: "blocking_boundary_ids",
                },
            );
        }
    }
}

/// A redaction-safe export row projected from the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryExportRow {
    /// Stable boundary id.
    pub boundary_id: String,
    /// Subject family.
    pub subject_family: String,
    /// Value line.
    pub value_line: ValueLine,
    /// The subject's canonical ceiling label.
    pub manifest_label: StableClaimLevel,
    /// Lifecycle label the value line publishes.
    pub published_label: StableClaimLevel,
    /// Whether the row publishes a label at or above the cutline.
    pub publishes_stable: bool,
    /// Boundary state.
    pub boundary_state: BoundaryState,
    /// Proof-packet freshness-SLO state.
    pub slo_state: FreshnessSloState,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
}

/// A redaction-safe export projection of the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryExportProjection {
    /// Manifest id this projection was produced from.
    pub manifest_id: String,
    /// Manifest as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Per-value-line rollups.
    pub line_rollups: Vec<ValueLineRollup>,
    /// Projected rows.
    pub rows: Vec<BoundaryExportRow>,
}

/// A validation violation for the stable boundary manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableBoundaryManifestViolation {
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
    /// The manifest has no rows.
    EmptyManifest,
    /// The manifest has no boundary rules.
    NoBoundaryRules,
    /// A required field is empty.
    EmptyField {
        /// Row, rule, or section id.
        boundary_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A boundary id appears more than once.
    DuplicateBoundaryId {
        /// Duplicate boundary id.
        boundary_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A boundary rule names no labels to watch.
    RuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A narrowing reason has no rule watching for it.
    NarrowingReasonWithoutRule {
        /// Uncovered reason.
        reason: NarrowingReason,
    },
    /// A published label is wider than the subject's canonical manifest label.
    PublishedWiderThanManifest {
        /// Row id.
        boundary_id: String,
        /// Manifest ceiling label.
        manifest: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A freshness SLO's warn window exceeds its target age.
    FreshnessSloInconsistent {
        /// Row id.
        boundary_id: String,
    },
    /// A value line holds a label while the subject's canonical label is narrowed.
    HeldOnNarrowedManifest {
        /// Row id.
        boundary_id: String,
        /// Manifest ceiling label.
        manifest: StableClaimLevel,
    },
    /// A subject narrowed by the manifest does not carry the manifest reason.
    ManifestNarrowedWithoutReason {
        /// Row id.
        boundary_id: String,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Row id.
        boundary_id: String,
        /// Boundary state.
        state: BoundaryState,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A narrowing state carries no active narrowing reason.
    NarrowingWithoutReason {
        /// Row id.
        boundary_id: String,
        /// Boundary state.
        state: BoundaryState,
    },
    /// A held row's published label is not equal to its manifest ceiling label.
    HeldLabelNotEqualManifest {
        /// Row id.
        boundary_id: String,
        /// Manifest ceiling label.
        manifest: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A held row carries an active narrowing reason.
    HeldWithActiveNarrowing {
        /// Row id.
        boundary_id: String,
    },
    /// A held row rides a proof packet with no capture or evidence.
    HeldWithoutFreshPacket {
        /// Row id.
        boundary_id: String,
    },
    /// A held row rides a proof packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Row id.
        boundary_id: String,
        /// The packet's freshness-SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held row has no owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        boundary_id: String,
    },
    /// A narrowing row with a breached packet does not name the breach reason.
    BreachedPacketWithoutReason {
        /// Row id.
        boundary_id: String,
    },
    /// A narrowing row with a missing packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Row id.
        boundary_id: String,
    },
    /// A boundary state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        boundary_id: String,
        /// Boundary state.
        state: BoundaryState,
        /// Reason the state requires.
        expected_reason: NarrowingReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        boundary_id: String,
        /// Boundary state.
        state: BoundaryState,
    },
    /// A subject is missing a row for a value line.
    SubjectMissingValueLine {
        /// Subject manifest entry ref.
        manifest_entry_ref: String,
        /// The missing value line.
        value_line: ValueLine,
    },
    /// A subject carries more than one row for a value line.
    DuplicateLineForSubject {
        /// Subject manifest entry ref.
        manifest_entry_ref: String,
        /// The duplicated value line.
        value_line: ValueLine,
    },
    /// A subject carries different manifest ceiling labels across its rows.
    InconsistentManifestLabelForSubject {
        /// Subject manifest entry ref.
        manifest_entry_ref: String,
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
}

impl fmt::Display for StableBoundaryManifestViolation {
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
            Self::EmptyManifest => write!(f, "manifest has no rows"),
            Self::NoBoundaryRules => write!(f, "manifest has no boundary rules"),
            Self::EmptyField {
                boundary_id,
                field_name,
            } => write!(f, "{boundary_id} has empty field {field_name}"),
            Self::DuplicateBoundaryId { boundary_id } => {
                write!(f, "duplicate boundary row id {boundary_id}")
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate boundary rule id {rule_id}")
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "boundary rule {rule_id} watches no labels")
            }
            Self::NarrowingReasonWithoutRule { reason } => write!(
                f,
                "narrowing reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::PublishedWiderThanManifest {
                boundary_id,
                manifest,
                published,
            } => write!(
                f,
                "boundary {boundary_id} published label {} is wider than the manifest ceiling {}",
                published.as_str(),
                manifest.as_str()
            ),
            Self::FreshnessSloInconsistent { boundary_id } => write!(
                f,
                "boundary {boundary_id} freshness SLO warn window exceeds its target age"
            ),
            Self::HeldOnNarrowedManifest {
                boundary_id,
                manifest,
            } => write!(
                f,
                "boundary {boundary_id} holds a label while the subject's manifest label {} is below the cutline",
                manifest.as_str()
            ),
            Self::ManifestNarrowedWithoutReason { boundary_id } => write!(
                f,
                "boundary {boundary_id} subject is narrowed by the manifest but does not name manifest_label_narrowed"
            ),
            Self::PublishedLabelNotNarrowed {
                boundary_id,
                state,
                published,
            } => write!(
                f,
                "boundary {boundary_id} state {} must narrow below the cutline but publishes {}",
                state.as_str(),
                published.as_str()
            ),
            Self::NarrowingWithoutReason { boundary_id, state } => write!(
                f,
                "boundary {boundary_id} state {} narrows without naming an active narrowing reason",
                state.as_str()
            ),
            Self::HeldLabelNotEqualManifest {
                boundary_id,
                manifest,
                published,
            } => write!(
                f,
                "boundary {boundary_id} publishes {} but its subject's manifest label is {}",
                published.as_str(),
                manifest.as_str()
            ),
            Self::HeldWithActiveNarrowing { boundary_id } => write!(
                f,
                "boundary {boundary_id} publishes its label while a narrowing reason is active"
            ),
            Self::HeldWithoutFreshPacket { boundary_id } => write!(
                f,
                "boundary {boundary_id} publishes its label with no captured, evidence-backed proof packet"
            ),
            Self::HeldOnStalePacket {
                boundary_id,
                slo_state,
            } => write!(
                f,
                "boundary {boundary_id} publishes its label while its packet is {} (outside its freshness SLO)",
                slo_state.as_str()
            ),
            Self::HeldWithoutSignoff { boundary_id } => {
                write!(f, "boundary {boundary_id} publishes its label without owner sign-off")
            }
            Self::BreachedPacketWithoutReason { boundary_id } => write!(
                f,
                "boundary {boundary_id} has a breached packet but does not name boundary_packet_freshness_breached"
            ),
            Self::MissingPacketWithoutReason { boundary_id } => write!(
                f,
                "boundary {boundary_id} has a missing packet but does not name boundary_packet_missing"
            ),
            Self::StateReasonIncoherent {
                boundary_id,
                state,
                expected_reason,
            } => write!(
                f,
                "boundary {boundary_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { boundary_id, state } => write!(
                f,
                "boundary {boundary_id} state {} names no waiver",
                state.as_str()
            ),
            Self::SubjectMissingValueLine {
                manifest_entry_ref,
                value_line,
            } => write!(
                f,
                "subject {manifest_entry_ref} has no row for value line {}",
                value_line.as_str()
            ),
            Self::DuplicateLineForSubject {
                manifest_entry_ref,
                value_line,
            } => write!(
                f,
                "subject {manifest_entry_ref} has more than one row for value line {}",
                value_line.as_str()
            ),
            Self::InconsistentManifestLabelForSubject { manifest_entry_ref } => write!(
                f,
                "subject {manifest_entry_ref} carries different manifest ceiling labels across its rows"
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
            Self::SummaryMismatch => write!(f, "manifest summary counts disagree with the rows"),
        }
    }
}

impl Error for StableBoundaryManifestViolation {}

/// Loads the embedded stable boundary manifest.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in manifest no longer matches
/// [`StableBoundaryManifest`] — including when a row carries a value line,
/// lifecycle label, boundary state, freshness-SLO state, narrowing reason, or
/// boundary action outside the closed vocabularies.
pub fn current_stable_boundary_manifest() -> Result<StableBoundaryManifest, serde_json::Error> {
    serde_json::from_str(STABLE_BOUNDARY_MANIFEST_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest() -> StableBoundaryManifest {
        current_stable_boundary_manifest().expect("manifest parses")
    }

    #[test]
    fn embedded_manifest_parses_and_validates() {
        let manifest = manifest();
        assert_eq!(
            manifest.schema_version,
            STABLE_BOUNDARY_MANIFEST_SCHEMA_VERSION
        );
        assert_eq!(manifest.record_kind, STABLE_BOUNDARY_MANIFEST_RECORD_KIND);
        assert_eq!(manifest.validate(), Vec::new());
        assert!(!manifest.rows.is_empty());
    }

    #[test]
    fn every_subject_covers_all_four_value_lines() {
        let manifest = manifest();
        let mut by_subject: BTreeMap<String, BTreeSet<ValueLine>> = BTreeMap::new();
        for row in &manifest.rows {
            by_subject
                .entry(row.manifest_entry_ref.clone())
                .or_default()
                .insert(row.value_line);
        }
        assert!(!by_subject.is_empty());
        for (subject, lines) in by_subject {
            for line in ValueLine::ALL {
                assert!(lines.contains(&line), "{subject} missing {}", line.as_str());
            }
        }
    }

    #[test]
    fn manifest_exercises_stable_rows_without_narrowing() {
        let manifest = manifest();
        assert!(
            !manifest.rows_published_stable().is_empty(),
            "manifest must show at least one published-stable boundary"
        );
        assert!(
            manifest.rows_narrowed().is_empty(),
            "clean manifest must not narrow a boundary"
        );
    }

    #[test]
    fn air_gapped_line_publishes_cleanly() {
        let manifest = manifest();
        let air_gapped = manifest
            .computed_line_rollups()
            .into_iter()
            .find(|r| r.value_line == ValueLine::AirGapped)
            .expect("air-gapped rollup exists");
        assert!(air_gapped.published_stable > 0);
        assert_eq!(air_gapped.narrowed_below_cutline, 0);
    }

    #[test]
    fn summary_counts_match_rows() {
        let manifest = manifest();
        assert_eq!(manifest.summary, manifest.computed_summary());
        assert_eq!(
            manifest.summary.boundaries_published_stable
                + manifest.summary.boundaries_narrowed_below_cutline,
            manifest.rows.len()
        );
        assert_eq!(
            manifest.summary.packets_current
                + manifest.summary.packets_due_for_refresh
                + manifest.summary.packets_breached
                + manifest.summary.packets_missing,
            manifest.rows.len()
        );
    }

    #[test]
    fn publication_proceeds_without_blocking_rules() {
        let manifest = manifest();
        assert_eq!(manifest.publication.decision, PromotionDecision::Proceed);
        assert_eq!(
            manifest.publication.decision,
            manifest.computed_publication_decision()
        );
        assert!(manifest.publication.blocking_rule_ids.is_empty());
        assert!(manifest.publication.blocking_boundary_ids.is_empty());
    }

    #[test]
    fn every_narrowing_reason_has_a_rule() {
        let manifest = manifest();
        let covered: BTreeSet<NarrowingReason> = manifest
            .boundary_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in NarrowingReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn no_row_publishes_wider_than_its_manifest_ceiling() {
        let manifest = manifest();
        for row in &manifest.rows {
            assert!(
                row.published_label.rank() <= row.manifest_label.rank(),
                "{} publishes wider than its ceiling",
                row.boundary_id
            );
        }
    }

    #[test]
    fn validate_flags_a_line_published_wider_than_ceiling() {
        let mut manifest = manifest();
        let row = manifest
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a published-stable row exists");
        row.manifest_label = StableClaimLevel::Beta;
        row.published_label = StableClaimLevel::Stable;
        let boundary_id = row.boundary_id.clone();
        manifest.summary = manifest.computed_summary();
        assert!(manifest.validate().iter().any(|v| matches!(
            v,
            StableBoundaryManifestViolation::PublishedWiderThanManifest { boundary_id: id, .. } if *id == boundary_id
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut manifest = manifest();
        let row = manifest
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a published-stable row exists");
        row.boundary_state = BoundaryState::NarrowedUnsupported;
        row.published_label = row.manifest_label;
        row.active_narrowing_reasons = vec![NarrowingReason::LineCapabilityAbsent];
        manifest.summary = manifest.computed_summary();
        manifest.publication.decision = manifest.computed_publication_decision();
        manifest.publication.blocking_rule_ids = manifest.computed_blocking_rule_ids();
        manifest.publication.blocking_boundary_ids = manifest.computed_blocking_boundary_ids();
        assert!(manifest.validate().iter().any(|v| matches!(
            v,
            StableBoundaryManifestViolation::PublishedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut manifest = manifest();
        manifest.publication.decision = PromotionDecision::Hold;
        assert!(manifest.validate().iter().any(|v| matches!(
            v,
            StableBoundaryManifestViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_held_label_without_signoff() {
        let mut manifest = manifest();
        let row = manifest
            .rows
            .iter_mut()
            .find(|row| row.holds_label())
            .expect("a held row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        let boundary_id = row.boundary_id.clone();
        manifest.summary = manifest.computed_summary();
        assert!(manifest
            .validate()
            .contains(&StableBoundaryManifestViolation::HeldWithoutSignoff { boundary_id }));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let manifest = manifest();
        let projection = manifest.support_export_projection();
        assert_eq!(projection.rows.len(), manifest.rows.len());
        assert_eq!(
            projection.publication_decision,
            manifest.publication.decision
        );
        assert_eq!(projection.line_rollups, manifest.computed_line_rollups());
        for (row, projected) in manifest.rows.iter().zip(&projection.rows) {
            assert_eq!(row.boundary_id, projected.boundary_id);
            assert_eq!(row.value_line, projected.value_line);
            assert_eq!(row.publishes_stable(), projected.publishes_stable);
            assert_eq!(row.published_label, projected.published_label);
            assert_eq!(row.boundary_packet.slo_state, projected.slo_state);
        }
    }
}
