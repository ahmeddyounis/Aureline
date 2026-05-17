//! Beta Project Doctor probe-pack family catalog.
//!
//! This module owns the beta-grade probe-pack catalog covering the seven
//! named failure families (entry, toolchain, search/index, trust/policy,
//! Git, provider, restore). It promotes the alpha probe runtime into a
//! versioned, named pack catalog whose every row pins prerequisites,
//! outputs that map a stable `doctor.finding.*` code to a recovery-ladder
//! action (safe mode, bisect, repair preview, locate, reauthenticate,
//! handoff, etc.), and unsupported-state handling so the recovery path is
//! not invented at diagnosis time.
//!
//! The module mirrors the boundary schema at
//! `/schemas/support/doctor_probe_pack.schema.json`. The
//! [`DoctorProbePackCatalog`] holds one pack per family, each
//! [`DoctorProbePackRecord`] declares its prerequisites and outputs, and
//! the [`DoctorProbePackCoverageScorecard`] projection lets supportability
//! scorecards see family coverage without scraping rendered text.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Frozen schema version for the doctor probe-pack records.
pub const DOCTOR_PROBE_PACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for one doctor probe-pack record.
pub const DOCTOR_PROBE_PACK_RECORD_KIND: &str = "doctor_probe_pack_record";

/// Stable record-kind tag for the doctor probe-pack catalog record.
pub const DOCTOR_PROBE_PACK_CATALOG_RECORD_KIND: &str = "doctor_probe_pack_catalog_record";

/// Stable record-kind tag for the doctor probe-pack coverage scorecard
/// projection.
pub const DOCTOR_PROBE_PACK_COVERAGE_SCORECARD_RECORD_KIND: &str =
    "doctor_probe_pack_coverage_scorecard_record";

/// Repo-relative path of the boundary schema mirrored by this module.
pub const DOCTOR_PROBE_PACK_SCHEMA_REF: &str = "schemas/support/doctor_probe_pack.schema.json";

/// Reviewer doc ref quoted by every emitted record.
pub const DOCTOR_PROBE_PACK_DOC_REF: &str = "docs/support/m3/doctor_probe_packs_beta.md";

/// Stable finding-code prefix every output must use.
pub const DOCTOR_FINDING_PREFIX: &str = "doctor.finding.";

/// Closed failure-family vocabulary for the doctor probe-pack lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureFamilyClass {
    /// Opening, clone/import handoff, or recent-workspace entry failed.
    Entry,
    /// Execution-context or toolchain selection failed.
    Toolchain,
    /// Search or indexing is not ready enough for the requested scope.
    SearchIndex,
    /// Trust, identity, or policy blocked the requested capability.
    TrustPolicy,
    /// Local Git status or repository identity is unavailable or degraded.
    Git,
    /// Connected provider or credential authority cannot admit the action.
    Provider,
    /// Session restore, crash replay, or continuity hydration is unsafe.
    Restore,
}

impl FailureFamilyClass {
    /// Returns every failure family in catalog order.
    pub const fn all() -> [Self; 7] {
        [
            Self::Entry,
            Self::Toolchain,
            Self::SearchIndex,
            Self::TrustPolicy,
            Self::Git,
            Self::Provider,
            Self::Restore,
        ]
    }

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Entry => "entry",
            Self::Toolchain => "toolchain",
            Self::SearchIndex => "search_index",
            Self::TrustPolicy => "trust_policy",
            Self::Git => "git",
            Self::Provider => "provider",
            Self::Restore => "restore",
        }
    }

    /// Stable beta probe-pack class for the family.
    pub const fn pack_class(self) -> ProbePackClass {
        match self {
            Self::Entry => ProbePackClass::EntryOpenReadiness,
            Self::Toolchain => ProbePackClass::ToolchainResolution,
            Self::SearchIndex => ProbePackClass::SearchIndexReadiness,
            Self::TrustPolicy => ProbePackClass::TrustPolicy,
            Self::Git => ProbePackClass::GitBaseline,
            Self::Provider => ProbePackClass::ProviderAuth,
            Self::Restore => ProbePackClass::RestoreContinuity,
        }
    }
}

impl fmt::Display for FailureFamilyClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Mirror of the beta `project_doctor` probe-pack class for the seven
/// families covered by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbePackClass {
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
}

impl ProbePackClass {
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
        }
    }
}

impl fmt::Display for ProbePackClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Closed prerequisite-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrerequisiteClass {
    /// Recent-workspace / clone-import admission manifest.
    AdmissionManifest,
    /// Entry intent record (locate-or-restore intent).
    EntryIntentRecord,
    /// Resolved execution-context / toolchain manifest.
    ExecutionContextManifest,
    /// Search/index readiness or scope-coverage record.
    SearchIndexStatusRecord,
    /// Trust or capability policy-decision record.
    PolicyDecisionRecord,
    /// Workspace-trust state record.
    TrustStateRecord,
    /// Local Git workspace state (HEAD, dirty, identity).
    GitWorkspaceState,
    /// Provider / credential authority state record.
    CredentialStateRecord,
    /// Session restore / crash replay manifest.
    SessionRestoreManifest,
}

impl PrerequisiteClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmissionManifest => "admission_manifest",
            Self::EntryIntentRecord => "entry_intent_record",
            Self::ExecutionContextManifest => "execution_context_manifest",
            Self::SearchIndexStatusRecord => "search_index_status_record",
            Self::PolicyDecisionRecord => "policy_decision_record",
            Self::TrustStateRecord => "trust_state_record",
            Self::GitWorkspaceState => "git_workspace_state",
            Self::CredentialStateRecord => "credential_state_record",
            Self::SessionRestoreManifest => "session_restore_manifest",
        }
    }
}

/// Closed recovery-ladder action class. Every output finding code routes to
/// one of these recovery steps so diagnosis never invents a recovery path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryLadderActionClass {
    /// Enter safe mode with a typed safe-mode profile.
    EnterSafeMode,
    /// Start a typed extension-bisect session.
    StartExtensionBisect,
    /// Open a typed repair-preview transaction.
    OpenRepairPreview,
    /// Locate the missing entry target or open with minimal context.
    LocateMissingTarget,
    /// Re-resolve the toolchain from existing manifests.
    ReresolveToolchain,
    /// Open the search/index readiness status surface.
    OpenIndexStatus,
    /// Open the policy / trust details surface.
    OpenPolicyDetails,
    /// Open the local Git baseline details surface.
    OpenGitBaselineDetails,
    /// Reauthenticate or renew a provider handle.
    ReauthenticateProvider,
    /// Open the workspace without unsafe restore replay.
    OpenWithoutRestore,
    /// Hand off to support with a governed escalation packet.
    HandoffToSupport,
}

impl RecoveryLadderActionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnterSafeMode => "enter_safe_mode",
            Self::StartExtensionBisect => "start_extension_bisect",
            Self::OpenRepairPreview => "open_repair_preview",
            Self::LocateMissingTarget => "locate_missing_target",
            Self::ReresolveToolchain => "reresolve_toolchain",
            Self::OpenIndexStatus => "open_index_status",
            Self::OpenPolicyDetails => "open_policy_details",
            Self::OpenGitBaselineDetails => "open_git_baseline_details",
            Self::ReauthenticateProvider => "reauthenticate_provider",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::HandoffToSupport => "handoff_to_support",
        }
    }
}

/// Closed unsupported-state class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnsupportedStateClass {
    /// Required evidence is not available; the probe cannot diagnose.
    EvidenceUnavailable,
    /// The requested scope is out of bounds for this pack.
    ScopeOutOfBounds,
    /// A required dependency record is missing.
    DependencyMissing,
    /// The current support context is not admitted by the pack.
    ContextNotAdmitted,
    /// The host platform is not supported by the pack.
    UnsupportedPlatform,
}

impl UnsupportedStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceUnavailable => "evidence_unavailable",
            Self::ScopeOutOfBounds => "scope_out_of_bounds",
            Self::DependencyMissing => "dependency_missing",
            Self::ContextNotAdmitted => "context_not_admitted",
            Self::UnsupportedPlatform => "unsupported_platform",
        }
    }
}

/// Closed severity class for one output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeverityClass {
    /// Informational output.
    Info,
    /// Degraded but still usable.
    Degraded,
    /// Blocks the requested workflow until handled.
    Blocking,
    /// Unsupported in the current context.
    Unsupported,
}

impl SeverityClass {
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

/// One typed prerequisite row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorProbePackPrerequisite {
    /// Stable prerequisite class.
    pub prerequisite_class: PrerequisiteClass,
    /// Opaque evidence reference.
    pub evidence_ref: String,
    /// Reviewer-facing description.
    pub description: String,
}

/// One typed output row mapping a stable finding code to a recovery action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorProbePackOutput {
    /// Stable finding code (must begin with `doctor.finding.`).
    pub finding_code: String,
    /// Severity class.
    pub severity_class: SeverityClass,
    /// Recovery ladder action class.
    pub recovery_action_class: RecoveryLadderActionClass,
    /// Opaque recovery-step ref (safe-mode profile id, bisect session id,
    /// repair-preview ref, runbook id, etc.).
    pub recovery_step_ref: String,
    /// Reviewer-facing description.
    pub description: String,
}

/// Typed unsupported-state handling row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorProbePackUnsupportedHandling {
    /// Stable unsupported-state class.
    pub unsupported_state_class: UnsupportedStateClass,
    /// Stable finding code emitted when the unsupported state is observed.
    pub unsupported_finding_code: String,
    /// Recovery action used as a handoff for the unsupported state.
    pub handoff_action_class: RecoveryLadderActionClass,
    /// Opaque handoff ref.
    pub handoff_ref: String,
    /// Reviewer-facing description.
    pub description: String,
}

/// One doctor probe-pack record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorProbePackRecord {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable pack identifier.
    pub pack_id: String,
    /// Failure family covered by the pack.
    pub failure_family_class: FailureFamilyClass,
    /// Beta probe-pack class.
    pub pack_class: ProbePackClass,
    /// Stable pack version.
    pub pack_version: String,
    /// Ref into the beta `project_doctor` catalog (the runtime pack).
    pub doctor_pack_ref: String,
    /// Reviewer-safe summary.
    pub summary: String,
    /// Prerequisites consumed by the pack.
    pub prerequisites: Vec<DoctorProbePackPrerequisite>,
    /// Outputs mapped to recovery-ladder actions.
    pub outputs: Vec<DoctorProbePackOutput>,
    /// Unsupported-state handling.
    pub unsupported_state_handling: DoctorProbePackUnsupportedHandling,
    /// Capture timestamp.
    pub captured_at: String,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
}

/// Doctor probe-pack catalog record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorProbePackCatalog {
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
    /// Pack records covering every failure family.
    pub packs: Vec<DoctorProbePackRecord>,
}

/// One coverage row in the supportability scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorProbePackCoverageRow {
    /// Failure family.
    pub failure_family_class: FailureFamilyClass,
    /// Pack id covering the family.
    pub pack_id: String,
    /// Pack version covering the family.
    pub pack_version: String,
    /// Number of stable finding codes the pack declares.
    pub finding_code_count: usize,
    /// Number of distinct recovery actions the pack routes through.
    pub recovery_action_count: usize,
    /// Whether the pack declares unsupported-state handling.
    pub unsupported_handling_present: bool,
    /// Whether the family is fully covered.
    pub covered: bool,
}

/// Doctor probe-pack coverage scorecard projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorProbePackCoverageScorecard {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Catalog id that backed the projection.
    pub catalog_id: String,
    /// Catalog version that backed the projection.
    pub catalog_version: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// One row per family.
    pub rows: Vec<DoctorProbePackCoverageRow>,
    /// Number of families covered.
    pub families_covered: usize,
    /// Families still uncovered (empty when scorecard is green).
    pub families_uncovered: Vec<FailureFamilyClass>,
    /// True when raw private material is excluded from the projection.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority is excluded from the projection.
    pub ambient_authority_excluded: bool,
}

impl DoctorProbePackCoverageScorecard {
    /// True when every required family is covered exactly once.
    pub fn is_fully_covered(&self) -> bool {
        self.families_uncovered.is_empty()
            && self.families_covered == FailureFamilyClass::all().len()
            && self.rows.iter().all(|row| row.covered)
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
    }
}

/// One validation failure emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorProbePackViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Reviewer-facing failure message.
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorProbePackValidationReport {
    /// Validation failures.
    pub violations: Vec<DoctorProbePackViolation>,
}

impl fmt::Display for DoctorProbePackValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} doctor-probe-pack violation(s)",
            self.violations.len()
        )
    }
}

impl Error for DoctorProbePackValidationReport {}

/// Loads a doctor probe-pack record from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`DoctorProbePackRecord`].
pub fn load_doctor_probe_pack(yaml: &str) -> Result<DoctorProbePackRecord, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads a doctor probe-pack catalog from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`DoctorProbePackCatalog`].
pub fn load_doctor_probe_pack_catalog(
    yaml: &str,
) -> Result<DoctorProbePackCatalog, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Doctor probe-pack evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct DoctorProbePackEvaluator;

impl DoctorProbePackEvaluator {
    /// Creates a new evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates one doctor probe-pack record.
    ///
    /// # Errors
    ///
    /// Returns [`DoctorProbePackValidationReport`] when the record has the
    /// wrong schema version, the wrong record kind, missing prerequisites or
    /// outputs, mis-prefixed finding codes, duplicate finding codes, a
    /// mismatched pack class for its declared family, or an unsupported
    /// handling row that points at an output finding code.
    pub fn validate_pack(
        &self,
        pack: &DoctorProbePackRecord,
    ) -> Result<(), DoctorProbePackValidationReport> {
        let violations = validate_pack(pack);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(DoctorProbePackValidationReport { violations })
        }
    }

    /// Validates a doctor probe-pack catalog.
    ///
    /// # Errors
    ///
    /// Returns [`DoctorProbePackValidationReport`] when the catalog omits a
    /// failure family, declares duplicate families or pack ids, or contains
    /// a pack that fails [`Self::validate_pack`].
    pub fn validate_catalog(
        &self,
        catalog: &DoctorProbePackCatalog,
    ) -> Result<(), DoctorProbePackValidationReport> {
        let violations = validate_catalog(catalog);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(DoctorProbePackValidationReport { violations })
        }
    }

    /// Builds the coverage-scorecard projection for the catalog.
    ///
    /// # Errors
    ///
    /// Returns [`DoctorProbePackValidationReport`] when the catalog fails
    /// [`Self::validate_catalog`].
    pub fn coverage_scorecard(
        &self,
        catalog: &DoctorProbePackCatalog,
    ) -> Result<DoctorProbePackCoverageScorecard, DoctorProbePackValidationReport> {
        let violations = validate_catalog(catalog);
        if !violations.is_empty() {
            return Err(DoctorProbePackValidationReport { violations });
        }

        let mut rows: Vec<DoctorProbePackCoverageRow> = catalog
            .packs
            .iter()
            .map(|pack| {
                let unique_actions: BTreeSet<_> = pack
                    .outputs
                    .iter()
                    .map(|output| output.recovery_action_class)
                    .collect();
                DoctorProbePackCoverageRow {
                    failure_family_class: pack.failure_family_class,
                    pack_id: pack.pack_id.clone(),
                    pack_version: pack.pack_version.clone(),
                    finding_code_count: pack.outputs.len(),
                    recovery_action_count: unique_actions.len(),
                    unsupported_handling_present: true,
                    covered: true,
                }
            })
            .collect();
        rows.sort_by(|left, right| {
            left.failure_family_class
                .cmp(&right.failure_family_class)
                .then_with(|| left.pack_id.cmp(&right.pack_id))
        });

        let covered_families: BTreeSet<FailureFamilyClass> = rows
            .iter()
            .map(|row| row.failure_family_class)
            .collect();
        let families_uncovered: Vec<FailureFamilyClass> = FailureFamilyClass::all()
            .into_iter()
            .filter(|family| !covered_families.contains(family))
            .collect();

        Ok(DoctorProbePackCoverageScorecard {
            schema_version: DOCTOR_PROBE_PACK_SCHEMA_VERSION,
            record_kind: DOCTOR_PROBE_PACK_COVERAGE_SCORECARD_RECORD_KIND.to_owned(),
            catalog_id: catalog.catalog_id.clone(),
            catalog_version: catalog.catalog_version.clone(),
            captured_at: catalog.captured_at.clone(),
            doc_ref: catalog.doc_ref.clone(),
            schema_ref: catalog.schema_ref.clone(),
            rows,
            families_covered: covered_families.len(),
            families_uncovered,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        })
    }
}

fn validate_pack(pack: &DoctorProbePackRecord) -> Vec<DoctorProbePackViolation> {
    let mut violations = Vec::new();

    if pack.schema_version != DOCTOR_PROBE_PACK_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "doctor_probe_pack.schema_version",
            &pack.pack_id,
            "pack schema_version must be 1",
        );
    }
    if pack.record_kind != DOCTOR_PROBE_PACK_RECORD_KIND {
        push_violation(
            &mut violations,
            "doctor_probe_pack.record_kind",
            &pack.pack_id,
            format!("pack record_kind must equal {DOCTOR_PROBE_PACK_RECORD_KIND}"),
        );
    }
    if pack.pack_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "doctor_probe_pack.pack_id_empty",
            &pack.pack_id,
            "pack_id must be non-empty",
        );
    }
    if pack.pack_version.trim().is_empty() {
        push_violation(
            &mut violations,
            "doctor_probe_pack.pack_version_empty",
            &pack.pack_id,
            "pack_version must be non-empty",
        );
    }
    if pack.doctor_pack_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "doctor_probe_pack.doctor_pack_ref_empty",
            &pack.pack_id,
            "doctor_pack_ref must be non-empty",
        );
    }
    if pack.summary.trim().is_empty() {
        push_violation(
            &mut violations,
            "doctor_probe_pack.summary_empty",
            &pack.pack_id,
            "pack summary must be non-empty",
        );
    }
    if pack.schema_ref != DOCTOR_PROBE_PACK_SCHEMA_REF {
        push_violation(
            &mut violations,
            "doctor_probe_pack.schema_ref",
            &pack.pack_id,
            format!("pack schema_ref must equal {DOCTOR_PROBE_PACK_SCHEMA_REF}"),
        );
    }
    if pack.doc_ref != DOCTOR_PROBE_PACK_DOC_REF {
        push_violation(
            &mut violations,
            "doctor_probe_pack.doc_ref",
            &pack.pack_id,
            format!("pack doc_ref must equal {DOCTOR_PROBE_PACK_DOC_REF}"),
        );
    }
    if pack.pack_class != pack.failure_family_class.pack_class() {
        push_violation(
            &mut violations,
            "doctor_probe_pack.pack_class_family_mismatch",
            &pack.pack_id,
            format!(
                "pack_class {} does not belong to failure_family_class {}",
                pack.pack_class, pack.failure_family_class
            ),
        );
    }

    if pack.prerequisites.is_empty() {
        push_violation(
            &mut violations,
            "doctor_probe_pack.prerequisites_empty",
            &pack.pack_id,
            "pack must declare at least one prerequisite",
        );
    }
    for prerequisite in &pack.prerequisites {
        if prerequisite.evidence_ref.trim().is_empty() {
            push_violation(
                &mut violations,
                "doctor_probe_pack.prerequisite_evidence_ref_empty",
                &pack.pack_id,
                "prerequisite evidence_ref must be non-empty",
            );
        }
        if prerequisite.description.trim().is_empty() {
            push_violation(
                &mut violations,
                "doctor_probe_pack.prerequisite_description_empty",
                &pack.pack_id,
                "prerequisite description must be non-empty",
            );
        }
    }

    if pack.outputs.is_empty() {
        push_violation(
            &mut violations,
            "doctor_probe_pack.outputs_empty",
            &pack.pack_id,
            "pack must declare at least one output",
        );
    }
    let mut output_codes: BTreeSet<&str> = BTreeSet::new();
    for output in &pack.outputs {
        if !output.finding_code.starts_with(DOCTOR_FINDING_PREFIX) {
            push_violation(
                &mut violations,
                "doctor_probe_pack.output_finding_code_prefix",
                &pack.pack_id,
                format!(
                    "output finding_code {} must start with {DOCTOR_FINDING_PREFIX}",
                    output.finding_code
                ),
            );
        }
        if !output_codes.insert(output.finding_code.as_str()) {
            push_violation(
                &mut violations,
                "doctor_probe_pack.output_finding_code_duplicate",
                &pack.pack_id,
                format!(
                    "duplicate output finding_code {} in pack",
                    output.finding_code
                ),
            );
        }
        if output.recovery_step_ref.trim().is_empty() {
            push_violation(
                &mut violations,
                "doctor_probe_pack.output_recovery_step_ref_empty",
                &pack.pack_id,
                format!(
                    "output {} must declare a non-empty recovery_step_ref",
                    output.finding_code
                ),
            );
        }
        if output.description.trim().is_empty() {
            push_violation(
                &mut violations,
                "doctor_probe_pack.output_description_empty",
                &pack.pack_id,
                format!("output {} must declare a description", output.finding_code),
            );
        }
    }

    let unsupported = &pack.unsupported_state_handling;
    if !unsupported
        .unsupported_finding_code
        .starts_with(DOCTOR_FINDING_PREFIX)
    {
        push_violation(
            &mut violations,
            "doctor_probe_pack.unsupported_finding_code_prefix",
            &pack.pack_id,
            format!(
                "unsupported_finding_code {} must start with {DOCTOR_FINDING_PREFIX}",
                unsupported.unsupported_finding_code
            ),
        );
    }
    if unsupported.handoff_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "doctor_probe_pack.unsupported_handoff_ref_empty",
            &pack.pack_id,
            "unsupported_state_handling.handoff_ref must be non-empty",
        );
    }
    if unsupported.description.trim().is_empty() {
        push_violation(
            &mut violations,
            "doctor_probe_pack.unsupported_description_empty",
            &pack.pack_id,
            "unsupported_state_handling.description must be non-empty",
        );
    }
    if output_codes.contains(unsupported.unsupported_finding_code.as_str()) {
        push_violation(
            &mut violations,
            "doctor_probe_pack.unsupported_finding_code_collision",
            &pack.pack_id,
            format!(
                "unsupported_finding_code {} must not be reused as an output finding code",
                unsupported.unsupported_finding_code
            ),
        );
    }

    violations
}

fn validate_catalog(catalog: &DoctorProbePackCatalog) -> Vec<DoctorProbePackViolation> {
    let mut violations = Vec::new();

    if catalog.schema_version != DOCTOR_PROBE_PACK_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "doctor_probe_pack.catalog_schema_version",
            &catalog.catalog_id,
            "catalog schema_version must be 1",
        );
    }
    if catalog.record_kind != DOCTOR_PROBE_PACK_CATALOG_RECORD_KIND {
        push_violation(
            &mut violations,
            "doctor_probe_pack.catalog_record_kind",
            &catalog.catalog_id,
            format!("catalog record_kind must equal {DOCTOR_PROBE_PACK_CATALOG_RECORD_KIND}"),
        );
    }
    if catalog.catalog_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "doctor_probe_pack.catalog_id_empty",
            &catalog.catalog_id,
            "catalog_id must be non-empty",
        );
    }
    if catalog.catalog_version.trim().is_empty() {
        push_violation(
            &mut violations,
            "doctor_probe_pack.catalog_version_empty",
            &catalog.catalog_id,
            "catalog_version must be non-empty",
        );
    }
    if catalog.schema_ref != DOCTOR_PROBE_PACK_SCHEMA_REF {
        push_violation(
            &mut violations,
            "doctor_probe_pack.catalog_schema_ref",
            &catalog.catalog_id,
            format!("catalog schema_ref must equal {DOCTOR_PROBE_PACK_SCHEMA_REF}"),
        );
    }
    if catalog.doc_ref != DOCTOR_PROBE_PACK_DOC_REF {
        push_violation(
            &mut violations,
            "doctor_probe_pack.catalog_doc_ref",
            &catalog.catalog_id,
            format!("catalog doc_ref must equal {DOCTOR_PROBE_PACK_DOC_REF}"),
        );
    }

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    let mut seen_families: BTreeSet<FailureFamilyClass> = BTreeSet::new();
    for pack in &catalog.packs {
        if !seen_ids.insert(pack.pack_id.as_str()) {
            push_violation(
                &mut violations,
                "doctor_probe_pack.catalog_duplicate_pack_id",
                &pack.pack_id,
                "duplicate pack_id in catalog is forbidden",
            );
        }
        if !seen_families.insert(pack.failure_family_class) {
            push_violation(
                &mut violations,
                "doctor_probe_pack.catalog_duplicate_family",
                &pack.pack_id,
                format!(
                    "duplicate failure_family_class {} in catalog is forbidden",
                    pack.failure_family_class
                ),
            );
        }
        violations.extend(validate_pack(pack));
    }

    for required in FailureFamilyClass::all() {
        if !seen_families.contains(&required) {
            push_violation(
                &mut violations,
                "doctor_probe_pack.catalog_family_missing",
                &catalog.catalog_id,
                format!(
                    "catalog must declare a pack for failure_family_class {required}"
                ),
            );
        }
    }

    violations
}

fn push_violation(
    violations: &mut Vec<DoctorProbePackViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(DoctorProbePackViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}
