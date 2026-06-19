//! Freeze of the M5 diagnostic-record, source-descriptor, collection-snapshot,
//! anchor-remap, and quality-session matrix for every claimed M5
//! diagnostic-producing surface.
//!
//! M5 widens the set of surfaces that produce or preserve findings: notebook
//! cells, framework packs, request / API tooling, data tooling, preview
//! runtimes, package lanes, the language-provider plane, the editor-structural
//! guard, and imported scanner / SARIF snapshots. Those lanes only stay
//! trustworthy if every finding resolves to one canonical diagnostic identity
//! with an explicit source kind, imported-versus-live class, freshness, remap
//! state, collection completeness, cluster meaning, and the quality-session
//! outcome that produced or can act on it — rather than letting Problems, the
//! editor, review, the CLI, AI evidence, and support export each infer
//! provider-local meanings.
//!
//! Where [`crate::diagnostics`] froze the per-record canonical diagnostic /
//! source / anchor-remap / cluster / plane-snapshot *objects* and
//! [`crate::quality`] froze the quality-action / session *governance objects*,
//! this module binds them into one bounded **diagnostic-truth lane matrix**.
//! The matrix is the single canonical answer to "for this claimed M5
//! diagnostic-producing surface, what is its source kind, origin class,
//! freshness, remap state, collection completeness, cluster meaning, and
//! governing quality-session outcome — and is the qualification it claims
//! actually backed by an identified diagnostic, source, collection, remap, and
//! session truth?"
//!
//! Each [`DiagnosticLaneRow`] reuses the frozen
//! [`DiagnosticSourceKind`](crate::diagnostics::DiagnosticSourceKind),
//! [`DiagnosticOriginClass`](crate::diagnostics::DiagnosticOriginClass),
//! [`DiagnosticFreshnessClass`](crate::diagnostics::DiagnosticFreshnessClass),
//! [`DiagnosticAnchorRemapStateClass`](crate::diagnostics::DiagnosticAnchorRemapStateClass),
//! and [`QualitySessionOutcomeClass`](crate::quality::QualitySessionOutcomeClass)
//! vocabularies rather than minting synonyms, and adds the matrix-level
//! dimensions this freeze owns: [`DiagnosticCollectionCompletenessClass`] and
//! [`DiagnosticClusterMeaningClass`]. The matrix *auto-downgrades*: a claimed row
//! that cannot identify a source kind, origin class, proven freshness, a remap
//! state, a collection completeness, or a governing quality session must carry
//! an `effective_qualification` strictly below its claim, a recorded downgrade
//! trigger, and a precise degraded label, so a diagnostic claim never outruns the
//! evidence that backs it.
//!
//! [`DiagnosticTruthLaneMatrixPacket::validate`] also refuses a row that flattens
//! unlike sources into a synthetic finding, lets convenience clustering erase
//! source / freshness / remap provenance, silently repairs an anchor instead of
//! recording append-only remap evidence, renders imported or replayed evidence as
//! live local truth, hides partial / filtered / imported collection completeness
//! behind a complete-looking enumeration, drops target / environment / policy
//! refs, or lets a mutating fix route bypass the typed quality-action proposal
//! contract (safety class, preview requirement, rollback boundary).
//!
//! Raw source bytes, raw provider payloads, raw scanner reports, provider
//! cursors, credentials, and raw artifact bodies never cross this boundary; the
//! packet carries only typed class tokens, booleans, opaque ids, and
//! redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/quality/m5-diagnostic-truth-lane.schema.json`](../../../../schemas/quality/m5-diagnostic-truth-lane.schema.json).
//! The contract doc is
//! [`docs/m5/diagnostic-truth-and-quality-sessions.md`](../../../../docs/m5/diagnostic-truth-and-quality-sessions.md).
//! The protected fixture directory is
//! [`fixtures/quality/m5/diagnostic-contract-regression/`](../../../../fixtures/quality/m5/diagnostic-contract-regression/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::diagnostics::{
    DiagnosticAnchorRemapStateClass, DiagnosticFreshnessClass, DiagnosticOriginClass,
    DiagnosticSourceKind,
};
use crate::quality::QualitySessionOutcomeClass;

/// Stable record-kind tag carried by [`DiagnosticTruthLaneMatrixPacket`].
pub const M5_DIAGNOSTIC_TRUTH_LANE_RECORD_KIND: &str =
    "freeze_the_m5_diagnostic_record_source_collection_snapshot_and_anchor_remap_matrix";

/// Schema version for the M5 diagnostic-truth lane matrix.
pub const M5_DIAGNOSTIC_TRUTH_LANE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_DIAGNOSTIC_TRUTH_LANE_SCHEMA_REF: &str =
    "schemas/quality/m5-diagnostic-truth-lane.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DIAGNOSTIC_TRUTH_LANE_DOC_REF: &str =
    "docs/m5/diagnostic-truth-and-quality-sessions.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DIAGNOSTIC_TRUTH_LANE_ARTIFACT_REF: &str =
    "artifacts/m5/diagnostics/freeze-packet/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_DIAGNOSTIC_TRUTH_LANE_SUMMARY_REF: &str =
    "artifacts/m5/diagnostics/freeze-packet/support_export.md";

/// One claimed M5 diagnostic-producing surface a matrix row covers.
///
/// These are the surfaces that mint or preserve findings in M5; the matrix maps
/// each onto the single canonical diagnostic lane so Problems, the editor,
/// review, the CLI, AI evidence, and support export never re-derive
/// provider-local finding identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiagnosticSurface {
    /// Notebook cell analysis, execution, and inline notebook diagnostics.
    NotebookCellDiagnostics,
    /// Framework-pack analyzers, schema checks, and framework lint findings.
    FrameworkPackDiagnostics,
    /// Request / API request tooling validation and response assertions.
    RequestToolingDiagnostics,
    /// Data tooling (query, dataset, connection) validation findings.
    DataToolingDiagnostics,
    /// Preview-runtime build / render / drift findings.
    PreviewRuntimeDiagnostics,
    /// Package-lane resolution, manifest, and policy findings.
    PackageLaneDiagnostics,
    /// Language-service / native semantic analyzer findings.
    LanguageProviderDiagnostics,
    /// Editor-structural parser, encoding, Unicode, or generated-file guards.
    EditorStructuralDiagnostics,
    /// Imported scanner / SARIF-like report / CI snapshot findings.
    ImportedScannerDiagnostics,
}

impl M5DiagnosticSurface {
    /// Every claimed surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::NotebookCellDiagnostics,
        Self::FrameworkPackDiagnostics,
        Self::RequestToolingDiagnostics,
        Self::DataToolingDiagnostics,
        Self::PreviewRuntimeDiagnostics,
        Self::PackageLaneDiagnostics,
        Self::LanguageProviderDiagnostics,
        Self::EditorStructuralDiagnostics,
        Self::ImportedScannerDiagnostics,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookCellDiagnostics => "notebook_cell_diagnostics",
            Self::FrameworkPackDiagnostics => "framework_pack_diagnostics",
            Self::RequestToolingDiagnostics => "request_tooling_diagnostics",
            Self::DataToolingDiagnostics => "data_tooling_diagnostics",
            Self::PreviewRuntimeDiagnostics => "preview_runtime_diagnostics",
            Self::PackageLaneDiagnostics => "package_lane_diagnostics",
            Self::LanguageProviderDiagnostics => "language_provider_diagnostics",
            Self::EditorStructuralDiagnostics => "editor_structural_diagnostics",
            Self::ImportedScannerDiagnostics => "imported_scanner_diagnostics",
        }
    }
}

/// Closed collection-completeness vocabulary owned by this freeze.
///
/// Names how a surface establishes the finding set it presents, so a partial,
/// incremental, filtered, or imported collection never reads as a complete local
/// enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCollectionCompletenessClass {
    /// Full local enumeration of findings for the declared scope.
    CompleteEnumeration,
    /// Partial or streaming scan whose uncovered scope stays visible.
    PartialVisibleScan,
    /// Incremental delta over a prior collection, with the base disclosed.
    IncrementalSinceLast,
    /// Imported scanner / CI snapshot set, not a live local enumeration.
    ImportedSnapshotSet,
    /// Filtered or suppression-applied view that discloses what was withheld.
    FilteredView,
    /// Completeness could not be established and requires review.
    UnknownRequiresReview,
}

impl DiagnosticCollectionCompletenessClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompleteEnumeration => "complete_enumeration",
            Self::PartialVisibleScan => "partial_visible_scan",
            Self::IncrementalSinceLast => "incremental_since_last",
            Self::ImportedSnapshotSet => "imported_snapshot_set",
            Self::FilteredView => "filtered_view",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Whether this completeness class requires visible disclosure rather than
    /// reading as a complete, certain enumeration.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::CompleteEnumeration)
    }

    /// Whether this completeness class is durable enough to back a public claim.
    ///
    /// Only [`Self::UnknownRequiresReview`] forces auto-downgrade; partial,
    /// incremental, imported, and filtered collections back a claim as long as
    /// their disclosure invariants hold.
    pub const fn backs_claim(self) -> bool {
        !matches!(self, Self::UnknownRequiresReview)
    }
}

/// Closed cluster-meaning vocabulary owned by this freeze.
///
/// Names what a display cluster asserts, so convenience clustering never erases
/// provenance or implies a stronger relationship than the evidence proves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticClusterMeaningClass {
    /// One record, no clustering applied.
    NoClustering,
    /// The same finding observed by the same source more than once.
    ExactDuplicate,
    /// The same underlying issue corroborated by multiple distinct sources.
    CrossSourceCorroboration,
    /// Findings grouped because they share a location or range.
    RelatedByLocation,
    /// Findings grouped because they share one causal origin.
    RelatedByCause,
    /// A display-only roll-up that must preserve each member's provenance.
    DisplayRollupOnly,
}

impl DiagnosticClusterMeaningClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoClustering => "no_clustering",
            Self::ExactDuplicate => "exact_duplicate",
            Self::CrossSourceCorroboration => "cross_source_corroboration",
            Self::RelatedByLocation => "related_by_location",
            Self::RelatedByCause => "related_by_cause",
            Self::DisplayRollupOnly => "display_rollup_only",
        }
    }

    /// Whether this cluster meaning groups more than one underlying finding and
    /// must therefore preserve every member's source, freshness, and remap class.
    pub const fn groups_multiple(self) -> bool {
        !matches!(self, Self::NoClustering)
    }
}

/// Whether a [`DiagnosticOriginClass`] is imported or replayed evidence that must
/// never be rendered as live local truth.
const fn origin_is_imported_or_replayed(origin: DiagnosticOriginClass) -> bool {
    origin.is_imported_or_replayed()
}

/// Whether a [`DiagnosticFreshnessClass`] is proven enough to back a claim.
///
/// [`DiagnosticFreshnessClass::Unverified`] cannot back a claim; every other
/// freshness state names a reviewable posture and may stand behind a claim as
/// long as its disclosure invariants hold.
const fn freshness_backs_claim(freshness: DiagnosticFreshnessClass) -> bool {
    !matches!(freshness, DiagnosticFreshnessClass::Unverified)
}

/// Headline qualification a diagnostic-lane row may claim or hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticLaneQualificationClass {
    /// Held below preview until the lane truth is identified.
    Held,
    /// Claimed at preview maturity.
    Preview,
    /// Claimed at beta maturity.
    Beta,
    /// Claimed at stable maturity.
    Stable,
}

impl DiagnosticLaneQualificationClass {
    /// Stable token recorded in the matrix.
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

/// Trigger that fired an auto-downgrade on a diagnostic-lane row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticLaneDowngradeTrigger {
    /// Source kind could not be identified.
    UnidentifiedSourceKind,
    /// Imported-versus-live origin class could not be identified.
    UnidentifiedOriginClass,
    /// Freshness could not be proven for the admitted scope.
    UnprovenFreshness,
    /// Anchor remap state could not be resolved.
    UnresolvedRemapState,
    /// Collection completeness could not be established.
    UnknownCollectionCompleteness,
    /// No governing quality session backs the lane.
    UnlinkedQualitySession,
    /// Evidence is stale beyond the freshness window.
    StaleEvidenceWindow,
}

impl DiagnosticLaneDowngradeTrigger {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnidentifiedSourceKind => "unidentified_source_kind",
            Self::UnidentifiedOriginClass => "unidentified_origin_class",
            Self::UnprovenFreshness => "unproven_freshness",
            Self::UnresolvedRemapState => "unresolved_remap_state",
            Self::UnknownCollectionCompleteness => "unknown_collection_completeness",
            Self::UnlinkedQualitySession => "unlinked_quality_session",
            Self::StaleEvidenceWindow => "stale_evidence_window",
        }
    }
}

/// One claimed M5 diagnostic-producing surface mapped onto the canonical lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticLaneRow {
    /// Stable row id.
    pub row_id: String,
    /// Claimed diagnostic-producing surface.
    pub surface: M5DiagnosticSurface,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Identified canonical source kind. `None` forces auto-downgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_kind: Option<DiagnosticSourceKind>,
    /// Identified imported-versus-live origin class. `None` forces auto-downgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_class: Option<DiagnosticOriginClass>,
    /// Identified freshness class. `None`, or [`DiagnosticFreshnessClass::Unverified`],
    /// forces auto-downgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness_class: Option<DiagnosticFreshnessClass>,
    /// Identified anchor-remap state class. `None` forces auto-downgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remap_state_class: Option<DiagnosticAnchorRemapStateClass>,
    /// Identified collection-completeness class. `None`, or
    /// [`DiagnosticCollectionCompletenessClass::UnknownRequiresReview`], forces
    /// auto-downgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection_completeness_class: Option<DiagnosticCollectionCompletenessClass>,
    /// Cluster meaning this surface asserts.
    pub cluster_meaning_class: DiagnosticClusterMeaningClass,
    /// Governing quality-session outcome that produced or can act on the lane.
    /// `None` forces auto-downgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality_session_outcome_class: Option<QualitySessionOutcomeClass>,
    /// True when clustering preserves each member's source, freshness, and remap
    /// provenance rather than collapsing unlike sources into a synthetic finding.
    pub provenance_preserved_in_clustering: bool,
    /// True when imported / replayed evidence is never rendered as live local
    /// truth.
    pub imported_not_shown_as_live: bool,
    /// True when non-current freshness and non-exact remap carry a visible cue.
    pub freshness_and_remap_disclosed: bool,
    /// True when anchor remap is append-only evidence, never a silent repair.
    pub anchor_remap_append_only: bool,
    /// True when partial / incremental / imported / filtered completeness is
    /// disclosed rather than shown as a complete enumeration.
    pub collection_completeness_disclosed: bool,
    /// True when target / environment / policy refs are preserved on the lane.
    pub target_environment_refs_preserved: bool,
    /// True when every mutating fix route is a typed quality-action proposal with
    /// a safety class, preview requirement, and rollback boundary.
    pub mutating_fix_is_typed_proposal: bool,
    /// Headline qualification publicly claimed for this row.
    pub claimed_qualification: DiagnosticLaneQualificationClass,
    /// Effective qualification after auto-downgrade; equals the claim when every
    /// lane dimension is identified, and ranks strictly below it otherwise.
    pub effective_qualification: DiagnosticLaneQualificationClass,
    /// Trigger that fired the downgrade, required when the row is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<DiagnosticLaneDowngradeTrigger>,
    /// Precise degraded label, required when the row is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl DiagnosticLaneRow {
    /// Whether this row carries a public claim.
    pub fn is_claimed(&self) -> bool {
        self.claimed_qualification.is_claimed()
    }

    /// Whether every required lane dimension (source kind, origin class, proven
    /// freshness, remap state, collection completeness, governing session) is
    /// identified.
    pub fn identity_complete(&self) -> bool {
        self.source_kind.is_some()
            && self.origin_class.is_some()
            && self.freshness_class.is_some_and(freshness_backs_claim)
            && self.remap_state_class.is_some()
            && self
                .collection_completeness_class
                .is_some_and(DiagnosticCollectionCompletenessClass::backs_claim)
            && self.quality_session_outcome_class.is_some()
    }

    /// Whether the row must downgrade below its claim because a lane dimension is
    /// missing or not durable.
    pub fn needs_downgrade(&self) -> bool {
        !self.identity_complete()
    }

    /// Whether the effective qualification and downgrade evidence are consistent.
    ///
    /// When every lane dimension is present the effective qualification equals
    /// the claim; otherwise it must rank strictly below the claim and carry both a
    /// recorded downgrade trigger and a precise degraded label.
    pub fn downgrade_consistent(&self) -> bool {
        if self.needs_downgrade() {
            self.effective_qualification.rank() < self.claimed_qualification.rank()
                && self.downgrade_trigger.is_some()
                && self
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.effective_qualification == self.claimed_qualification
        }
    }

    /// Whether imported / replayed evidence keeps explicit separation from live
    /// local truth.
    pub fn imported_local_separation_ok(&self) -> bool {
        match self.origin_class {
            Some(origin) if origin_is_imported_or_replayed(origin) => {
                self.imported_not_shown_as_live
            }
            _ => true,
        }
    }

    /// Whether non-current freshness and non-exact remap are disclosed.
    pub fn freshness_remap_disclosure_ok(&self) -> bool {
        let freshness_needs = self
            .freshness_class
            .is_some_and(|freshness| !matches!(freshness, DiagnosticFreshnessClass::Current));
        let remap_needs = self
            .remap_state_class
            .is_some_and(DiagnosticAnchorRemapStateClass::requires_disclosure);
        if freshness_needs || remap_needs {
            self.freshness_and_remap_disclosed
        } else {
            true
        }
    }

    /// Whether partial / incremental / imported / filtered completeness is
    /// disclosed rather than shown as a complete enumeration.
    pub fn collection_disclosure_ok(&self) -> bool {
        match self.collection_completeness_class {
            Some(completeness) if completeness.requires_disclosure() => {
                self.collection_completeness_disclosed
            }
            _ => true,
        }
    }

    /// Whether clustering preserves provenance whenever it groups multiple
    /// findings.
    pub fn clustering_provenance_ok(&self) -> bool {
        if self.cluster_meaning_class.groups_multiple() {
            self.provenance_preserved_in_clustering
        } else {
            true
        }
    }

    /// Whether this row holds every structural invariant the matrix requires.
    pub fn is_complete(&self) -> bool {
        self.downgrade_consistent()
            && self.imported_local_separation_ok()
            && self.freshness_remap_disclosure_ok()
            && self.collection_disclosure_ok()
            && self.clustering_provenance_ok()
            && self.anchor_remap_append_only
            && self.target_environment_refs_preserved
            && self.mutating_fix_is_typed_proposal
            && !self.row_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Matrix-level guardrail invariants that must all hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticLaneGuardrails {
    /// Unlike sources are never flattened into one synthetic finding.
    pub unlike_sources_never_flattened: bool,
    /// Anchors are never silently repaired; remap is append-only evidence.
    pub anchors_never_silently_repaired: bool,
    /// Convenience clustering never erases source / freshness / remap class.
    pub clustering_never_erases_class: bool,
    /// Imported-versus-live class stays explicit on every row.
    pub imported_live_class_explicit: bool,
    /// Freshness and remap state stay explicit on every row.
    pub freshness_and_remap_explicit: bool,
    /// Collection completeness stays disclosed and exportable.
    pub collection_completeness_exportable: bool,
    /// Every mutating fix route is a typed quality-action proposal.
    pub mutating_fixes_are_typed_proposals: bool,
    /// Rows auto-downgrade when any lane dimension is unidentified.
    pub rows_auto_downgrade_on_unidentified_lane: bool,
}

impl DiagnosticLaneGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.unlike_sources_never_flattened
            && self.anchors_never_silently_repaired
            && self.clustering_never_erases_class
            && self.imported_live_class_explicit
            && self.freshness_and_remap_explicit
            && self.collection_completeness_exportable
            && self.mutating_fixes_are_typed_proposals
            && self.rows_auto_downgrade_on_unidentified_lane
    }
}

/// Declares which downstream consumers ingest the frozen lane directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticLaneConsumerProjection {
    /// Editor decorations / markers ingest the lane.
    pub editor_ingests_lane: bool,
    /// Problems rows ingest the lane.
    pub problems_ingests_lane: bool,
    /// Review packets ingest the lane.
    pub review_ingests_lane: bool,
    /// CLI / headless explain output ingests the lane.
    pub cli_headless_ingests_lane: bool,
    /// AI evidence references ingest the lane.
    pub ai_evidence_ingests_lane: bool,
    /// Support export ingests the lane.
    pub support_export_ingests_lane: bool,
    /// Downgraded rows are labeled below their current claim everywhere.
    pub downgraded_rows_labeled_below_current: bool,
}

impl DiagnosticLaneConsumerProjection {
    /// Whether every consumer projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.editor_ingests_lane
            && self.problems_ingests_lane
            && self.review_ingests_lane
            && self.cli_headless_ingests_lane
            && self.ai_evidence_ingests_lane
            && self.support_export_ingests_lane
            && self.downgraded_rows_labeled_below_current
    }
}

/// Evidence freshness window for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticLaneEvidenceFreshness {
    /// Freshness SLO in hours; zero is invalid.
    pub evidence_freshness_slo_hours: u32,
    /// Timestamp of the last evidence refresh.
    pub last_evidence_refresh: String,
    /// Whether a row auto-downgrades when its evidence is stale.
    pub auto_downgrade_on_stale: bool,
}

/// Constructor input for a [`DiagnosticTruthLaneMatrixPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticTruthLaneMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Per-row lane qualifications.
    pub rows: Vec<DiagnosticLaneRow>,
    /// Guardrail invariants block.
    pub guardrails: DiagnosticLaneGuardrails,
    /// Consumer projection block.
    pub consumer_projection: DiagnosticLaneConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: DiagnosticLaneEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 diagnostic-truth lane matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticTruthLaneMatrixPacket {
    /// Record kind; must equal [`M5_DIAGNOSTIC_TRUTH_LANE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DIAGNOSTIC_TRUTH_LANE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Per-row lane qualifications.
    pub rows: Vec<DiagnosticLaneRow>,
    /// Guardrail invariants block.
    pub guardrails: DiagnosticLaneGuardrails,
    /// Consumer projection block.
    pub consumer_projection: DiagnosticLaneConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: DiagnosticLaneEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl DiagnosticTruthLaneMatrixPacket {
    /// Builds an M5 diagnostic-truth lane matrix packet.
    pub fn new(input: DiagnosticTruthLaneMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_DIAGNOSTIC_TRUTH_LANE_RECORD_KIND.to_owned(),
            schema_version: M5_DIAGNOSTIC_TRUTH_LANE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            evidence_freshness: input.evidence_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surfaces represented by some row in this matrix.
    pub fn represented_surfaces(&self) -> BTreeSet<M5DiagnosticSurface> {
        self.rows.iter().map(|row| row.surface).collect()
    }

    /// Count of rows whose effective qualification was downgraded below its claim.
    pub fn downgraded_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.needs_downgrade()).count()
    }

    /// Count of rows holding a public claim.
    pub fn claimed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.is_claimed()).count()
    }

    /// Validates the M5 diagnostic-truth lane invariants.
    pub fn validate(&self) -> Vec<DiagnosticTruthLaneViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DIAGNOSTIC_TRUTH_LANE_RECORD_KIND {
            violations.push(DiagnosticTruthLaneViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DIAGNOSTIC_TRUTH_LANE_SCHEMA_VERSION {
            violations.push(DiagnosticTruthLaneViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(DiagnosticTruthLaneViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_evidence_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("diagnostic-truth lane packet serializes"),
        ) {
            violations.push(DiagnosticTruthLaneViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("diagnostic-truth lane packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Diagnostic-Truth Lane Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!("- Minted: `{}`\n", self.minted_at));
        out.push_str(&format!("- Rows: {}\n", self.rows.len()));
        out.push_str(&format!("- Claimed rows: {}\n", self.claimed_row_count()));
        out.push_str(&format!(
            "- Downgraded rows: {}\n\n",
            self.downgraded_row_count()
        ));

        out.push_str(
            "| Surface | Source | Freshness | Completeness | Session | Claimed | Effective |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                row.surface.as_str(),
                row.source_kind
                    .map_or("unidentified", DiagnosticSourceKind::as_str),
                row.freshness_class
                    .map_or("unidentified", DiagnosticFreshnessClass::as_str),
                row.collection_completeness_class.map_or(
                    "unidentified",
                    DiagnosticCollectionCompletenessClass::as_str
                ),
                row.quality_session_outcome_class
                    .map_or("unlinked", QualitySessionOutcomeClass::as_str),
                row.claimed_qualification.as_str(),
                row.effective_qualification.as_str(),
            ));
        }

        out.push('\n');
        for row in &self.rows {
            if let Some(label) = &row.degraded_label {
                out.push_str(&format!(
                    "- Degraded: `{}` — {}\n",
                    row.surface.as_str(),
                    label
                ));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or
/// validate.
#[derive(Debug)]
pub enum DiagnosticTruthLaneArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<DiagnosticTruthLaneViolation>),
}

impl fmt::Display for DiagnosticTruthLaneArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => {
                write!(f, "diagnostic-truth lane support export parse error: {err}")
            }
            Self::Validation(violations) => write!(
                f,
                "diagnostic-truth lane support export failed validation: {violations:?}"
            ),
        }
    }
}

impl Error for DiagnosticTruthLaneArtifactError {}

/// Invariant violations reported by [`DiagnosticTruthLaneMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticTruthLaneViolation {
    /// Record kind is wrong.
    WrongRecordKind,
    /// Schema version is wrong.
    WrongSchemaVersion,
    /// Packet identity fields are missing.
    MissingIdentity,
    /// Required canonical source contracts are missing.
    MissingSourceContracts,
    /// A required diagnostic-producing surface is unrepresented.
    RequiredSurfaceMissing,
    /// No consistent downgraded row demonstrates the auto-downgrade rule.
    DowngradedRowCaseMissing,
    /// A row failed its structural completeness invariants.
    RowIncomplete,
    /// A row with an unidentified lane dimension was not downgraded.
    RowNotDowngradedOnUnidentifiedLane,
    /// A downgraded row is missing its precise label or trigger.
    DowngradedRowMissingLabelOrTrigger,
    /// Clustering erased member provenance.
    ClusteringErasesProvenance,
    /// Imported or replayed evidence was rendered as live local truth.
    ImportedShownAsLive,
    /// Non-current freshness or non-exact remap was not disclosed.
    FreshnessOrRemapHidden,
    /// Partial / imported / filtered completeness was hidden.
    CollectionCompletenessHidden,
    /// Anchor remap was not kept append-only.
    AnchorRemapNotAppendOnly,
    /// Target / environment / policy refs were dropped.
    TargetEnvironmentRefsDropped,
    /// A mutating fix route bypassed the typed quality-action proposal contract.
    MutatingFixNotTypedProposal,
    /// A row is missing backing evidence refs.
    RowEvidenceMissing,
    /// Guardrail block is incomplete.
    GuardrailsIncomplete,
    /// Consumer projection block is incomplete.
    ConsumerProjectionIncomplete,
    /// Evidence freshness block is incomplete.
    EvidenceFreshnessIncomplete,
    /// Export-safe JSON carried forbidden boundary material.
    RawBoundaryMaterialInExport,
}

impl DiagnosticTruthLaneViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::DowngradedRowCaseMissing => "downgraded_row_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::RowNotDowngradedOnUnidentifiedLane => "row_not_downgraded_on_unidentified_lane",
            Self::DowngradedRowMissingLabelOrTrigger => "downgraded_row_missing_label_or_trigger",
            Self::ClusteringErasesProvenance => "clustering_erases_provenance",
            Self::ImportedShownAsLive => "imported_shown_as_live",
            Self::FreshnessOrRemapHidden => "freshness_or_remap_hidden",
            Self::CollectionCompletenessHidden => "collection_completeness_hidden",
            Self::AnchorRemapNotAppendOnly => "anchor_remap_not_append_only",
            Self::TargetEnvironmentRefsDropped => "target_environment_refs_dropped",
            Self::MutatingFixNotTypedProposal => "mutating_fix_not_typed_proposal",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Loads and validates the checked support-export artifact.
///
/// This is the canonical entry point downstream support, AI evidence, review,
/// and release-visible debt surfaces use to ingest the frozen lane instead of
/// cloning provider-local state.
///
/// # Errors
///
/// Returns [`DiagnosticTruthLaneArtifactError`] when the artifact cannot be
/// parsed or fails validation.
pub fn current_m5_diagnostic_truth_lane_export(
) -> Result<DiagnosticTruthLaneMatrixPacket, DiagnosticTruthLaneArtifactError> {
    let packet: DiagnosticTruthLaneMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/m5/diagnostics/freeze-packet/support_export.json"
    )))
    .map_err(DiagnosticTruthLaneArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DiagnosticTruthLaneArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &DiagnosticTruthLaneMatrixPacket,
    violations: &mut Vec<DiagnosticTruthLaneViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DIAGNOSTIC_TRUTH_LANE_SCHEMA_REF,
        M5_DIAGNOSTIC_TRUTH_LANE_DOC_REF,
        M5_DIAGNOSTIC_TRUTH_LANE_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(DiagnosticTruthLaneViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &DiagnosticTruthLaneMatrixPacket,
    violations: &mut Vec<DiagnosticTruthLaneViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in M5DiagnosticSurface::ALL {
        if !surfaces.contains(&required) {
            violations.push(DiagnosticTruthLaneViolation::RequiredSurfaceMissing);
            break;
        }
    }

    if !packet
        .rows
        .iter()
        .any(|row| row.needs_downgrade() && row.downgrade_consistent())
    {
        violations.push(DiagnosticTruthLaneViolation::DowngradedRowCaseMissing);
    }
}

fn validate_rows(
    packet: &DiagnosticTruthLaneMatrixPacket,
    violations: &mut Vec<DiagnosticTruthLaneViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(DiagnosticTruthLaneViolation::RowIncomplete);
        }
        if row.needs_downgrade()
            && row.effective_qualification.rank() >= row.claimed_qualification.rank()
        {
            violations.push(DiagnosticTruthLaneViolation::RowNotDowngradedOnUnidentifiedLane);
        }
        if row.needs_downgrade()
            && (row.downgrade_trigger.is_none()
                || !row
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations.push(DiagnosticTruthLaneViolation::DowngradedRowMissingLabelOrTrigger);
        }
        if !row.clustering_provenance_ok() {
            violations.push(DiagnosticTruthLaneViolation::ClusteringErasesProvenance);
        }
        if !row.imported_local_separation_ok() {
            violations.push(DiagnosticTruthLaneViolation::ImportedShownAsLive);
        }
        if !row.freshness_remap_disclosure_ok() {
            violations.push(DiagnosticTruthLaneViolation::FreshnessOrRemapHidden);
        }
        if !row.collection_disclosure_ok() {
            violations.push(DiagnosticTruthLaneViolation::CollectionCompletenessHidden);
        }
        if !row.anchor_remap_append_only {
            violations.push(DiagnosticTruthLaneViolation::AnchorRemapNotAppendOnly);
        }
        if !row.target_environment_refs_preserved {
            violations.push(DiagnosticTruthLaneViolation::TargetEnvironmentRefsDropped);
        }
        if !row.mutating_fix_is_typed_proposal {
            violations.push(DiagnosticTruthLaneViolation::MutatingFixNotTypedProposal);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(DiagnosticTruthLaneViolation::RowEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &DiagnosticTruthLaneMatrixPacket,
    violations: &mut Vec<DiagnosticTruthLaneViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(DiagnosticTruthLaneViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &DiagnosticTruthLaneMatrixPacket,
    violations: &mut Vec<DiagnosticTruthLaneViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(DiagnosticTruthLaneViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_evidence_freshness(
    packet: &DiagnosticTruthLaneMatrixPacket,
    violations: &mut Vec<DiagnosticTruthLaneViolation>,
) {
    if packet.evidence_freshness.evidence_freshness_slo_hours == 0
        || packet
            .evidence_freshness
            .last_evidence_refresh
            .trim()
            .is_empty()
    {
        violations.push(DiagnosticTruthLaneViolation::EvidenceFreshnessIncomplete);
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
