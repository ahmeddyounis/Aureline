//! Certified reference workspaces, archetype pass matrices, and downgrade
//! automation.
//!
//! Where the [`support_class_ledger`](crate::support_class_ledger) publishes the
//! v1.0 support-class assignments and certified-archetype manifest, this module
//! hardens the *reference workspace* and *archetype pass matrix* lanes that
//! defend every Certified claim. For every marketed Certified archetype it
//! publishes one current reference-workspace report, binds that report to the
//! archetype pass matrix row that carries it, and automates the downgrade that
//! narrows a Certified claim when its report goes stale, missing, or manually
//! edited.
//!
//! Each [`ReferenceWorkspaceReport`] records one reference-workspace report for
//! a certified archetype. It carries:
//!
//! - the archetype it speaks for ([`ReferenceWorkspaceReport::archetype_ref`]),
//! - certification-harness output and matrix diff from the prior report
//!   ([`ReferenceWorkspaceReport::certification_harness_output_ref`],
//!   [`ReferenceWorkspaceReport::matrix_diff_from_prior_ref`]),
//! - named owners and sign-off ([`ReferenceWorkspaceReport::owner_signoff`]),
//! - a known-caveat summary ([`ReferenceWorkspaceReport::known_caveat_summary`]),
//! - a validity window ([`ReferenceWorkspaceReport::validity_window`]),
//! - the report state earned ([`ReportState`]), active downgrade reasons
//!   ([`DowngradeReason`]), and the effective state after narrowing
//!   ([`ReferenceWorkspaceReport::effective_state`]).
//!
//! Each [`ArchetypePassMatrixRow`] records one archetype pass-matrix row. It
//! carries the archetype it speaks for, whether the archetype is put forward as
//! certified ([`ArchetypePassMatrixRow::claimed_certified`]), the matrix row
//! state earned ([`MatrixRowState`]), active downgrade reasons, and whether the
//! archetype is effectively certified after narrowing
//! ([`ArchetypePassMatrixRow::effective_certified`]).
//!
//! The [`DowngradeRule`] set names the closed conditions that gate publication,
//! and [`CertifiedReferenceWorkspaces::publication`] records the resulting
//! proceed/hold verdict.
//!
//! The artifact is checked in at
//! `artifacts/release/harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material. Date
//! arithmetic (validity-window expiry and waiver expiry against an `as_of` date)
//! lives in the CI gate; this model enforces the structural and logical
//! invariants that hold regardless of the clock — narrowing consistency, the
//! no-widening rule, report-state coherence, owner sign-off on certified claims,
//! downgrade-rule wiring, and the publication verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_matrix::{OwnerSignoff, QualificationWaiver};

/// Supported artifact schema version.
pub const CERTIFIED_REFERENCE_WORKSPACES_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the artifact.
pub const CERTIFIED_REFERENCE_WORKSPACES_RECORD_KIND: &str = "certified_reference_workspaces";

/// Repo-relative path to the checked-in artifact.
pub const CERTIFIED_REFERENCE_WORKSPACES_PATH: &str =
    "artifacts/release/harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation.json";

/// Embedded checked-in artifact JSON.
pub const CERTIFIED_REFERENCE_WORKSPACES_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation.json"
));

/// Report state a reference-workspace report earns for its archetype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportState {
    /// Report is current within its validity window.
    Current,
    /// Within the window but nearing expiry; refresh is due soon.
    DueForRefresh,
    /// The report aged past its validity window.
    Expired,
    /// No reference-workspace report has been captured.
    Missing,
}

impl ReportState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Current,
        Self::DueForRefresh,
        Self::Expired,
        Self::Missing,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::DueForRefresh => "due_for_refresh",
            Self::Expired => "expired",
            Self::Missing => "missing",
        }
    }

    /// Whether the state lets a report back a Certified claim.
    pub const fn holds_certification(self) -> bool {
        matches!(self, Self::Current | Self::DueForRefresh)
    }

    /// Whether the state forces the report below a Certified claim.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_certification()
    }
}

/// Matrix row state an archetype pass-matrix row earns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixRowState {
    /// The archetype passes all criteria and is certified.
    Certified,
    /// The archetype is certified only because an active waiver covers a gap.
    ProvisionalOnWaiver,
    /// Required pass criteria are absent or failed; the row narrows.
    NarrowedUnqualified,
    /// The reference-workspace report or matrix evidence is stale; the row narrows.
    NarrowedStale,
    /// The row relied on a waiver that has expired; the row narrows.
    NarrowedWaiverExpired,
}

impl MatrixRowState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Certified,
        Self::ProvisionalOnWaiver,
        Self::NarrowedUnqualified,
        Self::NarrowedStale,
        Self::NarrowedWaiverExpired,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::ProvisionalOnWaiver => "provisional_on_waiver",
            Self::NarrowedUnqualified => "narrowed_unqualified",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
        }
    }

    /// Whether the state lets the row hold a certified claim.
    pub const fn holds_certified(self) -> bool {
        matches!(self, Self::Certified | Self::ProvisionalOnWaiver)
    }

    /// Whether the state forces the row below certified.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_certified()
    }
}

/// Closed reason a certified claim narrows or a downgrade rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeReason {
    /// The reference-workspace report is no longer current.
    ReferenceWorkspaceReportStale,
    /// No reference-workspace report has been captured.
    ReferenceWorkspaceReportMissing,
    /// The reference-workspace report was manually edited outside the harness.
    ReferenceWorkspaceReportManuallyEdited,
    /// The archetype pass matrix shows a regression against prior certification.
    ArchetypePassMatrixRegression,
    /// The archetype was decertified in the certified-archetype manifest.
    ArchetypeDecertified,
    /// A waiver the claim relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl DowngradeReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ReferenceWorkspaceReportStale,
        Self::ReferenceWorkspaceReportMissing,
        Self::ReferenceWorkspaceReportManuallyEdited,
        Self::ArchetypePassMatrixRegression,
        Self::ArchetypeDecertified,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReferenceWorkspaceReportStale => "reference_workspace_report_stale",
            Self::ReferenceWorkspaceReportMissing => "reference_workspace_report_missing",
            Self::ReferenceWorkspaceReportManuallyEdited => {
                "reference_workspace_report_manually_edited"
            }
            Self::ArchetypePassMatrixRegression => "archetype_pass_matrix_regression",
            Self::ArchetypeDecertified => "archetype_decertified",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a downgrade rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the certified claim below the certified cutline.
    NarrowCertifiedClaim,
    /// Refresh the reference-workspace report.
    RefreshReferenceWorkspace,
    /// Recapture the archetype pass matrix evidence.
    RecaptureArchetypePassMatrix,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl MatrixAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HoldPublication,
        Self::NarrowCertifiedClaim,
        Self::RefreshReferenceWorkspace,
        Self::RecaptureArchetypePassMatrix,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowCertifiedClaim => "narrow_certified_claim",
            Self::RefreshReferenceWorkspace => "refresh_reference_workspace",
            Self::RecaptureArchetypePassMatrix => "recapture_archetype_pass_matrix",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// Publication verdict for the certified-reference-workspaces lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationDecision {
    /// The certified line may publish.
    Proceed,
    /// Publication is blocked by one or more firing downgrade rules.
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

/// The validity window for a reference-workspace report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidityWindow {
    /// UTC date the report was captured, or null when none exists yet.
    #[serde(default)]
    pub captured_at: Option<String>,
    /// UTC date the report expires.
    pub expires_at: String,
    /// Days the report stays claim-bearing after capture.
    pub window_days: u32,
}

/// One reference-workspace report for a certified archetype.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceWorkspaceReport {
    /// Stable report id.
    pub report_id: String,
    /// Human-readable title.
    pub title: String,
    /// The certified-archetype id this report speaks for.
    pub archetype_ref: String,
    /// Ref to the certification-harness output that produced this report.
    pub certification_harness_output_ref: String,
    /// Ref to the matrix diff from the prior report.
    pub matrix_diff_from_prior_ref: String,
    /// Owning team or role.
    pub owner_ref: String,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Reviewable known-caveat summary.
    pub known_caveat_summary: String,
    /// Validity window for the report.
    pub validity_window: ValidityWindow,
    /// Report state earned.
    pub report_state: ReportState,
    /// Waiver authorizing a provisional certification, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Active downgrade reasons narrowing the report.
    #[serde(default)]
    pub active_downgrade_reasons: Vec<DowngradeReason>,
    /// The state the report effectively holds after narrowing.
    pub effective_state: ReportState,
    /// Publication destinations that render this report.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the report carries this posture.
    pub rationale: String,
}

impl ReferenceWorkspaceReport {
    /// True when the report's effective state backs a Certified claim.
    pub fn backs_certification(&self) -> bool {
        self.effective_state.holds_certification()
    }

    /// True when a downgrade reason is active on the report.
    pub fn has_active_reason(&self, reason: DowngradeReason) -> bool {
        self.active_downgrade_reasons.contains(&reason)
    }
}

/// One archetype pass-matrix row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArchetypePassMatrixRow {
    /// Stable row id.
    pub row_id: String,
    /// Human-readable title.
    pub title: String,
    /// The certified-archetype id this row speaks for.
    pub archetype_ref: String,
    /// Whether the archetype is put forward as certified.
    pub claimed_certified: bool,
    /// Matrix row state earned for the claimed certification.
    pub matrix_state: MatrixRowState,
    /// Pass-criteria refs backing the certification. Empty only on narrowed rows.
    #[serde(default)]
    pub pass_criteria_refs: Vec<String>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Waiver authorizing a provisional certification, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Active downgrade reasons narrowing the row.
    #[serde(default)]
    pub active_downgrade_reasons: Vec<DowngradeReason>,
    /// Whether the archetype is effectively certified after narrowing.
    pub effective_certified: bool,
    /// Publication destinations that render this row.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl ArchetypePassMatrixRow {
    /// True when the row effectively certifies the archetype.
    pub fn holds_certified(&self) -> bool {
        self.effective_certified
    }

    /// True when a downgrade reason is active on the row.
    pub fn has_active_reason(&self, reason: DowngradeReason) -> bool {
        self.active_downgrade_reasons.contains(&reason)
    }
}

/// One downgrade rule: a closed condition that narrows a certified claim and may
/// gate publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DowngradeRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The downgrade reason whose presence on a claimed row fires this rule.
    pub trigger_reason: DowngradeReason,
    /// Matrix row states this rule watches.
    pub applies_to_states: Vec<MatrixRowState>,
    /// Default action prescribed when the rule fires.
    pub default_action: MatrixAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// The recorded publication verdict for the certified-reference-workspaces lane.
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
    /// Matrix-row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_row_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedReferenceWorkspacesSummary {
    /// Total number of archetype pass-matrix rows.
    pub total_matrix_rows: usize,
    /// Matrix rows effectively certifying their archetype.
    pub matrix_rows_certified: usize,
    /// Matrix rows narrowed below certified.
    pub matrix_rows_narrowed: usize,
    /// Matrix rows holding certification via an active waiver.
    pub matrix_rows_on_active_waiver: usize,
    /// Reference-workspace reports whose state is current.
    pub reports_current: usize,
    /// Reference-workspace reports whose state is due for refresh.
    pub reports_due_for_refresh: usize,
    /// Reference-workspace reports whose state is expired.
    pub reports_expired: usize,
    /// Reference-workspace reports whose state is missing.
    pub reports_missing: usize,
    /// Total active downgrade reasons across all rows and reports.
    pub total_active_downgrade_reasons: usize,
    /// Number of downgrade rules currently firing.
    pub downgrade_rules_firing: usize,
}

/// The typed certified-reference-workspaces artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedReferenceWorkspaces {
    /// Artifact schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable artifact identifier.
    pub artifact_id: String,
    /// Lifecycle status of this artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this artifact ingests.
    pub claim_manifest_ref: String,
    /// Ref to the support-class ledger this artifact ingests.
    pub support_class_ledger_ref: String,
    /// Closed report-state vocabulary.
    pub report_states: Vec<ReportState>,
    /// Closed matrix-row-state vocabulary.
    pub matrix_row_states: Vec<MatrixRowState>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Closed matrix-action vocabulary.
    pub matrix_actions: Vec<MatrixAction>,
    /// Reference-workspace reports.
    pub reference_workspace_reports: Vec<ReferenceWorkspaceReport>,
    /// Archetype pass-matrix rows.
    pub archetype_pass_matrix_rows: Vec<ArchetypePassMatrixRow>,
    /// Downgrade rules.
    pub downgrade_rules: Vec<DowngradeRule>,
    /// Recorded publication verdict.
    pub publication: PublicationDecisionRecord,
    /// Summary counts.
    pub summary: CertifiedReferenceWorkspacesSummary,
}

impl CertifiedReferenceWorkspaces {
    /// Returns the reference-workspace report registered for `report_id`.
    pub fn report(&self, report_id: &str) -> Option<&ReferenceWorkspaceReport> {
        self.reference_workspace_reports
            .iter()
            .find(|r| r.report_id == report_id)
    }

    /// Returns the archetype pass-matrix row registered for `row_id`.
    pub fn matrix_row(&self, row_id: &str) -> Option<&ArchetypePassMatrixRow> {
        self.archetype_pass_matrix_rows
            .iter()
            .find(|r| r.row_id == row_id)
    }

    /// Returns the matrix rows effectively certifying their archetype.
    pub fn rows_holding_certified(&self) -> Vec<&ArchetypePassMatrixRow> {
        self.archetype_pass_matrix_rows
            .iter()
            .filter(|r| r.holds_certified())
            .collect()
    }

    /// Returns the matrix rows narrowed below certified.
    pub fn rows_narrowed(&self) -> Vec<&ArchetypePassMatrixRow> {
        self.archetype_pass_matrix_rows
            .iter()
            .filter(|r| !r.holds_certified())
            .collect()
    }

    /// True when `rule` fires: a claimed row in its watch set carries its
    /// trigger reason.
    pub fn downgrade_rule_fires(&self, rule: &DowngradeRule) -> bool {
        self.archetype_pass_matrix_rows.iter().any(|row| {
            rule.applies_to_states.contains(&row.matrix_state)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and downgrade rules.
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

    /// Matrix-row ids that trigger a blocking, firing downgrade rule, sorted and
    /// unique.
    pub fn computed_blocking_row_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<DowngradeReason> = self
            .downgrade_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.downgrade_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.archetype_pass_matrix_rows {
            if row.claimed_certified
                && row
                    .active_downgrade_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.row_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows, reports, and rules.
    pub fn computed_summary(&self) -> CertifiedReferenceWorkspacesSummary {
        let reports = |state: ReportState| {
            self.reference_workspace_reports
                .iter()
                .filter(|r| r.report_state == state)
                .count()
        };
        CertifiedReferenceWorkspacesSummary {
            total_matrix_rows: self.archetype_pass_matrix_rows.len(),
            matrix_rows_certified: self
                .archetype_pass_matrix_rows
                .iter()
                .filter(|r| r.holds_certified())
                .count(),
            matrix_rows_narrowed: self
                .archetype_pass_matrix_rows
                .iter()
                .filter(|r| !r.holds_certified())
                .count(),
            matrix_rows_on_active_waiver: self
                .archetype_pass_matrix_rows
                .iter()
                .filter(|r| r.matrix_state == MatrixRowState::ProvisionalOnWaiver)
                .count(),
            reports_current: reports(ReportState::Current),
            reports_due_for_refresh: reports(ReportState::DueForRefresh),
            reports_expired: reports(ReportState::Expired),
            reports_missing: reports(ReportState::Missing),
            total_active_downgrade_reasons: self
                .archetype_pass_matrix_rows
                .iter()
                .map(|r| r.active_downgrade_reasons.len())
                .sum::<usize>()
                + self
                    .reference_workspace_reports
                    .iter()
                    .map(|r| r.active_downgrade_reasons.len())
                    .sum::<usize>(),
            downgrade_rules_firing: self
                .downgrade_rules
                .iter()
                .filter(|rule| self.downgrade_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the artifact that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> CertifiedReferenceWorkspacesExportProjection {
        CertifiedReferenceWorkspacesExportProjection {
            artifact_id: self.artifact_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            matrix_rows: self
                .archetype_pass_matrix_rows
                .iter()
                .map(|row| ArchetypePassMatrixExportRow {
                    row_id: row.row_id.clone(),
                    archetype_ref: row.archetype_ref.clone(),
                    claimed_certified: row.claimed_certified,
                    effective_certified: row.effective_certified,
                    holds_certified: row.holds_certified(),
                    matrix_state: row.matrix_state,
                    active_downgrade_reasons: row.active_downgrade_reasons.clone(),
                })
                .collect(),
            reference_workspace_reports: self
                .reference_workspace_reports
                .iter()
                .map(|report| ReferenceWorkspaceExportRow {
                    report_id: report.report_id.clone(),
                    archetype_ref: report.archetype_ref.clone(),
                    report_state: report.report_state,
                    effective_state: report.effective_state,
                    backs_certification: report.backs_certification(),
                    active_downgrade_reasons: report.active_downgrade_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the artifact, returning every violation found.
    pub fn validate(&self) -> Vec<CertifiedReferenceWorkspacesViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen_reports = BTreeSet::new();
        for report in &self.reference_workspace_reports {
            if !seen_reports.insert(report.report_id.clone()) {
                violations.push(CertifiedReferenceWorkspacesViolation::DuplicateReportId {
                    report_id: report.report_id.clone(),
                });
            }
            self.validate_report(report, &mut violations);
        }

        let mut seen_rows = BTreeSet::new();
        for row in &self.archetype_pass_matrix_rows {
            if !seen_rows.insert(row.row_id.clone()) {
                violations.push(CertifiedReferenceWorkspacesViolation::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            self.validate_matrix_row(row, &mut violations);
        }
        if self.archetype_pass_matrix_rows.is_empty() {
            violations.push(CertifiedReferenceWorkspacesViolation::EmptyMatrix);
        }

        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(CertifiedReferenceWorkspacesViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(
        &self,
        violations: &mut Vec<CertifiedReferenceWorkspacesViolation>,
    ) {
        if self.schema_version != CERTIFIED_REFERENCE_WORKSPACES_SCHEMA_VERSION {
            violations.push(CertifiedReferenceWorkspacesViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != CERTIFIED_REFERENCE_WORKSPACES_RECORD_KIND {
            violations.push(CertifiedReferenceWorkspacesViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("artifact_id", &self.artifact_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("support_class_ledger_ref", &self.support_class_ledger_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(CertifiedReferenceWorkspacesViolation::EmptyField {
                    id: "<artifact>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.report_states != ReportState::ALL.to_vec() {
            violations.push(CertifiedReferenceWorkspacesViolation::ClosedVocabularyMismatch {
                field: "report_states",
            });
        }
        if self.matrix_row_states != MatrixRowState::ALL.to_vec() {
            violations.push(CertifiedReferenceWorkspacesViolation::ClosedVocabularyMismatch {
                field: "matrix_row_states",
            });
        }
        if self.downgrade_reasons != DowngradeReason::ALL.to_vec() {
            violations.push(CertifiedReferenceWorkspacesViolation::ClosedVocabularyMismatch {
                field: "downgrade_reasons",
            });
        }
        if self.matrix_actions != MatrixAction::ALL.to_vec() {
            violations.push(CertifiedReferenceWorkspacesViolation::ClosedVocabularyMismatch {
                field: "matrix_actions",
            });
        }
    }

    fn validate_rules(
        &self,
        violations: &mut Vec<CertifiedReferenceWorkspacesViolation>,
    ) {
        if self.downgrade_rules.is_empty() {
            violations.push(CertifiedReferenceWorkspacesViolation::NoDowngradeRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.downgrade_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(CertifiedReferenceWorkspacesViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(CertifiedReferenceWorkspacesViolation::EmptyField {
                        id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_states.is_empty() {
                violations.push(CertifiedReferenceWorkspacesViolation::RuleWithoutStates {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in DowngradeReason::ALL {
            if !covered.contains(&reason) {
                violations.push(
                    CertifiedReferenceWorkspacesViolation::DowngradeReasonWithoutRule { reason },
                );
            }
        }
    }

    fn validate_report(
        &self,
        report: &ReferenceWorkspaceReport,
        violations: &mut Vec<CertifiedReferenceWorkspacesViolation>,
    ) {
        for (field, value) in [
            ("report_id", &report.report_id),
            ("title", &report.title),
            ("archetype_ref", &report.archetype_ref),
            (
                "certification_harness_output_ref",
                &report.certification_harness_output_ref,
            ),
            ("matrix_diff_from_prior_ref", &report.matrix_diff_from_prior_ref),
            ("owner_ref", &report.owner_ref),
            ("known_caveat_summary", &report.known_caveat_summary),
            ("rationale", &report.rationale),
            ("owner_signoff.owner_ref", &report.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(CertifiedReferenceWorkspacesViolation::EmptyField {
                    id: report.report_id.clone(),
                    field_name: field,
                });
            }
        }

        if report.validity_window.window_days == 0 {
            violations.push(CertifiedReferenceWorkspacesViolation::EmptyField {
                id: report.report_id.clone(),
                field_name: "validity_window.window_days",
            });
        }

        // A report that is current or due-for-refresh must be owner-signed and
        // free of active downgrade reasons.
        if report.report_state.holds_certification() {
            if !report.active_downgrade_reasons.is_empty() {
                violations.push(
                    CertifiedReferenceWorkspacesViolation::HeldReportWithActiveDowngrade {
                        report_id: report.report_id.clone(),
                    },
                );
            }
            if !(report.owner_signoff.signed_off && report.owner_signoff.signed_at.is_some()) {
                violations.push(
                    CertifiedReferenceWorkspacesViolation::HeldReportWithoutSignoff {
                        report_id: report.report_id.clone(),
                    },
                );
            }
        }

        // A narrowing state must drop the effective state below certification
        // and name at least one active reason.
        if report.report_state.forces_narrowing() {
            if report.effective_state.holds_certification() {
                violations.push(
                    CertifiedReferenceWorkspacesViolation::EffectiveStateNotNarrowed {
                        report_id: report.report_id.clone(),
                        state: report.report_state,
                        effective: report.effective_state,
                    },
                );
            }
            if report.active_downgrade_reasons.is_empty() {
                violations.push(CertifiedReferenceWorkspacesViolation::ReportNarrowingWithoutReason {
                    report_id: report.report_id.clone(),
                    state: report.report_state,
                });
            }
        }

        self.validate_report_state_reason_coherence(report, violations);
    }

    fn validate_report_state_reason_coherence(
        &self,
        report: &ReferenceWorkspaceReport,
        violations: &mut Vec<CertifiedReferenceWorkspacesViolation>,
    ) {
        let push_incoherent =
            |violations: &mut Vec<CertifiedReferenceWorkspacesViolation>,
             expected: DowngradeReason| {
                violations.push(
                    CertifiedReferenceWorkspacesViolation::ReportStateReasonIncoherent {
                        report_id: report.report_id.clone(),
                        state: report.report_state,
                        expected_reason: expected,
                    },
                );
            };

        match report.report_state {
            ReportState::Expired => {
                if !report.has_active_reason(DowngradeReason::ReferenceWorkspaceReportStale) {
                    push_incoherent(violations, DowngradeReason::ReferenceWorkspaceReportStale);
                }
            }
            ReportState::Missing => {
                if !report.has_active_reason(DowngradeReason::ReferenceWorkspaceReportMissing) {
                    push_incoherent(violations, DowngradeReason::ReferenceWorkspaceReportMissing);
                }
            }
            ReportState::DueForRefresh | ReportState::Current => {}
        }
    }

    fn validate_matrix_row(
        &self,
        row: &ArchetypePassMatrixRow,
        violations: &mut Vec<CertifiedReferenceWorkspacesViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("title", &row.title),
            ("archetype_ref", &row.archetype_ref),
            ("rationale", &row.rationale),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(CertifiedReferenceWorkspacesViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }

        // No widening: effective_certified may not be true when claimed is false.
        if row.effective_certified && !row.claimed_certified {
            violations.push(
                CertifiedReferenceWorkspacesViolation::EffectiveWiderThanClaimed {
                    row_id: row.row_id.clone(),
                    claimed: row.claimed_certified,
                    effective: row.effective_certified,
                },
            );
        }

        // A narrowing state must drop effective_certified to false and name at
        // least one active reason.
        if row.matrix_state.forces_narrowing() {
            if row.effective_certified {
                violations.push(
                    CertifiedReferenceWorkspacesViolation::EffectiveCertifiedNotNarrowed {
                        row_id: row.row_id.clone(),
                        state: row.matrix_state,
                        effective: row.effective_certified,
                    },
                );
            }
            if row.active_downgrade_reasons.is_empty() {
                violations.push(CertifiedReferenceWorkspacesViolation::NarrowingWithoutReason {
                    row_id: row.row_id.clone(),
                    state: row.matrix_state,
                });
            }
        }

        // A certified state must have owner sign-off, pass criteria, and no
        // active downgrade reasons.
        if row.matrix_state.holds_certified() {
            if row.effective_certified != row.claimed_certified {
                violations.push(
                    CertifiedReferenceWorkspacesViolation::HeldCertifiedNotEqualClaimed {
                        row_id: row.row_id.clone(),
                        claimed: row.claimed_certified,
                        effective: row.effective_certified,
                    },
                );
            }
            if !row.active_downgrade_reasons.is_empty() {
                violations.push(
                    CertifiedReferenceWorkspacesViolation::HeldRowWithActiveDowngrade {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if row.pass_criteria_refs.is_empty() {
                violations.push(
                    CertifiedReferenceWorkspacesViolation::HeldRowWithoutPassCriteria {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(
                    CertifiedReferenceWorkspacesViolation::HeldRowWithoutSignoff {
                        row_id: row.row_id.clone(),
                    },
                );
            }
        }

        self.validate_matrix_state_reason_coherence(row, violations);
    }

    fn validate_matrix_state_reason_coherence(
        &self,
        row: &ArchetypePassMatrixRow,
        violations: &mut Vec<CertifiedReferenceWorkspacesViolation>,
    ) {
        let push_incoherent =
            |violations: &mut Vec<CertifiedReferenceWorkspacesViolation>,
             expected: DowngradeReason| {
                violations.push(CertifiedReferenceWorkspacesViolation::StateReasonIncoherent {
                    row_id: row.row_id.clone(),
                    state: row.matrix_state,
                    expected_reason: expected,
                });
            };

        match row.matrix_state {
            MatrixRowState::NarrowedUnqualified => {
                const ALLOWED: [DowngradeReason; 4] = [
                    DowngradeReason::ArchetypePassMatrixRegression,
                    DowngradeReason::ArchetypeDecertified,
                    DowngradeReason::ReferenceWorkspaceReportManuallyEdited,
                    DowngradeReason::OwnerSignoffMissing,
                ];
                if !ALLOWED.iter().any(|r| row.has_active_reason(*r)) {
                    push_incoherent(violations, DowngradeReason::ArchetypePassMatrixRegression);
                }
            }
            MatrixRowState::NarrowedStale => {
                if !(row.has_active_reason(DowngradeReason::ReferenceWorkspaceReportStale)
                    || row.has_active_reason(DowngradeReason::ReferenceWorkspaceReportMissing))
                {
                    push_incoherent(violations, DowngradeReason::ReferenceWorkspaceReportStale);
                }
            }
            MatrixRowState::NarrowedWaiverExpired => {
                if !row.has_active_reason(DowngradeReason::WaiverExpired) {
                    push_incoherent(violations, DowngradeReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(
                        CertifiedReferenceWorkspacesViolation::WaiverStateWithoutWaiver {
                            row_id: row.row_id.clone(),
                            state: row.matrix_state,
                        },
                    );
                }
            }
            MatrixRowState::ProvisionalOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(
                        CertifiedReferenceWorkspacesViolation::WaiverStateWithoutWaiver {
                            row_id: row.row_id.clone(),
                            state: row.matrix_state,
                        },
                    );
                }
            }
            MatrixRowState::Certified => {}
        }
    }

    fn validate_publication(
        &self,
        violations: &mut Vec<CertifiedReferenceWorkspacesViolation>,
    ) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(CertifiedReferenceWorkspacesViolation::EmptyField {
                id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(CertifiedReferenceWorkspacesViolation::EmptyField {
                id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                CertifiedReferenceWorkspacesViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                CertifiedReferenceWorkspacesViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_row_ids != self.computed_blocking_row_ids() {
            violations.push(
                CertifiedReferenceWorkspacesViolation::PublicationBlockingSetMismatch {
                    field: "blocking_row_ids",
                },
            );
        }
    }
}

/// A redaction-safe export row projected from an archetype pass-matrix row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchetypePassMatrixExportRow {
    /// Stable row id.
    pub row_id: String,
    /// The certified-archetype id.
    pub archetype_ref: String,
    /// Whether the archetype is put forward as certified.
    pub claimed_certified: bool,
    /// Whether the archetype is effectively certified.
    pub effective_certified: bool,
    /// Whether the row holds a certified claim.
    pub holds_certified: bool,
    /// Matrix row state.
    pub matrix_state: MatrixRowState,
    /// Active downgrade reasons.
    pub active_downgrade_reasons: Vec<DowngradeReason>,
}

/// A redaction-safe export row projected from a reference-workspace report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceWorkspaceExportRow {
    /// Stable report id.
    pub report_id: String,
    /// The certified-archetype id.
    pub archetype_ref: String,
    /// Report state.
    pub report_state: ReportState,
    /// Effective state after narrowing.
    pub effective_state: ReportState,
    /// Whether the report backs a Certified claim.
    pub backs_certification: bool,
    /// Active downgrade reasons.
    pub active_downgrade_reasons: Vec<DowngradeReason>,
}

/// A redaction-safe export projection of the artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedReferenceWorkspacesExportProjection {
    /// Artifact id this projection was produced from.
    pub artifact_id: String,
    /// Artifact as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PublicationDecision,
    /// Projected matrix rows.
    pub matrix_rows: Vec<ArchetypePassMatrixExportRow>,
    /// Projected reference-workspace reports.
    pub reference_workspace_reports: Vec<ReferenceWorkspaceExportRow>,
}

/// A validation violation for the certified-reference-workspaces artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertifiedReferenceWorkspacesViolation {
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
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The artifact has no archetype pass-matrix rows.
    EmptyMatrix,
    /// The artifact has no downgrade rules.
    NoDowngradeRules,
    /// A required field is empty.
    EmptyField {
        /// Row, report, rule, or section id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A report id appears more than once.
    DuplicateReportId {
        /// Duplicate report id.
        report_id: String,
    },
    /// A matrix row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A downgrade rule names no states to watch.
    RuleWithoutStates {
        /// Rule id.
        rule_id: String,
    },
    /// A downgrade reason has no rule watching for it.
    DowngradeReasonWithoutRule {
        /// Uncovered reason.
        reason: DowngradeReason,
    },
    /// An effective certified flag is true when claimed is false.
    EffectiveWiderThanClaimed {
        /// Row id.
        row_id: String,
        /// Claimed certified flag.
        claimed: bool,
        /// Effective certified flag.
        effective: bool,
    },
    /// A narrowing matrix state did not drop effective_certified to false.
    EffectiveCertifiedNotNarrowed {
        /// Row id.
        row_id: String,
        /// Matrix row state.
        state: MatrixRowState,
        /// Effective certified flag.
        effective: bool,
    },
    /// A narrowing matrix row state carries no active downgrade reason.
    NarrowingWithoutReason {
        /// Row id.
        row_id: String,
        /// State.
        state: MatrixRowState,
    },
    /// A narrowing report state carries no active downgrade reason.
    ReportNarrowingWithoutReason {
        /// Report id.
        report_id: String,
        /// State.
        state: ReportState,
    },
    /// A held matrix row's effective certified flag differs from its claim.
    HeldCertifiedNotEqualClaimed {
        /// Row id.
        row_id: String,
        /// Claimed certified flag.
        claimed: bool,
        /// Effective certified flag.
        effective: bool,
    },
    /// A held matrix row carries an active downgrade reason.
    HeldRowWithActiveDowngrade {
        /// Row id.
        row_id: String,
    },
    /// A held matrix row names no pass criteria.
    HeldRowWithoutPassCriteria {
        /// Row id.
        row_id: String,
    },
    /// A held matrix row has no owner sign-off.
    HeldRowWithoutSignoff {
        /// Row id.
        row_id: String,
    },
    /// A held report carries an active downgrade reason.
    HeldReportWithActiveDowngrade {
        /// Report id.
        report_id: String,
    },
    /// A held report has no owner sign-off.
    HeldReportWithoutSignoff {
        /// Report id.
        report_id: String,
    },
    /// A narrowing report state did not drop below certification.
    EffectiveStateNotNarrowed {
        /// Report id.
        report_id: String,
        /// Report state.
        state: ReportState,
        /// Effective state.
        effective: ReportState,
    },
    /// A matrix row state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        row_id: String,
        /// State.
        state: MatrixRowState,
        /// Reason the state requires.
        expected_reason: DowngradeReason,
    },
    /// A report state is incoherent with its active reasons.
    ReportStateReasonIncoherent {
        /// Report id.
        report_id: String,
        /// State.
        state: ReportState,
        /// Reason the state requires.
        expected_reason: DowngradeReason,
    },
    /// A waiver-bearing matrix row state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        row_id: String,
        /// State.
        state: MatrixRowState,
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
    /// The summary counts disagree with the rows and reports.
    SummaryMismatch,
}

impl fmt::Display for CertifiedReferenceWorkspacesViolation {
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
            Self::EmptyMatrix => write!(f, "artifact has no archetype pass-matrix rows"),
            Self::NoDowngradeRules => write!(f, "artifact has no downgrade rules"),
            Self::EmptyField { id, field_name } => write!(f, "{id} has empty field {field_name}"),
            Self::DuplicateReportId { report_id } => {
                write!(f, "duplicate report id {report_id}")
            }
            Self::DuplicateRowId { row_id } => write!(f, "duplicate matrix row id {row_id}"),
            Self::DuplicateRuleId { rule_id } => write!(f, "duplicate downgrade rule id {rule_id}"),
            Self::RuleWithoutStates { rule_id } => {
                write!(f, "downgrade rule {rule_id} watches no states")
            }
            Self::DowngradeReasonWithoutRule { reason } => write!(
                f,
                "downgrade reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::EffectiveWiderThanClaimed {
                row_id,
                claimed,
                effective,
            } => write!(
                f,
                "row {row_id} effective certified {effective} is wider than claimed {claimed}"
            ),
            Self::EffectiveCertifiedNotNarrowed {
                row_id,
                state,
                effective,
            } => write!(
                f,
                "row {row_id} state {} must narrow below certified but effective is {effective}",
                state.as_str()
            ),
            Self::NarrowingWithoutReason { row_id, state } => write!(
                f,
                "row {row_id} state {} narrows without naming an active downgrade reason",
                state.as_str()
            ),
            Self::ReportNarrowingWithoutReason { report_id, state } => write!(
                f,
                "report {report_id} state {} narrows without naming an active downgrade reason",
                state.as_str()
            ),
            Self::HeldCertifiedNotEqualClaimed {
                row_id,
                claimed,
                effective,
            } => write!(
                f,
                "row {row_id} holds certified but effective {effective} differs from claimed {claimed}"
            ),
            Self::HeldRowWithActiveDowngrade { row_id } => write!(
                f,
                "row {row_id} holds certified while a downgrade reason is active"
            ),
            Self::HeldRowWithoutPassCriteria { row_id } => {
                write!(f, "row {row_id} holds certified with no pass criteria")
            }
            Self::HeldRowWithoutSignoff { row_id } => {
                write!(f, "row {row_id} holds certified without owner sign-off")
            }
            Self::HeldReportWithActiveDowngrade { report_id } => write!(
                f,
                "report {report_id} is current while a downgrade reason is active"
            ),
            Self::HeldReportWithoutSignoff { report_id } => {
                write!(f, "report {report_id} is current without owner sign-off")
            }
            Self::EffectiveStateNotNarrowed {
                report_id,
                state,
                effective,
            } => write!(
                f,
                "report {report_id} state {} must narrow below certification but effective is {}",
                state.as_str(),
                effective.as_str()
            ),
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
            Self::ReportStateReasonIncoherent {
                report_id,
                state,
                expected_reason,
            } => write!(
                f,
                "report {report_id} state {} requires active reason {}",
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
                "publication {field} disagrees with the firing downgrade rules"
            ),
            Self::SummaryMismatch => write!(
                f,
                "artifact summary counts disagree with the rows and reports"
            ),
        }
    }
}

impl Error for CertifiedReferenceWorkspacesViolation {}

/// Loads the embedded certified-reference-workspaces artifact.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in artifact no longer matches
/// [`CertifiedReferenceWorkspaces`] — including when a row carries a report
/// state, matrix row state, downgrade reason, or action outside the closed
/// vocabularies.
pub fn current_certified_reference_workspaces() -> Result<CertifiedReferenceWorkspaces, serde_json::Error> {
    serde_json::from_str(CERTIFIED_REFERENCE_WORKSPACES_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn artifact() -> CertifiedReferenceWorkspaces {
        current_certified_reference_workspaces().expect("artifact parses")
    }

    #[test]
    fn embedded_artifact_parses_and_validates() {
        let artifact = artifact();
        assert_eq!(
            artifact.schema_version,
            CERTIFIED_REFERENCE_WORKSPACES_SCHEMA_VERSION
        );
        assert_eq!(
            artifact.record_kind,
            CERTIFIED_REFERENCE_WORKSPACES_RECORD_KIND
        );
        assert_eq!(artifact.validate(), Vec::new());
        assert!(!artifact.archetype_pass_matrix_rows.is_empty());
    }

    #[test]
    fn artifact_exercises_certified_and_narrowed_rows() {
        let artifact = artifact();
        assert!(
            !artifact.rows_holding_certified().is_empty(),
            "artifact must certify at least one archetype"
        );
        assert!(
            !artifact.rows_narrowed().is_empty(),
            "artifact must narrow at least one archetype"
        );
    }

    #[test]
    fn summary_counts_match_rows() {
        let artifact = artifact();
        assert_eq!(artifact.summary, artifact.computed_summary());
        assert_eq!(
            artifact.summary.matrix_rows_certified + artifact.summary.matrix_rows_narrowed,
            artifact.archetype_pass_matrix_rows.len()
        );
        assert_eq!(
            artifact.summary.reports_current
                + artifact.summary.reports_due_for_refresh
                + artifact.summary.reports_expired
                + artifact.summary.reports_missing,
            artifact.reference_workspace_reports.len()
        );
    }

    #[test]
    fn publication_holds_when_a_blocking_rule_fires() {
        let artifact = artifact();
        assert_eq!(artifact.publication.decision, PublicationDecision::Hold);
        assert_eq!(
            artifact.publication.decision,
            artifact.computed_publication_decision()
        );
        assert!(!artifact.publication.blocking_rule_ids.is_empty());
        assert!(!artifact.publication.blocking_row_ids.is_empty());
    }

    #[test]
    fn every_downgrade_reason_has_a_rule() {
        let artifact = artifact();
        let covered: BTreeSet<DowngradeReason> = artifact
            .downgrade_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in DowngradeReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn validate_flags_a_held_row_with_active_downgrade() {
        let mut artifact = artifact();
        let row = artifact
            .archetype_pass_matrix_rows
            .iter_mut()
            .find(|r| r.holds_certified())
            .expect("a certified row exists");
        row.active_downgrade_reasons
            .push(DowngradeReason::ReferenceWorkspaceReportStale);
        let row_id = row.row_id.clone();
        artifact.summary = artifact.computed_summary();
        assert!(artifact
            .validate()
            .contains(&CertifiedReferenceWorkspacesViolation::HeldRowWithActiveDowngrade {
                row_id
            }));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut artifact = artifact();
        let row = artifact
            .archetype_pass_matrix_rows
            .iter_mut()
            .find(|r| r.matrix_state == MatrixRowState::NarrowedStale)
            .expect("a narrowed-stale row exists");
        row.effective_certified = true;
        artifact.summary = artifact.computed_summary();
        artifact.publication.decision = artifact.computed_publication_decision();
        artifact.publication.blocking_rule_ids = artifact.computed_blocking_rule_ids();
        artifact.publication.blocking_row_ids = artifact.computed_blocking_row_ids();
        assert!(artifact.validate().iter().any(|v| matches!(
            v,
            CertifiedReferenceWorkspacesViolation::EffectiveCertifiedNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut artifact = artifact();
        artifact.publication.decision = PublicationDecision::Proceed;
        assert!(artifact.validate().iter().any(|v| matches!(
            v,
            CertifiedReferenceWorkspacesViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let artifact = artifact();
        let projection = artifact.support_export_projection();
        assert_eq!(
            projection.matrix_rows.len(),
            artifact.archetype_pass_matrix_rows.len()
        );
        assert_eq!(
            projection.reference_workspace_reports.len(),
            artifact.reference_workspace_reports.len()
        );
        assert_eq!(
            projection.publication_decision,
            artifact.publication.decision
        );
        for (row, projected) in artifact.archetype_pass_matrix_rows.iter().zip(&projection.matrix_rows) {
            assert_eq!(row.row_id, projected.row_id);
            assert_eq!(row.holds_certified(), projected.holds_certified);
            assert_eq!(row.effective_certified, projected.effective_certified);
        }
    }
}
