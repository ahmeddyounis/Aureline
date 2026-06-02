//! Stable Project Doctor probe-pack catalog, finding contract, explainability,
//! and unsupported-state reporting.
//!
//! The stable lane promotes Project Doctor from the beta probe-pack catalog to a
//! versioned, attributable, confidence-labeled, and explainable diagnosis system
//! with formal unsupported-state reporting. It owns four typed records mirroring
//! the boundary schema at
//! `/schemas/support/stabilize_project_doctor_probes_finding_codes_explainability_and.schema.json`:
//!
//! - [`ProjectDoctorStableProbePackCatalog`] names the closed list of supported
//!   probe packs with a stable lifecycle status.
//! - [`ProjectDoctorStableProbePack`] wraps one stable pack record.
//! - [`ProjectDoctorStableFinding`] is one typed finding emitted by a stable pack
//!   with explainability factors, stable finding code, and unsupported-state
//!   classification.
//! - [`ProjectDoctorStableUnsupportedStateReport`] is a formal unsupported-state
//!   report that Doctor emits when a probe refuses to diagnose beyond supported
//!   evidence.
//!
//! The [`ProjectDoctorStableEvaluator`] validates the catalog, findings, and
//! unsupported-state reports, and folds one catalog plus findings and reports
//! into a metadata-safe [`ProjectDoctorStableSupportPacket`] that includes
//! chain-of-custody events for support export.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Frozen schema version for the stable Project Doctor records.
pub const PROJECT_DOCTOR_STABLE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the stable probe-pack catalog record.
pub const PROJECT_DOCTOR_STABLE_PROBE_PACK_CATALOG_RECORD_KIND: &str =
    "project_doctor_stable_probe_pack_catalog_record";

/// Stable record-kind tag for a single stable probe-pack record.
pub const PROJECT_DOCTOR_STABLE_PROBE_PACK_RECORD_KIND: &str =
    "project_doctor_stable_probe_pack_record";

/// Stable record-kind tag for a stable finding record.
pub const PROJECT_DOCTOR_STABLE_FINDING_RECORD_KIND: &str = "project_doctor_stable_finding_record";

/// Stable record-kind tag for the stable unsupported-state report record.
pub const PROJECT_DOCTOR_STABLE_UNSUPPORTED_STATE_REPORT_RECORD_KIND: &str =
    "project_doctor_stable_unsupported_state_report_record";

/// Stable record-kind tag for the stable support-export packet.
pub const PROJECT_DOCTOR_STABLE_SUPPORT_PACKET_RECORD_KIND: &str =
    "project_doctor_stable_support_packet_record";

/// Repo-relative path of the boundary schema mirrored by this module.
pub const PROJECT_DOCTOR_STABLE_SCHEMA_REF: &str =
    "schemas/support/stabilize_project_doctor_probes_finding_codes_explainability_and.schema.json";

/// Reviewer doc ref quoted by every emitted packet.
pub const PROJECT_DOCTOR_STABLE_DOC_REF: &str =
    "docs/support/m4/stabilize_project_doctor_probes_finding_codes_explainability_and.md";

/// Stable finding-code prefix every stable finding must use.
pub const DOCTOR_FINDING_PREFIX: &str = "doctor.finding.";

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// Closed probe-pack class vocabulary. Matches the beta vocabulary so stable
/// packs can be traced back to beta packs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableProbePackClass {
    /// Entry / open / clone-import / recent-workspace readiness pack.
    EntryOpenReadiness,
    /// Execution-context and toolchain resolution pack.
    ToolchainResolution,
    /// Search and index readiness pack.
    SearchIndexReadiness,
    /// Trust, identity, and policy pack.
    TrustPolicy,
    /// Local Git baseline pack.
    GitBaseline,
    /// Provider / credential authority pack.
    ProviderAuth,
    /// Session restore / crash replay continuity pack.
    RestoreContinuity,
    /// Crash and symbolication pack.
    CrashSymbolication,
    /// Support-bundle integrity pack.
    SupportBundleIntegrity,
}

impl StableProbePackClass {
    /// Returns every probe-pack class in catalog order.
    pub const fn all() -> [Self; 9] {
        [
            Self::EntryOpenReadiness,
            Self::ToolchainResolution,
            Self::SearchIndexReadiness,
            Self::TrustPolicy,
            Self::GitBaseline,
            Self::ProviderAuth,
            Self::RestoreContinuity,
            Self::CrashSymbolication,
            Self::SupportBundleIntegrity,
        ]
    }

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EntryOpenReadiness => "entry_open_readiness",
            Self::ToolchainResolution => "toolchain_resolution",
            Self::SearchIndexReadiness => "search_index_readiness",
            Self::TrustPolicy => "trust_policy",
            Self::GitBaseline => "git_baseline",
            Self::ProviderAuth => "provider_auth",
            Self::RestoreContinuity => "restore_continuity",
            Self::CrashSymbolication => "crash_symbolication",
            Self::SupportBundleIntegrity => "support_bundle_integrity",
        }
    }
}

impl fmt::Display for StableProbePackClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Closed probe-pack lifecycle status. The catalog carries stable, beta, and
/// deprecated packs so the stable lane can reference beta predecessors and
/// deprecated successors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableProbePackLifecycleStatus {
    /// Stable-grade pack admitted for all supported contexts.
    Stable,
    /// Beta-grade pack admitted for headless and support-guided diagnosis.
    Beta,
    /// Deprecated pack retained for back-compat reads only.
    Deprecated,
}

impl StableProbePackLifecycleStatus {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Deprecated => "deprecated",
        }
    }
}

/// Closed read-only posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableReadOnlyPostureClass {
    /// The pack performs no mutation at all.
    ReadOnlyByDefaultNoMutation,
    /// The pack may write local evidence/preview rows only.
    MetadataLocalEvidenceOnly,
}

impl StableReadOnlyPostureClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyByDefaultNoMutation => "read_only_by_default_no_mutation",
            Self::MetadataLocalEvidenceOnly => "metadata_local_evidence_only",
        }
    }
}

/// Headless admission class for a stable probe pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableHeadlessAdmissionClass {
    /// Admitted to run headless / non-interactively.
    AdmittedSafeForHeadless,
    /// Admitted only when a support-guided session attests to the run.
    AdmittedSafeForSupportGuidedOnly,
    /// Headless run is denied.
    Denied,
}

impl StableHeadlessAdmissionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedSafeForHeadless => "admitted_safe_for_headless",
            Self::AdmittedSafeForSupportGuidedOnly => "admitted_safe_for_support_guided_only",
            Self::Denied => "denied",
        }
    }
}

/// Support-guided admission class for a stable probe pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableSupportGuidedAdmissionClass {
    /// Admitted under support-guided mode.
    AdmittedInSupportGuidedMode,
    /// Support-guided run is denied.
    Denied,
}

impl StableSupportGuidedAdmissionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedInSupportGuidedMode => "admitted_in_support_guided_mode",
            Self::Denied => "denied",
        }
    }
}

/// Closed finding severity class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableFindingSeverityClass {
    /// Informational finding.
    Info,
    /// Degraded but still usable.
    Degraded,
    /// Blocks the requested workflow until handled.
    Blocking,
    /// Unsupported in the current context.
    Unsupported,
}

impl StableFindingSeverityClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Degraded => "degraded",
            Self::Blocking => "blocking",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Closed finding confidence class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableFindingConfidenceClass {
    /// Evidence directly proves the finding.
    ObservedAuthoritative,
    /// Evidence proves the finding but leaves a typed gap.
    ObservedWithGap,
    /// Evidence is sufficient for a bounded inference.
    InferredFromEvidence,
    /// More evidence is required before Doctor can prove the state.
    UnknownRequiresProbe,
}

impl StableFindingConfidenceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObservedAuthoritative => "observed_authoritative",
            Self::ObservedWithGap => "observed_with_gap",
            Self::InferredFromEvidence => "inferred_from_evidence",
            Self::UnknownRequiresProbe => "unknown_requires_probe",
        }
    }
}

/// Closed diagnosis posture class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableDiagnosisPostureClass {
    /// The finding is proven from current evidence.
    ProvingDiagnosis,
    /// The finding is inferred and keeps remaining unknowns visible.
    InferringFromPartialEvidence,
    /// Doctor refuses to diagnose beyond supported evidence.
    RefusingUnsupported,
}

impl StableDiagnosisPostureClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvingDiagnosis => "proving_diagnosis",
            Self::InferringFromPartialEvidence => "inferring_from_partial_evidence",
            Self::RefusingUnsupported => "refusing_unsupported",
        }
    }
}

/// Closed recovery handoff class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableRecoveryHandoffClass {
    /// A reviewed repair path is available.
    ReviewedRepairAvailable,
    /// Doctor shows a repair preview but does not apply it.
    PreviewOnly,
    /// Doctor hands off to support or a governed external path.
    HandoffOnly,
    /// No supported local recovery exists.
    Unsupported,
}

impl StableRecoveryHandoffClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewedRepairAvailable => "reviewed_repair_available",
            Self::PreviewOnly => "preview_only",
            Self::HandoffOnly => "handoff_only",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Closed support-context class for stable findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableSupportContextClass {
    /// Desktop interactive shell.
    Desktop,
    /// CLI / headless run.
    CliHeadless,
    /// Remote managed run.
    RemoteManaged,
    /// Offline local run.
    OfflineLocal,
    /// Support-guided session (live or asynchronous).
    SupportGuided,
}

impl StableSupportContextClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::CliHeadless => "cli_headless",
            Self::RemoteManaged => "remote_managed",
            Self::OfflineLocal => "offline_local",
            Self::SupportGuided => "support_guided",
        }
    }
}

/// Closed render-surface class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableRenderSurfaceClass {
    /// Shell finding card / UI surface.
    UiFindingCard,
    /// Interactive CLI finding row.
    CliFindingRow,
    /// Support export row.
    SupportExportRow,
    /// Headless JSON row.
    HeadlessJsonRow,
}

impl StableRenderSurfaceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UiFindingCard => "ui_finding_card",
            Self::CliFindingRow => "cli_finding_row",
            Self::SupportExportRow => "support_export_row",
            Self::HeadlessJsonRow => "headless_json_row",
        }
    }
}

/// Closed attribution-kind class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableAttributionKindClass {
    /// Attribution back to the owning probe pack.
    ProbePackRef,
    /// Attribution back to a stable Doctor rule id.
    DoctorRuleRef,
    /// Attribution to an evidence row.
    EvidenceRef,
    /// Attribution to a support-bundle ref.
    SupportBundleRef,
    /// Attribution to a runbook ref.
    RunbookRef,
    /// Attribution to an escalation packet ref.
    EscalationPacketRef,
}

impl StableAttributionKindClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProbePackRef => "probe_pack_ref",
            Self::DoctorRuleRef => "doctor_rule_ref",
            Self::EvidenceRef => "evidence_ref",
            Self::SupportBundleRef => "support_bundle_ref",
            Self::RunbookRef => "runbook_ref",
            Self::EscalationPacketRef => "escalation_packet_ref",
        }
    }
}

/// Closed support-redaction class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableSupportRedactionClass {
    /// Metadata-safe by default; raw private material excluded.
    MetadataSafeDefault,
    /// Operator-only restricted view.
    OperatorOnlyRestricted,
}

impl StableSupportRedactionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
        }
    }
}

/// Closed explainability factor class. Each factor explains one dimension of
/// why a finding fired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainabilityFactorClass {
    /// Expected state that was not observed.
    ExpectedState,
    /// Observed state that differed from expectation.
    ObservedState,
    /// Basis for the diagnostic belief.
    BeliefBasis,
    /// Evidence that contradicts the finding.
    CounterEvidence,
    /// Known limitation that bounds confidence.
    Limitation,
    /// Impact on the user workflow.
    UserImpact,
    /// Safety reason that shaped the diagnosis posture.
    SafetyReason,
    /// Reason for the recommended next action.
    NextActionReason,
}

impl ExplainabilityFactorClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpectedState => "expected_state",
            Self::ObservedState => "observed_state",
            Self::BeliefBasis => "belief_basis",
            Self::CounterEvidence => "counter_evidence",
            Self::Limitation => "limitation",
            Self::UserImpact => "user_impact",
            Self::SafetyReason => "safety_reason",
            Self::NextActionReason => "next_action_reason",
        }
    }
}

/// Closed unsupported-state class. Consolidates the vocabulary used when
/// Doctor refuses to diagnose or when the context is unsupported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnsupportedStateClass {
    /// No unsupported state; diagnosis proceeded normally.
    None,
    /// The active profile is not supported by the probe.
    UnsupportedProfile,
    /// The probe target is not supported.
    UnsupportedProbeTarget,
    /// The current offline state prevents diagnosis.
    UnsupportedOfflineState,
    /// A managed policy blocks the probe.
    UnsupportedManagedPolicyState,
    /// The probe lacks required permissions.
    InsufficientPermissions,
    /// A high-risk capture is required but not admitted.
    UnsupportedHighRiskCapture,
    /// Required evidence is unavailable.
    EvidenceUnavailable,
    /// The requested scope is out of bounds.
    ScopeOutOfBounds,
    /// A required dependency record is missing.
    DependencyMissing,
    /// The current support context is not admitted.
    ContextNotAdmitted,
    /// The host platform is not supported.
    UnsupportedPlatform,
}

impl UnsupportedStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::UnsupportedProfile => "unsupported_profile",
            Self::UnsupportedProbeTarget => "unsupported_probe_target",
            Self::UnsupportedOfflineState => "unsupported_offline_state",
            Self::UnsupportedManagedPolicyState => "unsupported_managed_policy_state",
            Self::InsufficientPermissions => "insufficient_permissions",
            Self::UnsupportedHighRiskCapture => "unsupported_high_risk_capture",
            Self::EvidenceUnavailable => "evidence_unavailable",
            Self::ScopeOutOfBounds => "scope_out_of_bounds",
            Self::DependencyMissing => "dependency_missing",
            Self::ContextNotAdmitted => "context_not_admitted",
            Self::UnsupportedPlatform => "unsupported_platform",
        }
    }
}

// ---------------------------------------------------------------------------
// Row / record structs
// ---------------------------------------------------------------------------

/// One typed attribution row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableAttributionRef {
    /// Attribution kind.
    pub attribution_kind: StableAttributionKindClass,
    /// Opaque reference value.
    #[serde(rename = "ref")]
    pub ref_: String,
}

/// One explainability factor attached to a stable finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplainabilityFactor {
    /// Stable factor id.
    pub factor_id: String,
    /// Factor class.
    pub factor_class: ExplainabilityFactorClass,
    /// Stable text key for localized rendering.
    pub text_key: String,
    /// Evidence refs supporting the factor.
    pub evidence_refs: Vec<String>,
    /// Reviewer-safe summary.
    pub summary: String,
}

/// One typed probe-pack record stored inside the stable catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableProbePackEntry {
    /// Stable pack identifier.
    pub pack_id: String,
    /// Pack class.
    pub pack_class: StableProbePackClass,
    /// Pack implementation version.
    pub pack_version: String,
    /// Lifecycle status (stable, beta, or deprecated).
    pub lifecycle_status: StableProbePackLifecycleStatus,
    /// Read-only posture for the pack.
    pub read_only_posture: StableReadOnlyPostureClass,
    /// Headless admission class.
    pub headless_admission: StableHeadlessAdmissionClass,
    /// Support-guided admission class.
    pub support_guided_admission: StableSupportGuidedAdmissionClass,
    /// Stable finding codes the pack may emit.
    pub supported_finding_codes: Vec<String>,
    /// Support contexts in which the pack may run.
    pub supported_support_contexts: Vec<StableSupportContextClass>,
    /// Recovery handoff classes the pack may surface.
    pub supported_recovery_handoffs: Vec<StableRecoveryHandoffClass>,
    /// Default redaction class for emitted findings.
    pub default_redaction_class: StableSupportRedactionClass,
    /// Reviewer-safe summary.
    pub summary: String,
}

impl StableProbePackEntry {
    /// True when the pack is read-only and safe to run headlessly or under
    /// support-guided sessions.
    pub fn is_headless_or_support_guided_safe(&self) -> bool {
        matches!(
            self.read_only_posture,
            StableReadOnlyPostureClass::ReadOnlyByDefaultNoMutation
                | StableReadOnlyPostureClass::MetadataLocalEvidenceOnly
        ) && !matches!(
            self.headless_admission,
            StableHeadlessAdmissionClass::Denied
        ) || !matches!(
            self.support_guided_admission,
            StableSupportGuidedAdmissionClass::Denied
        )
    }
}

/// Stable probe-pack catalog record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorStableProbePackCatalog {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable catalog id.
    pub catalog_id: String,
    /// Stable catalog version.
    pub catalog_version: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Pack entries.
    pub packs: Vec<StableProbePackEntry>,
}

/// Single stable probe-pack record (wraps one entry with the discriminator).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorStableProbePack {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Pack entry.
    pub pack: StableProbePackEntry,
}

/// Stable-grade Project Doctor finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorStableFinding {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable finding id.
    pub finding_id: String,
    /// Stable finding code (must begin with `doctor.finding.`).
    pub finding_code: String,
    /// Pack id that emitted the finding.
    pub probe_pack_ref: String,
    /// Pack class that emitted the finding.
    pub probe_pack_class: StableProbePackClass,
    /// Pack version that emitted the finding.
    pub probe_pack_version: String,
    /// Machine severity class.
    pub severity_class: StableFindingSeverityClass,
    /// Confidence class.
    pub confidence_class: StableFindingConfidenceClass,
    /// Diagnosis posture.
    pub diagnosis_posture: StableDiagnosisPostureClass,
    /// Recovery handoff class.
    pub recovery_handoff_class: StableRecoveryHandoffClass,
    /// Support context in which the finding was emitted.
    pub support_context_class: StableSupportContextClass,
    /// Render surfaces that may carry the finding.
    pub render_surfaces: Vec<StableRenderSurfaceClass>,
    /// Attribution refs (must include the owning pack).
    pub attribution_refs: Vec<StableAttributionRef>,
    /// Evidence refs.
    pub evidence_refs: Vec<String>,
    /// Explainability factors.
    pub explainability_factors: Vec<ExplainabilityFactor>,
    /// Reviewer-safe summary.
    pub summary: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Redaction class.
    pub redaction_class: StableSupportRedactionClass,
    /// Whether raw private material is excluded (must be true).
    pub raw_private_material_excluded: bool,
}

/// Formal unsupported-state report emitted when Doctor refuses to diagnose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorStableUnsupportedStateReport {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable report id.
    pub report_id: String,
    /// Pack id that attempted diagnosis.
    pub probe_pack_ref: String,
    /// Pack class.
    pub probe_pack_class: StableProbePackClass,
    /// Unsupported state class.
    pub unsupported_state_class: UnsupportedStateClass,
    /// Stable finding code for the unsupported state.
    pub unsupported_finding_code: String,
    /// Evidence refs available at the time of refusal.
    pub evidence_refs: Vec<String>,
    /// Explainability factors describing why the state is unsupported.
    pub explainability_factors: Vec<ExplainabilityFactor>,
    /// Reviewer-safe summary.
    pub summary: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Redaction class.
    pub redaction_class: StableSupportRedactionClass,
    /// Whether raw private material is excluded (must be true).
    pub raw_private_material_excluded: bool,
}

/// One chain-of-custody event for the stable support packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableChainOfCustodyEvent {
    /// Stable event id.
    pub event_id: String,
    /// Sequence number (0-based, strictly increasing).
    pub sequence: u32,
    /// Actor class.
    pub actor_class: String,
    /// Opaque actor ref.
    pub actor_ref: String,
    /// Action class.
    pub action_class: String,
    /// RFC 3339 UTC timestamp.
    pub occurred_at: String,
    /// Redaction-safe note.
    pub note: String,
}

/// Metadata-safe support packet projecting one catalog, findings, and
/// unsupported-state reports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorStableSupportPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Catalog id that backed the findings.
    pub catalog_id: String,
    /// Catalog version that backed the findings.
    pub catalog_version: String,
    /// Pack rows projected by the packet.
    pub pack_rows: Vec<ProjectDoctorStablePackRow>,
    /// Finding rows projected by the packet.
    pub finding_rows: Vec<ProjectDoctorStableFindingRow>,
    /// Unsupported-state report rows projected by the packet.
    pub unsupported_state_rows: Vec<ProjectDoctorStableUnsupportedStateRow>,
    /// Chain-of-custody events for export traceability.
    pub chain_of_custody: Vec<StableChainOfCustodyEvent>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
}

impl ProjectDoctorStableSupportPacket {
    /// Returns true when the packet preserves the metadata-safe contract.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.pack_rows.is_empty()
            && self.finding_rows.iter().all(|row| {
                row.raw_private_material_excluded
                    && row.finding_code.starts_with(DOCTOR_FINDING_PREFIX)
            })
            && self.unsupported_state_rows.iter().all(|row| {
                row.raw_private_material_excluded
                    && (row.unsupported_finding_code.is_empty()
                        || row
                            .unsupported_finding_code
                            .starts_with(DOCTOR_FINDING_PREFIX))
            })
            && self
                .chain_of_custody
                .windows(2)
                .all(|w| w[0].sequence < w[1].sequence)
    }
}

/// Pack row in the stable support packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorStablePackRow {
    /// Pack id.
    pub pack_id: String,
    /// Pack class.
    pub pack_class: StableProbePackClass,
    /// Pack version.
    pub pack_version: String,
    /// Lifecycle status.
    pub lifecycle_status: StableProbePackLifecycleStatus,
    /// Read-only posture.
    pub read_only_posture: StableReadOnlyPostureClass,
    /// Headless admission.
    pub headless_admission: StableHeadlessAdmissionClass,
    /// Support-guided admission.
    pub support_guided_admission: StableSupportGuidedAdmissionClass,
    /// Default redaction class.
    pub default_redaction_class: StableSupportRedactionClass,
    /// Reviewer-safe summary.
    pub summary: String,
}

impl From<&StableProbePackEntry> for ProjectDoctorStablePackRow {
    fn from(pack: &StableProbePackEntry) -> Self {
        Self {
            pack_id: pack.pack_id.clone(),
            pack_class: pack.pack_class,
            pack_version: pack.pack_version.clone(),
            lifecycle_status: pack.lifecycle_status,
            read_only_posture: pack.read_only_posture,
            headless_admission: pack.headless_admission,
            support_guided_admission: pack.support_guided_admission,
            default_redaction_class: pack.default_redaction_class,
            summary: pack.summary.clone(),
        }
    }
}

/// Finding row in the stable support packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorStableFindingRow {
    /// Stable finding id.
    pub finding_id: String,
    /// Stable finding code.
    pub finding_code: String,
    /// Pack ref.
    pub probe_pack_ref: String,
    /// Pack class.
    pub probe_pack_class: StableProbePackClass,
    /// Severity class.
    pub severity_class: StableFindingSeverityClass,
    /// Confidence class.
    pub confidence_class: StableFindingConfidenceClass,
    /// Recovery handoff class.
    pub recovery_handoff_class: StableRecoveryHandoffClass,
    /// Support context class.
    pub support_context_class: StableSupportContextClass,
    /// Render surfaces.
    pub render_surfaces: Vec<StableRenderSurfaceClass>,
    /// Attribution refs.
    pub attribution_refs: Vec<StableAttributionRef>,
    /// Evidence refs.
    pub evidence_refs: Vec<String>,
    /// Explainability factors.
    pub explainability_factors: Vec<ExplainabilityFactor>,
    /// Reviewer-safe summary.
    pub summary: String,
    /// Redaction class.
    pub redaction_class: StableSupportRedactionClass,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

impl From<&ProjectDoctorStableFinding> for ProjectDoctorStableFindingRow {
    fn from(finding: &ProjectDoctorStableFinding) -> Self {
        Self {
            finding_id: finding.finding_id.clone(),
            finding_code: finding.finding_code.clone(),
            probe_pack_ref: finding.probe_pack_ref.clone(),
            probe_pack_class: finding.probe_pack_class,
            severity_class: finding.severity_class,
            confidence_class: finding.confidence_class,
            recovery_handoff_class: finding.recovery_handoff_class,
            support_context_class: finding.support_context_class,
            render_surfaces: finding.render_surfaces.clone(),
            attribution_refs: finding.attribution_refs.clone(),
            evidence_refs: finding.evidence_refs.clone(),
            explainability_factors: finding.explainability_factors.clone(),
            summary: finding.summary.clone(),
            redaction_class: finding.redaction_class,
            raw_private_material_excluded: finding.raw_private_material_excluded,
        }
    }
}

/// Unsupported-state report row in the stable support packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorStableUnsupportedStateRow {
    /// Stable report id.
    pub report_id: String,
    /// Pack ref.
    pub probe_pack_ref: String,
    /// Pack class.
    pub probe_pack_class: StableProbePackClass,
    /// Unsupported state class.
    pub unsupported_state_class: UnsupportedStateClass,
    /// Unsupported finding code.
    pub unsupported_finding_code: String,
    /// Evidence refs.
    pub evidence_refs: Vec<String>,
    /// Explainability factors.
    pub explainability_factors: Vec<ExplainabilityFactor>,
    /// Reviewer-safe summary.
    pub summary: String,
    /// Redaction class.
    pub redaction_class: StableSupportRedactionClass,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

impl From<&ProjectDoctorStableUnsupportedStateReport> for ProjectDoctorStableUnsupportedStateRow {
    fn from(report: &ProjectDoctorStableUnsupportedStateReport) -> Self {
        Self {
            report_id: report.report_id.clone(),
            probe_pack_ref: report.probe_pack_ref.clone(),
            probe_pack_class: report.probe_pack_class,
            unsupported_state_class: report.unsupported_state_class,
            unsupported_finding_code: report.unsupported_finding_code.clone(),
            evidence_refs: report.evidence_refs.clone(),
            explainability_factors: report.explainability_factors.clone(),
            summary: report.summary.clone(),
            redaction_class: report.redaction_class,
            raw_private_material_excluded: report.raw_private_material_excluded,
        }
    }
}

// ---------------------------------------------------------------------------
// Validation types
// ---------------------------------------------------------------------------

/// One validation failure emitted by the stable evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectDoctorStableViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Reviewer-facing failure message.
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectDoctorStableValidationReport {
    /// Validation failures.
    pub violations: Vec<ProjectDoctorStableViolation>,
}

impl fmt::Display for ProjectDoctorStableValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} project-doctor stable violation(s)",
            self.violations.len()
        )
    }
}

impl Error for ProjectDoctorStableValidationReport {}

// ---------------------------------------------------------------------------
// Load helpers
// ---------------------------------------------------------------------------

/// Loads a stable probe-pack catalog from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`ProjectDoctorStableProbePackCatalog`].
pub fn load_stable_probe_pack_catalog(
    yaml: &str,
) -> Result<ProjectDoctorStableProbePackCatalog, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads a single stable probe-pack record from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`ProjectDoctorStableProbePack`].
pub fn load_stable_probe_pack_record(
    yaml: &str,
) -> Result<ProjectDoctorStableProbePack, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads a stable finding record from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`ProjectDoctorStableFinding`].
pub fn load_stable_finding(yaml: &str) -> Result<ProjectDoctorStableFinding, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads a stable unsupported-state report from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`ProjectDoctorStableUnsupportedStateReport`].
pub fn load_stable_unsupported_state_report(
    yaml: &str,
) -> Result<ProjectDoctorStableUnsupportedStateReport, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

// ---------------------------------------------------------------------------
// Evaluator
// ---------------------------------------------------------------------------

/// Stable Project Doctor evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProjectDoctorStableEvaluator;

impl ProjectDoctorStableEvaluator {
    /// Creates a new stable evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates the catalog.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectDoctorStableValidationReport`] when the catalog
    /// duplicates pack ids, omits the required schema/doc refs, or contains a
    /// pack that is not read-only, denied on both admission paths, or names
    /// duplicate / empty / mis-prefixed finding codes.
    pub fn validate_catalog(
        &self,
        catalog: &ProjectDoctorStableProbePackCatalog,
    ) -> Result<(), ProjectDoctorStableValidationReport> {
        let violations = validate_catalog(catalog);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(ProjectDoctorStableValidationReport { violations })
        }
    }

    /// Validates the finding shape without binding it to a catalog.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectDoctorStableValidationReport`] when the finding has the
    /// wrong record-kind, the wrong schema version, an empty or mis-prefixed
    /// finding code, empty evidence/attribution/explainability lists, a
    /// redaction class that is not metadata-safe, or
    /// `raw_private_material_excluded` set to false.
    pub fn validate_finding(
        &self,
        finding: &ProjectDoctorStableFinding,
    ) -> Result<(), ProjectDoctorStableValidationReport> {
        let violations = validate_finding(finding, None);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(ProjectDoctorStableValidationReport { violations })
        }
    }

    /// Validates the finding against its declared catalog.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectDoctorStableValidationReport`] when the finding's
    /// `probe_pack_ref` is not in the catalog, when its class/version does not
    /// match the pack, when its finding code is not in the pack's supported
    /// finding codes, or when its support-context class is not declared by the
    /// pack.
    pub fn validate_finding_against_catalog(
        &self,
        catalog: &ProjectDoctorStableProbePackCatalog,
        finding: &ProjectDoctorStableFinding,
    ) -> Result<(), ProjectDoctorStableValidationReport> {
        let mut violations = validate_finding(finding, Some(catalog));
        if validate_catalog(catalog).is_empty() {
            // Already validated.
        } else {
            violations.extend(validate_catalog(catalog));
        }
        if violations.is_empty() {
            Ok(())
        } else {
            Err(ProjectDoctorStableValidationReport { violations })
        }
    }

    /// Validates an unsupported-state report shape.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectDoctorStableValidationReport`] when the report has the
    /// wrong record-kind, wrong schema version, empty evidence, or
    /// `raw_private_material_excluded` set to false.
    pub fn validate_unsupported_state_report(
        &self,
        report: &ProjectDoctorStableUnsupportedStateReport,
    ) -> Result<(), ProjectDoctorStableValidationReport> {
        let violations = validate_unsupported_state_report(report, None);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(ProjectDoctorStableValidationReport { violations })
        }
    }

    /// Validates an unsupported-state report against its declared catalog.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectDoctorStableValidationReport`] when the report's pack
    /// ref is not in the catalog or when the unsupported state class is
    /// `None` while the report claims an unsupported finding.
    pub fn validate_unsupported_state_report_against_catalog(
        &self,
        catalog: &ProjectDoctorStableProbePackCatalog,
        report: &ProjectDoctorStableUnsupportedStateReport,
    ) -> Result<(), ProjectDoctorStableValidationReport> {
        let mut violations = validate_unsupported_state_report(report, Some(catalog));
        if validate_catalog(catalog).is_empty() {
            // Already validated.
        } else {
            violations.extend(validate_catalog(catalog));
        }
        if violations.is_empty() {
            Ok(())
        } else {
            Err(ProjectDoctorStableValidationReport { violations })
        }
    }

    /// Builds the metadata-safe stable support packet projection.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectDoctorStableValidationReport`] when the catalog or any
    /// of the bound findings or reports fail validation.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        captured_at: impl Into<String>,
        catalog: &ProjectDoctorStableProbePackCatalog,
        findings: &[ProjectDoctorStableFinding],
        reports: &[ProjectDoctorStableUnsupportedStateReport],
        chain_of_custody: &[StableChainOfCustodyEvent],
    ) -> Result<ProjectDoctorStableSupportPacket, ProjectDoctorStableValidationReport> {
        let mut violations = validate_catalog(catalog);
        for finding in findings {
            violations.extend(validate_finding(finding, Some(catalog)));
        }
        for report in reports {
            violations.extend(validate_unsupported_state_report(report, Some(catalog)));
        }
        if !violations.is_empty() {
            return Err(ProjectDoctorStableValidationReport { violations });
        }

        let pack_rows = catalog
            .packs
            .iter()
            .map(ProjectDoctorStablePackRow::from)
            .collect::<Vec<_>>();
        let finding_rows = findings
            .iter()
            .map(ProjectDoctorStableFindingRow::from)
            .collect::<Vec<_>>();
        let unsupported_state_rows = reports
            .iter()
            .map(ProjectDoctorStableUnsupportedStateRow::from)
            .collect::<Vec<_>>();

        Ok(ProjectDoctorStableSupportPacket {
            record_kind: PROJECT_DOCTOR_STABLE_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: PROJECT_DOCTOR_STABLE_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            captured_at: captured_at.into(),
            doc_ref: PROJECT_DOCTOR_STABLE_DOC_REF.to_owned(),
            schema_ref: PROJECT_DOCTOR_STABLE_SCHEMA_REF.to_owned(),
            catalog_id: catalog.catalog_id.clone(),
            catalog_version: catalog.catalog_version.clone(),
            pack_rows,
            finding_rows,
            unsupported_state_rows,
            chain_of_custody: chain_of_custody.to_vec(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        })
    }
}

// ---------------------------------------------------------------------------
// Private validation helpers
// ---------------------------------------------------------------------------

fn validate_catalog(
    catalog: &ProjectDoctorStableProbePackCatalog,
) -> Vec<ProjectDoctorStableViolation> {
    let mut violations = Vec::new();

    if catalog.schema_version != PROJECT_DOCTOR_STABLE_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "project_doctor_stable.catalog_schema_version",
            &catalog.catalog_id,
            "catalog schema_version must be 1",
        );
    }
    if catalog.record_kind != PROJECT_DOCTOR_STABLE_PROBE_PACK_CATALOG_RECORD_KIND {
        push_violation(
            &mut violations,
            "project_doctor_stable.catalog_record_kind",
            &catalog.catalog_id,
            "catalog record_kind must be project_doctor_stable_probe_pack_catalog_record",
        );
    }
    if catalog.catalog_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.catalog_id_empty",
            &catalog.catalog_id,
            "catalog_id must be non-empty",
        );
    }
    if catalog.catalog_version.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.catalog_version_empty",
            &catalog.catalog_id,
            "catalog_version must be non-empty",
        );
    }
    if catalog.schema_ref != PROJECT_DOCTOR_STABLE_SCHEMA_REF {
        push_violation(
            &mut violations,
            "project_doctor_stable.catalog_schema_ref",
            &catalog.catalog_id,
            format!("catalog schema_ref must equal {PROJECT_DOCTOR_STABLE_SCHEMA_REF}"),
        );
    }
    if catalog.doc_ref != PROJECT_DOCTOR_STABLE_DOC_REF {
        push_violation(
            &mut violations,
            "project_doctor_stable.catalog_doc_ref",
            &catalog.catalog_id,
            format!("catalog doc_ref must equal {PROJECT_DOCTOR_STABLE_DOC_REF}"),
        );
    }
    if catalog.packs.is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.catalog_packs_missing",
            &catalog.catalog_id,
            "catalog must declare at least one probe pack",
        );
    }

    let mut pack_ids: BTreeSet<&str> = BTreeSet::new();
    for pack in &catalog.packs {
        if !pack_ids.insert(pack.pack_id.as_str()) {
            push_violation(
                &mut violations,
                "project_doctor_stable.duplicate_pack_id",
                &pack.pack_id,
                "duplicate pack_id in catalog is forbidden",
            );
        }
        violations.extend(validate_pack(pack));
    }

    violations
}

fn validate_pack(pack: &StableProbePackEntry) -> Vec<ProjectDoctorStableViolation> {
    let mut violations = Vec::new();

    if pack.pack_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.pack_id_empty",
            &pack.pack_id,
            "pack_id must be non-empty",
        );
    }
    if pack.pack_version.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.pack_version_empty",
            &pack.pack_id,
            "pack_version must be non-empty",
        );
    }
    if pack.summary.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.pack_summary_empty",
            &pack.pack_id,
            "pack summary must be non-empty",
        );
    }
    if pack.supported_finding_codes.is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.pack_finding_codes_empty",
            &pack.pack_id,
            "pack must declare at least one supported finding code",
        );
    }
    if pack.supported_support_contexts.is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.pack_support_contexts_empty",
            &pack.pack_id,
            "pack must declare at least one supported support context",
        );
    }
    if pack.supported_recovery_handoffs.is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.pack_recovery_handoffs_empty",
            &pack.pack_id,
            "pack must declare at least one supported recovery handoff",
        );
    }

    let mut seen_codes: BTreeSet<&str> = BTreeSet::new();
    for code in &pack.supported_finding_codes {
        if !code.starts_with(DOCTOR_FINDING_PREFIX) {
            push_violation(
                &mut violations,
                "project_doctor_stable.pack_finding_code_prefix",
                &pack.pack_id,
                format!("finding code {code} must start with {DOCTOR_FINDING_PREFIX}"),
            );
        }
        if !seen_codes.insert(code.as_str()) {
            push_violation(
                &mut violations,
                "project_doctor_stable.pack_duplicate_finding_code",
                &pack.pack_id,
                format!("duplicate finding code {code} in pack"),
            );
        }
    }

    if matches!(
        pack.headless_admission,
        StableHeadlessAdmissionClass::Denied
    ) && matches!(
        pack.support_guided_admission,
        StableSupportGuidedAdmissionClass::Denied
    ) {
        push_violation(
            &mut violations,
            "project_doctor_stable.pack_admission_denied_everywhere",
            &pack.pack_id,
            "pack must be admitted under headless or support-guided mode",
        );
    }

    if pack.default_redaction_class != StableSupportRedactionClass::MetadataSafeDefault {
        push_violation(
            &mut violations,
            "project_doctor_stable.pack_default_redaction_not_safe",
            &pack.pack_id,
            "pack default_redaction_class must be metadata_safe_default",
        );
    }

    violations
}

fn validate_finding(
    finding: &ProjectDoctorStableFinding,
    catalog: Option<&ProjectDoctorStableProbePackCatalog>,
) -> Vec<ProjectDoctorStableViolation> {
    let mut violations = Vec::new();

    if finding.schema_version != PROJECT_DOCTOR_STABLE_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "project_doctor_stable.finding_schema_version",
            &finding.finding_id,
            "finding schema_version must be 1",
        );
    }
    if finding.record_kind != PROJECT_DOCTOR_STABLE_FINDING_RECORD_KIND {
        push_violation(
            &mut violations,
            "project_doctor_stable.finding_record_kind",
            &finding.finding_id,
            "finding record_kind must be project_doctor_stable_finding_record",
        );
    }
    if finding.finding_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.finding_id_empty",
            &finding.finding_id,
            "finding_id must be non-empty",
        );
    }
    if !finding.finding_code.starts_with(DOCTOR_FINDING_PREFIX) {
        push_violation(
            &mut violations,
            "project_doctor_stable.finding_code_prefix",
            &finding.finding_id,
            format!("finding_code must start with {DOCTOR_FINDING_PREFIX}"),
        );
    }
    if finding.summary.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.finding_summary_empty",
            &finding.finding_id,
            "finding summary must be non-empty",
        );
    }
    if finding.evidence_refs.is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.finding_evidence_missing",
            &finding.finding_id,
            "finding must cite at least one evidence ref",
        );
    }
    if finding.render_surfaces.is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.finding_render_surfaces_empty",
            &finding.finding_id,
            "finding must declare at least one render surface",
        );
    }
    if finding.explainability_factors.is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.finding_explainability_missing",
            &finding.finding_id,
            "finding must declare at least one explainability factor",
        );
    }
    if finding.redaction_class != StableSupportRedactionClass::MetadataSafeDefault {
        push_violation(
            &mut violations,
            "project_doctor_stable.finding_redaction_not_safe",
            &finding.finding_id,
            "finding redaction_class must be metadata_safe_default",
        );
    }
    if !finding.raw_private_material_excluded {
        push_violation(
            &mut violations,
            "project_doctor_stable.finding_raw_material_present",
            &finding.finding_id,
            "finding raw_private_material_excluded must be true",
        );
    }

    let has_pack_attribution = finding.attribution_refs.iter().any(|attribution| {
        attribution.attribution_kind == StableAttributionKindClass::ProbePackRef
            && attribution.ref_ == finding.probe_pack_ref
    });
    if !has_pack_attribution {
        push_violation(
            &mut violations,
            "project_doctor_stable.finding_pack_attribution_missing",
            &finding.finding_id,
            "finding must carry a probe_pack_ref attribution back to probe_pack_ref",
        );
    }

    if let Some(catalog) = catalog {
        let pack_index: BTreeMap<&str, &StableProbePackEntry> = catalog
            .packs
            .iter()
            .map(|pack| (pack.pack_id.as_str(), pack))
            .collect();
        match pack_index.get(finding.probe_pack_ref.as_str()) {
            None => push_violation(
                &mut violations,
                "project_doctor_stable.finding_pack_ref_unknown",
                &finding.finding_id,
                format!(
                    "probe_pack_ref {} not present in catalog {}",
                    finding.probe_pack_ref, catalog.catalog_id
                ),
            ),
            Some(pack) => {
                if pack.pack_class != finding.probe_pack_class {
                    push_violation(
                        &mut violations,
                        "project_doctor_stable.finding_pack_class_mismatch",
                        &finding.finding_id,
                        format!(
                            "finding probe_pack_class {} does not match catalog pack_class {} for {}",
                            finding.probe_pack_class,
                            pack.pack_class,
                            pack.pack_id
                        ),
                    );
                }
                if pack.pack_version != finding.probe_pack_version {
                    push_violation(
                        &mut violations,
                        "project_doctor_stable.finding_pack_version_mismatch",
                        &finding.finding_id,
                        format!(
                            "finding probe_pack_version {} does not match catalog pack_version {} for {}",
                            finding.probe_pack_version,
                            pack.pack_version,
                            pack.pack_id
                        ),
                    );
                }
                if !pack
                    .supported_finding_codes
                    .iter()
                    .any(|code| code == &finding.finding_code)
                {
                    push_violation(
                        &mut violations,
                        "project_doctor_stable.finding_code_not_supported",
                        &finding.finding_id,
                        format!(
                            "finding_code {} is not in supported codes of pack {}",
                            finding.finding_code, pack.pack_id
                        ),
                    );
                }
                if !pack
                    .supported_support_contexts
                    .iter()
                    .any(|context| *context == finding.support_context_class)
                {
                    push_violation(
                        &mut violations,
                        "project_doctor_stable.finding_support_context_not_supported",
                        &finding.finding_id,
                        format!(
                            "support_context_class {} not declared by pack {}",
                            finding.support_context_class.as_str(),
                            pack.pack_id
                        ),
                    );
                }
                if !pack
                    .supported_recovery_handoffs
                    .iter()
                    .any(|handoff| *handoff == finding.recovery_handoff_class)
                {
                    push_violation(
                        &mut violations,
                        "project_doctor_stable.finding_recovery_handoff_not_supported",
                        &finding.finding_id,
                        format!(
                            "recovery_handoff_class {} not declared by pack {}",
                            finding.recovery_handoff_class.as_str(),
                            pack.pack_id
                        ),
                    );
                }
                if matches!(
                    finding.support_context_class,
                    StableSupportContextClass::CliHeadless
                ) && matches!(
                    pack.headless_admission,
                    StableHeadlessAdmissionClass::Denied
                ) {
                    push_violation(
                        &mut violations,
                        "project_doctor_stable.finding_headless_denied",
                        &finding.finding_id,
                        format!(
                            "pack {} denies headless admission; cli_headless findings are forbidden",
                            pack.pack_id
                        ),
                    );
                }
                if matches!(
                    finding.support_context_class,
                    StableSupportContextClass::SupportGuided
                ) && matches!(
                    pack.support_guided_admission,
                    StableSupportGuidedAdmissionClass::Denied
                ) {
                    push_violation(
                        &mut violations,
                        "project_doctor_stable.finding_support_guided_denied",
                        &finding.finding_id,
                        format!(
                            "pack {} denies support-guided admission; support_guided findings are forbidden",
                            pack.pack_id
                        ),
                    );
                }
            }
        }
    }

    violations
}

fn validate_unsupported_state_report(
    report: &ProjectDoctorStableUnsupportedStateReport,
    catalog: Option<&ProjectDoctorStableProbePackCatalog>,
) -> Vec<ProjectDoctorStableViolation> {
    let mut violations = Vec::new();

    if report.schema_version != PROJECT_DOCTOR_STABLE_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "project_doctor_stable.report_schema_version",
            &report.report_id,
            "report schema_version must be 1",
        );
    }
    if report.record_kind != PROJECT_DOCTOR_STABLE_UNSUPPORTED_STATE_REPORT_RECORD_KIND {
        push_violation(
            &mut violations,
            "project_doctor_stable.report_record_kind",
            &report.report_id,
            "report record_kind must be project_doctor_stable_unsupported_state_report_record",
        );
    }
    if report.report_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.report_id_empty",
            &report.report_id,
            "report_id must be non-empty",
        );
    }
    if !report.unsupported_finding_code.is_empty()
        && !report
            .unsupported_finding_code
            .starts_with(DOCTOR_FINDING_PREFIX)
    {
        push_violation(
            &mut violations,
            "project_doctor_stable.report_finding_code_prefix",
            &report.report_id,
            format!(
                "unsupported_finding_code must start with {DOCTOR_FINDING_PREFIX} when non-empty"
            ),
        );
    }
    if report.evidence_refs.is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.report_evidence_missing",
            &report.report_id,
            "report must cite at least one evidence ref",
        );
    }
    if report.summary.trim().is_empty() {
        push_violation(
            &mut violations,
            "project_doctor_stable.report_summary_empty",
            &report.report_id,
            "report summary must be non-empty",
        );
    }
    if report.redaction_class != StableSupportRedactionClass::MetadataSafeDefault {
        push_violation(
            &mut violations,
            "project_doctor_stable.report_redaction_not_safe",
            &report.report_id,
            "report redaction_class must be metadata_safe_default",
        );
    }
    if !report.raw_private_material_excluded {
        push_violation(
            &mut violations,
            "project_doctor_stable.report_raw_material_present",
            &report.report_id,
            "report raw_private_material_excluded must be true",
        );
    }
    if matches!(report.unsupported_state_class, UnsupportedStateClass::None)
        && !report.unsupported_finding_code.is_empty()
    {
        push_violation(
            &mut violations,
            "project_doctor_stable.report_none_with_finding_code",
            &report.report_id,
            "unsupported_state_class is None but unsupported_finding_code is non-empty",
        );
    }

    if let Some(catalog) = catalog {
        let pack_index: BTreeMap<&str, &StableProbePackEntry> = catalog
            .packs
            .iter()
            .map(|pack| (pack.pack_id.as_str(), pack))
            .collect();
        match pack_index.get(report.probe_pack_ref.as_str()) {
            None => push_violation(
                &mut violations,
                "project_doctor_stable.report_pack_ref_unknown",
                &report.report_id,
                format!(
                    "probe_pack_ref {} not present in catalog {}",
                    report.probe_pack_ref, catalog.catalog_id
                ),
            ),
            Some(pack) => {
                if pack.pack_class != report.probe_pack_class {
                    push_violation(
                        &mut violations,
                        "project_doctor_stable.report_pack_class_mismatch",
                        &report.report_id,
                        format!(
                            "report probe_pack_class {} does not match catalog pack_class {} for {}",
                            report.probe_pack_class,
                            pack.pack_class,
                            pack.pack_id
                        ),
                    );
                }
            }
        }
    }

    violations
}

fn push_violation(
    violations: &mut Vec<ProjectDoctorStableViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(ProjectDoctorStableViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}
