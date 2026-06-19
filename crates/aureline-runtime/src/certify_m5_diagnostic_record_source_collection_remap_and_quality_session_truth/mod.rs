//! Release-bearing certification of normalized diagnostic-record, source,
//! collection-snapshot, anchor-remap, and quality-session truth on every claimed
//! M5 code-quality and runtime-diagnostic row.
//!
//! Where
//! [`crate::freeze_the_m5_diagnostic_record_source_collection_snapshot_and_anchor_remap_matrix`]
//! freezes *which* canonical diagnostic-truth object class each claimed
//! diagnostic-producing surface resolves to, this module certifies whether each
//! claimed M5 **row** — a notebook, framework-pack, request/data-tooling,
//! preview/runtime, package, imported-scanner, or review/support/CLI row — actually
//! carries *current* evidence for every dimension it claims. It is the capstone
//! gate: a row may keep its certification grade only when its record-identity,
//! source-descriptor, collection-snapshot, anchor-remap, and (when claimed)
//! quality-session proof is present, reopenable, and inside its freshness window. A
//! row that loses current proof auto-narrows below its claim instead of coasting on
//! an adjacent green row.
//!
//! * a [`CertifiedDiagnosticRow`] ties a durable [`CertifiedDiagnosticSubject`]
//!   (keyed by a [`crate::diagnostics::DiagnosticSourceKind`], a
//!   [`crate::diagnostics::DiagnosticOriginClass`] imported-versus-live origin, and
//!   a non-display fingerprint distinct from its id) to a list of
//!   [`DiagnosticDimensionCertification`] rows over the
//!   [`DiagnosticEvidenceDimension`] vocabulary, a claimed
//!   [`DiagnosticCertificationGrade`], an effective grade, and — when narrowed — a
//!   [`DiagnosticCertificationNarrowTrigger`] plus a precise narrowed label;
//! * each [`DiagnosticDimensionCertification`] is **evidence-bound, not asserted**:
//!   it names a [`DiagnosticProofCurrency`] and, unless the proof is missing, a
//!   reopenable `proof_ref` keyed by a non-display fingerprint, so certification
//!   review can reopen the same record / source / collection / remap / session
//!   evidence object that backs the grade;
//! * the row **auto-narrows**: [`CertifiedDiagnosticRow::needs_narrow`] is true
//!   whenever a required-core dimension is uncertified or any certified dimension
//!   lacks current proof (stale, missing, requires-review, or imported proof
//!   standing in for a local claim). A narrowed row must carry an effective grade
//!   strictly below its claim, a recorded trigger, and a precise label — never a
//!   generic non-answer.
//!
//! [`DiagnosticTruthCertificationPacket::validate`] also refuses a packet that lets
//! convenience clustering erase a finding's source kind, lets an imported scanner
//! row read as a live local rerun, hides a partial/streaming collection behind a
//! generic green grade, silently repairs an anchor instead of recording append-only
//! remap evidence, or lets a mutating fix route bypass the typed quality-action /
//! session lifecycle used elsewhere.
//!
//! Raw diagnostic source, raw provider payloads, raw scanner report bytes, provider
//! cursors, credentials, and raw artifact bodies never cross this boundary; the
//! packet carries only typed class tokens, booleans, opaque ids, fingerprint
//! digests, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/quality/m5-diagnostic-cert-report.schema.json`](../../../../schemas/quality/m5-diagnostic-cert-report.schema.json).
//! The contract doc is
//! [`docs/m5/diagnostic-truth-certification.md`](../../../../docs/m5/diagnostic-truth-certification.md).
//! The protected fixture directory is
//! [`fixtures/quality/m5/certification-corpus/`](../../../../fixtures/quality/m5/certification-corpus/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::diagnostics::{DiagnosticOriginClass, DiagnosticSourceKind};

/// Stable record-kind tag carried by [`DiagnosticTruthCertificationPacket`].
pub const DIAGNOSTIC_TRUTH_CERT_RECORD_KIND: &str =
    "certify_m5_diagnostic_record_source_collection_remap_and_quality_session_truth_packet";

/// Schema version for the diagnostic-truth certification packet.
pub const DIAGNOSTIC_TRUTH_CERT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DIAGNOSTIC_TRUTH_CERT_SCHEMA_REF: &str =
    "schemas/quality/m5-diagnostic-cert-report.schema.json";

/// Repo-relative path of the contract doc.
pub const DIAGNOSTIC_TRUTH_CERT_DOC_REF: &str = "docs/m5/diagnostic-truth-certification.md";

/// Repo-relative path of the checked support-export artifact.
pub const DIAGNOSTIC_TRUTH_CERT_ARTIFACT_REF: &str =
    "artifacts/m5/diagnostics/certification-report/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DIAGNOSTIC_TRUTH_CERT_SUMMARY_REF: &str =
    "artifacts/m5/diagnostics/certification-report/support_export.md";

/// Repo-relative path of the checked waiver-and-downgrade log.
pub const DIAGNOSTIC_TRUTH_CERT_WAIVER_LOG_REF: &str =
    "artifacts/m5/diagnostics/waiver-and-downgrade-log/support_export.md";

/// Repo-relative path of the protected fixture directory.
pub const DIAGNOSTIC_TRUTH_CERT_FIXTURE_DIR: &str = "fixtures/quality/m5/certification-corpus";

/// Kind of claimed M5 diagnostic-producing row a certification covers. Each kind is
/// a distinct claim surface that must carry its own current evidence rather than
/// inheriting a neighbour's grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedDiagnosticRowKind {
    /// A notebook-backed diagnostic row (notebook cells / outputs).
    NotebookRow,
    /// A framework-pack diagnostic row (language / framework analyzers).
    FrameworkRow,
    /// A request / data-tooling diagnostic row.
    RequestDataRow,
    /// A preview / runtime diagnostic row (preview surface, observed execution).
    PreviewRuntimeRow,
    /// A package-lane diagnostic row (dependency / manifest / lockfile findings).
    PackageRow,
    /// An imported-scanner diagnostic row (SARIF-like report, CI scan, provider scan).
    ImportedScannerRow,
    /// A review / support / CLI diagnostic row (review panel, support packet, headless output).
    ReviewSupportCliRow,
}

impl CertifiedDiagnosticRowKind {
    /// Every row kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::NotebookRow,
        Self::FrameworkRow,
        Self::RequestDataRow,
        Self::PreviewRuntimeRow,
        Self::PackageRow,
        Self::ImportedScannerRow,
        Self::ReviewSupportCliRow,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookRow => "notebook_row",
            Self::FrameworkRow => "framework_row",
            Self::RequestDataRow => "request_data_row",
            Self::PreviewRuntimeRow => "preview_runtime_row",
            Self::PackageRow => "package_row",
            Self::ImportedScannerRow => "imported_scanner_row",
            Self::ReviewSupportCliRow => "review_support_cli_row",
        }
    }

    /// Whether this row kind is an imported / provider-backed scanner row by nature.
    pub const fn is_imported_kind(self) -> bool {
        matches!(self, Self::ImportedScannerRow)
    }
}

/// One evidence dimension a diagnostic row is certified against. The first four are
/// the **required core** every claimed row must certify; quality-session is a
/// quality dimension a row certifies only when it owns a mutating fix route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticEvidenceDimension {
    /// Normalized diagnostic-record identity (stable canonical id, reopen handle).
    RecordIdentity,
    /// Source-descriptor truth (source kind, imported-versus-live origin, confidence).
    SourceDescriptor,
    /// Collection-snapshot truth (scope, completeness, freshness, streaming cursor).
    CollectionSnapshot,
    /// Anchor-remap truth (append-only remap history, drift state, revision pairs).
    AnchorRemap,
    /// Quality-session truth (typed quality-action proposals and sessions).
    QualitySession,
}

impl DiagnosticEvidenceDimension {
    /// Every evidence dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RecordIdentity,
        Self::SourceDescriptor,
        Self::CollectionSnapshot,
        Self::AnchorRemap,
        Self::QualitySession,
    ];

    /// The required-core dimensions every claimed row must certify.
    pub const REQUIRED_CORE: [Self; 4] = [
        Self::RecordIdentity,
        Self::SourceDescriptor,
        Self::CollectionSnapshot,
        Self::AnchorRemap,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecordIdentity => "record_identity",
            Self::SourceDescriptor => "source_descriptor",
            Self::CollectionSnapshot => "collection_snapshot",
            Self::AnchorRemap => "anchor_remap",
            Self::QualitySession => "quality_session",
        }
    }

    /// Whether this dimension is part of the required core.
    pub const fn is_core(self) -> bool {
        matches!(
            self,
            Self::RecordIdentity
                | Self::SourceDescriptor
                | Self::CollectionSnapshot
                | Self::AnchorRemap
        )
    }
}

/// Currency of the proof backing one dimension certification. Only a current,
/// reopenable proof backs a claim; a stale, missing, imported-on-local, or
/// review-pending proof auto-narrows the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticProofCurrency {
    /// A fresh local proof verified inside its freshness window.
    VerifiedCurrent,
    /// A cached local proof still inside its freshness window.
    CachedWithinWindow,
    /// A current proof imported / provider-backed and read-only locally.
    ImportedCurrent,
    /// A proof that exists but has aged outside its freshness window.
    StaleExpired,
    /// No proof object exists for this dimension.
    MissingProof,
    /// A provider verdict that still requires review and fails closed.
    RequiresReview,
}

impl DiagnosticProofCurrency {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedCurrent => "verified_current",
            Self::CachedWithinWindow => "cached_within_window",
            Self::ImportedCurrent => "imported_current",
            Self::StaleExpired => "stale_expired",
            Self::MissingProof => "missing_proof",
            Self::RequiresReview => "requires_review",
        }
    }

    /// Whether this is a current, locally verified or cached proof.
    pub const fn is_current_local(self) -> bool {
        matches!(self, Self::VerifiedCurrent | Self::CachedWithinWindow)
    }

    /// Whether this is a current imported / provider-backed proof.
    pub const fn is_imported_current(self) -> bool {
        matches!(self, Self::ImportedCurrent)
    }

    /// Whether this currency carries no proof object (only [`Self::MissingProof`]).
    pub const fn is_absent(self) -> bool {
        matches!(self, Self::MissingProof)
    }
}

/// Certification grade a row claims or effectively holds. Higher [`Self::rank`] is a
/// stronger claim, so a narrowed row must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCertificationGrade {
    /// Fully certified and release-bearing.
    ReleaseCertified,
    /// Certified, publicly claimed.
    Certified,
    /// Provisionally certified (e.g. imported-current evidence only).
    ProvisionallyCertified,
    /// Not certified; held below a public claim.
    Uncertified,
    /// Certification does not apply on this row.
    NotApplicable,
}

impl DiagnosticCertificationGrade {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCertified => "release_certified",
            Self::Certified => "certified",
            Self::ProvisionallyCertified => "provisionally_certified",
            Self::Uncertified => "uncertified",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this grade carries a public certification claim.
    pub const fn is_certified(self) -> bool {
        matches!(
            self,
            Self::ReleaseCertified | Self::Certified | Self::ProvisionallyCertified
        )
    }

    /// Ordinal rank; higher is a stronger claim, so a narrow must move strictly
    /// lower.
    pub const fn rank(self) -> u8 {
        match self {
            Self::NotApplicable => 0,
            Self::Uncertified => 1,
            Self::ProvisionallyCertified => 2,
            Self::Certified => 3,
            Self::ReleaseCertified => 4,
        }
    }
}

/// Reason a claimed row auto-narrowed below its claim. The chrome quotes the trigger
/// verbatim instead of a generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCertificationNarrowTrigger {
    /// A required-core dimension carries no certification at all.
    MissingDimensionProof,
    /// A certified dimension's proof aged outside its freshness window.
    StaleDimensionProof,
    /// A local row leaned on imported / provider proof to back a local claim.
    ImportedProofOnLocalRow,
    /// A provider verdict still requires review and fails closed.
    VerdictRequiresReview,
    /// Source kind / imported-versus-live class could not be resolved.
    SourceKindUnresolved,
    /// Collection completeness / freshness could not be certified current.
    CollectionCompletenessUnproven,
    /// Anchor-remap evidence could not be certified current.
    AnchorRemapEvidenceMissing,
    /// Quality-session parity could not be certified current.
    QualitySessionParityLost,
    /// An upstream dependency narrowed and dragged this row down with it.
    UpstreamDependencyNarrowed,
}

impl DiagnosticCertificationNarrowTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingDimensionProof => "missing_dimension_proof",
            Self::StaleDimensionProof => "stale_dimension_proof",
            Self::ImportedProofOnLocalRow => "imported_proof_on_local_row",
            Self::VerdictRequiresReview => "verdict_requires_review",
            Self::SourceKindUnresolved => "source_kind_unresolved",
            Self::CollectionCompletenessUnproven => "collection_completeness_unproven",
            Self::AnchorRemapEvidenceMissing => "anchor_remap_evidence_missing",
            Self::QualitySessionParityLost => "quality_session_parity_lost",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Durable subject of a certified diagnostic row, keyed by a source kind, an
/// imported-versus-live origin class, and a non-display fingerprint distinct from
/// its id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedDiagnosticSubject {
    /// Durable diagnostic-collection / lane id of the certified row's subject.
    pub subject_id: String,
    /// Source kind, reusing the canonical diagnostic source vocabulary so unlike
    /// sources are never flattened into a synthetic finding.
    pub source_kind: DiagnosticSourceKind,
    /// Imported-versus-live origin class, reusing the canonical diagnostic origin
    /// vocabulary so imported evidence never reads as a live local result.
    pub origin_class: DiagnosticOriginClass,
    /// Non-display fingerprint token. Must differ from
    /// [`subject_id`](CertifiedDiagnosticSubject::subject_id).
    pub subject_fingerprint_token: String,
}

impl CertifiedDiagnosticSubject {
    /// Whether this subject is imported / replayed evidence held read-only.
    pub fn is_imported(&self) -> bool {
        self.origin_class.is_imported_or_replayed()
    }

    /// Whether the fingerprint is a real non-display basis distinct from the id.
    pub fn fingerprint_independent_of_id(&self) -> bool {
        let token = self.subject_fingerprint_token.trim();
        !token.is_empty() && token != self.subject_id.trim()
    }

    /// Whether the subject carries the durable identity a reopen needs.
    pub fn is_valid(&self) -> bool {
        !self.subject_id.trim().is_empty() && self.fingerprint_independent_of_id()
    }
}

/// One dimension's certification: the proof currency plus a reopenable evidence
/// object, so a grade is backed by an object a reviewer can reopen rather than an
/// asserted claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticDimensionCertification {
    /// Dimension being certified.
    pub dimension: DiagnosticEvidenceDimension,
    /// Currency of the proof backing this dimension.
    pub proof_currency: DiagnosticProofCurrency,
    /// Reopenable ref of the proof object. Present unless the proof is missing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_ref: Option<String>,
    /// Non-display fingerprint token of the proof object. Present iff `proof_ref` is
    /// present, and must differ from it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_fingerprint_token: Option<String>,
    /// Export-safe reviewable summary of the proof.
    pub summary: String,
}

impl DiagnosticDimensionCertification {
    /// Whether the proof object is reopenable: a present ref carries a distinct
    /// non-display fingerprint and a non-empty summary.
    pub fn proof_reopenable(&self) -> bool {
        match (&self.proof_ref, &self.proof_fingerprint_token) {
            (Some(reference), Some(fingerprint)) => {
                let reference = reference.trim();
                let fingerprint = fingerprint.trim();
                !reference.is_empty() && !fingerprint.is_empty() && fingerprint != reference
            }
            _ => false,
        }
    }

    /// Whether this certification is well-formed: a missing proof carries no ref, any
    /// other currency carries a reopenable proof, and the summary is present.
    pub fn is_well_formed(&self) -> bool {
        if self.summary.trim().is_empty() {
            return false;
        }
        if self.proof_currency.is_absent() {
            self.proof_ref.is_none() && self.proof_fingerprint_token.is_none()
        } else {
            self.proof_reopenable()
        }
    }

    /// Whether this certification backs a current claim for the given row imported
    /// posture. A local row needs locally verified or cached proof; an imported row
    /// needs current imported proof. Either way the proof must be reopenable.
    pub fn backs_claim(&self, imported_row: bool) -> bool {
        if !self.proof_reopenable() {
            return false;
        }
        if imported_row {
            self.proof_currency.is_imported_current()
        } else {
            self.proof_currency.is_current_local()
        }
    }
}

/// One claimed M5 diagnostic row certified against its evidence dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedDiagnosticRow {
    /// Stable row id.
    pub row_id: String,
    /// Kind of claimed M5 diagnostic row.
    pub row_kind: CertifiedDiagnosticRowKind,
    /// Durable subject the row certifies.
    pub subject: CertifiedDiagnosticSubject,
    /// Human-readable row label.
    pub label_summary: String,
    /// True when the row is imported / provider-backed and must never read as a live
    /// local rerun.
    pub imported_row: bool,
    /// Per-dimension certifications.
    pub certifications: Vec<DiagnosticDimensionCertification>,
    /// Whether display clustering preserves each finding's source kind rather than
    /// flattening unlike sources into a synthetic finding.
    pub source_kind_preserved: bool,
    /// Whether the imported-versus-live class survives every surface hop rather than
    /// being dropped to a generic provider name.
    pub imported_live_class_preserved: bool,
    /// Whether partial / streaming collection completeness stays visible rather than
    /// masquerading as a complete whole-workspace enumeration.
    pub collection_completeness_visible: bool,
    /// Whether anchor remap stays append-only evidence rather than being silently
    /// repaired or relabeled.
    pub remap_history_append_only: bool,
    /// Whether every mutating fix route serializes through the typed quality-action /
    /// session preview / apply / revert lifecycle.
    pub mutating_routes_use_quality_session: bool,
    /// Headline certification grade publicly claimed for this row.
    pub claimed_grade: DiagnosticCertificationGrade,
    /// Effective grade after auto-narrowing; equals the claim when every dimension is
    /// current, and ranks strictly below it otherwise.
    pub effective_grade: DiagnosticCertificationGrade,
    /// Trigger that fired the narrow, required when the row is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrow_trigger: Option<DiagnosticCertificationNarrowTrigger>,
    /// Precise narrowed label, required when the row is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowed_label: Option<String>,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl CertifiedDiagnosticRow {
    /// Dimensions certified by this row.
    pub fn certified_dimensions(&self) -> BTreeSet<DiagnosticEvidenceDimension> {
        self.certifications.iter().map(|c| c.dimension).collect()
    }

    /// Resolves a certification by dimension.
    pub fn certification(
        &self,
        dimension: DiagnosticEvidenceDimension,
    ) -> Option<&DiagnosticDimensionCertification> {
        self.certifications
            .iter()
            .find(|c| c.dimension == dimension)
    }

    /// Whether every required-core dimension is certified.
    pub fn has_all_required_core(&self) -> bool {
        let certified = self.certified_dimensions();
        DiagnosticEvidenceDimension::REQUIRED_CORE
            .iter()
            .all(|dimension| certified.contains(dimension))
    }

    /// Whether the row carries a public certification claim.
    pub fn is_claimed(&self) -> bool {
        self.claimed_grade.is_certified()
    }

    /// Whether every certified dimension backs a current claim for this row's
    /// imported posture.
    pub fn all_dimensions_current(&self) -> bool {
        self.certifications
            .iter()
            .all(|c| c.backs_claim(self.imported_row))
    }

    /// Whether the row must narrow below its claim because a required-core dimension
    /// is uncertified or any certified dimension lacks current proof.
    pub fn needs_narrow(&self) -> bool {
        !self.has_all_required_core() || !self.all_dimensions_current()
    }

    /// Whether the effective grade and narrow evidence are consistent.
    ///
    /// When every dimension is current the effective grade equals the claim;
    /// otherwise it must rank strictly below the claim and carry both a recorded
    /// trigger and a precise narrowed label.
    pub fn narrow_consistent(&self) -> bool {
        if self.needs_narrow() {
            self.effective_grade.rank() < self.claimed_grade.rank()
                && self.narrow_trigger.is_some()
                && self
                    .narrowed_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.effective_grade == self.claimed_grade
        }
    }

    /// Whether the imported posture is consistent: the row flag and its subject
    /// origin agree, so an imported row never reads as a local result.
    pub fn imported_posture_consistent(&self) -> bool {
        self.imported_row == self.subject.is_imported()
    }

    /// Whether every dimension required to record this row is present and its
    /// invariants hold.
    pub fn is_complete(&self) -> bool {
        !self.row_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.subject.is_valid()
            && !self.certifications.is_empty()
            && self
                .certifications
                .iter()
                .all(DiagnosticDimensionCertification::is_well_formed)
            && self.narrow_consistent()
            && self.imported_posture_consistent()
            && self.source_kind_preserved
            && self.imported_live_class_preserved
            && self.collection_completeness_visible
            && self.remap_history_append_only
            && self.mutating_routes_use_quality_session
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.source_contract_refs.is_empty()
            && self
                .source_contract_refs
                .iter()
                .all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticTruthCertificationGuardrails {
    /// Display clustering never erases a constituent's source provenance.
    pub display_clustering_never_erases_provenance: bool,
    /// Imported-versus-live class stays explicit across every surface.
    pub imported_versus_live_class_stays_explicit: bool,
    /// Freshness and remap states stay explicit rather than implied current.
    pub freshness_and_remap_states_stay_explicit: bool,
    /// Anchor remap is append-only evidence rather than a silent repair.
    pub anchor_remap_is_append_only_evidence: bool,
    /// Every mutating fix route is a typed quality-action proposal with preview.
    pub mutating_routes_are_typed_quality_proposals: bool,
    /// Any claimed row lacking current proof auto-narrows below its claim.
    pub rows_auto_narrow_without_current_proof: bool,
}

impl DiagnosticTruthCertificationGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.display_clustering_never_erases_provenance
            && self.imported_versus_live_class_stays_explicit
            && self.freshness_and_remap_states_stay_explicit
            && self.anchor_remap_is_append_only_evidence
            && self.mutating_routes_are_typed_quality_proposals
            && self.rows_auto_narrow_without_current_proof
    }
}

/// Consumer projection block: the surfaces that read this certification without
/// re-deriving diagnostic maturity by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticTruthCertificationConsumerProjection {
    /// Editor decorations ingest this certification.
    pub editor_ingests_certification: bool,
    /// The Problems surface ingests the same certification.
    pub problems_ingests_certification: bool,
    /// Review surfaces ingest the same certification.
    pub review_ingests_certification: bool,
    /// CLI / headless output ingests the same certification.
    pub cli_headless_ingests_certification: bool,
    /// Support / export ingests the same certification.
    pub support_export_ingests_certification: bool,
    /// AI evidence ingests the same certification.
    pub ai_evidence_ingests_certification: bool,
    /// Release-visible debt ingests the same certification.
    pub release_debt_ingests_certification: bool,
    /// Narrowed rows are visibly labeled below their claim in every surface.
    pub narrowed_rows_labeled_below_claim: bool,
}

impl DiagnosticTruthCertificationConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.editor_ingests_certification
            && self.problems_ingests_certification
            && self.review_ingests_certification
            && self.cli_headless_ingests_certification
            && self.support_export_ingests_certification
            && self.ai_evidence_ingests_certification
            && self.release_debt_ingests_certification
            && self.narrowed_rows_labeled_below_claim
    }
}

/// Evidence freshness block for the certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticTruthCertificationFreshness {
    /// Evidence-freshness SLO in hours.
    pub evidence_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last evidence refresh.
    pub last_evidence_refresh: String,
    /// True when stale evidence automatically narrows claimed rows.
    pub auto_narrow_on_stale: bool,
}

impl DiagnosticTruthCertificationFreshness {
    /// Whether the freshness block is well-formed.
    pub fn is_valid(&self) -> bool {
        self.evidence_freshness_slo_hours > 0 && !self.last_evidence_refresh.trim().is_empty()
    }
}

/// Constructor input for [`DiagnosticTruthCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticTruthCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub label: String,
    /// Per-row certifications.
    pub rows: Vec<CertifiedDiagnosticRow>,
    /// Guardrail invariants block.
    pub guardrails: DiagnosticTruthCertificationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: DiagnosticTruthCertificationConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: DiagnosticTruthCertificationFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe diagnostic-truth certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticTruthCertificationPacket {
    /// Record kind; must equal [`DIAGNOSTIC_TRUTH_CERT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DIAGNOSTIC_TRUTH_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub label: String,
    /// Per-row certifications.
    pub rows: Vec<CertifiedDiagnosticRow>,
    /// Guardrail invariants block.
    pub guardrails: DiagnosticTruthCertificationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: DiagnosticTruthCertificationConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: DiagnosticTruthCertificationFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl DiagnosticTruthCertificationPacket {
    /// Builds a diagnostic-truth certification packet.
    pub fn new(input: DiagnosticTruthCertificationPacketInput) -> Self {
        Self {
            record_kind: DIAGNOSTIC_TRUTH_CERT_RECORD_KIND.to_owned(),
            schema_version: DIAGNOSTIC_TRUTH_CERT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            evidence_freshness: input.evidence_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Row kinds represented by some row in this packet.
    pub fn represented_row_kinds(&self) -> BTreeSet<CertifiedDiagnosticRowKind> {
        self.rows.iter().map(|row| row.row_kind).collect()
    }

    /// Evidence dimensions certified by some row in this packet.
    pub fn represented_dimensions(&self) -> BTreeSet<DiagnosticEvidenceDimension> {
        self.rows
            .iter()
            .flat_map(|row| row.certified_dimensions())
            .collect()
    }

    /// Proof currencies represented across certifications.
    pub fn represented_currencies(&self) -> BTreeSet<DiagnosticProofCurrency> {
        self.rows
            .iter()
            .flat_map(|row| row.certifications.iter().map(|c| c.proof_currency))
            .collect()
    }

    /// Subject source kinds represented across rows.
    pub fn represented_source_kinds(&self) -> BTreeSet<DiagnosticSourceKind> {
        self.rows
            .iter()
            .map(|row| row.subject.source_kind)
            .collect()
    }

    /// Subject origin classes represented across rows.
    pub fn represented_origin_classes(&self) -> BTreeSet<DiagnosticOriginClass> {
        self.rows
            .iter()
            .map(|row| row.subject.origin_class)
            .collect()
    }

    /// Count of rows that auto-narrowed below their claim.
    pub fn narrowed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.needs_narrow()).count()
    }

    /// Count of rows holding a public certification claim.
    pub fn claimed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.is_claimed()).count()
    }

    /// Count of imported rows.
    pub fn imported_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.imported_row).count()
    }

    /// Rows that auto-narrowed below their claim, in packet order.
    pub fn narrowed_rows(&self) -> Vec<&CertifiedDiagnosticRow> {
        self.rows.iter().filter(|row| row.needs_narrow()).collect()
    }

    /// Resolves a row by its id.
    pub fn row(&self, row_id: &str) -> Option<&CertifiedDiagnosticRow> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }

    /// Validates the diagnostic-truth certification invariants.
    pub fn validate(&self) -> Vec<DiagnosticTruthCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != DIAGNOSTIC_TRUTH_CERT_RECORD_KIND {
            violations.push(DiagnosticTruthCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != DIAGNOSTIC_TRUTH_CERT_SCHEMA_VERSION {
            violations.push(DiagnosticTruthCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(DiagnosticTruthCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(DiagnosticTruthCertificationViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(DiagnosticTruthCertificationViolation::ConsumerProjectionIncomplete);
        }
        if !self.evidence_freshness.is_valid() {
            violations.push(DiagnosticTruthCertificationViolation::EvidenceFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("diagnostic truth certification packet serializes"),
        ) {
            violations.push(DiagnosticTruthCertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("diagnostic truth certification packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Diagnostic-Truth Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Rows: {} ({} claimed, {} imported, {} narrowed)\n",
            self.rows.len(),
            self.claimed_row_count(),
            self.imported_row_count(),
            self.narrowed_row_count()
        ));
        out.push_str(&format!(
            "- Row kinds: {} / {}\n",
            self.represented_row_kinds().len(),
            CertifiedDiagnosticRowKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Dimensions certified: {} / {}\n",
            self.represented_dimensions().len(),
            DiagnosticEvidenceDimension::ALL.len()
        ));
        out.push_str(&format!(
            "- Evidence freshness SLO: {} hours (last refresh: {})\n",
            self.evidence_freshness.evidence_freshness_slo_hours,
            self.evidence_freshness.last_evidence_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}): claim `{}` -> effective `{}`\n",
                row.row_id,
                row.row_kind.as_str(),
                row.claimed_grade.as_str(),
                row.effective_grade.as_str()
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!(
                "  - subject `{}` (source `{}`, origin `{}`), imported={}\n",
                row.subject.subject_id,
                row.subject.source_kind.as_str(),
                row.subject.origin_class.as_str(),
                row.imported_row
            ));
            for cert in &row.certifications {
                out.push_str(&format!(
                    "  - {} = `{}`\n",
                    cert.dimension.as_str(),
                    cert.proof_currency.as_str()
                ));
            }
            if let Some(label) = &row.narrowed_label {
                out.push_str(&format!("  - Narrowed: {label}\n"));
            }
        }
        out
    }

    /// Deterministic Markdown waiver-and-downgrade log: the release-visible record of
    /// every claimed row currently held below its claim, with the trigger and label
    /// that narrowed it. There are no manual waivers — auto-narrowing is the only
    /// mechanism by which a row sits below its claim.
    pub fn render_waiver_and_downgrade_log(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Diagnostic-Truth Waiver and Downgrade Log\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Generated from: `{}`\n",
            DIAGNOSTIC_TRUTH_CERT_ARTIFACT_REF
        ));
        out.push_str(&format!(
            "- Evidence freshness SLO: {} hours (last refresh: {})\n",
            self.evidence_freshness.evidence_freshness_slo_hours,
            self.evidence_freshness.last_evidence_refresh
        ));
        out.push_str(
            "\nNo manual waivers are granted: a diagnostic row sits below its claim only by \
             automatic narrowing when current, reopenable proof cannot back it.\n",
        );
        let narrowed = self.narrowed_rows();
        out.push_str(&format!(
            "\n## Auto-downgraded rows ({})\n\n",
            narrowed.len()
        ));
        if narrowed.is_empty() {
            out.push_str("None — every claimed row holds current proof for its claim.\n");
            return out;
        }
        for row in narrowed {
            out.push_str(&format!(
                "- **{}** ({}): claim `{}` -> effective `{}`\n",
                row.row_id,
                row.row_kind.as_str(),
                row.claimed_grade.as_str(),
                row.effective_grade.as_str()
            ));
            if let Some(trigger) = row.narrow_trigger {
                out.push_str(&format!("  - Trigger: `{}`\n", trigger.as_str()));
            }
            if let Some(label) = &row.narrowed_label {
                out.push_str(&format!("  - {label}\n"));
            }
            let uncurrent: Vec<&str> = row
                .certifications
                .iter()
                .filter(|c| !c.backs_claim(row.imported_row))
                .map(|c| c.dimension.as_str())
                .collect();
            if !uncurrent.is_empty() {
                out.push_str(&format!(
                    "  - Uncurrent dimensions: {}\n",
                    uncurrent.join(", ")
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum DiagnosticTruthCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<DiagnosticTruthCertificationViolation>),
}

impl fmt::Display for DiagnosticTruthCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "diagnostic truth certification export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "diagnostic truth certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DiagnosticTruthCertificationArtifactError {}

/// Validation failures emitted by [`DiagnosticTruthCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticTruthCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required claimed row kind is represented by no row.
    RequiredRowKindMissing,
    /// Some evidence dimension is certified by no row.
    DimensionCoverageMissing,
    /// Source-kind coverage does not prove unlike sources stay distinct.
    SourceKindCoverageMissing,
    /// Imported and live origins are not both represented.
    OriginCoverageMissing,
    /// No row demonstrates auto-narrowing on uncurrent proof.
    NarrowedRowCaseMissing,
    /// No row certifies current proof.
    CurrentProofCaseMissing,
    /// No imported row is present.
    ImportedRowCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A claimed row was not narrowed below its claim despite uncurrent proof.
    RowNotNarrowedOnUncurrentProof,
    /// A narrowed row lacks a precise narrowed label or trigger.
    NarrowedRowMissingLabelOrTrigger,
    /// A row's subject fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// Display clustering erased a finding's source kind.
    SourceKindErased,
    /// An imported row reads as a live local result.
    ImportedReadsAsLocal,
    /// A partial / streaming collection was hidden behind a green grade.
    CollectionCompletenessHidden,
    /// Anchor remap was silently repaired instead of recorded append-only.
    AnchorRemapNotAppendOnly,
    /// A mutating fix route bypassed the typed quality-action / session lifecycle.
    MutatingRouteBypassesQualitySession,
    /// A dimension proof is not reopenable (missing ref or fingerprint substitutes).
    DimensionProofNotReopenable,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Evidence freshness block is incomplete.
    EvidenceFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl DiagnosticTruthCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredRowKindMissing => "required_row_kind_missing",
            Self::DimensionCoverageMissing => "dimension_coverage_missing",
            Self::SourceKindCoverageMissing => "source_kind_coverage_missing",
            Self::OriginCoverageMissing => "origin_coverage_missing",
            Self::NarrowedRowCaseMissing => "narrowed_row_case_missing",
            Self::CurrentProofCaseMissing => "current_proof_case_missing",
            Self::ImportedRowCaseMissing => "imported_row_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::RowNotNarrowedOnUncurrentProof => "row_not_narrowed_on_uncurrent_proof",
            Self::NarrowedRowMissingLabelOrTrigger => "narrowed_row_missing_label_or_trigger",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::SourceKindErased => "source_kind_erased",
            Self::ImportedReadsAsLocal => "imported_reads_as_local",
            Self::CollectionCompletenessHidden => "collection_completeness_hidden",
            Self::AnchorRemapNotAppendOnly => "anchor_remap_not_append_only",
            Self::MutatingRouteBypassesQualitySession => "mutating_route_bypasses_quality_session",
            Self::DimensionProofNotReopenable => "dimension_proof_not_reopenable",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable packet export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_m5_diagnostic_truth_certification_export(
) -> Result<DiagnosticTruthCertificationPacket, DiagnosticTruthCertificationArtifactError> {
    let packet: DiagnosticTruthCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/m5/diagnostics/certification-report/support_export.json"
    )))
    .map_err(DiagnosticTruthCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DiagnosticTruthCertificationArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &DiagnosticTruthCertificationPacket,
    violations: &mut Vec<DiagnosticTruthCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        DIAGNOSTIC_TRUTH_CERT_SCHEMA_REF,
        DIAGNOSTIC_TRUTH_CERT_DOC_REF,
        DIAGNOSTIC_TRUTH_CERT_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(DiagnosticTruthCertificationViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &DiagnosticTruthCertificationPacket,
    violations: &mut Vec<DiagnosticTruthCertificationViolation>,
) {
    let row_kinds = packet.represented_row_kinds();
    for required in CertifiedDiagnosticRowKind::ALL {
        if !row_kinds.contains(&required) {
            violations.push(DiagnosticTruthCertificationViolation::RequiredRowKindMissing);
            break;
        }
    }

    let dimensions = packet.represented_dimensions();
    for required in DiagnosticEvidenceDimension::ALL {
        if !dimensions.contains(&required) {
            violations.push(DiagnosticTruthCertificationViolation::DimensionCoverageMissing);
            break;
        }
    }

    let source_kinds = packet.represented_source_kinds();
    if source_kinds.len() < 2 || !source_kinds.contains(&DiagnosticSourceKind::ScannerImport) {
        violations.push(DiagnosticTruthCertificationViolation::SourceKindCoverageMissing);
    }

    let origins = packet.represented_origin_classes();
    let has_imported = origins
        .iter()
        .any(|origin| origin.is_imported_or_replayed());
    let has_live = origins
        .iter()
        .any(|origin| !origin.is_imported_or_replayed());
    if !(has_imported && has_live) {
        violations.push(DiagnosticTruthCertificationViolation::OriginCoverageMissing);
    }

    if !packet
        .rows
        .iter()
        .any(|row| row.needs_narrow() && row.narrow_consistent())
    {
        violations.push(DiagnosticTruthCertificationViolation::NarrowedRowCaseMissing);
    }

    let currencies = packet.represented_currencies();
    if !currencies
        .iter()
        .any(|currency| currency.is_current_local() || currency.is_imported_current())
    {
        violations.push(DiagnosticTruthCertificationViolation::CurrentProofCaseMissing);
    }

    if packet.imported_row_count() == 0 {
        violations.push(DiagnosticTruthCertificationViolation::ImportedRowCaseMissing);
    }
}

fn validate_rows(
    packet: &DiagnosticTruthCertificationPacket,
    violations: &mut Vec<DiagnosticTruthCertificationViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(DiagnosticTruthCertificationViolation::RowIncomplete);
        }
        if row.needs_narrow() && row.effective_grade.rank() >= row.claimed_grade.rank() {
            violations.push(DiagnosticTruthCertificationViolation::RowNotNarrowedOnUncurrentProof);
        }
        if row.needs_narrow()
            && (row.narrow_trigger.is_none()
                || !row
                    .narrowed_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations
                .push(DiagnosticTruthCertificationViolation::NarrowedRowMissingLabelOrTrigger);
        }
        if !row.subject.fingerprint_independent_of_id() {
            violations.push(DiagnosticTruthCertificationViolation::FingerprintSubstitutesIdentity);
        }
        if !row.source_kind_preserved {
            violations.push(DiagnosticTruthCertificationViolation::SourceKindErased);
        }
        if !row.imported_live_class_preserved || !row.imported_posture_consistent() {
            violations.push(DiagnosticTruthCertificationViolation::ImportedReadsAsLocal);
        }
        if !row.collection_completeness_visible {
            violations.push(DiagnosticTruthCertificationViolation::CollectionCompletenessHidden);
        }
        if !row.remap_history_append_only {
            violations.push(DiagnosticTruthCertificationViolation::AnchorRemapNotAppendOnly);
        }
        if !row.mutating_routes_use_quality_session {
            violations
                .push(DiagnosticTruthCertificationViolation::MutatingRouteBypassesQualitySession);
        }
        if row.certifications.iter().any(|cert| !cert.is_well_formed()) {
            violations.push(DiagnosticTruthCertificationViolation::DimensionProofNotReopenable);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(DiagnosticTruthCertificationViolation::RowEvidenceMissing);
        }
    }
}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
///
/// A generic provider error must never stand in for a precise narrow truth.
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
            | "uncertified"
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
