//! Release-bearing certification of test discovery, session, watch, coverage,
//! flaky, snapshot, evidence-quality, and selector-portability proof on every
//! claimed M5 framework / notebook / CI-import test row.
//!
//! Where
//! [`crate::freeze_the_m5_test_item_discovery_snapshot_selection_object_and_session_attempt_quarantine_matrix`]
//! freezes *which* canonical object class each claimed test-intelligence surface
//! resolves to, this module certifies whether each claimed M5 **row** — a
//! framework-pack test, a notebook-backed test, an AI-test-generation row, a
//! review-panel row, or an imported-CI row — actually carries *current* evidence
//! for every dimension it claims. It is the capstone gate: a row may only keep its
//! certification grade if its discovery, session, watch, coverage, flaky, snapshot,
//! and selector-portability proof is present, reopenable, and inside its freshness
//! window. A row that loses current proof auto-narrows below its claim instead of
//! coasting on an adjacent green row.
//!
//! * a [`CertifiedTestRow`] ties a durable [`CertifiedSubject`] (keyed by a
//!   [`DurableTestNodeKind`] and a non-display fingerprint, so a parameterized
//!   template never collapses into a concrete invocation) to a list of
//!   [`DimensionCertification`] rows over the [`EvidenceDimension`] vocabulary, a
//!   claimed [`CertificationGrade`], an effective grade, and — when narrowed — a
//!   [`CertificationNarrowTrigger`] plus a precise narrowed label;
//! * each [`DimensionCertification`] is **evidence-bound, not asserted**: it names a
//!   [`ProofCurrency`] and, unless the proof is missing, a reopenable `proof_ref`
//!   keyed by a non-display fingerprint, so certification review can reopen the same
//!   discovery / session / watch / coverage / flaky / snapshot evidence object that
//!   backs the grade;
//! * the row **auto-narrows**: [`CertifiedTestRow::needs_narrow`] is true whenever a
//!   required core dimension is uncertified or any certified dimension lacks current
//!   proof (stale, missing, requires-review, or imported proof standing in for a
//!   local claim). A narrowed row must carry an effective grade strictly below its
//!   claim, a recorded trigger, and a precise label — never a generic non-answer.
//!
//! [`TestEvidenceCertificationPacket::validate`] also refuses a packet that lets a
//! parameterized template collapse into its concrete invocation, lets an imported CI
//! row read as a live local rerun, hides a quarantine or stale coverage behind a
//! generic green grade, or lets a test-generation / snapshot / golden proposal bypass
//! the same preview / diff / apply rules used elsewhere.
//!
//! Raw test source, raw provider payloads, raw log bytes, provider cursors,
//! credentials, and raw artifact bodies never cross this boundary; the packet carries
//! only typed class tokens, booleans, opaque ids, fingerprint digests, and
//! redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/testing/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality.schema.json`](../../../../schemas/testing/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality.schema.json).
//! The contract doc is
//! [`docs/testing/m5/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality.md`](../../../../docs/testing/m5/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality.md).
//! The protected fixture directory is
//! [`fixtures/testing/m5/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality/`](../../../../fixtures/testing/m5/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use crate::testing_identity::TestItemIdentityClass;

/// Stable record-kind tag carried by [`TestEvidenceCertificationPacket`].
pub const TEST_EVIDENCE_CERTIFICATION_RECORD_KIND: &str =
    "certify_test_discovery_session_watch_coverage_flaky_snapshot_evidence_quality_packet";

/// Schema version for the test-evidence certification packet.
pub const TEST_EVIDENCE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const TEST_EVIDENCE_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/testing/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality.schema.json";

/// Repo-relative path of the contract doc.
pub const TEST_EVIDENCE_CERTIFICATION_DOC_REF: &str =
    "docs/testing/m5/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality.md";

/// Repo-relative path of the checked support-export artifact.
pub const TEST_EVIDENCE_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/testing/m5/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const TEST_EVIDENCE_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/testing/m5/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality.md";

/// Repo-relative path of the protected fixture directory.
pub const TEST_EVIDENCE_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/testing/m5/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality";

/// Kind of claimed M5 test row a certification covers. Each kind is a distinct
/// claim surface that must carry its own current evidence rather than inheriting a
/// neighbour's grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedRowKind {
    /// A framework-pack test row (test explorer / test tree).
    FrameworkPackRow,
    /// A notebook-backed test row (notebook test cells).
    NotebookRow,
    /// An AI-assisted test-generation row.
    AiTestGenerationRow,
    /// A review / pull-request test-panel row.
    ReviewPanelRow,
    /// An imported-CI test row.
    CiImportRow,
}

impl CertifiedRowKind {
    /// Every row kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FrameworkPackRow,
        Self::NotebookRow,
        Self::AiTestGenerationRow,
        Self::ReviewPanelRow,
        Self::CiImportRow,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkPackRow => "framework_pack_row",
            Self::NotebookRow => "notebook_row",
            Self::AiTestGenerationRow => "ai_test_generation_row",
            Self::ReviewPanelRow => "review_panel_row",
            Self::CiImportRow => "ci_import_row",
        }
    }

    /// Whether this row kind is an imported / provider-backed CI row by nature.
    pub const fn is_imported_kind(self) -> bool {
        matches!(self, Self::CiImportRow)
    }
}

/// One evidence dimension a row is certified against. The first four are the
/// **required core** every claimed row must certify; the rest are quality
/// dimensions a row certifies only when it claims them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceDimension {
    /// Test discovery / partial-discovery truth.
    DiscoveryTruth,
    /// Session-plan and attempt-lineage truth.
    SessionTruth,
    /// Watch-mode rerun truth.
    WatchTruth,
    /// Selector / selection-object portability.
    SelectorPortability,
    /// Coverage overlay / merge evidence quality.
    CoverageEvidence,
    /// Flaky / quarantine evidence quality.
    FlakyEvidence,
    /// Snapshot / golden / baseline evidence quality.
    SnapshotEvidence,
}

impl EvidenceDimension {
    /// Every evidence dimension, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DiscoveryTruth,
        Self::SessionTruth,
        Self::WatchTruth,
        Self::SelectorPortability,
        Self::CoverageEvidence,
        Self::FlakyEvidence,
        Self::SnapshotEvidence,
    ];

    /// The required-core dimensions every claimed row must certify.
    pub const REQUIRED_CORE: [Self; 4] = [
        Self::DiscoveryTruth,
        Self::SessionTruth,
        Self::WatchTruth,
        Self::SelectorPortability,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiscoveryTruth => "discovery_truth",
            Self::SessionTruth => "session_truth",
            Self::WatchTruth => "watch_truth",
            Self::SelectorPortability => "selector_portability",
            Self::CoverageEvidence => "coverage_evidence",
            Self::FlakyEvidence => "flaky_evidence",
            Self::SnapshotEvidence => "snapshot_evidence",
        }
    }

    /// Whether this dimension is part of the required core.
    pub const fn is_core(self) -> bool {
        matches!(
            self,
            Self::DiscoveryTruth
                | Self::SessionTruth
                | Self::WatchTruth
                | Self::SelectorPortability
        )
    }
}

/// Currency of the proof backing one dimension certification. Only a current,
/// reopenable proof backs a claim; a stale, missing, imported-on-local, or
/// review-pending proof auto-narrows the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofCurrency {
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

impl ProofCurrency {
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

/// Certification grade a row claims or effectively holds. Higher [`Self::rank`] is
/// a stronger claim, so a narrowed row must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationGrade {
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

impl CertificationGrade {
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

/// Reason a claimed row auto-narrowed below its claim. The chrome quotes the
/// trigger verbatim instead of a generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationNarrowTrigger {
    /// A required-core dimension carries no certification at all.
    MissingDimensionProof,
    /// A certified dimension's proof aged outside its freshness window.
    StaleDimensionProof,
    /// A local row leaned on imported / provider proof to back a local claim.
    ImportedProofOnLocalRow,
    /// A provider verdict still requires review and fails closed.
    VerdictRequiresReview,
    /// Partial discovery could not be certified current.
    PartialDiscoveryUncertified,
    /// Quarantine / coverage debt could not be certified current.
    QuarantineDebtUncertified,
    /// Selector portability could not be proven.
    SelectorPortabilityUnproven,
    /// An upstream dependency narrowed and dragged this row down with it.
    UpstreamDependencyNarrowed,
}

impl CertificationNarrowTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingDimensionProof => "missing_dimension_proof",
            Self::StaleDimensionProof => "stale_dimension_proof",
            Self::ImportedProofOnLocalRow => "imported_proof_on_local_row",
            Self::VerdictRequiresReview => "verdict_requires_review",
            Self::PartialDiscoveryUncertified => "partial_discovery_uncertified",
            Self::QuarantineDebtUncertified => "quarantine_debt_uncertified",
            Self::SelectorPortabilityUnproven => "selector_portability_unproven",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Durable subject of a certified row, keyed by a node kind and a non-display
/// fingerprint distinct from its id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedSubject {
    /// Durable node id of the certified row's subject.
    pub subject_id: String,
    /// Node kind, reusing the frozen durable-discovery vocabulary so a parameterized
    /// template never collapses into a concrete invocation.
    pub node_kind: DurableTestNodeKind,
    /// Non-display fingerprint token. Must differ from
    /// [`subject_id`](CertifiedSubject::subject_id).
    pub subject_fingerprint_token: String,
    /// Identity stability, reusing the frozen identity vocabulary.
    pub identity_class: TestItemIdentityClass,
}

impl CertifiedSubject {
    /// Whether this subject is imported / provider-owned and read-only.
    pub fn is_imported(&self) -> bool {
        self.identity_class == TestItemIdentityClass::ImportedReadOnly
    }

    /// Whether the fingerprint is a real non-display basis distinct from the id.
    pub fn fingerprint_independent_of_id(&self) -> bool {
        let token = self.subject_fingerprint_token.trim();
        !token.is_empty() && token != self.subject_id.trim()
    }

    /// Whether the subject carries the durable identity a reopen needs.
    pub fn is_valid(&self) -> bool {
        !self.subject_id.trim().is_empty()
            && self.fingerprint_independent_of_id()
            && self.identity_class != TestItemIdentityClass::DisplayTextOnlyDenied
    }
}

/// One dimension's certification: the proof currency plus a reopenable evidence
/// object, so a grade is backed by an object a reviewer can reopen rather than an
/// asserted claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionCertification {
    /// Dimension being certified.
    pub dimension: EvidenceDimension,
    /// Currency of the proof backing this dimension.
    pub proof_currency: ProofCurrency,
    /// Reopenable ref of the proof object. Present unless the proof is missing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_ref: Option<String>,
    /// Non-display fingerprint token of the proof object. Present iff `proof_ref`
    /// is present, and must differ from it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_fingerprint_token: Option<String>,
    /// Export-safe reviewable summary of the proof.
    pub summary: String,
}

impl DimensionCertification {
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

    /// Whether this certification is well-formed: a missing proof carries no ref,
    /// any other currency carries a reopenable proof, and the summary is present.
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

/// One claimed M5 test row certified against its evidence dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedTestRow {
    /// Stable row id.
    pub row_id: String,
    /// Kind of claimed M5 row.
    pub row_kind: CertifiedRowKind,
    /// Durable subject the row certifies.
    pub subject: CertifiedSubject,
    /// Human-readable row label.
    pub label_summary: String,
    /// True when the row is imported / provider-backed and must never read as a
    /// live local rerun.
    pub imported_row: bool,
    /// Per-dimension certifications.
    pub certifications: Vec<DimensionCertification>,
    /// Whether a parameterized template stays distinct from its concrete
    /// invocations rather than collapsing into one row identity.
    pub template_distinct_from_invocation: bool,
    /// Whether partial / streaming discovery keeps its uncovered scope visible.
    pub partial_discovery_visible: bool,
    /// Whether quarantine / mute state stays visible and exportable rather than
    /// hidden behind a generic green grade.
    pub quarantine_visible_and_exportable: bool,
    /// Whether every snapshot / golden / test-generation proposal uses the shared
    /// preview / diff / apply rules.
    pub proposals_use_preview_apply: bool,
    /// Headline certification grade publicly claimed for this row.
    pub claimed_grade: CertificationGrade,
    /// Effective grade after auto-narrowing; equals the claim when every dimension
    /// is current, and ranks strictly below it otherwise.
    pub effective_grade: CertificationGrade,
    /// Trigger that fired the narrow, required when the row is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrow_trigger: Option<CertificationNarrowTrigger>,
    /// Precise narrowed label, required when the row is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowed_label: Option<String>,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl CertifiedTestRow {
    /// Dimensions certified by this row.
    pub fn certified_dimensions(&self) -> BTreeSet<EvidenceDimension> {
        self.certifications.iter().map(|c| c.dimension).collect()
    }

    /// Resolves a certification by dimension.
    pub fn certification(&self, dimension: EvidenceDimension) -> Option<&DimensionCertification> {
        self.certifications
            .iter()
            .find(|c| c.dimension == dimension)
    }

    /// Whether every required-core dimension is certified.
    pub fn has_all_required_core(&self) -> bool {
        let certified = self.certified_dimensions();
        EvidenceDimension::REQUIRED_CORE
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

    /// Whether the row must narrow below its claim because a required-core
    /// dimension is uncertified or any certified dimension lacks current proof.
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
    /// identity agree, so an imported row never reads as a local result.
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
                .all(DimensionCertification::is_well_formed)
            && self.narrow_consistent()
            && self.imported_posture_consistent()
            && self.template_distinct_from_invocation
            && self.partial_discovery_visible
            && self.quarantine_visible_and_exportable
            && self.proposals_use_preview_apply
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
pub struct TestEvidenceCertificationGuardrails {
    /// Parameterized templates stay distinct from their concrete invocations.
    pub parameterized_templates_distinct_from_invocations: bool,
    /// Imported CI rows never read as a live local rerun.
    pub imported_ci_never_reads_as_local_rerun: bool,
    /// Quarantine and stale coverage never hide behind a generic green grade.
    pub quarantines_and_stale_coverage_never_hidden: bool,
    /// Snapshot / golden / test-generation proposals use preview / diff / apply.
    pub proposals_use_preview_diff_apply: bool,
    /// Any claimed row lacking current proof auto-narrows below its claim.
    pub rows_auto_narrow_without_current_proof: bool,
}

impl TestEvidenceCertificationGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.parameterized_templates_distinct_from_invocations
            && self.imported_ci_never_reads_as_local_rerun
            && self.quarantines_and_stale_coverage_never_hidden
            && self.proposals_use_preview_diff_apply
            && self.rows_auto_narrow_without_current_proof
    }
}

/// Consumer projection block: the surfaces that read this certification without
/// re-deriving test maturity by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestEvidenceCertificationConsumerProjection {
    /// Product test-intelligence surfaces ingest this certification.
    pub product_ingests_certification: bool,
    /// Docs / help ingests the same certification.
    pub docs_help_ingests_certification: bool,
    /// Review surfaces ingest the same certification.
    pub review_ingests_certification: bool,
    /// Support / export ingests the same certification.
    pub support_ingests_certification: bool,
    /// Release-control surfaces ingest the same certification.
    pub release_control_ingests_certification: bool,
    /// Narrowed rows are visibly labeled below their claim in every surface.
    pub narrowed_rows_labeled_below_claim: bool,
}

impl TestEvidenceCertificationConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_certification
            && self.docs_help_ingests_certification
            && self.review_ingests_certification
            && self.support_ingests_certification
            && self.release_control_ingests_certification
            && self.narrowed_rows_labeled_below_claim
    }
}

/// Evidence freshness block for the certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestEvidenceCertificationFreshness {
    /// Evidence-freshness SLO in hours.
    pub evidence_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last evidence refresh.
    pub last_evidence_refresh: String,
    /// True when stale evidence automatically narrows claimed rows.
    pub auto_narrow_on_stale: bool,
}

impl TestEvidenceCertificationFreshness {
    /// Whether the freshness block is well-formed.
    pub fn is_valid(&self) -> bool {
        self.evidence_freshness_slo_hours > 0 && !self.last_evidence_refresh.trim().is_empty()
    }
}

/// Constructor input for [`TestEvidenceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestEvidenceCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub label: String,
    /// Per-row certifications.
    pub rows: Vec<CertifiedTestRow>,
    /// Guardrail invariants block.
    pub guardrails: TestEvidenceCertificationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: TestEvidenceCertificationConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: TestEvidenceCertificationFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe test-evidence certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestEvidenceCertificationPacket {
    /// Record kind; must equal [`TEST_EVIDENCE_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`TEST_EVIDENCE_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub label: String,
    /// Per-row certifications.
    pub rows: Vec<CertifiedTestRow>,
    /// Guardrail invariants block.
    pub guardrails: TestEvidenceCertificationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: TestEvidenceCertificationConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: TestEvidenceCertificationFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl TestEvidenceCertificationPacket {
    /// Builds a test-evidence certification packet.
    pub fn new(input: TestEvidenceCertificationPacketInput) -> Self {
        Self {
            record_kind: TEST_EVIDENCE_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: TEST_EVIDENCE_CERTIFICATION_SCHEMA_VERSION,
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
    pub fn represented_row_kinds(&self) -> BTreeSet<CertifiedRowKind> {
        self.rows.iter().map(|row| row.row_kind).collect()
    }

    /// Evidence dimensions certified by some row in this packet.
    pub fn represented_dimensions(&self) -> BTreeSet<EvidenceDimension> {
        self.rows
            .iter()
            .flat_map(|row| row.certified_dimensions())
            .collect()
    }

    /// Proof currencies represented across certifications.
    pub fn represented_currencies(&self) -> BTreeSet<ProofCurrency> {
        self.rows
            .iter()
            .flat_map(|row| row.certifications.iter().map(|c| c.proof_currency))
            .collect()
    }

    /// Subject node kinds represented across rows.
    pub fn represented_subject_kinds(&self) -> BTreeSet<DurableTestNodeKind> {
        self.rows.iter().map(|row| row.subject.node_kind).collect()
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

    /// Resolves a row by its id.
    pub fn row(&self, row_id: &str) -> Option<&CertifiedTestRow> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }

    /// Validates the test-evidence certification invariants.
    pub fn validate(&self) -> Vec<TestEvidenceCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != TEST_EVIDENCE_CERTIFICATION_RECORD_KIND {
            violations.push(TestEvidenceCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != TEST_EVIDENCE_CERTIFICATION_SCHEMA_VERSION {
            violations.push(TestEvidenceCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(TestEvidenceCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(TestEvidenceCertificationViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(TestEvidenceCertificationViolation::ConsumerProjectionIncomplete);
        }
        if !self.evidence_freshness.is_valid() {
            violations.push(TestEvidenceCertificationViolation::EvidenceFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("test evidence certification packet serializes"),
        ) {
            violations.push(TestEvidenceCertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("test evidence certification packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Test Discovery / Session / Watch / Coverage / Flaky / Snapshot Certification\n\n",
        );
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
            CertifiedRowKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Dimensions certified: {} / {}\n",
            self.represented_dimensions().len(),
            EvidenceDimension::ALL.len()
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
                "  - subject `{}` ({}), imported={}\n",
                row.subject.subject_id,
                row.subject.node_kind.as_str(),
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
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum TestEvidenceCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<TestEvidenceCertificationViolation>),
}

impl fmt::Display for TestEvidenceCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "test evidence certification export parse failed: {error}"
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
                    "test evidence certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for TestEvidenceCertificationArtifactError {}

/// Validation failures emitted by [`TestEvidenceCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestEvidenceCertificationViolation {
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
    /// A parameterized template was collapsed into its concrete invocation.
    TemplateCollapsedWithInvocation,
    /// Partial / streaming discovery was hidden.
    PartialDiscoveryHidden,
    /// An imported row reads as a live local result.
    ImportedReadsAsLocal,
    /// A quarantine / stale-coverage state was hidden behind a green grade.
    QuarantineHidden,
    /// A snapshot / golden / test-generation proposal bypassed preview / apply.
    ProposalBypassesPreview,
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

impl TestEvidenceCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredRowKindMissing => "required_row_kind_missing",
            Self::DimensionCoverageMissing => "dimension_coverage_missing",
            Self::NarrowedRowCaseMissing => "narrowed_row_case_missing",
            Self::CurrentProofCaseMissing => "current_proof_case_missing",
            Self::ImportedRowCaseMissing => "imported_row_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::RowNotNarrowedOnUncurrentProof => "row_not_narrowed_on_uncurrent_proof",
            Self::NarrowedRowMissingLabelOrTrigger => "narrowed_row_missing_label_or_trigger",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::TemplateCollapsedWithInvocation => "template_collapsed_with_invocation",
            Self::PartialDiscoveryHidden => "partial_discovery_hidden",
            Self::ImportedReadsAsLocal => "imported_reads_as_local",
            Self::QuarantineHidden => "quarantine_hidden",
            Self::ProposalBypassesPreview => "proposal_bypasses_preview",
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
pub fn current_test_evidence_certification_export(
) -> Result<TestEvidenceCertificationPacket, TestEvidenceCertificationArtifactError> {
    let packet: TestEvidenceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/testing/m5/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality/support_export.json"
    )))
    .map_err(TestEvidenceCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(TestEvidenceCertificationArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &TestEvidenceCertificationPacket,
    violations: &mut Vec<TestEvidenceCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        TEST_EVIDENCE_CERTIFICATION_SCHEMA_REF,
        TEST_EVIDENCE_CERTIFICATION_DOC_REF,
        TEST_EVIDENCE_CERTIFICATION_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(TestEvidenceCertificationViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &TestEvidenceCertificationPacket,
    violations: &mut Vec<TestEvidenceCertificationViolation>,
) {
    let row_kinds = packet.represented_row_kinds();
    for required in CertifiedRowKind::ALL {
        if !row_kinds.contains(&required) {
            violations.push(TestEvidenceCertificationViolation::RequiredRowKindMissing);
            break;
        }
    }

    let dimensions = packet.represented_dimensions();
    for required in EvidenceDimension::ALL {
        if !dimensions.contains(&required) {
            violations.push(TestEvidenceCertificationViolation::DimensionCoverageMissing);
            break;
        }
    }

    if !packet
        .rows
        .iter()
        .any(|row| row.needs_narrow() && row.narrow_consistent())
    {
        violations.push(TestEvidenceCertificationViolation::NarrowedRowCaseMissing);
    }

    let currencies = packet.represented_currencies();
    if !currencies
        .iter()
        .any(|currency| currency.is_current_local() || currency.is_imported_current())
    {
        violations.push(TestEvidenceCertificationViolation::CurrentProofCaseMissing);
    }

    if packet.imported_row_count() == 0 {
        violations.push(TestEvidenceCertificationViolation::ImportedRowCaseMissing);
    }

    let subject_kinds = packet.represented_subject_kinds();
    if !(subject_kinds.contains(&DurableTestNodeKind::ParameterizedTemplate)
        && subject_kinds.contains(&DurableTestNodeKind::ConcreteInvocation))
    {
        violations.push(TestEvidenceCertificationViolation::TemplateCollapsedWithInvocation);
    }
}

fn validate_rows(
    packet: &TestEvidenceCertificationPacket,
    violations: &mut Vec<TestEvidenceCertificationViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(TestEvidenceCertificationViolation::RowIncomplete);
        }
        if row.needs_narrow() && row.effective_grade.rank() >= row.claimed_grade.rank() {
            violations.push(TestEvidenceCertificationViolation::RowNotNarrowedOnUncurrentProof);
        }
        if row.needs_narrow()
            && (row.narrow_trigger.is_none()
                || !row
                    .narrowed_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations.push(TestEvidenceCertificationViolation::NarrowedRowMissingLabelOrTrigger);
        }
        if !row.subject.fingerprint_independent_of_id() {
            violations.push(TestEvidenceCertificationViolation::FingerprintSubstitutesIdentity);
        }
        if !row.template_distinct_from_invocation {
            violations.push(TestEvidenceCertificationViolation::TemplateCollapsedWithInvocation);
        }
        if !row.partial_discovery_visible {
            violations.push(TestEvidenceCertificationViolation::PartialDiscoveryHidden);
        }
        if !row.imported_posture_consistent() {
            violations.push(TestEvidenceCertificationViolation::ImportedReadsAsLocal);
        }
        if !row.quarantine_visible_and_exportable {
            violations.push(TestEvidenceCertificationViolation::QuarantineHidden);
        }
        if !row.proposals_use_preview_apply {
            violations.push(TestEvidenceCertificationViolation::ProposalBypassesPreview);
        }
        if row.certifications.iter().any(|cert| !cert.is_well_formed()) {
            violations.push(TestEvidenceCertificationViolation::DimensionProofNotReopenable);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(TestEvidenceCertificationViolation::RowEvidenceMissing);
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
