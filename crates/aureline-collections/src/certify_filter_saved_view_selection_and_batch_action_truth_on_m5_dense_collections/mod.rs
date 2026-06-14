//! Release-bearing certification of filter, saved-view, result-count, selection,
//! and batch-action truth on every claimed M5 dense collection surface.
//!
//! Where
//! [`crate::freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix`]
//! froze the per-surface qualification *matrix* (the filter-AST, selection-scope,
//! result-counter, and batch-action classes a claimed surface declares), this
//! module is the release gate that makes those claims *bearing*. For each claimed
//! M5 dense collection surface it answers one question: "does this row carry
//! current proof of filter-AST, saved-view, result-count, selection-scope, and
//! batch-action truth — and if not, has it visibly narrowed below its claim or
//! blocked promotion?"
//!
//! A claimed row that cannot present current proof on every required dimension
//! **auto-narrows**: its certified qualification ranks strictly below its claim,
//! it records which dimension is missing or stale, and it carries a precise
//! narrowed label. A row that regresses a release-gating invariant — erasing the
//! hidden-selected count, losing stale-snapshot review, hiding provider or policy
//! narrowing, blurring visible versus all-matching counts, bypassing batch
//! preview, or letting a row highlight stand in for durable selection — is
//! **blocked**: promotion is refused and the row is visibly narrowed rather than
//! shipped on an unbacked claim.
//!
//! The certification reuses the frozen
//! [`DenseCollectionSurface`](crate::freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix::DenseCollectionSurface)
//! and
//! [`CollectionMatrixQualificationClass`](crate::freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix::CollectionMatrixQualificationClass)
//! vocabularies rather than minting synonyms, and adds the dimensions this gate
//! owns: [`CertificationProofDimension`], [`ProofStatus`], [`CertificationVerdict`],
//! and [`CertificationRegressionClass`].
//!
//! Product, docs/help, accessibility, and release-control surfaces ingest this one
//! certification result instead of narrating dense-collection maturity by hand;
//! narrowed and blocked rows are labeled below current in every consumer surface.
//!
//! Raw query text, raw filter literal bytes, provider cursors, credentials, and
//! raw row bodies never cross this boundary; the packet carries only typed class
//! tokens, booleans, opaque ids, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/collections/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle.schema.json`](../../../../schemas/collections/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle.schema.json).
//! The contract doc is
//! [`docs/collections/m5/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle.md`](../../../../docs/collections/m5/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle.md).
//! The protected fixture directory is
//! [`fixtures/collections/m5/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle/`](../../../../fixtures/collections/m5/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix::{
    CollectionMatrixQualificationClass, DenseCollectionSurface,
};

/// Stable record-kind tag carried by [`M5CollectionCertificationPacket`].
pub const M5_COLLECTION_CERTIFICATION_RECORD_KIND: &str =
    "certify_filter_saved_view_selection_and_batch_action_truth_on_m5_dense_collections";

/// Schema version for the dense-collection certification packet.
pub const M5_COLLECTION_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_COLLECTION_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/collections/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_COLLECTION_CERTIFICATION_DOC_REF: &str =
    "docs/collections/m5/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_COLLECTION_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/collections/m5/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COLLECTION_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/collections/m5/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_COLLECTION_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/collections/m5/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle.md";

/// Closed proof-dimension vocabulary. Each claimed M5 dense collection row must
/// present current proof on every dimension before its claim is certified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationProofDimension {
    /// The filter-AST / saved-query grammar that scopes the surface is canonical.
    FilterAst,
    /// The saved-view and column-preset persistence contract is canonical.
    SavedView,
    /// The visible / loaded / matching / selected result-count truth is canonical.
    ResultCount,
    /// Durable selection scope by stable identity is canonical.
    SelectionScope,
    /// Batch-action scope, preview, and scope-receipt truth is canonical.
    BatchAction,
}

impl CertificationProofDimension {
    /// Every proof dimension a claimed row must demonstrate, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FilterAst,
        Self::SavedView,
        Self::ResultCount,
        Self::SelectionScope,
        Self::BatchAction,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FilterAst => "filter_ast",
            Self::SavedView => "saved_view",
            Self::ResultCount => "result_count",
            Self::SelectionScope => "selection_scope",
            Self::BatchAction => "batch_action",
        }
    }
}

/// Freshness of the proof a row presents for one dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofStatus {
    /// Proof is present and within the evidence-freshness SLO.
    Current,
    /// Proof is present but stale; the row must narrow until it refreshes.
    Stale,
    /// No proof is present for this dimension; the row must narrow.
    Missing,
}

impl ProofStatus {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }

    /// Whether this status certifies the dimension; only [`ProofStatus::Current`]
    /// does.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// Proof a row presents for one [`CertificationProofDimension`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationProof {
    /// Dimension this proof covers.
    pub dimension: CertificationProofDimension,
    /// Freshness of the proof.
    pub status: ProofStatus,
    /// Canonical packet record-kind that backs this dimension, e.g. the result
    /// scope counter, selection bar, or batch review sheet record kind. `None`
    /// only when the proof is [`ProofStatus::Missing`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backing_record_kind: Option<String>,
    /// Evidence ref backing this dimension. `None` only when the proof is missing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_ref: Option<String>,
}

impl CertificationProof {
    /// Whether this proof is internally consistent: a current or stale proof names
    /// a backing record kind and an evidence ref; a missing proof names neither.
    pub fn is_consistent(&self) -> bool {
        match self.status {
            ProofStatus::Current | ProofStatus::Stale => {
                self.backing_record_kind
                    .as_ref()
                    .is_some_and(|kind| !kind.trim().is_empty())
                    && self
                        .proof_ref
                        .as_ref()
                        .is_some_and(|r| !r.trim().is_empty())
            }
            ProofStatus::Missing => true,
        }
    }
}

/// Closed verdict vocabulary the gate assigns to each claimed row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationVerdict {
    /// Every required dimension proves current; the claim holds as certified.
    Certified,
    /// At least one dimension is missing or stale; the claim is auto-narrowed
    /// below its level until proof refreshes.
    AutoNarrowed,
    /// A release-gating invariant regressed; promotion is blocked and the row is
    /// visibly narrowed rather than shipped on an unbacked claim.
    Blocked,
}

impl CertificationVerdict {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::AutoNarrowed => "auto_narrowed",
            Self::Blocked => "blocked",
        }
    }
}

/// Closed regression-class vocabulary. Names the release-gating invariant a
/// blocked row regressed; the chrome quotes the class verbatim instead of a
/// generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationRegressionClass {
    /// The hidden-selected count was erased, blurring how many selected members
    /// are offscreen.
    HiddenSelectedCountErased,
    /// Stale-snapshot review was lost, so a stale saved query reopens as fresh.
    StaleSnapshotReviewLost,
    /// Provider or policy narrowing was hidden inside a generic filter chip.
    ProviderPolicyNarrowingErased,
    /// The visible count was blurred into the all-matching count without an
    /// explicit step.
    VisibleVersusMatchingBlurred,
    /// A broad batch action bypassed preview because the list is virtualized or
    /// provider-backed.
    BatchPreviewBypassed,
    /// A transient row highlight was allowed to stand in for durable selection.
    SelectionDurabilityLost,
}

impl CertificationRegressionClass {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HiddenSelectedCountErased => "hidden_selected_count_erased",
            Self::StaleSnapshotReviewLost => "stale_snapshot_review_lost",
            Self::ProviderPolicyNarrowingErased => "provider_policy_narrowing_erased",
            Self::VisibleVersusMatchingBlurred => "visible_versus_matching_blurred",
            Self::BatchPreviewBypassed => "batch_preview_bypassed",
            Self::SelectionDurabilityLost => "selection_durability_lost",
        }
    }
}

/// One claimed M5 dense collection row the certification gate covers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationRow {
    /// Stable row id.
    pub row_id: String,
    /// Claimed dense collection surface.
    pub surface: DenseCollectionSurface,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Ref to the qualification-matrix row this certification gates.
    pub matrix_row_ref: String,
    /// Headline qualification publicly claimed for this row.
    pub claimed_qualification: CollectionMatrixQualificationClass,
    /// Certified qualification after the gate runs; equals the claim when every
    /// proof is current, and ranks strictly below it when narrowed or blocked.
    pub certified_qualification: CollectionMatrixQualificationClass,
    /// Per-dimension proofs this row presents.
    pub proofs: Vec<CertificationProof>,
    /// Verdict the gate assigned.
    pub verdict: CertificationVerdict,
    /// Regression class, required when the verdict is [`CertificationVerdict::Blocked`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regression: Option<CertificationRegressionClass>,
    /// Precise narrowed label, required when the row is narrowed or blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowed_label: Option<String>,
    /// True when selection survives sort / filter / virtualization by stable
    /// identity rather than transient row highlight.
    pub selection_survives_by_stable_identity: bool,
    /// True when provider or policy narrowing is disclosed instead of hidden in a
    /// generic filter chip.
    pub provider_policy_narrowing_disclosed: bool,
    /// True when the visible row count is kept distinct from the all-matching
    /// count, so expanding to all matching requires an explicit step.
    pub visible_distinct_from_all_matching: bool,
    /// True when every broad batch action previews its scope before commit.
    pub broad_actions_preview_before_commit: bool,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl CertificationRow {
    /// Whether this row carries a public claim.
    pub fn is_claimed(&self) -> bool {
        self.claimed_qualification.is_claimed()
    }

    /// Proof this row presents for `dimension`, if any.
    pub fn proof_for(&self, dimension: CertificationProofDimension) -> Option<&CertificationProof> {
        self.proofs
            .iter()
            .find(|proof| proof.dimension == dimension)
    }

    /// Whether every required dimension is represented exactly once.
    pub fn proof_dimensions_complete(&self) -> bool {
        let present: BTreeSet<_> = self.proofs.iter().map(|proof| proof.dimension).collect();
        present.len() == self.proofs.len()
            && CertificationProofDimension::ALL
                .iter()
                .all(|dimension| present.contains(dimension))
    }

    /// Whether every required dimension proves current.
    pub fn all_proofs_current(&self) -> bool {
        CertificationProofDimension::ALL.iter().all(|dimension| {
            self.proof_for(*dimension)
                .is_some_and(|proof| proof.status.is_current())
        })
    }

    /// Whether any required dimension is missing or stale.
    pub fn has_uncovered_dimension(&self) -> bool {
        !self.all_proofs_current()
    }

    /// Whether the release-gating invariants hold for this row.
    pub fn release_invariants_hold(&self) -> bool {
        self.selection_survives_by_stable_identity
            && self.provider_policy_narrowing_disclosed
            && self.visible_distinct_from_all_matching
            && self.broad_actions_preview_before_commit
    }

    /// Whether the verdict, certified qualification, and narrowing evidence are
    /// internally consistent.
    ///
    /// A blocked row must record a regression class, rank strictly below its
    /// claim, and carry a precise narrowed label. An auto-narrowed row must have
    /// an uncovered dimension, rank strictly below its claim, and carry a precise
    /// narrowed label. A certified row must prove every dimension current, hold
    /// every release invariant, and keep its certified qualification at its claim.
    pub fn verdict_consistent(&self) -> bool {
        match self.verdict {
            CertificationVerdict::Blocked => {
                self.regression.is_some()
                    && self.certified_qualification.rank() < self.claimed_qualification.rank()
                    && self
                        .narrowed_label
                        .as_ref()
                        .is_some_and(|label| !label_is_generic(label))
            }
            CertificationVerdict::AutoNarrowed => {
                self.regression.is_none()
                    && self.release_invariants_hold()
                    && self.has_uncovered_dimension()
                    && self.certified_qualification.rank() < self.claimed_qualification.rank()
                    && self
                        .narrowed_label
                        .as_ref()
                        .is_some_and(|label| !label_is_generic(label))
            }
            CertificationVerdict::Certified => {
                self.regression.is_none()
                    && self.release_invariants_hold()
                    && self.all_proofs_current()
                    && self.certified_qualification == self.claimed_qualification
            }
        }
    }

    /// Whether every dimension required to record this row is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.row_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.matrix_row_ref.trim().is_empty()
            && self.proof_dimensions_complete()
            && self.proofs.iter().all(CertificationProof::is_consistent)
            && self.verdict_consistent()
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
pub struct CertificationGuardrails {
    /// Durable selection survives by stable identity; a row highlight never
    /// stands in for it.
    pub row_highlight_never_substitutes_durable_selection: bool,
    /// Provider or policy narrowing is always visible, never hidden in a generic
    /// filter chip.
    pub provider_policy_narrowing_never_hidden: bool,
    /// Visible rows are never treated as all matching rows without an explicit step.
    pub visible_never_all_matching_without_explicit_step: bool,
    /// Broad batch actions never bypass preview because the list is virtualized or
    /// provider-backed.
    pub broad_actions_never_bypass_preview: bool,
    /// Any claimed row without current proof auto-narrows below its claim.
    pub rows_without_current_proof_auto_narrow: bool,
    /// Regressions either block promotion or visibly narrow the affected claim.
    pub regressions_block_or_visibly_narrow: bool,
}

impl CertificationGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.row_highlight_never_substitutes_durable_selection
            && self.provider_policy_narrowing_never_hidden
            && self.visible_never_all_matching_without_explicit_step
            && self.broad_actions_never_bypass_preview
            && self.rows_without_current_proof_auto_narrow
            && self.regressions_block_or_visibly_narrow
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationConsumerProjection {
    /// Product surfaces ingest this certification instead of narrating maturity.
    pub product_ingests_certification: bool,
    /// Docs/help ingests the same certification.
    pub docs_help_ingests_certification: bool,
    /// Accessibility guidance ingests the same certification.
    pub accessibility_ingests_certification: bool,
    /// Release-control surfaces ingest the same certification.
    pub release_control_ingests_certification: bool,
    /// Narrowed and blocked rows are labeled below current in every surface.
    pub narrowed_rows_labeled_below_current: bool,
}

impl CertificationConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_certification
            && self.docs_help_ingests_certification
            && self.accessibility_ingests_certification
            && self.release_control_ingests_certification
            && self.narrowed_rows_labeled_below_current
    }
}

/// Release-gate block. Names how the certification result gates promotion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationReleaseGate {
    /// True when a blocked row refuses promotion.
    pub blocks_promotion_on_blocked_row: bool,
    /// True when a claimed row that is not certified cannot promote without
    /// visibly narrowing.
    pub blocks_promotion_on_uncertified_claim: bool,
    /// True when stale evidence auto-narrows claimed rows.
    pub stale_evidence_auto_narrows: bool,
    /// Evidence-freshness SLO in hours.
    pub evidence_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last evidence refresh.
    pub last_evidence_refresh: String,
}

impl CertificationReleaseGate {
    /// Whether the release-gate block is complete and enforcing.
    pub fn is_enforcing(&self) -> bool {
        self.blocks_promotion_on_blocked_row
            && self.blocks_promotion_on_uncertified_claim
            && self.stale_evidence_auto_narrows
            && self.evidence_freshness_slo_hours > 0
            && !self.last_evidence_refresh.trim().is_empty()
    }
}

/// Constructor input for [`M5CollectionCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CollectionCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Per-row certifications.
    pub rows: Vec<CertificationRow>,
    /// Guardrail invariants block.
    pub guardrails: CertificationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: CertificationConsumerProjection,
    /// Release-gate block.
    pub release_gate: CertificationReleaseGate,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe dense-collection certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollectionCertificationPacket {
    /// Record kind; must equal [`M5_COLLECTION_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COLLECTION_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Per-row certifications.
    pub rows: Vec<CertificationRow>,
    /// Guardrail invariants block.
    pub guardrails: CertificationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: CertificationConsumerProjection,
    /// Release-gate block.
    pub release_gate: CertificationReleaseGate,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CollectionCertificationPacket {
    /// Builds a dense-collection certification packet.
    pub fn new(input: M5CollectionCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_COLLECTION_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_COLLECTION_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            release_gate: input.release_gate,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surfaces represented by some row in this certification.
    pub fn represented_surfaces(&self) -> BTreeSet<DenseCollectionSurface> {
        self.rows.iter().map(|row| row.surface).collect()
    }

    /// Count of rows certified at their claim.
    pub fn certified_row_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.verdict == CertificationVerdict::Certified)
            .count()
    }

    /// Count of rows auto-narrowed below their claim.
    pub fn narrowed_row_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.verdict == CertificationVerdict::AutoNarrowed)
            .count()
    }

    /// Count of rows blocked from promotion.
    pub fn blocked_row_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.verdict == CertificationVerdict::Blocked)
            .count()
    }

    /// Count of rows holding a public claim.
    pub fn claimed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.is_claimed()).count()
    }

    /// Validates the dense-collection certification invariants.
    pub fn validate(&self) -> Vec<M5CollectionCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COLLECTION_CERTIFICATION_RECORD_KIND {
            violations.push(M5CollectionCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COLLECTION_CERTIFICATION_SCHEMA_VERSION {
            violations.push(M5CollectionCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CollectionCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_gate(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("collection certification packet serializes"),
        ) {
            violations.push(M5CollectionCertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("collection certification packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Dense Collection Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Rows: {} ({} claimed, {} certified, {} narrowed, {} blocked)\n",
            self.rows.len(),
            self.claimed_row_count(),
            self.certified_row_count(),
            self.narrowed_row_count(),
            self.blocked_row_count()
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {}\n",
            self.represented_surfaces().len(),
            DenseCollectionSurface::ALL.len()
        ));
        out.push_str(&format!(
            "- Evidence freshness SLO: {} hours (last refresh: {})\n",
            self.release_gate.evidence_freshness_slo_hours, self.release_gate.last_evidence_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}): claim `{}` -> certified `{}` [{}]\n",
                row.row_id,
                row.surface.as_str(),
                row.claimed_qualification.as_str(),
                row.certified_qualification.as_str(),
                row.verdict.as_str()
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!(
                "  - proofs: {}\n",
                row.proofs
                    .iter()
                    .map(|proof| format!("{}={}", proof.dimension.as_str(), proof.status.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            if let Some(regression) = row.regression {
                out.push_str(&format!("  - Regression: {}\n", regression.as_str()));
            }
            if let Some(label) = &row.narrowed_label {
                out.push_str(&format!("  - Narrowed: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum M5CollectionCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CollectionCertificationViolation>),
}

impl fmt::Display for M5CollectionCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "collection certification export parse failed: {error}"
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
                    "collection certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CollectionCertificationArtifactError {}

/// Validation failures emitted by [`M5CollectionCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CollectionCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required dense collection surface is represented by no row.
    RequiredSurfaceMissing,
    /// No row demonstrates auto-narrowing on an uncovered proof dimension.
    AutoNarrowedCaseMissing,
    /// No row demonstrates a blocked regression.
    BlockedRegressionCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A row does not present a proof for every required dimension.
    ProofDimensionMissing,
    /// A claimed row with an uncovered dimension was not narrowed below its claim.
    UncoveredClaimNotNarrowed,
    /// A narrowed or blocked row lacks a precise narrowed label.
    NarrowedRowMissingLabel,
    /// A blocked row lacks a recorded regression class.
    BlockedRowMissingRegression,
    /// Selection is not durable by stable identity.
    SelectionNotDurableByStableIdentity,
    /// Provider or policy narrowing is hidden in a generic chip.
    ProviderPolicyNarrowingHidden,
    /// Visible rows are treated as all matching rows without an explicit step.
    VisibleTreatedAsAllMatching,
    /// A broad batch action bypasses preview.
    BroadActionBypassesPreview,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release-gate block is incomplete or not enforcing.
    ReleaseGateNotEnforcing,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5CollectionCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::AutoNarrowedCaseMissing => "auto_narrowed_case_missing",
            Self::BlockedRegressionCaseMissing => "blocked_regression_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::ProofDimensionMissing => "proof_dimension_missing",
            Self::UncoveredClaimNotNarrowed => "uncovered_claim_not_narrowed",
            Self::NarrowedRowMissingLabel => "narrowed_row_missing_label",
            Self::BlockedRowMissingRegression => "blocked_row_missing_regression",
            Self::SelectionNotDurableByStableIdentity => "selection_not_durable_by_stable_identity",
            Self::ProviderPolicyNarrowingHidden => "provider_policy_narrowing_hidden",
            Self::VisibleTreatedAsAllMatching => "visible_treated_as_all_matching",
            Self::BroadActionBypassesPreview => "broad_action_bypasses_preview",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleaseGateNotEnforcing => "release_gate_not_enforcing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in certification export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_m5_collection_certification_export(
) -> Result<M5CollectionCertificationPacket, M5CollectionCertificationArtifactError> {
    let packet: M5CollectionCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/collections/m5/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle/support_export.json"
    )))
    .map_err(M5CollectionCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CollectionCertificationArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5CollectionCertificationPacket,
    violations: &mut Vec<M5CollectionCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_COLLECTION_CERTIFICATION_SCHEMA_REF,
        M5_COLLECTION_CERTIFICATION_DOC_REF,
        M5_COLLECTION_CERTIFICATION_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CollectionCertificationViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &M5CollectionCertificationPacket,
    violations: &mut Vec<M5CollectionCertificationViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in DenseCollectionSurface::ALL {
        if !surfaces.contains(&required) {
            violations.push(M5CollectionCertificationViolation::RequiredSurfaceMissing);
            break;
        }
    }

    if !packet
        .rows
        .iter()
        .any(|row| row.verdict == CertificationVerdict::AutoNarrowed && row.verdict_consistent())
    {
        violations.push(M5CollectionCertificationViolation::AutoNarrowedCaseMissing);
    }

    if !packet
        .rows
        .iter()
        .any(|row| row.verdict == CertificationVerdict::Blocked && row.verdict_consistent())
    {
        violations.push(M5CollectionCertificationViolation::BlockedRegressionCaseMissing);
    }
}

fn validate_rows(
    packet: &M5CollectionCertificationPacket,
    violations: &mut Vec<M5CollectionCertificationViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(M5CollectionCertificationViolation::RowIncomplete);
        }
        if !row.proof_dimensions_complete() {
            violations.push(M5CollectionCertificationViolation::ProofDimensionMissing);
        }
        if row.has_uncovered_dimension()
            && row.regression.is_none()
            && row.certified_qualification.rank() >= row.claimed_qualification.rank()
        {
            violations.push(M5CollectionCertificationViolation::UncoveredClaimNotNarrowed);
        }
        if matches!(
            row.verdict,
            CertificationVerdict::AutoNarrowed | CertificationVerdict::Blocked
        ) && !row
            .narrowed_label
            .as_ref()
            .is_some_and(|label| !label_is_generic(label))
        {
            violations.push(M5CollectionCertificationViolation::NarrowedRowMissingLabel);
        }
        if row.verdict == CertificationVerdict::Blocked && row.regression.is_none() {
            violations.push(M5CollectionCertificationViolation::BlockedRowMissingRegression);
        }
        if !row.selection_survives_by_stable_identity {
            violations
                .push(M5CollectionCertificationViolation::SelectionNotDurableByStableIdentity);
        }
        if !row.provider_policy_narrowing_disclosed {
            violations.push(M5CollectionCertificationViolation::ProviderPolicyNarrowingHidden);
        }
        if !row.visible_distinct_from_all_matching {
            violations.push(M5CollectionCertificationViolation::VisibleTreatedAsAllMatching);
        }
        if !row.broad_actions_preview_before_commit {
            violations.push(M5CollectionCertificationViolation::BroadActionBypassesPreview);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(M5CollectionCertificationViolation::RowEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &M5CollectionCertificationPacket,
    violations: &mut Vec<M5CollectionCertificationViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(M5CollectionCertificationViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &M5CollectionCertificationPacket,
    violations: &mut Vec<M5CollectionCertificationViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(M5CollectionCertificationViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_release_gate(
    packet: &M5CollectionCertificationPacket,
    violations: &mut Vec<M5CollectionCertificationViolation>,
) {
    if !packet.release_gate.is_enforcing() {
        violations.push(M5CollectionCertificationViolation::ReleaseGateNotEnforcing);
    }
}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
///
/// A generic provider error must never stand in for a precise narrowing truth.
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
            | "blocked"
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
