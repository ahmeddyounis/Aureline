//! Canonical save-truth packet for generated, virtual, exported, draft, and
//! remotely backed artifact families.
//!
//! This module extends the existing filesystem-identity, save-review, and
//! source-fidelity records with one shared packet that answers four questions
//! for every admitted artifact family:
//!
//! 1. which metadata-sensitive cues must stay visible before save,
//! 2. which fallback or alternate-target actions appear when the preferred save
//!    path is unavailable,
//! 3. which mutating save participants must rebase or abort on target drift,
//!    and
//! 4. which worked fixtures prove lossy-decode, metadata-preservation,
//!    execute-bit, regenerate/export, and logical-target safety posture.
//!
//! The packet is metadata-only and export-safe. It carries closed-vocabulary
//! rows, action affordances, and fixture references without embedding raw file
//! bytes, raw provider payloads, or raw patches.

use std::collections::BTreeSet;
use std::fmt;

use aureline_vfs::{
    MatrixPathIdentityClass, MatrixRootClass, MatrixSaveFallback, MatrixSurfaceClass,
};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version stamped onto packet and fixture records.
pub const ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the checked-in packet.
pub const ARTIFACT_SAVE_TRUTH_PACKET_RECORD_KIND: &str = "artifact_save_truth_packet_record";

/// Stable record-kind tag for checked-in fixtures.
pub const ARTIFACT_SAVE_TRUTH_FIXTURE_RECORD_KIND: &str = "artifact_save_truth_fixture_record";

/// Repo-relative schema reference.
pub const ARTIFACT_SAVE_TRUTH_SCHEMA_REF: &str = "schemas/state/artifact_save_truth.schema.json";

/// Repo-relative narrative documentation reference.
pub const ARTIFACT_SAVE_TRUTH_DOC_REF: &str = "docs/state/artifact_save_truth.md";

/// Repo-relative machine-readable packet reference.
pub const ARTIFACT_SAVE_TRUTH_PACKET_REF: &str = "artifacts/state/artifact_save_truth.json";

/// Repo-relative reviewer-facing report reference.
pub const ARTIFACT_SAVE_TRUTH_REPORT_REF: &str = "artifacts/state/artifact_save_truth.md";

/// Repo-relative fixture directory.
pub const ARTIFACT_SAVE_TRUTH_FIXTURE_DIR: &str = "fixtures/state/artifact_save_truth";

/// Repo-relative fixture README.
pub const ARTIFACT_SAVE_TRUTH_FIXTURE_README_REF: &str =
    "fixtures/state/artifact_save_truth/README.md";

/// Checked-in packet JSON embedded for CLI/headless parity tests.
pub const ARTIFACT_SAVE_TRUTH_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/state/artifact_save_truth.json"
));

/// Metadata-sensitive cue that must remain visible before save.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetadataSensitivityIndicator {
    /// Encoding or decode posture may change the durable bytes.
    Encoding,
    /// Newline mode is part of the durable representation contract.
    NewlineMode,
    /// BOM or final-newline posture must remain explicit.
    BomOrFinalNewline,
    /// Execute-bit, mode, or permission-sensitive metadata matters.
    ExecuteBitOrPermissions,
    /// The visible object is generated or derived from another source of truth.
    GeneratedStateBoundary,
    /// Presentation path and durable target identity may diverge.
    LogicalTargetAmbiguity,
}

impl MetadataSensitivityIndicator {
    /// Returns the stable token for this indicator.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Encoding => "encoding",
            Self::NewlineMode => "newline_mode",
            Self::BomOrFinalNewline => "bom_or_final_newline",
            Self::ExecuteBitOrPermissions => "execute_bit_or_permissions",
            Self::GeneratedStateBoundary => "generated_state_boundary",
            Self::LogicalTargetAmbiguity => "logical_target_ambiguity",
        }
    }
}

/// Fallback disclosure family shown when the preferred save path is not
/// available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveFallbackDisclosureClass {
    /// Prefer atomic replace, but disclose an in-place or conditional fallback.
    AtomicWithDisclosedFallback,
    /// Regeneration or export is the truthful save path, not direct overwrite.
    RegenerateOrExportDisclosed,
    /// Draft locally first, then reconcile or publish through a review step.
    DraftStageDisclosed,
    /// Direct save is blocked; the surface is compare-first or inspect-only.
    CompareOnlyBlocked,
}

impl SaveFallbackDisclosureClass {
    /// Returns the stable token for this disclosure class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AtomicWithDisclosedFallback => "atomic_with_disclosed_fallback",
            Self::RegenerateOrExportDisclosed => "regenerate_or_export_disclosed",
            Self::DraftStageDisclosed => "draft_stage_disclosed",
            Self::CompareOnlyBlocked => "compare_only_blocked",
        }
    }
}

/// Action row shown on a fallback disclosure sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackActionClass {
    /// Continue through the admitted degraded path after disclosure.
    Continue,
    /// Inspect the safety note explaining what may be lost or weakened.
    InspectSafetyNote,
    /// Route the write to a distinct admitted target instead.
    AlternateTarget,
    /// Compare the staged content with the current authoritative target first.
    CompareBeforeSave,
}

impl FallbackActionClass {
    /// Returns the stable token for this fallback action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Continue => "continue",
            Self::InspectSafetyNote => "inspect_safety_note",
            Self::AlternateTarget => "alternate_target",
            Self::CompareBeforeSave => "compare_before_save",
        }
    }
}

/// Mutating lane that may participate in save or derived-output writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationMutatorClass {
    /// Formatter or other save-participant text normalizer.
    FormatOnSave,
    /// Import organizer or equivalent source action.
    OrganizeImports,
    /// Refactor or structured apply flow.
    RefactorApply,
    /// AI-assisted reviewed apply flow.
    AiApply,
    /// Generator or refresh path that materializes artifact bytes.
    GeneratedOutputWrite,
}

impl AutomationMutatorClass {
    /// Returns the stable token for this mutator.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FormatOnSave => "format_on_save",
            Self::OrganizeImports => "organize_imports",
            Self::RefactorApply => "refactor_apply",
            Self::AiApply => "ai_apply",
            Self::GeneratedOutputWrite => "generated_output_write",
        }
    }
}

/// Target-drift handling rule for a mutating lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MidFlightDriftPolicyClass {
    /// The mutator must rebase or abort before any durable write lands.
    RebaseOrAbort,
    /// The surface must route to compare/manual recovery instead of mutating.
    CompareThenManualRecovery,
    /// The mutator is not admitted on the row.
    NotApplicable,
}

impl MidFlightDriftPolicyClass {
    /// Returns the stable token for this drift policy.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RebaseOrAbort => "rebase_or_abort",
            Self::CompareThenManualRecovery => "compare_then_manual_recovery",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Required evidence category covered by a fixture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveTruthEvidenceClass {
    /// Lossy or failed decode posture remains visible before save.
    LossyDecodeRisk,
    /// Metadata preservation risks or guarantees are disclosed explicitly.
    MetadataPreservationDisclosure,
    /// Execute-bit or mode retention remains explicit.
    ExecuteBitRetention,
    /// Export or regeneration is disclosed as not being an exact file save.
    ExportOrRegenerateNotExactSave,
    /// Logical path and durable target divergence remains explicit.
    LogicalTargetAmbiguityDisclosure,
    /// A mutating lane proves the rebase-or-abort rule.
    MidFlightDriftRebaseRequired,
}

impl SaveTruthEvidenceClass {
    /// Returns the stable token for this evidence class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LossyDecodeRisk => "lossy_decode_risk",
            Self::MetadataPreservationDisclosure => "metadata_preservation_disclosure",
            Self::ExecuteBitRetention => "execute_bit_retention",
            Self::ExportOrRegenerateNotExactSave => "export_or_regenerate_not_exact_save",
            Self::LogicalTargetAmbiguityDisclosure => "logical_target_ambiguity_disclosure",
            Self::MidFlightDriftRebaseRequired => "mid_flight_drift_rebase_required",
        }
    }
}

/// One visible action row on a fallback disclosure sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackActionDisclosure {
    /// Stable action class.
    pub action_class: FallbackActionClass,
    /// Reviewer-facing label shown on the sheet.
    pub visible_label: String,
    /// Whether the action is enabled in the current disclosure posture.
    pub enabled: bool,
    /// Export-safe explanation for why the action exists.
    pub summary: String,
    /// Disabled reason when `enabled` is `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
}

/// One mutator safety rule carried by a row or fixture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutatorGuardRow {
    /// Stable mutator class.
    pub mutator_class: AutomationMutatorClass,
    /// Whether the mutator is admitted for this surface family.
    pub available: bool,
    /// Drift handling rule for the mutator.
    pub drift_policy_class: MidFlightDriftPolicyClass,
    /// Export-safe explanation of the mutator's guardrail.
    pub summary: String,
}

/// Checked-in source references quoted by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceContractRefs {
    /// Narrative doc reference.
    pub doc_ref: String,
    /// Boundary schema reference.
    pub schema_ref: String,
    /// Machine-readable packet reference.
    pub packet_ref: String,
    /// Reviewer-facing report reference.
    pub report_ref: String,
    /// Fixture directory reference.
    pub fixture_dir_ref: String,
    /// Fixture README reference.
    pub fixture_readme_ref: String,
}

/// One artifact-family save truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactSaveTruthRow {
    /// Stable row id.
    pub row_id: String,
    /// Surface family this row covers.
    pub surface_class: MatrixSurfaceClass,
    /// Reviewer-facing title.
    pub title: String,
    /// Supported root classes for the family.
    pub supported_root_classes: Vec<MatrixRootClass>,
    /// Authoritative identity class for save or review.
    pub path_identity_class: MatrixPathIdentityClass,
    /// Preferred save path when the family can preserve its strongest guarantee.
    pub preferred_save_path: MatrixSaveFallback,
    /// Alternate or degraded paths disclosed when the preferred path is unavailable.
    pub disclosed_alternate_paths: Vec<MatrixSaveFallback>,
    /// Metadata-sensitive cues that remain visible on the surface.
    pub metadata_sensitivity_indicators: Vec<MetadataSensitivityIndicator>,
    /// Fallback disclosure family used by the row.
    pub save_fallback_disclosure_class: SaveFallbackDisclosureClass,
    /// Visible actions on the fallback disclosure sheet.
    pub fallback_actions: Vec<FallbackActionDisclosure>,
    /// Mutator guardrails for the row.
    pub mutator_guards: Vec<MutatorGuardRow>,
    /// Real consumer modules that quote the row.
    pub consumer_refs: Vec<String>,
    /// Checked-in fixtures proving the row.
    pub fixture_refs: Vec<String>,
    /// Export-safe summary of the row's save truth.
    pub summary: String,
}

/// One checked-in scenario proving a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactSaveTruthFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Expected row id.
    pub expected_row_id: String,
    /// Reviewer-facing scenario label.
    pub scenario: String,
    /// Root class under test.
    pub root_class: MatrixRootClass,
    /// Observed save path in the scenario.
    pub observed_save_path: MatrixSaveFallback,
    /// Metadata-sensitive cues kept visible in the scenario.
    pub metadata_sensitivity_indicators: Vec<MetadataSensitivityIndicator>,
    /// Fallback disclosure class visible in the scenario.
    pub save_fallback_disclosure_class: SaveFallbackDisclosureClass,
    /// Fallback actions visible in the scenario.
    pub fallback_actions: Vec<FallbackActionDisclosure>,
    /// Active mutator for the scenario, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_mutator: Option<AutomationMutatorClass>,
    /// Drift policy enforced for the active mutator or surface.
    pub drift_policy_class: MidFlightDriftPolicyClass,
    /// Required evidence classes the scenario proves.
    pub evidence_classes: Vec<SaveTruthEvidenceClass>,
    /// Export-safe note about the scenario.
    pub notes: String,
}

/// Derived packet summary used by tests and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactSaveTruthSummary {
    /// Total row count.
    pub row_count: usize,
    /// Total fixture count.
    pub fixture_count: usize,
    /// Row count whose preferred path is `atomic_replace`.
    pub preferred_atomic_row_count: usize,
    /// Row count disclosed as regenerate/export rather than exact file save.
    pub regenerate_or_export_row_count: usize,
    /// Row count that stage drafts before publish or replay.
    pub draft_stage_row_count: usize,
    /// Row count that block direct save and route to compare/review only.
    pub compare_only_blocked_row_count: usize,
    /// Row count carrying logical-target ambiguity cues.
    pub logical_target_indicator_row_count: usize,
    /// Count of admitted mutator lanes that must rebase or abort.
    pub rebase_or_abort_mutator_count: usize,
}

/// Top-level packet freezing artifact save truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactSaveTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Shared references for docs, schema, packet, and fixtures.
    pub source_contract_refs: SourceContractRefs,
    /// Artifact-family rows.
    pub rows: Vec<ArtifactSaveTruthRow>,
    /// Derived summary.
    pub summary: ArtifactSaveTruthSummary,
    /// Export-safe invariant lines.
    pub invariants: Vec<String>,
}

impl ArtifactSaveTruthPacket {
    /// Returns support/export-safe summary lines.
    pub fn support_export_lines(&self) -> Vec<String> {
        vec![
            format!("packet_id: {}", self.packet_id),
            format!("row_count: {}", self.summary.row_count),
            format!("fixture_count: {}", self.summary.fixture_count),
            format!(
                "preferred_atomic_row_count: {}",
                self.summary.preferred_atomic_row_count
            ),
            format!(
                "regenerate_or_export_row_count: {}",
                self.summary.regenerate_or_export_row_count
            ),
            format!(
                "draft_stage_row_count: {}",
                self.summary.draft_stage_row_count
            ),
            format!(
                "compare_only_blocked_row_count: {}",
                self.summary.compare_only_blocked_row_count
            ),
            format!(
                "logical_target_indicator_row_count: {}",
                self.summary.logical_target_indicator_row_count
            ),
            format!(
                "rebase_or_abort_mutator_count: {}",
                self.summary.rebase_or_abort_mutator_count
            ),
        ]
    }
}

/// One validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactSaveTruthValidationViolation {
    /// Stable check id.
    pub check_id: &'static str,
    /// Human-readable failure message.
    pub message: String,
}

/// Validation report for packet or fixture checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactSaveTruthValidationReport {
    /// Detected violations.
    pub violations: Vec<ArtifactSaveTruthValidationViolation>,
}

impl ArtifactSaveTruthValidationReport {
    fn push(&mut self, check_id: &'static str, message: impl Into<String>) {
        self.violations.push(ArtifactSaveTruthValidationViolation {
            check_id,
            message: message.into(),
        });
    }

    fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for ArtifactSaveTruthValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "artifact save truth validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ArtifactSaveTruthValidationReport {}

/// Returns the checked-in artifact-save-truth packet.
pub fn seeded_artifact_save_truth_packet() -> ArtifactSaveTruthPacket {
    let rows = seeded_rows();
    let fixtures = seeded_artifact_save_truth_fixtures();
    ArtifactSaveTruthPacket {
        record_kind: ARTIFACT_SAVE_TRUTH_PACKET_RECORD_KIND.to_owned(),
        schema_version: ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION,
        packet_id: "state.artifact_save_truth.v1".to_owned(),
        title: "Artifact save truth for generated, exported, draft, and remote surfaces".to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: ARTIFACT_SAVE_TRUTH_DOC_REF.to_owned(),
            schema_ref: ARTIFACT_SAVE_TRUTH_SCHEMA_REF.to_owned(),
            packet_ref: ARTIFACT_SAVE_TRUTH_PACKET_REF.to_owned(),
            report_ref: ARTIFACT_SAVE_TRUTH_REPORT_REF.to_owned(),
            fixture_dir_ref: ARTIFACT_SAVE_TRUTH_FIXTURE_DIR.to_owned(),
            fixture_readme_ref: ARTIFACT_SAVE_TRUTH_FIXTURE_README_REF.to_owned(),
        },
        summary: summarize(&rows, &fixtures),
        invariants: vec![
            "encoding, newline, bom/final-newline, execute-bit, generated-state, and logical-target cues remain visible before mutation when they affect durable truth".to_owned(),
            "fallback sheets disclose continue, inspect-safety-note, alternate-target, and compare-before-save actions instead of collapsing degraded saves into success-only copy".to_owned(),
            "format-on-save, organize-imports, refactor apply, ai apply, and generated-output writes either rebase or abort on target drift or are marked not applicable explicitly".to_owned(),
            "regeneration, export, draft staging, and compare-only flows stay distinct from exact local-file saves and never impersonate them".to_owned(),
        ],
        rows,
    }
}

/// Returns the checked-in fixture scenarios proving the packet rows.
pub fn seeded_artifact_save_truth_fixtures() -> Vec<ArtifactSaveTruthFixture> {
    vec![
        ArtifactSaveTruthFixture {
            record_kind: ARTIFACT_SAVE_TRUTH_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION,
            fixture_id: "fixture.notebook_document".to_owned(),
            expected_row_id: "notebook_document".to_owned(),
            scenario: "A notebook document saved on a local root keeps encoding, newline, BOM/final-newline, and logical-target cues visible while format-on-save must rebase or abort on any external drift.".to_owned(),
            root_class: MatrixRootClass::LocalFilesystem,
            observed_save_path: MatrixSaveFallback::AtomicReplace,
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::Encoding,
                MetadataSensitivityIndicator::NewlineMode,
                MetadataSensitivityIndicator::BomOrFinalNewline,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::AtomicWithDisclosedFallback,
            fallback_actions: atomic_fallback_actions(true),
            active_mutator: Some(AutomationMutatorClass::FormatOnSave),
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            evidence_classes: vec![
                SaveTruthEvidenceClass::MetadataPreservationDisclosure,
                SaveTruthEvidenceClass::MidFlightDriftRebaseRequired,
            ],
            notes: "Notebook saves stay source-fidelity aware even when structured metadata is the material risk.".to_owned(),
        },
        ArtifactSaveTruthFixture {
            record_kind: ARTIFACT_SAVE_TRUTH_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION,
            fixture_id: "fixture.notebook_output_artifact".to_owned(),
            expected_row_id: "notebook_output_artifact".to_owned(),
            scenario: "A notebook output refresh routes through regeneration, not exact file save, and the generated-output writer must rebase or abort if the backing result changed.".to_owned(),
            root_class: MatrixRootClass::GeneratedManaged,
            observed_save_path: MatrixSaveFallback::RegenerateFromSource,
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::RegenerateOrExportDisclosed,
            fallback_actions: regenerate_or_export_actions(true),
            active_mutator: Some(AutomationMutatorClass::GeneratedOutputWrite),
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            evidence_classes: vec![
                SaveTruthEvidenceClass::ExportOrRegenerateNotExactSave,
                SaveTruthEvidenceClass::MidFlightDriftRebaseRequired,
            ],
            notes: "Rendered outputs remain subordinate to the notebook source and runtime lineage.".to_owned(),
        },
        ArtifactSaveTruthFixture {
            record_kind: ARTIFACT_SAVE_TRUTH_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION,
            fixture_id: "fixture.request_workspace_document".to_owned(),
            expected_row_id: "request_workspace_document".to_owned(),
            scenario: "A remote request-workspace script falls back from atomic replace to conditional remote write while execute-bit retention stays explicit and AI apply must rebase or abort.".to_owned(),
            root_class: MatrixRootClass::RemoteAgent,
            observed_save_path: MatrixSaveFallback::ConditionalRemoteWrite,
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::Encoding,
                MetadataSensitivityIndicator::NewlineMode,
                MetadataSensitivityIndicator::BomOrFinalNewline,
                MetadataSensitivityIndicator::ExecuteBitOrPermissions,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::AtomicWithDisclosedFallback,
            fallback_actions: atomic_fallback_actions(true),
            active_mutator: Some(AutomationMutatorClass::AiApply),
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            evidence_classes: vec![
                SaveTruthEvidenceClass::ExecuteBitRetention,
                SaveTruthEvidenceClass::MidFlightDriftRebaseRequired,
            ],
            notes: "Remote request files do not get to hide execute-bit or revision-precondition truth.".to_owned(),
        },
        ArtifactSaveTruthFixture {
            record_kind: ARTIFACT_SAVE_TRUTH_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION,
            fixture_id: "fixture.request_response_snapshot".to_owned(),
            expected_row_id: "request_response_snapshot".to_owned(),
            scenario: "A provider-backed response snapshot only exports by save-as copy after compare-before-save review; it never impersonates an exact editable file.".to_owned(),
            root_class: MatrixRootClass::VirtualProviderBacked,
            observed_save_path: MatrixSaveFallback::SaveAsCopy,
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::RegenerateOrExportDisclosed,
            fallback_actions: regenerate_or_export_actions(false),
            active_mutator: None,
            drift_policy_class: MidFlightDriftPolicyClass::CompareThenManualRecovery,
            evidence_classes: vec![SaveTruthEvidenceClass::ExportOrRegenerateNotExactSave],
            notes: "Provider snapshots stay inspect-first and export-second.".to_owned(),
        },
        ArtifactSaveTruthFixture {
            record_kind: ARTIFACT_SAVE_TRUTH_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION,
            fixture_id: "fixture.database_export_artifact".to_owned(),
            expected_row_id: "database_export_artifact".to_owned(),
            scenario: "A database export refresh is regenerate-from-source rather than in-place overwrite, and the export writer must rebase or abort if the query basis changed.".to_owned(),
            root_class: MatrixRootClass::GeneratedManaged,
            observed_save_path: MatrixSaveFallback::RegenerateFromSource,
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::RegenerateOrExportDisclosed,
            fallback_actions: regenerate_or_export_actions(true),
            active_mutator: Some(AutomationMutatorClass::GeneratedOutputWrite),
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            evidence_classes: vec![
                SaveTruthEvidenceClass::ExportOrRegenerateNotExactSave,
                SaveTruthEvidenceClass::MidFlightDriftRebaseRequired,
            ],
            notes: "Exports remain derived artifacts with explicit provenance and replay posture.".to_owned(),
        },
        ArtifactSaveTruthFixture {
            record_kind: ARTIFACT_SAVE_TRUTH_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION,
            fixture_id: "fixture.profiler_trace_artifact".to_owned(),
            expected_row_id: "profiler_trace_artifact".to_owned(),
            scenario: "An imported profiler trace surfaces lossy-decode risk before any text conversion and only leaves the archive as an exported copy.".to_owned(),
            root_class: MatrixRootClass::ArchivePackaged,
            observed_save_path: MatrixSaveFallback::SaveAsCopy,
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::Encoding,
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::RegenerateOrExportDisclosed,
            fallback_actions: regenerate_or_export_actions(false),
            active_mutator: None,
            drift_policy_class: MidFlightDriftPolicyClass::CompareThenManualRecovery,
            evidence_classes: vec![
                SaveTruthEvidenceClass::LossyDecodeRisk,
                SaveTruthEvidenceClass::ExportOrRegenerateNotExactSave,
            ],
            notes: "Trace packets remain attributable imports rather than editable source buffers.".to_owned(),
        },
        ArtifactSaveTruthFixture {
            record_kind: ARTIFACT_SAVE_TRUTH_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION,
            fixture_id: "fixture.preview_output_artifact".to_owned(),
            expected_row_id: "preview_output_artifact".to_owned(),
            scenario: "A container-backed preview output refresh uses regeneration, keeps generated-state cues visible, and aborts instead of stomping newer output lineage.".to_owned(),
            root_class: MatrixRootClass::ContainerMount,
            observed_save_path: MatrixSaveFallback::RegenerateFromSource,
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::RegenerateOrExportDisclosed,
            fallback_actions: regenerate_or_export_actions(true),
            active_mutator: Some(AutomationMutatorClass::GeneratedOutputWrite),
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            evidence_classes: vec![
                SaveTruthEvidenceClass::ExportOrRegenerateNotExactSave,
                SaveTruthEvidenceClass::MidFlightDriftRebaseRequired,
            ],
            notes: "Preview bytes remain generator-owned even when cached on disk.".to_owned(),
        },
        ArtifactSaveTruthFixture {
            record_kind: ARTIFACT_SAVE_TRUTH_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION,
            fixture_id: "fixture.sync_packet_artifact".to_owned(),
            expected_row_id: "sync_packet_artifact".to_owned(),
            scenario: "An offline sync packet stages a local draft, keeps compare-before-save visible before replay, and requires generated-output refresh to rebase or abort on reconnect drift.".to_owned(),
            root_class: MatrixRootClass::ManagedOfflineBundle,
            observed_save_path: MatrixSaveFallback::StageLocalDraft,
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::Encoding,
                MetadataSensitivityIndicator::NewlineMode,
                MetadataSensitivityIndicator::BomOrFinalNewline,
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::DraftStageDisclosed,
            fallback_actions: draft_stage_actions(true),
            active_mutator: Some(AutomationMutatorClass::GeneratedOutputWrite),
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            evidence_classes: vec![
                SaveTruthEvidenceClass::MetadataPreservationDisclosure,
                SaveTruthEvidenceClass::MidFlightDriftRebaseRequired,
            ],
            notes: "Drafted packets preserve local continuity but never replay invisibly.".to_owned(),
        },
        ArtifactSaveTruthFixture {
            record_kind: ARTIFACT_SAVE_TRUTH_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION,
            fixture_id: "fixture.provider_local_draft".to_owned(),
            expected_row_id: "provider_local_draft".to_owned(),
            scenario: "A provider-linked local draft keeps logical-target ambiguity visible, stages locally first, and makes AI apply rebase or abort before any publish-later replay.".to_owned(),
            root_class: MatrixRootClass::VirtualProviderBacked,
            observed_save_path: MatrixSaveFallback::StageLocalDraft,
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::Encoding,
                MetadataSensitivityIndicator::NewlineMode,
                MetadataSensitivityIndicator::BomOrFinalNewline,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::DraftStageDisclosed,
            fallback_actions: draft_stage_actions(true),
            active_mutator: Some(AutomationMutatorClass::AiApply),
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            evidence_classes: vec![
                SaveTruthEvidenceClass::LogicalTargetAmbiguityDisclosure,
                SaveTruthEvidenceClass::MidFlightDriftRebaseRequired,
            ],
            notes: "Local drafts remain distinct from provider-owned publish truth.".to_owned(),
        },
        ArtifactSaveTruthFixture {
            record_kind: ARTIFACT_SAVE_TRUTH_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION,
            fixture_id: "fixture.infrastructure_overlay_document".to_owned(),
            expected_row_id: "infrastructure_overlay_document".to_owned(),
            scenario: "A provider-backed infrastructure overlay blocks ordinary save, keeps logical-target ambiguity visible, and routes the user through compare-first review or alternate-target export only.".to_owned(),
            root_class: MatrixRootClass::VirtualProviderBacked,
            observed_save_path: MatrixSaveFallback::CompareOnlyBlocked,
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
                MetadataSensitivityIndicator::ExecuteBitOrPermissions,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::CompareOnlyBlocked,
            fallback_actions: compare_only_actions(),
            active_mutator: None,
            drift_policy_class: MidFlightDriftPolicyClass::CompareThenManualRecovery,
            evidence_classes: vec![SaveTruthEvidenceClass::LogicalTargetAmbiguityDisclosure],
            notes: "Overlay documents remain provider truth layers, not local-source authority.".to_owned(),
        },
        ArtifactSaveTruthFixture {
            record_kind: ARTIFACT_SAVE_TRUTH_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION,
            fixture_id: "fixture.imported_archive_capture".to_owned(),
            expected_row_id: "imported_archive_capture".to_owned(),
            scenario: "An imported archive capture exposes lossy-decode risk and only allows save-as copy after compare-before-save review; it never claims exact source-file semantics.".to_owned(),
            root_class: MatrixRootClass::ArchivePackaged,
            observed_save_path: MatrixSaveFallback::SaveAsCopy,
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::Encoding,
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::RegenerateOrExportDisclosed,
            fallback_actions: regenerate_or_export_actions(false),
            active_mutator: None,
            drift_policy_class: MidFlightDriftPolicyClass::CompareThenManualRecovery,
            evidence_classes: vec![
                SaveTruthEvidenceClass::LossyDecodeRisk,
                SaveTruthEvidenceClass::ExportOrRegenerateNotExactSave,
            ],
            notes: "Imported captures remain inspect-only until copied into a new admitted target.".to_owned(),
        },
    ]
}

/// Validates the checked-in packet.
pub fn validate_artifact_save_truth_packet(
    packet: &ArtifactSaveTruthPacket,
) -> Result<(), ArtifactSaveTruthValidationReport> {
    let mut report = ArtifactSaveTruthValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != ARTIFACT_SAVE_TRUTH_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            format!(
                "record_kind must be {ARTIFACT_SAVE_TRUTH_PACKET_RECORD_KIND}, found {}",
                packet.record_kind
            ),
        );
    }
    if packet.schema_version != ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION {
        report.push(
            "packet.schema_version",
            format!(
                "schema_version must be {}, found {}",
                ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION, packet.schema_version
            ),
        );
    }

    let expected_surfaces: BTreeSet<_> = all_surface_classes().into_iter().collect();
    let actual_surfaces: BTreeSet<_> = packet.rows.iter().map(|row| row.surface_class).collect();
    if expected_surfaces != actual_surfaces {
        report.push(
            "packet.surface_coverage",
            "packet must cover every admitted artifact surface exactly once",
        );
    }

    let mut row_ids = BTreeSet::new();
    for row in &packet.rows {
        if !row_ids.insert(row.row_id.as_str()) {
            report.push(
                "row.duplicate_row_id",
                format!("duplicate row id {}", row.row_id),
            );
        }
        if row.supported_root_classes.is_empty() {
            report.push(
                "row.supported_root_classes",
                format!("row {} must list at least one root class", row.row_id),
            );
        }
        if row.consumer_refs.is_empty() {
            report.push(
                "row.consumer_refs",
                format!("row {} must cite at least one consumer ref", row.row_id),
            );
        }
        if row.fixture_refs.is_empty() {
            report.push(
                "row.fixture_refs",
                format!("row {} must cite at least one fixture ref", row.row_id),
            );
        }
        if row.metadata_sensitivity_indicators.is_empty() {
            report.push(
                "row.metadata_indicators",
                format!(
                    "row {} must expose at least one metadata-sensitive cue",
                    row.row_id
                ),
            );
        }
        validate_fallback_actions(&row.row_id, &row.fallback_actions, &mut report);
        validate_mutator_guards(&row.row_id, &row.mutator_guards, &mut report);
        validate_row_disclosure(row, &mut report);
    }

    let derived = summarize(&packet.rows, &seeded_artifact_save_truth_fixtures());
    if packet.summary != derived {
        report.push(
            "packet.summary",
            "packet summary must match the derived seeded summary",
        );
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

/// Validates one checked-in fixture against the packet.
pub fn validate_artifact_save_truth_fixture(
    packet: &ArtifactSaveTruthPacket,
    fixture: &ArtifactSaveTruthFixture,
) -> Result<(), ArtifactSaveTruthValidationReport> {
    let mut report = ArtifactSaveTruthValidationReport {
        violations: Vec::new(),
    };
    if fixture.record_kind != ARTIFACT_SAVE_TRUTH_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            format!(
                "fixture {} record_kind must be {}",
                fixture.fixture_id, ARTIFACT_SAVE_TRUTH_FIXTURE_RECORD_KIND
            ),
        );
    }
    if fixture.schema_version != ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION {
        report.push(
            "fixture.schema_version",
            format!(
                "fixture {} schema_version must be {}",
                fixture.fixture_id, ARTIFACT_SAVE_TRUTH_SCHEMA_VERSION
            ),
        );
    }

    let Some(row) = packet
        .rows
        .iter()
        .find(|row| row.row_id == fixture.expected_row_id)
    else {
        report.push(
            "fixture.row_exists",
            format!(
                "fixture {} must bind to an existing row {}",
                fixture.fixture_id, fixture.expected_row_id
            ),
        );
        return Err(report);
    };

    if !row.supported_root_classes.contains(&fixture.root_class) {
        report.push(
            "fixture.root_class",
            format!(
                "fixture {} root class {} is not admitted by row {}",
                fixture.fixture_id,
                fixture.root_class.as_str(),
                row.row_id
            ),
        );
    }
    if fixture.observed_save_path != row.preferred_save_path
        && !row
            .disclosed_alternate_paths
            .contains(&fixture.observed_save_path)
    {
        report.push(
            "fixture.observed_save_path",
            format!(
                "fixture {} observed save path {} is not admitted by row {}",
                fixture.fixture_id,
                fixture.observed_save_path.as_str(),
                row.row_id
            ),
        );
    }
    if fixture.save_fallback_disclosure_class != row.save_fallback_disclosure_class {
        report.push(
            "fixture.disclosure_class",
            format!(
                "fixture {} disclosure class {} must match row {} ({})",
                fixture.fixture_id,
                fixture.save_fallback_disclosure_class.as_str(),
                row.row_id,
                row.save_fallback_disclosure_class.as_str()
            ),
        );
    }

    let row_indicators: BTreeSet<_> = row
        .metadata_sensitivity_indicators
        .iter()
        .copied()
        .collect();
    let fixture_indicators: BTreeSet<_> = fixture
        .metadata_sensitivity_indicators
        .iter()
        .copied()
        .collect();
    if fixture_indicators != row_indicators {
        report.push(
            "fixture.metadata_indicators",
            format!(
                "fixture {} indicators must match row {} exactly",
                fixture.fixture_id, row.row_id
            ),
        );
    }

    let row_actions: BTreeSet<_> = row
        .fallback_actions
        .iter()
        .map(|action| action.action_class)
        .collect();
    let fixture_actions: BTreeSet<_> = fixture
        .fallback_actions
        .iter()
        .map(|action| action.action_class)
        .collect();
    if row_actions != fixture_actions {
        report.push(
            "fixture.fallback_actions",
            format!(
                "fixture {} must expose the same fallback action set as row {}",
                fixture.fixture_id, row.row_id
            ),
        );
    }

    if let Some(mutator) = fixture.active_mutator {
        let Some(guard) = row
            .mutator_guards
            .iter()
            .find(|guard| guard.mutator_class == mutator)
        else {
            report.push(
                "fixture.active_mutator_exists",
                format!(
                    "fixture {} active mutator {} must exist on row {}",
                    fixture.fixture_id,
                    mutator.as_str(),
                    row.row_id
                ),
            );
            return Err(report);
        };
        if !guard.available {
            report.push(
                "fixture.active_mutator_available",
                format!(
                    "fixture {} active mutator {} is not admitted by row {}",
                    fixture.fixture_id,
                    mutator.as_str(),
                    row.row_id
                ),
            );
        }
        if guard.drift_policy_class != fixture.drift_policy_class {
            report.push(
                "fixture.drift_policy",
                format!(
                    "fixture {} drift policy {} must match row guard {} for {}",
                    fixture.fixture_id,
                    fixture.drift_policy_class.as_str(),
                    guard.drift_policy_class.as_str(),
                    mutator.as_str()
                ),
            );
        }
    } else if fixture.drift_policy_class == MidFlightDriftPolicyClass::RebaseOrAbort {
        report.push(
            "fixture.drift_policy_without_mutator",
            format!(
                "fixture {} cannot claim rebase_or_abort without an active mutator",
                fixture.fixture_id
            ),
        );
    }

    if fixture.evidence_classes.is_empty() {
        report.push(
            "fixture.evidence_classes",
            format!(
                "fixture {} must cite at least one evidence class",
                fixture.fixture_id
            ),
        );
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

/// Parses the checked-in packet JSON.
pub fn current_artifact_save_truth_packet() -> Result<ArtifactSaveTruthPacket, serde_json::Error> {
    serde_json::from_str(ARTIFACT_SAVE_TRUTH_PACKET_JSON)
}

/// Renders a reviewer-facing Markdown report for the packet and fixtures.
pub fn artifact_save_truth_report_markdown(
    packet: &ArtifactSaveTruthPacket,
    fixtures: &[ArtifactSaveTruthFixture],
) -> String {
    let mut out = String::new();
    out.push_str("# Artifact save truth\n\n");
    out.push_str(
        "This report freezes metadata-sensitive save cues, fallback disclosure, and no-silent-stomp guards for generated, exported, draft, and remote artifact families.\n\n",
    );
    out.push_str("## Summary\n\n");
    for line in packet.support_export_lines() {
        out.push_str("- ");
        out.push_str(&line);
        out.push('\n');
    }
    out.push_str("\n## Rows\n\n");
    out.push_str(
        "| Row | Preferred path | Alternates | Indicators | Disclosure | Rebase/abort mutators |\n",
    );
    out.push_str("| --- | --- | --- | --- | --- | --- |\n");
    for row in &packet.rows {
        let alternates = row
            .disclosed_alternate_paths
            .iter()
            .map(|path| path.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let indicators = row
            .metadata_sensitivity_indicators
            .iter()
            .map(|indicator| indicator.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let mutators = row
            .mutator_guards
            .iter()
            .filter(|guard| {
                guard.available
                    && guard.drift_policy_class == MidFlightDriftPolicyClass::RebaseOrAbort
            })
            .map(|guard| guard.mutator_class.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            row.row_id,
            row.preferred_save_path.as_str(),
            alternates,
            indicators,
            row.save_fallback_disclosure_class.as_str(),
            mutators
        ));
    }
    out.push_str("\n## Fixture coverage\n\n");
    for fixture in fixtures {
        let evidence = fixture
            .evidence_classes
            .iter()
            .map(|class| class.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            "- `{}`: {} Evidence: `{}`.\n",
            fixture.expected_row_id, fixture.scenario, evidence
        ));
    }
    out
}

fn all_surface_classes() -> [MatrixSurfaceClass; 11] {
    [
        MatrixSurfaceClass::NotebookDocument,
        MatrixSurfaceClass::NotebookOutputArtifact,
        MatrixSurfaceClass::RequestWorkspaceDocument,
        MatrixSurfaceClass::RequestResponseSnapshot,
        MatrixSurfaceClass::DatabaseExportArtifact,
        MatrixSurfaceClass::ProfilerTraceArtifact,
        MatrixSurfaceClass::PreviewOutputArtifact,
        MatrixSurfaceClass::SyncPacketArtifact,
        MatrixSurfaceClass::ProviderLocalDraft,
        MatrixSurfaceClass::InfrastructureOverlayDocument,
        MatrixSurfaceClass::ImportedArchiveCapture,
    ]
}

fn validate_row_disclosure(
    row: &ArtifactSaveTruthRow,
    report: &mut ArtifactSaveTruthValidationReport,
) {
    match row.save_fallback_disclosure_class {
        SaveFallbackDisclosureClass::AtomicWithDisclosedFallback => {
            if row.preferred_save_path != MatrixSaveFallback::AtomicReplace {
                report.push(
                    "row.atomic_disclosure_preferred_path",
                    format!(
                        "row {} must prefer atomic_replace under atomic_with_disclosed_fallback",
                        row.row_id
                    ),
                );
            }
        }
        SaveFallbackDisclosureClass::RegenerateOrExportDisclosed => {
            if !matches!(
                row.preferred_save_path,
                MatrixSaveFallback::RegenerateFromSource | MatrixSaveFallback::SaveAsCopy
            ) {
                report.push(
                    "row.regenerate_export_preferred_path",
                    format!(
                        "row {} must prefer regenerate_from_source or save_as_copy under regenerate_or_export_disclosed",
                        row.row_id
                    ),
                );
            }
        }
        SaveFallbackDisclosureClass::DraftStageDisclosed => {
            if row.preferred_save_path != MatrixSaveFallback::StageLocalDraft {
                report.push(
                    "row.draft_stage_preferred_path",
                    format!(
                        "row {} must prefer stage_local_draft under draft_stage_disclosed",
                        row.row_id
                    ),
                );
            }
        }
        SaveFallbackDisclosureClass::CompareOnlyBlocked => {
            if row.preferred_save_path != MatrixSaveFallback::CompareOnlyBlocked {
                report.push(
                    "row.compare_only_preferred_path",
                    format!(
                        "row {} must prefer compare_only_blocked under compare_only_blocked disclosure",
                        row.row_id
                    ),
                );
            }
        }
    }
}

fn validate_fallback_actions(
    row_id: &str,
    actions: &[FallbackActionDisclosure],
    report: &mut ArtifactSaveTruthValidationReport,
) {
    let expected: BTreeSet<_> = [
        FallbackActionClass::Continue,
        FallbackActionClass::InspectSafetyNote,
        FallbackActionClass::AlternateTarget,
        FallbackActionClass::CompareBeforeSave,
    ]
    .into_iter()
    .collect();
    let actual: BTreeSet<_> = actions.iter().map(|action| action.action_class).collect();
    if actual != expected {
        report.push(
            "row.fallback_actions_coverage",
            format!(
                "row {} must expose continue, inspect_safety_note, alternate_target, and compare_before_save",
                row_id
            ),
        );
    }
    for action in actions {
        if !action.enabled && action.disabled_reason.is_none() {
            report.push(
                "row.fallback_action_disabled_reason",
                format!(
                    "row {} action {} is disabled without a reason",
                    row_id,
                    action.action_class.as_str()
                ),
            );
        }
    }
}

fn validate_mutator_guards(
    row_id: &str,
    guards: &[MutatorGuardRow],
    report: &mut ArtifactSaveTruthValidationReport,
) {
    let expected: BTreeSet<_> = [
        AutomationMutatorClass::FormatOnSave,
        AutomationMutatorClass::OrganizeImports,
        AutomationMutatorClass::RefactorApply,
        AutomationMutatorClass::AiApply,
        AutomationMutatorClass::GeneratedOutputWrite,
    ]
    .into_iter()
    .collect();
    let actual: BTreeSet<_> = guards.iter().map(|guard| guard.mutator_class).collect();
    if actual != expected {
        report.push(
            "row.mutator_guard_coverage",
            format!(
                "row {} must declare guard posture for every mutator class",
                row_id
            ),
        );
    }
    for guard in guards {
        if guard.available && guard.drift_policy_class == MidFlightDriftPolicyClass::NotApplicable {
            report.push(
                "row.available_mutator_policy",
                format!(
                    "row {} mutator {} is available but marked not_applicable",
                    row_id,
                    guard.mutator_class.as_str()
                ),
            );
        }
        if !guard.available && guard.drift_policy_class == MidFlightDriftPolicyClass::RebaseOrAbort
        {
            report.push(
                "row.unavailable_mutator_policy",
                format!(
                    "row {} mutator {} cannot require rebase_or_abort when unavailable",
                    row_id,
                    guard.mutator_class.as_str()
                ),
            );
        }
    }
}

fn summarize(
    rows: &[ArtifactSaveTruthRow],
    fixtures: &[ArtifactSaveTruthFixture],
) -> ArtifactSaveTruthSummary {
    ArtifactSaveTruthSummary {
        row_count: rows.len(),
        fixture_count: fixtures.len(),
        preferred_atomic_row_count: rows
            .iter()
            .filter(|row| row.preferred_save_path == MatrixSaveFallback::AtomicReplace)
            .count(),
        regenerate_or_export_row_count: rows
            .iter()
            .filter(|row| {
                row.save_fallback_disclosure_class
                    == SaveFallbackDisclosureClass::RegenerateOrExportDisclosed
            })
            .count(),
        draft_stage_row_count: rows
            .iter()
            .filter(|row| {
                row.save_fallback_disclosure_class
                    == SaveFallbackDisclosureClass::DraftStageDisclosed
            })
            .count(),
        compare_only_blocked_row_count: rows
            .iter()
            .filter(|row| {
                row.save_fallback_disclosure_class
                    == SaveFallbackDisclosureClass::CompareOnlyBlocked
            })
            .count(),
        logical_target_indicator_row_count: rows
            .iter()
            .filter(|row| {
                row.metadata_sensitivity_indicators
                    .contains(&MetadataSensitivityIndicator::LogicalTargetAmbiguity)
            })
            .count(),
        rebase_or_abort_mutator_count: rows
            .iter()
            .flat_map(|row| row.mutator_guards.iter())
            .filter(|guard| {
                guard.available
                    && guard.drift_policy_class == MidFlightDriftPolicyClass::RebaseOrAbort
            })
            .count(),
    }
}

fn seeded_rows() -> Vec<ArtifactSaveTruthRow> {
    vec![
        ArtifactSaveTruthRow {
            row_id: "notebook_document".to_owned(),
            surface_class: MatrixSurfaceClass::NotebookDocument,
            title: "Notebook document".to_owned(),
            supported_root_classes: vec![
                MatrixRootClass::LocalFilesystem,
                MatrixRootClass::RemoteAgent,
                MatrixRootClass::ContainerMount,
            ],
            path_identity_class: MatrixPathIdentityClass::CanonicalFilesystemObject,
            preferred_save_path: MatrixSaveFallback::AtomicReplace,
            disclosed_alternate_paths: vec![
                MatrixSaveFallback::ConditionalRemoteWrite,
                MatrixSaveFallback::InPlaceWrite,
            ],
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::Encoding,
                MetadataSensitivityIndicator::NewlineMode,
                MetadataSensitivityIndicator::BomOrFinalNewline,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::AtomicWithDisclosedFallback,
            fallback_actions: atomic_fallback_actions(true),
            mutator_guards: code_file_mutator_guards(false),
            consumer_refs: vec![
                "crates/aureline-notebook/src/implement_notebook_save_repair_and_round_trip_safety_for_metadata_attachments_and_unknown_namespaces/mod.rs".to_owned(),
                "crates/aureline-shell/src/notebook_alpha/mod.rs".to_owned(),
            ],
            fixture_refs: vec![
                "fixtures/state/artifact_save_truth/notebook_document.json".to_owned(),
            ],
            summary: "Notebook documents preserve source-fidelity cues and disclose any downgrade from atomic replace before commit.".to_owned(),
        },
        ArtifactSaveTruthRow {
            row_id: "notebook_output_artifact".to_owned(),
            surface_class: MatrixSurfaceClass::NotebookOutputArtifact,
            title: "Notebook output artifact".to_owned(),
            supported_root_classes: vec![
                MatrixRootClass::GeneratedManaged,
                MatrixRootClass::RemoteAgent,
                MatrixRootClass::ContainerMount,
            ],
            path_identity_class: MatrixPathIdentityClass::GeneratedSourceIdentity,
            preferred_save_path: MatrixSaveFallback::RegenerateFromSource,
            disclosed_alternate_paths: vec![MatrixSaveFallback::SaveAsCopy],
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::RegenerateOrExportDisclosed,
            fallback_actions: regenerate_or_export_actions(true),
            mutator_guards: generated_only_mutator_guards(),
            consumer_refs: vec![
                "crates/aureline-notebook/src/integrate_notebook_outputs_with_docs_browser_ai_context_and_retrieval_debug_provenance_export/mod.rs".to_owned(),
                "crates/aureline-shell/src/preview_truth/mod.rs".to_owned(),
            ],
            fixture_refs: vec![
                "fixtures/state/artifact_save_truth/notebook_output_artifact.json".to_owned(),
            ],
            summary: "Notebook outputs disclose regeneration and export as distinct from exact document save.".to_owned(),
        },
        ArtifactSaveTruthRow {
            row_id: "request_workspace_document".to_owned(),
            surface_class: MatrixSurfaceClass::RequestWorkspaceDocument,
            title: "Request workspace document".to_owned(),
            supported_root_classes: vec![
                MatrixRootClass::LocalFilesystem,
                MatrixRootClass::RemoteAgent,
                MatrixRootClass::ContainerMount,
            ],
            path_identity_class: MatrixPathIdentityClass::CanonicalFilesystemObject,
            preferred_save_path: MatrixSaveFallback::AtomicReplace,
            disclosed_alternate_paths: vec![
                MatrixSaveFallback::ConditionalRemoteWrite,
                MatrixSaveFallback::InPlaceWrite,
            ],
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::Encoding,
                MetadataSensitivityIndicator::NewlineMode,
                MetadataSensitivityIndicator::BomOrFinalNewline,
                MetadataSensitivityIndicator::ExecuteBitOrPermissions,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::AtomicWithDisclosedFallback,
            fallback_actions: atomic_fallback_actions(true),
            mutator_guards: code_file_mutator_guards(false),
            consumer_refs: vec![
                "crates/aureline-shell/src/request_workspace/mod.rs".to_owned(),
                "crates/aureline-runtime/src/quality/mod.rs".to_owned(),
            ],
            fixture_refs: vec![
                "fixtures/state/artifact_save_truth/request_workspace_document.json".to_owned(),
            ],
            summary: "Request-workspace documents keep execute-bit and compare-before-write truth visible across local and remote save paths.".to_owned(),
        },
        ArtifactSaveTruthRow {
            row_id: "request_response_snapshot".to_owned(),
            surface_class: MatrixSurfaceClass::RequestResponseSnapshot,
            title: "Request response snapshot".to_owned(),
            supported_root_classes: vec![
                MatrixRootClass::VirtualProviderBacked,
                MatrixRootClass::ArchivePackaged,
            ],
            path_identity_class: MatrixPathIdentityClass::ProviderObjectIdentity,
            preferred_save_path: MatrixSaveFallback::SaveAsCopy,
            disclosed_alternate_paths: vec![],
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::RegenerateOrExportDisclosed,
            fallback_actions: regenerate_or_export_actions(false),
            mutator_guards: compare_only_mutator_guards(),
            consumer_refs: vec![
                "crates/aureline-shell/src/request_workspace/mod.rs".to_owned(),
            ],
            fixture_refs: vec![
                "fixtures/state/artifact_save_truth/request_response_snapshot.json".to_owned(),
            ],
            summary: "Provider response snapshots stay inspect-first and export-only rather than pretending direct file-save authority.".to_owned(),
        },
        ArtifactSaveTruthRow {
            row_id: "database_export_artifact".to_owned(),
            surface_class: MatrixSurfaceClass::DatabaseExportArtifact,
            title: "Database export artifact".to_owned(),
            supported_root_classes: vec![
                MatrixRootClass::GeneratedManaged,
                MatrixRootClass::LocalFilesystem,
                MatrixRootClass::RemoteAgent,
            ],
            path_identity_class: MatrixPathIdentityClass::GeneratedSourceIdentity,
            preferred_save_path: MatrixSaveFallback::RegenerateFromSource,
            disclosed_alternate_paths: vec![MatrixSaveFallback::SaveAsCopy],
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::RegenerateOrExportDisclosed,
            fallback_actions: regenerate_or_export_actions(true),
            mutator_guards: generated_only_mutator_guards(),
            consumer_refs: vec![
                "crates/aureline-data/src/database_qualification.rs".to_owned(),
            ],
            fixture_refs: vec![
                "fixtures/state/artifact_save_truth/database_export_artifact.json".to_owned(),
            ],
            summary: "Database exports disclose regeneration and alternate-target copy instead of silently overwriting derived bytes.".to_owned(),
        },
        ArtifactSaveTruthRow {
            row_id: "profiler_trace_artifact".to_owned(),
            surface_class: MatrixSurfaceClass::ProfilerTraceArtifact,
            title: "Profiler trace artifact".to_owned(),
            supported_root_classes: vec![
                MatrixRootClass::ArchivePackaged,
                MatrixRootClass::LocalFilesystem,
                MatrixRootClass::ManagedOfflineBundle,
            ],
            path_identity_class: MatrixPathIdentityClass::ImportedSnapshotIdentity,
            preferred_save_path: MatrixSaveFallback::SaveAsCopy,
            disclosed_alternate_paths: vec![],
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::Encoding,
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::RegenerateOrExportDisclosed,
            fallback_actions: regenerate_or_export_actions(false),
            mutator_guards: compare_only_mutator_guards(),
            consumer_refs: vec![
                "crates/aureline-profiler/src/integrate_profile_and_trace_artifacts_into_incident_workspaces_ai_explanations_and_support_bundles/mod.rs".to_owned(),
            ],
            fixture_refs: vec![
                "fixtures/state/artifact_save_truth/profiler_trace_artifact.json".to_owned(),
            ],
            summary: "Profiler traces remain attributable imported packets and surface decode/export risk before any conversion.".to_owned(),
        },
        ArtifactSaveTruthRow {
            row_id: "preview_output_artifact".to_owned(),
            surface_class: MatrixSurfaceClass::PreviewOutputArtifact,
            title: "Preview output artifact".to_owned(),
            supported_root_classes: vec![
                MatrixRootClass::GeneratedManaged,
                MatrixRootClass::ContainerMount,
                MatrixRootClass::VirtualProviderBacked,
            ],
            path_identity_class: MatrixPathIdentityClass::GeneratedSourceIdentity,
            preferred_save_path: MatrixSaveFallback::RegenerateFromSource,
            disclosed_alternate_paths: vec![MatrixSaveFallback::SaveAsCopy],
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::RegenerateOrExportDisclosed,
            fallback_actions: regenerate_or_export_actions(true),
            mutator_guards: generated_only_mutator_guards(),
            consumer_refs: vec![
                "crates/aureline-preview/src/preview_origin/mutation_plan.rs".to_owned(),
                "crates/aureline-shell/src/preview_truth/mod.rs".to_owned(),
            ],
            fixture_refs: vec![
                "fixtures/state/artifact_save_truth/preview_output_artifact.json".to_owned(),
            ],
            summary: "Preview outputs remain source-first generated artifacts with explicit regenerate or copy-out recovery paths.".to_owned(),
        },
        ArtifactSaveTruthRow {
            row_id: "sync_packet_artifact".to_owned(),
            surface_class: MatrixSurfaceClass::SyncPacketArtifact,
            title: "Sync packet artifact".to_owned(),
            supported_root_classes: vec![
                MatrixRootClass::ManagedOfflineBundle,
                MatrixRootClass::LocalFilesystem,
            ],
            path_identity_class: MatrixPathIdentityClass::OfflineBundleIdentity,
            preferred_save_path: MatrixSaveFallback::StageLocalDraft,
            disclosed_alternate_paths: vec![MatrixSaveFallback::SaveAsCopy],
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::Encoding,
                MetadataSensitivityIndicator::NewlineMode,
                MetadataSensitivityIndicator::BomOrFinalNewline,
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::DraftStageDisclosed,
            fallback_actions: draft_stage_actions(true),
            mutator_guards: draft_packet_mutator_guards(),
            consumer_refs: vec![
                "crates/aureline-continuity/src/connectivity_state_and_deferred_intent/mod.rs".to_owned(),
                "crates/aureline-settings/src/inspector/conflict.rs".to_owned(),
            ],
            fixture_refs: vec![
                "fixtures/state/artifact_save_truth/sync_packet_artifact.json".to_owned(),
            ],
            summary: "Sync packets preserve local draft continuity and require compare-before-save plus replay revalidation instead of silent publish.".to_owned(),
        },
        ArtifactSaveTruthRow {
            row_id: "provider_local_draft".to_owned(),
            surface_class: MatrixSurfaceClass::ProviderLocalDraft,
            title: "Provider local draft".to_owned(),
            supported_root_classes: vec![
                MatrixRootClass::VirtualProviderBacked,
                MatrixRootClass::ManagedOfflineBundle,
            ],
            path_identity_class: MatrixPathIdentityClass::LocalDraftIdentity,
            preferred_save_path: MatrixSaveFallback::StageLocalDraft,
            disclosed_alternate_paths: vec![MatrixSaveFallback::SaveAsCopy],
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::Encoding,
                MetadataSensitivityIndicator::NewlineMode,
                MetadataSensitivityIndicator::BomOrFinalNewline,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::DraftStageDisclosed,
            fallback_actions: draft_stage_actions(true),
            mutator_guards: provider_draft_mutator_guards(),
            consumer_refs: vec![
                "crates/aureline-provider/src/publish_later/mod.rs".to_owned(),
                "crates/aureline-provider/src/work_item_sync/mod.rs".to_owned(),
            ],
            fixture_refs: vec![
                "fixtures/state/artifact_save_truth/provider_local_draft.json".to_owned(),
            ],
            summary: "Provider drafts stay local-first and keep logical-target ambiguity visible before any publish-later replay.".to_owned(),
        },
        ArtifactSaveTruthRow {
            row_id: "infrastructure_overlay_document".to_owned(),
            surface_class: MatrixSurfaceClass::InfrastructureOverlayDocument,
            title: "Infrastructure overlay document".to_owned(),
            supported_root_classes: vec![
                MatrixRootClass::VirtualProviderBacked,
                MatrixRootClass::RemoteAgent,
                MatrixRootClass::ContainerMount,
            ],
            path_identity_class: MatrixPathIdentityClass::ProviderObjectIdentity,
            preferred_save_path: MatrixSaveFallback::CompareOnlyBlocked,
            disclosed_alternate_paths: vec![MatrixSaveFallback::SaveAsCopy],
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
                MetadataSensitivityIndicator::ExecuteBitOrPermissions,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::CompareOnlyBlocked,
            fallback_actions: compare_only_actions(),
            mutator_guards: compare_only_mutator_guards(),
            consumer_refs: vec![
                "crates/aureline-infra/src/provider_overlay_and_vendor_console_handoff_continuity/mod.rs".to_owned(),
            ],
            fixture_refs: vec![
                "fixtures/state/artifact_save_truth/infrastructure_overlay_document.json".to_owned(),
            ],
            summary: "Infrastructure overlays stay compare-first provider truth layers and do not claim ordinary file-save semantics.".to_owned(),
        },
        ArtifactSaveTruthRow {
            row_id: "imported_archive_capture".to_owned(),
            surface_class: MatrixSurfaceClass::ImportedArchiveCapture,
            title: "Imported archive capture".to_owned(),
            supported_root_classes: vec![
                MatrixRootClass::ArchivePackaged,
                MatrixRootClass::ManagedOfflineBundle,
            ],
            path_identity_class: MatrixPathIdentityClass::ImportedSnapshotIdentity,
            preferred_save_path: MatrixSaveFallback::SaveAsCopy,
            disclosed_alternate_paths: vec![],
            metadata_sensitivity_indicators: vec![
                MetadataSensitivityIndicator::Encoding,
                MetadataSensitivityIndicator::GeneratedStateBoundary,
                MetadataSensitivityIndicator::LogicalTargetAmbiguity,
            ],
            save_fallback_disclosure_class: SaveFallbackDisclosureClass::RegenerateOrExportDisclosed,
            fallback_actions: regenerate_or_export_actions(false),
            mutator_guards: compare_only_mutator_guards(),
            consumer_refs: vec![
                "crates/aureline-vfs/src/roots/virtual_documents.rs".to_owned(),
                "crates/aureline-preview/src/safe_preview.rs".to_owned(),
            ],
            fixture_refs: vec![
                "fixtures/state/artifact_save_truth/imported_archive_capture.json".to_owned(),
            ],
            summary: "Imported captures surface decode and target-boundary risk and only leave the archive as reviewed copies.".to_owned(),
        },
    ]
}

fn atomic_fallback_actions(continue_enabled: bool) -> Vec<FallbackActionDisclosure> {
    vec![
        FallbackActionDisclosure {
            action_class: FallbackActionClass::Continue,
            visible_label: "Continue".to_owned(),
            enabled: continue_enabled,
            summary: "Proceed with the admitted degraded write path only after the weaker guarantee is disclosed.".to_owned(),
            disabled_reason: if continue_enabled {
                None
            } else {
                Some("No degraded direct-write path is admitted.".to_owned())
            },
        },
        FallbackActionDisclosure {
            action_class: FallbackActionClass::InspectSafetyNote,
            visible_label: "Inspect safety note".to_owned(),
            enabled: true,
            summary: "Review encoding, metadata, and target-identity risks before continuing.".to_owned(),
            disabled_reason: None,
        },
        FallbackActionDisclosure {
            action_class: FallbackActionClass::AlternateTarget,
            visible_label: "Alternate target".to_owned(),
            enabled: true,
            summary: "Route the write to a new admitted target instead of weakening the current one.".to_owned(),
            disabled_reason: None,
        },
        FallbackActionDisclosure {
            action_class: FallbackActionClass::CompareBeforeSave,
            visible_label: "Compare before save".to_owned(),
            enabled: true,
            summary: "Review the staged content against the current authoritative target before committing.".to_owned(),
            disabled_reason: None,
        },
    ]
}

fn regenerate_or_export_actions(continue_enabled: bool) -> Vec<FallbackActionDisclosure> {
    vec![
        FallbackActionDisclosure {
            action_class: FallbackActionClass::Continue,
            visible_label: "Continue".to_owned(),
            enabled: continue_enabled,
            summary: "Proceed by regeneration or reviewed export rather than exact file overwrite.".to_owned(),
            disabled_reason: if continue_enabled {
                None
            } else {
                Some("Only alternate-target export is admitted for this surface.".to_owned())
            },
        },
        FallbackActionDisclosure {
            action_class: FallbackActionClass::InspectSafetyNote,
            visible_label: "Inspect safety note".to_owned(),
            enabled: true,
            summary: "Inspect which source, generator, or import boundary owns the artifact bytes.".to_owned(),
            disabled_reason: None,
        },
        FallbackActionDisclosure {
            action_class: FallbackActionClass::AlternateTarget,
            visible_label: "Alternate target".to_owned(),
            enabled: true,
            summary: "Copy or export the artifact to a separate target without changing the authoritative source path.".to_owned(),
            disabled_reason: None,
        },
        FallbackActionDisclosure {
            action_class: FallbackActionClass::CompareBeforeSave,
            visible_label: "Compare before save".to_owned(),
            enabled: true,
            summary: "Compare current artifact bytes with the regeneration or export basis before materializing output.".to_owned(),
            disabled_reason: None,
        },
    ]
}

fn draft_stage_actions(continue_enabled: bool) -> Vec<FallbackActionDisclosure> {
    vec![
        FallbackActionDisclosure {
            action_class: FallbackActionClass::Continue,
            visible_label: "Continue".to_owned(),
            enabled: continue_enabled,
            summary: "Stage a local draft and defer publish or replay until revalidation succeeds.".to_owned(),
            disabled_reason: if continue_enabled {
                None
            } else {
                Some("This draft cannot advance without manual review.".to_owned())
            },
        },
        FallbackActionDisclosure {
            action_class: FallbackActionClass::InspectSafetyNote,
            visible_label: "Inspect safety note".to_owned(),
            enabled: true,
            summary: "Inspect draft-versus-publish truth, metadata sensitivity, and replay constraints.".to_owned(),
            disabled_reason: None,
        },
        FallbackActionDisclosure {
            action_class: FallbackActionClass::AlternateTarget,
            visible_label: "Alternate target".to_owned(),
            enabled: true,
            summary: "Export or duplicate the draft to a distinct target instead of overwriting the provider-linked object.".to_owned(),
            disabled_reason: None,
        },
        FallbackActionDisclosure {
            action_class: FallbackActionClass::CompareBeforeSave,
            visible_label: "Compare before save".to_owned(),
            enabled: true,
            summary: "Compare the staged draft with the current publish basis or replay target before continuing.".to_owned(),
            disabled_reason: None,
        },
    ]
}

fn compare_only_actions() -> Vec<FallbackActionDisclosure> {
    vec![
        FallbackActionDisclosure {
            action_class: FallbackActionClass::Continue,
            visible_label: "Continue".to_owned(),
            enabled: false,
            summary: "Direct save is blocked on this surface.".to_owned(),
            disabled_reason: Some("Only compare/review or alternate-target export is admitted.".to_owned()),
        },
        FallbackActionDisclosure {
            action_class: FallbackActionClass::InspectSafetyNote,
            visible_label: "Inspect safety note".to_owned(),
            enabled: true,
            summary: "Inspect the provider, metadata, and identity reasons ordinary save is blocked.".to_owned(),
            disabled_reason: None,
        },
        FallbackActionDisclosure {
            action_class: FallbackActionClass::AlternateTarget,
            visible_label: "Alternate target".to_owned(),
            enabled: true,
            summary: "Export or copy the visible content to a new target without claiming in-place authority.".to_owned(),
            disabled_reason: None,
        },
        FallbackActionDisclosure {
            action_class: FallbackActionClass::CompareBeforeSave,
            visible_label: "Compare before save".to_owned(),
            enabled: true,
            summary: "Compare the visible overlay with the current authoritative source before any export or follow-up action.".to_owned(),
            disabled_reason: None,
        },
    ]
}

fn code_file_mutator_guards(include_execute_bit_writer: bool) -> Vec<MutatorGuardRow> {
    vec![
        MutatorGuardRow {
            mutator_class: AutomationMutatorClass::FormatOnSave,
            available: true,
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            summary: "Format-on-save stages content and must rebase or abort if the target changed mid-flight.".to_owned(),
        },
        MutatorGuardRow {
            mutator_class: AutomationMutatorClass::OrganizeImports,
            available: true,
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            summary: "Organize-imports runs on staged content and refuses to stomp newer on-disk state.".to_owned(),
        },
        MutatorGuardRow {
            mutator_class: AutomationMutatorClass::RefactorApply,
            available: true,
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            summary: "Refactor apply must rebase or abort when canonical target identity or generation drifted.".to_owned(),
        },
        MutatorGuardRow {
            mutator_class: AutomationMutatorClass::AiApply,
            available: true,
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            summary: "AI apply shares the same staged-buffer and external-drift guardrail as save participants.".to_owned(),
        },
        MutatorGuardRow {
            mutator_class: AutomationMutatorClass::GeneratedOutputWrite,
            available: include_execute_bit_writer,
            drift_policy_class: if include_execute_bit_writer {
                MidFlightDriftPolicyClass::RebaseOrAbort
            } else {
                MidFlightDriftPolicyClass::NotApplicable
            },
            summary: if include_execute_bit_writer {
                "Generated companion output must rebase or abort before updating the paired artifact.".to_owned()
            } else {
                "Generated output writes do not apply to this surface family.".to_owned()
            },
        },
    ]
}

fn generated_only_mutator_guards() -> Vec<MutatorGuardRow> {
    vec![
        unavailable_mutator(AutomationMutatorClass::FormatOnSave),
        unavailable_mutator(AutomationMutatorClass::OrganizeImports),
        unavailable_mutator(AutomationMutatorClass::RefactorApply),
        unavailable_mutator(AutomationMutatorClass::AiApply),
        MutatorGuardRow {
            mutator_class: AutomationMutatorClass::GeneratedOutputWrite,
            available: true,
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            summary: "Generated output writes must rebase or abort when the source basis changed mid-flight.".to_owned(),
        },
    ]
}

fn draft_packet_mutator_guards() -> Vec<MutatorGuardRow> {
    vec![
        MutatorGuardRow {
            mutator_class: AutomationMutatorClass::FormatOnSave,
            available: true,
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            summary: "Structured packet formatting must rebase or abort if the packet target drifted.".to_owned(),
        },
        unavailable_mutator(AutomationMutatorClass::OrganizeImports),
        unavailable_mutator(AutomationMutatorClass::RefactorApply),
        MutatorGuardRow {
            mutator_class: AutomationMutatorClass::AiApply,
            available: true,
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            summary: "AI edits on draft packets must not silently overwrite newer staged or remote state.".to_owned(),
        },
        MutatorGuardRow {
            mutator_class: AutomationMutatorClass::GeneratedOutputWrite,
            available: true,
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            summary: "Generated packet refreshes must rebase or abort before replay or publish.".to_owned(),
        },
    ]
}

fn provider_draft_mutator_guards() -> Vec<MutatorGuardRow> {
    vec![
        MutatorGuardRow {
            mutator_class: AutomationMutatorClass::FormatOnSave,
            available: true,
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            summary:
                "Draft formatting must rebase or abort against the current local-draft authority."
                    .to_owned(),
        },
        unavailable_mutator(AutomationMutatorClass::OrganizeImports),
        unavailable_mutator(AutomationMutatorClass::RefactorApply),
        MutatorGuardRow {
            mutator_class: AutomationMutatorClass::AiApply,
            available: true,
            drift_policy_class: MidFlightDriftPolicyClass::RebaseOrAbort,
            summary:
                "AI apply on provider drafts must rebase or abort before any publish-later replay."
                    .to_owned(),
        },
        unavailable_mutator(AutomationMutatorClass::GeneratedOutputWrite),
    ]
}

fn compare_only_mutator_guards() -> Vec<MutatorGuardRow> {
    vec![
        blocked_mutator(AutomationMutatorClass::FormatOnSave),
        blocked_mutator(AutomationMutatorClass::OrganizeImports),
        blocked_mutator(AutomationMutatorClass::RefactorApply),
        blocked_mutator(AutomationMutatorClass::AiApply),
        blocked_mutator(AutomationMutatorClass::GeneratedOutputWrite),
    ]
}

fn unavailable_mutator(mutator_class: AutomationMutatorClass) -> MutatorGuardRow {
    MutatorGuardRow {
        mutator_class,
        available: false,
        drift_policy_class: MidFlightDriftPolicyClass::NotApplicable,
        summary: format!(
            "{} does not apply to this surface family.",
            mutator_class.as_str()
        ),
    }
}

fn blocked_mutator(mutator_class: AutomationMutatorClass) -> MutatorGuardRow {
    MutatorGuardRow {
        mutator_class,
        available: false,
        drift_policy_class: MidFlightDriftPolicyClass::CompareThenManualRecovery,
        summary: format!(
            "{} cannot mutate this surface directly; route to compare/review or alternate-target export instead.",
            mutator_class.as_str()
        ),
    }
}
