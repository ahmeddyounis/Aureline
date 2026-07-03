//! Typed validator for M5 benchmark/help/migration component fixtures.
//!
//! This module covers the benchmark-evidence-card lane from
//! `.plans/M05-773.md`: rendered performance claims must carry the workflow,
//! measured value versus budget, run/capture environment, freshness, source
//! class, caveats, and export refs needed to support the claim outside the UI.
//! It also covers the About/service-health and support-package lanes from
//! `.plans/M05-774.md`, where build facts, cached service health, local
//! diagnostics, and local-save-submit-later truth must remain available without
//! sign-in or browser handoff. The community-handoff tile lane from
//! `.plans/M05-775.md` keeps official, authenticated, community, vendor, and
//! local-only destinations visibly distinct before any browser handoff.

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

/// Supported About/service-health card schema version.
pub const M5_ABOUT_SERVICE_HEALTH_CARD_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for About/service-health cards.
pub const M5_ABOUT_SERVICE_HEALTH_CARD_RECORD_KIND: &str = "m5_about_service_health_card";

/// Repo-relative About/service-health schema ref.
pub const M5_ABOUT_SERVICE_HEALTH_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-about-service-health-card.schema.json";

/// Canonical About/service-health fixture.
pub const M5_ABOUT_SERVICE_HEALTH_CARD_FIXTURE_REF: &str =
    "fixtures/ui/m5-benchmark-help-migration-components/about_service_health_card.json";

/// Embedded About/service-health fixture.
pub const M5_ABOUT_SERVICE_HEALTH_CARD_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-benchmark-help-migration-components/about_service_health_card.json"
));

/// Supported support-package card schema version.
pub const M5_SUPPORT_PACKAGE_CARD_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for support-package cards.
pub const M5_SUPPORT_PACKAGE_CARD_RECORD_KIND: &str = "m5_support_package_card";

/// Repo-relative support-package schema ref.
pub const M5_SUPPORT_PACKAGE_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-support-package-card.schema.json";

/// Canonical support-package fixture.
pub const M5_SUPPORT_PACKAGE_CARD_FIXTURE_REF: &str =
    "fixtures/ui/m5-benchmark-help-migration-components/support_package_card.json";

/// Embedded support-package fixture.
pub const M5_SUPPORT_PACKAGE_CARD_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-benchmark-help-migration-components/support_package_card.json"
));

/// Embedded lab/reference fixture.
pub const M5_BENCHMARK_EVIDENCE_CARD_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-benchmark-help-migration-components/benchmark_evidence_card.json"
));

/// Supported community-handoff tile schema version.
pub const M5_COMMUNITY_HANDOFF_TILE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for community-handoff tiles.
pub const M5_COMMUNITY_HANDOFF_TILE_RECORD_KIND: &str = "m5_community_handoff_tile";

/// Repo-relative community-handoff tile schema ref.
pub const M5_COMMUNITY_HANDOFF_TILE_SCHEMA_REF: &str =
    "schemas/ui/m5-community-handoff-tile.schema.json";

/// Canonical community-handoff fixture.
pub const M5_COMMUNITY_HANDOFF_TILE_FIXTURE_REF: &str =
    "fixtures/ui/m5-benchmark-help-migration-components/community_handoff_tile.json";

/// Embedded canonical community-owned fixture.
pub const M5_COMMUNITY_HANDOFF_TILE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-benchmark-help-migration-components/community_handoff_tile.json"
));

const M5_COMMUNITY_HANDOFF_TILE_OFFICIAL_PUBLIC_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-benchmark-help-migration-components/community_handoff_tile_official_public.json"
));

const M5_COMMUNITY_HANDOFF_TILE_OFFICIAL_AUTHENTICATED_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-benchmark-help-migration-components/community_handoff_tile_official_authenticated.json"
));

const M5_COMMUNITY_HANDOFF_TILE_VENDOR_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-benchmark-help-migration-components/community_handoff_tile_vendor.json"
));

const M5_COMMUNITY_HANDOFF_TILE_LOCAL_ONLY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-benchmark-help-migration-components/community_handoff_tile_local_only.json"
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

/// About/service-health card family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AboutServiceHealthCardFamily {
    /// About summary card.
    AboutSummary,
    /// Service-health banner.
    ServiceHealthBanner,
    /// Service-health status card.
    ServiceHealthStatusCard,
}

/// Service contract state shown to users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceContractState {
    /// Service is ready.
    Ready,
    /// Managed/service feature is degraded.
    Degraded,
    /// Surface is local-only.
    LocalOnly,
    /// Health is stale.
    Stale,
    /// Contract mismatch.
    ContractMismatch,
    /// Policy blocked.
    PolicyBlocked,
    /// Unavailable.
    Unavailable,
}

impl ServiceContractState {
    const fn is_ready(self) -> bool {
        matches!(self, Self::Ready)
    }
}

/// Trust class for service-health/build state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AboutSourceTrustClass {
    /// Official live/source truth.
    Official,
    /// Mirrored official truth.
    MirroredOfficial,
    /// Local-only source.
    LocalOnly,
    /// Managed service source.
    ManagedService,
    /// Community-owned source.
    CommunityOwned,
    /// Vendor-managed source.
    VendorManaged,
    /// Unknown source.
    Unknown,
}

/// Service-health freshness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceFreshnessState {
    /// Live status.
    Live,
    /// Cached status.
    Cached,
    /// Mirrored status.
    Mirrored,
    /// Offline-pack status.
    OfflinePack,
    /// Stale cached status.
    StaleCache,
    /// Policy-limited status.
    PolicyLimited,
}

impl ServiceFreshnessState {
    const fn is_live(self) -> bool {
        matches!(self, Self::Live)
    }
}

/// Local continuity state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalContinuityState {
    /// Local workflows remain available.
    Available,
    /// Local workflows are narrowed.
    Narrowed,
    /// Local workflows are unavailable.
    Unavailable,
    /// Not applicable.
    NotApplicable,
}

impl LocalContinuityState {
    const fn has_local_path(self) -> bool {
        matches!(self, Self::Available | Self::Narrowed)
    }
}

/// About/service-health downgrade state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AboutDowngradeState {
    /// No downgrade.
    None,
    /// Cached service health.
    CachedServiceHealth,
    /// Local-only continuity.
    LocalOnlyContinuity,
    /// Service degraded.
    ServiceDegraded,
    /// Policy limited.
    PolicyLimited,
    /// Unavailable.
    Unavailable,
}

impl AboutDowngradeState {
    const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }
}

/// Release channel shown on About summary cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseChannel {
    /// Stable channel.
    Stable,
    /// Beta channel.
    Beta,
    /// Nightly channel.
    Nightly,
    /// Enterprise channel.
    Enterprise,
    /// Local dev build.
    LocalDev,
    /// Unknown channel.
    Unknown,
}

/// Install mode shown on About summary cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallMode {
    /// Local application install.
    LocalApp,
    /// Self-hosted install.
    SelfHosted,
    /// Managed cloud.
    ManagedCloud,
    /// Air-gapped install.
    AirGapped,
    /// Portable install.
    Portable,
    /// Unknown install mode.
    Unknown,
}

/// Provenance state shown on About summary cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildProvenanceState {
    /// Verified provenance.
    Verified,
    /// Mirrored and verified provenance.
    MirroredVerified,
    /// Local-only provenance.
    LocalOnly,
    /// Unsigned build.
    Unsigned,
    /// Stale provenance.
    Stale,
    /// Unknown provenance.
    Unknown,
}

/// Local action kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalActionKind {
    /// Copy build info.
    CopyBuildInfo,
    /// Open local diagnostics.
    OpenLocalDiagnostics,
    /// Export diagnostics.
    ExportDiagnostics,
}

/// Action that must not force sign-in or browser handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalAction {
    /// Stable action id.
    pub action_id: String,
    /// Action kind.
    pub action_kind: LocalActionKind,
    /// User-facing label.
    pub label: String,
    /// Whether the action requires auth.
    pub requires_auth: bool,
    /// Whether the action opens a browser.
    pub opens_browser: bool,
    /// Whether available locally.
    pub local_available: bool,
}

impl LocalAction {
    fn is_local_first(&self) -> bool {
        !self.requires_auth && !self.opens_browser && self.local_available
    }
}

/// Build summary rendered on About/help surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildSummary {
    /// Version string.
    pub version: String,
    /// Release channel.
    pub channel: ReleaseChannel,
    /// Install mode.
    pub install_mode: InstallMode,
    /// Provenance state.
    pub provenance_state: BuildProvenanceState,
    /// Open/local boundary note.
    pub open_local_boundary_note: String,
    /// Copy-build-info action.
    pub copy_build_info_action: LocalAction,
}

/// Service-health summary rendered on banners/status cards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceHealthSummary {
    /// Affected service family.
    pub affected_service_family: String,
    /// Current contract state.
    pub current_contract_state: ServiceContractState,
    /// Cached/freshness state.
    pub cached_freshness: ServiceFreshnessState,
    /// Local workflows that still work.
    pub local_workflows_available: Vec<String>,
    /// Managed features affected by the degraded service.
    pub managed_features_affected: Vec<String>,
    /// Diagnostics action.
    pub diagnostics_action: LocalAction,
    /// Export action.
    pub export_action: LocalAction,
}

/// Copy/export payload for About/service-health and support-package cards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentCopyExport {
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

/// Reusable About/service-health card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AboutServiceHealthCard {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable card id.
    pub card_id: String,
    /// Card family.
    pub card_family: AboutServiceHealthCardFamily,
    /// Exact build identity ref.
    pub build_identity_ref: String,
    /// Service family.
    pub service_family: String,
    /// Service contract state.
    pub service_contract_state: ServiceContractState,
    /// Source trust class.
    pub source_trust_class: AboutSourceTrustClass,
    /// Freshness state.
    pub freshness_state: ServiceFreshnessState,
    /// Local continuity state.
    pub local_continuity_state: LocalContinuityState,
    /// Downgrade state.
    pub downgrade_state: AboutDowngradeState,
    /// Build summary.
    pub build_summary: BuildSummary,
    /// Service-health summary.
    pub service_health_summary: ServiceHealthSummary,
    /// Canonical source refs.
    pub source_refs: Vec<String>,
    /// First consumer surfaces.
    pub consumer_surfaces: Vec<String>,
    /// Copy/export payload.
    pub copy_export: ComponentCopyExport,
}

impl AboutServiceHealthCard {
    /// Validate one card's non-schema invariants.
    pub fn validate(&self) -> Vec<AboutServiceHealthCardViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_ABOUT_SERVICE_HEALTH_CARD_RECORD_KIND {
            violations.push(AboutServiceHealthCardViolation::UnsupportedRecordKind {
                card_id: self.card_id.clone(),
            });
        }
        if self.schema_version != M5_ABOUT_SERVICE_HEALTH_CARD_SCHEMA_VERSION {
            violations.push(AboutServiceHealthCardViolation::UnsupportedSchemaVersion {
                card_id: self.card_id.clone(),
            });
        }
        for (field, value) in [
            ("card_id", &self.card_id),
            ("build_identity_ref", &self.build_identity_ref),
            ("service_family", &self.service_family),
            ("version", &self.build_summary.version),
            (
                "open_local_boundary_note",
                &self.build_summary.open_local_boundary_note,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(AboutServiceHealthCardViolation::EmptyField {
                    card_id: self.card_id.clone(),
                    field,
                });
            }
        }

        if self.build_summary.copy_build_info_action.action_kind != LocalActionKind::CopyBuildInfo {
            violations.push(
                AboutServiceHealthCardViolation::MissingCopyBuildInfoAction {
                    card_id: self.card_id.clone(),
                },
            );
        }
        for action in [
            &self.build_summary.copy_build_info_action,
            &self.service_health_summary.diagnostics_action,
            &self.service_health_summary.export_action,
        ] {
            if !action.is_local_first() {
                violations.push(AboutServiceHealthCardViolation::ForcedAuthOrBrowserAction {
                    card_id: self.card_id.clone(),
                    action_id: action.action_id.clone(),
                });
            }
        }
        if self.service_health_summary.affected_service_family != self.service_family {
            violations.push(AboutServiceHealthCardViolation::ServiceFamilyMismatch {
                card_id: self.card_id.clone(),
            });
        }
        if self.service_health_summary.current_contract_state != self.service_contract_state
            || self.service_health_summary.cached_freshness != self.freshness_state
        {
            violations.push(AboutServiceHealthCardViolation::HealthSummaryMismatch {
                card_id: self.card_id.clone(),
            });
        }

        let degraded = !self.service_contract_state.is_ready() || !self.freshness_state.is_live();
        if degraded && self.downgrade_state.is_none() {
            violations.push(AboutServiceHealthCardViolation::MissingDowngradeState {
                card_id: self.card_id.clone(),
            });
        }
        if degraded {
            if !self.local_continuity_state.has_local_path()
                || self
                    .service_health_summary
                    .local_workflows_available
                    .is_empty()
            {
                violations.push(AboutServiceHealthCardViolation::MissingLocalContinuity {
                    card_id: self.card_id.clone(),
                });
            }
            if self
                .service_health_summary
                .managed_features_affected
                .is_empty()
            {
                violations.push(AboutServiceHealthCardViolation::MissingManagedImpact {
                    card_id: self.card_id.clone(),
                });
            }
        }

        validate_component_copy_export(
            &self.card_id,
            &self.copy_export,
            ABOUT_SERVICE_HEALTH_COPY_EXPORT_FIELDS,
            &mut violations,
            |card_id, format| AboutServiceHealthCardViolation::MissingCopyFormat {
                card_id,
                format,
            },
            |card_id, field| AboutServiceHealthCardViolation::MissingCopyExportField {
                card_id,
                field,
            },
            |card_id| AboutServiceHealthCardViolation::ScreenshotOnlyExportAllowed { card_id },
        );
        let copy_text = self.copy_text();
        for required in [
            self.build_identity_ref.as_str(),
            self.service_family.as_str(),
            self.build_summary.version.as_str(),
            "local",
            "diagnostics",
            "export",
        ] {
            if !copy_text.contains(required) {
                violations.push(
                    AboutServiceHealthCardViolation::CopyExportDropsRequiredTruth {
                        card_id: self.card_id.clone(),
                        token: required.to_owned(),
                    },
                );
            }
        }
        violations
    }

    fn copy_text(&self) -> String {
        format!(
            "{}\n{}\n{}",
            self.copy_export.text, self.copy_export.json, self.copy_export.markdown
        )
    }
}

const ABOUT_SERVICE_HEALTH_COPY_EXPORT_FIELDS: &[&str] = &[
    "card_family",
    "build_identity_ref",
    "service_family",
    "service_contract_state",
    "source_trust_class",
    "freshness_state",
    "local_continuity_state",
    "downgrade_state",
    "version",
    "channel",
    "install_mode",
    "provenance_state",
    "open_local_boundary_note",
    "copy_build_info_action",
    "local_workflows_available",
    "diagnostics_action",
    "export_action",
];

/// Parse the canonical About/service-health card.
pub fn current_about_service_health_card() -> Result<AboutServiceHealthCard, serde_json::Error> {
    serde_json::from_str(M5_ABOUT_SERVICE_HEALTH_CARD_JSON)
}

/// Validation errors for About/service-health cards.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AboutServiceHealthCardViolation {
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
    /// Copy-build-info action is missing or wrong.
    MissingCopyBuildInfoAction {
        /// Card id.
        card_id: String,
    },
    /// Action forces sign-in, browser handoff, or is not locally available.
    ForcedAuthOrBrowserAction {
        /// Card id.
        card_id: String,
        /// Action id.
        action_id: String,
    },
    /// Service summary names a different service family.
    ServiceFamilyMismatch {
        /// Card id.
        card_id: String,
    },
    /// Health summary does not preserve state/freshness.
    HealthSummaryMismatch {
        /// Card id.
        card_id: String,
    },
    /// Degraded/cached health omitted downgrade state.
    MissingDowngradeState {
        /// Card id.
        card_id: String,
    },
    /// Degraded/cached health omitted local continuity.
    MissingLocalContinuity {
        /// Card id.
        card_id: String,
    },
    /// Degraded/cached health omitted affected managed features.
    MissingManagedImpact {
        /// Card id.
        card_id: String,
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
    /// Copy/export dropped a required token.
    CopyExportDropsRequiredTruth {
        /// Card id.
        card_id: String,
        /// Missing token.
        token: String,
    },
}

/// Support package state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportPackageState {
    /// Ready for review.
    ReviewReady,
    /// Narrowed review.
    NarrowedReview,
    /// Send blocked.
    SendBlocked,
    /// Saved locally only.
    SavedLocalOnly,
    /// Submitted.
    Submitted,
    /// Stale schema.
    StaleSchema,
}

/// Support destination class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportDestinationClass {
    /// Local-only review.
    LocalOnlyReview,
    /// Vendor case handoff.
    VendorCaseHandoff,
    /// User-initiated upload.
    UserInitiatedUpload,
    /// Managed admin handoff.
    ManagedAdminHandoff,
    /// Private security channel.
    PrivateSecurityChannel,
    /// Official support.
    OfficialSupport,
}

/// Support trust class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportTrustClass {
    /// Local-only.
    LocalOnly,
    /// Vendor-managed.
    VendorManaged,
    /// User-chosen upload.
    UserChosenUpload,
    /// Managed admin.
    ManagedAdmin,
    /// Private security.
    PrivateSecurity,
    /// Official authenticated.
    OfficialAuthenticated,
}

/// Local save state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalSaveState {
    /// Local save is first-class available.
    FirstClassAvailable,
    /// Already saved local-only.
    SavedLocalOnly,
    /// Not available.
    NotAvailable,
}

/// Redaction state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionState {
    /// Default-safe redaction.
    DefaultSafe,
    /// Policy narrowed.
    PolicyNarrowed,
    /// User broadened.
    UserBroadened,
    /// High risk blocked.
    BlockedHighRisk,
    /// Stale schema.
    StaleSchema,
}

/// Counts by diagnostic data class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataClassCounts {
    /// Metadata-only count.
    pub metadata_only: u32,
    /// Environment-adjacent count.
    pub environment_adjacent: u32,
    /// Code-adjacent count.
    pub code_adjacent: u32,
    /// High-risk count.
    pub high_risk: u32,
}

/// Support package content kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageContentKind {
    /// Build info.
    BuildInfo,
    /// Service-health snapshot.
    ServiceHealthSnapshot,
    /// Diagnostic summary.
    DiagnosticSummary,
    /// Redaction manifest.
    RedactionManifest,
    /// Import preview.
    ImportPreview,
    /// Migration diff.
    MigrationDiff,
    /// Policy receipts.
    PolicyReceipts,
    /// Local log metadata.
    LocalLogsMetadata,
    /// Reproduction steps.
    ReproductionSteps,
}

/// Local-save submit state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubmitState {
    /// Not submitted.
    NotSubmitted,
    /// Submit-ready after review.
    SubmitReadyAfterReview,
    /// Submitted.
    Submitted,
    /// Blocked.
    Blocked,
}

/// Local-save summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalSaveSummary {
    /// Whether saved to local store.
    pub saved_to_local_store: bool,
    /// Local packet ref.
    pub local_packet_ref: String,
    /// Submit state.
    pub submit_state: SubmitState,
    /// Whether auth is required to inspect locally.
    pub requires_auth_to_inspect: bool,
}

/// Redaction/export state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportState {
    /// Local export ready.
    LocalExportReady,
    /// Off-machine handoff blocked.
    OffMachineBlocked,
    /// Submitted.
    Submitted,
    /// Stale schema.
    StaleSchema,
}

/// Redaction/export summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RedactionExportSummary {
    /// Redaction state.
    pub redaction_state: RedactionState,
    /// Export state.
    pub export_state: ExportState,
    /// Whether high-risk data is excluded.
    pub high_risk_excluded: bool,
    /// Whether policy-locked exclusions are visible.
    pub policy_locked_exclusions_visible: bool,
}

/// Submit-later summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubmitLaterSummary {
    /// Later submit only happens after user action.
    pub would_submit_only_after_user_action: bool,
    /// Later destination class.
    pub would_send_destination_class: SupportDestinationClass,
    /// Later submit requires inspection.
    pub would_require_inspection: bool,
    /// Whether the current card represents a submission.
    pub current_card_represents_submission: bool,
    /// Whether browser opens before local review.
    pub opens_browser_before_local_review: bool,
}

/// Reusable support-package card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportPackageCard {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable package/card id.
    pub package_id: String,
    /// Support package ref.
    pub support_package_ref: String,
    /// Package state.
    pub package_state: SupportPackageState,
    /// Destination class.
    pub destination_class: SupportDestinationClass,
    /// Trust class.
    pub trust_class: SupportTrustClass,
    /// Local save state.
    pub local_save_state: LocalSaveState,
    /// Redaction state.
    pub redaction_state: RedactionState,
    /// Included data counts.
    pub included_counts: DataClassCounts,
    /// Excluded data counts.
    pub excluded_counts: DataClassCounts,
    /// Policy-locked data counts.
    pub policy_locked_counts: DataClassCounts,
    /// Whether inspection is required before submit.
    pub inspect_before_submit_required: bool,
    /// Package contents.
    pub package_contents: Vec<PackageContentKind>,
    /// Local-save summary.
    pub local_save_summary: LocalSaveSummary,
    /// Redaction/export summary.
    pub redaction_export_summary: RedactionExportSummary,
    /// Submit-later summary.
    pub submit_later_summary: SubmitLaterSummary,
    /// Canonical source refs.
    pub source_refs: Vec<String>,
    /// First consumer surfaces.
    pub consumer_surfaces: Vec<String>,
    /// Copy/export payload.
    pub copy_export: ComponentCopyExport,
}

impl SupportPackageCard {
    /// Validate one support package card's non-schema invariants.
    pub fn validate(&self) -> Vec<SupportPackageCardViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_SUPPORT_PACKAGE_CARD_RECORD_KIND {
            violations.push(SupportPackageCardViolation::UnsupportedRecordKind {
                package_id: self.package_id.clone(),
            });
        }
        if self.schema_version != M5_SUPPORT_PACKAGE_CARD_SCHEMA_VERSION {
            violations.push(SupportPackageCardViolation::UnsupportedSchemaVersion {
                package_id: self.package_id.clone(),
            });
        }
        for (field, value) in [
            ("package_id", &self.package_id),
            ("support_package_ref", &self.support_package_ref),
            (
                "local_packet_ref",
                &self.local_save_summary.local_packet_ref,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(SupportPackageCardViolation::EmptyField {
                    package_id: self.package_id.clone(),
                    field,
                });
            }
        }
        if self.package_contents.is_empty() {
            violations.push(SupportPackageCardViolation::MissingPackageContents {
                package_id: self.package_id.clone(),
            });
        }
        if self.redaction_export_summary.redaction_state != self.redaction_state {
            violations.push(SupportPackageCardViolation::RedactionSummaryMismatch {
                package_id: self.package_id.clone(),
            });
        }
        if self.policy_locked_counts.code_adjacent > 0 || self.policy_locked_counts.high_risk > 0 {
            if !self
                .redaction_export_summary
                .policy_locked_exclusions_visible
            {
                violations.push(SupportPackageCardViolation::PolicyLockedExclusionsHidden {
                    package_id: self.package_id.clone(),
                });
            }
        }
        if self.excluded_counts.high_risk > 0 && !self.redaction_export_summary.high_risk_excluded {
            violations.push(SupportPackageCardViolation::HighRiskExclusionMismatch {
                package_id: self.package_id.clone(),
            });
        }
        if self.destination_class != SupportDestinationClass::LocalOnlyReview
            && !self.inspect_before_submit_required
        {
            violations.push(
                SupportPackageCardViolation::OffMachineSubmitSkipsInspection {
                    package_id: self.package_id.clone(),
                },
            );
        }
        if self.package_state == SupportPackageState::SavedLocalOnly {
            if self.destination_class != SupportDestinationClass::LocalOnlyReview
                || self.trust_class != SupportTrustClass::LocalOnly
                || self.local_save_state != LocalSaveState::SavedLocalOnly
                || !self.local_save_summary.saved_to_local_store
                || self.local_save_summary.submit_state != SubmitState::NotSubmitted
                || self.local_save_summary.requires_auth_to_inspect
            {
                violations.push(SupportPackageCardViolation::SavedLocalOnlyNotLocalFirst {
                    package_id: self.package_id.clone(),
                });
            }
            if self.submit_later_summary.current_card_represents_submission
                || !self
                    .submit_later_summary
                    .would_submit_only_after_user_action
                || !self.submit_later_summary.would_require_inspection
                || self.submit_later_summary.opens_browser_before_local_review
            {
                violations.push(SupportPackageCardViolation::SubmitLaterTruthCollapsed {
                    package_id: self.package_id.clone(),
                });
            }
        }

        validate_component_copy_export(
            &self.package_id,
            &self.copy_export,
            SUPPORT_PACKAGE_COPY_EXPORT_FIELDS,
            &mut violations,
            |package_id, format| SupportPackageCardViolation::MissingCopyFormat {
                package_id,
                format,
            },
            |package_id, field| SupportPackageCardViolation::MissingCopyExportField {
                package_id,
                field,
            },
            |package_id| SupportPackageCardViolation::ScreenshotOnlyExportAllowed { package_id },
        );
        let copy_text = format!(
            "{}\n{}\n{}",
            self.copy_export.text, self.copy_export.json, self.copy_export.markdown
        );
        for required in [
            self.support_package_ref.as_str(),
            self.local_save_summary.local_packet_ref.as_str(),
            "saved_local_only",
            "not_submitted",
            "explicit user action",
        ] {
            if !copy_text.contains(required) {
                violations.push(SupportPackageCardViolation::CopyExportDropsRequiredTruth {
                    package_id: self.package_id.clone(),
                    token: required.to_owned(),
                });
            }
        }
        violations
    }
}

const SUPPORT_PACKAGE_COPY_EXPORT_FIELDS: &[&str] = &[
    "support_package_ref",
    "package_state",
    "destination_class",
    "trust_class",
    "local_save_state",
    "redaction_state",
    "included_counts",
    "excluded_counts",
    "policy_locked_counts",
    "inspect_before_submit_required",
    "package_contents",
    "local_save_summary",
    "redaction_export_summary",
    "submit_later_summary",
];

/// Parse the canonical support-package card.
pub fn current_support_package_card() -> Result<SupportPackageCard, serde_json::Error> {
    serde_json::from_str(M5_SUPPORT_PACKAGE_CARD_JSON)
}

/// Validation errors for support-package cards.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupportPackageCardViolation {
    /// Unsupported record kind.
    UnsupportedRecordKind {
        /// Package id.
        package_id: String,
    },
    /// Unsupported schema version.
    UnsupportedSchemaVersion {
        /// Package id.
        package_id: String,
    },
    /// Required text field is empty.
    EmptyField {
        /// Package id.
        package_id: String,
        /// Field name.
        field: &'static str,
    },
    /// Package contents are missing.
    MissingPackageContents {
        /// Package id.
        package_id: String,
    },
    /// Redaction summary diverges from card redaction state.
    RedactionSummaryMismatch {
        /// Package id.
        package_id: String,
    },
    /// Policy-locked exclusions are hidden.
    PolicyLockedExclusionsHidden {
        /// Package id.
        package_id: String,
    },
    /// High-risk exclusion summary diverges from counts.
    HighRiskExclusionMismatch {
        /// Package id.
        package_id: String,
    },
    /// Off-machine submit skips inspection.
    OffMachineSubmitSkipsInspection {
        /// Package id.
        package_id: String,
    },
    /// Saved-local-only package is not local-first.
    SavedLocalOnlyNotLocalFirst {
        /// Package id.
        package_id: String,
    },
    /// Save-local and submit-later truth collapsed.
    SubmitLaterTruthCollapsed {
        /// Package id.
        package_id: String,
    },
    /// Copy format missing.
    MissingCopyFormat {
        /// Package id.
        package_id: String,
        /// Format name.
        format: &'static str,
    },
    /// Copy export field missing.
    MissingCopyExportField {
        /// Package id.
        package_id: String,
        /// Field name.
        field: &'static str,
    },
    /// Screenshot-only export would be allowed.
    ScreenshotOnlyExportAllowed {
        /// Package id.
        package_id: String,
    },
    /// Copy/export dropped a required token.
    CopyExportDropsRequiredTruth {
        /// Package id.
        package_id: String,
        /// Missing token.
        token: String,
    },
}

/// Governed outbound route shown by a handoff tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommunityHandoffRoute {
    /// Public issue route.
    PublicIssue,
    /// Private security disclosure route.
    SecurityDisclosure,
    /// Documentation feedback route.
    DocsFeedback,
    /// RFC/discussion route.
    RfcDiscussion,
    /// Community support route.
    CommunitySupport,
    /// Official support route.
    OfficialSupport,
    /// Extension or vendor-owned support route.
    ExtensionVendorSupport,
    /// Local draft route.
    LocalDraft,
}

impl CommunityHandoffRoute {
    const fn as_str(self) -> &'static str {
        match self {
            Self::PublicIssue => "public_issue",
            Self::SecurityDisclosure => "security_disclosure",
            Self::DocsFeedback => "docs_feedback",
            Self::RfcDiscussion => "rfc_discussion",
            Self::CommunitySupport => "community_support",
            Self::OfficialSupport => "official_support",
            Self::ExtensionVendorSupport => "extension_vendor_support",
            Self::LocalDraft => "local_draft",
        }
    }

    const fn expects_issue_template_action(self) -> bool {
        matches!(
            self,
            Self::PublicIssue | Self::DocsFeedback | Self::RfcDiscussion | Self::CommunitySupport
        )
    }
}

/// Help destination grouping lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffDestinationGroup {
    /// Help/About lane.
    Help,
    /// Release lane.
    Release,
    /// Migration lane.
    Migration,
    /// Support lane.
    Support,
    /// Community lane.
    Community,
    /// Extension/vendor lane.
    ExtensionVendor,
    /// Local-only lane.
    LocalOnly,
}

/// Destination type shown on the tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffDestinationType {
    /// Public issue template.
    PublicIssueTemplate,
    /// Official documentation feedback.
    DocsFeedback,
    /// Release notes or release proof destination.
    ReleaseNotes,
    /// Migration guidance or migration-center handoff.
    MigrationGuidance,
    /// Authenticated support intake.
    SupportIntake,
    /// Security disclosure.
    SecurityDisclosure,
    /// Community discussion.
    CommunityDiscussion,
    /// Community support forum.
    CommunitySupport,
    /// Extension/vendor support.
    ExtensionVendorSupport,
    /// Local draft/save-later target.
    LocalDraft,
}

/// Destination ownership class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffOwnershipClass {
    /// First-party official destination.
    Official,
    /// Community-owned destination.
    Community,
    /// Private security destination.
    PrivateSecurity,
    /// Official authenticated destination.
    OfficialAuthenticated,
    /// Vendor-managed destination.
    VendorManaged,
    /// Local-only destination.
    LocalOnly,
}

impl HandoffOwnershipClass {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Official => "official",
            Self::Community => "community",
            Self::PrivateSecurity => "private_security",
            Self::OfficialAuthenticated => "official_authenticated",
            Self::VendorManaged => "vendor_managed",
            Self::LocalOnly => "local_only",
        }
    }
}

/// Destination trust class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffTrustClass {
    /// Official public destination.
    OfficialPublic,
    /// Official authenticated destination.
    OfficialAuthenticated,
    /// Community destination.
    Community,
    /// Private security destination.
    PrivateSecurity,
    /// Vendor-managed destination.
    VendorManaged,
    /// Local-only destination.
    LocalOnly,
}

impl HandoffTrustClass {
    /// Trust classes required by M05-775.
    pub const REQUIRED_DESTINATION_CLASSES: [Self; 5] = [
        Self::OfficialPublic,
        Self::OfficialAuthenticated,
        Self::Community,
        Self::VendorManaged,
        Self::LocalOnly,
    ];

    const fn as_str(self) -> &'static str {
        match self {
            Self::OfficialPublic => "official_public",
            Self::OfficialAuthenticated => "official_authenticated",
            Self::Community => "community",
            Self::PrivateSecurity => "private_security",
            Self::VendorManaged => "vendor_managed",
            Self::LocalOnly => "local_only",
        }
    }
}

/// Visibility boundary shown before exit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffVisibilityBoundary {
    /// World-readable public surface.
    WorldReadablePublic,
    /// Official account-visible surface.
    OfficialAccountVisible,
    /// Community-visible surface.
    CommunityVisible,
    /// Private security channel.
    PrivateSecurityChannel,
    /// Vendor-visible surface.
    VendorVisible,
    /// Local data never leaves the product.
    LocalNeverLeaves,
}

/// Auth expectation shown before exit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffAuthExpectation {
    /// No account needed.
    NoAccountNeeded,
    /// Official account required.
    OfficialAccountRequired,
    /// Community account typical.
    CommunityAccountTypical,
    /// Security-channel credential.
    SecurityChannelCredential,
    /// Vendor account required.
    VendorAccountRequired,
    /// Local, no network.
    LocalNoNetwork,
}

/// Data-exit boundary shown before exit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffDataExitBoundary {
    /// No payload leaves the product.
    NoPayloadLeavesProduct,
    /// Metadata-safe object refs only.
    MetadataSafeObjectRefs,
    /// Proposal refs only.
    ProposalRefsOnly,
    /// Redacted support packet.
    RedactedSupportPacket,
    /// Security payloads only.
    SecurityPayloadsOnly,
    /// External public browse only.
    ExternalPublicBrowse,
    /// Vendor or third-party outbound handoff.
    VendorOrThirdPartyOutbound,
}

/// Commitment class shown before exit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffCommitmentClass {
    /// Official supported commitment.
    OfficialSupportedCommitment,
    /// Best-effort community destination.
    BestEffortCommunity,
    /// No-commitment public forum.
    NoCommitmentPublicForum,
    /// Security handled privately.
    SecurityHandledPrivately,
    /// Vendor-owned destination with no Aureline product commitment.
    VendorOwnedNoProductCommitment,
    /// Local draft, no delivery.
    LocalDraftNoDelivery,
}

/// Destination reachability state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffDestinationState {
    /// Ready to open.
    Ready,
    /// Browser launch was blocked.
    BrowserBlocked,
    /// Offline.
    Offline,
    /// Policy blocked.
    PolicyBlocked,
    /// Cached target is stale.
    StaleCachedTarget,
    /// Unsupported profile.
    UnsupportedProfile,
}

impl HandoffDestinationState {
    const fn requires_continuity(self) -> bool {
        !matches!(self, Self::Ready)
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::BrowserBlocked => "browser_blocked",
            Self::Offline => "offline",
            Self::PolicyBlocked => "policy_blocked",
            Self::StaleCachedTarget => "stale_cached_target",
            Self::UnsupportedProfile => "unsupported_profile",
        }
    }
}

/// Version-awareness posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionAwarenessState {
    /// Current version-aware destination.
    Current,
    /// Version-specific destination.
    VersionSpecific,
    /// Cached or offline version note.
    CachedOrOffline,
    /// Not applicable.
    NotApplicable,
}

/// Handoff tile action kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffActionKind {
    /// Open destination.
    OpenDestination,
    /// Copy handoff context.
    CopyContext,
    /// Open issue template.
    OpenIssueTemplate,
    /// Copy issue template.
    CopyIssueTemplate,
    /// Export local draft.
    ExportLocalDraft,
}

impl HandoffActionKind {
    const fn is_copy_or_export(self) -> bool {
        matches!(
            self,
            Self::CopyContext | Self::CopyIssueTemplate | Self::ExportLocalDraft
        )
    }

    const fn is_issue_template(self) -> bool {
        matches!(self, Self::OpenIssueTemplate | Self::CopyIssueTemplate)
    }

    const fn is_open_external(self) -> bool {
        matches!(self, Self::OpenDestination | Self::OpenIssueTemplate)
    }
}

/// Open/copy/export action displayed on the tile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffAction {
    /// Stable action id.
    pub action_id: String,
    /// Action kind.
    pub action_kind: HandoffActionKind,
    /// User-facing label.
    pub label: String,
    /// Whether the action preserves destination identity.
    pub preserves_destination_identity: bool,
    /// Whether the action preserves trust-class context.
    pub preserves_trust_class_context: bool,
    /// Whether the action is available offline/cached.
    pub available_offline: bool,
    /// Whether the action opens an external surface.
    pub opens_external: bool,
}

/// Reusable community/help handoff tile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommunityHandoffTile {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable tile id.
    pub tile_id: String,
    /// Help destination group.
    pub destination_group: HandoffDestinationGroup,
    /// Destination type.
    pub destination_type: HandoffDestinationType,
    /// Governed route.
    pub route: CommunityHandoffRoute,
    /// Ownership class.
    pub ownership_class: HandoffOwnershipClass,
    /// Trust class.
    pub trust_class: HandoffTrustClass,
    /// Version-awareness state.
    pub version_awareness_state: VersionAwarenessState,
    /// Version-awareness note.
    pub version_awareness_note: String,
    /// Visibility boundary.
    pub visibility_boundary: HandoffVisibilityBoundary,
    /// Auth expectation.
    pub auth_expectation: HandoffAuthExpectation,
    /// Data-exit boundary.
    pub data_exit_boundary: HandoffDataExitBoundary,
    /// Commitment class.
    pub commitment_class: HandoffCommitmentClass,
    /// Destination state.
    pub destination_state: HandoffDestinationState,
    /// Whether review is required before leaving the product.
    pub pre_exit_review_required: bool,
    /// Local-safe fallback ref.
    pub local_safe_fallback_ref: String,
    /// Whether the destination is community-owned.
    pub community_owned_destination: bool,
    /// Available actions.
    pub actions: Vec<HandoffAction>,
    /// Canonical source refs.
    pub source_refs: Vec<String>,
    /// First consumer surfaces.
    pub consumer_surfaces: Vec<String>,
    /// Copy/export payload.
    pub copy_export: ComponentCopyExport,
}

impl CommunityHandoffTile {
    /// Validate one tile's non-schema invariants.
    pub fn validate(&self) -> Vec<CommunityHandoffTileViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_COMMUNITY_HANDOFF_TILE_RECORD_KIND {
            violations.push(CommunityHandoffTileViolation::UnsupportedRecordKind {
                tile_id: self.tile_id.clone(),
            });
        }
        if self.schema_version != M5_COMMUNITY_HANDOFF_TILE_SCHEMA_VERSION {
            violations.push(CommunityHandoffTileViolation::UnsupportedSchemaVersion {
                tile_id: self.tile_id.clone(),
            });
        }
        for (field, value) in [
            ("tile_id", &self.tile_id),
            ("version_awareness_note", &self.version_awareness_note),
            ("local_safe_fallback_ref", &self.local_safe_fallback_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(CommunityHandoffTileViolation::EmptyField {
                    tile_id: self.tile_id.clone(),
                    field,
                });
            }
        }
        if self.actions.is_empty() {
            violations.push(CommunityHandoffTileViolation::MissingAction {
                tile_id: self.tile_id.clone(),
            });
        }

        if expected_trust_class(self.ownership_class) != self.trust_class {
            violations.push(CommunityHandoffTileViolation::TrustClassMismatch {
                tile_id: self.tile_id.clone(),
                ownership_class: self.ownership_class,
                trust_class: self.trust_class,
            });
        }
        if self.community_owned_destination
            != matches!(self.ownership_class, HandoffOwnershipClass::Community)
        {
            violations.push(
                CommunityHandoffTileViolation::CommunityOwnershipFlagMismatch {
                    tile_id: self.tile_id.clone(),
                },
            );
        }
        if !matches!(self.trust_class, HandoffTrustClass::LocalOnly)
            && !self.pre_exit_review_required
        {
            violations.push(CommunityHandoffTileViolation::MissingPreExitReview {
                tile_id: self.tile_id.clone(),
            });
        }
        if matches!(self.trust_class, HandoffTrustClass::LocalOnly) {
            if self.data_exit_boundary != HandoffDataExitBoundary::NoPayloadLeavesProduct
                || self.visibility_boundary != HandoffVisibilityBoundary::LocalNeverLeaves
                || self.auth_expectation != HandoffAuthExpectation::LocalNoNetwork
            {
                violations.push(CommunityHandoffTileViolation::LocalOnlyBoundaryWidened {
                    tile_id: self.tile_id.clone(),
                });
            }
        }
        if matches!(
            self.trust_class,
            HandoffTrustClass::Community | HandoffTrustClass::VendorManaged
        ) && self.commitment_class == HandoffCommitmentClass::OfficialSupportedCommitment
        {
            violations.push(
                CommunityHandoffTileViolation::UnofficialCommitmentOverclaimed {
                    tile_id: self.tile_id.clone(),
                },
            );
        }
        if self.destination_state.requires_continuity()
            && !self.actions.iter().any(|action| {
                action.action_kind.is_copy_or_export()
                    && action.available_offline
                    && action.preserves_destination_identity
                    && action.preserves_trust_class_context
            })
        {
            violations.push(CommunityHandoffTileViolation::MissingBlockedContinuity {
                tile_id: self.tile_id.clone(),
                destination_state: self.destination_state,
            });
        }
        if self.route.expects_issue_template_action()
            && !self
                .actions
                .iter()
                .any(|action| action.action_kind.is_issue_template())
        {
            violations.push(CommunityHandoffTileViolation::MissingIssueTemplateAction {
                tile_id: self.tile_id.clone(),
            });
        }
        for action in &self.actions {
            if !action.preserves_destination_identity || !action.preserves_trust_class_context {
                violations.push(CommunityHandoffTileViolation::ActionDropsHandoffContext {
                    tile_id: self.tile_id.clone(),
                    action_id: action.action_id.clone(),
                });
            }
            if action.action_kind.is_copy_or_export() && !action.available_offline {
                violations.push(
                    CommunityHandoffTileViolation::CopyActionUnavailableOffline {
                        tile_id: self.tile_id.clone(),
                        action_id: action.action_id.clone(),
                    },
                );
            }
            if matches!(self.trust_class, HandoffTrustClass::LocalOnly)
                && action.action_kind.is_open_external()
            {
                violations.push(
                    CommunityHandoffTileViolation::LocalOnlyActionOpensExternal {
                        tile_id: self.tile_id.clone(),
                        action_id: action.action_id.clone(),
                    },
                );
            }
        }

        validate_component_copy_export(
            &self.tile_id,
            &self.copy_export,
            COMMUNITY_HANDOFF_TILE_COPY_EXPORT_FIELDS,
            &mut violations,
            |tile_id, format| CommunityHandoffTileViolation::MissingCopyFormat { tile_id, format },
            |tile_id, field| CommunityHandoffTileViolation::MissingCopyExportField {
                tile_id,
                field,
            },
            |tile_id| CommunityHandoffTileViolation::ScreenshotOnlyExportAllowed { tile_id },
        );

        let copy_text = format!(
            "{}\n{}\n{}",
            self.copy_export.text, self.copy_export.json, self.copy_export.markdown
        );
        for required in [
            self.route.as_str(),
            self.ownership_class.as_str(),
            self.trust_class.as_str(),
            self.destination_state.as_str(),
            self.local_safe_fallback_ref.as_str(),
            "version",
        ] {
            if !copy_text.contains(required) {
                violations.push(
                    CommunityHandoffTileViolation::CopyExportDropsRequiredTruth {
                        tile_id: self.tile_id.clone(),
                        token: required.to_owned(),
                    },
                );
            }
        }
        violations
    }
}

const COMMUNITY_HANDOFF_TILE_COPY_EXPORT_FIELDS: &[&str] = &[
    "destination_group",
    "destination_type",
    "route",
    "ownership_class",
    "trust_class",
    "version_awareness_state",
    "version_awareness_note",
    "visibility_boundary",
    "auth_expectation",
    "data_exit_boundary",
    "commitment_class",
    "destination_state",
    "pre_exit_review_required",
    "local_safe_fallback_ref",
    "community_owned_destination",
    "actions",
];

const REQUIRED_HANDOFF_GROUPS: &[HandoffDestinationGroup] = &[
    HandoffDestinationGroup::Help,
    HandoffDestinationGroup::Release,
    HandoffDestinationGroup::Migration,
    HandoffDestinationGroup::Support,
];

const fn expected_trust_class(ownership_class: HandoffOwnershipClass) -> HandoffTrustClass {
    match ownership_class {
        HandoffOwnershipClass::Official => HandoffTrustClass::OfficialPublic,
        HandoffOwnershipClass::Community => HandoffTrustClass::Community,
        HandoffOwnershipClass::PrivateSecurity => HandoffTrustClass::PrivateSecurity,
        HandoffOwnershipClass::OfficialAuthenticated => HandoffTrustClass::OfficialAuthenticated,
        HandoffOwnershipClass::VendorManaged => HandoffTrustClass::VendorManaged,
        HandoffOwnershipClass::LocalOnly => HandoffTrustClass::LocalOnly,
    }
}

/// Parse the canonical community-handoff tile.
pub fn current_community_handoff_tile() -> Result<CommunityHandoffTile, serde_json::Error> {
    serde_json::from_str(M5_COMMUNITY_HANDOFF_TILE_JSON)
}

/// Parse all checked-in community-handoff tile fixtures.
pub fn current_community_handoff_tiles() -> Result<Vec<CommunityHandoffTile>, serde_json::Error> {
    [
        M5_COMMUNITY_HANDOFF_TILE_OFFICIAL_PUBLIC_JSON,
        M5_COMMUNITY_HANDOFF_TILE_OFFICIAL_AUTHENTICATED_JSON,
        M5_COMMUNITY_HANDOFF_TILE_JSON,
        M5_COMMUNITY_HANDOFF_TILE_VENDOR_JSON,
        M5_COMMUNITY_HANDOFF_TILE_LOCAL_ONLY_JSON,
    ]
    .into_iter()
    .map(serde_json::from_str)
    .collect()
}

/// Validate a fixture set for required route/trust coverage.
pub fn validate_community_handoff_tiles(
    tiles: &[CommunityHandoffTile],
) -> Vec<CommunityHandoffTileViolation> {
    let mut violations = Vec::new();
    let mut seen = BTreeSet::new();
    let mut trust_classes = BTreeSet::new();
    let mut groups = BTreeSet::new();
    for tile in tiles {
        if !seen.insert(tile.tile_id.clone()) {
            violations.push(CommunityHandoffTileViolation::DuplicateTileId {
                tile_id: tile.tile_id.clone(),
            });
        }
        trust_classes.insert(tile.trust_class);
        groups.insert(tile.destination_group);
        violations.extend(tile.validate());
    }
    for trust_class in HandoffTrustClass::REQUIRED_DESTINATION_CLASSES {
        if !trust_classes.contains(&trust_class) {
            violations.push(CommunityHandoffTileViolation::MissingTrustClass { trust_class });
        }
    }
    for group in REQUIRED_HANDOFF_GROUPS {
        if !groups.contains(group) {
            violations.push(CommunityHandoffTileViolation::MissingDestinationGroup {
                destination_group: *group,
            });
        }
    }
    violations
}

/// Validation errors for community-handoff tiles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommunityHandoffTileViolation {
    /// Unsupported record kind.
    UnsupportedRecordKind {
        /// Tile id.
        tile_id: String,
    },
    /// Unsupported schema version.
    UnsupportedSchemaVersion {
        /// Tile id.
        tile_id: String,
    },
    /// Required text field is empty.
    EmptyField {
        /// Tile id.
        tile_id: String,
        /// Field name.
        field: &'static str,
    },
    /// No action is present.
    MissingAction {
        /// Tile id.
        tile_id: String,
    },
    /// Ownership and trust class diverge.
    TrustClassMismatch {
        /// Tile id.
        tile_id: String,
        /// Ownership class.
        ownership_class: HandoffOwnershipClass,
        /// Trust class.
        trust_class: HandoffTrustClass,
    },
    /// Community flag diverges from ownership class.
    CommunityOwnershipFlagMismatch {
        /// Tile id.
        tile_id: String,
    },
    /// Off-product tile omitted pre-exit review.
    MissingPreExitReview {
        /// Tile id.
        tile_id: String,
    },
    /// Local-only destination widened its boundary.
    LocalOnlyBoundaryWidened {
        /// Tile id.
        tile_id: String,
    },
    /// Community/vendor destination claimed official support.
    UnofficialCommitmentOverclaimed {
        /// Tile id.
        tile_id: String,
    },
    /// Browser-blocked/offline/policy-blocked target lacks copy/export continuity.
    MissingBlockedContinuity {
        /// Tile id.
        tile_id: String,
        /// Destination state.
        destination_state: HandoffDestinationState,
    },
    /// Issue/report route lacks issue-template action parity.
    MissingIssueTemplateAction {
        /// Tile id.
        tile_id: String,
    },
    /// Action drops identity or trust-class context.
    ActionDropsHandoffContext {
        /// Tile id.
        tile_id: String,
        /// Action id.
        action_id: String,
    },
    /// Copy/export action is unavailable offline.
    CopyActionUnavailableOffline {
        /// Tile id.
        tile_id: String,
        /// Action id.
        action_id: String,
    },
    /// Local-only action opens external destination.
    LocalOnlyActionOpensExternal {
        /// Tile id.
        tile_id: String,
        /// Action id.
        action_id: String,
    },
    /// Copy format missing.
    MissingCopyFormat {
        /// Tile id.
        tile_id: String,
        /// Format name.
        format: &'static str,
    },
    /// Copy export field missing.
    MissingCopyExportField {
        /// Tile id.
        tile_id: String,
        /// Field name.
        field: &'static str,
    },
    /// Screenshot-only export would be allowed.
    ScreenshotOnlyExportAllowed {
        /// Tile id.
        tile_id: String,
    },
    /// Copy/export dropped required truth.
    CopyExportDropsRequiredTruth {
        /// Tile id.
        tile_id: String,
        /// Missing token.
        token: String,
    },
    /// Duplicate tile id.
    DuplicateTileId {
        /// Tile id.
        tile_id: String,
    },
    /// Required trust class not covered.
    MissingTrustClass {
        /// Trust class.
        trust_class: HandoffTrustClass,
    },
    /// Required destination group not covered.
    MissingDestinationGroup {
        /// Destination group.
        destination_group: HandoffDestinationGroup,
    },
}

fn validate_component_copy_export<V, MissingFormat, MissingField, ScreenshotOnly>(
    id: &str,
    copy_export: &ComponentCopyExport,
    required_fields: &[&'static str],
    violations: &mut Vec<V>,
    missing_format: MissingFormat,
    missing_field: MissingField,
    screenshot_only: ScreenshotOnly,
) where
    MissingFormat: Fn(String, &'static str) -> V,
    MissingField: Fn(String, &'static str) -> V,
    ScreenshotOnly: Fn(String) -> V,
{
    for format in ["text", "json", "markdown"] {
        if !copy_export.formats.iter().any(|f| f == format) {
            violations.push(missing_format(id.to_owned(), format));
        }
    }
    for field in required_fields {
        if !copy_export.export_fields.iter().any(|f| f == field) {
            violations.push(missing_field(id.to_owned(), field));
        }
    }
    if !copy_export.screenshot_only_prohibited {
        violations.push(screenshot_only(id.to_owned()));
    }
}
