//! Finalized Support Center and Diagnostics Center surfaces: performance
//! inspector, language-service dashboard, index-health view, and AI evidence
//! inspector.
//!
//! This module mints the shared health-feed object model and the four
//! per-subsystem inspectors that drive Support Center cards, Diagnostics
//! Center summaries, and support/export packets identically across desktop
//! and CLI/headless surfaces.
//!
//! ## What this module owns
//!
//! - The [`HealthFeedItem`] shared record — one row per subsystem naming
//!   service family, boundary class, affected workflows, last-checked time,
//!   freshness state, and diagnostics actions.
//! - The [`DiagnosticsCenterRecord`] — the durable escalation surface that
//!   links Project Doctor findings, support-bundle preview, repair
//!   transactions, exact-build crash evidence, and per-subsystem inspectors
//!   through one typed navigation and export model.
//! - The [`PerformanceInspectorRecord`], [`LanguageServiceDashboardRecord`],
//!   [`IndexHealthViewRecord`], and [`AiEvidenceInspectorRecord`] — the four
//!   bounded inspector surfaces required by the stable lane.
//! - The [`SupportCenterCardRecord`] — one card per health-feed item so
//!   Support Center and Diagnostics Center consume the same object model.
//! - The [`DiagnosticsExportPacket`] — metadata-safe support/export
//!   projection bound to the exact-build identity and redacted by default.
//!
//! ## Posture
//!
//! Project Doctor remains read-only by default, safe mode stays bounded,
//! repairs are previewable, and crash/support exports are exact-build and
//! redacted-by-default. Partial-service outages keep unaffected subsystems
//! explicitly healthy and preserve a visible local-only continuity note.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for the diagnostics center record.
pub const DIAGNOSTICS_CENTER_RECORD_KIND: &str = "diagnostics_center_record";

/// Stable record-kind tag for one health-feed item.
pub const HEALTH_FEED_ITEM_RECORD_KIND: &str = "health_feed_item_record";

/// Stable record-kind tag for the support-center card record.
pub const SUPPORT_CENTER_CARD_RECORD_KIND: &str = "support_center_card_record";

/// Stable record-kind tag for the diagnostics export packet.
pub const DIAGNOSTICS_EXPORT_PACKET_RECORD_KIND: &str = "diagnostics_export_packet_record";

/// Integer schema version for diagnostics center records.
pub const DIAGNOSTICS_CENTER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DIAGNOSTICS_CENTER_SCHEMA_REF: &str =
    "schemas/support/finalize_support_center_surfaces_performance_inspector_language_service.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const DIAGNOSTICS_CENTER_DOC_REF: &str =
    "docs/support/m4/finalize_support_center_surfaces_performance_inspector_language_service.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const DIAGNOSTICS_CENTER_ARTIFACT_REF: &str =
    "artifacts/support/m4/finalize_support_center_surfaces_performance_inspector_language_service.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const DIAGNOSTICS_CENTER_FIXTURE_DIR: &str =
    "fixtures/support/m4/finalize_support_center_surfaces_performance_inspector_language_service";

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// Closed service-family vocabulary. Every health-feed item names exactly one
/// service family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceFamilyClass {
    /// Performance and runtime-efficiency subsystem.
    Performance,
    /// Language service and LSP router subsystem.
    LanguageService,
    /// Search index and retrieval subsystem.
    Index,
    /// AI evidence and composer subsystem.
    AiEvidence,
    /// Project Doctor diagnosis subsystem.
    ProjectDoctor,
    /// Support bundle assembly and preview subsystem.
    SupportBundle,
    /// Repair transaction and preview subsystem.
    RepairTransaction,
    /// Crash evidence and incident trail subsystem.
    CrashEvidence,
}

impl ServiceFamilyClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Performance => "performance",
            Self::LanguageService => "language_service",
            Self::Index => "index",
            Self::AiEvidence => "ai_evidence",
            Self::ProjectDoctor => "project_doctor",
            Self::SupportBundle => "support_bundle",
            Self::RepairTransaction => "repair_transaction",
            Self::CrashEvidence => "crash_evidence",
        }
    }

    /// Every required service family, in declaration order.
    pub const REQUIRED: [Self; 8] = [
        Self::Performance,
        Self::LanguageService,
        Self::Index,
        Self::AiEvidence,
        Self::ProjectDoctor,
        Self::SupportBundle,
        Self::RepairTransaction,
        Self::CrashEvidence,
    ];

    /// The four inspector-bound families.
    pub const INSPECTOR_FAMILIES: [Self; 4] = [
        Self::Performance,
        Self::LanguageService,
        Self::Index,
        Self::AiEvidence,
    ];
}

/// Closed boundary-class vocabulary. Names where the subsystem runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryClass {
    /// Local device only.
    LocalOnly,
    /// Remote-managed or fleet context.
    RemoteManaged,
    /// Offline or air-gapped context.
    OfflineLocal,
    /// CLI/headless context.
    CliHeadless,
}

impl BoundaryClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::RemoteManaged => "remote_managed",
            Self::OfflineLocal => "offline_local",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// Closed freshness-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessStateClass {
    /// Evidence is current and verified.
    Fresh,
    /// Evidence is older than the target refresh interval.
    Stale,
    /// Evidence state is not yet known.
    Unknown,
    /// Some evidence is current and some is missing or stale.
    Partial,
}

impl FreshnessStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::Unknown => "unknown",
            Self::Partial => "partial",
        }
    }
}

/// Closed diagnostics-action vocabulary. Names what a user can do from a
/// health-feed item or card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticsActionClass {
    /// Open the per-subsystem inspector.
    OpenInspector,
    /// Export a redacted support bundle.
    ExportSupportBundle,
    /// Open a repair preview.
    OpenRepairPreview,
    /// Escalate to a handoff draft or operator packet.
    Escalate,
    /// Open crash evidence or incident trail.
    OpenCrashEvidence,
    /// Open Project Doctor findings.
    OpenDoctorFindings,
    /// Review paused or shed work.
    ReviewPausedWork,
}

impl DiagnosticsActionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenInspector => "open_inspector",
            Self::ExportSupportBundle => "export_support_bundle",
            Self::OpenRepairPreview => "open_repair_preview",
            Self::Escalate => "escalate",
            Self::OpenCrashEvidence => "open_crash_evidence",
            Self::OpenDoctorFindings => "open_doctor_findings",
            Self::ReviewPausedWork => "review_paused_work",
        }
    }
}

/// Closed inspector-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorClass {
    /// Performance inspector surface.
    PerformanceInspector,
    /// Language-service dashboard surface.
    LanguageServiceDashboard,
    /// Index-health view surface.
    IndexHealthView,
    /// AI evidence inspector surface.
    AiEvidenceInspector,
}

impl InspectorClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerformanceInspector => "performance_inspector",
            Self::LanguageServiceDashboard => "language_service_dashboard",
            Self::IndexHealthView => "index_health_view",
            Self::AiEvidenceInspector => "ai_evidence_inspector",
        }
    }

    /// Every required inspector class.
    pub const REQUIRED: [Self; 4] = [
        Self::PerformanceInspector,
        Self::LanguageServiceDashboard,
        Self::IndexHealthView,
        Self::AiEvidenceInspector,
    ];

    /// Returns the matching service family for this inspector.
    pub const fn service_family(self) -> ServiceFamilyClass {
        match self {
            Self::PerformanceInspector => ServiceFamilyClass::Performance,
            Self::LanguageServiceDashboard => ServiceFamilyClass::LanguageService,
            Self::IndexHealthView => ServiceFamilyClass::Index,
            Self::AiEvidenceInspector => ServiceFamilyClass::AiEvidence,
        }
    }
}

/// Closed health-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthStateClass {
    /// Subsystem is healthy and within published budgets.
    Healthy,
    /// Subsystem is degraded but still operational.
    Degraded,
    /// Subsystem is unavailable.
    Unavailable,
    /// Subsystem is quarantined after repeated failures.
    Quarantined,
}

impl HealthStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
            Self::Quarantined => "quarantined",
        }
    }
}

/// Closed outage-scope vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutageScopeClass {
    /// No outage; all subsystems healthy.
    None,
    /// One subsystem is affected.
    SingleSubsystem,
    /// Multiple subsystems are affected but some remain healthy.
    PartialService,
    /// All subsystems are unavailable.
    FullUnavailable,
}

impl OutageScopeClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SingleSubsystem => "single_subsystem",
            Self::PartialService => "partial_service",
            Self::FullUnavailable => "full_unavailable",
        }
    }
}

/// Closed recovery-ladder hook class for diagnostics center.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticsRecoveryHookClass {
    /// Enter safe mode.
    SafeMode,
    /// Start extension bisect.
    ExtensionBisect,
    /// Open without restore.
    OpenWithoutRestore,
    /// Reset disposable cache or index.
    CacheIndexRepair,
    /// Export support bundle.
    ExportSupportBundle,
    /// Open repair preview.
    OpenRepairPreview,
}

impl DiagnosticsRecoveryHookClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeMode => "safe_mode",
            Self::ExtensionBisect => "extension_bisect",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::CacheIndexRepair => "cache_index_repair",
            Self::ExportSupportBundle => "export_support_bundle",
            Self::OpenRepairPreview => "open_repair_preview",
        }
    }
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// One shared health-feed item that drives support-center cards, diagnostics
/// summaries, and export packets identically across surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthFeedItem {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable item id.
    pub item_id: String,
    /// Service family this item belongs to.
    pub service_family: ServiceFamilyClass,
    /// Boundary class where the service runs.
    pub boundary_class: BoundaryClass,
    /// Human-readable affected-workflows summary.
    pub affected_workflows_summary: String,
    /// Last-checked UTC timestamp.
    pub last_checked_at: String,
    /// Freshness state of the evidence.
    pub freshness_state: FreshnessStateClass,
    /// Available diagnostics actions for this item.
    pub diagnostics_actions: Vec<DiagnosticsActionClass>,
    /// Health state of the subsystem.
    pub health_state: HealthStateClass,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Whether a local-only continuity note is required.
    pub local_only_continuity_note_required: bool,
    /// Local-only continuity note text when required.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub local_only_continuity_note: Option<String>,
}

/// Performance inspector record with p50/p95 budgets and benchmark traces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceInspectorRecord {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable inspector id.
    pub inspector_id: String,
    /// Health state.
    pub health_state: HealthStateClass,
    /// Published p50 latency budget in milliseconds.
    pub p50_budget_ms: u32,
    /// Published p95 latency budget in milliseconds.
    pub p95_budget_ms: u32,
    /// Observed p50 latency in milliseconds.
    pub observed_p50_ms: u32,
    /// Observed p95 latency in milliseconds.
    pub observed_p95_ms: u32,
    /// Whether observed latencies are within published budgets.
    pub within_budget: bool,
    /// Benchmark-lab trace refs.
    pub benchmark_trace_refs: Vec<String>,
    /// Corpus metadata ref.
    pub corpus_metadata_ref: String,
    /// Waiver hooks where thresholds are intentionally tightened.
    pub waiver_hooks: Vec<String>,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Language-service dashboard record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageServiceDashboardRecord {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable inspector id.
    pub inspector_id: String,
    /// Health state.
    pub health_state: HealthStateClass,
    /// Router decision refs.
    pub router_decision_refs: Vec<String>,
    /// Provider availability rows.
    pub provider_availability_rows: Vec<LanguageServiceProviderRow>,
    /// Whether any provider is quarantined.
    pub quarantined_provider_present: bool,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Capture timestamp.
    pub captured_at: String,
}

/// One language-service provider availability row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageServiceProviderRow {
    /// Provider id.
    pub provider_id: String,
    /// Provider display label.
    pub provider_display_label: String,
    /// Health state.
    pub health_state: HealthStateClass,
    /// Freshness state.
    pub freshness_state: FreshnessStateClass,
    /// Scope claim class.
    pub scope_claim_class: String,
    /// Restart strike count.
    pub restart_strike_count: u32,
    /// Quarantine ref when quarantined.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub quarantine_ref: Option<String>,
    /// Reviewer-facing summary.
    pub summary: String,
}

/// Index-health view record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexHealthViewRecord {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable inspector id.
    pub inspector_id: String,
    /// Health state.
    pub health_state: HealthStateClass,
    /// Index freshness state.
    pub index_freshness: FreshnessStateClass,
    /// Coverage percentage (0-100).
    pub coverage_percent: u32,
    /// Whether corruption checks passed.
    pub corruption_checks_passed: bool,
    /// Last full index timestamp.
    pub last_full_index_at: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Capture timestamp.
    pub captured_at: String,
}

/// AI evidence inspector record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiEvidenceInspectorRecord {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable inspector id.
    pub inspector_id: String,
    /// Health state.
    pub health_state: HealthStateClass,
    /// AI evidence packet refs.
    pub evidence_packet_refs: Vec<String>,
    /// Redaction class token.
    pub redaction_class_token: String,
    /// Replay posture token.
    pub replay_posture_token: String,
    /// Whether raw prompt bodies are excluded.
    pub raw_prompts_excluded: bool,
    /// Whether raw provider payloads are excluded.
    pub raw_provider_payloads_excluded: bool,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Support Center card record driven by one health-feed item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCenterCardRecord {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable card id.
    pub card_id: String,
    /// Health-feed item id this card projects from.
    pub health_feed_item_id: String,
    /// Card title.
    pub title: String,
    /// Card summary.
    pub summary: String,
    /// Health state.
    pub health_state: HealthStateClass,
    /// Available actions.
    pub actions: Vec<DiagnosticsActionClass>,
    /// Whether the card is reachable in headless mode.
    pub headless_reachable: bool,
    /// Whether the card surface is read-only.
    pub read_only: bool,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Diagnostics Center top-level record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsCenterRecord {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable record id.
    pub record_id: String,
    /// Health-feed items for all subsystems.
    pub health_feed_items: Vec<HealthFeedItem>,
    /// Performance inspector.
    pub performance_inspector: PerformanceInspectorRecord,
    /// Language-service dashboard.
    pub language_service_dashboard: LanguageServiceDashboardRecord,
    /// Index-health view.
    pub index_health_view: IndexHealthViewRecord,
    /// AI evidence inspector.
    pub ai_evidence_inspector: AiEvidenceInspectorRecord,
    /// Project Doctor finding refs.
    pub doctor_finding_refs: Vec<String>,
    /// Support-bundle preview ref.
    pub support_bundle_preview_ref: String,
    /// Repair transaction refs.
    pub repair_transaction_refs: Vec<String>,
    /// Crash evidence ref.
    pub crash_evidence_ref: String,
    /// Exact-build identity ref.
    pub exact_build_identity_ref: String,
    /// Outage scope.
    pub outage_scope: OutageScopeClass,
    /// Recovery-ladder hooks available.
    pub recovery_hooks: Vec<DiagnosticsRecoveryHookClass>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Capture timestamp.
    pub captured_at: String,
}

/// One metadata-safe support/export row for a diagnostics center record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsExportRow {
    /// Record id.
    pub record_id: String,
    /// Outage scope.
    pub outage_scope: OutageScopeClass,
    /// Health-feed item ids.
    pub health_feed_item_ids: Vec<String>,
    /// Inspector ids.
    pub inspector_ids: Vec<String>,
    /// Doctor finding refs.
    pub doctor_finding_refs: Vec<String>,
    /// Exact-build identity ref.
    pub exact_build_identity_ref: String,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

/// Metadata-safe diagnostics export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsExportPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Export rows.
    pub rows: Vec<DiagnosticsExportRow>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Whether every row in the packet is export-safe.
    pub all_rows_export_safe: bool,
}

impl DiagnosticsExportPacket {
    /// Returns true when the packet is safe for metadata-only export.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.all_rows_export_safe
    }
}

/// One validation violation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsCenterViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Reviewer-facing failure message.
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsCenterValidationReport {
    /// Validation failures.
    pub violations: Vec<DiagnosticsCenterViolation>,
}

impl DiagnosticsCenterValidationReport {
    /// True when no violations were found.
    pub fn is_valid(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for DiagnosticsCenterValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} diagnostics-center violation(s)",
            self.violations.len()
        )
    }
}

impl Error for DiagnosticsCenterValidationReport {}

// ---------------------------------------------------------------------------
// Evaluator
// ---------------------------------------------------------------------------

/// Diagnostics Center evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct DiagnosticsCenterEvaluator;

impl DiagnosticsCenterEvaluator {
    /// Creates a new evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates a diagnostics center record.
    pub fn validate_record(
        &self,
        record: &DiagnosticsCenterRecord,
    ) -> DiagnosticsCenterValidationReport {
        let mut violations = Vec::new();

        if record.schema_version != DIAGNOSTICS_CENTER_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "diagnostics_center.schema_version",
                &record.record_id,
                "schema_version must be 1",
            );
        }
        if record.record_kind != DIAGNOSTICS_CENTER_RECORD_KIND {
            push_violation(
                &mut violations,
                "diagnostics_center.record_kind",
                &record.record_id,
                format!("record_kind must be {DIAGNOSTICS_CENTER_RECORD_KIND}"),
            );
        }
        if record.exact_build_identity_ref.trim().is_empty() {
            push_violation(
                &mut violations,
                "diagnostics_center.empty_build_identity",
                &record.record_id,
                "exact_build_identity_ref must be non-empty",
            );
        }
        if !record.raw_private_material_excluded {
            push_violation(
                &mut violations,
                "diagnostics_center.raw_private_material_not_excluded",
                &record.record_id,
                "raw_private_material_excluded must be true",
            );
        }

        // Health-feed items must cover all required service families.
        let families: BTreeSet<ServiceFamilyClass> = record
            .health_feed_items
            .iter()
            .map(|i| i.service_family)
            .collect();
        for required in ServiceFamilyClass::REQUIRED {
            if !families.contains(&required) {
                push_violation(
                    &mut violations,
                    "diagnostics_center.missing_service_family",
                    &record.record_id,
                    format!(
                        "health feed must cover service family {}",
                        required.as_str()
                    ),
                );
            }
        }

        // Every health-feed item must have at least one diagnostics action.
        for item in &record.health_feed_items {
            if item.diagnostics_actions.is_empty() {
                push_violation(
                    &mut violations,
                    "diagnostics_center.empty_diagnostics_actions",
                    &item.item_id,
                    "health feed item must declare at least one diagnostics action",
                );
            }
            if item.local_only_continuity_note_required && item.local_only_continuity_note.is_none()
            {
                push_violation(
                    &mut violations,
                    "diagnostics_center.missing_continuity_note",
                    &item.item_id,
                    "local-only continuity note is required but missing",
                );
            }
        }

        // Partial-service outage: unaffected subsystems must be explicitly healthy.
        if record.outage_scope == OutageScopeClass::PartialService {
            let healthy_families: BTreeSet<ServiceFamilyClass> = record
                .health_feed_items
                .iter()
                .filter(|i| i.health_state == HealthStateClass::Healthy)
                .map(|i| i.service_family)
                .collect();
            if healthy_families.is_empty() {
                push_violation(
                    &mut violations,
                    "diagnostics_center.partial_outage_no_healthy",
                    &record.record_id,
                    "partial-service outage must keep at least one subsystem explicitly healthy",
                );
            }
        }

        // Inspector records must match required inspectors.
        let inspector_ids = [
            record.performance_inspector.inspector_id.clone(),
            record.language_service_dashboard.inspector_id.clone(),
            record.index_health_view.inspector_id.clone(),
            record.ai_evidence_inspector.inspector_id.clone(),
        ];
        if inspector_ids.iter().any(|id| id.trim().is_empty()) {
            push_violation(
                &mut violations,
                "diagnostics_center.empty_inspector_id",
                &record.record_id,
                "every inspector must have a non-empty inspector_id",
            );
        }

        // AI evidence inspector must exclude raw material.
        if !record.ai_evidence_inspector.raw_prompts_excluded {
            push_violation(
                &mut violations,
                "diagnostics_center.ai_raw_prompts_not_excluded",
                &record.ai_evidence_inspector.inspector_id,
                "AI evidence inspector must exclude raw prompt bodies",
            );
        }
        if !record.ai_evidence_inspector.raw_provider_payloads_excluded {
            push_violation(
                &mut violations,
                "diagnostics_center.ai_raw_provider_not_excluded",
                &record.ai_evidence_inspector.inspector_id,
                "AI evidence inspector must exclude raw provider payloads",
            );
        }

        // Recovery hooks must not be empty.
        if record.recovery_hooks.is_empty() {
            push_violation(
                &mut violations,
                "diagnostics_center.empty_recovery_hooks",
                &record.record_id,
                "at least one recovery-ladder hook is required",
            );
        }

        DiagnosticsCenterValidationReport { violations }
    }

    /// Builds a metadata-safe support packet from validated diagnostics center records.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        captured_at: impl Into<String>,
        records: &[DiagnosticsCenterRecord],
    ) -> Result<DiagnosticsExportPacket, DiagnosticsCenterValidationReport> {
        let mut all_violations = Vec::new();
        let mut rows = Vec::new();

        for record in records {
            let report = self.validate_record(record);
            all_violations.extend(report.violations);

            rows.push(DiagnosticsExportRow {
                record_id: record.record_id.clone(),
                outage_scope: record.outage_scope,
                health_feed_item_ids: record
                    .health_feed_items
                    .iter()
                    .map(|i| i.item_id.clone())
                    .collect(),
                inspector_ids: vec![
                    record.performance_inspector.inspector_id.clone(),
                    record.language_service_dashboard.inspector_id.clone(),
                    record.index_health_view.inspector_id.clone(),
                    record.ai_evidence_inspector.inspector_id.clone(),
                ],
                doctor_finding_refs: record.doctor_finding_refs.clone(),
                exact_build_identity_ref: record.exact_build_identity_ref.clone(),
                raw_private_material_excluded: record.raw_private_material_excluded,
            });
        }

        if !all_violations.is_empty() {
            return Err(DiagnosticsCenterValidationReport {
                violations: all_violations,
            });
        }

        Ok(DiagnosticsExportPacket {
            record_kind: DIAGNOSTICS_EXPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: DIAGNOSTICS_CENTER_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            captured_at: captured_at.into(),
            doc_ref: DIAGNOSTICS_CENTER_DOC_REF.to_owned(),
            schema_ref: DIAGNOSTICS_CENTER_SCHEMA_REF.to_owned(),
            rows,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            all_rows_export_safe: true,
        })
    }

    /// Derives support-center card records from the health-feed items of a
    /// diagnostics center record.
    pub fn support_center_cards(
        &self,
        record: &DiagnosticsCenterRecord,
        captured_at: impl Into<String>,
    ) -> Vec<SupportCenterCardRecord> {
        let captured_at = captured_at.into();
        record
            .health_feed_items
            .iter()
            .map(|item| SupportCenterCardRecord {
                schema_version: DIAGNOSTICS_CENTER_SCHEMA_VERSION,
                record_kind: SUPPORT_CENTER_CARD_RECORD_KIND.to_owned(),
                card_id: format!("support_center_card:{}", item.item_id),
                health_feed_item_id: item.item_id.clone(),
                title: format!("{} — {}", item.service_family.as_str(), item.summary),
                summary: item.summary.clone(),
                health_state: item.health_state,
                actions: item.diagnostics_actions.clone(),
                headless_reachable: item.boundary_class == BoundaryClass::CliHeadless
                    || item.boundary_class == BoundaryClass::LocalOnly,
                read_only: true,
                captured_at: captured_at.clone(),
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Load helpers
// ---------------------------------------------------------------------------

/// Deserialize a diagnostics center record from YAML.
pub fn load_diagnostics_center_record(
    yaml: &str,
) -> Result<DiagnosticsCenterRecord, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Deserialize a diagnostics export packet from YAML.
pub fn load_diagnostics_export_packet(
    yaml: &str,
) -> Result<DiagnosticsExportPacket, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

// ---------------------------------------------------------------------------
// Utility
// ---------------------------------------------------------------------------

fn push_violation(
    violations: &mut Vec<DiagnosticsCenterViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(DiagnosticsCenterViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}
