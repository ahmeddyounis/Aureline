//! Typed validator for M5 benchmark/help/migration component fixtures.
//!
//! This module covers the benchmark-evidence-card lane from
//! `.plans/M05-773.md`: rendered performance claims must carry the workflow,
//! measured value versus budget, run/capture environment, freshness, source
//! class, caveats, and export refs needed to support the claim outside the UI.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Supported benchmark evidence-card schema version.
pub const M5_BENCHMARK_EVIDENCE_CARD_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for benchmark evidence cards.
pub const M5_BENCHMARK_EVIDENCE_CARD_RECORD_KIND: &str = "m5_benchmark_evidence_card";

/// Repo-relative schema ref.
pub const M5_BENCHMARK_EVIDENCE_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-benchmark-evidence-card.schema.json";

/// Canonical lab/reference fixture.
pub const M5_BENCHMARK_EVIDENCE_CARD_FIXTURE_REF: &str =
    "fixtures/ui/m5-benchmark-help-migration-components/benchmark_evidence_card.json";

/// Embedded lab/reference fixture.
pub const M5_BENCHMARK_EVIDENCE_CARD_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-benchmark-help-migration-components/benchmark_evidence_card.json"
));

const M5_BENCHMARK_EVIDENCE_CARD_SELF_CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-benchmark-help-migration-components/benchmark_evidence_card_self_capture.json"
));

const M5_BENCHMARK_EVIDENCE_CARD_DESIGN_PARTNER_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-benchmark-help-migration-components/benchmark_evidence_card_design_partner.json"
));

const M5_BENCHMARK_EVIDENCE_CARD_COMMUNITY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-benchmark-help-migration-components/benchmark_evidence_card_community.json"
));

const M5_BENCHMARK_EVIDENCE_CARD_IMPORTED_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-benchmark-help-migration-components/benchmark_evidence_card_imported.json"
));

/// Source classes the card must distinguish before rendering or exporting a
/// measured claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkEvidenceSourceClass {
    /// Aureline-controlled reference lab run.
    LabReferenceRun,
    /// User/local capture. Useful, but not reference proof.
    SelfCapture,
    /// Design-partner result with bounded/redacted context.
    DesignPartnerResult,
    /// Community-reported result.
    CommunityReport,
    /// Imported benchmark evidence from another packet/tool.
    ImportedEvidence,
    /// Methodology-only claim with no measured proof.
    MethodologyOnly,
}

impl BenchmarkEvidenceSourceClass {
    /// Source classes required by M05-773 acceptance criteria.
    pub const REQUIRED_PROOF_CLASSES: [Self; 5] = [
        Self::LabReferenceRun,
        Self::SelfCapture,
        Self::DesignPartnerResult,
        Self::CommunityReport,
        Self::ImportedEvidence,
    ];
}

/// Scope of the benchmark claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkClaimScope {
    /// Methodology without a measured claim.
    MethodologyOnly,
    /// Aureline reference result only.
    AurelineOnlyReference,
    /// Head-to-head comparison.
    HeadToHeadComparison,
    /// Workflow-level measured claim.
    WorkflowClaim,
}

/// Cold/warm run state shown on the card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColdWarmState {
    /// Cold run.
    Cold,
    /// Warm run.
    Warm,
    /// Mixed cold/warm evidence.
    Mixed,
    /// Not applicable to this claim.
    NotApplicable,
}

/// Power mode or policy state for the run/capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PowerMode {
    /// Plugged in.
    PluggedIn,
    /// On battery.
    Battery,
    /// Low-power mode.
    LowPower,
    /// Performance mode.
    Performance,
    /// Managed policy determines power posture.
    ManagedPolicy,
    /// Unknown power posture.
    Unknown,
}

/// Local/remote scope of the measured workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionScope {
    /// Local-only execution.
    LocalOnly,
    /// Remote attached to a local UI/session.
    RemoteAttached,
    /// Managed remote execution.
    ManagedRemote,
    /// Mixed local/remote evidence.
    Mixed,
}

/// Freshness posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkFreshnessState {
    /// Current evidence.
    Current,
    /// Warm cached evidence.
    WarmCached,
    /// Stale evidence.
    Stale,
    /// Retest is pending.
    RetestPending,
    /// Expired evidence.
    Expired,
    /// Quarantined evidence.
    Quarantined,
}

impl BenchmarkFreshnessState {
    const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// Claim downgrade/narrowing state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkDowngradeState {
    /// No downgrade.
    None,
    /// Methodology only.
    MethodologyOnly,
    /// Narrowed to internal/local truth.
    NarrowedToInternal,
    /// Retest is pending.
    RetestPending,
    /// Quarantined evidence.
    Quarantined,
    /// Unsupported claim.
    Unsupported,
}

impl BenchmarkDowngradeState {
    const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }
}

/// Explicit degraded state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkDegradedState {
    /// No degradation.
    None,
    /// Stale benchmark evidence.
    StaleBenchmarkEvidence,
    /// Missing reproduction pack.
    MissingReproductionPack,
    /// Hardware cannot be compared.
    IncomparableHardware,
    /// Corpus is narrowed.
    NarrowedCorpus,
    /// Self-capture evidence only.
    SelfCaptureOnly,
    /// Design-partner result has bounded/redacted context.
    DesignPartnerLimited,
    /// Community evidence is unverified.
    CommunityUnverified,
    /// Imported evidence is unverified.
    ImportedEvidenceUnverified,
}

impl BenchmarkDegradedState {
    const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }
}

/// Metric comparison basis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkComparisonBasis {
    /// Current reference baseline.
    CurrentBaseline,
    /// Prior release.
    PriorRelease,
    /// Head-to-head comparison.
    HeadToHead,
    /// Not comparable.
    NotComparable,
    /// Methodology only.
    MethodologyOnly,
}

/// Compare view mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkCompareMode {
    /// Single claim card.
    SingleClaim,
    /// Compare to prior release.
    PriorRelease,
    /// Head-to-head compare.
    HeadToHead,
    /// Compare self capture to reference.
    SelfToReference,
    /// Compare community report to reference.
    CommunityToReference,
}

/// Downgrade banner label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkDowngradeBannerLabel {
    /// No banner.
    None,
    /// Current.
    Current,
    /// Stale, retest pending.
    StaleRetestPending,
    /// Methodology-only.
    MethodologyOnly,
    /// Incomparable.
    Incomparable,
    /// Self-capture only.
    SelfCaptureOnly,
    /// Design-partner limited.
    DesignPartnerLimited,
    /// Community unverified.
    CommunityUnverified,
    /// Imported evidence unverified.
    ImportedEvidenceUnverified,
    /// Quarantined.
    Quarantined,
}

/// One metric row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkMetricRow {
    /// Stable metric id.
    pub metric_id: String,
    /// Human-readable label.
    pub label: String,
    /// Measured value.
    pub value_repr: String,
    /// Budget value.
    pub budget_value_repr: String,
    /// Unit label.
    pub unit_label: String,
    /// Threshold ref.
    pub threshold_ref: String,
    /// Comparison basis.
    pub comparison_basis: BenchmarkComparisonBasis,
}

/// Compare-view truth included in the card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkCompareView {
    /// Compare mode.
    pub compare_mode: BenchmarkCompareMode,
    /// Baseline ref.
    pub baseline_ref: String,
    /// Comparison basis.
    pub comparison_basis: BenchmarkComparisonBasis,
    /// Whether this claim can be compared to the baseline.
    pub comparable: bool,
    /// Caveats visible in the compare view.
    pub caveat_summary_refs: Vec<String>,
}

/// Visible downgrade banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkDowngradeBanner {
    /// Whether the banner is shown.
    pub shown: bool,
    /// Banner label.
    pub label: BenchmarkDowngradeBannerLabel,
    /// Summary ref.
    pub summary_ref: String,
    /// Reason refs.
    pub reason_refs: Vec<String>,
}

/// Trace/report export parity for the card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkTraceReportExport {
    /// Trace ref.
    pub trace_ref: String,
    /// Report ref.
    pub report_ref: String,
    /// Export formats.
    pub formats: Vec<String>,
    /// Benchmark id is included in trace/report export.
    pub includes_benchmark_id: bool,
    /// Caveat summaries are included in trace/report export.
    pub includes_caveat_summaries: bool,
    /// Workflow and budget truth are included.
    pub includes_workflow_budget_truth: bool,
    /// Environment truth is included.
    pub includes_environment_truth: bool,
}

/// Copy/export payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkCopyExport {
    /// Copy formats.
    pub formats: Vec<String>,
    /// Stable exported fields.
    pub export_fields: Vec<String>,
    /// Plain text projection.
    pub text: String,
    /// JSON projection.
    pub json: String,
    /// Markdown projection.
    pub markdown: String,
    /// Screenshot-only explanations are prohibited.
    pub screenshot_only_prohibited: bool,
}

/// Reusable benchmark evidence card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkEvidenceCard {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable card id.
    pub card_id: String,
    /// Stable benchmark id.
    pub benchmark_id: String,
    /// Claim ref.
    pub claim_ref: String,
    /// Claim scope.
    pub claim_scope: BenchmarkClaimScope,
    /// Evidence source class.
    pub evidence_source_class: BenchmarkEvidenceSourceClass,
    /// Workflow ref.
    pub workflow_ref: String,
    /// Budget ref.
    pub budget_ref: String,
    /// Measured value.
    pub measured_value_repr: String,
    /// Budget value.
    pub budget_value_repr: String,
    /// Corpus ref.
    pub corpus_ref: String,
    /// Hardware or capture source ref.
    pub hardware_or_capture_ref: String,
    /// Cold/warm state.
    pub cold_warm_state: ColdWarmState,
    /// Sample size.
    pub sample_size: u32,
    /// Extension set ref.
    pub extension_set_ref: String,
    /// Power mode.
    pub power_mode: PowerMode,
    /// Execution scope.
    pub execution_scope: ExecutionScope,
    /// As-of date (`YYYY-MM-DD`).
    pub as_of_date: String,
    /// Metric rows.
    pub metric_rows: Vec<BenchmarkMetricRow>,
    /// Compare-view truth.
    pub compare_view: BenchmarkCompareView,
    /// Freshness state.
    pub freshness_state: BenchmarkFreshnessState,
    /// Downgrade state.
    pub downgrade_state: BenchmarkDowngradeState,
    /// Degraded state.
    pub degraded_state: BenchmarkDegradedState,
    /// Visible downgrade banner.
    pub downgrade_banner: BenchmarkDowngradeBanner,
    /// Caveat summary refs.
    pub caveat_summary_refs: Vec<String>,
    /// Trace/report export truth.
    pub trace_report_export: BenchmarkTraceReportExport,
    /// Canonical source refs.
    pub source_refs: Vec<String>,
    /// First consumer surfaces.
    pub consumer_surfaces: Vec<String>,
    /// Copy/export payloads.
    pub copy_export: BenchmarkCopyExport,
}

impl BenchmarkEvidenceCard {
    /// Validate one card's non-schema invariants.
    pub fn validate(&self) -> Vec<BenchmarkEvidenceCardViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_required_truth(&mut violations);
        self.validate_compare_and_downgrade(&mut violations);
        self.validate_export_parity(&mut violations);
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<BenchmarkEvidenceCardViolation>) {
        if self.record_kind != M5_BENCHMARK_EVIDENCE_CARD_RECORD_KIND {
            violations.push(BenchmarkEvidenceCardViolation::UnsupportedRecordKind {
                card_id: self.card_id.clone(),
            });
        }
        if self.schema_version != M5_BENCHMARK_EVIDENCE_CARD_SCHEMA_VERSION {
            violations.push(BenchmarkEvidenceCardViolation::UnsupportedSchemaVersion {
                card_id: self.card_id.clone(),
            });
        }
        for (field, value) in [
            ("card_id", &self.card_id),
            ("benchmark_id", &self.benchmark_id),
            ("claim_ref", &self.claim_ref),
            ("workflow_ref", &self.workflow_ref),
            ("budget_ref", &self.budget_ref),
            ("measured_value_repr", &self.measured_value_repr),
            ("budget_value_repr", &self.budget_value_repr),
            ("corpus_ref", &self.corpus_ref),
            ("hardware_or_capture_ref", &self.hardware_or_capture_ref),
            ("extension_set_ref", &self.extension_set_ref),
            ("as_of_date", &self.as_of_date),
        ] {
            if value.trim().is_empty() {
                violations.push(BenchmarkEvidenceCardViolation::EmptyField {
                    card_id: self.card_id.clone(),
                    field,
                });
            }
        }
    }

    fn validate_required_truth(&self, violations: &mut Vec<BenchmarkEvidenceCardViolation>) {
        if self.sample_size == 0 {
            violations.push(BenchmarkEvidenceCardViolation::MissingWorkflowBudgetTruth {
                card_id: self.card_id.clone(),
                field: "sample_size",
            });
        }
        if !is_yyyy_mm_dd(&self.as_of_date) {
            violations.push(BenchmarkEvidenceCardViolation::InvalidAsOfDate {
                card_id: self.card_id.clone(),
            });
        }
        if self.metric_rows.is_empty() {
            violations.push(BenchmarkEvidenceCardViolation::MissingMetricRows {
                card_id: self.card_id.clone(),
            });
        }
        for row in &self.metric_rows {
            if row.value_repr.trim().is_empty() || row.budget_value_repr.trim().is_empty() {
                violations.push(BenchmarkEvidenceCardViolation::MissingMetricBudgetTruth {
                    card_id: self.card_id.clone(),
                    metric_id: row.metric_id.clone(),
                });
            }
        }
    }

    fn validate_compare_and_downgrade(&self, violations: &mut Vec<BenchmarkEvidenceCardViolation>) {
        if !self.compare_view.comparable
            && !matches!(
                self.compare_view.comparison_basis,
                BenchmarkComparisonBasis::NotComparable | BenchmarkComparisonBasis::MethodologyOnly
            )
        {
            violations.push(
                BenchmarkEvidenceCardViolation::IncomparableViewClaimsComparableBasis {
                    card_id: self.card_id.clone(),
                },
            );
        }
        if self.compare_view.caveat_summary_refs.is_empty() {
            violations.push(BenchmarkEvidenceCardViolation::MissingCompareCaveats {
                card_id: self.card_id.clone(),
            });
        }

        let requires_banner = !self.freshness_state.is_current()
            || !self.downgrade_state.is_none()
            || !self.degraded_state.is_none()
            || self.evidence_source_class != BenchmarkEvidenceSourceClass::LabReferenceRun
            || !self.compare_view.comparable;
        if requires_banner && !self.downgrade_banner.shown {
            violations.push(BenchmarkEvidenceCardViolation::MissingDowngradeBanner {
                card_id: self.card_id.clone(),
            });
        }

        if self.evidence_source_class != BenchmarkEvidenceSourceClass::LabReferenceRun
            && self.degraded_state.is_none()
        {
            violations.push(
                BenchmarkEvidenceCardViolation::NonReferenceSourceNotNarrowed {
                    card_id: self.card_id.clone(),
                    source_class: self.evidence_source_class,
                },
            );
        }
    }

    fn validate_export_parity(&self, violations: &mut Vec<BenchmarkEvidenceCardViolation>) {
        for format in ["text", "json", "markdown"] {
            if !self.copy_export.formats.iter().any(|f| f == format) {
                violations.push(BenchmarkEvidenceCardViolation::MissingCopyFormat {
                    card_id: self.card_id.clone(),
                    format,
                });
            }
        }
        for field in REQUIRED_COPY_EXPORT_FIELDS {
            if !self.copy_export.export_fields.iter().any(|f| f == field) {
                violations.push(BenchmarkEvidenceCardViolation::MissingCopyExportField {
                    card_id: self.card_id.clone(),
                    field,
                });
            }
        }
        if !self.copy_export.screenshot_only_prohibited {
            violations.push(
                BenchmarkEvidenceCardViolation::ScreenshotOnlyExportAllowed {
                    card_id: self.card_id.clone(),
                },
            );
        }
        if !(self.trace_report_export.includes_benchmark_id
            && self.trace_report_export.includes_caveat_summaries
            && self.trace_report_export.includes_workflow_budget_truth
            && self.trace_report_export.includes_environment_truth)
        {
            violations.push(
                BenchmarkEvidenceCardViolation::TraceReportExportDropsRequiredTruth {
                    card_id: self.card_id.clone(),
                },
            );
        }
        let copy_text = format!(
            "{}\n{}\n{}",
            self.copy_export.text, self.copy_export.json, self.copy_export.markdown
        );
        if !copy_text.contains(&self.benchmark_id) {
            violations.push(BenchmarkEvidenceCardViolation::CopyExportDropsBenchmarkId {
                card_id: self.card_id.clone(),
            });
        }
        for caveat in &self.caveat_summary_refs {
            if !copy_text.contains(caveat) {
                violations.push(BenchmarkEvidenceCardViolation::CopyExportDropsCaveat {
                    card_id: self.card_id.clone(),
                    caveat_ref: caveat.clone(),
                });
            }
        }
    }
}

const REQUIRED_COPY_EXPORT_FIELDS: &[&str] = &[
    "benchmark_id",
    "claim_ref",
    "claim_scope",
    "evidence_source_class",
    "workflow_ref",
    "budget_ref",
    "measured_value_repr",
    "budget_value_repr",
    "corpus_ref",
    "hardware_or_capture_ref",
    "cold_warm_state",
    "sample_size",
    "extension_set_ref",
    "power_mode",
    "execution_scope",
    "as_of_date",
    "compare_view",
    "freshness_state",
    "downgrade_state",
    "degraded_state",
    "caveat_summary_refs",
    "trace_report_export",
    "metric_rows",
];

/// Parse the canonical benchmark card.
pub fn current_benchmark_evidence_card() -> Result<BenchmarkEvidenceCard, serde_json::Error> {
    serde_json::from_str(M5_BENCHMARK_EVIDENCE_CARD_JSON)
}

/// Parse all benchmark card fixtures used to prove source-class coverage.
pub fn current_benchmark_evidence_cards() -> Result<Vec<BenchmarkEvidenceCard>, serde_json::Error> {
    [
        M5_BENCHMARK_EVIDENCE_CARD_JSON,
        M5_BENCHMARK_EVIDENCE_CARD_SELF_CAPTURE_JSON,
        M5_BENCHMARK_EVIDENCE_CARD_DESIGN_PARTNER_JSON,
        M5_BENCHMARK_EVIDENCE_CARD_COMMUNITY_JSON,
        M5_BENCHMARK_EVIDENCE_CARD_IMPORTED_JSON,
    ]
    .into_iter()
    .map(serde_json::from_str)
    .collect()
}

/// Validate source-class coverage and all individual card invariants.
pub fn validate_benchmark_evidence_cards(
    cards: &[BenchmarkEvidenceCard],
) -> Vec<BenchmarkEvidenceCardViolation> {
    let mut violations = Vec::new();
    let classes: BTreeSet<_> = cards
        .iter()
        .map(|card| card.evidence_source_class)
        .collect();
    for required in BenchmarkEvidenceSourceClass::REQUIRED_PROOF_CLASSES {
        if !classes.contains(&required) {
            violations.push(BenchmarkEvidenceCardViolation::MissingEvidenceSourceClass {
                source_class: required,
            });
        }
    }
    let mut ids = BTreeSet::new();
    for card in cards {
        if !ids.insert(card.benchmark_id.clone()) {
            violations.push(BenchmarkEvidenceCardViolation::DuplicateBenchmarkId {
                benchmark_id: card.benchmark_id.clone(),
            });
        }
        violations.extend(card.validate());
    }
    violations
}

fn is_yyyy_mm_dd(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(idx, byte)| idx == 4 || idx == 7 || byte.is_ascii_digit())
}

/// Validation errors for benchmark evidence cards.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchmarkEvidenceCardViolation {
    /// Unsupported record kind.
    UnsupportedRecordKind {
        /// Card id.
        card_id: String,
    },
    /// Unsupported schema version.
    UnsupportedSchemaVersion {
        /// Card id.
        card_id: String,
    },
    /// Required text field is empty.
    EmptyField {
        /// Card id.
        card_id: String,
        /// Field name.
        field: &'static str,
    },
    /// Required workflow/budget field is missing.
    MissingWorkflowBudgetTruth {
        /// Card id.
        card_id: String,
        /// Field name.
        field: &'static str,
    },
    /// Invalid as-of date.
    InvalidAsOfDate {
        /// Card id.
        card_id: String,
    },
    /// No metric rows.
    MissingMetricRows {
        /// Card id.
        card_id: String,
    },
    /// Metric row does not show both measured and budget values.
    MissingMetricBudgetTruth {
        /// Card id.
        card_id: String,
        /// Metric id.
        metric_id: String,
    },
    /// An incomparable view uses a comparable basis.
    IncomparableViewClaimsComparableBasis {
        /// Card id.
        card_id: String,
    },
    /// Compare view omitted caveats.
    MissingCompareCaveats {
        /// Card id.
        card_id: String,
    },
    /// Required downgrade banner omitted.
    MissingDowngradeBanner {
        /// Card id.
        card_id: String,
    },
    /// Non-reference source class was not narrowed/degraded.
    NonReferenceSourceNotNarrowed {
        /// Card id.
        card_id: String,
        /// Source class.
        source_class: BenchmarkEvidenceSourceClass,
    },
    /// Copy format missing.
    MissingCopyFormat {
        /// Card id.
        card_id: String,
        /// Format name.
        format: &'static str,
    },
    /// Copy export field missing.
    MissingCopyExportField {
        /// Card id.
        card_id: String,
        /// Field name.
        field: &'static str,
    },
    /// Screenshot-only export would be allowed.
    ScreenshotOnlyExportAllowed {
        /// Card id.
        card_id: String,
    },
    /// Trace/report export drops required truth.
    TraceReportExportDropsRequiredTruth {
        /// Card id.
        card_id: String,
    },
    /// Copy/export dropped benchmark id.
    CopyExportDropsBenchmarkId {
        /// Card id.
        card_id: String,
    },
    /// Copy/export dropped a caveat summary.
    CopyExportDropsCaveat {
        /// Card id.
        card_id: String,
        /// Caveat ref.
        caveat_ref: String,
    },
    /// Fixture set does not prove a required source class.
    MissingEvidenceSourceClass {
        /// Source class.
        source_class: BenchmarkEvidenceSourceClass,
    },
    /// Duplicate benchmark id.
    DuplicateBenchmarkId {
        /// Duplicate id.
        benchmark_id: String,
    },
}
