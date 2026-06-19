//! Diagnostic-quality snapshots and imported-versus-live delta packets for the
//! M5 finding surfaces.
//!
//! Where [`crate::diagnostics`] froze the per-record canonical diagnostic /
//! source / anchor-remap objects,
//! [`crate::m5_diagnostic_source_descriptors_and_collection_snapshots`] froze
//! *where a finding set came from* and *what scope was analyzed*, and
//! [`crate::quality`] froze the effective profile, typed quality-action
//! proposals, suppression, and baseline records, this module binds those
//! threads into the two objects release-visible debt and support/export truth
//! must depend on:
//!
//! 1. A **diagnostic-quality snapshot** ([`DiagnosticQualitySnapshot`]) captures
//!    the governance state behind a finding set at one point in time: the active
//!    quality-profile ref and fingerprint, the rule-pack / tool versions in
//!    force, the recent collection ids the findings were drawn from, the
//!    suppression / baseline state and release-visible debt count, the imported
//!    scanner session refs, and the last save-participant outcomes. A snapshot
//!    that cannot prove its profile binding, name its tool versions, cite a
//!    recent collection, disclose an imported origin, or join release-visible
//!    debt to suppression / baseline truth is rejected as malformed; a snapshot
//!    whose governance state is stale, unverified, or left a fix rolled back
//!    *auto-downgrades* below its claim with a recorded trigger and a precise
//!    label.
//! 2. An **imported-versus-live delta packet** ([`DiagnosticDeltaPacket`])
//!    compares two finding sides — an imported SARIF / scanner / CI snapshot
//!    against a live local rerun, a runtime finding against a static one, or two
//!    snapshots of the same class — and states a
//!    [`DiagnosticDeltaCompatibilityClass`] verdict with explicit compatibility
//!    notes. Each side keeps its own [`DiagnosticOriginClass`] and
//!    [`DiagnosticFreshnessClass`], so an imported snapshot, a CI finding, a
//!    runtime finding, and a local rerun can never impersonate one another, and a
//!    profile / rule-pack / tool / anchor mismatch blocks an exact-delta claim
//!    rather than silently flattening the two sides.
//!
//! [`DiagnosticQualityParityPacket`] is the headline export that binds the
//! snapshots and delta packets to one shared model, plus a release-debt
//! projection that retains owner / expiry / baseline / suppression truth assembled
//! from the snapshots rather than a manually written summary, the guardrails the
//! lane depends on, and a consumer projection asserting Problems, review,
//! CLI/headless, support export, AI evidence, and release-visible debt all
//! reference the same manifests. [`DiagnosticQualityParityPacket::validate`]
//! refuses a packet that flattens unlike sources, renders imported evidence as
//! live truth, lets a non-compatible delta omit its compatibility note, lets a
//! delta's two sides impersonate one another, drops release-debt owner / expiry /
//! baseline / suppression truth, or fails to auto-downgrade a snapshot whose
//! evidence does not back its claim.
//!
//! Raw source bytes, raw provider payloads, raw scanner reports, provider
//! cursors, credentials, and raw artifact bodies never cross this boundary; the
//! packet carries only typed class tokens, booleans, opaque ids, counts, and
//! redaction-aware reviewable labels.
//!
//! The composed boundary schema is
//! [`schemas/quality/diagnostic-quality-parity.schema.json`](../../../../schemas/quality/diagnostic-quality-parity.schema.json),
//! composed from
//! [`schemas/quality/diagnostic-quality-snapshot.schema.json`](../../../../schemas/quality/diagnostic-quality-snapshot.schema.json)
//! and
//! [`schemas/quality/diagnostic-delta-packet.schema.json`](../../../../schemas/quality/diagnostic-delta-packet.schema.json).
//! The contract doc is
//! [`docs/m5/diagnostic-quality-snapshots-and-deltas.md`](../../../../docs/m5/diagnostic-quality-snapshots-and-deltas.md).
//! The protected fixture directory is
//! [`fixtures/quality/m5/imported-vs-live-deltas/`](../../../../fixtures/quality/m5/imported-vs-live-deltas/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::diagnostics::{DiagnosticFreshnessClass, DiagnosticOriginClass, DiagnosticSourceKind};
use crate::m5_diagnostic_source_descriptors_and_collection_snapshots::DiagnosticCollectionScope;

/// Stable record-kind tag carried by [`DiagnosticQualityParityPacket`].
pub const M5_DIAGNOSTIC_QUALITY_PARITY_RECORD_KIND: &str = "m5_diagnostic_quality_parity";

/// Stable record-kind tag for one [`DiagnosticQualitySnapshot`].
pub const DIAGNOSTIC_QUALITY_SNAPSHOT_RECORD_KIND: &str = "diagnostic_quality_snapshot";

/// Stable record-kind tag for one [`DiagnosticDeltaPacket`].
pub const DIAGNOSTIC_DELTA_PACKET_RECORD_KIND: &str = "diagnostic_delta_packet";

/// Schema version for the quality-parity packet and its records.
pub const M5_DIAGNOSTIC_QUALITY_PARITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the composed packet schema.
pub const M5_DIAGNOSTIC_QUALITY_PARITY_SCHEMA_REF: &str =
    "schemas/quality/diagnostic-quality-parity.schema.json";

/// Repo-relative path of the quality-snapshot component schema.
pub const DIAGNOSTIC_QUALITY_SNAPSHOT_SCHEMA_REF: &str =
    "schemas/quality/diagnostic-quality-snapshot.schema.json";

/// Repo-relative path of the delta-packet component schema.
pub const DIAGNOSTIC_DELTA_PACKET_SCHEMA_REF: &str =
    "schemas/quality/diagnostic-delta-packet.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DIAGNOSTIC_QUALITY_PARITY_DOC_REF: &str =
    "docs/m5/diagnostic-quality-snapshots-and-deltas.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DIAGNOSTIC_QUALITY_PARITY_ARTIFACT_REF: &str =
    "artifacts/m5/diagnostics/diagnostic-quality-parity-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_DIAGNOSTIC_QUALITY_PARITY_SUMMARY_REF: &str =
    "artifacts/m5/diagnostics/diagnostic-quality-parity-proof/support_export.md";

/// Qualification ladder for a [`DiagnosticQualitySnapshotEntry`].
///
/// A snapshot graduates from held to stable only as the evidence behind its
/// profile binding, tool versions, recent collection, imported disclosure, and
/// save-participant outcomes proves out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticQualitySnapshotQualificationClass {
    /// Held below any public claim.
    Held,
    /// Preview-grade claim.
    Preview,
    /// Beta-grade claim.
    Beta,
    /// Stable-grade claim.
    Stable,
}

impl DiagnosticQualitySnapshotQualificationClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Held => "held",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
        }
    }

    /// Whether this class carries a public claim above held.
    pub const fn is_claimed(self) -> bool {
        !matches!(self, Self::Held)
    }

    /// Monotonic rank used to compare claimed and effective qualifications.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Held => 0,
            Self::Preview => 1,
            Self::Beta => 2,
            Self::Stable => 3,
        }
    }
}

/// Trigger that fired an auto-downgrade on a [`DiagnosticQualitySnapshotEntry`].
///
/// A downgrade trigger names a *disclosed-but-weak* governance signal — stale or
/// unverified freshness, or a fix left rolled back — that caps the claim. A
/// structurally malformed snapshot (missing profile binding, tool versions,
/// recent collection, imported disclosure, or suppression / baseline truth) is a
/// hard validation failure, not a downgrade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticQualitySnapshotDowngradeTrigger {
    /// The governance state is stale or superseded by a newer epoch.
    StaleGovernanceState,
    /// The governance state's freshness could not be verified.
    UnverifiedGovernanceState,
    /// A last save-participant fix failed and was rolled back.
    UnresolvedSaveParticipant,
}

impl DiagnosticQualitySnapshotDowngradeTrigger {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaleGovernanceState => "stale_governance_state",
            Self::UnverifiedGovernanceState => "unverified_governance_state",
            Self::UnresolvedSaveParticipant => "unresolved_save_participant",
        }
    }
}

/// Outcome class for a save participant's last run captured by a snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveParticipantOutcomeClass {
    /// Applied cleanly with no follow-up required.
    AppliedClean,
    /// Applied, but follow-up findings or validations remain.
    AppliedWithFollowups,
    /// Previewed and intentionally not applied.
    PreviewedNotApplied,
    /// Blocked because a preview is required before apply.
    BlockedRequiresPreview,
    /// Failed and was rolled back to the checkpoint.
    FailedRolledBack,
    /// Skipped because the participant did not run for this scope.
    Skipped,
}

impl SaveParticipantOutcomeClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AppliedClean => "applied_clean",
            Self::AppliedWithFollowups => "applied_with_followups",
            Self::PreviewedNotApplied => "previewed_not_applied",
            Self::BlockedRequiresPreview => "blocked_requires_preview",
            Self::FailedRolledBack => "failed_rolled_back",
            Self::Skipped => "skipped",
        }
    }
}

/// Basis on which a [`DiagnosticDeltaPacket`] compares two finding sides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticDeltaComparisonBasisClass {
    /// An imported SARIF / scanner snapshot against a live local rerun.
    ImportedVsLiveRerun,
    /// A CI-produced finding set against a local rerun.
    CiVsLocalRerun,
    /// A runtime / test finding against a static-analysis finding.
    RuntimeVsStaticAnalysis,
    /// Two imported snapshots of the same provider family.
    ImportedSnapshotVsImportedSnapshot,
    /// Two live snapshots across epochs.
    LiveSnapshotVsLiveSnapshot,
}

impl DiagnosticDeltaComparisonBasisClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImportedVsLiveRerun => "imported_vs_live_rerun",
            Self::CiVsLocalRerun => "ci_vs_local_rerun",
            Self::RuntimeVsStaticAnalysis => "runtime_vs_static_analysis",
            Self::ImportedSnapshotVsImportedSnapshot => "imported_snapshot_vs_imported_snapshot",
            Self::LiveSnapshotVsLiveSnapshot => "live_snapshot_vs_live_snapshot",
        }
    }

    /// Whether this basis crosses the imported/live boundary, where the two
    /// sides must carry distinct origin classes so neither can impersonate the
    /// other.
    pub const fn crosses_imported_live_boundary(self) -> bool {
        matches!(self, Self::ImportedVsLiveRerun | Self::CiVsLocalRerun)
    }
}

/// Compatibility verdict between the two sides of a [`DiagnosticDeltaPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticDeltaCompatibilityClass {
    /// The two sides are exactly comparable.
    CompatibleExact,
    /// Comparable once the imported side is locally confirmed.
    CompatibleWithLocalConfirmation,
    /// Blocked because the active profile or tool versions differ.
    BlockedProfileOrToolMismatch,
    /// Blocked because the rule-pack versions differ.
    BlockedRulePackMismatch,
    /// Blocked because anchor mapping between the sides is uncertain.
    BlockedAnchorMappingUncertain,
    /// Not comparable: the sides describe distinct source kinds.
    NotComparableDistinctSource,
    /// Not comparable: an unknown scanner / source family needs review.
    NotComparableUnknownRequiresReview,
}

impl DiagnosticDeltaCompatibilityClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompatibleExact => "compatible_exact",
            Self::CompatibleWithLocalConfirmation => "compatible_with_local_confirmation",
            Self::BlockedProfileOrToolMismatch => "blocked_profile_or_tool_mismatch",
            Self::BlockedRulePackMismatch => "blocked_rule_pack_mismatch",
            Self::BlockedAnchorMappingUncertain => "blocked_anchor_mapping_uncertain",
            Self::NotComparableDistinctSource => "not_comparable_distinct_source",
            Self::NotComparableUnknownRequiresReview => "not_comparable_unknown_requires_review",
        }
    }

    /// Whether the verdict permits an exact-delta claim with no caveat.
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::CompatibleExact)
    }

    /// Whether the verdict requires at least one compatibility note.
    ///
    /// Anything short of an exact match must carry an explicit caveat so a
    /// blocked or conditional comparison cannot read as a clean delta.
    pub const fn requires_note(self) -> bool {
        !self.is_exact()
    }
}

/// Why two sides of a delta are not exactly compatible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticDeltaCompatibilityNoteClass {
    /// The active quality profiles differ across the two sides.
    ProfileMismatch,
    /// The rule-pack versions differ.
    RulePackVersionSkew,
    /// The tool / analyzer versions differ.
    ToolVersionSkew,
    /// The freshness states differ enough to caveat the comparison.
    FreshnessSkew,
    /// Anchor mapping between the sides is uncertain.
    AnchorMappingUncertain,
    /// The two sides describe distinct source kinds.
    DistinctSourceKind,
    /// The analyzed scope differs across the two sides.
    ScopeMismatch,
    /// The baseline families differ across the two sides.
    BaselineFamilyMismatch,
}

impl DiagnosticDeltaCompatibilityNoteClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProfileMismatch => "profile_mismatch",
            Self::RulePackVersionSkew => "rule_pack_version_skew",
            Self::ToolVersionSkew => "tool_version_skew",
            Self::FreshnessSkew => "freshness_skew",
            Self::AnchorMappingUncertain => "anchor_mapping_uncertain",
            Self::DistinctSourceKind => "distinct_source_kind",
            Self::ScopeMismatch => "scope_mismatch",
            Self::BaselineFamilyMismatch => "baseline_family_mismatch",
        }
    }
}

/// State of a single finding across an imported-versus-live comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticFindingDeltaState {
    /// Present on the compare side and absent on the base side.
    Added,
    /// Present on the base side and absent on the compare side.
    Resolved,
    /// Present on both sides.
    Persisting,
    /// Present but suppressed by policy.
    Suppressed,
    /// Present but waived against a baseline.
    Waived,
    /// Could not be mapped between the two sides.
    Unmapped,
}

impl DiagnosticFindingDeltaState {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Resolved => "resolved",
            Self::Persisting => "persisting",
            Self::Suppressed => "suppressed",
            Self::Waived => "waived",
            Self::Unmapped => "unmapped",
        }
    }
}

/// One rule-pack / tool version row pinned by a [`DiagnosticQualitySnapshot`].
///
/// Names the producing source kind, the tool and rule-pack identities, and their
/// versions, so a snapshot records exactly which analyzers and rule packs were in
/// force when its finding set was produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualityToolVersionRow {
    /// Producing source family.
    pub source_kind: DiagnosticSourceKind,
    /// Opaque ref to the tool / analyzer identity.
    pub tool_ref: String,
    /// Version string of the tool / analyzer.
    pub tool_version: String,
    /// Opaque ref to the rule pack identity.
    pub rule_pack_ref: String,
    /// Version string of the rule pack.
    pub rule_pack_version: String,
    /// Optional opaque ref to the adapter that normalized the findings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adapter_ref: Option<String>,
    /// Export-safe one-line summary.
    pub summary: String,
}

impl QualityToolVersionRow {
    /// Whether the row pins both a tool and a rule-pack version.
    pub fn is_pinned(&self) -> bool {
        !self.tool_ref.trim().is_empty()
            && !self.tool_version.trim().is_empty()
            && !self.rule_pack_ref.trim().is_empty()
            && !self.rule_pack_version.trim().is_empty()
    }
}

/// Last outcome recorded for a save participant within a snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveParticipantOutcomeRow {
    /// Opaque ref to the save participant.
    pub participant_ref: String,
    /// Opaque ref to the quality-action proposal the participant ran.
    pub proposal_ref: String,
    /// Stable action token (e.g. `format`, `organize_imports`, `fix_all`).
    pub action_token: String,
    /// Outcome class for the participant's last run.
    pub outcome_class: SaveParticipantOutcomeClass,
    /// Whether a preview was required before apply.
    pub preview_first_required: bool,
    /// Whether the apply was blocked.
    pub apply_blocked: bool,
    /// When the outcome was observed.
    pub observed_at: String,
    /// Export-safe one-line summary.
    pub summary: String,
}

/// Captures the governance state behind a finding set at one point in time.
///
/// See the [module docs](self) for the honesty guarantees this object carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticQualitySnapshot {
    /// Stable record-kind tag, always [`DIAGNOSTIC_QUALITY_SNAPSHOT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version, always [`M5_DIAGNOSTIC_QUALITY_PARITY_SCHEMA_VERSION`].
    pub diagnostic_quality_snapshot_schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Reviewer-facing label.
    pub snapshot_label: String,
    /// Workspace / workset / target scope the snapshot describes.
    pub scope: DiagnosticCollectionScope,
    /// Imported-versus-live origin of the snapshot's governance state.
    pub origin_class: DiagnosticOriginClass,
    /// Freshness of the snapshot's governance state.
    pub freshness_class: DiagnosticFreshnessClass,
    /// When the snapshot was captured.
    pub captured_at: String,
    /// Active quality-profile ref the findings were produced under.
    pub active_profile_ref: String,
    /// Fingerprint of the resolved profile.
    pub profile_fingerprint: String,
    /// Rule-pack / tool versions in force.
    pub tool_versions: Vec<QualityToolVersionRow>,
    /// Recent collection-snapshot ids the findings were drawn from.
    pub recent_collection_refs: Vec<String>,
    /// Suppression-record refs in force.
    pub suppression_refs: Vec<String>,
    /// Baseline-record refs in force.
    pub baseline_refs: Vec<String>,
    /// Count of release-visible debt items behind this snapshot.
    pub release_visible_debt_count: usize,
    /// Imported scanner session refs, when the origin is imported.
    pub imported_scanner_session_refs: Vec<String>,
    /// Last save-participant outcomes for the scope.
    pub save_participant_outcomes: Vec<SaveParticipantOutcomeRow>,
    /// Source-descriptor refs contributing to the snapshot.
    pub source_descriptor_refs: Vec<String>,
    /// Whether imported evidence is held read-only and never shown as live.
    pub imported_not_shown_as_live: bool,
    /// Export-safe one-line summary.
    pub export_safe_summary: String,
}

/// Input to [`DiagnosticQualitySnapshot::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticQualitySnapshotInput {
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Reviewer-facing label.
    pub snapshot_label: String,
    /// Workspace / workset / target scope the snapshot describes.
    pub scope: DiagnosticCollectionScope,
    /// Imported-versus-live origin of the snapshot's governance state.
    pub origin_class: DiagnosticOriginClass,
    /// Freshness of the snapshot's governance state.
    pub freshness_class: DiagnosticFreshnessClass,
    /// When the snapshot was captured.
    pub captured_at: String,
    /// Active quality-profile ref the findings were produced under.
    pub active_profile_ref: String,
    /// Fingerprint of the resolved profile.
    pub profile_fingerprint: String,
    /// Rule-pack / tool versions in force.
    pub tool_versions: Vec<QualityToolVersionRow>,
    /// Recent collection-snapshot ids the findings were drawn from.
    pub recent_collection_refs: Vec<String>,
    /// Suppression-record refs in force.
    pub suppression_refs: Vec<String>,
    /// Baseline-record refs in force.
    pub baseline_refs: Vec<String>,
    /// Count of release-visible debt items behind this snapshot.
    pub release_visible_debt_count: usize,
    /// Imported scanner session refs, when the origin is imported.
    pub imported_scanner_session_refs: Vec<String>,
    /// Last save-participant outcomes for the scope.
    pub save_participant_outcomes: Vec<SaveParticipantOutcomeRow>,
    /// Source-descriptor refs contributing to the snapshot.
    pub source_descriptor_refs: Vec<String>,
    /// Whether imported evidence is held read-only and never shown as live.
    pub imported_not_shown_as_live: bool,
    /// Export-safe one-line summary.
    pub export_safe_summary: String,
}

impl DiagnosticQualitySnapshot {
    /// Builds a snapshot from [`DiagnosticQualitySnapshotInput`], stamping the
    /// stable record kind and schema version.
    pub fn new(input: DiagnosticQualitySnapshotInput) -> Self {
        Self {
            record_kind: DIAGNOSTIC_QUALITY_SNAPSHOT_RECORD_KIND.to_owned(),
            diagnostic_quality_snapshot_schema_version: M5_DIAGNOSTIC_QUALITY_PARITY_SCHEMA_VERSION,
            snapshot_id: input.snapshot_id,
            snapshot_label: input.snapshot_label,
            scope: input.scope,
            origin_class: input.origin_class,
            freshness_class: input.freshness_class,
            captured_at: input.captured_at,
            active_profile_ref: input.active_profile_ref,
            profile_fingerprint: input.profile_fingerprint,
            tool_versions: input.tool_versions,
            recent_collection_refs: input.recent_collection_refs,
            suppression_refs: input.suppression_refs,
            baseline_refs: input.baseline_refs,
            release_visible_debt_count: input.release_visible_debt_count,
            imported_scanner_session_refs: input.imported_scanner_session_refs,
            save_participant_outcomes: input.save_participant_outcomes,
            source_descriptor_refs: input.source_descriptor_refs,
            imported_not_shown_as_live: input.imported_not_shown_as_live,
            export_safe_summary: input.export_safe_summary,
        }
    }

    /// Whether the snapshot binds an active profile ref and fingerprint.
    pub fn has_profile_binding(&self) -> bool {
        !self.active_profile_ref.trim().is_empty() && !self.profile_fingerprint.trim().is_empty()
    }

    /// Whether the snapshot pins at least one well-formed tool / rule-pack version.
    pub fn has_tool_versions(&self) -> bool {
        !self.tool_versions.is_empty()
            && self
                .tool_versions
                .iter()
                .all(QualityToolVersionRow::is_pinned)
    }

    /// Whether the snapshot cites at least one recent collection ref.
    pub fn has_recent_collection(&self) -> bool {
        !self.recent_collection_refs.is_empty()
            && self
                .recent_collection_refs
                .iter()
                .all(|r| !r.trim().is_empty())
    }

    /// Whether an imported / replayed origin is disclosed as non-live evidence.
    ///
    /// An imported origin must be held read-only, and an imported-snapshot origin
    /// must cite at least one imported scanner session.
    pub fn imported_disclosed(&self) -> bool {
        if !self.origin_class.is_imported_or_replayed() {
            return true;
        }
        if !self.imported_not_shown_as_live {
            return false;
        }
        if matches!(self.origin_class, DiagnosticOriginClass::ImportedSnapshot) {
            return !self.imported_scanner_session_refs.is_empty()
                && self
                    .imported_scanner_session_refs
                    .iter()
                    .all(|r| !r.trim().is_empty());
        }
        true
    }

    /// Whether the snapshot records its last save-participant outcomes.
    pub fn save_participants_recorded(&self) -> bool {
        !self.save_participant_outcomes.is_empty()
            && self.save_participant_outcomes.iter().all(|row| {
                !row.participant_ref.trim().is_empty() && !row.proposal_ref.trim().is_empty()
            })
    }

    /// Whether release-visible debt is backed by suppression / baseline truth.
    ///
    /// A non-zero release-visible debt count must be joined to at least one
    /// suppression or baseline record rather than asserted as a bare number.
    pub fn suppression_baseline_truth_present(&self) -> bool {
        if self.release_visible_debt_count == 0 {
            return true;
        }
        !self.suppression_refs.is_empty() || !self.baseline_refs.is_empty()
    }

    /// Whether every hard governance-truth obligation holds for this snapshot.
    ///
    /// A snapshot that fails any of these is malformed — a hard validation
    /// failure — rather than merely downgraded.
    pub fn governance_truth_present(&self) -> bool {
        self.has_profile_binding()
            && self.has_tool_versions()
            && self.has_recent_collection()
            && self.imported_disclosed()
            && self.save_participants_recorded()
            && self.suppression_baseline_truth_present()
    }

    /// Whether the snapshot's freshness supports a public claim.
    ///
    /// An imported snapshot's `imported_snapshot` freshness is its legitimate
    /// best and counts as provable; a stale, superseded, unverified, or degraded
    /// cached state does not.
    pub const fn freshness_provable(&self) -> bool {
        matches!(
            self.freshness_class,
            DiagnosticFreshnessClass::Current
                | DiagnosticFreshnessClass::Recent
                | DiagnosticFreshnessClass::WarmCached
                | DiagnosticFreshnessClass::ImportedSnapshot
        )
    }

    /// Whether no last save-participant fix failed and was rolled back.
    pub fn save_outcomes_resolved(&self) -> bool {
        self.save_participant_outcomes.iter().all(|row| {
            !matches!(
                row.outcome_class,
                SaveParticipantOutcomeClass::FailedRolledBack
            )
        })
    }

    /// Whether the snapshot's claim is supported by settled, provable evidence.
    ///
    /// Distinct from [`Self::governance_truth_present`]: this gates the *claim
    /// level* on disclosed-but-weak signals rather than on structural validity.
    pub fn claim_supported(&self) -> bool {
        self.freshness_provable() && self.save_outcomes_resolved()
    }

    /// The trigger that should fire when the claim is not supported.
    fn weak_truth_trigger(&self) -> Option<DiagnosticQualitySnapshotDowngradeTrigger> {
        if matches!(
            self.freshness_class,
            DiagnosticFreshnessClass::Stale | DiagnosticFreshnessClass::Superseded
        ) {
            return Some(DiagnosticQualitySnapshotDowngradeTrigger::StaleGovernanceState);
        }
        if matches!(
            self.freshness_class,
            DiagnosticFreshnessClass::Unverified | DiagnosticFreshnessClass::DegradedCached
        ) {
            return Some(DiagnosticQualitySnapshotDowngradeTrigger::UnverifiedGovernanceState);
        }
        if !self.save_outcomes_resolved() {
            return Some(DiagnosticQualitySnapshotDowngradeTrigger::UnresolvedSaveParticipant);
        }
        None
    }

    /// Whether the snapshot is structurally complete enough to inspect.
    pub fn is_structurally_complete(&self) -> bool {
        !self.snapshot_id.trim().is_empty()
            && !self.snapshot_label.trim().is_empty()
            && !self.captured_at.trim().is_empty()
            && !self.export_safe_summary.trim().is_empty()
            && !self.source_descriptor_refs.is_empty()
    }
}

/// A snapshot bound to a qualification with auto-downgrade on weak truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticQualitySnapshotEntry {
    /// Stable entry id.
    pub entry_id: String,
    /// The captured snapshot.
    pub snapshot: DiagnosticQualitySnapshot,
    /// Claimed qualification level.
    pub claimed_qualification: DiagnosticQualitySnapshotQualificationClass,
    /// Effective qualification after auto-downgrade.
    pub effective_qualification: DiagnosticQualitySnapshotQualificationClass,
    /// Trigger that fired the auto-downgrade, when downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<DiagnosticQualitySnapshotDowngradeTrigger>,
    /// Precise degraded label, when downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,
    /// Evidence refs backing the entry.
    pub evidence_refs: Vec<String>,
}

impl DiagnosticQualitySnapshotEntry {
    /// Whether the entry carries a public claim above held.
    pub fn is_claimed(&self) -> bool {
        self.claimed_qualification.is_claimed()
    }

    /// Whether the entry must auto-downgrade because its evidence is weak.
    pub fn needs_downgrade(&self) -> bool {
        self.is_claimed() && !self.snapshot.claim_supported()
    }

    /// Whether a downgraded entry is internally consistent.
    pub fn downgrade_consistent(&self) -> bool {
        if !self.needs_downgrade() {
            return true;
        }
        self.effective_qualification.rank() < self.claimed_qualification.rank()
            && self.downgrade_trigger.is_some()
            && self
                .degraded_label
                .as_ref()
                .is_some_and(|label| !label_is_generic(label))
    }

    /// Whether the entry is structurally complete.
    pub fn is_structurally_complete(&self) -> bool {
        !self.entry_id.trim().is_empty()
            && self.snapshot.is_structurally_complete()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// The trigger that should fire for this entry's weak truth, if any.
    pub fn expected_downgrade_trigger(&self) -> Option<DiagnosticQualitySnapshotDowngradeTrigger> {
        self.snapshot.weak_truth_trigger()
    }
}

/// One side of an imported-versus-live comparison.
///
/// Carries its own origin and freshness so the two sides of a delta can never
/// impersonate one another.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticDeltaSide {
    /// Reviewer-facing label.
    pub side_label: String,
    /// Imported-versus-live origin of this side.
    pub origin_class: DiagnosticOriginClass,
    /// Freshness of this side.
    pub freshness_class: DiagnosticFreshnessClass,
    /// Producing source family.
    pub source_kind: DiagnosticSourceKind,
    /// Quality-snapshot ref this side is drawn from.
    pub snapshot_ref: String,
    /// Collection-snapshot ref this side is drawn from.
    pub collection_ref: String,
    /// Active quality-profile ref of this side.
    pub active_profile_ref: String,
    /// Tool / rule-pack version refs of this side.
    pub tool_version_refs: Vec<String>,
    /// Export-safe one-line summary.
    pub summary: String,
}

impl DiagnosticDeltaSide {
    /// Whether this side carries imported or replayed evidence.
    pub const fn is_imported(&self) -> bool {
        self.origin_class.is_imported_or_replayed()
    }
}

/// A compatibility caveat between the two sides of a [`DiagnosticDeltaPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticDeltaCompatibilityNote {
    /// Why the two sides are not exactly compatible.
    pub note_class: DiagnosticDeltaCompatibilityNoteClass,
    /// Export-safe one-line summary.
    pub summary: String,
}

/// Counts per [`DiagnosticFindingDeltaState`] for a delta packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticDeltaCounts {
    /// Findings present only on the compare side.
    pub added: usize,
    /// Findings present only on the base side.
    pub resolved: usize,
    /// Findings present on both sides.
    pub persisting: usize,
    /// Findings suppressed or waived.
    pub suppressed_or_waived: usize,
    /// Findings that could not be mapped between the sides.
    pub unmapped: usize,
}

/// One finding's state across the comparison.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticFindingDelta {
    /// Opaque ref to the diagnostic record this delta describes.
    pub finding_ref: String,
    /// State of the finding across the two sides.
    pub delta_state: DiagnosticFindingDeltaState,
    /// Whether the finding is present on the base side.
    pub base_present: bool,
    /// Whether the finding is present on the compare side.
    pub compare_present: bool,
    /// Whether the finding could be compared across the two sides.
    pub comparable: bool,
    /// Export-safe one-line summary.
    pub summary: String,
}

/// Compares two finding sides and states a compatibility verdict with notes.
///
/// See the [module docs](self) for the impersonation guarantees this object
/// carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticDeltaPacket {
    /// Stable record-kind tag, always [`DIAGNOSTIC_DELTA_PACKET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version, always [`M5_DIAGNOSTIC_QUALITY_PARITY_SCHEMA_VERSION`].
    pub diagnostic_delta_packet_schema_version: u32,
    /// Stable delta id.
    pub delta_id: String,
    /// Reviewer-facing label.
    pub delta_label: String,
    /// Basis on which the two sides are compared.
    pub comparison_basis_class: DiagnosticDeltaComparisonBasisClass,
    /// The base (reference) side.
    pub base_side: DiagnosticDeltaSide,
    /// The compare (candidate) side.
    pub compare_side: DiagnosticDeltaSide,
    /// Compatibility verdict between the two sides.
    pub compatibility_class: DiagnosticDeltaCompatibilityClass,
    /// Explicit compatibility notes.
    pub compatibility_notes: Vec<DiagnosticDeltaCompatibilityNote>,
    /// Counts per delta state.
    pub delta_counts: DiagnosticDeltaCounts,
    /// Per-finding deltas.
    pub finding_deltas: Vec<DiagnosticFindingDelta>,
    /// Whether the two sides are guarded against impersonating one another.
    pub impersonation_guarded: bool,
    /// Export-safe one-line summary.
    pub export_safe_summary: String,
}

/// Input to [`DiagnosticDeltaPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticDeltaPacketInput {
    /// Stable delta id.
    pub delta_id: String,
    /// Reviewer-facing label.
    pub delta_label: String,
    /// Basis on which the two sides are compared.
    pub comparison_basis_class: DiagnosticDeltaComparisonBasisClass,
    /// The base (reference) side.
    pub base_side: DiagnosticDeltaSide,
    /// The compare (candidate) side.
    pub compare_side: DiagnosticDeltaSide,
    /// Compatibility verdict between the two sides.
    pub compatibility_class: DiagnosticDeltaCompatibilityClass,
    /// Explicit compatibility notes.
    pub compatibility_notes: Vec<DiagnosticDeltaCompatibilityNote>,
    /// Counts per delta state.
    pub delta_counts: DiagnosticDeltaCounts,
    /// Per-finding deltas.
    pub finding_deltas: Vec<DiagnosticFindingDelta>,
    /// Whether the two sides are guarded against impersonating one another.
    pub impersonation_guarded: bool,
    /// Export-safe one-line summary.
    pub export_safe_summary: String,
}

impl DiagnosticDeltaPacket {
    /// Builds a delta packet from [`DiagnosticDeltaPacketInput`], stamping the
    /// stable record kind and schema version.
    pub fn new(input: DiagnosticDeltaPacketInput) -> Self {
        Self {
            record_kind: DIAGNOSTIC_DELTA_PACKET_RECORD_KIND.to_owned(),
            diagnostic_delta_packet_schema_version: M5_DIAGNOSTIC_QUALITY_PARITY_SCHEMA_VERSION,
            delta_id: input.delta_id,
            delta_label: input.delta_label,
            comparison_basis_class: input.comparison_basis_class,
            base_side: input.base_side,
            compare_side: input.compare_side,
            compatibility_class: input.compatibility_class,
            compatibility_notes: input.compatibility_notes,
            delta_counts: input.delta_counts,
            finding_deltas: input.finding_deltas,
            impersonation_guarded: input.impersonation_guarded,
            export_safe_summary: input.export_safe_summary,
        }
    }

    /// Whether the two sides are distinct rather than a thing compared to itself.
    ///
    /// Two sides must differ in their origin, freshness, or snapshot ref so an
    /// imported snapshot can never be presented as both halves of a delta.
    pub fn sides_distinct(&self) -> bool {
        self.base_side.origin_class != self.compare_side.origin_class
            || self.base_side.freshness_class != self.compare_side.freshness_class
            || self.base_side.snapshot_ref != self.compare_side.snapshot_ref
    }

    /// Whether the compatibility notes are sufficient for the verdict.
    pub fn compatibility_notes_sufficient(&self) -> bool {
        if self.compatibility_class.requires_note() {
            !self.compatibility_notes.is_empty()
                && self
                    .compatibility_notes
                    .iter()
                    .all(|note| !note.summary.trim().is_empty())
        } else {
            true
        }
    }

    /// Whether the recorded counts match the per-finding deltas.
    pub fn counts_consistent(&self) -> bool {
        let mut tally = DiagnosticDeltaCounts {
            added: 0,
            resolved: 0,
            persisting: 0,
            suppressed_or_waived: 0,
            unmapped: 0,
        };
        for delta in &self.finding_deltas {
            match delta.delta_state {
                DiagnosticFindingDeltaState::Added => tally.added += 1,
                DiagnosticFindingDeltaState::Resolved => tally.resolved += 1,
                DiagnosticFindingDeltaState::Persisting => tally.persisting += 1,
                DiagnosticFindingDeltaState::Suppressed | DiagnosticFindingDeltaState::Waived => {
                    tally.suppressed_or_waived += 1
                }
                DiagnosticFindingDeltaState::Unmapped => tally.unmapped += 1,
            }
        }
        tally == self.delta_counts
    }

    /// Whether the two sides are guarded against impersonating one another.
    ///
    /// The guard flag must be set, the sides must be distinct, and a comparison
    /// that crosses the imported/live boundary must carry distinct origin
    /// classes on the two sides.
    pub fn impersonation_ok(&self) -> bool {
        if !self.impersonation_guarded || !self.sides_distinct() {
            return false;
        }
        if self.comparison_basis_class.crosses_imported_live_boundary() {
            return self.base_side.origin_class != self.compare_side.origin_class;
        }
        true
    }

    /// Whether the verdict blocks or refuses an exact-delta claim.
    pub fn is_blocked_or_incomparable(&self) -> bool {
        self.compatibility_class.requires_note()
            && !matches!(
                self.compatibility_class,
                DiagnosticDeltaCompatibilityClass::CompatibleWithLocalConfirmation
            )
    }

    /// Whether the delta packet is structurally complete.
    pub fn is_structurally_complete(&self) -> bool {
        !self.delta_id.trim().is_empty()
            && !self.delta_label.trim().is_empty()
            && !self.export_safe_summary.trim().is_empty()
            && !self.base_side.snapshot_ref.trim().is_empty()
            && !self.compare_side.snapshot_ref.trim().is_empty()
    }
}

/// Guardrails the quality-parity lane depends on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticQualityParityGuardrails {
    /// Unlike sources are never flattened into a synthetic finding.
    pub unlike_sources_never_flattened: bool,
    /// Anchors are never silently repaired across a comparison.
    pub anchors_never_silently_repaired: bool,
    /// Imported-versus-live class stays explicit on every side.
    pub imported_live_class_explicit: bool,
    /// Freshness and remap states stay explicit rather than hidden.
    pub freshness_and_remap_states_explicit: bool,
    /// Target / environment and policy state survive clustering.
    pub policy_state_preserved: bool,
    /// Every mutating fix route is a typed quality-action proposal.
    pub every_fix_route_is_typed_proposal: bool,
    /// Diagnostic ids and collection completeness stay exportable.
    pub ids_and_completeness_exportable: bool,
}

impl DiagnosticQualityParityGuardrails {
    /// Whether every guardrail holds.
    pub fn all_hold(&self) -> bool {
        self.unlike_sources_never_flattened
            && self.anchors_never_silently_repaired
            && self.imported_live_class_explicit
            && self.freshness_and_remap_states_explicit
            && self.policy_state_preserved
            && self.every_fix_route_is_typed_proposal
            && self.ids_and_completeness_exportable
    }
}

/// Asserts every consumer surface references the shared snapshot / delta model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticQualityParityConsumerProjection {
    /// Problems references the shared model.
    pub problems_references_shared_model: bool,
    /// Review references the shared model.
    pub review_references_shared_model: bool,
    /// CLI / headless references the shared model.
    pub cli_headless_references_shared_model: bool,
    /// Support export references the shared model.
    pub support_export_references_shared_model: bool,
    /// AI evidence references the shared model.
    pub ai_evidence_references_shared_model: bool,
    /// Release-visible debt references the shared model.
    pub release_debt_references_shared_model: bool,
}

impl DiagnosticQualityParityConsumerProjection {
    /// Whether every consumer references the shared model.
    pub fn all_hold(&self) -> bool {
        self.problems_references_shared_model
            && self.review_references_shared_model
            && self.cli_headless_references_shared_model
            && self.support_export_references_shared_model
            && self.ai_evidence_references_shared_model
            && self.release_debt_references_shared_model
    }
}

/// Release-visible debt assembled from the snapshots, retaining owner / expiry /
/// baseline / suppression truth instead of a manually written summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticQualityReleaseDebtProjection {
    /// Whether the debt count was assembled from the packet's snapshots.
    pub assembled_from_snapshots: bool,
    /// Whether owner truth is preserved for each debt item.
    pub owner_truth_preserved: bool,
    /// Whether expiry truth is preserved for each debt item.
    pub expiry_truth_preserved: bool,
    /// Whether the baseline join is preserved.
    pub baseline_join_preserved: bool,
    /// Whether the suppression join is preserved.
    pub suppression_join_preserved: bool,
    /// Count of release-visible debt items.
    pub release_visible_debt_count: usize,
    /// Refs to the snapshots / suppressions / baselines feeding the count.
    pub debt_source_refs: Vec<String>,
    /// Export-safe one-line summary.
    pub summary: String,
}

impl DiagnosticQualityReleaseDebtProjection {
    /// Whether owner / expiry / baseline / suppression truth is preserved and the
    /// debt is assembled from the snapshots rather than a hand-written summary.
    pub fn truth_preserved(&self) -> bool {
        self.assembled_from_snapshots
            && self.owner_truth_preserved
            && self.expiry_truth_preserved
            && self.baseline_join_preserved
            && self.suppression_join_preserved
            && !self.debt_source_refs.is_empty()
            && self.debt_source_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Reasons a [`DiagnosticQualityParityPacket`] fails validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticQualityParityViolation {
    /// Record kind is not the expected tag.
    WrongRecordKind,
    /// Schema version is not the expected version.
    WrongSchemaVersion,
    /// Packet identity fields are missing.
    MissingIdentity,
    /// Required schema / doc / artifact source contracts are missing.
    MissingSourceContracts,
    /// Both an imported and a live snapshot origin are not represented.
    RequiredOriginCoverageMissing,
    /// A snapshot entry is structurally incomplete.
    SnapshotEntryIncomplete,
    /// A snapshot does not bind an active profile ref and fingerprint.
    SnapshotMissingProfileBinding,
    /// A snapshot pins no well-formed rule-pack / tool versions.
    SnapshotMissingToolVersions,
    /// A snapshot cites no recent collection ref.
    SnapshotMissingRecentCollection,
    /// An imported snapshot is rendered as live local truth.
    SnapshotImportedShownAsLive,
    /// A snapshot claims release-visible debt without suppression / baseline truth.
    SnapshotSuppressionBaselineTruthMissing,
    /// No downgraded snapshot demonstrates the auto-downgrade path.
    DowngradedSnapshotCaseMissing,
    /// A weak-truth snapshot was not downgraded below its claim.
    SnapshotNotDowngradedOnWeakTruth,
    /// A downgraded snapshot is missing its trigger or precise label.
    DowngradedSnapshotMissingLabelOrTrigger,
    /// No delta packet is present.
    DeltaPacketMissing,
    /// No imported-versus-live delta demonstrates the cross-boundary path.
    ImportedVsLiveDeltaCaseMissing,
    /// No blocked / incomparable delta demonstrates impersonation prevention.
    BlockedDeltaCaseMissing,
    /// A delta packet is structurally incomplete.
    DeltaPacketIncomplete,
    /// A delta compares a thing to itself rather than two distinct sides.
    DeltaSidesNotDistinct,
    /// A non-exact delta omits its required compatibility note.
    DeltaCompatibilityNoteMissing,
    /// A delta's recorded counts do not match its per-finding deltas.
    DeltaCountsInconsistent,
    /// A delta's two sides risk impersonating one another.
    DeltaImpersonationRisk,
    /// Release-debt owner / expiry / baseline / suppression truth was dropped.
    ReleaseDebtTruthDropped,
    /// One or more guardrails do not hold.
    GuardrailsIncomplete,
    /// One or more consumer projections do not hold.
    ConsumerProjectionIncomplete,
    /// Raw boundary material leaked into the export.
    RawBoundaryMaterialInExport,
}

impl DiagnosticQualityParityViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredOriginCoverageMissing => "required_origin_coverage_missing",
            Self::SnapshotEntryIncomplete => "snapshot_entry_incomplete",
            Self::SnapshotMissingProfileBinding => "snapshot_missing_profile_binding",
            Self::SnapshotMissingToolVersions => "snapshot_missing_tool_versions",
            Self::SnapshotMissingRecentCollection => "snapshot_missing_recent_collection",
            Self::SnapshotImportedShownAsLive => "snapshot_imported_shown_as_live",
            Self::SnapshotSuppressionBaselineTruthMissing => {
                "snapshot_suppression_baseline_truth_missing"
            }
            Self::DowngradedSnapshotCaseMissing => "downgraded_snapshot_case_missing",
            Self::SnapshotNotDowngradedOnWeakTruth => "snapshot_not_downgraded_on_weak_truth",
            Self::DowngradedSnapshotMissingLabelOrTrigger => {
                "downgraded_snapshot_missing_label_or_trigger"
            }
            Self::DeltaPacketMissing => "delta_packet_missing",
            Self::ImportedVsLiveDeltaCaseMissing => "imported_vs_live_delta_case_missing",
            Self::BlockedDeltaCaseMissing => "blocked_delta_case_missing",
            Self::DeltaPacketIncomplete => "delta_packet_incomplete",
            Self::DeltaSidesNotDistinct => "delta_sides_not_distinct",
            Self::DeltaCompatibilityNoteMissing => "delta_compatibility_note_missing",
            Self::DeltaCountsInconsistent => "delta_counts_inconsistent",
            Self::DeltaImpersonationRisk => "delta_impersonation_risk",
            Self::ReleaseDebtTruthDropped => "release_debt_truth_dropped",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// The headline export binding quality snapshots and delta packets to one model.
///
/// See the [module docs](self) for the full contract this packet enforces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticQualityParityPacket {
    /// Stable record-kind tag, always [`M5_DIAGNOSTIC_QUALITY_PARITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version, always [`M5_DIAGNOSTIC_QUALITY_PARITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer-facing label.
    pub packet_label: String,
    /// Diagnostic-quality snapshot entries.
    pub snapshot_entries: Vec<DiagnosticQualitySnapshotEntry>,
    /// Imported-versus-live delta packets.
    pub delta_packets: Vec<DiagnosticDeltaPacket>,
    /// Release-visible debt assembled from the snapshots.
    pub release_debt_projection: DiagnosticQualityReleaseDebtProjection,
    /// Guardrails the lane depends on.
    pub guardrails: DiagnosticQualityParityGuardrails,
    /// Consumer-projection truth.
    pub consumer_projection: DiagnosticQualityParityConsumerProjection,
    /// Required schema / doc / artifact source contracts.
    pub source_contract_refs: Vec<String>,
    /// Redaction class token for the export boundary.
    pub redaction_class_token: String,
    /// When the packet was minted.
    pub minted_at: String,
}

/// Input to [`DiagnosticQualityParityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticQualityParityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer-facing label.
    pub packet_label: String,
    /// Diagnostic-quality snapshot entries.
    pub snapshot_entries: Vec<DiagnosticQualitySnapshotEntry>,
    /// Imported-versus-live delta packets.
    pub delta_packets: Vec<DiagnosticDeltaPacket>,
    /// Release-visible debt assembled from the snapshots.
    pub release_debt_projection: DiagnosticQualityReleaseDebtProjection,
    /// Guardrails the lane depends on.
    pub guardrails: DiagnosticQualityParityGuardrails,
    /// Consumer-projection truth.
    pub consumer_projection: DiagnosticQualityParityConsumerProjection,
    /// Required schema / doc / artifact source contracts.
    pub source_contract_refs: Vec<String>,
    /// Redaction class token for the export boundary.
    pub redaction_class_token: String,
    /// When the packet was minted.
    pub minted_at: String,
}

impl DiagnosticQualityParityPacket {
    /// Builds a packet from [`DiagnosticQualityParityPacketInput`], stamping the
    /// stable record kind and schema version.
    pub fn new(input: DiagnosticQualityParityPacketInput) -> Self {
        Self {
            record_kind: M5_DIAGNOSTIC_QUALITY_PARITY_RECORD_KIND.to_owned(),
            schema_version: M5_DIAGNOSTIC_QUALITY_PARITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            snapshot_entries: input.snapshot_entries,
            delta_packets: input.delta_packets,
            release_debt_projection: input.release_debt_projection,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// The origin classes represented across the snapshot entries.
    pub fn represented_origin_classes(&self) -> BTreeSet<DiagnosticOriginClass> {
        self.snapshot_entries
            .iter()
            .map(|entry| entry.snapshot.origin_class)
            .collect()
    }

    /// The comparison-basis classes represented across the delta packets.
    pub fn represented_basis_classes(&self) -> BTreeSet<DiagnosticDeltaComparisonBasisClass> {
        self.delta_packets
            .iter()
            .map(|delta| delta.comparison_basis_class)
            .collect()
    }

    /// Count of snapshot entries carrying a public claim above held.
    pub fn claimed_snapshot_count(&self) -> usize {
        self.snapshot_entries
            .iter()
            .filter(|entry| entry.is_claimed())
            .count()
    }

    /// Count of snapshot entries downgraded below their claim.
    pub fn downgraded_snapshot_count(&self) -> usize {
        self.snapshot_entries
            .iter()
            .filter(|entry| {
                entry.effective_qualification.rank() < entry.claimed_qualification.rank()
            })
            .count()
    }

    /// Count of delta packets whose verdict blocks or refuses an exact delta.
    pub fn blocked_delta_count(&self) -> usize {
        self.delta_packets
            .iter()
            .filter(|delta| delta.is_blocked_or_incomparable())
            .count()
    }

    /// Validates the packet against the lane's truth and guardrail contract.
    pub fn validate(&self) -> Vec<DiagnosticQualityParityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DIAGNOSTIC_QUALITY_PARITY_RECORD_KIND {
            violations.push(DiagnosticQualityParityViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DIAGNOSTIC_QUALITY_PARITY_SCHEMA_VERSION {
            violations.push(DiagnosticQualityParityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(DiagnosticQualityParityViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_snapshot_coverage(self, &mut violations);
        validate_snapshot_entries(self, &mut violations);
        validate_delta_packets(self, &mut violations);
        validate_release_debt(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("quality-parity packet serializes"),
        ) {
            violations.push(DiagnosticQualityParityViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("quality-parity packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Diagnostic Quality Snapshots and Imported-versus-Live Deltas\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!("- Minted: `{}`\n", self.minted_at));
        out.push_str(&format!("- Snapshots: {}\n", self.snapshot_entries.len()));
        out.push_str(&format!(
            "- Claimed snapshots: {}\n",
            self.claimed_snapshot_count()
        ));
        out.push_str(&format!(
            "- Downgraded snapshots: {}\n",
            self.downgraded_snapshot_count()
        ));
        out.push_str(&format!("- Delta packets: {}\n", self.delta_packets.len()));
        out.push_str(&format!(
            "- Blocked / incomparable deltas: {}\n\n",
            self.blocked_delta_count()
        ));

        out.push_str("## Quality snapshots\n\n");
        out.push_str(
            "| Snapshot | Scope | Origin | Freshness | Tools | Collections | Debt | Claimed | Effective |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
        for entry in &self.snapshot_entries {
            let snapshot = &entry.snapshot;
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                snapshot.snapshot_id,
                snapshot.scope.scope_class.as_str(),
                snapshot.origin_class.as_str(),
                snapshot.freshness_class.as_str(),
                snapshot.tool_versions.len(),
                snapshot.recent_collection_refs.len(),
                snapshot.release_visible_debt_count,
                entry.claimed_qualification.as_str(),
                entry.effective_qualification.as_str(),
            ));
        }

        out.push_str("\n## Imported-versus-live deltas\n\n");
        out.push_str("| Delta | Basis | Base origin | Compare origin | Compatibility | Notes |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- |\n");
        for delta in &self.delta_packets {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} |\n",
                delta.delta_id,
                delta.comparison_basis_class.as_str(),
                delta.base_side.origin_class.as_str(),
                delta.compare_side.origin_class.as_str(),
                delta.compatibility_class.as_str(),
                delta.compatibility_notes.len(),
            ));
        }

        out.push('\n');
        for entry in &self.snapshot_entries {
            if let Some(label) = &entry.degraded_label {
                out.push_str(&format!(
                    "- Degraded: `{}` — {}\n",
                    entry.snapshot.snapshot_id, label
                ));
            }
        }

        out.push_str(&format!(
            "\n- Release-visible debt: {} (assembled from snapshots: {})\n",
            self.release_debt_projection.release_visible_debt_count,
            self.release_debt_projection.assembled_from_snapshots,
        ));

        out
    }
}

fn validate_source_contracts(
    packet: &DiagnosticQualityParityPacket,
    violations: &mut Vec<DiagnosticQualityParityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DIAGNOSTIC_QUALITY_PARITY_SCHEMA_REF,
        DIAGNOSTIC_QUALITY_SNAPSHOT_SCHEMA_REF,
        DIAGNOSTIC_DELTA_PACKET_SCHEMA_REF,
        M5_DIAGNOSTIC_QUALITY_PARITY_DOC_REF,
        M5_DIAGNOSTIC_QUALITY_PARITY_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(DiagnosticQualityParityViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_snapshot_coverage(
    packet: &DiagnosticQualityParityPacket,
    violations: &mut Vec<DiagnosticQualityParityViolation>,
) {
    if packet.snapshot_entries.is_empty() {
        violations.push(DiagnosticQualityParityViolation::SnapshotEntryIncomplete);
        return;
    }

    let origins = packet.represented_origin_classes();
    let has_live = origins
        .iter()
        .any(|origin| !origin.is_imported_or_replayed());
    let has_imported = origins
        .iter()
        .any(|origin| origin.is_imported_or_replayed());
    if !has_live || !has_imported {
        violations.push(DiagnosticQualityParityViolation::RequiredOriginCoverageMissing);
    }

    if !packet
        .snapshot_entries
        .iter()
        .any(|entry| entry.needs_downgrade() && entry.downgrade_consistent())
    {
        violations.push(DiagnosticQualityParityViolation::DowngradedSnapshotCaseMissing);
    }
}

fn validate_snapshot_entries(
    packet: &DiagnosticQualityParityPacket,
    violations: &mut Vec<DiagnosticQualityParityViolation>,
) {
    for entry in &packet.snapshot_entries {
        let snapshot = &entry.snapshot;
        if !entry.is_structurally_complete() {
            violations.push(DiagnosticQualityParityViolation::SnapshotEntryIncomplete);
        }
        if !snapshot.has_profile_binding() {
            violations.push(DiagnosticQualityParityViolation::SnapshotMissingProfileBinding);
        }
        if !snapshot.has_tool_versions() {
            violations.push(DiagnosticQualityParityViolation::SnapshotMissingToolVersions);
        }
        if !snapshot.has_recent_collection() {
            violations.push(DiagnosticQualityParityViolation::SnapshotMissingRecentCollection);
        }
        if !snapshot.imported_disclosed() {
            violations.push(DiagnosticQualityParityViolation::SnapshotImportedShownAsLive);
        }
        if !snapshot.suppression_baseline_truth_present() {
            violations
                .push(DiagnosticQualityParityViolation::SnapshotSuppressionBaselineTruthMissing);
        }
        if entry.needs_downgrade()
            && entry.effective_qualification.rank() >= entry.claimed_qualification.rank()
        {
            violations.push(DiagnosticQualityParityViolation::SnapshotNotDowngradedOnWeakTruth);
        }
        if entry.needs_downgrade()
            && (entry.downgrade_trigger.is_none()
                || !entry
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations
                .push(DiagnosticQualityParityViolation::DowngradedSnapshotMissingLabelOrTrigger);
        }
    }
}

fn validate_delta_packets(
    packet: &DiagnosticQualityParityPacket,
    violations: &mut Vec<DiagnosticQualityParityViolation>,
) {
    if packet.delta_packets.is_empty() {
        violations.push(DiagnosticQualityParityViolation::DeltaPacketMissing);
        return;
    }

    if !packet
        .represented_basis_classes()
        .iter()
        .any(|basis| basis.crosses_imported_live_boundary())
    {
        violations.push(DiagnosticQualityParityViolation::ImportedVsLiveDeltaCaseMissing);
    }

    if packet.blocked_delta_count() == 0 {
        violations.push(DiagnosticQualityParityViolation::BlockedDeltaCaseMissing);
    }

    for delta in &packet.delta_packets {
        if !delta.is_structurally_complete() {
            violations.push(DiagnosticQualityParityViolation::DeltaPacketIncomplete);
        }
        if !delta.sides_distinct() {
            violations.push(DiagnosticQualityParityViolation::DeltaSidesNotDistinct);
        }
        if !delta.compatibility_notes_sufficient() {
            violations.push(DiagnosticQualityParityViolation::DeltaCompatibilityNoteMissing);
        }
        if !delta.counts_consistent() {
            violations.push(DiagnosticQualityParityViolation::DeltaCountsInconsistent);
        }
        if !delta.impersonation_ok() {
            violations.push(DiagnosticQualityParityViolation::DeltaImpersonationRisk);
        }
    }
}

fn validate_release_debt(
    packet: &DiagnosticQualityParityPacket,
    violations: &mut Vec<DiagnosticQualityParityViolation>,
) {
    if !packet.release_debt_projection.truth_preserved() {
        violations.push(DiagnosticQualityParityViolation::ReleaseDebtTruthDropped);
    }
}

fn validate_guardrails(
    packet: &DiagnosticQualityParityPacket,
    violations: &mut Vec<DiagnosticQualityParityViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(DiagnosticQualityParityViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &DiagnosticQualityParityPacket,
    violations: &mut Vec<DiagnosticQualityParityViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(DiagnosticQualityParityViolation::ConsumerProjectionIncomplete);
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
///
/// A generic provider error must never stand in for a precise downgrade truth.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "narrowed"
            | "downgraded"
            | "omitted"
            | "partial"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Error returned when the checked support-export artifact fails to load or
/// validate.
#[derive(Debug)]
pub enum DiagnosticQualityParityArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<DiagnosticQualityParityViolation>),
}

impl fmt::Display for DiagnosticQualityParityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => {
                write!(f, "quality-parity support export parse error: {err}")
            }
            Self::Validation(violations) => write!(
                f,
                "quality-parity packet failed validation: {}",
                violations
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

impl Error for DiagnosticQualityParityArtifactError {}

/// Loads and validates the checked support-export artifact.
///
/// This is the canonical entry point downstream Problems, review, CLI/headless,
/// AI evidence, support, and release-visible debt surfaces use to ingest the
/// frozen diagnostic-quality snapshot and imported-versus-live delta truth
/// instead of cloning provider-local quality state.
///
/// # Errors
///
/// Returns [`DiagnosticQualityParityArtifactError`] when the artifact cannot be
/// parsed or fails validation.
pub fn current_m5_diagnostic_quality_parity_export(
) -> Result<DiagnosticQualityParityPacket, DiagnosticQualityParityArtifactError> {
    let packet: DiagnosticQualityParityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/m5/diagnostics/diagnostic-quality-parity-proof/support_export.json"
    )))
    .map_err(DiagnosticQualityParityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DiagnosticQualityParityArtifactError::Validation(violations))
    }
}
