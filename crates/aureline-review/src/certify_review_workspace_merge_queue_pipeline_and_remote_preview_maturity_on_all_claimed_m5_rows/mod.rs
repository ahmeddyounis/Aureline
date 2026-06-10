//! Certification of review-workspace, merge-queue, pipeline, and remote-preview maturity across all claimed M5 rows.
//!
//! This module is the canonical certification layer over the M5 review, CI, and
//! preview depth lanes. Where the frozen maturity matrix locks four lanes at
//! lane granularity, this packet certifies every claimed M5 *row* that feeds
//! those lanes — durable review-workspace headers, merge-queue readiness,
//! pipeline-viewer depth, remote preview routes, attributable rerun/cancel
//! authority, AI review evidence cards, review/export bundles, and the maturity
//! matrix itself — binding each row to a certification verdict, the evidence
//! packet refs that back the claim, the downgrade triggers that can narrow it,
//! and a per-row proof-freshness observation.
//!
//! Each [`M5ReviewCertifiedRow`] references its upstream packet by record kind,
//! support-export artifact, schema, and contract doc rather than embedding the
//! packet body. A [`M5ReviewCertificationCompatibilityReport`] aggregates the
//! per-row verdicts into a single promotion verdict, and
//! [`M5ReviewCertificationPacket::apply_downgrade_automation`] narrows rows whose
//! proof is stale, whose evidence packet failed validation, or whose upstream
//! dependency narrowed — so CI or release tooling can fail promotion or narrow
//! the claim automatically instead of shipping greener than the evidence.
//!
//! [`certify_from_current_exports`] is the first real consumer: it reads each
//! claimed row's checked-in support export through its own producer, validates
//! it, and certifies the row only when its evidence currently validates. Raw
//! diff bodies, raw build logs, raw pipeline artifacts, raw provider payloads,
//! credentials, and live preview origin responses stay outside this boundary.
//!
//! The boundary schema is
//! [`schemas/review/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows.schema.json`](../../../../schemas/review/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows.schema.json).
//! The contract doc is
//! [`docs/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows.md`](../../../../docs/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows.md).
//! The protected fixture directory is
//! [`fixtures/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows/`](../../../../fixtures/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ReviewCertificationPacket`].
pub const M5_REVIEW_CERTIFICATION_RECORD_KIND: &str =
    "certify_review_workspace_merge_queue_pipeline_and_remote_preview_maturity_on_all_claimed_m5_rows";

/// Schema version for M5 review, CI, and preview certification records.
pub const M5_REVIEW_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_REVIEW_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/review/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows.schema.json";

/// Repo-relative path of the M5 review, CI, and preview certification contract doc.
pub const M5_REVIEW_CERTIFICATION_DOC_REF: &str =
    "docs/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows.md";

/// Repo-relative path of the frozen maturity-matrix authority this certification builds on.
pub const M5_REVIEW_CERTIFICATION_MATRIX_CONTRACT_REF: &str =
    "schemas/review/freeze-the-m5-review-workspace-merge-queue-and-pipeline-viewer-maturity-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_REVIEW_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows";

/// Repo-relative path of the checked support-export artifact.
pub const M5_REVIEW_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_REVIEW_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows.md";

/// One claimed M5 row certified by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ClaimedRow {
    /// Durable review-workspace headers, local CI parity, and anchor rehydration.
    DurableReviewHeader,
    /// Merge-queue readiness, stale-base invalidation, and approval recomputation.
    MergeQueueReadiness,
    /// Normalized pipeline run rows, log viewers, artifact browsers, safe-preview classes.
    PipelineViewer,
    /// Remote preview route lifecycle, expiry, target identity, and trust disclosure.
    RemotePreviewRoute,
    /// Attributable rerun/cancel actions with execution-context reuse and side-effect review.
    RerunCancelReview,
    /// AI review evidence finding cards and review-pack integration with change objects.
    EvidenceCard,
    /// Review/export bundles, publish-later packets, and offline follow-up flows.
    ReviewExportBundle,
    /// Frozen M5 review-workspace, merge-queue, and pipeline-viewer maturity matrix.
    MaturityMatrix,
}

impl M5ClaimedRow {
    /// Every claimed row, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DurableReviewHeader,
        Self::MergeQueueReadiness,
        Self::PipelineViewer,
        Self::RemotePreviewRoute,
        Self::RerunCancelReview,
        Self::EvidenceCard,
        Self::ReviewExportBundle,
        Self::MaturityMatrix,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableReviewHeader => "durable_review_header",
            Self::MergeQueueReadiness => "merge_queue_readiness",
            Self::PipelineViewer => "pipeline_viewer",
            Self::RemotePreviewRoute => "remote_preview_route",
            Self::RerunCancelReview => "rerun_cancel_review",
            Self::EvidenceCard => "evidence_card",
            Self::ReviewExportBundle => "review_export_bundle",
            Self::MaturityMatrix => "maturity_matrix",
        }
    }
}

/// Review, CI, or preview lane a certified row belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewCertificationLane {
    /// Review-workspace lane.
    ReviewWorkspace,
    /// Merge-queue lane.
    MergeQueue,
    /// Pipeline-viewer lane.
    PipelineViewer,
    /// Remote-preview lane.
    RemotePreview,
    /// Cross-cutting control row (e.g. the maturity matrix itself).
    CrossCutting,
}

impl M5ReviewCertificationLane {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewWorkspace => "review_workspace",
            Self::MergeQueue => "merge_queue",
            Self::PipelineViewer => "pipeline_viewer",
            Self::RemotePreview => "remote_preview",
            Self::CrossCutting => "cross_cutting",
        }
    }
}

/// Qualification class claimed by a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewCertificationQualificationClass {
    /// Row claims the Stable maturity.
    Stable,
    /// Row claims the Beta maturity.
    Beta,
    /// Row claims the Preview maturity.
    Preview,
    /// Row is experimental and not claimed.
    Experimental,
    /// Row is unavailable on this build.
    Unavailable,
    /// Row is held pending upstream resolution.
    Held,
}

impl M5ReviewCertificationQualificationClass {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }
}

/// Certification verdict earned by a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewCertificationVerdict {
    /// Row is certified at its claimed qualification with current, valid evidence.
    Certified,
    /// Row is certified, but narrowed below its claimed qualification.
    NarrowedCertified,
    /// Row is blocked from promotion until its evidence or dependency recovers.
    Blocked,
    /// Row could not be certified (missing or invalid evidence).
    NotCertified,
}

impl M5ReviewCertificationVerdict {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedCertified => "narrowed_certified",
            Self::Blocked => "blocked",
            Self::NotCertified => "not_certified",
        }
    }

    /// Whether the verdict still permits a public claim (possibly narrowed).
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Certified | Self::NarrowedCertified)
    }
}

/// Downgrade trigger that can narrow a certified row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewCertificationDowngradeTrigger {
    /// Proof packet has gone stale relative to its freshness SLO.
    ProofStale,
    /// Evidence packet failed validation or is missing.
    EvidencePacketInvalid,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Merge-queue CI-status truth has gone stale relative to the head it gates.
    MergeQueueStatusStale,
    /// A review-workspace anchor drifted off its target.
    AnchorDrift,
    /// Safe-preview rendering for pipeline logs or artifacts is unavailable.
    SafePreviewUnavailable,
    /// The remote preview route's time bound expired.
    PreviewRouteExpired,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified review/CI/preview boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency row narrowed.
    UpstreamDependencyNarrowed,
}

impl M5ReviewCertificationDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::EvidencePacketInvalid,
        Self::PolicyBlocked,
        Self::MergeQueueStatusStale,
        Self::AnchorDrift,
        Self::SafePreviewUnavailable,
        Self::PreviewRouteExpired,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::EvidencePacketInvalid => "evidence_packet_invalid",
            Self::PolicyBlocked => "policy_blocked",
            Self::MergeQueueStatusStale => "merge_queue_status_stale",
            Self::AnchorDrift => "anchor_drift",
            Self::SafePreviewUnavailable => "safe_preview_unavailable",
            Self::PreviewRouteExpired => "preview_route_expired",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for a certified row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewCertificationRollbackPosture {
    /// Read-only row that never mutates workspace, repository, or remote state.
    ReadOnlyNoMutation,
    /// Rerun/cancel actions remain individually attributable and reviewable.
    AttributableRerunOrCancel,
    /// Browser or preview handoff that always preserves a safe return path to the IDE.
    ReturnPathPreserved,
    /// Remote preview route auto-expires at its time bound with no lingering scope.
    TimeBoundedAutoExpire,
    /// Evidence is preserved but no automatic revert exists.
    EvidencePreservedNoRevert,
    /// Not applicable for the row's current verdict.
    NotApplicable,
}

impl M5ReviewCertificationRollbackPosture {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyNoMutation => "read_only_no_mutation",
            Self::AttributableRerunOrCancel => "attributable_rerun_or_cancel",
            Self::ReturnPathPreserved => "return_path_preserved",
            Self::TimeBoundedAutoExpire => "time_bounded_auto_expire",
            Self::EvidencePreservedNoRevert => "evidence_preserved_no_revert",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Per-row proof-freshness observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewCertificationRowFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the row's last proof refresh.
    pub last_proof_refresh: String,
    /// True when the row's proof is currently within its freshness SLO.
    pub proof_fresh: bool,
}

/// One certified claimed M5 row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewCertifiedRow {
    /// Claimed M5 row.
    pub row: M5ClaimedRow,
    /// Lane the row belongs to.
    pub lane: M5ReviewCertificationLane,
    /// Qualification class claimed by the row.
    pub claimed_qualification: M5ReviewCertificationQualificationClass,
    /// Certification verdict.
    pub verdict: M5ReviewCertificationVerdict,
    /// Upstream packet record kind backing the row.
    pub upstream_record_kind: String,
    /// Support-export artifact ref backing the row.
    pub evidence_artifact_ref: String,
    /// Schema ref backing the row.
    pub evidence_schema_ref: String,
    /// Contract doc ref backing the row.
    pub evidence_doc_ref: String,
    /// Downgrade triggers that can narrow the row.
    pub downgrade_triggers: Vec<M5ReviewCertificationDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5ReviewCertificationRollbackPosture,
    /// Per-row proof freshness.
    pub proof_freshness: M5ReviewCertificationRowFreshness,
}

/// Aggregate compatibility report across all certified rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewCertificationCompatibilityReport {
    /// Total certified rows in the packet.
    pub total_rows: u32,
    /// Count of fully certified rows.
    pub certified_count: u32,
    /// Count of narrowed-but-certified rows.
    pub narrowed_count: u32,
    /// Count of blocked rows.
    pub blocked_count: u32,
    /// Count of rows that could not be certified.
    pub not_certified_count: u32,
    /// True when every row is publishable (certified or narrowed).
    pub all_rows_publishable: bool,
    /// Human-readable promotion note.
    pub promotion_note: String,
}

impl M5ReviewCertificationCompatibilityReport {
    /// Recomputes the compatibility report from a row set.
    pub fn from_rows(rows: &[M5ReviewCertifiedRow]) -> Self {
        let mut certified = 0u32;
        let mut narrowed = 0u32;
        let mut blocked = 0u32;
        let mut not_certified = 0u32;
        for row in rows {
            match row.verdict {
                M5ReviewCertificationVerdict::Certified => certified += 1,
                M5ReviewCertificationVerdict::NarrowedCertified => narrowed += 1,
                M5ReviewCertificationVerdict::Blocked => blocked += 1,
                M5ReviewCertificationVerdict::NotCertified => not_certified += 1,
            }
        }
        let all_publishable = blocked == 0 && not_certified == 0;
        let promotion_note = if all_publishable {
            "all claimed M5 review/ci/preview rows are publishable".to_owned()
        } else {
            format!(
                "{} row(s) blocked and {} row(s) uncertified; promotion narrows",
                blocked, not_certified
            )
        };
        Self {
            total_rows: rows.len() as u32,
            certified_count: certified,
            narrowed_count: narrowed,
            blocked_count: blocked,
            not_certified_count: not_certified,
            all_rows_publishable: all_publishable,
            promotion_note,
        }
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewCertificationTrustReview {
    /// Review-workspace anchors stay durable across edits, rebases, and reopens.
    pub review_workspace_anchors_durable: bool,
    /// Stale-base and outdated-diff states are labeled, never silently hidden.
    pub stale_base_labels_explicit: bool,
    /// Merge-queue CI-status truth stays fresh relative to the head it gates.
    pub merge_queue_truth_fresh: bool,
    /// Every rerun and cancel action stays individually attributable and reviewable.
    pub rerun_cancel_authority_attributable: bool,
    /// Pipeline logs and artifacts are rendered through the safe-preview boundary.
    pub pipeline_logs_artifacts_safe_previewed: bool,
    /// Remote preview routes stay time-bounded and auto-expire at their bound.
    pub remote_preview_time_bounded: bool,
    /// Remote preview routes stay attributable to their opener and origin.
    pub remote_preview_attributable: bool,
    /// Browser and preview handoffs stay return-path safe.
    pub browser_handoff_return_path_safe: bool,
    /// No provider overlay, browser handoff, or preview server creates hidden write scope.
    pub no_hidden_write_scope: bool,
    /// Downgrade narrows the claim rather than hiding the row.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewCertificationConsumerProjection {
    /// Review workspace shows certification verdict and provenance.
    pub review_workspace_shows_certification: bool,
    /// Merge-queue panel shows certification verdict and freshness.
    pub merge_queue_shows_certification: bool,
    /// Pipeline viewer shows certification verdict and safe-preview state.
    pub pipeline_viewer_shows_certification: bool,
    /// Remote preview panel shows certification verdict, expiry, and attribution.
    pub remote_preview_shows_certification: bool,
    /// CLI / headless shows certification truth.
    pub cli_headless_shows_certification: bool,
    /// Support export shows certification truth.
    pub support_export_shows_certification: bool,
    /// Diagnostics shows certification truth.
    pub diagnostics_shows_certification: bool,
    /// Help / About shows certification truth.
    pub help_about_shows_certification: bool,
    /// Preview / Labs rows are visibly labeled when not certified Stable.
    pub preview_labs_label_for_unqualified_rows: bool,
}

/// Packet-level proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewCertificationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last certification refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the certification.
    pub auto_narrow_on_stale: bool,
}

/// Per-row observation fed to [`M5ReviewCertificationPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ReviewCertificationRowObservation {
    /// Row the observation applies to.
    pub row: M5ClaimedRow,
    /// True when the row's checked-in evidence currently validates.
    pub evidence_valid: bool,
    /// True when the row's proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an upstream dependency of the row narrowed.
    pub upstream_narrowed: bool,
}

/// Constructor input for [`M5ReviewCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ReviewCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified rows.
    pub certified_rows: Vec<M5ReviewCertifiedRow>,
    /// Compatibility report.
    pub compatibility_report: M5ReviewCertificationCompatibilityReport,
    /// Trust review block.
    pub trust_review: M5ReviewCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReviewCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReviewCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 review, CI, and preview certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewCertificationPacket {
    /// Record kind; must equal [`M5_REVIEW_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_REVIEW_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified rows.
    pub certified_rows: Vec<M5ReviewCertifiedRow>,
    /// Compatibility report.
    pub compatibility_report: M5ReviewCertificationCompatibilityReport,
    /// Trust review block.
    pub trust_review: M5ReviewCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReviewCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReviewCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ReviewCertificationPacket {
    /// Builds an M5 review, CI, and preview certification packet from stable-lane input.
    pub fn new(input: M5ReviewCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_REVIEW_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_REVIEW_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            certified_rows: input.certified_rows,
            compatibility_report: input.compatibility_report,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows rows whose evidence is invalid, whose proof is stale, or whose
    /// upstream dependency narrowed, then recomputes the compatibility report.
    ///
    /// This is the downgrade automation: invalid evidence blocks the row,
    /// stale proof or a narrowed upstream narrows it, and the per-row
    /// freshness flag is updated. Observations for rows not present in the
    /// packet are ignored; rows without an observation are left unchanged.
    pub fn apply_downgrade_automation(
        &mut self,
        observations: &[M5ReviewCertificationRowObservation],
    ) {
        for row in &mut self.certified_rows {
            let Some(observation) = observations.iter().find(|obs| obs.row == row.row) else {
                continue;
            };
            row.proof_freshness.proof_fresh = observation.proof_fresh;
            if !observation.evidence_valid {
                row.verdict = M5ReviewCertificationVerdict::Blocked;
            } else if (!observation.proof_fresh || observation.upstream_narrowed)
                && row.verdict == M5ReviewCertificationVerdict::Certified
            {
                row.verdict = M5ReviewCertificationVerdict::NarrowedCertified;
            }
        }
        self.compatibility_report =
            M5ReviewCertificationCompatibilityReport::from_rows(&self.certified_rows);
    }

    /// Validates the M5 review, CI, and preview certification invariants.
    pub fn validate(&self) -> Vec<M5ReviewCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_REVIEW_CERTIFICATION_RECORD_KIND {
            violations.push(M5ReviewCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_REVIEW_CERTIFICATION_SCHEMA_VERSION {
            violations.push(M5ReviewCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ReviewCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_compatibility_report(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 review certification packet serializes"),
        ) {
            violations.push(M5ReviewCertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 review certification packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Review, Merge-Queue, Pipeline, and Remote-Preview Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Rows: {} ({} certified, {} narrowed, {} blocked, {} uncertified)\n",
            self.compatibility_report.total_rows,
            self.compatibility_report.certified_count,
            self.compatibility_report.narrowed_count,
            self.compatibility_report.blocked_count,
            self.compatibility_report.not_certified_count,
        ));
        out.push_str(&format!(
            "- All rows publishable: {}\n",
            self.compatibility_report.all_rows_publishable
        ));
        out.push_str(&format!(
            "- Promotion: {}\n",
            self.compatibility_report.promotion_note
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.certified_rows {
            out.push_str(&format!(
                "- **{}** ({}): `{}` (claimed `{}`)\n",
                row.row.as_str(),
                row.lane.as_str(),
                row.verdict.as_str(),
                row.claimed_qualification.as_str(),
            ));
            out.push_str(&format!("  - Evidence: `{}`\n", row.evidence_artifact_ref));
            out.push_str(&format!(
                "  - Proof fresh: {} (last refresh: {})\n",
                row.proof_freshness.proof_fresh, row.proof_freshness.last_proof_refresh
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 review certification export.
#[derive(Debug)]
pub enum M5ReviewCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ReviewCertificationViolation>),
}

impl fmt::Display for M5ReviewCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 review certification export parse failed: {error}"
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
                    "m5 review certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ReviewCertificationArtifactError {}

/// Validation failures emitted by [`M5ReviewCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ReviewCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required claimed row is missing from the certification.
    RequiredRowMissing,
    /// A certified row is incomplete.
    RowIncomplete,
    /// A publishable row is missing evidence refs.
    PublishableRowMissingEvidence,
    /// A row has no downgrade triggers.
    DowngradeTriggersMissing,
    /// The compatibility report does not agree with the row verdicts.
    CompatibilityReportMismatch,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ReviewCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredRowMissing => "required_row_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::PublishableRowMissingEvidence => "publishable_row_missing_evidence",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::CompatibilityReportMismatch => "compatibility_report_mismatch",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in M5 review certification export.
pub fn current_m5_review_certification_export(
) -> Result<M5ReviewCertificationPacket, M5ReviewCertificationArtifactError> {
    let packet: M5ReviewCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows/support_export.json"
    )))
    .map_err(M5ReviewCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ReviewCertificationArtifactError::Validation(violations))
    }
}

/// First-consumer certification: reads every claimed row's checked-in support
/// export through its own producer and certifies a row only when its evidence
/// currently validates.
///
/// A row whose upstream export fails to parse or validate is recorded as
/// [`M5ReviewCertificationVerdict::Blocked`], so a stale or underqualified row
/// narrows the certification instead of leaving it greener than the evidence.
/// The returned packet's compatibility report and proof-freshness metadata are
/// minted from `minted_at` and `proof_freshness`.
pub fn certify_from_current_exports(
    packet_id: String,
    certification_label: String,
    minted_at: String,
    proof_freshness: M5ReviewCertificationProofFreshness,
) -> M5ReviewCertificationPacket {
    let rows = M5ClaimedRow::ALL
        .into_iter()
        .map(|row| {
            let descriptor = row_descriptor(row);
            let evidence_valid = descriptor.evidence_valid();
            let verdict = if !evidence_valid {
                M5ReviewCertificationVerdict::Blocked
            } else {
                descriptor.default_verdict
            };
            M5ReviewCertifiedRow {
                row,
                lane: descriptor.lane,
                claimed_qualification: descriptor.claimed_qualification,
                verdict,
                upstream_record_kind: descriptor.upstream_record_kind.to_owned(),
                evidence_artifact_ref: descriptor.evidence_artifact_ref.to_owned(),
                evidence_schema_ref: descriptor.evidence_schema_ref.to_owned(),
                evidence_doc_ref: descriptor.evidence_doc_ref.to_owned(),
                downgrade_triggers: descriptor.downgrade_triggers.clone(),
                rollback_posture: descriptor.rollback_posture,
                proof_freshness: M5ReviewCertificationRowFreshness {
                    proof_freshness_slo_hours: proof_freshness.proof_freshness_slo_hours,
                    last_proof_refresh: proof_freshness.last_proof_refresh.clone(),
                    proof_fresh: true,
                },
            }
        })
        .collect::<Vec<_>>();

    let compatibility_report = M5ReviewCertificationCompatibilityReport::from_rows(&rows);

    M5ReviewCertificationPacket::new(M5ReviewCertificationPacketInput {
        packet_id,
        certification_label,
        certified_rows: rows,
        compatibility_report,
        trust_review: canonical_trust_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness,
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at,
    })
}

/// Canonical trust review block with every invariant satisfied.
pub fn canonical_trust_review() -> M5ReviewCertificationTrustReview {
    M5ReviewCertificationTrustReview {
        review_workspace_anchors_durable: true,
        stale_base_labels_explicit: true,
        merge_queue_truth_fresh: true,
        rerun_cancel_authority_attributable: true,
        pipeline_logs_artifacts_safe_previewed: true,
        remote_preview_time_bounded: true,
        remote_preview_attributable: true,
        browser_handoff_return_path_safe: true,
        no_hidden_write_scope: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

/// Canonical consumer projection block with every surface projecting certification truth.
pub fn canonical_consumer_projection() -> M5ReviewCertificationConsumerProjection {
    M5ReviewCertificationConsumerProjection {
        review_workspace_shows_certification: true,
        merge_queue_shows_certification: true,
        pipeline_viewer_shows_certification: true,
        remote_preview_shows_certification: true,
        cli_headless_shows_certification: true,
        support_export_shows_certification: true,
        diagnostics_shows_certification: true,
        help_about_shows_certification: true,
        preview_labs_label_for_unqualified_rows: true,
    }
}

/// Canonical source contract refs that every certification export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        M5_REVIEW_CERTIFICATION_SCHEMA_REF.to_owned(),
        M5_REVIEW_CERTIFICATION_DOC_REF.to_owned(),
        M5_REVIEW_CERTIFICATION_MATRIX_CONTRACT_REF.to_owned(),
    ]
}

/// Static descriptor binding a claimed row to its lane, claim, and evidence refs.
struct RowDescriptor {
    lane: M5ReviewCertificationLane,
    claimed_qualification: M5ReviewCertificationQualificationClass,
    default_verdict: M5ReviewCertificationVerdict,
    upstream_record_kind: &'static str,
    evidence_artifact_ref: &'static str,
    evidence_schema_ref: &'static str,
    evidence_doc_ref: &'static str,
    downgrade_triggers: Vec<M5ReviewCertificationDowngradeTrigger>,
    rollback_posture: M5ReviewCertificationRollbackPosture,
    evidence_probe: fn() -> bool,
}

impl RowDescriptor {
    fn evidence_valid(&self) -> bool {
        (self.evidence_probe)()
    }
}

fn row_descriptor(row: M5ClaimedRow) -> RowDescriptor {
    use M5ReviewCertificationDowngradeTrigger as Trigger;
    use M5ReviewCertificationLane as Lane;
    use M5ReviewCertificationQualificationClass as Qual;
    use M5ReviewCertificationRollbackPosture as Rollback;
    use M5ReviewCertificationVerdict as Verdict;

    match row {
        M5ClaimedRow::DurableReviewHeader => RowDescriptor {
            lane: Lane::ReviewWorkspace,
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration::DURABLE_REVIEW_HEADER_RECORD_KIND,
            evidence_artifact_ref:
                crate::implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration::DURABLE_REVIEW_HEADER_ARTIFACT_REF,
            evidence_schema_ref:
                crate::implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration::DURABLE_REVIEW_HEADER_SCHEMA_REF,
            evidence_doc_ref:
                crate::implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration::DURABLE_REVIEW_HEADER_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::AnchorDrift,
                Trigger::TrustNarrowing,
            ],
            rollback_posture: Rollback::ReadOnlyNoMutation,
            evidence_probe: || crate::implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration::current_durable_review_header_export().is_ok(),
        },
        M5ClaimedRow::MergeQueueReadiness => RowDescriptor {
            lane: Lane::MergeQueue,
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows::MERGE_QUEUE_READINESS_RECORD_KIND,
            evidence_artifact_ref:
                crate::add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows::MERGE_QUEUE_READINESS_ARTIFACT_REF,
            evidence_schema_ref:
                crate::add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows::MERGE_QUEUE_READINESS_SCHEMA_REF,
            evidence_doc_ref:
                crate::add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows::MERGE_QUEUE_READINESS_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::MergeQueueStatusStale,
                Trigger::PolicyBlocked,
            ],
            rollback_posture: Rollback::AttributableRerunOrCancel,
            evidence_probe: || crate::add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows::current_merge_queue_readiness_export().is_ok(),
        },
        M5ClaimedRow::PipelineViewer => RowDescriptor {
            lane: Lane::PipelineViewer,
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes::PIPELINE_VIEWER_RECORD_KIND,
            evidence_artifact_ref:
                crate::implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes::PIPELINE_VIEWER_ARTIFACT_REF,
            evidence_schema_ref:
                crate::implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes::PIPELINE_VIEWER_SCHEMA_REF,
            evidence_doc_ref:
                crate::implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes::PIPELINE_VIEWER_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::SafePreviewUnavailable,
                Trigger::PolicyBlocked,
            ],
            rollback_posture: Rollback::ReadOnlyNoMutation,
            evidence_probe: || crate::implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes::current_pipeline_viewer_export().is_ok(),
        },
        M5ClaimedRow::RemotePreviewRoute => RowDescriptor {
            lane: Lane::RemotePreview,
            claimed_qualification: Qual::Beta,
            default_verdict: Verdict::NarrowedCertified,
            upstream_record_kind:
                crate::add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure::REMOTE_PREVIEW_ROUTE_RECORD_KIND,
            evidence_artifact_ref:
                crate::add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure::REMOTE_PREVIEW_ROUTE_ARTIFACT_REF,
            evidence_schema_ref:
                crate::add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure::REMOTE_PREVIEW_ROUTE_SCHEMA_REF,
            evidence_doc_ref:
                crate::add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure::REMOTE_PREVIEW_ROUTE_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::PreviewRouteExpired,
                Trigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: Rollback::TimeBoundedAutoExpire,
            evidence_probe: || crate::add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure::current_remote_preview_route_export().is_ok(),
        },
        M5ClaimedRow::RerunCancelReview => RowDescriptor {
            lane: Lane::MergeQueue,
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review::RERUN_CANCEL_RECORD_KIND,
            evidence_artifact_ref:
                crate::ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review::RERUN_CANCEL_ARTIFACT_REF,
            evidence_schema_ref:
                crate::ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review::RERUN_CANCEL_SCHEMA_REF,
            evidence_doc_ref:
                crate::ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review::RERUN_CANCEL_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::PolicyBlocked,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::AttributableRerunOrCancel,
            evidence_probe: || crate::ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review::current_rerun_cancel_review_export().is_ok(),
        },
        M5ClaimedRow::EvidenceCard => RowDescriptor {
            lane: Lane::ReviewWorkspace,
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects::EVIDENCE_CARD_RECORD_KIND,
            evidence_artifact_ref:
                crate::ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects::EVIDENCE_CARD_ARTIFACT_REF,
            evidence_schema_ref:
                crate::ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects::EVIDENCE_CARD_SCHEMA_REF,
            evidence_doc_ref:
                crate::ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects::EVIDENCE_CARD_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::PolicyBlocked,
                Trigger::TrustNarrowing,
            ],
            rollback_posture: Rollback::ReadOnlyNoMutation,
            evidence_probe: || crate::ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects::current_evidence_card_export().is_ok(),
        },
        M5ClaimedRow::ReviewExportBundle => RowDescriptor {
            lane: Lane::ReviewWorkspace,
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces::REVIEW_EXPORT_BUNDLE_RECORD_KIND,
            evidence_artifact_ref:
                crate::add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces::REVIEW_EXPORT_BUNDLE_ARTIFACT_REF,
            evidence_schema_ref:
                crate::add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces::REVIEW_EXPORT_BUNDLE_SCHEMA_REF,
            evidence_doc_ref:
                crate::add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces::REVIEW_EXPORT_BUNDLE_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::PolicyBlocked,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::EvidencePreservedNoRevert,
            evidence_probe: || crate::add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces::current_review_export_bundle_export().is_ok(),
        },
        M5ClaimedRow::MaturityMatrix => RowDescriptor {
            lane: Lane::CrossCutting,
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix::M5_REVIEW_CI_PREVIEW_MATRIX_RECORD_KIND,
            evidence_artifact_ref:
                crate::freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix::M5_REVIEW_CI_PREVIEW_MATRIX_ARTIFACT_REF,
            evidence_schema_ref:
                crate::freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix::M5_REVIEW_CI_PREVIEW_MATRIX_SCHEMA_REF,
            evidence_doc_ref:
                crate::freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix::M5_REVIEW_CI_PREVIEW_MATRIX_DOC_REF,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::UpstreamDependencyNarrowed,
                Trigger::TrustNarrowing,
            ],
            rollback_posture: Rollback::ReadOnlyNoMutation,
            evidence_probe: || crate::freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix::current_stable_m5_review_ci_preview_matrix_export().is_ok(),
        },
    }
}

fn validate_source_contracts(
    packet: &M5ReviewCertificationPacket,
    violations: &mut Vec<M5ReviewCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_REVIEW_CERTIFICATION_SCHEMA_REF,
        M5_REVIEW_CERTIFICATION_DOC_REF,
        M5_REVIEW_CERTIFICATION_MATRIX_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ReviewCertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &M5ReviewCertificationPacket,
    violations: &mut Vec<M5ReviewCertificationViolation>,
) {
    let present: BTreeSet<M5ClaimedRow> = packet.certified_rows.iter().map(|row| row.row).collect();
    for required in M5ClaimedRow::ALL {
        if !present.contains(&required) {
            violations.push(M5ReviewCertificationViolation::RequiredRowMissing);
            return;
        }
    }

    for row in &packet.certified_rows {
        if row.upstream_record_kind.trim().is_empty()
            || row.evidence_artifact_ref.trim().is_empty()
            || row.evidence_schema_ref.trim().is_empty()
            || row.evidence_doc_ref.trim().is_empty()
            || row.proof_freshness.last_proof_refresh.trim().is_empty()
            || row.proof_freshness.proof_freshness_slo_hours == 0
        {
            violations.push(M5ReviewCertificationViolation::RowIncomplete);
        }
        if row.verdict.is_publishable() && row.evidence_artifact_ref.trim().is_empty() {
            violations.push(M5ReviewCertificationViolation::PublishableRowMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ReviewCertificationViolation::DowngradeTriggersMissing);
        }
    }
}

fn validate_compatibility_report(
    packet: &M5ReviewCertificationPacket,
    violations: &mut Vec<M5ReviewCertificationViolation>,
) {
    let recomputed = M5ReviewCertificationCompatibilityReport::from_rows(&packet.certified_rows);
    if recomputed != packet.compatibility_report {
        violations.push(M5ReviewCertificationViolation::CompatibilityReportMismatch);
    }
}

fn validate_trust_review(
    packet: &M5ReviewCertificationPacket,
    violations: &mut Vec<M5ReviewCertificationViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.review_workspace_anchors_durable,
        review.stale_base_labels_explicit,
        review.merge_queue_truth_fresh,
        review.rerun_cancel_authority_attributable,
        review.pipeline_logs_artifacts_safe_previewed,
        review.remote_preview_time_bounded,
        review.remote_preview_attributable,
        review.browser_handoff_return_path_safe,
        review.no_hidden_write_scope,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(M5ReviewCertificationViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ReviewCertificationPacket,
    violations: &mut Vec<M5ReviewCertificationViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.review_workspace_shows_certification,
        projection.merge_queue_shows_certification,
        projection.pipeline_viewer_shows_certification,
        projection.remote_preview_shows_certification,
        projection.cli_headless_shows_certification,
        projection.support_export_shows_certification,
        projection.diagnostics_shows_certification,
        projection.help_about_shows_certification,
        projection.preview_labs_label_for_unqualified_rows,
    ] {
        if !ok {
            violations.push(M5ReviewCertificationViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ReviewCertificationPacket,
    violations: &mut Vec<M5ReviewCertificationViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ReviewCertificationViolation::ProofFreshnessIncomplete);
    }
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
